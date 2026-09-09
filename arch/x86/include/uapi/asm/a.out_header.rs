/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

#[repr(C)]
pub struct exec {
    pub a_info: ::core::ffi::c_uint,   /* Use macros N_MAGIC, etc for access */
    pub a_text: ::core::ffi::c_uint,   /* length of text, in bytes */
    pub a_data: ::core::ffi::c_uint,   /* length of data, in bytes */
    pub a_bss: ::core::ffi::c_uint,    /* length of uninitialized data area for file, in bytes */
    pub a_syms: ::core::ffi::c_uint,   /* length of symbol table data in file, in bytes */
    pub a_entry: ::core::ffi::c_uint,  /* start address */
    pub a_trsize: ::core::ffi::c_uint, /* length of relocation info for text, in bytes */
    pub a_drsize: ::core::ffi::c_uint, /* length of relocation info for data, in bytes */
}

#[macro_export]
macro_rules! N_TRSIZE {
    ($a:expr) => {
        ($a).a_trsize
    };
}

#[macro_export]
macro_rules! N_DRSIZE {
    ($a:expr) => {
        ($a).a_drsize
    };
}

#[macro_export]
macro_rules! N_SYMSIZE {
    ($a:expr) => {
        ($a).a_syms
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
