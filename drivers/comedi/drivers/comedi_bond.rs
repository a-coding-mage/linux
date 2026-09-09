// SPDX-License-Identifier: GPL-2.0+
/*
 * comedi_bond.c
 * A Comedi driver to 'bond' or merge multiple drivers and devices as one.
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 2000 David A. Schleef <ds@schleef.org>
 * Copyright (C) 2005 Calin A. Culianu <calin@ajvar.org>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct comedi_device {
    pub private: *mut c_void,
    pub minor: c_uint,
    pub class_dev: *mut c_void,
    pub board_name: *const c_char,
    pub subdevices: *mut comedi_subdevice,
    pub driver: *mut comedi_driver,
}

#[repr(C)]
pub struct comedi_subdevice {
    pub type_: c_uint,
    pub subdev_flags: c_uint,
    pub n_chan: c_uint,
    pub maxdata: c_uint,
    pub range_table: *mut c_void,
    pub insn_bits: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_subdevice, *mut comedi_insn, *mut c_uint) -> c_int>,
    pub insn_config: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_subdevice, *mut comedi_insn, *mut c_uint) -> c_int>,
}

#[repr(C)]
pub struct comedi_insn {
    pub n: c_uint,
    pub chanspec: c_uint,
}

#[repr(C)]
pub struct comedi_devconfig {
    pub options: [c_int; 32],
}

#[repr(C)]
pub struct comedi_driver {
    pub driver_name: *const c_char,
    pub module: *mut c_void,
    pub attach: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_devconfig) -> c_int>,
    pub detach: Option<unsafe extern "C" fn(*mut comedi_device)>,
}

#[repr(C)]
pub struct bonded_device {
    pub dev: *mut comedi_device,
    pub minor: c_uint,
    pub subdev: c_uint,
    pub nchans: c_uint,
}

#[repr(C)]
pub struct comedi_bond_private {
    pub name: [c_char; 256],
    pub devs: *mut *mut bonded_device,
    pub ndevs: c_uint,
    pub nchans: c_uint,
}

extern "C" {
    fn comedi_dio_bitfield2(dev: *mut comedi_device, subdev: c_uint, write_mask: c_uint, data: *mut c_uint, base_chan: c_uint) -> c_int;
    fn comedi_dio_config(dev: *mut comedi_device, subdev: c_uint, chan: c_uint, io: c_uint) -> c_int;
    fn comedi_dio_get_config(dev: *mut comedi_device, subdev: c_uint, chan: c_uint, data: *mut c_uint) -> c_int;
    fn comedi_open_from(file: *const c_char, minor: c_uint) -> *mut comedi_device;
    fn comedi_find_subdevice_by_type(dev: *mut comedi_device, type_: c_uint, subdev: c_int) -> c_int;
    fn comedi_get_n_channels(dev: *mut comedi_device, subdev: c_int) -> c_int;
    fn comedi_close_from(dev: *mut comedi_device, minor: c_uint);
    fn comedi_alloc_devpriv(dev: *mut comedi_device, size: usize) -> *mut c_void;
    fn comedi_alloc_subdevices(dev: *mut comedi_device, n: c_uint) -> c_int;
    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn krealloc(ptr: *mut c_void, size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
}

const COMEDI_SUBD_DIO: c_uint = 2;
const SDF_READABLE: c_uint = 0x01;
const SDF_WRITABLE: c_uint = 0x02;
const INSN_CONFIG_DIO_OUTPUT: c_uint = 0;
const INSN_CONFIG_DIO_INPUT: c_uint = 1;
const INSN_CONFIG_DIO_QUERY: c_uint = 2;
const COMEDI_OUTPUT: c_uint = INSN_CONFIG_DIO_OUTPUT;
const COMEDI_INPUT: c_uint = INSN_CONFIG_DIO_INPUT;
const COMEDI_NDEVCONFOPTS: usize = 16;
const COMEDI_NUM_BOARD_MINORS: usize = 256;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;

extern "C" {
    static mut range_digital: c_void;
}

#[inline]
unsafe fn cr_chan(chanspec: c_uint) -> c_uint { chanspec & 0xff }

unsafe extern "C" fn bonding_dio_insn_bits(
    dev: *mut comedi_device, _s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut c_uint,
) -> c_int {
    let devpriv = &mut *((*dev).private as *mut comedi_bond_private);
    let mut write_mask = *data;
    let data_bits = *data.add(1);
    let mut base_chan = cr_chan((*insn).chanspec);
    let mut n_left = devpriv.nchans.wrapping_sub(base_chan).min(32);
    let mut n_done = 0;
    let mut devs = devpriv.devs;
    while n_left != 0 {
        let bdev = *devs;
        devs = devs.add(1);
        if base_chan < (*bdev).nchans {
            let b_chans = (*bdev).nchans.wrapping_sub(base_chan).min(n_left);
            let b_mask = if b_chans < 32 { (1u32 << b_chans) - 1 } else { 0xffff_ffff };
            let b_write_mask = (write_mask >> n_done) & b_mask;
            let mut b_data_bits = (data_bits >> n_done) & b_mask;
            let ret = comedi_dio_bitfield2((*bdev).dev, (*bdev).subdev, b_write_mask, &mut b_data_bits, base_chan);
            if ret < 0 { return ret; }
            *data.add(1) &= !(b_mask << n_done);
            *data.add(1) |= (b_data_bits & b_mask) << n_done;
            base_chan = 0;
            n_done += b_chans;
            n_left -= b_chans;
        } else {
            base_chan -= (*bdev).nchans;
        }
    }
    (*insn).n as c_int
}

unsafe extern "C" fn bonding_dio_insn_config(
    dev: *mut comedi_device, _s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut c_uint,
) -> c_int {
    let devpriv = &mut *((*dev).private as *mut comedi_bond_private);
    let mut chan = cr_chan((*insn).chanspec);
    let mut devs = devpriv.devs;
    let mut bdev = *devs;
    while chan >= (*bdev).nchans { chan -= (*bdev).nchans; devs = devs.add(1); bdev = *devs; }
    let ret = match *data {
        INSN_CONFIG_DIO_OUTPUT | INSN_CONFIG_DIO_INPUT => comedi_dio_config((*bdev).dev, (*bdev).subdev, chan, *data),
        INSN_CONFIG_DIO_QUERY => comedi_dio_get_config((*bdev).dev, (*bdev).subdev, chan, data.add(1)),
        _ => -EINVAL,
    };
    if ret >= 0 { (*insn).n as c_int } else { ret }
}

unsafe extern "C" fn do_dev_config(dev: *mut comedi_device, it: *mut comedi_devconfig) -> c_int {
    let p = &mut *((*dev).private as *mut comedi_bond_private);
    p.name[0] = 0;
    for i in 0..COMEDI_NDEVCONFOPTS {
        let minor = (*it).options[i];
        if i != 0 && minor == 0 { break; }
        if minor < 0 || minor as usize >= COMEDI_NUM_BOARD_MINORS || minor as c_uint == (*dev).minor { return -EINVAL; }
        let path = core::ffi::CStr::from_bytes_with_nul_unchecked(b"/dev/comedi0\0");
        let d = comedi_open_from(path.as_ptr(), (*dev).minor);
        if d.is_null() { return -ENODEV; }
        let mut sdev = -1;
        loop {
            sdev = comedi_find_subdevice_by_type(d, COMEDI_SUBD_DIO, sdev + 1);
            if sdev <= -1 { break; }
            let nchans = comedi_get_n_channels(d, sdev);
            if nchans <= 0 { return -EINVAL; }
            let bdev = kmalloc(core::mem::size_of::<bonded_device>(), 0) as *mut bonded_device;
            if bdev.is_null() { return -ENOMEM; }
            (*bdev).dev = d; (*bdev).minor = minor as c_uint; (*bdev).subdev = sdev as c_uint; (*bdev).nchans = nchans as c_uint;
            p.nchans += nchans as c_uint;
            let list = krealloc(p.devs as *mut c_void, (p.ndevs as usize + 1) * core::mem::size_of::<*mut bonded_device>(), 0) as *mut *mut bonded_device;
            if list.is_null() { kfree(bdev as *mut c_void); return -ENOMEM; }
            p.devs = list; *p.devs.add(p.ndevs as usize) = bdev; p.ndevs += 1;
        }
    }
    if p.nchans == 0 { -EINVAL } else { 0 }
}

unsafe extern "C" fn bonding_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> c_int {
    let p = comedi_alloc_devpriv(dev, core::mem::size_of::<comedi_bond_private>()) as *mut comedi_bond_private;
    if p.is_null() { return -ENOMEM; }
    core::ptr::write_bytes(p, 0, 1); (*dev).private = p as *mut c_void;
    let ret = do_dev_config(dev, it); if ret != 0 { return ret; }
    (*dev).board_name = (*p).name.as_ptr();
    let ret = comedi_alloc_subdevices(dev, 1); if ret != 0 { return ret; }
    let s = &mut *(*dev).subdevices;
    s.type_ = COMEDI_SUBD_DIO; s.subdev_flags = SDF_READABLE | SDF_WRITABLE; s.n_chan = (*p).nchans; s.maxdata = 1;
    s.range_table = &mut range_digital; s.insn_bits = Some(bonding_dio_insn_bits); s.insn_config = Some(bonding_dio_insn_config);
    0
}

unsafe extern "C" fn bonding_detach(dev: *mut comedi_device) {
    let p = (*dev).private as *mut comedi_bond_private;
    if !p.is_null() && !(*p).devs.is_null() {
        while (*p).ndevs != 0 { (*p).ndevs -= 1; let b = *(*p).devs.add((*p).ndevs as usize); if !b.is_null() { kfree(b as *mut c_void); } }
        kfree((*p).devs as *mut c_void); (*p).devs = core::ptr::null_mut();
    }
}

#[no_mangle]
pub static mut bonding_driver: comedi_driver = comedi_driver {
    driver_name: b"comedi_bond\0".as_ptr() as *const c_char,
    module: core::ptr::null_mut(),
    attach: Some(bonding_attach),
    detach: Some(bonding_detach),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
