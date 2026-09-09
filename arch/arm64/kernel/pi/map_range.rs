// SPDX-License-Identifier: GPL-2.0-only
// Copyright 2023 Google LLC
// Author: Ard Biesheuvel <ardb@google.com>

/**
 * map_range - Map a contiguous range of physical pages into virtual memory
 *
 * @pte:         Address of physical pointer to array of pages to
 *              allocate page tables from
 * @start:       Virtual address of the start of the range
 * @end:         Virtual address of the end of the range (exclusive)
 * @pa:          Physical address of the start of the range
 * @prot:        Access permissions of the range
 * @level:       Translation level for the mapping
 * @tbl:         The level @level page table to create the mappings in
 * @may_use_cont: Whether the use of the contiguous attribute is allowed
 * @va_offset:   Offset between a physical page and its current mapping
 *              in the VA space
 */
pub unsafe fn map_range(
    pte: *mut phys_addr_t,
    mut start: u64,
    end: u64,
    mut pa: phys_addr_t,
    prot: pgprot_t,
    level: i32,
    mut tbl: *mut pte_t,
    may_use_cont: bool,
    va_offset: u64,
) {
    let cmask: u64 = if level == 3 { CONT_PTE_SIZE - 1 } else { U64_MAX };
    let mut protval: ptval_t = pgprot_val(prot) & !PTE_TYPE_MASK;
    let lshift: i32 = (3 - level) * PTDESC_TABLE_SHIFT;
    let lmask: u64 = (PAGE_SIZE << lshift) - 1;

    start &= PAGE_MASK;
    pa &= PAGE_MASK;

    // Advance tbl to the entry that covers start
    tbl = tbl.add(((start >> (lshift + PAGE_SHIFT)) % PTRS_PER_PTE) as usize);

    // Set the right block/page bits for this level unless we are clearing the mapping
    if protval != 0 {
        protval |= if level == 2 { PMD_TYPE_SECT } else { PTE_TYPE_PAGE };
    }

    while start < end {
        let next = core::cmp::min((start | lmask) + 1, PAGE_ALIGN(end));

        if level < 2 || (level == 2 && ((start | next | pa) & lmask) != 0) {
            // This chunk needs a finer grained mapping. Create a table mapping if necessary and recurse.
            if pte_none(*tbl) {
                *tbl = __pte(__phys_to_pte_val(*pte) | PMD_TYPE_TABLE | PMD_TABLE_UXN);
                *pte += (PTRS_PER_PTE * core::mem::size_of::<pte_t>()) as phys_addr_t;
            }
            map_range(
                pte,
                start,
                next,
                pa,
                prot,
                level + 1,
                (__pte_to_phys(*tbl) + va_offset) as *mut pte_t,
                may_use_cont,
                va_offset,
            );
        } else {
            // Start a contiguous range if start and pa are suitably aligned
            if ((start | pa) & cmask) == 0 && may_use_cont {
                protval |= PTE_CONT;
            }

            // Clear the contiguous attribute if the remaining range does not cover a contiguous block
            if (end & !cmask) <= start {
                protval &= !PTE_CONT;
            }

            // Put down a block or page mapping
            *tbl = __pte(__phys_to_pte_val(pa) | protval);
        }
        pa += next - start;
        start = next;
        tbl = tbl.add(1);
    }
}

pub unsafe fn create_init_idmap(pg_dir: *mut pgd_t, clrmask: ptval_t) -> phys_addr_t {
    let mut ptep = pg_dir as phys_addr_t + PAGE_SIZE; // MMU is off
    let mut text_prot: pgprot_t = PAGE_KERNEL_ROX;
    let mut data_prot: pgprot_t = PAGE_KERNEL;

    text_prot = __pgprot(pgprot_val(text_prot) & !clrmask);
    data_prot = __pgprot(pgprot_val(data_prot) & !clrmask);

    // MMU is off; pointer casts to phys_addr_t are safe
    map_range(
        &mut ptep,
        _stext as u64,
        __initdata_begin as u64,
        _stext as phys_addr_t,
        text_prot,
        IDMAP_ROOT_LEVEL,
        pg_dir as *mut pte_t,
        false,
        0,
    );
    map_range(
        &mut ptep,
        __initdata_begin as u64,
        _end as u64,
        __initdata_begin as phys_addr_t,
        data_prot,
        IDMAP_ROOT_LEVEL,
        pg_dir as *mut pte_t,
        false,
        0,
    );

    ptep
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
