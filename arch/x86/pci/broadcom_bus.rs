// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Read address ranges from a Broadcom CNB20LE Host Bridge
 *
 * Copyright (c) 2010 Ira W. Snyder <iws@ovro.caltech.edu>
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn cnb20le_res(bus: u8, slot: u8, func: u8) {
    let info: *mut pci_root_info;
    let mut root_res: *mut pci_root_res;
    let mut res: resource;
    let mut word1: u16;
    let mut word2: u16;
    let fbus: u8;
    let lbus: u8;

    /* read the PCI bus numbers */
    fbus = read_pci_config_byte(bus, slot, func, 0x44);
    lbus = read_pci_config_byte(bus, slot, func, 0x45);
    info = alloc_pci_root_info(fbus, lbus, 0, 0);

    /*
     * Add the legacy IDE ports on bus 0
     *
     * These do not exist anywhere in the bridge registers, AFAICT. I do
     * not have the datasheet, so this is the best I can do.
     */
    if fbus == 0 {
        update_res(info, 0x01f0, 0x01f7, IORESOURCE_IO, 0);
        update_res(info, 0x03f6, 0x03f6, IORESOURCE_IO, 0);
        update_res(info, 0x0170, 0x0177, IORESOURCE_IO, 0);
        update_res(info, 0x0376, 0x0376, IORESOURCE_IO, 0);
        update_res(info, 0xffa0, 0xffaf, IORESOURCE_IO, 0);
    }

    /* read the non-prefetchable memory window */
    word1 = read_pci_config_16(bus, slot, func, 0xc0);
    word2 = read_pci_config_16(bus, slot, func, 0xc2);
    if word1 != word2 {
        res.start = ((word1 as resource_size_t) << 16) | 0x0000;
        res.end = ((word2 as resource_size_t) << 16) | 0xffff;
        res.flags = IORESOURCE_MEM;
        update_res(info, res.start, res.end, res.flags, 0);
    }

    /* read the prefetchable memory window */
    word1 = read_pci_config_16(bus, slot, func, 0xc4);
    word2 = read_pci_config_16(bus, slot, func, 0xc6);
    if word1 != word2 {
        res.start = ((word1 as resource_size_t) << 16) | 0x0000;
        res.end = ((word2 as resource_size_t) << 16) | 0xffff;
        res.flags = IORESOURCE_MEM | IORESOURCE_PREFETCH;
        update_res(info, res.start, res.end, res.flags, 0);
    }

    /* read the IO port window */
    word1 = read_pci_config_16(bus, slot, func, 0xd0);
    word2 = read_pci_config_16(bus, slot, func, 0xd2);
    if word1 != word2 {
        res.start = word1 as resource_size_t;
        res.end = word2 as resource_size_t;
        res.flags = IORESOURCE_IO;
        update_res(info, res.start, res.end, res.flags, 0);
    }

    /* print information about this host bridge */
    res.start = fbus as resource_size_t;
    res.end = lbus as resource_size_t;
    res.flags = IORESOURCE_BUS;
    printk!(KERN_INFO, "CNB20LE PCI Host Bridge (domain 0000 %pR)\n", &res);

    list_for_each_entry!(root_res, &(*info).resources, list) {
        printk!(KERN_INFO, "host bridge window %pR\n", &(*root_res).res);
    }
}

unsafe fn broadcom_postcore_init() -> i32 {
    let bus: u8 = 0;
    let slot: u8 = 0;
    let id: u32;
    let vendor: u16;
    let device: u16;

    /* CONFIG_ACPI conditional: retain the ACPI fallback behavior. */
    #[cfg(CONFIG_ACPI)]
    {
        /*
         * We should get host bridge information from ACPI unless the BIOS
         * doesn't support it.
         */
        if !acpi_disabled && acpi_os_get_root_pointer() != 0 {
            return 0;
        }
    }

    id = read_pci_config(bus, slot, 0, PCI_VENDOR_ID);
    vendor = (id & 0xffff) as u16;
    device = ((id >> 16) & 0xffff) as u16;

    if vendor == PCI_VENDOR_ID_SERVERWORKS &&
        device == PCI_DEVICE_ID_SERVERWORKS_LE
    {
        cnb20le_res(bus, slot, 0);
        cnb20le_res(bus, slot, 1);
    }
    0
}

postcore_initcall!(broadcom_postcore_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
