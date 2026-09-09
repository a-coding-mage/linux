/* SPDX-License-Identifier: GPL-2.0+ */

/* Dependencies supplied by the surrounding kernel translation unit. */

#[repr(C)]
pub enum probe_insn {
    INSN_REJECTED,
    INSN_GOOD_NO_SLOT,
    INSN_GOOD,
}

/* __kprobes */
extern "C" {
    pub fn riscv_probe_decode_insn(
        addr: *mut probe_opcode_t,
        asi: *mut arch_probe_insn,
    ) -> probe_insn;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
