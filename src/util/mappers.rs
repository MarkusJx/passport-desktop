use std::error::Error;

pub trait MapNapiError<T> {
    fn map_napi_error(self) -> napi::Result<T>;
}

impl<T, U: Error> MapNapiError<T> for Result<T, U> {
    fn map_napi_error(self) -> napi::Result<T> {
        self.map_err(|e| napi::Error::from_reason(e.to_string()))
    }
}
