// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  linux/drivers/clocksource/timer-sp.c
 *
 *  Copyright (C) 1999 - 2003 ARM Limited
 *  Copyright (C) 2000 Deep Blue Solutions Ltd
 */

// Kernel dependencies supplied by other translation units.

/* Hisilicon 64-bit timer (a variant of ARM SP804). */
const HISI_TIMER_1_BASE: usize = 0x00;
const HISI_TIMER_2_BASE: usize = 0x40;
const HISI_TIMER_LOAD: usize = 0x00;
const HISI_TIMER_LOAD_H: usize = 0x04;
const HISI_TIMER_VALUE: usize = 0x08;
const HISI_TIMER_VALUE_H: usize = 0x0c;
const HISI_TIMER_CTRL: usize = 0x10;
const HISI_TIMER_INTCLR: usize = 0x14;
const HISI_TIMER_RIS: usize = 0x18;
const HISI_TIMER_MIS: usize = 0x1c;
const HISI_TIMER_BGLOAD: usize = 0x20;
const HISI_TIMER_BGLOAD_H: usize = 0x24;

static mut arm_sp804_timer: sp804_timer = sp804_timer {
    load: TIMER_LOAD,
    value: TIMER_VALUE,
    ctrl: TIMER_CTRL,
    intclr: TIMER_INTCLR,
    timer_base: [TIMER_1_BASE, TIMER_2_BASE],
    width: 32,
    ..unsafe { core::mem::zeroed() }
};

static mut hisi_sp804_timer: sp804_timer = sp804_timer {
    load: HISI_TIMER_LOAD,
    load_h: HISI_TIMER_LOAD_H,
    value: HISI_TIMER_VALUE,
    value_h: HISI_TIMER_VALUE_H,
    ctrl: HISI_TIMER_CTRL,
    intclr: HISI_TIMER_INTCLR,
    timer_base: [HISI_TIMER_1_BASE, HISI_TIMER_2_BASE],
    width: 64,
    ..unsafe { core::mem::zeroed() }
};

static mut sp804_clkevt: [sp804_clkevt; NR_TIMERS] = unsafe { core::mem::zeroed() };

unsafe fn sp804_get_clock_rate(mut clk: *mut clk, name: *const core::ffi::c_char) -> i64 {
    let err: i32;
    if clk.is_null() {
        clk = clk_get_sys(b"sp804\0".as_ptr() as _, name);
    }
    if IS_ERR(clk) {
        pr_err(b"%s clock not found: %ld\n\0".as_ptr() as _, name, PTR_ERR(clk));
        return PTR_ERR(clk);
    }
    err = clk_prepare_enable(clk);
    if err != 0 {
        pr_err(b"clock failed to enable: %d\n\0".as_ptr() as _, err);
        clk_put(clk);
        return err as i64;
    }
    clk_get_rate(clk)
}

unsafe fn sp804_clkevt_get(base: *mut core::ffi::c_void) -> *mut sp804_clkevt {
    for i in 0..NR_TIMERS {
        if sp804_clkevt[i].base == base { return &mut sp804_clkevt[i]; }
    }
    WARN_ON(1);
    core::ptr::null_mut()
}

static mut sched_clkevt: *mut sp804_clkevt = core::ptr::null_mut();

unsafe extern "C" fn sp804_read() -> u64 {
    !(readl_relaxed((*sched_clkevt).value) as u64)
}

#[cfg(CONFIG_ARM)]
static mut delay: delay_timer = unsafe { core::mem::zeroed() };
#[cfg(CONFIG_ARM)]
static mut delay_clkevt: *mut sp804_clkevt = core::ptr::null_mut();

#[cfg(CONFIG_ARM)]
unsafe extern "C" fn sp804_read_delay_timer_read() -> u32 {
    !(readl_relaxed((*delay_clkevt).value))
}

#[cfg(CONFIG_ARM)]
unsafe fn sp804_register_delay_timer(clk: *mut sp804_clkevt, freq: i32) {
    delay_clkevt = clk;
    delay.freq = freq;
    delay.read_current_timer = Some(sp804_read_delay_timer_read);
    register_current_timer_delay(&mut delay);
}

#[cfg(not(CONFIG_ARM))]
unsafe fn sp804_register_delay_timer(_clk: *mut sp804_clkevt, _freq: i32) {}

