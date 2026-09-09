/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * INET  An implementation of the TCP/IP protocol suite for the LINUX
 *       operating system.  INET is implemented using the BSD Socket
 *       interface as the means of communication with the user level.
 *
 *       Definitions for the RAW-IP module.
 *
 * Version: @(#)raw.h  1.0.2  05/07/93
 *
 * Author: Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented from the original C includes.

extern "C" {
    static mut raw_prot: proto;

    static mut raw_v4_hashinfo: raw_hashinfo;
    fn raw_v4_match(
        net: *mut net,
        sk: *const sock,
        num: u16,
        raddr: __be32,
        laddr: __be32,
        dif: i32,
        sdif: i32,
    ) -> bool;

    fn raw_abort(sk: *mut sock, err: i32) -> i32;
    fn raw_icmp_error(skb: *mut sk_buff, type_: i32, data: u32);
    fn raw_local_deliver(skb: *mut sk_buff, protocol: i32) -> i32;
    fn raw_rcv(sk: *mut sock, skb: *mut sk_buff) -> i32;
}

pub const RAW_HTABLE_LOG: u32 = 8;
pub const RAW_HTABLE_SIZE: usize = 1usize << RAW_HTABLE_LOG;

#[repr(C)]
pub struct raw_hashinfo {
    pub lock: spinlock_t,
    pub ht: [hlist_head; RAW_HTABLE_SIZE], // ____cacheline_aligned
}

#[inline]
pub unsafe fn raw_hashfunc(net: *const net, proto: u32) -> u32 {
    hash_32(net_hash_mix(net) ^ proto, RAW_HTABLE_LOG)
}

#[inline]
pub unsafe fn raw_hashinfo_init(hashinfo: *mut raw_hashinfo) {
    spin_lock_init(core::ptr::addr_of_mut!((*hashinfo).lock));
    for i in 0..RAW_HTABLE_SIZE {
        INIT_HLIST_HEAD(core::ptr::addr_of_mut!((*hashinfo).ht[i]));
    }
}

#[cfg(CONFIG_PROC_FS)]
extern "C" {
    fn raw_proc_init() -> i32;
    fn raw_proc_exit();
}

#[cfg(CONFIG_PROC_FS)]
#[repr(C)]
pub struct raw_iter_state {
    pub p: seq_net_private,
    pub bucket: i32,
}

#[cfg(CONFIG_PROC_FS)]
#[inline]
pub unsafe fn raw_seq_private(seq: *mut seq_file) -> *mut raw_iter_state {
    (*seq).private as *mut raw_iter_state
}

#[cfg(CONFIG_PROC_FS)]
extern "C" {
    fn raw_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void;
    fn raw_seq_next(
        seq: *mut seq_file,
        v: *mut core::ffi::c_void,
        pos: *mut loff_t,
    ) -> *mut core::ffi::c_void;
    fn raw_seq_stop(seq: *mut seq_file, v: *mut core::ffi::c_void);
}

extern "C" {
    fn raw_hash_sk(sk: *mut sock) -> i32;
    fn raw_unhash_sk(sk: *mut sock);
    fn raw_init();
}

#[repr(C)]
pub struct raw_sock {
    // inet_sock has to be the first member
    pub inet: inet_sock,
    pub filter: icmp_filter,
    pub ipmr_table: u32,
    pub drop_counters: numa_drop_counters,
}

// Direct translation of: container_of_const(ptr, struct raw_sock, inet.sk).
macro_rules! raw_sk {
    ($ptr:expr) => {{
        container_of_const!($ptr, raw_sock, inet.sk)
    }};
}

#[inline]
pub unsafe fn raw_sk_bound_dev_eq(
    net: *mut net,
    bound_dev_if: i32,
    dif: i32,
    sdif: i32,
) -> bool {
    #[cfg(CONFIG_NET_L3_MASTER_DEV)]
    {
        inet_bound_dev_eq(
            READ_ONCE!((*net).ipv4.sysctl_raw_l3mdev_accept),
            bound_dev_if,
            dif,
            sdif,
        )
    }
    #[cfg(not(CONFIG_NET_L3_MASTER_DEV))]
    {
        inet_bound_dev_eq(true, bound_dev_if, dif, sdif)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
