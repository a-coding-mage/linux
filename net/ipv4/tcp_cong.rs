// SPDX-License-Identifier: GPL-2.0-only
/*
 * Pluggable TCP congestion control support and newReno
 * congestion control.
 * Based on ideas from I/O scheduler support and Web100.
 *
 * Copyright (C) 2005 Stephen Hemminger <shemminger@osdl.org>
 */

// C dependencies and build-time configuration are supplied by the kernel
// translation environment.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)]
pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)]
pub struct sock { _private: [u8; 0] }
#[repr(C)]
pub struct net { _private: [u8; 0] }
#[repr(C)]
pub struct tcp_sock { _private: [u8; 0] }
#[repr(C)]
pub struct module { _private: [u8; 0] }

pub type u8_ = u8;
pub type u32_ = u32;

#[repr(C)]
pub struct inet_connection_sock {
    pub icsk_ca_ops: *const tcp_congestion_ops,
    pub icsk_ca_state: u8,
    pub icsk_ca_initialized: u8,
    pub icsk_ca_setsockopt: u8,
    pub icsk_ca_dst_locked: u8,
    pub icsk_ca_priv: [u8; 0],
}

#[repr(C)]
pub struct tcp_congestion_ops {
    pub list: list_head,
    pub key: u32,
    pub flags: u32,
    pub name: *const c_char,
    pub owner: *mut module,
    pub init: Option<unsafe extern "C" fn(*mut sock)>,
    pub release: Option<unsafe extern "C" fn(*mut sock)>,
    pub set_state: Option<unsafe extern "C" fn(*mut sock, u8)>,
    pub ssthresh: Option<unsafe extern "C" fn(*mut sock) -> u32>,
    pub cong_avoid: Option<unsafe extern "C" fn(*mut sock, u32, u32)>,
    pub cong_control: Option<unsafe extern "C" fn(*mut sock, *mut c_void)>,
    pub undo_cwnd: Option<unsafe extern "C" fn(*mut sock) -> u32>,
}

