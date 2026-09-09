// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mm/copypage-v6.c
 *
 *  Copyright (C) 2002 Deep Blue Solutions Ltd, All Rights Reserved.
 */

// Dependencies supplied by the surrounding kernel translation unit.

#[cfg(any())]
compile_error!("FIX ME"); // C condition: SHMLBA > 16384

static mut V6_LOCK: raw_spinlock_t = DEFINE_RAW_SPINLOCK!();

/*
 * Copy the user page.  No aliasing to deal with so we can just
 * attack the kernel's existing mapping of these pages.
 */
unsafe fn v6_copy_user_highpage_nonaliasing(
    to: *mut page,
    from: *mut page,
    _vaddr: c_ulong,
    _vma: *mut vm_area_struct,
) {
    let kfrom: *mut c_void = kmap_atomic(from);
    let kto: *mut c_void = kmap_atomic(to);
    copy_page(kto, kfrom);
    kunmap_atomic(kto);
    kunmap_atomic(kfrom);
}

/*
 * Clear the user page.  No aliasing to deal with so we can just
 * attack the kernel's existing mapping of this page.
 */
unsafe fn v6_clear_user_highpage_nonaliasing(page: *mut page, _vaddr: c_ulong) {
    let kaddr: *mut c_void = kmap_atomic(page);
    clear_page(kaddr);
    kunmap_atomic(kaddr);
}

/*
 * Discard data in the kernel mapping for the new page.
 * FIXME: needs this MCRR to be supported.
 */
unsafe fn discard_old_kernel_data(kto: *mut c_void) {
    core::arch::asm!(
        "mcrr p15, 0, {lo}, {hi}, c6",
        lo = in(reg) kto,
        hi = in(reg) ((kto as c_ulong).wrapping_add(PAGE_SIZE).wrapping_sub(1)),
        options(nostack)
    );
}

/*
 * Copy the page, taking account of the cache colour.
 */
unsafe fn v6_copy_user_highpage_aliasing(
    to: *mut page,
    from: *mut page,
    vaddr: c_ulong,
    _vma: *mut vm_area_struct,
) {
    let src: *mut folio = page_folio(from);
    let offset: c_uint = CACHE_COLOUR(vaddr);
    let mut kfrom: c_ulong;
    let mut kto: c_ulong;

    if !test_and_set_bit(PG_dcache_clean, &mut (*src).flags.f) {
        __flush_dcache_folio(folio_flush_mapping(src), src);
    }

    /* FIXME: not highmem safe */
    discard_old_kernel_data(page_address(to));

    /*
     * Now copy the page using the same cache colour as the
     * pages ultimate destination.
     */
    raw_spin_lock(&mut V6_LOCK);

    kfrom = COPYPAGE_V6_FROM.wrapping_add((offset as c_ulong) << PAGE_SHIFT);
    kto = COPYPAGE_V6_TO.wrapping_add((offset as c_ulong) << PAGE_SHIFT);

    set_top_pte(kfrom, mk_pte(from, PAGE_KERNEL));
    set_top_pte(kto, mk_pte(to, PAGE_KERNEL));

    copy_page(kto as *mut c_void, kfrom as *mut c_void);

    raw_spin_unlock(&mut V6_LOCK);
}

/*
 * Clear the user page.  We need to deal with the aliasing issues,
 * so remap the kernel page into the same cache colour as the user
 * page.
 */
unsafe fn v6_clear_user_highpage_aliasing(page: *mut page, vaddr: c_ulong) {
    let to: c_ulong = COPYPAGE_V6_TO
        .wrapping_add((CACHE_COLOUR(vaddr) as c_ulong) << PAGE_SHIFT);

    /* FIXME: not highmem safe */
    discard_old_kernel_data(page_address(page));

    /*
     * Now clear the page using the same cache colour as
     * the pages ultimate destination.
     */
    raw_spin_lock(&mut V6_LOCK);

    set_top_pte(to, mk_pte(page, PAGE_KERNEL));
    clear_page(to as *mut c_void);

    raw_spin_unlock(&mut V6_LOCK);
}

static mut v6_user_fns: cpu_user_fns = cpu_user_fns {
    cpu_clear_user_highpage: v6_clear_user_highpage_nonaliasing,
    cpu_copy_user_highpage: v6_copy_user_highpage_nonaliasing,
};

unsafe fn v6_userpage_init() -> c_int {
    if cache_is_vipt_aliasing() {
        cpu_user.cpu_clear_user_highpage = v6_clear_user_highpage_aliasing;
        cpu_user.cpu_copy_user_highpage = v6_copy_user_highpage_aliasing;
    }

    0
}

// C: core_initcall(v6_userpage_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
