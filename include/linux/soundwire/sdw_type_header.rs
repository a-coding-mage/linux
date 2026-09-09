/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright(c) 2015-17 Intel Corporation. */

// C header guard: __SOUNDWIRE_TYPES_H

extern "C" {
    pub static sdw_bus_type: bus_type;
    pub static sdw_slave_type: device_type;
    pub static sdw_master_type: device_type;

    pub fn __sdw_register_driver(drv: *mut sdw_driver, owner: *mut module) -> i32;
    pub fn sdw_unregister_driver(drv: *mut sdw_driver);

    pub fn sdw_slave_uevent(
        dev: *const device,
        env: *mut kobj_uevent_env,
    ) -> i32;
}

#[inline]
pub unsafe fn is_sdw_slave(dev: *const device) -> bool {
    (*dev).type_ == core::ptr::addr_of!(sdw_slave_type)
}

// #define drv_to_sdw_driver(_drv) container_of_const(_drv, struct sdw_driver, driver)
#[macro_export]
macro_rules! drv_to_sdw_driver {
    ($drv:expr) => {
        container_of_const!($drv, sdw_driver, driver)
    };
}

// #define sdw_register_driver(drv) __sdw_register_driver(drv, THIS_MODULE)
#[macro_export]
macro_rules! sdw_register_driver {
    ($drv:expr) => {
        __sdw_register_driver($drv, THIS_MODULE)
    };
}

/**
 * module_sdw_driver() - Helper macro for registering a Soundwire driver
 * @__sdw_driver: soundwire slave driver struct
 *
 * Helper macro for Soundwire drivers which do not do anything special in
 * module init/exit. This eliminates a lot of boilerplate. Each module may only
 * use this macro once, and calling it replaces module_init() and module_exit()
 */
// #define module_sdw_driver(__sdw_driver) \
//     module_driver(__sdw_driver, sdw_register_driver, sdw_unregister_driver)
#[macro_export]
macro_rules! module_sdw_driver {
    ($sdw_driver:expr) => {
        module_driver!($sdw_driver, sdw_register_driver, sdw_unregister_driver)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
