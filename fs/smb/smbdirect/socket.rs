// SPDX-License-Identifier: GPL-2.0-or-later
/* Copyright (C) 2017, Microsoft Corporation. Copyright (c) 2025, Stefan Metzmacher. */

// Translated from socket.c. Kernel and project-provided declarations are external dependencies.

unsafe fn smbdirect_frwr_is_supported(attrs: *const ib_device_attr) -> bool {
    if (*attrs).device_cap_flags & IB_DEVICE_MEM_MGT_EXTENSIONS == 0 { return false; }
    if (*attrs).max_fast_reg_page_list_len == 0 { return false; }
    true
}

unsafe fn smbdirect_socket_rdma_event_handler(id: *mut rdma_cm_id, event: *mut rdma_cm_event) -> c_int {
    let sc = (*id).context as *mut smbdirect_socket;
    let mut ret = -ESTALE;
    if (*event).event == RDMA_CM_EVENT_DEVICE_REMOVAL { ret = -ENETDOWN; }
    if IS_ERR(SMBDIRECT_DEBUG_ERR_PTR((*event).status)) { ret = (*event).status; }
    pr_err!("%s (first_error=%1pe, expected=%s) => event=%s status=%d => ret=%1pe\\n",
        smbdirect_socket_status_string((*sc).status), SMBDIRECT_DEBUG_ERR_PTR((*sc).first_error),
        rdma_event_msg((*sc).rdma.expected_event), rdma_event_msg((*event).event),
        (*event).status, SMBDIRECT_DEBUG_ERR_PTR(ret));
    WARN_ONCE!(true, "%s should not be called!\\n", __func__);
    (*sc).rdma.cm_id = core::ptr::null_mut();
    -ESTALE
}

unsafe fn smbdirect_socket_init_new(net: *mut net, sc: *mut smbdirect_socket) -> c_int {
    smbdirect_socket_init(sc);
    let id = rdma_create_id(net, smbdirect_socket_rdma_event_handler, sc as *mut c_void, RDMA_PS_TCP, IB_QPT_RC);
    if IS_ERR(id) { pr_err!("%s: rdma_create_id() failed %1pe\\n", __func__, id); return PTR_ERR(id); }
    let ret = rdma_set_afonly(id, 1);
    if ret != 0 { rdma_destroy_id(id); pr_err!("%s: rdma_set_afonly() failed %1pe\\n", __func__, SMBDIRECT_DEBUG_ERR_PTR(ret)); return ret; }
    (*sc).rdma.cm_id = id;
    INIT_WORK!(&mut (*sc).disconnect_work, smbdirect_socket_cleanup_work);
    0
}

unsafe fn smbdirect_socket_create_kern(net: *mut net, out: *mut *mut smbdirect_socket) -> c_int {
    let mut ret = -ENOMEM;
    let sc = kzalloc_obj!(smbdirect_socket);
    if sc.is_null() { return ret; }
    ret = smbdirect_socket_init_new(net, sc);
    if ret != 0 { kfree(sc); return ret; }
    kref_init!(&mut (*sc).refs.destroy);
    *out = sc; 0
}

unsafe fn smbdirect_socket_init_accepting(id: *mut rdma_cm_id, sc: *mut smbdirect_socket) -> c_int {
    smbdirect_socket_init(sc);
    (*sc).rdma.cm_id = id;
    (*id).context = sc as *mut c_void;
    (*id).event_handler = Some(smbdirect_socket_rdma_event_handler);
    (*sc).ib.dev = (*id).device;
    INIT_WORK!(&mut (*sc).disconnect_work, smbdirect_socket_cleanup_work);
    0
}

unsafe fn smbdirect_socket_create_accepting(id: *mut rdma_cm_id, out: *mut *mut smbdirect_socket) -> c_int {
    let mut ret = -ENOMEM;
    let sc = kzalloc_obj!(smbdirect_socket);
    if sc.is_null() { return ret; }
    ret = smbdirect_socket_init_accepting(id, sc);
    if ret != 0 { kfree(sc); return ret; }
    kref_init!(&mut (*sc).refs.destroy); *out = sc; 0
}

