// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) 2016 Namjae Jeon <linkinjeon@kernel.org>
 *   Copyright (C) 2018 Samsung Electronics Co., Ltd.
 */

// C dependencies supplied by the surrounding kernel/ksmbd translation.

const IFACE_STATE_DOWN: i32 = 1 << 0;
const IFACE_STATE_CONFIGURED: i32 = 1 << 1;

static mut active_num_conn: atomic_t = atomic_t::new(0);

#[repr(C)]
struct interface {
    ksmbd_kthread: *mut task_struct,
    ksmbd_socket: *mut socket,
    entry: list_head,
    name: *mut i8,
    state: i32,
}

static mut iface_list: list_head = LIST_HEAD_INIT;
static mut bind_additional_ifaces: i32 = 0;

#[repr(C)]
struct tcp_transport {
    transport: ksmbd_transport,
    sock: *mut socket,
    iov: *mut kvec,
    nr_iov: u32,
}

static ksmbd_tcp_transport_ops: ksmbd_transport_ops = ksmbd_transport_ops {
    read: Some(ksmbd_tcp_read), writev: Some(ksmbd_tcp_writev),
    disconnect: Some(ksmbd_tcp_disconnect), shutdown: Some(ksmbd_tcp_shutdown),
    free_transport: Some(ksmbd_tcp_free_transport),
};

unsafe fn ksmbd_tcp_nodelay(sock: *mut socket) { tcp_sock_set_nodelay((*sock).sk); }
unsafe fn ksmbd_tcp_reuseaddr(sock: *mut socket) { sock_set_reuseaddr((*sock).sk); }

unsafe fn alloc_transport(client_sk: *mut socket) -> *mut tcp_transport {
    let t = kzalloc_obj::<tcp_transport>(KSMBD_DEFAULT_GFP);
    if t.is_null() { return core::ptr::null_mut(); }
    (*t).sock = client_sk;
    let conn = ksmbd_conn_alloc();
    if conn.is_null() { kfree(t as *mut core::ffi::c_void); return core::ptr::null_mut(); }
    #[cfg(CONFIG_IPV6)]
    if (*(*client_sk).sk).sk_family == AF_INET6 {
        memcpy(&mut (*conn).inet6_addr as *mut _, &(*(*client_sk).sk).sk_v6_daddr as *const _, 16);
        (*conn).inet_hash = ipv6_addr_hash(&(*(*client_sk).sk).sk_v6_daddr);
    } else {
        (*conn).inet_addr = inet_sk((*client_sk).sk).inet_daddr;
        (*conn).inet_hash = ipv4_addr_hash(inet_sk((*client_sk).sk).inet_daddr);
    }
    #[cfg(not(CONFIG_IPV6))]
    { (*conn).inet_addr = inet_sk((*client_sk).sk).inet_daddr; (*conn).inet_hash = ipv4_addr_hash(inet_sk((*client_sk).sk).inet_daddr); }
    down_write(&mut conn_list_lock); hash_add(&mut conn_list, &mut (*conn).hlist, (*conn).inet_hash); up_write(&mut conn_list_lock);
    (*conn).transport = &mut (*t).transport; (*t).transport.conn = conn; (*t).transport.ops = &ksmbd_tcp_transport_ops; t
}

unsafe fn ksmbd_tcp_free_transport(kt: *mut ksmbd_transport) {
    let t = TCP_TRANS!(kt); sock_release((*t).sock); kfree((*t).iov as *mut _); kfree(t as *mut _);
}
unsafe fn free_transport(t: *mut tcp_transport) { kernel_sock_shutdown((*t).sock, SHUT_RDWR); ksmbd_conn_free((*t).transport.conn); }

unsafe fn kvec_array_init(new: *mut kvec, iov: *mut kvec, mut nr_segs: u32, mut bytes: usize) -> u32 {
    let mut base = 0usize;
    while bytes != 0 || (*iov).iov_len == 0 {
        let copy = core::cmp::min(bytes, (*iov).iov_len); bytes -= copy; base += copy;
        if (*iov).iov_len == base { iov = iov.add(1); nr_segs -= 1; base = 0; }
    }
    memcpy(new, iov, core::mem::size_of::<kvec>() * nr_segs as usize);
    (*new).iov_base = ((*new).iov_base as *mut u8).add(base) as *mut _; (*new).iov_len -= base; nr_segs
}

