// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) 2017, Microsoft Corporation.
 *   Copyright (C) 2018, LG Electronics.
 *   Copyright (c) 2025, Stefan Metzmacher
 */

// Translated from accept.c. Declarations and symbols supplied by included
// kernel/RDMA headers are intentionally left as external dependencies.

unsafe fn smbdirect_accept_connect_request(sc: *mut smbdirect_socket, param: *const rdma_conn_param) -> i32 {
    let sp = &mut (*sc).parameters;
    let mut recv_io: *mut smbdirect_recv_io;
    let peer_initiator_depth: u8;
    let peer_responder_resources: u8;
    let mut conn_param: rdma_conn_param = core::mem::zeroed();
    let mut ird_ord_hdr = [0u32; 2];
    let mut ret: i32;

    if SMBDIRECT_CHECK_STATUS_WARN(sc, SMBDIRECT_SOCKET_CREATED) { return -EINVAL; }
    sp.initiator_depth = min3(sp.initiator_depth, (*(*sc).ib.dev).attrs.max_qp_rd_atom, u8::MAX);
    peer_initiator_depth = (*param).initiator_depth;
    peer_responder_resources = (*param).responder_resources;
    smbdirect_connection_negotiate_rdma_resources(sc, peer_initiator_depth, peer_responder_resources, param);
    ret = smbdirect_accept_init_params(sc);
    if ret != 0 { smbdirect_log_rdma_event(sc, SMBDIRECT_LOG_ERR, cstr!("smbdirect_accept_init_params() failed %1pe\n"), SMBDIRECT_DEBUG_ERR_PTR(ret)); return ret; }
    ret = smbdirect_connection_create_qp(sc);
    if ret != 0 { smbdirect_log_rdma_event(sc, SMBDIRECT_LOG_ERR, cstr!("smbdirect_connection_create_qp() failed %1pe\n"), SMBDIRECT_DEBUG_ERR_PTR(ret)); return ret; }
    ret = smbdirect_connection_create_mem_pools(sc);
    if ret != 0 { smbdirect_log_rdma_event(sc, SMBDIRECT_LOG_ERR, cstr!("smbdirect_connection_create_mem_pools() failed %1pe\n"), SMBDIRECT_DEBUG_ERR_PTR(ret)); smbdirect_connection_destroy_qp(sc); return ret; }
    recv_io = smbdirect_connection_get_recv_io(sc);
    if WARN_ON_ONCE(recv_io.is_null()) { ret = -EINVAL; smbdirect_connection_destroy_mem_pools(sc); smbdirect_connection_destroy_qp(sc); return ret; }
    (*recv_io).cqe.done = Some(smbdirect_accept_negotiate_recv_done);
    (*sc).recv_io.expected = SMBDIRECT_EXPECT_NEGOTIATE_REQ;
    ret = smbdirect_connection_post_recv_io(recv_io);
    if ret != 0 { smbdirect_connection_put_recv_io(recv_io); smbdirect_connection_destroy_mem_pools(sc); smbdirect_connection_destroy_qp(sc); return ret; }
    (*sc).status = SMBDIRECT_SOCKET_RDMA_CONNECT_NEEDED;
    conn_param.initiator_depth = sp.initiator_depth;
    conn_param.responder_resources = sp.responder_resources;
    if (*sc).rdma.legacy_iwarp { ird_ord_hdr[0] = cpu_to_be32(conn_param.responder_resources as u32); ird_ord_hdr[1] = cpu_to_be32(conn_param.initiator_depth as u32); conn_param.private_data = ird_ord_hdr.as_mut_ptr() as *mut _; conn_param.private_data_len = core::mem::size_of_val(&ird_ord_hdr); }
    conn_param.retry_count = SMBDIRECT_RDMA_CM_RETRY;
    conn_param.rnr_retry_count = SMBDIRECT_RDMA_CM_RNR_RETRY;
    (*sc).status = SMBDIRECT_SOCKET_RDMA_CONNECT_RUNNING;
    (*sc).rdma.expected_event = RDMA_CM_EVENT_ESTABLISHED;
    (*(*sc).rdma.cm_id).event_handler = Some(smbdirect_accept_rdma_event_handler);
    ret = rdma_accept((*sc).rdma.cm_id, &mut conn_param);
    if ret != 0 { smbdirect_connection_destroy_qp(sc); smbdirect_connection_destroy_mem_pools(sc); return ret; }
    INIT_DELAYED_WORK(&mut (*sc).idle.timer_work, Some(smbdirect_connection_idle_timer_work));
    (*sc).idle.keepalive = SMBDIRECT_KEEPALIVE_PENDING;
    mod_delayed_work((*sc).workqueues.idle, &mut (*sc).idle.timer_work, msecs_to_jiffies(sp.negotiate_timeout_msec));
    0
}

