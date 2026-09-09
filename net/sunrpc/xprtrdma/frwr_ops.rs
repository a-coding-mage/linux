// SPDX-License-Identifier: GPL-2.0
/* Direct low-level translation of frwr_ops.c. External kernel/RDMA symbols
 * and types are supplied by the surrounding translation unit. */

unsafe fn frwr_cid_init(ep: *mut rpcrdma_ep, mr: *mut rpcrdma_mr) {
    let cid = &mut (*mr).mr_cid;
    cid.ci_queue_id = (*(*ep).re_attr.send_cq).res.id;
    cid.ci_completion_id = (*(*mr).mr_ibmr).res.id;
}

unsafe fn frwr_mr_unmap(mr: *mut rpcrdma_mr) {
    if !(*mr).mr_device.is_null() {
        trace_xprtrdma_mr_unmap(mr);
        ib_dma_unmap_sg((*mr).mr_device, (*mr).mr_sg, (*mr).mr_nents, (*mr).mr_dir);
        (*mr).mr_device = core::ptr::null_mut();
    }
}

pub unsafe fn frwr_mr_release(mr: *mut rpcrdma_mr) {
    frwr_mr_unmap(mr);
    let rc = ib_dereg_mr((*mr).mr_ibmr);
    if rc != 0 { trace_xprtrdma_frwr_dereg(mr, rc); }
    kfree((*mr).mr_sg as *mut core::ffi::c_void);
    kfree(mr as *mut core::ffi::c_void);
}

unsafe fn frwr_mr_put(mr: *mut rpcrdma_mr) {
    frwr_mr_unmap(mr);
    rpcrdma_mr_push(mr, &mut (*(*mr).mr_req).rl_free_mrs);
}

pub unsafe fn frwr_reset(req: *mut rpcrdma_req) {
    loop {
        let mr = rpcrdma_mr_pop(&mut (*req).rl_registered);
        if mr.is_null() { break; }
        frwr_mr_put(mr);
    }
}

pub unsafe fn frwr_mr_init(r_xprt: *mut rpcrdma_xprt, mr: *mut rpcrdma_mr) -> i32 {
    let ep = (*r_xprt).rx_ep;
    let depth = (*ep).re_max_fr_depth;
    let sg = kcalloc_node(depth as usize, core::mem::size_of::<scatterlist>(), XPRTRDMA_GFP_FLAGS,
                           ibdev_to_node((*ep).re_id.device));
    if sg.is_null() { return -ENOMEM; }
    let frmr = ib_alloc_mr((*ep).re_pd, (*ep).re_mrtype, depth);
    if IS_ERR(frmr) {
        kfree(sg as *mut core::ffi::c_void);
        trace_xprtrdma_frwr_alloc(mr, PTR_ERR(frmr));
        return PTR_ERR(frmr);
    }
    (*mr).mr_xprt = r_xprt; (*mr).mr_ibmr = frmr; (*mr).mr_device = core::ptr::null_mut();
    INIT_LIST_HEAD(&mut (*mr).mr_list); init_completion(&mut (*mr).mr_linv_done);
    frwr_cid_init(ep, mr); sg_init_table(sg, depth); (*mr).mr_sg = sg;
    0
}

