// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  arch/m68k/bvme6000/config.c
 *
 *  Copyright (C) 1997 Richard Hirst [richard@sleepie.demon.co.uk]
 *
 * Based on:
 *
 *  linux/amiga/config.c
 *
 *  Copyright (C) 1993 Hamish Macdonald
 */

// C dependencies supplied by the surrounding kernel translation unit.

unsafe extern "C" {
    fn bvme6000_sched_init();
    fn bvme6000_hwclk(op: i32, t: *mut rtc_time) -> i32;
    fn bvme6000_reset();
    fn bvme6000_set_vectors();
}

unsafe extern "C" {
    static mut m68k_cputype: i32;
    static mut vme_brdtype: i32;
    static mut config_reg_ptr: *mut u8;
    static mut bvme_acr_addrctl: u32;
    static mut vectors: *mut core::ffi::c_void;
    static mut mach_sched_init: Option<unsafe extern "C" fn()>;
    static mut mach_init_IRQ: Option<unsafe extern "C" fn()>;
    static mut mach_hwclk: Option<unsafe extern "C" fn(i32, *mut rtc_time) -> i32>;
    static mut mach_reset: Option<unsafe extern "C" fn()>;
    static mut mach_get_model: Option<unsafe extern "C" fn(*mut u8)>;
}

#[repr(C)]
pub struct bi_record {
    pub tag: u16,
}

#[repr(C)]
pub struct rtc_time {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
}

#[repr(C)]
pub struct PitRegs {
    pub pgcr: u8, pub psrr: u8, pub pacr: u8, pub padr: u8, pub paddr: u8,
    pub pbcr: u8, pub pbdr: u8, pub pbddr: u8, pub pcdr: u8, pub pcddr: u8,
}

#[repr(C)]
pub struct RtcRegs {
    pub msr: u8, pub t1cr_omr: u8, pub t1msb: u8, pub t1lsb: u8,
    pub irr_icr1: u8, pub pfr_icr0: u8, pub t0cr_rtmr: u8,
    pub bcd_tenms: u8, pub bcd_sec: u8, pub bcd_min: u8, pub bcd_hr: u8,
    pub bcd_dom: u8, pub bcd_mth: u8, pub bcd_year: u8, pub bcd_dow: u8,
}

type PitRegsPtr = *mut PitRegs;
type RtcPtr_t = *mut RtcRegs;

extern "C" {
    fn m68k_setup_user_interrupt(vector: i32, count: i32);
    fn pr_info(fmt: *const u8, ...);
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn legacy_timer_tick(n: i32);
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32, flags: u32, name: *const u8, dev_id: *mut core::ffi::c_void) -> i32;
    fn panic(fmt: *const u8, ... ) -> !;
    fn clocksource_register_hz(cs: *mut clocksource, hz: u32) -> i32;
    fn bin2bcd(value: i32) -> u8;
    fn bcd2bin(value: u8) -> i32;
}

#[repr(C)]
pub struct clocksource {
    pub name: *const u8,
    pub rating: i32,
    pub read: Option<unsafe extern "C" fn(*mut clocksource) -> u64>,
    pub mask: u64,
    pub flags: u32,
}

const RTC_TIMER_CLOCK_FREQ: u32 = 8_000_000;
const RTC_TIMER_CYCLES: u32 = RTC_TIMER_CLOCK_FREQ / HZ;
const RTC_TIMER_COUNT: u32 = (RTC_TIMER_CYCLES / 2) - 1;

unsafe extern "C" fn bvme6000_get_model(model: *mut u8) {
    sprintf(model, b"BVME%d000\0".as_ptr(), if m68k_cputype == CPU_68060 { 6 } else { 4 });
}

extern "C" { fn sprintf(dst: *mut u8, fmt: *const u8, ...) -> i32; }

#[no_mangle]
pub unsafe extern "C" fn bvme6000_parse_bootinfo(bi: *const bi_record) -> i32 {
    if u16::from_be((*bi).tag) == BI_VME_TYPE { 0 } else { 1 }
}

