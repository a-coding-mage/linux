/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm/include/asm/hardware/cache-l2x0.h
 *
 * Copyright (C) 2007 ARM Limited
 */

/* C header dependencies: linux/errno.h, linux/init.h, linux/types.h. */

pub const L2X0_CACHE_ID: u32 = 0x000;
pub const L2X0_CACHE_TYPE: u32 = 0x004;
pub const L2X0_CTRL: u32 = 0x100;
pub const L2X0_AUX_CTRL: u32 = 0x104;
pub const L310_TAG_LATENCY_CTRL: u32 = 0x108;
pub const L310_DATA_LATENCY_CTRL: u32 = 0x10C;
pub const L2X0_EVENT_CNT_CTRL: u32 = 0x200;
pub const L2X0_EVENT_CNT1_CFG: u32 = 0x204;
pub const L2X0_EVENT_CNT0_CFG: u32 = 0x208;
pub const L2X0_EVENT_CNT1_VAL: u32 = 0x20C;
pub const L2X0_EVENT_CNT0_VAL: u32 = 0x210;
pub const L2X0_INTR_MASK: u32 = 0x214;
pub const L2X0_MASKED_INTR_STAT: u32 = 0x218;
pub const L2X0_RAW_INTR_STAT: u32 = 0x21C;
pub const L2X0_INTR_CLEAR: u32 = 0x220;
pub const L2X0_CACHE_SYNC: u32 = 0x730;
pub const L2X0_DUMMY_REG: u32 = 0x740;
pub const L2X0_INV_LINE_PA: u32 = 0x770;
pub const L2X0_INV_WAY: u32 = 0x77C;
pub const L2X0_CLEAN_LINE_PA: u32 = 0x7B0;
pub const L2X0_CLEAN_LINE_IDX: u32 = 0x7B8;
pub const L2X0_CLEAN_WAY: u32 = 0x7BC;
pub const L2X0_CLEAN_INV_LINE_PA: u32 = 0x7F0;
pub const L2X0_CLEAN_INV_LINE_IDX: u32 = 0x7F8;
pub const L2X0_CLEAN_INV_WAY: u32 = 0x7FC;
pub const L2X0_LOCKDOWN_WAY_D_BASE: u32 = 0x900;
pub const L2X0_LOCKDOWN_WAY_I_BASE: u32 = 0x904;
pub const L2X0_LOCKDOWN_STRIDE: u32 = 0x08;
pub const L310_ADDR_FILTER_START: u32 = 0xC00;
pub const L310_ADDR_FILTER_END: u32 = 0xC04;
pub const L2X0_TEST_OPERATION: u32 = 0xF00;
pub const L2X0_LINE_DATA: u32 = 0xF10;
pub const L2X0_LINE_TAG: u32 = 0xF30;
pub const L2X0_DEBUG_CTRL: u32 = 0xF40;
pub const L310_PREFETCH_CTRL: u32 = 0xF60;
pub const L310_POWER_CTRL: u32 = 0xF80;
pub const L310_DYNAMIC_CLK_GATING_EN: u32 = 1 << 1;
pub const L310_STNDBY_MODE_EN: u32 = 1 << 0;

pub const L2X0_CACHE_ID_PART_MASK: u32 = 0xf << 6;
pub const L2X0_CACHE_ID_PART_L210: u32 = 1 << 6;
pub const L2X0_CACHE_ID_PART_L220: u32 = 2 << 6;
pub const L2X0_CACHE_ID_PART_L310: u32 = 3 << 6;
pub const L2X0_CACHE_ID_RTL_MASK: u32 = 0x3f;
pub const L210_CACHE_ID_RTL_R0P2_02: u32 = 0x00;
pub const L210_CACHE_ID_RTL_R0P1: u32 = 0x01;
pub const L210_CACHE_ID_RTL_R0P2_01: u32 = 0x02;
pub const L210_CACHE_ID_RTL_R0P3: u32 = 0x03;
pub const L210_CACHE_ID_RTL_R0P4: u32 = 0x0b;
pub const L210_CACHE_ID_RTL_R0P5: u32 = 0x0f;
pub const L220_CACHE_ID_RTL_R1P7_01REL0: u32 = 0x06;
pub const L310_CACHE_ID_RTL_R0P0: u32 = 0x00;
pub const L310_CACHE_ID_RTL_R1P0: u32 = 0x02;
pub const L310_CACHE_ID_RTL_R2P0: u32 = 0x04;
pub const L310_CACHE_ID_RTL_R3P0: u32 = 0x05;
pub const L310_CACHE_ID_RTL_R3P1: u32 = 0x06;
pub const L310_CACHE_ID_RTL_R3P1_50REL0: u32 = 0x07;
pub const L310_CACHE_ID_RTL_R3P2: u32 = 0x08;
pub const L310_CACHE_ID_RTL_R3P3: u32 = 0x09;

pub const L2X0_EVENT_CNT_CTRL_ENABLE: u32 = 1 << 0;
pub const L2X0_EVENT_CNT_CFG_SRC_SHIFT: u32 = 2;
pub const L2X0_EVENT_CNT_CFG_SRC_MASK: u32 = 0xf;
pub const L2X0_EVENT_CNT_CFG_SRC_DISABLED: u32 = 0;
pub const L2X0_EVENT_CNT_CFG_INT_DISABLED: u32 = 0;
pub const L2X0_EVENT_CNT_CFG_INT_INCR: u32 = 1;
pub const L2X0_EVENT_CNT_CFG_INT_OVERFLOW: u32 = 2;

