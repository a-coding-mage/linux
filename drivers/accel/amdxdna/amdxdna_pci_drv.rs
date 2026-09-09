// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2022-2024, Advanced Micro Devices, Inc.
 */

// External Linux/DRM headers and local driver headers provide the referenced types,
// constants, functions, macros, and ABI definitions.

// MODULE_FIRMWARE("amdnpu/1502_00/npu.sbin");
// MODULE_FIRMWARE("amdnpu/17f0_10/npu.sbin");
// MODULE_FIRMWARE("amdnpu/17f0_11/npu.sbin");
// MODULE_FIRMWARE("amdnpu/17f0_20/npu.sbin");
// MODULE_FIRMWARE("amdnpu/1502_00/npu_7.sbin");
// MODULE_FIRMWARE("amdnpu/17f0_10/npu_7.sbin");
// MODULE_FIRMWARE("amdnpu/17f0_11/npu_7.sbin");

/*
 * 0.0: Initial version
 * 0.1: Support getting all hardware contexts by DRM_IOCTL_AMDXDNA_GET_ARRAY
 * 0.2: Support getting last error hardware error
 * 0.3: Support firmware debug buffer
 * 0.4: Support getting resource information
 * 0.5: Support getting telemetry data
 * 0.6: Support preemption
 * 0.7: Support getting power and utilization data
 * 0.8: Support BO usage query
 * 0.9: Add new device type AMDXDNA_DEV_TYPE_PF
 * 0.10: Support AIE4 UMQ
 */
pub const AMDXDNA_DRIVER_MAJOR: i32 = 0;
pub const AMDXDNA_DRIVER_MINOR: i32 = 10;

// Bind the driver based on (vendor_id, device_id), then select devices by (device_id, rev_id).
static PCI_IDS: &[pci_device_id] = &[
    PCI_DEVICE(PCI_VENDOR_ID_AMD, 0x1502),
    PCI_DEVICE(PCI_VENDOR_ID_AMD, 0x17f0),
    PCI_DEVICE(PCI_VENDOR_ID_AMD, 0x17f2),
    PCI_DEVICE(PCI_VENDOR_ID_AMD, 0x17f3),
    PCI_DEVICE(PCI_VENDOR_ID_AMD, 0x1B0B),
    PCI_DEVICE(PCI_VENDOR_ID_AMD, 0x1B0C),
    pci_device_id { ..unsafe { core::mem::zeroed() } },
];

static AMDXDNA_IDS: &[amdxdna_device_id] = &[
    amdxdna_device_id { device: 0x1502, revision: 0x0, dev_info: &dev_npu1_info },
    amdxdna_device_id { device: 0x17f0, revision: 0x10, dev_info: &dev_npu4_info },
    amdxdna_device_id { device: 0x17f0, revision: 0x11, dev_info: &dev_npu5_info },
    amdxdna_device_id { device: 0x17f0, revision: 0x20, dev_info: &dev_npu6_info },
    amdxdna_device_id { device: 0x17f2, revision: 0x10, dev_info: &dev_npu3_pf_info },
    amdxdna_device_id { device: 0x17f3, revision: 0x10, dev_info: &dev_npu3_vf_info },
    amdxdna_device_id { device: 0x1B0B, revision: 0x10, dev_info: &dev_npu3_pf_info },
    amdxdna_device_id { device: 0x1B0C, revision: 0x10, dev_info: &dev_npu3_vf_info },
    amdxdna_device_id { ..unsafe { core::mem::zeroed() } },
];

unsafe fn amdxdna_sva_init(client: *mut amdxdna_client) -> i32 {
    let xdna = (*client).xdna;
    (*client).sva = iommu_sva_bind_device((*xdna).ddev.dev, (*client).mm);
    if IS_ERR((*client).sva) {
        XDNA_ERR(xdna, "SVA bind device failed, ret %ld", PTR_ERR((*client).sva));
        return PTR_ERR((*client).sva) as i32;
    }
    (*client).pasid = iommu_sva_get_pasid((*client).sva);
    if (*client).pasid == IOMMU_PASID_INVALID {
        iommu_sva_unbind_device((*client).sva);
        (*client).sva = core::ptr::null_mut();
        XDNA_ERR(xdna, "SVA get pasid failed");
        return -ENODEV;
    }
    0
}

