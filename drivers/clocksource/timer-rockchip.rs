// SPDX-License-Identifier: GPL-2.0-only
/*
 * Rockchip timer support
 *
 * Copyright (C) Daniel Lezcano <daniel.lezcano@linaro.org>
 */

// Kernel dependencies supplied by other translation units.
use core::ffi::c_void;

const TIMER_NAME: &[u8] = b"rk_timer\0";

const TIMER_LOAD_COUNT0: usize = 0x00;
const TIMER_LOAD_COUNT1: usize = 0x04;
const TIMER_CURRENT_VALUE0: usize = 0x08;
const TIMER_CURRENT_VALUE1: usize = 0x0C;
const TIMER_CONTROL_REG3288: usize = 0x10;
const TIMER_CONTROL_REG3399: usize = 0x1c;
const TIMER_INT_STATUS: usize = 0x18;

const TIMER_DISABLE: u32 = 0x0;
const TIMER_ENABLE: u32 = 0x1;
const TIMER_MODE_FREE_RUNNING: u32 = 0 << 1;
const TIMER_MODE_USER_DEFINED_COUNT: u32 = 1 << 1;
const TIMER_INT_UNMASK: u32 = 1 << 2;

#[repr(C)]
struct RkTimer {
    base: *mut c_void,
    ctrl: *mut c_void,
    clk: *mut Clk,
    pclk: *mut Clk,
    freq: u32,
    irq: i32,
}

#[repr(C)]
struct RkClkevt {
    ce: ClockEventDevice,
    timer: RkTimer,
}

static mut RK_CLKEVT: *mut RkClkevt = core::ptr::null_mut();
static mut RK_CLKSRC: *mut RkTimer = core::ptr::null_mut();

unsafe fn rk_timer(ce: *mut ClockEventDevice) -> *mut RkTimer {
    let clkevt = (ce as *mut u8).sub(core::mem::offset_of!(RkClkevt, ce)) as *mut RkClkevt;
    &mut (*clkevt).timer
}

unsafe fn rk_timer_disable(timer: *mut RkTimer) {
    writel_relaxed(TIMER_DISABLE, (*timer).ctrl);
}

unsafe fn rk_timer_enable(timer: *mut RkTimer, flags: u32) {
    writel_relaxed(TIMER_ENABLE | flags, (*timer).ctrl);
}

unsafe fn rk_timer_update_counter(cycles: usize, timer: *mut RkTimer) {
    writel_relaxed(cycles as u32, ((*timer).base as *mut u8).add(TIMER_LOAD_COUNT0) as *mut c_void);
    writel_relaxed(0, ((*timer).base as *mut u8).add(TIMER_LOAD_COUNT1) as *mut c_void);
}

unsafe fn rk_timer_interrupt_clear(timer: *mut RkTimer) {
    writel_relaxed(1, ((*timer).base as *mut u8).add(TIMER_INT_STATUS) as *mut c_void);
}

unsafe fn rk_timer_set_next_event(cycles: usize, ce: *mut ClockEventDevice) -> i32 {
    let timer = rk_timer(ce);
    rk_timer_disable(timer);
    rk_timer_update_counter(cycles, timer);
    rk_timer_enable(timer, TIMER_MODE_USER_DEFINED_COUNT | TIMER_INT_UNMASK);
    0
}

unsafe fn rk_timer_shutdown(ce: *mut ClockEventDevice) -> i32 {
    rk_timer_disable(rk_timer(ce));
    0
}

unsafe fn rk_timer_set_periodic(ce: *mut ClockEventDevice) -> i32 {
    let timer = rk_timer(ce);
    rk_timer_disable(timer);
    rk_timer_update_counter((*timer).freq / HZ - 1, timer);
    rk_timer_enable(timer, TIMER_MODE_FREE_RUNNING | TIMER_INT_UNMASK);
    0
}

unsafe extern "C" fn rk_timer_interrupt(irq: i32, dev_id: *mut c_void) -> IrqReturn {
    let _ = irq;
    let ce = dev_id as *mut ClockEventDevice;
    let timer = rk_timer(ce);
    rk_timer_interrupt_clear(timer);
    if clockevent_state_oneshot(ce) {
        rk_timer_disable(timer);
    }
    ((*ce).event_handler)(ce);
    IRQ_HANDLED
}

unsafe fn rk_timer_sched_read() -> u64 {
    (!readl_relaxed(((*RK_CLKSRC).base as *mut u8).add(TIMER_CURRENT_VALUE0) as *mut c_void)) as u64
}

