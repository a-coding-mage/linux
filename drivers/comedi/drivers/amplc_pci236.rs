// SPDX-License-Identifier: GPL-2.0+
/*
 * comedi/drivers/amplc_pci236.c
 * Driver for Amplicon PCI236 DIO boards.
 *
 * Copyright (C) 2002-2014 MEV Ltd. <https://www.mev.co.uk/>
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 2000 David A. Schleef <ds@schleef.org>
 */
/*
 * Driver: amplc_pci236
 * Description: Amplicon PCI236
 * Author: Ian Abbott <abbotti@mev.co.uk>
 * Devices: [Amplicon] PCI236 (amplc_pci236)
 * Updated: Fri, 25 Jul 2014 15:32:40 +0000
 * Status: works
 *
 * Configuration options:
 *   none
 *
 * Manual configuration of PCI board (PCI236) is not supported; it is
 * configured automatically.
 *
 * The PCI236 board has a single 8255 appearing as subdevice 0.
 *
 * Subdevice 1 pretends to be a digital input device, but it always
 * returns 0 when read. However, if you run a command with
 * scan_begin_src=TRIG_EXT, a rising edge on port C bit 3 acts as an
 * external trigger, which can be used to wake up tasks.  This is like
 * the comedi_parport device.  If no interrupt is connected, then
 * subdevice 1 is unused.
 */

// Linux/Comedi headers and local headers are supplied by other translation units.

/* Disable, and clear, interrupts */
pub const PCI236_INTR_DISABLE: u32 = PLX9052_INTCSR_LI1POL
    | PLX9052_INTCSR_LI2POL
    | PLX9052_INTCSR_LI1SEL
    | PLX9052_INTCSR_LI1CLRINT;

/* Enable, and clear, interrupts */
pub const PCI236_INTR_ENABLE: u32 = PLX9052_INTCSR_LI1ENAB
    | PLX9052_INTCSR_LI1POL
    | PLX9052_INTCSR_LI2POL
    | PLX9052_INTCSR_PCIENAB
    | PLX9052_INTCSR_LI1SEL
    | PLX9052_INTCSR_LI1CLRINT;

unsafe extern "C" {
    static PLX9052_INTCSR: usize;
    static PLX9052_INTCSR_LI1POL: u32;
    static PLX9052_INTCSR_LI2POL: u32;
    static PLX9052_INTCSR_LI1SEL: u32;
    static PLX9052_INTCSR_LI1CLRINT: u32;
    static PLX9052_INTCSR_LI1ENAB: u32;
    static PLX9052_INTCSR_PCIENAB: u32;
    static PLX9052_INTCSR_LI1STAT: u32;
    static IRQF_SHARED: u32;
}

