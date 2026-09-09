// SPDX-License-Identifier: GPL-2.0+
/*
 * pcl726.c
 * Comedi driver for 6/12-Channel D/A Output and DIO cards
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1998 David A. Schleef <ds@schleef.org>
 */

/*
 * Driver: pcl726
 * Description: Advantech PCL-726 & compatibles
 * Author: David A. Schleef <ds@schleef.org>
 * Status: untested
 * Devices: [Advantech] PCL-726 (pcl726), PCL-727 (pcl727), PCL-728 (pcl728),
 *   [ADLink] ACL-6126 (acl6126), ACL-6128 (acl6128)
 */

// C headers and build-time kernel dependencies are supplied externally.

const fn pcl726_ao_msb_reg(x: u32) -> u32 { 0x00 + x * 2 }
const fn pcl726_ao_lsb_reg(x: u32) -> u32 { 0x01 + x * 2 }
const PCL726_DO_MSB_REG: u32 = 0x0c;
const PCL726_DO_LSB_REG: u32 = 0x0d;
const PCL726_DI_MSB_REG: u32 = 0x0e;
const PCL726_DI_LSB_REG: u32 = 0x0f;

const PCL727_DI_MSB_REG: u32 = 0x00;
const PCL727_DI_LSB_REG: u32 = 0x01;
const PCL727_DO_MSB_REG: u32 = 0x18;
const PCL727_DO_LSB_REG: u32 = 0x19;

static rangelist_726: [*const comedi_lrange; 6] = [
    &range_unipolar5, &range_unipolar10, &range_bipolar5,
    &range_bipolar10, &range_4_20mA, &range_unknown,
];

static rangelist_727: [*const comedi_lrange; 4] = [
    &range_unipolar5, &range_unipolar10, &range_bipolar5, &range_4_20mA,
];

static rangelist_728: [*const comedi_lrange; 6] = [
    &range_unipolar5, &range_unipolar10, &range_bipolar5,
    &range_bipolar10, &range_4_20mA, &range_0_20mA,
];

#[repr(C)]
struct pcl726_board {
    name: *const i8,
    io_len: u32,
    min_io_start: u32,
    irq_mask: u32,
    ao_ranges: *const *const comedi_lrange,
    ao_num_ranges: i32,
    ao_nchan: i32,
    have_dio: u32,
    is_pcl727: u32,
}

static pcl726_boards: [pcl726_board; 5] = [
    pcl726_board { name: c"pcl726".as_ptr(), io_len: 0x10, min_io_start: 0x200, irq_mask: 0, ao_ranges: rangelist_726.as_ptr(), ao_num_ranges: 6, ao_nchan: 6, have_dio: 1, is_pcl727: 0 },
    pcl726_board { name: c"pcl727".as_ptr(), io_len: 0x20, min_io_start: 0x200, irq_mask: 0, ao_ranges: rangelist_727.as_ptr(), ao_num_ranges: 4, ao_nchan: 12, have_dio: 1, is_pcl727: 1 },
    pcl726_board { name: c"pcl728".as_ptr(), io_len: 0x08, min_io_start: 0, irq_mask: 0, ao_ranges: rangelist_728.as_ptr(), ao_num_ranges: 6, ao_nchan: 2, have_dio: 0, is_pcl727: 0 },
    pcl726_board { name: c"acl6126".as_ptr(), io_len: 0x10, min_io_start: 0x200, irq_mask: 0x96e8, ao_ranges: rangelist_726.as_ptr(), ao_num_ranges: 6, ao_nchan: 6, have_dio: 1, is_pcl727: 0 },
    pcl726_board { name: c"acl6128".as_ptr(), io_len: 0x08, min_io_start: 0, irq_mask: 0, ao_ranges: rangelist_728.as_ptr(), ao_num_ranges: 6, ao_nchan: 2, have_dio: 0, is_pcl727: 0 },
];

#[repr(C)]
struct pcl726_private {
    rangelist: [*const comedi_lrange; 12],
    cmd_running: u32,
}

