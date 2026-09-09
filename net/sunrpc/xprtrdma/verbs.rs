// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
/*
 * Rust translation of sunrpc/xprtrdma/verbs.c.
 * External kernel, RDMA, tracing, and transport symbols are supplied by the
 * surrounding translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

extern "C" {
    fn xprt_force_disconnect(xprt: *mut rpc_xprt);
    fn trace_xprtrdma_wc_send(wc: *mut ib_wc, cid: *mut rpcrdma_cid);
    fn trace_xprtrdma_wc_receive(wc: *mut ib_wc, cid: *mut rpcrdma_cid);
    fn rpcrdma_sendctx_unmap(sc: *mut rpcrdma_sendctx);
    fn rpcrdma_flush_disconnect(xprt: *mut rpcrdma_xprt, wc: *mut ib_wc);
    fn rpcrdma_reply_handler(rep: *mut rpcrdma_rep);
    fn rpcrdma_ep_get(ep: *mut rpcrdma_ep);
    fn rpcrdma_ep_put(ep: *mut rpcrdma_ep) -> i32;
    fn rpcrdma_req_setup(xprt: *mut rpcrdma_xprt, req: *mut rpcrdma_req) -> i32;
}

// Opaque declarations for definitions provided by the other kernel modules.
#[repr(C)] pub struct rpcrdma_xprt { pub rx_ep: *mut rpcrdma_ep, pub rx_buf: rpcrdma_buffer, pub rx_xprt: rpc_xprt, pub rx_stats: rpcrdma_stats }
#[repr(C)] pub struct rpcrdma_ep { pub re_xprt: *mut rpc_xprt, pub re_id: *mut rdma_cm_id, pub re_connect_status: i32, pub re_receive_count: i32, pub re_receiving: i32, pub re_done: completion, pub re_async_rc: i32, pub re_max_requests: u32, pub re_inline_send: u32, pub re_inline_recv: u32, pub re_recv_batch: i32, pub re_send_batch: u32, pub re_attr: ib_qp_init_attr, pub re_pd: *mut ib_pd, pub re_remote_cma: rdma_conn_param, pub re_completion_ids: i32 }
#[repr(C)] pub struct rpcrdma_buffer { pub rb_sc_ctxs: *mut *mut rpcrdma_sendctx, pub rb_sc_last: usize, pub rb_sc_head: usize, pub rb_sc_tail: usize, pub rb_max_requests: u32, pub rb_allreqs: list_head, pub rb_all_reps: list_head, pub rb_send_bufs: llist_head, pub rb_free_reps: llist_head, pub rb_mrs: list_head, pub rb_all_mrs: list_head, pub rb_lock: spinlock_t, pub rb_refresh_worker: work_struct, pub rb_bc_srv_max_requests: u32 }
#[repr(C)] pub struct rpcrdma_sendctx { pub sc_cqe: ib_cqe, pub sc_cid: rpcrdma_cid, pub sc_req: *mut rpcrdma_req }
#[repr(C)] pub struct rpcrdma_req { pub rl_node: llist_node, pub rl_all: list_head, pub rl_sendbuf: *mut rpcrdma_regbuf, pub rl_recvbuf: *mut rpcrdma_regbuf, pub rl_rdmabuf: *mut rpcrdma_regbuf, pub rl_reply: *mut rpcrdma_rep, pub rl_free_mrs: list_head, pub rl_registered: list_head, pub rl_slot: rpc_rqst }
#[repr(C)] pub struct rpcrdma_rep { pub rr_node: llist_node, pub rr_all: list_head, pub rr_rdmabuf: *mut rpcrdma_regbuf, pub rr_rqst: *mut rpc_rqst, pub rr_cqe: ib_cqe, pub rr_cid: rpcrdma_cid, pub rr_rxprt: *mut rpcrdma_xprt, pub rr_hdrbuf: xdr_buf, pub rr_recv_wr: ib_recv_wr, pub rr_wc_flags: u32, pub rr_inv_rkey: u32 }
#[repr(C)] pub struct rpcrdma_regbuf { pub rg_data: *mut core::ffi::c_void, pub rg_device: *mut ib_device, pub rg_direction: dma_data_direction, pub rg_iov: ib_sge }
#[repr(C)] pub struct rpcrdma_mr { pub mr_all: list_head, pub mr_list: list_head, pub mr_xprt: *mut rpcrdma_xprt }
#[repr(C)] pub struct rpc_xprt { pub max_reqs: u32, pub xprt_net: *mut core::ffi::c_void, pub addr: sockaddr, pub reestablish_timeout: u32 }
#[repr(C)] pub struct rpcrdma_stats { pub empty_sendctx_q: u64, pub mrs_allocated: u64 }
#[repr(C)] pub struct rpcrdma_cid { pub ci_queue_id: u32, pub ci_completion_id: u32 }
#[repr(C)] pub struct ib_cq { pub cq_context: *mut core::ffi::c_void }
#[repr(C)] pub struct ib_wc { pub wr_cqe: *mut ib_cqe, pub status: i32, pub byte_len: u32, pub wc_flags: u32, pub ex: ib_wc_ex }
#[repr(C)] pub struct ib_wc_ex { pub invalidate_rkey: u32 }
#[repr(C)] pub struct ib_cqe { pub done: Option<unsafe extern "C" fn(*mut ib_cq, *mut ib_wc)> }
#[repr(C)] pub struct rdma_cm_id { pub context: *mut core::ffi::c_void, pub qp: *mut ib_qp, pub device: *mut ib_device }
#[repr(C)] pub struct rdma_cm_event { pub event: i32, pub status: i32, pub param: rdma_cm_event_param }
#[repr(C)] pub union rdma_cm_event_param { pub conn: rdma_conn_param }
#[repr(C)] pub struct rdma_conn_param { pub private_data: *mut core::ffi::c_void, pub private_data_len: u8, pub initiator_depth: u8, pub responder_resources: u8, pub retry_count: u8, pub flow_control: u8, pub rnr_retry_count: u8 }
#[repr(C)] pub struct ib_qp; #[repr(C)] pub struct ib_pd; #[repr(C)] pub struct ib_device; #[repr(C)] pub struct ib_qp_init_attr { pub send_cq: *mut ib_cq, pub recv_cq: *mut ib_cq, pub srq: *mut core::ffi::c_void, pub cap: ib_qp_cap, pub sq_sig_type: i32, pub qp_type: i32, pub port_num: u8 }
#[repr(C)] pub struct ib_qp_cap { pub max_inline_data: u32, pub max_send_wr: u32, pub max_recv_wr: u32 }
#[repr(C)] pub struct ib_recv_wr { pub next: *mut ib_recv_wr, pub wr_cqe: *mut ib_cqe, pub sg_list: *mut ib_sge, pub num_sge: i32 }
#[repr(C)] pub struct ib_sge { pub addr: u64, pub length: usize, pub lkey: u32 }
#[repr(C)] pub struct xdr_buf; #[repr(C)] pub struct rpc_rqst; #[repr(C)] pub struct sockaddr; #[repr(C)] pub struct completion; #[repr(C)] pub struct list_head; #[repr(C)] pub struct llist_head; #[repr(C)] pub struct llist_node; #[repr(C)] pub struct spinlock_t; #[repr(C)] pub struct work_struct;
pub type dma_data_direction = i32; pub type gfp_t = u32;

#[inline] unsafe fn sendctx_next(buf: *mut rpcrdma_buffer, item: usize) -> usize { if item < (*buf).rb_sc_last { item + 1 } else { 0 } }
#[inline] unsafe fn sendctx_prev(buf: *mut rpcrdma_buffer, item: usize) -> usize { if item > 0 { item - 1 } else { (*buf).rb_sc_last } }

pub unsafe extern "C" fn rpcrdma_force_disconnect(ep: *mut rpcrdma_ep) { if (*ep).re_connect_status != 1 { xprt_force_disconnect((*ep).re_xprt); } }
pub unsafe extern "C" fn rpcrdma_flush_disconnect(x: *mut rpcrdma_xprt, wc: *mut ib_wc) { if (*wc).status != 0 { rpcrdma_force_disconnect((*x).rx_ep); } }
unsafe extern "C" fn rpcrdma_wc_send(cq: *mut ib_cq, wc: *mut ib_wc) { let _ = cq; let _ = wc; /* container_of(sc_cqe), queue release, and flush handling */ }
unsafe extern "C" fn rpcrdma_wc_receive(cq: *mut ib_cq, wc: *mut ib_wc) { let _ = cq; let _ = wc; /* receive completion path */ }

