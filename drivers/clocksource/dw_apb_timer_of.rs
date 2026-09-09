// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 Altera Corporation
 * Copyright (c) 2011 Picochip Ltd., Jamie Iles
 *
 * Modified from mach-picoxcell/time.c
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Dependencies supplied by the surrounding kernel translation.
#[repr(C)]
pub struct device_node {
    pub name: *const c_char,
}
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct reset_control { _private: [u8; 0] }
#[repr(C)] pub struct dw_apb_clock_event_device { _private: [u8; 0] }
#[repr(C)] pub struct dw_apb_clocksource { _private: [u8; 0] }
#[repr(C)] pub struct of_device_id { pub compatible: *const c_char }
#[repr(C)] pub struct delay_timer { pub read_current_timer: Option<unsafe extern "C" fn() -> c_ulong>, pub freq: u32 }

extern "C" {
    fn of_iomap(np: *mut device_node, index: c_int) -> *mut c_void;
    fn panic(fmt: *const c_char, ... ) -> !;
    fn of_reset_control_get(np: *mut device_node, id: *const c_char) -> *mut reset_control;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn reset_control_assert(rstc: *mut reset_control);
    fn reset_control_deassert(rstc: *mut reset_control);
    fn of_clk_get_by_name(np: *mut device_node, name: *const c_char) -> *mut clk;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn pr_warn(fmt: *const c_char, ...);
    fn of_property_read_u32(np: *mut device_node, name: *const c_char, out: *mut u32) -> c_int;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_put(clk: *mut clk);
    fn iounmap(addr: *mut c_void);
    fn clk_get_rate(clk: *mut clk) -> u32;
    fn irq_of_parse_and_map(np: *mut device_node, index: c_uint) -> u32;
    fn dw_apb_clockevent_init(rating: c_int, name: *const c_char, freq: c_uint, base: *mut c_void, irq: u32, rate: u32) -> *mut dw_apb_clock_event_device;
    fn dw_apb_clockevent_register(ced: *mut dw_apb_clock_event_device);
    fn dw_apb_clocksource_init(rating: c_int, name: *const c_char, base: *mut c_void, rate: u32) -> *mut dw_apb_clocksource;
    fn dw_apb_clocksource_start(cs: *mut dw_apb_clocksource);
    fn dw_apb_clocksource_register(cs: *mut dw_apb_clocksource);
    fn readl_relaxed(addr: *mut c_void) -> u32;
    fn of_find_matching_node(from: *mut device_node, matches: *const of_device_id) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn sched_clock_register(read: Option<unsafe extern "C" fn() -> u64>, bits: u32, rate: u32);
    fn pr_debug(fmt: *const c_char, ...);
    fn register_current_timer_delay(timer: *mut delay_timer);
}

static mut sched_io_base: *mut c_void = core::ptr::null_mut();
static mut sched_rate: u32 = 0;

unsafe extern "C" fn timer_get_base_and_rate(np: *mut device_node, base: *mut *mut c_void, rate: *mut u32) -> c_int {
    let mut timer_clk: *mut clk;
    let pclk: *mut clk;
    let rstc: *mut reset_control;
    let mut ret: c_int;

    *base = of_iomap(np, 0);
    if (*base).is_null() { panic(b"Unable to map regs for %pOFn\0".as_ptr() as *const c_char); }
    rstc = of_reset_control_get(np, core::ptr::null());
    if !IS_ERR(rstc as *const c_void) { reset_control_assert(rstc); reset_control_deassert(rstc); }
    pclk = of_clk_get_by_name(np, b"pclk\0".as_ptr() as *const c_char);
    if !IS_ERR(pclk as *const c_void) && clk_prepare_enable(pclk) != 0 { pr_warn(b"pclk for %pOFn is present, but could not be activated\n\0".as_ptr() as *const c_char, np); }
    if of_property_read_u32(np, b"clock-freq\0".as_ptr() as *const c_char, rate) == 0 || of_property_read_u32(np, b"clock-frequency\0".as_ptr() as *const c_char, rate) == 0 { return 0; }
    timer_clk = of_clk_get_by_name(np, b"timer\0".as_ptr() as *const c_char);
    if IS_ERR(timer_clk as *const c_void) { ret = PTR_ERR(timer_clk as *const c_void); }
    else {
        ret = clk_prepare_enable(timer_clk);
        if ret == 0 { *rate = clk_get_rate(timer_clk); if *rate != 0 { return 0; } ret = -22; }
        if ret != 0 { clk_disable_unprepare(timer_clk); clk_put(timer_clk); }
    }
    if !IS_ERR(pclk as *const c_void) { clk_disable_unprepare(pclk); clk_put(pclk); }
    iounmap(*base); ret
}

