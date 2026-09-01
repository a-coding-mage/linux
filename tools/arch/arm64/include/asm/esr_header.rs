/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2013 - ARM Ltd
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 */

/* Dependency intent from C header: #include <asm/sysreg.h> */
/* Dependency intent from C header: #include <asm/types.h> when not __ASSEMBLER__ */

pub const ESR_ELx_EC_UNKNOWN: u64 = 0x00;
pub const ESR_ELx_EC_WFx: u64 = 0x01;
/* Unallocated EC: 0x02 */
pub const ESR_ELx_EC_CP15_32: u64 = 0x03;
pub const ESR_ELx_EC_CP15_64: u64 = 0x04;
pub const ESR_ELx_EC_CP14_MR: u64 = 0x05;
pub const ESR_ELx_EC_CP14_LS: u64 = 0x06;
pub const ESR_ELx_EC_FP_ASIMD: u64 = 0x07;
pub const ESR_ELx_EC_CP10_ID: u64 = 0x08; /* EL2 only */
pub const ESR_ELx_EC_PAC: u64 = 0x09; /* EL2 and above */
/* Unallocated EC: 0x0A - 0x0B */
pub const ESR_ELx_EC_CP14_64: u64 = 0x0C;
pub const ESR_ELx_EC_BTI: u64 = 0x0D;
pub const ESR_ELx_EC_ILL: u64 = 0x0E;
/* Unallocated EC: 0x0F - 0x10 */
pub const ESR_ELx_EC_SVC32: u64 = 0x11;
pub const ESR_ELx_EC_HVC32: u64 = 0x12; /* EL2 only */
pub const ESR_ELx_EC_SMC32: u64 = 0x13; /* EL2 and above */
/* Unallocated EC: 0x14 */
pub const ESR_ELx_EC_SVC64: u64 = 0x15;
pub const ESR_ELx_EC_HVC64: u64 = 0x16; /* EL2 and above */
pub const ESR_ELx_EC_SMC64: u64 = 0x17; /* EL2 and above */
pub const ESR_ELx_EC_SYS64: u64 = 0x18;
pub const ESR_ELx_EC_SVE: u64 = 0x19;
pub const ESR_ELx_EC_ERET: u64 = 0x1a; /* EL2 only */
/* Unallocated EC: 0x1B */
pub const ESR_ELx_EC_FPAC: u64 = 0x1C; /* EL1 and above */
pub const ESR_ELx_EC_SME: u64 = 0x1D;
/* Unallocated EC: 0x1E */
pub const ESR_ELx_EC_IMP_DEF: u64 = 0x1f; /* EL3 only */
pub const ESR_ELx_EC_IABT_LOW: u64 = 0x20;
pub const ESR_ELx_EC_IABT_CUR: u64 = 0x21;
pub const ESR_ELx_EC_PC_ALIGN: u64 = 0x22;
/* Unallocated EC: 0x23 */
pub const ESR_ELx_EC_DABT_LOW: u64 = 0x24;
pub const ESR_ELx_EC_DABT_CUR: u64 = 0x25;
pub const ESR_ELx_EC_SP_ALIGN: u64 = 0x26;
pub const ESR_ELx_EC_MOPS: u64 = 0x27;
pub const ESR_ELx_EC_FP_EXC32: u64 = 0x28;
/* Unallocated EC: 0x29 - 0x2B */
pub const ESR_ELx_EC_FP_EXC64: u64 = 0x2C;
/* Unallocated EC: 0x2D - 0x2E */
pub const ESR_ELx_EC_SERROR: u64 = 0x2F;
pub const ESR_ELx_EC_BREAKPT_LOW: u64 = 0x30;
pub const ESR_ELx_EC_BREAKPT_CUR: u64 = 0x31;
pub const ESR_ELx_EC_SOFTSTP_LOW: u64 = 0x32;
pub const ESR_ELx_EC_SOFTSTP_CUR: u64 = 0x33;
pub const ESR_ELx_EC_WATCHPT_LOW: u64 = 0x34;
pub const ESR_ELx_EC_WATCHPT_CUR: u64 = 0x35;
/* Unallocated EC: 0x36 - 0x37 */
pub const ESR_ELx_EC_BKPT32: u64 = 0x38;
/* Unallocated EC: 0x39 */
pub const ESR_ELx_EC_VECTOR32: u64 = 0x3A; /* EL2 only */
/* Unallocated EC: 0x3B */
pub const ESR_ELx_EC_BRK64: u64 = 0x3C;
/* Unallocated EC: 0x3D - 0x3F */
pub const ESR_ELx_EC_MAX: u64 = 0x3F;

