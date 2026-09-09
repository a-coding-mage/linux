/* SPDX-License-Identifier: GPL-2.0 */
/*
 * ACPI helpers for GPIO API
 *
 * Copyright (C) 2012,2019 Intel Corporation
 */

/* Declarations corresponding to the C header's kernel dependencies. */
use core::ffi::c_char;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_chip {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum gpiod_flags {
    _Opaque,
}

#[cfg(feature = "CONFIG_ACPI")]
extern "C" {
    pub fn acpi_gpiochip_add(chip: *mut gpio_chip);
    pub fn acpi_gpiochip_remove(chip: *mut gpio_chip);

    pub fn acpi_gpiochip_request_interrupts(chip: *mut gpio_chip);
    pub fn acpi_gpiochip_free_interrupts(chip: *mut gpio_chip);

    pub fn acpi_find_gpio(
        fwnode: *mut fwnode_handle,
        con_id: *const c_char,
        idx: u32,
        dflags: *mut gpiod_flags,
        lookupflags: *mut ::core::ffi::c_ulong,
    ) -> *mut gpio_desc;

    pub fn acpi_gpio_count(fwnode: *const fwnode_handle, con_id: *const c_char) -> i32;
}

#[cfg(not(feature = "CONFIG_ACPI"))]
pub unsafe fn acpi_gpiochip_add(_chip: *mut gpio_chip) {}

#[cfg(not(feature = "CONFIG_ACPI"))]
pub unsafe fn acpi_gpiochip_remove(_chip: *mut gpio_chip) {}

#[cfg(not(feature = "CONFIG_ACPI"))]
pub unsafe fn acpi_gpiochip_request_interrupts(_chip: *mut gpio_chip) {}

#[cfg(not(feature = "CONFIG_ACPI"))]
pub unsafe fn acpi_gpiochip_free_interrupts(_chip: *mut gpio_chip) {}

#[cfg(not(feature = "CONFIG_ACPI"))]
pub unsafe fn acpi_find_gpio(
    _fwnode: *mut fwnode_handle,
    _con_id: *const c_char,
    _idx: u32,
    _dflags: *mut gpiod_flags,
    _lookupflags: *mut ::core::ffi::c_ulong,
) -> *mut gpio_desc {
    // ERR_PTR(-ENOENT), with Linux errno ENOENT = 2.
    (-2isize) as *mut gpio_desc
}

#[cfg(not(feature = "CONFIG_ACPI"))]
pub unsafe fn acpi_gpio_count(
    _fwnode: *const fwnode_handle,
    _con_id: *const c_char,
) -> i32 {
    // -ENODEV, with Linux errno ENODEV = 19.
    -19
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum acpi_gpio_ignore_list {
    ACPI_GPIO_IGNORE_WAKE,
    ACPI_GPIO_IGNORE_INTERRUPT,
}

extern "C" {
    pub fn acpi_gpio_process_deferred_list(list: *mut list_head);

    pub fn acpi_gpio_add_to_deferred_list(list: *mut list_head) -> bool;
    pub fn acpi_gpio_remove_from_deferred_list(list: *mut list_head);

    pub fn acpi_gpio_need_run_edge_events_on_boot() -> i32;

    pub fn acpi_gpio_in_ignore_list(
        list: acpi_gpio_ignore_list,
        controller_in: *const c_char,
        pin_in: u32,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
