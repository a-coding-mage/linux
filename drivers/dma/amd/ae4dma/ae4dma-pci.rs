// SPDX-License-Identifier: GPL-2.0
/*
 * AMD AE4DMA driver
 *
 * Copyright (c) 2024, Advanced Micro Devices, Inc.
 * All Rights Reserved.
 *
 * Author: Basavaraj Natikar <Basavaraj.Natikar@amd.com>
 */

// Dependency declarations and build-time kernel configuration are supplied by
// the surrounding translation unit.

unsafe fn ae4_get_irqs(ae4: *mut ae4_device) -> i32 {
    let ae4_msix = (*ae4).ae4_msix;
    let pt = &mut (*ae4).pt as *mut pt_device;
    let dev = (*pt).dev;
    let pdev: *mut pci_dev;
    let mut i: i32;
    let mut v: i32;
    let mut ret: i32;

    pdev = to_pci_dev(dev);

    v = 0;
    while v < (*ae4_msix).msix_entry.len() as i32 {
        (*ae4_msix).msix_entry[v as usize].entry = v;
        v += 1;
    }

    ret = pci_alloc_irq_vectors(pdev, v, v, PCI_IRQ_MSIX);
    if ret != v {
        if ret > 0 {
            pci_free_irq_vectors(pdev);
        }

        dev_err(dev, "could not enable MSI-X (%d), trying MSI\\n", ret);
        ret = pci_alloc_irq_vectors(pdev, 1, 1, PCI_IRQ_MSI);
        if ret < 0 {
            dev_err(dev, "could not enable MSI (%d)\\n", ret);
            return ret;
        }

        ret = pci_irq_vector(pdev, 0);
        if ret < 0 {
            pci_free_irq_vectors(pdev);
            return ret;
        }

        i = 0;
        while i < MAX_AE4_HW_QUEUES {
            (*ae4).ae4_irq[i as usize] = ret;
            i += 1;
        }
    } else {
        (*ae4_msix).msix_count = ret;
        i = 0;
        while i < (*ae4_msix).msix_count {
            (*ae4).ae4_irq[i as usize] = pci_irq_vector(pdev, i);
            i += 1;
        }
    }

    ret
}

unsafe fn ae4_free_irqs(ae4: *mut ae4_device) {
    let ae4_msix = (*ae4).ae4_msix;
    let pt = &mut (*ae4).pt as *mut pt_device;
    let dev = (*pt).dev;
    let pdev: *mut pci_dev;

    pdev = to_pci_dev(dev);

    if !ae4_msix.is_null()
        && ((*ae4_msix).msix_count != 0
            || (*ae4).ae4_irq[(MAX_AE4_HW_QUEUES - 1) as usize] != 0)
    {
        pci_free_irq_vectors(pdev);
    }
}

unsafe fn ae4_deinit(ae4: *mut ae4_device) {
    ae4_free_irqs(ae4);
}

unsafe fn ae4_pci_probe(pdev: *mut pci_dev, _id: *const pci_device_id) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let ae4: *mut ae4_device;
    let pt: *mut pt_device;
    let mut bar_mask: i32;
    let mut ret: i32 = 0;

    ae4 = devm_kzalloc(dev, core::mem::size_of::<ae4_device>(), GFP_KERNEL) as *mut ae4_device;
    if ae4.is_null() {
        return -ENOMEM;
    }

    (*ae4).ae4_msix = devm_kzalloc(dev, core::mem::size_of::<ae4_msix>(), GFP_KERNEL)
        as *mut ae4_msix;
    if (*ae4).ae4_msix.is_null() {
        return -ENOMEM;
    }

    ret = pcim_enable_device(pdev);
    if ret != 0 {
        ae4_deinit(ae4);
        return ret;
    }

    bar_mask = pci_select_bars(pdev, IORESOURCE_MEM);
    ret = pcim_iomap_regions(pdev, bar_mask, "ae4dma");
    if ret != 0 {
        ae4_deinit(ae4);
        return ret;
    }

    pt = &mut (*ae4).pt;
    (*pt).dev = dev;
    (*pt).ver = AE4_DMA_VERSION;

    (*pt).io_regs = pcim_iomap_table(pdev)[0];
    if (*pt).io_regs.is_null() {
        ret = -ENOMEM;
        ae4_deinit(ae4);
        return ret;
    }

    ret = ae4_get_irqs(ae4);
    if ret < 0 {
        ae4_deinit(ae4);
        return ret;
    }

    pci_set_master(pdev);
    dma_set_mask_and_coherent(dev, DMA_BIT_MASK(48));
    dev_set_drvdata(dev, ae4 as *mut core::ffi::c_void);

    ret = ae4_core_init(ae4);
    if ret != 0 {
        ae4_deinit(ae4);
        return ret;
    }

    return 0;
}

unsafe fn ae4_pci_remove(pdev: *mut pci_dev) {
    let ae4 = dev_get_drvdata(&mut (*pdev).dev) as *mut ae4_device;
    ae4_destroy_work(ae4);
    ae4_deinit(ae4);
}

static mut ae4_pci_table: [pci_device_id; 2] = [
    PCI_VDEVICE!(AMD, 0x149B),
    // Last entry must be zero
    pci_device_id { vendor: 0, device: 0 },
];

static mut ae4_pci_driver: pci_driver = pci_driver {
    name: "ae4dma",
    id_table: ae4_pci_table.as_ptr(),
    probe: Some(ae4_pci_probe),
    remove: Some(ae4_pci_remove),
};

// MODULE_DEVICE_TABLE(pci, ae4_pci_table);
// module_pci_driver(ae4_pci_driver);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("AMD AE4DMA driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