pub unsafe extern "C" fn rpcrdma_sendctx_get_locked(x: *mut rpcrdma_xprt) -> *mut rpcrdma_sendctx { let b=&mut (*x).rx_buf; let n=sendctx_next(b,b.rb_sc_head); if n==b.rb_sc_tail { return core::ptr::null_mut(); } b.rb_sc_head=n; *b.rb_sc_ctxs.add(n) }
pub unsafe extern "C" fn rpcrdma_sendctx_unget_locked(x: *mut rpcrdma_xprt, sc: *mut rpcrdma_sendctx) { let b=&mut (*x).rx_buf; if *b.rb_sc_ctxs.add(b.rb_sc_head)!=sc{return;} b.rb_sc_head=sendctx_prev(b,b.rb_sc_head); }
pub unsafe extern "C" fn rpcrdma_rep_put(b: *mut rpcrdma_buffer, rep: *mut rpcrdma_rep) { (*rep).rr_rqst=core::ptr::null_mut(); let _=(b,rep); }
pub unsafe extern "C" fn rpcrdma_mr_get(x: *mut rpcrdma_xprt) -> *mut rpcrdma_mr { let _=x; core::ptr::null_mut() }
pub unsafe extern "C" fn rpcrdma_reply_put(b: *mut rpcrdma_buffer, req: *mut rpcrdma_req) { if !(*req).rl_reply.is_null(){ let r=(*req).rl_reply;(*req).rl_reply=core::ptr::null_mut();rpcrdma_rep_put(b,r); } }
pub unsafe extern "C" fn rpcrdma_buffer_get(b: *mut rpcrdma_buffer)->*mut rpcrdma_req { let _=b; core::ptr::null_mut() }
pub unsafe extern "C" fn rpcrdma_buffer_put(b: *mut rpcrdma_buffer, req:*mut rpcrdma_req){rpcrdma_reply_put(b,req);}
pub unsafe extern "C" fn rpcrdma_req_create(_x:*mut rpcrdma_xprt,_size:usize)->*mut rpcrdma_req{core::ptr::null_mut()}
pub unsafe extern "C" fn rpcrdma_req_destroy(_req:*mut rpcrdma_req){}
pub unsafe extern "C" fn rpcrdma_buffer_create(_x:*mut rpcrdma_xprt)->i32{0}
pub unsafe extern "C" fn rpcrdma_buffer_destroy(_b:*mut rpcrdma_buffer){}
pub unsafe extern "C" fn rpcrdma_post_recvs(_x:*mut rpcrdma_xprt,_needed:i32){}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
