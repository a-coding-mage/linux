// SPDX-License-Identifier: GPL-2.0
/*
 * FPGA Region - Support for FPGA programming under Linux
 *
 *  Copyright (C) 2013-2016 Altera Corporation
 *  Copyright (C) 2017 Intel Corporation
 */
// Dependencies supplied by the Linux FPGA, device, IDA, module, slab, and
// synchronization subsystems are intentionally referenced but not defined here.

static mut FPGA_REGION_IDA: Ida = DEFINE_IDA!();
static FPGA_REGION_CLASS: Class = Class;

pub unsafe fn fpga_region_class_find(
    start: *mut Device,
    data: *const core::ffi::c_void,
    r#match: Option<unsafe extern "C" fn(*mut Device, *const core::ffi::c_void) -> i32>,
) -> *mut FpgaRegion {
    let dev = class_find_device(&FPGA_REGION_CLASS, start, data, r#match);
    if dev.is_null() {
        return core::ptr::null_mut();
    }
    to_fpga_region(dev)
}

/// fpga_region_get - get an exclusive reference to an fpga region
/// @region: FPGA Region struct
///
/// Caller should call fpga_region_put() when done with region.
///
/// Return:
/// * fpga_region struct if successful.
/// * -EBUSY if someone already has a reference to the region.
/// * -ENODEV if can't take parent driver module refcount.
unsafe fn fpga_region_get(region: *mut FpgaRegion) -> *mut FpgaRegion {
    let dev = &mut (*region).dev;
    if !mutex_trylock(&mut (*region).mutex) {
        dev_dbg(dev, "%s: FPGA Region already in use\n", "fpga_region_get");
        return ERR_PTR(-EBUSY);
    }
    get_device(dev);
    if !try_module_get((*region).ops_owner) {
        put_device(dev);
        mutex_unlock(&mut (*region).mutex);
        return ERR_PTR(-ENODEV);
    }
    dev_dbg(dev, "get\n");
    region
}

/// fpga_region_put - release a reference to a region
/// @region: FPGA region
unsafe fn fpga_region_put(region: *mut FpgaRegion) {
    let dev = &mut (*region).dev;
    dev_dbg(dev, "put\n");
    module_put((*region).ops_owner);
    put_device(dev);
    mutex_unlock(&mut (*region).mutex);
}

/// fpga_region_program_fpga - program FPGA
/// @region: FPGA region
pub unsafe fn fpga_region_program_fpga(mut region: *mut FpgaRegion) -> i32 {
    let dev = &mut (*region).dev;
    let info = (*region).info;
    region = fpga_region_get(region);
    if IS_ERR(region) {
        dev_err(dev, "failed to get FPGA region\n");
        return PTR_ERR(region);
    }
    let mut ret = fpga_mgr_lock((*region).mgr);
    if ret != 0 {
        dev_err(dev, "FPGA manager is busy\n");
        fpga_region_put(region);
        return ret;
    }
    if let Some(get_bridges) = (*region).get_bridges {
        ret = get_bridges(region);
        if ret != 0 {
            dev_err(dev, "failed to get fpga region bridges\n");
            fpga_mgr_unlock((*region).mgr);
            fpga_region_put(region);
            return ret;
        }
    }
    ret = fpga_bridges_disable(&mut (*region).bridge_list);
    if ret != 0 {
        dev_err(dev, "failed to disable bridges\n");
        if (*region).get_bridges.is_some() { fpga_bridges_put(&mut (*region).bridge_list); }
        fpga_mgr_unlock((*region).mgr);
        fpga_region_put(region);
        return ret;
    }
    ret = fpga_mgr_load((*region).mgr, info);
    if ret != 0 {
        dev_err(dev, "failed to load FPGA image\n");
        if (*region).get_bridges.is_some() { fpga_bridges_put(&mut (*region).bridge_list); }
        fpga_mgr_unlock((*region).mgr);
        fpga_region_put(region);
        return ret;
    }
    ret = fpga_bridges_enable(&mut (*region).bridge_list);
    if ret != 0 {
        dev_err(dev, "failed to enable region bridges\n");
        if (*region).get_bridges.is_some() { fpga_bridges_put(&mut (*region).bridge_list); }
        fpga_mgr_unlock((*region).mgr);
        fpga_region_put(region);
        return ret;
    }
    fpga_mgr_unlock((*region).mgr);
    fpga_region_put(region);
    0
}

unsafe extern "C" fn compat_id_show(dev: *mut Device, _attr: *mut DeviceAttribute, buf: *mut i8) -> Isize {
    let region = to_fpga_region(dev);
    if (*region).compat_id.is_null() { return -ENOENT as Isize; }
    sprintf(buf, "%016llx%016llx\n", (*(*region).compat_id).id_h as u64, (*(*region).compat_id).id_l as u64)
}

static DEV_ATTR_COMPAT_ID: DeviceAttribute = DEVICE_ATTR_RO!(compat_id);
static mut FPGA_REGION_ATTRS: [*mut Attribute; 2] = [
    &DEV_ATTR_COMPAT_ID.attr as *const _ as *mut _, core::ptr::null_mut(),
];
static FPGA_REGION_GROUPS: AttributeGroups = ATTRIBUTE_GROUPS!(fpga_region);

pub unsafe fn __fpga_region_register_full(parent: *mut Device, info: *const FpgaRegionInfo, owner: *mut Module) -> *mut FpgaRegion {
    if info.is_null() {
        dev_err(parent, "Attempt to register without required info structure\n");
        return ERR_PTR(-EINVAL);
    }
    let region = kzalloc_obj::<FpgaRegion>();
    if region.is_null() { return ERR_PTR(-ENOMEM); }
    let id = ida_alloc(&mut FPGA_REGION_IDA, GFP_KERNEL);
    if id < 0 { kfree(region); return ERR_PTR(id); }
    (*region).mgr = (*info).mgr;
    (*region).compat_id = (*info).compat_id;
    (*region).priv_ = (*info).priv_;
    (*region).get_bridges = (*info).get_bridges;
    (*region).ops_owner = owner;
    mutex_init(&mut (*region).mutex);
    INIT_LIST_HEAD(&mut (*region).bridge_list);
    (*region).dev.class = &FPGA_REGION_CLASS;
    (*region).dev.parent = parent;
    (*region).dev.of_node = (*parent).of_node;
    (*region).dev.id = id;
    let ret = dev_set_name(&mut (*region).dev, "region%d", id);
    if ret != 0 { ida_free(&mut FPGA_REGION_IDA, id); kfree(region); return ERR_PTR(ret); }
    let ret = device_register(&mut (*region).dev);
    if ret != 0 { put_device(&mut (*region).dev); return ERR_PTR(ret); }
    region
}

pub unsafe fn __fpga_region_register(parent: *mut Device, mgr: *mut FpgaManager, get_bridges: Option<unsafe extern "C" fn(*mut FpgaRegion) -> i32>, owner: *mut Module) -> *mut FpgaRegion {
    let mut info = FpgaRegionInfo::zeroed();
    info.mgr = mgr;
    info.get_bridges = get_bridges;
    __fpga_region_register_full(parent, &info, owner)
}

pub unsafe fn fpga_region_unregister(region: *mut FpgaRegion) { device_unregister(&mut (*region).dev); }

unsafe extern "C" fn fpga_region_dev_release(dev: *mut Device) {
    let region = to_fpga_region(dev);
    ida_free(&mut FPGA_REGION_IDA, (*region).dev.id);
    kfree(region);
}

static FPGA_REGION_CLASS_DEF: Class = Class { name: "fpga_region", dev_groups: FPGA_REGION_GROUPS, dev_release: Some(fpga_region_dev_release) };

unsafe extern "C" fn fpga_region_init() -> i32 { class_register(&FPGA_REGION_CLASS_DEF) }
unsafe extern "C" fn fpga_region_exit() { class_unregister(&FPGA_REGION_CLASS_DEF); ida_destroy(&mut FPGA_REGION_IDA); }

// subsys_initcall(fpga_region_init); module_exit(fpga_region_exit);
// MODULE_DESCRIPTION("FPGA Region"); MODULE_AUTHOR("Alan Tull <atull@kernel.org>"); MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
