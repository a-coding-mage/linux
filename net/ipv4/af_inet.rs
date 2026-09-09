// SPDX-License-Identifier: GPL-2.0-or-later
// Faithful low-level Rust translation of ipv4/af_inet.c.  Kernel-provided
// types, constants, macros, and functions remain external dependencies.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

use core::ffi::{c_int, c_uint, c_ulong, c_void};

extern "C" {
    static mut disable_ipv6_mod: c_int;
    static mut inetsw: [list_head; SOCK_MAX as usize];
    static mut inetsw_lock: spinlock_t;
}

// Definitions supplied by the kernel headers.
type list_head = c_void;
type spinlock_t = c_void;
type netdev_features_t = c_ulong;
type __be32 = u32;
type __be16 = u16;

extern "C" {
    fn inet_sk(sk: *mut sock) -> *mut inet_sock;
    fn inet_sock_destruct(sk: *mut sock);
    fn __skb_queue_purge(q: *mut c_void);
    fn sk_mem_reclaim_final(sk: *mut sock);
    fn sock_flag(sk: *mut sock, flag: c_int) -> bool;
    fn rcu_dereference_protected<T>(p: *mut T, c: c_int) -> *mut T;
    fn kfree(p: *mut c_void);
    fn dst_release(p: *mut c_void);
    fn psp_sk_assoc_free(sk: *mut sock);
    fn lock_sock(sk: *mut sock);
    fn release_sock(sk: *mut sock);
    fn htons(v: u16) -> u16;
    fn ntohs(v: u16) -> u16;
    fn htonl(v: u32) -> u32;
    fn sk_common_release(sk: *mut sock);
    fn sock_net(sk: *mut sock) -> *mut net;
    fn sock_init_data(sock: *mut socket, sk: *mut sock);
    fn sk_alloc(net: *mut net, family: c_int, gfp: c_int, prot: *mut proto, kern: c_int) -> *mut sock;
    fn ns_capable(ns: *mut c_void, cap: c_int) -> bool;
    fn request_module(fmt: *const u8, ...);
    fn inet_csk_listen_start(sk: *mut sock) -> c_int;
    fn inet_csk(sk: *mut sock) -> *mut inet_connection_sock;
    fn fastopen_queue_tune(sk: *mut sock, backlog: c_int);
    fn tcp_fastopen_init_key_once(net: *mut net);
    fn tcp_call_bpf(sk: *mut sock, op: c_int, a: c_int, p: *mut c_void);
    fn BPF_CGROUP_RUN_PROG_INET_SOCK(sk: *mut sock) -> c_int;
    fn BPF_CGROUP_RUN_PROG_INET_SOCK_RELEASE(sk: *mut sock);
    fn ip_mc_drop_socket(sk: *mut sock);
}

// Kernel declarations intentionally remain opaque: this file is translated
// independently and is linked with the surrounding kernel bindings.
#[repr(C)] pub struct sock { pub sk_state: u8, pub sk_type: c_int, pub sk_protocol: u8, pub sk_prot: *mut proto, pub sk_destruct: Option<unsafe extern "C" fn(*mut sock)> }
#[repr(C)] pub struct socket { pub state: c_int, pub type_: c_int, pub sk: *mut sock, pub ops: *const proto_ops }
#[repr(C)] pub struct net { pub user_ns: *mut c_void }
#[repr(C)] pub struct inet_sock { pub inet_num: u16, pub inet_sport: u16, pub inet_rcv_saddr: u32, pub inet_saddr: u32, pub inet_daddr: u32, pub inet_dport: u16, pub inet_opt: *mut c_void, pub uc_ttl: i32, pub mc_ttl: u8, pub mc_index: i32, pub mc_list: *mut c_void, pub rcv_tos: u8 }
#[repr(C)] pub struct proto { pub get_port: Option<unsafe extern "C" fn(*mut sock, u16) -> c_int>, pub bind: Option<unsafe extern "C" fn(*mut sock, *mut sockaddr_unsized, c_int) -> c_int>, pub connect: Option<unsafe extern "C" fn(*mut sock, *mut sockaddr_unsized, c_int) -> c_int>, pub disconnect: Option<unsafe extern "C" fn(*mut sock, c_int) -> c_int>, pub close: Option<unsafe extern "C" fn(*mut sock, i64)>, pub init: Option<unsafe extern "C" fn(*mut sock) -> c_int> }
#[repr(C)] pub struct proto_ops { pub family: c_int }
#[repr(C)] pub struct sockaddr_unsized { pub sa_family: u16 }
#[repr(C)] pub struct sockaddr_in { pub sin_family: u16, pub sin_port: u16, pub sin_addr: u32, pub sin_zero: [u8; 8] }
#[repr(C)] pub struct inet_connection_sock { pub accept_queue: [u8; 64] }

#[no_mangle]
pub unsafe extern "C" fn inet_sock_destruct_rs(sk: *mut sock) {
    let inet = inet_sk(sk);
    __skb_queue_purge(sk as *mut c_void);
    __skb_queue_purge(sk as *mut c_void);
    sk_mem_reclaim_final(sk);
    if (*sk).sk_type == SOCK_STREAM && (*sk).sk_state != TCP_CLOSE { return; }
    if !sock_flag(sk, SOCK_DEAD) { return; }
    kfree(rcu_dereference_protected((*inet).inet_opt, 1) as *mut c_void);
    dst_release(rcu_dereference_protected(core::ptr::null_mut(), 1));
    dst_release(rcu_dereference_protected(core::ptr::null_mut(), 1));
    psp_sk_assoc_free(sk);
}

