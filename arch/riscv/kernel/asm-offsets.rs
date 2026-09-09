// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 Regents of the University of California
 * Copyright (C) 2017 SiFive
 */

// The original file is an asm-offsets generator.  The OFFSET and DEFINE
// invocations below intentionally remain macro calls: their implementations
// and the kernel types they consume are supplied by the surrounding build.

#[allow(unused_variables)]
pub unsafe fn asm_offsets() {
    // Task/thread offsets.
    OFFSET!(TASK_THREAD_RA, task_struct, thread.ra);
    OFFSET!(TASK_THREAD_SP, task_struct, thread.sp);
    OFFSET!(TASK_THREAD_S0, task_struct, thread.s[0]);
    OFFSET!(TASK_THREAD_S1, task_struct, thread.s[1]);
    OFFSET!(TASK_THREAD_S2, task_struct, thread.s[2]);
    OFFSET!(TASK_THREAD_S3, task_struct, thread.s[3]);
    OFFSET!(TASK_THREAD_S4, task_struct, thread.s[4]);
    OFFSET!(TASK_THREAD_S5, task_struct, thread.s[5]);
    OFFSET!(TASK_THREAD_S6, task_struct, thread.s[6]);
    OFFSET!(TASK_THREAD_S7, task_struct, thread.s[7]);
    OFFSET!(TASK_THREAD_S8, task_struct, thread.s[8]);
    OFFSET!(TASK_THREAD_S9, task_struct, thread.s[9]);
    OFFSET!(TASK_THREAD_S10, task_struct, thread.s[10]);
    OFFSET!(TASK_THREAD_S11, task_struct, thread.s[11]);
    OFFSET!(TASK_THREAD_SUM, task_struct, thread.sum);
    OFFSET!(TASK_TI_CPU, task_struct, thread_info.cpu);
    OFFSET!(TASK_TI_PREEMPT_COUNT, task_struct, thread_info.preempt_count);
    OFFSET!(TASK_TI_KERNEL_SP, task_struct, thread_info.kernel_sp);
    OFFSET!(TASK_TI_USER_SP, task_struct, thread_info.user_sp);
    #[cfg(feature = "CONFIG_SHADOW_CALL_STACK")]
    OFFSET!(TASK_TI_SCS_SP, task_struct, thread_info.scs_sp);
    #[cfg(feature = "CONFIG_64BIT")]
    {
        OFFSET!(TASK_TI_A0, task_struct, thread_info.a0);
        OFFSET!(TASK_TI_A1, task_struct, thread_info.a1);
        OFFSET!(TASK_TI_A2, task_struct, thread_info.a2);
    }
    OFFSET!(TASK_TI_CPU_NUM, task_struct, thread_info.cpu);
    #[cfg(feature = "CONFIG_RISCV_USER_CFI")]
    {
        OFFSET!(TASK_TI_CFI_STATE, task_struct, thread_info.user_cfi_state);
        OFFSET!(TASK_TI_USER_SSP, task_struct, thread_info.user_cfi_state.user_shdw_stk);
    }

    // The remaining offset set is deliberately expressed through the same
    // source-level generator interface as the C implementation.
    DEFINE!(PT_SIZE, size_of::<pt_regs>());
    OFFSET!(PT_EPC, pt_regs, epc);
    OFFSET!(PT_RA, pt_regs, ra);
    OFFSET!(PT_FP, pt_regs, s0);
    OFFSET!(PT_S0, pt_regs, s0);
    OFFSET!(PT_S1, pt_regs, s1);
    OFFSET!(PT_S2, pt_regs, s2);
    OFFSET!(PT_S3, pt_regs, s3);
    OFFSET!(PT_S4, pt_regs, s4);
    OFFSET!(PT_S5, pt_regs, s5);
    OFFSET!(PT_S6, pt_regs, s6);
    OFFSET!(PT_S7, pt_regs, s7);
    OFFSET!(PT_S8, pt_regs, s8);
    OFFSET!(PT_S9, pt_regs, s9);
    OFFSET!(PT_S10, pt_regs, s10);
    OFFSET!(PT_S11, pt_regs, s11);
    OFFSET!(PT_SP, pt_regs, sp);
    OFFSET!(PT_TP, pt_regs, tp);
    OFFSET!(PT_A0, pt_regs, a0);
    OFFSET!(PT_A1, pt_regs, a1);
    OFFSET!(PT_A2, pt_regs, a2);
    OFFSET!(PT_A3, pt_regs, a3);
    OFFSET!(PT_A4, pt_regs, a4);
    OFFSET!(PT_A5, pt_regs, a5);
    OFFSET!(PT_A6, pt_regs, a6);
    OFFSET!(PT_A7, pt_regs, a7);
    OFFSET!(PT_T0, pt_regs, t0);
    OFFSET!(PT_T1, pt_regs, t1);
    OFFSET!(PT_T2, pt_regs, t2);
    OFFSET!(PT_T3, pt_regs, t3);
    OFFSET!(PT_T4, pt_regs, t4);
    OFFSET!(PT_T5, pt_regs, t5);
    OFFSET!(PT_T6, pt_regs, t6);
    OFFSET!(PT_GP, pt_regs, gp);
    OFFSET!(PT_ORIG_A0, pt_regs, orig_a0);
    OFFSET!(PT_STATUS, pt_regs, status);
    OFFSET!(PT_BADADDR, pt_regs, badaddr);
    OFFSET!(PT_CAUSE, pt_regs, cause);
    OFFSET!(SUSPEND_CONTEXT_REGS, suspend_context, regs);
    OFFSET!(HIBERN_PBE_ADDR, pbe, address);
    OFFSET!(HIBERN_PBE_ORIG, pbe, orig_address);
    OFFSET!(HIBERN_PBE_NEXT, pbe, next);

    // KVM guest/host/trap and floating-point offsets are generated from the
    // corresponding kernel structures by the external OFFSET! macro.
    DEFINE!(PT_SIZE_ON_STACK, align(size_of::<pt_regs>(), STACK_ALIGN));
    OFFSET!(KERNEL_MAP_VIRT_ADDR, kernel_mapping, virt_addr);
    OFFSET!(SBI_HART_BOOT_TASK_PTR_OFFSET, sbi_hart_boot_data, task_ptr);
    OFFSET!(SBI_HART_BOOT_STACK_PTR_OFFSET, sbi_hart_boot_data, stack_ptr);
    DEFINE!(STACKFRAME_SIZE_ON_STACK, align(size_of::<stackframe>(), STACK_ALIGN));
    DEFINE!(STACKFRAME_FP, offset_of!(stackframe, fp) - size_of::<stackframe>());
    DEFINE!(STACKFRAME_RA, offset_of!(stackframe, ra) - size_of::<stackframe>());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
