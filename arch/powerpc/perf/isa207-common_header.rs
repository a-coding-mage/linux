/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2009 Paul Mackerras, IBM Corporation.
 * Copyright 2013 Michael Ellerman, IBM Corporation.
 * Copyright 2016 Madhavan Srinivasan, IBM Corporation.
 */

// C dependencies: linux/kernel.h, linux/perf_event.h, asm/firmware.h,
// asm/cputable.h, and internal.h.

pub const EVENT_EBB_MASK: u64 = 1;
pub const EVENT_EBB_SHIFT: u32 = PERF_EVENT_CONFIG_EBB_SHIFT;
pub const EVENT_BHRB_MASK: u64 = 1;
pub const EVENT_BHRB_SHIFT: u32 = 62;
pub const EVENT_WANTS_BHRB: u64 = EVENT_BHRB_MASK << EVENT_BHRB_SHIFT;
pub const EVENT_IFM_MASK: u64 = 3;
pub const EVENT_IFM_SHIFT: u32 = 60;
pub const EVENT_THR_CMP_SHIFT: u32 = 40;
pub const EVENT_THR_CMP_MASK: u64 = 0x3ff;
pub const EVENT_THR_CTL_SHIFT: u32 = 32;
pub const EVENT_THR_CTL_MASK: u64 = 0xff;
pub const EVENT_THR_SEL_SHIFT: u32 = 29;
pub const EVENT_THR_SEL_MASK: u64 = 0x7;
pub const EVENT_THRESH_SHIFT: u32 = 29;
pub const EVENT_THRESH_MASK: u64 = 0x1fffff;
pub const EVENT_SAMPLE_SHIFT: u32 = 24;
pub const EVENT_SAMPLE_MASK: u64 = 0x1f;
pub const EVENT_CACHE_SEL_SHIFT: u32 = 20;
pub const EVENT_CACHE_SEL_MASK: u64 = 0xf;
pub const EVENT_IS_L1: u64 = 4 << EVENT_CACHE_SEL_SHIFT;
pub const EVENT_PMC_SHIFT: u32 = 16;
pub const EVENT_PMC_MASK: u64 = 0xf;
pub const EVENT_UNIT_SHIFT: u32 = 12;
pub const EVENT_UNIT_MASK: u64 = 0xf;
pub const EVENT_COMBINE_SHIFT: u32 = 11;
pub const EVENT_COMBINE_MASK: u64 = 1;
#[inline] pub const fn EVENT_COMBINE(v: u64) -> u64 { (v >> EVENT_COMBINE_SHIFT) & EVENT_COMBINE_MASK }
pub const EVENT_MARKED_SHIFT: u32 = 8;
pub const EVENT_MARKED_MASK: u64 = 1;
pub const EVENT_IS_MARKED: u64 = EVENT_MARKED_MASK << EVENT_MARKED_SHIFT;
pub const EVENT_PSEL_MASK: u64 = 0xff;
pub const EVENT_LINUX_MASK: u64 = (EVENT_EBB_MASK << EVENT_EBB_SHIFT) | (EVENT_BHRB_MASK << EVENT_BHRB_SHIFT) | (EVENT_IFM_MASK << EVENT_IFM_SHIFT);
pub const EVENT_VALID_MASK: u64 = (EVENT_THRESH_MASK << EVENT_THRESH_SHIFT) | (EVENT_SAMPLE_MASK << EVENT_SAMPLE_SHIFT) | (EVENT_CACHE_SEL_MASK << EVENT_CACHE_SEL_SHIFT) | (EVENT_PMC_MASK << EVENT_PMC_SHIFT) | (EVENT_UNIT_MASK << EVENT_UNIT_SHIFT) | (EVENT_COMBINE_MASK << EVENT_COMBINE_SHIFT) | (EVENT_MARKED_MASK << EVENT_MARKED_SHIFT) | EVENT_LINUX_MASK | EVENT_PSEL_MASK;
pub const ONLY_PLM: u64 = PERF_SAMPLE_BRANCH_USER | PERF_SAMPLE_BRANCH_KERNEL | PERF_SAMPLE_BRANCH_HV;

