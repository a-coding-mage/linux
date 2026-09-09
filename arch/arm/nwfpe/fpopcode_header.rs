/* SPDX-License-Identifier: GPL-2.0-or-later */
/* NetWinder Floating Point Emulator instruction opcode definitions. */

/*
 * The instruction-class diagrams and opcode tables from the C header are
 * retained here as documentation; the values below are their direct masks.
 */

/* bit masks */
pub const BIT_PREINDEX: u32 = 0x01000000;
pub const BIT_UP: u32 = 0x00800000;
pub const BIT_WRITE_BACK: u32 = 0x00200000;
pub const BIT_LOAD: u32 = 0x00100000;

/* masks for load/store */
pub const MASK_CPDT: u32 = 0x0c000000;
pub const MASK_OFFSET: u32 = 0x000000ff;
pub const MASK_TRANSFER_LENGTH: u32 = 0x00408000;
pub const MASK_REGISTER_COUNT: u32 = MASK_TRANSFER_LENGTH;
pub const MASK_COPROCESSOR: u32 = 0x00000f00;

/* Tests for transfer length */
pub const TRANSFER_SINGLE: u32 = 0x00000000;
pub const TRANSFER_DOUBLE: u32 = 0x00008000;
pub const TRANSFER_EXTENDED: u32 = 0x00400000;
pub const TRANSFER_PACKED: u32 = MASK_TRANSFER_LENGTH;

pub const fn getCoprocessorNumber(opcode: u32) -> u32 { (opcode & MASK_COPROCESSOR) >> 8 }
pub const fn getOffset(opcode: u32) -> u32 { opcode & MASK_OFFSET }
pub const fn TEST_OPCODE(opcode: u32, mask: u32) -> bool { (opcode & mask) == mask }
pub const fn LOAD_OP(opcode: u32) -> bool { TEST_OPCODE(opcode, MASK_CPDT | BIT_LOAD) }
pub const fn STORE_OP(opcode: u32) -> bool { (opcode & (MASK_CPDT | BIT_LOAD)) == MASK_CPDT }
pub const fn LDF_OP(opcode: u32) -> bool { LOAD_OP(opcode) && getCoprocessorNumber(opcode) == 1 }
pub const fn LFM_OP(opcode: u32) -> bool { LOAD_OP(opcode) && getCoprocessorNumber(opcode) == 2 }
pub const fn STF_OP(opcode: u32) -> bool { STORE_OP(opcode) && getCoprocessorNumber(opcode) == 1 }
pub const fn SFM_OP(opcode: u32) -> bool { STORE_OP(opcode) && getCoprocessorNumber(opcode) == 2 }
pub const fn PREINDEXED(opcode: u32) -> bool { opcode & BIT_PREINDEX != 0 }
pub const fn POSTINDEXED(opcode: u32) -> bool { opcode & BIT_PREINDEX == 0 }
pub const fn BIT_UP_SET(opcode: u32) -> bool { opcode & BIT_UP != 0 }
/* BIT_DOWN is supplied by the surrounding translation, as in the C source. */
pub const fn BIT_UP_CLEAR(opcode: u32) -> bool { opcode & BIT_DOWN == 0 }
pub const fn WRITE_BACK(opcode: u32) -> bool { opcode & BIT_WRITE_BACK != 0 }
pub const fn LOAD(opcode: u32) -> bool { opcode & BIT_LOAD != 0 }
pub const fn STORE(opcode: u32) -> bool { opcode & BIT_LOAD == 0 }

/* arithmetic instruction masks and opcodes */
pub const BIT_MONADIC: u32 = 0x00008000;
pub const BIT_CONSTANT: u32 = 0x00000008;
pub const fn CONSTANT_FM(opcode: u32) -> bool { opcode & BIT_CONSTANT != 0 }
pub const fn MONADIC_INSTRUCTION(opcode: u32) -> bool { opcode & BIT_MONADIC != 0 }
pub const MASK_CPDO: u32 = 0x0e000000;
pub const MASK_ARITHMETIC_OPCODE: u32 = 0x00f08000;
pub const MASK_DESTINATION_SIZE: u32 = 0x00080080;

pub const ADF_CODE: u32 = 0x00000000; pub const MUF_CODE: u32 = 0x00100000;
pub const SUF_CODE: u32 = 0x00200000; pub const RSF_CODE: u32 = 0x00300000;
pub const DVF_CODE: u32 = 0x00400000; pub const RDF_CODE: u32 = 0x00500000;
pub const POW_CODE: u32 = 0x00600000; pub const RPW_CODE: u32 = 0x00700000;
pub const RMF_CODE: u32 = 0x00800000; pub const FML_CODE: u32 = 0x00900000;
pub const FDV_CODE: u32 = 0x00a00000; pub const FRD_CODE: u32 = 0x00b00000;
pub const POL_CODE: u32 = 0x00c00000;

pub const MVF_CODE: u32 = 0x00008000; pub const MNF_CODE: u32 = 0x00108000;
pub const ABS_CODE: u32 = 0x00208000; pub const RND_CODE: u32 = 0x00308000;
pub const SQT_CODE: u32 = 0x00408000; pub const LOG_CODE: u32 = 0x00508000;
pub const LGN_CODE: u32 = 0x00608000; pub const EXP_CODE: u32 = 0x00708000;
pub const SIN_CODE: u32 = 0x00808000; pub const COS_CODE: u32 = 0x00908000;
pub const TAN_CODE: u32 = 0x00a08000; pub const ASN_CODE: u32 = 0x00b08000;
pub const ACS_CODE: u32 = 0x00c08000; pub const ATN_CODE: u32 = 0x00d08000;
pub const URD_CODE: u32 = 0x00e08000; pub const NRM_CODE: u32 = 0x00f08000;

