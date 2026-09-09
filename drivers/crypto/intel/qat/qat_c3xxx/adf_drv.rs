// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2014 - 2020 Intel Corporation */

// Linux kernel dependencies and build-time configuration are supplied by the
// surrounding Rust kernel environment.

unsafe fn adf_cleanup_pci_dev(accel_dev: *mut adf_accel_dev) {
    pci_release_regions((*accel_dev).accel_pci_dev.pci_dev);
    pci_disable_device((*accel_dev).accel_pci_dev.pci_dev);
}

unsafe fn adf_cleanup_accel(accel_dev: *mut adf_accel_dev) {
    let accel_pci_dev: *mut adf_accel_pci = &mut (*accel_dev).accel_pci_dev;

    for i in 0..ADF_PCI_MAX_BARS {
        let bar: *mut adf_bar = &mut (*accel_pci_dev).pci_bars[i];
        if !(*bar).virt_addr.is_null() {
            pci_iounmap((*accel_pci_dev).pci_dev, (*bar).virt_addr);
        }
    }

    if !(*accel_dev).hw_device.is_null() {
        match (*(*accel_pci_dev).pci_dev).device {
            PCI_DEVICE_ID_INTEL_QAT_C3XXX => {
                adf_clean_hw_data_c3xxx((*accel_dev).hw_device);
            }
            _ => {}
        }
        kfree((*accel_dev).hw_device as *mut core::ffi::c_void);
        (*accel_dev).hw_device = core::ptr::null_mut();
    }
    adf_dbgfs_exit(accel_dev);
    adf_cfg_dev_remove(accel_dev);
    adf_devmgr_rm_dev(accel_dev, core::ptr::null_mut());
}