pub const ESR_ELx_EC_SHIFT: u64 = 26;
pub const ESR_ELx_EC_WIDTH: u64 = 6;
pub const ESR_ELx_EC_MASK: u64 = 0x3F << ESR_ELx_EC_SHIFT;
pub const fn ESR_ELx_EC(esr: u64) -> u64 {
    (esr & ESR_ELx_EC_MASK) >> ESR_ELx_EC_SHIFT
}

pub const ESR_ELx_IL_SHIFT: u64 = 25;
pub const ESR_ELx_IL: u64 = 1 << ESR_ELx_IL_SHIFT;
pub const ESR_ELx_ISS_MASK: u64 = (1u64 << 25) - 1;
pub const fn ESR_ELx_ISS(esr: u64) -> u64 {
    esr & ESR_ELx_ISS_MASK
}
pub const ESR_ELx_ISS2_SHIFT: u64 = 32;
pub const ESR_ELx_ISS2_MASK: u64 = ((1u64 << (55 - 32 + 1)) - 1) << 32;
pub const fn ESR_ELx_ISS2(esr: u64) -> u64 {
    (esr & ESR_ELx_ISS2_MASK) >> ESR_ELx_ISS2_SHIFT
}

/* ISS field definitions shared by different classes */
pub const ESR_ELx_WNR_SHIFT: u64 = 6;
pub const ESR_ELx_WNR: u64 = 1 << ESR_ELx_WNR_SHIFT;

/* Asynchronous Error Type */
pub const ESR_ELx_IDS_SHIFT: u64 = 24;
pub const ESR_ELx_IDS: u64 = 1 << ESR_ELx_IDS_SHIFT;
pub const ESR_ELx_AET_SHIFT: u64 = 10;
pub const ESR_ELx_AET: u64 = 0x7 << ESR_ELx_AET_SHIFT;

pub const ESR_ELx_AET_UC: u64 = 0 << ESR_ELx_AET_SHIFT;
pub const ESR_ELx_AET_UEU: u64 = 1 << ESR_ELx_AET_SHIFT;
pub const ESR_ELx_AET_UEO: u64 = 2 << ESR_ELx_AET_SHIFT;
pub const ESR_ELx_AET_UER: u64 = 3 << ESR_ELx_AET_SHIFT;
pub const ESR_ELx_AET_CE: u64 = 6 << ESR_ELx_AET_SHIFT;

/* Shared ISS field definitions for Data/Instruction aborts */
pub const ESR_ELx_SET_SHIFT: u64 = 11;
pub const ESR_ELx_SET_MASK: u64 = 3 << ESR_ELx_SET_SHIFT;
pub const ESR_ELx_FnV_SHIFT: u64 = 10;
pub const ESR_ELx_FnV: u64 = 1 << ESR_ELx_FnV_SHIFT;
pub const ESR_ELx_EA_SHIFT: u64 = 9;
pub const ESR_ELx_EA: u64 = 1 << ESR_ELx_EA_SHIFT;
pub const ESR_ELx_S1PTW_SHIFT: u64 = 7;
pub const ESR_ELx_S1PTW: u64 = 1 << ESR_ELx_S1PTW_SHIFT;

