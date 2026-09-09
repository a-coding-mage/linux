/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/*
 * Copyright (c) 2004, 2005 Intel Corporation.  All rights reserved.
 * Copyright (c) 2004 Topspin Corporation.  All rights reserved.
 * Copyright (c) 2004 Voltaire Corporation.  All rights reserved.
 * Copyright (c) 2005 Sun Microsystems, Inc. All rights reserved.
 * Copyright (c) 2019, Mellanox Technologies inc.  All rights reserved.
 */

// Dependencies supplied by <rdma/ib_mad.h>, <rdma/ib_sa.h>, and
// <rdma/rdma_cm.h> remain external to this translation.

#[repr(C)]
pub enum ib_cm_state { IB_CM_IDLE, IB_CM_LISTEN, IB_CM_REQ_SENT, IB_CM_REQ_RCVD,
    IB_CM_MRA_REQ_SENT, IB_CM_MRA_REQ_RCVD, IB_CM_REP_SENT, IB_CM_REP_RCVD,
    IB_CM_MRA_REP_SENT, IB_CM_MRA_REP_RCVD, IB_CM_ESTABLISHED, IB_CM_DREQ_SENT,
    IB_CM_DREQ_RCVD, IB_CM_TIMEWAIT, IB_CM_SIDR_REQ_SENT, IB_CM_SIDR_REQ_RCVD }

#[repr(C)]
pub enum ib_cm_lap_state { IB_CM_LAP_UNINIT, IB_CM_LAP_IDLE, IB_CM_LAP_SENT,
    IB_CM_LAP_RCVD, IB_CM_MRA_LAP_SENT, IB_CM_MRA_LAP_RCVD }

#[repr(C)]
pub enum ib_cm_event_type { IB_CM_REQ_ERROR, IB_CM_REQ_RECEIVED, IB_CM_REP_ERROR,
    IB_CM_REP_RECEIVED, IB_CM_RTU_RECEIVED, IB_CM_USER_ESTABLISHED, IB_CM_DREQ_ERROR,
    IB_CM_DREQ_RECEIVED, IB_CM_DREP_RECEIVED, IB_CM_TIMEWAIT_EXIT, IB_CM_MRA_RECEIVED,
    IB_CM_REJ_RECEIVED, IB_CM_LAP_ERROR, IB_CM_LAP_RECEIVED, IB_CM_APR_RECEIVED,
    IB_CM_SIDR_REQ_ERROR, IB_CM_SIDR_REQ_RECEIVED, IB_CM_SIDR_REP_RECEIVED }

#[repr(C)]
pub enum ib_cm_data_size { IB_CM_REQ_PRIVATE_DATA_SIZE = 92, IB_CM_MRA_PRIVATE_DATA_SIZE = 222,
    IB_CM_REJ_PRIVATE_DATA_SIZE = 148, IB_CM_REP_PRIVATE_DATA_SIZE = 196,
    IB_CM_RTU_PRIVATE_DATA_SIZE = 224, IB_CM_DREQ_PRIVATE_DATA_SIZE = 220,
    IB_CM_DREP_PRIVATE_DATA_SIZE = 224, IB_CM_REJ_ARI_LENGTH = 72,
    IB_CM_LAP_PRIVATE_DATA_SIZE = 168, IB_CM_APR_PRIVATE_DATA_SIZE = 148,
    IB_CM_APR_INFO_LENGTH = 72, IB_CM_SIDR_REQ_PRIVATE_DATA_SIZE = 216,
    IB_CM_SIDR_REP_PRIVATE_DATA_SIZE = 136, IB_CM_SIDR_REP_INFO_LENGTH = 72 }

pub enum ib_cm_id {}

