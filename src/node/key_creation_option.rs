use napi::bindgen_prelude::{FromNapiValue, ToNapiValue};
use windows::Security::Credentials::KeyCredentialCreationOption;

#[napi]
/// A key creation option.
pub enum KeyCreationOption {
    /// Replace an existing key.
    ReplaceExisting,
    /// Throw an error if the key already exists.
    FailIfExists,
}

impl From<KeyCreationOption> for KeyCredentialCreationOption {
    fn from(value: KeyCreationOption) -> Self {
        match value {
            KeyCreationOption::ReplaceExisting => Self::ReplaceExisting,
            KeyCreationOption::FailIfExists => Self::FailIfExists,
        }
    }
}
