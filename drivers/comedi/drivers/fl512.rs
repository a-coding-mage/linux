// SPDX-License-Identifier: GPL-2.0+
/*
 * fl512.c
 * Anders Gnistrup <ex18@kalman.iau.dtu.dk>
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 2000 David A. Schleef <ds@schleef.org>
 */

/*
 * Driver: fl512
 * Description: unknown
 * Author: Anders Gnistrup <ex18@kalman.iau.dtu.dk>
 * Devices: [unknown] FL512 (fl512)
 * Status: unknown
 *
 * Digital I/O is not supported.
 *
 * Configuration options:
 *   [0] - I/O port base address
 */

// Linux/Comedi dependencies supplied by other translation units.

const FL512_AI_LSB_REG: usize = 0x02;
const FL512_AI_MSB_REG: usize = 0x03;
const FL512_AI_MUX_REG: usize = 0x02;
const FL512_AI_START_CONV_REG: usize = 0x03;

#[inline]
const fn fl512_ao_data_reg(x: usize) -> usize { 0x04 + (x * 2) }
#[inline]
const fn fl512_ao_trig_reg(x: usize) -> usize { 0x04 + (x * 2) }

#[repr(C)]
pub struct ComediLrange {
    pub length: u32,
    pub range: [ComediRange; 7],
}

#[repr(C)]
pub struct ComediRange {
    pub min: f64,
    pub max: f64,
    pub unit: u32,
}

// BIP_RANGE/UNI_RANGE are provided by the Comedi range definitions.
extern "C" {
    static range_fl512: ComediLrange;
    fn outb(value: u8, port: usize);
    fn inb(port: usize) -> u8;
    fn usleep_range(min: u32, max: u32);
    fn comedi_check_request_region(dev: *mut ComediDevice, from: u32, len: u32,
                                   min: u32, max: u32, align: u32) -> i32;
    fn comedi_alloc_subdevices(dev: *mut ComediDevice, num: u32) -> i32;
    fn comedi_alloc_subdev_readback(s: *mut ComediSubdevice) -> i32;
    fn comedi_legacy_detach(dev: *mut ComediDevice) -> i32;
}

#[repr(C)]
pub struct ComediDevice {
    pub iobase: usize,
    pub subdevices: *mut ComediSubdevice,
}

#[repr(C)]
pub struct ComediDevconfig {
    pub options: *mut u32,
}

#[repr(C)]
pub struct ComediInsn {
    pub chanspec: u32,
    pub n: u32,
}

#[repr(C)]
pub struct ComediSubdevice {
    pub type_: u32,
    pub subdev_flags: u32,
    pub n_chan: u32,
    pub maxdata: u32,
    pub range_table: *const ComediLrange,
    pub readback: *mut u32,
    pub insn_read: Option<unsafe extern "C" fn(*mut ComediDevice, *mut ComediSubdevice, *mut ComediInsn, *mut u32) -> i32>,
    pub insn_write: Option<unsafe extern "C" fn(*mut ComediDevice, *mut ComediSubdevice, *mut ComediInsn, *mut u32) -> i32>,
}

const COMEDI_SUBD_AI: u32 = 0;
const COMEDI_SUBD_AO: u32 = 1;
const SDF_READABLE: u32 = 1 << 0;
const SDF_WRITABLE: u32 = 1 << 1;
const SDF_GROUND: u32 = 1 << 2;

#[inline]
unsafe fn cr_chan(chanspec: u32) -> u32 { chanspec & 0xff }

static RANGE_FL512: ComediLrange = ComediLrange {
    length: 4,
    range: [
        ComediRange { min: -0.5, max: 0.5, unit: 0 },
        ComediRange { min: -1.0, max: 1.0, unit: 0 },
        ComediRange { min: -5.0, max: 5.0, unit: 0 },
        ComediRange { min: -10.0, max: 10.0, unit: 0 },
        ComediRange { min: 0.0, max: 1.0, unit: 0 },
        ComediRange { min: 0.0, max: 5.0, unit: 0 },
        ComediRange { min: 0.0, max: 10.0, unit: 0 },
    ],
};

unsafe extern "C" fn fl512_ai_insn_read(dev: *mut ComediDevice, s: *mut ComediSubdevice,
                                          insn: *mut ComediInsn, data: *mut u32) -> i32 {
    let chan = cr_chan((*insn).chanspec);
    outb(chan as u8, (*dev).iobase + FL512_AI_MUX_REG);
    for i in 0..(*insn).n {
        outb(0, (*dev).iobase + FL512_AI_START_CONV_REG);
        // XXX should test "done" flag instead of delay
        usleep_range(30, 100);
        let mut val = inb((*dev).iobase + FL512_AI_LSB_REG) as u32;
        val |= (inb((*dev).iobase + FL512_AI_MSB_REG) as u32) << 8;
        val &= (*s).maxdata;
        *data.add(i as usize) = val;
    }
    (*insn).n as i32
}

unsafe extern "C" fn fl512_ao_insn_write(dev: *mut ComediDevice, s: *mut ComediSubdevice,
                                           insn: *mut ComediInsn, data: *mut u32) -> i32 {
    let chan = cr_chan((*insn).chanspec);
    let mut val = *(*s).readback.add(chan as usize);
    for i in 0..(*insn).n {
        val = *data.add(i as usize);
        // write LSB, MSB then trigger conversion
        outb((val & 0x0ff) as u8, (*dev).iobase + fl512_ao_data_reg(chan as usize));
        outb(((val >> 8) & 0xf) as u8, (*dev).iobase + fl512_ao_data_reg(chan as usize));
        inb((*dev).iobase + fl512_ao_trig_reg(chan as usize));
    }
    *(*s).readback.add(chan as usize) = val;
    (*insn).n as i32
}

unsafe extern "C" fn fl512_attach(dev: *mut ComediDevice, it: *mut ComediDevconfig) -> i32 {
    let iobase = *(*it).options;
    let mut ret = comedi_check_request_region(dev, iobase, 0x10, 0, u32::MAX, 16);
    if ret != 0 { return ret; }
    ret = comedi_alloc_subdevices(dev, 2);
    if ret != 0 { return ret; }

    let s = &mut *(*dev).subdevices;
    s.type_ = COMEDI_SUBD_AI;
    s.subdev_flags = SDF_READABLE | SDF_GROUND;
    s.n_chan = 16;
    s.maxdata = 0x0fff;
    s.range_table = &RANGE_FL512;
    s.insn_read = Some(fl512_ai_insn_read);

    let s = &mut *(*dev).subdevices.add(1);
    s.type_ = COMEDI_SUBD_AO;
    s.subdev_flags = SDF_WRITABLE;
    s.n_chan = 2;
    s.maxdata = 0x0fff;
    s.range_table = &RANGE_FL512;
    s.insn_write = Some(fl512_ao_insn_write);
    comedi_alloc_subdev_readback(s)
}

// static struct comedi_driver fl512_driver = { driver_name = "fl512", module = THIS_MODULE,
//     attach = fl512_attach, detach = comedi_legacy_detach };
// module_comedi_driver(fl512_driver);
// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_DESCRIPTION("Comedi low-level driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
