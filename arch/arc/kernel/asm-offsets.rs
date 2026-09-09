// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

// The C source enables COMPILE_OFFSETS and includes the kernel declarations
// that provide the types and offset-generation macros used below.

fn main() -> i32 {
    DEFINE!(TASK_THREAD, offset_of!(task_struct, thread));
    DEFINE!(TASK_THREAD_INFO, offset_of!(task_struct, stack));

    BLANK!();

    DEFINE!(THREAD_CALLEE_REG, offset_of!(thread_struct, callee_reg));
    DEFINE!(THREAD_FAULT_ADDR, offset_of!(thread_struct, fault_address));

    BLANK!();

    DEFINE!(THREAD_INFO_KSP, offset_of!(thread_info, ksp));
    DEFINE!(THREAD_INFO_FLAGS, offset_of!(thread_info, flags));
    DEFINE!(
        THREAD_INFO_PREEMPT_COUNT,
        offset_of!(thread_info, preempt_count)
    );

    BLANK!();

    DEFINE!(TASK_ACT_MM, offset_of!(task_struct, active_mm));
    DEFINE!(TASK_TGID, offset_of!(task_struct, tgid));
    DEFINE!(TASK_PID, offset_of!(task_struct, pid));
    DEFINE!(TASK_COMM, offset_of!(task_struct, comm));

    DEFINE!(MM_CTXT, offset_of!(mm_struct, context));
    DEFINE!(MM_PGD, offset_of!(mm_struct, pgd));

    DEFINE!(MM_CTXT_ASID, offset_of!(mm_context_t, asid));

    BLANK!();

    DEFINE!(PT_status32, offset_of!(pt_regs, status32));
    DEFINE!(PT_event, offset_of!(pt_regs, ecr));
    DEFINE!(PT_bta, offset_of!(pt_regs, bta));
    DEFINE!(PT_sp, offset_of!(pt_regs, sp));
    DEFINE!(PT_r0, offset_of!(pt_regs, r0));
    DEFINE!(PT_r1, offset_of!(pt_regs, r1));
    DEFINE!(PT_r2, offset_of!(pt_regs, r2));
    DEFINE!(PT_r3, offset_of!(pt_regs, r3));
    DEFINE!(PT_r4, offset_of!(pt_regs, r4));
    DEFINE!(PT_r5, offset_of!(pt_regs, r5));
    DEFINE!(PT_r6, offset_of!(pt_regs, r6));
    DEFINE!(PT_r7, offset_of!(pt_regs, r7));
    DEFINE!(PT_r8, offset_of!(pt_regs, r8));
    DEFINE!(PT_r10, offset_of!(pt_regs, r10));
    DEFINE!(PT_r26, offset_of!(pt_regs, r26));
    DEFINE!(PT_ret, offset_of!(pt_regs, ret));
    DEFINE!(PT_blink, offset_of!(pt_regs, blink));
    OFFSET!(PT_fp, pt_regs, fp);
    DEFINE!(PT_lpe, offset_of!(pt_regs, lp_end));
    DEFINE!(PT_lpc, offset_of!(pt_regs, lp_count));

    // Preserved build-time condition: CONFIG_ISA_ARCV2.
    #[cfg(CONFIG_ISA_ARCV2)]
    {
        OFFSET!(PT_r12, pt_regs, r12);
        OFFSET!(PT_r30, pt_regs, r30);
    }

    // Preserved build-time condition: CONFIG_ARC_HAS_ACCL_REGS.
    #[cfg(CONFIG_ARC_HAS_ACCL_REGS)]
    {
        OFFSET!(PT_r58, pt_regs, r58);
        OFFSET!(PT_r59, pt_regs, r59);
    }

    // Preserved build-time condition: CONFIG_ARC_DSP_SAVE_RESTORE_REGS.
    #[cfg(CONFIG_ARC_DSP_SAVE_RESTORE_REGS)]
    {
        OFFSET!(PT_DSP_CTRL, pt_regs, DSP_CTRL);
    }

    DEFINE!(SZ_CALLEE_REGS, size_of!(callee_regs));
    DEFINE!(SZ_PT_REGS, size_of!(pt_regs));

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
