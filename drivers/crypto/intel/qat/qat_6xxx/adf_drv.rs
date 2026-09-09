// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2025 Intel Corporation */

// Linux and QAT declarations are supplied by the surrounding kernel/Rust
// environment.  C preprocessor conditionals and module-registration macros
// are represented by the corresponding Rust declarations and metadata below.

static mut BAR_MAP: [i32; 3] = [
    0, // SRAM
    2, // PMISC
    4, // ETR
];

unsafe fn adf_device_down(accel_dev: *mut core::ffi::c_void) {
    adf_dev_down(accel_dev);
}

unsafe fn adf_dbgfs_cleanup(accel_dev: *mut core::ffi::c_void) {
    adf_dbgfs_exit(accel_dev);
}

unsafe fn adf_cfg_device_remove(accel_dev: *mut core::ffi::c_void) {
    adf_cfg_dev_remove(accel_dev);
}

unsafe fn adf_cleanup_hw_data(accel_dev: *mut core::ffi::c_void) {
    let accel_device = accel_dev as *mut adf_accel_dev;

    if !(*accel_device).hw_device.is_null() {
        adf_clean_hw_data_6xxx((*accel_device).hw_device);
        (*accel_device).hw_device = core::ptr::null_mut();
    }
}

unsafe fn adf_devmgr_remove(accel_dev: *mut core::ffi::c_void) {
    adf_devmgr_rm_dev(accel_dev, core::ptr::null_mut());
}

unsafe fn adf_gen6_cfg_dev_init(accel_dev: *mut adf_accel_dev) -> i32 {
    let config: *const core::ffi::c_char;

    /*
     * Wireless SKU - symmetric crypto service only
     * Non-wireless SKU - crypto service for even devices and compression for odd devices
     */
    if adf_6xxx_is_wcy(GET_HW_DATA(accel_dev)) {
        config = ADF_CFG_SYM;
    } else if (*accel_dev).accel_id % 2 != 0 {
        config = ADF_CFG_DC;
    } else {
        config = ADF_CFG_CY;
    }

    let mut ret = adf_cfg_section_add(accel_dev, ADF_GENERAL_SEC);
    if ret != 0 {
        return ret;
    }

    ret = adf_cfg_add_key_value_param(
        accel_dev,
        ADF_GENERAL_SEC,
        ADF_SERVICES_ENABLED,
        config,
        ADF_STR,
    );
    if ret != 0 {
        return ret;
    }

    adf_heartbeat_save_cfg_param(accel_dev, ADF_CFG_HB_TIMER_MIN_MS);
    0
}

