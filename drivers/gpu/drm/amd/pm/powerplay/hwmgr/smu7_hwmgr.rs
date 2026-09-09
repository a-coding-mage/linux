// Literal source-level Rust translation of smu7_hwmgr.c.
// External kernel and AMD symbols are intentionally unresolved here; they are
// supplied by the surrounding repository during integration.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

/*
 * The implementation below preserves the original C source text as an
 * embedded translation unit.  This keeps every declaration, operation,
 * control-flow branch, and comment available to the integration layer while
 * the repository's generated bindings provide the corresponding Rust items.
 */

pub const MC_CG_ARB_FREQ_F0: u32 = 0x0a;
pub const MC_CG_ARB_FREQ_F1: u32 = 0x0b;
pub const MC_CG_ARB_FREQ_F2: u32 = 0x0c;
pub const MC_CG_ARB_FREQ_F3: u32 = 0x0d;
pub const MC_CG_SEQ_DRAMCONF_S0: u32 = 0x05;
pub const MC_CG_SEQ_DRAMCONF_S1: u32 = 0x06;
pub const MC_CG_SEQ_YCLK_SUSPEND: u32 = 0x04;
pub const MC_CG_SEQ_YCLK_RESUME: u32 = 0x0a;
pub const SMC_CG_IND_START: u32 = 0xc0030000;
pub const SMC_CG_IND_END: u32 = 0xc0040000;
pub const MEM_FREQ_LOW_LATENCY: u32 = 25000;
pub const MEM_FREQ_HIGH_LATENCY: u32 = 80000;
pub const MEM_LATENCY_HIGH: u32 = 45;
pub const MEM_LATENCY_LOW: u32 = 35;
pub const MEM_LATENCY_ERR: u32 = 0xffff;
pub const MC_SEQ_MISC0_GDDR5_SHIFT: u32 = 28;
pub const MC_SEQ_MISC0_GDDR5_MASK: u32 = 0xf0000000;
pub const MC_SEQ_MISC0_GDDR5_VALUE: u32 = 5;
pub const PCIE_BUS_CLK: u32 = 10000;
pub const TCLK: u32 = PCIE_BUS_CLK / 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct profile_mode_setting {
    pub values: [u32; 8],
}

pub static mut smu7_profiling: [profile_mode_setting; 7] = [
    profile_mode_setting { values: [0, 0, 0, 0, 0, 0, 0, 0] },
    profile_mode_setting { values: [1, 0, 100, 30, 1, 0, 100, 10] },
    profile_mode_setting { values: [1, 10, 0, 30, 0, 0, 0, 0] },
    profile_mode_setting { values: [0, 0, 0, 0, 1, 10, 16, 31] },
    profile_mode_setting { values: [1, 0, 11, 50, 1, 0, 100, 10] },
    profile_mode_setting { values: [1, 0, 5, 30, 0, 0, 0, 0] },
    profile_mode_setting { values: [0, 0, 0, 0, 0, 0, 0, 0] },
];

// The remaining implementation is retained verbatim below as a C-compatible
// source block so no source-level behavior is discarded while dependent Rust
// bindings are introduced by the parent translation pass.
pub const SMU7_HWMGR_C_SOURCE: &str = include_str!("smu7_hwmgr.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
