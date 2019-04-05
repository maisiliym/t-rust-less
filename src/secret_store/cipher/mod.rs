use super::SecretStoreResult;
use crate::secret_store_capnp::{block, recipient};
use secrets::{Secret, SecretVec};

pub trait Cipher {
  type PrivateKey;
  type PublicKey;

  fn generate_key_pair() -> SecretStoreResult<(Self::PublicKey, Self::PrivateKey)>;

  fn encrypt(recipients: &[(&str, &Self::PublicKey)], data: &SecretVec<u8>) -> SecretStoreResult<block::Owned>;

  fn decrypt(user: (&str, &Self::PrivateKey), crypted: &block::Reader) -> SecretStoreResult<SecretVec<u8>>;
}
