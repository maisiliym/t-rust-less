@0x89ae7248ac2e8067;

enum KeyType {
    rsaAesGcm @0;
    ed25519Chacha20Poly1305 @1;
}

struct PublicKey {
    type @0 : KeyType;
    key @1 : KeyType;
}

struct Recipient {
    id @0 : Data;
    name @1 : Text;
    email @2 : Text;
    publicKeys @3 : List(PublicKey);
}

struct PublicRing {
    recipients @0 : List(Recipient);

}

struct Ring {
    recipients @0 : List(User);

    struct User {
        id @0 : Data;
        privateKeys @1 : List(PrivateKey);
    }

    struct PrivateKey {
        type @0 : KeyType;
        salt @1 : Data;
        cryptedKey @2 : Data;
    }
}

struct Block {
    headers @0 : List(Header);
    content @1 : Data;

    struct Header {
        type @0 : KeyType;
        commonKey @1 : Data;
        recipients @2 : List(RecipientKey);
    }

    struct RecipientKey {
        id @0 : Data;
        cryptedKey @1: Data;
    }
}