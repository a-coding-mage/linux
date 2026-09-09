// SPDX-License-Identifier: GPL-2.0+
/*
 * Comedi driver for NI 670x devices
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1997-2001 David A. Schleef <ds@schleef.org>
 */

/*
 * Driver: ni_670x
 * Description: National Instruments 670x
 * Author: Bart Joris <bjoris@advalvas.be>
 * Updated: Wed, 11 Dec 2002 18:25:35 -0800
 * Devices: [National Instruments] PCI-6703 (ni_670x), PCI-6704
 * Status: unknown
 *
 * Commands are not supported.
 *
 * Manuals:
 *   322110a.pdf\tPCI/PXI-6704 User Manual
 *   322110b.pdf\tPCI/PXI-6703/6704 User Manual
 */

// Linux and Comedi dependencies supplied externally.

const AO_VALUE_OFFSET: usize = 0x00;
const AO_CHAN_OFFSET: usize = 0x0c;
const AO_STATUS_OFFSET: usize = 0x10;
const AO_CONTROL_OFFSET: usize = 0x10;
const DIO_PORT0_DIR_OFFSET: usize = 0x20;
const DIO_PORT0_DATA_OFFSET: usize = 0x24;
const DIO_PORT1_DIR_OFFSET: usize = 0x28;
const DIO_PORT1_DATA_OFFSET: usize = 0x2c;
const MISC_STATUS_OFFSET: usize = 0x14;
const MISC_CONTROL_OFFSET: usize = 0x14;

#[repr(C)]
enum ni_670x_boardid {
    BOARD_PCI6703,
    BOARD_PXI6704,
    BOARD_PCI6704,
}

#[repr(C)]
struct ni_670x_board {
    name: *const core::ffi::c_char,
    ao_chans: u16,
}

static NI_670X_BOARDS: [ni_670x_board; 3] = [
    ni_670x_board { name: b"PCI-6703\0".as_ptr() as *const _, ao_chans: 16 },
    ni_670x_board { name: b"PXI-6704\0".as_ptr() as *const _, ao_chans: 32 },
    ni_670x_board { name: b"PCI-6704\0".as_ptr() as *const _, ao_chans: 32 },
];

#[repr(C)]
struct ni_670x_private {
    boardtype: i32,
    dio: i32,
}

unsafe fn ni_670x_ao_insn_write(
    dev: *mut comedi_device,
    s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut u32,
) -> i32 {
    let chan = CR_CHAN((*insn).chanspec);
    let mut val = (*s).readback[chan as usize];
    let mut i = 0;
    while i < (*insn).n {
        val = *data.add(i as usize);
        writel(((chan & 15) << 1) | ((chan & 16) >> 4), (*dev).mmio.add(AO_CHAN_OFFSET));
        writel(val, (*dev).mmio.add(AO_VALUE_OFFSET));
        i += 1;
    }
    (*s).readback[chan as usize] = val;
    (*insn).n as i32
}

unsafe fn ni_670x_dio_insn_bits(
    dev: *mut comedi_device,
    s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut u32,
) -> i32 {
    if comedi_dio_update_state(s, data) != 0 {
        writel((*s).state, (*dev).mmio.add(DIO_PORT0_DATA_OFFSET));
    }
    *data.add(1) = readl((*dev).mmio.add(DIO_PORT0_DATA_OFFSET));
    (*insn).n as i32
}

unsafe fn ni_670x_dio_insn_config(
    dev: *mut comedi_device,
    s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut u32,
) -> i32 {
    let ret = comedi_dio_insn_config(dev, s, insn, data, 0);
    if ret != 0 { return ret; }
    writel((*s).io_bits, (*dev).mmio.add(DIO_PORT0_DIR_OFFSET));
    (*insn).n as i32
}

/* ripped from mite.h and mite_setup2() to avoid mite dependency */
const MITE_IODWBSR: usize = 0xc0; /* IO Device Window Base Size Register */
const WENAB: u32 = 1 << 7; /* window enable */

unsafe fn ni_670x_mite_init(pcidev: *mut pci_dev) -> i32 {
    let mite_base = pci_ioremap_bar(pcidev, 0);
    if mite_base.is_null() { return -ENOMEM; }
    let main_phys_addr: u32 = pci_resource_start(pcidev, 1);
    writel(main_phys_addr | WENAB, mite_base.add(MITE_IODWBSR));
    iounmap(mite_base);
    0
}

unsafe fn ni_670x_auto_attach(dev: *mut comedi_device, context: usize) -> i32 {
    let pcidev = comedi_to_pci_dev(dev);
    if context >= NI_670X_BOARDS.len() { return -ENODEV; }
    let board = &NI_670X_BOARDS[context];
    (*dev).board_ptr = board as *const _ as *mut _;
    (*dev).board_name = board.name;
    let mut ret = comedi_pci_enable(dev);
    if ret != 0 { return ret; }
    let devpriv = comedi_alloc_devpriv(dev, core::mem::size_of::<ni_670x_private>());
    if devpriv.is_null() { return -ENOMEM; }
    ret = ni_670x_mite_init(pcidev);
    if ret != 0 { return ret; }
    (*dev).mmio = pci_ioremap_bar(pcidev, 1);
    if (*dev).mmio.is_null() { return -ENOMEM; }
    ret = comedi_alloc_subdevices(dev, 2);
    if ret != 0 { return ret; }
    let s = &mut (*dev).subdevices[0];
    s.type_ = COMEDI_SUBD_AO; s.subdev_flags = SDF_WRITABLE; s.n_chan = board.ao_chans as u32; s.maxdata = 0xffff;
    if s.n_chan == 32 {
        let range_table_list = kmalloc_objs::<*const comedi_lrange>(32);
        if range_table_list.is_null() { return -ENOMEM; }
        s.range_table_list = range_table_list;
        for i in 0..16 { *range_table_list.add(i) = &range_bipolar10; *range_table_list.add(16 + i) = &range_0_20mA; }
    } else { s.range_table = &range_bipolar10; }
    s.insn_write = Some(ni_670x_ao_insn_write);
    ret = comedi_alloc_subdev_readback(s);
    if ret != 0 { return ret; }
    let s = &mut (*dev).subdevices[1];
    s.type_ = COMEDI_SUBD_DIO; s.subdev_flags = SDF_READABLE | SDF_WRITABLE; s.n_chan = 8; s.maxdata = 1;
    s.range_table = &range_digital; s.insn_bits = Some(ni_670x_dio_insn_bits); s.insn_config = Some(ni_670x_dio_insn_config);
    writel(0x10, (*dev).mmio.add(MISC_CONTROL_OFFSET));
    writel(0x00, (*dev).mmio.add(AO_CONTROL_OFFSET));
    0
}

unsafe fn ni_670x_detach(dev: *mut comedi_device) {
    let mut s: *mut comedi_subdevice;
    comedi_pci_detach(dev);
    if (*dev).n_subdevices != 0 { s = (*dev).subdevices.as_mut_ptr(); if !s.is_null() { kfree((*s).range_table_list); } }
}

// Driver, PCI table, module registration, and metadata are supplied through the
// corresponding external Comedi/Linux declarations.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