#[repr(C)]
pub struct ib_cm_req_event_param {
    pub listen_id: *mut ib_cm_id, pub bth_pkey: u16, pub port: u8,
    pub primary_path: *mut sa_path_rec, pub alternate_path: *mut sa_path_rec,
    pub ppath_sgid_attr: *const ib_gid_attr, pub remote_ca_guid: __be64,
    pub remote_qkey: u32, pub remote_qpn: u32, pub qp_type: ib_qp_type,
    pub starting_psn: u32, pub responder_resources: u8, pub initiator_depth: u8,
    pub local_cm_response_timeout: u8, pub flow_control: u8,
    pub remote_cm_response_timeout: u8, pub retry_count: u8, pub rnr_retry_count: u8,
    pub srq: u8, pub ece: rdma_ucm_ece,
}

#[repr(C)]
pub struct ib_cm_rep_event_param { pub remote_ca_guid: __be64, pub remote_qkey: u32,
    pub remote_qpn: u32, pub starting_psn: u32, pub responder_resources: u8,
    pub initiator_depth: u8, pub target_ack_delay: u8, pub failover_accepted: u8,
    pub flow_control: u8, pub rnr_retry_count: u8, pub srq: u8, pub ece: rdma_ucm_ece }

#[repr(C)]
pub enum ib_cm_rej_reason { IB_CM_REJ_NO_QP = 1, IB_CM_REJ_NO_EEC, IB_CM_REJ_NO_RESOURCES,
    IB_CM_REJ_TIMEOUT, IB_CM_REJ_UNSUPPORTED, IB_CM_REJ_INVALID_COMM_ID,
    IB_CM_REJ_INVALID_COMM_INSTANCE, IB_CM_REJ_INVALID_SERVICE_ID,
    IB_CM_REJ_INVALID_TRANSPORT_TYPE, IB_CM_REJ_STALE_CONN, IB_CM_REJ_RDC_NOT_EXIST,
    IB_CM_REJ_INVALID_GID, IB_CM_REJ_INVALID_LID, IB_CM_REJ_INVALID_SL,
    IB_CM_REJ_INVALID_TRAFFIC_CLASS, IB_CM_REJ_INVALID_HOP_LIMIT,
    IB_CM_REJ_INVALID_PACKET_RATE, IB_CM_REJ_INVALID_ALT_GID, IB_CM_REJ_INVALID_ALT_LID,
    IB_CM_REJ_INVALID_ALT_SL, IB_CM_REJ_INVALID_ALT_TRAFFIC_CLASS,
    IB_CM_REJ_INVALID_ALT_HOP_LIMIT, IB_CM_REJ_INVALID_ALT_PACKET_RATE,
    IB_CM_REJ_PORT_CM_REDIRECT, IB_CM_REJ_PORT_REDIRECT, IB_CM_REJ_INVALID_MTU,
    IB_CM_REJ_INSUFFICIENT_RESP_RESOURCES, IB_CM_REJ_CONSUMER_DEFINED,
    IB_CM_REJ_INVALID_RNR_RETRY, IB_CM_REJ_DUPLICATE_LOCAL_COMM_ID,
    IB_CM_REJ_INVALID_CLASS_VERSION, IB_CM_REJ_INVALID_FLOW_LABEL,
    IB_CM_REJ_INVALID_ALT_FLOW_LABEL, IB_CM_REJ_VENDOR_OPTION_NOT_SUPPORTED = 35 }

