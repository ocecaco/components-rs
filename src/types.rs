#![allow(non_camel_case_types)]

use libc::c_void;
use std::fmt;

bitflags! {
    #[repr(C)]
    pub flags COINIT: u32 {
        const COINIT_APARTMENTTHREADED  = 0x2,
        const COINIT_MULTITHREADED      = 0x0,
        const COINIT_DISABLE_OLE1DDE    = 0x4,
        const COINIT_SPEED_OVER_MEMORY  = 0x8
    }
}

bitflags! {
    #[repr(C)]
    pub flags CLSCTX: u32 {
        const CLSCTX_INPROC_SERVER           = 0x1,
        const CLSCTX_INPROC_HANDLER          = 0x2,
        const CLSCTX_LOCAL_SERVER            = 0x4,
        const CLSCTX_INPROC_SERVER16         = 0x8,
        const CLSCTX_REMOTE_SERVER           = 0x10,
        const CLSCTX_INPROC_HANDLER16        = 0x20,
        const CLSCTX_RESERVED1               = 0x40,
        const CLSCTX_RESERVED2               = 0x80,
        const CLSCTX_RESERVED3               = 0x100,
        const CLSCTX_RESERVED4               = 0x200,
        const CLSCTX_NO_CODE_DOWNLOAD        = 0x400,
        const CLSCTX_RESERVED5               = 0x800,
        const CLSCTX_NO_CUSTOM_MARSHAL       = 0x1000,
        const CLSCTX_ENABLE_CODE_DOWNLOAD    = 0x2000,
        const CLSCTX_NO_FAILURE_LOG          = 0x4000,
        const CLSCTX_DISABLE_AAA             = 0x8000,
        const CLSCTX_ENABLE_AAA              = 0x10000,
        const CLSCTX_FROM_DEFAULT_CONTEXT    = 0x20000,
        const CLSCTX_ACTIVATE_32_BIT_SERVER  = 0x40000,
        const CLSCTX_ACTIVATE_64_BIT_SERVER  = 0x80000,
        const CLSCTX_ENABLE_CLOAKING         = 0x100000,
        const CLSCTX_APPCONTAINER            = 0x400000,
        const CLSCTX_ACTIVATE_AAA_AS_IU      = 0x800000,
        const CLSCTX_PS_DLL                  = 0x80000000
    }
}

#[must_use]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct HRESULT(pub u32);
pub const E_NOINTERFACE: HRESULT = HRESULT(0x80004002);
pub const E_POINTER: HRESULT = HRESULT(0x80004003);

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
        write!(f,
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
               self.data4[7])
    }
}

pub type IID = GUID;
pub type CLSID = GUID;

pub type RawComPtr = *const c_void;
