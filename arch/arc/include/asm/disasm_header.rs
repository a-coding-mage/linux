/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * several functions that help interpret ARC instructions
 * used for unaligned accesses, kprobes and kgdb
 *
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

#[repr(i32)]
pub enum Opcode {
    op_Bcc = 0, op_BLcc = 1, op_LD = 2, op_ST = 3, op_MAJOR_4 = 4,
    op_MAJOR_5 = 5, op_LD_ADD = 12, op_ADD_SUB_SHIFT = 13,
    op_ADD_MOV_CMP = 14, op_S = 15, op_LD_S = 16, op_LDB_S = 17,
    op_LDW_S = 18, op_LDWX_S = 19, op_ST_S = 20, op_STB_S = 21,
    op_STW_S = 22, op_Su5 = 23, op_SP = 24, op_GP = 25,
    op_Pcl = 26, op_MOV_S = 27, op_ADD_CMP = 28, op_BR_S = 29,
    op_B_S = 30, op_BL_S = 31,
}

#[repr(i32)]
pub enum flow {
    noflow,
    direct_jump,
    direct_call,
    indirect_jump,
    indirect_call,
    invalid_instr,
}

#[inline]
pub fn IS_BIT(word: i32, n: i32) -> i32 { word & (1i32 << n) }

#[inline]
pub fn BITS(word: i32, s: i32, e: i32) -> i32 {
    (word >> s) & !((-2i32) << (e - s))
}

#[inline] pub fn MAJOR_OPCODE(word: i32) -> i32 { BITS(word, 27, 31) }
#[inline] pub fn MINOR_OPCODE(word: i32) -> i32 { BITS(word, 16, 21) }
#[inline] pub fn FIELD_A(word: i32) -> i32 { BITS(word, 0, 5) }
#[inline] pub fn FIELD_B(word: i32) -> i32 { (BITS(word, 12, 14) << 3) | BITS(word, 24, 26) }
#[inline] pub fn FIELD_C(word: i32) -> i32 { BITS(word, 6, 11) }
#[inline] pub fn FIELD_u6(word: i32) -> i32 { FIELDC(word) }
#[inline] pub fn FIELD_s12(word: i32) -> i32 { sign_extend((BITS(word, 0, 5) << 6) | BITS(word, 6, 11), 12) }
#[inline] pub fn FIELD_s9(word: i32) -> i32 { sign_extend((BITS(word, 15, 15) << 8) | BITS(word, 16, 23), 9) }
#[inline] pub fn FIELD_s21(word: i32) -> i32 { sign_extend((BITS(word, 6, 15) << 11) | (BITS(word, 17, 26) << 1), 12) }
#[inline] pub fn FIELD_s25(word: i32) -> i32 { sign_extend((BITS(word, 0, 3) << 21) | (BITS(word, 6, 15) << 11) | (BITS(word, 17, 26) << 1), 12) }

#[inline] pub fn FIELD_S_A(word: i32) -> i32 { (BITS(word, 2, 2) << 3) | BITS(word, 0, 2) }
#[inline] pub fn FIELD_S_B(word: i32) -> i32 { (BITS(word, 10, 10) << 3) | BITS(word, 8, 10) }
#[inline] pub fn FIELD_S_C(word: i32) -> i32 { (BITS(word, 7, 7) << 3) | BITS(word, 5, 7) }
#[inline] pub fn FIELD_S_H(word: i32) -> i32 { (BITS(word, 0, 2) << 3) | BITS(word, 5, 8) }
#[inline] pub fn FIELD_S_u5(word: i32) -> i32 { BITS(word, 0, 4) }
#[inline] pub fn FIELD_S_u6(word: i32) -> i32 { BITS(word, 0, 4) << 1 }
#[inline] pub fn FIELD_S_u7(word: i32) -> i32 { BITS(word, 0, 4) << 2 }
#[inline] pub fn FIELD_S_u10(word: i32) -> i32 { BITS(word, 0, 7) << 2 }
#[inline] pub fn FIELD_S_s7(word: i32) -> i32 { sign_extend(BITS(word, 0, 5) << 1, 9) }
#[inline] pub fn FIELD_S_s8(word: i32) -> i32 { sign_extend(BITS(word, 0, 7) << 1, 9) }
#[inline] pub fn FIELD_S_s9(word: i32) -> i32 { sign_extend(BITS(word, 0, 8), 9) }
#[inline] pub fn FIELD_S_s10(word: i32) -> i32 { sign_extend(BITS(word, 0, 8) << 1, 10) }
#[inline] pub fn FIELD_S_s11(word: i32) -> i32 { sign_extend(BITS(word, 0, 8) << 2, 11) }
#[inline] pub fn FIELD_S_s13(word: i32) -> i32 { sign_extend(BITS(word, 0, 10) << 2, 13) }

pub const STATUS32_L: i32 = 0x00000100;
pub const REG_LIMM: i32 = 62;

#[repr(C)]
pub struct disasm_state {
    pub words: [core::ffi::c_ulong; 2],
    pub instr_len: i32,
    pub major_opcode: i32,
    pub is_branch: i32,
    pub target: i32,
    pub delay_slot: i32,
    pub flow: flow,
    pub src1: i32, pub src2: i32, pub src3: i32, pub dest: i32, pub wb_reg: i32,
    pub zz: i32, pub aa: i32, pub x: i32, pub pref: i32, pub di: i32,
    pub fault: i32, pub write: i32,
}

#[inline]
pub fn sign_extend(mut value: i32, bits: i32) -> i32 {
    if IS_BIT(value, bits - 1) != 0 { value |= 0xffffffffu32 as i32 << bits; }
    value
}

#[inline]
pub unsafe fn is_short_instr(addr: core::ffi::c_ulong) -> bool {
    let word = *(addr as *const u16);
    let opcode = (word >> 11) & 0x1F;
    opcode >= 0x0B
}

extern "C" {
    pub fn disasm_instr(addr: core::ffi::c_ulong, state: *mut disasm_state,
        userspace: i32, regs: *mut pt_regs, cregs: *mut callee_regs);
    pub fn disasm_next_pc(pc: core::ffi::c_ulong, regs: *mut pt_regs,
        cregs: *mut callee_regs, fall_thru: *mut core::ffi::c_ulong,
        target: *mut core::ffi::c_ulong) -> i32;
    pub fn get_reg(reg: i32, regs: *mut pt_regs, cregs: *mut callee_regs) -> i64;
    pub fn set_reg(reg: i32, val: i64, regs: *mut pt_regs, cregs: *mut callee_regs);
}

extern "C" {
    pub type pt_regs;
    pub type callee_regs;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
