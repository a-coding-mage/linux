// SPDX-License-Identifier: GPL-2.0-only
/*
 *  PS3 system bus driver.
 *
 *  Copyright (C) 2006 Sony Computer Entertainment Inc.
 *  Copyright 2006 Sony Corp.
 */

// Kernel/platform declarations are supplied by the surrounding translation.

static mut PS3_SYSTEM_BUS: device = device { init_name: "ps3_system" };

/* FIXME: need device usage counters! */
struct UsageHack { mutex: mutex, sb_11: i32, sb_12: i32, gpu: i32 }
static mut USAGE_HACK: UsageHack = UsageHack { mutex: mutex {}, sb_11: 0, sb_12: 0, gpu: 0 };

unsafe fn ps3_is_device(dev: *mut ps3_system_bus_device, bus_id: u64, dev_id: u64) -> i32 {
    ((*dev).bus_id == bus_id && (*dev).dev_id == dev_id) as i32
}

unsafe fn ps3_open_hv_device_sb(dev: *mut ps3_system_bus_device) -> i32 {
    let mut result: i32;
    BUG_ON((*dev).bus_id == 0);
    mutex_lock(&mut USAGE_HACK.mutex);
    if ps3_is_device(dev, 1, 1) != 0 {
        USAGE_HACK.sb_11 += 1;
        if USAGE_HACK.sb_11 > 1 { result = 0; mutex_unlock(&mut USAGE_HACK.mutex); return result; }
    }
    if ps3_is_device(dev, 1, 2) != 0 {
        USAGE_HACK.sb_12 += 1;
        if USAGE_HACK.sb_12 > 1 { result = 0; mutex_unlock(&mut USAGE_HACK.mutex); return result; }
    }
    result = lv1_open_device((*dev).bus_id, (*dev).dev_id, 0);
    if result != 0 {
        pr_warn("%s:%d: lv1_open_device dev=%u.%u(%s) failed: %s\n", "ps3_open_hv_device_sb", line!(), (*dev).match_id, (*dev).match_sub_id, dev_name(&(*dev).core), ps3_result(result));
        result = -EPERM;
    }
    mutex_unlock(&mut USAGE_HACK.mutex); result
}

unsafe fn ps3_close_hv_device_sb(dev: *mut ps3_system_bus_device) -> i32 {
    BUG_ON((*dev).bus_id == 0); mutex_lock(&mut USAGE_HACK.mutex);
    if ps3_is_device(dev, 1, 1) != 0 { USAGE_HACK.sb_11 -= 1; if USAGE_HACK.sb_11 != 0 { mutex_unlock(&mut USAGE_HACK.mutex); return 0; } }
    if ps3_is_device(dev, 1, 2) != 0 { USAGE_HACK.sb_12 -= 1; if USAGE_HACK.sb_12 != 0 { mutex_unlock(&mut USAGE_HACK.mutex); return 0; } }
    let result = lv1_close_device((*dev).bus_id, (*dev).dev_id); BUG_ON(result);
    mutex_unlock(&mut USAGE_HACK.mutex); result
}

unsafe fn ps3_open_hv_device_gpu(_dev: *mut ps3_system_bus_device) -> i32 {
    mutex_lock(&mut USAGE_HACK.mutex); USAGE_HACK.gpu += 1;
    if USAGE_HACK.gpu > 1 { mutex_unlock(&mut USAGE_HACK.mutex); return 0; }
    let mut result = lv1_gpu_open(0);
    if result != 0 { pr_warn!("%s:%d: lv1_gpu_open failed: %s\n", "ps3_open_hv_device_gpu", line!(), ps3_result(result)); result = -EPERM; }
    mutex_unlock(&mut USAGE_HACK.mutex); result
}

