// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  MMU context allocation for 64-bit kernels.
 *
 *  Copyright (C) 2004 Anton Blanchard, IBM Corp. <anton@samba.org>
 */

// Kernel headers and architecture headers from the C source provide the
// external types, constants, functions, and configuration conditions used here.

static mut MMU_CONTEXT_IDA: ida = DEFINE_IDA!();

unsafe fn alloc_context_id(min_id: i32, max_id: i32) -> i32 {
    ida_alloc_range(&raw mut MMU_CONTEXT_IDA, min_id, max_id, GFP_KERNEL)
}

#[cfg(CONFIG_PPC_64S_HASH_MMU)]
pub unsafe fn hash__reserve_context_id(id: i32) {
    let result = ida_alloc_range(&raw mut MMU_CONTEXT_IDA, id, id, GFP_KERNEL);
    WARN(result != id, "mmu: Failed to reserve context id %d (rc %d)\n", id, result);
}

#[cfg(CONFIG_PPC_64S_HASH_MMU)]
pub unsafe fn hash__alloc_context_id() -> i32 {
    let max: i32;

    if mmu_has_feature(MMU_FTR_68_BIT_VA) {
        max = MAX_USER_CONTEXT;
    } else {
        max = MAX_USER_CONTEXT_65BIT_VA;
    }

    alloc_context_id(MIN_USER_CONTEXT, max)
}

#[cfg(CONFIG_PPC_64S_HASH_MMU)]
unsafe fn realloc_context_ids(ctx: *mut mm_context_t) -> i32 {
    let mut i: i32;
    let mut id: i32;

    for index in 0..(*ctx).extended_id.len() {
        i = index as i32;
        if index == 0 || (*ctx).extended_id[index] != 0 {
            id = hash__alloc_context_id();
            if id < 0 {
                i -= 1;
                while i >= 0 {
                    if (*ctx).extended_id[i as usize] != 0 {
                        ida_free(&raw mut MMU_CONTEXT_IDA, (*ctx).extended_id[i as usize]);
                    }
                    i -= 1;
                }
                return id;
            }
            (*ctx).extended_id[index] = id;
        }
    }

    /* The caller expects us to return id */
    (*ctx).id
}

#[cfg(CONFIG_PPC_64S_HASH_MMU)]
unsafe fn hash__init_new_context(mm: *mut mm_struct) -> i32 {
    let index: i32;

    (*mm).context.hash_context = kmalloc_obj::<hash_mm_context>();
    if (*mm).context.hash_context.is_null() {
        return -ENOMEM;
    }

    if (*mm).context.id == 0 {
        core::ptr::write_bytes((*mm).context.hash_context, 0, 1);
        slice_init_new_context_exec(mm);
    } else {
        core::ptr::copy_nonoverlapping(
            (*current).mm.context.hash_context,
            (*mm).context.hash_context,
            1,
        );
        #[cfg(CONFIG_PPC_SUBPAGE_PROT)]
        if !(*current).mm.context.hash_context.spt.is_null() {
            (*mm).context.hash_context.spt = kmalloc_obj::<subpage_prot_table>();
            if (*mm).context.hash_context.spt.is_null() {
                kfree((*mm).context.hash_context);
                return -ENOMEM;
            }
        }
    }

    index = realloc_context_ids(&raw mut (*mm).context);
    if index < 0 {
        #[cfg(CONFIG_PPC_SUBPAGE_PROT)]
        kfree((*mm).context.hash_context.spt);
        kfree((*mm).context.hash_context);
        return index;
    }

    pkey_mm_init(mm);
    index
}

#[cfg(CONFIG_PPC_64S_HASH_MMU)]
pub unsafe fn hash__setup_new_exec() {
    slice_setup_new_exec();
}

#[cfg(not(CONFIG_PPC_64S_HASH_MMU))]
unsafe fn hash__init_new_context(_mm: *mut mm_struct) -> i32 {
    BUILD_BUG!();
    0
}

unsafe fn radix__init_new_context(mm: *mut mm_struct) -> i32 {
    let max_id = (1i32 << mmu_pid_bits) - 1;
    let index = alloc_context_id(mmu_base_pid, max_id);
    if index < 0 {
        return index;
    }

    let rts_field = radix__get_tree_size();
    process_tb[index as usize].prtb0 = cpu_to_be64(rts_field | __pa((*mm).pgd) | RADIX_PGD_INDEX_SIZE);
    asm!("ptesync;isync", options(nostack, preserves_flags));

    #[cfg(CONFIG_PPC_64S_HASH_MMU)]
    {
        (*mm).context.hash_context = core::ptr::null_mut();
    }

    index
}

