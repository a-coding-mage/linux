// SPDX-License-Identifier: GPL-2.0

// C dependencies supplied by the surrounding kernel translation unit.

/*
 * Not sure about some of these
 */
static P6_PERFMON_EVENT_MAP: [u64; PERF_COUNT_HW_MAX] = {
    let mut map = [0u64; PERF_COUNT_HW_MAX];
    map[PERF_COUNT_HW_CPU_CYCLES] = 0x0079; // CPU_CLK_UNHALTED
    map[PERF_COUNT_HW_INSTRUCTIONS] = 0x00c0; // INST_RETIRED
    map[PERF_COUNT_HW_CACHE_REFERENCES] = 0x0f2e; // L2_RQSTS:M:E:S:I
    map[PERF_COUNT_HW_CACHE_MISSES] = 0x012e; // L2_RQSTS:I
    map[PERF_COUNT_HW_BRANCH_INSTRUCTIONS] = 0x00c4; // BR_INST_RETIRED
    map[PERF_COUNT_HW_BRANCH_MISSES] = 0x00c5; // BR_MISS_PRED_RETIRED
    map[PERF_COUNT_HW_BUS_CYCLES] = 0x0062; // BUS_DRDY_CLOCKS
    map[PERF_COUNT_HW_STALLED_CYCLES_FRONTEND] = 0x00a2; // RESOURCE_STALLS
    map
};

static P6_HW_CACHE_EVENT_IDS: [[[u64; PERF_COUNT_HW_CACHE_RESULT_MAX]; PERF_COUNT_HW_CACHE_OP_MAX]; PERF_COUNT_HW_CACHE_MAX] = {
    let mut ids = [[[0u64; PERF_COUNT_HW_CACHE_RESULT_MAX]; PERF_COUNT_HW_CACHE_OP_MAX]; PERF_COUNT_HW_CACHE_MAX];
    ids[C_L1D][C_OP_READ][C_RESULT_ACCESS] = 0x0043; // DATA_MEM_REFS
    ids[C_L1D][C_OP_READ][C_RESULT_MISS] = 0x0045; // DCU_LINES_IN
    ids[C_L1D][C_OP_WRITE][C_RESULT_ACCESS] = 0;
    ids[C_L1D][C_OP_WRITE][C_RESULT_MISS] = 0x0f29; // L2_LD:M:E:S:I
    ids[C_L1D][C_OP_PREFETCH][C_RESULT_ACCESS] = 0;
    ids[C_L1D][C_OP_PREFETCH][C_RESULT_MISS] = 0;
    ids[C_L1I][C_OP_READ][C_RESULT_ACCESS] = 0x0080; // IFU_IFETCH
    ids[C_L1I][C_OP_READ][C_RESULT_MISS] = 0x0f28; // L2_IFETCH:M:E:S:I
    ids[C_L1I][C_OP_WRITE][C_RESULT_ACCESS] = u64::MAX;
    ids[C_L1I][C_OP_WRITE][C_RESULT_MISS] = u64::MAX;
    ids[C_L1I][C_OP_PREFETCH][C_RESULT_ACCESS] = 0;
    ids[C_L1I][C_OP_PREFETCH][C_RESULT_MISS] = 0;
    ids[C_LL][C_OP_WRITE][C_RESULT_MISS] = 0x0025; // L2_M_LINES_INM
    ids[C_DTLB][C_OP_READ][C_RESULT_ACCESS] = 0x0043; // DATA_MEM_REFS
    ids[C_ITLB][C_OP_READ][C_RESULT_ACCESS] = 0x0080; // IFU_IFETCH
    ids[C_ITLB][C_OP_READ][C_RESULT_MISS] = 0x0085; // ITLB_MISS
    ids[C_ITLB][C_OP_WRITE][C_RESULT_ACCESS] = u64::MAX;
    ids[C_ITLB][C_OP_WRITE][C_RESULT_MISS] = u64::MAX;
    ids[C_ITLB][C_OP_PREFETCH][C_RESULT_ACCESS] = u64::MAX;
    ids[C_ITLB][C_OP_PREFETCH][C_RESULT_MISS] = u64::MAX;
    ids[C_BPU][C_OP_READ][C_RESULT_ACCESS] = 0x00c4; // BR_INST_RETIRED
    ids[C_BPU][C_OP_READ][C_RESULT_MISS] = 0x00c5; // BR_MISS_PRED_RETIRED
    ids[C_BPU][C_OP_WRITE][C_RESULT_ACCESS] = u64::MAX;
    ids[C_BPU][C_OP_WRITE][C_RESULT_MISS] = u64::MAX;
    ids[C_BPU][C_OP_PREFETCH][C_RESULT_ACCESS] = u64::MAX;
    ids[C_BPU][C_OP_PREFETCH][C_RESULT_MISS] = u64::MAX;
    ids
};

unsafe fn p6_pmu_event_map(hw_event: i32) -> u64 {
    P6_PERFMON_EVENT_MAP[hw_event as usize]
}

// Event setting that is specified not to count anything.
// We use this to effectively disable a counter.
// L2_RQSTS with 0 MESI unit mask.
const P6_NOP_EVENT: u64 = 0x0000002E;

