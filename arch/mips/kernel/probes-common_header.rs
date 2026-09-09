/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2016 Imagination Technologies
 * Author: Marcin Nowakowski <marcin.nowakowski@mips.com>
 */

// Dependency supplied by <asm/inst.h> in the original source.

extern "C" {
    pub fn __insn_is_compact_branch(insn: mips_instruction) -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn __insn_has_delay_slot(insn: mips_instruction) -> ::core::ffi::c_int {
    match insn.i_format.opcode {
        // jr and jalr are in r_format format.
        spec_op => match insn.r_format.func {
            jalr_op | jr_op => 1,
            _ => 0,
        },

        // This group contains bltz_op, bgez_op, bltzl_op, bgezl_op,
        // bltzal_op, bgezal_op, bltzall_op, bgezall_op.
        bcond_op => match insn.i_format.rt {
            bltz_op | bltzl_op | bgez_op | bgezl_op |
            bltzal_op | bltzall_op | bgezal_op | bgezall_op |
            bposge32_op => 1,
            _ => 0,
        },

        // These are unconditional and in j_format.
        jal_op | j_op | beq_op | beql_op | bne_op | bnel_op |
        blez_op | blezl_op | bgtz_op | bgtzl_op => 1,

        // And now the FPA/cp1 branch instructions.
        cop1_op => 1,

        // CONFIG_CPU_CAVIUM_OCTEON condition from the original build.
        #[cfg(feature = "CONFIG_CPU_CAVIUM_OCTEON")]
        lwc2_op | ldc2_op | swc2_op | sdc2_op => 1,

        _ => 0,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
