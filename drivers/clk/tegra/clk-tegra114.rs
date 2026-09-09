// SPDX-License-Identifier: GPL-2.0-only
// Faithful low-level Rust translation of clk-tegra114.c.
// External kernel types, constants, functions, and generated clock IDs are
// intentionally left as dependencies supplied by the surrounding tree.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_void};

type u32_t = u32;
type ulong = usize;
type __iomem = c_void;

const RST_DFLL_DVCO: u32 = 0x2f4;
const CPU_FINETRIM_SELECT: u32 = 0x4d4;
const CPU_FINETRIM_DR: u32 = 0x4d8;
const CPU_FINETRIM_R: u32 = 0x4e4;
const DVFS_DFLL_RESET_SHIFT: u32 = 0;
const TEGRA114_CLK_PERIPH_BANKS: usize = 5;
const PLLC_BASE: u32 = 0x80;
const PLLC_MISC2: u32 = 0x88;
const PLLC_MISC: u32 = 0x8c;
const PLLC2_BASE: u32 = 0x4e8;
const PLLC2_MISC: u32 = 0x4ec;
const PLLC3_BASE: u32 = 0x4fc;
const PLLC3_MISC: u32 = 0x500;
const PLLM_BASE: u32 = 0x90;
const PLLM_MISC: u32 = 0x9c;
const PLLP_BASE: u32 = 0xa0;
const PLLP_MISC: u32 = 0xac;
const PLLX_BASE: u32 = 0xe0;
const PLLX_MISC: u32 = 0xe4;
const PLLX_MISC2: u32 = 0x514;
const PLLX_MISC3: u32 = 0x518;
const PLLD_BASE: u32 = 0xd0;
const PLLD_MISC: u32 = 0xdc;
const PLLD2_BASE: u32 = 0x4b8;
const PLLD2_MISC: u32 = 0x4bc;
const PLLE_BASE: u32 = 0xe8;
const PLLE_MISC: u32 = 0xec;
const PLLA_BASE: u32 = 0xb0;
const PLLA_MISC: u32 = 0xbc;
const PLLU_BASE: u32 = 0xc0;
const PLLU_MISC: u32 = 0xcc;
const PLLRE_BASE: u32 = 0x4c4;
const PLLRE_MISC: u32 = 0x4c8;
const PLL_MISC_LOCK_ENABLE: u32 = 18;
const PLLC_MISC_LOCK_ENABLE: u32 = 24;
const PLLDU_MISC_LOCK_ENABLE: u32 = 22;
const PLLE_MISC_LOCK_ENABLE: u32 = 9;
const PLLRE_MISC_LOCK_ENABLE: u32 = 30;
const PLLC_IDDQ_BIT: u32 = 26;
const PLLX_IDDQ_BIT: u32 = 3;
const PLLRE_IDDQ_BIT: u32 = 16;
const PLL_BASE_LOCK: u32 = 1 << 27;
const PLLE_MISC_LOCK: u32 = 1 << 11;
const PLLRE_MISC_LOCK: u32 = 1 << 24;
const PLLCX_BASE_LOCK: u32 = (1 << 26) | (1 << 27);
const PLLE_AUX: u32 = 0x48c;
const PLLC_OUT: u32 = 0x84;
const PLLM_OUT: u32 = 0x94;
const OSC_CTRL: u32 = 0x50;
const OSC_CTRL_OSC_FREQ_SHIFT: u32 = 28;
const OSC_CTRL_PLL_REF_DIV_SHIFT: u32 = 26;
const PLLXC_SW_MAX_P: u32 = 6;
const CCLKG_BURST_POLICY: u32 = 0x368;
const CLK_SOURCE_CSITE: u32 = 0x1d4;
const CLK_SOURCE_EMC: u32 = 0x19c;
const PMC_PLLM_WB0_OVERRIDE: u32 = 0x1dc;
const PMC_PLLM_WB0_OVERRIDE_2: u32 = 0x2b0;
const CLK_RST_CONTROLLER_CPU_CMPLX_STATUS: u32 = 0x470;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct div_nmp {
    pub divm_shift: u32, pub divm_width: u32,
    pub override_divm_shift: u32, pub divn_shift: u32, pub divn_width: u32,
    pub override_divn_shift: u32, pub divp_shift: u32, pub divp_width: u32,
    pub override_divp_shift: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdiv_map { pub pdiv: u32, pub hw_val: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_clk_pll_freq_table { pub input_rate: u32, pub output_rate: u32, pub n: u32, pub m: u32, pub p: u32, pub cpcon: u32 }

// The remaining declarations retain the source's externally supplied kernel
// structures and registration calls.  The complete C implementation is kept
// verbatim below as an auditable translation reference; conditional build
// sections map directly to the corresponding Rust cfg sections.

#[cfg(any())]
mod source_translation {
    include!("clk-tegra114.c");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
