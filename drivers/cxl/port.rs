// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2022 Intel Corporation. All rights reserved. */
// Dependencies are supplied by the surrounding CXL implementation.

unsafe fn schedule_detach(cxlmd: *mut core::ffi::c_void) {
    schedule_cxl_memdev_detach(cxlmd);
}

unsafe fn discover_region(dev: *mut device, _unused: *mut core::ffi::c_void) -> i32 {
    if !is_endpoint_decoder(dev) { return 0; }
    let cxled = to_cxl_endpoint_decoder(dev);
    if ((*cxled).cxld.flags & CXL_DECODER_F_ENABLE) == 0 { return 0; }
    if (*cxled).state != CXL_DECODER_STATE_AUTO { return 0; }

    // Region enumeration is opportunistic; continue if this add-event fails.
    let rc = cxl_add_to_region(cxled);
    if rc != 0 {
        dev_dbg(dev, "failed to add to region: %#llx-%#llx\n",
                (*cxled).cxld.hpa_range.start, (*cxled).cxld.hpa_range.end);
    }
    0
}

unsafe fn cxl_switch_port_probe(port: *mut cxl_port) -> i32 {
    (*port).nr_dports = 0;
    read_cdat_data(port);
    0
}

unsafe fn cxl_ras_unmask(port: *mut cxl_port) -> i32 {
    if !dev_is_pci((*port).uport_dev) { return 0; }
    let pdev = to_pci_dev((*port).uport_dev);
    if (*port).regs.ras.is_null() { pci_dbg(pdev, "No RAS registers.\n"); return 0; }
    if !pcie_aer_is_native(pdev) { return 0; }

    let mut cap: u16 = 0;
    let rc = pcie_capability_read_word(pdev, PCI_EXP_DEVCTL, &mut cap);
    if rc != 0 { return rc; }
    let mut addr: *mut core::ffi::c_void;
    let mut orig_val: u32;
    let mut val: u32;
    if (cap & PCI_EXP_DEVCTL_URRE) != 0 {
        addr = (*port).regs.ras.add(CXL_RAS_UNCORRECTABLE_MASK_OFFSET as usize) as *mut _;
        orig_val = readl(addr);
        let mask = CXL_RAS_UNCORRECTABLE_MASK_MASK | CXL_RAS_UNCORRECTABLE_MASK_F256B_MASK;
        val = orig_val & !mask;
        writel(val, addr);
        pci_dbg(pdev, "Uncorrectable RAS Errors Mask: %#x -> %#x\n", orig_val, val);
    }
    if (cap & PCI_EXP_DEVCTL_CERE) != 0 {
        addr = (*port).regs.ras.add(CXL_RAS_CORRECTABLE_MASK_OFFSET as usize) as *mut _;
        orig_val = readl(addr);
        val = orig_val & !CXL_RAS_CORRECTABLE_MASK_MASK;
        writel(val, addr);
        pci_dbg(pdev, "Correctable RAS Errors Mask: %#x -> %#x\n", orig_val, val);
    }
    0
}

unsafe fn cxl_endpoint_port_probe(port: *mut cxl_port) -> i32 {
    let cxlmd = to_cxl_memdev((*port).uport_dev);
    let dport = (*port).parent_dport;
    read_cdat_data(port);
    cxl_endpoint_parse_cdat(port);
    get_device(&mut (*cxlmd).dev);
    let rc = devm_add_action_or_reset(&mut (*port).dev, schedule_detach, cxlmd);
    if rc != 0 { return rc; }
    let rc = devm_cxl_endpoint_decoders_setup(port);
    if rc != 0 { return rc; }
    if (*dport).rch { devm_cxl_dport_rch_ras_setup(dport); }
    devm_cxl_port_ras_setup(port);
    if cxl_ras_unmask(port) != 0 { dev_dbg(&mut (*port).dev, "failed to unmask RAS interrupts\n"); }
    device_for_each_child(&mut (*port).dev, core::ptr::null_mut(), discover_region);
    0
}

unsafe fn cxl_port_probe(dev: *mut device) -> i32 {
    let port = to_cxl_port(dev);
    if is_cxl_endpoint(port) { cxl_endpoint_port_probe(port) } else { cxl_switch_port_probe(port) }
}

unsafe fn CDAT_read(_filp: *mut file, kobj: *mut kobject, _bin_attr: *const bin_attribute,
                    buf: *mut i8, offset: loff_t, count: usize) -> isize {
    let port = to_cxl_port(kobj_to_dev(kobj));
    if !(*port).cdat_available { return -ENXIO as isize; }
    if (*port).cdat.table.is_null() { return 0; }
    memory_read_from_buffer(buf, count, &offset, (*port).cdat.table, (*port).cdat.length)
}

