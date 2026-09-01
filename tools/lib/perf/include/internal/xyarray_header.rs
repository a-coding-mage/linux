/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_int, c_void};
use core::ptr;

#[repr(C, align(8))]
pub struct xyarray_aligned_char {
    pub value: i8,
}

#[repr(C)]
pub struct xyarray {
    pub row_size: usize,
    pub entry_size: usize,
    pub entries: usize,
    pub max_x: usize,
    pub max_y: usize,
    pub contents: [xyarray_aligned_char; 0],
}

unsafe extern "C" {
    pub fn xyarray__new(xlen: c_int, ylen: c_int, entry_size: usize) -> *mut xyarray;
    pub fn xyarray__delete(xy: *mut xyarray);
    pub fn xyarray__reset(xy: *mut xyarray);
}

#[inline]
pub unsafe fn __xyarray__entry(xy: *mut xyarray, x: c_int, y: c_int) -> *mut c_void {
    unsafe {
        (&mut (*xy).contents as *mut [xyarray_aligned_char; 0] as *mut u8)
            .add((x as usize).wrapping_mul((*xy).row_size)
                .wrapping_add((y as usize).wrapping_mul((*xy).entry_size)))
            as *mut c_void
    }
}

#[inline]
pub unsafe fn xyarray__entry(xy: *mut xyarray, x: usize, y: usize) -> *mut c_void {
    unsafe {
        if x >= (*xy).max_x || y >= (*xy).max_y {
            return ptr::null_mut();
        }
        __xyarray__entry(xy, x as c_int, y as c_int)
    }
}

#[inline]
pub unsafe fn xyarray__max_y(xy: *mut xyarray) -> c_int {
    unsafe { (*xy).max_y as c_int }
}

#[inline]
pub unsafe fn xyarray__max_x(xy: *mut xyarray) -> c_int {
    unsafe { (*xy).max_x as c_int }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
