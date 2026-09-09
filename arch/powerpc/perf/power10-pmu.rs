// SPDX-License-Identifier: GPL-2.0-or-later
/* Performance counter support for POWER10 processors. */

// Dependencies supplied by the surrounding kernel translation unit:
// isa207-common.h, power10-events-list.h, and the GENERIC_EVENT_ATTR,
// CACHE_EVENT_ATTR, PMU_FORMAT_ATTR, and *_PTR declarations.

const POWER10_MMCRA_IFM1: u64 = 0x0000000040000000;
const POWER10_MMCRA_IFM2: u64 = 0x0000000080000000;
const POWER10_MMCRA_IFM3: u64 = 0x00000000c0000000;
const POWER10_MMCRA_BHRB_MASK: u64 = 0x00000000c0000000;

extern "C" {
    static mut PERF_REG_EXTENDED_MASK: u64;
}

static POWER10_EVENT_ALTERNATIVES: [[u32; MAX_ALT]; 2] = [
    [PM_INST_CMPL_ALT, PM_INST_CMPL],
    [PM_CYC_ALT, PM_CYC],
];

unsafe fn power10_get_alternatives(event: u64, flags: u32, alt: *mut u64) -> i32 {
    isa207_get_alternatives(event, alt, POWER10_EVENT_ALTERNATIVES.len(), flags,
                            POWER10_EVENT_ALTERNATIVES.as_ptr())
}

unsafe fn power10_check_attr_config(ev: *mut perf_event) -> i32 {
    let event = (*ev).attr.config;
    let val = (event >> EVENT_SAMPLE_SHIFT) & EVENT_SAMPLE_MASK;
    if val == 0x10 || isa3XX_check_attr_config(ev) != 0 { -EINVAL } else { 0 }
}

// GENERIC_EVENT_ATTR and CACHE_EVENT_ATTR expand to kernel attribute objects.
// The declarations are retained here as their direct Rust-side symbol names.
GENERIC_EVENT_ATTR!(cpu_cycles, PM_CYC);
GENERIC_EVENT_ATTR!(instructions, PM_INST_CMPL);
GENERIC_EVENT_ATTR!(branch_instructions, PM_BR_CMPL);
GENERIC_EVENT_ATTR!(branch_misses, PM_BR_MPRED_CMPL);
GENERIC_EVENT_ATTR!(cache_references, PM_LD_REF_L1);
GENERIC_EVENT_ATTR!(cache_misses, PM_LD_MISS_L1);
GENERIC_EVENT_ATTR!(mem_loads, MEM_LOADS);
GENERIC_EVENT_ATTR!(mem_stores, MEM_STORES);
GENERIC_EVENT_ATTR!(branch_instructions_fin, PM_BR_FIN);
GENERIC_EVENT_ATTR!(branch_misses_fin, PM_MPRED_BR_FIN);
GENERIC_EVENT_ATTR!(cache_misses_fin, PM_LD_DEMAND_MISS_L1_FIN);

// The following tables preserve the C event mapping; attribute construction is
// supplied by the kernel compatibility layer.
static mut POWER10_GENERIC_EVENTS_DD1: [i32; 6] = [
    PM_CYC, PM_INST_CMPL, PM_BR_CMPL, PM_BR_MPRED_CMPL, PM_LD_REF_L1,
    PM_LD_MISS_L1,
];
static mut POWER10_GENERIC_EVENTS: [i32; 6] = [
    PM_CYC, PM_INST_CMPL, PM_BR_FIN, PM_MPRED_BR_FIN, PM_LD_REF_L1,
    PM_LD_DEMAND_MISS_L1_FIN,
];

unsafe fn power10_bhrb_filter_map(branch_sample_type: u64) -> u64 {
    let mut filter = 0;
    if branch_sample_type & PERF_SAMPLE_BRANCH_ANY != 0 { return filter; }
    if branch_sample_type & PERF_SAMPLE_BRANCH_ANY_RETURN != 0 { return u64::MAX; }
    if branch_sample_type & PERF_SAMPLE_BRANCH_IND_CALL != 0 { filter |= POWER10_MMCRA_IFM2; return filter; }
    if branch_sample_type & PERF_SAMPLE_BRANCH_COND != 0 { filter |= POWER10_MMCRA_IFM3; return filter; }
    if branch_sample_type & PERF_SAMPLE_BRANCH_CALL != 0 { return u64::MAX; }
    if branch_sample_type & PERF_SAMPLE_BRANCH_ANY_CALL != 0 { filter |= POWER10_MMCRA_IFM1; return filter; }
    u64::MAX
}

unsafe fn power10_config_bhrb(mut pmu_bhrb_filter: u64) {
    pmu_bhrb_filter &= POWER10_MMCRA_BHRB_MASK;
    mtspr(SPRN_MMCRA, mfspr(SPRN_MMCRA) | pmu_bhrb_filter);
}

// Cache-event tables use the C layout [MAX][OP_MAX][RESULT_MAX].
static mut POWER10_CACHE_EVENTS_DD1: [[[u64; RESULT_MAX]; OP_MAX]; MAX] =
    power10_cache_events_dd1_init();
static mut POWER10_CACHE_EVENTS: [[[u64; RESULT_MAX]; OP_MAX]; MAX] =
    power10_cache_events_init();

