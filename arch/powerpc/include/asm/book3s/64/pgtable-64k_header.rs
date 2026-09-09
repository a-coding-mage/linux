/* SPDX-License-Identifier: GPL-2.0 */

/*
 * C header guard: _ASM_POWERPC_BOOK3S_64_PGTABLE_64K_H
 *
 * The declarations below are available only when not building for the
 * assembler and when CONFIG_HUGETLB_PAGE is enabled, as in the source
 * header.  The CONFIG_HUGETLB_PAGE section contains no declarations.
 */

#[cfg(not(any(target_arch = "asm")))]
#[inline]
pub unsafe fn remap_4k_pfn(
    vma: *mut vm_area_struct,
    addr: usize,
    pfn: usize,
    prot: pgprot_t,
) -> i32 {
    if radix_enabled() {
        BUG();
    }
    hash__remap_4k_pfn(vma, addr, pfn, prot)
}

/* External types and functions supplied by other headers/translation units. */
extern "C" {
    fn radix_enabled() -> bool;
    fn BUG() -> !;
    fn hash__remap_4k_pfn(
        vma: *mut vm_area_struct,
        addr: usize,
        pfn: usize,
        prot: pgprot_t,
    ) -> i32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
