// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Qualcomm Secure Execution Environment (SEE) interface (QSEECOM).
 * Responsible for setting up and managing QSEECOM client devices.
 *
 * Copyright (C) 2023 Maximilian Luz <luzmaximilian@gmail.com>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.
use core::ffi::{c_char, c_void};

#[repr(C)]
struct QseecomAppDesc {
    app_name: *const c_char,
    dev_name: *const c_char,
}

// These kernel types and functions are provided by the corresponding headers.
#[repr(C)]
struct Device {
    _private: [u8; 0],
}
#[repr(C)]
struct AuxiliaryDevice {
    _private: [u8; 0],
}
#[repr(C)]
struct PlatformDevice {
    _private: [u8; 0],
}
#[repr(C)]
struct QseecomClient {
    _private: [u8; 0],
}
#[repr(C)]
struct PlatformDriver {
    _private: [u8; 0],
}

extern "C" {
    fn qcom_scm_qseecom_app_get_id(app_name: *const c_char, app_id: *mut u32) -> i32;
    fn auxiliary_device_delete(aux_dev: *mut AuxiliaryDevice);
    fn auxiliary_device_uninit(aux_dev: *mut AuxiliaryDevice);
    fn auxiliary_device_init(aux_dev: *mut AuxiliaryDevice) -> i32;
    fn auxiliary_device_add(aux_dev: *mut AuxiliaryDevice) -> i32;
    fn devm_add_action_or_reset(
        dev: *mut Device,
        action: unsafe extern "C" fn(*mut c_void),
        data: *mut c_void,
    ) -> i32;
    fn platform_driver_register(driver: *mut PlatformDriver) -> i32;
}

// The following low-level member/container operations correspond directly to
// the kernel structures and macros declared by the included Linux headers.
extern "C" {
    fn qseecom_client_aux_dev(client: *mut QseecomClient) -> *mut AuxiliaryDevice;
    fn qseecom_client_aux_dev_dev(client: *mut QseecomClient) -> *mut Device;
    fn qseecom_client_set_app_id(client: *mut QseecomClient, app_id: u32);
    fn qseecom_dev_device(qseecom_dev: *mut PlatformDevice) -> *mut Device;
    fn qseecom_aux_dev_set_name(aux_dev: *mut AuxiliaryDevice, name: *const c_char);
    fn qseecom_aux_dev_set_parent(aux_dev: *mut AuxiliaryDevice, parent: *mut Device);
    fn qseecom_aux_dev_set_release(
        aux_dev: *mut AuxiliaryDevice,
        release: unsafe extern "C" fn(*mut Device),
    );
    fn kzalloc_qseecom_client() -> *mut QseecomClient;
    fn kfree(ptr: *mut c_void);
}

unsafe extern "C" fn qseecom_client_release(dev: *mut Device) {
    // container_of(dev, struct qseecom_client, aux_dev.dev)
    let client = dev as *mut QseecomClient;
    kfree(client as *mut c_void);
}

unsafe extern "C" fn qseecom_client_remove(data: *mut c_void) {
    let client = data as *mut QseecomClient;
    auxiliary_device_delete(qseecom_client_aux_dev(client));
    auxiliary_device_uninit(qseecom_client_aux_dev(client));
}

unsafe fn qseecom_client_register(
    qseecom_dev: *mut PlatformDevice,
    desc: *const QseecomAppDesc,
) -> i32 {
    let mut app_id: u32 = 0;
    let ret = qcom_scm_qseecom_app_get_id((*desc).app_name, &mut app_id);
    if ret != 0 {
        // -ENOENT is represented by the kernel errno constant supplied by the build.
        return if ret == -2 { 0 } else { ret };
    }

    let client = kzalloc_qseecom_client();
    if client.is_null() {
        return -12; // -ENOMEM
    }

    let aux_dev = qseecom_client_aux_dev(client);
    qseecom_aux_dev_set_name(aux_dev, (*desc).dev_name);
    qseecom_aux_dev_set_parent(aux_dev, qseecom_dev_device(qseecom_dev));
    qseecom_aux_dev_set_release(aux_dev, qseecom_client_release);
    qseecom_client_set_app_id(client, app_id);

    let ret = auxiliary_device_init(aux_dev);
    if ret != 0 {
        kfree(client as *mut c_void);
        return ret;
    }

    let ret = auxiliary_device_add(aux_dev);
    if ret != 0 {
        auxiliary_device_uninit(aux_dev);
        return ret;
    }

    let ret = devm_add_action_or_reset(
        qseecom_dev_device(qseecom_dev),
        qseecom_client_remove,
        client as *mut c_void,
    );
    if ret != 0 {
        return ret;
    }

    0
}

/*
 * List of supported applications. One client device will be created per entry,
 * assuming the app has already been loaded (usually by firmware bootloaders)
 * and its ID can be queried successfully.
 */
static QCOM_QSEECOM_APPS: [QseecomAppDesc; 1] = [QseecomAppDesc {
    app_name: b"qcom.tz.uefisecapp\0".as_ptr() as *const c_char,
    dev_name: b"uefisecapp\0".as_ptr() as *const c_char,
}];

unsafe extern "C" fn qcom_qseecom_probe(qseecom_dev: *mut PlatformDevice) -> i32 {
    let mut i = 0usize;
    while i < QCOM_QSEECOM_APPS.len() {
        let ret = qseecom_client_register(qseecom_dev, &QCOM_QSEECOM_APPS[i]);
        if ret != 0 {
            return ret;
        }
        i += 1;
    }
    0
}

static mut QCOM_QSEECOM_DRIVER: PlatformDriver = PlatformDriver { _private: [] };

unsafe extern "C" fn qcom_qseecom_init() -> i32 {
    platform_driver_register(&mut QCOM_QSEECOM_DRIVER)
}

// subsys_initcall(qcom_qseecom_init);
// MODULE_AUTHOR("Maximilian Luz <luzmaximilian@gmail.com>");
// MODULE_DESCRIPTION("Driver for the Qualcomm SEE (QSEECOM) interface");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
