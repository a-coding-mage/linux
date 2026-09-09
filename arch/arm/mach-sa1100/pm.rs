/*
 * SA1100 Power Management Routines
 *
 * Copyright (c) 2001 Cliff Brake <cbrake@accelent.com>
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License.
 *
 * History:
 *
 * 2001-02-06: Cliff Brake         Initial code
 *
 * 2001-02-25: Sukjae Cho <sjcho@east.isi.edu> &
 *             Chester Kuo <chester@linux.org.tw>
 *             Save more value for the resume function! Support
 *             Bitsy/Assabet/Freebird board
 *
 * 2001-08-29: Nicolas Pitre <nico@fluxnic.net>
 *             Cleaned up, pushed platform dependent stuff
 *             in the platform specific files.
 *
 * 2002-05-27: Nicolas Pitre Killed sleep.h and the kmalloced save array.
 *             Storage is local on the stack now.
 */

extern "C" {
    fn sa1100_finish_suspend(arg: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    fn cpu_resume();
    fn __pa_symbol(symbol: unsafe extern "C" fn()) -> ::core::ffi::c_ulong;
    fn cpu_suspend(arg: ::core::ffi::c_ulong, fn_ptr: unsafe extern "C" fn(::core::ffi::c_ulong) -> ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn suspend_set_ops(ops: *const platform_suspend_ops);
    fn suspend_valid_only_mem(state: suspend_state_t) -> bool;
}

/* Register symbols and constants are supplied by the architecture headers. */
extern {
    static mut GPLR: ::core::ffi::c_ulong;
    static mut GPDR: ::core::ffi::c_ulong;
    static mut GAFR: ::core::ffi::c_ulong;
    static mut PPDR: ::core::ffi::c_ulong;
    static mut PPSR: ::core::ffi::c_ulong;
    static mut PPAR: ::core::ffi::c_ulong;
    static mut PSDR: ::core::ffi::c_ulong;
    static mut Ser1SDCR0: ::core::ffi::c_ulong;
    static mut RCSR: ::core::ffi::c_ulong;
    static mut PSPR: ::core::ffi::c_ulong;
    static mut ICLR: ::core::ffi::c_ulong;
    static mut ICCR: ::core::ffi::c_ulong;
    static mut ICMR: ::core::ffi::c_ulong;
    static mut GPSR: ::core::ffi::c_ulong;
    static mut GPCR: ::core::ffi::c_ulong;
    static mut PSSR: ::core::ffi::c_ulong;
    static RCSR_HWR: ::core::ffi::c_ulong;
    static RCSR_SWR: ::core::ffi::c_ulong;
    static RCSR_WDR: ::core::ffi::c_ulong;
    static RCSR_SMR: ::core::ffi::c_ulong;
    static PSSR_PH: ::core::ffi::c_ulong;
}

#[repr(C)]
pub struct platform_suspend_ops {
    pub enter: Option<unsafe extern "C" fn(suspend_state_t) -> ::core::ffi::c_int>,
    pub valid: Option<unsafe extern "C" fn(suspend_state_t) -> bool>,
}

pub type suspend_state_t = ::core::ffi::c_uint;

/*
 * List of global SA11x0 peripheral registers to preserve.
 * More ones like CP and general purpose register values are preserved
 * on the stack and then the stack pointer is stored last in sleep.S.
 */
pub const SLEEP_SAVE_GPDR: usize = 0;
pub const SLEEP_SAVE_GAFR: usize = 1;
pub const SLEEP_SAVE_PPDR: usize = 2;
pub const SLEEP_SAVE_PPSR: usize = 3;
pub const SLEEP_SAVE_PPAR: usize = 4;
pub const SLEEP_SAVE_PSDR: usize = 5;
pub const SLEEP_SAVE_SER1SDCR0: usize = 6;
pub const SLEEP_SAVE_COUNT: usize = 7;

unsafe fn sa11x0_pm_enter(_state: suspend_state_t) -> ::core::ffi::c_int {
    let mut gpio: ::core::ffi::c_ulong;
    let mut sleep_save = [0 as ::core::ffi::c_ulong; SLEEP_SAVE_COUNT];

    gpio = GPLR;

    /* save vital registers */
    sleep_save[SLEEP_SAVE_GPDR] = GPDR;
    sleep_save[SLEEP_SAVE_GAFR] = GAFR;

    sleep_save[SLEEP_SAVE_PPDR] = PPDR;
    sleep_save[SLEEP_SAVE_PPSR] = PPSR;
    sleep_save[SLEEP_SAVE_PPAR] = PPAR;
    sleep_save[SLEEP_SAVE_PSDR] = PSDR;

    sleep_save[SLEEP_SAVE_SER1SDCR0] = Ser1SDCR0;

    /* Clear previous reset status */
    RCSR = RCSR_HWR | RCSR_SWR | RCSR_WDR | RCSR_SMR;

    /* set resume return address */
    PSPR = __pa_symbol(cpu_resume);

    /* go zzz */
    cpu_suspend(0, sa1100_finish_suspend);

    /*
     * Ensure not to come back here if it wasn't intended
     */
    RCSR = RCSR_SMR;
    PSPR = 0;

    /*
     * Ensure interrupt sources are disabled; we will re-init
     * the interrupt subsystem via the device manager.
     */
    ICLR = 0;
    ICCR = 1;
    ICMR = 0;

    /* restore registers */
    GPDR = sleep_save[SLEEP_SAVE_GPDR];
    GAFR = sleep_save[SLEEP_SAVE_GAFR];

    PPDR = sleep_save[SLEEP_SAVE_PPDR];
    PPSR = sleep_save[SLEEP_SAVE_PPSR];
    PPAR = sleep_save[SLEEP_SAVE_PPAR];
    PSDR = sleep_save[SLEEP_SAVE_PSDR];

    Ser1SDCR0 = sleep_save[SLEEP_SAVE_SER1SDCR0];

    GPSR = gpio;
    GPCR = !gpio;

    /*
     * Clear the peripheral sleep-hold bit.
     */
    PSSR = PSSR_PH;

    0
}

static SA11X0_PM_OPS: platform_suspend_ops = platform_suspend_ops {
    enter: Some(sa11x0_pm_enter),
    valid: Some(suspend_valid_only_mem),
};

pub unsafe extern "C" fn sa11x0_pm_init() -> ::core::ffi::c_int {
    suspend_set_ops(&SA11X0_PM_OPS);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
