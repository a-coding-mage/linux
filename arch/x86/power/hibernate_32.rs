// SPDX-License-Identifier: GPL-2.0-only
/*
 * Hibernation support specific for i386 - temporary page tables
 *
 * Copyright (c) 2006 Rafael J. Wysocki <rjw@sisk.pl>
 */

// Linux kernel dependencies supplied by other translation units.

/* Pointer to the temporary resume page tables */
pub static mut resume_pg_dir: *mut pgd_t = core::ptr::null_mut();

/* The following three functions are based on the analogous code in
 * arch/x86/mm/init_32.c
 */

/*
 * Create a middle page table on a resume-safe page and put a pointer to it in
 * the given global directory entry.  This only returns the gd entry
 * in non-PAE compilation mode, since the middle layer is folded.
 */
unsafe fn resume_one_md_table_init(pgd: *mut pgd_t) -> *mut pmd_t {
    let p4d: *mut p4d_t;
    let pud: *mut pud_t;
    let pmd_table: *mut pmd_t;

    #[cfg(feature = "CONFIG_X86_PAE")]
    {
        pmd_table = get_safe_page(GFP_ATOMIC as _).cast::<pmd_t>();
        if pmd_table.is_null() {
            return core::ptr::null_mut();
        }

        set_pgd(pgd, __pgd(__pa(pmd_table) | _PAGE_PRESENT));
        p4d = p4d_offset(pgd, 0);
        pud = pud_offset(p4d, 0);

        BUG_ON(pmd_table != pmd_offset(pud, 0));
    }

    #[cfg(not(feature = "CONFIG_X86_PAE"))]
    {
        p4d = p4d_offset(pgd, 0);
        pud = pud_offset(p4d, 0);
        pmd_table = pmd_offset(pud, 0);
    }

    pmd_table
}

/*
 * Create a page table on a resume-safe page and place a pointer to it in
 * a middle page directory entry.
 */
unsafe fn resume_one_page_table_init(pmd: *mut pmd_t) -> *mut pte_t {
    if pmd_none(*pmd) {
        let page_table = get_safe_page(GFP_ATOMIC as _).cast::<pte_t>();
        if page_table.is_null() {
            return core::ptr::null_mut();
        }

        set_pmd(pmd, __pmd(__pa(page_table) | _PAGE_TABLE));
        BUG_ON(page_table != pte_offset_kernel(pmd, 0));
        return page_table;
    }

    pte_offset_kernel(pmd, 0)
}

/*
 * This maps the physical memory to kernel virtual address space, a total
 * of max_low_pfn pages, by creating page tables starting from address
 * PAGE_OFFSET.  The page tables are allocated out of resume-safe pages.
 */
unsafe fn resume_physical_mapping_init(pgd_base: *mut pgd_t) -> i32 {
    let mut pfn: c_ulong = 0;
    let mut pgd = pgd_base.add(pgd_index(PAGE_OFFSET) as usize);
    let mut pgd_idx = pgd_index(PAGE_OFFSET);
    let mut pmd: *mut pmd_t;
    let mut pte: *mut pte_t;
    let mut pmd_idx: i32;

    while pgd_idx < PTRS_PER_PGD {
        pmd = resume_one_md_table_init(pgd);
        if pmd.is_null() {
            return -ENOMEM;
        }

        if pfn >= max_low_pfn {
            pgd = pgd.add(1);
            pgd_idx += 1;
            continue;
        }

        pmd_idx = 0;
        while pmd_idx < PTRS_PER_PMD {
            if pfn >= max_low_pfn {
                break;
            }

            /* Map with big pages if possible, otherwise create
             * normal page tables.
             * NOTE: We can mark everything as executable here
             */
            if boot_cpu_has(X86_FEATURE_PSE) {
                set_pmd(pmd, pfn_pmd(pfn, PAGE_KERNEL_LARGE_EXEC));
                pfn += PTRS_PER_PTE;
            } else {
                pte = resume_one_page_table_init(pmd);
                if pte.is_null() {
                    return -ENOMEM;
                }

                let max_pte = pte.add(PTRS_PER_PTE as usize);
                while pte < max_pte {
                    if pfn >= max_low_pfn {
                        break;
                    }

                    set_pte(pte, pfn_pte(pfn, PAGE_KERNEL_EXEC));
                    pte = pte.add(1);
                    pfn += 1;
                }
            }

            pmd = pmd.add(1);
            pmd_idx += 1;
        }

        pgd = pgd.add(1);
        pgd_idx += 1;
    }

    0
}

unsafe fn resume_init_first_level_page_table(pg_dir: *mut pgd_t) {
    #[cfg(feature = "CONFIG_X86_PAE")]
    {
        /* Init entries of the first-level page table to the zero page */
        let mut i = 0;
        while i < PTRS_PER_PGD {
            set_pgd(
                pg_dir.add(i as usize),
                __pgd(__pa(empty_zero_page) | _PAGE_PRESENT),
            );
            i += 1;
        }
    }
}

unsafe fn set_up_temporary_text_mapping(pgd_base: *mut pgd_t) -> i32 {
    let pgd = pgd_base.add(pgd_index(restore_jump_address) as usize);
    let pmd = resume_one_md_table_init(pgd);
    if pmd.is_null() {
        return -ENOMEM;
    }

    if boot_cpu_has(X86_FEATURE_PSE) {
        set_pmd(
            pmd.add(pmd_index(restore_jump_address) as usize),
            __pmd((jump_address_phys & PMD_MASK) | pgprot_val(PAGE_KERNEL_LARGE_EXEC)),
        );
    } else {
        let pte = resume_one_page_table_init(pmd);
        if pte.is_null() {
            return -ENOMEM;
        }
        set_pte(
            pte.add(pte_index(restore_jump_address) as usize),
            __pte((jump_address_phys & PAGE_MASK) | pgprot_val(PAGE_KERNEL_EXEC)),
        );
    }

    0
}

pub unsafe extern "C" fn swsusp_arch_resume() -> i32 {
    let mut error: i32;

    resume_pg_dir = get_safe_page(GFP_ATOMIC as _).cast::<pgd_t>();
    if resume_pg_dir.is_null() {
        return -ENOMEM;
    }

    resume_init_first_level_page_table(resume_pg_dir);

    error = set_up_temporary_text_mapping(resume_pg_dir);
    if error != 0 {
        return error;
    }

    error = resume_physical_mapping_init(resume_pg_dir);
    if error != 0 {
        return error;
    }

    temp_pgt = __pa(resume_pg_dir);

    error = relocate_restore_code();
    if error != 0 {
        return error;
    }

    /* We have got enough memory and from now on we cannot recover */
    restore_image();
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
