// SPDX-License-Identifier: GPL-2.0
/* Driver for Intel Xeon Phi "Knights Corner" PMU */

// C dependencies: linux/perf_event.h, linux/types.h, asm/hardirq.h,
// asm/msr.h, and ../perf_event.h.

static KNC_PERFMON_EVENT_MAP: [u64; 6] = [
    0x002a, 0x0016, 0x0028, 0x0029, 0x0012, 0x002b,
];

// The C C(...) cache-index macro and kernel constants are supplied by the
// surrounding PMU implementation.
static KNC_HW_CACHE_EVENT_IDS: [[[u64; PERF_COUNT_HW_CACHE_RESULT_MAX];
    PERF_COUNT_HW_CACHE_OP_MAX]; PERF_COUNT_HW_CACHE_MAX] = {
    let mut a = [[[0u64; PERF_COUNT_HW_CACHE_RESULT_MAX];
        PERF_COUNT_HW_CACHE_OP_MAX]; PERF_COUNT_HW_CACHE_MAX];
    a[C_L1D][C_OP_READ][C_RESULT_ACCESS] = ARCH_PERFMON_EVENTSEL_INT;
    a[C_L1D][C_OP_READ][C_RESULT_MISS] = 0x0003;
    a[C_L1D][C_OP_WRITE][C_RESULT_ACCESS] = 0x0001;
    a[C_L1D][C_OP_WRITE][C_RESULT_MISS] = 0x0004;
    a[C_L1D][C_OP_PREFETCH][C_RESULT_ACCESS] = 0x0011;
    a[C_L1D][C_OP_PREFETCH][C_RESULT_MISS] = 0x001c;
    a[C_L1I][C_OP_READ][C_RESULT_ACCESS] = 0x000c;
    a[C_L1I][C_OP_READ][C_RESULT_MISS] = 0x000e;
    a[C_L1I][C_OP_WRITE][C_RESULT_ACCESS] = u64::MAX;
    a[C_L1I][C_OP_WRITE][C_RESULT_MISS] = u64::MAX;
    a[C_L1I][C_OP_PREFETCH][C_RESULT_ACCESS] = 0x0;
    a[C_L1I][C_OP_PREFETCH][C_RESULT_MISS] = 0x0;
    a[C_LL][C_OP_READ][C_RESULT_ACCESS] = 0;
    a[C_LL][C_OP_READ][C_RESULT_MISS] = 0x10cb;
    a[C_LL][C_OP_WRITE][C_RESULT_ACCESS] = 0x10cc;
    a[C_LL][C_OP_WRITE][C_RESULT_MISS] = 0;
    a[C_LL][C_OP_PREFETCH][C_RESULT_ACCESS] = 0x10fc;
    a[C_LL][C_OP_PREFETCH][C_RESULT_MISS] = 0x10fe;
    a[C_DTLB][C_OP_READ][C_RESULT_ACCESS] = ARCH_PERFMON_EVENTSEL_INT;
    a[C_DTLB][C_OP_READ][C_RESULT_MISS] = 0x0002;
    a[C_DTLB][C_OP_WRITE][C_RESULT_ACCESS] = 0x0001;
    a[C_DTLB][C_OP_WRITE][C_RESULT_MISS] = 0x0002;
    a[C_DTLB][C_OP_PREFETCH][C_RESULT_ACCESS] = 0x0;
    a[C_DTLB][C_OP_PREFETCH][C_RESULT_MISS] = 0x0;
    a[C_ITLB][C_OP_READ][C_RESULT_ACCESS] = 0x000c;
    a[C_ITLB][C_OP_READ][C_RESULT_MISS] = 0x000d;
    a[C_ITLB][C_OP_WRITE][C_RESULT_ACCESS] = u64::MAX;
    a[C_ITLB][C_OP_WRITE][C_RESULT_MISS] = u64::MAX;
    a[C_ITLB][C_OP_PREFETCH][C_RESULT_ACCESS] = u64::MAX;
    a[C_ITLB][C_OP_PREFETCH][C_RESULT_MISS] = u64::MAX;
    a[C_BPU][C_OP_READ][C_RESULT_ACCESS] = 0x0012;
    a[C_BPU][C_OP_READ][C_RESULT_MISS] = 0x002b;
    a[C_BPU][C_OP_WRITE][C_RESULT_ACCESS] = u64::MAX;
    a[C_BPU][C_OP_WRITE][C_RESULT_MISS] = u64::MAX;
    a[C_BPU][C_OP_PREFETCH][C_RESULT_ACCESS] = u64::MAX;
    a[C_BPU][C_OP_PREFETCH][C_RESULT_MISS] = u64::MAX;
    a
};

