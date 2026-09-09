/* SPDX-License-Identifier: GPL-2.0-only */
/* VMware vSockets Driver */

use core::ffi::{c_int, c_uint, c_long, c_void};

pub const LAST_RESERVED_PORT: u32 = 1023;
pub const VSOCK_HASH_SIZE: usize = 251;

extern "C" {
    pub static mut vsock_bind_table: [list_head; VSOCK_HASH_SIZE + 1];
    pub static mut vsock_connected_table: [list_head; VSOCK_HASH_SIZE];
    pub static mut vsock_table_lock: spinlock_t;
}

#[inline]
pub unsafe fn vsock_sk(sk: *mut sock) -> *mut vsock_sock { sk as *mut vsock_sock }
#[inline]
pub unsafe fn sk_vsock(vsk: *mut vsock_sock) -> *mut sock { &mut (*vsk).sk }

#[repr(C)]
pub struct vsock_sock {
    pub sk: sock,
    pub transport: *const vsock_transport,
    pub local_addr: sockaddr_vm,
    pub remote_addr: sockaddr_vm,
    pub bound_table: list_head,
    pub connected_table: list_head,
    pub trusted: bool,
    pub cached_peer_allow_dgram: bool,
    pub cached_peer: u32,
    pub owner: *const cred,
    pub connect_timeout: c_long,
    pub listener: *mut sock,
    pub pending_links: list_head,
    pub accept_queue: list_head,
    pub connect_work: delayed_work,
    pub pending_work: delayed_work,
    pub close_work: delayed_work,
    pub close_work_scheduled: bool,
    pub peer_shutdown: u32,
    pub sent_request: bool,
    pub ignore_connecting_rst: bool,
    pub buffer_size: u64,
    pub buffer_min_size: u64,
    pub buffer_max_size: u64,
    pub trans: *mut c_void,
}

extern "C" {
    pub fn vsock_connectible_has_data(vsk: *mut vsock_sock) -> i64;
    pub fn vsock_stream_has_data(vsk: *mut vsock_sock) -> i64;
    pub fn vsock_stream_has_space(vsk: *mut vsock_sock) -> i64;
    pub fn vsock_create_connected(parent: *mut sock) -> *mut sock;
    pub fn vsock_data_ready(sk: *mut sock);
}

#[repr(C)]
pub struct vsock_transport_recv_notify_data { pub data1: u64, pub data2: u64, pub notify_on_block: bool }
#[repr(C)]
pub struct vsock_transport_send_notify_data { pub data1: u64, pub data2: u64 }

pub const VSOCK_TRANSPORT_F_H2G: u32 = 0x00000001;
pub const VSOCK_TRANSPORT_F_G2H: u32 = 0x00000002;
pub const VSOCK_TRANSPORT_F_DGRAM: u32 = 0x00000004;
pub const VSOCK_TRANSPORT_F_LOCAL: u32 = 0x00000008;

