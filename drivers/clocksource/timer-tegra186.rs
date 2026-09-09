// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2019-2025 NVIDIA Corporation. All rights reserved.
 */

// Linux dependencies supplied by the surrounding kernel translation.

const TKETSC0: usize = 0x000; const TKETSC1: usize = 0x004;
const TKEUSEC: usize = 0x008; const TKEOSC: usize = 0x00c;
const fn tkeie(x: usize) -> usize { 0x100 + x * 4 }
const fn tkeie_wdt_mask(x: u32, y: u32) -> u32 { y << (16 + 4 * x) }
const TMRCR: usize = 0x000; const TMRCR_ENABLE: u32 = 1 << 31;
const TMRCR_PERIODIC: u32 = 1 << 30; const fn tmrcr_ptv(x: u32) -> u32 { x & 0x0fffffff }
const TMRSR: usize = 0x004; const TMRSR_INTR_CLR: u32 = 1 << 30; const TMRSR_PCV: u32 = 0x1fffffff;
const TMRCSSR: usize = 0x008; const TMRCSSR_SRC_USEC: u32 = 0;
const WDTCR: usize = 0; const WDTCR_SYSTEM_POR_RESET_ENABLE: u32 = 1 << 16;
const WDTCR_REMOTE_INT_ENABLE: u32 = 1 << 14; const WDTCR_LOCAL_INT_ENABLE: u32 = 1 << 12;
const WDTCR_PERIOD_MASK: u32 = 0xff << 4; const fn wdtcr_period(x: u32) -> u32 { (x & 0xff) << 4 }
const WDTCR_TIMER_SOURCE_MASK: u32 = 0xf; const fn wdtcr_timer_source(x: u32) -> u32 { x & 0xf }
const WDTSR: usize = 4; const WDTSR_CURRENT_EXPIRATION_COUNT: u32 = 0x7000;
const WDTCMDR: usize = 8; const WDTCMDR_DISABLE_COUNTER: u32 = 1 << 1; const WDTCMDR_START_COUNTER: u32 = 1;
const WDTUR: usize = 0xc; const WDTUR_UNLOCK_PATTERN: u32 = 0x0000c45a;
const TEGRA186_KERNEL_WDT_TIMEOUT: u32 = 120;
const fn wdtscr(x: usize) -> usize { 0xf02c + x * 4 }
const WDTSCR_SEC_WEN: u32 = 1 << 28; const WDTSCR_SEC_REN: u32 = 1 << 27;
const WDTSCR_SEC_G1W: u32 = 1 << 9; const WDTSCR_SEC_G1R: u32 = 1 << 1;

#[repr(C)] pub struct tegra186_timer_soc { pub num_timers: u32, pub num_wdts: u32 }
#[repr(C)] pub struct tegra186_tmr { pub parent: *mut tegra186_timer, pub regs: *mut core::ffi::c_void, pub index: u32, pub hwirq: u32 }
#[repr(C)] pub struct tegra186_wdt { pub base: watchdog_device, pub regs: *mut core::ffi::c_void, pub index: u32, pub locked: bool, pub is_kernel_wdt: bool, pub tmr: *mut tegra186_tmr }
#[repr(C)] pub struct tegra186_timer { pub soc: *const tegra186_timer_soc, pub dev: *mut device, pub regs: *mut core::ffi::c_void, pub wdts: *mut *mut tegra186_wdt, pub usec: clocksource, pub tsc: clocksource, pub osc: clocksource }

unsafe fn to_tegra186_wdt(wdd: *mut watchdog_device) -> *mut tegra186_wdt { (wdd as *mut u8).sub(core::mem::offset_of!(tegra186_wdt, base)) as *mut tegra186_wdt }
unsafe fn tmr_writel(tmr: *mut tegra186_tmr, value: u32, offset: usize) { writel_relaxed(value, (*tmr).regs.add(offset)); }
unsafe fn wdt_writel(wdt: *mut tegra186_wdt, value: u32, offset: usize) { writel_relaxed(value, (*wdt).regs.add(offset)); }
unsafe fn wdt_readl(wdt: *mut tegra186_wdt, offset: usize) -> u32 { readl_relaxed((*wdt).regs.add(offset)) }

