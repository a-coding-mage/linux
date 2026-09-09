/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * arch/arm/probes/decode-thumb.h
 *
 * Copyright 2013 Linaro Ltd.
 * Written by: David A. Long
 */

// Dependency intent: declarations from "decode.h" are supplied by other files.

/*
 * True if current instruction is in an IT block.
 */
#[inline]
pub const fn in_it_block(cpsr: u32) -> bool {
    (cpsr & 0x0600_0c00) != 0x0000_0000
}

/*
 * Return the condition code to check for the currently executing instruction.
 * This is in ITSTATE<7:4> which is in CPSR<15:12> but is only valid if
 * in_it_block returns true.
 */
#[inline]
pub const fn current_cond(cpsr: u32) -> u32 {
    (cpsr >> 12) & 0xf
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum probes_t32_action {
    PROBES_T32_EMULATE_NONE,
    PROBES_T32_SIMULATE_NOP,
    PROBES_T32_LDMSTM,
    PROBES_T32_LDRDSTRD,
    PROBES_T32_TABLE_BRANCH,
    PROBES_T32_TST,
    PROBES_T32_CMP,
    PROBES_T32_MOV,
    PROBES_T32_ADDSUB,
    PROBES_T32_LOGICAL,
    PROBES_T32_ADDWSUBW_PC,
    PROBES_T32_ADDWSUBW,
    PROBES_T32_MOVW,
    PROBES_T32_SAT,
    PROBES_T32_BITFIELD,
    PROBES_T32_SEV,
    PROBES_T32_WFE,
    PROBES_T32_MRS,
    PROBES_T32_BRANCH_COND,
    PROBES_T32_BRANCH,
    PROBES_T32_PLDI,
    PROBES_T32_LDR_LIT,
    PROBES_T32_LDRSTR,
    PROBES_T32_SIGN_EXTEND,
    PROBES_T32_MEDIA,
    PROBES_T32_REVERSE,
    PROBES_T32_MUL_ADD,
    PROBES_T32_MUL_ADD2,
    PROBES_T32_MUL_ADD_LONG,
    NUM_PROBES_T32_ACTIONS,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum probes_t16_action {
    PROBES_T16_ADD_SP,
    PROBES_T16_CBZ,
    PROBES_T16_SIGN_EXTEND,
    PROBES_T16_PUSH,
    PROBES_T16_POP,
    PROBES_T16_SEV,
    PROBES_T16_WFE,
    PROBES_T16_IT,
    PROBES_T16_CMP,
    PROBES_T16_ADDSUB,
    PROBES_T16_LOGICAL,
    PROBES_T16_BLX,
    PROBES_T16_HIREGOPS,
    PROBES_T16_LDR_LIT,
    PROBES_T16_LDRHSTRH,
    PROBES_T16_LDRSTR,
    PROBES_T16_ADR,
    PROBES_T16_LDMSTM,
    PROBES_T16_BRANCH_COND,
    PROBES_T16_BRANCH,
    NUM_PROBES_T16_ACTIONS,
}

unsafe extern "C" {
    pub static probes_decode_thumb32_table: [decode_item; 0];
    pub static probes_decode_thumb16_table: [decode_item; 0];

    pub fn thumb16_probes_decode_insn(
        insn: probes_opcode_t,
        asi: *mut arch_probes_insn,
        emulate: bool,
        actions: *const decode_action,
        checkers: *const *const decode_checker,
    ) -> probes_insn;

    pub fn thumb32_probes_decode_insn(
        insn: probes_opcode_t,
        asi: *mut arch_probes_insn,
        emulate: bool,
        actions: *const decode_action,
        checkers: *const *const decode_checker,
    ) -> probes_insn;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
