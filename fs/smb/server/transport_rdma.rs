// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) 2017, Microsoft Corporation.
 *   Copyright (C) 2018, LG Electronics.
 *
 *   Author(s): Long Li <longli@microsoft.com>,
 *      Hyunchul Lee <hyc.lee@gmail.com>
 */

// C dependencies supplied by the surrounding kernel/SMB implementation.

const SMB_DIRECT_PORT_IWARP: i32 = 5445;
const SMB_DIRECT_PORT_INFINIBAND: i32 = 445;
const SMB_DIRECT_NEGOTIATE_TIMEOUT: i32 = 5;
const SMB_DIRECT_KEEPALIVE_SEND_INTERVAL: i32 = 120;
const SMB_DIRECT_KEEPALIVE_RECV_TIMEOUT: i32 = 5;
const SMB_DIRECT_CM_INITIATOR_DEPTH: i32 = 8;

static mut smb_direct_receive_credit_max: i32 = 255;
static mut smb_direct_send_credit_target: i32 = 255;
static mut smb_direct_max_send_size: i32 = 1364;
static mut smb_direct_max_fragmented_recv_size: i32 = (1364 * 255) / 2;
static mut smb_direct_max_receive_size: i32 = 1364;
static mut smb_direct_max_read_write_size: i32 = SMBD_DEFAULT_IOSIZE;
static mut smb_direct_enabled: bool = false;

#[repr(C)]
struct smb_direct_listener {
    port: i32,
    thread: *mut task_struct,
    socket: *mut smbdirect_socket,
}

static mut smb_direct_ib_listener: smb_direct_listener = smb_direct_listener { port: 0, thread: core::ptr::null_mut(), socket: core::ptr::null_mut() };
static mut smb_direct_iw_listener: smb_direct_listener = smb_direct_listener { port: 0, thread: core::ptr::null_mut(), socket: core::ptr::null_mut() };

#[repr(C)]
struct smb_direct_transport {
    transport: ksmbd_transport,
    socket: *mut smbdirect_socket,
}

unsafe fn smb_direct_logging_needed(_sc: *mut smbdirect_socket, _private_ptr: *mut core::ffi::c_void, lvl: u32, cls: u32) -> bool {
    if lvl <= SMBDIRECT_LOG_ERR { return true; }
    if lvl > SMBDIRECT_LOG_INFO { return false; }
    match cls {
        SMBDIRECT_LOG_RDMA_EVENT | SMBDIRECT_LOG_RDMA_SEND | SMBDIRECT_LOG_RDMA_RECV |
        SMBDIRECT_LOG_WRITE | SMBDIRECT_LOG_READ | SMBDIRECT_LOG_NEGOTIATE |
        SMBDIRECT_LOG_OUTGOING | SMBDIRECT_LOG_RDMA_RW | SMBDIRECT_LOG_RDMA_MR => true,
        SMBDIRECT_LOG_KEEP_ALIVE | SMBDIRECT_LOG_INCOMING => false,
        _ => true,
    }
}

unsafe fn smb_direct_logging_vaprintf(_sc: *mut smbdirect_socket, _func: *const i8, _line: u32, _private_ptr: *mut core::ffi::c_void, lvl: u32, _cls: u32, vaf: *mut va_format) {
    if lvl <= SMBDIRECT_LOG_ERR { pr_err!("%pV", vaf); } else { ksmbd_debug!(RDMA, "%pV", vaf); }
}

unsafe fn alloc_transport(sc: *mut smbdirect_socket) -> *mut smb_direct_transport {
    let t = kzalloc_obj::<smb_direct_transport>(KSMBD_DEFAULT_GFP);
    if t.is_null() { return core::ptr::null_mut(); }
    (*t).socket = sc;
    let conn = ksmbd_conn_alloc();
    if conn.is_null() { kfree(t as *mut core::ffi::c_void); return core::ptr::null_mut(); }
    down_write(&mut conn_list_lock);
    hash_add(&mut conn_list, &mut (*conn).hlist, 0);
    up_write(&mut conn_list_lock);
    (*conn).transport = &mut (*t).transport;
    (*t).transport.conn = conn;
    (*t).transport.ops = &ksmbd_smb_direct_transport_ops;
    t
}

unsafe fn smb_direct_free_transport(kt: *mut ksmbd_transport) {
    let t = SMBD_TRANS(kt);
    smbdirect_socket_release((*t).socket);
    kfree(t as *mut core::ffi::c_void);
}

unsafe fn free_transport(t: *mut smb_direct_transport) {
    smbdirect_socket_shutdown((*t).socket);
    ksmbd_conn_free((*t).transport.conn);
}

