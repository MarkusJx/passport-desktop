#[macro_export]
macro_rules! check_account_exists {
    ($account_exists: expr) => {
        if !$account_exists.lock().unwrap().clone() {
            return Err(napi::Error::new(
                napi::Status::GenericFailure,
                "The passport account does not exist",
            ));
        }
    };
}
