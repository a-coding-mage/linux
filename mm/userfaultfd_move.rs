// SPDX-License-Identifier: GPL-2.0-only
// Original unit lines 1237-2193: atomic anonymous-page relocation.
#[no_mangle]
pub unsafe extern "C" fn double_pt_lock(mut a: *mut spinlock_t, mut b: *mut spinlock_t) {
    if a > b {
        core::mem::swap(&mut a, &mut b);
    }
    spin_lock(a);
    if a != b {
        spin_lock_nested(b, SINGLE_DEPTH_NESTING as c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn double_pt_unlock(a: *mut spinlock_t, b: *mut spinlock_t) {
    spin_unlock(a);
    if a != b {
        spin_unlock(b);
    }
}
unsafe fn is_pte_pages_stable(
    dp: *mut pte_t,
    sp: *mut pte_t,
    de: pte_t,
    se: pte_t,
    dm: *mut pmd_t,
    dv: pmd_t,
) -> bool {
    pte_same(ptep_get(sp), se) && pte_same(ptep_get(dp), de) && pmd_same(dv, pmdp_get_lockless(dm))
}
unsafe fn check_ptes_for_batched_move(
    v: *mut vm_area_struct,
    a: c_ulong,
    sp: *mut pte_t,
    dp: *mut pte_t,
) -> *mut folio {
    if !pte_none(ptep_get(dp)) {
        return null_mut();
    }
    let s = ptep_get(sp);
    if !pte_present(s) || is_zero_pfn(pte_pfn(s)) {
        return null_mut();
    }
    let f = vm_normal_folio(v, a, s);
    if f.is_null() || !folio_trylock(f) {
        return null_mut();
    }
    if !PageAnonExclusive(folio_page(f)) || folio_test_large(f) {
        folio_unlock(f);
        return null_mut();
    }
    f
}
unsafe fn move_present_ptes(
    mm: *mut mm_struct,
    dv: *mut vm_area_struct,
    sv: *mut vm_area_struct,
    mut da: c_ulong,
    mut sa: c_ulong,
    mut dp: *mut pte_t,
    mut sp: *mut pte_t,
    mut de: pte_t,
    mut se: pte_t,
    dm: *mut pmd_t,
    dmval: pmd_t,
    dl: *mut spinlock_t,
    sl: *mut spinlock_t,
    first: &mut *mut folio,
    mut len: c_ulong,
) -> c_long {
    let mut ret = 0;
    let mut f = *first;
    let start = sa;
    len = pmd_addr_end(da, da.wrapping_add(len)) - da;
    let end = pmd_addr_end(sa, sa.wrapping_add(len));
    flush_cache_range(sv, sa, end);
    double_pt_lock(dl, sl);
    if !is_pte_pages_stable(dp, sp, de, se, dm, dmval) {
        ret = error(EAGAIN);
    } else if folio_test_large(f) || folio_maybe_dma_pinned(f) || !PageAnonExclusive(folio_page(f))
    {
        ret = error(EBUSY);
    } else {
        folio_put(*first);
        *first = null_mut();
        lazy_mmu_mode_enable();
        loop {
            se = ptep_get_and_clear(mm, sa, sp);
            if folio_maybe_dma_pinned(f) {
                set_pte_at(mm, sa, sp, se);
                ret = error(EBUSY);
                break;
            }
            folio_move_anon_rmap(f, dv);
            *folio_index_ptr(f) = linear_anon_page_index(dv, da);
            de = folio_mk_pte(f, (*dv).vm_page_prot);
            if pgtable_supports_soft_dirty() {
                de = pte_mksoft_dirty(de);
            }
            if pte_dirty(se) {
                de = pte_mkdirty(de);
            }
            de = pte_mkwrite(de, dv);
            #[cfg(CONFIG_ARCH_HAS_PTE_PROTNONE)]
            if userfaultfd_rwp(dv) {
                de = pte_mkuffd(pte_modify(de, page_none()));
            }
            set_pte_at(mm, da, dp, de);
            sa = sa.wrapping_add(PAGE_SIZE);
            if sa == end {
                break;
            }
            da = da.wrapping_add(PAGE_SIZE);
            dp = dp.add(1);
            sp = sp.add(1);
            folio_unlock(f);
            f = check_ptes_for_batched_move(sv, sa, sp, dp);
            if f.is_null() {
                break;
            }
        }
        lazy_mmu_mode_disable();
        if sa > start {
            flush_tlb_range(sv, start, sa);
        }
        if !f.is_null() {
            folio_unlock(f);
        }
    }
    double_pt_unlock(dl, sl);
    if sa > start {
        (sa - start) as c_long
    } else {
        ret as c_long
    }
}
unsafe fn move_swap_pte(
    mm: *mut mm_struct,
    dv: *mut vm_area_struct,
    da: c_ulong,
    sa: c_ulong,
    dp: *mut pte_t,
    sp: *mut pte_t,
    de: pte_t,
    mut se: pte_t,
    dm: *mut pmd_t,
    dmval: pmd_t,
    dl: *mut spinlock_t,
    sl: *mut spinlock_t,
    f: *mut folio,
    si: *mut swap_info_struct,
    entry: swp_entry_t,
) -> c_int {
    if !f.is_null() && (!folio_test_swapcache(f) || entry.val != folio_swap(f).val) {
        return error(EAGAIN);
    }
    double_pt_lock(dl, sl);
    if !is_pte_pages_stable(dp, sp, de, se, dm, dmval) {
        double_pt_unlock(dl, sl);
        return error(EAGAIN);
    }
    if !f.is_null() {
        folio_move_anon_rmap(f, dv);
        *folio_index_ptr(f) = linear_anon_page_index(dv, da);
    } else if swap_cache_has_folio(entry) {
        double_pt_unlock(dl, sl);
        return error(EAGAIN);
    }
    se = ptep_get_and_clear(mm, sa, sp);
    if pgtable_supports_soft_dirty() {
        se = pte_swp_mksoft_dirty(se);
    }
    if userfaultfd_rwp(dv) {
        se = pte_swp_mkuffd(se);
    }
    set_pte_at(mm, da, dp, se);
    double_pt_unlock(dl, sl);
    PAGE_SIZE as c_int
}
unsafe fn move_zeropage_pte(
    mm: *mut mm_struct,
    dv: *mut vm_area_struct,
    sv: *mut vm_area_struct,
    da: c_ulong,
    sa: c_ulong,
    dp: *mut pte_t,
    sp: *mut pte_t,
    de: pte_t,
    se: pte_t,
    dm: *mut pmd_t,
    dmval: pmd_t,
    dl: *mut spinlock_t,
    sl: *mut spinlock_t,
) -> c_int {
    double_pt_lock(dl, sl);
    if !is_pte_pages_stable(dp, sp, de, se, dm, dmval) {
        double_pt_unlock(dl, sl);
        return error(EAGAIN);
    }
    let mut zero = pte_mkspecial(pfn_pte(zero_pfn(da), (*dv).vm_page_prot));
    #[cfg(CONFIG_ARCH_HAS_PTE_PROTNONE)]
    if userfaultfd_rwp(dv) {
        zero = pte_mkuffd(pte_modify(zero, page_none()));
    }
    ptep_clear_flush(sv, sa, sp);
    set_pte_at(mm, da, dp, zero);
    double_pt_unlock(dl, sl);
    PAGE_SIZE as c_int
}
unsafe fn move_pages_ptes(
    mm: *mut mm_struct,
    dm: *mut pmd_t,
    sm: *mut pmd_t,
    dv: *mut vm_area_struct,
    sv: *mut vm_area_struct,
    da: c_ulong,
    sa: c_ulong,
    len: c_ulong,
    mode: u64,
) -> c_long {
    let mut si = null_mut();
    let mut sp = null_mut();
    let mut dp = null_mut();
    let mut f: *mut folio = null_mut();
    let mut fpte: pte_t = zeroed();
    let mut range = zeroed();
    mmu_notifier_range_init(
        &mut range,
        MMU_NOTIFY_CLEAR,
        0,
        mm,
        sa,
        sa.wrapping_add(len),
    );
    mmu_notifier_invalidate_range_start(&mut range);
    let ret = 'retry: loop {
        let mut dmval = zeroed();
        let mut dummy = zeroed();
        let mut dl = null_mut();
        let mut sl = null_mut();
        dp = pte_offset_map_rw_nolock(mm, dm, da, &mut dmval, &mut dl);
        if dp.is_null() {
            break error(EAGAIN) as c_long;
        }
        sp = pte_offset_map_rw_nolock(mm, sm, sa, &mut dummy, &mut sl);
        if sp.is_null() {
            break error(EAGAIN) as c_long;
        }
        if pmd_none(*dm) || pmd_none(*sm) || pmd_trans_huge(*dm) || pmd_trans_huge(*sm) {
            break error(EINVAL) as c_long;
        }
        spin_lock(dl);
        let de = ptep_get(dp);
        spin_unlock(dl);
        if !pte_none(de) {
            break error(EEXIST) as c_long;
        }
        spin_lock(sl);
        let se = ptep_get(sp);
        spin_unlock(sl);
        if pte_none(se) {
            break if mode & RUST_UFFD_UFFDIO_MOVE_MODE_ALLOW_SRC_HOLES == 0 {
                error(ENOENT) as c_long
            } else {
                PAGE_SIZE as c_long
            };
        }
        if !f.is_null() && !pte_same(fpte, se) {
            break error(EAGAIN) as c_long;
        }
        if pte_present(se) {
            if is_zero_pfn(pte_pfn(se)) {
                break move_zeropage_pte(mm, dv, sv, da, sa, dp, sp, de, se, dm, dmval, dl, sl)
                    as c_long;
            }
            if f.is_null() {
                spin_lock(sl);
                if !pte_same(se, ptep_get(sp)) {
                    spin_unlock(sl);
                    break error(EAGAIN) as c_long;
                }
                let candidate = vm_normal_folio(sv, sa, se);
                if candidate.is_null() || !PageAnonExclusive(folio_page(candidate)) {
                    spin_unlock(sl);
                    break error(EBUSY) as c_long;
                }
                let locked = folio_trylock(candidate);
                if !locked && folio_test_large(candidate) {
                    spin_unlock(sl);
                    break error(EAGAIN) as c_long;
                }
                folio_get(candidate);
                f = candidate;
                fpte = se;
                spin_unlock(sl);
                if !locked {
                    pte_unmap(sp);
                    pte_unmap(dp);
                    sp = null_mut();
                    dp = null_mut();
                    folio_lock(f);
                    continue 'retry;
                }
                if warn_move_anon(!folio_test_anon(f)) {
                    break error(EBUSY) as c_long;
                }
            }
            if folio_test_large(f) {
                pte_unmap(sp);
                pte_unmap(dp);
                sp = null_mut();
                dp = null_mut();
                let r = split_folio(f);
                if r != 0 {
                    break r as c_long;
                }
                folio_unlock(f);
                folio_put(f);
                f = null_mut();
                continue 'retry;
            }
            break move_present_ptes(
                mm, dv, sv, da, sa, dp, sp, de, se, dm, dmval, dl, sl, &mut f, len,
            );
        } else {
            let entry = softleaf_from_pte(se);
            if softleaf_is_migration(entry) {
                pte_unmap(sp);
                pte_unmap(dp);
                sp = null_mut();
                dp = null_mut();
                migration_entry_wait(mm, sm, sa);
                break error(EAGAIN) as c_long;
            }
            if !softleaf_is_swap(entry) {
                break error(EFAULT) as c_long;
            }
            if !pte_swp_exclusive(se) {
                break error(EBUSY) as c_long;
            }
            si = get_swap_device(entry);
            if si.is_null() {
                break error(EAGAIN) as c_long;
            }
            let candidate = if f.is_null() {
                swap_cache_get_folio(entry)
            } else {
                null_mut()
            };
            if !candidate.is_null() {
                if folio_test_large(candidate) {
                    folio_put(candidate);
                    break error(EBUSY) as c_long;
                }
                f = candidate;
                fpte = se;
                if !folio_trylock(f) {
                    pte_unmap(sp);
                    pte_unmap(dp);
                    sp = null_mut();
                    dp = null_mut();
                    put_swap_device(si);
                    si = null_mut();
                    folio_lock(f);
                    continue 'retry;
                }
            }
            break move_swap_pte(
                mm, dv, da, sa, dp, sp, de, se, dm, dmval, dl, sl, f, si, entry,
            ) as c_long;
        }
    };
    if !f.is_null() {
        folio_unlock(f);
        folio_put(f);
    }
    // LIFO is required for CONFIG_HIGHPTE kmap_local indices.
    if !sp.is_null() {
        pte_unmap(sp);
    }
    if !dp.is_null() {
        pte_unmap(dp);
    }
    mmu_notifier_invalidate_range_end(&mut range);
    if !si.is_null() {
        put_swap_device(si);
    }
    ret
}
unsafe fn move_splits_huge_pmd(dst: c_ulong, src: c_ulong, end: c_ulong) -> bool {
    #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
    {
        let size = RUST_UFFD_HPAGE_PMD_SIZE;
        return (dst | src) & (size - 1) != 0 || end - src < size;
    }
    #[cfg(not(CONFIG_TRANSPARENT_HUGEPAGE))]
    {
        false
    }
}
unsafe fn vma_move_compatible(v: *mut vm_area_struct) -> bool {
    vflags(v)
        & (RUST_UFFD_VM_PFNMAP
            | RUST_UFFD_VM_IO
            | RUST_UFFD_VM_HUGETLB
            | RUST_UFFD_VM_MIXEDMAP
            | RUST_UFFD_VM_SHADOW_STACK)
        == 0
}
unsafe fn validate_move_areas(
    c: *mut userfaultfd_ctx,
    s: *mut vm_area_struct,
    d: *mut vm_area_struct,
) -> c_int {
    if vflags(s) & RUST_UFFD_VM_ACCESS_FLAGS != vflags(d) & RUST_UFFD_VM_ACCESS_FLAGS
        || pgprot_val((*s).vm_page_prot) != pgprot_val((*d).vm_page_prot)
        || vflags(s) & RUST_UFFD_VM_LOCKED != vflags(d) & RUST_UFFD_VM_LOCKED
        || vflags(s) & RUST_UFFD_VM_WRITE == 0
        || !vma_move_compatible(s)
        || !vma_move_compatible(d)
        || vctx(d).is_null()
        || vctx(d) != c
        || !vma_is_anonymous(s)
        || !vma_is_anonymous(d)
    {
        error(EINVAL)
    } else {
        0
    }
}
unsafe fn find_vmas_mm_locked(
    mm: *mut mm_struct,
    dst: c_ulong,
    src: c_ulong,
    dp: *mut *mut vm_area_struct,
    sp: *mut *mut vm_area_struct,
) -> c_int {
    mmap_assert_locked(mm);
    let mut v = find_vma_and_prepare_anon(mm, dst);
    if is_err(v) {
        return ptr_err(v);
    }
    *dp = v;
    if src < vstart(v) || src >= vend(v) {
        v = vma_lookup(mm, src);
        if v.is_null() {
            return error(ENOENT);
        }
    }
    *sp = v;
    0
}
unsafe fn uffd_move_lock(
    mm: *mut mm_struct,
    dst: c_ulong,
    src: c_ulong,
    dp: *mut *mut vm_area_struct,
    sp: *mut *mut vm_area_struct,
) -> c_int {
    #[cfg(CONFIG_PER_VMA_LOCK)]
    {
        let v = uffd_lock_vma(mm, dst);
        if is_err(v) {
            return ptr_err(v);
        }
        *dp = v;
        if src >= vstart(v) && src < vend(v) {
            *sp = v;
            return 0;
        }
        *sp = lock_vma_under_rcu(mm, src);
        if !(*sp).is_null() {
            return 0;
        }
        vma_end_read(*dp);
        mmap_read_lock(mm);
        let mut ret = find_vmas_mm_locked(mm, dst, src, dp, sp);
        if ret == 0 {
            if !vma_start_read_locked(*dp) {
                ret = error(EAGAIN);
            } else if *dp != *sp
                && !vma_start_read_locked_nested(*sp, SINGLE_DEPTH_NESTING as c_int)
            {
                vma_end_read(*dp);
                ret = error(EAGAIN);
            }
        }
        mmap_read_unlock(mm);
        ret
    }
    #[cfg(not(CONFIG_PER_VMA_LOCK))]
    {
        mmap_read_lock(mm);
        let ret = find_vmas_mm_locked(mm, dst, src, dp, sp);
        if ret != 0 {
            mmap_read_unlock(mm);
        }
        ret
    }
}
unsafe fn uffd_move_unlock(d: *mut vm_area_struct, s: *mut vm_area_struct) {
    #[cfg(CONFIG_PER_VMA_LOCK)]
    {
        vma_end_read(s);
        if d != s {
            vma_end_read(d);
        }
    }
    #[cfg(not(CONFIG_PER_VMA_LOCK))]
    {
        mmap_assert_locked((*s).vm_mm);
        mmap_read_unlock((*d).vm_mm);
    }
}
unsafe fn move_pages(
    c: *mut userfaultfd_ctx,
    dst: c_ulong,
    src: c_ulong,
    len: c_ulong,
    mode: u64,
) -> c_long {
    let mm = (*c).mm;
    let mut dv = null_mut();
    let mut sv = null_mut();
    let mut moved = 0;
    vm_warn!(warn_021, src & !PAGE_MASK != 0);
    vm_warn!(warn_022, dst & !PAGE_MASK != 0);
    vm_warn!(warn_023, len & !PAGE_MASK != 0);
    vm_warn!(warn_024, src.wrapping_add(len) < src);
    vm_warn!(warn_025, dst.wrapping_add(len) < dst);
    let ret = uffd_move_lock(mm, dst, src, &mut dv, &mut sv);
    if ret != 0 {
        return ret as c_long;
    }
    down_read(addr_of_mut!((*c).map_changing_lock));
    let ret = (|| {
        if atomic_read(addr_of!((*c).mmap_changing)) != 0 {
            return error(EAGAIN) as c_long;
        }
        if vflags(sv) & RUST_UFFD_VM_SHARED != 0
            || src.wrapping_add(len) > vend(sv)
            || vflags(dv) & RUST_UFFD_VM_SHARED != 0
            || dst.wrapping_add(len) > vend(dv)
        {
            return error(EINVAL) as c_long;
        }
        let mut ret = validate_move_areas(c, sv, dv) as c_long;
        if ret != 0 {
            return ret;
        }
        let mut sa = src;
        let mut da = dst;
        let end = src.wrapping_add(len);
        while sa < end {
            let mut sm = mm_find_pmd(mm, sa);
            if sm.is_null() {
                if mode & RUST_UFFD_UFFDIO_MOVE_MODE_ALLOW_SRC_HOLES == 0 {
                    ret = error(ENOENT) as c_long;
                    break;
                }
                sm = mm_alloc_pmd(mm, sa);
                if sm.is_null() {
                    ret = error(ENOMEM) as c_long;
                    break;
                }
            }
            let dm = mm_alloc_pmd(mm, da);
            if dm.is_null() {
                ret = error(ENOMEM) as c_long;
                break;
            }
            let dmval = pmdp_get_lockless(dm);
            if pmd_trans_huge(dmval) {
                ret = error(EEXIST) as c_long;
                break;
            }
            let mut step = 0;
            let mut huge_moved = false;
            #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
            let ptl = pmd_trans_huge_lock(sm, sv);
            #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
            if !ptl.is_null() {
                if move_splits_huge_pmd(da, sa, end) || !pmd_none(dmval) {
                    if pmd_present(*sm) {
                        let f = pmd_folio(*sm);
                        if !is_huge_zero_folio(f) && !PageAnonExclusive(folio_page(f)) {
                            spin_unlock(ptl);
                            ret = error(EBUSY) as c_long;
                            break;
                        }
                    }
                    spin_unlock(ptl);
                    split_huge_pmd(sv, sm, sa);
                    continue;
                }
                ret = move_pages_huge_pmd(mm, dm, sm, dmval, dv, sv, da, sa) as c_long;
                step = RUST_UFFD_HPAGE_PMD_SIZE;
                huge_moved = true;
            }
            if !huge_moved {
                if pmd_none(*sm) {
                    if mode & RUST_UFFD_UFFDIO_MOVE_MODE_ALLOW_SRC_HOLES == 0 {
                        ret = error(ENOENT) as c_long;
                        break;
                    }
                    if __pte_alloc(mm, sm) != 0 {
                        ret = error(ENOMEM) as c_long;
                        break;
                    }
                }
                if pte_alloc(mm, dm) != 0 {
                    ret = error(ENOMEM) as c_long;
                    break;
                }
                let r = move_pages_ptes(mm, dm, sm, dv, sv, da, sa, end - sa, mode);
                if r < 0 {
                    ret = r;
                } else {
                    step = r as c_ulong;
                }
            }
            cond_resched();
            if fatal_signal_pending(current()) {
                if ret == 0 || ret == error(EAGAIN) as c_long {
                    ret = error(EINTR) as c_long;
                }
                break;
            }
            if ret != 0 {
                if ret == error(EAGAIN) as c_long {
                    continue;
                }
                break;
            }
            da = da.wrapping_add(step);
            sa = sa.wrapping_add(step);
            moved += step as c_long;
        }
        ret
    })();
    up_read(addr_of_mut!((*c).map_changing_lock));
    uffd_move_unlock(dv, sv);
    vm_warn!(warn_026, moved < 0);
    vm_warn!(warn_027, ret > 0);
    vm_warn!(warn_028, moved == 0 && ret == 0);
    if moved != 0 {
        moved
    } else {
        ret
    }
}
