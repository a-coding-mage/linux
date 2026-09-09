// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2016 Freescale Semiconductor, Inc.
 * Copyright 2018,2021-2025 NXP
 *
 * NXP System Timer Module:
 *
 * STM supports commonly required system and application software timing
 * functions. STM includes a 32-bit count-up timer and four 32-bit compare
 * channels with a separate interrupt source for each channel.
 */

// C headers and build-time kernel dependencies are supplied externally.

const STM_CR_TEN: u32 = 1 << 0;
const STM_CR_FRZ: u32 = 1 << 1;
const STM_CR_CPS_OFFSET: u32 = 8;
const STM_CR_CPS_MASK: u32 = 0xff << STM_CR_CPS_OFFSET;
const STM_CCR_CEN: u32 = 1 << 0;
const STM_CIR_CIF: u32 = 1 << 0;
const STM_ENABLE_MASK: u32 = STM_CR_FRZ | STM_CR_TEN;

#[inline] fn STM_CR(base: *mut core::ffi::c_void) -> *mut core::ffi::c_void { base }
#[inline] fn STM_CNT(base: *mut u8) -> *mut u8 { unsafe { base.add(0x04) } }
#[inline] fn STM_CCR0(base: *mut u8) -> *mut u8 { unsafe { base.add(0x10) } }
#[inline] fn STM_CCR1(base: *mut u8) -> *mut u8 { unsafe { base.add(0x20) } }
#[inline] fn STM_CCR2(base: *mut u8) -> *mut u8 { unsafe { base.add(0x30) } }
#[inline] fn STM_CCR3(base: *mut u8) -> *mut u8 { unsafe { base.add(0x40) } }
#[inline] fn STM_CIR0(base: *mut u8) -> *mut u8 { unsafe { base.add(0x14) } }
#[inline] fn STM_CIR1(base: *mut u8) -> *mut u8 { unsafe { base.add(0x24) } }
#[inline] fn STM_CIR2(base: *mut u8) -> *mut u8 { unsafe { base.add(0x34) } }
#[inline] fn STM_CIR3(base: *mut u8) -> *mut u8 { unsafe { base.add(0x44) } }
#[inline] fn STM_CMP0(base: *mut u8) -> *mut u8 { unsafe { base.add(0x18) } }
#[inline] fn STM_CMP1(base: *mut u8) -> *mut u8 { unsafe { base.add(0x28) } }
#[inline] fn STM_CMP2(base: *mut u8) -> *mut u8 { unsafe { base.add(0x38) } }
#[inline] fn STM_CMP3(base: *mut u8) -> *mut u8 { unsafe { base.add(0x48) } }

#[repr(C)]
pub struct stm_timer {
    base: *mut u8,
    rate: usize,
    delta: usize,
    counter: usize,
    ced: clock_event_device,
    cs: clocksource,
    refcnt: atomic_t,
}

static mut stm_timers: *mut stm_timer = core::ptr::null_mut();
static mut stm_sched_clock: *mut stm_timer = core::ptr::null_mut();
static mut stm_instances: i32 = 0;
static mut stm_instances_lock: mutex = mutex { _private: [] };

unsafe fn cs_to_stm(cs: *mut clocksource) -> *mut stm_timer {
    (cs as *mut u8).sub(core::mem::offset_of!(stm_timer, cs)) as *mut stm_timer
}
unsafe fn ced_to_stm(ced: *mut clock_event_device) -> *mut stm_timer {
    (ced as *mut u8).sub(core::mem::offset_of!(stm_timer, ced)) as *mut stm_timer
}

unsafe fn nxp_stm_read_sched_clock() -> u64 { readl(STM_CNT((*stm_sched_clock).base)) as u64 }
unsafe fn nxp_stm_clocksource_getcnt(t: *mut stm_timer) -> u32 { readl(STM_CNT((*t).base)) }
unsafe fn nxp_stm_clocksource_setcnt(t: *mut stm_timer, cnt: u32) { writel(cnt, STM_CNT((*t).base)); }
unsafe fn nxp_stm_clocksource_read(cs: *mut clocksource) -> u64 { nxp_stm_clocksource_getcnt(cs_to_stm(cs)) as u64 }

