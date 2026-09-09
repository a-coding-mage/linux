// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Performance counter support for POWER8 processors.
 *
 * Copyright 2009 Paul Mackerras, IBM Corporation.
 * Copyright 2013 Michael Ellerman, IBM Corporation.
 */

// Dependency declarations and macro-generated kernel attributes are supplied by isa207-common.h.

/* Some power8 event codes are supplied by power8-events-list.h. */

pub const POWER8_MMCRA_IFM1: u64 = 0x0000000040000000;
pub const POWER8_MMCRA_IFM2: u64 = 0x0000000080000000;
pub const POWER8_MMCRA_IFM3: u64 = 0x00000000c0000000;
pub const POWER8_MMCRA_BHRB_MASK: u64 = 0x00000000c0000000;

/* PowerISA v2.07 raw event encoding and bit layout: see the source comments above. */

extern "C" {
    pub static isa207_pmu_format_group: attribute_group;
    fn isa207_get_alternatives(event: u64, alt: *mut u64, n: usize, flags: u32,
                                alternatives: *const [u32; MAX_ALT]) -> i32;
    fn mfspr(spr: u32) -> u64;
    fn mtspr(spr: u32, value: u64);
    fn register_power_pmu(pmu: *mut power_pmu) -> i32;
    fn cpu_has_feature(feature: u32) -> bool;
    fn pr_info(fmt: *const u8, ...);
    static mut cur_cpu_spec: *mut cpu_spec;
}

// The event constants, attribute macros, and dependency types are defined by the surrounding kernel translation.

static EVENT_ALTERNATIVES: [[u32; MAX_ALT]; 11] = [
    [PM_MRK_ST_CMPL, PM_MRK_ST_CMPL_ALT],
    [PM_BR_MRK_2PATH, PM_BR_MRK_2PATH_ALT],
    [PM_L3_CO_MEPF, PM_L3_CO_MEPF_ALT],
    [PM_MRK_DATA_FROM_L2MISS, PM_MRK_DATA_FROM_L2MISS_ALT],
    [PM_CMPLU_STALL_ALT, PM_CMPLU_STALL],
    [PM_BR_2PATH, PM_BR_2PATH_ALT],
    [PM_INST_DISP, PM_INST_DISP_ALT],
    [PM_RUN_CYC_ALT, PM_RUN_CYC],
    [PM_MRK_FILT_MATCH, PM_MRK_FILT_MATCH_ALT],
    [PM_LD_MISS_L1, PM_LD_MISS_L1_ALT],
    [PM_RUN_INST_CMPL_ALT, PM_RUN_INST_CMPL],
];

unsafe fn power8_get_alternatives(event: u64, flags: u32, alt: *mut u64) -> i32 {
    isa207_get_alternatives(event, alt, EVENT_ALTERNATIVES.len(), flags,
                            &EVENT_ALTERNATIVES)
}

// GENERIC_EVENT_ATTR and CACHE_EVENT_ATTR declarations from the C source are represented by
// the corresponding generated attributes in the dependency translation.

static mut POWER8_EVENTS_ATTR: [*mut attribute; 26] = [
    GENERIC_EVENT_PTR!(PM_CYC), GENERIC_EVENT_PTR!(PM_GCT_NOSLOT_CYC),
    GENERIC_EVENT_PTR!(PM_CMPLU_STALL), GENERIC_EVENT_PTR!(PM_INST_CMPL),
    GENERIC_EVENT_PTR!(PM_BRU_FIN), GENERIC_EVENT_PTR!(PM_BR_MPRED_CMPL),
    GENERIC_EVENT_PTR!(PM_LD_REF_L1), GENERIC_EVENT_PTR!(PM_LD_MISS_L1),
    GENERIC_EVENT_PTR!(MEM_ACCESS), CACHE_EVENT_PTR!(PM_LD_MISS_L1),
    CACHE_EVENT_PTR!(PM_LD_REF_L1), CACHE_EVENT_PTR!(PM_L1_PREF),
    CACHE_EVENT_PTR!(PM_ST_MISS_L1), CACHE_EVENT_PTR!(PM_L1_ICACHE_MISS),
    CACHE_EVENT_PTR!(PM_INST_FROM_L1), CACHE_EVENT_PTR!(PM_IC_PREF_WRITE),
    CACHE_EVENT_PTR!(PM_DATA_FROM_L3MISS), CACHE_EVENT_PTR!(PM_DATA_FROM_L3),
    CACHE_EVENT_PTR!(PM_L3_PREF_ALL), CACHE_EVENT_PTR!(PM_L2_ST_MISS),
    CACHE_EVENT_PTR!(PM_L2_ST), CACHE_EVENT_PTR!(PM_BR_MPRED_CMPL),
    CACHE_EVENT_PTR!(PM_BRU_FIN), CACHE_EVENT_PTR!(PM_DTLB_MISS),
    CACHE_EVENT_PTR!(PM_ITLB_MISS), core::ptr::null_mut(),
];

