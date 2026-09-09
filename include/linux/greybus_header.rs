/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Greybus driver and device API
 *
 * Copyright 2014-2015 Google Inc.
 * Copyright 2014-2015 Linaro Ltd.
 */

/* C header dependencies are supplied by the surrounding kernel translation. */

/* Matches up with the Greybus Protocol specification document */
pub const GREYBUS_VERSION_MAJOR: u32 = 0x00;
pub const GREYBUS_VERSION_MINOR: u32 = 0x01;

pub const GREYBUS_ID_MATCH_DEVICE: u32 = GREYBUS_ID_MATCH_VENDOR | GREYBUS_ID_MATCH_PRODUCT;

macro_rules! GREYBUS_DEVICE {
    ($v:expr, $p:expr) => {
        .match_flags = GREYBUS_ID_MATCH_DEVICE,
        .vendor = $v,
        .product = $p,
    };
}

macro_rules! GREYBUS_DEVICE_CLASS {
    ($c:expr) => {
        .match_flags = GREYBUS_ID_MATCH_CLASS,
        .class = $c,
    };
}

/* Maximum number of CPorts */
pub const CPORT_ID_MAX: u16 = 4095; /* UniPro max id is 4095 */
pub const CPORT_ID_BAD: u16 = u16::MAX;

#[repr(C)]
pub struct greybus_driver {
    pub name: *const core::ffi::c_char,

    pub probe: Option<unsafe extern "C" fn(
        bundle: *mut gb_bundle,
        id: *const greybus_bundle_id,
    ) -> core::ffi::c_int>,
    pub disconnect: Option<unsafe extern "C" fn(bundle: *mut gb_bundle)>,

    pub id_table: *const greybus_bundle_id,

    pub driver: device_driver,
}

/* C: container_of_const(d, struct greybus_driver, driver). */
#[inline]
pub unsafe fn to_greybus_driver(d: *const device_driver) -> *const greybus_driver {
    (d as *const u8).sub(core::mem::offset_of!(greybus_driver, driver)) as *const greybus_driver
}

#[inline]
pub unsafe fn greybus_set_drvdata(bundle: *mut gb_bundle, data: *mut core::ffi::c_void) {
    dev_set_drvdata(&mut (*bundle).dev, data);
}

#[inline]
pub unsafe fn greybus_get_drvdata(bundle: *mut gb_bundle) -> *mut core::ffi::c_void {
    dev_get_drvdata(&mut (*bundle).dev)
}

/* Don't call these directly, use the module_greybus_driver! macro instead */
extern "C" {
    pub fn greybus_register_driver(
        driver: *mut greybus_driver,
        module: *mut module,
        mod_name: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    pub fn greybus_deregister_driver(driver: *mut greybus_driver);
}

/* define to get proper THIS_MODULE and KBUILD_MODNAME values */
macro_rules! greybus_register {
    ($driver:expr) => {
        greybus_register_driver($driver, THIS_MODULE, KBUILD_MODNAME)
    };
}
macro_rules! greybus_deregister {
    ($driver:expr) => {
        greybus_deregister_driver($driver)
    };
}

/**
 * module_greybus_driver() - Helper macro for registering a Greybus driver
 * @__greybus_driver: greybus_driver structure
 *
 * Helper macro for Greybus drivers to set up proper module init / exit
 * functions.  Replaces module_init() and module_exit() and keeps people from
 * printing pointless things to the kernel log when their driver is loaded.
 */
macro_rules! module_greybus_driver {
    ($greybus_driver:expr) => {
        module_driver!($greybus_driver, greybus_register, greybus_deregister)
    };
}

extern "C" {
    pub fn greybus_disabled() -> bool;

    pub fn gb_debugfs_init();
    pub fn gb_debugfs_cleanup();
    pub fn gb_debugfs_get() -> *mut dentry;

    pub static greybus_bus_type: bus_type;

    pub static greybus_hd_type: device_type;
    pub static greybus_module_type: device_type;
    pub static greybus_interface_type: device_type;
    pub static greybus_control_type: device_type;
    pub static greybus_bundle_type: device_type;
    pub static greybus_svc_type: device_type;
}

#[inline]
pub unsafe fn cport_id_valid(hd: *const gb_host_device, cport_id: u16) -> bool {
    cport_id != CPORT_ID_BAD && cport_id < (*hd).num_cports
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
