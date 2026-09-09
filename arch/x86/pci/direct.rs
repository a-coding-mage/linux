// SPDX-License-Identifier: GPL-2.0
/*
 * direct.c - Low-level direct PCI config space access
 */

/* Dependencies supplied by the surrounding kernel translation. */

const PCI_CONF1_ADDRESS: fn(u32, u32, u32) -> u32 = |bus, devfn, reg| {
    0x80000000 | ((reg & 0xF00) << 16) | (bus << 16) | (devfn << 8) | (reg & 0xFC)
};

unsafe fn pci_conf1_read(
    seg: u32,
    bus: u32,
    devfn: u32,
    reg: i32,
    len: i32,
    value: *mut u32,
) -> i32 {
    let mut flags: usize;

    if seg != 0 || bus > 255 || devfn > 255 || reg > 4095 {
        *value = u32::MAX;
        return -EINVAL;
    }

    raw_spin_lock_irqsave(&pci_config_lock, &mut flags);

    outl(PCI_CONF1_ADDRESS(bus, devfn, reg as u32), 0xCF8);

    match len {
        1 => *value = inb(0xCFC + (reg & 3) as u16) as u32,
        2 => *value = inw(0xCFC + (reg & 2) as u16) as u32,
        4 => *value = inl(0xCFC),
        _ => {}
    }

    raw_spin_unlock_irqrestore(&pci_config_lock, flags);
    0
}

unsafe fn pci_conf1_write(
    seg: u32,
    bus: u32,
    devfn: u32,
    reg: i32,
    len: i32,
    value: u32,
) -> i32 {
    let mut flags: usize;

    if seg != 0 || bus > 255 || devfn > 255 || reg > 4095 {
        return -EINVAL;
    }

    raw_spin_lock_irqsave(&pci_config_lock, &mut flags);
    outl(PCI_CONF1_ADDRESS(bus, devfn, reg as u32), 0xCF8);

    match len {
        1 => outb(value as u8, 0xCFC + (reg & 3) as u16),
        2 => outw(value as u16, 0xCFC + (reg & 2) as u16),
        4 => outl(value, 0xCFC),
        _ => {}
    }

    raw_spin_unlock_irqrestore(&pci_config_lock, flags);
    0
}

const pci_direct_conf1: pci_raw_ops = pci_raw_ops {
    read: pci_conf1_read,
    write: pci_conf1_write,
};

const PCI_CONF2_ADDRESS: fn(u32, u32) -> u16 = |dev, reg| (0xC000 | (dev << 8) | reg) as u16;

unsafe fn pci_conf2_read(
    seg: u32,
    bus: u32,
    devfn: u32,
    reg: i32,
    len: i32,
    value: *mut u32,
) -> i32 {
    let mut flags: usize;
    let dev: i32;
    let func: i32;

    WARN_ON(seg != 0);
    if bus > 255 || devfn > 255 || reg > 255 {
        *value = u32::MAX;
        return -EINVAL;
    }

    dev = PCI_SLOT(devfn) as i32;
    func = PCI_FUNC(devfn) as i32;
    if dev & 0x10 != 0 {
        return PCIBIOS_DEVICE_NOT_FOUND;
    }

    raw_spin_lock_irqsave(&pci_config_lock, &mut flags);
    outb((0xF0 | (func << 1)) as u8, 0xCF8);
    outb(bus as u8, 0xCFA);

    match len {
        1 => *value = inb(PCI_CONF2_ADDRESS(dev as u32, reg as u32)) as u32,
        2 => *value = inw(PCI_CONF2_ADDRESS(dev as u32, reg as u32)) as u32,
        4 => *value = inl(PCI_CONF2_ADDRESS(dev as u32, reg as u32)),
        _ => {}
    }

    outb(0, 0xCF8);
    raw_spin_unlock_irqrestore(&pci_config_lock, flags);
    0
}

