use comptr::*;
use comutil::*;
use errors::*;
use std::ptr;
use types::*;

define_guid!(
    IID_IUnknown = 0x00000000,
    0x0000,
    0x0000,
    0xC0,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x46
);

#[repr(C)]
pub struct IUnknown {
    vtable: *const IUnknownVtable,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub struct IUnknownVtable {
    pub query_interface: extern "system" fn(*const IUnknown,
                                             *const IID,
                                             *mut RawComPtr) -> HRESULT,
    pub add_ref: extern "system" fn(*const IUnknown) -> ULONG,
    pub release: extern "system" fn(*const IUnknown) -> ULONG,
}

impl IUnknown {
    pub unsafe fn query_interface(&self, iid: *const IID, v: *mut RawComPtr) -> HRESULT {
        ((*self.vtable).query_interface)(self, iid, v)
    }

    pub unsafe fn add_ref(&self) -> ULONG {
        ((*self.vtable).add_ref)(self)
    }

    pub unsafe fn release(&self) -> ULONG {
        ((*self.vtable).release)(self)
    }
}

unsafe impl Send for IUnknown {}
unsafe impl Sync for IUnknown {}

unsafe impl ComInterface for IUnknown {
    type Vtable = IUnknownVtable;

    fn iid() -> IID {
        IID_IUnknown
    }
}

// unsafe to implement because it implies the type can safely be cast to IUnknown
pub unsafe trait ComInterface: Send + Sync {
    type Vtable: Copy + Clone;

    fn iid() -> IID;

    fn iunknown(&self) -> &IUnknown {
        let result: *const IUnknown = self as *const _ as *const IUnknown;
        unsafe { &*result }
    }
}

pub trait Upcast: ComInterface {
    type Target;
    fn upcast(&self) -> &Self::Target;
}

fn query_interface<T: ComInterface, U: ComInterface>(unk: &T) -> Result<ComPtr<U>> {
    let mut ptr: RawComPtr = ptr::null_mut();

    let rc = unsafe { unk.iunknown().query_interface(&U::iid(), &mut ptr) };

    try!(rc.result());

    unsafe { Ok(raw_to_comptr(ptr, true)) }
}

pub trait Cast: ComInterface {
    fn cast<U: ComInterface>(&self) -> Result<ComPtr<U>>;
}

impl<T: ComInterface> Cast for T {
    fn cast<U: ComInterface>(&self) -> Result<ComPtr<U>> {
        query_interface(self)
    }
}
