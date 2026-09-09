// SPDX-License-Identifier: GPL-2.0-only
/*
 * HugeTLB sysfs interfaces.
 * (C) Nadia Yvette Chambers, April 2004
 */

// Dependencies are supplied by the surrounding kernel translation.

static mut hugepages_kobj: *mut kobject = core::ptr::null_mut();
static mut hstate_kobjs: [*mut kobject; HUGE_MAX_HSTATE] = [core::ptr::null_mut(); HUGE_MAX_HSTATE];

static mut nr_hugepages_attr: kobj_attribute = __ATTR_RW!(nr_hugepages);
static mut nr_overcommit_hugepages_attr: kobj_attribute = __ATTR_RW!(nr_overcommit_hugepages);
static mut free_hugepages_attr: kobj_attribute = __ATTR_RO!(free_hugepages);
static mut resv_hugepages_attr: kobj_attribute = __ATTR_RO!(resv_hugepages);
static mut surplus_hugepages_attr: kobj_attribute = __ATTR_RO!(surplus_hugepages);
#[cfg(CONFIG_NUMA)]
static mut nr_hugepages_mempolicy_attr: kobj_attribute = __ATTR_RW!(nr_hugepages_mempolicy);
static mut demote_attr: kobj_attribute = __ATTR_WO!(demote);
static mut demote_size_attr: kobj_attribute = __ATTR_RW!(demote_size);

unsafe fn kobj_to_node_hstate(kobj: *mut kobject, nidp: *mut i32) -> *mut hstate;

unsafe fn kobj_to_hstate(kobj: *mut kobject, nidp: *mut i32) -> *mut hstate {
    for i in 0..HUGE_MAX_HSTATE {
        if hstate_kobjs[i] == kobj {
            if !nidp.is_null() { *nidp = NUMA_NO_NODE; }
            return &mut hstates[i];
        }
    }
    kobj_to_node_hstate(kobj, nidp)
}

unsafe fn nr_hugepages_show_common(kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut nid = 0;
    let h = kobj_to_hstate(kobj, &mut nid);
    let nr = if nid == NUMA_NO_NODE { (*h).nr_huge_pages } else { (*h).nr_huge_pages_node[nid as usize] };
    sysfs_emit(buf, "%lu\n", nr)
}

unsafe fn nr_hugepages_store_common(obey_mempolicy: bool, kobj: *mut kobject, buf: *const c_char, len: usize) -> ssize_t {
    let mut count = 0u64;
    let err = kstrtoul(buf, 10, &mut count);
    if err != 0 { return err; }
    let mut nid = 0;
    let h = kobj_to_hstate(kobj, &mut nid);
    __nr_hugepages_store_common(obey_mempolicy, h, nid, count, len)
}

unsafe fn nr_hugepages_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t { nr_hugepages_show_common(kobj, attr, buf) }
unsafe fn nr_hugepages_store(kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *const c_char, len: usize) -> ssize_t { nr_hugepages_store_common(false, kobj, buf, len) }

#[cfg(CONFIG_NUMA)]
unsafe fn nr_hugepages_mempolicy_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t { nr_hugepages_show_common(kobj, attr, buf) }
#[cfg(CONFIG_NUMA)]
unsafe fn nr_hugepages_mempolicy_store(kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *const c_char, len: usize) -> ssize_t { nr_hugepages_store_common(true, kobj, buf, len) }

unsafe fn nr_overcommit_hugepages_show(kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let h = kobj_to_hstate(kobj, core::ptr::null_mut()); sysfs_emit(buf, "%lu\n", (*h).nr_overcommit_huge_pages)
}
unsafe fn nr_overcommit_hugepages_store(kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *const c_char, count: usize) -> ssize_t {
    let h = kobj_to_hstate(kobj, core::ptr::null_mut());
    if hstate_is_gigantic_no_runtime(h) { return -EINVAL; }
    let mut input = 0u64; let err = kstrtoul(buf, 10, &mut input); if err != 0 { return err; }
    spin_lock_irq(&mut hugetlb_lock); (*h).nr_overcommit_huge_pages = input; spin_unlock_irq(&mut hugetlb_lock); count as ssize_t
}

