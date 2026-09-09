// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2022 Intel Corporation. All rights reserved. */
// Dependencies supplied by cxlmem.h, cxlpci.h, and Linux kernel headers.

unsafe fn enable_suspend(_data: *mut core::ffi::c_void) {
    cxl_mem_active_dec();
}

unsafe fn remove_debugfs(dentry: *mut core::ffi::c_void) {
    debugfs_remove_recursive(dentry);
}

unsafe fn cxl_mem_dpa_show(file: *mut seq_file, _data: *mut core::ffi::c_void) -> i32 {
    let dev = (*file).private;
    let cxlmd = to_cxl_memdev(dev);
    cxl_dpa_debug(file, (*cxlmd).cxlds);
    0
}

unsafe fn cxl_debugfs_poison_inject(data: *mut core::ffi::c_void, dpa: u64) -> i32 {
    let cxlmd = data as *mut cxl_memdev;
    let rc = acquire_device_intr(&mut (*cxlmd).dev);
    if rc != 0 {
        return rc;
    }
    cxl_inject_poison(cxlmd, dpa)
}

// DEFINE_DEBUGFS_ATTRIBUTE(cxl_poison_inject_fops, NULL,
//                          cxl_debugfs_poison_inject, "%llx\n");
static mut cxl_poison_inject_fops: *mut file_operations = core::ptr::null_mut();

unsafe fn cxl_debugfs_poison_clear(data: *mut core::ffi::c_void, dpa: u64) -> i32 {
    let cxlmd = data as *mut cxl_memdev;
    let rc = acquire_device_intr(&mut (*cxlmd).dev);
    if rc != 0 {
        return rc;
    }
    cxl_clear_poison(cxlmd, dpa)
}

// DEFINE_DEBUGFS_ATTRIBUTE(cxl_poison_clear_fops, NULL,
//                          cxl_debugfs_poison_clear, "%llx\n");
static mut cxl_poison_clear_fops: *mut file_operations = core::ptr::null_mut();

unsafe fn cxl_memdev_poison_enable(
    mds: *mut cxl_memdev_state,
    cxlmd: *mut cxl_memdev,
    dentry: *mut dentry,
) {
    // Avoid poison debugfs for DEVMEM aka accelerators as they rely on
    // cxl_memdev_state.
    if mds.is_null() {
        return;
    }

    if test_bit(CXL_POISON_ENABLED_INJECT, (*mds).poison.enabled_cmds) {
        debugfs_create_file(
            b"inject_poison\0".as_ptr() as *const i8,
            0o200,
            dentry,
            cxlmd as *mut core::ffi::c_void,
            &mut cxl_poison_inject_fops,
        );
    }

    if test_bit(CXL_POISON_ENABLED_CLEAR, (*mds).poison.enabled_cmds) {
        debugfs_create_file(
            b"clear_poison\0".as_ptr() as *const i8,
            0o200,
            dentry,
            cxlmd as *mut core::ffi::c_void,
            &mut cxl_poison_clear_fops,
        );
    }
}

unsafe fn cxl_mem_probe(dev: *mut device) -> i32 {
    let cxlmd = to_cxl_memdev(dev);
    let mds = to_cxl_memdev_state((*cxlmd).cxlds);
    let cxlds = (*cxlmd).cxlds;
    let mut endpoint_parent: *mut device;
    let mut dport: *mut cxl_dport = core::ptr::null_mut();
    let dentry: *mut dentry;
    let mut rc: i32;

    if !(*cxlds).media_ready {
        return -EBUSY;
    }
    if work_pending(&mut (*cxlmd).detach_work) {
        return -EBUSY;
    }

    dentry = cxl_debugfs_create_dir(dev_name(dev));
    debugfs_create_devm_seqfile(dev, b"dpamem\0".as_ptr() as *const i8, dentry, cxl_mem_dpa_show);
    cxl_memdev_poison_enable(mds, cxlmd, dentry);

    rc = devm_add_action_or_reset(dev, remove_debugfs, dentry as *mut core::ffi::c_void);
    if rc != 0 { return rc; }
    rc = devm_cxl_enumerate_ports(cxlmd);
    if rc != 0 { return rc; }

    let parent_port = cxl_mem_find_port(cxlmd, &mut dport);
    if parent_port.is_null() {
        dev_err(dev, b"CXL port topology not found\n\0".as_ptr() as *const i8);
        return -ENXIO;
    }

    if cxl_pmem_size(cxlds) != 0 && IS_ENABLED_CONFIG_CXL_PMEM {
        rc = devm_cxl_add_nvdimm(dev, parent_port, cxlmd);
        if rc != 0 {
            if rc == -ENODEV { dev_info(dev, b"PMEM disabled by platform\n\0".as_ptr() as *const i8); }
            return rc;
        }
    }

    if (*dport).rch {
        endpoint_parent = (*parent_port).uport_dev;
    } else {
        endpoint_parent = &mut (*parent_port).dev;
    }

    let _guard = device_guard(endpoint_parent);
    if (*endpoint_parent).driver.is_null() {
        dev_err(dev, b"CXL port topology %s not enabled\n\0".as_ptr() as *const i8, dev_name(endpoint_parent));
        return -ENXIO;
    }
    rc = devm_cxl_add_endpoint(endpoint_parent, cxlmd, dport);
    if rc != 0 { return rc; }

    if !(*cxlmd).attach.is_null() {
        rc = ((*(*cxlmd).attach).probe)(cxlmd);
        if rc != 0 { return rc; }
    }
    rc = devm_cxl_memdev_edac_register(cxlmd);
    if rc != 0 { dev_dbg(dev, b"CXL memdev EDAC registration failed rc=%d\n\0".as_ptr() as *const i8, rc); }
    cxl_mem_active_inc();
    devm_add_action_or_reset(dev, enable_suspend, core::ptr::null_mut())
}

