/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header.  Types and functions supplied by the
// surrounding kernel headers are intentionally referenced but not defined here.

use core::mem::size_of;

pub struct inet_hashinfo;

#[repr(C)]
pub struct inet_diag_handler {
    pub owner: *mut module,
    pub dump: Option<unsafe extern "C" fn(
        skb: *mut sk_buff,
        cb: *mut netlink_callback,
        r: *const inet_diag_req_v2,
    )>,
    pub dump_one: Option<unsafe extern "C" fn(
        cb: *mut netlink_callback,
        req: *const inet_diag_req_v2,
    ) -> core::ffi::c_int>,
    pub idiag_get_info: Option<unsafe extern "C" fn(
        sk: *mut sock,
        r: *mut inet_diag_msg,
        info: *mut core::ffi::c_void,
    )>,
    pub idiag_get_aux: Option<unsafe extern "C" fn(
        sk: *mut sock,
        net_admin: bool,
        skb: *mut sk_buff,
    ) -> core::ffi::c_int>,
    pub destroy: Option<unsafe extern "C" fn(
        in_skb: *mut sk_buff,
        req: *const inet_diag_req_v2,
    ) -> core::ffi::c_int>,
    pub idiag_type: __u16,
    pub idiag_info_size: __u16,
}

pub struct bpf_sk_storage_diag;

#[repr(C)]
pub struct inet_diag_dump_data {
    pub req_nlas: [*mut nlattr; __INET_DIAG_REQ_MAX as usize],
    pub bpf_stg_diag: *mut bpf_sk_storage_diag,
    pub mark_needed: bool, /* INET_DIAG_BC_MARK_COND present. */
    // CONFIG_SOCK_CGROUP_DATA
    pub cgroup_needed: bool, /* INET_DIAG_BC_CGROUP_COND present. */
    pub userlocks_needed: bool, /* INET_DIAG_BC_AUTO present. */
}

// C aliases:
// #define inet_diag_nla_bc req_nlas[INET_DIAG_REQ_BYTECODE]
// #define inet_diag_nla_bpf_stgs req_nlas[INET_DIAG_REQ_SK_BPF_STORAGES]

pub struct inet_connection_sock;

extern "C" {
    pub fn inet_sk_diag_fill(
        sk: *mut sock,
        icsk: *mut inet_connection_sock,
        skb: *mut sk_buff,
        cb: *mut netlink_callback,
        req: *const inet_diag_req_v2,
        nlmsg_flags: u16,
        net_admin: bool,
    ) -> core::ffi::c_int;

    pub fn inet_diag_bc_sk(cb_data: *const inet_diag_dump_data, sk: *mut sock)
        -> core::ffi::c_int;

    pub fn inet_diag_msg_common_fill(r: *mut inet_diag_msg, sk: *mut sock);

    pub fn inet_diag_msg_attrs_fill(
        sk: *mut sock,
        skb: *mut sk_buff,
        r: *mut inet_diag_msg,
        ext: core::ffi::c_int,
        user_ns: *mut user_namespace,
        net_admin: bool,
    ) -> core::ffi::c_int;

    pub fn inet_diag_register(handler: *const inet_diag_handler) -> core::ffi::c_int;
    pub fn inet_diag_unregister(handler: *const inet_diag_handler);
}

#[inline]
pub unsafe fn inet_diag_msg_attrs_size() -> usize {
    nla_total_size(1)
        + nla_total_size(1)
        // IS_ENABLED(CONFIG_IPV6)
        + nla_total_size(1)
        + nla_total_size(1)
        + nla_total_size(4)
        + nla_total_size(4)
        // CONFIG_SOCK_CGROUP_DATA
        + nla_total_size_64bit(size_of::<u64>())
        + nla_total_size(size_of::<inet_diag_sockopt>())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
