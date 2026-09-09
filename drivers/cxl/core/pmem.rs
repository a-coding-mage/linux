// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2020 Intel Corporation. */
// Dependencies are supplied by the surrounding kernel translation.

/**
 * DOC: cxl pmem
 *
 * The core CXL PMEM infrastructure supports persistent memory
 * provisioning and serves as a bridge to the LIBNVDIMM subsystem. A CXL
 * 'bridge' device is added at the root of a CXL device topology if
 * platform firmware advertises at least one persistent memory capable
 * CXL window. That root-level bridge corresponds to a LIBNVDIMM 'bus'
 * device. Then for each cxl_memdev in the CXL device topology a bridge
 * device is added to host a LIBNVDIMM dimm object. When these bridges are
 * registered native LIBNVDIMM uapis are translated to CXL operations, for
 * example, namespace label access commands.
 */

static mut CXL_NVDIMM_BRIDGE_IDA: Ida = DEFINE_IDA!();

unsafe extern "C" fn cxl_nvdimm_bridge_release(dev: *mut device) {
    let cxl_nvb = to_cxl_nvdimm_bridge(dev);
    ida_free(&raw mut CXL_NVDIMM_BRIDGE_IDA, (*cxl_nvb).id);
    kfree(cxl_nvb);
}

static CXL_NVDIMM_BRIDGE_ATTRIBUTE_GROUPS: [*const attribute_group; 2] = [
    &cxl_base_attribute_group,
    core::ptr::null(),
];

#[no_mangle]
pub static cxl_nvdimm_bridge_type: device_type = device_type {
    name: c"cxl_nvdimm_bridge".as_ptr(),
    release: Some(cxl_nvdimm_bridge_release),
    groups: CXL_NVDIMM_BRIDGE_ATTRIBUTE_GROUPS.as_ptr(),
};

#[no_mangle]
pub unsafe extern "C" fn to_cxl_nvdimm_bridge(dev: *mut device) -> *mut cxl_nvdimm_bridge {
    if dev_WARN_ONCE(dev, (*dev).type_ != &raw const cxl_nvdimm_bridge_type,
                     c"not a cxl_nvdimm_bridge device\n".as_ptr()) {
        return core::ptr::null_mut();
    }
    container_of!(dev, cxl_nvdimm_bridge, dev)
}

#[no_mangle]
pub unsafe extern "C" fn cxl_find_nvdimm_bridge(port: *mut cxl_port) -> *mut cxl_nvdimm_bridge {
    let cxl_root = find_cxl_root(port);
    if cxl_root.is_null() { return core::ptr::null_mut(); }
    let dev = device_find_child(&mut (*(*cxl_root).port).dev,
                                &raw const cxl_nvdimm_bridge_type,
                                Some(device_match_type));
    if dev.is_null() { return core::ptr::null_mut(); }
    to_cxl_nvdimm_bridge(dev)
}

static mut CXL_NVDIMM_BRIDGE_KEY: lock_class_key = lock_class_key {};

unsafe fn cxl_nvdimm_bridge_alloc(port: *mut cxl_port) -> *mut cxl_nvdimm_bridge {
    let cxl_nvb = kzalloc_obj::<cxl_nvdimm_bridge>();
    if cxl_nvb.is_null() { return ERR_PTR(-ENOMEM); }
    let rc = ida_alloc(&raw mut CXL_NVDIMM_BRIDGE_IDA, GFP_KERNEL);
    if rc < 0 { kfree(cxl_nvb); return ERR_PTR(rc); }
    (*cxl_nvb).id = rc;
    let dev = &mut (*cxl_nvb).dev;
    (*cxl_nvb).port = port;
    device_initialize(dev);
    lockdep_set_class(&mut (*dev).mutex, &raw mut CXL_NVDIMM_BRIDGE_KEY);
    device_set_pm_not_required(dev);
    (*dev).parent = &mut (*port).dev;
    (*dev).bus = &raw const cxl_bus_type;
    (*dev).type_ = &raw const cxl_nvdimm_bridge_type;
    cxl_nvb
}

unsafe extern "C" fn unregister_nvb(data: *mut core::ffi::c_void) {
    let cxl_nvb = data as *mut cxl_nvdimm_bridge;
    device_unregister(&mut (*cxl_nvb).dev);
}

unsafe fn cxl_nvdimm_bridge_failed_attach(cxl_nvb: *mut cxl_nvdimm_bridge) -> bool {
    let dev = &mut (*cxl_nvb).dev;
    // guard(device)(dev)
    (*dev).driver.is_null()
}

#[no_mangle]
pub unsafe extern "C" fn __devm_cxl_add_nvdimm_bridge(host: *mut device,
                                                        port: *mut cxl_port) -> *mut cxl_nvdimm_bridge {
    if !IS_ENABLED!(CONFIG_CXL_PMEM) { return ERR_PTR(-ENXIO); }
    let cxl_nvb = cxl_nvdimm_bridge_alloc(port);
    if IS_ERR(cxl_nvb) { return cxl_nvb; }
    let dev = &mut (*cxl_nvb).dev;
    let mut rc = dev_set_name(dev, c"nvdimm-bridge%d".as_ptr(), (*cxl_nvb).id);
    if rc != 0 { put_device(dev); return ERR_PTR(rc); }
    rc = device_add(dev);
    if rc != 0 { put_device(dev); return ERR_PTR(rc); }
    if cxl_nvdimm_bridge_failed_attach(cxl_nvb) {
        unregister_nvb(cxl_nvb.cast());
        return ERR_PTR(-ENODEV);
    }
    rc = devm_add_action_or_reset(host, Some(unregister_nvb), cxl_nvb.cast());
    if rc != 0 { return ERR_PTR(rc); }
    cxl_nvb
}