unsafe fn ps3_close_hv_device_gpu(_dev: *mut ps3_system_bus_device) -> i32 {
    mutex_lock(&mut USAGE_HACK.mutex); USAGE_HACK.gpu -= 1;
    if USAGE_HACK.gpu != 0 { mutex_unlock(&mut USAGE_HACK.mutex); return 0; }
    let result = lv1_gpu_close(); BUG_ON(result); mutex_unlock(&mut USAGE_HACK.mutex); result
}

pub unsafe fn ps3_open_hv_device(dev: *mut ps3_system_bus_device) -> i32 {
    BUG_ON(dev.is_null()); pr_debug!("%s:%d: match_id: %u\n", "ps3_open_hv_device", line!(), (*dev).match_id);
    match (*dev).match_id {
        PS3_MATCH_ID_EHCI | PS3_MATCH_ID_OHCI | PS3_MATCH_ID_GELIC | PS3_MATCH_ID_STOR_DISK | PS3_MATCH_ID_STOR_ROM | PS3_MATCH_ID_STOR_FLASH => ps3_open_hv_device_sb(dev),
        PS3_MATCH_ID_SOUND | PS3_MATCH_ID_GPU => ps3_open_hv_device_gpu(dev),
        PS3_MATCH_ID_AV_SETTINGS | PS3_MATCH_ID_SYSTEM_MANAGER => { pr_debug!("%s:%d: unsupported match_id: %u\n", "ps3_open_hv_device", line!(), (*dev).match_id); pr_debug!("%s:%d: bus_id: %llu\n", "ps3_open_hv_device", line!(), (*dev).bus_id); BUG(); -EINVAL }
        _ => { pr_debug!("%s:%d: unknown match_id: %u\n", "ps3_open_hv_device", line!(), (*dev).match_id); BUG(); -ENODEV }
    }
}

pub unsafe fn ps3_close_hv_device(dev: *mut ps3_system_bus_device) -> i32 {
    BUG_ON(dev.is_null()); pr_debug!("%s:%d: match_id: %u\n", "ps3_close_hv_device", line!(), (*dev).match_id);
    match (*dev).match_id {
        PS3_MATCH_ID_EHCI | PS3_MATCH_ID_OHCI | PS3_MATCH_ID_GELIC | PS3_MATCH_ID_STOR_DISK | PS3_MATCH_ID_STOR_ROM | PS3_MATCH_ID_STOR_FLASH => ps3_close_hv_device_sb(dev),
        PS3_MATCH_ID_SOUND | PS3_MATCH_ID_GPU => ps3_close_hv_device_gpu(dev),
        PS3_MATCH_ID_AV_SETTINGS | PS3_MATCH_ID_SYSTEM_MANAGER => { pr_debug!("%s:%d: unsupported match_id: %u\n", "ps3_close_hv_device", line!(), (*dev).match_id); pr_debug!("%s:%d: bus_id: %llu\n", "ps3_close_hv_device", line!(), (*dev).bus_id); BUG(); -EINVAL }
        _ => { pr_debug!("%s:%d: unknown match_id: %u\n", "ps3_close_hv_device", line!(), (*dev).match_id); BUG(); -ENODEV }
    }
}

unsafe fn _dump_mmio_region(r: *const ps3_mmio_region, func: *const i8, line: i32) {
    pr_debug!("%s:%d: dev       %llu:%llu\n", func, line, (*(*r).dev).bus_id, (*(*r).dev).dev_id);
    pr_debug!("%s:%d: bus_addr  %lxh\n", func, line, (*r).bus_addr); pr_debug!("%s:%d: len       %lxh\n", func, line, (*r).len); pr_debug!("%s:%d: lpar_addr %lxh\n", func, line, (*r).lpar_addr);
}

