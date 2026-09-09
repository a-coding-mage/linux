// SPDX-License-Identifier: GPL-2.0
/* Rust translation of linux/mm/swap_state.c. Kernel dependencies are external. */

// C headers and conditional compilation dependencies are supplied by the kernel build.

static mut PAGE_CLUSTER: i32 = 0;
const PAGE_CLUSTER_MAX: i32 = 31;

// swapper_space is a fiction, retained to simplify the path through vmscan's shrink_folio_list.
static mut SWAP_AOPS: address_space_operations = address_space_operations {
    dirty_folio: noop_dirty_folio,
    #[cfg(feature = "CONFIG_MIGRATION")]
    migrate_folio: migrate_folio,
};
static mut swap_space: address_space = address_space { a_ops: unsafe { &SWAP_AOPS } };
static mut enable_vma_readahead: bool = true;

const SWAP_RA_ORDER_CEILING: i32 = 5;
const SWAP_RA_WIN_SHIFT: usize = PAGE_SHIFT / 2;
const SWAP_RA_HITS_MASK: usize = (1usize << SWAP_RA_WIN_SHIFT) - 1;
const SWAP_RA_HITS_MAX: usize = SWAP_RA_HITS_MASK;
const SWAP_RA_WIN_MASK: usize = (!PAGE_MASK) & (!SWAP_RA_HITS_MASK);

#[inline] unsafe fn swap_ra_hits(v: usize) -> usize { v & SWAP_RA_HITS_MASK }
#[inline] unsafe fn swap_ra_win(v: usize) -> usize { (v & SWAP_RA_WIN_MASK) >> SWAP_RA_WIN_SHIFT }
#[inline] unsafe fn swap_ra_addr(v: usize) -> usize { v & PAGE_MASK }
#[inline] unsafe fn swap_ra_val(addr: usize, win: usize, hits: usize) -> usize {
    (addr & PAGE_MASK) | ((win << SWAP_RA_WIN_SHIFT) & SWAP_RA_WIN_MASK) | (hits & SWAP_RA_HITS_MASK)
}

static mut swapin_readahead_hits: atomic_t = ATOMIC_INIT(4);

pub unsafe fn show_swap_cache_info() {
    printk!("%lu pages in swap cache\\n", total_swapcache_pages());
    printk!("Free swap  = %ldkB\\n", K(get_nr_swap_pages()));
    printk!("Total swap = %lukB\\n", K(total_swap_pages));
}

pub unsafe fn swap_cache_get_folio(entry: swp_entry_t) -> *mut folio {
    loop {
        let swp_tb = swap_table_get(__swap_entry_to_cluster(entry), swp_cluster_offset(entry));
        if !swp_tb_is_folio(swp_tb) { return core::ptr::null_mut(); }
        let folio = swp_tb_to_folio(swp_tb);
        if folio_try_get(folio) { return folio; }
    }
}

pub unsafe fn swap_cache_has_folio(entry: swp_entry_t) -> bool {
    swp_tb_is_folio(swap_table_get(__swap_entry_to_cluster(entry), swp_cluster_offset(entry)))
}

pub unsafe fn swap_cache_get_shadow(entry: swp_entry_t) -> *mut core::ffi::c_void {
    let tb = swap_table_get(__swap_entry_to_cluster(entry), swp_cluster_offset(entry));
    if swp_tb_is_shadow(tb) { swp_tb_to_shadow(tb) } else { core::ptr::null_mut() }
}

unsafe fn __swap_cache_add_check(ci: *mut swap_cluster_info, targ_entry: swp_entry_t,
    nr: usize, shadowp: *mut *mut core::ffi::c_void, memcg_id: *mut u16) -> i32 {
    lockdep_assert_held(&(*ci).lock);
    if (*ci).table.is_null() { return -ENOENT; }
    let mut ci_off = swp_cluster_offset(targ_entry);
    let mut old_tb = __swap_table_get(ci, ci_off);
    if swp_tb_is_folio(old_tb) { return -EEXIST; }
    if __swp_tb_get_count(old_tb) == 0 { return -ENOENT; }
    if !shadowp.is_null() && swp_tb_is_shadow(old_tb) { *shadowp = swp_tb_to_shadow(old_tb); }
    if !memcg_id.is_null() { *memcg_id = __swap_cgroup_get(ci, ci_off); }
    if nr == 1 { return 0; }
    let is_zero = __swap_table_test_zero(ci, ci_off);
    ci_off = round_down(ci_off, nr);
    let ci_end = ci_off + nr;
    loop {
        old_tb = __swap_table_get(ci, ci_off);
        if swp_tb_is_folio(old_tb) || __swp_tb_get_count(old_tb) == 0 ||
            is_zero != __swap_table_test_zero(ci, ci_off) ||
            (!memcg_id.is_null() && *memcg_id != __swap_cgroup_get(ci, ci_off)) { return -EBUSY; }
        ci_off += 1;
        if ci_off >= ci_end { break; }
    }
    0
}

