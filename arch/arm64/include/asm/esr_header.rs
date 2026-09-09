/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of asm/esr.h. */

/* The included C headers provide UL, GENMASK, GENMASK_ULL and sys_reg. */

pub const ESR_ELx_EC_UNKNOWN: u64 = 0x00;
pub const ESR_ELx_EC_WFx: u64 = 0x01;
pub const ESR_ELx_EC_CP15_32: u64 = 0x03;
pub const ESR_ELx_EC_CP15_64: u64 = 0x04;
pub const ESR_ELx_EC_CP14_MR: u64 = 0x05;
pub const ESR_ELx_EC_CP14_LS: u64 = 0x06;
pub const ESR_ELx_EC_FP_ASIMD: u64 = 0x07;
pub const ESR_ELx_EC_CP10_ID: u64 = 0x08;
pub const ESR_ELx_EC_PAC: u64 = 0x09;
pub const ESR_ELx_EC_OTHER: u64 = 0x0a;
pub const ESR_ELx_EC_CP14_64: u64 = 0x0c;
pub const ESR_ELx_EC_BTI: u64 = 0x0d;
pub const ESR_ELx_EC_ILL: u64 = 0x0e;
pub const ESR_ELx_EC_SVC32: u64 = 0x11;
pub const ESR_ELx_EC_HVC32: u64 = 0x12;
pub const ESR_ELx_EC_SMC32: u64 = 0x13;
pub const ESR_ELx_EC_SVC64: u64 = 0x15;
pub const ESR_ELx_EC_HVC64: u64 = 0x16;
pub const ESR_ELx_EC_SMC64: u64 = 0x17;
pub const ESR_ELx_EC_SYS64: u64 = 0x18;
pub const ESR_ELx_EC_SVE: u64 = 0x19;
pub const ESR_ELx_EC_ERET: u64 = 0x1a;
pub const ESR_ELx_EC_FPAC: u64 = 0x1c;
pub const ESR_ELx_EC_SME: u64 = 0x1d;
pub const ESR_ELx_EC_IMP_DEF: u64 = 0x1f;
pub const ESR_ELx_EC_IABT_LOW: u64 = 0x20;
pub const ESR_ELx_EC_IABT_CUR: u64 = 0x21;
pub const ESR_ELx_EC_PC_ALIGN: u64 = 0x22;
pub const ESR_ELx_EC_DABT_LOW: u64 = 0x24;
pub const ESR_ELx_EC_DABT_CUR: u64 = 0x25;
pub const ESR_ELx_EC_SP_ALIGN: u64 = 0x26;
pub const ESR_ELx_EC_MOPS: u64 = 0x27;
pub const ESR_ELx_EC_FP_EXC32: u64 = 0x28;
pub const ESR_ELx_EC_FP_EXC64: u64 = 0x2c;
pub const ESR_ELx_EC_GCS: u64 = 0x2d;
pub const ESR_ELx_EC_SERROR: u64 = 0x2f;
pub const ESR_ELx_EC_BREAKPT_LOW: u64 = 0x30;
pub const ESR_ELx_EC_BREAKPT_CUR: u64 = 0x31;
pub const ESR_ELx_EC_SOFTSTP_LOW: u64 = 0x32;
pub const ESR_ELx_EC_SOFTSTP_CUR: u64 = 0x33;
pub const ESR_ELx_EC_WATCHPT_LOW: u64 = 0x34;
pub const ESR_ELx_EC_WATCHPT_CUR: u64 = 0x35;
pub const ESR_ELx_EC_BKPT32: u64 = 0x38;
pub const ESR_ELx_EC_VECTOR32: u64 = 0x3a;
pub const ESR_ELx_EC_BRK64: u64 = 0x3c;
pub const ESR_ELx_EC_MAX: u64 = 0x3f;

