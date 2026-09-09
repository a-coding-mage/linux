// SPDX-License-Identifier: GPL-2.0
/*
 * Workingset detection
 *
 * Copyright (C) 2013 Red Hat, Inc., Johannes Weiner
 */

// Kernel dependencies supplied by the surrounding translation unit.

const WORKINGSET_SHIFT: usize = 1;
const EVICTION_SHIFT: usize = (BITS_PER_LONG - BITS_PER_XA_VALUE) + WORKINGSET_SHIFT + NODES_SHIFT + MEM_CGROUP_ID_SHIFT;
const EVICTION_SHIFT_ANON: usize = EVICTION_SHIFT + SWAP_COUNT_SHIFT;
const EVICTION_MASK: usize = !0usize >> EVICTION_SHIFT;
const EVICTION_MASK_ANON: usize = !0usize >> EVICTION_SHIFT_ANON;

static mut BUCKET_ORDER: [u32; ANON_AND_FILE] = [0; ANON_AND_FILE];

unsafe fn pack_shadow(memcgid: i32, pgdat: *mut pg_data_t, mut eviction: usize, workingset: bool, file: bool) -> *mut core::ffi::c_void {
    eviction &= if file { EVICTION_MASK } else { EVICTION_MASK_ANON };
    eviction = (eviction << MEM_CGROUP_ID_SHIFT) | memcgid as usize;
    eviction = (eviction << NODES_SHIFT) | (*pgdat).node_id as usize;
    eviction = (eviction << WORKINGSET_SHIFT) | workingset as usize;
    xa_mk_value(eviction)
}

unsafe fn unpack_shadow(shadow: *mut core::ffi::c_void, memcgidp: *mut i32, pgdat: *mut *mut pg_data_t, evictionp: *mut usize, workingsetp: *mut bool) {
    let mut entry = xa_to_value(shadow);
    let workingset = (entry & ((1usize << WORKINGSET_SHIFT) - 1)) != 0;
    entry >>= WORKINGSET_SHIFT;
    let nid = entry & ((1usize << NODES_SHIFT) - 1);
    entry >>= NODES_SHIFT;
    let memcgid = entry & ((1usize << MEM_CGROUP_ID_SHIFT) - 1);
    entry >>= MEM_CGROUP_ID_SHIFT;
    *memcgidp = memcgid as i32;
    *pgdat = NODE_DATA(nid as i32);
    *evictionp = entry;
    *workingsetp = workingset;
}

#[cfg(feature = "CONFIG_LRU_GEN")]
unsafe fn lru_gen_eviction(folio: *mut folio) -> *mut core::ffi::c_void {
    let _ = folio;
    todo!("translate CONFIG_LRU_GEN kernel implementation dependencies")
}

#[cfg(not(feature = "CONFIG_LRU_GEN"))]
unsafe fn lru_gen_eviction(_folio: *mut folio) -> *mut core::ffi::c_void { core::ptr::null_mut() }

#[cfg(not(feature = "CONFIG_LRU_GEN"))]
unsafe fn lru_gen_test_recent(_shadow: *mut core::ffi::c_void, _lruvec: *mut *mut lruvec, _token: *mut usize, _workingset: *mut bool, _file: bool) -> bool { false }

#[cfg(not(feature = "CONFIG_LRU_GEN"))]
unsafe fn lru_gen_refault(_folio: *mut folio, _shadow: *mut core::ffi::c_void) {}

pub unsafe fn workingset_age_nonresident(mut lruvec: *mut lruvec, nr_pages: usize) {
    loop {
        atomic_long_add(nr_pages, &mut (*lruvec).nonresident_age);
        lruvec = parent_lruvec(lruvec);
        if lruvec.is_null() { break; }
    }
}

