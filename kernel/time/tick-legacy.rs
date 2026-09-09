// SPDX-License-Identifier: GPL-2.0
/*
 * Timer tick function for architectures that lack generic clockevents,
 * consolidated here from m68k/ia64/parisc/arm.
 */

use core::ffi::{c_int, c_ulong, c_void};

// Declarations supplied by the Linux kernel headers and tick-internal.h.
#[repr(C)]
pub struct PtRegs {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut jiffies_lock: c_void;
    static mut jiffies_seq: c_void;
    static CPU_PROFILING: c_int;

    fn raw_spin_lock(lock: *mut c_void);
    fn raw_spin_unlock(lock: *mut c_void);
    fn write_seqcount_begin(seq: *mut c_void);
    fn write_seqcount_end(seq: *mut c_void);
    fn do_timer(ticks: c_ulong);
    fn update_wall_time();
    fn get_irq_regs() -> *mut PtRegs;
    fn user_mode(regs: *mut PtRegs) -> bool;
    fn update_process_times(user: bool);
    fn profile_tick(profile_type: c_int);
}

/**
 * legacy_timer_tick() - advances the timekeeping infrastructure
 * @ticks: number of ticks, that have elapsed since the last call.
 *
 * This is used by platforms that have not been converted to
 * generic clockevents.
 *
 * If 'ticks' is zero, the CPU is not handling timekeeping, so
 * only perform process accounting and profiling.
 *
 * Must be called with interrupts disabled.
 */
pub unsafe fn legacy_timer_tick(ticks: c_ulong) {
    if ticks != 0 {
        raw_spin_lock(&raw mut jiffies_lock as *mut c_void);
        write_seqcount_begin(&raw mut jiffies_seq as *mut c_void);
        do_timer(ticks);
        write_seqcount_end(&raw mut jiffies_seq as *mut c_void);
        raw_spin_unlock(&raw mut jiffies_lock as *mut c_void);
        update_wall_time();
    }
    update_process_times(user_mode(get_irq_regs()));
    profile_tick(CPU_PROFILING);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
