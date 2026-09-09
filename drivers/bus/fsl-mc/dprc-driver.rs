// SPDX-License-Identifier: GPL-2.0
/*
 * Freescale data path resource container (DPRC) driver
 *
 * Copyright (C) 2014-2016 Freescale Semiconductor, Inc.
 * Copyright 2019-2020 NXP
 * Author: German Rivera <German.Rivera@freescale.com>
 */

const FSL_MC_DPRC_DRIVER_NAME: &str = "fsl_mc_dprc";

#[repr(C)]
struct fsl_mc_child_objs {
    child_count: i32,
    child_array: *mut fsl_mc_obj_desc,
}

unsafe fn fsl_mc_device_match(
    mc_dev: *const fsl_mc_device,
    obj_desc: *const fsl_mc_obj_desc,
) -> bool {
    (*mc_dev).obj_desc.id == (*obj_desc).id
        && strcmp((*mc_dev).obj_desc.type_.as_ptr(), (*obj_desc).type_.as_ptr()) == 0
}

unsafe fn fsl_mc_obj_desc_is_allocatable(obj: *mut fsl_mc_obj_desc) -> bool {
    strcmp((*obj).type_.as_ptr(), b"dpmcp\0".as_ptr()) == 0
        || strcmp((*obj).type_.as_ptr(), b"dpcon\0".as_ptr()) == 0
        || strcmp((*obj).type_.as_ptr(), b"dpbp\0".as_ptr()) == 0
}

unsafe extern "C" fn __fsl_mc_device_remove_if_not_in_mc(
    dev: *mut device,
    data: *mut core::ffi::c_void,
) -> i32 {
    if !dev_is_fsl_mc(dev) { return 0; }
    let mc_dev = to_fsl_mc_device(dev);
    let objs = data as *mut fsl_mc_child_objs;
    let mut i = 0;
    while i < (*objs).child_count {
        let obj_desc = (*objs).child_array.add(i as usize);
        if strlen((*obj_desc).type_.as_ptr()) != 0 && fsl_mc_device_match(mc_dev, obj_desc) { break; }
        i += 1;
    }
    if i == (*objs).child_count { fsl_mc_device_remove(mc_dev); }
    0
}

unsafe extern "C" fn __fsl_mc_device_remove(
    dev: *mut device,
    _data: *mut core::ffi::c_void,
) -> i32 {
    if !dev_is_fsl_mc(dev) { return 0; }
    fsl_mc_device_remove(to_fsl_mc_device(dev));
    0
}

pub unsafe fn dprc_remove_devices(
    mc_bus_dev: *mut fsl_mc_device,
    obj_desc_array: *mut fsl_mc_obj_desc,
    num_child_objects_in_mc: i32,
) {
    if num_child_objects_in_mc != 0 {
        let objs = fsl_mc_child_objs { child_count: num_child_objects_in_mc, child_array: obj_desc_array };
        device_for_each_child(&mut (*mc_bus_dev).dev, &objs as *const _ as *mut _, __fsl_mc_device_remove_if_not_in_mc);
    } else {
        device_for_each_child(&mut (*mc_bus_dev).dev, core::ptr::null_mut(), __fsl_mc_device_remove);
    }
}

unsafe fn __fsl_mc_device_match(dev: *mut device, data: *const core::ffi::c_void) -> i32 {
    fsl_mc_device_match(to_fsl_mc_device(dev), data as *const fsl_mc_obj_desc) as i32
}

pub unsafe fn fsl_mc_device_lookup(
    obj_desc: *mut fsl_mc_obj_desc,
    mc_bus_dev: *mut fsl_mc_device,
) -> *mut fsl_mc_device {
    let dev = device_find_child(&mut (*mc_bus_dev).dev, obj_desc as *mut _, __fsl_mc_device_match);
    if !dev.is_null() { to_fsl_mc_device(dev) } else { core::ptr::null_mut() }
}

