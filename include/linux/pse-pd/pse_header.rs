// SPDX-License-Identifier: GPL-2.0-only
/*
// Copyright (c) 2022 Pengutronix, Oleksij Rempel <kernel@pengutronix.de>
*/

pub const MAX_PI_CURRENT: u32 = 1920000;
pub const MAX_PI_PW: u32 = 99900;

#[repr(C)]
pub struct net_device;
#[repr(C)]
pub struct phy_device;
#[repr(C)]
pub struct pse_controller_dev;
#[repr(C)]
pub struct netlink_ext_ack;
#[repr(C)]
pub struct module;
#[repr(C)]
pub struct device_node;
#[repr(C)]
pub struct of_phandle_args;
#[repr(C)]
pub struct pse_control;
#[repr(C)]
pub struct device;
#[repr(C)]
pub struct regulator_dev;
#[repr(C)]
pub struct pse_power_domain;

// These enum types are supplied by the included Linux/UAPI headers.
pub type ethtool_c33_pse_ext_state = u32;
pub type ethtool_c33_pse_ext_substate_error_condition = u32;
pub type ethtool_c33_pse_ext_substate_mr_pse_enable = u32;
pub type ethtool_c33_pse_ext_substate_option_detect_ted = u32;
pub type ethtool_c33_pse_ext_substate_option_vport_lim = u32;
pub type ethtool_c33_pse_ext_substate_ovld_detected = u32;
pub type ethtool_c33_pse_ext_substate_power_not_available = u32;
pub type ethtool_c33_pse_ext_substate_short_detected = u32;
pub type ethtool_podl_pse_admin_state = u32;
pub type ethtool_c33_pse_admin_state = u32;
pub type ethtool_podl_pse_pw_d_status = u32;
pub type ethtool_c33_pse_pw_d_status = u32;
pub type ethtool_pse_types = u32;

#[repr(C)]
pub union ethtool_c33_pse_ext_state_info_substate {
    pub error_condition: ethtool_c33_pse_ext_substate_error_condition,
    pub mr_pse_enable: ethtool_c33_pse_ext_substate_mr_pse_enable,
    pub option_detect_ted: ethtool_c33_pse_ext_substate_option_detect_ted,
    pub option_vport_lim: ethtool_c33_pse_ext_substate_option_vport_lim,
    pub ovld_detected: ethtool_c33_pse_ext_substate_ovld_detected,
    pub power_not_available: ethtool_c33_pse_ext_substate_power_not_available,
    pub short_detected: ethtool_c33_pse_ext_substate_short_detected,
    pub c33_pse_ext_substate: u32,
}

#[repr(C)]
pub struct ethtool_c33_pse_ext_state_info {
    pub c33_pse_ext_state: ethtool_c33_pse_ext_state,
    pub substate: ethtool_c33_pse_ext_state_info_substate,
}

#[repr(C)]
pub struct ethtool_c33_pse_pw_limit_range { pub min: u32, pub max: u32 }

#[repr(C)]
pub struct pse_irq_desc {
    pub name: *const u8,
    pub map_event: Option<unsafe extern "C" fn(i32, *mut pse_controller_dev, *mut c_ulong, *mut c_ulong) -> i32>,
}

#[repr(C)]
pub struct pse_control_config {
    pub podl_admin_control: ethtool_podl_pse_admin_state,
    pub c33_admin_control: ethtool_c33_pse_admin_state,
}
#[repr(C)]
pub struct pse_admin_state { pub podl_admin_state: ethtool_podl_pse_admin_state, pub c33_admin_state: ethtool_c33_pse_admin_state }
#[repr(C)]
pub struct pse_pw_status { pub podl_pw_status: ethtool_podl_pse_pw_d_status, pub c33_pw_status: ethtool_c33_pse_pw_d_status }
#[repr(C)]
pub struct pse_ext_state_info { pub c33_ext_state_info: ethtool_c33_pse_ext_state_info }
#[repr(C)]
pub struct pse_pw_limit_ranges { pub c33_pw_limit_ranges: *mut ethtool_c33_pse_pw_limit_range }

