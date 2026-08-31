/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2012 ARM Ltd.
 */

// C header dependencies removed: <assert.h>, <asm/bug.h>.

pub const fn BIT(n: u32) -> u64 {
    1u64 << n
}

pub const fn GENMASK(h: u32, l: u32) -> u64 {
    if h == 63 {
        u64::MAX << l
    } else {
        ((1u64 << (h + 1)) - 1) & !((1u64 << l) - 1)
    }
}

pub const ARMV8_PMU_MAX_COUNTERS: u64 = 32;
pub const ARMV8_PMU_COUNTER_MASK: u64 = ARMV8_PMU_MAX_COUNTERS - 1;

/*
 * Common architectural and microarchitectural event numbers.
 */
pub const ARMV8_PMUV3_PERFCTR_SW_INCR: u64 = 0x0000;
pub const ARMV8_PMUV3_PERFCTR_L1I_CACHE_REFILL: u64 = 0x0001;
pub const ARMV8_PMUV3_PERFCTR_L1I_TLB_REFILL: u64 = 0x0002;
pub const ARMV8_PMUV3_PERFCTR_L1D_CACHE_REFILL: u64 = 0x0003;
pub const ARMV8_PMUV3_PERFCTR_L1D_CACHE: u64 = 0x0004;
pub const ARMV8_PMUV3_PERFCTR_L1D_TLB_REFILL: u64 = 0x0005;
pub const ARMV8_PMUV3_PERFCTR_LD_RETIRED: u64 = 0x0006;
pub const ARMV8_PMUV3_PERFCTR_ST_RETIRED: u64 = 0x0007;
pub const ARMV8_PMUV3_PERFCTR_INST_RETIRED: u64 = 0x0008;
pub const ARMV8_PMUV3_PERFCTR_EXC_TAKEN: u64 = 0x0009;
pub const ARMV8_PMUV3_PERFCTR_EXC_RETURN: u64 = 0x000A;
pub const ARMV8_PMUV3_PERFCTR_CID_WRITE_RETIRED: u64 = 0x000B;
pub const ARMV8_PMUV3_PERFCTR_PC_WRITE_RETIRED: u64 = 0x000C;
pub const ARMV8_PMUV3_PERFCTR_BR_IMMED_RETIRED: u64 = 0x000D;
pub const ARMV8_PMUV3_PERFCTR_BR_RETURN_RETIRED: u64 = 0x000E;
pub const ARMV8_PMUV3_PERFCTR_UNALIGNED_LDST_RETIRED: u64 = 0x000F;
pub const ARMV8_PMUV3_PERFCTR_BR_MIS_PRED: u64 = 0x0010;
pub const ARMV8_PMUV3_PERFCTR_CPU_CYCLES: u64 = 0x0011;
pub const ARMV8_PMUV3_PERFCTR_BR_PRED: u64 = 0x0012;
pub const ARMV8_PMUV3_PERFCTR_MEM_ACCESS: u64 = 0x0013;
pub const ARMV8_PMUV3_PERFCTR_L1I_CACHE: u64 = 0x0014;
pub const ARMV8_PMUV3_PERFCTR_L1D_CACHE_WB: u64 = 0x0015;
pub const ARMV8_PMUV3_PERFCTR_L2D_CACHE: u64 = 0x0016;
pub const ARMV8_PMUV3_PERFCTR_L2D_CACHE_REFILL: u64 = 0x0017;
pub const ARMV8_PMUV3_PERFCTR_L2D_CACHE_WB: u64 = 0x0018;
pub const ARMV8_PMUV3_PERFCTR_BUS_ACCESS: u64 = 0x0019;
pub const ARMV8_PMUV3_PERFCTR_MEMORY_ERROR: u64 = 0x001A;
pub const ARMV8_PMUV3_PERFCTR_INST_SPEC: u64 = 0x001B;
pub const ARMV8_PMUV3_PERFCTR_TTBR_WRITE_RETIRED: u64 = 0x001C;
pub const ARMV8_PMUV3_PERFCTR_BUS_CYCLES: u64 = 0x001D;
pub const ARMV8_PMUV3_PERFCTR_CHAIN: u64 = 0x001E;
pub const ARMV8_PMUV3_PERFCTR_L1D_CACHE_ALLOCATE: u64 = 0x001F;
pub const ARMV8_PMUV3_PERFCTR_L2D_CACHE_ALLOCATE: u64 = 0x0020;
pub const ARMV8_PMUV3_PERFCTR_BR_RETIRED: u64 = 0x0021;
pub const ARMV8_PMUV3_PERFCTR_BR_MIS_PRED_RETIRED: u64 = 0x0022;
pub const ARMV8_PMUV3_PERFCTR_STALL_FRONTEND: u64 = 0x0023;
pub const ARMV8_PMUV3_PERFCTR_STALL_BACKEND: u64 = 0x0024;
pub const ARMV8_PMUV3_PERFCTR_L1D_TLB: u64 = 0x0025;
pub const ARMV8_PMUV3_PERFCTR_L1I_TLB: u64 = 0x0026;
pub const ARMV8_PMUV3_PERFCTR_L2I_CACHE: u64 = 0x0027;
pub const ARMV8_PMUV3_PERFCTR_L2I_CACHE_REFILL: u64 = 0x0028;
pub const ARMV8_PMUV3_PERFCTR_L3D_CACHE_ALLOCATE: u64 = 0x0029;
pub const ARMV8_PMUV3_PERFCTR_L3D_CACHE_REFILL: u64 = 0x002A;
pub const ARMV8_PMUV3_PERFCTR_L3D_CACHE: u64 = 0x002B;
pub const ARMV8_PMUV3_PERFCTR_L3D_CACHE_WB: u64 = 0x002C;
pub const ARMV8_PMUV3_PERFCTR_L2D_TLB_REFILL: u64 = 0x002D;
pub const ARMV8_PMUV3_PERFCTR_L2I_TLB_REFILL: u64 = 0x002E;
pub const ARMV8_PMUV3_PERFCTR_L2D_TLB: u64 = 0x002F;
pub const ARMV8_PMUV3_PERFCTR_L2I_TLB: u64 = 0x0030;
pub const ARMV8_PMUV3_PERFCTR_REMOTE_ACCESS: u64 = 0x0031;
pub const ARMV8_PMUV3_PERFCTR_LL_CACHE: u64 = 0x0032;
pub const ARMV8_PMUV3_PERFCTR_LL_CACHE_MISS: u64 = 0x0033;
pub const ARMV8_PMUV3_PERFCTR_DTLB_WALK: u64 = 0x0034;
pub const ARMV8_PMUV3_PERFCTR_ITLB_WALK: u64 = 0x0035;
pub const ARMV8_PMUV3_PERFCTR_LL_CACHE_RD: u64 = 0x0036;
pub const ARMV8_PMUV3_PERFCTR_LL_CACHE_MISS_RD: u64 = 0x0037;
pub const ARMV8_PMUV3_PERFCTR_REMOTE_ACCESS_RD: u64 = 0x0038;
pub const ARMV8_PMUV3_PERFCTR_L1D_CACHE_LMISS_RD: u64 = 0x0039;
pub const ARMV8_PMUV3_PERFCTR_OP_RETIRED: u64 = 0x003A;
pub const ARMV8_PMUV3_PERFCTR_OP_SPEC: u64 = 0x003B;
pub const ARMV8_PMUV3_PERFCTR_STALL: u64 = 0x003C;
pub const ARMV8_PMUV3_PERFCTR_STALL_SLOT_BACKEND: u64 = 0x003D;
pub const ARMV8_PMUV3_PERFCTR_STALL_SLOT_FRONTEND: u64 = 0x003E;
pub const ARMV8_PMUV3_PERFCTR_STALL_SLOT: u64 = 0x003F;

