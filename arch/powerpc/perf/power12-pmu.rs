// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Performance counter support for Power12 processors.
 *
 * Copyright 2026 Athira Rajeev, IBM Corporation.
 */

// Dependency declarations and build-time event definitions are supplied by
// isa207-common.h, power12-events-list.h, and the kernel PMU interfaces.

pub const POWER12_MMCRA_IFM1: u64 = 0x0000000040000000;
pub const POWER12_MMCRA_IFM2: u64 = 0x0000000080000000;
pub const POWER12_MMCRA_IFM3: u64 = 0x00000000c0000000;
pub const POWER12_MMCRA_BHRB_MASK: u64 = 0x00000000c0000000;

extern "C" {
    static mut PERF_REG_EXTENDED_MASK: u64;
}

static POWER12_EVENT_ALTERNATIVES: [[u32; MAX_ALT]; 2] = [
    [PM_INST_CMPL_ALT, PM_INST_CMPL],
    [PM_CYC_ALT, PM_CYC],
];

unsafe fn power12_get_alternatives(event: u64, flags: u32, alt: *mut u64) -> i32 {
    isa207_get_alternatives(
        event,
        alt,
        POWER12_EVENT_ALTERNATIVES.len(),
        flags,
        POWER12_EVENT_ALTERNATIVES.as_ptr(),
    )
}

unsafe fn power12_check_attr_config(ev: *mut perf_event) -> i32 {
    let event = (*ev).attr.config;
    let val = (event >> EVENT_SAMPLE_SHIFT) & EVENT_SAMPLE_MASK;
    if val == 0x10 || isa3XX_check_attr_config(ev) != 0 {
        return -EINVAL;
    }
    0
}

// GENERIC_EVENT_ATTR and CACHE_EVENT_ATTR declarations from the C source.
// Their kernel attribute objects are represented by the corresponding
// dependency-provided declarations.

static mut POWER12_EVENTS_ATTR: [*mut attribute; 25] = [
    GENERIC_EVENT_PTR(PM_CYC), GENERIC_EVENT_PTR(PM_INST_CMPL),
    GENERIC_EVENT_PTR(PM_BR_FIN), GENERIC_EVENT_PTR(PM_MPRED_BR_FIN),
    GENERIC_EVENT_PTR(PM_LD_REF_L1), GENERIC_EVENT_PTR(PM_LD_DEMAND_MISS_L1_FIN),
    GENERIC_EVENT_PTR(MEM_LOADS), GENERIC_EVENT_PTR(MEM_STORES),
    CACHE_EVENT_PTR(PM_LD_MISS_L1), CACHE_EVENT_PTR(PM_LD_REF_L1),
    CACHE_EVENT_PTR(PM_LD_PREFETCH_CACHE_LINE_MISS), CACHE_EVENT_PTR(PM_ST_MISS_L1),
    CACHE_EVENT_PTR(PM_L1_ICACHE_MISS), CACHE_EVENT_PTR(PM_INST_FROM_L1),
    CACHE_EVENT_PTR(PM_IC_PREF_REQ), CACHE_EVENT_PTR(PM_DATA_FROM_L3MISS),
    CACHE_EVENT_PTR(PM_DATA_FROM_L3), CACHE_EVENT_PTR(PM_L3_PF_MISS_L3),
    CACHE_EVENT_PTR(PM_L2_ST_MISS), CACHE_EVENT_PTR(PM_L2_ST),
    CACHE_EVENT_PTR(PM_BR_MPRED_CMPL), CACHE_EVENT_PTR(PM_BR_CMPL),
    CACHE_EVENT_PTR(PM_DTLB_MISS), CACHE_EVENT_PTR(PM_ITLB_MISS),
    core::ptr::null_mut(),
];

static POWER12_PMU_EVENTS_GROUP: attribute_group = attribute_group {
    name: c"events".as_ptr(), attrs: unsafe { POWER12_EVENTS_ATTR.as_mut_ptr() },
};

// PMU_FORMAT_ATTR declarations and format_attr_* objects are supplied by the
// kernel PMU attribute layer.
static mut POWER12_PMU_FORMAT_ATTR: [*mut attribute; 20] = [
    &format_attr_event.attr, &format_attr_pmcxsel.attr, &format_attr_mark.attr,
    &format_attr_combine.attr, &format_attr_unit.attr, &format_attr_pmc.attr,
    &format_attr_cache_sel.attr, &format_attr_sdar_mode.attr,
    &format_attr_sample_mode.attr, &format_attr_thresh_sel.attr,
    &format_attr_thresh_stop.attr, &format_attr_thresh_start.attr,
    &format_attr_l2l3_sel.attr, &format_attr_src_sel.attr,
    &format_attr_invert_bit.attr, &format_attr_src_mask.attr,
    &format_attr_src_match.attr, &format_attr_radix_scope.attr,
    &format_attr_thresh_cmp.attr, core::ptr::null_mut(),
];

static POWER12_PMU_FORMAT_GROUP: attribute_group = attribute_group {
    name: c"format".as_ptr(), attrs: unsafe { POWER12_PMU_FORMAT_ATTR.as_mut_ptr() },
};
static POWER12_PMU_ATTR_GROUPS: [*const attribute_group; 3] = [
    &POWER12_PMU_FORMAT_GROUP, &POWER12_PMU_EVENTS_GROUP, core::ptr::null(),
];

