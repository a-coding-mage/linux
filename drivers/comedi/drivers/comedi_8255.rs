// SPDX-License-Identifier: GPL-2.0+
/*
 * comedi_8255.c
 * Generic 8255 digital I/O support
 *
 * Split from the Comedi "8255" driver module.
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1998 David A. Schleef <ds@schleef.org>
 */

/*
 * Module: comedi_8255
 * Description: Generic 8255 support
 * Author: ds
 * Updated: Fri, 22 May 2015 12:14:17 +0000
 * Status: works
 *
 * This module is not used directly by end-users.  Rather, it is used by
 * other drivers to provide support for an 8255 "Programmable Peripheral
 * Interface" (PPI) chip.
 *
 * The classic in digital I/O.  The 8255 appears in Comedi as a single
 * digital I/O subdevice with 24 channels.  The channel 0 corresponds to
 * the 8255's port A, bit 0; channel 23 corresponds to port C, bit 7.
 * Direction configuration is done in blocks, with channels 0-7, 8-15,
 * 16-19, and 20-23 making up the 4 blocks.  The only 8255 mode
 * supported is mode 0.
 */

#[repr(C)]
pub struct Subdev8255Private {
    pub context: ::core::ffi::c_ulong,
    pub io: Option<unsafe extern "C" fn(
        *mut ComediDevice, ::core::ffi::c_int, ::core::ffi::c_int,
        ::core::ffi::c_int, ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int>,
}

#[repr(C)] pub struct ComediDevice { pub iobase: ::core::ffi::c_ulong, pub mmio: *mut u8 }
#[repr(C)] pub struct ComediSubdevice {
    pub private: *mut ::core::ffi::c_void,
    pub state: u32,
    pub io_bits: u32,
    pub type_: ::core::ffi::c_uint,
    pub subdev_flags: ::core::ffi::c_uint,
    pub n_chan: ::core::ffi::c_uint,
    pub range_table: *const ::core::ffi::c_void,
    pub maxdata: ::core::ffi::c_uint,
    pub insn_bits: Option<unsafe extern "C" fn(*mut ComediDevice, *mut ComediSubdevice, *mut ComediInsn, *mut u32) -> ::core::ffi::c_int>,
    pub insn_config: Option<unsafe extern "C" fn(*mut ComediDevice, *mut ComediSubdevice, *mut ComediInsn, *mut u32) -> ::core::ffi::c_int>,
}
#[repr(C)] pub struct ComediInsn { pub chanspec: u32, pub n: ::core::ffi::c_uint }

extern "C" {
    static range_digital: ::core::ffi::c_void;
    fn comedi_dio_update_state(s: *mut ComediSubdevice, data: *mut u32) -> u32;
    fn comedi_dio_insn_config(dev: *mut ComediDevice, s: *mut ComediSubdevice, insn: *mut ComediInsn, data: *mut u32, mask: u32) -> ::core::ffi::c_int;
    fn comedi_alloc_spriv(s: *mut ComediSubdevice, size: usize) -> *mut ::core::ffi::c_void;
    fn outb(value: u8, port: ::core::ffi::c_ulong);
    fn inb(port: ::core::ffi::c_ulong) -> u8;
    fn writeb(value: u8, addr: *mut u8);
    fn readb(addr: *mut u8) -> u8;
}

const I8255_DATA_A_REG: ::core::ffi::c_int = 0;
const I8255_DATA_B_REG: ::core::ffi::c_int = 1;
const I8255_DATA_C_REG: ::core::ffi::c_int = 2;
const I8255_CTRL_REG: ::core::ffi::c_int = 3;
const I8255_CTRL_CW: ::core::ffi::c_int = 0x80;
const I8255_CTRL_A_IO: ::core::ffi::c_int = 0x10;
const I8255_CTRL_B_IO: ::core::ffi::c_int = 0x02;
const I8255_CTRL_C_LO_IO: ::core::ffi::c_int = 0x01;
const I8255_CTRL_C_HI_IO: ::core::ffi::c_int = 0x08;
const COMEDI_SUBD_DIO: ::core::ffi::c_uint = 0;
const SDF_READABLE: ::core::ffi::c_uint = 1;
const SDF_WRITABLE: ::core::ffi::c_uint = 2;

unsafe extern "C" fn subdev_8255_io(dev: *mut ComediDevice, dir: i32, port: i32, data: i32, regbase: usize) -> i32 {
    if dir != 0 { outb(data as u8, (*dev).iobase.wrapping_add(regbase as u64).wrapping_add(port as u64)); return 0; }
    inb((*dev).iobase.wrapping_add(regbase as u64).wrapping_add(port as u64)) as i32
}

unsafe extern "C" fn subdev_8255_mmio(dev: *mut ComediDevice, dir: i32, port: i32, data: i32, regbase: usize) -> i32 {
    let addr = (*dev).mmio.add(regbase).add(port as usize);
    if dir != 0 { writeb(data as u8, addr); return 0; }
    readb(addr) as i32
}