#[repr(C)] pub struct ib_cm_rej_event_param { pub reason: ib_cm_rej_reason, pub ari: *mut core::ffi::c_void, pub ari_length: u8 }
#[repr(C)] pub struct ib_cm_mra_event_param { pub service_timeout: u8 }
#[repr(C)] pub struct ib_cm_lap_event_param { pub alternate_path: *mut sa_path_rec }
#[repr(C)] pub enum ib_cm_apr_status { IB_CM_APR_SUCCESS, IB_CM_APR_INVALID_COMM_ID, IB_CM_APR_UNSUPPORTED, IB_CM_APR_REJECT, IB_CM_APR_REDIRECT, IB_CM_APR_IS_CURRENT, IB_CM_APR_INVALID_QPN_EECN, IB_CM_APR_INVALID_LID, IB_CM_APR_INVALID_GID, IB_CM_APR_INVALID_FLOW_LABEL, IB_CM_APR_INVALID_TCLASS, IB_CM_APR_INVALID_HOP_LIMIT, IB_CM_APR_INVALID_PACKET_RATE, IB_CM_APR_INVALID_SL }
#[repr(C)] pub struct ib_cm_apr_event_param { pub ap_status: ib_cm_apr_status, pub apr_info: *mut core::ffi::c_void, pub info_len: u8 }
#[repr(C)] pub struct ib_cm_sidr_req_event_param { pub listen_id: *mut ib_cm_id, pub service_id: __be64, pub sgid_attr: *const ib_gid_attr, pub bth_pkey: u16, pub port: u8, pub pkey: u16 }
#[repr(C)] pub enum ib_cm_sidr_status { IB_SIDR_SUCCESS, IB_SIDR_UNSUPPORTED, IB_SIDR_REJECT, IB_SIDR_NO_QP, IB_SIDR_REDIRECT, IB_SIDR_UNSUPPORTED_VERSION }
#[repr(C)] pub struct ib_cm_sidr_rep_event_param { pub status: ib_cm_sidr_status, pub qkey: u32, pub qpn: u32, pub info: *mut core::ffi::c_void, pub sgid_attr: *const ib_gid_attr, pub info_len: u8 }

#[repr(C)] pub union ib_cm_event_param { pub req_rcvd: ib_cm_req_event_param, pub rep_rcvd: ib_cm_rep_event_param, pub rej_rcvd: ib_cm_rej_event_param, pub mra_rcvd: ib_cm_mra_event_param, pub lap_rcvd: ib_cm_lap_event_param, pub apr_rcvd: ib_cm_apr_event_param, pub sidr_req_rcvd: ib_cm_sidr_req_event_param, pub sidr_rep_rcvd: ib_cm_sidr_rep_event_param, pub send_status: ib_wc_status }
#[repr(C)] pub struct ib_cm_event { pub event: ib_cm_event_type, pub param: ib_cm_event_param, pub private_data: *mut core::ffi::c_void }

pub const CM_REQ_ATTR_ID: u16 = 0x0010u16.to_be(); pub const CM_MRA_ATTR_ID: u16 = 0x0011u16.to_be(); pub const CM_REJ_ATTR_ID: u16 = 0x0012u16.to_be(); pub const CM_REP_ATTR_ID: u16 = 0x0013u16.to_be(); pub const CM_RTU_ATTR_ID: u16 = 0x0014u16.to_be(); pub const CM_DREQ_ATTR_ID: u16 = 0x0015u16.to_be(); pub const CM_DREP_ATTR_ID: u16 = 0x0016u16.to_be(); pub const CM_SIDR_REQ_ATTR_ID: u16 = 0x0017u16.to_be(); pub const CM_SIDR_REP_ATTR_ID: u16 = 0x0018u16.to_be(); pub const CM_LAP_ATTR_ID: u16 = 0x0019u16.to_be(); pub const CM_APR_ATTR_ID: u16 = 0x001Au16.to_be();

pub type ib_cm_handler = unsafe extern "C" fn(*mut ib_cm_id, *const ib_cm_event) -> i32;
#[repr(C)] pub struct ib_cm_id { pub cm_handler: ib_cm_handler, pub context: *mut core::ffi::c_void, pub device: *mut ib_device, pub service_id: __be64, pub state: ib_cm_state, pub lap_state: ib_cm_lap_state, pub local_id: __be32, pub remote_id: __be32, pub remote_cm_qpn: u32 }

extern "C" { pub fn ib_create_cm_id(device: *mut ib_device, cm_handler: ib_cm_handler, context: *mut core::ffi::c_void) -> *mut ib_cm_id; pub fn ib_destroy_cm_id(cm_id: *mut ib_cm_id); }
pub const IB_SERVICE_ID_AGN_MASK: u64 = 0xFF00000000000000u64.to_be(); pub const IB_CM_ASSIGN_SERVICE_ID: u64 = 0x0200000000000000u64.to_be(); pub const IB_CMA_SERVICE_ID: u64 = 0x0000000001000000u64.to_be(); pub const IB_CMA_SERVICE_ID_MASK: u64 = 0xFFFFFFFFFF000000u64.to_be(); pub const IB_SDP_SERVICE_ID: u64 = 0x0000000000010000u64.to_be(); pub const IB_SDP_SERVICE_ID_MASK: u64 = 0xFFFFFFFFFFFF0000u64.to_be();

