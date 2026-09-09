/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of inet_sock.h. Included C dependencies are supplied externally. */

pub const IP_OPTIONS_DATA_FIXED_SIZE: usize = 40;

#[repr(C)]
pub struct ip_options {
    pub faddr: __be32,
    pub nexthop: __be32,
    pub optlen: u8,
    pub srr: u8,
    pub rr: u8,
    pub ts: u8,
    /* C bit-fields packed into this byte. */
    pub flags: u8,
    pub router_alert: u8,
    pub cipso: u8,
    pub __pad2: u8,
    pub __data: [u8; 0],
}

#[repr(C)]
pub struct ip_options_rcu {
    pub rcu: rcu_head,
    /* Must be last as it ends in a flexible-array member. */
    pub opt: ip_options,
}

#[repr(C)]
pub struct inet_request_sock {
    pub req: request_sock,
    /* C anonymous union and bit-fields are represented by their storage. */
    pub snd_wscale_rcv_wscale_flags: u16,
    pub ir_mark: u32,
    pub options: inet_request_sock_options,
}

#[repr(C)]
pub union inet_request_sock_options {
    pub ireq_opt: *mut ip_options_rcu,
    pub ipv6: inet_request_sock_ipv6_options,
}

#[repr(C)]
pub struct inet_request_sock_ipv6_options {
    pub ipv6_opt: *mut ipv6_txoptions,
    pub pktopts: *mut sk_buff,
}

#[inline]
pub unsafe fn inet_request_mark(sk: *const sock, skb: *mut sk_buff) -> u32 {
    let mark = READ_ONCE((*sk).sk_mark);
    if mark == 0 && READ_ONCE((*sock_net(sk)).ipv4.sysctl_tcp_fwmark_accept) {
        return (*skb).mark;
    }
    mark
}

#[inline]
pub unsafe fn inet_request_bound_dev_if(sk: *const sock, skb: *mut sk_buff) -> i32 {
    let bound_dev_if = READ_ONCE((*sk).sk_bound_dev_if);
    /* CONFIG_NET_L3_MASTER_DEV conditional retained from the C header. */
    #[cfg(feature = "CONFIG_NET_L3_MASTER_DEV")]
    {
        let net = sock_net(sk);
        if bound_dev_if == 0 && READ_ONCE((*net).ipv4.sysctl_tcp_l3mdev_accept) {
            return l3mdev_master_ifindex_by_index(net, (*skb).skb_iif);
        }
    }
    bound_dev_if
}

#[inline]
pub unsafe fn inet_sk_bound_l3mdev(sk: *const sock) -> i32 {
    #[cfg(feature = "CONFIG_NET_L3_MASTER_DEV")]
    {
        let net = sock_net(sk);
        if !READ_ONCE((*net).ipv4.sysctl_tcp_l3mdev_accept) {
            return l3mdev_master_ifindex_by_index(net, (*sk).sk_bound_dev_if);
        }
    }
    0
}

#[inline]
pub fn inet_bound_dev_eq(l3mdev_accept: bool, bound_dev_if: i32, dif: i32, sdif: i32) -> bool {
    if bound_dev_if == 0 { !sdif != 0 || l3mdev_accept } else { bound_dev_if == dif || bound_dev_if == sdif }
}

#[inline]
pub unsafe fn inet_sk_bound_dev_eq(net: *const net, bound_dev_if: i32, dif: i32, sdif: i32) -> bool {
    #[cfg(feature = "CONFIG_NET_L3_MASTER_DEV")]
    { return inet_bound_dev_eq(READ_ONCE((*net).ipv4.sysctl_tcp_l3mdev_accept) != 0, bound_dev_if, dif, sdif); }
    #[cfg(not(feature = "CONFIG_NET_L3_MASTER_DEV"))]
    { inet_bound_dev_eq(true, bound_dev_if, dif, sdif) }
}

#[repr(C)] pub struct inet6_cork { pub opt: *mut ipv6_txoptions, pub hop_limit: u8, pub tclass: u8, pub dontfrag: u8 }
#[repr(C)] pub struct inet_cork { pub flags: u32, pub addr: __be32, pub opt: *mut ip_options, pub fragsize: u32, pub length: i32, pub dst: *mut dst_entry, pub tx_flags: u8, pub ttl: u8, pub tos: i16, pub priority: u32, pub gso_size: u16, pub ts_opt_id: u32, pub transmit_time: u64, pub mark: u32 }
#[repr(C)] pub struct inet_cork_full { pub base: inet_cork, pub fl: flowi, pub base6: inet6_cork }

