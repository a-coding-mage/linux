// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

// COMPILE_OFFSETS
// External kernel dependencies supplied by the surrounding build:
// linux/sched.h, linux/kernel_stat.h, linux/kbuild.h, abi/regdef.h

pub fn main() -> i32 {
    /* offsets into the task struct */
    DEFINE!(TASK_THREAD_INFO, offset_of!(task_struct, stack));
    DEFINE!(TASK_FLAGS, offset_of!(task_struct, flags));
    DEFINE!(TASK_PTRACE, offset_of!(task_struct, ptrace));
    DEFINE!(TASK_THREAD, offset_of!(task_struct, thread));
    DEFINE!(TASK_MM, offset_of!(task_struct, mm));
    DEFINE!(TASK_ACTIVE_MM, offset_of!(task_struct, active_mm));

    /* offsets into the thread struct */
    DEFINE!(THREAD_KSP, offset_of!(thread_struct, sp));
    DEFINE!(THREAD_FESR, offset_of!(thread_struct, user_fp.fesr));
    DEFINE!(THREAD_FCR, offset_of!(thread_struct, user_fp.fcr));
    DEFINE!(THREAD_FPREG, offset_of!(thread_struct, user_fp.vr));

    /* offsets into the thread_info struct */
    DEFINE!(TINFO_FLAGS, offset_of!(thread_info, flags));
    DEFINE!(TINFO_PREEMPT, offset_of!(thread_info, preempt_count));
    DEFINE!(TINFO_TP_VALUE, offset_of!(thread_info, tp_value));
    DEFINE!(TINFO_TASK, offset_of!(thread_info, task));

    /* offsets into the pt_regs */
    DEFINE!(PT_PC, offset_of!(pt_regs, pc));
    DEFINE!(PT_ORIG_AO, offset_of!(pt_regs, orig_a0));
    DEFINE!(PT_SR, offset_of!(pt_regs, sr));

    DEFINE!(PT_A0, offset_of!(pt_regs, a0));
    DEFINE!(PT_A1, offset_of!(pt_regs, a1));
    DEFINE!(PT_A2, offset_of!(pt_regs, a2));
    DEFINE!(PT_A3, offset_of!(pt_regs, a3));
    DEFINE!(PT_REGS0, offset_of!(pt_regs, regs[0]));
    DEFINE!(PT_REGS1, offset_of!(pt_regs, regs[1]));
    DEFINE!(PT_REGS2, offset_of!(pt_regs, regs[2]));
    DEFINE!(PT_REGS3, offset_of!(pt_regs, regs[3]));
    DEFINE!(PT_REGS4, offset_of!(pt_regs, regs[4]));
    DEFINE!(PT_REGS5, offset_of!(pt_regs, regs[5]));
    DEFINE!(PT_REGS6, offset_of!(pt_regs, regs[6]));
    DEFINE!(PT_REGS7, offset_of!(pt_regs, regs[7]));
    DEFINE!(PT_REGS8, offset_of!(pt_regs, regs[8]));
    DEFINE!(PT_REGS9, offset_of!(pt_regs, regs[9]));
    DEFINE!(PT_R15, offset_of!(pt_regs, lr));
    // C conditional: #if defined(__CSKYABIV2__)
    DEFINE!(PT_R16, offset_of!(pt_regs, exregs[0]));
    DEFINE!(PT_R17, offset_of!(pt_regs, exregs[1]));
    DEFINE!(PT_R18, offset_of!(pt_regs, exregs[2]));
    DEFINE!(PT_R19, offset_of!(pt_regs, exregs[3]));
    DEFINE!(PT_R20, offset_of!(pt_regs, exregs[4]));
    DEFINE!(PT_R21, offset_of!(pt_regs, exregs[5]));
    DEFINE!(PT_R22, offset_of!(pt_regs, exregs[6]));
    DEFINE!(PT_R23, offset_of!(pt_regs, exregs[7]));
    DEFINE!(PT_R24, offset_of!(pt_regs, exregs[8]));
    DEFINE!(PT_R25, offset_of!(pt_regs, exregs[9]));
    DEFINE!(PT_R26, offset_of!(pt_regs, exregs[10]));
    DEFINE!(PT_R27, offset_of!(pt_regs, exregs[11]));
    DEFINE!(PT_R28, offset_of!(pt_regs, exregs[12]));
    DEFINE!(PT_R29, offset_of!(pt_regs, exregs[13]));
    DEFINE!(PT_R30, offset_of!(pt_regs, exregs[14]));
    DEFINE!(PT_R31, offset_of!(pt_regs, exregs[15]));
    DEFINE!(PT_RHI, offset_of!(pt_regs, rhi));
    DEFINE!(PT_RLO, offset_of!(pt_regs, rlo));
    // End of C conditional: #endif
    DEFINE!(PT_USP, offset_of!(pt_regs, usp));
    DEFINE!(PT_FRAME_SIZE, size_of!(pt_regs));

    /* offsets into the irq_cpustat_t struct */
    DEFINE!(CPUSTAT_SOFTIRQ_PENDING, offset_of!(irq_cpustat_t, __softirq_pending));

    /* signal defines */
    DEFINE!(SIGSEGV, SIGSEGV);
    DEFINE!(SIGTRAP, SIGTRAP);

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
