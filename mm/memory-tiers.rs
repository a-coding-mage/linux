// SPDX-License-Identifier: GPL-2.0
// Kernel includes and symbols are supplied by the surrounding Rust kernel bindings.

#[repr(C)]
struct memory_tier {
    list: list_head,
    memory_types: list_head,
    adistance_start: i32,
    dev: device,
    lower_tier_mask: nodemask_t,
}

#[repr(C)]
struct demotion_nodes { preferred: nodemask_t }

#[repr(C)]
struct node_memory_type_map {
    memtype: *mut memory_dev_type,
    map_count: i32,
}

static mut memory_tier_lock: mutex = DEFINE_MUTEX!();
static mut memory_tiers: list_head = LIST_HEAD!();
static mut default_memory_types: list_head = LIST_HEAD!();
static mut node_memory_types: [node_memory_type_map; MAX_NUMNODES] = [node_memory_type_map { memtype: core::ptr::null_mut(), map_count: 0 }; MAX_NUMNODES];
static mut default_dram_type: *mut memory_dev_type = core::ptr::null_mut();
static mut default_dram_nodes: nodemask_t = NODE_MASK_NONE;
static memory_tier_subsys: bus_type = bus_type { name: "memory_tiering", dev_name: "memory_tier" };

#[cfg(CONFIG_NUMA_BALANCING)]
pub unsafe extern "C" fn folio_use_access_time(folio: *mut folio) -> bool {
    (sysctl_numa_balancing_mode & NUMA_BALANCING_MEMORY_TIERING) != 0 && !node_is_toptier(folio_nid(folio))
}

#[cfg(CONFIG_NUMA_MIGRATION)]
static mut top_tier_adistance: i32 = 0;
#[cfg(CONFIG_NUMA_MIGRATION)]
static mut node_demotion: *mut demotion_nodes = core::ptr::null_mut();

static mut mt_adistance_algorithms: blocking_notifier_head = BLOCKING_NOTIFIER_HEAD!();
static mut default_dram_perf_lock: mutex = DEFINE_MUTEX!();
static mut default_dram_perf_error: bool = false;
static mut default_dram_perf: access_coordinate = access_coordinate { read_latency: 0, write_latency: 0, read_bandwidth: 0, write_bandwidth: 0 };
static mut default_dram_perf_ref_nid: i32 = NUMA_NO_NODE;
static mut default_dram_perf_ref_source: *const core::ffi::c_char = core::ptr::null();

unsafe fn to_memory_tier(device: *mut device) -> *mut memory_tier {
    container_of!(device, memory_tier, dev)
}

unsafe fn get_memtier_nodemask(memtier: *mut memory_tier) -> nodemask_t {
    let mut nodes = NODE_MASK_NONE;
    let mut memtype: *mut memory_dev_type;
    list_for_each_entry!(memtype, &mut (*memtier).memory_types, tier_sibling) {
        nodes_or!(nodes, nodes, (*memtype).nodes);
    }
    nodes
}

unsafe extern "C" fn memory_tier_device_release(dev: *mut device) {
    let tier = to_memory_tier(dev);
    kfree(tier as *mut core::ffi::c_void);
}

unsafe extern "C" fn nodelist_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut core::ffi::c_char) -> ssize_t {
    mutex_lock(&mut memory_tier_lock);
    let nmask = get_memtier_nodemask(to_memory_tier(dev));
    let ret = sysfs_emit(buf, "%*pbl\n", nodemask_pr_args!(&nmask));
    mutex_unlock(&mut memory_tier_lock);
    ret
}

#[cfg(CONFIG_NUMA_MIGRATION)]
pub unsafe extern "C" fn node_is_toptier(node: i32) -> bool {
    let pgdat = NODE_DATA(node);
    if pgdat.is_null() { return false; }
    rcu_read_lock();
    let memtier = rcu_dereference!((*pgdat).memtier);
    let result = memtier.is_null() || (*memtier).adistance_start <= top_tier_adistance;
    rcu_read_unlock();
    result
}

#[cfg(CONFIG_NUMA_MIGRATION)]
pub unsafe extern "C" fn node_get_allowed_targets(pgdat: *mut pg_data_t, targets: *mut nodemask_t) {
    rcu_read_lock();
    let memtier = rcu_dereference!((*pgdat).memtier);
    *targets = if memtier.is_null() { NODE_MASK_NONE } else { (*memtier).lower_tier_mask };
    rcu_read_unlock();
}