unsafe extern "C" fn cxl_nvdimm_release(dev: *mut device) {
    kfree(to_cxl_nvdimm(dev));
}

static CXL_NVDIMM_ATTRIBUTE_GROUPS: [*const attribute_group; 2] = [
    &cxl_base_attribute_group, core::ptr::null(),
];

#[no_mangle]
pub static cxl_nvdimm_type: device_type = device_type {
    name: c"cxl_nvdimm".as_ptr(), release: Some(cxl_nvdimm_release),
    groups: CXL_NVDIMM_ATTRIBUTE_GROUPS.as_ptr(),
};

#[no_mangle]
pub unsafe extern "C" fn is_cxl_nvdimm(dev: *mut device) -> bool {
    (*dev).type_ == &raw const cxl_nvdimm_type
}

#[no_mangle]
pub unsafe extern "C" fn to_cxl_nvdimm(dev: *mut device) -> *mut cxl_nvdimm {
    if dev_WARN_ONCE(dev, !is_cxl_nvdimm(dev), c"not a cxl_nvdimm device\n".as_ptr()) {
        return core::ptr::null_mut();
    }
    container_of!(dev, cxl_nvdimm, dev)
}

static mut CXL_NVDIMM_KEY: lock_class_key = lock_class_key {};

unsafe fn cxl_nvdimm_alloc(cxl_nvb: *mut cxl_nvdimm_bridge,
                           cxlmd: *mut cxl_memdev) -> *mut cxl_nvdimm {
    let cxl_nvd = kzalloc_obj::<cxl_nvdimm>();
    if cxl_nvd.is_null() { return ERR_PTR(-ENOMEM); }
    let dev = &mut (*cxl_nvd).dev;
    (*cxl_nvd).cxlmd = cxlmd; (*cxlmd).cxl_nvd = cxl_nvd;
    device_initialize(dev);
    lockdep_set_class(&mut (*dev).mutex, &raw mut CXL_NVDIMM_KEY);
    device_set_pm_not_required(dev); (*dev).parent = &mut (*cxlmd).dev;
    (*dev).bus = &raw const cxl_bus_type; (*dev).type_ = &raw const cxl_nvdimm_type;
    BUILD_BUG_ON!(core::mem::size_of::<[u8; 21]>() < 21 || core::mem::size_of::<[u8; 21]>() > NVDIMM_KEY_DESC_LEN);
    sprintf((*cxl_nvd).dev_id.as_mut_ptr(), c"%llu".as_ptr(), (*(*cxlmd).cxlds).serial);
    cxl_nvd
}

unsafe extern "C" fn cxlmd_release_nvdimm(data: *mut core::ffi::c_void) {
    let cxlmd = data as *mut cxl_memdev; let cxl_nvd = (*cxlmd).cxl_nvd;
    let cxl_nvb = (*cxlmd).cxl_nvb;
    (*cxl_nvd).cxlmd = core::ptr::null_mut(); (*cxlmd).cxl_nvd = core::ptr::null_mut();
    (*cxlmd).cxl_nvb = core::ptr::null_mut(); device_unregister(&mut (*cxl_nvd).dev);
    put_device(&mut (*cxl_nvb).dev);
}

#[no_mangle]
pub unsafe extern "C" fn devm_cxl_add_nvdimm(host: *mut device, port: *mut cxl_port,
                                               cxlmd: *mut cxl_memdev) -> c_int {
    let cxl_nvb = cxl_find_nvdimm_bridge(port);
    if cxl_nvb.is_null() { return -ENODEV; }
    // guard(device)(cxl_nvb->port->uport_dev); guard(device)(&cxl_nvb->dev);
    if (*cxl_nvb).nvdimm_bus.is_null() { put_device(&mut (*cxl_nvb).dev); return -ENODEV; }
    let cxl_nvd = cxl_nvdimm_alloc(cxl_nvb, cxlmd);
    if IS_ERR(cxl_nvd) { put_device(&mut (*cxl_nvb).dev); return PTR_ERR(cxl_nvd); }
    (*cxlmd).cxl_nvb = cxl_nvb;
    let dev = &mut (*cxl_nvd).dev;
    let mut rc = dev_set_name(dev, c"pmem%d".as_ptr(), (*cxlmd).id);
    if rc == 0 { rc = device_add(dev); }
    if rc != 0 { put_device(dev); (*cxlmd).cxl_nvb = core::ptr::null_mut(); (*cxlmd).cxl_nvd = core::ptr::null_mut(); put_device(&mut (*cxl_nvb).dev); return rc; }
    dev_dbg(host, c"register %s\n".as_ptr(), dev_name(dev));
    devm_add_action_or_reset(host, Some(cxlmd_release_nvdimm), cxlmd.cast())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