unsafe extern "C" fn subdev_8255_insn(dev: *mut ComediDevice, s: *mut ComediSubdevice, insn: *mut ComediInsn, data: *mut u32) -> i32 {
    let spriv = &mut *((*s).private as *mut Subdev8255Private);
    let context = spriv.context;
    let mask = comedi_dio_update_state(s, data);
    if mask != 0 {
        if mask & 0xff != 0 { (spriv.io.unwrap())(dev, 1, I8255_DATA_A_REG, ((*s).state & 0xff) as i32, context); }
        if mask & 0xff00 != 0 { (spriv.io.unwrap())(dev, 1, I8255_DATA_B_REG, (((*s).state >> 8) & 0xff) as i32, context); }
        if mask & 0xff0000 != 0 { (spriv.io.unwrap())(dev, 1, I8255_DATA_C_REG, (((*s).state >> 16) & 0xff) as i32, context); }
    }
    let mut v = (spriv.io.unwrap())(dev, 0, I8255_DATA_A_REG, 0, context) as u32;
    v |= ((spriv.io.unwrap())(dev, 0, I8255_DATA_B_REG, 0, context) as u32) << 8;
    v |= ((spriv.io.unwrap())(dev, 0, I8255_DATA_C_REG, 0, context) as u32) << 16;
    *data.add(1) = v;
    (*insn).n as i32
}

unsafe fn subdev_8255_do_config(dev: *mut ComediDevice, s: *mut ComediSubdevice) {
    let spriv = &mut *((*s).private as *mut Subdev8255Private);
    let mut config = I8255_CTRL_CW;
    if (*s).io_bits & 0x0000ff == 0 { config |= I8255_CTRL_A_IO; }
    if (*s).io_bits & 0x00ff00 == 0 { config |= I8255_CTRL_B_IO; }
    if (*s).io_bits & 0x0f0000 == 0 { config |= I8255_CTRL_C_LO_IO; }
    if (*s).io_bits & 0xf00000 == 0 { config |= I8255_CTRL_C_HI_IO; }
    (spriv.io.unwrap())(dev, 1, I8255_CTRL_REG, config, spriv.context);
}

unsafe extern "C" fn subdev_8255_insn_config(dev: *mut ComediDevice, s: *mut ComediSubdevice, insn: *mut ComediInsn, data: *mut u32) -> i32 {
    let chan = (*insn).chanspec & 0xff;
    let mask = if chan < 8 { 0x0000ff } else if chan < 16 { 0x00ff00 } else if chan < 20 { 0x0f0000 } else { 0xf00000 };
    let ret = comedi_dio_insn_config(dev, s, insn, data, mask);
    if ret != 0 { return ret; }
    subdev_8255_do_config(dev, s);
    (*insn).n as i32
}

unsafe fn __subdev_8255_init(dev: *mut ComediDevice, s: *mut ComediSubdevice, io: Option<unsafe extern "C" fn(*mut ComediDevice, i32, i32, i32, usize) -> i32>, context: usize) -> i32 {
    if io.is_none() { return -22; }
    let spriv = comedi_alloc_spriv(s, core::mem::size_of::<Subdev8255Private>()) as *mut Subdev8255Private;
    if spriv.is_null() { return -12; }
    (*spriv).context = context; (*spriv).io = io; (*s).type_ = COMEDI_SUBD_DIO; (*s).subdev_flags = SDF_READABLE | SDF_WRITABLE;
    (*s).n_chan = 24; (*s).range_table = &range_digital; (*s).maxdata = 1; (*s).insn_bits = Some(subdev_8255_insn); (*s).insn_config = Some(subdev_8255_insn_config);
    subdev_8255_do_config(dev, s); 0
}

// CONFIG_HAS_IOPORT conditionally provides the I/O-mapped initializer.
#[cfg(feature = "CONFIG_HAS_IOPORT")]
pub unsafe extern "C" fn subdev_8255_io_init(dev: *mut ComediDevice, s: *mut ComediSubdevice, regbase: usize) -> i32 { __subdev_8255_init(dev, s, Some(subdev_8255_io), regbase) }

pub unsafe extern "C" fn subdev_8255_mm_init(dev: *mut ComediDevice, s: *mut ComediSubdevice, regbase: usize) -> i32 { __subdev_8255_init(dev, s, Some(subdev_8255_mmio), regbase) }

pub unsafe extern "C" fn subdev_8255_cb_init(dev: *mut ComediDevice, s: *mut ComediSubdevice, io: Option<unsafe extern "C" fn(*mut ComediDevice, i32, i32, i32, usize) -> i32>, context: usize) -> i32 { __subdev_8255_init(dev, s, io, context) }

pub unsafe extern "C" fn subdev_8255_regbase(s: *mut ComediSubdevice) -> usize { (*( (*s).private as *mut Subdev8255Private)).context }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
