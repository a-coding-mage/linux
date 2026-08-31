// SPDX-License-Identifier: GPL-2.0
// C dependencies: <internal/xyarray.h>, <linux/zalloc.h>, <stdlib.h>, <string.h>

use core::ffi::{c_int, c_void};

pub type size_t = usize;

#[repr(C)]
pub struct xyarray {
    pub entry_size: size_t,
    pub row_size: size_t,
    pub entries: size_t,
    pub max_x: c_int,
    pub max_y: c_int,
    pub contents: [u8; 0],
}

unsafe extern "C" {
    fn zalloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xyarray__new(
    xlen: c_int,
    ylen: c_int,
    entry_size: size_t,
) -> *mut xyarray {
    let row_size: size_t = (ylen as size_t).wrapping_mul(entry_size);
    let xy: *mut xyarray = unsafe {
        zalloc(
            core::mem::size_of::<xyarray>()
                .wrapping_add((xlen as size_t).wrapping_mul(row_size)),
        ) as *mut xyarray
    };

    if !xy.is_null() {
        unsafe {
            (*xy).entry_size = entry_size;
            (*xy).row_size = row_size;
            (*xy).entries = (xlen as size_t).wrapping_mul(ylen as size_t);
            (*xy).max_x = xlen;
            (*xy).max_y = ylen;
        }
    }

    xy
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xyarray__reset(xy: *mut xyarray) {
    let n: size_t = unsafe { (*xy).entries.wrapping_mul((*xy).entry_size) };

    unsafe {
        memset((*xy).contents.as_mut_ptr() as *mut c_void, 0, n);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xyarray__delete(xy: *mut xyarray) {
    unsafe {
        free(xy as *mut c_void);
    }
}
