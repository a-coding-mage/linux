/* SPDX-License-Identifier: GPL-2.0 */

// Translated from linux/usb/typec_retimer.h.
// The declarations supplied by linux/property.h and linux/usb/typec.h remain
// external dependencies of this translation.

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct typec_retimer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct typec_altmode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct typec_retimer_state {
    pub alt: *mut typec_altmode,
    pub mode: c_ulong,
    pub data: *mut c_void,
}

pub type typec_retimer_set_fn_t =
    Option<unsafe extern "C" fn(retimer: *mut typec_retimer, state: *mut typec_retimer_state) -> c_int>;

#[repr(C)]
pub struct typec_retimer_desc {
    pub fwnode: *mut fwnode_handle,
    pub set: typec_retimer_set_fn_t,
    pub name: *const c_char,
    pub drvdata: *mut c_void,
}

extern "C" {
    pub fn dev_fwnode(dev: *mut device) -> *mut fwnode_handle;

    pub fn fwnode_typec_retimer_get(fwnode: *mut fwnode_handle) -> *mut typec_retimer;
    pub fn typec_retimer_put(retimer: *mut typec_retimer);
    pub fn typec_retimer_set(
        retimer: *mut typec_retimer,
        state: *mut typec_retimer_state,
    ) -> c_int;

    pub fn typec_retimer_register(
        parent: *mut device,
        desc: *const typec_retimer_desc,
    ) -> *mut typec_retimer;
    pub fn typec_retimer_unregister(retimer: *mut typec_retimer);

    pub fn typec_retimer_get_drvdata(retimer: *mut typec_retimer) -> *mut c_void;
}

#[inline]
pub unsafe fn typec_retimer_get(dev: *mut device) -> *mut typec_retimer {
    fwnode_typec_retimer_get(dev_fwnode(dev))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
