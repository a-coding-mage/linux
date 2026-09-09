// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2008,2009 Ben Herrenschmidt <benh@kernel.crashing.org>
 *                     IBM Corp.
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

// C headers omitted; symbols supplied by the surrounding kernel are external dependencies.

/* The variables below are currently only used on 64-bit Book3E
 * though this will probably be made common with other nohash
 * implementations at some point
 */
static mut mmu_pte_psize: i32 = 0; /* Page size used for PTE pages */
static mut mmu_vmemmap_psize: i32 = 0; /* Page size used for the virtual mem map */
static mut book3e_htw_mode: i32 = 0; /* HW tablewalk?  Value is PPC_HTW_* */
static mut linear_map_top: u64 = 0; /* Top of linear mapping */

/* Number of bytes to add to SPRN_SPRG_TLB_EXFRAME on crit/mcheck/debug
 * exceptions.  This is used for bolted and e6500 TLB miss handlers which
 * do not modify this SPRG in the TLB miss code; for other TLB miss handlers,
 * this is set to zero.
 */
static mut extlb_level_exc: i32 = 0;

/* Handling of virtual linear page tables or indirect TLB entries
 * flushing when PTE pages are freed
 */
pub unsafe fn tlb_flush_pgtable(tlb: *mut mmu_gather, address: u64) {
    let tsize = mmu_psize_defs[mmu_pte_psize as usize].shift - 10;

    if book3e_htw_mode != PPC_HTW_NONE {
        let mut start = address & PMD_MASK;
        let end = address.wrapping_add(PMD_SIZE);
        let size = 1u64 << mmu_psize_defs[mmu_pte_psize as usize].shift;

        /* This isn't the most optimal, ideally we would factor out the
         * while preempt & CPU mask mucking around, or even the IPI but
         * it will do for now
         */
        while start < end {
            __flush_tlb_page((*tlb).mm, start, tsize, 1);
            start = start.wrapping_add(size);
        }
    } else {
        let rmask = 0xf000000000000000u64;
        let rid = (address & rmask) | 0x1000000000000000u64;
        let mut vpte = address & !rmask;

        vpte = (vpte >> (PAGE_SHIFT - 3)) & !0xfffu64;
        vpte |= rid;
        __flush_tlb_page((*tlb).mm, vpte, tsize, 0);
    }
}

unsafe fn setup_page_sizes() {
    let mut tlb0cfg: u32;
    let mut eptcfg: u32;
    let mut psize: i32;

    let mmucfg = mfspr(SPRN_MMUCFG);

    if (mmucfg & MMUCFG_MAVN) == MMUCFG_MAVN_V1 {
        let tlb1cfg = mfspr(SPRN_TLB1CFG);
        let min_pg = (tlb1cfg & TLBnCFG_MINSIZE) >> TLBnCFG_MINSIZE_SHIFT;
        let max_pg = (tlb1cfg & TLBnCFG_MAXSIZE) >> TLBnCFG_MAXSIZE_SHIFT;

        psize = 0;
        while psize < MMU_PAGE_COUNT {
            let def = &mut mmu_psize_defs[psize as usize];
            let mut shift = def.shift;

            if shift == 0 || (shift & 1) != 0 {
                psize += 1;
                continue;
            }
            /* adjust to be in terms of 4^shift Kb */
            shift = (shift - 10) >> 1;
            if shift >= min_pg && shift <= max_pg {
                def.flags |= MMU_PAGE_SIZE_DIRECT;
            }
            psize += 1;
        }
    } else if (mmucfg & MMUCFG_MAVN) == MMUCFG_MAVN_V2 {
        let tlb1cfg: u32;
        let tlb1ps: u32;
        tlb0cfg = mfspr(SPRN_TLB0CFG);
        tlb1cfg = mfspr(SPRN_TLB1CFG);
        tlb1ps = mfspr(SPRN_TLB1PS);
        eptcfg = mfspr(SPRN_EPTCFG);

        if (tlb1cfg & TLBnCFG_IND) != 0 && (tlb0cfg & TLBnCFG_PT) != 0 {
            book3e_htw_mode = PPC_HTW_E6500;
        }
        /* We expect 4K subpage size and unrestricted indirect size. */
        if eptcfg != 2 {
            book3e_htw_mode = PPC_HTW_NONE;
        }
        psize = 0;
        while psize < MMU_PAGE_COUNT {
            let def = &mut mmu_psize_defs[psize as usize];
            if def.shift != 0 {
                if (tlb1ps & (1u32 << (def.shift - 10))) != 0 {
                    def.flags |= MMU_PAGE_SIZE_DIRECT;
                    if book3e_htw_mode != 0 && psize == MMU_PAGE_2M {
                        def.flags |= MMU_PAGE_SIZE_INDIRECT;
                    }
                }
            }
            psize += 1;
        }
    }

    /* Cleanup array and print summary */
    pr_info!("MMU: Supported page sizes\n");
    psize = 0;
    while psize < MMU_PAGE_COUNT {
        let def = &mut mmu_psize_defs[psize as usize];
        let page_type_names = ["unsupported", "direct", "indirect", "direct & indirect"];
        if def.flags == 0 {
            def.shift = 0;
            psize += 1;
            continue;
        }
        pr_info!("  {:8} KB as {}\n", 1u64 << (def.shift - 10), page_type_names[(def.flags & 0x3) as usize]);
        psize += 1;
    }
}