#[repr(C)]
pub struct comedi_device {
    pub private: *mut pc236_private,
    pub board_ptr: *const pc236_board,
    pub board_name: *const core::ffi::c_char,
    pub class_dev: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct pc236_private {
    pub lcr_iobase: usize,
    pub enable_irq: bool,
}

#[repr(C)]
pub struct pc236_board {
    pub name: *const core::ffi::c_char,
    pub intr_update_cb: Option<unsafe extern "C" fn(*mut comedi_device, bool)>,
    pub intr_chk_clr_cb: Option<unsafe extern "C" fn(*mut comedi_device) -> bool>,
}

#[repr(C)]
pub struct pci_dev {
    pub irq: i32,
}

#[repr(C)]
pub struct pci_device_id {
    pub driver_data: usize,
}

unsafe extern "C" {
    fn outl(value: u32, port: usize);
    fn inl(port: usize) -> u32;
    fn comedi_to_pci_dev(dev: *mut comedi_device) -> *mut pci_dev;
    fn comedi_alloc_devpriv(dev: *mut comedi_device, size: usize) -> *mut pc236_private;
    fn comedi_pci_enable(dev: *mut comedi_device) -> i32;
    fn pci_resource_start(dev: *mut pci_dev, bar: u32) -> usize;
    fn amplc_pc236_common_attach(dev: *mut comedi_device, iobase: usize, irq: i32, flags: u32) -> i32;
    fn comedi_pci_detach(dev: *mut comedi_device) -> i32;
    fn comedi_pci_auto_config(dev: *mut pci_dev, driver: *mut comedi_driver, data: usize) -> i32;
    fn comedi_pci_auto_unconfig(dev: *mut pci_dev);
}

#[repr(C)]
pub struct comedi_driver {
    pub driver_name: *const core::ffi::c_char,
    pub module: *mut core::ffi::c_void,
    pub auto_attach: Option<unsafe extern "C" fn(*mut comedi_device, usize) -> i32>,
    pub detach: Option<unsafe extern "C" fn(*mut comedi_device) -> i32>,
}

unsafe extern "C" fn pci236_intr_update_cb(dev: *mut comedi_device, enable: bool) {
    let devpriv = (*dev).private;
    /* this will also clear the "local interrupt 1" latch */
    outl(if enable { PCI236_INTR_ENABLE } else { PCI236_INTR_DISABLE },
         (*devpriv).lcr_iobase + PLX9052_INTCSR);
}

unsafe extern "C" fn pci236_intr_chk_clr_cb(dev: *mut comedi_device) -> bool {
    let devpriv = (*dev).private;
    /* check if interrupt occurred */
    if (inl((*devpriv).lcr_iobase + PLX9052_INTCSR) & PLX9052_INTCSR_LI1STAT) == 0 {
        return false;
    }
    /* clear the interrupt */
    pci236_intr_update_cb(dev, (*devpriv).enable_irq);
    true
}

pub static PC236_PCI_BOARD: pc236_board = pc236_board {
    name: b"pci236\0".as_ptr() as *const _,
    intr_update_cb: Some(pci236_intr_update_cb),
    intr_chk_clr_cb: Some(pci236_intr_chk_clr_cb),
};

unsafe extern "C" fn pci236_auto_attach(dev: *mut comedi_device, _context_unused: usize) -> i32 {
    let pci_dev = comedi_to_pci_dev(dev);
    let devpriv = comedi_alloc_devpriv(dev, core::mem::size_of::<pc236_private>());
    if devpriv.is_null() {
        return -12; // -ENOMEM
    }
    (*dev).board_ptr = &PC236_PCI_BOARD;
    (*dev).board_name = PC236_PCI_BOARD.name;
    let ret = comedi_pci_enable(dev);
    if ret != 0 { return ret; }
    (*devpriv).lcr_iobase = pci_resource_start(pci_dev, 1);
    let iobase = pci_resource_start(pci_dev, 2);
    amplc_pc236_common_attach(dev, iobase, (*pci_dev).irq, IRQF_SHARED)
}

pub static mut AMPLC_PCI236_DRIVER: comedi_driver = comedi_driver {
    driver_name: b"amplc_pci236\0".as_ptr() as *const _,
    module: core::ptr::null_mut(),
    auto_attach: Some(pci236_auto_attach),
    detach: Some(comedi_pci_detach),
};

pub static PCI236_PCI_TABLE: [pci_device_id; 2] = [
    pci_device_id { driver_data: 0x0009 },
    pci_device_id { driver_data: 0 },
];

unsafe extern "C" fn amplc_pci236_pci_probe(dev: *mut pci_dev, id: *const pci_device_id) -> i32 {
    comedi_pci_auto_config(dev, &raw mut AMPLC_PCI236_DRIVER, (*id).driver_data)
}

#[repr(C)]
pub struct pci_driver {
    pub name: *const core::ffi::c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut pci_dev)>,
}

pub static AMPLC_PCI236_PCI_DRIVER: pci_driver = pci_driver {
    name: b"amplc_pci236\0".as_ptr() as *const _,
    id_table: PCI236_PCI_TABLE.as_ptr(),
    probe: Some(amplc_pci236_pci_probe),
    remove: Some(comedi_pci_auto_unconfig),
};

// module_comedi_pci_driver(amplc_pci236_driver, amplc_pci236_pci_driver);
// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_DESCRIPTION("Comedi driver for Amplicon PCI236 DIO boards");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
