// SPDX-License-Identifier: GPL-2.0
/* Copyright 2025 Google LLC */

/* Depends on bpf_tracing_net.h, bpf/bpf_helpers.h, bpf/bpf_tracing.h,
 * errno.h, and err.h from the original C environment.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

type __u32 = u32;

const SOL_SOCKET: i32 = 1;
const SK_BPF_BYPASS_PROT_MEM: i32 = 69;
const EINVAL: i32 = 22;
const EFAULT: i32 = 14;

#[repr(C)]
pub struct bpf_sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sock {
    pub __sk_common: sock_common,
}

#[repr(C)]
pub struct sock_common {
    pub skc_prot: *mut proto,
}

#[repr(C)]
pub struct proto {
    pub memory_allocated: *mut percpu_counter,
}

#[repr(C)]
pub struct percpu_counter {
    pub counter: i64,
}

#[repr(C)]
struct sk_prot {
    memory_allocated: *mut i64,
    memory_per_cpu_fw_alloc: *mut i32,
}

extern "C" {
    #[link_name = "tcp_memory_per_cpu_fw_alloc"]
    static mut tcp_memory_per_cpu_fw_alloc: i32;
    #[link_name = "udp_memory_per_cpu_fw_alloc"]
    static mut udp_memory_per_cpu_fw_alloc: i32;

    fn bpf_per_cpu_ptr(ptr: *mut i32, cpu: __u32) -> *mut i32;
    fn bpf_loop(
        nr_loops: i32,
        callback_fn: unsafe extern "C" fn(__u32, *mut sk_prot) -> i32,
        callback_ctx: *mut sk_prot,
        flags: u64,
    ) -> i64;
    fn bpf_setsockopt(
        ctx: *mut bpf_sock,
        level: i32,
        optname: i32,
        optval: *const core::ffi::c_void,
        optlen: u32,
    ) -> i32;
    fn bpf_getsockopt(
        ctx: *mut bpf_sock,
        level: i32,
        optname: i32,
        optval: *mut core::ffi::c_void,
        optlen: u32,
    ) -> i32;
    fn set_if_not_errno_or_zero(err: i32, fallback: i32);
    fn bpf_set_retval(retval: i32) -> i32;
}

#[no_mangle]
static mut nr_cpus: i32 = 0;
#[no_mangle]
static mut tcp_activated: bool = false;
#[no_mangle]
static mut udp_activated: bool = false;
#[no_mangle]
static mut tcp_memory_allocated: i64 = 0;
#[no_mangle]
static mut udp_memory_allocated: i64 = 0;

unsafe extern "C" fn drain_memory_per_cpu_fw_alloc(
    i: __u32,
    sk_prot_ctx: *mut sk_prot,
) -> i32 {
    let memory_per_cpu_fw_alloc: *mut i32;

    memory_per_cpu_fw_alloc =
        bpf_per_cpu_ptr((*sk_prot_ctx).memory_per_cpu_fw_alloc, i);
    if !memory_per_cpu_fw_alloc.is_null() {
        *(*sk_prot_ctx).memory_allocated += *memory_per_cpu_fw_alloc as i64;
    }

    0
}

unsafe fn get_memory_allocated(_sk: *mut sock, memory_per_cpu_fw_alloc: *mut i32) -> i64 {
    let sk: *mut sock = _sk as *mut sock;
    let mut sk_prot_ctx: sk_prot;
    let mut memory_allocated: i64;

    /* net_aligned_data.{tcp,udp}_memory_allocated was not available. */
    memory_allocated = (*(*(*sk).__sk_common.skc_prot).memory_allocated).counter;

    sk_prot_ctx = sk_prot {
        memory_allocated: &mut memory_allocated,
        memory_per_cpu_fw_alloc,
    };

    bpf_loop(
        nr_cpus,
        drain_memory_per_cpu_fw_alloc,
        &mut sk_prot_ctx,
        0,
    );

    memory_allocated
}

unsafe fn fentry_init_sock(
    sk: *mut sock,
    activated: *mut bool,
    memory_allocated: *mut i64,
    memory_per_cpu_fw_alloc: *mut i32,
) {
    if !*activated {
        return;
    }

    *memory_allocated = get_memory_allocated(sk, memory_per_cpu_fw_alloc);
    *activated = false;
}

#[no_mangle]
#[link_section = "fentry/tcp_init_sock"]
pub unsafe extern "C" fn fentry_tcp_init_sock(sk: *mut sock) -> i32 {
    fentry_init_sock(
        sk,
        core::ptr::addr_of_mut!(tcp_activated),
        core::ptr::addr_of_mut!(tcp_memory_allocated),
        core::ptr::addr_of_mut!(tcp_memory_per_cpu_fw_alloc),
    );
    0
}

#[no_mangle]
#[link_section = "fentry/udp_init_sock"]
pub unsafe extern "C" fn fentry_udp_init_sock(sk: *mut sock) -> i32 {
    fentry_init_sock(
        sk,
        core::ptr::addr_of_mut!(udp_activated),
        core::ptr::addr_of_mut!(udp_memory_allocated),
        core::ptr::addr_of_mut!(udp_memory_per_cpu_fw_alloc),
    );
    0
}

#[no_mangle]
#[link_section = "cgroup/sock_create"]
pub unsafe extern "C" fn sock_create(ctx: *mut bpf_sock) -> i32 {
    let mut err: i32;
    let mut val: i32 = 1;

    err = bpf_setsockopt(
        ctx,
        SOL_SOCKET,
        SK_BPF_BYPASS_PROT_MEM,
        &val as *const _ as *const core::ffi::c_void,
        core::mem::size_of_val(&val) as u32,
    );
    if err != 0 {
        set_if_not_errno_or_zero(err, -EFAULT);
        bpf_set_retval(err);
        return 0;
    }

    val = 0;

    err = bpf_getsockopt(
        ctx,
        SOL_SOCKET,
        SK_BPF_BYPASS_PROT_MEM,
        &mut val as *mut _ as *mut core::ffi::c_void,
        core::mem::size_of_val(&val) as u32,
    );
    if err != 0 {
        set_if_not_errno_or_zero(err, -EFAULT);
        bpf_set_retval(err);
        return 0;
    }

    if val != 1 {
        err = -EINVAL;
        set_if_not_errno_or_zero(err, -EFAULT);
        bpf_set_retval(err);
        return 0;
    }

    1
}

#[no_mangle]
#[link_section = "license"]
pub static LICENSE: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
