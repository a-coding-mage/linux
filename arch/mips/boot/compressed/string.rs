// SPDX-License-Identifier: GPL-2.0
/*
 * arch/mips/boot/compressed/string.c
 *
 * Very small subset of simple string routines
 */

// Dependencies supplied by the surrounding kernel build provide the C ABI
// types and weak-linkage behavior represented by this file.

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut core::ffi::c_void,
                                src: *const core::ffi::c_void,
                                n: usize) -> *mut core::ffi::c_void
{
    let s = src as *const u8;
    let d = dest as *mut u8;

    let mut i: usize = 0;
    while i < n {
        *d.add(i) = *s.add(i);
        i += 1;
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut core::ffi::c_void,
                                c: core::ffi::c_int,
                                n: usize) -> *mut core::ffi::c_void
{
    let ss = s as *mut u8;

    let mut i: usize = 0;
    while i < n {
        *ss.add(i) = c as u8;
        i += 1;
    }
    s
}

// The C declaration uses __weak; weak linkage is supplied by the target
// build system for this externally visible implementation.
#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut core::ffi::c_void,
                                  src: *const core::ffi::c_void,
                                  n: usize) -> *mut core::ffi::c_void
{
    let s = src as *const u8;
    let d = dest as *mut u8;

    if (dest as usize) < (src as usize) {
        let mut i: usize = 0;
        while i < n {
            *d.add(i) = *s.add(i);
            i += 1;
        }
    } else {
        let mut i: usize = n;
        while i > 0 {
            i -= 1;
            *d.add(i) = *s.add(i);
        }
    }
    dest
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
