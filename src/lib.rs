#![cfg(windows)]
extern crate winapi;

extern crate failure;
#[macro_use]
extern crate failure_derive;

#[macro_use]
pub mod macros;

pub mod bstr;
pub mod comptr;
mod comutil;
mod iunknown;
pub mod refcount;
mod types;

pub use comutil::*;
pub use iunknown::*;
pub use types::*;

pub mod errors {
    pub type Result<T> = ::std::result::Result<T, ComError>;

    #[derive(Debug, Fail)]
    #[fail(display = "COM call returned error: {}", result)]
    pub struct ComError {
        pub result: ::HRESULT,
    }
}
