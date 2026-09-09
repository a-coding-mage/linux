// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * hwmon interface for the ACPI Fan driver.
 *
 * Copyright (C) 2024 Armin Wolf <W_Armin@gmx.de>
 */

// Kernel and fan-driver declarations supplied by the surrounding translation unit.

use core::ffi::c_void;

extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn acpi_fan_power_valid(power: u32) -> bool;
    fn acpi_fan_speed_valid(speed: u64) -> bool;
    fn acpi_fan_get_fst(handle: *mut c_void, fst: *mut acpi_fan_fst) -> i32;
    fn hwmon_notify_event(hdev: *mut device, sensor_type: u32, attr: u32, channel: i32);
    fn devm_hwmon_device_register_with_info(
        dev: *mut device,
        name: *const u8,
        drvdata: *mut acpi_fan,
        chip: *const hwmon_chip_info,
        config: *const c_void,
    ) -> *mut device;
    fn ptr_err_or_zero(ptr: *mut device) -> i32;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acpi_fan_fps {
    pub control: u64,
    pub speed: u64,
    pub power: u32,
}

#[repr(C)]
pub struct acpi_fan_fst {
    pub control: u64,
    pub speed: u64,
}

#[repr(C)]
pub struct acpi_fan_fif {
    pub fine_grain_ctrl: bool,
}

#[repr(C)]
pub struct acpi_fan {
    pub fps_count: u32,
    pub fps: *mut acpi_fan_fps,
    pub acpi4: bool,
    pub fif: acpi_fan_fif,
    pub handle: *mut c_void,
    pub hdev: *mut device,
}

#[repr(C)]
pub struct hwmon_ops {
    pub is_visible: Option<unsafe extern "C" fn(*const c_void, u32, u32, i32) -> u32>,
    pub read: Option<unsafe extern "C" fn(*mut device, u32, u32, i32, *mut i64) -> i32>,
}

#[repr(C)]
pub struct hwmon_channel_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hwmon_chip_info {
    pub ops: *const hwmon_ops,
    pub info: *const *const hwmon_channel_info,
}

const HWMON_FAN: u32 = 0;
const HWMON_POWER: u32 = 1;
const HWMON_FAN_INPUT: u32 = 0;
const HWMON_FAN_TARGET: u32 = 1;
const HWMON_POWER_INPUT: u32 = 0;
const MICROWATT_PER_MILLIWATT: u64 = 1000;

unsafe fn acpi_fan_get_current_fps(fan: *mut acpi_fan, control: u64) -> *mut acpi_fan_fps {
    let mut i: u32 = 0;

    while i < (*fan).fps_count {
        let fps = (*fan).fps.add(i as usize);
        if (*fps).control == control {
            return fps;
        }
        i += 1;
    }

    core::ptr::null_mut()
}

unsafe extern "C" fn acpi_fan_hwmon_is_visible(
    drvdata: *const c_void,
    sensor_type: u32,
    attr: u32,
    _channel: i32,
) -> u32 {
    let fan = drvdata as *const acpi_fan;
    let mut i: u32;

    match sensor_type {
        HWMON_FAN => match attr {
            HWMON_FAN_INPUT => 0o444,
            HWMON_FAN_TARGET => {
                // Only acpi4 fans support fan control.
                if !(*fan).acpi4 { return 0; }
                // When in fine grain control mode, not every fan control value
                // has an associated fan performance state.
                if (*fan).fif.fine_grain_ctrl { return 0; }
                0o444
            }
            _ => 0,
        },
        HWMON_POWER => match attr {
            HWMON_POWER_INPUT => {
                // Only acpi4 fans support fan control.
                if !(*fan).acpi4 { return 0; }
                // When in fine grain control mode, not every fan control value
                // has an associated fan performance state.
                if (*fan).fif.fine_grain_ctrl { return 0; }
                // When all fan performance states contain no valid power data,
                // the associated attribute should not be created.
                i = 0;
                while i < (*fan).fps_count {
                    if acpi_fan_power_valid((*(*fan).fps.add(i as usize)).power) { return 0o444; }
                    i += 1;
                }
                0
            }
            _ => 0,
        },
        _ => 0,
    }
}

unsafe extern "C" fn acpi_fan_hwmon_read(
    dev: *mut device, sensor_type: u32, attr: u32, _channel: i32, val: *mut i64,
) -> i32 {
    let fan = dev_get_drvdata(dev) as *mut acpi_fan;
    let mut fst = acpi_fan_fst { control: 0, speed: 0 };
    let ret = acpi_fan_get_fst((*fan).handle, &mut fst);
    if ret < 0 { return ret; }

    match sensor_type {
        HWMON_FAN => match attr {
            HWMON_FAN_INPUT => {
                if !acpi_fan_speed_valid(fst.speed) { return -19; }
                if fst.speed > i64::MAX as u64 { return -75; }
                *val = fst.speed as i64; 0
            }
            HWMON_FAN_TARGET => {
                let fps = acpi_fan_get_current_fps(fan, fst.control);
                if fps.is_null() { return -5; }
                if (*fps).speed > i64::MAX as u64 { return -75; }
                *val = (*fps).speed as i64; 0
            }
            _ => -95,
        },
        HWMON_POWER => match attr {
            HWMON_POWER_INPUT => {
                let fps = acpi_fan_get_current_fps(fan, fst.control);
                if fps.is_null() { return -5; }
                if !acpi_fan_power_valid((*fps).power) { return -19; }
                if (*fps).power as u64 > i64::MAX as u64 / MICROWATT_PER_MILLIWATT { return -75; }
                *val = ((*fps).power as u64 * MICROWATT_PER_MILLIWATT) as i64; 0
            }
            _ => -95,
        },
        _ => -95,
    }
}

static ACPI_FAN_HWMON_OPS: hwmon_ops = hwmon_ops {
    is_visible: Some(acpi_fan_hwmon_is_visible),
    read: Some(acpi_fan_hwmon_read),
};

static ACPI_FAN_HWMON_CHIP_INFO: hwmon_chip_info = hwmon_chip_info {
    ops: &ACPI_FAN_HWMON_OPS,
    info: core::ptr::null(),
};

pub unsafe extern "C" fn acpi_fan_notify_hwmon(dev: *mut device) {
    let fan = dev_get_drvdata(dev) as *mut acpi_fan;
    hwmon_notify_event((*fan).hdev, HWMON_FAN, HWMON_FAN_INPUT, 0);
}

pub unsafe extern "C" fn devm_acpi_fan_create_hwmon(dev: *mut device) -> i32 {
    let fan = dev_get_drvdata(dev) as *mut acpi_fan;
    (*fan).hdev = devm_hwmon_device_register_with_info(
        dev, b"acpi_fan\0".as_ptr(), fan, &ACPI_FAN_HWMON_CHIP_INFO, core::ptr::null(),
    );
    ptr_err_or_zero((*fan).hdev)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