pub const P9_EVENT_COMBINE_SHIFT: u32 = 10;
pub const P9_EVENT_COMBINE_MASK: u64 = 3;
#[inline] pub const fn p9_EVENT_COMBINE(v: u64) -> u64 { (v >> P9_EVENT_COMBINE_SHIFT) & P9_EVENT_COMBINE_MASK }
pub const P9_SDAR_MODE_SHIFT: u32 = 50;
pub const P9_SDAR_MODE_MASK: u64 = 3;
#[inline] pub const fn p9_SDAR_MODE(v: u64) -> u64 { (v >> P9_SDAR_MODE_SHIFT) & P9_SDAR_MODE_MASK }
pub const P9_EVENT_VALID_MASK: u64 = (P9_SDAR_MODE_MASK << P9_SDAR_MODE_SHIFT) | (EVENT_THRESH_MASK << EVENT_THRESH_SHIFT) | (EVENT_SAMPLE_MASK << EVENT_SAMPLE_SHIFT) | (EVENT_CACHE_SEL_MASK << EVENT_CACHE_SEL_SHIFT) | (EVENT_PMC_MASK << EVENT_PMC_SHIFT) | (EVENT_UNIT_MASK << EVENT_UNIT_SHIFT) | (P9_EVENT_COMBINE_MASK << P9_EVENT_COMBINE_SHIFT) | (EVENT_MARKED_MASK << EVENT_MARKED_SHIFT) | EVENT_LINUX_MASK | EVENT_PSEL_MASK;

pub const P10_SDAR_MODE_SHIFT: u32 = 22;
pub const P10_SDAR_MODE_MASK: u64 = 3;
#[inline] pub const fn p10_SDAR_MODE(v: u64) -> u64 { (v >> P10_SDAR_MODE_SHIFT) & P10_SDAR_MODE_MASK }
pub const P10_EVENT_L2L3_SEL_MASK: u64 = 0x1f;
pub const P10_L2L3_SEL_SHIFT: u32 = 3;
pub const P10_L2L3_EVENT_SHIFT: u32 = 40;
pub const P10_EVENT_THRESH_MASK: u64 = 0xffff;
pub const P10_EVENT_CACHE_SEL_MASK: u64 = 3;
pub const P10_EVENT_MMCR3_MASK: u64 = 0x7fff;
pub const P10_EVENT_MMCR3_SHIFT: u32 = 45;
pub const P10_EVENT_RADIX_SCOPE_QUAL_SHIFT: u32 = 9;
pub const P10_EVENT_RADIX_SCOPE_QUAL_MASK: u64 = 1;
pub const P10_MMCR1_RADIX_SCOPE_QUAL_SHIFT: u32 = 45;
pub const P10_EVENT_THR_CMP_SHIFT: u32 = 0;
pub const P10_EVENT_THR_CMP_MASK: u64 = 0x3ffff;
pub const P10_EVENT_VALID_MASK: u64 = (P10_SDAR_MODE_MASK << P10_SDAR_MODE_SHIFT) | (P10_EVENT_THRESH_MASK << EVENT_THRESH_SHIFT) | (EVENT_SAMPLE_MASK << EVENT_SAMPLE_SHIFT) | (P10_EVENT_CACHE_SEL_MASK << EVENT_CACHE_SEL_SHIFT) | (EVENT_PMC_MASK << EVENT_PMC_SHIFT) | (EVENT_UNIT_MASK << EVENT_UNIT_SHIFT) | (P9_EVENT_COMBINE_MASK << P9_EVENT_COMBINE_SHIFT) | (P10_EVENT_MMCR3_MASK << P10_EVENT_MMCR3_SHIFT) | (EVENT_MARKED_MASK << EVENT_MARKED_SHIFT) | (P10_EVENT_RADIX_SCOPE_QUAL_MASK << P10_EVENT_RADIX_SCOPE_QUAL_SHIFT) | EVENT_LINUX_MASK | EVENT_PSEL_MASK;

