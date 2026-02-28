use pqcrypto_dilithium::dilithium5;
use pqcrypto_traits::sign::DetachedSignature;
use zeroize::Zeroize;

pub struct Dilithium5PublicKey(pub dilithium5::PublicKey);
pub struct Dilithium5SecretKey(pub dilithium5::SecretKey);
pub struct Dilithium5Signature(pub dilithium5::DetachedSignature);

pub fn generate_keypair() -> (Dilithium5PublicKey, Dilithium5SecretKey) {
    let (pk, sk) = dilithium5::keypair();
    (Dilithium5PublicKey(pk), Dilithium5SecretKey(sk))
}

pub fn sign(
    message: &[u8],
    sk: &Dilithium5SecretKey,
) -> Dilithium5Signature {
    let sig = dilithium5::detached_sign(message, &sk.0);
    Dilithium5Signature(sig)
}

pub fn verify(
    message: &[u8],
    sig: &Dilithium5Signature,
    pk: &Dilithium5PublicKey,
) -> bool {
    dilithium5::verify_detached_signature(&sig.0, message, &pk.0).is_ok()
}
