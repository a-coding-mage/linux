// SPDX-License-Identifier: GPL-2.0
/*
 * pcl724.c
 * Comedi driver for 8255 based ISA and PC/104 DIO boards
 *
 * Michal Dobes <dobes@tesnet.cz>
 */

/*
 * Driver: pcl724
 * Description: Comedi driver for 8255 based ISA DIO boards
 * Devices: [Advantech] PCL-724 (pcl724), PCL-722 (pcl722), PCL-731 (pcl731),
 *  [ADLink] ACL-7122 (acl7122), ACL-7124 (acl7124), PET-48DIO (pet48dio),
 *  [WinSystems] PCM-IO48 (pcmio48),
 *  [Diamond Systems] ONYX-MM-DIO (onyx-mm-dio)
 * Author: Michal Dobes <dobes@tesnet.cz>
 * Status: untested
 *
 * Configuration options:
 *   [0] - IO Base
 *   [1] - IRQ (not supported)
 *   [2] - number of DIO (pcl722 and acl7122 boards)
 *       0, 144: 144 DIO configuration
 *       1,  96: 96 DIO configuration
 */

#[repr(C)]
pub struct pcl724_board {
    pub name: *const core::ffi::c_char,
    pub io_range: u32,
    pub min_io_start: u32,
    pub max_io_end: u32,
    pub can_have96: u32,
    pub is_pet48: u32,
    pub numofports: i32,
}

pub static boardtypes: [pcl724_board; 8] = [
    pcl724_board { name: b"pcl724\0".as_ptr() as *const _, io_range: 0x04, min_io_start: 0x200, max_io_end: 0x3ff, can_have96: 0, is_pet48: 0, numofports: 1 },
    pcl724_board { name: b"pcl722\0".as_ptr() as *const _, io_range: 0x20, min_io_start: 0x200, max_io_end: 0x3ff, can_have96: 1, is_pet48: 0, numofports: 6 },
    pcl724_board { name: b"pcl731\0".as_ptr() as *const _, io_range: 0x08, min_io_start: 0, max_io_end: 0x3ff, can_have96: 0, is_pet48: 0, numofports: 2 },
    pcl724_board { name: b"acl7122\0".as_ptr() as *const _, io_range: 0x20, min_io_start: 0x200, max_io_end: 0x3ff, can_have96: 1, is_pet48: 0, numofports: 6 },
    pcl724_board { name: b"acl7124\0".as_ptr() as *const _, io_range: 0x04, min_io_start: 0x200, max_io_end: 0x3ff, can_have96: 0, is_pet48: 0, numofports: 1 },
    pcl724_board { name: b"pet48dio\0".as_ptr() as *const _, io_range: 0x02, min_io_start: 0, max_io_end: 0x3ff, can_have96: 0, is_pet48: 1, numofports: 2 },
    pcl724_board { name: b"pcmio48\0".as_ptr() as *const _, io_range: 0x08, min_io_start: 0x100, max_io_end: 0x17f, can_have96: 0, is_pet48: 0, numofports: 2 },
    pcl724_board { name: b"onyx-mm-dio\0".as_ptr() as *const _, io_range: 0x10, min_io_start: 0, max_io_end: 0x3ff, can_have96: 0, is_pet48: 0, numofports: 2 },
];

extern "C" {
    fn outb(value: u8, port: u16);
    fn inb(port: u16) -> u8;
    fn comedi_check_request_region(dev: *mut comedi_device, start: u32, len: u32, min: u32, max: u32, align: u32) -> i32;
    fn comedi_alloc_subdevices(dev: *mut comedi_device, n: i32) -> i32;
    fn subdev_8255_cb_init(dev: *mut comedi_device, s: *mut comedi_subdevice, cb: unsafe extern "C" fn(*mut comedi_device, i32, i32, i32, usize) -> i32, iobase: usize) -> i32;
    fn subdev_8255_io_init(dev: *mut comedi_device, s: *mut comedi_subdevice, offset: i32) -> i32;
}

#[repr(C)] pub struct comedi_device { pub board_ptr: *const pcl724_board, pub iobase: usize, pub n_subdevices: i32, pub subdevices: *mut comedi_subdevice }
#[repr(C)] pub struct comedi_subdevice { _private: [u8; 0] }
#[repr(C)] pub struct comedi_devconfig { pub options: [u32; 4] }

const I8255_SIZE: usize = 4;

pub unsafe extern "C" fn pcl724_8255mapped_io(dev: *mut comedi_device, dir: i32, port: i32, data: i32, iobase: usize) -> i32 {
    let movport = I8255_SIZE * (iobase >> 12);
    let base = (iobase & 0x0fff) as u16;
    outb((port as usize + movport) as u8, base);
    if dir != 0 { outb(data as u8, base + 1); return 0; }
    inb(base + 1) as i32
}

pub unsafe extern "C" fn pcl724_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> i32 {
    let board = (*dev).board_ptr;
    let mut iorange = (*board).io_range;
    let mut n_subdevices = (*board).numofports;
    if (*board).can_have96 != 0 && ((*it).options[2] == 1 || (*it).options[2] == 96) { iorange = 0x10; n_subdevices = 4; }
    let mut ret = comedi_check_request_region(dev, (*it).options[0], iorange, (*board).min_io_start, (*board).max_io_end, iorange);
    if ret != 0 { return ret; }
    ret = comedi_alloc_subdevices(dev, n_subdevices);
    if ret != 0 { return ret; }
    for i in 0..(*dev).n_subdevices {
        let s = (*dev).subdevices.add(i as usize);
        if (*board).is_pet48 != 0 { ret = subdev_8255_cb_init(dev, s, pcl724_8255mapped_io, (*dev).iobase + (i as usize * 0x1000)); }
        else { ret = subdev_8255_io_init(dev, s, i * I8255_SIZE as i32); }
        if ret != 0 { return ret; }
    }
    0
}

// Kernel module registration and metadata are supplied by the surrounding Comedi/Rust integration.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
