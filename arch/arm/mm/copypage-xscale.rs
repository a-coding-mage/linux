// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/lib/copypage-xscale.S
 *
 *  Copyright (C) 1995-2005 Russell King
 *
 * This handles the mini data cache, as found on SA11x0 and XScale
 * processors.  When we copy a user page page, we map it in such a way
 * that accesses to this page will not touch the main data cache, but
 * will be cached in the mini data cache.  This prevents us thrashing
 * the main data cache on page faults.
 */

// Dependencies supplied by the surrounding kernel translation unit.

const MINICACHE_PGPROT: usize = __pgprot(L_PTE_PRESENT | L_PTE_YOUNG | L_PTE_MT_MINICACHE);

static mut MINICACHE_LOCK: raw_spinlock_t = DEFINE_RAW_SPINLOCK();

/*
 * XScale mini-dcache optimised copy_user_highpage
 *
 * We flush the destination cache lines just before we write the data into the
 * corresponding address.  Since the Dcache is read-allocate, this removes the
 * Dcache aliasing issue.  The writes will be forwarded to the write buffer,
 * and merged as appropriate.
 */
unsafe fn mc_copy_user_page(mut from: *mut core::ffi::c_void, mut to: *mut core::ffi::c_void) {
    let mut tmp: i32;

    /*
     * Strangely enough, best performance is achieved
     * when prefetching destination as well.  (NP)
     */
    core::arch::asm!(
        ".arch xscale",
        "pld [{from}, #0]",
        "pld [{from}, #32]",
        "pld [{to}, #0]",
        "pld [{to}, #32]",
        "1: pld [{from}, #64]",
        "pld [{from}, #96]",
        "pld [{to}, #64]",
        "pld [{to}, #96]",
        "2: ldrd r2, r3, [{from}], #8",
        "ldrd r4, r5, [{from}], #8",
        "mov ip, {to}",
        "strd r2, r3, [{to}], #8",
        "ldrd r2, r3, [{from}], #8",
        "strd r4, r5, [{to}], #8",
        "ldrd r4, r5, [{from}], #8",
        "strd r2, r3, [{to}], #8",
        "strd r4, r5, [{to}], #8",
        "mcr p15, 0, ip, c7, c10, 1",
        "ldrd r2, r3, [{from}], #8",
        "mcr p15, 0, ip, c7, c6, 1",
        "ldrd r4, r5, [{from}], #8",
        "mov ip, {to}",
        "strd r2, r3, [{to}], #8",
        "ldrd r2, r3, [{from}], #8",
        "strd r4, r5, [{to}], #8",
        "ldrd r4, r5, [{from}], #8",
        "strd r2, r3, [{to}], #8",
        "strd r4, r5, [{to}], #8",
        "mcr p15, 0, ip, c7, c10, 1",
        "subs {tmp}, {tmp}, #1",
        "mcr p15, 0, ip, c7, c6, 1",
        "bgt 1b",
        "beq 2b",
        from = inout(reg) from,
        to = inout(reg) to,
        tmp = out(reg) tmp,
        in("r2") 0i32,
        options(nostack)
    );
}

unsafe fn xscale_mc_copy_user_highpage(
    to: *mut page,
    from: *mut page,
    _vaddr: c_ulong,
    _vma: *mut vm_area_struct,
) {
    let src: *mut folio = page_folio(from);
    let kto: *mut core::ffi::c_void = kmap_atomic(to);

    if !test_and_set_bit(PG_dcache_clean, &mut (*src).flags.f) {
        __flush_dcache_folio(folio_flush_mapping(src), src);
    }

    raw_spin_lock(&mut MINICACHE_LOCK);
    set_top_pte(COPYPAGE_MINICACHE, mk_pte(from, MINICACHE_PGPROT));
    mc_copy_user_page(COPYPAGE_MINICACHE as *mut core::ffi::c_void, kto);
    raw_spin_unlock(&mut MINICACHE_LOCK);
    kunmap_atomic(kto);
}

/*
 * XScale optimised clear_user_page
 */
unsafe fn xscale_mc_clear_user_highpage(page: *mut page, _vaddr: c_ulong) {
    let mut ptr: *mut core::ffi::c_void;
    let kaddr: *mut core::ffi::c_void = kmap_atomic(page);
    core::arch::asm!(
        ".arch xscale",
        "mov r1, {count}",
        "mov r2, #0",
        "mov r3, #0",
        "1: mov ip, {ptr}",
        "strd r2, r3, [{ptr}], #8",
        "strd r2, r3, [{ptr}], #8",
        "strd r2, r3, [{ptr}], #8",
        "strd r2, r3, [{ptr}], #8",
        "mcr p15, 0, ip, c7, c10, 1",
        "subs r1, r1, #1",
        "mcr p15, 0, ip, c7, c6, 1",
        "bne 1b",
        ptr = inout(reg) kaddr => ptr,
        count = const PAGE_SIZE / 32,
        options(nostack)
    );
    kunmap_atomic(kaddr);
}

static mut xscale_mc_user_fns: cpu_user_fns = cpu_user_fns {
    cpu_clear_user_highpage: Some(xscale_mc_clear_user_highpage),
    cpu_copy_user_highpage: Some(xscale_mc_copy_user_highpage),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
