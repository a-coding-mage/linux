/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies:
//   linux/compiler_attributes.h
//   linux/types.h
//   linux/stdarg.h
// The corresponding externally supplied Rust types are referenced below.

unsafe extern "C" {
    pub fn num_to_str(
        buf: *mut ::core::ffi::c_char,
        size: ::core::ffi::c_int,
        num: u64,
        width: u32,
    ) -> ::core::ffi::c_int;

    // C __printf(2, 3)
    pub fn sprintf(
        buf: *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    // C __printf(2, 0)
    pub fn vsprintf(
        buf: *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        args: va_list,
    ) -> ::core::ffi::c_int;
    // C __printf(3, 4)
    pub fn snprintf(
        buf: *mut ::core::ffi::c_char,
        size: usize,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    // C __printf(3, 0)
    pub fn vsnprintf(
        buf: *mut ::core::ffi::c_char,
        size: usize,
        fmt: *const ::core::ffi::c_char,
        args: va_list,
    ) -> ::core::ffi::c_int;
    // C __printf(3, 4)
    pub fn scnprintf(
        buf: *mut ::core::ffi::c_char,
        size: usize,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    // C __printf(3, 0)
    pub fn vscnprintf(
        buf: *mut ::core::ffi::c_char,
        size: usize,
        fmt: *const ::core::ffi::c_char,
        args: va_list,
    ) -> ::core::ffi::c_int;
    // C __printf(2, 3), __malloc
    pub fn kasprintf(
        gfp: gfp_t,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    // C __printf(2, 0), __malloc
    pub fn kvasprintf(
        gfp: gfp_t,
        fmt: *const ::core::ffi::c_char,
        args: va_list,
    ) -> *mut ::core::ffi::c_char;
    // C __printf(2, 0)
    pub fn kvasprintf_const(
        gfp: gfp_t,
        fmt: *const ::core::ffi::c_char,
        args: va_list,
    ) -> *const ::core::ffi::c_char;

    // C __scanf(2, 3)
    pub fn sscanf(
        input: *const ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    // C __scanf(2, 0)
    pub fn vsscanf(
        input: *const ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        args: va_list,
    ) -> ::core::ffi::c_int;

    /* These are for specific cases, do not use without real need */
    pub static mut no_hash_pointers: bool;
    pub fn hash_pointers_finalize(slub_debug: bool);

    /* Used for Rust formatting ('%pA') */
    pub fn rust_fmt_argument(
        buf: *mut ::core::ffi::c_char,
        end: *mut ::core::ffi::c_char,
        ptr: *const ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_char;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