pub const fn CNST_FAB_MATCH_VAL(v: u64) -> u64 { (v & EVENT_THR_CTL_MASK) << 56 }
pub const CNST_FAB_MATCH_MASK: u64 = CNST_FAB_MATCH_VAL(EVENT_THR_CTL_MASK);
pub const fn CNST_THRESH_VAL(v: u64) -> u64 { (v & EVENT_THRESH_MASK) << 32 }
pub const CNST_THRESH_MASK: u64 = CNST_THRESH_VAL(EVENT_THRESH_MASK);
pub const fn CNST_THRESH_CTL_SEL_VAL(v: u64) -> u64 { (v & 0x7ff) << 32 }
pub const CNST_THRESH_CTL_SEL_MASK: u64 = CNST_THRESH_CTL_SEL_VAL(0x7ff);
pub const fn p10_CNST_THRESH_CMP_VAL(v: u64) -> u64 { (v & 0x7ff) << 43 }
pub const p10_CNST_THRESH_CMP_MASK: u64 = p10_CNST_THRESH_CMP_VAL(0x7ff);
pub const fn CNST_EBB_VAL(v: u64) -> u64 { (v & EVENT_EBB_MASK) << 24 }
pub const CNST_EBB_MASK: u64 = CNST_EBB_VAL(EVENT_EBB_MASK);
pub const fn CNST_IFM_VAL(v: u64) -> u64 { (v & EVENT_IFM_MASK) << 25 }
pub const CNST_IFM_MASK: u64 = CNST_IFM_VAL(EVENT_IFM_MASK);
pub const fn CNST_L1_QUAL_VAL(v: u64) -> u64 { (v & 3) << 22 }
pub const CNST_L1_QUAL_MASK: u64 = CNST_L1_QUAL_VAL(3);
pub const fn CNST_SAMPLE_VAL(v: u64) -> u64 { (v & EVENT_SAMPLE_MASK) << 16 }
pub const CNST_SAMPLE_MASK: u64 = CNST_SAMPLE_VAL(EVENT_SAMPLE_MASK);
pub const fn CNST_CACHE_GROUP_VAL(v: u64) -> u64 { (v & 0xff) << 55 }
pub const CNST_CACHE_GROUP_MASK: u64 = CNST_CACHE_GROUP_VAL(0xff);
pub const CNST_CACHE_PMC4_VAL: u64 = 1 << 54;
pub const CNST_CACHE_PMC4_MASK: u64 = CNST_CACHE_PMC4_VAL;
pub const fn CNST_L2L3_GROUP_VAL(v: u64) -> u64 { (v & 0x1f) << 55 }
pub const CNST_L2L3_GROUP_MASK: u64 = CNST_L2L3_GROUP_VAL(0x1f);
pub const fn CNST_RADIX_SCOPE_GROUP_VAL(v: u64) -> u64 { (v & 1) << 21 }
pub const CNST_RADIX_SCOPE_GROUP_MASK: u64 = CNST_RADIX_SCOPE_GROUP_VAL(1);
pub const CNST_NC_SHIFT: u32 = 12;
pub const CNST_NC_VAL: u64 = 1 << CNST_NC_SHIFT;
pub const CNST_NC_MASK: u64 = 8 << CNST_NC_SHIFT;
pub const ISA207_TEST_ADDER: u64 = 3 << CNST_NC_SHIFT;
pub const fn CNST_PMC_SHIFT(pmc: u64) -> u32 { ((pmc - 1) * 2) as u32 }
pub const fn CNST_PMC_VAL(pmc: u64) -> u64 { 1 << CNST_PMC_SHIFT(pmc) }
pub const fn CNST_PMC_MASK(pmc: u64) -> u64 { 2 << CNST_PMC_SHIFT(pmc) }
pub const ISA207_ADD_FIELDS: u64 = CNST_PMC_VAL(1) | CNST_PMC_VAL(2) | CNST_PMC_VAL(3) | CNST_PMC_VAL(4) | CNST_PMC_VAL(5) | CNST_PMC_VAL(6) | CNST_NC_VAL;

