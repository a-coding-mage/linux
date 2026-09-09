// SPDX-License-Identifier: GPL-2.0-only
/* Zhaoxin PMU; like Intel Architectural PerfMon-v2 */

// Kernel includes and macros are supplied by the surrounding translation unit.

static mut zx_pmon_event_map: [u64; PERF_COUNT_HW_MAX as usize] = {
    let mut a = [0u64; PERF_COUNT_HW_MAX as usize];
    a[PERF_COUNT_HW_CPU_CYCLES as usize] = 0x0082;
    a[PERF_COUNT_HW_INSTRUCTIONS as usize] = 0x00c0;
    a[PERF_COUNT_HW_CACHE_REFERENCES as usize] = 0x0515;
    a[PERF_COUNT_HW_CACHE_MISSES as usize] = 0x051a;
    a[PERF_COUNT_HW_BUS_CYCLES as usize] = 0x0083;
    a
};

static mut zxc_event_constraints: [struct_event_constraint; 2] = [
    FIXED_EVENT_CONSTRAINT!(0x0082, 1), EVENT_CONSTRAINT_END!,
];
static mut zxd_event_constraints: [struct_event_constraint; 4] = [
    FIXED_EVENT_CONSTRAINT!(0x00c0, 0), FIXED_EVENT_CONSTRAINT!(0x0082, 1),
    FIXED_EVENT_CONSTRAINT!(0x0083, 2), EVENT_CONSTRAINT_END!,
];

// C designated initializers below are represented as nested Rust arrays. -1 is the
// C sentinel and is retained through a signed element type.
static zxd_hw_cache_event_ids: [[[i64; PERF_COUNT_HW_CACHE_RESULT_MAX as usize]; PERF_COUNT_HW_CACHE_OP_MAX as usize]; PERF_COUNT_HW_CACHE_MAX as usize] = [
    [[0x0042,0x0538],[0x0043,0x0562],[-1,-1]],
    [[0x0300,0x0301],[-1,-1],[0x030a,0x030b]],
    [[-1,-1],[-1,-1],[-1,-1]],
    [[0x0042,0x052c],[0x0043,0x0530],[0x0564,0x0565]],
    [[0x00c0,0x0534],[-1,-1],[-1,-1]],
    [[0x0700,0x0709],[-1,-1],[-1,-1]],
    [[-1,-1],[-1,-1],[-1,-1]],
];
static zxe_hw_cache_event_ids: [[[i64; PERF_COUNT_HW_CACHE_RESULT_MAX as usize]; PERF_COUNT_HW_CACHE_OP_MAX as usize]; PERF_COUNT_HW_CACHE_MAX as usize] = [
    [[0x0568,0x054b],[0x0669,0x0562],[-1,-1]],
    [[0x0300,0x0301],[-1,-1],[0x030a,0x030b]],
    [[0,0],[0,0],[0,0]],
    [[0x0568,0x052c],[0x0669,0x0530],[0x0564,0x0565]],
    [[0x00c0,0x0534],[-1,-1],[-1,-1]],
    [[0x0028,0x0029],[-1,-1],[-1,-1]],
    [[-1,-1],[-1,-1],[-1,-1]],
];

unsafe fn zhaoxin_pmu_disable_all() { wrmsrq(MSR_CORE_PERF_GLOBAL_CTRL, 0); }
unsafe fn zhaoxin_pmu_enable_all(_added: i32) { wrmsrq(MSR_CORE_PERF_GLOBAL_CTRL, x86_pmu.intel_ctrl); }
unsafe fn zhaoxin_pmu_get_status() -> u64 { let mut status = 0; rdmsrq(MSR_CORE_PERF_GLOBAL_STATUS, status); status }
unsafe fn zhaoxin_pmu_ack_status(ack: u64) { wrmsrq(MSR_CORE_PERF_GLOBAL_OVF_CTRL, ack); }
unsafe fn zxc_pmu_ack_status(ack: u64) { zhaoxin_pmu_enable_all(0); zhaoxin_pmu_ack_status(ack); zhaoxin_pmu_disable_all(); }

unsafe fn zhaoxin_pmu_disable_fixed(hwc: *mut hw_perf_event) {
    let idx = (*hwc).idx - INTEL_PMC_IDX_FIXED; let mask = 0xfu64 << (idx * 4); let mut ctrl_val = 0;
    rdmsrq((*hwc).config_base, ctrl_val); ctrl_val &= !mask; wrmsrq((*hwc).config_base, ctrl_val);
}
unsafe fn zhaoxin_pmu_disable_event(event: *mut perf_event) {
    let hwc = &mut (*event).hw;
    if unlikely(hwc.config_base == MSR_ARCH_PERFMON_FIXED_CTR_CTRL) { zhaoxin_pmu_disable_fixed(hwc); return; }
    x86_pmu_disable_event(event);
}
unsafe fn zhaoxin_pmu_enable_fixed(hwc: *mut hw_perf_event) {
    let idx = (*hwc).idx - INTEL_PMC_IDX_FIXED; let mut bits = 0x8u64;
    if (*hwc).config & ARCH_PERFMON_EVENTSEL_USR != 0 { bits |= 0x2; }
    if (*hwc).config & ARCH_PERFMON_EVENTSEL_OS != 0 { bits |= 0x1; }
    bits <<= idx * 4; let mask = 0xfu64 << (idx * 4); let mut ctrl_val = 0;
    rdmsrq((*hwc).config_base, ctrl_val); ctrl_val &= !mask; ctrl_val |= bits; wrmsrq((*hwc).config_base, ctrl_val);
}
unsafe fn zhaoxin_pmu_enable_event(event: *mut perf_event) {
    let hwc = &mut (*event).hw;
    if unlikely(hwc.config_base == MSR_ARCH_PERFMON_FIXED_CTR_CTRL) { zhaoxin_pmu_enable_fixed(hwc); return; }
    __x86_pmu_enable_event(hwc, ARCH_PERFMON_EVENTSEL_ENABLE);
}

