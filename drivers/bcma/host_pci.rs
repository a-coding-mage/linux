/*
 * Broadcom specific AMBA
 * PCI Host
 *
 * Licensed under the GNU/GPL. See COPYING for details.
 */

/* Dependencies are supplied by the surrounding kernel translation. */

unsafe fn bcma_host_pci_switch_core(core: *mut bcma_device) {
    let win2: i32 = if (*(*core).bus).host_is_pcie2 {
        BCMA_PCIE2_BAR0_WIN2
    } else {
        BCMA_PCI_BAR0_WIN2
    };

    pci_write_config_dword((*core).bus.host_pci, BCMA_PCI_BAR0_WIN, (*core).addr);
    pci_write_config_dword((*core).bus.host_pci, win2, (*core).wrap);
    (*(*core).bus).mapped_core = core;
    bcma_debug((*core).bus, "Switched to core: 0x%X\n", (*core).id.id);
}

/* Provides access to the requested core. Returns base offset that has to be
 * used. It makes use of fixed windows when possible. */
unsafe fn bcma_host_pci_provide_access_to_core(core: *mut bcma_device) -> u16 {
    match (*core).id.id {
        BCMA_CORE_CHIPCOMMON => return 3 * BCMA_CORE_SIZE,
        BCMA_CORE_PCIE => return 2 * BCMA_CORE_SIZE,
        _ => {}
    }

    if (*(*core).bus).mapped_core != core {
        bcma_host_pci_switch_core(core);
    }
    0
}

unsafe fn bcma_host_pci_read8(core: *mut bcma_device, mut offset: u16) -> u8 {
    offset = offset.wrapping_add(bcma_host_pci_provide_access_to_core(core));
    ioread8((*(*core).bus).mmio.add(offset as usize))
}

unsafe fn bcma_host_pci_read16(core: *mut bcma_device, mut offset: u16) -> u16 {
    offset = offset.wrapping_add(bcma_host_pci_provide_access_to_core(core));
    ioread16((*(*core).bus).mmio.add(offset as usize))
}

unsafe fn bcma_host_pci_read32(core: *mut bcma_device, mut offset: u16) -> u32 {
    offset = offset.wrapping_add(bcma_host_pci_provide_access_to_core(core));
    ioread32((*(*core).bus).mmio.add(offset as usize))
}

unsafe fn bcma_host_pci_write8(core: *mut bcma_device, mut offset: u16, value: u8) {
    offset = offset.wrapping_add(bcma_host_pci_provide_access_to_core(core));
    iowrite8(value, (*(*core).bus).mmio.add(offset as usize));
}

unsafe fn bcma_host_pci_write16(core: *mut bcma_device, mut offset: u16, value: u16) {
    offset = offset.wrapping_add(bcma_host_pci_provide_access_to_core(core));
    iowrite16(value, (*(*core).bus).mmio.add(offset as usize));
}

unsafe fn bcma_host_pci_write32(core: *mut bcma_device, mut offset: u16, value: u32) {
    offset = offset.wrapping_add(bcma_host_pci_provide_access_to_core(core));
    iowrite32(value, (*(*core).bus).mmio.add(offset as usize));
}

#[cfg(CONFIG_BCMA_BLOCKIO)]
unsafe fn bcma_host_pci_block_read(core: *mut bcma_device, buffer: *mut core::ffi::c_void,
                                   count: usize, offset: u16, reg_width: u8) {
    let addr = (*(*core).bus).mmio.add(offset as usize);
    if (*(*core).bus).mapped_core != core {
        bcma_host_pci_switch_core(core);
    }
    match reg_width as usize {
        size if size == core::mem::size_of::<u8>() => ioread8_rep(addr, buffer, count),
        size if size == core::mem::size_of::<u16>() => {
            WARN_ON(count & 1 != 0);
            ioread16_rep(addr, buffer, count >> 1);
        }
        size if size == core::mem::size_of::<u32>() => {
            WARN_ON(count & 3 != 0);
            ioread32_rep(addr, buffer, count >> 2);
        }
        _ => WARN_ON(true),
    }
}

#[cfg(CONFIG_BCMA_BLOCKIO)]
unsafe fn bcma_host_pci_block_write(core: *mut bcma_device, buffer: *const core::ffi::c_void,
                                    count: usize, offset: u16, reg_width: u8) {
    let addr = (*(*core).bus).mmio.add(offset as usize);
    if (*(*core).bus).mapped_core != core {
        bcma_host_pci_switch_core(core);
    }
    match reg_width as usize {
        size if size == core::mem::size_of::<u8>() => iowrite8_rep(addr, buffer, count),
        size if size == core::mem::size_of::<u16>() => {
            WARN_ON(count & 1 != 0);
            iowrite16_rep(addr, buffer, count >> 1);
        }
        size if size == core::mem::size_of::<u32>() => {
            WARN_ON(count & 3 != 0);
            iowrite32_rep(addr, buffer, count >> 2);
        }
        _ => WARN_ON(true),
    }
}

