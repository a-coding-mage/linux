// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  arch/m68k/mvme147/config.c
 *
 *  Copyright (C) 1996 Dave Frascone [chaos@mindspring.com]
 *  Cloned from        Richard Hirst [richard@sleepie.demon.co.uk]
 *
 * Based on:
 *
 *  Copyright (C) 1993 Hamish Macdonald
 */

// C dependencies supplied by the surrounding kernel translation unit.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    static mut m147_pcc: *mut M147Pcc;
    static mut vme_brdtype: u32;
    static mut mach_sched_init: Option<unsafe extern "C" fn()>;
    static mut mach_init_IRQ: Option<unsafe extern "C" fn()>;
    static mut mach_reset: Option<unsafe extern "C" fn()>;
    static mut mach_get_model: Option<unsafe extern "C" fn(*mut c_char)>;
    fn be16_to_cpu(x: u16) -> u16;
    fn m68k_setup_user_interrupt(vec: c_int, count: c_int);
    fn pr_info(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...);
    fn platform_device_register_resndata(
        parent: *mut c_void, name: *const c_char, id: c_int,
        res: *mut Resource, num: usize, data: *mut c_void, size: usize,
    ) -> c_int;
    fn request_irq(irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> IrqReturn,
                   flags: c_ulong, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn local_irq_save(flags: *mut c_ulong);
    fn local_irq_restore(flags: c_ulong);
    fn legacy_timer_tick(n: c_int);
    fn clocksource_register_hz(cs: *mut Clocksource, hz: u32) -> c_int;
    fn in_8(addr: *const u8) -> u8;
    fn out_8(addr: *mut u8, value: u8);
}

#[repr(C)] pub struct BiRecord { pub tag: u16 }
#[repr(C)] pub struct M147Pcc { pub watchdog: u8, pub t1_cntrl: u8, pub t1_int_cntrl: u8, pub t1_preload: u16, pub t1_count: u16 }
#[repr(C)] pub struct Resource { pub start: usize, pub end: usize, pub flags: usize }
#[repr(C)] pub struct M48t59PlatData { pub r#type: u32, pub yy_offset: u32 }
#[repr(C)] pub struct Clocksource { pub name: *const c_char, pub rating: c_int, pub read: Option<unsafe extern "C" fn(*mut Clocksource) -> u64>, pub mask: u64, pub flags: u32 }
#[repr(transparent)] pub struct IrqReturn(pub c_int);

const BI_VME_TYPE: u16 = 0;
const BI_VME_BRDINFO: u16 = 1;
const VEC_USER: c_int = 64;
const VME_TYPE_MVME147: u32 = 1;
const MVME147_RTC_BASE: usize = 0;
const M48T59RTC_TYPE_M48T02: u32 = 0;
const PCC_TIMER_CLOCK_FREQ: u32 = 160000;
const HZ: u32 = 100;
const PCC_TIMER_CYCLES: u16 = (PCC_TIMER_CLOCK_FREQ / HZ) as u16;
const PCC_TIMER_PRELOAD: u16 = 0x10000u32.wrapping_sub(PCC_TIMER_CYCLES as u32) as u16;
const PCC_TIMER_CLR_OVF: u8 = 1;
const PCC_TIMER_COC_EN: u8 = 2;
const PCC_TIMER_TIC_EN: u8 = 4;
const PCC_INT_ENAB: u8 = 1;
const PCC_TIMER_INT_CLR: u8 = 2;
const PCC_LEVEL_TIMER1: u8 = 4;
const PCC_IRQ_TIMER1: c_int = 0;
const IRQF_TIMER: c_ulong = 0;
const CLOCK_SOURCE_IS_CONTINUOUS: u32 = 1;
const M147_SCC_A_ADDR: *mut u8 = 0 as *mut u8;
const IRQ_HANDLED: IrqReturn = IrqReturn(1);

unsafe extern "C" fn mvme147_parse_bootinfo(bi: *const BiRecord) -> c_int {
    let tag = be16_to_cpu((*bi).tag);
    if tag == BI_VME_TYPE || tag == BI_VME_BRDINFO { 0 } else { 1 }
}

#[no_mangle] pub unsafe extern "C" fn mvme147_reset() {
    pr_info(b"\r\n\nCalled mvme147_reset\r\n\0".as_ptr() as *const c_char);
    (*m147_pcc).watchdog = 0x0a;
    (*m147_pcc).watchdog = 0xa5;
    loop {}
}

unsafe extern "C" fn mvme147_get_model(model: *mut c_char) {
    sprintf(model, b"Motorola MVME147\0".as_ptr() as *const c_char);
}

unsafe extern "C" fn mvme147_init_IRQ() { m68k_setup_user_interrupt(VEC_USER, 192); }