unsafe fn ps3_sb_mmio_region_create(r: *mut ps3_mmio_region) -> i32 {
    let mut lpar_addr = 0u64; let result = lv1_map_device_mmio_region((*(*r).dev).bus_id, (*(*r).dev).dev_id, (*r).bus_addr, (*r).len, (*r).page_size, &mut lpar_addr); (*r).lpar_addr = lpar_addr;
    if result != 0 { pr_debug!("%s:%d: lv1_map_device_mmio_region failed: %s\n", "ps3_sb_mmio_region_create", line!(), ps3_result(result)); (*r).lpar_addr = 0; } _dump_mmio_region(r, "ps3_sb_mmio_region_create".as_ptr() as *const i8, line!()); result
}
unsafe fn ps3_ioc0_mmio_region_create(_r: *mut ps3_mmio_region) -> i32 { 0 }
pub unsafe fn ps3_mmio_region_create(r: *mut ps3_mmio_region) -> i32 { ((*r).mmio_ops).create.unwrap()(r) }
unsafe fn ps3_sb_free_mmio_region(r: *mut ps3_mmio_region) -> i32 { _dump_mmio_region(r, "ps3_sb_free_mmio_region".as_ptr() as *const i8, line!()); let result = lv1_unmap_device_mmio_region((*(*r).dev).bus_id, (*(*r).dev).dev_id, (*r).lpar_addr); if result != 0 { pr_debug!("%s:%d: lv1_unmap_device_mmio_region failed: %s\n", "ps3_sb_free_mmio_region", line!(), ps3_result(result)); } (*r).lpar_addr = 0; result }
unsafe fn ps3_ioc0_free_mmio_region(_r: *mut ps3_mmio_region) -> i32 { 0 }
pub unsafe fn ps3_free_mmio_region(r: *mut ps3_mmio_region) -> i32 { ((*r).mmio_ops).free.unwrap()(r) }

static PS3_MMIO_SB_REGION_OPS: ps3_mmio_region_ops = ps3_mmio_region_ops { create: Some(ps3_sb_mmio_region_create), free: Some(ps3_sb_free_mmio_region) };
static PS3_MMIO_IOC0_REGION_OPS: ps3_mmio_region_ops = ps3_mmio_region_ops { create: Some(ps3_ioc0_mmio_region_create), free: Some(ps3_ioc0_free_mmio_region) };

pub unsafe fn ps3_mmio_region_init(dev: *mut ps3_system_bus_device, r: *mut ps3_mmio_region, bus_addr: usize, len: usize, page_size: ps3_mmio_page_size) -> i32 {
    (*r).dev = dev; (*r).bus_addr = bus_addr; (*r).len = len; (*r).page_size = page_size;
    match (*dev).dev_type { PS3_DEVICE_TYPE_SB => (*r).mmio_ops = &PS3_MMIO_SB_REGION_OPS, PS3_DEVICE_TYPE_IOC0 => (*r).mmio_ops = &PS3_MMIO_IOC0_REGION_OPS, _ => { BUG(); return -EINVAL } } 0
}

