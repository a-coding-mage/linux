// SPDX-License-Identifier: GPL-2.0
// C dependencies: <stdlib.h>, <gelf.h>

use core::ffi::c_void;
use core::ptr::null_mut;

unsafe extern "C" {
    fn gelf_getnote(
        data: *mut c_void,
        offset: usize,
        result: *mut c_void,
        name_offset: *mut usize,
        desc_offset: *mut usize,
    ) -> usize;
}

#[no_mangle]
pub unsafe extern "C" fn main() -> i32 {
    (unsafe {
        gelf_getnote(
            null_mut(),
            0,
            null_mut(),
            null_mut(),
            null_mut(),
        )
    } == 0) as i32
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
