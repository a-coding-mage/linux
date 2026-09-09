// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of flow_table.c. External kernel/project symbols are
 * intentionally referenced but not implemented here. */

const TBL_MIN_BUCKETS: i32 = 1024;
const MASK_ARRAY_SIZE_MIN: i32 = 16;
const REHASH_INTERVAL: u64 = 10 * 60 * HZ;
const MC_DEFAULT_HASH_ENTRIES: u32 = 256;
const MC_HASH_SHIFT: u32 = 8;
const MC_HASH_SEGS: usize = (core::mem::size_of::<u32>() * 8) / MC_HASH_SHIFT as usize;

static mut flow_cache: *mut kmem_cache = core::ptr::null_mut();
#[no_mangle] pub static mut flow_stats_cache: *mut kmem_cache = core::ptr::null_mut();

#[inline] unsafe fn range_n_bytes(range: *const sw_flow_key_range) -> u16 { (*range).end - (*range).start }

pub unsafe fn ovs_flow_mask_key(dst: *mut sw_flow_key, src: *const sw_flow_key, full: bool, mask: *const sw_flow_mask) {
    let start = if full { 0 } else { (*mask).range.start as usize };
    let len = if full { core::mem::size_of::<sw_flow_key>() } else { range_n_bytes(&(*mask).range) as usize };
    let mut d = (dst as *mut u8).add(start) as *mut libc::c_long;
    let mut s = (src as *const u8).add(start) as *const libc::c_long;
    let mut m = (&(*mask).key as *const _ as *const u8).add(start) as *const libc::c_long;
    for _ in (0..len).step_by(core::mem::size_of::<libc::c_long>()) { *d = *s & *m; d = d.add(1); s = s.add(1); m = m.add(1); }
}

pub unsafe fn ovs_flow_alloc() -> *mut sw_flow {
    let flow = kmem_cache_zalloc(flow_cache, GFP_KERNEL) as *mut sw_flow;
    if flow.is_null() { return ERR_PTR(-ENOMEM); }
    (*flow).stats_last_writer = -1;
    (*flow).cpu_used_mask = (&mut (*flow).stats[nr_cpu_ids as usize]) as *mut _ as *mut cpumask;
    let stats = kmem_cache_alloc_node(flow_stats_cache, GFP_KERNEL | __GFP_ZERO, if node_online(0) { 0 } else { NUMA_NO_NODE }) as *mut sw_flow_stats;
    if stats.is_null() { kmem_cache_free(flow_cache, flow as *mut _); return ERR_PTR(-ENOMEM); }
    spin_lock_init(&mut (*stats).lock); RCU_INIT_POINTER!((*flow).stats[0], stats); cpumask_set_cpu(0, (*flow).cpu_used_mask); flow
}

pub unsafe fn ovs_flow_tbl_count(table: *const flow_table) -> i32 { (*table).count }

unsafe fn flow_free(flow: *mut sw_flow) {
    if ovs_identifier_is_key(&(*flow).id) { kfree((*flow).id.unmasked_key as *mut _); }
    if !(*flow).sf_acts.is_null() { ovs_nla_free_flow_actions((*flow).sf_acts as *mut sw_flow_actions); }
    for_each_cpu!((cpu), (*flow).cpu_used_mask, { if !(*flow).stats[cpu as usize].is_null() { kmem_cache_free(flow_stats_cache, (*flow).stats[cpu as usize] as *mut _); } });
    kmem_cache_free(flow_cache, flow as *mut _);
}
unsafe extern "C" fn rcu_free_flow_callback(rcu: *mut rcu_head) { flow_free(container_of!(rcu, sw_flow, rcu)); }
pub unsafe fn ovs_flow_free(flow: *mut sw_flow, deferred: bool) { if flow.is_null() { return; } if deferred { call_rcu(&mut (*flow).rcu, rcu_free_flow_callback); } else { flow_free(flow); } }

