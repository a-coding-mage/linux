/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from linux/usb/typec.h. */

pub const USB_TYPEC_REV_1_0: u16 = 0x100;
pub const USB_TYPEC_REV_1_1: u16 = 0x110;
pub const USB_TYPEC_REV_1_2: u16 = 0x120;
pub const USB_TYPEC_REV_1_3: u16 = 0x130;
pub const USB_TYPEC_REV_1_4: u16 = 0x140;
pub const USB_TYPEC_REV_2_0: u16 = 0x200;

#[repr(C)] pub struct typec_partner { _private: [u8; 0] }
#[repr(C)] pub struct typec_cable { _private: [u8; 0] }
#[repr(C)] pub struct typec_plug { _private: [u8; 0] }
#[repr(C)] pub struct typec_port { _private: [u8; 0] }
#[repr(C)] pub struct typec_altmode { _private: [u8; 0] }
#[repr(C)] pub struct typec_altmode_ops { _private: [u8; 0] }
#[repr(C)] pub struct typec_cable_ops { _private: [u8; 0] }
#[repr(C)] pub struct bus_type { _private: [u8; 0] }
#[repr(C)] pub struct fwnode_handle { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct usb_power_delivery { _private: [u8; 0] }
#[repr(C)] pub struct usb_power_delivery_desc { _private: [u8; 0] }

extern "C" { pub static typec_bus: bus_type; }

#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum typec_port_type { TYPEC_PORT_SRC, TYPEC_PORT_SNK, TYPEC_PORT_DRP }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum typec_port_data { TYPEC_PORT_DFP, TYPEC_PORT_UFP, TYPEC_PORT_DRD }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum typec_plug_type { USB_PLUG_NONE, USB_PLUG_TYPE_A, USB_PLUG_TYPE_B, USB_PLUG_TYPE_C, USB_PLUG_CAPTIVE }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum typec_data_role { TYPEC_DEVICE, TYPEC_HOST }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum typec_role { TYPEC_SINK, TYPEC_SOURCE }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum typec_pwr_opmode { TYPEC_PWR_MODE_USB, TYPEC_PWR_MODE_1_5A, TYPEC_PWR_MODE_3_0A, TYPEC_PWR_MODE_PD }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum typec_accessory { TYPEC_ACCESSORY_NONE, TYPEC_ACCESSORY_AUDIO, TYPEC_ACCESSORY_DEBUG }
pub const TYPEC_MAX_ACCESSORY: usize = 3;
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum typec_orientation { TYPEC_ORIENTATION_NONE, TYPEC_ORIENTATION_NORMAL, TYPEC_ORIENTATION_REVERSE }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum usb_mode { USB_MODE_NONE, USB_MODE_USB2, USB_MODE_USB3, USB_MODE_USB4 }
pub const USB_CAPABILITY_USB2: u8 = 1 << 0;
pub const USB_CAPABILITY_USB3: u8 = 1 << 1;
pub const USB_CAPABILITY_USB4: u8 = 1 << 2;

#[repr(C)] pub struct enter_usb_data { pub eudo: u32, pub active_link_training: u8 }
#[repr(C)] pub struct usb_pd_identity { pub id_header: u32, pub cert_stat: u32, pub product: u32, pub vdo: [u32; 3] }

