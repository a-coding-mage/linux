/*
 * Copyright (C) 2007-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2007-2009 PetaLogix
 * Copyright (C) 2006 Atmark Techno, Inc.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// COMPILE_OFFSETS
// C dependencies: linux/init.h, linux/stddef.h, linux/sched.h,
// linux/kernel_stat.h, linux/ptrace.h, linux/hardirq.h,
// linux/thread_info.h, linux/kbuild.h, and asm/cpuinfo.h.

pub unsafe fn main(_argc: i32, _argv: *mut *mut u8) -> i32 {
    /* struct pt_regs */
    DEFINE!(PT_SIZE, core::mem::size_of::<pt_regs>());
    DEFINE!(PT_MSR, core::mem::offset_of!(pt_regs, msr));
    DEFINE!(PT_EAR, core::mem::offset_of!(pt_regs, ear));
    DEFINE!(PT_ESR, core::mem::offset_of!(pt_regs, esr));
    DEFINE!(PT_FSR, core::mem::offset_of!(pt_regs, fsr));
    DEFINE!(PT_PC, core::mem::offset_of!(pt_regs, pc));
    DEFINE!(PT_R0, core::mem::offset_of!(pt_regs, r0));
    DEFINE!(PT_R1, core::mem::offset_of!(pt_regs, r1));
    DEFINE!(PT_R2, core::mem::offset_of!(pt_regs, r2));
    DEFINE!(PT_R3, core::mem::offset_of!(pt_regs, r3));
    DEFINE!(PT_R4, core::mem::offset_of!(pt_regs, r4));
    DEFINE!(PT_R5, core::mem::offset_of!(pt_regs, r5));
    DEFINE!(PT_R6, core::mem::offset_of!(pt_regs, r6));
    DEFINE!(PT_R7, core::mem::offset_of!(pt_regs, r7));
    DEFINE!(PT_R8, core::mem::offset_of!(pt_regs, r8));
    DEFINE!(PT_R9, core::mem::offset_of!(pt_regs, r9));
    DEFINE!(PT_R10, core::mem::offset_of!(pt_regs, r10));
    DEFINE!(PT_R11, core::mem::offset_of!(pt_regs, r11));
    DEFINE!(PT_R12, core::mem::offset_of!(pt_regs, r12));
    DEFINE!(PT_R13, core::mem::offset_of!(pt_regs, r13));
    DEFINE!(PT_R14, core::mem::offset_of!(pt_regs, r14));
    DEFINE!(PT_R15, core::mem::offset_of!(pt_regs, r15));
    DEFINE!(PT_R16, core::mem::offset_of!(pt_regs, r16));
    DEFINE!(PT_R17, core::mem::offset_of!(pt_regs, r17));
    DEFINE!(PT_R18, core::mem::offset_of!(pt_regs, r18));
    DEFINE!(PT_R19, core::mem::offset_of!(pt_regs, r19));
    DEFINE!(PT_R20, core::mem::offset_of!(pt_regs, r20));
    DEFINE!(PT_R21, core::mem::offset_of!(pt_regs, r21));
    DEFINE!(PT_R22, core::mem::offset_of!(pt_regs, r22));
    DEFINE!(PT_R23, core::mem::offset_of!(pt_regs, r23));
    DEFINE!(PT_R24, core::mem::offset_of!(pt_regs, r24));
    DEFINE!(PT_R25, core::mem::offset_of!(pt_regs, r25));
    DEFINE!(PT_R26, core::mem::offset_of!(pt_regs, r26));
    DEFINE!(PT_R27, core::mem::offset_of!(pt_regs, r27));
    DEFINE!(PT_R28, core::mem::offset_of!(pt_regs, r28));
    DEFINE!(PT_R29, core::mem::offset_of!(pt_regs, r29));
    DEFINE!(PT_R30, core::mem::offset_of!(pt_regs, r30));
    DEFINE!(PT_R31, core::mem::offset_of!(pt_regs, r31));
    DEFINE!(PT_MODE, core::mem::offset_of!(pt_regs, pt_mode));
    BLANK!();

    /* Magic offsets for PTRACE PEEK/POKE etc */
    DEFINE!(PT_TEXT_ADDR, core::mem::size_of::<pt_regs>() + 1);
    DEFINE!(PT_TEXT_LEN, core::mem::size_of::<pt_regs>() + 2);
    DEFINE!(PT_DATA_ADDR, core::mem::size_of::<pt_regs>() + 3);
    BLANK!();

    /* struct task_struct */
    DEFINE!(TS_THREAD_INFO, core::mem::offset_of!(task_struct, stack));
    DEFINE!(TASK_FLAGS, core::mem::offset_of!(task_struct, flags));
    DEFINE!(TASK_PTRACE, core::mem::offset_of!(task_struct, ptrace));
    DEFINE!(TASK_BLOCKED, core::mem::offset_of!(task_struct, blocked));
    DEFINE!(TASK_MM, core::mem::offset_of!(task_struct, mm));
    DEFINE!(TASK_ACTIVE_MM, core::mem::offset_of!(task_struct, active_mm));
    DEFINE!(TASK_PID, core::mem::offset_of!(task_struct, pid));
    DEFINE!(TASK_THREAD, core::mem::offset_of!(task_struct, thread));
    DEFINE!(THREAD_KSP, core::mem::offset_of!(thread_struct, ksp));
    BLANK!();

    DEFINE!(PGDIR, core::mem::offset_of!(thread_struct, pgdir));
    BLANK!();

    /* struct thread_info */
    DEFINE!(TI_TASK, core::mem::offset_of!(thread_info, task));
    DEFINE!(TI_FLAGS, core::mem::offset_of!(thread_info, flags));
    DEFINE!(TI_CPU_CONTEXT, core::mem::offset_of!(thread_info, cpu_context));
    DEFINE!(TI_PREEMPT_COUNT, core::mem::offset_of!(thread_info, preempt_count));
    BLANK!();

    /* struct cpu_context */
    DEFINE!(CC_R1, core::mem::offset_of!(cpu_context, r1)); /* r1 */
    DEFINE!(CC_R2, core::mem::offset_of!(cpu_context, r2));
    /* dedicated registers */
    DEFINE!(CC_R13, core::mem::offset_of!(cpu_context, r13));
    DEFINE!(CC_R14, core::mem::offset_of!(cpu_context, r14));
    DEFINE!(CC_R15, core::mem::offset_of!(cpu_context, r15));
    DEFINE!(CC_R16, core::mem::offset_of!(cpu_context, r16));
    DEFINE!(CC_R17, core::mem::offset_of!(cpu_context, r17));
    DEFINE!(CC_R18, core::mem::offset_of!(cpu_context, r18));
    /* non-volatile registers */
    DEFINE!(CC_R19, core::mem::offset_of!(cpu_context, r19));
    DEFINE!(CC_R20, core::mem::offset_of!(cpu_context, r20));
    DEFINE!(CC_R21, core::mem::offset_of!(cpu_context, r21));
    DEFINE!(CC_R22, core::mem::offset_of!(cpu_context, r22));
    DEFINE!(CC_R23, core::mem::offset_of!(cpu_context, r23));
    DEFINE!(CC_R24, core::mem::offset_of!(cpu_context, r24));
    DEFINE!(CC_R25, core::mem::offset_of!(cpu_context, r25));
    DEFINE!(CC_R26, core::mem::offset_of!(cpu_context, r26));
    DEFINE!(CC_R27, core::mem::offset_of!(cpu_context, r27));
    DEFINE!(CC_R28, core::mem::offset_of!(cpu_context, r28));
    DEFINE!(CC_R29, core::mem::offset_of!(cpu_context, r29));
    DEFINE!(CC_R30, core::mem::offset_of!(cpu_context, r30));
    /* special purpose registers */
    DEFINE!(CC_MSR, core::mem::offset_of!(cpu_context, msr));
    DEFINE!(CC_EAR, core::mem::offset_of!(cpu_context, ear));
    DEFINE!(CC_ESR, core::mem::offset_of!(cpu_context, esr));
    DEFINE!(CC_FSR, core::mem::offset_of!(cpu_context, fsr));
    BLANK!();

    /* struct cpuinfo */
    DEFINE!(CI_DCS, core::mem::offset_of!(cpuinfo, dcache_size));
    DEFINE!(CI_DCL, core::mem::offset_of!(cpuinfo, dcache_line_length));
    DEFINE!(CI_ICS, core::mem::offset_of!(cpuinfo, icache_size));
    DEFINE!(CI_ICL, core::mem::offset_of!(cpuinfo, icache_line_length));
    BLANK!();

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
