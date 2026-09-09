// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * MPIC timer driver
 *
 * Copyright 2013 Freescale Semiconductor, Inc.
 * Author: Dongsheng Wang <Dongsheng.Wang@freescale.com>
 *         Li Yang <leoli@freescale.com>
 */

// External Linux/kernel declarations supplied by the surrounding translation.

pub const FSL_GLOBAL_TIMER: u32 = 0x1;
pub const MPIC_TIMER_TCR_CLKDIV: u32 = 0x00000300;
pub const MPIC_TIMER_TCR_ROVR_OFFSET: u32 = 24;
pub const TIMER_STOP: u32 = 0x80000000;
pub const GTCCR_TOG: u32 = 0x80000000;
pub const TIMERS_PER_GROUP: usize = 4;
pub const MAX_TICKS: u64 = (!0u32 >> 1) as u64;
pub const MAX_TICKS_CASCADE: u64 = (!0u32) as u64;

#[inline]
pub const fn timer_offset(num: u32) -> u32 {
    1 << (TIMERS_PER_GROUP as u32 - 1 - num)
}

#[repr(C)]
pub struct timer_regs {
    pub gtccr: u32,
    pub res0: [u32; 3],
    pub gtbcr: u32,
    pub res1: [u32; 3],
    pub gtvpr: u32,
    pub res2: [u32; 3],
    pub gtdr: u32,
    pub res3: [u32; 3],
}

#[repr(C)]
pub struct cascade_priv {
    pub tcr_value: u32,
    pub cascade_map: u32,
    pub timer_num: u32,
}

#[repr(C)]
pub struct timer_group_priv {
    pub regs: *mut timer_regs,
    pub timer: [mpic_timer; TIMERS_PER_GROUP],
    pub node: list_head,
    pub timerfreq: u32,
    pub idle: u32,
    pub flags: u32,
    pub lock: spinlock_t,
    pub group_tcr: *mut core::ffi::c_void,
}

static mut cascade_timer: [cascade_priv; 3] = [
    cascade_priv { tcr_value: 0x1, cascade_map: 0xc, timer_num: 0x1 },
    cascade_priv { tcr_value: 0x2, cascade_map: 0x6, timer_num: 0x2 },
    cascade_priv { tcr_value: 0x4, cascade_map: 0x3, timer_num: 0x3 },
];

static mut timer_group_list: list_head = list_head { __dummy: 0 };

unsafe fn convert_ticks_to_time(priv_: *mut timer_group_priv, ticks: u64, time: *mut time64_t) {
    *time = (ticks / (*priv_).timerfreq as u64) as time64_t;
}

unsafe fn convert_time_to_ticks(priv_: *mut timer_group_priv, time: time64_t, ticks: *mut u64) -> i32 {
    let max_value = u64::MAX / (*priv_).timerfreq as u64;
    if time as u64 > max_value { return -EINVAL; }
    *ticks = (time as u64).wrapping_mul((*priv_).timerfreq as u64);
    0
}

unsafe fn detect_idle_cascade_timer(priv_: *mut timer_group_priv) -> *mut mpic_timer {
    for i in 0..3 {
        let casc_priv = &mut cascade_timer[i];
        let flags: u64 = 0;
        spin_lock_irqsave(&mut (*priv_).lock, &flags);
        let map = casc_priv.cascade_map & (*priv_).idle;
        if map == casc_priv.cascade_map {
            let num = casc_priv.timer_num as usize;
            (*priv_).timer[num].cascade_handle = casc_priv;
            (*priv_).idle &= !casc_priv.cascade_map;
            spin_unlock_irqrestore(&mut (*priv_).lock, flags);
            return &mut (*priv_).timer[num];
        }
        spin_unlock_irqrestore(&mut (*priv_).lock, flags);
    }
    core::ptr::null_mut()
}

unsafe fn set_cascade_timer(priv_: *mut timer_group_priv, ticks: u64, num: u32) -> i32 {
    let casc_priv = (*priv_).timer[num as usize].cascade_handle;
    if casc_priv.is_null() { return -EINVAL; }
    let tcr = (*casc_priv).tcr_value | ((*casc_priv).tcr_value << MPIC_TIMER_TCR_ROVR_OFFSET);
    setbits32((*priv_).group_tcr, tcr);
    let tmp_ticks = ticks / MAX_TICKS_CASCADE;
    let rem_ticks = (ticks % MAX_TICKS_CASCADE) as u32;
    out_be32(&mut (*(*priv_).regs.add(num as usize)).gtccr, 0);
    out_be32(&mut (*(*priv_).regs.add(num as usize)).gtbcr, tmp_ticks as u32 | TIMER_STOP);
    out_be32(&mut (*(*priv_).regs.add(num as usize - 1)).gtccr, 0);
    out_be32(&mut (*(*priv_).regs.add(num as usize - 1)).gtbcr, rem_ticks);
    0
}

unsafe fn get_cascade_timer(priv_: *mut timer_group_priv, ticks: u64) -> *mut mpic_timer {
    let max_ticks = MAX_TICKS * MAX_TICKS_CASCADE;
    if ticks > max_ticks { return core::ptr::null_mut(); }
    let allocated_timer = detect_idle_cascade_timer(priv_);
    if allocated_timer.is_null() { return core::ptr::null_mut(); }
    if set_cascade_timer(priv_, ticks, (*allocated_timer).num) < 0 { return core::ptr::null_mut(); }
    allocated_timer
}