pub unsafe fn init_new_context(_tsk: *mut task_struct, mm: *mut mm_struct) -> i32 {
    let index = if radix_enabled() {
        radix__init_new_context(mm)
    } else {
        hash__init_new_context(mm)
    };

    if index < 0 {
        return index;
    }

    (*mm).context.id = index;
    (*mm).context.pte_frag = core::ptr::null_mut();
    (*mm).context.pmd_frag = core::ptr::null_mut();
    #[cfg(CONFIG_SPAPR_TCE_IOMMU)]
    mm_iommu_init(mm);
    atomic_set(&raw mut (*mm).context.active_cpus, 0);
    atomic_set(&raw mut (*mm).context.copros, 0);
    0
}

pub unsafe fn __destroy_context(context_id: i32) {
    ida_free(&raw mut MMU_CONTEXT_IDA, context_id);
}

unsafe fn destroy_contexts(ctx: *mut mm_context_t) {
    if radix_enabled() {
        ida_free(&raw mut MMU_CONTEXT_IDA, (*ctx).id);
    } else {
        #[cfg(CONFIG_PPC_64S_HASH_MMU)]
        {
            for index in 0..(*ctx).extended_id.len() {
                let context_id = (*ctx).extended_id[index];
                if context_id != 0 {
                    ida_free(&raw mut MMU_CONTEXT_IDA, context_id);
                }
            }
            kfree((*ctx).hash_context);
        }
        #[cfg(not(CONFIG_PPC_64S_HASH_MMU))]
        BUILD_BUG!();
    }
}

unsafe fn pmd_frag_destroy(pmd_frag: *mut core::ffi::c_void) {
    let ptdesc = virt_to_ptdesc(pmd_frag);
    let count = ((pmd_frag as usize & !PAGE_MASK) >> PMD_FRAG_SIZE_SHIFT) as i32;
    if atomic_sub_and_test(PMD_FRAG_NR - count, &raw mut (*ptdesc).pt_frag_refcount) {
        pagetable_dtor(ptdesc);
        pagetable_free(ptdesc);
    }
}

unsafe fn destroy_pagetable_cache(mm: *mut mm_struct) {
    let frag = (*mm).context.pte_frag;
    if !frag.is_null() {
        pte_frag_destroy(frag);
    }
    let frag = (*mm).context.pmd_frag;
    if !frag.is_null() {
        pmd_frag_destroy(frag);
    }
}

pub unsafe fn destroy_context(mm: *mut mm_struct) {
    #[cfg(CONFIG_SPAPR_TCE_IOMMU)]
    WARN_ON_ONCE(!list_empty(&raw mut (*mm).context.iommu_group_mem_list));

    if radix_enabled() {
        process_tb[(*mm).context.id as usize].prtb0 = 0;
    } else {
        subpage_prot_free(mm);
    }
    destroy_contexts(&raw mut (*mm).context);
    (*mm).context.id = MMU_NO_CONTEXT;
}

pub unsafe fn arch_exit_mmap(mm: *mut mm_struct) {
    destroy_pagetable_cache(mm);
    if radix_enabled() {
        process_tb[(*mm).context.id as usize].prtb0 = 0;
    }
}

#[cfg(CONFIG_PPC_RADIX_MMU)]
pub unsafe fn radix__switch_mmu_context(_prev: *mut mm_struct, next: *mut mm_struct) {
    mtspr(SPRN_PID, (*next).context.id);
    isync();
}

/**
 * cleanup_cpu_mmu_context - Clean up MMU details for this CPU (newly offlined)
 *
 * This clears the CPU from mm_cpumask for all processes, and then flushes the
 * local TLB to ensure TLB coherency in case the CPU is onlined again.
 *
 * KVM guest translations are not necessarily flushed here. If KVM started
 * using mm_cpumask or the Linux APIs which do, this would have to be resolved.
 */
#[cfg(CONFIG_HOTPLUG_CPU)]
pub unsafe fn cleanup_cpu_mmu_context() {
    let cpu = smp_processor_id();
    clear_tasks_mm_cpumask(cpu);
    tlbiel_all();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
