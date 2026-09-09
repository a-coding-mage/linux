/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause */
/* Direct Rust translation of svc_rdma.h. */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced by their source-level names here.

pub const RPCRDMA_PULLUP_THRESH: u32 = RPCRDMA_V1_DEF_INLINE_SIZE >> 1;
pub const RPCRDMA_DEF_INLINE_THRESH: u32 = 4096;
pub const RPCRDMA_MAX_INLINE_THRESH: u32 = 65536;

extern "C" {
    pub static mut svcrdma_ord: ::core::ffi::c_uint;
    pub static mut svcrdma_max_requests: ::core::ffi::c_uint;
    pub static mut svcrdma_max_bc_requests: ::core::ffi::c_uint;
    pub static mut svcrdma_max_req_size: ::core::ffi::c_uint;
    pub static mut svcrdma_stat_read: percpu_counter;
    pub static mut svcrdma_stat_recv: percpu_counter;
    pub static mut svcrdma_stat_sq_starve: percpu_counter;
    pub static mut svcrdma_stat_write: percpu_counter;
}

#[repr(C)]
pub struct svcxprt_rdma {
    pub sc_xprt: svc_xprt,
    pub sc_cm_id: *mut rdma_cm_id,
    pub sc_accept_q: list_head,
    pub sc_rn: rpcrdma_notification,
    pub sc_ord: u32,
    pub sc_max_send_sges: ::core::ffi::c_uint,
    pub sc_snd_w_inv: bool,
    pub sc_sq_avail: atomic_t,
    pub sc_sq_depth: ::core::ffi::c_uint,
    pub sc_sq_ticket_head: atomic_t,
    pub sc_sq_ticket_tail: atomic_t,
    pub sc_sq_ticket_wait: wait_queue_head_t,
    pub sc_fc_credits: __be32,
    pub sc_max_requests: u32,
    pub sc_max_bc_requests: u32,
    pub sc_max_req_size: ::core::ffi::c_int,
    pub sc_port_num: u8,
    pub sc_pd: *mut ib_pd,
    pub sc_send_lock: spinlock_t,
    pub sc_send_ctxts: llist_head,
    pub sc_rw_ctxt_lock: spinlock_t,
    pub sc_rw_ctxts: llist_head,
    pub sc_pending_recvs: u32,
    pub sc_recv_batch: u32,
    pub sc_rq_dto_q: list_head,
    pub sc_read_complete_q: list_head,
    pub sc_rq_dto_lock: spinlock_t,
    pub sc_qp: *mut ib_qp,
    pub sc_rq_cq: *mut ib_cq,
    pub sc_sq_cq: *mut ib_cq,
    pub sc_lock: spinlock_t,
    pub sc_send_wait: wait_queue_head_t,
    pub sc_flags: ::core::ffi::c_ulong,
    pub sc_work: work_struct,
    pub sc_recv_ctxts: llist_head,
    pub sc_send_release_list: llist_head,
    pub sc_completion_ids: atomic_t,
}

pub const RDMAXPRT_CONN_PENDING: u32 = 3;

#[inline]
pub unsafe fn svc_rdma_rqst_rdma(rqstp: *mut svc_rqst) -> *mut svcxprt_rdma {
    let xprt = (*rqstp).rq_xprt;
    container_of!(xprt, svcxprt_rdma, sc_xprt)
}

pub const RPCRDMA_LISTEN_BACKLOG: u32 = 10;
pub const RPCRDMA_MAX_REQUESTS: u32 = 128;
pub const RPCRDMA_MAX_BC_REQUESTS: u32 = 2;
pub const RPCSVC_MAXPAYLOAD_RDMA: u32 = RPCSVC_MAXPAYLOAD;

#[inline]
pub unsafe fn svc_rdma_recv_cid_init(rdma: *mut svcxprt_rdma, cid: *mut rpc_rdma_cid) {
    (*cid).ci_queue_id = (*(*rdma).sc_rq_cq).res.id;
    (*cid).ci_completion_id = atomic_inc_return(&mut (*rdma).sc_completion_ids);
}

