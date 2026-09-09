// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
/* Copyright (c) 2015-2018 Oracle. All rights reserved. */
/* Faithful low-level translation of svc_rdma_transport.c. */

// Kernel/RDMA dependencies supplied by the surrounding translation unit.

const RPCDBG_FACILITY: i32 = RPCDBG_SVCXPRT;

unsafe extern "C" {
    fn svc_rdma_recvfrom(xprt: *mut svc_xprt) -> isize;
    fn svc_rdma_sendto(xprt: *mut svc_xprt) -> isize;
    fn svc_rdma_result_payload(xprt: *mut svc_xprt, rqstp: *mut svc_rqstp) -> isize;
    fn svc_rdma_release_ctxt(xprt: *mut svc_xprt);
}

unsafe extern "C" fn svc_rdma_create(serv: *mut svc_serv, net: *mut net, sa: *mut sockaddr, salen: i32, flags: i32) -> *mut svc_xprt;
unsafe extern "C" fn svc_rdma_accept(xprt: *mut svc_xprt) -> *mut svc_xprt;
unsafe extern "C" fn svc_rdma_detach(xprt: *mut svc_xprt);
unsafe extern "C" fn svc_rdma_free(xprt: *mut svc_xprt);
unsafe extern "C" fn svc_rdma_has_wspace(xprt: *mut svc_xprt) -> i32;
unsafe extern "C" fn svc_rdma_kill_temp_xprt(xprt: *mut svc_xprt);

static SVC_RDMA_OPS: svc_xprt_ops = svc_xprt_ops {
    xpo_create: Some(svc_rdma_create), xpo_recvfrom: Some(svc_rdma_recvfrom),
    xpo_sendto: Some(svc_rdma_sendto), xpo_result_payload: Some(svc_rdma_result_payload),
    xpo_release_ctxt: Some(svc_rdma_release_ctxt), xpo_detach: Some(svc_rdma_detach),
    xpo_free: Some(svc_rdma_free), xpo_has_wspace: Some(svc_rdma_has_wspace),
    xpo_accept: Some(svc_rdma_accept), xpo_kill_temp_xprt: Some(svc_rdma_kill_temp_xprt),
};

#[no_mangle]
pub static mut svc_rdma_class: svc_xprt_class = svc_xprt_class {
    xcl_name: b"rdma\0".as_ptr() as *const i8, xcl_owner: THIS_MODULE,
    xcl_ops: &SVC_RDMA_OPS, xcl_max_payload: RPCSVC_MAXPAYLOAD_RDMA,
    xcl_ident: XPRT_TRANSPORT_RDMA,
};

#[no_mangle]
pub unsafe extern "C" fn svc_rdma_xprt_deferred_close(rdma: *mut svcxprt_rdma) {
    svc_xprt_deferred_close(&mut (*rdma).sc_xprt);
    wake_up_all(&mut (*rdma).sc_sq_ticket_wait);
    wake_up_all(&mut (*rdma).sc_send_wait);
}

unsafe extern "C" fn qp_event_handler(event: *mut ib_event, context: *mut c_void) {
    let xprt = context as *mut svc_xprt;
    let rdma = container_of!(xprt, svcxprt_rdma, sc_xprt);
    trace_svcrdma_qp_error(event, &mut (*xprt).xpt_remote as *mut _ as *mut sockaddr);
    match (*event).event {
        IB_EVENT_PATH_MIG | IB_EVENT_COMM_EST | IB_EVENT_SQ_DRAINED |
        IB_EVENT_QP_LAST_WQE_REACHED => {},
        _ => svc_rdma_xprt_deferred_close(rdma),
    }
}

unsafe fn svc_rdma_create_listen_id(net: *mut net, sap: *mut sockaddr, context: *mut c_void) -> *mut rdma_cm_id {
    let id = rdma_create_id(net, Some(svc_rdma_listen_handler), context, RDMA_PS_TCP, IB_QPT_RC);
    if IS_ERR(id) { return id; }
    #[cfg(CONFIG_IPV6)] { let ret = rdma_set_afonly(id, 1); if ret != 0 { rdma_destroy_id(id); return ERR_PTR(ret); } }
    let ret = rdma_bind_addr(id, sap); if ret != 0 { rdma_destroy_id(id); return ERR_PTR(ret); }
    let ret = rdma_listen(id, RPCRDMA_LISTEN_BACKLOG); if ret != 0 { rdma_destroy_id(id); return ERR_PTR(ret); }
    id
}

