// SPDX-License-Identifier: GPL-2.0+
/*
 * ssv_dnp.c
 * generic comedi driver for SSV Embedded Systems' DIL/Net-PCs
 * Copyright (C) 2001 Robert Schwebel <robert@schwebel.de>
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 2000 David A. Schleef <ds@schleef.org>
 */

/*
 * Driver: ssv_dnp
 * Description: SSV Embedded Systems DIL/Net-PC
 * Author: Robert Schwebel <robert@schwebel.de>
 * Devices: [SSV Embedded Systems] DIL/Net-PC 1486 (dnp-1486)
 * Status: unknown
 */

// External Linux/Comedi declarations are supplied by the surrounding bindings.

const CSCIR: u8 = 0x22;
const CSCDR: u8 = 0x23;
const PAMR: u8 = 0xa5;
const PADR: u8 = 0xa9;
const PBMR: u8 = 0xa4;
const PBDR: u8 = 0xa8;
const PCMR: u8 = 0xa3;
const PCDR: u8 = 0xa7;

unsafe fn dnp_dio_insn_bits(
    _dev: *mut comedi_device,
    s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut u32,
) -> i32 {
    let mut val: u32;
    let mask = comedi_dio_update_state(s, data);
    if mask != 0 {
        outb(PADR, CSCIR);
        outb((*s).state & 0xff, CSCDR);

        outb(PBDR, CSCIR);
        outb(((*s).state >> 8) & 0xff, CSCDR);

        outb(PCDR, CSCIR);
        val = (inb(CSCDR) as u32) & 0x0f;
        outb((((*s).state >> 12) & 0xf0) | val, CSCDR);
    }

    outb(PADR, CSCIR);
    val = inb(CSCDR) as u32;
    outb(PBDR, CSCIR);
    val |= (inb(CSCDR) as u32) << 8;
    outb(PCDR, CSCIR);
    val |= ((inb(CSCDR) as u32 & 0xf0) << 12);

    *data.add(1) = val;
    (*insn).n as i32
}

unsafe fn dnp_dio_insn_config(
    dev: *mut comedi_device,
    s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut u32,
) -> i32 {
    let chan = CR_CHAN((*insn).chanspec);
    let mask: u32;
    let mut val: u32;
    let ret = comedi_dio_insn_config(dev, s, insn, data, 0);
    if ret != 0 {
        return ret;
    }

    if chan < 8 {
        mask = 1 << chan;
        outb(PAMR, CSCIR);
    } else if chan < 16 {
        mask = 1 << (chan - 8);
        outb(PBMR, CSCIR);
    } else {
        // Port C mode register maps pins d0..d3 to bits 4, 6, 8, and 10;
        // multiplication by two brings the requested bit into position.
        mask = 1 << ((chan - 16) * 2);
        outb(PCMR, CSCIR);
    }

    val = inb(CSCDR) as u32;
    if *data == COMEDI_OUTPUT {
        val |= mask;
    } else {
        val &= !mask;
    }
    outb(val, CSCDR);
    (*insn).n as i32
}

unsafe fn dnp_attach(dev: *mut comedi_device, _it: *mut comedi_devconfig) -> i32 {
    let ret = comedi_alloc_subdevices(dev, 1);
    if ret != 0 {
        return ret;
    }

    let s = (*dev).subdevices;
    (*s).type_ = COMEDI_SUBD_DIO;
    (*s).subdev_flags = SDF_READABLE | SDF_WRITABLE;
    (*s).n_chan = 20;
    (*s).maxdata = 1;
    (*s).range_table = &range_digital;
    (*s).insn_bits = Some(dnp_dio_insn_bits);
    (*s).insn_config = Some(dnp_dio_insn_config);

    outb(PAMR, CSCIR);
    outb(0x00, CSCDR);
    outb(PBMR, CSCIR);
    outb(0x00, CSCDR);
    outb(PCMR, CSCIR);
    outb(inb(CSCDR) & 0xAA, CSCDR);
    0
}

unsafe fn dnp_detach(_dev: *mut comedi_device) {
    outb(PAMR, CSCIR);
    outb(0x00, CSCDR);
    outb(PBMR, CSCIR);
    outb(0x00, CSCDR);
    outb(PCMR, CSCIR);
    outb(inb(CSCDR) & 0xAA, CSCDR);
}

static mut dnp_driver: comedi_driver = comedi_driver {
    driver_name: "dnp-1486",
    module: THIS_MODULE,
    attach: Some(dnp_attach),
    detach: Some(dnp_detach),
};

module_comedi_driver!(dnp_driver);

MODULE_AUTHOR!("Comedi https://www.comedi.org");
MODULE_DESCRIPTION!("Comedi low-level driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
