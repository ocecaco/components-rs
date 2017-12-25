use comptr::ComPtr;
use errors::*;
use iunknown::{ComInterface, IUnknown};
use libc::c_void;
use std::ptr;
use types::*;

#[link(name = "ole32")]
extern "system" {
    fn CoInitializeEx(reserved: *const c_void, coinit: COINIT) -> HRESULT;
    // fn CoUninitialize();

    fn CoCreateInstance(
        clsid: *const CLSID,
        unk_outer: RawComPtr,
        cls_context: CLSCTX,
        iid: *const IID,
        v: *mut RawComPtr,
    ) -> HRESULT;

    fn CoTaskMemFree(ptr: *const c_void);
}

pub fn com_initialize() -> Result<()> {
    let rc = unsafe { CoInitializeEx(ptr::null(), COINIT_MULTITHREADED) };
    try!(rc.result());

    Ok(())
}

pub unsafe fn com_memory_free(ptr: *const c_void) {
    CoTaskMemFree(ptr)
}

pub unsafe fn raw_to_comptr<T: ComInterface>(ptr: RawComPtr, owned: bool) -> ComPtr<T> {
    let interface_ptr: *const T = ptr as *const T;
    let result = ComPtr::from_raw(interface_ptr);
    if !owned {
        result.iunknown().add_ref();
    }
    result
}

pub fn create_instance<U>(
    clsid: &CLSID,
    unk_outer: Option<&IUnknown>,
    cls_context: CLSCTX,
) -> Result<ComPtr<U>>
where
    U: ComInterface,
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
