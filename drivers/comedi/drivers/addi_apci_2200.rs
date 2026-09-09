// SPDX-License-Identifier: GPL-2.0+
/*
 * addi_apci_2200.c
 * Copyright (C) 2004,2005  ADDI-DATA GmbH for the source code of this module.
 * Project manager: Eric Stolz
 *
 *\tADDI-DATA GmbH
 *\tDieselstrasse 3
 *\tD-77833 Ottersweier
 *\tTel: +19(0)7223/9493-0
 *\tFax: +49(0)7223/9493-92
 *\thttp://www.addi-data.com
 *\tinfo@addi-data.com
 */

// External Linux/comedi declarations and the addi_watchdog dependency are supplied elsewhere.

const APCI2200_DI_REG: usize = 0x00;
const APCI2200_DO_REG: usize = 0x04;
const APCI2200_WDOG_REG: usize = 0x08;

extern "C" {
    fn inw(port: usize) -> u16;
    fn outw(value: u16, port: usize);
    fn comedi_to_pci_dev(dev: *mut comedi_device) -> *mut pci_dev;
    fn comedi_pci_enable(dev: *mut comedi_device) -> i32;
    fn pci_resource_start(dev: *mut pci_dev, bar: u32) -> usize;
    fn comedi_alloc_subdevices(dev: *mut comedi_device, n: u32) -> i32;
    fn comedi_dio_update_state(s: *mut comedi_subdevice, data: *mut u32) -> i32;
    fn addi_watchdog_reset(base: usize);
    fn addi_watchdog_init(s: *mut comedi_subdevice, base: usize) -> i32;
    fn comedi_pci_detach(dev: *mut comedi_device);
    fn comedi_pci_auto_config(dev: *mut pci_dev, drv: *mut comedi_driver, data: usize) -> i32;
    fn comedi_pci_auto_unconfig(dev: *mut pci_dev);
}

#[repr(C)]
pub struct comedi_device {
    pub iobase: usize,
    pub subdevices: *mut comedi_subdevice,
}

#[repr(C)]
pub struct comedi_subdevice {
    pub state: u16,
    pub r#type: u32,
    pub subdev_flags: u32,
    pub n_chan: u32,
    pub maxdata: u32,
    pub range_table: *mut range_table,
    pub insn_bits: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_subdevice, *mut comedi_insn, *mut u32) -> i32>,
}

#[repr(C)]
pub struct comedi_insn { pub n: u32 }
#[repr(C)]
pub struct pci_dev;
#[repr(C)]
pub struct pci_device_id { pub driver_data: usize }
#[repr(C)]
pub struct range_table;
#[repr(C)]
pub struct comedi_driver {
    pub driver_name: *const u8,
    pub module: *mut core::ffi::c_void,
    pub auto_attach: Option<unsafe extern "C" fn(*mut comedi_device, usize) -> i32>,
    pub detach: Option<unsafe extern "C" fn(*mut comedi_device)>,
}
#[repr(C)]
pub struct pci_driver {
    pub name: *const u8,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut pci_dev)>,
}

extern "C" {
    static mut range_digital: range_table;
}

const COMEDI_SUBD_DI: u32 = 1;
const COMEDI_SUBD_DO: u32 = 2;
const SDF_READABLE: u32 = 0x0001;
const SDF_WRITABLE: u32 = 0x0002;

unsafe extern "C" fn apci2200_di_insn_bits(
    dev: *mut comedi_device, _s: *mut comedi_subdevice,
    insn: *mut comedi_insn, data: *mut u32,
) -> i32 {
    *data.add(1) = inw((*dev).iobase + APCI2200_DI_REG) as u32;
    (*insn).n as i32
}

unsafe extern "C" fn apci2200_do_insn_bits(
    dev: *mut comedi_device, s: *mut comedi_subdevice,
    insn: *mut comedi_insn, data: *mut u32,
) -> i32 {
    (*s).state = inw((*dev).iobase + APCI2200_DO_REG);
    if comedi_dio_update_state(s, data) != 0 {
        outw((*s).state, (*dev).iobase + APCI2200_DO_REG);
    }
    *data.add(1) = (*s).state as u32;
    (*insn).n as i32
}

unsafe extern "C" fn apci2200_reset(dev: *mut comedi_device) -> i32 {
    outw(0, (*dev).iobase + APCI2200_DO_REG);
    addi_watchdog_reset((*dev).iobase + APCI2200_WDOG_REG);
    0
}

unsafe extern "C" fn apci2200_auto_attach(dev: *mut comedi_device, _context_unused: usize) -> i32 {
    let pcidev = comedi_to_pci_dev(dev);
    let mut s: *mut comedi_subdevice;
    let mut ret = comedi_pci_enable(dev);
    if ret != 0 { return ret; }
    (*dev).iobase = pci_resource_start(pcidev, 1);
    ret = comedi_alloc_subdevices(dev, 3);
    if ret != 0 { return ret; }

    // Initialize the digital input subdevice
    s = (*dev).subdevices;
    (*s).r#type = COMEDI_SUBD_DI;
    (*s).subdev_flags = SDF_READABLE;
    (*s).n_chan = 8;
    (*s).maxdata = 1;
    (*s).range_table = &mut range_digital;
    (*s).insn_bits = Some(apci2200_di_insn_bits);

    // Initialize the digital output subdevice
    s = (*dev).subdevices.add(1);
    (*s).r#type = COMEDI_SUBD_DO;
    (*s).subdev_flags = SDF_WRITABLE;
    (*s).n_chan = 16;
    (*s).maxdata = 1;
    (*s).range_table = &mut range_digital;
    (*s).insn_bits = Some(apci2200_do_insn_bits);

    // Initialize the watchdog subdevice
    s = (*dev).subdevices.add(2);
    ret = addi_watchdog_init(s, (*dev).iobase + APCI2200_WDOG_REG);
    if ret != 0 { return ret; }
    apci2200_reset(dev);
    0
}

unsafe extern "C" fn apci2200_detach(dev: *mut comedi_device) {
    if (*dev).iobase != 0 { apci2200_reset(dev); }
    comedi_pci_detach(dev);
}

static mut apci2200_driver: comedi_driver = comedi_driver {
    driver_name: b"addi_apci_2200\0".as_ptr(),
    module: core::ptr::null_mut(),
    auto_attach: Some(apci2200_auto_attach),
    detach: Some(apci2200_detach),
};

unsafe extern "C" fn apci2200_pci_probe(dev: *mut pci_dev, id: *const pci_device_id) -> i32 {
    comedi_pci_auto_config(dev, &mut apci2200_driver, (*id).driver_data)
}

static apci2200_pci_table: [pci_device_id; 2] = [
    pci_device_id { driver_data: 0 },
    pci_device_id { driver_data: 0 },
];

static mut apci2200_pci_driver: pci_driver = pci_driver {
    name: b"addi_apci_2200\0".as_ptr(),
    id_table: apci2200_pci_table.as_ptr(),
    probe: Some(apci2200_pci_probe),
    remove: Some(comedi_pci_auto_unconfig),
};

// MODULE_DEVICE_TABLE(pci, apci2200_pci_table);
// module_comedi_pci_driver(apci2200_driver, apci2200_pci_driver);
// MODULE_DESCRIPTION("ADDI-DATA APCI-2200 Relay board, optically isolated");
// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
