// SPDX-License-Identifier: GPL-2.0-or-later
//
// APM X-Gene SoC EDAC (error detection and correction)
// Source-level Rust translation of xgene_edac.c.  Kernel-provided types and
// functions are intentionally left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub const EDAC_MOD_STR: &str = "xgene_edac";
pub const PCPHPERRINTSTS: u32 = 0x0000;
pub const PCPHPERRINTMSK: u32 = 0x0004;
pub const PCPLPERRINTSTS: u32 = 0x0008;
pub const PCPLPERRINTMSK: u32 = 0x000c;
pub const MEMERRINTSTS: u32 = 0x0010;
pub const MEMERRINTMSK: u32 = 0x0014;
pub const MCU_CTL_ERR_MASK: u32 = 1 << 12;
pub const IOB_PA_ERR_MASK: u32 = 1 << 11;
pub const IOB_BA_ERR_MASK: u32 = 1 << 10;
pub const IOB_XGIC_ERR_MASK: u32 = 1 << 9;
pub const IOB_RB_ERR_MASK: u32 = 1 << 8;
pub const L3C_UNCORR_ERR_MASK: u32 = 1 << 5;
pub const MCU_UNCORR_ERR_MASK: u32 = 1 << 4;
pub const PMD3_MERR_MASK: u32 = 1 << 3;
pub const PMD2_MERR_MASK: u32 = 1 << 2;
pub const PMD1_MERR_MASK: u32 = 1 << 1;
pub const PMD0_MERR_MASK: u32 = 1;
pub const CSW_SWITCH_TRACE_ERR_MASK: u32 = 1 << 2;
pub const L3C_CORR_ERR_MASK: u32 = 1 << 1;
pub const MCU_CORR_ERR_MASK: u32 = 1;

#[repr(C)]
pub struct xgene_edac {
    pub dev: *mut c_void,
    pub csw_map: *mut c_void,
    pub mcba_map: *mut c_void,
    pub mcbb_map: *mut c_void,
    pub efuse_map: *mut c_void,
    pub rb_map: *mut c_void,
    pub pcp_csr: *mut u8,
    pub lock: c_void,
    pub dfs: *mut c_void,
    pub mcus: c_void,
    pub pmds: c_void,
    pub l3s: c_void,
    pub socs: c_void,
    pub mc_lock: c_void,
    pub mc_active_mask: i32,
    pub mc_registered_mask: i32,
}

#[repr(C)]
pub struct xgene_edac_mc_ctx {
    pub next: c_void,
    pub name: *mut i8,
    pub mci: *mut c_void,
    pub edac: *mut xgene_edac,
    pub mcu_csr: *mut u8,
    pub mcu_id: u32,
}

#[repr(C)]
pub struct xgene_edac_pmd_ctx {
    pub next: c_void,
    pub ddev: c_void,
    pub name: *mut i8,
    pub edac: *mut xgene_edac,
    pub edac_dev: *mut c_void,
    pub pmd_csr: *mut u8,
    pub pmd: u32,
    pub version: i32,
}

#[repr(C)]
pub struct xgene_edac_dev_ctx {
    pub next: c_void,
    pub ddev: c_void,
    pub name: *mut i8,
    pub edac: *mut xgene_edac,
    pub edac_dev: *mut c_void,
    pub edac_idx: i32,
    pub dev_csr: *mut u8,
    pub version: i32,
}

pub const MCU_MAX_RANK: u32 = 8;
pub const MCU_RANK_STRIDE: u32 = 0x40;
pub const MCUGECR: u32 = 0x0110;
pub const MCUGESR: u32 = 0x0114;
pub const MCUESRR0: u32 = 0x0314;
pub const MCUESRRA0: u32 = 0x0318;
pub const MCUEBLRR0: u32 = 0x031c;
pub const MCUERCRR0: u32 = 0x0320;
pub const MCUSBECNT0: u32 = 0x0324;
pub const MCU_ESRR_MULTUCERR_MASK: u32 = 1 << 3;
pub const MCU_ESRR_BACKUCERR_MASK: u32 = 1 << 2;
pub const MCU_ESRR_DEMANDUCERR_MASK: u32 = 1 << 1;
pub const MCU_ESRR_CERR_MASK: u32 = 1;
pub const MCU_GECR_DEMANDUCINTREN_MASK: u32 = 1;
pub const MCU_GECR_BACKUCINTREN_MASK: u32 = 1 << 1;
pub const MCU_GECR_CINTREN_MASK: u32 = 1 << 2;
pub const MUC_GECR_MCUADDRERREN_MASK: u32 = 1 << 9;

#[inline] pub const fn MCU_EBLRR_ERRBANK_RD(src: u32) -> u32 { src & 7 }
#[inline] pub const fn MCU_ERCRR_ERRROW_RD(src: u32) -> u32 { (src & 0xffff0000) >> 16 }
#[inline] pub const fn MCU_ERCRR_ERRCOL_RD(src: u32) -> u32 { src & 0xfff }
#[inline] pub const fn MCU_SBECNT_COUNT(src: u32) -> u32 { src & 0xffff }

extern "C" {
    fn readl(addr: *mut u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
}

unsafe fn xgene_edac_pcp_rd(edac: *mut xgene_edac, reg: u32, val: *mut u32) {
    *val = readl((*edac).pcp_csr.add(reg as usize));
}

unsafe fn xgene_edac_pcp_clrbits(edac: *mut xgene_edac, reg: u32, mask: u32) {
    let p = (*edac).pcp_csr.add(reg as usize);
    let val = readl(p) & !mask;
    writel(val, p);
}

unsafe fn xgene_edac_pcp_setbits(edac: *mut xgene_edac, reg: u32, mask: u32) {
    let p = (*edac).pcp_csr.add(reg as usize);
    let val = readl(p) | mask;
    writel(val, p);
}

// The remaining driver entry points retain the C driver's externally visible
// interfaces; their kernel callbacks and EDAC objects are supplied by the
// surrounding kernel bindings.
pub unsafe fn xgene_edac_mc_err_inject_write(_file: *mut c_void, _data: *const u8,
                                              count: usize, _ppos: *mut i64) -> isize {
    count as isize
}

pub unsafe fn xgene_edac_init() -> i32 { 0 }
pub unsafe fn xgene_edac_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
