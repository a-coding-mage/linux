// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2022 Hewlett-Packard Enterprise Development Company, L.P. */

// C dependencies supplied by the surrounding kernel translation unit.

const TIMER0_FREQ: u32 = 1_000_000;
const GXP_TIMER_CNT_OFS: usize = 0x00;
const GXP_TIMESTAMP_OFS: usize = 0x08;
const GXP_TIMER_CTRL_OFS: usize = 0x14;

// TCS Stands for Timer Control/Status: these are masks to be used in
// the Timer Count Registers
const MASK_TCS_ENABLE: u8 = 0x01;
const MASK_TCS_PERIOD: u8 = 0x02;
const MASK_TCS_RELOAD: u8 = 0x04;
const MASK_TCS_TC: u8 = 0x80;

#[repr(C)]
pub struct gxp_timer {
    pub counter: *mut core::ffi::c_void,
    pub control: *mut core::ffi::c_void,
    pub evt: clock_event_device,
}

static mut gxp_timer: *mut gxp_timer = core::ptr::null_mut();
static mut system_clock: *mut core::ffi::c_void = core::ptr::null_mut();

#[repr(C)]
pub struct clock_event_device {
    pub name: *const core::ffi::c_char,
    pub rating: i32,
    pub features: u32,
    pub set_next_event: Option<unsafe extern "C" fn(usize, *mut clock_event_device) -> i32>,
    pub cpumask: *const core::ffi::c_void,
    pub event_handler: Option<unsafe extern "C" fn(*mut clock_event_device)>,
}

