// SPDX-License-Identifier: GPL-2.0+
/*
 * c6xdigio.c
 * Hardware driver for Mechatronic Systems Inc. C6x_DIGIO DSP daughter card.
 * http://web.archive.org/web/%2A/http://robot0.ge.uiuc.edu/~spong/mecha/
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1999 Dan Block
 */

/*
 * Driver: c6xdigio
 * Description: Mechatronic Systems Inc. C6x_DIGIO DSP daughter card
 * Author: Dan Block
 * Status: unknown
 * Devices: [Mechatronic Systems Inc.] C6x_DIGIO DSP daughter card (c6xdigio)
 * Updated: Sun Nov 20 20:18:34 EST 2005
 *
 * Configuration Options:
 *	[0] - base address
 */

// Dependencies supplied by the surrounding kernel/Comedi environment.

const C6XDIGIO_DATA_REG: usize = 0x00;
const C6XDIGIO_DATA_CHAN_SHIFT: u32 = 4;
const C6XDIGIO_DATA_PWM: u32 = 1 << 5;
const C6XDIGIO_DATA_ENCODER: u32 = 1 << 6;
const C6XDIGIO_STATUS_REG: usize = 0x01;
const C6XDIGIO_CTRL_REG: usize = 0x02;

const C6XDIGIO_TIME_OUT: i32 = 20;

#[repr(C)]
pub struct comedi_device {
    pub iobase: usize,
    pub subdevices: *mut comedi_subdevice,
}

#[repr(C)]
pub struct comedi_subdevice {
    pub state: u32,
    pub type_: u32,
    pub subdev_flags: u32,
    pub n_chan: u32,
    pub maxdata: u32,
    pub range_table: *mut range_table,
    pub insn_write: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_subdevice, *mut comedi_insn, *mut u32) -> i32>,
    pub insn_read: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_subdevice, *mut comedi_insn, *mut u32) -> i32>,
}

#[repr(C)]
pub struct comedi_insn {
    pub chanspec: u32,
    pub n: u32,
}

#[repr(C)]
pub struct comedi_devconfig {
    pub options: [u32; 1],
}

#[repr(C)]
pub struct range_table;
#[repr(C)]
pub struct pnp_device_id {
    pub id: *const u8,
}
#[repr(C)]
pub struct pnp_driver {
    pub name: *const u8,
    pub id_table: *const pnp_device_id,
}
#[repr(C)]
pub struct comedi_driver {
    pub driver_name: *const u8,
    pub module: *mut core::ffi::c_void,
    pub attach: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_devconfig) -> i32>,
    pub detach: Option<unsafe extern "C" fn(*mut comedi_device) -> i32>,
}

extern "C" {
    fn inb(port: usize) -> u8;
    fn outb_p(value: u8, port: usize);
    fn comedi_offset_munge(s: *mut comedi_subdevice, val: u32) -> u32;
    fn comedi_check_request_region(dev: *mut comedi_device, start: u32, len: u32, from: u32, to: u32, flags: u32) -> i32;
    fn comedi_alloc_subdevices(dev: *mut comedi_device, n: u32) -> i32;
    fn comedi_driver_register(driver: *mut comedi_driver) -> i32;
    fn comedi_driver_unregister(driver: *mut comedi_driver);
    fn pnp_register_driver(driver: *mut pnp_driver) -> i32;
    fn pnp_unregister_driver(driver: *mut pnp_driver);
    static mut range_unknown: range_table;
    static mut THIS_MODULE: core::ffi::c_void;
    static comedi_legacy_detach: unsafe extern "C" fn(*mut comedi_device) -> i32;
}

const COMEDI_SUBD_PWM: u32 = 0;
const COMEDI_SUBD_COUNTER: u32 = 1;
const SDF_WRITABLE: u32 = 1 << 0;
const SDF_READABLE: u32 = 1 << 1;
const SDF_LSAMPL: u32 = 1 << 2;
const EBUSY: i32 = 16;
const UINT_MAX: u32 = u32::MAX;

