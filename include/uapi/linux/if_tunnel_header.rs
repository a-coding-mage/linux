/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
// Translated from linux/if_tunnel.h. Required Linux type and interface
// definitions are supplied by the surrounding bindings.

pub const SIOCGETTUNNEL: u32 = SIOCDEVPRIVATE + 0;
pub const SIOCADDTUNNEL: u32 = SIOCDEVPRIVATE + 1;
pub const SIOCDELTUNNEL: u32 = SIOCDEVPRIVATE + 2;
pub const SIOCCHGTUNNEL: u32 = SIOCDEVPRIVATE + 3;
pub const SIOCGETPRL: u32 = SIOCDEVPRIVATE + 4;
pub const SIOCADDPRL: u32 = SIOCDEVPRIVATE + 5;
pub const SIOCDELPRL: u32 = SIOCDEVPRIVATE + 6;
pub const SIOCCHGPRL: u32 = SIOCDEVPRIVATE + 7;
pub const SIOCGET6RD: u32 = SIOCDEVPRIVATE + 8;
pub const SIOCADD6RD: u32 = SIOCDEVPRIVATE + 9;
pub const SIOCDEL6RD: u32 = SIOCDEVPRIVATE + 10;
pub const SIOCCHG6RD: u32 = SIOCDEVPRIVATE + 11;

pub const GRE_CSUM: u16 = 0x8000u16.to_be();
pub const GRE_ROUTING: u16 = 0x4000u16.to_be();
pub const GRE_KEY: u16 = 0x2000u16.to_be();
pub const GRE_SEQ: u16 = 0x1000u16.to_be();
pub const GRE_STRICT: u16 = 0x0800u16.to_be();
pub const GRE_REC: u16 = 0x0700u16.to_be();
pub const GRE_ACK: u16 = 0x0080u16.to_be();
pub const GRE_FLAGS: u16 = 0x0078u16.to_be();
pub const GRE_VERSION: u16 = 0x0007u16.to_be();

pub const fn GRE_IS_CSUM(f: u16) -> u16 { f & GRE_CSUM }
pub const fn GRE_IS_ROUTING(f: u16) -> u16 { f & GRE_ROUTING }
pub const fn GRE_IS_KEY(f: u16) -> u16 { f & GRE_KEY }
pub const fn GRE_IS_SEQ(f: u16) -> u16 { f & GRE_SEQ }
pub const fn GRE_IS_STRICT(f: u16) -> u16 { f & GRE_STRICT }
pub const fn GRE_IS_REC(f: u16) -> u16 { f & GRE_REC }
pub const fn GRE_IS_ACK(f: u16) -> u16 { f & GRE_ACK }

pub const GRE_VERSION_0: u16 = 0x0000u16.to_be();
pub const GRE_VERSION_1: u16 = 0x0001u16.to_be();
pub const GRE_PROTO_PPP: u16 = 0x880bu16.to_be();
pub const GRE_PPTP_KEY_MASK: u32 = 0xffffu32.to_be();

#[repr(C)]
pub struct ip_tunnel_parm {
    pub name: [::core::ffi::c_char; IFNAMSIZ],
    pub link: ::core::ffi::c_int,
    pub i_flags: __be16,
    pub o_flags: __be16,
    pub i_key: __be32,
    pub o_key: __be32,
    pub iph: iphdr,
}

