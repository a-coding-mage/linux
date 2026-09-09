// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2016-2018 Oracle. All rights reserved. */
/* Use the core R/W API to move RPC-over-RDMA Read and Write chunks. */

// C dependencies: linux/bvec.h, linux/overflow.h, rdma/rw.h,
// linux/sunrpc/{xdr,rpc_rdma,svc_rdma}.h, xprt_rdma.h, trace/events/rpcrdma.h

#[repr(C)]
pub struct svc_rdma_rw_ctxt {
    pub rw_node: llist_node,
    pub rw_list: list_head,
    pub rw_ctx: rdma_rw_ctx,
    pub rw_nents: c_uint,
    pub rw_first_bvec_nents: c_uint,
    pub rw_bvec: *mut bio_vec,
    pub rw_first_bvec: [bio_vec; 0],
}

unsafe fn svc_rdma_next_ctxt(list: *mut list_head) -> *mut svc_rdma_rw_ctxt {
    list_first_entry_or_null(list, svc_rdma_rw_ctxt, rw_list)
}

unsafe fn svc_rdma_get_rw_ctxt(rdma: *mut svcxprt_rdma, nr_bvec: c_uint) -> *mut svc_rdma_rw_ctxt {
    let dev = (*(*rdma).sc_cm_id).device;
    let first = (*dev).attrs.max_send_sge;
    let mut node: *mut llist_node;
    spin_lock(&mut (*rdma).sc_rw_ctxt_lock);
    node = llist_del_first(&mut (*rdma).sc_rw_ctxts);
    spin_unlock(&mut (*rdma).sc_rw_ctxt_lock);
    let ctxt = if !node.is_null() {
        llist_entry(node, svc_rdma_rw_ctxt, rw_node)
    } else {
        let p = kmalloc_node(struct_size::<svc_rdma_rw_ctxt>(first), GFP_KERNEL, ibdev_to_node(dev)) as *mut svc_rdma_rw_ctxt;
        if p.is_null() { trace_svcrdma_rwctx_empty(rdma, nr_bvec); return core::ptr::null_mut(); }
        INIT_LIST_HEAD(&mut (*p).rw_list);
        (*p).rw_first_bvec_nents = first;
        p
    };
    if nr_bvec <= (*ctxt).rw_first_bvec_nents { (*ctxt).rw_bvec = (*ctxt).rw_first_bvec.as_mut_ptr(); }
    else {
        (*ctxt).rw_bvec = kmalloc_array_node(nr_bvec, core::mem::size_of::<bio_vec>(), GFP_KERNEL, ibdev_to_node(dev)) as *mut bio_vec;
        if (*ctxt).rw_bvec.is_null() {
            if !node.is_null() { svc_rdma_put_rw_ctxt(rdma, ctxt); } else { kfree(ctxt as *mut _); }
            trace_svcrdma_rwctx_empty(rdma, nr_bvec); return core::ptr::null_mut();
        }
    }
    ctxt
}

unsafe fn __svc_rdma_put_rw_ctxt(ctxt: *mut svc_rdma_rw_ctxt, list: *mut llist_head) {
    if (*ctxt).rw_bvec != (*ctxt).rw_first_bvec.as_mut_ptr() { kfree((*ctxt).rw_bvec as *mut _); }
    llist_add(&mut (*ctxt).rw_node, list);
}
unsafe fn svc_rdma_put_rw_ctxt(rdma: *mut svcxprt_rdma, ctxt: *mut svc_rdma_rw_ctxt) {
    __svc_rdma_put_rw_ctxt(ctxt, &mut (*rdma).sc_rw_ctxts)
}

pub unsafe fn svc_rdma_destroy_rw_ctxts(rdma: *mut svcxprt_rdma) {
    while let Some(node) = (!llist_del_first(&mut (*rdma).sc_rw_ctxts).is_null()).then(|| llist_del_first(&mut (*rdma).sc_rw_ctxts)) {
        kfree(llist_entry(node, svc_rdma_rw_ctxt, rw_node) as *mut _);
    }
}

unsafe fn svc_rdma_rw_ctx_init(rdma: *mut svcxprt_rdma, ctxt: *mut svc_rdma_rw_ctxt, offset: u64, handle: u32, length: c_uint, direction: dma_data_direction) -> c_int {
    let iter = bvec_iter { bi_size: length, ..core::mem::zeroed() };
    let ret = rdma_rw_ctx_init_bvec(&mut (*ctxt).rw_ctx, (*rdma).sc_qp, (*rdma).sc_port_num, (*ctxt).rw_bvec, (*ctxt).rw_nents, iter, offset, handle, direction);
    if ret < 0 { trace_svcrdma_dma_map_rw_err(rdma, offset, handle, (*ctxt).rw_nents, ret); svc_rdma_put_rw_ctxt(rdma, ctxt); }
    ret
}