/* Shared ISS fault status code(IFSC/DFSC) for Data/Instruction aborts */
pub const ESR_ELx_FSC: u64 = 0x3F;
pub const ESR_ELx_FSC_TYPE: u64 = 0x3C;
pub const ESR_ELx_FSC_LEVEL: u64 = 0x03;
pub const ESR_ELx_FSC_EXTABT: u64 = 0x10;
pub const ESR_ELx_FSC_MTE: u64 = 0x11;
pub const ESR_ELx_FSC_SERROR: u64 = 0x11;
pub const ESR_ELx_FSC_ACCESS: u64 = 0x08;
pub const ESR_ELx_FSC_FAULT: u64 = 0x04;
pub const ESR_ELx_FSC_PERM: u64 = 0x0C;
pub const fn ESR_ELx_FSC_SEA_TTW(n: i64) -> u64 {
    (0x14i64 + n) as u64
}
pub const ESR_ELx_FSC_SECC: u64 = 0x18;
pub const fn ESR_ELx_FSC_SECC_TTW(n: i64) -> u64 {
    (0x1ci64 + n) as u64
}

/* Status codes for individual page table levels */
pub const fn ESR_ELx_FSC_ACCESS_L(n: i64) -> u64 {
    (ESR_ELx_FSC_ACCESS as i64 + n) as u64
}
pub const fn ESR_ELx_FSC_PERM_L(n: i64) -> u64 {
    (ESR_ELx_FSC_PERM as i64 + n) as u64
}

pub const ESR_ELx_FSC_FAULT_nL: u64 = 0x2C;
pub const fn ESR_ELx_FSC_FAULT_L(n: i64) -> u64 {
    ((if n < 0 {
        ESR_ELx_FSC_FAULT_nL
    } else {
        ESR_ELx_FSC_FAULT
    }) as i64
        + n) as u64
}

/* ISS field definitions for Data Aborts */
pub const ESR_ELx_ISV_SHIFT: u64 = 24;
pub const ESR_ELx_ISV: u64 = 1 << ESR_ELx_ISV_SHIFT;
pub const ESR_ELx_SAS_SHIFT: u64 = 22;
pub const ESR_ELx_SAS: u64 = 3 << ESR_ELx_SAS_SHIFT;
pub const ESR_ELx_SSE_SHIFT: u64 = 21;
pub const ESR_ELx_SSE: u64 = 1 << ESR_ELx_SSE_SHIFT;
pub const ESR_ELx_SRT_SHIFT: u64 = 16;
pub const ESR_ELx_SRT_MASK: u64 = 0x1F << ESR_ELx_SRT_SHIFT;
pub const ESR_ELx_SF_SHIFT: u64 = 15;
pub const ESR_ELx_SF: u64 = 1 << ESR_ELx_SF_SHIFT;
pub const ESR_ELx_AR_SHIFT: u64 = 14;
pub const ESR_ELx_AR: u64 = 1 << ESR_ELx_AR_SHIFT;
pub const ESR_ELx_VNCR_SHIFT: u64 = 13;
pub const ESR_ELx_VNCR: u64 = 1 << ESR_ELx_VNCR_SHIFT;
pub const ESR_ELx_CM_SHIFT: u64 = 8;
pub const ESR_ELx_CM: u64 = 1 << ESR_ELx_CM_SHIFT;

/* ISS2 field definitions for Data Aborts */
pub const ESR_ELx_TnD_SHIFT: u64 = 10;
pub const ESR_ELx_TnD: u64 = 1 << ESR_ELx_TnD_SHIFT;
pub const ESR_ELx_TagAccess_SHIFT: u64 = 9;
pub const ESR_ELx_TagAccess: u64 = 1 << ESR_ELx_TagAccess_SHIFT;
pub const ESR_ELx_GCS_SHIFT: u64 = 8;
pub const ESR_ELx_GCS: u64 = 1 << ESR_ELx_GCS_SHIFT;
pub const ESR_ELx_Overlay_SHIFT: u64 = 6;
pub const ESR_ELx_Overlay: u64 = 1 << ESR_ELx_Overlay_SHIFT;
pub const ESR_ELx_DirtyBit_SHIFT: u64 = 5;
pub const ESR_ELx_DirtyBit: u64 = 1 << ESR_ELx_DirtyBit_SHIFT;
pub const ESR_ELx_Xs_SHIFT: u64 = 0;
pub const ESR_ELx_Xs_MASK: u64 = (1u64 << 5) - 1;

