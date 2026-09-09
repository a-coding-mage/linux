/*
 * Platform specific functions
 *
 * This file is subject to the terms and conditions of the GNU General
 * Public License.  See the file "COPYING" in the main directory of
 * this archive for more details.
 *
 * Copyright (C) 2001 - 2005 Tensilica Inc.
 */

// Dependency intent from <linux/types.h> and <asm/bootparam.h> is preserved
// through the externally supplied `bp_tag_t` type below.

use core::ffi::c_char;

/*
 * platform_init is called before the mmu is initialized to give the
 * platform a early hook-up. bp_tag_t is a list of configuration tags
 * passed from the boot-loader.
 */
extern "C" {
    pub fn platform_init(tags: *mut bp_tag_t);
}

/*
 * platform_setup is called from setup_arch with a pointer to the command-line
 * string.
 */
extern "C" {
    pub fn platform_setup(command_line: *mut *mut c_char);
}

/*
 * platform_idle is called from the idle function.
 */
extern "C" {
    pub fn platform_idle();
}

/*
 * platform_calibrate_ccount calibrates cpu clock freq (CONFIG_XTENSA_CALIBRATE_CCOUNT)
 */
extern "C" {
    pub fn platform_calibrate_ccount();
}

/*
 * Flush and reset the mmu, simulate a processor reset, and
 * jump to the reset vector.
 */
extern "C" {
    pub fn cpu_reset() -> !;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
