// SPDX-License-Identifier: GPL-2.0
/*
 * Workingset detection, translated from mm/workingset.c.
 * Copyright (C) 2013 Red Hat, Inc., Johannes Weiner
 *
 * The kernel's inline primitives retain their native implementation. This
 * unit owns eviction/refault policy, shadow tokens, list accounting, shrinking
 * and initialization, using layouts generated from the target kernel headers.
 */
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
    missing_docs, unsafe_op_in_unsafe_fn, clippy::all, unreachable_pub)]
#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/workingset_generated.rs"));
}
use bindings::*;
use core::mem::{offset_of, zeroed};
use core::ptr::{addr_of, addr_of_mut, null_mut};
use kernel::ffi::{c_int, c_long, c_uint, c_ulong, c_void};

const WORKINGSET_SHIFT: u32 = 1;
const EVICTION_SHIFT: u32 = RUST_WS_BITS_PER_LONG - RUST_WS_BITS_PER_XA_VALUE
    + WORKINGSET_SHIFT + RUST_WS_NODES_SHIFT + RUST_WS_MEM_CGROUP_ID_SHIFT;
const EVICTION_MASK: c_ulong = c_ulong::MAX >> EVICTION_SHIFT;
const _: () = assert!(RUST_WS_BITS_PER_LONG >= EVICTION_SHIFT);

#[link_section = ".data..read_mostly"]
static mut bucket_order: [c_uint; ANON_AND_FILE as usize] = [0; ANON_AND_FILE as usize];

#[inline]
unsafe fn eviction_mask(file: bool) -> c_ulong {
    if file { EVICTION_MASK }
    else { c_ulong::MAX >> (EVICTION_SHIFT + rust_ws_swap_count_shift()) }
}

unsafe fn pack_shadow(memcgid: c_int, pgdat: *mut pglist_data, mut eviction: c_ulong,
    workingset: bool, file: bool) -> *mut c_void
{
    eviction &= eviction_mask(file);
    eviction = (eviction << RUST_WS_MEM_CGROUP_ID_SHIFT) | memcgid as c_ulong;
    eviction = (eviction << RUST_WS_NODES_SHIFT) | rust_ws_node_id(pgdat) as c_ulong;
    eviction = (eviction << WORKINGSET_SHIFT) | workingset as c_ulong;
    rust_ws_xa_mk_value(eviction)
}

unsafe fn unpack_shadow(shadow: *mut c_void, memcgid: *mut c_int,
    pgdat: *mut *mut pglist_data, eviction: *mut c_ulong, workingset: *mut bool)
{
    let mut entry = rust_ws_xa_to_value(shadow);
    *workingset = entry & ((1 << WORKINGSET_SHIFT) - 1) != 0;
    entry >>= WORKINGSET_SHIFT;
    let nid = entry & ((1 << RUST_WS_NODES_SHIFT) - 1);
    entry >>= RUST_WS_NODES_SHIFT;
    *memcgid = (entry & ((1 << RUST_WS_MEM_CGROUP_ID_SHIFT) - 1)) as c_int;
    entry >>= RUST_WS_MEM_CGROUP_ID_SHIFT;
    *pgdat = rust_ws_node_data(nid as c_int);
    *eviction = entry;
}

#[cfg(CONFIG_LRU_GEN)]
unsafe fn lru_gen_eviction(folio: *mut folio) -> *mut c_void {
    let file = rust_ws_folio_is_file_lru(folio);
    let kind = file as usize;
    let delta = rust_ws_folio_nr_pages(folio) as c_int;
    let refs = rust_ws_folio_lru_refs(folio);
    let workingset = rust_ws_folio_test_workingset(folio);
    let tier = rust_ws_lru_tier_from_refs(refs, workingset) as usize;
    let pgdat = rust_ws_folio_pgdat(folio);

    rust_ws_rcu_read_lock();
    let memcg = rust_ws_folio_memcg(folio);
    let lruvec = rust_ws_mem_cgroup_lruvec(memcg, pgdat);
    let lrugen = addr_of_mut!((*lruvec).lrugen);
    let min_seq = rust_ws_read_once_ulong(addr_of!((*lrugen).min_seq[kind]));
    let token = (min_seq << rust_ws_lru_refs_width()) | refs.wrapping_sub(1).max(0) as c_ulong;
    let hist = rust_ws_lru_hist_from_seq(min_seq) as usize;
    rust_ws_atomic_long_add(delta as c_long, addr_of_mut!((*lrugen).evicted[hist][kind][tier]));
    let memcg_id = rust_ws_mem_cgroup_private_id(memcg);
    rust_ws_rcu_read_unlock();

    pack_shadow(memcg_id as c_int, pgdat, token, workingset, file)
}

