// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) 2017, Microsoft Corporation.
 *   Copyright (C) 2018, LG Electronics.
 *   Copyright (c) 2025, Stefan Metzmacher
 */

// Translated from listen.c. Symbols and types supplied by internal.h and the
// kernel/RDMA environment remain external dependencies.

pub unsafe extern "C" fn smbdirect_socket_listen(
    sc: *mut smbdirect_socket,
    mut backlog: c_int,
) -> c_int {
    let mut ret: c_int;

    if backlog < 0 { return -EINVAL; }
    if backlog == 0 { backlog = 1; } /* use 1 as default for now */

    if (*sc).first_error != 0 { return -EINVAL; }
    if (*sc).status != SMBDIRECT_SOCKET_CREATED { return -EINVAL; }
    if WARN_ON_ONCE((*sc).rdma.cm_id.is_null()) { return -EINVAL; }

    if !(*(*sc).rdma.cm_id).device.is_null() {
        smbdirect_log_rdma_event(sc, SMBDIRECT_LOG_INFO,
            c"try to listen on addr: %pISpsfc dev: %.*s\n".as_ptr(),
            &(*(*sc).rdma.cm_id).route.addr.src_addr,
            IB_DEVICE_NAME_MAX,
            (*(*(*sc).rdma.cm_id).device).name);
    } else {
        smbdirect_log_rdma_event(sc, SMBDIRECT_LOG_INFO,
            c"try to listen on addr: %pISpsfc\n".as_ptr(),
            &(*(*sc).rdma.cm_id).route.addr.src_addr);
    }

    WARN_ON_ONCE((*sc).status != SMBDIRECT_SOCKET_CREATED);
    (*sc).status = SMBDIRECT_SOCKET_LISTENING;
    (*sc).rdma.expected_event = RDMA_CM_EVENT_CONNECT_REQUEST;
    rdma_lock_handler((*sc).rdma.cm_id);
    (*(*sc).rdma.cm_id).event_handler = Some(smbdirect_listen_rdma_event_handler);
    rdma_unlock_handler((*sc).rdma.cm_id);

    ret = rdma_listen((*sc).rdma.cm_id, backlog);
    if ret != 0 {
        (*sc).first_error = ret;
        (*sc).status = SMBDIRECT_SOCKET_DISCONNECTED;
        if !(*(*sc).rdma.cm_id).device.is_null() {
            smbdirect_log_rdma_event(sc, SMBDIRECT_LOG_INFO,
                c"listening failed %1pe on addr: %pISpsfc dev: %.*s\n".as_ptr(),
                SMBDIRECT_DEBUG_ERR_PTR(ret),
                &(*(*sc).rdma.cm_id).route.addr.src_addr,
                IB_DEVICE_NAME_MAX,
                (*(*(*sc).rdma.cm_id).device).name);
        } else {
            smbdirect_log_rdma_event(sc, SMBDIRECT_LOG_INFO,
                c"listening failed %1pe on addr: %pISpsfc\n".as_ptr(),
                SMBDIRECT_DEBUG_ERR_PTR(ret),
                &(*(*sc).rdma.cm_id).route.addr.src_addr);
        }
        return ret;
    }

    /*
     * This is a value > 0, checked above,
     * so we are able to use sc->listen.backlog == -1,
     * as indication that the socket was never
     * a listener.
     */
    (*sc).listen.backlog = backlog;

    if !(*(*sc).rdma.cm_id).device.is_null() {
        smbdirect_log_rdma_event(sc, SMBDIRECT_LOG_INFO,
            c"listening on addr: %pISpsfc dev: %.*s\n".as_ptr(),
            &(*(*sc).rdma.cm_id).route.addr.src_addr,
            IB_DEVICE_NAME_MAX,
            (*(*(*sc).rdma.cm_id).device).name);
    } else {
        smbdirect_log_rdma_event(sc, SMBDIRECT_LOG_INFO,
            c"listening on addr: %pISpsfc\n".as_ptr(),
            &(*(*sc).rdma.cm_id).route.addr.src_addr);
    }

    /* The rest happens async via smbdirect_listen_rdma_event_handler() */
    0
}

unsafe extern "C" fn smbdirect_new_rdma_event_handler(
    _new_id: *mut rdma_cm_id,
    event: *mut rdma_cm_event,
) -> c_int {
    let mut ret = -ESTALE;
    if (*event).event == RDMA_CM_EVENT_DEVICE_REMOVAL { ret = -ENETDOWN; }
    if IS_ERR(SMBDIRECT_DEBUG_ERR_PTR((*event).status)) { ret = (*event).status; }
    WARN_ONCE!(true, "{} should not be called! event={} status={} => ret={:p}",
        "smbdirect_new_rdma_event_handler", rdma_event_msg((*event).event),
        (*event).status, SMBDIRECT_DEBUG_ERR_PTR(ret));
    -ESTALE
}

