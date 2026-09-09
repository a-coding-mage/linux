/* SPDX-License-Identifier: GPL-2.0 */
/*
 * The interface that a back-end should provide to bpf_jit_core.c.
 *
 * Copyright (c) 2024 Synopsys Inc.
 * Author: Shahab Vahedi <shahab@synopsys.com>
 */

/* Print debug info and assert. */
// ARC_BPF_JIT_DEBUG

/* Determine the address type of the target. */
#[cfg(CONFIG_ISA_ARCV2)]
pub type ARC_ADDR = u32;

/* For the translation of some BPF instructions, a temporary register
 * might be needed for some interim data. */
pub const JIT_REG_TMP: u32 = MAX_BPF_JIT_REG;

/* Buffer access: If buffer "b" is not NULL, advance by "n" bytes. */
#[inline]
pub unsafe fn BUF(b: *mut u8, n: usize) -> *mut u8 {
    if !b.is_null() { b.add(n) } else { b }
}

extern "C" {
    pub fn zext(buf: *mut u8, rd: u8) -> u8;
    pub fn mov_r32(buf: *mut u8, rd: u8, rs: u8, sign_ext: u8) -> u8;
    pub fn mov_r32_i32(buf: *mut u8, reg: u8, imm: i32) -> u8;
    pub fn mov_r64(buf: *mut u8, rd: u8, rs: u8, sign_ext: u8) -> u8;
    pub fn mov_r64_i32(buf: *mut u8, reg: u8, imm: i32) -> u8;
    pub fn mov_r64_i64(buf: *mut u8, reg: u8, lo: u32, hi: u32) -> u8;
    pub fn load_r(buf: *mut u8, rd: u8, rs: u8, off: i16, size: u8, sign_ext: bool) -> u8;
    pub fn store_r(buf: *mut u8, rd: u8, rs: u8, off: i16, size: u8) -> u8;
    pub fn store_i(buf: *mut u8, imm: i32, rd: u8, off: i16, size: u8) -> u8;
    pub fn add_r32(buf: *mut u8, rd: u8, rs: u8) -> u8;
    pub fn add_r32_i32(buf: *mut u8, rd: u8, imm: i32) -> u8;
    pub fn add_r64(buf: *mut u8, rd: u8, rs: u8) -> u8;
    pub fn add_r64_i32(buf: *mut u8, rd: u8, imm: i32) -> u8;
    pub fn sub_r32(buf: *mut u8, rd: u8, rs: u8) -> u8;
    pub fn sub_r32_i32(buf: *mut u8, rd: u8, imm: i32) -> u8;
    pub fn sub_r64(buf: *mut u8, rd: u8, rs: u8) -> u8;
    pub fn sub_r64_i32(buf: *mut u8, rd: u8, imm: i32) -> u8;
    pub fn mul_r32(buf: *mut u8, rd: u8, rs: u8) -> u8;
    pub fn mul_r32_i32(buf: *mut u8, rd: u8, imm: i32) -> u8;
    pub fn mul_r64(buf: *mut u8, rd: u8, rs: u8) -> u8;
    pub fn mul_r64_i32(buf: *mut u8, rd: u8, imm: i32) -> u8;
    pub fn div_r32(buf: *mut u8, rd: u8, rs: u8, sign_ext: bool) -> u8;
    pub fn div_r32_i32(buf: *mut u8, rd: u8, imm: i32, sign_ext: bool) -> u8;
    pub fn mod_r32(buf: *mut u8, rd: u8, rs: u8, sign_ext: bool) -> u8;
    pub fn mod_r32_i32(buf: *mut u8, rd: u8, imm: i32, sign_ext: bool) -> u8;
    pub fn and_r32(buf: *mut u8, rd: u8, rs: u8) -> u8;
    pub fn and_r32_i32(buf: *mut u8, rd: u8, imm: i32) -> u8;
    pub fn and_r64(buf: *mut u8, rd: u8, rs: u8) -> u8;
    pub fn and_r64_i32(buf: *mut u8, rd: u8, imm: i32) -> u8;
    pub fn or_r32(buf: *mut u8, rd: u8, rs: u8) -> u8;
    pub fn or_r32_i32(buf: *mut u8, rd: u8, imm: i32) -> u8;
    pub fn or_r64(buf: *mut u8, rd: u8, rs: u8) -> u8;
    pub fn or_r64_i32(buf: *mut u8, rd: u8, imm: i32) -> u8;
    pub fn xor_r32(buf: *mut u8, rd: u8, rs: u8) -> u8;
    pub fn xor_r32_i32(buf: *mut u8, rd: u8, imm: i32) -> u8;
    pub fn xor_r64(buf: *mut u8, rd: u8, rs: u8) -> u8;
    pub fn xor_r64_i32(buf: *mut u8, rd: u8, imm: i32) -> u8;
    pub fn neg_r32(buf: *mut u8, r: u8) -> u8;
    pub fn neg_r64(buf: *mut u8, r: u8) -> u8;
    pub fn lsh_r32(buf: *mut u8, rd: u8, rs: u8) -> u8;
    pub fn lsh_r32_i32(buf: *mut u8, rd: u8, imm: u8) -> u8;
    pub fn lsh_r64(buf: *mut u8, rd: u8, rs: u8) -> u8;
    pub fn lsh_r64_i32(buf: *mut u8, rd: u8, imm: i32) -> u8;
    pub fn rsh_r32(buf: *mut u8, rd: u8, rs: u8) -> u8;
    pub fn rsh_r32_i32(buf: *mut u8, rd: u8, imm: u8) -> u8;
    pub fn rsh_r64(buf: *mut u8, rd: u8, rs: u8) -> u8;
    pub fn rsh_r64_i32(buf: *mut u8, rd: u8, imm: i32) -> u8;
    pub fn arsh_r32(buf: *mut u8, rd: u8, rs: u8) -> u8;
    pub fn arsh_r32_i32(buf: *mut u8, rd: u8, imm: u8) -> u8;
    pub fn arsh_r64(buf: *mut u8, rd: u8, rs: u8) -> u8;
    pub fn arsh_r64_i32(buf: *mut u8, rd: u8, imm: i32) -> u8;
    pub fn mask_for_used_regs(bpf_reg: u8, is_call: bool) -> u32;
    pub fn arc_prologue(buf: *mut u8, usage: u32, frame_size: u16) -> u8;
    pub fn arc_epilogue(buf: *mut u8, usage: u32, frame_size: u16) -> u8;
    pub fn check_jmp_32(curr_off: u32, targ_off: u32, cond: u8) -> bool;
    pub fn check_jmp_64(curr_off: u32, targ_off: u32, cond: u8) -> bool;
    pub fn gen_jmp_32(buf: *mut u8, rd: u8, rs: u8, cond: u8, c_off: u32, t_off: u32) -> u8;
    pub fn gen_jmp_64(buf: *mut u8, rd: u8, rs: u8, cond: u8, c_off: u32, t_off: u32) -> u8;
    pub fn gen_func_call(buf: *mut u8, func_addr: ARC_ADDR, external_func: bool) -> u8;
    pub fn arc_to_bpf_return(buf: *mut u8) -> u8;
    pub fn gen_swap(buf: *mut u8, rd: u8, size: u8, endian: u8, force: bool, do_zext: bool) -> u8;
}

#[repr(C)]
pub enum ARC_CC {
    ARC_CC_UGT = 0, ARC_CC_UGE, ARC_CC_ULT, ARC_CC_ULE,
    ARC_CC_SGT, ARC_CC_SGE, ARC_CC_SLT, ARC_CC_SLE,
    ARC_CC_AL, ARC_CC_EQ, ARC_CC_NE, ARC_CC_SET, ARC_CC_LAST,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
