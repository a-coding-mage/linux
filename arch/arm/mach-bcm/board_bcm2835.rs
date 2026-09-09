// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2010 Broadcom
 */

// Translated from the Linux kernel implementation. The included kernel
// declarations and configuration symbols are supplied by other modules.

use core::ffi::c_char;

#[repr(C)]
pub struct Bcm2835MachineDesc {
    pub dt_compat: *const *const c_char,
    pub smp: *const core::ffi::c_void,
}

// Equivalent to the CONFIG_ARCH_MULTI_V6/CONFIG_ARCH_MULTI_V7 conditional
// compatibility table, terminated by a null pointer as in the C source.
#[cfg(all(CONFIG_ARCH_MULTI_V6, CONFIG_ARCH_MULTI_V7))]
static BCM2835_COMPAT: [*const c_char; 4] = [
    b"brcm,bcm2835\0".as_ptr() as *const c_char,
    b"brcm,bcm2836\0".as_ptr() as *const c_char,
    b"brcm,bcm2837\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

#[cfg(all(CONFIG_ARCH_MULTI_V6, not(CONFIG_ARCH_MULTI_V7)))]
static BCM2835_COMPAT: [*const c_char; 2] = [
    b"brcm,bcm2835\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

#[cfg(all(not(CONFIG_ARCH_MULTI_V6), CONFIG_ARCH_MULTI_V7))]
static BCM2835_COMPAT: [*const c_char; 3] = [
    b"brcm,bcm2836\0".as_ptr() as *const c_char,
    b"brcm,bcm2837\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

#[cfg(all(not(CONFIG_ARCH_MULTI_V6), not(CONFIG_ARCH_MULTI_V7)))]
static BCM2835_COMPAT: [*const c_char; 1] = [core::ptr::null()];

extern "C" {
    static bcm2836_smp_ops: core::ffi::c_void;
}

// DT_MACHINE_START(BCM2835, "BCM2835")
#[used]
pub static BCM2835: Bcm2835MachineDesc = Bcm2835MachineDesc {
    dt_compat: BCM2835_COMPAT.as_ptr(),
    // .smp = smp_ops(bcm2836_smp_ops)
    smp: unsafe { &bcm2836_smp_ops as *const core::ffi::c_void },
};

// MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
