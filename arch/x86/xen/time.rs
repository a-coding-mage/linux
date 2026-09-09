// SPDX-License-Identifier: GPL-2.0
/* Xen time implementation. */

// Kernel/Xen dependencies supplied by other translation units.

const TIMER_SLOP: u64 = 1;

static mut xen_sched_clock_offset: u64 = 0;

unsafe fn xen_tsc_khz() -> c_ulong {
    let info = &(*HYPERVISOR_shared_info).vcpu_info[0].time;
    setup_force_cpu_cap(X86_FEATURE_TSC_KNOWN_FREQ);
    pvclock_tsc_khz(info)
}

unsafe fn xen_clocksource_read() -> u64 {
    let mut ret;
    preempt_disable_notrace();
    let src = &mut (*__this_cpu_read(xen_vcpu)).time;
    ret = pvclock_clocksource_read(src);
    preempt_enable_notrace();
    ret
}

unsafe fn xen_clocksource_get_cycles(_cs: *mut clocksource) -> u64 {
    xen_clocksource_read()
}

unsafe fn xen_sched_clock() -> u64 {
    let src = &mut (*__this_cpu_read(xen_vcpu)).time;
    let ret = pvclock_clocksource_read_nowd(src);
    ret.wrapping_sub(xen_sched_clock_offset)
}

unsafe fn xen_read_wallclock(ts: *mut timespec64) {
    let s = HYPERVISOR_shared_info;
    let wall_clock = &(*s).wc;
    let vcpu_time = &mut get_cpu_var(xen_vcpu).time;
    pvclock_read_wallclock(wall_clock, vcpu_time, ts);
    put_cpu_var(xen_vcpu);
}

unsafe fn xen_get_wallclock(now: *mut timespec64) { xen_read_wallclock(now); }

unsafe fn xen_set_wallclock(_now: *const timespec64) -> c_int { -ENODEV }

unsafe fn xen_pvclock_gtod_notify(
    _nb: *mut notifier_block, was_set: c_ulong, priv_: *mut c_void,
) -> c_int {
    static mut next_sync: timespec64 = timespec64 { tv_sec: 0, tv_nsec: 0 };
    static mut settime64_supported: bool = true;
    let mut op: xen_platform_op = core::mem::zeroed();
    let mut now: timespec64 = core::mem::zeroed();
    let tk = priv_ as *mut timekeeper;
    now.tv_sec = (*tk).xtime_sec;
    now.tv_nsec = ((*tk).tkr_mono.xtime_nsec >> (*tk).tkr_mono.shift) as c_long;
    if was_set == 0 && timespec64_compare(&now, &next_sync) < 0 { return NOTIFY_OK; }
    loop {
        if settime64_supported {
            op.cmd = XENPF_settime64;
            op.u.settime64.mbz = 0;
            op.u.settime64.secs = now.tv_sec;
            op.u.settime64.nsecs = now.tv_nsec;
            op.u.settime64.system_time = xen_clocksource_read();
        } else {
            op.cmd = XENPF_settime32;
            op.u.settime32.secs = now.tv_sec;
            op.u.settime32.nsecs = now.tv_nsec;
            op.u.settime32.system_time = xen_clocksource_read();
        }
        let ret = HYPERVISOR_platform_op(&mut op);
        if ret == -ENOSYS && settime64_supported { settime64_supported = false; continue; }
        if ret < 0 { return NOTIFY_BAD; }
        next_sync = now;
        next_sync.tv_sec += 11 * 60;
        return NOTIFY_OK;
    }
}

static mut xen_pvclock_gtod_notifier: notifier_block = notifier_block { notifier_call: Some(xen_pvclock_gtod_notify) };

unsafe fn xen_cs_enable(_cs: *mut clocksource) -> c_int {
    vclocks_set_used(VDSO_CLOCKMODE_PVCLOCK); 0
}

static mut xen_clocksource: clocksource = clocksource {
    name: b"xen\0".as_ptr() as *const c_char, rating: 400,
    read: Some(xen_clocksource_get_cycles), mask: u64::MAX,
    flags: CLOCK_SOURCE_IS_CONTINUOUS, enable: Some(xen_cs_enable),
    ..unsafe { core::mem::zeroed() }
};

unsafe fn get_abs_timeout(delta: c_ulong) -> i64 { xen_clocksource_read() as i64 + delta as i64 }

unsafe fn xen_timerop_shutdown(_evt: *mut clock_event_device) -> c_int { HYPERVISOR_set_timer_op(0); 0 }

