// SPDX-License-Identifier: GPL-2.0-only or MIT
// Copyright (C) 2025 Arm, Ltd.

// Linux, DRM, and local driver dependencies are supplied by the surrounding crate.

const ETHOSU_BO_FLAGS: u32 = DRM_ETHOSU_BO_NO_MMAP;

unsafe fn ethosu_ioctl_dev_query(ddev: *mut drm_device, data: *mut core::ffi::c_void,
                                 _file: *mut drm_file) -> i32 {
    let ethosudev = to_ethosu_device(ddev);
    let args = data as *mut drm_ethosu_dev_query;
    if (*args).pointer == 0 {
        match (*args).type_ {
            DRM_ETHOSU_DEV_QUERY_NPU_INFO => { (*args).size = core::mem::size_of::<drm_ethosu_npu_info>() as u64; 0 }
            _ => -EINVAL,
        }
    } else {
        match (*args).type_ {
            DRM_ETHOSU_DEV_QUERY_NPU_INFO => {
                if (*args).size < offsetofend::<drm_ethosu_npu_info>("sram_size") { return -EINVAL; }
                copy_struct_to_user(u64_to_user_ptr((*args).pointer), (*args).size,
                                    &(*ethosudev).npu_info, core::mem::size_of::<drm_ethosu_npu_info>(), core::ptr::null_mut())
            }
            _ => -EINVAL,
        }
    }
}

unsafe fn ethosu_ioctl_bo_create(ddev: *mut drm_device, data: *mut core::ffi::c_void,
                                 file: *mut drm_file) -> i32 {
    let args = data as *mut drm_ethosu_bo_create; let mut cookie = 0; let ret;
    if !drm_dev_enter(ddev, &mut cookie) { return -ENODEV; }
    if (*args).size == 0 || ((*args).flags & !ETHOSU_BO_FLAGS) != 0 { ret = -EINVAL; }
    else { ret = ethosu_gem_create_with_handle(file, ddev, &mut (*args).size, (*args).flags, &mut (*args).handle); }
    drm_dev_exit(cookie); ret
}

unsafe fn ethosu_ioctl_bo_wait(ddev: *mut drm_device, data: *mut core::ffi::c_void, file: *mut drm_file) -> i32 {
    let args = data as *mut drm_ethosu_bo_wait; if (*args).pad != 0 { return -EINVAL; }
    let mut cookie = 0; if !drm_dev_enter(ddev, &mut cookie) { return -ENODEV; }
    let ret = drm_gem_dma_resv_wait(file, (*args).handle, true, drm_timeout_abs_to_jiffies((*args).timeout_ns)); drm_dev_exit(cookie); ret
}

unsafe fn ethosu_ioctl_bo_mmap_offset(_ddev: *mut drm_device, data: *mut core::ffi::c_void, file: *mut drm_file) -> i32 {
    let args = data as *mut drm_ethosu_bo_mmap_offset; if (*args).pad != 0 { return -EINVAL; }
    let obj = drm_gem_object_lookup(file, (*args).handle); if obj.is_null() { return -ENOENT; }
    (*args).offset = drm_vma_node_offset_addr(&(*obj).vma_node); drm_gem_object_put(obj); 0
}

unsafe fn ethosu_ioctl_cmdstream_bo_create(ddev: *mut drm_device, data: *mut core::ffi::c_void, file: *mut drm_file) -> i32 {
    let args = data as *mut drm_ethosu_cmdstream_bo_create; let mut cookie = 0; let ret;
    if !drm_dev_enter(ddev, &mut cookie) { return -ENODEV; }
    if (*args).size == 0 || (*args).data == 0 || (*args).pad != 0 || (*args).flags != 0 { ret = -EINVAL; }
    else { (*args).flags |= DRM_ETHOSU_BO_NO_MMAP; ret = ethosu_gem_cmdstream_create(file, ddev, (*args).size, (*args).data, (*args).flags, &mut (*args).handle); }
    drm_dev_exit(cookie); ret
}

