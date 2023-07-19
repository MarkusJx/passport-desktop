use napi::bindgen_prelude::{FromNapiValue, ToNapiValue};
use windows::Security::Credentials::UI::UserConsentVerificationResult;

#[napi]
/// The result of a user consent verification operation.
pub enum VerificationResult {
    /// The user consent verification operation succeeded.
    Verified,
    /// The user consent verification operation failed
    /// because the device is not present.
    DeviceNotPresent,
    /// The user consent verification operation failed
    /// because Windows Hello is not configured for the user.
    NotConfiguredForUser,
    /// The user consent verification operation failed
    /// because Windows Hello is disabled by policy.
    DisabledByPolicy,
    /// The user consent verification operation
    /// failed because the device is busy.
    DeviceBusy,
    /// The user consent verification operation failed
    /// because the maximum number of attempts was exceeded.
    RetriesExhausted,
    /// The user consent verification operation
    /// was canceled by the user.
    Canceled,
}

impl TryFrom<UserConsentVerificationResult> for VerificationResult {
    type Error = napi::Error;

    fn try_from(value: UserConsentVerificationResult) -> napi::Result<Self> {
        Ok(match value {
            UserConsentVerificationResult::Verified => Self::Verified,
            UserConsentVerificationResult::DeviceNotPresent => Self::DeviceNotPresent,
            UserConsentVerificationResult::NotConfiguredForUser => Self::NotConfiguredForUser,
            UserConsentVerificationResult::DisabledByPolicy => Self::DisabledByPolicy,
            UserConsentVerificationResult::DeviceBusy => Self::DeviceBusy,
            UserConsentVerificationResult::RetriesExhausted => Self::RetriesExhausted,
            UserConsentVerificationResult::Canceled => Self::Canceled,
            v => {
                return Err(napi::Error::new(
                    napi::Status::GenericFailure,
                    format!("Unknown verification result: {}", v.0),
                ))
            }
        })
    }
}
