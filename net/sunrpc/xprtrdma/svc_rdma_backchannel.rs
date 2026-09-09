// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2015-2018 Oracle.  All rights reserved.
 *
 * Support for reverse-direction RPCs on RPC/RDMA (server-side).
 */

// External Linux SunRPC/RDMA dependencies are supplied by the surrounding tree.

/**
 * svc_rdma_handle_bc_reply - Process incoming backchannel Reply
 * @rqstp: resources for handling the Reply
 * @rctxt: Received message
 */
pub unsafe fn svc_rdma_handle_bc_reply(
    rqstp: *mut svc_rqst,
    rctxt: *mut svc_rdma_recv_ctxt,
) {
    let sxprt: *mut svc_xprt = (*rqstp).rq_xprt;
    let xprt: *mut rpc_xprt = (*sxprt).xpt_bc_xprt;
    let r_xprt: *mut rpcrdma_xprt = rpcx_to_rdmax(xprt);
    let rcvbuf: *mut xdr_buf = &mut (*rqstp).rq_arg;
    let mut dst: *mut kvec;
    let src: *mut kvec = &mut (*rcvbuf).head[0];
    let rdma_resp: *mut __be32 = (*rctxt).rc_recv_buf;
    let req: *mut rpc_rqst;
    let mut credits: u32;

    spin_lock(&mut (*xprt).queue_lock);
    req = xprt_lookup_rqst(xprt, *rdma_resp);
    if req.is_null() {
        spin_unlock(&mut (*xprt).queue_lock);
        return;
    }

    dst = &mut (*req).rq_private_buf.head[0];
    memcpy(
        &mut (*req).rq_private_buf as *mut xdr_buf as *mut c_void,
        &(*req).rq_rcv_buf as *const xdr_buf as *const c_void,
        core::mem::size_of::<xdr_buf>(),
    );
    if (*dst).iov_len < (*src).iov_len {
        spin_unlock(&mut (*xprt).queue_lock);
        return;
    }
    memcpy((*dst).iov_base, (*src).iov_base, (*src).iov_len);
    xprt_pin_rqst(req);
    spin_unlock(&mut (*xprt).queue_lock);

    credits = be32_to_cpup(rdma_resp.add(2));
    if credits == 0 {
        credits = 1; // don't deadlock
    } else if credits > (*r_xprt).rx_buf.rb_bc_max_requests {
        credits = (*r_xprt).rx_buf.rb_bc_max_requests;
    }
    spin_lock(&mut (*xprt).transport_lock);
    (*xprt).cwnd = credits << RPC_CWNDSHIFT;
    spin_unlock(&mut (*xprt).transport_lock);

    spin_lock(&mut (*xprt).queue_lock);
    xprt_complete_rqst((*req).rq_task, (*rcvbuf).len);
    xprt_unpin_rqst(req);
    (*rcvbuf).len = 0;
    spin_unlock(&mut (*xprt).queue_lock);
}

/* Send a reverse-direction RPC Call.
 *
 * Caller holds the connection's mutex and has already marshaled
 * the RPC/RDMA request.
 *
 * This is similar to svc_rdma_send_reply_msg, but takes a struct
 * rpc_rqst instead, does not support chunks, and avoids blocking
 * memory allocation.
 *
 * XXX: There is still an opportunity to block in svc_rdma_send()
 * if there are no SQ entries to post the Send. This may occur if
 * the adapter has a small maximum SQ depth.
 */
unsafe fn svc_rdma_bc_sendto(
    rdma: *mut svcxprt_rdma,
    rqst: *mut rpc_rqst,
    sctxt: *mut svc_rdma_send_ctxt,
) -> c_int {
    let mut empty_pcl: svc_rdma_pcl;
    let ret: c_int;

    pcl_init(&mut empty_pcl);
    ret = svc_rdma_map_reply_msg(
        rdma,
        sctxt,
        &mut empty_pcl,
        &mut empty_pcl,
        &mut (*rqst).rq_snd_buf,
    );
    if ret < 0 {
        return -EIO;
    }

    /* Bump page refcnt so Send completion doesn't release
     * the rq_buffer before all retransmits are complete.
     */
    get_page(virt_to_page((*rqst).rq_buffer));
    (*sctxt).sc_send_wr.opcode = IB_WR_SEND;
    svc_rdma_post_send(rdma, sctxt)
}