#[repr(C)]
pub struct vsock_transport {
    pub module: *mut module,
    pub init: Option<unsafe extern "C" fn(*mut vsock_sock, *mut vsock_sock) -> c_int>,
    pub destruct: Option<unsafe extern "C" fn(*mut vsock_sock)>,
    pub release: Option<unsafe extern "C" fn(*mut vsock_sock)>,
    pub cancel_pkt: Option<unsafe extern "C" fn(*mut vsock_sock) -> c_int>,
    pub connect: Option<unsafe extern "C" fn(*mut vsock_sock) -> c_int>,
    pub dgram_bind: Option<unsafe extern "C" fn(*mut vsock_sock, *mut sockaddr_vm) -> c_int>,
    pub dgram_dequeue: Option<unsafe extern "C" fn(*mut vsock_sock, *mut msghdr, usize, c_int) -> c_int>,
    pub dgram_enqueue: Option<unsafe extern "C" fn(*mut vsock_sock, *mut sockaddr_vm, *mut msghdr, usize) -> c_int>,
    pub dgram_allow: Option<unsafe extern "C" fn(*mut vsock_sock, u32, u32) -> bool>,
    pub stream_dequeue: Option<unsafe extern "C" fn(*mut vsock_sock, *mut msghdr, usize, c_int) -> isize>,
    pub stream_enqueue: Option<unsafe extern "C" fn(*mut vsock_sock, *mut msghdr, usize) -> isize>,
    pub stream_has_data: Option<unsafe extern "C" fn(*mut vsock_sock) -> i64>,
    pub stream_has_space: Option<unsafe extern "C" fn(*mut vsock_sock) -> i64>,
    pub stream_rcvhiwat: Option<unsafe extern "C" fn(*mut vsock_sock) -> u64>,
    pub stream_is_active: Option<unsafe extern "C" fn(*mut vsock_sock) -> bool>,
    pub stream_allow: Option<unsafe extern "C" fn(*mut vsock_sock, u32, u32) -> bool>,
    pub seqpacket_dequeue: Option<unsafe extern "C" fn(*mut vsock_sock, *mut msghdr, c_int) -> isize>,
    pub seqpacket_enqueue: Option<unsafe extern "C" fn(*mut vsock_sock, *mut msghdr, usize) -> c_int>,
    pub seqpacket_allow: Option<unsafe extern "C" fn(*mut vsock_sock, u32) -> bool>,
    pub seqpacket_has_data: Option<unsafe extern "C" fn(*mut vsock_sock) -> u32>,
    pub notify_poll_in: Option<unsafe extern "C" fn(*mut vsock_sock, usize, *mut bool) -> c_int>,
    pub notify_poll_out: Option<unsafe extern "C" fn(*mut vsock_sock, usize, *mut bool) -> c_int>,
    pub notify_recv_init: Option<unsafe extern "C" fn(*mut vsock_sock, usize, *mut vsock_transport_recv_notify_data) -> c_int>,
    pub notify_recv_pre_block: Option<unsafe extern "C" fn(*mut vsock_sock, usize, *mut vsock_transport_recv_notify_data) -> c_int>,
    pub notify_recv_pre_dequeue: Option<unsafe extern "C" fn(*mut vsock_sock, usize, *mut vsock_transport_recv_notify_data) -> c_int>,
    pub notify_recv_post_dequeue: Option<unsafe extern "C" fn(*mut vsock_sock, usize, isize, bool, *mut vsock_transport_recv_notify_data) -> c_int>,
    pub notify_send_init: Option<unsafe extern "C" fn(*mut vsock_sock, *mut vsock_transport_send_notify_data) -> c_int>,
    pub notify_send_pre_block: Option<unsafe extern "C" fn(*mut vsock_sock, *mut vsock_transport_send_notify_data) -> c_int>,
    pub notify_send_pre_enqueue: Option<unsafe extern "C" fn(*mut vsock_sock, *mut vsock_transport_send_notify_data) -> c_int>,
    pub notify_send_post_enqueue: Option<unsafe extern "C" fn(*mut vsock_sock, isize, *mut vsock_transport_send_notify_data) -> c_int>,
    pub notify_buffer_size: Option<unsafe extern "C" fn(*mut vsock_sock, *mut u64)>,
    pub notify_set_rcvlowat: Option<unsafe extern "C" fn(*mut vsock_sock, c_int) -> c_int>,
    pub unsent_bytes: Option<unsafe extern "C" fn(*mut vsock_sock) -> isize>,
    pub shutdown: Option<unsafe extern "C" fn(*mut vsock_sock, c_int) -> c_int>,
    pub get_local_cid: Option<unsafe extern "C" fn() -> u32>,
    pub has_remote_cid: Option<unsafe extern "C" fn(*mut vsock_sock, u32) -> bool>,
    pub read_skb: Option<unsafe extern "C" fn(*mut vsock_sock, skb_read_actor_t) -> c_int>,
    pub msgzerocopy_allow: Option<unsafe extern "C" fn() -> bool>,
}

extern "C" {
    pub fn vsock_core_register(t: *const vsock_transport, features: c_int) -> c_int;
    pub fn vsock_core_unregister(t: *const vsock_transport);
    pub fn vsock_core_get_transport(vsk: *mut vsock_sock) -> *const vsock_transport;
    pub fn vsock_add_pending(listener: *mut sock, pending: *mut sock);
    pub fn vsock_remove_pending(listener: *mut sock, pending: *mut sock);
    pub fn vsock_enqueue_accept(listener: *mut sock, connected: *mut sock);
    pub fn vsock_pending_to_accept(listener: *mut sock, pending: *mut sock);
    pub fn vsock_insert_connected(vsk: *mut vsock_sock);
    pub fn vsock_remove_bound(vsk: *mut vsock_sock);
    pub fn vsock_remove_connected(vsk: *mut vsock_sock);
    pub fn vsock_find_bound_socket(addr: *mut sockaddr_vm) -> *mut sock;
    pub fn vsock_find_connected_socket(src: *mut sockaddr_vm, dst: *mut sockaddr_vm) -> *mut sock;
    pub fn vsock_find_bound_socket_net(addr: *mut sockaddr_vm, net: *mut net) -> *mut sock;
    pub fn vsock_find_connected_socket_net(src: *mut sockaddr_vm, dst: *mut sockaddr_vm, net: *mut net) -> *mut sock;
    pub fn vsock_remove_sock(vsk: *mut vsock_sock);
    pub fn vsock_for_each_connected_socket(transport: *mut vsock_transport, f: Option<unsafe extern "C" fn(*mut sock)>);
    pub fn vsock_assign_transport(vsk: *mut vsock_sock, psk: *mut vsock_sock) -> c_int;
    pub fn vsock_find_cid(cid: c_uint) -> bool;
    pub fn vsock_linger(sk: *mut sock);
}