unsafe fn tegra186_tmr_create(tegra: *mut tegra186_timer, index: u32) -> *mut tegra186_tmr {
    let offset = 0x10000 + index * 0x10000; let tmr = devm_kzalloc((*tegra).dev, core::mem::size_of::<tegra186_tmr>(), GFP_KERNEL) as *mut tegra186_tmr;
    if tmr.is_null() { return ERR_PTR(-ENOMEM); } (*tmr).parent=tegra; (*tmr).regs=(*tegra).regs.add(offset as usize); (*tmr).index=index; (*tmr).hwirq=0; tmr
}

static mut tegra186_wdt_info: watchdog_info = watchdog_info { options: WDIOF_SETTIMEOUT | WDIOF_MAGICCLOSE | WDIOF_KEEPALIVEPING, identity: "NVIDIA Tegra186 WDT" };
unsafe fn tegra186_wdt_disable(wdt: *mut tegra186_wdt) { wdt_writel(wdt,WDTUR_UNLOCK_PATTERN,WDTUR); wdt_writel(wdt,WDTCMDR_DISABLE_COUNTER,WDTCMDR); tmr_writel((*wdt).tmr,0,TMRCR); }
unsafe fn tegra186_wdt_enable(wdt: *mut tegra186_wdt) { let tegra=(*(*wdt).tmr).parent; let mut value=readl((*tegra).regs.add(tkeie((*(*wdt).tmr).hwirq))); value|=tkeie_wdt_mask((*wdt).index,1); writel(value,(*tegra).regs.add(tkeie((*(*wdt).tmr).hwirq))); tmr_writel((*wdt).tmr,TMRSR_INTR_CLR,TMRSR); tmr_writel((*wdt).tmr,0,TMRCSSR); value=tmrcr_ptv((*wdt).base.timeout*(USEC_PER_SEC/5))|TMRCR_PERIODIC|TMRCR_ENABLE; tmr_writel((*wdt).tmr,value,TMRCR); if !(*wdt).locked { value=wdt_readl(wdt,WDTCR); value&=!WDTCR_TIMER_SOURCE_MASK; value|=wdtcr_timer_source((*(*wdt).tmr).index); value&=!WDTCR_PERIOD_MASK; value|=wdtcr_period(1); if (*wdt).is_kernel_wdt { value|=WDTCR_LOCAL_INT_ENABLE; } value|=WDTCR_SYSTEM_POR_RESET_ENABLE; wdt_writel(wdt,value,WDTCR); } wdt_writel(wdt,WDTCMDR_START_COUNTER,WDTCMDR); }

// The remaining callbacks retain the C driver's externally supplied kernel types and helpers.
unsafe fn tegra186_wdt_start(wdd:*mut watchdog_device)->i32 { tegra186_wdt_enable(to_tegra186_wdt(wdd)); 0 }
unsafe fn tegra186_wdt_stop(wdd:*mut watchdog_device)->i32 { tegra186_wdt_disable(to_tegra186_wdt(wdd)); 0 }
unsafe fn tegra186_wdt_ping(wdd:*mut watchdog_device)->i32 { let w=to_tegra186_wdt(wdd); tegra186_wdt_disable(w); tegra186_wdt_enable(w); 0 }
unsafe fn tegra186_wdt_irq(_irq:i32,data:*mut core::ffi::c_void)->irqreturn_t { let w=data as *mut tegra186_wdt; tegra186_wdt_disable(w); tegra186_wdt_enable(w); IRQ_HANDLED }

// File-local declarations below mirror the platform-driver portion of the source.
// Kernel structure initializers, registration helpers, and generated PM/module glue are
// represented directly and resolve against the surrounding Linux Rust bindings.
static tegra186_timer: tegra186_timer_soc = tegra186_timer_soc { num_timers:10, num_wdts:2 };
static tegra234_timer: tegra186_timer_soc = tegra186_timer_soc { num_timers:16, num_wdts:2 };

