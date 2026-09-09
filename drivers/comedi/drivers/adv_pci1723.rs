// SPDX-License-Identifier: GPL-2.0+
/*
 * adv_pci1723.c
 * Comedi driver for the Advantech PCI-1723 card.
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 2000 David A. Schleef <ds@schleef.org>
 */

/*
 * Driver: adv_pci1723
 * Description: Advantech PCI-1723
 * Author: yonggang <rsmgnu@gmail.com>, Ian Abbott <abbotti@mev.co.uk>
 * Devices: [Advantech] PCI-1723 (adv_pci1723)
 * Updated: Mon, 14 Apr 2008 15:12:56 +0100
 * Status: works
 *
 * Configuration Options: not applicable, uses comedi PCI auto config
 *
 * Subdevice 0 is 8-channel AO, 16-bit, range +/- 10 V.
 *
 * Subdevice 1 is 16-channel DIO.  The channels are configurable as
 * input or output in 2 groups (0 to 7, 8 to 15). Configuring any
 * channel implicitly configures all channels in the same group.
 *
 * TODO:
 * 1. Add the two milliamp ranges to the AO subdevice (0 to 20 mA,
 *    4 to 20 mA).
 * 2. Read the initial ranges and values of the AO subdevice at
 *    start-up instead of reinitializing them.
 * 3. Implement calibration.
 */

// Linux/Comedi headers provide the types, constants, functions, and globals
// referenced below.

pub const PCI1723_AO_REG: u32 = 0x00;
pub const PCI1723_BOARD_ID_REG: u32 = 0x10;
pub const PCI1723_BOARD_ID_MASK: u32 = 0xf << 0;
pub const PCI1723_SYNC_CTRL_REG: u32 = 0x12;
pub const PCI1723_SYNC_CTRL_ASYNC: u32 = 0;
pub const PCI1723_SYNC_CTRL_SYNC: u32 = 1;
pub const PCI1723_CTRL_REG: u32 = 0x14;
pub const PCI1723_CTRL_BUSY: u32 = 1 << 15;
pub const PCI1723_CTRL_INIT: u32 = 1 << 14;
pub const PCI1723_CTRL_SELF: u32 = 1 << 8;
pub const PCI1723_CALIB_CTRL_REG: u32 = 0x16;
pub const PCI1723_CALIB_CTRL_CS: u32 = 1 << 2;
pub const PCI1723_CALIB_CTRL_DAT: u32 = 1 << 1;
pub const PCI1723_CALIB_CTRL_CLK: u32 = 1 << 0;
pub const PCI1723_CALIB_STROBE_REG: u32 = 0x18;
pub const PCI1723_DIO_CTRL_REG: u32 = 0x1a;
pub const PCI1723_DIO_CTRL_HDIO: u32 = 1 << 1;
pub const PCI1723_DIO_CTRL_LDIO: u32 = 1 << 0;
pub const PCI1723_DIO_DATA_REG: u32 = 0x1c;
pub const PCI1723_CALIB_DATA_REG: u32 = 0x1e;
pub const PCI1723_SYNC_STROBE_REG: u32 = 0x20;
pub const PCI1723_RESET_AO_STROBE_REG: u32 = 0x22;
pub const PCI1723_RESET_CALIB_STROBE_REG: u32 = 0x24;
pub const PCI1723_RANGE_STROBE_REG: u32 = 0x26;
pub const PCI1723_VREF_REG: u32 = 0x28;
pub const PCI1723_VREF_NEG10V: u32 = 0;
pub const PCI1723_VREF_0V: u32 = 1;
pub const PCI1723_VREF_POS10V: u32 = 3;

#[inline]
pub const fn pci1723_ao_reg(x: u32) -> u32 { x * 2 }
#[inline]
pub const fn pci1723_sync_ctrl(x: u32) -> u32 { x & 1 }
#[inline]
pub const fn pci1723_ctrl_idx(x: u32) -> u32 { (x & 3) << 6 }
#[inline]
pub const fn pci1723_ctrl_range(x: u32) -> u32 { (x & 3) << 4 }
#[inline]
pub const fn pci1723_ctrl_sel(x: u32) -> u32 { (x & 1) << 3 }
#[inline]
pub const fn pci1723_ctrl_chan(x: u32) -> u32 { x & 7 }
#[inline]
pub const fn pci1723_vref(x: u32) -> u32 { x & 3 }
pub const PCI1723_CTRL_GAIN: u32 = pci1723_ctrl_sel(0);
pub const PCI1723_CTRL_OFFSET: u32 = pci1723_ctrl_sel(1);

unsafe fn pci1723_ao_insn_write(
    dev: *mut comedi_device, s: *mut comedi_subdevice,
    insn: *mut comedi_insn, data: *mut u32,
) -> i32 {
    let chan = unsafe { CR_CHAN((*insn).chanspec) };
    for i in 0..(*insn).n {
        let val = *data.add(i as usize);
        outw(val as u16, (*dev).iobase + pci1723_ao_reg(chan));
        (*s).readback.add(chan as usize).write(val);
    }
    (*insn).n as i32
}

