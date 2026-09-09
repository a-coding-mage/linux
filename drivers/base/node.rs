// SPDX-License-Identifier: GPL-2.0
/* Basic Node interface support. Direct low-level translation of node.c. */

// Kernel headers and symbols referenced below are supplied by the surrounding
// kernel translation. Configuration conditionals are retained with cfg flags.

#[repr(C)]
pub struct NodeAccessNodes {
    pub dev: Device,
    pub list_node: ListHead,
    pub access: u32,
    #[cfg(feature = "CONFIG_HMEM_REPORTING")]
    pub coord: AccessCoordinate,
}

#[repr(C)]
pub struct NodeCacheInfo {
    pub dev: Device,
    pub node: ListHead,
    pub cache_attrs: NodeCacheAttrs,
}

#[repr(C)]
pub struct NodeAttr {
    pub attr: DeviceAttribute,
    pub state: NodeStates,
}

extern "C" {
    static node_subsys: BusType;
    static mut node_devices: [*mut Node; MAX_NUMNODES];
    static cpu_online_mask: CpuMask;
    static node_states: [NodeMask; NR_NODE_STATES];

    fn alloc_cpumask_var(mask: *mut CpuMaskVar, flags: u32) -> bool;
    fn free_cpumask_var(mask: CpuMaskVar);
    fn cpumask_of_node(nid: i32) -> *const CpuMask;
    fn cpumask_and(dst: CpuMaskVar, a: *const CpuMask, b: *const CpuMask);
    fn cpumap_print_bitmask_to_buf(buf: *mut i8, mask: CpuMaskVar, off: LoFF, count: usize) -> Isize;
    fn cpumap_print_list_to_buf(buf: *mut i8, mask: CpuMaskVar, off: LoFF, count: usize) -> Isize;
    fn kobj_to_dev(kobj: *mut KObject) -> *mut Device;
    fn to_node(dev: *mut Device) -> *mut Node;
    fn device_unregister(dev: *mut Device);
    fn device_register(dev: *mut Device) -> i32;
    fn device_initialize(dev: *mut Device);
    fn device_add(dev: *mut Device) -> i32;
    fn put_device(dev: *mut Device);
    fn dev_set_name(dev: *mut Device, fmt: *const i8, ...) -> i32;
    fn dev_name(dev: *mut Device) -> *const i8;
    fn kobject_name(kobj: *mut KObject) -> *const i8;
    fn kfree_const(p: *const i8);
    fn kfree(p: *mut core::ffi::c_void);
    fn kzalloc_obj<T>() -> *mut T;
    fn pm_runtime_no_callbacks(dev: *mut Device);
    fn list_for_each_entry_safe<T>(a: *mut *mut T, b: *mut *mut T, head: *mut ListHead, member: usize);
    fn list_for_each_entry<T>(a: *mut *mut T, head: *mut ListHead, member: usize);
    fn list_add_tail(entry: *mut ListHead, head: *mut ListHead);
    fn list_del(entry: *mut ListHead);
    fn INIT_LIST_HEAD(head: *mut ListHead);
    fn sysfs_emit(buf: *mut i8, fmt: *const i8, ...) -> Isize;
    fn sysfs_emit_at(buf: *mut i8, at: i32, fmt: *const i8, ...) -> Isize;
    fn sysfs_notify(kobj: *mut KObject, dir: *const i8, attr: *const i8);
    fn sysfs_add_file_to_group(kobj: *mut KObject, attr: *mut Attribute, group: *const i8) -> i32;
    fn sysfs_add_link_to_group(kobj: *mut KObject, group: *const i8, target: *mut KObject, name: *const i8) -> i32;
    fn sysfs_remove_link_from_group(kobj: *mut KObject, group: *const i8, name: *const i8);
    fn sysfs_create_link(kobj: *mut KObject, target: *mut KObject, name: *const i8) -> i32;
    fn sysfs_create_link_nowarn(kobj: *mut KObject, target: *mut KObject, name: *const i8) -> i32;
    fn sysfs_remove_link(kobj: *mut KObject, name: *const i8);
    fn register_node_notifier(nb: *mut NotifierBlock) -> i32;
    fn unregister_node_notifier(nb: *mut NotifierBlock);
    fn blocking_notifier_call_chain(chain: *mut BlockingNotifierHead, val: u64, v: *mut core::ffi::c_void) -> i32;
    fn blocking_notifier_chain_register(chain: *mut BlockingNotifierHead, nb: *mut NotifierBlock) -> i32;
    fn blocking_notifier_chain_unregister(chain: *mut BlockingNotifierHead, nb: *mut NotifierBlock) -> i32;
    fn mempolicy_set_node_perf(nid: u32, coord: *mut AccessCoordinate) -> i32;
    fn node_online(nid: u32) -> bool;
    fn node_page_state_pages(pgdat: *mut PgListData, item: i32) -> u64;
    fn node_page_state(pgdat: *mut PgListData, item: i32) -> u64;
    fn sum_zone_node_page_state(nid: u32, item: i32) -> u64;
    fn sum_zone_numa_event_state(nid: i32, item: i32) -> u64;
    fn fold_vm_numa_events();
    fn zone_stat_name(i: i32) -> *const i8;
    fn numa_stat_name(i: i32) -> *const i8;
    fn node_stat_name(i: i32) -> *const i8;
    fn vmstat_item_print_in_thp(i: i32) -> bool;
    fn node_distance(a: i32, b: i32) -> i32;
    fn get_cpu_device(cpu: u32) -> *mut Device;
    fn cpu_to_node(cpu: i32) -> i32;
    fn hugetlb_register_node(node: *mut Node);
    fn hugetlb_unregister_node(node: *mut Node);
    fn compaction_register_node(node: *mut Node);
    fn compaction_unregister_node(node: *mut Node);
    fn reclaim_register_node(node: *mut Node);
    fn reclaim_unregister_node(node: *mut Node);
    fn hugetlb_report_node_meminfo(buf: *mut i8, len: i32, nid: i32) -> i32;
    fn si_meminfo_node(info: *mut SysInfo, nid: i32);
    fn NODE_DATA(nid: i32) -> *mut PgListData;
    fn subsys_system_register(bus: *const BusType, groups: *const *const AttributeGroup) -> i32;
    fn panic(fmt: *const i8, ...);
}