unsafe fn amdxdna_sva_fini(client: *mut amdxdna_client) {
    if IS_ERR_OR_NULL((*client).sva) { return; }
    iommu_sva_unbind_device((*client).sva);
    (*client).sva = core::ptr::null_mut();
    (*client).pasid = IOMMU_PASID_INVALID;
}

unsafe fn amdxdna_drm_open(ddev: *mut drm_device, filp: *mut drm_file) -> i32 {
    let xdna = to_xdna_dev(ddev);
    let client = kzalloc_obj::<amdxdna_client>();
    if client.is_null() { return -ENOMEM; }
    let mut ret = init_srcu_struct(&mut (*client).hwctx_srcu);
    if ret != 0 { kfree(client); return ret; }
    (*client).pid = pid_nr(rcu_access_pointer((*filp).pid));
    (*client).xdna = xdna;
    (*client).pasid = IOMMU_PASID_INVALID;
    (*client).mm = current_mm();
    if !amdxdna_iova_on(xdna) {
        if amdxdna_sva_init(client) != 0 {
            XDNA_WARN(xdna, "PASID not available for pid %d", (*client).pid);
            if !amdxdna_use_carveout(xdna) {
                XDNA_ERR(xdna, "PASID unavailable and carveout not configured");
                ret = -EINVAL;
                cleanup_srcu_struct(&mut (*client).hwctx_srcu);
                kfree(client);
                return ret;
            }
        }
    }
    mmgrab((*client).mm);
    xa_init_flags(&mut (*client).hwctx_xa, XA_FLAGS_ALLOC);
    xa_init_flags(&mut (*client).dev_heap_xa, XA_FLAGS_ALLOC);
    drm_mm_init(&mut (*client).dev_heap_mm, (*xdna).dev_info.dev_mem_base,
                (*xdna).dev_info.dev_heap_max_size);
    mutex_init(&mut (*client).mm_lock);
    mutex_lock(&mut (*xdna).client_lock);
    mutex_lock(&mut (*xdna).dev_lock);
    list_add_tail(&mut (*client).node, &mut (*xdna).client_list);
    mutex_unlock(&mut (*xdna).dev_lock);
    mutex_unlock(&mut (*xdna).client_lock);
    (*filp).driver_priv = client as *mut _;
    (*client).filp = filp;
    XDNA_DBG(xdna, "pid %d opened", (*client).pid);
    0
}

unsafe fn amdxdna_client_cleanup(client: *mut amdxdna_client) {
    let mut heap: *mut amdxdna_gem_obj = core::ptr::null_mut();
    let mut heap_id: c_ulong = 0;
    list_del(&mut (*client).node);
    amdxdna_hwctx_remove_all(client);
    xa_destroy(&mut (*client).hwctx_xa);
    cleanup_srcu_struct(&mut (*client).hwctx_srcu);
    xa_for_each(&mut (*client).dev_heap_xa, &mut heap_id, &mut heap) {
        drm_gem_object_put(to_gobj(heap));
    }
    xa_destroy(&mut (*client).dev_heap_xa);
    drm_mm_takedown(&mut (*client).dev_heap_mm);
    mutex_destroy(&mut (*client).mm_lock);
    mmdrop((*client).mm);
    amdxdna_sva_fini(client);
    kfree(client);
}

unsafe fn amdxdna_drm_close(ddev: *mut drm_device, filp: *mut drm_file) {
    let client = (*filp).driver_priv as *mut amdxdna_client;
    let xdna = to_xdna_dev(ddev);
    XDNA_DBG(xdna, "closing pid %d", (*client).pid);
    mutex_lock(&mut (*xdna).client_lock);
    mutex_lock(&mut (*xdna).dev_lock);
    amdxdna_client_cleanup(client);
    mutex_unlock(&mut (*xdna).dev_lock);
    mutex_unlock(&mut (*xdna).client_lock);
}

