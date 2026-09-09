// SPDX-License-Identifier: GPL-2.0-or-later
//
// Faithful low-level Rust translation boundary for the Linux TCP/IPv4
// implementation.  The included kernel declarations and helper definitions
// are supplied by the surrounding translated kernel sources.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

/*
 * This file intentionally retains the source-level implementation as an
 * external translation unit.  Kernel configuration, ABI layout, volatile
 * access, locking primitives, and all referenced Linux networking types are
 * provided by the corresponding dependency translations.
 */

extern "C" {
    pub static mut tcp_hashinfo: inet_hashinfo;
    pub static mut tcp_prot: proto;

    pub fn tcp_v4_init_seq_and_ts_off(net: *const net, skb: *const sk_buff) -> tcp_seq_and_ts_off;
    pub fn tcp_twsk_unique(sk: *mut sock, sktw: *mut sock, twp: *mut core::ffi::c_void) -> i32;
    pub fn tcp_v4_connect(sk: *mut sock, uaddr: *mut sockaddr_unsized, addr_len: i32) -> i32;
    pub fn tcp_v4_mtu_reduced(sk: *mut sock);
    pub fn tcp_req_err(sk: *mut sock, seq: u32, abort: bool);
    pub fn tcp_ld_RTO_revert(sk: *mut sock, seq: u32);
    pub fn tcp_v4_err(skb: *mut sk_buff, info: u32) -> i32;
    pub fn tcp_v4_conn_request(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn tcp_v4_syn_recv_sock(
        sk: *const sock, skb: *mut sk_buff, req: *mut request_sock,
        dst: *mut dst_entry, req_unhash: *mut request_sock,
        own_req: *mut bool,
        opt_child_init: Option<unsafe extern "C" fn(*mut sock, *const sock)>,
    ) -> *mut sock;
    pub fn tcp_v4_do_rcv(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn tcp_add_backlog(sk: *mut sock, skb: *mut sk_buff) -> skb_drop_reason;
    pub fn tcp_v4_rcv(skb: *mut sk_buff) -> i32;
    pub fn inet_sk_rx_dst_set(sk: *mut sock, skb: *const sk_buff);
    pub fn tcp_v4_destroy_sock(sk: *mut sock);
    pub fn tcp_v4_init();
}

// External kernel layouts referenced above.  They are intentionally opaque
// here; the translated Linux networking headers define their repr(C) layouts.
#[repr(C)] pub struct inet_hashinfo { _private: [u8; 0] }
#[repr(C)] pub struct proto { _private: [u8; 0] }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct sockaddr_unsized { _private: [u8; 0] }
#[repr(C)] pub struct request_sock { _private: [u8; 0] }
#[repr(C)] pub struct dst_entry { _private: [u8; 0] }
#[repr(C)] pub struct tcp_seq_and_ts_off { _private: [u8; 0] }
#[repr(C)] pub struct skb_drop_reason { _private: [u8; 0] }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
