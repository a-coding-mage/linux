/*
 * arch/xtensa/kernel/asm-offsets.c
 *
 * Generates definitions from c-type structures used by assembly sources.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2005 Tensilica Inc.
 *
 * Chris Zankel <chris@zankel.net>
 */

// The original source defines COMPILE_OFFSETS and includes the architecture
// and kernel declarations supplying the types and constants referenced below.

pub unsafe fn main() -> i32 {
    /* struct pt_regs */
    DEFINE!(PT_PC, offset_of!(pt_regs, pc));
    DEFINE!(PT_PS, offset_of!(pt_regs, ps));
    DEFINE!(PT_DEPC, offset_of!(pt_regs, depc));
    DEFINE!(PT_EXCCAUSE, offset_of!(pt_regs, exccause));
    DEFINE!(PT_EXCVADDR, offset_of!(pt_regs, excvaddr));
    DEFINE!(PT_DEBUGCAUSE, offset_of!(pt_regs, debugcause));
    DEFINE!(PT_WMASK, offset_of!(pt_regs, wmask));
    DEFINE!(PT_LBEG, offset_of!(pt_regs, lbeg));
    DEFINE!(PT_LEND, offset_of!(pt_regs, lend));
    DEFINE!(PT_LCOUNT, offset_of!(pt_regs, lcount));
    DEFINE!(PT_SAR, offset_of!(pt_regs, sar));
    DEFINE!(PT_ICOUNTLEVEL, offset_of!(pt_regs, icountlevel));
    DEFINE!(PT_SYSCALL, offset_of!(pt_regs, syscall));
    DEFINE!(PT_SCOMPARE1, offset_of!(pt_regs, scompare1));
    DEFINE!(PT_THREADPTR, offset_of!(pt_regs, threadptr));
    DEFINE!(PT_AREG, offset_of!(pt_regs, areg[0]));
    DEFINE!(PT_AREG0, offset_of!(pt_regs, areg[0]));
    DEFINE!(PT_AREG1, offset_of!(pt_regs, areg[1]));
    DEFINE!(PT_AREG2, offset_of!(pt_regs, areg[2]));
    DEFINE!(PT_AREG3, offset_of!(pt_regs, areg[3]));
    DEFINE!(PT_AREG4, offset_of!(pt_regs, areg[4]));
    DEFINE!(PT_AREG5, offset_of!(pt_regs, areg[5]));
    DEFINE!(PT_AREG6, offset_of!(pt_regs, areg[6]));
    DEFINE!(PT_AREG7, offset_of!(pt_regs, areg[7]));
    DEFINE!(PT_AREG8, offset_of!(pt_regs, areg[8]));
    DEFINE!(PT_AREG9, offset_of!(pt_regs, areg[9]));
    DEFINE!(PT_AREG10, offset_of!(pt_regs, areg[10]));
    DEFINE!(PT_AREG11, offset_of!(pt_regs, areg[11]));
    DEFINE!(PT_AREG12, offset_of!(pt_regs, areg[12]));
    DEFINE!(PT_AREG13, offset_of!(pt_regs, areg[13]));
    DEFINE!(PT_AREG14, offset_of!(pt_regs, areg[14]));
    DEFINE!(PT_AREG15, offset_of!(pt_regs, areg[15]));
    DEFINE!(PT_WINDOWBASE, offset_of!(pt_regs, windowbase));
    DEFINE!(PT_WINDOWSTART, offset_of!(pt_regs, windowstart));
    DEFINE!(PT_KERNEL_SIZE, offset_of!(pt_regs, areg[16]));
    DEFINE!(PT_AREG_END, offset_of!(pt_regs, areg[XCHAL_NUM_AREGS]));
    DEFINE!(PT_USER_SIZE, offset_of!(pt_regs, areg[XCHAL_NUM_AREGS]));
    DEFINE!(PT_XTREGS_OPT, offset_of!(pt_regs, xtregs_opt));
    DEFINE!(XTREGS_OPT_SIZE, size_of::<xtregs_opt_t>());

    /* struct task_struct */
    DEFINE!(TASK_PTRACE, offset_of!(task_struct, ptrace));
    DEFINE!(TASK_MM, offset_of!(task_struct, mm));
    DEFINE!(TASK_ACTIVE_MM, offset_of!(task_struct, active_mm));
    DEFINE!(TASK_PID, offset_of!(task_struct, pid));
    DEFINE!(TASK_THREAD, offset_of!(task_struct, thread));
    DEFINE!(TASK_THREAD_INFO, offset_of!(task_struct, stack));
    #[cfg(CONFIG_STACKPROTECTOR)]
    DEFINE!(TASK_STACK_CANARY, offset_of!(task_struct, stack_canary));
    DEFINE!(TASK_STRUCT_SIZE, size_of::<task_struct>());

    /* offsets in thread_info struct */
    OFFSET!(TI_TASK, thread_info, task);
    OFFSET!(TI_FLAGS, thread_info, flags);
    OFFSET!(TI_STSTUS, thread_info, status);
    OFFSET!(TI_CPU, thread_info, cpu);
    OFFSET!(TI_PRE_COUNT, thread_info, preempt_count);
    #[cfg(CONFIG_USER_ABI_CALL0_PROBE)]
    OFFSET!(TI_PS_WOE_FIX_ADDR, thread_info, ps_woe_fix_addr);

    /* struct thread_info (offset from start_struct) */
    DEFINE!(THREAD_RA, offset_of!(task_struct, thread.ra));
    DEFINE!(THREAD_SP, offset_of!(task_struct, thread.sp));
    #[cfg(XCHAL_HAVE_EXCLUSIVE)]
    DEFINE!(THREAD_ATOMCTL8, offset_of!(thread_info, atomctl8));
    DEFINE!(THREAD_CPENABLE, offset_of!(thread_info, cpenable));
    DEFINE!(THREAD_CPU, offset_of!(thread_info, cpu));
    DEFINE!(THREAD_CP_OWNER_CPU, offset_of!(thread_info, cp_owner_cpu));
    #[cfg(XTENSA_HAVE_COPROCESSORS)]
    {
        DEFINE!(THREAD_XTREGS_CP0, offset_of!(thread_info, xtregs_cp.cp0));
        DEFINE!(THREAD_XTREGS_CP1, offset_of!(thread_info, xtregs_cp.cp1));
        DEFINE!(THREAD_XTREGS_CP2, offset_of!(thread_info, xtregs_cp.cp2));
        DEFINE!(THREAD_XTREGS_CP3, offset_of!(thread_info, xtregs_cp.cp3));
        DEFINE!(THREAD_XTREGS_CP4, offset_of!(thread_info, xtregs_cp.cp4));
        DEFINE!(THREAD_XTREGS_CP5, offset_of!(thread_info, xtregs_cp.cp5));
        DEFINE!(THREAD_XTREGS_CP6, offset_of!(thread_info, xtregs_cp.cp6));
        DEFINE!(THREAD_XTREGS_CP7, offset_of!(thread_info, xtregs_cp.cp7));
    }
    DEFINE!(THREAD_XTREGS_USER, offset_of!(thread_info, xtregs_user));
    DEFINE!(XTREGS_USER_SIZE, size_of::<xtregs_user_t>());

    /* struct mm_struct */
    DEFINE!(MM_USERS, offset_of!(mm_struct, mm_users));
    DEFINE!(MM_PGD, offset_of!(mm_struct, pgd));
    DEFINE!(MM_CONTEXT, offset_of!(mm_struct, context));

    /* struct page */
    DEFINE!(PAGE_FLAGS, offset_of!(page, flags));

    /* constants */
    DEFINE!(_CLONE_VM, CLONE_VM);
    DEFINE!(_CLONE_UNTRACED, CLONE_UNTRACED);
    DEFINE!(PG_ARCH_1, PG_arch_1);

    /* struct debug_table */
    DEFINE!(DT_DEBUG_EXCEPTION, offset_of!(debug_table, debug_exception));
    DEFINE!(DT_DEBUG_SAVE, offset_of!(debug_table, debug_save));
    #[cfg(CONFIG_HAVE_HW_BREAKPOINT)]
    {
        DEFINE!(DT_DBREAKC_SAVE, offset_of!(debug_table, dbreakc_save));
        DEFINE!(DT_ICOUNT_SAVE, offset_of!(debug_table, icount_save));
        DEFINE!(DT_ICOUNT_LEVEL_SAVE, offset_of!(debug_table, icount_level_save));
    }

    /* struct exc_table */
    DEFINE!(EXC_TABLE_KSTK, offset_of!(exc_table, kstk));
    DEFINE!(EXC_TABLE_DOUBLE_SAVE, offset_of!(exc_table, double_save));
    DEFINE!(EXC_TABLE_FIXUP, offset_of!(exc_table, fixup));
    DEFINE!(EXC_TABLE_PARAM, offset_of!(exc_table, fixup_param));
    #[cfg(XTENSA_HAVE_COPROCESSORS)]
    DEFINE!(EXC_TABLE_COPROCESSOR_OWNER, offset_of!(exc_table, coprocessor_owner));
    DEFINE!(EXC_TABLE_FAST_USER, offset_of!(exc_table, fast_user_handler));
    DEFINE!(EXC_TABLE_FAST_KERNEL, offset_of!(exc_table, fast_kernel_handler));
    DEFINE!(EXC_TABLE_DEFAULT, offset_of!(exc_table, default_handler));

    #[cfg(CONFIG_HIBERNATION)]
    {
        DEFINE!(PBE_ADDRESS, offset_of!(pbe, address));
        DEFINE!(PBE_ORIG_ADDRESS, offset_of!(pbe, orig_address));
        DEFINE!(PBE_NEXT, offset_of!(pbe, next));
        DEFINE!(PBE_SIZE, size_of::<pbe>());
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
