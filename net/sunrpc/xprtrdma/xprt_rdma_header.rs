/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause */
/* Source-level Rust translation of xprt_rdma.h. */

// External kernel/RDMA types and constants are supplied by other translation units.
use core::ffi::c_void;

pub type u8 = core::primitive::u8;
pub type u32 = core::primitive::u32;
pub type u64 = core::primitive::u64;
pub type i32 = core::primitive::i32;
pub type size_t = usize;
pub type __be32 = u32;
pub type gfp_t = u32;
pub type dma_addr_t = u64;

extern "C" {
    pub static mut HZ: u32;
    pub static mut PAGE_SIZE: usize;
    pub static mut PAGE_SHIFT: usize;
}

#[repr(C)] pub struct kref { _priv: [u8; 0] }
#[repr(C)] pub struct rdma_cm_id { _priv: [u8; 0] }
#[repr(C)] pub struct ib_pd { _priv: [u8; 0] }
#[repr(C)] pub struct rpcrdma_mr { _priv: [u8; 0] }
#[repr(C)] pub struct completion { _priv: [u8; 0] }
#[repr(C)] pub struct ib_qp_init_attr { _priv: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _priv: [u8; 0] }
#[repr(C)] pub struct rpc_xprt { pub address_strings: *mut *const i8 }
#[repr(C)] pub struct rpcrdma_connect_private { _priv: [u8; 0] }
#[repr(C)] pub struct rdma_conn_param { _priv: [u8; 0] }
#[repr(C)] pub struct rpcrdma_notification { _priv: [u8; 0] }
#[repr(C)] pub struct ib_device { _priv: [u8; 0] }
#[repr(C)] pub struct ib_sge { pub addr: u64, pub length: u32, pub lkey: u32 }
#[repr(C)] pub struct ib_cqe { _priv: [u8; 0] }
#[repr(C)] pub struct rpc_rdma_cid { _priv: [u8; 0] }
#[repr(C)] pub struct rpc_rqst { _priv: [u8; 0] }
#[repr(C)] pub struct xdr_buf { pub head: *mut xdr_buf_head, pub len: usize }
#[repr(C)] pub struct xdr_buf_head { pub iov_len: usize }
#[repr(C)] pub struct xdr_stream { _priv: [u8; 0] }
#[repr(C)] pub struct llist_node { _priv: [u8; 0] }
#[repr(C)] pub struct ib_recv_wr { _priv: [u8; 0] }
#[repr(C)] pub struct list_head { _priv: [u8; 0] }
#[repr(C)] pub struct ib_mr { _priv: [u8; 0] }
#[repr(C)] pub struct scatterlist { _priv: [u8; 0] }
#[repr(C)] pub struct ib_reg_wr { _priv: [u8; 0] }
#[repr(C)] pub struct ib_send_wr { _priv: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _priv: [u8; 0] }
#[repr(C)] pub struct llist_head { _priv: [u8; 0] }
#[repr(C)] pub struct work_struct { _priv: [u8; 0] }
#[repr(C)] pub struct delayed_work { _priv: [u8; 0] }
#[repr(C)] pub struct rpc_timeout { _priv: [u8; 0] }
#[repr(C)] pub struct ib_wc { _priv: [u8; 0] }
#[repr(C)] pub struct sockaddr { _priv: [u8; 0] }
#[repr(C)] pub struct seq_file { _priv: [u8; 0] }
#[repr(C)] pub struct xprt_class { _priv: [u8; 0] }
#[repr(C)] pub struct atomic_t { _priv: [u8; 0] }
#[repr(C)] pub struct enum_placeholder { _priv: [u8; 0] }

pub const RDMA_RESOLVE_TIMEOUT: u32 = 5000;
pub const RDMA_CONNECT_RETRY_MAX: u32 = 2;
pub const RPCRDMA_BIND_TO: u32 = 60 * 1000;
pub const RPCRDMA_INIT_REEST_TO: u32 = 5 * 1000;
pub const RPCRDMA_MAX_REEST_TO: u32 = 30 * 1000;
pub const RPCRDMA_IDLE_DISC_TO: u32 = 5 * 60 * 1000;

