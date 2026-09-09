// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) 2017, Microsoft Corporation.
 *
 *   Author(s): Long Li <longli@microsoft.com>
 */

// C headers and kernel-provided symbols are supplied by the surrounding translation.

pub const SMB_PORT: i32 = 445;
pub const SMBD_PORT: i32 = 5445;
pub const RDMA_RESOLVE_TIMEOUT: i32 = 5000;
pub const SMBD_NEGOTIATE_TIMEOUT: i32 = 120;
pub const KEEPALIVE_RECV_TIMEOUT: i32 = 5;
pub const SMBD_CM_RESPONDER_RESOURCES: i32 = 32;

pub static mut smbd_receive_credit_max: i32 = 255;
pub static mut smbd_send_credit_target: i32 = 255;
pub static mut smbd_max_send_size: i32 = 1364;
pub static mut smbd_max_fragmented_recv_size: i32 = (1364 * 255) / 2;
pub static mut smbd_max_receive_size: i32 = 1364;
pub static mut smbd_keep_alive_interval: i32 = 120;
pub static mut smbd_max_frmr_depth: i32 = 2048;
pub static mut rdma_readwrite_threshold: i32 = 4096;

pub const LOG_OUTGOING: u32 = 0x1;
pub const LOG_INCOMING: u32 = 0x2;
pub const LOG_READ: u32 = 0x4;
pub const LOG_WRITE: u32 = 0x8;
pub const LOG_RDMA_SEND: u32 = 0x10;
pub const LOG_RDMA_RECV: u32 = 0x20;
pub const LOG_KEEP_ALIVE: u32 = 0x40;
pub const LOG_RDMA_EVENT: u32 = 0x80;
pub const LOG_RDMA_MR: u32 = 0x100;
pub const ERR: u32 = 0;
pub const INFO: u32 = 1;
static mut smbd_logging_class: u32 = 0;
static mut smbd_logging_level: u32 = ERR;

unsafe fn smbd_logging_needed(_sc: *mut smbdirect_socket, _private_ptr: *mut core::ffi::c_void, lvl: u32, cls: u32) -> bool {
    lvl <= smbd_logging_level || (cls & smbd_logging_class) != 0
}

unsafe fn smbd_logging_vaprintf(_sc: *mut smbdirect_socket, _func: *const core::ffi::c_char,
                                _line: u32, _private_ptr: *mut core::ffi::c_void,
                                _lvl: u32, _cls: u32, _vaf: *mut va_format) {}

unsafe fn smbd_post_send_full_iter(sc: *mut smbdirect_socket, batch: *mut smbdirect_send_batch,
                                   iter: *mut iov_iter, mut remaining_data_length: u32) -> i32 {
    let mut bytes: i32 = 0;
    while iov_iter_count(iter) > 0 {
        let rc = smbdirect_connection_send_single_iter(sc, batch, iter, 0, remaining_data_length);
        if rc < 0 { return rc; }
        remaining_data_length = remaining_data_length.wrapping_sub(rc as u32);
        bytes = bytes.wrapping_add(rc);
    }
    bytes
}

pub unsafe fn smbd_destroy(server: *mut TCP_Server_Info) {
    let info = (*server).smbd_conn;
    if info.is_null() { return; }
    smbdirect_socket_release((*info).socket);
    kfree(info as *mut core::ffi::c_void);
    (*server).smbd_conn = core::ptr::null_mut();
}

pub unsafe fn smbd_reconnect(server: *mut TCP_Server_Info) -> i32 {
    if !(*server).smbd_conn.is_null() { smbd_destroy(server); }
    (*server).smbd_conn = smbd_get_connection(server, &mut (*server).dstaddr as *mut _ as *mut sockaddr);
    if !(*server).smbd_conn.is_null() { return 0; }
    -ENOENT
}

unsafe fn _smbd_get_connection(server: *mut TCP_Server_Info, dstaddr: *mut sockaddr, port: i32) -> *mut smbd_connection {
    let net = cifs_net_ns(server);
    let mut info: *mut smbd_connection;
    let mut sc: *mut smbdirect_socket = core::ptr::null_mut();
    let mut init_params: smbdirect_socket_parameters = core::mem::zeroed();
    let mut port_flags: u64 = 0;
    match port { SMBD_PORT => port_flags |= SMBDIRECT_FLAG_PORT_RANGE_ONLY_IW as u64,
                  SMB_PORT => port_flags |= SMBDIRECT_FLAG_PORT_RANGE_ONLY_IB as u64, _ => {} }
    init_params.flags = port_flags;
    init_params.resolve_addr_timeout_msec = RDMA_RESOLVE_TIMEOUT;
    init_params.resolve_route_timeout_msec = RDMA_RESOLVE_TIMEOUT;
    init_params.rdma_connect_timeout_msec = RDMA_RESOLVE_TIMEOUT;
    init_params.negotiate_timeout_msec = SMBD_NEGOTIATE_TIMEOUT * 1000;
    init_params.initiator_depth = 1;
    init_params.responder_resources = SMBD_CM_RESPONDER_RESOURCES;
    init_params.recv_credit_max = smbd_receive_credit_max;
    init_params.send_credit_target = smbd_send_credit_target;
    init_params.max_send_size = smbd_max_send_size;
    init_params.max_fragmented_recv_size = smbd_max_fragmented_recv_size;
    init_params.max_recv_size = smbd_max_receive_size;
    init_params.max_frmr_depth = smbd_max_frmr_depth;
    init_params.keepalive_interval_msec = smbd_keep_alive_interval * 1000;
    init_params.keepalive_timeout_msec = KEEPALIVE_RECV_TIMEOUT * 1000;
    info = kzalloc_obj();
    if info.is_null() { return core::ptr::null_mut(); }
    if smbdirect_socket_create_kern(net, &mut sc) != 0 { kfree(info as *mut _); return core::ptr::null_mut(); }
    smbdirect_socket_set_logging(sc, core::ptr::null_mut(), smbd_logging_needed, smbd_logging_vaprintf);
    if smbdirect_socket_set_initial_parameters(sc, &init_params) != 0 ||
       smbdirect_socket_set_kernel_settings(sc, IB_POLL_SOFTIRQ, GFP_KERNEL) != 0 {
        smbdirect_socket_release(sc); kfree(info as *mut _); return core::ptr::null_mut();
    }
    if (*dstaddr).sa_family == AF_INET6 { (*(dstaddr as *mut sockaddr_in6)).sin6_port = htons(port as u16); }
    else { (*(dstaddr as *mut sockaddr_in)).sin_port = htons(port as u16); }
    if smbdirect_connect_sync(sc, dstaddr) != 0 { smbdirect_socket_release(sc); kfree(info as *mut _); return core::ptr::null_mut(); }
    (*info).socket = sc;
    info
}