/* ISS field definitions for exceptions taken in to Hyp */
pub const ESR_ELx_FSC_ADDRSZ: u64 = 0x00;
pub const fn ESR_ELx_FSC_ADDRSZ_L(n: i64) -> u64 {
    (ESR_ELx_FSC_ADDRSZ as i64 + n) as u64
}
pub const ESR_ELx_CV: u64 = 1 << 24;
pub const ESR_ELx_COND_SHIFT: u64 = 20;
pub const ESR_ELx_COND_MASK: u64 = 0xF << ESR_ELx_COND_SHIFT;
pub const ESR_ELx_WFx_ISS_RN: u64 = 0x1F << 5;
pub const ESR_ELx_WFx_ISS_RV: u64 = 1 << 2;
pub const ESR_ELx_WFx_ISS_TI: u64 = 3 << 0;
pub const ESR_ELx_WFx_ISS_WFxT: u64 = 2 << 0;
pub const ESR_ELx_WFx_ISS_WFI: u64 = 0 << 0;
pub const ESR_ELx_WFx_ISS_WFE: u64 = 1 << 0;
pub const ESR_ELx_xVC_IMM_MASK: u64 = (1 << 16) - 1;

pub const DISR_EL1_IDS: u64 = 1 << 24;
/*
 * DISR_EL1 and ESR_ELx share the bottom 13 bits, but the RES0 bits may mean
 * different things in the future...
 */
pub const DISR_EL1_ESR_MASK: u64 = ESR_ELx_AET | ESR_ELx_EA | ESR_ELx_FSC;

/* ESR value templates for specific events */
pub const ESR_ELx_WFx_MASK: u64 = ESR_ELx_EC_MASK | (ESR_ELx_WFx_ISS_TI & !ESR_ELx_WFx_ISS_WFxT);
pub const ESR_ELx_WFx_WFI_VAL: u64 = (ESR_ELx_EC_WFx << ESR_ELx_EC_SHIFT) | ESR_ELx_WFx_ISS_WFI;

/* BRK instruction trap from AArch64 state */
pub const ESR_ELx_BRK64_ISS_COMMENT_MASK: u64 = 0xffff;

/* ISS field definitions for System instruction traps */
pub const ESR_ELx_SYS64_ISS_RES0_SHIFT: u64 = 22;
pub const ESR_ELx_SYS64_ISS_RES0_MASK: u64 = 0x7 << ESR_ELx_SYS64_ISS_RES0_SHIFT;
pub const ESR_ELx_SYS64_ISS_DIR_MASK: u64 = 0x1;
pub const ESR_ELx_SYS64_ISS_DIR_READ: u64 = 0x1;
pub const ESR_ELx_SYS64_ISS_DIR_WRITE: u64 = 0x0;

pub const ESR_ELx_SYS64_ISS_RT_SHIFT: u64 = 5;
pub const ESR_ELx_SYS64_ISS_RT_MASK: u64 = 0x1f << ESR_ELx_SYS64_ISS_RT_SHIFT;
pub const ESR_ELx_SYS64_ISS_CRM_SHIFT: u64 = 1;
pub const ESR_ELx_SYS64_ISS_CRM_MASK: u64 = 0xf << ESR_ELx_SYS64_ISS_CRM_SHIFT;
pub const ESR_ELx_SYS64_ISS_CRN_SHIFT: u64 = 10;
pub const ESR_ELx_SYS64_ISS_CRN_MASK: u64 = 0xf << ESR_ELx_SYS64_ISS_CRN_SHIFT;
pub const ESR_ELx_SYS64_ISS_OP1_SHIFT: u64 = 14;
pub const ESR_ELx_SYS64_ISS_OP1_MASK: u64 = 0x7 << ESR_ELx_SYS64_ISS_OP1_SHIFT;
pub const ESR_ELx_SYS64_ISS_OP2_SHIFT: u64 = 17;
pub const ESR_ELx_SYS64_ISS_OP2_MASK: u64 = 0x7 << ESR_ELx_SYS64_ISS_OP2_SHIFT;
pub const ESR_ELx_SYS64_ISS_OP0_SHIFT: u64 = 20;
pub const ESR_ELx_SYS64_ISS_OP0_MASK: u64 = 0x3 << ESR_ELx_SYS64_ISS_OP0_SHIFT;
pub const ESR_ELx_SYS64_ISS_SYS_MASK: u64 = ESR_ELx_SYS64_ISS_OP0_MASK
    | ESR_ELx_SYS64_ISS_OP1_MASK
    | ESR_ELx_SYS64_ISS_OP2_MASK
    | ESR_ELx_SYS64_ISS_CRN_MASK
    | ESR_ELx_SYS64_ISS_CRM_MASK;
