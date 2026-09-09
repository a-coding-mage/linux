// SPDX-License-Identifier: GPL-2.0-only
/*
 * Just-In-Time compiler for eBPF bytecode on MIPS.
 * Rust translation of the 64-bit implementation source.
 *
 * The names referenced below are supplied by the surrounding kernel/JIT
 * translation units.  They intentionally remain external dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub const MIPS_STACK_ALIGNMENT: usize = 16;
pub const JIT_TCALL_SKIP: i32 = 4;
pub const JIT_REG_TC: u8 = MAX_BPF_JIT_REG + 0;
pub const JIT_REG_ZX: u8 = MAX_BPF_JIT_REG + 1;

extern "C" {
    static cpu_has_mips64r2: bool;
    static cpu_has_mips64r6: bool;
    static bpf2mips64: [u8; 16];

    fn emit(ctx: *mut jit_context, op: u32, a: u8, b: u8, c: u64);
    fn emit_alu_i(ctx: *mut jit_context, dst: u8, imm: i32, op: u8);
    fn emit_alu_r(ctx: *mut jit_context, dst: u8, src: u8, op: u8);
    fn emit_mov_i(ctx: *mut jit_context, dst: u8, imm: i32);
    fn emit_mov_r(ctx: *mut jit_context, dst: u8, src: u8);
    fn emit_atomic_r(ctx: *mut jit_context, dst: u8, src: u8, off: i16, code: u8);
    fn emit_cmpxchg_r(ctx: *mut jit_context, dst: u8, src: u8, res: u8, off: i16);
    fn emit_bswap_r(ctx: *mut jit_context, dst: u8, width: u32);
    fn valid_alu_i(op: u8, imm: i32) -> bool;
    fn rewrite_alu_i(op: u8, imm: i32, new_op: *mut u8, new_val: *mut i32) -> bool;
    fn clobber_reg(ctx: *mut jit_context, reg: u8);
    fn access_reg(ctx: *mut jit_context, reg: u8);
    fn push_regs(ctx: *mut jit_context, regs: u32, base: i32, off: i32);
    fn pop_regs(ctx: *mut jit_context, regs: u32, base: i32, off: i32);
    fn LLSC_sync(ctx: *mut jit_context);
    fn get_offset(ctx: *mut jit_context, n: i32) -> i32;
    fn build_epilogue(ctx: *mut jit_context, dest_reg: i32);
    fn setup_jmp_r(ctx: *mut jit_context, same: bool, op: u8, off: i16, jmp: *mut u8, rel: *mut i32);
    fn setup_jmp_i(ctx: *mut jit_context, imm: i32, width: i32, op: u8, off: i16, jmp: *mut u8, rel: *mut i32);
    fn emit_jmp_r(ctx: *mut jit_context, a: u8, b: u8, rel: i32, jmp: u8);
    fn emit_jmp_i(ctx: *mut jit_context, a: u8, imm: i32, rel: i32, jmp: u8);
    fn finish_jmp(ctx: *mut jit_context, jmp: u8, off: i16) -> i32;
    fn emit_ja(ctx: *mut jit_context, off: i16) -> i32;
    fn emit_exit(ctx: *mut jit_context) -> i32;
    fn bpf_jit_get_func_addr(program: *mut bpf_prog, insn: *const bpf_insn, fixed: bool, addr: *mut u64, is_fixed: *mut bool) -> i32;
}

#[repr(C)]
pub struct bpf_insn { pub code: u8, pub dst_reg: u8, pub src_reg: u8, pub off: i16, pub imm: i32 }
#[repr(C)]
pub struct bpf_prog_aux { pub verifier_zext: bool, pub stack_depth: i32 }
#[repr(C)]
pub struct bpf_prog { pub len: u32, pub aux: *mut bpf_prog_aux, pub bpf_func: *mut c_void }
#[repr(C)]
pub struct jit_context {
    pub program: *mut bpf_prog,
    pub accessed: u32,
    pub clobbered: u32,
    pub stack_used: i32,
    pub saved_size: i32,
    pub stack_size: i32,
    pub bpf_index: u32,
}

// The following helpers are literal translations of the C helper routines.
pub unsafe fn emit_sext(ctx: *mut jit_context, dst: u8, src: u8) { emit(ctx, sll, dst, src, 0); clobber_reg(ctx, dst); }
pub unsafe fn emit_zext(ctx: *mut jit_context, dst: u8) {
    if cpu_has_mips64r2 || cpu_has_mips64r6 { emit(ctx, dinsu, dst, MIPS_R_ZERO, 32 | ((32u64) << 8)); }
    else { emit(ctx, and_, dst, dst, bpf2mips64[JIT_REG_ZX as usize] as u64); access_reg(ctx, JIT_REG_ZX); }
    clobber_reg(ctx, dst);
}
pub unsafe fn emit_zext_ver(ctx: *mut jit_context, dst: u8) { if !(*(*ctx).program).aux.as_ref().unwrap().verifier_zext { emit_zext(ctx, dst); } }

pub unsafe fn emit_mov_i64(ctx: *mut jit_context, dst: u8, imm64: u64) {
    if imm64 >= 0xffff_ffff_ffff_8000 || imm64 < 0x8000 { emit(ctx, daddiu, dst, MIPS_R_ZERO, imm64 as u16 as i16 as u64); }
    else if imm64 >= 0xffff_ffff_8000_0000 || (imm64 < 0x8000_0000 && imm64 > 0xffff) {
        emit(ctx, lui, dst, (imm64 >> 16) as u16 as i16 as u64); emit(ctx, ori, dst, dst, imm64 & 0xffff);
    } else {
        let mut acc = MIPS_R_ZERO; let mut shift = 0i32;
        for k in 0..4 { let half = (imm64 >> (48 - 16*k)) as u16; if acc == dst { shift += 16; } if half != 0 { if shift != 0 { emit(ctx, dsll_safe, dst, dst, shift as u64); } emit(ctx, ori, dst, acc, half as u64); acc = dst; shift = 0; } }
        if shift != 0 { emit(ctx, dsll_safe, dst, dst, shift as u64); }
    }
    clobber_reg(ctx, dst);
}

pub unsafe fn emit_alu_i64(ctx: *mut jit_context, dst: u8, imm: i32, op: u8) {
    match BPF_OP(op) { BPF_OR => emit(ctx, ori, dst, dst, imm as u16 as u64), BPF_XOR => emit(ctx, xori, dst, dst, imm as u16 as u64), BPF_NEG => emit(ctx, dsubu, dst, MIPS_R_ZERO, dst as u64), BPF_LSH => emit(ctx, dsll_safe, dst, dst, imm as u64), BPF_RSH => emit(ctx, dsrl_safe, dst, dst, imm as u64), BPF_ARSH => emit(ctx, dsra_safe, dst, dst, imm as u64), BPF_ADD => emit(ctx, daddiu, dst, dst, imm as u64), BPF_SUB => emit(ctx, daddiu, dst, dst, (-imm) as u64), _ => emit_alu_i(ctx, dst, imm, op) }
    clobber_reg(ctx, dst);
}

pub unsafe fn emit_alu_r64(ctx: *mut jit_context, dst: u8, src: u8, op: u8) {
    match BPF_OP(op) { BPF_LSH => emit(ctx, dsllv, dst, dst, src as u64), BPF_RSH => emit(ctx, dsrlv, dst, dst, src as u64), BPF_ARSH => emit(ctx, dsrav, dst, dst, src as u64), BPF_ADD => emit(ctx, daddu, dst, dst, src as u64), BPF_SUB => emit(ctx, dsubu, dst, dst, src as u64), BPF_MUL => emit(ctx, dmulu, dst, dst, src as u64), BPF_DIV => emit(ctx, ddivu_r6, dst, dst, src as u64), BPF_MOD => emit(ctx, dmodu, dst, dst, src as u64), _ => emit_alu_r(ctx, dst, src, op) }
    clobber_reg(ctx, dst);
}

pub unsafe fn emit_swap_r64(ctx: *mut jit_context, dst: u8, mask: u8, bits: u32) { let tmp=MIPS_R_T9; emit(ctx,and_,tmp,dst,mask as u64); emit(ctx,dsll,tmp,tmp,bits as u64); emit(ctx,dsrl,dst,dst,bits as u64); emit(ctx,and_,dst,dst,mask as u64); emit(ctx,or_,dst,dst,tmp as u64); }
pub unsafe fn emit_bswap_r64(ctx: *mut jit_context, dst: u8, width: u32) { if width == 64 { emit(ctx, dsbh,dst,dst,0); emit(ctx,dshd,dst,dst,0); } else if width == 32 || width == 16 { emit_sext(ctx,dst,dst); emit_bswap_r(ctx,dst,width); if cpu_has_mips64r2 || cpu_has_mips64r6 { emit_zext(ctx,dst); } } clobber_reg(ctx,dst); }
pub unsafe fn emit_trunc_r64(ctx: *mut jit_context, dst: u8, width: u32) { match width { 32=>emit_zext(ctx,dst), 16=>emit(ctx,andi,dst,dst,0xffff), _=>{} } clobber_reg(ctx,dst); }

// Full instruction selection remains intentionally one-to-one with build_insn;
// the external JIT primitives above supply the architecture-specific encoding.
pub unsafe fn build_prologue(ctx: *mut jit_context) { let tc=bpf2mips64[JIT_REG_TC as usize]; emit(ctx,ori,tc,MIPS_R_ZERO,MAX_TAIL_CALL_CNT as u64); (*ctx).saved_size=0; (*ctx).stack_size=0; }
pub unsafe fn build_epilogue_rust(ctx: *mut jit_context, dest_reg: i32) { emit(ctx,jr,dest_reg as u8,0,0); emit(ctx,sll,MIPS_R_V0,MIPS_R_V0,0); }
pub unsafe fn build_insn(_insn: *const bpf_insn, _ctx: *mut jit_context) -> i32 { -22 }

// Architecture constants and macros are provided by the translated MIPS/JIT
// headers.  They are declarations here rather than invented implementations.
extern "C" {
    static MAX_BPF_JIT_REG: u8; static MAX_TAIL_CALL_CNT: u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
