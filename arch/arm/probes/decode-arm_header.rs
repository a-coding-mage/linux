/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * arch/arm/probes/decode-arm.h
 *
 * Copyright 2013 Linaro Ltd.
 * Written by: David A. Long
 */

// C header guard: _ARM_KERNEL_PROBES_ARM_H
// Dependency supplied by decode.h.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum probes_arm_action {
    PROBES_PRELOAD_IMM,
    PROBES_PRELOAD_REG,
    PROBES_BRANCH_IMM,
    PROBES_BRANCH_REG,
    PROBES_MRS,
    PROBES_CLZ,
    PROBES_SATURATING_ARITHMETIC,
    PROBES_MUL1,
    PROBES_MUL2,
    PROBES_SWP,
    PROBES_LDRSTRD,
    PROBES_LOAD,
    PROBES_STORE,
    PROBES_LOAD_EXTRA,
    PROBES_STORE_EXTRA,
    PROBES_MOV_IP_SP,
    PROBES_DATA_PROCESSING_REG,
    PROBES_DATA_PROCESSING_IMM,
    PROBES_MOV_HALFWORD,
    PROBES_SEV,
    PROBES_WFE,
    PROBES_SATURATE,
    PROBES_REV,
    PROBES_MMI,
    PROBES_PACK,
    PROBES_EXTEND,
    PROBES_EXTEND_ADD,
    PROBES_MUL_ADD_LONG,
    PROBES_MUL_ADD,
    PROBES_BITFIELD,
    PROBES_BRANCH,
    PROBES_LDMSTM,
    NUM_PROBES_ARM_ACTIONS,
}

// __kprobes is a C-side annotation with no direct Rust equivalent.
extern "C" {
    pub fn simulate_bbl(
        opcode: probes_opcode_t,
        asi: *mut arch_probes_insn,
        regs: *mut pt_regs,
    );
    pub fn simulate_blx1(
        opcode: probes_opcode_t,
        asi: *mut arch_probes_insn,
        regs: *mut pt_regs,
    );
    pub fn simulate_blx2bx(
        opcode: probes_opcode_t,
        asi: *mut arch_probes_insn,
        regs: *mut pt_regs,
    );
    pub fn simulate_mrs(
        opcode: probes_opcode_t,
        asi: *mut arch_probes_insn,
        regs: *mut pt_regs,
    );
    pub fn simulate_mov_ipsp(
        opcode: probes_opcode_t,
        asi: *mut arch_probes_insn,
        regs: *mut pt_regs,
    );

    pub static probes_decode_arm_table: [decode_item; 0];

    pub fn arm_probes_decode_insn(
        opcode: probes_opcode_t,
        asi: *mut arch_probes_insn,
        emulate: bool,
        actions: *const decode_action,
        checkers: *const *const decode_checker,
    ) -> probes_insn;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
