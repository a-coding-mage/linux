/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/* QLogic qed NIC Driver
 * Copyright (c) 2015-2017  QLogic Corporation
 * Copyright (c) 2019-2020 Marvell International Ltd.
 */

/* Translated from qed_eth_if.h. C includes and header guards omitted. */

pub const QED_MIN_L2_CONS: u32 = 2 + NUM_PHYS_TCS_4PORT_K2 as u32;
pub const QED_MAX_L2_CONS: u32 = 64 * QED_MIN_L2_CONS;

#[repr(C)]
pub struct qed_queue_start_common_params {
    /* Should always be relative to entity sending this. */
    pub vport_id: u8,
    pub queue_id: u16,
    /* Relative, but relevant only for PFs */
    pub stats_id: u8,
    pub p_sb: *mut qed_sb_info,
    pub sb_idx: u8,
    pub tc: u8,
}

#[repr(C)]
pub struct qed_rxq_start_ret_params {
    pub p_prod: *mut core::ffi::c_void,
    pub p_handle: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct qed_txq_start_ret_params {
    pub p_doorbell: *mut core::ffi::c_void,
    pub p_handle: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum qed_filter_config_mode { QED_FILTER_CONFIG_MODE_DISABLE, QED_FILTER_CONFIG_MODE_5_TUPLE, QED_FILTER_CONFIG_MODE_L4_PORT, QED_FILTER_CONFIG_MODE_IP_DEST, QED_FILTER_CONFIG_MODE_IP_SRC }

#[repr(C)]
pub struct qed_ntuple_filter_params {
    pub addr: dma_addr_t,
    pub length: u16,
    pub qid: u16,
    pub b_is_vf: bool,
    pub vport_id: u8,
    pub vf_id: u8,
    pub b_is_add: bool,
    pub b_is_drop: bool,
}
pub const QED_RFS_NTUPLE_QID_RSS: u16 = u16::MAX;

#[repr(C)]
pub struct qed_dev_eth_info {
    pub common: qed_dev_info,
    pub num_queues: u8,
    pub num_tc: u8,
    pub port_mac: [u8; ETH_ALEN as usize],
    pub num_vlan_filters: u16,
    pub num_mac_filters: u16,
    pub is_legacy: bool,
    pub xdp_supported: bool,
}

#[repr(C)]
pub struct qed_update_vport_rss_params { pub rss_ind_table: [*mut core::ffi::c_void; 128], pub rss_key: [u32; 10], pub rss_caps: u8 }
#[repr(C)]
pub struct qed_update_vport_params {
    pub vport_id: u8, pub update_vport_active_flg: u8, pub vport_active_flg: u8,
    pub update_tx_switching_flg: u8, pub tx_switching_flg: u8,
    pub update_accept_any_vlan_flg: u8, pub accept_any_vlan: u8,
    pub update_rss_flg: u8, pub rss_params: qed_update_vport_rss_params,
}
#[repr(C)]
pub struct qed_start_vport_params { pub remove_inner_vlan: bool, pub handle_ptp_pkts: bool, pub gro_enable: bool, pub drop_ttl0: bool, pub vport_id: u8, pub mtu: u16, pub clear_stats: bool }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum qed_filter_rx_mode_type { QED_FILTER_RX_MODE_TYPE_REGULAR, QED_FILTER_RX_MODE_TYPE_MULTI_PROMISC, QED_FILTER_RX_MODE_TYPE_PROMISC }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum qed_filter_xcast_params_type { QED_FILTER_XCAST_TYPE_ADD, QED_FILTER_XCAST_TYPE_DEL, QED_FILTER_XCAST_TYPE_REPLACE }
#[repr(C)]
pub struct qed_filter_ucast_params { pub type_: qed_filter_xcast_params_type, pub vlan_valid: u8, pub vlan: u16, pub mac_valid: u8, pub mac: [u8; ETH_ALEN as usize] }
#[repr(C)]
pub struct qed_filter_mcast_params { pub type_: qed_filter_xcast_params_type, pub num: u8, pub mac: [[u8; ETH_ALEN as usize]; 64] }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum qed_filter_type { QED_FILTER_TYPE_UCAST, QED_FILTER_TYPE_MCAST, QED_FILTER_TYPE_RX_MODE, QED_MAX_FILTER_TYPES }
#[repr(C)]
pub struct qed_tunn_params { pub vxlan_port: u16, pub update_vxlan_port: u8, pub geneve_port: u16, pub update_geneve_port: u8 }

pub type ForceMac = unsafe extern "C" fn(*mut core::ffi::c_void, *mut u8, bool);
pub type PortsUpdate = unsafe extern "C" fn(*mut core::ffi::c_void, u16, u16);
#[repr(C)]
pub struct qed_eth_cb_ops { pub common: qed_common_cb_ops, pub force_mac: Option<ForceMac>, pub ports_update: Option<PortsUpdate> }
pub const QED_MAX_PHC_DRIFT_PPB: u32 = 291666666;
#[repr(C)]
#[derive(Copy, Clone)]
pub enum qed_ptp_filter_type { QED_PTP_FILTER_NONE, QED_PTP_FILTER_ALL, QED_PTP_FILTER_V1_L4_EVENT, QED_PTP_FILTER_V1_L4_GEN, QED_PTP_FILTER_V2_L4_EVENT, QED_PTP_FILTER_V2_L4_GEN, QED_PTP_FILTER_V2_L2_EVENT, QED_PTP_FILTER_V2_L2_GEN, QED_PTP_FILTER_V2_EVENT, QED_PTP_FILTER_V2_GEN }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum qed_ptp_hwtstamp_tx_type { QED_PTP_HWTSTAMP_TX_OFF, QED_PTP_HWTSTAMP_TX_ON }

/* CONFIG_DCB conditional declaration preserved; dependent DCB types are external. */
#[repr(C)]
pub struct qed_eth_dcbnl_ops {
    pub ieee_getpfc: Option<unsafe extern "C" fn(*mut qed_dev, *mut ieee_pfc) -> i32>, pub ieee_setpfc: Option<unsafe extern "C" fn(*mut qed_dev, *mut ieee_pfc) -> i32>, pub ieee_getets: Option<unsafe extern "C" fn(*mut qed_dev, *mut ieee_ets) -> i32>, pub ieee_setets: Option<unsafe extern "C" fn(*mut qed_dev, *mut ieee_ets) -> i32>, pub ieee_peer_getets: Option<unsafe extern "C" fn(*mut qed_dev, *mut ieee_ets) -> i32>, pub ieee_peer_getpfc: Option<unsafe extern "C" fn(*mut qed_dev, *mut ieee_pfc) -> i32>, pub ieee_getapp: Option<unsafe extern "C" fn(*mut qed_dev, *mut dcb_app) -> i32>, pub ieee_setapp: Option<unsafe extern "C" fn(*mut qed_dev, *mut dcb_app) -> i32>,
    pub getstate: Option<unsafe extern "C" fn(*mut qed_dev) -> u8>, pub setstate: Option<unsafe extern "C" fn(*mut qed_dev, u8) -> u8>,
    pub getpgtccfgtx: Option<unsafe extern "C" fn(*mut qed_dev, i32, *mut u8, *mut u8, *mut u8, *mut u8)>, pub getpgbwgcfgtx: Option<unsafe extern "C" fn(*mut qed_dev, i32, *mut u8)>, pub getpgtccfgrx: Option<unsafe extern "C" fn(*mut qed_dev, i32, *mut u8, *mut u8, *mut u8, *mut u8)>, pub getpgbwgcfgrx: Option<unsafe extern "C" fn(*mut qed_dev, i32, *mut u8)>, pub getpfccfg: Option<unsafe extern "C" fn(*mut qed_dev, i32, *mut u8)>, pub setpfccfg: Option<unsafe extern "C" fn(*mut qed_dev, i32, u8)>, pub getcap: Option<unsafe extern "C" fn(*mut qed_dev, i32, *mut u8) -> u8>, pub getnumtcs: Option<unsafe extern "C" fn(*mut qed_dev, i32, *mut u8) -> i32>, pub getpfcstate: Option<unsafe extern "C" fn(*mut qed_dev) -> u8>, pub getapp: Option<unsafe extern "C" fn(*mut qed_dev, u8, u16) -> i32>, pub getfeatcfg: Option<unsafe extern "C" fn(*mut qed_dev, i32, *mut u8) -> u8>,
    pub getdcbx: Option<unsafe extern "C" fn(*mut qed_dev) -> u8>, pub setpgtccfgtx: Option<unsafe extern "C" fn(*mut qed_dev, i32, u8, u8, u8, u8)>, pub setpgtccfgrx: Option<unsafe extern "C" fn(*mut qed_dev, i32, u8, u8, u8, u8)>, pub setpgbwgcfgtx: Option<unsafe extern "C" fn(*mut qed_dev, i32, u8)>, pub setpgbwgcfgrx: Option<unsafe extern "C" fn(*mut qed_dev, i32, u8)>, pub setall: Option<unsafe extern "C" fn(*mut qed_dev) -> u8>, pub setnumtcs: Option<unsafe extern "C" fn(*mut qed_dev, i32, u8) -> i32>, pub setpfcstate: Option<unsafe extern "C" fn(*mut qed_dev, u8)>, pub setapp: Option<unsafe extern "C" fn(*mut qed_dev, u8, u16, u8) -> i32>, pub setdcbx: Option<unsafe extern "C" fn(*mut qed_dev, u8) -> u8>, pub setfeatcfg: Option<unsafe extern "C" fn(*mut qed_dev, i32, u8) -> u8>,
    pub peer_getappinfo: Option<unsafe extern "C" fn(*mut qed_dev, *mut dcb_peer_app_info, *mut u16) -> i32>, pub peer_getapptable: Option<unsafe extern "C" fn(*mut qed_dev, *mut dcb_app) -> i32>, pub cee_peer_getpfc: Option<unsafe extern "C" fn(*mut qed_dev, *mut cee_pfc) -> i32>, pub cee_peer_getpg: Option<unsafe extern "C" fn(*mut qed_dev, *mut cee_pg) -> i32>,
}

#[repr(C)]
pub struct qed_eth_ptp_ops {
    pub cfg_filters: Option<unsafe extern "C" fn(*mut qed_dev, qed_ptp_filter_type, qed_ptp_hwtstamp_tx_type) -> i32>,
    pub read_rx_ts: Option<unsafe extern "C" fn(*mut qed_dev, *mut u64) -> i32>, pub read_tx_ts: Option<unsafe extern "C" fn(*mut qed_dev, *mut u64) -> i32>, pub read_cc: Option<unsafe extern "C" fn(*mut qed_dev, *mut u64) -> i32>, pub disable: Option<unsafe extern "C" fn(*mut qed_dev) -> i32>, pub adjfreq: Option<unsafe extern "C" fn(*mut qed_dev, i32) -> i32>, pub enable: Option<unsafe extern "C" fn(*mut qed_dev) -> i32>,
}

#[repr(C)]
pub struct qed_eth_ops {
    pub common: *const qed_common_ops,
    pub iov: *const qed_iov_hv_ops,
    pub dcb: *const qed_eth_dcbnl_ops,
    pub ptp: *const qed_eth_ptp_ops,
    pub fill_dev_info: Option<unsafe extern "C" fn(*mut qed_dev, *mut qed_dev_eth_info) -> i32>,
    pub register_ops: Option<unsafe extern "C" fn(*mut qed_dev, *mut qed_eth_cb_ops, *mut core::ffi::c_void)>,
    pub check_mac: Option<unsafe extern "C" fn(*mut qed_dev, *mut u8) -> bool>,
    pub vport_start: Option<unsafe extern "C" fn(*mut qed_dev, *mut qed_start_vport_params) -> i32>, pub vport_stop: Option<unsafe extern "C" fn(*mut qed_dev, u8) -> i32>, pub vport_update: Option<unsafe extern "C" fn(*mut qed_dev, *mut qed_update_vport_params) -> i32>,
    pub q_rx_start: Option<unsafe extern "C" fn(*mut qed_dev, u8, *mut qed_queue_start_common_params, u16, dma_addr_t, dma_addr_t, u16, *mut qed_rxq_start_ret_params) -> i32>, pub q_rx_stop: Option<unsafe extern "C" fn(*mut qed_dev, u8, *mut core::ffi::c_void) -> i32>,
    pub q_tx_start: Option<unsafe extern "C" fn(*mut qed_dev, u8, *mut qed_queue_start_common_params, dma_addr_t, u16, *mut qed_txq_start_ret_params) -> i32>, pub q_tx_stop: Option<unsafe extern "C" fn(*mut qed_dev, u8, *mut core::ffi::c_void) -> i32>,
    pub filter_config_rx_mode: Option<unsafe extern "C" fn(*mut qed_dev, qed_filter_rx_mode_type) -> i32>, pub filter_config_ucast: Option<unsafe extern "C" fn(*mut qed_dev, *mut qed_filter_ucast_params) -> i32>, pub filter_config_mcast: Option<unsafe extern "C" fn(*mut qed_dev, *mut qed_filter_mcast_params) -> i32>, pub fastpath_stop: Option<unsafe extern "C" fn(*mut qed_dev) -> i32>, pub eth_cqe_completion: Option<unsafe extern "C" fn(*mut qed_dev, u8, *mut eth_slow_path_rx_cqe) -> i32>,
    pub get_vport_stats: Option<unsafe extern "C" fn(*mut qed_dev, *mut qed_eth_stats)>, pub tunn_config: Option<unsafe extern "C" fn(*mut qed_dev, *mut qed_tunn_params) -> i32>, pub ntuple_filter_config: Option<unsafe extern "C" fn(*mut qed_dev, *mut core::ffi::c_void, *mut qed_ntuple_filter_params) -> i32>, pub configure_arfs_searcher: Option<unsafe extern "C" fn(*mut qed_dev, qed_filter_config_mode) -> i32>, pub get_coalesce: Option<unsafe extern "C" fn(*mut qed_dev, *mut u16, *mut core::ffi::c_void) -> i32>, pub req_bulletin_update_mac: Option<unsafe extern "C" fn(*mut qed_dev, *const u8) -> i32>,
}

extern "C" { pub fn qed_get_eth_ops() -> *const qed_eth_ops; pub fn qed_put_eth_ops(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
