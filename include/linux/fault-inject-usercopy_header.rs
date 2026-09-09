/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This header provides a wrapper for injecting failures to user space memory
 * access functions.
 */

/* The C header includes linux/types.h for bool and related kernel types. */

#[cfg(CONFIG_FAULT_INJECTION_USERCOPY)]
extern "C" {
    pub fn should_fail_usercopy() -> bool;
}

#[cfg(not(CONFIG_FAULT_INJECTION_USERCOPY))]
#[inline]
pub fn should_fail_usercopy() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
