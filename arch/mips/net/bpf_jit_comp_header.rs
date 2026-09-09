/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Just-In-Time compiler for eBPF bytecode on 32-bit and 64-bit MIPS.
 *
 * Copyright (c) 2021 Anyfi Networks AB.
 * Author: Johan Almbladh <johan.almbladh@gmail.com>
 *
 * Based on code and ideas from
 * Copyright (c) 2017 Cavium, Inc.
 * Copyright (c) 2017 Shubham Bansal <illusionist.neo@gmail.com>
 * Copyright (c) 2011 Mircea Gherzan <mgherzan@gmail.com>
 */

/* MIPS registers */
pub const MIPS_R_ZERO: u32 = 0; /* Const zero */
pub const MIPS_R_AT: u32 = 1; /* Asm temp */
pub const MIPS_R_V0: u32 = 2; /* Result */
pub const MIPS_R_V1: u32 = 3; /* Result */
pub const MIPS_R_A0: u32 = 4; /* Argument */
pub const MIPS_R_A1: u32 = 5; /* Argument */
pub const MIPS_R_A2: u32 = 6; /* Argument */
pub const MIPS_R_A3: u32 = 7; /* Argument */
pub const MIPS_R_A4: u32 = 8; /* Arg (n64) */
pub const MIPS_R_A5: u32 = 9; /* Arg (n64) */
pub const MIPS_R_A6: u32 = 10; /* Arg (n64) */
pub const MIPS_R_A7: u32 = 11; /* Arg (n64) */
pub const MIPS_R_T0: u32 = 8; /* Temp (o32) */
pub const MIPS_R_T1: u32 = 9; /* Temp (o32) */
pub const MIPS_R_T2: u32 = 10; /* Temp (o32) */
pub const MIPS_R_T3: u32 = 11; /* Temp (o32) */
pub const MIPS_R_T4: u32 = 12; /* Temporary */
pub const MIPS_R_T5: u32 = 13; /* Temporary */
pub const MIPS_R_T6: u32 = 14; /* Temporary */
pub const MIPS_R_T7: u32 = 15; /* Temporary */
pub const MIPS_R_S0: u32 = 16; /* Saved */
pub const MIPS_R_S1: u32 = 17; /* Saved */
pub const MIPS_R_S2: u32 = 18; /* Saved */
pub const MIPS_R_S3: u32 = 19; /* Saved */
pub const MIPS_R_S4: u32 = 20; /* Saved */
pub const MIPS_R_S5: u32 = 21; /* Saved */
pub const MIPS_R_S6: u32 = 22; /* Saved */
pub const MIPS_R_S7: u32 = 23; /* Saved */
pub const MIPS_R_T8: u32 = 24; /* Temporary */
pub const MIPS_R_T9: u32 = 25; /* Temporary */
pub const MIPS_R_GP: u32 = 28; /* Global ptr */
pub const MIPS_R_SP: u32 = 29; /* Stack ptr */
pub const MIPS_R_FP: u32 = 30; /* Frame ptr */
pub const MIPS_R_RA: u32 = 31; /* Return */

pub const MIPS_JMP_MASK: u64 = 0x0fffffff;
pub const JIT_MAX_ITERATIONS: u32 = 8;
pub const JIT_JNSET: u32 = 0xe0;
pub const JIT_JNOP: u32 = 0xf0;
pub const JIT_DESC_CONVERT: u32 = 1u32 << 31;

#[repr(C)]
pub struct jit_context {
    pub program: *mut bpf_prog,
    pub descriptors: *mut u32,
    pub target: *mut u32,
    pub bpf_index: u32,
    pub jit_index: u32,
    pub changes: u32,
    pub accessed: u32,
    pub clobbered: u32,
    pub stack_size: u32,
    pub saved_size: u32,
    pub stack_used: u32,
}

/* External types and instruction emitters are supplied by the surrounding translation unit. */
extern "C" {
    pub fn uasm_i_sync(p: *mut *mut u32, value: u32);
}

#[macro_export]
macro_rules! __emit {
    ($ctx:expr, $func:path $(, $arg:expr)*) => {{
        unsafe {
            if !$ctx.target.is_null() {
                let mut p: *mut u32 = $ctx.target.add($ctx.jit_index as usize);
                $func(&mut p $(, $arg)*);
            }
        }
        $ctx.jit_index = $ctx.jit_index.wrapping_add(1);
    }};
}

