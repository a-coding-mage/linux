// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2017 Spreadtrum Communications Inc.
 */

use core::ffi::c_void;

// Linux kernel dependencies supplied by other files.
extern "C" {
    fn readl_relaxed(addr: *mut c_void) -> u32;
    fn writel_relaxed(value: u32, addr: *mut c_void);
    fn to_timer_of(ce: *mut clock_event_device) -> *mut timer_of;
    fn timer_of_base(to: *mut timer_of) -> *mut c_void;
    fn timer_of_period(to: *mut timer_of) -> c_ulong;
    fn timer_of_rate(to: *mut timer_of) -> u32;
    fn timer_of_init(np: *mut device_node, to: *mut timer_of) -> i32;
    fn clockevent_state_oneshot(ce: *mut clock_event_device) -> bool;
    fn clockevents_config_and_register(ce: *mut clock_event_device, freq: u32, min_delta: u32, max_delta: u32);
    fn clocksource_register_hz(cs: *mut clocksource, hz: u32) -> i32;
}

type c_ulong = usize;

#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct clocksource { pub name: *const u8, pub rating: i32, pub read: Option<unsafe extern "C" fn(*mut clocksource) -> u64>, pub enable: Option<unsafe extern "C" fn(*mut clocksource) -> i32>, pub disable: Option<unsafe extern "C" fn(*mut clocksource)>, pub mask: u64, pub flags: u32 }
#[repr(C)] pub struct clock_event_device { pub name: *const u8, pub rating: i32, pub features: u32, pub set_state_shutdown: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>, pub set_state_periodic: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>, pub set_next_event: Option<unsafe extern "C" fn(c_ulong, *mut clock_event_device) -> i32>, pub cpumask: *const u8, pub event_handler: Option<unsafe extern "C" fn(*mut clock_event_device)> }
#[repr(C)] pub struct timer_of { pub flags: u32, pub clkevt: clock_event_device, pub of_irq: of_irq }
#[repr(C)] pub struct of_irq { pub handler: Option<unsafe extern "C" fn(i32, *mut c_void) -> irqreturn_t>, pub flags: u32 }
type irqreturn_t = i32;

const TIMER_NAME: &[u8] = b"sprd_timer\0";
const TIMER_LOAD_LO: usize = 0x0; const TIMER_LOAD_HI: usize = 0x4;
const TIMER_VALUE_LO: usize = 0x8; const TIMER_VALUE_HI: usize = 0xc;
const TIMER_CTL: usize = 0x10; const TIMER_CTL_PERIOD_MODE: u32 = 1 << 0;
const TIMER_CTL_ENABLE: u32 = 1 << 1; const TIMER_CTL_64BIT_WIDTH: u32 = 1 << 16;
const TIMER_INT: usize = 0x14; const TIMER_INT_EN: u32 = 1 << 0;
const TIMER_INT_RAW_STS: u32 = 1 << 1; const TIMER_INT_MASK_STS: u32 = 1 << 2; const TIMER_INT_CLR: u32 = 1 << 3;
const TIMER_VALUE_SHDW_LO: usize = 0x18; const TIMER_VALUE_SHDW_HI: usize = 0x1c;
const TIMER_VALUE_LO_MASK: u32 = 0xffff_ffff; const TIMER_VALUE_HI_MASK: u32 = 0xffff_ffff;

unsafe fn sprd_timer_enable(base: *mut c_void, flag: u32) { let mut val = readl_relaxed(base.add(TIMER_CTL)); val |= TIMER_CTL_ENABLE; if flag & TIMER_CTL_64BIT_WIDTH != 0 { val |= TIMER_CTL_64BIT_WIDTH; } else { val &= !TIMER_CTL_64BIT_WIDTH; } if flag & TIMER_CTL_PERIOD_MODE != 0 { val |= TIMER_CTL_PERIOD_MODE; } else { val &= !TIMER_CTL_PERIOD_MODE; } writel_relaxed(val, base.add(TIMER_CTL)); }
unsafe fn sprd_timer_disable(base: *mut c_void) { let mut val = readl_relaxed(base.add(TIMER_CTL)); val &= !TIMER_CTL_ENABLE; writel_relaxed(val, base.add(TIMER_CTL)); }
unsafe fn sprd_timer_update_counter(base: *mut c_void, cycles: c_ulong) { writel_relaxed(cycles as u32 & TIMER_VALUE_LO_MASK, base.add(TIMER_LOAD_LO)); writel_relaxed(0, base.add(TIMER_LOAD_HI)); }
unsafe fn sprd_timer_enable_interrupt(base: *mut c_void) { writel_relaxed(TIMER_INT_EN, base.add(TIMER_INT)); }
unsafe fn sprd_timer_clear_interrupt(base: *mut c_void) { let mut val = readl_relaxed(base.add(TIMER_INT)); val |= TIMER_INT_CLR; writel_relaxed(val, base.add(TIMER_INT)); }