unsafe fn zhaoxin_pmu_handle_irq(regs: *mut pt_regs) -> i32 {
    let mut data = perf_sample_data::default(); let cpuc = this_cpu_ptr(&cpu_hw_events); let mut handled = 0; let mut status;
    apic_write(APIC_LVTPC, APIC_DM_NMI); zhaoxin_pmu_disable_all(); status = zhaoxin_pmu_get_status(); if status == 0 { zhaoxin_pmu_enable_all(0); return 0; }
    loop {
        if x86_pmu.enabled_ack != 0 { zxc_pmu_ack_status(status); } else { zhaoxin_pmu_ack_status(status); }
        inc_perf_irq_stat();
        if __test_and_clear_bit(63, &mut status as *mut u64 as *mut unsigned_long) != 0 && status == 0 { break; }
        for_each_set_bit!(bit, &status as *const u64 as *const unsigned_long, X86_PMC_IDX_MAX) {
            let event = (*cpuc).events[bit]; handled += 1;
            if !test_bit(bit, (*cpuc).active_mask) { continue; }
            x86_perf_event_update(event); perf_sample_data_init(&mut data, 0, (*event).hw.last_period);
            if !x86_perf_event_set_period(event) { continue; } perf_event_overflow(event, &mut data, regs);
        }
        status = zhaoxin_pmu_get_status(); if status == 0 { break; }
    }
    zhaoxin_pmu_enable_all(0); handled
}

unsafe fn zhaoxin_pmu_event_map(hw_event: i32) -> u64 { zx_pmon_event_map[hw_event as usize] }
unsafe fn zhaoxin_get_event_constraints(_cpuc: *mut cpu_hw_events, _idx: i32, event: *mut perf_event) -> *mut event_constraint {
    if !x86_pmu.event_constraints.is_null() { for_each_event_constraint!(c, x86_pmu.event_constraints) { if ((*event).hw.config & (*c).cmask) == (*c).code { return c; } } }
    &mut unconstrained
}

// PMU_FORMAT_ATTR declarations and the x86_pmu initializer retain their kernel ABI
// fields; their concrete macro/type definitions are supplied externally.
PMU_FORMAT_ATTR!(event, "config:0-7"); PMU_FORMAT_ATTR!(umask, "config:8-15");
PMU_FORMAT_ATTR!(edge, "config:18"); PMU_FORMAT_ATTR!(inv, "config:23"); PMU_FORMAT_ATTR!(cmask, "config:24-31");
static mut zx_arch_formats_attr: [*mut attribute; 6] = [
    &mut format_attr_event.attr, &mut format_attr_umask.attr, &mut format_attr_edge.attr,
    &mut format_attr_inv.attr, &mut format_attr_cmask.attr, core::ptr::null_mut(),
];
unsafe fn zhaoxin_event_sysfs_show(page: *mut i8, config: u64) -> isize { let event = config & ARCH_PERFMON_EVENTSEL_EVENT; x86_event_sysfs_show(page, config, event) }

static mut zhaoxin_pmu: x86_pmu = x86_pmu {
    name: "zhaoxin", handle_irq: Some(zhaoxin_pmu_handle_irq), disable_all: Some(zhaoxin_pmu_disable_all), enable_all: Some(zhaoxin_pmu_enable_all),
    enable: Some(zhaoxin_pmu_enable_event), disable: Some(zhaoxin_pmu_disable_event), hw_config: Some(x86_pmu_hw_config), schedule_events: Some(x86_schedule_events),
    eventsel: MSR_ARCH_PERFMON_EVENTSEL0, perfctr: MSR_ARCH_PERFMON_PERFCTR0, event_map: Some(zhaoxin_pmu_event_map), max_events: ARRAY_SIZE!(zx_pmon_event_map), apic: 1,
    max_period: (1u64 << 47) - 1, get_event_constraints: Some(zhaoxin_get_event_constraints), format_attrs: zx_arch_formats_attr.as_mut_ptr(), events_sysfs_show: Some(zhaoxin_event_sysfs_show),
};

