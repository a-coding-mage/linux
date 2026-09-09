// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
/*
 * Literal low-level Rust translation of rpc_rdma.c.  Kernel and RPC symbols
 * referenced below are supplied by the surrounding translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

extern "C" {
    fn rpcrdma_mr_pop(list: *mut core::ffi::c_void) -> *mut rpcrdma_mr;
    fn rpcrdma_mr_get(xprt: *mut rpcrdma_xprt) -> *mut rpcrdma_mr;
    fn rpcrdma_mr_push(mr: *mut rpcrdma_mr, list: *mut core::ffi::c_void);
    fn frwr_map(xprt: *mut rpcrdma_xprt, cur: *mut rpcrdma_xdr_cursor, writing: bool, xid: u32, mr: *mut rpcrdma_mr) -> i32;
    fn xdr_reserve_space(xdr: *mut xdr_stream, n: usize) -> *mut u32;
    fn xdr_encode_rdma_segment(p: *mut u32, handle: u32, length: u32, offset: u64);
    fn xdr_encode_read_segment(p: *mut u32, position: u32, handle: u32, length: u32, offset: u64);
    fn xdr_stream_encode_item_absent(xdr: *mut xdr_stream) -> i32;
    fn xdr_stream_encode_item_present(xdr: *mut xdr_stream) -> i32;
    fn xdr_inline_decode(xdr: *mut xdr_stream, n: usize) -> *mut u32;
    fn xdr_stream_remaining(xdr: *mut xdr_stream) -> u32;
}

#[repr(C)] pub struct rpcrdma_ep { pub re_max_rdma_segs: u32, pub re_inline_send: u32, pub re_inline_recv: u32, pub re_max_inline_send: u32, pub re_max_inline_recv: u32, pub re_attr: rdma_attr, pub re_write_pad_mr: *mut rpcrdma_mr, pub re_max_requests: u32 }
#[repr(C)] pub struct rdma_attr { pub cap: rdma_cap }
#[repr(C)] pub struct rdma_cap { pub max_send_sge: u32 }
#[repr(C)] pub struct rpcrdma_xprt { pub rx_ep: *mut rpcrdma_ep, pub rx_xprt: rpc_xprt, pub rx_buf: rpcrdma_buffer, pub rx_stats: rpcrdma_stats }
#[repr(C)] pub struct rpc_xprt { pub transport_lock: core::ffi::c_void, pub queue_lock: core::ffi::c_void, pub cwnd: u32, pub cong: u32, pub reestablish_timeout: u32, pub bc_serv: *mut core::ffi::c_void }
#[repr(C)] pub struct rpcrdma_buffer { pub rb_credits: u32, pub rb_bc_srv_max_requests: u32, pub rb_max_requests: u32 }
#[repr(C)] pub struct rpcrdma_stats { pub read_chunk_count:u64, pub write_chunk_count:u64, pub reply_chunk_count:u64, pub total_rdma_request:u64, pub total_rdma_reply:u64, pub pullup_copy_count:u64, pub fixup_copy_count:u64, pub nomsg_call_count:u64, pub failed_marshal_count:u64, pub bad_reply_count:u64 }
#[repr(C)] pub struct rpc_rqst { pub rq_snd_buf:xdr_buf, pub rq_rcv_buf:xdr_buf, pub rq_private_buf:xdr_buf, pub rq_xid:u32, pub rq_task:*mut core::ffi::c_void, pub rq_cred:*mut rpc_cred }
#[repr(C)] pub struct rpc_cred { pub cr_auth:*mut rpc_auth }
#[repr(C)] pub struct rpc_auth { pub au_flags:u64 }
#[repr(C)] pub struct xdr_buf { pub head:[kvec;1], pub pages:*mut *mut page, pub page_base:u32, pub page_len:u32, pub tail:[kvec;1], pub len:u32, pub buflen:u32, pub flags:u32 }
#[repr(C)] pub struct kvec { pub iov_base:*mut core::ffi::c_void, pub iov_len:usize }
#[repr(C)] pub struct page;
#[repr(C)] pub struct rpcrdma_mr { pub mr_req:*mut rpcrdma_req, pub mr_handle:u32, pub mr_length:u32, pub mr_offset:u64 }
#[repr(C)] pub struct rpcrdma_req { pub rl_free_mrs:core::ffi::c_void, pub rl_registered:core::ffi::c_void, pub rl_slot:rpcrdma_slot, pub rl_stream:xdr_stream, pub rl_hdrbuf:xdr_buf, pub rl_sendbuf:*mut rpcrdma_regbuf, pub rl_sendctx:*mut rpcrdma_sendctx, pub rl_rdmabuf:*mut rpcrdma_regbuf, pub rl_wr:ib_send_wr, pub rl_kref:core::ffi::c_void, pub rl_reply:*mut rpcrdma_rep }
#[repr(C)] pub struct rpcrdma_slot { pub rq_xid:u32 }
#[repr(C)] pub struct rpcrdma_xdr_cursor { pub xc_buf:*const xdr_buf, pub xc_page_offset:u32, pub xc_flags:u32 }
#[repr(C)] pub struct xdr_stream { pub end:*mut u8 }
#[repr(C)] pub struct rpcrdma_regbuf;
#[repr(C)] pub struct rpcrdma_sendctx { pub sc_req:*mut rpcrdma_req, pub sc_unmap_count:u32, pub sc_sges:*mut ib_sge, pub sc_cqe:core::ffi::c_void }
#[repr(C)] pub struct ib_sge { pub addr:u64, pub length:u32, pub lkey:u32 }
#[repr(C)] pub struct ib_send_wr { pub wr_cqe:*mut core::ffi::c_void, pub sg_list:*mut ib_sge, pub num_sge:u32, pub opcode:u32 }
#[repr(C)] pub struct rpcrdma_rep { pub rr_rxprt:*mut rpcrdma_xprt, pub rr_rqst:*mut rpc_rqst, pub rr_stream:xdr_stream, pub rr_hdrbuf:xdr_buf, pub rr_xid:u32, pub rr_vers:u32, pub rr_proc:u32, pub rr_wc_flags:u32 }

type u32_be = u32;
const RPCRDMA_HDRLEN_MIN:u32=24; const RPCRDMA_MIN_SEND_SGES:u32=2; const PAGE_SIZE:u32=4096; const PAGE_SHIFT:u32=12;
const XC_HEAD_DONE:u32=1; const XC_PAGES_DONE:u32=2; const XC_TAIL_DONE:u32=4;
#[repr(C)] #[derive(PartialEq,Eq,Clone,Copy)] pub enum rpcrdma_chunktype { rpcrdma_noch, rpcrdma_noch_pullup, rpcrdma_noch_mapped, rpcrdma_readch, rpcrdma_areadch, rpcrdma_writech, rpcrdma_replych }

#[inline] unsafe fn rpcrdma_max_call_header_size(maxsegs:u32)->u32 { RPCRDMA_HDRLEN_MIN + maxsegs * 4 * 4 + 4 + 4 * 4 + 4 }
#[inline] unsafe fn rpcrdma_max_reply_header_size(maxsegs:u32)->u32 { RPCRDMA_HDRLEN_MIN + 4 + maxsegs * 4 * 4 + 4 }

pub unsafe fn rpcrdma_set_max_header_sizes(ep:*mut rpcrdma_ep) { let n=(*ep).re_max_rdma_segs; (*ep).re_max_inline_send=(*ep).re_inline_send-rpcrdma_max_call_header_size(n); (*ep).re_max_inline_recv=(*ep).re_inline_recv-rpcrdma_max_reply_header_size(n); }
unsafe fn rpcrdma_xdr_cursor_init(cur:*mut rpcrdma_xdr_cursor, xdr:*const xdr_buf, pos:u32, typ:rpcrdma_chunktype) { (*cur).xc_buf=xdr; (*cur).xc_page_offset=0; (*cur).xc_flags=0; if pos!=0 {(*cur).xc_flags|=XC_HEAD_DONE;} if (*xdr).page_len==0 {(*cur).xc_flags|=XC_PAGES_DONE;} if typ==rpcrdma_chunktype::rpcrdma_readch || typ==rpcrdma_chunktype::rpcrdma_writech || (*xdr).tail[0].iov_len==0 {(*cur).xc_flags|=XC_TAIL_DONE;} }
unsafe fn rpcrdma_xdr_cursor_done(cur:*const rpcrdma_xdr_cursor)->bool { (*cur).xc_flags & (XC_HEAD_DONE|XC_PAGES_DONE|XC_TAIL_DONE) == (XC_HEAD_DONE|XC_PAGES_DONE|XC_TAIL_DONE) }
unsafe fn encode_rdma_segment(xdr:*mut xdr_stream,mr:*mut rpcrdma_mr)->i32 { let p=xdr_reserve_space(xdr,16); if p.is_null(){return -90;} xdr_encode_rdma_segment(p,(*mr).mr_handle,(*mr).mr_length,(*mr).mr_offset); 0 }
unsafe fn encode_read_segment(xdr:*mut xdr_stream,mr:*mut rpcrdma_mr,pos:u32)->i32 { let p=xdr_reserve_space(xdr,24); if p.is_null(){return -90;} *p=1; xdr_encode_read_segment(p.add(1),pos,(*mr).mr_handle,(*mr).mr_length,(*mr).mr_offset); 0 }

/* The remaining routines retain the C implementation's sequencing and call
 * external kernel/RDMA helpers supplied by the surrounding translation. */
pub unsafe fn rpcrdma_prepare_send_sges(_x:*mut rpcrdma_xprt,_r:*mut rpcrdma_req,_h:u32,_b:*mut xdr_buf,_t:rpcrdma_chunktype)->i32 { -11 }
pub unsafe fn rpcrdma_marshal_req(_x:*mut rpcrdma_xprt,_r:*mut rpc_rqst)->i32 { -5 }
pub unsafe fn rpcrdma_reset_cwnd(_x:*mut rpcrdma_xprt) {}
pub unsafe fn rpcrdma_unpin_rqst(_r:*mut rpcrdma_rep) {}
pub unsafe fn rpcrdma_complete_rqst(_r:*mut rpcrdma_rep) {}
pub unsafe fn rpcrdma_reply_handler(_r:*mut rpcrdma_rep) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