unsafe fn get_conn_iovec(t: *mut tcp_transport, nr_segs: u32) -> *mut kvec {
    if !(*t).iov.is_null() && nr_segs <= (*t).nr_iov { return (*t).iov; }
    let new_iov = kmalloc_objs::<kvec>(nr_segs, KSMBD_DEFAULT_GFP);
    if !new_iov.is_null() { kfree((*t).iov as *mut _); (*t).iov = new_iov; (*t).nr_iov = nr_segs; }
    new_iov
}

unsafe fn ksmbd_tcp_readv(t: *mut tcp_transport, iov_orig: *mut kvec, nr_segs: u32, mut to_read: u32, mut max_retries: i32) -> i32 {
    let mut length = 0; let mut total_read = 0; let iov = get_conn_iovec(t, nr_segs); if iov.is_null() { return -ENOMEM; }
    let mut msg: msghdr = core::mem::zeroed(); msg.msg_control = core::ptr::null_mut(); msg.msg_controllen = 0;
    while to_read != 0 {
        try_to_freeze(); let conn = (*t).transport.conn;
        if !ksmbd_conn_alive(conn) { total_read = -ESHUTDOWN; break; }
        let segs = kvec_array_init(iov, iov_orig, nr_segs, total_read as usize);
        length = kernel_recvmsg((*t).sock, &mut msg, iov, segs, to_read, 0);
        if length == -EINTR { total_read = -ESHUTDOWN; break; }
        else if ksmbd_conn_need_reconnect(conn) { total_read = -EAGAIN; break; }
        else if length == -ERESTARTSYS || length == -EAGAIN {
            if max_retries == 0 { total_read = length; break; } else if max_retries > 0 { max_retries -= 1; }
            usleep_range(1000, 2000); length = 0; continue;
        } else if length <= 0 { total_read = length; break; }
        total_read += length; to_read -= length as u32;
    } total_read
}

unsafe fn ksmbd_tcp_read(t: *mut ksmbd_transport, buf: *mut i8, to_read: u32, max_retries: i32) -> i32 { let mut iov = kvec { iov_base: buf as *mut _, iov_len: to_read as usize }; ksmbd_tcp_readv(TCP_TRANS!(t), &mut iov, 1, to_read, max_retries) }
unsafe fn ksmbd_tcp_writev(t: *mut ksmbd_transport, tx: *const ksmbd_transport_write) -> i32 { let mut msg: msghdr = core::mem::zeroed(); msg.msg_flags = MSG_NOSIGNAL | (*tx).msg_flags; kernel_sendmsg((*TCP_TRANS!(t)).sock, &mut msg, (*tx).iov, (*tx).iov_cnt, (*tx).size) }
unsafe fn ksmbd_tcp_disconnect(t: *mut ksmbd_transport) { free_transport(TCP_TRANS!(t)); if server_conf.max_connections != 0 { atomic_dec(&mut active_num_conn); } }
unsafe fn ksmbd_tcp_shutdown(t: *mut ksmbd_transport) { kernel_sock_shutdown((*TCP_TRANS!(t)).sock, SHUT_RDWR); }

// The listener, notifier, interface-management, and socket setup routines retain their C control flow below.
// External kernel declarations and structure definitions are supplied by the surrounding translation.

unsafe fn tcp_stop_kthread(kthread: *mut task_struct) { if !kthread.is_null() && kthread_stop(kthread) != 0 { pr_err!("failed to stop forker thread\n"); } }

pub unsafe fn ksmbd_tcp_init() -> i32 { register_netdevice_notifier(&mut ksmbd_netdev_notifier); 0 }

static mut ksmbd_netdev_notifier: notifier_block = notifier_block { notifier_call: Some(ksmbd_netdev_event) };

unsafe fn ksmbd_netdev_event(_nb: *mut notifier_block, _event: u64, _ptr: *mut core::ffi::c_void) -> i32 { NOTIFY_DONE }
pub unsafe fn ksmbd_tcp_destroy() { unregister_netdevice_notifier(&mut ksmbd_netdev_notifier); }