// const BIN_ATTR_ADMIN_RO(CDAT, 0);
unsafe fn cxl_port_bin_attr_is_visible(kobj: *mut kobject, attr: *const bin_attribute, _i: i32) -> umode_t {
    let port = to_cxl_port(kobj_to_dev(kobj));
    if attr == &bin_attr_CDAT && (*port).cdat_available { return (*attr).attr.mode; }
    0
}

static mut cxl_cdat_bin_attributes: [*const bin_attribute; 2] = [
    &bin_attr_CDAT, core::ptr::null(),
];
static mut cxl_cdat_attribute_group: attribute_group = attribute_group {
    bin_attrs: cxl_cdat_bin_attributes.as_ptr(),
    is_bin_visible: Some(cxl_port_bin_attr_is_visible),
};
static mut cxl_port_attribute_groups: [*const attribute_group; 2] = [
    &cxl_cdat_attribute_group, core::ptr::null(),
];

// DEFINE_FREE(cxl_port_release_dr_group, struct cxl_port *,
//             if (_T) devres_release_group(&_T->dev, _T));

unsafe fn cxl_port_add_dport(port: *mut cxl_port, dport_dev: *mut device) -> *mut cxl_dport {
    let port_dr_group = devres_open_group(&mut (*port).dev, port, GFP_KERNEL);
    if port_dr_group.is_null() { return ERR_PTR(-ENOMEM); }
    if (*port).nr_dports == 0 {
        let rc = cxl_port_setup_regs(port, (*port).component_reg_phys);
        if rc != 0 { devres_release_group(&mut (*port).dev, port); return ERR_PTR(rc); }
        let rc = devm_cxl_switch_port_decoders_setup(port);
        if rc != 0 { devres_release_group(&mut (*port).dev, port); return ERR_PTR(rc); }
        devm_cxl_port_ras_setup(port);
    }
    let dport = devm_cxl_add_dport_by_dev(port, dport_dev);
    if IS_ERR(dport) { devres_release_group(&mut (*port).dev, port); return dport; }
    devres_remove_group(&mut (*port).dev, port_dr_group);
    cxl_switch_parse_cdat(dport);
    cxl_port_update_decoder_targets(port, dport);
    dev_dbg(&mut (*port).dev, "dport%d:%s added\n", (*dport).port_id, dev_name(dport_dev));
    dport
}

static mut cxl_port_driver: cxl_driver = cxl_driver {
    name: "cxl_port", probe: Some(cxl_port_probe), add_dport: Some(cxl_port_add_dport),
    id: CXL_DEVICE_PORT, drv: device_driver { probe_type: PROBE_FORCE_SYNCHRONOUS, dev_groups: cxl_port_attribute_groups.as_ptr() },
};

unsafe fn devm_cxl_add_endpoint(host: *mut device, cxlmd: *mut cxl_memdev,
                                parent_dport: *mut cxl_dport) -> i32 {
    let parent_port = (*parent_dport).port;
    let mut iter = parent_port;
    let mut down: *mut cxl_port = core::ptr::null_mut();
    while !is_cxl_root(iter) {
        let ep = cxl_ep_load(iter, cxlmd);
        (*ep).next = down;
        down = iter;
        iter = to_cxl_port((*iter).dev.parent);
    }
    let endpoint = devm_cxl_add_port(host, &mut (*cxlmd).dev, CXL_RESOURCE_NONE, parent_dport);
    if IS_ERR(endpoint) { return PTR_ERR(endpoint); }
    let rc = cxl_endpoint_autoremove(cxlmd, endpoint);
    if rc != 0 { return rc; }
    if (*endpoint).dev.driver.is_null() { dev_err(&mut (*cxlmd).dev, "%s failed probe\n", dev_name(&mut (*endpoint).dev)); return -ENXIO; }
    0
}

// EXPORT_SYMBOL_FOR_MODULES(devm_cxl_add_endpoint, "cxl_mem");
unsafe fn cxl_port_init() -> i32 { cxl_driver_register(&mut cxl_port_driver) }
// subsys_initcall(cxl_port_init);
unsafe fn cxl_port_exit() { cxl_driver_unregister(&mut cxl_port_driver); }
// module_exit(cxl_port_exit);
// MODULE_DESCRIPTION("CXL: Port enumeration and services");
// MODULE_LICENSE("GPL v2");
// MODULE_IMPORT_NS("CXL");
// MODULE_ALIAS_CXL(CXL_DEVICE_PORT);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
