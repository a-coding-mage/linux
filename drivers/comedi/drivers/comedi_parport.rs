// SPDX-License-Identifier: GPL-2.0+
/*
 * comedi_parport.c
 * Comedi driver for standard parallel port
 *
 * For more information see:
 *	http://retired.beyondlogic.org/spp/parallel.htm
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1998,2001 David A. Schleef <ds@schleef.org>
 */

/*
 * Driver: comedi_parport
 * Description: Standard PC parallel port
 * Author: ds
 * Status: works in immediate mode
 * Devices: [standard] parallel port (comedi_parport)
 * Updated: Tue, 30 Apr 2002 21:11:45 -0700
 *
 * A cheap and easy way to get a few more digital I/O lines. Steal
 * additional parallel ports from old computers or your neighbors'
 * computers.
 */

// External Linux/Comedi declarations are supplied by the surrounding translation unit.

const PARPORT_DATA_REG: u32 = 0x00;
const PARPORT_STATUS_REG: u32 = 0x01;
const PARPORT_CTRL_REG: u32 = 0x02;
const PARPORT_CTRL_IRQ_ENA: u32 = 1 << 4;
const PARPORT_CTRL_BIDIR_ENA: u32 = 1 << 5;

unsafe fn parport_data_reg_insn_bits(
    dev: *mut comedi_device,
    s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut u32,
) -> i32 {
    if comedi_dio_update_state(s, data) != 0 {
        outb((*s).state, (*dev).iobase + PARPORT_DATA_REG);
    }
    *data.add(1) = inb((*dev).iobase + PARPORT_DATA_REG);
    (*insn).n as i32
}

unsafe fn parport_data_reg_insn_config(
    dev: *mut comedi_device,
    s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut u32,
) -> i32 {
    let mut ctrl: u32;
    let ret = comedi_dio_insn_config(dev, s, insn, data, 0xff);
    if ret != 0 { return ret; }
    ctrl = inb((*dev).iobase + PARPORT_CTRL_REG);
    if (*s).io_bits != 0 { ctrl &= !PARPORT_CTRL_BIDIR_ENA; }
    else { ctrl |= PARPORT_CTRL_BIDIR_ENA; }
    outb(ctrl, (*dev).iobase + PARPORT_CTRL_REG);
    (*insn).n as i32
}

unsafe fn parport_status_reg_insn_bits(
    dev: *mut comedi_device, _s: *mut comedi_subdevice,
    insn: *mut comedi_insn, data: *mut u32,
) -> i32 {
    *data.add(1) = inb((*dev).iobase + PARPORT_STATUS_REG) >> 3;
    (*insn).n as i32
}

unsafe fn parport_ctrl_reg_insn_bits(
    dev: *mut comedi_device, s: *mut comedi_subdevice,
    insn: *mut comedi_insn, data: *mut u32,
) -> i32 {
    let mut ctrl: u32;
    if comedi_dio_update_state(s, data) != 0 {
        ctrl = inb((*dev).iobase + PARPORT_CTRL_REG);
        ctrl &= PARPORT_CTRL_IRQ_ENA | PARPORT_CTRL_BIDIR_ENA;
        ctrl |= (*s).state;
        outb(ctrl, (*dev).iobase + PARPORT_CTRL_REG);
    }
    *data.add(1) = (*s).state;
    (*insn).n as i32
}

unsafe fn parport_intr_insn_bits(
    _dev: *mut comedi_device, _s: *mut comedi_subdevice,
    insn: *mut comedi_insn, data: *mut u32,
) -> i32 {
    *data.add(1) = 0;
    (*insn).n as i32
}

