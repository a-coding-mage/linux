// SPDX-License-Identifier: GPL-2.0+
/*
 *     comedi/drivers/ni_daq_700.c
 *     Driver for DAQCard-700 DIO/AI
 *     copied from 8255
 *
 *     COMEDI - Linux Control and Measurement Device Interface
 *     Copyright (C) 1998 David A. Schleef <ds@schleef.org>
 */

// Driver: ni_daq_700
// Description: National Instruments PCMCIA DAQCard-700
// Author: Fred Brooks <nsaspook@nsaspook.com>,
//   based on ni_daq_dio24 by Daniel Vecino Castel <dvecino@able.es>
// Devices: [National Instruments] PCMCIA DAQ-Card-700 (ni_daq_700)
// Status: works
// Updated: Wed, 21 May 2014 12:07:20 +0000

/* External Linux/Comedi symbols and types are supplied by the surrounding crate. */

const DIO_W: usize = 0x04;
const DIO_R: usize = 0x05;
const CMD_R1: usize = 0x00;
const CMD_R2: usize = 0x07;
const CMD_R3: usize = 0x05;
const STA_R1: usize = 0x00;
const STA_R2: usize = 0x01;
const ADFIFO_R: usize = 0x02;
const ADCLEAR_R: usize = 0x01;
const CDA_R0: usize = 0x08;
const CDA_R1: usize = 0x09;
const CDA_R2: usize = 0x0a;
const CMO_R: usize = 0x0b;
const TIC_R: usize = 0x06;
const CMD_R3_DIFF: u32 = 0x04;

static range_daq700_ai: comedi_lrange = comedi_lrange {
    length: 3,
    range: [BIP_RANGE!(10), BIP_RANGE!(5), BIP_RANGE!(2.5)],
};

unsafe fn daq700_dio_insn_bits(
    dev: *mut comedi_device,
    s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut u32,
) -> i32 {
    let mask = comedi_dio_update_state(s, data);
    if mask != 0 && (mask & 0xff) != 0 {
        outb((*s).state & 0xff, (*dev).iobase + DIO_W as u64);
    }
    let mut val = (*s).state & 0xff;
    val |= (inb((*dev).iobase + DIO_R as u64) as u32) << 8;
    *data.add(1) = val;
    (*insn).n
}

unsafe fn daq700_dio_insn_config(
    dev: *mut comedi_device,
    s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut u32,
) -> i32 {
    let ret = comedi_dio_insn_config(dev, s, insn, data, 0);
    if ret != 0 { return ret; }
    (*s).io_bits = 0x00ff;
    (*insn).n
}

unsafe fn daq700_ai_eoc(
    dev: *mut comedi_device,
    _s: *mut comedi_subdevice,
    _insn: *mut comedi_insn,
    _context: u64,
) -> i32 {
    let mut status = inb((*dev).iobase + STA_R2 as u64);
    if (status & 0x03) != 0 { return -EOVERFLOW; }
    status = inb((*dev).iobase + STA_R1 as u64);
    if (status & 0x02) != 0 { return -ENODATA; }
    if (status & 0x11) == 0x01 { return 0; }
    -EBUSY
}

unsafe fn daq700_ai_rinsn(
    dev: *mut comedi_device,
    s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut u32,
) -> i32 {
    let chan = CR_CHAN!((*insn).chanspec);
    let aref = CR_AREF!((*insn).chanspec);
    let range = CR_RANGE!((*insn).chanspec);
    let mut r3_bits: u32 = 0;
    if aref == AREF_DIFF { r3_bits |= CMD_R3_DIFF; }
    let hw_range = if range >= 1 { range + 1 } else { range };
    outb(r3_bits | (hw_range & 0x03), (*dev).iobase + CMD_R3 as u64);
    outb(chan | 0x80, (*dev).iobase + CMD_R1 as u64);
    udelay(2);
    for n in 0..(*insn).n {
        outb(0x00, (*dev).iobase + CMD_R2 as u64);
        outb(0x30, (*dev).iobase + CMO_R as u64);
        outb(0x00, (*dev).iobase + ADCLEAR_R as u64);
        inw((*dev).iobase + ADFIFO_R as u64);
        outb(0x32, (*dev).iobase + CMO_R as u64);
        let ret = comedi_timeout(dev, s, insn, daq700_ai_eoc, 0);
        if ret != 0 { return ret; }
        let mut d = inw((*dev).iobase + ADFIFO_R as u64) as i32;
        d &= 0x0fff;
        d ^= 0x0800;
        *data.add(n as usize) = d as u32;
    }
    (*insn).n
}

