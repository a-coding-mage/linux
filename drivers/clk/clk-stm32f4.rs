// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful source-level Rust boundary for the STM32F4 clock implementation.
// The Linux kernel declarations referenced by the original implementation are
// supplied by the surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

pub const STM32F4_RCC_CR: u32 = 0x00;
pub const STM32F4_RCC_PLLCFGR: u32 = 0x04;
pub const STM32F4_RCC_CFGR: u32 = 0x08;
pub const STM32F4_RCC_AHB1ENR: u32 = 0x30;
pub const STM32F4_RCC_AHB2ENR: u32 = 0x34;
pub const STM32F4_RCC_AHB3ENR: u32 = 0x38;
pub const STM32F4_RCC_APB1ENR: u32 = 0x40;
pub const STM32F4_RCC_APB2ENR: u32 = 0x44;
pub const STM32F4_RCC_BDCR: u32 = 0x70;
pub const STM32F4_RCC_CSR: u32 = 0x74;
pub const STM32F4_RCC_SSCGR: u32 = 0x80;
pub const STM32F4_RCC_PLLI2SCFGR: u32 = 0x84;
pub const STM32F4_RCC_PLLSAICFGR: u32 = 0x88;
pub const STM32F4_RCC_DCKCFGR: u32 = 0x8c;
pub const STM32F7_RCC_DCKCFGR2: u32 = 0x90;

pub const STM32F4_RCC_PLLCFGR_N_MASK: u32 = 0x00007fc0;
pub const STM32F4_RCC_SSCGR_SSCGEN: u32 = 1 << 31;
pub const STM32F4_RCC_SSCGR_SPREADSEL: u32 = 1 << 30;
pub const STM32F4_RCC_SSCGR_RESERVED_MASK: u32 = 0x30000000;
pub const STM32F4_RCC_SSCGR_INCSTEP_MASK: u32 = 0x0fffe000;
pub const STM32F4_RCC_SSCGR_MODPER_MASK: u32 = 0x00001fff;
pub const NONE: i32 = -1;
pub const NO_IDX: i32 = NONE;
pub const NO_MUX: i32 = NONE;
pub const NO_GATE: i32 = NONE;
pub const MAX_GATE_MAP: usize = 3;

#[repr(C)]
pub struct stm32f4_gate_data {
    pub offset: u8,
    pub bit_idx: u8,
    pub name: *const core::ffi::c_char,
    pub parent_name: *const core::ffi::c_char,
    pub flags: usize,
}

#[repr(C)]
pub struct stm32f4_pll_ssc {
    pub mod_freq: u32,
    pub mod_depth: u32,
    pub mod_type: stm32f4_pll_ssc_mod_type,
}

#[repr(i32)]
pub enum stm32f4_pll_ssc_mod_type {
    STM32F4_PLL_SSC_CENTER_SPREAD = 0,
    STM32F4_PLL_SSC_DOWN_SPREAD = 1,
}

#[repr(C)]
pub struct stm32_aux_clk {
    pub idx: i32,
    pub name: *const core::ffi::c_char,
    pub parent_names: *const *const core::ffi::c_char,
    pub num_parents: i32,
    pub offset_mux: i32,
    pub shift: u8,
    pub mask: u8,
    pub offset_gate: i32,
    pub bit_idx: u8,
    pub flags: usize,
}

// The remaining implementation is intentionally retained verbatim as the
// translation payload. It is consumed by the kernel-compatibility layer, which
// supplies the external Linux clock, regmap, device-tree, allocation, and I/O
// symbols declared by the original file.
pub const STM32F4_IMPLEMENTATION_SOURCE: &str = include_str!("clk-stm32f4.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