#[cfg(CONFIG_LRU_GEN)]
unsafe fn lru_gen_test_recent(shadow: *mut c_void, lruvec: *mut *mut lruvec,
    token: *mut c_ulong, workingset: *mut bool, file: bool) -> bool
{
    let mut memcg_id = 0;
    let mut pgdat = null_mut();
    unpack_shadow(shadow, &mut memcg_id, &mut pgdat, token, workingset);
    let memcg = rust_ws_mem_cgroup_from_private_id(memcg_id as u16);
    *lruvec = rust_ws_mem_cgroup_lruvec(memcg, pgdat);
    let mut max_seq = rust_ws_read_once_ulong(addr_of!((**lruvec).lrugen.max_seq));
    max_seq &= eviction_mask(file) >> rust_ws_lru_refs_width();
    max_seq.abs_diff(*token >> rust_ws_lru_refs_width()) < MAX_NR_GENS as c_ulong
}

#[cfg(CONFIG_LRU_GEN)]
unsafe fn lru_gen_refault(folio: *mut folio, shadow: *mut c_void) {
    let file = rust_ws_folio_is_file_lru(folio);
    let kind = file as usize;
    let delta = rust_ws_folio_nr_pages(folio) as c_int;
    let mut lruvec = null_mut();
    let mut token = 0;
    let mut workingset = false;

    rust_ws_rcu_read_lock();
    let recent = lru_gen_test_recent(shadow, &mut lruvec, &mut token, &mut workingset, file);
    if lruvec != rust_ws_folio_lruvec(folio) {
        rust_ws_rcu_read_unlock();
        return;
    }
    rust_ws_mod_lruvec_state(lruvec, node_stat_item_WORKINGSET_REFAULT_BASE + file as u32, delta as c_long);
    if !recent {
        rust_ws_rcu_read_unlock();
        return;
    }

    let lrugen = addr_of_mut!((*lruvec).lrugen);
    let hist = rust_ws_lru_hist_from_seq(rust_ws_read_once_ulong(addr_of!((*lrugen).min_seq[kind]))) as usize;
    let refs = ((token & ((1 << rust_ws_lru_refs_width()) - 1)) + 1) as c_int;
    let tier = rust_ws_lru_tier_from_refs(refs, workingset) as usize;
    rust_ws_atomic_long_add(delta as c_long, addr_of_mut!((*lrugen).refaulted[hist][kind][tier]));

    if workingset {
        if rust_ws_lru_gen_in_fault() {
            rust_ws_mod_lruvec_state(lruvec, node_stat_item_WORKINGSET_ACTIVATE_BASE + file as u32, delta as c_long);
        }
        rust_ws_folio_set_workingset(folio);
        rust_ws_mod_lruvec_state(lruvec, node_stat_item_WORKINGSET_RESTORE_BASE + file as u32, delta as c_long);
    } else {
        rust_ws_set_mask_bits(rust_ws_folio_flags(folio), rust_ws_lru_refs_mask(),
            (refs as c_ulong).wrapping_sub(1) << rust_ws_lru_refs_pgoff());
    }
    rust_ws_rcu_read_unlock();
}

// These are the original !CONFIG_LRU_GEN stubs; lru_gen_enabled() is false.
#[cfg(not(CONFIG_LRU_GEN))]
unsafe fn lru_gen_eviction(_folio: *mut folio) -> *mut c_void { null_mut() }
#[cfg(not(CONFIG_LRU_GEN))]
unsafe fn lru_gen_test_recent(_shadow: *mut c_void, _lruvec: *mut *mut lruvec,
    _token: *mut c_ulong, _workingset: *mut bool, _file: bool) -> bool { false }