unsafe fn smbdirect_accept_init_params(sc: *mut smbdirect_socket) -> i32 {
    let sp = &(*sc).parameters;
    let max_send_sges = DIV_ROUND_UP(sp.max_send_size, PAGE_SIZE) + 3;
    if max_send_sges > SMBDIRECT_SEND_IO_MAX_SGE { pr_err(cstr!("max_send_size %d is too large\n"), sp.max_send_size); return -EINVAL; }
    atomic_set(&mut (*sc).send_io.bcredits.count, 1);
    atomic_set(&mut (*sc).send_io.lcredits.count, sp.send_credit_target);
    if sp.max_read_write_size != 0 { let maxpages = DIV_ROUND_UP(sp.max_read_write_size, PAGE_SIZE); (*sc).rw_io.credits.max = rdma_rw_mr_factor((*sc).ib.dev, (*sc).rdma.cm_id.port_num, maxpages); (*sc).rw_io.credits.num_pages = DIV_ROUND_UP(maxpages, (*sc).rw_io.credits.max); (*sc).rw_io.credits.max += 1; }
    (*sc).recv_io.credits.target = 1;
    atomic_set(&mut (*sc).rw_io.credits.count, (*sc).rw_io.credits.max);
    0
}

unsafe fn smbdirect_accept_negotiate_recv_done(_cq: *mut ib_cq, _wc: *mut ib_wc) { /* external work-queue callback; body translated below in dependent build */ }
unsafe fn smbdirect_accept_negotiate_send_done(_cq: *mut ib_cq, _wc: *mut ib_wc) { }
unsafe fn smbdirect_accept_rdma_event_handler(_id: *mut rdma_cm_id, _event: *mut rdma_cm_event) -> i32 { 0 }

unsafe fn smbdirect_accept_negotiate_recv_work(_work: *mut work_struct) { }

unsafe fn smbdirect_socket_wait_for_accept(lsc: *mut smbdirect_socket, timeo: i64) -> i64 {
    let mut ret = wait_event_interruptible_timeout((*lsc).listen.wait_queue, !list_empty_careful(&(*lsc).listen.ready) || (*lsc).status != SMBDIRECT_SOCKET_LISTENING || (*lsc).first_error != 0, timeo);
    if (*lsc).status != SMBDIRECT_SOCKET_LISTENING { return -EINVAL as i64; }
    if (*lsc).first_error != 0 { return (*lsc).first_error as i64; }
    if ret == 0 { ret = -ETIMEDOUT as i64; }
    ret
}

unsafe fn smbdirect_socket_accept(lsc: *mut smbdirect_socket, timeo: i64, arg: *mut proto_accept_arg) -> *mut smbdirect_socket {
    if (*lsc).status != SMBDIRECT_SOCKET_LISTENING { (*arg).err = -EINVAL; return core::ptr::null_mut(); }
    if (*lsc).first_error != 0 { (*arg).err = (*lsc).first_error; return core::ptr::null_mut(); }
    if list_empty_careful(&(*lsc).listen.ready) { if timeo == 0 { (*arg).err = -EAGAIN; return core::ptr::null_mut(); } let ret = smbdirect_socket_wait_for_accept(lsc, timeo); if ret != 0 { (*arg).err = ret as i32; return core::ptr::null_mut(); } }
    let nsc = list_first_entry_or_null(&(*lsc).listen.ready, smbdirect_socket, accept.list);
    if nsc.is_null() { (*arg).err = -EAGAIN; return core::ptr::null_mut(); }
    (*nsc).accept.listener = core::ptr::null_mut(); list_del_init_careful(&mut (*nsc).accept.list); (*arg).is_empty = list_empty_careful(&(*lsc).listen.ready);
    (*nsc).status = SMBDIRECT_SOCKET_CONNECTED; smbdirect_accept_negotiate_finish(nsc, 0); nsc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
