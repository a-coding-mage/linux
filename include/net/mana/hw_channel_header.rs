/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause */
/* Copyright (c) 2021, Microsoft Corporation. */

pub const DEFAULT_LOG2_THROTTLING_FOR_ERROR_EQ: u32 = 4;
pub const HW_CHANNEL_MAX_REQUEST_SIZE: u32 = 0x1000;
pub const HW_CHANNEL_MAX_RESPONSE_SIZE: u32 = 0x1000;
pub const HW_CHANNEL_VF_BOOTSTRAP_QUEUE_DEPTH: u32 = 1;

pub const HWC_INIT_DATA_CQID: u32 = 1;
pub const HWC_INIT_DATA_RQID: u32 = 2;
pub const HWC_INIT_DATA_SQID: u32 = 3;
pub const HWC_INIT_DATA_QUEUE_DEPTH: u32 = 4;
pub const HWC_INIT_DATA_MAX_REQUEST: u32 = 5;
pub const HWC_INIT_DATA_MAX_RESPONSE: u32 = 6;
pub const HWC_INIT_DATA_MAX_NUM_CQS: u32 = 7;
pub const HWC_INIT_DATA_PDID: u32 = 8;
pub const HWC_INIT_DATA_GPA_MKEY: u32 = 9;
pub const HWC_INIT_DATA_PF_DEST_RQ_ID: u32 = 10;
pub const HWC_INIT_DATA_PF_DEST_CQ_ID: u32 = 11;

pub const HWC_DATA_CFG_HWC_TIMEOUT: u32 = 1;
pub const HWC_DATA_HW_LINK_CONNECT: u32 = 2;
pub const HWC_DATA_HW_LINK_DISCONNECT: u32 = 3;
pub const HW_CHANNEL_WAIT_RESOURCE_TIMEOUT_MS: u32 = 30000;

#[repr(C)]
pub union hwc_init_eq_id_db {
    pub as_uint32: u32,
    /* eq_id: bits 0..16, doorbell: bits 16..32 */
}

#[repr(C)]
pub union hwc_init_type_data {
    pub as_uint32: u32,
    /* value: bits 0..24, type: bits 24..32 */
}

#[repr(C)]
pub union hwc_init_soc_service_type {
    pub as_uint32: u32,
    /* value: bits 0..28, type: bits 28..32 */
}

#[repr(C)]
pub struct hwc_rx_oob {
    /* type: 6, eom: 1, som: 1, vendor_err: 8, reserved1: 16 */
    pub type_: u32,
    pub eom: u32,
    pub som: u32,
    pub vendor_err: u32,
    pub reserved1: u32,
    /* src_virt_wq: 24, src_vfid: 8 */
    pub src_virt_wq: u32,
    pub src_vfid: u32,
    pub reserved2: u32,
    pub wqe_addr_low: u32,
    pub wqe_addr_high: u32,
    /* client_data_unit: 14, reserved3: 18 */
    pub client_data_unit: u32,
    pub reserved3: u32,
    pub tx_oob_data_size: u32,
    /* chunk_offset: 21, reserved4: 11 */
    pub chunk_offset: u32,
    pub reserved4: u32,
}

#[repr(C)]
pub struct hwc_tx_oob {
    pub reserved1: u32,
    pub reserved2: u32,
    /* vrq_id: 24, dest_vfid: 8 */
    pub vrq_id: u32,
    pub dest_vfid: u32,
    /* vrcq_id: 24, reserved3: 8 */
    pub vrcq_id: u32,
    pub reserved3: u32,
    /* vscq_id: 24, loopback: 1, lso_override: 1, dest_pf: 1, reserved4: 5 */
    pub vscq_id: u32,
    pub loopback: u32,
    pub lso_override: u32,
    pub dest_pf: u32,
    pub reserved4: u32,
    /* vsq_id: 24, reserved5: 8 */
    pub vsq_id: u32,
    pub reserved5: u32,
}

#[repr(C)]
pub struct hwc_work_request {
    pub buf_va: *mut core::ffi::c_void,
    pub buf_sge_addr: *mut core::ffi::c_void,
    pub buf_len: u32,
    pub msg_size: u32,
    pub wqe_req: gdma_wqe_request,
    pub tx_oob: hwc_tx_oob,
    pub sge: gdma_sge,
}

#[repr(C)]
pub struct hwc_dma_buf {
    pub mem_info: gdma_mem_info,
    pub gpa_mkey: u32,
    pub num_reqs: u32,
    pub reqs: [hwc_work_request; 0],
}

pub type hwc_rx_event_handler_t = unsafe extern "C" fn(
    ctx: *mut core::ffi::c_void,
    gdma_rxq_id: u32,
    rx_oob: *const hwc_rx_oob,
);
pub type hwc_tx_event_handler_t = unsafe extern "C" fn(
    ctx: *mut core::ffi::c_void,
    gdma_txq_id: u32,
    rx_oob: *const hwc_rx_oob,
);

#[repr(C)]
pub struct hwc_cq {
    pub hwc: *mut hw_channel_context,
    pub gdma_cq: *mut gdma_queue,
    pub gdma_eq: *mut gdma_queue,
    pub comp_buf: *mut gdma_comp,
    pub queue_depth: u16,
    pub rx_event_handler: Option<hwc_rx_event_handler_t>,
    pub rx_event_ctx: *mut core::ffi::c_void,
    pub tx_event_handler: Option<hwc_tx_event_handler_t>,
    pub tx_event_ctx: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct hwc_wq {
    pub hwc: *mut hw_channel_context,
    pub gdma_wq: *mut gdma_queue,
    pub msg_buf: *mut hwc_dma_buf,
    pub queue_depth: u16,
    pub hwc_cq: *mut hwc_cq,
}

#[repr(C)]
pub struct hwc_caller_ctx {
    pub comp_event: completion,
    pub output_buf: *mut core::ffi::c_void,
    pub output_buflen: u32,
    pub error: u32,
    pub status_code: u32,
}

#[repr(C)]
pub struct hw_channel_context {
    pub gdma_dev: *mut gdma_dev,
    pub dev: *mut device,
    pub num_inflight_msg: u16,
    pub max_req_msg_size: u32,
    pub hwc_init_q_depth_max: u16,
    pub hwc_init_max_req_msg_size: u32,
    pub hwc_init_max_resp_msg_size: u32,
    pub hwc_init_eqe_comp: completion,
    pub rxq: *mut hwc_wq,
    pub txq: *mut hwc_wq,
    pub cq: *mut hwc_cq,
    pub sema: semaphore,
    pub inflight_msg_res: gdma_resource,
    pub pf_dest_vrq_id: u32,
    pub pf_dest_vrcq_id: u32,
    pub hwc_timeout: u32,
    pub caller_ctx: *mut hwc_caller_ctx,
}

unsafe extern "C" {
    pub fn mana_hwc_create_channel(gc: *mut gdma_context) -> i32;
    pub fn mana_hwc_destroy_channel(gc: *mut gdma_context);
    pub fn mana_hwc_send_request(
        hwc: *mut hw_channel_context,
        req_len: u32,
        req: *const core::ffi::c_void,
        resp_len: u32,
        resp: *mut core::ffi::c_void,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
