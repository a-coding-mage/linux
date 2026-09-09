// SPDX-License-Identifier: GPL-2.0+
/*
 * aio_iiro_16.c
 * Comedi driver for ACCES I/O Products 104-IIRO-16 board
 * Copyright (C) 2006 C&C Technologies, Inc.
 */

/*
 * Driver: aio_iiro_16
 * Description: ACCES I/O Products PC/104 Isolated Input/Relay Output Board
 * Author: Zachary Ware <zach.ware@cctechnol.com>
 * Devices: [ACCES I/O] 104-IIRO-16 (aio_iiro_16)
 * Status: experimental
 *
 * Configuration Options:
 *   [0] - I/O port base address
 *   [1] - IRQ (optional)
 *
 * The board supports interrupts on change of state of the digital inputs.
 * The sample data returned by the async command indicates which inputs
 * changed state and the current state of the inputs.
 */

// Linux/Comedi declarations and symbols are supplied by the surrounding crate.

pub const AIO_IIRO_16_RELAY_0_7: u32 = 0x00;
pub const AIO_IIRO_16_INPUT_0_7: u32 = 0x01;
pub const AIO_IIRO_16_IRQ: u32 = 0x02;
pub const AIO_IIRO_16_RELAY_8_15: u32 = 0x04;
pub const AIO_IIRO_16_INPUT_8_15: u32 = 0x05;
pub const AIO_IIRO_16_STATUS: u32 = 0x07;
pub const AIO_IIRO_16_STATUS_IRQE: u32 = 1 << 7;
pub const AIO_IIRO_16_STATUS_INPUT_8_15: u32 = 1 << 1;
pub const AIO_IIRO_16_STATUS_INPUT_0_7: u32 = 1 << 0;

extern "C" {
    fn inb(port: u32) -> u8;
    fn outb(value: u8, port: u32);
    fn comedi_buf_write_samples(s: *mut comedi_subdevice, data: *const u32, count: u32);
    fn comedi_handle_events(dev: *mut comedi_device, s: *mut comedi_subdevice);
    fn comedi_check_trigger_src(src: *mut u32, flags: u32) -> i32;
    fn comedi_check_trigger_arg_is(arg: *mut u32, value: u32) -> i32;
    fn comedi_dio_update_state(s: *mut comedi_subdevice, data: *mut u32) -> bool;
    fn comedi_check_request_region(dev: *mut comedi_device, start: u32, len: u32,
                                   from: u32, to: u32, align: u32) -> i32;
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t,
                   flags: u32, name: *const core::ffi::c_char, dev: *mut core::ffi::c_void) -> i32;
    fn comedi_alloc_subdevices(dev: *mut comedi_device, n: u32) -> i32;
    fn comedi_legacy_detach(dev: *mut comedi_device) -> i32;
}

// External types, constants, and globals are defined by Comedi bindings.
#[repr(C)] pub struct comedi_device { pub iobase: u32, pub attached: bool, pub read_subdev: *mut comedi_subdevice, pub irq: u32, pub board_name: *const core::ffi::c_char, pub subdevices: *mut comedi_subdevice }
#[repr(C)] pub struct comedi_subdevice { pub state: u32, pub type_: u32, pub subdev_flags: u32, pub n_chan: u32, pub maxdata: u32, pub range_table: *const core::ffi::c_void, pub insn_bits: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_subdevice, *mut comedi_insn, *mut u32) -> i32>, pub len_chanlist: u32, pub do_cmdtest: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_subdevice, *mut comedi_cmd) -> i32>, pub do_cmd: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_subdevice) -> i32>, pub cancel: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_subdevice) -> i32> }
#[repr(C)] pub struct comedi_insn { pub n: u32 }
#[repr(C)] pub struct comedi_cmd { pub start_src: u32, pub scan_begin_src: u32, pub convert_src: u32, pub scan_end_src: u32, pub stop_src: u32, pub start_arg: u32, pub scan_begin_arg: u32, pub convert_arg: u32, pub scan_end_arg: u32, pub stop_arg: u32, pub chanlist_len: u32 }
#[repr(C)] pub struct comedi_devconfig { pub options: [u32; 4] }
pub type irqreturn_t = i32;
pub const IRQ_NONE: irqreturn_t = 0;
pub const IRQ_HANDLED: irqreturn_t = 1;
pub const TRIG_NOW: u32 = 1; pub const TRIG_EXT: u32 = 2; pub const TRIG_FOLLOW: u32 = 4; pub const TRIG_COUNT: u32 = 8; pub const TRIG_NONE: u32 = 16;
pub const COMEDI_SUBD_DO: u32 = 1; pub const COMEDI_SUBD_DI: u32 = 2; pub const SDF_WRITABLE: u32 = 1; pub const SDF_READABLE: u32 = 2; pub const SDF_CMD_READ: u32 = 4; pub const SDF_LSAMPL: u32 = 8;
extern "C" { static range_digital: core::ffi::c_void; }

unsafe fn aio_iiro_16_read_inputs(dev: *mut comedi_device) -> u32 {
    inb((*dev).iobase + AIO_IIRO_16_INPUT_0_7) as u32 |
        ((inb((*dev).iobase + AIO_IIRO_16_INPUT_8_15) as u32) << 8)
}