#[cfg(not(CONFIG_LRU_GEN))]
unsafe fn lru_gen_refault(_folio: *mut folio, _shadow: *mut c_void) {}

#[no_mangle]
pub unsafe extern "C" fn workingset_age_nonresident(mut lruvec: *mut lruvec, nr_pages: c_ulong) {
    loop {
        rust_ws_atomic_long_add(nr_pages as c_long, addr_of_mut!((*lruvec).nonresident_age));
        lruvec = rust_ws_parent_lruvec(lruvec);
        if lruvec.is_null() { break; }
    }
}

#[no_mangle]
pub unsafe extern "C" fn workingset_eviction(folio: *mut folio, target_memcg: *mut mem_cgroup) -> *mut c_void {
    let pgdat = rust_ws_folio_pgdat(folio);
    let file = rust_ws_folio_is_file_lru(folio);
    rust_ws_assert_eviction_folio(folio);
    if rust_ws_lru_gen_enabled() { return lru_gen_eviction(folio); }

    let lruvec = rust_ws_mem_cgroup_lruvec(target_memcg, pgdat);
    let memcgid = rust_ws_mem_cgroup_private_id(rust_ws_lruvec_memcg(lruvec));
    let mut eviction = rust_ws_atomic_long_read(addr_of!((*lruvec).nonresident_age)) as c_ulong;
    eviction >>= bucket_order[file as usize];
    workingset_age_nonresident(lruvec, rust_ws_folio_nr_pages(folio));
    pack_shadow(memcgid as c_int, pgdat, eviction, rust_ws_folio_test_workingset(folio), file)
}

#[no_mangle]
pub unsafe extern "C" fn workingset_test_recent(shadow: *mut c_void, file: bool,
    workingset: *mut bool, flush: bool) -> bool
{
    let mut lruvec = null_mut();
    let mut eviction = 0;
    if rust_ws_lru_gen_enabled() {
        rust_ws_rcu_read_lock();
        let recent = lru_gen_test_recent(shadow, &mut lruvec, &mut eviction, workingset, file);
        rust_ws_rcu_read_unlock();
        return recent;
    }

    let mut memcgid = 0;
    let mut pgdat = null_mut();
    rust_ws_rcu_read_lock();
    unpack_shadow(shadow, &mut memcgid, &mut pgdat, &mut eviction, workingset);
    eviction <<= bucket_order[file as usize];
    let mut eviction_memcg = rust_ws_mem_cgroup_from_private_id(memcgid as u16);
    if !rust_ws_mem_cgroup_tryget(eviction_memcg) { eviction_memcg = null_mut(); }
    rust_ws_rcu_read_unlock();

    if !rust_ws_mem_cgroup_disabled() && eviction_memcg.is_null() { return false; }
    // Flushing can sleep and must happen after the RCU read section.
    if flush { rust_ws_mem_cgroup_flush_stats_ratelimited(eviction_memcg); }
    lruvec = rust_ws_mem_cgroup_lruvec(eviction_memcg, pgdat);
    let refault = rust_ws_atomic_long_read(addr_of!((*lruvec).nonresident_age)) as c_ulong;
    let refault_distance = refault.wrapping_sub(eviction) & eviction_mask(file);
    let mut workingset_size = rust_ws_lruvec_page_state(lruvec, node_stat_item_NR_ACTIVE_FILE);
    if !file {
        workingset_size = workingset_size.wrapping_add(rust_ws_lruvec_page_state(lruvec, node_stat_item_NR_INACTIVE_FILE));
    }
    if rust_ws_mem_cgroup_get_nr_swap_pages(eviction_memcg) > 0 {
        workingset_size = workingset_size.wrapping_add(rust_ws_lruvec_page_state(lruvec, node_stat_item_NR_ACTIVE_ANON));
        if file {
            workingset_size = workingset_size.wrapping_add(rust_ws_lruvec_page_state(lruvec, node_stat_item_NR_INACTIVE_ANON));
        }
    }
    rust_ws_mem_cgroup_put(eviction_memcg);
    refault_distance <= workingset_size
}