#[inline]
pub unsafe fn svc_rdma_send_cid_init(rdma: *mut svcxprt_rdma, cid: *mut rpc_rdma_cid) {
    (*cid).ci_queue_id = (*(*rdma).sc_sq_cq).res.id;
    (*cid).ci_completion_id = atomic_inc_return(&mut (*rdma).sc_completion_ids);
}

#[repr(C)]
pub struct svc_rdma_chunk_ctxt {
    pub cc_cid: rpc_rdma_cid,
    pub cc_cqe: ib_cqe,
    pub cc_rwctxts: list_head,
    pub cc_posttime: ktime_t,
    pub cc_sqecount: ::core::ffi::c_int,
}

#[repr(C)]
pub struct svc_rdma_recv_ctxt {
    pub rc_node: llist_node,
    pub rc_list: list_head,
    pub rc_recv_wr: ib_recv_wr,
    pub rc_cqe: ib_cqe,
    pub rc_cid: rpc_rdma_cid,
    pub rc_recv_sge: ib_sge,
    pub rc_recv_buf: *mut ::core::ffi::c_void,
    pub rc_stream: xdr_stream,
    pub rc_byte_len: u32,
    pub rc_inv_rkey: u32,
    pub rc_msgtype: __be32,
    pub rc_pageoff: ::core::ffi::c_uint,
    pub rc_curpage: ::core::ffi::c_uint,
    pub rc_readbytes: ::core::ffi::c_uint,
    pub rc_saved_arg: xdr_buf,
    pub rc_cc: svc_rdma_chunk_ctxt,
    pub rc_call_pcl: svc_rdma_pcl,
    pub rc_read_pcl: svc_rdma_pcl,
    pub rc_cur_result_payload: *mut svc_rdma_chunk,
    pub rc_write_pcl: svc_rdma_pcl,
    pub rc_reply_pcl: svc_rdma_pcl,
    pub rc_page_count: ::core::ffi::c_uint,
    pub rc_maxpages: ::core::ffi::c_ulong,
    pub rc_pages: [*mut page; 0],
}

#[repr(C)]
pub struct svc_rdma_write_info {
    pub wi_rdma: *mut svcxprt_rdma,
    pub wi_list: list_head,
    pub wi_chunk: *const svc_rdma_chunk,
    pub wi_seg_off: ::core::ffi::c_uint,
    pub wi_seg_no: ::core::ffi::c_uint,
    pub wi_xdr: *const xdr_buf,
    pub wi_base: *mut u8,
    pub wi_next_off: ::core::ffi::c_uint,
    pub wi_cc: svc_rdma_chunk_ctxt,
}

#[repr(C)]
pub struct svc_rdma_send_ctxt {
    pub sc_node: llist_node,
    pub sc_cid: rpc_rdma_cid,
    pub sc_rdma: *mut svcxprt_rdma,
    pub sc_send_wr: ib_send_wr,
    pub sc_wr_chain: *mut ib_send_wr,
    pub sc_sqecount: ::core::ffi::c_int,
    pub sc_cqe: ib_cqe,
    pub sc_hdrbuf: xdr_buf,
    pub sc_stream: xdr_stream,
    pub sc_write_info_list: list_head,
    pub sc_reply_info: svc_rdma_write_info,
    pub sc_xprt_buf: *mut ::core::ffi::c_void,
    pub sc_page_count: ::core::ffi::c_int,
    pub sc_cur_sge_no: ::core::ffi::c_int,
    pub sc_maxpages: ::core::ffi::c_ulong,
    pub sc_pages: *mut *mut page,
    pub sc_sges: [ib_sge; 0],
}

