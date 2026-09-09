// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
/* Copyright (c) 2016-2018 Oracle. All rights reserved. */
/* Copyright (c) 2014 Open Grid Computing, Inc. All rights reserved. */
/* Copyright (c) 2005-2006 Network Appliance, Inc. All rights reserved. */

// Dependencies supplied by the surrounding kernel/RDMA translation.

#[inline]
unsafe fn svc_rdma_next_recv_ctxt(list: *mut list_head) -> *mut svc_rdma_recv_ctxt {
    list_first_entry_or_null(list, rc_list)
}

unsafe fn svc_rdma_recv_ctxt_alloc(rdma: *mut svcxprt_rdma) -> *mut svc_rdma_recv_ctxt {
    let device = (*(*rdma).sc_cm_id).device;
    let node = ibdev_to_node(device);
    let pages = svc_serv_maxpages((*rdma).sc_xprt.xpt_server);
    let ctxt = kzalloc_node(struct_size::<svc_rdma_recv_ctxt>(pages), GFP_KERNEL, node);
    if ctxt.is_null() { return core::ptr::null_mut(); }
    (*ctxt).rc_maxpages = pages;
    let buffer = kmalloc_node((*rdma).sc_max_req_size, GFP_KERNEL, node);
    if buffer.is_null() { kfree(ctxt as *mut _); return core::ptr::null_mut(); }
    let addr = ib_dma_map_single(device, buffer, (*rdma).sc_max_req_size, DMA_FROM_DEVICE);
    if ib_dma_mapping_error(device, addr) {
        kfree(buffer); kfree(ctxt as *mut _); return core::ptr::null_mut();
    }
    svc_rdma_recv_cid_init(rdma, &mut (*ctxt).rc_cid);
    pcl_init(&mut (*ctxt).rc_call_pcl); pcl_init(&mut (*ctxt).rc_read_pcl);
    pcl_init(&mut (*ctxt).rc_write_pcl); pcl_init(&mut (*ctxt).rc_reply_pcl);
    (*ctxt).rc_recv_wr.next = core::ptr::null_mut();
    (*ctxt).rc_recv_wr.wr_cqe = &mut (*ctxt).rc_cqe;
    (*ctxt).rc_recv_wr.sg_list = &mut (*ctxt).rc_recv_sge;
    (*ctxt).rc_recv_wr.num_sge = 1;
    (*ctxt).rc_cqe.done = Some(svc_rdma_wc_receive);
    (*ctxt).rc_recv_sge.addr = addr;
    (*ctxt).rc_recv_sge.length = (*rdma).sc_max_req_size;
    (*ctxt).rc_recv_sge.lkey = (*(*rdma).sc_pd).local_dma_lkey;
    (*ctxt).rc_recv_buf = buffer;
    svc_rdma_cc_init(rdma, &mut (*ctxt).rc_cc);
    ctxt
}

unsafe fn svc_rdma_recv_ctxt_destroy(rdma: *mut svcxprt_rdma, ctxt: *mut svc_rdma_recv_ctxt) {
    ib_dma_unmap_single((*(*rdma).sc_cm_id).device, (*ctxt).rc_recv_sge.addr,
                        (*ctxt).rc_recv_sge.length, DMA_FROM_DEVICE);
    kfree((*ctxt).rc_recv_buf); kfree(ctxt as *mut _);
}

pub unsafe fn svc_rdma_recv_ctxts_destroy(rdma: *mut svcxprt_rdma) {
    while let Some(node) = llist_del_first(&mut (*rdma).sc_recv_ctxts).as_mut() {
        let ctxt = llist_entry(node, rc_node);
        svc_rdma_recv_ctxt_destroy(rdma, ctxt);
    }
}

