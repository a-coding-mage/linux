/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Generic framer profider header file
 *
 * Copyright 2023 CS GROUP France
 *
 * Author: Herve Codina <herve.codina@bootlin.com>
 */

// C dependencies: linux/export.h, linux/framer/framer.h, linux/types.h

pub const FRAMER_FLAG_POLL_STATUS: u32 = 1u32 << 0;

#[repr(C)]
pub struct framer_ops {
    pub init: Option<unsafe extern "C" fn(framer: *mut framer) -> i32>,
    pub exit: Option<unsafe extern "C" fn(framer: *mut framer)>,
    pub power_on: Option<unsafe extern "C" fn(framer: *mut framer) -> i32>,
    pub power_off: Option<unsafe extern "C" fn(framer: *mut framer) -> i32>,
    pub get_status: Option<unsafe extern "C" fn(framer: *mut framer, status: *mut framer_status) -> i32>,
    pub set_config: Option<unsafe extern "C" fn(framer: *mut framer, config: *const framer_config) -> i32>,
    pub get_config: Option<unsafe extern "C" fn(framer: *mut framer, config: *mut framer_config) -> i32>,
    pub flags: u32,
    pub owner: *mut module,
}

#[repr(C)]
pub struct framer_provider {
    pub dev: *mut device,
    pub owner: *mut module,
    pub list: list_head,
    pub of_xlate: Option<unsafe extern "C" fn(dev: *mut device, args: *const of_phandle_args) -> *mut framer>,
}

pub unsafe extern "C" fn framer_set_drvdata(framer: *mut framer, data: *mut core::ffi::c_void) {
    dev_set_drvdata(unsafe { &mut (*framer).dev }, data);
}

pub unsafe extern "C" fn framer_get_drvdata(framer: *mut framer) -> *mut core::ffi::c_void {
    dev_get_drvdata(unsafe { &mut (*framer).dev })
}

#[cfg(CONFIG_GENERIC_FRAMER)]
extern "C" {
    pub fn framer_create(dev: *mut device, node: *mut device_node, ops: *const framer_ops) -> *mut framer;
    pub fn framer_destroy(framer: *mut framer);
    pub fn devm_framer_create(dev: *mut device, node: *mut device_node, ops: *const framer_ops) -> *mut framer;
    pub fn framer_provider_simple_of_xlate(dev: *mut device, args: *const of_phandle_args) -> *mut framer;
    pub fn __framer_provider_of_register(dev: *mut device, owner: *mut module,
        of_xlate: Option<unsafe extern "C" fn(*mut device, *const of_phandle_args) -> *mut framer>) -> *mut framer_provider;
    pub fn framer_provider_of_unregister(framer_provider: *mut framer_provider);
    pub fn __devm_framer_provider_of_register(dev: *mut device, owner: *mut module,
        of_xlate: Option<unsafe extern "C" fn(*mut device, *const of_phandle_args) -> *mut framer>) -> *mut framer_provider;
    pub fn framer_notify_status_change(framer: *mut framer);
}

#[cfg(not(CONFIG_GENERIC_FRAMER))]
pub unsafe extern "C" fn framer_create(_dev: *mut device, _node: *mut device_node, _ops: *const framer_ops) -> *mut framer { ERR_PTR(-ENOSYS) }
#[cfg(not(CONFIG_GENERIC_FRAMER))]
pub unsafe extern "C" fn framer_destroy(_framer: *mut framer) {}
#[cfg(not(CONFIG_GENERIC_FRAMER))]
pub unsafe extern "C" fn devm_framer_create(_dev: *mut device, _node: *mut device_node, _ops: *const framer_ops) -> *mut framer { ERR_PTR(-ENOSYS) }
#[cfg(not(CONFIG_GENERIC_FRAMER))]
pub unsafe extern "C" fn framer_provider_simple_of_xlate(_dev: *mut device, _args: *const of_phandle_args) -> *mut framer { ERR_PTR(-ENOSYS) }
#[cfg(not(CONFIG_GENERIC_FRAMER))]
pub unsafe extern "C" fn __framer_provider_of_register(_dev: *mut device, _owner: *mut module,
    _of_xlate: Option<unsafe extern "C" fn(*mut device, *const of_phandle_args) -> *mut framer>) -> *mut framer_provider { ERR_PTR(-ENOSYS) }
#[cfg(not(CONFIG_GENERIC_FRAMER))]
pub unsafe extern "C" fn framer_provider_of_unregister(_framer_provider: *mut framer_provider) {}
#[cfg(not(CONFIG_GENERIC_FRAMER))]
pub unsafe extern "C" fn __devm_framer_provider_of_register(_dev: *mut device, _owner: *mut module,
    _of_xlate: Option<unsafe extern "C" fn(*mut device, *const of_phandle_args) -> *mut framer>) -> *mut framer_provider { ERR_PTR(-ENOSYS) }
#[cfg(not(CONFIG_GENERIC_FRAMER))]
pub unsafe extern "C" fn framer_notify_status_change(_framer: *mut framer) {}

#[macro_export]
macro_rules! framer_provider_of_register {
    ($dev:expr, $xlate:expr) => { __framer_provider_of_register(($dev), THIS_MODULE, ($xlate)) };
}

#[macro_export]
macro_rules! devm_framer_provider_of_register {
    ($dev:expr, $xlate:expr) => { __devm_framer_provider_of_register(($dev), THIS_MODULE, ($xlate)) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