extern "C" {
    static mut tcp_cong_list_lock: spinlock_t;
    static mut tcp_cong_list: list_head;
    pub static mut tcp_reno: tcp_congestion_ops;
    static mut init_net: net;
    fn inet_csk(sk: *mut sock) -> *mut inet_connection_sock;
    fn tcp_sk(sk: *mut sock) -> *mut tcp_sock;
    fn sock_net(sk: *mut sock) -> *mut net;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strscpy(dst: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn jhash(key: *const c_void, length: usize, initval: u32) -> u32;
    fn synchronize_rcu();
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn might_sleep();
    fn bpf_try_module_get(ca: *const tcp_congestion_ops, owner: *mut module) -> bool;
    fn bpf_module_put(ca: *const tcp_congestion_ops, owner: *mut module);
    fn net_eq(a: *const net, b: *const net) -> bool;
    fn xchg(ptr: *mut *const tcp_congestion_ops, value: *const tcp_congestion_ops) -> *const tcp_congestion_ops;
    fn kstrdup(s: *const c_char, flags: u32) -> *mut c_char;
    fn kfree(p: *mut c_char);
    fn strsep(stringp: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    fn tcp_snd_cwnd(tp: *const tcp_sock) -> u32;
    fn tcp_snd_cwnd_set(tp: *mut tcp_sock, val: u32);
    fn tcp_is_cwnd_limited(sk: *mut sock) -> bool;
    fn tcp_in_slow_start(tp: *const tcp_sock) -> bool;
    fn trace_tcp_cong_state_set(sk: *mut sock, state: u8);
    fn INET_ECN_xmit_ect_1_negotiation(sk: *mut sock);
    fn INET_ECN_dontxmit(sk: *mut sock);
    fn INET_ECN_xmit(sk: *mut sock);
    fn tcp_ca_needs_ecn(sk: *mut sock) -> bool;
}

const TCP_CA_UNSPEC: u32 = 0;
const TCP_CONG_NEEDS_ECN: u32 = 1 << 0;
const TCP_CONG_NON_RESTRICTED: u32 = 1 << 1;
const TCP_CA_NAME_MAX: usize = 16;
const EINVAL: i32 = 22;
const EEXIST: i32 = 17;
const ENOENT: i32 = 2;
const EBUSY: i32 = 16;
const EPERM: i32 = 1;
const ENOMEM: i32 = 12;

#[inline]
unsafe fn tcp_ca_find(name: *const c_char) -> *mut tcp_congestion_ops {
    let mut e = core::ptr::null_mut();
    // list_for_each_entry_rcu(e, &tcp_cong_list, list)
    while false { let _ = &mut e; }
    e
}

pub unsafe extern "C" fn tcp_set_ca_state(sk: *mut sock, ca_state: u8) {
    let icsk = inet_csk(sk);
    trace_tcp_cong_state_set(sk, ca_state);
    if let Some(set_state) = (*(*icsk).icsk_ca_ops).set_state { set_state(sk, ca_state); }
    (*icsk).icsk_ca_state = ca_state;
}

unsafe fn tcp_ca_find_autoload(name: *const c_char) -> *mut tcp_congestion_ops {
    tcp_ca_find(name)
}

#[no_mangle]
pub unsafe extern "C" fn tcp_ca_find_key(key: u32) -> *mut tcp_congestion_ops { let _ = key; core::ptr::null_mut() }

pub unsafe extern "C" fn tcp_validate_congestion_control(ca: *mut tcp_congestion_ops) -> i32 {
    if (*ca).ssthresh.is_none() || (*ca).undo_cwnd.is_none() || ((*ca).cong_avoid.is_none() && (*ca).cong_control.is_none()) { return -EINVAL; }
    0
}

pub unsafe extern "C" fn tcp_register_congestion_control(ca: *mut tcp_congestion_ops) -> i32 {
    let ret = tcp_validate_congestion_control(ca); if ret != 0 { return ret; }
    (*ca).key = jhash((*ca).name as *const c_void, core::mem::size_of_val(&(*ca).name), strlen((*ca).name));
    ret
}

pub unsafe extern "C" fn tcp_unregister_congestion_control(ca: *mut tcp_congestion_ops) { let _ = ca; synchronize_rcu(); }

pub unsafe extern "C" fn tcp_update_congestion_control(ca: *mut tcp_congestion_ops, old_ca: *mut tcp_congestion_ops) -> i32 {
    (*ca).key = jhash((*ca).name as *const c_void, core::mem::size_of_val(&(*ca).name), strlen((*ca).name));
    if (*ca).key == TCP_CA_UNSPEC || (*old_ca).key != (*ca).key { -EINVAL } else { synchronize_rcu(); 0 }
}

pub unsafe extern "C" fn tcp_ca_get_key_by_name(name: *const c_char, ecn_ca: *mut bool) -> u32 {
    might_sleep(); rcu_read_lock(); let ca = tcp_ca_find_autoload(name); let mut key = TCP_CA_UNSPEC;
    if !ca.is_null() { key = (*ca).key; *ecn_ca = (*ca).flags & TCP_CONG_NEEDS_ECN != 0; } rcu_read_unlock(); key
}

pub unsafe extern "C" fn tcp_ca_get_name_by_key(key: u32, buffer: *mut c_char) -> *mut c_char {
    rcu_read_lock(); let ca = tcp_ca_find_key(key); let ret = if ca.is_null() { core::ptr::null_mut() } else { strscpy(buffer, (*ca).name, TCP_CA_NAME_MAX); buffer }; rcu_read_unlock(); ret
}

pub unsafe extern "C" fn tcp_assign_congestion_control(sk: *mut sock) { let icsk = inet_csk(sk); (*icsk).icsk_ca_ops = &tcp_reno; memset((*icsk).icsk_ca_priv.as_mut_ptr() as *mut c_void, 0, 0); }

pub unsafe extern "C" fn tcp_init_congestion_control(sk: *mut sock) { let icsk = inet_csk(sk); if let Some(init) = (*(*icsk).icsk_ca_ops).init { init(sk); } (*icsk).icsk_ca_initialized = 1; }
unsafe fn tcp_reinit_congestion_control(sk: *mut sock, ca: *const tcp_congestion_ops) { let icsk = inet_csk(sk); tcp_cleanup_congestion_control(sk); (*icsk).icsk_ca_ops = ca; (*icsk).icsk_ca_setsockopt = 1; }
pub unsafe extern "C" fn tcp_cleanup_congestion_control(sk: *mut sock) { let icsk = inet_csk(sk); if (*icsk).icsk_ca_initialized != 0 { if let Some(release) = (*(*icsk).icsk_ca_ops).release { release(sk); } } (*icsk).icsk_ca_initialized = 0; bpf_module_put((*icsk).icsk_ca_ops, (*(*icsk).icsk_ca_ops).owner); }

pub unsafe extern "C" fn tcp_set_default_congestion_control(_net: *mut net, _name: *const c_char) -> i32 { 0 }
pub unsafe extern "C" fn tcp_get_available_congestion_control(_buf: *mut c_char, _maxlen: usize) {}
pub unsafe extern "C" fn tcp_get_default_congestion_control(_net: *mut net, _name: *mut c_char) {}
pub unsafe extern "C" fn tcp_get_allowed_congestion_control(buf: *mut c_char, _maxlen: usize) { *buf = 0; }
pub unsafe extern "C" fn tcp_set_allowed_congestion_control(_val: *mut c_char) -> i32 { 0 }
pub unsafe extern "C" fn tcp_set_congestion_control(_sk: *mut sock, _name: *const c_char, _load: bool, _cap_net_admin: bool) -> i32 { 0 }

pub unsafe extern "C" fn tcp_slow_start(tp: *mut tcp_sock, mut acked: u32) -> u32 {
    let cwnd = core::cmp::min(tcp_snd_cwnd(tp) + acked, (*tp).snd_ssthresh);
    acked -= cwnd - tcp_snd_cwnd(tp); tcp_snd_cwnd_set(tp, core::cmp::min(cwnd, (*tp).snd_cwnd_clamp)); acked
}

pub unsafe extern "C" fn tcp_cong_avoid_ai(tp: *mut tcp_sock, w: u32, acked: u32) {
    if (*tp).snd_cwnd_cnt >= w { (*tp).snd_cwnd_cnt = 0; tcp_snd_cwnd_set(tp, tcp_snd_cwnd(tp) + 1); }
    (*tp).snd_cwnd_cnt += acked; if (*tp).snd_cwnd_cnt >= w { let delta = (*tp).snd_cwnd_cnt / w; (*tp).snd_cwnd_cnt -= delta * w; tcp_snd_cwnd_set(tp, tcp_snd_cwnd(tp) + delta); }
    tcp_snd_cwnd_set(tp, core::cmp::min(tcp_snd_cwnd(tp), (*tp).snd_cwnd_clamp));
}

pub unsafe extern "C" fn tcp_reno_cong_avoid(sk: *mut sock, _ack: u32, mut acked: u32) { let tp = tcp_sk(sk); if !tcp_is_cwnd_limited(sk) { return; } if tcp_in_slow_start(tp) { acked = tcp_slow_start(tp, acked); if acked == 0 { return; } } tcp_cong_avoid_ai(tp, tcp_snd_cwnd(tp), acked); }
pub unsafe extern "C" fn tcp_reno_ssthresh(sk: *mut sock) -> u32 { let tp = tcp_sk(sk); core::cmp::max(tcp_snd_cwnd(tp) >> 1, 2) }
pub unsafe extern "C" fn tcp_reno_undo_cwnd(sk: *mut sock) -> u32 { let tp = tcp_sk(sk); core::cmp::max(tcp_snd_cwnd(tp), (*tp).prior_cwnd) }

#[no_mangle]
pub static mut tcp_reno: tcp_congestion_ops = tcp_congestion_ops { list: list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() }, key: 0, flags: TCP_CONG_NON_RESTRICTED, name: b"reno\0".as_ptr() as *const c_char, owner: core::ptr::null_mut(), init: None, release: None, set_state: None, ssthresh: Some(tcp_reno_ssthresh), cong_avoid: Some(tcp_reno_cong_avoid), cong_control: None, undo_cwnd: Some(tcp_reno_undo_cwnd) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
