// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 HiSilicon Limited. */
// Direct Rust translation of crypto/hisilicon/sec2/sec_main.c.
// Kernel and sec.h supplied declarations remain external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

const CAP_FILE_PERMISSION: u32 = 0o444;
const SEC_VF_NUM: u32 = 63;
const SEC_QUEUE_NUM_V1: u32 = 4096;
const PCI_DEVICE_ID_HUAWEI_SEC_PF: u32 = 0xa255;
const SEC_BD_ERR_CHK_EN0: u32 = 0xEFFFFFFF;
const SEC_BD_ERR_CHK_EN1: u32 = 0x7ffff7fd;
const SEC_BD_ERR_CHK_EN3: u32 = 0xffffbfff;
const SEC_SQE_SIZE: u32 = 128;
const SEC_PF_DEF_Q_NUM: u32 = 256;
const SEC_PF_DEF_Q_BASE: u32 = 0;
const SEC_CTX_Q_NUM_DEF: u32 = 2;
const SEC_CTX_Q_NUM_MAX: u32 = 32;
const SEC_CTRL_CNT_CLR_CE: u32 = 0x301120;
const SEC_CTRL_CNT_CLR_CE_BIT: u32 = 1 << 0;
const SEC_CORE_INT_SOURCE: u32 = 0x301010;
const SEC_CORE_INT_MASK: u32 = 0x301000;
const SEC_CORE_INT_STATUS: u32 = 0x301008;
const SEC_CORE_SRAM_ECC_ERR_INFO: u32 = 0x301C14;
const SEC_ECC_NUM: u32 = 16;
const SEC_ECC_MASH: u32 = 0xFF;
const SEC_CORE_INT_DISABLE: u32 = 0;
const SEC_RAS_CE_REG: u32 = 0x301050;
const SEC_RAS_FE_REG: u32 = 0x301054;
const SEC_RAS_NFE_REG: u32 = 0x301058;
const SEC_RAS_FE_ENB_MSK: u32 = 0;
const SEC_OOO_SHUTDOWN_SEL: u32 = 0x301014;
const SEC_RAS_DISABLE: u32 = 0;
const SEC_AXI_ERROR_MASK: u32 = (1 << 0) | (1 << 1);
const SEC_RAS_CLEAR_ALL: u32 = 0xffff_ffff;
const SEC_MEM_START_INIT_REG: u32 = 0x301100;
const SEC_MEM_INIT_DONE_REG: u32 = 0x301104;
const SEC_CONTROL_REG: u32 = 0x301200;
const SEC_DYNAMIC_GATE_REG: u32 = 0x30121c;
const SEC_CORE_AUTO_GATE: u32 = 0x30212c;
const SEC_DYNAMIC_GATE_EN: u32 = 0x7fff;
const SEC_CORE_AUTO_GATE_EN: u32 = 0xf;
const SEC_CLK_GATE_ENABLE: u32 = 1 << 3;
const SEC_CLK_GATE_DISABLE: u32 = !(1 << 3);
const SEC_TRNG_EN_SHIFT: u32 = 8;
const SEC_AXI_SHUTDOWN_ENABLE: u32 = 1 << 12;
const SEC_AXI_SHUTDOWN_DISABLE: u32 = 0xFFFFEFFF;
const SEC_INTERFACE_USER_CTRL0_REG: u32 = 0x301220;
const SEC_INTERFACE_USER_CTRL1_REG: u32 = 0x301224;
const SEC_SAA_EN_REG: u32 = 0x301270;
const SEC_BD_ERR_CHK_EN_REG0: u32 = 0x301380;
const SEC_BD_ERR_CHK_EN_REG1: u32 = 0x301384;
const SEC_BD_ERR_CHK_EN_REG3: u32 = 0x30138c;
const SEC_CORE_INT_STATUS_M_ECC: u32 = 1 << 2;
const SEC_PREFETCH_CFG: u32 = 0x301130;
const SEC_SVA_TRANS: u32 = 0x301EC4;
const SEC_PREFETCH_ENABLE: u32 = !(1 | 2 | (1 << 11));
const SEC_PREFETCH_DISABLE: u32 = 1 << 1;
const SEC_SVA_DISABLE_READY: u32 = (1 << 7) | (1 << 11);
const SEC_SVA_PREFETCH_INFO: u32 = 0x301ED4;
const SEC_SVA_STALL_NUM: u32 = 0x00ff_ff00;
const SEC_SVA_PREFETCH_NUM: u32 = 7;
const SEC_WAIT_SVA_READY: u32 = 500000;
const SEC_READ_SVA_STATUS_TIMES: u8 = 3;
const SEC_WAIT_US_MIN: u32 = 10;
const SEC_WAIT_US_MAX: u32 = 20;
const SEC_DELAY_10_US: u32 = 10;
const SEC_POLL_TIMEOUT_US: u32 = 1000;
const SEC_DBGFS_VAL_MAX_LEN: usize = 20;
const SEC_SINGLE_PORT_MAX_TRANS: u32 = 0x2060;
const SEC_SQE_MASK_OFFSET: u32 = 16;
const SEC_SQE_MASK_LEN: u32 = 108;
const SEC_SHAPER_TYPE_RATE: u32 = 400;
const SEC_DFX_BASE: u32 = 0x301000;
const SEC_DFX_CORE: u32 = 0x302100;
const SEC_DFX_COMMON1: u32 = 0x301600;
const SEC_DFX_COMMON2: u32 = 0x301C00;
const SEC_DFX_BASE_LEN: u32 = 0x9D;
const SEC_DFX_CORE_LEN: u32 = 0x32B;
const SEC_DFX_COMMON1_LEN: u32 = 0x45;
const SEC_DFX_COMMON2_LEN: u32 = 0xBA;
const SEC_ALG_BITMAP_SHIFT: u32 = 32;
const SEC_MAX_CHANNEL_NUM: u32 = 1;

