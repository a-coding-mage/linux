// SPDX-License-Identifier: GPL-2.0

// C header dependencies: linux/err.h, linux/property.h, linux/usb/typec.h

use core::ffi::c_void;

pub enum device {}
pub enum typec_mux {}
pub enum typec_mux_dev {}
pub enum typec_switch {}
pub enum typec_switch_dev {}
pub enum typec_altmode {}
pub enum fwnode_handle {}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum typec_orientation {
    TYPEC_ORIENTATION_NONE = 0,
}

pub type typec_switch_set_fn_t = unsafe extern "C" fn(
    sw: *mut typec_switch_dev,
    orientation: typec_orientation,
) -> i32;

#[repr(C)]
pub struct typec_switch_desc {
    pub fwnode: *mut fwnode_handle,
    pub set: Option<typec_switch_set_fn_t>,
    pub name: *const i8,
    pub drvdata: *mut c_void,
}

#[cfg(feature = "CONFIG_TYPEC")]
extern "C" {
    pub fn fwnode_typec_switch_get(fwnode: *mut fwnode_handle) -> *mut typec_switch;
    pub fn typec_switch_put(sw: *mut typec_switch);
    pub fn typec_switch_set(sw: *mut typec_switch, orientation: typec_orientation) -> i32;
    pub fn typec_switch_register(
        parent: *mut device,
        desc: *const typec_switch_desc,
    ) -> *mut typec_switch_dev;
    pub fn typec_switch_unregister(sw: *mut typec_switch_dev);
    pub fn typec_switch_set_drvdata(sw: *mut typec_switch_dev, data: *mut c_void);
    pub fn typec_switch_get_drvdata(sw: *mut typec_switch_dev) -> *mut c_void;
}

#[cfg(not(feature = "CONFIG_TYPEC"))]
pub unsafe fn fwnode_typec_switch_get(_fwnode: *mut fwnode_handle) -> *mut typec_switch { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_TYPEC"))]
pub unsafe fn typec_switch_put(_sw: *mut typec_switch) {}
#[cfg(not(feature = "CONFIG_TYPEC"))]
pub unsafe fn typec_switch_set(_sw: *mut typec_switch, _orientation: typec_orientation) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_TYPEC"))]
pub unsafe fn typec_switch_register(_parent: *mut device, _desc: *const typec_switch_desc) -> *mut typec_switch_dev { (-95isize) as *mut typec_switch_dev }
#[cfg(not(feature = "CONFIG_TYPEC"))]
pub unsafe fn typec_switch_unregister(_sw: *mut typec_switch_dev) {}
#[cfg(not(feature = "CONFIG_TYPEC"))]
pub unsafe fn typec_switch_set_drvdata(_sw: *mut typec_switch_dev, _data: *mut c_void) {}
#[cfg(not(feature = "CONFIG_TYPEC"))]
pub unsafe fn typec_switch_get_drvdata(_sw: *mut typec_switch_dev) -> *mut c_void { (-95isize) as *mut c_void }

pub unsafe fn typec_switch_get(dev: *mut device) -> *mut typec_switch {
    fwnode_typec_switch_get(dev_fwnode(dev))
}

#[repr(C)]
pub struct typec_mux_state {
    pub alt: *mut typec_altmode,
    pub mode: usize,
    pub data: *mut c_void,
}

pub type typec_mux_set_fn_t = unsafe extern "C" fn(
    mux: *mut typec_mux_dev,
    state: *mut typec_mux_state,
) -> i32;

#[repr(C)]
pub struct typec_mux_desc {
    pub fwnode: *mut fwnode_handle,
    pub set: Option<typec_mux_set_fn_t>,
    pub name: *const i8,
    pub drvdata: *mut c_void,
}

#[cfg(feature = "CONFIG_TYPEC")]
extern "C" {
    pub fn fwnode_typec_mux_get(fwnode: *mut fwnode_handle) -> *mut typec_mux;
    pub fn typec_mux_put(mux: *mut typec_mux);
    pub fn typec_mux_set(mux: *mut typec_mux, state: *mut typec_mux_state) -> i32;
    pub fn typec_mux_register(parent: *mut device, desc: *const typec_mux_desc) -> *mut typec_mux_dev;
    pub fn typec_mux_unregister(mux: *mut typec_mux_dev);
    pub fn typec_mux_set_drvdata(mux: *mut typec_mux_dev, data: *mut c_void);
    pub fn typec_mux_get_drvdata(mux: *mut typec_mux_dev) -> *mut c_void;
}

#[cfg(not(feature = "CONFIG_TYPEC"))]
pub unsafe fn fwnode_typec_mux_get(_fwnode: *mut fwnode_handle) -> *mut typec_mux { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_TYPEC"))]
pub unsafe fn typec_mux_put(_mux: *mut typec_mux) {}
#[cfg(not(feature = "CONFIG_TYPEC"))]
pub unsafe fn typec_mux_set(_mux: *mut typec_mux, _state: *mut typec_mux_state) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_TYPEC"))]
pub unsafe fn typec_mux_register(_parent: *mut device, _desc: *const typec_mux_desc) -> *mut typec_mux_dev { (-95isize) as *mut typec_mux_dev }
#[cfg(not(feature = "CONFIG_TYPEC"))]
pub unsafe fn typec_mux_unregister(_mux: *mut typec_mux_dev) {}
#[cfg(not(feature = "CONFIG_TYPEC"))]
pub unsafe fn typec_mux_set_drvdata(_mux: *mut typec_mux_dev, _data: *mut c_void) {}
#[cfg(not(feature = "CONFIG_TYPEC"))]
pub unsafe fn typec_mux_get_drvdata(_mux: *mut typec_mux_dev) -> *mut c_void { (-95isize) as *mut c_void }

pub unsafe fn typec_mux_get(dev: *mut device) -> *mut typec_mux {
    fwnode_typec_mux_get(dev_fwnode(dev))
}

extern "C" {
    pub fn dev_fwnode(dev: *mut device) -> *mut fwnode_handle;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
