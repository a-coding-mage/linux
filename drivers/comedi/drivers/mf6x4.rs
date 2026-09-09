// SPDX-License-Identifier: GPL-2.0+
/*
 *  comedi/drivers/mf6x4.c
 *  Driver for Humusoft MF634 and MF624 Data acquisition cards
 *
 *  COMEDI - Linux Control and Measurement Device Interface
 *  Copyright (C) 2000 David A. Schleef <ds@schleef.org>
 */
/*
 * Driver: mf6x4
 * Description: Humusoft MF634 and MF624 Data acquisition card driver
 * Devices: [Humusoft] MF634 (mf634), MF624 (mf624)
 * Author: Rostislav Lisovy <lisovy@gmail.com>
 * Status: works
 * Updated:
 * Configuration Options: none
 */

// Translated dependencies supplied by the surrounding kernel/comedi bindings.

const MF624_GPIOC_REG: usize = 0x54;
const MF6X4_GPIOC_EOLC: u32 = 1 << 17;
const MF6X4_GPIOC_LDAC: u32 = 1 << 23;
const MF6X4_GPIOC_DACEN: u32 = 1 << 26;
const MF6X4_ADDATA_REG: usize = 0x00;
const MF6X4_ADCTRL_REG: usize = 0x00;
const MF6X4_DIN_REG: usize = 0x10;
const MF6X4_DIN_MASK: u16 = 0xff;
const MF6X4_DOUT_REG: usize = 0x10;
const MF6X4_ADSTART_REG: usize = 0x20;
const MF634_GPIOC_REG: usize = 0x68;

#[inline]
const fn mf6x4_adctrl_chan(chan: u32) -> u16 { (1u16).wrapping_shl(chan) }
#[inline]
const fn mf6x4_dac_reg(x: usize) -> usize { 0x20 + x * 2 }

#[repr(C)]
enum mf6x4_boardid { BOARD_MF634, BOARD_MF624 }

#[repr(C)]
struct mf6x4_board {
    name: *const core::ffi::c_char,
    bar_nums: [u32; 3],
}

#[repr(C)]
struct mf6x4_private {
    bar0_mem: *mut core::ffi::c_void,
    bar2_mem: *mut core::ffi::c_void,
    gpioc_reg: *mut core::ffi::c_void,
}

extern "C" {
    static mf6x4_boards: [mf6x4_board; 2];
    fn ioread16(addr: *mut core::ffi::c_void) -> u16;
    fn ioread32(addr: *mut core::ffi::c_void) -> u32;
    fn iowrite16(value: u16, addr: *mut core::ffi::c_void);
    fn iowrite32(value: u32, addr: *mut core::ffi::c_void);
}

unsafe fn mf6x4_di_insn_bits(dev: *mut comedi_device, _s: *mut comedi_subdevice,
                              insn: *mut comedi_insn, data: *mut u32) -> i32 {
    (*data.add(1)) = ioread16((*dev).mmio.add(MF6X4_DIN_REG)) as u32 & MF6X4_DIN_MASK as u32;
    (*insn).n as i32
}

unsafe fn mf6x4_do_insn_bits(dev: *mut comedi_device, s: *mut comedi_subdevice,
                              insn: *mut comedi_insn, data: *mut u32) -> i32 {
    if comedi_dio_update_state(s, data) != 0 { iowrite16((*s).state as u16, (*dev).mmio.add(MF6X4_DOUT_REG)); }
    *data.add(1) = (*s).state;
    (*insn).n as i32
}

unsafe fn mf6x4_ai_eoc(dev: *mut comedi_device, _s: *mut comedi_subdevice,
                       _insn: *mut comedi_insn, _context: usize) -> i32 {
    let status = ioread32((*( (*dev).private as *mut mf6x4_private)).gpioc_reg);
    if status & MF6X4_GPIOC_EOLC == 0 { 0 } else { -16 }
}

unsafe fn mf6x4_ai_insn_read(dev: *mut comedi_device, s: *mut comedi_subdevice,
                             insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let chan = ((*insn).chanspec & 0xff) as u32;
    iowrite16(mf6x4_adctrl_chan(chan), (*dev).mmio.add(MF6X4_ADCTRL_REG));
    for i in 0..(*insn).n as usize {
        let _ = ioread16((*dev).mmio.add(MF6X4_ADSTART_REG));
        let ret = comedi_timeout(dev, s, insn, mf6x4_ai_eoc, 0);
        if ret != 0 { return ret; }
        let d = ioread16((*dev).mmio.add(MF6X4_ADDATA_REG)) as u32 & (*s).maxdata;
        *data.add(i) = comedi_offset_munge(s, d);
    }
    iowrite16(0, (*dev).mmio.add(MF6X4_ADCTRL_REG));
    (*insn).n as i32
}

