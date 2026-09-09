// SPDX-License-Identifier: GPL-2.0+
/*
 * rti802.c
 * Comedi driver for Analog Devices RTI-802 board
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1999 Anders Blomdell <anders.blomdell@control.lth.se>
 */

/*
 * Driver: rti802
 * Description: Analog Devices RTI-802
 * Author: Anders Blomdell <anders.blomdell@control.lth.se>
 * Devices: [Analog Devices] RTI-802 (rti802)
 * Status: works
 *
 * Configuration Options:
 *   [0] - i/o base
 *   [1] - unused
 *   [2,4,6,8,10,12,14,16] - dac#[0-7]  0=two's comp, 1=straight
 *   [3,5,7,9,11,13,15,17] - dac#[0-7]  0=bipolar, 1=unipolar
 */

// Dependencies supplied by the surrounding Comedi/Linux environment.

pub const RTI802_SELECT: u16 = 0x00;
pub const RTI802_DATALOW: u16 = 0x01;
pub const RTI802_DATAHIGH: u16 = 0x02;

#[repr(C)]
pub struct Rti802Private {
    pub dac_coding: [DacCoding; 8],
    pub range_type_list: [*const ComediLrange; 8],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DacCoding {
    Dac2comp,
    DacStraight,
}

#[repr(C)]
pub struct ComediDevice {
    pub private: *mut Rti802Private,
    pub iobase: usize,
    pub subdevices: *mut ComediSubdevice,
}

#[repr(C)]
pub struct ComediSubdevice {
    pub readback: *mut u32,
    pub type_: u32,
    pub subdev_flags: u32,
    pub maxdata: u32,
    pub n_chan: u32,
    pub insn_write: Option<unsafe extern "C" fn(*mut ComediDevice, *mut ComediSubdevice, *mut ComediInsn, *mut u32) -> i32>,
    pub range_table_list: *mut *const ComediLrange,
}

#[repr(C)]
pub struct ComediInsn {
    pub chanspec: u32,
    pub n: u32,
}

#[repr(C)]
pub struct ComediDevconfig {
    pub options: [u32; 18],
}

#[repr(C)]
pub struct ComediLrange;

extern "C" {
    pub fn comedi_check_request_region(dev: *mut ComediDevice, base: u32, len: u32, unused: u32, max: u32, align: u32) -> i32;
    pub fn comedi_alloc_devpriv(dev: *mut ComediDevice, size: usize) -> *mut Rti802Private;
    pub fn comedi_alloc_subdevices(dev: *mut ComediDevice, n: u32) -> i32;
    pub fn comedi_alloc_subdev_readback(s: *mut ComediSubdevice) -> i32;
    pub fn comedi_offset_munge(s: *mut ComediSubdevice, val: u32) -> u32;
    pub fn outb(value: u8, port: usize);
    pub static range_unipolar10: ComediLrange;
    pub static range_bipolar10: ComediLrange;
}

pub const COMEDI_SUBD_AO: u32 = 0;
pub const SDF_WRITABLE: u32 = 0;

#[inline]
unsafe fn cr_chan(chanspec: u32) -> usize {
    (chanspec & 0xff) as usize
}

pub unsafe extern "C" fn rti802_ao_insn_write(
    dev: *mut ComediDevice,
    s: *mut ComediSubdevice,
    insn: *mut ComediInsn,
    data: *mut u32,
) -> i32 {
    let devpriv = (*dev).private;
    let chan = cr_chan((*insn).chanspec);

    outb(chan as u8, (*dev).iobase + RTI802_SELECT as usize);

    for i in 0..(*insn).n as usize {
        let mut val = *data.add(i);

        *(*s).readback.add(chan) = val;

        // munge offset binary to two's complement if needed
        if (*devpriv).dac_coding[chan] == DacCoding::Dac2comp {
            val = comedi_offset_munge(s, val);
        }

        outb((val & 0xff) as u8, (*dev).iobase + RTI802_DATALOW as usize);
        outb(((val >> 8) & 0xff) as u8, (*dev).iobase + RTI802_DATAHIGH as usize);
    }

    (*insn).n as i32
}

pub unsafe extern "C" fn rti802_attach(
    dev: *mut ComediDevice,
    it: *mut ComediDevconfig,
) -> i32 {
    let devpriv: *mut Rti802Private;
    let s: *mut ComediSubdevice;

    let ret = comedi_check_request_region(dev, (*it).options[0], 0x04, 0, 0x3ff, 4);
    if ret != 0 {
        return ret;
    }

    devpriv = comedi_alloc_devpriv(dev, core::mem::size_of::<Rti802Private>());
    if devpriv.is_null() {
        return -12; // -ENOMEM
    }

    let ret = comedi_alloc_subdevices(dev, 1);
    if ret != 0 {
        return ret;
    }

    // Analog Output subdevice
    s = (*dev).subdevices;
    (*s).type_ = COMEDI_SUBD_AO;
    (*s).subdev_flags = SDF_WRITABLE;
    (*s).maxdata = 0xfff;
    (*s).n_chan = 8;
    (*s).insn_write = Some(rti802_ao_insn_write);

    let ret = comedi_alloc_subdev_readback(s);
    if ret != 0 {
        return ret;
    }

    (*s).range_table_list = (*devpriv).range_type_list.as_mut_ptr();
    for i in 0..8 {
        (*devpriv).dac_coding[i] = if (*it).options[3 + 2 * i] != 0 {
            DacCoding::DacStraight
        } else {
            DacCoding::Dac2comp
        };
        (*devpriv).range_type_list[i] = if (*it).options[2 + 2 * i] != 0 {
            &range_unipolar10
        } else {
            &range_bipolar10
        };
    }

    0
}

// The C module registration and metadata are supplied by the kernel/module integration layer.
// module_comedi_driver(rti802_driver);
// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_DESCRIPTION("Comedi driver for Analog Devices RTI-802 board");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