pub const ESR_ELx_EC_SHIFT: u64 = 26;
pub const ESR_ELx_EC_WIDTH: u64 = 6;
pub const ESR_ELx_EC_MASK: u64 = 0x3f << ESR_ELx_EC_SHIFT;
#[macro_export] macro_rules! ESR_ELx_EC { ($esr:expr) => { (($esr & $crate::ESR_ELx_EC_MASK) >> $crate::ESR_ELx_EC_SHIFT) }; }
pub const ESR_ELx_IL_SHIFT: u64 = 25;
pub const ESR_ELx_IL: u64 = 1 << ESR_ELx_IL_SHIFT;
pub const ESR_ELx_ISS_MASK: u64 = (1 << 25) - 1;
#[macro_export] macro_rules! ESR_ELx_ISS { ($esr:expr) => { $esr & $crate::ESR_ELx_ISS_MASK }; }
pub const ESR_ELx_ISS2_SHIFT: u64 = 32;
pub const ESR_ELx_ISS2_MASK: u64 = ((1u64 << 56) - 1) & !((1u64 << 32) - 1);
#[macro_export] macro_rules! ESR_ELx_ISS2 { ($esr:expr) => { (($esr & $crate::ESR_ELx_ISS2_MASK) >> $crate::ESR_ELx_ISS2_SHIFT) }; }

pub const ESR_ELx_WNR_SHIFT: u64 = 6;
pub const ESR_ELx_WNR: u64 = 1 << 6;
pub const ESR_ELx_IDS_SHIFT: u64 = 24;
pub const ESR_ELx_IDS: u64 = 1 << 24;
pub const ESR_ELx_AET_SHIFT: u64 = 10;
pub const ESR_ELx_AET: u64 = 7 << 10;
pub const ESR_ELx_AET_UC: u64 = 0 << 10;
pub const ESR_ELx_AET_UEU: u64 = 1 << 10;
pub const ESR_ELx_AET_UEO: u64 = 2 << 10;
pub const ESR_ELx_AET_UER: u64 = 3 << 10;
pub const ESR_ELx_AET_CE: u64 = 6 << 10;
pub const ESR_ELx_VNCR_SHIFT: u64 = 13;
pub const ESR_ELx_VNCR: u64 = 1 << 13;
pub const ESR_ELx_SET_SHIFT: u64 = 11;
pub const ESR_ELx_SET_MASK: u64 = 3 << 11;
pub const ESR_ELx_FnV_SHIFT: u64 = 10;
pub const ESR_ELx_FnV: u64 = 1 << 10;
pub const ESR_ELx_EA_SHIFT: u64 = 9;
pub const ESR_ELx_EA: u64 = 1 << 9;
pub const ESR_ELx_S1PTW_SHIFT: u64 = 7;
pub const ESR_ELx_S1PTW: u64 = 1 << 7;

pub const ESR_ELx_FSC: u64 = 0x3f;
pub const ESR_ELx_FSC_TYPE: u64 = 0x3c;
pub const ESR_ELx_FSC_LEVEL: u64 = 3;
pub const ESR_ELx_FSC_EXTABT: u64 = 0x10;
pub const ESR_ELx_FSC_MTE: u64 = 0x11;
pub const ESR_ELx_FSC_SERROR: u64 = 0x11;
pub const ESR_ELx_FSC_ACCESS: u64 = 8;
pub const ESR_ELx_FSC_FAULT: u64 = 4;
pub const ESR_ELx_FSC_PERM: u64 = 0xc;
#[macro_export] macro_rules! ESR_ELx_FSC_SEA_TTW { ($n:expr) => { 0x14 + $n }; }
pub const ESR_ELx_FSC_SECC: u64 = 0x18;
#[macro_export] macro_rules! ESR_ELx_FSC_SECC_TTW { ($n:expr) => { 0x1c + $n }; }
pub const ESR_ELx_FSC_EXCL_ATOMIC: u64 = 0x35;
pub const ESR_ELx_FSC_ADDRSZ: u64 = 0;
#[macro_export] macro_rules! ESR_ELx_FSC_ADDRSZ_nL { ($n:expr) => { if $n == -1 { 0x29 } else { 0x2c } }; }
#[macro_export] macro_rules! ESR_ELx_FSC_ADDRSZ_L { ($n:expr) => { if $n < 0 { ESR_ELx_FSC_ADDRSZ_nL!($n) } else { $crate::ESR_ELx_FSC_ADDRSZ + $n } }; }
#[macro_export] macro_rules! ESR_ELx_FSC_ACCESS_L { ($n:expr) => { $crate::ESR_ELx_FSC_ACCESS + $n }; }
#[macro_export] macro_rules! ESR_ELx_FSC_PERM_L { ($n:expr) => { $crate::ESR_ELx_FSC_PERM + $n }; }
pub const ESR_ELx_FSC_FAULT_nL: u64 = 0x2c;
#[macro_export] macro_rules! ESR_ELx_FSC_FAULT_L { ($n:expr) => { (if $n < 0 { $crate::ESR_ELx_FSC_FAULT_nL } else { $crate::ESR_ELx_FSC_FAULT }) + $n }; }

