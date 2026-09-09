// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2014 Oleksij Rempel <linux@rempel-privat.de>
 */

// Linux kernel dependencies supplied by other translation units.

const DRIVER_NAME: &str = "asm9260-timer";

/*
 * this device provide 4 offsets for each register:
 * 0x0 - plain read write mode
 * 0x4 - set mode, OR logic.
 * 0x8 - clr mode, XOR logic.
 * 0xc - togle mode.
 */
const SET_REG: usize = 4;
const CLR_REG: usize = 8;

const HW_IR: usize = 0x0000;
const BM_IR_CR0: u32 = 1 << 4;
const BM_IR_MR3: u32 = 1 << 3;
const BM_IR_MR2: u32 = 1 << 2;
const BM_IR_MR1: u32 = 1 << 1;
const BM_IR_MR0: u32 = 1 << 0;

const HW_TCR: usize = 0x0010;
const BM_C3_RST: u32 = 1 << 7;
const BM_C2_RST: u32 = 1 << 6;
const BM_C1_RST: u32 = 1 << 5;
const BM_C0_RST: u32 = 1 << 4;
const BM_C3_EN: u32 = 1 << 3;
const BM_C2_EN: u32 = 1 << 2;
const BM_C1_EN: u32 = 1 << 1;
const BM_C0_EN: u32 = 1 << 0;

const HW_DIR: usize = 0x0020;
const BM_DIR_COUNT_UP: u32 = 0;
const BM_DIR_COUNT_DOWN: u32 = 1;
const BM_DIR0_SHIFT: u32 = 0;
const BM_DIR1_SHIFT: u32 = 4;
const BM_DIR2_SHIFT: u32 = 8;
const BM_DIR3_SHIFT: u32 = 12;
const BM_DIR_DEFAULT: u32 = (BM_DIR_COUNT_UP << BM_DIR0_SHIFT)
    | (BM_DIR_COUNT_UP << BM_DIR1_SHIFT)
    | (BM_DIR_COUNT_UP << BM_DIR2_SHIFT)
    | (BM_DIR_COUNT_UP << BM_DIR3_SHIFT);

const HW_TC0: usize = 0x0030;
const HW_TC1: usize = 0x0040;
const HW_TC2: usize = 0x0050;
const HW_TC3: usize = 0x0060;
const HW_PR: usize = 0x0070;
const BM_PR_DISABLE: u32 = 0;
const HW_PC: usize = 0x0080;
const HW_MCR: usize = 0x0090;
const HW_MR0: usize = 0x00a0;
const HW_MR1: usize = 0x00b0;
const HW_MR2: usize = 0x00C0;
const HW_MR3: usize = 0x00D0;
const HW_CTCR: usize = 0x0180;
const BM_CTCR0_SHIFT: u32 = 0;
const BM_CTCR1_SHIFT: u32 = 2;
const BM_CTCR2_SHIFT: u32 = 4;
const BM_CTCR3_SHIFT: u32 = 6;
const BM_CTCR_TM: u32 = 0;
const BM_CTCR_DEFAULT: u32 = (BM_CTCR_TM << BM_CTCR0_SHIFT)
    | (BM_CTCR_TM << BM_CTCR1_SHIFT)
    | (BM_CTCR_TM << BM_CTCR2_SHIFT)
    | (BM_CTCR_TM << BM_CTCR3_SHIFT);

const fn bm_mcr_int_en(n: u32) -> u32 { 1 << (n * 3 + 0) }
const fn bm_mcr_res_en(n: u32) -> u32 { 1 << (n * 3 + 1) }
const fn bm_mcr_stop_en(n: u32) -> u32 { 1 << (n * 3 + 2) }

#[repr(C)]
struct Asm9260TimerPriv {
    base: *mut core::ffi::c_void,
    ticks_per_jiffy: usize,
}

static mut PRIV: Asm9260TimerPriv = Asm9260TimerPriv { base: core::ptr::null_mut(), ticks_per_jiffy: 0 };

extern "C" {
    fn writel_relaxed(value: u32, address: *mut core::ffi::c_void);
}

#[repr(C)] struct ClockEventDevice { event_handler: Option<unsafe extern "C" fn(*mut ClockEventDevice)>, cpumask: *const core::ffi::c_void }

unsafe fn asm9260_timer_set_next_event(delta: usize, _evt: *mut ClockEventDevice) -> i32 {
    writel_relaxed(delta as u32, PRIV.base.add(HW_MR0));
    writel_relaxed(BM_C0_EN, PRIV.base.add(HW_TCR + SET_REG));
    0
}

