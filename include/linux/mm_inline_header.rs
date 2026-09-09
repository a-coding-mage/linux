/* SPDX-License-Identifier: GPL-2.0 */

/* C header dependencies are supplied by the surrounding kernel translation. */

pub unsafe fn folio_is_file_lru(folio: *const folio) -> c_int {
    (!folio_test_swapbacked(folio)) as c_int
}

pub unsafe fn __update_lru_size(lruvec: *mut lruvec, lru: lru_list, zid: zone_type, nr_pages: c_long) {
    let pgdat = lruvec_pgdat(lruvec);
    lockdep_assert_held((*lruvec).lru_lock);
    WARN_ON_ONCE(nr_pages != nr_pages as c_int as c_long);
    mod_lruvec_state(lruvec, NR_LRU_BASE + lru as c_int, nr_pages);
    __mod_zone_page_state(&mut (*pgdat).node_zones[zid as usize], NR_ZONE_LRU_BASE + lru as c_int, nr_pages);
}

pub unsafe fn update_lru_size(lruvec: *mut lruvec, lru: lru_list, zid: zone_type, nr_pages: c_long) {
    __update_lru_size(lruvec, lru, zid, nr_pages);
    #[cfg(CONFIG_MEMCG)]
    mem_cgroup_update_lru_size(lruvec, lru, zid, nr_pages);
}

pub unsafe fn __folio_clear_lru_flags(folio: *mut folio) {
    VM_BUG_ON_FOLIO(!folio_test_lru(folio), folio);
    __folio_clear_lru(folio);
    /* this shouldn't happen, so leave the flags to bad_page() */
    if folio_test_active(folio) && folio_test_unevictable(folio) { return; }
    __folio_clear_active(folio);
    __folio_clear_unevictable(folio);
}

pub unsafe fn folio_lru_list(folio: *const folio) -> lru_list {
    VM_BUG_ON_FOLIO(folio_test_active(folio) && folio_test_unevictable(folio), folio);
    if folio_test_unevictable(folio) { return LRU_UNEVICTABLE; }
    let mut lru = if folio_is_file_lru(folio) != 0 { LRU_INACTIVE_FILE } else { LRU_INACTIVE_ANON };
    if folio_test_active(folio) { lru = (lru as c_int + LRU_ACTIVE) as lru_list; }
    lru
}

#[cfg(CONFIG_LRU_GEN)]
pub unsafe fn lru_gen_switching() -> bool { DECLARE_STATIC_KEY_FALSE!(lru_switch); static_branch_unlikely(&lru_switch) }
#[cfg(all(CONFIG_LRU_GEN, CONFIG_LRU_GEN_ENABLED))]
pub unsafe fn lru_gen_enabled() -> bool { DECLARE_STATIC_KEY_TRUE!(lru_gen_caps[NR_LRU_GEN_CAPS]); static_branch_likely(&lru_gen_caps[LRU_GEN_CORE]) }
#[cfg(all(CONFIG_LRU_GEN, not(CONFIG_LRU_GEN_ENABLED)))]
pub unsafe fn lru_gen_enabled() -> bool { DECLARE_STATIC_KEY_FALSE!(lru_gen_caps[NR_LRU_GEN_CAPS]); static_branch_unlikely(&lru_gen_caps[LRU_GEN_CORE]) }
#[cfg(CONFIG_LRU_GEN)]
pub unsafe fn lru_gen_in_fault() -> bool { (*current).in_lru_fault }
#[cfg(CONFIG_LRU_GEN)]
pub fn lru_gen_from_seq(seq: c_ulong) -> c_int { (seq % MAX_NR_GENS as c_ulong) as c_int }
#[cfg(CONFIG_LRU_GEN)]
pub fn lru_hist_from_seq(seq: c_ulong) -> c_int { (seq % NR_HIST_GENS as c_ulong) as c_int }
#[cfg(CONFIG_LRU_GEN)]
pub fn lru_tier_from_refs(refs: c_int, workingset: bool) -> c_int {
    VM_WARN_ON_ONCE(refs > BIT(LRU_REFS_WIDTH));
    if workingset { MAX_NR_TIERS - 1 } else { order_base_2(refs) }
}
#[cfg(CONFIG_LRU_GEN)]
pub unsafe fn folio_lru_refs(folio: *const folio) -> c_int {
    let flags = READ_ONCE((*folio).flags.f);
    if flags & BIT(PG_referenced) == 0 { return 0; }
    ((flags & LRU_REFS_MASK) >> LRU_REFS_PGOFF) as c_int + 1
}
#[cfg(CONFIG_LRU_GEN)]
pub unsafe fn folio_lru_gen(folio: *const folio) -> c_int {
    let flags = READ_ONCE((*folio).flags.f);
    ((flags & LRU_GEN_MASK) >> LRU_GEN_PGOFF) as c_int - 1
}