unsafe fn __swap_cache_do_add_folio(ci: *mut swap_cluster_info, folio: *mut folio, entry: swp_entry_t) {
    let mut ci_off = swp_cluster_offset(entry);
    let ci_end = ci_off + folio_nr_pages(folio);
    let pfn = folio_pfn(folio);
    VM_WARN_ON_ONCE_FOLIO(!folio_test_locked(folio), folio);
    VM_WARN_ON_ONCE_FOLIO(folio_test_swapcache(folio), folio);
    VM_WARN_ON_ONCE_FOLIO(!folio_test_swapbacked(folio), folio);
    loop {
        let old_tb = __swap_table_get(ci, ci_off);
        VM_WARN_ON_ONCE(swp_tb_is_folio(old_tb));
        __swap_table_set(ci, ci_off, pfn_to_swp_tb(pfn, __swp_tb_get_flags(old_tb)));
        ci_off += 1;
        if ci_off >= ci_end { break; }
    }
    folio_ref_add(folio, folio_nr_pages(folio));
    folio_set_swapcache(folio);
    (*folio).swap = entry;
}

pub unsafe fn __swap_cache_add_folio(ci: *mut swap_cluster_info, folio: *mut folio, entry: swp_entry_t) {
    let nr = folio_nr_pages(folio);
    __swap_cache_do_add_folio(ci, folio, entry);
    node_stat_mod_folio(folio, NR_FILE_PAGES, nr); lruvec_stat_mod_folio(folio, NR_SWAPCACHE, nr);
}

unsafe fn __swap_cache_do_del_folio(ci: *mut swap_cluster_info, folio: *mut folio,
    entry: swp_entry_t, shadow: *mut core::ffi::c_void) {
    let si = __swap_entry_to_info(entry);
    let ci_start = swp_cluster_offset(entry);
    let nr = folio_nr_pages(folio);
    let ci_end = ci_start + nr;
    let mut ci_off = ci_start;
    let mut folio_swapped = false;
    let mut need_free = false;
    VM_WARN_ON_ONCE(__swap_entry_to_cluster(entry) != ci);
    VM_WARN_ON_ONCE_FOLIO(!folio_test_locked(folio), folio);
    VM_WARN_ON_ONCE_FOLIO(!folio_test_swapcache(folio), folio);
    VM_WARN_ON_ONCE_FOLIO(folio_test_writeback(folio), folio);
    loop {
        let old_tb = __swap_table_get(ci, ci_off);
        WARN_ON_ONCE(!swp_tb_is_folio(old_tb) || swp_tb_to_folio(old_tb) != folio);
        if __swp_tb_get_count(old_tb) != 0 { folio_swapped = true; } else { need_free = true; }
        __swap_table_set(ci, ci_off, shadow_to_swp_tb(shadow, __swp_tb_get_flags(old_tb)));
        ci_off += 1; if ci_off >= ci_end { break; }
    }
    (*folio).swap.val = 0; folio_clear_swapcache(folio);
    if !folio_swapped { __swap_cluster_free_entries(si, ci, ci_start, nr); }
    else if need_free {
        ci_off = ci_start;
        loop { if __swp_tb_get_count(__swap_table_get(ci, ci_off)) == 0 { __swap_cluster_free_entries(si, ci, ci_off, 1); }
            ci_off += 1; if ci_off >= ci_end { break; } }
    }
}

pub unsafe fn __swap_cache_del_folio(ci: *mut swap_cluster_info, folio: *mut folio, entry: swp_entry_t, shadow: *mut core::ffi::c_void) {
    let nr = folio_nr_pages(folio); __swap_cache_do_del_folio(ci, folio, entry, shadow);
    node_stat_mod_folio(folio, NR_FILE_PAGES, -(nr as isize)); lruvec_stat_mod_folio(folio, NR_SWAPCACHE, -(nr as isize));
}