pub const ESR_ELx_ISV_SHIFT: u64 = 24;
pub const ESR_ELx_ISV: u64 = 1 << 24;
pub const ESR_ELx_SAS_SHIFT: u64 = 22;
pub const ESR_ELx_SAS: u64 = 3 << 22;
pub const ESR_ELx_SSE_SHIFT: u64 = 21;
pub const ESR_ELx_SSE: u64 = 1 << 21;
pub const ESR_ELx_SRT_SHIFT: u64 = 16;
pub const ESR_ELx_SRT_MASK: u64 = 0x1f << 16;
pub const ESR_ELx_SF_SHIFT: u64 = 15;
pub const ESR_ELx_SF: u64 = 1 << 15;
pub const ESR_ELx_AR_SHIFT: u64 = 14;
pub const ESR_ELx_AR: u64 = 1 << 14;
pub const ESR_ELx_CM_SHIFT: u64 = 8;
pub const ESR_ELx_CM: u64 = 1 << 8;
pub const ESR_ELx_HDBSSF_SHIFT: u64 = 11;
pub const ESR_ELx_HDBSSF: u64 = 1 << 11;
pub const ESR_ELx_TnD_SHIFT: u64 = 10;
pub const ESR_ELx_TnD: u64 = 1 << 10;
pub const ESR_ELx_TagAccess_SHIFT: u64 = 9;
pub const ESR_ELx_TagAccess: u64 = 1 << 9;
pub const ESR_ELx_GCS_SHIFT: u64 = 8;
pub const ESR_ELx_GCS: u64 = 1 << 8;
pub const ESR_ELx_Overlay_SHIFT: u64 = 6;
pub const ESR_ELx_Overlay: u64 = 1 << 6;
pub const ESR_ELx_DirtyBit_SHIFT: u64 = 5;
pub const ESR_ELx_DirtyBit: u64 = 1 << 5;
pub const ESR_ELx_Xs_SHIFT: u64 = 0;
pub const ESR_ELx_Xs_MASK: u64 = 0x1f;

pub const ESR_ELx_CV: u64 = 1 << 24;
pub const ESR_ELx_COND_SHIFT: u64 = 20;
pub const ESR_ELx_COND_MASK: u64 = 0xf << 20;
pub const ESR_ELx_WFx_ISS_RN: u64 = 0x1f << 5;
pub const ESR_ELx_WFx_ISS_RV: u64 = 1 << 2;
pub const ESR_ELx_WFx_ISS_TI: u64 = 3;
pub const ESR_ELx_WFx_ISS_WFxT: u64 = 2;
pub const ESR_ELx_WFx_ISS_WFI: u64 = 0;
pub const ESR_ELx_WFx_ISS_WFE: u64 = 1;
pub const ESR_ELx_xVC_IMM_MASK: u64 = 0xffff;
pub const ESR_ELx_ISS_OTHER_ST64BV: u64 = 0;
pub const ESR_ELx_ISS_OTHER_ST64BV0: u64 = 1;
pub const ESR_ELx_ISS_OTHER_LDST64B: u64 = 2;
pub const ESR_ELx_ISS_OTHER_TSBCSYNC: u64 = 3;
pub const ESR_ELx_ISS_OTHER_PSBCSYNC: u64 = 4;
pub const DISR_EL1_IDS: u64 = 1 << 24;
pub const DISR_EL1_ESR_MASK: u64 = ESR_ELx_AET | ESR_ELx_EA | ESR_ELx_FSC;
pub const ESR_ELx_WFx_MASK: u64 = ESR_ELx_EC_MASK | (ESR_ELx_WFx_ISS_TI & !ESR_ELx_WFx_ISS_WFxT);
pub const ESR_ELx_WFx_WFI_VAL: u64 = (ESR_ELx_EC_WFx << ESR_ELx_EC_SHIFT) | ESR_ELx_WFx_ISS_WFI;
pub const ESR_ELx_BRK64_ISS_COMMENT_MASK: u64 = 0xffff;

