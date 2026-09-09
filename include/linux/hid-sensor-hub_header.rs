/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * HID Sensors Driver
 * Copyright (c) 2012, Intel Corporation.
 */

/* Declarations from linux/hid.h, linux/iio/iio.h, and related headers are
 * intentionally left as external dependencies. */

#[repr(C)]
pub struct hid_sensor_hub_attribute_info {
    pub usage_id: u32,
    pub attrib_id: u32,
    pub report_id: i32,
    pub index: i32,
    pub units: i32,
    pub unit_expo: i32,
    pub size: i32,
    pub logical_minimum: i32,
    pub logical_maximum: i32,
}

#[repr(C)]
pub struct sensor_hub_pending {
    pub status: bool,
    pub ready: completion,
    pub usage_id: u32,
    pub attr_usage_id: u32,
    pub raw_size: i32,
    pub raw_data: *mut u8,
    pub index: u32,
    pub max_raw_size: u32,
}

#[repr(C)]
pub struct hid_sensor_hub_device {
    pub hdev: *mut hid_device,
    pub vendor_id: u32,
    pub product_id: u32,
    pub usage: u32,
    pub start_collection_index: i32,
    pub end_collection_index: i32,
    pub mutex_ptr: *mut mutex,
    pub pending: sensor_hub_pending,
}

#[repr(C)]
pub struct hid_sensor_hub_callbacks {
    pub pdev: *mut platform_device,
    pub suspend: Option<unsafe extern "C" fn(*mut hid_sensor_hub_device, *mut core::ffi::c_void) -> i32>,
    pub resume: Option<unsafe extern "C" fn(*mut hid_sensor_hub_device, *mut core::ffi::c_void) -> i32>,
    pub capture_sample: Option<unsafe extern "C" fn(
        *mut hid_sensor_hub_device,
        u32,
        usize,
        *mut i8,
        *mut core::ffi::c_void,
    ) -> i32>,
    pub send_event: Option<unsafe extern "C" fn(
        *mut hid_sensor_hub_device,
        u32,
        *mut core::ffi::c_void,
    ) -> i32>,
}

unsafe extern "C" {
    pub fn sensor_hub_device_open(hsdev: *mut hid_sensor_hub_device) -> i32;
    pub fn sensor_hub_device_close(hsdev: *mut hid_sensor_hub_device);

    pub fn sensor_hub_register_callback(
        hsdev: *mut hid_sensor_hub_device,
        usage_id: u32,
        usage_callback: *mut hid_sensor_hub_callbacks,
    ) -> i32;
    pub fn sensor_hub_remove_callback(hsdev: *mut hid_sensor_hub_device, usage_id: u32) -> i32;

    pub fn sensor_hub_input_get_attribute_info(
        hsdev: *mut hid_sensor_hub_device,
        type_: u8,
        usage_id: u32,
        attr_usage_id: u32,
        info: *mut hid_sensor_hub_attribute_info,
    ) -> i32;

    pub fn sensor_hub_input_attr_get_raw_value(
        hsdev: *mut hid_sensor_hub_device,
        usage_id: u32,
        attr_usage_id: u32,
        report_id: u32,
        flag: sensor_hub_read_flags,
        is_signed: bool,
    ) -> i32;

    pub fn sensor_hub_input_attr_read_values(
        hsdev: *mut hid_sensor_hub_device,
        usage_id: u32,
        attr_usage_id: u32,
        report_id: u32,
        flag: sensor_hub_read_flags,
        buffer_size: u32,
        buffer: *mut u8,
    ) -> i32;

    pub fn sensor_hub_set_feature(
        hsdev: *mut hid_sensor_hub_device,
        report_id: u32,
        field_index: u32,
        buffer_size: i32,
        buffer: *mut core::ffi::c_void,
    ) -> i32;

    pub fn sensor_hub_get_feature(
        hsdev: *mut hid_sensor_hub_device,
        report_id: u32,
        field_index: u32,
        buffer_size: i32,
        buffer: *mut core::ffi::c_void,
    ) -> i32;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum sensor_hub_read_flags {
    SENSOR_HUB_SYNC,
    SENSOR_HUB_ASYNC,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