pub unsafe extern "C" fn aio_iiro_16_cos(irq: i32, d: *mut core::ffi::c_void) -> irqreturn_t {
    let dev = d as *mut comedi_device;
    let s = (*dev).read_subdev;
    if !(*dev).attached { return IRQ_NONE; }
    let status = inb((*dev).iobase + AIO_IIRO_16_STATUS) as u32;
    if status & AIO_IIRO_16_STATUS_IRQE == 0 { return IRQ_NONE; }
    let val = aio_iiro_16_read_inputs(dev) | (status << 16);
    comedi_buf_write_samples(s, &val, 1);
    comedi_handle_events(dev, s);
    IRQ_HANDLED
}

unsafe fn aio_iiro_enable_irq(dev: *mut comedi_device, enable: bool) {
    if enable { inb((*dev).iobase + AIO_IIRO_16_IRQ); }
    else { outb(0, (*dev).iobase + AIO_IIRO_16_IRQ); }
}

unsafe extern "C" fn aio_iiro_16_cos_cancel(dev: *mut comedi_device, _s: *mut comedi_subdevice) -> i32 { aio_iiro_enable_irq(dev, false); 0 }
unsafe extern "C" fn aio_iiro_16_cos_cmd(dev: *mut comedi_device, _s: *mut comedi_subdevice) -> i32 { aio_iiro_enable_irq(dev, true); 0 }

unsafe extern "C" fn aio_iiro_16_cos_cmdtest(_dev: *mut comedi_device, _s: *mut comedi_subdevice, cmd: *mut comedi_cmd) -> i32 {
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

unsafe extern "C" fn aio_iiro_16_do_insn_bits(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    if comedi_dio_update_state(s, data) { outb((*s).state as u8, (*dev).iobase + AIO_IIRO_16_RELAY_0_7); outb(((*s).state >> 8) as u8, (*dev).iobase + AIO_IIRO_16_RELAY_8_15); }
    *data.add(1) = (*s).state; (*insn).n as i32
}
unsafe extern "C" fn aio_iiro_16_di_insn_bits(dev: *mut comedi_device, _s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 { *data.add(1) = aio_iiro_16_read_inputs(dev); (*insn).n as i32 }

unsafe extern "C" fn aio_iiro_16_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> i32 {
    let mut ret = comedi_check_request_region(dev, (*it).options[0], 0x8, 0x100, 0x3ff, 0x8);
    if ret != 0 { return ret; }
    aio_iiro_enable_irq(dev, false);
    let irq = (*it).options[1];
    if irq > 0 && irq < 16 && ((1u32 << irq) & 0xdcfc) != 0 {
        ret = request_irq(irq as i32, aio_iiro_16_cos, 0, (*dev).board_name, dev as *mut core::ffi::c_void);
        if ret == 0 { (*dev).irq = irq; }
    }
    ret = comedi_alloc_subdevices(dev, 2);
    if ret != 0 { return ret; }
    let s0 = (*dev).subdevices;
    (*s0).type_ = COMEDI_SUBD_DO; (*s0).subdev_flags = SDF_WRITABLE; (*s0).n_chan = 16; (*s0).maxdata = 1;
    (*s0).range_table = &range_digital; (*s0).insn_bits = Some(aio_iiro_16_do_insn_bits);
    (*s0).state = inb((*dev).iobase + AIO_IIRO_16_RELAY_0_7) as u32 |
        ((inb((*dev).iobase + AIO_IIRO_16_RELAY_8_15) as u32) << 8);
    let s = s0.add(1);
    (*s).type_ = COMEDI_SUBD_DI; (*s).subdev_flags = SDF_READABLE; (*s).n_chan = 16; (*s).maxdata = 1;
    (*s).range_table = &range_digital; (*s).insn_bits = Some(aio_iiro_16_di_insn_bits);
    if (*dev).irq != 0 {
        (*dev).read_subdev = s; (*s).subdev_flags |= SDF_CMD_READ | SDF_LSAMPL; (*s).len_chanlist = 1;
        (*s).do_cmdtest = Some(aio_iiro_16_cos_cmdtest); (*s).do_cmd = Some(aio_iiro_16_cos_cmd); (*s).cancel = Some(aio_iiro_16_cos_cancel);
    }
    0
}

#[repr(C)] pub struct comedi_driver { pub driver_name: *const core::ffi::c_char, pub module: *mut core::ffi::c_void, pub attach: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_devconfig) -> i32>, pub detach: Option<unsafe extern "C" fn(*mut comedi_device) -> i32> }
#[no_mangle] pub static mut aio_iiro_16_driver: comedi_driver = comedi_driver {
    driver_name: b"aio_iiro_16\0".as_ptr() as *const _, module: core::ptr::null_mut(), attach: Some(aio_iiro_16_attach), detach: Some(comedi_legacy_detach),
};

// module_comedi_driver(aio_iiro_16_driver);
// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_DESCRIPTION("Comedi driver for ACCES I/O Products 104-IIRO-16 board");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
