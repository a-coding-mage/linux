// SPDX-License-Identifier: GPL-2.0-only
// Rust translation of the Atmel Extensible DMA Controller driver.
// Kernel-provided types and operations are intentionally left as external
// dependencies, matching the declarations supplied by the original source.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

macro_rules! BIT { ($n:expr) => { 1u32 << ($n) }; }
macro_rules! GENMASK { ($h:expr, $l:expr) => { ((u32::MAX >> (31 - ($h))) & (u32::MAX << ($l))) }; }

pub const AT_XDMAC_GTYPE: u32 = 0x00;
pub const AT_XDMAC_GCFG: u32 = 0x04;
pub const AT_XDMAC_GWAC: u32 = 0x08;
pub const AT_XDMAC_GIE: u32 = 0x0c;
pub const AT_XDMAC_GID: u32 = 0x10;
pub const AT_XDMAC_GIM: u32 = 0x14;
pub const AT_XDMAC_GIS: u32 = 0x18;
pub const AT_XDMAC_GE: u32 = 0x1c;
pub const AT_XDMAC_GD: u32 = 0x20;
pub const AT_XDMAC_GS: u32 = 0x24;
pub const AT_XDMAC_VERSION: u32 = 0xffc;
pub const AT_XDMAC_CIE: u32 = 0x00;
pub const AT_XDMAC_CID: u32 = 0x04;
pub const AT_XDMAC_CIM: u32 = 0x08;
pub const AT_XDMAC_CIS: u32 = 0x0c;
pub const AT_XDMAC_CSA: u32 = 0x10;
pub const AT_XDMAC_CDA: u32 = 0x14;
pub const AT_XDMAC_CNDA: u32 = 0x18;
pub const AT_XDMAC_CNDC: u32 = 0x1c;
pub const AT_XDMAC_CUBC: u32 = 0x20;
pub const AT_XDMAC_CBC: u32 = 0x24;
pub const AT_XDMAC_CC: u32 = 0x28;
pub const AT_XDMAC_CDS_MSP: u32 = 0x2c;
pub const AT_XDMAC_CSUS: u32 = 0x30;
pub const AT_XDMAC_CDUS: u32 = 0x34;

pub const AT_XDMAC_CIE_BIE: u32 = BIT!(0);
pub const AT_XDMAC_CIE_LIE: u32 = BIT!(1);
pub const AT_XDMAC_CIE_DIE: u32 = BIT!(2);
pub const AT_XDMAC_CIE_FIE: u32 = BIT!(3);
pub const AT_XDMAC_CIE_RBEIE: u32 = BIT!(4);
pub const AT_XDMAC_CIE_WBEIE: u32 = BIT!(5);
pub const AT_XDMAC_CIE_ROIE: u32 = BIT!(6);
pub const AT_XDMAC_CNDC_NDE: u32 = 1;
pub const AT_XDMAC_CNDC_NDSUP: u32 = 1 << 1;
pub const AT_XDMAC_CNDC_NDDUP: u32 = 1 << 2;
pub const AT_XDMAC_CNDC_NDVIEW_MASK: u32 = GENMASK!(28, 27);
pub const AT_XDMAC_CNDC_NDVIEW_NDV0: u32 = 0;
pub const AT_XDMAC_CNDC_NDVIEW_NDV1: u32 = 1 << 3;
pub const AT_XDMAC_CNDC_NDVIEW_NDV2: u32 = 2 << 3;
pub const AT_XDMAC_CNDC_NDVIEW_NDV3: u32 = 3 << 3;
pub const AT_XDMAC_CC_TYPE_PER_TRAN: u32 = 1;
pub const AT_XDMAC_CC_DWIDTH_MASK: u32 = 3 << 11;
pub const AT_XDMAC_CC_DWIDTH_OFFSET: u32 = 11;
pub const AT_XDMAC_CC_DWIDTH_BYTE: u32 = 0;
pub const AT_XDMAC_CC_DWIDTH_HALFWORD: u32 = 1;
pub const AT_XDMAC_CC_DWIDTH_WORD: u32 = 2;
pub const AT_XDMAC_CC_DWIDTH_DWORD: u32 = 3;
pub const AT_XDMAC_MBR_UBC_UBLEN_MAX: u32 = 0x00ff_ffff;
pub const AT_XDMAC_MBR_UBC_NDE: u32 = 1 << 24;
pub const AT_XDMAC_MBR_UBC_NSEN: u32 = 1 << 25;
pub const AT_XDMAC_MBR_UBC_NDEN: u32 = 1 << 26;
pub const AT_XDMAC_MBR_UBC_NDV0: u32 = 0;
pub const AT_XDMAC_MBR_UBC_NDV1: u32 = 1 << 27;
pub const AT_XDMAC_MBR_UBC_NDV2: u32 = 2 << 27;
pub const AT_XDMAC_MBR_UBC_NDV3: u32 = 3 << 27;
pub const AT_XDMAC_MAX_CHAN: usize = 0x20;
pub const AT_XDMAC_MAX_CSIZE: u32 = 16;
pub const AT_XDMAC_MAX_DWIDTH: u32 = 8;
pub const AT_XDMAC_RESIDUE_MAX_RETRIES: u32 = 5;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct at_xdmac_layout { pub grs:u8, pub gws:u8, pub grws:u8, pub grwr:u8, pub gswr:u8, pub gsws:u8, pub gswf:u8, pub chan_cc_reg_base:u8, pub sdif:bool, pub axi_config:bool }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct at_xdmac_lld { pub mbr_nda:u32, pub mbr_ubc:u32, pub mbr_sa:u32, pub mbr_da:u32, pub mbr_cfg:u32, pub mbr_bc:u32, pub mbr_ds:u32, pub mbr_sus:u32, pub mbr_dus:u32 }

#[repr(C)]
pub struct at_xdmac_desc { pub lld: at_xdmac_lld, pub direction: u32, pub tx_dma_desc: *mut core::ffi::c_void, pub desc_node: *mut core::ffi::c_void, pub active_xfer: bool, pub xfer_size: u32, pub descs_list: *mut core::ffi::c_void, pub xfer_node: *mut core::ffi::c_void }

#[inline] pub const fn at_xdmac_chan_is_peripheral_xfer(cfg: u32) -> bool { cfg & AT_XDMAC_CC_TYPE_PER_TRAN != 0 }
#[inline] pub const fn at_xdmac_get_dwidth(cfg: u32) -> u8 { ((cfg & AT_XDMAC_CC_DWIDTH_MASK) >> AT_XDMAC_CC_DWIDTH_OFFSET) as u8 }
#[inline] pub const fn at_xdmac_cn(n: u32) -> u32 { (n & 0x1f) + 1 }
#[inline] pub const fn at_xdmac_fifo_sz(n: u32) -> u32 { (n >> 5) & 0x7ff }
#[inline] pub const fn at_xdmac_nb_req(n: u32) -> u32 { ((n >> 16) & 0x3f) + 1 }
#[inline] pub const fn at_xdmac_cnda_nda(n: u32) -> u32 { n & 0xffff_fffc }
#[inline] pub const fn at_xdmac_cnda_ndaif(n: u32) -> u32 { n & 1 }

// The remaining driver entry points retain the original external-kernel
// interfaces and are supplied by the surrounding kernel translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
