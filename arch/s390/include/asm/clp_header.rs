/* SPDX-License-Identifier: GPL-2.0 */

/* CLP common request & response block size */
pub const CLP_BLK_SIZE: usize = PAGE_SIZE;

/* Call Logical Processor - Command Code */
pub const CLP_SLPC: u16 = 0x0001;

pub const CLP_LPS_BASE: u32 = 0;
pub const CLP_LPS_PCI: u32 = 2;

#[repr(C, packed)]
pub struct clp_req_hdr {
    pub len: u16,
    pub cmd: u16,
    /* C bit-fields: fmt occupies bits 0..4 and reserved1 bits 4..32. */
    pub fmt_reserved1: u32,
    pub reserved2: u64,
}

#[repr(C, packed)]
pub struct clp_rsp_hdr {
    pub len: u16,
    pub rsp: u16,
    /* C bit-fields: fmt occupies bits 0..4 and reserved1 bits 4..32. */
    pub fmt_reserved1: u32,
    pub reserved2: u64,
}

/* CLP Response Codes */
pub const CLP_RC_OK: u16 = 0x0010; /* Command request successfully */
pub const CLP_RC_CMD: u16 = 0x0020; /* Command code not recognized */
pub const CLP_RC_PERM: u16 = 0x0030; /* Command not authorized */
pub const CLP_RC_FMT: u16 = 0x0040; /* Invalid command request format */
pub const CLP_RC_LEN: u16 = 0x0050; /* Invalid command request length */
pub const CLP_RC_8K: u16 = 0x0060; /* Command requires 8K LPCB */
pub const CLP_RC_RESNOT0: u16 = 0x0070; /* Reserved field not zero */
pub const CLP_RC_NODATA: u16 = 0x0080; /* No data available */
pub const CLP_RC_FC_UNKNOWN: u16 = 0x0100; /* Function code not recognized */

/* Store logical-processor characteristics request */
#[repr(C, packed)]
pub struct clp_req_slpc {
    pub hdr: clp_req_hdr,
}

#[repr(C, packed)]
pub struct clp_rsp_slpc {
    pub hdr: clp_rsp_hdr,
    pub reserved2: [u32; 4],
    pub lpif: [u32; 8],
    pub reserved3: [u32; 8],
    pub lpic: [u32; 8],
}

#[repr(C, packed)]
pub struct clp_req_rsp_slpc {
    pub request: clp_req_slpc,
    pub response: clp_rsp_slpc,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