unsafe fn parport_intr_cmdtest(
    _dev: *mut comedi_device, _s: *mut comedi_subdevice, cmd: *mut comedi_cmd,
) -> i32 {
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

unsafe fn parport_intr_cmd(dev: *mut comedi_device, _s: *mut comedi_subdevice) -> i32 {
    let mut ctrl = inb((*dev).iobase + PARPORT_CTRL_REG);
    ctrl |= PARPORT_CTRL_IRQ_ENA;
    outb(ctrl, (*dev).iobase + PARPORT_CTRL_REG);
    0
}

unsafe fn parport_intr_cancel(dev: *mut comedi_device, _s: *mut comedi_subdevice) -> i32 {
    let mut ctrl = inb((*dev).iobase + PARPORT_CTRL_REG);
    ctrl &= !PARPORT_CTRL_IRQ_ENA;
    outb(ctrl, (*dev).iobase + PARPORT_CTRL_REG);
    0
}

unsafe extern "C" fn parport_interrupt(_irq: i32, d: *mut core::ffi::c_void) -> irqreturn_t {
    let dev = d as *mut comedi_device;
    let s = (*dev).read_subdev;
    let mut val: u16 = 0;
    if (*dev).attached == 0 { return IRQ_NONE; }
    let ctrl = inb((*dev).iobase + PARPORT_CTRL_REG);
    if ctrl & PARPORT_CTRL_IRQ_ENA == 0 { return IRQ_NONE; }
    comedi_buf_write_samples(s, &mut val as *mut u16, 1);
    comedi_handle_events(dev, s);
    IRQ_HANDLED
}

unsafe fn parport_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> i32 {
    let mut s: *mut comedi_subdevice;
    let iobase = (*it).options[0];
    let mut ret = comedi_check_request_region(dev, iobase, 0x03, 0, u32::MAX, 4);
    if ret != 0 { return ret; }
    outb(0, (*dev).iobase + PARPORT_DATA_REG);
    outb(0, (*dev).iobase + PARPORT_CTRL_REG);
    if (*it).options[1] != 0 {
        ret = request_irq((*it).options[1], Some(parport_interrupt), 0, (*dev).board_name, dev);
        if ret == 0 { (*dev).irq = (*it).options[1]; }
    }
    ret = comedi_alloc_subdevices(dev, if (*dev).irq != 0 { 4 } else { 3 });
    if ret != 0 { return ret; }
    s = (*dev).subdevices.add(0);
    (*s).type_ = COMEDI_SUBD_DIO; (*s).subdev_flags = SDF_READABLE | SDF_WRITABLE;
    (*s).n_chan = 8; (*s).maxdata = 1; (*s).range_table = &range_digital;
    (*s).insn_bits = Some(parport_data_reg_insn_bits); (*s).insn_config = Some(parport_data_reg_insn_config);
    s = (*dev).subdevices.add(1);
    (*s).type_ = COMEDI_SUBD_DI; (*s).subdev_flags = SDF_READABLE; (*s).n_chan = 5;
    (*s).maxdata = 1; (*s).range_table = &range_digital; (*s).insn_bits = Some(parport_status_reg_insn_bits);
    s = (*dev).subdevices.add(2);
    (*s).type_ = COMEDI_SUBD_DO; (*s).subdev_flags = SDF_WRITABLE; (*s).n_chan = 4;
    (*s).maxdata = 1; (*s).range_table = &range_digital; (*s).insn_bits = Some(parport_ctrl_reg_insn_bits);
    if (*dev).irq != 0 {
        s = (*dev).subdevices.add(3); (*dev).read_subdev = s;
        (*s).type_ = COMEDI_SUBD_DI; (*s).subdev_flags = SDF_READABLE | SDF_CMD_READ;
        (*s).n_chan = 1; (*s).maxdata = 1; (*s).range_table = &range_digital;
        (*s).insn_bits = Some(parport_intr_insn_bits); (*s).len_chanlist = 1;
        (*s).do_cmdtest = Some(parport_intr_cmdtest); (*s).do_cmd = Some(parport_intr_cmd);
        (*s).cancel = Some(parport_intr_cancel);
    }
    0
}

static mut parport_driver: comedi_driver = comedi_driver {
    driver_name: "comedi_parport" as *const str,
    module: THIS_MODULE,
    attach: Some(parport_attach),
    detach: Some(comedi_legacy_detach),
};

// module_comedi_driver(parport_driver);
// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_DESCRIPTION("Comedi: Standard parallel port driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