unsafe fn ethosu_open(ddev: *mut drm_device, file: *mut drm_file) -> i32 {
    if !try_module_get(THIS_MODULE) { return -EINVAL; }
    let priv_ = kzalloc_obj::<ethosu_file_priv>(); if priv_.is_null() { module_put(THIS_MODULE); return -ENOMEM; }
    (*priv_).edev = to_ethosu_device(ddev); let ret = ethosu_job_open(priv_);
    if ret != 0 { module_put(THIS_MODULE); return ret; }
    ethosu_perfmon_open_file(priv_); (*file).driver_priv = priv_ as *mut _; 0
}

unsafe fn ethosu_postclose(_ddev: *mut drm_device, file: *mut drm_file) {
    ethosu_job_close((*file).driver_priv); ethosu_perfmon_close_file((*file).driver_priv); kfree((*file).driver_priv); module_put(THIS_MODULE);
}

const U65_DRAM_AXI_LIMIT_CFG: u32 = 0x1f3f0002;
const U65_SRAM_AXI_LIMIT_CFG: u32 = 0x1f3f00b0;
const U85_AXI_EXT_CFG: u32 = 0x00021f3f;
const U85_AXI_SRAM_CFG: u32 = 0x00021f3f;
const U85_MEM_ATTR0_CFG: u32 = 0x00000000;
const U85_MEM_ATTR2_CFG: u32 = 0x000000b7;

unsafe fn ethosu_reset(ethosudev: *mut ethosu_device) -> i32 {
    let mut reg = 0u32;
    writel_relaxed(RESET_PENDING_CSL, (*ethosudev).regs.add(NPU_REG_RESET as usize));
    let ret = readl_poll_timeout((*ethosudev).regs.add(NPU_REG_STATUS as usize), &mut reg, (FIELD_GET(STATUS_RESET_STATUS, reg) == 0), USEC_PER_MSEC, USEC_PER_SEC);
    if ret != 0 { return ret; }
    if FIELD_GET(PROT_ACTIVE_CSL, readl_relaxed((*ethosudev).regs.add(NPU_REG_PROT as usize))) == 0 { dev_warn((*ethosudev).base.dev, "Could not reset to non-secure mode (PROT = %x)\n", readl_relaxed((*ethosudev).regs.add(NPU_REG_PROT as usize))); }
    // Assign region 2 (SRAM) to AXI M0, everything else to AXI M1.
    writel_relaxed(0x0000aa8a, (*ethosudev).regs.add(NPU_REG_REGIONCFG as usize));
    if ethosu_is_u65(ethosudev) { writel_relaxed(U65_SRAM_AXI_LIMIT_CFG, (*ethosudev).regs.add(NPU_REG_AXILIMIT0 as usize)); writel_relaxed(U65_DRAM_AXI_LIMIT_CFG, (*ethosudev).regs.add(NPU_REG_AXILIMIT2 as usize)); }
    else { writel_relaxed(U85_AXI_SRAM_CFG, (*ethosudev).regs.add(NPU_REG_AXI_SRAM as usize)); writel_relaxed(U85_AXI_EXT_CFG, (*ethosudev).regs.add(NPU_REG_AXI_EXT as usize)); writel_relaxed(U85_MEM_ATTR0_CFG, (*ethosudev).regs.add(NPU_REG_MEM_ATTR0 as usize)); writel_relaxed(U85_MEM_ATTR2_CFG, (*ethosudev).regs.add(NPU_REG_MEM_ATTR2 as usize)); }
    if !(*ethosudev).sram.is_null() { memset_io((*ethosudev).sram, 0, (*ethosudev).npu_info.sram_size as usize); } 0
}

// Remaining driver registration and PM declarations mirror the source module and depend on external kernel bindings.
extern "C" {
    static mut ethosu_drm_driver: drm_driver;
    static mut ethosu_driver: platform_driver;
}

