/*
 * Copyright (C) 2007-2013 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2012-2013 Xilinx, Inc.
 * Copyright (C) 2007-2009 PetaLogix
 * Copyright (C) 2006 Atmark Techno, Inc.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 */

use core::ffi::c_void;

type U32 = u32;
type U64 = u64;
type CInt = i32;
type Uint = u32;
type Ulong = usize;
type Irqreturn = CInt;

#[repr(C)] pub struct ClockEventDevice { pub _private: [u8; 0] }
#[repr(C)] pub struct ClockSource { pub _private: [u8; 0] }
#[repr(C)] pub struct CycleCounter { pub _private: [u8; 0] }
#[repr(C)] pub struct TimeCounter { pub cc: *mut CycleCounter }
#[repr(C)] pub struct Clock { pub _private: [u8; 0] }
#[repr(C)] pub struct DeviceNode { pub _private: [u8; 0] }

extern "C" {
    fn iowrite32(val: U32, addr: *mut c_void);
    fn ioread32(addr: *mut c_void) -> Uint;
    fn iowrite32be(val: U32, addr: *mut c_void);
    fn ioread32be(addr: *mut c_void) -> Uint;
    fn pr_debug(fmt: *const u8, ...);
    fn pr_info(fmt: *const u8, ...);
    fn pr_err(fmt: *const u8, ...);
    fn div_sc(a: U32, b: U32, shift: u32) -> U32;
    fn clockevent_delta2ns(delta: U32, dev: *const ClockEventDevice) -> U64;
    fn cpumask_of(cpu: u32) -> *mut c_void;
    fn clockevents_register_device(dev: *mut ClockEventDevice);
    fn sched_clock() -> U64;
    fn timecounter_init(tc: *mut TimeCounter, cc: *mut CycleCounter, nsec: U64);
    fn clocksource_register_hz(cs: *mut ClockSource, hz: U32) -> CInt;
    fn of_property_present(node: *mut DeviceNode, name: *const u8) -> bool;
    fn of_iomap(node: *mut DeviceNode, index: CInt) -> *mut c_void;
    fn irq_of_parse_and_map(node: *mut DeviceNode, index: CInt) -> U32;
    fn of_property_read_u32(node: *mut DeviceNode, name: *const u8, value: *mut U32) -> CInt;
    fn of_clk_get(node: *mut DeviceNode, index: CInt) -> *mut Clock;
    fn is_err(ptr: *mut Clock) -> bool;
    fn clk_get_rate(clk: *mut Clock) -> U32;
    fn request_irq(irq: U32, handler: unsafe extern "C" fn(CInt, *mut c_void) -> Irqreturn,
                   flags: U32, name: *const u8, dev: *mut c_void) -> CInt;
    fn sched_clock_register(read: unsafe extern "C" fn() -> U64, bits: u32, rate: U32);
    static mut cpuinfo: CpuInfo;
}

#[repr(C)] struct CpuInfo { pub cpu_clock_freq: U32 }

static mut timer_baseaddr: *mut c_void = core::ptr::null_mut();
static mut freq_div_hz: U32 = 0;
static mut timer_clock_freq: U32 = 0;

const TCSR0: Ulong = 0x00;
const TLR0: Ulong = 0x04;
const TCR0: Ulong = 0x08;
const TCSR1: Ulong = 0x10;
const TLR1: Ulong = 0x14;
const TCR1: Ulong = 0x18;
const TCSR_MDT: U32 = 1 << 0;
const TCSR_UDT: U32 = 1 << 1;
const TCSR_ARHT: U32 = 1 << 4;
const TCSR_LOAD: U32 = 1 << 5;
const TCSR_ENIT: U32 = 1 << 6;
const TCSR_ENT: U32 = 1 << 7;
const TCSR_TINT: U32 = 1 << 8;

static mut read_fn: Option<unsafe extern "C" fn(*mut c_void) -> Uint> = None;
static mut write_fn: Option<unsafe extern "C" fn(U32, *mut c_void)> = None;

unsafe extern "C" fn timer_write32(val: U32, addr: *mut c_void) { iowrite32(val, addr); }
unsafe extern "C" fn timer_read32(addr: *mut c_void) -> Uint { ioread32(addr) }
unsafe extern "C" fn timer_write32_be(val: U32, addr: *mut c_void) { iowrite32be(val, addr); }
unsafe extern "C" fn timer_read32_be(addr: *mut c_void) -> Uint { ioread32be(addr) }

#[inline] unsafe fn reg(off: Ulong) -> *mut c_void { (timer_baseaddr as *mut u8).add(off) as *mut c_void }
#[inline] unsafe fn xilinx_timer0_stop() { write_fn.unwrap()(read_fn.unwrap()(reg(TCSR0)) & !TCSR_ENT, reg(TCSR0)); }
#[inline] unsafe fn xilinx_timer0_start_periodic(mut load_val: Ulong) {
    if load_val == 0 { load_val = 1; }
    write_fn.unwrap()(load_val as U32, reg(TLR0)); write_fn.unwrap()(TCSR_LOAD, reg(TCSR0));
    write_fn.unwrap()(TCSR_TINT | TCSR_ENIT | TCSR_ENT | TCSR_ARHT | TCSR_UDT, reg(TCSR0));
}
#[inline] unsafe fn xilinx_timer0_start_oneshot(mut load_val: Ulong) {
    if load_val == 0 { load_val = 1; }
    write_fn.unwrap()(load_val as U32, reg(TLR0)); write_fn.unwrap()(TCSR_LOAD, reg(TCSR0));
    write_fn.unwrap()(TCSR_TINT | TCSR_ENIT | TCSR_ENT | TCSR_ARHT | TCSR_UDT, reg(TCSR0));
}

