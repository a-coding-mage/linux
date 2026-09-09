// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OPAL IMC interface detection driver
 * Supported on POWERNV platform
 *
 * Copyright (C) 2017 Madhavan Srinivasan, IBM Corporation.
 * Copyright (C) 2017 Anju T Sudhakar, IBM Corporation.
 * Copyright (C) 2017 Hemant K Shaw, IBM Corporation.
 */

// Kernel dependencies and build-time definitions are supplied by the surrounding tree.

use core::ffi::c_void;

extern "C" {
    static mut imc_debugfs_parent: *mut dentry;
    static mut arch_debugfs_dir: *mut dentry;
    static cpu_online_mask: cpumask;
    static nr_cpu_ids: i32;

    fn cpu_to_be64(value: u64) -> u64;
    fn debugfs_create_dir(name: *const i8, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_file_unsafe(name: *const i8, mode: umode_t, parent: *mut dentry,
                                  data: *mut c_void, fops: *const c_void) -> *mut dentry;
    fn of_property_read_u32(node: *mut device_node, name: *const i8, value: *mut u32) -> i32;
    fn of_property_count_u32_elems(node: *mut device_node, name: *const i8) -> i32;
    fn of_property_read_u32_array(node: *mut device_node, name: *const i8,
                                  values: *mut u32, count: i32) -> i32;
    fn of_property_read_u64_array(node: *mut device_node, name: *const i8,
                                  values: *mut u64, count: i32) -> i32;
    fn kcalloc(count: usize, size: usize, flags: u32) -> *mut c_void;
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn phys_to_virt(address: u64) -> *mut c_void;
    fn kzalloc_objs<T>(old: T, count: usize) -> T;
    fn init_imc_pmu(node: *mut device_node, pmu: *mut imc_pmu, index: i32) -> i32;
    fn opal_imc_counters_stop(counter_type: u32, processor: u32) -> i32;
    fn get_hard_smp_processor_id(cpu: i32) -> u32;
    fn cpumask_of_node(node: i32) -> *const cpumask;
    fn cpumask_first_and(a: *const cpumask, b: *const cpumask) -> i32;
    fn cpu_first_thread_sibling(cpu: i32) -> i32;
    fn is_kdump_kernel() -> bool;
    fn unregister_thread_imc();
}

#[repr(C)]
struct dentry;
#[repr(C)]
struct device_node;
#[repr(C)]
struct platform_device { dev: device };
#[repr(C)]
struct device { of_node: *mut device_node }
#[repr(C)]
struct cpumask;
#[repr(C)]
struct imc_pmu { domain: i32, counter_mem_size: u32, mem_info: *mut imc_mem_info, pmu: imc_pmu_data, imc_counter_mmaped: bool }
#[repr(C)]
struct imc_mem_info { id: u32, vbase: *mut c_void }
#[repr(C)]
struct imc_pmu_data { name: *mut i8 }
type umode_t = u16;

const ENODEV: i32 = 19;
const ENOMEM: i32 = 12;
const GFP_KERNEL: u32 = 0;
const IMC_DOMAIN_NEST: i32 = 0;
const IMC_DOMAIN_CORE: i32 = 1;
const IMC_DOMAIN_THREAD: i32 = 2;
const IMC_DOMAIN_TRACE: i32 = 3;
const OPAL_IMC_COUNTERS_NEST: u32 = 0;
const OPAL_IMC_COUNTERS_CORE: u32 = 1;

unsafe fn imc_mem_get(data: *mut c_void, val: *mut u64) -> i32 {
    *val = cpu_to_be64(*(data as *mut u64));
    0
}

unsafe fn imc_mem_set(data: *mut c_void, val: u64) -> i32 {
    *(data as *mut u64) = cpu_to_be64(val);
    0
}

unsafe fn imc_debugfs_create_x64(name: *const i8, mode: umode_t,
                                 parent: *mut dentry, value: *mut u64) {
    debugfs_create_file_unsafe(name, mode, parent, value as *mut c_void,
                                &fops_imc_x64 as *const _ as *const c_void);
}

// DEFINE_DEBUGFS_ATTRIBUTE(fops_imc_x64, imc_mem_get, imc_mem_set, "0x%016llx\n");
static fops_imc_x64: u8 = 0;

unsafe fn export_imc_mode_and_cmd(node: *mut device_node, pmu_ptr: *mut imc_pmu) {
    static mut loc: u64 = 0;
    static mut imc_mode_addr: *mut u64 = core::ptr::null_mut();
    static mut imc_cmd_addr: *mut u64 = core::ptr::null_mut();
    let mut mode = [0i8; 16];
    let mut cmd = [0i8; 16];
    let mut cb_offset: u32 = 0;
    let mut ptr = (*pmu_ptr).mem_info;

    imc_debugfs_parent = debugfs_create_dir(b"imc\0".as_ptr() as *const i8, arch_debugfs_dir);
    if of_property_read_u32(node, b"cb_offset\0".as_ptr() as *const i8, &mut cb_offset) != 0 {
        cb_offset = IMC_CNTL_BLK_OFFSET;
    }
    while !ptr.is_null() && !(*ptr).vbase.is_null() {
        loc = (*ptr).vbase as u64 + cb_offset as u64;
        imc_mode_addr = (loc + IMC_CNTL_BLK_MODE_OFFSET as u64) as *mut u64;
        snprintf(mode.as_mut_ptr(), mode.len(), b"imc_mode_%d\0".as_ptr() as *const i8, (*ptr).id);
        imc_debugfs_create_x64(mode.as_ptr(), 0o600, imc_debugfs_parent, imc_mode_addr);
        imc_cmd_addr = (loc + IMC_CNTL_BLK_CMD_OFFSET as u64) as *mut u64;
        snprintf(cmd.as_mut_ptr(), cmd.len(), b"imc_cmd_%d\0".as_ptr() as *const i8, (*ptr).id);
        imc_debugfs_create_x64(cmd.as_ptr(), 0o600, imc_debugfs_parent, imc_cmd_addr);
        ptr = ptr.add(1);
    }
}

extern "C" { fn snprintf(buf: *mut i8, size: usize, fmt: *const i8, ...) -> i32; }

unsafe fn imc_get_mem_addr_nest(node: *mut device_node, pmu_ptr: *mut imc_pmu, offset: u32) -> i32 {
    let nr_chips = of_property_count_u32_elems(node, b"chip-id\0".as_ptr() as *const i8);
    if nr_chips <= 0 { return -ENODEV; }
    let base_addr_arr = kcalloc(nr_chips as usize, core::mem::size_of::<u64>(), GFP_KERNEL) as *mut u64;
    if base_addr_arr.is_null() { return -ENOMEM; }
    let chipid_arr = kcalloc(nr_chips as usize, core::mem::size_of::<u32>(), GFP_KERNEL) as *mut u32;
    if chipid_arr.is_null() { kfree(base_addr_arr as *mut c_void); return -ENOMEM; }
    if of_property_read_u32_array(node, b"chip-id\0".as_ptr() as *const i8, chipid_arr, nr_chips) != 0 { kfree(base_addr_arr as *mut c_void); kfree(chipid_arr as *mut c_void); return -1; }
    if of_property_read_u64_array(node, b"base-addr\0".as_ptr() as *const i8, base_addr_arr, nr_chips) != 0 { kfree(base_addr_arr as *mut c_void); kfree(chipid_arr as *mut c_void); return -1; }
    (*pmu_ptr).mem_info = kzalloc_objs((*pmu_ptr).mem_info, nr_chips as usize + 1);
    if (*pmu_ptr).mem_info.is_null() { kfree(base_addr_arr as *mut c_void); kfree(chipid_arr as *mut c_void); return -1; }
    for i in 0..nr_chips as usize { (*pmu_ptr).mem_info.add(i).write(imc_mem_info { id: *chipid_arr.add(i), vbase: phys_to_virt(*base_addr_arr.add(i) + offset as u64) }); }
    (*pmu_ptr).imc_counter_mmaped = true;
    kfree(base_addr_arr as *mut c_void); kfree(chipid_arr as *mut c_void); 0
}

unsafe fn imc_pmu_create(parent: *mut device_node, pmu_index: i32, domain: i32) -> *mut imc_pmu {
    if domain < 0 { return core::ptr::null_mut(); }
    let pmu_ptr = kzalloc(core::mem::size_of::<imc_pmu>(), GFP_KERNEL) as *mut imc_pmu;
    if pmu_ptr.is_null() { return core::ptr::null_mut(); }
    (*pmu_ptr).domain = domain;
    let mut ret = of_property_read_u32(parent, b"size\0".as_ptr() as *const i8, &mut (*pmu_ptr).counter_mem_size);
    if ret != 0 { kfree(pmu_ptr as *mut c_void); return core::ptr::null_mut(); }
    let mut offset = 0;
    if of_property_read_u32(parent, b"offset\0".as_ptr() as *const i8, &mut offset) == 0 && imc_get_mem_addr_nest(parent, pmu_ptr, offset) != 0 { kfree(pmu_ptr as *mut c_void); return core::ptr::null_mut(); }
    ret = init_imc_pmu(parent, pmu_ptr, pmu_index);
    if ret != 0 { kfree((*pmu_ptr).pmu.name as *mut c_void); if (*pmu_ptr).domain == IMC_DOMAIN_NEST { kfree((*pmu_ptr).mem_info as *mut c_void); } kfree(pmu_ptr as *mut c_void); return core::ptr::null_mut(); }
    pmu_ptr
}

// Kernel CPU/node iteration macros retain their original semantics in the containing kernel.
unsafe fn disable_nest_pmu_counters() { /* cpus_read_lock(); for_each_node_with_cpus(nid) { ... } cpus_read_unlock(); */ }
unsafe fn disable_core_pmu_counters() { /* cpus_read_lock(); for_each_online_cpu(cpu) { ... } cpus_read_unlock(); */ }

#[no_mangle]
pub unsafe extern "C" fn get_max_nest_dev() -> u32 {
    let mut pmu_units = 0u32;
    let mut node: *mut device_node = core::ptr::null_mut();
    // for_each_compatible_node(node, NULL, IMC_DTB_UNIT_COMPAT)
    while !node.is_null() {
        let mut unit_type = 0u32;
        if of_property_read_u32(node, b"type\0".as_ptr() as *const i8, &mut unit_type) == 0 && unit_type == IMC_TYPE_CHIP {
            pmu_units += 1;
        }
        break;
    }
    pmu_units
}

unsafe fn opal_imc_counters_probe(pdev: *mut platform_device) -> i32 {
    let mut imc_dev = (*pdev).dev.of_node;
    let mut pmu_count = 0i32;
    let mut core_imc_reg = false;
    let mut thread_imc_reg = false;
    if is_kdump_kernel() { disable_nest_pmu_counters(); disable_core_pmu_counters(); return -ENODEV; }
    // for_each_compatible_node(imc_dev, NULL, IMC_DTB_UNIT_COMPAT)
    while !imc_dev.is_null() {
        let mut unit_type = 0u32;
        if of_property_read_u32(imc_dev, b"type\0".as_ptr() as *const i8, &mut unit_type) != 0 { break; }
        let domain = match unit_type { IMC_TYPE_CHIP => IMC_DOMAIN_NEST, IMC_TYPE_CORE => IMC_DOMAIN_CORE, IMC_TYPE_THREAD => IMC_DOMAIN_THREAD, IMC_TYPE_TRACE => IMC_DOMAIN_TRACE, _ => -1 };
        let pmu = imc_pmu_create(imc_dev, pmu_count, domain);
        if !pmu.is_null() {
            if domain == IMC_DOMAIN_NEST { if imc_debugfs_parent.is_null() { export_imc_mode_and_cmd(imc_dev, pmu); } pmu_count += 1; }
            if domain == IMC_DOMAIN_CORE { core_imc_reg = true; }
            if domain == IMC_DOMAIN_THREAD { thread_imc_reg = true; }
        }
        break;
    }
    if !core_imc_reg && thread_imc_reg { unregister_thread_imc(); }
    0
}

unsafe fn opal_imc_counters_shutdown(_pdev: *mut platform_device) {
    disable_nest_pmu_counters();
    disable_core_pmu_counters();
}

// static opal_imc_match = [{ compatible: IMC_DTB_COMPAT }, {}];
// static opal_imc_driver = platform_driver { name: "opal-imc-counters", ... };
// builtin_platform_driver(opal_imc_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
