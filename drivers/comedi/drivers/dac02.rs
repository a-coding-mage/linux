// SPDX-License-Identifier: GPL-2.0+
/*
 * dac02.c
 * Comedi driver for DAC02 compatible boards
 * Copyright (C) 2014 H Hartley Sweeten <hsweeten@visionengravers.com>
 *
 * Based on the poc driver
 * Copyright (C) 2000 Frank Mori Hess <fmhess@users.sourceforge.net>
 * Copyright (C) 2001 David A. Schleef <ds@schleef.org>
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1998 David A. Schleef <ds@schleef.org>
 */

/*
 * Driver: dac02
 * Description: Comedi driver for DAC02 compatible boards
 * Devices: [Keithley Metrabyte] DAC-02 (dac02)
 * Author: H Hartley Sweeten <hsweeten@visionengravers.com>
 * Updated: Tue, 11 Mar 2014 11:27:19 -0700
 * Status: unknown
 *
 * Configuration options:
 * [0] - I/O port base
 */

// External Linux/Comedi declarations supplied by other translation units.

/*
 * The output range is selected by jumpering pins on the I/O connector.
 *
 *     Range      Chan #   Jumper pins        Output
 * -------------  ------  -------------  -----------------
 *    0 to 5V       0        21 to 22      24
 *                  1        15 to 16      18
 *    0 to 10V      0        20 to 22      24
 *                  1        14 to 16      18
 *     +/-5V        0        21 to 22      23
 *                  1        15 to 16      17
 *     +/-10V       0        20 to 22      23
 *                  1        14 to 16      17
 *   4 to 20mA      0        21 to 22      25
 *                  1        15 to 16      19
 * AC reference     0      In on pin 22    24 (2-quadrant)
 *                         In on pin 22    23 (4-quadrant)
 *                  1      In on pin 16    18 (2-quadrant)
 *                         In on pin 16    17 (4-quadrant)
 */
static const das02_ao_ranges: comedi_lrange = comedi_lrange {
    length: 6,
    range: [
        UNI_RANGE(5),
        UNI_RANGE(10),
        BIP_RANGE(5),
        BIP_RANGE(10),
        RANGE_mA(4, 20),
        RANGE_ext(0, 1),
    ],
};

/* Register I/O map */
const fn dac02_ao_lsb(x: usize) -> usize { 0x00 + (x * 2) }
const fn dac02_ao_msb(x: usize) -> usize { 0x01 + (x * 2) }

unsafe fn dac02_ao_insn_write(
    dev: *mut comedi_device,
    s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut c_uint,
) -> c_int {
    let chan: c_uint = CR_CHAN((*insn).chanspec);
    let range: c_uint = CR_RANGE((*insn).chanspec);
    let mut val: c_uint;
    let mut i: c_int = 0;

    while i < (*insn).n {
        val = *data.add(i as usize);
        (*s).readback[chan as usize] = val;

        /*
         * Unipolar outputs are true binary encoding.
         * Bipolar outputs are complementary offset binary
         * (that is, 0 = +full scale, maxdata = -full scale).
         */
        if comedi_range_is_bipolar(s, range) {
            val = (*s).maxdata - val;
        }

        /*
         * DACs are double-buffered.
         * Write LSB then MSB to latch output.
         */
        outb((val << 4) & 0xf0, (*dev).iobase + dac02_ao_lsb(chan as usize));
        outb((val >> 4) & 0xff, (*dev).iobase + dac02_ao_msb(chan as usize));
        i += 1;
    }

    (*insn).n
}

unsafe fn dac02_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> c_int {
    let mut s: *mut comedi_subdevice;
    let mut ret: c_int;

    ret = comedi_check_request_region(dev, (*it).options[0], 0x08, 0x200, 0x3ff, 8);
    if ret != 0 {
        return ret;
    }

    ret = comedi_alloc_subdevices(dev, 1);
    if ret != 0 {
        return ret;
    }

    /* Analog Output subdevice */
    s = (*dev).subdevices.add(0);
    (*s).type = COMEDI_SUBD_AO;
    (*s).subdev_flags = SDF_WRITABLE;
    (*s).n_chan = 2;
    (*s).maxdata = 0x0fff;
    (*s).range_table = &das02_ao_ranges;
    (*s).insn_write = Some(dac02_ao_insn_write);

    comedi_alloc_subdev_readback(s)
}

static mut dac02_driver: comedi_driver = comedi_driver {
    driver_name: b"dac02\\0".as_ptr() as *const c_char,
    module: THIS_MODULE,
    attach: Some(dac02_attach),
    detach: Some(comedi_legacy_detach),
};

module_comedi_driver!(dac02_driver);

MODULE_AUTHOR!("H Hartley Sweeten <hsweeten@visionengravers.com>");
MODULE_DESCRIPTION!("Comedi driver for DAC02 compatible boards");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
