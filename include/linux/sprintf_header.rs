/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies:
//   linux/compiler_attributes.h
//   linux/types.h
//   linux/stdarg.h
// The corresponding externally supplied Rust types are referenced below.

unsafe extern "C" {
    pub fn num_to_str(
        buf: *mut ::kernel::ffi::c_char,
        size: ::kernel::ffi::c_int,
        num: u64,
        width: u32,
    ) -> ::kernel::ffi::c_int;

    // C __printf(2, 3)
    pub fn sprintf(
        buf: *mut ::kernel::ffi::c_char,
        fmt: *const ::kernel::ffi::c_char,
        ...
    ) -> ::kernel::ffi::c_int;
    // C __printf(2, 0)
    pub fn vsprintf(
        buf: *mut ::kernel::ffi::c_char,
        fmt: *const ::kernel::ffi::c_char,
        args: va_list,
    ) -> ::kernel::ffi::c_int;
    // C __printf(3, 4)
    pub fn snprintf(
        buf: *mut ::kernel::ffi::c_char,
        size: usize,
        fmt: *const ::kernel::ffi::c_char,
        ...
    ) -> ::kernel::ffi::c_int;
    // C __printf(3, 0)
    pub fn vsnprintf(
        buf: *mut ::kernel::ffi::c_char,
        size: usize,
        fmt: *const ::kernel::ffi::c_char,
        args: va_list,
    ) -> ::kernel::ffi::c_int;
    // C __printf(3, 4)
    pub fn scnprintf(
        buf: *mut ::kernel::ffi::c_char,
        size: usize,
        fmt: *const ::kernel::ffi::c_char,
        ...
    ) -> ::kernel::ffi::c_int;
    // C __printf(3, 0)
    pub fn vscnprintf(
        buf: *mut ::kernel::ffi::c_char,
        size: usize,
        fmt: *const ::kernel::ffi::c_char,
        args: va_list,
    ) -> ::kernel::ffi::c_int;
    // C __printf(2, 3), __malloc
    pub fn kasprintf(
        gfp: gfp_t,
        fmt: *const ::kernel::ffi::c_char,
        ...
    ) -> *mut ::kernel::ffi::c_char;
    // C __printf(2, 0), __malloc
    pub fn kvasprintf(
        gfp: gfp_t,
        fmt: *const ::kernel::ffi::c_char,
        args: va_list,
    ) -> *mut ::kernel::ffi::c_char;
    // C __printf(2, 0)
    pub fn kvasprintf_const(
        gfp: gfp_t,
        fmt: *const ::kernel::ffi::c_char,
        args: va_list,
    ) -> *const ::kernel::ffi::c_char;

    // C __scanf(2, 3)
    pub fn sscanf(
        input: *const ::kernel::ffi::c_char,
        fmt: *const ::kernel::ffi::c_char,
        ...
    ) -> ::kernel::ffi::c_int;
    // C __scanf(2, 0)
    pub fn vsscanf(
        input: *const ::kernel::ffi::c_char,
        fmt: *const ::kernel::ffi::c_char,
        args: va_list,
    ) -> ::kernel::ffi::c_int;

    /* These are for specific cases, do not use without real need */
    pub static mut no_hash_pointers: bool;
    pub fn hash_pointers_finalize(slub_debug: bool);

    /* Used for Rust formatting ('%pA') */
    pub fn rust_fmt_argument(
        buf: *mut ::kernel::ffi::c_char,
        end: *mut ::kernel::ffi::c_char,
        ptr: *const ::kernel::ffi::c_void,
    ) -> *mut ::kernel::ffi::c_char;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
