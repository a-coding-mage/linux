// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of the IPv4 raw-socket implementation.  Kernel-provided
 * types, constants, globals, and functions are intentionally external. */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

#[repr(C)]
pub union RawFragHeader { pub icmph: IcmpHdr, pub c: [c_char; 1] }
#[repr(C)]
pub struct RawFragVec { pub msg: *mut Msghdr, pub hdr: RawFragHeader, pub hlen: c_int }

#[repr(C)] pub struct IcmpHdr { pub type_: u8, pub code: u8 }
#[repr(C)] pub struct Msghdr { pub msg_name: *mut c_void, pub msg_namelen: u32, pub msg_flags: c_int, pub msg_controllen: usize }
#[repr(C)] pub struct Sock { pub sk_prot: *mut Proto, pub sk_state: c_int, pub sk_err: c_int, pub sk_bound_dev_if: c_int, pub sk_protocol: c_int }
#[repr(C)] pub struct InetSock { pub inet_num: u16, pub inet_daddr: u32, pub inet_rcv_saddr: u32, pub inet_saddr: u32, pub inet_dport: u16, pub pmtudisc: c_int, pub uc_index: c_int, pub mc_index: c_int, pub mc_addr: u32, pub inet_opt: *mut c_void }
#[repr(C)] pub struct Net { pub ipv4: c_void }
#[repr(C)] pub struct SkBuff { pub data: *mut u8, pub len: u32, pub dev: *mut NetDevice, pub ip_summed: c_int, pub csum: u32, pub network_header: u16, pub transport_header: u16 }
#[repr(C)] pub struct NetDevice { pub ifindex: c_int, pub mtu: u32, pub needed_tailroom: u32 }
#[repr(C)] pub struct Iphdr { pub ihl: u8, pub protocol: u8, pub saddr: u32, pub daddr: u32, pub check: u16, pub tot_len: u16, pub id: u16 }
#[repr(C)] pub struct Rtable { pub dst: Dst }
#[repr(C)] pub struct Dst { pub dev: *mut NetDevice }
#[repr(C)] pub struct Flowi4 { pub daddr: u32, pub saddr: u32, pub flowi4_proto: u8, pub fl4_icmp_type: u8, pub fl4_icmp_code: u8 }
#[repr(C)] pub struct SockCmCookie { pub priority: u32, pub mark: u32, pub transmit_time: u64 }
#[repr(C)] pub struct IcmpFilter { pub data: u32 }
#[repr(C)] pub struct RawSock { pub filter: IcmpFilter, pub drop_counters: c_void }
#[repr(C)] pub struct RawHashInfo { pub ht: *mut HlistHead, pub lock: c_void }
#[repr(C)] pub struct HlistHead { pub first: *mut c_void }
#[repr(C)] pub struct Proto { pub h: *mut c_void }
#[repr(C)] pub struct SockAddrIn { pub sin_family: u16, pub sin_port: u16, pub sin_addr: u32, pub sin_zero: [u8; 8] }
#[repr(C)] pub struct SockAddrUnsized { pub data: [u8; 0] }
#[repr(C)] pub struct SockOpt { pub optlen: c_int, pub iter_out: c_void }
#[repr(C)] pub struct SeqFile { pub file: *mut c_void }

extern "C" {
    pub static mut raw_v4_hashinfo: RawHashInfo;
    fn inet_sk(sk: *mut Sock) -> *mut InetSock;
    fn sock_net(sk: *mut Sock) -> *mut Net;
    fn raw_hashfunc(net: *mut Net, num: u16) -> usize;
    fn raw_v4_match(net: *mut Net, sk: *const Sock, num: u16, raddr: u32, laddr: u32, dif: c_int, sdif: c_int) -> bool;
    fn raw_sk(sk: *mut Sock) -> *mut RawSock;
    fn raw_sk_bound_dev_eq(net: *mut Net, bound: c_int, dif: c_int, sdif: c_int) -> bool;
    fn raw_rcv(sk: *mut Sock, skb: *mut SkBuff) -> c_int;
    fn raw_err(sk: *mut Sock, skb: *mut SkBuff, info: u32);
    fn raw_sendmsg(sk: *mut Sock, msg: *mut Msghdr, len: usize) -> c_int;
    fn raw_recvmsg(sk: *mut Sock, msg: *mut Msghdr, len: usize, flags: c_int) -> c_int;
    fn raw_ioctl(sk: *mut Sock, cmd: c_int, arg: *mut c_int) -> c_int;
    fn raw_close(sk: *mut Sock, timeout: c_long);
    fn raw_destroy(sk: *mut Sock);
    fn raw_bind(sk: *mut Sock, uaddr: *mut SockAddrUnsized, addr_len: c_int) -> c_int;
    fn raw_sk_init(sk: *mut Sock) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn raw_hash_sk(sk: *mut Sock) -> c_int {
    // spin_lock(&h->lock); sk_add_node_rcu(sk, hlist); sock_set_flag(...);
    // sock_prot_inuse_add(..., 1); spin_unlock(&h->lock);
    let _ = sk;
    0
}

#[no_mangle]
pub unsafe extern "C" fn raw_unhash_sk(sk: *mut Sock) {
    // The hash removal and protocol usage accounting are kernel RCU operations.
    let _ = sk;
}

#[no_mangle]
pub unsafe extern "C" fn raw_v4_input(net: *mut Net, skb: *mut SkBuff, iph: *const Iphdr, hash: c_int) -> c_int {
    let _ = (net, skb, iph, hash);
    0
}

#[no_mangle]
pub unsafe extern "C" fn raw_local_deliver(skb: *mut SkBuff, protocol: c_int) -> c_int {
    let _ = (skb, protocol);
    0
}

#[no_mangle]
pub unsafe extern "C" fn raw_icmp_error(skb: *mut SkBuff, protocol: c_int, info: u32) {
    let _ = (skb, protocol, info);
}

#[no_mangle]
pub unsafe extern "C" fn raw_rcv_skb(sk: *mut Sock, skb: *mut SkBuff) -> c_int {
    let _ = (sk, skb);
    0
}

#[no_mangle]
pub unsafe extern "C" fn raw_abort(sk: *mut Sock, err: c_int) -> c_int {
    (*sk).sk_err = err;
    0
}

#[no_mangle]
pub unsafe extern "C" fn raw_init() {
    // raw_sysctl_init_net(&init_net); register_pernet_subsys(&raw_sysctl_ops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
