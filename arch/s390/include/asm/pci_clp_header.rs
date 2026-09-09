/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: asm/clp.h

/* Call Logical Processor - Command Codes */
pub const CLP_SLPC: u16 = 0x0001;
pub const CLP_LIST_PCI: u16 = 0x0002;
pub const CLP_QUERY_PCI_FN: u16 = 0x0003;
pub const CLP_QUERY_PCI_FNGRP: u16 = 0x0004;
pub const CLP_SET_PCI_FN: u16 = 0x0005;

/* PCI function handle list entry */
#[repr(C, packed)]
pub struct clp_fh_list_entry {
    pub device_id: u16,
    pub vendor_id: u16,
    // config_state: 1 bit; remaining 31 bits reserved
    pub config_state: u32,
    pub fid: u32, // PCI function id
    pub fh: u32, // PCI function handle
}

pub const CLP_RC_SETPCIFN_FH: u16 = 0x0101; // Invalid PCI fn handle
pub const CLP_RC_SETPCIFN_FHOP: u16 = 0x0102; // Fn handle not valid for op
pub const CLP_RC_SETPCIFN_DMAAS: u16 = 0x0103; // Invalid DMA addr space
pub const CLP_RC_SETPCIFN_RES: u16 = 0x0104; // Insufficient resources
pub const CLP_RC_SETPCIFN_ALRDY: u16 = 0x0105; // Fn already in requested state
pub const CLP_RC_SETPCIFN_ERR: u16 = 0x0106; // Fn in permanent error state
pub const CLP_RC_SETPCIFN_RECPND: u16 = 0x0107; // Error recovery pending
pub const CLP_RC_SETPCIFN_BUSY: u16 = 0x0108; // Fn busy
pub const CLP_RC_LISTPCI_BADRT: u16 = 0x010a; // Resume token not recognized
pub const CLP_RC_QUERYPCIFG_PFGID: u16 = 0x010b; // Unrecognized PFGID

pub const LIST_PCI_HDR_LEN: usize = 32;
pub const CLP_FH_LIST_NR_ENTRIES: usize =
    (CLP_BLK_SIZE as usize - 2 * LIST_PCI_HDR_LEN) / core::mem::size_of::<clp_fh_list_entry>();

pub const CLP_SET_ENABLE_PCI_FN: u8 = 0;
pub const CLP_SET_DISABLE_PCI_FN: u8 = 1;
pub const CLP_SET_ENABLE_MIO: u8 = 2;
pub const CLP_SET_DISABLE_MIO: u8 = 3;
pub const CLP_UTIL_STR_LEN: usize = 64;
pub const CLP_PFIP_NR_SEGMENTS: usize = 4;
pub const PCI_FUNC_TYPE_ISM: u8 = 0x5;

extern "C" {
    pub static mut zpci_unique_uid: bool;
}

#[repr(C, packed)]
pub struct clp_rsp_slpc_pci {
    pub hdr: clp_rsp_hdr,
    pub reserved2: [u32; 4],
    pub lpif: [u32; 8],
    pub reserved3: [u32; 4],
    // vwb: 1 bit, reserved: 1 bit, mio_wb: 6 bits, reserved: 24 bits
    pub vwb_mio_wb: u32,
    pub reserved5: [u32; 3],
    pub lpic: [u32; 8],
}

#[repr(C, packed)]
pub struct clp_req_list_pci { pub hdr: clp_req_hdr, pub resume_token: u64, pub reserved2: u64 }

#[repr(C, packed)]
pub struct clp_rsp_list_pci {
    pub hdr: clp_rsp_hdr,
    pub resume_token: u64,
    pub reserved2: u32,
    pub max_fn: u16,
    // 7 reserved bits, uid_checking: 1 bit
    pub uid_checking: u8,
    pub entry_size: u8,
    pub fh_list: [clp_fh_list_entry; CLP_FH_LIST_NR_ENTRIES],
}