pub const fn ESR_ELx_SYS64_ISS_SYS_VAL(op0: u64, op1: u64, op2: u64, crn: u64, crm: u64) -> u64 {
    (op0 << ESR_ELx_SYS64_ISS_OP0_SHIFT)
        | (op1 << ESR_ELx_SYS64_ISS_OP1_SHIFT)
        | (op2 << ESR_ELx_SYS64_ISS_OP2_SHIFT)
        | (crn << ESR_ELx_SYS64_ISS_CRN_SHIFT)
        | (crm << ESR_ELx_SYS64_ISS_CRM_SHIFT)
}

pub const ESR_ELx_SYS64_ISS_SYS_OP_MASK: u64 = ESR_ELx_SYS64_ISS_SYS_MASK | ESR_ELx_SYS64_ISS_DIR_MASK;
pub const fn ESR_ELx_SYS64_ISS_RT(esr: u64) -> u64 {
    (esr & ESR_ELx_SYS64_ISS_RT_MASK) >> ESR_ELx_SYS64_ISS_RT_SHIFT
}
/*
 * User space cache operations have the following sysreg encoding
 * in System instructions.
 * op0=1, op1=3, op2=1, crn=7, crm={ 5, 10, 11, 12, 13, 14 }, WRITE (L=0)
 */
pub const ESR_ELx_SYS64_ISS_CRM_DC_CIVAC: u64 = 14;
pub const ESR_ELx_SYS64_ISS_CRM_DC_CVADP: u64 = 13;
pub const ESR_ELx_SYS64_ISS_CRM_DC_CVAP: u64 = 12;
pub const ESR_ELx_SYS64_ISS_CRM_DC_CVAU: u64 = 11;
pub const ESR_ELx_SYS64_ISS_CRM_DC_CVAC: u64 = 10;
pub const ESR_ELx_SYS64_ISS_CRM_IC_IVAU: u64 = 5;

pub const ESR_ELx_SYS64_ISS_EL0_CACHE_OP_MASK: u64 = ESR_ELx_SYS64_ISS_OP0_MASK
    | ESR_ELx_SYS64_ISS_OP1_MASK
    | ESR_ELx_SYS64_ISS_OP2_MASK
    | ESR_ELx_SYS64_ISS_CRN_MASK
    | ESR_ELx_SYS64_ISS_DIR_MASK;
pub const ESR_ELx_SYS64_ISS_EL0_CACHE_OP_VAL: u64 =
    ESR_ELx_SYS64_ISS_SYS_VAL(1, 3, 1, 7, 0) | ESR_ELx_SYS64_ISS_DIR_WRITE;
/*
 * User space MRS operations which are supported for emulation
 * have the following sysreg encoding in System instructions.
 * op0 = 3, op1= 0, crn = 0, {crm = 0, 4-7}, READ (L = 1)
 */
pub const ESR_ELx_SYS64_ISS_SYS_MRS_OP_MASK: u64 = ESR_ELx_SYS64_ISS_OP0_MASK
    | ESR_ELx_SYS64_ISS_OP1_MASK
    | ESR_ELx_SYS64_ISS_CRN_MASK
    | ESR_ELx_SYS64_ISS_DIR_MASK;
pub const ESR_ELx_SYS64_ISS_SYS_MRS_OP_VAL: u64 =
    ESR_ELx_SYS64_ISS_SYS_VAL(3, 0, 0, 0, 0) | ESR_ELx_SYS64_ISS_DIR_READ;

