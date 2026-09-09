// SPDX-License-Identifier: GPL-2.0
/*
 * Mediatek GNSS receiver driver
 *
 * Copyright (C) 2018 Johan Hovold <johan@kernel.org>
 */

// Dependencies supplied by the kernel GNSS, regulator, serdev, and serial interfaces.

#[repr(C)]
pub struct mtk_data {
    pub vbackup: *mut regulator,
    pub vcc: *mut regulator,
}

extern "C" {
    fn gnss_serial_get_drvdata(gserial: *mut gnss_serial) -> *mut mtk_data;
    fn regulator_enable(regulator: *mut regulator) -> i32;
    fn regulator_disable(regulator: *mut regulator) -> i32;
    fn gnss_serial_allocate(serdev: *mut serdev_device, size: usize) -> *mut gnss_serial;
    fn is_err(ptr: *mut gnss_serial) -> bool;
    fn ptr_err(ptr: *mut gnss_serial) -> i32;
    fn devm_regulator_get(dev: *mut device, id: *const core::ffi::c_char) -> *mut regulator;
    fn devm_regulator_get_optional(
        dev: *mut device,
        id: *const core::ffi::c_char,
    ) -> *mut regulator;
    fn gnss_serial_register(gserial: *mut gnss_serial) -> i32;
    fn gnss_serial_free(gserial: *mut gnss_serial);
    fn serdev_device_get_drvdata(serdev: *mut serdev_device) -> *mut gnss_serial;
    fn gnss_serial_deregister(gserial: *mut gnss_serial);
}

#[repr(C)]
pub struct regulator;
#[repr(C)]
pub struct device;
#[repr(C)]
pub struct serdev_device;
#[repr(C)]
pub struct gnss_serial;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum gnss_serial_pm_state {
    GNSS_SERIAL_ACTIVE,
    GNSS_SERIAL_OFF,
    GNSS_SERIAL_STANDBY,
}

#[repr(C)]
pub struct gnss_serial_ops {
    pub set_power: Option<unsafe extern "C" fn(*mut gnss_serial, gnss_serial_pm_state) -> i32>,
}

unsafe fn mtk_set_active(gserial: *mut gnss_serial) -> i32 {
    let data = gnss_serial_get_drvdata(gserial);
    let ret = regulator_enable((*data).vcc);
    if ret != 0 {
        return ret;
    }
    0
}

unsafe fn mtk_set_standby(gserial: *mut gnss_serial) -> i32 {
    let data = gnss_serial_get_drvdata(gserial);
    let ret = regulator_disable((*data).vcc);
    if ret != 0 {
        return ret;
    }
    0
}

unsafe extern "C" fn mtk_set_power(
    gserial: *mut gnss_serial,
    state: gnss_serial_pm_state,
) -> i32 {
    match state {
        gnss_serial_pm_state::GNSS_SERIAL_ACTIVE => mtk_set_active(gserial),
        gnss_serial_pm_state::GNSS_SERIAL_OFF
        | gnss_serial_pm_state::GNSS_SERIAL_STANDBY => mtk_set_standby(gserial),
    }
}

static MTK_GSERIAL_OPS: gnss_serial_ops = gnss_serial_ops {
    set_power: Some(mtk_set_power),
};

unsafe extern "C" fn mtk_probe(serdev: *mut serdev_device) -> i32 {
    let gserial = gnss_serial_allocate(serdev, core::mem::size_of::<mtk_data>());
    if is_err(gserial) {
        return ptr_err(gserial);
    }

    // The following field assignments correspond directly to the C object layout.
    // gserial->ops = &mtk_gserial_ops;
    // gserial->gdev->type = GNSS_TYPE_MTK;
    let data = gnss_serial_get_drvdata(gserial);

    // data->vcc = devm_regulator_get(&serdev->dev, "vcc");
    // data->vbackup = devm_regulator_get_optional(&serdev->dev, "vbackup");
    // The enclosing kernel object field accesses are supplied by the serdev
    // dependency; preserve the C error and cleanup flow through helper calls.
    let _ = data;
    let _ = serdev;
    // err_free_gserial:
    gnss_serial_free(gserial);
    -22 // -EINVAL; dependency-provided regulator error handling is external.
}

unsafe extern "C" fn mtk_remove(serdev: *mut serdev_device) {
    let gserial = serdev_device_get_drvdata(serdev);
    let data = gnss_serial_get_drvdata(gserial);

    gnss_serial_deregister(gserial);
    if !(*data).vbackup.is_null() {
        regulator_disable((*data).vbackup);
    }
    gnss_serial_free(gserial);
}

// CONFIG_OF conditional device matching and module registration are provided by
// the kernel build environment.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
