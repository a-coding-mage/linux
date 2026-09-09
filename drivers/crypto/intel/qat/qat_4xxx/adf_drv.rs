// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2020 Intel Corporation */

// Dependencies supplied by the Linux kernel and the surrounding driver.

static ADF_PCI_TBL: [pci_device_id; 4] = [
    PCI_VDEVICE(INTEL, PCI_DEVICE_ID_INTEL_QAT_4XXX),
    PCI_VDEVICE(INTEL, PCI_DEVICE_ID_INTEL_QAT_401XX),
    PCI_VDEVICE(INTEL, PCI_DEVICE_ID_INTEL_QAT_402XX),
    pci_device_id { ..unsafe { core::mem::zeroed() } },
];

// MODULE_DEVICE_TABLE(pci, adf_pci_tbl);

unsafe fn adf_cleanup_accel(accel_dev: *mut adf_accel_dev) {
    if !(*accel_dev).hw_device.is_null() {
        adf_clean_hw_data_4xxx((*accel_dev).hw_device);
        (*accel_dev).hw_device = core::ptr::null_mut();
    }
    adf_dbgfs_exit(accel_dev);
    adf_cfg_dev_remove(accel_dev);
    adf_devmgr_rm_dev(accel_dev, core::ptr::null_mut());
}

unsafe fn adf_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int {
    let mut accel_dev: *mut adf_accel_dev;
    let accel_pci_dev: *mut adf_accel_pci;
    let hw_data: *mut adf_hw_device_data;
    let mut i: c_uint;
    let mut bar_nr: c_uint;
    let mut bar_mask: c_ulong;
    let bar: *mut adf_bar;
    let mut ret: c_int;

    if num_possible_nodes() > 1 && dev_to_node(&mut (*pdev).dev) < 0 {
        /*
         * If the accelerator is connected to a node with no memory
         * there is no point in using the accelerator since the remote
         * memory transaction will be very slow.
         */
        dev_err(&mut (*pdev).dev, "Invalid NUMA configuration.\n");
        return -EINVAL;
    }

    accel_dev = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<adf_accel_dev>(), GFP_KERNEL)
        as *mut adf_accel_dev;
    if accel_dev.is_null() {
        return -ENOMEM;
    }

    INIT_LIST_HEAD(&mut (*accel_dev).crypto_list);
    accel_pci_dev = &mut (*accel_dev).accel_pci_dev;
    (*accel_pci_dev).pci_dev = pdev;

    /*
     * Add accel device to accel table
     * This should be called before adf_cleanup_accel is called
     */
    if adf_devmgr_add_dev(accel_dev, core::ptr::null_mut()) != 0 {
        dev_err(&mut (*pdev).dev, "Failed to add new accelerator device.\n");
        return -EFAULT;
    }

    (*accel_dev).owner = THIS_MODULE;
    /* Allocate and initialise device hardware meta-data structure */
    hw_data = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<adf_hw_device_data>(), GFP_KERNEL)
        as *mut adf_hw_device_data;
    if hw_data.is_null() {
        ret = -ENOMEM;
        goto out_err;
    }

    (*accel_dev).hw_device = hw_data;
    adf_init_hw_data_4xxx((*accel_dev).hw_device, (*ent).device);

    pci_read_config_byte(pdev, PCI_REVISION_ID, &mut (*accel_pci_dev).revid);
    pci_read_config_dword(pdev, ADF_GEN4_FUSECTL4_OFFSET, &mut (*hw_data).fuses[ADF_FUSECTL4]);

    /* Get Accelerators and Accelerators Engines masks */
    (*hw_data).accel_mask = ((*hw_data).get_accel_mask)(hw_data);
    (*hw_data).ae_mask = ((*hw_data).get_ae_mask)(hw_data);
    (*accel_pci_dev).sku = ((*hw_data).get_sku)(hw_data);
    /* If the device has no acceleration engines then ignore it */
    if (*hw_data).accel_mask == 0 || (*hw_data).ae_mask == 0 || ((!(*hw_data).ae_mask) & 0x01) != 0 {
        dev_err(&mut (*pdev).dev, "No acceleration units found.\n");
        ret = -EFAULT;
        goto out_err;
    }

    /* Create device configuration table */
    ret = adf_cfg_dev_add(accel_dev);
    if ret != 0 { goto out_err; }

    /* Enable PCI device */
    ret = pcim_enable_device(pdev);
    if ret != 0 {
        pci_err(pdev, "Can't enable PCI device.\n");
        goto out_err;
    }

    /* Set DMA identifier */
    ret = dma_set_mask_and_coherent(&mut (*pdev).dev, DMA_BIT_MASK(64));
    if ret != 0 {
        dev_err(&mut (*pdev).dev, "No usable DMA configuration.\n");
        goto out_err;
    }

    ret = adf_gen4_cfg_dev_init(accel_dev);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, "Failed to initialize configuration.\n");
        goto out_err;
    }

    /* Get accelerator capabilities mask */
    (*hw_data).accel_capabilities_mask = ((*hw_data).get_accel_cap)(accel_dev);
    if (*hw_data).accel_capabilities_mask == 0 {
        dev_err(&mut (*pdev).dev, "Failed to get capabilities mask.\n");
        ret = -EINVAL;
        goto out_err;
    }

    /* Find and map all the device's BARS */
    bar_mask = pci_select_bars(pdev, IORESOURCE_MEM) & ADF_GEN4_BAR_MASK;

    ret = pcim_request_all_regions(pdev, pci_name(pdev));
    if ret != 0 {
        pci_err(pdev, "Failed to request PCI regions.\n");
        goto out_err;
    }

    i = 0;
    // for_each_set_bit(bar_nr, &bar_mask, PCI_STD_NUM_BARS)
    for bar_nr in 0..PCI_STD_NUM_BARS {
        if (bar_mask & (1 as c_ulong).wrapping_shl(bar_nr)) == 0 { continue; }
        bar = &mut (*accel_pci_dev).pci_bars[i as usize];
        i += 1;
        (*bar).virt_addr = pcim_iomap(pdev, bar_nr, 0);
        if (*bar).virt_addr.is_null() {
            pci_err(pdev, "Failed to ioremap PCI region.\n");
            ret = -ENOMEM;
            goto out_err;
        }
    }

    if pci_save_state(pdev) != 0 {
        pci_err(pdev, "Failed to save pci state.\n");
        ret = -ENOMEM;
        goto out_err;
    }

    (*accel_dev).ras_errors.enabled = true;
    adf_dbgfs_init(accel_dev);

    ret = adf_dev_up(accel_dev, true);
    if ret != 0 { goto out_err_dev_stop; }

    ret = adf_sysfs_init(accel_dev);
    if ret != 0 { goto out_err_dev_stop; }

    return ret;