unsafe extern "C" fn smbdirect_listen_rdma_event_handler(
    new_id: *mut rdma_cm_id,
    event: *mut rdma_cm_event,
) -> c_int {
    let lsc = (*new_id).context as *mut smbdirect_socket;
    let mut ret: c_int;
    if (*event).event == RDMA_CM_EVENT_CONNECT_REQUEST {
        (*new_id).context = core::ptr::null_mut();
        (*new_id).event_handler = Some(smbdirect_new_rdma_event_handler);
    } else { new_id = core::ptr::null_mut(); }
    WARN_ON_ONCE(in_interrupt());
    if (*event).status != 0 || (*event).event != (*lsc).rdma.expected_event {
        ret = -ECONNABORTED;
        if (*event).event == RDMA_CM_EVENT_DEVICE_REMOVAL { ret = -ENETDOWN; }
        if IS_ERR(SMBDIRECT_DEBUG_ERR_PTR((*event).status)) { ret = (*event).status; }
        smbdirect_log_rdma_event(lsc, SMBDIRECT_LOG_ERR,
            c"%s (first_error=%1pe, expected=%s) => event=%s status=%d => ret=%1pe\n".as_ptr(),
            smbdirect_socket_status_string((*lsc).status), SMBDIRECT_DEBUG_ERR_PTR((*lsc).first_error),
            rdma_event_msg((*lsc).rdma.expected_event), rdma_event_msg((*event).event),
            (*event).status, SMBDIRECT_DEBUG_ERR_PTR(ret));
        smbdirect_socket_schedule_cleanup(lsc, ret);
        return if !new_id.is_null() { ret } else { 0 };
    }
    smbdirect_log_rdma_event(lsc, SMBDIRECT_LOG_INFO,
        c"%s (first_error=%1pe) event=%s\n".as_ptr(),
        smbdirect_socket_status_string((*lsc).status), SMBDIRECT_DEBUG_ERR_PTR((*lsc).first_error),
        rdma_event_msg((*event).event));
    if (*lsc).first_error != 0 { return if !new_id.is_null() { (*lsc).first_error } else { 0 }; }
    match (*event).event {
        RDMA_CM_EVENT_CONNECT_REQUEST => {
            WARN_ON_ONCE((*lsc).status != SMBDIRECT_SOCKET_LISTENING);
            ret = smbdirect_listen_connect_request(lsc, new_id, event);
            if ret != 0 { return ret; }
            0
        },
        _ => {
            WARN_ON_ONCE((*lsc).rdma.expected_event != RDMA_CM_EVENT_CONNECT_REQUEST);
            smbdirect_socket_schedule_cleanup(lsc, -EINVAL);
            0
        }
    }
}

unsafe extern "C" fn smbdirect_listen_connect_request(
    lsc: *mut smbdirect_socket, new_id: *mut rdma_cm_id,
    event: *const rdma_cm_event,
) -> c_int {
    let lsp = &(*lsc).parameters;
    let mut nsc: *mut smbdirect_socket = core::ptr::null_mut();
    let mut flags: c_ulong = 0;
    let backlog = core::cmp::max(1usize, (*lsc).listen.backlog as usize);
    let psockets; let rsockets; let mut ret: c_int;
    if !smbdirect_frwr_is_supported(&(*(*new_id).device).attrs) { return -EPROTONOSUPPORT; }
    if ((*lsp).flags & SMBDIRECT_FLAG_PORT_RANGE_ONLY_IB) != 0 && !rdma_ib_or_roce((*new_id).device, (*new_id).port_num) { return -EPROTONOSUPPORT; }
    if ((*lsp).flags & SMBDIRECT_FLAG_PORT_RANGE_ONLY_IW) != 0 && !rdma_protocol_iwarp((*new_id).device, (*new_id).port_num) { return -EPROTONOSUPPORT; }
    spin_lock_irqsave(&(*lsc).listen.lock, &mut flags);
    psockets = list_count_nodes(&(*lsc).listen.pending);
    rsockets = list_count_nodes(&(*lsc).listen.ready);
    spin_unlock_irqrestore(&(*lsc).listen.lock, flags);
    if psockets > backlog || rsockets > backlog || psockets + rsockets > backlog { return -EBUSY; }
    ret = smbdirect_socket_create_accepting(new_id, &mut nsc); if ret != 0 { return ret; }
    (*nsc).logging = (*lsc).logging;
    ret = smbdirect_socket_set_initial_parameters(nsc, &(*lsc).parameters); if ret != 0 { (*nsc).ib.dev = core::ptr::null_mut(); (*nsc).rdma.cm_id = core::ptr::null_mut(); smbdirect_socket_release(nsc); return ret; }
    ret = smbdirect_socket_set_kernel_settings(nsc, (*lsc).ib.poll_ctx, (*lsc).send_io.mem.gfp_mask); if ret != 0 { (*nsc).ib.dev = core::ptr::null_mut(); (*nsc).rdma.cm_id = core::ptr::null_mut(); smbdirect_socket_release(nsc); return ret; }
    spin_lock_irqsave(&(*lsc).listen.lock, &mut flags); list_add_tail(&mut (*nsc).accept.list, &mut (*lsc).listen.pending); (*nsc).accept.listener = lsc; spin_unlock_irqrestore(&(*lsc).listen.lock, flags);
    ret = smbdirect_accept_connect_request(nsc, &(*event).param.conn); if ret != 0 {
        spin_lock_irqsave(&(*lsc).listen.lock, &mut flags);
        list_del_init(&mut (*nsc).accept.list); (*nsc).accept.listener = core::ptr::null_mut();
        spin_unlock_irqrestore(&(*lsc).listen.lock, flags);
        (*nsc).ib.dev = core::ptr::null_mut(); (*nsc).rdma.cm_id = core::ptr::null_mut(); smbdirect_socket_release(nsc); return ret;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
