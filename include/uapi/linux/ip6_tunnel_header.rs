/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from the C UAPI header. The included Linux type definitions and
// constants are supplied by the surrounding translation environment.

pub const IPV6_TLV_TNL_ENCAP_LIMIT: u32 = 4;
pub const IPV6_DEFAULT_TNL_ENCAP_LIMIT: u32 = 4;

/* don't add encapsulation limit if one isn't present in inner packet */
pub const IP6_TNL_F_IGN_ENCAP_LIMIT: u32 = 0x1;
/* copy the traffic class field from the inner packet */
pub const IP6_TNL_F_USE_ORIG_TCLASS: u32 = 0x2;
/* copy the flowlabel from the inner packet */
pub const IP6_TNL_F_USE_ORIG_FLOWLABEL: u32 = 0x4;
/* being used for Mobile IPv6 */
pub const IP6_TNL_F_MIP6_DEV: u32 = 0x8;
/* copy DSCP from the outer packet */
pub const IP6_TNL_F_RCV_DSCP_COPY: u32 = 0x10;
/* copy fwmark from inner packet */
pub const IP6_TNL_F_USE_ORIG_FWMARK: u32 = 0x20;
/* allow remote endpoint on the local node */
pub const IP6_TNL_F_ALLOW_LOCAL_REMOTE: u32 = 0x40;

#[repr(C)]
pub struct ip6_tnl_parm {
    pub name: [::core::ffi::c_char; IFNAMSIZ], /* name of tunnel device */
    pub link: ::core::ffi::c_int, /* ifindex of underlying L2 interface */
    pub proto: __u8, /* tunnel protocol */
    pub encap_limit: __u8, /* encapsulation limit for tunnel */
    pub hop_limit: __u8, /* hop limit for tunnel */
    pub flowinfo: __be32, /* traffic class and flowlabel for tunnel */
    pub flags: __u32, /* tunnel flags */
    pub laddr: in6_addr, /* local tunnel end-point address */
    pub raddr: in6_addr, /* remote tunnel end-point address */
}

#[repr(C)]
pub struct ip6_tnl_parm2 {
    pub name: [::core::ffi::c_char; IFNAMSIZ], /* name of tunnel device */
    pub link: ::core::ffi::c_int, /* ifindex of underlying L2 interface */
    pub proto: __u8, /* tunnel protocol */
    pub encap_limit: __u8, /* encapsulation limit for tunnel */
    pub hop_limit: __u8, /* hop limit for tunnel */
    pub flowinfo: __be32, /* traffic class and flowlabel for tunnel */
    pub flags: __u32, /* tunnel flags */
    pub laddr: in6_addr, /* local tunnel end-point address */
    pub raddr: in6_addr, /* remote tunnel end-point address */

    pub i_flags: __be16,
    pub o_flags: __be16,
    pub i_key: __be32,
    pub o_key: __be32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