unsafe fn xen_timerop_set_next_event(delta: c_ulong, evt: *mut clock_event_device) -> c_int {
    WARN_ON(!clockevent_state_oneshot(evt));
    if HYPERVISOR_set_timer_op(get_abs_timeout(delta)) < 0 { BUG(); }
    0
}

static mut xen_timerop_clockevent: clock_event_device = clock_event_device {
    name: b"xen\0".as_ptr() as *const c_char, features: CLOCK_EVT_FEAT_ONESHOT,
    max_delta_ns: 0xffffffff, max_delta_ticks: 0xffffffff,
    min_delta_ns: TIMER_SLOP, min_delta_ticks: TIMER_SLOP, mult: 1, shift: 0, rating: 500,
    set_state_shutdown: Some(xen_timerop_shutdown), set_next_event: Some(xen_timerop_set_next_event),
    ..unsafe { core::mem::zeroed() }
};

unsafe fn xen_vcpuop_shutdown(_evt: *mut clock_event_device) -> c_int {
    let cpu = smp_processor_id();
    if HYPERVISOR_vcpu_op(VCPUOP_stop_singleshot_timer, xen_vcpu_nr(cpu), core::ptr::null_mut()) != 0 ||
       HYPERVISOR_vcpu_op(VCPUOP_stop_periodic_timer, xen_vcpu_nr(cpu), core::ptr::null_mut()) != 0 { BUG(); }
    0
}

unsafe fn xen_vcpuop_set_oneshot(_evt: *mut clock_event_device) -> c_int {
    let cpu = smp_processor_id();
    if HYPERVISOR_vcpu_op(VCPUOP_stop_periodic_timer, xen_vcpu_nr(cpu), core::ptr::null_mut()) != 0 { BUG(); }
    0
}

unsafe fn xen_vcpuop_set_next_event(delta: c_ulong, evt: *mut clock_event_device) -> c_int {
    let cpu = smp_processor_id();
    let mut single: vcpu_set_singleshot_timer = core::mem::zeroed();
    WARN_ON(!clockevent_state_oneshot(evt));
    single.timeout_abs_ns = get_abs_timeout(delta); single.flags = 0;
    let ret = HYPERVISOR_vcpu_op(VCPUOP_set_singleshot_timer, xen_vcpu_nr(cpu), &mut single);
    BUG_ON(ret != 0); ret
}

static mut xen_vcpuop_clockevent: clock_event_device = clock_event_device {
    name: b"xen\0".as_ptr() as *const c_char, features: CLOCK_EVT_FEAT_ONESHOT,
    max_delta_ns: 0xffffffff, max_delta_ticks: 0xffffffff,
    min_delta_ns: TIMER_SLOP, min_delta_ticks: TIMER_SLOP, mult: 1, shift: 0, rating: 500,
    set_state_shutdown: Some(xen_vcpuop_shutdown), set_state_oneshot: Some(xen_vcpuop_set_oneshot),
    set_next_event: Some(xen_vcpuop_set_next_event), ..unsafe { core::mem::zeroed() }
};

static mut xen_clockevent: *const clock_event_device = &xen_timerop_clockevent;

#[repr(C)]
struct xen_clock_event_device { evt: clock_event_device, name: [c_char; 16] }
static mut xen_clock_events: PerCpu<xen_clock_event_device> = DEFINE_PER_CPU_INIT(xen_clock_event_device { evt: clock_event_device { irq: -1, ..unsafe { core::mem::zeroed() } }, name: [0; 16] });

unsafe fn xen_timer_interrupt(_irq: c_int, _dev_id: *mut c_void) -> irqreturn_t {
    let evt = this_cpu_ptr(&mut xen_clock_events).cast::<xen_clock_event_device>().as_mut().unwrap();
    if let Some(handler) = evt.evt.event_handler { handler(&mut evt.evt); IRQ_HANDLED } else { IRQ_NONE }
}

#[no_mangle] pub unsafe fn xen_teardown_timer(cpu: c_int) {
    let evt = &mut per_cpu(&mut xen_clock_events, cpu).evt;
    if evt.irq >= 0 { unbind_from_irqhandler(evt.irq, core::ptr::null_mut()); evt.irq = -1; }
}

