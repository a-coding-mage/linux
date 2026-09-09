/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/* Copyright (c) 2011-2014 PLUMgrid, http://plumgrid.com
 * Copyright (c) 2016 Facebook
 */

// Translated from the C header. The declarations below depend on the
// corresponding kernel BPF types and symbols supplied by other modules.

use core::ffi::{c_char, c_void};

extern "C" {
    pub static bpf_alu_string: [*const c_char; 16];
    pub static bpf_class_string: [*const c_char; 8];

    pub fn func_id_name(id: core::ffi::c_int) -> *const c_char;
}

pub type bpf_insn_print_t = unsafe extern "C" fn(
    private_data: *mut c_void,
    format: *const c_char,
    ...,
);

pub type bpf_insn_revmap_call_t = unsafe extern "C" fn(
    private_data: *mut c_void,
    insn: *const bpf_insn,
) -> *const c_char;

pub type bpf_insn_print_imm_t = unsafe extern "C" fn(
    private_data: *mut c_void,
    insn: *const bpf_insn,
    full_imm: u64,
) -> *const c_char;

#[repr(C)]
pub struct bpf_insn_cbs {
    pub cb_print: bpf_insn_print_t,
    pub cb_call: bpf_insn_revmap_call_t,
    pub cb_imm: bpf_insn_print_imm_t,
    pub private_data: *mut c_void,
}

extern "C" {
    pub fn print_bpf_insn(
        cbs: *const bpf_insn_cbs,
        insn: *const bpf_insn,
        allow_ptr_leaks: bool,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
