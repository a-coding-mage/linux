// SPDX-License-Identifier: GPL-2.0+
/*
 * COMEDI driver for Advantech PCI-1720U
 * Copyright (c) 2015 H Hartley Sweeten <hsweeten@visionengravers.com>
 *
 * Separated from the adv_pci1710 driver written by:
 * Michal Dobes <dobes@tesnet.cz>
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 2000 David A. Schleef <ds@schleef.org>
 */

/*
 * Driver: adv_pci1720
 * Description: 4-channel Isolated D/A Output board
 * Devices: [Advantech] PCI-7120U (adv_pci1720)
 * Author: H Hartley Sweeten <hsweeten@visionengravers.com>
 * Updated: Fri, 29 Oct 2015 17:19:35 -0700
 * Status: untested
 *
 * Configuration options: not applicable, uses PCI auto config
 *
 * The PCI-1720 has 4 isolated 12-bit analog output channels with multiple
 * output ranges. It also has a BoardID switch to allow differentiating
 * multiple boards in the system.
 *
 * The analog outputs can operate in two modes, immediate and synchronized.
 * This driver currently does not support the synchronized output mode.
 *
 * Jumpers JP1 to JP4 are used to set the current sink ranges for each
 * analog output channel. In order to use the current sink ranges, the
 * unipolar 5V range must be used. The voltage output and sink output for
 * each channel is available on the connector as separate pins.
 *
 * Jumper JP5 controls the "hot" reset state of the analog outputs.
 * Depending on its setting, the analog outputs will either keep the
 * last settings and output values or reset to the default state after
 * a "hot" reset. The default state for all channels is uniploar 5V range
 * and all the output values are 0V. To allow this feature to work, the
 * analog outputs are not "reset" when the driver attaches.
 */

// Linux kernel/comedi headers are supplied by the surrounding translation unit.

macro_rules! PCI1720_AO_LSB_REG { ($x:expr) => { 0x00 + (($x) * 2) }; }
macro_rules! PCI1720_AO_MSB_REG { ($x:expr) => { 0x01 + (($x) * 2) }; }
const PCI1720_AO_RANGE_REG: usize = 0x08;
macro_rules! PCI1720_AO_RANGE { ($c:expr, $r:expr) => { (($r & 0x3) << (($c) * 2)) }; }
macro_rules! PCI1720_AO_RANGE_MASK { ($c:expr) => { PCI1720_AO_RANGE!($c, 0x3) }; }
const PCI1720_SYNC_REG: usize = 0x09;
const PCI1720_SYNC_CTRL_REG: usize = 0x0f;
const PCI1720_SYNC_CTRL_SC0: usize = 1 << 0;
const PCI1720_BOARDID_REG: usize = 0x14;

static pci1720_ao_range: comedi_lrange = comedi_lrange {
    length: 4,
    range: [UNI_RANGE!(5), UNI_RANGE!(10), BIP_RANGE!(5), BIP_RANGE!(10)],
};

unsafe fn pci1720_ao_insn_write(
    dev: *mut comedi_device,
    s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut u32,
) -> i32 {
    let chan: u32 = CR_CHAN!((*insn).chanspec);
    let range: u32 = CR_RANGE!((*insn).chanspec);
    let mut val: u32;
    let mut i: u32;

    /* set the channel range and polarity */
    val = inb((*dev).iobase + PCI1720_AO_RANGE_REG as u64) as u32;
    val &= !(PCI1720_AO_RANGE_MASK!(chan));
    val |= PCI1720_AO_RANGE!(chan, range);
    outb(val as u8, (*dev).iobase + PCI1720_AO_RANGE_REG as u64);

    val = (*s).readback[chan as usize];
    i = 0;
    while i < (*insn).n {
        val = *data.add(i as usize);

        outb((val & 0xff) as u8, (*dev).iobase + PCI1720_AO_LSB_REG!(chan) as u64);
        outb(((val >> 8) & 0xff) as u8, (*dev).iobase + PCI1720_AO_MSB_REG!(chan) as u64);

        /* conversion time is 2us (500 kHz throughput) */
        usleep_range(2, 100);
        i += 1;
    }

    (*s).readback[chan as usize] = val;

    (*insn).n as i32
}

unsafe fn pci1720_di_insn_bits(
    dev: *mut comedi_device,
    _s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut u32,
) -> i32 {
    *data.add(1) = inb((*dev).iobase + PCI1720_BOARDID_REG as u64) as u32;
    (*insn).n as i32
}

unsafe fn pci1720_auto_attach(dev: *mut comedi_device, _context: u64) -> i32 {
    let pcidev = comedi_to_pci_dev(dev);
    let mut s: *mut comedi_subdevice;
    let mut ret: i32;

    ret = comedi_pci_enable(dev);
    if ret != 0 { return ret; }
    (*dev).iobase = pci_resource_start(pcidev, 2);

    ret = comedi_alloc_subdevices(dev, 2);
    if ret != 0 { return ret; }

    /* Analog Output subdevice */
    s = (*dev).subdevices.add(0);
    (*s).type_ = COMEDI_SUBD_AO;
    (*s).subdev_flags = SDF_WRITABLE;
    (*s).n_chan = 4;
    (*s).maxdata = 0x0fff;
    (*s).range_table = &pci1720_ao_range;
    (*s).insn_write = Some(pci1720_ao_insn_write);

    ret = comedi_alloc_subdev_readback(s);
    if ret != 0 { return ret; }

    /* Digital Input subdevice (BoardID SW1) */
    s = (*dev).subdevices.add(1);
    (*s).type_ = COMEDI_SUBD_DI;
    (*s).subdev_flags = SDF_READABLE;
    (*s).n_chan = 4;
    (*s).maxdata = 1;
    (*s).range_table = &range_digital;
    (*s).insn_bits = Some(pci1720_di_insn_bits);

    /* disable synchronized output, channels update when written */
    outb(0, (*dev).iobase + PCI1720_SYNC_CTRL_REG as u64);

    0
}

static mut adv_pci1720_driver: comedi_driver = comedi_driver {
    driver_name: "adv_pci1720",
    module: THIS_MODULE,
    auto_attach: Some(pci1720_auto_attach),
    detach: Some(comedi_pci_detach),
};

unsafe fn adv_pci1720_pci_probe(dev: *mut pci_dev, id: *const pci_device_id) -> i32 {
    comedi_pci_auto_config(dev, &mut adv_pci1720_driver, (*id).driver_data)
}

static adv_pci1720_pci_table: [pci_device_id; 2] = [
    PCI_VDEVICE!(ADVANTECH, 0x1720),
    pci_device_id::default(),
];

static mut adv_pci1720_pci_driver: pci_driver = pci_driver {
    name: "adv_pci1720",
    id_table: adv_pci1720_pci_table.as_ptr(),
    probe: Some(adv_pci1720_pci_probe),
    remove: Some(comedi_pci_auto_unconfig),
};

module_comedi_pci_driver!(adv_pci1720_driver, adv_pci1720_pci_driver);

MODULE_AUTHOR!("H Hartley Sweeten <hsweeten@visionengravers.com>");
MODULE_DESCRIPTION!("Comedi driver for Advantech PCI-1720 Analog Output board");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
