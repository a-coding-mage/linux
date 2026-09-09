// SPDX-License-Identifier: GPL-2.0
/* Copyright(c) 2016-2019 Intel Corporation. All rights reserved. */
// Linux kernel dependencies are supplied by the surrounding translation unit.

const MEMTIER_DEFAULT_DAX_ADISTANCE: i32 = MEMTIER_ADISTANCE_DRAM * 5;

static mut kmem_name: *const c_char = core::ptr::null();
static mut any_hotremove_failed: bool = false;

unsafe fn dax_kmem_range(dev_dax: *mut dev_dax, i: i32, r: *mut range) -> i32 {
    let dax_range = &(*dev_dax).ranges[i as usize];
    let range = &dax_range.range;
    *r = memory_block_aligned_range(range);
    if (*r).start >= (*r).end {
        (*r).start = range.start;
        (*r).end = range.end;
        return -ENOSPC;
    }
    0
}

#[repr(C)]
struct dax_kmem_data {
    res_name: *const c_char,
    mgid: i32,
    state: i32,
    lock: mutex,
    res: [*mut resource; 0],
}

static mut kmem_memory_type_lock: mutex = DEFINE_MUTEX!();
static mut kmem_memory_types: list_head = LIST_HEAD!();

unsafe fn kmem_find_alloc_memory_type(adist: i32) -> *mut memory_dev_type {
    let _guard = guard_mutex(&mut kmem_memory_type_lock);
    mt_find_alloc_memory_type(adist, &mut kmem_memory_types)
}

unsafe fn kmem_put_memory_types() {
    let _guard = guard_mutex(&mut kmem_memory_type_lock);
    mt_put_memory_types(&mut kmem_memory_types);
}

unsafe fn dax_kmem_state_is_online(state: i32) -> bool {
    state == MMOP_ONLINE || state == MMOP_ONLINE_KERNEL || state == MMOP_ONLINE_MOVABLE
}

unsafe fn dax_kmem_do_hotplug(dev_dax: *mut dev_dax, data: *mut dax_kmem_data, online_type: i32) -> i32 {
    let dev = &mut (*dev_dax).dev;
    let mut added = 0;
    if dax_kmem_state_is_online((*data).state) || online_type < MMOP_OFFLINE || online_type > MMOP_ONLINE_MOVABLE { return -EINVAL; }
    for i in 0..(*dev_dax).nr_range {
        let mut range = range::default();
        if dax_kmem_range(dev_dax, i, &mut range) != 0 || (*data).res[i as usize].is_null() { continue; }
        let mut flags = MHP_NID_IS_MGID;
        if (*dev_dax).memmap_on_memory { flags |= MHP_MEMMAP_ON_MEMORY; }
        let rc = __add_memory_driver_managed((*data).mgid, range.start, range_len(&range), kmem_name, flags, online_type);
        if rc != 0 {
            dev_warn(dev, "mapping%d: %#llx-%#llx memory add failed\n", i, range.start, range.end);
            remove_resource((*data).res[i as usize]); kfree((*data).res[i as usize]); (*data).res[i as usize] = core::ptr::null_mut();
            if added != 0 { continue; }
            return rc;
        }
        added += 1;
    }
    added
}

unsafe fn dax_kmem_init_resources(dev_dax: *mut dev_dax, data: *mut dax_kmem_data) -> i32 {
    let dev = &mut (*dev_dax).dev;
    let mut mapped = 0;
    for i in 0..(*dev_dax).nr_range {
        let mut range = range::default();
        if dax_kmem_range(dev_dax, i, &mut range) != 0 || !(*data).res[i as usize].is_null() { continue; }
        let res = request_mem_region(range.start, range_len(&range), (*data).res_name);
        if res.is_null() { dev_warn(dev, "mapping%d: %#llx-%#llx could not reserve region\n", i, range.start, range.end); if mapped != 0 { continue; } return -EBUSY; }
        (*data).res[i as usize] = res; (*res).flags = IORESOURCE_SYSTEM_RAM; mapped += 1;
    }
    mapped
}

#[cfg(feature = "CONFIG_MEMORY_HOTREMOVE")]
unsafe fn dax_kmem_do_hotremove(dev_dax: *mut dev_dax, data: *mut dax_kmem_data) -> i32 {
    let dev = &mut (*dev_dax).dev;
    let ranges = kmalloc_objs::<range>((*dev_dax).nr_range as usize);
    if ranges.is_null() { return -ENOMEM; }
    let mut nr = 0;
    for i in 0..(*dev_dax).nr_range { if (*data).res[i as usize].is_null() { continue; } let mut r = range::default(); if dax_kmem_range(dev_dax, i, &mut r) == 0 { *ranges.add(nr as usize) = r; nr += 1; } }
    if nr == 0 { kfree(ranges); return 0; }
    let rc = offline_and_remove_memory_ranges(ranges, nr); kfree(ranges);
    if rc != 0 { dev_err(dev, "hotremove failed, device left online: %d\n", rc); return rc; }
    for i in 0..(*dev_dax).nr_range { if !(*data).res[i as usize].is_null() { remove_resource((*data).res[i as usize]); kfree((*data).res[i as usize]); (*data).res[i as usize] = core::ptr::null_mut(); } }
    0
}
#[cfg(not(feature = "CONFIG_MEMORY_HOTREMOVE"))]
unsafe fn dax_kmem_do_hotremove(_: *mut dev_dax, _: *mut dax_kmem_data) -> i32 { -EBUSY }

