// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2012 Regents of the University of California
 * Copyright (C) 2017 SiFive
 *
 * All RISC-V systems have a timer attached to every hart.  These timers can
 * either be read from the "time" and "timeh" CSRs, and can use the SBI to
 * setup events, or directly accessed using MMIO registers.
 */

// Linux kernel dependencies supplied by other translation units.

static mut RISCV_SSTC_AVAILABLE: bool = false;
static mut riscv_timer_cannot_wake_cpu: bool = false;

unsafe fn riscv_clock_event_stop() {
    if RISCV_SSTC_AVAILABLE {
        csr_write(CSR_STIMECMP, ULONG_MAX);
        // CONFIG_32BIT: also write CSR_STIMECMPH.
        if cfg!(target_pointer_width = "32") {
            csr_write(CSR_STIMECMPH, ULONG_MAX);
        }
    } else {
        sbi_set_timer(U64_MAX);
    }
}

unsafe fn riscv_clock_next_event(delta: c_ulong, _ce: *mut clock_event_device) -> c_int {
    let next_tval: u64 = get_cycles64().wrapping_add(delta as u64);

    if RISCV_SSTC_AVAILABLE {
        // CONFIG_32BIT uses the high/low CSR write sequence.
        if cfg!(target_pointer_width = "32") {
            csr_write(CSR_STIMECMP, ULONG_MAX);
            csr_write(CSR_STIMECMPH, next_tval >> 32);
            csr_write(CSR_STIMECMP, next_tval & 0xffff_ffff);
        } else {
            csr_write(CSR_STIMECMP, next_tval);
        }
    } else {
        sbi_set_timer(next_tval);
    }
    0
}

unsafe fn riscv_clock_shutdown(_evt: *mut clock_event_device) -> c_int {
    riscv_clock_event_stop();
    0
}

static mut riscv_clock_event_irq: c_uint = 0;
static mut riscv_clock_event: clock_event_device = clock_event_device {
    name: b"riscv_timer_clockevent\0".as_ptr() as *const c_char,
    features: CLOCK_EVT_FEAT_ONESHOT,
    rating: 100,
    set_next_event: Some(riscv_clock_next_event),
    set_state_shutdown: Some(riscv_clock_shutdown),
    ..clock_event_device::default()
};

/*
 * It is guaranteed that all the timers across all the harts are synchronized
 * within one tick of each other, so while this could technically go
 * backwards when hopping between CPUs, practically it won't happen.
 */
unsafe fn riscv_clocksource_rdtime(_cs: *mut clocksource) -> c_ulonglong {
    get_cycles64()
}

unsafe fn riscv_sched_clock() -> u64 {
    get_cycles64()
}

static mut riscv_clocksource: clocksource = clocksource {
    name: b"riscv_clocksource\0".as_ptr() as *const c_char,
    rating: 400,
    mask: CLOCKSOURCE_MASK(64),
    flags: CLOCK_SOURCE_IS_CONTINUOUS,
    read: Some(riscv_clocksource_rdtime),
    vdso_clock_mode: VDSO_CLOCKMODE_ARCHTIMER,
    ..clocksource::default()
};

unsafe fn riscv_timer_starting_cpu(cpu: c_uint) -> c_int {
    let ce = per_cpu_ptr(&mut riscv_clock_event, cpu);
    riscv_clock_event_stop();
    (*ce).cpumask = cpumask_of(cpu);
    (*ce).irq = riscv_clock_event_irq;
    if riscv_timer_cannot_wake_cpu {
        (*ce).features |= CLOCK_EVT_FEAT_C3STOP;
    }
    if RISCV_SSTC_AVAILABLE {
        (*ce).rating = 450;
    }
    clockevents_config_and_register(ce, riscv_timebase, 100, ULONG_MAX);
    enable_percpu_irq(riscv_clock_event_irq, irq_get_trigger_type(riscv_clock_event_irq));
    0
}

unsafe fn riscv_timer_dying_cpu(_cpu: c_uint) -> c_int {
    /* Stop the timer when the cpu is going to be offline otherwise
     * the timer interrupt may be pending while performing power-down. */
    riscv_clock_event_stop();
    disable_percpu_irq(riscv_clock_event_irq);
    0
}