unsafe fn sp804_clocksource_and_sched_clock_init(base: *mut core::ffi::c_void, name: *const core::ffi::c_char, clk: *mut clk, use_sched_clock: i32) -> i32 {
    let rate = sp804_get_clock_rate(clk, name);
    if rate < 0 { return -EINVAL; }
    let clkevt = sp804_clkevt_get(base);
    writel(0, (*clkevt).ctrl);
    writel(0xffff_ffff, (*clkevt).load);
    writel(0xffff_ffff, (*clkevt).value);
    if (*clkevt).width == 64 {
        writel(0xffff_ffff, (*clkevt).load_h);
        writel(0xffff_ffff, (*clkevt).value_h);
    }
    writel(TIMER_CTRL_32BIT | TIMER_CTRL_ENABLE | TIMER_CTRL_PERIODIC, (*clkevt).ctrl);
    clocksource_mmio_init((*clkevt).value, name, rate as u32, 200, 32, Some(clocksource_mmio_readl_down));
    sp804_register_delay_timer(clkevt, rate as i32);
    if use_sched_clock != 0 {
        sched_clkevt = clkevt;
        sched_clock_register(Some(sp804_read), 32, rate as u32);
    }
    0
}

static mut common_clkevt: *mut sp804_clkevt = core::ptr::null_mut();

unsafe extern "C" fn sp804_timer_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let evt = dev_id as *mut clock_event_device;
    writel(1, (*common_clkevt).intclr);
    ((*evt).event_handler.unwrap())(evt);
    IRQ_HANDLED
}

unsafe fn evt_timer_shutdown(_evt: *mut clock_event_device) { writel(0, (*common_clkevt).ctrl); }
unsafe extern "C" fn sp804_shutdown(evt: *mut clock_event_device) -> i32 { evt_timer_shutdown(evt); 0 }

unsafe extern "C" fn sp804_set_periodic(evt: *mut clock_event_device) -> i32 {
    let ctrl = TIMER_CTRL_32BIT | TIMER_CTRL_IE | TIMER_CTRL_PERIODIC | TIMER_CTRL_ENABLE;
    evt_timer_shutdown(evt);
    writel((*common_clkevt).reload, (*common_clkevt).load);
    writel(ctrl, (*common_clkevt).ctrl);
    0
}

unsafe extern "C" fn sp804_set_next_event(next: u32, _evt: *mut clock_event_device) -> i32 {
    let ctrl = TIMER_CTRL_32BIT | TIMER_CTRL_IE | TIMER_CTRL_ONESHOT | TIMER_CTRL_ENABLE;
    writel(next, (*common_clkevt).load);
    writel(ctrl, (*common_clkevt).ctrl);
    0
}

static mut sp804_clockevent: clock_event_device = clock_event_device {
    features: CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT | CLOCK_EVT_FEAT_DYNIRQ,
    set_state_shutdown: Some(sp804_shutdown),
    set_state_periodic: Some(sp804_set_periodic),
    set_state_oneshot: Some(sp804_shutdown),
    tick_resume: Some(sp804_shutdown),
    set_next_event: Some(sp804_set_next_event),
    rating: 300,
    ..unsafe { core::mem::zeroed() }
};

unsafe fn sp804_clockevents_init(base: *mut core::ffi::c_void, irq: u32, clk: *mut clk, name: *const core::ffi::c_char) -> i32 {
    let evt = &mut sp804_clockevent;
    let rate = sp804_get_clock_rate(clk, name);
    if rate < 0 { return -EINVAL; }
    common_clkevt = sp804_clkevt_get(base);
    (*common_clkevt).reload = DIV_ROUND_CLOSEST(rate as u32, HZ);
    (*evt).name = name;
    (*evt).irq = irq;
    (*evt).cpumask = cpu_possible_mask;
    writel(0, (*common_clkevt).ctrl);
    if request_irq(irq, Some(sp804_timer_interrupt), IRQF_TIMER | IRQF_IRQPOLL, b"timer\0".as_ptr() as _, &mut sp804_clockevent as _) != 0 { pr_err(b"request_irq() failed\n\0".as_ptr() as _); }
    clockevents_config_and_register(evt, rate as u32, 0xf, 0xffff_ffff);
    0
}

