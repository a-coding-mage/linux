// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015 ARM Limited
 *
 * Author: Vladimir Murzin <vladimir.murzin@arm.com>
 */

// Kernel dependencies supplied by other translation units.

const TIMER_CTRL: usize = 0x0;
const TIMER_CTRL_ENABLE: u32 = 1 << 0;
const TIMER_CTRL_IE: u32 = 1 << 3;
const TIMER_VALUE: usize = 0x4;
const TIMER_RELOAD: usize = 0x8;
const TIMER_INT: usize = 0xc;

#[repr(C)]
struct clockevent_mps2 {
    reg: *mut core::ffi::c_void,
    clock_count_per_tick: u32,
    clkevt: clock_event_device,
}

static mut sched_clock_base: *mut core::ffi::c_void = core::ptr::null_mut();

extern "C" {
    fn readl_relaxed(addr: *const core::ffi::c_void) -> u32;
    fn writel_relaxed(value: u32, addr: *mut core::ffi::c_void);
    fn pr_warn(fmt: *const core::ffi::c_char, ...);
    fn pr_err(fmt: *const core::ffi::c_char, ...);
    fn of_property_read_u32(np: *mut device_node, name: *const core::ffi::c_char, value: *mut u32) -> i32;
    fn of_clk_get(np: *mut device_node, index: i32) -> *mut clk;
    fn clk_prepare_enable(clk: *mut clk) -> i32;
    fn clk_get_rate(clk: *mut clk) -> u32;
    fn of_iomap(np: *mut device_node, index: i32) -> *mut core::ffi::c_void;
    fn irq_of_parse_and_map(np: *mut device_node, index: i32) -> i32;
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t, flags: u32, name: *const core::ffi::c_char, dev: *mut core::ffi::c_void) -> i32;
    fn clockevents_config_and_register(dev: *mut clock_event_device, rate: u32, min: u32, max: u32);
    fn clocksource_mmio_init(addr: *mut core::ffi::c_void, name: *const core::ffi::c_char, rate: u32, rating: u32, bits: u32, read: unsafe extern "C" fn(*mut core::ffi::c_void) -> u64) -> i32;
    fn clocksource_mmio_readl_down(addr: *mut core::ffi::c_void) -> u64;
    fn sched_clock_register(read: unsafe extern "C" fn() -> u64, bits: u32, rate: u32);
    fn iounmap(addr: *mut core::ffi::c_void);
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_put(clk: *mut clk);
    fn kfree(ptr: *mut core::ffi::c_void);
}

#[repr(C)] struct device_node;
#[repr(C)] struct clk;
#[repr(C)] struct cpumask;
type irqreturn_t = i32;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_TIMER: u32 = 0x0000_0200;
const HZ: u32 = 100;
const CLOCK_EVT_FEAT_PERIODIC: u32 = 1 << 0;
const CLOCK_EVT_FEAT_ONESHOT: u32 = 1 << 1;

#[repr(C)]
struct clock_event_device {
    irq: i32,
    name: *const core::ffi::c_char,
    rating: u32,
    features: u32,
    cpumask: *const cpumask,
    set_state_shutdown: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    set_state_periodic: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    set_state_oneshot: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    set_next_event: Option<unsafe extern "C" fn(usize, *mut clock_event_device) -> i32>,
    event_handler: unsafe extern "C" fn(*mut clock_event_device),
}

unsafe extern "C" fn mps2_sched_read() -> u64 {
    !(readl_relaxed(sched_clock_base.add(TIMER_VALUE))) as u64
}

unsafe fn to_mps2_clkevt(c: *mut clock_event_device) -> *mut clockevent_mps2 {
    (c as *mut u8).sub(core::mem::offset_of!(clockevent_mps2, clkevt)) as *mut clockevent_mps2
}

unsafe fn clockevent_mps2_writel(val: u32, c: *mut clock_event_device, offset: usize) {
    let ce = to_mps2_clkevt(c);
    writel_relaxed(val, (*ce).reg.add(offset));
}

unsafe extern "C" fn mps2_timer_shutdown(ce: *mut clock_event_device) -> i32 {
    clockevent_mps2_writel(0, ce, TIMER_RELOAD);
    clockevent_mps2_writel(0, ce, TIMER_CTRL);
    0
}

unsafe extern "C" fn mps2_timer_set_next_event(next: usize, ce: *mut clock_event_device) -> i32 {
    clockevent_mps2_writel(next as u32, ce, TIMER_VALUE);
    clockevent_mps2_writel(TIMER_CTRL_IE | TIMER_CTRL_ENABLE, ce, TIMER_CTRL);
    0
}

unsafe extern "C" fn mps2_timer_set_periodic(ce: *mut clock_event_device) -> i32 {
    let count = (*to_mps2_clkevt(ce)).clock_count_per_tick;
    clockevent_mps2_writel(count, ce, TIMER_RELOAD);
    clockevent_mps2_writel(count, ce, TIMER_VALUE);
    clockevent_mps2_writel(TIMER_CTRL_IE | TIMER_CTRL_ENABLE, ce, TIMER_CTRL);
    0
}