unsafe fn pcl726_intr_insn_bits(_dev: *mut comedi_device, _s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    (*data.add(1)) = 0;
    (*insn).n as i32
}

unsafe fn pcl726_intr_cmdtest(_dev: *mut comedi_device, _s: *mut comedi_subdevice, cmd: *mut comedi_cmd) -> i32 {
    let mut err = 0;
    err |= comedi_check_trigger_src(&mut (*cmd).start_src, TRIG_NOW);
    err |= comedi_check_trigger_src(&mut (*cmd).scan_begin_src, TRIG_EXT);
    err |= comedi_check_trigger_src(&mut (*cmd).convert_src, TRIG_FOLLOW);
    err |= comedi_check_trigger_src(&mut (*cmd).scan_end_src, TRIG_COUNT);
    err |= comedi_check_trigger_src(&mut (*cmd).stop_src, TRIG_NONE);
    if err != 0 { return 1; }
    err |= comedi_check_trigger_arg_is(&mut (*cmd).start_arg, 0);
    err |= comedi_check_trigger_arg_is(&mut (*cmd).scan_begin_arg, 0);
    err |= comedi_check_trigger_arg_is(&mut (*cmd).convert_arg, 0);
    err |= comedi_check_trigger_arg_is(&mut (*cmd).scan_end_arg, (*cmd).chanlist_len);
    err |= comedi_check_trigger_arg_is(&mut (*cmd).stop_arg, 0);
    if err != 0 { return 3; }
    0
}

unsafe fn pcl726_intr_cmd(dev: *mut comedi_device, _s: *mut comedi_subdevice) -> i32 {
    (*( (*dev).private as *mut pcl726_private)).cmd_running = 1;
    0
}

unsafe fn pcl726_intr_cancel(dev: *mut comedi_device, _s: *mut comedi_subdevice) -> i32 {
    (*( (*dev).private as *mut pcl726_private)).cmd_running = 0;
    0
}

unsafe extern "C" fn pcl726_interrupt(_irq: i32, d: *mut core::ffi::c_void) -> irqreturn_t {
    let dev = d as *mut comedi_device;
    let s = (*dev).read_subdev;
    let devpriv = (*dev).private as *mut pcl726_private;
    if (*devpriv).cmd_running != 0 {
        let mut val: u16 = 0;
        pcl726_intr_cancel(dev, s);
        comedi_buf_write_samples(s, &mut val as *mut u16, 1);
        comedi_handle_events(dev, s);
    }
    IRQ_HANDLED
}

unsafe fn pcl726_ao_insn_write(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let chan = CR_CHAN((*insn).chanspec);
    let range = CR_RANGE((*insn).chanspec);
    for i in 0..(*insn).n as usize {
        let mut val = *data.add(i);
        (*s).readback.add(chan as usize).write(val);
        if comedi_chan_range_is_bipolar(s, chan, range) != 0 { val = comedi_offset_munge(s, val); }
        outb((val >> 8) & 0xff, (*dev).iobase + pcl726_ao_msb_reg(chan));
        outb(val & 0xff, (*dev).iobase + pcl726_ao_lsb_reg(chan));
    }
    (*insn).n as i32
}

unsafe fn pcl726_di_insn_bits(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let board = (*dev).board_ptr as *const pcl726_board;
    let val = if (*board).is_pcl727 != 0 {
        inb((*dev).iobase + PCL727_DI_LSB_REG) | (inb((*dev).iobase + PCL727_DI_MSB_REG) << 8)
    } else {
        inb((*dev).iobase + PCL726_DI_LSB_REG) | (inb((*dev).iobase + PCL726_DI_MSB_REG) << 8)
    };
    *data.add(1) = val;
    (*insn).n as i32
}

unsafe fn pcl726_do_insn_bits(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let board = (*dev).board_ptr as *const pcl726_board;
    let mask = comedi_dio_update_state(s, data);
    if mask != 0 {
        let (lsb, msb) = if (*board).is_pcl727 != 0 { (PCL727_DO_LSB_REG, PCL727_DO_MSB_REG) } else { (PCL726_DO_LSB_REG, PCL726_DO_MSB_REG) };
        if mask & 0x00ff != 0 { outb((*s).state & 0xff, (*dev).iobase + lsb); }
        if mask & 0xff00 != 0 { outb((*s).state >> 8, (*dev).iobase + msb); }
    }
    *data.add(1) = (*s).state;
    (*insn).n as i32
}