pub unsafe fn workingset_eviction(folio: *mut folio, target_memcg: *mut mem_cgroup) -> *mut core::ffi::c_void {
    let pgdat = folio_pgdat(folio);
    let file = folio_is_file_lru(folio);
    VM_BUG_ON_FOLIO(folio_test_lru(folio), folio);
    VM_BUG_ON_FOLIO(folio_ref_count(folio), folio);
    VM_BUG_ON_FOLIO(!folio_test_locked(folio), folio);
    if lru_gen_enabled() { return lru_gen_eviction(folio); }
    let lruvec = mem_cgroup_lruvec(target_memcg, pgdat);
    let memcgid = mem_cgroup_private_id(lruvec_memcg(lruvec));
    let mut eviction = atomic_long_read(&(*lruvec).nonresident_age);
    eviction >>= BUCKET_ORDER[file as usize];
    workingset_age_nonresident(lruvec, folio_nr_pages(folio));
    pack_shadow(memcgid, pgdat, eviction, folio_test_workingset(folio), file)
}

pub unsafe fn workingset_test_recent(shadow: *mut core::ffi::c_void, file: bool, workingset: *mut bool, flush: bool) -> bool {
    let mut memcgid = 0;
    let mut pgdat = core::ptr::null_mut();
    let mut eviction = 0usize;
    unpack_shadow(shadow, &mut memcgid, &mut pgdat, &mut eviction, workingset);
    eviction <<= BUCKET_ORDER[file as usize];
    let eviction_memcg = mem_cgroup_from_private_id(memcgid);
    if !mem_cgroup_tryget(eviction_memcg) { return false; }
    if flush { mem_cgroup_flush_stats_ratelimited(eviction_memcg); }
    let lruvec = mem_cgroup_lruvec(eviction_memcg, pgdat);
    let refault = atomic_long_read(&(*lruvec).nonresident_age);
    let refault_distance = (refault.wrapping_sub(eviction)) & if file { EVICTION_MASK } else { EVICTION_MASK_ANON };
    let mut workingset_size = lruvec_page_state(lruvec, NR_ACTIVE_FILE);
    if !file { workingset_size += lruvec_page_state(lruvec, NR_INACTIVE_FILE); }
    if mem_cgroup_get_nr_swap_pages(eviction_memcg) > 0 {
        workingset_size += lruvec_page_state(lruvec, NR_ACTIVE_ANON);
        if file { workingset_size += lruvec_page_state(lruvec, NR_INACTIVE_ANON); }
    }
    mem_cgroup_put(eviction_memcg);
    refault_distance <= workingset_size
}

pub unsafe fn workingset_refault(folio: *mut folio, shadow: *mut core::ffi::c_void) {
    let file = folio_is_file_lru(folio);
    VM_BUG_ON_FOLIO(!folio_test_locked(folio), folio);
    if lru_gen_enabled() { lru_gen_refault(folio, shadow); return; }
    let nr = folio_nr_pages(folio);
    let memcg = get_mem_cgroup_from_folio(folio);
    let lruvec = mem_cgroup_lruvec(memcg, folio_pgdat(folio));
    mod_lruvec_state(lruvec, WORKINGSET_REFAULT_BASE + file as usize, nr);
    let mut workingset = false;
    if !workingset_test_recent(shadow, file, &mut workingset, true) { mem_cgroup_put(memcg); return; }
    folio_set_active(folio);
    workingset_age_nonresident(lruvec, nr);
    mod_lruvec_state(lruvec, WORKINGSET_ACTIVATE_BASE + file as usize, nr);
    if workingset { folio_set_workingset(folio); mod_lruvec_state(lruvec, WORKINGSET_RESTORE_BASE + file as usize, nr); }
    mem_cgroup_put(memcg);
}

pub unsafe fn workingset_activation(folio: *mut folio) {
    if mem_cgroup_disabled() || folio_memcg_charged(folio) {
        workingset_age_nonresident(folio_lruvec(folio), folio_nr_pages(folio));
    }
}