pub unsafe fn swap_cache_del_folio(folio: *mut folio) {
    let entry = (*folio).swap; let ci = swap_cluster_lock(__swap_entry_to_info(entry), swp_offset(entry));
    __swap_cache_del_folio(ci, folio, entry, core::ptr::null_mut()); swap_cluster_unlock(ci); folio_ref_sub(folio, folio_nr_pages(folio));
}

pub unsafe fn __swap_cache_replace_folio(ci: *mut swap_cluster_info, old: *mut folio, new: *mut folio) {
    let entry = (*new).swap; let nr = folio_nr_pages(new); let mut off = swp_cluster_offset(entry); let end = off + nr; let pfn = folio_pfn(new);
    VM_WARN_ON_ONCE(!folio_test_swapcache(old) || !folio_test_swapcache(new)); VM_WARN_ON_ONCE(!folio_test_locked(old) || !folio_test_locked(new)); VM_WARN_ON_ONCE(!entry.val);
    while off < end { let tb = __swap_table_get(ci, off); WARN_ON_ONCE(!swp_tb_is_folio(tb) || swp_tb_to_folio(tb) != old); __swap_table_set(ci, off, pfn_to_swp_tb(pfn, __swp_tb_get_flags(tb))); off += 1; }
    if IS_ENABLED(CONFIG_DEBUG_VM) && folio_order(old) != folio_order(new) { off = swp_cluster_offset((*old).swap); let e = off + folio_nr_pages(old); while off < e { off += 1; WARN_ON_ONCE(swp_tb_to_folio(__swap_table_get(ci, off)) != old); } }
}

unsafe fn __swap_cache_alloc(ci: *mut swap_cluster_info, targ: swp_entry_t, gfp: gfp_t, order: u32, vmf: *mut vm_fault, mpol: *mut mempolicy, ilx: pgoff_t) -> *mut folio {
    let nr = 1usize << order; let mut shadow = core::ptr::null_mut(); let mut memcg: u16 = 0;
    let mut entry = targ; entry.val = round_down(targ.val, nr);
    spin_lock(&mut (*ci).lock); let mut err = __swap_cache_add_check(ci, targ, nr, core::ptr::null_mut(), core::ptr::null_mut()); spin_unlock(&mut (*ci).lock);
    if err != 0 { return ERR_PTR(err); }
    let folio = if !mpol.is_null() || vmf.is_null() { folio_alloc_mpol(gfp, order, mpol, ilx, numa_node_id()) } else { vma_alloc_folio(gfp, order, (*vmf).vma, round_down((*vmf).address, PAGE_SIZE << order)) };
    if folio.is_null() { return ERR_PTR(-ENOMEM); }
    spin_lock(&mut (*ci).lock); err = __swap_cache_add_check(ci, targ, nr, &mut shadow, &mut memcg);
    if err != 0 { spin_unlock(&mut (*ci).lock); folio_put(folio); return ERR_PTR(err); }
    __folio_set_locked(folio); __folio_set_swapbacked(folio); __swap_cache_do_add_folio(ci, folio, entry); spin_unlock(&mut (*ci).lock);
    if mem_cgroup_swapin_charge_folio(folio, memcg, if vmf.is_null() { core::ptr::null_mut() } else { (*vmf).vma.vm_mm }, gfp) {
        spin_lock(&mut (*ci).lock); __swap_cache_do_del_folio(ci, folio, entry, shadow); spin_unlock(&mut (*ci).lock); folio_unlock(folio); folio_put_refs(folio, nr + 1); return ERR_PTR(-ENOMEM);
    }
    memcg1_swapin(folio); if !shadow.is_null() { workingset_refault(folio, shadow); }
    node_stat_mod_folio(folio, NR_FILE_PAGES, nr); lruvec_stat_mod_folio(folio, NR_SWAPCACHE, nr); folio_add_lru(folio); folio
}

