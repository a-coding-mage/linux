// SPDX-License-Identifier: GPL-2.0+
/*
 * kcomedilib/kcomedilib.c
 * a comedlib interface for kernel modules
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1997-2000 David A. Schleef <ds@schleef.org>
 */

// Linux kernel headers and COMEDI headers are supplied by the surrounding
// translation unit/build; their declarations are intentionally external here.

extern "C" {
    fn comedi_dev_get_from_minor(minor: libc::c_uint) -> *mut comedi_device;
    fn comedi_dev_put(dev: *mut comedi_device);
    fn comedi_check_chanlist(s: *mut comedi_subdevice, n: libc::c_uint,
                             chanspec: *mut libc::c_uint) -> libc::c_int;
    fn comedi_get_n_channels(dev: *mut comedi_device, subdevice: libc::c_uint) -> libc::c_int;
}

const COMEDI_NUM_BOARD_MINORS: usize = 16; // supplied by <linux/comedi.h>
const COMEDI_SUBD_UNUSED: libc::c_int = 0;
const INSN_BITS: libc::c_uint = 0;
const INSN_CONFIG: libc::c_uint = 1;
const INSN_CONFIG_DIO_QUERY: libc::c_uint = 0;
const EINVAL: libc::c_int = 22;
const EIO: libc::c_int = 5;
const EBUSY: libc::c_int = 16;
const ENODEV: libc::c_int = 19;

#[repr(C)]
pub struct comedi_device {
    pub attach_lock: linux_rwsem,
    pub attached: bool,
    pub n_subdevices: libc::c_uint,
    pub subdevices: *mut comedi_subdevice,
    pub mutex: linux_mutex,
    pub minor: libc::c_uint,
    pub class_dev: *mut libc::c_void,
}

#[repr(C)]
pub struct comedi_subdevice {
    pub type_: libc::c_int,
    pub n_chan: libc::c_int,
    pub busy: *mut comedi_device,
    pub insn_bits: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_subdevice, *mut comedi_insn, *mut libc::c_uint) -> libc::c_int>,
    pub insn_config: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_subdevice, *mut comedi_insn, *mut libc::c_uint) -> libc::c_int>,
}

#[repr(C)] pub struct comedi_insn { pub insn: libc::c_uint, pub n: libc::c_uint, pub subdev: libc::c_uint, pub chanspec: libc::c_uint }
#[repr(C)] pub struct linux_mutex { _private: [u8; 0] }
#[repr(C)] pub struct linux_rwsem { _private: [u8; 0] }

static mut KCOMEDILIB_TO_FROM: [[u8; COMEDI_NUM_BOARD_MINORS]; COMEDI_NUM_BOARD_MINORS] = [[0; COMEDI_NUM_BOARD_MINORS]; COMEDI_NUM_BOARD_MINORS];

unsafe fn kcomedilib_set_link_from_to(from: libc::c_uint, to: libc::c_uint) -> bool {
    let mut destinations = [[false; COMEDI_NUM_BOARD_MINORS]; 2];
    let mut cur = 0usize;
    let mut okay = true;
    if to as usize >= COMEDI_NUM_BOARD_MINORS { return false; }
    if from as usize >= COMEDI_NUM_BOARD_MINORS { return true; }
    destinations[cur][from as usize] = true;
    loop {
        let next = 1 - cur;
        if destinations[cur][to as usize] { okay = false; break; }
        destinations[next] = [false; COMEDI_NUM_BOARD_MINORS];
        for t in 0..COMEDI_NUM_BOARD_MINORS {
            if destinations[cur][t] {
                for f in 0..COMEDI_NUM_BOARD_MINORS {
                    if KCOMEDILIB_TO_FROM[t][f] != 0 { destinations[next][f] = true; }
                }
            }
        }
        cur = next;
        if !destinations[cur].iter().any(|&v| v) { break; }
    }
    if okay {
        if KCOMEDILIB_TO_FROM[to as usize][from as usize] < 255 { KCOMEDILIB_TO_FROM[to as usize][from as usize] += 1; }
        else { okay = false; }
    }
    okay
}

unsafe fn kcomedilib_clear_link_from_to(from: libc::c_uint, to: libc::c_uint) {
    if (to as usize) < COMEDI_NUM_BOARD_MINORS && (from as usize) < COMEDI_NUM_BOARD_MINORS && KCOMEDILIB_TO_FROM[to as usize][from as usize] != 0 { KCOMEDILIB_TO_FROM[to as usize][from as usize] -= 1; }
}