static POWER8_PMU_EVENTS_GROUP: attribute_group = attribute_group {
    name: "events", attrs: unsafe { POWER8_EVENTS_ATTR.as_mut_ptr() },
};
static mut POWER8_PMU_CAPS_ATTRS: [*mut attribute; 1] = [core::ptr::null_mut()];
static POWER8_PMU_CAPS_GROUP: attribute_group = attribute_group {
    name: "caps", attrs: unsafe { POWER8_PMU_CAPS_ATTRS.as_mut_ptr() },
};
static POWER8_PMU_ATTR_GROUPS: [*const attribute_group; 4] = [
    &isa207_pmu_format_group, &POWER8_PMU_EVENTS_GROUP, &POWER8_PMU_CAPS_GROUP, core::ptr::null(),
];

static mut POWER8_GENERIC_EVENTS: [i32; 8] = [
    PM_CYC, PM_GCT_NOSLOT_CYC, PM_CMPLU_STALL, PM_INST_CMPL,
    PM_BRU_FIN, PM_BR_MPRED_CMPL, PM_LD_REF_L1, PM_LD_MISS_L1,
];

unsafe fn power8_bhrb_filter_map(branch_sample_type: u64) -> u64 {
    let mut pmu_bhrb_filter = 0;
    if branch_sample_type & PERF_SAMPLE_BRANCH_ANY != 0 { return pmu_bhrb_filter; }
    if branch_sample_type & PERF_SAMPLE_BRANCH_ANY_RETURN != 0 { return u64::MAX; }
    if branch_sample_type & PERF_SAMPLE_BRANCH_IND_CALL != 0 { return u64::MAX; }
    if branch_sample_type & PERF_SAMPLE_BRANCH_CALL != 0 { return u64::MAX; }
    if branch_sample_type & PERF_SAMPLE_BRANCH_ANY_CALL != 0 {
        pmu_bhrb_filter |= POWER8_MMCRA_IFM1;
        return pmu_bhrb_filter;
    }
    u64::MAX
}

unsafe fn power8_config_bhrb(mut pmu_bhrb_filter: u64) {
    pmu_bhrb_filter &= POWER8_MMCRA_BHRB_MASK;
    mtspr(SPRN_MMCRA, mfspr(SPRN_MMCRA) | pmu_bhrb_filter);
}

/* Table of generalized cache-related events.  0 means unsupported, -1 nonsensical. */
static mut POWER8_CACHE_EVENTS: [[[u64; 2]; 3]; 7] = [
    [[PM_LD_REF_L1, PM_LD_MISS_L1], [0, PM_ST_MISS_L1], [PM_L1_PREF, 0]],
    [[PM_INST_FROM_L1, PM_L1_ICACHE_MISS], [PM_L1_DEMAND_WRITE, u64::MAX], [PM_IC_PREF_WRITE, 0]],
    [[PM_DATA_FROM_L3, PM_DATA_FROM_L3MISS], [PM_L2_ST, PM_L2_ST_MISS], [PM_L3_PREF_ALL, 0]],
    [[0, PM_DTLB_MISS], [u64::MAX, u64::MAX], [u64::MAX, u64::MAX]],
    [[0, PM_ITLB_MISS], [u64::MAX, u64::MAX], [u64::MAX, u64::MAX]],
    [[PM_BRU_FIN, PM_BR_MPRED_CMPL], [u64::MAX, u64::MAX], [u64::MAX, u64::MAX]],
    [[u64::MAX, u64::MAX], [u64::MAX, u64::MAX], [u64::MAX, u64::MAX]],
];

static mut POWER8_PMU: power_pmu = power_pmu {
    name: "POWER8", n_counter: MAX_PMU_COUNTERS, max_alternatives: MAX_ALT + 1,
    add_fields: ISA207_ADD_FIELDS, test_adder: ISA207_TEST_ADDER,
    compute_mmcr: isa207_compute_mmcr, config_bhrb: power8_config_bhrb,
    bhrb_filter_map: power8_bhrb_filter_map, get_constraint: isa207_get_constraint,
    get_alternatives: power8_get_alternatives, get_mem_data_src: isa207_get_mem_data_src,
    get_mem_weight: isa207_get_mem_weight, disable_pmc: isa207_disable_pmc,
    flags: PPMU_HAS_SIER | PPMU_ARCH_207S, n_generic: POWER8_GENERIC_EVENTS.len(),
    generic_events: POWER8_GENERIC_EVENTS.as_mut_ptr(), cache_events: &mut POWER8_CACHE_EVENTS,
    attr_groups: POWER8_PMU_ATTR_GROUPS.as_ptr(), bhrb_nr: 32,
};

pub unsafe fn init_power8_pmu() -> i32 {
    let pvr = mfspr(SPRN_PVR);
    if PVR_VER(pvr) != PVR_POWER8E && PVR_VER(pvr) != PVR_POWER8NVL && PVR_VER(pvr) != PVR_POWER8 { return -ENODEV; }
    let rc = register_power_pmu(&mut POWER8_PMU);
    if rc != 0 { return rc; }
    (*cur_cpu_spec).cpu_user_features2 |= PPC_FEATURE2_EBB;
    if cpu_has_feature(CPU_FTR_PMAO_BUG) { pr_info(b"PMAO restore workaround active.\0".as_ptr()); }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