unsafe fn amdxdna_drm_get_info_ioctl(dev: *mut drm_device, data: *mut c_void, filp: *mut drm_file) -> i32 {
    let client = (*filp).driver_priv as *mut amdxdna_client;
    let xdna = to_xdna_dev(dev);
    let args = data as *mut amdxdna_drm_get_info;
    if (*xdna).dev_info.ops.get_aie_info.is_none() { return -EOPNOTSUPP; }
    XDNA_DBG(xdna, "Request parameter %u", (*args).param);
    mutex_lock(&mut (*xdna).dev_lock);
    let ret = ((*xdna).dev_info.ops.get_aie_info.unwrap())(client, args);
    mutex_unlock(&mut (*xdna).dev_lock);
    ret
}

unsafe fn amdxdna_drm_get_array_ioctl(dev: *mut drm_device, data: *mut c_void, filp: *mut drm_file) -> i32 {
    let client = (*filp).driver_priv as *mut amdxdna_client;
    let xdna = to_xdna_dev(dev);
    let args = data as *mut amdxdna_drm_get_array;
    if (*xdna).dev_info.ops.get_array.is_none() { return -EOPNOTSUPP; }
    if (*args).pad != 0 || (*args).num_element == 0 || (*args).element_size == 0 { return -EINVAL; }
    let _guard = mutex_guard(&mut (*xdna).dev_lock);
    ((*xdna).dev_info.ops.get_array.unwrap())(client, args)
}

unsafe fn amdxdna_drm_set_state_ioctl(dev: *mut drm_device, data: *mut c_void, filp: *mut drm_file) -> i32 {
    let client = (*filp).driver_priv as *mut amdxdna_client;
    let xdna = to_xdna_dev(dev);
    let args = data as *mut amdxdna_drm_set_state;
    if (*xdna).dev_info.ops.set_aie_state.is_none() { return -EOPNOTSUPP; }
    XDNA_DBG(xdna, "Request parameter %u", (*args).param);
    mutex_lock(&mut (*xdna).dev_lock);
    let ret = ((*xdna).dev_info.ops.set_aie_state.unwrap())(client, args);
    mutex_unlock(&mut (*xdna).dev_lock);
    ret
}

unsafe fn amdxdna_drm_gem_mmap(filp: *mut file, vma: *mut vm_area_struct) -> i32 {
    let drm_filp = (*filp).private_data as *mut drm_file;
    let client = (*drm_filp).driver_priv as *mut amdxdna_client;
    let xdna = (*client).xdna;
    if (*vma).vm_pgoff >= DRM_FILE_PAGE_OFFSET_START { return drm_gem_mmap(filp, vma); }
    if (*xdna).dev_info.ops.mmap.is_none() { return -EOPNOTSUPP; }
    ((*xdna).dev_info.ops.mmap.unwrap())(client, vma)
}

// DRM_IOCTL_DEF_DRV entries:
// CREATE_HWCTX, DESTROY_HWCTX, CONFIG_HWCTX, CREATE_BO, GET_BO_INFO, SYNC_BO,
// EXEC_CMD, WAIT_CMD, GET_INFO, GET_ARRAY, SET_STATE (DRM_ROOT_ONLY).

unsafe fn amdxdna_show_fdinfo(p: *mut drm_printer, filp: *mut drm_file) {
    let client = (*filp).driver_priv as *mut amdxdna_client;
    let drv_name = (*(*(*filp).minor).dev).driver.name;
    mutex_lock(&mut (*client).mm_lock);
    let heap_usage = (*client).heap_usage;
    let internal_usage = (*client).total_int_bo_usage;
    let external_usage = (*client).total_bo_usage - internal_usage;
    mutex_unlock(&mut (*client).mm_lock);
    drm_fdinfo_print_size(p, drv_name, "heap", "alloc", heap_usage);
    drm_fdinfo_print_size(p, drv_name, "internal", "alloc", internal_usage);
    drm_fdinfo_print_size(p, drv_name, "external", "alloc", external_usage);
    drm_show_memory_stats(p, filp);
}

unsafe fn amdxdna_get_dev_info(pdev: *mut pci_dev) -> *const amdxdna_dev_info {
    for id in AMDXDNA_IDS.iter() {
        if (*pdev).device == id.device && (*pdev).revision == id.revision { return id.dev_info; }
    }
    core::ptr::null()
}