#[cfg(CONFIG_LRU_GEN)]
pub unsafe fn lru_gen_is_active(lruvec: *const lruvec, gen: c_int) -> bool {
    let max_seq = (*lruvec).lrugen.max_seq;
    VM_WARN_ON_ONCE(gen >= MAX_NR_GENS);
    gen == lru_gen_from_seq(max_seq) || gen == lru_gen_from_seq(max_seq - 1)
}

#[cfg(CONFIG_LRU_GEN)]
pub unsafe fn lru_gen_update_size(lruvec: *mut lruvec, folio: *mut folio, old_gen: c_int, new_gen: c_int) {
    let typ = folio_is_file_lru(folio) as usize;
    let zone = folio_zonenum(folio) as usize;
    let delta = folio_nr_pages(folio) as c_long;
    let mut lru = (typ as c_int * LRU_INACTIVE_FILE) as lru_list;
    let lrugen = &mut (*lruvec).lrugen;
    VM_WARN_ON_ONCE(old_gen != -1 && old_gen >= MAX_NR_GENS);
    VM_WARN_ON_ONCE(new_gen != -1 && new_gen >= MAX_NR_GENS);
    VM_WARN_ON_ONCE(old_gen == -1 && new_gen == -1);
    if old_gen >= 0 { WRITE_ONCE(lrugen.nr_pages[old_gen as usize][typ][zone], lrugen.nr_pages[old_gen as usize][typ][zone].wrapping_sub(delta as _)); }
    if new_gen >= 0 { WRITE_ONCE(lrugen.nr_pages[new_gen as usize][typ][zone], lrugen.nr_pages[new_gen as usize][typ][zone].wrapping_add(delta as _)); }
    if old_gen < 0 { if lru_gen_is_active(lruvec, new_gen) { lru = (lru as c_int + LRU_ACTIVE) as lru_list; } __update_lru_size(lruvec, lru, zone_type_from_usize(zone), delta); return; }
    if new_gen < 0 { if lru_gen_is_active(lruvec, old_gen) { lru = (lru as c_int + LRU_ACTIVE) as lru_list; } __update_lru_size(lruvec, lru, zone_type_from_usize(zone), -delta); return; }
    if !lru_gen_is_active(lruvec, old_gen) && lru_gen_is_active(lruvec, new_gen) {
        __update_lru_size(lruvec, lru, zone_type_from_usize(zone), -delta);
        __update_lru_size(lruvec, (lru as c_int + LRU_ACTIVE) as lru_list, zone_type_from_usize(zone), delta);
    }
    VM_WARN_ON_ONCE(lru_gen_is_active(lruvec, old_gen) && !lru_gen_is_active(lruvec, new_gen));
}

#[cfg(CONFIG_LRU_GEN)]
pub unsafe fn lru_gen_folio_seq(lruvec: *const lruvec, folio: *const folio, reclaiming: bool) -> c_ulong {
    let gen: c_int;
    let typ = folio_is_file_lru(folio) as usize;
    let lrugen = &(*lruvec).lrugen;
    if folio_test_active(folio) { gen = MIN_NR_GENS - folio_test_workingset(folio) as c_int; }
    else if reclaiming { gen = MAX_NR_GENS; }
    else if ((!folio_is_file_lru(folio) != 0 && !folio_test_swapcache(folio)) ||
             (folio_test_reclaim(folio) && (folio_test_dirty(folio) || folio_test_writeback(folio)))) { gen = MIN_NR_GENS; }
    else { gen = MAX_NR_GENS - (folio_test_workingset(folio) || folio_test_referenced(folio)) as c_int; }
    core::cmp::max(READ_ONCE(lrugen.max_seq).wrapping_sub(gen as c_ulong).wrapping_add(1), READ_ONCE(lrugen.min_seq[typ]))
}

#[cfg(CONFIG_LRU_GEN)]
pub unsafe fn lru_gen_add_folio(lruvec: *mut lruvec, folio: *mut folio, reclaiming: bool) -> bool {
    let gen = folio_lru_gen(folio);
    let typ = folio_is_file_lru(folio) as usize;
    let zone = folio_zonenum(folio) as usize;
    let lrugen = &mut (*lruvec).lrugen;
    VM_WARN_ON_ONCE_FOLIO(gen != -1, folio);
    if folio_test_unevictable(folio) || !lrugen.enabled { return false; }
    let seq = lru_gen_folio_seq(lruvec, folio, reclaiming);
    let gen = lru_gen_from_seq(seq);
    let flags = ((gen as c_ulong + 1) << LRU_GEN_PGOFF) as _;
    set_mask_bits(&mut (*folio).flags.f, LRU_GEN_MASK | BIT(PG_active), flags);
    lru_gen_update_size(lruvec, folio, -1, gen);
    if reclaiming { list_add_tail(&mut (*folio).lru, &mut lrugen.folios[gen as usize][typ][zone]); }
    else { list_add(&mut (*folio).lru, &mut lrugen.folios[gen as usize][typ][zone]); }
    true
}

