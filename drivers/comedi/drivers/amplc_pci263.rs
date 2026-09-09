// SPDX-License-Identifier: GPL-2.0+
/*
 * Driver for Amplicon PCI263 relay board.
 *
 * Copyright (C) 2002 MEV Ltd. <https://www.mev.co.uk/>
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 2000 David A. Schleef <ds@schleef.org>
 */

/*
 * Driver: amplc_pci263
 * Description: Amplicon PCI263
 * Author: Ian Abbott <abbotti@mev.co.uk>
 * Devices: [Amplicon] PCI263 (amplc_pci263)
 * Updated: Fri, 12 Apr 2013 15:19:36 +0100
 * Status: works
 *
 * Configuration options: not applicable, uses PCI auto config
 *
 * The board appears as one subdevice, with 16 digital outputs, each
 * connected to a reed-relay. Relay contacts are closed when output is 1.
 * The state of the outputs can be read.
 */

// Linux and Comedi dependencies are supplied by the surrounding translation unit.

/* PCI263 registers */
pub const PCI263_DO_0_7_REG: usize = 0x00;
pub const PCI263_DO_8_15_REG: usize = 0x01;

pub unsafe fn pci263_do_insn_bits(
    dev: *mut comedi_device,
    s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    if comedi_dio_update_state(s, data) != 0 {
        outb((*s).state & 0xff, (*dev).iobase + PCI263_DO_0_7_REG);
        outb(((*s).state >> 8) & 0xff, (*dev).iobase + PCI263_DO_8_15_REG);
    }

    *data.add(1) = (*s).state;

    (*insn).n as ::std::os::raw::c_int
}

pub unsafe fn pci263_auto_attach(
    dev: *mut comedi_device,
    _context_unused: ::std::os::raw::c_ulong,
) -> ::std::os::raw::c_int {
    let pci_dev: *mut pci_dev = comedi_to_pci_dev(dev);
    let mut s: *mut comedi_subdevice;
    let mut ret: ::std::os::raw::c_int;

    ret = comedi_pci_enable(dev);
    if ret != 0 {
        return ret;
    }

    (*dev).iobase = pci_resource_start(pci_dev, 2);
    ret = comedi_alloc_subdevices(dev, 1);
    if ret != 0 {
        return ret;
    }

    /* Digital Output subdevice */
    s = (*dev).subdevices;
    (*s).type_ = COMEDI_SUBD_DO;
    (*s).subdev_flags = SDF_WRITABLE;
    (*s).n_chan = 16;
    (*s).maxdata = 1;
    (*s).range_table = &range_digital;
    (*s).insn_bits = Some(pci263_do_insn_bits);

    /* read initial relay state */
    (*s).state = inb((*dev).iobase + PCI263_DO_0_7_REG)
        | (inb((*dev).iobase + PCI263_DO_8_15_REG) << 8);

    0
}

pub static mut amplc_pci263_driver: comedi_driver = comedi_driver {
    driver_name: b"amplc_pci263\0".as_ptr() as *const _,
    module: THIS_MODULE,
    auto_attach: Some(pci263_auto_attach),
    detach: Some(comedi_pci_detach),
};

// PCI_VDEVICE(AMPLICON, 0x000c), followed by the terminating empty entry.
pub static pci263_pci_table: [pci_device_id; 2] = [
    pci_device_id { vendor: PCI_VENDOR_ID_AMPLICON, device: 0x000c, ..pci_device_id::default() },
    pci_device_id::default(),
];

// MODULE_DEVICE_TABLE(pci, pci263_pci_table);

pub unsafe fn amplc_pci263_pci_probe(
    dev: *mut pci_dev,
    id: *const pci_device_id,
) -> ::std::os::raw::c_int {
    comedi_pci_auto_config(dev, &amplc_pci263_driver, (*id).driver_data)
}

pub static mut amplc_pci263_pci_driver: pci_driver = pci_driver {
    name: b"amplc_pci263\0".as_ptr() as *const _,
    id_table: pci263_pci_table.as_ptr(),
    probe: Some(amplc_pci263_pci_probe),
    remove: Some(comedi_pci_auto_unconfig),
};

// module_comedi_pci_driver(amplc_pci263_driver, amplc_pci263_pci_driver);

// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_DESCRIPTION("Comedi driver for Amplicon PCI263 relay board");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
