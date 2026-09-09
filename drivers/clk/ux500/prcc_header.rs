/* SPDX-License-Identifier: GPL-2.0-only */

pub const PRCC_NUM_PERIPH_CLUSTERS: i32 = 6;
pub const PRCC_PERIPHS_PER_CLUSTER: i32 = 32;

/* CLKRST4 is missing making it hard to index things */
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum clkrst_index {
    CLKRST1_INDEX = 0,
    CLKRST2_INDEX,
    CLKRST3_INDEX,
    CLKRST5_INDEX,
    CLKRST6_INDEX,
    CLKRST_MAX,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
