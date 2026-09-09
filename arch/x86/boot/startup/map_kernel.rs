// SPDX-License-Identifier: GPL-2.0

extern "C" {
    static mut early_dynamic_pgts: [[pmd_t; PTRS_PER_PMD]; EARLY_DYNAMIC_PAGE_TABLES];
    static mut next_early_pgt: c_uint;
}

unsafe fn check_la57_support() -> bool {
    /*
     * 5-level paging is detected and enabled at kernel decompression
     * stage. Only check if it has been enabled there.
     */
    if (native_read_cr4() & X86_CR4_LA57) == 0 {
        return false;
    }

    __pgtable_l5_enabled = 1;
    pgdir_shift = 48;
    ptrs_per_p4d = 512;

    true
}

unsafe fn sme_postprocess_startup(
    bp: *mut boot_params,
    pmd: *mut pmdval_t,
    p2v_offset: c_ulong,
) -> c_ulong {
    let (mut paddr, mut paddr_end): (c_ulong, c_ulong);
    let mut i: c_int;

    /* Encrypt the kernel and related (if SME is active) */
    sme_encrypt_kernel(bp);

    /*
     * Clear the memory encryption mask from the .bss..decrypted section.
     * The bss section will be memset to zero later in the initialization so
     * there is no need to zero it after changing the memory encryption
     * attribute.
     */
    if sme_get_me_mask() != 0 {
        paddr = rip_rel_ptr(__start_bss_decrypted) as c_ulong;
        paddr_end = rip_rel_ptr(__end_bss_decrypted) as c_ulong;

        while paddr < paddr_end {
            /*
             * On SNP, transition the page to shared in the RMP table so that
             * it is consistent with the page table attribute change.
             *
             * __start_bss_decrypted has a virtual address in the high range
             * mapping (kernel .text). PVALIDATE, by way of
             * early_snp_set_memory_shared(), requires a valid virtual
             * address but the kernel is currently running off of the identity
             * mapping so use the PA to get a *currently* valid virtual address.
             */
            early_snp_set_memory_shared(paddr, paddr, PTRS_PER_PMD);

            i = pmd_index(paddr - p2v_offset);
            *pmd.add(i as usize) -= sme_get_me_mask();
            paddr += PMD_SIZE;
        }
    }

    /*
     * Return the SME encryption mask (if SME is active) to be used as a
     * modifier for the initial pgdir entry programmed into CR3.
     */
    sme_get_me_mask()
}

/*
 * This code is compiled using PIC codegen because it will execute from the
 * early 1:1 mapping of memory, which deviates from the mapping expected by
 * the linker. Due to this deviation, taking the address of a global variable
 * will produce an ambiguous result when using the plain & operator. Instead,
 * rip_rel_ptr() must be used, which will return the RIP-relative address in
 * the 1:1 mapping of memory. Kernel virtual addresses can be determined by
 * subtracting p2v_offset from the RIP-relative address.
 */
