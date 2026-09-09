// SPDX-License-Identifier: GPL-2.0
/*
 * comedi/drivers/pcl730.c
 * Driver for Advantech PCL-730 and clones
 * José Luis Sánchez
 */

/*
 * Driver: pcl730
 * Description: Advantech PCL-730 (& compatibles)
 * Devices: [Advantech] PCL-730 (pcl730), PCM-3730 (pcm3730), PCL-725 (pcl725),
 *   PCL-733 (pcl733), PCL-734 (pcl734),
 *   [ADLink] ACL-7130 (acl7130), ACL-7225b (acl7225b),
 *   [ICP] ISO-730 (iso730), P8R8-DIO (p8r8dio), P16R16-DIO (p16r16dio),
 *   [Diamond Systems] OPMM-1616-XT (opmm-1616-xt), PEARL-MM-P (pearl-mm-p),
 *   IR104-PBF (ir104-pbf),
 * Author: José Luis Sánchez (jsanchezv@teleline.es)
 * Status: untested
 *
 * Configuration options:
 *   [0] - I/O port base
 *
 * Interrupts are not supported.
 * The ACL-7130 card has an 8254 timer/counter not supported by this driver.
 */

/* Includes supplied by the kernel/comedi environment. */

#[repr(C)]
pub struct pcl730_board {
    pub name: *const core::ffi::c_char,
    pub io_range: u16,
    pub min_io_start: u16,
    pub align_io_start: u16,
    pub is_pcl725: u32,
    pub is_acl7225b: u32,
    pub is_ir104: u32,
    pub has_readback: u32,
    pub has_ttl_io: u32,
    pub n_subdevs: i32,
    pub n_iso_out_chan: i32,
    pub n_iso_in_chan: i32,
    pub n_ttl_chan: i32,
}