#[no_mangle]
pub unsafe extern "C" fn bvme6000_reset() {
    let pit = BVME_PIT_BASE as PitRegsPtr;
    pr_info(b"\r\n\nCalled bvme6000_reset\r\n\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\0".as_ptr());
    (*pit).pcddr |= 0x10;
    loop {}
}

unsafe extern "C" fn bvme6000_init_IRQ() { m68k_setup_user_interrupt(VEC_USER, 192); }

#[no_mangle]
pub unsafe extern "C" fn config_bvme6000() {
    let pit = BVME_PIT_BASE as PitRegsPtr;
    if vme_brdtype == 0 { vme_brdtype = if m68k_cputype == CPU_68060 { VME_TYPE_BVME6000 } else { VME_TYPE_BVME4000 }; }
    mach_sched_init = Some(bvme6000_sched_init);
    mach_init_IRQ = Some(bvme6000_init_IRQ);
    mach_hwclk = Some(bvme6000_hwclk);
    mach_reset = Some(bvme6000_reset);
    mach_get_model = Some(bvme6000_get_model);
    pr_info(b"Board is %sconfigured as a System Controller\n\0".as_ptr(), if *config_reg_ptr & BVME_CONFIG_SW1 != 0 { b"\0".as_ptr() } else { b"not \0".as_ptr() });
    (*pit).pgcr = 0; (*pit).psrr = 0x18; (*pit).pacr = 0; (*pit).padr = 0; (*pit).paddr = 0;
    (*pit).pbcr = 0x80; (*pit).pbdr = 0xbc | if *config_reg_ptr & BVME_CONFIG_SW1 != 0 { 0 } else { 0x40 };
    (*pit).pbddr = 0xf3; (*pit).pcdr = 1; (*pit).pcddr = 3;
    bvme_acr_addrctl = 0;
}

unsafe extern "C" fn bvme6000_abort_int(_irq: i32, _dev_id: *mut core::ffi::c_void) -> i32 {
    let new = vectors as *mut usize; let old = 0xf8000000usize as *mut usize;
    while *(BVME_LOCAL_IRQ_STAT as *const u8) & BVME_ABORT_STATUS != 0 {}
    *new.add(4) = *old.add(4); *new.add(9) = *old.add(9); *new.add(47) = *old.add(47); *new.add(0x1f) = *old.add(0x1f);
    IRQ_HANDLED
}

static mut clk_total: u32 = 0;
static mut clk_offset: u32 = 0;

static mut bvme6000_clk: clocksource = clocksource { name: b"rtc\0".as_ptr(), rating: 250, read: Some(bvme6000_read_clk), mask: 0xffff_ffff, flags: CLOCK_SOURCE_IS_CONTINUOUS };

#[no_mangle]
pub unsafe extern "C" fn bvme6000_sched_init() {
    let rtc=BVME_RTC_BASE as RtcPtr_t; let msr=(*rtc).msr&0xc0; (*rtc).msr=0;
    if request_irq(BVME_IRQ_RTC,bvme6000_timer_int,IRQF_TIMER,b"timer\0".as_ptr(),core::ptr::null_mut()) != 0 { panic(b"Couldn't register timer int\0".as_ptr()); }
    (*rtc).t1cr_omr=4; (*rtc).t1msb=(RTC_TIMER_COUNT>>8) as u8; (*rtc).t1lsb=(RTC_TIMER_COUNT&0xff) as u8; (*rtc).irr_icr1&=!0x10; (*rtc).msr=0x40; (*rtc).pfr_icr0=0x80; (*rtc).irr_icr1=0; (*rtc).t1cr_omr=0x0a; (*rtc).t0cr_rtmr&=!0x20; (*rtc).msr=0; (*rtc).t1cr_omr=5; (*rtc).msr=msr;
    clocksource_register_hz(&mut bvme6000_clk,RTC_TIMER_CLOCK_FREQ);
    if request_irq(BVME_IRQ_ABORT,bvme6000_abort_int,0,b"abort\0".as_ptr(),bvme6000_abort_int as *mut core::ffi::c_void) != 0 { panic(b"Couldn't register abort int\0".as_ptr()); }
}

