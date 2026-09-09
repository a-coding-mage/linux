// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020 Western Digital Corporation or its affiliates.
 *
 * Most of the M-mode (i.e. NoMMU) RISC-V systems usually have a
 * CLINT MMIO timer device.
 */

// C dependencies supplied by the surrounding kernel translation are intentionally external.

const CLINT_IPI_OFF: usize = 0;
const CLINT_TIMER_CMP_OFF: usize = 0x4000;
const CLINT_TIMER_VAL_OFF: usize = 0xbff8;

/* CLINT manages IPI and Timer for RISC-V M-mode */
static mut clint_ipi_base: *mut u32 = core::ptr::null_mut();
static mut clint_ipi_irq: u32 = 0;
static mut clint_timer_cmp: *mut u64 = core::ptr::null_mut();
static mut clint_timer_val: *mut u64 = core::ptr::null_mut();
static mut clint_timer_freq: usize = 0;
static mut clint_timer_irq: u32 = 0;

#[cfg(CONFIG_RISCV_M_MODE)]
#[no_mangle]
pub static mut clint_time_val: *mut u64 = core::ptr::null_mut();

#[cfg(CONFIG_SMP)]
unsafe fn clint_send_ipi(cpu: u32) {
    writel(1, clint_ipi_base.add(cpuid_to_hartid_map(cpu) as usize));
}

#[cfg(CONFIG_SMP)]
unsafe fn clint_clear_ipi() {
    writel(0, clint_ipi_base.add(cpuid_to_hartid_map(smp_processor_id()) as usize));
}

#[cfg(CONFIG_SMP)]
unsafe fn clint_ipi_interrupt(desc: *mut irq_desc) {
    let chip = irq_desc_get_chip(desc);
    chained_irq_enter(chip, desc);
    clint_clear_ipi();
    ipi_mux_process();
    chained_irq_exit(chip, desc);
}

#[cfg(target_pointer_width = "64")]
unsafe fn clint_get_cycles() -> u64 {
    core::ptr::read_volatile(clint_timer_val)
}

#[cfg(target_pointer_width = "32")]
unsafe fn clint_get_cycles() -> u32 {
    core::ptr::read_volatile(clint_timer_val as *const u32)
}

#[cfg(target_pointer_width = "32")]
unsafe fn clint_get_cycles_hi() -> u32 {
    core::ptr::read_volatile((clint_timer_val as *const u32).add(1))
}

#[cfg(target_pointer_width = "64")]
unsafe fn clint_get_cycles64() -> u64 { clint_get_cycles() }

#[cfg(target_pointer_width = "32")]
unsafe fn clint_get_cycles64() -> u64 {
    let (mut hi, mut lo): (u32, u32);
    loop {
        hi = clint_get_cycles_hi();
        lo = clint_get_cycles();
        if hi == clint_get_cycles_hi() { break; }
    }
    ((hi as u64) << 32) | lo as u64
}

unsafe fn clint_rdtime(_cs: *mut clocksource) -> u64 { clint_get_cycles64() }

static mut clint_clocksource: clocksource = clocksource {
    name: "clint_clocksource",
    rating: 300,
    mask: CLOCKSOURCE_MASK(64),
    flags: CLOCK_SOURCE_IS_CONTINUOUS,
    read: clint_rdtime,
};

unsafe fn clint_clock_next_event(delta: usize, _ce: *mut clock_event_device) -> i32 {
    let r = clint_timer_cmp.add(cpuid_to_hartid_map(smp_processor_id()) as usize);
    csr_set(CSR_IE, IE_TIE);
    core::ptr::write_volatile(r, clint_get_cycles64().wrapping_add(delta as u64));
    0
}

static mut clint_clock_event: clock_event_device = clock_event_device {
    name: "clint_clockevent",
    features: CLOCK_EVT_FEAT_ONESHOT,
    rating: 100,
    set_next_event: clint_clock_next_event,
    ..clock_event_device::zeroed()
};

unsafe fn clint_timer_starting_cpu(cpu: u32) -> i32 {
    let ce = per_cpu_ptr(&mut clint_clock_event, cpu);
    (*ce).cpumask = cpumask_of(cpu);
    clockevents_config_and_register(ce, clint_timer_freq, 100, usize::MAX);
    enable_percpu_irq(clint_timer_irq, irq_get_trigger_type(clint_timer_irq));
    enable_percpu_irq(clint_ipi_irq, irq_get_trigger_type(clint_ipi_irq));
    0
}