pub unsafe extern "C" fn __startup_64(
    p2v_offset: c_ulong,
    bp: *mut boot_params,
) -> c_ulong {
    let early_pgts = rip_rel_ptr(early_dynamic_pgts);
    let physaddr = rip_rel_ptr(_text) as c_ulong;
    let (mut va_text, mut va_end): (c_ulong, c_ulong);
    let mut pgtable_flags: c_ulong;
    let mut load_delta: c_ulong;
    let mut pgd: *mut pgdval_t;
    let mut p4d: *mut p4dval_t;
    let mut pud: *mut pudval_t;
    let mut pmd: *mut pmdval_t;
    let mut pmd_entry: pmdval_t;
    let la57: bool;
    let mut i: c_int;

    la57 = check_la57_support();

    /* Is the address too large? */
    if (physaddr >> MAX_PHYSMEM_BITS) != 0 {
        loop {}
    }

    /*
     * Compute the delta between the address I am compiled to run at
     * and the address I am actually running at.
     */
    phys_base = __START_KERNEL_map + p2v_offset;
    load_delta = phys_base;

    /* Is the address not 2M aligned? */
    if (load_delta & !PMD_MASK) != 0 {
        loop {}
    }

    va_text = physaddr - p2v_offset;
    va_end = rip_rel_ptr(_end) as c_ulong - p2v_offset;

    /* Include the SME encryption mask in the fixup value */
    load_delta += sme_get_me_mask();

    /* Fixup the physical addresses in the page table */
    pgd = rip_rel_ptr(early_top_pgt);
    *pgd.add(pgd_index(__START_KERNEL_map) as usize) += load_delta;

    if la57 {
        p4d = rip_rel_ptr(level4_kernel_pgt);
        *p4d.add((MAX_PTRS_PER_P4D - 1) as usize) += load_delta;
        *pgd.add(pgd_index(__START_KERNEL_map) as usize) = p4d as pgdval_t | _PAGE_TABLE;
    }

    level3_kernel_pgt[PTRS_PER_PUD - 2].pud += load_delta;
    level3_kernel_pgt[PTRS_PER_PUD - 1].pud += load_delta;

    i = FIXMAP_PMD_TOP;
    while i > FIXMAP_PMD_TOP - FIXMAP_PMD_NUM {
        level2_fixmap_pgt[i as usize].pmd += load_delta;
        i -= 1;
    }

    /* Set up the identity mapping for the switchover. */
    pud = &mut (*early_pgts.add(0)).pmd as *mut _;
    pmd = &mut (*early_pgts.add(1)).pmd as *mut _;
    next_early_pgt = 2;
    pgtable_flags = _KERNPG_TABLE_NOENC + sme_get_me_mask();

    if la57 {
        p4d = &mut (*early_pgts.add(next_early_pgt as usize)).pmd as *mut _;
        next_early_pgt += 1;
        i = ((physaddr >> PGDIR_SHIFT) % PTRS_PER_PGD) as c_int;
        *pgd.add((i + 0) as usize) = p4d as pgdval_t + pgtable_flags;
        *pgd.add((i + 1) as usize) = p4d as pgdval_t + pgtable_flags;
        i = (physaddr >> P4D_SHIFT) as c_int;
        *p4d.add(((i + 0) % PTRS_PER_P4D) as usize) = pud as pgdval_t + pgtable_flags;
        *p4d.add(((i + 1) % PTRS_PER_P4D) as usize) = pud as pgdval_t + pgtable_flags;
    } else {
        i = ((physaddr >> PGDIR_SHIFT) % PTRS_PER_PGD) as c_int;
        *pgd.add((i + 0) as usize) = pud as pgdval_t + pgtable_flags;
        *pgd.add((i + 1) as usize) = pud as pgdval_t + pgtable_flags;
    }

    i = (physaddr >> PUD_SHIFT) as c_int;
    *pud.add(((i + 0) % PTRS_PER_PUD) as usize) = pmd as pudval_t + pgtable_flags;
    *pud.add(((i + 1) % PTRS_PER_PUD) as usize) = pmd as pudval_t + pgtable_flags;

    pmd_entry = __PAGE_KERNEL_LARGE_EXEC & !_PAGE_GLOBAL;
    pmd_entry += sme_get_me_mask();
    pmd_entry += physaddr;

    i = 0;
    while i < DIV_ROUND_UP(va_end - va_text, PMD_SIZE) as c_int {
        let idx = i + (physaddr >> PMD_SHIFT) as c_int;
        *pmd.add((idx as usize) % PTRS_PER_PMD) = pmd_entry + i as c_ulong * PMD_SIZE;
        i += 1;
    }

    pmd = rip_rel_ptr(level2_kernel_pgt);
    i = 0;
    while i < pmd_index(va_text) {
        *pmd.add(i as usize) &= !_PAGE_PRESENT;
        i += 1;
    }
    while i <= pmd_index(va_end) {
        if *pmd.add(i as usize) & _PAGE_PRESENT != 0 {
            *pmd.add(i as usize) += load_delta;
        }
        i += 1;
    }
    while i < PTRS_PER_PMD as c_int {
        *pmd.add(i as usize) &= !_PAGE_PRESENT;
        i += 1;
    }

    sme_postprocess_startup(bp, pmd, p2v_offset)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