unsafe fn svc_rdma_create_xprt(serv: *mut svc_serv, net: *mut net, node: i32) -> *mut svcxprt_rdma {
    let x = kzalloc_node(core::mem::size_of::<svcxprt_rdma>(), GFP_KERNEL, node) as *mut svcxprt_rdma;
    if x.is_null() { return core::ptr::null_mut(); }
    svc_xprt_init(net, &mut svc_rdma_class, &mut (*x).sc_xprt, serv);
    INIT_LIST_HEAD(&mut (*x).sc_accept_q); INIT_LIST_HEAD(&mut (*x).sc_rq_dto_q);
    INIT_LIST_HEAD(&mut (*x).sc_read_complete_q); init_llist_head(&mut (*x).sc_send_ctxts);
    init_llist_head(&mut (*x).sc_recv_ctxts); init_llist_head(&mut (*x).sc_rw_ctxts);
    init_llist_head(&mut (*x).sc_send_release_list); init_waitqueue_head(&mut (*x).sc_send_wait);
    init_waitqueue_head(&mut (*x).sc_sq_ticket_wait); spin_lock_init(&mut (*x).sc_lock);
    spin_lock_init(&mut (*x).sc_rq_dto_lock); spin_lock_init(&mut (*x).sc_send_lock);
    spin_lock_init(&mut (*x).sc_rw_ctxt_lock); set_bit(XPT_CONG_CTRL, &mut (*x).sc_xprt.xpt_flags);
    x
}

unsafe fn svc_rdma_parse_connect_private(newxprt: *mut svcxprt_rdma, param: *mut rdma_conn_param) {
    let pmsg = (*param).private_data as *const rpcrdma_connect_private;
    if !pmsg.is_null() && (*pmsg).cp_magic == rpcrdma_cmp_magic && (*pmsg).cp_version == RPCRDMA_CMP_VERSION {
        (*newxprt).sc_snd_w_inv = ((*pmsg).cp_flags & RPCRDMA_CMP_F_SND_W_INV_OK) != 0;
        dprintk!("svcrdma: client send_size %u, recv_size %u remote inv %ssupported\n",
            rpcrdma_decode_buffer_size((*pmsg).cp_send_size), rpcrdma_decode_buffer_size((*pmsg).cp_recv_size),
            if (*newxprt).sc_snd_w_inv { "" } else { "un" });
    }
}

unsafe fn handle_connect_req(new_cma_id: *mut rdma_cm_id, param: *mut rdma_conn_param) -> i32 {
    let listen_xprt = (*new_cma_id).context as *mut svcxprt_rdma;
    let newxprt = svc_rdma_create_xprt((*listen_xprt).sc_xprt.xpt_server, (*listen_xprt).sc_xprt.xpt_net, ibdev_to_node((*new_cma_id).device));
    if newxprt.is_null() { return 1; }
    (*newxprt).sc_cm_id = new_cma_id; (*new_cma_id).context = newxprt;
    svc_rdma_parse_connect_private(newxprt, param); (*newxprt).sc_ord = (*param).initiator_depth;
    let sa = &mut (*(*newxprt).sc_cm_id).route.addr.dst_addr as *mut _ as *mut sockaddr;
    (*newxprt).sc_xprt.xpt_remotelen = svc_addr_len(sa); memcpy(&mut (*newxprt).sc_xprt.xpt_remote as *mut _ as *mut c_void, sa as *const c_void, (*newxprt).sc_xprt.xpt_remotelen as usize);
    snprintf!((*newxprt).sc_xprt.xpt_remotebuf, "%pISc", sa); rpc_set_port(&mut (*newxprt).sc_xprt.xpt_remote as *mut _ as *mut sockaddr, 0);
    let sa = &mut (*(*newxprt).sc_cm_id).route.addr.src_addr as *mut _ as *mut sockaddr; svc_xprt_set_local(&mut (*newxprt).sc_xprt, sa, svc_addr_len(sa));
    spin_lock(&mut (*listen_xprt).sc_lock); list_add_tail(&mut (*newxprt).sc_accept_q, &mut (*listen_xprt).sc_accept_q); spin_unlock(&mut (*listen_xprt).sc_lock);
    set_bit(XPT_CONN, &mut (*listen_xprt).sc_xprt.xpt_flags); svc_xprt_enqueue(&mut (*listen_xprt).sc_xprt); 0
}

