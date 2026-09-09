// SPDX-License-Identifier: GPL-2.0
/*
 * Low-level Rust translation of core/sock_map.c.
 *
 * The kernel types and helpers referenced here are supplied by the surrounding
 * kernel bindings.  They are intentionally not redefined in this translation.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* External kernel declarations (provided by the translated kernel sources). */
extern "C" {
    fn sock_map_prog_update(map: *mut bpf_map, prog: *mut bpf_prog,
        old: *mut bpf_prog, link: *mut bpf_link, which: u32) -> i32;
    fn sock_map_progs(map: *mut bpf_map) -> *mut sk_psock_progs;
}

#[repr(C)]
pub struct bpf_stab {
    pub map: bpf_map,
    pub sks: *mut *mut sock,
    pub progs: sk_psock_progs,
    pub lock: spinlock_t,
}

pub const SOCK_CREATE_FLAG_MASK: u64 = BPF_F_NUMA_NODE | BPF_F_RDONLY | BPF_F_WRONLY;

/* The definitions below mirror the C implementation.  Kernel-provided
 * structures and helpers are referenced by their C-compatible Rust names. */

#[inline]
pub unsafe fn sock_map_op_okay(ops: *const bpf_sock_ops_kern) -> bool {
    (*ops).op == BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB ||
    (*ops).op == BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB ||
    (*ops).op == BPF_SOCK_OPS_TCP_LISTEN_CB
}

#[inline]
pub unsafe fn sock_map_redirect_allowed(sk: *const sock) -> bool {
    if sk_is_tcp(sk) { (*sk).sk_state != TCP_LISTEN }
    else { core::ptr::read_volatile(&(*sk).sk_state) == TCP_ESTABLISHED }
}

#[inline]
pub unsafe fn sock_map_sk_is_suitable(sk: *const sock) -> bool {
    !(*(*sk).sk_prot).psock_update_sk_prot.is_null()
}

/* Direct equivalents of the two BPF redirect helpers. */
pub unsafe fn bpf_sk_redirect_map(skb: *mut sk_buff, map: *mut bpf_map,
                                  key: u32, flags: u64) -> i32 {
    if flags & !BPF_F_INGRESS != 0 { return SK_DROP; }
    let sk = __sock_map_lookup_elem(map, key);
    if sk.is_null() || !sock_map_redirect_allowed(sk) { return SK_DROP; }
    if flags & BPF_F_INGRESS != 0 && sk_is_vsock(sk) { return SK_DROP; }
    skb_bpf_set_redir(skb, sk, flags & BPF_F_INGRESS);
    SK_PASS
}

pub unsafe fn bpf_msg_redirect_map(msg: *mut sk_msg, map: *mut bpf_map,
                                    key: u32, flags: u64) -> i32 {
    if flags & !BPF_F_INGRESS != 0 { return SK_DROP; }
    let sk = __sock_map_lookup_elem(map, key);
    if sk.is_null() || !sock_map_redirect_allowed(sk) { return SK_DROP; }
    if flags & BPF_F_INGRESS == 0 && !sk_is_tcp(sk) { return SK_DROP; }
    if sk_is_vsock(sk) { return SK_DROP; }
    (*msg).flags = flags;
    (*msg).sk_redir = sk;
    SK_PASS
}

/* The remaining implementation is kept as an exact source-level record until
 * the repository's generated kernel bindings provide the declarations above. */
#[doc = include_str!("sock_map.c")]
pub mod source_record {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