unsafe fn bcma_host_pci_aread32(core: *mut bcma_device, offset: u16) -> u32 {
    if (*(*core).bus).mapped_core != core {
        bcma_host_pci_switch_core(core);
    }
    ioread32((*(*core).bus).mmio.add((BCMA_CORE_SIZE + offset) as usize))
}

unsafe fn bcma_host_pci_awrite32(core: *mut bcma_device, offset: u16, value: u32) {
    if (*(*core).bus).mapped_core != core {
        bcma_host_pci_switch_core(core);
    }
    iowrite32(value, (*(*core).bus).mmio.add((BCMA_CORE_SIZE + offset) as usize));
}

static bcma_host_pci_ops: bcma_host_ops = bcma_host_ops {
    read8: Some(bcma_host_pci_read8),
    read16: Some(bcma_host_pci_read16),
    read32: Some(bcma_host_pci_read32),
    write8: Some(bcma_host_pci_write8),
    write16: Some(bcma_host_pci_write16),
    write32: Some(bcma_host_pci_write32),
    #[cfg(CONFIG_BCMA_BLOCKIO)]
    block_read: Some(bcma_host_pci_block_read),
    #[cfg(CONFIG_BCMA_BLOCKIO)]
    block_write: Some(bcma_host_pci_block_write),
    aread32: Some(bcma_host_pci_aread32),
    awrite32: Some(bcma_host_pci_awrite32),
};

