/* SPDX-License-Identifier: GPL-2.0 */
/* Backlight Lowlevel Control Abstraction */

// C dependencies: linux/device.h, linux/mutex.h, linux/types.h

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum backlight_update_reason {
    BACKLIGHT_UPDATE_HOTKEY,
    BACKLIGHT_UPDATE_SYSFS,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum backlight_type {
    BACKLIGHT_RAW = 1,
    BACKLIGHT_PLATFORM,
    BACKLIGHT_FIRMWARE,
    BACKLIGHT_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum backlight_scale {
    BACKLIGHT_SCALE_UNKNOWN = 0,
    BACKLIGHT_SCALE_LINEAR,
    BACKLIGHT_SCALE_NON_LINEAR,
}

pub const BL_CORE_SUSPENDRESUME: ::core::ffi::c_uint = 1 << 0;
pub const BACKLIGHT_POWER_ON: ::core::ffi::c_int = 0;
pub const BACKLIGHT_POWER_OFF: ::core::ffi::c_int = 4;
pub const BACKLIGHT_POWER_REDUCED: ::core::ffi::c_int = 1; // deprecated; don't use in new code
pub const BL_CORE_SUSPENDED: ::core::ffi::c_uint = 1 << 0;
pub const BL_CORE_FBBLANK: ::core::ffi::c_uint = 1 << 1;

#[repr(C)]
pub struct backlight_ops {
    pub options: ::core::ffi::c_uint,
    pub update_status: Option<unsafe extern "C" fn(*mut backlight_device) -> ::core::ffi::c_int>,
    pub get_brightness: Option<unsafe extern "C" fn(*mut backlight_device) -> ::core::ffi::c_int>,
    pub controls_device: Option<unsafe extern "C" fn(*mut backlight_device, *mut device) -> bool>,
}

#[repr(C)]
pub struct backlight_properties {
    pub brightness: ::core::ffi::c_int,
    pub max_brightness: ::core::ffi::c_int,
    pub power: ::core::ffi::c_int,
    pub r#type: backlight_type,
    pub state: ::core::ffi::c_uint,
    pub scale: backlight_scale,
}

#[repr(C)]
pub struct backlight_device {
    pub props: backlight_properties,
    pub update_lock: mutex,
    pub ops_lock: mutex,
    pub ops: *const backlight_ops,
    pub entry: list_head,
    pub dev: device,
    pub use_count: ::core::ffi::c_int,
}

pub unsafe fn backlight_update_status(bd: *mut backlight_device) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = -2; // -ENOENT
    mutex_lock(&mut (*bd).update_lock);
    if !(*bd).ops.is_null() {
        if let Some(update_status) = (*(*bd).ops).update_status {
            ret = update_status(bd);
        }
    }
    mutex_unlock(&mut (*bd).update_lock);
    ret
}

pub unsafe fn backlight_enable(bd: *mut backlight_device) -> ::core::ffi::c_int {
    if bd.is_null() { return 0; }
    (*bd).props.power = BACKLIGHT_POWER_ON;
    (*bd).props.state &= !BL_CORE_FBBLANK;
    backlight_update_status(bd)
}

pub unsafe fn backlight_disable(bd: *mut backlight_device) -> ::core::ffi::c_int {
    if bd.is_null() { return 0; }
    (*bd).props.power = BACKLIGHT_POWER_OFF;
    (*bd).props.state |= BL_CORE_FBBLANK;
    backlight_update_status(bd)
}

pub unsafe fn backlight_is_blank(bd: *const backlight_device) -> bool {
    (*bd).props.power != BACKLIGHT_POWER_ON || ((*bd).props.state & (BL_CORE_SUSPENDED | BL_CORE_FBBLANK)) != 0
}

pub unsafe fn backlight_get_brightness(bd: *const backlight_device) -> ::core::ffi::c_int {
    if backlight_is_blank(bd) { 0 } else { (*bd).props.brightness }
}

unsafe extern "C" {
    pub fn backlight_device_register(name: *const ::core::ffi::c_char, dev: *mut device, devdata: *mut ::core::ffi::c_void, ops: *const backlight_ops, props: *const backlight_properties) -> *mut backlight_device;
    pub fn devm_backlight_device_register(dev: *mut device, name: *const ::core::ffi::c_char, parent: *mut device, devdata: *mut ::core::ffi::c_void, ops: *const backlight_ops, props: *const backlight_properties) -> *mut backlight_device;
    pub fn backlight_device_unregister(bd: *mut backlight_device);
    pub fn devm_backlight_device_unregister(dev: *mut device, bd: *mut backlight_device);
    pub fn backlight_force_update(bd: *mut backlight_device, reason: backlight_update_reason);
    pub fn backlight_device_get_by_name(name: *const ::core::ffi::c_char) -> *mut backlight_device;
    pub fn backlight_device_get_by_type(r#type: backlight_type) -> *mut backlight_device;
    pub fn backlight_device_set_brightness(bd: *mut backlight_device, brightness: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn backlight_notify_blank(bd: *mut backlight_device, display_dev: *mut device, fb_on: bool, prev_fb_on: bool);
    pub fn backlight_notify_blank_all(display_dev: *mut device, fb_on: bool, prev_fb_on: bool);
    pub fn dev_get_drvdata(dev: *mut device) -> *mut ::core::ffi::c_void;
}

pub unsafe fn bl_get_data(bl_dev: *mut backlight_device) -> *mut ::core::ffi::c_void {
    dev_get_drvdata(&mut (*bl_dev).dev)
}

// CONFIG_OF-dependent declaration; the NULL fallback is retained when disabled.
unsafe extern "C" { pub fn of_find_backlight_by_node(node: *mut device_node) -> *mut backlight_device; }
// IS_ENABLED(CONFIG_BACKLIGHT_CLASS_DEVICE)-dependent declaration; the NULL fallback is retained when disabled.
unsafe extern "C" { pub fn devm_of_find_backlight(dev: *mut device) -> *mut backlight_device; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
