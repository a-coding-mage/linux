/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 Rabin Vincent <rabin at rab.in>
 */

// C header guard: __ARM_KERNEL_UPROBES_H

extern "C" {
    pub fn uprobe_decode_ldmstm(
        insn: probes_opcode_t,
        asi: *mut arch_probes_insn,
        d: *const decode_header,
    ) -> probes_insn;

    pub fn decode_ldr(
        insn: probes_opcode_t,
        asi: *mut arch_probes_insn,
        d: *const decode_header,
    ) -> probes_insn;

    pub fn decode_rd12rn16rm0rs8_rwflags(
        insn: probes_opcode_t,
        asi: *mut arch_probes_insn,
        d: *const decode_header,
    ) -> probes_insn;

    pub fn decode_wb_pc(
        insn: probes_opcode_t,
        asi: *mut arch_probes_insn,
        d: *const decode_header,
        alu: bool,
    ) -> probes_insn;

    pub fn decode_pc_ro(
        insn: probes_opcode_t,
        asi: *mut arch_probes_insn,
        d: *const decode_header,
    ) -> probes_insn;

    pub static uprobes_probes_actions: [decode_action; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
