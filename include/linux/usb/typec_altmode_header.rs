/* SPDX-License-Identifier: GPL-2.0 */

// Translated from linux/usb/typec_altmode.h.
// Dependencies supplied by the surrounding kernel bindings are intentionally
// referenced but not defined here.

pub const MODE_DISCOVERY_MAX: ::core::ffi::c_int = 6;

pub const TYPEC_STATE_SAFE: ::core::ffi::c_uint = 0;
pub const TYPEC_STATE_USB: ::core::ffi::c_uint = 1;
pub const TYPEC_STATE_MODAL: ::core::ffi::c_uint = 2;

pub const TYPEC_MODE_USB2: ::core::ffi::c_uint = TYPEC_STATE_MODAL;
pub const TYPEC_MODE_USB3: ::core::ffi::c_uint = TYPEC_MODE_USB2 + 1;
pub const TYPEC_MODE_USB4: ::core::ffi::c_uint = TYPEC_MODE_USB3 + 1;
pub const TYPEC_MODE_AUDIO: ::core::ffi::c_uint = TYPEC_MODE_USB4 + 1;
pub const TYPEC_MODE_DEBUG: ::core::ffi::c_uint = TYPEC_MODE_AUDIO + 1;

#[repr(C)]
pub struct typec_altmode {
    pub dev: device,
    pub svid: u16,
    pub mode: ::core::ffi::c_int,
    pub vdo: u32,
    pub active: u32,
    pub priority: u8,
    pub mode_selection: bool,
    pub desc: *mut ::core::ffi::c_char,
    pub ops: *const typec_altmode_ops,
    pub cable_ops: *const typec_cable_ops,
}

pub enum typec_altmode_ops {}

#[repr(C)]
pub struct typec_altmode_ops {
    pub enter: Option<unsafe extern "C" fn(*mut typec_altmode, *mut u32) -> ::core::ffi::c_int>,
    pub exit: Option<unsafe extern "C" fn(*mut typec_altmode) -> ::core::ffi::c_int>,
    pub attention: Option<unsafe extern "C" fn(*mut typec_altmode, u32)>,
    pub vdm: Option<unsafe extern "C" fn(*mut typec_altmode, u32, *const u32, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub notify: Option<unsafe extern "C" fn(*mut typec_altmode, ::core::ffi::c_ulong, *mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub activate: Option<unsafe extern "C" fn(*mut typec_altmode, ::core::ffi::c_int) -> ::core::ffi::c_int>,
}

#[repr(C)]
pub struct typec_cable_ops {
    pub enter: Option<unsafe extern "C" fn(*mut typec_altmode, typec_plug_index, *mut u32) -> ::core::ffi::c_int>,
    pub exit: Option<unsafe extern "C" fn(*mut typec_altmode, typec_plug_index) -> ::core::ffi::c_int>,
    pub vdm: Option<unsafe extern "C" fn(*mut typec_altmode, typec_plug_index, u32, *const u32, ::core::ffi::c_int) -> ::core::ffi::c_int>,
}

#[repr(C)]
pub struct typec_altmode_driver {
    pub id_table: *const typec_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut typec_altmode) -> ::core::ffi::c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut typec_altmode)>,
    pub driver: device_driver,
}

extern "C" {
    pub static typec_port_altmode_dev_type: device_type;
    pub static typec_plug_altmode_dev_type: device_type;
    pub static typec_partner_altmode_dev_type: device_type;

    pub fn dev_set_drvdata(dev: *mut device, data: *mut ::core::ffi::c_void);
    pub fn dev_get_drvdata(dev: *mut device) -> *mut ::core::ffi::c_void;
    pub fn typec_altmode_enter(altmode: *mut typec_altmode, vdo: *mut u32) -> ::core::ffi::c_int;
    pub fn typec_altmode_exit(altmode: *mut typec_altmode) -> ::core::ffi::c_int;
    pub fn typec_altmode_attention(altmode: *mut typec_altmode, vdo: u32) -> ::core::ffi::c_int;
    pub fn typec_altmode_vdm(altmode: *mut typec_altmode, header: u32, vdo: *const u32, count: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn typec_altmode_notify(altmode: *mut typec_altmode, conf: ::core::ffi::c_ulong, data: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn typec_altmode_get_partner(altmode: *mut typec_altmode) -> *const typec_altmode;
    pub fn typec_cable_altmode_enter(altmode: *mut typec_altmode, sop: typec_plug_index, vdo: *mut u32) -> ::core::ffi::c_int;
    pub fn typec_cable_altmode_exit(altmode: *mut typec_altmode, sop: typec_plug_index) -> ::core::ffi::c_int;
    pub fn typec_cable_altmode_vdm(altmode: *mut typec_altmode, sop: typec_plug_index, header: u32, vdo: *const u32, count: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn typec_altmode_get_plug(altmode: *mut typec_altmode, index: typec_plug_index) -> *mut typec_altmode;
    pub fn typec_altmode_put_plug(plug: *mut typec_altmode);
    pub fn typec_match_altmode(altmodes: *mut *mut typec_altmode, n: usize, svid: u16, mode: u8) -> *mut typec_altmode;
    pub fn __typec_altmode_register_driver(drv: *mut typec_altmode_driver, module: *mut module) -> ::core::ffi::c_int;
    pub fn typec_altmode_unregister_driver(drv: *mut typec_altmode_driver);
    pub fn typec_mode_selection_start(partner: *mut typec_partner, delay: ::core::ffi::c_uint, timeout: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn typec_altmode_state_update(partner: *mut typec_partner, svid: u16, result: ::core::ffi::c_int);
    pub fn typec_mode_selection_delete(partner: *mut typec_partner);
    pub fn typec_get_cable_svdm_version(port: *mut typec_port) -> ::core::ffi::c_int;
    pub fn typec_get_orientation(port: *mut typec_port) -> typec_orientation;
    pub fn typec_get_negotiated_svdm_version(port: *mut typec_port) -> ::core::ffi::c_int;
    pub fn typec_get_data_role(port: *mut typec_port) -> typec_data_role;
}

#[inline]
pub unsafe fn typec_altmode_set_drvdata(altmode: *mut typec_altmode, data: *mut ::core::ffi::c_void) {
    dev_set_drvdata(&mut (*altmode).dev, data);
}

#[inline]
pub unsafe fn typec_altmode_get_drvdata(altmode: *mut typec_altmode) -> *mut ::core::ffi::c_void {
    dev_get_drvdata(&mut (*altmode).dev)
}

#[inline]
pub unsafe fn typec_altmode_get_cable_svdm_version(altmode: *mut typec_altmode) -> ::core::ffi::c_int {
    typec_get_cable_svdm_version(typec_altmode2port(altmode))
}

#[inline]
pub unsafe fn typec_altmode_get_orientation(altmode: *mut typec_altmode) -> typec_orientation {
    typec_get_orientation(typec_altmode2port(altmode))
}

#[inline]
pub unsafe fn typec_altmode_get_svdm_version(altmode: *mut typec_altmode) -> ::core::ffi::c_int {
    typec_get_negotiated_svdm_version(typec_altmode2port(altmode))
}

#[inline]
pub unsafe fn typec_altmode_get_data_role(altmode: *mut typec_altmode) -> typec_data_role {
    typec_get_data_role(typec_altmode2port(altmode))
}

#[inline]
pub const fn typec_modal_state(state: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    state + TYPEC_STATE_MODAL
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
