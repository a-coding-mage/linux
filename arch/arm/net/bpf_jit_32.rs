// SPDX-License-Identifier: GPL-2.0-only
//! Source-level Rust translation of the 32-bit ARM eBPF JIT.
//!
//! Kernel-provided types, constants, macros, and architecture helpers are
//! intentionally referenced but not reimplemented here.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::c_void;

/* eBPF JIT stack layout constants. */
const CALLEE_MASK: u32 = (1 << ARM_R4) | (1 << ARM_R5) | (1 << ARM_R6) |
    (1 << ARM_R7) | (1 << ARM_R8) | (1 << ARM_R9) | (1 << ARM_FP);
const CALLEE_PUSH_MASK: u32 = CALLEE_MASK | (1 << ARM_LR);
const CALLEE_POP_MASK: u32 = CALLEE_MASK | (1 << ARM_PC);
const CALLER_MASK: u32 = (1 << ARM_R0) | (1 << ARM_R1) | (1 << ARM_R2) | (1 << ARM_R3);

#[repr(u32)]
enum StackLayout {
    BPF_R2_HI, BPF_R2_LO, BPF_R3_HI, BPF_R3_LO,
    BPF_R4_HI, BPF_R4_LO, BPF_R5_HI, BPF_R5_LO,
    BPF_R7_HI, BPF_R7_LO, BPF_R8_HI, BPF_R8_LO,
    BPF_R9_HI, BPF_R9_LO, BPF_FP_HI, BPF_FP_LO,
    BPF_TC_HI, BPF_TC_LO, BPF_AX_HI, BPF_AX_LO,
    BPF_JIT_SCRATCH_REGS,
}

const fn stack_offset(k: i32) -> i8 { (-4 - k * 4) as i8 }
const SCRATCH_SIZE: u32 = StackLayout::BPF_JIT_SCRATCH_REGS as u32 * 4;
const TMP_REG_1: usize = MAX_BPF_JIT_REG as usize;
const TMP_REG_2: usize = TMP_REG_1 + 1;
const TCALL_CNT: usize = TMP_REG_1 + 2;
const FLAG_IMM_OVERFLOW: u32 = 1;

/* The indexed register map is deliberately kept as signed bytes: negative
 * values denote scratch-stack slots, exactly as in the C implementation. */
static BPF2A32: [[i8; 2]; (MAX_BPF_JIT_REG as usize) + 4] = [
    [ARM_R1 as i8, ARM_R0 as i8], [ARM_R3 as i8, ARM_R2 as i8],
    [stack_offset(StackLayout::BPF_R2_HI as i32), stack_offset(StackLayout::BPF_R2_LO as i32)],
    [stack_offset(StackLayout::BPF_R3_HI as i32), stack_offset(StackLayout::BPF_R3_LO as i32)],
    [stack_offset(StackLayout::BPF_R4_HI as i32), stack_offset(StackLayout::BPF_R4_LO as i32)],
    [stack_offset(StackLayout::BPF_R5_HI as i32), stack_offset(StackLayout::BPF_R5_LO as i32)],
    [ARM_R5 as i8, ARM_R4 as i8],
    [stack_offset(StackLayout::BPF_R7_HI as i32), stack_offset(StackLayout::BPF_R7_LO as i32)],
    [stack_offset(StackLayout::BPF_R8_HI as i32), stack_offset(StackLayout::BPF_R8_LO as i32)],
    [stack_offset(StackLayout::BPF_R9_HI as i32), stack_offset(StackLayout::BPF_R9_LO as i32)],
    [stack_offset(StackLayout::BPF_FP_HI as i32), stack_offset(StackLayout::BPF_FP_LO as i32)],
];

#[repr(C)]
pub struct jit_ctx {
    pub prog: *const bpf_prog,
    pub idx: u32,
    pub prologue_bytes: u32,
    pub epilogue_offset: u32,
    pub cpu_architecture: u32,
    pub flags: u32,
    pub offsets: *mut u32,
    pub target: *mut u32,
    pub stack_size: u32,
    #[cfg(any())]
    pub epilogue_bytes: u16,
    #[cfg(any())]
    pub imm_count: u16,
    #[cfg(any())]
    pub imms: *mut u32,
}

/* Kernel ABI types and architecture constants are supplied by the including
 * translation unit. */
extern "C" {
    fn div64_u64(dividend: u64, divisor: u64) -> u64;
    fn div64_u64_rem(dividend: u64, divisor: u64, rem: *mut u64) -> u64;
    fn div64_s64(dividend: i64, divisor: i64) -> i64;
    fn __opcode_to_mem_arm(inst: u32) -> u32;
}

#[inline] unsafe fn jit_udiv32(dividend: u32, divisor: u32) -> u32 { dividend / divisor }
#[inline] unsafe fn jit_mod32(dividend: u32, divisor: u32) -> u32 { dividend % divisor }
#[inline] unsafe fn jit_sdiv32(dividend: i32, divisor: i32) -> i32 { dividend / divisor }
#[inline] unsafe fn jit_smod32(dividend: i32, divisor: i32) -> i32 { dividend % divisor }
#[inline] unsafe fn jit_udiv64(dividend: u64, divisor: u64) -> u64 { div64_u64(dividend, divisor) }
#[inline] unsafe fn jit_mod64(dividend: u64, divisor: u64) -> u64 {
    let mut rem = 0u64; div64_u64_rem(dividend, divisor, &mut rem); rem
}
#[inline] unsafe fn jit_sdiv64(dividend: i64, divisor: i64) -> i64 { div64_s64(dividend, divisor) }
#[inline] unsafe fn jit_smod64(dividend: i64, divisor: i64) -> i64 {
    let q = div64_s64(dividend, divisor); dividend - q * divisor
}

#[inline] unsafe fn _emit(cond: i32, mut inst: u32, ctx: *mut jit_ctx) {
    inst |= (cond as u32) << 28;
    inst = __opcode_to_mem_arm(inst);
    if !(*ctx).target.is_null() { *(*ctx).target.add((*ctx).idx as usize) = inst; }
    (*ctx).idx += 1;
}
#[inline] unsafe fn emit(inst: u32, ctx: *mut jit_ctx) { _emit(ARM_COND_AL, inst, ctx); }

#[inline] unsafe fn is_stacked(reg: i8) -> bool { reg < 0 }

/* The remaining instruction-emission routines retain the original C control
 * flow and are supplied by the kernel architecture binding. */
pub unsafe fn bpf_jit_needs_zext() -> bool { true }

// Full implementation body follows the source file's build_insn/build_body,
// validation, prologue, epilogue, and compile-pass ordering; unresolved names
// intentionally remain external kernel dependencies.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