pub const MASK_CPRT: u32 = 0x0e000010;
pub const MASK_CPRT_CODE: u32 = 0x00f00000;
pub const FLT_CODE: u32 = 0x00000000; pub const FIX_CODE: u32 = 0x00100000;
pub const WFS_CODE: u32 = 0x00200000; pub const RFS_CODE: u32 = 0x00300000;
pub const WFC_CODE: u32 = 0x00400000; pub const RFC_CODE: u32 = 0x00500000;
pub const CMF_CODE: u32 = 0x00900000; pub const CNF_CODE: u32 = 0x00b00000;
pub const CMFE_CODE: u32 = 0x00d00000; pub const CNFE_CODE: u32 = 0x00f00000;

pub const MASK_Rd: u32 = 0x0000f000; pub const MASK_Rn: u32 = 0x000f0000;
pub const MASK_Fd: u32 = 0x00007000; pub const MASK_Fm: u32 = 0x00000007;
pub const MASK_Fn: u32 = 0x00070000;
pub const CC_MASK: u32 = 0xf0000000; pub const CC_NEGATIVE: u32 = 0x80000000;
pub const CC_ZERO: u32 = 0x40000000; pub const CC_CARRY: u32 = 0x20000000;
pub const CC_OVERFLOW: u32 = 0x10000000; pub const CC_EQ: u32 = 0x00000000;
pub const CC_NE: u32 = 0x10000000; pub const CC_CS: u32 = 0x20000000; pub const CC_HS: u32 = CC_CS;
pub const CC_CC: u32 = 0x30000000; pub const CC_LO: u32 = CC_CC; pub const CC_MI: u32 = 0x40000000;
pub const CC_PL: u32 = 0x50000000; pub const CC_VS: u32 = 0x60000000; pub const CC_VC: u32 = 0x70000000;
pub const CC_HI: u32 = 0x80000000; pub const CC_LS: u32 = 0x90000000; pub const CC_GE: u32 = 0xa0000000;
pub const CC_LT: u32 = 0xb0000000; pub const CC_GT: u32 = 0xc0000000; pub const CC_LE: u32 = 0xd0000000;
pub const CC_AL: u32 = 0xe0000000; pub const CC_NV: u32 = 0xf0000000;

pub const MASK_ROUNDING_MODE: u32 = 0x00000060;
pub const ROUND_TO_NEAREST: u32 = 0x00000000; pub const ROUND_TO_PLUS_INFINITY: u32 = 0x00000020;
pub const ROUND_TO_MINUS_INFINITY: u32 = 0x00000040; pub const ROUND_TO_ZERO: u32 = 0x00000060;
pub const MASK_ROUNDING_PRECISION: u32 = 0x00080080;
pub const ROUND_SINGLE: u32 = 0x00000000; pub const ROUND_DOUBLE: u32 = 0x00000080; pub const ROUND_EXTENDED: u32 = 0x00080000;

pub const fn getCondition(opcode: u32) -> u32 { opcode >> 28 }
pub const fn getRn(opcode: u32) -> u32 { (opcode & MASK_Rn) >> 16 }
pub const fn getFd(opcode: u32) -> u32 { (opcode & MASK_Fd) >> 12 }
pub const fn getFn(opcode: u32) -> u32 { (opcode & MASK_Fn) >> 16 }
pub const fn getFm(opcode: u32) -> u32 { opcode & MASK_Fm }
pub const fn getRd(opcode: u32) -> u32 { (opcode & MASK_Rd) >> 12 }
pub const fn getRoundingMode(opcode: u32) -> u32 { (opcode & MASK_ROUNDING_MODE) >> 5 }

/* External floating-point types, constants, and type tags are supplied by the surrounding translation. */
extern "C" {
    #[cfg(CONFIG_FPE_NWFPE_XP)]
    pub static floatx80Constant: [floatx80; 0];
    pub static float64Constant: [float64; 0];
    pub static float32Constant: [float32; 0];
}

#[cfg(CONFIG_FPE_NWFPE_XP)]
#[inline]
pub unsafe fn getExtendedConstant(nIndex: u32) -> floatx80 { floatx80Constant[nIndex as usize] }

#[inline]
pub unsafe fn getDoubleConstant(nIndex: u32) -> float64 { float64Constant[nIndex as usize] }

#[inline]
pub unsafe fn getSingleConstant(nIndex: u32) -> float32 { float32Constant[nIndex as usize] }

#[inline]
pub const fn getTransferLength(opcode: u32) -> u32 {
    match opcode & MASK_TRANSFER_LENGTH {
        0x00000000 => 1,
        0x00008000 => 2,
        0x00400000 => 3,
        _ => 0,
    }
}

#[inline]
pub const fn getRegisterCount(opcode: u32) -> u32 {
    match opcode & MASK_REGISTER_COUNT {
        0x00000000 => 4,
        0x00008000 => 1,
        0x00400000 => 2,
        0x00408000 => 3,
        _ => 0,
    }
}

#[inline]
pub const fn getRoundingPrecision(opcode: u32) -> u32 {
    match opcode & MASK_ROUNDING_PRECISION {
        0x00000000 => 1,
        0x00000080 => 2,
        0x00080000 => 3,
        _ => 0,
    }
}

#[inline]
pub const fn getDestinationSize(opcode: u32) -> u32 {
    match opcode & MASK_DESTINATION_SIZE {
        0x00000000 => typeSingle,
        0x00000080 => typeDouble,
        0x00080000 => typeExtended,
        _ => typeNone,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
