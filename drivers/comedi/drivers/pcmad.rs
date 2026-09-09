// SPDX-License-Identifier: GPL-2.0+
/*
 * pcmad.c
 * Hardware driver for Winsystems PCM-A/D12 and PCM-A/D16
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 2000,2001 David A. Schleef <ds@schleef.org>
 */

/*
 * Driver: pcmad
 * Description: Winsystems PCM-A/D12, PCM-A/D16
 * Devices: [Winsystems] PCM-A/D12 (pcmad12), PCM-A/D16 (pcmad16)
 * Author: ds
 * Status: untested
 *
 * This driver was written on a bet that I couldn't write a driver
 * in less than 2 hours.  I won the bet, but never got paid.  =(
 *
 * Configuration options:
 *   [0] - I/O port base
 *   [1] - IRQ (unused)
 *   [2] - Analog input reference (must match jumpers)
 *       0 = single-ended (16 channels)
 *       1 = differential (8 channels)
 *   [3] - Analog input encoding (must match jumpers)
 *       0 = straight binary (0-5V input range)
 *       1 = two's complement (+-10V input range)
 */

// Linux and Comedi declarations are supplied by the surrounding translation unit.

const PCMAD_STATUS: usize = 0;
const PCMAD_LSB: usize = 1;
const PCMAD_MSB: usize = 2;
const PCMAD_CONVERT: usize = 1;

#[repr(C)]
struct pcmad_board_struct {
    name: *const core::ffi::c_char,
    ai_maxdata: u32,
}

static pcmad_boards: [pcmad_board_struct; 2] = [
    pcmad_board_struct {
        name: b"pcmad12\0".as_ptr() as *const core::ffi::c_char,
        ai_maxdata: 0x0fff,
    },
    pcmad_board_struct {
        name: b"pcmad16\0".as_ptr() as *const core::ffi::c_char,
        ai_maxdata: 0xffff,
    },
];

unsafe fn pcmad_ai_eoc(
    dev: *mut comedi_device,
    _s: *mut comedi_subdevice,
    _insn: *mut comedi_insn,
    _context: c_ulong,
) -> c_int {
    let status: c_uint = inb((*dev).iobase + PCMAD_STATUS as c_ulong) as c_uint;
    if (status & 0x3) == 0x3 {
        return 0;
    }
    -EBUSY
}

unsafe fn pcmad_ai_insn_read(
    dev: *mut comedi_device,
    s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut c_uint,
) -> c_int {
    let chan: c_uint = CR_CHAN((*insn).chanspec);
    let range: c_uint = CR_RANGE((*insn).chanspec);
    let mut val: c_uint;
    let ret: c_int;
    let mut i: c_uint = 0;

    while i < (*insn).n {
        outb(chan as u8, (*dev).iobase + PCMAD_CONVERT as c_ulong);

        ret = comedi_timeout(dev, s, insn, Some(pcmad_ai_eoc), 0);
        if ret != 0 {
            return ret;
        }

        val = inb((*dev).iobase + PCMAD_LSB as c_ulong) as c_uint
            | ((inb((*dev).iobase + PCMAD_MSB as c_ulong) as c_uint) << 8);

        /* data is shifted on the pcmad12, fix it */
        if (*s).maxdata == 0x0fff {
            val >>= 4;
        }

        if comedi_range_is_bipolar(s, range) {
            /* munge the two's complement value */
            val ^= ((*s).maxdata + 1) >> 1;
        }

        *data.add(i as usize) = val;
        i += 1;
    }

    (*insn).n as c_int
}

unsafe fn pcmad_attach(
    dev: *mut comedi_device,
    it: *mut comedi_devconfig,
) -> c_int {
    let board: *const pcmad_board_struct = (*dev).board_ptr as *const pcmad_board_struct;
    let s: *mut comedi_subdevice;
    let ret: c_int;

    ret = comedi_check_request_region(dev, (*it).options[0], 0x04, 0, 0x3ff, 4);
    if ret != 0 {
        return ret;
    }

    ret = comedi_alloc_subdevices(dev, 1);
    if ret != 0 {
        return ret;
    }

    s = (*dev).subdevices;
    (*s).type_ = COMEDI_SUBD_AI;
    if (*it).options[1] != 0 {
        /* 8 differential channels */
        (*s).subdev_flags = SDF_READABLE | AREF_DIFF;
        (*s).n_chan = 8;
    } else {
        /* 16 single-ended channels */
        (*s).subdev_flags = SDF_READABLE | AREF_GROUND;
        (*s).n_chan = 16;
    }
    (*s).len_chanlist = 1;
    (*s).maxdata = (*board).ai_maxdata;
    (*s).range_table = if (*it).options[2] != 0 {
        &range_bipolar10
    } else {
        &range_unipolar5
    };
    (*s).insn_read = Some(pcmad_ai_insn_read);

    0
}

// The comedi driver registration and module metadata are provided by the kernel integration layer.
#[allow(dead_code)]
unsafe fn pcmad_driver_definition() -> comedi_driver {
    comedi_driver {
        driver_name: b"pcmad\0".as_ptr() as *const c_char,
        module: THIS_MODULE,
        attach: Some(pcmad_attach),
        detach: Some(comedi_legacy_detach),
        board_name: &pcmad_boards[0].name,
        num_names: pcmad_boards.len(),
        offset: core::mem::size_of::<pcmad_board_struct>(),
    }
}

// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_DESCRIPTION("Comedi low-level driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
