/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2019 Facebook */

// C header dependencies are supplied by other translated units:
// linux/rculist.h, linux/list.h, linux/hash.h, linux/types.h,
// linux/spinlock.h, linux/bpf.h, net/sock.h, uapi/linux/sock_diag.h,
// uapi/linux/btf.h, and linux/bpf_local_storage.h.

pub struct sock;
pub struct bpf_func_proto;
pub struct bpf_local_storage_elem;
pub struct bpf_sk_storage_diag;
pub struct sk_buff;
pub struct nlattr;

extern "C" {
    pub fn bpf_sk_storage_free(sk: *mut sock);

    pub static bpf_sk_storage_get_proto: bpf_func_proto;
    pub static bpf_sk_storage_delete_proto: bpf_func_proto;
    pub static bpf_sk_storage_get_tracing_proto: bpf_func_proto;
    pub static bpf_sk_storage_delete_tracing_proto: bpf_func_proto;
}

#[cfg(CONFIG_BPF_SYSCALL)]
extern "C" {
    pub fn bpf_sk_storage_clone(sk: *const sock, newsk: *mut sock) -> ::core::ffi::c_int;

    pub fn bpf_sk_storage_diag_alloc(
        nla_stgs: *const nlattr,
    ) -> *mut bpf_sk_storage_diag;

    pub fn bpf_sk_storage_diag_free(diag: *mut bpf_sk_storage_diag);

    pub fn bpf_sk_storage_diag_put(
        diag: *mut bpf_sk_storage_diag,
        sk: *mut sock,
        skb: *mut sk_buff,
        stg_array_type: ::core::ffi::c_int,
        res_diag_size: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
}

#[cfg(not(CONFIG_BPF_SYSCALL))]
#[inline]
pub unsafe fn bpf_sk_storage_clone(_sk: *const sock, _newsk: *mut sock) -> ::core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_BPF_SYSCALL))]
#[inline]
pub unsafe fn bpf_sk_storage_diag_alloc(_nla: *const nlattr) -> *mut bpf_sk_storage_diag {
    ::core::ptr::null_mut()
}

#[cfg(not(CONFIG_BPF_SYSCALL))]
#[inline]
pub unsafe fn bpf_sk_storage_diag_free(_diag: *mut bpf_sk_storage_diag) {}

#[cfg(not(CONFIG_BPF_SYSCALL))]
#[inline]
pub unsafe fn bpf_sk_storage_diag_put(
    _diag: *mut bpf_sk_storage_diag,
    _sk: *mut sock,
    _skb: *mut sk_buff,
    _stg_array_type: ::core::ffi::c_int,
    _res_diag_size: *mut ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