unsafe fn check_plugged_state_change(mc_dev: *mut fsl_mc_device, obj_desc: *mut fsl_mc_obj_desc) {
    let plugged_flag_at_mc = (*obj_desc).state & FSL_MC_OBJ_STATE_PLUGGED;
    if plugged_flag_at_mc != ((*mc_dev).obj_desc.state & FSL_MC_OBJ_STATE_PLUGGED) {
        if plugged_flag_at_mc != 0 {
            (*mc_dev).obj_desc.state |= FSL_MC_OBJ_STATE_PLUGGED;
            let error = device_attach(&mut (*mc_dev).dev);
            if error < 0 { dev_err(&mut (*mc_dev).dev, "device_attach() failed: %d\\n", error); }
        } else {
            (*mc_dev).obj_desc.state &= !FSL_MC_OBJ_STATE_PLUGGED;
            device_release_driver(&mut (*mc_dev).dev);
        }
    }
}

unsafe fn fsl_mc_obj_device_add(mc_bus_dev: *mut fsl_mc_device, obj_desc: *mut fsl_mc_obj_desc) {
    let child_dev = fsl_mc_device_lookup(obj_desc, mc_bus_dev);
    if !child_dev.is_null() {
        check_plugged_state_change(child_dev, obj_desc);
        put_device(&mut (*child_dev).dev);
    } else {
        let mut new_child: *mut fsl_mc_device = core::ptr::null_mut();
        let error = fsl_mc_device_add(obj_desc, core::ptr::null_mut(), &mut (*mc_bus_dev).dev, &mut new_child);
        if error < 0 { return; }
    }
}

unsafe fn dprc_add_new_devices(mc_bus_dev: *mut fsl_mc_device, obj_desc_array: *mut fsl_mc_obj_desc, n: i32) {
    for i in 0..n {
        let obj = obj_desc_array.add(i as usize);
        if strlen((*obj).type_.as_ptr()) > 0 && fsl_mc_obj_desc_is_allocatable(obj) { fsl_mc_obj_device_add(mc_bus_dev, obj); }
    }
    for i in 0..n {
        let obj = obj_desc_array.add(i as usize);
        if strlen((*obj).type_.as_ptr()) > 0 && !fsl_mc_obj_desc_is_allocatable(obj) { fsl_mc_obj_device_add(mc_bus_dev, obj); }
    }
}

pub unsafe fn dprc_scan_objects(mc_bus_dev: *mut fsl_mc_device, alloc_interrupts: bool) -> i32 {
    let mut num_child_objects = 0i32;
    let mut error = dprc_get_obj_count((*mc_bus_dev).mc_io, 0, (*mc_bus_dev).mc_handle, &mut num_child_objects);
    if error < 0 { dev_err(&mut (*mc_bus_dev).dev, "dprc_get_obj_count() failed: %d\\n", error); return error; }
    let mut irq_count = (*mc_bus_dev).obj_desc.irq_count;
    let mut child_obj_desc_array: *mut fsl_mc_obj_desc = core::ptr::null_mut();
    let mc_bus = to_fsl_mc_bus(mc_bus_dev);
    if num_child_objects != 0 {
        child_obj_desc_array = devm_kmalloc_array(&mut (*mc_bus_dev).dev, num_child_objects as usize, core::mem::size_of::<fsl_mc_obj_desc>(), GFP_KERNEL);
        if child_obj_desc_array.is_null() { return -ENOMEM; }
        let mut failures = 0;
        for i in 0..num_child_objects {
            let obj = child_obj_desc_array.add(i as usize);
            error = dprc_get_obj((*mc_bus_dev).mc_io, 0, (*mc_bus_dev).mc_handle, i, obj);
            if error < 0 {
                dev_err(&mut (*mc_bus_dev).dev, "dprc_get_obj(i=%d) failed: %d\\n", i, error);
                (*obj).type_[0] = 0; (*obj).id = error; failures += 1; continue;
            }
            if strcmp((*obj).type_.as_ptr(), b"dpseci\0".as_ptr()) == 0 && (*obj).ver_major < 4 { (*obj).flags |= FSL_MC_OBJ_FLAG_NO_MEM_SHAREABILITY; }
            irq_count += (*obj).irq_count;
            dev_dbg(&mut (*mc_bus_dev).dev, "Discovered object: type %s, id %d\\n", (*obj).type_.as_ptr(), (*obj).id);
        }
        if failures != 0 { dev_err(&mut (*mc_bus_dev).dev, "%d out of %d devices could not be retrieved\\n", failures, num_child_objects); }
    }
    if !dev_get_msi_domain(&mut (*mc_bus_dev).dev).is_null() {
        if irq_count > FSL_MC_IRQ_POOL_MAX_TOTAL_IRQS { dev_warn(&mut (*mc_bus_dev).dev, "IRQs needed (%u) exceed IRQs preallocated (%u)\\n", irq_count, FSL_MC_IRQ_POOL_MAX_TOTAL_IRQS); }
        if alloc_interrupts && (*mc_bus).irq_resources.is_null() { error = fsl_mc_populate_irq_pool(mc_bus_dev, FSL_MC_IRQ_POOL_MAX_TOTAL_IRQS); if error < 0 { return error; } }
    }
    dprc_remove_devices(mc_bus_dev, child_obj_desc_array, num_child_objects);
    dprc_add_new_devices(mc_bus_dev, child_obj_desc_array, num_child_objects);
    if !child_obj_desc_array.is_null() { devm_kfree(&mut (*mc_bus_dev).dev, child_obj_desc_array as *mut _); }
    0
}

