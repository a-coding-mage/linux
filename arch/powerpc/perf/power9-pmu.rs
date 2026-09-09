// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Performance counter support for POWER9 processors.
 *
 * Copyright 2009 Paul Mackerras, IBM Corporation.
 * Copyright 2013 Michael Ellerman, IBM Corporation.
 * Copyright 2016 Madhavan Srinivasan, IBM Corporation.
 */

// Dependency: isa207-common.h and power9-events-list.h supply the referenced
// types, constants, macros, and functions.

/* Raw event encoding and bit assignments are documented in the original C source. */

pub const POWER9_MMCRA_IFM1: u64 = 0x0000000040000000;
pub const POWER9_MMCRA_IFM2: u64 = 0x0000000080000000;
pub const POWER9_MMCRA_IFM3: u64 = 0x00000000C0000000;
pub const POWER9_MMCRA_BHRB_MASK: u64 = 0x00000000C0000000;

extern "C" {
    pub static mut PERF_REG_EXTENDED_MASK: u64;
    pub static isa207_pmu_format_group: attribute_group;
}

pub const PVR_POWER9_CUMULUS: u32 = 0x00002000;

/* Event constants are supplied by power9-events-list.h. */

static mut p9_dd21_bl_ev: [i32; 11] = [
    PM_MRK_ST_DONE_L2, PM_RADIX_PWC_L1_HIT, PM_FLOP_CMPL, PM_MRK_NTF_FIN,
    PM_RADIX_PWC_L2_HIT, PM_IFETCH_THROTTLE, PM_MRK_L2_TM_ST_ABORT_SISTER,
    PM_RADIX_PWC_L3_HIT, PM_RUN_CYC_SMT2_MODE, PM_TM_TX_PASS_RUN_INST,
    PM_DISP_HELD_SYNC_HOLD,
];

static mut p9_dd22_bl_ev: [i32; 16] = [
    PM_DTLB_MISS_16G, PM_DERAT_MISS_2M, PM_DTLB_MISS_2M, PM_MRK_DTLB_MISS_1G,
    PM_DTLB_MISS_4K, PM_DERAT_MISS_1G, PM_MRK_DERAT_MISS_2M,
    PM_MRK_DTLB_MISS_4K, PM_MRK_DTLB_MISS_16G, PM_DTLB_MISS_64K,
    PM_MRK_DERAT_MISS_1G, PM_MRK_DTLB_MISS_64K, PM_DISP_HELD_SYNC_HOLD,
    PM_DTLB_MISS_16M, PM_DTLB_MISS_1G, PM_MRK_DTLB_MISS_16M,
];

static power9_event_alternatives: [[u64; MAX_ALT]; 5] = [
    [PM_BR_2PATH, PM_BR_2PATH_ALT],
    [PM_INST_DISP, PM_INST_DISP_ALT],
    [PM_RUN_CYC_ALT, PM_RUN_CYC],
    [PM_LD_MISS_L1, PM_LD_MISS_L1_ALT],
    [PM_RUN_INST_CMPL_ALT, PM_RUN_INST_CMPL],
];

unsafe fn power9_get_alternatives(event: u64, flags: u32, alt: *mut u64) -> i32 {
    isa207_get_alternatives(event, alt, power9_event_alternatives.len(), flags,
                            power9_event_alternatives.as_ptr())
}

unsafe fn power9_check_attr_config(ev: *mut perf_event) -> i32 {
    let event = (*ev).attr.config;
    let val = (event >> EVENT_SAMPLE_SHIFT) & EVENT_SAMPLE_MASK;
    if val == 0xC || isa3XX_check_attr_config(ev) != 0 { -EINVAL } else { 0 }
}

