/* SPDX-License-Identifier: GPL-2.0-only */

#[repr(C)]
pub struct trie_key {
    pub prefixlen: __u32,
    pub data: __u32,
}

/* Benchmark operations */
pub const LPM_OP_NOOP: u32 = 0;
pub const LPM_OP_BASELINE: u32 = 1;
pub const LPM_OP_LOOKUP: u32 = 2;
pub const LPM_OP_INSERT: u32 = 3;
pub const LPM_OP_UPDATE: u32 = 4;
pub const LPM_OP_DELETE: u32 = 5;
pub const LPM_OP_FREE: u32 = 6;

/*
 * Return values from run_bench.
 *
 * Negative values are also allowed and represent kernel error codes.
 */
pub const LPM_BENCH_SUCCESS: i32 = 0;
pub const LPM_BENCH_REINIT_MAP: i32 = 1; /* Reset trie to initial state for current op */