unsafe fn pci_conf2_write(
    seg: u32,
    bus: u32,
    devfn: u32,
    reg: i32,
    len: i32,
    value: u32,
) -> i32 {
    let mut flags: usize;
    let dev: i32;
    let func: i32;

    WARN_ON(seg != 0);
    if bus > 255 || devfn > 255 || reg > 255 {
        return -EINVAL;
    }
    dev = PCI_SLOT(devfn) as i32;
    func = PCI_FUNC(devfn) as i32;
    if dev & 0x10 != 0 {
        return PCIBIOS_DEVICE_NOT_FOUND;
    }

    raw_spin_lock_irqsave(&pci_config_lock, &mut flags);
    outb((0xF0 | (func << 1)) as u8, 0xCF8);
    outb(bus as u8, 0xCFA);

    match len {
        1 => outb(value as u8, PCI_CONF2_ADDRESS(dev as u32, reg as u32)),
        2 => outw(value as u16, PCI_CONF2_ADDRESS(dev as u32, reg as u32)),
        4 => outl(value, PCI_CONF2_ADDRESS(dev as u32, reg as u32)),
        _ => {}
    }

    outb(0, 0xCF8);
    raw_spin_unlock_irqrestore(&pci_config_lock, flags);
    0
}

static pci_direct_conf2: pci_raw_ops = pci_raw_ops {
    read: pci_conf2_read,
    write: pci_conf2_write,
};

unsafe fn pci_sanity_check(o: *const pci_raw_ops) -> i32 {
    let mut x: u32 = 0;
    if pci_probe & PCI_NO_CHECKS != 0 { return 1; }
    if dmi_get_bios_year() >= 2001 { return 1; }
    for devfn in 0..0x100 {
        if ((*o).read)(0, 0, devfn, PCI_CLASS_DEVICE, 2, &mut x) != 0 { continue; }
        if x == PCI_CLASS_BRIDGE_HOST || x == PCI_CLASS_DISPLAY_VGA { return 1; }
        if ((*o).read)(0, 0, devfn, PCI_VENDOR_ID, 2, &mut x) != 0 { continue; }
        if x == PCI_VENDOR_ID_INTEL || x == PCI_VENDOR_ID_COMPAQ { return 1; }
    }
    DBG(KERN_WARNING, "PCI: Sanity check failed\n");
    0
}

unsafe fn pci_check_type1() -> i32 {
    let mut flags: usize;
    let mut tmp: u32;
    let mut works = 0;
    local_irq_save(&mut flags);
    outb(0x01, 0xCFB);
    tmp = inl(0xCF8);
    outl(0x80000000, 0xCF8);
    if inl(0xCF8) == 0x80000000 && pci_sanity_check(&pci_direct_conf1) != 0 { works = 1; }
    outl(tmp, 0xCF8);
    local_irq_restore(flags);
    works
}

unsafe fn pci_check_type2() -> i32 {
    let mut flags: usize;
    let mut works = 0;
    local_irq_save(&mut flags);
    outb(0x00, 0xCFB);
    outb(0x00, 0xCF8);
    outb(0x00, 0xCFA);
    if inb(0xCF8) == 0 && inb(0xCFA) == 0 && pci_sanity_check(&pci_direct_conf2) != 0 { works = 1; }
    local_irq_restore(flags);
    works
}

pub unsafe fn pci_direct_init(type_: i32) {
    if type_ == 0 { return; }
    printk!(KERN_INFO, "PCI: Using configuration type {} for base access\n", type_);
    if type_ == 1 {
        raw_pci_ops = &pci_direct_conf1;
        if raw_pci_ext_ops.is_null() == false { return; }
        if pci_probe & PCI_HAS_IO_ECS == 0 { return; }
        printk!(KERN_INFO, "PCI: Using configuration type 1 for extended access\n");
        raw_pci_ext_ops = &pci_direct_conf1;
        return;
    }
    raw_pci_ops = &pci_direct_conf2;
}

pub unsafe fn pci_direct_probe() -> i32 {
    if pci_probe & PCI_PROBE_CONF1 == 0 { return pci_direct_probe_type2(); }
    if request_region(0xCF8, 8, "PCI conf1") == 0 { return pci_direct_probe_type2(); }
    if pci_check_type1() != 0 {
        raw_pci_ops = &pci_direct_conf1;
        port_cf9_safe = true;
        return 1;
    }
    release_region(0xCF8, 8);
    pci_direct_probe_type2()
}

unsafe fn pci_direct_probe_type2() -> i32 {
    if pci_probe & PCI_PROBE_CONF2 == 0 { return 0; }
    if request_region(0xCF8, 4, "PCI conf2") == 0 { return 0; }
    if request_region(0xC000, 0x1000, "PCI conf2") == 0 {
        release_region(0xCF8, 4);
        return 0;
    }
    if pci_check_type2() != 0 {
        raw_pci_ops = &pci_direct_conf2;
        port_cf9_safe = true;
        return 2;
    }
    release_region(0xC000, 0x1000);
    release_region(0xCF8, 4);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