#[no_mangle]
pub unsafe extern "C" fn workingset_refault(folio: *mut folio, shadow: *mut c_void) {
    let file = rust_ws_folio_is_file_lru(folio);
    rust_ws_assert_locked_folio(folio);
    if rust_ws_lru_gen_enabled() { lru_gen_refault(folio, shadow); return; }

    let nr = rust_ws_folio_nr_pages(folio) as c_long;
    let memcg = rust_ws_get_mem_cgroup_from_folio(folio);
    let lruvec = rust_ws_mem_cgroup_lruvec(memcg, rust_ws_folio_pgdat(folio));
    rust_ws_mod_lruvec_state(lruvec, node_stat_item_WORKINGSET_REFAULT_BASE + file as u32, nr);
    let mut workingset = false;
    if workingset_test_recent(shadow, file, &mut workingset, true) {
        rust_ws_folio_set_active(folio);
        workingset_age_nonresident(lruvec, nr as c_ulong);
        rust_ws_mod_lruvec_state(lruvec, node_stat_item_WORKINGSET_ACTIVATE_BASE + file as u32, nr);
        if workingset {
            rust_ws_folio_set_workingset(folio);
            rust_ws_mod_lruvec_state(lruvec, node_stat_item_WORKINGSET_RESTORE_BASE + file as u32, nr);
        }
    }
    rust_ws_mem_cgroup_put(memcg);
}

#[no_mangle]
pub unsafe extern "C" fn workingset_activation(folio: *mut folio) {
    if rust_ws_mem_cgroup_disabled() || rust_ws_folio_memcg_charged(folio) {
        rust_ws_rcu_read_lock();
        workingset_age_nonresident(rust_ws_folio_lruvec(folio), rust_ws_folio_nr_pages(folio));
        rust_ws_rcu_read_unlock();
    }
}

#[no_mangle]
pub static mut shadow_nodes: list_lru = unsafe { zeroed() };
static mut shadow_nodes_key: lock_class_key = unsafe { zeroed() };

#[no_mangle]
pub unsafe extern "C" fn workingset_update_node(node: *mut xa_node) {
    let page = rust_ws_virt_to_page(node.cast());
    rust_ws_assert_xa_locked((*node).array);
    let list = addr_of_mut!((*node).__bindgen_anon_1.private_list);
    if (*node).count != 0 && (*node).count == (*node).nr_values {
        if rust_ws_list_empty(list) {
            list_lru_add_obj(addr_of_mut!(shadow_nodes), list);
            rust_ws_inc_node_page_state(page, node_stat_item_WORKINGSET_NODES);
        }
    } else if !rust_ws_list_empty(list) {
        list_lru_del_obj(addr_of_mut!(shadow_nodes), list);
        rust_ws_dec_node_page_state(page, node_stat_item_WORKINGSET_NODES);
    }
}

unsafe extern "C" fn count_shadow_nodes(_shrinker: *mut shrinker, sc: *mut shrink_control) -> c_ulong {
    let nodes = rust_ws_list_lru_shrink_count(addr_of_mut!(shadow_nodes), sc);
    if nodes == 0 { return SHRINK_EMPTY as c_ulong; }

    let pages;
    #[cfg(CONFIG_MEMCG)]
    if !(*sc).memcg.is_null() {
        rust_ws_mem_cgroup_flush_stats_ratelimited((*sc).memcg);
        let lruvec = rust_ws_mem_cgroup_lruvec((*sc).memcg, rust_ws_node_data((*sc).nid));
        let mut count: c_ulong = 0;
        for i in 0..lru_list_NR_LRU_LISTS {
            count = count.wrapping_add(lruvec_lru_size(lruvec, i, MAX_NR_ZONES as c_int - 1));
        }
        count = count.wrapping_add(rust_ws_lruvec_page_state_local(lruvec, node_stat_item_NR_SLAB_RECLAIMABLE_B) >> RUST_WS_PAGE_SHIFT);
        count = count.wrapping_add(rust_ws_lruvec_page_state_local(lruvec, node_stat_item_NR_SLAB_UNRECLAIMABLE_B) >> RUST_WS_PAGE_SHIFT);
        pages = count;
    } else { pages = rust_ws_node_present_pages((*sc).nid); }
    #[cfg(not(CONFIG_MEMCG))]
    { pages = rust_ws_node_present_pages((*sc).nid); }

    let max_nodes = pages >> (RUST_WS_XA_CHUNK_SHIFT - 3);
    if nodes <= max_nodes { 0 } else { nodes - max_nodes }
}