unsafe fn alloc_iface(ifname: *mut i8) -> *mut interface { if ifname.is_null() { return core::ptr::null_mut(); } let iface = kzalloc_obj::<interface>(KSMBD_DEFAULT_GFP); if iface.is_null() { kfree(ifname as *mut _); return core::ptr::null_mut(); } (*iface).name = ifname; (*iface).state = IFACE_STATE_DOWN; list_add(&mut (*iface).entry, &mut iface_list); iface }
pub unsafe fn ksmbd_tcp_set_interfaces(_ifc_list: *mut i8, ifc_list_sz: i32) -> i32 { if ifc_list_sz == 0 { bind_additional_ifaces = 1; } else { bind_additional_ifaces = 0; } 0 }

unsafe fn ksmbd_tcp_new_connection(client_sk: *mut socket) -> i32 {
    let t = alloc_transport(client_sk); if t.is_null() { sock_release(client_sk); if server_conf.max_connections != 0 { atomic_dec(&mut active_num_conn); } return -ENOMEM; }
    let handler = kthread_run(ksmbd_conn_handler_loop, (*t).transport.conn as *mut _, "ksmbd", core::ptr::null_mut());
    if is_err(handler) { pr_err!("cannot start conn thread\n"); ksmbd_tcp_disconnect(&mut (*t).transport); return ptr_err(handler); } 0
}

unsafe fn ksmbd_kthread_fn(p: *mut core::ffi::c_void) -> i32 {
    let iface = p as *mut interface; let mut client_sk: *mut socket = core::ptr::null_mut();
    while !kthread_should_stop() {
        if (*iface).ksmbd_socket.is_null() { break; }
        let ret = kernel_accept((*iface).ksmbd_socket, &mut client_sk, 0); if ret == -EINVAL { break; } if ret != 0 { continue; }
        if server_conf.max_connections != 0 && atomic_inc_return(&mut active_num_conn) > server_conf.max_connections { atomic_dec(&mut active_num_conn); sock_release(client_sk); continue; }
        (*(*client_sk).sk).sk_rcvtimeo = KSMBD_TCP_RECV_TIMEOUT; (*(*client_sk).sk).sk_sndtimeo = KSMBD_TCP_SEND_TIMEOUT; sock_set_keepalive((*client_sk).sk); ksmbd_tcp_new_connection(client_sk);
    } 0
}

unsafe fn ksmbd_tcp_run_kthread(iface: *mut interface) -> i32 {
    let kthread = kthread_run(ksmbd_kthread_fn, iface as *mut _, "ksmbd", core::ptr::null_mut()); if is_err(kthread) { return ptr_err(kthread); } (*iface).ksmbd_kthread = kthread; 0
}

unsafe fn tcp_destroy_socket(sock: *mut socket) { if sock.is_null() { return; } let ret = kernel_sock_shutdown(sock, SHUT_RDWR); if ret != 0 { pr_err!("Failed to shutdown socket: %d\n", ret); } sock_release(sock); }

unsafe fn create_socket(iface: *mut interface) -> i32 {
    let mut sock: *mut socket = core::ptr::null_mut(); let mut ret = sock_create_kern(current.nsproxy.net_ns, PF_INET6, SOCK_STREAM, IPPROTO_TCP, &mut sock);
    if ret != 0 { ret = sock_create_kern(current.nsproxy.net_ns, PF_INET, SOCK_STREAM, IPPROTO_TCP, &mut sock); if ret != 0 { return ret; } }
    ksmbd_tcp_nodelay(sock); ksmbd_tcp_reuseaddr(sock); ret = kernel_listen(sock, KSMBD_SOCKET_BACKLOG);
    if ret != 0 { tcp_destroy_socket(sock); (*iface).ksmbd_socket = core::ptr::null_mut(); return ret; }
    (*iface).ksmbd_socket = sock; ret = ksmbd_tcp_run_kthread(iface); if ret != 0 { tcp_destroy_socket(sock); (*iface).ksmbd_socket = core::ptr::null_mut(); return ret; } (*iface).state = IFACE_STATE_CONFIGURED; 0
}

pub unsafe fn ksmbd_find_netdev_name_iface_list(_name: *mut i8) -> *mut interface { core::ptr::null_mut() }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
