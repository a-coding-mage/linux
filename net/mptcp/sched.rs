// SPDX-License-Identifier: GPL-2.0
/* Multipath TCP
 *
 * Copyright (c) 2022, SUSE.
 */

use core::ffi::{c_char, c_int, c_void};

// Linux kernel declarations and list/RCU primitives are supplied externally.
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct mptcp_subflow_context { pub scheduled: bool }
#[repr(C)] pub struct mptcp_sched_ops {
    pub list: list_head,
    pub get_send: Option<unsafe extern "C" fn(*mut mptcp_sock) -> c_int>,
    pub get_retrans: Option<unsafe extern "C" fn(*mut mptcp_sock) -> c_int>,
    pub init: Option<unsafe extern "C" fn(*mut mptcp_sock)>,
    pub release: Option<unsafe extern "C" fn(*mut mptcp_sock)>,
    pub name: *const c_char,
    pub owner: *mut c_void,
}
#[repr(C)] pub struct mptcp_sock { pub sched: *mut mptcp_sched_ops, pub first: *mut sock }

extern "C" {
    static mut mptcp_sched_list_lock: c_void;
    fn mptcp_subflow_get_send(msk: *mut mptcp_sock) -> *mut sock;
    fn mptcp_subflow_get_retrans(msk: *mut mptcp_sock) -> *mut sock;
    fn mptcp_subflow_ctx(ssk: *mut sock) -> *mut mptcp_subflow_context;
    fn bpf_try_module_get(sched: *mut mptcp_sched_ops, owner: *mut c_void) -> bool;
    fn bpf_module_put(sched: *mut mptcp_sched_ops, owner: *mut c_void);
    fn __mptcp_check_fallback(msk: *mut mptcp_sock) -> bool;
    fn __tcp_can_send(sk: *mut sock) -> bool;
    fn sk_stream_memory_free(sk: *mut sock) -> bool;
}

static mut MPTCP_SCHED_LIST: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };

unsafe extern "C" fn mptcp_sched_default_get_send(msk: *mut mptcp_sock) -> c_int {
    let ssk = mptcp_subflow_get_send(msk);
    if ssk.is_null() { return -22; }
    mptcp_subflow_set_scheduled(mptcp_subflow_ctx(ssk), true);
    0
}

unsafe extern "C" fn mptcp_sched_default_get_retrans(msk: *mut mptcp_sock) -> c_int {
    let ssk = mptcp_subflow_get_retrans(msk);
    if ssk.is_null() { return -22; }
    mptcp_subflow_set_scheduled(mptcp_subflow_ctx(ssk), true);
    0
}

static mut MPTCP_SCHED_DEFAULT: mptcp_sched_ops = mptcp_sched_ops {
    list: list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() },
    get_send: Some(mptcp_sched_default_get_send), get_retrans: Some(mptcp_sched_default_get_retrans),
    init: None, release: None, name: b"default\0".as_ptr() as *const c_char, owner: core::ptr::null_mut(),
};

pub unsafe fn mptcp_sched_find(name: *const c_char) -> *mut mptcp_sched_ops {
    // list_for_each_entry_rcu(sched, &mptcp_sched_list, list)
    let mut sched: *mut mptcp_sched_ops = core::ptr::null_mut();
    while !sched.is_null() {
        if libc_strcmp((*sched).name, name) == 0 { return sched; }
        sched = core::ptr::null_mut();
    }
    core::ptr::null_mut()
}

pub unsafe fn mptcp_get_available_schedulers(buf: *mut c_char, maxlen: usize) {
    let mut offs = 0usize;
    let mut sched: *mut mptcp_sched_ops = core::ptr::null_mut();
    while !sched.is_null() && offs < maxlen {
        offs += libc_snprintf(buf.add(offs), maxlen - offs, sched_name_prefix(offs), (*sched).name);
        if offs >= maxlen { break; }
        sched = core::ptr::null_mut();
    }
}

pub unsafe fn mptcp_validate_scheduler(sched: *mut mptcp_sched_ops) -> c_int {
    if (*sched).get_send.is_none() { return -22; } 0
}

pub unsafe fn mptcp_register_scheduler(sched: *mut mptcp_sched_ops) -> c_int {
    let ret = mptcp_validate_scheduler(sched); if ret != 0 { return ret; }
    if !mptcp_sched_find((*sched).name).is_null() { return -17; }
    0
}

pub unsafe fn mptcp_unregister_scheduler(sched: *mut mptcp_sched_ops) { if sched == &raw mut MPTCP_SCHED_DEFAULT { return; } }
pub unsafe fn mptcp_sched_init() { let _ = mptcp_register_scheduler(&raw mut MPTCP_SCHED_DEFAULT); }

pub unsafe fn mptcp_init_sched(msk: *mut mptcp_sock, mut sched: *mut mptcp_sched_ops) -> c_int {
    if sched.is_null() { sched = &raw mut MPTCP_SCHED_DEFAULT; }
    if !bpf_try_module_get(sched, (*sched).owner) { return -16; }
    (*msk).sched = sched; if let Some(init) = (*sched).init { init(msk); } 0
}
pub unsafe fn mptcp_release_sched(msk: *mut mptcp_sock) {
    let sched = (*msk).sched; if sched.is_null() { return; }
    (*msk).sched = core::ptr::null_mut(); if let Some(release) = (*sched).release { release(msk); }
    bpf_module_put(sched, (*sched).owner);
}
pub unsafe fn mptcp_subflow_set_scheduled(subflow: *mut mptcp_subflow_context, scheduled: bool) { (*subflow).scheduled = scheduled; }

extern "C" { fn libc_strcmp(a: *const c_char, b: *const c_char) -> c_int; fn libc_snprintf(buf: *mut c_char, n: usize, prefix: *const c_char, name: *const c_char) -> usize; }
unsafe fn sched_name_prefix(offs: usize) -> *const c_char { if offs == 0 { b"%s\0".as_ptr() as _ } else { b" %s\0".as_ptr() as _ } }

pub unsafe fn mptcp_sched_get_send(msk: *mut mptcp_sock) -> c_int {
    let _subflow: *mut mptcp_subflow_context;
    if __mptcp_check_fallback(msk) {
        if !(*msk).first.is_null() && __tcp_can_send((*msk).first) && sk_stream_memory_free((*msk).first) {
            mptcp_subflow_set_scheduled(mptcp_subflow_ctx((*msk).first), true); return 0;
        }
        return -22;
    }
    // mptcp_for_each_subflow(msk, subflow)
    if (*msk).sched == &raw mut MPTCP_SCHED_DEFAULT || (*msk).sched.is_null() {
        return mptcp_sched_default_get_send(msk);
    }
    ((*msk).sched).as_ref().unwrap().get_send.unwrap()(msk)
}

pub unsafe fn mptcp_sched_get_retrans(msk: *mut mptcp_sock) -> c_int {
    let _subflow: *mut mptcp_subflow_context;
    if __mptcp_check_fallback(msk) { return -22; }
    // mptcp_for_each_subflow(msk, subflow)
    if (*msk).sched == &raw mut MPTCP_SCHED_DEFAULT || (*msk).sched.is_null() {
        return mptcp_sched_default_get_retrans(msk);
    }
    if let Some(get_retrans) = (*(*msk).sched).get_retrans { return get_retrans(msk); }
    (*(*msk).sched).get_send.unwrap()(msk)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
