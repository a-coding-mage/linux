/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * TI DaVinci platform support for power management.
 *
 * Copyright (C) 2009 Texas Instruments, Inc. https://www.ti.com/
 */

use core::ffi::c_void;

/*
 * Caution: Assembly code in sleep.S makes assumtion on the order
 * of the members of this structure.
 */
#[repr(C)]
pub struct davinci_pm_config {
    pub ddr2_ctlr_base: *mut c_void,
    pub ddrpsc_reg_base: *mut c_void,
    pub ddrpsc_num: i32,
    pub ddrpll_reg_base: *mut c_void,
    pub deepsleep_reg: *mut c_void,
    pub cpupll_reg_base: *mut c_void,
    /*
     * Note on SLEEPCOUNT:
     * The SLEEPCOUNT feature is mainly intended for cases in which
     * the internal oscillator is used. The internal oscillator is
     * fully disabled in deep sleep mode.  When you exist deep sleep
     * mode, the oscillator will be turned on and will generate very
     * small oscillations which will not be detected by the deep sleep
     * counter.  Eventually those oscillations will grow to an amplitude
     * large enough to start incrementing the deep sleep counter.
     * In this case recommendation from hardware engineers is that the
     * SLEEPCOUNT be set to 4096.  This means that 4096 valid clock cycles
     * must be detected before the clock is passed to the rest of the
     * system.
     * In the case that the internal oscillator is not used and the
     * clock is generated externally, the SLEEPCOUNT value can be very
     * small since the clock input is assumed to be stable before SoC
     * is taken out of deepsleep mode.  A value of 128 would be more
     * than adequate.
     */
    pub sleepcount: i32,
}

extern "C" {
    pub static mut davinci_cpu_suspend_sz: u32;
    pub fn davinci_cpu_suspend(config: *mut davinci_pm_config);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
