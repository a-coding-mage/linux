/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * HD audio Component Binding Interface
 *
 * Copyright (C) 2021 Cirrus Logic, Inc. and
 *                    Cirrus Logic International Semiconductor Ltd.
 */

use core::ffi::{c_char, c_int, c_void};

/* Dependencies from the original C header:
 * <linux/acpi.h>
 * <linux/component.h>
 * <linux/mutex.h>
 * <sound/hda_codec.h>
 */

pub const HDA_MAX_COMPONENTS: usize = 4;
pub const HDA_MAX_NAME_SIZE: usize = 50;

pub type u32 = u32;
pub type acpi_handle = *mut c_void;
pub type acpi_notify_handler =
    Option<unsafe extern "C" fn(handle: acpi_handle, event: u32, data: *mut c_void)>;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acpi_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hda_codec {
    _private: [u8; 0],
}

#[repr(C)]
pub struct component_master_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hda_component {
    pub dev: *mut device,
    pub name: [c_char; HDA_MAX_NAME_SIZE],
    pub adev: *mut acpi_device,
    pub acpi_notifications_supported: bool,
    pub acpi_notify:
        Option<unsafe extern "C" fn(handle: acpi_handle, event: u32, dev: *mut device)>,
    pub pre_playback_hook: Option<unsafe extern "C" fn(dev: *mut device, action: c_int)>,
    pub playback_hook: Option<unsafe extern "C" fn(dev: *mut device, action: c_int)>,
    pub post_playback_hook: Option<unsafe extern "C" fn(dev: *mut device, action: c_int)>,
}

#[repr(C)]
pub struct hda_component_parent {
    pub mutex: mutex,
    pub codec: *mut hda_codec,
    pub comps: [hda_component; HDA_MAX_COMPONENTS],
}

unsafe extern "C" {
    /*
     * CONFIG_ACPI:
     * In C these are declarations when CONFIG_ACPI is enabled, and static
     * inline no-op / zero-return functions when CONFIG_ACPI is disabled.
     */
    pub fn hda_component_acpi_device_notify(
        parent: *mut hda_component_parent,
        handle: acpi_handle,
        event: u32,
        data: *mut c_void,
    );

    pub fn hda_component_manager_bind_acpi_notifications(
        cdc: *mut hda_codec,
        parent: *mut hda_component_parent,
        handler: acpi_notify_handler,
        data: *mut c_void,
    ) -> c_int;

    pub fn hda_component_manager_unbind_acpi_notifications(
        cdc: *mut hda_codec,
        parent: *mut hda_component_parent,
        handler: acpi_notify_handler,
    );

    pub fn hda_component_manager_playback_hook(
        parent: *mut hda_component_parent,
        action: c_int,
    );

    pub fn hda_component_manager_init(
        cdc: *mut hda_codec,
        parent: *mut hda_component_parent,
        count: c_int,
        bus: *const c_char,
        hid: *const c_char,
        match_str: *const c_char,
        ops: *const component_master_ops,
    ) -> c_int;

    pub fn hda_component_manager_free(
        parent: *mut hda_component_parent,
        ops: *const component_master_ops,
    );

    pub fn hda_component_manager_bind(
        cdc: *mut hda_codec,
        parent: *mut hda_component_parent,
    ) -> c_int;

    fn hda_codec_dev(cdc: *mut hda_codec) -> *mut device;
    fn component_unbind_all(master: *mut device, data: *mut c_void);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
}

#[cfg(not(CONFIG_ACPI))]
#[inline]
pub unsafe fn hda_component_acpi_device_notify_no_config_acpi(
    _parent: *mut hda_component_parent,
    _handle: acpi_handle,
    _event: u32,
    _data: *mut c_void,
) {
}

#[cfg(not(CONFIG_ACPI))]
#[inline]
pub unsafe fn hda_component_manager_bind_acpi_notifications_no_config_acpi(
    _cdc: *mut hda_codec,
    _parent: *mut hda_component_parent,
    _handler: acpi_notify_handler,
    _data: *mut c_void,
) -> c_int {
    0
}

#[cfg(not(CONFIG_ACPI))]
#[inline]
pub unsafe fn hda_component_manager_unbind_acpi_notifications_no_config_acpi(
    _cdc: *mut hda_codec,
    _parent: *mut hda_component_parent,
    _handler: acpi_notify_handler,
) {
}

#[inline]
pub unsafe fn hda_component_from_index(
    parent: *mut hda_component_parent,
    index: c_int,
) -> *mut hda_component {
    if parent.is_null() {
        return core::ptr::null_mut();
    }

    if index < 0 || index as usize >= HDA_MAX_COMPONENTS {
        return core::ptr::null_mut();
    }

    unsafe { &mut (*parent).comps[index as usize] }
}

#[inline]
pub unsafe fn hda_component_manager_unbind(
    cdc: *mut hda_codec,
    parent: *mut hda_component_parent,
) {
    /*
     * C source uses guard(mutex)(&parent->mutex), holding the mutex until
     * the function returns.
     */
    unsafe {
        mutex_lock(&mut (*parent).mutex);
        component_unbind_all(hda_codec_dev(cdc), parent.cast::<c_void>());
        mutex_unlock(&mut (*parent).mutex);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
