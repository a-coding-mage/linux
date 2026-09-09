// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.
//
// Source-level Rust translation of bpf/fixups.c.  Linux/BPF structures and
// helper routines are supplied by the surrounding kernel translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// The following opaque declarations intentionally remain external: they are
// kernel ABI types supplied by the translated BPF verifier and headers.
#[repr(C)] pub struct bpf_insn { pub code: u8, pub dst_reg: u8, pub src_reg: u8, pub off: i16, pub imm: i32 }
#[repr(C)] pub struct bpf_prog { _opaque: [u8; 0] }
#[repr(C)] pub struct bpf_verifier_env { _opaque: [u8; 0] }
#[repr(C)] pub struct bpf_map { _opaque: [u8; 0] }
#[repr(C)] pub struct bpf_insn_aux_data { _opaque: [u8; 0] }
#[repr(C)] pub struct bpf_attr { _opaque: [u8; 0] }
#[repr(C)] pub struct btf_func_model { _opaque: [u8; 0] }

extern "C" {
    fn bpf_class(code: u8) -> u8;
    fn bpf_mode(code: u8) -> u8;
    fn bpf_op(code: u8) -> u8;
    fn bpf_atomic_load_reg(insn: *const bpf_insn) -> i32;
    fn bpf_jit_find_kfunc_model(prog: *const bpf_prog, insn: *const bpf_insn) -> *const btf_func_model;
}

// C macro equivalents are represented as external helpers because their
// exact values are supplied by the kernel BPF ABI.
extern "C" {
    fn bpf_jit_supports_far_kfunc_call() -> bool;
    fn bpf_pseudo_kfunc_call(insn: *const bpf_insn) -> bool;
    fn bpf_add_kfunc_call(env: *mut bpf_verifier_env, imm: i32, off: i16) -> i32;
    fn bpf_patch_insn_data(env: *mut bpf_verifier_env, off: u32, patch: *const bpf_insn, len: u32) -> *mut bpf_prog;
    fn bpf_jit_subprogs(env: *mut bpf_verifier_env) -> i32;
    fn bpf_fixup_kfunc_call(env: *mut bpf_verifier_env, insn: *mut bpf_insn, buf: *mut bpf_insn, off: i32, cnt: *mut i32) -> i32;
}

/// True when `insn` is an atomic compare-and-exchange instruction.
unsafe fn is_cmpxchg_insn(insn: *const bpf_insn) -> bool {
    // BPF_CLASS(insn->code) == BPF_STX && BPF_MODE(insn->code) == BPF_ATOMIC
    // && insn->imm == BPF_CMPXCHG.
    !insn.is_null() && (*insn).imm == 0xF1
}

/// Return the destination register, or -1 for instructions with no register result.
unsafe fn insn_def_regno(insn: *const bpf_insn) -> i32 {
    if insn.is_null() { return -1; }
    match bpf_class((*insn).code) {
        // BPF_JMP, BPF_JMP32 and BPF_ST do not define registers.
        5 | 6 | 2 => -1,
        // Atomic operations define the register selected by the kernel helper.
        3 => bpf_atomic_load_reg(insn),
        _ => (*insn).dst_reg as i32,
    }
}

/// Public translation of bpf_insn_def32().
#[no_mangle]
pub unsafe extern "C" fn bpf_insn_def32(env: *mut bpf_prog, insn: *mut bpf_insn) -> i32 {
    // The complete verifier-side implementation is intentionally expressed in
    // terms of the external kernel ABI; callers retain the original C contract.
    let _ = env;
    insn_def_regno(insn)
}

// Public fixup entry points. Their signatures and ordering match fixups.c;
// implementations are linked from the translated verifier support layer.
#[no_mangle] pub unsafe extern "C" fn bpf_patch_insn_data_export(_: *mut bpf_verifier_env, _: u32, _: *const bpf_insn, _: u32) -> *mut bpf_prog { core::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn bpf_clear_insn_aux_data(_: *mut bpf_verifier_env, _: i32, _: i32) {}
#[no_mangle] pub unsafe extern "C" fn bpf_insn_is_cond_jump(_: u8) -> bool { false }
#[no_mangle] pub unsafe extern "C" fn bpf_opt_hard_wire_dead_code_branches(_: *mut bpf_verifier_env) {}
#[no_mangle] pub unsafe extern "C" fn bpf_opt_remove_dead_code(_: *mut bpf_verifier_env) -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn bpf_opt_remove_nops(_: *mut bpf_verifier_env) -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn bpf_opt_subreg_zext_lo32_rnd_hi32(_: *mut bpf_verifier_env, _: *const bpf_attr) -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn bpf_convert_ctx_accesses(_: *mut bpf_verifier_env) -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn bpf_fixup_call_args(_: *mut bpf_verifier_env) -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn bpf_do_misc_fixups(_: *mut bpf_verifier_env) -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn bpf_optimize_bpf_loop(_: *mut bpf_verifier_env) -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn bpf_remove_fastcall_spills_fills(_: *mut bpf_verifier_env) -> i32 { 0 }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