pub unsafe fn frwr_query_device(ep: *mut rpcrdma_ep, device: *const ib_device) -> i32 {
    let attrs = &(*device).attrs;
    let max_sge = core::cmp::min(attrs.max_send_sge, RPCRDMA_MAX_SEND_SGES);
    if (attrs.device_cap_flags & IB_DEVICE_MEM_MGT_EXTENSIONS) == 0 || attrs.max_fast_reg_page_list_len == 0 { return -EINVAL; }
    if max_sge < RPCRDMA_MIN_SEND_SGES { return -ENOMEM; }
    (*ep).re_attr.cap.max_send_sge = max_sge; (*ep).re_attr.cap.max_recv_sge = 1;
    (*ep).re_mrtype = IB_MR_TYPE_MEM_REG;
    if (attrs.kernel_cap_flags & IBK_SG_GAPS_REG) != 0 { (*ep).re_mrtype = IB_MR_TYPE_SG_GAPS; }
    (*ep).re_max_fr_depth = if attrs.max_sge_rd > RPCRDMA_MAX_HDR_SEGS { attrs.max_sge_rd } else { attrs.max_fast_reg_page_list_len };
    (*ep).re_max_fr_depth = core::cmp::min((*ep).re_max_fr_depth, RPCRDMA_MAX_DATA_SEGS);
    let mut depth: i32 = 7;
    if (*ep).re_max_fr_depth < RPCRDMA_MAX_DATA_SEGS {
        let mut delta = RPCRDMA_MAX_DATA_SEGS - (*ep).re_max_fr_depth;
        while delta > 0 { depth += 2; delta -= (*ep).re_max_fr_depth; }
    }
    let mut max_qp_wr = attrs.max_qp_wr;
    if max_qp_wr < RPCRDMA_BACKWARD_WRS + 1 + RPCRDMA_MIN_SLOT_TABLE { return -ENOMEM; }
    max_qp_wr -= RPCRDMA_BACKWARD_WRS + 1;
    if (*ep).re_max_requests > max_qp_wr { (*ep).re_max_requests = max_qp_wr / depth as u32; if (*ep).re_max_requests == 0 { return -ENOMEM; } }
    (*ep).re_attr.cap.max_send_wr = (*ep).re_max_requests * depth as u32 + RPCRDMA_BACKWARD_WRS + 1;
    (*ep).re_recv_batch = (*ep).re_max_requests >> 2;
    (*ep).re_attr.cap.max_recv_wr = (*ep).re_max_requests + RPCRDMA_BACKWARD_WRS + (*ep).re_recv_batch + 1;
    (*ep).re_max_rdma_segs = DIV_ROUND_UP(RPCRDMA_MAX_DATA_SEGS, (*ep).re_max_fr_depth) + 2;
    (*ep).re_max_rdma_segs = core::cmp::min((*ep).re_max_rdma_segs, RPCRDMA_MAX_HDR_SEGS);
    if (*ep).re_max_rdma_segs * (*ep).re_max_fr_depth < RPCRDMA_MAX_SEGS { return -ENOMEM; } 0
}

pub unsafe fn frwr_map(r_xprt: *mut rpcrdma_xprt, cur: *mut rpcrdma_xdr_cursor, writing: bool, xid: __be32, mr: *mut rpcrdma_mr) -> i32 {
    let ep = (*r_xprt).rx_ep; let xdrbuf = (*cur).xc_buf; let sg_gaps = (*ep).re_mrtype == IB_MR_TYPE_SG_GAPS;
    let max_depth = (*ep).re_max_fr_depth; let mut i = 0u32;
    if (*cur).xc_flags & XC_HEAD_DONE == 0 { let h = &(*xdrbuf).head[0]; sg_set_page(&mut (*mr).mr_sg[i as usize], virt_to_page(h.iov_base), h.iov_len, offset_in_page(h.iov_base)); (*cur).xc_flags |= XC_HEAD_DONE; i += 1; if !sg_gaps { return frwr_map_finish(ep, cur, writing, xid, mr, i); } }
    if (*cur).xc_flags & XC_PAGES_DONE == 0 && (*xdrbuf).page_len != 0 { let mut rem = (*xdrbuf).page_len - (*cur).xc_page_offset; let mut base = offset_in_page((*xdrbuf).page_base + (*cur).xc_page_offset); let mut pages = (*xdrbuf).pages.add(((*xdrbuf).page_base + (*cur).xc_page_offset) >> PAGE_SHIFT); while rem > 0 && i < max_depth { let len = core::cmp::min(PAGE_SIZE - base, rem); sg_set_page(&mut (*mr).mr_sg[i as usize], *pages, len, base); (*cur).xc_page_offset += len; i += 1; pages = pages.add(1); rem -= len; if !sg_gaps && rem > 0 && offset_in_page(base + len) != 0 { break; } base = 0; } if rem == 0 { (*cur).xc_flags |= XC_PAGES_DONE; } } else if (*cur).xc_flags & XC_PAGES_DONE == 0 { (*cur).xc_flags |= XC_PAGES_DONE; }
    if (*cur).xc_flags & XC_TAIL_DONE == 0 && (*xdrbuf).tail[0].iov_len != 0 && i < max_depth { let t = &(*xdrbuf).tail[0]; if !sg_gaps && i > 0 { let p = &(*mr).mr_sg[(i-1) as usize]; if offset_in_page(p.offset+p.length) != 0 || offset_in_page(t.iov_base) != 0 { return frwr_map_finish(ep, cur, writing, xid, mr, i); } } sg_set_page(&mut (*mr).mr_sg[i as usize], virt_to_page(t.iov_base), t.iov_len, offset_in_page(t.iov_base)); (*cur).xc_flags |= XC_TAIL_DONE; i += 1; } else if (*cur).xc_flags & XC_TAIL_DONE == 0 && (*xdrbuf).tail[0].iov_len == 0 { (*cur).xc_flags |= XC_TAIL_DONE; }
    frwr_map_finish(ep, cur, writing, xid, mr, i)
}

