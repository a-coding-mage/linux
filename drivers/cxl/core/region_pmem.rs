// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2022 Intel Corporation. All rights reserved. */
// Dependencies supplied by the surrounding kernel/CXL translation unit.

unsafe fn cxl_pmem_region_release(dev: *mut device) {
    let cxlr_pmem = to_cxl_pmem_region(dev);
    let mut i: i32 = 0;

    while i < (*cxlr_pmem).nr_mappings {
        let cxlmd = (*cxlr_pmem).mapping[i as usize].cxlmd;
        put_device(&mut (*cxlmd).dev);
        i += 1;
    }

    kfree(cxlr_pmem);
}

static mut CXL_PMEM_REGION_ATTRIBUTE_GROUPS: [*const attribute_group; 2] = [
    &cxl_base_attribute_group,
    core::ptr::null(),
];

pub static mut cxl_pmem_region_type: device_type = device_type {
    name: "cxl_pmem_region",
    release: Some(cxl_pmem_region_release),
    groups: CXL_PMEM_REGION_ATTRIBUTE_GROUPS.as_ptr(),
};

pub unsafe fn is_cxl_pmem_region(dev: *mut device) -> bool {
    (*dev).type_ == &cxl_pmem_region_type
}

// EXPORT_SYMBOL_NS_GPL(is_cxl_pmem_region, "CXL");

pub unsafe fn to_cxl_pmem_region(dev: *mut device) -> *mut cxl_pmem_region {
    if dev_WARN_ONCE(dev, !is_cxl_pmem_region(dev), "not a cxl_pmem_region device\n") {
        return core::ptr::null_mut();
    }
    container_of!(dev, cxl_pmem_region, dev)
}

// EXPORT_SYMBOL_NS_GPL(to_cxl_pmem_region, "CXL");

static mut cxl_pmem_region_key: lock_class_key = lock_class_key {};

unsafe fn cxl_pmem_region_alloc(cxlr: *mut cxl_region) -> i32 {
    let p = &mut (*cxlr).params;
    let mut cxl_nvb: *mut cxl_nvdimm_bridge;
    let mut dev: *mut device;
    let mut i: i32;

    // guard(rwsem_read)(&cxl_rwsem.region);
    if p.state != CXL_CONFIG_COMMIT {
        return -ENXIO;
    }

    let cxlr_pmem = kzalloc_flex::<cxl_pmem_region>(p.nr_targets);
    if cxlr_pmem.is_null() {
        return -ENOMEM;
    }

    (*cxlr_pmem).hpa_range.start = (*p.res).start;
    (*cxlr_pmem).hpa_range.end = (*p.res).end;

    /* Snapshot the region configuration underneath the cxl_rwsem.region */
    (*cxlr_pmem).nr_mappings = p.nr_targets;
    i = 0;
    while i < p.nr_targets {
        let cxled = p.targets[i as usize];
        let cxlmd = cxled_to_memdev(cxled);
        let m = &mut (*cxlr_pmem).mapping[i as usize];

        /*
         * Regions never span CXL root devices, so by definition the
         * bridge for one device is the same for all.
         */
        if i == 0 {
            cxl_nvb = cxl_find_nvdimm_bridge((*cxlmd).endpoint);
            if cxl_nvb.is_null() {
                kfree(cxlr_pmem as *mut core::ffi::c_void);
                return -ENODEV;
            }
            (*cxlr).cxl_nvb = cxl_nvb;
        }
        (*m).cxlmd = cxlmd;
        get_device(&mut (*cxlmd).dev);
        (*m).start = (*(*cxled).dpa_res).start;
        (*m).size = resource_size((*cxled).dpa_res);
        (*m).position = i;
        i += 1;
    }

    dev = &mut (*cxlr_pmem).dev;
    device_initialize(dev);
    lockdep_set_class(&mut (*dev).mutex, &mut cxl_pmem_region_key);
    device_set_pm_not_required(dev);
    (*dev).parent = &mut (*cxlr).dev;
    (*dev).bus = &mut cxl_bus_type;
    (*dev).type_ = &cxl_pmem_region_type;
    (*cxlr_pmem).cxlr = cxlr;
    (*cxlr).cxlr_pmem = cxlr_pmem;

    0
}