/* GENERIC_EVENT_ATTR and CACHE_EVENT_ATTR declarations from the C source. */
GENERIC_EVENT_ATTR!(cpu_cycles, PM_CYC);
GENERIC_EVENT_ATTR!(stalled_cycles_frontend, PM_ICT_NOSLOT_CYC);
GENERIC_EVENT_ATTR!(stalled_cycles_backend, PM_CMPLU_STALL);
GENERIC_EVENT_ATTR!(instructions, PM_INST_CMPL);
GENERIC_EVENT_ATTR!(branch_instructions, PM_BR_CMPL);
GENERIC_EVENT_ATTR!(branch_misses, PM_BR_MPRED_CMPL);
GENERIC_EVENT_ATTR!(cache_references, PM_LD_REF_L1);
GENERIC_EVENT_ATTR!(cache_misses, PM_LD_MISS_L1_FIN);
GENERIC_EVENT_ATTR!(mem_loads, MEM_LOADS);
GENERIC_EVENT_ATTR!(mem_stores, MEM_STORES);
CACHE_EVENT_ATTR!(l1_dcache_load_misses, PM_LD_MISS_L1_FIN);
CACHE_EVENT_ATTR!(l1_dcache_loads, PM_LD_REF_L1);
CACHE_EVENT_ATTR!(l1_dcache_prefetches, PM_L1_PREF);
CACHE_EVENT_ATTR!(l1_dcache_store_misses, PM_ST_MISS_L1);
CACHE_EVENT_ATTR!(l1_icache_load_misses, PM_L1_ICACHE_MISS);
CACHE_EVENT_ATTR!(l1_icache_loads, PM_INST_FROM_L1);
CACHE_EVENT_ATTR!(l1_icache_prefetches, PM_IC_PREF_WRITE);
CACHE_EVENT_ATTR!(llc_load_misses, PM_DATA_FROM_L3MISS);
CACHE_EVENT_ATTR!(llc_loads, PM_DATA_FROM_L3);
CACHE_EVENT_ATTR!(llc_prefetches, PM_L3_PREF_ALL);
CACHE_EVENT_ATTR!(branch_load_misses, PM_BR_MPRED_CMPL);
CACHE_EVENT_ATTR!(branch_loads, PM_BR_CMPL);
CACHE_EVENT_ATTR!(dtlb_load_misses, PM_DTLB_MISS);
CACHE_EVENT_ATTR!(itlb_load_misses, PM_ITLB_MISS);

PMU_FORMAT_ATTR!(event, "config:0-51");
PMU_FORMAT_ATTR!(pmcxsel, "config:0-7");
PMU_FORMAT_ATTR!(mark, "config:8");
PMU_FORMAT_ATTR!(combine, "config:10-11");
PMU_FORMAT_ATTR!(unit, "config:12-15");
PMU_FORMAT_ATTR!(pmc, "config:16-19");
PMU_FORMAT_ATTR!(cache_sel, "config:20-23");
PMU_FORMAT_ATTR!(sample_mode, "config:24-28");
PMU_FORMAT_ATTR!(thresh_sel, "config:29-31");
PMU_FORMAT_ATTR!(thresh_stop, "config:32-35");
PMU_FORMAT_ATTR!(thresh_start, "config:36-39");
PMU_FORMAT_ATTR!(thresh_cmp, "config:40-49");
PMU_FORMAT_ATTR!(sdar_mode, "config:50-51");

static mut power9_generic_events: [i32; 8] = [
    PM_CYC, PM_ICT_NOSLOT_CYC, PM_CMPLU_STALL, PM_INST_CMPL,
    PM_BR_CMPL, PM_BR_MPRED_CMPL, PM_LD_REF_L1, PM_LD_MISS_L1_FIN,
];

unsafe fn power9_bhrb_filter_map(branch_sample_type: u64) -> u64 {
    let mut pmu_bhrb_filter = 0;
    /* BHRB and regular PMU events share the same privilege state filter. */
    if branch_sample_type & PERF_SAMPLE_BRANCH_ANY != 0 { return pmu_bhrb_filter; }
    if branch_sample_type & PERF_SAMPLE_BRANCH_ANY_RETURN != 0 { return u64::MAX; }
    if branch_sample_type & PERF_SAMPLE_BRANCH_IND_CALL != 0 { return u64::MAX; }
    if branch_sample_type & PERF_SAMPLE_BRANCH_CALL != 0 { return u64::MAX; }
    if branch_sample_type & PERF_SAMPLE_BRANCH_ANY_CALL != 0 {
        pmu_bhrb_filter |= POWER9_MMCRA_IFM1;
        return pmu_bhrb_filter;
    }
    u64::MAX
}