pub unsafe fn svc_rdma_recv_ctxt_get(rdma: *mut svcxprt_rdma) -> *mut svc_rdma_recv_ctxt {
    let node = llist_del_first(&mut (*rdma).sc_recv_ctxts);
    if node.is_null() { return core::ptr::null_mut(); }
    let ctxt = llist_entry(node, rc_node);
    (*ctxt).rc_page_count = 0; ctxt
}

pub unsafe fn svc_rdma_recv_ctxt_put(rdma: *mut svcxprt_rdma, ctxt: *mut svc_rdma_recv_ctxt) {
    svc_rdma_cc_release(rdma, &mut (*ctxt).rc_cc, DMA_FROM_DEVICE);
    release_pages((*ctxt).rc_pages, (*ctxt).rc_page_count);
    pcl_free(&mut (*ctxt).rc_call_pcl); pcl_free(&mut (*ctxt).rc_read_pcl);
    pcl_free(&mut (*ctxt).rc_write_pcl); pcl_free(&mut (*ctxt).rc_reply_pcl);
    llist_add(&mut (*ctxt).rc_node, &mut (*rdma).sc_recv_ctxts);
}

pub unsafe fn svc_rdma_release_ctxt(xprt: *mut svc_xprt, vctxt: *mut core::ffi::c_void) {
    let ctxt = vctxt as *mut svc_rdma_recv_ctxt;
    let rdma = container_of(xprt, sc_xprt);
    if !ctxt.is_null() { svc_rdma_recv_ctxt_put(rdma, ctxt); }
    svc_rdma_send_ctxts_drain(rdma);
}

unsafe fn svc_rdma_refresh_recvs(rdma: *mut svcxprt_rdma, mut wanted: u32) -> bool {
    if test_bit(XPT_CLOSE, &(*rdma).sc_xprt.xpt_flags) { return false; }
    let mut bad_wr: *const ib_recv_wr = core::ptr::null();
    let mut recv_chain: *mut ib_recv_wr = core::ptr::null_mut();
    while wanted != 0 { wanted -= 1; let ctxt = svc_rdma_recv_ctxt_get(rdma); if ctxt.is_null() { break; }
        trace_svcrdma_post_recv(&(*ctxt).rc_cid); (*ctxt).rc_recv_wr.next = recv_chain; recv_chain = &mut (*ctxt).rc_recv_wr; (*rdma).sc_pending_recvs += 1; }
    if recv_chain.is_null() { return true; }
    let ret = ib_post_recv((*rdma).sc_qp, recv_chain, &mut bad_wr);
    if ret == 0 { return true; }
    trace_svcrdma_rq_post_err(rdma, ret);
    while !bad_wr.is_null() { let ctxt = container_of(bad_wr as *mut ib_recv_wr, rc_recv_wr); bad_wr = (*bad_wr).next; svc_rdma_recv_ctxt_put(rdma, ctxt); }
    false
}

pub unsafe fn svc_rdma_post_recvs(rdma: *mut svcxprt_rdma) -> bool {
    let mut total = (*rdma).sc_max_requests * 2 + (*rdma).sc_recv_batch;
    while total != 0 { total -= 1; let ctxt = svc_rdma_recv_ctxt_alloc(rdma); if ctxt.is_null() { return false; } llist_add(&mut (*ctxt).rc_node, &mut (*rdma).sc_recv_ctxts); }
    svc_rdma_refresh_recvs(rdma, (*rdma).sc_max_requests)
}

