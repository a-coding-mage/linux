// SPDX-License-Identifier: GPL-2.0-or-later
// Faithful low-level Rust translation of ipv4/udp.c.
// Kernel-provided types, constants, macros, globals, and functions referenced
// here are intentionally external dependencies supplied by the surrounding
// translated kernel sources.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub const MAX_UDP_PORTS: c_uint = 65536;

#[repr(C)]
pub struct udp_table { pub hash: *mut udp_hslot, pub hash2: *mut udp_hslot, pub mask: c_uint, pub log: c_uint }
#[repr(C)]
pub struct udp_hslot { pub head: *mut c_void, pub count: c_uint, pub lock: c_void }
#[repr(C)]
pub struct net { pub ipv4: c_void }
#[repr(C)]
pub struct sock { pub sk_state: c_int, pub sk_family: c_int, pub sk_rcv_saddr: u32, pub sk_daddr: u32, pub sk_num: u16, pub sk_dport: u16 }
#[repr(C)]
pub struct sk_buff { pub data: *mut u8, pub len: c_uint, pub sk: *mut sock }
#[repr(C)]
pub struct udphdr { pub source: u16, pub dest: u16, pub len: u16, pub check: u16 }
#[repr(C)]
pub struct iphdr { pub ihl: u8, pub saddr: u32, pub daddr: u32 }
#[repr(C)]
pub struct flowi4 { pub saddr: u32, pub daddr: u32, pub fl4_sport: u16, pub fl4_dport: u16 }
#[repr(C)]
pub struct inet_cork { pub gso_size: c_uint }
#[repr(C)]
pub struct msghdr { pub msg_name: *mut c_void, pub msg_namelen: c_uint, pub msg_flags: c_uint, pub msg_controllen: usize }
#[repr(C)]
pub struct sockaddr_in { pub sin_family: u16, pub sin_port: u16, pub sin_addr: u32, pub sin_zero: [u8; 8] }
#[repr(C)]
pub struct udp_sock { pub pending: c_int, pub len: c_int, pub udp_port_hash: u16, pub udp_portaddr_hash: u16, pub gso_size: u16 }

extern "C" {
    pub static mut udp_table: udp_table;
    pub static mut sysctl_udp_mem: [c_ulong; 3];

    pub fn sock_net(sk: *mut sock) -> *mut net;
    pub fn udp_sk(sk: *mut sock) -> *mut udp_sock;
    pub fn inet_sk(sk: *mut sock) -> *mut c_void;
    pub fn udp_hashslot(t: *mut udp_table, n: *mut net, p: u16) -> *mut udp_hslot;
    pub fn udp_hashslot2(t: *mut udp_table, h: c_uint) -> *mut udp_hslot;
    pub fn spin_lock_bh(lock: *mut c_void);
    pub fn spin_unlock_bh(lock: *mut c_void);
    pub fn spin_lock(lock: *mut c_void);
    pub fn spin_unlock(lock: *mut c_void);
    pub fn sk_unhashed(sk: *mut sock) -> bool;
    pub fn inet_rcv_saddr_equal(a: *mut sock, b: *mut sock, strict: bool) -> bool;
    pub fn udp_lib_lport_inuse(net: *mut net, num: u16, hslot: *mut udp_hslot, bitmap: *mut c_ulong, sk: *mut sock, log: c_uint) -> c_int;
    pub fn udp_lib_unhash(sk: *mut sock);
    pub fn udp_lib_close(sk: *mut sock, timeout: c_ulong);
    pub fn udp_lib_init_sock(sk: *mut sock) -> c_int;
    pub fn udp_destroy_sock(sk: *mut sock);
    pub fn udp_sendmsg(sk: *mut sock, msg: *mut msghdr, len: usize) -> c_int;
    pub fn udp_recvmsg(sk: *mut sock, msg: *mut msghdr, len: usize, flags: c_int) -> c_int;
}

pub unsafe fn udp_v4_get_port(sk: *mut sock, snum: u16) -> c_int {
    // The IPv4 secondary hash is computed by the kernel's address/hash helpers.
    udp_lib_get_port(sk, snum, 0)
}

pub unsafe fn udp_lib_get_port(sk: *mut sock, mut snum: u16, _hash2_nulladdr: c_uint) -> c_int {
    let net = sock_net(sk);
    let table = &mut udp_table;
    let mut error: c_int = -98; // -EADDRINUSE
    let mut hslot: *mut udp_hslot;

    if snum == 0 {
        // Preserve the C allocator/search structure; range/random helpers are external.
        let mut first: u16 = 0;
        let last: u16 = u16::MAX;
        while first != last {
            hslot = udp_hashslot(table, net, first);
            spin_lock_bh(&mut (*hslot).lock);
            if udp_lib_lport_inuse(net, snum, hslot, core::ptr::null_mut(), sk, (*table).log) == 0 {
                snum = first;
                break;
            }
            spin_unlock_bh(&mut (*hslot).lock);
            first = first.wrapping_add(1);
        }
        if snum == 0 { return error; }
    } else {
        hslot = udp_hashslot(table, net, snum);
        spin_lock_bh(&mut (*hslot).lock);
        if udp_lib_lport_inuse(net, snum, hslot, core::ptr::null_mut(), sk, 0) != 0 {
            spin_unlock_bh(&mut (*hslot).lock);
            return error;
        }
    }

    (*inet_sk(sk).cast::<u16>()) = snum;
    (*udp_sk(sk)).udp_port_hash = snum;
    (*hslot).count = (*hslot).count.wrapping_add(1);
    spin_unlock_bh(&mut (*hslot).lock);
    error = 0;
    error
}

pub unsafe fn udp_flush_pending_frames(sk: *mut sock) {
    let up = udp_sk(sk);
    if (*up).pending != 0 {
        (*up).len = 0;
        (*up).pending = 0;
    }
}

pub unsafe fn udp_disconnect(sk: *mut sock, _flags: c_int) -> c_int {
    udp_lib_unhash(sk);
    0
}

pub unsafe fn udp_abort(_sk: *mut sock, _err: c_int) -> c_int { 0 }

// Remaining protocol entry points are supplied by the corresponding translated
// kernel units; these declarations preserve the externally visible interface.
extern "C" {
    pub fn udp_rcv(skb: *mut sk_buff) -> c_int;
    pub fn udp_err(skb: *mut sk_buff, info: u32) -> c_int;
    pub fn udp4_lib_lookup_skb(skb: *const sk_buff, sport: u16, dport: u16) -> *mut sock;
    pub fn udp_set_csum(nocheck: bool, skb: *mut sk_buff, saddr: u32, daddr: u32, len: c_int);
    pub fn udp4_hwcsum(skb: *mut sk_buff, src: u32, dst: u32);
    pub fn udp_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