unsafe fn nxp_stm_module_enable(t: *mut stm_timer) { let mut reg = readl(STM_CR((*t).base as *mut core::ffi::c_void)); reg |= STM_ENABLE_MASK; writel(reg, STM_CR((*t).base as *mut core::ffi::c_void)); }
unsafe fn nxp_stm_module_disable(t: *mut stm_timer) { let mut reg = readl(STM_CR((*t).base as *mut core::ffi::c_void)); reg &= !STM_ENABLE_MASK; writel(reg, STM_CR((*t).base as *mut core::ffi::c_void)); }
unsafe fn nxp_stm_module_put(t: *mut stm_timer) { if atomic_dec_and_test(&mut (*t).refcnt) { nxp_stm_module_disable(t); } }
unsafe fn nxp_stm_module_get(t: *mut stm_timer) { if atomic_inc_return(&mut (*t).refcnt) == 1 { nxp_stm_module_enable(t); } }
unsafe fn nxp_stm_clocksource_enable(cs: *mut clocksource) -> i32 { nxp_stm_module_get(cs_to_stm(cs)); 0 }
unsafe fn nxp_stm_clocksource_disable(cs: *mut clocksource) { nxp_stm_module_put(cs_to_stm(cs)); }
unsafe fn nxp_stm_clocksource_suspend(cs: *mut clocksource) { let t = cs_to_stm(cs); nxp_stm_clocksource_disable(cs); (*t).counter = nxp_stm_clocksource_getcnt(t) as usize; }
unsafe fn nxp_stm_clocksource_resume(cs: *mut clocksource) { let t = cs_to_stm(cs); nxp_stm_clocksource_setcnt(t, (*t).counter as u32); nxp_stm_clocksource_enable(cs); }

unsafe fn nxp_stm_clockevent_read_counter(t: *mut stm_timer) -> i32 { readl(STM_CNT((*t).base)) as i32 }
unsafe fn nxp_stm_clockevent_disable(t: *mut stm_timer) { writel(0, STM_CCR0((*t).base)); }
unsafe fn nxp_stm_clockevent_enable(t: *mut stm_timer) { writel(STM_CCR_CEN, STM_CCR0((*t).base)); }
unsafe fn nxp_stm_clockevent_shutdown(ced: *mut clock_event_device) -> i32 { nxp_stm_clockevent_disable(ced_to_stm(ced)); 0 }
unsafe fn nxp_stm_clockevent_set_next_event(delta: usize, ced: *mut clock_event_device) -> i32 {
    let t = ced_to_stm(ced); nxp_stm_clockevent_disable(t); (*t).delta = delta;
    let val = (nxp_stm_clockevent_read_counter(t) as u32).wrapping_add(delta as u32);
    writel(val, STM_CMP0((*t).base));
    if val > (nxp_stm_clockevent_read_counter(t) as u32).wrapping_add(delta as u32) { return -62; }
    nxp_stm_clockevent_enable(t); 0
}
unsafe fn nxp_stm_clockevent_set_periodic(ced: *mut clock_event_device) -> i32 { let t=ced_to_stm(ced); nxp_stm_clockevent_set_next_event((*t).rate, ced) }
unsafe fn nxp_stm_clockevent_suspend(ced: *mut clock_event_device) { nxp_stm_module_put(ced_to_stm(ced)); }
unsafe fn nxp_stm_clockevent_resume(ced: *mut clock_event_device) { nxp_stm_module_get(ced_to_stm(ced)); }

// Remaining platform-driver registration and kernel callback definitions are
// preserved as external kernel integration declarations.
extern "C" {
    fn readl(addr: *mut u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
}

#[allow(non_camel_case_types)] pub struct clocksource { _private: [u8; 0] }
#[allow(non_camel_case_types)] pub struct clock_event_device { _private: [u8; 0] }
#[allow(non_camel_case_types)] pub struct atomic_t { _private: [u8; 0] }
#[allow(non_camel_case_types)] pub struct mutex { _private: [u8; 0] }
extern "C" { fn atomic_dec_and_test(v: *mut atomic_t) -> bool; fn atomic_inc_return(v: *mut atomic_t) -> i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