#[repr(C)] pub struct rpcrdma_ep {
    pub re_kref: kref, pub re_id: *mut rdma_cm_id, pub re_pd: *mut ib_pd,
    pub re_max_rdma_segs: u32, pub re_max_fr_depth: u32, pub re_write_pad_mr: *mut rpcrdma_mr,
    pub re_mrtype: i32, pub re_done: completion, pub re_send_count: u32, pub re_send_batch: u32,
    pub re_max_inline_send: u32, pub re_max_inline_recv: u32, pub re_async_rc: i32,
    pub re_connect_status: i32, pub re_receiving: atomic_t, pub re_force_disconnect: atomic_t,
    pub re_attr: ib_qp_init_attr, pub re_connect_wait: wait_queue_head_t, pub re_xprt: *mut rpc_xprt,
    pub re_cm_private: rpcrdma_connect_private, pub re_remote_cma: rdma_conn_param,
    pub re_rn: rpcrdma_notification, pub re_receive_count: i32, pub re_max_requests: u32,
    pub re_recv_batch: u32, pub re_inline_send: u32, pub re_inline_recv: u32,
    pub re_completion_ids: atomic_t, pub re_write_pad: [u8; 4],
}

pub const RPCRDMA_BACKWARD_WRS: u32 = 0; // CONFIG_SUNRPC_BACKCHANNEL selects 32.

#[repr(C)] pub struct rpcrdma_regbuf { pub rg_iov: ib_sge, pub rg_device: *mut ib_device, pub rg_direction: i32, pub rg_data: *mut c_void }
#[inline] pub unsafe fn rdmab_addr(rb: *mut rpcrdma_regbuf) -> u64 { (*rb).rg_iov.addr }
#[inline] pub unsafe fn rdmab_length(rb: *mut rpcrdma_regbuf) -> u32 { (*rb).rg_iov.length }
#[inline] pub unsafe fn rdmab_lkey(rb: *mut rpcrdma_regbuf) -> u32 { (*rb).rg_iov.lkey }
#[inline] pub unsafe fn rdmab_device(rb: *mut rpcrdma_regbuf) -> *mut ib_device { (*rb).rg_device }
#[inline] pub unsafe fn rdmab_data(rb: *const rpcrdma_regbuf) -> *mut c_void { (*rb).rg_data }
pub const RPCRDMA_MAX_HDR_SEGS: u32 = 16;

#[repr(C)] pub struct rpcrdma_rep {
    pub rr_cqe: ib_cqe, pub rr_cid: rpc_rdma_cid, pub rr_xid: __be32, pub rr_vers: __be32,
    pub rr_proc: __be32, pub rr_wc_flags: i32, pub rr_inv_rkey: u32, pub rr_rdmabuf: *mut rpcrdma_regbuf,
    pub rr_rxprt: *mut rpcrdma_xprt, pub rr_rqst: *mut rpc_rqst, pub rr_hdrbuf: xdr_buf,
    pub rr_stream: xdr_stream, pub rr_node: llist_node, pub rr_recv_wr: ib_recv_wr, pub rr_all: list_head,
}
pub const RPCRDMA_MAX_RECV_BATCH: u32 = 7;
#[repr(C)] pub struct rpcrdma_sendctx { pub sc_cqe: ib_cqe, pub sc_cid: rpc_rdma_cid, pub sc_req: *mut rpcrdma_req, pub sc_unmap_count: u32, pub sc_sges: [ib_sge; 0] }

#[repr(C)] pub struct rpcrdma_mr {
    pub mr_list: list_head, pub mr_req: *mut rpcrdma_req, pub mr_ibmr: *mut ib_mr, pub mr_device: *mut ib_device,
    pub mr_sg: *mut scatterlist, pub mr_nents: i32, pub mr_dir: i32, pub mr_cqe: ib_cqe, pub mr_linv_done: completion,
    pub mr_regwr: ib_reg_wr, pub mr_xprt: *mut rpcrdma_xprt, pub mr_handle: u32, pub mr_length: u32,
    pub mr_offset: u64, pub mr_all: list_head, pub mr_cid: rpc_rdma_cid,
}

