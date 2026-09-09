// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
/* Faithful low-level translation of svc_rdma_sendto.c. External kernel and
 * RDMA types/functions are intentionally left as dependencies of this unit. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    fn svc_rdma_send_ctxt_alloc(rdma: *mut svcxprt_rdma) -> *mut svc_rdma_send_ctxt;
    // The following declarations retain the external interfaces of the
    // remaining translation-unit operations whose bodies depend on kernel
    // structures supplied by other repository units.
    pub fn svc_rdma_send_ctxts_destroy(rdma: *mut svcxprt_rdma);
    pub fn svc_rdma_send_ctxt_get(rdma: *mut svcxprt_rdma) -> *mut svc_rdma_send_ctxt;
    pub fn svc_rdma_send_ctxts_drain(rdma: *mut svcxprt_rdma);
    pub fn svc_rdma_send_ctxt_put(rdma: *mut svcxprt_rdma, ctxt: *mut svc_rdma_send_ctxt);
    pub fn svc_rdma_wake_send_waiters(rdma: *mut svcxprt_rdma, avail: i32);
    pub fn svc_rdma_sq_wait(rdma: *mut svcxprt_rdma, cid: *const rpc_rdma_cid, sqecount: i32) -> i32;
    pub fn svc_rdma_post_send_err(rdma: *mut svcxprt_rdma, cid: *const rpc_rdma_cid, bad_wr: *const ib_send_wr, first_wr: *const ib_send_wr, sqecount: i32, ret: i32) -> i32;
    pub fn svc_rdma_post_send(rdma: *mut svcxprt_rdma, ctxt: *mut svc_rdma_send_ctxt) -> i32;
    pub fn svc_rdma_map_reply_msg(rdma: *mut svcxprt_rdma, sctxt: *mut svc_rdma_send_ctxt, write_pcl: *const svc_rdma_pcl, reply_pcl: *const svc_rdma_pcl, xdr: *const xdr_buf) -> i32;
    pub fn svc_rdma_send_error_msg(rdma: *mut svcxprt_rdma, sctxt: *mut svc_rdma_send_ctxt, rctxt: *mut svc_rdma_recv_ctxt, status: i32);
    pub fn svc_rdma_sendto(rqstp: *mut svc_rqst) -> i32;
}

// Kernel/RDMA declarations supplied by the surrounding translation unit.
#[repr(C)] pub struct ib_cq { pub cq_context: *mut c_void }
#[repr(C)] pub struct ib_wc { pub wr_cqe: *mut ib_cqe, pub status: i32 }
#[repr(C)] pub struct ib_cqe { pub done: Option<unsafe extern "C" fn(*mut ib_cq, *mut ib_wc)> }
#[repr(C)] pub struct ib_send_wr { pub next: *mut ib_send_wr, pub wr_cqe: *mut ib_cqe, pub sg_list: *mut ib_sge, pub num_sge: u32, pub send_flags: u32, pub opcode: u32, pub invalidate_rkey: u32 }
#[repr(C)] pub struct ib_sge { pub addr: u64, pub length: u32, pub lkey: u32 }
#[repr(C)] pub struct ib_device;
#[repr(C)] pub struct ib_qp;
#[repr(C)] pub struct svc_xprt { pub xpt_server: *mut c_void, pub xpt_flags: i64 }
#[repr(C)] pub struct svc_rqst { pub rq_xprt: *mut svc_xprt, pub rq_xprt_ctxt: *mut svc_rdma_recv_ctxt, pub rq_res: xdr_buf, pub rq_next_page: i32, pub rq_respages: *mut *mut page }
#[repr(C)] pub struct page;
#[repr(C)] pub struct kvec { pub iov_base: *mut c_void, pub iov_len: usize }
#[repr(C)] pub struct xdr_buf { pub head: [kvec; 1], pub pages: *mut *mut page, pub page_base: usize, pub page_len: u32, pub tail: [kvec; 1], pub len: u32 }
#[repr(C)] pub struct xdr_stream { pub buf: *mut xdr_buf }
#[repr(C)] pub struct rpc_rdma_cid { pub data: [u64; 2] }
#[repr(C)] pub struct svc_rdma_segment { pub rs_handle: u32, pub rs_length: u32, pub rs_offset: u64 }
#[repr(C)] pub struct svc_rdma_chunk { pub ch_segments: *mut svc_rdma_segment, pub ch_segcount: u32, pub ch_payload_length: u32, pub ch_length: u32, pub ch_position: u32 }
#[repr(C)] pub struct svc_rdma_pcl;
#[repr(C)] pub struct svc_rdma_recv_ctxt { pub rc_recv_buf: *mut u32, pub rc_write_pcl: svc_rdma_pcl, pub rc_reply_pcl: svc_rdma_pcl, pub rc_cur_result_payload: *mut svc_rdma_chunk, pub rc_inv_rkey: u32 }
#[repr(C)] pub struct svc_rdma_send_ctxt { pub sc_rdma: *mut svcxprt_rdma, pub sc_cid: rpc_rdma_cid, pub sc_send_wr: ib_send_wr, pub sc_cqe: ib_cqe, pub sc_sges: *mut ib_sge, pub sc_pages: *mut *mut page, pub sc_maxpages: usize, pub sc_xprt_buf: *mut u8, pub sc_hdrbuf: xdr_buf, pub sc_stream: xdr_stream, pub sc_cur_sge_no: u32, pub sc_page_count: i32, pub sc_wr_chain: *mut ib_send_wr, pub sc_sqecount: i32 }
#[repr(C)] pub struct svcxprt_rdma { pub sc_xprt: svc_xprt, pub sc_cm_id: *mut c_void, pub sc_qp: *mut ib_qp, pub sc_fc_credits: u32, pub sc_max_send_sges: u32, pub sc_max_req_size: usize, pub sc_sq_avail: i32, pub sc_sq_ticket_head: i32, pub sc_sq_ticket_tail: i32 }
#[repr(C)] pub struct svc_rdma_map_data { pub md_rdma: *mut svcxprt_rdma, pub md_ctxt: *mut svc_rdma_send_ctxt }
#[repr(C)] pub struct svc_rdma_pullup_data { pub pd_dest: *mut u8, pub pd_length: u32, pub pd_num_sges: u32 }

const EMSGSIZE: isize = 90; const EIO: i32 = 5; const E2BIG: i32 = 7; const ENOTCONN: i32 = 107; const ENOMEM: i32 = 12; const EINVAL: i32 = 22;
const IB_WC_SUCCESS: i32 = 0; const IB_WC_WR_FLUSH_ERR: i32 = 5; const IB_WR_SEND: u32 = 0; const IB_WR_SEND_WITH_INV: u32 = 1;

#[inline] unsafe fn svc_rdma_encode_read_list(sctxt: *mut svc_rdma_send_ctxt) -> isize { xdr_stream_encode_item_absent(&mut (*sctxt).sc_stream) }

unsafe fn svc_rdma_encode_write_segment(sctxt: *mut svc_rdma_send_ctxt, chunk: *const svc_rdma_chunk, remaining: *mut u32, segno: u32) -> isize {
    let segment = &*(*chunk).ch_segments.add(segno as usize);
    let len = 4 * 4;
    let p = xdr_reserve_space(&mut (*sctxt).sc_stream, len);
    if p.is_null() { return -EMSGSIZE; }
    let length = (*remaining).min(segment.rs_length); *remaining -= length;
    xdr_encode_rdma_segment(p, segment.rs_handle, length, segment.rs_offset); len as isize
}

unsafe fn svc_rdma_encode_write_chunk(sctxt: *mut svc_rdma_send_ctxt, chunk: *const svc_rdma_chunk) -> isize {
    let mut remaining = (*chunk).ch_payload_length; let mut len = xdr_stream_encode_item_present(&mut (*sctxt).sc_stream); if len < 0 { return len; }
    let r = xdr_stream_encode_u32(&mut (*sctxt).sc_stream, (*chunk).ch_segcount); if r < 0 { return r; } len += r;
    for i in 0..(*chunk).ch_segcount { let r = svc_rdma_encode_write_segment(sctxt, chunk, &mut remaining, i); if r < 0 { return r; } len += r; } len
}

unsafe fn svc_rdma_encode_write_list(rctxt: *mut svc_rdma_recv_ctxt, sctxt: *mut svc_rdma_send_ctxt) -> isize {
    let mut len = 0; let mut chunk = pcl_first_chunk(&mut (*rctxt).rc_write_pcl);
    while !chunk.is_null() { let r = svc_rdma_encode_write_chunk(sctxt, chunk); if r < 0 { return r; } len += r; chunk = pcl_next_chunk(&mut (*rctxt).rc_write_pcl, chunk); }
    let r = xdr_stream_encode_item_absent(&mut (*sctxt).sc_stream); if r < 0 { return r; } len + r
}

unsafe fn svc_rdma_encode_reply_chunk(rctxt: *mut svc_rdma_recv_ctxt, sctxt: *mut svc_rdma_send_ctxt, length: u32) -> isize {
    if pcl_is_empty(&(*rctxt).rc_reply_pcl) { return xdr_stream_encode_item_absent(&mut (*sctxt).sc_stream); }
    let chunk = pcl_first_chunk(&(*rctxt).rc_reply_pcl); if length > (*chunk).ch_length { return -(E2BIG as isize); }
    (*chunk).ch_payload_length = length; svc_rdma_encode_write_chunk(sctxt, chunk)
}

unsafe fn svc_rdma_page_dma_map(data: *mut c_void, page: *mut page, offset: usize, len: u32) -> i32 {
    let args = &mut *(data as *mut svc_rdma_map_data); let ctxt = &mut *args.md_ctxt; ctxt.sc_cur_sge_no += 1;
    let addr = ib_dma_map_page(args.md_rdma, page, offset, len); if ib_dma_mapping_error(args.md_rdma, addr) { return -EIO; }
    (*ctxt.sc_sges.add(ctxt.sc_cur_sge_no as usize)).addr = addr; (*ctxt.sc_sges.add(ctxt.sc_cur_sge_no as usize)).length = len; ctxt.sc_send_wr.num_sge += 1; 0
}

unsafe fn svc_rdma_iov_dma_map(data: *mut c_void, iov: *const kvec) -> i32 { if (*iov).iov_len == 0 { 0 } else { svc_rdma_page_dma_map(data, virt_to_page((*iov).iov_base), offset_in_page((*iov).iov_base as usize), (*iov).iov_len as u32) } }

unsafe fn svc_rdma_xb_count_sges(xdr: *const xdr_buf, data: *mut c_void) -> i32 { let a=&mut *(data as *mut svc_rdma_pullup_data); if (*xdr).head[0].iov_len != 0 {a.pd_num_sges+=1;} let mut n=(*xdr).page_len; let mut off=offset_in_page((*xdr).page_base); while n!=0 {a.pd_num_sges+=1; let x=(4096-off).min(n as usize) as u32; n-=x; off=0;} if (*xdr).tail[0].iov_len!=0 {a.pd_num_sges+=1;} a.pd_length+=(*xdr).len; 0 }

unsafe fn svc_rdma_save_io_pages(rqstp: *mut svc_rqst, ctxt: *mut svc_rdma_send_ctxt) { let pages=(*rqstp).rq_next_page - (*rqstp).rq_respages as i32; (*ctxt).sc_page_count+=pages; for i in 0..pages { *(*ctxt).sc_pages.add(i as usize)=*(*rqstp).rq_respages.add(i as usize); *(*rqstp).rq_respages.add(i as usize)=core::ptr::null_mut(); } }

pub unsafe fn svc_rdma_result_payload(rqstp: *mut svc_rqst, offset: u32, length: u32) -> i32 { let r=&mut *(*rqstp).rq_xprt_ctxt; let c=r.rc_cur_result_payload; if length==0 || c.is_null(){return 0;} r.rc_cur_result_payload=pcl_next_chunk(&r.rc_write_pcl,c); if length>(*c).ch_length{return -E2BIG;} (*c).ch_position=offset; (*c).ch_payload_length=length; 0 }

// External declarations corresponding to the Linux/RDMA helpers referenced above.
extern "C" { fn xdr_stream_encode_item_absent(*mut xdr_stream)->isize; fn xdr_stream_encode_item_present(*mut xdr_stream)->isize; fn xdr_stream_encode_u32(*mut xdr_stream,u32)->isize; fn xdr_reserve_space(*mut xdr_stream,usize)->*mut u32; fn xdr_encode_rdma_segment(*mut u32,u32,u32,u64); fn pcl_is_empty(*const svc_rdma_pcl)->bool; fn pcl_first_chunk(*const svc_rdma_pcl)->*mut svc_rdma_chunk; fn pcl_next_chunk(*const svc_rdma_pcl,*mut svc_rdma_chunk)->*mut svc_rdma_chunk; fn virt_to_page(*mut c_void)->*mut page; fn offset_in_page(usize)->usize; fn ib_dma_map_page(*mut svcxprt_rdma,*mut page,usize,u32)->u64; fn ib_dma_mapping_error(*mut svcxprt_rdma,u64)->bool; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
