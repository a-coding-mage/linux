// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
/*
 * libfdt - Flat Device Tree manipulation
 * Copyright (C) 2006 David Gibson, IBM Corporation.
 *     EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

// The declarations and constants below are supplied by libfdt's headers.

#[repr(C)]
struct FdtErrtabent {
    str_: *const core::ffi::c_char,
}

static FDT_ERRTABLE: [FdtErrtabent; FDT_ERRTABSIZE as usize] = [
    FdtErrtabent { str_: core::ptr::null() },
    FdtErrtabent { str_: b"FDT_ERR_NOTFOUND\0".as_ptr() as *const core::ffi::c_char },
    FdtErrtabent { str_: b"FDT_ERR_EXISTS\0".as_ptr() as *const core::ffi::c_char },
    FdtErrtabent { str_: b"FDT_ERR_NOSPACE\0".as_ptr() as *const core::ffi::c_char },
    FdtErrtabent { str_: b"FDT_ERR_BADOFFSET\0".as_ptr() as *const core::ffi::c_char },
    FdtErrtabent { str_: b"FDT_ERR_BADPATH\0".as_ptr() as *const core::ffi::c_char },
    FdtErrtabent { str_: b"FDT_ERR_BADPHANDLE\0".as_ptr() as *const core::ffi::c_char },
    FdtErrtabent { str_: b"FDT_ERR_BADSTATE\0".as_ptr() as *const core::ffi::c_char },
    FdtErrtabent { str_: b"FDT_ERR_TRUNCATED\0".as_ptr() as *const core::ffi::c_char },
    FdtErrtabent { str_: b"FDT_ERR_BADMAGIC\0".as_ptr() as *const core::ffi::c_char },
    FdtErrtabent { str_: b"FDT_ERR_BADVERSION\0".as_ptr() as *const core::ffi::c_char },
    FdtErrtabent { str_: b"FDT_ERR_BADSTRUCTURE\0".as_ptr() as *const core::ffi::c_char },
    FdtErrtabent { str_: b"FDT_ERR_BADLAYOUT\0".as_ptr() as *const core::ffi::c_char },
    FdtErrtabent { str_: b"FDT_ERR_INTERNAL\0".as_ptr() as *const core::ffi::c_char },
    FdtErrtabent { str_: b"FDT_ERR_BADNCELLS\0".as_ptr() as *const core::ffi::c_char },
    FdtErrtabent { str_: b"FDT_ERR_BADVALUE\0".as_ptr() as *const core::ffi::c_char },
    FdtErrtabent { str_: b"FDT_ERR_BADOVERLAY\0".as_ptr() as *const core::ffi::c_char },
    FdtErrtabent { str_: b"FDT_ERR_NOPHANDLES\0".as_ptr() as *const core::ffi::c_char },
    FdtErrtabent { str_: b"FDT_ERR_BADFLAGS\0".as_ptr() as *const core::ffi::c_char },
    FdtErrtabent { str_: b"FDT_ERR_ALIGNMENT\0".as_ptr() as *const core::ffi::c_char },
];

const FDT_ERRTABSIZE: i32 = 20;

pub unsafe fn fdt_strerror(errval: i32) -> *const core::ffi::c_char {
    if errval > 0 {
        b"<valid offset/length>\0".as_ptr() as *const core::ffi::c_char
    } else if errval == 0 {
        b"<no error>\0".as_ptr() as *const core::ffi::c_char
    } else if -errval < FDT_ERRTABSIZE {
        let s = FDT_ERRTABLE[(-errval) as usize].str_;

        if !s.is_null() {
            return s;
        }

        b"<unknown error>\0".as_ptr() as *const core::ffi::c_char
    } else {
        b"<unknown error>\0".as_ptr() as *const core::ffi::c_char
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
