// SPDX-License-Identifier: GPL-2.0-only
/*
 * Delay loops based on the OpenRISC implementation.
 *
 * Copyright (C) 2012 ARM Limited
 *
 * Author: Will Deacon <will.deacon@arm.com>
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct ArmDelayOps {
    pub delay: unsafe extern "C" fn(unsigned_long: u64),
    pub const_udelay: unsafe extern "C" fn(unsigned_long: u64),
    pub udelay: unsafe extern "C" fn(unsigned_long: u64),
    pub ticks_per_jiffy: u64,
}

#[repr(C)]
pub struct DelayTimer {
    pub read_current_timer: unsafe extern "C" fn() -> u64,
    pub freq: u64,
}

pub type unsigned_long = u64;
pub type cycles_t = u64;

extern "C" {
    pub fn __loop_delay(loops: u64);
    pub fn __loop_const_udelay(loops: u64);
    pub fn __loop_udelay(usecs: u64);
    pub fn get_cycles() -> cycles_t;
    pub fn cpu_relax();
    pub fn clocks_calc_mult_shift(mult: *mut u32, shift: *mut u32, from: u64, to: u64, maxsec: u32);
    pub fn pr_err(fmt: *const u8, ...);
    pub fn pr_info(fmt: *const u8, ...);
    pub static mut lpj_fine: u64;
}

pub const UDELAY_SHIFT: u32 = 0;
pub const UDELAY_MULT: u64 = 0;
pub const NSEC_PER_SEC: u64 = 1_000_000_000;
pub const HZ: u64 = 0;

/*
 * Default to the loop-based delay implementation.
 */
#[no_mangle]
pub static mut arm_delay_ops: ArmDelayOps = ArmDelayOps {
    delay: __loop_delay,
    const_udelay: __loop_const_udelay,
    udelay: __loop_udelay,
    ticks_per_jiffy: 0,
};

static mut delay_timer: *const DelayTimer = core::ptr::null();
static mut delay_calibrated: bool = false;
static mut delay_res: u64 = 0;

#[no_mangle]
pub unsafe extern "C" fn delay_read_timer(timer_val: *mut u64) -> bool {
    if delay_timer.is_null() {
        return false;
    }
    *timer_val = ((*delay_timer).read_current_timer)();
    true
}

#[inline]
unsafe fn cyc_to_ns(cyc: u64, mult: u32, shift: u32) -> u64 {
    cyc.wrapping_mul(mult as u64) >> shift
}

unsafe extern "C" fn __timer_delay(cycles: u64) {
    let start: cycles_t = get_cycles();

    while get_cycles().wrapping_sub(start) < cycles {
        cpu_relax();
    }
}

unsafe extern "C" fn __timer_const_udelay(xloops: u64) {
    let loops = xloops.wrapping_mul(arm_delay_ops.ticks_per_jiffy);
    __timer_delay(loops >> UDELAY_SHIFT);
}

unsafe extern "C" fn __timer_udelay(usecs: u64) {
    __timer_const_udelay(usecs.wrapping_mul(UDELAY_MULT));
}

#[no_mangle]
pub unsafe extern "C" fn register_current_timer_delay(timer: *const DelayTimer) {
    let mut new_mult: u32 = 0;
    let mut new_shift: u32 = 0;
    let res: u64;

    clocks_calc_mult_shift(&mut new_mult, &mut new_shift, (*timer).freq,
                           NSEC_PER_SEC, 3600);
    res = cyc_to_ns(1, new_mult, new_shift);

    if res > 1000 {
        pr_err(b"Ignoring delay timer %ps, which has insufficient resolution of %lluns\n\0".as_ptr(),
               timer, res);
        return;
    }

    if !delay_calibrated && (delay_res == 0 || res < delay_res) {
        pr_info(b"Switching to timer-based delay loop, resolution %lluns\n\0".as_ptr(), res);
        delay_timer = timer;
        lpj_fine = (*timer).freq / HZ;
        delay_res = res;

        /* cpufreq may scale loops_per_jiffy, so keep a private copy */
        arm_delay_ops.ticks_per_jiffy = lpj_fine;
        arm_delay_ops.delay = __timer_delay;
        arm_delay_ops.const_udelay = __timer_const_udelay;
        arm_delay_ops.udelay = __timer_udelay;
    } else {
        pr_info(b"Ignoring duplicate/late registration of read_current_timer delay\n\0".as_ptr());
    }
}

#[no_mangle]
pub unsafe extern "C" fn calibrate_delay_is_known() -> u64 {
    delay_calibrated = true;
    lpj_fine
}

#[no_mangle]
pub unsafe extern "C" fn calibration_delay_done() {
    delay_calibrated = true;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