pub const fn MMCR1_UNIT_SHIFT(pmc: u64) -> u32 { (60 - 4 * (pmc - 1)) as u32 }
pub const fn MMCR1_COMBINE_SHIFT(pmc: u64) -> u32 { (35 - (pmc - 1)) as u32 }
pub const fn MMCR1_PMCSEL_SHIFT(pmc: u64) -> u32 { (24 - ((pmc - 1) * 8)) as u32 }
pub const MMCR1_FAB_SHIFT: u32 = 36;
pub const MMCR1_DC_IC_QUAL_MASK: u64 = 3;
pub const MMCR1_DC_IC_QUAL_SHIFT: u32 = 46;
pub const fn p9_MMCR1_COMBINE_SHIFT(pmc: u64) -> u32 { (38 - ((pmc - 1) * 2)) as u32 }
pub const MMCRA_SAMP_MODE_SHIFT: u32 = 1;
pub const MMCRA_SAMP_ELIG_SHIFT: u32 = 4;
pub const MMCRA_SAMP_ELIG_MASK: u64 = 7;
pub const MMCRA_THR_CTL_SHIFT: u32 = 8;
pub const MMCRA_THR_SEL_SHIFT: u32 = 16;
pub const MMCRA_THR_CMP_SHIFT: u32 = 32;
pub const MMCRA_SDAR_MODE_SHIFT: u32 = 42;
pub const MMCRA_SDAR_MODE_TLB: u64 = 1 << MMCRA_SDAR_MODE_SHIFT;
pub const MMCRA_SDAR_MODE_NO_UPDATES: u64 = !(3u64 << MMCRA_SDAR_MODE_SHIFT);
pub const MMCRA_SDAR_MODE_DCACHE: u64 = 2 << MMCRA_SDAR_MODE_SHIFT;
pub const MMCRA_IFM_SHIFT: u32 = 30;
pub const MMCRA_THR_CTR_MANT_SHIFT: u32 = 19;
pub const MMCRA_THR_CTR_MANT_MASK: u64 = 0x7f;
pub const fn MMCRA_THR_CTR_MANT(v: u64) -> u64 { (v >> MMCRA_THR_CTR_MANT_SHIFT) & MMCRA_THR_CTR_MANT_MASK }
pub const MMCRA_THR_CTR_EXP_SHIFT: u32 = 27;
pub const MMCRA_THR_CTR_EXP_MASK: u64 = 7;
pub const fn MMCRA_THR_CTR_EXP(v: u64) -> u64 { (v >> MMCRA_THR_CTR_EXP_SHIFT) & MMCRA_THR_CTR_EXP_MASK }
pub const P10_MMCRA_THR_CTR_MANT_MASK: u64 = 0xff;
pub const fn P10_MMCRA_THR_CTR_MANT(v: u64) -> u64 { (v >> MMCRA_THR_CTR_MANT_SHIFT) & P10_MMCRA_THR_CTR_MANT_MASK }
pub const P9_MMCRA_THR_CMP_SHIFT: u32 = 45;
pub const fn MMCR2_FCS(pmc: u64) -> u64 { 1u64 << (63 - ((pmc - 1) * 9)) }
pub const fn MMCR2_FCP(pmc: u64) -> u64 { 1u64 << (62 - ((pmc - 1) * 9)) }
pub const fn MMCR2_FCWAIT(pmc: u64) -> u64 { 1u64 << (58 - ((pmc - 1) * 9)) }
pub const fn MMCR2_FCH(pmc: u64) -> u64 { 1u64 << (57 - ((pmc - 1) * 9)) }
pub const fn MMCR3_SHIFT(pmc: u64) -> u32 { (49 - (15 * (pmc - 1))) as u32 }
pub const MAX_ALT: usize = 2;
pub const MAX_PMU_COUNTERS: usize = 6;
pub const ISA207_SIER_TYPE_SHIFT: u32 = 15;
pub const ISA207_SIER_TYPE_MASK: u64 = 0x7 << ISA207_SIER_TYPE_SHIFT;
pub const ISA207_SIER_LDST_SHIFT: u32 = 1;
pub const ISA207_SIER_LDST_MASK: u64 = 0x7 << ISA207_SIER_LDST_SHIFT;
pub const ISA207_SIER_DATA_SRC_SHIFT: u32 = 53;
pub const ISA207_SIER_DATA_SRC_MASK: u64 = 0x7 << ISA207_SIER_DATA_SRC_SHIFT;
pub const fn P10_SIER2_FINISH_CYC(v: u64) -> u64 { (v >> 26) & 0x7ff }
pub const fn P10_SIER2_DISPATCH_CYC(v: u64) -> u64 { (v >> 50) & 0x7ff }

// C convenience macros retained as dependency-facing Rust functions.
#[inline] pub fn P(a: u64, b: u64) -> u64 { PERF_MEM_S(a, b) }
#[inline] pub fn PH(a: u64, b: u64) -> u64 { P(LVL, HIT) | P(a, b) }
#[inline] pub fn PM(a: u64, b: u64) -> u64 { P(LVL, MISS) | P(a, b) }
#[inline] pub fn LEVEL(x: u64) -> u64 { P(LVLNUM, x) }
pub const REM: u64 = P(REMOTE, REMOTE);

extern "C" {
    pub fn isa207_get_constraint(event: u64, maskp: *mut libc::c_ulong, valp: *mut libc::c_ulong, event_config1: u64) -> libc::c_int;
    pub fn isa207_compute_mmcr(event: *const u64, n_ev: libc::c_int, hwc: *const libc::c_uint, mmcr: *mut mmcr_regs, pevents: *mut *mut perf_event, flags: u32) -> libc::c_int;
    pub fn isa207_disable_pmc(pmc: libc::c_uint, mmcr: *mut mmcr_regs);
    pub fn isa207_get_alternatives(event: u64, alt: *mut u64, size: libc::c_int, flags: libc::c_uint, ev_alt: *const [libc::c_uint; MAX_ALT]) -> libc::c_int;
    pub fn isa207_get_mem_data_src(dsrc: *mut perf_mem_data_src, flags: u32, regs: *mut pt_regs);
    pub fn isa207_get_mem_weight(weight: *mut u64, type_: u64);
    pub fn isa3XX_check_attr_config(ev: *mut perf_event) -> libc::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
