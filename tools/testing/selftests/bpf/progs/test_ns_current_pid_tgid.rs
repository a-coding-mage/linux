// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 Carlos Neira cneirabustos@gmail.com */

// C dependencies:
// #include <linux/bpf.h>
// #include <stdint.h>
// #include <bpf/bpf_helpers.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type __u32 = u32;
type __u64 = u64;

const BPF_MAP_TYPE_SOCKMAP: u32 = 15;
const SK_PASS: i32 = 1;

#[repr(C)]
pub struct bpf_pidns_info {
    pub pid: __u32,
    pub tgid: __u32,
}

#[repr(C)]
pub struct bpf_sock_addr {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct sk_msg_md {
    _unused: [u8; 0],
}

unsafe extern "C" {
    fn bpf_get_ns_current_pid_tgid(
        dev: __u64,
        ino: __u64,
        nsdata: *mut bpf_pidns_info,
        size: u32,
    ) -> i64;
}

#[repr(C)]
pub struct sock_map {
    pub type_: u32,
    pub max_entries: u32,
    pub key: __u32,
    pub value: __u32,
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut sock_map: sock_map = sock_map {
    type_: BPF_MAP_TYPE_SOCKMAP,
    max_entries: 2,
    key: 0,
    value: 0,
};

#[unsafe(no_mangle)]
pub static mut user_pid: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut user_tgid: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut dev: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut ino: __u64 = 0;

unsafe fn get_pid_tgid() {
    let mut nsdata: bpf_pidns_info = core::mem::zeroed();

    if bpf_get_ns_current_pid_tgid(
        dev,
        ino,
        &mut nsdata,
        core::mem::size_of::<bpf_pidns_info>() as u32,
    ) != 0
    {
        return;
    }

    user_pid = nsdata.pid as __u64;
    user_tgid = nsdata.tgid as __u64;
}

#[unsafe(link_section = "?tracepoint/syscalls/sys_enter_nanosleep")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tp_handler(ctx: *const core::ffi::c_void) -> i32 {
    let _ = ctx;
    get_pid_tgid();
    return 0;
}

#[unsafe(link_section = "?cgroup/bind4")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cgroup_bind4(ctx: *mut bpf_sock_addr) -> i32 {
    let _ = ctx;
    get_pid_tgid();
    return 1;
}

#[unsafe(link_section = "?sk_msg")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sk_msg(msg: *mut sk_msg_md) -> i32 {
    let _ = msg;
    get_pid_tgid();
    return SK_PASS;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