#[cfg(CONFIG_LRU_GEN)]
pub unsafe fn lru_gen_del_folio(lruvec: *mut lruvec, folio: *mut folio, reclaiming: bool) -> bool {
    let mut gen = folio_lru_gen(folio);
    if gen < 0 { return false; }
    VM_WARN_ON_ONCE_FOLIO(folio_test_active(folio), folio);
    VM_WARN_ON_ONCE_FOLIO(folio_test_unevictable(folio), folio);
    let flags = if !reclaiming && lru_gen_is_active(lruvec, gen) { BIT(PG_active) } else { 0 };
    let flags = set_mask_bits(&mut (*folio).flags.f, LRU_GEN_MASK, flags);
    gen = ((flags & LRU_GEN_MASK) >> LRU_GEN_PGOFF) as c_int - 1;
    lru_gen_update_size(lruvec, folio, gen, -1);
    list_del(&mut (*folio).lru);
    true
}

#[cfg(CONFIG_LRU_GEN)]
pub unsafe fn folio_migrate_refs(new: *mut folio, old: *const folio) {
    let refs = READ_ONCE((*old).flags.f) & LRU_REFS_MASK;
    set_mask_bits(&mut (*new).flags.f, LRU_REFS_MASK, refs);
}

#[cfg(not(CONFIG_LRU_GEN))]
pub fn lru_gen_enabled() -> bool { false }
#[cfg(not(CONFIG_LRU_GEN))]
pub fn lru_gen_switching() -> bool { false }
#[cfg(not(CONFIG_LRU_GEN))]
pub fn lru_gen_in_fault() -> bool { false }
#[cfg(not(CONFIG_LRU_GEN))]
pub unsafe fn lru_gen_add_folio(_: *mut lruvec, _: *mut folio, _: bool) -> bool { false }
#[cfg(not(CONFIG_LRU_GEN))]
pub unsafe fn lru_gen_del_folio(_: *mut lruvec, _: *mut folio, _: bool) -> bool { false }
#[cfg(not(CONFIG_LRU_GEN))]
pub unsafe fn folio_migrate_refs(_: *mut folio, _: *const folio) {}

pub unsafe fn lruvec_add_folio(lruvec: *mut lruvec, folio: *mut folio) {
    let lru = folio_lru_list(folio);
    VM_WARN_ON_ONCE_FOLIO(!folio_matches_lruvec(folio, lruvec), folio);
    if lru_gen_add_folio(lruvec, folio, false) { return; }
    update_lru_size(lruvec, lru, folio_zonenum(folio), folio_nr_pages(folio));
    if lru != LRU_UNEVICTABLE { list_add(&mut (*folio).lru, &mut (*lruvec).lists[lru as usize]); }
}

pub unsafe fn lruvec_add_folio_tail(lruvec: *mut lruvec, folio: *mut folio) {
    let lru = folio_lru_list(folio);
    VM_WARN_ON_ONCE_FOLIO(!folio_matches_lruvec(folio, lruvec), folio);
    if lru_gen_add_folio(lruvec, folio, true) { return; }
    update_lru_size(lruvec, lru, folio_zonenum(folio), folio_nr_pages(folio));
    list_add_tail(&mut (*folio).lru, &mut (*lruvec).lists[lru as usize]);
}

pub unsafe fn lruvec_del_folio(lruvec: *mut lruvec, folio: *mut folio) {
    let lru = folio_lru_list(folio);
    VM_WARN_ON_ONCE_FOLIO(!folio_matches_lruvec(folio, lruvec), folio);
    if lru_gen_del_folio(lruvec, folio, false) { return; }
    if lru != LRU_UNEVICTABLE { list_del(&mut (*folio).lru); }
    update_lru_size(lruvec, lru, folio_zonenum(folio), -(folio_nr_pages(folio) as c_long));
}