unsafe fn free_hugepages_show(kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut nid = 0; let h = kobj_to_hstate(kobj, &mut nid); let n = if nid == NUMA_NO_NODE { (*h).free_huge_pages } else { (*h).free_huge_pages_node[nid as usize] }; sysfs_emit(buf, "%lu\n", n)
}
unsafe fn resv_hugepages_show(kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t { let h = kobj_to_hstate(kobj, core::ptr::null_mut()); sysfs_emit(buf, "%lu\n", (*h).resv_huge_pages) }
unsafe fn surplus_hugepages_show(kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut nid = 0; let h = kobj_to_hstate(kobj, &mut nid); let n = if nid == NUMA_NO_NODE { (*h).surplus_huge_pages } else { (*h).surplus_huge_pages_node[nid as usize] }; sysfs_emit(buf, "%lu\n", n)
}

unsafe fn demote_store(kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *const c_char, len: usize) -> ssize_t {
    let mut nr_demote = 0u64; let mut err = kstrtoul(buf, 10, &mut nr_demote); if err != 0 { return err; }
    let mut nid = 0; let h = kobj_to_hstate(kobj, &mut nid); let mut nodes_allowed = core::mem::zeroed();
    let n_mask = if nid != NUMA_NO_NODE { init_nodemask_of_node(&mut nodes_allowed, nid); &mut nodes_allowed } else { &mut node_states[N_MEMORY] };
    mutex_lock(&mut (*h).resize_lock); spin_lock_irq(&mut hugetlb_lock);
    while nr_demote != 0 {
        let available = if nid != NUMA_NO_NODE { (*h).free_huge_pages_node[nid as usize] } else { (*h).free_huge_pages } - (*h).resv_huge_pages;
        if available == 0 { break; }
        let rc = demote_pool_huge_page(h, n_mask, nr_demote); if rc < 0 { err = rc; break; } nr_demote -= rc as u64;
    }
    spin_unlock_irq(&mut hugetlb_lock); mutex_unlock(&mut (*h).resize_lock); if err != 0 { err } else { len as ssize_t }
}

unsafe fn demote_size_show(kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t { let h = kobj_to_hstate(kobj, core::ptr::null_mut()); let size = (PAGE_SIZE << (*h).demote_order) / SZ_1K; sysfs_emit(buf, "%lukB\n", size) }
unsafe fn demote_size_store(kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *const c_char, count: usize) -> ssize_t {
    let size = memparse(buf, core::ptr::null_mut()); let dh = size_to_hstate(size); if dh.is_null() { return -EINVAL; }
    let order = (*dh).order; if order < HUGETLB_PAGE_ORDER { return -EINVAL; } let h = kobj_to_hstate(kobj, core::ptr::null_mut()); if order >= (*h).order { return -EINVAL; }
    mutex_lock(&mut (*h).resize_lock); (*h).demote_order = order; mutex_unlock(&mut (*h).resize_lock); count as ssize_t
}

static mut hstate_attrs: [*mut attribute; 6] = [unsafe { &mut nr_hugepages_attr.attr }, unsafe { &mut nr_overcommit_hugepages_attr.attr }, unsafe { &mut free_hugepages_attr.attr }, unsafe { &mut resv_hugepages_attr.attr }, unsafe { &mut surplus_hugepages_attr.attr }, core::ptr::null_mut()];
static hstate_attr_group: attribute_group = attribute_group { attrs: unsafe { &hstate_attrs as *const _ as *mut _ } };
static mut hstate_demote_attrs: [*mut attribute; 3] = [unsafe { &mut demote_size_attr.attr }, unsafe { &mut demote_attr.attr }, core::ptr::null_mut()];
static hstate_demote_attr_group: attribute_group = attribute_group { attrs: unsafe { &hstate_demote_attrs as *const _ as *mut _ } };

unsafe fn hugetlb_sysfs_add_hstate(h: *mut hstate, parent: *mut kobject, objs: *mut *mut kobject, group: *const attribute_group) -> i32 {
    let hi = hstate_index(h); *objs.add(hi) = kobject_create_and_add((*h).name, parent); if (*objs.add(hi)).is_null() { return -ENOMEM; }
    let mut ret = sysfs_create_group(*objs.add(hi), group); if ret != 0 { kobject_put(*objs.add(hi)); *objs.add(hi) = core::ptr::null_mut(); return ret; }
    if (*h).demote_order != 0 { ret = sysfs_create_group(*objs.add(hi), &hstate_demote_attr_group); if ret != 0 { pr_warn!("HugeTLB unable to create demote interfaces for %s\n", (*h).name); sysfs_remove_group(*objs.add(hi), group); kobject_put(*objs.add(hi)); *objs.add(hi) = core::ptr::null_mut(); return ret; } } 0
}

#[cfg(CONFIG_NUMA)]
static mut hugetlb_sysfs_initialized: bool = false;
#[cfg(CONFIG_NUMA)]
#[repr(C)] struct node_hstate { hugepages_kobj: *mut kobject, hstate_kobjs: [*mut kobject; HUGE_MAX_HSTATE] }
#[cfg(CONFIG_NUMA)] static mut node_hstates: [node_hstate; MAX_NUMNODES] = [node_hstate { hugepages_kobj: core::ptr::null_mut(), hstate_kobjs: [core::ptr::null_mut(); HUGE_MAX_HSTATE] }; MAX_NUMNODES];
#[cfg(CONFIG_NUMA)] static mut per_node_hstate_attrs: [*mut attribute; 4] = [unsafe { &mut nr_hugepages_attr.attr }, unsafe { &mut free_hugepages_attr.attr }, unsafe { &mut surplus_hugepages_attr.attr }, core::ptr::null_mut()];
#[cfg(CONFIG_NUMA)] static per_node_hstate_attr_group: attribute_group = attribute_group { attrs: unsafe { &per_node_hstate_attrs as *const _ as *mut _ } };
#[cfg(CONFIG_NUMA)]
unsafe fn kobj_to_node_hstate(kobj: *mut kobject, nidp: *mut i32) -> *mut hstate { for nid in 0..nr_node_ids { for i in 0..HUGE_MAX_HSTATE { if node_hstates[nid as usize].hstate_kobjs[i] == kobj { if !nidp.is_null() { *nidp = nid; } return &mut hstates[i]; } } } BUG!(); core::ptr::null_mut() }

#[cfg(CONFIG_NUMA)] unsafe fn hugetlb_unregister_node(node: *mut node) { let nhs = &mut node_hstates[(*node).dev.id as usize]; if nhs.hugepages_kobj.is_null() { return; } for_each_hstate!(h, { let idx = hstate_index(h); let k = nhs.hstate_kobjs[idx]; if !k.is_null() { if (*h).demote_order != 0 { sysfs_remove_group(k, &hstate_demote_attr_group); } sysfs_remove_group(k, &per_node_hstate_attr_group); kobject_put(k); nhs.hstate_kobjs[idx] = core::ptr::null_mut(); } }); kobject_put(nhs.hugepages_kobj); nhs.hugepages_kobj = core::ptr::null_mut(); }
#[cfg(CONFIG_NUMA)] unsafe fn hugetlb_register_node(node: *mut node) { if !hugetlb_sysfs_initialized { return; } let nhs = &mut node_hstates[(*node).dev.id as usize]; if !nhs.hugepages_kobj.is_null() { return; } nhs.hugepages_kobj = kobject_create_and_add("hugepages", &mut (*node).dev.kobj); if nhs.hugepages_kobj.is_null() { return; } for_each_hstate!(h, { let err = hugetlb_sysfs_add_hstate(h, nhs.hugepages_kobj, nhs.hstate_kobjs.as_mut_ptr(), &per_node_hstate_attr_group); if err != 0 { pr_err!("HugeTLB: Unable to add hstate %s for node %d\n", (*h).name, (*node).dev.id); hugetlb_unregister_node(node); break; } }); }
#[cfg(CONFIG_NUMA)] unsafe fn hugetlb_register_all_nodes() { for_each_online_node!(nid, { hugetlb_register_node(node_devices[nid as usize]); }); }
#[cfg(not(CONFIG_NUMA))] unsafe fn kobj_to_node_hstate(_kobj: *mut kobject, nidp: *mut i32) -> *mut hstate { BUG!(); if !nidp.is_null() { *nidp = -1; } core::ptr::null_mut() }
#[cfg(not(CONFIG_NUMA))] unsafe fn hugetlb_register_all_nodes() {}

unsafe fn hugetlb_sysfs_init() { hugepages_kobj = kobject_create_and_add("hugepages", mm_kobj); if hugepages_kobj.is_null() { return; } for_each_hstate!(h, { let err = hugetlb_sysfs_add_hstate(h, hugepages_kobj, hstate_kobjs.as_mut_ptr(), &hstate_attr_group); if err != 0 { pr_err!("HugeTLB: Unable to add hstate %s\n", (*h).name); } }); #[cfg(CONFIG_NUMA)] { hugetlb_sysfs_initialized = true; } hugetlb_register_all_nodes(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
