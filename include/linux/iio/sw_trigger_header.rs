/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Industrial I/O software trigger interface
 *
 * Copyright (c) 2015 Intel Corporation
 */

/* Dependencies supplied by the surrounding kernel translation. */

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iio_trigger {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct config_group {
    _private: [u8; 0],
}

#[repr(C)]
pub struct config_item {
    _private: [u8; 0],
}

#[repr(C)]
pub struct config_item_type {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iio_sw_trigger_ops;

#[repr(C)]
pub struct iio_sw_trigger_type {
    pub name: *const core::ffi::c_char,
    pub owner: *mut module,
    pub ops: *const iio_sw_trigger_ops,
    pub list: list_head,
    pub group: *mut config_group,
}

#[repr(C)]
pub struct iio_sw_trigger {
    pub trigger: *mut iio_trigger,
    pub trigger_type: *mut iio_sw_trigger_type,
    pub group: config_group,
}

pub type IioSwTriggerProbe = unsafe extern "C" fn(
    name: *const core::ffi::c_char,
) -> *mut iio_sw_trigger;
pub type IioSwTriggerRemove = unsafe extern "C" fn(
    trigger: *mut iio_sw_trigger,
) -> core::ffi::c_int;

#[repr(C)]
pub struct iio_sw_trigger_ops {
    pub probe: Option<IioSwTriggerProbe>,
    pub remove: Option<IioSwTriggerRemove>,
}

/*
 * The C macro module_iio_sw_trigger_driver expands to module_driver with
 * iio_register_sw_trigger_type and iio_unregister_sw_trigger_type.
 */

unsafe extern "C" {
    pub fn iio_register_sw_trigger_type(tt: *mut iio_sw_trigger_type)
        -> core::ffi::c_int;
    pub fn iio_unregister_sw_trigger_type(tt: *mut iio_sw_trigger_type);

    pub fn iio_sw_trigger_create(
        name: *const core::ffi::c_char,
        parent: *const core::ffi::c_char,
    ) -> *mut iio_sw_trigger;
    pub fn iio_sw_trigger_destroy(trigger: *mut iio_sw_trigger);
}

/* Equivalent of container_of(to_config_group(item), iio_sw_trigger, group). */
#[inline]
pub unsafe fn to_iio_sw_trigger(item: *mut config_item) -> *mut iio_sw_trigger {
    let group = to_config_group(item);
    let offset = core::mem::offset_of!(iio_sw_trigger, group);
    (group as *mut u8).sub(offset) as *mut iio_sw_trigger
}

unsafe extern "C" {
    pub fn to_config_group(item: *mut config_item) -> *mut config_group;
}

#[inline]
pub unsafe fn iio_swt_group_init_type_name(
    t: *mut iio_sw_trigger,
    name: *const core::ffi::c_char,
    type_: *const config_item_type,
) {
    /* Preserved conditional intent: CONFIG_CONFIGFS_FS controls this call. */
    #[cfg(CONFIG_CONFIGFS_FS)]
    {
        config_group_init_type_name(&mut (*t).group, name, type_);
    }
}

unsafe extern "C" {
    pub fn config_group_init_type_name(
        group: *mut config_group,
        name: *const core::ffi::c_char,
        type_: *const config_item_type,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