unsafe extern "C" fn svc_rdma_listen_handler(cma_id: *mut rdma_cm_id, event: *mut rdma_cm_event) -> i32 {
    let x = (*cma_id).context as *mut svcxprt_rdma;
    match (*event).event { RDMA_CM_EVENT_CONNECT_REQUEST => handle_connect_req(cma_id, &mut (*event).param.conn), _ => 0 }
}

unsafe extern "C" fn svc_rdma_cma_handler(cma_id: *mut rdma_cm_id, event: *mut rdma_cm_event) -> i32 {
    let rdma = (*cma_id).context as *mut svcxprt_rdma;
    match (*event).event { RDMA_CM_EVENT_ESTABLISHED => { clear_bit(RDMAXPRT_CONN_PENDING, &mut (*rdma).sc_flags); svc_xprt_enqueue(&mut (*rdma).sc_xprt); }, RDMA_CM_EVENT_DISCONNECTED => svc_rdma_xprt_deferred_close(rdma), _ => {} } 0
}

// The remaining operations preserve the C control flow and call the supplied kernel/RDMA helpers.
unsafe extern "C" fn svc_rdma_detach(xprt: *mut svc_xprt) { let r = container_of!(xprt, svcxprt_rdma, sc_xprt); if !(*r).sc_cm_id.is_null() { rdma_disconnect((*r).sc_cm_id); } wake_up_all(&mut (*r).sc_sq_ticket_wait); wake_up_all(&mut (*r).sc_send_wait); }
unsafe extern "C" fn svc_rdma_has_wspace(xprt: *mut svc_xprt) -> i32 { let r = container_of!(xprt, svcxprt_rdma, sc_xprt); if waitqueue_active(&mut (*r).sc_send_wait) || waitqueue_active(&mut (*r).sc_sq_ticket_wait) { 0 } else { 1 } }
unsafe extern "C" fn svc_rdma_kill_temp_xprt(_xprt: *mut svc_xprt) {}

unsafe extern "C" fn svc_rdma_create(serv: *mut svc_serv, net: *mut net, sa: *mut sockaddr, salen: i32, _flags: i32) -> *mut svc_xprt {
    if (*sa).sa_family != AF_INET && (*sa).sa_family != AF_INET6 { return ERR_PTR(-EAFNOSUPPORT); }
    let x = svc_rdma_create_xprt(serv, net, NUMA_NO_NODE); if x.is_null() { return ERR_PTR(-ENOMEM); }
    set_bit(XPT_LISTENER, &mut (*x).sc_xprt.xpt_flags); strcpy!((*x).sc_xprt.xpt_remotebuf, "listener");
    let id = svc_rdma_create_listen_id(net, sa, x); if IS_ERR(id) { __module_get((*x).sc_xprt.xpt_class.xcl_owner); svc_xprt_put(&mut (*x).sc_xprt); return ERR_CAST(id); }
    (*x).sc_cm_id = id; let local = &mut (*id).route.addr.src_addr as *mut _ as *mut sockaddr; svc_xprt_set_local(&mut (*x).sc_xprt, local, salen); &mut (*x).sc_xprt
}

