// SPDX-License-Identifier: GPL-2.0+
/*
 * pcmda12.c
 * Driver for Winsystems PC-104 based PCM-D/A-12 8-channel AO board.
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 2006 Calin A. Culianu <calin@ajvar.org>
 */

/*
 * Driver: pcmda12
 * Description: A driver for the Winsystems PCM-D/A-12
 * Devices: [Winsystems] PCM-D/A-12 (pcmda12)
 * Author: Calin Culianu <calin@ajvar.org>
 * Updated: Fri, 13 Jan 2006 12:01:01 -0500
 * Status: works
 *
 * A driver for the relatively straightforward-to-program PCM-D/A-12.
 * This board doesn't support commands, and the only way to set its
 * analog output range is to jumper the board. As such,
 * comedi_data_write() ignores the range value specified.
 *
 * The board uses 16 consecutive I/O addresses starting at the I/O port
 * base address. Each address corresponds to the LSB then MSB of a
 * particular channel from 0-7.
 *
 * Note that the board is not ISA-PNP capable and thus needs the I/O
 * port comedi_config parameter.
 *
 * Note that passing a nonzero value as the second config option will
 * enable "simultaneous xfer" mode for this board, in which AO writes
 * will not take effect until a subsequent read of any AO channel. This
 * is so that one can speed up programming by preloading all AO registers
 * with values before simultaneously setting them to take effect with one
 * read command.
 */

// Dependencies supplied by the surrounding Comedi/Linux bindings.
use core::ffi::c_void;

#[repr(C)]
pub struct ComediLrange {
    pub length: u32,
    pub range: [u32; 3],
}

#[repr(C)]
pub struct Pcmda12Private {
    pub simultaneous_xfer_mode: i32,
}

#[repr(C)]
pub struct ComediDevice {
    pub private: *mut Pcmda12Private,
    pub iobase: usize,
    pub subdevices: *mut ComediSubdevice,
}

#[repr(C)]
pub struct ComediSubdevice {
    pub type_: u32,
    pub subdev_flags: u32,
    pub n_chan: i32,
    pub maxdata: u32,
    pub range_table: *const ComediLrange,
    pub readback: *mut u32,
    pub insn_write: Option<unsafe extern "C" fn(*mut ComediDevice, *mut ComediSubdevice, *mut ComediInsn, *mut u32) -> i32>,
    pub insn_read: Option<unsafe extern "C" fn(*mut ComediDevice, *mut ComediSubdevice, *mut ComediInsn, *mut u32) -> i32>,
}

#[repr(C)]
pub struct ComediInsn {
    pub chanspec: u32,
    pub n: u32,
}

#[repr(C)]
pub struct ComediDevconfig {
    pub options: *const u32,
}

extern "C" {
    fn comedi_check_request_region(dev: *mut ComediDevice, from: u32, len: u32,
                                    min: u32, max: u32, align: u32) -> i32;
    fn comedi_alloc_devpriv(dev: *mut ComediDevice, size: usize) -> *mut Pcmda12Private;
    fn comedi_alloc_subdevices(dev: *mut ComediDevice, num: u32) -> i32;
    fn comedi_alloc_subdev_readback(s: *mut ComediSubdevice) -> i32;
    fn comedi_readback_insn_read(dev: *mut ComediDevice, s: *mut ComediSubdevice,
                                 insn: *mut ComediInsn, data: *mut u32) -> i32;
    fn outb(value: u8, port: usize);
    fn inb(port: usize) -> u8;
}

const ENOMEM: i32 = 12;
const COMEDI_SUBD_AO: u32 = 2;
const SDF_READABLE: u32 = 1 << 0;
const SDF_WRITABLE: u32 = 1 << 1;

// AI range is not configurable, it's set by jumpers on the board.
static PCMDA12_RANGES: ComediLrange = ComediLrange {
    length: 3,
    range: [5, 10, 0xffff_fffbu32], // UNI_RANGE(5), UNI_RANGE(10), BIP_RANGE(5)
};

unsafe fn pcmda12_ao_insn_write(
    dev: *mut ComediDevice,
    s: *mut ComediSubdevice,
    insn: *mut ComediInsn,
    data: *mut u32,
) -> i32 {
    let devpriv = (*dev).private;
    let chan = ((*insn).chanspec & 0xff) as usize;
    let mut val = *(*s).readback.add(chan);
    let ioreg = (*dev).iobase.wrapping_add(chan.wrapping_mul(2));
    let mut i = 0u32;
    while i < (*insn).n {
        val = *data.add(i as usize);
        outb((val & 0xff) as u8, ioreg);
        outb(((val >> 8) & 0xff) as u8, ioreg.wrapping_add(1));

        /* Initiate transfer if not in simultaneous xfer mode. */
        if (*devpriv).simultaneous_xfer_mode == 0 {
            inb(ioreg);
        }
        i = i.wrapping_add(1);
    }
    *(*s).readback.add(chan) = val;
    (*insn).n as i32
}

unsafe fn pcmda12_ao_insn_read(
    dev: *mut ComediDevice,
    s: *mut ComediSubdevice,
    insn: *mut ComediInsn,
    data: *mut u32,
) -> i32 {
    let devpriv = (*dev).private;
    /* Initiate simultaneous xfer mode by reading one AO register. */
    if (*devpriv).simultaneous_xfer_mode != 0 {
        inb((*dev).iobase);
    }
    comedi_readback_insn_read(dev, s, insn, data)
}

unsafe fn pcmda12_ao_reset(dev: *mut ComediDevice, s: *mut ComediSubdevice) {
    let mut i = 0i32;
    while i < (*s).n_chan {
        let port = (*dev).iobase.wrapping_add((i as usize).wrapping_mul(2));
        outb(0, port);
        outb(0, port.wrapping_add(1));
        i += 1;
    }
    /* Initiate transfer by reading one of the AO registers. */
    inb((*dev).iobase);
}

unsafe fn pcmda12_attach(dev: *mut ComediDevice, it: *mut ComediDevconfig) -> i32 {
    let ret = comedi_check_request_region(dev, *(*it).options, 0x10, 0, 0x3ff, 32);
    if ret != 0 { return ret; }

    let devpriv = comedi_alloc_devpriv(dev, core::mem::size_of::<Pcmda12Private>());
    if devpriv.is_null() { return -ENOMEM; }
    (*devpriv).simultaneous_xfer_mode = *(*it).options.add(1) as i32;

    let ret = comedi_alloc_subdevices(dev, 1);
    if ret != 0 { return ret; }
    let s = (*dev).subdevices;
    (*s).type_ = COMEDI_SUBD_AO;
    (*s).subdev_flags = SDF_READABLE | SDF_WRITABLE;
    (*s).n_chan = 8;
    (*s).maxdata = 0x0fff;
    (*s).range_table = &PCMDA12_RANGES;
    (*s).insn_write = Some(pcmda12_ao_insn_write);
    (*s).insn_read = Some(pcmda12_ao_insn_read);

    let ret = comedi_alloc_subdev_readback(s);
    if ret != 0 { return ret; }
    pcmda12_ao_reset(dev, s);
    0
}

// module_comedi_driver(pcmda12_driver);
// The Linux module metadata and registration are supplied by the bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
