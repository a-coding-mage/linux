// SPDX-License-Identifier: GPL-2.0
/* Copyright(c) 2015 Intel Corporation. All rights reserved. */
// Kernel dependencies supplied by other translation units are intentionally
// referenced here rather than reimplemented.

static mut pgmap_array: XArray = DEFINE_XARRAY!();

/*
 * The memremap() and memremap_pages() interfaces are alternately used
 * to map persistent memory namespaces. These interfaces place different
 * constraints on the alignment and size of the mapping (namespace).
 * memremap() can map individual PAGE_SIZE pages. memremap_pages() can
 * only map subsections (2MB), and at least one architecture (PowerPC)
 * the minimum mapping granularity of memremap_pages() is 16MB.
 *
 * The role of memremap_compat_align() is to communicate the minimum
 * arch supported alignment of a namespace such that it can freely
 * switch modes without violating the arch constraint. Namely, do not
 * allow a namespace to be PAGE_SIZE aligned since that namespace may be
 * reconfigured into a mode that requires SUBSECTION_SIZE alignment.
 */
#[cfg(not(CONFIG_ARCH_HAS_MEMREMAP_COMPAT_ALIGN))]
pub unsafe extern "C" fn memremap_compat_align() -> c_ulong {
    SUBSECTION_SIZE
}

unsafe fn pgmap_array_delete(range: *mut range) {
    xa_store_range(&mut pgmap_array, PHYS_PFN((*range).start), PHYS_PFN((*range).end), core::ptr::null_mut(), GFP_KERNEL);
    synchronize_rcu();
}

unsafe fn pfn_first(pgmap: *mut dev_pagemap, range_id: c_int) -> c_ulong {
    let range = &mut (*pgmap).ranges[range_id as usize];
    let pfn = PHYS_PFN(range.start);
    if range_id != 0 { return pfn; }
    pfn + vmem_altmap_offset(pgmap_altmap(pgmap))
}

pub unsafe extern "C" fn pgmap_pfn_valid(pgmap: *mut dev_pagemap, pfn: c_ulong) -> bool {
    for i in 0..(*pgmap).nr_range {
        let range = &mut (*pgmap).ranges[i as usize];
        if pfn >= PHYS_PFN(range.start) && pfn <= PHYS_PFN(range.end) {
            return pfn >= pfn_first(pgmap, i as c_int);
        }
    }
    false
}

unsafe fn pfn_end(pgmap: *mut dev_pagemap, range_id: c_int) -> c_ulong {
    let range = &(*pgmap).ranges[range_id as usize];
    (range.start + range_len(range)) >> PAGE_SHIFT
}

unsafe fn pfn_len(pgmap: *mut dev_pagemap, range_id: c_ulong) -> c_ulong {
    (pfn_end(pgmap, range_id as c_int) - pfn_first(pgmap, range_id as c_int)) >> (*pgmap).vmemmap_shift
}

unsafe fn pageunmap_range(pgmap: *mut dev_pagemap, range_id: c_int) {
    let range = &mut (*pgmap).ranges[range_id as usize];
    let first_page = pfn_to_page(pfn_first(pgmap, range_id));
    mem_hotplug_begin();
    remove_pfn_range_from_zone(page_zone(first_page), PHYS_PFN(range.start), PHYS_PFN(range_len(range)));
    if (*pgmap).type_ == MEMORY_DEVICE_PRIVATE {
        __remove_pages(PHYS_PFN(range.start), PHYS_PFN(range_len(range)), core::ptr::null_mut(), pgmap);
    } else {
        arch_remove_memory(range.start, range_len(range), pgmap_altmap(pgmap), pgmap);
        kasan_remove_zero_shadow(__va(range.start), range_len(range));
    }
    mem_hotplug_done();
    pfnmap_untrack(PHYS_PFN(range.start), range_len(range));
    pgmap_array_delete(range);
}

pub unsafe extern "C" fn memunmap_pages(pgmap: *mut dev_pagemap) {
    percpu_ref_kill(&mut (*pgmap).ref_);
    if (*pgmap).type_ != MEMORY_DEVICE_PRIVATE && (*pgmap).type_ != MEMORY_DEVICE_COHERENT {
        for i in 0..(*pgmap).nr_range { percpu_ref_put_many(&mut (*pgmap).ref_, pfn_len(pgmap, i as c_ulong)); }
    }
    wait_for_completion(&mut (*pgmap).done);
    for i in 0..(*pgmap).nr_range { pageunmap_range(pgmap, i as c_int); }
    percpu_ref_exit(&mut (*pgmap).ref_);
    WARN_ONCE((*pgmap).altmap.alloc != 0, c"failed to free all reserved pages\n");
}

