/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced but not defined here.

pub const MRP_OPT_PADDING: u32 = 0x2;

#[repr(C)]
pub struct br_mrp {
    /* list of mrp instances */
    pub list: hlist_node,

    pub p_port: *mut net_bridge_port,
    pub s_port: *mut net_bridge_port,
    pub i_port: *mut net_bridge_port,

    pub ring_id: u32,
    pub in_id: u16,
    pub prio: u16,

    pub ring_role: br_mrp_ring_role_type,
    pub ring_role_offloaded: u8,
    pub ring_state: br_mrp_ring_state_type,
    pub ring_transitions: u32,

    pub in_role: br_mrp_in_role_type,
    pub in_role_offloaded: u8,
    pub in_state: br_mrp_in_state_type,
    pub in_transitions: u32,

    pub test_work: delayed_work,
    pub test_interval: u32,
    pub test_end: usize,
    pub test_count_miss: u32,
    pub test_max_miss: u32,
    pub test_monitor: bool,

    pub in_test_work: delayed_work,
    pub in_test_interval: u32,
    pub in_test_end: usize,
    pub in_test_count_miss: u32,
    pub in_test_max_miss: u32,

    pub seq_id: u32,

    pub rcu: rcu_head,
}

/* This type is returned by br_mrp_switchdev functions that allow to have a SW
 * backup in case the HW can't implement completely the protocol.
 * BR_MRP_NONE - means the HW can't run at all the protocol, so the SW stops
 *               configuring the node anymore.
 * BR_MRP_SW - the HW can help the SW to run the protocol, by redirecting MRP
 *             frames to CPU.
 * BR_MRP_HW - the HW can implement completely the protocol.
 */
#[repr(C)]
pub enum br_mrp_hw_support {
    BR_MRP_NONE,
    BR_MRP_SW,
    BR_MRP_HW,
}

extern "C" {
    pub fn br_mrp_add(br: *mut net_bridge, instance: *mut br_mrp_instance) -> i32;
    pub fn br_mrp_del(br: *mut net_bridge, instance: *mut br_mrp_instance) -> i32;
    pub fn br_mrp_set_port_state(p: *mut net_bridge_port, state: br_mrp_port_state_type) -> i32;
    pub fn br_mrp_set_port_role(p: *mut net_bridge_port, role: br_mrp_port_role_type) -> i32;
    pub fn br_mrp_set_ring_state(br: *mut net_bridge, state: *mut br_mrp_ring_state) -> i32;
    pub fn br_mrp_set_ring_role(br: *mut net_bridge, role: *mut br_mrp_ring_role) -> i32;
    pub fn br_mrp_start_test(br: *mut net_bridge, test: *mut br_mrp_start_test) -> i32;
    pub fn br_mrp_set_in_state(br: *mut net_bridge, state: *mut br_mrp_in_state) -> i32;
    pub fn br_mrp_set_in_role(br: *mut net_bridge, role: *mut br_mrp_in_role) -> i32;
    pub fn br_mrp_start_in_test(br: *mut net_bridge, test: *mut br_mrp_start_in_test) -> i32;

    pub fn br_mrp_switchdev_add(br: *mut net_bridge, mrp: *mut br_mrp) -> i32;
    pub fn br_mrp_switchdev_del(br: *mut net_bridge, mrp: *mut br_mrp) -> i32;
    pub fn br_mrp_switchdev_set_ring_role(br: *mut net_bridge, mrp: *mut br_mrp, role: br_mrp_ring_role_type) -> br_mrp_hw_support;
    pub fn br_mrp_switchdev_set_ring_state(br: *mut net_bridge, mrp: *mut br_mrp, state: br_mrp_ring_state_type) -> i32;
    pub fn br_mrp_switchdev_send_ring_test(br: *mut net_bridge, mrp: *mut br_mrp, interval: u32, max_miss: u8, period: u32, monitor: bool) -> br_mrp_hw_support;
    pub fn br_mrp_port_switchdev_set_state(p: *mut net_bridge_port, state: u32) -> i32;
    pub fn br_mrp_port_switchdev_set_role(p: *mut net_bridge_port, role: br_mrp_port_role_type) -> i32;
    pub fn br_mrp_switchdev_set_in_role(br: *mut net_bridge, mrp: *mut br_mrp, in_id: u16, ring_id: u32, role: br_mrp_in_role_type) -> br_mrp_hw_support;
    pub fn br_mrp_switchdev_set_in_state(br: *mut net_bridge, mrp: *mut br_mrp, state: br_mrp_in_state_type) -> i32;
    pub fn br_mrp_switchdev_send_in_test(br: *mut net_bridge, mrp: *mut br_mrp, interval: u32, max_miss: u8, period: u32) -> br_mrp_hw_support;

    pub fn br_mrp_ring_port_open(dev: *mut net_device, loc: u8) -> i32;
    pub fn br_mrp_in_port_open(dev: *mut net_device, loc: u8) -> i32;
}

/* MRP protocol data units */
#[repr(C)]
pub struct br_mrp_tlv_hdr { pub type_: u8, pub length: u8 }

#[repr(C)]
pub struct br_mrp_common_hdr { pub seq_id: u16, pub domain: [u8; MRP_DOMAIN_UUID_LENGTH] }

#[repr(C, packed)]
pub struct br_mrp_ring_test_hdr {
    pub prio: u16, pub sa: [u8; ETH_ALEN], pub port_role: u16,
    pub state: u16, pub transitions: u16, pub timestamp: u32,
}

#[repr(C, packed)]
pub struct br_mrp_in_test_hdr {
    pub id: u16, pub sa: [u8; ETH_ALEN], pub port_role: u16,
    pub state: u16, pub transitions: u16, pub timestamp: u32,
}

#[repr(C)]
pub struct br_mrp_oui_hdr { pub oui: [u8; MRP_OUI_LENGTH] }

#[repr(C)]
pub struct br_mrp_sub_option1_hdr { pub type_: u8, pub data: [u8; MRP_MANUFACTURE_DATA_LENGTH] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