/* Statistical profiling extension microarchitectural events */
pub const ARMV8_SPE_PERFCTR_SAMPLE_POP: u64 = 0x4000;
pub const ARMV8_SPE_PERFCTR_SAMPLE_FEED: u64 = 0x4001;
pub const ARMV8_SPE_PERFCTR_SAMPLE_FILTRATE: u64 = 0x4002;
pub const ARMV8_SPE_PERFCTR_SAMPLE_COLLISION: u64 = 0x4003;

/* AMUv1 architecture events */
pub const ARMV8_AMU_PERFCTR_CNT_CYCLES: u64 = 0x4004;
pub const ARMV8_AMU_PERFCTR_STALL_BACKEND_MEM: u64 = 0x4005;

/* long-latency read miss events */
pub const ARMV8_PMUV3_PERFCTR_L1I_CACHE_LMISS: u64 = 0x4006;
pub const ARMV8_PMUV3_PERFCTR_L2D_CACHE_LMISS_RD: u64 = 0x4009;
pub const ARMV8_PMUV3_PERFCTR_L2I_CACHE_LMISS: u64 = 0x400A;
pub const ARMV8_PMUV3_PERFCTR_L3D_CACHE_LMISS_RD: u64 = 0x400B;

/* Trace buffer events */
pub const ARMV8_PMUV3_PERFCTR_TRB_WRAP: u64 = 0x400C;
pub const ARMV8_PMUV3_PERFCTR_TRB_TRIG: u64 = 0x400E;