unsafe fn ps3_system_bus_match(_dev: *mut device, _drv: *const device_driver) -> i32 { let dev = ps3_dev_to_system_bus_dev(_dev); let drv = ps3_drv_to_system_bus_drv(_drv); let result = if (*dev).match_sub_id == 0 { ((*dev).match_id == (*drv).match_id) as i32 } else { ((*dev).match_sub_id == (*drv).match_sub_id && (*dev).match_id == (*drv).match_id) as i32 }; result }
unsafe fn ps3_system_bus_probe(_dev: *mut device) -> i32 { let dev = ps3_dev_to_system_bus_dev(_dev); BUG_ON(dev.is_null()); let drv = ps3_system_bus_dev_to_system_bus_drv(dev); BUG_ON(drv.is_null()); if let Some(f) = (*drv).probe { f(dev) } else { 0 } }
unsafe fn ps3_system_bus_remove(_dev: *mut device) { let dev = ps3_dev_to_system_bus_dev(_dev); BUG_ON(dev.is_null()); let drv = ps3_system_bus_dev_to_system_bus_drv(dev); BUG_ON(drv.is_null()); if let Some(f) = (*drv).remove { f(dev); } }
unsafe fn ps3_system_bus_shutdown(_dev: *mut device) { let dev = ps3_dev_to_system_bus_dev(_dev); BUG_ON(dev.is_null()); if (*dev).core.driver.is_null() { return; } let drv = ps3_system_bus_dev_to_system_bus_drv(dev); BUG_ON(drv.is_null()); if let Some(f) = (*drv).shutdown { f(dev); } else if let Some(f) = (*drv).remove { f(dev); } else { BUG(); } }
unsafe fn ps3_system_bus_uevent(_dev: *const device, env: *mut kobj_uevent_env) -> i32 { let dev = ps3_dev_to_system_bus_dev(_dev as *mut device); if add_uevent_var(env, "MODALIAS=ps3:%d:%d", (*dev).match_id, (*dev).match_sub_id) != 0 { -ENOMEM } else { 0 } }
unsafe fn modalias_show(_dev: *mut device, _a: *mut device_attribute, buf: *mut i8) -> isize { let dev = ps3_dev_to_system_bus_dev(_dev); sysfs_emit(buf, "ps3:%d:%d\n", (*dev).match_id, (*dev).match_sub_id) }

static PS3_SYSTEM_BUS_TYPE: bus_type = bus_type { name: "ps3_system_bus", match_: Some(ps3_system_bus_match), uevent: Some(ps3_system_bus_uevent), probe: Some(ps3_system_bus_probe), remove: Some(ps3_system_bus_remove), shutdown: Some(ps3_system_bus_shutdown) };

unsafe fn ps3_system_bus_init() -> i32 {
    if !firmware_has_feature(FW_FEATURE_PS3_LV1) { return -ENODEV; }
    mutex_init(&mut USAGE_HACK.mutex); let result = device_register(&mut PS3_SYSTEM_BUS); BUG_ON(result); let result = bus_register(&PS3_SYSTEM_BUS_TYPE); BUG_ON(result); result
}

unsafe fn ps3_alloc_coherent(_dev: *mut device, size: usize, dma_handle: *mut dma_addr_t, mut flag: gfp_t, _attrs: usize) -> *mut core::ffi::c_void {
    let dev = ps3_dev_to_system_bus_dev(_dev); flag &= !(__GFP_DMA | __GFP_HIGHMEM); flag |= __GFP_ZERO; let virt_addr = __get_free_pages(flag, get_order(size)); if virt_addr == 0 { return core::ptr::null_mut(); }
    let result = ps3_dma_map((*dev).d_region, virt_addr, size, dma_handle, CBE_IOPTE_PP_W | CBE_IOPTE_PP_R | CBE_IOPTE_SO_RW | CBE_IOPTE_M); if result != 0 { BUG_ON(true); free_pages(virt_addr, get_order(size)); return core::ptr::null_mut(); } virt_addr as *mut core::ffi::c_void
}
unsafe fn ps3_free_coherent(_dev: *mut device, size: usize, vaddr: *mut core::ffi::c_void, dma_handle: dma_addr_t, _attrs: usize) { let dev = ps3_dev_to_system_bus_dev(_dev); ps3_dma_unmap((*dev).d_region, dma_handle, size); free_pages(vaddr as usize, get_order(size)); }

