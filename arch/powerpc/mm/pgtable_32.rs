// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * This file contains the routines setting up the linux page tables.
 *  -- paulus
 *
 *  Derived from arch/ppc/mm/init.c:
 *    Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
 *
 *  Modifications by Paul Mackerras (PowerMac) (paulus@cs.anu.edu.au)
 *  and Cort Dougan (PReP) (cort@cs.nmt.edu)
 *    Copyright (C) 1996 Paul Mackerras
 *
 *  Derived from "arch/i386/mm/init.c"
 *    Copyright (C) 1991, 1992, 1993, 1994  Linus Torvalds
 */

// C dependencies supplied by other translation units.

static mut early_fixmap_pagetable: [u8; FIXMAP_PTE_SIZE] = [0; FIXMAP_PTE_SIZE];

#[no_mangle]
pub unsafe fn early_ioremap_init() {
    let mut addr: c_ulong = ALIGN_DOWN(FIXADDR_START, PGDIR_SIZE);
    let mut ptep: *mut pte_t = early_fixmap_pagetable.as_mut_ptr() as *mut pte_t;
    let mut pmdp: *mut pmd_t = pmd_off_k(addr);

    while (FIXADDR_TOP.wrapping_sub(addr) as c_s32) > 0 {
        pmd_populate_kernel(&init_mm, pmdp, ptep);
        addr = addr.wrapping_add(PGDIR_SIZE);
        ptep = ptep.add(PTRS_PER_PTE);
        pmdp = pmdp.add(1);
    }

    early_ioremap_setup();
}

#[no_mangle]
pub unsafe fn early_alloc_pgtable(size: c_ulong) -> *mut c_void {
    memblock_alloc_or_panic(size, size)
}

#[no_mangle]
pub unsafe fn early_pte_alloc_kernel(pmdp: *mut pmd_t, va: c_ulong) -> *mut pte_t {
    if pmd_none(*pmdp) {
        let ptep: *mut pte_t = early_alloc_pgtable(PTE_FRAG_SIZE) as *mut pte_t;
        pmd_populate_kernel(&init_mm, pmdp, ptep);
    }
    pte_offset_kernel(pmdp, va)
}

#[no_mangle]
pub unsafe fn map_kernel_page(va: c_ulong, pa: phys_addr_t, prot: pgprot_t) -> c_int {
    let pd: *mut pmd_t;
    let pg: *mut pte_t;
    let mut err: c_int = -ENOMEM;

    // Use upper 10 bits of VA to index the first level map
    pd = pmd_off_k(va);
    // Use middle 10 bits of VA to index the second-level map
    if likely(slab_is_available()) {
        pg = pte_alloc_kernel(pd, va);
    } else {
        pg = early_pte_alloc_kernel(pd, va);
    }
    if !pg.is_null() {
        err = 0;
        // The PTE should never be already set nor present in the hash table
        BUG_ON((pte_present(*pg) | pte_hashpte(*pg)) && pgprot_val(prot));
        set_pte_at(&init_mm, va, pg, pfn_pte(pa >> PAGE_SHIFT, prot));
    }
    smp_wmb();
    err
}

/*
 * Map in a chunk of physical memory starting at start.
 */
unsafe fn __mapin_ram_chunk(offset: c_ulong, top: c_ulong) {
    let mut v: c_ulong = PAGE_OFFSET.wrapping_add(offset);
    let mut s: c_ulong = offset;
    let mut p: phys_addr_t = memstart_addr.wrapping_add(offset as phys_addr_t);
    let mut ktext: bool;

    while s < top {
        ktext = core_kernel_text(v);
        map_kernel_page(v, p, if ktext { PAGE_KERNEL_X } else { PAGE_KERNEL });
        s = s.wrapping_add(PAGE_SIZE);
        v = v.wrapping_add(PAGE_SIZE);
        p = p.wrapping_add(PAGE_SIZE as phys_addr_t);
    }
}

#[no_mangle]
pub unsafe fn mapin_ram() {
    let mut base: phys_addr_t;
    let mut end: phys_addr_t;
    let mut i: u64 = 0;

    for_each_mem_range(&mut i, &mut base, &mut end) {
        let top: phys_addr_t = min(end, total_lowmem);

        if base >= top {
            continue;
        }
        base = mmu_mapin_ram(base, top);
        __mapin_ram_chunk(base, top);
    }
}

unsafe fn __mark_initmem_nx() -> c_int {
    let numpages: c_ulong = PFN_UP((&_einittext as *const _ as c_ulong))
        .wrapping_sub(PFN_DOWN((&_sinittext as *const _ as c_ulong)));
    let mut err: c_int;

    err = mmu_mark_initmem_nx();

    if !v_block_mapped(&_sinittext as *const _ as c_ulong) {
        err = set_memory_nx(&_sinittext as *const _ as c_ulong, numpages);
        if err != 0 {
            return err;
        }
        err = set_memory_rw(&_sinittext as *const _ as c_ulong, numpages);
    }
    err
}

#[no_mangle]
pub unsafe fn mark_initmem_nx() {
    let err: c_int = __mark_initmem_nx();

    if err != 0 {
        panic!("%s() failed, err = %d\n", __func__, err);
    }
}

#[cfg(CONFIG_STRICT_KERNEL_RWX)]
unsafe fn __mark_rodata_ro() -> c_int {
    let numpages: c_ulong;

    if IS_ENABLED(CONFIG_STRICT_MODULE_RWX) && mmu_has_feature(MMU_FTR_HPTE_TABLE) {
        pr_warn!("This platform has HASH MMU, STRICT_MODULE_RWX won't work\n");
    }

    if v_block_mapped((&_stext as *const _ as c_ulong).wrapping_add(1)) {
        return mmu_mark_rodata_ro();
    }

    /*
     * mark text and rodata as read only. __end_rodata is set by
     * powerpc's linker script and includes tables and data
     * requiring relocation which are not put in RO_DATA.
     */
    numpages = PFN_UP(&__end_rodata as *const _ as c_ulong)
        .wrapping_sub(PFN_DOWN(&_stext as *const _ as c_ulong));

    set_memory_ro(&_stext as *const _ as c_ulong, numpages)
}

#[cfg(CONFIG_STRICT_KERNEL_RWX)]
#[no_mangle]
pub unsafe fn mark_rodata_ro() {
    let err: c_int = __mark_rodata_ro();

    if err != 0 {
        panic!("%s() failed, err = %d\n", __func__, err);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
