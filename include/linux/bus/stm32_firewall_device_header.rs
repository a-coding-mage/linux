/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2023, STMicroelectronics - All Rights Reserved
 */

// C dependencies supplied by other translation units.

pub const STM32_FIREWALL_MAX_EXTRA_ARGS: usize = 5;

/* Opaque reference to stm32_firewall_controller */
pub struct stm32_firewall_controller;
pub struct device_node;
pub struct device;

/**
 * struct stm32_firewall - Information on a device's firewall. Each device can have more than one
 *                           firewall.
 *
 * @firewall_ctrl:          Pointer referencing a firewall controller of the device. It is
 *                          opaque so a device cannot manipulate the controller's ops or access
 *                          the controller's data
 * @extra_args:             Extra arguments that are implementation dependent
 * @entry:                  Name of the firewall entry
 * @extra_args_size:        Number of extra arguments
 * @firewall_id:            Firewall ID associated the device for this firewall controller
 */
#[repr(C)]
pub struct stm32_firewall {
    pub firewall_ctrl: *mut stm32_firewall_controller,
    pub extra_args: [u32; STM32_FIREWALL_MAX_EXTRA_ARGS],
    pub entry: *const core::ffi::c_char,
    pub extra_args_size: usize,
    pub firewall_id: u32,
}

// CONFIG_STM32_FIREWALL is a build-time kernel configuration condition.
#[cfg(feature = "CONFIG_STM32_FIREWALL")]
extern "C" {
    pub fn stm32_firewall_get_firewall(
        np: *mut device_node,
        firewall: *mut stm32_firewall,
        nb_firewall: u32,
    ) -> i32;
    pub fn stm32_firewall_grant_access(firewall: *mut stm32_firewall) -> i32;
    pub fn stm32_firewall_release_access(firewall: *mut stm32_firewall);
    pub fn stm32_firewall_grant_access_by_id(
        firewall: *mut stm32_firewall,
        subsystem_id: u32,
    ) -> i32;
    pub fn stm32_firewall_release_access_by_id(
        firewall: *mut stm32_firewall,
        subsystem_id: u32,
    );
    pub fn stm32_firewall_get_grant_all_access(
        dev: *mut device,
        firewall: *mut *mut stm32_firewall,
        nb_firewall: *mut i32,
    ) -> i32;
}

#[cfg(not(feature = "CONFIG_STM32_FIREWALL"))]
pub unsafe fn stm32_firewall_get_firewall(
    _np: *mut device_node,
    _firewall: *mut stm32_firewall,
    _nb_firewall: u32,
) -> i32 {
    -19 /* -ENODEV */
}

#[cfg(not(feature = "CONFIG_STM32_FIREWALL"))]
pub unsafe fn stm32_firewall_grant_access(_firewall: *mut stm32_firewall) -> i32 {
    -19 /* -ENODEV */
}

#[cfg(not(feature = "CONFIG_STM32_FIREWALL"))]
pub unsafe fn stm32_firewall_release_access(_firewall: *mut stm32_firewall) {}

#[cfg(not(feature = "CONFIG_STM32_FIREWALL"))]
pub unsafe fn stm32_firewall_grant_access_by_id(
    _firewall: *mut stm32_firewall,
    _subsystem_id: u32,
) -> i32 {
    -19 /* -ENODEV */
}

#[cfg(not(feature = "CONFIG_STM32_FIREWALL"))]
pub unsafe fn stm32_firewall_release_access_by_id(
    _firewall: *mut stm32_firewall,
    _subsystem_id: u32,
) {
}

#[cfg(not(feature = "CONFIG_STM32_FIREWALL"))]
pub unsafe fn stm32_firewall_get_grant_all_access(
    _dev: *mut device,
    _firewall: *mut *mut stm32_firewall,
    _nb_firewall: *mut i32,
) -> i32 {
    -19 /* -ENODEV */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
