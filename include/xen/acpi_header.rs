/*
 * acpi.h
 * acpi file for domain 0 kernel
 *
 * Copyright (c) 2011 Konrad Rzeszutek Wilk <konrad.wilk@oracle.com>
 * Copyright (c) 2011 Yu Ke <ke.yu@intel.com>
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License version 2
 * as published by the Free Software Foundation; either version 2 of the
 * License, or (at your option) any later version.
 */

// Dependency corresponding to <linux/types.h>.

pub type GetGsiFromSbdfT = unsafe extern "C" fn(sbdf: u32) -> i32;

// The following items correspond to CONFIG_XEN_DOM0.  The cfg feature is
// the Rust build-time equivalent of that C preprocessor condition.
#[cfg(feature = "CONFIG_XEN_DOM0")]
extern "C" {
    pub fn xen_acpi_notify_hypervisor_sleep(
        sleep_state: u8,
        pm1a_cnt: u32,
        pm1b_cnd: u32,
    ) -> i32;
    pub fn xen_acpi_notify_hypervisor_extended_sleep(
        sleep_state: u8,
        val_a: u32,
        val_b: u32,
    ) -> i32;
    pub fn xen_initial_domain() -> bool;
    pub fn acpi_enter_sleep_state(state: i32);
    pub fn acpi_os_set_prepare_sleep(func: *const ());
    pub fn acpi_os_set_prepare_extended_sleep(func: *const ());
    pub static mut acpi_suspend_lowlevel: Option<unsafe extern "C" fn() -> i32>;
}

#[cfg(feature = "CONFIG_XEN_DOM0")]
#[inline]
pub unsafe extern "C" fn xen_acpi_suspend_lowlevel() -> i32 {
    /*
     * Xen will save and restore CPU context, so
     * we can skip that and just go straight to
     * the suspend.
     */
    acpi_enter_sleep_state(3); // ACPI_STATE_S3
    0
}

#[cfg(feature = "CONFIG_XEN_DOM0")]
#[inline]
pub unsafe extern "C" fn xen_acpi_sleep_register() {
    if xen_initial_domain() {
        acpi_os_set_prepare_sleep(
            xen_acpi_notify_hypervisor_sleep as *const (),
        );
        acpi_os_set_prepare_extended_sleep(
            xen_acpi_notify_hypervisor_extended_sleep as *const (),
        );

        acpi_suspend_lowlevel = Some(xen_acpi_suspend_lowlevel);
    }
}

#[cfg(feature = "CONFIG_XEN_DOM0")]
extern "C" {
    pub fn xen_pvh_setup_gsi(gsi: i32, trigger: i32, polarity: i32) -> i32;
    pub fn xen_acpi_get_gsi_info(
        dev: *mut pci_dev,
        gsi_out: *mut i32,
        trigger_out: *mut i32,
        polarity_out: *mut i32,
    ) -> i32;
    pub fn xen_acpi_register_get_gsi_func(func: GetGsiFromSbdfT);
    pub fn xen_acpi_get_gsi_from_sbdf(sbdf: u32) -> i32;
}

// External type corresponding to struct pci_dev, supplied by another file.
#[cfg(feature = "CONFIG_XEN_DOM0")]
pub enum pci_dev {}

#[cfg(not(feature = "CONFIG_XEN_DOM0"))]
#[inline]
pub unsafe extern "C" fn xen_acpi_sleep_register() {}

#[cfg(not(feature = "CONFIG_XEN_DOM0"))]
#[inline]
pub unsafe extern "C" fn xen_pvh_setup_gsi(
    _gsi: i32,
    _trigger: i32,
    _polarity: i32,
) -> i32 {
    -1
}

#[cfg(not(feature = "CONFIG_XEN_DOM0"))]
#[inline]
pub unsafe extern "C" fn xen_acpi_get_gsi_info(
    _dev: *mut pci_dev,
    _gsi_out: *mut i32,
    _trigger_out: *mut i32,
    _polarity_out: *mut i32,
) -> i32 {
    -1
}

#[cfg(not(feature = "CONFIG_XEN_DOM0"))]
#[inline]
pub unsafe extern "C" fn xen_acpi_register_get_gsi_func(_func: GetGsiFromSbdfT) {}

#[cfg(not(feature = "CONFIG_XEN_DOM0"))]
#[inline]
pub unsafe extern "C" fn xen_acpi_get_gsi_from_sbdf(_sbdf: u32) -> i32 {
    -1
}

#[cfg(not(feature = "CONFIG_XEN_DOM0"))]
pub enum pci_dev {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
