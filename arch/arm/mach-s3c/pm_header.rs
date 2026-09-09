/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2004 Simtec Electronics
 *	http://armlinux.simtec.co.uk/
 *	Written by Ben Dooks, <ben@simtec.co.uk>
 */

/* s3c_pm_init
 *
 * called from board at initialisation time to setup the power
 * management
 */

/* Declarations supplied by pm-common.h are external dependencies. */

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_SAMSUNG_PM")]
extern "C" {
    pub fn s3c_pm_init() -> ::core::ffi::c_int;
    pub fn s3c64xx_pm_init() -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_SAMSUNG_PM"))]
#[inline]
pub fn s3c_pm_init() -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_SAMSUNG_PM"))]
#[inline]
pub fn s3c64xx_pm_init() -> ::core::ffi::c_int {
    0
}

/* configuration for the IRQ mask over sleep */
extern "C" {
    pub static mut s3c_irqwake_intmask: ::core::ffi::c_ulong;
    pub static mut s3c_irqwake_eintmask: ::core::ffi::c_ulong;
}

/* per-cpu sleep functions */
extern "C" {
    pub static mut pm_cpu_prep: Option<unsafe extern "C" fn()>;
    pub static mut pm_cpu_sleep: Option<unsafe extern "C" fn(::core::ffi::c_ulong) -> ::core::ffi::c_int>;
}

/* Flags for PM Control */
extern "C" {
    pub static mut s3c_pm_flags: ::core::ffi::c_ulong;
}

/* from sleep.S */
#[repr(C)]
pub struct irq_data {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_PM_SLEEP")]
extern "C" {
    pub fn s3c_irq_wake(data: *mut irq_data, state: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn s3c_cpu_resume();
}

#[cfg(not(feature = "CONFIG_PM_SLEEP"))]
pub const s3c_irq_wake: Option<unsafe extern "C" fn(*mut irq_data, ::core::ffi::c_uint) -> ::core::ffi::c_int> = None;

#[cfg(not(feature = "CONFIG_PM_SLEEP"))]
pub const s3c_cpu_resume: Option<unsafe extern "C" fn()> = None;

#[cfg(feature = "CONFIG_SAMSUNG_PM")]
extern "C" {
    pub fn s3c_irqext_wake(data: *mut irq_data, state: ::core::ffi::c_uint) -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_SAMSUNG_PM"))]
pub const s3c_irqext_wake: Option<unsafe extern "C" fn(*mut irq_data, ::core::ffi::c_uint) -> ::core::ffi::c_int> = None;

/**
 * s3c_pm_configure_extint() - ensure pins are correctly set for IRQ
 *
 * Setup all the necessary GPIO pins for waking the system on external
 * interrupt.
 */
extern "C" {
    pub fn s3c_pm_configure_extint();
}

#[cfg(feature = "CONFIG_GPIO_SAMSUNG")]
extern "C" {
    /**
     * samsung_pm_restore_gpios() - restore the state of the gpios after sleep.
     *
     * Restore the state of the GPIO pins after sleep, which may involve ensuring
     * that we do not glitch the state of the pins from that the bootloader's
     * resume code has done.
     */
    pub fn samsung_pm_restore_gpios();

    /**
     * samsung_pm_save_gpios() - save the state of the GPIOs for restoring after sleep.
     *
     * Save the GPIO states for resotration on resume. See samsung_pm_restore_gpios().
     */
    pub fn samsung_pm_save_gpios();
}

#[cfg(not(feature = "CONFIG_GPIO_SAMSUNG"))]
#[inline]
pub fn samsung_pm_restore_gpios() {}

#[cfg(not(feature = "CONFIG_GPIO_SAMSUNG"))]
#[inline]
pub fn samsung_pm_save_gpios() {}

extern "C" {
    pub fn s3c_pm_save_core();
    pub fn s3c_pm_restore_core();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