unsafe fn daq700_ai_config(dev: *mut comedi_device, _s: *mut comedi_subdevice) {
    let iobase = (*dev).iobase;
    outb(0x80, iobase + CMD_R1 as u64);
    outb(0x00, iobase + CMD_R2 as u64);
    outb(0x00, iobase + CMD_R3 as u64);
    outb(0x32, iobase + CMO_R as u64);
    outb(0x00, iobase + TIC_R as u64);
    outb(0x00, iobase + ADCLEAR_R as u64);
    inw(iobase + ADFIFO_R as u64);
}

unsafe fn daq700_auto_attach(dev: *mut comedi_device, _context: u64) -> i32 {
    let link = comedi_to_pcmcia_dev(dev);
    (*link).config_flags |= CONF_AUTO_SET_IO;
    let mut ret = comedi_pcmcia_enable(dev, core::ptr::null_mut());
    if ret != 0 { return ret; }
    (*dev).iobase = (*(*link).resource[0]).start;
    ret = comedi_alloc_subdevices(dev, 2);
    if ret != 0 { return ret; }
    let s = (*dev).subdevices;
    (*s).type_ = COMEDI_SUBD_DIO;
    (*s).subdev_flags = SDF_READABLE | SDF_WRITABLE;
    (*s).n_chan = 16;
    (*s).range_table = &range_digital;
    (*s).maxdata = 1;
    (*s).insn_bits = Some(daq700_dio_insn_bits);
    (*s).insn_config = Some(daq700_dio_insn_config);
    (*s).io_bits = 0x00ff;
    let s = s.add(1);
    (*s).type_ = COMEDI_SUBD_AI;
    (*s).subdev_flags = SDF_READABLE | SDF_GROUND | SDF_DIFF;
    (*s).n_chan = 16;
    (*s).maxdata = BIT!(12) - 1;
    (*s).range_table = &range_daq700_ai;
    (*s).insn_read = Some(daq700_ai_rinsn);
    daq700_ai_config(dev, s);
    0
}

static daq700_driver: comedi_driver = comedi_driver {
    driver_name: "ni_daq_700",
    module_: THIS_MODULE,
    auto_attach: Some(daq700_auto_attach),
    detach: Some(comedi_pcmcia_disable),
};

unsafe fn daq700_cs_attach(link: *mut pcmcia_device) -> i32 {
    comedi_pcmcia_auto_config(link, &daq700_driver)
}

static daq700_cs_ids: [pcmcia_device_id; 2] = [
    PCMCIA_DEVICE_MANF_CARD!(0x010b, 0x4743),
    PCMCIA_DEVICE_NULL,
];

static daq700_cs_driver: pcmcia_driver = pcmcia_driver {
    name: "ni_daq_700",
    owner: THIS_MODULE,
    id_table: daq700_cs_ids.as_ptr(),
    probe: Some(daq700_cs_attach),
    remove: Some(comedi_pcmcia_auto_unconfig),
};

// module_comedi_pcmcia_driver!(daq700_driver, daq700_cs_driver);
// MODULE_AUTHOR("Fred Brooks <nsaspook@nsaspook.com>");
// MODULE_DESCRIPTION!("Comedi driver for National Instruments PCMCIA DAQCard-700 DIO/AI");
// MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
