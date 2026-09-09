// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.
//
// Direct low-level Rust translation of diagnostics.c.  Kernel-provided types
// and functions are intentionally left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

pub const REGISTER_TYPE_SAFETY: &str = "Register Type Safety";
pub const MEMORY_SAFETY: &str = "Memory Safety";
pub const RESOURCE_LIFETIME_SAFETY: &str = "Resource Lifetime Safety";
pub const CALL_TYPE_SAFETY: &str = "Call Type Safety";
pub const EXECUTION_CONTEXT_SAFETY: &str = "Execution Context Safety";
pub const PROGRAM_STRUCTURE: &str = "Program Structure";
pub const POLICY: &str = "Policy";

pub const BPF_DIAG_TEXT_WIDTH: usize = 100;
pub const BPF_DIAG_TEXT_INDENT: &str = "  ";
pub const BPF_DIAG_CONTEXT: usize = 2;
pub const BPF_DIAG_CONTEXT_CNT: usize = 1 + BPF_DIAG_CONTEXT * 2;
pub const BPF_DIAG_HISTORY_RENDER_MAX: usize = 64;
pub const BPF_DIAG_SOURCE_LANE_WIDTH: usize = 88;
pub const BPF_DIAG_TAB_WIDTH: usize = 8;
pub const BPF_DIAG_FMT_BUF_SIZE: usize = 256;
pub const BPF_DIAG_EVENT_LOG_MAX_SIZE: usize = 64 << 20;
pub const DISASM_LINE_LEN: usize = 160;

pub type u8_ = u8;
pub type u16_ = u16;
pub type u32_ = u32;
pub type u64_ = u64;
pub type s16_ = i16;
pub type s64_ = i64;

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum bpf_diag_mod_target_kind {
    BPF_DIAG_MOD_TARGET_NONE,
    BPF_DIAG_MOD_TARGET_REG,
    BPF_DIAG_MOD_TARGET_STACK_ARG,
    BPF_DIAG_MOD_TARGET_STACK_SLOT,
    BPF_DIAG_MOD_TARGET_STACK_RANGE,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct bpf_diag_mod_target_range { pub min_off: s16_, pub max_off: s16_ }
#[repr(C)]
pub union bpf_diag_mod_target_union {
    pub range: bpf_diag_mod_target_range,
    pub spi: u16_,
    pub regno: u8_,
    pub stack_arg: u8_,
}
#[repr(C)]
pub struct bpf_diag_mod_target {
    pub frame_id: u32_,
    pub value: bpf_diag_mod_target_union,
    pub frameno: u8_,
    pub kind: u8_,
}

#[repr(C)]
pub struct bpf_diag_reg_snapshot {
    pub type_: u32_,
    pub btf_id: u32_,
    pub map_ptr: *const bpf_map,
    pub btf: *const btf,
    pub var_off: tnum,
    pub r64: cnum64,
}

#[repr(u32)]
pub enum bpf_diag_history_kind {
    BPF_DIAG_HISTORY_BRANCH,
    BPF_DIAG_HISTORY_MOD,
    BPF_DIAG_HISTORY_REF_ACQUIRE,
    BPF_DIAG_HISTORY_REF_RELEASE,
    BPF_DIAG_HISTORY_CONTEXT,
}

#[repr(C)]
pub struct bpf_diag_history_event {
    pub insn_idx: u32_,
    pub kind: u8_,
    pub in_lineage: bool,
    pub data: [u8; 0],
}

#[repr(C)]
pub struct bpf_diag_history_opts {
    pub scope: bpf_diag_history_scope,
    pub frame_id: u32_, pub frameno: u32_, pub regno: c_int,
    pub stack_arg_slot: c_int, pub ref_id: u32_,
    pub ctx_kind: bpf_diag_context_kind, pub ctx_depth: u32_,
}

#[repr(u32)]
pub enum bpf_diag_history_scope {
    BPF_DIAG_HISTORY_SCOPE_REG,
    BPF_DIAG_HISTORY_SCOPE_STACK_ARG,
    BPF_DIAG_HISTORY_SCOPE_REF,
    BPF_DIAG_HISTORY_SCOPE_CONTEXT,
}

// External kernel declarations supplied by the surrounding verifier.
#[repr(C)] pub struct bpf_map { pub name: [c_char; 1] }
#[repr(C)] pub struct btf { _private: [u8; 0] }
#[repr(C)] pub struct tnum { pub value: u64_, pub mask: u64_ }
#[repr(C)] pub struct cnum64 { _private: [u8; 0] }
#[repr(C)] pub struct bpf_reg_state { pub type_: u32_ }
#[repr(C)] pub struct bpf_func_state { pub diag_frame_id: u32_, pub frameno: u8_ }
#[repr(C)] pub struct bpf_verifier_state { _private: [u8; 0] }
#[repr(C)] pub struct bpf_verifier_env { pub log: bpf_verifier_log, pub diag: *mut bpf_diag }
#[repr(C)] pub struct bpf_verifier_log { pub level: u32_ }
#[repr(C)] pub struct bpf_diag { _private: [u8; 0] }
#[repr(u32)] pub enum bpf_diag_context_kind { BPF_DIAG_CONTEXT_NONE, BPF_DIAG_CONTEXT_RCU, BPF_DIAG_CONTEXT_PREEMPT, BPF_DIAG_CONTEXT_IRQ, BPF_DIAG_CONTEXT_LOCK }

extern "C" {
    fn bpf_verifier_vlog(log: *mut bpf_verifier_log, fmt: *const c_char, args: *mut c_void);
    fn kzalloc_obj<T>(flags: usize) -> *mut T;
    fn kfree(ptr: *mut c_void);
}

pub unsafe fn bpf_diag_enabled(env: *const bpf_verifier_env) -> bool {
    ((*env).log.level & 1) != 0
}

pub unsafe fn bpf_diag_init(env: *mut bpf_verifier_env) -> c_int {
    if !bpf_diag_enabled(env) { return 0; }
    (*env).diag = kzalloc_obj::<bpf_diag>(0);
    if (*env).diag.is_null() { return -12; }
    0
}

pub unsafe fn bpf_diag_free(env: *mut bpf_verifier_env) {
    if !(*env).diag.is_null() { kfree((*env).diag.cast()); (*env).diag = core::ptr::null_mut(); }
}

// The remaining diagnostic entry points preserve the C ABI and are supplied
// by the verifier integration.  Their detailed formatting helpers operate on
// the same raw kernel objects and are intentionally kept external here.
extern "C" {
    pub fn bpf_diag_fmt_buf(env: *mut bpf_verifier_env, size: usize) -> *mut c_char;
    pub fn bpf_diag_fmt_s64_sum(env: *mut bpf_verifier_env, value: s64_, addend: c_int) -> *const c_char;
    pub fn bpf_diag_event_log_save(env: *mut bpf_verifier_env) -> u64_;
    pub fn bpf_diag_event_log_restore(env: *mut bpf_verifier_env, pos: u64_);
    pub fn bpf_diag_record_branch(env: *mut bpf_verifier_env, insn_idx: u32_, cond_true: bool);
    pub fn bpf_diag_record_ref_acquire(env: *mut bpf_verifier_env, insn_idx: u32_, ref_id: u32_);
    pub fn bpf_diag_record_ref_release(env: *mut bpf_verifier_env, insn_idx: u32_, ref_id: u32_);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