#[cfg(feature = "CONFIG_MEMORY_HOTPLUG")]
static mut node_chain: BlockingNotifierHead = BlockingNotifierHead::new();

#[cfg(feature = "CONFIG_MEMORY_HOTPLUG")]
#[no_mangle]
pub unsafe extern "C" fn register_node_notifier_export(nb: *mut NotifierBlock) -> i32 {
    blocking_notifier_chain_register(&mut node_chain, nb)
}

#[cfg(feature = "CONFIG_MEMORY_HOTPLUG")]
#[no_mangle]
pub unsafe extern "C" fn unregister_node_notifier_export(nb: *mut NotifierBlock) {
    blocking_notifier_chain_unregister(&mut node_chain, nb);
}

#[cfg(feature = "CONFIG_MEMORY_HOTPLUG")]
#[no_mangle]
pub unsafe extern "C" fn node_notify(val: u64, v: *mut core::ffi::c_void) -> i32 {
    blocking_notifier_call_chain(&mut node_chain, val, v)
}

unsafe fn node_remove_accesses(node: *mut Node) {
    let mut c: *mut NodeAccessNodes = core::ptr::null_mut();
    let mut next: *mut NodeAccessNodes = core::ptr::null_mut();
    list_for_each_entry_safe(&mut c, &mut next, &mut (*node).access_list, 0);
    while !c.is_null() {
        list_del(&mut (*c).list_node);
        device_unregister(&mut (*c).dev);
        c = next;
    }
}

unsafe extern "C" fn node_access_release(dev: *mut Device) {
    kfree(dev as *mut core::ffi::c_void);
}

unsafe fn node_init_node_access(node: *mut Node, access: AccessCoordinateClass) -> *mut NodeAccessNodes {
    let mut p: *mut NodeAccessNodes = core::ptr::null_mut();
    list_for_each_entry(&mut p, &mut (*node).access_list, 0);
    while !p.is_null() {
        if (*p).access == access as u32 { return p; }
        break;
    }
    let access_node = kzalloc_obj::<NodeAccessNodes>();
    if access_node.is_null() { return core::ptr::null_mut(); }
    (*access_node).access = access as u32;
    (*access_node).dev.parent = &mut (*node).dev;
    (*access_node).dev.release = Some(node_access_release);
    if dev_set_name(&mut (*access_node).dev, b"access%u\0".as_ptr() as *const i8, access as u32) != 0 {
        kfree(access_node as *mut core::ffi::c_void); return core::ptr::null_mut();
    }
    if device_register(&mut (*access_node).dev) != 0 {
        kfree_const((*access_node).dev.kobj.name); kfree(access_node as *mut core::ffi::c_void); return core::ptr::null_mut();
    }
    pm_runtime_no_callbacks(&mut (*access_node).dev);
    list_add_tail(&mut (*access_node).list_node, &mut (*node).access_list);
    access_node
}

#[cfg(feature = "CONFIG_HMEM_REPORTING")]
#[no_mangle]
pub unsafe extern "C" fn node_set_perf_attrs(nid: u32, coord: *mut AccessCoordinate, access: AccessCoordinateClass) {
    if !node_online(nid) { return; }
    let c = node_init_node_access(node_devices[nid as usize], access);
    if c.is_null() { return; }
    (*c).coord = *coord;
    if access == AccessCoordinateClass::Cpu && mempolicy_set_node_perf(nid, coord) != 0 { }
}