unsafe fn cxlr_pmem_unregister(_cxlr_pmem: *mut core::ffi::c_void) {
    let cxlr_pmem = _cxlr_pmem as *mut cxl_pmem_region;
    let cxlr = (*cxlr_pmem).cxlr;
    let cxl_nvb = (*cxlr).cxl_nvb;

    /*
     * Either the bridge is in ->remove() context under the device_lock(),
     * or cxlr_release_nvdimm() is cancelling the bridge's release action
     * for @cxlr_pmem and doing it itself (while manually holding the bridge
     * lock).
     */
    device_lock_assert(&mut (*cxl_nvb).dev);
    (*cxlr).cxlr_pmem = core::ptr::null_mut();
    (*cxlr_pmem).cxlr = core::ptr::null_mut();
    device_unregister(&mut (*cxlr_pmem).dev);
}

unsafe fn cxlr_release_nvdimm(_cxlr: *mut core::ffi::c_void) {
    let cxlr = _cxlr as *mut cxl_region;
    let cxl_nvb = (*cxlr).cxl_nvb;

    // scoped_guard(device, &cxl_nvb->dev)
    {
        if !(*cxlr).cxlr_pmem.is_null() {
            devm_release_action(&mut (*cxl_nvb).dev, cxlr_pmem_unregister,
                                (*cxlr).cxlr_pmem);
        }
    }
    (*cxlr).cxl_nvb = core::ptr::null_mut();
    put_device(&mut (*cxl_nvb).dev);
}

/**
 * devm_cxl_add_pmem_region() - add a cxl_region-to-nd_region bridge
 * @cxlr: parent CXL region for this pmem region bridge device
 *
 * Return: 0 on success negative error code on failure.
 */
pub unsafe fn devm_cxl_add_pmem_region(cxlr: *mut cxl_region) -> i32 {
    let mut cxlr_pmem: *mut cxl_pmem_region;
    let cxl_nvb: *mut cxl_nvdimm_bridge;
    let dev: *mut device;
    let mut rc: i32;

    rc = cxl_pmem_region_alloc(cxlr);
    if rc != 0 {
        return rc;
    }
    cxlr_pmem = (*cxlr).cxlr_pmem;
    cxl_nvb = (*cxlr).cxl_nvb;

    dev = &mut (*cxlr_pmem).dev;
    rc = dev_set_name(dev, "pmem_region%d", (*cxlr).id);
    if rc != 0 {
        put_device(dev);
        put_device(&mut (*cxl_nvb).dev);
        (*cxlr).cxl_nvb = core::ptr::null_mut();
        return rc;
    }

    rc = device_add(dev);
    if rc != 0 {
        put_device(dev);
        put_device(&mut (*cxl_nvb).dev);
        (*cxlr).cxl_nvb = core::ptr::null_mut();
        return rc;
    }

    dev_dbg(&mut (*cxlr).dev, "%s: register %s\n", dev_name((*dev).parent),
            dev_name(dev));

    // scoped_guard(device, &cxl_nvb->dev)
    {
        if !(*cxl_nvb).dev.driver.is_null() {
            rc = devm_add_action_or_reset(&mut (*cxl_nvb).dev,
                                          cxlr_pmem_unregister,
                                          cxlr_pmem as *mut core::ffi::c_void);
        } else {
            rc = -ENXIO;
        }
    }

    if rc != 0 {
        put_device(dev);
        put_device(&mut (*cxl_nvb).dev);
        (*cxlr).cxl_nvb = core::ptr::null_mut();
        return rc;
    }

    /* @cxlr carries a reference on @cxl_nvb until cxlr_release_nvdimm */
    devm_add_action_or_reset(&mut (*cxlr).dev, cxlr_release_nvdimm,
                             cxlr as *mut core::ffi::c_void)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
