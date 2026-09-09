/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/mmu_context.h
 *
 *  Copyright (C) 1996 Russell King.
 *
 *  Changelog:
 *   27-06-1996 RMK Created
 */

// Dependencies supplied by the surrounding kernel translation.

extern "C" {
    pub fn __check_vmalloc_seq(mm: *mut mm_struct);
}

#[cfg(CONFIG_MMU)]
#[inline]
pub unsafe fn check_vmalloc_seq(mm: *mut mm_struct) {
    if !cfg!(CONFIG_ARM_LPAE)
        && unlikely(atomic_read(&(*mm).context.vmalloc_seq) != atomic_read(&init_mm.context.vmalloc_seq))
    {
        __check_vmalloc_seq(mm);
    }
}

#[cfg(CONFIG_CPU_HAS_ASID)]
extern "C" {
    pub fn check_and_switch_context(mm: *mut mm_struct, tsk: *mut task_struct);
}

#[cfg(CONFIG_CPU_HAS_ASID)]
#[inline]
pub unsafe fn init_new_context(_tsk: *mut task_struct, mm: *mut mm_struct) -> i32 {
    atomic64_set(&mut (*mm).context.id, 0);
    0
}

#[cfg(CONFIG_CPU_HAS_ASID)]
#[cfg(CONFIG_ARM_ERRATA_798181)]
extern "C" {
    pub fn a15_erratum_get_cpumask(this_cpu: i32, mm: *mut mm_struct, mask: *mut cpumask_t);
}

#[cfg(CONFIG_CPU_HAS_ASID)]
#[cfg(not(CONFIG_ARM_ERRATA_798181))]
#[inline]
pub unsafe fn a15_erratum_get_cpumask(
    _this_cpu: i32,
    _mm: *mut mm_struct,
    _mask: *mut cpumask_t,
) {
}

#[cfg(not(CONFIG_CPU_HAS_ASID))]
#[cfg(CONFIG_MMU)]
#[inline]
pub unsafe fn check_and_switch_context(mm: *mut mm_struct, _tsk: *mut task_struct) {
    check_vmalloc_seq(mm);

    if irqs_disabled() {
        (*mm).context.switch_pending = 1;
    } else {
        cpu_switch_mm((*mm).pgd, mm);
    }
}

#[cfg(not(CONFIG_CPU_HAS_ASID))]
#[cfg(CONFIG_MMU)]
#[cfg(not(MODULE))]
#[inline]
pub unsafe fn finish_arch_post_lock_switch() {
    let mm = (*current).mm;

    if !mm.is_null() && (*mm).context.switch_pending != 0 {
        preempt_disable();
        if (*mm).context.switch_pending != 0 {
            (*mm).context.switch_pending = 0;
            cpu_switch_mm((*mm).pgd, mm);
        }
        preempt_enable_no_resched();
    }
}

#[inline]
pub unsafe fn activate_mm(prev: *mut mm_struct, next: *mut mm_struct) {
    switch_mm(prev, next, core::ptr::null_mut());
}

/*
 * This is the actual mm switch as far as the scheduler
 * is concerned. No registers are touched. We avoid
 * calling the CPU specific function when the mm hasn't
 * actually changed.
 */
#[inline]
pub unsafe fn switch_mm(
    prev: *mut mm_struct,
    next: *mut mm_struct,
    tsk: *mut task_struct,
) {
    #[cfg(CONFIG_MMU)]
    {
        let cpu = smp_processor_id();

        /*
         * __sync_icache_dcache doesn't broadcast the I-cache invalidation,
         * so check for possible thread migration and invalidate the I-cache
         * if we're new to this CPU.
         */
        if cache_ops_need_broadcast()
            && !cpumask_empty(mm_cpumask(next))
            && !cpumask_test_cpu(cpu, mm_cpumask(next))
        {
            __flush_icache_all();
        }

        if !cpumask_test_and_set_cpu(cpu, mm_cpumask(next)) || prev != next {
            check_and_switch_context(next, tsk);
            if cache_is_vivt() {
                cpumask_clear_cpu(cpu, mm_cpumask(prev));
            }
        }
    }
}

#[cfg(CONFIG_VMAP_STACK)]
#[inline]
pub unsafe fn enter_lazy_tlb(mm: *mut mm_struct, _tsk: *mut task_struct) {
    if mm != &raw mut init_mm {
        check_vmalloc_seq(mm);
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