pub const ESR_ELx_SYS64_ISS_SYS_CTR: u64 = ESR_ELx_SYS64_ISS_SYS_VAL(3, 3, 1, 0, 0);
pub const ESR_ELx_SYS64_ISS_SYS_CTR_READ: u64 = ESR_ELx_SYS64_ISS_SYS_CTR | ESR_ELx_SYS64_ISS_DIR_READ;

pub const ESR_ELx_SYS64_ISS_SYS_CNTVCT: u64 =
    ESR_ELx_SYS64_ISS_SYS_VAL(3, 3, 2, 14, 0) | ESR_ELx_SYS64_ISS_DIR_READ;

pub const ESR_ELx_SYS64_ISS_SYS_CNTVCTSS: u64 =
    ESR_ELx_SYS64_ISS_SYS_VAL(3, 3, 6, 14, 0) | ESR_ELx_SYS64_ISS_DIR_READ;

pub const ESR_ELx_SYS64_ISS_SYS_CNTFRQ: u64 =
    ESR_ELx_SYS64_ISS_SYS_VAL(3, 3, 0, 14, 0) | ESR_ELx_SYS64_ISS_DIR_READ;

pub fn esr_sys64_to_sysreg(e: u64) -> u32 {
    sys_reg(
        (e & ESR_ELx_SYS64_ISS_OP0_MASK) >> ESR_ELx_SYS64_ISS_OP0_SHIFT,
        (e & ESR_ELx_SYS64_ISS_OP1_MASK) >> ESR_ELx_SYS64_ISS_OP1_SHIFT,
        (e & ESR_ELx_SYS64_ISS_CRN_MASK) >> ESR_ELx_SYS64_ISS_CRN_SHIFT,
        (e & ESR_ELx_SYS64_ISS_CRM_MASK) >> ESR_ELx_SYS64_ISS_CRM_SHIFT,
        (e & ESR_ELx_SYS64_ISS_OP2_MASK) >> ESR_ELx_SYS64_ISS_OP2_SHIFT,
    )
}

pub fn esr_cp15_to_sysreg(e: u64) -> u32 {
    sys_reg(
        3,
        (e & ESR_ELx_SYS64_ISS_OP1_MASK) >> ESR_ELx_SYS64_ISS_OP1_SHIFT,
        (e & ESR_ELx_SYS64_ISS_CRN_MASK) >> ESR_ELx_SYS64_ISS_CRN_SHIFT,
        (e & ESR_ELx_SYS64_ISS_CRM_MASK) >> ESR_ELx_SYS64_ISS_CRM_SHIFT,
        (e & ESR_ELx_SYS64_ISS_OP2_MASK) >> ESR_ELx_SYS64_ISS_OP2_SHIFT,
    )
}

/* ISS field definitions for ERET/ERETAA/ERETAB trapping */
pub const ESR_ELx_ERET_ISS_ERET: u64 = 0x2;
pub const ESR_ELx_ERET_ISS_ERETA: u64 = 0x1;

/*
 * ISS field definitions for floating-point exception traps
 * (FP_EXC_32/FP_EXC_64).
 *
 * (The FPEXC_* constants are used instead for common bits.)
 */

pub const ESR_ELx_FP_EXC_TFV: u64 = 1 << 23;

/*
 * ISS field definitions for CP15 accesses
 */
pub const ESR_ELx_CP15_32_ISS_DIR_MASK: u64 = 0x1;
pub const ESR_ELx_CP15_32_ISS_DIR_READ: u64 = 0x1;
pub const ESR_ELx_CP15_32_ISS_DIR_WRITE: u64 = 0x0;

pub const ESR_ELx_CP15_32_ISS_RT_SHIFT: u64 = 5;
pub const ESR_ELx_CP15_32_ISS_RT_MASK: u64 = 0x1f << ESR_ELx_CP15_32_ISS_RT_SHIFT;
pub const ESR_ELx_CP15_32_ISS_CRM_SHIFT: u64 = 1;
pub const ESR_ELx_CP15_32_ISS_CRM_MASK: u64 = 0xf << ESR_ELx_CP15_32_ISS_CRM_SHIFT;
pub const ESR_ELx_CP15_32_ISS_CRN_SHIFT: u64 = 10;
pub const ESR_ELx_CP15_32_ISS_CRN_MASK: u64 = 0xf << ESR_ELx_CP15_32_ISS_CRN_SHIFT;
pub const ESR_ELx_CP15_32_ISS_OP1_SHIFT: u64 = 14;
pub const ESR_ELx_CP15_32_ISS_OP1_MASK: u64 = 0x7 << ESR_ELx_CP15_32_ISS_OP1_SHIFT;
pub const ESR_ELx_CP15_32_ISS_OP2_SHIFT: u64 = 17;
pub const ESR_ELx_CP15_32_ISS_OP2_MASK: u64 = 0x7 << ESR_ELx_CP15_32_ISS_OP2_SHIFT;