pub unsafe fn smbd_get_parameters(conn: *mut smbd_connection) -> *const smbdirect_socket_parameters {
    if (*conn).socket.is_null() { static ZERO: smbdirect_socket_parameters = smbdirect_socket_parameters::zero(); return &ZERO; }
    smbdirect_socket_get_current_parameters((*conn).socket)
}

pub unsafe fn smbd_get_connection(server: *mut TCP_Server_Info, dstaddr: *mut sockaddr) -> *mut smbd_connection {
    let mut port = SMBD_PORT;
    let ret = loop { let r = _smbd_get_connection(server, dstaddr, port); if !r.is_null() || port != SMBD_PORT { break r; } port = SMB_PORT; };
    if ret.is_null() { return core::ptr::null_mut(); }
    let sp = smbd_get_parameters(ret);
    (*server).rdma_readwrite_threshold = if rdma_readwrite_threshold > (*sp).max_fragmented_send_size { (*sp).max_fragmented_send_size } else { rdma_readwrite_threshold };
    ret
}

pub unsafe fn smbd_recv(info: *mut smbd_connection, msg: *mut msghdr) -> i32 {
    if !smbdirect_connection_is_connected((*info).socket) { return -ENOTCONN; }
    smbdirect_connection_recvmsg((*info).socket, msg, 0)
}

pub unsafe fn smbd_send(server: *mut TCP_Server_Info, num_rqst: i32, rqst_array: *mut smb_rqst) -> i32 {
    let info = (*server).smbd_conn;
    let sc = (*info).socket;
    let sp = smbd_get_parameters(info);
    if !smbdirect_connection_is_connected(sc) { return -EAGAIN; }
    let mut remaining: u32 = 0;
    for i in 0..num_rqst { remaining = remaining.wrapping_add(smb_rqst_len(server, rqst_array.add(i as usize))); }
    if remaining > (*sp).max_fragmented_send_size as u32 { return -EINVAL; }
    let mut storage: smbdirect_send_batch_storage = core::mem::zeroed();
    let batch = smbdirect_init_send_batch_storage(&mut storage, false, 0);
    let mut error = 0;
    for idx in 0..num_rqst {
        let rqst = rqst_array.add(idx as usize);
        let mut klen = 0;
        for i in 0..(*rqst).rq_nvec { klen += (*rqst).rq_iov.add(i as usize).read().iov_len; }
        let mut iter: iov_iter = core::mem::zeroed();
        iov_iter_kvec(&mut iter, ITER_SOURCE, (*rqst).rq_iov, (*rqst).rq_nvec, klen);
        let rc = smbd_post_send_full_iter(sc, batch, &mut iter, remaining);
        if rc < 0 { error = rc; break; }
        remaining = remaining.wrapping_sub(rc as u32);
        if iov_iter_count(&mut (*rqst).rq_iter) > 0 {
            let rc = smbd_post_send_full_iter(sc, batch, &mut (*rqst).rq_iter, remaining);
            if rc < 0 { error = rc; break; }
            remaining = remaining.wrapping_sub(rc as u32);
        }
    }
    let mut rc = smbdirect_connection_send_batch_flush(sc, batch, true);
    if rc == 0 && error != 0 { rc = error; }
    error = rc;
    rc = smbdirect_connection_send_wait_zero_pending(sc);
    if rc != 0 && error == 0 { error = -EAGAIN; }
    if error != 0 { error } else { 0 }
}

pub unsafe fn smbd_register_mr(info: *mut smbd_connection, iter: *mut iov_iter, writing: bool, need_invalidate: bool) -> *mut smbdirect_mr_io {
    if !smbdirect_connection_is_connected((*info).socket) { return core::ptr::null_mut(); }
    smbdirect_connection_register_mr_io((*info).socket, iter, writing, need_invalidate)
}

pub unsafe fn smbd_mr_fill_buffer_descriptor(mr: *mut smbdirect_mr_io, v1: *mut smbdirect_buffer_descriptor_v1) { smbdirect_mr_io_fill_buffer_descriptor(mr, v1); }
pub unsafe fn smbd_deregister_mr(mr: *mut smbdirect_mr_io) { smbdirect_connection_deregister_mr_io(mr); }

pub unsafe fn smbd_debug_proc_show(server: *mut TCP_Server_Info, m: *mut seq_file) {
    if !(*server).rdma { return; }
    if (*server).smbd_conn.is_null() { seq_puts(m, b"\nSMBDirect transport not available\0".as_ptr() as *const _); return; }
    smbdirect_connection_legacy_debug_proc_show((*server).smbd_conn.socket, (*server).rdma_readwrite_threshold, m);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
