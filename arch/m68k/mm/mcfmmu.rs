// SPDX-License-Identifier: GPL-2.0
/*
 * Based upon linux/arch/m68k/mm/sun3mmu.c
 * Based upon linux/arch/ppc/mm/mmu_context.c
 *
 * Implementations of mm routines specific to the Coldfire MMU.
 *
 * Copyright (c) 2008 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the surrounding kernel translation.

#[inline]
unsafe fn kmaparea(x: usize) -> bool {
    x >= VMALLOC_START && x < KMAP_END
}

static mut next_mmu_context: mm_context_t = 0;
static mut context_map: [c_ulong; LAST_CONTEXT / BITS_PER_LONG + 1] =
    [0; LAST_CONTEXT / BITS_PER_LONG + 1];
static mut nr_free_contexts: atomic_t = atomic_t { counter: 0 };
static mut context_mm: [*mut mm_struct; LAST_CONTEXT + 1] = [core::ptr::null_mut(); LAST_CONTEXT + 1];
static mut num_pages: c_ulong = 0;

/*
 * ColdFire paging_init derived from sun3.
 */
unsafe fn paging_init() {
    let mut pg_dir: *mut pgd_t;
    let mut pg_table: *mut pte_t;
    let mut address: c_ulong;
    let mut size: c_ulong;
    let mut next_pgtable: c_ulong;
    let mut i: c_int;

    pg_dir = swapper_pg_dir;
    core::ptr::write_bytes(swapper_pg_dir, 0, core::mem::size_of_val(&*swapper_pg_dir));

    size = num_pages * core::mem::size_of::<pte_t>() as c_ulong;
    size = (size + PAGE_SIZE) & !(PAGE_SIZE - 1);
    next_pgtable = memblock_alloc_or_panic(size, PAGE_SIZE) as c_ulong;

    pg_dir = pg_dir.add((PAGE_OFFSET >> PGDIR_SHIFT) as usize);

    address = PAGE_OFFSET;
    while address < high_memory as c_ulong {
        pg_table = next_pgtable as *mut pte_t;
        next_pgtable += (PTRS_PER_PTE * core::mem::size_of::<pte_t>()) as c_ulong;
        (*pg_dir).val = pg_table as c_ulong;
        pg_dir = pg_dir.add(1);

        /* now change pg_table to kernel virtual addresses */
        i = 0;
        while i < PTRS_PER_PTE {
            let mut pte: pte_t = pfn_pte(virt_to_pfn(address as *mut core::ffi::c_void), PAGE_INIT);
            if address >= high_memory as c_ulong {
                pte.val = 0;
            }
            set_pte(pg_table, pte);
            address += PAGE_SIZE;
            pg_table = pg_table.add(1);
            i += 1;
        }
    }

    (*current).mm = core::ptr::null_mut();
}

unsafe fn cf_tlb_miss(regs: *mut pt_regs, write: c_int, dtlb: c_int, extension_word: c_int) -> c_int {
    let mut flags: c_ulong = 0;
    let mut mmuar: c_ulong;
    let mut mmutr: c_ulong;
    let mm: *mut mm_struct;
    let pgd: *mut pgd_t;
    let p4d: *mut p4d_t;
    let pud: *mut pud_t;
    let pmd: *mut pmd_t;
    let mut pte: *mut pte_t = core::ptr::null_mut();
    let mut ret: c_int = -1;
    let asid: c_int;

    local_irq_save(&mut flags);

    mmuar = if dtlb != 0 { mmu_read(MMUAR) } else {
        (*regs).pc + (extension_word as c_ulong * core::mem::size_of::<c_ulong>() as c_ulong)
    };

    mm = if !user_mode(regs) && kmaparea(mmuar) { &mut init_mm } else { (*current).mm };
    if mm.is_null() { return cf_tlb_miss_out(flags, pte, mmuar, ret); }

    pgd = pgd_offset(mm, mmuar);
    if pgd_none(*pgd) { return cf_tlb_miss_out(flags, pte, mmuar, ret); }
    p4d = p4d_offset(pgd, mmuar);
    if p4d_none(*p4d) { return cf_tlb_miss_out(flags, pte, mmuar, ret); }
    pud = pud_offset(p4d, mmuar);
    if pud_none(*pud) { return cf_tlb_miss_out(flags, pte, mmuar, ret); }
    pmd = pmd_offset(pud, mmuar);
    if pmd_none(*pmd) { return cf_tlb_miss_out(flags, pte, mmuar, ret); }

    pte = if kmaparea(mmuar) { pte_offset_kernel(pmd, mmuar) } else { pte_offset_map(pmd, mmuar) };
    if pte.is_null() || pte_none(*pte) || !pte_present(*pte) { return cf_tlb_miss_out(flags, pte, mmuar, ret); }

    if write != 0 {
        if !pte_write(*pte) { return cf_tlb_miss_out(flags, pte, mmuar, ret); }
        set_pte(pte, pte_mkdirty(*pte));
    }
    set_pte(pte, pte_mkyoung(*pte));
    asid = ((*mm).context & 0xff) as c_int;
    if !pte_dirty(*pte) && !kmaparea(mmuar) { set_pte(pte, pte_wrprotect(*pte)); }

    mmutr = (mmuar & PAGE_MASK) | ((asid as c_ulong) << MMUTR_IDN) | MMUTR_V;
    if mmuar < TASK_UNMAPPED_BASE || mmuar >= TASK_SIZE { mmutr |= ((*pte).pte & CF_PAGE_MMUTR_MASK) >> CF_PAGE_MMUTR_SHIFT; }
    mmu_write(MMUTR, mmutr);
    mmu_write(MMUDR, (pte_val(*pte) & PAGE_MASK) | ((*pte).pte & CF_PAGE_MMUDR_MASK) | MMUDR_SZ_8KB | MMUDR_X);
    if dtlb != 0 { mmu_write(MMUOR, MMUOR_ACC | MMUOR_UAA); } else { mmu_write(MMUOR, MMUOR_ITLB | MMUOR_ACC | MMUOR_UAA); }
    ret = 0;
    cf_tlb_miss_out(flags, pte, mmuar, ret)
}

