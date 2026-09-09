// SPDX-License-Identifier: GPL-2.0
/*
 * FPGA Region - Device Tree support for FPGA programming under Linux
 *
 *  Copyright (C) 2013-2016 Altera Corporation
 *  Copyright (C) 2017 Intel Corporation
 */

// Kernel dependencies supplied by other translation units.

static fpga_region_of_match: [of_device_id; 2] = [
    of_device_id { compatible: c"fpga-region".as_ptr() },
    of_device_id { ..unsafe { core::mem::zeroed() } },
];

unsafe fn of_fpga_region_find(np: *mut device_node) -> *mut fpga_region {
    fpga_region_class_find(core::ptr::null_mut(), np, device_match_of_node)
}

unsafe fn of_fpga_region_get_mgr(mut np: *mut device_node) -> *mut fpga_manager {
    let mut mgr_node: *mut device_node;
    let mgr: *mut fpga_manager;

    of_node_get(np);
    while !np.is_null() {
        if of_device_is_compatible(np, c"fpga-region".as_ptr()) {
            mgr_node = of_parse_phandle(np, c"fpga-mgr".as_ptr(), 0);
            if !mgr_node.is_null() {
                mgr = of_fpga_mgr_get(mgr_node);
                of_node_put(mgr_node);
                of_node_put(np);
                return mgr;
            }
        }
        np = of_get_next_parent(np);
    }
    of_node_put(np);
    ERR_PTR(-EINVAL)
}

unsafe fn of_fpga_region_get_bridges(region: *mut fpga_region) -> i32 {
    let dev = &mut (*region).dev;
    let region_np = (*dev).of_node;
    let info = (*region).info;
    let mut br: *mut device_node;
    let mut np: *mut device_node;
    let mut parent_br: *mut device_node = core::ptr::null_mut();
    let mut ret: i32;

    ret = of_fpga_bridge_get_to_list((*region_np).parent, info, &mut (*region).bridge_list);
    if ret == -EBUSY { return ret; }
    if ret == 0 { parent_br = (*region_np).parent; }

    br = of_parse_phandle((*info).overlay, c"fpga-bridges".as_ptr(), 0);
    if !br.is_null() {
        of_node_put(br);
        np = (*info).overlay;
    } else { np = region_np; }

    let mut i: i32 = 0;
    loop {
        br = of_parse_phandle(np, c"fpga-bridges".as_ptr(), i);
        if br.is_null() { break; }
        if br == parent_br {
            of_node_put(br);
            i += 1;
            continue;
        }
        ret = of_fpga_bridge_get_to_list(br, info, &mut (*region).bridge_list);
        of_node_put(br);
        if ret != 0 {
            fpga_bridges_put(&mut (*region).bridge_list);
            return ret;
        }
        i += 1;
    }
    0
}

unsafe fn child_regions_with_firmware(mut overlay: *mut device_node) -> i32 {
    let mut child_region: *mut device_node;
    let mut child_firmware_name: *const i8 = core::ptr::null();
    let mut ret: i32 = 0;

    of_node_get(overlay);
    child_region = of_find_matching_node(overlay, fpga_region_of_match.as_ptr());
    while !child_region.is_null() {
        if of_property_read_string(child_region, c"firmware-name".as_ptr(), &mut child_firmware_name) == 0 {
            ret = -EINVAL;
            break;
        }
        child_region = of_find_matching_node(child_region, fpga_region_of_match.as_ptr());
    }
    if ret != 0 { pr_err(c"firmware-name not allowed in child FPGA region: %pOF".as_ptr(), child_region); }
    of_node_put(child_region);
    ret
}

unsafe fn of_fpga_region_parse_ov(region: *mut fpga_region, overlay: *mut device_node) -> *mut fpga_image_info {
    let dev = &mut (*region).dev;
    let info: *mut fpga_image_info;
    let mut firmware_name: *const i8 = core::ptr::null();
    let ret: i32;

    if !(*region).info.is_null() {
        dev_err(dev, c"Region already has overlay applied.\n".as_ptr());
        return ERR_PTR(-EINVAL);
    }
    ret = child_regions_with_firmware(overlay);
    if ret != 0 { return ERR_PTR(ret); }
    info = fpga_image_info_alloc(dev);
    if info.is_null() { return ERR_PTR(-ENOMEM); }
    (*info).overlay = overlay;
    if of_property_read_bool(overlay, c"partial-fpga-config".as_ptr()) { (*info).flags |= FPGA_MGR_PARTIAL_RECONFIG; }
    if of_property_read_bool(overlay, c"external-fpga-config".as_ptr()) { (*info).flags |= FPGA_MGR_EXTERNAL_CONFIG; }
    if of_property_read_bool(overlay, c"encrypted-fpga-config".as_ptr()) { (*info).flags |= FPGA_MGR_ENCRYPTED_BITSTREAM; }
    if of_property_read_string(overlay, c"firmware-name".as_ptr(), &mut firmware_name) == 0 {
        (*info).firmware_name = devm_kstrdup(dev, firmware_name, GFP_KERNEL);
        if (*info).firmware_name.is_null() { return ERR_PTR(-ENOMEM); }
    }
    of_property_read_u32(overlay, c"region-unfreeze-timeout-us".as_ptr(), &mut (*info).enable_timeout_us);
    of_property_read_u32(overlay, c"region-freeze-timeout-us".as_ptr(), &mut (*info).disable_timeout_us);
    of_property_read_u32(overlay, c"config-complete-timeout-us".as_ptr(), &mut (*info).config_complete_timeout_us);
    if (*info).firmware_name.is_null() { fpga_image_info_free(info); return ERR_PTR(0); }
    if (*info).flags & FPGA_MGR_EXTERNAL_CONFIG != 0 {
        dev_err(dev, c"error: specified firmware and external-fpga-config".as_ptr());
        fpga_image_info_free(info);
        return ERR_PTR(-EINVAL);
    }
    info
}

