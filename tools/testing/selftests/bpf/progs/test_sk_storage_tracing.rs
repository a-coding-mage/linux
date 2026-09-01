// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

// C dependencies translated as external intent:
// <vmlinux.h>, <bpf/bpf_tracing.h>, <bpf/bpf_core_read.h>,
// and <bpf/bpf_helpers.h>.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type __u32 = u32;

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct proto_accept_arg {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    pub comm: [::core::ffi::c_char; 16],
}

#[repr(C)]
pub struct request_sock {
    pub sk: *mut sock,
}

#[repr(C)]
pub struct sk_buff {
    pub sk: *mut sock,
}

#[repr(C)]
pub struct sk_stg {
    pub pid: __u32,
    pub last_notclose_state: __u32,
    pub comm: [::core::ffi::c_char; 16],
}

// struct {
//     __uint(type, BPF_MAP_TYPE_SK_STORAGE);
//     __uint(map_flags, BPF_F_NO_PREALLOC);
//     __type(key, int);
//     __type(value, struct sk_stg);
// } sk_stg_map SEC(".maps");
#[repr(C)]
pub struct sk_stg_map_def {
    _private: [u8; 0],
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut sk_stg_map: sk_stg_map_def = sk_stg_map_def { _private: [] };

/* Testing delete */
// struct {
//     __uint(type, BPF_MAP_TYPE_SK_STORAGE);
//     __uint(map_flags, BPF_F_NO_PREALLOC);
//     __type(key, int);
//     __type(value, int);
// } del_sk_stg_map SEC(".maps");
#[repr(C)]
pub struct del_sk_stg_map_def {
    _private: [u8; 0],
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut del_sk_stg_map: del_sk_stg_map_def = del_sk_stg_map_def { _private: [] };

#[no_mangle]
pub static mut task_comm: [::core::ffi::c_char; 16] = [0; 16];

extern "C" {
    static BPF_TCP_CLOSE: ::core::ffi::c_int;
    static BPF_SK_STORAGE_GET_F_CREATE: u64;

    fn bpf_sk_storage_get(
        map: *mut ::core::ffi::c_void,
        sk: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
        flags: u64,
    ) -> *mut ::core::ffi::c_void;
    fn bpf_sk_storage_delete(
        map: *mut ::core::ffi::c_void,
        sk: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_long;
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_get_current_task() -> u64;
    fn bpf_core_read_str(
        dst: *mut ::core::ffi::c_void,
        sz: u32,
        src: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_long;
}

#[no_mangle]
#[link_section = "tp_btf/inet_sock_set_state"]
pub unsafe extern "C" fn trace_inet_sock_set_state(
    sk: *mut sock,
    oldstate: ::core::ffi::c_int,
    newstate: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut stg: *mut sk_stg;

    let _ = oldstate;

    if newstate == BPF_TCP_CLOSE {
        return 0;
    }

    stg = bpf_sk_storage_get(
        &mut sk_stg_map as *mut _ as *mut ::core::ffi::c_void,
        sk as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut(),
        BPF_SK_STORAGE_GET_F_CREATE,
    ) as *mut sk_stg;
    if stg.is_null() {
        return 0;
    }

    (*stg).last_notclose_state = newstate as __u32;

    bpf_sk_storage_delete(
        &mut del_sk_stg_map as *mut _ as *mut ::core::ffi::c_void,
        sk as *mut ::core::ffi::c_void,
    );

    0
}

unsafe fn set_task_info(sk: *mut sock) {
    let mut task: *mut task_struct;
    let mut stg: *mut sk_stg;

    stg = bpf_sk_storage_get(
        &mut sk_stg_map as *mut _ as *mut ::core::ffi::c_void,
        sk as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut(),
        BPF_SK_STORAGE_GET_F_CREATE,
    ) as *mut sk_stg;
    if stg.is_null() {
        return;
    }

    (*stg).pid = bpf_get_current_pid_tgid() as __u32;

    task = bpf_get_current_task() as *mut task_struct;
    bpf_core_read_str(
        &mut (*stg).comm as *mut _ as *mut ::core::ffi::c_void,
        ::core::mem::size_of_val(&(*stg).comm) as u32,
        &(*task).comm as *const _ as *const ::core::ffi::c_void,
    );
    bpf_core_read_str(
        &mut task_comm as *mut _ as *mut ::core::ffi::c_void,
        ::core::mem::size_of_val(&task_comm) as u32,
        &(*task).comm as *const _ as *const ::core::ffi::c_void,
    );
}

#[no_mangle]
#[link_section = "fentry/inet_csk_listen_start"]
pub unsafe extern "C" fn trace_inet_csk_listen_start(sk: *mut sock) -> ::core::ffi::c_int {
    set_task_info(sk);

    0
}

#[no_mangle]
#[link_section = "fentry/tcp_connect"]
pub unsafe extern "C" fn trace_tcp_connect(sk: *mut sock) -> ::core::ffi::c_int {
    set_task_info(sk);

    0
}

#[no_mangle]
#[link_section = "fexit/inet_csk_accept"]
pub unsafe extern "C" fn inet_csk_accept(
    sk: *mut sock,
    arg: *mut proto_accept_arg,
    accepted_sk: *mut sock,
) -> ::core::ffi::c_int {
    let _ = sk;
    let _ = arg;

    set_task_info(accepted_sk);

    0
}

#[no_mangle]
#[link_section = "tp_btf/tcp_retransmit_synack"]
pub unsafe extern "C" fn tcp_retransmit_synack(
    sk: *mut sock,
    req: *mut request_sock,
) -> ::core::ffi::c_int {
    /* load only test */
    bpf_sk_storage_get(
        &mut sk_stg_map as *mut _ as *mut ::core::ffi::c_void,
        sk as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut(),
        0,
    );
    bpf_sk_storage_get(
        &mut sk_stg_map as *mut _ as *mut ::core::ffi::c_void,
        (*req).sk as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut(),
        0,
    );
    0
}

#[no_mangle]
#[link_section = "tp_btf/tcp_bad_csum"]
pub unsafe extern "C" fn tcp_bad_csum(skb: *mut sk_buff) -> ::core::ffi::c_int {
    bpf_sk_storage_get(
        &mut sk_stg_map as *mut _ as *mut ::core::ffi::c_void,
        (*skb).sk as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut(),
        0,
    );
    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [::core::ffi::c_char; 4] = [b'G' as _, b'P' as _, b'L' as _, 0];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