#[no_mangle] pub unsafe fn xen_setup_timer(cpu: c_int) {
    let xevt = &mut per_cpu(&mut xen_clock_events, cpu);
    let evt = &mut xevt.evt;
    WARN(evt.irq >= 0, b"IRQ%d for CPU%d is already allocated\0".as_ptr() as *const c_char, evt.irq, cpu);
    if evt.irq >= 0 { xen_teardown_timer(cpu); }
    printk(KERN_INFO, b"installing Xen timer for CPU %d\n\0".as_ptr() as *const c_char, cpu);
    snprintf(xevt.name.as_mut_ptr(), xevt.name.len(), b"timer%d\0".as_ptr() as *const c_char, cpu);
    let irq = bind_virq_to_irqhandler(VIRQ_TIMER, cpu, Some(xen_timer_interrupt), IRQF_PERCPU|IRQF_NOBALANCING|IRQF_TIMER|IRQF_FORCE_RESUME|IRQF_EARLY_RESUME, xevt.name.as_ptr(), core::ptr::null_mut());
    let _ = xen_set_irq_priority(irq, XEN_IRQ_PRIORITY_MAX);
    core::ptr::copy_nonoverlapping(xen_clockevent, evt, 1);
    evt.cpumask = cpumask_of(cpu); evt.irq = irq;
}

#[no_mangle] pub unsafe fn xen_setup_cpu_clockevents() { clockevents_register_device(&mut this_cpu_ptr(&mut xen_clock_events).evt); }

#[no_mangle] pub unsafe fn xen_timer_resume() {
    if xen_clockevent != &xen_vcpuop_clockevent { return; }
    for_each_online_cpu!(cpu, { if HYPERVISOR_vcpu_op(VCPUOP_stop_periodic_timer, xen_vcpu_nr(cpu), core::ptr::null_mut()) != 0 { BUG(); } });
}

static mut xen_clock: *mut pvclock_vsyscall_time_info = core::ptr::null_mut();
static mut xen_clock_value_saved: u64 = 0;

#[no_mangle] pub unsafe fn xen_save_time_memory_area() {
    let mut t: vcpu_register_time_memory_area = core::mem::zeroed();
    xen_clock_value_saved = xen_clocksource_read().wrapping_sub(xen_sched_clock_offset);
    if xen_clock.is_null() { return; }
    t.addr.v = core::ptr::null_mut();
    let ret = HYPERVISOR_vcpu_op(VCPUOP_register_vcpu_time_memory_area, 0, &mut t);
    if ret != 0 { pr_notice(b"Cannot save secondary vcpu_time_info (err %d)\0".as_ptr() as *const c_char, ret); } else { clear_page(xen_clock); }
}

#[no_mangle] pub unsafe fn xen_restore_time_memory_area() {
    let mut t: vcpu_register_time_memory_area = core::mem::zeroed();
    if !xen_clock.is_null() {
        t.addr.v = &mut (*xen_clock).pvti;
        let ret = HYPERVISOR_vcpu_op(VCPUOP_register_vcpu_time_memory_area, 0, &mut t);
        if ret != 0 { pr_notice(b"Cannot restore secondary vcpu_time_info (err %d)\0".as_ptr() as *const c_char, ret); }
    }
    pvclock_resume(); xen_sched_clock_offset = xen_clocksource_read().wrapping_sub(xen_clock_value_saved);
}

unsafe fn xen_setup_vsyscall_time_info() {
    let mut t: vcpu_register_time_memory_area = core::mem::zeroed();
    let ti = get_zeroed_page(GFP_KERNEL) as *mut pvclock_vsyscall_time_info;
    if ti.is_null() { return; }
    t.addr.v = &mut (*ti).pvti;
    let ret = HYPERVISOR_vcpu_op(VCPUOP_register_vcpu_time_memory_area, 0, &mut t);
    if ret != 0 { pr_notice(b"xen: VDSO_CLOCKMODE_PVCLOCK not supported (err %d)\n\0".as_ptr() as *const c_char, ret); free_page(ti as c_ulong); return; }
    if (*ti).pvti.flags & PVCLOCK_TSC_STABLE_BIT == 0 {
        t.addr.v = core::ptr::null_mut();
        ret_if!(HYPERVISOR_vcpu_op(VCPUOP_register_vcpu_time_memory_area, 0, &mut t), free_page(ti as c_ulong));
        pr_notice(b"xen: VDSO_CLOCKMODE_PVCLOCK not supported (tsc unstable)\n\0".as_ptr() as *const c_char); return;
    }
    xen_clock = ti; pvclock_set_pvti_cpu0_va(xen_clock); xen_clocksource.vdso_clock_mode = VDSO_CLOCKMODE_PVCLOCK;
}