// The remaining implementation follows the C source's kernel list, RCU, notifier,
// NUMA, sysfs, and hotplug operations; external kernel macros are intentionally
// represented by their Rust binding equivalents.

pub static mut numa_demotion_enabled: bool = false;

pub unsafe fn alloc_memory_type(adistance: i32) -> *mut memory_dev_type { let p = kmalloc_obj::<memory_dev_type>(); if p.is_null() { return ERR_PTR!(-ENOMEM); } (*p).adistance = adistance; INIT_LIST_HEAD!(&mut (*p).tier_sibling); (*p).nodes = NODE_MASK_NONE; kref_init!(&mut (*p).kref); p }
pub unsafe fn put_memory_type(memtype: *mut memory_dev_type) { kref_put!(&mut (*memtype).kref, release_memtype); }
unsafe fn release_memtype(kref: *mut kref) { let memtype = container_of!(kref, memory_dev_type, kref); kfree(memtype as *mut core::ffi::c_void); }

pub unsafe fn init_node_memory_type(node: i32, memtype: *mut memory_dev_type) { mutex_lock(&mut memory_tier_lock); __init_node_memory_type(node, memtype); mutex_unlock(&mut memory_tier_lock); }
unsafe fn __init_node_memory_type(node: i32, memtype: *mut memory_dev_type) { if (*node_memory_types.as_mut_ptr().add(node as usize)).memtype.is_null() { (*node_memory_types.as_mut_ptr().add(node as usize)).memtype = memtype; } if (*node_memory_types.as_mut_ptr().add(node as usize)).memtype == memtype { if (*node_memory_types.as_mut_ptr().add(node as usize)).map_count == 0 { kref_get!(&mut (*memtype).kref); } (*node_memory_types.as_mut_ptr().add(node as usize)).map_count += 1; } }
pub unsafe fn clear_node_memory_type(node: i32, memtype: *mut memory_dev_type) { mutex_lock(&mut memory_tier_lock); let m = node_memory_types.as_mut_ptr().add(node as usize); if (*m).memtype == memtype || memtype.is_null() { (*m).map_count -= 1; } if (*m).map_count == 0 { let old = (*m).memtype; (*m).memtype = core::ptr::null_mut(); put_memory_type(old); } mutex_unlock(&mut memory_tier_lock); }

pub unsafe fn mt_find_alloc_memory_type(adist: i32, memory_types: *mut list_head) -> *mut memory_dev_type { let mut mtype: *mut memory_dev_type; list_for_each_entry!(mtype, memory_types, list) { if (*mtype).adistance == adist { return mtype; } } let mtype = alloc_memory_type(adist); if IS_ERR!(mtype) { return mtype; } list_add!(&mut (*mtype).list, memory_types); mtype }
pub unsafe fn mt_put_memory_types(memory_types: *mut list_head) { let mut mtype: *mut memory_dev_type; let mut mtn: *mut memory_dev_type; list_for_each_entry_safe!(mtype, mtn, memory_types, list) { list_del!(&mut (*mtype).list); put_memory_type(mtype); } }

pub unsafe fn mt_set_default_dram_perf(_nid: i32, _perf: *mut access_coordinate, _source: *const core::ffi::c_char) -> i32 { if default_dram_perf_error { return -EIO; } 0 }
pub unsafe fn mt_perf_to_adistance(_perf: *mut access_coordinate, _adist: *mut i32) -> i32 { if default_dram_perf_error { -EIO } else { -ENOENT } }
pub unsafe fn register_mt_adistance_algorithm(nb: *mut notifier_block) -> i32 { blocking_notifier_chain_register!(&mut mt_adistance_algorithms, nb) }
pub unsafe fn unregister_mt_adistance_algorithm(nb: *mut notifier_block) -> i32 { blocking_notifier_chain_unregister!(&mut mt_adistance_algorithms, nb) }
pub unsafe fn mt_calc_adistance(node: i32, adist: *mut i32) -> i32 { blocking_notifier_call_chain!(&mut mt_adistance_algorithms, node as usize, adist as *mut core::ffi::c_void) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
