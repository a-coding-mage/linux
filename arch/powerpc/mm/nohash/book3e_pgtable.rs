// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2005, Paul Mackerras, IBM Corporation.
 * Copyright 2009, Benjamin Herrenschmidt, IBM Corporation.
 * Copyright 2015-2016, Aneesh Kumar K.V, IBM Corporation.
 */

// Dependency intent preserved from the C source:
// linux/sched.h, linux/memblock.h, asm/pgalloc.h, asm/tlb.h, asm/dma.h,
// asm/text-patching.h, and mm/mmu_decl.h

// CONFIG_SPARSEMEM_VMEMMAP
#[cfg(feature = "CONFIG_SPARSEMEM_VMEMMAP")]
pub unsafe fn vmemmap_create_mapping(
    start: c_ulong,
    page_size: c_ulong,
    phys: c_ulong,
) -> c_int {
    /* Create a PTE encoding without page size */
    let mut flags: c_ulong = _PAGE_PRESENT | _PAGE_ACCESSED | _PAGE_KERNEL_RW;

    /* PTEs only contain page size encodings up to 32M */
    BUG_ON(mmu_psize_defs[mmu_vmemmap_psize].shift - 10 > 0xf);

    /* Encode the size in the PTE */
    flags |= (mmu_psize_defs[mmu_vmemmap_psize].shift - 10) << 8;

    /* For each PTE for that area, map things. Note that we don't
     * increment phys because all PTEs are of the large size and
     * thus must have the low bits clear
    +     */
    let mut i: c_ulong = 0;
    while i < page_size {
        BUG_ON(map_kernel_page(start + i, phys, __pgprot(flags)));
        i += PAGE_SIZE;
    }

    0
}

// CONFIG_MEMORY_HOTPLUG
#[cfg(feature = "CONFIG_MEMORY_HOTPLUG")]
pub unsafe fn vmemmap_remove_mapping(_start: c_ulong, _page_size: c_ulong) {}

unsafe fn early_alloc_pgtable(size: c_ulong) -> *mut c_void {
    let ptr: *mut c_void = memblock_alloc_try_nid(
        size,
        size,
        MEMBLOCK_LOW_LIMIT,
        __pa(MAX_DMA_ADDRESS),
        NUMA_NO_NODE,
    );

    if ptr.is_null() {
        panic!(
            "{}: Failed to allocate {} bytes align=0x{:x} max_addr={:x}\n",
            "early_alloc_pgtable",
            size,
            size,
            __pa(MAX_DMA_ADDRESS)
        );
    }

    ptr
}

/*
 * map_kernel_page currently only called by __ioremap
 * map_kernel_page adds an entry to the ioremap page table
 * and adds an entry to the HPT, possibly bolting it
 */
pub unsafe fn map_kernel_page(ea: c_ulong, pa: phys_addr_t, prot: pgprot_t) -> c_int {
    let pgdp: *mut pgd_t;
    let p4dp: *mut p4d_t;
    let pudp: *mut pud_t;
    let pmdp: *mut pmd_t;
    let ptep: *mut pte_t;

    BUILD_BUG_ON(TASK_SIZE_USER64 > PGTABLE_RANGE);
    if slab_is_available() {
        pgdp = pgd_offset_k(ea);
        p4dp = p4d_offset(pgdp, ea);
        pudp = pud_alloc(&mut init_mm, p4dp, ea);
        if pudp.is_null() {
            return -ENOMEM;
        }
        pmdp = pmd_alloc(&mut init_mm, pudp, ea);
        if pmdp.is_null() {
            return -ENOMEM;
        }
        ptep = pte_alloc_kernel(pmdp, ea);
        if ptep.is_null() {
            return -ENOMEM;
        }
    } else {
        pgdp = pgd_offset_k(ea);
        p4dp = p4d_offset(pgdp, ea);
        if p4d_none(*p4dp) {
            let new_pudp = early_alloc_pgtable(PUD_TABLE_SIZE);
            p4d_populate(&mut init_mm, p4dp, new_pudp);
        }
        pudp = pud_offset(p4dp, ea);
        if pud_none(*pudp) {
            let new_pmdp = early_alloc_pgtable(PMD_TABLE_SIZE);
            pud_populate(&mut init_mm, pudp, new_pmdp);
        }
        pmdp = pmd_offset(pudp, ea);
        if !pmd_present(*pmdp) {
            let new_ptep = early_alloc_pgtable(PTE_TABLE_SIZE);
            pmd_populate_kernel(&mut init_mm, pmdp, new_ptep);
        }
        ptep = pte_offset_kernel(pmdp, ea);
    }
    set_pte_at(
        &mut init_mm,
        ea,
        ptep,
        pfn_pte(pa >> PAGE_SHIFT, prot),
    );

    smp_wmb();
    0
}

pub unsafe fn __patch_exception(exc: c_int, addr: c_ulong) {
    let ibase: *mut c_uint = &mut interrupt_base_book3e;

    /*
     * Our exceptions vectors start with a NOP and -then- a branch
     * to deal with single stepping from userspace which stops on
     * the second instruction. Thus we need to patch the second
     * instruction of the exception, not the first one.
     */

    patch_branch(ibase.add((exc / 4 + 1) as usize), addr, 0);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
