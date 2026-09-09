// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * linux/ipc/msgutil.c
 * Copyright (C) 1999, 2004 Manfred Spraul
 */

use core::ffi::{c_char, c_int, c_void};

// Kernel types, constants, globals, and functions supplied by other files.
#[repr(C)]
pub struct msg_msg {
    pub next: *mut msg_msgseg,
    pub security: *mut c_void,
    pub m_type: i64,
    pub m_ts: usize,
}

#[repr(C)]
pub struct msg_msgseg {
    pub next: *mut msg_msgseg,
    // The next part of the message follows immediately.
}

#[repr(C)]
pub struct ipc_namespace {
    pub ns: c_void,
    pub user_ns: *mut c_void,
}

#[repr(C)]
pub struct kmem_buckets {
    _private: [u8; 0],
}

extern "C" {
    pub static mut mq_lock: c_void;
    pub static mut init_ipc_ns: ipc_namespace;
    pub static mut init_user_ns: c_void;
    pub static PAGE_SIZE: usize;
    pub static mut msg_buckets: *mut kmem_buckets;

    fn kmem_buckets_create(name: *const c_char, flags: usize, size: usize,
                            data_len: usize, ctor: *mut c_void) -> *mut kmem_buckets;
    fn kmem_buckets_alloc(bucket: *mut kmem_buckets, size: usize, flags: usize) -> *mut c_void;
    fn kmalloc(size: usize, flags: usize) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn cond_resched();
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
    fn security_msg_msg_alloc(msg: *mut msg_msg) -> c_int;
    fn security_msg_msg_free(msg: *mut msg_msg);
}

const SLAB_ACCOUNT: usize = 0;
const GFP_KERNEL: usize = 0;
const GFP_KERNEL_ACCOUNT: usize = 0;
const EFAULT: c_int = 14;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENOSYS: c_int = 38;

static mut MSG_BUCKETS: *mut kmem_buckets = core::ptr::null_mut();

unsafe fn datalen_msg() -> usize { PAGE_SIZE - core::mem::size_of::<msg_msg>() }
unsafe fn datalen_seg() -> usize { PAGE_SIZE - core::mem::size_of::<msg_msgseg>() }

unsafe fn init_msg_buckets() -> c_int {
    MSG_BUCKETS = kmem_buckets_create(
        b"msg_msg\0".as_ptr() as *const c_char, SLAB_ACCOUNT,
        core::mem::size_of::<msg_msg>(), datalen_msg(), core::ptr::null_mut());
    0
}

unsafe fn alloc_msg(mut len: usize) -> *mut msg_msg {
    let mut alen = core::cmp::min(len, datalen_msg());
    let msg = kmem_buckets_alloc(MSG_BUCKETS, core::mem::size_of::<msg_msg>() + alen,
                                  GFP_KERNEL) as *mut msg_msg;
    if msg.is_null() { return core::ptr::null_mut(); }

    (*msg).next = core::ptr::null_mut();
    (*msg).security = core::ptr::null_mut();
    len -= alen;
    let mut pseg = &mut (*msg).next as *mut *mut msg_msgseg;
    while len > 0 {
        cond_resched();
        alen = core::cmp::min(len, datalen_seg());
        let seg = kmalloc(core::mem::size_of::<msg_msgseg>() + alen,
                          GFP_KERNEL_ACCOUNT) as *mut msg_msgseg;
        if seg.is_null() { free_msg(msg); return core::ptr::null_mut(); }
        *pseg = seg;
        (*seg).next = core::ptr::null_mut();
        pseg = &mut (*seg).next;
        len -= alen;
    }
    msg
}

pub unsafe fn load_msg(src: *const c_void, mut len: usize) -> *mut msg_msg {
    let msg = alloc_msg(len);
    if msg.is_null() { return (-ENOMEM as isize) as *mut msg_msg; }
    let mut alen = core::cmp::min(len, datalen_msg());
    if copy_from_user(msg.add(1) as *mut c_void, src, alen) != 0 {
        free_msg(msg); return (-EFAULT as isize) as *mut msg_msg;
    }
    let mut seg = (*msg).next;
    let mut source = src as *const u8;
    while !seg.is_null() {
        len -= alen; source = source.add(alen); alen = core::cmp::min(len, datalen_seg());
        if copy_from_user(seg.add(1) as *mut c_void, source as *const c_void, alen) != 0 {
            free_msg(msg); return (-EFAULT as isize) as *mut msg_msg;
        }
        seg = (*seg).next;
    }
    let err = security_msg_msg_alloc(msg);
    if err != 0 { free_msg(msg); return (-(err as isize)) as *mut msg_msg; }
    msg
}

pub unsafe fn copy_msg(src: *mut msg_msg, dst: *mut msg_msg) -> *mut msg_msg {
    if (*src).m_ts > (*dst).m_ts { return (-EINVAL as isize) as *mut msg_msg; }
    let mut len = (*src).m_ts;
    let mut alen = core::cmp::min(len, datalen_msg());
    core::ptr::copy_nonoverlapping(src.add(1) as *const u8, dst.add(1) as *mut u8, alen);
    let mut sp = (*src).next; let mut dp = (*dst).next;
    while !sp.is_null() {
        len -= alen; alen = core::cmp::min(len, datalen_seg());
        core::ptr::copy_nonoverlapping(sp.add(1) as *const u8, dp.add(1) as *mut u8, alen);
        sp = (*sp).next; dp = (*dp).next;
    }
    (*dst).m_type = (*src).m_type; (*dst).m_ts = (*src).m_ts; dst
}

pub unsafe fn store_msg(mut dest: *mut c_void, msg: *mut msg_msg, mut len: usize) -> c_int {
    let mut alen = core::cmp::min(len, datalen_msg());
    if copy_to_user(dest, msg.add(1) as *const c_void, alen) != 0 { return -1; }
    let mut seg = (*msg).next;
    while !seg.is_null() {
        len -= alen; dest = (dest as *mut u8).add(alen) as *mut c_void;
        alen = core::cmp::min(len, datalen_seg());
        if copy_to_user(dest, seg.add(1) as *const c_void, alen) != 0 { return -1; }
        seg = (*seg).next;
    }
    0
}

pub unsafe fn free_msg(msg: *mut msg_msg) {
    security_msg_msg_free(msg);
    let mut seg = (*msg).next;
    kfree(msg as *mut c_void);
    while !seg.is_null() {
        let tmp = (*seg).next; cond_resched(); kfree(seg as *mut c_void); seg = tmp;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
