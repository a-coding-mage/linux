// SPDX-License-Identifier: GPL-2.0+
/*
 * Comedi driver for National Instruments PCMCIA DAQ-Card DIO-24
 * Copyright (C) 2002 Daniel Vecino Castel <dvecino@able.es>
 *
 * PCMCIA crap at end of file is adapted from dummy_cs.c 1.31
 * 2001/08/24 12:13:13 from the pcmcia package.
 * The initial developer of the pcmcia dummy_cs.c code is David A. Hinds
 * <dahinds@users.sourceforge.net>.  Portions created by David A. Hinds
 * are Copyright (C) 1999 David A. Hinds.  All Rights Reserved.
 */

/*
 * Driver: ni_daq_dio24
 * Description: National Instruments PCMCIA DAQ-Card DIO-24
 * Author: Daniel Vecino Castel <dvecino@able.es>
 * Devices: [National Instruments] PCMCIA DAQ-Card DIO-24 (ni_daq_dio24)
 * Status: ?
 * Updated: Thu, 07 Nov 2002 21:53:06 -0800
 *
 * This is just a wrapper around the 8255.o driver to properly handle
 * the PCMCIA interface.
 */

// Dependencies supplied by the surrounding kernel/comedi translation.
use core::ffi::c_void;

extern "C" {
    static THIS_MODULE: *mut c_void;

    fn comedi_to_pcmcia_dev(dev: *mut comedi_device) -> *mut pcmcia_device;
    fn comedi_pcmcia_enable(dev: *mut comedi_device, data: *mut c_void) -> i32;
    fn comedi_alloc_subdevices(dev: *mut comedi_device, n: u32) -> i32;
    fn subdev_8255_io_init(
        dev: *mut comedi_device,
        s: *mut comedi_subdevice,
        io_offset: u32,
    ) -> i32;
    fn comedi_pcmcia_disable(dev: *mut comedi_device) -> i32;
    fn comedi_pcmcia_auto_config(link: *mut pcmcia_device, driver: *mut comedi_driver) -> i32;
    fn comedi_pcmcia_auto_unconfig(link: *mut pcmcia_device) -> i32;

}

#[repr(C)]
pub struct resource {
    pub start: usize,
}

#[repr(C)]
pub struct pcmcia_device {
    pub config_flags: u32,
    pub resource: [*mut resource; 1],
}

#[repr(C)]
pub struct comedi_subdevice {
    _private: [u8; 0],
}

#[repr(C)]
pub struct comedi_device {
    pub iobase: usize,
    pub subdevices: *mut comedi_subdevice,
}

#[repr(C)]
pub struct comedi_driver {
    pub driver_name: *const u8,
    pub module: *mut c_void,
    pub auto_attach: Option<unsafe extern "C" fn(*mut comedi_device, u64) -> i32>,
    pub detach: Option<unsafe extern "C" fn(*mut comedi_device) -> i32>,
}

#[repr(C)]
pub struct pcmcia_device_id {
    pub manf: u16,
    pub card: u16,
}

#[repr(C)]
pub struct pcmcia_driver {
    pub name: *const u8,
    pub owner: *mut c_void,
    pub id_table: *const pcmcia_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pcmcia_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut pcmcia_device) -> i32>,
}

const CONF_AUTO_SET_IO: u32 = 1 << 0;

unsafe extern "C" fn dio24_auto_attach(dev: *mut comedi_device, _context: u64) -> i32 {
    let link = comedi_to_pcmcia_dev(dev);
    let s: *mut comedi_subdevice;
    let ret: i32;

    (*link).config_flags |= CONF_AUTO_SET_IO;
    ret = comedi_pcmcia_enable(dev, core::ptr::null_mut());
    if ret != 0 {
        return ret;
    }
    (*dev).iobase = (*(*link).resource[0]).start;

    ret = comedi_alloc_subdevices(dev, 1);
    if ret != 0 {
        return ret;
    }

    /* 8255 dio */
    s = (*dev).subdevices;
    subdev_8255_io_init(dev, s, 0x00)
}

static mut driver_dio24: comedi_driver = comedi_driver {
    driver_name: b"ni_daq_dio24\0".as_ptr(),
    module: THIS_MODULE,
    auto_attach: Some(dio24_auto_attach),
    detach: Some(comedi_pcmcia_disable),
};

unsafe extern "C" fn dio24_cs_attach(link: *mut pcmcia_device) -> i32 {
    comedi_pcmcia_auto_config(link, &raw mut driver_dio24)
}

static dio24_cs_ids: [pcmcia_device_id; 2] = [
    pcmcia_device_id { manf: 0x010b, card: 0x475c }, /* daqcard-dio24 */
    pcmcia_device_id { manf: 0, card: 0 },
];

static mut dio24_cs_driver: pcmcia_driver = pcmcia_driver {
    name: b"ni_daq_dio24\0".as_ptr(),
    owner: THIS_MODULE,
    id_table: dio24_cs_ids.as_ptr(),
    probe: Some(dio24_cs_attach),
    remove: Some(comedi_pcmcia_auto_unconfig),
};

// Equivalent of module_comedi_pcmcia_driver(driver_dio24, dio24_cs_driver);

// MODULE_AUTHOR("Daniel Vecino Castel <dvecino@able.es>");
// MODULE_DESCRIPTION("Comedi driver for National Instruments PCMCIA DAQ-Card DIO-24");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
