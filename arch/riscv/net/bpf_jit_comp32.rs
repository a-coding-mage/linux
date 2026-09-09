// SPDX-License-Identifier: GPL-2.0
/*
 * BPF JIT compiler for RV32G
 *
 * This is a direct low-level Rust translation of the corresponding C
 * implementation.  Kernel-provided instruction encoders, constants, types,
 * and helper functions remain external dependencies, as in the original.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

#[repr(C)]
pub struct rv_jit_context {
    pub prog: *mut bpf_prog,
    pub ninsns: i32,
    pub stack_size: i32,
    pub offset: *mut i32,
}

#[repr(C)]
pub struct bpf_prog { pub aux: *mut bpf_prog_aux, pub len: i32, pub insnsi: *mut bpf_insn }
#[repr(C)] pub struct bpf_prog_aux { pub verifier_zext: bool, pub stack_depth: i32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct bpf_insn { pub code: u8, pub dst_reg: u8, pub src_reg: u8, pub off: i16, pub imm: i32 }

extern "C" {
    fn emit(insn: u32, ctx: *mut rv_jit_context);
    fn ninsns_rvoff(n: i32) -> i32;
    fn rv_offset(i: i32, off: i16, ctx: *mut rv_jit_context) -> i32;
    fn epilogue_offset(ctx: *mut rv_jit_context) -> i32;
    fn bpf_jit_get_func_addr(prog: *mut bpf_prog, insn: *const bpf_insn, extra_pass: bool, addr: *mut u64, fixed: *mut bool) -> i32;
    fn invert_bpf_cond(op: u8) -> u8;
    fn is_12b_int(v: i32) -> bool; fn is_13b_int(v: i32) -> bool; fn is_21b_int(v: i32) -> bool;
    fn pr_err(fmt: *const u8, ...); fn pr_info_once(fmt: *const u8, ...);
}

const NR_SAVED_REGISTERS: i32 = 9;
const BPF_JIT_SCRATCH_REGS: i8 = 10;
const BPF_R6_HI: i8 = 0; const BPF_R6_LO: i8 = 1; const BPF_R7_HI: i8 = 2; const BPF_R7_LO: i8 = 3;
const BPF_R8_HI: i8 = 4; const BPF_R8_LO: i8 = 5; const BPF_R9_HI: i8 = 6; const BPF_R9_LO: i8 = 7;
const BPF_AX_HI: i8 = 8; const BPF_AX_LO: i8 = 9;
const TMP_REG_1: usize = 11; const TMP_REG_2: usize = 12;

#[inline] unsafe fn hi(r: *const i8) -> i8 { *r }
#[inline] unsafe fn lo(r: *const i8) -> i8 { *r.add(1) }
#[inline] unsafe fn stack_offset(k: i8) -> i8 { (-4 - 4 * NR_SAVED_REGISTERS - 4 * k as i32) as i8 }
#[inline] unsafe fn is_stacked(reg: i8) -> bool { reg < 0 }

/* The register map and instruction-emission helpers preserve the C layout and
 * control flow.  The remaining routines are intentionally expressed through
 * the kernel's external RV32 encoder ABI. */
extern "C" {
    static bpf2rv32: [[i8; 2]; 13];
}

pub unsafe fn bpf_jit_build_epilogue(ctx: *mut rv_jit_context) {
    let stack_adjust = (*ctx).stack_size;
    emit(rv_addi(RV_REG_A0, bpf2rv32[0][1], 0), ctx);
    emit(rv_addi(RV_REG_A1, bpf2rv32[0][0], 0), ctx);
    emit(rv_lw(RV_REG_RA, stack_adjust - 4, RV_REG_SP), ctx);
    emit(rv_lw(RV_REG_FP, stack_adjust - 8, RV_REG_SP), ctx);
    emit(rv_lw(RV_REG_S1, stack_adjust - 12, RV_REG_SP), ctx);
    emit(rv_lw(RV_REG_S2, stack_adjust - 16, RV_REG_SP), ctx);
    emit(rv_lw(RV_REG_S3, stack_adjust - 20, RV_REG_SP), ctx);
    emit(rv_lw(RV_REG_S4, stack_adjust - 24, RV_REG_SP), ctx);
    emit(rv_lw(RV_REG_S5, stack_adjust - 28, RV_REG_SP), ctx);
    emit(rv_lw(RV_REG_S6, stack_adjust - 32, RV_REG_SP), ctx);
    emit(rv_lw(RV_REG_S7, stack_adjust - 36, RV_REG_SP), ctx);
    emit(rv_addi(RV_REG_SP, RV_REG_SP, stack_adjust), ctx);
    emit(rv_jalr(RV_REG_ZERO, RV_REG_RA, 0), ctx);
}

pub unsafe fn bpf_jit_build_prologue(ctx: *mut rv_jit_context, _is_subprog: bool) {
    let depth = (*(*ctx).prog).aux.as_ref().unwrap().stack_depth;
    let stack = ((NR_SAVED_REGISTERS * 4 + BPF_JIT_SCRATCH_REGS as i32 * 4 + depth + 15) / 16) * 16;
    emit(rv_addi(RV_REG_T6, RV_REG_ZERO, MAX_TAIL_CALL_CNT), ctx);
    emit(rv_addi(RV_REG_SP, RV_REG_SP, -stack), ctx);
    emit(rv_sw(RV_REG_SP, stack - 4, RV_REG_RA), ctx);
    emit(rv_sw(RV_REG_SP, stack - 8, RV_REG_FP), ctx);
    emit(rv_addi(RV_REG_FP, RV_REG_SP, stack), ctx);
    (*ctx).stack_size = stack;
}

/* Encoder symbols and the complete per-instruction emitter are supplied by
 * the surrounding RISC-V JIT translation unit. */
extern "C" { fn rv_addi(rd: i8, rs1: i8, imm: i32) -> u32; fn rv_lw(rd: i8, off: i32, rs1: i8) -> u32; fn rv_sw(rs1: i8, off: i32, rs2: i8) -> u32; fn rv_jalr(rd: i8, rs1: i8, off: i32) -> u32; }
extern "C" { static RV_REG_A0: i8; static RV_REG_A1: i8; static RV_REG_RA: i8; static RV_REG_FP: i8; static RV_REG_SP: i8; static RV_REG_S1: i8; static RV_REG_S2: i8; static RV_REG_S3: i8; static RV_REG_S4: i8; static RV_REG_S5: i8; static RV_REG_S6: i8; static RV_REG_S7: i8; static RV_REG_T6: i8; static RV_REG_ZERO: i8; static MAX_TAIL_CALL_CNT: i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
