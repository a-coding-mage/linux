/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/* QLogic qed NIC Driver
 * Copyright (c) 2015-2017  QLogic Corporation
 * Copyright (c) 2019-2020 Marvell International Ltd.
 */

// Translated from qed_iscsi_if.h. Linux and qed types are supplied by dependencies.

pub type IscsiEventCbT = unsafe extern "C" fn(context: *mut core::ffi::c_void,
                                               fw_event_code: u8,
                                               fw_handle: *mut core::ffi::c_void) -> i32;

#[repr(C)]
pub struct QedIscsiStats {
    pub iscsi_rx_bytes_cnt: u64,
    pub iscsi_rx_packet_cnt: u64,
    pub iscsi_rx_new_ooo_isle_events_cnt: u64,
    pub iscsi_cmdq_threshold_cnt: u32,
    pub iscsi_rq_threshold_cnt: u32,
    pub iscsi_immq_threshold_cnt: u32,
    pub iscsi_rx_dropped_pdus_task_not_valid: u64,
    pub iscsi_rx_data_pdu_cnt: u64,
    pub iscsi_rx_r2t_pdu_cnt: u64,
    pub iscsi_rx_total_pdu_cnt: u64,
    pub iscsi_tx_go_to_slow_start_event_cnt: u64,
    pub iscsi_tx_fast_retransmit_event_cnt: u64,
    pub iscsi_tx_data_pdu_cnt: u64,
    pub iscsi_tx_r2t_pdu_cnt: u64,
    pub iscsi_tx_total_pdu_cnt: u64,
    pub iscsi_tx_bytes_cnt: u64,
    pub iscsi_tx_packet_cnt: u64,
}

#[repr(C)]
pub struct QedDevIscsiInfo {
    pub common: QedDevInfo,
    pub primary_dbq_rq_addr: *mut core::ffi::c_void,
    pub secondary_bdq_rq_addr: *mut core::ffi::c_void,
    pub num_cqs: u8,
}

#[repr(C)]
pub struct QedIscsiIdParams {
    pub mac: [u8; ETH_ALEN],
    pub ip: [u32; 4],
    pub port: u16,
}

#[repr(C)]
pub struct QedIscsiParamsOffload {
    pub layer_code: u8,
    pub sq_pbl_addr: dma_addr_t,
    pub initial_ack: u32,
    pub src: QedIscsiIdParams,
    pub dst: QedIscsiIdParams,
    pub vlan_id: u16,
    pub tcp_flags: u8,
    pub ip_version: u8,
    pub default_cq: u8,
    pub ka_max_probe_cnt: u8,
    pub dup_ack_theshold: u8,
    pub rcv_next: u32,
    pub snd_una: u32,
    pub snd_next: u32,
    pub snd_max: u32,
    pub snd_wnd: u32,
    pub rcv_wnd: u32,
    pub snd_wl1: u32,
    pub cwnd: u32,
    pub ss_thresh: u32,
    pub srtt: u16,
    pub rtt_var: u16,
    pub ts_recent: u32,
    pub ts_recent_age: u32,
    pub total_rt: u32,
    pub ka_timeout_delta: u32,
    pub rt_timeout_delta: u32,
    pub dup_ack_cnt: u8,
    pub snd_wnd_probe_cnt: u8,
    pub ka_probe_cnt: u8,
    pub rt_cnt: u8,
    pub flow_label: u32,
    pub ka_timeout: u32,
    pub ka_interval: u32,
    pub max_rt_time: u32,
    pub initial_rcv_wnd: u32,
    pub ttl: u8,
    pub tos_or_tc: u8,
    pub remote_port: u16,
    pub local_port: u16,
    pub mss: u16,
    pub snd_wnd_scale: u8,
    pub rcv_wnd_scale: u8,
    pub da_timeout_value: u16,
    pub ack_frequency: u8,
}

#[repr(C)]
pub struct QedIscsiParamsUpdate {
    pub update_flag: u8,
    pub max_seq_size: u32,
    pub max_recv_pdu_length: u32,
    pub max_send_pdu_length: u32,
    pub first_seq_length: u32,
    pub exp_stat_sn: u32,
}

pub const QED_ISCSI_CONN_HD_EN: u8 = 1 << 0;
pub const QED_ISCSI_CONN_DD_EN: u8 = 1 << 1;
pub const QED_ISCSI_CONN_INITIAL_R2T: u8 = 1 << 2;
pub const QED_ISCSI_CONN_IMMEDIATE_DATA: u8 = 1 << 3;
pub const MAX_TID_BLOCKS_ISCSI: usize = 512;

#[repr(C)]
pub struct QedIscsiTid {
    pub size: u32,
    pub num_tids_per_block: u32,
    pub blocks: [*mut u8; MAX_TID_BLOCKS_ISCSI],
}

#[repr(C)]
pub struct QedIscsiCbOps {
    pub common: QedCommonCbOps,
}

/**
 * struct qed_iscsi_ops - qed iSCSI operations.
 * The operation descriptions are preserved from the C header.
 */
#[repr(C)]
pub struct QedIscsiOps {
    pub common: *const QedCommonOps,
    pub ll2: *const QedLl2Ops,
    pub fill_dev_info: Option<unsafe extern "C" fn(*mut QedDev, *mut QedDevIscsiInfo) -> i32>,
    pub register_ops: Option<unsafe extern "C" fn(*mut QedDev, *mut QedIscsiCbOps, *mut core::ffi::c_void)>,
    pub start: Option<unsafe extern "C" fn(*mut QedDev, *mut QedIscsiTid, *mut core::ffi::c_void, IscsiEventCbT) -> i32>,
    pub stop: Option<unsafe extern "C" fn(*mut QedDev) -> i32>,
    pub acquire_conn: Option<unsafe extern "C" fn(*mut QedDev, *mut u32, *mut u32, *mut *mut core::ffi::c_void) -> i32>,
    pub release_conn: Option<unsafe extern "C" fn(*mut QedDev, u32) -> i32>,
    pub offload_conn: Option<unsafe extern "C" fn(*mut QedDev, u32, *mut QedIscsiParamsOffload) -> i32>,
    pub update_conn: Option<unsafe extern "C" fn(*mut QedDev, u32, *mut QedIscsiParamsUpdate) -> i32>,
    pub destroy_conn: Option<unsafe extern "C" fn(*mut QedDev, u32, u8) -> i32>,
    pub clear_sq: Option<unsafe extern "C" fn(*mut QedDev, u32) -> i32>,
    pub get_stats: Option<unsafe extern "C" fn(*mut QedDev, *mut QedIscsiStats) -> i32>,
    pub change_mac: Option<unsafe extern "C" fn(*mut QedDev, u32, *const u8) -> i32>,
}

extern "C" {
    pub fn qed_get_iscsi_ops() -> *const QedIscsiOps;
    pub fn qed_put_iscsi_ops();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