pub const L2C_AUX_CTRL_WAY_SIZE_SHIFT: u32 = 17;
pub const L2C_AUX_CTRL_WAY_SIZE_MASK: u32 = 7 << 17;
#[inline] pub const fn L2C_AUX_CTRL_WAY_SIZE(n: u32) -> u32 { n << 17 }
pub const L2C_AUX_CTRL_EVTMON_ENABLE: u32 = 1 << 20;
pub const L2C_AUX_CTRL_PARITY_ENABLE: u32 = 1 << 21;
pub const L2C_AUX_CTRL_SHARED_OVERRIDE: u32 = 1 << 22;
pub const L2X0_AUX_CTRL_DATA_RD_LATENCY_SHIFT: u32 = 0;
pub const L2X0_AUX_CTRL_DATA_RD_LATENCY_MASK: u32 = 7 << 0;
pub const L2X0_AUX_CTRL_DATA_WR_LATENCY_SHIFT: u32 = 3;
pub const L2X0_AUX_CTRL_DATA_WR_LATENCY_MASK: u32 = 7 << 3;
pub const L2X0_AUX_CTRL_TAG_LATENCY_SHIFT: u32 = 6;
pub const L2X0_AUX_CTRL_TAG_LATENCY_MASK: u32 = 7 << 6;
pub const L2X0_AUX_CTRL_DIRTY_LATENCY_SHIFT: u32 = 9;
pub const L2X0_AUX_CTRL_DIRTY_LATENCY_MASK: u32 = 7 << 9;
pub const L2X0_AUX_CTRL_ASSOC_SHIFT: u32 = 13;
pub const L2X0_AUX_CTRL_ASSOC_MASK: u32 = 15 << 13;
pub const L210_AUX_CTRL_WRAP_DISABLE: u32 = 1 << 12;
pub const L210_AUX_CTRL_WA_OVERRIDE: u32 = 1 << 23;
pub const L210_AUX_CTRL_EXCLUSIVE_ABORT: u32 = 1 << 24;
pub const L220_AUX_CTRL_EXCLUSIVE_CACHE: u32 = 1 << 12;
pub const L220_AUX_CTRL_FWA_SHIFT: u32 = 23;
pub const L220_AUX_CTRL_FWA_MASK: u32 = 3 << 23;
pub const L220_AUX_CTRL_NS_LOCKDOWN: u32 = 1 << 26;
pub const L220_AUX_CTRL_NS_INT_CTRL: u32 = 1 << 27;
pub const L310_AUX_CTRL_FULL_LINE_ZERO: u32 = 1 << 0;
pub const L310_AUX_CTRL_HIGHPRIO_SO_DEV: u32 = 1 << 10;
pub const L310_AUX_CTRL_STORE_LIMITATION: u32 = 1 << 11;
pub const L310_AUX_CTRL_EXCLUSIVE_CACHE: u32 = 1 << 12;
pub const L310_AUX_CTRL_ASSOCIATIVITY_16: u32 = 1 << 16;
pub const L310_AUX_CTRL_FWA_SHIFT: u32 = 23;
pub const L310_AUX_CTRL_FWA_MASK: u32 = 3 << 23;
pub const L310_AUX_CTRL_CACHE_REPLACE_RR: u32 = 1 << 25;
pub const L310_AUX_CTRL_NS_LOCKDOWN: u32 = 1 << 26;
pub const L310_AUX_CTRL_NS_INT_CTRL: u32 = 1 << 27;
pub const L310_AUX_CTRL_DATA_PREFETCH: u32 = 1 << 28;
pub const L310_AUX_CTRL_INSTR_PREFETCH: u32 = 1 << 29;
pub const L310_AUX_CTRL_EARLY_BRESP: u32 = 1 << 30;

#[inline] pub const fn L310_LATENCY_CTRL_SETUP(n: u32) -> u32 { n << 0 }
#[inline] pub const fn L310_LATENCY_CTRL_RD(n: u32) -> u32 { n << 4 }
#[inline] pub const fn L310_LATENCY_CTRL_WR(n: u32) -> u32 { n << 8 }
pub const L310_ADDR_FILTER_EN: u32 = 1;
pub const L310_PREFETCH_CTRL_OFFSET_MASK: u32 = 0x1f;
pub const L310_PREFETCH_CTRL_DBL_LINEFILL_INCR: u32 = 1 << 23;
pub const L310_PREFETCH_CTRL_PREFETCH_DROP: u32 = 1 << 24;
pub const L310_PREFETCH_CTRL_DBL_LINEFILL_WRAP: u32 = 1 << 27;
pub const L310_PREFETCH_CTRL_DATA_PREFETCH: u32 = 1 << 28;
pub const L310_PREFETCH_CTRL_INSTR_PREFETCH: u32 = 1 << 29;
pub const L310_PREFETCH_CTRL_DBL_LINEFILL: u32 = 1 << 30;
pub const L2X0_CTRL_EN: u32 = 1;
pub const L2X0_WAY_SIZE_SHIFT: u32 = 3;

extern "C" {
    pub fn l2x0_init(base: *mut core::ffi::c_void, aux_val: u32, aux_mask: u32);
    pub fn l2x0_of_init(aux_val: u32, aux_mask: u32) -> i32;
    pub fn l2x0_pmu_register(base: *mut core::ffi::c_void, part: u32);
    pub fn l2x0_pmu_suspend();
    pub fn l2x0_pmu_resume();
}

#[repr(C)]
pub struct l2x0_regs {
    pub phy_base: usize,
    pub aux_ctrl: usize,
    /* Whether the following registers need to be saved/restored depends on platform. */
    pub tag_latency: usize,
    pub data_latency: usize,
    pub filter_start: usize,
    pub filter_end: usize,
    pub prefetch_ctrl: usize,
    pub pwr_ctrl: usize,
    pub ctrl: usize,
    pub aux2_ctrl: usize,
}

extern "C" { pub static mut l2x0_saved_regs: l2x0_regs; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
