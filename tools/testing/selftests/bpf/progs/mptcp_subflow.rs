// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020, Tessares SA. */
/* Copyright (c) 2024, Kylin Software */

/* vmlinux.h, bpf_helpers.h and other 'define' */
/* Depends on Rust equivalents of bpf_tracing_net.h and mptcp_bpf.h. */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub type __u32 = u32;

pub const TCP_CA_NAME_MAX: usize = 16;
pub const BPF_MAP_TYPE_HASH: u32 = 1;
pub const BPF_ANY: u64 = 0;
pub const BPF_SOCK_OPS_TCP_CONNECT_CB: i32 = 2;
pub const SOL_SOCKET: i32 = 1;
pub const SO_MARK: i32 = 36;
pub const SOL_TCP: i32 = 6;
pub const TCP_CONGESTION: i32 = 13;
pub const IPPROTO_MPTCP: i32 = 262;

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut cc: [u8; TCP_CA_NAME_MAX] = [
    b'r', b'e', b'n', b'o', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

#[no_mangle]
pub static mut pid: i32 = 0;

/* Associate a subflow counter to each token */
#[repr(C)]
pub struct mptcp_sf_map {
    /* __uint(type, BPF_MAP_TYPE_HASH); */
    pub type_: u32,
    /* __uint(key_size, sizeof(__u32)); */
    pub key_size: u32,
    /* __uint(value_size, sizeof(__u32)); */
    pub value_size: u32,
    /* __uint(max_entries, 100); */
    pub max_entries: u32,
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut mptcp_sf: mptcp_sf_map = mptcp_sf_map {
    type_: BPF_MAP_TYPE_HASH,
    key_size: core::mem::size_of::<__u32>() as u32,
    value_size: core::mem::size_of::<__u32>() as u32,
    max_entries: 100,
};

#[repr(C)]
pub struct bpf_sock_ops {
    pub op: i32,
    pub sk: *mut bpf_sock,
}

#[repr(C)]
pub struct bpf_sock {
    pub protocol: i32,
}

#[repr(C)]
pub struct bpf_sockopt {
    pub sk: *mut bpf_sock,
    pub level: i32,
    pub optname: i32,
    pub retval: i32,
}

#[repr(C)]
pub struct mptcp_sock {
    pub token: __u32,
    pub pm: mptcp_pm_data,
}

#[repr(C)]
pub struct mptcp_pm_data {
    pub extra_subflows: i32,
}

#[repr(C)]
pub struct mptcp_subflow_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sock {
    pub sk_mark: __u32,
}

#[repr(C)]
pub struct inet_connection_sock {
    pub icsk_ca_ops: *mut tcp_congestion_ops,
}

#[repr(C)]
pub struct tcp_congestion_ops {
    pub name: [u8; TCP_CA_NAME_MAX],
}

unsafe extern "C" {
    pub fn bpf_skc_to_mptcp_sock(sk: *mut bpf_sock) -> *mut mptcp_sock;
    pub fn bpf_map_lookup_elem(map: *mut mptcp_sf_map, key: *const __u32) -> *mut __u32;
    pub fn bpf_map_update_elem(
        map: *mut mptcp_sf_map,
        key: *const __u32,
        value: *const __u32,
        flags: u64,
    ) -> i64;
    pub fn bpf_setsockopt(
        skops: *mut bpf_sock_ops,
        level: i32,
        optname: i32,
        optval: *const core::ffi::c_void,
        optlen: i32,
    ) -> i32;
    pub fn bpf_get_current_pid_tgid() -> u64;
    pub fn mptcp_subflow_tcp_sock(subflow: *mut mptcp_subflow_context) -> *mut sock;
}

/* External iterator equivalent of mptcp_for_each_subflow(msk, subflow). */
unsafe extern "C" {
    pub fn mptcp_for_each_subflow_next(
        msk: *mut mptcp_sock,
        cursor: *mut *mut mptcp_subflow_context,
    ) -> bool;
}

#[inline(always)]
unsafe fn bpf_core_cast<T, U>(ptr: *mut T) -> *mut U {
    ptr as *mut U
}

#[no_mangle]
#[link_section = "sockops"]
pub unsafe extern "C" fn mptcp_subflow(skops: *mut bpf_sock_ops) -> i32 {
    let init: __u32 = 1;
    let mut key: __u32;
    let mut mark: __u32;
    let cnt: *mut __u32;
    let msk: *mut mptcp_sock;
    let sk: *mut bpf_sock;
    let mut err: i32;

    if (*skops).op != BPF_SOCK_OPS_TCP_CONNECT_CB {
        return 1;
    }

    sk = (*skops).sk;
    if sk.is_null() {
        return 1;
    }

    msk = bpf_skc_to_mptcp_sock(sk);
    if msk.is_null() {
        return 1;
    }

    key = (*msk).token;
    cnt = bpf_map_lookup_elem(&raw mut mptcp_sf, &raw const key);
    if !cnt.is_null() {
        /* A new subflow is added to an existing MPTCP connection */
        (*(cnt as *mut core::sync::atomic::AtomicU32))
            .fetch_add(1, core::sync::atomic::Ordering::SeqCst);
        mark = *cnt;
    } else {
        /* A new MPTCP connection is just initiated and this is its primary subflow */
        bpf_map_update_elem(&raw mut mptcp_sf, &raw const key, &raw const init, BPF_ANY);
        mark = init;
    }

    /* Set the mark of the subflow's socket based on appearance order */
    err = bpf_setsockopt(
        skops,
        SOL_SOCKET,
        SO_MARK,
        &raw const mark as *const core::ffi::c_void,
        core::mem::size_of_val(&mark) as i32,
    );
    if err < 0 {
        return 1;
    }
    if mark == 2 {
        err = bpf_setsockopt(
            skops,
            SOL_TCP,
            TCP_CONGESTION,
            (&raw const cc) as *const core::ffi::c_void,
            TCP_CA_NAME_MAX as i32,
        );
    }

    return 1;
}

unsafe fn _check_getsockopt_subflow_mark(msk: *mut mptcp_sock, ctx: *mut bpf_sockopt) -> i32 {
    let mut subflow: *mut mptcp_subflow_context = core::ptr::null_mut();
    let mut i: i32 = 0;

    while mptcp_for_each_subflow_next(msk, &mut subflow) {
        let ssk: *mut sock;

        ssk = mptcp_subflow_tcp_sock(bpf_core_cast::<
            mptcp_subflow_context,
            mptcp_subflow_context,
        >(subflow));

        i += 1;
        if (*ssk).sk_mark != i as __u32 {
            (*ctx).retval = -2;
            break;
        }
    }

    return 1;
}

unsafe fn _check_getsockopt_subflow_cc(msk: *mut mptcp_sock, ctx: *mut bpf_sockopt) -> i32 {
    let mut subflow: *mut mptcp_subflow_context = core::ptr::null_mut();

    while mptcp_for_each_subflow_next(msk, &mut subflow) {
        let icsk: *mut inet_connection_sock;
        let ssk: *mut sock;

        ssk = mptcp_subflow_tcp_sock(bpf_core_cast::<
            mptcp_subflow_context,
            mptcp_subflow_context,
        >(subflow));
        icsk = bpf_core_cast::<sock, inet_connection_sock>(ssk);

        if (*ssk).sk_mark == 2
            && core::slice::from_raw_parts(
                core::ptr::addr_of!((*(*icsk).icsk_ca_ops).name).cast::<u8>(),
                TCP_CA_NAME_MAX,
            ) != &cc[..]
        {
            (*ctx).retval = -2;
            break;
        }
    }

    return 1;
}

#[no_mangle]
#[link_section = "cgroup/getsockopt"]
pub unsafe extern "C" fn _getsockopt_subflow(ctx: *mut bpf_sockopt) -> i32 {
    let sk: *mut bpf_sock = (*ctx).sk;
    let msk: *mut mptcp_sock;

    if (bpf_get_current_pid_tgid() >> 32) != pid as u64 {
        return 1;
    }

    if sk.is_null()
        || (*sk).protocol != IPPROTO_MPTCP
        || (!((*ctx).level == SOL_SOCKET && (*ctx).optname == SO_MARK)
            && !((*ctx).level == SOL_TCP && (*ctx).optname == TCP_CONGESTION))
    {
        return 1;
    }

    msk = bpf_core_cast::<bpf_sock, mptcp_sock>(sk);
    if (*msk).pm.extra_subflows != 1 {
        (*ctx).retval = -1;
        return 1;
    }

    if (*ctx).optname == SO_MARK {
        return _check_getsockopt_subflow_mark(msk, ctx);
    }
    return _check_getsockopt_subflow_cc(msk, ctx);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
