use core::ffi::{c_char, c_int, c_uint};
use core::ptr;

#[repr(C)]
pub struct srccode_state {
    pub srcfile: *mut c_char,
    pub line: c_uint,
}

#[inline]
pub unsafe fn srccode_state_init(state: *mut srccode_state) {
    unsafe {
        (*state).srcfile = ptr::null_mut();
        (*state).line = 0;
    }
}

unsafe extern "C" {
    pub fn srccode_state_free(state: *mut srccode_state);

    /* Result is not 0 terminated */
    pub fn find_sourceline(fn_: *mut c_char, line: c_uint, lenp: *mut c_int) -> *mut c_char;
}
