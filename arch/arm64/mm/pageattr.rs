// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014, The Linux Foundation. All rights reserved.
 */

#[repr(C)]
struct page_change_data {
    set_mask: pgprot_t,
    clear_mask: pgprot_t,
}

unsafe fn set_pageattr_masks(val: ptval_t, walk: *mut mm_walk) -> ptval_t {
    let masks = (*walk).private as *mut page_change_data;

    /*
     * Some users clear and set bits which alias each other (e.g. PTE_NG and
     * PTE_PRESENT_INVALID). It is therefore important that we always clear
     * first then set.
     */
    let val = val & !(pgprot_val((*masks).clear_mask));
    val | pgprot_val((*masks).set_mask)
}

unsafe fn pageattr_pud_entry(
    pud: *mut pud_t,
    addr: c_ulong,
    next: c_ulong,
    walk: *mut mm_walk,
) -> c_int {
    let mut val = pudp_get(pud);

    if pud_leaf(val) {
        if WARN_ON_ONCE((next - addr) != PUD_SIZE) {
            return -EINVAL;
        }
        val = __pud(set_pageattr_masks(pud_val(val), walk));
        set_pud(pud, val);
        (*walk).action = ACTION_CONTINUE;
    }

    0
}

unsafe fn pageattr_pmd_entry(
    pmd: *mut pmd_t,
    addr: c_ulong,
    next: c_ulong,
    walk: *mut mm_walk,
) -> c_int {
    let mut val = pmdp_get(pmd);

    if pmd_leaf(val) {
        if WARN_ON_ONCE((next - addr) != PMD_SIZE) {
            return -EINVAL;
        }
        val = __pmd(set_pageattr_masks(pmd_val(val), walk));
        set_pmd(pmd, val);
        (*walk).action = ACTION_CONTINUE;
    }

    0
}

unsafe fn pageattr_pte_entry(
    pte: *mut pte_t,
    _addr: c_ulong,
    _next: c_ulong,
    walk: *mut mm_walk,
) -> c_int {
    let mut val = __ptep_get(pte);

    val = __pte(set_pageattr_masks(pte_val(val), walk));
    __set_pte(pte, val);

    0
}

static pageattr_ops: mm_walk_ops = mm_walk_ops {
    pud_entry: Some(pageattr_pud_entry),
    pmd_entry: Some(pageattr_pmd_entry),
    pte_entry: Some(pageattr_pte_entry),
};

static mut rodata_full: bool = true;

unsafe fn can_set_direct_map() -> bool {
    /*
     * rodata_full, DEBUG_PAGEALLOC and a Realm guest all require linear
     * map to be mapped at page granularity, so that it is possible to
     * protect/unprotect single pages.
     *
     * KFENCE pool requires page-granular mapping if initialized late.
     *
     * Realms need to make pages shared/protected at page granularity.
     */
    rodata_full || debug_pagealloc_enabled() ||
        arm64_kfence_can_set_direct_map() || is_realm_world()
}

unsafe fn update_range_prot(
    start: c_ulong,
    size: c_ulong,
    set_mask: pgprot_t,
    clear_mask: pgprot_t,
) -> c_int {
    let mut data = page_change_data { set_mask, clear_mask };

    let mut ret = split_kernel_leaf_mapping(start, start + size);
    if WARN_ON_ONCE(ret != 0) {
        return ret;
    }

    lazy_mmu_mode_enable();

    /*
     * The caller must ensure that the range we are operating on does not
     * partially overlap a block mapping, or a cont mapping. Any such case
     * must be eliminated by splitting the mapping.
     */
    ret = walk_kernel_page_table_range_lockless(
        start,
        start + size,
        &pageattr_ops,
        core::ptr::null_mut(),
        &mut data,
    );
    lazy_mmu_mode_disable();

    ret
}

unsafe fn __change_memory_common(
    start: c_ulong,
    size: c_ulong,
    set_mask: pgprot_t,
    clear_mask: pgprot_t,
) -> c_int {
    let ret = update_range_prot(start, size, set_mask, clear_mask);

    /*
     * If the memory is being switched from present-invalid to valid without
     * changing any other bits then a TLBI isn't required as a non-valid
     * entry cannot be cached in the TLB.
     */
    if pgprot_val(set_mask) != PTE_PRESENT_VALID_KERNEL ||
        pgprot_val(clear_mask) != PTE_PRESENT_INVALID
    {
        flush_tlb_kernel_range(start, start + size);
    }
    ret
}

unsafe fn change_memory_common(
    addr: c_ulong,
    mut numpages: c_int,
    set_mask: pgprot_t,
    clear_mask: pgprot_t,
) -> c_int {
    let mut start = addr;
    let size = PAGE_SIZE * numpages as c_ulong;
    let mut end = start + size;
    let area: *mut vm_struct;
    let ret: c_int;

    if !PAGE_ALIGNED(addr) {
        start &= PAGE_MASK;
        end = start + size;
        WARN_ON_ONCE(true);
    }

    /* See the C implementation for the VM area restrictions enforced here. */
    area = find_vm_area(addr as *mut core::ffi::c_void);
    if area.is_null() ||
        (kasan_reset_tag(end as *mut core::ffi::c_void) as c_ulong >
            kasan_reset_tag((*area).addr) as c_ulong + (*area).size) ||
        (((*area).flags & (VM_ALLOC | VM_ALLOW_HUGE_VMAP)) != VM_ALLOC)
    {
        return -EINVAL;
    }

    if numpages == 0 {
        return 0;
    }

    if rodata_full &&
        (pgprot_val(set_mask) == PTE_RDONLY || pgprot_val(clear_mask) == PTE_RDONLY)
    {
        let mut idx = ((kasan_reset_tag(start as *mut core::ffi::c_void) as c_ulong -
            kasan_reset_tag((*area).addr) as c_ulong) >> PAGE_SHIFT) as usize;
        while numpages != 0 {
            ret = __change_memory_common(
                page_address((*area).pages.add(idx)) as u64,
                PAGE_SIZE,
                set_mask,
                clear_mask,
            );
            if ret != 0 {
                return ret;
            }
            idx += 1;
            numpages -= 1;
        }
    }

    /* Get rid of potentially aliasing lazily unmapped vm areas. */
    vm_unmap_aliases();

    __change_memory_common(start, size, set_mask, clear_mask)
}

unsafe fn set_memory_ro(addr: c_ulong, numpages: c_int) -> c_int {
    change_memory_common(addr, numpages, __pgprot(PTE_RDONLY), __pgprot(PTE_WRITE))
}

unsafe fn set_memory_rw(addr: c_ulong, numpages: c_int) -> c_int {
    change_memory_common(addr, numpages, __pgprot(PTE_WRITE), __pgprot(PTE_RDONLY))
}

unsafe fn set_memory_nx(addr: c_ulong, numpages: c_int) -> c_int {
    change_memory_common(addr, numpages, __pgprot(PTE_PXN), __pgprot(PTE_MAYBE_GP))
}

unsafe fn set_memory_x(addr: c_ulong, numpages: c_int) -> c_int {
    change_memory_common(addr, numpages, __pgprot(PTE_MAYBE_GP), __pgprot(PTE_PXN))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
