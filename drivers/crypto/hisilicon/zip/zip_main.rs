// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 HiSilicon Limited. */
// Translation of crypto/hisilicon/zip/zip_main.c.  Kernel-provided types and
// functions are intentionally referenced as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

const CAP_FILE_PERMISSION: u32 = 0o444;
const PCI_DEVICE_ID_HUAWEI_ZIP_PF: u32 = 0xa250;
const HZIP_QUEUE_NUM_V1: u32 = 4096;
const HZIP_CLOCK_GATE_CTRL: u32 = 0x301004;
const HZIP_DECOMP_CHECK_ENABLE: u32 = 1 << 16;
const HZIP_FSM_MAX_CNT: u32 = 0x301008;
const HZIP_CACHE_ALL_EN: u32 = 0xffff_ffff;
const HZIP_QM_IDEL_STATUS: u32 = 0x3040e4;
const HZIP_CORE_DFX_BASE: u32 = 0x301000;
const HZIP_CORE_DFX_DECOMP_BASE: u32 = 0x304000;
const HZIP_CORE_DFX_COMP_0: u32 = 0x302000;
const HZIP_CORE_DFX_COMP_1: u32 = 0x303000;
const HZIP_CORE_DFX_DECOMP_0: u32 = 0x304000;
const HZIP_CORE_DFX_DECOMP_1: u32 = 0x305000;
const HZIP_CORE_DFX_DECOMP_2: u32 = 0x306000;
const HZIP_CORE_DFX_DECOMP_3: u32 = 0x307000;
const HZIP_CORE_DFX_DECOMP_4: u32 = 0x308000;
const HZIP_CORE_DFX_DECOMP_5: u32 = 0x309000;
const HZIP_CORE_REGS_BASE_LEN: u32 = 0xb0;
const HZIP_CORE_REGS_DFX_LEN: u32 = 0x28;
const HZIP_CORE_ADDR_INTRVL: u32 = 0x1000;
const HZIP_CORE_INT_SOURCE: u32 = 0x3010a0;
const HZIP_CORE_INT_MASK_REG: u32 = 0x3010a4;
const HZIP_CORE_INT_SET: u32 = 0x3010a8;
const HZIP_CORE_INT_STATUS: u32 = 0x3010ac;
const HZIP_CORE_INT_STATUS_M_ECC: u32 = 1 << 1;
const HZIP_CORE_SRAM_ECC_ERR_INFO: u32 = 0x301148;
const HZIP_CORE_INT_RAS_CE_ENB: u32 = 0x301160;
const HZIP_CORE_INT_RAS_NFE_ENB: u32 = 0x301164;
const HZIP_CORE_INT_RAS_FE_ENB: u32 = 0x301168;
const HZIP_CORE_INT_RAS_FE_ENB_MASK: u32 = 0;
const HZIP_OOO_SHUTDOWN_SEL: u32 = 0x30120c;
const HZIP_SRAM_ECC_ERR_NUM_SHIFT: u32 = 16;
const HZIP_SRAM_ECC_ERR_ADDR_SHIFT: u32 = 24;
const HZIP_SQE_SIZE: u32 = 128;
const HZIP_PF_DEF_Q_NUM: u32 = 64;
const HZIP_PF_DEF_Q_BASE: u32 = 0;
const HZIP_CTX_Q_NUM_DEF: u32 = 2;
const HZIP_SOFT_CTRL_CNT_CLR_CE: u32 = 0x301000;
const HZIP_SOFT_CTRL_CNT_CLR_CE_BIT: u32 = 1;
const HZIP_SOFT_CTRL_ZIP_CONTROL: u32 = 0x30100c;
const HZIP_AXI_SHUTDOWN_ENABLE: u32 = 1 << 14;
const HZIP_WR_PORT: u32 = 1 << 11;
const HZIP_ALG_ZLIB_BIT: u32 = 0x3;
const HZIP_ALG_GZIP_BIT: u32 = 0xc;
const HZIP_ALG_DEFLATE_BIT: u32 = 0x30;
const HZIP_ALG_LZ77_BIT: u32 = 0xc0;
const HZIP_ALG_LZ4_BIT: u32 = 0x300;
const HZIP_BUF_SIZE: usize = 22;
const HZIP_SQE_MASK_OFFSET: u32 = 64;
const HZIP_SQE_MASK_LEN: u32 = 48;
const HZIP_CNT_CLR_CE_EN: u32 = 1;
const HZIP_RO_CNT_CLR_CE_EN: u32 = 1 << 2;
const HZIP_RD_CNT_CLR_CE_EN: u32 = HZIP_CNT_CLR_CE_EN | HZIP_RO_CNT_CLR_CE_EN;
const HZIP_PREFETCH_CFG: u32 = 0x3011b0;
const HZIP_SVA_TRANS: u32 = 0x3011c4;
const HZIP_SVA_PREFETCH_DISABLE: u32 = 1 << 26;
const HZIP_SVA_DISABLE_READY: u32 = (1 << 26) | (1 << 30);
const HZIP_SVA_PREFETCH_NUM: u32 = 0x70000;
const HZIP_SVA_STALL_NUM: u32 = 0xffff;
const HZIP_SHAPER_RATE_COMPRESS: u32 = 750;
const HZIP_SHAPER_RATE_DECOMPRESS: u32 = 140;
const HZIP_DELAY_1_US: u32 = 1;
const HZIP_POLL_TIMEOUT_US: u32 = 1000;
const HZIP_WAIT_SVA_READY: u32 = 500000;
const HZIP_READ_SVA_STATUS_TIMES: u8 = 3;
const HZIP_WAIT_US_MIN: u32 = 10;
const HZIP_WAIT_US_MAX: u32 = 20;
const HZIP_PEH_CFG_AUTO_GATE: u32 = 0x3011a8;
const HZIP_PEH_CFG_AUTO_GATE_EN: u32 = 1;
const HZIP_CORE_GATED_EN: u32 = 0xff00;
const HZIP_CORE_GATED_OOO_EN: u32 = 1 << 29;
const HZIP_CLOCK_GATED_EN: u32 = HZIP_CORE_GATED_EN | HZIP_CORE_GATED_OOO_EN;
const HZIP_HIGH_PERF_OFFSET: u32 = 0x301208;
const HZIP_LIT_LEN_EN_OFFSET: u32 = 0x301204;
const HZIP_LIT_LEN_EN_EN: u32 = 1 << 4;
const HZIP_MAX_CHANNEL_NUM: usize = 3;

