use pqcrypto_kyber::kyber1024;
use pqcrypto_traits::kem::{SharedSecret, Ciphertext};
use zeroize::Zeroize;

pub struct KyberPublicKey(pub kyber1024::PublicKey);
pub struct KyberSecretKey(pub kyber1024::SecretKey);
pub struct KyberSharedSecret(pub kyber1024::SharedSecret);
pub struct KyberCiphertext(pub kyber1024::Ciphertext);

pub fn generate_keypair() -> (KyberPublicKey, KyberSecretKey) {
    let (pk, sk) = kyber1024::keypair();
    (KyberPublicKey(pk), KyberSecretKey(sk))
}

pub fn encapsulate(
    pk: &KyberPublicKey,
) -> (KyberSharedSecret, KyberCiphertext) {
    let (ss, ct) = kyber1024::encapsulate(&pk.0);
    (KyberSharedSecret(ss), KyberCiphertext(ct))
}

pub fn decapsulate(
    ct: &KyberCiphertext,
    sk: &KyberSecretKey,
) -> KyberSharedSecret {
    let ss = kyber1024::decapsulate(&ct.0, &sk.0);
    KyberSharedSecret(ss)
}
