#![allow(non_camel_case_types)]

use errors::*;
use std::fmt;

#[must_use]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct HRESULT(pub u32);
pub const S_OK: HRESULT = HRESULT(0);
pub const E_NOINTERFACE: HRESULT = HRESULT(0x80004002);
pub const E_POINTER: HRESULT = HRESULT(0x80004003);

impl HRESULT {
    pub fn result(&self) -> Result<()> {
        if *self == S_OK {
            Ok(())
        } else {
            Err(ComError { result: *self })
        }
    }
}

impl fmt::Display for HRESULT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HRESULT(0x{:08x})", self.0 as u32)
    }
}

pub use winapi::shared::guiddef::CLSID;
pub use winapi::shared::guiddef::GUID;
pub use winapi::shared::guiddef::IID;
pub use winapi::shared::minwindef::LPVOID;
pub use winapi::shared::wtypesbase::CLSCTX;
pub use winapi::shared::wtypesbase::ULONG;
pub use winapi::um::unknwnbase::IUnknown;

pub type RawComPtr = *mut IUnknown;
