// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2018-2019 HiSilicon Limited. */
// Direct low-level translation of hpre_main.c. Kernel-provided types and
// functions are intentionally left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

const CAP_FILE_PERMISSION: u32 = 0o444;
const HPRE_CTRL_CNT_CLR_CE_BIT: u32 = 1 << 0;
const HPRE_CTRL_CNT_CLR_CE: usize = 0x301000;
const HPRE_FSM_MAX_CNT: usize = 0x301008;
const HPRE_VFG_AXQOS: usize = 0x30100c;
const HPRE_VFG_AXCACHE: usize = 0x301010;
const HPRE_RDCHN_INI_CFG: usize = 0x301014;
const HPRE_AWUSR_FP_CFG: usize = 0x301018;
const HPRE_BD_ENDIAN: usize = 0x301020;
const HPRE_ECC_BYPASS: usize = 0x301024;
const HPRE_RAS_WIDTH_CFG: usize = 0x301028;
const HPRE_POISON_BYPASS: usize = 0x30102c;
const HPRE_BD_ARUSR_CFG: usize = 0x301030;
const HPRE_BD_AWUSR_CFG: usize = 0x301034;
const HPRE_TYPES_ENB: usize = 0x301038;
const HPRE_RSA_ENB: u32 = 1 << 0;
const HPRE_ECC_ENB: u32 = 1 << 1;
const HPRE_DATA_RUSER_CFG: usize = 0x30103c;
const HPRE_DATA_WUSER_CFG: usize = 0x301040;
const HPRE_INT_MASK: usize = 0x301400;
const HPRE_INT_STATUS: usize = 0x301800;
const HPRE_HAC_INT_MSK: usize = 0x301400;
const HPRE_HAC_RAS_CE_ENB: usize = 0x301410;
const HPRE_HAC_RAS_NFE_ENB: usize = 0x301414;
const HPRE_HAC_RAS_FE_ENB: usize = 0x301418;
const HPRE_HAC_INT_SET: usize = 0x301500;
const HPRE_AXI_ERROR_MASK: u32 = ((1 << (21 - 10 + 1)) - 1) << 10;
const HPRE_RNG_TIMEOUT_NUM: usize = 0x301A34;
const HPRE_RDCHN_INI_ST: usize = 0x301a00;
const HPRE_CLSTR_BASE: usize = 0x302000;
const HPRE_CORE_EN_OFFSET: usize = 0x04;
const HPRE_CORE_INI_CFG_OFFSET: usize = 0x20;
const HPRE_CORE_INI_STATUS_OFFSET: usize = 0x80;
const HPRE_CORE_HTBT_WARN_OFFSET: usize = 0x8c;
const HPRE_CORE_IS_SCHD_OFFSET: usize = 0x90;
const HPRE_RAS_CE_ENB: usize = 0x301410;
const HPRE_RAS_NFE_ENB: usize = 0x301414;
const HPRE_RAS_FE_ENB: usize = 0x301418;
const HPRE_OOO_SHUTDOWN_SEL: usize = 0x301a3c;
const HPRE_RAS_MASK_ALL: u32 = u32::MAX;
const HPRE_RAS_CLEAR_ALL: u32 = u32::MAX;
const HPRE_HAC_ECC1_CNT: usize = 0x301a04;
const HPRE_HAC_ECC2_CNT: usize = 0x301a08;
const HPRE_HAC_SOURCE_INT: usize = 0x301600;
const HPRE_CLSTR_ADDR_INTRVL: usize = 0x1000;
const HPRE_CLUSTER_INQURY: usize = 0x100;
const HPRE_CLSTR_ADDR_INQRY_RSLT: usize = 0x104;
const HPRE_PASID_EN_BIT: u32 = 9;
const HPRE_REG_RD_INTVRL_US: u32 = 10;
const HPRE_REG_RD_TMOUT_US: u32 = 1000;
const HPRE_DBGFS_VAL_MAX_LEN: usize = 20;
const PCI_DEVICE_ID_HUAWEI_HPRE_PF: u16 = 0xa258;
const HPRE_QM_USR_CFG_MASK: u32 = u32::MAX & !1;
const HPRE_QM_AXI_CFG_MASK: u32 = 0xffff;
const HPRE_QM_VFG_AX_MASK: u32 = 0xff;
const HPRE_BD_USR_MASK: u32 = 3;
const HPRE_PREFETCH_CFG: usize = 0x301130;
const HPRE_SVA_PREFTCH_DFX: usize = 0x30115C;
const HPRE_PREFETCH_ENABLE: u32 = !(1 | (1 << 30));
const HPRE_PREFETCH_DISABLE: u32 = 1 << 30;
const HPRE_SVA_DISABLE_READY: u32 = (1 << 4) | (1 << 8);
const HPRE_SVA_PREFTCH_DFX4: usize = 0x301144;
const HPRE_WAIT_SVA_READY: u32 = 500000;
const HPRE_READ_SVA_STATUS_TIMES: u8 = 3;
const HPRE_WAIT_US_MIN: u32 = 10;
const HPRE_WAIT_US_MAX: u32 = 20;
const HPRE_CLKGATE_CTL: usize = 0x301a10;
const HPRE_PEH_CFG_AUTO_GATE: usize = 0x301a2c;
const HPRE_CLUSTER_DYN_CTL: usize = 0x302010;
const HPRE_CORE_SHB_CFG: usize = 0x302088;
const HPRE_CLKGATE_CTL_EN: u32 = 1;
const HPRE_PEH_CFG_AUTO_GATE_EN: u32 = 1;
const HPRE_CLUSTER_DYN_CTL_EN: u32 = 1;
const HPRE_CORE_GATE_EN: u32 = (1 << 30) | (1 << 31);
const HPRE_AM_OOO_SHUTDOWN_ENB: usize = 0x301044;
const HPRE_AM_OOO_SHUTDOWN_ENABLE: u32 = 1;
const HPRE_WR_MSI_PORT: u32 = 1 << 2;
const HPRE_CORE_ECC_2BIT_ERR: u32 = 1 << 1;
const HPRE_OOO_ECC_2BIT_ERR: u32 = 1 << 5;
const HPRE_QM_BME_FLR: u32 = 1 << 7;
const HPRE_QM_PM_FLR: u32 = 1 << 11;
const HPRE_QM_SRIOV_FLR: u32 = 1 << 12;
const HPRE_SHAPER_TYPE_RATE: u32 = 640;
const HPRE_VIA_MSI_DSM: u32 = 1;
const HPRE_SQE_MASK_OFFSET: u32 = 8;
const HPRE_SQE_MASK_LEN: u32 = 44;
const HPRE_CTX_Q_NUM_DEF: u32 = 1;
const HPRE_MAX_CHANNEL_NUM: usize = 2;

static HPRE_NAME: &[u8] = b"hisi_hpre\0";

#[repr(C)]
pub struct hpre_hw_error { pub int_msk: u32, pub msg: *const i8 }

pub const HPRE_RSA: &[u8] = b"rsa\n\0";
pub const HPRE_DH: &[u8] = b"dh\n\0";
pub const HPRE_ECDH: &[u8] = b"ecdh\n\0";
pub const HPRE_ECDSA: &[u8] = b"ecdsa\n\0";
pub const HPRE_SM2: &[u8] = b"sm2\n\0";
pub const HPRE_X25519: &[u8] = b"x25519\n\0";
pub const HPRE_X448: &[u8] = b"x448\n\0";

extern "C" {
    fn hpre_algs_register();
    fn hpre_algs_unregister();
}

// The remaining definitions retain the C driver's externally supplied kernel
// structures and callbacks verbatim in interface form.
extern "C" {
    pub fn hpre_create_qp(type_: u8) -> *mut core::ffi::c_void;
    pub fn hisi_hpre_get_pf_driver() -> *mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