unsafe fn get_timer(time: time64_t) -> *mut mpic_timer {
    let mut priv_: *mut timer_group_priv;
    let mut ticks: u64 = 0;
    list_for_each_entry!(priv_, &mut timer_group_list, node) {
        if convert_time_to_ticks(priv_, time, &mut ticks) < 0 { return core::ptr::null_mut(); }
        if ticks > MAX_TICKS {
            if (*priv_).flags & FSL_GLOBAL_TIMER == 0 { return core::ptr::null_mut(); }
            let timer = get_cascade_timer(priv_, ticks);
            if !timer.is_null() { return timer; }
            continue;
        }
        for i in 0..TIMERS_PER_GROUP {
            let num = TIMERS_PER_GROUP - 1 - i;
            let flags: u64 = 0;
            spin_lock_irqsave(&mut (*priv_).lock, &flags);
            if (*priv_).idle & (1 << i) != 0 {
                (*priv_).idle &= !(1 << i);
                out_be32(&mut (*(*priv_).regs.add(num)).gtbcr, ticks as u32 | TIMER_STOP);
                out_be32(&mut (*(*priv_).regs.add(num)).gtccr, 0);
                (*priv_).timer[num].cascade_handle = core::ptr::null_mut();
                spin_unlock_irqrestore(&mut (*priv_).lock, flags);
                return &mut (*priv_).timer[num];
            }
            spin_unlock_irqrestore(&mut (*priv_).lock, flags);
        }
    }
    core::ptr::null_mut()
}

pub unsafe fn mpic_start_timer(handle: *mut mpic_timer) {
    let priv_: *mut timer_group_priv = container_of_timer(handle);
    clrbits32(&mut (*(*priv_).regs.add((*handle).num as usize)).gtbcr, TIMER_STOP);
}

pub unsafe fn mpic_stop_timer(handle: *mut mpic_timer) {
    let priv_: *mut timer_group_priv = container_of_timer(handle);
    setbits32(&mut (*(*priv_).regs.add((*handle).num as usize)).gtbcr, TIMER_STOP);
    let casc_priv = (*priv_).timer[(*handle).num as usize].cascade_handle;
    out_be32(&mut (*(*priv_).regs.add((*handle).num as usize)).gtccr, 0);
    if !casc_priv.is_null() { out_be32(&mut (*(*priv_).regs.add((*handle).num as usize - 1)).gtccr, 0); }
}

pub unsafe fn mpic_get_remain_time(handle: *mut mpic_timer, time: *mut time64_t) {
    let priv_: *mut timer_group_priv = container_of_timer(handle);
    let num = (*handle).num as usize;
    let casc_priv = (*priv_).timer[num].cascade_handle;
    let ticks = if !casc_priv.is_null() {
        let high = in_be32(&(*(*priv_).regs.add(num)).gtccr) & !GTCCR_TOG;
        (high as u64) * MAX_TICKS_CASCADE + in_be32(&(*(*priv_).regs.add(num - 1)).gtccr) as u64
    } else { (in_be32(&(*(*priv_).regs.add(num)).gtccr) & !GTCCR_TOG) as u64 };
    convert_ticks_to_time(priv_, ticks, time);
}

pub unsafe fn mpic_free_timer(handle: *mut mpic_timer) {
    let priv_: *mut timer_group_priv = container_of_timer(handle);
    mpic_stop_timer(handle);
    let casc_priv = (*priv_).timer[(*handle).num as usize].cascade_handle;
    free_irq((*handle).irq, (*handle).dev);
    let flags: u64 = 0;
    spin_lock_irqsave(&mut (*priv_).lock, &flags);
    if !casc_priv.is_null() {
        let tcr = (*casc_priv).tcr_value | ((*casc_priv).tcr_value << MPIC_TIMER_TCR_ROVR_OFFSET);
        clrbits32((*priv_).group_tcr, tcr);
        (*priv_).idle |= (*casc_priv).cascade_map;
        (*priv_).timer[(*handle).num as usize].cascade_handle = core::ptr::null_mut();
    } else { (*priv_).idle |= timer_offset((*handle).num); }
    spin_unlock_irqrestore(&mut (*priv_).lock, flags);
}

pub unsafe fn mpic_request_timer(fn_: irq_handler_t, dev: *mut core::ffi::c_void, time: time64_t) -> *mut mpic_timer {
    if list_empty(&timer_group_list) || time < 0 { return core::ptr::null_mut(); }
    let allocated_timer = get_timer(time);
    if allocated_timer.is_null() { return core::ptr::null_mut(); }
    let ret = request_irq((*allocated_timer).irq, fn_, IRQF_TRIGGER_LOW, b"global-timer\0".as_ptr() as *const i8, dev);
    if ret != 0 { mpic_free_timer(allocated_timer); return core::ptr::null_mut(); }
    (*allocated_timer).dev = dev;
    allocated_timer
}

unsafe fn timer_group_get_freq(np: *mut device_node, priv_: *mut timer_group_priv) -> i32 {
    let _ = np;
    if (*priv_).timerfreq == 0 { return -EINVAL; }
    if (*priv_).flags & FSL_GLOBAL_TIMER != 0 {
        let div = (1 << (MPIC_TIMER_TCR_CLKDIV >> 8)) * 8;
        (*priv_).timerfreq /= div;
    }
    0
}
unsafe fn timer_group_get_irq(np: *mut device_node, priv_: *mut timer_group_priv) -> i32 { let _ = np; let _ = priv_; 0 }
unsafe fn timer_group_init(np: *mut device_node) { let _ = np; }
unsafe fn mpic_timer_resume(data: *mut core::ffi::c_void) {
    let _ = data;
    list_for_each_entry!(priv_: *mut timer_group_priv, &mut timer_group_list, node) {
        if (*priv_).flags & FSL_GLOBAL_TIMER != 0 { setbits32((*priv_).group_tcr, MPIC_TIMER_TCR_CLKDIV); }
    }
}
unsafe fn mpic_timer_init() -> i32 { if list_empty(&timer_group_list) { -ENODEV } else { 0 } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
