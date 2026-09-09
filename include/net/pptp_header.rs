/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: linux/types.h and net/gre.h provide the endian-qualified
// integer types and `gre_base_hdr` used by this header.

pub const PPP_LCP_ECHOREQ: u32 = 0x09;
pub const PPP_LCP_ECHOREP: u32 = 0x0A;
pub const SC_RCV_BITS: u32 = SC_RCV_B7_1 | SC_RCV_B7_0 | SC_RCV_ODDP | SC_RCV_EVNP;

pub const MISSING_WINDOW: u32 = 20;

#[inline]
pub const fn wrapped(curseq: u32, lastseq: u32) -> bool {
    ((curseq & 0xffffff00) == 0) && ((lastseq & 0xffffff00) == 0xffffff00)
}

#[repr(C, packed)]
pub struct pptp_gre_header {
    pub gre_hd: gre_base_hdr,
    pub payload_len: u16,
    pub call_id: u16,
    pub seq: u32,
    pub ack: u32,
}

pub const PPTP_HEADER_OVERHEAD: usize = 2 + core::mem::size_of::<pptp_gre_header>();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
