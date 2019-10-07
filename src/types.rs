#![allow(non_camel_case_types)]

use crate::errors::*;
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

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl GUID {
    pub(crate) fn to_native(&self) -> guiddef::GUID {
        guiddef::GUID {
            Data1: self.data1,
            Data2: self.data2,
            Data3: self.data3,
            Data4: self.data4,
        }
    }
}

pub type IID = GUID;
pub type CLSID = GUID;

use winapi::shared::guiddef;
pub(crate) use winapi::shared::minwindef::LPVOID;
pub use winapi::shared::windef::HWND;
pub use winapi::shared::wtypesbase::CLSCTX;
pub use winapi::shared::wtypesbase::CLSCTX_LOCAL_SERVER;
pub use winapi::shared::wtypesbase::ULONG;
use winapi::um::unknwnbase::IUnknown;

pub type RawComPtr = *mut IUnknown;