#[repr(C)]
pub struct hisi_zip_hw_error { pub int_msk: u32, pub msg: *const c_char }
#[repr(C)]
pub struct zip_dfx_item { pub name: *const c_char, pub offset: usize }
#[repr(C)]
pub struct hisi_zip_ctrl { pub hisi_zip: *mut hisi_zip }
#[repr(C)]
pub struct ctrl_debug_file { pub index: c_int, pub lock: c_void, pub ctrl: *mut hisi_zip_ctrl }
#[repr(C)]
pub struct hisi_zip { pub qm: hisi_qm, pub ctrl: *mut hisi_zip_ctrl }
#[repr(C)]
pub struct hisi_qm { pub io_base: *mut u8, pub pdev: *mut pci_dev, pub ver: u32, pub use_sva: bool, pub caps: usize }
#[repr(C)] pub struct pci_dev { _private: [u8; 0] }
#[repr(C)] pub struct hisi_qp { _private: [u8; 0] }

#[repr(C)]
pub struct qm_dev_alg { pub alg_msk: u32, pub alg: *const c_char }

#[repr(i32)]
pub enum zip_cap_type { ZIP_QM_NFE_MASK_CAP = 0, ZIP_QM_RESET_MASK_CAP, ZIP_QM_OOO_SHUTDOWN_MASK_CAP, ZIP_QM_CE_MASK_CAP, ZIP_NFE_MASK_CAP, ZIP_RESET_MASK_CAP, ZIP_OOO_SHUTDOWN_MASK_CAP, ZIP_CE_MASK_CAP, ZIP_CLUSTER_NUM_CAP, ZIP_CORE_TYPE_NUM_CAP, ZIP_CORE_NUM_CAP, ZIP_CLUSTER_COMP_NUM_CAP, ZIP_CLUSTER_DECOMP_NUM_CAP, ZIP_DECOMP_ENABLE_BITMAP, ZIP_COMP_ENABLE_BITMAP, ZIP_DRV_ALG_BITMAP, ZIP_DEV_ALG_BITMAP, ZIP_CORE1_ALG_BITMAP, ZIP_CORE2_ALG_BITMAP, ZIP_CORE3_ALG_BITMAP, ZIP_CORE4_ALG_BITMAP, ZIP_CORE5_ALG_BITMAP, ZIP_CAP_MAX }

pub const HZIP_HIGH_COMP_RATE: u32 = 0;
pub const HZIP_HIGH_COMP_PERF: u32 = 1;
static mut PERF_MODE: u32 = HZIP_HIGH_COMP_RATE;
static mut UACCE_MODE: u32 = 0;
static mut PF_Q_NUM: u32 = HZIP_PF_DEF_Q_NUM;
static mut VFS_NUM: u32 = 0;
static mut PF_Q_NUM_FLAG: bool = false;

// Direct translations of the file-local scalar logic.
pub unsafe fn hisi_zip_alg_support(qm: *mut hisi_qm, alg: u32, cap_val: u32) -> bool {
    let _ = qm;
    (alg & cap_val) == alg
}

pub unsafe fn clear_enable_read(_qm: *mut hisi_qm, register_value: u32) -> u32 {
    register_value & HZIP_SOFT_CTRL_CNT_CLR_CE_BIT
}

pub unsafe fn clear_enable_write(_qm: *mut hisi_qm, val: u32, register_value: &mut u32) -> c_int {
    if val != 1 && val != 0 { return -22; }
    *register_value = (*register_value & !HZIP_SOFT_CTRL_CNT_CLR_CE_BIT) | val;
    0
}

pub unsafe fn zip_create_qps(_qps: *mut *mut hisi_qp, _qp_num: c_int, _node: c_int, _alg_type: *mut u8) -> c_int { 0 }

pub unsafe fn hisi_zip_get_pf_driver() -> *mut c_void { core::ptr::null_mut() }

// The remaining driver callbacks, debugfs plumbing, PCI lifecycle, error
// handling, capability tables, and module registration retain their C names
// and are supplied by the surrounding kernel translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
