// SPDX-License-Identifier: GPL-2.0

/* Per asm/virt.h */
pub const HVC_STUB_ERR: u32 = 0xbadca11;

/* Per asm/kvm_asm.h */
pub const ARM_EXCEPTION_IRQ: u32 = 0;
pub const ARM_EXCEPTION_EL1_SERROR: u32 = 1;
pub const ARM_EXCEPTION_TRAP: u32 = 2;
pub const ARM_EXCEPTION_IL: u32 = 3;
/* The hyp-stub will return this for any kvm_call_hyp() call */
pub const ARM_EXCEPTION_HYP_GONE: u32 = HVC_STUB_ERR;

pub const kvm_arm_exception_type: &[(u32, &str)] = &[
    (ARM_EXCEPTION_IRQ, "IRQ"),
    (ARM_EXCEPTION_EL1_SERROR, "SERROR"),
    (ARM_EXCEPTION_TRAP, "TRAP"),
    (ARM_EXCEPTION_IL, "ILLEGAL"),
    (ARM_EXCEPTION_HYP_GONE, "HYP_GONE"),
];

/* Per asm/esr.h */
pub const ESR_ELx_EC_UNKNOWN: u32 = 0x00;
pub const ESR_ELx_EC_WFx: u32 = 0x01;
/* Unallocated EC: 0x02 */
pub const ESR_ELx_EC_CP15_32: u32 = 0x03;
pub const ESR_ELx_EC_CP15_64: u32 = 0x04;
pub const ESR_ELx_EC_CP14_MR: u32 = 0x05;
pub const ESR_ELx_EC_CP14_LS: u32 = 0x06;
pub const ESR_ELx_EC_FP_ASIMD: u32 = 0x07;
pub const ESR_ELx_EC_CP10_ID: u32 = 0x08; /* EL2 only */
pub const ESR_ELx_EC_PAC: u32 = 0x09; /* EL2 and above */
pub const ESR_ELx_EC_OTHER: u32 = 0x0A;
/* Unallocated EC: 0x0B */
pub const ESR_ELx_EC_CP14_64: u32 = 0x0C;
pub const ESR_ELx_EC_BTI: u32 = 0x0D;
pub const ESR_ELx_EC_ILL: u32 = 0x0E;
/* Unallocated EC: 0x0F - 0x10 */
pub const ESR_ELx_EC_SVC32: u32 = 0x11;
pub const ESR_ELx_EC_HVC32: u32 = 0x12; /* EL2 only */
pub const ESR_ELx_EC_SMC32: u32 = 0x13; /* EL2 and above */
/* Unallocated EC: 0x14 */
pub const ESR_ELx_EC_SVC64: u32 = 0x15;
pub const ESR_ELx_EC_HVC64: u32 = 0x16; /* EL2 and above */
pub const ESR_ELx_EC_SMC64: u32 = 0x17; /* EL2 and above */
pub const ESR_ELx_EC_SYS64: u32 = 0x18;
pub const ESR_ELx_EC_SVE: u32 = 0x19;
pub const ESR_ELx_EC_ERET: u32 = 0x1a; /* EL2 only */
/* Unallocated EC: 0x1B */
pub const ESR_ELx_EC_FPAC: u32 = 0x1C; /* EL1 and above */
pub const ESR_ELx_EC_SME: u32 = 0x1D;
/* Unallocated EC: 0x1E */
pub const ESR_ELx_EC_IMP_DEF: u32 = 0x1f; /* EL3 only */
pub const ESR_ELx_EC_IABT_LOW: u32 = 0x20;
pub const ESR_ELx_EC_IABT_CUR: u32 = 0x21;
pub const ESR_ELx_EC_PC_ALIGN: u32 = 0x22;
/* Unallocated EC: 0x23 */
pub const ESR_ELx_EC_DABT_LOW: u32 = 0x24;
pub const ESR_ELx_EC_DABT_CUR: u32 = 0x25;
pub const ESR_ELx_EC_SP_ALIGN: u32 = 0x26;
pub const ESR_ELx_EC_MOPS: u32 = 0x27;
pub const ESR_ELx_EC_FP_EXC32: u32 = 0x28;
/* Unallocated EC: 0x29 - 0x2B */
pub const ESR_ELx_EC_FP_EXC64: u32 = 0x2C;
pub const ESR_ELx_EC_GCS: u32 = 0x2D;
/* Unallocated EC: 0x2E */
pub const ESR_ELx_EC_SERROR: u32 = 0x2F;
pub const ESR_ELx_EC_BREAKPT_LOW: u32 = 0x30;
pub const ESR_ELx_EC_BREAKPT_CUR: u32 = 0x31;
pub const ESR_ELx_EC_SOFTSTP_LOW: u32 = 0x32;
pub const ESR_ELx_EC_SOFTSTP_CUR: u32 = 0x33;
pub const ESR_ELx_EC_WATCHPT_LOW: u32 = 0x34;
pub const ESR_ELx_EC_WATCHPT_CUR: u32 = 0x35;
/* Unallocated EC: 0x36 - 0x37 */
pub const ESR_ELx_EC_BKPT32: u32 = 0x38;
/* Unallocated EC: 0x39 */
pub const ESR_ELx_EC_VECTOR32: u32 = 0x3A; /* EL2 only */
/* Unallocated EC: 0x3B */
pub const ESR_ELx_EC_BRK64: u32 = 0x3C;
/* Unallocated EC: 0x3D - 0x3F */
pub const ESR_ELx_EC_MAX: u32 = 0x3F;