unsafe fn adf_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> i32 {
    match (*ent).device {
        PCI_DEVICE_ID_INTEL_QAT_C3XXX => {}
        _ => {
            dev_err!(&(*pdev).dev, "Invalid device 0x%x.\n", (*ent).device);
            return -ENODEV;
        }
    }

    if num_possible_nodes() > 1 && dev_to_node(&(*pdev).dev) < 0 {
        /* If the accelerator is connected to a node with no memory
         * there is no point in using the accelerator since the remote
         * memory transaction will be very slow. */
        dev_err!(&(*pdev).dev, "Invalid NUMA configuration.\n");
        return -EINVAL;
    }

    let accel_dev = kzalloc_node(core::mem::size_of::<adf_accel_dev>(), GFP_KERNEL,
                                 dev_to_node(&(*pdev).dev)) as *mut adf_accel_dev;
    if accel_dev.is_null() { return -ENOMEM; }

    INIT_LIST_HEAD(&mut (*accel_dev).crypto_list);
    let accel_pci_dev: *mut adf_accel_pci = &mut (*accel_dev).accel_pci_dev;
    (*accel_pci_dev).pci_dev = pdev;

    /* Add accel device to accel table.
     * This should be called before adf_cleanup_accel is called */
    if adf_devmgr_add_dev(accel_dev, core::ptr::null_mut()) != 0 {
        dev_err!(&(*pdev).dev, "Failed to add new accelerator device.\n");
        kfree(accel_dev as *mut core::ffi::c_void);
        return -EFAULT;
    }

    (*accel_dev).owner = THIS_MODULE;
    /* Allocate and configure device configuration structure */
    let hw_data = kzalloc_node(core::mem::size_of::<adf_hw_device_data>(), GFP_KERNEL,
                               dev_to_node(&(*pdev).dev)) as *mut adf_hw_device_data;
    if hw_data.is_null() {
        let ret = -ENOMEM;
        adf_cleanup_accel(accel_dev);
        kfree(accel_dev as *mut core::ffi::c_void);
        return ret;
    }

    (*accel_dev).hw_device = hw_data;
    adf_init_hw_data_c3xxx((*accel_dev).hw_device);
    pci_read_config_byte(pdev, PCI_REVISION_ID, &mut (*accel_pci_dev).revid);
    pci_read_config_dword(pdev, ADF_DEVICE_FUSECTL_OFFSET,
                          &mut (*hw_data).fuses[ADF_FUSECTL0]);
    pci_read_config_dword(pdev, ADF_C3XXX_SOFTSTRAP_CSR_OFFSET, &mut (*hw_data).straps);

    /* Get Accelerators and Accelerators Engines masks */
    (*hw_data).accel_mask = ((*hw_data).get_accel_mask)(hw_data);
    (*hw_data).ae_mask = ((*hw_data).get_ae_mask)(hw_data);
    (*accel_pci_dev).sku = ((*hw_data).get_sku)(hw_data);
    /* If the device has no acceleration engines then ignore it. */
    if (*hw_data).accel_mask == 0 || (*hw_data).ae_mask == 0 ||
       (((!*hw_data).ae_mask) & 0x01) != 0 {
        dev_err!(&(*pdev).dev, "No acceleration units found");
        let ret = -EFAULT;
        adf_cleanup_accel(accel_dev);
        kfree(accel_dev as *mut core::ffi::c_void);
        return ret;
    }

    /* Create device configuration table */
    let mut ret = adf_cfg_dev_add(accel_dev);
    if ret != 0 { adf_cleanup_accel(accel_dev); kfree(accel_dev as *mut core::ffi::c_void); return ret; }

    /* enable PCI device */
    if pci_enable_device(pdev) != 0 {
        ret = -EFAULT;
        adf_cleanup_accel(accel_dev); kfree(accel_dev as *mut core::ffi::c_void); return ret;
    }

    /* set dma identifier */
    ret = dma_set_mask_and_coherent(&mut (*pdev).dev, DMA_BIT_MASK(48));
    if ret != 0 {
        dev_err!(&(*pdev).dev, "No usable DMA configuration\n");
        pci_disable_device((*accel_pci_dev).pci_dev);
        adf_cleanup_accel(accel_dev); kfree(accel_dev as *mut core::ffi::c_void); return ret;
    }

    if pci_request_regions(pdev, ADF_C3XXX_DEVICE_NAME) != 0 {
        ret = -EFAULT;
        pci_disable_device((*accel_pci_dev).pci_dev);
        adf_cleanup_accel(accel_dev); kfree(accel_dev as *mut core::ffi::c_void); return ret;
    }

    /* Get accelerator capabilities mask */
    (*hw_data).accel_capabilities_mask = ((*hw_data).get_accel_cap)(accel_dev);

    /* Find and map all the device's BARS */
    let mut i = 0usize;
    let mut bar_mask = pci_select_bars(pdev, IORESOURCE_MEM);
    let mut bar_nr = 0usize;
    while bar_nr < ADF_PCI_MAX_BARS * 2 {
        if (bar_mask & (1ul << bar_nr)) != 0 {
            let bar: *mut adf_bar = &mut (*accel_pci_dev).pci_bars[i];
            i += 1;
            (*bar).base_addr = pci_resource_start(pdev, bar_nr);
            if (*bar).base_addr == 0 { break; }
            (*bar).size = pci_resource_len(pdev, bar_nr);
            (*bar).virt_addr = pci_iomap((*accel_pci_dev).pci_dev, bar_nr, 0);
            if (*bar).virt_addr.is_null() {
                pci_err!(pdev, "Failed to map BAR %d\n", bar_nr);
                ret = -EFAULT;
                pci_release_regions((*accel_pci_dev).pci_dev);
                pci_disable_device((*accel_pci_dev).pci_dev);
                adf_cleanup_accel(accel_dev); kfree(accel_dev as *mut core::ffi::c_void); return ret;
            }
        }
        bar_nr += 1;
    }

    if pci_save_state(pdev) != 0 {
        pci_err!(pdev, "Failed to save pci state\n");
        ret = -ENOMEM;
        pci_release_regions((*accel_pci_dev).pci_dev);
        pci_disable_device((*accel_pci_dev).pci_dev);
        adf_cleanup_accel(accel_dev); kfree(accel_dev as *mut core::ffi::c_void); return ret;
    }

    adf_dbgfs_init(accel_dev);
    ret = adf_dev_up(accel_dev, true);
    if ret != 0 {
        adf_dev_down(accel_dev);
        pci_release_regions((*accel_pci_dev).pci_dev);
        pci_disable_device((*accel_pci_dev).pci_dev);
        adf_cleanup_accel(accel_dev); kfree(accel_dev as *mut core::ffi::c_void); return ret;
    }
    ret
}

unsafe fn adf_remove(pdev: *mut pci_dev) {
    let accel_dev = adf_devmgr_pci_to_accel_dev(pdev);
    if accel_dev.is_null() { pr_err!("QAT: Driver removal failed\n"); return; }
    adf_dev_down(accel_dev);
    adf_cleanup_accel(accel_dev);
    adf_cleanup_pci_dev(accel_dev);
    kfree(accel_dev as *mut core::ffi::c_void);
}

unsafe fn adf_shutdown(pdev: *mut pci_dev) {
    let accel_dev = adf_devmgr_pci_to_accel_dev(pdev);
    adf_dev_down(accel_dev);
}

static ADF_PCI_TBL: [pci_device_id; 2] = [
    pci_vdevice!(INTEL, PCI_DEVICE_ID_INTEL_QAT_C3XXX),
    pci_device_id::default(),
];

static mut ADF_DRIVER: pci_driver = pci_driver {
    id_table: ADF_PCI_TBL.as_ptr(),
    name: ADF_C3XXX_DEVICE_NAME,
    probe: Some(adf_probe),
    remove: Some(adf_remove),
    shutdown: Some(adf_shutdown),
    sriov_configure: Some(adf_sriov_configure),
    err_handler: &adf_err_handler,
};

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
}

// module_init(adfdrv_init);
// module_exit(adfdrv_release);
// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_AUTHOR("Intel");
// MODULE_FIRMWARE(ADF_C3XXX_FW);
// MODULE_FIRMWARE(ADF_C3XXX_MMP);
// MODULE_DESCRIPTION("Intel(R) QuickAssist Technology");
// MODULE_IMPORT_NS("CRYPTO_QAT");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