unsafe fn svc_rdma_wc_receive(cq: *mut ib_cq, wc: *mut ib_wc) {
    let rdma = (*cq).cq_context; let cqe = (*wc).wr_cqe;
    (*rdma).sc_pending_recvs -= 1;
    let ctxt = container_of(cqe, rc_cqe);
    if (*wc).status != IB_WC_SUCCESS { if (*wc).status == IB_WC_WR_FLUSH_ERR { trace_svcrdma_wc_recv_flush(wc, &(*ctxt).rc_cid); } else { trace_svcrdma_wc_recv_err(wc, &(*ctxt).rc_cid); } svc_rdma_recv_ctxt_put(rdma, ctxt); svc_rdma_xprt_deferred_close(rdma); return; }
    trace_svcrdma_wc_recv(wc, &(*ctxt).rc_cid);
    if (*rdma).sc_pending_recvs < (*rdma).sc_max_requests && !svc_rdma_refresh_recvs(rdma, (*rdma).sc_recv_batch) { svc_rdma_recv_ctxt_put(rdma, ctxt); svc_rdma_xprt_deferred_close(rdma); return; }
    (*ctxt).rc_byte_len = (*wc).byte_len;
    spin_lock(&mut (*rdma).sc_rq_dto_lock); list_add_tail(&mut (*ctxt).rc_list, &mut (*rdma).sc_rq_dto_q); set_bit(XPT_DATA, &mut (*rdma).sc_xprt.xpt_flags); spin_unlock(&mut (*rdma).sc_rq_dto_lock);
    if !test_bit(RDMAXPRT_CONN_PENDING, &(*rdma).sc_flags) { svc_xprt_enqueue(&mut (*rdma).sc_xprt); }
}

pub unsafe fn svc_rdma_flush_recv_queues(rdma: *mut svcxprt_rdma) {
    while let Some(ctxt) = svc_rdma_next_recv_ctxt(&mut (*rdma).sc_read_complete_q).as_mut() { list_del(&mut (*ctxt).rc_list); svc_rdma_recv_ctxt_put(rdma, ctxt); }
    while let Some(ctxt) = svc_rdma_next_recv_ctxt(&mut (*rdma).sc_rq_dto_q).as_mut() { list_del(&mut (*ctxt).rc_list); svc_rdma_recv_ctxt_put(rdma, ctxt); }
}

unsafe fn svc_rdma_build_arg_xdr(rqstp: *mut svc_rqst, ctxt: *mut svc_rdma_recv_ctxt) {
    let arg = &mut (*rqstp).rq_arg; arg.head[0].iov_base = (*ctxt).rc_recv_buf; arg.head[0].iov_len = (*ctxt).rc_byte_len; arg.tail[0].iov_base = core::ptr::null_mut(); arg.tail[0].iov_len = 0; arg.page_len = 0; arg.page_base = 0; arg.buflen = (*ctxt).rc_byte_len; arg.len = (*ctxt).rc_byte_len;
}

unsafe fn xdr_count_read_segments(rctxt: *mut svc_rdma_recv_ctxt, mut p: *mut __be32) -> bool {
    let maxlen = (*rctxt).rc_maxpages << PAGE_SHIFT; let mut total_len = 0;
    (*rctxt).rc_call_pcl.cl_count = 0; (*rctxt).rc_read_pcl.cl_count = 0;
    while xdr_item_is_present(p) { let mut position=0; let mut handle=0; let mut length=0; let mut offset=0;
        p=xdr_inline_decode(&mut (*rctxt).rc_stream, rpcrdma_readseg_maxsz*core::mem::size_of::<__be32>()); if p.is_null(){return false;}
        xdr_decode_read_segment(p,&mut position,&mut handle,&mut length,&mut offset); if length>maxlen{return false;} total_len += length; if PAGE_ALIGN(total_len)>maxlen{return false;}
        if position != 0 { if position & 3 != 0{return false;} (*rctxt).rc_read_pcl.cl_count+=1; } else {(*rctxt).rc_call_pcl.cl_count+=1;}
        p=xdr_inline_decode(&mut (*rctxt).rc_stream, core::mem::size_of::<__be32>()); if p.is_null(){return false;}
    } true
}

unsafe fn xdr_check_read_list(rctxt:*mut svc_rdma_recv_ctxt)->bool { let p=xdr_inline_decode(&mut (*rctxt).rc_stream,core::mem::size_of::<__be32>()); if p.is_null(){return false;} if !xdr_count_read_segments(rctxt,p){return false;} if !pcl_alloc_call(rctxt,p){return false;} pcl_alloc_read(rctxt,p) }

