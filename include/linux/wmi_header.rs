/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * wmi.h - ACPI WMI interface
 *
 * Copyright (c) 2015 Andrew Lutomirski
 */

// C dependencies supplied by other translated units:
// linux/compiler_attributes.h, linux/device.h, linux/acpi.h,
// linux/device-id/wmi.h, and linux/types.h.

/**
 * struct wmi_device - WMI device structure
 * @dev: Device associated with this WMI device
 * @setable: True for devices implementing the Set Control Method
 *
 * This represents WMI devices discovered by the WMI driver core.
 */
#[repr(C)]
pub struct wmi_device {
    pub dev: device,
    pub setable: bool,
}

/**
 * to_wmi_device() - Helper macro to cast a device to a wmi_device
 * @device: device struct
 *
 * Cast a struct device to a struct wmi_device.
 */
#[macro_export]
macro_rules! to_wmi_device {
    ($device:expr) => {
        container_of_const!($device, wmi_device, dev)
    };
}

/**
 * struct wmi_buffer - WMI data buffer
 * @length: Buffer length in bytes
 * @data: Pointer to the buffer content
 *
 * This structure is used to exchange data with the WMI driver core.
 */
#[repr(C)]
pub struct wmi_buffer {
    pub length: usize,
    pub data: *mut core::ffi::c_void,
}

/**
 * struct wmi_string - WMI string representation
 * @length: Size of @chars in bytes
 * @chars: UTF16-LE characters with optional nul termination and padding
 *
 * This structure is used when exchanging string data over the WMI interface.
 */
#[repr(C, packed)]
pub struct wmi_string {
    pub length: __le16,
    pub chars: [__le16; 0],
}

unsafe extern "C" {
    pub fn wmi_string_to_utf8s(str_: *const wmi_string, dst: *mut u8, length: usize) -> isize;

    pub fn wmi_string_from_utf8s(
        str_: *mut wmi_string,
        max_chars: usize,
        src: *const u8,
        src_length: usize,
    ) -> isize;

    pub fn wmidev_invoke_method(
        wdev: *mut wmi_device,
        instance: u8,
        method_id: u32,
        input: *const wmi_buffer,
        out: *mut wmi_buffer,
        min_size: usize,
    ) -> i32;

    pub fn wmidev_invoke_procedure(
        wdev: *mut wmi_device,
        instance: u8,
        method_id: u32,
        input: *const wmi_buffer,
    ) -> i32;

    pub fn wmidev_query_block(
        wdev: *mut wmi_device,
        instance: u8,
        out: *mut wmi_buffer,
        min_size: usize,
    ) -> i32;

    pub fn wmidev_set_block(wdev: *mut wmi_device, instance: u8, input: *const wmi_buffer) -> i32;

    pub fn wmidev_evaluate_method(
        wdev: *mut wmi_device,
        instance: u8,
        method_id: u32,
        input: *const acpi_buffer,
        out: *mut acpi_buffer,
    ) -> acpi_status;

    pub fn wmidev_block_query(wdev: *mut wmi_device, instance: u8) -> *mut acpi_object;

    pub fn wmidev_block_set(
        wdev: *mut wmi_device,
        instance: u8,
        input: *const acpi_buffer,
    ) -> acpi_status;

    pub fn wmidev_instance_count(wdev: *mut wmi_device) -> u8;
}

/**
 * struct wmi_driver - WMI driver structure
 * @driver: Driver model structure
 * @id_table: List of WMI GUIDs supported by this driver
 * @min_event_size: Minimum event payload size supported by this driver
 * @no_singleton: Driver can be instantiated multiple times
 * @probe: Callback for device binding
 * @remove: Callback for device unbinding
 * @shutdown: Callback for device shutdown
 * @notify: Callback for receiving WMI events (deprecated)
 * @notify_new: Callback for receiving WMI events
 *
 * This represents WMI drivers which handle WMI devices. The data inside the buffer
 * passed to the @notify_new callback is guaranteed to be aligned on a 8-byte boundary.
 * The minimum supported size for said buffer can be specified using @min_event_size.
 * WMI drivers that still use the deprecated @notify callback can still set @min_event_size
 * to 0 in order to signal that they support WMI events which provide no event data.
 */
#[repr(C)]
pub struct wmi_driver {
    pub driver: device_driver,
    pub id_table: *const wmi_device_id,
    pub min_event_size: usize,
    pub no_singleton: bool,

    pub probe: Option<unsafe extern "C" fn(*mut wmi_device, *const core::ffi::c_void) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut wmi_device)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut wmi_device)>,
    pub notify: Option<unsafe extern "C" fn(*mut wmi_device, *mut acpi_object)>,
    pub notify_new: Option<unsafe extern "C" fn(*mut wmi_device, *const wmi_buffer)>,
}

/**
 * to_wmi_driver() - Helper macro to cast a driver to a wmi_driver
 * @drv: driver struct
 *
 * Cast a struct device_driver to a struct wmi_driver.
 */
#[macro_export]
macro_rules! to_wmi_driver {
    ($drv:expr) => {
        container_of_const!($drv, wmi_driver, driver)
    };
}

unsafe extern "C" {
    pub fn __wmi_driver_register(driver: *mut wmi_driver, owner: *mut module) -> i32;
    pub fn wmi_driver_unregister(driver: *mut wmi_driver);
}

/** Helper macro for registering a WMI driver. */
#[macro_export]
macro_rules! wmi_driver_register {
    ($driver:expr) => {
        __wmi_driver_register($driver, THIS_MODULE)
    };
}

/**
 * Helper macro for WMI drivers which do not do anything special in module
 * init/exit. Each module may only use this macro once.
 */
#[macro_export]
macro_rules! module_wmi_driver {
    ($wmi_driver:expr) => {
        module_driver!($wmi_driver, wmi_driver_register, wmi_driver_unregister)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