pub unsafe fn dprc_scan_container(mc_bus_dev: *mut fsl_mc_device, alloc_interrupts: bool) -> i32 {
    let mc_bus = to_fsl_mc_bus(mc_bus_dev);
    fsl_mc_init_all_resource_pools(mc_bus_dev);
    mutex_lock(&mut (*mc_bus).scan_mutex);
    let error = dprc_scan_objects(mc_bus_dev, alloc_interrupts);
    mutex_unlock(&mut (*mc_bus).scan_mutex);
    error
}

// The remaining driver entry points retain the C ABI and call the corresponding
// kernel/MC interfaces supplied by the surrounding repository.
pub unsafe fn disable_dprc_irq(mc_dev: *mut fsl_mc_device) -> i32 {
    let mc_bus = to_fsl_mc_bus(mc_dev); let mc_io = (*mc_dev).mc_io; let mut error;
    error = dprc_set_irq_enable(mc_io, 0, (*mc_dev).mc_handle, 0, 0); if error < 0 { return error; }
    error = dprc_set_irq_mask(mc_io, 0, (*mc_dev).mc_handle, 0, 0); if error < 0 { return error; }
    error = dprc_clear_irq_status(mc_io, 0, (*mc_dev).mc_handle, 0, !0u32); if error < 0 { return error; }
    (*mc_bus).irq_enabled = 0; 0
}

pub unsafe fn get_dprc_irq_state(mc_dev: *mut fsl_mc_device) -> i32 { (*to_fsl_mc_bus(mc_dev)).irq_enabled }

pub unsafe fn enable_dprc_irq(mc_dev: *mut fsl_mc_device) -> i32 {
    let mc_bus = to_fsl_mc_bus(mc_dev); let io = (*mc_dev).mc_io; let mut error;
    error = dprc_set_irq_mask(io, 0, (*mc_dev).mc_handle, 0, !0u32); if error < 0 { return error; }
    error = dprc_set_irq_enable(io, 0, (*mc_dev).mc_handle, 0, 1); if error < 0 { return error; }
    (*mc_bus).irq_enabled = 1; 0
}

