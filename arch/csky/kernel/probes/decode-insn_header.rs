/* SPDX-License-Identifier: GPL-2.0+ */

/*
 * Dependencies supplied by the corresponding architecture and kprobes
 * headers are intentionally left external to this translation.
 */

#[repr(C)]
pub enum probe_insn {
    INSN_REJECTED,
    INSN_GOOD_NO_SLOT,
    INSN_GOOD,
}

macro_rules! is_insn32 {
    ($insn:expr) => {
        (($insn & 0xc000) == 0xc000)
    };
}

pub unsafe extern "C" {
    pub fn csky_probe_decode_insn(
        addr: *mut probe_opcode_t,
        asi: *mut arch_probe_insn,
    ) -> probe_insn;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
