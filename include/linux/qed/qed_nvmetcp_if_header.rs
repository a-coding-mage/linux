/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/* Copyright 2021 Marvell. All rights reserved. */

// Dependencies supplied by the surrounding kernel/Rust translation.

pub const QED_NVMETCP_MAX_IO_SIZE: u32 = 0x800000;
pub const QED_NVMETCP_CMN_HDR_SIZE: usize = core::mem::size_of::<nvme_tcp_hdr>();
pub const QED_NVMETCP_CMD_HDR_SIZE: usize = core::mem::size_of::<nvme_tcp_cmd_pdu>();
pub const QED_NVMETCP_NON_IO_HDR_SIZE: usize = QED_NVMETCP_CMN_HDR_SIZE + 16;

pub type nvmetcp_event_cb_t = unsafe extern "C" fn(
    context: *mut core::ffi::c_void,
    fw_event_code: u8,
    fw_handle: *mut core::ffi::c_void,
) -> i32;

#[repr(C)]
pub struct qed_dev_nvmetcp_info {
    pub common: qed_dev_info,
    pub port_id: u8, /* Physical port */
    pub num_cqs: u8,
}

pub const MAX_TID_BLOCKS_NVMETCP: usize = 512;

#[repr(C)]
pub struct qed_nvmetcp_tid {
    pub size: u32, /* In bytes per task */
    pub num_tids_per_block: u32,
    pub blocks: [*mut u8; MAX_TID_BLOCKS_NVMETCP],
}

#[repr(C)]
pub struct qed_nvmetcp_id_params {
    pub mac: [u8; ETH_ALEN],
    pub ip: [u32; 4],
    pub port: u16,
}

#[repr(C)]
pub struct qed_nvmetcp_params_offload {
    /* FW initializations */
    pub sq_pbl_addr: dma_addr_t,
    pub nvmetcp_cccid_itid_table_addr: dma_addr_t,
    pub nvmetcp_cccid_max_range: u16,
    pub default_cq: u8,

    /* Networking and TCP stack initializations */
    pub src: qed_nvmetcp_id_params,
    pub dst: qed_nvmetcp_id_params,
    pub ka_timeout: u32,
    pub ka_interval: u32,
    pub max_rt_time: u32,
    pub cwnd: u32,
    pub mss: u16,
    pub vlan_id: u16,
    pub timestamp_en: bool,
    pub delayed_ack_en: bool,
    pub tcp_keep_alive_en: bool,
    pub ecn_en: bool,
    pub ip_version: u8,
    pub ka_max_probe_cnt: u8,
    pub ttl: u8,
    pub tos_or_tc: u8,
    pub rcv_wnd_scale: u8,
}

#[repr(C)]
pub struct qed_nvmetcp_params_update {
    pub max_io_size: u32,
    pub max_recv_pdu_length: u32,
    pub max_send_pdu_length: u32,

    /* Placeholder: pfv, cpda, hpda */
    pub hdr_digest_en: bool,
    pub data_digest_en: bool,
}

#[repr(C)]
pub struct qed_nvmetcp_cb_ops {
    pub common: qed_common_cb_ops,
}

#[repr(C)]
pub struct nvmetcp_sge {
    pub sge_addr: regpair, /* SGE address */
    pub sge_len: __le32, /* SGE length */
    pub reserved: __le32,
}

/* IO path HSI function SGL params */
#[repr(C)]
pub struct storage_sgl_task_params {
    pub sgl: *mut nvmetcp_sge,
    pub sgl_phys_addr: regpair,
    pub total_buffer_size: u32,
    pub num_sges: u16,
    pub small_mid_sge: bool,
}

/* IO path HSI function FW task context params */
#[repr(C)]
pub struct nvmetcp_task_params {
    pub context: *mut core::ffi::c_void, /* Output parameter - set/filled by the HSI function */
    pub sqe: *mut nvmetcp_wqe,
    pub tx_io_size: u32, /* in bytes (Without DIF, if exists) */
    pub rx_io_size: u32, /* in bytes (Without DIF, if exists) */
    pub conn_icid: u16,
    pub itid: u16,
    pub opq: regpair, /* qedn_task_ctx address */
    pub host_cccid: u16,
    pub cq_rss_number: u8,
    pub send_write_incapsule: bool,
}

#[repr(C)]
pub struct qed_nvmetcp_ops {
    pub common: *const qed_common_ops,
    pub ll2: *const qed_ll2_ops,
    pub fill_dev_info: Option<unsafe extern "C" fn(*mut qed_dev, *mut qed_dev_nvmetcp_info) -> i32>,
    pub register_ops: Option<unsafe extern "C" fn(*mut qed_dev, *mut qed_nvmetcp_cb_ops, *mut core::ffi::c_void)>,
    pub start: Option<unsafe extern "C" fn(*mut qed_dev, *mut qed_nvmetcp_tid, *mut core::ffi::c_void, nvmetcp_event_cb_t) -> i32>,
    pub stop: Option<unsafe extern "C" fn(*mut qed_dev) -> i32>,
    pub acquire_conn: Option<unsafe extern "C" fn(*mut qed_dev, *mut u32, *mut u32, *mut *mut core::ffi::c_void) -> i32>,
    pub release_conn: Option<unsafe extern "C" fn(*mut qed_dev, u32) -> i32>,
    pub offload_conn: Option<unsafe extern "C" fn(*mut qed_dev, u32, *mut qed_nvmetcp_params_offload) -> i32>,
    pub update_conn: Option<unsafe extern "C" fn(*mut qed_dev, u32, *mut qed_nvmetcp_params_update) -> i32>,
    pub destroy_conn: Option<unsafe extern "C" fn(*mut qed_dev, u32, u8) -> i32>,
    pub clear_sq: Option<unsafe extern "C" fn(*mut qed_dev, u32) -> i32>,
    pub add_src_tcp_port_filter: Option<unsafe extern "C" fn(*mut qed_dev, u16) -> i32>,
    pub remove_src_tcp_port_filter: Option<unsafe extern "C" fn(*mut qed_dev, u16)>,
    pub add_dst_tcp_port_filter: Option<unsafe extern "C" fn(*mut qed_dev, u16) -> i32>,
    pub remove_dst_tcp_port_filter: Option<unsafe extern "C" fn(*mut qed_dev, u16)>,
    pub clear_all_filters: Option<unsafe extern "C" fn(*mut qed_dev)>,
    pub init_read_io: Option<unsafe extern "C" fn(*mut nvmetcp_task_params, *mut nvme_tcp_cmd_pdu, *mut nvme_command, *mut storage_sgl_task_params)>,
    pub init_write_io: Option<unsafe extern "C" fn(*mut nvmetcp_task_params, *mut nvme_tcp_cmd_pdu, *mut nvme_command, *mut storage_sgl_task_params)>,
    pub init_icreq_exchange: Option<unsafe extern "C" fn(*mut nvmetcp_task_params, *mut nvme_tcp_icreq_pdu, *mut storage_sgl_task_params, *mut storage_sgl_task_params)>,
    pub init_task_cleanup: Option<unsafe extern "C" fn(*mut nvmetcp_task_params)>,
}

unsafe extern "C" {
    pub fn qed_get_nvmetcp_ops() -> *const qed_nvmetcp_ops;
    pub fn qed_put_nvmetcp_ops();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
