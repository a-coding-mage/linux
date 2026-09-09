/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: linux/mm.h, linux/slab.h
use core::ffi::{c_int, c_ulong};

extern "C" {
    pub fn tlb_remove_table(tlb: *mut mmu_gather, table: *mut core::ffi::c_void);
}

// CONFIG_PPC64 provides this declaration.  For non-CONFIG_PPC64 builds the
// C header provides an empty inline implementation instead.
#[cfg(feature = "CONFIG_PPC64")]
extern "C" {
    pub fn tlb_flush_pgtable(tlb: *mut mmu_gather, address: c_ulong);
}

#[cfg(not(feature = "CONFIG_PPC64"))]
#[inline]
pub unsafe fn tlb_flush_pgtable(_tlb: *mut mmu_gather, _address: c_ulong) {
}

#[inline]
pub unsafe fn pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t {
    let pgd = kmem_cache_alloc(
        PGT_CACHE(PGD_INDEX_SIZE),
        pgtable_gfp_flags(mm, GFP_KERNEL),
    ) as *mut pgd_t;

    // CONFIG_PPC_8xx: copy the kernel portion of the page directory.
    #[cfg(feature = "CONFIG_PPC_8xx")]
    {
        core::ptr::copy_nonoverlapping(
            swapper_pg_dir.add(USER_PTRS_PER_PGD),
            pgd.add(USER_PTRS_PER_PGD),
            (MAX_PTRS_PER_PGD - USER_PTRS_PER_PGD),
        );
    }
    pgd
}

#[inline]
pub unsafe fn pgd_free(_mm: *mut mm_struct, pgd: *mut pgd_t) {
    kmem_cache_free(PGT_CACHE(PGD_INDEX_SIZE), pgd as *mut core::ffi::c_void);
}

// CONFIG_PPC64 includes asm/nohash/64/pgalloc.h; other builds include
// asm/nohash/32/pgalloc.h.  Those declarations are supplied externally.

#[inline]
pub unsafe fn pgtable_free(table: *mut core::ffi::c_void, shift: c_int) {
    if shift == 0 {
        pte_fragment_free(table as *mut c_ulong, 0);
    } else {
        BUG_ON(shift > MAX_PGTABLE_INDEX_SIZE);
        kmem_cache_free(PGT_CACHE(shift), table);
    }
}

#[inline]
pub unsafe fn pgtable_free_tlb(
    tlb: *mut mmu_gather,
    table: *mut core::ffi::c_void,
    shift: c_int,
) {
    let mut pgf = table as c_ulong;

    BUG_ON(shift > MAX_PGTABLE_INDEX_SIZE);
    pgf |= shift as c_ulong;
    tlb_remove_table(tlb, pgf as *mut core::ffi::c_void);
}

#[inline]
pub unsafe fn __tlb_remove_table(_table: *mut core::ffi::c_void) {
    let table = ((_table as c_ulong) & !(MAX_PGTABLE_INDEX_SIZE as c_ulong))
        as *mut core::ffi::c_void;
    let shift = (_table as c_ulong) & (MAX_PGTABLE_INDEX_SIZE as c_ulong);

    pgtable_free(table, shift as c_int);
}

#[inline]
pub unsafe fn __pte_free_tlb(
    tlb: *mut mmu_gather,
    table: pgtable_t,
    address: c_ulong,
) {
    tlb_flush_pgtable(tlb, address);
    pgtable_free_tlb(tlb, table as *mut core::ffi::c_void, 0);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
