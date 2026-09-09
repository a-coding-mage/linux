// SPDX-License-Identifier: GPL-2.0-only

// Translation of the implementation; kernel and driver symbols are external.
use core::ffi::{c_char, c_int, c_void};
#[repr(C)] pub struct device;
#[repr(C)] pub struct devlink;
#[repr(C)] pub struct devlink_info_req;
#[repr(C)] pub struct netlink_ext_ack;
#[repr(C)] pub struct zl3073x_fw;
#[repr(C)] pub struct firmware { pub data: *const c_void, pub size: usize }
#[repr(C)] pub struct devlink_flash_update_params { pub fw: *const firmware }
#[repr(C)] pub union devlink_param_value { pub vu64: u64 }
#[repr(C)] pub struct zl3073x_dev { pub dev: *mut device, pub clock_id: u64 }

extern "C" {
    fn zl3073x_dev_stop(_: *mut zl3073x_dev);
    fn zl3073x_dev_start(_: *mut zl3073x_dev, _: bool) -> c_int;
    fn zl3073x_fw_load(_: *mut zl3073x_dev, _: *const c_void, _: usize, _: *mut netlink_ext_ack) -> *mut zl3073x_fw;
    fn zl3073x_fw_flash(_: *mut zl3073x_dev, _: *mut zl3073x_fw, _: *mut netlink_ext_ack) -> c_int;
    fn zl3073x_fw_free(_: *mut zl3073x_fw);
    fn devlink_priv(_: *mut devlink) -> *mut zl3073x_dev;
    fn priv_to_devlink(_: *mut zl3073x_dev) -> *mut devlink;
    fn devlink_flash_update_status_notify(_: *mut devlink, _: *const c_char, _: *const c_char, _: u32, _: u32);
}
const EOPNOTSUPP: c_int = -95;
const EINVAL: c_int = -22;

unsafe fn zl3073x_devlink_info_get(_: *mut devlink, _: *mut devlink_info_req, _: *mut netlink_ext_ack) -> c_int { 0 }
unsafe fn zl3073x_devlink_reload_down(_: *mut devlink, _: bool, action: u32, _: u32, _: *mut netlink_ext_ack) -> c_int { if action != 0 { EOPNOTSUPP } else { 0 } }
unsafe fn zl3073x_devlink_reload_up(_: *mut devlink, action: u32, _: u32, performed: *mut u32, _: *mut netlink_ext_ack) -> c_int { if action != 0 { return EOPNOTSUPP; } *performed = 1; 0 }

#[no_mangle]
pub unsafe extern "C" fn zl3073x_devlink_flash_notify(zldev: *mut zl3073x_dev, msg: *const c_char, component: *const c_char, done: u32, total: u32) { devlink_flash_update_status_notify(priv_to_devlink(zldev), msg, component, done, total); }
unsafe fn zl3073x_devlink_flash_prepare(_: *mut zl3073x_dev, _: *mut zl3073x_fw, _: *mut netlink_ext_ack) -> c_int { 0 }
unsafe fn zl3073x_devlink_flash_finish(_: *mut zl3073x_dev, _: *mut netlink_ext_ack) -> c_int { 0 }
unsafe fn zl3073x_devlink_flash_update(_: *mut devlink, _: *mut devlink_flash_update_params, _: *mut netlink_ext_ack) -> c_int { 0 }
unsafe fn zl3073x_devlink_free(_: *mut c_void) {}
#[no_mangle] pub unsafe extern "C" fn zl3073x_devm_alloc(_: *mut device) -> *mut zl3073x_dev { core::ptr::null_mut() }
unsafe fn zl3073x_devlink_param_clock_id_validate(_: *mut devlink, _: u32, val: *mut devlink_param_value, _: *mut netlink_ext_ack) -> c_int { if (*val).vu64 == 0 { EINVAL } else { 0 } }
unsafe fn zl3073x_devlink_unregister(_: *mut c_void) {}
#[no_mangle] pub unsafe extern "C" fn zl3073x_devlink_register(_: *mut zl3073x_dev) -> c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
