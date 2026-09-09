// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * MSI support for PPC4xx SoCs using High Speed Transfer Assist (HSTA) for
 * generation of the interrupt.
 *
 * Copyright © 2013 Alistair Popple <alistair@popple.id.au> IBM Corporation
 */

#[repr(C)]
struct Ppc4xxHstaMsi {
    dev: *mut device,

    /* The ioremapped HSTA MSI IO space */
    data: *mut u32,

    /* Physical address of HSTA MSI IO space */
    address: u64,
    bmp: msi_bitmap,

    /* An array mapping offsets to hardware IRQs */
    irq_map: *mut i32,

    /* Number of hwirqs supported */
    irq_count: i32,
}

static mut PPC4XX_HSTA_MSI: Ppc4xxHstaMsi = Ppc4xxHstaMsi {
    dev: core::ptr::null_mut(),
    data: core::ptr::null_mut(),
    address: 0,
    bmp: unsafe { core::mem::zeroed() },
    irq_map: core::ptr::null_mut(),
    irq_count: 0,
};

unsafe fn hsta_setup_msi_irqs(dev: *mut pci_dev, _nvec: i32, type_: i32) -> i32 {
    let mut msg: msi_msg = core::mem::zeroed();
    let mut entry: *mut msi_desc;
    let mut irq: i32;
    let mut hwirq: i32;
    let mut addr: u64;

    /* We don't support MSI-X */
    if type_ == PCI_CAP_ID_MSIX {
        pr_debug!("{}: MSI-X not supported.\n", "hsta_setup_msi_irqs");
        return -EINVAL;
    }

    msi_for_each_desc!(entry, (*dev).dev, MSI_DESC_NOTASSOCIATED, {
        irq = msi_bitmap_alloc_hwirqs(&mut PPC4XX_HSTA_MSI.bmp, 1);
        if irq < 0 {
            pr_debug!("{}: Failed to allocate msi interrupt\n", "hsta_setup_msi_irqs");
            return irq;
        }

        hwirq = *PPC4XX_HSTA_MSI.irq_map.add(irq as usize);
        if hwirq == 0 {
            pr_err!("{}: Failed mapping irq {}\n", "hsta_setup_msi_irqs", irq);
            return -EINVAL;
        }

        /*
         * HSTA generates interrupts on writes to 128-bit aligned
         * addresses.
         */
        addr = PPC4XX_HSTA_MSI.address.wrapping_add((irq as u64).wrapping_mul(0x10));
        msg.address_hi = (addr >> 32) as u32;
        msg.address_lo = addr as u32;

        /* Data is not used by the HSTA. */
        msg.data = 0;

        pr_debug!("{}: Setup irq {} (0x{:0llx})\n", "hsta_setup_msi_irqs", hwirq,
                  ((msg.address_hi as u64) << 32) | msg.address_lo as u64);

        if irq_set_msi_desc(hwirq, entry) != 0 {
            pr_err!("{}: Invalid hwirq {} specified in device tree\n",
                    "hsta_setup_msi_irqs", hwirq);
            msi_bitmap_free_hwirqs(&mut PPC4XX_HSTA_MSI.bmp, irq, 1);
            return -EINVAL;
        }
        pci_write_msi_msg(hwirq, &msg);
    });

    0
}

unsafe fn hsta_find_hwirq_offset(hwirq: i32) -> i32 {
    let mut irq: i32 = 0;
    while irq < PPC4XX_HSTA_MSI.irq_count {
        if *PPC4XX_HSTA_MSI.irq_map.add(irq as usize) == hwirq {
            return irq;
        }
        irq += 1;
    }
    -EINVAL
}

unsafe fn hsta_teardown_msi_irqs(dev: *mut pci_dev) {
    let mut entry: *mut msi_desc;
    let mut irq: i32;

    msi_for_each_desc!(entry, (*dev).dev, MSI_DESC_ASSOCIATED, {
        irq = hsta_find_hwirq_offset((*entry).irq);

        /* entry->irq should always be in irq_map */
        BUG_ON!(irq < 0);
        irq_set_msi_desc((*entry).irq, core::ptr::null_mut());
        msi_bitmap_free_hwirqs(&mut PPC4XX_HSTA_MSI.bmp, irq, 1);
        pr_debug!("{}: Teardown IRQ {} (index {})\n", "hsta_teardown_msi_irqs", (*entry).irq, irq);
        (*entry).irq = 0;
    });
}

unsafe fn hsta_msi_probe(pdev: *mut platform_device) -> i32 {
    let dev: *mut device = &mut (*pdev).dev;
    let mut mem: *mut resource;
    let mut irq: i32;
    let mut ret: i32;
    let mut irq_count: i32;
    let mut phb: *mut pci_controller;

    mem = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if mem.is_null() {
        dev_err!(dev, "Unable to get mmio space\n");
        return -EINVAL;
    }

    irq_count = of_irq_count((*dev).of_node);
    if irq_count == 0 {
        dev_err!(dev, "Unable to find IRQ range\n");
        return -EINVAL;
    }

    PPC4XX_HSTA_MSI.dev = dev;
    PPC4XX_HSTA_MSI.address = (*mem).start;
    PPC4XX_HSTA_MSI.data = ioremap((*mem).start, resource_size(mem));
    PPC4XX_HSTA_MSI.irq_count = irq_count;
    if PPC4XX_HSTA_MSI.data.is_null() {
        dev_err!(dev, "Unable to map memory\n");
        return -ENOMEM;
    }

    ret = msi_bitmap_alloc(&mut PPC4XX_HSTA_MSI.bmp, irq_count, (*dev).of_node);
    if ret != 0 { goto_out!(); }

    PPC4XX_HSTA_MSI.irq_map = kmalloc_objs!(i32, irq_count);
    if PPC4XX_HSTA_MSI.irq_map.is_null() {
        ret = -ENOMEM;
        goto_out1!();
    }

    /* Setup a mapping from irq offsets to hardware irq numbers */
    irq = 0;
    while irq < irq_count {
        *PPC4XX_HSTA_MSI.irq_map.add(irq as usize) = irq_of_parse_and_map((*dev).of_node, irq);
        if *PPC4XX_HSTA_MSI.irq_map.add(irq as usize) == 0 {
            dev_err!(dev, "Unable to map IRQ\n");
            ret = -EINVAL;
            goto_out2!();
        }
        irq += 1;
    }

    list_for_each_entry!(phb, hose_list, list_node, {
        (*phb).controller_ops.setup_msi_irqs = Some(hsta_setup_msi_irqs);
        (*phb).controller_ops.teardown_msi_irqs = Some(hsta_teardown_msi_irqs);
    });
    return 0;

    // C cleanup labels preserved as explicit control-flow intent.
    goto_out2!();
    goto_out1!();
    goto_out!();
}

static HSTA_MSI_IDS: [of_device_id; 2] = [
    of_device_id { compatible: "ibm,hsta-msi" },
    of_device_id { compatible: "" },
];

static mut HSTA_MSI_DRIVER: platform_driver = platform_driver {
    probe: Some(hsta_msi_probe),
    driver: driver { name: "hsta-msi", of_match_table: HSTA_MSI_IDS.as_ptr() },
};

unsafe fn hsta_msi_init() -> i32 {
    platform_driver_register(&mut HSTA_MSI_DRIVER)
}

subsys_initcall!(hsta_msi_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
