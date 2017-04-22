use types::*;
use comptr::ComPtr;
use iunknown::{IUnknown, ComInterface};
use errors::*;

use std::ptr;
use libc::c_void;

#[link(name = "ole32")]
extern "system" {
    pub fn CoInitializeEx(reserved: *const c_void, coinit: COINIT) -> HRESULT;
    pub fn CoUninitialize();

    fn CoCreateInstance(clsid: *const CLSID,
                        unk_outer: RawComPtr,
                        cls_context: CLSCTX,
                        iid: *const IID,
                        v: *mut RawComPtr)
                        -> HRESULT;
}

pub unsafe fn raw_to_comptr<T: ComInterface>(ptr: RawComPtr, owned: bool) -> ComPtr<T> {
    let interface_ptr: *const T = ptr as *const T;
    let result = ComPtr::from_raw(interface_ptr);
    if !owned {
        result.as_ref().add_ref();
    }
    result
}

pub fn create_instance<U>(clsid: &CLSID,
                          unk_outer: Option<&IUnknown>,
                          cls_context: CLSCTX)
                          -> Result<ComPtr<U>>
    where U: ComInterface
{
    let mut ptr: RawComPtr = ptr::null();
    let outer: *const IUnknown = if let Some(x) = unk_outer {
        x
    } else {
        ptr::null()
    };
    let rc =
        unsafe { CoCreateInstance(clsid, outer as RawComPtr, cls_context, &U::iid(), &mut ptr) };

    try!(rc.result());

    unsafe { Ok(raw_to_comptr(ptr, true)) }
}

pub fn query_interface<U: ComInterface>(unk: &IUnknown) -> Result<ComPtr<U>> {
    let mut ptr: RawComPtr = ptr::null();

    let rc = unsafe { unk.query_interface(&U::iid(), &mut ptr) };

    try!(rc.result());

    unsafe { Ok(raw_to_comptr(ptr, true)) }
}