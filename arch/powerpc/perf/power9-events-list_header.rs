/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Performance counter support for POWER9 processors.
 *
 * Copyright 2016 Madhavan Srinivasan, IBM Corporation.
 */

/*
 * Power9 event codes.
 *
 * The C EVENT(name, code) macro invocations are represented as constants.
 */
pub const PM_CYC: u64 = 0x0001e;
pub const PM_ICT_NOSLOT_CYC: u64 = 0x100f8;
pub const PM_CMPLU_STALL: u64 = 0x1e054;
pub const PM_INST_CMPL: u64 = 0x00002;
pub const PM_BR_CMPL: u64 = 0x4d05e;
pub const PM_BR_MPRED_CMPL: u64 = 0x400f6;

/* All L1 D cache load references counted at finish, gated by reject */
pub const PM_LD_REF_L1: u64 = 0x100fc;
/* Load Missed L1 */
pub const PM_LD_MISS_L1_FIN: u64 = 0x2c04e;
pub const PM_LD_MISS_L1: u64 = 0x3e054;
/* Alternate event code for PM_LD_MISS_L1 */
pub const PM_LD_MISS_L1_ALT: u64 = 0x400f0;
/* Store Missed L1 */
pub const PM_ST_MISS_L1: u64 = 0x300f0;
/* L1 cache data prefetches */
pub const PM_L1_PREF: u64 = 0x20054;
/* Instruction fetches from L1 */
pub const PM_INST_FROM_L1: u64 = 0x04080;
/* Demand iCache Miss */
pub const PM_L1_ICACHE_MISS: u64 = 0x200fd;
/* Instruction Demand sectors wriittent into IL1 */
pub const PM_L1_DEMAND_WRITE: u64 = 0x0408c;
/* Instruction prefetch written into IL1 */
pub const PM_IC_PREF_WRITE: u64 = 0x0488c;
/* The data cache was reloaded from local core's L3 due to a demand load */
pub const PM_DATA_FROM_L3: u64 = 0x4c042;
/* Demand LD - L3 Miss (not L2 hit and not L3 hit) */
pub const PM_DATA_FROM_L3MISS: u64 = 0x300fe;
/* All successful D-side store dispatches for this thread */
pub const PM_L2_ST: u64 = 0x16880;
/* All successful D-side store dispatches for this thread that were L2 Miss */
pub const PM_L2_ST_MISS: u64 = 0x26880;
/* Total HW L3 prefetches(Load+store) */
pub const PM_L3_PREF_ALL: u64 = 0x4e052;
/* Data PTEG reload */
pub const PM_DTLB_MISS: u64 = 0x300fc;
/* ITLB Reloaded */
pub const PM_ITLB_MISS: u64 = 0x400fc;
/* Run_Instructions */
pub const PM_RUN_INST_CMPL: u64 = 0x500fa;
/* Alternate event code for PM_RUN_INST_CMPL */
pub const PM_RUN_INST_CMPL_ALT: u64 = 0x400fa;
/* Run_cycles */
pub const PM_RUN_CYC: u64 = 0x600f4;
/* Alternate event code for Run_cycles */
pub const PM_RUN_CYC_ALT: u64 = 0x200f4;
/* Instruction Dispatched */
pub const PM_INST_DISP: u64 = 0x200f2;
pub const PM_INST_DISP_ALT: u64 = 0x300f2;
/* Branch event that are not strongly biased */
pub const PM_BR_2PATH: u64 = 0x20036;
/* ALternate branch event that are not strongly biased */
pub const PM_BR_2PATH_ALT: u64 = 0x40036;

/* Blacklisted events */
pub const PM_MRK_ST_DONE_L2: u64 = 0x10134;
pub const PM_RADIX_PWC_L1_HIT: u64 = 0x1f056;
pub const PM_FLOP_CMPL: u64 = 0x100f4;
pub const PM_MRK_NTF_FIN: u64 = 0x20112;
pub const PM_RADIX_PWC_L2_HIT: u64 = 0x2d024;
pub const PM_IFETCH_THROTTLE: u64 = 0x3405e;
pub const PM_MRK_L2_TM_ST_ABORT_SISTER: u64 = 0x3e15c;
pub const PM_RADIX_PWC_L3_HIT: u64 = 0x3f056;
pub const PM_RUN_CYC_SMT2_MODE: u64 = 0x3006c;
pub const PM_TM_TX_PASS_RUN_INST: u64 = 0x4e014;
pub const PM_DISP_HELD_SYNC_HOLD: u64 = 0x4003c;
pub const PM_DTLB_MISS_16G: u64 = 0x1c058;
pub const PM_DERAT_MISS_2M: u64 = 0x1c05a;
pub const PM_DTLB_MISS_2M: u64 = 0x1c05c;
pub const PM_MRK_DTLB_MISS_1G: u64 = 0x1d15c;
pub const PM_DTLB_MISS_4K: u64 = 0x2c056;
pub const PM_DERAT_MISS_1G: u64 = 0x2c05a;
pub const PM_MRK_DERAT_MISS_2M: u64 = 0x2d152;
pub const PM_MRK_DTLB_MISS_4K: u64 = 0x2d156;
pub const PM_MRK_DTLB_MISS_16G: u64 = 0x2d15e;
pub const PM_DTLB_MISS_64K: u64 = 0x3c056;
pub const PM_MRK_DERAT_MISS_1G: u64 = 0x3d152;
pub const PM_MRK_DTLB_MISS_64K: u64 = 0x3d156;
pub const PM_DTLB_MISS_16M: u64 = 0x4c056;
pub const PM_DTLB_MISS_1G: u64 = 0x4c05a;
pub const PM_MRK_DTLB_MISS_16M: u64 = 0x4c15e;

/*
 * Memory Access Events
 *
 * Primary PMU event used here is PM_MRK_INST_CMPL (0x401e0)
 * To enable capturing of memory profiling, these MMCRA bits
 * needs to be programmed and corresponding raw event format
 * encoding.
 *
 * MMCRA bits encoding needed are
 *     SM (Sampling Mode)
 *     EM (Eligibility for Random Sampling)
 *     TECE (Threshold Event Counter Event)
 *     TS (Threshold Start Event)
 *     TE (Threshold End Event)
 *
 * Corresponding Raw Encoding bits:
 *     sample [EM,SM]
 *     thresh_sel (TECE)
 *     thresh start (TS)
 *     thresh end (TE)
 */
pub const MEM_LOADS: u64 = 0x34340401e0;
pub const MEM_STORES: u64 = 0x343c0401e0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
