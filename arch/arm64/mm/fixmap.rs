// SPDX-License-Identifier: GPL-2.0-only
/*
 * Fixmap manipulation code
 */

// Dependencies supplied by the surrounding kernel translation.

/* ensure that the fixmap region does not grow down into the PCI I/O region */
const _: () = assert!(FIXADDR_TOT_START > PCI_IO_END);

const NR_BM_PTE_TABLES: usize = SPAN_NR_ENTRIES(FIXADDR_TOT_START, FIXADDR_TOP, PMD_SHIFT);
const NR_BM_PMD_TABLES: usize = SPAN_NR_ENTRIES(FIXADDR_TOT_START, FIXADDR_TOP, PUD_SHIFT);

const _: () = assert!(NR_BM_PMD_TABLES == 1);

#[inline]
const fn bm_table_idx(addr: usize, shift: usize) -> usize {
    (addr >> shift) - (FIXADDR_TOT_START >> shift)
}

#[inline]
const fn bm_pte_table_idx(addr: usize) -> usize {
    bm_table_idx(addr, PMD_SHIFT)
}

static mut BM_PTE: [[pte_t; PTRS_PER_PTE]; NR_BM_PTE_TABLES] =
    [[pte_t::default(); PTRS_PER_PTE]; NR_BM_PTE_TABLES];
static mut BM_PMD: [pmd_t; PTRS_PER_PMD] = [pmd_t::default(); PTRS_PER_PMD];
static mut BM_PUD: [pud_t; PTRS_PER_PUD] = [pud_t::default(); PTRS_PER_PUD];

#[inline]
unsafe fn fixmap_pte(addr: usize) -> *mut pte_t {
    &mut BM_PTE[bm_pte_table_idx(addr)][pte_index(addr)]
}

unsafe fn early_fixmap_init_pte(pmdp: *mut pmd_t, addr: usize) {
    let pmd = core::ptr::read_volatile(pmdp);
    let ptep: *mut pte_t;

    if pmd_none(pmd) {
        ptep = BM_PTE[bm_pte_table_idx(addr)].as_mut_ptr();
        __pmd_populate(pmdp, __pa_symbol(ptep), PMD_TYPE_TABLE | PMD_TABLE_AF);
    }
}

unsafe fn early_fixmap_init_pmd(pudp: *mut pud_t, mut addr: usize, end: usize) {
    let mut next: usize;
    let pud = core::ptr::read_volatile(pudp);
    let pmdp: *mut pmd_t;

    if pud_none(pud) {
        __pud_populate(pudp, __pa_symbol(BM_PMD.as_mut_ptr()),
                       PUD_TYPE_TABLE | PUD_TABLE_AF);
    }

    pmdp = pmd_offset_kimg(pudp, addr);
    loop {
        next = pmd_addr_end(addr, end);
        early_fixmap_init_pte(pmdp, addr);
        addr = next;
        if addr == end {
            break;
        }
        // C expression: pmdp++
        pmdp = pmdp.add(1);
    }
}

unsafe fn early_fixmap_init_pud(p4dp: *mut p4d_t, addr: usize, end: usize) {
    let p4d = core::ptr::read_volatile(p4dp);
    let pudp: *mut pud_t;

    if CONFIG_PGTABLE_LEVELS > 3 && !p4d_none(p4d)
        && p4d_page_paddr(p4d) != __pa_symbol(BM_PUD.as_mut_ptr())
    {
        /*
         * We only end up here if the kernel mapping and the fixmap
         * share the top level pgd entry, which should only happen on
         * 16k/4 levels configurations.
         */
        BUG_ON(!IS_ENABLED(CONFIG_ARM64_16K_PAGES));
    }

    if p4d_none(p4d) {
        __p4d_populate(p4dp, __pa_symbol(BM_PUD.as_mut_ptr()),
                       P4D_TYPE_TABLE | P4D_TABLE_AF);
    }

    pudp = pud_offset_kimg(p4dp, addr);
    early_fixmap_init_pmd(pudp, addr, end);
}

/*
 * The p*d_populate functions call virt_to_phys implicitly so they can't be used
 * directly on kernel symbols (bm_p*d). This function is called too early to use
 * lm_alias so __p*d_populate functions must be used to populate with the
 * physical address from __pa_symbol.
 */
pub unsafe fn early_fixmap_init() {
    let addr: usize = FIXADDR_TOT_START;
    let end: usize = FIXADDR_TOP;

    let pgdp: *mut pgd_t = pgd_offset_k(addr);
    let p4dp: *mut p4d_t = p4d_offset_kimg(pgdp, addr);

    early_fixmap_init_pud(p4dp, addr, end);
}

/*
 * Unusually, this is also called in IRQ context (ghes_iounmap_irq) so if we
 * ever need to use IPIs for TLB broadcasting, then we're in trouble here.
 */
pub unsafe fn __set_fixmap(idx: fixed_addresses, phys: phys_addr_t, flags: pgprot_t) {
    let addr: usize = __fix_to_virt(idx);
    let ptep: *mut pte_t;

    BUG_ON(idx <= FIX_HOLE || idx >= __end_of_fixed_addresses);

    ptep = fixmap_pte(addr);

    if pgprot_val(flags) != 0 {
        __set_pte(ptep, pfn_pte(phys >> PAGE_SHIFT, flags));
    } else {
        __pte_clear(&raw mut init_mm, addr, ptep);
        flush_tlb_kernel_range(addr, addr.wrapping_add(PAGE_SIZE));
    }
}

pub unsafe fn fixmap_remap_fdt(dt_phys: phys_addr_t, size: *mut i32, prot: pgprot_t) -> *mut core::ffi::c_void {
    let dt_virt_base: u64 = __fix_to_virt(FIX_FDT);
    let dt_phys_base: phys_addr_t;
    let offset: usize;
    let dt_virt: *mut core::ffi::c_void;

    /*
     * Check whether the physical FDT address is set and meets the minimum
     * alignment requirement. Since we are relying on MIN_FDT_ALIGN to be
     * at least 8 bytes so that we can always access the magic and size
     * fields of the FDT header after mapping the first chunk, double check
     * here if that is indeed the case.
     */
    const _: () = assert!(MIN_FDT_ALIGN >= 8);
    if dt_phys == 0 || dt_phys % MIN_FDT_ALIGN != 0 {
        return core::ptr::null_mut();
    }

    dt_phys_base = round_down(dt_phys, PAGE_SIZE);
    offset = dt_phys % PAGE_SIZE;
    dt_virt = (dt_virt_base as *mut u8).add(offset) as *mut core::ffi::c_void;

    /* map the first chunk so we can read the size from the header */
    create_mapping_noalloc(dt_phys_base, dt_virt_base, PAGE_SIZE, prot);

    if fdt_magic(dt_virt) != FDT_MAGIC {
        return core::ptr::null_mut();
    }

    *size = fdt_totalsize(dt_virt);
    if *size > MAX_FDT_SIZE {
        return core::ptr::null_mut();
    }

    if offset + (*size as usize) > PAGE_SIZE {
        create_mapping_noalloc(dt_phys_base, dt_virt_base,
                               offset + (*size as usize), prot);
    }

    dt_virt
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