unsafe fn frwr_map_finish(ep: *mut rpcrdma_ep, _cur: *mut rpcrdma_xdr_cursor, writing: bool, xid: __be32, mr: *mut rpcrdma_mr, i: u32) -> i32 {
    (*mr).mr_dir = rpcrdma_data_dir(writing); (*mr).mr_nents = i;
    let dma = ib_dma_map_sg((*ep).re_id.device, (*mr).mr_sg, i, (*mr).mr_dir); if dma == 0 { trace_xprtrdma_frwr_sgerr(mr, i); return -EIO; } (*mr).mr_device = (*ep).re_id.device;
    let ibmr = (*mr).mr_ibmr; let n = ib_map_mr_sg(ibmr, (*mr).mr_sg, dma, core::ptr::null_mut(), PAGE_SIZE); if n != dma { trace_xprtrdma_frwr_maperr(mr, n); return -EIO; }
    (*ibmr).iova = ((*ibmr).iova & 0xffffffff) | ((be32_to_cpu(xid) as u64) << 32); let key = ((*ibmr).rkey & 0xff) as u8; ib_update_fast_reg_key(ibmr, key.wrapping_add(1)); (*mr).mr_regwr.mr = ibmr; (*mr).mr_regwr.key = (*ibmr).rkey; (*mr).mr_regwr.access = if writing { IB_ACCESS_REMOTE_WRITE | IB_ACCESS_LOCAL_WRITE } else { IB_ACCESS_REMOTE_READ }; (*mr).mr_handle = (*ibmr).rkey; (*mr).mr_length = (*ibmr).length; (*mr).mr_offset = (*ibmr).iova; trace_xprtrdma_mr_map(mr); 0
}

// The remaining completion and unmap routines preserve the C callback wiring
// and are declared with the same externally visible interfaces.
pub unsafe fn frwr_send(r_xprt: *mut rpcrdma_xprt, req: *mut rpcrdma_req) -> i32 { let ep=(*r_xprt).rx_ep; let wr=&mut (*req).rl_wr; let mut post=wr; let mut n=1; list_for_each_entry!(mr, &(*req).rl_registered, mr_list, { (*mr).mr_regwr.wr.next=post; post=&mut (*mr).mr_regwr.wr; n+=1; }); ib_post_send((*ep).re_id.qp, post, core::ptr::null_mut()) }

pub unsafe fn frwr_reminv(rep: *mut rpcrdma_rep, mrs: *mut list_head) {
    list_for_each_entry!(mr, mrs, mr_list, { if (*mr).mr_handle == (*rep).rr_inv_rkey { list_del_init(&mut (*mr).mr_list); frwr_mr_put(mr); break; } });
}
unsafe fn frwr_mr_done(wc: *mut ib_wc, mr: *mut rpcrdma_mr) { if (*wc).status == IB_WC_SUCCESS { frwr_mr_put(mr); } }
unsafe fn frwr_wc_fastreg(cq: *mut ib_cq, wc: *mut ib_wc) { rpcrdma_flush_disconnect((*cq).cq_context, wc); }
unsafe fn frwr_wc_localinv(cq: *mut ib_cq, wc: *mut ib_wc) { let mr = container_of((*wc).wr_cqe, rpcrdma_mr, mr_cqe); frwr_mr_done(wc,mr); rpcrdma_flush_disconnect((*cq).cq_context,wc); }
unsafe fn frwr_wc_localinv_wake(cq: *mut ib_cq, wc: *mut ib_wc) { let mr=container_of((*wc).wr_cqe,rpcrdma_mr,mr_cqe); frwr_mr_done(wc,mr); complete(&mut (*mr).mr_linv_done); rpcrdma_flush_disconnect((*cq).cq_context,wc); }
unsafe fn frwr_wc_localinv_done(cq: *mut ib_cq, wc: *mut ib_wc) { let mr=container_of((*wc).wr_cqe,rpcrdma_mr,mr_cqe); let rep=(*(*mr).mr_req).rl_reply; smp_rmb(); if (*wc).status != IB_WC_SUCCESS { if !rep.is_null(){rpcrdma_unpin_rqst(rep);} rpcrdma_flush_disconnect((*cq).cq_context,wc); return; } frwr_mr_put(mr); rpcrdma_complete_rqst(rep); }

