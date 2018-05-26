use std::marker::PhantomData;
use std::ptr;

use winapi::shared::wtypes::BSTR;
use winapi::um::oleauto::{SysAllocStringLen, SysFreeString};

pub struct BString {
    ptr: BSTR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BStr<'a> {
    ptr: BSTR,
    phantom: PhantomData<&'a u16>,
}

impl<'a> BStr<'a> {
    pub fn null() -> BStr<'static> {
        BStr {
            ptr: ptr::null_mut(),
            phantom: PhantomData,
        }
    }
}

impl BString {
    pub fn as_ref<'a>(&'a self) -> BStr<'a> {
        BStr {
            ptr: self.ptr,
            phantom: PhantomData,
        }
    }
}

unsafe impl Send for BString {}
unsafe impl Sync for BString {}

impl Drop for BString {
    fn drop(&mut self) {
        unsafe {
            SysFreeString(self.ptr);
        }
    }
}

impl<'a> From<&'a str> for BString {
    fn from(s: &'a str) -> BString {
        let encoded: Vec<u16> = s.encode_utf16().collect();
        let slice = encoded.as_slice();

        let ptr = unsafe { SysAllocStringLen(slice.as_ptr(), slice.len() as u32) };

        if ptr.is_null() {
            panic!("SysAllocStringLen failed to allocate memory")
        }

        BString { ptr: ptr }
    }
}