unsafe fn clint_timer_dying_cpu(_cpu: u32) -> i32 {
    disable_percpu_irq(clint_timer_irq);
    /*
     * Don't disable IPI when CPU goes offline because
     * the masking/unmasking of virtual IPIs is done
     * via generic IPI-Mux
     */
    0
}

unsafe fn clint_timer_interrupt(_irq: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let evdev = this_cpu_ptr(&mut clint_clock_event);
    csr_clear(CSR_IE, IE_TIE);
    ((*evdev).event_handler)(evdev);
    IRQ_HANDLED
}

unsafe fn clint_timer_init_dt(np: *mut device_node) -> i32 {
    let mut rc: i32;
    let mut i: u32;
    let nr_irqs: u32;
    let base: *mut core::ffi::c_void;
    let mut oirq: of_phandle_args = core::mem::zeroed();

    /* Ensure that CLINT device interrupts are either RV_IRQ_TIMER or RV_IRQ_SOFT. */
    nr_irqs = of_irq_count(np);
    i = 0;
    while i < nr_irqs {
        if of_irq_parse_one(np, i, &mut oirq) != 0 {
            pr_err("%pOFP: failed to parse irq %d.\n", np, i);
            i += 1;
            continue;
        }
        if oirq.args_count != 1 ||
           (oirq.args[0] != RV_IRQ_TIMER && oirq.args[0] != RV_IRQ_SOFT) {
            pr_err("%pOFP: invalid irq %d (hwirq %d)\n", np, i, oirq.args[0]);
            return -ENODEV;
        }
        if clint_ipi_irq == 0 && oirq.args[0] == RV_IRQ_SOFT && !irq_find_host(oirq.np).is_null() {
            clint_ipi_irq = irq_of_parse_and_map(np, i);
        }
        if clint_timer_irq == 0 && oirq.args[0] == RV_IRQ_TIMER && !irq_find_host(oirq.np).is_null() {
            clint_timer_irq = irq_of_parse_and_map(np, i);
        }
        i += 1;
    }
    if clint_ipi_irq == 0 || clint_timer_irq == 0 {
        pr_err("%pOFP: ipi/timer irq not found\n", np);
        return -ENODEV;
    }
    base = of_iomap(np, 0);
    if base.is_null() {
        pr_err("%pOFP: could not map registers\n", np);
        return -ENODEV;
    }
    clint_ipi_base = (base as *mut u8).add(CLINT_IPI_OFF) as *mut u32;
    clint_timer_cmp = (base as *mut u8).add(CLINT_TIMER_CMP_OFF) as *mut u64;
    clint_timer_val = (base as *mut u8).add(CLINT_TIMER_VAL_OFF) as *mut u64;
    clint_timer_freq = riscv_timebase;

    #[cfg(CONFIG_RISCV_M_MODE)]
    { clint_time_val = clint_timer_val; }

    pr_info("%pOFP: timer running at %ld Hz\n", np, clint_timer_freq);
    rc = clocksource_register_hz(&mut clint_clocksource, clint_timer_freq);
    if rc != 0 { pr_err("%pOFP: clocksource register failed [%d]\n", np, rc); iounmap(base); return rc; }
    sched_clock_register(clint_get_cycles64, 64, clint_timer_freq);
    rc = request_percpu_irq(clint_timer_irq, clint_timer_interrupt, "clint-timer", &mut clint_clock_event);
    if rc != 0 { pr_err("registering percpu irq failed [%d]\n", rc); iounmap(base); return rc; }

    #[cfg(CONFIG_SMP)]
    {
        rc = ipi_mux_create(BITS_PER_BYTE, clint_send_ipi);
        if rc <= 0 { pr_err("unable to create muxed IPIs\n"); return if rc < 0 { rc } else { -ENODEV }; }
        irq_set_chained_handler(clint_ipi_irq, clint_ipi_interrupt);
        riscv_ipi_set_virq_range(rc, BITS_PER_BYTE);
        clint_clear_ipi();
    }
    rc = cpuhp_setup_state(CPUHP_AP_CLINT_TIMER_STARTING, "clockevents/clint/timer:starting", clint_timer_starting_cpu, clint_timer_dying_cpu);
    if rc != 0 { pr_err("%pOFP: cpuhp setup state failed [%d]\n", np, rc); free_percpu_irq(clint_timer_irq, &mut clint_clock_event); iounmap(base); return rc; }
    0
}

// TIMER_OF_DECLARE(clint_timer, "riscv,clint0", clint_timer_init_dt);
// TIMER_OF_DECLARE(clint_timer1, "sifive,clint0", clint_timer_init_dt);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
