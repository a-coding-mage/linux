/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* Definitions for the IP protocol. */

// Dependencies supplied by the surrounding Linux Rust bindings:
// linux::types, linux::stddef, and asm::byteorder.

pub const IPTOS_TOS_MASK: u32 = 0x1E;
pub const IPTOS_TOS: fn(u32) -> u32 = |tos| tos & IPTOS_TOS_MASK;
pub const IPTOS_LOWDELAY: u32 = 0x10;
pub const IPTOS_THROUGHPUT: u32 = 0x08;
pub const IPTOS_RELIABILITY: u32 = 0x04;
pub const IPTOS_MINCOST: u32 = 0x02;

pub const IPTOS_PREC_MASK: u32 = 0xE0;
pub const IPTOS_PREC: fn(u32) -> u32 = |tos| tos & IPTOS_PREC_MASK;
pub const IPTOS_PREC_NETCONTROL: u32 = 0xe0;
pub const IPTOS_PREC_INTERNETCONTROL: u32 = 0xc0;
pub const IPTOS_PREC_CRITIC_ECP: u32 = 0xa0;
pub const IPTOS_PREC_FLASHOVERRIDE: u32 = 0x80;
pub const IPTOS_PREC_FLASH: u32 = 0x60;
pub const IPTOS_PREC_IMMEDIATE: u32 = 0x40;
pub const IPTOS_PREC_PRIORITY: u32 = 0x20;
pub const IPTOS_PREC_ROUTINE: u32 = 0x00;

/* IP options */
pub const IPOPT_COPY: u32 = 0x80;
pub const IPOPT_CLASS_MASK: u32 = 0x60;
pub const IPOPT_NUMBER_MASK: u32 = 0x1f;
pub const IPOPT_COPIED: fn(u32) -> u32 = |o| o & IPOPT_COPY;
pub const IPOPT_CLASS: fn(u32) -> u32 = |o| o & IPOPT_CLASS_MASK;
pub const IPOPT_NUMBER: fn(u32) -> u32 = |o| o & IPOPT_NUMBER_MASK;
pub const IPOPT_CONTROL: u32 = 0x00;
pub const IPOPT_RESERVED1: u32 = 0x20;
pub const IPOPT_MEASUREMENT: u32 = 0x40;
pub const IPOPT_RESERVED2: u32 = 0x60;
pub const IPOPT_END: u32 = 0 | IPOPT_CONTROL;
pub const IPOPT_NOOP: u32 = 1 | IPOPT_CONTROL;
pub const IPOPT_SEC: u32 = 2 | IPOPT_CONTROL | IPOPT_COPY;
pub const IPOPT_LSRR: u32 = 3 | IPOPT_CONTROL | IPOPT_COPY;
pub const IPOPT_TIMESTAMP: u32 = 4 | IPOPT_MEASUREMENT;
pub const IPOPT_CIPSO: u32 = 6 | IPOPT_CONTROL | IPOPT_COPY;
pub const IPOPT_RR: u32 = 7 | IPOPT_CONTROL;
pub const IPOPT_SID: u32 = 8 | IPOPT_CONTROL | IPOPT_COPY;
pub const IPOPT_SSRR: u32 = 9 | IPOPT_CONTROL | IPOPT_COPY;
pub const IPOPT_RA: u32 = 20 | IPOPT_CONTROL | IPOPT_COPY;
pub const IPVERSION: u32 = 4;
pub const MAXTTL: u32 = 255;
pub const IPDEFTTL: u32 = 64;
pub const IPOPT_OPTVAL: u32 = 0;
pub const IPOPT_OLEN: u32 = 1;
pub const IPOPT_OFFSET: u32 = 2;
pub const IPOPT_MINOFF: u32 = 4;
pub const MAX_IPOPTLEN: u32 = 40;
pub const IPOPT_NOP: u32 = IPOPT_NOOP;
pub const IPOPT_EOL: u32 = IPOPT_END;
pub const IPOPT_TS: u32 = IPOPT_TIMESTAMP;
pub const IPOPT_TS_TSONLY: u32 = 0; /* timestamps only */
pub const IPOPT_TS_TSANDADDR: u32 = 1; /* timestamps and addresses */
pub const IPOPT_TS_PRESPEC: u32 = 3; /* specified modules only */
pub const IPV4_BEET_PHMAXLEN: u32 = 8;