/* Server-side transport endpoint wants a whole page for its send
 * buffer. The client RPC code constructs the RPC header in this
 * buffer before it invokes ->send_request.
 */
unsafe fn xprt_rdma_bc_allocate(task: *mut rpc_task) -> c_int {
    let rqst: *mut rpc_rqst = (*task).tk_rqstp;
    let size: usize = (*rqst).rq_callsize;
    let page: *mut page;

    if size > PAGE_SIZE {
        WARN_ONCE!(true, "svcrdma: large bc buffer request (size %zu)\n", size);
        return -EINVAL;
    }
    page = alloc_page(GFP_NOIO | __GFP_NOWARN);
    if page.is_null() {
        return -ENOMEM;
    }
    (*rqst).rq_buffer = page_address(page);
    (*rqst).rq_rbuffer = kmalloc((*rqst).rq_rcvsize, GFP_NOIO | __GFP_NOWARN);
    if (*rqst).rq_rbuffer.is_null() {
        put_page(page);
        return -ENOMEM;
    }
    0
}

unsafe fn xprt_rdma_bc_free(task: *mut rpc_task) {
    let rqst: *mut rpc_rqst = (*task).tk_rqstp;
    put_page(virt_to_page((*rqst).rq_buffer));
    kfree((*rqst).rq_rbuffer);
}

unsafe fn rpcrdma_bc_send_request(
    rdma: *mut svcxprt_rdma,
    rqst: *mut rpc_rqst,
) -> c_int {
    let xprt: *mut rpc_xprt = (*rqst).rq_xprt;
    let r_xprt: *mut rpcrdma_xprt = rpcx_to_rdmax(xprt);
    let ctxt: *mut svc_rdma_send_ctxt;
    let p: *mut __be32;
    let rc: c_int;

    ctxt = svc_rdma_send_ctxt_get(rdma);
    if ctxt.is_null() {
        return -ENOTCONN;
    }
    p = xdr_reserve_space(&mut (*ctxt).sc_stream, RPCRDMA_HDRLEN_MIN);
    if p.is_null() {
        svc_rdma_send_ctxt_put(rdma, ctxt);
        return -ENOTCONN;
    }
    *p.add(0) = (*rqst).rq_xid;
    *p.add(1) = rpcrdma_version;
    *p.add(2) = cpu_to_be32((*r_xprt).rx_buf.rb_bc_max_requests);
    *p.add(3) = rdma_msg;
    *p.add(4) = xdr_zero;
    *p.add(5) = xdr_zero;
    *p.add(6) = xdr_zero;

    (*rqst).rq_xtime = ktime_get();
    rc = svc_rdma_bc_sendto(rdma, rqst, ctxt);
    if rc != 0 {
        svc_rdma_send_ctxt_put(rdma, ctxt);
        return -ENOTCONN;
    }
    0
}

/** xprt_rdma_bc_send_request - Send a reverse-direction Call */
unsafe fn xprt_rdma_bc_send_request(rqst: *mut rpc_rqst) -> c_int {
    let sxprt: *mut svc_xprt = (*(*rqst).rq_xprt).bc_xprt;
    let rdma: *mut svcxprt_rdma = container_of!(sxprt, svcxprt_rdma, sc_xprt);
    let ret: c_int;

    if test_bit(XPT_DEAD, &(*sxprt).xpt_flags) {
        return -ENOTCONN;
    }
    ret = rpcrdma_bc_send_request(rdma, rqst);
    if ret == -ENOTCONN {
        svc_xprt_close(sxprt);
    }
    ret
}

