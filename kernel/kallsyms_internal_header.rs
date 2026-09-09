/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency: <linux/types.h>

unsafe extern "C" {
    pub static kallsyms_offsets: [i32; 0];
    pub static kallsyms_names: [u8; 0];

    pub static kallsyms_num_syms: u32;

    pub static kallsyms_token_table: [core::ffi::c_char; 0];
    pub static kallsyms_token_index: [u16; 0];

    pub static kallsyms_markers: [u32; 0];
    pub static kallsyms_seqs_of_names: [u8; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
