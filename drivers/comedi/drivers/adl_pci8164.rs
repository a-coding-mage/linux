// SPDX-License-Identifier: GPL-2.0+
/*
 * comedi/drivers/adl_pci8164.c
 *
 * Hardware comedi driver for PCI-8164 Adlink card
 * Copyright (C) 2004 Michel Lachine <mike@mikelachaine.ca>
 */

/*
 * Driver: adl_pci8164
 * Description: Driver for the Adlink PCI-8164 4 Axes Motion Control board
 * Devices: [ADLink] PCI-8164 (adl_pci8164)
 * Author: Michel Lachaine <mike@mikelachaine.ca>
 * Status: experimental
 * Updated: Mon, 14 Apr 2008 15:10:32 +0100
 *
 * Configuration Options: not applicable, uses PCI auto config
 */

// Linux kernel, module, and Comedi PCI dependencies are supplied externally.

#[inline]
const fn pci8164_axis(x: u32) -> u32 { x * 0x08 }

const PCI8164_CMD_MSTS_REG: usize = 0x00;
const PCI8164_OTP_SSTS_REG: usize = 0x02;
const PCI8164_BUF0_REG: usize = 0x04;
const PCI8164_BUF1_REG: usize = 0x06;

unsafe fn adl_pci8164_insn_read(
    dev: *mut comedi_device,
    s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut u32,
) -> i32 {
    let offset = (*s).private as usize;
    let chan = cr_chan((*insn).chanspec);
    let mut i: i32 = 0;

    while i < (*insn).n {
        *data.add(i as usize) = inw((*dev).iobase + pci8164_axis(chan) as usize + offset);
        i += 1;
    }

    (*insn).n
}

unsafe fn adl_pci8164_insn_write(
    dev: *mut comedi_device,
    s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut u32,
) -> i32 {
    let offset = (*s).private as usize;
    let chan = cr_chan((*insn).chanspec);
    let mut i: i32 = 0;

    while i < (*insn).n {
        outw(*data.add(i as usize), (*dev).iobase + pci8164_axis(chan) as usize + offset);
        i += 1;
    }

    (*insn).n
}

unsafe fn adl_pci8164_auto_attach(dev: *mut comedi_device, _context_unused: u64) -> i32 {
    let pcidev = comedi_to_pci_dev(dev);
    let mut ret: i32;

    ret = comedi_pci_enable(dev);
    if ret != 0 {
        return ret;
    }
    (*dev).iobase = pci_resource_start(pcidev, 2);

    ret = comedi_alloc_subdevices(dev, 4);
    if ret != 0 {
        return ret;
    }

    // read MSTS register / write CMD register for each axis (channel)
    let s = &mut (*dev).subdevices[0];
    s.type_ = COMEDI_SUBD_PROC;
    s.subdev_flags = SDF_READABLE | SDF_WRITABLE;
    s.n_chan = 4;
    s.maxdata = 0xffff;
    s.len_chanlist = 4;
    s.insn_read = Some(adl_pci8164_insn_read);
    s.insn_write = Some(adl_pci8164_insn_write);
    s.private = PCI8164_CMD_MSTS_REG as *mut core::ffi::c_void;

    // read SSTS register / write OTP register for each axis (channel)
    let s = &mut (*dev).subdevices[1];
    s.type_ = COMEDI_SUBD_PROC;
    s.subdev_flags = SDF_READABLE | SDF_WRITABLE;
    s.n_chan = 4;
    s.maxdata = 0xffff;
    s.len_chanlist = 4;
    s.insn_read = Some(adl_pci8164_insn_read);
    s.insn_write = Some(adl_pci8164_insn_write);
    s.private = PCI8164_OTP_SSTS_REG as *mut core::ffi::c_void;

    // read/write BUF0 register for each axis (channel)
    let s = &mut (*dev).subdevices[2];
    s.type_ = COMEDI_SUBD_PROC;
    s.subdev_flags = SDF_READABLE | SDF_WRITABLE;
    s.n_chan = 4;
    s.maxdata = 0xffff;
    s.len_chanlist = 4;
    s.insn_read = Some(adl_pci8164_insn_read);
    s.insn_write = Some(adl_pci8164_insn_write);
    s.private = PCI8164_BUF0_REG as *mut core::ffi::c_void;

    // read/write BUF1 register for each axis (channel)
    let s = &mut (*dev).subdevices[3];
    s.type_ = COMEDI_SUBD_PROC;
    s.subdev_flags = SDF_READABLE | SDF_WRITABLE;
    s.n_chan = 4;
    s.maxdata = 0xffff;
    s.len_chanlist = 4;
    s.insn_read = Some(adl_pci8164_insn_read);
    s.insn_write = Some(adl_pci8164_insn_write);
    s.private = PCI8164_BUF1_REG as *mut core::ffi::c_void;

    0
}

static mut adl_pci8164_driver: comedi_driver = comedi_driver {
    driver_name: "adl_pci8164", module: THIS_MODULE,
    auto_attach: Some(adl_pci8164_auto_attach), detach: Some(comedi_pci_detach),
};

unsafe fn adl_pci8164_pci_probe(dev: *mut pci_dev, id: *const pci_device_id) -> i32 {
    comedi_pci_auto_config(dev, &mut adl_pci8164_driver, (*id).driver_data)
}

// PCI_VDEVICE(ADLINK, 0x8164), followed by the terminating empty entry.
static adl_pci8164_pci_table: [pci_device_id; 2] = [
    pci_device_id { vendor: ADLINK, device: 0x8164, driver_data: 0 },
    pci_device_id::default(),
];

static mut adl_pci8164_pci_driver: pci_driver = pci_driver {
    name: "adl_pci8164", id_table: adl_pci8164_pci_table.as_ptr(),
    probe: Some(adl_pci8164_pci_probe), remove: Some(comedi_pci_auto_unconfig),
};

// module_comedi_pci_driver(adl_pci8164_driver, adl_pci8164_pci_driver);
// MODULE_DEVICE_TABLE(pci, adl_pci8164_pci_table);
// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_DESCRIPTION("Comedi low-level driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