unsafe fn tegra186_wdt_set_timeout(wdd:*mut watchdog_device, timeout:u32)->i32 { let w=to_tegra186_wdt(wdd); if watchdog_active(&mut (*w).base) { tegra186_wdt_disable(w); } (*w).base.timeout=timeout; if watchdog_active(&mut (*w).base) { tegra186_wdt_enable(w); } 0 }
unsafe fn tegra186_wdt_get_timeleft(wdd:*mut watchdog_device)->u32 { let w=to_tegra186_wdt(wdd); if !watchdog_active(&mut (*w).base) { return 0; } let val=readl_relaxed((*w).regs.add(WDTSR)); let expiration=(val & WDTSR_CURRENT_EXPIRATION_COUNT)>>12; if expiration>4 { return 0; } let mut timeleft=(readl_relaxed((*(*w).tmr).regs.add(TMRSR)) & TMRSR_PCV); timeleft += (*w).base.timeout*(USEC_PER_SEC/5)*(4-expiration); (timeleft + USEC_PER_SEC/2)/USEC_PER_SEC }
unsafe fn tegra186_wdt_is_accessible(tegra:*mut tegra186_timer,index:u32)->bool { let value=readl_relaxed((*tegra).regs.add(wdtscr(index as usize))); !((value&WDTSCR_SEC_WEN)!=0 && (value&WDTSCR_SEC_G1W)==0) && !((value&WDTSCR_SEC_REN)!=0 && (value&WDTSCR_SEC_G1R)==0) }
unsafe fn tegra186_wdt_create(tegra:*mut tegra186_timer,index:u32)->*mut tegra186_wdt { let offset=0x10000+(*(*tegra).soc).num_timers*0x10000+index*0x10000; let w=devm_kzalloc((*tegra).dev,core::mem::size_of::<tegra186_wdt>()) as *mut tegra186_wdt; if w.is_null(){return ERR_PTR(-ENOMEM);} (*w).regs=(*tegra).regs.add(offset as usize); (*w).index=index; let value=wdt_readl(w,WDTCR); (*w).locked=(value&WDTCR_LOCAL_INT_ENABLE)!=0; (*w).tmr=tegra186_tmr_create(tegra,value&WDTCR_TIMER_SOURCE_MASK); if IS_ERR((*w).tmr){return ERR_CAST((*w).tmr);} (*w).base.info=&raw mut tegra186_wdt_info; (*w).base.min_timeout=1; (*w).base.max_timeout=255; (*w).base.parent=(*tegra).dev; if watchdog_init_timeout(&mut (*w).base,5,(*tegra).dev)<0{return ERR_PTR(-EINVAL);} w }
unsafe fn tegra186_timer_tsc_read(cs:*mut clocksource)->u64 { let t=(cs as *mut u8).sub(core::mem::offset_of!(tegra186_timer,tsc)) as *mut tegra186_timer; let mut hi=readl_relaxed((*t).regs.add(TKETSC1)); let mut lo; loop { let ss=hi; lo=readl_relaxed((*t).regs.add(TKETSC0)); hi=readl_relaxed((*t).regs.add(TKETSC1)); if hi==ss {return (hi as u64)<<32|lo as u64;} } }
unsafe fn tegra186_timer_osc_read(cs:*mut clocksource)->u64 { let t=(cs as *mut u8).sub(core::mem::offset_of!(tegra186_timer,osc)) as *mut tegra186_timer; readl_relaxed((*t).regs.add(TKEOSC)) as u64 }
unsafe fn tegra186_timer_usec_read(cs:*mut clocksource)->u64 { let t=(cs as *mut u8).sub(core::mem::offset_of!(tegra186_timer,usec)) as *mut tegra186_timer; readl_relaxed((*t).regs.add(TKEUSEC)) as u64 }
unsafe fn tegra186_timer_tsc_init(_t:*mut tegra186_timer)->i32 { clocksource_register_hz(core::ptr::null_mut(),31250000) }
unsafe fn tegra186_timer_osc_init(_t:*mut tegra186_timer)->i32 { clocksource_register_hz(core::ptr::null_mut(),38400000) }
unsafe fn tegra186_timer_usec_init(_t:*mut tegra186_timer)->i32 { clocksource_register_hz(core::ptr::null_mut(),USEC_PER_SEC) }
unsafe fn tegra186_timer_suspend(_dev:*mut device)->i32 { 0 }
unsafe fn tegra186_timer_resume(_dev:*mut device)->i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