#[inline]
const fn c6xdigio_data_chan(x: u32) -> u32 { (x + 1) << C6XDIGIO_DATA_CHAN_SHIFT }
#[inline]
const fn cr_chan(chanspec: u32) -> u32 { chanspec & 0xff }

unsafe fn c6xdigio_chk_status(dev: *mut comedi_device, context: u32) -> i32 {
    let mut timeout = 0;
    loop {
        let mut status = inb((*dev).iobase + C6XDIGIO_STATUS_REG);
        if (status as u32 & 0x80) != context { return 0; }
        timeout += 1;
        if timeout >= C6XDIGIO_TIME_OUT { break; }
        status = status;
    }
    -EBUSY
}

unsafe fn c6xdigio_write_data(dev: *mut comedi_device, val: u32, status: u32) -> i32 {
    outb_p(val as u8, (*dev).iobase + C6XDIGIO_DATA_REG);
    c6xdigio_chk_status(dev, status)
}

unsafe fn c6xdigio_get_encoder_bits(dev: *mut comedi_device, bits: *mut u32, cmd: u32, status: u32) -> i32 {
    let mut val = inb((*dev).iobase + C6XDIGIO_STATUS_REG) as u32;
    val >>= 3;
    val &= 0x07;
    *bits = val;
    c6xdigio_write_data(dev, cmd, status)
}

unsafe fn c6xdigio_pwm_write(dev: *mut comedi_device, chan: u32, mut val: u32) {
    let cmd = C6XDIGIO_DATA_PWM | c6xdigio_data_chan(chan);
    if val > 498 { val = 498; }
    if val < 2 { val = 2; }
    let mut bits = (val >> 0) & 0x03; c6xdigio_write_data(dev, cmd | bits | (0 << 2), 0x00);
    bits = (val >> 2) & 0x03; c6xdigio_write_data(dev, cmd | bits | (1 << 2), 0x80);
    bits = (val >> 4) & 0x03; c6xdigio_write_data(dev, cmd | bits | (0 << 2), 0x00);
    bits = (val >> 6) & 0x03; c6xdigio_write_data(dev, cmd | bits | (1 << 2), 0x80);
    bits = (val >> 8) & 0x03; c6xdigio_write_data(dev, cmd | bits | (0 << 2), 0x00);
    c6xdigio_write_data(dev, 0x00, 0x80);
}

unsafe fn c6xdigio_encoder_read(dev: *mut comedi_device, chan: u32) -> u32 {
    let cmd = C6XDIGIO_DATA_ENCODER | c6xdigio_data_chan(chan);
    let mut val = 0; let mut bits = 0;
    c6xdigio_write_data(dev, cmd, 0x00);
    c6xdigio_get_encoder_bits(dev, &mut bits, cmd | (1 << 2), 0x80); val |= bits << 0;
    c6xdigio_get_encoder_bits(dev, &mut bits, cmd | (0 << 2), 0x00); val |= bits << 3;
    c6xdigio_get_encoder_bits(dev, &mut bits, cmd | (1 << 2), 0x80); val |= bits << 6;
    c6xdigio_get_encoder_bits(dev, &mut bits, cmd | (0 << 2), 0x00); val |= bits << 9;
    c6xdigio_get_encoder_bits(dev, &mut bits, cmd | (1 << 2), 0x80); val |= bits << 12;
    c6xdigio_get_encoder_bits(dev, &mut bits, cmd | (0 << 2), 0x00); val |= bits << 15;
    c6xdigio_get_encoder_bits(dev, &mut bits, cmd | (1 << 2), 0x80); val |= bits << 18;
    c6xdigio_get_encoder_bits(dev, &mut bits, cmd | (0 << 2), 0x00); val |= bits << 21;
    c6xdigio_write_data(dev, 0x00, 0x80); val
}

