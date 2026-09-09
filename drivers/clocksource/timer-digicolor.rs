// SPDX-License-Identifier: GPL-2.0
/*
 * Conexant Digicolor timer driver
 *
 * Author: Baruch Siach <baruch@tkos.co.il>
 *
 * Copyright (C) 2014 Paradox Innovation Ltd.
 *
 * Based on:
 *	Allwinner SoCs hstimer driver
 *
 * Copyright (C) 2013 Maxime Ripard
 *
 * Maxime Ripard <maxime.ripard@free-electrons.com>
 */

/*
 * Conexant Digicolor SoCs have 8 configurable timers, named from "Timer A" to
 * "Timer H". Timer A is the only one with watchdog support, so it is dedicated
 * to the watchdog driver. This driver uses Timer B for sched_clock(), and
 * Timer C for clockevents.
 */

// #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// Linux kernel headers and symbols are supplied by the surrounding translation.

const TIMER_A: i32 = 0;
const TIMER_B: i32 = 1;
const TIMER_C: i32 = 2;
const TIMER_D: i32 = 3;
const TIMER_E: i32 = 4;
const TIMER_F: i32 = 5;
const TIMER_G: i32 = 6;
const TIMER_H: i32 = 7;

const fn control(t: i32) -> usize { (t * 8) as usize }
const fn count(t: i32) -> usize { (t * 8 + 4) as usize }

const CONTROL_DISABLE: u8 = 0;
const CONTROL_ENABLE: u8 = 1 << 0;
const fn control_mode(m: u8) -> u8 { m << 4 }
const CONTROL_MODE_ONESHOT: u8 = control_mode(1);
const CONTROL_MODE_PERIODIC: u8 = control_mode(2);

#[repr(C)]
struct digicolor_timer {
    ce: clock_event_device,
    base: *mut core::ffi::c_void,
    ticks_per_jiffy: u32,
    timer_id: i32, /* one of TIMER_* */
}

unsafe fn dc_timer(ce: *mut clock_event_device) -> *mut digicolor_timer {
    // Equivalent to container_of(ce, struct digicolor_timer, ce).
    (ce as *mut u8).sub(core::mem::offset_of!(digicolor_timer, ce)) as *mut digicolor_timer
}

unsafe fn dc_timer_disable(ce: *mut clock_event_device) {
    let dt = dc_timer(ce);
    writeb(CONTROL_DISABLE, (*dt).base.add(control((*dt).timer_id)));
}

unsafe fn dc_timer_enable(ce: *mut clock_event_device, mode: u8) {
    let dt = dc_timer(ce);
    writeb(CONTROL_ENABLE | mode, (*dt).base.add(control((*dt).timer_id)));
}

unsafe fn dc_timer_set_count(ce: *mut clock_event_device, count_value: usize) {
    let dt = dc_timer(ce);
    writel(count_value as u32, (*dt).base.add(count((*dt).timer_id)));
}

unsafe extern "C" fn digicolor_clkevt_shutdown(ce: *mut clock_event_device) -> i32 {
    dc_timer_disable(ce);
    0
}

unsafe extern "C" fn digicolor_clkevt_set_oneshot(ce: *mut clock_event_device) -> i32 {
    dc_timer_disable(ce);
    dc_timer_enable(ce, CONTROL_MODE_ONESHOT);
    0
}

unsafe extern "C" fn digicolor_clkevt_set_periodic(ce: *mut clock_event_device) -> i32 {
    let dt = dc_timer(ce);
    dc_timer_disable(ce);
    dc_timer_set_count(ce, (*dt).ticks_per_jiffy as usize);
    dc_timer_enable(ce, CONTROL_MODE_PERIODIC);
    0
}

unsafe extern "C" fn digicolor_clkevt_next_event(
    evt: usize,
    ce: *mut clock_event_device,
) -> i32 {
    dc_timer_disable(ce);
    dc_timer_set_count(ce, evt);
    dc_timer_enable(ce, CONTROL_MODE_ONESHOT);
    0
}

static mut dc_timer_dev: digicolor_timer = digicolor_timer {
    ce: clock_event_device {
        name: "digicolor_tick" as *const str,
        rating: 340,
        features: CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT,
        set_state_shutdown: Some(digicolor_clkevt_shutdown),
        set_state_periodic: Some(digicolor_clkevt_set_periodic),
        set_state_oneshot: Some(digicolor_clkevt_set_oneshot),
        tick_resume: Some(digicolor_clkevt_shutdown),
        set_next_event: Some(digicolor_clkevt_next_event),
        ..unsafe { core::mem::zeroed() }
    },
    base: core::ptr::null_mut(),
    ticks_per_jiffy: 0,
    timer_id: TIMER_C,
};

unsafe extern "C" fn digicolor_timer_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let evt = dev_id as *mut clock_event_device;
    ((*evt).event_handler)(evt);
    IRQ_HANDLED
}

unsafe extern "C" fn digicolor_timer_sched_read() -> u64 {
    !(readl(dc_timer_dev.base.add(count(TIMER_B)))) as u64
}

unsafe extern "C" fn digicolor_timer_init(node: *mut device_node) -> i32 {
    let mut rate: usize;
    let clk: *mut clk;
    let mut ret: i32;
    let irq: i32;

    /* timer registers are shared with the watchdog timer; don't map exclusively */
    dc_timer_dev.base = of_iomap(node, 0);
    if dc_timer_dev.base.is_null() { pr_err("Can't map registers\n"); return -ENXIO; }

    irq = irq_of_parse_and_map(node, dc_timer_dev.timer_id);
    if irq <= 0 { pr_err("Can't parse IRQ\n"); return -EINVAL; }

    clk = of_clk_get(node, 0);
    if IS_ERR(clk) { pr_err("Can't get timer clock\n"); return PTR_ERR(clk); }
    clk_prepare_enable(clk);
    rate = clk_get_rate(clk);
    dc_timer_dev.ticks_per_jiffy = div_round_up(rate, HZ);

    writeb(CONTROL_DISABLE, dc_timer_dev.base.add(control(TIMER_B)));
    writel(u32::MAX, dc_timer_dev.base.add(count(TIMER_B)));
    writeb(CONTROL_ENABLE, dc_timer_dev.base.add(control(TIMER_B)));

    sched_clock_register(digicolor_timer_sched_read, 32, rate);
    clocksource_mmio_init(dc_timer_dev.base.add(count(TIMER_B)), (*node).name, rate, 340, 32, clocksource_mmio_readl_down);

    ret = request_irq(irq, digicolor_timer_interrupt, IRQF_TIMER | IRQF_IRQPOLL, "digicolor_timerC", &mut dc_timer_dev.ce as *mut _ as *mut _);
    if ret != 0 { pr_warn!("request of timer irq %d failed (%d)\n", irq, ret); return ret; }

    dc_timer_dev.ce.cpumask = cpu_possible_mask;
    dc_timer_dev.ce.irq = irq;
    clockevents_config_and_register(&mut dc_timer_dev.ce, rate, 0, 0xffff_ffff);
    0
}

// TIMER_OF_DECLARE(conexant_digicolor, "cnxt,cx92755-timer", digicolor_timer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
