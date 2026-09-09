// SPDX-License-Identifier: GPL-2.0+

// Dependencies supplied by the surrounding kernel translation unit:
// linux/kernel.h, linux/kprobes.h, linux/module.h, linux/kallsyms.h,
// asm/sections.h, decode-insn.h, and simulate-insn.h.

/* Return:
 *   INSN_REJECTED     If instruction is one not allowed to kprobe,
 *   INSN_GOOD_NO_SLOT If instruction is supported but doesn't use its slot.
 */
pub unsafe extern "C" fn csky_probe_decode_insn(
    addr: *mut probe_opcode_t,
    api: *mut arch_probe_insn,
) -> probe_insn {
    let insn: probe_opcode_t = le32_to_cpu(*addr);

    CSKY_INSN_SET_SIMULATE!(br16, insn);
    CSKY_INSN_SET_SIMULATE!(bt16, insn);
    CSKY_INSN_SET_SIMULATE!(bf16, insn);
    CSKY_INSN_SET_SIMULATE!(jmp16, insn);
    CSKY_INSN_SET_SIMULATE!(jsr16, insn);
    CSKY_INSN_SET_SIMULATE!(lrw16, insn);
    CSKY_INSN_SET_SIMULATE!(pop16, insn);

    CSKY_INSN_SET_SIMULATE!(br32, insn);
    CSKY_INSN_SET_SIMULATE!(bt32, insn);
    CSKY_INSN_SET_SIMULATE!(bf32, insn);
    CSKY_INSN_SET_SIMULATE!(jmp32, insn);
    CSKY_INSN_SET_SIMULATE!(jsr32, insn);
    CSKY_INSN_SET_SIMULATE!(lrw32, insn);
    CSKY_INSN_SET_SIMULATE!(pop32, insn);

    CSKY_INSN_SET_SIMULATE!(bez32, insn);
    CSKY_INSN_SET_SIMULATE!(bnez32, insn);
    CSKY_INSN_SET_SIMULATE!(bnezad32, insn);
    CSKY_INSN_SET_SIMULATE!(bhsz32, insn);
    CSKY_INSN_SET_SIMULATE!(bhz32, insn);
    CSKY_INSN_SET_SIMULATE!(blsz32, insn);
    CSKY_INSN_SET_SIMULATE!(blz32, insn);
    CSKY_INSN_SET_SIMULATE!(bsr32, insn);
    CSKY_INSN_SET_SIMULATE!(jmpi32, insn);
    CSKY_INSN_SET_SIMULATE!(jsri32, insn);

    INSN_GOOD
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