unsafe extern "C" fn c6xdigio_pwm_insn_write(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let chan = cr_chan((*insn).chanspec); let mut val = ((*s).state >> (16 * chan)) & 0xffff;
    for i in 0..(*insn).n { val = *data.add(i as usize); c6xdigio_pwm_write(dev, chan, val); }
    (*s).state &= 0xffff << (16 * chan); (*s).state |= val << (16 * chan); (*insn).n as i32
}

unsafe extern "C" fn c6xdigio_pwm_insn_read(_dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let chan = cr_chan((*insn).chanspec); let val = ((*s).state >> (16 * chan)) & 0xffff;
    for i in 0..(*insn).n { *data.add(i as usize) = val; } (*insn).n as i32
}

unsafe extern "C" fn c6xdigio_encoder_insn_read(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let chan = cr_chan((*insn).chanspec);
    for i in 0..(*insn).n { *data.add(i as usize) = comedi_offset_munge(s, c6xdigio_encoder_read(dev, chan)); }
    (*insn).n as i32
}

unsafe fn c6xdigio_init(dev: *mut comedi_device) {
    c6xdigio_write_data(dev, 0x70, 0x00); c6xdigio_write_data(dev, 0x74, 0x80); c6xdigio_write_data(dev, 0x70, 0x00); c6xdigio_write_data(dev, 0x00, 0x80);
    c6xdigio_write_data(dev, 0x68, 0x00); c6xdigio_write_data(dev, 0x6c, 0x80); c6xdigio_write_data(dev, 0x68, 0x00); c6xdigio_write_data(dev, 0x00, 0x80);
}

static mut c6xdigio_pnp_tbl: [pnp_device_id; 3] = [
    pnp_device_id { id: b"PNP0400\0".as_ptr() },
    pnp_device_id { id: b"PNP0401\0".as_ptr() },
    pnp_device_id { id: core::ptr::null() },
];
static mut c6xdigio_pnp_driver: pnp_driver = pnp_driver { name: b"c6xdigio\0".as_ptr(), id_table: c6xdigio_pnp_tbl.as_ptr() };

unsafe extern "C" fn c6xdigio_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> i32 {
    let iobase = (*it).options[0];
    let mut ret = comedi_check_request_region(dev, iobase, 0x03, 0, UINT_MAX, 4); if ret != 0 { return ret; }
    ret = comedi_alloc_subdevices(dev, 2); if ret != 0 { return ret; }
    let s = (*dev).subdevices;
    (*s).type_ = COMEDI_SUBD_PWM; (*s).subdev_flags = SDF_WRITABLE; (*s).n_chan = 2; (*s).maxdata = 500; (*s).range_table = &mut range_unknown; (*s).insn_write = Some(c6xdigio_pwm_insn_write); (*s).insn_read = Some(c6xdigio_pwm_insn_read);
    let s = s.add(1);
    (*s).type_ = COMEDI_SUBD_COUNTER; (*s).subdev_flags = SDF_READABLE | SDF_LSAMPL; (*s).n_chan = 2; (*s).maxdata = 0xffffff; (*s).range_table = &mut range_unknown; (*s).insn_read = Some(c6xdigio_encoder_insn_read);
    c6xdigio_init(dev); 0
}

static mut c6xdigio_driver: comedi_driver = comedi_driver { driver_name: b"c6xdigio\0".as_ptr(), module: unsafe { &mut THIS_MODULE }, attach: Some(c6xdigio_attach), detach: Some(comedi_legacy_detach) };
static mut c6xdigio_pnp_registered: bool = false;

#[no_mangle]
pub unsafe extern "C" fn c6xdigio_module_init() -> i32 {
    let mut ret = comedi_driver_register(&mut c6xdigio_driver); if ret != 0 { return ret; }
    // CONFIG_PNP controls whether PnP support is compiled in.
    ret = pnp_register_driver(&mut c6xdigio_pnp_driver);
    if ret != 0 { ret = 0; } else { c6xdigio_pnp_registered = true; }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn c6xdigio_module_exit() {
    if c6xdigio_pnp_registered { pnp_unregister_driver(&mut c6xdigio_pnp_driver); }
    comedi_driver_unregister(&mut c6xdigio_driver);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
