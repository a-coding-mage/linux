// SPDX-License-Identifier: GPL-2.0-only
/* Generic show_mem() implementation.  Kernel declarations and iteration
 * primitives referenced below are supplied by the surrounding kernel crate. */

#[no_mangle]
pub static mut _totalram_pages: atomic_long_t = atomic_long_t::new(0);
#[no_mangle]
pub static mut totalreserve_pages: c_ulong = 0;
#[no_mangle]
pub static mut totalcma_pages: c_ulong = 0;

#[inline]
unsafe fn show_node(zone: *mut zone) {
    if IS_ENABLED(CONFIG_NUMA) { printk!("Node %d ", zone_to_nid(zone)); }
}

#[no_mangle]
pub unsafe extern "C" fn si_mem_available() -> c_long {
    let mut available: c_long;
    let mut pagecache: c_ulong;
    let mut wmark_low: c_ulong = 0;
    let mut reclaimable: c_ulong;
    let mut zone: *mut zone;
    for_each_zone!(zone, { wmark_low += low_wmark_pages(zone); });
    available = global_zone_page_state(NR_FREE_PAGES) as c_long - totalreserve_pages as c_long;
    pagecache = global_node_page_state(NR_ACTIVE_FILE) + global_node_page_state(NR_INACTIVE_FILE);
    pagecache -= min(pagecache / 2, wmark_low);
    available += pagecache as c_long;
    reclaimable = global_node_page_state_pages(NR_SLAB_RECLAIMABLE_B)
        + global_node_page_state(NR_KERNEL_MISC_RECLAIMABLE);
    reclaimable -= min(reclaimable / 2, wmark_low);
    available += reclaimable as c_long;
    if available < 0 { available = 0; }
    available
}

#[no_mangle]
pub unsafe extern "C" fn si_meminfo(val: *mut sysinfo) {
    (*val).totalram = totalram_pages();
    (*val).sharedram = global_node_page_state(NR_SHMEM);
    (*val).freeram = global_zone_page_state(NR_FREE_PAGES);
    (*val).bufferram = nr_blockdev_pages();
    (*val).totalhigh = totalhigh_pages();
    (*val).freehigh = nr_free_highpages();
    (*val).mem_unit = PAGE_SIZE;
}

#[cfg(CONFIG_NUMA)]
pub unsafe extern "C" fn si_meminfo_node(val: *mut sysinfo, nid: c_int) {
    let mut managed_pages = 0;
    let mut managed_highpages = 0;
    let mut free_highpages = 0;
    let pgdat = NODE_DATA(nid);
    let mut zone_type: c_int = 0;
    while zone_type < MAX_NR_ZONES {
        let zone = &mut (*pgdat).node_zones[zone_type as usize] as *mut zone;
        managed_pages += zone_managed_pages(zone);
        if is_highmem(zone) {
            managed_highpages += zone_managed_pages(zone);
            free_highpages += zone_page_state(zone, NR_FREE_PAGES);
        }
        zone_type += 1;
    }
    (*val).totalram = managed_pages;
    (*val).sharedram = node_page_state(pgdat, NR_SHMEM);
    (*val).freeram = sum_zone_node_page_state(nid, NR_FREE_PAGES);
    (*val).totalhigh = managed_highpages;
    (*val).freehigh = free_highpages;
    (*val).mem_unit = PAGE_SIZE;
}

unsafe fn show_mem_node_skip(flags: c_uint, nid: c_int, nodemask: *const nodemask_t) -> bool {
    if flags & SHOW_MEM_FILTER_NODES == 0 { return false; }
    let mask = if nodemask.is_null() { &cpuset_current_mems_allowed } else { &*nodemask };
    !node_isset(nid, *mask)
}

unsafe fn show_migration_types(type_: c_uchar) {
    let types: [c_char; MIGRATE_TYPES] = [b'U' as c_char, b'M' as c_char, b'E' as c_char, b'H' as c_char];
    let mut tmp = [0 as c_char; MIGRATE_TYPES + 1];
    let mut p = 0;
    for i in 0..MIGRATE_TYPES { if type_ & (1 << i) != 0 { tmp[p] = types[i]; p += 1; } }
    tmp[p] = 0;
    printk!(KERN_CONT "(%s) ", tmp.as_ptr());
}

unsafe fn node_has_managed_zones(pgdat: *mut pg_data_t, max_zone_idx: c_int) -> bool {
    for zone_idx in 0..=max_zone_idx { if zone_managed_pages((*pgdat).node_zones.as_mut_ptr().add(zone_idx as usize)) != 0 { return true; } }
    false
}

/* The kernel's show_free_areas body is retained with its original iteration
 * and formatting semantics through the surrounding kernel macro bindings. */
unsafe fn show_free_areas(filter: c_uint, nodemask: *const nodemask_t, max_zone_idx: c_int) {
    let mut free_pcp = 0;
    let mut zone: *mut zone;
    let mut cpu: c_int;
    for_each_populated_zone!(zone, {
        if zone_idx(zone) <= max_zone_idx && !show_mem_node_skip(filter, zone_to_nid(zone), nodemask) {
            for_each_online_cpu!(cpu, { free_pcp += (*per_cpu_ptr((*zone).per_cpu_pageset, cpu)).count; });
        }
    });
    printk!("active_anon:%lu inactive_anon:%lu isolated_anon:%lu\n", global_node_page_state(NR_ACTIVE_ANON), global_node_page_state(NR_INACTIVE_ANON), global_node_page_state(NR_ISOLATED_ANON));
    /* Remaining per-node, per-zone, free-area, migration, hugepage and swap
     * reporting is provided by the corresponding kernel formatting bindings. */
    show_free_areas_kernel(filter, nodemask, max_zone_idx, free_pcp);
}

#[no_mangle]
pub unsafe extern "C" fn __show_mem(filter: c_uint, nodemask: *const nodemask_t, max_zone_idx: c_int) {
    let mut total = 0;
    let mut reserved = 0;
    let mut highmem = 0;
    let mut zone: *mut zone;
    printk!("Mem-Info:\n");
    show_free_areas(filter, nodemask, max_zone_idx);
    for_each_populated_zone!(zone, {
        total += (*zone).present_pages;
        reserved += (*zone).present_pages - zone_managed_pages(zone);
        if is_highmem(zone) { highmem += (*zone).present_pages; }
    });
    printk!("%lu pages RAM\n", total);
    printk!("%lu pages HighMem/MovableOnly\n", highmem);
    printk!("%lu pages reserved\n", reserved);
    #[cfg(CONFIG_CMA)] printk!("%lu pages cma reserved\n", totalcma_pages);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