unsafe extern "C" fn sprd_timer_set_next_event(cycles: c_ulong, ce: *mut clock_event_device) -> i32 { let to = to_timer_of(ce); sprd_timer_disable(timer_of_base(to)); sprd_timer_update_counter(timer_of_base(to), cycles); sprd_timer_enable(timer_of_base(to), 0); 0 }
unsafe extern "C" fn sprd_timer_set_periodic(ce: *mut clock_event_device) -> i32 { let to = to_timer_of(ce); sprd_timer_disable(timer_of_base(to)); sprd_timer_update_counter(timer_of_base(to), timer_of_period(to)); sprd_timer_enable(timer_of_base(to), TIMER_CTL_PERIOD_MODE); 0 }
unsafe extern "C" fn sprd_timer_shutdown(ce: *mut clock_event_device) -> i32 { let to = to_timer_of(ce); sprd_timer_disable(timer_of_base(to)); 0 }
unsafe extern "C" fn sprd_timer_interrupt(_irq: i32, dev_id: *mut c_void) -> irqreturn_t { let ce = dev_id as *mut clock_event_device; let to = to_timer_of(ce); sprd_timer_clear_interrupt(timer_of_base(to)); if clockevent_state_oneshot(ce) { sprd_timer_disable(timer_of_base(to)); } if let Some(handler) = (*ce).event_handler { handler(ce); } 1 }

static mut TO: timer_of = timer_of { flags: 0, clkevt: clock_event_device { name: TIMER_NAME.as_ptr(), rating: 300, features: 0, set_state_shutdown: Some(sprd_timer_shutdown), set_state_periodic: Some(sprd_timer_set_periodic), set_next_event: Some(sprd_timer_set_next_event), cpumask: core::ptr::null(), event_handler: None }, of_irq: of_irq { handler: Some(sprd_timer_interrupt), flags: 0 } };

unsafe extern "C" fn sprd_timer_init(np: *mut device_node) -> i32 { let ret = timer_of_init(np, &raw mut TO); if ret != 0 { return ret; } sprd_timer_enable_interrupt(timer_of_base(&raw mut TO)); clockevents_config_and_register(&raw mut TO.clkevt, timer_of_rate(&raw mut TO), 1, u32::MAX); 0 }

static mut SUSPEND_TO: timer_of = timer_of { flags: 0, clkevt: unsafe { core::mem::zeroed() }, of_irq: of_irq { handler: None, flags: 0 } };
unsafe extern "C" fn sprd_suspend_timer_read(_cs: *mut clocksource) -> u64 { let (mut lo, mut hi); loop { hi = readl_relaxed(timer_of_base(&raw mut SUSPEND_TO).add(TIMER_VALUE_SHDW_HI)); lo = readl_relaxed(timer_of_base(&raw mut SUSPEND_TO).add(TIMER_VALUE_SHDW_LO)); if hi == readl_relaxed(timer_of_base(&raw mut SUSPEND_TO).add(TIMER_VALUE_SHDW_HI)) { break; } } !(((hi as u64) << 32) | lo as u64) }
unsafe extern "C" fn sprd_suspend_timer_enable(_cs: *mut clocksource) -> i32 { writel_relaxed(TIMER_VALUE_LO_MASK, timer_of_base(&raw mut SUSPEND_TO).add(TIMER_LOAD_LO)); writel_relaxed(TIMER_VALUE_HI_MASK, timer_of_base(&raw mut SUSPEND_TO).add(TIMER_LOAD_HI)); sprd_timer_enable(timer_of_base(&raw mut SUSPEND_TO), TIMER_CTL_PERIOD_MODE | TIMER_CTL_64BIT_WIDTH); 0 }
unsafe extern "C" fn sprd_suspend_timer_disable(_cs: *mut clocksource) { sprd_timer_disable(timer_of_base(&raw mut SUSPEND_TO)); }
static mut SUSPEND_CLOCKSOURCE: clocksource = clocksource { name: b"sprd_suspend_timer\0".as_ptr(), rating: 200, read: Some(sprd_suspend_timer_read), enable: Some(sprd_suspend_timer_enable), disable: Some(sprd_suspend_timer_disable), mask: u64::MAX, flags: 0 };
unsafe extern "C" fn sprd_suspend_timer_init(np: *mut device_node) -> i32 { let ret = timer_of_init(np, &raw mut SUSPEND_TO); if ret != 0 { return ret; } clocksource_register_hz(&raw mut SUSPEND_CLOCKSOURCE, timer_of_rate(&raw mut SUSPEND_TO)); 0 }

// TIMER_OF_DECLARE(sc9860_timer, "sprd,sc9860-timer", sprd_timer_init);
// TIMER_OF_DECLARE(sc9860_persistent_timer, "sprd,sc9860-suspend-timer", sprd_suspend_timer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
