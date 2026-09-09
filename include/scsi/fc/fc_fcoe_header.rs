/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright(c) 2007 Intel Corporation. All rights reserved.
 *
 * Maintained at www.Open-FCoE.org
 */

/*
 * FCoE - Fibre Channel over Ethernet.
 * See T11 FC-BB-5 Rev 2.00 (09-056v5.pdf)
 */

/* Default FC_FCOE_OUI / FC-MAP value. */
pub const FC_FCOE_OUI: u32 = 0x0efc00; /* upper 24 bits of FCOE MAC */

/* Fabric Login (FLOGI) MAC for non-FIP use. Non-FIP use is deprecated. */
pub const FC_FCOE_FLOGI_MAC: [u8; 6] = [0x0e, 0xfc, 0x00, 0xff, 0xff, 0xfe];

pub const FC_FCOE_VER: u32 = 0; /* version */

/* Ethernet Addresses based on FC S_ID and D_ID. */
#[inline]
pub const fn FC_FCOE_ENCAPS_ID(n: u64) -> u64 {
    ((FC_FCOE_OUI as u64) << 24) | n
}

#[inline]
pub const fn FC_FCOE_DECAPS_ID(n: u64) -> u64 {
    n >> 24
}

/*
 * FCoE frame header - 14 bytes
 * This follows the VLAN header, which includes the ethertype.
 */
#[repr(C)]
pub struct fcoe_hdr {
    pub fcoe_ver: u8,       /* version field - upper 4 bits */
    pub fcoe_resvd: [u8; 12], /* reserved - send zero and ignore */
    pub fcoe_sof: u8,       /* start of frame per RFC 3643 */
}

#[inline]
pub unsafe fn FC_FCOE_DECAPS_VER(hp: *const fcoe_hdr) -> u8 {
    (*hp).fcoe_ver >> 4
}

#[inline]
pub unsafe fn FC_FCOE_ENCAPS_VER(hp: *mut fcoe_hdr, ver: u8) {
    (*hp).fcoe_ver = ver << 4;
}

/* FCoE CRC & EOF - 8 bytes. */
#[repr(C, packed)]
pub struct fcoe_crc_eof {
    pub fcoe_crc32: u32, /* __le32: CRC for FC packet */
    pub fcoe_eof: u8,    /* EOF from RFC 3643 */
    pub fcoe_resvd: [u8; 3], /* reserved - send zero and ignore */
}

/* Minimum FCoE + FC header length. */
pub const FCOE_HEADER_LEN: usize = 38;

/* Minimum FCoE frame size. */
pub const FCOE_MIN_FRAME: usize = 46;

/* FCoE Link Error Status Block: T11 FC-BB-5 Rev2.0, Clause 7.10. */
#[repr(C)]
pub struct fcoe_fc_els_lesb {
    pub lesb_link_fail: u32, /* __be32: link failure count */
    pub lesb_vlink_fail: u32, /* virtual link failure count */
    pub lesb_miss_fka: u32, /* missing FIP keep-alive count */
    pub lesb_symb_err: u32, /* symbol error during carrier count */
    pub lesb_err_block: u32, /* errored block count */
    pub lesb_fcs_error: u32, /* frame check sequence error count */
}

/*
 * fc_fcoe_set_mac - Store OUI + DID into MAC address field.
 * @mac: mac address to be set
 * @did: fc dest id to use
 */
#[inline]
pub unsafe fn fc_fcoe_set_mac(mac: *mut u8, did: *const u8) {
    *mac.add(0) = (FC_FCOE_OUI >> 16) as u8;
    *mac.add(1) = (FC_FCOE_OUI >> 8) as u8;
    *mac.add(2) = FC_FCOE_OUI as u8;
    *mac.add(3) = *did.add(0);
    *mac.add(4) = *did.add(1);
    *mac.add(5) = *did.add(2);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