unsafe fn dax_kmem_cleanup_resources(dev_dax: *mut dev_dax, data: *mut dax_kmem_data) {
    if ((*data).state != DAX_KMEM_UNPLUGGED && (*data).state != MMOP_OFFLINE) && WARN(true, c_str!("Hotplug memory regions stuck online until reboot")) { return; }
    for i in 0..(*dev_dax).nr_range { if !(*data).res[i as usize].is_null() { remove_resource((*data).res[i as usize]); kfree((*data).res[i as usize]); (*data).res[i as usize] = core::ptr::null_mut(); } }
}

unsafe fn state_show(dev: *mut device, _: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let data = dev_get_drvdata(dev) as *mut dax_kmem_data;
    let s = if (*data).state == DAX_KMEM_UNPLUGGED { c_str!("unplugged") } else { mhp_online_type_to_str((*data).state) };
    sysfs_emit(buf, c_str!("%s\n"), if s.is_null() { c_str!("unknown") } else { s })
}

unsafe fn state_store(dev: *mut device, _: *mut device_attribute, buf: *const c_char, len: usize) -> ssize_t {
    let dd = to_dev_dax(dev); let data = dev_get_drvdata(dev) as *mut dax_kmem_data; let typ = dax_kmem_parse_state(buf); if typ < DAX_KMEM_UNPLUGGED { return typ as ssize_t; }
    let _g = guard_mutex(&mut (*data).lock); if (*data).state == typ { return len as ssize_t; }
    if typ == DAX_KMEM_UNPLUGGED { let rc = dax_kmem_do_hotremove(dd, data); if rc != 0 { return rc as ssize_t; } (*data).state = typ; return len as ssize_t; }
    if (*data).state != DAX_KMEM_UNPLUGGED { return -EBUSY as ssize_t; }
    let rc = dax_kmem_init_resources(dd, data); if rc < 0 { return rc as ssize_t; }
    let rc = dax_kmem_do_hotplug(dd, data, typ); if rc < 0 { dax_kmem_cleanup_resources(dd, data); return rc as ssize_t; } (*data).state = typ; len as ssize_t
}

unsafe fn dev_dax_kmem_probe(dev_dax: *mut dev_dax) -> i32 {
    let dev = &mut (*dev_dax).dev; let node = (*dev_dax).target_node; if node < 0 { dev_warn(dev, "rejecting DAX region with invalid node: %d\n", node); return -EINVAL; }
    let mut adist = MEMTIER_DEFAULT_DAX_ADISTANCE; mt_calc_adistance(node, &mut adist); let mt = kmem_find_alloc_memory_type(adist); if IS_ERR(mt) { return PTR_ERR(mt); }
    let mut total = 0; for i in 0..(*dev_dax).nr_range { let mut r = range::default(); if dax_kmem_range(dev_dax, i, &mut r) == 0 { total += range_len(&r); } }
    if total == 0 { return -EINVAL; } init_node_memory_type(node, mt);
    let data = kzalloc_flex::<dax_kmem_data>((*dev_dax).nr_range as usize); if data.is_null() { clear_node_memory_type(node, mt); return -ENOMEM; }
    (*data).res_name = kstrdup(dev_name(dev), GFP_KERNEL); (*data).mgid = memory_group_register_static(node, PFN_UP(total)); (*data).state = DAX_KMEM_UNPLUGGED; mutex_init(&mut (*data).lock); dev_set_drvdata(dev, data);
    let rc = dax_kmem_init_resources(dev_dax, data); if rc < 0 { return rc; } let rc = dax_kmem_do_hotplug(dev_dax, data, mhp_get_default_online_type()); if rc < 0 { dax_kmem_cleanup_resources(dev_dax, data); return rc; } (*data).state = mhp_get_default_online_type(); 0
}

unsafe fn dev_dax_kmem_remove(dev_dax: *mut dev_dax) { let dev = &mut (*dev_dax).dev; let data = dev_get_drvdata(dev) as *mut dax_kmem_data; if dax_kmem_do_hotremove(dev_dax, data) != 0 { dev_err(dev, "Hotplug regions stuck online until reboot\n"); any_hotremove_failed = true; return; } memory_group_unregister((*data).mgid); kfree((*data).res_name as *mut _); kfree(data); dev_set_drvdata(dev, core::ptr::null_mut()); }

// Remaining device callbacks and module registration retain the C driver's interfaces.
// The declarations below intentionally use the kernel-provided types and helpers.
unsafe fn dax_kmem_parse_state(buf: *const c_char) -> i32 { if sysfs_streq(buf, c_str!("unplugged")) { DAX_KMEM_UNPLUGGED } else { let t = mhp_online_type_from_str(buf); if t == MMOP_OFFLINE { -EINVAL } else { t } } }

static DEVICE_ATTR_RW!(state);
static mut dev_dax_kmem_driver: dax_device_driver = dax_device_driver { probe: Some(dev_dax_kmem_probe), remove: Some(dev_dax_kmem_remove), type_: DAXDRV_KMEM_TYPE, drv: driver { dev_groups: dev_dax_kmem_groups } };

unsafe fn dax_kmem_init() -> i32 { kmem_name = kstrdup_const(c_str!("System RAM (kmem)"), GFP_KERNEL); if kmem_name.is_null() { return -ENOMEM; } let rc = dax_driver_register(&mut dev_dax_kmem_driver); if rc != 0 { kmem_put_memory_types(); kfree_const(kmem_name); } rc }
unsafe fn dax_kmem_exit() { dax_driver_unregister(&mut dev_dax_kmem_driver); if !any_hotremove_failed { kfree_const(kmem_name); } kmem_put_memory_types(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
