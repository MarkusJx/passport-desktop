use windows::Security::Credentials::KeyCredentialStatus;

pub trait PassportError {
    fn user_cancelled() -> Self;
    fn not_found() -> Self;
    fn user_prefers_password() -> Self;
    fn credential_already_exists() -> Self;
    fn security_device_locked() -> Self;
    fn unknown(status: KeyCredentialStatus) -> Self;

    fn from_credential_status(status: KeyCredentialStatus) -> Self;
}

impl PassportError for napi::Error {
    fn user_cancelled() -> Self {
        napi::Error::new(
            napi::Status::GenericFailure,
            "The user canceled the operation",
        )
    }

    fn not_found() -> Self {
        napi::Error::new(
            napi::Status::GenericFailure,
            "The user needs to create a PIN or biometric gesture before creating a Passport key",
        )
    }

    fn user_prefers_password() -> Self {
        napi::Error::new(napi::Status::GenericFailure, "The user prefers a password")
    }

    fn credential_already_exists() -> Self {
        napi::Error::new(
            napi::Status::GenericFailure,
            "The credential already exists",
        )
    }

    fn security_device_locked() -> Self {
        napi::Error::new(
            napi::Status::GenericFailure,
            "The security device is locked",
        )
    }

    fn unknown(status: KeyCredentialStatus) -> Self {
        napi::Error::new(
            napi::Status::GenericFailure,
            format!("An unknown error occurred. Status code: {}", status.0),
        )
    }

    fn from_credential_status(status: KeyCredentialStatus) -> Self {
        match status {
            KeyCredentialStatus::Success => napi::Error::new(
                napi::Status::GenericFailure,
                "No error occurred although an error was expected",
            ),
            KeyCredentialStatus::UserCanceled => Self::user_cancelled(),
            KeyCredentialStatus::NotFound => Self::not_found(),
            KeyCredentialStatus::UserPrefersPassword => Self::user_prefers_password(),
            KeyCredentialStatus::CredentialAlreadyExists => Self::credential_already_exists(),
            KeyCredentialStatus::SecurityDeviceLocked => Self::security_device_locked(),
            s => Self::unknown(s),
        }
    }
}
