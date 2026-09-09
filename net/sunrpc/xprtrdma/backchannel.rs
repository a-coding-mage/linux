// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2015-2020, Oracle and/or its affiliates.
 *
 * Support for reverse-direction RPCs on RPC/RDMA.
 */

// Linux/RPC-RDMA dependencies are supplied by the surrounding translation.

pub unsafe fn xprt_rdma_bc_setup(xprt: *mut rpc_xprt, reqs: libc::c_uint) -> libc::c_int {
    let r_xprt = rpcx_to_rdmax(xprt);
    (*(*r_xprt).rx_buf).rb_bc_srv_max_requests = RPCRDMA_BACKWARD_WRS >> 1;
    trace_xprtrdma_cb_setup(r_xprt, reqs);
    0
}

pub unsafe fn xprt_rdma_bc_maxpayload(xprt: *mut rpc_xprt) -> usize {
    let r_xprt = rpcx_to_rdmax(xprt);
    let ep = (*r_xprt).rx_ep;
    let mut maxmsg = core::cmp::min((*ep).re_inline_send, (*ep).re_inline_recv) as usize;
    maxmsg = core::cmp::min(maxmsg, PAGE_SIZE as usize);
    maxmsg - RPCRDMA_HDRLEN_MIN as usize
}

pub unsafe fn xprt_rdma_bc_max_slots(_xprt: *mut rpc_xprt) -> libc::c_uint {
    RPCRDMA_BACKWARD_WRS >> 1
}

unsafe fn rpcrdma_bc_marshal_reply(rqst: *mut rpc_rqst) -> libc::c_int {
    let r_xprt = rpcx_to_rdmax((*rqst).rq_xprt);
    let req = rpcr_to_rdmar(rqst);
    let mut p: *mut __be32;

    rpcrdma_set_xdrlen(&mut (*req).rl_hdrbuf, 0);
    xdr_init_encode(&mut (*req).rl_stream, &mut (*req).rl_hdrbuf,
                    rdmab_data((*req).rl_rdmabuf), rqst);
    p = xdr_reserve_space(&mut (*req).rl_stream, 28);
    if p.is_null() {
        return -EIO;
    }
    *p = (*rqst).rq_xid; p = p.add(1);
    *p = rpcrdma_version; p = p.add(1);
    *p = cpu_to_be32((*(*r_xprt).rx_buf).rb_bc_srv_max_requests); p = p.add(1);
    *p = rdma_msg; p = p.add(1);
    *p = xdr_zero; p = p.add(1);
    *p = xdr_zero; p = p.add(1);
    *p = xdr_zero;

    if rpcrdma_prepare_send_sges(r_xprt, req, RPCRDMA_HDRLEN_MIN,
                                 &mut (*rqst).rq_snd_buf, rpcrdma_noch_pullup) != 0 {
        return -EIO;
    }
    trace_xprtrdma_cb_reply(r_xprt, rqst);
    0
}

pub unsafe fn xprt_rdma_bc_send_reply(rqst: *mut rpc_rqst) -> libc::c_int {
    let xprt = (*rqst).rq_xprt;
    let r_xprt = rpcx_to_rdmax(xprt);
    let req = rpcr_to_rdmar(rqst);
    let rc;

    if !xprt_connected(xprt) { return -ENOTCONN; }
    if !xprt_request_get_cong(xprt, rqst) { return -EBADSLT; }
    rc = rpcrdma_bc_marshal_reply(rqst);
    if rc < 0 {
        if rc != -ENOTCONN { return rc; }
    } else if frwr_send(r_xprt, req) == 0 {
        return 0;
    }
    xprt_rdma_close(xprt);
    -ENOTCONN
}

