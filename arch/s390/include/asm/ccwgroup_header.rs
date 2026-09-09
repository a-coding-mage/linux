/* SPDX-License-Identifier: GPL-2.0 */

// Forward declarations supplied by the surrounding kernel translation.
pub enum ccw_device {}
pub enum ccw_driver {}

/**
 * struct ccwgroup_device - ccw group device
 * @state: online/offline state
 * @count: number of attached slave devices
 * @dev: embedded device structure
 * @cdev: variable number of slave devices, allocated as needed
 * @ungroup_work: used to ungroup the ccwgroup device
 */
#[repr(C)]
pub struct ccwgroup_device {
    pub state: ccwgroup_device_state,
    /* private: */
    pub onoff: atomic_t,
    pub reg_mutex: mutex,
    /* public: */
    pub count: ::core::ffi::c_uint,
    pub dev: device,
    pub ungroup_work: work_struct,
    pub cdev: *mut *mut ccw_device,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ccwgroup_device_state {
    CCWGROUP_OFFLINE,
    CCWGROUP_ONLINE,
}

/**
 * struct ccwgroup_driver - driver for ccw group devices
 * @setup: function called during device creation to setup the device
 * @remove: function called on remove
 * @set_online: function called when device is set online
 * @set_offline: function called when device is set offline
 * @shutdown: function called when device is shut down
 * @driver: embedded driver structure
 * @ccw_driver: supported ccw_driver (optional)
 */
#[repr(C)]
pub struct ccwgroup_driver {
    pub setup: Option<unsafe extern "C" fn(*mut ccwgroup_device) -> ::core::ffi::c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut ccwgroup_device)>,
    pub set_online: Option<unsafe extern "C" fn(*mut ccwgroup_device) -> ::core::ffi::c_int>,
    pub set_offline: Option<unsafe extern "C" fn(*mut ccwgroup_device) -> ::core::ffi::c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut ccwgroup_device)>,
    pub driver: device_driver,
    pub ccw_driver: *mut ccw_driver,
}

extern "C" {
    pub fn ccwgroup_driver_register(cdriver: *mut ccwgroup_driver) -> ::core::ffi::c_int;
    pub fn ccwgroup_driver_unregister(cdriver: *mut ccwgroup_driver);
    pub fn ccwgroup_create_dev(
        root: *mut device,
        gdrv: *mut ccwgroup_driver,
        num_devices: ::core::ffi::c_int,
        buf: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;

    pub fn ccwgroup_set_online(gdev: *mut ccwgroup_device) -> ::core::ffi::c_int;
    pub fn ccwgroup_set_offline(
        gdev: *mut ccwgroup_device,
        call_gdrv: bool,
    ) -> ::core::ffi::c_int;

    pub fn ccwgroup_probe_ccwdev(cdev: *mut ccw_device) -> ::core::ffi::c_int;
    pub fn ccwgroup_remove_ccwdev(cdev: *mut ccw_device);
}

// Equivalent of container_of((x), struct ccwgroup_device, dev).
#[macro_export]
macro_rules! to_ccwgroupdev {
    ($x:expr) => {
        $x as *mut _ as *mut ccwgroup_device
    };
}

// Equivalent of container_of((x), struct ccwgroup_driver, driver).
#[macro_export]
macro_rules! to_ccwgroupdrv {
    ($x:expr) => {
        $x as *mut _ as *mut ccwgroup_driver
    };
}

// CONFIG_CCWGROUP selects the external implementation; otherwise this is the
// header's static inline fallback.
#[cfg(feature = "CONFIG_CCWGROUP")]
extern "C" {
    pub fn dev_is_ccwgroup(dev: *mut device) -> bool;
}

#[cfg(not(feature = "CONFIG_CCWGROUP"))]
#[inline]
pub unsafe fn dev_is_ccwgroup(_dev: *mut device) -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
