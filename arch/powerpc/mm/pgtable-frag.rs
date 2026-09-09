// SPDX-License-Identifier: GPL-2.0

/*
 *  Handling Page Tables through page fragments
 *
 */

pub unsafe fn pte_frag_destroy(pte_frag: *mut core::ffi::c_void) {
    let count: i32;
    let ptdesc: *mut ptdesc;

    ptdesc = virt_to_ptdesc(pte_frag);
    /* drop all the pending references */
    count = (((pte_frag as usize) & !PAGE_MASK) >> PTE_FRAG_SIZE_SHIFT) as i32;
    /* We allow PTE_FRAG_NR fragments from a PTE page */
    if atomic_sub_and_test(PTE_FRAG_NR - count, &mut (*ptdesc).pt_frag_refcount) {
        folio_clear_active(ptdesc_folio(ptdesc));
        pagetable_dtor(ptdesc);
        pagetable_free(ptdesc);
    }
}

unsafe fn get_pte_from_cache(mm: *mut mm_struct) -> *mut pte_t {
    let mut pte_frag: *mut core::ffi::c_void;
    let ret: *mut core::ffi::c_void;

    if PTE_FRAG_NR == 1 {
        return core::ptr::null_mut();
    }

    spin_lock(&mut (*mm).page_table_lock);
    ret = pte_frag_get(&mut (*mm).context);
    if !ret.is_null() {
        pte_frag = (ret as usize + PTE_FRAG_SIZE) as *mut core::ffi::c_void;
        /*
         * If we have taken up all the fragments mark PTE page NULL
         */
        if ((pte_frag as usize) & !PAGE_MASK) == 0 {
            pte_frag = core::ptr::null_mut();
        }
        pte_frag_set(&mut (*mm).context, pte_frag);
    }
    spin_unlock(&mut (*mm).page_table_lock);
    ret as *mut pte_t
}

unsafe fn __alloc_for_ptecache(mm: *mut mm_struct, kernel: i32) -> *mut pte_t {
    let mut ret: *mut core::ffi::c_void = core::ptr::null_mut();
    let ptdesc: *mut ptdesc;
    let mut gfp: gfp_t = PGALLOC_GFP;

    if kernel == 0 {
        gfp |= __GFP_ACCOUNT;
    }

    ptdesc = pagetable_alloc(gfp, 0);
    if ptdesc.is_null() {
        return core::ptr::null_mut();
    }
    if !pagetable_pte_ctor(mm, ptdesc) {
        pagetable_free(ptdesc);
        return core::ptr::null_mut();
    }

    atomic_set(&mut (*ptdesc).pt_frag_refcount, 1);

    ret = ptdesc_address(ptdesc);
    /*
     * if we support only one fragment just return the
     * allocated page.
     */
    if PTE_FRAG_NR == 1 {
        return ret as *mut pte_t;
    }
    spin_lock(&mut (*mm).page_table_lock);
    /*
     * If we find ptdesc_page set, we return
     * the allocated page with single fragment
     * count.
     */
    if likely(pte_frag_get(&mut (*mm).context).is_null()) {
        atomic_set(&mut (*ptdesc).pt_frag_refcount, PTE_FRAG_NR);
        pte_frag_set(
            &mut (*mm).context,
            (ret as usize + PTE_FRAG_SIZE) as *mut core::ffi::c_void,
        );
    }
    spin_unlock(&mut (*mm).page_table_lock);

    ret as *mut pte_t
}

pub unsafe fn pte_fragment_alloc(mm: *mut mm_struct, kernel: i32) -> *mut pte_t {
    let pte: *mut pte_t;

    pte = get_pte_from_cache(mm);
    if !pte.is_null() {
        return pte;
    }

    __alloc_for_ptecache(mm, kernel)
}

unsafe fn pte_free_now(head: *mut rcu_head) {
    let ptdesc: *mut ptdesc;

    ptdesc = container_of(head, ptdesc, pt_rcu_head);
    pagetable_dtor(ptdesc);
    pagetable_free(ptdesc);
}

pub unsafe fn pte_fragment_free(table: *mut usize, kernel: i32) {
    let ptdesc: *mut ptdesc = virt_to_ptdesc(table as *mut core::ffi::c_void);

    if pagetable_is_reserved(ptdesc) {
        return free_reserved_ptdesc(ptdesc);
    }

    BUG_ON(atomic_read(&(*ptdesc).pt_frag_refcount) <= 0);
    if atomic_dec_and_test(&mut (*ptdesc).pt_frag_refcount) {
        if kernel != 0 || !folio_test_clear_active(ptdesc_folio(ptdesc)) {
            pte_free_now(&mut (*ptdesc).pt_rcu_head);
        } else {
            call_rcu(&mut (*ptdesc).pt_rcu_head, pte_free_now);
        }
    }
}

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
pub unsafe fn pte_free_defer(mm: *mut mm_struct, pgtable: pgtable_t) {
    let folio: *mut folio;

    folio = virt_to_folio(pgtable);
    folio_set_active(folio);
    pte_fragment_free(pgtable as *mut usize, 0);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