unsafe fn ethosu_device_resume(dev: *mut device) -> i32 {
    let e = dev_get_drvdata(dev); let ret = clk_bulk_prepare_enable((*e).num_clks, (*e).clks); if ret != 0 { return ret; }
    let ret = ethosu_reset(e); if ret != 0 { clk_bulk_disable_unprepare((*e).num_clks, (*e).clks); } ret
}
unsafe fn ethosu_device_suspend(dev: *mut device) -> i32 { let e = dev_get_drvdata(dev); clk_bulk_disable_unprepare((*e).num_clks, (*e).clks); 0 }
unsafe fn ethosu_sram_init(e: *mut ethosu_device) -> i32 {
    (*e).npu_info.sram_size = 0; (*e).srampool = of_gen_pool_get((*e).base.dev.of_node, "sram", 0); if (*e).srampool.is_null() { return 0; }
    (*e).npu_info.sram_size = gen_pool_size((*e).srampool); (*e).sram = gen_pool_dma_alloc((*e).srampool, (*e).npu_info.sram_size, &mut (*e).sramphys) as *mut _;
    if (*e).sram.is_null() { dev_err((*e).base.dev, "failed to allocate from SRAM pool\n"); return -ENOMEM; } 0
}
unsafe fn ethosu_init(e: *mut ethosu_device) -> i32 {
    let ret = ethosu_device_resume((*e).base.dev); if ret != 0 { return ret; }
    pm_runtime_set_autosuspend_delay((*e).base.dev, 50); pm_runtime_use_autosuspend((*e).base.dev); let ret = devm_pm_runtime_set_active_enabled((*e).base.dev); if ret != 0 { return ret; } pm_runtime_get_noresume((*e).base.dev);
    let id = readl_relaxed((*e).regs.add(NPU_REG_ID as usize)); let config = readl_relaxed((*e).regs.add(NPU_REG_CONFIG as usize)); (*e).npu_info.id = id; (*e).npu_info.config = config; ethosu_sram_init(e);
    if !ethosu_is_u65(e) { (*e).pmu_regs = (*e).pmu_regs.add(0x1000); } (*e).npu_info.pmu_counters = FIELD_GET(PMCR_NUM_EVENT_CNT_MASK, readl_relaxed((*e).pmu_regs.add(NPU_REG_PMCR as usize))); 0
}
unsafe fn ethosu_probe(pdev: *mut platform_device) -> i32 {
    let e = devm_drm_dev_alloc(&mut (*pdev).dev, &ethosu_drm_driver, core::mem::size_of::<ethosu_device>(), 0); if IS_ERR(e) { return -ENOMEM; } platform_set_drvdata(pdev, e);
    dma_set_mask_and_coherent(&mut (*pdev).dev, DMA_BIT_MASK(40)); (*e).regs = devm_platform_ioremap_resource(pdev, 0); (*e).pmu_regs = (*e).regs; (*e).num_clks = devm_clk_bulk_get_all(&mut (*pdev).dev, &mut (*e).clks); if (*e).num_clks < 0 { return (*e).num_clks; }
    let ret = drmm_mutex_init(&mut (*e).base, &mut (*e).perfmon_state.lock); if ret != 0 { return ret; } let ret = ethosu_job_init(e); if ret != 0 { return ret; } let ret = ethosu_init(e); if ret != 0 { return ret; }
    let ret = drm_dev_register(&mut (*e).base, 0); if ret != 0 { pm_runtime_dont_use_autosuspend((*e).base.dev); } pm_runtime_put_autosuspend((*e).base.dev); ret
}
unsafe fn ethosu_remove(pdev: *mut platform_device) { let e = dev_get_drvdata(&mut (*pdev).dev); drm_dev_unregister(&mut (*e).base); ethosu_job_fini(e); if !(*e).sram.is_null() { gen_pool_free((*e).srampool, (*e).sram as usize, (*e).npu_info.sram_size); } }

// Device matching, runtime-PM operations, module metadata, ioctl tables, and DRM file operations are emitted by the corresponding kernel binding macros.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