pub unsafe fn riscv_cs_get_mult_shift(mult: *mut u32, shift: *mut u32) {
    *mult = riscv_clocksource.mult;
    *shift = riscv_clocksource.shift;
}

/* called directly from the low-level interrupt handler */
unsafe fn riscv_timer_interrupt(_irq: c_int, _dev_id: *mut c_void) -> irqreturn_t {
    let evdev = this_cpu_ptr(&mut riscv_clock_event);
    riscv_clock_event_stop();
    ((*evdev).event_handler)(evdev);
    IRQ_HANDLED
}

unsafe fn riscv_timer_init_common() -> c_int {
    let mut error: c_int;
    let intc_fwnode = riscv_get_intc_hwnode();
    let domain = irq_find_matching_fwnode(intc_fwnode, DOMAIN_BUS_ANY);
    if domain.is_null() {
        pr_err!("Failed to find irq_domain for INTC node [%pfwP]", intc_fwnode);
        return -ENODEV;
    }
    riscv_clock_event_irq = irq_create_mapping(domain, RV_IRQ_TIMER);
    if riscv_clock_event_irq == 0 {
        pr_err!("Failed to map timer interrupt for node [%pfwP]", intc_fwnode);
        return -ENODEV;
    }
    error = clocksource_register_hz(&mut riscv_clocksource, riscv_timebase);
    if error != 0 {
        pr_err!("RISCV timer registration failed [{}]", error);
        return error;
    }
    sched_clock_register(Some(riscv_sched_clock), 64, riscv_timebase);
    error = request_percpu_irq(riscv_clock_event_irq, Some(riscv_timer_interrupt), b"riscv-timer\0".as_ptr() as *const c_char, &mut riscv_clock_event);
    if error != 0 {
        pr_err!("registering percpu irq failed [{}]", error);
        return error;
    }
    if riscv_isa_extension_available(core::ptr::null_mut(), SSTC) {
        pr_info!("Timer interrupt in S-mode is available via sstc extension");
        RISCV_SSTC_AVAILABLE = true;
    }
    error = cpuhp_setup_state(CPUHP_AP_RISCV_TIMER_STARTING, b"clockevents/riscv/timer:starting\0".as_ptr() as *const c_char, Some(riscv_timer_starting_cpu), Some(riscv_timer_dying_cpu));
    if error != 0 { pr_err!("cpu hp setup state failed for RISCV timer [{}]", error); }
    error
}

unsafe fn riscv_timer_init_dt(n: *mut device_node) -> c_int {
    let mut hartid: c_ulong = 0;
    let error = riscv_of_processor_hartid(n, &mut hartid);
    if error < 0 { pr_warn!("Invalid hartid for node [%pOF] error = [%lu]", n, hartid); return error; }
    let cpuid = riscv_hartid_to_cpuid(hartid);
    if cpuid < 0 { pr_warn!("Invalid cpuid for hartid [%lu]", hartid); return cpuid; }
    if cpuid != smp_processor_id() { return 0; }
    let child = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), b"riscv,timer\0".as_ptr() as *const c_char);
    if !child.is_null() {
        riscv_timer_cannot_wake_cpu = of_property_read_bool(child, b"riscv,timer-cannot-wake-cpu\0".as_ptr() as *const c_char);
        of_node_put(child);
    }
    riscv_timer_init_common()
}

// TIMER_OF_DECLARE(riscv_timer, "riscv", riscv_timer_init_dt);

// CONFIG_ACPI
unsafe fn riscv_timer_acpi_init(table: *mut acpi_table_header) -> c_int {
    let rhct = table as *mut acpi_table_rhct;
    riscv_timer_cannot_wake_cpu = (*rhct).flags & ACPI_RHCT_TIMER_CANNOT_WAKEUP_CPU != 0;
    riscv_timer_init_common()
}

// TIMER_ACPI_DECLARE(aclint_mtimer, ACPI_SIG_RHCT, riscv_timer_acpi_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