#[repr(C)]
pub struct ethtool_pse_control_status {
    pub pw_d_id: u32,
    pub podl_admin_state: ethtool_podl_pse_admin_state,
    pub podl_pw_status: ethtool_podl_pse_pw_d_status,
    pub c33_admin_state: ethtool_c33_pse_admin_state,
    pub c33_pw_status: ethtool_c33_pse_pw_d_status,
    pub c33_pw_class: u32,
    pub c33_actual_pw: u32,
    pub c33_ext_state_info: ethtool_c33_pse_ext_state_info,
    pub c33_avail_pw_limit: u32,
    pub c33_pw_limit_ranges: *mut ethtool_c33_pse_pw_limit_range,
    pub c33_pw_limit_nb_ranges: u32,
    pub prio_max: u32,
    pub prio: u32,
}

#[repr(C)]
pub struct pse_controller_ops {
    pub setup_pi_matrix: Option<unsafe extern "C" fn(*mut pse_controller_dev) -> i32>,
    pub pi_get_admin_state: Option<unsafe extern "C" fn(*mut pse_controller_dev, i32, *mut pse_admin_state) -> i32>,
    pub pi_get_pw_status: Option<unsafe extern "C" fn(*mut pse_controller_dev, i32, *mut pse_pw_status) -> i32>,
    pub pi_get_ext_state: Option<unsafe extern "C" fn(*mut pse_controller_dev, i32, *mut pse_ext_state_info) -> i32>,
    pub pi_get_pw_class: Option<unsafe extern "C" fn(*mut pse_controller_dev, i32) -> i32>,
    pub pi_get_actual_pw: Option<unsafe extern "C" fn(*mut pse_controller_dev, i32) -> i32>,
    pub pi_enable: Option<unsafe extern "C" fn(*mut pse_controller_dev, i32) -> i32>,
    pub pi_disable: Option<unsafe extern "C" fn(*mut pse_controller_dev, i32) -> i32>,
    pub pi_get_voltage: Option<unsafe extern "C" fn(*mut pse_controller_dev, i32) -> i32>,
    pub pi_get_pw_limit: Option<unsafe extern "C" fn(*mut pse_controller_dev, i32) -> i32>,
    pub pi_set_pw_limit: Option<unsafe extern "C" fn(*mut pse_controller_dev, i32, i32) -> i32>,
    pub pi_get_pw_limit_ranges: Option<unsafe extern "C" fn(*mut pse_controller_dev, i32, *mut pse_pw_limit_ranges) -> i32>,
    pub pi_get_prio: Option<unsafe extern "C" fn(*mut pse_controller_dev, i32) -> i32>,
    pub pi_set_prio: Option<unsafe extern "C" fn(*mut pse_controller_dev, i32, u32) -> i32>,
    pub pi_get_pw_req: Option<unsafe extern "C" fn(*mut pse_controller_dev, i32) -> i32>,
}

#[repr(C)]
pub enum pse_pi_pairset_pinout { ALTERNATIVE_A, ALTERNATIVE_B }
#[repr(C)]
pub struct pse_pi_pairset { pub pinout: pse_pi_pairset_pinout, pub np: *mut device_node }
#[repr(C)]
pub struct pse_pi {
    pub pairset: [pse_pi_pairset; 2], pub np: *mut device_node, pub rdev: *mut regulator_dev,
    pub admin_state_enabled: bool, pub pw_d: *mut pse_power_domain, pub prio: i32,
    pub isr_pd_detected: bool, pub pw_allocated_mW: i32,
}
#[repr(C)]
pub struct pse_ntf { pub id: i32, pub notifs: c_ulong }

#[repr(C)]
pub struct list_head;
#[repr(C)]
pub struct mutex;
#[repr(C)]
pub struct work_struct;
#[repr(C)]
pub struct spinlock_t;
// DECLARE_KFIFO_PTR is supplied by Linux's kfifo header; preserve it as an opaque field.
#[repr(C)]
pub struct pse_ntf_fifo_opaque { _private: [u8; 0] }