unsafe fn of_fpga_region_notify_pre_apply(region: *mut fpga_region, nd: *mut of_overlay_notify_data) -> i32 {
    let dev = &mut (*region).dev;
    let info = of_fpga_region_parse_ov(region, (*nd).overlay);
    if IS_ERR(info) { return PTR_ERR(info); }
    if info.is_null() { return 0; }
    if !(*region).info.is_null() { dev_err(dev, c"Region already has overlay applied.\n".as_ptr()); return -EINVAL; }
    (*region).info = info;
    let ret = fpga_region_program_fpga(region);
    if ret != 0 { fpga_image_info_free(info); (*region).info = core::ptr::null_mut(); }
    ret
}

unsafe fn of_fpga_region_notify_post_remove(region: *mut fpga_region, _nd: *mut of_overlay_notify_data) {
    fpga_bridges_disable(&mut (*region).bridge_list);
    fpga_bridges_put(&mut (*region).bridge_list);
    fpga_image_info_free((*region).info);
    (*region).info = core::ptr::null_mut();
}

unsafe fn of_fpga_region_notify(nb: *mut notifier_block, action: u64, arg: *mut core::ffi::c_void) -> i32 {
    let nd = arg as *mut of_overlay_notify_data;
    let region: *mut fpga_region;
    let mut ret: i32 = 0;
    match action {
        OF_OVERLAY_PRE_APPLY => pr_debug(c"%s OF_OVERLAY_PRE_APPLY\n".as_ptr(), __func__),
        OF_OVERLAY_POST_APPLY => { pr_debug(c"%s OF_OVERLAY_POST_APPLY\n".as_ptr(), __func__); return NOTIFY_OK; },
        OF_OVERLAY_PRE_REMOVE => { pr_debug(c"%s OF_OVERLAY_PRE_REMOVE\n".as_ptr(), __func__); return NOTIFY_OK; },
        OF_OVERLAY_POST_REMOVE => pr_debug(c"%s OF_OVERLAY_POST_REMOVE\n".as_ptr(), __func__),
        _ => return NOTIFY_OK,
    }
    region = of_fpga_region_find((*nd).target);
    if region.is_null() { return NOTIFY_OK; }
    match action {
        OF_OVERLAY_PRE_APPLY => ret = of_fpga_region_notify_pre_apply(region, nd),
        OF_OVERLAY_POST_REMOVE => of_fpga_region_notify_post_remove(region, nd),
        _ => (),
    }
    put_device(&mut (*region).dev);
    if ret != 0 { return notifier_from_errno(ret); }
    NOTIFY_OK
}

static mut fpga_region_of_nb: notifier_block = notifier_block { notifier_call: Some(of_fpga_region_notify) };

unsafe fn of_fpga_region_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let np = (*dev).of_node;
    let mgr = of_fpga_region_get_mgr(np);
    if IS_ERR(mgr) { return -EPROBE_DEFER; }
    let region = fpga_region_register(dev, mgr, Some(of_fpga_region_get_bridges));
    if IS_ERR(region) { let ret = PTR_ERR(region); fpga_mgr_put(mgr); return ret; }
    of_platform_populate(np, fpga_region_of_match.as_ptr(), core::ptr::null(), &mut (*region).dev);
    platform_set_drvdata(pdev, region as *mut core::ffi::c_void);
    dev_info(dev, c"FPGA Region probed\n".as_ptr());
    0
}

unsafe fn of_fpga_region_remove(pdev: *mut platform_device) {
    let region = platform_get_drvdata(pdev) as *mut fpga_region;
    let mgr = (*region).mgr;
    fpga_region_unregister(region);
    fpga_mgr_put(mgr);
}

static mut of_fpga_region_driver: platform_driver = platform_driver {
    probe: Some(of_fpga_region_probe), remove: Some(of_fpga_region_remove),
    driver: driver { name: c"of-fpga-region".as_ptr(), of_match_table: of_match_ptr(fpga_region_of_match.as_ptr()) },
};

unsafe fn of_fpga_region_init() -> i32 {
    let mut ret = of_overlay_notifier_register(&mut fpga_region_of_nb);
    if ret != 0 { return ret; }
    ret = platform_driver_register(&mut of_fpga_region_driver);
    if ret != 0 { of_overlay_notifier_unregister(&mut fpga_region_of_nb); }
    ret
}

unsafe fn of_fpga_region_exit() {
    platform_driver_unregister(&mut of_fpga_region_driver);
    of_overlay_notifier_unregister(&mut fpga_region_of_nb);
}

// Equivalent registration and module metadata supplied by the kernel build system.
subsys_initcall!(of_fpga_region_init);
module_exit!(of_fpga_region_exit);
module_description!(c"FPGA Region");
module_author!(c"Alan Tull <atull@kernel.org>");
module_license!(c"GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