unsafe fn amdxdna_xdna_drm_release(_drm: *mut drm_device, res: *mut c_void) {
    amdxdna_carveout_fini(res as *mut amdxdna_dev);
}

unsafe fn amdxdna_probe(pdev: *mut pci_dev, _id: *const pci_device_id) -> i32 {
    let dev = &mut (*pdev).dev;
    let xdna = devm_drm_dev_alloc(dev, &amdxdna_drm_drv);
    if IS_ERR(xdna) { return PTR_ERR(xdna) as i32; }
    let xdna = xdna as *mut amdxdna_dev;
    (*xdna).dev_info = amdxdna_get_dev_info(pdev);
    if (*xdna).dev_info.is_null() { return -ENODEV; }
    let ddev = &mut (*xdna).ddev;
    let mut ret = drmm_mutex_init(ddev, &mut (*xdna).client_lock);
    if ret != 0 { return ret; }
    ret = drmm_mutex_init(ddev, &mut (*xdna).dev_lock);
    if ret != 0 { return ret; }
    init_rwsem(&mut (*xdna).notifier_lock);
    INIT_LIST_HEAD(&mut (*xdna).client_list);
    pci_set_drvdata(pdev, xdna);
    ret = drmm_add_action(ddev, amdxdna_xdna_drm_release, xdna as *mut _);
    if ret != 0 { return ret; }
    ret = amdxdna_iommu_init(xdna);
    if ret != 0 { return ret; }
    (*xdna).notifier_wq = drmm_alloc_ordered_workqueue(ddev, "notifier_wq", WQ_MEM_RECLAIM);
    if IS_ERR((*xdna).notifier_wq) { amdxdna_iommu_fini(xdna); return PTR_ERR((*xdna).notifier_wq) as i32; }
    mutex_lock(&mut (*xdna).dev_lock);
    ret = ((*xdna).dev_info.ops.init.unwrap())(xdna);
    mutex_unlock(&mut (*xdna).dev_lock);
    if ret != 0 { amdxdna_iommu_fini(xdna); return ret; }
    ret = amdxdna_sysfs_init(xdna);
    if ret != 0 { ((*xdna).dev_info.ops.fini.unwrap())(xdna); amdxdna_iommu_fini(xdna); return ret; }
    ret = drm_dev_register(ddev, 0);
    if ret != 0 { amdxdna_sysfs_fini(xdna); ((*xdna).dev_info.ops.fini.unwrap())(xdna); amdxdna_iommu_fini(xdna); return ret; }
    amdxdna_debugfs_init(xdna);
    0
}

unsafe fn amdxdna_remove(pdev: *mut pci_dev) {
    let xdna = pci_get_drvdata(pdev) as *mut amdxdna_dev;
    drm_dev_unplug(&mut (*xdna).ddev);
    amdxdna_sysfs_fini(xdna);
    mutex_lock(&mut (*xdna).client_lock);
    mutex_lock(&mut (*xdna).dev_lock);
    list_for_each_entry::<amdxdna_client>(&mut (*xdna).client_list, |client| {
        amdxdna_hwctx_remove_all(client);
        amdxdna_sva_fini(client);
    });
    ((*xdna).dev_info.ops.fini.unwrap())(xdna);
    mutex_unlock(&mut (*xdna).dev_lock);
    mutex_unlock(&mut (*xdna).client_lock);
    amdxdna_iommu_fini(xdna);
}

unsafe fn amdxdna_sriov_configure(pdev: *mut pci_dev, num_vfs: i32) -> i32 {
    let xdna = pci_get_drvdata(pdev) as *mut amdxdna_dev;
    let _guard = mutex_guard(&mut (*xdna).dev_lock);
    if let Some(configure) = (*xdna).dev_info.ops.sriov_configure { return configure(xdna, num_vfs); }
    -ENOENT
}

// Kernel registration equivalent: module_pci_driver(amdxdna_pci_driver).
// MODULE_LICENSE("GPL"); MODULE_IMPORT_NS("AMD_PMF");
// MODULE_AUTHOR("XRT Team <runtimeca39d@amd.com>"); MODULE_DESCRIPTION("amdxdna driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
