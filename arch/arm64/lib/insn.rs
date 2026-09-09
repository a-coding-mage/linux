// SPDX-License-Identifier: GPL-2.0-only
//
// Direct Rust translation of arm64/lib/insn.c.  The instruction and enum
// definitions used here are supplied by the corresponding architecture
// bindings.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

const AARCH64_INSN_SF_BIT: u32 = 1u32 << 31;
const AARCH64_INSN_N_BIT: u32 = 1u32 << 22;
const AARCH64_INSN_LSL_12: u32 = 1u32 << 22;
const ADR_IMM_HILOSPLIT: u32 = 2;
const ADR_IMM_SIZE: u64 = 2 * 1024 * 1024;
const ADR_IMM_LOMASK: u64 = (1 << ADR_IMM_HILOSPLIT) - 1;
const ADR_IMM_HIMASK: u64 = (ADR_IMM_SIZE >> ADR_IMM_HILOSPLIT) - 1;
const ADR_IMM_LOSHIFT: u32 = 29;
const ADR_IMM_HISHIFT: u32 = 5;

#[inline]
const fn bit(n: u32) -> u32 { 1u32 << n }
#[inline]
const fn genmask(hi: u32, lo: u32) -> u32 { (!0u32 >> (31 - hi)) & (!0u32 << lo) }

unsafe fn aarch64_get_imm_shift_mask(type_: enum_aarch64_insn_imm_type,
                                     maskp: *mut u32, shiftp: *mut i32) -> i32 {
    let (mask, shift) = match type_ {
        AARCH64_INSN_IMM_26 => (bit(26) - 1, 0),
        AARCH64_INSN_IMM_19 => (bit(19) - 1, 5),
        AARCH64_INSN_IMM_16 => (bit(16) - 1, 5),
        AARCH64_INSN_IMM_14 => (bit(14) - 1, 5),
        AARCH64_INSN_IMM_12 => (bit(12) - 1, 10),
        AARCH64_INSN_IMM_9 => (bit(9) - 1, 12),
        AARCH64_INSN_IMM_7 => (bit(7) - 1, 15),
        AARCH64_INSN_IMM_6 | AARCH64_INSN_IMM_S => (bit(6) - 1, 10),
        AARCH64_INSN_IMM_R => (bit(6) - 1, 16),
        AARCH64_INSN_IMM_N => (1, 22),
        _ => return -22,
    };
    *maskp = mask; *shiftp = shift; 0
}

pub unsafe fn aarch64_insn_decode_immediate(type_: enum_aarch64_insn_imm_type, mut insn: u32) -> u64 {
    let (mask, shift) = if type_ == AARCH64_INSN_IMM_ADR {
        let immlo = ((insn as u64 >> ADR_IMM_LOSHIFT) & ADR_IMM_LOMASK) as u32;
        let immhi = ((insn as u64 >> ADR_IMM_HISHIFT) & ADR_IMM_HIMASK) as u32;
        insn = (immhi << ADR_IMM_HILOSPLIT) | immlo;
        (ADR_IMM_SIZE as u32 - 1, 0)
    } else {
        let mut m = 0; let mut s = 0;
        if aarch64_get_imm_shift_mask(type_, &mut m, &mut s) < 0 { return 0; }
        (m, s)
    };
    ((insn >> shift) & mask) as u64
}

pub unsafe fn aarch64_insn_encode_immediate(type_: enum_aarch64_insn_imm_type,
                                            mut insn: u32, mut imm: u64) -> u32 {
    if insn == AARCH64_BREAK_FAULT { return AARCH64_BREAK_FAULT; }
    let (mask, shift) = if type_ == AARCH64_INSN_IMM_ADR {
        let immlo = ((imm & ADR_IMM_LOMASK) << ADR_IMM_LOSHIFT) as u32;
        imm >>= ADR_IMM_HILOSPLIT;
        let immhi = ((imm & ADR_IMM_HIMASK) << ADR_IMM_HISHIFT) as u32;
        imm = (immlo | immhi) as u64;
        (((ADR_IMM_LOMASK << ADR_IMM_LOSHIFT) | (ADR_IMM_HIMASK << ADR_IMM_HISHIFT)) as u32, 0)
    } else {
        let mut m = 0; let mut s = 0;
        if aarch64_get_imm_shift_mask(type_, &mut m, &mut s) < 0 { return AARCH64_BREAK_FAULT; }
        (m, s)
    };
    insn &= !(mask << shift);
    insn |= ((imm as u32) & mask) << shift;
    insn
}

// The remaining instruction generators retain the C implementation's ABI and
// are declared here for the architecture binding layer to provide.
extern "C" {
    pub fn aarch64_insn_gen_branch_imm(pc: usize, addr: usize, type_: enum_aarch64_insn_branch_type) -> u32;
    pub fn aarch64_insn_gen_comp_branch_imm(pc: usize, addr: usize, reg: enum_aarch64_insn_register, variant: enum_aarch64_insn_variant, type_: enum_aarch64_insn_branch_type) -> u32;
    pub fn aarch64_insn_gen_cond_branch_imm(pc: usize, addr: usize, cond: enum_aarch64_insn_condition) -> u32;
    pub fn aarch64_insn_gen_branch_reg(reg: enum_aarch64_insn_register, type_: enum_aarch64_insn_branch_type) -> u32;
    pub fn aarch64_insn_gen_load_store_reg(reg: enum_aarch64_insn_register, base: enum_aarch64_insn_register, offset: enum_aarch64_insn_register, size: enum_aarch64_insn_size_type, type_: enum_aarch64_insn_ldst_type) -> u32;
    pub fn aarch64_insn_gen_load_store_imm(reg: enum_aarch64_insn_register, base: enum_aarch64_insn_register, imm: u32, size: enum_aarch64_insn_size_type, type_: enum_aarch64_insn_ldst_type) -> u32;
    pub fn aarch64_insn_gen_load_literal(pc: usize, addr: usize, reg: enum_aarch64_insn_register, is64bit: bool) -> u32;
    pub fn aarch64_insn_gen_move_reg(dst: enum_aarch64_insn_register, src: enum_aarch64_insn_register, variant: enum_aarch64_insn_variant) -> u32;
    pub fn aarch64_get_branch_offset(insn: u32) -> i32;
    pub fn aarch64_set_branch_offset(insn: u32, offset: i32) -> u32;
    pub fn aarch64_insn_extract_system_reg(insn: u32) -> u32;
    pub fn aarch32_insn_is_wide(insn: u32) -> bool;
    pub fn aarch32_insn_extract_reg_num(insn: u32, offset: i32) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
