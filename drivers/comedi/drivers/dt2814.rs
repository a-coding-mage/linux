// SPDX-License-Identifier: GPL-2.0+
/*
 * comedi/drivers/dt2814.c
 * Hardware driver for Data Translation DT2814
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1998 David A. Schleef <ds@schleef.org>
 */
/*
 * Driver: dt2814
 * Description: Data Translation DT2814
 * Author: ds
 * Status: complete
 * Devices: [Data Translation] DT2814 (dt2814)
 *
 * Configuration options:
 * [0] - I/O port base address
 * [1] - IRQ
 *
 * This card has 16 analog inputs multiplexed onto a 12 bit ADC.  There
 * is a minimally useful onboard clock.  The base frequency for the
 * clock is selected by jumpers, and the clock divider can be selected
 * via programmed I/O.  Unfortunately, the clock divider can only be
 * a power of 10, from 1 to 10^7, of which only 3 or 4 are useful.  In
 * addition, the clock does not seem to be very accurate.
 */

// External Linux/Comedi declarations are supplied by the surrounding crate.

const DT2814_CSR: usize = 0;
const DT2814_DATA: usize = 1;

const DT2814_FINISH: u32 = 0x80;
const DT2814_ERR: u32 = 0x40;
const DT2814_BUSY: u32 = 0x20;
const DT2814_ENB: u32 = 0x10;
const DT2814_CHANMASK: u32 = 0x0f;

const DT2814_TIMEOUT: i32 = 10;
const DT2814_MAX_SPEED: u32 = 100000; // Arbitrary 10 khz limit

unsafe fn dt2814_ai_notbusy(
    dev: *mut comedi_device,
    _s: *mut comedi_subdevice,
    _insn: *mut comedi_insn,
    context: usize,
) -> i32 {
    let status: u32 = inb((*dev).iobase + DT2814_CSR);
    if context != 0 {
        *(context as *mut u32) = status;
    }
    if status & DT2814_BUSY != 0 { -EBUSY } else { 0 }
}

unsafe fn dt2814_ai_clear(dev: *mut comedi_device) -> i32 {
    let mut status: u32 = 0;
    // Wait until not busy and get status register value.
    let ret = comedi_timeout(dev, core::ptr::null_mut(), core::ptr::null_mut(),
                             Some(dt2814_ai_notbusy), &mut status as *mut _ as usize);
    if ret != 0 { return ret; }
    if status & (DT2814_FINISH | DT2814_ERR) != 0 {
        // There unread data, or the error flag is set.
        // Read the data register twice to clear the condition.
        inb((*dev).iobase + DT2814_DATA);
        inb((*dev).iobase + DT2814_DATA);
    }
    0
}

unsafe fn dt2814_ai_eoc(dev: *mut comedi_device, _s: *mut comedi_subdevice,
                        _insn: *mut comedi_insn, _context: usize) -> i32 {
    let status = inb((*dev).iobase + DT2814_CSR);
    if status & DT2814_FINISH != 0 { 0 } else { -EBUSY }
}

unsafe fn dt2814_ai_insn_read(dev: *mut comedi_device, _s: *mut comedi_subdevice,
                              insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let mut n: i32 = 0;
    let mut hi: i32;
    let mut lo: i32;
    dt2814_ai_clear(dev); // clear stale data or error
    while n < (*insn).n as i32 {
        let chan = CR_CHAN((*insn).chanspec);
        outb(chan, (*dev).iobase + DT2814_CSR);
        let ret = comedi_timeout(dev, _s, insn, Some(dt2814_ai_eoc), 0);
        if ret != 0 { return ret; }
        hi = inb((*dev).iobase + DT2814_DATA) as i32;
        lo = inb((*dev).iobase + DT2814_DATA) as i32;
        *data.add(n as usize) = ((hi << 4) | (lo >> 4)) as u32;
        n += 1;
    }
    n
}

unsafe fn dt2814_ns_to_timer(ns: *mut u32, _flags: u32) -> i32 {
    // XXX ignores flags
    let mut f: u32 = 10000; // ns
    let mut i: i32 = 0;
    while i < 8 {
        if 2u32.wrapping_mul(*ns) < f.wrapping_mul(11) { break; }
        f = f.wrapping_mul(10);
        i += 1;
    }
    *ns = f;
    i
}

unsafe fn dt2814_ai_cmdtest(_dev: *mut comedi_device, _s: *mut comedi_subdevice,
                            cmd: *mut comedi_cmd) -> i32 {
    let mut err = 0;
    err |= comedi_check_trigger_src(&mut (*cmd).start_src, TRIG_NOW);
    err |= comedi_check_trigger_src(&mut (*cmd).scan_begin_src, TRIG_TIMER);
    err |= comedi_check_trigger_src(&mut (*cmd).convert_src, TRIG_NOW);
    err |= comedi_check_trigger_src(&mut (*cmd).scan_end_src, TRIG_COUNT);
    err |= comedi_check_trigger_src(&mut (*cmd).stop_src, TRIG_COUNT | TRIG_NONE);
    if err != 0 { return 1; }
    err |= comedi_check_trigger_is_unique((*cmd).stop_src);
    if err != 0 { return 2; }
    err |= comedi_check_trigger_arg_is(&mut (*cmd).start_arg, 0);
    err |= comedi_check_trigger_arg_max(&mut (*cmd).scan_begin_arg, 1000000000);
    err |= comedi_check_trigger_arg_min(&mut (*cmd).scan_begin_arg, DT2814_MAX_SPEED);
    err |= comedi_check_trigger_arg_is(&mut (*cmd).scan_end_arg, (*cmd).chanlist_len);
    if (*cmd).stop_src == TRIG_COUNT {
        err |= comedi_check_trigger_arg_min(&mut (*cmd).stop_arg, 2);
    } else {
        err |= comedi_check_trigger_arg_is(&mut (*cmd).stop_arg, 0);
    }
    if err != 0 { return 3; }
    let mut arg = (*cmd).scan_begin_arg;
    dt2814_ns_to_timer(&mut arg, (*cmd).flags);
    err |= comedi_check_trigger_arg_is(&mut (*cmd).scan_begin_arg, arg);
    if err != 0 { return 4; }
    0
}

