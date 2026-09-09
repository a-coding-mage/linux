/* SPDX-License-Identifier: GPL-2.0+ */
/* Rust translation of linux/surface_aggregator/device.h. */

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SsamDeviceDomain {
    Virtual = 0x00,
    Serialhub = 0x01,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SsamVirtualTc {
    Hub = 0x00,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct SsamDeviceUid {
    pub domain: u8,
    pub category: u8,
    pub target: u8,
    pub instance: u8,
    pub function: u8,
}

pub const SSAM_SSH_TID_ANY: u16 = 0xffff;
pub const SSAM_SSH_IID_ANY: u16 = 0xffff;
pub const SSAM_SSH_FUN_ANY: u16 = 0xffff;

pub const SSAM_DEVICE_HOT_REMOVED_BIT: u32 = 0;

#[repr(C)]
pub struct SsamDevice {
    pub dev: Device,
    pub ctrl: *mut SsamController,
    pub uid: SsamDeviceUid,
    pub flags: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct SsamDeviceDriver {
    pub driver: DeviceDriver,
    pub match_table: *const SsamDeviceId,
    pub probe: Option<unsafe extern "C" fn(*mut SsamDevice) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut SsamDevice)>,
}

extern "C" {
    pub static ssam_device_type: DeviceType;
    pub fn ssam_device_id_match(table: *const SsamDeviceId, uid: SsamDeviceUid) -> *const SsamDeviceId;
    pub fn ssam_device_get_match(dev: *const SsamDevice) -> *const SsamDeviceId;
    pub fn ssam_device_get_match_data(dev: *const SsamDevice) -> *const ::core::ffi::c_void;
    pub fn ssam_device_alloc(ctrl: *mut SsamController, uid: SsamDeviceUid) -> *mut SsamDevice;
    pub fn ssam_device_add(sdev: *mut SsamDevice) -> i32;
    pub fn ssam_device_remove(sdev: *mut SsamDevice);
    pub fn __ssam_device_driver_register(d: *mut SsamDeviceDriver, o: *mut Module) -> i32;
    pub fn ssam_device_driver_unregister(d: *mut SsamDeviceDriver);
}

#[inline]
pub unsafe fn is_ssam_device(d: *mut Device) -> bool {
    #[cfg(CONFIG_SURFACE_AGGREGATOR_BUS)]
    { (*d).type_ == &ssam_device_type }
    #[cfg(not(CONFIG_SURFACE_AGGREGATOR_BUS))]
    { let _ = d; false }
}

#[inline]
pub unsafe fn to_ssam_device(d: *mut Device) -> *mut SsamDevice {
    (d as *mut u8).sub(offset_of!(SsamDevice, dev)) as *mut SsamDevice
}

#[inline]
pub unsafe fn to_ssam_device_driver(d: *mut DeviceDriver) -> *mut SsamDeviceDriver {
    (d as *mut u8).sub(offset_of!(SsamDeviceDriver, driver)) as *mut SsamDeviceDriver
}

extern "C" {
    pub fn ssam_device_mark_hot_removed(sdev: *mut SsamDevice);
    pub fn ssam_device_is_hot_removed(sdev: *mut SsamDevice) -> bool;
}

#[inline]
pub unsafe fn ssam_device_get(sdev: *mut SsamDevice) -> *mut SsamDevice {
    if !sdev.is_null() { to_ssam_device(get_device(&mut (*sdev).dev)) } else { core::ptr::null_mut() }
}

#[inline]
pub unsafe fn ssam_device_put(sdev: *mut SsamDevice) {
    if !sdev.is_null() { put_device(&mut (*sdev).dev); }
}

#[inline]
pub unsafe fn ssam_device_get_drvdata(sdev: *mut SsamDevice) -> *mut ::core::ffi::c_void {
    dev_get_drvdata(&mut (*sdev).dev)
}

#[inline]
pub unsafe fn ssam_device_set_drvdata(sdev: *mut SsamDevice, data: *mut ::core::ffi::c_void) {
    dev_set_drvdata(&mut (*sdev).dev, data);
}

#[inline]
pub unsafe fn ssam_device_driver_register(drv: *mut SsamDeviceDriver) -> i32 {
    __ssam_device_driver_register(drv, THIS_MODULE)
}

extern "C" {
    pub fn __ssam_register_clients(parent: *mut Device, ctrl: *mut SsamController, node: *mut FwnodeHandle) -> i32;
    pub fn ssam_remove_clients(dev: *mut Device);
    pub fn ssam_notifier_register(ctrl: *mut SsamController, n: *mut SsamEventNotifier) -> i32;
    pub fn __ssam_notifier_unregister(ctrl: *mut SsamController, n: *mut SsamEventNotifier, disable: bool) -> i32;
}

#[inline]
pub unsafe fn ssam_register_clients(dev: *mut Device, ctrl: *mut SsamController) -> i32 {
    __ssam_register_clients(dev, ctrl, dev_fwnode(dev))
}

#[inline]
pub unsafe fn ssam_device_register_clients(sdev: *mut SsamDevice) -> i32 {
    ssam_register_clients(&mut (*sdev).dev, (*sdev).ctrl)
}

#[inline]
pub unsafe fn ssam_device_notifier_register(sdev: *mut SsamDevice, n: *mut SsamEventNotifier) -> i32 {
    if ssam_device_is_hot_removed(sdev) { return -19; }
    ssam_notifier_register((*sdev).ctrl, n)
}

#[inline]
pub unsafe fn ssam_device_notifier_unregister(sdev: *mut SsamDevice, n: *mut SsamEventNotifier) -> i32 {
    __ssam_notifier_unregister((*sdev).ctrl, n, !ssam_device_is_hot_removed(sdev))
}

// The C request-generation macros are represented as Rust macros preserving
// the generated function's client/controller and UID argument forwarding.
#[macro_export]
macro_rules! SSAM_DEFINE_SYNC_REQUEST_CL_N {
    ($name:ident, $raw:ident) => {
        unsafe fn $name(sdev: *mut $crate::SsamDevice) -> i32 {
            $raw((*sdev).ctrl, (*sdev).uid.target, (*sdev).uid.instance)
        }
    };
}

#[macro_export]
macro_rules! SSAM_DEFINE_SYNC_REQUEST_CL_W {
    ($name:ident, $raw:ident, $atype:ty) => {
        unsafe fn $name(sdev: *mut $crate::SsamDevice, arg: *const $atype) -> i32 {
            $raw((*sdev).ctrl, (*sdev).uid.target, (*sdev).uid.instance, arg)
        }
    };
}

#[macro_export]
macro_rules! SSAM_DEFINE_SYNC_REQUEST_CL_R {
    ($name:ident, $raw:ident, $rtype:ty) => {
        unsafe fn $name(sdev: *mut $crate::SsamDevice, ret: *mut $rtype) -> i32 {
            $raw((*sdev).ctrl, (*sdev).uid.target, (*sdev).uid.instance, ret)
        }
    };
}

#[macro_export]
macro_rules! SSAM_DEFINE_SYNC_REQUEST_CL_WR {
    ($name:ident, $raw:ident, $atype:ty, $rtype:ty) => {
        unsafe fn $name(sdev: *mut $crate::SsamDevice, arg: *const $atype, ret: *mut $rtype) -> i32 {
            $raw((*sdev).ctrl, (*sdev).uid.target, (*sdev).uid.instance, arg, ret)
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
