/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * INET	An implementation of the TCP/IP protocol suite for the LINUX
 * operating system.  INET is implemented using the BSD Socket
 * interface as the means of communication with the user level.
 *
 * Definitions for the "ping" module.
 */

// C dependencies supplied by the surrounding kernel translation.

/* PING_HTABLE_SIZE must be power of 2 */
pub const PING_HTABLE_SIZE: u32 = 64;
pub const PING_HTABLE_MASK: u32 = PING_HTABLE_SIZE - 1;

pub const GID_T_MAX: gid_t = ((!0u32) as gid_t) - 1;

/* Compatibility glue so we can support IPv6 when it's compiled as a module */
#[repr(C)]
pub struct pingv6_ops {
    pub ipv6_recv_error: Option<unsafe extern "C" fn(sk: *mut sock, msg: *mut msghdr, len: i32) -> i32>,
    pub ip6_datagram_recv_common_ctl: Option<unsafe extern "C" fn(sk: *mut sock, msg: *mut msghdr, skb: *mut sk_buff)>,
    pub ip6_datagram_recv_specific_ctl: Option<unsafe extern "C" fn(sk: *mut sock, msg: *mut msghdr, skb: *mut sk_buff)>,
    pub icmpv6_err_convert: Option<unsafe extern "C" fn(type_: u8, code: u8, err: *mut i32) -> i32>,
    pub ipv6_icmp_error: Option<unsafe extern "C" fn(sk: *mut sock, skb: *mut sk_buff, err: i32, port: __be16, info: u32, payload: *mut u8)>,
    pub ipv6_chk_addr: Option<unsafe extern "C" fn(net: *mut net, addr: *const in6_addr, dev: *const net_device, strict: i32) -> i32>,
}

#[repr(C)]
pub struct ping_iter_state {
    pub p: seq_net_private,
    pub bucket: i32,
    pub family: sa_family_t,
}

extern "C" {
    pub static mut ping_prot: proto;
    // Preserved build-time condition: CONFIG_IPV6 enables this declaration.
    #[cfg(CONFIG_IPV6)]
    pub static mut pingv6_ops: pingv6_ops;
}

#[repr(C)]
pub struct pingfakehdr {
    pub icmph: icmphdr,
    pub msg: *mut msghdr,
    pub family: sa_family_t,
    pub wcheck: __wsum,
}

extern "C" {
    pub fn ping_get_port(sk: *mut sock, ident: u16) -> i32;
    pub fn ping_unhash(sk: *mut sock);

    pub fn ping_init_sock(sk: *mut sock) -> i32;
    pub fn ping_close(sk: *mut sock, timeout: i64);
    pub fn ping_bind(sk: *mut sock, uaddr: *mut sockaddr_unsized, addr_len: i32) -> i32;
    pub fn ping_err(skb: *mut sk_buff, offset: i32, info: u32);
    pub fn ping_getfrag(from: *mut core::ffi::c_void, to: *mut i8, offset: i32, fraglen: i32, odd: i32, skb: *mut sk_buff) -> i32;

    pub fn ping_recvmsg(sk: *mut sock, msg: *mut msghdr, len: usize, flags: i32) -> i32;
    pub fn ping_common_sendmsg(family: i32, msg: *mut msghdr, len: usize, user_icmph: *mut core::ffi::c_void, icmph_len: usize) -> i32;
    pub fn ping_queue_rcv_skb(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn ping_rcv(skb: *mut sk_buff) -> skb_drop_reason;

    // Preserved build-time condition: CONFIG_PROC_FS enables these declarations.
    #[cfg(CONFIG_PROC_FS)]
    pub fn ping_seq_start(seq: *mut seq_file, pos: *mut loff_t, family: sa_family_t) -> *mut core::ffi::c_void;
    #[cfg(CONFIG_PROC_FS)]
    pub fn ping_seq_next(seq: *mut seq_file, v: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void;
    #[cfg(CONFIG_PROC_FS)]
    pub fn ping_seq_stop(seq: *mut seq_file, v: *mut core::ffi::c_void);
    #[cfg(CONFIG_PROC_FS)]
    pub fn ping_proc_init() -> i32;
    #[cfg(CONFIG_PROC_FS)]
    pub fn ping_proc_exit();

    pub fn ping_init();
    pub fn pingv6_init() -> i32;
    pub fn pingv6_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
