/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies from the original header:
// <elf.h>, <linux/ctype.h>, <linux/types.h>

use core::ffi::{c_char, c_int, c_void};

pub type u8 = core::ffi::c_uchar;
pub type u64 = core::ffi::c_ulonglong;

pub const KSYM_NAME_LEN: usize = 512;

pub const STB_LOCAL: u8 = 0;
pub const STB_GLOBAL: u8 = 1;
pub const STB_WEAK: u8 = 2;

unsafe extern "C" {
    fn isupper(c: c_int) -> c_int;
}

pub unsafe extern "C" fn kallsyms2elf_binding(r#type: c_char) -> u8 {
    if r#type == b'W' as c_char {
        return STB_WEAK;
    }

    if unsafe { isupper(r#type as c_int) } != 0 {
        STB_GLOBAL
    } else {
        STB_LOCAL
    }
}

unsafe extern "C" {
    pub fn kallsyms2elf_type(r#type: c_char) -> u8;

    pub fn kallsyms__is_function(symbol_type: c_char) -> bool;

    pub fn kallsyms__parse(
        filename: *const c_char,
        arg: *mut c_void,
        process_symbol: Option<
            unsafe extern "C" fn(
                arg: *mut c_void,
                name: *const c_char,
                r#type: c_char,
                start: u64,
            ) -> c_int,
        >,
    ) -> c_int;
}