unsafe fn power9_config_bhrb(mut pmu_bhrb_filter: u64) {
    pmu_bhrb_filter &= POWER9_MMCRA_BHRB_MASK;
    mtspr(SPRN_MMCRA, mfspr(SPRN_MMCRA) | pmu_bhrb_filter);
}

/* Table of generalized cache-related events. 0 means unsupported, -1 nonsensical. */
static mut power9_cache_events: [[[u64; C_RESULT_MAX]; C_OP_MAX]; C_MAX] = [
    [[PM_LD_REF_L1, PM_LD_MISS_L1_FIN, 0], [0, PM_ST_MISS_L1, 0], [PM_L1_PREF, 0, 0]],
    [[PM_INST_FROM_L1, PM_L1_ICACHE_MISS, 0], [PM_L1_DEMAND_WRITE, u64::MAX, 0], [PM_IC_PREF_WRITE, 0, 0]],
    [[PM_DATA_FROM_L3, PM_DATA_FROM_L3MISS, 0], [0, 0, 0], [PM_L3_PREF_ALL, 0, 0]],
    [[0, PM_DTLB_MISS, 0], [u64::MAX, u64::MAX, 0], [u64::MAX, u64::MAX, 0]],
    [[0, PM_ITLB_MISS, 0], [u64::MAX, u64::MAX, 0], [u64::MAX, u64::MAX, 0]],
    [[PM_BR_CMPL, PM_BR_MPRED_CMPL, 0], [u64::MAX, u64::MAX, 0], [u64::MAX, u64::MAX, 0]],
    [[u64::MAX, u64::MAX, 0], [u64::MAX, u64::MAX, 0], [u64::MAX, u64::MAX, 0]],
];

static mut power9_pmu: power_pmu = power_pmu {
    name: "POWER9", n_counter: MAX_PMU_COUNTERS,
    add_fields: ISA207_ADD_FIELDS, test_adder: ISA207_TEST_ADDER,
    group_constraint_mask: CNST_CACHE_PMC4_MASK, group_constraint_val: CNST_CACHE_PMC4_VAL,
    compute_mmcr: isa207_compute_mmcr, config_bhrb: power9_config_bhrb,
    bhrb_filter_map: power9_bhrb_filter_map, get_constraint: isa207_get_constraint,
    get_alternatives: power9_get_alternatives, get_mem_data_src: isa207_get_mem_data_src,
    get_mem_weight: isa207_get_mem_weight, disable_pmc: isa207_disable_pmc,
    flags: PPMU_HAS_SIER | PPMU_ARCH_207S, n_generic: power9_generic_events.len(),
    generic_events: power9_generic_events.as_mut_ptr(), cache_events: power9_cache_events.as_mut_ptr(),
    attr_groups: core::ptr::null(), bhrb_nr: 32,
    capabilities: PERF_PMU_CAP_EXTENDED_REGS, check_attr_config: power9_check_attr_config,
};

pub unsafe fn init_power9_pmu() -> i32 {
    let mut rc = 0;
    let pvr = mfspr(SPRN_PVR) as u32;
    if PVR_VER(pvr) != PVR_POWER9 { return -ENODEV; }
    if pvr & PVR_POWER9_CUMULUS == 0 {
        if PVR_CFG(pvr) == 2 && PVR_MIN(pvr) == 1 {
            power9_pmu.blacklist_ev = p9_dd21_bl_ev.as_mut_ptr();
            power9_pmu.n_blacklist_ev = p9_dd21_bl_ev.len();
        } else if PVR_CFG(pvr) == 2 && PVR_MIN(pvr) == 2 {
            power9_pmu.blacklist_ev = p9_dd22_bl_ev.as_mut_ptr();
            power9_pmu.n_blacklist_ev = p9_dd22_bl_ev.len();
        }
    }
    PERF_REG_EXTENDED_MASK = PERF_REG_PMU_MASK_300;
    rc = register_power_pmu(&mut power9_pmu);
    if rc != 0 { return rc; }
    (*cur_cpu_spec).cpu_user_features2 |= PPC_FEATURE2_EBB;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
