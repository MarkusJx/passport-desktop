use windows::core::HSTRING;
use windows::Security::Credentials::{
    KeyCredentialCreationOption, KeyCredentialManager, KeyCredentialRetrievalResult,
    KeyCredentialStatus,
};

pub async fn create_passport_key(
    account_id: &String,
    create_option: Option<KeyCredentialCreationOption>,
) -> windows::core::Result<KeyCredentialStatus> {
    KeyCredentialManager::RequestCreateAsync(
        &HSTRING::from(account_id),
        create_option.unwrap_or(KeyCredentialCreationOption::ReplaceExisting),
    )?
    .await?
    .Status()
}

pub async fn get_passport_account(
    account_id: &String,
) -> windows::core::Result<KeyCredentialRetrievalResult> {
    KeyCredentialManager::OpenAsync(&HSTRING::from(account_id))?.await
}

pub fn get_passport_account_sync(
    account_id: &String,
) -> windows::core::Result<KeyCredentialRetrievalResult> {
    KeyCredentialManager::OpenAsync(&HSTRING::from(account_id))?.get()
}