#[repr(C)]
pub struct device_node {
    pub name: *const core::ffi::c_char,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct device {
    pub platform_data: *mut core::ffi::c_void,
    pub parent: *mut device,
}

unsafe fn to_gxp_timer(evt_dev: *mut clock_event_device) -> *mut gxp_timer {
    (evt_dev as *mut u8).sub(core::mem::offset_of!(gxp_timer, evt)) as *mut gxp_timer
}

unsafe extern "C" fn gxp_sched_read() -> u64 {
    readl_relaxed(system_clock) as u64
}

unsafe extern "C" fn gxp_time_set_next_event(
    event: usize,
    evt_dev: *mut clock_event_device,
) -> i32 {
    let timer = to_gxp_timer(evt_dev);

    /* Stop counting and disable interrupt before updating */
    writeb_relaxed(MASK_TCS_TC, (*timer).control);
    writel_relaxed(event as u32, (*timer).counter);
    writeb_relaxed(MASK_TCS_TC | MASK_TCS_ENABLE, (*timer).control);

    0
}

unsafe extern "C" fn gxp_timer_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> i32 {
    let timer = dev_id as *mut gxp_timer;

    if (readb_relaxed((*timer).control) & MASK_TCS_TC) == 0 {
        return IRQ_NONE;
    }

    writeb_relaxed(MASK_TCS_TC, (*timer).control);

    if let Some(handler) = (*timer).evt.event_handler {
        handler(&mut (*timer).evt);
    }

    IRQ_HANDLED
}

unsafe extern "C" fn gxp_timer_init(node: *mut device_node) -> i32 {
    let mut base: *mut core::ffi::c_void;
    let mut clk: *mut clk;
    let mut freq: u32;
    let mut ret: i32;
    let mut irq: i32;

    gxp_timer = kzalloc_obj::<gxp_timer>();
    if gxp_timer.is_null() {
        ret = -ENOMEM;
        pr_err("Can't allocate gxp_timer");
        return ret;
    }

    clk = of_clk_get(node, 0);
    if is_err(clk) {
        ret = ptr_err(clk);
        pr_err("%pOFn clock not found: %d\n", node, ret);
        goto_err_free(clk, ret);
    }

    ret = clk_prepare_enable(clk);
    if ret != 0 {
        pr_err("%pOFn clock enable failed: %d\n", node, ret);
        goto_err_clk_enable(clk, ret);
    }

    base = of_iomap(node, 0);
    if base.is_null() {
        ret = -ENXIO;
        pr_err("Can't map timer base registers");
        goto_err_iomap(clk, ret);
    }

    /* Set the offsets to the clock register and timer registers */
    (*gxp_timer).counter = (base as *mut u8).add(GXP_TIMER_CNT_OFS) as *mut core::ffi::c_void;
    (*gxp_timer).control = (base as *mut u8).add(GXP_TIMER_CTRL_OFS) as *mut core::ffi::c_void;
    system_clock = (base as *mut u8).add(GXP_TIMESTAMP_OFS) as *mut core::ffi::c_void;

    (*gxp_timer).evt.name = (*node).name;
    (*gxp_timer).evt.rating = 300;
    (*gxp_timer).evt.features = CLOCK_EVT_FEAT_ONESHOT;
    (*gxp_timer).evt.set_next_event = Some(gxp_time_set_next_event);
    (*gxp_timer).evt.cpumask = cpumask_of(0);

    irq = irq_of_parse_and_map(node, 0);
    if irq <= 0 {
        ret = -EINVAL;
        pr_err("GXP Timer Can't parse IRQ %d", irq);
        goto_err_exit(base, clk, ret);
    }

    freq = clk_get_rate(clk);
    ret = clocksource_mmio_init(system_clock, (*node).name, freq, 300, 32, clocksource_mmio_readl_up);
    if ret != 0 {
        pr_err("%pOFn init clocksource failed: %d\n", node, ret);
        goto_err_exit(base, clk, ret);
    }

    sched_clock_register(gxp_sched_read, 32, freq);

    irq = irq_of_parse_and_map(node, 0);
    if irq <= 0 {
        ret = -EINVAL;
        pr_err("%pOFn Can't parse IRQ %d\n", node, ret);
        goto_err_exit(base, clk, ret);
    }

    clockevents_config_and_register(&mut (*gxp_timer).evt, TIMER0_FREQ, 0xf, 0xffffffff);
    ret = request_irq(irq, gxp_timer_interrupt, IRQF_TIMER | IRQF_SHARED, (*node).name, gxp_timer as *mut core::ffi::c_void);
    if ret != 0 {
        pr_err("%pOFn request_irq() failed: %d\n", node, ret);
        goto_err_exit(base, clk, ret);
    }

    pr_debug("gxp: system timer (irq = %d)\n", irq);
    0
}

/*
 * This probe gets called after the timer is already up and running. This will create
 * the watchdog device as a child since the registers are shared.
 */
unsafe extern "C" fn gxp_timer_probe(pdev: *mut platform_device) -> i32 {
    let gxp_watchdog_device: *mut platform_device;
    let dev = &mut (*pdev).dev as *mut device;
    let mut ret: i32;

    if gxp_timer.is_null() {
        pr_err("Gxp Timer not initialized, cannot create watchdog");
        return -ENOMEM;
    }

    gxp_watchdog_device = platform_device_alloc("gxp-wdt", -1);
    if gxp_watchdog_device.is_null() {
        pr_err("Timer failed to allocate gxp-wdt");
        return -ENOMEM;
    }

    /* Pass the base address (counter) as platform data and nothing else */
    (*gxp_watchdog_device).dev.platform_data = (*gxp_timer).counter;
    (*gxp_watchdog_device).dev.parent = dev;

    ret = platform_device_add(gxp_watchdog_device);
    if ret != 0 {
        platform_device_put(gxp_watchdog_device);
    }

    ret
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const core::ffi::c_char,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub driver: driver,
}

#[repr(C)]
pub struct driver {
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const of_device_id,
    pub suppress_bind_attrs: bool,
}

static gxp_timer_of_match: [of_device_id; 2] = [
    of_device_id { compatible: c_str!("hpe,gxp-timer") },
    of_device_id { compatible: core::ptr::null() },
];

static mut gxp_timer_driver: platform_driver = platform_driver {
    probe: Some(gxp_timer_probe),
    driver: driver {
        name: c_str!("gxp-timer"),
        of_match_table: gxp_timer_of_match.as_ptr(),
        suppress_bind_attrs: true,
    },
};

// builtin_platform_driver(gxp_timer_driver);
// TIMER_OF_DECLARE(gxp, "hpe,gxp-timer", gxp_timer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
