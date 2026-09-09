/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright 2015-2017 Google, Inc */

// Dependencies supplied by the surrounding kernel translation:
// linux/bitops.h, linux/usb/typec.h, and pd.h.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum typec_cc_status {
    TYPEC_CC_OPEN,
    TYPEC_CC_RA,
    TYPEC_CC_RD,
    TYPEC_CC_RP_DEF,
    TYPEC_CC_RP_1_5,
    TYPEC_CC_RP_3_0,
}

/* Collision Avoidance */
pub const SINK_TX_NG: typec_cc_status = typec_cc_status::TYPEC_CC_RP_1_5;
pub const SINK_TX_OK: typec_cc_status = typec_cc_status::TYPEC_CC_RP_3_0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum typec_cc_polarity {
    TYPEC_POLARITY_CC1,
    TYPEC_POLARITY_CC2,
}

/* Time to wait for TCPC to complete transmit */
pub const PD_T_TCPC_TX_TIMEOUT: u32 = 100; /* in ms */
pub const PD_ROLE_SWAP_TIMEOUT: u32 = MSEC_PER_SEC * 10;
pub const PD_AUG_PSY_CTRL_TIMEOUT: u32 = MSEC_PER_SEC * 10;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcpm_transmit_status {
    TCPC_TX_SUCCESS = 0,
    TCPC_TX_DISCARDED = 1,
    TCPC_TX_FAILED = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcpm_transmit_type {
    TCPC_TX_SOP = 0,
    TCPC_TX_SOP_PRIME = 1,
    TCPC_TX_SOP_PRIME_PRIME = 2,
    TCPC_TX_SOP_DEBUG_PRIME = 3,
    TCPC_TX_SOP_DEBUG_PRIME_PRIME = 4,
    TCPC_TX_HARD_RESET = 5,
    TCPC_TX_CABLE_RESET = 6,
    TCPC_TX_BIST_MODE_2 = 7,
}

/* Mux state attributes */
pub const TCPC_MUX_USB_ENABLED: u32 = 1 << 0; /* USB enabled */
pub const TCPC_MUX_DP_ENABLED: u32 = 1 << 1; /* DP enabled */
pub const TCPC_MUX_POLARITY_INVERTED: u32 = 1 << 2; /* Polarity inverted */

#[repr(C)]
pub struct tcpc_dev {
    pub fwnode: *mut fwnode_handle,
    pub init: Option<unsafe extern "C" fn(dev: *mut tcpc_dev) -> i32>,
    pub get_vbus: Option<unsafe extern "C" fn(dev: *mut tcpc_dev) -> i32>,
    pub get_current_limit: Option<unsafe extern "C" fn(dev: *mut tcpc_dev) -> i32>,
    pub set_cc: Option<unsafe extern "C" fn(dev: *mut tcpc_dev, cc: typec_cc_status) -> i32>,
    pub apply_rc: Option<unsafe extern "C" fn(dev: *mut tcpc_dev, cc: typec_cc_status, polarity: typec_cc_polarity) -> i32>,
    pub get_cc: Option<unsafe extern "C" fn(dev: *mut tcpc_dev, cc1: *mut typec_cc_status, cc2: *mut typec_cc_status) -> i32>,
    pub set_polarity: Option<unsafe extern "C" fn(dev: *mut tcpc_dev, polarity: typec_cc_polarity) -> i32>,
    pub set_orientation: Option<unsafe extern "C" fn(dev: *mut tcpc_dev, orientation: typec_orientation) -> i32>,
    pub set_vconn: Option<unsafe extern "C" fn(dev: *mut tcpc_dev, on: bool) -> i32>,
    pub set_vbus: Option<unsafe extern "C" fn(dev: *mut tcpc_dev, on: bool, charge: bool) -> i32>,
    pub set_current_limit: Option<unsafe extern "C" fn(dev: *mut tcpc_dev, max_ma: u32, mv: u32) -> i32>,
    pub set_pd_rx: Option<unsafe extern "C" fn(dev: *mut tcpc_dev, on: bool) -> i32>,
    pub set_roles: Option<unsafe extern "C" fn(dev: *mut tcpc_dev, attached: bool, role: typec_role, data: typec_data_role) -> i32>,
    pub start_toggling: Option<unsafe extern "C" fn(dev: *mut tcpc_dev, port_type: typec_port_type, cc: typec_cc_status) -> i32>,
    pub try_role: Option<unsafe extern "C" fn(dev: *mut tcpc_dev, role: i32) -> i32>,
    pub pd_transmit: Option<unsafe extern "C" fn(dev: *mut tcpc_dev, type_: tcpm_transmit_type, msg: *const pd_message, negotiated_rev: u32) -> i32>,
    pub set_bist_data: Option<unsafe extern "C" fn(dev: *mut tcpc_dev, on: bool) -> i32>,
    pub enable_frs: Option<unsafe extern "C" fn(dev: *mut tcpc_dev, enable: bool) -> i32>,
    pub frs_sourcing_vbus: Option<unsafe extern "C" fn(dev: *mut tcpc_dev)>,
    pub enable_auto_vbus_discharge: Option<unsafe extern "C" fn(dev: *mut tcpc_dev, enable: bool) -> i32>,
    pub set_auto_vbus_discharge_threshold: Option<unsafe extern "C" fn(dev: *mut tcpc_dev, mode: typec_pwr_opmode, pps_active: bool, requested_vbus_voltage: u32, pps_apdo_min_voltage: u32) -> i32>,
    pub is_vbus_vsafe0v: Option<unsafe extern "C" fn(dev: *mut tcpc_dev) -> bool>,
    pub set_partner_usb_comm_capable: Option<unsafe extern "C" fn(dev: *mut tcpc_dev, enable: bool)>,
    pub check_contaminant: Option<unsafe extern "C" fn(dev: *mut tcpc_dev)>,
    pub cable_comm_capable: Option<unsafe extern "C" fn(dev: *mut tcpc_dev) -> bool>,
    pub attempt_vconn_swap_discovery: Option<unsafe extern "C" fn(dev: *mut tcpc_dev) -> bool>,
}

#[repr(C)]
pub struct tcpm_port;

extern "C" {
    pub fn tcpm_register_port(dev: *mut device, tcpc: *mut tcpc_dev) -> *mut tcpm_port;
    pub fn tcpm_unregister_port(port: *mut tcpm_port);
    pub fn tcpm_vbus_change(port: *mut tcpm_port);
    pub fn tcpm_cc_change(port: *mut tcpm_port);
    pub fn tcpm_sink_frs(port: *mut tcpm_port);
    pub fn tcpm_sourcing_vbus(port: *mut tcpm_port);
    pub fn tcpm_pd_receive(port: *mut tcpm_port, msg: *const pd_message, rx_sop_type: tcpm_transmit_type);
    pub fn tcpm_pd_transmit_complete(port: *mut tcpm_port, status: tcpm_transmit_status);
    pub fn tcpm_pd_hard_reset(port: *mut tcpm_port);
    pub fn tcpm_tcpc_reset(port: *mut tcpm_port);
    pub fn tcpm_port_clean(port: *mut tcpm_port);
    pub fn tcpm_port_is_toggling(port: *mut tcpm_port) -> bool;
    pub fn tcpm_port_error_recovery(port: *mut tcpm_port);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