unsafe fn rk_timer_probe(timer: *mut RkTimer, np: *mut DeviceNode) -> i32 {
    let timer_clk: *mut Clk;
    let pclk: *mut Clk;
    let mut ret: i32 = -EINVAL;
    let mut irq: i32;
    let mut ctrl_reg: usize = TIMER_CONTROL_REG3288;

    (*timer).base = of_iomap(np, 0);
    if (*timer).base.is_null() {
        pr_err(b"Failed to get base address for '%s'\n\0", TIMER_NAME.as_ptr());
        return -ENXIO;
    }
    if of_device_is_compatible(np, b"rockchip,rk3399-timer\0".as_ptr()) {
        ctrl_reg = TIMER_CONTROL_REG3399;
    }
    (*timer).ctrl = ((*timer).base as *mut u8).add(ctrl_reg) as *mut c_void;
    pclk = of_clk_get_by_name(np, b"pclk\0".as_ptr());
    if is_err(pclk) {
        ret = ptr_err(pclk);
        pr_err(b"Failed to get pclk for '%s'\n\0", TIMER_NAME.as_ptr());
        goto_out_unmap(timer, ret);
    }
    ret = clk_prepare_enable(pclk);
    if ret != 0 {
        pr_err(b"Failed to enable pclk for '%s'\n\0", TIMER_NAME.as_ptr());
        goto_out_unmap(timer, ret);
    }
    (*timer).pclk = pclk;
    timer_clk = of_clk_get_by_name(np, b"timer\0".as_ptr());
    if is_err(timer_clk) {
        ret = ptr_err(timer_clk);
        pr_err(b"Failed to get timer clock for '%s'\n\0", TIMER_NAME.as_ptr());
        clk_disable_unprepare(pclk);
        iounmap((*timer).base);
        return ret;
    }
    ret = clk_prepare_enable(timer_clk);
    if ret != 0 {
        pr_err(b"Failed to enable timer clock\n\0");
        clk_disable_unprepare(pclk);
        iounmap((*timer).base);
        return ret;
    }
    (*timer).clk = timer_clk;
    (*timer).freq = clk_get_rate(timer_clk);
    irq = irq_of_parse_and_map(np, 0);
    if irq == 0 {
        ret = -EINVAL;
        pr_err(b"Failed to map interrupts for '%s'\n\0", TIMER_NAME.as_ptr());
        clk_disable_unprepare(timer_clk);
        clk_disable_unprepare(pclk);
        iounmap((*timer).base);
        return ret;
    }
    (*timer).irq = irq;
    rk_timer_interrupt_clear(timer);
    rk_timer_disable(timer);
    return 0;
}

unsafe fn goto_out_unmap(timer: *mut RkTimer, ret: i32) -> i32 {
    iounmap((*timer).base);
    ret
}

unsafe fn rk_timer_cleanup(timer: *mut RkTimer) {
    clk_disable_unprepare((*timer).clk);
    clk_disable_unprepare((*timer).pclk);
    iounmap((*timer).base);
}

unsafe fn rk_clkevt_init(np: *mut DeviceNode) -> i32 {
    let mut ret: i32 = -EINVAL;
    let clkevt = kzalloc_rk_clkevt();
    if clkevt.is_null() {
        ret = -ENOMEM;
        rk_clkevt = err_ptr(ret) as *mut RkClkevt;
        return ret;
    }
    RK_CLKEVT = clkevt;
    ret = rk_timer_probe(&mut (*clkevt).timer, np);
    if ret != 0 {
        kfree(clkevt as *mut c_void);
        rk_clkevt = err_ptr(ret) as *mut RkClkevt;
        return ret;
    }
    (*clkevt).ce.name = TIMER_NAME.as_ptr();
    (*clkevt).ce.features = CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT | CLOCK_EVT_FEAT_DYNIRQ;
    (*clkevt).ce.set_next_event = Some(rk_timer_set_next_event);
    (*clkevt).ce.set_state_shutdown = Some(rk_timer_shutdown);
    (*clkevt).ce.set_state_periodic = Some(rk_timer_set_periodic);
    (*clkevt).ce.irq = (*clkevt).timer.irq;
    (*clkevt).ce.cpumask = cpu_possible_mask;
    (*clkevt).ce.rating = 250;
    ret = request_irq((*clkevt).timer.irq, Some(rk_timer_interrupt), IRQF_TIMER, TIMER_NAME.as_ptr(), &mut (*clkevt).ce as *mut _ as *mut c_void);
    if ret != 0 {
        pr_err(b"Failed to initialize '%s': %d\n\0", TIMER_NAME.as_ptr(), ret);
        rk_timer_cleanup(&mut (*clkevt).timer);
        kfree(clkevt as *mut c_void);
        rk_clkevt = err_ptr(ret) as *mut RkClkevt;
        return ret;
    }
    clockevents_config_and_register(&mut (*clkevt).ce, (*clkevt).timer.freq, 1, u32::MAX);
    0
}

