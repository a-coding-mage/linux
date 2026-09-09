// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Generate definitions needed by assembly language modules.
 * This code generates raw asm output which is post-processed to extract
 * and format the required data.
 *
 * Copyright (C) 2000-2003, the original C authors.
 */

// C dependencies: linux/types.h, linux/sched.h, linux/thread_info.h,
// linux/ptrace.h, linux/hardirq.h, linux/kbuild.h, linux/pgtable.h,
// asm/assembly.h, asm/ptrace.h, asm/processor.h, asm/pdc.h,
// uapi/asm/sigcontext.h, asm/ucontext.h, asm/rt_sigframe.h,
// linux/uaccess.h, and signal32.h.

/* Add FRAME_SIZE to x and align it to y. */
macro_rules! align_frame {
    ($x:expr, $y:expr) => { (($x) + FRAME_SIZE + ($y) - 1) - ((($x) + ($y) - 1) % ($y)) };
}

pub fn main() {
    DEFINE!(TASK_TI_FLAGS, offset_of!(task_struct, thread_info.flags));
    #[cfg(CONFIG_SMP)]
    DEFINE!(TASK_TI_CPU, offset_of!(task_struct, thread_info.cpu));
    DEFINE!(TASK_STACK, offset_of!(task_struct, stack));
    DEFINE!(TASK_PAGEFAULT_DISABLED, offset_of!(task_struct, pagefault_disabled));
    BLANK!();
    DEFINE!(TASK_REGS, offset_of!(task_struct, thread.regs));

    $(DEFINE!(TASK_PT_GR$N, offset_of!(task_struct, thread.regs.gr[$N]));)*
    $(DEFINE!(TASK_PT_FR$N, offset_of!(task_struct, thread.regs.fr[$N]));)*
    $(DEFINE!(TASK_PT_SR$N, offset_of!(task_struct, thread.regs.sr[$N]));)*
    DEFINE!(TASK_PT_PSW, offset_of!(task_struct, thread.regs.gr[0]));
    DEFINE!(TASK_PT_IASQ0, offset_of!(task_struct, thread.regs.iasq[0]));
    DEFINE!(TASK_PT_IASQ1, offset_of!(task_struct, thread.regs.iasq[1]));
    DEFINE!(TASK_PT_IAOQ0, offset_of!(task_struct, thread.regs.iaoq[0]));
    DEFINE!(TASK_PT_IAOQ1, offset_of!(task_struct, thread.regs.iaoq[1]));
    DEFINE!(TASK_PT_CR27, offset_of!(task_struct, thread.regs.cr27));
    DEFINE!(TASK_PT_ORIG_R28, offset_of!(task_struct, thread.regs.orig_r28));
    DEFINE!(TASK_PT_KSP, offset_of!(task_struct, thread.regs.ksp));
    DEFINE!(TASK_PT_KPC, offset_of!(task_struct, thread.regs.kpc));
    DEFINE!(TASK_PT_SAR, offset_of!(task_struct, thread.regs.sar));
    DEFINE!(TASK_PT_IIR, offset_of!(task_struct, thread.regs.iir));
    DEFINE!(TASK_PT_ISR, offset_of!(task_struct, thread.regs.isr));
    DEFINE!(TASK_PT_IOR, offset_of!(task_struct, thread.regs.ior));
    BLANK!();

    // The following register offsets correspond exactly to struct pt_regs.
    DEFINE!(PT_PSW, offset_of!(pt_regs, gr[0]));
    $(DEFINE!(PT_GR$N, offset_of!(pt_regs, gr[$N]));)*
    $(DEFINE!(PT_FR$N, offset_of!(pt_regs, fr[$N]));)*
    $(DEFINE!(PT_SR$N, offset_of!(pt_regs, sr[$N]));)*
    DEFINE!(PT_IASQ0, offset_of!(pt_regs, iasq[0]));
    DEFINE!(PT_IASQ1, offset_of!(pt_regs, iasq[1]));
    DEFINE!(PT_IAOQ0, offset_of!(pt_regs, iaoq[0]));
    DEFINE!(PT_IAOQ1, offset_of!(pt_regs, iaoq[1]));
    DEFINE!(PT_CR27, offset_of!(pt_regs, cr27));
    DEFINE!(PT_ORIG_R28, offset_of!(pt_regs, orig_r28));
    DEFINE!(PT_KSP, offset_of!(pt_regs, ksp));
    DEFINE!(PT_KPC, offset_of!(pt_regs, kpc));
    DEFINE!(PT_SAR, offset_of!(pt_regs, sar));
    DEFINE!(PT_IIR, offset_of!(pt_regs, iir));
    DEFINE!(PT_ISR, offset_of!(pt_regs, isr));
    DEFINE!(PT_IOR, offset_of!(pt_regs, ior));
    /* PT_SZ_ALGN includes space for a stack frame. */
    DEFINE!(PT_SZ_ALGN, align_frame!(size_of!(pt_regs), FRAME_ALIGN));
    BLANK!();
    DEFINE!(TI_FLAGS, offset_of!(thread_info, flags));
    DEFINE!(TI_PRE_COUNT, offset_of!(task_struct, thread_info.preempt_count));
    BLANK!();
    DEFINE!(ASM_SIGFRAME_SIZE, PARISC_RT_SIGFRAME_SIZE);
    DEFINE!(SIGFRAME_CONTEXT_REGS, offset_of!(rt_sigframe, uc.uc_mcontext) - PARISC_RT_SIGFRAME_SIZE);
    #[cfg(CONFIG_64BIT)]
    {
        DEFINE!(ASM_SIGFRAME_SIZE32, PARISC_RT_SIGFRAME_SIZE32);
        DEFINE!(SIGFRAME_CONTEXT_REGS32, offset_of!(compat_rt_sigframe, uc.uc_mcontext) - PARISC_RT_SIGFRAME_SIZE32);
    }
    #[cfg(not(CONFIG_64BIT))]
    {
        DEFINE!(ASM_SIGFRAME_SIZE32, PARISC_RT_SIGFRAME_SIZE);
        DEFINE!(SIGFRAME_CONTEXT_REGS32, offset_of!(rt_sigframe, uc.uc_mcontext) - PARISC_RT_SIGFRAME_SIZE);
    }
    BLANK!();
    DEFINE!(ICACHE_BASE, offset_of!(pdc_cache_info, ic_base));
    DEFINE!(ICACHE_STRIDE, offset_of!(pdc_cache_info, ic_stride));
    DEFINE!(ICACHE_COUNT, offset_of!(pdc_cache_info, ic_count));
    DEFINE!(ICACHE_LOOP, offset_of!(pdc_cache_info, ic_loop));
    DEFINE!(DCACHE_BASE, offset_of!(pdc_cache_info, dc_base));
    DEFINE!(DCACHE_STRIDE, offset_of!(pdc_cache_info, dc_stride));
    DEFINE!(DCACHE_COUNT, offset_of!(pdc_cache_info, dc_count));
    DEFINE!(DCACHE_LOOP, offset_of!(pdc_cache_info, dc_loop));
    DEFINE!(ITLB_SID_BASE, offset_of!(pdc_cache_info, it_sp_base));
    DEFINE!(ITLB_SID_STRIDE, offset_of!(pdc_cache_info, it_sp_stride));
    DEFINE!(ITLB_SID_COUNT, offset_of!(pdc_cache_info, it_sp_count));
    DEFINE!(ITLB_OFF_BASE, offset_of!(pdc_cache_info, it_off_base));
    DEFINE!(ITLB_OFF_STRIDE, offset_of!(pdc_cache_info, it_off_stride));
    DEFINE!(ITLB_OFF_COUNT, offset_of!(pdc_cache_info, it_off_count));
    DEFINE!(ITLB_LOOP, offset_of!(pdc_cache_info, it_loop));
    DEFINE!(DTLB_SID_BASE, offset_of!(pdc_cache_info, dt_sp_base));
    DEFINE!(DTLB_SID_STRIDE, offset_of!(pdc_cache_info, dt_sp_stride));
    DEFINE!(DTLB_SID_COUNT, offset_of!(pdc_cache_info, dt_sp_count));
    DEFINE!(DTLB_OFF_BASE, offset_of!(pdc_cache_info, dt_off_base));
    DEFINE!(DTLB_OFF_STRIDE, offset_of!(pdc_cache_info, dt_off_stride));
    DEFINE!(DTLB_OFF_COUNT, offset_of!(pdc_cache_info, dt_off_count));
    DEFINE!(DTLB_LOOP, offset_of!(pdc_cache_info, dt_loop));
    BLANK!();
    DEFINE!(TIF_BLOCKSTEP_PA_BIT, 31 - TIF_BLOCKSTEP);
    DEFINE!(TIF_SINGLESTEP_PA_BIT, 31 - TIF_SINGLESTEP);
    DEFINE!(TIF_32BIT_PA_BIT, 31 - TIF_32BIT);
    BLANK!();
    DEFINE!(ASM_PMD_SHIFT, PMD_SHIFT);
    DEFINE!(ASM_PGDIR_SHIFT, PGDIR_SHIFT);
    DEFINE!(ASM_BITS_PER_PGD, BITS_PER_PGD);
    DEFINE!(ASM_BITS_PER_PMD, BITS_PER_PMD);
    DEFINE!(ASM_BITS_PER_PTE, BITS_PER_PTE);
    DEFINE!(ASM_PMD_ENTRY, ((PAGE_OFFSET & PMD_MASK) >> PMD_SHIFT));
    DEFINE!(ASM_PGD_ENTRY, PAGE_OFFSET >> PGDIR_SHIFT);
    DEFINE!(ASM_PGD_ENTRY_SIZE, PGD_ENTRY_SIZE);
    DEFINE!(ASM_PMD_ENTRY_SIZE, PMD_ENTRY_SIZE);
    DEFINE!(ASM_PTE_ENTRY_SIZE, PTE_ENTRY_SIZE);
    DEFINE!(ASM_PFN_PTE_SHIFT, PFN_PTE_SHIFT);
    DEFINE!(ASM_PT_INITIAL, PT_INITIAL);
    BLANK!();
    /* HUGEPAGE_SIZE is used by vmlinux.lds.S for physical huge-page alignment. */
    #[cfg(CONFIG_HUGETLB_PAGE)]
    DEFINE!(HUGEPAGE_SIZE, 1UL << REAL_HPAGE_SHIFT);
    #[cfg(all(not(CONFIG_HUGETLB_PAGE), not(CONFIG_64BIT)))]
    DEFINE!(HUGEPAGE_SIZE, 4 * 1024 * 1024);
    #[cfg(all(not(CONFIG_HUGETLB_PAGE), CONFIG_64BIT))]
    DEFINE!(HUGEPAGE_SIZE, PAGE_SIZE);
    BLANK!();
    DEFINE!(ASM_PDC_RESULT_SIZE, NUM_PDC_RESULT * size_of!(unsigned_long));
    BLANK!();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