unsafe fn smbdirect_socket_set_initial_parameters(sc: *mut smbdirect_socket, sp: *const smbdirect_socket_parameters) -> c_int {
    WARN_ONCE!((*sc).status != SMBDIRECT_SOCKET_CREATED, "status=%s first_error=%1pe", smbdirect_socket_status_string((*sc).status), SMBDIRECT_DEBUG_ERR_PTR((*sc).first_error));
    if (*sc).status != SMBDIRECT_SOCKET_CREATED { return -EINVAL; }
    if (*sp).flags & !SMBDIRECT_FLAG_PORT_RANGE_MASK != 0 || (*sp).initiator_depth > U8_MAX || (*sp).responder_resources > U8_MAX { return -EINVAL; }
    if (*sp).flags & SMBDIRECT_FLAG_PORT_RANGE_ONLY_IB != 0 && (*sp).flags & SMBDIRECT_FLAG_PORT_RANGE_ONLY_IW != 0 { return -EINVAL; }
    if (*sp).flags & SMBDIRECT_FLAG_PORT_RANGE_ONLY_IB != 0 { rdma_restrict_node_type((*sc).rdma.cm_id, RDMA_NODE_IB_CA); }
    else if (*sp).flags & SMBDIRECT_FLAG_PORT_RANGE_ONLY_IW != 0 { rdma_restrict_node_type((*sc).rdma.cm_id, RDMA_NODE_RNIC); }
    (*sc).parameters = *sp; 0
}

unsafe fn smbdirect_socket_get_current_parameters(sc: *mut smbdirect_socket) -> *const smbdirect_socket_parameters { &(*sc).parameters }

unsafe fn smbdirect_socket_set_kernel_settings(sc: *mut smbdirect_socket, poll_ctx: ib_poll_context, gfp_mask: gfp_t) -> c_int {
    WARN_ONCE!((*sc).status != SMBDIRECT_SOCKET_CREATED, "status=%s first_error=%1pe", smbdirect_socket_status_string((*sc).status), SMBDIRECT_DEBUG_ERR_PTR((*sc).first_error));
    if (*sc).status != SMBDIRECT_SOCKET_CREATED { return -EINVAL; }
    (*sc).ib.poll_ctx = poll_ctx; (*sc).send_io.mem.gfp_mask = gfp_mask; (*sc).recv_io.mem.gfp_mask = gfp_mask; (*sc).rw_io.mem.gfp_mask = gfp_mask; 0
}

unsafe fn smbdirect_socket_set_logging(sc: *mut smbdirect_socket, private_ptr: *mut c_void, needed: Option<unsafe extern "C" fn(*mut smbdirect_socket,*mut c_void,c_uint,c_uint)->bool>, vaprintf: Option<unsafe extern "C" fn(*mut smbdirect_socket,*const c_char,c_uint,*mut c_void,c_uint,c_uint,*mut va_format)>) {
    (*sc).logging.private_ptr = private_ptr; (*sc).logging.needed = needed; (*sc).logging.vaprintf = vaprintf;
}

unsafe fn smbdirect_socket_wake_up_all(sc: *mut smbdirect_socket) {
    wake_up_all!(&mut (*sc).status_wait); wake_up_all!(&mut (*sc).listen.wait_queue);
    wake_up_all!(&mut (*sc).send_io.bcredits.wait_queue); wake_up_all!(&mut (*sc).send_io.lcredits.wait_queue);
    wake_up_all!(&mut (*sc).send_io.credits.wait_queue); wake_up_all!(&mut (*sc).send_io.pending.zero_wait_queue);
    wake_up_all!(&mut (*sc).recv_io.reassembly.wait_queue); wake_up_all!(&mut (*sc).rw_io.credits.wait_queue);
    wake_up_all!(&mut (*sc).mr_io.ready.wait_queue);
}

