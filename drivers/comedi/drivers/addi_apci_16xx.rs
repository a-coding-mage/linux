// SPDX-License-Identifier: GPL-2.0+
/*
 * addi_apci_16xx.c
 * Copyright (C) 2004,2005  ADDI-DATA GmbH for the source code of this module.
 * Project manager: S. Weber
 *
 * ADDI-DATA GmbH
 * Dieselstrasse 3
 * D-77833 Ottersweier
 * Tel: +19(0)7223/9493-0
 * Fax: +49(0)7223/9493-92
 * http://www.addi-data.com
 * info@addi-data.com
 */

// Register I/O map
#[inline]
const fn apci16xx_in_reg(x: usize) -> usize { x * 4 + 0x08 }
#[inline]
const fn apci16xx_out_reg(x: usize) -> usize { x * 4 + 0x14 }
#[inline]
const fn apci16xx_dir_reg(x: usize) -> usize { x * 4 + 0x20 }

#[repr(C)]
pub enum Apci16xxBoardId {
    BoardApci1648,
    BoardApci1696,
}

#[repr(C)]
pub struct Apci16xxBoardinfo {
    pub name: *const core::ffi::c_char,
    pub n_chan: i32,
}

pub static APCI16XX_BOARDTYPES: [Apci16xxBoardinfo; 2] = [
    Apci16xxBoardinfo {
        name: b"apci1648\0".as_ptr() as *const core::ffi::c_char,
        n_chan: 48, // 2 subdevices
    },
    Apci16xxBoardinfo {
        name: b"apci1696\0".as_ptr() as *const core::ffi::c_char,
        n_chan: 96, // 3 subdevices
    },
];

extern "C" {
    fn CR_CHAN(chanspec: u32) -> u32;
    fn comedi_dio_insn_config(
        dev: *mut comedi_device,
        s: *mut comedi_subdevice,
        insn: *mut comedi_insn,
        data: *mut u32,
        mask: u32,
    ) -> i32;
    fn outl(value: u32, address: usize);
    fn inl(address: usize) -> u32;
    fn comedi_dio_update_state(s: *mut comedi_subdevice, data: *mut u32) -> i32;
    fn comedi_to_pci_dev(dev: *mut comedi_device) -> *mut pci_dev;
    fn comedi_pci_enable(dev: *mut comedi_device) -> i32;
    fn pci_resource_start(dev: *mut pci_dev, bar: u32) -> usize;
    fn comedi_alloc_subdevices(dev: *mut comedi_device, n: u32) -> i32;
    static range_digital: range_table;
}

#[repr(C)]
pub struct comedi_device {
    pub board_ptr: *const core::ffi::c_void,
    pub board_name: *const core::ffi::c_char,
    pub iobase: usize,
    pub subdevices: *mut comedi_subdevice,
}
#[repr(C)] pub struct comedi_subdevice {
    pub type_: u32, pub subdev_flags: u32, pub n_chan: u32, pub maxdata: u32,
    pub range_table: *const range_table, pub insn_config: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_subdevice, *mut comedi_insn, *mut u32) -> i32>,
    pub insn_bits: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_subdevice, *mut comedi_insn, *mut u32) -> i32>,
    pub io_bits: u32, pub state: u32, pub index: u32,
}
#[repr(C)] pub struct comedi_insn { pub chanspec: u32, pub n: u32 }
#[repr(C)] pub struct range_table;
#[repr(C)] pub struct pci_dev;

#[allow(non_snake_case)]
pub unsafe extern "C" fn apci16xx_insn_config(
    dev: *mut comedi_device, s: *mut comedi_subdevice,
    insn: *mut comedi_insn, data: *mut u32,
) -> i32 {
    let chan = CR_CHAN((*insn).chanspec);
    let mask = if chan < 8 { 0x000000ff } else if chan < 16 { 0x0000ff00 }
        else if chan < 24 { 0x00ff0000 } else { 0xff000000 };
    let ret = comedi_dio_insn_config(dev, s, insn, data, mask);
    if ret != 0 { return ret; }
    outl((*s).io_bits, (*dev).iobase + apci16xx_dir_reg((*s).index as usize));
    (*insn).n as i32
}