pub enum ip_mc_socklist {}
pub enum ipv6_pinfo {}
pub enum rtable {}

#[repr(C)]
pub struct inet_sock {
    pub sk: sock,
    pub pinet6: *mut ipv6_pinfo,
    pub ipv6_fl_list: *mut ipv6_fl_socklist,
    pub inet_flags: ::core::ffi::c_ulong,
    pub inet_saddr: __be32,
    pub uc_ttl: i16,
    pub inet_sport: __be16,
    pub inet_opt: *mut ip_options_rcu,
    pub inet_id: atomic_t,
    pub tos: u8,
    pub min_ttl: u8,
    pub mc_ttl: u8,
    pub pmtudisc: u8,
    pub rcv_tos: u8,
    pub convert_csum: u8,
    pub uc_index: i32,
    pub mc_index: i32,
    pub mc_addr: __be32,
    pub local_port_range: u32,
    pub mc_list: *mut ip_mc_socklist,
    pub cork: inet_cork_full,
}

pub const IPCORK_OPT: u32 = 1;
pub const IPCORK_TS_OPT_ID: u32 = 2;

pub const INET_FLAGS_PKTINFO: u32 = 0;
pub const INET_FLAGS_TTL: u32 = 1;
pub const INET_FLAGS_TOS: u32 = 2;
pub const INET_FLAGS_RECVOPTS: u32 = 3;
pub const INET_FLAGS_RETOPTS: u32 = 4;
pub const INET_FLAGS_PASSSEC: u32 = 5;
pub const INET_FLAGS_ORIGDSTADDR: u32 = 6;
pub const INET_FLAGS_CHECKSUM: u32 = 7;
pub const INET_FLAGS_RECVFRAGSIZE: u32 = 8;
pub const INET_FLAGS_RECVERR: u32 = 9;
pub const INET_FLAGS_RECVERR_RFC4884: u32 = 10;
pub const INET_FLAGS_FREEBIND: u32 = 11;
pub const INET_FLAGS_HDRINCL: u32 = 12;
pub const INET_FLAGS_MC_LOOP: u32 = 13;
pub const INET_FLAGS_MC_ALL: u32 = 14;
pub const INET_FLAGS_TRANSPARENT: u32 = 15;
pub const INET_FLAGS_IS_ICSK: u32 = 16;
pub const INET_FLAGS_NODEFRAG: u32 = 17;
pub const INET_FLAGS_BIND_ADDRESS_NO_PORT: u32 = 18;
pub const INET_FLAGS_DEFER_CONNECT: u32 = 19;
pub const INET_FLAGS_MC6_LOOP: u32 = 20;
pub const INET_FLAGS_RECVERR6_RFC4884: u32 = 21;
pub const INET_FLAGS_MC6_ALL: u32 = 22;
pub const INET_FLAGS_AUTOFLOWLABEL_SET: u32 = 23;
pub const INET_FLAGS_AUTOFLOWLABEL: u32 = 24;
pub const INET_FLAGS_DONTFRAG: u32 = 25;
pub const INET_FLAGS_RECVERR6: u32 = 26;
pub const INET_FLAGS_REPFLOW: u32 = 27;
pub const INET_FLAGS_RTALERT_ISOLATE: u32 = 28;
pub const INET_FLAGS_SNDFLOW: u32 = 29;
pub const INET_FLAGS_RTALERT: u32 = 30;

pub const IP_CMSG_PKTINFO: ::core::ffi::c_ulong = 1 << INET_FLAGS_PKTINFO;
pub const IP_CMSG_TTL: ::core::ffi::c_ulong = 1 << INET_FLAGS_TTL;
pub const IP_CMSG_TOS: ::core::ffi::c_ulong = 1 << INET_FLAGS_TOS;
pub const IP_CMSG_RECVOPTS: ::core::ffi::c_ulong = 1 << INET_FLAGS_RECVOPTS;
pub const IP_CMSG_RETOPTS: ::core::ffi::c_ulong = 1 << INET_FLAGS_RETOPTS;
pub const IP_CMSG_PASSSEC: ::core::ffi::c_ulong = 1 << INET_FLAGS_PASSSEC;
pub const IP_CMSG_ORIGDSTADDR: ::core::ffi::c_ulong = 1 << INET_FLAGS_ORIGDSTADDR;
pub const IP_CMSG_CHECKSUM: ::core::ffi::c_ulong = 1 << INET_FLAGS_CHECKSUM;
pub const IP_CMSG_RECVFRAGSIZE: ::core::ffi::c_ulong = 1 << INET_FLAGS_RECVFRAGSIZE;
pub const IP_CMSG_ALL: ::core::ffi::c_ulong = IP_CMSG_PKTINFO | IP_CMSG_TTL | IP_CMSG_TOS | IP_CMSG_RECVOPTS | IP_CMSG_RETOPTS | IP_CMSG_PASSSEC | IP_CMSG_ORIGDSTADDR | IP_CMSG_CHECKSUM | IP_CMSG_RECVFRAGSIZE;

