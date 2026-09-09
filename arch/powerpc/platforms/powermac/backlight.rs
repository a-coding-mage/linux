// SPDX-License-Identifier: GPL-2.0-only
/*
 * Miscellaneous procedures for dealing with the PowerMac hardware.
 * Contains support for the backlight.
 *
 *   Copyright (C) 2000 Benjamin Herrenschmidt
 *   Copyright (C) 2006 Michael Hanselmann <linux-kernel@hansmi.ch>
 */

// Dependencies supplied by the surrounding kernel translation.

const OLD_BACKLIGHT_MAX: i32 = 15;

extern "C" {
    fn of_find_node_by_name(from: *mut device_node, name: *const i8) -> *mut device_node;
    fn of_property_match_string(
        np: *mut device_node,
        propname: *const i8,
        string: *const i8,
    ) -> i32;
    fn of_node_put(node: *mut device_node);
    fn atomic_read(v: *const atomic_t) -> i32;
    fn atomic_inc(v: *mut atomic_t);
    fn atomic_dec(v: *mut atomic_t);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn schedule_work(work: *mut work_struct) -> bool;
    fn backlight_update_status(bd: *mut backlight_device) -> i32;
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct atomic_t {
    pub counter: i32,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct backlight_properties {
    pub brightness: i32,
    pub max_brightness: i32,
}

#[repr(C)]
pub struct backlight_device {
    pub props: backlight_properties,
}

static mut PMAC_BACKLIGHT_KEY_QUEUED: i32 = 0;
static mut PMAC_BACKLIGHT_SET_LEGACY_QUEUED: i32 = 0;
static mut KERNEL_BACKLIGHT_DISABLED: atomic_t = atomic_t { counter: 0 };

#[no_mangle]
pub static mut pmac_backlight_mutex: mutex = mutex { _private: [] };

#[no_mangle]
pub static mut pmac_backlight: *mut backlight_device = core::ptr::null_mut();

static mut pmac_backlight_key_work: work_struct = work_struct { _private: [] };
static mut pmac_backlight_set_legacy_work: work_struct = work_struct { _private: [] };

#[no_mangle]
pub unsafe extern "C" fn pmac_has_backlight_type(type_: *const i8) -> bool {
    let bk_node = of_find_node_by_name(core::ptr::null_mut(), b"backlight\0".as_ptr());
    let i = of_property_match_string(bk_node, b"backlight-control\0".as_ptr(), type_);

    of_node_put(bk_node);
    i >= 0
}

unsafe fn pmac_backlight_key_worker(_work: *mut work_struct) {
    if atomic_read(&KERNEL_BACKLIGHT_DISABLED) != 0 {
        return;
    }

    mutex_lock(&mut pmac_backlight_mutex);
    if !pmac_backlight.is_null() {
        let props = &mut (*pmac_backlight).props;
        let mut brightness = props.brightness
            + ((if PMAC_BACKLIGHT_KEY_QUEUED != 0 { -1 } else { 1 })
                * (props.max_brightness / 15));

        if brightness < 0 {
            brightness = 0;
        } else if brightness > props.max_brightness {
            brightness = props.max_brightness;
        }

        props.brightness = brightness;
        backlight_update_status(pmac_backlight);
    }
    mutex_unlock(&mut pmac_backlight_mutex);
}

#[no_mangle]
pub unsafe extern "C" fn pmac_backlight_key(direction: i32) {
    if atomic_read(&KERNEL_BACKLIGHT_DISABLED) != 0 {
        return;
    }

    PMAC_BACKLIGHT_KEY_QUEUED = direction;
    schedule_work(&mut pmac_backlight_key_work);
}

unsafe fn __pmac_backlight_set_legacy_brightness(brightness: i32) -> i32 {
    let mut error = -6; // -ENXIO

    mutex_lock(&mut pmac_backlight_mutex);
    if !pmac_backlight.is_null() {
        let props = &mut (*pmac_backlight).props;
        props.brightness = brightness * (props.max_brightness + 1) / (OLD_BACKLIGHT_MAX + 1);

        if props.brightness > props.max_brightness {
            props.brightness = props.max_brightness;
        } else if props.brightness < 0 {
            props.brightness = 0;
        }

        backlight_update_status(pmac_backlight);
        error = 0;
    }
    mutex_unlock(&mut pmac_backlight_mutex);
    error
}

unsafe fn pmac_backlight_set_legacy_worker(_work: *mut work_struct) {
    if atomic_read(&KERNEL_BACKLIGHT_DISABLED) != 0 {
        return;
    }
    __pmac_backlight_set_legacy_brightness(PMAC_BACKLIGHT_SET_LEGACY_QUEUED);
}

#[no_mangle]
pub unsafe extern "C" fn pmac_backlight_set_legacy_brightness_pmu(brightness: i32) {
    if atomic_read(&KERNEL_BACKLIGHT_DISABLED) != 0 {
        return;
    }
    PMAC_BACKLIGHT_SET_LEGACY_QUEUED = brightness;
    schedule_work(&mut pmac_backlight_set_legacy_work);
}

#[no_mangle]
pub unsafe extern "C" fn pmac_backlight_set_legacy_brightness(brightness: i32) -> i32 {
    __pmac_backlight_set_legacy_brightness(brightness)
}

#[no_mangle]
pub unsafe extern "C" fn pmac_backlight_get_legacy_brightness() -> i32 {
    let mut result = -6; // -ENXIO

    mutex_lock(&mut pmac_backlight_mutex);
    if !pmac_backlight.is_null() {
        let props = &(*pmac_backlight).props;
        result = props.brightness * (OLD_BACKLIGHT_MAX + 1) / (props.max_brightness + 1);
    }
    mutex_unlock(&mut pmac_backlight_mutex);
    result
}

#[no_mangle]
pub unsafe extern "C" fn pmac_backlight_disable() {
    atomic_inc(&mut KERNEL_BACKLIGHT_DISABLED);
}

#[no_mangle]
pub unsafe extern "C" fn pmac_backlight_enable() {
    atomic_dec(&mut KERNEL_BACKLIGHT_DISABLED);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
