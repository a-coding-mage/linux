// SPDX-License-Identifier: GPL-2.0+
/*
 * comedi/drivers/cb_pcimdda.c
 * Computer Boards PCIM-DDA06-16 Comedi driver
 * Author: Calin Culianu <calin@ajvar.org>
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 2000 David A. Schleef <ds@schleef.org>
 */
/*
 * Driver: cb_pcimdda
 * Description: Measurement Computing PCIM-DDA06-16
 * Devices: [Measurement Computing] PCIM-DDA06-16 (cb_pcimdda)
 * Author: Calin Culianu <calin@ajvar.org>
 * Updated: Mon, 14 Apr 2008 15:15:51 +0100
 * Status: works
 *
 * All features of the PCIM-DDA06-16 board are supported.
 * This board has 6 16-bit AO channels, and the usual 8255 DIO setup.
 * (24 channels, configurable in banks of 8 and 4, etc.).
 * This board does not support commands.
 *
 * The board has a peculiar way of specifying AO gain/range settings -- You have
 * 1 jumper bank on the card, which either makes all 6 AO channels either
 * 5 Volt unipolar, 5V bipolar, 10 Volt unipolar or 10V bipolar.
 *
 * Since there is absolutely _no_ way to tell in software how this jumper is set
 * (well, at least according to the rather thin spec. from Measurement Computing
 * that comes with the board), the driver assumes the jumper is at its factory
 * default setting of +/-5V.
 *
 * Also of note is the fact that this board features another jumper, whose
 * state is also completely invisible to software.  It toggles two possible AO
 * output modes on the board:
 *
 *   - Update Mode: Writing to an AO channel instantaneously updates the actual
 *     signal output by the DAC on the board (this is the factory default).
 *   - Simultaneous XFER Mode: Writing to an AO channel has no effect until you
 *     read from any one of the AO channels.  This is useful for loading
 *     all 6 AO values, and then reading from any one of the AO channels on the
 *     device to instantly update all 6 AO values in unison.  Useful for some
 *     control apps, I would assume? If your jumper is in this setting, then you
 *     need to issue your comedi_data_write()s to load all the values you want,
 *     then issue one comedi_data_read() on any channel on the AO subdevice
 *     to initiate the simultaneous XFER.
 *
 * Configuration Options: not applicable, uses PCI auto config
 */

// This driver is a direct Rust translation of the Linux Comedi PCI driver.
// Required Linux/Comedi symbols are supplied by other translation units.

pub const PCI_ID_PCIM_DDA06_16: u16 = 0x0053;
pub const PCIMDDA_8255_BASE_REG: usize = 0x0c;

#[inline]
pub const fn pcimdda_da_chan(x: usize) -> usize { 0x00 + x * 2 }

#[repr(C)]
pub struct comedi_device {
    pub iobase: usize,
    pub subdevices: *mut comedi_subdevice,
}
#[repr(C)]
pub struct comedi_subdevice {
    pub type_: u32,
    pub subdev_flags: u32,
    pub n_chan: u32,
    pub maxdata: u32,
    pub range_table: *mut core::ffi::c_void,
    pub readback: *mut u32,
    pub insn_write: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_subdevice, *mut comedi_insn, *mut u32) -> i32>,
    pub insn_read: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_subdevice, *mut comedi_insn, *mut u32) -> i32>,
}
#[repr(C)] pub struct comedi_insn { pub chanspec: u32, pub n: u32 }
#[repr(C)] pub struct pci_dev;
#[repr(C)] pub struct pci_device_id { pub driver_data: usize }
#[repr(C)] pub struct comedi_driver;
#[repr(C)] pub struct pci_driver;

