// SPDX-License-Identifier: GPL-2.0
/*
 * u-blox GNSS receiver driver
 *
 * Copyright (C) 2018 Johan Hovold <johan@kernel.org>
 */

// Linux kernel dependencies supplied by the surrounding crate.

use core::ffi::c_void;

#[repr(C)]
pub struct regulator {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct serdev_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gnss_serial {
    pub ops: *const gnss_serial_ops,
    pub gdev: *mut gnss_device,
    _private: [u8; 0],
}

#[repr(C)]
pub struct gnss_device {
    pub type_: i32,
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gnss_serial_ops {
    pub set_power: Option<unsafe extern "C" fn(*mut gnss_serial, gnss_serial_pm_state) -> i32>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum gnss_serial_pm_state {
    GNSS_SERIAL_ACTIVE,
    GNSS_SERIAL_OFF,
    GNSS_SERIAL_STANDBY,
}

extern "C" {
    fn gnss_serial_get_drvdata(gserial: *mut gnss_serial) -> *mut c_void;
    fn regulator_enable(regulator: *mut regulator) -> i32;
    fn regulator_disable(regulator: *mut regulator) -> i32;
    fn gnss_serial_allocate(serdev: *mut serdev_device, size: usize) -> *mut gnss_serial;
    fn devm_regulator_get(dev: *mut device, id: *const u8) -> *mut regulator;
    fn devm_regulator_get_enable_optional(dev: *mut device, id: *const u8) -> i32;
    fn devm_gpiod_get_optional(
        dev: *mut device,
        con_id: *const u8,
        flags: u32,
    ) -> *mut gpio_desc;
    fn gnss_serial_register(gserial: *mut gnss_serial) -> i32;
    fn gnss_serial_free(gserial: *mut gnss_serial);
    fn serdev_device_get_drvdata(serdev: *mut serdev_device) -> *mut gnss_serial;
    fn gnss_serial_deregister(gserial: *mut gnss_serial);
}

const ENODEV: i32 = 19;
const EINVAL: i32 = 22;
const GNSS_TYPE_UBX: i32 = 1;
const GPIOD_OUT_LOW: u32 = 0;

#[repr(C)]
struct ubx_data {
    vcc: *mut regulator,
}

unsafe extern "C" fn ubx_set_active(gserial: *mut gnss_serial) -> i32 {
    let data = gnss_serial_get_drvdata(gserial) as *mut ubx_data;
    let ret = regulator_enable((*data).vcc);
    if ret != 0 {
        return ret;
    }
    0
}

unsafe extern "C" fn ubx_set_standby(gserial: *mut gnss_serial) -> i32 {
    let data = gnss_serial_get_drvdata(gserial) as *mut ubx_data;
    let ret = regulator_disable((*data).vcc);
    if ret != 0 {
        return ret;
    }
    0
}

unsafe extern "C" fn ubx_set_power(
    gserial: *mut gnss_serial,
    state: gnss_serial_pm_state,
) -> i32 {
    match state {
        gnss_serial_pm_state::GNSS_SERIAL_ACTIVE => ubx_set_active(gserial),
        gnss_serial_pm_state::GNSS_SERIAL_OFF
        | gnss_serial_pm_state::GNSS_SERIAL_STANDBY => ubx_set_standby(gserial),
    }
}

static UBX_GSERIAL_OPS: gnss_serial_ops = gnss_serial_ops {
    set_power: Some(ubx_set_power),
};

unsafe extern "C" fn ubx_probe(serdev: *mut serdev_device) -> i32 {
    let gserial = gnss_serial_allocate(serdev, core::mem::size_of::<ubx_data>());
    if gserial.is_null() {
        return -EINVAL;
    }

    (*gserial).ops = &UBX_GSERIAL_OPS;
    (*(*gserial).gdev).type_ = GNSS_TYPE_UBX;

    let data = gnss_serial_get_drvdata(gserial) as *mut ubx_data;
    (*data).vcc = devm_regulator_get(core::ptr::null_mut(), b"vcc\0".as_ptr());
    if (*data).vcc.is_null() {
        gnss_serial_free(gserial);
        return -EINVAL;
    }

    let ret = devm_regulator_get_enable_optional(core::ptr::null_mut(), b"v-bckp\0".as_ptr());
    if ret < 0 && ret != -ENODEV {
        gnss_serial_free(gserial);
        return ret;
    }

    let safeboot = devm_gpiod_get_optional(
        core::ptr::null_mut(),
        b"safeboot\0".as_ptr(),
        GPIOD_OUT_LOW,
    );
    if safeboot.is_null() {
        gnss_serial_free(gserial);
        return -EINVAL;
    }

    let reset = devm_gpiod_get_optional(core::ptr::null_mut(), b"reset\0".as_ptr(), GPIOD_OUT_LOW);
    if reset.is_null() {
        gnss_serial_free(gserial);
        return -EINVAL;
    }

    let ret = gnss_serial_register(gserial);
    if ret != 0 {
        gnss_serial_free(gserial);
        return ret;
    }
    0
}

unsafe extern "C" fn ubx_remove(serdev: *mut serdev_device) {
    let gserial = serdev_device_get_drvdata(serdev);
    gnss_serial_deregister(gserial);
    gnss_serial_free(gserial);
}

#[repr(C)]
struct of_device_id {
    compatible: *const u8,
}

#[cfg(feature = "CONFIG_OF")]
static UBX_OF_MATCH: &[of_device_id] = &[
    of_device_id { compatible: b"u-blox,neo-6m\0".as_ptr() },
    of_device_id { compatible: b"u-blox,neo-8\0".as_ptr() },
    of_device_id { compatible: b"u-blox,neo-m8\0".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

#[repr(C)]
struct serdev_device_driver {
    name: *const u8,
    probe: Option<unsafe extern "C" fn(*mut serdev_device) -> i32>,
    remove: Option<unsafe extern "C" fn(*mut serdev_device)>,
    #[cfg(feature = "CONFIG_OF")]
    of_match_table: *const of_device_id,
}

static mut UBX_DRIVER: serdev_device_driver = serdev_device_driver {
    name: b"gnss-ubx\0".as_ptr(),
    probe: Some(ubx_probe),
    remove: Some(ubx_remove),
    #[cfg(feature = "CONFIG_OF")]
    of_match_table: UBX_OF_MATCH.as_ptr(),
};

// MODULE_DEVICE_TABLE(of, ubx_of_match);
// module_serdev_device_driver(ubx_driver);
// MODULE_AUTHOR("Johan Hovold <johan@kernel.org>");
// MODULE_DESCRIPTION("u-blox GNSS receiver driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
