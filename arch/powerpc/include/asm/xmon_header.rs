/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 * Copyright (C) 2006 IBM Corp
 */

/* C header guard: __ASM_POWERPC_XMON_H */

/* The following declarations are available only when __KERNEL__ is defined. */

/* Dependency supplied externally: irqreturn_t from <linux/irqreturn.h>. */

/* CONFIG_XMON conditional. */
#[cfg(feature = "CONFIG_XMON")]
unsafe extern "C" {
    pub fn xmon_setup();
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

/* CONFIG_XMON conditional. */
#[cfg(feature = "CONFIG_XMON")]
unsafe extern "C" {
    pub fn xmon(excp: *mut pt_regs) -> ::core::ffi::c_int;
    pub fn xmon_irq(irq: ::core::ffi::c_int, dev_id: *mut ::core::ffi::c_void) -> irqreturn_t;
}

/* CONFIG_XMON inverse conditional. */
#[inline]
#[cfg(not(feature = "CONFIG_XMON"))]
pub fn xmon_setup() {}

/* CONFIG_XMON && CONFIG_SMP conditional. */
#[cfg(all(feature = "CONFIG_XMON", feature = "CONFIG_SMP"))]
unsafe extern "C" {
    pub fn cpus_are_in_xmon() -> ::core::ffi::c_int;
}

/* __printf(1, 2): format string is argument 1 and variadic arguments begin at 2. */
unsafe extern "C" {
    pub fn xmon_printf(format: *const ::core::ffi::c_char, ...);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