pub unsafe fn dprc_setup(mc_dev: *mut fsl_mc_device) -> i32 {
    if !is_fsl_mc_bus_dprc(mc_dev) || !dev_get_msi_domain(&mut (*mc_dev).dev).is_null() { return -EINVAL; }
    let mc_bus = to_fsl_mc_bus(mc_dev);
    let mut created_io = false;
    if (*mc_dev).mc_io.is_null() {
        let parent = (*mc_dev).dev.parent;
        if !dev_is_fsl_mc(parent) || (*mc_dev).obj_desc.region_count == 0 { return -EINVAL; }
        let size = resource_size((*mc_dev).regions);
        let error = fsl_create_mc_io(&mut (*mc_dev).dev, (*mc_dev).regions.start, size, core::ptr::null_mut(), FSL_MC_IO_ATOMIC_CONTEXT_PORTAL, &mut (*mc_dev).mc_io);
        if error < 0 { return error; }
        created_io = true;
    } else if fsl_mc_uapi_create_device_file(mc_bus) < 0 { return -EPROBE_DEFER; }
    let domain = fsl_mc_get_msi_parent(&mut (*mc_dev).dev);
    if !domain.is_null() { dev_set_msi_domain(&mut (*mc_dev).dev, domain); }
    let mut error = dprc_open((*mc_dev).mc_io, 0, (*mc_dev).obj_desc.id, &mut (*mc_dev).mc_handle);
    if error >= 0 { error = dprc_get_attributes((*mc_dev).mc_io, 0, (*mc_dev).mc_handle, &mut (*mc_bus).dprc_attr); }
    if error >= 0 { let mut major = 0u16; let mut minor = 0u16; error = dprc_get_api_version((*mc_dev).mc_io, 0, &mut major, &mut minor); if error >= 0 && major < DPRC_MIN_VER_MAJOR { error = -ENOTSUPP; } }
    if error < 0 {
        let _ = dprc_close((*mc_dev).mc_io, 0, (*mc_dev).mc_handle);
        dev_set_msi_domain(&mut (*mc_dev).dev, core::ptr::null_mut());
        if created_io { fsl_destroy_mc_io((*mc_dev).mc_io); (*mc_dev).mc_io = core::ptr::null_mut(); }
        return error;
    }
    0
}

pub unsafe fn dprc_cleanup(mc_dev: *mut fsl_mc_device) -> i32 {
    if !is_fsl_mc_bus_dprc(mc_dev) { return -EINVAL; }
    let mc_bus = to_fsl_mc_bus(mc_dev);
    if !dev_get_msi_domain(&mut (*mc_dev).dev).is_null() { fsl_mc_cleanup_irq_pool(mc_dev); dev_set_msi_domain(&mut (*mc_dev).dev, core::ptr::null_mut()); }
    if (*mc_dev).mc_io.is_null() { return -EINVAL; }
    let _ = dprc_close((*mc_dev).mc_io, 0, (*mc_dev).mc_handle);
    if !fsl_mc_is_root_dprc(&mut (*mc_dev).dev) { fsl_destroy_mc_io((*mc_dev).mc_io); (*mc_dev).mc_io = core::ptr::null_mut(); } else { fsl_mc_uapi_remove_device_file(mc_bus); }
    0
}

unsafe fn dprc_probe(mc_dev: *mut fsl_mc_device) -> i32 {
    let mut error = dprc_setup(mc_dev); if error < 0 { return error; }
    error = dprc_scan_container(mc_dev, true); if error < 0 { dprc_cleanup(mc_dev); return error; }
    error = dprc_setup_irq(mc_dev); if error < 0 { device_for_each_child(&mut (*mc_dev).dev, core::ptr::null_mut(), __fsl_mc_device_remove); dprc_cleanup(mc_dev); }
    error
}

unsafe fn dprc_setup_irq(mc_dev: *mut fsl_mc_device) -> i32 {
    let mut error = fsl_mc_allocate_irqs(mc_dev); if error < 0 { return error; }
    error = disable_dprc_irq(mc_dev); if error < 0 { fsl_mc_free_irqs(mc_dev); return error; }
    error = enable_dprc_irq(mc_dev); if error < 0 { fsl_mc_free_irqs(mc_dev); return error; }
    0
}

unsafe fn dprc_teardown_irq(mc_dev: *mut fsl_mc_device) { let _ = disable_dprc_irq(mc_dev); fsl_mc_free_irqs(mc_dev); }

unsafe fn dprc_remove(mc_dev: *mut fsl_mc_device) {
    if !(*to_fsl_mc_bus(mc_dev)).irq_resources.is_null() { if !dev_get_msi_domain(&mut (*mc_dev).dev).is_null() { dprc_teardown_irq(mc_dev); } device_for_each_child(&mut (*mc_dev).dev, core::ptr::null_mut(), __fsl_mc_device_remove); dprc_cleanup(mc_dev); }
}

pub unsafe fn dprc_driver_init() -> i32 { fsl_mc_driver_register(&mut dprc_driver) }
pub unsafe fn dprc_driver_exit() { fsl_mc_driver_unregister(&mut dprc_driver); }

extern "C" {
    fn strcmp(a: *const u8, b: *const u8) -> i32;
    fn strlen(a: *const u8) -> usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