#[repr(C)] pub struct typec_altmode_desc { pub svid: u16, pub mode: u8, pub vdo: u32, pub roles: typec_port_data, pub inactive: bool, pub mode_selection: bool }
#[repr(C)] pub struct typec_plug_desc { pub index: typec_plug_index }
#[repr(C)] pub struct typec_cable_desc { pub type_: typec_plug_type, pub active: u32, pub identity: *mut usb_pd_identity, pub pd_revision: u16 }
#[repr(C)] pub struct typec_partner_desc {
    pub usb_pd: u32, pub accessory: typec_accessory, pub identity: *mut usb_pd_identity,
    pub pd_revision: u16, pub usb_capability: u8,
    pub attach: Option<unsafe extern "C" fn(*mut typec_partner, *mut device)>,
    pub deattach: Option<unsafe extern "C" fn(*mut typec_partner, *mut device)>,
}
#[repr(C)] pub struct typec_operations {
    pub try_role: Option<unsafe extern "C" fn(*mut typec_port, i32) -> i32>,
    pub dr_set: Option<unsafe extern "C" fn(*mut typec_port, typec_data_role) -> i32>,
    pub pr_set: Option<unsafe extern "C" fn(*mut typec_port, typec_role) -> i32>,
    pub vconn_set: Option<unsafe extern "C" fn(*mut typec_port, typec_role) -> i32>,
    pub port_type_set: Option<unsafe extern "C" fn(*mut typec_port, typec_port_type) -> i32>,
    pub pd_get: Option<unsafe extern "C" fn(*mut typec_port) -> *mut *mut usb_power_delivery>,
    pub pd_set: Option<unsafe extern "C" fn(*mut typec_port, *mut usb_power_delivery) -> i32>,
    pub default_usb_mode_set: Option<unsafe extern "C" fn(*mut typec_port, usb_mode) -> i32>,
    pub enter_usb_mode: Option<unsafe extern "C" fn(*mut typec_port, usb_mode) -> i32>,
}
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum usb_pd_svdm_ver { SVDM_VER_1_0 = 0, SVDM_VER_2_0 = 1, SVDM_VER_MAX = 1 }
#[repr(C)] pub struct typec_capability {
    pub type_: typec_port_type, pub data: typec_port_data, pub revision: u16, pub pd_revision: u16,
    pub svdm_version: usb_pd_svdm_ver, pub prefer_role: i32, pub accessory: [typec_accessory; TYPEC_MAX_ACCESSORY],
    pub orientation_aware: u32, pub usb_capability: u8, pub no_mode_control: bool,
    pub fwnode: *mut fwnode_handle, pub driver_data: *mut core::ffi::c_void,
    pub pd: *mut usb_power_delivery, pub ops: *const typec_operations,
}
pub const TYPEC_NO_PREFERRED_ROLE: i32 = -1;
#[repr(C)] pub enum typec_plug_index { TYPEC_PLUG_SOP_P, TYPEC_PLUG_SOP_PP }
#[repr(C)] pub struct typec_connector {
    pub attach: Option<unsafe extern "C" fn(*mut typec_connector, *mut device)>,
    pub deattach: Option<unsafe extern "C" fn(*mut typec_connector, *mut device)>,
}

#[inline] pub fn is_sink(role: typec_role) -> i32 { (role == typec_role::TYPEC_SINK) as i32 }
#[inline] pub fn is_source(role: typec_role) -> i32 { (role == typec_role::TYPEC_SOURCE) as i32 }

