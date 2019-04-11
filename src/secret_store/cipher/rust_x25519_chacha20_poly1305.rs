use super::{Cipher, PrivateData, PrivateKey, PublicData, PublicKey, SealKey};
use crate::memguard::{memory, SecretBytes};
use crate::secret_store::{SecretStoreError, SecretStoreResult};
use crate::secret_store_capnp::{block, KeyType};
use chacha20_poly1305_aead::{decrypt, encrypt};
use core::borrow::Borrow;
use rand::{thread_rng, OsRng, RngCore};
use std::io::Cursor;

pub struct RustX25519ChaCha20Poly1305Cipher;

const TAG_LENGTH: usize = 16;

fn xorbytes(src1: &[u8], src2: &[u8], tgt: &mut [u8]) {
  for ((s1, s2), t) in src1.iter().zip(src2).zip(tgt) {
    *t = *s1 ^ *s2
  }
}

impl RustX25519ChaCha20Poly1305Cipher {
  fn unpack_public(key: &[u8]) -> x25519_dalek::PublicKey {
    let mut raw = [0u8; 32];

    raw.copy_from_slice(&key.borrow());

    x25519_dalek::PublicKey::from(raw)
  }

  fn unpack_private(key: &PrivateKey) -> x25519_dalek::StaticSecret {
    let mut raw = [0u8; 32];
    let ptr = raw.as_mut_ptr();

    raw.copy_from_slice(&key.borrow());

    let private = x25519_dalek::StaticSecret::from(raw);

    unsafe {
      memory::memzero(ptr, 32);
    }
    private
  }
}

impl Cipher for RustX25519ChaCha20Poly1305Cipher {
  fn generate_key_pair() -> SecretStoreResult<(PublicKey, PrivateKey)> {
    let mut rng = thread_rng();
    let private = x25519_dalek::StaticSecret::new(&mut rng);
    let public = x25519_dalek::PublicKey::from(&private);
    let mut private_raw = private.to_bytes();

    Ok((public.as_bytes().to_vec(), SecretBytes::from(&mut private_raw[..])))
  }

  fn seal_key_length() -> usize {
    32
  }

  fn seal_min_nonce_length() -> usize {
    12
  }

  fn seal_private_key(seal_key: &SealKey, nonce: &[u8], private_key: &PrivateKey) -> SecretStoreResult<PublicData> {
    let mut result = Vec::with_capacity(private_key.len());
    let tag = encrypt(&seal_key.borrow(), nonce, &[], &private_key.borrow(), &mut result)?;
    result.extend_from_slice(&tag);

    Ok(result)
  }

  fn open_private_key(seal_key: &SealKey, nonce: &[u8], crypted_key: &[u8]) -> SecretStoreResult<PrivateKey> {
    if crypted_key.len() < TAG_LENGTH {
      return Err(SecretStoreError::Cipher("Data too short".to_string()));
    }
    let tag_offset = crypted_key.len() - TAG_LENGTH;
    let mut result = SecretBytes::with_capacity(crypted_key.len() - TAG_LENGTH);
    decrypt(
      &seal_key.borrow(),
      nonce,
      &[],
      &crypted_key[0..tag_offset],
      &crypted_key[tag_offset..],
      &mut result.borrow_mut(),
    )?;

    Ok(result)
  }

  fn encrypt(
    recipients: &[(&str, &PublicKey)],
    data: &PrivateData,
    mut header_builder: block::header::Builder,
  ) -> SecretStoreResult<PublicData> {
    let mut rng = thread_rng();
    let seal_key = SecretBytes::random(&mut rng, 32);
    let mut public_data = Vec::with_capacity(data.len() + TAG_LENGTH + 32);
    let mut nonce = [0u8; 12];

    rng.fill_bytes(&mut nonce[..]);

    let tag = encrypt(&seal_key.borrow(), &nonce, &[], &data.borrow(), &mut public_data)?;
    public_data.extend_from_slice(&tag);

    header_builder.set_type(KeyType::Ed25519Chacha20Poly1305);
    header_builder.reborrow().init_common_key(12).copy_from_slice(&nonce);

    let mut recipient_keys = header_builder.init_recipients(recipients.len() as u32);

    for (idx, (recipient_id, recipient_public_key)) in recipients.iter().enumerate() {
      let ephemeral_private = x25519_dalek::EphemeralSecret::new(&mut rng);
      let ephemeral_public = x25519_dalek::PublicKey::from(&ephemeral_private);
      let recipient_public = Self::unpack_public(recipient_public_key);
      let shared_secret = ephemeral_private.diffie_hellman(&recipient_public);

      let mut recipient_key = recipient_keys.reborrow().get(idx as u32);

      recipient_key.set_id(recipient_id);
      let crypted_key = recipient_key.init_crypted_key(64);
      crypted_key[0..32].copy_from_slice(ephemeral_public.as_bytes());
      xorbytes(&seal_key.borrow(), shared_secret.as_bytes(), &mut crypted_key[32..64]);
    }

    Ok(public_data)
  }

  fn decrypt(
    user: (&str, &PrivateKey),
    header: block::header::Reader,
    crypted: &[u8],
  ) -> SecretStoreResult<PrivateData> {
    if crypted.len() < TAG_LENGTH {
      return Err(SecretStoreError::Cipher("Data too short".to_string()));
    }
    let nonce = header.get_common_key()?;

    if nonce.len() != 12 {
      return Err(SecretStoreError::Cipher("Invalid nonce".to_string()));
    }

    for recipient in header.get_recipients()?.iter() {
      if user.0 != recipient.get_id()? {
        continue;
      }
      let crypted_key = recipient.get_crypted_key()?;

      if crypted_key.len() != 64 {
        return Err(SecretStoreError::Cipher("Invalid crypted key".to_string()));
      }
      let mut ephemeral_public_raw = [0u8; 32];
      ephemeral_public_raw.copy_from_slice(&crypted_key[0..32]);
      let ephemeral_public = x25519_dalek::PublicKey::from(ephemeral_public_raw);
      let recipient_private = Self::unpack_private(&user.1);
      let shared_secret = recipient_private.diffie_hellman(&ephemeral_public);
      let mut seal_key = SecretBytes::zeroed(32);

      xorbytes(
        shared_secret.as_bytes(),
        &crypted_key[32..64],
        seal_key.borrow_mut().as_mut(),
      );

      let tag_offset = crypted.len() - TAG_LENGTH;
      let mut decrypted = SecretBytes::with_capacity(crypted.len() - TAG_LENGTH);

      decrypt(
        &seal_key.borrow(),
        nonce,
        &[],
        &crypted[0..tag_offset],
        &crypted[tag_offset..],
        &mut decrypted.borrow_mut(),
      )?;

      return Ok(decrypted);
    }
    Err(SecretStoreError::NoRecipient)
  }
}