#[no_mangle] pub unsafe extern "C" fn config_mvme147() {
    mach_sched_init = Some(mvme147_sched_init);
    mach_init_IRQ = Some(mvme147_init_IRQ);
    mach_reset = Some(mvme147_reset);
    mach_get_model = Some(mvme147_get_model);
    if vme_brdtype == 0 { vme_brdtype = VME_TYPE_MVME147; }
}

static mut M48T59_RSRC: [Resource; 1] = [Resource { start: MVME147_RTC_BASE, end: MVME147_RTC_BASE + 0x800 - 1, flags: 0 }];
static mut M48T59_DATA: M48t59PlatData = M48t59PlatData { r#type: M48T59RTC_TYPE_M48T02, yy_offset: 70 };

unsafe extern "C" fn mvme147_platform_init() -> c_int {
    if !MACH_IS_MVME147() { return 0; }
    platform_device_register_resndata(core::ptr::null_mut(), b"rtc-m48t59\0".as_ptr() as *const c_char, -1, M48T59_RSRC.as_mut_ptr(), 1, &mut M48T59_DATA as *mut _ as *mut c_void, core::mem::size_of::<M48t59PlatData>());
    0
}

unsafe extern "C" fn MACH_IS_MVME147() -> bool { false }

static mut MVME147_CLK: Clocksource = Clocksource { name: b"pcc\0".as_ptr() as *const c_char, rating: 250, read: Some(mvme147_read_clk), mask: u32::MAX as u64, flags: CLOCK_SOURCE_IS_CONTINUOUS };
static mut CLK_TOTAL: u32 = 0;

unsafe extern "C" fn mvme147_timer_int(_irq: c_int, _dev_id: *mut c_void) -> IrqReturn {
    let mut flags = 0;
    local_irq_save(&mut flags);
    (*m147_pcc).t1_cntrl = PCC_TIMER_CLR_OVF | PCC_TIMER_COC_EN | PCC_TIMER_TIC_EN;
    (*m147_pcc).t1_int_cntrl = PCC_INT_ENAB | PCC_TIMER_INT_CLR | PCC_LEVEL_TIMER1;
    CLK_TOTAL = CLK_TOTAL.wrapping_add(PCC_TIMER_CYCLES as u32);
    legacy_timer_tick(1);
    local_irq_restore(flags);
    IRQ_HANDLED
}

unsafe extern "C" fn mvme147_sched_init() {
    if request_irq(PCC_IRQ_TIMER1, mvme147_timer_int, IRQF_TIMER, b"timer 1\0".as_ptr() as *const c_char, core::ptr::null_mut()) != 0 { pr_err(b"Couldn't register timer interrupt\n\0".as_ptr() as *const c_char); }
    (*m147_pcc).t1_preload = PCC_TIMER_PRELOAD;
    (*m147_pcc).t1_cntrl = PCC_TIMER_CLR_OVF | PCC_TIMER_COC_EN | PCC_TIMER_TIC_EN;
    (*m147_pcc).t1_int_cntrl = PCC_INT_ENAB | PCC_TIMER_INT_CLR | PCC_LEVEL_TIMER1;
    clocksource_register_hz(&mut MVME147_CLK, PCC_TIMER_CLOCK_FREQ);
}

unsafe extern "C" fn mvme147_read_clk(_cs: *mut Clocksource) -> u64 {
    let mut flags = 0; local_irq_save(&mut flags);
    let tmp = (*m147_pcc).t1_cntrl >> 4; let mut count = (*m147_pcc).t1_count; let overflow = (*m147_pcc).t1_cntrl >> 4;
    if overflow != tmp { count = (*m147_pcc).t1_count; }
    count = count.wrapping_sub(PCC_TIMER_PRELOAD);
    let mut ticks = count as u32 + overflow as u32 * PCC_TIMER_CYCLES as u32; ticks = ticks.wrapping_add(CLK_TOTAL);
    local_irq_restore(flags); ticks as u64
}

unsafe fn scc_delay() { core::arch::asm!("nop; nop;"); }
unsafe fn scc_write(ch: u8) { loop { scc_delay(); if in_8(M147_SCC_A_ADDR) & (1 << 2) != 0 { break; } } scc_delay(); out_8(M147_SCC_A_ADDR, 8); scc_delay(); out_8(M147_SCC_A_ADDR, ch); }

#[no_mangle] pub unsafe extern "C" fn mvme147_scc_write(_co: *mut c_void, mut str_: *const c_char, mut count: c_uint) {
    let mut flags = 0; local_irq_save(&mut flags);
    while count != 0 { if *str_ as u8 == b'\n' { scc_write(b'\r'); } scc_write(*str_ as u8); str_ = str_.add(1); count -= 1; }
    local_irq_restore(flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
