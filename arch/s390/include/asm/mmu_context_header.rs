/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  S390 version
 *
 *  Derived from "include/asm-i386/mmu_context.h"
 */

// Dependencies supplied by the surrounding kernel translation unit:
// asm/pgalloc.h, linux/uaccess.h, linux/mm_types.h, asm/tlbflush.h,
// asm/ctlreg.h, asm/asce.h, and asm-generic/mm_hooks.h.

pub unsafe fn init_new_context(tsk: *mut task_struct, mm: *mut mm_struct) -> ::core::ffi::c_int {
    let (mut asce_type, mut init_entry): (c_ulong, c_ulong);

    spin_lock_init(&mut (*mm).context.lock);
    INIT_LIST_HEAD(&mut (*mm).context.gmap_list);
    cpumask_clear(&mut (*mm).context.cpu_attach_mask);
    atomic_set(&mut (*mm).context.flush_count, 0);
    atomic_set(&mut (*mm).context.protected_count, 0);
    (*mm).context.gmap_asce = 0;
    (*mm).context.flush_mm = 0;
    // Build-time condition: CONFIG_KVM.
    #[cfg(feature = "CONFIG_KVM")]
    {
        (*mm).context.allow_cow_sharing = 1;
    }
    match (*mm).context.asce_limit {
        _REGION2_SIZE => {
            /* forked 3-level task */
            init_entry = _REGION3_ENTRY_EMPTY;
            asce_type = _ASCE_TYPE_REGION3;
        }
        TASK_SIZE_MAX => {
            /* forked 5-level task */
            init_entry = _REGION1_ENTRY_EMPTY;
            asce_type = _ASCE_TYPE_REGION1;
        }
        _REGION1_SIZE => {
            /* forked 4-level task */
            init_entry = _REGION2_ENTRY_EMPTY;
            asce_type = _ASCE_TYPE_REGION2;
        }
        _ => {
            /*
             * context created by exec, the value of asce_limit can
             * only be zero in this case
             */
            VM_BUG_ON((*mm).context.asce_limit);
            /* continue as 3-level task */
            (*mm).context.asce_limit = _REGION2_SIZE;
            init_entry = _REGION3_ENTRY_EMPTY;
            asce_type = _ASCE_TYPE_REGION3;
        }
    }
    (*mm).context.asce = __pa((*mm).pgd) | _ASCE_TABLE_LENGTH
        | _ASCE_USER_BITS | asce_type;
    crst_table_init((*mm).pgd as *mut c_ulong, init_entry);
    0
}

pub unsafe fn switch_mm_irqs_off(
    prev: *mut mm_struct,
    next: *mut mm_struct,
    tsk: *mut task_struct,
) {
    let cpu = smp_processor_id();

    if next == &mut init_mm as *mut mm_struct {
        (*get_lowcore()).user_asce = s390_invalid_asce;
    } else {
        (*get_lowcore()).user_asce.val = (*next).context.asce;
    }
    cpumask_set_cpu(cpu, &mut (*next).context.cpu_attach_mask);
    /* Clear previous user-ASCE from CR1 and CR7 */
    local_ctl_load(1, &s390_invalid_asce);
    local_ctl_load(7, &s390_invalid_asce);
    if prev != next {
        cpumask_clear_cpu(cpu, &mut (*prev).context.cpu_attach_mask);
    }
}

pub unsafe fn switch_mm(
    prev: *mut mm_struct,
    next: *mut mm_struct,
    tsk: *mut task_struct,
) {
    let mut flags: c_ulong = 0;

    local_irq_save(&mut flags);
    switch_mm_irqs_off(prev, next, tsk);
    local_irq_restore(flags);
}

pub unsafe fn finish_arch_post_lock_switch() {
    let tsk = current;
    let mm = (*tsk).mm;
    let mut flags: c_ulong = 0;

    if !mm.is_null() {
        preempt_disable();
        while atomic_read(&(*mm).context.flush_count) != 0 {
            cpu_relax();
        }
        cpumask_set_cpu(smp_processor_id(), mm_cpumask(mm));
        __tlb_flush_mm_lazy(mm);
        preempt_enable();
    }
    local_irq_save(&mut flags);
    if test_thread_flag(TIF_ASCE_PRIMARY) {
        local_ctl_load(1, &(*get_lowcore()).kernel_asce);
    } else {
        local_ctl_load(1, &(*get_lowcore()).user_asce);
    }
    local_ctl_load(7, &(*get_lowcore()).user_asce);
    local_irq_restore(flags);
}

pub unsafe fn activate_mm(prev: *mut mm_struct, next: *mut mm_struct) {
    switch_mm_irqs_off(prev, next, current);
    cpumask_set_cpu(smp_processor_id(), mm_cpumask(next));
    if test_thread_flag(TIF_ASCE_PRIMARY) {
        local_ctl_load(1, &(*get_lowcore()).kernel_asce);
    } else {
        local_ctl_load(1, &(*get_lowcore()).user_asce);
    }
    local_ctl_load(7, &(*get_lowcore()).user_asce);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
