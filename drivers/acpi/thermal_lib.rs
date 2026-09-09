// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2023 Linaro Limited
 * Copyright 2023 Intel Corporation
 *
 * Library routines for retrieving trip point temperature values from the
 * platform firmware via ACPI.
 */

use core::ffi::{c_char, c_int, c_ulonglong, c_void};

// Dependencies supplied by the corresponding ACPI and thermal headers.
#[repr(C)]
pub struct acpi_device {
    pub handle: *mut c_void,
}

type acpi_status = c_int;

unsafe extern "C" {
    fn acpi_evaluate_integer(
        handle: *mut c_void,
        pathname: *mut c_char,
        arguments: *mut c_void,
        data: *mut c_ulonglong,
    ) -> acpi_status;
    fn acpi_handle_debug(handle: *mut c_void, format: *const c_char, ...);
    fn deci_kelvin_to_millicelsius(temp: c_int) -> c_int;
}

const TEMP_MIN_DECIK: u64 = 2180;
const TEMP_MAX_DECIK: u64 = 4480;
const THERMAL_TEMP_INVALID: c_int = i32::MIN;
const ENODATA: c_int = 61;
const EINVAL: c_int = 22;

#[inline]
unsafe fn acpi_failure(status: acpi_status) -> bool {
    status != 0
}

unsafe fn acpi_trip_temp(
    adev: *mut acpi_device,
    obj_name: *mut c_char,
    ret_temp: *mut c_int,
) -> c_int {
    let mut temp: c_ulonglong = 0;
    let status = acpi_evaluate_integer(
        (*adev).handle,
        obj_name,
        core::ptr::null_mut(),
        &mut temp,
    );

    if acpi_failure(status) {
        acpi_handle_debug((*adev).handle, c"%s evaluation failed\n".as_ptr(), obj_name);
        return -ENODATA;
    }

    if temp >= TEMP_MIN_DECIK && temp <= TEMP_MAX_DECIK {
        *ret_temp = temp as c_int;
    } else {
        acpi_handle_debug(
            (*adev).handle,
            c"%s result %llu out of range\n".as_ptr(),
            obj_name,
            temp,
        );
        *ret_temp = THERMAL_TEMP_INVALID;
    }

    0
}

pub unsafe fn acpi_active_trip_temp(
    adev: *mut acpi_device,
    id: c_int,
    ret_temp: *mut c_int,
) -> c_int {
    let mut obj_name = [b'_', b'A', b'C', (b'0' as c_int + id) as u8, 0];

    if id < 0 || id > 9 {
        return -EINVAL;
    }

    acpi_trip_temp(adev, obj_name.as_mut_ptr() as *mut c_char, ret_temp)
}

pub unsafe fn acpi_passive_trip_temp(adev: *mut acpi_device, ret_temp: *mut c_int) -> c_int {
    acpi_trip_temp(adev, c"_PSV".as_ptr() as *mut c_char, ret_temp)
}

pub unsafe fn acpi_hot_trip_temp(adev: *mut acpi_device, ret_temp: *mut c_int) -> c_int {
    acpi_trip_temp(adev, c"_HOT".as_ptr() as *mut c_char, ret_temp)
}

pub unsafe fn acpi_critical_trip_temp(adev: *mut acpi_device, ret_temp: *mut c_int) -> c_int {
    acpi_trip_temp(adev, c"_CRT".as_ptr() as *mut c_char, ret_temp)
}

unsafe fn thermal_temp(error: c_int, temp_decik: c_int, ret_temp: *mut c_int) -> c_int {
    if error != 0 {
        return error;
    }

    if temp_decik == THERMAL_TEMP_INVALID {
        *ret_temp = THERMAL_TEMP_INVALID;
    } else {
        *ret_temp = deci_kelvin_to_millicelsius(temp_decik);
    }

    0
}

/**
 * thermal_acpi_active_trip_temp - Retrieve active trip point temperature
 * @adev: Target thermal zone ACPI device object.
 * @id: Active cooling level (0 - 9).
 * @ret_temp: Address to store the retrieved temperature value on success.
 *
 * Evaluate the _ACx object for the thermal zone represented by @adev to obtain
 * the temperature of the active cooling trip point corresponding to the active
 * cooling level given by @id.
 *
 * Return 0 on success or a negative error value on failure.
 */
pub unsafe fn thermal_acpi_active_trip_temp(
    adev: *mut acpi_device,
    id: c_int,
    ret_temp: *mut c_int,
) -> c_int {
    let mut temp_decik: c_int = 0;
    let ret = acpi_active_trip_temp(adev, id, &mut temp_decik);
    thermal_temp(ret, temp_decik, ret_temp)
}

pub unsafe fn thermal_acpi_passive_trip_temp(adev: *mut acpi_device, ret_temp: *mut c_int) -> c_int {
    let mut temp_decik: c_int = 0;
    let ret = acpi_passive_trip_temp(adev, &mut temp_decik);
    thermal_temp(ret, temp_decik, ret_temp)
}

pub unsafe fn thermal_acpi_hot_trip_temp(adev: *mut acpi_device, ret_temp: *mut c_int) -> c_int {
    let mut temp_decik: c_int = 0;
    let ret = acpi_hot_trip_temp(adev, &mut temp_decik);
    thermal_temp(ret, temp_decik, ret_temp)
}

pub unsafe fn thermal_acpi_critical_trip_temp(adev: *mut acpi_device, ret_temp: *mut c_int) -> c_int {
    let mut temp_decik: c_int = 0;
    let ret = acpi_critical_trip_temp(adev, &mut temp_decik);
    thermal_temp(ret, temp_decik, ret_temp)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
