// SPDX-License-Identifier: GPL-2.0

/* Clocksource driver for the synthetic counter and timers provided by Hyper-V. */

// Kernel headers and configuration-provided symbols are external dependencies.

static mut hv_clock_event: *mut clock_event_device = core::ptr::null_mut();
// Note: offset can hold negative values after hibernation.
static mut hv_sched_clock_offset: u64 = 0;

static mut stimer0_irq: i32 = -1;
static mut stimer0_evt: i64 = 0;

unsafe fn hv_stimer0_isr() {
    let ce = this_cpu_ptr(hv_clock_event);
    ((*ce).event_handler.unwrap())(ce);
}

unsafe extern "C" fn hv_stimer0_percpu_isr(_irq: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    hv_stimer0_isr();
    IRQ_HANDLED
}

unsafe extern "C" fn hv_ce_set_next_event(delta: u64, _evt: *mut clock_event_device) -> i32 {
    let mut current_tick = hv_read_reference_counter();
    current_tick = current_tick.wrapping_add(delta);
    hv_set_msr(HV_MSR_STIMER0_COUNT, current_tick);
    0
}

unsafe extern "C" fn hv_ce_shutdown(_evt: *mut clock_event_device) -> i32 {
    hv_set_msr(HV_MSR_STIMER0_COUNT, 0);
    hv_set_msr(HV_MSR_STIMER0_CONFIG, 0);
    if stimer0_irq >= 0 { disable_percpu_irq(stimer0_irq); }
    0
}

unsafe extern "C" fn hv_ce_set_oneshot(_evt: *mut clock_event_device) -> i32 {
    let mut timer_cfg = hv_stimer_config { as_uint64: 0 };
    timer_cfg.enable = 1;
    timer_cfg.auto_enable = 1;
    timer_cfg.direct_mode = 1;
    timer_cfg.apic_vector = HYPERV_STIMER0_VECTOR;
    if stimer0_irq >= 0 { enable_percpu_irq(stimer0_irq, IRQ_TYPE_NONE); }
    hv_set_msr(HV_MSR_STIMER0_CONFIG, timer_cfg.as_uint64);
    0
}

unsafe extern "C" fn hv_stimer_init(cpu: u32) -> i32 {
    if hv_clock_event.is_null() { return 0; }
    let ce = per_cpu_ptr(hv_clock_event, cpu);
    (*ce).name = "Hyper-V clockevent\0".as_ptr() as *mut i8;
    (*ce).features = CLOCK_EVT_FEAT_ONESHOT;
    (*ce).cpumask = cpumask_of(cpu);
    if !ms_hyperv.paravisor_present && hv_isolation_type_tdx() { (*ce).rating = 90; }
    else { (*ce).rating = 1000; }
    (*ce).set_state_shutdown = Some(hv_ce_shutdown);
    (*ce).set_state_oneshot = Some(hv_ce_set_oneshot);
    (*ce).set_next_event = Some(hv_ce_set_next_event);
    clockevents_config_and_register(ce, HV_CLOCK_HZ, HV_MIN_DELTA_TICKS, HV_MAX_MAX_DELTA_TICKS);
    0
}

pub unsafe extern "C" fn hv_stimer_cleanup(cpu: u32) -> i32 {
    if hv_clock_event.is_null() { return 0; }
    let ce = per_cpu_ptr(hv_clock_event, cpu);
    hv_ce_shutdown(ce);
    0
}

pub unsafe extern "C" fn hv_setup_stimer0_handler(_handler: unsafe fn()) {}
pub unsafe extern "C" fn hv_remove_stimer0_handler() {}

unsafe fn hv_setup_stimer0_irq() -> i32 {
    let ret = acpi_register_gsi(core::ptr::null_mut(), HYPERV_STIMER0_VECTOR, ACPI_EDGE_SENSITIVE, ACPI_ACTIVE_HIGH);
    if ret < 0 { pr_err!("Can't register Hyper-V stimer0 GSI. Error %d", ret); return ret; }
    stimer0_irq = ret;
    let ret = request_percpu_irq(stimer0_irq, hv_stimer0_percpu_isr, "Hyper-V stimer0\0".as_ptr() as *const i8, &mut stimer0_evt);
    if ret != 0 {
        pr_err!("Can't request Hyper-V stimer0 IRQ %d. Error %d", stimer0_irq, ret);
        acpi_unregister_gsi(stimer0_irq);
        stimer0_irq = -1;
    }
    ret
}