unsafe fn bcma_host_pci_probe(dev: *mut pci_dev, _id: *const pci_device_id) -> i32 {
    let mut bus: *mut bcma_bus;
    let mut err: i32 = -ENOMEM;
    let mut val: u32 = 0;

    /* Alloc */
    bus = kzalloc_obj::<bcma_bus>();
    if bus.is_null() { return err; }

    /* Basic PCI configuration */
    err = pci_enable_device(dev);
    if err != 0 { goto_err_kfree_bus: { kfree(bus); return err; } }

    err = pci_request_regions(dev, "bcma-pci-bridge");
    if err != 0 { pci_disable_device(dev); goto_err_kfree_bus: { kfree(bus); return err; } }
    pci_set_master(dev);

    /* Disable the RETRY_TIMEOUT register (0x41) to keep
     * PCI Tx retries from interfering with C3 CPU state */
    pci_read_config_dword(dev, 0x40, &mut val);
    if (val & 0x0000ff00 != 0 { pci_write_config_dword(dev, 0x40, val & 0xffff00ff); }

    /* SSB needed additional powering up, do we have any AMBA PCI cards? */
    if !pci_is_pcie(dev) {
        bcma_err(bus, "PCI card detected, they are not supported.\n");
        err = -ENXIO;
        pci_release_regions(dev); pci_disable_device(dev); kfree(bus); return err;
    }

    (*bus).dev = &mut (*dev).dev;
    /* Map MMIO */
    err = -ENOMEM;
    (*bus).mmio = pci_iomap(dev, 0, !0usize);
    if (*bus).mmio.is_null() { pci_release_regions(dev); pci_disable_device(dev); kfree(bus); return err; }

    /* Host specific */
    (*bus).host_pci = dev;
    (*bus).hosttype = BCMA_HOSTTYPE_PCI;
    (*bus).ops = &bcma_host_pci_ops;
    (*bus).boardinfo.vendor = (*dev).subsystem_vendor;
    (*bus).boardinfo.type_ = (*dev).subsystem_device;

    /* Initialize struct, detect chip */
    bcma_init_bus(bus);
    /* Scan bus to find out generation of PCIe core */
    err = bcma_bus_scan(bus);
    if err != 0 { pci_iounmap(dev, (*bus).mmio); pci_release_regions(dev); pci_disable_device(dev); kfree(bus); return err; }
    if !bcma_find_core(bus, BCMA_CORE_PCIE2).is_null() { (*bus).host_is_pcie2 = true; }
    /* Register */
    err = bcma_bus_register(bus);
    if err != 0 { bcma_unregister_cores(bus); pci_iounmap(dev, (*bus).mmio); pci_release_regions(dev); pci_disable_device(dev); kfree(bus); return err; }
    pci_set_drvdata(dev, bus);
    err
}

unsafe fn bcma_host_pci_remove(dev: *mut pci_dev) {
    let bus = pci_get_drvdata(dev);
    bcma_bus_unregister(bus);
    pci_iounmap(dev, (*bus).mmio);
    pci_release_regions(dev);
    pci_disable_device(dev);
    kfree(bus);
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn bcma_host_pci_suspend(dev: *mut device) -> i32 {
    let bus = dev_get_drvdata(dev);
    (*bus).mapped_core = core::ptr::null_mut();
    bcma_bus_suspend(bus)
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn bcma_host_pci_resume(dev: *mut device) -> i32 {
    bcma_bus_resume(dev_get_drvdata(dev))
}

/* PCI_DEVICE and PCI_DEVICE_SUB entries are retained as dependency macros. */
static bcma_pci_bridge_tbl: [pci_device_id; 21] = [
    PCI_DEVICE!(PCI_VENDOR_ID_BROADCOM, 0x0576), PCI_DEVICE!(PCI_VENDOR_ID_BROADCOM, 0x4313),
    PCI_DEVICE!(PCI_VENDOR_ID_BROADCOM, 43224), PCI_DEVICE!(PCI_VENDOR_ID_BROADCOM, 0x4331),
    PCI_DEVICE!(PCI_VENDOR_ID_BROADCOM, 0x4353), PCI_DEVICE!(PCI_VENDOR_ID_BROADCOM, 0x4357),
    PCI_DEVICE!(PCI_VENDOR_ID_BROADCOM, 0x4358), PCI_DEVICE!(PCI_VENDOR_ID_BROADCOM, 0x4359),
    PCI_DEVICE!(PCI_VENDOR_ID_BROADCOM, 0x4360),
    PCI_DEVICE_SUB!(PCI_VENDOR_ID_BROADCOM, 0x4365, PCI_VENDOR_ID_DELL, 0x0016),
    PCI_DEVICE_SUB!(PCI_VENDOR_ID_BROADCOM, 0x4365, PCI_VENDOR_ID_DELL, 0x0018),
    PCI_DEVICE_SUB!(PCI_VENDOR_ID_BROADCOM, 0x4365, PCI_VENDOR_ID_FOXCONN, 0xe092),
    PCI_DEVICE_SUB!(PCI_VENDOR_ID_BROADCOM, 0x4365, PCI_VENDOR_ID_HP, 0x804a),
    PCI_DEVICE!(PCI_VENDOR_ID_BROADCOM, 0x43a0), PCI_DEVICE!(PCI_VENDOR_ID_BROADCOM, 0x43a9),
    PCI_DEVICE!(PCI_VENDOR_ID_BROADCOM, 0x43aa), PCI_DEVICE!(PCI_VENDOR_ID_BROADCOM, 0x43b1),
    PCI_DEVICE!(PCI_VENDOR_ID_BROADCOM, 0x4727), PCI_DEVICE!(PCI_VENDOR_ID_BROADCOM, 43227),
    PCI_DEVICE!(PCI_VENDOR_ID_BROADCOM, 43228), pci_device_id { ..unsafe { core::mem::zeroed() } },
];

/* See also pcicore_up */
unsafe fn bcma_host_pci_up(bus: *mut bcma_bus) {
    if (*bus).hosttype != BCMA_HOSTTYPE_PCI { return; }
    if (*bus).host_is_pcie2 { bcma_core_pcie2_up(&mut (*bus).drv_pcie2); }
    else { bcma_core_pci_up(&mut (*bus).drv_pci[0]); }
}

/* See also pcicore_down */
unsafe fn bcma_host_pci_down(bus: *mut bcma_bus) {
    if (*bus).hosttype != BCMA_HOSTTYPE_PCI { return; }
    if !(*bus).host_is_pcie2 { bcma_core_pci_down(&mut (*bus).drv_pci[0]); }
}

unsafe fn bcma_host_pci_irq_ctl(bus: *mut bcma_bus, core: *mut bcma_device, enable: bool) -> i32 {
    if (*bus).hosttype != BCMA_HOSTTYPE_PCI { return 0; }
    let pdev = (*bus).host_pci;
    let mut tmp = 0u32;
    let mut err = pci_read_config_dword(pdev, BCMA_PCI_IRQMASK, &mut tmp);
    if err != 0 { return err; }
    let coremask = BIT((*core).core_index) << 8;
    if enable { tmp |= coremask; } else { tmp &= !coremask; }
    err = pci_write_config_dword(pdev, BCMA_PCI_IRQMASK, tmp);
    err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
