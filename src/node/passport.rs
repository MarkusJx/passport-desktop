use crate::check_account_exists;
use crate::node::key_creation_option::KeyCreationOption;
use crate::node::public_key_encoding::PublicKeyEncoding;
use crate::node::verification_result::VerificationResult;
use crate::util::errors::PassportError;
use crate::util::mappers::MapNapiError;
use crate::util::traits::IntoWinBuffer;
use crate::win::passport::{create_passport_key, get_passport_account, get_passport_account_sync};
use napi::bindgen_prelude::Buffer;
use std::sync::Mutex;
use windows::core::HSTRING;
use windows::Security::Credentials::UI::UserConsentVerifier;
use windows::Security::Credentials::{KeyCredentialManager, KeyCredentialStatus};
use windows::Security::Cryptography::Core::CryptographicPublicKeyBlobType;

#[napi]
/// The Passport module provides an interface to the Windows Hello API.
/// It allows you to create a key pair and sign data using the private key.
/// The public key can be exported in a variety of formats.
///
/// # Example
/// ```ts
/// import { Passport, PublicKeyEncoding, KeyCreationOption } from 'ms-passport';
/// import { randomBytes, createPublicKey, createVerify } from 'node:crypto';
///
/// if (!Passport.available()) {
///   throw new Error('Windows Hello is not available');
/// }
///
/// await Passport.accountWithIdExists('my-account-id'); // false
///
/// const passport = new Passport('my-account-id');
/// if (!passport.accountExists) {
///   await passport.createAccount(KeyCreationOption.FailIfExists);
/// }
///
/// const challenge = randomBytes(32);
/// const signature = await passport.sign(challenge);
///
/// // Verify the signature with the public key
/// const keyBuffer = await passport.getPublicKey(PublicKeyEncoding.Pkcs1RsaPublicKey);
/// const key = createPublicKey({
///   key: keyBuffer,
///   format: 'der',
///   type: 'pkcs1'
/// });
///
/// // Create a verifier and verify the challenge
/// const verify = createVerify('SHA256');
/// verify.write(challenge);
/// verify.end();
///
/// verify.verify(key, signature); // true
///
/// // Delete the account
/// await passport.deleteAccount();
/// ```
pub struct Passport {
    account_id: String,
    account_exists: Mutex<bool>,
}

#[napi]
impl Passport {
    #[napi(constructor)]
    /// Create a new Passport instance.
    /// The account_id is used to identify the account in the Windows Credential Manager.
    /// If an account with the given id already exists, it will be used.
    /// You can check if an account exists with the `accountExists` getter.
    ///
    /// # Example
    /// ```ts
    /// import { Passport } from 'ms-passport';
    ///
    /// const passport = new Passport('my-account-id');
    /// ```
    ///
    /// @param accountId The id of the account in the Windows Credential Manager.
    pub fn new(account_id: String) -> napi::Result<Self> {
        Ok(Self {
            account_exists: Mutex::new(Self::account_with_id_exists(account_id.clone())?),
            account_id,
        })
    }

    #[napi]
    /// Create a new passport account.
    /// You can optionally pass a {@link KeyCreationOption} to customize the key creation.
    /// If no option is passed, an existing key will be replaced.
    /// If the account does not exist, it will be created.
    ///
    /// # Example
    /// ```ts
    /// import { Passport, KeyCreationOption } from 'ms-passport';
    ///
    /// const passport = new Passport('my-account-id');
    /// await passport.createAccount(KeyCreationOption.FailIfExists);
    /// ```
    ///
    /// @param creationOption The {@link KeyCreationOption} to use when creating the key.
    pub async fn create_account(
        &self,
        creation_option: Option<KeyCreationOption>,
    ) -> napi::Result<()> {
        let create_result =
            create_passport_key(&self.account_id, creation_option.map(|o| o.into()))
                .await
                .map_napi_error()?;

        match create_result {
            KeyCredentialStatus::Success => {
                *self.account_exists.lock().unwrap() = true;
                Ok(())
            }
            s => Err(napi::Error::from_credential_status(s)),
        }
    }

    #[napi]
    /// Sign a challenge with the private key.
    /// If the account does not exist, an error will be thrown.
    /// This will open a Windows Hello dialog to verify the user.
    /// If the challenge is not verified, an error will be thrown.
    ///
    /// The signature can be verified with the public key, for example
    /// using the `crypto` module.
    ///
    /// # Example
    /// ```ts
    /// import { Passport, PublicKeyEncoding } from 'ms-passport';
    /// import { randomBytes, createPublicKey, createVerify } from 'node:crypto';
    ///
    /// const passport = new Passport('my-account-id');
    /// // Create the key pair
    /// await passport.createAccount();
    ///
    /// // Create a challenge and sign it
    /// const challenge = randomBytes(32);
    /// const signature = await passport.sign(challenge);
    ///
    /// // Verify the signature with the public key
    /// const keyBuffer = await passport.getPublicKey(PublicKeyEncoding.Pkcs1RsaPublicKey);
    /// const key = createPublicKey({
    ///   key: keyBuffer,
    ///   format: 'der',
    ///   type: 'pkcs1'
    /// });
    ///
    /// // Create a verifier and verify the challenge
    /// const verify = createVerify('SHA256');
    /// verify.write(challenge);
    /// verify.end();
    ///
    /// verify.verify(key, signature); // true
    /// ```
    ///
    /// @see {@link getPublicKey}
    /// @param challenge The challenge to sign.
    /// @return The signature.
    pub async fn sign(&self, challenge: Buffer) -> napi::Result<Buffer> {
        check_account_exists!(self.account_exists);
        let credential = get_passport_account(&self.account_id)
            .await
            .map_napi_error()?;

        let status = credential.Status().map_napi_error()?;
        if status != KeyCredentialStatus::Success {
            return Err(napi::Error::from_credential_status(status));
        }

        let credential = credential.Credential().map_napi_error()?;
        let res = credential
            .RequestSignAsync(&challenge.into_win_buffer().map_napi_error()?)
            .map_napi_error()?
            .get()
            .map_napi_error()?;

        let status = res.Status().map_napi_error()?;
        if status != KeyCredentialStatus::Success {
            return Err(napi::Error::from_credential_status(status));
        }

        Buffer::from_win_buffer(res.Result().map_napi_error()?).map_napi_error()
    }

