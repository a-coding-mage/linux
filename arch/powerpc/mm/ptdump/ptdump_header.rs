/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the Linux kernel declarations in ptdump.h.
// Dependencies corresponding to <linux/types.h> and <linux/seq_file.h> are
// supplied by the surrounding translation unit.

use core::ffi::{c_char, c_ulong};

#[repr(C)]
pub struct flag_info {
    pub mask: u64,
    pub val: u64,
    pub set: *const c_char,
    pub clear: *const c_char,
    pub is_val: bool,
    pub shift: i32,
}

#[repr(C)]
pub struct ptdump_pg_level {
    pub flag: *const flag_info,
    pub name: [c_char; 4],
    pub num: usize,
    pub mask: u64,
}

// Opaque type corresponding to struct seq_file from <linux/seq_file.h>.
pub enum seq_file {}

extern "C" {
    pub static mut pg_level: [ptdump_pg_level; 5];

    pub fn pt_dump_size(m: *mut seq_file, delta: c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