pub const IFLA_IPTUN_UNSPEC: u32 = 0;
pub const IFLA_IPTUN_LINK: u32 = 1;
pub const IFLA_IPTUN_LOCAL: u32 = 2;
pub const IFLA_IPTUN_REMOTE: u32 = 3;
pub const IFLA_IPTUN_TTL: u32 = 4;
pub const IFLA_IPTUN_TOS: u32 = 5;
pub const IFLA_IPTUN_ENCAP_LIMIT: u32 = 6;
pub const IFLA_IPTUN_FLOWINFO: u32 = 7;
pub const IFLA_IPTUN_FLAGS: u32 = 8;
pub const IFLA_IPTUN_PROTO: u32 = 9;
pub const IFLA_IPTUN_PMTUDISC: u32 = 10;
pub const IFLA_IPTUN_6RD_PREFIX: u32 = 11;
pub const IFLA_IPTUN_6RD_RELAY_PREFIX: u32 = 12;
pub const IFLA_IPTUN_6RD_PREFIXLEN: u32 = 13;
pub const IFLA_IPTUN_6RD_RELAY_PREFIXLEN: u32 = 14;
pub const IFLA_IPTUN_ENCAP_TYPE: u32 = 15;
pub const IFLA_IPTUN_ENCAP_FLAGS: u32 = 16;
pub const IFLA_IPTUN_ENCAP_SPORT: u32 = 17;
pub const IFLA_IPTUN_ENCAP_DPORT: u32 = 18;
pub const IFLA_IPTUN_COLLECT_METADATA: u32 = 19;
pub const IFLA_IPTUN_FWMARK: u32 = 20;
pub const __IFLA_IPTUN_MAX: u32 = 21;
pub const IFLA_IPTUN_MAX: u32 = __IFLA_IPTUN_MAX - 1;

pub const TUNNEL_ENCAP_NONE: u32 = 0;
pub const TUNNEL_ENCAP_FOU: u32 = 1;
pub const TUNNEL_ENCAP_GUE: u32 = 2;
pub const TUNNEL_ENCAP_MPLS: u32 = 3;
pub const TUNNEL_ENCAP_FLAG_CSUM: u32 = 1 << 0;
pub const TUNNEL_ENCAP_FLAG_CSUM6: u32 = 1 << 1;
pub const TUNNEL_ENCAP_FLAG_REMCSUM: u32 = 1 << 2;
pub const SIT_ISATAP: u32 = 0x0001;

#[repr(C)]
pub struct ip_tunnel_prl {
    pub addr: __be32,
    pub flags: __u16,
    pub __reserved: __u16,
    pub datalen: __u32,
    pub __reserved2: __u32,
    // data follows
}
pub const PRL_DEFAULT: u32 = 0x0001;

#[repr(C)]
pub struct ip_tunnel_6rd {
    pub prefix: in6_addr,
    pub relay_prefix: __be32,
    pub prefixlen: __u16,
    pub relay_prefixlen: __u16,
}

pub const IFLA_GRE_UNSPEC: u32 = 0;
pub const IFLA_GRE_LINK: u32 = 1;
pub const IFLA_GRE_IFLAGS: u32 = 2;
pub const IFLA_GRE_OFLAGS: u32 = 3;
pub const IFLA_GRE_IKEY: u32 = 4;
pub const IFLA_GRE_OKEY: u32 = 5;
pub const IFLA_GRE_LOCAL: u32 = 6;
pub const IFLA_GRE_REMOTE: u32 = 7;
pub const IFLA_GRE_TTL: u32 = 8;
pub const IFLA_GRE_TOS: u32 = 9;
pub const IFLA_GRE_PMTUDISC: u32 = 10;
pub const IFLA_GRE_ENCAP_LIMIT: u32 = 11;
pub const IFLA_GRE_FLOWINFO: u32 = 12;
pub const IFLA_GRE_FLAGS: u32 = 13;
pub const IFLA_GRE_ENCAP_TYPE: u32 = 14;
pub const IFLA_GRE_ENCAP_FLAGS: u32 = 15;
pub const IFLA_GRE_ENCAP_SPORT: u32 = 16;
pub const IFLA_GRE_ENCAP_DPORT: u32 = 17;
pub const IFLA_GRE_COLLECT_METADATA: u32 = 18;
pub const IFLA_GRE_IGNORE_DF: u32 = 19;
pub const IFLA_GRE_FWMARK: u32 = 20;
pub const IFLA_GRE_ERSPAN_INDEX: u32 = 21;
pub const IFLA_GRE_ERSPAN_VER: u32 = 22;
pub const IFLA_GRE_ERSPAN_DIR: u32 = 23;
pub const IFLA_GRE_ERSPAN_HWID: u32 = 24;
pub const __IFLA_GRE_MAX: u32 = 25;
pub const IFLA_GRE_MAX: u32 = __IFLA_GRE_MAX - 1;

