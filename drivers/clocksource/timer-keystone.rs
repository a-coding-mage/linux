// SPDX-License-Identifier: GPL-2.0-only
/*
 * Keystone broadcast clock-event
 *
 * Copyright 2013 Texas Instruments, Inc.
 *
 * Author: Ivan Khoronzhuk <ivan.khoronzhuk@ti.com>
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

const TIMER_NAME: *const u8 = b"timer-keystone\0".as_ptr();

/* Timer register offsets */
const TIM12: usize = 0x10;
const TIM34: usize = 0x14;
const PRD12: usize = 0x18;
const PRD34: usize = 0x1c;
const TCR: usize = 0x20;
const TGCR: usize = 0x24;
const INTCTLSTAT: usize = 0x44;

/* Timer register bitfields */
const TCR_ENAMODE_MASK: u32 = 0xC0;
const TCR_ENAMODE_ONESHOT_MASK: i32 = 0x40;
const TCR_ENAMODE_PERIODIC_MASK: i32 = 0x80;

const TGCR_TIM_UNRESET_MASK: u32 = 0x03;
const INTCTLSTAT_ENINT_MASK: u32 = 0x01;

extern "C" {
    fn readl_relaxed(addr: *const u8) -> u32;
    fn writel_relaxed(val: u32, addr: *mut u8);
    fn __iowmb();
    fn irq_of_parse_and_map(np: *mut device_node, index: u32) -> i32;
    fn of_iomap(np: *mut device_node, index: u32) -> *mut u8;
    fn of_clk_get(np: *mut device_node, index: u32) -> *mut clk;
    fn clk_prepare_enable(clk: *mut clk) -> i32;
    fn clk_get_rate(clk: *mut clk) -> u64;
    fn clk_put(clk: *mut clk);
    fn iounmap(addr: *mut u8);
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t,
                   flags: u32, name: *const u8, dev: *mut clock_event_device) -> i32;
    fn clockevents_config_and_register(dev: *mut clock_event_device, freq: u64,
                                       min_delta: u32, max_delta: usize);
}

// Types and constants are provided by the kernel environment.
enum device_node {}
enum clk {}
type irqreturn_t = i32;
#[repr(C)] struct clock_event_device {
    event_handler: unsafe extern "C" fn(*mut clock_event_device),
    features: u32,
    set_next_event: unsafe extern "C" fn(usize, *mut clock_event_device) -> i32,
    set_state_shutdown: unsafe extern "C" fn(*mut clock_event_device) -> i32,
    set_state_periodic: unsafe extern "C" fn(*mut clock_event_device) -> i32,
    set_state_oneshot: unsafe extern "C" fn(*mut clock_event_device) -> i32,
    cpumask: *const core::ffi::c_void,
    owner: *mut core::ffi::c_void,
    name: *const u8,
    irq: i32,
}

#[repr(C)] struct keystone_timer {
    base: *mut u8,
    hz_period: usize,
    event_dev: clock_event_device,
}

static mut timer: keystone_timer = unsafe { core::mem::zeroed() };

unsafe fn keystone_timer_readl(rg: usize) -> u32 {
    readl_relaxed(timer.base.add(rg))
}

unsafe fn keystone_timer_writel(val: u32, rg: usize) {
    writel_relaxed(val, timer.base.add(rg));
}

/*
 * keystone_timer_barrier: write memory barrier
 * use explicit barrier to avoid using readl/writel non relaxed function
 * variants, because in our case non relaxed variants hide the true places
 * where barrier is needed.
 */
#[inline]
unsafe fn keystone_timer_barrier() { __iowmb(); }

/* keystone_timer_config: configures timer to work in oneshot/periodic modes. */
unsafe fn keystone_timer_config(period: u64, mask: i32) -> i32 {
    let mut tcr = keystone_timer_readl(TCR);
    let off = tcr & !TCR_ENAMODE_MASK;
    tcr |= mask as u32;
    keystone_timer_writel(off, TCR);
    keystone_timer_barrier();
    keystone_timer_writel(0, TIM12);
    keystone_timer_writel(0, TIM34);
    keystone_timer_writel((period & 0xffff_ffff) as u32, PRD12);
    keystone_timer_writel((period >> 32) as u32, PRD34);
    keystone_timer_barrier();
    keystone_timer_writel(tcr, TCR);
    0
}

unsafe fn keystone_timer_disable() {
    let mut tcr = keystone_timer_readl(TCR);
    tcr &= !TCR_ENAMODE_MASK;
    keystone_timer_writel(tcr, TCR);
}

unsafe extern "C" fn keystone_timer_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let evt = dev_id as *mut clock_event_device;
    ((*evt).event_handler)(evt);
    1
}

unsafe extern "C" fn keystone_set_next_event(cycles: usize, _evt: *mut clock_event_device) -> i32 {
    keystone_timer_config(cycles as u64, TCR_ENAMODE_ONESHOT_MASK)
}

unsafe extern "C" fn keystone_shutdown(_evt: *mut clock_event_device) -> i32 {
    keystone_timer_disable(); 0
}

unsafe extern "C" fn keystone_set_periodic(_evt: *mut clock_event_device) -> i32 {
    keystone_timer_config(timer.hz_period as u64, TCR_ENAMODE_PERIODIC_MASK); 0
}

// __init; registered by TIMER_OF_DECLARE(keystone_timer, "ti,keystone-timer", ...).
unsafe extern "C" fn keystone_timer_init(np: *mut device_node) -> i32 {
    let event_dev = &mut timer.event_dev as *mut clock_event_device;
    let irq = irq_of_parse_and_map(np, 0);
    if irq == 0 { return -22; }
    timer.base = of_iomap(np, 0);
    if timer.base.is_null() { return -6; }
    let clk = of_clk_get(np, 0);
    if clk.is_null() { iounmap(timer.base); return -1; }
    let error = clk_prepare_enable(clk);
    if error != 0 { clk_put(clk); iounmap(timer.base); return error; }
    let rate = clk_get_rate(clk);
    keystone_timer_writel(0, TCR);
    keystone_timer_barrier();
    keystone_timer_writel(0, TGCR);
    keystone_timer_writel(TGCR_TIM_UNRESET_MASK, TGCR);
    keystone_timer_writel(0, TIM12);
    keystone_timer_writel(0, TIM34);
    timer.hz_period = (rate as usize + 99) / 100;
    keystone_timer_writel(INTCTLSTAT_ENINT_MASK, INTCTLSTAT);
    let error = request_irq(irq, keystone_timer_interrupt, 0, TIMER_NAME, event_dev);
    if error != 0 { clk_put(clk); iounmap(timer.base); return error; }
    (*event_dev).features = 0x1 | 0x2;
    (*event_dev).set_next_event = keystone_set_next_event;
    (*event_dev).set_state_shutdown = keystone_shutdown;
    (*event_dev).set_state_periodic = keystone_set_periodic;
    (*event_dev).set_state_oneshot = keystone_shutdown;
    (*event_dev).cpumask = core::ptr::null();
    (*event_dev).owner = core::ptr::null_mut();
    (*event_dev).name = TIMER_NAME;
    (*event_dev).irq = irq;
    clockevents_config_and_register(event_dev, rate, 1, usize::MAX);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