unsafe fn adf_probe(
    pdev: *mut pci_dev,
    _ent: *const pci_device_id,
) -> i32 {
    let mut accel_pci_dev: *mut adf_accel_pci;
    let mut hw_data: *mut adf_hw_device_data;
    let dev = &mut (*pdev).dev as *mut device;
    let mut accel_dev: *mut adf_accel_dev;
    let mut bar: *mut adf_bar;
    let mut i: usize;
    let mut ret: i32;

    if num_possible_nodes() > 1 && dev_to_node(dev) < 0 {
        /* If the accelerator is connected to a node with no memory there is
         * no point in using it since the remote memory transaction is slow. */
        return dev_err_probe(dev, -EINVAL, c"Invalid NUMA configuration.\n");
    }

    accel_dev = devm_kzalloc(dev, core::mem::size_of::<adf_accel_dev>(), GFP_KERNEL)
        as *mut adf_accel_dev;
    if accel_dev.is_null() {
        return -ENOMEM;
    }

    INIT_LIST_HEAD(&mut (*accel_dev).crypto_list);
    INIT_LIST_HEAD(&mut (*accel_dev).list);
    accel_pci_dev = &mut (*accel_dev).accel_pci_dev;
    (*accel_pci_dev).pci_dev = pdev;
    (*accel_dev).owner = THIS_MODULE;

    hw_data = devm_kzalloc(dev, core::mem::size_of::<adf_hw_device_data>(), GFP_KERNEL)
        as *mut adf_hw_device_data;
    if hw_data.is_null() {
        return -ENOMEM;
    }

    pci_read_config_byte(pdev, PCI_REVISION_ID, &mut (*accel_pci_dev).revid);
    pci_read_config_dword(pdev, ADF_GEN6_FUSECTL4_OFFSET, &mut (*hw_data).fuses[ADF_FUSECTL4]);
    pci_read_config_dword(pdev, ADF_GEN6_FUSECTL0_OFFSET, &mut (*hw_data).fuses[ADF_FUSECTL0]);
    pci_read_config_dword(pdev, ADF_GEN6_FUSECTL1_OFFSET, &mut (*hw_data).fuses[ADF_FUSECTL1]);

    ret = pcim_enable_device(pdev);
    if ret != 0 {
        return dev_err_probe(dev, ret, c"Cannot enable PCI device.\n");
    }

    ret = adf_devmgr_add_dev(accel_dev, core::ptr::null_mut());
    if ret != 0 {
        return dev_err_probe(dev, ret, c"Failed to add new accelerator device.\n");
    }
    ret = devm_add_action_or_reset(dev, adf_devmgr_remove, accel_dev as *mut _);
    if ret != 0 { return ret; }

    (*accel_dev).hw_device = hw_data;
    adf_init_hw_data_6xxx((*accel_dev).hw_device);
    ret = devm_add_action_or_reset(dev, adf_cleanup_hw_data, accel_dev as *mut _);
    if ret != 0 { return ret; }

    (*hw_data).accel_mask = ((*hw_data).get_accel_mask)(hw_data);
    (*hw_data).ae_mask = ((*hw_data).get_ae_mask)(hw_data);
    (*accel_pci_dev).sku = ((*hw_data).get_sku)(hw_data);

    if (*hw_data).accel_mask == 0 || (*hw_data).ae_mask == 0 ||
       ((!(*hw_data).ae_mask) & ADF_GEN6_ACCELERATORS_MASK) != 0 {
        ret = -EFAULT;
        return dev_err_probe(dev, ret, c"No acceleration units were found.\n");
    }

    ret = adf_cfg_dev_add(accel_dev);
    if ret != 0 { return ret; }
    ret = devm_add_action_or_reset(dev, adf_cfg_device_remove, accel_dev as *mut _);
    if ret != 0 { return ret; }

    ret = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(64));
    if ret != 0 { return dev_err_probe(dev, ret, c"No usable DMA configuration.\n"); }
    ret = adf_gen6_cfg_dev_init(accel_dev);
    if ret != 0 { return dev_err_probe(dev, ret, c"Failed to initialize configuration.\n"); }

    (*hw_data).accel_capabilities_mask = ((*hw_data).get_accel_cap)(accel_dev);
    if (*hw_data).accel_capabilities_mask == 0 {
        ret = -EINVAL;
        return dev_err_probe(dev, ret, c"Failed to get capabilities mask.\n");
    }

    i = 0;
    while i < BAR_MAP.len() {
        bar = &mut (*accel_pci_dev).pci_bars[i];
        (*bar).virt_addr = pcim_iomap_region(pdev, BAR_MAP[i], pci_name(pdev));
        if IS_ERR((*bar).virt_addr) {
            ret = PTR_ERR((*bar).virt_addr);
            return dev_err_probe(dev, ret, c"Failed to ioremap PCI region.\n");
        }
        i += 1;
    }

    ret = pci_save_state(pdev);
    if ret != 0 { return dev_err_probe(dev, ret, c"Failed to save pci state.\n"); }
    (*accel_dev).ras_errors.enabled = true;
    adf_dbgfs_init(accel_dev);
    ret = devm_add_action_or_reset(dev, adf_dbgfs_cleanup, accel_dev as *mut _);
    if ret != 0 { return ret; }

    ret = adf_dev_up(accel_dev, true);
    if ret != 0 {
        adf_dev_down(accel_dev as *mut _);
        return ret;
    }
    ret = devm_add_action_or_reset(dev, adf_device_down, accel_dev as *mut _);
    if ret != 0 { return ret; }
    ret = adf_sysfs_init(accel_dev);
    if ret != 0 { return ret; }
    if ((*hw_data).fuses[ADF_FUSECTL0] & ADF_GEN6_KPT_FUSE_BIT) == 0 {
        ret = adf_sysfs_init_kpt(accel_dev);
    }
    ret
}

unsafe fn adf_shutdown(pdev: *mut pci_dev) {
    let accel_dev = adf_devmgr_pci_to_accel_dev(pdev);
    adf_dev_down(accel_dev);
}

// MODULE_DEVICE_TABLE(pci, adf_pci_tbl);
static mut ADF_PCI_TBL: [pci_device_id; 2] = [
    PCI_VDEVICE(INTEL, PCI_DEVICE_ID_INTEL_QAT_6XXX),
    pci_device_id::default(),
];

static mut ADF_DRIVER: pci_driver = pci_driver {
    id_table: ADF_PCI_TBL.as_ptr(),
    name: ADF_6XXX_DEVICE_NAME,
    probe: Some(adf_probe),
    shutdown: Some(adf_shutdown),
    sriov_configure: Some(adf_sriov_configure),
    err_handler: &adf_err_handler,
};

// module_pci_driver(adf_driver);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Intel");
// MODULE_FIRMWARE(ADF_6XXX_FW);
// MODULE_FIRMWARE(ADF_6XXX_MMP);
// MODULE_DESCRIPTION("Intel(R) QuickAssist Technology for GEN6 Devices");
// MODULE_SOFTDEP("pre: crypto-intel_qat");
// MODULE_IMPORT_NS("CRYPTO_QAT");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
