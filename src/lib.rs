#![cfg(windows)]

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
    use failure::Fail;

    pub type Result<T> = ::std::result::Result<T, ComError>;

    #[derive(Debug, Fail)]
    #[fail(display = "COM call returned error: {}", result)]
    pub struct ComError {
        pub result: crate::HRESULT,
    }
}
