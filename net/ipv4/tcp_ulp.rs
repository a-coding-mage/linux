// SPDX-License-Identifier: GPL-2.0-only
/*
 * Pluggable TCP upper layer protocol support.
 *
 * Copyright (c) 2016-2017, Mellanox Technologies. All rights reserved.
 * Copyright (c) 2016-2017, Dave Watson <davejwatson@fb.com>. All rights reserved.
 */

// Linux kernel dependencies supplied by other translation units.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct socket { pub flags: c_ulong }
#[repr(C)] pub struct proto { _private: [u8; 0] }
#[repr(C)] pub struct sock { pub sk_socket: *mut socket, pub sk_state: c_int }
#[repr(C)] pub struct inet_connection_sock {
    pub icsk_ulp_ops: *const tcp_ulp_ops,
}

pub type c_ulong = usize;
pub type size_t = usize;
pub type write_space_fn = Option<unsafe extern "C" fn(*mut sock)>;

#[repr(C)]
pub struct tcp_ulp_ops {
    pub list: list_head,
    pub name: *const c_char,
    pub owner: *mut module,
    pub clone: Option<unsafe extern "C" fn(*mut sock) -> c_int>,
    pub init: unsafe extern "C" fn(*mut sock) -> c_int,
    pub update: Option<unsafe extern "C" fn(*mut sock, *mut proto, write_space_fn)>,
    pub release: Option<unsafe extern "C" fn(*mut sock)>,
}

extern "C" {
    static mut tcp_ulp_list_lock: spinlock_t;
    static mut tcp_ulp_list: list_head;
    fn inet_csk(sk: *mut sock) -> *mut inet_connection_sock;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn capable(cap: c_int) -> bool;
    fn request_module(fmt: *const c_char, ...);
    fn try_module_get(owner: *mut module) -> bool;
    fn module_put(owner: *mut module);
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn synchronize_rcu();
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn list_add_tail_rcu(new: *mut list_head, head: *mut list_head);
    fn list_del_rcu(entry: *mut list_head);
    fn clear_bit(nr: usize, addr: *mut c_ulong);
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn warn_on_once(condition: bool) -> bool;
    fn sock_owned_by_me(sk: *mut sock);
}

const EEXIST: c_int = 17;
const ENOTCONN: c_int = 107;
const ENOENT: c_int = 2;
const CAP_NET_ADMIN: c_int = 12;
const TCP_LISTEN: c_int = 10;
const SOCK_SUPPORT_ZC: usize = 5;

/* Simple linear search, don't expect many entries! */
unsafe fn tcp_ulp_find(name: *const c_char) -> *mut tcp_ulp_ops {
    // list_for_each_entry_rcu(e, &tcp_ulp_list, list, lockdep_is_held(...))
    // is the kernel list traversal represented here by its direct C semantics.
    let mut e: *mut tcp_ulp_ops = core::ptr::null_mut();
    while !e.is_null() {
        if strcmp((*e).name, name) == 0 { return e; }
        e = (*((*e).list.next as *mut tcp_ulp_ops)).list.next as *mut tcp_ulp_ops;
    }
    core::ptr::null_mut()
}

unsafe fn __tcp_ulp_find_autoload(name: *const c_char) -> *const tcp_ulp_ops {
    let mut ulp: *const tcp_ulp_ops = core::ptr::null();
    rcu_read_lock();
    ulp = tcp_ulp_find(name);
    if ulp.is_null() && capable(CAP_NET_ADMIN) {
        rcu_read_unlock();
        request_module(b"tcp-ulp-%s\0".as_ptr() as *const c_char, name);
        rcu_read_lock();
        ulp = tcp_ulp_find(name);
    }
    if ulp.is_null() || !try_module_get((*ulp).owner) { ulp = core::ptr::null(); }
    rcu_read_unlock();
    ulp
}

/* Attach new upper layer protocol to the list of available protocols. */
#[no_mangle]
pub unsafe extern "C" fn tcp_register_ulp(ulp: *mut tcp_ulp_ops) -> c_int {
    let mut ret: c_int = 0;
    spin_lock(&raw mut tcp_ulp_list_lock);
    if !tcp_ulp_find((*ulp).name).is_null() { ret = -EEXIST; }
    else { list_add_tail_rcu(&mut (*ulp).list, &raw mut tcp_ulp_list); }
    spin_unlock(&raw mut tcp_ulp_list_lock);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn tcp_unregister_ulp(ulp: *mut tcp_ulp_ops) {
    spin_lock(&raw mut tcp_ulp_list_lock);
    list_del_rcu(&mut (*ulp).list);
    spin_unlock(&raw mut tcp_ulp_list_lock);
    synchronize_rcu();
}

/* Build string with list of available upper layer protocl values */
#[no_mangle]
pub unsafe extern "C" fn tcp_get_available_ulp(buf: *mut c_char, maxlen: size_t) {
    let mut offs: size_t = 0;
    *buf = 0;
    rcu_read_lock();
    // list_for_each_entry_rcu(ulp_ops, &tcp_ulp_list, list)
    let mut ulp_ops: *mut tcp_ulp_ops = core::ptr::null_mut();
    while !ulp_ops.is_null() {
        offs += snprintf(buf.add(offs), maxlen - offs, b"%s%s\0".as_ptr() as *const c_char,
                         if offs == 0 { b"\0".as_ptr() } else { b" \0".as_ptr() }, (*ulp_ops).name);
        if warn_on_once(offs >= maxlen) { break; }
    }
    rcu_read_unlock();
}

#[no_mangle]
pub unsafe extern "C" fn tcp_update_ulp(sk: *mut sock, proto: *mut proto, write_space: write_space_fn) {
    let icsk = inet_csk(sk);
    if let Some(update) = (*(*icsk).icsk_ulp_ops).update { update(sk, proto, write_space); }
}

#[no_mangle]
pub unsafe extern "C" fn tcp_cleanup_ulp(sk: *mut sock) {
    let icsk = inet_csk(sk);
    /* No sock_owned_by_me() check here as the socket is dead and about to be destroyed. */
    if (*icsk).icsk_ulp_ops.is_null() { return; }
    let ops = (*icsk).icsk_ulp_ops;
    if let Some(release) = (*ops).release { release(sk); }
    module_put((*ops).owner);
    (*icsk).icsk_ulp_ops = core::ptr::null();
}

unsafe fn __tcp_set_ulp(sk: *mut sock, ulp_ops: *const tcp_ulp_ops) -> c_int {
    let icsk = inet_csk(sk);
    let mut err = -EEXIST;
    if !(*icsk).icsk_ulp_ops.is_null() { module_put((*ulp_ops).owner); return err; }
    if !(*sk).sk_socket.is_null() { clear_bit(SOCK_SUPPORT_ZC, &mut (*(*sk).sk_socket).flags); }
    err = -ENOTCONN;
    if (*ulp_ops).clone.is_none() && (*sk).sk_state == TCP_LISTEN { module_put((*ulp_ops).owner); return err; }
    err = ((*ulp_ops).init)(sk);
    if err != 0 { module_put((*ulp_ops).owner); return err; }
    (*icsk).icsk_ulp_ops = ulp_ops;
    0
}

#[no_mangle]
pub unsafe extern "C" fn tcp_set_ulp(sk: *mut sock, name: *const c_char) -> c_int {
    sock_owned_by_me(sk);
    let ulp_ops = __tcp_ulp_find_autoload(name);
    if ulp_ops.is_null() { return -ENOENT; }
    __tcp_set_ulp(sk, ulp_ops)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