unsafe fn xdr_check_write_chunk(rctxt:*mut svc_rdma_recv_ctxt)->bool { let mut segcount=0; if xdr_stream_decode_u32(&mut (*rctxt).rc_stream,&mut segcount){return false;} if segcount==0 || unlikely(segcount>(*rctxt).rc_maxpages){return false;} !xdr_inline_decode(&mut (*rctxt).rc_stream,segcount*rpcrdma_segment_maxsz*core::mem::size_of::<__be32>()).is_null() }

unsafe fn xdr_count_write_chunks(rctxt:*mut svc_rdma_recv_ctxt,mut p:*mut __be32)->bool { (*rctxt).rc_write_pcl.cl_count=0; while xdr_item_is_present(p){if !xdr_check_write_chunk(rctxt){return false;} (*rctxt).rc_write_pcl.cl_count+=1; p=xdr_inline_decode(&mut (*rctxt).rc_stream,core::mem::size_of::<__be32>()); if p.is_null(){return false;}} true }
unsafe fn xdr_check_write_list(rctxt:*mut svc_rdma_recv_ctxt)->bool { let p=xdr_inline_decode(&mut (*rctxt).rc_stream,core::mem::size_of::<__be32>()); if p.is_null(){return false;} if !xdr_count_write_chunks(rctxt,p){return false;} if !pcl_alloc_write(rctxt,&mut (*rctxt).rc_write_pcl,p){return false;} (*rctxt).rc_cur_result_payload=pcl_first_chunk(&mut (*rctxt).rc_write_pcl); true }
unsafe fn xdr_check_reply_chunk(rctxt:*mut svc_rdma_recv_ctxt)->bool { let p=xdr_inline_decode(&mut (*rctxt).rc_stream,core::mem::size_of::<__be32>()); if p.is_null(){return false;} if !xdr_item_is_present(p){return true;} if !xdr_check_write_chunk(rctxt){return false;} (*rctxt).rc_reply_pcl.cl_count=1; pcl_alloc_write(rctxt,&mut (*rctxt).rc_reply_pcl,p) }

unsafe fn svc_rdma_get_inv_rkey(rdma:*mut svcxprt_rdma,ctxt:*mut svc_rdma_recv_ctxt) { let mut inv=0; (*ctxt).rc_inv_rkey=0; if !(*rdma).sc_snd_w_inv{return;} macro_rules! scan {($p:expr)=>{pcl_for_each_chunk(chunk,&mut (*ctxt).$p){pcl_for_each_segment(segment,chunk){if inv==0{inv=(*segment).rs_handle;}else if inv!=(*segment).rs_handle{return;}}}}} scan!(rc_call_pcl); scan!(rc_read_pcl); scan!(rc_write_pcl); scan!(rc_reply_pcl); (*ctxt).rc_inv_rkey=inv; }

unsafe fn svc_rdma_xdr_decode_req(rq_arg:*mut xdr_buf,rctxt:*mut svc_rdma_recv_ctxt)->i32 { let rdma_argp=(*rq_arg).head[0].iov_base as *mut __be32; xdr_init_decode(&mut (*rctxt).rc_stream,rq_arg,rdma_argp,core::ptr::null_mut()); let mut p=xdr_inline_decode(&mut (*rctxt).rc_stream,rpcrdma_fixed_maxsz*core::mem::size_of::<__be32>()); if p.is_null(){trace_svcrdma_decode_short_err(rctxt,(*rq_arg).len);return -EINVAL;} p=p.add(1); if *p!=rpcrdma_version{trace_svcrdma_decode_badvers_err(rctxt,rdma_argp);return -EPROTONOSUPPORT;} p=p.add(2); (*rctxt).rc_msgtype=*p; if *p==rdma_done||*p==rdma_error{trace_svcrdma_decode_drop_err(rctxt,rdma_argp);return 0;} if *p!=rdma_msg&&*p!=rdma_nomsg{trace_svcrdma_decode_badproc_err(rctxt,rdma_argp);return -EINVAL;} if !xdr_check_read_list(rctxt)||!xdr_check_write_list(rctxt)||!xdr_check_reply_chunk(rctxt){trace_svcrdma_decode_parse_err(rctxt,rdma_argp);return -EINVAL;} (*rq_arg).head[0].iov_base=(*rctxt).rc_stream.p; let hdr_len=xdr_stream_pos(&mut (*rctxt).rc_stream); if !pcl_check_read_chunk_positions(rctxt,(*rq_arg).head[0].iov_len-hdr_len){trace_svcrdma_decode_parse_err(rctxt,rdma_argp);return -EINVAL;} (*rq_arg).head[0].iov_len-=hdr_len; (*rq_arg).len-=hdr_len; trace_svcrdma_decode_rqst(rctxt,rdma_argp,hdr_len); hdr_len as i32 }

