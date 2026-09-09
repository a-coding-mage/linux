// SPDX-License-Identifier: GPL-2.0+
/*
 * adv_pci1724.c
 * Comedi driver for the Advantech PCI-1724U card.
 *
 * Author:  Frank Mori Hess <fmh6jj@gmail.com>
 * Copyright (C) 2013 GnuBIO Inc
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1997-8 David A. Schleef <ds@schleef.org>
 */

/*
 * Driver: adv_pci1724
 * Description: Advantech PCI-1724U
 * Devices: [Advantech] PCI-1724U (adv_pci1724)
 * Author: Frank Mori Hess <fmh6jj@gmail.com>
 * Updated: 2013-02-09
 * Status: works
 *
 * Configuration Options: not applicable, uses comedi PCI auto config
 *
 * Subdevice 0 is the analog output.
 * Subdevice 1 is the offset calibration for the analog output.
 * Subdevice 2 is the gain calibration for the analog output.
 *
 * The calibration offset and gains have quite a large effect on the
 * analog output, so it is possible to adjust the analog output to
 * have an output range significantly different from the board's
 * nominal output ranges. For a calibrated +/-10V range, the analog
 * output's offset will be set somewhere near mid-range (0x2000) and
 * its gain will be near maximum (0x3fff).
 *
 * There is really no difference between the board's documented 0-20mA
 * versus 4-20mA output ranges. To pick one or the other is simply a
 * matter of adjusting the offset and gain calibration until the board
 * outputs in the desired range.
 */

// Dependencies supplied by the surrounding Comedi/Linux bindings.

pub const PCI1724_DAC_CTRL_REG: u32 = 0x00;
#[inline] pub const fn PCI1724_DAC_CTRL_GX(x: u32) -> u32 { 1u32 << (20 + (x / 8)) }
#[inline] pub const fn PCI1724_DAC_CTRL_CX(x: u32) -> u32 { (x % 8) << 16 }
#[inline] pub const fn PCI1724_DAC_CTRL_MODE(x: u32) -> u32 { (x & 0x3) << 14 }
pub const PCI1724_DAC_CTRL_MODE_GAIN: u32 = PCI1724_DAC_CTRL_MODE(1);
pub const PCI1724_DAC_CTRL_MODE_OFFSET: u32 = PCI1724_DAC_CTRL_MODE(2);
pub const PCI1724_DAC_CTRL_MODE_NORMAL: u32 = PCI1724_DAC_CTRL_MODE(3);
pub const PCI1724_DAC_CTRL_MODE_MASK: u32 = PCI1724_DAC_CTRL_MODE(3);
#[inline] pub const fn PCI1724_DAC_CTRL_DATA(x: u32) -> u32 { (x & 0x3fff) << 0 }
pub const PCI1724_SYNC_CTRL_REG: u32 = 0x04;
pub const PCI1724_SYNC_CTRL_DACSTAT: u32 = 1 << 1;
pub const PCI1724_SYNC_CTRL_SYN: u32 = 1 << 0;
pub const PCI1724_EEPROM_CTRL_REG: u32 = 0x08;
pub const PCI1724_SYNC_TRIG_REG: u32 = 0x0c; // any value works
pub const PCI1724_BOARD_ID_REG: u32 = 0x10;
pub const PCI1724_BOARD_ID_MASK: u32 = 0xf << 0;

static adv_pci1724_ao_ranges: comedi_lrange = comedi_lrange {
    length: 4,
    range: [BIP_RANGE(10), RANGE_mA(0, 20), RANGE_mA(4, 20), RANGE_unitless(0, 1)],
};

unsafe fn adv_pci1724_dac_idle(
    dev: *mut comedi_device, _s: *mut comedi_subdevice,
    _insn: *mut comedi_insn, _context: u64,
) -> i32 {
    let status = inl((*dev).iobase + PCI1724_SYNC_CTRL_REG);
    if (status & PCI1724_SYNC_CTRL_DACSTAT) == 0 { 0 } else { -EBUSY }
}

