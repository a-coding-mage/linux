// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/kernel/smp_twd.c
 *
 *  Copyright (C) 2002 ARM Ltd.
 *  All Rights Reserved
 */

// Kernel and architecture dependencies supplied by other translation units.
use core::ffi::c_void;

extern "C" {
    static mut twd_base: *mut c_void;
    static mut twd_clk: *mut clk;
    static mut twd_timer_rate: c_ulong;
    static mut percpu_setup_called: bool;
    static mut twd_evt: *mut clock_event_device;
    static mut twd_features: c_uint;
    static mut twd_ppi: c_int;
    static mut late_time_init: Option<unsafe extern "C" fn()>;
}

type c_ulong = usize;
type c_uint = u32;
type c_int = i32;
type irqreturn_t = c_int;

#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct clock_event_device {
    pub name: *const u8,
    pub features: c_uint,
    pub rating: c_int,
    pub set_state_shutdown: Option<unsafe extern "C" fn(*mut clock_event_device) -> c_int>,
    pub set_state_periodic: Option<unsafe extern "C" fn(*mut clock_event_device) -> c_int>,
    pub set_state_oneshot: Option<unsafe extern "C" fn(*mut clock_event_device) -> c_int>,
    pub tick_resume: Option<unsafe extern "C" fn(*mut clock_event_device) -> c_int>,
    pub set_next_event: Option<unsafe extern "C" fn(c_ulong, *mut clock_event_device) -> c_int>,
    pub irq: c_int,
    pub cpumask: *const c_void,
    pub event_handler: Option<unsafe extern "C" fn(*mut clock_event_device)>,
}
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int> }
#[repr(C)] pub struct clk_notifier_data { pub new_rate: c_ulong }

extern "C" {
    fn writel_relaxed(value: u32, addr: *mut c_void);
    fn readl_relaxed(addr: *mut c_void) -> u32;
    fn disable_percpu_irq(irq: c_int);
    fn enable_percpu_irq(irq: c_int, flags: c_uint);
    fn clockevents_update_freq(dev: *mut clock_event_device, freq: c_ulong);
    fn on_each_cpu(func: unsafe extern "C" fn(*mut c_void), info: *mut c_void, wait: c_int);
    fn clk_notifier_register(clk: *mut clk, nb: *mut notifier_block) -> c_int;
    fn get_jiffies_64() -> u64;
    fn udelay(usecs: c_uint);
    fn pr_info(fmt: *const u8, ...);
    fn pr_cont(fmt: *const u8, ...);
    fn pr_err(fmt: *const u8, ...);
    fn of_clk_get(np: *mut device_node, index: c_int) -> *mut clk;
    fn clk_get_sys(dev_id: *const u8, con_id: *const u8) -> *mut clk;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_put(clk: *mut clk);
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn raw_cpu_ptr(evt: *mut clock_event_device) -> *mut clock_event_device;
    fn smp_processor_id() -> c_int;
    fn clockevents_register_device(dev: *mut clock_event_device);
    fn clockevents_config_and_register(dev: *mut clock_event_device, freq: c_ulong, min: u32, max: u32);
    fn request_percpu_irq(irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, name: *const u8, dev: *mut clock_event_device) -> c_int;
    fn cpuhp_setup_state_nocalls(state: c_int, name: *const u8, startup: unsafe extern "C" fn(c_uint) -> c_int, teardown: unsafe extern "C" fn(c_uint) -> c_int);
    fn iounmap(addr: *mut c_void);
    fn free_percpu(ptr: *mut clock_event_device);
    fn irq_of_parse_and_map(np: *mut device_node, index: c_int) -> c_int;
    fn of_iomap(np: *mut device_node, index: c_int) -> *mut c_void;
    fn of_property_read_bool(np: *mut device_node, propname: *const u8) -> bool;
    fn alloc_percpu_clock_event_device() -> *mut clock_event_device;
}

