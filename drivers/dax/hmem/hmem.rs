// SPDX-License-Identifier: GPL-2.0
// C dependencies are supplied by the surrounding kernel translation.

static mut REGION_IDLE: bool = false;

static mut DAX_HMEM_WQ: *mut workqueue_struct = core::ptr::null_mut();

unsafe fn dax_hmem_probe(pdev: *mut platform_device) -> c_int {
    let mut flags: c_ulong = IORESOURCE_DAX_KMEM as c_ulong;
    let dev: *mut device = &mut (*pdev).dev;
    let mut dax_region: *mut dax_region;
    let mri: *mut memregion_info;
    let data: dev_dax_data;

    /*
     * @region_idle == true indicates that an administrative agent
     * wants to manipulate the range partitioning before the devices
     * are created, so do not send them to the dax_kmem driver by
     * default.
     */
    if REGION_IDLE {
        flags = 0;
    }

    mri = (*pdev).dev.platform_data as *mut memregion_info;
    dax_region = alloc_dax_region(dev, (*pdev).id, &(*mri).range,
                                  (*mri).target_node, PMD_SIZE, flags);
    if dax_region.is_null() {
        return -ENOMEM;
    }

    data = dev_dax_data {
        dax_region,
        id: -1,
        size: if REGION_IDLE { 0 } else { range_len(&(*mri).range) },
        memmap_on_memory: false,
    };

    PTR_ERR_OR_ZERO(devm_create_dev_dax(&data))
}

static mut DAX_HMEM_DRIVER: platform_driver = platform_driver {
    probe: Some(dax_hmem_probe),
    driver: driver { name: c"hmem".as_ptr() as *const c_char },
};

unsafe fn release_memregion(data: *mut core::ffi::c_void) {
    memregion_free(data as isize);
}

unsafe fn release_hmem(pdev: *mut core::ffi::c_void) {
    platform_device_unregister(pdev as *mut platform_device);
}

pub unsafe fn dax_hmem_flush_work() {
    flush_workqueue(DAX_HMEM_WQ);
}

unsafe fn __hmem_register_device(host: *mut device, target_nid: c_int,
                                  res: *const resource) -> c_int {
    let mut pdev: *mut platform_device;
    let mut info: memregion_info;
    let id: c_long;
    let mut rc: c_int;

    rc = region_intersects_soft_reserve((*res).start, resource_size(res));
    if rc != REGION_INTERSECTS { return 0; }

    // TODO: Add Soft-Reserved memory back to iomem
    id = memregion_alloc(GFP_KERNEL);
    if id < 0 { dev_err(host, c"memregion allocation failure for %pr\n", res); return -ENOMEM; }
    rc = devm_add_action_or_reset(host, Some(release_memregion), id as *mut _);
    if rc != 0 { return rc; }

    pdev = platform_device_alloc(c"hmem".as_ptr() as *const c_char, id as c_int);
    if pdev.is_null() { dev_err(host, c"device allocation failure for %pr\n", res); return -ENOMEM; }
    (*pdev).dev.parent = host;
    (*pdev).dev.numa_node = numa_map_to_online_node(target_nid);
    info = memregion_info { target_node: target_nid, range: resource { start: (*res).start, end: (*res).end } };
    rc = platform_device_add_data(pdev, &info as *const _ as *const _, core::mem::size_of::<memregion_info>());
    if rc < 0 { dev_err(host, c"memregion_info allocation failure for %pr\n", res); platform_device_put(pdev); return rc; }
    rc = platform_device_add(pdev);
    if rc < 0 { dev_err(host, c"%s add failed for %pr\n", dev_name(&(*pdev).dev), res); platform_device_put(pdev); return rc; }
    devm_add_action_or_reset(host, Some(release_hmem), pdev as *mut _)
}

unsafe fn hmem_register_cxl_device(host: *mut device, target_nid: c_int, res: *const resource) -> c_int {
    if region_intersects((*res).start, resource_size(res), IORESOURCE_MEM, IORES_DESC_CXL) == REGION_DISJOINT { return 0; }
    if cxl_region_contains_resource(res) { dev_dbg(host, c"CXL claims resource, dropping: %pr\n", res); return 0; }
    dev_dbg(host, c"CXL did not claim resource, registering: %pr\n", res);
    __hmem_register_device(host, target_nid, res)
}

unsafe fn process_defer_work(w: *mut work_struct) {
    let hpdev: *mut hmem_platform_device = container_of!(w, hmem_platform_device, work);
    let dev = &mut (*(*hpdev).pdev).dev;
    wait_for_device_probe();
    // C guard(device)(dev) is represented by the surrounding device guard API.
    if (*dev).driver.is_null() { put_device(dev); return; }
    if !(*hpdev).did_probe { (*hpdev).did_probe = true; walk_hmem_resources(dev, Some(hmem_register_cxl_device)); }
    put_device(dev);
}

unsafe fn hmem_register_device(host: *mut device, target_nid: c_int, res: *const resource) -> c_int {
    let pdev = to_platform_device(host);
    let hpdev = to_hmem_platform_device(pdev);
    if IS_ENABLED_CONFIG_DEV_DAX_CXL && region_intersects((*res).start, resource_size(res), IORESOURCE_MEM, IORES_DESC_CXL) != REGION_DISJOINT {
        if !(*hpdev).did_probe { dev_dbg(host, c"await CXL initial probe: %pr\n", res); (*hpdev).work.func = Some(process_defer_work); get_device(host); if !queue_work(DAX_HMEM_WQ, &mut (*hpdev).work) { put_device(host); } return 0; }
        dev_dbg(host, c"deferring range to CXL: %pr\n", res); return 0;
    }
    __hmem_register_device(host, target_nid, res)
}

unsafe fn dax_hmem_platform_probe(pdev: *mut platform_device) -> c_int {
    let hpdev = to_hmem_platform_device(pdev);
    if work_pending(&(*hpdev).work) { return -EBUSY; }
    walk_hmem_resources(&mut (*pdev).dev, Some(hmem_register_device))
}

static mut DAX_HMEM_PLATFORM_DRIVER: platform_driver = platform_driver {
    probe: Some(dax_hmem_platform_probe), driver: driver { name: c"hmem_platform".as_ptr() as *const c_char },
};

unsafe fn dax_hmem_init() -> c_int {
    let mut rc: c_int;
    if IS_ENABLED_CONFIG_DEV_DAX_CXL {
        request_module(c"cxl_acpi".as_ptr() as *const c_char);
        request_module(c"cxl_pci".as_ptr() as *const c_char);
    }
    DAX_HMEM_WQ = alloc_ordered_workqueue(c"dax_hmem_wq".as_ptr() as *const c_char, 0);
    if DAX_HMEM_WQ.is_null() { return -ENOMEM; }
    rc = platform_driver_register(&mut DAX_HMEM_PLATFORM_DRIVER);
    if rc != 0 { destroy_workqueue(DAX_HMEM_WQ); return rc; }
    rc = platform_driver_register(&mut DAX_HMEM_DRIVER);
    if rc != 0 { platform_driver_unregister(&mut DAX_HMEM_PLATFORM_DRIVER); destroy_workqueue(DAX_HMEM_WQ); return rc; }
    0
}

unsafe fn dax_hmem_exit() { platform_driver_unregister(&mut DAX_HMEM_DRIVER); platform_driver_unregister(&mut DAX_HMEM_PLATFORM_DRIVER); destroy_workqueue(DAX_HMEM_WQ); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