pub const VTI_ISVTI: __be16 = 0x0001;
pub const IFLA_VTI_UNSPEC: u32 = 0;
pub const IFLA_VTI_LINK: u32 = 1;
pub const IFLA_VTI_IKEY: u32 = 2;
pub const IFLA_VTI_OKEY: u32 = 3;
pub const IFLA_VTI_LOCAL: u32 = 4;
pub const IFLA_VTI_REMOTE: u32 = 5;
pub const IFLA_VTI_FWMARK: u32 = 6;
pub const __IFLA_VTI_MAX: u32 = 7;
pub const IFLA_VTI_MAX: u32 = __IFLA_VTI_MAX - 1;

// The following definitions are userspace-only in the C header (__KERNEL__ absent).
pub const TUNNEL_CSUM: u16 = 0x01u16.to_be();
pub const TUNNEL_ROUTING: u16 = 0x02u16.to_be();
pub const TUNNEL_KEY: u16 = 0x04u16.to_be();
pub const TUNNEL_SEQ: u16 = 0x08u16.to_be();
pub const TUNNEL_STRICT: u16 = 0x10u16.to_be();
pub const TUNNEL_REC: u16 = 0x20u16.to_be();
pub const TUNNEL_VERSION: u16 = 0x40u16.to_be();
pub const TUNNEL_NO_KEY: u16 = 0x80u16.to_be();
pub const TUNNEL_DONT_FRAGMENT: u16 = 0x0100u16.to_be();
pub const TUNNEL_OAM: u16 = 0x0200u16.to_be();
pub const TUNNEL_CRIT_OPT: u16 = 0x0400u16.to_be();
pub const TUNNEL_GENEVE_OPT: u16 = 0x0800u16.to_be();
pub const TUNNEL_VXLAN_OPT: u16 = 0x1000u16.to_be();
pub const TUNNEL_NOCACHE: u16 = 0x2000u16.to_be();
pub const TUNNEL_ERSPAN_OPT: u16 = 0x4000u16.to_be();
pub const TUNNEL_GTP_OPT: u16 = 0x8000u16.to_be();
pub const TUNNEL_OPTIONS_PRESENT: u16 = TUNNEL_GENEVE_OPT | TUNNEL_VXLAN_OPT | TUNNEL_ERSPAN_OPT | TUNNEL_GTP_OPT;

pub const IP_TUNNEL_CSUM_BIT: u32 = 0;
pub const IP_TUNNEL_ROUTING_BIT: u32 = 1;
pub const IP_TUNNEL_KEY_BIT: u32 = 2;
pub const IP_TUNNEL_SEQ_BIT: u32 = 3;
pub const IP_TUNNEL_STRICT_BIT: u32 = 4;
pub const IP_TUNNEL_REC_BIT: u32 = 5;
pub const IP_TUNNEL_VERSION_BIT: u32 = 6;
pub const IP_TUNNEL_NO_KEY_BIT: u32 = 7;
pub const IP_TUNNEL_DONT_FRAGMENT_BIT: u32 = 8;
pub const IP_TUNNEL_OAM_BIT: u32 = 9;
pub const IP_TUNNEL_CRIT_OPT_BIT: u32 = 10;
pub const IP_TUNNEL_GENEVE_OPT_BIT: u32 = 11;
pub const IP_TUNNEL_VXLAN_OPT_BIT: u32 = 12;
pub const IP_TUNNEL_NOCACHE_BIT: u32 = 13;
pub const IP_TUNNEL_ERSPAN_OPT_BIT: u32 = 14;
pub const IP_TUNNEL_GTP_OPT_BIT: u32 = 15;
pub const IP_TUNNEL_VTI_BIT: u32 = 16;
pub const IP_TUNNEL_SIT_ISATAP_BIT: u32 = IP_TUNNEL_VTI_BIT;
// Flags starting from here are not available via the old UAPI.
pub const IP_TUNNEL_PFCP_OPT_BIT: u32 = 17;
pub const __IP_TUNNEL_FLAG_NUM: u32 = 18;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
