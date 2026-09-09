/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm/include/asm/probes.h
 *
 * Original contents copied from arch/arm/include/asm/kprobes.h
 * which contains the following notice...
 *
 * Copyright (C) 2006, 2007 Motorola Inc.
 */

/* The C header guard is omitted in Rust; module inclusion provides this role. */

pub type probes_opcode_t = u32;

pub struct arch_probes_insn;
pub struct pt_regs;

pub type probes_insn_handler_t = unsafe extern "C" fn(
    probes_opcode_t,
    *mut arch_probes_insn,
    *mut pt_regs,
);
pub type probes_check_cc = unsafe extern "C" fn(::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
pub type probes_insn_singlestep_t = unsafe extern "C" fn(
    probes_opcode_t,
    *mut arch_probes_insn,
    *mut pt_regs,
);
pub type probes_insn_fn_t = unsafe extern "C" fn();

/* Architecture specific copy of original instruction. */
#[repr(C)]
pub struct arch_probes_insn {
    pub insn: *mut probes_opcode_t,
    pub insn_handler: Option<probes_insn_handler_t>,
    pub insn_check_cc: Option<probes_check_cc>,
    pub insn_singlestep: Option<probes_insn_singlestep_t>,
    pub insn_fn: Option<probes_insn_fn_t>,
    pub stack_space: ::core::ffi::c_int,
    pub register_usage_flags: ::core::ffi::c_ulong,
    pub kprobe_direct_exec: bool,
}

/*
 * We assume one instruction can consume at most 64 bytes stack, which is
 * 'push {r0-r15}'. Instructions consume more or unknown stack space like
 * 'str r0, [sp, #-80]' and 'str r0, [sp, r1]' should be prohibit to probe.
 */
pub const MAX_STACK_SIZE: ::core::ffi::c_int = 64;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
