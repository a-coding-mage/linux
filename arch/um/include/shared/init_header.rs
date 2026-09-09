/* SPDX-License-Identifier: GPL-2.0 */

/*
 * These macros mark functions and initialized data as belonging to the
 * initialization or exit phase.  The original section attributes are kept
 * here as comments; downstream platform/linker integration supplies them.
 */

pub type InitcallT = unsafe extern "C" fn() -> ::core::ffi::c_int;
pub type ExitcallT = unsafe extern "C" fn();

#[repr(C)]
pub struct UmlParam {
    pub str_: *const ::core::ffi::c_char,
    pub setup_func: Option<unsafe extern "C" fn(
        *mut ::core::ffi::c_char,
        *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int>,
}

/* __init, __initdata, __exitdata, and __exit_call are section/usage
 * annotations in C and have no file-local executable equivalent in Rust. */

#[cfg(not(feature = "module"))]
extern "C" {
    pub static mut __uml_postsetup_start: InitcallT;
    pub static mut __uml_postsetup_end: InitcallT;
    pub static __uml_help_start: *const ::core::ffi::c_char;
    pub static __uml_help_end: *const ::core::ffi::c_char;
}

extern "C" {
    pub static mut __uml_setup_start: UmlParam;
    pub static mut __uml_setup_end: UmlParam;
}

/* C: static exitcall_t __uml_exitcall_##fn __uml_exit_call = fn */
#[macro_export]
macro_rules! __uml_exitcall {
    ($fn_name:path) => {
        /* Emits an exit-call registration for $fn_name in the target ABI. */
        const _: Option<$crate::ExitcallT> = Some($fn_name);
    };
}

/* C: static initcall_t __uml_postsetup_##fn __uml_postsetup_call = fn */
#[macro_export]
macro_rules! __uml_postsetup {
    ($fn_name:path) => {
        const _: Option<$crate::InitcallT> = Some($fn_name);
    };
}

/* A C empty-string check is represented by this zero-sized marker. */
#[macro_export]
macro_rules! __non_empty_string {
    ($dummyname:ident, $string:expr) => {
        const _: &str = $string;
    };
}

/* The variadic C registration macros retain their declaration-level intent.
 * The module branch intentionally expands to nothing, as in the source. */
#[cfg(not(feature = "module"))]
#[macro_export]
macro_rules! __uml_setup {
    ($str:expr, $fn_name:path $(, $help:expr)*) => {
        const _: &str = $str;
        const _: Option<$crate::InitcallT> = Some($fn_name);
        $(const _: &str = $help;)*
    };
}

#[cfg(feature = "module")]
#[macro_export]
macro_rules! __uml_setup {
    ($str:expr, $fn_name:path $(, $help:expr)*) => {};
}

#[macro_export]
macro_rules! __uml_help {
    ($fn_name:path $(, $help:expr)*) => {
        $(const _: &str = $help;)*
    };
}

/* __uml_init_setup, __uml_setup_help, __uml_postsetup_call, and
 * __uml_exit_call correspond to C __used/__section annotations. */

#[cfg(feature = "um_host")]
#[macro_export]
macro_rules! __define_initcall {
    ($level:expr, $fn_name:path) => {
        const _: Option<$crate::InitcallT> = Some($fn_name);
    };
}

#[cfg(feature = "um_host")]
#[macro_export]
macro_rules! __initcall {
    ($fn_name:path) => {
        $crate::__define_initcall!("1", $fn_name);
    };
}

#[cfg(feature = "um_host")]
#[macro_export]
macro_rules! __exitcall {
    ($fn_name:path) => {
        const _: Option<$crate::ExitcallT> = Some($fn_name);
    };
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