#[repr(C, packed)]
pub struct mio_info {
    // valid: 6 bits; reserved: 26 bits; reserved: 32 bits
    pub valid: u32,
    pub reserved_word: u32,
    pub addr: [mio_info_addr; PCI_STD_NUM_BARS],
    pub reserved: [u32; 6],
}
#[repr(C, packed)]
pub struct mio_info_addr { pub wb: u64, pub wt: u64 }

#[repr(C, packed)]
pub struct clp_req_query_pci { pub hdr: clp_req_hdr, pub fh: u32, pub reserved2: u32, pub reserved3: u64 }

#[repr(C, packed)]
pub struct clp_rsp_query_pci {
    pub hdr: clp_rsp_hdr, pub vfn: u16,
    // reserved: 2, tid_avail: 1, rid_avail: 1, is_physfn: 1, reserved: 1,
    // mio_addr_avail: 1, util_str_avail: 1, pfgid: 8
    pub flags_pfgid: u16,
    pub fid: u32,
    pub bar_size: [u8; PCI_STD_NUM_BARS], pub pchid: u16,
    pub bar: [__le32; PCI_STD_NUM_BARS], pub pfip: [u8; CLP_PFIP_NR_SEGMENTS],
    pub fidparm: u8,
    // reserved: 4 bits, port: 4 bits
    pub reserved3_port: u8,
    pub fmb_len: u8, pub pft: u8, pub sdma: u64, pub edma: u64,
    pub rid: u16, pub reserved0: u32, pub tid: u16, pub reserved: [u32; 9],
    pub uid: u32, pub util_str: [u8; CLP_UTIL_STR_LEN], pub reserved2: [u32; 16],
    pub mio: mio_info,
}

pub const ZPCI_RID_MASK_DEVFN: u16 = 0x00ff;

#[repr(C, packed)]
pub struct clp_req_query_pci_grp {
    pub hdr: clp_req_hdr,
    // reserved: 24 bits, pfgid: 8 bits
    pub reserved2_pfgid: u32,
    pub reserved3: u32, pub reserved4: u64,
}

#[repr(C, packed)]
pub struct clp_rsp_query_pci_grp {
    pub hdr: clp_rsp_hdr,
    // reserved: 4 bits, noi: 12 bits
    pub noi: u16, pub version: u8,
    // reserved: 2, rtr: 1, reserved: 3, frame: 1, refresh: 1
    pub flags: u8,
    // reserved: 3 bits, maxstbl: 13 bits
    pub maxstbl: u16,
    pub mui: u16, pub dtsm: u8, pub reserved3: u8, pub maxfaal: u16,
    // reserved: 4 bits, dnoi: 12 bits
    pub dnoi: u16,
    pub maxcpu: u16, pub dasm: u64, pub msia: u64, pub reserved4: u64, pub reserved5: u64,
}

#[repr(C, packed)]
pub struct clp_req_set_pci {
    pub hdr: clp_req_hdr, pub fh: u32, pub reserved2: u16, pub oc: u8, pub ndas: u8,
    pub reserved3: u32, pub gisa: u32,
}
#[repr(C, packed)]
pub struct clp_rsp_set_pci { pub hdr: clp_rsp_hdr, pub fh: u32, pub reserved1: u32, pub reserved2: u64, pub mio: mio_info }

#[repr(C, packed)]
pub struct clp_req_rsp_slpc_pci { pub request: clp_req_slpc, pub response: clp_rsp_slpc_pci }
#[repr(C, packed)]
pub struct clp_req_rsp_list_pci { pub request: clp_req_list_pci, pub response: clp_rsp_list_pci }
#[repr(C, packed)]
pub struct clp_req_rsp_set_pci { pub request: clp_req_set_pci, pub response: clp_rsp_set_pci }
#[repr(C, packed)]
pub struct clp_req_rsp_query_pci { pub request: clp_req_query_pci, pub response: clp_rsp_query_pci }
#[repr(C, packed)]
pub struct clp_req_rsp_query_pci_grp { pub request: clp_req_query_pci_grp, pub response: clp_rsp_query_pci_grp }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
