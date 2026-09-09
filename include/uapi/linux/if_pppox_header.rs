/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* Linux PPP over X - Generic PPP transport layer sockets. */

/* C headers are dependencies supplied by the surrounding UAPI translation. */

#[cfg(not(feature = "kernel"))]
pub const AF_PPPOX: i32 = 24;
#[cfg(not(feature = "kernel"))]
pub const PF_PPPOX: i32 = AF_PPPOX;

pub type sid_t = __be16;

#[repr(C)]
pub struct pppoe_addr {
    pub sid: sid_t,
    pub remote: [u8; ETH_ALEN],
    pub dev: [::core::ffi::c_char; IFNAMSIZ],
}

#[repr(C)]
pub struct pptp_addr {
    pub call_id: __u16,
    pub sin_addr: in_addr,
}

pub const PX_PROTO_OE: u32 = 0;
pub const PX_PROTO_OL2TP: u32 = 1;
pub const PX_PROTO_PPTP: u32 = 2;
pub const PX_MAX_PROTO: u32 = 3;

#[repr(C)]
pub union sockaddr_pppox_sa_addr {
    pub pppoe: pppoe_addr,
    pub pptp: pptp_addr,
}

#[repr(C, packed)]
pub struct sockaddr_pppox {
    pub sa_family: __kernel_sa_family_t,
    pub sa_protocol: ::core::ffi::c_uint,
    pub sa_addr: sockaddr_pppox_sa_addr,
}

#[repr(C, packed)]
pub struct sockaddr_pppol2tp {
    pub sa_family: __kernel_sa_family_t,
    pub sa_protocol: ::core::ffi::c_uint,
    pub pppol2tp: pppol2tp_addr,
}

#[repr(C, packed)]
pub struct sockaddr_pppol2tpin6 {
    pub sa_family: __kernel_sa_family_t,
    pub sa_protocol: ::core::ffi::c_uint,
    pub pppol2tp: pppol2tpin6_addr,
}

#[repr(C, packed)]
pub struct sockaddr_pppol2tpv3 {
    pub sa_family: __kernel_sa_family_t,
    pub sa_protocol: ::core::ffi::c_uint,
    pub pppol2tp: pppol2tpv3_addr,
}

#[repr(C, packed)]
pub struct sockaddr_pppol2tpv3in6 {
    pub sa_family: __kernel_sa_family_t,
    pub sa_protocol: ::core::ffi::c_uint,
    pub pppol2tp: pppol2tpv3in6_addr,
}

pub const PADI_CODE: u8 = 0x09;
pub const PADO_CODE: u8 = 0x07;
pub const PADR_CODE: u8 = 0x19;
pub const PADS_CODE: u8 = 0x65;
pub const PADT_CODE: u8 = 0xa7;

#[repr(C, packed)]
pub struct pppoe_tag {
    pub tag_type: __be16,
    pub tag_len: __be16,
    #[cfg(not(feature = "kernel"))]
    pub tag_data: [::core::ffi::c_char; 0],
}

pub const PTT_EOL: __be16 = 0x0000u16.to_be();
pub const PTT_SRV_NAME: __be16 = 0x0101u16.to_be();
pub const PTT_AC_NAME: __be16 = 0x0102u16.to_be();
pub const PTT_HOST_UNIQ: __be16 = 0x0103u16.to_be();
pub const PTT_AC_COOKIE: __be16 = 0x0104u16.to_be();
pub const PTT_VENDOR: __be16 = 0x0105u16.to_be();
pub const PTT_RELAY_SID: __be16 = 0x0110u16.to_be();
pub const PTT_SRV_ERR: __be16 = 0x0201u16.to_be();
pub const PTT_SYS_ERR: __be16 = 0x0202u16.to_be();
pub const PTT_GEN_ERR: __be16 = 0x0203u16.to_be();

#[repr(C, packed)]
pub struct pppoe_hdr {
    /* C bitfields: little endian has type in bits 0..4 and ver in 4..8;
     * big endian has ver in bits 0..4 and type in 4..8. */
    pub ver_type: __u8,
    pub code: __u8,
    pub sid: __be16,
    pub length: __be16,
    #[cfg(not(feature = "kernel"))]
    pub tag: [pppoe_tag; 0],
}

pub const PPPOE_SES_HLEN: usize = 8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