static pcl730_boards: [pcl730_board; 13] = [
    pcl730_board { name: c"pcl730".as_ptr(), io_range: 0x04, min_io_start: 0, align_io_start: 0x04, is_pcl725: 0, is_acl7225b: 0, is_ir104: 0, has_readback: 0, has_ttl_io: 1, n_subdevs: 4, n_iso_out_chan: 16, n_iso_in_chan: 16, n_ttl_chan: 16 },
    pcl730_board { name: c"iso730".as_ptr(), io_range: 0x04, min_io_start: 0, align_io_start: 0x04, is_pcl725: 0, is_acl7225b: 0, is_ir104: 0, has_readback: 0, has_ttl_io: 0, n_subdevs: 4, n_iso_out_chan: 16, n_iso_in_chan: 16, n_ttl_chan: 16 },
    pcl730_board { name: c"acl7130".as_ptr(), io_range: 0x08, min_io_start: 0x200, align_io_start: 0x08, is_pcl725: 0, is_acl7225b: 0, is_ir104: 0, has_readback: 0, has_ttl_io: 1, n_subdevs: 4, n_iso_out_chan: 16, n_iso_in_chan: 16, n_ttl_chan: 16 },
    pcl730_board { name: c"pcm3730".as_ptr(), io_range: 0x04, min_io_start: 0, align_io_start: 0x04, is_pcl725: 0, is_acl7225b: 0, is_ir104: 0, has_readback: 0, has_ttl_io: 1, n_subdevs: 4, n_iso_out_chan: 8, n_iso_in_chan: 8, n_ttl_chan: 16 },
    pcl730_board { name: c"pcl725".as_ptr(), io_range: 0x02, min_io_start: 0x200, align_io_start: 0x02, is_pcl725: 1, is_acl7225b: 0, is_ir104: 0, has_readback: 0, has_ttl_io: 0, n_subdevs: 2, n_iso_out_chan: 8, n_iso_in_chan: 8, n_ttl_chan: 0 },
    pcl730_board { name: c"p8r8dio".as_ptr(), io_range: 0x02, min_io_start: 0, align_io_start: 0x10, is_pcl725: 1, is_acl7225b: 0, is_ir104: 0, has_readback: 1, has_ttl_io: 0, n_subdevs: 2, n_iso_out_chan: 8, n_iso_in_chan: 8, n_ttl_chan: 0 },
    pcl730_board { name: c"acl7225b".as_ptr(), io_range: 0x08, min_io_start: 0x200, align_io_start: 0x08, is_pcl725: 0, is_acl7225b: 1, is_ir104: 0, has_readback: 1, has_ttl_io: 0, n_subdevs: 2, n_iso_out_chan: 16, n_iso_in_chan: 16, n_ttl_chan: 0 },
    pcl730_board { name: c"p16r16dio".as_ptr(), io_range: 0x04, min_io_start: 0, align_io_start: 0x08, is_pcl725: 0, is_acl7225b: 1, is_ir104: 0, has_readback: 1, has_ttl_io: 0, n_subdevs: 2, n_iso_out_chan: 16, n_iso_in_chan: 16, n_ttl_chan: 0 },
    pcl730_board { name: c"pcl733".as_ptr(), io_range: 0x04, min_io_start: 0, align_io_start: 0x04, is_pcl725: 0, is_acl7225b: 0, is_ir104: 0, has_readback: 0, has_ttl_io: 0, n_subdevs: 1, n_iso_out_chan: 0, n_iso_in_chan: 32, n_ttl_chan: 0 },
    pcl730_board { name: c"pcl734".as_ptr(), io_range: 0x04, min_io_start: 0, align_io_start: 0x04, is_pcl725: 0, is_acl7225b: 0, is_ir104: 0, has_readback: 0, has_ttl_io: 0, n_subdevs: 1, n_iso_out_chan: 32, n_iso_in_chan: 0, n_ttl_chan: 0 },
    pcl730_board { name: c"opmm-1616-xt".as_ptr(), io_range: 0x10, min_io_start: 0x100, align_io_start: 0x10, is_pcl725: 0, is_acl7225b: 1, is_ir104: 0, has_readback: 1, has_ttl_io: 0, n_subdevs: 2, n_iso_out_chan: 16, n_iso_in_chan: 16, n_ttl_chan: 0 },
    pcl730_board { name: c"pearl-mm-p".as_ptr(), io_range: 0x02, min_io_start: 0x240, align_io_start: 0x40, is_pcl725: 0, is_acl7225b: 0, is_ir104: 0, has_readback: 0, has_ttl_io: 0, n_subdevs: 1, n_iso_out_chan: 16, n_iso_in_chan: 0, n_ttl_chan: 0 },
    pcl730_board { name: c"ir104-pbf".as_ptr(), io_range: 0x08, min_io_start: 0x240, align_io_start: 0x20, is_pcl725: 0, is_acl7225b: 0, is_ir104: 1, has_readback: 1, has_ttl_io: 0, n_subdevs: 2, n_iso_out_chan: 20, n_iso_in_chan: 20, n_ttl_chan: 0 },
];

/* The following external types/functions are supplied by the comedi environment. */
extern "C" {
    fn comedi_dio_update_state(s: *mut comedi_subdevice, data: *mut u32) -> u32;
    fn outb(value: u8, port: usize);
    fn inb(port: usize) -> u8;
    fn comedi_check_request_region(dev: *mut comedi_device, start: u32, len: u16, min: u16, max: u16, align: u16) -> i32;
    fn comedi_alloc_subdevices(dev: *mut comedi_device, n: i32) -> i32;
}

#[repr(C)] pub struct comedi_device { pub board_ptr: *const pcl730_board, pub iobase: usize, pub subdevices: *mut comedi_subdevice, pub board_name: *const core::ffi::c_char, pub class_dev: *mut core::ffi::c_void }
#[repr(C)] pub struct comedi_subdevice { pub type_: u32, pub subdev_flags: u32, pub n_chan: u32, pub maxdata: u32, pub range_table: *const core::ffi::c_void, pub insn_bits: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_subdevice, *mut comedi_insn, *mut u32) -> i32>, pub private: *mut core::ffi::c_void, pub state: u32 }
#[repr(C)] pub struct comedi_insn { pub n: u32 }
#[repr(C)] pub struct comedi_devconfig { pub options: [u32; 4] }