unsafe fn hv_remove_stimer0_irq() {
    if stimer0_irq == -1 { hv_remove_stimer0_handler(); }
    else { free_percpu_irq(stimer0_irq, &mut stimer0_evt); acpi_unregister_gsi(stimer0_irq); stimer0_irq = -1; }
}

pub unsafe extern "C" fn hv_stimer_alloc(have_percpu_irqs: bool) -> i32 {
    if (ms_hyperv.features & HV_MSR_SYNTIMER_AVAILABLE) == 0 || (ms_hyperv.misc_features & HV_STIMER_DIRECT_MODE_AVAILABLE) == 0 { return -EINVAL; }
    hv_clock_event = alloc_percpu::<clock_event_device>();
    if hv_clock_event.is_null() { return -ENOMEM; }
    let mut ret;
    if have_percpu_irqs { ret = hv_setup_stimer0_irq(); if ret != 0 { free_percpu(hv_clock_event); hv_clock_event = core::ptr::null_mut(); return ret; } }
    else { hv_setup_stimer0_handler(hv_stimer0_isr); }
    ret = cpuhp_setup_state(CPUHP_AP_HYPERV_TIMER_STARTING, "clockevents/hyperv/stimer:starting\0".as_ptr() as *const i8, hv_stimer_init, hv_stimer_cleanup);
    if ret < 0 { hv_remove_stimer0_irq(); free_percpu(hv_clock_event); hv_clock_event = core::ptr::null_mut(); }
    ret
}

pub unsafe extern "C" fn hv_stimer_global_cleanup() {
    if hv_clock_event.is_null() { return; }
    cpuhp_remove_state(CPUHP_AP_HYPERV_TIMER_STARTING); hv_remove_stimer0_irq(); stimer0_irq = -1;
    free_percpu(hv_clock_event); hv_clock_event = core::ptr::null_mut();
}

unsafe fn read_hv_clock_msr() -> u64 { hv_raw_get_msr(HV_MSR_TIME_REF_COUNT) }

#[repr(C)]
union tsc_page_storage { page: ms_hyperv_tsc_page, reserved: [u8; PAGE_SIZE] }
static mut tsc_pg: tsc_page_storage = tsc_page_storage { reserved: [0; PAGE_SIZE] };
static mut tsc_page: *mut ms_hyperv_tsc_page = core::ptr::null_mut();
static mut tsc_pfn: usize = 0;

pub unsafe extern "C" fn hv_get_tsc_pfn() -> usize { tsc_pfn }
pub unsafe extern "C" fn hv_get_tsc_page() -> *mut ms_hyperv_tsc_page { if tsc_page.is_null() { &mut tsc_pg.page } else { tsc_page } }

unsafe fn read_hv_clock_tsc() -> u64 {
    let mut cur_tsc = 0; let mut time = 0;
    if !hv_read_tsc_page_tsc(hv_get_tsc_page(), &mut cur_tsc, &mut time) { time = read_hv_clock_msr(); }
    time
}

unsafe extern "C" fn read_hv_clock_tsc_cs(_arg: *mut clocksource) -> u64 { read_hv_clock_tsc() }
unsafe extern "C" fn read_hv_clock_tsc_cs_snapshot(_arg: *mut clocksource, chs: *mut clocksource_hw_snapshot) -> u64 {
    let mut time = 0; let mut cycles = 0;
    if hv_read_tsc_page_tsc(hv_get_tsc_page(), &mut cycles, &mut time) { (*chs).hw_cycles = cycles; (*chs).hw_csid = CSID_X86_TSC; }
    else { (*chs).hw_cycles = 0; (*chs).hw_csid = CSID_GENERIC; time = read_hv_clock_msr(); }
    time
}
unsafe fn read_hv_sched_clock_tsc() -> u64 { (read_hv_clock_tsc().wrapping_sub(hv_sched_clock_offset)).wrapping_mul(NSEC_PER_SEC / HV_CLOCK_HZ) }
unsafe extern "C" fn read_hv_clock_msr_cs(_arg: *mut clocksource) -> u64 { read_hv_clock_msr() }

