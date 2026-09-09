// SPDX-License-Identifier: GPL-2.0
/*
 * IXP4 timer driver
 * Copyright (C) 2019 Linus Walleij <linus.walleij@linaro.org>
 *
 * Based on arch/arm/mach-ixp4xx/common.c
 * Copyright 2002 (C) Intel Corporation
 * Copyright 2003-2004 (C) MontaVista, Software, Inc.
 * Copyright (C) Deepak Saxena <dsaxena@plexity.net>
 */

// External kernel declarations and types are supplied by other translation units.

const IXP4XX_OSTS_OFFSET: usize = 0x00;
const IXP4XX_OST1_OFFSET: usize = 0x04;
const IXP4XX_OSRT1_OFFSET: usize = 0x08;
const IXP4XX_OST2_OFFSET: usize = 0x0C;
const IXP4XX_OSRT2_OFFSET: usize = 0x10;
const IXP4XX_OSST_OFFSET: usize = 0x20;

const IXP4XX_OST_ENABLE: u32 = 0x00000001;
const IXP4XX_OST_ONE_SHOT: u32 = 0x00000002;
const IXP4XX_OST_RELOAD_MASK: u32 = 0x00000003;
const IXP4XX_OST_DISABLED: u32 = 0x00000000;
const IXP4XX_OSST_TIMER_1_PEND: u32 = 0x00000001;
const IXP4XX_OSST_TIMER_2_PEND: u32 = 0x00000002;
const IXP4XX_OSST_TIMER_TS_PEND: u32 = 0x00000004;

#[repr(C)]
struct ixp4xx_timer {
    base: *mut core::ffi::c_void,
    latch: u32,
    clkevt: clock_event_device,
    #[cfg(CONFIG_ARM)]
    delay_timer: delay_timer,
}

static mut local_ixp4xx_timer: *mut ixp4xx_timer = core::ptr::null_mut();

#[inline]
unsafe fn to_ixp4xx_timer(evt: *mut clock_event_device) -> *mut ixp4xx_timer {
    (evt as *mut u8).sub(core::mem::offset_of!(ixp4xx_timer, clkevt)) as *mut ixp4xx_timer
}

unsafe fn ixp4xx_read_timer() -> usize {
    core::ptr::read_volatile((*local_ixp4xx_timer).base.cast::<u32>().add(IXP4XX_OSTS_OFFSET / 4)) as usize
}

unsafe fn ixp4xx_read_sched_clock() -> u64 { ixp4xx_read_timer() as u64 }

unsafe fn ixp4xx_clocksource_read(_c: *mut clocksource) -> u64 { ixp4xx_read_timer() as u64 }

unsafe extern "C" fn ixp4xx_timer_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let tmr = dev_id as *mut ixp4xx_timer;
    let evt = &mut (*tmr).clkevt;
    core::ptr::write_volatile((*tmr).base.cast::<u32>().add(IXP4XX_OSST_OFFSET / 4), IXP4XX_OSST_TIMER_1_PEND);
    ((*evt).event_handler.unwrap())(evt);
    IRQ_HANDLED
}

unsafe extern "C" fn ixp4xx_set_next_event(cycles: usize, evt: *mut clock_event_device) -> i32 {
    let tmr = to_ixp4xx_timer(evt);
    let mut val = core::ptr::read_volatile((*tmr).base.cast::<u32>().add(IXP4XX_OSRT1_OFFSET / 4));
    val &= IXP4XX_OST_RELOAD_MASK;
    core::ptr::write_volatile((*tmr).base.cast::<u32>().add(IXP4XX_OSRT1_OFFSET / 4), (cycles as u32 & !IXP4XX_OST_RELOAD_MASK) | val);
    0
}

unsafe extern "C" fn ixp4xx_shutdown(evt: *mut clock_event_device) -> i32 {
    let tmr = to_ixp4xx_timer(evt);
    let mut val = core::ptr::read_volatile((*tmr).base.cast::<u32>().add(IXP4XX_OSRT1_OFFSET / 4));
    val &= !IXP4XX_OST_ENABLE;
    core::ptr::write_volatile((*tmr).base.cast::<u32>().add(IXP4XX_OSRT1_OFFSET / 4), val);
    0
}

unsafe extern "C" fn ixp4xx_set_oneshot(evt: *mut clock_event_device) -> i32 {
    let tmr = to_ixp4xx_timer(evt);
    core::ptr::write_volatile((*tmr).base.cast::<u32>().add(IXP4XX_OSRT1_OFFSET / 4), IXP4XX_OST_ENABLE | IXP4XX_OST_ONE_SHOT);
    0
}

