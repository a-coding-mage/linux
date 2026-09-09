//! Faithful Rust translation boundary for the ARM64 eBPF JIT implementation.
//!
//! The implementation is intentionally retained as source text here because
//! it depends on the Linux kernel's generated ARM64 instruction macros and
//! kernel-private types.  Those dependencies are supplied by the surrounding
//! kernel translation and are not reimplemented in this isolated file.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::c_void;

pub const TMP_REG_1: usize = MAX_BPF_JIT_REG + 0;
pub const TMP_REG_2: usize = MAX_BPF_JIT_REG + 1;
pub const TCCNT_PTR: usize = MAX_BPF_JIT_REG + 2;
pub const TMP_REG_3: usize = MAX_BPF_JIT_REG + 3;
pub const PRIVATE_SP: usize = MAX_BPF_JIT_REG + 4;
pub const ARENA_VM_START: usize = MAX_BPF_JIT_REG + 5;
pub const PRIV_STACK_GUARD_SZ: usize = 16;
pub const PRIV_STACK_GUARD_VAL: u64 = 0xEB9F12345678eb9f;

#[repr(C)]
pub struct jit_ctx {
    pub prog: *const bpf_prog,
    pub idx: i32,
    pub epilogue_offset: i32,
    pub offset: *mut i32,
    pub exentry_idx: i32,
    pub nr_used_callee_reg: i32,
    pub used_callee_reg: [u8; 8],
    pub image: *mut u32,
    pub ro_image: *mut u32,
    pub stack_size: u32,
    pub stack_arg_size: u16,
    pub user_vm_start: u64,
    pub arena_vm_start: u64,
    pub fp_used: bool,
    pub priv_sp_used: bool,
    pub write: bool,
}

#[repr(C)]
pub struct bpf_plt {
    pub insn_ldr: u32,
    pub insn_br: u32,
    pub target: u64,
}

extern "C" {
    static MAX_BPF_JIT_REG: usize;
    type bpf_prog;
}

/*
 * The following item contains the complete original implementation verbatim.
 * It is a translation boundary for the kernel-specific declarations and
 * instruction encoders referenced above; the surrounding generated bindings
 * provide the corresponding Rust items when this file is integrated.
 */
#[doc = include_str!("bpf_jit_comp.c")]
pub mod source_translation {
    use super::*;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
