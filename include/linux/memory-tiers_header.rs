/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than redefined.

pub const MEMTIER_CHUNK_BITS: u32 = 7;
pub const MEMTIER_CHUNK_SIZE: i32 = 1 << MEMTIER_CHUNK_BITS;
pub const MEMTIER_ADISTANCE_DRAM: i64 =
    (4_i64 * MEMTIER_CHUNK_SIZE as i64) + (MEMTIER_CHUNK_SIZE as i64 >> 1);

#[repr(C)]
pub struct memory_tier {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct memory_dev_type {
    /* list of memory types that are part of same tier as this type */
    pub tier_sibling: list_head,
    /* list of memory types that are managed by one driver */
    pub list: list_head,
    /* abstract distance for this specific memory type */
    pub adistance: i32,
    /* Nodes of same abstract distance */
    pub nodes: nodemask_t,
    pub kref: kref,
}

#[repr(C)]
pub struct access_coordinate {
    _opaque: [u8; 0],
}

#[cfg(CONFIG_NUMA)]
extern "C" {
    pub static mut numa_demotion_enabled: bool;
    pub static mut default_dram_type: *mut memory_dev_type;
    pub static mut default_dram_nodes: nodemask_t;

    pub fn alloc_memory_type(adistance: i32) -> *mut memory_dev_type;
    pub fn put_memory_type(memtype: *mut memory_dev_type);
    pub fn init_node_memory_type(node: i32, default_type: *mut memory_dev_type);
    pub fn clear_node_memory_type(node: i32, memtype: *mut memory_dev_type);
    pub fn register_mt_adistance_algorithm(nb: *mut notifier_block) -> i32;
    pub fn unregister_mt_adistance_algorithm(nb: *mut notifier_block) -> i32;
    pub fn mt_calc_adistance(node: i32, adist: *mut i32) -> i32;
    pub fn mt_set_default_dram_perf(
        nid: i32,
        perf: *mut access_coordinate,
        source: *const i8,
    ) -> i32;
    pub fn mt_perf_to_adistance(perf: *mut access_coordinate, adist: *mut i32) -> i32;
    pub fn mt_find_alloc_memory_type(
        adist: i32,
        memory_types: *mut list_head,
    ) -> *mut memory_dev_type;
    pub fn mt_put_memory_types(memory_types: *mut list_head);

    #[cfg(CONFIG_NUMA_MIGRATION)]
    pub fn next_demotion_node(node: i32, allowed_mask: *const nodemask_t) -> i32;
    #[cfg(CONFIG_NUMA_MIGRATION)]
    pub fn node_get_allowed_targets(pgdat: *mut pg_data_t, targets: *mut nodemask_t);
    #[cfg(CONFIG_NUMA_MIGRATION)]
    pub fn node_is_toptier(node: i32) -> bool;
}

#[cfg(all(CONFIG_NUMA, not(CONFIG_NUMA_MIGRATION)))]
pub unsafe fn next_demotion_node(_node: i32, _allowed_mask: *const nodemask_t) -> i32 {
    NUMA_NO_NODE
}

#[cfg(all(CONFIG_NUMA, not(CONFIG_NUMA_MIGRATION)))]
pub unsafe fn node_get_allowed_targets(_pgdat: *mut pg_data_t, targets: *mut nodemask_t) {
    *targets = NODE_MASK_NONE;
}

#[cfg(all(CONFIG_NUMA, not(CONFIG_NUMA_MIGRATION)))]
pub unsafe fn node_is_toptier(_node: i32) -> bool {
    true
}

/* CONFIG_NUMA implementation returns non NULL error. */
#[cfg(not(CONFIG_NUMA))]
pub const numa_demotion_enabled: bool = false;
#[cfg(not(CONFIG_NUMA))]
pub const default_dram_type: *mut memory_dev_type = core::ptr::null_mut();
#[cfg(not(CONFIG_NUMA))]
pub const default_dram_nodes: nodemask_t = NODE_MASK_NONE;

#[cfg(not(CONFIG_NUMA))]
pub unsafe fn alloc_memory_type(_adistance: i32) -> *mut memory_dev_type { core::ptr::null_mut() }
#[cfg(not(CONFIG_NUMA))]
pub unsafe fn put_memory_type(_memtype: *mut memory_dev_type) {}
#[cfg(not(CONFIG_NUMA))]
pub unsafe fn init_node_memory_type(_node: i32, _default_type: *mut memory_dev_type) {}
#[cfg(not(CONFIG_NUMA))]
pub unsafe fn clear_node_memory_type(_node: i32, _memtype: *mut memory_dev_type) {}
#[cfg(not(CONFIG_NUMA))]
pub unsafe fn next_demotion_node(_node: i32, _allowed_mask: *const nodemask_t) -> i32 { NUMA_NO_NODE }
#[cfg(not(CONFIG_NUMA))]
pub unsafe fn node_get_allowed_targets(_pgdat: *mut pg_data_t, targets: *mut nodemask_t) { *targets = NODE_MASK_NONE; }
#[cfg(not(CONFIG_NUMA))]
pub unsafe fn node_is_toptier(_node: i32) -> bool { true }
#[cfg(not(CONFIG_NUMA))]
pub unsafe fn register_mt_adistance_algorithm(_nb: *mut notifier_block) -> i32 { 0 }
#[cfg(not(CONFIG_NUMA))]
pub unsafe fn unregister_mt_adistance_algorithm(_nb: *mut notifier_block) -> i32 { 0 }
#[cfg(not(CONFIG_NUMA))]
pub unsafe fn mt_calc_adistance(_node: i32, _adist: *mut i32) -> i32 { NOTIFY_DONE }
#[cfg(not(CONFIG_NUMA))]
pub unsafe fn mt_set_default_dram_perf(_nid: i32, _perf: *mut access_coordinate, _source: *const i8) -> i32 { -EIO }
#[cfg(not(CONFIG_NUMA))]
pub unsafe fn mt_perf_to_adistance(_perf: *mut access_coordinate, _adist: *mut i32) -> i32 { -EIO }
#[cfg(not(CONFIG_NUMA))]
pub unsafe fn mt_find_alloc_memory_type(_adist: i32, _memory_types: *mut list_head) -> *mut memory_dev_type { core::ptr::null_mut() }
#[cfg(not(CONFIG_NUMA))]
pub unsafe fn mt_put_memory_types(_memory_types: *mut list_head) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
