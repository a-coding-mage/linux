// SPDX-License-Identifier: GPL-2.0+

// Dependencies supplied by the surrounding kernel translation unit:
// linux/kernel.h, linux/kprobes.h, linux/module.h, linux/kallsyms.h,
// asm/sections.h, decode-insn.h, and simulate-insn.h.

/* Return:
 *   INSN_REJECTED     If instruction is one not allowed to kprobe,
 *   INSN_GOOD_NO_SLOT If instruction is supported but doesn't use its slot.
 */
// The C __kprobes annotation is retained as a source-level comment.
pub unsafe fn riscv_probe_decode_insn(
    addr: *mut probe_opcode_t,
    api: *mut arch_probe_insn,
) -> probe_insn {
    let insn: probe_opcode_t = *addr;

    /*
     * Reject instructions list:
     */
    RISCV_INSN_REJECTED!(system, insn);
    RISCV_INSN_REJECTED!(fence, insn);

    /*
     * Simulate instructions list:
     * TODO: the REJECTED ones below need to be implemented
     */
    // CONFIG_RISCV_ISA_C conditional compilation from the C source.
    #[cfg(feature = "CONFIG_RISCV_ISA_C")]
    {
        RISCV_INSN_REJECTED!(c_ebreak, insn);

        RISCV_INSN_SET_SIMULATE!(c_j, insn);
        RISCV_INSN_SET_SIMULATE!(c_jr, insn);
        RISCV_INSN_SET_SIMULATE!(c_jalr, insn);
        RISCV_INSN_SET_SIMULATE!(c_jal, insn);
        RISCV_INSN_SET_SIMULATE!(c_beqz, insn);
        RISCV_INSN_SET_SIMULATE!(c_bnez, insn);
    }

    RISCV_INSN_SET_SIMULATE!(jal, insn);
    RISCV_INSN_SET_SIMULATE!(jalr, insn);
    RISCV_INSN_SET_SIMULATE!(auipc, insn);
    RISCV_INSN_SET_SIMULATE!(branch, insn);

    INSN_GOOD
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