#[repr(C)] pub struct vsock_tap { pub dev: *mut net_device, pub module: *mut module, pub list: list_head }
extern "C" {
    pub fn vsock_add_tap(vt: *mut vsock_tap) -> c_int;
    pub fn vsock_remove_tap(vt: *mut vsock_tap) -> c_int;
    pub fn vsock_deliver_tap(build_skb: Option<unsafe extern "C" fn(*mut c_void) -> *mut sk_buff>, opaque: *mut c_void);
    pub fn __vsock_connectible_recvmsg(sock: *mut socket, msg: *mut msghdr, len: usize, flags: c_int) -> c_int;
    pub fn vsock_connectible_recvmsg(sock: *mut socket, msg: *mut msghdr, len: usize, flags: c_int) -> c_int;
    pub fn __vsock_dgram_recvmsg(sock: *mut socket, msg: *mut msghdr, len: usize, flags: c_int) -> c_int;
    pub fn vsock_dgram_recvmsg(sock: *mut socket, msg: *mut msghdr, len: usize, flags: c_int) -> c_int;
    pub static mut vsock_proto: proto;
}

#[inline] pub unsafe fn vsock_msgzerocopy_allow(t: *const vsock_transport) -> bool { (*t).msgzerocopy_allow.map_or(false, |f| f()) }
#[inline] pub unsafe fn __vsock_in_bound_table(vsk: *mut vsock_sock) -> bool { !list_empty(&(*vsk).bound_table) }
#[inline] pub unsafe fn __vsock_in_connected_table(vsk: *mut vsock_sock) -> bool { !list_empty(&(*vsk).connected_table) }

#[cfg(feature = "CONFIG_BPF_SYSCALL")]
extern "C" {
    pub fn vsock_bpf_update_proto(sk: *mut sock, psock: *mut sk_psock, restore: bool) -> c_int;
    pub fn vsock_bpf_build_proto();
}
#[cfg(not(feature = "CONFIG_BPF_SYSCALL"))]
#[inline] pub unsafe fn vsock_bpf_build_proto() {}

#[inline]
pub unsafe fn vsock_net_mode(net: *mut net) -> vsock_net_mode {
    if net.is_null() { return VSOCK_NET_MODE_GLOBAL; }
    core::ptr::read_volatile(&(*net).vsock.mode)
}

#[inline]
pub unsafe fn vsock_net_mode_global(vsk: *mut vsock_sock) -> bool {
    vsock_net_mode(sock_net(sk_vsock(vsk))) == VSOCK_NET_MODE_GLOBAL
}

#[inline]
pub unsafe fn vsock_net_set_child_mode(net: *mut net, mode: vsock_net_mode) -> bool {
    let mut old_locked: c_int = 0;
    let new_locked = (mode as c_int) + 1;
    if try_cmpxchg(&mut (*net).vsock.child_ns_mode_locked, &mut old_locked, new_locked) {
        core::ptr::write_volatile(&mut (*net).vsock.child_ns_mode, mode);
        true
    } else { old_locked == new_locked }
}

#[inline]
pub unsafe fn vsock_net_child_mode(net: *mut net) -> vsock_net_mode {
    core::ptr::read_volatile(&(*net).vsock.child_ns_mode)
}

/* A NULL namespace is treated as VSOCK_NET_MODE_GLOBAL. */
#[inline]
pub unsafe fn vsock_net_check_mode(ns0: *mut net, ns1: *mut net) -> bool {
    if net_eq(ns0, ns1) { return true; }
    let mode0 = vsock_net_mode(ns0);
    let mode1 = vsock_net_mode(ns1);
    mode0 == VSOCK_NET_MODE_GLOBAL && mode0 == mode1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