unsafe extern "C" fn bvme6000_timer_int(_irq: i32, _dev_id: *mut core::ffi::c_void) -> i32 {
    let mut flags = 0usize; let rtc = BVME_RTC_BASE as RtcPtr_t;
    local_irq_save(&mut flags); let msr = (*rtc).msr & 0xc0; (*rtc).msr = msr | 0x20;
    clk_total = clk_total.wrapping_add(RTC_TIMER_CYCLES); clk_offset = 0; legacy_timer_tick(1); local_irq_restore(flags); IRQ_HANDLED
}

unsafe extern "C" fn bvme6000_read_clk(_cs: *mut clocksource) -> u64 {
    let mut flags = 0usize; let rtc = BVME_RTC_BASE as RtcPtr_t; let pit = BVME_PIT_BASE as PitRegsPtr;
    local_irq_save(&mut flags); let msr = (*rtc).msr & 0xc0; (*rtc).msr = 0; let mut v = 800000u32; let mut ov;
    loop { ov=v; let t1int=(*rtc).msr&0x20; let t1op=(*pit).pcdr&4; (*rtc).t1cr_omr|=0x40; let msb=(*rtc).t1msb; v=((msb as u32)<<8)|(*rtc).t1lsb as u32; if t1int==((*rtc).msr&0x20)&&t1op==((*pit).pcdr&4)&&(ov.abs_diff(v)<=80)&&v<=RTC_TIMER_COUNT-(RTC_TIMER_COUNT/100) { if !t1op { v+=RTC_TIMER_CYCLES/2; } if msb>0&&t1int!=0 { clk_offset=RTC_TIMER_CYCLES; } break; } }
    v=RTC_TIMER_COUNT-v; (*rtc).msr=msr; let result=v+clk_offset+clk_total; local_irq_restore(flags); result as u64
}

pub unsafe fn bvme6000_hwclk(op: i32, t: *mut rtc_time) -> i32 {
    let rtc=BVME_RTC_BASE as RtcPtr_t; let msr=(*rtc).msr&0xc0; (*rtc).msr=0x40;
    if op!=0 { (*rtc).t0cr_rtmr=((*t).tm_year%4) as u8; (*rtc).bcd_tenms=0; (*rtc).bcd_sec=bin2bcd((*t).tm_sec); (*rtc).bcd_min=bin2bcd((*t).tm_min); (*rtc).bcd_hr=bin2bcd((*t).tm_hour); (*rtc).bcd_dom=bin2bcd((*t).tm_mday); (*rtc).bcd_mth=bin2bcd((*t).tm_mon+1); (*rtc).bcd_year=bin2bcd((*t).tm_year%100); if (*t).tm_wday>=0 { (*rtc).bcd_dow=bin2bcd((*t).tm_wday+1); } (*rtc).t0cr_rtmr=(((*t).tm_year%4)|8) as u8; }
    else { loop { (*t).tm_sec=bcd2bin((*rtc).bcd_sec); (*t).tm_min=bcd2bin((*rtc).bcd_min); (*t).tm_hour=bcd2bin((*rtc).bcd_hr); (*t).tm_mday=bcd2bin((*rtc).bcd_dom); (*t).tm_mon=bcd2bin((*rtc).bcd_mth)-1; (*t).tm_year=bcd2bin((*rtc).bcd_year); if (*t).tm_year<70 { (*t).tm_year+=100; } (*t).tm_wday=bcd2bin((*rtc).bcd_dow)-1; if (*t).tm_sec==bcd2bin((*rtc).bcd_sec) { break; } } }
    (*rtc).msr=msr; 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
