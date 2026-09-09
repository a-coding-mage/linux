/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Common time prototypes and such for all ppc machines.
 *
 * Written by Cort Dougan (cort@cs.nmt.edu) to merge
 * Paul Mackerras' version and mine for PReP and Pmac.
 */

/* The declarations below correspond to the kernel-only portion guarded by
 * __KERNEL__ in the C header. */

extern "C" {
    static mut decrementer_max: u64;

    static mut tb_ticks_per_jiffy: c_ulong;
    static mut tb_ticks_per_usec: c_ulong;
    static mut tb_ticks_per_sec: c_ulong;
    static mut decrementer_clockevent: clock_event_device;

    fn generic_calibrate_decr();

    /* Present when CONFIG_PPC_SPLPAR is enabled. */
    fn get_boot_tb() -> u64;

    /* Some sane defaults: 125 MHz timebase, 1GHz processor. */
    static mut ppc_proc_freq: c_ulong;
    static mut ppc_tb_freq: c_ulong;

    fn secondary_cpu_time_init();
    fn time_init();

    /* DECLARE_PER_CPU(u64, decrementers_next_tb). */
    static mut decrementers_next_tb: u64;

    /* Present when CONFIG_KVM_BOOK3S_HV_POSSIBLE is enabled. */
    fn timer_rearm_host_dec(now: u64);

    fn tb_to_ns(tb_ticks: u64) -> c_ulonglong;
    fn timer_broadcast_interrupt();
    fn pseries_accumulate_stolen_time();
    fn pseries_calculate_stolen_time(stop_tb: u64) -> u64;
}

pub const DEFAULT_TB_FREQ: c_ulong = 125000000 as c_ulong;
pub const DEFAULT_PROC_FREQ: c_ulong = (DEFAULT_TB_FREQ * 8) as c_ulong;

extern "C" {
    static mut tb_invalid: bool;
}

#[repr(C)]
pub struct div_result {
    pub result_high: u64,
    pub result_low: u64,
}

pub unsafe fn get_vtb() -> u64 {
    if cpu_has_feature(CPU_FTR_ARCH_207S) {
        return mfspr(SPRN_VTB);
    }
    0
}

/* Accessor functions for the decrementer register. */
pub unsafe fn get_dec() -> u64 {
    mfspr(SPRN_DEC)
}

/* Book E and 4xx interrupt on 1-to-0; other PowerPC processors on 0-to--1. */
pub unsafe fn set_dec(val: u64) {
    /* CONFIG_BOOKE selects the first branch at build time. */
    if IS_ENABLED(CONFIG_BOOKE) {
        mtspr(SPRN_DEC, val);
    } else {
        mtspr(SPRN_DEC, val.wrapping_sub(1));
    }
}

pub unsafe fn tb_ticks_since(tstamp: c_ulong) -> c_ulong {
    mftb().wrapping_sub(tstamp)
}

/* mulhwu(x, y): high half of unsigned 32-bit multiplication. */
pub unsafe fn mulhwu(x: u32, y: u32) -> u32 {
    let product = (x as u64).wrapping_mul(y as u64);
    (product >> 32) as u32
}

/* mulhdu(x, y): high half of unsigned word-sized multiplication. */
#[cfg(target_pointer_width = "64")]
pub unsafe fn mulhdu(x: c_ulong, y: c_ulong) -> c_ulong {
    let product = (x as u128).wrapping_mul(y as u128);
    (product >> 64) as c_ulong
}

#[cfg(not(target_pointer_width = "64"))]
pub unsafe fn mulhdu(x: u64, y: u64) -> u64 {
    mul_u64_u64_shr(x, y, 64)
}

pub unsafe fn timer_get_next_tb() -> u64 {
    decrementers_next_tb
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