const TWD_TIMER_CONTROL: usize = 0x08;
const TWD_TIMER_LOAD: usize = 0x00;
const TWD_TIMER_COUNTER: usize = 0x04;
const TWD_TIMER_INTSTAT: usize = 0x0c;
const TWD_TIMER_CONTROL_ENABLE: u32 = 1;
const TWD_TIMER_CONTROL_IT_ENABLE: u32 = 2;
const TWD_TIMER_CONTROL_PERIODIC: u32 = 4;
const TWD_TIMER_CONTROL_ONESHOT: u32 = 0;
const CLOCK_EVT_FEAT_PERIODIC: c_uint = 1;
const CLOCK_EVT_FEAT_ONESHOT: c_uint = 2;
const CLOCK_EVT_FEAT_C3STOP: c_uint = 4;
const HZ: c_ulong = 100;
const POST_RATE_CHANGE: c_ulong = 1;
const NOTIFY_OK: c_int = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQ_NONE: irqreturn_t = 0;

unsafe extern "C" fn twd_shutdown(_clk: *mut clock_event_device) -> c_int {
    writel_relaxed(0, (twd_base as usize + TWD_TIMER_CONTROL) as *mut c_void); 0
}
unsafe extern "C" fn twd_set_oneshot(_clk: *mut clock_event_device) -> c_int {
    writel_relaxed(TWD_TIMER_CONTROL_IT_ENABLE | TWD_TIMER_CONTROL_ONESHOT, (twd_base as usize + TWD_TIMER_CONTROL) as *mut c_void); 0
}
unsafe extern "C" fn twd_set_periodic(_clk: *mut clock_event_device) -> c_int {
    let ctrl = TWD_TIMER_CONTROL_ENABLE | TWD_TIMER_CONTROL_IT_ENABLE | TWD_TIMER_CONTROL_PERIODIC;
    writel_relaxed((twd_timer_rate + HZ / 2) as u32 / HZ as u32, (twd_base as usize + TWD_TIMER_LOAD) as *mut c_void);
    writel_relaxed(ctrl, (twd_base as usize + TWD_TIMER_CONTROL) as *mut c_void); 0
}
unsafe extern "C" fn twd_set_next_event(evt: c_ulong, _unused: *mut clock_event_device) -> c_int {
    let ctrl = readl_relaxed((twd_base as usize + TWD_TIMER_CONTROL) as *mut c_void) | TWD_TIMER_CONTROL_ENABLE;
    writel_relaxed(evt as u32, (twd_base as usize + TWD_TIMER_COUNTER) as *mut c_void);
    writel_relaxed(ctrl, (twd_base as usize + TWD_TIMER_CONTROL) as *mut c_void); 0
}
unsafe fn twd_timer_ack() -> c_int {
    if readl_relaxed((twd_base as usize + TWD_TIMER_INTSTAT) as *mut c_void) != 0 { writel_relaxed(1, (twd_base as usize + TWD_TIMER_INTSTAT) as *mut c_void); return 1; } 0
}
unsafe fn twd_timer_stop() { disable_percpu_irq((*raw_cpu_ptr(twd_evt)).irq); }
unsafe extern "C" fn twd_update_frequency(new_rate: *mut c_void) { twd_timer_rate = *(new_rate as *mut c_ulong); clockevents_update_freq(raw_cpu_ptr(twd_evt), twd_timer_rate); }
unsafe extern "C" fn twd_rate_change(_nb: *mut notifier_block, flags: c_ulong, data: *mut c_void) -> c_int { if flags == POST_RATE_CHANGE { on_each_cpu(twd_update_frequency, &mut (*(data as *mut clk_notifier_data)).new_rate as *mut _ as *mut c_void, 1); } NOTIFY_OK }
static mut twd_clk_nb: notifier_block = notifier_block { notifier_call: Some(twd_rate_change) };