static mut POWER12_GENERIC_EVENTS: [i32; 6] = [
    PM_CYC, PM_INST_CMPL, PM_BR_FIN, PM_MPRED_BR_FIN, PM_LD_REF_L1,
    PM_LD_DEMAND_MISS_L1_FIN,
];

unsafe fn power12_bhrb_filter_map(branch_sample_type: u64) -> u64 {
    let mut filter = 0;
    if branch_sample_type & PERF_SAMPLE_BRANCH_ANY != 0 { return filter; }
    if branch_sample_type & PERF_SAMPLE_BRANCH_ANY_RETURN != 0 { return u64::MAX; }
    if branch_sample_type & PERF_SAMPLE_BRANCH_IND_CALL != 0 { return POWER12_MMCRA_IFM2; }
    if branch_sample_type & PERF_SAMPLE_BRANCH_COND != 0 { return POWER12_MMCRA_IFM3; }
    if branch_sample_type & PERF_SAMPLE_BRANCH_CALL != 0 { return u64::MAX; }
    if branch_sample_type & PERF_SAMPLE_BRANCH_ANY_CALL != 0 { filter |= POWER12_MMCRA_IFM1; return filter; }
    u64::MAX
}

unsafe fn power12_config_bhrb(mut filter: u64) {
    filter &= POWER12_MMCRA_BHRB_MASK;
    mtspr(SPRN_MMCRA, mfspr(SPRN_MMCRA) | filter);
}

static mut POWER12_CACHE_EVENTS: [[[u64; 2]; 3]; 7] = [
    [[PM_LD_REF_L1, PM_LD_MISS_L1], [0, PM_ST_MISS_L1], [PM_LD_PREFETCH_CACHE_LINE_MISS, 0]],
    [[PM_INST_FROM_L1, PM_L1_ICACHE_MISS], [PM_INST_FROM_L1MISS, u64::MAX], [PM_IC_PREF_REQ, 0]],
    [[PM_DATA_FROM_L3, PM_DATA_FROM_L3MISS], [PM_L2_ST, PM_L2_ST_MISS], [PM_L3_PF_MISS_L3, 0]],
    [[0, PM_DTLB_MISS], [u64::MAX, u64::MAX], [u64::MAX, u64::MAX]],
    [[0, PM_ITLB_MISS], [u64::MAX, u64::MAX], [u64::MAX, u64::MAX]],
    [[PM_BR_CMPL, PM_BR_MPRED_CMPL], [u64::MAX, u64::MAX], [u64::MAX, u64::MAX]],
    [[u64::MAX, u64::MAX], [u64::MAX, u64::MAX], [u64::MAX, u64::MAX]],
];

unsafe fn power12_compute_mmcr(event: *mut u64, n_ev: i32, hwc: *mut u32, mmcr: *mut mmcr_regs, pevents: *mut *mut perf_event, flags: u32) -> i32 {
    let ret = isa207_compute_mmcr(event, n_ev, hwc, mmcr, pevents, flags);
    if ret == 0 { (*mmcr).mmcr0 |= MMCR0_C56RUN; }
    ret
}

static mut POWER12_PMU: power_pmu = power_pmu {
    name: c"Power12".as_ptr(), n_counter: MAX_PMU_COUNTERS, add_fields: ISA207_ADD_FIELDS,
    test_adder: ISA207_TEST_ADDER, group_constraint_mask: CNST_CACHE_PMC4_MASK,
    group_constraint_val: CNST_CACHE_PMC4_VAL, compute_mmcr: Some(power12_compute_mmcr),
    config_bhrb: Some(power12_config_bhrb), bhrb_filter_map: Some(power12_bhrb_filter_map),
    get_constraint: Some(isa207_get_constraint), get_alternatives: Some(power12_get_alternatives),
    get_mem_data_src: Some(isa207_get_mem_data_src), get_mem_weight: Some(isa207_get_mem_weight),
    disable_pmc: Some(isa207_disable_pmc), flags: PPMU_HAS_SIER | PPMU_ARCH_207S | PPMU_ARCH_31 | PPMU_HAS_ATTR_CONFIG1 | PPMU_P10,
    n_generic: POWER12_GENERIC_EVENTS.len(), generic_events: POWER12_GENERIC_EVENTS.as_mut_ptr(),
    cache_events: POWER12_CACHE_EVENTS.as_mut_ptr(), attr_groups: POWER12_PMU_ATTR_GROUPS.as_ptr(),
    bhrb_nr: 32, capabilities: PERF_PMU_CAP_EXTENDED_REGS, check_attr_config: Some(power12_check_attr_config),
};

pub unsafe fn init_power12_pmu() -> i32 {
    let pvr = mfspr(SPRN_PVR);
    if PVR_VER(pvr) != PVR_POWER12 { return -ENODEV; }
    PERF_REG_EXTENDED_MASK = PERF_REG_PMU_MASK_31;
    let rc = register_power_pmu(&mut POWER12_PMU);
    if rc != 0 { return rc; }
    (*cur_cpu_spec).cpu_user_features2 |= PPC_FEATURE2_EBB;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
