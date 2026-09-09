// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2014 - 2020 Intel Corporation */

// Linux kernel headers and local headers from the C translation unit are
// expected to provide the declarations referenced below.

static ADF_PCI_TBL: [pci_device_id; 2] = [
    pci_device_id { vendor: PCI_VENDOR_ID_INTEL, device: PCI_DEVICE_ID_INTEL_QAT_C62X_VF },
    pci_device_id { vendor: 0, device: 0 },
];

static mut ADF_DRIVER: pci_driver = pci_driver {
    id_table: ADF_PCI_TBL.as_ptr(),
    name: ADF_C62XVF_DEVICE_NAME,
    probe: Some(adf_probe),
    remove: Some(adf_remove),
};

unsafe fn adf_cleanup_pci_dev(accel_dev: *mut adf_accel_dev) {
    pci_release_regions((*(*accel_dev).accel_pci_dev.pci_dev));
    pci_disable_device((*accel_dev).accel_pci_dev.pci_dev);
}

unsafe fn adf_cleanup_accel(accel_dev: *mut adf_accel_dev) {
    let accel_pci_dev = &mut (*accel_dev).accel_pci_dev;
    let mut pf: *mut adf_accel_dev;

    for i in 0..ADF_PCI_MAX_BARS {
        let bar = &mut accel_pci_dev.pci_bars[i as usize];
        if !bar.virt_addr.is_null() {
            pci_iounmap(accel_pci_dev.pci_dev, bar.virt_addr);
        }
    }

    if !(*accel_dev).hw_device.is_null() {
        match (*accel_pci_dev.pci_dev).device {
            PCI_DEVICE_ID_INTEL_QAT_C62X_VF => {
                adf_clean_hw_data_c62xiov((*accel_dev).hw_device);
            }
            _ => {}
        }
        kfree((*accel_dev).hw_device as *mut core::ffi::c_void);
        (*accel_dev).hw_device = core::ptr::null_mut();
    }
    adf_dbgfs_exit(accel_dev);
    adf_cfg_dev_remove(accel_dev);
    pf = adf_devmgr_pci_to_accel_dev((*accel_pci_dev.pci_dev).physfn);
    adf_devmgr_rm_dev(accel_dev, pf);
}

unsafe fn adf_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> i32 {
    let mut accel_dev: *mut adf_accel_dev;
    let pf: *mut adf_accel_dev;
    let accel_pci_dev: *mut adf_accel_pci;
    let hw_data: *mut adf_hw_device_data;
    let mut i: u32;
    let mut bar_nr: u32;
    let mut bar_mask: c_ulong;
    let mut ret: i32;

    match (*ent).device {
        PCI_DEVICE_ID_INTEL_QAT_C62X_VF => {}
        _ => {
            dev_err(&mut (*pdev).dev, "Invalid device 0x%x.\n", (*ent).device);
            return -ENODEV;
        }
    }

    accel_dev = kzalloc_node(core::mem::size_of::<adf_accel_dev>(), GFP_KERNEL,
                             dev_to_node(&mut (*pdev).dev)) as *mut adf_accel_dev;
    if accel_dev.is_null() { return -ENOMEM; }

    (*accel_dev).is_vf = true;
    pf = adf_devmgr_pci_to_accel_dev((*pdev).physfn);
    accel_pci_dev = &mut (*accel_dev).accel_pci_dev;
    (*accel_pci_dev).pci_dev = pdev;

    if adf_devmgr_add_dev(accel_dev, pf) != 0 {
        dev_err(&mut (*pdev).dev, "Failed to add new accelerator device.\n");
        kfree(accel_dev as *mut core::ffi::c_void);
        return -EFAULT;
    }
    INIT_LIST_HEAD(&mut (*accel_dev).crypto_list);
    (*accel_dev).owner = THIS_MODULE;

    hw_data = kzalloc_node(core::mem::size_of::<adf_hw_device_data>(), GFP_KERNEL,
                           dev_to_node(&mut (*pdev).dev)) as *mut adf_hw_device_data;
    if hw_data.is_null() { ret = -ENOMEM; goto out_err; }
    (*accel_dev).hw_device = hw_data;
    adf_init_hw_data_c62xiov((*accel_dev).hw_device);
    (*hw_data).accel_mask = ((*hw_data).get_accel_mask)(hw_data);
    (*hw_data).ae_mask = ((*hw_data).get_ae_mask)(hw_data);
    (*accel_pci_dev).sku = ((*hw_data).get_sku)(hw_data);

    ret = adf_cfg_dev_add(accel_dev);
    if ret != 0 { goto out_err; }
    if pci_enable_device(pdev) != 0 { ret = -EFAULT; goto out_err; }
    ret = dma_set_mask_and_coherent(&mut (*pdev).dev, DMA_BIT_MASK(48));
    if ret != 0 {
        dev_err(&mut (*pdev).dev, "No usable DMA configuration\n");
        goto out_err_disable;
    }
    if pci_request_regions(pdev, ADF_C62XVF_DEVICE_NAME) != 0 { ret = -EFAULT; goto out_err_disable; }

    i = 0;
    bar_mask = pci_select_bars(pdev, IORESOURCE_MEM);
    for_each_set_bit!(bar_nr, &bar_mask, ADF_PCI_MAX_BARS * 2) {
        let bar = &mut (*accel_pci_dev).pci_bars[i as usize];
        i += 1;
        bar.base_addr = pci_resource_start(pdev, bar_nr);
        if bar.base_addr == 0 { break; }
        bar.size = pci_resource_len(pdev, bar_nr);
        bar.virt_addr = pci_iomap((*accel_pci_dev).pci_dev, bar_nr, 0);
        if bar.virt_addr.is_null() {
            pci_err(pdev, "Failed to map BAR %d\n", bar_nr);
            ret = -EFAULT;
            goto out_err_free_reg;
        }
    }
    init_completion(&mut (*accel_dev).vf.msg_received);
    adf_dbgfs_init(accel_dev);
    ret = adf_dev_up(accel_dev, false);
    if ret != 0 { goto out_err_dev_stop; }
    return ret;

out_err_dev_stop:
    adf_dev_down(accel_dev);
out_err_free_reg:
    pci_release_regions((*accel_pci_dev).pci_dev);
out_err_disable:
    pci_disable_device((*accel_pci_dev).pci_dev);
out_err:
    adf_cleanup_accel(accel_dev);
    kfree(accel_dev as *mut core::ffi::c_void);
    return ret;
}

unsafe fn adf_remove(pdev: *mut pci_dev) {
    let accel_dev = adf_devmgr_pci_to_accel_dev(pdev);
    if accel_dev.is_null() { pr_err!("QAT: Driver removal failed\n"); return; }
    adf_flush_vf_wq(accel_dev);
    adf_dev_down(accel_dev);
    adf_cleanup_accel(accel_dev);
    adf_cleanup_pci_dev(accel_dev);
    kfree(accel_dev as *mut core::ffi::c_void);
}

unsafe fn adfdrv_init() -> i32 {
    request_module!("intel_qat");
    if pci_register_driver(&mut ADF_DRIVER) != 0 {
        pr_err!("QAT: Driver initialization failed\n");
        return -EFAULT;
    }
    0
}

unsafe fn adfdrv_release() {
    pci_unregister_driver(&mut ADF_DRIVER);
    adf_clean_vf_map(true);
}

// module_init(adfdrv_init);
// module_exit(adfdrv_release);
// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_AUTHOR("Intel");
// MODULE_DESCRIPTION("Intel(R) QuickAssist Technology");
// MODULE_IMPORT_NS("CRYPTO_QAT");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
