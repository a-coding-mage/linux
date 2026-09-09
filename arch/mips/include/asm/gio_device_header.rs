/* SPDX-License-Identifier: GPL-2.0 */
// Dependency supplied by the Linux device headers: linux/device.h

#[repr(C)]
pub struct gio_device_id {
    pub id: __u8,
}

#[repr(C)]
pub struct gio_device {
    pub dev: device,
    pub resource: resource,
    pub irq: ::core::ffi::c_uint,
    pub slotno: ::core::ffi::c_uint,

    pub name: *const ::core::ffi::c_char,
    pub id: gio_device_id,
    // C bit-fields; represented by their containing unsigned-int storage.
    pub id32: ::core::ffi::c_uint,
    pub gio64: ::core::ffi::c_uint,
}

macro_rules! to_gio_device {
    ($d:expr) => {
        container_of!($d, gio_device, dev)
    };
}

#[repr(C)]
pub struct gio_driver {
    pub name: *const ::core::ffi::c_char,
    pub owner: *mut module,
    pub id_table: *const gio_device_id,

    pub probe: Option<unsafe extern "C" fn(*mut gio_device, *const gio_device_id) -> ::core::ffi::c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut gio_device)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut gio_device)>,

    pub driver: device_driver,
}

macro_rules! to_gio_driver {
    ($drv:expr) => {
        container_of!($drv, gio_driver, driver)
    };
}

extern "C" {
    pub fn gio_dev_get(dev: *mut gio_device) -> *mut gio_device;
    pub fn gio_dev_put(dev: *mut gio_device);

    pub fn gio_device_register(dev: *mut gio_device) -> ::core::ffi::c_int;
    pub fn gio_device_unregister(dev: *mut gio_device);

    pub fn gio_register_driver(driver: *mut gio_driver) -> ::core::ffi::c_int;
    pub fn gio_unregister_driver(driver: *mut gio_driver);

    pub fn gio_set_master(dev: *mut gio_device);
}

macro_rules! gio_get_drvdata {
    ($dev:expr) => {
        dev_get_drvdata(&mut (*$dev).dev)
    };
}

macro_rules! gio_set_drvdata {
    ($dev:expr, $data:expr) => {
        dev_set_drvdata(&mut (*$dev).dev, $data)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
