// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Common implementation of switch_mm_irqs_off
 *
 *  Copyright IBM Corp. 2017
 */

// Dependencies are supplied by the surrounding kernel translation.

#[cfg(CONFIG_PPC32)]
#[inline]
unsafe fn switch_mm_pgdir(tsk: *mut task_struct, mm: *mut mm_struct) {
    /* 32-bit keeps track of the current PGDIR in the thread struct */
    (*tsk).thread.pgdir = (*mm).pgd;
    #[cfg(CONFIG_PPC_BOOK3S_32)]
    {
        (*tsk).thread.sr0 = (*mm).context.sr0;
    }
    #[cfg(all(CONFIG_BOOKE, CONFIG_PPC_KUAP))]
    {
        (*tsk).thread.pid = (*mm).context.id;
    }
}

#[cfg(all(not(CONFIG_PPC32), CONFIG_PPC_BOOK3E_64))]
#[inline]
unsafe fn switch_mm_pgdir(tsk: *mut task_struct, mm: *mut mm_struct) {
    /* 64-bit Book3E keeps track of current PGD in the PACA */
    (*get_paca()).pgd = (*mm).pgd;
    #[cfg(CONFIG_PPC_KUAP)]
    {
        (*tsk).thread.pid = (*mm).context.id;
    }
}

#[cfg(all(not(CONFIG_PPC32), not(CONFIG_PPC_BOOK3E_64)))]
#[inline]
unsafe fn switch_mm_pgdir(_tsk: *mut task_struct, _mm: *mut mm_struct) {}

pub unsafe fn switch_mm_irqs_off(
    prev: *mut mm_struct,
    next: *mut mm_struct,
    tsk: *mut task_struct,
) {
    let cpu: i32 = smp_processor_id();
    let mut new_on_cpu = false;

    /* Mark this context has been used on the new CPU */
    if !cpumask_test_cpu(cpu, mm_cpumask(next)) {
        VM_WARN_ON_ONCE(next == &mut init_mm);
        cpumask_set_cpu(cpu, mm_cpumask(next));
        inc_mm_active_cpus(next);

        /*
         * This full barrier orders the store to the cpumask above vs
         * a subsequent load which allows this CPU/MMU to begin loading
         * translations for 'next' from page table PTEs into the TLB.
         *
         * When using the radix MMU, that operation is the load of the
         * MMU context id, which is then moved to SPRN_PID.
         *
         * For the hash MMU it is either the first load from slb_cache
         * in switch_slb() to preload the SLBs, or the load of
         * get_user_context which loads the context for the VSID hash
         * to insert a new SLB, in the SLB fault handler.
         *
         * On the other side, the barrier is in mm/tlb-radix.c for
         * radix which orders earlier stores to clear the PTEs before
         * the load of mm_cpumask to check which CPU TLBs should be
         * flushed. For hash, pte_xchg to clear the PTE includes the
         * barrier.
         *
         * This full barrier is also needed by membarrier when
         * switching between processes after store to rq->curr, before
         * user-space memory accesses.
         */
        smp_mb();

        new_on_cpu = true;
    }

    /* Some subarchs need to track the PGD elsewhere */
    switch_mm_pgdir(tsk, next);

    /* Nothing else to do if we aren't actually switching */
    if prev == next {
        return;
    }

    /*
     * We must stop all altivec streams before changing the HW
     * context
     */
    if cpu_has_feature(CPU_FTR_ALTIVEC) {
        core::arch::asm!("dssall");
    }

    if !new_on_cpu {
        membarrier_arch_switch_mm(prev, next, tsk);
    }

    /*
     * The actual HW switching method differs between the various
     * sub architectures. Out of line for now
     */
    switch_mmu_context(prev, next, tsk);

    VM_WARN_ON_ONCE(!cpumask_test_cpu(cpu, mm_cpumask(prev)));
}

#[cfg(not(CONFIG_PPC_BOOK3S_64))]
pub unsafe fn arch_exit_mmap(mm: *mut mm_struct) {
    let frag: *mut core::ffi::c_void = pte_frag_get(&mut (*mm).context);

    if !frag.is_null() {
        pte_frag_destroy(frag);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
