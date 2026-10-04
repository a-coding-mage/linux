// SPDX-License-Identifier: GPL-2.0-only
// Original mm/vmscan.c:3269-4363: generation aging, walkers and rmap feedback.
unsafe fn folio_update_gen(f: *mut folio, gen: i32, flags: *const vma_flags_t) -> i32 {
    let p = folio_flags_ptr(f);
    let mut old = rd(p);
    vmwarn!(9, gen >= MAX_NR_GENS as i32);
    if !folio_test_referenced(f) && !folio_test_workingset(f) && !is_exec_file_folio(f, flags) {
        rust_mglru_set_mask_bits(p, LRU_REFS_MASK as ULong, 1 << PG_referenced);
        return -1;
    }
    loop {
        if old & LRU_GEN_MASK as ULong == 0 {
            return -1;
        }
        let new = (old & !(LRU_GEN_MASK as ULong | LRU_REFS_FLAGS as ULong))
            | (((gen as ULong).wrapping_add(1)) << LRU_GEN_PGOFF)
            | (1 << PG_workingset);
        if rust_mglru_try_cmpxchg_ulong(p, &mut old, new) {
            break;
        }
    }
    ((old & LRU_GEN_MASK as ULong) >> LRU_GEN_PGOFF).wrapping_sub(1) as i32
}
unsafe fn folio_inc_gen(v: *mut lruvec, f: *mut folio) -> usize {
    let typ = folio_is_file_lru(f) as usize;
    let old_gen = lru_gen_from_seq((*v).lrugen.min_seq[typ]) as usize;
    let p = folio_flags_ptr(f);
    let mut old = rd(p);
    folio_warn!(1, old & LRU_GEN_MASK as ULong == 0, f);
    let new_gen;
    loop {
        let observed = ((old & LRU_GEN_MASK as ULong) >> LRU_GEN_PGOFF).wrapping_sub(1) as i32;
        if observed >= 0 && observed as usize != old_gen {
            return observed as usize;
        }
        let gen = (old_gen + 1) % MAX_NR_GENS as usize;
        let new = (old & !(LRU_GEN_MASK as ULong | LRU_REFS_FLAGS as ULong))
            | (((gen + 1) as ULong) << LRU_GEN_PGOFF);
        if rust_mglru_try_cmpxchg_ulong(p, &mut old, new) {
            new_gen = gen;
            break;
        }
    }
    lru_gen_update_size(v, f, old_gen as i32, new_gen as i32);
    new_gen
}
unsafe fn update_batch_size(w: *mut lru_gen_mm_walk, f: *mut folio, old: i32, new: i32) {
    let typ = folio_is_file_lru(f) as usize;
    let zone = folio_zonenum(f) as usize;
    let delta = folio_nr_pages(f) as i32;
    vmwarn!(10, old >= MAX_NR_GENS as i32);
    vmwarn!(11, new >= MAX_NR_GENS as i32);
    (*w).batched = (*w).batched.wrapping_add(1);
    (*w).nr_pages[old as usize][typ][zone] =
        (*w).nr_pages[old as usize][typ][zone].wrapping_sub(delta);
    (*w).nr_pages[new as usize][typ][zone] =
        (*w).nr_pages[new as usize][typ][zone].wrapping_add(delta);
}
unsafe fn reset_batch_size(w: *mut lru_gen_mm_walk) {
    let v = lruvec_live_lock_irq((*w).lruvec);
    let g = addr_of_mut!((*v).lrugen);
    (*w).batched = 0;
    for gen in 0..MAX_NR_GENS as usize {
        for typ in 0..ANON_AND_FILE as usize {
            for zone in 0..MAX_NR_ZONES as usize {
                let mut lru = typ as u32 * LRU_INACTIVE_FILE as u32;
                let delta = (*w).nr_pages[gen][typ][zone];
                if delta == 0 {
                    continue;
                }
                (*w).nr_pages[gen][typ][zone] = 0;
                wr(
                    addr_of_mut!((*g).nr_pages[gen][typ][zone]),
                    (*g).nr_pages[gen][typ][zone].wrapping_add(delta as Long),
                );
                if lru_gen_is_active(v, gen as i32) {
                    lru += LRU_ACTIVE as u32;
                }
                __update_lru_size(v, lru as _, zone as _, delta as _);
            }
        }
    }
    lruvec_unlock_irq(v);
}
#[export_name = "rust_mglru_should_skip_vma"]
unsafe extern "C" fn should_skip_vma(_start: ULong, _end: ULong, args: *mut mm_walk) -> i32 {
    let vma = (*args).vma;
    let w = (*args).private as *mut lru_gen_mm_walk;
    if !vma_is_accessible(vma) || is_vm_hugetlb_page(vma) || !vma_has_recency(vma) {
        return 1;
    }
    if rust_mglru_vma_locked_or_special(vma) || vma == get_gate_vma((*vma).vm_mm) {
        return 1;
    }
    if vma_is_anonymous(vma) {
        return ((*w).swappiness == 0) as i32;
    }
    if warn!(
        1,
        (*vma).vm_file.is_null() || (*(*vma).vm_file).f_mapping.is_null()
    ) {
        return 1;
    }
    let mapping = (*(*vma).vm_file).f_mapping;
    if mapping_unevictable(mapping) {
        return 1;
    }
    if shmem_mapping(mapping) {
        return ((*w).swappiness == 0) as i32;
    }
    if (*w).swappiness > MAX_SWAPPINESS as i32 {
        return 1;
    }
    (*(*mapping).a_ops).read_folio.is_none() as i32
}
unsafe fn get_next_vma(
    mask: ULong,
    size: ULong,
    args: *mut mm_walk,
    vm_start: *mut ULong,
    vm_end: *mut ULong,
) -> bool {
    let start = (*vm_end).wrapping_add(size - 1) & !(size - 1);
    let end = (start | !mask).wrapping_add(1);
    let mut vmi: vma_iterator = zeroed();
    rust_mglru_vma_iter_init(&mut vmi, (*args).mm, start);
    vmwarn!(12, mask & size != 0);
    vmwarn!(13, start & mask != *vm_start & mask);
    loop {
        (*args).vma = rust_mglru_vma_next(&mut vmi);
        if (*args).vma.is_null() {
            return false;
        }
        let v = (*args).vma;
        if end != 0 && end <= rust_mglru_vma_start(v) {
            return false;
        }
        if should_skip_vma(rust_mglru_vma_start(v), rust_mglru_vma_end(v), args) != 0 {
            continue;
        }
        *vm_start = max(start, rust_mglru_vma_start(v));
        *vm_end = min(end.wrapping_sub(1), rust_mglru_vma_end(v).wrapping_sub(1)).wrapping_add(1);
        return true;
    }
}
unsafe fn get_pte_pfn(
    pte: pte_t,
    vma: *mut vm_area_struct,
    addr: ULong,
    pgdat: *mut pglist_data,
) -> ULong {
    let pfn = rust_mglru_pte_pfn(pte);
    vmwarn!(
        14,
        addr < rust_mglru_vma_start(vma) || addr >= rust_mglru_vma_end(vma)
    );
    if !rust_mglru_pte_present(pte) || rust_mglru_is_zero_pfn(pfn) {
        return ULong::MAX;
    }
    if warn!(2, rust_mglru_pte_special(pte)) {
        return ULong::MAX;
    }
    if !rust_mglru_pte_young(pte) && !mm_has_notifiers((*vma).vm_mm) {
        return ULong::MAX;
    }
    if warn!(3, !rust_mglru_pfn_valid(pfn)) {
        return ULong::MAX;
    }
    if pfn < (*pgdat).node_start_pfn || pfn >= pgdat_end_pfn(pgdat) {
        return ULong::MAX;
    }
    pfn
}
unsafe fn get_pmd_pfn(
    pmd: pmd_t,
    vma: *mut vm_area_struct,
    addr: ULong,
    pgdat: *mut pglist_data,
) -> ULong {
    let pfn = rust_mglru_pmd_pfn(pmd);
    vmwarn!(
        15,
        addr < rust_mglru_vma_start(vma) || addr >= rust_mglru_vma_end(vma)
    );
    if !rust_mglru_pmd_present(pmd) || rust_mglru_is_huge_zero_pmd(pmd) {
        return ULong::MAX;
    }
    if !rust_mglru_pmd_young(pmd) && !mm_has_notifiers((*vma).vm_mm) {
        return ULong::MAX;
    }
    if warn!(4, !rust_mglru_pfn_valid(pfn)) {
        return ULong::MAX;
    }
    if pfn < (*pgdat).node_start_pfn || pfn >= pgdat_end_pfn(pgdat) {
        return ULong::MAX;
    }
    pfn
}
unsafe fn get_pfn_folio(pfn: ULong, memcg: *mut mem_cgroup, pgdat: *mut pglist_data) -> *mut folio {
    let mut f = rust_mglru_pfn_folio(pfn);
    if folio_lru_gen(f) < 0 || folio_nid(f) != (*pgdat).node_id {
        return null_mut();
    }
    rcu_read_lock();
    if folio_memcg(f) != memcg {
        f = null_mut();
    }
    rcu_read_unlock();
    f
}
unsafe fn suitable_to_scan(total: i32, young: i32) -> bool {
    let n = (rust_mglru_cache_line_size() as usize / size_of::<pte_t>()).clamp(2, 8) as i32;
    young.wrapping_mul(n) >= total
}
unsafe fn walk_update_folio(
    w: *mut lru_gen_mm_walk,
    vma: *mut vm_area_struct,
    f: *mut folio,
    new: i32,
    dirty: bool,
) {
    if f.is_null() {
        return;
    }
    if dirty
        && !folio_test_dirty(f)
        && !(folio_test_anon(f) && folio_test_swapbacked(f) && !folio_test_swapcache(f))
    {
        folio_mark_dirty(f);
    }
    if !w.is_null() {
        let old = folio_update_gen(f, new, rust_mglru_vma_flags_ptr(vma));
        if old >= 0 && old != new {
            update_batch_size(w, f, old, new);
        }
    } else if lru_gen_set_refs(f, rust_mglru_vma_flags_ptr(vma)) {
        let old = folio_lru_gen(f);
        if old >= 0 && old != new {
            folio_activate(f);
        }
    }
}
unsafe fn walk_pte_range(
    pmd: *mut pmd_t,
    mut start: ULong,
    mut end: ULong,
    args: *mut mm_walk,
) -> bool {
    let mut ptl = null_mut();
    let mut total = 0i32;
    let mut young = 0i32;
    let mut last = null_mut();
    // C's dirty is unused while last == NULL; initialize to avoid Rust UB.
    let mut dirty = false;
    let w = (*args).private as *mut lru_gen_mm_walk;
    let memcg = lruvec_memcg((*w).lruvec);
    let pgdat = lruvec_pgdat((*w).lruvec);
    let gen = lru_gen_from_seq(max_seq((*w).lruvec));
    let mut val: pmd_t = zeroed();
    let pte = rust_mglru_pte_offset_map_rw_nolock(
        (*args).mm,
        pmd,
        start & PMD_MASK as ULong,
        &mut val,
        &mut ptl,
    );
    if pte.is_null() {
        return false;
    }
    if !spin_trylock(ptl) {
        rust_mglru_pte_unmap(pte);
        return true;
    }
    if !rust_mglru_pmd_same(val, rust_mglru_pmdp_get_lockless(pmd)) {
        rust_mglru_pte_unmap_unlock(pte, ptl);
        return false;
    }
    rust_mglru_lazy_mmu_mode_enable();
    loop {
        let mut i = rust_mglru_pte_index(start) as usize;
        let mut addr = start;
        while addr != end {
            let cur = pte.add(i);
            let mut ptent = rust_mglru_ptep_get(cur);
            let mut nr = 1u32;
            total = total.wrapping_add(1);
            (*w).mm_stats[MM_LEAF_TOTAL as usize] =
                (*w).mm_stats[MM_LEAF_TOTAL as usize].wrapping_add(1);
            let pfn = get_pte_pfn(ptent, (*args).vma, addr, pgdat);
            if pfn != ULong::MAX {
                let f = get_pfn_folio(pfn, memcg, pgdat);
                if !f.is_null() {
                    if folio_test_large(f) {
                        nr = rust_mglru_folio_pte_batch(
                            f,
                            cur,
                            &mut ptent,
                            (end.wrapping_sub(addr) >> PAGE_SHIFT) as u32,
                        );
                        total = total.wrapping_add((nr as i32).wrapping_sub(1));
                        (*w).mm_stats[MM_LEAF_TOTAL as usize] = (*w).mm_stats
                            [MM_LEAF_TOTAL as usize]
                            .wrapping_add((nr as i32).wrapping_sub(1));
                    }
                    if rust_mglru_clear_young_ptes((*args).vma, addr, cur, nr) {
                        if last != f {
                            walk_update_folio(w, (*args).vma, last, gen, dirty);
                            last = f;
                            dirty = false;
                        }
                        if rust_mglru_pte_dirty(ptent) {
                            dirty = true;
                        }
                        young = young.wrapping_add(nr as i32);
                        (*w).mm_stats[MM_LEAF_YOUNG as usize] =
                            (*w).mm_stats[MM_LEAF_YOUNG as usize].wrapping_add(nr as i32);
                    }
                }
            }
            i += nr as usize;
            addr = addr.wrapping_add((nr as ULong).wrapping_mul(PAGE_SIZE as ULong));
        }
        walk_update_folio(w, (*args).vma, last, gen, dirty);
        last = null_mut();
        if !(i < PTRS_PER_PTE as usize
            && get_next_vma(
                PMD_MASK as ULong,
                PAGE_SIZE as ULong,
                args,
                &mut start,
                &mut end,
            ))
        {
            break;
        }
    }
    rust_mglru_lazy_mmu_mode_disable();
    rust_mglru_pte_unmap_unlock(pte, ptl);
    suitable_to_scan(total, young)
}
unsafe fn walk_pmd_range_locked(
    pud: *mut pud_t,
    mut addr: ULong,
    vma: *mut vm_area_struct,
    args: *mut mm_walk,
    bitmap: *mut ULong,
    first: *mut ULong,
) {
    let mut last = null_mut();
    let mut dirty = false;
    let w = (*args).private as *mut lru_gen_mm_walk;
    let memcg = lruvec_memcg((*w).lruvec);
    let pgdat = lruvec_pgdat((*w).lruvec);
    let gen = lru_gen_from_seq(max_seq((*w).lruvec));
    vmwarn!(16, rust_mglru_pud_leaf(*pud));
    if *first == ULong::MAX {
        *first = addr;
        rust_mglru_bitmap_zero(bitmap, MIN_LRU_BATCH as u32);
        return;
    }
    let mut i: i32 = if addr == ULong::MAX {
        0
    } else {
        rust_mglru_pmd_index(addr).wrapping_sub(rust_mglru_pmd_index(*first)) as i32
    };
    if i != 0 && i <= MIN_LRU_BATCH as i32 {
        rust_mglru___set_bit(i.wrapping_sub(1), bitmap);
        return;
    }
    let pmd = rust_mglru_pmd_offset(pud, *first);
    let ptl = rust_mglru_pmd_lockptr((*args).mm, pmd);
    if spin_trylock(ptl) {
        rust_mglru_lazy_mmu_mode_enable();
        loop {
            addr = if i != 0 {
                (*first & PMD_MASK as ULong)
                    .wrapping_add((i as ULong).wrapping_mul(PMD_SIZE as ULong))
            } else {
                *first
            };
            let p = pmd.offset(i as isize);
            if rust_mglru_pmd_present(*p) {
                if !rust_mglru_pmd_trans_huge(*p) {
                    if !(*w).force_scan && should_clear_pmd_young() && !mm_has_notifiers((*args).mm)
                    {
                        rust_mglru_pmdp_test_and_clear_young(vma, addr, p);
                    }
                } else {
                    let pfn = get_pmd_pfn(*p, vma, addr, pgdat);
                    if pfn != ULong::MAX {
                        let f = get_pfn_folio(pfn, memcg, pgdat);
                        if !f.is_null() && rust_mglru_pmdp_test_and_clear_young_notify(vma, addr, p)
                        {
                            if last != f {
                                walk_update_folio(w, vma, last, gen, dirty);
                                last = f;
                                dirty = false;
                            }
                            if rust_mglru_pmd_dirty(*p) {
                                dirty = true;
                            }
                            (*w).mm_stats[MM_LEAF_YOUNG as usize] =
                                (*w).mm_stats[MM_LEAF_YOUNG as usize].wrapping_add(1);
                        }
                    }
                }
            }
            i = if i > MIN_LRU_BATCH as i32 {
                0
            } else {
                rust_mglru_find_next_bit(bitmap, MIN_LRU_BATCH as ULong, i as ULong).wrapping_add(1)
                    as i32
            };
            if i > MIN_LRU_BATCH as i32 {
                break;
            }
        }
        walk_update_folio(w, vma, last, gen, dirty);
        rust_mglru_lazy_mmu_mode_disable();
        spin_unlock(ptl);
    }
    *first = ULong::MAX;
}
unsafe fn walk_pmd_range(pud: *mut pud_t, mut start: ULong, mut end: ULong, args: *mut mm_walk) {
    let mut bitmap =
        [0 as ULong; (MIN_LRU_BATCH as usize + ULong::BITS as usize - 1) / ULong::BITS as usize];
    let mut first = ULong::MAX;
    let w = (*args).private as *mut lru_gen_mm_walk;
    let state = get_mm_state((*w).lruvec);
    vmwarn!(17, rust_mglru_pud_leaf(*pud));
    let pmd = rust_mglru_pmd_offset(pud, start & PUD_MASK as ULong);
    loop {
        let vma = (*args).vma;
        let mut i = rust_mglru_pmd_index(start) as usize;
        let mut addr = start;
        while addr != end {
            let mut val = rust_mglru_pmdp_get_lockless(pmd.add(i));
            let next = rust_mglru_pmd_addr_end(addr, end);
            if !rust_mglru_pmd_present(val) || rust_mglru_is_huge_zero_pmd(val) {
                (*w).mm_stats[MM_LEAF_TOTAL as usize] =
                    (*w).mm_stats[MM_LEAF_TOTAL as usize].wrapping_add(1);
            } else if rust_mglru_pmd_trans_huge(val) {
                let pfn = get_pmd_pfn(val, vma, addr, lruvec_pgdat((*w).lruvec));
                (*w).mm_stats[MM_LEAF_TOTAL as usize] =
                    (*w).mm_stats[MM_LEAF_TOTAL as usize].wrapping_add(1);
                if pfn != ULong::MAX {
                    walk_pmd_range_locked(pud, addr, vma, args, bitmap.as_mut_ptr(), &mut first);
                }
            } else {
                let mut scan = true;
                if !(*w).force_scan && should_clear_pmd_young() && !mm_has_notifiers((*args).mm) {
                    if !rust_mglru_pmd_young(val) {
                        scan = false;
                    } else {
                        walk_pmd_range_locked(
                            pud,
                            addr,
                            vma,
                            args,
                            bitmap.as_mut_ptr(),
                            &mut first,
                        );
                    }
                }
                if scan
                    && ((*w).force_scan || test_bloom_filter(state, (*w).seq, pmd.add(i).cast()))
                {
                    (*w).mm_stats[MM_NONLEAF_FOUND as usize] =
                        (*w).mm_stats[MM_NONLEAF_FOUND as usize].wrapping_add(1);
                    if walk_pte_range(&mut val, addr, next, args) {
                        (*w).mm_stats[MM_NONLEAF_ADDED as usize] =
                            (*w).mm_stats[MM_NONLEAF_ADDED as usize].wrapping_add(1);
                        update_bloom_filter(state, (*w).seq.wrapping_add(1), pmd.add(i).cast());
                    }
                }
            }
            i += 1;
            addr = next;
        }
        walk_pmd_range_locked(pud, ULong::MAX, vma, args, bitmap.as_mut_ptr(), &mut first);
        if !(i < PTRS_PER_PMD as usize
            && get_next_vma(
                PUD_MASK as ULong,
                PMD_SIZE as ULong,
                args,
                &mut start,
                &mut end,
            ))
        {
            break;
        }
    }
}
#[export_name = "rust_mglru_walk_pud_range"]
unsafe extern "C" fn walk_pud_range(
    p4d: *mut p4d_t,
    mut start: ULong,
    mut end: ULong,
    args: *mut mm_walk,
) -> i32 {
    let w = (*args).private as *mut lru_gen_mm_walk;
    vmwarn!(18, rust_mglru_p4d_leaf(*p4d));
    let pud = rust_mglru_pud_offset(p4d, start & P4D_MASK as ULong);
    'walk: loop {
        let mut i = rust_mglru_pud_index(start) as usize;
        let mut addr = start;
        while addr != end {
            let mut val = rust_mglru_pudp_get(pud.add(i));
            let next = rust_mglru_pud_addr_end(addr, end);
            if rust_mglru_pud_present(val) && !warn!(5, rust_mglru_pud_leaf(val)) {
                walk_pmd_range(&mut val, addr, next, args);
                if need_resched() || (*w).batched >= MAX_LRU_BATCH as i32 {
                    end = (addr | !(PUD_MASK as ULong)).wrapping_add(1);
                    break 'walk;
                }
            }
            i += 1;
            addr = next;
        }
        if i < PTRS_PER_PUD as usize
            && get_next_vma(
                P4D_MASK as ULong,
                PUD_SIZE as ULong,
                args,
                &mut start,
                &mut end,
            )
        {
            continue;
        }
        end = end.wrapping_add(P4D_SIZE as ULong - 1) & !(P4D_SIZE as ULong - 1);
        break;
    }
    if end == 0 || (*args).vma.is_null() {
        return 1;
    }
    (*w).next_addr = max(end, rust_mglru_vma_start((*args).vma));
    -(EAGAIN as i32)
}
unsafe fn walk_mm(mm: *mut mm_struct, w: *mut lru_gen_mm_walk) {
    let v = (*w).lruvec;
    (*w).next_addr = FIRST_USER_ADDRESS as ULong;
    loop {
        let mut err = -(EBUSY as i32);
        if (*w).seq != max_seq(v) {
            break;
        }
        if mmap_read_trylock(mm) {
            err = walk_page_range(
                mm,
                (*w).next_addr,
                ULong::MAX,
                rust_mglru_mm_walk_ops(),
                w.cast(),
            );
            mmap_read_unlock(mm);
        }
        if (*w).batched != 0 {
            reset_batch_size(w);
        }
        cond_resched();
        if err != -(EAGAIN as i32) {
            break;
        }
    }
}
unsafe fn set_mm_walk(pgdat: *mut pglist_data, force_alloc: bool) -> *mut lru_gen_mm_walk {
    let state = (*current()).reclaim_state;
    let mut w = (*state).mm_walk;
    if !pgdat.is_null() && current_is_kswapd() {
        vmwarn!(19, !w.is_null());
        w = addr_of_mut!((*pgdat).mm_walk);
    } else if w.is_null() && force_alloc {
        vmwarn!(20, current_is_kswapd());
        w = rust_mglru_alloc_mm_walk();
    }
    (*state).mm_walk = w;
    w
}
unsafe fn clear_mm_walk() {
    let state = (*current()).reclaim_state;
    let w = (*state).mm_walk;
    vmwarn!(
        21,
        !w.is_null()
            && !memchr_inv(
                addr_of!((*w).nr_pages).cast(),
                0,
                size_of_val_raw_nr_pages(w)
            )
            .is_null()
    );
    vmwarn!(
        22,
        !w.is_null()
            && !memchr_inv(
                addr_of!((*w).mm_stats).cast(),
                0,
                size_of::<[i32; NR_MM_STATS as usize]>()
            )
            .is_null()
    );
    (*state).mm_walk = null_mut();
    if !current_is_kswapd() {
        kfree(w.cast());
    }
}
fn size_of_val_raw_nr_pages(_: *mut lru_gen_mm_walk) -> usize {
    size_of::<[[[i32; MAX_NR_ZONES as usize]; ANON_AND_FILE as usize]; MAX_NR_GENS as usize]>()
}
unsafe fn inc_min_seq(v: *mut lruvec, typ: usize, sw: i32) -> bool {
    let mut remaining = MAX_LRU_BATCH as i32;
    let g = addr_of_mut!((*v).lrugen);
    let hist = lru_hist_from_seq((*g).min_seq[typ]) as usize;
    let old = lru_gen_from_seq((*g).min_seq[typ]) as usize;
    if !(typ != 0 && sw == SWAPPINESS_ANON_ONLY as i32 || typ == 0 && sw == 0) {
        for zone in 0..MAX_NR_ZONES as usize {
            let head = addr_of_mut!((*g).folios[old][typ][zone]);
            while !list_empty(head) {
                let f = lru_to_folio(head);
                let refs = folio_lru_refs(f);
                let workingset = folio_test_workingset(f);
                folio_warn!(2, folio_test_unevictable(f), f);
                folio_warn!(3, folio_test_active(f), f);
                folio_warn!(4, folio_is_file_lru(f) as usize != typ, f);
                folio_warn!(5, folio_zonenum(f) as usize != zone, f);
                let new = folio_inc_gen(v, f);
                list_move_tail(folio_lru_ptr(f), addr_of_mut!((*g).folios[new][typ][zone]));
                if refs + workingset as i32 != (1 << LRU_REFS_WIDTH) + 1 {
                    let tier = lru_tier_from_refs(refs, workingset) as usize;
                    let delta = folio_nr_pages(f) as ULong;
                    wr(
                        addr_of_mut!((*g).protected[hist][typ][tier]),
                        (*g).protected[hist][typ][tier].wrapping_add(delta),
                    );
                }
                remaining -= 1;
                if remaining == 0 {
                    return false;
                }
            }
        }
    }
    reset_ctrl_pos(v, typ, true);
    wr(
        addr_of_mut!((*g).min_seq[typ]),
        (*g).min_seq[typ].wrapping_add(1),
    );
    true
}
unsafe fn try_to_inc_min_seq(v: *mut lruvec, sw: i32) {
    let mut increased = false;
    let g = addr_of_mut!((*v).lrugen);
    let mut seq = min_seq(v);
    vmwarn!(23, !seq_is_valid(v));
    for typ in min_type(sw)..=max_type(sw) {
        while seq[typ].wrapping_add(MIN_NR_GENS as ULong) <= (*g).max_seq {
            let gen = lru_gen_from_seq(seq[typ]) as usize;
            let mut populated = false;
            for zone in 0..MAX_NR_ZONES as usize {
                if !list_empty(addr_of!((*g).folios[gen][typ][zone])) {
                    populated = true;
                    break;
                }
            }
            if populated {
                break;
            }
            seq[typ] = seq[typ].wrapping_add(1);
            increased = true;
        }
    }
    if !increased {
        return;
    }
    if sw != 0 && sw <= MAX_SWAPPINESS as i32 {
        let floor = (*g).max_seq.wrapping_sub(MIN_NR_GENS as ULong);
        let anon = LRU_GEN_ANON as usize;
        let file = LRU_GEN_FILE as usize;
        if seq[anon] > floor && seq[file] < floor {
            seq[anon] = floor;
        } else if seq[file] > floor && seq[anon] < floor {
            seq[file] = floor;
        }
    }
    for typ in min_type(sw)..=max_type(sw) {
        if seq[typ] <= (*g).min_seq[typ] {
            continue;
        }
        reset_ctrl_pos(v, typ, true);
        wr(addr_of_mut!((*g).min_seq[typ]), seq[typ]);
    }
}
unsafe fn inc_max_seq(v: *mut lruvec, seq: ULong, sw: i32) -> bool {
    let g = addr_of_mut!((*v).lrugen);
    'restart: loop {
        if seq < rd(addr_of!((*g).max_seq)) {
            return false;
        }
        lruvec_lock_irq(v);
        vmwarn!(24, !seq_is_valid(v));
        let success = seq == (*g).max_seq;
        if !success {
            lruvec_unlock_irq(v);
            return false;
        }
        for typ in 0..ANON_AND_FILE as usize {
            if get_nr_gens(v, typ) != MAX_NR_GENS as i32 || inc_min_seq(v, typ, sw) {
                continue;
            }
            lruvec_unlock_irq(v);
            cond_resched();
            continue 'restart;
        }
        let prev = lru_gen_from_seq((*g).max_seq.wrapping_sub(1)) as usize;
        let next = lru_gen_from_seq((*g).max_seq.wrapping_add(1)) as usize;
        for typ in 0..ANON_AND_FILE as usize {
            for zone in 0..MAX_NR_ZONES as usize {
                let lru = typ as u32 * LRU_INACTIVE_FILE as u32;
                let delta =
                    (*g).nr_pages[prev][typ][zone].wrapping_sub((*g).nr_pages[next][typ][zone]);
                if delta == 0 {
                    continue;
                }
                __update_lru_size(v, lru as _, zone as _, delta);
                __update_lru_size(
                    v,
                    (lru + LRU_ACTIVE as u32) as _,
                    zone as _,
                    delta.wrapping_neg(),
                );
            }
        }
        for typ in 0..ANON_AND_FILE as usize {
            reset_ctrl_pos(v, typ, false);
        }
        wr(addr_of_mut!((*g).timestamps[next]), rust_mglru_jiffies());
        rust_mglru_store_release_ulong(addr_of_mut!((*g).max_seq), (*g).max_seq.wrapping_add(1));
        lruvec_unlock_irq(v);
        return success;
    }
}
unsafe fn try_to_inc_max_seq(v: *mut lruvec, seq: ULong, sw: i32, force: bool) -> bool {
    let state = get_mm_state(v);
    vmwarn!(25, seq > max_seq(v));
    if state.is_null() {
        return inc_max_seq(v, seq, sw);
    }
    if seq <= rd(addr_of!((*state).seq)) {
        return false;
    }
    let mut success;
    let w = if should_walk_mmu() {
        set_mm_walk(null_mut(), true)
    } else {
        null_mut()
    };
    if w.is_null() {
        success = iterate_mm_list_nowalk(v, seq);
    } else {
        (*w).lruvec = v;
        (*w).seq = seq;
        (*w).swappiness = sw;
        (*w).force_scan = force;
        let mut mm = null_mut();
        loop {
            success = iterate_mm_list(w, &mut mm);
            if mm.is_null() {
                break;
            }
            walk_mm(mm, w);
        }
    }
    if success {
        success = inc_max_seq(v, seq, sw);
        warn!(6, !success);
    }
    success
}
unsafe fn set_initial_priority(pgdat: *mut pglist_data, sc: *mut scan_control) {
    if (*sc).priority as i32 != DEF_PRIORITY as i32 || (*sc).nr_to_reclaim < MIN_LRU_BATCH as ULong
    {
        return;
    }
    let mut reclaimable = node_page_state(pgdat, NR_INACTIVE_FILE as _) as ULong;
    if can_reclaim_anon_pages(null_mut(), (*pgdat).node_id, sc) {
        reclaimable =
            reclaimable.wrapping_add(node_page_state(pgdat, NR_INACTIVE_ANON as _) as ULong);
    }
    let priority =
        rust_mglru_fls_long(reclaimable) - 1 - rust_mglru_fls_long((*sc).nr_to_reclaim - 1);
    (*sc).priority = priority.clamp(DEF_PRIORITY as i32 / 2, DEF_PRIORITY as i32) as _;
}
unsafe fn lruvec_evictable_size(v: *mut lruvec, sw: i32) -> ULong {
    let g = addr_of!((*v).lrugen);
    let max = max_seq(v);
    let min = min_seq(v);
    let mut total: ULong = 0;
    for typ in min_type(sw)..=max_type(sw) {
        let mut seq = min[typ];
        while seq <= max {
            let gen = lru_gen_from_seq(seq) as usize;
            for zone in 0..MAX_NR_ZONES as usize {
                total = total
                    .wrapping_add(
                        core::cmp::max(rd(addr_of!((*g).nr_pages[gen][typ][zone])), 0) as ULong,
                    );
            }
            seq = seq.wrapping_add(1);
        }
    }
    total
}
unsafe fn lruvec_is_sizable(v: *mut lruvec, sc: *mut scan_control) -> bool {
    let total = lruvec_evictable_size(v, get_swappiness(v, sc));
    (if mem_cgroup_online(lruvec_memcg(v)) {
        total >> (*sc).priority
    } else {
        total
    }) != 0
}
unsafe fn lruvec_is_reclaimable(v: *mut lruvec, sc: *mut scan_control, ttl: ULong) -> bool {
    let sw = get_swappiness(v, sc);
    let memcg = lruvec_memcg(v);
    let seq = min_seq(v);
    if mem_cgroup_below_min(null_mut(), memcg) || !lruvec_is_sizable(v, sc) {
        return false;
    }
    let gen = lru_gen_from_seq(evictable_min_seq(&seq, sw)) as usize;
    let birth = rd(addr_of!((*v).lrugen.timestamps[gen]));
    rust_mglru_time_is_before_jiffies(birth.wrapping_add(ttl))
}
pub(super) unsafe fn lru_gen_age_node(pgdat: *mut pglist_data, sc: *mut scan_control) {
    let ttl = rd(rust_mglru_min_ttl_ptr());
    let mut reclaimable = ttl == 0;
    vmwarn!(26, !current_is_kswapd());
    set_initial_priority(pgdat, sc);
    let mut memcg = mem_cgroup_iter(null_mut(), null_mut(), null_mut());
    loop {
        let v = mem_cgroup_lruvec(memcg, pgdat);
        mem_cgroup_calculate_protection(null_mut(), memcg);
        if !reclaimable {
            reclaimable = lruvec_is_reclaimable(v, sc, ttl);
        }
        memcg = mem_cgroup_iter(null_mut(), memcg, null_mut());
        if memcg.is_null() {
            break;
        }
    }
    if !reclaimable && mutex_trylock(addr_of_mut!(oom_lock)) {
        let mut oc: oom_control = zeroed();
        oc.gfp_mask = (*sc).gfp_mask;
        out_of_memory(&mut oc);
        mutex_unlock(addr_of_mut!(oom_lock));
    }
}
#[no_mangle]
pub unsafe extern "C" fn lru_gen_look_around(pvmw: *mut page_vma_mapped_walk, mut nr: u32) -> bool {
    let mut dirty = false;
    let mut last = null_mut();
    let mut young = nr as i32;
    let mut pte = (*pvmw).pte;
    let mut addr = (*pvmw).address;
    let vma = (*pvmw).vma;
    let mut f = rust_mglru_pfn_folio((*pvmw).pfn);
    let pgdat = folio_pgdat(f);
    rust_mglru_assert_ptl((*pvmw).ptl);
    folio_warn!(6, folio_test_lru(f), f);
    if !rust_mglru_clear_young_ptes(vma, addr, pte, nr) {
        return false;
    }
    if spin_is_contended((*pvmw).ptl) || rust_mglru_vma_special(vma) {
        return true;
    }
    let state = (*current()).reclaim_state;
    let w = if state.is_null() {
        null_mut()
    } else {
        (*state).mm_walk
    };
    let mut start = max(addr & PMD_MASK as ULong, rust_mglru_vma_start(vma));
    let mut end = min(
        addr | !(PMD_MASK as ULong),
        rust_mglru_vma_end(vma).wrapping_sub(1),
    )
    .wrapping_add(1);
    if end.wrapping_sub(start) == PAGE_SIZE as ULong {
        return true;
    }
    let window = MIN_LRU_BATCH as ULong * PAGE_SIZE as ULong;
    if end.wrapping_sub(start) > window {
        if addr.wrapping_sub(start) < window / 2 {
            end = start.wrapping_add(window);
        } else if end.wrapping_sub(addr) < window / 2 {
            start = end.wrapping_sub(window);
        } else {
            start = addr.wrapping_sub(window / 2);
            end = addr.wrapping_add(window / 2);
        }
    }
    let memcg = get_mem_cgroup_from_folio(f);
    let v = mem_cgroup_lruvec(memcg, pgdat);
    let max = max_seq(v);
    let gen = lru_gen_from_seq(max);
    let mm_state = get_mm_state(v);
    rust_mglru_lazy_mmu_mode_enable();
    pte = pte.sub(((addr.wrapping_sub(start)) / PAGE_SIZE as ULong) as usize);
    let mut i = 0i32;
    addr = start;
    while addr != end {
        let mut ptent = rust_mglru_ptep_get(pte);
        nr = 1;
        let pfn = get_pte_pfn(ptent, vma, addr, pgdat);
        if pfn != ULong::MAX {
            f = get_pfn_folio(pfn, memcg, pgdat);
            if !f.is_null() {
                if folio_test_large(f) {
                    nr = rust_mglru_folio_pte_batch(
                        f,
                        pte,
                        &mut ptent,
                        (end.wrapping_sub(addr) >> PAGE_SHIFT) as u32,
                    );
                }
                if rust_mglru_clear_young_ptes(vma, addr, pte, nr) {
                    if last != f {
                        walk_update_folio(w, vma, last, gen, dirty);
                        last = f;
                        dirty = false;
                    }
                    if rust_mglru_pte_dirty(ptent) {
                        dirty = true;
                    }
                    young = young.wrapping_add(nr as i32);
                }
            }
        }
        i += nr as i32;
        pte = pte.add(nr as usize);
        addr = addr.wrapping_add((nr as ULong).wrapping_mul(PAGE_SIZE as ULong));
    }
    walk_update_folio(w, vma, last, gen, dirty);
    rust_mglru_lazy_mmu_mode_disable();
    if !mm_state.is_null() && suitable_to_scan(i, young) {
        update_bloom_filter(mm_state, max, (*pvmw).pmd.cast());
    }
    mem_cgroup_put(memcg);
    true
}