unsafe fn xprt_rdma_bc_close(xprt: *mut rpc_xprt) {
    xprt_disconnect_done(xprt);
    (*xprt).cwnd = RPC_CWNDSHIFT;
}

unsafe fn xprt_rdma_bc_put(xprt: *mut rpc_xprt) {
    xprt_rdma_free_addresses(xprt);
    xprt_free(xprt);
}

static XPRT_RDMA_BC_PROCS: rpc_xprt_ops = rpc_xprt_ops {
    reserve_xprt: Some(xprt_reserve_xprt_cong),
    release_xprt: Some(xprt_release_xprt_cong),
    alloc_slot: Some(xprt_alloc_slot),
    free_slot: Some(xprt_free_slot),
    release_request: Some(xprt_release_rqst_cong),
    buf_alloc: Some(xprt_rdma_bc_allocate),
    buf_free: Some(xprt_rdma_bc_free),
    send_request: Some(xprt_rdma_bc_send_request),
    wait_for_reply_request: Some(xprt_wait_for_reply_request_def),
    close: Some(xprt_rdma_bc_close),
    destroy: Some(xprt_rdma_bc_put),
    print_stats: Some(xprt_rdma_print_stats),
};

static XPRT_RDMA_BC_TIMEOUT: rpc_timeout = rpc_timeout {
    to_initval: 60 * HZ,
    to_maxval: 60 * HZ,
};

/* It shouldn't matter if the number of backchannel session slots
 * doesn't match the number of RPC/RDMA credits. That just means
 * one or the other will have extra slots that aren't used.
 */
unsafe fn xprt_setup_rdma_bc(args: *mut xprt_create) -> *mut rpc_xprt {
    let xprt: *mut rpc_xprt;
    let new_xprt: *mut rpcrdma_xprt;

    if (*args).addrlen > core::mem::size_of_val(&(*xprt).addr) {
        return ERR_PTR(-EBADF);
    }
    xprt = xprt_alloc(
        (*args).net,
        core::mem::size_of::<rpcrdma_xprt>(),
        RPCRDMA_MAX_BC_REQUESTS,
        RPCRDMA_MAX_BC_REQUESTS,
    );
    if xprt.is_null() {
        return ERR_PTR(-ENOMEM);
    }
    (*xprt).timeout = &XPRT_RDMA_BC_TIMEOUT;
    xprt_set_bound(xprt);
    xprt_set_connected(xprt);
    (*xprt).bind_timeout = 0;
    (*xprt).reestablish_timeout = 0;
    (*xprt).idle_timeout = 0;
    (*xprt).prot = XPRT_TRANSPORT_BC_RDMA;
    (*xprt).ops = &XPRT_RDMA_BC_PROCS;
    memcpy(&mut (*xprt).addr as *mut _ as *mut c_void, (*args).dstaddr, (*args).addrlen);
    (*xprt).addrlen = (*args).addrlen;
    xprt_rdma_format_addresses(xprt, &mut (*xprt).addr as *mut _ as *mut sockaddr);
    (*xprt).resvport = 0;
    (*xprt).max_payload = xprt_rdma_max_inline_read;
    new_xprt = rpcx_to_rdmax(xprt);
    (*new_xprt).rx_buf.rb_bc_max_requests = (*xprt).max_reqs;
    xprt_get(xprt);
    (*(*args).bc_xprt).xpt_bc_xprt = xprt;
    (*xprt).bc_xprt = (*args).bc_xprt;
    /* Final put for backchannel xprt is in __svc_rdma_free */
    xprt_get(xprt);
    xprt
}

pub static mut xprt_rdma_bc: xprt_class = xprt_class {
    list: LIST_HEAD_INIT!(xprt_rdma_bc.list),
    name: "rdma backchannel",
    owner: THIS_MODULE,
    ident: XPRT_TRANSPORT_BC_RDMA,
    setup: Some(xprt_setup_rdma_bc),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
