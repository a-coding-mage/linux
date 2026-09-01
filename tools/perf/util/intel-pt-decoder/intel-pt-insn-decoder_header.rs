/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * intel_pt_insn_decoder.h: Intel Processor Trace support
 * Copyright (c) 2013-2014, Intel Corporation.
 */

// C header dependencies: <stddef.h>, <stdint.h>

use std::os::raw::{c_char, c_int, c_uchar};

pub const INTEL_PT_INSN_DESC_MAX: usize = 32;
pub const INTEL_PT_INSN_BUF_SZ: usize = 16;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum intel_pt_insn_op {
    INTEL_PT_OP_OTHER,
    INTEL_PT_OP_CALL,
    INTEL_PT_OP_RET,
    INTEL_PT_OP_JCC,
    INTEL_PT_OP_JMP,
    INTEL_PT_OP_LOOP,
    INTEL_PT_OP_IRET,
    INTEL_PT_OP_INT,
    INTEL_PT_OP_SYSCALL,
    INTEL_PT_OP_SYSRET,
    INTEL_PT_OP_VMENTRY,
    INTEL_PT_OP_ERETS,
    INTEL_PT_OP_ERETU,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum intel_pt_insn_branch {
    INTEL_PT_BR_NO_BRANCH,
    INTEL_PT_BR_INDIRECT,
    INTEL_PT_BR_CONDITIONAL,
    INTEL_PT_BR_UNCONDITIONAL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pt_insn {
    pub op: intel_pt_insn_op,
    pub branch: intel_pt_insn_branch,
    pub emulated_ptwrite: bool,
    pub length: c_int,
    pub rel: i32,
    pub buf: [c_uchar; INTEL_PT_INSN_BUF_SZ],
}

unsafe extern "C" {
    pub fn intel_pt_get_insn(
        buf: *const c_uchar,
        len: usize,
        x86_64: c_int,
        intel_pt_insn: *mut intel_pt_insn,
    ) -> c_int;

    pub fn intel_pt_insn_name(op: intel_pt_insn_op) -> *const c_char;

    pub fn intel_pt_insn_desc(
        intel_pt_insn: *const intel_pt_insn,
        buf: *mut c_char,
        buf_len: usize,
    ) -> c_int;

    pub fn intel_pt_insn_type(op: intel_pt_insn_op) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
