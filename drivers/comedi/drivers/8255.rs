// SPDX-License-Identifier: GPL-2.0+
/*
 * comedi/drivers/8255.c
 * Driver for 8255
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1998 David A. Schleef <ds@schleef.org>
 */

/*
 * Driver: 8255
 * Description: generic 8255 support
 * Devices: [standard] 8255 (8255)
 * Author: ds
 * Status: works
 * Updated: Fri,  7 Jun 2002 12:56:45 -0700
 *
 * The classic in digital I/O.  The 8255 appears in Comedi as a single
 * digital I/O subdevice with 24 channels.  The channel 0 corresponds
 * to the 8255's port A, bit 0; channel 23 corresponds to port C, bit
 * 7.  Direction configuration is done in blocks, with channels 0-7,
 * 8-15, 16-19, and 20-23 making up the 4 blocks.  The only 8255 mode
 * supported is mode 0.
 *
 * You should enable compilation this driver if you plan to use a board
 * that has an 8255 chip.  For multifunction boards, the main driver will
 * configure the 8255 subdevice automatically.
 *
 * This driver also works independently with ISA and PCI cards that
 * directly map the 8255 registers to I/O ports, including cards with
 * multiple 8255 chips.  To configure the driver for such a card, the
 * option list should be a list of the I/O port bases for each of the
 * 8255 chips.  For example,
 *
 *   comedi_config /dev/comedi0 8255 0x200,0x204,0x208,0x20c
 *
 * Note that most PCI 8255 boards do NOT work with this driver, and
 * need a separate driver as a wrapper.  For those that do work, the
 * I/O port base address can be found in the output of 'lspci -v'.
 */

// Linux/comedi dependencies are supplied by the surrounding crate.

unsafe extern "C" {
    fn dev_warn(dev: *mut core::ffi::c_void, fmt: *const core::ffi::c_char, ...);
    fn comedi_alloc_subdevices(dev: *mut comedi_device, n: i32) -> i32;
    fn __comedi_check_request_region(
        dev: *mut comedi_device,
        from: u32,
        len: u32,
        flags: u32,
        max: u32,
        align: u32,
    ) -> i32;
    fn subdev_8255_io_init(
        dev: *mut comedi_device,
        s: *mut comedi_subdevice,
        iobase: u32,
    ) -> i32;
    fn release_region(start: u32, n: u32);
    fn subdev_8255_regbase(s: *mut comedi_subdevice) -> u32;
}

unsafe extern "C" {
    type comedi_device;
    type comedi_devconfig;
    type comedi_subdevice;
    type comedi_driver;
}

#[repr(C)]
struct ComediDevconfigOptions {
    options: [u32; COMEDI_NDEVCONFOPTS as usize],
}

// The concrete definitions and constants below are provided by comedidev.h.
extern "C" {
    static THIS_MODULE: *mut core::ffi::c_void;
}

const EINVAL: i32 = 22;

unsafe extern "C" fn dev_8255_attach(
    dev: *mut comedi_device,
    it: *mut comedi_devconfig,
) -> i32 {
    let mut s: *mut comedi_subdevice;
    let mut iobase: u32;
    let mut ret: i32;
    let mut i: i32;

    i = 0;
    while i < COMEDI_NDEVCONFOPTS {
        iobase = (*it).options[i as usize];
        if iobase == 0 {
            break;
        }
        i += 1;
    }
    if i == 0 {
        dev_warn((*dev).class_dev, b"no devices specified\0".as_ptr() as *const _,);
        return -EINVAL;
    }

    ret = comedi_alloc_subdevices(dev, i);
    if ret != 0 {
        return ret;
    }

    i = 0;
    while i < (*dev).n_subdevices {
        s = (*dev).subdevices.add(i as usize);
        iobase = (*it).options[i as usize];

        /*
         * __comedi_check_request_region() does not set dev->iobase.
         *
         * For 8255 devices that are manually attached using
         * comedi_config, the 'iobase' is the actual I/O port
         * base address of the chip.  It should be aligned on a
         * 4-byte boundary.
         */
        ret = __comedi_check_request_region(dev, iobase, I8255_SIZE, 0, u32::MAX, 4);
        if ret != 0 {
            return ret;
        }
        ret = subdev_8255_io_init(dev, s, iobase);
        if ret != 0 {
            /* Release the I/O port region here, as the "detach" handler cannot find it. */
            release_region(iobase, I8255_SIZE);
            (*s).type_ = COMEDI_SUBD_UNUSED;
            return ret;
        }
        i += 1;
    }

    0
}

unsafe extern "C" fn dev_8255_detach(dev: *mut comedi_device) {
    let mut s: *mut comedi_subdevice;
    let mut i: i32 = 0;

    while i < (*dev).n_subdevices {
        s = (*dev).subdevices.add(i as usize);
        if (*s).type_ != COMEDI_SUBD_UNUSED {
            let regbase: u32 = subdev_8255_regbase(s);

            release_region(regbase, I8255_SIZE);
        }
        i += 1;
    }
}

static mut dev_8255_driver: comedi_driver = comedi_driver {
    driver_name: b"8255\0".as_ptr() as *const _,
    module: unsafe { THIS_MODULE },
    attach: Some(dev_8255_attach),
    detach: Some(dev_8255_detach),
};

// Equivalent of module_comedi_driver(dev_8255_driver);
// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_DESCRIPTION("Comedi driver for standalone 8255 devices");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
