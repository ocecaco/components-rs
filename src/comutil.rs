use comptr::ComPtr;
use errors::*;
use iunknown::{ComInterface, IUnknown};
use std::ptr;
use types::*;

use winapi::um::combaseapi::{CoCreateInstance, CoInitializeEx, CoTaskMemFree};
use winapi::um::objbase::COINIT_MULTITHREADED;

pub fn com_initialize() -> Result<()> {
    let rc = unsafe { CoInitializeEx(ptr::null_mut(), COINIT_MULTITHREADED) };
    try!(HRESULT(rc as u32).result());

    Ok(())
}

pub unsafe fn com_memory_free(ptr: LPVOID) {
    CoTaskMemFree(ptr)
}

pub unsafe fn raw_to_comptr<T: ComInterface>(ptr: LPVOID, owned: bool) -> ComPtr<T> {
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
    let mut ptr: LPVOID = ptr::null_mut();
    let outer: *const IUnknown = if let Some(x) = unk_outer {
        x
    } else {
        ptr::null()
    };
    let rc =
        unsafe { CoCreateInstance(clsid, outer as RawComPtr, cls_context, &U::iid(), &mut ptr) };

    try!(HRESULT(rc as u32).result());

    unsafe { Ok(raw_to_comptr(ptr, true)) }
}