unsafe fn rk_clksrc_init(np: *mut DeviceNode) -> i32 {
    let mut ret: i32 = -EINVAL;
    let clksrc = kzalloc_rk_timer();
    if clksrc.is_null() {
        ret = -ENOMEM;
        rk_clksrc = err_ptr(ret) as *mut RkTimer;
        return ret;
    }
    RK_CLKSRC = clksrc;
    ret = rk_timer_probe(clksrc, np);
    if ret != 0 {
        kfree(clksrc as *mut c_void);
        rk_clksrc = err_ptr(ret) as *mut RkTimer;
        return ret;
    }
    rk_timer_update_counter(u32::MAX as usize, clksrc);
    rk_timer_enable(clksrc, 0);
    ret = clocksource_mmio_init(((*clksrc).base as *mut u8).add(TIMER_CURRENT_VALUE0) as *mut c_void, TIMER_NAME.as_ptr(), (*clksrc).freq, 250, 32, clocksource_mmio_readl_down);
    if ret != 0 {
        pr_err(b"Failed to register clocksource\n\0");
        rk_timer_cleanup(clksrc);
        kfree(clksrc as *mut c_void);
        rk_clksrc = err_ptr(ret) as *mut RkTimer;
        return ret;
    }
    sched_clock_register(rk_timer_sched_read, 32, (*clksrc).freq);
    0
}

unsafe fn rk_timer_init(np: *mut DeviceNode) -> i32 {
    if RK_CLKEVT.is_null() { return rk_clkevt_init(np); }
    if RK_CLKSRC.is_null() { return rk_clksrc_init(np); }
    pr_err(b"Too many timer definitions for '%s'\n\0", TIMER_NAME.as_ptr());
    -EINVAL
}

// TIMER_OF_DECLARE(rk3288_timer, "rockchip,rk3288-timer", rk_timer_init);
// TIMER_OF_DECLARE(rk3399_timer, "rockchip,rk3399-timer", rk_timer_init);

// The remaining kernel declarations and registration macro are external dependencies.
extern "C" {
    type Clk;
    type DeviceNode;
    type ClockEventDevice;
    type IrqReturn;
    static HZ: u32;
    static EINVAL: i32;
    static ENXIO: i32;
    static IRQ_HANDLED: IrqReturn;
    static mut cpu_possible_mask: *mut c_void;
    fn kzalloc_rk_clkevt() -> *mut RkClkevt;
    fn kzalloc_rk_timer() -> *mut RkTimer;
    fn kfree(ptr: *mut c_void);
    fn err_ptr(ret: i32) -> *mut c_void;
    fn writel_relaxed(value: u32, addr: *mut c_void);
    fn readl_relaxed(addr: *mut c_void) -> u32;
    fn of_iomap(np: *mut DeviceNode, index: i32) -> *mut c_void;
    fn of_device_is_compatible(np: *mut DeviceNode, compatible: *const u8) -> bool;
    fn of_clk_get_by_name(np: *mut DeviceNode, name: *const u8) -> *mut Clk;
    fn is_err(ptr: *mut Clk) -> bool;
    fn ptr_err(ptr: *mut Clk) -> i32;
    fn clk_prepare_enable(clk: *mut Clk) -> i32;
    fn clk_disable_unprepare(clk: *mut Clk);
    fn clk_get_rate(clk: *mut Clk) -> u32;
    fn irq_of_parse_and_map(np: *mut DeviceNode, index: i32) -> i32;
    fn iounmap(addr: *mut c_void);
    fn clockevent_state_oneshot(ce: *mut ClockEventDevice) -> bool;
    fn pr_err(format: *const u8, ...);
    fn request_irq(irq: i32, handler: Option<unsafe extern "C" fn(i32, *mut c_void) -> IrqReturn>, flags: u32, name: *const u8, dev_id: *mut c_void) -> i32;
    fn clockevents_config_and_register(ce: *mut ClockEventDevice, freq: u32, min_delta: u32, max_delta: u32);
    fn clocksource_mmio_init(addr: *mut c_void, name: *const u8, rating: u32, freq: u32, bits: u32, read: unsafe extern "C" fn(*mut c_void) -> u64) -> i32;
    fn clocksource_mmio_readl_down(addr: *mut c_void) -> u64;
    fn sched_clock_register(read: unsafe fn() -> u64, bits: u32, freq: u32);
}

const ENOMEM: i32 = 12;
const IRQF_TIMER: u32 = 0x0000_0020;
const CLOCK_EVT_FEAT_PERIODIC: u32 = 1 << 0;
const CLOCK_EVT_FEAT_ONESHOT: u32 = 1 << 1;
const CLOCK_EVT_FEAT_DYNIRQ: u32 = 1 << 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
