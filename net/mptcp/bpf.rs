// SPDX-License-Identifier: GPL-2.0
/* Multipath TCP
 *
 * Copyright (c) 2020, Tessares SA.
 * Copyright (c) 2022, SUSE.
 *
 * Author: Nicolas Rybowski <nicolas.rybowski@tessares.net>
 */

// C dependency: <linux/bpf.h> and "protocol.h".

use core::ptr;

use crate::protocol::{mptcp_sk, mptcp_subflow_ctx, sk_fullsock, sk_is_mptcp, sk_is_tcp};

extern "C" {
    pub type sock;
    pub type mptcp_sock;

    pub fn register_btf_fmodret_id_set(set: *const btf_kfunc_id_set) -> i32;
    pub static THIS_MODULE: *mut core::ffi::c_void;
}

#[repr(C)]
pub struct btf_kfunc_id_set {
    pub owner: *mut core::ffi::c_void,
    pub set: *const btf_id_set8,
}

#[repr(C)]
pub struct btf_id_set8 {
    pub ids: *const core::ffi::c_void,
}

/// C: `bpf_mptcp_sock_from_subflow`.
pub unsafe extern "C" fn bpf_mptcp_sock_from_subflow(sk: *mut sock) -> *mut mptcp_sock {
    if !sk.is_null() && sk_fullsock(sk) && sk_is_tcp(sk) && sk_is_mptcp(sk) {
        return mptcp_sk((*mptcp_subflow_ctx(sk)).conn);
    }

    ptr::null_mut()
}

// BTF_SET8_START(bpf_mptcp_fmodret_ids)
// BTF_ID_FLAGS(func, update_socket_protocol)
// BTF_SET8_END(bpf_mptcp_fmodret_ids)
// The BTF set contents are supplied by the kernel BTF macro definitions.
extern "C" {
    static bpf_mptcp_fmodret_ids: btf_id_set8;
}

static bpf_mptcp_fmodret_set: btf_kfunc_id_set = btf_kfunc_id_set {
    owner: unsafe { THIS_MODULE },
    set: unsafe { &bpf_mptcp_fmodret_ids },
};

unsafe extern "C" fn bpf_mptcp_kfunc_init() -> i32 {
    register_btf_fmodret_id_set(&bpf_mptcp_fmodret_set)
}

// C registration directive: late_initcall(bpf_mptcp_kfunc_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