pub const ESR_ELx_SYS64_ISS_RES0_SHIFT: u64 = 22;
pub const ESR_ELx_SYS64_ISS_RES0_MASK: u64 = 7 << 22;
pub const ESR_ELx_SYS64_ISS_DIR_MASK: u64 = 1;
pub const ESR_ELx_SYS64_ISS_DIR_READ: u64 = 1;
pub const ESR_ELx_SYS64_ISS_DIR_WRITE: u64 = 0;
pub const ESR_ELx_SYS64_ISS_RT_SHIFT: u64 = 5;
pub const ESR_ELx_SYS64_ISS_RT_MASK: u64 = 0x1f << 5;
pub const ESR_ELx_SYS64_ISS_CRM_SHIFT: u64 = 1;
pub const ESR_ELx_SYS64_ISS_CRM_MASK: u64 = 0xf << 1;
pub const ESR_ELx_SYS64_ISS_CRN_SHIFT: u64 = 10;
pub const ESR_ELx_SYS64_ISS_CRN_MASK: u64 = 0xf << 10;
pub const ESR_ELx_SYS64_ISS_OP1_SHIFT: u64 = 14;
pub const ESR_ELx_SYS64_ISS_OP1_MASK: u64 = 7 << 14;
pub const ESR_ELx_SYS64_ISS_OP2_SHIFT: u64 = 17;
pub const ESR_ELx_SYS64_ISS_OP2_MASK: u64 = 7 << 17;
pub const ESR_ELx_SYS64_ISS_OP0_SHIFT: u64 = 20;
pub const ESR_ELx_SYS64_ISS_OP0_MASK: u64 = 3 << 20;
pub const ESR_ELx_SYS64_ISS_SYS_MASK: u64 = ESR_ELx_SYS64_ISS_OP0_MASK | ESR_ELx_SYS64_ISS_OP1_MASK | ESR_ELx_SYS64_ISS_OP2_MASK | ESR_ELx_SYS64_ISS_CRN_MASK | ESR_ELx_SYS64_ISS_CRM_MASK;
#[macro_export] macro_rules! ESR_ELx_SYS64_ISS_SYS_VAL { ($op0:expr,$op1:expr,$op2:expr,$crn:expr,$crm:expr) => { ($op0 << $crate::ESR_ELx_SYS64_ISS_OP0_SHIFT) | ($op1 << $crate::ESR_ELx_SYS64_ISS_OP1_SHIFT) | ($op2 << $crate::ESR_ELx_SYS64_ISS_OP2_SHIFT) | ($crn << $crate::ESR_ELx_SYS64_ISS_CRN_SHIFT) | ($crm << $crate::ESR_ELx_SYS64_ISS_CRM_SHIFT) }; }
pub const ESR_ELx_SYS64_ISS_SYS_OP_MASK: u64 = ESR_ELx_SYS64_ISS_SYS_MASK | ESR_ELx_SYS64_ISS_DIR_MASK;
#[macro_export] macro_rules! ESR_ELx_SYS64_ISS_RT { ($esr:expr) => { (($esr & $crate::ESR_ELx_SYS64_ISS_RT_MASK) >> $crate::ESR_ELx_SYS64_ISS_RT_SHIFT) }; }
pub const ESR_ELx_SYS64_ISS_CRM_DC_CIVAC: u64 = 14;
pub const ESR_ELx_SYS64_ISS_CRM_DC_CVADP: u64 = 13;
pub const ESR_ELx_SYS64_ISS_CRM_DC_CVAP: u64 = 12;
pub const ESR_ELx_SYS64_ISS_CRM_DC_CVAU: u64 = 11;
pub const ESR_ELx_SYS64_ISS_CRM_DC_CVAC: u64 = 10;
pub const ESR_ELx_SYS64_ISS_CRM_IC_IVAU: u64 = 5;
pub const ESR_ELx_SYS64_ISS_EL0_CACHE_OP_MASK: u64 = ESR_ELx_SYS64_ISS_OP0_MASK | ESR_ELx_SYS64_ISS_OP1_MASK | ESR_ELx_SYS64_ISS_OP2_MASK | ESR_ELx_SYS64_ISS_CRN_MASK | ESR_ELx_SYS64_ISS_DIR_MASK;
pub const ESR_ELx_SYS64_ISS_EL0_CACHE_OP_VAL: u64 = ESR_ELx_SYS64_ISS_SYS_VAL!(1,3,1,7,0) | ESR_ELx_SYS64_ISS_DIR_WRITE;
pub const ESR_ELx_SYS64_ISS_SYS_MRS_OP_MASK: u64 = ESR_ELx_SYS64_ISS_OP0_MASK | ESR_ELx_SYS64_ISS_OP1_MASK | ESR_ELx_SYS64_ISS_CRN_MASK | ESR_ELx_SYS64_ISS_DIR_MASK;
pub const ESR_ELx_SYS64_ISS_SYS_MRS_OP_VAL: u64 = ESR_ELx_SYS64_ISS_SYS_VAL!(3,0,0,0,0) | ESR_ELx_SYS64_ISS_DIR_READ;
pub const ESR_ELx_SYS64_ISS_SYS_CTR: u64 = ESR_ELx_SYS64_ISS_SYS_VAL!(3,3,1,0,0);
pub const ESR_ELx_SYS64_ISS_SYS_CTR_READ: u64 = ESR_ELx_SYS64_ISS_SYS_CTR | ESR_ELx_SYS64_ISS_DIR_READ;
pub const ESR_ELx_SYS64_ISS_SYS_CNTVCT: u64 = ESR_ELx_SYS64_ISS_SYS_VAL!(3,3,2,14,0) | 1;
pub const ESR_ELx_SYS64_ISS_SYS_CNTVCTSS: u64 = ESR_ELx_SYS64_ISS_SYS_VAL!(3,3,6,14,0) | 1;
pub const ESR_ELx_SYS64_ISS_SYS_CNTFRQ: u64 = ESR_ELx_SYS64_ISS_SYS_VAL!(3,3,0,14,0) | 1;
#[macro_export] macro_rules! esr_sys64_to_sysreg { ($e:expr) => { sys_reg!(($e & $crate::ESR_ELx_SYS64_ISS_OP0_MASK) >> $crate::ESR_ELx_SYS64_ISS_OP0_SHIFT, ($e & $crate::ESR_ELx_SYS64_ISS_OP1_MASK) >> $crate::ESR_ELx_SYS64_ISS_OP1_SHIFT, ($e & $crate::ESR_ELx_SYS64_ISS_CRN_MASK) >> $crate::ESR_ELx_SYS64_ISS_CRN_SHIFT, ($e & $crate::ESR_ELx_SYS64_ISS_CRM_MASK) >> $crate::ESR_ELx_SYS64_ISS_CRM_SHIFT, ($e & $crate::ESR_ELx_SYS64_ISS_OP2_MASK) >> $crate::ESR_ELx_SYS64_ISS_OP2_SHIFT) }; }
#[macro_export] macro_rules! esr_cp15_to_sysreg { ($e:expr) => { sys_reg!(3, ($e & $crate::ESR_ELx_SYS64_ISS_OP1_MASK) >> $crate::ESR_ELx_SYS64_ISS_OP1_SHIFT, ($e & $crate::ESR_ELx_SYS64_ISS_CRN_MASK) >> $crate::ESR_ELx_SYS64_ISS_CRN_SHIFT, ($e & $crate::ESR_ELx_SYS64_ISS_CRM_MASK) >> $crate::ESR_ELx_SYS64_ISS_CRM_SHIFT, ($e & $crate::ESR_ELx_SYS64_ISS_OP2_MASK) >> $crate::ESR_ELx_SYS64_ISS_OP2_SHIFT) }; }

