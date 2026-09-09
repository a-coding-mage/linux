// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * This file contains the routines for initializing the MMU
 * on the 8xx series of chips.
 *  -- christophe
 *
 *  Derived from arch/powerpc/mm/40x_mmu.c:
 */

// C dependencies: linux/memblock.h, linux/hugetlb.h, asm/fixmap.h,
// asm/pgalloc.h, and mm/mmu_decl.h.

const IMMR_SIZE: usize = FIX_IMMR_SIZE << PAGE_SHIFT;

static mut block_mapped_ram: c_ulong = 0;

/*
 * Return PA for this VA if it is in an area mapped with LTLBs or fixmap.
 * Otherwise, returns 0
 */
pub unsafe fn v_block_mapped(va: c_ulong) -> phys_addr_t {
    let p: c_ulong = PHYS_IMMR_BASE;

    if va >= VIRT_IMMR_BASE && va < VIRT_IMMR_BASE + IMMR_SIZE {
        return p + va - VIRT_IMMR_BASE;
    }
    if va >= PAGE_OFFSET && va < PAGE_OFFSET + block_mapped_ram {
        return __pa(va);
    }
    0
}

/*
 * Return VA for a given PA mapped with LTLBs or fixmap
 * Return 0 if not mapped
 */
pub unsafe fn p_block_mapped(pa: phys_addr_t) -> c_ulong {
    let p: c_ulong = PHYS_IMMR_BASE;

    if pa >= p && pa < p + IMMR_SIZE {
        return VIRT_IMMR_BASE + pa - p;
    }
    if pa < block_mapped_ram {
        return __va(pa) as c_ulong;
    }
    0
}

unsafe fn __early_map_kernel_hugepage(
    va: c_ulong,
    pa: phys_addr_t,
    prot: pgprot_t,
    psize: c_int,
    new: bool,
) -> c_int {
    let pmdp: *mut pmd_t = pmd_off_k(va);
    let mut ptep: *mut pte_t;
    let shift: c_uint = mmu_psize_to_shift(psize);

    if new {
        if WARN_ON(slab_is_available()) {
            return -EINVAL;
        }

        if psize == MMU_PAGE_8M {
            if WARN_ON(!pmd_none(*pmdp) || !pmd_none(*pmdp.add(1))) {
                return -EINVAL;
            }

            ptep = early_alloc_pgtable(PTE_FRAG_SIZE);
            pmd_populate_kernel(&mut init_mm, pmdp, ptep);

            ptep = early_alloc_pgtable(PTE_FRAG_SIZE);
            pmd_populate_kernel(&mut init_mm, pmdp.add(1), ptep);

            ptep = pmdp as *mut pte_t;
        } else {
            ptep = early_pte_alloc_kernel(pmdp, va);
            /* The PTE should never be already present */
            if WARN_ON(pte_present(*ptep) && pgprot_val(prot) != 0) {
                return -EINVAL;
            }
        }
    } else if psize == MMU_PAGE_8M {
        ptep = pmdp as *mut pte_t;
    } else {
        ptep = pte_offset_kernel(pmdp, va);
    }

    if WARN_ON(ptep.is_null()) {
        return -ENOMEM;
    }

    set_huge_pte_at(
        &mut init_mm,
        va,
        ptep,
        arch_make_huge_pte(pfn_pte(pa >> PAGE_SHIFT, prot), shift, 0),
        1usize << shift,
    );

    0
}

/* MMU_init_hw does the chip-specific initialization of the MMU hardware. */
pub unsafe fn MMU_init_hw() {}

static mut immr_is_mapped: bool = false;

pub unsafe fn mmu_mapin_immr() {
    if immr_is_mapped {
        return;
    }

    immr_is_mapped = true;
    __early_map_kernel_hugepage(
        VIRT_IMMR_BASE,
        PHYS_IMMR_BASE,
        PAGE_KERNEL_NCG,
        MMU_PAGE_512K,
        true,
    );
}

