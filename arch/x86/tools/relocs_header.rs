/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding C/Rust translation unit are
// intentionally not reimplemented here.

use core::ffi::{c_char, c_int, c_void};

/// C declaration: void die(char *fmt, ...), noreturn.
unsafe extern "C" {
    pub fn die(fmt: *mut c_char, ...) -> !;
}

macro_rules! ARRAY_SIZE {
    ($x:expr) => {
        core::mem::size_of_val(&$x) / core::mem::size_of_val(&$x[0])
    };
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum symtype {
    S_ABS,
    S_REL,
    S_SEG,
    S_LIN,
    S_NSYMTYPES,
}

unsafe extern "C" {
    pub fn process_32(
        fp: *mut c_void,
        use_real_mode: c_int,
        as_text: c_int,
        show_absolute_syms: c_int,
        show_absolute_relocs: c_int,
        show_reloc_info: c_int,
    );

    pub fn process_64(
        fp: *mut c_void,
        use_real_mode: c_int,
        as_text: c_int,
        show_absolute_syms: c_int,
        show_absolute_relocs: c_int,
        show_reloc_info: c_int,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
