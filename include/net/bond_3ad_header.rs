/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright(c) 1999 - 2004 Intel Corporation. All rights reserved.
 */

// C dependencies: asm/byteorder.h, linux/skbuff.h, linux/netdevice.h,
// linux/if_ether.h

/* General definitions */
pub const PKT_TYPE_LACPDU: u16 = cpu_to_be16(ETH_P_SLOW);
pub const AD_TIMER_INTERVAL: u32 = 100; /* msec */

pub const AD_LACP_SLOW: u32 = 0;
pub const AD_LACP_FAST: u32 = 1;

#[repr(C, packed)]
pub struct mac_addr {
    pub mac_addr_value: [u8; ETH_ALEN],
}
pub type mac_addr_t = mac_addr;

pub const BOND_AD_STABLE: u32 = 0;
pub const BOND_AD_BANDWIDTH: u32 = 1;
pub const BOND_AD_COUNT: u32 = 2;
pub const BOND_AD_PRIO: u32 = 3;

/* rx machine states (43.4.11 in the 802.3ad standard) */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rx_states_t {
    AD_RX_DUMMY,
    AD_RX_INITIALIZE,
    AD_RX_PORT_DISABLED,
    AD_RX_LACP_DISABLED,
    AD_RX_EXPIRED,
    AD_RX_DEFAULTED,
    AD_RX_CURRENT,
}

/* periodic machine states (43.4.12 in the 802.3ad standard) */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum periodic_states_t {
    AD_PERIODIC_DUMMY,
    AD_NO_PERIODIC,
    AD_FAST_PERIODIC,
    AD_SLOW_PERIODIC,
    AD_PERIODIC_TX,
}

/* mux machine states (43.4.13 in the 802.3ad standard) */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mux_states_t {
    AD_MUX_DUMMY,
    AD_MUX_DETACHED,
    AD_MUX_WAITING,
    AD_MUX_ATTACHED,
    AD_MUX_COLLECTING,
    AD_MUX_DISTRIBUTING,
    AD_MUX_COLLECTING_DISTRIBUTING,
}

/* tx machine states (43.4.15 in the 802.3ad standard) */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tx_states_t {
    AD_TX_DUMMY,
    AD_TRANSMIT,
}

/* churn machine states (43.4.17 in the 802.3ad standard) */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum churn_state_t {
    AD_CHURN_MONITOR,
    AD_CHURN,
    AD_NO_CHURN,
}

/* rx indication types */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pdu_type_t {
    AD_TYPE_LACPDU = 1,
    AD_TYPE_MARKER,
}

/* rx marker indication types */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bond_marker_subtype_t {
    AD_MARKER_INFORMATION_SUBTYPE = 1,
    AD_MARKER_RESPONSE_SUBTYPE,
}

/* timers types (43.4.9 in the 802.3ad standard) */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ad_timers_t {
    AD_CURRENT_WHILE_TIMER,
    AD_ACTOR_CHURN_TIMER,
    AD_PERIODIC_TIMER,
    AD_PARTNER_CHURN_TIMER,
    AD_WAIT_WHILE_TIMER,
}

/* LACP data unit structure (43.4.2.2 in the 802.3ad standard) */
#[repr(C, packed)]
pub struct lacpdu {
    pub subtype: u8,
    pub version_number: u8,
    pub tlv_type_actor_info: u8,
    pub actor_information_length: u8,
    pub actor_system_priority: __be16,
    pub actor_system: mac_addr,
    pub actor_key: __be16,
    pub actor_port_priority: __be16,
    pub actor_port: __be16,
    pub actor_state: u8,
    pub reserved_3_1: [u8; 3],
    pub tlv_type_partner_info: u8,
    pub partner_information_length: u8,
    pub partner_system_priority: __be16,
    pub partner_system: mac_addr,
    pub partner_key: __be16,
    pub partner_port_priority: __be16,
    pub partner_port: __be16,
    pub partner_state: u8,
    pub reserved_3_2: [u8; 3],
    pub tlv_type_collector_info: u8,
    pub collector_information_length: u8,
    pub collector_max_delay: __be16,
    pub reserved_12: [u8; 12],
    pub tlv_type_terminator: u8,
    pub terminator_length: u8,
    pub reserved_50: [u8; 50],
}
pub type lacpdu_t = lacpdu;

#[repr(C, packed)]
pub struct lacpdu_header {
    pub hdr: ethhdr,
    pub lacpdu: lacpdu,
}
pub type lacpdu_header_t = lacpdu_header;

/* Marker Protocol Data Unit (PDU) structure (43.5.3.2 in the 802.3ad standard) */
#[repr(C, packed)]
pub struct bond_marker {
    pub subtype: u8,
    pub version_number: u8,
    pub tlv_type: u8,
    pub marker_length: u8,
    pub requester_port: u16,
    pub requester_system: mac_addr,
    pub requester_transaction_id: u32,
    pub pad: u16,
    pub tlv_type_terminator: u8,
    pub terminator_length: u8,
    pub reserved_90: [u8; 90],
}
pub type bond_marker_t = bond_marker;

#[repr(C, packed)]
pub struct bond_marker_header {
    pub hdr: ethhdr,
    pub marker: bond_marker,
}
pub type bond_marker_header_t = bond_marker_header;

#[repr(C)]
pub struct slave;
#[repr(C)]
pub struct bonding;
#[repr(C)]
pub struct ad_info;
#[repr(C)]
pub struct port;

