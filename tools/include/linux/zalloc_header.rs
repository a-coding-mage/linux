// SPDX-License-Identifier: LGPL-2.1

pub type size_t = usize;

unsafe extern "C" {
    pub fn zalloc(size: size_t) -> *mut core::ffi::c_void;
    pub fn __zfree(ptr: *mut *mut core::ffi::c_void);
}

#[inline]
pub unsafe fn zfree<T>(ptr: *mut *mut T) {
    unsafe {
        __zfree(ptr as *mut *mut core::ffi::c_void);
    }
}