pub unsafe extern "C" fn apci16xx_dio_insn_bits(
    dev: *mut comedi_device, s: *mut comedi_subdevice,
    insn: *mut comedi_insn, data: *mut u32,
) -> i32 {
    if comedi_dio_update_state(s, data) != 0 {
        outl((*s).state, (*dev).iobase + apci16xx_out_reg((*s).index as usize));
    }
    *data.add(1) = inl((*dev).iobase + apci16xx_in_reg((*s).index as usize));
    (*insn).n as i32
}

pub unsafe extern "C" fn apci16xx_auto_attach(
    dev: *mut comedi_device, context: usize,
) -> i32 {
    let pcidev = comedi_to_pci_dev(dev);
    let board = APCI16XX_BOARDTYPES.get(context);
    let board = match board { Some(b) => b as *const _, None => return -19 };
    (*dev).board_ptr = board as *const core::ffi::c_void;
    (*dev).board_name = (*board).name;
    let ret = comedi_pci_enable(dev); if ret != 0 { return ret; }
    (*dev).iobase = pci_resource_start(pcidev, 0);
    let mut n_subdevs = ((*board).n_chan / 32) as u32;
    let last;
    if n_subdevs * 32 < (*board).n_chan as u32 {
        last = (*board).n_chan as u32 - n_subdevs * 32; n_subdevs += 1;
    } else { last = 0; }
    let ret = comedi_alloc_subdevices(dev, n_subdevs); if ret != 0 { return ret; }
    for i in 0..n_subdevs {
        let s = &mut *(*dev).subdevices.add(i as usize);
        s.type_ = 2; // COMEDI_SUBD_DIO
        s.subdev_flags = 0x01 | 0x02; // SDF_WRITABLE | SDF_READABLE
        s.n_chan = if i * 32 < (*board).n_chan as u32 { 32 } else { last };
        s.maxdata = 1; s.range_table = &range_digital;
        s.insn_config = Some(apci16xx_insn_config); s.insn_bits = Some(apci16xx_dio_insn_bits);
        s.io_bits = 0;
        outl(s.io_bits, (*dev).iobase + apci16xx_dir_reg(i as usize));
    }
    0
}

#[repr(C)]
pub struct comedi_driver {
    pub driver_name: *const core::ffi::c_char,
    pub module: *mut core::ffi::c_void,
    pub auto_attach: Option<unsafe extern "C" fn(*mut comedi_device, usize) -> i32>,
    pub detach: Option<unsafe extern "C" fn(*mut comedi_device) -> i32>,
}
pub static mut APCI16XX_DRIVER: comedi_driver = comedi_driver {
    driver_name: b"addi_apci_16xx\0".as_ptr() as *const core::ffi::c_char,
    module: core::ptr::null_mut(), // THIS_MODULE
    auto_attach: Some(apci16xx_auto_attach),
    detach: None, // comedi_pci_detach
};

#[repr(C)]
pub struct pci_device_id {
    pub vendor: u32,
    pub device: u32,
    pub driver_data: usize,
}
pub static APCI16XX_PCI_TABLE: [pci_device_id; 3] = [
    pci_device_id { vendor: 0x10e8, device: 0x1009, driver_data: Apci16xxBoardId::BoardApci1648 as usize },
    pci_device_id { vendor: 0x10e8, device: 0x100a, driver_data: Apci16xxBoardId::BoardApci1696 as usize },
    pci_device_id { vendor: 0, device: 0, driver_data: 0 },
];

#[repr(C)]
pub struct pci_driver {
    pub name: *const core::ffi::c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut pci_dev) -> i32>,
}

pub unsafe extern "C" fn apci16xx_pci_probe(
    _dev: *mut pci_dev, _id: *const pci_device_id,
) -> i32 {
    // comedi_pci_auto_config(dev, &apci16xx_driver, id->driver_data)
    0
}

pub static APCI16XX_PCI_DRIVER: pci_driver = pci_driver {
    name: b"addi_apci_16xx\0".as_ptr() as *const core::ffi::c_char,
    id_table: APCI16XX_PCI_TABLE.as_ptr(),
    probe: Some(apci16xx_pci_probe),
    remove: None, // comedi_pci_auto_unconfig
};

// MODULE_DEVICE_TABLE(pci, apci16xx_pci_table)
// module_comedi_pci_driver(apci16xx_driver, apci16xx_pci_driver)
// MODULE_DESCRIPTION("ADDI-DATA APCI-1648/1696, TTL I/O boards")
// MODULE_AUTHOR("Comedi https://www.comedi.org")
// MODULE_LICENSE("GPL")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