#[repr(C)]
pub struct bond_3ad_stats {
    pub lacpdu_rx: atomic64_t,
    pub lacpdu_tx: atomic64_t,
    pub lacpdu_unknown_rx: atomic64_t,
    pub lacpdu_illegal_rx: atomic64_t,
    pub marker_rx: atomic64_t,
    pub marker_tx: atomic64_t,
    pub marker_resp_rx: atomic64_t,
    pub marker_resp_tx: atomic64_t,
    pub marker_unknown_rx: atomic64_t,
}

#[repr(C)]
pub struct aggregator {
    pub aggregator_mac_address: mac_addr,
    pub aggregator_identifier: u16,
    pub is_individual: bool,
    pub actor_admin_aggregator_key: u16,
    pub actor_oper_aggregator_key: u16,
    pub partner_system: mac_addr,
    pub partner_system_priority: u16,
    pub partner_oper_aggregator_key: u16,
    pub receive_state: u16,
    pub transmit_state: u16,
    pub lag_ports: *mut port,
    pub slave: *mut slave,
    pub is_active: u16,
    pub num_of_ports: u16,
}
pub type aggregator_t = aggregator;

#[repr(C)]
pub struct port_params {
    pub system: mac_addr,
    pub system_priority: u16,
    pub key: u16,
    pub port_number: u16,
    pub port_priority: u16,
    pub port_state: u16,
}

#[repr(C)]
pub struct port {
    pub actor_port_number: u16,
    pub actor_port_priority: u16,
    pub actor_system: mac_addr,
    pub actor_system_priority: u16,
    pub actor_port_aggregator_identifier: u16,
    pub ntt: bool,
    pub actor_admin_port_key: u16,
    pub actor_oper_port_key: u16,
    pub actor_admin_port_state: u8,
    pub actor_oper_port_state: u8,
    pub partner_admin: port_params,
    pub partner_oper: port_params,
    pub is_enabled: bool,
    pub sm_vars: u16,
    pub sm_rx_state: rx_states_t,
    pub sm_rx_timer_counter: u16,
    pub sm_periodic_state: periodic_states_t,
    pub sm_periodic_timer_counter: u16,
    pub sm_mux_state: mux_states_t,
    pub sm_mux_timer_counter: u16,
    pub sm_tx_state: tx_states_t,
    pub sm_tx_timer_counter: u16,
    pub sm_churn_actor_timer_counter: u16,
    pub sm_churn_partner_timer_counter: u16,
    pub churn_actor_count: u32,
    pub churn_partner_count: u32,
    pub sm_churn_actor_state: churn_state_t,
    pub sm_churn_partner_state: churn_state_t,
    pub slave: *mut slave,
    pub aggregator: *mut aggregator,
    pub next_port_in_aggregator: *mut port,
    pub transaction_id: u32,
    pub lacpdu: lacpdu,
}
pub type port_t = port;

#[repr(C)]
pub struct ad_system {
    pub sys_priority: u16,
    pub sys_mac_addr: mac_addr,
}

#[repr(C)]
pub struct ad_bond_info {
    pub system: ad_system,
    pub stats: bond_3ad_stats,
    pub agg_select_timer: atomic_t,
    pub aggregator_identifier: u16,
}

#[repr(C)]
pub struct ad_slave_info {
    pub aggregator: aggregator,
    pub port: port,
    pub stats: bond_3ad_stats,
    pub id: u16,
    pub port_priority: u16,
}

pub unsafe fn bond_3ad_churn_desc(state: churn_state_t) -> *const core::ffi::c_char {
    static CHURN_DESCRIPTION: [&[u8]; 4] = [b"monitoring\0", b"churned\0", b"none\0", b"unknown\0"];
    let mut index = state as usize;
    if index >= CHURN_DESCRIPTION.len() {
        index = CHURN_DESCRIPTION.len() - 1;
    }
    CHURN_DESCRIPTION[index].as_ptr() as *const core::ffi::c_char
}

extern "C" {
    pub fn bond_3ad_initialize(bond: *mut bonding);
    pub fn bond_3ad_bind_slave(slave: *mut slave);
    pub fn bond_3ad_unbind_slave(slave: *mut slave);
    pub fn bond_3ad_state_machine_handler(work: *mut work_struct);
    pub fn bond_3ad_initiate_agg_selection(bond: *mut bonding, timeout: i32);
    pub fn bond_3ad_adapter_speed_duplex_changed(slave: *mut slave);
    pub fn bond_3ad_handle_link_change(slave: *mut slave, link: core::ffi::c_char);
    pub fn bond_3ad_get_active_agg_info(bond: *const bonding, ad_info: *mut ad_info) -> i32;
    pub fn __bond_3ad_get_active_agg_info(bond: *const bonding, ad_info: *mut ad_info) -> i32;
    pub fn bond_3ad_lacpdu_recv(skb: *const sk_buff, bond: *mut bonding, slave: *mut slave) -> i32;
    pub fn bond_3ad_set_carrier(bond: *mut bonding) -> i32;
    pub fn bond_3ad_update_lacp_rate(bond: *mut bonding);
    pub fn bond_3ad_update_lacp_active(bond: *mut bonding);
    pub fn bond_3ad_update_ad_actor_settings(bond: *mut bonding);
    pub fn bond_3ad_stats_fill(skb: *mut sk_buff, stats: *mut bond_3ad_stats) -> i32;
    pub fn bond_3ad_stats_size() -> usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