unsafe fn xen_tsc_safe_clocksource() -> c_int {
    let (mut eax, mut ebx, mut ecx, mut edx) = (0, 0, 0, 0);
    if !boot_cpu_has(X86_FEATURE_CONSTANT_TSC) || !boot_cpu_has(X86_FEATURE_NONSTOP_TSC) || check_tsc_unstable() { return 0; }
    cpuid_count(xen_cpuid_base() + 3, 0, &mut eax, &mut ebx, &mut ecx, &mut edx);
    (ebx == XEN_CPUID_TSC_MODE_NEVER_EMULATE) as c_int
}

unsafe fn xen_time_init() {
    let cpu = smp_processor_id(); let mut tp: timespec64 = core::mem::zeroed();
    if xen_initial_domain() { xen_clocksource.rating = 275; } else if xen_tsc_safe_clocksource() != 0 { xen_clocksource.rating = 299; }
    clocksource_register_hz(&mut xen_clocksource, NSEC_PER_SEC);
    if HYPERVISOR_vcpu_op(VCPUOP_stop_periodic_timer, xen_vcpu_nr(cpu), core::ptr::null_mut()) == 0 { printk(KERN_DEBUG, b"Xen: using vcpuop timer interface\n\0".as_ptr() as *const c_char); xen_clockevent = &xen_vcpuop_clockevent; }
    xen_read_wallclock(&mut tp); do_settimeofday64(&tp); setup_force_cpu_cap(X86_FEATURE_TSC);
    let pvti = &(*__this_cpu_read(xen_vcpu)).time;
    if pvti.flags & PVCLOCK_TSC_STABLE_BIT != 0 { pvclock_set_flags(PVCLOCK_TSC_STABLE_BIT); xen_setup_vsyscall_time_info(); }
    xen_setup_runstate_info(cpu); xen_setup_timer(cpu); xen_setup_cpu_clockevents(); xen_time_setup_guest();
    if xen_initial_domain() { pvclock_gtod_register_notifier(&mut xen_pvclock_gtod_notifier); }
}

unsafe fn xen_init_time_common() { xen_sched_clock_offset = xen_clocksource_read(); static_call_update(pv_steal_clock, xen_steal_clock); paravirt_set_sched_clock(xen_sched_clock); x86_platform.calibrate_tsc = Some(xen_tsc_khz); x86_platform.get_wallclock = Some(xen_get_wallclock); }

#[no_mangle] pub unsafe fn xen_init_time_ops() { xen_init_time_common(); x86_init.timers.timer_init = Some(xen_time_init); x86_init.timers.setup_percpu_clockev = Some(x86_init_noop); x86_cpuinit.setup_percpu_clockev = Some(x86_init_noop); if !xen_initial_domain() { x86_platform.set_wallclock = Some(xen_set_wallclock); } }

unsafe fn xen_hvm_setup_cpu_clockevents() { let cpu = smp_processor_id(); xen_setup_runstate_info(cpu); xen_setup_cpu_clockevents(); }

#[no_mangle] pub unsafe fn xen_hvm_init_time_ops() {
    static mut hvm_time_initialized: bool = false;
    if hvm_time_initialized || !xen_have_vector_callback || !xen_feature(XENFEAT_hvm_safe_pvclock) { return; }
    if __this_cpu_read(xen_vcpu).is_null() { pr_info(b"Delay xen_init_time_common() as kernel is running on vcpu=%d\n\0".as_ptr() as *const c_char, xen_vcpu_nr(0)); return; }
    xen_init_time_common(); x86_init.timers.setup_percpu_clockev = Some(xen_time_init); x86_cpuinit.setup_percpu_clockev = Some(xen_hvm_setup_cpu_clockevents); x86_platform.set_wallclock = Some(xen_set_wallclock); hvm_time_initialized = true;
}

unsafe fn parse_xen_timer_slop(ptr: *mut c_char) -> c_int {
    let slop = memparse(ptr, core::ptr::null_mut());
    xen_timerop_clockevent.min_delta_ns = slop; xen_timerop_clockevent.min_delta_ticks = slop; xen_vcpuop_clockevent.min_delta_ns = slop; xen_vcpuop_clockevent.min_delta_ticks = slop; 0
}

// early_param("xen_timer_slop", parse_xen_timer_slop);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