#[repr(C)]
pub struct iphdr {
    // C bitfields: little endian is ihl:4, version:4; big endian is reversed.
    pub ihl_version: __u8,
    pub tos: __u8,
    pub tot_len: __be16,
    pub id: __be16,
    pub frag_off: __be16,
    pub ttl: __u8,
    pub protocol: __u8,
    pub check: __sum16,
    pub saddr: __be32,
    pub daddr: __be32,
    /* The options start here. */
}

#[repr(C)]
pub struct ip_auth_hdr {
    pub nexthdr: __u8,
    pub hdrlen: __u8, /* This one is measured in 32 bit units! */
    pub reserved: __be16,
    pub spi: __be32,
    pub seq_no: __be32, /* Sequence number */
    pub auth_data: [__u8; 0], /* Variable len but >=4. Mind the 64 bit alignment! */
}

#[repr(C)]
pub struct ip_esp_hdr {
    pub spi: __be32,
    pub seq_no: __be32, /* Sequence number */
    pub enc_data: [__u8; 0], /* Variable len but >=8. Mind the 64 bit alignment! */
}

#[repr(C)]
pub struct ip_comp_hdr { pub nexthdr: __u8, pub flags: __u8, pub cpi: __be16 }

#[repr(C)]
pub struct ip_beet_phdr { pub nexthdr: __u8, pub hdrlen: __u8, pub padlen: __u8, pub reserved: __u8 }

#[repr(C)]
pub struct ip_iptfs_hdr {
    pub subtype: __u8, /* 0*: basic, 1: CC */
    pub flags: __u8,
    pub block_offset: __be16,
}

#[repr(C)]
pub struct ip_iptfs_cc_hdr {
    pub subtype: __u8, /* 0: basic, 1*: CC */
    pub flags: __u8,
    pub block_offset: __be16,
    pub loss_rate: __be32,
    pub rtt_adelay_xdelay: __be64,
    pub tval: __be32,
    pub techo: __be32,
}

/* index values for the variables in ipv4_devconf */
#[repr(i32)]
pub enum ipv4_devconf {
    IPV4_DEVCONF_FORWARDING = 1,
    IPV4_DEVCONF_MC_FORWARDING,
    IPV4_DEVCONF_PROXY_ARP,
    IPV4_DEVCONF_ACCEPT_REDIRECTS,
    IPV4_DEVCONF_SECURE_REDIRECTS,
    IPV4_DEVCONF_SEND_REDIRECTS,
    IPV4_DEVCONF_SHARED_MEDIA,
    IPV4_DEVCONF_RP_FILTER,
    IPV4_DEVCONF_ACCEPT_SOURCE_ROUTE,
    IPV4_DEVCONF_BOOTP_RELAY,
    IPV4_DEVCONF_LOG_MARTIANS,
    IPV4_DEVCONF_TAG,
    IPV4_DEVCONF_ARPFILTER,
    IPV4_DEVCONF_MEDIUM_ID,
    IPV4_DEVCONF_NOXFRM,
    IPV4_DEVCONF_NOPOLICY,
    IPV4_DEVCONF_FORCE_IGMP_VERSION,
    IPV4_DEVCONF_ARP_ANNOUNCE,
    IPV4_DEVCONF_ARP_IGNORE,
    IPV4_DEVCONF_PROMOTE_SECONDARIES,
    IPV4_DEVCONF_ARP_ACCEPT,
    IPV4_DEVCONF_ARP_NOTIFY,
    IPV4_DEVCONF_ACCEPT_LOCAL,
    IPV4_DEVCONF_SRC_VMARK,
    IPV4_DEVCONF_PROXY_ARP_PVLAN,
    IPV4_DEVCONF_ROUTE_LOCALNET,
    IPV4_DEVCONF_IGMPV2_UNSOLICITED_REPORT_INTERVAL,
    IPV4_DEVCONF_IGMPV3_UNSOLICITED_REPORT_INTERVAL,
    IPV4_DEVCONF_IGNORE_ROUTES_WITH_LINKDOWN,
    IPV4_DEVCONF_DROP_UNICAST_IN_L2_MULTICAST,
    IPV4_DEVCONF_DROP_GRATUITOUS_ARP,
    IPV4_DEVCONF_BC_FORWARDING,
    IPV4_DEVCONF_ARP_EVICT_NOCARRIER,
    __IPV4_DEVCONF_MAX,
}

pub const IPV4_DEVCONF_MAX: i32 = __IPV4_DEVCONF_MAX as i32 - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