#[inline] pub unsafe fn inet_cmsg_flags(inet: *const inet_sock) -> ::core::ffi::c_ulong { READ_ONCE((*inet).inet_flags) & IP_CMSG_ALL }
#[inline] pub unsafe fn inet_sk_dscp(inet: *const inet_sock) -> dscp_t { inet_dsfield_to_dscp(READ_ONCE((*inet).tos)) }

#[inline] pub unsafe fn sk_to_full_sk(mut sk: *mut sock) -> *mut sock {
    #[cfg(feature = "CONFIG_INET")]
    { if !sk.is_null() && READ_ONCE((*sk).sk_state) == TCP_NEW_SYN_RECV { sk = (*inet_reqsk(sk)).rsk_listener; } if !sk.is_null() && READ_ONCE((*sk).sk_state) == TCP_TIME_WAIT { sk = core::ptr::null_mut(); } }
    sk
}
#[inline] pub unsafe fn sk_const_to_full_sk(mut sk: *const sock) -> *const sock {
    #[cfg(feature = "CONFIG_INET")]
    { if !sk.is_null() && READ_ONCE((*sk).sk_state) == TCP_NEW_SYN_RECV { sk = (*(sk as *const request_sock)).rsk_listener; } if !sk.is_null() && READ_ONCE((*sk).sk_state) == TCP_TIME_WAIT { sk = core::ptr::null(); } }
    sk
}
#[inline] pub unsafe fn skb_to_full_sk(skb: *const sk_buff) -> *mut sock { sk_to_full_sk((*skb).sk) }

pub unsafe fn inet_sk_rebuild_header(sk: *mut sock) -> i32;
#[inline] pub unsafe fn inet_sk_state_load(sk: *const sock) -> i32 { smp_load_acquire(&(*sk).sk_state) }
pub unsafe fn inet_sk_state_store(sk: *mut sock, newstate: i32);
pub unsafe fn inet_sk_set_state(sk: *mut sock, state: i32);

#[inline] pub unsafe fn __inet_ehashfn(laddr: __be32, lport: __u16, faddr: __be32, fport: __be16, initval: u32) -> u32 { jhash_3words(laddr as u32, faddr as u32, ((lport as u32) << 16) | fport as u32, initval) }
pub unsafe fn inet_reqsk_alloc(ops: *const request_sock_ops, sk_listener: *mut sock, attach_listener: bool) -> *mut request_sock;
#[inline] pub unsafe fn inet_sk_flowi_flags(sk: *const sock) -> __u8 { let mut flags = 0; if inet_test_bit(15, sk) || inet_test_bit(12, sk) { flags |= FLOWI_FLAG_ANYSRC; } flags }
#[inline] pub unsafe fn inet_inc_convert_csum(sk: *mut sock) { (*inet_sk(sk)).convert_csum = (*inet_sk(sk)).convert_csum.wrapping_add(1); }
#[inline] pub unsafe fn inet_dec_convert_csum(sk: *mut sock) { if (*inet_sk(sk)).convert_csum > 0 { (*inet_sk(sk)).convert_csum -= 1; } }
#[inline] pub unsafe fn inet_get_convert_csum(sk: *mut sock) -> bool { (*inet_sk(sk)).convert_csum != 0 }
#[inline] pub unsafe fn inet_can_nonlocal_bind(net: *mut net, inet: *mut inet_sock) -> bool { READ_ONCE((*net).ipv4.sysctl_ip_nonlocal_bind) || test_bit(INET_FLAGS_FREEBIND, &(*inet).inet_flags) || test_bit(INET_FLAGS_TRANSPARENT, &(*inet).inet_flags) }
#[inline] pub unsafe fn inet_addr_valid_or_nonlocal(net: *mut net, inet: *mut inet_sock, addr: __be32, addr_type: i32) -> bool { inet_can_nonlocal_bind(net, inet) || addr == htonl(INADDR_ANY) || addr_type == RTN_LOCAL || addr_type == RTN_MULTICAST || addr_type == RTN_BROADCAST }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