unsafe fn cf_tlb_miss_out(flags: c_ulong, pte: *mut pte_t, mmuar: c_ulong, ret: c_int) -> c_int {
    if !pte.is_null() && !kmaparea(mmuar) { pte_unmap(pte); }
    local_irq_restore(flags);
    ret
}

unsafe fn cf_bootmem_alloc() {
    let mut memstart: c_ulong;
    m68k_memory[0].addr = _rambase;
    m68k_memory[0].size = _ramend - _rambase;
    memblock_add_node(m68k_memory[0].addr, m68k_memory[0].size, 0, MEMBLOCK_NONE);
    num_pages = PFN_DOWN(_ramend - _rambase);
    memstart = PAGE_ALIGN(_ramstart);
    min_low_pfn = PFN_DOWN(_rambase);
    max_pfn = PFN_DOWN(_ramend);
    max_low_pfn = PFN_DOWN(_ramend);
    high_memory = _ramend as *mut core::ffi::c_void;
    memblock_reserve(_rambase, memstart - _rambase);
    m68k_virt_to_node_shift = fls(_ramend - 1) - 6;
    module_fixup(core::ptr::null_mut(), __start_fixup, __stop_fixup);
    m68k_setup_node(0);
}

unsafe fn cf_mmu_context_init() {
    context_map[0] = (1 << FIRST_CONTEXT) - 1;
    next_mmu_context = FIRST_CONTEXT;
    atomic_set(&mut nr_free_contexts, LAST_CONTEXT - FIRST_CONTEXT + 1);
}

unsafe fn steal_context() {
    if next_mmu_context < FIRST_CONTEXT { next_mmu_context = FIRST_CONTEXT; }
    let mm = context_mm[next_mmu_context as usize];
    flush_tlb_mm(mm);
    destroy_context(mm);
}

static protection_map: [pgprot_t; 16] = [
    PAGE_NONE, __pgprot(CF_PAGE_VALID | CF_PAGE_ACCESSED | CF_PAGE_READABLE),
    __pgprot(CF_PAGE_VALID | CF_PAGE_ACCESSED | CF_PAGE_WRITABLE),
    __pgprot(CF_PAGE_VALID | CF_PAGE_ACCESSED | CF_PAGE_READABLE | CF_PAGE_WRITABLE),
    __pgprot(CF_PAGE_VALID | CF_PAGE_ACCESSED | CF_PAGE_EXEC),
    __pgprot(CF_PAGE_VALID | CF_PAGE_ACCESSED | CF_PAGE_READABLE | CF_PAGE_EXEC),
    __pgprot(CF_PAGE_VALID | CF_PAGE_ACCESSED | CF_PAGE_WRITABLE | CF_PAGE_EXEC),
    __pgprot(CF_PAGE_VALID | CF_PAGE_ACCESSED | CF_PAGE_READABLE | CF_PAGE_WRITABLE | CF_PAGE_EXEC),
    PAGE_NONE, __pgprot(CF_PAGE_VALID | CF_PAGE_ACCESSED | CF_PAGE_READABLE), PAGE_SHARED,
    __pgprot(CF_PAGE_VALID | CF_PAGE_ACCESSED | CF_PAGE_READABLE | CF_PAGE_SHARED),
    __pgprot(CF_PAGE_VALID | CF_PAGE_ACCESSED | CF_PAGE_EXEC),
    __pgprot(CF_PAGE_VALID | CF_PAGE_ACCESSED | CF_PAGE_READABLE | CF_PAGE_EXEC),
    __pgprot(CF_PAGE_VALID | CF_PAGE_ACCESSED | CF_PAGE_SHARED | CF_PAGE_EXEC),
    __pgprot(CF_PAGE_VALID | CF_PAGE_ACCESSED | CF_PAGE_READABLE | CF_PAGE_SHARED | CF_PAGE_EXEC),
];

// DECLARE_VM_GET_PAGE_PROT

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
