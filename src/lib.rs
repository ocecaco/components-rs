#![cfg(windows)]
#[macro_use]
extern crate bitflags;
extern crate libc;

extern crate failure;
#[macro_use]
extern crate failure_derive;

#[macro_use]
pub mod macros;

pub mod bstr;
pub mod comptr;
pub mod refcount;
mod types;
mod comutil;
mod iunknown;

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