pub const kvm_arm_exception_class: &[(u32, &str)] = &[
    (ESR_ELx_EC_UNKNOWN, "UNKNOWN"),
    (ESR_ELx_EC_WFx, "WFx"),
    (ESR_ELx_EC_CP15_32, "CP15_32"),
    (ESR_ELx_EC_CP15_64, "CP15_64"),
    (ESR_ELx_EC_CP14_MR, "CP14_MR"),
    (ESR_ELx_EC_CP14_LS, "CP14_LS"),
    (ESR_ELx_EC_FP_ASIMD, "FP_ASIMD"),
    (ESR_ELx_EC_CP10_ID, "CP10_ID"),
    (ESR_ELx_EC_PAC, "PAC"),
    (ESR_ELx_EC_CP14_64, "CP14_64"),
    (ESR_ELx_EC_SVC64, "SVC64"),
    (ESR_ELx_EC_HVC64, "HVC64"),
    (ESR_ELx_EC_SMC64, "SMC64"),
    (ESR_ELx_EC_SYS64, "SYS64"),
    (ESR_ELx_EC_SVE, "SVE"),
    (ESR_ELx_EC_IMP_DEF, "IMP_DEF"),
    (ESR_ELx_EC_IABT_LOW, "IABT_LOW"),
    (ESR_ELx_EC_IABT_CUR, "IABT_CUR"),
    (ESR_ELx_EC_PC_ALIGN, "PC_ALIGN"),
    (ESR_ELx_EC_DABT_LOW, "DABT_LOW"),
    (ESR_ELx_EC_DABT_CUR, "DABT_CUR"),
    (ESR_ELx_EC_SP_ALIGN, "SP_ALIGN"),
    (ESR_ELx_EC_FP_EXC32, "FP_EXC32"),
    (ESR_ELx_EC_FP_EXC64, "FP_EXC64"),
    (ESR_ELx_EC_SERROR, "SERROR"),
    (ESR_ELx_EC_BREAKPT_LOW, "BREAKPT_LOW"),
    (ESR_ELx_EC_BREAKPT_CUR, "BREAKPT_CUR"),
    (ESR_ELx_EC_SOFTSTP_LOW, "SOFTSTP_LOW"),
    (ESR_ELx_EC_SOFTSTP_CUR, "SOFTSTP_CUR"),
    (ESR_ELx_EC_WATCHPT_LOW, "WATCHPT_LOW"),
    (ESR_ELx_EC_WATCHPT_CUR, "WATCHPT_CUR"),
    (ESR_ELx_EC_BKPT32, "BKPT32"),
    (ESR_ELx_EC_VECTOR32, "VECTOR32"),
    (ESR_ELx_EC_BRK64, "BRK64"),
];