unsafe fn dt2814_ai_cmd(dev: *mut comedi_device, s: *mut comedi_subdevice) -> i32 {
    let cmd = &mut (*(*s).async_).cmd;
    dt2814_ai_clear(dev); // clear stale data or error
    let trigvar = dt2814_ns_to_timer(&mut cmd.scan_begin_arg, cmd.flags);
    let chan = CR_CHAN(cmd.chanlist[0]);
    outb(chan | DT2814_ENB | ((trigvar as u32) << 5), (*dev).iobase + DT2814_CSR);
    0
}

unsafe fn dt2814_ai_cancel(dev: *mut comedi_device, _s: *mut comedi_subdevice) -> i32 {
    let flags: *mut unsigned_long = core::ptr::null_mut();
    spin_lock_irqsave(&mut (*dev).spinlock, flags);
    let status = inb((*dev).iobase + DT2814_CSR);
    if status & DT2814_ENB != 0 {
        // Clear the timed trigger enable bit.
        // Note: turning off timed mode triggers another sample.
        // This will be mopped up by the calls to dt2814_ai_clear().
        outb(status & DT2814_CHANMASK, (*dev).iobase + DT2814_CSR);
    }
    spin_unlock_irqrestore(&mut (*dev).spinlock, flags);
    0
}

// The interrupt handler and driver registration retain the C ABI-facing layout.
// Their external kernel structure definitions are supplied by the surrounding crate.
unsafe extern "C" fn dt2814_interrupt(_irq: i32, _d: *mut core::ffi::c_void) -> irqreturn_t {
    let dev = _d as *mut comedi_device;
    let s = (*dev).read_subdev;
    let async_ = (*s).async_;
    let mut status;
    spin_lock(&mut (*dev).spinlock);
    status = inb((*dev).iobase + DT2814_CSR);
    if status & DT2814_ENB == 0 || status & (DT2814_FINISH | DT2814_ERR) == 0 {
        spin_unlock(&mut (*dev).spinlock);
        return IRQ_HANDLED;
    }
    let hi = inb((*dev).iobase + DT2814_DATA);
    let lo = inb((*dev).iobase + DT2814_DATA);
    let data = ((hi << 4) | (lo >> 4)) as u16;
    if status & DT2814_ERR != 0 {
        (*async_).events |= COMEDI_CB_ERROR;
    } else {
        comedi_buf_write_samples(s, &data, 1);
        if (*async_).cmd.stop_src == TRIG_COUNT && (*async_).scans_done >= (*async_).cmd.stop_arg {
            (*async_).events |= COMEDI_CB_EOA;
        }
    }
    if (*async_).events & COMEDI_CB_CANCEL_MASK != 0 {
        outb(status & DT2814_CHANMASK, (*dev).iobase + DT2814_CSR);
    }
    spin_unlock(&mut (*dev).spinlock);
    comedi_handle_events(dev, s);
    IRQ_HANDLED
}

unsafe fn dt2814_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> i32 {
    let mut ret = comedi_check_request_region(dev, (*it).options[0], 0x2, 0x200, 0x3ff, 2);
    if ret != 0 { return ret; }
    outb(0, (*dev).iobase + DT2814_CSR);
    if dt2814_ai_clear(dev) != 0 { dev_err((*dev).class_dev, "reset error (fatal)\n"); return -EIO; }
    if (*it).options[1] != 0 {
        ret = request_irq((*it).options[1], Some(dt2814_interrupt), 0, (*dev).board_name, dev);
        if ret == 0 { (*dev).irq = (*it).options[1]; }
    }
    ret = comedi_alloc_subdevices(dev, 1);
    if ret != 0 { return ret; }
    let s = &mut (*dev).subdevices[0];
    s.type_ = COMEDI_SUBD_AI;
    s.subdev_flags = SDF_READABLE | SDF_GROUND;
    s.n_chan = 16;
    s.insn_read = Some(dt2814_ai_insn_read);
    s.maxdata = 0xfff;
    s.range_table = &range_unknown;
    if (*dev).irq != 0 {
        (*dev).read_subdev = s;
        s.subdev_flags |= SDF_CMD_READ;
        s.len_chanlist = 1;
        s.do_cmd = Some(dt2814_ai_cmd);
        s.do_cmdtest = Some(dt2814_ai_cmdtest);
        s.cancel = Some(dt2814_ai_cancel);
    }
    0
}

unsafe fn dt2814_detach(dev: *mut comedi_device) {
    if (*dev).irq != 0 { dt2814_ai_clear(dev); }
    comedi_legacy_detach(dev);
}

static mut dt2814_driver: comedi_driver = comedi_driver {
    driver_name: "dt2814",
    module: THIS_MODULE,
    attach: Some(dt2814_attach),
    detach: Some(dt2814_detach),
};

module_comedi_driver!(dt2814_driver);
module_author!("Comedi https://www.comedi.org");
module_description!("Comedi low-level driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
