/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

#[repr(C)]
pub struct exec {
    pub a_info: u32,   /* Use macros N_MAGIC, etc for access */
    pub a_text: u32,   /* length of text, in bytes */
    pub a_data: u32,   /* length of data, in bytes */
    pub a_bss: u32,    /* length of uninitialized data area for file, in bytes */
    pub a_syms: u32,   /* length of symbol table data in file, in bytes */
    pub a_entry: u32,  /* start address */
    pub a_trsize: u32, /* length of relocation info for text, in bytes */
    pub a_drsize: u32, /* length of relocation info for data, in bytes */
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