pub const ESR_ELx_ERET_ISS_ERET: u64 = 2;
pub const ESR_ELx_ERET_ISS_ERETA: u64 = 1;
pub const ESR_ELx_FP_EXC_TFV: u64 = 1 << 23;

pub const ESR_ELx_CP15_32_ISS_DIR_MASK: u64 = 1;
pub const ESR_ELx_CP15_32_ISS_DIR_READ: u64 = 1;
pub const ESR_ELx_CP15_32_ISS_DIR_WRITE: u64 = 0;
pub const ESR_ELx_CP15_32_ISS_RT_SHIFT: u64 = 5;
pub const ESR_ELx_CP15_32_ISS_RT_MASK: u64 = 0x1f << 5;
pub const ESR_ELx_CP15_32_ISS_CRM_SHIFT: u64 = 1;
pub const ESR_ELx_CP15_32_ISS_CRM_MASK: u64 = 0xf << 1;
pub const ESR_ELx_CP15_32_ISS_CRN_SHIFT: u64 = 10;
pub const ESR_ELx_CP15_32_ISS_CRN_MASK: u64 = 0xf << 10;
pub const ESR_ELx_CP15_32_ISS_OP1_SHIFT: u64 = 14;
pub const ESR_ELx_CP15_32_ISS_OP1_MASK: u64 = 7 << 14;
pub const ESR_ELx_CP15_32_ISS_OP2_SHIFT: u64 = 17;
pub const ESR_ELx_CP15_32_ISS_OP2_MASK: u64 = 7 << 17;
pub const ESR_ELx_CP15_32_ISS_SYS_MASK: u64 = ESR_ELx_CP15_32_ISS_OP1_MASK | ESR_ELx_CP15_32_ISS_OP2_MASK | ESR_ELx_CP15_32_ISS_CRN_MASK | ESR_ELx_CP15_32_ISS_CRM_MASK | ESR_ELx_CP15_32_ISS_DIR_MASK;
#[macro_export] macro_rules! ESR_ELx_CP15_32_ISS_SYS_VAL { ($op1:expr,$op2:expr,$crn:expr,$crm:expr) => { ($op1 << $crate::ESR_ELx_CP15_32_ISS_OP1_SHIFT) | ($op2 << $crate::ESR_ELx_CP15_32_ISS_OP2_SHIFT) | ($crn << $crate::ESR_ELx_CP15_32_ISS_CRN_SHIFT) | ($crm << $crate::ESR_ELx_CP15_32_ISS_CRM_SHIFT) }; }
pub const ESR_ELx_CP15_64_ISS_DIR_MASK: u64 = 1;
pub const ESR_ELx_CP15_64_ISS_DIR_READ: u64 = 1;
pub const ESR_ELx_CP15_64_ISS_DIR_WRITE: u64 = 0;
pub const ESR_ELx_CP15_64_ISS_RT_SHIFT: u64 = 5;
pub const ESR_ELx_CP15_64_ISS_RT_MASK: u64 = 0x1f << 5;
pub const ESR_ELx_CP15_64_ISS_RT2_SHIFT: u64 = 10;
pub const ESR_ELx_CP15_64_ISS_RT2_MASK: u64 = 0x1f << 10;
pub const ESR_ELx_CP15_64_ISS_OP1_SHIFT: u64 = 16;
pub const ESR_ELx_CP15_64_ISS_OP1_MASK: u64 = 0xf << 16;
pub const ESR_ELx_CP15_64_ISS_CRM_SHIFT: u64 = 1;
pub const ESR_ELx_CP15_64_ISS_CRM_MASK: u64 = 0xf << 1;
#[macro_export] macro_rules! ESR_ELx_CP15_64_ISS_SYS_VAL { ($op1:expr,$crm:expr) => { ($op1 << $crate::ESR_ELx_CP15_64_ISS_OP1_SHIFT) | ($crm << $crate::ESR_ELx_CP15_64_ISS_CRM_SHIFT) }; }
pub const ESR_ELx_CP15_64_ISS_SYS_MASK: u64 = ESR_ELx_CP15_64_ISS_OP1_MASK | ESR_ELx_CP15_64_ISS_CRM_MASK | ESR_ELx_CP15_64_ISS_DIR_MASK;
pub const ESR_ELx_CP15_64_ISS_SYS_CNTVCT: u64 = ESR_ELx_CP15_64_ISS_SYS_VAL!(1,14) | 1;
pub const ESR_ELx_CP15_64_ISS_SYS_CNTVCTSS: u64 = ESR_ELx_CP15_64_ISS_SYS_VAL!(9,14) | 1;
pub const ESR_ELx_CP15_32_ISS_SYS_CNTFRQ: u64 = ESR_ELx_CP15_32_ISS_SYS_VAL!(0,0,14,0) | 1;

