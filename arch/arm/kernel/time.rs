// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/kernel/time.c
 *
 *  Copyright (C) 1991, 1992, 1995  Linus Torvalds
 *  Modifications for ARM (C) 1994-2001 Russell King
 *
 *  This file contains the ARM-specific time handling details:
 *  reading the RTC at bootup, etc...
 */

// C dependencies supplied by the surrounding kernel translation unit.

#[cfg(any(
    feature = "CONFIG_RTC_DRV_CMOS",
    feature = "CONFIG_RTC_DRV_CMOS_MODULE",
    feature = "CONFIG_NVRAM",
    feature = "CONFIG_NVRAM_MODULE"
))]
#[no_mangle]
pub static mut rtc_lock: Spinlock = Spinlock { _opaque: 0 };

// This needs a better home.
#[repr(C)]
pub struct Spinlock {
    _opaque: usize,
}

// Change this if you have some constant time drift.
pub const USECS_PER_JIFFY: u32 = 1_000_000 / HZ;

extern "C" {
    static HZ: u32;
}

#[repr(C)]
pub struct PtRegs {
    pub ARM_pc: u32,
}

#[repr(C)]
pub struct Stackframe {
    pub pc: u32,
}

extern "C" {
    fn in_lock_functions(pc: u32) -> bool;
    fn arm_get_current_stackframe(regs: *mut PtRegs, frame: *mut Stackframe);
    fn unwind_frame(frame: *mut Stackframe) -> i32;
}

#[cfg(feature = "CONFIG_SMP")]
#[no_mangle]
pub unsafe extern "C" fn profile_pc(regs: *mut PtRegs) -> u32 {
    let mut frame = Stackframe { pc: 0 };

    if !in_lock_functions((*regs).ARM_pc) {
        return (*regs).ARM_pc;
    }

    arm_get_current_stackframe(regs, &mut frame);
    loop {
        let ret = unwind_frame(&mut frame);
        if ret < 0 {
            return 0;
        }
        if !in_lock_functions(frame.pc) {
            break;
        }
    }

    frame.pc
}

#[repr(C)]
pub struct Timespec64 {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

type ClockAccessFn = unsafe extern "C" fn(*mut Timespec64);

unsafe extern "C" fn dummy_clock_access(ts: *mut Timespec64) {
    (*ts).tv_sec = 0;
    (*ts).tv_nsec = 0;
}

static mut __read_persistent_clock: ClockAccessFn = dummy_clock_access;

#[no_mangle]
pub unsafe extern "C" fn read_persistent_clock64(ts: *mut Timespec64) {
    (__read_persistent_clock)(ts);
}

#[no_mangle]
pub unsafe extern "C" fn register_persistent_clock(
    read_persistent: Option<ClockAccessFn>,
) -> i32 {
    // Only allow the clockaccess functions to be registered once.
    if (__read_persistent_clock as usize) == (dummy_clock_access as usize) {
        if let Some(read_persistent) = read_persistent {
            __read_persistent_clock = read_persistent;
        }
        return 0;
    }

    -22 // -EINVAL
}

#[repr(C)]
pub struct MachineDesc {
    pub init_time: Option<unsafe extern "C" fn()>,
}

extern "C" {
    static mut machine_desc: *mut MachineDesc;
    fn of_clk_init(np: *const core::ffi::c_void);
    fn timer_probe();
    fn tick_setup_hrtimer_broadcast();
}

#[no_mangle]
pub unsafe extern "C" fn time_init() {
    if let Some(init_time) = (*machine_desc).init_time {
        init_time();
    } else {
        #[cfg(feature = "CONFIG_COMMON_CLK")]
        {
            of_clk_init(core::ptr::null());
        }
        timer_probe();
        tick_setup_hrtimer_broadcast();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