unsafe fn smb_direct_read(t: *mut ksmbd_transport, buf: *mut i8, size: u32, _unused: i32) -> i32 {
    let st = SMBD_TRANS(t); let sc = (*st).socket;
    let mut msg: msghdr = core::mem::zeroed(); msg.msg_flags = 0;
    let mut iov = kvec { iov_base: buf as *mut core::ffi::c_void, iov_len: size as usize };
    iov_iter_kvec(&mut msg.msg_iter, ITER_DEST, &mut iov, 1, size as usize);
    let mut ret = smbdirect_connection_recvmsg(sc, &mut msg, 0);
    if ret == -ERESTARTSYS { ret = -EINTR; } ret
}

unsafe fn smb_direct_writev(t: *mut ksmbd_transport, tx: *const ksmbd_transport_write) -> i32 {
    let sc = (*SMBD_TRANS(t)).socket; let mut iter: iov_iter = core::mem::zeroed();
    iov_iter_kvec(&mut iter, ITER_SOURCE, (*tx).iov, (*tx).iov_cnt, (*tx).size);
    smbdirect_connection_send_iter(sc, &mut iter, 0, (*tx).need_invalidate_rkey, (*tx).remote_key)
}

unsafe fn smb_direct_rdma_write(t: *mut ksmbd_transport, buf: *mut core::ffi::c_void, buflen: u32, desc: *mut smbdirect_buffer_descriptor_v1, desc_len: u32) -> i32 { smbdirect_connection_rdma_xmit((*SMBD_TRANS(t)).socket, buf, buflen, desc, desc_len, false) }
unsafe fn smb_direct_rdma_read(t: *mut ksmbd_transport, buf: *mut core::ffi::c_void, buflen: u32, desc: *mut smbdirect_buffer_descriptor_v1, desc_len: u32) -> i32 { smbdirect_connection_rdma_xmit((*SMBD_TRANS(t)).socket, buf, buflen, desc, desc_len, true) }

unsafe fn smb_direct_disconnect(t: *mut ksmbd_transport) { let st = SMBD_TRANS(t); ksmbd_debug!(RDMA, "Disconnecting sc=%p\n", (*st).socket); free_transport(st); }
unsafe fn smb_direct_shutdown(t: *mut ksmbd_transport) { let st = SMBD_TRANS(t); ksmbd_debug!(RDMA, "smb-direct shutdown sc=%p\n", (*st).socket); smbdirect_socket_shutdown((*st).socket); }

unsafe fn smb_direct_listener_destroy(listener: *mut smb_direct_listener) {
    if !(*listener).socket.is_null() { smbdirect_socket_shutdown((*listener).socket); }
    if !(*listener).thread.is_null() { let ret = kthread_stop((*listener).thread); if ret != 0 { pr_err!("failed to stop forker thread\n"); } (*listener).thread = core::ptr::null_mut(); }
    if !(*listener).socket.is_null() { smbdirect_socket_release((*listener).socket); (*listener).socket = core::ptr::null_mut(); }
    (*listener).port = 0;
}

unsafe fn smb_direct_listener_kthread_fn(p: *mut core::ffi::c_void) -> i32 {
    let listener = p as *mut smb_direct_listener;
    while !kthread_should_stop() {
        if (*listener).socket.is_null() { break; }
        let mut arg = proto_accept_arg { err: -EINVAL };
        let client = smbdirect_socket_accept((*listener).socket, MAX_SCHEDULE_TIMEOUT, &mut arg);
        if client.is_null() && arg.err == -EINVAL { break; }
        if client.is_null() { continue; }
        ksmbd_debug!(CONN, "connect success: accepted new connection\n");
        let _ = smb_direct_new_connection(listener, client);
    }
    ksmbd_debug!(CONN, "releasing socket\n"); 0
}

unsafe fn smb_direct_new_connection(listener: *mut smb_direct_listener, client_sc: *mut smbdirect_socket) -> i32 {
    let t = alloc_transport(client_sc); if t.is_null() { smbdirect_socket_release(client_sc); return -ENOMEM; }
    let handler = kthread_run(ksmbd_conn_handler_loop, (*t).transport.conn as *mut core::ffi::c_void, "ksmbd:r%u", (*listener).port);
    if is_err(handler) { let ret = ptr_err(handler); pr_err!("Can't start thread\n"); free_transport(t); return ret; } 0
}