extern "C" {
    pub fn svc_rdma_handle_bc_reply(rqstp: *mut svc_rqst, rctxt: *mut svc_rdma_recv_ctxt);
    pub fn svc_rdma_recv_ctxts_destroy(rdma: *mut svcxprt_rdma);
    pub fn svc_rdma_post_recvs(rdma: *mut svcxprt_rdma) -> bool;
    pub fn svc_rdma_recv_ctxt_get(rdma: *mut svcxprt_rdma) -> *mut svc_rdma_recv_ctxt;
    pub fn svc_rdma_recv_ctxt_put(rdma: *mut svcxprt_rdma, ctxt: *mut svc_rdma_recv_ctxt);
    pub fn svc_rdma_flush_recv_queues(rdma: *mut svcxprt_rdma);
    pub fn svc_rdma_release_ctxt(xprt: *mut svc_xprt, ctxt: *mut ::core::ffi::c_void);
    pub fn svc_rdma_recvfrom(rqstp: *mut svc_rqst) -> ::core::ffi::c_int;
    pub fn svc_rdma_cc_init(rdma: *mut svcxprt_rdma, cc: *mut svc_rdma_chunk_ctxt);
    pub fn svc_rdma_destroy_rw_ctxts(rdma: *mut svcxprt_rdma);
    pub fn svc_rdma_cc_release(rdma: *mut svcxprt_rdma, cc: *mut svc_rdma_chunk_ctxt, dir: dma_data_direction);
    pub fn svc_rdma_write_chunk_release(rdma: *mut svcxprt_rdma, ctxt: *mut svc_rdma_send_ctxt);
    pub fn svc_rdma_reply_chunk_release(rdma: *mut svcxprt_rdma, ctxt: *mut svc_rdma_send_ctxt);
    pub fn svc_rdma_prepare_write_list(rdma: *mut svcxprt_rdma, rctxt: *const svc_rdma_recv_ctxt, sctxt: *mut svc_rdma_send_ctxt, xdr: *const xdr_buf) -> ::core::ffi::c_int;
    pub fn svc_rdma_prepare_reply_chunk(rdma: *mut svcxprt_rdma, write_pcl: *const svc_rdma_pcl, reply_pcl: *const svc_rdma_pcl, sctxt: *mut svc_rdma_send_ctxt, xdr: *const xdr_buf) -> ::core::ffi::c_int;
    pub fn svc_rdma_process_read_list(rdma: *mut svcxprt_rdma, rqstp: *mut svc_rqst, head: *mut svc_rdma_recv_ctxt) -> ::core::ffi::c_int;
    pub fn svc_rdma_send_ctxts_destroy(rdma: *mut svcxprt_rdma);
    pub fn svc_rdma_send_ctxts_drain(rdma: *mut svcxprt_rdma);
    pub fn svc_rdma_send_ctxt_get(rdma: *mut svcxprt_rdma) -> *mut svc_rdma_send_ctxt;
    pub fn svc_rdma_send_ctxt_put(rdma: *mut svcxprt_rdma, ctxt: *mut svc_rdma_send_ctxt);
    pub fn svc_rdma_post_send(rdma: *mut svcxprt_rdma, ctxt: *mut svc_rdma_send_ctxt) -> ::core::ffi::c_int;
    pub fn svc_rdma_map_reply_msg(rdma: *mut svcxprt_rdma, sctxt: *mut svc_rdma_send_ctxt, write_pcl: *const svc_rdma_pcl, reply_pcl: *const svc_rdma_pcl, xdr: *const xdr_buf) -> ::core::ffi::c_int;
    pub fn svc_rdma_send_error_msg(rdma: *mut svcxprt_rdma, sctxt: *mut svc_rdma_send_ctxt, rctxt: *mut svc_rdma_recv_ctxt, status: ::core::ffi::c_int);
    pub fn svc_rdma_wake_send_waiters(rdma: *mut svcxprt_rdma, avail: ::core::ffi::c_int);
    pub fn svc_rdma_sq_wait(rdma: *mut svcxprt_rdma, cid: *const rpc_rdma_cid, sqecount: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn svc_rdma_post_send_err(rdma: *mut svcxprt_rdma, cid: *const rpc_rdma_cid, bad_wr: *const ib_send_wr, first_wr: *const ib_send_wr, sqecount: ::core::ffi::c_int, ret: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn svc_rdma_sendto(rqstp: *mut svc_rqst) -> ::core::ffi::c_int;
    pub fn svc_rdma_result_payload(rqstp: *mut svc_rqst, offset: ::core::ffi::c_uint, length: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn svc_rdma_xprt_deferred_close(rdma: *mut svcxprt_rdma);
    pub static mut svc_rdma_class: svc_xprt_class;
    pub fn svc_rdma_init() -> ::core::ffi::c_int;
    pub fn svc_rdma_cleanup();
}

#[cfg(CONFIG_SUNRPC_BACKCHANNEL)]
extern "C" {
    pub static mut svc_rdma_bc_class: svc_xprt_class;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