const fn power10_cache_events_dd1_init() -> [[[u64; RESULT_MAX]; OP_MAX]; MAX] {
    let mut a = [[[0; RESULT_MAX]; OP_MAX]; MAX];
    a[L1D][OP_READ][RESULT_ACCESS] = PM_LD_REF_L1; a[L1D][OP_READ][RESULT_MISS] = PM_LD_MISS_L1;
    a[L1D][OP_WRITE][RESULT_MISS] = PM_ST_MISS_L1; a[L1D][OP_PREFETCH][RESULT_ACCESS] = PM_LD_PREFETCH_CACHE_LINE_MISS;
    a[L1I][OP_READ][RESULT_ACCESS] = PM_INST_FROM_L1; a[L1I][OP_READ][RESULT_MISS] = PM_L1_ICACHE_MISS;
    a[L1I][OP_WRITE][RESULT_ACCESS] = PM_INST_FROM_L1MISS; a[L1I][OP_WRITE][RESULT_MISS] = u64::MAX;
    a[L1I][OP_PREFETCH][RESULT_ACCESS] = PM_IC_PREF_REQ;
    a[LL][OP_READ][RESULT_ACCESS] = PM_DATA_FROM_L3; a[LL][OP_READ][RESULT_MISS] = PM_DATA_FROM_L3MISS;
    a[LL][OP_WRITE][RESULT_ACCESS] = u64::MAX; a[LL][OP_WRITE][RESULT_MISS] = u64::MAX;
    a[LL][OP_PREFETCH][RESULT_ACCESS] = u64::MAX;
    a[DTLB][OP_READ][RESULT_MISS] = PM_DTLB_MISS; a[ITLB][OP_READ][RESULT_MISS] = PM_ITLB_MISS;
    a[BPU][OP_READ][RESULT_ACCESS] = PM_BR_CMPL; a[BPU][OP_READ][RESULT_MISS] = PM_BR_MPRED_CMPL;
    a
}

const fn power10_cache_events_init() -> [[[u64; RESULT_MAX]; OP_MAX]; MAX] {
    let mut a = power10_cache_events_dd1_init();
    a[LL][OP_WRITE][RESULT_ACCESS] = PM_L2_ST; a[LL][OP_WRITE][RESULT_MISS] = PM_L2_ST_MISS;
    a[LL][OP_PREFETCH][RESULT_ACCESS] = PM_L3_PF_MISS_L3;
    a
}

unsafe fn power10_compute_mmcr(event: *mut u64, n_ev: i32, hwc: *mut u32,
                               mmcr: *mut mmcr_regs, pevents: *mut *mut perf_event,
                               flags: u32) -> i32 {
    let ret = isa207_compute_mmcr(event, n_ev, hwc, mmcr, pevents, flags);
    if ret == 0 { (*mmcr).mmcr0 |= MMCR0_C56RUN; }
    ret
}

static mut POWER10_PMU: power_pmu = power_pmu {
    name: "POWER10", n_counter: MAX_PMU_COUNTERS, add_fields: ISA207_ADD_FIELDS,
    test_adder: ISA207_TEST_ADDER, group_constraint_mask: CNST_CACHE_PMC4_MASK,
    group_constraint_val: CNST_CACHE_PMC4_VAL, compute_mmcr: Some(power10_compute_mmcr),
    config_bhrb: Some(power10_config_bhrb), bhrb_filter_map: Some(power10_bhrb_filter_map),
    get_constraint: Some(isa207_get_constraint), get_alternatives: Some(power10_get_alternatives),
    get_mem_data_src: Some(isa207_get_mem_data_src), get_mem_weight: Some(isa207_get_mem_weight),
    disable_pmc: Some(isa207_disable_pmc), flags: PPMU_HAS_SIER | PPMU_ARCH_207S |
        PPMU_ARCH_31 | PPMU_HAS_ATTR_CONFIG1 | PPMU_P10,
    n_generic: POWER10_GENERIC_EVENTS.len(), generic_events: POWER10_GENERIC_EVENTS.as_mut_ptr(),
    cache_events: POWER10_CACHE_EVENTS.as_mut_ptr(), attr_groups: power10_pmu_attr_groups,
    bhrb_nr: 32, capabilities: PERF_PMU_CAP_EXTENDED_REGS, check_attr_config: Some(power10_check_attr_config),
};

pub unsafe fn init_power10_pmu() -> i32 {
    let pvr = mfspr(SPRN_PVR);
    if PVR_VER(pvr) != PVR_POWER10 { return -ENODEV; }
    if PVR_CFG(pvr) == 1 { POWER10_PMU.flags |= PPMU_P10_DD1; }
    PERF_REG_EXTENDED_MASK = PERF_REG_PMU_MASK_31;
    if PVR_CFG(pvr) == 1 {
        POWER10_PMU.generic_events = POWER10_GENERIC_EVENTS_DD1.as_mut_ptr();
        POWER10_PMU.cache_events = POWER10_CACHE_EVENTS_DD1.as_mut_ptr();
    }
    let rc = register_power_pmu(&mut POWER10_PMU);
    if rc != 0 { return rc; }
    (*cur_cpu_spec).cpu_user_features2 |= PPC_FEATURE2_EBB;
    0
}

static mut POWER11_PMU: power_pmu = power_pmu::zeroed();

pub unsafe fn init_power11_pmu() -> i32 {
    let pvr = mfspr(SPRN_PVR);
    if PVR_VER(pvr) != PVR_POWER11 { return -ENODEV; }
    PERF_REG_EXTENDED_MASK = PERF_REG_PMU_MASK_31;
    POWER11_PMU = POWER10_PMU;
    POWER11_PMU.name = "Power11";
    let rc = register_power_pmu(&mut POWER11_PMU);
    if rc != 0 { return rc; }
    (*cur_cpu_spec).cpu_user_features2 |= PPC_FEATURE2_EBB;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