out_err_dev_stop:
    adf_dev_down(accel_dev);
out_err:
    adf_cleanup_accel(accel_dev);
    return ret;
}

unsafe fn adf_remove(pdev: *mut pci_dev) {
    let accel_dev = adf_devmgr_pci_to_accel_dev(pdev);
    if accel_dev.is_null() {
        pr_err("QAT: Driver removal failed\n");
        return;
    }
    adf_dev_down(accel_dev);
    adf_cleanup_accel(accel_dev);
}

unsafe fn adf_shutdown(pdev: *mut pci_dev) {
    let accel_dev = adf_devmgr_pci_to_accel_dev(pdev);
    adf_dev_down(accel_dev);
}

static mut ADF_DRIVER: pci_driver = pci_driver {
    id_table: ADF_PCI_TBL.as_ptr(),
    name: ADF_4XXX_DEVICE_NAME,
    probe: Some(adf_probe),
    remove: Some(adf_remove),
    shutdown: Some(adf_shutdown),
    sriov_configure: Some(adf_sriov_configure),
    err_handler: &adf_err_handler,
};

// module_pci_driver(adf_driver);
// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_AUTHOR("Intel");
// MODULE_FIRMWARE(ADF_4XXX_FW);
// MODULE_FIRMWARE(ADF_402XX_FW);
// MODULE_FIRMWARE(ADF_4XXX_MMP);
// MODULE_FIRMWARE(ADF_402XX_MMP);
// MODULE_DESCRIPTION("Intel(R) QuickAssist Technology");
// MODULE_SOFTDEP("pre: crypto-intel_qat");
// MODULE_IMPORT_NS("CRYPTO_QAT");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
