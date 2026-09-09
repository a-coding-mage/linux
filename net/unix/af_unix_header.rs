/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by linux/uidgid.h.

pub const UNIX_HASH_MOD: u32 = 256 - 1;
pub const UNIX_HASH_SIZE: u32 = 256 * 2;
pub const UNIX_HASH_BITS: u32 = 8;

extern "C" {
    pub fn unix_peer_get(sk: *mut sock) -> *mut sock;
}

#[repr(C)]
pub struct unix_skb_parms {
    pub pid: *mut pid, // skb credentials
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub fp: *mut scm_fp_list, // Passed files
    #[cfg(CONFIG_SECURITY_NETWORK)]
    pub secid: u32, // Security ID
    pub consumed: u32,
}

// Equivalent to: (*(struct unix_skb_parms *)&((skb)->cb))
#[macro_export]
macro_rules! UNIXCB {
    ($skb:expr) => {
        unsafe { &mut *((&mut ($skb).cb as *mut _) as *mut $crate::unix_skb_parms) }
    };
}

/* GC for SCM_RIGHTS */
extern "C" {
    pub fn unix_add_edges(fpl: *mut scm_fp_list, receiver: *mut unix_sock);
    pub fn unix_del_edges(fpl: *mut scm_fp_list);
    pub fn unix_update_edges(receiver: *mut unix_sock);
    pub fn unix_prepare_fpl(fpl: *mut scm_fp_list) -> ::core::ffi::c_int;
    pub fn unix_destroy_fpl(fpl: *mut scm_fp_list);
    pub fn unix_peek_fpl(fpl: *mut scm_fp_list);
    pub fn unix_schedule_gc(user: *mut user_struct);
}

/* SOCK_DIAG */
extern "C" {
    pub fn unix_inq_len(sk: *mut sock) -> ::core::ffi::c_long;
    pub fn unix_outq_len(sk: *mut sock) -> ::core::ffi::c_long;
}

/* sysctl */
#[cfg(CONFIG_SYSCTL)]
extern "C" {
    pub fn unix_sysctl_register(net: *mut net) -> ::core::ffi::c_int;
    pub fn unix_sysctl_unregister(net: *mut net);
}

#[cfg(not(CONFIG_SYSCTL))]
#[inline]
pub unsafe fn unix_sysctl_register(_net: *mut net) -> ::core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_SYSCTL))]
#[inline]
pub unsafe fn unix_sysctl_unregister(_net: *mut net) {}

/* BPF SOCKMAP */
extern "C" {
    pub fn __unix_dgram_recvmsg(
        sk: *mut sock,
        msg: *mut msghdr,
        size: usize,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn __unix_stream_recvmsg(
        sk: *mut sock,
        msg: *mut msghdr,
        size: usize,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

#[cfg(CONFIG_BPF_SYSCALL)]
extern "C" {
    pub static mut unix_dgram_proto: proto;
    pub static mut unix_stream_proto: proto;

    pub fn unix_dgram_bpf_update_proto(
        sk: *mut sock,
        psock: *mut sk_psock,
        restore: bool,
    ) -> ::core::ffi::c_int;
    pub fn unix_stream_bpf_update_proto(
        sk: *mut sock,
        psock: *mut sk_psock,
        restore: bool,
    ) -> ::core::ffi::c_int;
    // __init
    pub fn unix_bpf_build_proto();
}

#[cfg(not(CONFIG_BPF_SYSCALL))]
#[inline]
pub unsafe fn unix_bpf_build_proto() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