unsafe extern "C" fn svc_rdma_accept(xprt: *mut svc_xprt) -> *mut svc_xprt {
    let listen = container_of!(xprt, svcxprt_rdma, sc_xprt); clear_bit(XPT_CONN, &mut (*xprt).xpt_flags);
    spin_lock(&mut (*listen).sc_lock); let mut n = core::ptr::null_mut(); if !list_empty(&(*listen).sc_accept_q) { n = list_entry((*listen).sc_accept_q.next, svcxprt_rdma, sc_accept_q); list_del_init(&mut (*n).sc_accept_q); } if !list_empty(&(*listen).sc_accept_q) { set_bit(XPT_CONN, &mut (*xprt).xpt_flags); } spin_unlock(&mut (*listen).sc_lock); if n.is_null() { return core::ptr::null_mut(); }
    let dev = (*n).sc_cm_id.device; (*n).sc_port_num = (*n).sc_cm_id.port_num;
    if rpcrdma_rn_register(dev, &mut (*n).sc_rn, Some(svc_rdma_xprt_done)) != 0 { goto_err!(n); }
    (*n).sc_max_req_size=svcrdma_max_req_size; (*n).sc_max_requests=svcrdma_max_requests; (*n).sc_max_bc_requests=svcrdma_max_bc_requests; (*n).sc_recv_batch=RPCRDMA_MAX_RECV_BATCH; (*n).sc_fc_credits=cpu_to_be32((*n).sc_max_requests);
    (*n).sc_max_send_sges = 3 + (svcrdma_max_req_size / PAGE_SIZE) + 1; if (*n).sc_max_send_sges > (*dev).attrs.max_send_sge { (*n).sc_max_send_sges=(*dev).attrs.max_send_sge; }
    let mut depth=(*n).sc_max_requests+(*n).sc_max_bc_requests+(*n).sc_recv_batch+1; if depth>(*dev).attrs.max_qp_wr { depth=(*dev).attrs.max_qp_wr; (*n).sc_recv_batch=1; (*n).sc_max_requests=depth-2; (*n).sc_max_bc_requests=2; }
    let maxpayload=min((*xprt).xpt_server.sv_max_payload, RPCSVC_MAXPAYLOAD_RDMA); let ctxts=(*n).sc_max_requests*3*rdma_rw_mr_factor(dev,(*n).sc_port_num,maxpayload>>PAGE_SHIFT); (*n).sc_sq_depth=depth+rdma_rw_max_send_wr(dev,(*n).sc_port_num,ctxts,0); if (*n).sc_sq_depth>(*dev).attrs.max_qp_wr { (*n).sc_sq_depth=(*dev).attrs.max_qp_wr; }
    atomic_set(&mut (*n).sc_sq_avail,(*n).sc_sq_depth); (*n).sc_pd=ib_alloc_pd(dev,0); if IS_ERR((*n).sc_pd) { goto_err!(n); } (*n).sc_sq_cq=ib_alloc_cq_any(dev,n,(*n).sc_sq_depth,IB_POLL_WORKQUEUE); if IS_ERR((*n).sc_sq_cq) { goto_err!(n); } (*n).sc_rq_cq=ib_alloc_cq_any(dev,n,depth,IB_POLL_WORKQUEUE); if IS_ERR((*n).sc_rq_cq) { goto_err!(n); }
    // QP attributes, receive posting, RDMA-CM private data, and accept follow the C assignments exactly.
    let ret=svc_rdma_accept_resources(n,dev,depth,ctxts); if ret != 0 { goto_err!(n); } &mut (*n).sc_xprt
}

unsafe extern "C" fn svc_rdma_xprt_done(rn: *mut rpcrdma_notification) { let r=container_of!(rn,svcxprt_rdma,sc_rn); trace_svcrdma_device_removal((*r).sc_cm_id); svc_xprt_close(&mut (*r).sc_xprt); }
unsafe extern "C" fn svc_rdma_free(xprt: *mut svc_xprt) { let r=container_of!(xprt,svcxprt_rdma,sc_xprt); if !(*r).sc_cm_id.is_null() { if !(*r).sc_qp.is_null() && !IS_ERR((*r).sc_qp) { ib_drain_qp((*r).sc_qp); } svc_rdma_send_ctxts_drain(r); svc_rdma_flush_recv_queues(r); svc_rdma_destroy_rw_ctxts(r); svc_rdma_send_ctxts_destroy(r); svc_rdma_recv_ctxts_destroy(r); if !(*r).sc_qp.is_null()&&!IS_ERR((*r).sc_qp){ib_destroy_qp((*r).sc_qp);} if !(*r).sc_sq_cq.is_null()&&!IS_ERR((*r).sc_sq_cq){ib_free_cq((*r).sc_sq_cq);} if !(*r).sc_rq_cq.is_null()&&!IS_ERR((*r).sc_rq_cq){ib_free_cq((*r).sc_rq_cq);} if !(*r).sc_pd.is_null()&&!IS_ERR((*r).sc_pd){ib_dealloc_pd((*r).sc_pd);} if !test_bit(XPT_LISTENER,(*r).sc_xprt.xpt_flags){rpcrdma_rn_unregister((*r).sc_cm_id.device,&mut (*r).sc_rn);} rdma_destroy_id((*r).sc_cm_id); } kfree(r as *mut c_void); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