pub const ESR_ELx_CP15_32_ISS_SYS_MASK: u64 = ESR_ELx_CP15_32_ISS_OP1_MASK
    | ESR_ELx_CP15_32_ISS_OP2_MASK
    | ESR_ELx_CP15_32_ISS_CRN_MASK
    | ESR_ELx_CP15_32_ISS_CRM_MASK
    | ESR_ELx_CP15_32_ISS_DIR_MASK;
pub const fn ESR_ELx_CP15_32_ISS_SYS_VAL(op1: u64, op2: u64, crn: u64, crm: u64) -> u64 {
    (op1 << ESR_ELx_CP15_32_ISS_OP1_SHIFT)
        | (op2 << ESR_ELx_CP15_32_ISS_OP2_SHIFT)
        | (crn << ESR_ELx_CP15_32_ISS_CRN_SHIFT)
        | (crm << ESR_ELx_CP15_32_ISS_CRM_SHIFT)
}

pub const ESR_ELx_CP15_64_ISS_DIR_MASK: u64 = 0x1;
pub const ESR_ELx_CP15_64_ISS_DIR_READ: u64 = 0x1;
pub const ESR_ELx_CP15_64_ISS_DIR_WRITE: u64 = 0x0;

pub const ESR_ELx_CP15_64_ISS_RT_SHIFT: u64 = 5;
pub const ESR_ELx_CP15_64_ISS_RT_MASK: u64 = 0x1f << ESR_ELx_CP15_64_ISS_RT_SHIFT;

pub const ESR_ELx_CP15_64_ISS_RT2_SHIFT: u64 = 10;
pub const ESR_ELx_CP15_64_ISS_RT2_MASK: u64 = 0x1f << ESR_ELx_CP15_64_ISS_RT2_SHIFT;

pub const ESR_ELx_CP15_64_ISS_OP1_SHIFT: u64 = 16;
pub const ESR_ELx_CP15_64_ISS_OP1_MASK: u64 = 0xf << ESR_ELx_CP15_64_ISS_OP1_SHIFT;
pub const ESR_ELx_CP15_64_ISS_CRM_SHIFT: u64 = 1;
pub const ESR_ELx_CP15_64_ISS_CRM_MASK: u64 = 0xf << ESR_ELx_CP15_64_ISS_CRM_SHIFT;

pub const fn ESR_ELx_CP15_64_ISS_SYS_VAL(op1: u64, crm: u64) -> u64 {
    (op1 << ESR_ELx_CP15_64_ISS_OP1_SHIFT) | (crm << ESR_ELx_CP15_64_ISS_CRM_SHIFT)
}

pub const ESR_ELx_CP15_64_ISS_SYS_MASK: u64 =
    ESR_ELx_CP15_64_ISS_OP1_MASK | ESR_ELx_CP15_64_ISS_CRM_MASK | ESR_ELx_CP15_64_ISS_DIR_MASK;

pub const ESR_ELx_CP15_64_ISS_SYS_CNTVCT: u64 =
    ESR_ELx_CP15_64_ISS_SYS_VAL(1, 14) | ESR_ELx_CP15_64_ISS_DIR_READ;

pub const ESR_ELx_CP15_64_ISS_SYS_CNTVCTSS: u64 =
    ESR_ELx_CP15_64_ISS_SYS_VAL(9, 14) | ESR_ELx_CP15_64_ISS_DIR_READ;

pub const ESR_ELx_CP15_32_ISS_SYS_CNTFRQ: u64 =
    ESR_ELx_CP15_32_ISS_SYS_VAL(0, 0, 14, 0) | ESR_ELx_CP15_32_ISS_DIR_READ;

