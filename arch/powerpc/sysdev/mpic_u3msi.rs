// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2006, Segher Boessenkool, IBM Corporation.
 * Copyright 2006-2007, Michael Ellerman, IBM Corporation.
 */

// Linux and architecture-specific declarations are supplied by external dependencies.

static mut MSI_MPIC: *mut mpic = core::ptr::null_mut();

unsafe fn mpic_u3msi_mask_irq(data: *mut irq_data) {
    pci_msi_mask_irq(data);
    mpic_mask_irq(data);
}

unsafe fn mpic_u3msi_unmask_irq(data: *mut irq_data) {
    mpic_unmask_irq(data);
    pci_msi_unmask_irq(data);
}

static mut MPIC_U3MSI_CHIP: irq_chip = irq_chip {
    irq_shutdown: Some(mpic_u3msi_mask_irq),
    irq_mask: Some(mpic_u3msi_mask_irq),
    irq_unmask: Some(mpic_u3msi_unmask_irq),
    irq_eoi: Some(mpic_end_irq),
    irq_set_type: Some(mpic_set_irq_type),
    irq_set_affinity: Some(mpic_set_affinity),
    name: "MPIC-U3MSI\0".as_ptr() as *const i8,
};

unsafe fn read_ht_magic_addr(pdev: *mut pci_dev, pos: u32) -> u64 {
    let mut flags: u8 = 0;
    let mut tmp: u32 = 0;
    let mut addr: u64;

    pci_read_config_byte(pdev, pos + HT_MSI_FLAGS, &mut flags);

    if flags & HT_MSI_FLAGS_FIXED != 0 {
        return HT_MSI_FIXED_ADDR;
    }

    pci_read_config_dword(pdev, pos + HT_MSI_ADDR_LO, &mut tmp);
    addr = (tmp & HT_MSI_ADDR_LO_MASK) as u64;
    pci_read_config_dword(pdev, pos + HT_MSI_ADDR_HI, &mut tmp);
    addr |= (tmp as u64) << 32;

    addr
}

unsafe fn find_ht_magic_addr(pdev: *mut pci_dev, _hwirq: u32) -> u64 {
    let mut bus: *mut pci_bus = (*pdev).bus;
    let mut pos: u32;

    while !bus.is_null() && !(*bus).self_.is_null() {
        pos = pci_find_ht_capability((*bus).self_, HT_CAPTYPE_MSI_MAPPING);
        if pos != 0 {
            return read_ht_magic_addr((*bus).self_, pos);
        }
        bus = (*bus).parent;
    }

    0
}

unsafe fn find_u4_magic_addr(pdev: *mut pci_dev, hwirq: u32) -> u64 {
    let hose: *mut pci_controller = pci_bus_to_host((*pdev).bus);

    /* U4 PCIe MSIs need to write to the special register in the bridge that
     * generates interrupts. There should be theoretically a register at
     * 0xf8005000 where you just write the MSI number and that triggers the
     * right interrupt, but unfortunately, this is busted in HW, the bridge
     * endian swaps the value and hits the wrong nibble in the register.
     *
     * So instead we use another register set which is used normally for
     * converting HT interrupts to MPIC interrupts, which decodes the interrupt
     * number as part of the low address bits
     *
     * This will not work if we ever use more than one legacy MSI in a block but
     * we never do. For one MSI or multiple MSI-X where each interrupt address
     * can be specified separately, it works just fine.
     */
    if of_device_is_compatible((*hose).dn, b"u4-pcie\0") != 0
        || of_device_is_compatible((*hose).dn, b"U4-pcie\0") != 0
    {
        return 0xf8004000 | ((hwirq as u64) << 4);
    }

    0
}

unsafe fn u3msi_teardown_msi_irqs(pdev: *mut pci_dev) {
    let mut entry: *mut msi_desc;
    let mut hwirq: irq_hw_number_t;

    msi_for_each_desc!(entry, &mut (*pdev).dev, MSI_DESC_ASSOCIATED, {
        hwirq = virq_to_hw((*entry).irq);
        irq_set_msi_desc((*entry).irq, core::ptr::null_mut());
        irq_dispose_mapping((*entry).irq);
        (*entry).irq = 0;
        msi_bitmap_free_hwirqs(&mut (*(*MSI_MPIC).msi_bitmap), hwirq, 1);
    });
}

unsafe fn u3msi_setup_msi_irqs(pdev: *mut pci_dev, _nvec: i32, type_: i32) -> i32 {
    let mut virq: u32;
    let mut entry: *mut msi_desc;
    let mut msg = msi_msg { address_lo: 0, address_hi: 0, data: 0 };
    let mut addr: u64;
    let mut hwirq: i32;

    if type_ == PCI_CAP_ID_MSIX {
        pr_debug!("u3msi: MSI-X untested, trying anyway.\n");
    }

    if find_ht_magic_addr(pdev, 0) == 0 && find_u4_magic_addr(pdev, 0) == 0 {
        pr_debug!("u3msi: no magic address found for %s\n", pci_name(pdev));
        return -ENXIO;
    }

    msi_for_each_desc!(entry, &mut (*pdev).dev, MSI_DESC_NOTASSOCIATED, {
        hwirq = msi_bitmap_alloc_hwirqs(&mut (*(*MSI_MPIC).msi_bitmap), 1);
        if hwirq < 0 { return hwirq; }

        addr = find_ht_magic_addr(pdev, hwirq as u32);
        if addr == 0 { addr = find_u4_magic_addr(pdev, hwirq as u32); }
        msg.address_lo = (addr & 0xffff_ffff) as u32;
        msg.address_hi = (addr >> 32) as u32;

        virq = irq_create_mapping((*MSI_MPIC).irqhost, hwirq as irq_hw_number_t);
        if virq == 0 {
            msi_bitmap_free_hwirqs(&mut (*(*MSI_MPIC).msi_bitmap), hwirq as irq_hw_number_t, 1);
            return -ENOSPC;
        }

        irq_set_msi_desc(virq, entry);
        irq_set_chip(virq, &mut MPIC_U3MSI_CHIP);
        irq_set_irq_type(virq, IRQ_TYPE_EDGE_RISING);
        pr_debug!("u3msi: allocated virq 0x%x (hw 0x%x) addr 0x%lx\n", virq, hwirq, addr as usize);
        printk!("u3msi: allocated virq 0x%x (hw 0x%x) addr 0x%lx\n", virq, hwirq, addr as usize);
        msg.data = hwirq as u32;
        pci_write_msi_msg(virq, &mut msg);
    });

    0
}

pub unsafe fn mpic_u3msi_init(mpic_: *mut mpic) -> i32 {
    let rc = mpic_msi_init_allocator(mpic_);
    if rc != 0 { return rc; }

    if !MSI_MPIC.is_null() { bug!(); }
    MSI_MPIC = mpic_;

    list_for_each_entry!(phb, hose_list, list_node, {
        warn_on!((*phb).controller_ops.setup_msi_irqs.is_some());
        (*phb).controller_ops.setup_msi_irqs = Some(u3msi_setup_msi_irqs);
        (*phb).controller_ops.teardown_msi_irqs = Some(u3msi_teardown_msi_irqs);
    });

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