pub unsafe fn devm_cxl_add_classdev(cxlds: *mut cxl_dev_state) -> *mut cxl_memdev {
    __devm_cxl_add_memdev(cxlds, core::ptr::null_mut())
}

pub unsafe fn devm_cxl_probe_mem(cxlds: *mut cxl_dev_state, hpa_range: *mut range) -> *mut cxl_memdev {
    let attach = devm_kmalloc((*cxlds).dev, core::mem::size_of::<cxl_attach_region>(), GFP_KERNEL) as *mut cxl_attach_region;
    if attach.is_null() { return ERR_PTR(-ENOMEM); }
    (*attach).attach.probe = Some(cxl_memdev_attach_region);
    (*attach).hpa_range = range { start: 0, end: u64::MAX };
    let cxlmd = __devm_cxl_add_memdev(cxlds, &mut (*attach).attach);
    *hpa_range = (*attach).hpa_range;
    cxlmd
}

unsafe fn trigger_poison_list_store(dev: *mut device, _attr: *mut device_attribute, buf: *const i8, len: usize) -> isize {
    let mut trigger = false;
    if kstrtobool(buf, &mut trigger) != 0 || !trigger { return -EINVAL as isize; }
    let rc = cxl_trigger_poison_list(to_cxl_memdev(dev));
    if rc != 0 { rc as isize } else { len as isize }
}

// DEVICE_ATTR_WO(trigger_poison_list);
static mut dev_attr_trigger_poison_list: attribute = attribute { mode: 0 };

unsafe fn cxl_poison_attr_visible(kobj: *mut kobject, _a: *mut attribute) -> bool {
    let dev = kobject_to_dev(kobj);
    let cxlmd = to_cxl_memdev(dev);
    let mds = to_cxl_memdev_state((*cxlmd).cxlds);
    !mds.is_null() && test_bit(CXL_POISON_ENABLED_LIST, (*mds).poison.enabled_cmds)
}

unsafe fn cxl_mem_visible(kobj: *mut kobject, a: *mut attribute, _n: i32) -> umode_t {
    if a == &mut dev_attr_trigger_poison_list && !cxl_poison_attr_visible(kobj, a) { return 0; }
    (*a).mode
}

static mut cxl_mem_attrs: [*mut attribute; 2] = [&mut dev_attr_trigger_poison_list, core::ptr::null_mut()];
static mut cxl_mem_group: attribute_group = attribute_group { attrs: cxl_mem_attrs.as_mut_ptr(), is_visible: Some(cxl_mem_visible) };
static mut cxl_mem_groups: [*mut attribute_group; 2] = [&mut cxl_mem_group, core::ptr::null_mut()];

static mut cxl_mem_driver: cxl_driver = cxl_driver {
    name: b"cxl_mem\0".as_ptr() as *const i8,
    probe: Some(cxl_mem_probe),
    id: CXL_DEVICE_MEMORY_EXPANDER,
    drv: driver { probe_type: PROBE_FORCE_SYNCHRONOUS, dev_groups: cxl_mem_groups.as_ptr() },
};

// module_cxl_driver(cxl_mem_driver);
// MODULE_DESCRIPTION("CXL: Memory Expansion");
// MODULE_LICENSE("GPL v2");
// MODULE_IMPORT_NS("CXL");
// MODULE_ALIAS_CXL(CXL_DEVICE_MEMORY_EXPANDER);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