unsafe fn mmu_mapin_ram_chunk(
    offset: c_ulong,
    top: c_ulong,
    prot: pgprot_t,
    new: bool,
) -> c_int {
    let mut v = PAGE_OFFSET + offset;
    let mut p = offset;
    let mut err = 0;

    WARN_ON(!IS_ALIGNED(offset, SZ_16K) || !IS_ALIGNED(top, SZ_16K));

    while p < ALIGN(p, SZ_512K) && p < top && err == 0 {
        err = __early_map_kernel_hugepage(v, p, prot, MMU_PAGE_16K, new);
        p += SZ_16K;
        v += SZ_16K;
    }
    while p < ALIGN(p, SZ_8M) && p < top && err == 0 {
        err = __early_map_kernel_hugepage(v, p, prot, MMU_PAGE_512K, new);
        p += SZ_512K;
        v += SZ_512K;
    }
    while p < ALIGN_DOWN(top, SZ_8M) && p < top && err == 0 {
        err = __early_map_kernel_hugepage(v, p, prot, MMU_PAGE_8M, new);
        p += SZ_8M;
        v += SZ_8M;
    }
    while p < ALIGN_DOWN(top, SZ_512K) && p < top && err == 0 {
        err = __early_map_kernel_hugepage(v, p, prot, MMU_PAGE_512K, new);
        p += SZ_512K;
        v += SZ_512K;
    }
    while p < ALIGN_DOWN(top, SZ_16K) && p < top && err == 0 {
        err = __early_map_kernel_hugepage(v, p, prot, MMU_PAGE_16K, new);
        p += SZ_16K;
        v += SZ_16K;
    }

    if !new {
        flush_tlb_kernel_range(PAGE_OFFSET + v, PAGE_OFFSET + top);
    }
    err
}

pub unsafe fn mmu_mapin_ram(base: c_ulong, top: c_ulong) -> c_ulong {
    let etext8 = ALIGN(__pa(_etext), SZ_8M);
    let sinittext = __pa(_sinittext);
    let strict_boundary = strict_kernel_rwx_enabled() || debug_pagealloc_enabled_or_kfence();
    let boundary = if strict_boundary { sinittext } else { etext8 };
    let einittext8 = ALIGN(__pa(_einittext), SZ_8M);

    WARN_ON(top < einittext8);
    mmu_mapin_immr();
    mmu_mapin_ram_chunk(0, boundary, PAGE_KERNEL_X, true);
    if debug_pagealloc_enabled_or_kfence() {
        top = boundary;
    } else {
        mmu_mapin_ram_chunk(boundary, einittext8, PAGE_KERNEL_X, true);
        mmu_mapin_ram_chunk(einittext8, top, PAGE_KERNEL, true);
    }
    if top > SZ_32M {
        memblock_set_current_limit(top);
    }
    block_mapped_ram = top;
    top
}

pub unsafe fn mmu_mark_initmem_nx() -> c_int {
    let etext8 = ALIGN(__pa(_etext), SZ_8M);
    let sinittext = __pa(_sinittext);
    let boundary = if strict_kernel_rwx_enabled() { sinittext } else { etext8 };
    let einittext8 = ALIGN(__pa(_einittext), SZ_8M);
    let mut err = 0;

    if !debug_pagealloc_enabled_or_kfence() {
        err = mmu_mapin_ram_chunk(boundary, einittext8, PAGE_KERNEL, false);
    }
    if IS_ENABLED(CONFIG_PIN_TLB_TEXT) {
        mmu_pin_tlb(block_mapped_ram, false);
    }
    err
}

// #ifdef CONFIG_STRICT_KERNEL_RWX
#[cfg(feature = "CONFIG_STRICT_KERNEL_RWX")]
pub unsafe fn mmu_mark_rodata_ro() -> c_int {
    let sinittext = __pa(_sinittext);
    let err = mmu_mapin_ram_chunk(0, sinittext, PAGE_KERNEL_ROX, false);
    if IS_ENABLED(CONFIG_PIN_TLB_DATA) {
        mmu_pin_tlb(block_mapped_ram, true);
    }
    err
}
// #endif

pub unsafe fn setup_initial_memory_limit(
    first_memblock_base: phys_addr_t,
    first_memblock_size: phys_addr_t,
) {
    /* We don't currently support the first MEMBLOCK not mapping 0
     * physical on those processors
     */
    BUG_ON(first_memblock_base != 0);
    /* 8xx can only access 32MB at the moment */
    memblock_set_current_limit(min_t(first_memblock_size, SZ_32M));
}

pub unsafe fn pud_clear_huge(_pud: *mut pud_t) -> c_int {
    0
}

pub unsafe fn pmd_clear_huge(_pmd: *mut pmd_t) -> c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
