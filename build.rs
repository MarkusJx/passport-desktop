extern crate napi_build;

fn main() {
    #[cfg(not(windows))]
    panic!("This crate does not support non-Windows platforms");

    napi_build::setup();
}
