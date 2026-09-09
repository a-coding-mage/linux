// SPDX-License-Identifier: GPL-2.0+
/*
 * adl_pci6208.c
 * Comedi driver for ADLink 6208 series cards
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 2000 David A. Schleef <ds@schleef.org>
 */

/*
 * Driver: adl_pci6208
 * Description: ADLink PCI-6208/6216 Series Multi-channel Analog Output Cards
 * Devices: [ADLink] PCI-6208 (adl_pci6208), PCI-6216
 * Author: nsyeow <nsyeow@pd.jaring.my>
 * Updated: Wed, 11 Feb 2015 11:37:18 +0000
 * Status: untested
 *
 * Configuration Options: not applicable, uses PCI auto config
 *
 * All supported devices share the same PCI device ID and are treated as a
 * PCI-6216 with 16 analog output channels.  On a PCI-6208, the upper 8
 * channels exist in registers, but don't go to DAC chips.
 */

// Linux kernel/comedi headers are supplied by the surrounding translation unit.

/* PCI-6208/6216-GL register map */
const PCI6208_AO_STATUS: usize = 0x00;
const PCI6208_AO_STATUS_DATA_SEND: u32 = 1 << 0;
const PCI6208_DIO: usize = 0x40;
const PCI6208_DIO_DO_MASK: u32 = 0x0f;
const PCI6208_DIO_DO_SHIFT: u32 = 0;
const PCI6208_DIO_DI_MASK: u32 = 0xf0;
const PCI6208_DIO_DI_SHIFT: u32 = 4;

#[inline]
const fn pci6208_ao_control(x: usize) -> usize {
    0x00 + 2 * x
}

unsafe fn pci6208_ao_eoc(
    dev: *mut comedi_device,
    _s: *mut comedi_subdevice,
    _insn: *mut comedi_insn,
    _context: c_ulong,
) -> c_int {
    let status: u32 = inw((*dev).iobase + PCI6208_AO_STATUS);
    if (status & PCI6208_AO_STATUS_DATA_SEND) == 0 {
        return 0;
    }
    -EBUSY
}

unsafe fn pci6208_ao_insn_write(
    dev: *mut comedi_device,
    s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut u32,
) -> c_int {
    let chan: u32 = CR_CHAN((*insn).chanspec);
    let mut i: u32 = 0;

    while i < (*insn).n {
        let val = *data.add(i as usize);

        /* D/A transfer rate is 2.2us */
        let ret = comedi_timeout(dev, s, insn, Some(pci6208_ao_eoc), 0);
        if ret != 0 {
            return ret;
        }

        /* the hardware expects two's complement values */
        outw(
            comedi_offset_munge(s, val),
            (*dev).iobase + pci6208_ao_control(chan as usize),
        );

        *(*s).readback.add(chan as usize) = val;
        i += 1;
    }

    (*insn).n as c_int
}

unsafe fn pci6208_di_insn_bits(
    dev: *mut comedi_device,
    _s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut u32,
) -> c_int {
    let mut val: u32 = inw((*dev).iobase + PCI6208_DIO);
    val = (val & PCI6208_DIO_DI_MASK) >> PCI6208_DIO_DI_SHIFT;
    *data.add(1) = val;
    (*insn).n as c_int
}

unsafe fn pci6208_do_insn_bits(
    dev: *mut comedi_device,
    s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut u32,
) -> c_int {
    if comedi_dio_update_state(s, data) != 0 {
        outw((*s).state, (*dev).iobase + PCI6208_DIO);
    }
    *data.add(1) = (*s).state;
    (*insn).n as c_int
}

unsafe fn pci6208_auto_attach(dev: *mut comedi_device, _context_unused: c_ulong) -> c_int {
    let pcidev = comedi_to_pci_dev(dev);
    let mut s: *mut comedi_subdevice;
    let mut val: u32;

    let mut ret = comedi_pci_enable(dev);
    if ret != 0 { return ret; }
    (*dev).iobase = pci_resource_start(pcidev, 2);

    ret = comedi_alloc_subdevices(dev, 3);
    if ret != 0 { return ret; }

    s = (*dev).subdevices;
    /* analog output subdevice */
    (*s).type_ = COMEDI_SUBD_AO;
    (*s).subdev_flags = SDF_WRITABLE;
    (*s).n_chan = 16; /* Only 8 usable on PCI-6208 */
    (*s).maxdata = 0xffff;
    (*s).range_table = &range_bipolar10;
    (*s).insn_write = Some(pci6208_ao_insn_write);
    ret = comedi_alloc_subdev_readback(s);
    if ret != 0 { return ret; }

    s = (*dev).subdevices.add(1);
    /* digital input subdevice */
    (*s).type_ = COMEDI_SUBD_DI;
    (*s).subdev_flags = SDF_READABLE;
    (*s).n_chan = 4;
    (*s).maxdata = 1;
    (*s).range_table = &range_digital;
    (*s).insn_bits = Some(pci6208_di_insn_bits);

    s = (*dev).subdevices.add(2);
    /* digital output subdevice */
    (*s).type_ = COMEDI_SUBD_DO;
    (*s).subdev_flags = SDF_WRITABLE;
    (*s).n_chan = 4;
    (*s).maxdata = 1;
    (*s).range_table = &range_digital;
    (*s).insn_bits = Some(pci6208_do_insn_bits);

    /* Get the read back signals from the digital outputs
     * and save it as the initial state for the subdevice. */
    val = inw((*dev).iobase + PCI6208_DIO);
    val = (val & PCI6208_DIO_DO_MASK) >> PCI6208_DIO_DO_SHIFT;
    (*s).state = val;
    0
}

static mut adl_pci6208_driver: comedi_driver = comedi_driver {
    driver_name: "adl_pci6208",
    module: THIS_MODULE,
    auto_attach: Some(pci6208_auto_attach),
    detach: Some(comedi_pci_detach),
};

unsafe fn adl_pci6208_pci_probe(
    dev: *mut pci_dev,
    id: *const pci_device_id,
) -> c_int {
    comedi_pci_auto_config(dev, &mut adl_pci6208_driver, (*id).driver_data)
}

static adl_pci6208_pci_table: [pci_device_id; 3] = [
    PCI_VDEVICE!(ADLINK, 0x6208),
    PCI_VDEVICE_SUB!(PLX, PCI_DEVICE_ID_PLX_9050, 0x9999, 0x6208),
    pci_device_id::default(),
];

static mut adl_pci6208_pci_driver: pci_driver = pci_driver {
    name: "adl_pci6208",
    id_table: adl_pci6208_pci_table.as_ptr(),
    probe: Some(adl_pci6208_pci_probe),
    remove: Some(comedi_pci_auto_unconfig),
};

module_comedi_pci_driver!(adl_pci6208_driver, adl_pci6208_pci_driver);

MODULE_AUTHOR!("Comedi https://www.comedi.org");
MODULE_DESCRIPTION!("Comedi driver for ADLink 6208 series cards");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