static zx_arch_events_map: [(i32, &str); 7] = [(PERF_COUNT_HW_CPU_CYCLES,"cpu cycles"),(PERF_COUNT_HW_INSTRUCTIONS,"instructions"),(PERF_COUNT_HW_BUS_CYCLES,"bus cycles"),(PERF_COUNT_HW_CACHE_REFERENCES,"cache references"),(PERF_COUNT_HW_CACHE_MISSES,"cache misses"),(PERF_COUNT_HW_BRANCH_INSTRUCTIONS,"branch instructions"),(PERF_COUNT_HW_BRANCH_MISSES,"branch misses")];
unsafe fn zhaoxin_arch_events_quirk() { for_each_set_bit!(bit, x86_pmu.events_mask, ARRAY_SIZE!(zx_arch_events_map)) { zx_pmon_event_map[zx_arch_events_map[bit].0 as usize] = 0; pr_warn!("CPUID marked event: '{}' unavailable\n", zx_arch_events_map[bit].1); } }

unsafe fn zhaoxin_pmu_init() -> i32 {
    let mut edx = union_cpuid10_edx::default(); let mut eax = union_cpuid10_eax::default(); let mut ebx = union_cpuid10_ebx::default(); let mut unused = 0; let mut c; let mut version;
    pr_info!("Welcome to zhaoxin pmu!\n"); cpuid(10, &mut eax.full, &mut ebx.full, &mut unused, &mut edx.full);
    if eax.split.mask_length < ARCH_PERFMON_EVENTS_COUNT - 1 { return -ENODEV; } version = eax.split.version_id; if version != 2 { return -ENODEV; }
    x86_pmu = zhaoxin_pmu; pr_info!("Version check pass!\n"); x86_pmu.version = version; x86_pmu.cntr_mask64 = GENMASK_ULL!(eax.split.num_counters - 1, 0); x86_pmu.cntval_bits = eax.split.bit_width; x86_pmu.cntval_mask = (1u64 << eax.split.bit_width) - 1; x86_pmu.events_maskl = ebx.full; x86_pmu.events_mask_len = eax.split.mask_length; x86_pmu.fixed_cntr_mask64 = GENMASK_ULL!(edx.split.num_counters_fixed - 1, 0); x86_add_quirk(zhaoxin_arch_events_quirk);
    match boot_cpu_data.x86 { 0x06 => { if (boot_cpu_data.x86_model == 0x0f && boot_cpu_data.x86_stepping >= 0x0e) || boot_cpu_data.x86_model == 0x19 { x86_pmu.max_period = x86_pmu.cntval_mask >> 1; x86_pmu.enabled_ack = 1; x86_pmu.event_constraints = zxc_event_constraints.as_mut_ptr(); zx_pmon_event_map[PERF_COUNT_HW_INSTRUCTIONS as usize]=0; zx_pmon_event_map[PERF_COUNT_HW_CACHE_REFERENCES as usize]=0; zx_pmon_event_map[PERF_COUNT_HW_CACHE_MISSES as usize]=0; zx_pmon_event_map[PERF_COUNT_HW_BUS_CYCLES as usize]=0; pr_cont!("ZXC events, "); } else { return -ENODEV; } },
        0x07 => { zx_pmon_event_map[PERF_COUNT_HW_STALLED_CYCLES_FRONTEND as usize] = X86_CONFIG!(event=0x01,umask=0x01,inv=0x01,cmask=0x01); zx_pmon_event_map[PERF_COUNT_HW_STALLED_CYCLES_BACKEND as usize] = X86_CONFIG!(event=0x0f,umask=0x04,inv=0,cmask=0); match boot_cpu_data.x86_model { 0x1b => { hw_cache_event_ids.copy_from_slice(&zxd_hw_cache_event_ids); x86_pmu.event_constraints=zxd_event_constraints.as_mut_ptr(); zx_pmon_event_map[PERF_COUNT_HW_BRANCH_INSTRUCTIONS as usize]=0x0700; zx_pmon_event_map[PERF_COUNT_HW_BRANCH_MISSES as usize]=0x0709; pr_cont!("ZXD events, "); }, 0x3b => { hw_cache_event_ids.copy_from_slice(&zxe_hw_cache_event_ids); x86_pmu.event_constraints=zxd_event_constraints.as_mut_ptr(); zx_pmon_event_map[PERF_COUNT_HW_BRANCH_INSTRUCTIONS as usize]=0x0028; zx_pmon_event_map[PERF_COUNT_HW_BRANCH_MISSES as usize]=0x0029; pr_cont!("ZXE events, "); }, _ => return -ENODEV } }, _ => return -ENODEV }
    x86_pmu.intel_ctrl = x86_pmu.cntr_mask64 | (x86_pmu.fixed_cntr_mask64 << INTEL_PMC_IDX_FIXED); if !x86_pmu.event_constraints.is_null() { for_each_event_constraint!(c, x86_pmu.event_constraints) { (*c).idxmsk64 |= x86_pmu.cntr_mask64; (*c).weight += x86_pmu_num_counters(core::ptr::null_mut()); } } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