/* Early initialization of the MMU TLB code */
unsafe fn early_init_this_mmu() {
    let mut mas4 = 0x4 << MAS4_WIMGED_SHIFT;
    match book3e_htw_mode {
        PPC_HTW_E6500 => {
            mas4 |= MAS4_INDD;
            mas4 |= BOOK3E_PAGESZ_2M << MAS4_TSIZED_SHIFT;
            mas4 |= MAS4_TLBSELD(1);
            mmu_pte_psize = MMU_PAGE_2M;
        }
        PPC_HTW_NONE => {
            mas4 |= BOOK3E_PAGESZ_4K << MAS4_TSIZED_SHIFT;
            mmu_pte_psize = mmu_virtual_psize;
        }
        _ => {}
    }
    mtspr(SPRN_MAS4, mas4);

    let num_cams = (mfspr(SPRN_TLB1CFG) & TLBnCFG_N_ENTRY) / 4;
    let mut map = true;
    /* use a quarter of the TLBCAM for bolted linear map */
    // CONFIG_SMP conditional is preserved by the runtime-independent source structure.
    #[cfg(CONFIG_SMP)]
    if hweight32(get_tensr()) > 1 { map = false; }
    if map {
        linear_map_top = map_mem_in_cams(linear_map_top, num_cams, false, true);
    }
    /* A sync won't hurt us after mucking around with the MMU configuration */
    mb();
}

unsafe fn early_init_mmu_global() {
    mmu_vmemmap_psize = MMU_PAGE_4K;
    /* Look for supported page sizes */
    setup_page_sizes();
    extlb_level_exc = EX_TLB_SIZE;
    match book3e_htw_mode {
        PPC_HTW_E6500 => {
            patch_exception(0x1c0, exc_data_tlb_miss_e6500_book3e);
            patch_exception(0x1e0, exc_instruction_tlb_miss_e6500_book3e);
        }
        _ => {}
    }
    pr_info!("MMU: Book3E HW tablewalk {}\n", if book3e_htw_mode != PPC_HTW_NONE { "enabled" } else { "not supported" });
    linear_map_top = memblock_end_of_DRAM();
    ioremap_bot = IOREMAP_BASE;
}

unsafe fn early_mmu_set_memory_limit() {
    memblock_enforce_memory_limit(linear_map_top);
    memblock_set_current_limit(linear_map_top);
}

pub unsafe fn early_init_mmu() {
    early_init_mmu_global();
    early_init_this_mmu();
    early_mmu_set_memory_limit();
}

pub unsafe fn early_init_mmu_secondary() {
    early_init_this_mmu();
}

pub unsafe fn setup_initial_memory_limit(first_memblock_base: u64, first_memblock_size: u64) {
    /* On FSL Embedded 64-bit, usually all RAM is bolted, but unusual memory sizes can leave RAM unmapped. */
    let num_cams = (mfspr(SPRN_TLB1CFG) & TLBnCFG_N_ENTRY) / 4;
    let linear_sz = map_mem_in_cams(first_memblock_size, num_cams, true, true);
    ppc64_rma_size = core::cmp::min(linear_sz, 0x40000000u64);
    memblock_set_current_limit(first_memblock_base.wrapping_add(ppc64_rma_size));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