extern "C" {
    pub fn typec_partner_set_identity(*mut typec_partner) -> i32;
    pub fn typec_cable_set_identity(*mut typec_cable) -> i32;
    pub fn typec_partner_set_pd_revision(*mut typec_partner, u16);
    pub fn typec_partner_set_num_altmodes(*mut typec_partner, i32) -> i32;
    pub fn typec_partner_register_altmode(*mut typec_partner, *const typec_altmode_desc) -> *mut typec_altmode;
    pub fn typec_plug_set_num_altmodes(*mut typec_plug, i32) -> i32;
    pub fn typec_plug_register_altmode(*mut typec_plug, *const typec_altmode_desc) -> *mut typec_altmode;
    pub fn typec_port_register_altmode(*mut typec_port, *const typec_altmode_desc) -> *mut typec_altmode;
    pub fn typec_port_register_altmodes(*mut typec_port, *const typec_altmode_ops, *mut core::ffi::c_void, *mut *mut typec_altmode, usize);
    pub fn typec_port_register_cable_ops(*mut *mut typec_altmode, i32, *const typec_cable_ops);
    pub fn typec_unregister_altmode(*mut typec_altmode);
    pub fn typec_altmode2port(*mut typec_altmode) -> *mut typec_port;
    pub fn typec_altmode_update_active(*mut typec_altmode, bool);
    pub fn typec_altmode_set_ops(*mut typec_altmode, *const typec_altmode_ops);
    pub fn typec_register_port(*mut device, *const typec_capability) -> *mut typec_port;
    pub fn typec_unregister_port(*mut typec_port);
    pub fn typec_register_partner(*mut typec_port, *mut typec_partner_desc) -> *mut typec_partner;
    pub fn typec_unregister_partner(*mut typec_partner);
    pub fn typec_register_cable(*mut typec_port, *mut typec_cable_desc) -> *mut typec_cable;
    pub fn typec_unregister_cable(*mut typec_cable);
    pub fn typec_cable_get(*mut typec_port) -> *mut typec_cable;
    pub fn typec_cable_put(*mut typec_cable);
    pub fn typec_cable_is_active(*mut typec_cable) -> i32;
    pub fn typec_cable_altmode_unsupported(*mut typec_altmode) -> bool;
    pub fn typec_register_plug(*mut typec_cable, *mut typec_plug_desc) -> *mut typec_plug;
    pub fn typec_unregister_plug(*mut typec_plug);
    pub fn typec_set_data_role(*mut typec_port, typec_data_role);
    pub fn typec_get_data_role(*mut typec_port) -> typec_data_role;
    pub fn typec_set_pwr_role(*mut typec_port, typec_role);
    pub fn typec_set_vconn_role(*mut typec_port, typec_role);
    pub fn typec_set_pwr_opmode(*mut typec_port, typec_pwr_opmode);
    pub fn typec_set_orientation(*mut typec_port, typec_orientation) -> i32;
    pub fn typec_get_orientation(*mut typec_port) -> typec_orientation;
    pub fn typec_set_mode(*mut typec_port, i32) -> i32;
    pub fn typec_get_drvdata(*mut typec_port) -> *mut core::ffi::c_void;
    pub fn typec_get_fw_cap(*mut typec_capability, *mut fwnode_handle) -> i32;
    pub fn typec_find_pwr_opmode(*const core::ffi::c_char) -> i32;
    pub fn typec_find_orientation(*const core::ffi::c_char) -> i32;
    pub fn typec_find_port_power_role(*const core::ffi::c_char) -> i32;
    pub fn typec_find_power_role(*const core::ffi::c_char) -> i32;
    pub fn typec_find_port_data_role(*const core::ffi::c_char) -> i32;
    pub fn typec_partner_set_svdm_version(*mut typec_partner, usb_pd_svdm_ver);
    pub fn typec_get_negotiated_svdm_version(*mut typec_port) -> i32;
    pub fn typec_get_cable_svdm_version(*mut typec_port) -> i32;
    pub fn typec_cable_set_svdm_version(*mut typec_cable, usb_pd_svdm_ver);
    pub fn typec_partner_usb_power_delivery_register(*mut typec_partner, *mut usb_power_delivery_desc) -> *mut usb_power_delivery;
    pub fn typec_port_set_usb_power_delivery(*mut typec_port, *mut usb_power_delivery) -> i32;
    pub fn typec_partner_set_usb_power_delivery(*mut typec_partner, *mut usb_power_delivery) -> i32;
    pub fn typec_partner_set_usb_mode(*mut typec_partner, usb_mode);
    pub fn typec_port_set_usb_mode(*mut typec_port, usb_mode);
}

#[inline] pub unsafe fn typec_attach(con: *mut typec_connector, dev: *mut device) { if !con.is_null() { if let Some(f) = (*con).attach { f(con, dev); } } }
#[inline] pub unsafe fn typec_deattach(con: *mut typec_connector, dev: *mut device) { if !con.is_null() { if let Some(f) = (*con).deattach { f(con, dev); } } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