pub const RPCRDMA_MAX_IOV_SEGS: u32 = 3;
pub const RPCRDMA_MAX_DATA_SEGS: usize = (1024 * 1024) / 4096 + 1;
pub const RPCRDMA_MAX_SEGS: usize = RPCRDMA_MAX_DATA_SEGS + RPCRDMA_MAX_IOV_SEGS as usize;
#[repr(C)] pub struct rpcrdma_xdr_cursor { pub xc_buf: *const xdr_buf, pub xc_page_offset: u32, pub xc_flags: u32 }
pub const XC_HEAD_DONE: u32 = 1; pub const XC_PAGES_DONE: u32 = 2; pub const XC_TAIL_DONE: u32 = 4;
pub const RPCRDMA_MIN_SEND_SGES: u32 = 3;
pub const RPCRDMA_MAX_PAGE_SGES: usize = 0; // RPCRDMA_MAX_INLINE >> PAGE_SHIFT
pub const RPCRDMA_MAX_SEND_SGES: usize = 3 + RPCRDMA_MAX_PAGE_SGES;

#[repr(C)] pub struct rpcrdma_req {
    pub rl_node: llist_node, pub rl_slot: rpc_rqst, pub rl_reply: *mut rpcrdma_rep, pub rl_stream: xdr_stream,
    pub rl_hdrbuf: xdr_buf, pub rl_wr: ib_send_wr, pub rl_sendctx: *mut rpcrdma_sendctx,
    pub rl_rdmabuf: *mut rpcrdma_regbuf, pub rl_sendbuf: *mut rpcrdma_regbuf, pub rl_recvbuf: *mut rpcrdma_regbuf,
    pub rl_all: list_head, pub rl_kref: kref, pub rl_free_mrs: list_head, pub rl_registered: list_head,
}
#[repr(C)] pub struct rpcrdma_buffer { pub rb_lock: spinlock_t, pub rb_send_bufs: llist_head, pub rb_mrs: list_head, pub rb_sc_head: usize, pub rb_sc_tail: usize, pub rb_sc_last: usize, pub rb_sc_ctxs: *mut *mut rpcrdma_sendctx, pub rb_allreqs: list_head, pub rb_all_mrs: list_head, pub rb_all_reps: list_head, pub rb_free_reps: llist_head, pub rb_max_requests: __be32, pub rb_credits: u32, pub rb_bc_srv_max_requests: u32, pub rb_bc_max_requests: u32, pub rb_refresh_worker: work_struct }
#[repr(C)] pub struct rpcrdma_stats { pub read_chunk_count: usize, pub write_chunk_count: usize, pub reply_chunk_count: usize, pub total_rdma_request: u64, pub pullup_copy_count: u64, pub hardway_register_count: u64, pub failed_marshal_count: u64, pub bad_reply_count: u64, pub mrs_recycled: usize, pub mrs_orphaned: usize, pub mrs_allocated: usize, pub empty_sendctx_q: usize, pub total_rdma_reply: u64, pub fixup_copy_count: u64, pub local_inv_needed: u64, pub nomsg_call_count: u64, pub bcall_count: u64 }
#[repr(C)] pub struct rpcrdma_xprt { pub rx_xprt: rpc_xprt, pub rx_ep: *mut rpcrdma_ep, pub rx_buf: rpcrdma_buffer, pub rx_connect_worker: delayed_work, pub rx_timeout: rpc_timeout, pub rx_stats: rpcrdma_stats }

#[repr(C)] pub enum rpcrdma_chunktype { rpcrdma_noch = 0, rpcrdma_noch_pullup, rpcrdma_noch_mapped, rpcrdma_readch, rpcrdma_areadch, rpcrdma_writech, rpcrdma_replych }

