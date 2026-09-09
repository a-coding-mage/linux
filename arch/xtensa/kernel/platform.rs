/*
 * arch/xtensa/kernel/platform.c
 *
 * Default platform functions.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2005 Tensilica Inc.
 *
 * Chris Zankel <chris@zankel.net>
 */

/* Dependencies supplied by the surrounding kernel build. */
use core::ffi::{c_char, c_void};

/* bp_tag_t is declared by asm/platform.h. */
#[repr(C)]
pub struct bp_tag_t {
    _private: [u8; 0],
}

/* ccount_freq and HZ_PER_MHZ are supplied by asm/timex.h and linux/units.h. */
unsafe extern "C" {
    static mut ccount_freq: u32;
}

/* pr_err is the kernel printk error logging entry point. */
unsafe extern "C" {
    fn pr_err(fmt: *const c_char, ...);
}

/*
 * Default functions that are used if no platform specific function is defined.
 * (Please, refer to arch/xtensa/include/asm/platform.h for more information)
 */

/* __weak __init */
#[no_mangle]
pub unsafe extern "C" fn platform_init(_first: *mut bp_tag_t) {
}

/* __weak __init */
#[no_mangle]
pub unsafe extern "C" fn platform_setup(_cmd: *mut *mut c_char) {
}

/* __weak */
#[no_mangle]
pub unsafe extern "C" fn platform_idle() {
    core::arch::asm!("waiti 0", options(nostack, preserves_flags));
}

/* CONFIG_XTENSA_CALIBRATE_CCOUNT is a build-time configuration condition. */
#[cfg(CONFIG_XTENSA_CALIBRATE_CCOUNT)]
#[no_mangle]
pub unsafe extern "C" fn platform_calibrate_ccount() {
    pr_err(b"ERROR: Cannot calibrate cpu frequency! Assuming 10 MHz.\n\0".as_ptr() as *const c_char);
    ccount_freq = 10 * 1_000_000;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