unsafe fn smb_direct_listen(listener: *mut smb_direct_listener, port: i32) -> i32 {
    let mut port_flags: u64 = 0;
    match port { SMB_DIRECT_PORT_IWARP => port_flags |= SMBDIRECT_FLAG_PORT_RANGE_ONLY_IW, SMB_DIRECT_PORT_INFINIBAND => port_flags |= SMBDIRECT_FLAG_PORT_RANGE_ONLY_IB, _ => { pr_err!("unsupported smbdirect port=%d!\n", port); return -ENODEV; } }
    let mut sc: *mut smbdirect_socket = core::ptr::null_mut();
    let mut ret = smbdirect_socket_create_kern(current_ns_net(), &mut sc); if ret != 0 { pr_err!("smbdirect_socket_create_kern() failed: %d\n", ret); return ret; }
    let mut p: smbdirect_socket_parameters = core::mem::zeroed();
    p.flags |= port_flags; p.negotiate_timeout_msec = SMB_DIRECT_NEGOTIATE_TIMEOUT * 1000; p.initiator_depth = SMB_DIRECT_CM_INITIATOR_DEPTH; p.responder_resources = 1;
    p.recv_credit_max = smb_direct_receive_credit_max; p.send_credit_target = smb_direct_send_credit_target; p.max_send_size = smb_direct_max_send_size; p.max_fragmented_recv_size = smb_direct_max_fragmented_recv_size; p.max_recv_size = smb_direct_max_receive_size; p.max_read_write_size = smb_direct_max_read_write_size; p.keepalive_interval_msec = SMB_DIRECT_KEEPALIVE_SEND_INTERVAL * 1000; p.keepalive_timeout_msec = SMB_DIRECT_KEEPALIVE_RECV_TIMEOUT * 1000;
    smbdirect_socket_set_logging(sc, core::ptr::null_mut(), smb_direct_logging_needed, smb_direct_logging_vaprintf);
    ret = smbdirect_socket_set_initial_parameters(sc, &mut p); if ret != 0 { goto_err(listener, sc); return ret; }
    ret = smbdirect_socket_set_kernel_settings(sc, IB_POLL_WORKQUEUE, KSMBD_DEFAULT_GFP); if ret != 0 { goto_err(listener, sc); return ret; }
    let sin = sockaddr_in { sin_family: AF_INET, sin_addr: in_addr { s_addr: htonl(INADDR_ANY) }, sin_port: htons(port as u16) };
    ret = smbdirect_socket_bind(sc, &sin as *const _ as *mut sockaddr); if ret != 0 { goto_err(listener, sc); return ret; }
    ret = smbdirect_socket_listen(sc, 10); if ret != 0 { goto_err(listener, sc); return ret; }
    (*listener).port = port; (*listener).socket = sc;
    let thread = kthread_run(smb_direct_listener_kthread_fn, listener as *mut core::ffi::c_void, "ksmbd-smbdirect-listener-%u", port); if is_err(thread) { ret = ptr_err(thread); smb_direct_listener_destroy(listener); return ret; }
    (*listener).thread = thread; 0
}

unsafe fn goto_err(listener: *mut smb_direct_listener, sc: *mut smbdirect_socket) { smbdirect_socket_shutdown(sc); smbdirect_socket_release(sc); (*listener).socket = core::ptr::null_mut(); }

pub unsafe fn init_smbd_max_io_size(mut sz: u32) { sz = clamp_val(sz, SMBD_MIN_IOSIZE, SMBD_MAX_IOSIZE); smb_direct_max_read_write_size = sz as i32; }
pub unsafe fn get_smbd_max_read_write_size(kt: *mut ksmbd_transport) -> u32 { if (*kt).ops != &ksmbd_smb_direct_transport_ops { return 0; } smbdirect_socket_get_current_parameters((*SMBD_TRANS(kt)).socket).max_read_write_size }
pub unsafe fn ksmbd_rdma_init() -> i32 { let ret = smb_direct_listen(&mut smb_direct_ib_listener, SMB_DIRECT_PORT_INFINIBAND); if ret != 0 { ksmbd_rdma_stop_listening(); return ret; } smb_direct_listen(&mut smb_direct_iw_listener, SMB_DIRECT_PORT_IWARP) }
pub unsafe fn ksmbd_rdma_stop_listening() { smb_direct_enabled = false; smb_direct_listener_destroy(&mut smb_direct_ib_listener); smb_direct_listener_destroy(&mut smb_direct_iw_listener); }
pub unsafe fn ksmbd_rdma_enabled() -> bool { smb_direct_enabled }
pub unsafe fn ksmbd_rdma_capable_netdev(netdev: *mut net_device) -> bool { smbdirect_netdev_rdma_capable_node_type(netdev) != RDMA_NODE_UNSPECIFIED }

static ksmbd_smb_direct_transport_ops: ksmbd_transport_ops = ksmbd_transport_ops { disconnect: smb_direct_disconnect, shutdown: smb_direct_shutdown, writev: smb_direct_writev, read: smb_direct_read, rdma_read: smb_direct_rdma_read, rdma_write: smb_direct_rdma_write, free_transport: smb_direct_free_transport };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