/* Trace unit events */
pub const ARMV8_PMUV3_PERFCTR_TRCEXTOUT0: u64 = 0x4010;
pub const ARMV8_PMUV3_PERFCTR_TRCEXTOUT1: u64 = 0x4011;
pub const ARMV8_PMUV3_PERFCTR_TRCEXTOUT2: u64 = 0x4012;
pub const ARMV8_PMUV3_PERFCTR_TRCEXTOUT3: u64 = 0x4013;
pub const ARMV8_PMUV3_PERFCTR_CTI_TRIGOUT4: u64 = 0x4018;
pub const ARMV8_PMUV3_PERFCTR_CTI_TRIGOUT5: u64 = 0x4019;
pub const ARMV8_PMUV3_PERFCTR_CTI_TRIGOUT6: u64 = 0x401A;
pub const ARMV8_PMUV3_PERFCTR_CTI_TRIGOUT7: u64 = 0x401B;

/* additional latency from alignment events */
pub const ARMV8_PMUV3_PERFCTR_LDST_ALIGN_LAT: u64 = 0x4020;
pub const ARMV8_PMUV3_PERFCTR_LD_ALIGN_LAT: u64 = 0x4021;
pub const ARMV8_PMUV3_PERFCTR_ST_ALIGN_LAT: u64 = 0x4022;

/* Armv8.5 Memory Tagging Extension events */
pub const ARMV8_MTE_PERFCTR_MEM_ACCESS_CHECKED: u64 = 0x4024;
pub const ARMV8_MTE_PERFCTR_MEM_ACCESS_CHECKED_RD: u64 = 0x4025;
pub const ARMV8_MTE_PERFCTR_MEM_ACCESS_CHECKED_WR: u64 = 0x4026;

/* ARMv8 recommended implementation defined event types */
pub const ARMV8_IMPDEF_PERFCTR_L1D_CACHE_RD: u64 = 0x0040;
pub const ARMV8_IMPDEF_PERFCTR_L1D_CACHE_WR: u64 = 0x0041;
pub const ARMV8_IMPDEF_PERFCTR_L1D_CACHE_REFILL_RD: u64 = 0x0042;
pub const ARMV8_IMPDEF_PERFCTR_L1D_CACHE_REFILL_WR: u64 = 0x0043;
pub const ARMV8_IMPDEF_PERFCTR_L1D_CACHE_REFILL_INNER: u64 = 0x0044;
pub const ARMV8_IMPDEF_PERFCTR_L1D_CACHE_REFILL_OUTER: u64 = 0x0045;
pub const ARMV8_IMPDEF_PERFCTR_L1D_CACHE_WB_VICTIM: u64 = 0x0046;
pub const ARMV8_IMPDEF_PERFCTR_L1D_CACHE_WB_CLEAN: u64 = 0x0047;
pub const ARMV8_IMPDEF_PERFCTR_L1D_CACHE_INVAL: u64 = 0x0048;

pub const ARMV8_IMPDEF_PERFCTR_L1D_TLB_REFILL_RD: u64 = 0x004C;
pub const ARMV8_IMPDEF_PERFCTR_L1D_TLB_REFILL_WR: u64 = 0x004D;
pub const ARMV8_IMPDEF_PERFCTR_L1D_TLB_RD: u64 = 0x004E;
pub const ARMV8_IMPDEF_PERFCTR_L1D_TLB_WR: u64 = 0x004F;
pub const ARMV8_IMPDEF_PERFCTR_L2D_CACHE_RD: u64 = 0x0050;
pub const ARMV8_IMPDEF_PERFCTR_L2D_CACHE_WR: u64 = 0x0051;
pub const ARMV8_IMPDEF_PERFCTR_L2D_CACHE_REFILL_RD: u64 = 0x0052;
pub const ARMV8_IMPDEF_PERFCTR_L2D_CACHE_REFILL_WR: u64 = 0x0053;

