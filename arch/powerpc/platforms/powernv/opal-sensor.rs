// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PowerNV sensor code
 *
 * Copyright (C) 2013 IBM
 */

// Dependencies supplied by the surrounding kernel translation unit.

type U32 = u32;
type U64 = u64;
type Be32 = u32;
type Be64 = u64;

#[repr(C)]
pub struct opal_msg {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

extern "C" {
    fn opal_async_get_token_interruptible() -> i32;
    fn opal_sensor_read(sensor_hndl: U32, token: i32, data: *mut Be32) -> i32;
    fn opal_sensor_read_u64(sensor_hndl: U32, token: i32, data: *mut Be64) -> i32;
    fn opal_async_wait_response(token: i32, msg: *mut opal_msg) -> i32;
    fn opal_get_async_rc(msg: opal_msg) -> i32;
    fn opal_error_code(rc: i32) -> i32;
    fn opal_async_release_token(token: i32);
    fn opal_check_token(token: i32) -> bool;
    fn of_find_node_by_path(path: *const u8) -> *mut device_node;
    fn of_platform_device_create(
        node: *mut device_node,
        name: *const u8,
        parent: *mut core::ffi::c_void,
    ) -> *mut platform_device;
    fn of_node_put(node: *mut device_node);
    fn ptr_err_or_zero(device: *mut platform_device) -> i32;
    fn be32_to_cpu(value: Be32) -> U32;
    fn be64_to_cpu(value: Be64) -> U64;
}

extern "C" {
    static OPAL_ASYNC_COMPLETION: i32;
    static OPAL_SUCCESS: i32;
    static OPAL_WRONG_STATE: i32;
    static OPAL_SENSOR_READ_U64: i32;
    static EIO: i32;
    static ENODEV: i32;
}

/*
 * This will return sensor information to driver based on the requested sensor
 * handle. A handle is an opaque id for the powernv, read by the driver from the
 * device tree..
 */
pub unsafe fn opal_get_sensor_data(sensor_hndl: U32, sensor_data: *mut U32) -> i32 {
    let mut ret: i32;
    let token: i32;
    let mut msg: opal_msg;
    let mut data: Be32 = 0;

    token = opal_async_get_token_interruptible();
    if token < 0 {
        return token;
    }

    ret = opal_sensor_read(sensor_hndl, token, &mut data);
    if ret == OPAL_ASYNC_COMPLETION {
        ret = opal_async_wait_response(token, &mut msg);
        if ret != 0 {
            // pr_err("%s: Failed to wait for the async response, %d\n", __func__, ret);
            opal_async_release_token(token);
            return ret;
        }

        ret = opal_error_code(opal_get_async_rc(msg));
        *sensor_data = be32_to_cpu(data);
    } else if ret == OPAL_SUCCESS {
        ret = 0;
        *sensor_data = be32_to_cpu(data);
    } else if ret == OPAL_WRONG_STATE {
        ret = -EIO;
    } else {
        ret = opal_error_code(ret);
    }

    opal_async_release_token(token);
    ret
}

// EXPORT_SYMBOL_GPL(opal_get_sensor_data);

pub unsafe fn opal_get_sensor_data_u64(sensor_hndl: U32, sensor_data: *mut U64) -> i32 {
    let mut ret: i32;
    let token: i32;
    let mut msg: opal_msg;
    let mut data: Be64 = 0;

    if !opal_check_token(OPAL_SENSOR_READ_U64) {
        let mut sdata: U32 = 0;

        ret = opal_get_sensor_data(sensor_hndl, &mut sdata);
        if ret == 0 {
            *sensor_data = sdata as U64;
        }
        return ret;
    }

    token = opal_async_get_token_interruptible();
    if token < 0 {
        return token;
    }

    ret = opal_sensor_read_u64(sensor_hndl, token, &mut data);
    if ret == OPAL_ASYNC_COMPLETION {
        ret = opal_async_wait_response(token, &mut msg);
        if ret != 0 {
            // pr_err("%s: Failed to wait for the async response, %d\n", __func__, ret);
            opal_async_release_token(token);
            return ret;
        }

        ret = opal_error_code(opal_get_async_rc(msg));
        *sensor_data = be64_to_cpu(data);
    } else if ret == OPAL_SUCCESS {
        ret = 0;
        *sensor_data = be64_to_cpu(data);
    } else if ret == OPAL_WRONG_STATE {
        ret = -EIO;
    } else {
        ret = opal_error_code(ret);
    }

    opal_async_release_token(token);
    ret
}

// EXPORT_SYMBOL_GPL(opal_get_sensor_data_u64);

// __init
pub unsafe fn opal_sensor_init() -> i32 {
    let pdev: *mut platform_device;
    let sensor: *mut device_node;

    sensor = of_find_node_by_path(b"/ibm,opal/sensors\0".as_ptr());
    if sensor.is_null() {
        // pr_err("Opal node 'sensors' not found\n");
        return -ENODEV;
    }

    pdev = of_platform_device_create(sensor, b"opal-sensor\0".as_ptr(), core::ptr::null_mut());
    of_node_put(sensor);

    ptr_err_or_zero(pdev)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
