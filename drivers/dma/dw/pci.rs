// SPDX-License-Identifier: GPL-2.0
/*
 * PCI driver for the Synopsys DesignWare DMA Controller
 *
 * Copyright (C) 2013 Intel Corporation
 * Author: Andy Shevchenko <andriy.shevchenko@linux.intel.com>
 */

// Dependencies supplied by the surrounding kernel/Rust translation.

unsafe fn dw_pci_probe(
    pdev: *mut pci_dev,
    pid: *const pci_device_id,
) -> i32 {
    let drv_data = (*pid).driver_data as *const dw_dma_chip_pdata;
    let mut data: *mut dw_dma_chip_pdata;
    let mut chip: *mut dw_dma_chip;
    let mut ret: i32;

    ret = pcim_enable_device(pdev);
    if ret != 0 {
        return ret;
    }

    ret = pcim_iomap_regions(pdev, 1 << 0, pci_name(pdev));
    if ret != 0 {
        dev_err(&mut (*pdev).dev, "I/O memory remapping failed\n");
        return ret;
    }

    pci_set_master(pdev);
    pci_try_set_mwi(pdev);

    ret = dma_set_mask_and_coherent(&mut (*pdev).dev, dma_bit_mask(32));
    if ret != 0 {
        return ret;
    }

    data = devm_kmemdup(
        &mut (*pdev).dev,
        drv_data as *const core::ffi::c_void,
        core::mem::size_of::<dw_dma_chip_pdata>(),
        GFP_KERNEL,
    ) as *mut dw_dma_chip_pdata;
    if data.is_null() {
        return -12; // -ENOMEM
    }

    chip = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<dw_dma_chip>(),
        GFP_KERNEL,
    ) as *mut dw_dma_chip;
    if chip.is_null() {
        return -12; // -ENOMEM
    }

    (*chip).dev = &mut (*pdev).dev;
    (*chip).id = (*pdev).devfn;
    (*chip).regs = *pcim_iomap_table(pdev);
    (*chip).irq = (*pdev).irq;
    (*chip).pdata = (*data).pdata;

    (*data).chip = chip;

    ret = ((*data).probe)(chip);
    if ret != 0 {
        return ret;
    }

    pci_set_drvdata(pdev, data as *mut core::ffi::c_void);

    dw_dma_acpi_controller_register((*chip).dw);

    0
}

unsafe fn dw_pci_remove(pdev: *mut pci_dev) {
    let data = pci_get_drvdata(pdev) as *mut dw_dma_chip_pdata;
    let chip = (*data).chip;
    let mut ret: i32;

    dw_dma_acpi_controller_free((*chip).dw);

    ret = ((*data).remove)(chip);
    if ret != 0 {
        dev_warn(&mut (*pdev).dev, "can't remove device properly: %d\n", ret);
    }
}

unsafe fn dw_pci_suspend_late(dev: *mut device) -> i32 {
    let data = dev_get_drvdata(dev) as *mut dw_dma_chip_pdata;
    let chip = (*data).chip;

    do_dw_dma_disable(chip)
}

unsafe fn dw_pci_resume_early(dev: *mut device) -> i32 {
    let data = dev_get_drvdata(dev) as *mut dw_dma_chip_pdata;
    let chip = (*data).chip;

    do_dw_dma_enable(chip)
}

static DW_PCI_DEV_PM_OPS: dev_pm_ops = dev_pm_ops {
    // LATE_SYSTEM_SLEEP_PM_OPS(dw_pci_suspend_late, dw_pci_resume_early)
    ..Default::default()
};

static DW_PCI_ID_TABLE: [pci_device_id; 12] = [
    // Medfield (GPDMA)
    pci_device_id::vdevice(INTEL, 0x0827, &dw_dma_chip_pdata),

    // BayTrail
    pci_device_id::vdevice(INTEL, 0x0f06, &dw_dma_chip_pdata),
    pci_device_id::vdevice(INTEL, 0x0f40, &dw_dma_chip_pdata),

    // Merrifield
    pci_device_id::vdevice(INTEL, 0x11a2, &idma32_chip_pdata),

    // Braswell
    pci_device_id::vdevice(INTEL, 0x2286, &dw_dma_chip_pdata),
    pci_device_id::vdevice(INTEL, 0x22c0, &dw_dma_chip_pdata),

    // Elkhart Lake iDMA 32-bit (PSE DMA)
    pci_device_id::vdevice(INTEL, 0x4bb4, &xbar_chip_pdata),
    pci_device_id::vdevice(INTEL, 0x4bb5, &xbar_chip_pdata),
    pci_device_id::vdevice(INTEL, 0x4bb6, &xbar_chip_pdata),

    // Haswell
    pci_device_id::vdevice(INTEL, 0x9c60, &dw_dma_chip_pdata),

    // Broadwell
    pci_device_id::vdevice(INTEL, 0x9ce0, &dw_dma_chip_pdata),

    pci_device_id::default_entry(),
];

static mut DW_PCI_DRIVER: pci_driver = pci_driver {
    name: "dw_dmac_pci",
    id_table: DW_PCI_ID_TABLE.as_ptr(),
    probe: Some(dw_pci_probe),
    remove: Some(dw_pci_remove),
    driver: driver {
        pm: pm_sleep_ptr(&DW_PCI_DEV_PM_OPS),
        ..Default::default()
    },
};

// module_pci_driver(DW_PCI_DRIVER)
// MODULE_LICENSE("GPL v2");
// MODULE_DESCRIPTION("Synopsys DesignWare DMA Controller PCI driver");
// MODULE_AUTHOR("Andy Shevchenko <andriy.shevchenko@linux.intel.com>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