unsafe fn __smbdirect_socket_schedule_cleanup(sc: *mut smbdirect_socket, macro_name: *const c_char, lvl: c_uint, func: *const c_char, line: c_uint, error: c_int, force_status: *mut smbdirect_socket_status) {
    let mut was_first = false;
    if (*sc).first_error == 0 {
        ___smbdirect_log_generic(sc, func, line, lvl, SMBDIRECT_LOG_RDMA_EVENT, "%s(%1pe%s%s) called from %s in line=%u status=%s\\n", macro_name, SMBDIRECT_DEBUG_ERR_PTR(error), if !force_status.is_null(){", "}else{""}, if !force_status.is_null(){smbdirect_socket_status_string(*force_status)}else{""}, func, line, smbdirect_socket_status_string((*sc).status));
        (*sc).first_error = if error != 0 { error } else { -ECONNABORTED }; was_first = true;
    }
    disable_work!(&mut (*sc).connect.work); disable_work!(&mut (*sc).recv_io.posted.refill_work); disable_work!(&mut (*sc).idle.immediate_work); (*sc).idle.keepalive = SMBDIRECT_KEEPALIVE_NONE; disable_delayed_work!(&mut (*sc).idle.timer_work);
    if (*sc).listen.backlog != -1 { let mut flags=0; spin_lock_irqsave!(&mut (*sc).listen.lock, flags); list_splice_init!(&mut (*sc).listen.ready, &mut (*sc).listen.pending); list_for_each_entry_safe!(psc, tsc, &mut (*sc).listen.pending, accept.list, { smbdirect_socket_schedule_cleanup(psc, (*sc).first_error); }); spin_unlock_irqrestore!(&mut (*sc).listen.lock, flags); }
    match (*sc).status {
        SMBDIRECT_SOCKET_RESOLVE_ADDR_FAILED|SMBDIRECT_SOCKET_RESOLVE_ROUTE_FAILED|SMBDIRECT_SOCKET_RDMA_CONNECT_FAILED|SMBDIRECT_SOCKET_NEGOTIATE_FAILED|SMBDIRECT_SOCKET_ERROR|SMBDIRECT_SOCKET_DISCONNECTING|SMBDIRECT_SOCKET_DISCONNECTED|SMBDIRECT_SOCKET_DESTROYED => {},
        SMBDIRECT_SOCKET_RESOLVE_ADDR_NEEDED|SMBDIRECT_SOCKET_RESOLVE_ADDR_RUNNING => (*sc).status=SMBDIRECT_SOCKET_RESOLVE_ADDR_FAILED,
        SMBDIRECT_SOCKET_RESOLVE_ROUTE_NEEDED|SMBDIRECT_SOCKET_RESOLVE_ROUTE_RUNNING => (*sc).status=SMBDIRECT_SOCKET_RESOLVE_ROUTE_FAILED,
        SMBDIRECT_SOCKET_RDMA_CONNECT_NEEDED|SMBDIRECT_SOCKET_RDMA_CONNECT_RUNNING => (*sc).status=SMBDIRECT_SOCKET_RDMA_CONNECT_FAILED,
        SMBDIRECT_SOCKET_NEGOTIATE_NEEDED|SMBDIRECT_SOCKET_NEGOTIATE_RUNNING => (*sc).status=SMBDIRECT_SOCKET_NEGOTIATE_FAILED,
        SMBDIRECT_SOCKET_CREATED|SMBDIRECT_SOCKET_LISTENING => (*sc).status=SMBDIRECT_SOCKET_DISCONNECTED,
        SMBDIRECT_SOCKET_CONNECTED => (*sc).status=SMBDIRECT_SOCKET_ERROR,
    }
    if !force_status.is_null() && (was_first || *force_status > (*sc).status) { (*sc).status=*force_status; }
    smbdirect_socket_wake_up_all(sc); queue_work!((*sc).workqueues.cleanup, &mut (*sc).disconnect_work);
}