unsafe extern "C" fn mps2_timer_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let ce = dev_id as *mut clockevent_mps2;
    let status = readl_relaxed((*ce).reg.add(TIMER_INT));
    if status == 0 {
        pr_warn(b"spurious interrupt\n\0".as_ptr() as *const _);
        return IRQ_NONE;
    }
    writel_relaxed(1, (*ce).reg.add(TIMER_INT));
    ((*ce).clkevt.event_handler)(&mut (*ce).clkevt);
    IRQ_HANDLED
}

// The remaining init routines retain the kernel API's external allocation,
// clock, IRQ, and device-tree operations; build-time kernel constants and
// registration are provided by the surrounding kernel translation.

unsafe extern "C" fn mps2_clockevent_init(np: *mut device_node) -> i32 {
    let mut rate = 0u32;
    let mut clk_ptr: *mut clk = core::ptr::null_mut();
    let mut ret = of_property_read_u32(np, b"clock-frequency\0".as_ptr() as *const _, &mut rate);
    if ret != 0 {
        clk_ptr = of_clk_get(np, 0);
        if clk_ptr.is_null() { return ret; }
        ret = clk_prepare_enable(clk_ptr);
        if ret != 0 { clk_put(clk_ptr); return ret; }
        rate = clk_get_rate(clk_ptr);
    }
    let base = of_iomap(np, 0);
    if base.is_null() { clk_disable_unprepare(clk_ptr); clk_put(clk_ptr); return -99; }
    let irq = irq_of_parse_and_map(np, 0);
    if irq == 0 { iounmap(base); clk_disable_unprepare(clk_ptr); clk_put(clk_ptr); return -2; }
    let ce = alloc_clockevent();
    if ce.is_null() { iounmap(base); clk_disable_unprepare(clk_ptr); clk_put(clk_ptr); return -12; }
    (*ce).reg = base;
    (*ce).clock_count_per_tick = ((rate + HZ / 2) / HZ);
    (*ce).clkevt.irq = irq;
    (*ce).clkevt.name = b"mps2-clkevt\0".as_ptr() as *const _;
    (*ce).clkevt.rating = 200;
    (*ce).clkevt.features = CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT;
    (*ce).clkevt.set_state_shutdown = Some(mps2_timer_shutdown);
    (*ce).clkevt.set_state_periodic = Some(mps2_timer_set_periodic);
    (*ce).clkevt.set_state_oneshot = Some(mps2_timer_shutdown);
    (*ce).clkevt.set_next_event = Some(mps2_timer_set_next_event);
    writel_relaxed(0, base.add(TIMER_CTRL));
    ret = request_irq(irq, mps2_timer_interrupt, IRQF_TIMER, (*ce).clkevt.name, ce as *mut _);
    if ret != 0 { kfree(ce as *mut _); iounmap(base); clk_disable_unprepare(clk_ptr); clk_put(clk_ptr); return ret; }
    clockevents_config_and_register(&mut (*ce).clkevt, rate, 0xf, 0xffff_ffff);
    0
}

unsafe extern "C" fn mps2_clocksource_init(np: *mut device_node) -> i32 {
    let mut rate = 0u32;
    let mut clk_ptr: *mut clk = core::ptr::null_mut();
    let mut ret = of_property_read_u32(np, b"clock-frequency\0".as_ptr() as *const _, &mut rate);
    if ret != 0 { clk_ptr = of_clk_get(np, 0); if clk_ptr.is_null() { return ret; } ret = clk_prepare_enable(clk_ptr); if ret != 0 { clk_put(clk_ptr); return ret; } rate = clk_get_rate(clk_ptr); }
    let base = of_iomap(np, 0); if base.is_null() { clk_disable_unprepare(clk_ptr); clk_put(clk_ptr); return -99; }
    writel_relaxed(0, base.add(TIMER_CTRL)); writel_relaxed(0xffff_ffff, base.add(TIMER_VALUE)); writel_relaxed(0xffff_ffff, base.add(TIMER_RELOAD)); writel_relaxed(TIMER_CTRL_ENABLE, base.add(TIMER_CTRL));
    ret = clocksource_mmio_init(base.add(TIMER_VALUE), b"mps2-clksrc\0".as_ptr() as *const _, rate, 200, 32, clocksource_mmio_readl_down);
    if ret != 0 { iounmap(base); clk_disable_unprepare(clk_ptr); clk_put(clk_ptr); return ret; }
    sched_clock_base = base; sched_clock_register(mps2_sched_read, 32, rate); 0
}

extern "C" { fn alloc_clockevent() -> *mut clockevent_mps2; }

#[no_mangle]
pub unsafe extern "C" fn mps2_timer_init(np: *mut device_node) -> i32 {
    static mut HAS_CLOCKSOURCE: bool = false;
    static mut HAS_CLOCKEVENT: bool = false;
    if !HAS_CLOCKSOURCE { if mps2_clocksource_init(np) == 0 { HAS_CLOCKSOURCE = true; return 0; } }
    if !HAS_CLOCKEVENT { if mps2_clockevent_init(np) == 0 { HAS_CLOCKEVENT = true; return 0; } }
    0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