unsafe fn knc_pmu_event_map(hw_event: i32) -> u64 { KNC_PERFMON_EVENT_MAP[hw_event as usize] }

static mut KNC_EVENT_CONSTRAINTS: [event_constraint; 22] = [
    INTEL_EVENT_CONSTRAINT!(0xc3, 0x1), INTEL_EVENT_CONSTRAINT!(0xc4, 0x1),
    INTEL_EVENT_CONSTRAINT!(0xc8, 0x1), INTEL_EVENT_CONSTRAINT!(0xc9, 0x1),
    INTEL_EVENT_CONSTRAINT!(0xca, 0x1), INTEL_EVENT_CONSTRAINT!(0xcb, 0x1),
    INTEL_EVENT_CONSTRAINT!(0xcc, 0x1), INTEL_EVENT_CONSTRAINT!(0xce, 0x1),
    INTEL_EVENT_CONSTRAINT!(0xcf, 0x1), INTEL_EVENT_CONSTRAINT!(0xd7, 0x1),
    INTEL_EVENT_CONSTRAINT!(0xe3, 0x1), INTEL_EVENT_CONSTRAINT!(0xe6, 0x1),
    INTEL_EVENT_CONSTRAINT!(0xe7, 0x1), INTEL_EVENT_CONSTRAINT!(0xf1, 0x1),
    INTEL_EVENT_CONSTRAINT!(0xf2, 0x1), INTEL_EVENT_CONSTRAINT!(0xf6, 0x1),
    INTEL_EVENT_CONSTRAINT!(0xf7, 0x1), INTEL_EVENT_CONSTRAINT!(0xfc, 0x1),
    INTEL_EVENT_CONSTRAINT!(0xfd, 0x1), INTEL_EVENT_CONSTRAINT!(0xfe, 0x1),
    INTEL_EVENT_CONSTRAINT!(0xff, 0x1), EVENT_CONSTRAINT_END!(),
];

const MSR_KNC_IA32_PERF_GLOBAL_STATUS: u32 = 0x0000002d;
const MSR_KNC_IA32_PERF_GLOBAL_OVF_CONTROL: u32 = 0x0000002e;
const MSR_KNC_IA32_PERF_GLOBAL_CTRL: u32 = 0x0000002f;
const KNC_ENABLE_COUNTER0: u64 = 0x00000001;
const KNC_ENABLE_COUNTER1: u64 = 0x00000002;

unsafe fn knc_pmu_disable_all() {
    let mut val: u64;
    rdmsrq!(MSR_KNC_IA32_PERF_GLOBAL_CTRL, val);
    val &= !(KNC_ENABLE_COUNTER0 | KNC_ENABLE_COUNTER1);
    wrmsrq!(MSR_KNC_IA32_PERF_GLOBAL_CTRL, val);
}

unsafe fn knc_pmu_enable_all(_added: i32) {
    let mut val: u64;
    rdmsrq!(MSR_KNC_IA32_PERF_GLOBAL_CTRL, val);
    val |= KNC_ENABLE_COUNTER0 | KNC_ENABLE_COUNTER1;
    wrmsrq!(MSR_KNC_IA32_PERF_GLOBAL_CTRL, val);
}

unsafe fn knc_pmu_disable_event(event: *mut perf_event) {
    let hwc = &mut (*event).hw;
    let val = hwc.config & !ARCH_PERFMON_EVENTSEL_ENABLE;
    let _ = wrmsrq_safe(hwc.config_base + hwc.idx, val);
}