unsafe fn pci1723_dio_insn_config(
    dev: *mut comedi_device, s: *mut comedi_subdevice,
    insn: *mut comedi_insn, data: *mut u32,
) -> i32 {
    let chan = CR_CHAN((*insn).chanspec);
    let mask = if chan < 8 { 0x00ff } else { 0xff00 };
    let mut mode: u16 = 0;
    let ret = comedi_dio_insn_config(dev, s, insn, data, mask);
    if ret != 0 { return ret; }
    if ((*s).io_bits & 0x00ff) == 0 { mode |= PCI1723_DIO_CTRL_LDIO as u16; }
    if ((*s).io_bits & 0xff00) == 0 { mode |= PCI1723_DIO_CTRL_HDIO as u16; }
    outw(mode, (*dev).iobase + PCI1723_DIO_CTRL_REG);
    (*insn).n as i32
}

unsafe fn pci1723_dio_insn_bits(
    dev: *mut comedi_device, s: *mut comedi_subdevice,
    insn: *mut comedi_insn, data: *mut u32,
) -> i32 {
    if comedi_dio_update_state(s, data) != 0 {
        outw((*s).state as u16, (*dev).iobase + PCI1723_DIO_DATA_REG);
    }
    *data.add(1) = inw((*dev).iobase + PCI1723_DIO_DATA_REG) as u32;
    (*insn).n as i32
}

unsafe fn pci1723_auto_attach(dev: *mut comedi_device, _context_unused: usize) -> i32 {
    let pcidev = comedi_to_pci_dev(dev);
    let mut ret = comedi_pci_enable(dev);
    if ret != 0 { return ret; }
    (*dev).iobase = pci_resource_start(pcidev, 2);
    ret = comedi_alloc_subdevices(dev, 2);
    if ret != 0 { return ret; }

    let s = (*dev).subdevices;
    (*s).type_ = COMEDI_SUBD_AO;
    (*s).subdev_flags = SDF_WRITABLE | SDF_GROUND | SDF_COMMON;
    (*s).n_chan = 8;
    (*s).maxdata = 0xffff;
    (*s).range_table = &range_bipolar10;
    (*s).insn_write = Some(pci1723_ao_insn_write);
    ret = comedi_alloc_subdev_readback(s);
    if ret != 0 { return ret; }

    outw(PCI1723_SYNC_CTRL_SYNC as u16, (*dev).iobase + PCI1723_SYNC_CTRL_REG);
    for i in 0..(*s).n_chan {
        outw((pci1723_ctrl_range(0) | pci1723_ctrl_chan(i)) as u16, (*dev).iobase + PCI1723_CTRL_REG);
        outw(0, (*dev).iobase + PCI1723_RANGE_STROBE_REG);
        outw(0x8000, (*dev).iobase + pci1723_ao_reg(i));
        (*s).readback.add(i as usize).write(0x8000);
    }
    outw(0, (*dev).iobase + PCI1723_SYNC_STROBE_REG);
    outw(PCI1723_SYNC_CTRL_ASYNC as u16, (*dev).iobase + PCI1723_SYNC_CTRL_REG);

    let s = s.add(1);
    (*s).type_ = COMEDI_SUBD_DIO;
    (*s).subdev_flags = SDF_READABLE | SDF_WRITABLE;
    (*s).n_chan = 16;
    (*s).maxdata = 1;
    (*s).range_table = &range_digital;
    (*s).insn_config = Some(pci1723_dio_insn_config);
    (*s).insn_bits = Some(pci1723_dio_insn_bits);
    let val = inw((*dev).iobase + PCI1723_DIO_CTRL_REG);
    if val & PCI1723_DIO_CTRL_LDIO as u16 == 0 { (*s).io_bits |= 0x00ff; }
    if val & PCI1723_DIO_CTRL_HDIO as u16 == 0 { (*s).io_bits |= 0xff00; }
    (*s).state = inw((*dev).iobase + PCI1723_DIO_DATA_REG) as u32;
    0
}

static mut adv_pci1723_driver: comedi_driver = comedi_driver {
    driver_name: "adv_pci1723", module: THIS_MODULE, auto_attach: Some(pci1723_auto_attach),
    detach: Some(comedi_pci_detach),
};

unsafe fn adv_pci1723_pci_probe(dev: *mut pci_dev, id: *const pci_device_id) -> i32 {
    comedi_pci_auto_config(dev, &mut adv_pci1723_driver, (*id).driver_data)
}

static adv_pci1723_pci_table: [pci_device_id; 2] = [
    pci_device_id { vendor: PCI_VENDOR_ID_ADVANTECH, device: 0x1723, ..pci_device_id::default() },
    pci_device_id::default(),
];

static mut adv_pci1723_pci_driver: pci_driver = pci_driver {
    name: "adv_pci1723", id_table: adv_pci1723_pci_table.as_ptr(),
    probe: Some(adv_pci1723_pci_probe), remove: Some(comedi_pci_auto_unconfig),
};

// Equivalent to module_comedi_pci_driver(adv_pci1723_driver, adv_pci1723_pci_driver).
// MODULE_DEVICE_TABLE(pci, adv_pci1723_pci_table);
// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_DESCRIPTION("Advantech PCI-1723 Comedi driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
