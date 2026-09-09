// SPDX-License-Identifier: GPL-2.0
/*
 * This program is used to generate definitions needed by
 * assembly language modules.
 *
 * We use the technique used in the OSF Mach kernel code:
 * generate asm statements containing #defines,
 * compile this file to assembler, and then extract the
 * #defines from the assembly-language output.
 *
 * On sparc, thread_info data is static and TI_XXX offsets are computed by hand.
 */

// Corresponds to the C preprocessor definition COMPILE_OFFSETS.
// The Linux headers and kbuild offset-generation facilities are external
// dependencies of this translation.

#[cfg(target_pointer_width = "32")]
#[allow(dead_code)]
unsafe fn sparc32_foo() -> i32 {
    DEFINE!(AOFF_thread_fork_kpsr, offset_of!(thread_struct, fork_kpsr));
    0
}

#[cfg(not(target_pointer_width = "32"))]
#[allow(dead_code)]
unsafe fn sparc64_foo() -> i32 {
    #[cfg(feature = "CONFIG_HIBERNATION")]
    {
        BLANK!();
        OFFSET!(SC_REG_FP, saved_context, fp);
        OFFSET!(SC_REG_CWP, saved_context, cwp);
        OFFSET!(SC_REG_WSTATE, saved_context, wstate);

        OFFSET!(SC_REG_TICK, saved_context, tick);
        OFFSET!(SC_REG_PSTATE, saved_context, pstate);

        OFFSET!(SC_REG_G4, saved_context, g4);
        OFFSET!(SC_REG_G5, saved_context, g5);
        OFFSET!(SC_REG_G6, saved_context, g6);
    }
    0
}

#[allow(dead_code)]
unsafe fn foo() -> i32 {
    BLANK!();
    DEFINE!(AOFF_task_thread, offset_of!(task_struct, thread));
    BLANK!();
    DEFINE!(AOFF_mm_context, offset_of!(mm_struct, context));
    BLANK!();
    DEFINE!(VMA_VM_MM, offset_of!(vm_area_struct, vm_mm));

    // DEFINE!(NUM_USER_SEGMENTS, TASK_SIZE >> 28);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