pub const ARMV8_IMPDEF_PERFCTR_L2D_CACHE_WB_VICTIM: u64 = 0x0056;
pub const ARMV8_IMPDEF_PERFCTR_L2D_CACHE_WB_CLEAN: u64 = 0x0057;
pub const ARMV8_IMPDEF_PERFCTR_L2D_CACHE_INVAL: u64 = 0x0058;

pub const ARMV8_IMPDEF_PERFCTR_L2D_TLB_REFILL_RD: u64 = 0x005C;
pub const ARMV8_IMPDEF_PERFCTR_L2D_TLB_REFILL_WR: u64 = 0x005D;
pub const ARMV8_IMPDEF_PERFCTR_L2D_TLB_RD: u64 = 0x005E;
pub const ARMV8_IMPDEF_PERFCTR_L2D_TLB_WR: u64 = 0x005F;
pub const ARMV8_IMPDEF_PERFCTR_BUS_ACCESS_RD: u64 = 0x0060;
pub const ARMV8_IMPDEF_PERFCTR_BUS_ACCESS_WR: u64 = 0x0061;
pub const ARMV8_IMPDEF_PERFCTR_BUS_ACCESS_SHARED: u64 = 0x0062;
pub const ARMV8_IMPDEF_PERFCTR_BUS_ACCESS_NOT_SHARED: u64 = 0x0063;
pub const ARMV8_IMPDEF_PERFCTR_BUS_ACCESS_NORMAL: u64 = 0x0064;
pub const ARMV8_IMPDEF_PERFCTR_BUS_ACCESS_PERIPH: u64 = 0x0065;
pub const ARMV8_IMPDEF_PERFCTR_MEM_ACCESS_RD: u64 = 0x0066;
pub const ARMV8_IMPDEF_PERFCTR_MEM_ACCESS_WR: u64 = 0x0067;
pub const ARMV8_IMPDEF_PERFCTR_UNALIGNED_LD_SPEC: u64 = 0x0068;
pub const ARMV8_IMPDEF_PERFCTR_UNALIGNED_ST_SPEC: u64 = 0x0069;
pub const ARMV8_IMPDEF_PERFCTR_UNALIGNED_LDST_SPEC: u64 = 0x006A;

pub const ARMV8_IMPDEF_PERFCTR_LDREX_SPEC: u64 = 0x006C;
pub const ARMV8_IMPDEF_PERFCTR_STREX_PASS_SPEC: u64 = 0x006D;
pub const ARMV8_IMPDEF_PERFCTR_STREX_FAIL_SPEC: u64 = 0x006E;
pub const ARMV8_IMPDEF_PERFCTR_STREX_SPEC: u64 = 0x006F;
pub const ARMV8_IMPDEF_PERFCTR_LD_SPEC: u64 = 0x0070;
pub const ARMV8_IMPDEF_PERFCTR_ST_SPEC: u64 = 0x0071;
pub const ARMV8_IMPDEF_PERFCTR_LDST_SPEC: u64 = 0x0072;
pub const ARMV8_IMPDEF_PERFCTR_DP_SPEC: u64 = 0x0073;
pub const ARMV8_IMPDEF_PERFCTR_ASE_SPEC: u64 = 0x0074;
pub const ARMV8_IMPDEF_PERFCTR_VFP_SPEC: u64 = 0x0075;
pub const ARMV8_IMPDEF_PERFCTR_PC_WRITE_SPEC: u64 = 0x0076;
pub const ARMV8_IMPDEF_PERFCTR_CRYPTO_SPEC: u64 = 0x0077;
pub const ARMV8_IMPDEF_PERFCTR_BR_IMMED_SPEC: u64 = 0x0078;
pub const ARMV8_IMPDEF_PERFCTR_BR_RETURN_SPEC: u64 = 0x0079;
pub const ARMV8_IMPDEF_PERFCTR_BR_INDIRECT_SPEC: u64 = 0x007A;

pub const ARMV8_IMPDEF_PERFCTR_ISB_SPEC: u64 = 0x007C;
pub const ARMV8_IMPDEF_PERFCTR_DSB_SPEC: u64 = 0x007D;
pub const ARMV8_IMPDEF_PERFCTR_DMB_SPEC: u64 = 0x007E;

