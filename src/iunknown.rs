use types::*;

define_guid!(IID_IUnknown = 0x00000000,
             0x0000,
             0x0000,
             0xC0,
             0x00,
             0x00,
             0x00,
             0x00,
             0x00,
             0x00,
             0x46);

#[repr(C)]
pub struct IUnknown {
    vtable: *const IUnknownVtable,
}

#[repr(C)]
#[derive(Copy)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub struct IUnknownVtable {
    pub query_interface: extern "stdcall" fn(*const IUnknown,
                                             *const IID,
                                             *mut RawComPtr) -> HRESULT,
    pub add_ref: extern "stdcall" fn(*const IUnknown) -> ULONG,
    pub release: extern "stdcall" fn(*const IUnknown) -> ULONG,
}

impl Clone for IUnknownVtable {
    fn clone(&self) -> Self {
        IUnknownVtable {
            query_interface: self.query_interface,
            add_ref: self.add_ref,
            release: self.release,
        }
    }
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

impl AsRef<IUnknown> for IUnknown {
    fn as_ref(&self) -> &IUnknown {
        self
    }
}

// unsafe to implement because it implies the type can safely be cast to IUnknown
pub unsafe trait ComInterface: AsRef<IUnknown> + Send + Sync {
    type Vtable: Copy + Clone;

    fn iid() -> IID;
}
