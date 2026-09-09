// SPDX-License-Identifier: GPL-2.0+
/*
 * comedi/drivers/contec_pci_dio.c
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 2000 David A. Schleef <ds@schleef.org>
 */

/*
 * Driver: contec_pci_dio
 * Description: Contec PIO1616L digital I/O board
 * Devices: [Contec] PIO1616L (contec_pci_dio)
 * Author: Stefano Rivoir <s.rivoir@gts.it>
 * Updated: Wed, 27 Jun 2007 13:00:06 +0100
 * Status: works
 *
 * Configuration Options: not applicable, uses comedi PCI auto config
 */

// Dependencies supplied by the surrounding kernel/Comedi translation.
use crate::*;

/* Register map */
pub const PIO1616L_DI_REG: usize = 0x00;
pub const PIO1616L_DO_REG: usize = 0x02;

unsafe fn contec_do_insn_bits(
    dev: *mut comedi_device,
    s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if comedi_dio_update_state(s, data) != 0 {
        outw((*s).state, (*dev).iobase.wrapping_add(PIO1616L_DO_REG));
    }

    *data.add(1) = (*s).state;

    (*insn).n
}

unsafe fn contec_di_insn_bits(
    dev: *mut comedi_device,
    _s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    *data.add(1) = inw((*dev).iobase.wrapping_add(PIO1616L_DI_REG));

    (*insn).n
}

unsafe fn contec_auto_attach(
    dev: *mut comedi_device,
    _context_unused: ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    let pcidev = comedi_to_pci_dev(dev);
    let mut s: *mut comedi_subdevice;
    let mut ret: ::core::ffi::c_int;

    ret = comedi_pci_enable(dev);
    if ret != 0 {
        return ret;
    }
    (*dev).iobase = pci_resource_start(pcidev, 0);

    ret = comedi_alloc_subdevices(dev, 2);
    if ret != 0 {
        return ret;
    }

    s = (*dev).subdevices;
    (*s).type_ = COMEDI_SUBD_DI;
    (*s).subdev_flags = SDF_READABLE;
    (*s).n_chan = 16;
    (*s).maxdata = 1;
    (*s).range_table = &range_digital;
    (*s).insn_bits = Some(contec_di_insn_bits);

    s = s.add(1);
    (*s).type_ = COMEDI_SUBD_DO;
    (*s).subdev_flags = SDF_WRITABLE;
    (*s).n_chan = 16;
    (*s).maxdata = 1;
    (*s).range_table = &range_digital;
    (*s).insn_bits = Some(contec_do_insn_bits);

    0
}

static mut contec_pci_dio_driver: comedi_driver = comedi_driver {
    driver_name: "contec_pci_dio",
    module: THIS_MODULE,
    auto_attach: Some(contec_auto_attach),
    detach: Some(comedi_pci_detach),
};

unsafe fn contec_pci_dio_pci_probe(
    dev: *mut pci_dev,
    id: *const pci_device_id,
) -> ::core::ffi::c_int {
    comedi_pci_auto_config(dev, &mut contec_pci_dio_driver, (*id).driver_data)
}

static contec_pci_dio_pci_table: [pci_device_id; 2] = [
    pci_device_id {
        vendor: PCI_VENDOR_ID_CONTEC,
        device: 0x8172,
        subvendor: PCI_ANY_ID,
        subdevice: PCI_ANY_ID,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    },
    pci_device_id::default_value(),
];

static mut contec_pci_dio_pci_driver: pci_driver = pci_driver {
    name: "contec_pci_dio",
    id_table: contec_pci_dio_pci_table.as_ptr(),
    probe: Some(contec_pci_dio_pci_probe),
    remove: Some(comedi_pci_auto_unconfig),
};

// Equivalent of module_comedi_pci_driver(contec_pci_dio_driver,
// contec_pci_dio_pci_driver);

// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_DESCRIPTION("Comedi low-level driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