unsafe fn timer_ack() { write_fn.unwrap()(read_fn.unwrap()(reg(TCSR0)), reg(TCSR0)); }
unsafe extern "C" fn xilinx_clock_read() -> U64 { read_fn.unwrap()(reg(TCR1)) as U64 }

unsafe fn xilinx_timer_set_next_event(delta: Ulong, _dev: *mut ClockEventDevice) -> CInt {
    xilinx_timer0_start_oneshot(delta); 0
}
unsafe fn xilinx_timer_shutdown(_evt: *mut ClockEventDevice) -> CInt { xilinx_timer0_stop(); 0 }
unsafe fn xilinx_timer_set_periodic(_evt: *mut ClockEventDevice) -> CInt {
    xilinx_timer0_start_periodic(freq_div_hz as Ulong); 0
}
unsafe extern "C" fn timer_interrupt(_irq: CInt, _dev_id: *mut c_void) -> Irqreturn {
    timer_ack(); 1
}
unsafe extern "C" fn xilinx_read(_cs: *mut ClockSource) -> U64 { xilinx_clock_read() }
unsafe extern "C" fn xilinx_cc_read(_cc: *mut CycleCounter) -> U64 { xilinx_read(core::ptr::null_mut()) }

unsafe fn xilinx_clockevent_init() -> CInt { 0 }
unsafe fn init_xilinx_timecounter() -> CInt {
    timecounter_init(&mut xilinx_tc, core::ptr::null_mut(), sched_clock()); 0
}
unsafe fn xilinx_clocksource_init() -> CInt {
    let ret = clocksource_register_hz(core::ptr::null_mut(), timer_clock_freq);
    if ret != 0 { return ret; }
    write_fn.unwrap()(read_fn.unwrap()(reg(TCSR1)) & !TCSR_ENT, reg(TCSR1));
    write_fn.unwrap()(TCSR_TINT | TCSR_ENT | TCSR_ARHT, reg(TCSR1));
    init_xilinx_timecounter()
}

unsafe fn xilinx_timer_init(timer: *mut DeviceNode) -> CInt {
    static mut initialized: CInt = 0;
    let mut irq: U32;
    let mut timer_num: U32 = 1;
    if of_property_present(timer, b"#pwm-cells\0".as_ptr()) { return 0; }
    if initialized != 0 { return -22; }
    initialized = 1;
    timer_baseaddr = of_iomap(timer, 0);
    if timer_baseaddr.is_null() { return -6; }
    write_fn = Some(timer_write32); read_fn = Some(timer_read32);
    write_fn.unwrap()(TCSR_MDT, reg(TCSR0));
    if read_fn.unwrap()(reg(TCSR0)) & TCSR_MDT == 0 {
        write_fn = Some(timer_write32_be); read_fn = Some(timer_read32_be);
    }
    irq = irq_of_parse_and_map(timer, 0);
    if irq == 0 { return -22; }
    of_property_read_u32(timer, b"xlnx,one-timer-only\0".as_ptr(), &mut timer_num);
    if timer_num != 0 { return -22; }
    let clk = of_clk_get(timer, 0);
    if is_err(clk) {
        of_property_read_u32(timer, b"clock-frequency\0".as_ptr(), &mut timer_clock_freq);
    } else { timer_clock_freq = clk_get_rate(clk); }
    if timer_clock_freq == 0 { timer_clock_freq = cpuinfo.cpu_clock_freq; }
    freq_div_hz = timer_clock_freq / 100;
    let ret = request_irq(irq, timer_interrupt, 0, b"timer\0".as_ptr(), core::ptr::null_mut());
    if ret != 0 { return ret; }
    let ret = xilinx_clocksource_init(); if ret != 0 { return ret; }
    let ret = xilinx_clockevent_init(); if ret != 0 { return ret; }
    sched_clock_register(xilinx_clock_read, 32, timer_clock_freq); 0
}

static mut xilinx_tc: TimeCounter = TimeCounter { cc: core::ptr::null_mut() };

#[allow(dead_code)]
unsafe fn _timer_translation_anchor() {
    let _ = (TCR0, TLR1, xilinx_timer0_stop as unsafe fn(), xilinx_timer0_start_periodic as unsafe fn(Ulong), xilinx_timer0_start_oneshot as unsafe fn(Ulong), timer_ack as unsafe fn(), xilinx_clock_read as unsafe extern "C" fn() -> U64, &mut xilinx_tc, xilinx_timer_set_next_event as unsafe fn(Ulong, *mut ClockEventDevice) -> CInt, xilinx_timer_shutdown as unsafe fn(*mut ClockEventDevice) -> CInt, xilinx_timer_set_periodic as unsafe fn(*mut ClockEventDevice) -> CInt, xilinx_timer_init as unsafe fn(*mut DeviceNode) -> CInt);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