unsafe fn smbdirect_socket_cleanup_work(work: *mut work_struct) {
    let sc = container_of!(work, smbdirect_socket, disconnect_work); let mut flags=0;
    WARN_ON_ONCE!(in_interrupt());
    if (*sc).first_error == 0 { smbdirect_log_rdma_event(sc, SMBDIRECT_LOG_ERR, "%s called with first_error==0\\n", smbdirect_socket_status_string((*sc).status)); (*sc).first_error=-ECONNABORTED; }
    disable_work!(&mut (*sc).disconnect_work); disable_work!(&mut (*sc).connect.work); disable_work!(&mut (*sc).recv_io.posted.refill_work); disable_work!(&mut (*sc).idle.immediate_work); (*sc).idle.keepalive=SMBDIRECT_KEEPALIVE_NONE; disable_delayed_work!(&mut (*sc).idle.timer_work);
    if (*sc).listen.backlog != -1 { spin_lock_irqsave!(&mut (*sc).listen.lock, flags); list_splice_init!(&mut (*sc).listen.ready,&mut (*sc).listen.pending); list_for_each_entry_safe!(psc,tsc,&mut (*sc).listen.pending,accept.list,{smbdirect_socket_schedule_cleanup(psc,(*sc).first_error);}); spin_unlock_irqrestore!(&mut (*sc).listen.lock,flags); }
    match (*sc).status {
        SMBDIRECT_SOCKET_NEGOTIATE_NEEDED|SMBDIRECT_SOCKET_NEGOTIATE_RUNNING|SMBDIRECT_SOCKET_NEGOTIATE_FAILED|SMBDIRECT_SOCKET_CONNECTED|SMBDIRECT_SOCKET_ERROR => { (*sc).status=SMBDIRECT_SOCKET_DISCONNECTING; rdma_lock_handler((*sc).rdma.cm_id); rdma_disconnect((*sc).rdma.cm_id); rdma_unlock_handler((*sc).rdma.cm_id); },
        SMBDIRECT_SOCKET_CREATED|SMBDIRECT_SOCKET_LISTENING|SMBDIRECT_SOCKET_RESOLVE_ADDR_NEEDED|SMBDIRECT_SOCKET_RESOLVE_ADDR_RUNNING|SMBDIRECT_SOCKET_RESOLVE_ADDR_FAILED|SMBDIRECT_SOCKET_RESOLVE_ROUTE_NEEDED|SMBDIRECT_SOCKET_RESOLVE_ROUTE_RUNNING|SMBDIRECT_SOCKET_RESOLVE_ROUTE_FAILED|SMBDIRECT_SOCKET_RDMA_CONNECT_NEEDED|SMBDIRECT_SOCKET_RDMA_CONNECT_RUNNING|SMBDIRECT_SOCKET_RDMA_CONNECT_FAILED => (*sc).status=SMBDIRECT_SOCKET_DISCONNECTED,
        SMBDIRECT_SOCKET_DISCONNECTING|SMBDIRECT_SOCKET_DISCONNECTED|SMBDIRECT_SOCKET_DESTROYED => {},
    }
    smbdirect_socket_wake_up_all(sc);
}

unsafe fn smbdirect_socket_bind(sc:*mut smbdirect_socket, addr:*mut sockaddr)->c_int { if (*sc).status!=SMBDIRECT_SOCKET_CREATED{return -EINVAL;} rdma_bind_addr((*sc).rdma.cm_id,addr) }
unsafe fn smbdirect_socket_shutdown(sc:*mut smbdirect_socket) { smbdirect_socket_schedule_cleanup_lvl(sc,SMBDIRECT_LOG_INFO,-ESHUTDOWN); }