unsafe fn devm_memremap_pages_release(data: *mut c_void) { memunmap_pages(data as *mut dev_pagemap); }

unsafe fn dev_pagemap_percpu_release(ref_: *mut percpu_ref) {
    let pgmap = container_of!(ref_, dev_pagemap, ref_);
    complete(&mut (*pgmap).done);
}

unsafe fn pagemap_range(pgmap: *mut dev_pagemap, params: *mut mhp_params, range_id: c_int, mut nid: c_int) -> c_int {
    let is_private = (*pgmap).type_ == MEMORY_DEVICE_PRIVATE;
    let range = &mut (*pgmap).ranges[range_id as usize];
    if WARN_ONCE(pgmap_altmap(pgmap) != core::ptr::null_mut() && range_id > 0, c"altmap not supported for multiple ranges\n") { return -EINVAL; }
    let mut conflict = get_dev_pagemap(PHYS_PFN(range.start));
    if !conflict.is_null() { WARN!(1, c"Conflicting mapping in same section\n"); put_dev_pagemap(conflict); return -ENOMEM; }
    conflict = get_dev_pagemap(PHYS_PFN(range.end));
    if !conflict.is_null() { WARN!(1, c"Conflicting mapping in same section\n"); put_dev_pagemap(conflict); return -ENOMEM; }
    let is_ram = region_intersects(range.start, range_len(range), IORESOURCE_SYSTEM_RAM, IORES_DESC_NONE);
    if is_ram != REGION_DISJOINT { WARN_ONCE(true, c"attempted on region\n"); return -ENXIO; }
    let mut error = xa_err(xa_store_range(&mut pgmap_array, PHYS_PFN(range.start), PHYS_PFN(range.end), pgmap, GFP_KERNEL));
    if error != 0 { return error; }
    if nid < 0 { nid = numa_mem_id(); }
    error = pfnmap_track(PHYS_PFN(range.start), range_len(range), &mut (*params).pgprot);
    if error != 0 { pgmap_array_delete(range); return error; }
    if !mhp_range_allowed(range.start, range_len(range), !is_private) { pfnmap_untrack(PHYS_PFN(range.start), range_len(range)); pgmap_array_delete(range); return -EINVAL; }
    mem_hotplug_begin();
    if is_private { error = add_pages(nid, PHYS_PFN(range.start), PHYS_PFN(range_len(range)), params); }
    else { error = kasan_add_zero_shadow(__va(range.start), range_len(range)); if error != 0 { mem_hotplug_done(); pfnmap_untrack(PHYS_PFN(range.start), range_len(range)); pgmap_array_delete(range); return error; } error = arch_add_memory(nid, range.start, range_len(range), params); }
    if error == 0 { let zone = &mut (*NODE_DATA(nid)).node_zones[ZONE_DEVICE]; move_pfn_range_to_zone(zone, PHYS_PFN(range.start), PHYS_PFN(range_len(range)), (*params).altmap, MIGRATE_MOVABLE, false); }
    mem_hotplug_done();
    if error != 0 { if !is_private { kasan_remove_zero_shadow(__va(range.start), range_len(range)); } pfnmap_untrack(PHYS_PFN(range.start), range_len(range)); pgmap_array_delete(range); return error; }
    memmap_init_zone_device(&mut (*NODE_DATA(nid)).node_zones[ZONE_DEVICE], PHYS_PFN(range.start), PHYS_PFN(range_len(range)), pgmap);
    if (*pgmap).type_ != MEMORY_DEVICE_PRIVATE && (*pgmap).type_ != MEMORY_DEVICE_COHERENT { percpu_ref_get_many(&mut (*pgmap).ref_, pfn_len(pgmap, range_id as c_ulong)); }
    0
}