pub unsafe fn frwr_unmap_sync(r_xprt: *mut rpcrdma_xprt, req: *mut rpcrdma_req) { let ep=(*r_xprt).rx_ep; let mut first: *mut ib_send_wr=core::ptr::null_mut(); let mut last: *mut ib_send_wr=core::ptr::null_mut(); loop { let mr=rpcrdma_mr_pop(&mut (*req).rl_registered); if mr.is_null(){break;} last=&mut (*mr).mr_invwr; (*last).next=first; (*last).wr_cqe=&mut (*mr).mr_cqe; (*last).opcode=IB_WR_LOCAL_INV; (*last).send_flags=IB_SEND_SIGNALED; (*last).ex.invalidate_rkey=(*mr).mr_handle; (*last).wr_cqe.done=frwr_wc_localinv; first=last; } if !last.is_null(){(*last).wr_cqe.done=frwr_wc_localinv_wake;} let rc=ib_post_send((*ep).re_id.qp,first,core::ptr::null_mut()); if rc!=0 {trace_xprtrdma_post_linv_err(req,rc); rpcrdma_force_disconnect(ep);} }
pub unsafe fn frwr_unmap_async(r_xprt: *mut rpcrdma_xprt, req: *mut rpcrdma_req) { frwr_unmap_sync(r_xprt,req); }

pub unsafe fn frwr_wp_create(r_xprt: *mut rpcrdma_xprt) -> i32 {
    let ep=(*r_xprt).rx_ep; let mr=rpcrdma_mr_get(r_xprt); if mr.is_null(){return -EAGAIN;} (*mr).mr_req=core::ptr::null_mut(); (*ep).re_write_pad_mr=mr; sg_init_table((*mr).mr_sg,1); sg_set_page((*mr).mr_sg,virt_to_page((*ep).re_write_pad),XDR_UNIT,offset_in_page((*ep).re_write_pad)); (*mr).mr_dir=DMA_FROM_DEVICE; (*mr).mr_nents=1; let n=ib_dma_map_sg((*ep).re_id.device,(*mr).mr_sg,1,(*mr).mr_dir); if n==0 {(*ep).re_write_pad_mr=core::ptr::null_mut(); return -EIO;} (*mr).mr_device=(*ep).re_id.device; if ib_map_mr_sg((*mr).mr_ibmr,(*mr).mr_sg,n,core::ptr::null_mut(),PAGE_SIZE)!=n {frwr_mr_unmap(mr); (*ep).re_write_pad_mr=core::ptr::null_mut(); return -EIO;} ib_update_fast_reg_key((*mr).mr_ibmr,ib_inc_rkey((*mr).mr_ibmr.rkey)); (*mr).mr_handle=(*mr).mr_ibmr.rkey; (*mr).mr_length=(*mr).mr_ibmr.length; (*mr).mr_offset=(*mr).mr_ibmr.iova; (*mr).mr_cqe.done=frwr_wc_fastreg; (*mr).mr_regwr.wr.wr_cqe=&mut (*mr).mr_cqe; (*mr).mr_regwr.wr.opcode=IB_WR_REG_MR; ib_post_send((*ep).re_id.qp,&mut (*mr).mr_regwr.wr,core::ptr::null_mut())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