#[macro_export]
macro_rules! emit {
    ($($args:tt)*) => { $crate::__emit!($($args)*) };
}

/* Build-time configuration conditions from the C header are preserved here. */
#[cfg(CONFIG_WAR_R10000_LLSC)]
pub const LLSC_beqz: &str = "beqzl";
#[cfg(not(CONFIG_WAR_R10000_LLSC))]
pub const LLSC_beqz: &str = "beqz";

#[cfg(CONFIG_CPU_LOONGSON3_WORKAROUNDS)]
pub const LLSC_offset: u32 = 4;
#[cfg(CONFIG_CPU_LOONGSON3_WORKAROUNDS)]
#[macro_export]
macro_rules! LLSC_sync {
    ($ctx:expr) => { $crate::emit!($ctx, uasm_i_sync, 0) };
}
#[cfg(not(CONFIG_CPU_LOONGSON3_WORKAROUNDS))]
pub const LLSC_offset: u32 = 0;
#[cfg(not(CONFIG_CPU_LOONGSON3_WORKAROUNDS))]
#[macro_export]
macro_rules! LLSC_sync {
    ($ctx:expr) => {};
}

#[cfg(CONFIG_CPU_JUMP_WORKAROUNDS)]
pub const JALR_MASK: u64 = 0xffffffffcfffffff;
#[cfg(not(CONFIG_CPU_JUMP_WORKAROUNDS))]
pub const JALR_MASK: u64 = !0u64;

#[inline]
pub unsafe fn access_reg(ctx: *mut jit_context, reg: u8) {
    (*ctx).accessed |= 1u32.wrapping_shl(reg as u32);
}

#[inline]
pub unsafe fn clobber_reg(ctx: *mut jit_context, reg: u8) {
    (*ctx).clobbered |= 1u32.wrapping_shl(reg as u32);
}

extern "C" {
    pub fn push_regs(ctx: *mut jit_context, mask: u32, excl: u32, depth: i32) -> i32;
    pub fn pop_regs(ctx: *mut jit_context, mask: u32, excl: u32, depth: i32) -> i32;
    pub fn get_target(ctx: *mut jit_context, loc: u32) -> i32;
    pub fn get_offset(ctx: *const jit_context, off: i32) -> i32;
    pub fn emit_mov_i(ctx: *mut jit_context, dst: u8, imm: i32);
    pub fn emit_mov_r(ctx: *mut jit_context, dst: u8, src: u8);
    pub fn valid_alu_i(op: u8, imm: i32) -> bool;
    pub fn rewrite_alu_i(op: u8, imm: i32, alu: *mut u8, val: *mut i32) -> bool;
    pub fn emit_alu_i(ctx: *mut jit_context, dst: u8, imm: i32, op: u8);
    pub fn emit_alu_r(ctx: *mut jit_context, dst: u8, src: u8, op: u8);
    pub fn emit_atomic_r(ctx: *mut jit_context, dst: u8, src: u8, off: i16, code: u8);
    pub fn emit_cmpxchg_r(ctx: *mut jit_context, dst: u8, src: u8, res: u8, off: i16);
    pub fn emit_bswap_r(ctx: *mut jit_context, dst: u8, width: u32);
    pub fn valid_jmp_i(op: u8, imm: i32) -> bool;
    pub fn setup_jmp_i(ctx: *mut jit_context, imm: i32, width: u8, bpf_op: u8, bpf_off: i16, jit_op: *mut u8, jit_off: *mut i32);
    pub fn setup_jmp_r(ctx: *mut jit_context, same_reg: bool, bpf_op: u8, bpf_off: i16, jit_op: *mut u8, jit_off: *mut i32);
    pub fn finish_jmp(ctx: *mut jit_context, jit_op: u8, bpf_off: i16) -> i32;
    pub fn emit_jmp_i(ctx: *mut jit_context, dst: u8, imm: i32, off: i32, op: u8);
    pub fn emit_jmp_r(ctx: *mut jit_context, dst: u8, src: u8, off: i32, op: u8);
    pub fn emit_ja(ctx: *mut jit_context, off: i16) -> i32;
    pub fn emit_exit(ctx: *mut jit_context) -> i32;
    pub fn build_prologue(ctx: *mut jit_context);
    pub fn build_epilogue(ctx: *mut jit_context, dest_reg: i32);
    pub fn build_insn(insn: *const bpf_insn, ctx: *mut jit_context) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