unsafe fn ps3_sb_map_phys(_dev: *mut device, phys: phys_addr_t, size: usize, _direction: dma_data_direction, attrs: usize) -> dma_addr_t { let dev = ps3_dev_to_system_bus_dev(_dev); if attrs & DMA_ATTR_MMIO != 0 { return DMA_MAPPING_ERROR; } let ptr = phys_to_virt(phys); let mut bus_addr = 0; let _ = ps3_dma_map((*dev).d_region, ptr as usize, size, &mut bus_addr, CBE_IOPTE_PP_R | CBE_IOPTE_PP_W | CBE_IOPTE_SO_RW | CBE_IOPTE_M); bus_addr }
unsafe fn ps3_ioc0_map_phys(_dev: *mut device, phys: phys_addr_t, size: usize, direction: dma_data_direction, attrs: usize) -> dma_addr_t { let dev = ps3_dev_to_system_bus_dev(_dev); if attrs & DMA_ATTR_MMIO != 0 { return DMA_MAPPING_ERROR; } let mut flags = CBE_IOPTE_M; flags |= match direction { DMA_BIDIRECTIONAL => CBE_IOPTE_PP_R | CBE_IOPTE_PP_W | CBE_IOPTE_SO_RW, DMA_TO_DEVICE => CBE_IOPTE_PP_R | CBE_IOPTE_SO_R, DMA_FROM_DEVICE => CBE_IOPTE_PP_W | CBE_IOPTE_SO_RW, _ => { BUG(); 0 } }; let ptr = phys_to_virt(phys); let mut bus_addr = 0; let _ = ps3_dma_map((*dev).d_region, ptr as usize, size, &mut bus_addr, flags); bus_addr }
unsafe fn ps3_unmap_phys(_dev: *mut device, dma_addr: dma_addr_t, size: usize, _direction: dma_data_direction, _attrs: usize) { let dev = ps3_dev_to_system_bus_dev(_dev); let _ = ps3_dma_unmap((*dev).d_region, dma_addr, size); }
unsafe fn ps3_sb_map_sg(_dev: *mut device, _sgl: *mut scatterlist, nents: i32, _direction: dma_data_direction, _attrs: usize) -> i32 { BUG_ON(false); nents }
unsafe fn ps3_ioc0_map_sg(_dev: *mut device, _sg: *mut scatterlist, _nents: i32, _direction: dma_data_direction, _attrs: usize) -> i32 { BUG(); -EINVAL }
unsafe fn ps3_sb_unmap_sg(_dev: *mut device, _sg: *mut scatterlist, _nents: i32, _direction: dma_data_direction, _attrs: usize) {}
unsafe fn ps3_ioc0_unmap_sg(_dev: *mut device, _sg: *mut scatterlist, _nents: i32, _direction: dma_data_direction, _attrs: usize) { BUG(); }
unsafe fn ps3_dma_supported(_dev: *mut device, mask: u64) -> i32 { (mask >= DMA_BIT_MASK(32)) as i32 }

unsafe fn ps3_system_bus_release_device(_dev: *mut device) { let dev = ps3_dev_to_system_bus_dev(_dev); kfree(dev); }

pub unsafe fn ps3_system_bus_device_register(dev: *mut ps3_system_bus_device) -> i32 {
    if (*dev).core.parent.is_null() { (*dev).core.parent = &mut PS3_SYSTEM_BUS; } (*dev).core.bus = &PS3_SYSTEM_BUS_TYPE; (*dev).core.release = Some(ps3_system_bus_release_device);
    match (*dev).dev_type { PS3_DEVICE_TYPE_IOC0 => { (*dev).core.dma_ops = &ps3_ioc0_dma_ops; }, PS3_DEVICE_TYPE_SB => { (*dev).core.dma_ops = &ps3_sb_dma_ops; }, PS3_DEVICE_TYPE_VUART | PS3_DEVICE_TYPE_LPM => {}, _ => BUG() }
    (*dev).core.of_node = core::ptr::null_mut(); set_dev_node(&mut (*dev).core, 0); device_register(&mut (*dev).core)
}
pub unsafe fn ps3_system_bus_driver_register(drv: *mut ps3_system_bus_driver) -> i32 { if !firmware_has_feature(FW_FEATURE_PS3_LV1) { return -ENODEV; } (*drv).core.bus = &PS3_SYSTEM_BUS_TYPE; driver_register(&mut (*drv).core) }
pub unsafe fn ps3_system_bus_driver_unregister(drv: *mut ps3_system_bus_driver) { driver_unregister(&mut (*drv).core); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
