/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2026 Isovalent */

// C include dependency: "errno.h"

pub const SOCK_DGRAM: i32 = 2;
pub const IPPROTO_UDP: i32 = 17;

unsafe extern "C" {
    pub fn bpf_ksock_create(
        opts: *const bpf_ksock_create_opts,
        opts__sz: u32,
        err__uninit: *mut i32,
    ) -> *mut bpf_ksock;
    pub fn bpf_ksock_connect(
        ks: *mut bpf_ksock,
        addr: *const bpf_ksock_addr,
        addr__sz: u32,
    ) -> i32;
    pub fn bpf_ksock_acquire(ks: *mut bpf_ksock) -> *mut bpf_ksock;
    pub fn bpf_ksock_release(ks: *mut bpf_ksock);
    pub fn bpf_ksock_send(ks: *mut bpf_ksock, data: *const core::ffi::c_void, data__sz: u32) -> i32;
    pub fn bpf_rcu_read_lock();
    pub fn bpf_rcu_read_unlock();

    pub fn bpf_map_lookup_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    pub fn bpf_kptr_xchg(kptr: *mut *mut bpf_ksock, ptr: *mut bpf_ksock) -> *mut bpf_ksock;
}

#[repr(C)]
pub struct __ksock_ctx_value {
    pub ctx: *mut bpf_ksock,
}

// Original BPF map declaration:
// struct {
//     __uint(type, BPF_MAP_TYPE_ARRAY);
//     __type(key, int);
//     __type(value, struct __ksock_ctx_value);
//     __uint(max_entries, 1);
// } __ksock_ctx_map SEC(".maps");
unsafe extern "C" {
    pub static mut __ksock_ctx_map: core::ffi::c_void;
}

#[inline]
pub unsafe fn ksock_ctx_value_lookup() -> *mut __ksock_ctx_value {
    let mut key: u32 = 0;

    unsafe {
        bpf_map_lookup_elem(
            core::ptr::addr_of_mut!(__ksock_ctx_map),
            (&mut key as *mut u32).cast::<core::ffi::c_void>(),
        )
        .cast::<__ksock_ctx_value>()
    }
}

#[inline]
pub unsafe fn ksock_ctx_get() -> *mut bpf_ksock {
    let v: *mut __ksock_ctx_value;
    let mut ks: *mut bpf_ksock = core::ptr::null_mut();
    let tmp: *mut bpf_ksock;

    v = unsafe { ksock_ctx_value_lookup() };
    if v.is_null() {
        return core::ptr::null_mut();
    }

    unsafe {
        bpf_rcu_read_lock();
        tmp = (*v).ctx;
        if !tmp.is_null() {
            ks = bpf_ksock_acquire(tmp);
        }
        bpf_rcu_read_unlock();
    }

    ks
}

#[inline]
pub unsafe fn ksock_ctx_insert(ctx: *mut bpf_ksock) -> i32 {
    let v: *mut __ksock_ctx_value;
    let old: *mut bpf_ksock;

    v = unsafe { ksock_ctx_value_lookup() };
    if v.is_null() {
        unsafe {
            bpf_ksock_release(ctx);
        }
        return -ENOENT;
    }

    old = unsafe { bpf_kptr_xchg(core::ptr::addr_of_mut!((*v).ctx), ctx) };
    if !old.is_null() {
        unsafe {
            bpf_ksock_release(old);
        }
        return -EEXIST;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
