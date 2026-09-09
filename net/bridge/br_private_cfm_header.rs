/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependencies supplied by br_private.h and <uapi/linux/cfm_bridge.h> are
// intentionally referenced but not defined here.

#[repr(C)]
pub struct br_cfm_mep_create {
    pub domain: br_cfm_domain, // Domain for this MEP
    pub direction: br_cfm_mep_direction, // Up or Down MEP direction
    pub ifindex: u32, // Residence port
}

extern "C" {
    pub fn br_cfm_mep_create(
        br: *mut net_bridge,
        instance: u32,
        create: *mut br_cfm_mep_create,
        extack: *mut netlink_ext_ack,
    ) -> i32;

    pub fn br_cfm_mep_delete(
        br: *mut net_bridge,
        instance: u32,
        extack: *mut netlink_ext_ack,
    ) -> i32;
}

#[repr(C)]
pub struct br_cfm_mep_config {
    pub mdlevel: u32,
    pub mepid: u32, // MEPID for this MEP
    pub unicast_mac: mac_addr, // The MEP unicast MAC
}

extern "C" {
    pub fn br_cfm_mep_config_set(
        br: *mut net_bridge,
        instance: u32,
        config: *const br_cfm_mep_config,
        extack: *mut netlink_ext_ack,
    ) -> i32;
}

#[repr(C)]
pub struct br_cfm_maid {
    pub data: [u8; CFM_MAID_LENGTH],
}

#[repr(C)]
pub struct br_cfm_cc_config {
    // Expected received CCM PDU MAID.
    pub exp_maid: br_cfm_maid,
    // Expected received CCM PDU interval.
    // Transmitting CCM PDU interval when CCM tx is enabled.
    pub exp_interval: br_cfm_ccm_interval,
    pub enable: bool, // Enable/disable CCM PDU handling
}

extern "C" {
    pub fn br_cfm_cc_config_set(
        br: *mut net_bridge,
        instance: u32,
        config: *const br_cfm_cc_config,
        extack: *mut netlink_ext_ack,
    ) -> i32;

    pub fn br_cfm_cc_peer_mep_add(
        br: *mut net_bridge,
        instance: u32,
        peer_mep_id: u32,
        extack: *mut netlink_ext_ack,
    ) -> i32;

    pub fn br_cfm_cc_peer_mep_remove(
        br: *mut net_bridge,
        instance: u32,
        peer_mep_id: u32,
        extack: *mut netlink_ext_ack,
    ) -> i32;

    // Transmitted CCM Remote Defect Indication status set.
    // This RDI is inserted in transmitted CCM PDUs if CCM transmission is enabled.
    // See br_cfm_cc_ccm_tx() with interval != BR_CFM_CCM_INTERVAL_NONE
    pub fn br_cfm_cc_rdi_set(
        br: *mut net_bridge,
        instance: u32,
        rdi: bool,
        extack: *mut netlink_ext_ack,
    ) -> i32;
}

#[repr(C)]
pub struct br_cfm_cc_ccm_tx_info {
    pub dmac: mac_addr,
    // The CCM will be transmitted for this period in seconds.
    // Call br_cfm_cc_ccm_tx before timeout to keep transmission alive.
    // When period is zero any ongoing transmission will be stopped.
    pub period: u32,
    pub seq_no_update: bool, // Update Tx CCM sequence number
    pub if_tlv: bool, // Insert Interface Status TLV
    pub if_tlv_value: u8, // Interface Status TLV value
    pub port_tlv: bool, // Insert Port Status TLV
    pub port_tlv_value: u8, // Port Status TLV value
    // Sender ID TLV ??
    // Organization-Specific TLV ??
}

extern "C" {
    pub fn br_cfm_cc_ccm_tx(
        br: *mut net_bridge,
        instance: u32,
        tx_info: *const br_cfm_cc_ccm_tx_info,
        extack: *mut netlink_ext_ack,
    ) -> i32;
}

#[repr(C)]
pub struct br_cfm_mep_status {
    // Indications that an OAM PDU has been seen.
    pub opcode_unexp_seen: bool, // RX of OAM PDU with unexpected opcode
    pub version_unexp_seen: bool, // RX of OAM PDU with unexpected version
    pub rx_level_low_seen: bool, // Rx of OAM PDU with level low
}

#[repr(C)]
pub struct br_cfm_cc_peer_status {
    // This CCM related status is based on the latest received CCM PDU.
    pub port_tlv_value: u8, // Port Status TLV value
    pub if_tlv_value: u8, // Interface Status TLV value
    // CCM has not been received for 3.25 intervals
    pub ccm_defect: u8, // bitfield: 1 bit
    // (RDI == 1) for last received CCM PDU
    pub rdi: u8, // bitfield: 1 bit
    // Indications that a CCM PDU has been seen.
    pub seen: u8, // bitfield: 1 bit; CCM PDU received
    pub tlv_seen: u8, // bitfield: 1 bit; CCM PDU with TLV received
    // CCM PDU with unexpected sequence number received
    pub seq_unexp_seen: u8, // bitfield: 1 bit
}

#[repr(C)]
pub struct br_cfm_mep {
    // list header of MEP instances
    pub head: hlist_node,
    pub instance: u32,
    pub create: br_cfm_mep_create,
    pub config: br_cfm_mep_config,
    pub cc_config: br_cfm_cc_config,
    pub cc_ccm_tx_info: br_cfm_cc_ccm_tx_info,
    // List of multiple peer MEPs
    pub peer_mep_list: hlist_head,
    pub b_port: *mut net_bridge_port,
    pub ccm_tx_end: c_ulong,
    pub ccm_tx_dwork: delayed_work,
    pub ccm_tx_snumber: u32,
    pub ccm_rx_snumber: u32,
    pub status: br_cfm_mep_status,
    pub rdi: bool,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct br_cfm_peer_mep {
    pub head: hlist_node,
    pub mep: *mut br_cfm_mep,
    pub ccm_rx_dwork: delayed_work,
    pub mepid: u32,
    pub cc_status: br_cfm_cc_peer_status,
    pub ccm_rx_count_miss: u32,
    pub rcu: rcu_head,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