unsafe fn twd_calibrate_rate() {
    if twd_timer_rate == 0 {
        pr_info(b"Calibrating local timer... \0".as_ptr());
        let mut waitjiffies = get_jiffies_64() + 1; while get_jiffies_64() < waitjiffies { udelay(10); }
        waitjiffies += 5; writel_relaxed(1, (twd_base as usize + TWD_TIMER_CONTROL) as *mut c_void); writel_relaxed(0xffff_ffff, (twd_base as usize + TWD_TIMER_COUNTER) as *mut c_void);
        while get_jiffies_64() < waitjiffies { udelay(10); }
        let count = readl_relaxed((twd_base as usize + TWD_TIMER_COUNTER) as *mut c_void); twd_timer_rate = (0xffff_ffffu32 - count) as usize * (HZ / 5);
        pr_cont(b"%lu.%02luMHz.\n\0".as_ptr(), twd_timer_rate / 1_000_000, (twd_timer_rate / 10_000) % 100);
    }
}
unsafe extern "C" fn twd_handler(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t { let evt = dev_id as *mut clock_event_device; if twd_timer_ack() != 0 { ((*evt).event_handler.unwrap())(evt); IRQ_HANDLED } else { IRQ_NONE } }

// The remaining platform registration and per-CPU setup retain the C driver's externally supplied kernel machinery.
unsafe extern "C" fn twd_timer_starting_cpu(_cpu: c_uint) -> c_int { twd_timer_setup(); 0 }
unsafe extern "C" fn twd_timer_dying_cpu(_cpu: c_uint) -> c_int { twd_timer_stop(); 0 }
unsafe fn twd_timer_setup() {
    let clk = raw_cpu_ptr(twd_evt); let cpu = smp_processor_id();
    if percpu_setup_called { writel_relaxed(0, (twd_base as usize + TWD_TIMER_CONTROL) as *mut c_void); clockevents_register_device(clk); enable_percpu_irq((*clk).irq, 0); return; }
    percpu_setup_called = true; twd_calibrate_rate(); writel_relaxed(0, (twd_base as usize + TWD_TIMER_CONTROL) as *mut c_void);
    (*clk).name = b"local_timer\0".as_ptr(); (*clk).features = twd_features; (*clk).rating = 350; (*clk).set_state_shutdown = Some(twd_shutdown); (*clk).set_state_periodic = Some(twd_set_periodic); (*clk).set_state_oneshot = Some(twd_set_oneshot); (*clk).tick_resume = Some(twd_shutdown); (*clk).set_next_event = Some(twd_set_next_event); (*clk).irq = twd_ppi; (*clk).cpumask = core::ptr::null(); clockevents_config_and_register(clk, twd_timer_rate, 0xf, 0xffff_ffff); enable_percpu_irq((*clk).irq, 0); let _ = cpu;
}

unsafe fn twd_get_clock(np: *mut device_node) {
    let clk = if !np.is_null() { of_clk_get(np, 0) } else { clk_get_sys(b"smp_twd\0".as_ptr(), core::ptr::null()) };
    twd_clk = clk;
    if clk.is_null() { pr_err(b"smp_twd: clock not found %d\n\0".as_ptr(), -1); return; }
    let err = clk_prepare_enable(clk);
    if err != 0 { pr_err(b"smp_twd: clock failed to prepare+enable: %d\n\0".as_ptr(), err); clk_put(clk); return; }
    twd_timer_rate = clk_get_rate(clk);
}

unsafe fn twd_local_timer_common_register(np: *mut device_node) -> c_int {
    twd_evt = alloc_percpu_clock_event_device();
    if twd_evt.is_null() { iounmap(twd_base); twd_base = core::ptr::null_mut(); free_percpu(twd_evt); return -12; }
    let err = request_percpu_irq(twd_ppi, twd_handler, b"twd\0".as_ptr(), twd_evt);
    if err != 0 { pr_err(b"twd: can't register interrupt %d (%d)\n\0".as_ptr(), twd_ppi, err); iounmap(twd_base); twd_base = core::ptr::null_mut(); free_percpu(twd_evt); return err; }
    cpuhp_setup_state_nocalls(0, b"arm/timer/twd:starting\0".as_ptr(), twd_timer_starting_cpu, twd_timer_dying_cpu);
    twd_get_clock(np);
    if !of_property_read_bool(np, b"always-on\0".as_ptr()) { twd_features |= CLOCK_EVT_FEAT_C3STOP; }
    if twd_timer_rate != 0 { twd_timer_setup(); } else { late_time_init = Some(twd_timer_setup); }
    0
}

unsafe fn twd_local_timer_of_register(np: *mut device_node) -> c_int {
    twd_ppi = irq_of_parse_and_map(np, 0);
    if twd_ppi == 0 { return -22; }
    twd_base = of_iomap(np, 0);
    if twd_base.is_null() { return -12; }
    twd_local_timer_common_register(np)
}

// TIMER_OF_DECLARE entries:
// arm_twd_a9:  "arm,cortex-a9-twd-timer"
// arm_twd_a5:  "arm,cortex-a5-twd-timer"
// arm_twd_11mp: "arm,arm11mp-twd-timer"

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
