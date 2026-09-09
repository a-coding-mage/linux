// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2007, Olof Johansson, PA Semi
 *
 * Based on arch/powerpc/sysdev/mpic_u3msi.c:
 *
 * Copyright 2006, Segher Boessenkool, IBM Corporation.
 * Copyright 2006-2007, Michael Ellerman, IBM Corporation.
 */

// Linux kernel dependencies supplied by the surrounding repository.

const ALLOC_CHUNK: usize = 16;
const PASEMI_MSI_ADDR: u32 = 0xfc080000;

static mut MSI_MPIC: *mut mpic = core::ptr::null_mut();

unsafe fn mpic_pasemi_msi_mask_irq(data: *mut irq_data) {
    pr_debug!("mpic_pasemi_msi_mask_irq {}\n", (*data).irq);
    pci_msi_mask_irq(data);
    mpic_mask_irq(data);
}

unsafe fn mpic_pasemi_msi_unmask_irq(data: *mut irq_data) {
    pr_debug!("mpic_pasemi_msi_unmask_irq {}\n", (*data).irq);
    mpic_unmask_irq(data);
    pci_msi_unmask_irq(data);
}

static mut MPIC_PASEMI_MSI_CHIP: irq_chip = irq_chip {
    irq_shutdown: Some(mpic_pasemi_msi_mask_irq),
    irq_mask: Some(mpic_pasemi_msi_mask_irq),
    irq_unmask: Some(mpic_pasemi_msi_unmask_irq),
    irq_eoi: Some(mpic_end_irq),
    irq_set_type: Some(mpic_set_irq_type),
    irq_set_affinity: Some(mpic_set_affinity),
    name: b"PASEMI-MSI\0".as_ptr() as *const i8,
};

unsafe fn pasemi_msi_teardown_msi_irqs(pdev: *mut pci_dev) {
    let mut entry: *mut msi_desc;
    let mut hwirq: irq_hw_number_t;

    pr_debug!("pasemi_msi_teardown_msi_irqs, pdev {:p}\n", pdev);

    // msi_for_each_desc(entry, &pdev->dev, MSI_DESC_ASSOCIATED)
    msi_for_each_desc!(entry, &mut (*pdev).dev, MSI_DESC_ASSOCIATED, {
        hwirq = virq_to_hw((*entry).irq);
        irq_set_msi_desc((*entry).irq, core::ptr::null_mut());
        irq_dispose_mapping((*entry).irq);
        (*entry).irq = 0;
        msi_bitmap_free_hwirqs(&mut (*MSI_MPIC).msi_bitmap, hwirq, ALLOC_CHUNK);
    });
}

unsafe fn pasemi_msi_setup_msi_irqs(
    pdev: *mut pci_dev,
    nvec: i32,
    type_: i32,
) -> i32 {
    let mut virq: u32;
    let mut entry: *mut msi_desc;
    let mut msg = msi_msg {
        address_hi: 0,
        address_lo: PASEMI_MSI_ADDR,
        data: 0,
    };
    let mut hwirq: i32;

    if type_ == PCI_CAP_ID_MSIX {
        pr_debug!("pasemi_msi: MSI-X untested, trying anyway\n");
    }
    pr_debug!(
        "pasemi_msi_setup_msi_irqs, pdev {:p} nvec {} type {}\n",
        pdev, nvec, type_
    );

    // msi_for_each_desc(entry, &pdev->dev, MSI_DESC_NOTASSOCIATED)
    msi_for_each_desc!(entry, &mut (*pdev).dev, MSI_DESC_NOTASSOCIATED, {
        hwirq = msi_bitmap_alloc_hwirqs(&mut (*MSI_MPIC).msi_bitmap, ALLOC_CHUNK);
        if hwirq < 0 {
            pr_debug!("pasemi_msi: failed allocating hwirq\n");
            return hwirq;
        }

        virq = irq_create_mapping((*MSI_MPIC).irqhost, hwirq as irq_hw_number_t);
        if virq == 0 {
            pr_debug!("pasemi_msi: failed mapping hwirq 0x{:x}\n", hwirq);
            msi_bitmap_free_hwirqs(
                &mut (*MSI_MPIC).msi_bitmap,
                hwirq as irq_hw_number_t,
                ALLOC_CHUNK,
            );
            return -ENOSPC;
        }

        mpic_set_vector(virq, 0);
        irq_set_msi_desc(virq, entry);
        irq_set_chip(virq, &raw mut MPIC_PASEMI_MSI_CHIP);
        irq_set_irq_type(virq, IRQ_TYPE_EDGE_RISING);

        pr_debug!(
            "pasemi_msi: allocated virq 0x{:x} (hw 0x{:x}) addr 0x{:x}\n",
            virq, hwirq, msg.address_lo
        );

        msg.data = (hwirq - 0x200) as u32;
        pci_write_msi_msg(virq, &mut msg);
    });

    0
}

pub unsafe fn mpic_pasemi_msi_init(mpic: *mut mpic) -> i32 {
    let mut rc: i32;
    let mut phb: *mut pci_controller;
    let of_node: *mut device_node;

    of_node = irq_domain_get_of_node((*mpic).irqhost);
    if of_node.is_null()
        || !of_device_is_compatible(of_node, b"pasemi,pwrficient-openpic\0".as_ptr() as *const i8)
    {
        return -ENODEV;
    }

    rc = mpic_msi_init_allocator(mpic);
    if rc != 0 {
        pr_debug!("pasemi_msi: Error allocating bitmap!\n");
        return rc;
    }

    pr_debug!("pasemi_msi: Registering PA Semi MPIC MSI callbacks\n");

    MSI_MPIC = mpic;
    list_for_each_entry!(phb, &mut hose_list, list_node, {
        WARN_ON((*phb).controller_ops.setup_msi_irqs.is_some());
        (*phb).controller_ops.setup_msi_irqs = Some(pasemi_msi_setup_msi_irqs);
        (*phb).controller_ops.teardown_msi_irqs = Some(pasemi_msi_teardown_msi_irqs);
    });

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