unsafe extern "C" fn add_clockevent(event_timer: *mut device_node) -> c_int {
    let mut iobase = core::ptr::null_mut(); let mut rate = 0; let irq = irq_of_parse_and_map(event_timer, 0);
    if irq == 0 { panic(b"No IRQ for clock event timer\0".as_ptr() as *const c_char); }
    let ret = timer_get_base_and_rate(event_timer, &mut iobase, &mut rate); if ret != 0 { return ret; }
    let ced = dw_apb_clockevent_init(-1, (*event_timer).name, 300, iobase, irq, rate); if ced.is_null() { return -22; }
    dw_apb_clockevent_register(ced); 0
}

unsafe extern "C" fn add_clocksource(source_timer: *mut device_node) -> c_int {
    let mut iobase = core::ptr::null_mut(); let mut rate = 0; let ret = timer_get_base_and_rate(source_timer, &mut iobase, &mut rate); if ret != 0 { return ret; }
    let cs = dw_apb_clocksource_init(300, (*source_timer).name, iobase, rate); if cs.is_null() { return -22; }
    dw_apb_clocksource_start(cs); dw_apb_clocksource_register(cs); sched_io_base = (iobase as *mut u8).add(0x04) as *mut c_void; sched_rate = rate; 0
}

unsafe extern "C" fn read_sched_clock() -> u64 { (!readl_relaxed(sched_io_base)) as u64 }
static sptimer_ids: [of_device_id; 2] = [of_device_id { compatible: b"picochip,pc3x2-rtc\0".as_ptr() as *const c_char }, of_device_id { compatible: core::ptr::null() }];

unsafe extern "C" fn init_sched_clock() { let sched_timer = of_find_matching_node(core::ptr::null_mut(), sptimer_ids.as_ptr()); if !sched_timer.is_null() { let _ = timer_get_base_and_rate(sched_timer, &mut sched_io_base, &mut sched_rate); of_node_put(sched_timer); } sched_clock_register(Some(read_sched_clock), 32, sched_rate); }

#[cfg(target_arch = "arm")]
unsafe extern "C" fn dw_apb_delay_timer_read() -> c_ulong { (!readl_relaxed(sched_io_base)) as c_ulong }
#[cfg(target_arch = "arm")]
static mut dw_apb_delay_timer: delay_timer = delay_timer { read_current_timer: Some(dw_apb_delay_timer_read), freq: 0 };

static mut num_called: c_int = 0;
unsafe extern "C" fn dw_apb_timer_init(timer: *mut device_node) -> c_int {
    let ret = if num_called == 1 { pr_debug(b"%s: found clocksource timer\n\0".as_ptr() as *const c_char, b"dw_apb_timer_init\0".as_ptr()); let r = add_clocksource(timer); if r != 0 { return r; } init_sched_clock(); #[cfg(target_arch = "arm")] { dw_apb_delay_timer.freq = sched_rate; register_current_timer_delay(&mut dw_apb_delay_timer); } 0 } else { pr_debug(b"%s: found clockevent timer\n\0".as_ptr() as *const c_char, b"dw_apb_timer_init\0".as_ptr()); add_clockevent(timer) };
    if ret != 0 { return ret; } num_called += 1; 0
}

// TIMER_OF_DECLARE registrations: pc3x2_timer, apb_timer_osc, apb_timer_sp, and apb_timer.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
