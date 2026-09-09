/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * TLB shootdown specifics for powerpc
 *
 * Copyright (C) 2002 Anton Blanchard, IBM Corp.
 * Copyright (C) 2002 Paul Mackerras, IBM Corp.
 */

/* C header guard: _ASM_POWERPC_TLB_H */
/* Kernel-only declarations from the original header. */

/* Dependencies supplied by the surrounding translation unit:
 * mmu_gather, pte_t, mm_struct, radix_enabled, pte_val, _PAGE_HASHPTE,
 * flush_hash_entry, mm_cpumask, topology_sibling_cpumask,
 * smp_processor_id, atomic_read, cpumask_subset, cpumask_test_cpu,
 * cpumask_equal, and cpumask_of.
 */

#[allow(non_camel_case_types)]
pub type __tlb_remove_tlb_entry_fn = unsafe fn(
    tlb: *mut mmu_gather,
    ptep: *mut pte_t,
    address: ::core::ffi::c_ulong,
);

pub unsafe extern "C" {
    pub fn tlb_flush(tlb: *mut mmu_gather);
}

/*
 * book3s:
 * Hash does not use the linux page-tables, so we can avoid
 * the TLB invalidate for page-table freeing, Radix otoh does use
 * the page-tables and needs the TLBI.
 *
 * nohash:
 * We still do TLB invalidate in the __pte_free_tlb routine before
 * we add the page table pages to mmu gather table batch.
 */
#[inline(always)]
pub unsafe fn tlb_needs_table_invalidate() -> bool {
    radix_enabled()
}

/* Get the generic bits... */

#[inline(always)]
pub unsafe fn __tlb_remove_tlb_entry(
    tlb: *mut mmu_gather,
    ptep: *mut pte_t,
    address: ::core::ffi::c_ulong,
) {
    /* CONFIG_PPC_BOOK3S_32 */
    #[cfg(CONFIG_PPC_BOOK3S_32)]
    {
        if (pte_val(*ptep) & _PAGE_HASHPTE) != 0 {
            flush_hash_entry((*tlb).mm, ptep, address);
        }
    }
}

/* CONFIG_SMP */
#[cfg(CONFIG_SMP)]
#[inline(always)]
pub unsafe fn mm_is_core_local(mm: *mut mm_struct) -> ::core::ffi::c_int {
    cpumask_subset(
        mm_cpumask(mm),
        topology_sibling_cpumask(smp_processor_id()),
    )
}

/* CONFIG_PPC_BOOK3S_64 */
#[cfg(all(CONFIG_SMP, CONFIG_PPC_BOOK3S_64))]
#[inline(always)]
pub unsafe fn mm_is_thread_local(mm: *mut mm_struct) -> bool {
    if atomic_read(&(*mm).context.active_cpus) > 1 {
        return false;
    }
    cpumask_test_cpu(smp_processor_id(), mm_cpumask(mm))
}

/* !CONFIG_PPC_BOOK3S_64 */
#[cfg(all(CONFIG_SMP, not(CONFIG_PPC_BOOK3S_64)))]
#[inline(always)]
pub unsafe fn mm_is_thread_local(mm: *mut mm_struct) -> bool {
    cpumask_equal(mm_cpumask(mm), cpumask_of(smp_processor_id()))
}

/* !CONFIG_SMP */
#[cfg(not(CONFIG_SMP))]
#[inline(always)]
pub unsafe fn mm_is_core_local(_mm: *mut mm_struct) -> ::core::ffi::c_int {
    1
}

#[cfg(not(CONFIG_SMP))]
#[inline(always)]
pub unsafe fn mm_is_thread_local(_mm: *mut mm_struct) -> ::core::ffi::c_int {
    1
}

#[inline(always)]
pub unsafe fn arch_supports_page_table_move() -> bool {
    radix_enabled()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
