/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Translated from the Linux UAPI header xt_HMARK.h. */
/* External dependency: nf_inet_addr is supplied by linux/netfilter.h. */

#[repr(i32)]
pub enum XtHmark {
    XT_HMARK_SADDR_MASK = 0,
    XT_HMARK_DADDR_MASK,
    XT_HMARK_SPI,
    XT_HMARK_SPI_MASK,
    XT_HMARK_SPORT,
    XT_HMARK_DPORT,
    XT_HMARK_SPORT_MASK,
    XT_HMARK_DPORT_MASK,
    XT_HMARK_PROTO_MASK,
    XT_HMARK_RND,
    XT_HMARK_MODULUS,
    XT_HMARK_OFFSET,
    XT_HMARK_CT,
    XT_HMARK_METHOD_L3,
    XT_HMARK_METHOD_L3_4,
}

#[inline]
pub const fn XT_HMARK_FLAG(flag: i32) -> i32 {
    1i32.wrapping_shl(flag as u32)
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union HmarkPorts {
    pub p16: HmarkPortsP16,
    pub b16: HmarkPortsB16,
    pub v32: u32,
    pub b32: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HmarkPortsP16 {
    pub src: u16,
    pub dst: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HmarkPortsB16 {
    pub src: u16,
    pub dst: u16,
}

/* External dependency: nf_inet_addr is supplied by linux/netfilter.h. */
#[repr(C)]
pub struct XtHmarkInfo {
    pub src_mask: NfInetAddr,
    pub dst_mask: NfInetAddr,
    pub port_mask: HmarkPorts,
    pub port_set: HmarkPorts,
    pub flags: u32,
    pub proto_mask: u16,
    pub hashrnd: u32,
    pub hmodulus: u32,
    pub hoffset: u32, /* Mark offset to start from */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