pub const ESR_ELx_SME_ISS_SMTC_MASK: u64 = 7;
#[macro_export] macro_rules! ESR_ELx_SME_ISS_SMTC { ($esr:expr) => { $esr & $crate::ESR_ELx_SME_ISS_SMTC_MASK }; }
pub const ESR_ELx_SME_ISS_SMTC_SME_DISABLED: u64 = 0;
pub const ESR_ELx_SME_ISS_SMTC_ILL: u64 = 1;
pub const ESR_ELx_SME_ISS_SMTC_SM_DISABLED: u64 = 2;
pub const ESR_ELx_SME_ISS_SMTC_ZA_DISABLED: u64 = 3;
pub const ESR_ELx_SME_ISS_SMTC_ZT_DISABLED: u64 = 4;
pub const ESR_ELx_MOPS_ISS_MEM_INST: u64 = 1 << 24;
pub const ESR_ELx_MOPS_ISS_FROM_EPILOGUE: u64 = 1 << 18;
pub const ESR_ELx_MOPS_ISS_WRONG_OPTION: u64 = 1 << 17;
pub const ESR_ELx_MOPS_ISS_OPTION_A: u64 = 1 << 16;
#[macro_export] macro_rules! ESR_ELx_MOPS_ISS_DESTREG { ($esr:expr) => { (($esr & (0x1f << 10)) >> 10) }; }
#[macro_export] macro_rules! ESR_ELx_MOPS_ISS_SRCREG { ($esr:expr) => { (($esr & (0x1f << 5)) >> 5) }; }
#[macro_export] macro_rules! ESR_ELx_MOPS_ISS_SIZEREG { ($esr:expr) => { $esr & 0x1f }; }
pub const ESR_ELx_ExType_SHIFT: u64 = 20;
pub const ESR_ELx_ExType_MASK: u64 = 0xf << 20;
pub const ESR_ELx_Raddr_SHIFT: u64 = 10;
pub const ESR_ELx_Raddr_MASK: u64 = 0x1f << 10;
pub const ESR_ELx_Rn_SHIFT: u64 = 5;
pub const ESR_ELx_Rn_MASK: u64 = 0x1f << 5;
pub const ESR_ELx_Rvalue_SHIFT: u64 = 5;
pub const ESR_ELx_Rvalue_MASK: u64 = 0x1f << 5;
pub const ESR_ELx_IT_SHIFT: u64 = 0;
pub const ESR_ELx_IT_MASK: u64 = 0x1f;
pub const ESR_ELx_ExType_DATA_CHECK: u64 = 0;
pub const ESR_ELx_ExType_EXLOCK: u64 = 1;
pub const ESR_ELx_ExType_STR: u64 = 2;
pub const ESR_ELx_IT_RET: u64 = 0;
pub const ESR_ELx_IT_GCSPOPM: u64 = 1;
pub const ESR_ELx_IT_RET_KEYA: u64 = 2;
pub const ESR_ELx_IT_RET_KEYB: u64 = 3;
pub const ESR_ELx_IT_GCSSS1: u64 = 4;
pub const ESR_ELx_IT_GCSSS2: u64 = 5;
pub const ESR_ELx_IT_GCSPOPCX: u64 = 6;
pub const ESR_ELx_IT_GCSPOPX: u64 = 7;