unsafe fn smbdirect_socket_destroy(sc:*mut smbdirect_socket) {
    if (*sc).status==SMBDIRECT_SOCKET_DESTROYED{return;}
    WARN_ONCE!((*sc).status!=SMBDIRECT_SOCKET_DISCONNECTED,"status=%s first_error=%1pe",smbdirect_socket_status_string((*sc).status),SMBDIRECT_DEBUG_ERR_PTR((*sc).first_error));
    smbdirect_socket_wake_up_all(sc); disable_work_sync!(&mut (*sc).disconnect_work); disable_work_sync!(&mut (*sc).connect.work); disable_work_sync!(&mut (*sc).recv_io.posted.refill_work); disable_work_sync!(&mut (*sc).idle.immediate_work); disable_delayed_work_sync!(&mut (*sc).idle.timer_work);
    if !(*sc).rdma.cm_id.is_null(){rdma_lock_handler((*sc).rdma.cm_id);} if !(*sc).ib.qp.is_null(){ib_drain_qp((*sc).ib.qp);}
    let mut flags=0; let mut all_list=LIST_HEAD!(); let mut pending_list=LIST_HEAD!();
    spin_lock_irqsave!(&mut (*sc).listen.lock,flags); list_splice_tail_init!(&mut (*sc).listen.ready,&mut pending_list); list_splice_tail_init!(&mut (*sc).listen.pending,&mut pending_list); spin_unlock_irqrestore!(&mut (*sc).listen.lock,flags);
    spin_lock_irqsave!(&mut (*sc).recv_io.reassembly.lock,flags); list_splice_tail_init!(&mut (*sc).recv_io.reassembly.list,&mut all_list); spin_unlock_irqrestore!(&mut (*sc).recv_io.reassembly.lock,flags);
    list_for_each_entry_safe!(recv_io,recv_tmp,&mut all_list,list,{smbdirect_connection_put_recv_io(recv_io);}); (*sc).recv_io.reassembly.data_length=0;
    smbdirect_connection_destroy_mr_list(sc); smbdirect_connection_destroy_qp(sc);
    if !(*sc).rdma.cm_id.is_null(){rdma_unlock_handler((*sc).rdma.cm_id); rdma_destroy_id((*sc).rdma.cm_id); (*sc).rdma.cm_id=core::ptr::null_mut();}
    list_for_each_entry_safe!(psc,tsc,&mut pending_list,accept.list,{list_del_init!(&mut (*psc).accept.list);(*psc).accept.listener=core::ptr::null_mut();smbdirect_socket_release(psc);}); smbdirect_connection_destroy_mem_pools(sc); (*sc).status=SMBDIRECT_SOCKET_DESTROYED;
}

unsafe fn smbdirect_socket_release_disconnect(kref:*mut kref){let sc=container_of!(kref,smbdirect_socket,refs.disconnect);smbdirect_socket_destroy_sync(sc);}
unsafe fn smbdirect_socket_release_destroy(kref:*mut kref){let sc=container_of!(kref,smbdirect_socket,refs.destroy);smbdirect_socket_destroy_sync(sc);kfree(sc);}
unsafe fn smbdirect_socket_release(sc:*mut smbdirect_socket){WARN_ON_ONCE!(kref_read(&(*sc).refs.disconnect)!=1);WARN_ON!(!kref_put(&mut (*sc).refs.disconnect,smbdirect_socket_release_disconnect));kref_put(&mut (*sc).refs.destroy,smbdirect_socket_release_destroy);}

unsafe fn smbdirect_socket_destroy_sync(sc:*mut smbdirect_socket) {
    smbdirect_log_rdma_event(sc,SMBDIRECT_LOG_INFO,"status=%s first_error=%1pe",smbdirect_socket_status_string((*sc).status),SMBDIRECT_DEBUG_ERR_PTR((*sc).first_error)); WARN_ON_ONCE!(in_interrupt()); disable_work!(&mut (*sc).disconnect_work);
    if (*sc).first_error==0 { smbdirect_socket_schedule_cleanup_lvl(sc,SMBDIRECT_LOG_INFO,-ESHUTDOWN); } disable_work_sync!(&mut (*sc).disconnect_work);
    if (*sc).status < SMBDIRECT_SOCKET_DISCONNECTING { smbdirect_socket_cleanup_work(&mut (*sc).disconnect_work); }
    if (*sc).status < SMBDIRECT_SOCKET_DISCONNECTED { wait_event!(&mut (*sc).status_wait, (*sc).status==SMBDIRECT_SOCKET_DISCONNECTED); }
    smbdirect_socket_destroy(sc);
}

unsafe fn smbdirect_socket_wait_for_credits(sc:*mut smbdirect_socket, expected_status:smbdirect_socket_status, unexpected_errno:c_int, waitq:*mut wait_queue_head_t, total_credits:*mut atomic_t, needed:c_int)->c_int {
    if WARN_ON_ONCE!(needed<0){return -EINVAL;} loop { if atomic_sub_return(needed,total_credits)>=0{return 0;} atomic_add(needed,total_credits); let ret=wait_event_interruptible!(waitq,atomic_read(total_credits)>=needed||(*sc).status!=expected_status); if (*sc).status!=expected_status{return unexpected_errno;} if ret<0{return ret;} }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