unsafe fn adv_pci1724_insn_write(
    dev: *mut comedi_device, s: *mut comedi_subdevice,
    insn: *mut comedi_insn, data: *mut u32,
) -> i32 {
    let mode = (*s).private as usize as u32;
    let chan = CR_CHAN((*insn).chanspec);
    let ctrl = PCI1724_DAC_CTRL_GX(chan) | PCI1724_DAC_CTRL_CX(chan) | mode;
    outl(0, (*dev).iobase + PCI1724_SYNC_CTRL_REG);
    for i in 0..(*insn).n {
        let val = *data.add(i as usize);
        let ret = comedi_timeout(dev, s, insn, Some(adv_pci1724_dac_idle), 0);
        if ret != 0 { return ret; }
        outl(ctrl | PCI1724_DAC_CTRL_DATA(val), (*dev).iobase + PCI1724_DAC_CTRL_REG);
        *(*s).readback.add(chan as usize) = val;
    }
    (*insn).n as i32
}

unsafe fn adv_pci1724_auto_attach(dev: *mut comedi_device, _context_unused: u64) -> i32 {
    let pcidev = comedi_to_pci_dev(dev);
    let mut s: *mut comedi_subdevice;
    let board_id: u32;
    let mut ret = comedi_pci_enable(dev);
    if ret != 0 { return ret; }
    (*dev).iobase = pci_resource_start(pcidev, 2);
    board_id = inl((*dev).iobase + PCI1724_BOARD_ID_REG);
    dev_info((*dev).class_dev, "board id: %d\n", board_id & PCI1724_BOARD_ID_MASK);
    ret = comedi_alloc_subdevices(dev, 3);
    if ret != 0 { return ret; }

    s = (*dev).subdevices.add(0);
    (*s).type_ = COMEDI_SUBD_AO; (*s).subdev_flags = SDF_READABLE | SDF_WRITABLE | SDF_GROUND;
    (*s).n_chan = 32; (*s).maxdata = 0x3fff; (*s).range_table = &adv_pci1724_ao_ranges;
    (*s).insn_write = Some(adv_pci1724_insn_write); (*s).private = PCI1724_DAC_CTRL_MODE_NORMAL as usize as *mut _;
    ret = comedi_alloc_subdev_readback(s); if ret != 0 { return ret; }

    s = (*dev).subdevices.add(1);
    (*s).type_ = COMEDI_SUBD_CALIB; (*s).subdev_flags = SDF_READABLE | SDF_WRITABLE | SDF_INTERNAL;
    (*s).n_chan = 32; (*s).maxdata = 0x3fff; (*s).insn_write = Some(adv_pci1724_insn_write);
    (*s).private = PCI1724_DAC_CTRL_MODE_OFFSET as usize as *mut _;
    ret = comedi_alloc_subdev_readback(s); if ret != 0 { return ret; }

    s = (*dev).subdevices.add(2);
    (*s).type_ = COMEDI_SUBD_CALIB; (*s).subdev_flags = SDF_READABLE | SDF_WRITABLE | SDF_INTERNAL;
    (*s).n_chan = 32; (*s).maxdata = 0x3fff; (*s).insn_write = Some(adv_pci1724_insn_write);
    (*s).private = PCI1724_DAC_CTRL_MODE_GAIN as usize as *mut _;
    comedi_alloc_subdev_readback(s)
}

static mut adv_pci1724_driver: comedi_driver = comedi_driver {
    driver_name: "adv_pci1724", module: THIS_MODULE, auto_attach: Some(adv_pci1724_auto_attach), detach: Some(comedi_pci_detach),
};

unsafe fn adv_pci1724_pci_probe(dev: *mut pci_dev, id: *const pci_device_id) -> i32 {
    comedi_pci_auto_config(dev, &mut adv_pci1724_driver, (*id).driver_data)
}

static adv_pci1724_pci_table: [pci_device_id; 2] = [
    PCI_VDEVICE(ADVANTECH, 0x1724), pci_device_id::default(),
];

static mut adv_pci1724_pci_driver: pci_driver = pci_driver {
    name: "adv_pci1724", id_table: adv_pci1724_pci_table.as_ptr(),
    probe: Some(adv_pci1724_pci_probe), remove: Some(comedi_pci_auto_unconfig),
};

// module_comedi_pci_driver(adv_pci1724_driver, adv_pci1724_pci_driver);
// MODULE_DEVICE_TABLE(pci, adv_pci1724_pci_table);
// MODULE_AUTHOR("Frank Mori Hess <fmh6jj@gmail.com>");
// MODULE_DESCRIPTION("Advantech PCI-1724U Comedi driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