unsafe extern "C" fn pcl730_do_insn_bits(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let reg = (*s).private as usize;
    let mask = comedi_dio_update_state(s, data);
    if mask != 0 {
        if mask & 0x00ff != 0 { outb(((*s).state & 0xff) as u8, (*dev).iobase + reg); }
        if mask & 0xff00 != 0 && (*s).n_chan > 8 { outb(((*s).state >> 8) as u8, (*dev).iobase + reg + 1); }
        if mask & 0xff0000 != 0 && (*s).n_chan > 16 { outb(((*s).state >> 16) as u8, (*dev).iobase + reg + 2); }
        if mask & 0xff000000 != 0 && (*s).n_chan > 24 { outb(((*s).state >> 24) as u8, (*dev).iobase + reg + 3); }
    }
    *data.add(1) = (*s).state;
    (*insn).n as i32
}

unsafe fn pcl730_get_bits(dev: *mut comedi_device, s: *mut comedi_subdevice) -> u32 {
    let reg = (*s).private as usize;
    let mut val = inb((*dev).iobase + reg) as u32;
    if (*s).n_chan > 8 { val |= (inb((*dev).iobase + reg + 1) as u32) << 8; }
    if (*s).n_chan > 16 { val |= (inb((*dev).iobase + reg + 2) as u32) << 16; }
    if (*s).n_chan > 24 { val |= (inb((*dev).iobase + reg + 3) as u32) << 24; }
    val
}

unsafe extern "C" fn pcl730_di_insn_bits(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    *data.add(1) = pcl730_get_bits(dev, s);
    (*insn).n as i32
}

unsafe extern "C" fn pcl730_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> i32 {
    let board = (*dev).board_ptr;
    let iobase = (*it).options[0];
    if (*board).is_ir104 != 0 && iobase != 0x240 && iobase != 0x260 && iobase != 0x280 && iobase != 0x300 { return -22; }
    let ret = comedi_check_request_region(dev, iobase, (*board).io_range, (*board).min_io_start, 0x3ff, (*board).align_io_start);
    if ret != 0 { return ret; }
    let ret = comedi_alloc_subdevices(dev, (*board).n_subdevs);
    if ret != 0 { return ret; }
    let mut subdev = 0;
    if (*board).n_iso_out_chan != 0 {
        let s = (*dev).subdevices.add(subdev); subdev += 1;
        (*s).type_ = 2; (*s).subdev_flags = 0x02; (*s).n_chan = (*board).n_iso_out_chan as u32; (*s).maxdata = 1; (*s).insn_bits = Some(pcl730_do_insn_bits); (*s).private = core::ptr::null_mut();
        if (*board).has_readback != 0 { (*s).state = pcl730_get_bits(dev, s); }
    }
    if (*board).n_iso_in_chan != 0 {
        let s = (*dev).subdevices.add(subdev); subdev += 1;
        (*s).type_ = 1; (*s).subdev_flags = 0x01; (*s).n_chan = (*board).n_iso_in_chan as u32; (*s).maxdata = 1; (*s).insn_bits = Some(pcl730_di_insn_bits);
        (*s).private = if (*board).is_ir104 != 0 { 4usize } else if (*board).is_acl7225b != 0 { 2 } else if (*board).is_pcl725 != 0 { 1 } else { 0 } as *mut core::ffi::c_void;
    }
    if (*board).has_ttl_io != 0 {
        let s = (*dev).subdevices.add(subdev); subdev += 1;
        (*s).type_ = 2; (*s).subdev_flags = 0x02; (*s).n_chan = (*board).n_ttl_chan as u32; (*s).maxdata = 1; (*s).insn_bits = Some(pcl730_do_insn_bits); (*s).private = 2usize as *mut core::ffi::c_void;
        let s = (*dev).subdevices.add(subdev);
        (*s).type_ = 1; (*s).subdev_flags = 0x01; (*s).n_chan = (*board).n_ttl_chan as u32; (*s).maxdata = 1; (*s).insn_bits = Some(pcl730_di_insn_bits); (*s).private = 2usize as *mut core::ffi::c_void;
    }
    0
}

#[repr(C)] struct comedi_driver { driver_name: *const core::ffi::c_char, attach: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_devconfig) -> i32>, board_name: *const *const core::ffi::c_char, num_names: usize, offset: usize }
static mut pcl730_driver: comedi_driver = comedi_driver { driver_name: c"pcl730".as_ptr(), attach: Some(pcl730_attach), board_name: unsafe { &pcl730_boards[0].name }, num_names: 13, offset: core::mem::size_of::<pcl730_board>() };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
