/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Performance counter support for POWER8 processors.
 *
 * Copyright 2014 Sukadev Bhattiprolu, IBM Corporation.
 */

/*
 * Power8 event codes.
 */
pub const PM_CYC: u32 = 0x0001e;
pub const PM_GCT_NOSLOT_CYC: u32 = 0x100f8;
pub const PM_CMPLU_STALL: u32 = 0x4000a;
pub const PM_INST_CMPL: u32 = 0x00002;
pub const PM_BRU_FIN: u32 = 0x10068;
pub const PM_BR_MPRED_CMPL: u32 = 0x400f6;

/* All L1 D cache load references counted at finish, gated by reject */
pub const PM_LD_REF_L1: u32 = 0x100ee;
/* Load Missed L1 */
pub const PM_LD_MISS_L1: u32 = 0x3e054;
/* Store Missed L1 */
pub const PM_ST_MISS_L1: u32 = 0x300f0;
/* L1 cache data prefetches */
pub const PM_L1_PREF: u32 = 0x0d8b8;
/* Instruction fetches from L1 */
pub const PM_INST_FROM_L1: u32 = 0x04080;
/* Demand iCache Miss */
pub const PM_L1_ICACHE_MISS: u32 = 0x200fd;
/* Instruction Demand sectors wriittent into IL1 */
pub const PM_L1_DEMAND_WRITE: u32 = 0x0408c;
/* Instruction prefetch written into IL1 */
pub const PM_IC_PREF_WRITE: u32 = 0x0408e;
/* The data cache was reloaded from local core's L3 due to a demand load */
pub const PM_DATA_FROM_L3: u32 = 0x4c042;
/* Demand LD - L3 Miss (not L2 hit and not L3 hit) */
pub const PM_DATA_FROM_L3MISS: u32 = 0x300fe;
/* All successful D-side store dispatches for this thread */
pub const PM_L2_ST: u32 = 0x17080;
/* All successful D-side store dispatches for this thread that were L2 Miss */
pub const PM_L2_ST_MISS: u32 = 0x17082;
/* Total HW L3 prefetches(Load+store) */
pub const PM_L3_PREF_ALL: u32 = 0x4e052;
/* Data PTEG reload */
pub const PM_DTLB_MISS: u32 = 0x300fc;
/* ITLB Reloaded */
pub const PM_ITLB_MISS: u32 = 0x400fc;
/* Run_Instructions */
pub const PM_RUN_INST_CMPL: u32 = 0x500fa;
/* Alternate event code for PM_RUN_INST_CMPL */
pub const PM_RUN_INST_CMPL_ALT: u32 = 0x400fa;
/* Run_cycles */
pub const PM_RUN_CYC: u32 = 0x600f4;
/* Alternate event code for Run_cycles */
pub const PM_RUN_CYC_ALT: u32 = 0x200f4;
/* Marked store completed */
pub const PM_MRK_ST_CMPL: u32 = 0x10134;
/* Alternate event code for Marked store completed */
pub const PM_MRK_ST_CMPL_ALT: u32 = 0x301e2;
/* Marked two path branch */
pub const PM_BR_MRK_2PATH: u32 = 0x10138;
/* Alternate event code for PM_BR_MRK_2PATH */
pub const PM_BR_MRK_2PATH_ALT: u32 = 0x40138;
/* L3 castouts in Mepf state */
pub const PM_L3_CO_MEPF: u32 = 0x18082;
/* Alternate event code for PM_L3_CO_MEPF */
pub const PM_L3_CO_MEPF_ALT: u32 = 0x3e05e;
/* Data cache was reloaded from a location other than L2 due to a marked load */
pub const PM_MRK_DATA_FROM_L2MISS: u32 = 0x1d14e;
/* Alternate event code for PM_MRK_DATA_FROM_L2MISS */
pub const PM_MRK_DATA_FROM_L2MISS_ALT: u32 = 0x401e8;
/* Alternate event code for  PM_CMPLU_STALL */
pub const PM_CMPLU_STALL_ALT: u32 = 0x1e054;
/* Two path branch */
pub const PM_BR_2PATH: u32 = 0x20036;
/* Alternate event code for PM_BR_2PATH */
pub const PM_BR_2PATH_ALT: u32 = 0x40036;
/* # PPC Dispatched */
pub const PM_INST_DISP: u32 = 0x200f2;
/* Alternate event code for PM_INST_DISP */
pub const PM_INST_DISP_ALT: u32 = 0x300f2;
/* Marked filter Match */
pub const PM_MRK_FILT_MATCH: u32 = 0x2013c;
/* Alternate event code for PM_MRK_FILT_MATCH */
pub const PM_MRK_FILT_MATCH_ALT: u32 = 0x3012e;
/* Alternate event code for PM_LD_MISS_L1 */
pub const PM_LD_MISS_L1_ALT: u32 = 0x400f0;
/*
 * Memory Access Event -- mem_access
 * Primary PMU event used here is PM_MRK_INST_CMPL, along with
 * Random Load/Store Facility Sampling (RIS) in Random sampling mode (MMCRA[SM]).
 */
pub const MEM_ACCESS: u32 = 0x10401e0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
