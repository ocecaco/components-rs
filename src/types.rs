#![allow(non_camel_case_types)]

use errors::*;
use libc::c_void;
use std::fmt;

bitflags! {
    #[repr(C)]
    pub struct COINIT: u32 {
        const APARTMENTTHREADED  = 0x2;
        const MULTITHREADED      = 0x0;
        const DISABLE_OLE1DDE    = 0x4;
        const SPEED_OVER_MEMORY  = 0x8;
    }
}

bitflags! {
    #[repr(C)]
    pub struct CLSCTX: u32 {
        const INPROC_SERVER           = 0x1;
        const INPROC_HANDLER          = 0x2;
        const LOCAL_SERVER            = 0x4;
        const INPROC_SERVER16         = 0x8;
        const REMOTE_SERVER           = 0x10;
        const INPROC_HANDLER16        = 0x20;
        const RESERVED1               = 0x40;
        const RESERVED2               = 0x80;
        const RESERVED3               = 0x100;
        const RESERVED4               = 0x200;
        const NO_CODE_DOWNLOAD        = 0x400;
        const RESERVED5               = 0x800;
        const NO_CUSTOM_MARSHAL       = 0x1000;
        const ENABLE_CODE_DOWNLOAD    = 0x2000;
        const NO_FAILURE_LOG          = 0x4000;
        const DISABLE_AAA             = 0x8000;
        const ENABLE_AAA              = 0x10000;
        const FROM_DEFAULT_CONTEXT    = 0x20000;
        const ACTIVATE_32_BIT_SERVER  = 0x40000;
        const ACTIVATE_64_BIT_SERVER  = 0x80000;
        const ENABLE_CLOAKING         = 0x100000;
        const APPCONTAINER            = 0x400000;
        const ACTIVATE_AAA_AS_IU      = 0x800000;
        const PS_DLL                  = 0x80000000;
    }
}

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
            Err(ErrorKind::ComCallFailed(*self).into())
        }
    }
}

impl fmt::Display for HRESULT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HRESULT(0x{:08x})", self.0)
    }
}

pub type BOOL = i32;

pub type HWND = *const c_void;

pub type ULONG = u32;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl fmt::Display for GUID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.data1,
            self.data2,
            self.data3,
            self.data4[0],
            self.data4[1],
            self.data4[2],
            self.data4[3],
            self.data4[4],
            self.data4[5],
            self.data4[6],
            self.data4[7]
        )
    }
}

pub type IID = GUID;
pub type CLSID = GUID;

pub type RawComPtr = *const c_void;
