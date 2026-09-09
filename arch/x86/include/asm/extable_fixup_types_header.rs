/* SPDX-License-Identifier: GPL-2.0 */

pub const EX_DATA_TYPE_MASK: u32 = 0x000000FF;
pub const EX_DATA_REG_MASK: u32 = 0x00000F00;
pub const EX_DATA_FLAG_MASK: u32 = 0x0000F000;
pub const EX_DATA_IMM_MASK: u32 = 0xFFFF0000;

pub const EX_DATA_REG_SHIFT: u32 = 8;
pub const EX_DATA_FLAG_SHIFT: u32 = 12;
pub const EX_DATA_IMM_SHIFT: u32 = 16;

#[inline]
pub const fn EX_DATA_REG(reg: u32) -> u32 {
    reg << EX_DATA_REG_SHIFT
}

#[inline]
pub const fn EX_DATA_FLAG(flag: u32) -> u32 {
    flag << EX_DATA_FLAG_SHIFT
}

#[inline]
pub const fn EX_DATA_IMM(imm: i32) -> u32 {
    (imm as u32) << EX_DATA_IMM_SHIFT
}

/* segment regs */
pub const EX_REG_DS: u32 = EX_DATA_REG(8);
pub const EX_REG_ES: u32 = EX_DATA_REG(9);
pub const EX_REG_FS: u32 = EX_DATA_REG(10);
pub const EX_REG_GS: u32 = EX_DATA_REG(11);

/* flags */
pub const EX_FLAG_CLEAR_AX: u32 = EX_DATA_FLAG(1);
pub const EX_FLAG_CLEAR_DX: u32 = EX_DATA_FLAG(2);
pub const EX_FLAG_CLEAR_AX_DX: u32 = EX_DATA_FLAG(3);

/* types */
pub const EX_TYPE_NONE: u32 = 0;
pub const EX_TYPE_DEFAULT: u32 = 1;
pub const EX_TYPE_FAULT: u32 = 2;
pub const EX_TYPE_UACCESS: u32 = 3;
/* unused, was: #define EX_TYPE_COPY 4 */
pub const EX_TYPE_CLEAR_FS: u32 = 5;
pub const EX_TYPE_FPU_RESTORE: u32 = 6;
pub const EX_TYPE_BPF: u32 = 7;
pub const EX_TYPE_WRMSR: u32 = 8;
pub const EX_TYPE_RDMSR: u32 = 9;
pub const EX_TYPE_WRMSR_SAFE: u32 = 10; /* reg := -EIO */
pub const EX_TYPE_RDMSR_SAFE: u32 = 11; /* reg := -EIO */
pub const EX_TYPE_WRMSR_IN_MCE: u32 = 12;
pub const EX_TYPE_RDMSR_IN_MCE: u32 = 13;
pub const EX_TYPE_DEFAULT_MCE_SAFE: u32 = 14;
pub const EX_TYPE_FAULT_MCE_SAFE: u32 = 15;

pub const EX_TYPE_POP_REG: u32 = 16; /* sp += sizeof(long) */
pub const EX_TYPE_POP_ZERO: u32 = EX_TYPE_POP_REG | EX_DATA_IMM(0);

pub const EX_TYPE_IMM_REG: u32 = 17; /* reg := (long)imm */
pub const EX_TYPE_EFAULT_REG: u32 = EX_TYPE_IMM_REG | EX_DATA_IMM(-EFAULT);
pub const EX_TYPE_ZERO_REG: u32 = EX_TYPE_IMM_REG | EX_DATA_IMM(0);
pub const EX_TYPE_ONE_REG: u32 = EX_TYPE_IMM_REG | EX_DATA_IMM(1);

pub const EX_TYPE_FAULT_SGX: u32 = 18;

pub const EX_TYPE_UCOPY_LEN: u32 = 19; /* cx := reg + imm*cx */
pub const EX_TYPE_UCOPY_LEN1: u32 = EX_TYPE_UCOPY_LEN | EX_DATA_IMM(1);
pub const EX_TYPE_UCOPY_LEN4: u32 = EX_TYPE_UCOPY_LEN | EX_DATA_IMM(4);
pub const EX_TYPE_UCOPY_LEN8: u32 = EX_TYPE_UCOPY_LEN | EX_DATA_IMM(8);

pub const EX_TYPE_ZEROPAD: u32 = 20; /* longword load with zeropad on fault */

pub const EX_TYPE_ERETU: u32 = 21;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