#[repr(C)] pub struct ib_cm_req_param { pub primary_path:*mut sa_path_rec, pub primary_path_inbound:*mut sa_path_rec, pub primary_path_outbound:*mut sa_path_rec, pub alternate_path:*mut sa_path_rec, pub ppath_sgid_attr:*const ib_gid_attr, pub service_id:__be64, pub qp_num:u32, pub qp_type:ib_qp_type, pub starting_psn:u32, pub private_data:*const core::ffi::c_void, pub private_data_len:u8, pub responder_resources:u8, pub initiator_depth:u8, pub remote_cm_response_timeout:u8, pub flow_control:u8, pub local_cm_response_timeout:u8, pub retry_count:u8, pub rnr_retry_count:u8, pub max_cm_retries:u8, pub srq:u8, pub ece:rdma_ucm_ece }
#[repr(C)] pub struct ib_cm_rep_param { pub qp_num:u32, pub starting_psn:u32, pub private_data:*const core::ffi::c_void, pub private_data_len:u8, pub responder_resources:u8, pub initiator_depth:u8, pub failover_accepted:u8, pub flow_control:u8, pub rnr_retry_count:u8, pub srq:u8, pub ece:rdma_ucm_ece }
#[repr(C)] pub struct ib_cm_sidr_req_param { pub path:*mut sa_path_rec, pub sgid_attr:*const ib_gid_attr, pub service_id:__be64, pub timeout_ms:usize, pub private_data:*const core::ffi::c_void, pub private_data_len:u8, pub max_cm_retries:u8 }
#[repr(C)] pub struct ib_cm_sidr_rep_param { pub qp_num:u32, pub qkey:u32, pub status:ib_cm_sidr_status, pub info:*const core::ffi::c_void, pub info_length:u8, pub private_data:*const core::ffi::c_void, pub private_data_len:u8, pub ece:rdma_ucm_ece }

extern "C" {
    pub fn ib_cm_listen(*mut ib_cm_id, __be64) -> i32;
    pub fn ib_cm_insert_listen(*mut ib_device, ib_cm_handler, __be64) -> *mut ib_cm_id;
    pub fn ib_send_cm_req(*mut ib_cm_id, *mut ib_cm_req_param) -> i32;
    pub fn ib_send_cm_rep(*mut ib_cm_id, *mut ib_cm_rep_param) -> i32;
    pub fn ib_send_cm_rtu(*mut ib_cm_id, *const core::ffi::c_void, u8) -> i32;
    pub fn ib_send_cm_dreq(*mut ib_cm_id, *const core::ffi::c_void, u8) -> i32;
    pub fn ib_send_cm_drep(*mut ib_cm_id, *const core::ffi::c_void, u8) -> i32;
    pub fn ib_cm_notify(*mut ib_cm_id, ib_event_type) -> i32;
    pub fn ib_send_cm_rej(*mut ib_cm_id, ib_cm_rej_reason, *mut core::ffi::c_void, u8, *const core::ffi::c_void, u8) -> i32;
    pub fn ib_prepare_cm_mra(*mut ib_cm_id) -> i32;
    pub fn ib_cm_init_qp_attr(*mut ib_cm_id, *mut ib_qp_attr, *mut i32) -> i32;
    pub fn ib_send_cm_sidr_req(*mut ib_cm_id, *mut ib_cm_sidr_req_param) -> i32;
    pub fn ib_send_cm_sidr_rep(*mut ib_cm_id, *mut ib_cm_sidr_rep_param) -> i32;
    pub fn ibcm_reject_msg(reason: i32) -> *const core::ffi::c_char;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
