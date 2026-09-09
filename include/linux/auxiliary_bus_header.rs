/* SPDX-License-Identifier: GPL-2.0-only */
/* Direct Rust translation of auxiliary_bus.h. */

/* Dependencies supplied by the surrounding kernel translation. */
use core::ffi::c_void;

extern "C" {
    pub fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    pub fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    pub fn mutex_destroy(lock: *mut mutex);
    pub fn put_device(dev: *mut device);
    pub fn device_del(dev: *mut device);
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xarray {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct auxiliary_device_id {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pm_message_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct auxiliary_device {
    pub dev: device,
    pub name: *const i8,
    pub id: u32,
    pub sysfs: auxiliary_device_sysfs,
    pub registration_data_rust: *mut c_void,
}

#[repr(C)]
pub struct auxiliary_device_sysfs {
    pub irqs: xarray,
    /* Synchronize irq sysfs creation. */
    pub lock: mutex,
    pub irq_dir_exists: bool,
}

#[repr(C)]
pub struct auxiliary_driver {
    pub probe: Option<unsafe extern "C" fn(
        auxdev: *mut auxiliary_device,
        id: *const auxiliary_device_id,
    ) -> i32>,
    pub remove: Option<unsafe extern "C" fn(auxdev: *mut auxiliary_device)>,
    pub shutdown: Option<unsafe extern "C" fn(auxdev: *mut auxiliary_device)>,
    pub suspend: Option<unsafe extern "C" fn(
        auxdev: *mut auxiliary_device,
        state: pm_message_t,
    ) -> i32>,
    pub resume: Option<unsafe extern "C" fn(auxdev: *mut auxiliary_device) -> i32>,
    pub name: *const i8,
    pub driver: device_driver,
    pub id_table: *const auxiliary_device_id,
}

#[inline]
pub unsafe fn auxiliary_get_drvdata(auxdev: *mut auxiliary_device) -> *mut c_void {
    dev_get_drvdata(&mut (*auxdev).dev)
}

#[inline]
pub unsafe fn auxiliary_set_drvdata(auxdev: *mut auxiliary_device, data: *mut c_void) {
    dev_set_drvdata(&mut (*auxdev).dev, data);
}

#[inline]
pub unsafe fn to_auxiliary_dev(dev: *mut device) -> *mut auxiliary_device {
    dev as *mut auxiliary_device
}

#[inline]
pub unsafe fn to_auxiliary_drv(drv: *const device_driver) -> *const auxiliary_driver {
    drv as *const auxiliary_driver
}

extern "C" {
    pub fn auxiliary_device_init(auxdev: *mut auxiliary_device) -> i32;
    pub fn __auxiliary_device_add(auxdev: *mut auxiliary_device, modname: *const i8) -> i32;

    /* CONFIG_SYSFS */
    pub fn auxiliary_device_sysfs_irq_add(auxdev: *mut auxiliary_device, irq: i32) -> i32;
    pub fn auxiliary_device_sysfs_irq_remove(auxdev: *mut auxiliary_device, irq: i32);

    pub fn __auxiliary_driver_register(
        auxdrv: *mut auxiliary_driver,
        owner: *mut module,
        modname: *const i8,
    ) -> i32;
    pub fn auxiliary_driver_unregister(auxdrv: *mut auxiliary_driver);
    pub fn auxiliary_device_create(
        dev: *mut device,
        modname: *const i8,
        devname: *const i8,
        platform_data: *mut c_void,
        id: i32,
    ) -> *mut auxiliary_device;
    pub fn auxiliary_device_destroy(auxdev: *mut c_void);
    pub fn __devm_auxiliary_device_create(
        dev: *mut device,
        modname: *const i8,
        devname: *const i8,
        platform_data: *mut c_void,
        id: i32,
    ) -> *mut auxiliary_device;
    pub fn dev_is_auxiliary(dev: *mut device) -> bool;
}

#[cfg(not(feature = "CONFIG_SYSFS"))]
#[inline]
pub unsafe fn auxiliary_device_sysfs_irq_add(_auxdev: *mut auxiliary_device, _irq: i32) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_SYSFS"))]
#[inline]
pub unsafe fn auxiliary_device_sysfs_irq_remove(_auxdev: *mut auxiliary_device, _irq: i32) {}

#[inline]
pub unsafe fn auxiliary_device_uninit(auxdev: *mut auxiliary_device) {
    mutex_destroy(&mut (*auxdev).sysfs.lock);
    put_device(&mut (*auxdev).dev);
}

#[inline]
pub unsafe fn auxiliary_device_delete(auxdev: *mut auxiliary_device) {
    device_del(&mut (*auxdev).dev);
}

/* C macro wrappers retain the build-time module symbols and call ordering. */
#[macro_export]
macro_rules! auxiliary_device_add {
    ($auxdev:expr) => {
        unsafe { $crate::__auxiliary_device_add($auxdev, KBUILD_MODNAME) }
    };
}

#[macro_export]
macro_rules! auxiliary_driver_register {
    ($auxdrv:expr) => {
        unsafe { $crate::__auxiliary_driver_register($auxdrv, THIS_MODULE, KBUILD_MODNAME) }
    };
}

#[macro_export]
macro_rules! devm_auxiliary_device_create {
    ($dev:expr, $devname:expr, $platform_data:expr) => {
        unsafe {
            $crate::__devm_auxiliary_device_create(
                $dev,
                KBUILD_MODNAME,
                $devname,
                $platform_data,
                0,
            )
        }
    };
}

/* module_auxiliary_driver() expands to the kernel's module_driver helper. */
#[macro_export]
macro_rules! module_auxiliary_driver {
    ($auxiliary_driver:expr) => {
        module_driver!($auxiliary_driver, auxiliary_driver_register, auxiliary_driver_unregister);
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