extern "C" {
    fn outb(value: u8, port: usize);
    fn inw(port: usize) -> u16;
    fn comedi_to_pci_dev(dev: *mut comedi_device) -> *mut pci_dev;
    fn comedi_pci_enable(dev: *mut comedi_device) -> i32;
    fn pci_resource_start(dev: *mut pci_dev, bar: u32) -> usize;
    fn comedi_alloc_subdevices(dev: *mut comedi_device, n: u32) -> i32;
    fn comedi_alloc_subdev_readback(s: *mut comedi_subdevice) -> i32;
    fn subdev_8255_io_init(dev: *mut comedi_device, s: *mut comedi_subdevice, regbase: usize) -> i32;
    fn comedi_readback_insn_read(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32;
    fn comedi_pci_auto_config(dev: *mut pci_dev, driver: *mut comedi_driver, context: usize) -> i32;
    fn comedi_pci_detach(dev: *mut comedi_device) -> i32;
    fn comedi_pci_auto_unconfig(dev: *mut pci_dev) -> i32;
    static mut range_bipolar5: core::ffi::c_void;
}

pub const COMEDI_SUBD_AO: u32 = 1;
pub const SDF_WRITABLE: u32 = 1 << 0;
pub const SDF_READABLE: u32 = 1 << 1;

#[inline] unsafe fn cr_chan(chanspec: u32) -> usize { (chanspec & 0xff) as usize }

pub unsafe extern "C" fn cb_pcimdda_ao_insn_write(
    dev: *mut comedi_device, s: *mut comedi_subdevice,
    insn: *mut comedi_insn, data: *mut u32,
) -> i32 {
    let chan = cr_chan((*insn).chanspec);
    let offset = (*dev).iobase + pcimdda_da_chan(chan);
    let mut val = *(*s).readback.add(chan);
    for i in 0..(*insn).n as usize {
        val = *data.add(i);
        // Write the LSB then MSB. A read initiates simultaneous transfer mode.
        outb((val & 0x00ff) as u8, offset);
        outb(((val >> 8) & 0x00ff) as u8, offset + 1);
    }
    *(*s).readback.add(chan) = val;
    (*insn).n as i32
}

pub unsafe extern "C" fn cb_pcimdda_ao_insn_read(
    dev: *mut comedi_device, s: *mut comedi_subdevice,
    insn: *mut comedi_insn, data: *mut u32,
) -> i32 {
    let chan = cr_chan((*insn).chanspec);
    // Initiate the simultaneous transfer.
    let _ = inw((*dev).iobase + pcimdda_da_chan(chan));
    comedi_readback_insn_read(dev, s, insn, data)
}

pub unsafe extern "C" fn cb_pcimdda_auto_attach(dev: *mut comedi_device, _context_unused: usize) -> i32 {
    let pcidev = comedi_to_pci_dev(dev);
    let mut ret = comedi_pci_enable(dev);
    if ret != 0 { return ret; }
    (*dev).iobase = pci_resource_start(pcidev, 3);
    ret = comedi_alloc_subdevices(dev, 2);
    if ret != 0 { return ret; }
    let s = (*dev).subdevices;
    (*s).type_ = COMEDI_SUBD_AO;
    (*s).subdev_flags = SDF_WRITABLE | SDF_READABLE;
    (*s).n_chan = 6;
    (*s).maxdata = 0xffff;
    (*s).range_table = &mut range_bipolar5;
    (*s).insn_write = Some(cb_pcimdda_ao_insn_write);
    (*s).insn_read = Some(cb_pcimdda_ao_insn_read);
    ret = comedi_alloc_subdev_readback(s);
    if ret != 0 { return ret; }
    subdev_8255_io_init(dev, s.add(1), PCIMDDA_8255_BASE_REG)
}

// The C driver registration records and module metadata are retained as declarations;
// their concrete kernel layouts and registration macros are supplied by Comedi.
extern "C" {
    static mut cb_pcimdda_driver: comedi_driver;
    static mut cb_pcimdda_driver_pci_driver: pci_driver;
}

#[no_mangle]
pub unsafe extern "C" fn cb_pcimdda_pci_probe(dev: *mut pci_dev, id: *const pci_device_id) -> i32 {
    comedi_pci_auto_config(dev, &mut cb_pcimdda_driver, (*id).driver_data)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
