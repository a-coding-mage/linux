// SPDX-License-Identifier: GPL-2.0
/*
 * Generate definitions needed by assembly language modules.
 * This code generates raw asm output which is post-processed to extract
 * and format the required data.
 */

// COMPILE_OFFSETS
// Dependencies supplied by the corresponding Linux and Alpha headers.

#[allow(dead_code)]
fn foo() {
    DEFINE!(TI_FLAGS, core::mem::offset_of!(thread_info, flags));
    DEFINE!(TI_FP, core::mem::offset_of!(thread_info, fp));
    DEFINE!(TI_STATUS, core::mem::offset_of!(thread_info, status));
    BLANK!();

    DEFINE!(SP_OFF, core::mem::offset_of!(pt_regs, ps));
    DEFINE!(SIZEOF_PT_REGS, core::mem::size_of::<pt_regs>());
    BLANK!();

    DEFINE!(SWITCH_STACK_SIZE, core::mem::size_of::<switch_stack>());
    BLANK!();

    DEFINE!(HAE_CACHE, core::mem::offset_of!(alpha_machine_vector, hae_cache));
    DEFINE!(HAE_REG, core::mem::offset_of!(alpha_machine_vector, hae_register));
    DEFINE!(PT_REGS_USP, core::mem::offset_of!(pt_regs, usp));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