#[repr(C)]
pub struct pse_controller_dev {
    pub ops: *const pse_controller_ops, pub owner: *mut module, pub list: list_head,
    pub pse_control_head: list_head, pub dev: *mut device, pub of_pse_n_cells: i32,
    pub nr_lines: u32, pub lock: mutex, pub types: ethtool_pse_types, pub pi: *mut pse_pi,
    pub no_of_pse_pi: bool, pub irq: i32, pub pis_prio_max: u32,
    pub supp_budget_eval_strategies: u32, pub ntf_work: work_struct,
    pub ntf_fifo: pse_ntf_fifo_opaque, pub ntf_fifo_lock: spinlock_t,
}

pub const PSE_BUDGET_EVAL_STRAT_DISABLED: u32 = 1 << 0;
pub const PSE_BUDGET_EVAL_STRAT_STATIC: u32 = 1 << 1;
pub const PSE_BUDGET_EVAL_STRAT_DYNAMIC: u32 = 1 << 2;

// CONFIG_PSE_CONTROLLER conditional declarations and fallback inline definitions.
#[cfg(feature = "CONFIG_PSE_CONTROLLER")]
extern "C" {
    pub fn pse_controller_register(pcdev: *mut pse_controller_dev) -> i32;
    pub fn pse_controller_unregister(pcdev: *mut pse_controller_dev);
    pub fn devm_pse_controller_register(dev: *mut device, pcdev: *mut pse_controller_dev) -> i32;
    pub fn devm_pse_irq_helper(pcdev: *mut pse_controller_dev, irq: i32, irq_flags: i32, d: *const pse_irq_desc) -> i32;
    pub fn of_pse_control_get(node: *mut device_node, phydev: *mut phy_device) -> *mut pse_control;
    pub fn pse_control_put(psec: *mut pse_control);
    pub fn pse_ethtool_get_status(psec: *mut pse_control, extack: *mut netlink_ext_ack, status: *mut ethtool_pse_control_status) -> i32;
    pub fn pse_ethtool_set_config(psec: *mut pse_control, extack: *mut netlink_ext_ack, config: *const pse_control_config) -> i32;
    pub fn pse_ethtool_set_pw_limit(psec: *mut pse_control, extack: *mut netlink_ext_ack, pw_limit: u32) -> i32;
    pub fn pse_ethtool_set_prio(psec: *mut pse_control, extack: *mut netlink_ext_ack, prio: u32) -> i32;
    pub fn pse_has_podl(psec: *mut pse_control) -> bool;
    pub fn pse_has_c33(psec: *mut pse_control) -> bool;
}

#[cfg(not(feature = "CONFIG_PSE_CONTROLLER"))]
pub unsafe fn of_pse_control_get(_node: *mut device_node, _phydev: *mut phy_device) -> *mut pse_control { core::ptr::invalid_mut::<pse_control>(-2isize as usize) }
#[cfg(not(feature = "CONFIG_PSE_CONTROLLER"))]
pub unsafe fn pse_control_put(_psec: *mut pse_control) {}
#[cfg(not(feature = "CONFIG_PSE_CONTROLLER"))]
pub unsafe fn pse_ethtool_get_status(_psec: *mut pse_control, _extack: *mut netlink_ext_ack, _status: *mut ethtool_pse_control_status) -> i32 { -95 }
#[cfg(not(feature = "CONFIG_PSE_CONTROLLER"))]
pub unsafe fn pse_ethtool_set_config(_psec: *mut pse_control, _extack: *mut netlink_ext_ack, _config: *const pse_control_config) -> i32 { -95 }
#[cfg(not(feature = "CONFIG_PSE_CONTROLLER"))]
pub unsafe fn pse_ethtool_set_pw_limit(_psec: *mut pse_control, _extack: *mut netlink_ext_ack, _pw_limit: u32) -> i32 { -95 }
#[cfg(not(feature = "CONFIG_PSE_CONTROLLER"))]
pub unsafe fn pse_ethtool_set_prio(_psec: *mut pse_control, _extack: *mut netlink_ext_ack, _prio: u32) -> i32 { -95 }
#[cfg(not(feature = "CONFIG_PSE_CONTROLLER"))]
pub unsafe fn pse_has_podl(_psec: *mut pse_control) -> bool { false }
#[cfg(not(feature = "CONFIG_PSE_CONTROLLER"))]
pub unsafe fn pse_has_c33(_psec: *mut pse_control) -> bool { false }

use core::ffi::c_ulong;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