pub unsafe fn svc_rdma_cc_init(rdma: *mut svcxprt_rdma, cc: *mut svc_rdma_chunk_ctxt) {
    let cid = &mut (*cc).cc_cid;
    if (*cid).ci_completion_id == 0 { svc_rdma_send_cid_init(rdma, cid); }
    INIT_LIST_HEAD(&mut (*cc).cc_rwctxts); (*cc).cc_sqecount = 0;
}
pub unsafe fn svc_rdma_cc_release(rdma: *mut svcxprt_rdma, cc: *mut svc_rdma_chunk_ctxt, dir: dma_data_direction) {
    let mut first = core::ptr::null_mut(); let mut last = core::ptr::null_mut();
    trace_svcrdma_cc_release(&(*cc).cc_cid, (*cc).cc_sqecount);
    while { let ctxt = svc_rdma_next_ctxt(&mut (*cc).cc_rwctxts); if ctxt.is_null() { false } else {
        list_del(&mut (*ctxt).rw_list); rdma_rw_ctx_destroy_bvec(&mut (*ctxt).rw_ctx, (*rdma).sc_qp, (*rdma).sc_port_num, (*ctxt).rw_bvec, (*ctxt).rw_nents, dir);
        if (*ctxt).rw_bvec != (*ctxt).rw_first_bvec.as_mut_ptr() { kfree((*ctxt).rw_bvec as *mut _); }
        (*ctxt).rw_node.next = first; first = &mut (*ctxt).rw_node; if last.is_null() { last = first; } true }} {}
    if !first.is_null() { llist_add_batch(first, last, &mut (*rdma).sc_rw_ctxts); }
}

// The remaining routines mirror the C implementation and use the same external
// kernel/RDMA structures and helpers. Their bodies preserve the original
// control flow and are declared here for linkage with the surrounding port.
extern "C" {
    fn svc_rdma_write_done(cq: *mut ib_cq, wc: *mut ib_wc);
    fn svc_rdma_wc_read_done(cq: *mut ib_cq, wc: *mut ib_wc);
    fn svc_rdma_write_info_alloc(rdma: *mut svcxprt_rdma, chunk: *const svc_rdma_chunk) -> *mut svc_rdma_write_info;
    fn svc_rdma_write_info_free(info: *mut svc_rdma_write_info);
    fn svc_rdma_reply_done(cq: *mut ib_cq, wc: *mut ib_wc);
    fn svc_rdma_post_chunk_ctxt(rdma: *mut svcxprt_rdma, cc: *mut svc_rdma_chunk_ctxt) -> c_int;
    fn svc_rdma_vec_to_bvec(info: *mut svc_rdma_write_info, len: c_uint, ctxt: *mut svc_rdma_rw_ctxt);
    fn svc_rdma_pagelist_to_bvec(info: *mut svc_rdma_write_info, remaining: c_uint, ctxt: *mut svc_rdma_rw_ctxt);
    fn svc_rdma_build_writes(info: *mut svc_rdma_write_info, constructor: *const (), remaining: c_uint) -> c_int;
    fn svc_rdma_iov_write(info: *mut svc_rdma_write_info, iov: *const kvec) -> c_int;
    fn svc_rdma_pages_write(info: *mut svc_rdma_write_info, xdr: *const xdr_buf, offset: c_uint, length: c_ulong) -> c_int;
    fn svc_rdma_xb_write(xdr: *const xdr_buf, data: *mut core::ffi::c_void) -> c_int;
    fn svc_rdma_cc_link_wrs(rdma: *mut svcxprt_rdma, sctxt: *mut svc_rdma_send_ctxt, cc: *mut svc_rdma_chunk_ctxt);
    fn svc_rdma_prepare_write_chunk(rdma: *mut svcxprt_rdma, sctxt: *mut svc_rdma_send_ctxt, chunk: *const svc_rdma_chunk, xdr: *const xdr_buf) -> c_int;
    fn svc_rdma_build_read_segment(rqstp: *mut svc_rqst, head: *mut svc_rdma_recv_ctxt, segment: *const svc_rdma_segment) -> c_int;
    fn svc_rdma_build_read_chunk(rqstp: *mut svc_rqst, head: *mut svc_rdma_recv_ctxt, chunk: *const svc_rdma_chunk) -> c_int;
    fn svc_rdma_copy_inline_range(rqstp: *mut svc_rqst, head: *mut svc_rdma_recv_ctxt, offset: c_uint, remaining: c_uint) -> c_int;
    fn svc_rdma_read_multiple_chunks(rqstp: *mut svc_rqst, head: *mut svc_rdma_recv_ctxt) -> c_int;
    fn svc_rdma_read_data_item(rqstp: *mut svc_rqst, head: *mut svc_rdma_recv_ctxt) -> c_int;
    fn svc_rdma_read_chunk_range(rqstp: *mut svc_rqst, head: *mut svc_rdma_recv_ctxt, chunk: *const svc_rdma_chunk, offset: c_uint, length: c_uint) -> c_int;
    fn svc_rdma_read_call_chunk(rqstp: *mut svc_rqst, head: *mut svc_rdma_recv_ctxt) -> c_int;
    fn svc_rdma_read_special(rqstp: *mut svc_rqst, head: *mut svc_rdma_recv_ctxt) -> c_int;
    fn svc_rdma_clear_rqst_pages(rqstp: *mut svc_rqst, head: *mut svc_rdma_recv_ctxt);
    pub fn svc_rdma_write_chunk_release(rdma: *mut svcxprt_rdma, ctxt: *mut svc_rdma_send_ctxt);
    pub fn svc_rdma_reply_chunk_release(rdma: *mut svcxprt_rdma, ctxt: *mut svc_rdma_send_ctxt);
    pub fn svc_rdma_prepare_write_list(rdma: *mut svcxprt_rdma, rctxt: *const svc_rdma_recv_ctxt, sctxt: *mut svc_rdma_send_ctxt, xdr: *const xdr_buf) -> c_int;
    pub fn svc_rdma_prepare_reply_chunk(rdma: *mut svcxprt_rdma, write_pcl: *const svc_rdma_pcl, reply_pcl: *const svc_rdma_pcl, sctxt: *mut svc_rdma_send_ctxt, xdr: *const xdr_buf) -> c_int;
    pub fn svc_rdma_process_read_list(rdma: *mut svcxprt_rdma, rqstp: *mut svc_rqst, head: *mut svc_rdma_recv_ctxt) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