#[cfg(feature = "CONFIG_HMEM_REPORTING")]
#[no_mangle]
pub unsafe extern "C" fn node_update_perf_attrs(nid: u32, coord: *mut AccessCoordinate, access: AccessCoordinateClass) {
    if !node_online(nid) { return; }
    let node = node_devices[nid as usize];
    let mut p: *mut NodeAccessNodes = core::ptr::null_mut();
    list_for_each_entry(&mut p, &mut (*node).access_list, 0);
    while !p.is_null() {
        if (*p).access == access as u32 { (*p).coord = *coord; break; }
        break;
    }
    if access == AccessCoordinateClass::Cpu && mempolicy_set_node_perf(nid, coord) != 0 { }
}

unsafe extern "C" fn node_cache_release(dev: *mut Device) { kfree(dev as *mut core::ffi::c_void); }
unsafe extern "C" fn node_cacheinfo_release(dev: *mut Device) { kfree(dev as *mut core::ffi::c_void); }

#[cfg(feature = "CONFIG_MEMORY_HOTPLUG")]
unsafe fn node_init_caches(nid: u32) { INIT_LIST_HEAD(&mut (*node_devices[nid as usize]).cache_attrs); }
#[cfg(not(feature = "CONFIG_MEMORY_HOTPLUG"))]
unsafe fn node_init_caches(_nid: u32) {}

#[cfg(feature = "CONFIG_MEMORY_HOTPLUG")]
unsafe fn node_remove_caches(node: *mut Node) {
    if (*node).cache_dev.is_null() { return; }
    device_unregister((*node).cache_dev);
}
#[cfg(not(feature = "CONFIG_MEMORY_HOTPLUG"))]
unsafe fn node_remove_caches(_node: *mut Node) {}

unsafe extern "C" fn node_device_release(dev: *mut Device) { kfree(dev as *mut core::ffi::c_void); }

#[no_mangle]
pub static mut node_devices_export: [*mut Node; MAX_NUMNODES] = [core::ptr::null_mut(); MAX_NUMNODES];

#[no_mangle]
pub unsafe extern "C" fn register_cpu_under_node(cpu: u32, nid: u32) -> i32 {
    if !node_online(nid) { return 0; }
    let obj = get_cpu_device(cpu); if obj.is_null() { return 0; }
    let ret = sysfs_create_link(&mut (*node_devices[nid as usize]).dev.kobj, &mut (*obj).kobj, kobject_name(&mut (*obj).kobj));
    if ret != 0 { return ret; }
    sysfs_create_link(&mut (*obj).kobj, &mut (*node_devices[nid as usize]).dev.kobj, kobject_name(&mut (*node_devices[nid as usize]).dev.kobj))
}

#[no_mangle]
pub unsafe extern "C" fn unregister_cpu_under_node(cpu: u32, nid: u32) -> i32 {
    if !node_online(nid) { return 0; }
    let obj = get_cpu_device(cpu); if obj.is_null() { return 0; }
    sysfs_remove_link(&mut (*node_devices[nid as usize]).dev.kobj, kobject_name(&mut (*obj).kobj));
    sysfs_remove_link(&mut (*obj).kobj, kobject_name(&mut (*node_devices[nid as usize]).dev.kobj));
    0
}

#[no_mangle]
pub unsafe extern "C" fn register_node(nid: i32) -> i32 {
    let node = kzalloc_obj::<Node>(); if node.is_null() { return -12; }
    INIT_LIST_HEAD(&mut (*node).access_list);
    (*node).dev.id = nid;
    (*node).dev.bus = &node_subsys as *const BusType as *mut BusType;
    (*node).dev.release = Some(node_device_release);
    let error = device_register(&mut (*node).dev);
    if error != 0 { put_device(&mut (*node).dev); return error; }
    node_devices[nid as usize] = node;
    hugetlb_register_node(node); compaction_register_node(node); reclaim_register_node(node);
    node_init_caches(nid as u32);
    node_devices_export[nid as usize] = node;
    0
}

#[no_mangle]
pub unsafe extern "C" fn unregister_node(nid: i32) {
    let node = node_devices[nid as usize]; if node.is_null() { return; }
    hugetlb_unregister_node(node); compaction_unregister_node(node); reclaim_unregister_node(node);
    node_remove_accesses(node); node_remove_caches(node); device_unregister(&mut (*node).dev);
    node_devices[nid as usize] = core::ptr::null_mut();
}

#[no_mangle]
pub unsafe extern "C" fn node_dev_init() {
    let ret = subsys_system_register(&node_subsys, core::ptr::null());
    if ret != 0 { panic(b"node_dev_init() failed to register subsystem: %d\n\0".as_ptr() as *const i8, ret); }
}

// Opaque kernel types and constants are intentionally referenced from the
// surrounding translation unit; no dependency implementations are invented.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