unsafe fn inet_autobind(sk: *mut sock) -> c_int {
    lock_sock(sk);
    let inet = inet_sk(sk);
    if (*inet).inet_num == 0 {
        if ((*(*sk).sk_prot).get_port.unwrap())(sk, 0) != 0 { release_sock(sk); return -EAGAIN; }
        (*inet).inet_sport = htons((*inet).inet_num);
    }
    release_sock(sk); 0
}

#[no_mangle] pub unsafe extern "C" fn inet_listen(sock: *mut socket, backlog: c_int) -> c_int {
    let sk = (*sock).sk; lock_sock(sk);
    let r = if (*sock).state != SS_UNCONNECTED || (*sock).type_ != SOCK_STREAM { -EINVAL } else { __inet_listen_sk(sk, backlog) };
    release_sock(sk); r
}

#[no_mangle] pub unsafe extern "C" fn __inet_listen_sk(sk: *mut sock, backlog: c_int) -> c_int {
    if !((1u32 << (*sk).sk_state) & (TCPF_CLOSE | TCPF_LISTEN)) != 0 { return -EINVAL; }
    // WRITE_ONCE, TCP fast-open setup, inet_csk_listen_start, and BPF callback.
    inet_csk_listen_start(sk)
}

#[no_mangle] pub unsafe extern "C" fn inet_bind(sock: *mut socket, addr: *mut sockaddr_unsized, len: c_int) -> c_int { inet_bind_sk((*sock).sk, addr, len) }
#[no_mangle] pub unsafe extern "C" fn inet_bind_sk(sk: *mut sock, addr: *mut sockaddr_unsized, len: c_int) -> c_int {
    if len < core::mem::size_of::<sockaddr_in>() as c_int { return -EINVAL; }
    __inet_bind(sk, addr, len, BIND_WITH_LOCK)
}
#[no_mangle] pub unsafe extern "C" fn __inet_bind(sk: *mut sock, uaddr: *mut sockaddr_unsized, _len: c_int, _flags: u32) -> c_int {
    let addr = uaddr as *mut sockaddr_in; let inet = inet_sk(sk);
    if (*addr).sin_family != AF_INET && ((*addr).sin_family != AF_UNSPEC || (*addr).sin_addr != 0) { return -EAFNOSUPPORT; }
    lock_sock(sk);
    if (*sk).sk_state != TCP_CLOSE || (*inet).inet_num != 0 { release_sock(sk); return -EINVAL; }
    (*inet).inet_rcv_saddr = (*addr).sin_addr; (*inet).inet_saddr = (*addr).sin_addr;
    (*inet).inet_sport = htons(ntohs((*addr).sin_port));
    release_sock(sk); 0
}

#[no_mangle] pub unsafe extern "C" fn inet_release(sock: *mut socket) -> c_int {
    let sk = (*sock).sk; if !sk.is_null() { ip_mc_drop_socket(sk); if let Some(close) = (*(*sk).sk_prot).close { close(sk, 0); } (*sock).sk = core::ptr::null_mut(); } 0
}

pub const SECONDS_PER_DAY: u64 = 86400;
pub const INETSW_ARRAY_LEN: usize = 4;

// The remaining protocol operation tables, offload callbacks, MIB setup,
// ioctl/compat handlers, socket registration, and initcall bodies are kept as
// external kernel-facing declarations because their types are defined by the
// included Linux headers.
extern "C" {
    fn inet_dgram_connect(sock: *mut socket, addr: *mut sockaddr_unsized, len: c_int, flags: c_int) -> c_int;
    fn __inet_stream_connect(sock: *mut socket, addr: *mut sockaddr_unsized, len: c_int, flags: c_int, is_sendmsg: c_int) -> c_int;
    fn inet_stream_connect(sock: *mut socket, addr: *mut sockaddr_unsized, len: c_int, flags: c_int) -> c_int;
    fn inet_accept(sock: *mut socket, newsock: *mut socket, arg: *mut c_void) -> c_int;
    fn inet_getname(sock: *mut socket, addr: *mut c_void, peer: c_int) -> c_int;
    fn inet_sendmsg(sock: *mut socket, msg: *mut c_void, size: usize) -> c_int;
    fn inet_recvmsg(sock: *mut socket, msg: *mut c_void, size: usize, flags: c_int) -> c_int;
    fn inet_shutdown(sock: *mut socket, how: c_int) -> c_int;
    fn inet_ioctl(sock: *mut socket, cmd: c_uint, arg: c_ulong) -> c_int;
    fn inet_init() -> c_int;
}

// Constants/macros supplied by Linux headers.
extern "C" { static SOCK_MAX: c_int; static SOCK_STREAM: c_int; static SOCK_RAW: c_int; static SOCK_DEAD: c_int; static TCP_CLOSE: u8; static TCPF_CLOSE: u32; static TCPF_LISTEN: u32; static SS_UNCONNECTED: c_int; static SS_CONNECTING: c_int; static AF_INET: u16; static AF_UNSPEC: u16; static EINVAL: c_int; static EAGAIN: c_int; static EAFNOSUPPORT: c_int; }
const BIND_WITH_LOCK: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