pub unsafe fn xprt_rdma_bc_destroy(xprt: *mut rpc_xprt, _reqs: libc::c_uint) {
    let mut rqst: *mut rpc_rqst;
    let mut tmp: *mut rpc_rqst;
    spin_lock(&mut (*xprt).bc_pa_lock);
    list_for_each_entry_safe!(rqst, tmp, &mut (*xprt).bc_pa_list, rq_bc_pa_list, {
        list_del(&mut (*rqst).rq_bc_pa_list);
        spin_unlock(&mut (*xprt).bc_pa_lock);
        rpcrdma_req_destroy(rpcr_to_rdmar(rqst));
        spin_lock(&mut (*xprt).bc_pa_lock);
    });
    spin_unlock(&mut (*xprt).bc_pa_lock);
}

pub unsafe fn xprt_rdma_bc_free_rqst(rqst: *mut rpc_rqst) {
    let req = rpcr_to_rdmar(rqst);
    let rep = (*req).rl_reply;
    let xprt = (*rqst).rq_xprt;
    let r_xprt = rpcx_to_rdmax(xprt);
    rpcrdma_rep_put(&mut (*r_xprt).rx_buf, rep);
    (*req).rl_reply = core::ptr::null_mut();
    rpcrdma_req_put(req);
    xprt_put(xprt);
}

unsafe fn rpcrdma_bc_rqst_get(r_xprt: *mut rpcrdma_xprt) -> *mut rpc_rqst {
    let xprt = &mut (*r_xprt).rx_xprt as *mut rpc_xprt;
    let req;
    let rqst;
    let size;
    spin_lock(&mut (*xprt).bc_pa_lock);
    rqst = list_first_entry_or_null!(&mut (*xprt).bc_pa_list, rpc_rqst, rq_bc_pa_list);
    if !rqst.is_null() {
        list_del(&mut (*rqst).rq_bc_pa_list);
        spin_unlock(&mut (*xprt).bc_pa_lock);
        return rqst;
    }
    spin_unlock(&mut (*xprt).bc_pa_lock);
    if (*xprt).bc_alloc_count >= RPCRDMA_BACKWARD_WRS { return core::ptr::null_mut(); }
    size = core::cmp::min((*(*r_xprt).rx_ep).re_inline_recv as usize, PAGE_SIZE as usize);
    req = rpcrdma_req_create(r_xprt, size);
    if req.is_null() { return core::ptr::null_mut(); }
    if rpcrdma_req_setup(r_xprt, req) != 0 {
        rpcrdma_req_destroy(req);
        return core::ptr::null_mut();
    }
    (*xprt).bc_alloc_count += 1;
    let rqst = &mut (*req).rl_slot as *mut rpc_rqst;
    (*rqst).rq_xprt = xprt;
    __set_bit(RPC_BC_PA_IN_USE, &mut (*rqst).rq_bc_pa_state);
    xdr_buf_init(&mut (*rqst).rq_snd_buf, rdmab_data((*req).rl_sendbuf), size);
    kref_init(&mut (*req).rl_kref);
    rqst
}

pub unsafe fn rpcrdma_bc_receive_call(r_xprt: *mut rpcrdma_xprt, rep: *mut rpcrdma_rep) {
    let xprt = &mut (*r_xprt).rx_xprt as *mut rpc_xprt;
    let p = xdr_inline_decode(&mut (*rep).rr_stream, 0);
    let size = xdr_stream_remaining(&mut (*rep).rr_stream);
    let rqst = rpcrdma_bc_rqst_get(r_xprt);
    if rqst.is_null() {
        pr_warn!("RPC/RDMA backchannel overflow\n");
        xprt_force_disconnect(xprt);
        return;
    }
    (*rqst).rq_reply_bytes_recvd = 0;
    (*rqst).rq_xid = *p;
    (*rqst).rq_private_buf.len = size;
    let buf = &mut (*rqst).rq_rcv_buf;
    core::ptr::write_bytes(buf, 0, 1);
    (*buf).head[0].iov_base = p as *mut libc::c_void;
    (*buf).head[0].iov_len = size;
    (*buf).len = size;
    let req = rpcr_to_rdmar(rqst);
    (*req).rl_reply = rep;
    trace_xprtrdma_cb_call(r_xprt, rqst);
    xprt_enqueue_bc_request(rqst);
    (*r_xprt).rx_stats.bcall_count += 1;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