pub const ARMV8_IMPDEF_PERFCTR_EXC_UNDEF: u64 = 0x0081;
pub const ARMV8_IMPDEF_PERFCTR_EXC_SVC: u64 = 0x0082;
pub const ARMV8_IMPDEF_PERFCTR_EXC_PABORT: u64 = 0x0083;
pub const ARMV8_IMPDEF_PERFCTR_EXC_DABORT: u64 = 0x0084;

pub const ARMV8_IMPDEF_PERFCTR_EXC_IRQ: u64 = 0x0086;
pub const ARMV8_IMPDEF_PERFCTR_EXC_FIQ: u64 = 0x0087;
pub const ARMV8_IMPDEF_PERFCTR_EXC_SMC: u64 = 0x0088;

pub const ARMV8_IMPDEF_PERFCTR_EXC_HVC: u64 = 0x008A;
pub const ARMV8_IMPDEF_PERFCTR_EXC_TRAP_PABORT: u64 = 0x008B;
pub const ARMV8_IMPDEF_PERFCTR_EXC_TRAP_DABORT: u64 = 0x008C;
pub const ARMV8_IMPDEF_PERFCTR_EXC_TRAP_OTHER: u64 = 0x008D;
pub const ARMV8_IMPDEF_PERFCTR_EXC_TRAP_IRQ: u64 = 0x008E;
pub const ARMV8_IMPDEF_PERFCTR_EXC_TRAP_FIQ: u64 = 0x008F;
pub const ARMV8_IMPDEF_PERFCTR_RC_LD_SPEC: u64 = 0x0090;
pub const ARMV8_IMPDEF_PERFCTR_RC_ST_SPEC: u64 = 0x0091;

pub const ARMV8_IMPDEF_PERFCTR_L3D_CACHE_RD: u64 = 0x00A0;
pub const ARMV8_IMPDEF_PERFCTR_L3D_CACHE_WR: u64 = 0x00A1;
pub const ARMV8_IMPDEF_PERFCTR_L3D_CACHE_REFILL_RD: u64 = 0x00A2;
pub const ARMV8_IMPDEF_PERFCTR_L3D_CACHE_REFILL_WR: u64 = 0x00A3;

pub const ARMV8_IMPDEF_PERFCTR_L3D_CACHE_WB_VICTIM: u64 = 0x00A6;
pub const ARMV8_IMPDEF_PERFCTR_L3D_CACHE_WB_CLEAN: u64 = 0x00A7;
pub const ARMV8_IMPDEF_PERFCTR_L3D_CACHE_INVAL: u64 = 0x00A8;

/*
 * Per-CPU PMCR: config reg
 */
pub const ARMV8_PMU_PMCR_E: u64 = 1 << 0; /* Enable all counters */
pub const ARMV8_PMU_PMCR_P: u64 = 1 << 1; /* Reset all counters */
pub const ARMV8_PMU_PMCR_C: u64 = 1 << 2; /* Cycle counter reset */
pub const ARMV8_PMU_PMCR_D: u64 = 1 << 3; /* CCNT counts every 64th cpu cycle */
pub const ARMV8_PMU_PMCR_X: u64 = 1 << 4; /* Export to ETM */
pub const ARMV8_PMU_PMCR_DP: u64 = 1 << 5; /* Disable CCNT if non-invasive debug*/
pub const ARMV8_PMU_PMCR_LC: u64 = 1 << 6; /* Overflow on 64 bit cycle counter */
pub const ARMV8_PMU_PMCR_LP: u64 = 1 << 7; /* Long event counter enable */
pub const ARMV8_PMU_PMCR_N: u64 = GENMASK(15, 11); /* Number of counters supported */
/* Mask for writable bits */
pub const ARMV8_PMU_PMCR_MASK: u64 = ARMV8_PMU_PMCR_E
    | ARMV8_PMU_PMCR_P
    | ARMV8_PMU_PMCR_C
    | ARMV8_PMU_PMCR_D
    | ARMV8_PMU_PMCR_X
    | ARMV8_PMU_PMCR_DP
    | ARMV8_PMU_PMCR_LC
    | ARMV8_PMU_PMCR_LP;

/*
 * PMOVSR: counters overflow flag status reg
 */