/*
 * ISS values for SME traps
 */

pub const ESR_ELx_SME_ISS_SME_DISABLED: u64 = 0;
pub const ESR_ELx_SME_ISS_ILL: u64 = 1;
pub const ESR_ELx_SME_ISS_SM_DISABLED: u64 = 2;
pub const ESR_ELx_SME_ISS_ZA_DISABLED: u64 = 3;
pub const ESR_ELx_SME_ISS_ZT_DISABLED: u64 = 4;

/* ISS field definitions for MOPS exceptions */
pub const ESR_ELx_MOPS_ISS_MEM_INST: u64 = 1 << 24;
pub const ESR_ELx_MOPS_ISS_FROM_EPILOGUE: u64 = 1 << 18;
pub const ESR_ELx_MOPS_ISS_WRONG_OPTION: u64 = 1 << 17;
pub const ESR_ELx_MOPS_ISS_OPTION_A: u64 = 1 << 16;
pub const fn ESR_ELx_MOPS_ISS_DESTREG(esr: u64) -> u64 {
    (esr & (0x1f << 10)) >> 10
}
pub const fn ESR_ELx_MOPS_ISS_SRCREG(esr: u64) -> u64 {
    (esr & (0x1f << 5)) >> 5
}
pub const fn ESR_ELx_MOPS_ISS_SIZEREG(esr: u64) -> u64 {
    (esr & (0x1f << 0)) >> 0
}

pub const fn esr_brk_comment(esr: u64) -> u64 {
    esr & ESR_ELx_BRK64_ISS_COMMENT_MASK
}

pub const fn esr_is_data_abort(esr: u64) -> bool {
    let ec = ESR_ELx_EC(esr);

    ec == ESR_ELx_EC_DABT_LOW || ec == ESR_ELx_EC_DABT_CUR
}

pub fn esr_is_cfi_brk(esr: u64) -> bool {
    ESR_ELx_EC(esr) == ESR_ELx_EC_BRK64 && (esr_brk_comment(esr) & !CFI_BRK_IMM_MASK) == CFI_BRK_IMM_BASE
}

pub const fn esr_fsc_is_translation_fault(mut esr: u64) -> bool {
    esr = esr & ESR_ELx_FSC;

    (esr == ESR_ELx_FSC_FAULT_L(3))
        || (esr == ESR_ELx_FSC_FAULT_L(2))
        || (esr == ESR_ELx_FSC_FAULT_L(1))
        || (esr == ESR_ELx_FSC_FAULT_L(0))
        || (esr == ESR_ELx_FSC_FAULT_L(-1))
}

pub const fn esr_fsc_is_permission_fault(mut esr: u64) -> bool {
    esr = esr & ESR_ELx_FSC;

    (esr == ESR_ELx_FSC_PERM_L(3))
        || (esr == ESR_ELx_FSC_PERM_L(2))
        || (esr == ESR_ELx_FSC_PERM_L(1))
        || (esr == ESR_ELx_FSC_PERM_L(0))
}

pub const fn esr_fsc_is_access_flag_fault(mut esr: u64) -> bool {
    esr = esr & ESR_ELx_FSC;

    (esr == ESR_ELx_FSC_ACCESS_L(3))
        || (esr == ESR_ELx_FSC_ACCESS_L(2))
        || (esr == ESR_ELx_FSC_ACCESS_L(1))
        || (esr == ESR_ELx_FSC_ACCESS_L(0))
}

/* Indicate whether ESR.EC==0x1A is for an ERETAx instruction */
pub const fn esr_iss_is_eretax(esr: u64) -> bool {
    (esr & ESR_ELx_ERET_ISS_ERET) != 0
}

/* Indicate which key is used for ERETAx (false: A-Key, true: B-Key) */
pub const fn esr_iss_is_eretab(esr: u64) -> bool {
    (esr & ESR_ELx_ERET_ISS_ERETA) != 0
}

extern "C" {
    pub fn esr_get_class_string(esr: u64) -> *const core::ffi::c_char;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