unsafe fn __asm9260_timer_shutdown(_evt: *mut ClockEventDevice) {
    writel_relaxed(BM_C0_EN, PRIV.base.add(HW_TCR + CLR_REG));
}
unsafe fn asm9260_timer_shutdown(evt: *mut ClockEventDevice) -> i32 { __asm9260_timer_shutdown(evt); 0 }

unsafe fn asm9260_timer_set_oneshot(evt: *mut ClockEventDevice) -> i32 {
    __asm9260_timer_shutdown(evt);
    writel_relaxed(bm_mcr_res_en(0) | bm_mcr_stop_en(0), PRIV.base.add(HW_MCR + SET_REG));
    0
}

unsafe fn asm9260_timer_set_periodic(evt: *mut ClockEventDevice) -> i32 {
    __asm9260_timer_shutdown(evt);
    writel_relaxed(bm_mcr_res_en(0) | bm_mcr_stop_en(0), PRIV.base.add(HW_MCR + CLR_REG));
    writel_relaxed(PRIV.ticks_per_jiffy as u32, PRIV.base.add(HW_MR0));
    writel_relaxed(BM_C0_EN, PRIV.base.add(HW_TCR + SET_REG));
    0
}

static mut EVENT_DEV: ClockEventDevice = ClockEventDevice { event_handler: None, cpumask: core::ptr::null() };

unsafe fn asm9260_timer_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> i32 {
    let evt = dev_id as *mut ClockEventDevice;
    if let Some(handler) = (*evt).event_handler { handler(evt); }
    writel_relaxed(BM_IR_MR0, PRIV.base.add(HW_IR));
    1
}

extern "C" {
    fn of_io_request_and_map(np: *mut core::ffi::c_void, index: i32, name: *const i8) -> *mut core::ffi::c_void;
    fn of_clk_get(np: *mut core::ffi::c_void, index: i32) -> *mut core::ffi::c_void;
    fn clk_prepare_enable(clk: *mut core::ffi::c_void) -> i32;
    fn irq_of_parse_and_map(np: *mut core::ffi::c_void, index: i32) -> i32;
    fn request_irq(irq: i32, handler: unsafe fn(i32, *mut core::ffi::c_void) -> i32, flags: u32, name: *const i8, dev: *mut core::ffi::c_void) -> i32;
    fn clk_disable_unprepare(clk: *mut core::ffi::c_void);
    fn clk_get_rate(clk: *mut core::ffi::c_void) -> usize;
    fn clocksource_mmio_init(base: *mut core::ffi::c_void, name: *const i8, rate: usize, rating: u32, bits: u32, read: *const core::ffi::c_void) -> i32;
    fn clockevents_config_and_register(dev: *mut ClockEventDevice, rate: usize, min: u32, max: u32);
    fn cpumask_of(cpu: u32) -> *const core::ffi::c_void;
}

unsafe fn asm9260_timer_init(np: *mut core::ffi::c_void) -> i32 {
    let base = of_io_request_and_map(np, 0, core::ptr::null());
    if base.is_null() { return -1; }
    PRIV.base = base;
    let clk = of_clk_get(np, 0);
    if clk.is_null() { return -1; }
    let ret = clk_prepare_enable(clk);
    if ret != 0 { return ret; }
    let irq = irq_of_parse_and_map(np, 0);
    let ret = request_irq(irq, asm9260_timer_interrupt, 0, DRIVER_NAME.as_ptr() as *const i8, &raw mut EVENT_DEV as *mut _ as *mut _);
    if ret != 0 { clk_disable_unprepare(clk); return ret; }
    writel_relaxed(BM_DIR_DEFAULT, PRIV.base.add(HW_DIR));
    writel_relaxed(BM_PR_DISABLE, PRIV.base.add(HW_PR));
    writel_relaxed(BM_CTCR_DEFAULT, PRIV.base.add(HW_CTCR));
    writel_relaxed(bm_mcr_int_en(0), PRIV.base.add(HW_MCR));
    let rate = clk_get_rate(clk);
    clocksource_mmio_init(PRIV.base.add(HW_TC1), DRIVER_NAME.as_ptr() as *const i8, rate, 200, 32, core::ptr::null());
    writel_relaxed(0xffff_ffff, PRIV.base.add(HW_MR1));
    writel_relaxed(BM_C1_EN, PRIV.base.add(HW_TCR + SET_REG));
    PRIV.ticks_per_jiffy = (rate + 100 / 2) / 100;
    EVENT_DEV.cpumask = cpumask_of(0);
    clockevents_config_and_register(&raw mut EVENT_DEV, rate, 0x2c00, 0xffff_fffe);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