pub unsafe extern "C" fn comedi_open_from(filename: *const libc::c_char, from: libc::c_int) -> *mut comedi_device {
    let prefix = b"/dev/comedi\0";
    if libc::strncmp(filename, prefix.as_ptr() as *const libc::c_char, 11) != 0 { return core::ptr::null_mut(); }
    let text = libc::CStr::from_ptr(filename.add(11));
    let minor: usize = match text.to_str().ok().and_then(|s| s.parse().ok()) { Some(v) => v, None => return core::ptr::null_mut() };
    if minor >= COMEDI_NUM_BOARD_MINORS { return core::ptr::null_mut(); }
    let dev = comedi_dev_get_from_minor(minor as libc::c_uint);
    if dev.is_null() { return dev; }
    if (*dev).attached && kcomedilib_set_link_from_to(from as libc::c_uint, minor as libc::c_uint) { dev } else { comedi_dev_put(dev); core::ptr::null_mut() }
}

pub unsafe extern "C" fn comedi_close_from(dev: *mut comedi_device, from: libc::c_int) -> libc::c_int { kcomedilib_clear_link_from_to(from as libc::c_uint, (*dev).minor); comedi_dev_put(dev); 0 }

unsafe fn comedi_do_insn(dev: *mut comedi_device, insn: *mut comedi_insn, data: *mut libc::c_uint) -> libc::c_int {
    if !(*dev).attached || (*insn).subdev >= (*dev).n_subdevices { return -EINVAL; }
    let s = (*dev).subdevices.add((*insn).subdev as usize);
    if (*s).type_ == COMEDI_SUBD_UNUSED || (*s).busy.is_null() == false { return -EIO; }
    let ret = match (*insn).insn {
        INSN_BITS => (*s).insn_bits.map(|f| f(dev, s, insn, data)).unwrap_or(-EINVAL),
        INSN_CONFIG => (*s).insn_config.map(|f| f(dev, s, insn, data)).unwrap_or(-EINVAL),
        _ => -EINVAL,
    };
    (*s).busy = core::ptr::null_mut(); ret
}

pub unsafe extern "C" fn comedi_dio_get_config(dev: *mut comedi_device, subdev: libc::c_uint, chan: libc::c_uint, io: *mut libc::c_uint) -> libc::c_int {
    let mut insn = comedi_insn { insn: INSN_CONFIG, n: 2, subdev, chanspec: chan };
    let mut data = [INSN_CONFIG_DIO_QUERY, 0]; let ret = comedi_do_insn(dev, &mut insn, data.as_mut_ptr()); if ret >= 0 { *io = data[1]; } ret
}
pub unsafe extern "C" fn comedi_dio_config(dev: *mut comedi_device, subdev: libc::c_uint, chan: libc::c_uint, io: libc::c_uint) -> libc::c_int {
    let mut insn = comedi_insn { insn: INSN_CONFIG, n: 1, subdev, chanspec: chan }; let mut data = io; comedi_do_insn(dev, &mut insn, &mut data)
}
pub unsafe extern "C" fn comedi_dio_bitfield2(dev: *mut comedi_device, subdev: libc::c_uint, mask: libc::c_uint, bits: *mut libc::c_uint, base_channel: libc::c_uint) -> libc::c_int {
    let n = comedi_get_n_channels(dev, subdev); if base_channel >= n as libc::c_uint { return -EINVAL; }
    let shift = if n <= 32 { base_channel } else { 0 }; let mut insn = comedi_insn { insn: INSN_BITS, n: 2, subdev, chanspec: if shift != 0 { 0 } else { base_channel } }; let mut data = [mask << shift, *bits << shift]; let ret = comedi_do_insn(dev, &mut insn, data.as_mut_ptr()); *bits = data[1] >> shift; ret
}
pub unsafe extern "C" fn comedi_find_subdevice_by_type(dev: *mut comedi_device, type_: libc::c_int, mut subd: libc::c_uint) -> libc::c_int { while subd < (*dev).n_subdevices { if (*(*dev).subdevices.add(subd as usize)).type_ == type_ { return subd as libc::c_int; } subd += 1; } -ENODEV }
pub unsafe extern "C" fn comedi_get_n_channels(dev: *mut comedi_device, subdevice: libc::c_uint) -> libc::c_int { if (*dev).attached && subdevice < (*dev).n_subdevices { (*(*dev).subdevices.add(subdevice as usize)).n_chan } else { 0 } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
