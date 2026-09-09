/* SPDX-License-Identifier: GPL-2.0 */
// Translated from the m68k string header.
// The original header includes linux/types.h and linux/compiler.h.

pub const __HAVE_ARCH_STRNLEN: bool = true;

/// Return the length of the string, limited to `count` bytes.
///
/// This preserves the original raw-pointer and byte-scanning behavior.
#[inline]
pub unsafe fn strnlen(s: *const core::ffi::c_char, mut count: usize) -> usize {
    let mut sc = s;

    while count != 0 {
        count = count.wrapping_sub(1);
        let byte = *(sc as *const u8);
        sc = sc.add(1);
        if byte == 0 {
            sc = sc.sub(1);
            break;
        }
    }

    sc.offset_from(s) as usize
}

pub const __HAVE_ARCH_MEMMOVE: bool = true;

unsafe extern "C" {
    pub fn memmove(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: usize,
    ) -> *mut core::ffi::c_void;

    pub fn memcmp(
        dest: *const core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: usize,
    ) -> core::ffi::c_int;

    pub fn memset(
        dest: *mut core::ffi::c_void,
        c: core::ffi::c_int,
        n: usize,
    ) -> *mut core::ffi::c_void;

    pub fn memcpy(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: usize,
    ) -> *mut core::ffi::c_void;
}

pub const __HAVE_ARCH_MEMSET: bool = true;
pub const __HAVE_ARCH_MEMCPY: bool = true;

// The C macros redirect memcmp, memset, and memcpy to compiler builtins.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
