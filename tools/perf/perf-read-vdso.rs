// SPDX-License-Identifier: GPL-2.0
// C dependencies: <stdio.h>, <string.h>

const VDSO__MAP_NAME: &[u8] = b"[vdso]\0";

/*
 * Include definition of find_map() also used in util/vdso.c for
 * building perf.
 */
// C included "util/find-map.c"; provided externally in this translation unit.

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut stdout: *mut FILE;

    fn fwrite(ptr: *const core::ffi::c_void, size: usize, nmemb: usize, stream: *mut FILE) -> usize;
    fn fflush(stream: *mut FILE) -> core::ffi::c_int;
    fn find_map(
        start: *mut *mut core::ffi::c_void,
        end: *mut *mut core::ffi::c_void,
        name: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
}

fn main() -> core::ffi::c_int {
    let mut start: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut end: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut size: usize;
    let mut written: usize;

    unsafe {
        if find_map(
            &mut start,
            &mut end,
            VDSO__MAP_NAME.as_ptr() as *const core::ffi::c_char,
        ) != 0
        {
            return 1;
        }

        size = (end as *mut u8).offset_from(start as *mut u8) as usize;

        while size != 0 {
            written = fwrite(start as *const core::ffi::c_void, 1, size, stdout);
            if written == 0 {
                return 1;
            }
            start = (start as *mut u8).add(written) as *mut core::ffi::c_void;
            size -= written;
        }

        if fflush(stdout) != 0 {
            return 1;
        }
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