unsafe extern "C" fn ixp4xx_set_periodic(evt: *mut clock_event_device) -> i32 {
    let tmr = to_ixp4xx_timer(evt);
    let mut val = (*tmr).latch & !IXP4XX_OST_RELOAD_MASK;
    val |= IXP4XX_OST_ENABLE;
    core::ptr::write_volatile((*tmr).base.cast::<u32>().add(IXP4XX_OSRT1_OFFSET / 4), val);
    0
}

unsafe extern "C" fn ixp4xx_resume(evt: *mut clock_event_device) -> i32 {
    let tmr = to_ixp4xx_timer(evt);
    let mut val = core::ptr::read_volatile((*tmr).base.cast::<u32>().add(IXP4XX_OSRT1_OFFSET / 4));
    val |= IXP4XX_OST_ENABLE;
    core::ptr::write_volatile((*tmr).base.cast::<u32>().add(IXP4XX_OSRT1_OFFSET / 4), val);
    0
}

unsafe fn ixp4xx_timer_register(base: *mut core::ffi::c_void, timer_irq: i32, timer_freq: u32) -> i32 {
    let tmr = kzalloc_obj::<ixp4xx_timer>();
    if tmr.is_null() { return -ENOMEM; }
    (*tmr).base = base;
    (*tmr).latch = div_round_closest(timer_freq, (IXP4XX_OST_RELOAD_MASK + 1) * HZ) * (IXP4XX_OST_RELOAD_MASK + 1);
    local_ixp4xx_timer = tmr;
    core::ptr::write_volatile(base.cast::<u32>().add(IXP4XX_OSRT1_OFFSET / 4), 0);
    core::ptr::write_volatile(base.cast::<u32>().add(IXP4XX_OSST_OFFSET / 4), IXP4XX_OSST_TIMER_1_PEND);
    core::ptr::write_volatile(base.cast::<u32>().add(IXP4XX_OSTS_OFFSET / 4), 0);
    clocksource_mmio_init(core::ptr::null_mut(), "OSTS", timer_freq, 200, 32, ixp4xx_clocksource_read);
    (*tmr).clkevt.name = "ixp4xx timer1";
    (*tmr).clkevt.features = CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT;
    (*tmr).clkevt.rating = 200;
    (*tmr).clkevt.set_state_shutdown = Some(ixp4xx_shutdown);
    (*tmr).clkevt.set_state_periodic = Some(ixp4xx_set_periodic);
    (*tmr).clkevt.set_state_oneshot = Some(ixp4xx_set_oneshot);
    (*tmr).clkevt.tick_resume = Some(ixp4xx_resume);
    (*tmr).clkevt.set_next_event = Some(ixp4xx_set_next_event);
    (*tmr).clkevt.cpumask = cpumask_of(0);
    (*tmr).clkevt.irq = timer_irq;
    let ret = request_irq(timer_irq, Some(ixp4xx_timer_interrupt), IRQF_TIMER, "IXP4XX-TIMER1", tmr.cast());
    if ret != 0 { pr_crit!("no timer IRQ\n"); return -ENODEV; }
    clockevents_config_and_register(&mut (*tmr).clkevt, timer_freq, 0xf, 0xfffffffe);
    sched_clock_register(ixp4xx_read_sched_clock, 32, timer_freq);
    #[cfg(CONFIG_ARM)] {
        (*tmr).delay_timer.read_current_timer = Some(ixp4xx_read_timer);
        (*tmr).delay_timer.freq = timer_freq;
        register_current_timer_delay(&mut (*tmr).delay_timer);
    }
    0
}

static mut ixp4xx_watchdog_device: platform_device = platform_device { name: "ixp4xx-watchdog", id: -1 };

unsafe fn ixp4xx_timer_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    (*ixp4xx_watchdog_device).dev.platform_data = (*local_ixp4xx_timer).base;
    (*ixp4xx_watchdog_device).dev.parent = dev;
    platform_device_register(&mut ixp4xx_watchdog_device)
}

static ixp4xx_timer_dt_id: [of_device_id; 2] = [
    of_device_id { compatible: "intel,ixp4xx-timer" },
    of_device_id { compatible: core::ptr::null() },
];

static mut ixp4xx_timer_driver: platform_driver = platform_driver {
    probe: Some(ixp4xx_timer_probe),
    driver: driver { name: "ixp4xx-timer", of_match_table: ixp4xx_timer_dt_id.as_ptr(), suppress_bind_attrs: true },
};

unsafe fn ixp4xx_of_timer_init(np: *mut device_node) -> i32 {
    let base = of_iomap(np, 0);
    if base.is_null() { pr_crit!("IXP4xx: can't remap timer\n"); return -ENODEV; }
    let irq = irq_of_parse_and_map(np, 0);
    if irq <= 0 { pr_err!("Can't parse IRQ\n"); iounmap(base); return -EINVAL; }
    let ret = ixp4xx_timer_register(base, irq, 66666000);
    if ret != 0 { iounmap(base); }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