pub const ARMV8_PMU_OVSR_P: u64 = GENMASK(30, 0);
pub const ARMV8_PMU_OVSR_C: u64 = BIT(31);
/* Mask for writable bits is both P and C fields */
pub const ARMV8_PMU_OVERFLOWED_MASK: u64 = ARMV8_PMU_OVSR_P | ARMV8_PMU_OVSR_C;

/*
 * PMXEVTYPER: Event selection reg
 */
pub const ARMV8_PMU_EVTYPE_EVENT: u64 = GENMASK(15, 0); /* Mask for EVENT bits */
pub const ARMV8_PMU_EVTYPE_TH: u64 = GENMASK(43, 32);
pub const ARMV8_PMU_EVTYPE_TC: u64 = GENMASK(63, 61);

/*
 * Event filters for PMUv3
 */
pub const ARMV8_PMU_EXCLUDE_EL1: u32 = 1u32 << 31;
pub const ARMV8_PMU_EXCLUDE_EL0: u32 = 1u32 << 30;
pub const ARMV8_PMU_EXCLUDE_NS_EL1: u32 = 1u32 << 29;
pub const ARMV8_PMU_EXCLUDE_NS_EL0: u32 = 1u32 << 28;
pub const ARMV8_PMU_INCLUDE_EL2: u32 = 1u32 << 27;
pub const ARMV8_PMU_EXCLUDE_EL3: u32 = 1u32 << 26;

/*
 * PMUSERENR: user enable reg
 */
pub const ARMV8_PMU_USERENR_EN: u64 = 1 << 0; /* PMU regs can be accessed at EL0 */
pub const ARMV8_PMU_USERENR_SW: u64 = 1 << 1; /* PMSWINC can be written at EL0 */
pub const ARMV8_PMU_USERENR_CR: u64 = 1 << 2; /* Cycle counter can be read at EL0 */
pub const ARMV8_PMU_USERENR_ER: u64 = 1 << 3; /* Event counter can be read at EL0 */
/* Mask for writable bits */
pub const ARMV8_PMU_USERENR_MASK: u64 = ARMV8_PMU_USERENR_EN
    | ARMV8_PMU_USERENR_SW
    | ARMV8_PMU_USERENR_CR
    | ARMV8_PMU_USERENR_ER;

/* PMMIR_EL1.SLOTS mask */
pub const ARMV8_PMU_SLOTS: u64 = GENMASK(7, 0);
pub const ARMV8_PMU_BUS_SLOTS: u64 = GENMASK(15, 8);
pub const ARMV8_PMU_BUS_WIDTH: u64 = GENMASK(19, 16);
pub const ARMV8_PMU_THWIDTH: u64 = GENMASK(23, 20);

/*
 * This code is really good
 */

#[macro_export]
macro_rules! PMEVN_CASE {
    ($n:expr, $case_macro:ident) => {
        $n => {
            $case_macro!($n);
        }
    };
}

#[macro_export]
macro_rules! PMEVN_SWITCH {
    ($x:expr, $case_macro:ident) => {{
        match $x {
            0 => { $case_macro!(0); }
            1 => { $case_macro!(1); }
            2 => { $case_macro!(2); }
            3 => { $case_macro!(3); }
            4 => { $case_macro!(4); }
            5 => { $case_macro!(5); }
            6 => { $case_macro!(6); }
            7 => { $case_macro!(7); }
            8 => { $case_macro!(8); }
            9 => { $case_macro!(9); }
            10 => { $case_macro!(10); }
            11 => { $case_macro!(11); }
            12 => { $case_macro!(12); }
            13 => { $case_macro!(13); }
            14 => { $case_macro!(14); }
            15 => { $case_macro!(15); }
            16 => { $case_macro!(16); }
            17 => { $case_macro!(17); }
            18 => { $case_macro!(18); }
            19 => { $case_macro!(19); }
            20 => { $case_macro!(20); }
            21 => { $case_macro!(21); }
            22 => { $case_macro!(22); }
            23 => { $case_macro!(23); }
            24 => { $case_macro!(24); }
            25 => { $case_macro!(25); }
            26 => { $case_macro!(26); }
            27 => { $case_macro!(27); }
            28 => { $case_macro!(28); }
            29 => { $case_macro!(29); }
            30 => { $case_macro!(30); }
            _ => {
                // C source: WARN(1, "Invalid PMEV* index\n"); assert(0);
                panic!("Invalid PMEV* index");
            }
        }
    }};
}
