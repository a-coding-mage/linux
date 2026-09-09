/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/* QLogic qed NIC Driver
 * Copyright (c) 2015-2017  QLogic Corporation
 * Copyright (c) 2019-2020 Marvell International Ltd.
 */

// Dependencies supplied by the surrounding kernel/qed translation.

#[repr(C)]
#[derive(Copy, Clone)]
pub enum qed_ll2_conn_type {
    QED_LL2_TYPE_FCOE,
    QED_LL2_TYPE_TCP_ULP,
    QED_LL2_TYPE_TEST,
    QED_LL2_TYPE_OOO,
    QED_LL2_TYPE_RESERVED2,
    QED_LL2_TYPE_ROCE,
    QED_LL2_TYPE_IWARP,
    QED_LL2_TYPE_RESERVED3,
    MAX_QED_LL2_CONN_TYPE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum qed_ll2_rx_conn_type {
    QED_LL2_RX_TYPE_LEGACY,
    QED_LL2_RX_TYPE_CTX,
    MAX_QED_LL2_RX_CONN_TYPE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum qed_ll2_roce_flavor_type {
    QED_LL2_ROCE,
    QED_LL2_RROCE,
    MAX_QED_LL2_ROCE_FLAVOR_TYPE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum qed_ll2_tx_dest {
    QED_LL2_TX_DEST_NW,
    QED_LL2_TX_DEST_LB,
    QED_LL2_TX_DEST_DROP,
    QED_LL2_TX_DEST_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum qed_ll2_error_handle {
    QED_LL2_DROP_PACKET,
    QED_LL2_DO_NOTHING,
    QED_LL2_ASSERT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_ll2_stats {
    pub gsi_invalid_hdr: u64,
    pub gsi_invalid_pkt_length: u64,
    pub gsi_unsupported_pkt_typ: u64,
    pub gsi_crcchksm_error: u64,
    pub packet_too_big_discard: u64,
    pub no_buff_discard: u64,
    pub rcv_ucast_bytes: u64,
    pub rcv_mcast_bytes: u64,
    pub rcv_bcast_bytes: u64,
    pub rcv_ucast_pkts: u64,
    pub rcv_mcast_pkts: u64,
    pub rcv_bcast_pkts: u64,
    pub sent_ucast_bytes: u64,
    pub sent_mcast_bytes: u64,
    pub sent_bcast_bytes: u64,
    pub sent_ucast_pkts: u64,
    pub sent_mcast_pkts: u64,
    pub sent_bcast_pkts: u64,
}

#[repr(C)]
pub union qed_ll2_comp_rx_data_length {
    pub packet_length: u16,
    pub data_length: u16,
}

#[repr(C)]
pub union qed_ll2_comp_rx_data_u {
    pub placement_offset: u8,
    pub data_length_error: u8,
}

#[repr(C)]
pub struct qed_ll2_comp_rx_data {
    pub cookie: *mut core::ffi::c_void,
    pub rx_buf_addr: dma_addr_t,
    pub parse_flags: u16,
    pub err_flags: u16,
    pub vlan: u16,
    pub b_last_packet: bool,
    pub connection_handle: u8,
    pub length: qed_ll2_comp_rx_data_length,
    pub opaque_data_0: u32,
    pub opaque_data_1: u32,
    pub src_qp: u32,
    pub qp_id: u16,
    pub u: qed_ll2_comp_rx_data_u,
}

pub type qed_ll2_complete_rx_packet_cb = unsafe extern "C" fn(*mut core::ffi::c_void, *mut qed_ll2_comp_rx_data);
pub type qed_ll2_release_rx_packet_cb = unsafe extern "C" fn(*mut core::ffi::c_void, u8, *mut core::ffi::c_void, dma_addr_t, bool);
pub type qed_ll2_complete_tx_packet_cb = unsafe extern "C" fn(*mut core::ffi::c_void, u8, *mut core::ffi::c_void, dma_addr_t, bool, bool);
pub type qed_ll2_release_tx_packet_cb = unsafe extern "C" fn(*mut core::ffi::c_void, u8, *mut core::ffi::c_void, dma_addr_t, bool, bool);
pub type qed_ll2_slowpath_cb = unsafe extern "C" fn(*mut core::ffi::c_void, u8, u32, u32);

#[repr(C)]
pub struct qed_ll2_cbs {
    pub rx_comp_cb: Option<qed_ll2_complete_rx_packet_cb>,
    pub rx_release_cb: Option<qed_ll2_release_rx_packet_cb>,
    pub tx_comp_cb: Option<qed_ll2_complete_tx_packet_cb>,
    pub tx_release_cb: Option<qed_ll2_release_tx_packet_cb>,
    pub slowpath_cb: Option<qed_ll2_slowpath_cb>,
    pub cookie: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct qed_ll2_acquire_data_inputs {
    pub rx_conn_type: qed_ll2_rx_conn_type,
    pub conn_type: qed_ll2_conn_type,
    pub mtu: u16,
    pub rx_num_desc: u16,
    pub rx_num_ooo_buffers: u16,
    pub rx_drop_ttl0_flg: u8,
    pub rx_vlan_removal_en: u8,
    pub tx_num_desc: u16,
    pub tx_max_bds_per_packet: u8,
    pub tx_tc: u8,
    pub tx_dest: qed_ll2_tx_dest,
    pub ai_err_packet_too_big: qed_ll2_error_handle,
    pub ai_err_no_buf: qed_ll2_error_handle,
    pub secondary_queue: bool,
    pub gsi_enable: u8,
}

#[repr(C)]
pub struct qed_ll2_acquire_data {
    pub input: qed_ll2_acquire_data_inputs,
    pub cbs: *const qed_ll2_cbs,
    pub p_connection_handle: *mut u8,
}

#[repr(C)]
pub struct qed_ll2_tx_pkt_info {
    pub cookie: *mut core::ffi::c_void,
    pub first_frag: dma_addr_t,
    pub tx_dest: qed_ll2_tx_dest,
    pub qed_roce_flavor: qed_ll2_roce_flavor_type,
    pub vlan: u16,
    pub l4_hdr_offset_w: u16,
    pub first_frag_len: u16,
    pub num_of_bds: u8,
    pub bd_flags: u8,
    pub enable_ip_cksum: bool,
    pub enable_l4_cksum: bool,
    pub calc_ip_len: bool,
    pub remove_stag: bool,
}

pub const QED_LL2_UNUSED_HANDLE: u8 = 0xff;

#[repr(C)]
pub struct qed_ll2_cb_ops {
    pub rx_cb: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut sk_buff, u32, u32) -> i32>,
    pub tx_cb: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut sk_buff, bool) -> i32>,
}

#[repr(C)]
pub struct qed_ll2_params {
    pub mtu: u16,
    pub drop_ttl0_packets: bool,
    pub rx_vlan_stripping: bool,
    pub tx_tc: u8,
    pub frags_mapped: bool,
    pub ll2_mac_address: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum qed_ll2_xmit_flags {
    QED_LL2_XMIT_FLAGS_FIP_DISCOVERY,
}

#[repr(C)]
pub struct qed_ll2_ops {
    pub start: Option<unsafe extern "C" fn(*mut qed_dev, *mut qed_ll2_params) -> i32>,
    pub stop: Option<unsafe extern "C" fn(*mut qed_dev) -> i32>,
    pub start_xmit: Option<unsafe extern "C" fn(*mut qed_dev, *mut sk_buff, core::ffi::c_ulong) -> i32>,
    pub register_cb_ops: Option<unsafe extern "C" fn(*mut qed_dev, *const qed_ll2_cb_ops, *mut core::ffi::c_void)>,
    pub get_stats: Option<unsafe extern "C" fn(*mut qed_dev, *mut qed_ll2_stats) -> i32>,
}

pub const qed_ll2_ops_pass: qed_ll2_ops = qed_ll2_ops {
    start: None,
    stop: None,
    start_xmit: None,
    register_cb_ops: None,
    get_stats: None,
};

#[cfg(CONFIG_QED_LL2)]
extern "C" {
    pub fn qed_ll2_alloc_if(cdev: *mut qed_dev) -> i32;
    pub fn qed_ll2_dealloc_if(cdev: *mut qed_dev);
}

#[cfg(not(CONFIG_QED_LL2))]
pub unsafe fn qed_ll2_alloc_if(_cdev: *mut qed_dev) -> i32 {
    0
}

#[cfg(not(CONFIG_QED_LL2))]
pub unsafe fn qed_ll2_dealloc_if(_cdev: *mut qed_dev) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