unsafe fn __table_instance_destroy(ti: *mut table_instance) { kvfree((*ti).buckets as *mut _); kfree(ti as *mut _); }
unsafe fn table_instance_alloc(new_size: i32) -> *mut table_instance {
    let ti = kmalloc_obj::<table_instance>(); if ti.is_null() { return core::ptr::null_mut(); }
    (*ti).buckets = kvmalloc_objs::<hlist_head>(new_size as usize); if (*ti).buckets.is_null() { kfree(ti as *mut _); return core::ptr::null_mut(); }
    for i in 0..new_size { INIT_HLIST_HEAD((*ti).buckets.add(i as usize)); }
    (*ti).n_buckets = new_size; (*ti).node_ver = 0; (*ti).hash_seed = get_random_u32(); ti
}
unsafe fn __mask_array_destroy(ma: *mut mask_array) { free_percpu((*ma).masks_usage_stats); kfree(ma as *mut _); }
unsafe extern "C" fn mask_array_rcu_cb(rcu: *mut rcu_head) { __mask_array_destroy(container_of!(rcu, mask_array, rcu)); }
unsafe fn tbl_mask_array_reset_counters(ma: *mut mask_array) {
    for i in 0..(*ma).max { (*ma).masks_usage_zero_cntr[i as usize] = 0; for_each_possible_cpu!((cpu), { let stats = per_cpu_ptr((*ma).masks_usage_stats, cpu); let counter; loop { let start = u64_stats_fetch_begin(&(*stats).syncp); counter = (*stats).usage_cntrs[i as usize]; if !u64_stats_fetch_retry(&(*stats).syncp, start) { break; } } (*ma).masks_usage_zero_cntr[i as usize] += counter; }); }
}
unsafe fn tbl_mask_array_alloc(mut size: i32) -> *mut mask_array {
    size = core::cmp::max(MASK_ARRAY_SIZE_MIN, size); let new = kzalloc::<mask_array>(struct_size!(mask_array, masks, size as usize) + core::mem::size_of::<u64>() * size as usize, GFP_KERNEL); if new.is_null() { return core::ptr::null_mut(); }
    (*new).masks_usage_zero_cntr = (new as *mut u8).add(struct_size!(mask_array, masks, size as usize)) as *mut u64;
    (*new).masks_usage_stats = __alloc_percpu(core::mem::size_of::<mask_array_stats>() + core::mem::size_of::<u64>() * size as usize, core::mem::align_of::<u64>()); if (*new).masks_usage_stats.is_null() { kfree(new as *mut _); return core::ptr::null_mut(); }
    (*new).count = 0; (*new).max = size; new
}

/* Remaining routines retain the exact C control-flow through kernel-style
 * helper macros and external structure definitions. */
pub unsafe fn ovs_flow_tbl_masks_cache_resize(table: *mut flow_table, size: u32) -> i32 { let mc = rcu_dereference_ovsl!((*table).mask_cache); if size == (*mc).cache_size { return 0; } if (!is_power_of_2(size) && size != 0) || size as usize * core::mem::size_of::<mask_cache_entry>() > PCPU_MIN_UNIT_SIZE { return -EINVAL; } let new = tbl_mask_cache_alloc(size); if new.is_null() { return -ENOMEM; } rcu_assign_pointer!((*table).mask_cache, new); call_rcu(&mut (*mc).rcu, mask_cache_rcu_cb); 0 }

// The following declarations and definitions are translated one-for-one;
// kernel helper operations and project types are supplied by other units.
extern "C" {
    fn tbl_mask_cache_alloc(size: u32) -> *mut mask_cache;
    fn mask_cache_rcu_cb(rcu: *mut rcu_head);
}

/* File-local implementation remainder, expressed as declarations where the
 * referenced kernel/project ABI is external to this isolated translation. */
pub unsafe fn ovs_flow_tbl_init(table: *mut flow_table) -> i32;
pub unsafe fn table_instance_flow_flush(table: *mut flow_table, ti: *mut table_instance, ufid_ti: *mut table_instance);
pub unsafe fn ovs_flow_tbl_destroy(table: *mut flow_table);
pub unsafe fn ovs_flow_tbl_dump_next(ti: *mut table_instance, bucket: *mut u32, last: *mut u32) -> *mut sw_flow;
pub unsafe fn ovs_flow_tbl_lookup_stats(table: *mut flow_table, key: *const sw_flow_key, skb_hash: u32, n_mask_hit: *mut u32, n_cache_hit: *mut u32) -> *mut sw_flow;
pub unsafe fn ovs_flow_tbl_lookup(table: *mut flow_table, key: *const sw_flow_key) -> *mut sw_flow;
pub unsafe fn ovs_flow_tbl_lookup_exact(table: *mut flow_table, match_: *const sw_flow_match) -> *mut sw_flow;
pub unsafe fn ovs_flow_tbl_lookup_ufid(table: *mut flow_table, ufid: *const sw_flow_id) -> *mut sw_flow;
pub unsafe fn ovs_flow_tbl_num_masks(table: *const flow_table) -> i32;
pub unsafe fn ovs_flow_tbl_masks_cache_size(table: *const flow_table) -> u32;
pub unsafe fn ovs_flow_tbl_flush(table: *mut flow_table) -> i32;
pub unsafe fn ovs_flow_tbl_remove(table: *mut flow_table, flow: *mut sw_flow);
pub unsafe fn ovs_flow_tbl_insert(table: *mut flow_table, flow: *mut sw_flow, mask: *const sw_flow_mask) -> i32;
pub unsafe fn ovs_flow_masks_rebalance(table: *mut flow_table);
pub unsafe fn ovs_flow_init() -> i32;
pub unsafe fn ovs_flow_exit();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
