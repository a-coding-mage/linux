// SPDX-License-Identifier: GPL-2.0+
/*
 * adl_pci7250.c
 *
 * Comedi driver for ADLink PCI-7250 series cards.
 *
 * Copyright (C) 2015, 2025 Ian Abbott <abbotti@mev.co.uk>
 */

/*
 * Driver: adl_pci7250
 * Description: Driver for the ADLINK PCI-7250 relay output & digital input card
 * Devices: [ADLINK] PCI-7250 (adl_pci7250) LPCI-7250 LPCIe-7250
 * Author: Ian Abbott <abbotti@mev.co.uk>
 * Status: works
 * Updated: Mon, 02 Jun 2025 13:54:11 +0100
 *
 * The driver assumes that 3 PCI-7251 modules are fitted to the PCI-7250,
 * giving 32 channels of relay outputs and 32 channels of isolated digital
 * inputs.  That is also the case for the LPCI-7250 and older LPCIe-7250
 * cards although they do not physically support the PCI-7251 modules.
 * Newer LPCIe-7250 cards have a different PCI subsystem device ID, so
 * set the number of channels to 8 for these cards.
 *
 * Not fitting the PCI-7251 modules shouldn't do any harm, but the extra
 * inputs and relay outputs won't work!
 *
 * Configuration Options: not applicable, uses PCI auto config
 */

// Linux kernel and Comedi symbols supplied by external dependencies.

unsafe fn adl_pci7250_read8(dev: *mut comedi_device, offset: u32) -> u8 {
    // CONFIG_HAS_IOPORT is a build-time condition from the original source.
    #[cfg(CONFIG_HAS_IOPORT)]
    {
        if (*dev).mmio.is_null() {
            return inb((*dev).iobase.wrapping_add(offset));
        }
    }
    readb((*dev).mmio.add(offset as usize))
}

unsafe fn adl_pci7250_write8(dev: *mut comedi_device, offset: u32, val: u8) {
    // CONFIG_HAS_IOPORT is a build-time condition from the original source.
    #[cfg(CONFIG_HAS_IOPORT)]
    {
        if (*dev).mmio.is_null() {
            outb(val, (*dev).iobase.wrapping_add(offset));
            return;
        }
    }
    writeb(val, (*dev).mmio.add(offset as usize));
}

unsafe fn adl_pci7250_do_insn_bits(
    dev: *mut comedi_device,
    s: *mut comedi_subdevice,
    _insn: *mut comedi_insn,
    data: *mut u32,
) -> i32 {
    let mut mask = comedi_dio_update_state(s, data);

    if mask != 0 {
        let mut state = (*s).state;
        let mut i: u32 = 0;
        while i.wrapping_mul(8) < (*s).n_chan {
            if (mask & 0xffu32) != 0 {
                // write relay data to even offset registers
                adl_pci7250_write8(dev, i.wrapping_mul(2), (state & 0xffu32) as u8);
            }
            state >>= 8;
            mask >>= 8;
            i = i.wrapping_add(1);
        }
    }

    *data.add(1) = (*s).state;
    2
}

unsafe fn adl_pci7250_di_insn_bits(
    dev: *mut comedi_device,
    s: *mut comedi_subdevice,
    _insn: *mut comedi_insn,
    data: *mut u32,
) -> i32 {
    let mut value: u32 = 0;
    let mut i: u32 = 0;

    while i.wrapping_mul(8) < (*s).n_chan {
        // read DI value from odd offset registers
        value |= (adl_pci7250_read8(dev, i.wrapping_mul(2).wrapping_add(1)) as u32)
            << i.wrapping_mul(8);
        i = i.wrapping_add(1);
    }

    *data.add(1) = value;
    2
}