pub unsafe fn swap_cache_alloc_folio(targ: swp_entry_t, gfp: gfp_t, orders: usize, vmf: *mut vm_fault, mpol: *mut mempolicy, ilx: pgoff_t) -> *mut folio {
    let ci = __swap_entry_to_cluster(targ); let mut order = highest_order(orders); let mut ret;
    if WARN_ON_ONCE(orders == 0 || (1usize << order) > SWAPFILE_CLUSTER) { return ERR_PTR(-EINVAL); }
    loop { ret = __swap_cache_alloc(ci, targ, gfp, order, vmf, mpol, ilx); if !IS_ERR(ret) { break; } let e = PTR_ERR(ret); if order == 0 || (e != -EBUSY && e != -ENOMEM) { break; } count_mthp_stat(order, MTHP_STAT_SWPIN_FALLBACK); order = next_order(&mut (orders as usize), order); }
    ret
}

pub unsafe fn free_pages_and_swap_cache(pages: *mut *mut encoded_page, nr: i32) {
    let mut folios = core::mem::MaybeUninit::<folio_batch>::zeroed().assume_init(); let mut refs = [0u32; FOLIO_BATCH_SIZE]; folio_batch_init(&mut folios);
    let mut i = 0; while i < nr { let page = page_folio(encoded_page_ptr(*pages.add(i as usize))); free_swap_cache(page); refs[folios.nr] = 1; if encoded_page_flags(*pages.add(i as usize)) & ENCODED_PAGE_BIT_NR_PAGES_NEXT != 0 { i += 1; refs[folios.nr] = encoded_nr_pages(*pages.add(i as usize)); } if folio_batch_add(&mut folios, page) == 0 { folios_put_refs(&mut folios, refs.as_mut_ptr()); } i += 1; } if folios.nr != 0 { folios_put_refs(&mut folios, refs.as_mut_ptr()); }
}

pub unsafe fn swap_update_readahead(folio: *mut folio, vma: *mut vm_area_struct, addr: usize) {
    if folio_test_large(folio) { return; } let ra = folio_test_clear_readahead(folio); if !vma.is_null() && swap_use_vma_readahead() { let old = GET_SWAP_RA_VAL(vma); let w = swap_ra_win(old); let mut h = swap_ra_hits(old); if ra { h = core::cmp::min(h + 1, SWAP_RA_HITS_MAX); } atomic_long_set(&mut (*vma).swap_readahead_info, swap_ra_val(addr, w, h)); } if ra { count_vm_event(SWAP_RA_HIT); if vma.is_null() || !swap_use_vma_readahead() { atomic_inc(&mut swapin_readahead_hits); } }
}

pub unsafe fn swap_init() -> i32 { swap_readahead_setup(); swap_sysfs_init() }
unsafe fn swap_readahead_setup() { let megs = PAGES_TO_MB(totalram_pages()); PAGE_CLUSTER = if megs < 16 { 2 } else { 3 }; register_sysctl_init("vm", swap_readahead_sysctl_table); }
unsafe fn swap_sysfs_init() -> i32 { 0 }
pub unsafe fn free_swap_cache(folio: *mut folio) { if folio_test_swapcache(folio) && !folio_mapped(folio) && folio_trylock(folio) { folio_free_swap(folio); folio_unlock(folio); } }
pub unsafe fn free_folio_and_swap_cache(folio: *mut folio) { free_swap_cache(folio); if !is_huge_zero_folio(folio) { folio_put(folio); } }
pub unsafe fn swap_use_vma_readahead() -> bool { READ_ONCE(enable_vma_readahead) && !atomic_read(&nr_rotate_swap) }

// The following declarations preserve the externally visible interfaces from the source.
extern "C" {
    fn swap_cache_alloc_folio(targ_entry: swp_entry_t, gfp: gfp_t, orders: usize, vmf: *mut vm_fault, mpol: *mut mempolicy, ilx: pgoff_t) -> *mut folio;
    fn read_swap_cache_async(ctx: *mut swap_io_ctx, entry: swp_entry_t, gfp_mask: gfp_t, vma: *mut vm_area_struct, addr: usize) -> *mut folio;
    fn swapin_sync(entry: swp_entry_t, gfp: gfp_t, orders: usize, vmf: *mut vm_fault, mpol: *mut mempolicy, ilx: pgoff_t) -> *mut folio;
    fn swap_cluster_readahead(entry: swp_entry_t, gfp: gfp_t, mpol: *mut mempolicy, ilx: pgoff_t) -> *mut folio;
    fn swapin_readahead(entry: swp_entry_t, gfp: gfp_t, vmf: *mut vm_fault) -> *mut folio;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