static mut P6_EVENT_CONSTRAINTS: [event_constraint; 7] = [
    INTEL_EVENT_CONSTRAINT!(0xc1, 0x1), // FLOPS
    INTEL_EVENT_CONSTRAINT!(0x10, 0x1), // FP_COMP_OPS_EXE
    INTEL_EVENT_CONSTRAINT!(0x11, 0x2), // FP_ASSIST
    INTEL_EVENT_CONSTRAINT!(0x12, 0x2), // MUL
    INTEL_EVENT_CONSTRAINT!(0x13, 0x2), // DIV
    INTEL_EVENT_CONSTRAINT!(0x14, 0x1), // CYCLES_DIV_BUSY
    EVENT_CONSTRAINT_END,
];

unsafe fn p6_pmu_disable_all() {
    let mut val: u64;
    rdmsrq!(MSR_P6_EVNTSEL0, val);
    val &= !ARCH_PERFMON_EVENTSEL_ENABLE;
    wrmsrq!(MSR_P6_EVNTSEL0, val);
}

unsafe fn p6_pmu_enable_all(_added: i32) {
    let mut val: usize;
    rdmsrq!(MSR_P6_EVNTSEL0, val);
    val |= ARCH_PERFMON_EVENTSEL_ENABLE as usize;
    wrmsrq!(MSR_P6_EVNTSEL0, val);
}

unsafe fn p6_pmu_disable_event(event: *mut perf_event) {
    let hwc = &mut (*event).hw;
    let val = P6_NOP_EVENT;
    let _ = wrmsrq_safe(hwc.config_base, val);
}

unsafe fn p6_pmu_enable_event(event: *mut perf_event) {
    let hwc = &mut (*event).hw;
    let val = hwc.config;
    // p6 only has a global event enable, set on PerfEvtSel0.
    // We "disable" events by programming P6_NOP_EVENT and rely on
    // p6_pmu_enable_all() being called to actually enable the events.
    let _ = wrmsrq_safe(hwc.config_base, val);
}

PMU_FORMAT_ATTR!(event, "config:0-7");
PMU_FORMAT_ATTR!(umask, "config:8-15");
PMU_FORMAT_ATTR!(edge, "config:18");
PMU_FORMAT_ATTR!(pc, "config:19");
PMU_FORMAT_ATTR!(inv, "config:23");
PMU_FORMAT_ATTR!(cmask, "config:24-31");

static mut INTEL_P6_FORMATS_ATTR: [*mut attribute; 7] = [
    &raw mut format_attr_event.attr,
    &raw mut format_attr_umask.attr,
    &raw mut format_attr_edge.attr,
    &raw mut format_attr_pc.attr,
    &raw mut format_attr_inv.attr,
    &raw mut format_attr_cmask.attr,
    core::ptr::null_mut(),
];

static P6_PMU: x86_pmu = x86_pmu {
    name: "p6",
    handle_irq: x86_pmu_handle_irq,
    disable_all: p6_pmu_disable_all,
    enable_all: p6_pmu_enable_all,
    enable: p6_pmu_enable_event,
    disable: p6_pmu_disable_event,
    hw_config: x86_pmu_hw_config,
    schedule_events: x86_schedule_events,
    eventsel: MSR_P6_EVNTSEL0,
    perfctr: MSR_P6_PERFCTR0,
    event_map: p6_pmu_event_map,
    max_events: P6_PERFMON_EVENT_MAP.len(),
    apic: 1,
    max_period: (1u64 << 31).wrapping_sub(1),
    version: 0,
    cntr_mask64: 0x3,
    // Events have 40 bits implemented, but bits [32-39] are sign
    // extensions of bit 31. The effective width is therefore 32 bits.
    cntval_bits: 32,
    cntval_mask: (1u64 << 32).wrapping_sub(1),
    get_event_constraints: x86_get_event_constraints,
    event_constraints: unsafe { &raw mut P6_EVENT_CONSTRAINTS[0] },
    format_attrs: unsafe { &raw mut INTEL_P6_FORMATS_ATTR[0] },
    events_sysfs_show: intel_event_sysfs_show,
};

unsafe fn p6_pmu_rdpmc_quirk() {
    if boot_cpu_data.x86_stepping < 9 {
        // PPro erratum 26; fixed in stepping 9 and above.
        pr_warn!("Userspace RDPMC support disabled due to a CPU erratum\n");
        x86_pmu.attr_rdpmc_broken = 1;
        x86_pmu.attr_rdpmc = X86_USER_RDPMC_NEVER_ENABLE;
    }
}

unsafe fn p6_pmu_init() -> i32 {
    x86_pmu = P6_PMU;
    if boot_cpu_data.x86_vfm == INTEL_PENTIUM_PRO {
        x86_add_quirk(p6_pmu_rdpmc_quirk);
    }
    core::ptr::copy_nonoverlapping(
        P6_HW_CACHE_EVENT_IDS.as_ptr(),
        hw_cache_event_ids.as_mut_ptr(),
        P6_HW_CACHE_EVENT_IDS.len(),
    );
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