unsafe fn pci7250_auto_attach(dev: *mut comedi_device, _context_unused: u64) -> i32 {
    let pcidev = comedi_to_pci_dev(dev);
    let mut s: *mut comedi_subdevice;
    let mut max_chans: u32;
    let mut i: u32;
    let mut ret: i32;

    ret = comedi_pci_enable(dev);
    if ret != 0 {
        return ret;
    }

    if pci_resource_len(pcidev, 2) < 8 {
        return -ENXIO;
    }

    /*
     * Newer LPCIe-7250 boards use MMIO.  Older LPCIe-7250, LPCI-7250, and
     * PCI-7250 boards use Port I/O.
     */
    if (pci_resource_flags(pcidev, 2) & IORESOURCE_MEM) != 0 {
        (*dev).mmio = pci_ioremap_bar(pcidev, 2);
        if (*dev).mmio.is_null() {
            return -ENOMEM;
        }
    } else if IS_ENABLED!(CONFIG_HAS_IOPORT) {
        (*dev).iobase = pci_resource_start(pcidev, 2);
    } else {
        dev_err((*dev).class_dev, "error! need I/O port support\n");
        return -ENXIO;
    }

    if (*pcidev).subsystem_device == 0x7000 {
        /* This is a newer LPCIe-7250 variant and cannot possibly have
         * PCI-7251 modules fitted, so limit the number of channels to 8. */
        max_chans = 8;
    } else {
        /* It is unknown whether the board is a PCI-7250, an LPCI-7250, or
         * an older LPCIe-7250 variant, so treat it as a PCI-7250 and assume
         * it can have PCI-7251 modules fitted to increase the number of
         * channels to a maximum of 32. */
        max_chans = 32;
    }

    ret = comedi_alloc_subdevices(dev, 2);
    if ret != 0 {
        return ret;
    }

    /* Relay digital output. */
    s = (*dev).subdevices.add(0);
    (*s).type = COMEDI_SUBD_DO;
    (*s).subdev_flags = SDF_WRITABLE;
    (*s).n_chan = max_chans;
    (*s).maxdata = 1;
    (*s).range_table = &range_digital;
    (*s).insn_bits = Some(adl_pci7250_do_insn_bits);
    /* Read initial state of relays from the even offset registers. */
    (*s).state = 0;
    i = 0;
    while i.wrapping_mul(8) < max_chans {
        (*s).state |= (adl_pci7250_read8(dev, i.wrapping_mul(2)) as u32)
            << i.wrapping_mul(8);
        i = i.wrapping_add(1);
    }

    /* Isolated digital input. */
    s = (*dev).subdevices.add(1);
    (*s).type = COMEDI_SUBD_DI;
    (*s).subdev_flags = SDF_READABLE;
    (*s).n_chan = max_chans;
    (*s).maxdata = 1;
    (*s).range_table = &range_digital;
    (*s).insn_bits = Some(adl_pci7250_di_insn_bits);

    0
}

static mut adl_pci7250_driver: comedi_driver = comedi_driver {
    driver_name: "adl_pci7250",
    module: THIS_MODULE,
    auto_attach: Some(pci7250_auto_attach),
    detach: Some(comedi_pci_detach),
};

unsafe fn adl_pci7250_pci_probe(
    dev: *mut pci_dev,
    id: *const pci_device_id,
) -> i32 {
    comedi_pci_auto_config(dev, &mut adl_pci7250_driver, (*id).driver_data)
}

static adl_pci7250_pci_table: [pci_device_id; 5] = [
    // CONFIG_HAS_IOPORT entries from the original source are conditional.
    pci_device_id::vdevice_sub(PLX, PCI_DEVICE_ID_PLX_9050, 0x9999, 0x7250),
    pci_device_id::vdevice_sub(ADLINK, 0x7250, 0x9999, 0x7250),
    pci_device_id::vdevice_sub(ADLINK, 0x7250, PCI_VENDOR_ID_ADLINK, 0x7250),
    pci_device_id::vdevice_sub(ADLINK, 0x7250, PCI_VENDOR_ID_ADLINK, 0x7000),
    pci_device_id::default(),
];

static mut adl_pci7250_pci_driver: pci_driver = pci_driver {
    name: "adl_pci7250",
    id_table: adl_pci7250_pci_table.as_ptr(),
    probe: Some(adl_pci7250_pci_probe),
    remove: Some(comedi_pci_auto_unconfig),
};

// module_comedi_pci_driver(adl_pci7250_driver, adl_pci7250_pci_driver);
// MODULE_DEVICE_TABLE(pci, adl_pci7250_pci_table);
// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_DESCRIPTION("Comedi driver for ADLink PCI-7250 series boards");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
