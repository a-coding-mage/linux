// SPDX-License-Identifier: GPL-2.0
/*
 * J-Core SoC PIT/clocksource driver
 *
 * Copyright (C) 2015-2016 Smart Energy Instruments, Inc.
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

const PIT_IRQ_SHIFT: u32 = 12;
const PIT_PRIO_SHIFT: u32 = 20;
const PIT_ENABLE_SHIFT: u32 = 26;
const PIT_PRIO_MASK: u32 = 0xf;

const REG_PITEN: usize = 0x00;
const REG_THROT: usize = 0x10;
const REG_COUNT: usize = 0x14;
const REG_BUSPD: usize = 0x18;
const REG_SECHI: usize = 0x20;
const REG_SECLO: usize = 0x24;
const REG_NSEC: usize = 0x28;

#[repr(C)]
struct jcore_pit {
    ced: clock_event_device,
    base: *mut core::ffi::c_void,
    periodic_delta: c_ulong,
    enable_val: u32,
}

static mut jcore_pit_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut jcore_pit_percpu: *mut jcore_pit = core::ptr::null_mut();

unsafe fn jcore_sched_clock_read() -> u64 {
    let mut seclo: u32;
    let mut nsec: u32;
    let mut seclo0: u32;
    let base = jcore_pit_base;

    seclo = readl(base.add(REG_SECLO));
    loop {
        seclo0 = seclo;
        nsec = readl(base.add(REG_NSEC));
        seclo = readl(base.add(REG_SECLO));
        if seclo0 == seclo {
            break;
        }
    }

    seclo as u64 * NSEC_PER_SEC as u64 + nsec as u64
}

unsafe fn jcore_clocksource_read(_cs: *mut clocksource) -> u64 {
    jcore_sched_clock_read()
}

unsafe fn jcore_pit_disable(pit: *mut jcore_pit) -> i32 {
    writel(0, (*pit).base.add(REG_PITEN));
    0
}

unsafe fn jcore_pit_set(delta: c_ulong, pit: *mut jcore_pit) -> i32 {
    jcore_pit_disable(pit);
    writel(delta as u32, (*pit).base.add(REG_THROT));
    writel((*pit).enable_val, (*pit).base.add(REG_PITEN));
    0
}

unsafe fn jcore_pit_set_state_shutdown(ced: *mut clock_event_device) -> i32 {
    let pit = container_of!(ced, jcore_pit, ced);
    jcore_pit_disable(pit)
}

unsafe fn jcore_pit_set_state_oneshot(ced: *mut clock_event_device) -> i32 {
    let pit = container_of!(ced, jcore_pit, ced);
    jcore_pit_disable(pit)
}

unsafe fn jcore_pit_set_state_periodic(ced: *mut clock_event_device) -> i32 {
    let pit = container_of!(ced, jcore_pit, ced);
    jcore_pit_set((*pit).periodic_delta, pit)
}

unsafe fn jcore_pit_set_next_event(delta: c_ulong, ced: *mut clock_event_device) -> i32 {
    let pit = container_of!(ced, jcore_pit, ced);
    jcore_pit_set(delta, pit)
}

unsafe fn jcore_pit_local_init(cpu: u32) -> i32 {
    let pit = this_cpu_ptr(jcore_pit_percpu);
    let buspd = readl((*pit).base.add(REG_BUSPD));
    let freq = div_round_closest(NSEC_PER_SEC, buspd);
    (*pit).periodic_delta = div_round_closest(NSEC_PER_SEC, HZ * buspd);

    pr_info!("Local J-Core PIT init on cpu %u\n", cpu);
    clockevents_config_and_register(&mut (*pit).ced, freq, 1, ULONG_MAX);
    enable_percpu_irq((*pit).ced.irq, IRQ_TYPE_NONE);
    0
}

unsafe fn jcore_pit_local_teardown(cpu: u32) -> i32 {
    let pit = this_cpu_ptr(jcore_pit_percpu);

    pr_info!("Local J-Core PIT teardown on cpu %u\n", cpu);
    disable_percpu_irq((*pit).ced.irq);
    0
}

unsafe fn jcore_timer_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let pit = dev_id as *mut jcore_pit;

    if clockevent_state_oneshot(&mut (*pit).ced) {
        jcore_pit_disable(pit);
    }
    ((*pit).ced.event_handler)(&mut (*pit).ced);
    IRQ_HANDLED
}

unsafe fn jcore_pit_init(node: *mut device_node) -> i32 {
    let mut err: i32;
    let mut pit_irq: u32;
    let mut cpu: u32;
    let hwirq: c_ulong;
    let irqprio: u32;
    let enable_val: u32;

    jcore_pit_base = of_iomap(node, 0);
    if jcore_pit_base.is_null() {
        pr_err!("Error: Cannot map base address for J-Core PIT\n");
        return -ENXIO;
    }

    pit_irq = irq_of_parse_and_map(node, 0);
    if pit_irq == 0 {
        pr_err!("Error: J-Core PIT has no IRQ\n");
        return -ENXIO;
    }

    pr_info!("Initializing J-Core PIT at %p IRQ %d\n", jcore_pit_base, pit_irq);
    err = clocksource_mmio_init(jcore_pit_base, "jcore_pit_cs", NSEC_PER_SEC, 400, 32, jcore_clocksource_read);
    if err != 0 {
        pr_err!("Error registering clocksource device: %d\n", err);
        return err;
    }
    sched_clock_register(jcore_sched_clock_read, 32, NSEC_PER_SEC);

    jcore_pit_percpu = alloc_percpu::<jcore_pit>();
    if jcore_pit_percpu.is_null() {
        pr_err!("Failed to allocate memory for clock event device\n");
        return -ENOMEM;
    }
    irq_set_percpu_devid(pit_irq);
    err = request_percpu_irq(pit_irq, jcore_timer_interrupt, "jcore_pit", jcore_pit_percpu);
    if err != 0 {
        pr_err!("pit irq request failed: %d\n", err);
        free_percpu(jcore_pit_percpu);
        return err;
    }

    // The PIT enable-register layout and AIC1/AIC2 programming details are
    // preserved from the original source comments.
    hwirq = (*irq_get_irq_data(pit_irq)).hwirq;
    irqprio = ((hwirq >> 2) as u32) & PIT_PRIO_MASK;
    enable_val = (1u32 << PIT_ENABLE_SHIFT)
        | ((hwirq as u32) << PIT_IRQ_SHIFT)
        | (irqprio << PIT_PRIO_SHIFT);

    for_each_present_cpu!(cpu, {
        let pit = per_cpu_ptr(jcore_pit_percpu, cpu);
        (*pit).base = of_iomap(node, cpu);
        if (*pit).base.is_null() {
            pr_err!("Unable to map PIT for cpu %u\n", cpu);
            continue;
        }
        (*pit).ced.name = "jcore_pit";
        (*pit).ced.features = CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT | CLOCK_EVT_FEAT_PERCPU;
        (*pit).ced.cpumask = cpumask_of(cpu);
        (*pit).ced.rating = 400;
        (*pit).ced.irq = pit_irq;
        (*pit).ced.set_state_shutdown = Some(jcore_pit_set_state_shutdown);
        (*pit).ced.set_state_periodic = Some(jcore_pit_set_state_periodic);
        (*pit).ced.set_state_oneshot = Some(jcore_pit_set_state_oneshot);
        (*pit).ced.set_next_event = Some(jcore_pit_set_next_event);
        (*pit).enable_val = enable_val;
    });

    cpuhp_setup_state!(CPUHP_AP_JCORE_TIMER_STARTING, "clockevents/jcore:starting", jcore_pit_local_init, jcore_pit_local_teardown);
    0
}

TIMER_OF_DECLARE!(jcore_pit, "jcore,pit", jcore_pit_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
