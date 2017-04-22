#![recursion_limit = "1024"]

#[macro_use]
extern crate bitflags;
extern crate libc;

#[macro_use]
extern crate error_chain;

#[macro_use]
pub mod macros;

pub mod bstr;
pub mod comptr;
pub mod refcount;
mod types;
mod comutil;
mod iunknown;

pub use types::*;
pub use comutil::*;
pub use iunknown::*;

pub mod errors {
    error_chain! {
        errors {
            ComCallFailed(result: ::HRESULT) {
                description("COM call failed")
                display("COM call failed: returned {}", result)
            }
        }
    }
}