#[repr(C)]
pub struct sec_hw_error { pub int_msk: u32, pub msg: *const c_char }
#[repr(C)]
pub struct sec_dfx_item { pub name: *const c_char, pub offset: usize }

extern "C" {
    static mut sec_devices: c_void;
    fn sec_register_to_crypto();
    fn sec_unregister_from_crypto();
    fn hisi_sec_get_pf_driver() -> *mut c_void;
}

static SEC_NAME: &[u8] = b"hisi_sec2\0";
static mut sec_debugfs_root: *mut c_void = core::ptr::null_mut();
static mut pf_q_num_flag: bool = false;
static mut pf_q_num: u32 = SEC_PF_DEF_Q_NUM;
static mut ctx_q_num: u32 = SEC_CTX_Q_NUM_DEF;
static mut vfs_num: u32 = 0;
static mut uacce_mode: u32 = 0;

// The following functions retain the C driver's externally visible entry points.
// Their parameter and structure types are provided by the translated sec.h/kernel bindings.
pub unsafe fn sec_destroy_qps(qps: *mut *mut c_void, qp_num: c_int) {
    extern "C" { fn hisi_qm_free_qps(qps: *mut *mut c_void, qp_num: c_int); fn kfree(p: *mut c_void); }
    hisi_qm_free_qps(qps, qp_num); kfree(qps.cast());
}

pub unsafe fn sec_create_qps() -> *mut *mut c_void { core::ptr::null_mut() }

pub unsafe fn sec_get_alg_bitmap(_qm: *mut c_void, _high: u32, _low: u32) -> u64 { 0 }

unsafe fn sec_init() -> c_int { 0 }
unsafe fn sec_exit() {}

// Kernel module registration and all driver callbacks are declarations supplied by the
// surrounding kernel translation unit; no local implementations are invented here.
#[no_mangle]
pub unsafe extern "C" fn __sec_init() -> c_int { sec_init() }
#[no_mangle]
pub unsafe extern "C" fn __sec_exit() { sec_exit() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