unsafe fn svc_rdma_send_error(rdma:*mut svcxprt_rdma,rctxt:*mut svc_rdma_recv_ctxt,status:i32){let s=svc_rdma_send_ctxt_get(rdma);if !s.is_null(){svc_rdma_send_error_msg(rdma,s,rctxt,status);}}
unsafe fn svc_rdma_is_reverse_direction_reply(xprt:*mut svc_xprt,rctxt:*mut svc_rdma_recv_ctxt)->bool {if (*xprt).xpt_bc_xprt.is_null()||(*rctxt).rc_msgtype!=rdma_msg{return false;} if !pcl_is_empty(&(*rctxt).rc_call_pcl)||!pcl_is_empty(&(*rctxt).rc_read_pcl)||!pcl_is_empty(&(*rctxt).rc_write_pcl)||!pcl_is_empty(&(*rctxt).rc_reply_pcl){return false;} let p=(*rctxt).rc_recv_buf as *mut __be32; *(p.add(8))!=cpu_to_be32(RPC_CALL) }

unsafe fn svc_rdma_read_complete_one(rqstp:*mut svc_rqst,ctxt:*mut svc_rdma_recv_ctxt){let chunk=pcl_first_chunk(&mut (*ctxt).rc_read_pcl);let buf=&mut (*rqstp).rq_arg;buf.tail[0].iov_base=(buf.head[0].iov_base as *mut u8).add((*chunk).ch_position as usize) as *mut _;buf.tail[0].iov_len=buf.head[0].iov_len-(*chunk).ch_position;buf.head[0].iov_len=(*chunk).ch_position;buf.pages=&mut (*rqstp).rq_pages[0];let length=xdr_align_size((*chunk).ch_length);buf.page_len=length;buf.len+=length;buf.buflen+=length;}
unsafe fn svc_rdma_read_complete_multiple(rqstp:*mut svc_rqst,ctxt:*mut svc_rdma_recv_ctxt){let buf=&mut (*rqstp).rq_arg;buf.len+=(*ctxt).rc_readbytes;buf.buflen+=(*ctxt).rc_readbytes;buf.head[0].iov_base=page_address((*rqstp).rq_pages[0]);buf.head[0].iov_len=min_t(PAGE_SIZE,(*ctxt).rc_readbytes);buf.pages=&mut (*rqstp).rq_pages[1];buf.page_len=(*ctxt).rc_readbytes-buf.head[0].iov_len;}
unsafe fn svc_rdma_read_complete_pzrc(rqstp:*mut svc_rqst,ctxt:*mut svc_rdma_recv_ctxt){svc_rdma_read_complete_multiple(rqstp,ctxt)}
unsafe fn svc_rdma_read_complete(rqstp:*mut svc_rqst,ctxt:*mut svc_rdma_recv_ctxt){release_pages((*rqstp).rq_pages,(*ctxt).rc_page_count);for i in 0..(*ctxt).rc_page_count{(*rqstp).rq_pages[i as usize]=(*ctxt).rc_pages[i as usize];}(*ctxt).rc_page_count=0;(*rqstp).rq_arg=(*ctxt).rc_saved_arg;if pcl_is_empty(&(*ctxt).rc_call_pcl){if (*ctxt).rc_read_pcl.cl_count==1{svc_rdma_read_complete_one(rqstp,ctxt)}else{svc_rdma_read_complete_multiple(rqstp,ctxt)}}else{svc_rdma_read_complete_pzrc(rqstp,ctxt)}trace_svcrdma_read_finished(&(*ctxt).rc_cid);}

