/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Performance counter support for POWER12 processors.
 *
 * Copyright 2026 Athira Rajeev, IBM Corporation.
 */

/*
 * Power12 event codes.
 *
 * The C source expresses these through the EVENT(name, code) macro.  Rust
 * constants preserve the event names and integer encodings directly.
 */
pub const PM_CYC: u64 = 0x600f4;
pub const PM_DISP_STALL_CYC: u64 = 0x100f8;
pub const PM_EXEC_STALL: u64 = 0x30008;
pub const PM_INST_CMPL: u64 = 0x500fa;
pub const PM_BR_CMPL: u64 = 0x4d05e;
pub const PM_BR_MPRED_CMPL: u64 = 0x400f6;
pub const PM_BR_FIN: u64 = 0x10068;
pub const PM_MPRED_BR_FIN: u64 = 0x27098;
pub const PM_LD_DEMAND_MISS_L1_FIN: u64 = 0x400f0;

/* All L1 D cache load references counted at finish, gated by reject */
pub const PM_LD_REF_L1: u64 = 0x100fc;
/* Load Missed L1 */
pub const PM_LD_MISS_L1: u64 = 0x3e054;
/* Store Missed L1 */
pub const PM_ST_MISS_L1: u64 = 0x300f0;
/* L1 cache data prefetches */
pub const PM_LD_PREFETCH_CACHE_LINE_MISS: u64 = 0x1002c;
/* Demand iCache Miss */
pub const PM_L1_ICACHE_MISS: u64 = 0x200fc;
/* Instruction fetches from L1 */
pub const PM_INST_FROM_L1: u64 = 0x04080;
/* Instruction Demand sectors writtent into IL1 */
pub const PM_INST_FROM_L1MISS: u64 = 0x03F00000001C040;
/* Instruction prefetch written into IL1 */
pub const PM_IC_PREF_REQ: u64 = 0x040a0;
/* The data cache was reloaded from local core's L3 due to a demand load */
pub const PM_DATA_FROM_L3: u64 = 0x10340000003C040;
/* Demand LD - L3 Miss (not L2 hit and not L3 hit) */
pub const PM_DATA_FROM_L3MISS: u64 = 0x300fe;
/* All successful D-side store dispatches for this thread */
pub const PM_L2_ST: u64 = 0x010000046080;
/* All successful D-side store dispatches for this thread that were L2 Miss */
pub const PM_L2_ST_MISS: u64 = 0x26880;
/* Total HW L3 prefetches(Load+store) */
pub const PM_L3_PF_MISS_L3: u64 = 0x100000016080;
/* Data PTEG reload */
pub const PM_DTLB_MISS: u64 = 0x300fc;
/* ITLB Reloaded */
pub const PM_ITLB_MISS: u64 = 0x400fc;

pub const PM_CYC_ALT: u64 = 0x0001e;
pub const PM_INST_CMPL_ALT: u64 = 0x00002;

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

pub const MEM_LOADS: u64 = 0x35340401e0;
pub const MEM_STORES: u64 = 0x353c0401e0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
