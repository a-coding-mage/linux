/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// Translated from bpf/diagnostics.h. Linux kernel includes and __printf
// annotations are represented by the Rust declarations below.

use core::ffi::{c_char, c_int, c_void, VaList};

pub type s16 = i16;
pub type s64 = i64;
pub type u32 = u32;
pub type u64 = u64;
pub type size_t = usize;
pub type bool = core::ffi::c_uchar;

#[repr(C)]
pub struct bpf_reference_state {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_func_state {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_reg_state {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_verifier_env {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_verifier_state {
    _private: [u8; 0],
}
#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

// Supplied by the Linux BPF declarations.
pub type bpf_reg_type = c_int;

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum bpf_diag_mod_reason {
    BPF_DIAG_MOD_WRITE,
    BPF_DIAG_MOD_SPILL,
    BPF_DIAG_MOD_VAR_WRITE,
    BPF_DIAG_MOD_REF_RELEASE,
    BPF_DIAG_MOD_PKT_DATA_CHANGE,
    BPF_DIAG_MOD_NON_OWN_REF,
    BPF_DIAG_MOD_CALLER_SAVED,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum bpf_diag_context_kind {
    BPF_DIAG_CONTEXT_NONE,
    BPF_DIAG_CONTEXT_RCU,
    BPF_DIAG_CONTEXT_PREEMPT,
    BPF_DIAG_CONTEXT_IRQ,
    BPF_DIAG_CONTEXT_LOCK,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum bpf_diag_invalid_deref_kind {
    BPF_DIAG_DEREF_SCALAR,
    BPF_DIAG_DEREF_NULLABLE_PTR,
    BPF_DIAG_DEREF_MODIFIED_PTR,
    BPF_DIAG_DEREF_INVALID_PTR,
}

extern "C" {
    pub fn bpf_diag_fmt_s64_sum(env: *mut bpf_verifier_env, value: s64, addend: c_int) -> *const c_char;
    pub fn bpf_diag_enabled(env: *const bpf_verifier_env) -> bool;
    pub fn bpf_diag_init(env: *mut bpf_verifier_env) -> c_int;
    pub fn bpf_diag_init_frame(env: *mut bpf_verifier_env, state: *mut bpf_func_state);
    pub fn bpf_diag_fmt_buf(env: *mut bpf_verifier_env, size: size_t) -> *mut c_char;
    pub fn bpf_diag_vfmt(env: *mut bpf_verifier_env, fmt: *const c_char, args: VaList<'_>) -> *const c_char;
    pub fn bpf_diag_fmt(env: *mut bpf_verifier_env, fmt: *const c_char, ... ) -> *const c_char;
    pub fn bpf_diag_fmt_btf_type(env: *mut bpf_verifier_env, btf: *const btf, type_id: u32) -> *const c_char;
    pub fn bpf_diag_reg_type_plain(env: *mut bpf_verifier_env, type_: bpf_reg_type) -> *const c_char;
    pub fn bpf_diag_event_log_save(env: *mut bpf_verifier_env) -> u64;
    pub fn bpf_diag_event_log_restore(env: *mut bpf_verifier_env, log_pos: u64);
    pub fn bpf_diag_irq_depth(state: *const bpf_verifier_state) -> u32;
    pub fn bpf_diag_free(env: *mut bpf_verifier_env);
    pub fn bpf_diag_register_type(env: *mut bpf_verifier_env, insn_idx: u32, regno: c_int, problem: *const c_char, reason: *const c_char, suggestion: *const c_char);
    pub fn bpf_diag_invalid_deref(env: *mut bpf_verifier_env, insn_idx: u32, regno: c_int, reg_name: *const c_char, reg: *const bpf_reg_state, kind: bpf_diag_invalid_deref_kind, offset: s64);
    pub fn bpf_diag_unreadable_reg(env: *mut bpf_verifier_env, insn_idx: u32, regno: c_int);
    pub fn bpf_diag_stack_arg_uninit(env: *mut bpf_verifier_env, insn_idx: u32, nargs: c_int, stack_arg_slot: c_int, callee_name: *const c_char, arg_name: *const c_char);
    pub fn bpf_diag_memory(env: *mut bpf_verifier_env, insn_idx: u32, problem: *const c_char, reason: *const c_char, suggestion: *const c_char);
    pub fn bpf_diag_mem_bounds(env: *mut bpf_verifier_env, insn_idx: u32, regno: c_int, reg_name: *const c_char, type_name: *const c_char, proof: *const c_char, off: c_int, size: c_int, mem_size: u32, reg: *const bpf_reg_state);
    pub fn bpf_diag_res(env: *mut bpf_verifier_env, insn_idx: u32, problem: *const c_char, reason: *const c_char, suggestion: *const c_char);
    pub fn bpf_diag_lock(env: *mut bpf_verifier_env, insn_idx: u32, problem: *const c_char, reason: *const c_char, suggestion: *const c_char, active_lock: *const bpf_reference_state);
    pub fn bpf_diag_irq(env: *mut bpf_verifier_env, insn_idx: u32, problem: *const c_char, reason: *const c_char, suggestion: *const c_char, depth: u32);
    pub fn bpf_diag_leak(env: *mut bpf_verifier_env, ref_id: u32, alloc_insn: u32, fail_insn: u32);
    pub fn bpf_diag_call_type(env: *mut bpf_verifier_env, insn_idx: u32, argno: c_int, regno: c_int, stack_arg_slot: c_int, call_name: *const c_char, arg_name: *const c_char, reason: *const c_char, suggestion: *const c_char);
    pub fn bpf_diag_ctx_forbidden(env: *mut bpf_verifier_env, insn_idx: u32, operation: *const c_char, suggestion: *const c_char);
    pub fn bpf_diag_ctx_active(env: *mut bpf_verifier_env, insn_idx: u32, operation: *const c_char, ctx_kind: bpf_diag_context_kind, suggestion: *const c_char);
    pub fn bpf_diag_ctx_required(env: *mut bpf_verifier_env, insn_idx: u32, operation: *const c_char, ctx_kind: bpf_diag_context_kind, suggestion: *const c_char);
    pub fn bpf_diag_ctx_underflow(env: *mut bpf_verifier_env, insn_idx: u32, operation: *const c_char, ctx_kind: bpf_diag_context_kind, suggestion: *const c_char);
    pub fn bpf_diag_program_structure(env: *mut bpf_verifier_env, insn_idx: u32, problem: *const c_char, suggestion: *const c_char, reason_fmt: *const c_char, ...);
    pub fn bpf_diag_policy(env: *mut bpf_verifier_env, insn_idx: u32, operation: *const c_char, reason: *const c_char, suggestion: *const c_char);
    pub fn bpf_diag_record_branch(env: *mut bpf_verifier_env, insn_idx: u32, cond_true: bool);
    pub fn bpf_diag_mod_begin(env: *mut bpf_verifier_env, reg: *const bpf_reg_state, origin: *const bpf_reg_state, reason: bpf_diag_mod_reason);
    pub fn bpf_diag_mod_end(env: *mut bpf_verifier_env);
    pub fn bpf_diag_record_scrub(env: *mut bpf_verifier_env, reg: *const bpf_reg_state, reason: bpf_diag_mod_reason);
    pub fn bpf_diag_record_scrub_stack(env: *mut bpf_verifier_env, state: *const bpf_func_state, min_off: s16, max_off: s16, reason: bpf_diag_mod_reason);
    pub fn bpf_diag_record_ref_acquire(env: *mut bpf_verifier_env, insn_idx: u32, ref_id: u32);
    pub fn bpf_diag_record_ref_release(env: *mut bpf_verifier_env, insn_idx: u32, ref_id: u32);
    pub fn bpf_diag_record_context(env: *mut bpf_verifier_env, insn_idx: u32, ctx_kind: bpf_diag_context_kind, enter: bool, depth: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
