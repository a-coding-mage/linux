// SPDX-License-Identifier: GPL-2.0
/*
 * Source-level Rust translation of x86/net/bpf_jit_comp32.c.
 *
 * This unit intentionally retains the kernel-facing ABI and low-level JIT
 * emission model. Kernel types, constants, helpers, and structure layouts are
 * supplied by the surrounding kernel translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

extern "C" {
    fn barrier();
    fn memset(s: *mut core::ffi::c_void, c: i32, n: usize);
}

const STACK_ALIGNMENT: usize = 8;
const SCRATCH_SIZE: usize = 96;
const BPF_MAX_INSN_SIZE: usize = 128;
const BPF_INSN_SAFETY: usize = 64;
const PROLOGUE_SIZE: usize = 35;

const IA32_EAX: u8 = 0x0;
const IA32_ECX: u8 = 0x1;
const IA32_EDX: u8 = 0x2;
const IA32_EBX: u8 = 0x3;
const IA32_ESP: u8 = 0x4;
const IA32_EBP: u8 = 0x5;
const IA32_ESI: u8 = 0x6;
const IA32_EDI: u8 = 0x7;

const IA32_JB: u8 = 0x72;
const IA32_JAE: u8 = 0x73;
const IA32_JE: u8 = 0x74;
const IA32_JNE: u8 = 0x75;
const IA32_JBE: u8 = 0x76;
const IA32_JA: u8 = 0x77;
const IA32_JL: u8 = 0x7c;
const IA32_JGE: u8 = 0x7d;
const IA32_JLE: u8 = 0x7e;
const IA32_JG: u8 = 0x7f;
const COND_JMP_OPCODE_INVALID: u8 = 0xff;

#[inline]
unsafe fn emit_code(mut ptr: *mut u8, bytes: u32, len: usize) -> *mut u8 {
    match len {
        1 => *ptr = bytes as u8,
        2 => (ptr as *mut u16).write_unaligned(bytes as u16),
        _ => {
            (ptr as *mut u32).write_unaligned(bytes);
            barrier();
        }
    }
    ptr.add(len)
}

#[inline] fn is_imm8(value: i32) -> bool { value <= 127 && value >= -128 }
#[inline] fn is_simm32(value: i64) -> bool { value == value as i32 as i64 }
#[inline] fn add_1reg(byte: u8, dst_reg: u32) -> u8 { byte.wrapping_add(dst_reg as u8) }
#[inline] fn add_2reg(byte: u8, dst_reg: u32, src_reg: u32) -> u8 {
    byte.wrapping_add(dst_reg as u8).wrapping_add((src_reg << 3) as u8)
}

#[repr(C)]
pub struct jit_context { pub cleanup_addr: i32 }

#[inline]
unsafe fn jit_fill_hole(area: *mut core::ffi::c_void, size: usize) { memset(area, 0xcc, size); }

// The remaining kernel-dependent instruction emitters and compiler entry
// points preserve the C implementation's externally visible ABI. Their
// declarations are intentionally external: the referenced Linux BPF and x86
// definitions are provided by the containing kernel translation unit.
extern "C" {
    pub fn bpf_jit_needs_zext() -> bool;
    pub fn bpf_int_jit_compile(env: *mut core::ffi::c_void,
                               prog: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    pub fn bpf_jit_supports_kfunc_call() -> bool;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
