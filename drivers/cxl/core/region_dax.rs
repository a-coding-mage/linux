// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright(c) 2022 Intel Corporation. All rights reserved.
 * Copyright(c) 2026 Meta Technologies Inc. All rights reserved.
 */

// Dependencies supplied by the Linux CXL implementation are intentionally
// referenced here rather than reimplemented in this translation unit.

unsafe fn cxl_dax_region_release(dev: *mut device) {
    let cxlr_dax = to_cxl_dax_region(dev);
    kfree(cxlr_dax);
}

static mut cxl_dax_region_attribute_groups: [*const attribute_group; 2] = [
    &cxl_base_attribute_group,
    core::ptr::null(),
];

#[repr(C)]
static cxl_dax_region_type: device_type = device_type {
    name: "cxl_dax_region\0".as_ptr() as *const core::ffi::c_char,
    release: Some(cxl_dax_region_release),
    groups: unsafe { cxl_dax_region_attribute_groups.as_ptr() },
};

unsafe fn is_cxl_dax_region(dev: *mut device) -> bool {
    (*dev).type_ == &cxl_dax_region_type
}

#[no_mangle]
pub unsafe fn to_cxl_dax_region(dev: *mut device) -> *mut cxl_dax_region {
    if dev_WARN_ONCE(
        dev,
        !is_cxl_dax_region(dev),
        "not a cxl_dax_region device\n\0".as_ptr() as *const core::ffi::c_char,
    ) {
        return core::ptr::null_mut();
    }
    container_of_device(dev)
}

// EXPORT_SYMBOL_NS_GPL(to_cxl_dax_region, "CXL");

static mut cxl_dax_region_key: lock_class_key = lock_class_key {};

unsafe fn cxl_dax_region_alloc(cxlr: *mut cxl_region) -> *mut cxl_dax_region {
    let p = &mut (*cxlr).params;
    let cxlr_dax = kzalloc_obj::<cxl_dax_region>();

    // guard(rwsem_read)(&cxl_rwsem.region);
    if p.state != CXL_CONFIG_COMMIT {
        return ERR_PTR(-ENXIO);
    }

    if cxlr_dax.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    (*cxlr_dax).hpa_range.start = (*p.res).start;
    (*cxlr_dax).hpa_range.end = (*p.res).end;

    let dev = &mut (*cxlr_dax).dev;
    (*cxlr_dax).cxlr = cxlr;
    device_initialize(dev);
    lockdep_set_class(&mut (*dev).mutex, &mut cxl_dax_region_key);
    device_set_pm_not_required(dev);
    (*dev).parent = &mut (*cxlr).dev;
    (*dev).bus = &cxl_bus_type;
    (*dev).type_ = &cxl_dax_region_type;

    cxlr_dax
}

unsafe fn cxlr_dax_unregister(_cxlr_dax: *mut core::ffi::c_void) {
    let cxlr_dax = _cxlr_dax as *mut cxl_dax_region;
    device_unregister(&mut (*cxlr_dax).dev);
}

pub unsafe fn devm_cxl_add_dax_region(cxlr: *mut cxl_region) -> i32 {
    let mut rc: i32;

    let cxlr_dax = cxl_dax_region_alloc(cxlr);
    if IS_ERR(cxlr_dax) {
        return PTR_ERR(cxlr_dax);
    }

    let dev = &mut (*cxlr_dax).dev;
    rc = dev_set_name(dev, "dax_region%d\0".as_ptr() as *const core::ffi::c_char, (*cxlr).id);
    if rc != 0 {
        return rc;
    }

    rc = device_add(dev);
    if rc != 0 {
        return rc;
    }

    dev_dbg(
        &mut (*cxlr).dev,
        "%s: register %s\n\0".as_ptr() as *const core::ffi::c_char,
        dev_name((*dev).parent),
        dev_name(dev),
    );

    devm_add_action_or_reset(
        &mut (*cxlr).dev,
        Some(cxlr_dax_unregister),
        cxlr_dax as *mut core::ffi::c_void,
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