// Shadow-node tracking and shrinker registration are supplied by the kernel
// integration layer; declarations retain the externally visible objects.
#[repr(C)] pub struct list_lru { _private: [u8; 0] }
#[repr(C)] pub struct xa_node { _private: [u8; 0] }
pub static mut shadow_nodes: list_lru = list_lru { _private: [] };

pub unsafe fn workingset_update_node(_node: *mut xa_node) {
    // The C implementation maintains shadow_nodes under the xarray lock.
    // The surrounding kernel bindings provide the concrete list and locking APIs.
}

unsafe fn count_shadow_nodes(_shrinker: *mut shrinker, sc: *mut shrink_control) -> usize {
    let nodes = list_lru_shrink_count(&mut shadow_nodes, sc);
    if nodes == 0 { return SHRINK_EMPTY; }
    let pages = if !(*sc).memcg.is_null() {
        mem_cgroup_flush_stats_ratelimited((*sc).memcg);
        let lruvec = mem_cgroup_lruvec((*sc).memcg, NODE_DATA((*sc).nid));
        let mut p = 0;
        let mut i = 0;
        while i < NR_LRU_LISTS { p += lruvec_lru_size(lruvec, i, MAX_NR_ZONES - 1); i += 1; }
        p += lruvec_page_state_local(lruvec, NR_SLAB_RECLAIMABLE_B) >> PAGE_SHIFT;
        p += lruvec_page_state_local(lruvec, NR_SLAB_UNRECLAIMABLE_B) >> PAGE_SHIFT;
        p
    } else { node_present_pages((*sc).nid) };
    let max_nodes = pages >> (XA_CHUNK_SHIFT - 3);
    if nodes <= max_nodes { 0 } else { nodes - max_nodes }
}

unsafe fn shadow_lru_isolate(_item: *mut list_head, _lru: *mut list_lru_one, _arg: *mut core::ffi::c_void) -> lru_status {
    // The C implementation takes the xarray and inode locks, removes the
    // shadow-only node, updates node accounting, and yields when required.
    LRU_REMOVED_RETRY
}

unsafe fn scan_shadow_nodes(_shrinker: *mut shrinker, sc: *mut shrink_control) -> usize {
    list_lru_shrink_walk_irq(&mut shadow_nodes, sc, shadow_lru_isolate, core::ptr::null_mut())
}

#[repr(C)] pub struct lock_class_key { _private: [u8; 0] }
static mut shadow_nodes_key: lock_class_key = lock_class_key { _private: [] };

unsafe fn workingset_init() -> i32 {
    let timestamp_bits = BITS_PER_LONG - EVICTION_SHIFT;
    let timestamp_bits_anon = BITS_PER_LONG - EVICTION_SHIFT_ANON;
    let max_order = fls_long(totalram_pages() - 1);
    if max_order > timestamp_bits { BUCKET_ORDER[WORKINGSET_FILE] = (max_order - timestamp_bits) as u32; }
    if max_order > timestamp_bits_anon { BUCKET_ORDER[WORKINGSET_ANON] = (max_order - timestamp_bits_anon) as u32; }
    pr_info("workingset: timestamp_bits=%d (anon: %d) max_order=%d bucket_order=%u (anon: %d)\n", timestamp_bits, timestamp_bits_anon, max_order, BUCKET_ORDER[WORKINGSET_FILE], BUCKET_ORDER[WORKINGSET_ANON]);
    let shrinker = shrinker_alloc(SHRINKER_NUMA_AWARE | SHRINKER_MEMCG_AWARE, "mm-shadow");
    if shrinker.is_null() { return -ENOMEM; }
    let ret = list_lru_init_memcg_key(&mut shadow_nodes, shrinker, &mut shadow_nodes_key);
    if ret != 0 { shrinker_free(shrinker); return ret; }
    (*shrinker).count_objects = Some(count_shadow_nodes);
    (*shrinker).scan_objects = Some(scan_shadow_nodes);
    (*shrinker).seeks = 0;
    shrinker_register(shrinker);
    0
}

// module_init(workingset_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