pub unsafe extern "C" fn memremap_pages(pgmap: *mut dev_pagemap, nid: c_int) -> *mut c_void {
    let mut params = mhp_params { altmap: pgmap_altmap(pgmap), pgmap, pgprot: PAGE_KERNEL };
    let nr_range = (*pgmap).nr_range;
    if WARN_ONCE(nr_range == 0, c"nr_range must be specified\n") || WARN_ONCE((*pgmap).vmemmap_shift > MAX_FOLIO_ORDER, c"requested folio size unsupported\n") { return ERR_PTR(-EINVAL); }
    init_completion(&mut (*pgmap).done);
    let error = percpu_ref_init(&mut (*pgmap).ref_, dev_pagemap_percpu_release, 0, GFP_KERNEL);
    if error != 0 { return ERR_PTR(error); }
    (*pgmap).nr_range = 0;
    for i in 0..nr_range { if pagemap_range(pgmap, &mut params, i as c_int, nid) != 0 { memunmap_pages(pgmap); (*pgmap).nr_range = nr_range; return ERR_PTR(-EINVAL); } (*pgmap).nr_range += 1; }
    __va((*pgmap).ranges[0].start)
}

unsafe fn devm_memremap_pages_release_action(data: *mut c_void) { memunmap_pages(data as *mut dev_pagemap); }
pub unsafe extern "C" fn devm_memremap_pages(dev: *mut device, pgmap: *mut dev_pagemap) -> *mut c_void {
    let ret = memremap_pages(pgmap, dev_to_node(dev));
    if IS_ERR(ret) { return ret; }
    let error = devm_add_action_or_reset(dev, devm_memremap_pages_release_action, pgmap as *mut c_void);
    if error != 0 { return ERR_PTR(error); }
    ret
}
pub unsafe extern "C" fn devm_memunmap_pages(dev: *mut device, pgmap: *mut dev_pagemap) { devm_release_action(dev, devm_memremap_pages_release_action, pgmap as *mut c_void); }

pub unsafe extern "C" fn get_dev_pagemap(pfn: c_ulong) -> *mut dev_pagemap {
    let phys = PFN_PHYS(pfn); rcu_read_lock(); let pgmap = xa_load(&mut pgmap_array, PHYS_PFN(phys)); let result = if !pgmap.is_null() && !percpu_ref_tryget_live_rcu(&mut (*pgmap).ref_) { core::ptr::null_mut() } else { pgmap }; rcu_read_unlock(); result
}

pub unsafe extern "C" fn free_zone_device_folio(folio: *mut folio) {
    let pgmap = (*folio).pgmap; if WARN_ON_ONCE(pgmap.is_null()) { return; }
    mem_cgroup_uncharge(folio); if folio_test_anon(folio) { mod_mthp_stat(folio_order(folio), MTHP_STAT_NR_ANON, -1); for i in 0..folio_nr_pages(folio) { __ClearPageAnonExclusive(folio_page(folio, i)); } }
    if (*pgmap).type_ != MEMORY_DEVICE_FS_DAX && (*pgmap).type_ != MEMORY_DEVICE_GENERIC { (*folio).mapping = core::ptr::null_mut(); }
    match (*pgmap).type_ { MEMORY_DEVICE_PRIVATE | MEMORY_DEVICE_COHERENT => { if !WARN_ON_ONCE((*pgmap).ops.is_null()) { ((*(*pgmap).ops).folio_free)(folio); } percpu_ref_put_many(&mut (*pgmap).ref_, folio_nr_pages(folio)); }, MEMORY_DEVICE_GENERIC => folio_set_count(folio, 1), MEMORY_DEVICE_FS_DAX => wake_up_var(&mut (*folio).page), MEMORY_DEVICE_PCI_P2PDMA => { if !WARN_ON_ONCE((*pgmap).ops.is_null()) { ((*(*pgmap).ops).folio_free)(folio); } }, _ => {} }
}

pub unsafe extern "C" fn zone_device_page_init(page: *mut page, pgmap: *mut dev_pagemap, order: c_uint) {
    let mut new_page = page; for _ in 0..(1UL << order) { let new_folio = new_page as *mut folio; (*new_page).flags.f &= !0xffUL; (*new_folio).mapping = core::ptr::null_mut(); (*new_folio).pgmap = pgmap; (*new_folio).share = 0; new_page = new_page.add(1); }
    WARN_ON_ONCE(!percpu_ref_tryget_many(&mut (*page_pgmap(page)).ref_, 1 << order)); set_page_count(page, 1); lock_page(page); if order != 0 { prep_compound_page(page, order); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