    #[napi]
    /// Delete the account from the Windows Credential Manager.
    /// If the account does not exist, an error will be thrown.
    pub async fn delete_account(&self) -> napi::Result<()> {
        check_account_exists!(self.account_exists);
        KeyCredentialManager::DeleteAsync(&HSTRING::from(&self.account_id))
            .map_napi_error()?
            .await
            .map_napi_error()?;

        *self.account_exists.lock().unwrap() = false;
        Ok(())
    }

    #[napi]
    /// Get the public key of the account.
    /// If the account does not exist, an error will be thrown.
    /// The encoding of the key can be specified, defaulting to
    /// {@link PublicKeyEncoding.Pkcs1RsaPublicKey}.
    ///
    /// In order to verify a signature using this public key,
    /// use the {@link PublicKeyEncoding.Pkcs1RsaPublicKey}
    /// encoding and pass the result to the `crypto` module.
    ///
    /// # Example
    /// ```ts
    /// import { Passport, PublicKeyEncoding } from 'ms-passport';
    /// import { createPublicKey } from 'node:crypto';
    ///
    /// const passport = new Passport('my-account-id');
    /// // Create the key pair
    /// await passport.createAccount();
    ///
    /// const keyBuffer = await passport.getPublicKey(PublicKeyEncoding.Pkcs1RsaPublicKey);
    /// // Use these options to load the public key
    /// const key = createPublicKey({
    ///   key: keyBuffer,
    ///   format: 'der',
    ///   type: 'pkcs1'
    /// });
    /// ```
    ///
    /// @see {@link sign}
    /// @param encoding The encoding to use for the public key.
    /// @return The public key.
    pub async fn get_public_key(
        &self,
        encoding: Option<PublicKeyEncoding>,
    ) -> napi::Result<Buffer> {
        check_account_exists!(self.account_exists);
        let credential = get_passport_account(&self.account_id)
            .await
            .map_napi_error()?;

        let status = credential.Status().map_napi_error()?;
        if status != KeyCredentialStatus::Success {
            return Err(napi::Error::from_credential_status(status));
        }

        let credential = credential.Credential().map_napi_error()?;
        let res = credential
            .RetrievePublicKeyWithBlobType(
                encoding
                    .map(|e| e.into())
                    .unwrap_or(CryptographicPublicKeyBlobType::Pkcs1RsaPublicKey),
            )
            .map_napi_error()?;

        Buffer::from_win_buffer(res).map_napi_error()
    }

    #[napi(getter)]
    /// Whether the account exists in the Windows Credential Manager.
    /// This is only updated if the account exists when the Passport
    /// instance is created and when {@link createAccount} or
    /// {@link deleteAccount} are called. If the account is deleted
    /// or created outside of this instance, this value will not be
    /// updated.
    pub fn account_exists(&self) -> bool {
        *self.account_exists.lock().unwrap()
    }

    #[napi]
    /// Whether the Passport API is available on the current platform
    /// and the current user has permission to use it.
    /// This will return `false` on non-Windows platforms and if the
    /// user does not have permission to use Windows Hello.
    ///
    /// @return Whether the Passport API is available.
    pub fn available() -> napi::Result<bool> {
        KeyCredentialManager::IsSupportedAsync()
            .map_napi_error()?
            .get()
            .map_napi_error()
    }

    #[napi]
    /// Whether an account with the given ID exists in the Windows
    /// Credential Manager.
    ///
    /// @param id The ID of the account to check.
    /// @return Whether the account exists.
    pub fn account_with_id_exists(id: String) -> napi::Result<bool> {
        let status = get_passport_account_sync(&id)
            .map_napi_error()?
            .Status()
            .map_napi_error()?;

        match status {
            KeyCredentialStatus::Success => Ok(true),
            KeyCredentialStatus::NotFound => Ok(false),
            s => Err(napi::Error::from_credential_status(s)),
        }
    }

    #[napi]
    /// Request verification from the user. This will show a dialog
    /// to the user asking them to verify their identity. If the user
    /// accepts, the returned value will be {@link VerificationResult.Verified}.
    /// If the user rejects or cancels the dialog, the returned value will be
    /// another value from {@link VerificationResult} specifying the rejection
    /// reason.
    ///
    /// # Example
    /// ```ts
    /// import { Passport, VerificationResult } from 'ms-passport';
    ///
    /// const result = await Passport.requestVerification('Please verify your identity');
    /// if (result === VerificationResult.Verified) {
    ///   console.log('User verified');
    /// } else {
    ///   console.log('User rejected verification');
    /// }
    /// ```
    ///
    /// @param message The message to show to the user.
    /// @return The result of the verification request.
    pub async fn request_verification(message: String) -> napi::Result<VerificationResult> {
        VerificationResult::try_from(
            UserConsentVerifier::RequestVerificationAsync(&HSTRING::from(message))
                .map_napi_error()?
                .await
                .map_napi_error()?,
        )
    }
}