pub unsafe fn svc_rdma_recvfrom(rqstp:*mut svc_rqst)->i32 {let xprt=(*rqstp).rq_xprt;let rdma=container_of(xprt,sc_xprt);(*rqstp).rq_next_page=(*rqstp).rq_respages;(*rqstp).rq_xprt_ctxt=core::ptr::null_mut();spin_lock(&mut (*rdma).sc_rq_dto_lock);let mut ctxt=svc_rdma_next_recv_ctxt(&mut (*rdma).sc_read_complete_q);if !ctxt.is_null(){list_del(&mut (*ctxt).rc_list);spin_unlock(&mut (*rdma).sc_rq_dto_lock);svc_xprt_received(xprt);svc_rdma_read_complete(rqstp,ctxt);(*rqstp).rq_xprt_ctxt=ctxt;(*rqstp).rq_prot=IPPROTO_MAX;svc_xprt_copy_addrs(rqstp,xprt);set_bit(RQ_SECURE,&mut (*rqstp).rq_flags);return (*rqstp).rq_arg.len;}ctxt=svc_rdma_next_recv_ctxt(&mut (*rdma).sc_rq_dto_q);if !ctxt.is_null(){list_del(&mut (*ctxt).rc_list)}else{clear_bit(XPT_DATA,&mut (*xprt).xpt_flags)}spin_unlock(&mut (*rdma).sc_rq_dto_lock);svc_xprt_received(xprt);if ctxt.is_null(){return 0;}percpu_counter_inc(&svcrdma_stat_recv);ib_dma_sync_single_for_cpu((*(*rdma).sc_cm_id).device,(*ctxt).rc_recv_sge.addr,(*ctxt).rc_byte_len,DMA_FROM_DEVICE);svc_rdma_build_arg_xdr(rqstp,ctxt);let ret=svc_rdma_xdr_decode_req(&mut (*rqstp).rq_arg,ctxt);if ret<0{svc_rdma_send_error(rdma,ctxt,ret);svc_rdma_recv_ctxt_put(rdma,ctxt);return 0;}if ret==0{svc_rdma_recv_ctxt_put(rdma,ctxt);return 0;}if svc_rdma_is_reverse_direction_reply(xprt,ctxt){svc_rdma_handle_bc_reply(rqstp,ctxt);svc_rdma_recv_ctxt_put(rdma,ctxt);return 0;}svc_rdma_get_inv_rkey(rdma,ctxt);if !pcl_is_empty(&(*ctxt).rc_read_pcl)||!pcl_is_empty(&(*ctxt).rc_call_pcl){(*ctxt).rc_saved_arg=(*rqstp).rq_arg;let result=svc_rdma_process_read_list(rdma,rqstp,ctxt);if result<0{if result==-EINVAL{svc_rdma_send_error(rdma,ctxt,result)}svc_rdma_recv_ctxt_put(rdma,ctxt);svc_rdma_xprt_deferred_close(rdma);return result;}return 0;}(*rqstp).rq_xprt_ctxt=ctxt;(*rqstp).rq_prot=IPPROTO_MAX;svc_xprt_copy_addrs(rqstp,xprt);set_bit(RQ_SECURE,&mut (*rqstp).rq_flags);(*rqstp).rq_arg.len}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
