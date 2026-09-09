// SPDX-License-Identifier: GPL-2.0+
/*
 *  das08_pci.c
 *  comedi driver for DAS08 PCI boards
 *
 *  COMEDI - Linux Control and Measurement Device Interface
 *  Copyright (C) 2000 David A. Schleef <ds@schleef.org>
 *  Copyright (C) 2001,2002,2003 Frank Mori Hess <fmhess@users.sourceforge.net>
 *  Copyright (C) 2004 Salvador E. Tropea <set@users.sf.net> <set@ieee.org>
 */

/*
 * Driver: das08_pci
 * Description: DAS-08 PCI compatible boards
 * Devices: [ComputerBoards] PCI-DAS08 (pci-das08)
 * Author: Warren Jasper, ds, Frank Hess
 * Updated: Fri, 31 Aug 2012 19:19:06 +0100
 * Status: works
 *
 * This is the PCI-specific support split off from the das08 driver.
 *
 * Configuration Options: not applicable, uses PCI auto config
 */

// C dependencies: linux/module.h, linux/comedi/comedi_pci.h, and das08.h.

static DAS08_BOARD_STRUCT das08_pci_boards: [DAS08_BOARD_STRUCT; 1] = [
    DAS08_BOARD_STRUCT {
        name: b"pci-das08\0".as_ptr() as *const i8,
        ai_nbits: 12,
        ai_pg: das08_bipolar5,
        ai_encoding: das08_encode12,
        di_nchan: 3,
        do_nchan: 4,
        i8254_offset: 4,
        iosize: 8,
    },
];

unsafe extern "C" {
    static THIS_MODULE: *mut core::ffi::c_void;

    fn comedi_alloc_devpriv(
        dev: *mut comedi_device,
        size: usize,
    ) -> *mut das08_private_struct;
    fn comedi_to_pci_dev(dev: *mut comedi_device) -> *mut pci_dev;
    fn comedi_pci_enable(dev: *mut comedi_device) -> i32;
    fn pci_resource_start(dev: *mut pci_dev, bar: u32) -> usize;
    fn das08_common_attach(dev: *mut comedi_device, iobase: usize) -> i32;
    fn comedi_pci_detach(dev: *mut comedi_device) -> i32;
    fn comedi_pci_auto_config(
        dev: *mut pci_dev,
        driver: *mut comedi_driver,
        driver_data: usize,
    ) -> i32;
    fn comedi_pci_auto_unconfig(dev: *mut pci_dev) -> i32;
}

#[allow(non_camel_case_types)]
type c_int = i32;

#[repr(C)]
struct comedi_device {
    board_ptr: *const core::ffi::c_void,
    iobase: usize,
}

#[repr(C)]
struct pci_dev;

#[repr(C)]
struct pci_device_id {
    vendor: u32,
    device: u32,
    subvendor: u32,
    subdevice: u32,
    class: u32,
    class_mask: u32,
    driver_data: usize,
}

#[repr(C)]
struct das08_private_struct;

#[repr(C)]
struct DAS08_BOARD_STRUCT {
    name: *const i8,
    ai_nbits: u32,
    ai_pg: u32,
    ai_encoding: u32,
    di_nchan: u32,
    do_nchan: u32,
    i8254_offset: u32,
    iosize: u32,
}

#[repr(C)]
struct comedi_driver {
    driver_name: *const i8,
    module: *mut core::ffi::c_void,
    auto_attach: Option<unsafe extern "C" fn(*mut comedi_device, usize) -> c_int>,
    detach: Option<unsafe extern "C" fn(*mut comedi_device) -> c_int>,
}

unsafe extern "C" {
    static das08_bipolar5: u32;
    static das08_encode12: u32;
}

const ENOMEM: c_int = 12;

unsafe extern "C" fn das08_pci_auto_attach(
    dev: *mut comedi_device,
    _context_unused: usize,
) -> c_int {
    let pdev = comedi_to_pci_dev(dev);
    let devpriv = comedi_alloc_devpriv(dev, core::mem::size_of::<das08_private_struct>());
    if devpriv.is_null() {
        return -ENOMEM;
    }

    /* The das08 driver needs the board_ptr */
    (*dev).board_ptr = das08_pci_boards.as_ptr() as *const core::ffi::c_void;

    let ret = comedi_pci_enable(dev);
    if ret != 0 {
        return ret;
    }
    (*dev).iobase = pci_resource_start(pdev, 2);

    das08_common_attach(dev, (*dev).iobase)
}

static mut das08_pci_comedi_driver: comedi_driver = comedi_driver {
    driver_name: b"pci-das08\0".as_ptr() as *const i8,
    module: core::ptr::addr_of_mut!(THIS_MODULE) as *mut core::ffi::c_void,
    auto_attach: Some(das08_pci_auto_attach),
    detach: Some(comedi_pci_detach),
};

unsafe extern "C" fn das08_pci_probe(
    dev: *mut pci_dev,
    id: *const pci_device_id,
) -> c_int {
    comedi_pci_auto_config(
        dev,
        core::ptr::addr_of_mut!(das08_pci_comedi_driver),
        (*id).driver_data,
    )
}

static das08_pci_table: [pci_device_id; 2] = [
    pci_device_id {
        vendor: 0x1307,
        device: 0x0029,
        subvendor: 0,
        subdevice: 0,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    },
    pci_device_id {
        vendor: 0,
        device: 0,
        subvendor: 0,
        subdevice: 0,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    },
];

#[repr(C)]
struct pci_driver {
    name: *const i8,
    id_table: *const pci_device_id,
    probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut pci_dev) -> c_int>,
}

static mut das08_pci_driver: pci_driver = pci_driver {
    name: b"pci-das08\0".as_ptr() as *const i8,
    id_table: das08_pci_table.as_ptr(),
    probe: Some(das08_pci_probe),
    remove: Some(comedi_pci_auto_unconfig),
};

// MODULE_DEVICE_TABLE(pci, das08_pci_table);
// module_comedi_pci_driver(das08_pci_comedi_driver, das08_pci_driver);
// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_DESCRIPTION("Comedi low-level driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