unsafe fn knc_pmu_enable_event(event: *mut perf_event) {
    let hwc = &mut (*event).hw;
    let val = hwc.config | ARCH_PERFMON_EVENTSEL_ENABLE;
    let _ = wrmsrq_safe(hwc.config_base + hwc.idx, val);
}

unsafe fn knc_pmu_get_status() -> u64 {
    let mut status: u64;
    rdmsrq!(MSR_KNC_IA32_PERF_GLOBAL_STATUS, status);
    status
}

unsafe fn knc_pmu_ack_status(ack: u64) { wrmsrq!(MSR_KNC_IA32_PERF_GLOBAL_OVF_CONTROL, ack); }

unsafe fn knc_pmu_handle_irq(regs: *mut pt_regs) -> i32 {
    let mut data: perf_sample_data = core::mem::zeroed();
    let cpuc = this_cpu_ptr!(cpu_hw_events);
    let mut handled = 0;
    let mut status;
    knc_pmu_disable_all();
    status = knc_pmu_get_status();
    if status == 0 { knc_pmu_enable_all(0); return handled; }
    let mut loops = 0;
    loop {
        knc_pmu_ack_status(status);
        loops += 1;
        if loops > 100 { WARN_ONCE!(true, "perf: irq loop stuck!\n"); perf_event_print_debug(); break; }
        inc_perf_irq_stat!();
        for bit in for_each_set_bit(status, X86_PMC_IDX_MAX) {
            let event = (*cpuc).events[bit];
            handled += 1;
            if !test_bit(bit, (*cpuc).active_mask) { continue; }
            let last_period = (*event).hw.last_period;
            if !intel_pmu_save_and_restart(event) { continue; }
            perf_sample_data_init(&mut data, 0, last_period);
            perf_event_overflow(event, &mut data, regs);
        }
        status = knc_pmu_get_status();
        if status == 0 { break; }
    }
    if (*cpuc).enabled { knc_pmu_enable_all(0); }
    handled
}

PMU_FORMAT_ATTR!(event, "config:0-7");
PMU_FORMAT_ATTR!(umask, "config:8-15");
PMU_FORMAT_ATTR!(edge, "config:18");
PMU_FORMAT_ATTR!(inv, "config:23");
PMU_FORMAT_ATTR!(cmask, "config:24-31");

static mut INTEL_KNC_FORMATS_ATTR: [*mut attribute; 6] = [
    &format_attr_event.attr, &format_attr_umask.attr, &format_attr_edge.attr,
    &format_attr_inv.attr, &format_attr_cmask.attr, core::ptr::null_mut(),
];

static KNC_PMU: x86_pmu = x86_pmu {
    name: "knc", handle_irq: Some(knc_pmu_handle_irq), disable_all: Some(knc_pmu_disable_all),
    enable_all: Some(knc_pmu_enable_all), enable: Some(knc_pmu_enable_event),
    disable: Some(knc_pmu_disable_event), hw_config: Some(x86_pmu_hw_config),
    schedule_events: Some(x86_schedule_events), eventsel: MSR_KNC_EVNTSEL0,
    perfctr: MSR_KNC_PERFCTR0, event_map: Some(knc_pmu_event_map),
    max_events: KNC_PERFMON_EVENT_MAP.len(), apic: 1, max_period: (1u64 << 39) - 1,
    version: 0, cntr_mask64: 0x3, cntval_bits: 40, cntval_mask: (1u64 << 40) - 1,
    get_event_constraints: Some(x86_get_event_constraints), event_constraints: unsafe { &mut KNC_EVENT_CONSTRAINTS },
    format_attrs: unsafe { &mut INTEL_KNC_FORMATS_ATTR },
};

unsafe fn knc_pmu_init() -> i32 {
    x86_pmu = KNC_PMU;
    core::ptr::copy_nonoverlapping(KNC_HW_CACHE_EVENT_IDS.as_ptr(), hw_cache_event_ids.as_mut_ptr(),
        core::mem::size_of_val(&KNC_HW_CACHE_EVENT_IDS));
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