// Entered through the C enum-returning declaration thunk so indirect callers
// receive precisely the compiler's list_lru_walk_cb KCFI signature.
#[no_mangle]
pub unsafe extern "C" fn rust_workingset_shadow_lru_isolate(item: *mut list_head,
    lru: *mut list_lru_one, _arg: *mut c_void) -> c_uint
{
    let node = item.cast::<u8>().sub(offset_of!(xa_node, __bindgen_anon_1)).cast::<xa_node>();
    let mapping = (*node).array.cast::<u8>().sub(offset_of!(address_space, i_pages)).cast::<address_space>();
    let xa = addr_of_mut!((*mapping).i_pages);
    let lock = addr_of_mut!((*lru).lock);
    if !rust_ws_xa_trylock(xa) {
        rust_ws_spin_unlock_irq(lock);
        rust_ws_cond_resched();
        return lru_status_LRU_RETRY;
    }
    if !(*mapping).host.is_null() && !rust_ws_inode_trylock((*mapping).host) {
        rust_ws_xa_unlock(xa);
        rust_ws_spin_unlock_irq(lock);
        rust_ws_cond_resched();
        return lru_status_LRU_RETRY;
    }
    list_lru_isolate(lru, item);
    rust_ws_dec_node_page_state(rust_ws_virt_to_page(node.cast()), node_stat_item_WORKINGSET_NODES);
    rust_ws_spin_unlock(lock);

    if !rust_ws_warn_no_values((*node).nr_values == 0)
        && !rust_ws_warn_count_mismatch((*node).count != (*node).nr_values)
    {
        xa_delete_node(node, Some(workingset_update_node));
        rust_ws_mod_lruvec_kmem_state(node.cast(), node_stat_item_WORKINGSET_NODERECLAIM, 1);
    }
    rust_ws_xa_unlock_irq(xa);
    if !(*mapping).host.is_null() {
        if rust_ws_mapping_shrinkable(mapping) { inode_lru_list_add((*mapping).host); }
        rust_ws_inode_unlock((*mapping).host);
    }
    rust_ws_cond_resched();
    lru_status_LRU_REMOVED_RETRY
}

unsafe extern "C" fn scan_shadow_nodes(_shrinker: *mut shrinker, sc: *mut shrink_control) -> c_ulong {
    rust_ws_list_lru_shrink_walk_irq(addr_of_mut!(shadow_nodes), sc,
        Some(rust_ws_shadow_lru_isolate), null_mut())
}

#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_workingset_init() -> c_int {
    let timestamp_bits = RUST_WS_BITS_PER_LONG - EVICTION_SHIFT;
    let timestamp_bits_anon = timestamp_bits - rust_ws_swap_count_shift();
    let max_order = rust_ws_fls_long(rust_ws_totalram_pages().wrapping_sub(1)) as u32;
    if max_order > timestamp_bits { bucket_order[WORKINGSET_FILE as usize] = max_order - timestamp_bits; }
    if max_order > timestamp_bits_anon { bucket_order[WORKINGSET_ANON as usize] = max_order - timestamp_bits_anon; }
    rust_ws_log_init(timestamp_bits, timestamp_bits_anon, max_order,
        bucket_order[WORKINGSET_FILE as usize], bucket_order[WORKINGSET_ANON as usize]);

    let shrinker = shrinker_alloc((RUST_WS_SHRINKER_NUMA_AWARE | RUST_WS_SHRINKER_MEMCG_AWARE) as _, c"mm-shadow".as_ptr().cast());
    if shrinker.is_null() { return -(ENOMEM as c_int); }
    let ret = rust_ws_list_lru_init(addr_of_mut!(shadow_nodes), shrinker, addr_of_mut!(shadow_nodes_key));
    if ret != 0 { shrinker_free(shrinker); return ret; }
    (*shrinker).count_objects = Some(count_shadow_nodes);
    (*shrinker).scan_objects = Some(scan_shadow_nodes);
    (*shrinker).seeks = 0;
    shrinker_register(shrinker);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
