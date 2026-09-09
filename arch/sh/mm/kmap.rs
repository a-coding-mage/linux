// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/sh/mm/kmap.c
 *
 * Copyright (C) 1999, 2000, 2002  Niibe Yutaka
 * Copyright (C) 2002 - 2009  Paul Mundt
 */
// C dependencies: linux/mm.h, linux/init.h, linux/mutex.h, linux/fs.h,
// linux/highmem.h, linux/module.h, asm/mmu_context.h, asm/cacheflush.h

static mut kmap_coherent_pte: *mut pte_t = core::ptr::null_mut();

pub unsafe fn kmap_coherent_init() {
    let mut vaddr: c_ulong;

    /* cache the first coherent kmap pte */
    vaddr = __fix_to_virt(FIX_CMAP_BEGIN);
    kmap_coherent_pte = virt_to_kpte(vaddr);
}

pub unsafe fn kmap_coherent(page: *mut page, addr: c_ulong) -> *mut core::ffi::c_void {
    let folio: *mut folio = page_folio(page);
    let idx: fixed_addresses;
    let mut vaddr: c_ulong;

    BUG_ON(!test_bit(PG_dcache_clean, &(*folio).flags.f));

    preempt_disable();
    pagefault_disable();

    idx = (FIX_CMAP_END as c_ulong
        - (((addr >> PAGE_SHIFT) & (FIX_N_COLOURS - 1))
            + (FIX_N_COLOURS * smp_processor_id() as c_ulong))) as fixed_addresses;

    vaddr = __fix_to_virt(idx);

    BUG_ON(pte_none(*kmap_coherent_pte.sub(idx as usize)));
    set_pte(
        kmap_coherent_pte.sub(idx as usize),
        mk_pte(page, PAGE_KERNEL),
    );

    vaddr as *mut core::ffi::c_void
}

pub unsafe fn kunmap_coherent(kvaddr: *mut core::ffi::c_void) {
    if (kvaddr as c_ulong) >= (FIXADDR_START as c_ulong) {
        let vaddr: c_ulong = kvaddr as c_ulong & PAGE_MASK;
        let idx: fixed_addresses = __virt_to_fix(vaddr);

        /* XXX.. Kill this later, here for sanity at the moment.. */
        __flush_purge_region(vaddr as *mut core::ffi::c_void, PAGE_SIZE);

        pte_clear(
            &mut init_mm,
            vaddr,
            kmap_coherent_pte.sub(idx as usize),
        );
        local_flush_tlb_one(get_asid(), vaddr);
    }

    pagefault_enable();
    preempt_enable();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