unsafe fn pcl726_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> i32 {
    let board = (*dev).board_ptr as *const pcl726_board;
    let devpriv: *mut pcl726_private;
    let mut s: *mut comedi_subdevice;
    let mut subdev: i32;
    let mut ret: i32;

    ret = comedi_check_request_region(dev, (*it).options[0], (*board).io_len,
                                      (*board).min_io_start, 0x3ff, (*board).io_len);
    if ret != 0 { return ret; }
    devpriv = comedi_alloc_devpriv(dev, core::mem::size_of::<pcl726_private>()) as *mut pcl726_private;
    if devpriv.is_null() { return -12; }

    if (*it).options[1] > 0 && (*it).options[1] < 16 &&
       ((*board).irq_mask & (1u32 << (*it).options[1])) != 0 {
        ret = request_irq((*it).options[1] as i32, pcl726_interrupt, 0, (*dev).board_name, dev as *mut core::ffi::c_void);
        if ret == 0 { (*dev).irq = (*it).options[1] as i32; }
    }

    for i in 0..12usize {
        let opt = (*it).options[2 + i];
        (*devpriv).rangelist[i] = if opt < (*board).ao_num_ranges as u32 && i < (*board).ao_nchan as usize {
            *(*board).ao_ranges.add(opt as usize)
        } else { &range_unknown };
    }

    subdev = if (*board).have_dio != 0 { 3 } else { 1 };
    if (*dev).irq != 0 { subdev += 1; }
    ret = comedi_alloc_subdevices(dev, subdev);
    if ret != 0 { return ret; }
    subdev = 0;

    s = (*dev).subdevices.add(subdev as usize); subdev += 1;
    (*s).type_ = COMEDI_SUBD_AO; (*s).subdev_flags = SDF_WRITABLE | SDF_GROUND;
    (*s).n_chan = (*board).ao_nchan as u32; (*s).maxdata = 0x0fff;
    (*s).range_table_list = (*devpriv).rangelist.as_mut_ptr(); (*s).insn_write = Some(pcl726_ao_insn_write);
    ret = comedi_alloc_subdev_readback(s); if ret != 0 { return ret; }

    if (*board).have_dio != 0 {
        s = (*dev).subdevices.add(subdev as usize); subdev += 1;
        (*s).type_ = COMEDI_SUBD_DI; (*s).subdev_flags = SDF_READABLE; (*s).n_chan = 16; (*s).maxdata = 1;
        (*s).insn_bits = Some(pcl726_di_insn_bits); (*s).range_table = &range_digital;
        s = (*dev).subdevices.add(subdev as usize); subdev += 1;
        (*s).type_ = COMEDI_SUBD_DO; (*s).subdev_flags = SDF_WRITABLE; (*s).n_chan = 16; (*s).maxdata = 1;
        (*s).insn_bits = Some(pcl726_do_insn_bits); (*s).range_table = &range_digital;
    }
    if (*dev).irq != 0 {
        s = (*dev).subdevices.add(subdev as usize); (*dev).read_subdev = s;
        (*s).type_ = COMEDI_SUBD_DI; (*s).subdev_flags = SDF_READABLE | SDF_CMD_READ;
        (*s).n_chan = 1; (*s).maxdata = 1; (*s).range_table = &range_digital;
        (*s).insn_bits = Some(pcl726_intr_insn_bits); (*s).len_chanlist = 1;
        (*s).do_cmdtest = Some(pcl726_intr_cmdtest); (*s).do_cmd = Some(pcl726_intr_cmd); (*s).cancel = Some(pcl726_intr_cancel);
    }
    0
}

// Original module metadata: author Comedi https://www.comedi.org; description
// "Comedi driver for Advantech PCL-726 & compatibles"; license "GPL".

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
