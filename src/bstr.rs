use std::ptr;

#[link(name = "oleaut32")]
extern "system" {
    fn SysAllocStringLen(s: *const u16, len: u32) -> *const u16;
    fn SysFreeString(s: *const u16);
}

pub struct BString {
    ptr: *const u16,
}

#[repr(C)]
pub struct BStr {
    ptr: *const u16,
}

impl BString {
    pub fn as_ref(&self) -> BStr {
        BStr { ptr: self.ptr }
    }
}

impl BStr {
    pub fn null() -> Self {
        BStr { ptr: ptr::null() }
    }
}

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
