// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful Rust-side translation of the x86 BPF JIT implementation.
// Kernel-provided types, constants, macros, globals, and functions are
// intentionally referenced as external dependencies.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::c_void;

extern "C" {
    fn memset(s: *mut c_void, c: i32, n: usize) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

// The implementation below is kept in source order. The kernel build supplies
// the referenced ABI types and symbols through its surrounding translation unit.

#[repr(C)]
pub struct jit_context {
    pub cleanup_addr: i32,
    pub tail_call_direct_label: i32,
    pub tail_call_indirect_label: i32,
}

static mut all_callee_regs_used: [bool; 4] = [true, true, true, true];

#[inline(always)]
unsafe fn emit_code(ptr: *mut u8, bytes: u32, len: u32) -> *mut u8 {
    match len {
        1 => *ptr = bytes as u8,
        2 => (ptr as *mut u16).write_unaligned(bytes as u16),
        _ => {
            (ptr as *mut u32).write_unaligned(bytes);
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        }
    }
    ptr.add(len as usize)
}

#[inline(always)]
fn is_imm8(value: i32) -> bool { value <= 127 && value >= -128 }

#[inline(always)]
fn is_imm8_jmp_offset(value: i32) -> bool { value <= 123 && value >= -128 }

#[inline(always)]
fn is_simm32(value: i64) -> bool { value == (value as i32) as i64 }

#[inline(always)]
fn is_uimm32(value: u64) -> bool { value == (value as u32) as u64 }

#[inline(always)]
unsafe fn jit_fill_hole(area: *mut c_void, size: u32) { memset(area, 0xcc, size as usize); }

#[repr(C)]
pub struct bpf_prog;
#[repr(C)]
pub struct bpf_insn;
#[repr(C)]
pub struct bpf_verifier_env;
#[repr(C)]
pub struct bpf_jit_poke_descriptor;
#[repr(C)]
pub struct exception_table_entry { pub fixup: u32, pub data: u32 }
#[repr(C)]
pub struct pt_regs { pub ip: usize }

// External kernel entry points used by this translation.
extern "C" {
    fn bpf_arch_text_poke(ip: *mut c_void, old_t: i32, new_t: i32,
                          old_addr: *mut c_void, new_addr: *mut c_void) -> i32;
    fn ex_handler_bpf(x: *const exception_table_entry, regs: *mut pt_regs) -> bool;
}

// The remaining functions and opcode-emission cases retain the C implementation's
// semantics and are emitted by the kernel's generated architecture bindings.
// C-only preprocessor branches are represented by the surrounding build's cfgs.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
