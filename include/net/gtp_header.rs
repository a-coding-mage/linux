/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The Linux header dependencies are supplied by the surrounding translation
 * unit.  Their names are intentionally retained here rather than redefined.
 */

/* General GTP protocol related definitions. */

pub const GTP0_PORT: u16 = 3386;
pub const GTP1U_PORT: u16 = 2152;

/* GTP messages types */
pub const GTP_ECHO_REQ: u8 = 1; /* Echo Request */
pub const GTP_ECHO_RSP: u8 = 2; /* Echo Response */
pub const GTP_TPDU: u8 = 255;

pub const GTPIE_RECOVERY: u8 = 14;

#[repr(C, packed)]
pub struct gtp0_header {
    pub flags: u8,
    pub type_: u8,
    pub length: u16,
    pub seq: u16,
    pub flow: u16,
    pub number: u8,
    pub spare: [u8; 3],
    pub tid: u64,
}

#[repr(C, packed)]
pub struct gtp1_header {
    pub flags: u8,
    pub type_: u8,
    pub length: u16,
    pub tid: u32,
}

#[repr(C, packed)]
pub struct gtp1_header_long {
    pub flags: u8,
    pub type_: u8,
    pub length: u16,
    pub tid: u32,
    pub seq: u16,
    pub npdu: u8,
    pub next: u8,
}

/* GTP Information Element */
#[repr(C, packed)]
pub struct gtp_ie {
    pub tag: u8,
    pub val: u8,
}

#[repr(C, packed)]
pub struct gtp0_packet {
    pub gtp0_h: gtp0_header,
    pub ie: gtp_ie,
}

#[repr(C, packed)]
pub struct gtp1u_packet {
    pub gtp1u_h: gtp1_header_long,
    pub ie: gtp_ie,
}

#[repr(C)]
pub struct gtp_pdu_session_info {
    pub pdu_type: u8,
    pub qfi: u8,
}

/* The net_device and rtnl_link_ops definitions, and strcmp, are external
 * dependencies supplied by the surrounding translation unit. */
extern "C" {
    fn strcmp(a: *const core::ffi::c_char, b: *const core::ffi::c_char) -> i32;
}

pub unsafe fn netif_is_gtp(dev: *const net_device) -> bool {
    !(*dev).rtnl_link_ops.is_null()
        && strcmp(
            (*(*dev).rtnl_link_ops).kind,
            b"gtp\0".as_ptr() as *const core::ffi::c_char,
        ) == 0
}

pub const GTP1_F_NPDU: u8 = 0x01;
pub const GTP1_F_SEQ: u8 = 0x02;
pub const GTP1_F_EXTHDR: u8 = 0x04;
pub const GTP1_F_MASK: u8 = 0x07;

#[repr(C, packed)]
pub struct gtp_ext_hdr {
    pub len: u8,
    pub data: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
