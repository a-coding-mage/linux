/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of nvme-fc-driver.h. */

/* C dependencies retained as external types: dma_addr_t, u8/u16/u32/u64,
 * __le16, sg_table, scatterlist, blk_mq_queue_map, device, and fc_ba_rjt. */

#[repr(C, align(8))]
pub struct nvmefc_ls_req {
    pub rqstaddr: *mut core::ffi::c_void,
    pub rqstdma: dma_addr_t,
    pub rqstlen: u32,
    pub rspaddr: *mut core::ffi::c_void,
    pub rspdma: dma_addr_t,
    pub rsplen: u32,
    pub timeout: u32,
    pub private: *mut core::ffi::c_void,
    pub done: Option<unsafe extern "C" fn(*mut nvmefc_ls_req, i32)>,
}

#[repr(C)]
pub struct nvmefc_ls_rsp {
    pub rspbuf: *mut core::ffi::c_void,
    pub rspdma: dma_addr_t,
    pub rsplen: u16,
    pub done: Option<unsafe extern "C" fn(*mut nvmefc_ls_rsp)>,
    pub nvme_fc_private: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct nvme_fc_port_info { pub node_name: u64, pub port_name: u64, pub port_role: u32, pub port_id: u32, pub dev_loss_tmo: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum nvmefc_fcp_datadir { NVMEFC_FCP_NODATA, NVMEFC_FCP_WRITE, NVMEFC_FCP_READ }

#[repr(C, align(8))]
pub struct nvmefc_fcp_req {
    pub cmdaddr: *mut core::ffi::c_void, pub rspaddr: *mut core::ffi::c_void,
    pub cmddma: dma_addr_t, pub rspdma: dma_addr_t, pub cmdlen: u16, pub rsplen: u16,
    pub payload_length: u32, pub sg_table: sg_table, pub first_sgl: *mut scatterlist,
    pub sg_cnt: i32, pub io_dir: nvmefc_fcp_datadir,
    pub done: Option<unsafe extern "C" fn(*mut nvmefc_fcp_req)>,
    pub private: *mut core::ffi::c_void, pub sqid: __le16,
    pub rcv_rsplen: u16, pub transferred_length: u32, pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum nvme_fc_obj_state {
    FC_OBJSTATE_UNKNOWN, FC_OBJSTATE_NOTPRESENT, FC_OBJSTATE_ONLINE,
    FC_OBJSTATE_OFFLINE, FC_OBJSTATE_BLOCKED, FC_OBJSTATE_BYPASSED,
    FC_OBJSTATE_DIAGNOSTICS, FC_OBJSTATE_LINKDOWN, FC_OBJSTATE_ERROR,
    FC_OBJSTATE_LOOPBACK, FC_OBJSTATE_DELETED,
}

#[repr(C, align(8))]
pub struct nvme_fc_local_port {
    pub port_num: u32, pub port_role: u32, pub node_name: u64, pub port_name: u64,
    pub private: *mut core::ffi::c_void, pub port_id: u32, pub port_state: nvme_fc_obj_state,
}

#[repr(C, align(8))]
pub struct nvme_fc_remote_port {
    pub port_num: u32, pub port_role: u32, pub node_name: u64, pub port_name: u64,
    pub localport: *mut nvme_fc_local_port, pub private: *mut core::ffi::c_void,
    pub dev_loss_tmo: u32, pub port_id: u32, pub port_state: nvme_fc_obj_state,
}

#[repr(C)]
pub struct nvme_fc_port_template {
    pub localport_delete: Option<unsafe extern "C" fn(*mut nvme_fc_local_port)>,
    pub remoteport_delete: Option<unsafe extern "C" fn(*mut nvme_fc_remote_port)>,
    pub create_queue: Option<unsafe extern "C" fn(*mut nvme_fc_local_port, u32, u16, *mut *mut core::ffi::c_void) -> i32>,
    pub delete_queue: Option<unsafe extern "C" fn(*mut nvme_fc_local_port, u32, *mut core::ffi::c_void)>,
    pub ls_req: Option<unsafe extern "C" fn(*mut nvme_fc_local_port, *mut nvme_fc_remote_port, *mut nvmefc_ls_req) -> i32>,
    pub fcp_io: Option<unsafe extern "C" fn(*mut nvme_fc_local_port, *mut nvme_fc_remote_port, *mut core::ffi::c_void, *mut nvmefc_fcp_req) -> i32>,
    pub ls_abort: Option<unsafe extern "C" fn(*mut nvme_fc_local_port, *mut nvme_fc_remote_port, *mut nvmefc_ls_req)>,
    pub fcp_abort: Option<unsafe extern "C" fn(*mut nvme_fc_local_port, *mut nvme_fc_remote_port, *mut core::ffi::c_void, *mut nvmefc_fcp_req)>,
    pub xmt_ls_rsp: Option<unsafe extern "C" fn(*mut nvme_fc_local_port, *mut nvme_fc_remote_port, *mut nvmefc_ls_rsp) -> i32>,
    pub map_queues: Option<unsafe extern "C" fn(*mut nvme_fc_local_port, *mut blk_mq_queue_map)>,
    pub max_hw_queues: u32, pub max_sgl_segments: u16, pub max_dif_sgl_segments: u16,
    pub dma_boundary: u64, pub local_priv_sz: u32, pub remote_priv_sz: u32,
    pub lsrqst_priv_sz: u32, pub fcprqst_priv_sz: u32,
}

#[repr(C)]
pub struct nvmet_fc_port_info { pub node_name: u64, pub port_name: u64, pub port_id: u32 }
pub const NVMET_FCOP_READDATA: i32 = 1;
pub const NVMET_FCOP_WRITEDATA: i32 = 2;
pub const NVMET_FCOP_READDATA_RSP: i32 = 3;
pub const NVMET_FCOP_RSP: i32 = 4;

#[repr(C)]
pub struct nvmefc_tgt_fcp_req {
    pub op: u8, pub hwqid: u16, pub offset: u32, pub timeout: u32, pub transfer_length: u32,
    pub ba_rjt: fc_ba_rjt, pub sg: *mut scatterlist, pub sg_cnt: i32,
    pub rspaddr: *mut core::ffi::c_void, pub rspdma: dma_addr_t, pub rsplen: u16,
    pub done: Option<unsafe extern "C" fn(*mut nvmefc_tgt_fcp_req)>,
    pub nvmet_fc_private: *mut core::ffi::c_void, pub transferred_length: u32, pub fcp_error: i32,
}
pub const NVMET_FCTGTFEAT_READDATA_RSP: u32 = 1 << 0;

#[repr(C, align(8))]
pub struct nvmet_fc_target_port {
    pub port_num: u32, pub node_name: u64, pub port_name: u64, pub private: *mut core::ffi::c_void,
    pub port_id: u32, pub port_state: nvme_fc_obj_state,
}

#[repr(C)]
pub struct nvmet_fc_target_template {
    pub targetport_delete: Option<unsafe extern "C" fn(*mut nvmet_fc_target_port)>,
    pub xmt_ls_rsp: Option<unsafe extern "C" fn(*mut nvmet_fc_target_port, *mut nvmefc_ls_rsp) -> i32>,
    pub fcp_op: Option<unsafe extern "C" fn(*mut nvmet_fc_target_port, *mut nvmefc_tgt_fcp_req) -> i32>,
    pub fcp_abort: Option<unsafe extern "C" fn(*mut nvmet_fc_target_port, *mut nvmefc_tgt_fcp_req)>,
    pub fcp_req_release: Option<unsafe extern "C" fn(*mut nvmet_fc_target_port, *mut nvmefc_tgt_fcp_req)>,
    pub defer_rcv: Option<unsafe extern "C" fn(*mut nvmet_fc_target_port, *mut nvmefc_tgt_fcp_req)>,
    pub discovery_event: Option<unsafe extern "C" fn(*mut nvmet_fc_target_port)>,
    pub ls_req: Option<unsafe extern "C" fn(*mut nvmet_fc_target_port, *mut core::ffi::c_void, *mut nvmefc_ls_req) -> i32>,
    pub ls_abort: Option<unsafe extern "C" fn(*mut nvmet_fc_target_port, *mut core::ffi::c_void, *mut nvmefc_ls_req)>,
    pub host_release: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub host_traddr: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut u64, *mut u64) -> i32>,
    pub max_hw_queues: u32, pub max_sgl_segments: u16, pub max_dif_sgl_segments: u16,
    pub dma_boundary: u64, pub target_features: u32, pub target_priv_sz: u32, pub lsrqst_priv_sz: u32,
}

extern "C" {
    pub fn nvme_fc_register_localport(pinfo: *mut nvme_fc_port_info, template: *mut nvme_fc_port_template, dev: *mut device, lport_p: *mut *mut nvme_fc_local_port) -> i32;
    pub fn nvme_fc_unregister_localport(localport: *mut nvme_fc_local_port) -> i32;
    pub fn nvme_fc_register_remoteport(localport: *mut nvme_fc_local_port, pinfo: *mut nvme_fc_port_info, rport_p: *mut *mut nvme_fc_remote_port) -> i32;
    pub fn nvme_fc_unregister_remoteport(remoteport: *mut nvme_fc_remote_port) -> i32;
    pub fn nvme_fc_rescan_remoteport(remoteport: *mut nvme_fc_remote_port);
    pub fn nvme_fc_set_remoteport_devloss(remoteport: *mut nvme_fc_remote_port, dev_loss_tmo: u32) -> i32;
    pub fn nvme_fc_rcv_ls_req(remoteport: *mut nvme_fc_remote_port, lsrsp: *mut nvmefc_ls_rsp, lsreqbuf: *mut core::ffi::c_void, lsreqbuf_len: u32) -> i32;
    pub fn nvme_fc_io_getuuid(req: *mut nvmefc_fcp_req) -> *mut i8;
    pub fn nvmet_fc_register_targetport(portinfo: *mut nvmet_fc_port_info, template: *mut nvmet_fc_target_template, dev: *mut device, tgtport_p: *mut *mut nvmet_fc_target_port) -> i32;
    pub fn nvmet_fc_unregister_targetport(tgtport: *mut nvmet_fc_target_port) -> i32;
    pub fn nvmet_fc_rcv_ls_req(tgtport: *mut nvmet_fc_target_port, hosthandle: *mut core::ffi::c_void, rsp: *mut nvmefc_ls_rsp, lsreqbuf: *mut core::ffi::c_void, lsreqbuf_len: u32) -> i32;
    pub fn nvmet_fc_invalidate_host(tgtport: *mut nvmet_fc_target_port, hosthandle: *mut core::ffi::c_void);
    pub fn nvmet_fc_rcv_fcp_req(tgtport: *mut nvmet_fc_target_port, fcpreq: *mut nvmefc_tgt_fcp_req, cmdiubuf: *mut core::ffi::c_void, cmdiubuf_len: u32) -> i32;
    pub fn nvmet_fc_rcv_fcp_abort(tgtport: *mut nvmet_fc_target_port, fcpreq: *mut nvmefc_tgt_fcp_req);
}

pub const NVME_FC_FEAT_UUID: u32 = 0x0001;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