extern "C" {
    pub static mut xprt_rdma_pad_optimize: i32;
    pub static mut xprt_rdma_memreg_strategy: u32;
    pub fn rpcrdma_force_disconnect(ep: *mut rpcrdma_ep);
    pub fn rpcrdma_flush_disconnect(x: *mut rpcrdma_xprt, wc: *mut ib_wc);
    pub fn rpcrdma_xprt_connect(x: *mut rpcrdma_xprt) -> i32;
    pub fn rpcrdma_xprt_disconnect(x: *mut rpcrdma_xprt);
    pub fn rpcrdma_post_recvs(x: *mut rpcrdma_xprt, needed: i32);
    pub fn rpcrdma_req_create(x: *mut rpcrdma_xprt, size: usize) -> *mut rpcrdma_req;
    pub fn rpcrdma_req_setup(x: *mut rpcrdma_xprt, req: *mut rpcrdma_req) -> i32;
    pub fn rpcrdma_req_destroy(req: *mut rpcrdma_req);
    pub fn rpcrdma_buffer_create(x: *mut rpcrdma_xprt) -> i32;
    pub fn rpcrdma_buffer_destroy(b: *mut rpcrdma_buffer);
    pub fn rpcrdma_sendctx_get_locked(x: *mut rpcrdma_xprt) -> *mut rpcrdma_sendctx;
    pub fn rpcrdma_sendctx_unget_locked(x: *mut rpcrdma_xprt, sc: *mut rpcrdma_sendctx);
    pub fn rpcrdma_mr_get(x: *mut rpcrdma_xprt) -> *mut rpcrdma_mr;
    pub fn rpcrdma_mrs_refresh(x: *mut rpcrdma_xprt);
    pub fn rpcrdma_buffer_get(b: *mut rpcrdma_buffer) -> *mut rpcrdma_req;
    pub fn rpcrdma_buffer_put(b: *mut rpcrdma_buffer, req: *mut rpcrdma_req);
    pub fn rpcrdma_rep_put(b: *mut rpcrdma_buffer, rep: *mut rpcrdma_rep);
    pub fn rpcrdma_reply_put(b: *mut rpcrdma_buffer, req: *mut rpcrdma_req);
    pub fn rpcrdma_req_put(req: *mut rpcrdma_req);
    pub fn rpcrdma_regbuf_realloc(rb: *mut rpcrdma_regbuf, size: usize, flags: gfp_t) -> bool;
    pub fn __rpcrdma_regbuf_dma_map(x: *mut rpcrdma_xprt, rb: *mut rpcrdma_regbuf) -> bool;
    pub fn frwr_reset(req: *mut rpcrdma_req);
    pub fn frwr_query_device(ep: *mut rpcrdma_ep, device: *const ib_device) -> i32;
    pub fn frwr_mr_init(x: *mut rpcrdma_xprt, mr: *mut rpcrdma_mr) -> i32;
    pub fn frwr_mr_release(mr: *mut rpcrdma_mr);
    pub fn frwr_map(x: *mut rpcrdma_xprt, cur: *mut rpcrdma_xdr_cursor, writing: bool, xid: __be32, mr: *mut rpcrdma_mr) -> i32;
    pub fn frwr_send(x: *mut rpcrdma_xprt, req: *mut rpcrdma_req) -> i32;
    pub fn frwr_reminv(rep: *mut rpcrdma_rep, mrs: *mut list_head);
    pub fn frwr_unmap_sync(x: *mut rpcrdma_xprt, req: *mut rpcrdma_req);
    pub fn frwr_unmap_async(x: *mut rpcrdma_xprt, req: *mut rpcrdma_req);
    pub fn frwr_wp_create(x: *mut rpcrdma_xprt) -> i32;
    pub fn rpcrdma_prepare_send_sges(x: *mut rpcrdma_xprt, req: *mut rpcrdma_req, hdrlen: u32, xdr: *mut xdr_buf, rtype: rpcrdma_chunktype) -> i32;
    pub fn rpcrdma_sendctx_unmap(sc: *mut rpcrdma_sendctx);
    pub fn rpcrdma_marshal_req(x: *mut rpcrdma_xprt, rqst: *mut rpc_rqst) -> i32;
    pub fn rpcrdma_set_max_header_sizes(ep: *mut rpcrdma_ep);
    pub fn rpcrdma_reset_cwnd(x: *mut rpcrdma_xprt);
    pub fn rpcrdma_complete_rqst(rep: *mut rpcrdma_rep);
    pub fn rpcrdma_unpin_rqst(rep: *mut rpcrdma_rep);
    pub fn rpcrdma_reply_handler(rep: *mut rpcrdma_rep);
    pub static mut xprt_rdma_max_inline_read: u32;
    pub static mut xprt_rdma_max_inline_write: u32;
    pub fn xprt_rdma_format_addresses(x: *mut rpc_xprt, sap: *mut sockaddr);
    pub fn xprt_rdma_free_addresses(x: *mut rpc_xprt);
    pub fn xprt_rdma_close(x: *mut rpc_xprt);
    pub fn xprt_rdma_print_stats(x: *mut rpc_xprt, seq: *mut seq_file);
    pub fn xprt_rdma_init() -> i32;
    pub fn xprt_rdma_cleanup();
    pub static mut xprt_rdma_bc: xprt_class;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