#[cfg(CONFIG_ANON_VMA_NAME)]
pub unsafe fn anon_vma_name_get(anon_name: *mut anon_vma_name) { if !anon_name.is_null() { kref_get(&mut (*anon_name).kref); } }
#[cfg(CONFIG_ANON_VMA_NAME)]
pub unsafe fn anon_vma_name_put(anon_name: *mut anon_vma_name) { if !anon_name.is_null() { kref_put(&mut (*anon_name).kref, anon_vma_name_free); } }
#[cfg(CONFIG_ANON_VMA_NAME)]
pub unsafe fn anon_vma_name_reuse(anon_name: *mut anon_vma_name) -> *mut anon_vma_name {
    if kref_read(&(*anon_name).kref) < REFCOUNT_MAX { anon_vma_name_get(anon_name); anon_name } else { anon_vma_name_alloc((*anon_name).name) }
}
#[cfg(CONFIG_ANON_VMA_NAME)]
pub unsafe fn dup_anon_vma_name(orig_vma: *mut vm_area_struct, new_vma: *mut vm_area_struct) { let n = anon_vma_name(orig_vma); if !n.is_null() { (*new_vma).anon_name = anon_vma_name_reuse(n); } }
#[cfg(CONFIG_ANON_VMA_NAME)]
pub unsafe fn free_anon_vma_name(vma: *mut vm_area_struct) { anon_vma_name_put((*vma).anon_name); }
#[cfg(CONFIG_ANON_VMA_NAME)]
pub unsafe fn anon_vma_name_eq(a: *mut anon_vma_name, b: *mut anon_vma_name) -> bool { a == b || (!a.is_null() && !b.is_null() && !strcmp((*a).name, (*b).name)) }
#[cfg(not(CONFIG_ANON_VMA_NAME))]
pub unsafe fn anon_vma_name_get(_: *mut anon_vma_name) {}
#[cfg(not(CONFIG_ANON_VMA_NAME))]
pub unsafe fn anon_vma_name_put(_: *mut anon_vma_name) {}
#[cfg(not(CONFIG_ANON_VMA_NAME))]
pub unsafe fn dup_anon_vma_name(_: *mut vm_area_struct, _: *mut vm_area_struct) {}
#[cfg(not(CONFIG_ANON_VMA_NAME))]
pub unsafe fn free_anon_vma_name(_: *mut vm_area_struct) {}
#[cfg(not(CONFIG_ANON_VMA_NAME))]
pub unsafe fn anon_vma_name_eq(_: *mut anon_vma_name, _: *mut anon_vma_name) -> bool { true }

extern "C" { pub fn pfnmap_track_ctx_release(ref_: *mut kref); }

pub unsafe fn init_tlb_flush_pending(mm: *mut mm_struct) { atomic_set(&mut (*mm).tlb_flush_pending, 0); }
pub unsafe fn inc_tlb_flush_pending(mm: *mut mm_struct) { atomic_inc(&mut (*mm).tlb_flush_pending); }
pub unsafe fn dec_tlb_flush_pending(mm: *mut mm_struct) { atomic_dec(&mut (*mm).tlb_flush_pending); }
pub unsafe fn mm_tlb_flush_pending(mm: *const mm_struct) -> bool { atomic_read(&(*mm).tlb_flush_pending) != 0 }
pub unsafe fn mm_tlb_flush_nested(mm: *const mm_struct) -> bool { atomic_read(&(*mm).tlb_flush_pending) > 1 }

#[cfg(CONFIG_MMU)]
pub unsafe fn copy_pte_marker(entry: softleaf_t, dst_vma: *mut vm_area_struct) -> pte_marker {
    let srcm = softleaf_to_marker(entry);
    let mut dstm = srcm & (PTE_MARKER_POISONED | PTE_MARKER_GUARD);
    if srcm & PTE_MARKER_UFFD_WP != 0 && userfaultfd_wp(dst_vma) { dstm |= PTE_MARKER_UFFD_WP; }
    dstm
}
#[cfg(CONFIG_MMU)]
pub unsafe fn vma_has_recency(vma: *const vm_area_struct) -> bool {
    if (*vma).vm_flags & (VM_SEQ_READ | VM_RAND_READ) != 0 { return false; }
    if !(*vma).vm_file.is_null() && (*(*vma).vm_file).f_mode & FMODE_NOREUSE != 0 { return false; }
    true
}

pub unsafe fn num_pages_contiguous(pages: *mut *mut page, nr_pages: usize) -> usize {
    let mut cur_page = *pages;
    let section = memdesc_section(&(*cur_page).flags);
    let mut i = 1;
    while i < nr_pages {
        cur_page = cur_page.add(1);
        if cur_page != *pages.add(i) { break; }
        if memdesc_section(&(*cur_page).flags) != section { break; }
        i += 1;
    }
    i
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
