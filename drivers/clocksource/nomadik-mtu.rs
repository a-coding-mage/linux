// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008 STMicroelectronics
 * Copyright (C) 2010 Alessandro Rubini
 * Copyright (C) 2010 Linus Walleij for ST-Ericsson
 */

use core::ffi::c_void;

// Linux kernel dependencies supplied by other translation units.
#[repr(C)] pub struct clock_event_device { pub name: *const i8, pub features: u32, pub rating: i32, pub set_state_shutdown: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>, pub set_state_periodic: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>, pub set_state_oneshot: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>, pub set_next_event: Option<unsafe extern "C" fn(usize, *mut clock_event_device) -> i32>, pub resume: Option<unsafe extern "C" fn(*mut clock_event_device)>, pub cpumask: *const c_void, pub irq: i32, pub event_handler: Option<unsafe extern "C" fn(*mut clock_event_device)> }
#[repr(C)] pub struct delay_timer { pub read_current_timer: Option<unsafe extern "C" fn() -> usize>, pub freq: usize }
#[repr(C)] pub struct device_node;
#[repr(C)] pub struct clk;

type __iomem = u8;
type irqreturn_t = i32;

extern "C" {
    fn readl(addr: *const c_void) -> u32;
    fn readl_relaxed(addr: *const c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn clk_prepare_enable(clk: *mut clk) -> i32;
    fn clk_get_rate(clk: *mut clk) -> usize;
    fn clocksource_mmio_init(addr: *mut c_void, name: *const i8, rate: usize, rating: i32, bits: u32, read: Option<unsafe extern "C" fn(*const c_void) -> u64>) -> i32;
    fn sched_clock_register(read: Option<unsafe extern "C" fn() -> u64>, bits: u32, rate: usize);
    fn request_irq(irq: i32, handler: Option<unsafe extern "C" fn(i32, *mut c_void) -> irqreturn_t>, flags: u32, name: *const i8, dev_id: *mut c_void) -> i32;
    fn cpumask_of(cpu: u32) -> *const c_void;
    fn clockevents_config_and_register(dev: *mut clock_event_device, rate: usize, min_delta: i32, max_delta: u32);
    fn register_current_timer_delay(timer: *mut delay_timer);
    fn of_iomap(node: *mut device_node, index: i32) -> *mut c_void;
    fn of_clk_get_by_name(node: *mut device_node, name: *const i8) -> *mut clk;
    fn irq_of_parse_and_map(node: *mut device_node, index: i32) -> i32;
    fn pr_err(fmt: *const i8, ...);
    fn clocksource_mmio_readl_down(addr: *const c_void) -> u64;
}

const MTU_IMSC: usize = 0x00;
const MTU_RIS: usize = 0x04;
const MTU_MIS: usize = 0x08;
const MTU_ICR: usize = 0x0C;
const MTU_ITCR: usize = 0xff0;
const MTU_ITOP: usize = 0xff4;
const MTU_PERIPH_ID0: usize = 0xfe0;
const MTU_PERIPH_ID1: usize = 0xfe4;
const MTU_PERIPH_ID2: usize = 0xfe8;
const MTU_PERIPH_ID3: usize = 0xfeC;
const MTU_PCELL0: usize = 0xff0;
const MTU_PCELL1: usize = 0xff4;
const MTU_PCELL2: usize = 0xff8;
const MTU_PCELL3: usize = 0xffC;
const MTU_CRn_ENA: u32 = 0x80;
const MTU_CRn_PERIODIC: u32 = 0x40;
const MTU_CRn_PRESCALE_MASK: u32 = 0x0c;
const MTU_CRn_PRESCALE_1: u32 = 0x00;
const MTU_CRn_PRESCALE_16: u32 = 0x04;
const MTU_CRn_PRESCALE_256: u32 = 0x08;
const MTU_CRn_32BITS: u32 = 0x02;
const MTU_CRn_ONESHOT: u32 = 0x01;

#[inline] fn mtu_lr(x: usize) -> usize { 0x10 + 0x10 * x }
#[inline] fn mtu_val(x: usize) -> usize { 0x10 + 0x10 * x + 0x04 }
#[inline] fn mtu_cr(x: usize) -> usize { 0x10 + 0x10 * x + 0x08 }
#[inline] fn mtu_bglr(x: usize) -> usize { 0x10 + 0x10 * x + 0x0c }

static mut mtu_base: *mut c_void = core::ptr::null_mut();
static mut clkevt_periodic: bool = false;
static mut clk_prescale: u32 = 0;
static mut nmdk_cycle: u32 = 0;
static mut mtu_delay_timer: delay_timer = delay_timer { read_current_timer: None, freq: 0 };

unsafe extern "C" fn nomadik_read_sched_clock() -> u64 {
    if mtu_base.is_null() { return 0; }
    (0u32.wrapping_sub(readl((mtu_base as *mut u8).add(mtu_val(0)) as *const c_void))) as u64
}

unsafe extern "C" fn nmdk_timer_read_current_timer() -> usize { (!readl_relaxed((mtu_base as *mut u8).add(mtu_val(0)) as *const c_void)) as usize }

unsafe extern "C" fn nmdk_clkevt_next(evt: usize, _ev: *mut clock_event_device) -> i32 {
    writel(1 << 1, (mtu_base as *mut u8).add(MTU_IMSC) as *mut c_void);
    writel(evt as u32, (mtu_base as *mut u8).add(mtu_lr(1)) as *mut c_void);
    writel(MTU_CRn_ONESHOT | clk_prescale | MTU_CRn_32BITS | MTU_CRn_ENA, (mtu_base as *mut u8).add(mtu_cr(1)) as *mut c_void);
    0
}

unsafe fn nmdk_clkevt_reset() {
    if clkevt_periodic {
        writel(nmdk_cycle, (mtu_base as *mut u8).add(mtu_lr(1)) as *mut c_void);
        writel(nmdk_cycle, (mtu_base as *mut u8).add(mtu_bglr(1)) as *mut c_void);
        writel(MTU_CRn_PERIODIC | clk_prescale | MTU_CRn_32BITS | MTU_CRn_ENA, (mtu_base as *mut u8).add(mtu_cr(1)) as *mut c_void);
        writel(1 << 1, (mtu_base as *mut u8).add(MTU_IMSC) as *mut c_void);
    } else { nmdk_clkevt_next(nmdk_cycle as usize, core::ptr::null_mut()); }
}

unsafe extern "C" fn nmdk_clkevt_shutdown(_evt: *mut clock_event_device) -> i32 { writel(0, (mtu_base as *mut u8).add(MTU_IMSC) as *mut c_void); writel(0, (mtu_base as *mut u8).add(mtu_cr(1)) as *mut c_void); writel(0xffffffff, (mtu_base as *mut u8).add(mtu_lr(1)) as *mut c_void); 0 }
unsafe extern "C" fn nmdk_clkevt_set_oneshot(_evt: *mut clock_event_device) -> i32 { clkevt_periodic = false; 0 }
unsafe extern "C" fn nmdk_clkevt_set_periodic(_evt: *mut clock_event_device) -> i32 { clkevt_periodic = true; nmdk_clkevt_reset(); 0 }
unsafe fn nmdk_clksrc_reset() { writel(0, (mtu_base as *mut u8).add(mtu_cr(0)) as *mut c_void); writel(nmdk_cycle, (mtu_base as *mut u8).add(mtu_lr(0)) as *mut c_void); writel(nmdk_cycle, (mtu_base as *mut u8).add(mtu_bglr(0)) as *mut c_void); writel(clk_prescale | MTU_CRn_32BITS | MTU_CRn_ENA, (mtu_base as *mut u8).add(mtu_cr(0)) as *mut c_void); }
unsafe extern "C" fn nmdk_clkevt_resume(_cedev: *mut clock_event_device) { nmdk_clkevt_reset(); nmdk_clksrc_reset(); }

static mut nmdk_clkevt: clock_event_device = clock_event_device { name: b"mtu_1\0".as_ptr() as *const i8, features: 0, rating: 200, set_state_shutdown: Some(nmdk_clkevt_shutdown), set_state_periodic: Some(nmdk_clkevt_set_periodic), set_state_oneshot: Some(nmdk_clkevt_set_oneshot), set_next_event: Some(nmdk_clkevt_next), resume: Some(nmdk_clkevt_resume), cpumask: core::ptr::null(), irq: 0, event_handler: None };

unsafe extern "C" fn nmdk_timer_interrupt(_irq: i32, dev_id: *mut c_void) -> irqreturn_t { writel(1 << 1, (mtu_base as *mut u8).add(MTU_ICR) as *mut c_void); let evdev = dev_id as *mut clock_event_device; if let Some(handler) = (*evdev).event_handler { handler(evdev); } 1 }

// Device-tree initialization and timer setup are retained below as the direct kernel-facing entry points.
unsafe extern "C" fn nmdk_timer_init(base: *mut c_void, irq: i32, pclk: *mut clk, clk: *mut clk) -> i32 {
    mtu_base = base;
    if clk_prepare_enable(pclk) != 0 || clk_prepare_enable(clk) != 0 { return -1; }
    let mut rate = clk_get_rate(clk);
    if rate > 32000000 { rate /= 16; clk_prescale = MTU_CRn_PRESCALE_16; } else { clk_prescale = MTU_CRn_PRESCALE_1; }
    nmdk_cycle = ((rate + 100 / 2) / 100) as u32;
    nmdk_clksrc_reset();
    let ret = clocksource_mmio_init((mtu_base as *mut u8).add(mtu_val(0)) as *mut c_void, b"mtu_0\0".as_ptr() as *const i8, rate, 200, 32, Some(clocksource_mmio_readl_down));
    if ret != 0 { pr_err(b"timer: failed to initialize clock source %s\n\0".as_ptr() as *const i8, b"mtu_0\0".as_ptr() as *const i8); return ret; }
    sched_clock_register(Some(nomadik_read_sched_clock), 32, rate);
    if request_irq(irq, Some(nmdk_timer_interrupt), 0, b"Nomadik Timer Tick\0".as_ptr() as *const i8, &mut nmdk_clkevt as *mut _ as *mut c_void) != 0 { pr_err(b"%s: request_irq() failed\n\0".as_ptr() as *const i8, b"Nomadik Timer Tick\0".as_ptr() as *const i8); }
    nmdk_clkevt.cpumask = cpumask_of(0); nmdk_clkevt.irq = irq;
    let min_ticks = if rate < 100000 { 5 } else { 2 };
    clockevents_config_and_register(&mut nmdk_clkevt, rate, min_ticks, 0xffffffff);
    mtu_delay_timer.read_current_timer = Some(nmdk_timer_read_current_timer); mtu_delay_timer.freq = rate; register_current_timer_delay(&mut mtu_delay_timer); 0
}

unsafe extern "C" fn nmdk_timer_of_init(node: *mut device_node) -> i32 {
    let base = of_iomap(node, 0); if base.is_null() { pr_err(b"Can't remap registers\n\0".as_ptr() as *const i8); return -6; }
    let pclk = of_clk_get_by_name(node, b"apb_pclk\0".as_ptr() as *const i8); if pclk.is_null() { pr_err(b"could not get apb_pclk\n\0".as_ptr() as *const i8); return -1; }
    let clk = of_clk_get_by_name(node, b"timclk\0".as_ptr() as *const i8); if clk.is_null() { pr_err(b"could not get timclk\n\0".as_ptr() as *const i8); return -1; }
    let irq = irq_of_parse_and_map(node, 0); if irq <= 0 { pr_err(b"Can't parse IRQ\n\0".as_ptr() as *const i8); return -22; }
    nmdk_timer_init(base, irq, pclk, clk)
}

// TIMER_OF_DECLARE(nomadik_mtu, "st,nomadik-mtu", nmdk_timer_of_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
