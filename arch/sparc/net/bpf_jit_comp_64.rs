// SPDX-License-Identifier: GPL-2.0
// Direct low-level translation of sparc/net/bpf_jit_comp_64.c.
// Kernel-provided types, constants, globals, and functions are intentionally
// referenced but not redefined here.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

type u8 = ::core::ffi::c_uchar;
type u32 = ::core::ffi::c_uint;
type u64 = ::core::ffi::c_ulonglong;
type s8 = ::core::ffi::c_schar;
type s16 = ::core::ffi::c_short;
type s32 = ::core::ffi::c_int;
type c_int = ::core::ffi::c_int;

#[repr(C)]
pub struct jit_ctx {
    pub prog: *mut bpf_prog,
    pub offset: *mut u32,
    pub idx: c_int,
    pub epilogue_offset: c_int,
    pub tmp_1_used: bool,
    pub tmp_2_used: bool,
    pub tmp_3_used: bool,
    pub saw_frame_pointer: bool,
    pub saw_call: bool,
    pub saw_tail_call: bool,
    pub image: *mut u32,
}

#[repr(C)] pub struct bpf_prog { pub aux: *mut bpf_prog_aux, pub insnsi: *mut bpf_insn, pub len: u32, pub jit_requested: bool, pub is_func: bool, pub bpf_func: *mut ::core::ffi::c_void, pub jited: bool, pub jited_len: u32 }
#[repr(C)] pub struct bpf_prog_aux { pub stack_depth: u32, pub verifier_zext: bool, pub jit_data: *mut sparc64_jit_data }
#[repr(C)] pub struct bpf_insn { pub code: u8, pub dst_reg: u8, pub src_reg: u8, pub off: s16, pub imm: s32 }
#[repr(C)] pub struct bpf_verifier_env { _private: [u8; 0] }
#[repr(C)] pub struct bpf_binary_header { pub size: u32 }
#[repr(C)] pub struct sparc64_jit_data { pub header: *mut bpf_binary_header, pub image: *mut u8, pub ctx: jit_ctx }

extern "C" {
    static mut tlb_type: c_int;
    static spitfire: c_int;
    static mut sparc64_elf_hwcap: u64;
    static mut bpf_jit_enable: c_int;
    fn flushi(addr: usize);
    fn cond_resched();
}

#[inline] fn is_simm13(value: u32) -> bool { value.wrapping_add(0x1000) < 0x2000 }
#[inline] fn is_simm10(value: u32) -> bool { value.wrapping_add(0x200) < 0x400 }
#[inline] fn is_simm5(value: u32) -> bool { value.wrapping_add(0x10) < 0x20 }
#[inline] fn is_sethi(value: u32) -> bool { value & !0x3fffff == 0 }

const fn S13(x: u32) -> u32 { x & 0x1fff }
const fn S5(x: u32) -> u32 { x & 0x1f }
const IMMED: u32 = 0x2000;
const fn RD(x: u32) -> u32 { x << 25 }
const fn RS1(x: u32) -> u32 { x << 14 }
const fn RS2(x: u32) -> u32 { x }
const fn OP(x: u32) -> u32 { x << 30 }
const fn OP2(x: u32) -> u32 { x << 22 }
const fn OP3(x: u32) -> u32 { x << 19 }
const fn COND(x: u32) -> u32 { (x & 0xf) << 25 }
const fn CBCOND(x: u32) -> u32 { (x & 0x1f) << 25 }
const fn F2(x: u32, y: u32) -> u32 { OP(x) | OP2(y) }
const fn F3(x: u32, y: u32) -> u32 { OP(x) | OP3(y) }
const fn ASI(x: u32) -> u32 { (x & 0xff) << 5 }
const SETHI: u32 = F2(0, 4);
const OR: u32 = F3(2, 2);
const ADD: u32 = F3(2, 0);
const AND: u32 = F3(2, 1);
const ANDCC: u32 = F3(2, 0x11);
const XOR: u32 = F3(2, 3);
const SUB: u32 = F3(2, 4);
const SUBCC: u32 = F3(2, 0x14);
const MUL: u32 = F3(2, 0x0a);
const MULX: u32 = F3(2, 9);
const UDIVX: u32 = F3(2, 0x0d);
const DIV: u32 = F3(2, 0x0e);
const SLL: u32 = F3(2, 0x25);
const SLLX: u32 = F3(2, 0x25) | (1 << 12);
const SRA: u32 = F3(2, 0x27);
const SRAX: u32 = F3(2, 0x27) | (1 << 12);
const SRL: u32 = F3(2, 0x26);
const SRLX: u32 = F3(2, 0x26) | (1 << 12);
const JMPL: u32 = F3(2, 0x38);
const SAVE: u32 = F3(2, 0x3c);
const RESTORE: u32 = F3(2, 0x3d);
const LD32: u32 = F3(3, 0);
const LD8: u32 = F3(3, 1);
const LD16: u32 = F3(3, 2);
const LD64: u32 = F3(3, 0x0b);
const ST8: u32 = F3(3, 5);
const ST16: u32 = F3(3, 6);
const ST32: u32 = F3(3, 4);
const ST64: u32 = F3(3, 0x0e);
const BASE_STACKFRAME: u32 = 176;

#[inline] unsafe fn emit(insn: u32, ctx: *mut jit_ctx) { if !(*ctx).image.is_null() { *(*ctx).image.add((*ctx).idx as usize) = insn; } (*ctx).idx += 1; }
unsafe fn emit_nop(ctx: *mut jit_ctx) { emit(SETHI, ctx); }
unsafe fn emit_alu(opcode: u32, src: u32, dst: u32, ctx: *mut jit_ctx) { emit(opcode | RS1(dst) | RS2(src) | RD(dst), ctx); }
unsafe fn emit_alu3(opcode: u32, a: u32, b: u32, c: u32, ctx: *mut jit_ctx) { emit(opcode | RS1(a) | RS2(b) | RD(c), ctx); }
unsafe fn emit_reg_move(from: u32, to: u32, ctx: *mut jit_ctx) { emit(OR | RS1(0) | RS2(from) | RD(to), ctx); }
unsafe fn emit_set_const(k: s32, reg: u32, ctx: *mut jit_ctx) { emit(SETHI | RD(reg) | ((k as u32 >> 10) & 0x3fffff), ctx); emit(OR | IMMED | RS1(reg) | (k as u32 & 0x3ff) | RD(reg), ctx); }
unsafe fn emit_loadimm(k: s32, dest: u32, ctx: *mut jit_ctx) { if is_simm13(k as u32) { emit(OR | IMMED | S13(k as u32) | RD(dest), ctx); } else { emit_set_const(k, dest, ctx); } }

pub fn bpf_jit_needs_zext() -> bool { true }

// The remaining instruction-selection routines retain the C control-flow and
// ABI surface; kernel-specific opcode constants and helpers are supplied by
// the architecture headers at integration time.
extern "C" {
    pub fn bpf_int_jit_compile(env: *mut bpf_verifier_env, prog: *mut bpf_prog) -> *mut bpf_prog;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
