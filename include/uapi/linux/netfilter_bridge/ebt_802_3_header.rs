/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependencies corresponding to <linux/types.h> and <linux/if_ether.h>.

pub const EBT_802_3_SAP: u8 = 0x01;
pub const EBT_802_3_TYPE: u8 = 0x02;

pub const EBT_802_3_MATCH: &str = "802_3";

/*
 * If frame has DSAP/SSAP value 0xaa you must check the SNAP type
 * to discover what kind of packet we're carrying.
 */
pub const CHECK_TYPE: u8 = 0xaa;

/*
 * Control field may be one or two bytes.  If the first byte has
 * the value 0x03 then the entire length is one byte, otherwise it is two.
 * One byte controls are used in Unnumbered Information frames.
 * Two byte controls are used in Numbered Information frames.
 */
pub const IS_UI: u8 = 0x03;

// The C source combines EBT_802_3_SAP, EBT_802_3_TYPE, and EBT_802_3 here;
// EBT_802_3 is not defined locally, so the locally representable mask is used.
pub const EBT_802_3_MASK: u8 = EBT_802_3_SAP | EBT_802_3_TYPE;

/* ui has one byte ctrl, ni has two */
#[repr(C)]
pub struct hdr_ui {
    pub dsap: u8,
    pub ssap: u8,
    pub ctrl: u8,
    pub orig: [u8; 3],
    pub r#type: u16,
}

#[repr(C)]
pub struct hdr_ni {
    pub dsap: u8,
    pub ssap: u8,
    pub ctrl: u16,
    pub orig: [u8; 3],
    pub r#type: u16,
}

#[repr(C)]
pub union ebt_802_3_hdr_llc {
    pub ui: hdr_ui,
    pub ni: hdr_ni,
}

#[repr(C)]
pub struct ebt_802_3_hdr {
    pub daddr: [u8; 6],
    pub saddr: [u8; 6],
    pub len: u16,
    pub llc: ebt_802_3_hdr_llc,
}

#[repr(C)]
pub struct ebt_802_3_info {
    pub sap: u8,
    pub r#type: u16,
    pub bitmask: u8,
    pub invflags: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
