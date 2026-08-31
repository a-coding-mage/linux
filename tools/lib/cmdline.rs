// SPDX-License-Identifier: GPL-2.0-only
/*
 * From lib/cmdline.c
 */

use core::ffi::{c_char, c_int, c_longlong};

unsafe extern "C" {
    fn strtoll(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_longlong;
}

#[no_mangle]
pub unsafe extern "C" fn memparse(ptr: *const c_char, retptr: *mut *mut c_char) -> u64 {
    let mut endptr: *mut c_char = core::ptr::null_mut(); /* local pointer to end of parsed string */

    let mut ret: u64 = unsafe { strtoll(ptr, &mut endptr, 0) as u64 };

    match unsafe { *endptr } {
        c if c == b'E' as c_char || c == b'e' as c_char => {
            ret = ret.wrapping_shl(10);
            ret = ret.wrapping_shl(10);
            ret = ret.wrapping_shl(10);
            ret = ret.wrapping_shl(10);
            ret = ret.wrapping_shl(10);
            ret = ret.wrapping_shl(10);
            endptr = unsafe { endptr.add(1) };
        }
        c if c == b'P' as c_char || c == b'p' as c_char => {
            ret = ret.wrapping_shl(10);
            ret = ret.wrapping_shl(10);
            ret = ret.wrapping_shl(10);
            ret = ret.wrapping_shl(10);
            ret = ret.wrapping_shl(10);
            endptr = unsafe { endptr.add(1) };
        }
        c if c == b'T' as c_char || c == b't' as c_char => {
            ret = ret.wrapping_shl(10);
            ret = ret.wrapping_shl(10);
            ret = ret.wrapping_shl(10);
            ret = ret.wrapping_shl(10);
            endptr = unsafe { endptr.add(1) };
        }
        c if c == b'G' as c_char || c == b'g' as c_char => {
            ret = ret.wrapping_shl(10);
            ret = ret.wrapping_shl(10);
            ret = ret.wrapping_shl(10);
            endptr = unsafe { endptr.add(1) };
        }
        c if c == b'M' as c_char || c == b'm' as c_char => {
            ret = ret.wrapping_shl(10);
            ret = ret.wrapping_shl(10);
            endptr = unsafe { endptr.add(1) };
        }
        c if c == b'K' as c_char || c == b'k' as c_char => {
            ret = ret.wrapping_shl(10);
            endptr = unsafe { endptr.add(1) };
        }
        _ => {}
    }

    if !retptr.is_null() {
        unsafe {
            *retptr = endptr;
        }
    }

    ret
}
