use napi::bindgen_prelude::{FromNapiValue, ToNapiValue};
use windows::Security::Cryptography::Core::CryptographicPublicKeyBlobType;

#[napi]
/// The public key encoding to use.
pub enum PublicKeyEncoding {
    X509SubjectPublicKeyInfo,
    /// The key is a PKCS#1 RSA DER-encoded public key.
    /// This is the default encoding as it is the
    /// most useful for interoperability.
    Pkcs1RsaPublicKey,
    BCryptPublicKey,
    Capi1PublicKey,
    BCryptEccFullPublicKey,
}

impl From<PublicKeyEncoding> for CryptographicPublicKeyBlobType {
    fn from(value: PublicKeyEncoding) -> Self {
        match value {
            PublicKeyEncoding::X509SubjectPublicKeyInfo => {
                CryptographicPublicKeyBlobType::X509SubjectPublicKeyInfo
            }
            PublicKeyEncoding::Pkcs1RsaPublicKey => {
                CryptographicPublicKeyBlobType::Pkcs1RsaPublicKey
            }
            PublicKeyEncoding::BCryptPublicKey => CryptographicPublicKeyBlobType::BCryptPublicKey,
            PublicKeyEncoding::Capi1PublicKey => CryptographicPublicKeyBlobType::Capi1PublicKey,
            PublicKeyEncoding::BCryptEccFullPublicKey => {
                CryptographicPublicKeyBlobType::BCryptEccFullPublicKey
            }
        }
    }
}
