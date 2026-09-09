/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * INET An implementation of the TCP/IP protocol suite for the LINUX
 * operating system. Definitions for the ICMP protocol.
 */

pub const ICMP_ECHOREPLY: i32 = 0; /* Echo Reply */
pub const ICMP_DEST_UNREACH: i32 = 3; /* Destination Unreachable */
pub const ICMP_SOURCE_QUENCH: i32 = 4; /* Source Quench */
pub const ICMP_REDIRECT: i32 = 5; /* Redirect (change route) */
pub const ICMP_ECHO: i32 = 8; /* Echo Request */
pub const ICMP_TIME_EXCEEDED: i32 = 11; /* Time Exceeded */
pub const ICMP_PARAMETERPROB: i32 = 12; /* Parameter Problem */
pub const ICMP_TIMESTAMP: i32 = 13; /* Timestamp Request */
pub const ICMP_TIMESTAMPREPLY: i32 = 14; /* Timestamp Reply */
pub const ICMP_INFO_REQUEST: i32 = 15; /* Information Request */
pub const ICMP_INFO_REPLY: i32 = 16; /* Information Reply */
pub const ICMP_ADDRESS: i32 = 17; /* Address Mask Request */
pub const ICMP_ADDRESSREPLY: i32 = 18; /* Address Mask Reply */
pub const NR_ICMP_TYPES: i32 = 18;

/* Codes for UNREACH. */
pub const ICMP_NET_UNREACH: i32 = 0;
pub const ICMP_HOST_UNREACH: i32 = 1;
pub const ICMP_PROT_UNREACH: i32 = 2;
pub const ICMP_PORT_UNREACH: i32 = 3;
pub const ICMP_FRAG_NEEDED: i32 = 4;
pub const ICMP_SR_FAILED: i32 = 5;
pub const ICMP_NET_UNKNOWN: i32 = 6;
pub const ICMP_HOST_UNKNOWN: i32 = 7;
pub const ICMP_HOST_ISOLATED: i32 = 8;
pub const ICMP_NET_ANO: i32 = 9;
pub const ICMP_HOST_ANO: i32 = 10;
pub const ICMP_NET_UNR_TOS: i32 = 11;
pub const ICMP_HOST_UNR_TOS: i32 = 12;
pub const ICMP_PKT_FILTERED: i32 = 13;
pub const ICMP_PREC_VIOLATION: i32 = 14;
pub const ICMP_PREC_CUTOFF: i32 = 15;
pub const NR_ICMP_UNREACH: i32 = 15;

/* Codes for REDIRECT. */
pub const ICMP_REDIR_NET: i32 = 0;
pub const ICMP_REDIR_HOST: i32 = 1;
pub const ICMP_REDIR_NETTOS: i32 = 2;
pub const ICMP_REDIR_HOSTTOS: i32 = 3;

/* Codes for TIME_EXCEEDED. */
pub const ICMP_EXC_TTL: i32 = 0;
pub const ICMP_EXC_FRAGTIME: i32 = 1;

/* Codes for EXT_ECHO (PROBE). */
pub const ICMP_EXT_ECHO: i32 = 42;
pub const ICMP_EXT_ECHOREPLY: i32 = 43;
pub const ICMP_EXT_CODE_MAL_QUERY: i32 = 1;
pub const ICMP_EXT_CODE_NO_IF: i32 = 2;
pub const ICMP_EXT_CODE_NO_TABLE_ENT: i32 = 3;
pub const ICMP_EXT_CODE_MULT_IFS: i32 = 4;

/* Constants for EXT_ECHO (PROBE). */
pub const ICMP_EXT_ECHOREPLY_ACTIVE: i32 = 1 << 2;
pub const ICMP_EXT_ECHOREPLY_IPV4: i32 = 1 << 1;
pub const ICMP_EXT_ECHOREPLY_IPV6: i32 = 1;
pub const ICMP_EXT_ECHO_CTYPE_NAME: i32 = 1;
pub const ICMP_EXT_ECHO_CTYPE_INDEX: i32 = 2;
pub const ICMP_EXT_ECHO_CTYPE_ADDR: i32 = 3;
pub const ICMP_AFI_IP: i32 = 1;
pub const ICMP_AFI_IP6: i32 = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmphdr_echo {
    pub id: __be16,
    pub sequence: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmphdr_frag {
    pub __unused: __be16,
    pub mtu: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union icmphdr_un {
    pub echo: icmphdr_echo,
    pub gateway: __be32,
    pub frag: icmphdr_frag,
    pub reserved: [__u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmphdr {
    pub type_: __u8,
    pub code: __u8,
    pub checksum: __sum16,
    pub un: icmphdr_un,
}

/* Constants for (set|get)sockopt. */
pub const ICMP_FILTER: i32 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmp_filter {
    pub data: __u32,
}

/* RFC 4884 extension struct: one per message. The bitfield occupies one byte;
 * its bit ordering follows __LITTLE_ENDIAN_BITFIELD or __BIG_ENDIAN_BITFIELD.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmp_ext_hdr {
    pub reserved1_version: __u8,
    pub reserved2: __u8,
    pub checksum: __sum16,
}

/* RFC 4884 extension object header: one for each object. */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmp_extobj_hdr {
    pub length: __be16,
    pub class_num: __u8,
    pub class_type: __u8,
}

/* RFC 8335: 2.1 Header for c-type 3 payload. */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmp_ext_echo_ctype3_hdr {
    pub afi: __be16,
    pub addrlen: __u8,
    pub reserved: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmp_ext_echo_iio_addr {
    pub ctype3_hdr: icmp_ext_echo_ctype3_hdr,
    pub ip_addr: icmp_ext_echo_iio_ip_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union icmp_ext_echo_iio_ip_addr {
    pub ipv4_addr: __be32,
    pub ipv6_addr: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union icmp_ext_echo_iio_ident {
    pub name: [core::ffi::c_char; IFNAMSIZ],
    pub ifindex: __be32,
    pub addr: icmp_ext_echo_iio_addr,
}

/* RFC 8335: 2.1 Interface Identification Object. */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmp_ext_echo_iio {
    pub extobj_hdr: icmp_extobj_hdr,
    pub ident: icmp_ext_echo_iio_ident,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
