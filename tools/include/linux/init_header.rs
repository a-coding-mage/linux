/* SPDX-License-Identifier: GPL-2.0 */

/* C include dependency preserved for context: <linux/compiler.h>. */

/* __init and __exit are empty annotations in this header when not predefined. */
pub const __init: () = ();
pub const __exit: () = ();

/*
 * C attribute helpers and section/data annotations from this header.
 * __section(section) maps to Rust's link_section attribute at each use site.
 */
pub const __initconst: () = ();
pub const __meminit: () = ();
pub const __meminitdata: () = ();
pub const __refdata: () = ();
pub const __initdata: () = ();

#[repr(C)]
pub struct obs_kernel_param {
    pub str: *const ::core::ffi::c_char,
    pub setup_func: Option<unsafe extern "C" fn(st: *mut ::core::ffi::c_char) -> ::core::ffi::c_int>,
    pub early: ::core::ffi::c_int,
}

/*
 * Translation of:
 *
 *   __setup_param(str, unique_id, fn, early)
 *
 * The C macro creates one private static string and one private static
 * obs_kernel_param object in the ".init.setup" section. The string literal is
 * represented as bytes with an explicit trailing NUL to match C char[] string
 * initialization.
 */
#[macro_export]
macro_rules! __setup_param {
    ($str:literal, $unique_id:ident, $fn:path, $early:expr) => {
        const _: &[u8] = concat!($str, "\0").as_bytes();

        #[used]
        #[unsafe(link_section = ".init.setup")]
        static $unique_id: $crate::obs_kernel_param = $crate::obs_kernel_param {
            str: concat!($str, "\0").as_ptr() as *const ::core::ffi::c_char,
            setup_func: Some($fn),
            early: $early as ::core::ffi::c_int,
        };
    };
}

#[macro_export]
macro_rules! __setup {
    ($str:literal, $fn:path) => {
        $crate::__setup_param!($str, $fn, $fn, 0)
    };
}

#[macro_export]
macro_rules! early_param {
    ($str:literal, $fn:path) => {
        $crate::__setup_param!($str, $fn, $fn, 1)
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