#[inline] pub fn esr_brk_comment(esr: u64) -> u64 { esr & ESR_ELx_BRK64_ISS_COMMENT_MASK }
#[inline] pub fn esr_is_data_abort(esr: u64) -> bool { let ec = ESR_ELx_EC!(esr); ec == ESR_ELx_EC_DABT_LOW || ec == ESR_ELx_EC_DABT_CUR }
#[inline] pub fn esr_is_cfi_brk(esr: u64) -> bool { ESR_ELx_EC!(esr) == ESR_ELx_EC_BRK64 && (esr_brk_comment(esr) & !CFI_BRK_IMM_MASK) == CFI_BRK_IMM_BASE }
#[inline] pub fn esr_is_ubsan_brk(esr: u64) -> bool { (esr_brk_comment(esr) & !UBSAN_BRK_MASK) == UBSAN_BRK_IMM }
#[inline] pub fn esr_fsc_is_translation_fault(esr: u64) -> bool { let e=esr&ESR_ELx_FSC; e==ESR_ELx_FSC_FAULT_L!(3)||e==ESR_ELx_FSC_FAULT_L!(2)||e==ESR_ELx_FSC_FAULT_L!(1)||e==ESR_ELx_FSC_FAULT_L!(0)||e==ESR_ELx_FSC_FAULT_L!(-1i64) }
#[inline] pub fn esr_fsc_is_permission_fault(esr: u64) -> bool { let e=esr&ESR_ELx_FSC; e==ESR_ELx_FSC_PERM_L!(3)||e==ESR_ELx_FSC_PERM_L!(2)||e==ESR_ELx_FSC_PERM_L!(1)||e==ESR_ELx_FSC_PERM_L!(0) }
#[inline] pub fn esr_fsc_is_access_flag_fault(esr: u64) -> bool { let e=esr&ESR_ELx_FSC; e==ESR_ELx_FSC_ACCESS_L!(3)||e==ESR_ELx_FSC_ACCESS_L!(2)||e==ESR_ELx_FSC_ACCESS_L!(1)||e==ESR_ELx_FSC_ACCESS_L!(0) }
#[inline] pub fn esr_fsc_is_excl_atomic_fault(esr: u64) -> bool { (esr&ESR_ELx_FSC)==ESR_ELx_FSC_EXCL_ATOMIC }
#[inline] pub fn esr_fsc_is_addr_sz_fault(esr: u64) -> bool { let e=esr&ESR_ELx_FSC; e==ESR_ELx_FSC_ADDRSZ_L!(3)||e==ESR_ELx_FSC_ADDRSZ_L!(2)||e==ESR_ELx_FSC_ADDRSZ_L!(1)||e==ESR_ELx_FSC_ADDRSZ_L!(0)||e==ESR_ELx_FSC_ADDRSZ_L!(-1i64) }
#[inline] pub fn esr_fsc_is_sea_ttw(esr: u64) -> bool { let e=esr&ESR_ELx_FSC; e==ESR_ELx_FSC_SEA_TTW!(3)||e==ESR_ELx_FSC_SEA_TTW!(2)||e==ESR_ELx_FSC_SEA_TTW!(1)||e==ESR_ELx_FSC_SEA_TTW!(0)||e==ESR_ELx_FSC_SEA_TTW!(-1i64) }
#[inline] pub fn esr_fsc_is_secc_ttw(esr: u64) -> bool { let e=esr&ESR_ELx_FSC; e==ESR_ELx_FSC_SECC_TTW!(3)||e==ESR_ELx_FSC_SECC_TTW!(2)||e==ESR_ELx_FSC_SECC_TTW!(1)||e==ESR_ELx_FSC_SECC_TTW!(0)||e==ESR_ELx_FSC_SECC_TTW!(-1i64) }
#[inline] pub fn esr_iss_is_eretax(esr: u64) -> bool { (esr & ESR_ELx_ERET_ISS_ERET) != 0 }
#[inline] pub fn esr_iss_is_eretab(esr: u64) -> bool { (esr & ESR_ELx_ERET_ISS_ERETA) != 0 }
extern "C" { pub fn esr_get_class_string(esr: u64) -> *const core::ffi::c_char; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