unsafe fn sp804_clkevt_init(timer: *mut sp804_timer, base: *mut core::ffi::c_void) {
    for i in 0..NR_TIMERS {
        let timer_base = (base as *mut u8).add((*timer).timer_base[i]) as _;
        let clkevt = &mut sp804_clkevt[i];
        clkevt.base = timer_base;
        clkevt.load = (timer_base as *mut u8).add((*timer).load) as _;
        clkevt.load_h = (timer_base as *mut u8).add((*timer).load_h) as _;
        clkevt.value = (timer_base as *mut u8).add((*timer).value) as _;
        clkevt.value_h = (timer_base as *mut u8).add((*timer).value_h) as _;
        clkevt.ctrl = (timer_base as *mut u8).add((*timer).ctrl) as _;
        clkevt.intclr = (timer_base as *mut u8).add((*timer).intclr) as _;
        clkevt.width = (*timer).width;
    }
}

unsafe fn sp804_of_init(np: *mut device_node, timer: *mut sp804_timer) -> i32 {
    static mut initialized: bool = false;
    let mut base: *mut core::ffi::c_void;
    let mut irq_num = 0u32;
    let name = of_get_property(np, b"compatible\0".as_ptr() as _, core::ptr::null_mut());
    if initialized { pr_debug(b"%pOF: skipping further SP804 timer device\n\0".as_ptr() as _, np); return 0; }
    base = of_iomap(np, 0); if base.is_null() { return -ENXIO; }
    let timer1_base = (base as *mut u8).add((*timer).timer_base[0]) as *mut core::ffi::c_void;
    let timer2_base = (base as *mut u8).add((*timer).timer_base[1]) as *mut core::ffi::c_void;
    writel(0, (timer1_base as *mut u8).add((*timer).ctrl) as _); writel(0, (timer2_base as *mut u8).add((*timer).ctrl) as _);
    let clk1 = { let c = of_clk_get(np, 0); if IS_ERR(c) { core::ptr::null_mut() } else { c } };
    let clk2 = if of_clk_get_parent_count(np) == 3 { let c = of_clk_get(np, 1); if IS_ERR(c) { pr_err(b"%pOFn clock not found: %d\n\0".as_ptr() as _, np, PTR_ERR(c)); core::ptr::null_mut() } else { c } } else { clk1 };
    let irq = irq_of_parse_and_map(np, 0); if irq <= 0 { iounmap(base); return -EINVAL; }
    sp804_clkevt_init(timer, base);
    of_property_read_u32(np, b"arm,sp804-has-irq\0".as_ptr() as _, &mut irq_num);
    let ret = if irq_num == 2 { sp804_clockevents_init(timer2_base, irq as u32, clk2, name); if ret != 0 { ret } else { sp804_clocksource_and_sched_clock_init(timer1_base, name, clk1, 1) } } else { sp804_clockevents_init(timer1_base, irq as u32, clk1, name); if ret != 0 { ret } else { sp804_clocksource_and_sched_clock_init(timer2_base, name, clk2, 1) } };
    if ret != 0 { iounmap(base); return ret; } initialized = true; 0
}

unsafe extern "C" fn arm_sp804_of_init(np: *mut device_node) -> i32 { sp804_of_init(np, &mut arm_sp804_timer) }
TIMER_OF_DECLARE!(sp804, "arm,sp804", arm_sp804_of_init);
unsafe extern "C" fn hisi_sp804_of_init(np: *mut device_node) -> i32 { sp804_of_init(np, &mut hisi_sp804_timer) }
TIMER_OF_DECLARE!(hisi_sp804, "hisilicon,sp804", hisi_sp804_of_init);

unsafe extern "C" fn integrator_cp_of_init(np: *mut device_node) -> i32 {
    static mut init_count: i32 = 0;
    let base = of_iomap(np, 0); if base.is_null() { pr_err(b"Failed to iomap\n\0".as_ptr() as _); return -ENXIO; }
    let name = of_get_property(np, b"compatible\0".as_ptr() as _, core::ptr::null_mut());
    let clk = of_clk_get(np, 0); if IS_ERR(clk) { pr_err(b"Failed to get clock\n\0".as_ptr() as _); return PTR_ERR(clk); }
    writel(0, (base as *mut u8).add(arm_sp804_timer.ctrl) as _);
    if init_count == 2 || !of_device_is_available(np) { iounmap(base); return -EINVAL; }
    sp804_clkevt_init(&mut arm_sp804_timer, base);
    let ret = if init_count == 0 { sp804_clocksource_and_sched_clock_init(base, name, clk, 0) } else { let irq = irq_of_parse_and_map(np, 0); if irq <= 0 { iounmap(base); return -EINVAL; } sp804_clockevents_init(base, irq as u32, clk, name) };
    if ret != 0 { iounmap(base); return ret; } init_count += 1; 0
}
TIMER_OF_DECLARE!(intcp, "arm,integrator-cp-timer", integrator_cp_of_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
