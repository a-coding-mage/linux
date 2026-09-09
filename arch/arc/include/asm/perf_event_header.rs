/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Linux performance counter support for ARC
 *
 * Copyright (C) 2014-2015 Synopsys, Inc. (www.synopsys.com)
 * Copyright (C) 2011-2013 Synopsys, Inc. (www.synopsys.com)
 */

/* Max number of counters that PCT block may ever have */
pub const ARC_PERF_MAX_COUNTERS: u32 = 32;

pub const ARC_REG_CC_BUILD: u32 = 0xF6;
pub const ARC_REG_CC_INDEX: u32 = 0x240;
pub const ARC_REG_CC_NAME0: u32 = 0x241;
pub const ARC_REG_CC_NAME1: u32 = 0x242;

pub const ARC_REG_PCT_BUILD: u32 = 0xF5;
pub const ARC_REG_PCT_COUNTL: u32 = 0x250;
pub const ARC_REG_PCT_COUNTH: u32 = 0x251;
pub const ARC_REG_PCT_SNAPL: u32 = 0x252;
pub const ARC_REG_PCT_SNAPH: u32 = 0x253;
pub const ARC_REG_PCT_CONFIG: u32 = 0x254;
pub const ARC_REG_PCT_CONTROL: u32 = 0x255;
pub const ARC_REG_PCT_INDEX: u32 = 0x256;
pub const ARC_REG_PCT_INT_CNTL: u32 = 0x25C;
pub const ARC_REG_PCT_INT_CNTH: u32 = 0x25D;
pub const ARC_REG_PCT_INT_CTRL: u32 = 0x25E;
pub const ARC_REG_PCT_INT_ACT: u32 = 0x25F;

pub const ARC_REG_PCT_CONFIG_USER: u32 = 1 << 18; /* count in user mode */
pub const ARC_REG_PCT_CONFIG_KERN: u32 = 1 << 19; /* count in kernel mode */

pub const ARC_REG_PCT_CONTROL_CC: u32 = 1 << 16; /* clear counts */
pub const ARC_REG_PCT_CONTROL_SN: u32 = 1 << 17; /* snapshot */

#[repr(C)]
pub struct arc_reg_pct_build {
    /* CONFIG_CPU_BIG_ENDIAN: m:8, c:8, r:5, i:1, s:2, v:8. */
    /* Otherwise: v:8, s:2, i:1, r:5, c:8, m:8. */
    pub bits: u32,
}

#[repr(C)]
pub struct arc_reg_cc_build {
    /* CONFIG_CPU_BIG_ENDIAN: c:16, r:8, v:8. */
    /* Otherwise: v:8, r:8, c:16. */
    pub bits: u32,
}

pub const PERF_COUNT_ARC_DCLM: u32 = PERF_COUNT_HW_MAX + 0;
pub const PERF_COUNT_ARC_DCSM: u32 = PERF_COUNT_HW_MAX + 1;
pub const PERF_COUNT_ARC_ICM: u32 = PERF_COUNT_HW_MAX + 2;
pub const PERF_COUNT_ARC_BPOK: u32 = PERF_COUNT_HW_MAX + 3;
pub const PERF_COUNT_ARC_EDTLB: u32 = PERF_COUNT_HW_MAX + 4;
pub const PERF_COUNT_ARC_EITLB: u32 = PERF_COUNT_HW_MAX + 5;
pub const PERF_COUNT_ARC_LDC: u32 = PERF_COUNT_HW_MAX + 6;
pub const PERF_COUNT_ARC_STC: u32 = PERF_COUNT_HW_MAX + 7;

pub const PERF_COUNT_ARC_HW_MAX: u32 = PERF_COUNT_HW_MAX + 8;

/* CONFIG_PERF_EVENTS: perf_arch_bpf_user_pt_regs(regs) casts regs to
 * (struct user_regs_struct *). */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
