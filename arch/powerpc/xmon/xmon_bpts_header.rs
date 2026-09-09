/* SPDX-License-Identifier: GPL-2.0 */

pub const NBPTS: usize = 256;

/* Supplied by the architecture instruction definitions. */
pub const BPT_SIZE: usize = core::mem::size_of::<ppc_inst_t>() * 2;
pub const BPT_WORDS: usize = BPT_SIZE / core::mem::size_of::<ppc_inst_t>();

unsafe extern "C" {
    pub static mut bpt_table: [u32; NBPTS * BPT_WORDS];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
