/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Values for the `mt_flags` field in `struct ip6t_srh`.
pub const IP6T_SRH_NEXTHDR: u16 = 0x0001;
pub const IP6T_SRH_LEN_EQ: u16 = 0x0002;
pub const IP6T_SRH_LEN_GT: u16 = 0x0004;
pub const IP6T_SRH_LEN_LT: u16 = 0x0008;
pub const IP6T_SRH_SEGS_EQ: u16 = 0x0010;
pub const IP6T_SRH_SEGS_GT: u16 = 0x0020;
pub const IP6T_SRH_SEGS_LT: u16 = 0x0040;
pub const IP6T_SRH_LAST_EQ: u16 = 0x0080;
pub const IP6T_SRH_LAST_GT: u16 = 0x0100;
pub const IP6T_SRH_LAST_LT: u16 = 0x0200;
pub const IP6T_SRH_TAG: u16 = 0x0400;
pub const IP6T_SRH_PSID: u16 = 0x0800;
pub const IP6T_SRH_NSID: u16 = 0x1000;
pub const IP6T_SRH_LSID: u16 = 0x2000;
pub const IP6T_SRH_MASK: u16 = 0x3FFF;

// Values for the `mt_invflags` field in `struct ip6t_srh`.
pub const IP6T_SRH_INV_NEXTHDR: u16 = 0x0001;
pub const IP6T_SRH_INV_LEN_EQ: u16 = 0x0002;
pub const IP6T_SRH_INV_LEN_GT: u16 = 0x0004;
pub const IP6T_SRH_INV_LEN_LT: u16 = 0x0008;
pub const IP6T_SRH_INV_SEGS_EQ: u16 = 0x0010;
pub const IP6T_SRH_INV_SEGS_GT: u16 = 0x0020;
pub const IP6T_SRH_INV_SEGS_LT: u16 = 0x0040;
pub const IP6T_SRH_INV_LAST_EQ: u16 = 0x0080;
pub const IP6T_SRH_INV_LAST_GT: u16 = 0x0100;
pub const IP6T_SRH_INV_LAST_LT: u16 = 0x0200;
pub const IP6T_SRH_INV_TAG: u16 = 0x0400;
pub const IP6T_SRH_INV_PSID: u16 = 0x0800;
pub const IP6T_SRH_INV_NSID: u16 = 0x1000;
pub const IP6T_SRH_INV_LSID: u16 = 0x2000;
pub const IP6T_SRH_INV_MASK: u16 = 0x3FFF;

/**
 *      struct ip6t_srh - SRH match options
 *      @next_hdr: Next header field of SRH
 *      @hdr_len: Extension header length field of SRH
 *      @segs_left: Segments left field of SRH
 *      @last_entry: Last entry field of SRH
 *      @tag: Tag field of SRH
 *      @mt_flags: match options
 *      @mt_invflags: Invert the sense of match options
 */
#[repr(C)]
pub struct ip6t_srh {
    pub next_hdr: u8,
    pub hdr_len: u8,
    pub segs_left: u8,
    pub last_entry: u8,
    pub tag: u16,
    pub mt_flags: u16,
    pub mt_invflags: u16,
}

/**
 *      struct ip6t_srh1 - SRH match options (revision 1)
 *      @next_hdr: Next header field of SRH
 *      @hdr_len: Extension header length field of SRH
 *      @segs_left: Segments left field of SRH
 *      @last_entry: Last entry field of SRH
 *      @tag: Tag field of SRH
 *      @psid_addr: Address of previous SID in SRH SID list
 *      @nsid_addr: Address of NEXT SID in SRH SID list
 *      @lsid_addr: Address of LAST SID in SRH SID list
 *      @psid_msk: Mask of previous SID in SRH SID list
 *      @nsid_msk: Mask of next SID in SRH SID list
 *      @lsid_msk: MAsk of last SID in SRH SID list
 *      @mt_flags: match options
 *      @mt_invflags: Invert the sense of match options
 */
#[repr(C)]
pub struct ip6t_srh1 {
    pub next_hdr: u8,
    pub hdr_len: u8,
    pub segs_left: u8,
    pub last_entry: u8,
    pub tag: u16,
    pub psid_addr: in6_addr,
    pub nsid_addr: in6_addr,
    pub lsid_addr: in6_addr,
    pub psid_msk: in6_addr,
    pub nsid_msk: in6_addr,
    pub lsid_msk: in6_addr,
    pub mt_flags: u16,
    pub mt_invflags: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
