/* SPDX-License-Identifier: GPL-2.0 */

// Translated from firewire.h. The Linux integer types __be64 and __be16 are
// represented by their underlying integer widths; endian interpretation is
// preserved by the surrounding protocol contract.

/* Pseudo L2 address */
pub const FWNET_ALEN: usize = 16;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct fwnet_hwaddr_uc {
    pub uniq_id: u64, /* EUI-64 */
    pub max_rec: u8, /* max packet size */
    pub sspd: u8,    /* max speed */
    pub fifo: [u8; 6], /* FIFO addr */
}

#[repr(C)]
pub union fwnet_hwaddr {
    pub u: [u8; FWNET_ALEN],
    /* "Hardware address" defined in RFC2734/RF3146 */
    pub uc: fwnet_hwaddr_uc,
}

/* Pseudo L2 Header */
pub const FWNET_HLEN: usize = 18;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct fwnet_header {
    pub h_dest: [u8; FWNET_ALEN], /* destination address */
    pub h_proto: u16,              /* packet type ID field */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
