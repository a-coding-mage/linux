// SPDX-License-Identifier: GPL-2.0
/*
 * This program is used to generate definitions needed by
 * assembly language modules.
 *
 * We use the technique used in the OSF Mach kernel code:
 * generate asm statements containing #defines,
 * compile this file to assembler, and then extract the
 * #defines from the assembly-language output.
 */

// C build-time definitions: COMPILE_OFFSETS and ASM_OFFSETS_C.
// C header dependencies are supplied by the surrounding build.

pub unsafe fn main() -> i32 {
    /* offsets into the task struct */
    DEFINE!(TASK_THREAD, offsetof!(task_struct, thread));
    DEFINE!(TASK_MM, offsetof!(task_struct, mm));
    DEFINE!(TASK_STACK, offsetof!(task_struct, stack));

    /* offsets into the thread struct */
    DEFINE!(THREAD_KSP, offsetof!(thread_struct, ksp));
    DEFINE!(THREAD_USP, offsetof!(thread_struct, usp));
    DEFINE!(THREAD_SR, offsetof!(thread_struct, sr));
    DEFINE!(THREAD_FC, offsetof!(thread_struct, fc));
    DEFINE!(THREAD_CRP, offsetof!(thread_struct, crp));
    DEFINE!(THREAD_ESP0, offsetof!(thread_struct, esp0));
    DEFINE!(THREAD_FPREG, offsetof!(thread_struct, fp));
    DEFINE!(THREAD_FPCNTL, offsetof!(thread_struct, fpcntl));
    DEFINE!(THREAD_FPSTATE, offsetof!(thread_struct, fpstate));

    /* offsets into the thread_info struct */
    DEFINE!(TINFO_PREEMPT, offsetof!(thread_info, preempt_count));
    DEFINE!(TINFO_FLAGS, offsetof!(thread_info, flags));

    /* offsets into the pt_regs */
    DEFINE!(PT_OFF_D0, offsetof!(pt_regs, d0));
    DEFINE!(PT_OFF_ORIG_D0, offsetof!(pt_regs, orig_d0));
    DEFINE!(PT_OFF_D1, offsetof!(pt_regs, d1));
    DEFINE!(PT_OFF_D2, offsetof!(pt_regs, d2));
    DEFINE!(PT_OFF_D3, offsetof!(pt_regs, d3));
    DEFINE!(PT_OFF_D4, offsetof!(pt_regs, d4));
    DEFINE!(PT_OFF_D5, offsetof!(pt_regs, d5));
    DEFINE!(PT_OFF_A0, offsetof!(pt_regs, a0));
    DEFINE!(PT_OFF_A1, offsetof!(pt_regs, a1));
    DEFINE!(PT_OFF_A2, offsetof!(pt_regs, a2));
    DEFINE!(PT_OFF_PC, offsetof!(pt_regs, pc));
    DEFINE!(PT_OFF_SR, offsetof!(pt_regs, sr));

    /* bitfields are a bit difficult */
    #[cfg(CONFIG_COLDFIRE)]
    DEFINE!(PT_OFF_FORMATVEC, offsetof!(pt_regs, sr) - 2);
    #[cfg(not(CONFIG_COLDFIRE))]
    DEFINE!(PT_OFF_FORMATVEC, offsetof!(pt_regs, pc) + 4);

    /* offsets into the irq_cpustat_t struct */
    DEFINE!(CPUSTAT_SOFTIRQ_PENDING, offsetof!(irq_cpustat_t, __softirq_pending));

    /* signal defines */
    DEFINE!(LSIGSEGV, SIGSEGV);
    DEFINE!(LSEGV_MAPERR, SEGV_MAPERR);
    DEFINE!(LSIGTRAP, SIGTRAP);
    DEFINE!(LTRAP_TRACE, TRAP_TRACE);

    #[cfg(CONFIG_MMU)]
    {
        /* offsets into the bi_record struct */
        DEFINE!(BIR_TAG, offsetof!(bi_record, tag));
        DEFINE!(BIR_SIZE, offsetof!(bi_record, size));
        DEFINE!(BIR_DATA, offsetof!(bi_record, data));

        /* offsets into the font_desc struct */
        DEFINE!(FONT_DESC_IDX, offsetof!(font_desc, idx));
        DEFINE!(FONT_DESC_NAME, offsetof!(font_desc, name));
        DEFINE!(FONT_DESC_WIDTH, offsetof!(font_desc, width));
        DEFINE!(FONT_DESC_HEIGHT, offsetof!(font_desc, height));
        DEFINE!(FONT_DESC_DATA, offsetof!(font_desc, data));
        DEFINE!(FONT_DESC_PREF, offsetof!(font_desc, pref));

        /* offsets into the custom struct */
        DEFINE!(CUSTOMBASE, &amiga_custom);
        DEFINE!(C_INTENAR, offsetof!(CUSTOM, intenar));
        DEFINE!(C_INTREQR, offsetof!(CUSTOM, intreqr));
        DEFINE!(C_INTENA, offsetof!(CUSTOM, intena));
        DEFINE!(C_INTREQ, offsetof!(CUSTOM, intreq));
        DEFINE!(C_SERDATR, offsetof!(CUSTOM, serdatr));
        DEFINE!(C_SERDAT, offsetof!(CUSTOM, serdat));
        DEFINE!(C_SERPER, offsetof!(CUSTOM, serper));
        DEFINE!(CIAABASE, &ciaa);
        DEFINE!(CIABBASE, &ciab);
        DEFINE!(C_PRA, offsetof!(CIA, pra));
        DEFINE!(ZTWOBASE, zTwoBase);

        /* enum m68k_fixup_type */
        DEFINE!(M68K_FIXUP_MEMOFFSET, m68k_fixup_memoffset);
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
