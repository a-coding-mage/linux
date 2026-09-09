// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright 2012 Simon Arlott
 */

// The following symbols are supplied by the kernel environment.

const REG_CONTROL: usize = 0x00;
const REG_COUNTER_LO: usize = 0x04;
const REG_COUNTER_HI: usize = 0x08;
const MAX_TIMER: usize = 3;
const DEFAULT_TIMER: usize = 3;

#[inline]
const fn reg_compare(n: usize) -> usize {
    0x0c + n * 4
}

#[repr(C)]
struct bcm2835_timer {
    control: *mut core::ffi::c_void,
    compare: *mut core::ffi::c_void,
    match_mask: i32,
    evt: clock_event_device,
}

static mut system_clock: *mut core::ffi::c_void = core::ptr::null_mut();

unsafe fn bcm2835_sched_read() -> u64 {
    readl_relaxed(system_clock) as u64
}

unsafe fn bcm2835_time_set_next_event(
    event: usize,
    evt_dev: *mut clock_event_device,
) -> i32 {
    let timer = container_of!(evt_dev, bcm2835_timer, evt);
    writel_relaxed(
        readl_relaxed(system_clock).wrapping_add(event as u32),
        (*timer).compare,
    );
    0
}

unsafe fn bcm2835_time_interrupt(
    _irq: i32,
    dev_id: *mut core::ffi::c_void,
) -> irqreturn_t {
    let timer = dev_id as *mut bcm2835_timer;
    let event_handler: Option<unsafe extern "C" fn(*mut clock_event_device)>;
    if readl_relaxed((*timer).control) & (*timer).match_mask as u32 != 0 {
        writel_relaxed((*timer).match_mask as u32, (*timer).control);

        event_handler = core::ptr::read_volatile(&(*timer).evt.event_handler);
        if let Some(handler) = event_handler {
            handler(&mut (*timer).evt);
        }
        IRQ_HANDLED
    } else {
        IRQ_NONE
    }
}

unsafe fn bcm2835_timer_init(node: *mut device_node) -> i32 {
    let base: *mut core::ffi::c_void;
    let mut freq: u32 = 0;
    let mut irq: i32;
    let mut ret: i32;
    let timer: *mut bcm2835_timer;

    base = of_iomap(node, 0);
    if base.is_null() {
        pr_err!("Can't remap registers\n");
        return -ENXIO;
    }

    ret = of_property_read_u32(node, "clock-frequency", &mut freq);
    if ret != 0 {
        pr_err!("Can't read clock-frequency\n");
        iounmap(base);
        return ret;
    }

    system_clock = base.add(REG_COUNTER_LO);
    sched_clock_register(bcm2835_sched_read, 32, freq);

    clocksource_mmio_init(
        base.add(REG_COUNTER_LO),
        (*node).name,
        freq,
        300,
        32,
        clocksource_mmio_readl_up,
    );

    irq = irq_of_parse_and_map(node, DEFAULT_TIMER);
    if irq <= 0 {
        pr_err!("Can't parse IRQ\n");
        ret = -EINVAL;
        iounmap(base);
        return ret;
    }

    timer = kzalloc_obj!();
    if timer.is_null() {
        ret = -ENOMEM;
        iounmap(base);
        return ret;
    }

    (*timer).control = base.add(REG_CONTROL);
    (*timer).compare = base.add(reg_compare(DEFAULT_TIMER));
    (*timer).match_mask = 1 << DEFAULT_TIMER;
    (*timer).evt.name = (*node).name;
    (*timer).evt.rating = 300;
    (*timer).evt.features = CLOCK_EVT_FEAT_ONESHOT;
    (*timer).evt.set_next_event = Some(bcm2835_time_set_next_event);
    (*timer).evt.cpumask = cpumask_of(0);

    ret = request_irq(
        irq,
        bcm2835_time_interrupt,
        IRQF_TIMER | IRQF_SHARED,
        (*node).name,
        timer as *mut core::ffi::c_void,
    );
    if ret != 0 {
        pr_err!("Can't set up timer IRQ\n");
        kfree(timer);
        iounmap(base);
        return ret;
    }

    clockevents_config_and_register(&mut (*timer).evt, freq, 0xf, 0xffffffff);
    pr_info!("bcm2835: system timer (irq = %d)\n", irq);
    0
}

// Equivalent of TIMER_OF_DECLARE(bcm2835, "brcm,bcm2835-system-timer",
//                                bcm2835_timer_init).
TIMER_OF_DECLARE!(bcm2835, "brcm,bcm2835-system-timer", bcm2835_timer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
