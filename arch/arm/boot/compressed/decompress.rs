// SPDX-License-Identifier: GPL-2.0

// C headers removed; their supplied declarations and definitions remain external.

// #define STATIC static
// #define STATIC_RW_DATA /* non-static please */

/* Diagnostic functions. The DEBUG-dependent C macros are intentionally empty
 * in this translation unless provided by the surrounding build. */

use core::ffi::c_char;
use core::ffi::c_int;
use core::ffi::c_void;

// Not needed, but used in some headers pulled in by decompressors.
extern "C" {
    pub fn strstr(s1: *const c_char, s2: *const c_char) -> *mut c_char;
    pub fn strlen(s: *const c_char) -> usize;
    pub fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    pub fn memcmp(cs: *const c_void, ct: *const c_void, count: usize) -> c_int;
    pub fn strchrnul(s: *const c_char, c: c_int) -> *mut c_char;
}

// Under the corresponding build-time configuration, these C decompressor
// sources are included here and provide __decompress.
// #ifdef CONFIG_KERNEL_GZIP
// #include "../../../../lib/decompress_inflate.c"
// #endif
// #ifdef CONFIG_KERNEL_LZO
// #include "../../../../lib/decompress_unlzo.c"
// #endif
// #ifdef CONFIG_KERNEL_LZMA
// #include "../../../../lib/decompress_unlzma.c"
// #endif
// #ifdef CONFIG_KERNEL_XZ
// /* Prevent KASAN override of string helpers in decompressor */
// #undef memmove
// #define memmove memmove
// #undef memcpy
// #define memcpy memcpy
// #include "../../../../lib/decompress_unxz.c"
// #endif
// #ifdef CONFIG_KERNEL_LZ4
// #include "../../../../lib/decompress_unlz4.c"
// #endif

extern "C" {
    fn __decompress(
        input: *mut u8,
        len: c_int,
        fill: *mut c_void,
        flush: *mut c_void,
        output: *mut u8,
        pos: c_int,
        error: *mut c_void,
        error_fn: Option<unsafe extern "C" fn(*mut c_char)>,
    ) -> c_int;
}

pub unsafe extern "C" fn do_decompress(
    input: *mut u8,
    len: c_int,
    output: *mut u8,
    error: Option<unsafe extern "C" fn(*mut c_char)>,
) -> c_int {
    __decompress(
        input,
        len,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        output,
        0,
        core::ptr::null_mut(),
        error,
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