unsafe fn suspend_hv_clock_tsc(_arg: *mut clocksource) { let mut m = hv_reference_tsc_msr { as_uint64: hv_get_msr(HV_MSR_REFERENCE_TSC) }; m.enable = 0; hv_set_msr(HV_MSR_REFERENCE_TSC, m.as_uint64); }
unsafe fn resume_hv_clock_tsc(_arg: *mut clocksource) { let mut m = hv_reference_tsc_msr { as_uint64: hv_get_msr(HV_MSR_REFERENCE_TSC) }; m.enable = 1; m.pfn = tsc_pfn; hv_set_msr(HV_MSR_REFERENCE_TSC, m.as_uint64); }
pub unsafe extern "C" fn hv_adj_sched_clock_offset(offset: u64) { hv_sched_clock_offset = hv_sched_clock_offset.wrapping_sub(offset); }

#[cfg(feature = "HAVE_VDSO_CLOCKMODE_HVCLOCK")]
unsafe fn hv_cs_enable(_cs: *mut clocksource) -> i32 { vclocks_set_used(VDSO_CLOCKMODE_HVCLOCK); 0 }

static mut hyperv_cs_tsc: clocksource = clocksource {
    name: "hyperv_clocksource_tsc_page\0".as_ptr() as *const i8, rating: 500,
    read: Some(read_hv_clock_tsc_cs), read_snapshot: Some(read_hv_clock_tsc_cs_snapshot),
    mask: CLOCKSOURCE_MASK(64), flags: CLOCK_SOURCE_IS_CONTINUOUS,
    suspend: Some(suspend_hv_clock_tsc), resume: Some(resume_hv_clock_tsc), ..clocksource::ZERO
};
static mut hyperv_cs_msr: clocksource = clocksource {
    name: "hyperv_clocksource_msr\0".as_ptr() as *const i8, rating: 495,
    read: Some(read_hv_clock_msr_cs), mask: CLOCKSOURCE_MASK(64), flags: CLOCK_SOURCE_IS_CONTINUOUS,
    ..clocksource::ZERO
};

unsafe fn hv_setup_sched_clock(sched_clock: *mut core::ffi::c_void) {
    // Configuration selects generic sched_clock, paravirtualized sched_clock, or no setup.
    sched_clock_register(sched_clock, 64, NSEC_PER_SEC);
}

pub unsafe extern "C" fn hv_init_tsc_clocksource() {
    let mut m = hv_reference_tsc_msr { as_uint64: hv_get_msr(HV_MSR_REFERENCE_TSC) };
    if (ms_hyperv.features & HV_MSR_REFERENCE_TSC_AVAILABLE) == 0 { return; }
    if hv_root_partition() { tsc_pfn = m.pfn as usize; } else { tsc_pfn = HVPFN_DOWN(virt_to_phys(hv_get_tsc_page())) as usize; }
    m.enable = 1; m.pfn = tsc_pfn as _; hv_set_msr(HV_MSR_REFERENCE_TSC, m.as_uint64);
    clocksource_register_hz(&mut hyperv_cs_tsc, NSEC_PER_SEC / 100);
    if (ms_hyperv.features & HV_ACCESS_TSC_INVARIANT) == 0 { hv_sched_clock_offset = hv_read_reference_counter(); hv_setup_sched_clock(read_hv_clock_tsc as *mut _); }
}

pub unsafe extern "C" fn hv_init_clocksource() { hv_init_tsc_clocksource(); if (ms_hyperv.features & HV_MSR_TIME_REF_COUNT_AVAILABLE) != 0 { clocksource_register_hz(&mut hyperv_cs_msr, NSEC_PER_SEC / 100); } }

pub unsafe extern "C" fn hv_remap_tsc_clocksource() {
    if (ms_hyperv.features & HV_MSR_REFERENCE_TSC_AVAILABLE) == 0 || !hv_root_partition() { return; }
    tsc_page = memremap((tsc_pfn << HV_HYP_PAGE_SHIFT) as _, core::mem::size_of::<tsc_page_storage>(), MEMREMAP_WB);
    if tsc_page.is_null() { pr_err!("Failed to remap Hyper-V TSC page.\n"); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
