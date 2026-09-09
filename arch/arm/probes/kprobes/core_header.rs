/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm/kernel/kprobes.h
 *
 * Copyright (C) 2011 Jon Medhurst <tixy@yxit.co.uk>.
 *
 * Some contents moved here from arch/arm/include/asm/kprobes.h which is
 * Copyright (C) 2006, 2007 Motorola Inc.
 */

// C header guard: _ARM_KERNEL_KPROBES_H
// Dependencies: <asm/kprobes.h> and "../decode.h"

/*
 * These undefined instructions must be unique and
 * reserved solely for kprobes' use.
 */
pub const KPROBE_ARM_BREAKPOINT_INSTRUCTION: u32 = 0x07f001f8;
pub const KPROBE_THUMB16_BREAKPOINT_INSTRUCTION: u16 = 0xde18;
pub const KPROBE_THUMB32_BREAKPOINT_INSTRUCTION: u32 = 0xf7f0a018;

pub unsafe extern "C" fn kprobes_remove_breakpoint(
    addr: *mut core::ffi::c_void,
    insn: core::ffi::c_uint,
);

// The `__kprobes` annotation is a build/link-time attribute in the C source.
pub unsafe extern "C" fn kprobe_decode_ldmstm(
    insn: kprobe_opcode_t,
    asi: *mut arch_probes_insn,
    h: *const decode_header,
) -> probes_insn;

pub type kprobe_decode_insn_t = unsafe extern "C" fn(
    probes_opcode_t,
    *mut arch_probes_insn,
    bool,
    *const decode_action,
    *const *const decode_checker,
) -> probes_insn;

#[cfg(CONFIG_THUMB2_KERNEL)]
extern "C" {
    pub static kprobes_t32_actions: [decode_action; 0];
    pub static kprobes_t16_actions: [decode_action; 0];
    pub static kprobes_t32_checkers: [*const decode_checker; 0];
    pub static kprobes_t16_checkers: [*const decode_checker; 0];
}

#[cfg(not(CONFIG_THUMB2_KERNEL))]
extern "C" {
    pub static kprobes_arm_actions: [decode_action; 0];
    pub static kprobes_arm_checkers: [*const decode_checker; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