unsafe fn mf6x4_ao_insn_write(dev: *mut comedi_device, s: *mut comedi_subdevice,
                              insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let p = &mut *((*dev).private as *mut mf6x4_private);
    let chan = ((*insn).chanspec & 0xff) as usize;
    let mut val = *(*s).readback.add(chan);
    let gpioc = ioread32(p.gpioc_reg);
    iowrite32((gpioc & !MF6X4_GPIOC_LDAC) | MF6X4_GPIOC_DACEN, p.gpioc_reg);
    for i in 0..(*insn).n as usize { val = *data.add(i); iowrite16(val as u16, (*dev).mmio.add(mf6x4_dac_reg(chan))); }
    *(*s).readback.add(chan) = val;
    (*insn).n as i32
}

unsafe fn mf6x4_auto_attach(dev: *mut comedi_device, context: usize) -> i32 {
    if context >= 2 { return -19; }
    let board = &mf6x4_boards[context];
    (*dev).board_ptr = board as *const _ as *mut _;
    (*dev).board_name = board.name;
    let ret = comedi_pci_enable(dev);
    if ret != 0 { return ret; }
    let devpriv = comedi_alloc_devpriv(dev, core::mem::size_of::<mf6x4_private>()) as *mut mf6x4_private;
    if devpriv.is_null() { return -12; }
    (*devpriv).bar0_mem = pci_ioremap_bar(comedi_to_pci_dev(dev), board.bar_nums[0]);
    if (*devpriv).bar0_mem.is_null() { return -19; }
    (*dev).mmio = pci_ioremap_bar(comedi_to_pci_dev(dev), board.bar_nums[1]);
    if (*dev).mmio.is_null() { return -19; }
    (*devpriv).bar2_mem = pci_ioremap_bar(comedi_to_pci_dev(dev), board.bar_nums[2]);
    if (*devpriv).bar2_mem.is_null() { return -19; }
    (*devpriv).gpioc_reg = if context == 0 { (*devpriv).bar2_mem.add(MF634_GPIOC_REG) } else { (*devpriv).bar0_mem.add(MF624_GPIOC_REG) };
    let ret = comedi_alloc_subdevices(dev, 4);
    if ret != 0 { return ret; }
    let s = (*dev).subdevices;
    (*s.add(0)).type_ = COMEDI_SUBD_AI; (*s.add(0)).subdev_flags = SDF_READABLE | SDF_GROUND; (*s.add(0)).n_chan = 8; (*s.add(0)).maxdata = 0x3fff; (*s.add(0)).range_table = &range_bipolar10; (*s.add(0)).insn_read = Some(mf6x4_ai_insn_read);
    (*s.add(1)).type_ = COMEDI_SUBD_AO; (*s.add(1)).subdev_flags = SDF_WRITABLE; (*s.add(1)).n_chan = 8; (*s.add(1)).maxdata = 0x3fff; (*s.add(1)).range_table = &range_bipolar10; (*s.add(1)).insn_write = Some(mf6x4_ao_insn_write);
    let ret = comedi_alloc_subdev_readback(s.add(1)); if ret != 0 { return ret; }
    (*s.add(2)).type_ = COMEDI_SUBD_DI; (*s.add(2)).subdev_flags = SDF_READABLE; (*s.add(2)).n_chan = 8; (*s.add(2)).maxdata = 1; (*s.add(2)).range_table = &range_digital; (*s.add(2)).insn_bits = Some(mf6x4_di_insn_bits);
    (*s.add(3)).type_ = COMEDI_SUBD_DO; (*s.add(3)).subdev_flags = SDF_WRITABLE; (*s.add(3)).n_chan = 8; (*s.add(3)).maxdata = 1; (*s.add(3)).range_table = &range_digital; (*s.add(3)).insn_bits = Some(mf6x4_do_insn_bits);
    0
}

unsafe fn mf6x4_detach(dev: *mut comedi_device) {
    let p = (*dev).private as *mut mf6x4_private;
    if !p.is_null() { if !(*p).bar0_mem.is_null() { iounmap((*p).bar0_mem); } if !(*p).bar2_mem.is_null() { iounmap((*p).bar2_mem); } }
    comedi_pci_detach(dev);
}

// PCI driver tables and module registration are provided by the surrounding bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
