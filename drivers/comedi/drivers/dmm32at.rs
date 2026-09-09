// SPDX-License-Identifier: GPL-2.0+
/* dmm32at.c - Diamond Systems Diamond-MM-32-AT Comedi driver */

// External kernel/comedi declarations are supplied by the surrounding crate.

const DMM32AT_AI_START_CONV_REG: usize = 0x00;
const DMM32AT_AI_LSB_REG: usize = 0x00;
const DMM32AT_AUX_DOUT_REG: usize = 0x01;
const DMM32AT_AUX_DOUT2: u8 = 1 << 2;
const DMM32AT_AUX_DOUT1: u8 = 1 << 1;
const DMM32AT_AUX_DOUT0: u8 = 1 << 0;
const DMM32AT_AI_MSB_REG: usize = 0x01;
const DMM32AT_AI_LO_CHAN_REG: usize = 0x02;
const DMM32AT_AI_HI_CHAN_REG: usize = 0x03;
const DMM32AT_AUX_DI_REG: usize = 0x04;
const DMM32AT_AUX_DI_DACBUSY: u8 = 1 << 7;
const DMM32AT_AUX_DI_CALBUSY: u8 = 1 << 6;
const DMM32AT_AUX_DI3: u8 = 1 << 3;
const DMM32AT_AUX_DI2: u8 = 1 << 2;
const DMM32AT_AUX_DI1: u8 = 1 << 1;
const DMM32AT_AUX_DI0: u8 = 1 << 0;
const DMM32AT_AO_LSB_REG: usize = 0x04;
const DMM32AT_AO_MSB_REG: usize = 0x05;
const DMM32AT_FIFO_DEPTH_REG: usize = 0x06;
const DMM32AT_FIFO_CTRL_REG: usize = 0x07;
const DMM32AT_FIFO_CTRL_FIFOEN: u8 = 1 << 3;
const DMM32AT_FIFO_CTRL_SCANEN: u8 = 1 << 2;
const DMM32AT_FIFO_CTRL_FIFORST: u8 = 1 << 1;
const DMM32AT_FIFO_STATUS_REG: usize = 0x07;
const DMM32AT_FIFO_STATUS_EF: u8 = 1 << 7;
const DMM32AT_FIFO_STATUS_HF: u8 = 1 << 6;
const DMM32AT_FIFO_STATUS_FF: u8 = 1 << 5;
const DMM32AT_FIFO_STATUS_OVF: u8 = 1 << 4;
const DMM32AT_FIFO_STATUS_FIFOEN: u8 = 1 << 3;
const DMM32AT_FIFO_STATUS_SCANEN: u8 = 1 << 2;
const DMM32AT_FIFO_STATUS_PAGE_MASK: u8 = 3;
const DMM32AT_CTRL_REG: usize = 0x08;
const DMM32AT_CTRL_RESETA: u8 = 1 << 5;
const DMM32AT_CTRL_RESETD: u8 = 1 << 4;
const DMM32AT_CTRL_INTRST: u8 = 1 << 3;
const DMM32AT_AI_STATUS_REG: usize = 0x08;
const DMM32AT_AI_STATUS_STS: u8 = 1 << 7;
const DMM32AT_AI_STATUS_SD1: u8 = 1 << 6;
const DMM32AT_AI_STATUS_SD0: u8 = 1 << 5;
const DMM32AT_INTCLK_REG: usize = 0x09;
const DMM32AT_INTCLK_ADINT: u8 = 1 << 7;
const DMM32AT_INTCLK_DINT: u8 = 1 << 6;
const DMM32AT_INTCLK_TINT: u8 = 1 << 5;
const DMM32AT_INTCLK_CLKEN: u8 = 1 << 1;
const DMM32AT_INTCLK_CLKSEL: u8 = 1;
const DMM32AT_CTRDIO_CFG_REG: usize = 0x0a;
const DMM32AT_AI_CFG_REG: usize = 0x0b;
const DMM32AT_AI_READBACK_REG: usize = 0x0b;
const DMM32AT_CLK1: usize = 0x0d;
const DMM32AT_CLK2: usize = 0x0e;
const DMM32AT_CLKCT: usize = 0x0f;
const DMM32AT_8255_IOBASE: usize = 0x0c;

const DMM32AT_RANGE_U10: u8 = 0x0c;
const DMM32AT_RANGE_U5: u8 = 0x0d;
const DMM32AT_RANGE_B10: u8 = 0x08;
const DMM32AT_RANGE_B5: u8 = 0x00;
const DMM32AT_CLKCT1: u8 = 0x56;
const DMM32AT_CLKCT2: u8 = 0xb6;

static DMM32AT_AIRANGES: comedi_lrange = comedi_lrange {
    length: 4,
    range: [UNI_RANGE!(10), UNI_RANGE!(5), BIP_RANGE!(10), BIP_RANGE!(5)],
};
static DMM32AT_RANGEBITS: [u8; 4] = [DMM32AT_RANGE_U10, DMM32AT_RANGE_U5,
    DMM32AT_RANGE_B10, DMM32AT_RANGE_B5];
static DMM32AT_AORANGES: comedi_lrange = comedi_lrange {
    length: 4,
    range: [UNI_RANGE!(10), UNI_RANGE!(5), BIP_RANGE!(10), BIP_RANGE!(5)],
};

unsafe fn dmm32at_ai_set_chanspec(dev: *mut comedi_device, s: *mut comedi_subdevice,
                                   chanspec: c_uint, nchan: c_int) {
    let chan = CR_CHAN(chanspec);
    let range = CR_RANGE(chanspec);
    let last_chan = (chan + nchan as c_uint - 1) % (*s).n_chan;
    outb(DMM32AT_FIFO_CTRL_FIFORST, (*dev).iobase + DMM32AT_FIFO_CTRL_REG as c_ulong);
    if nchan > 1 { outb(DMM32AT_FIFO_CTRL_SCANEN, (*dev).iobase + DMM32AT_FIFO_CTRL_REG as c_ulong); }
    outb(chan as u8, (*dev).iobase + DMM32AT_AI_LO_CHAN_REG as c_ulong);
    outb(last_chan as u8, (*dev).iobase + DMM32AT_AI_HI_CHAN_REG as c_ulong);
    outb(DMM32AT_RANGEBITS[range as usize], (*dev).iobase + DMM32AT_AI_CFG_REG as c_ulong);
}

unsafe fn dmm32at_ai_get_sample(dev: *mut comedi_device, s: *mut comedi_subdevice) -> c_uint {
    let mut val = inb((*dev).iobase + DMM32AT_AI_LSB_REG as c_ulong) as c_uint;
    val |= (inb((*dev).iobase + DMM32AT_AI_MSB_REG as c_ulong) as c_uint) << 8;
    comedi_offset_munge(s, val)
}

unsafe extern "C" fn dmm32at_ai_status(dev: *mut comedi_device, _s: *mut comedi_subdevice,
                                         _insn: *mut comedi_insn, context: c_ulong) -> c_int {
    if inb((*dev).iobase + context) & DMM32AT_AI_STATUS_STS == 0 { 0 } else { -EBUSY }
}

unsafe extern "C" fn dmm32at_ai_insn_read(dev: *mut comedi_device, s: *mut comedi_subdevice,
                                            insn: *mut comedi_insn, data: *mut c_uint) -> c_int {
    dmm32at_ai_set_chanspec(dev, s, (*insn).chanspec, 1);
    let mut ret = comedi_timeout(dev, s, insn, Some(dmm32at_ai_status), DMM32AT_AI_READBACK_REG as c_ulong);
    if ret != 0 { return ret; }
    for i in 0..(*insn).n as isize {
        outb(0xff, (*dev).iobase + DMM32AT_AI_START_CONV_REG as c_ulong);
        ret = comedi_timeout(dev, s, insn, Some(dmm32at_ai_status), DMM32AT_AI_STATUS_REG as c_ulong);
        if ret != 0 { return ret; }
        *data.offset(i) = dmm32at_ai_get_sample(dev, s);
    }
    (*insn).n as c_int
}

unsafe extern "C" fn dmm32at_ai_check_chanlist(dev: *mut comedi_device, s: *mut comedi_subdevice,
                                                 cmd: *mut comedi_cmd) -> c_int {
    let chan0 = CR_CHAN((*cmd).chanlist[0]); let range0 = CR_RANGE((*cmd).chanlist[0]);
    for i in 1..(*cmd).chanlist_len as usize {
        let chan = CR_CHAN((*cmd).chanlist[i]); let range = CR_RANGE((*cmd).chanlist[i]);
        if chan != (chan0 + i as c_uint) % (*s).n_chan { dev_dbg((*dev).class_dev, "entries in chanlist must be consecutive channels, counting upwards\0"); return -EINVAL; }
        if range != range0 { dev_dbg((*dev).class_dev, "entries in chanlist must all have the same gain\0"); return -EINVAL; }
    }
    0
}

unsafe extern "C" fn dmm32at_ai_cmdtest(dev: *mut comedi_device, s: *mut comedi_subdevice,
                                          cmd: *mut comedi_cmd) -> c_int {
    let mut err = 0;
    err |= comedi_check_trigger_src(&mut (*cmd).start_src, TRIG_NOW);
    err |= comedi_check_trigger_src(&mut (*cmd).scan_begin_src, TRIG_TIMER);
    err |= comedi_check_trigger_src(&mut (*cmd).convert_src, TRIG_TIMER);
    err |= comedi_check_trigger_src(&mut (*cmd).scan_end_src, TRIG_COUNT);
    err |= comedi_check_trigger_src(&mut (*cmd).stop_src, TRIG_COUNT | TRIG_NONE);
    if err != 0 { return 1; }
    err |= comedi_check_trigger_is_unique((*cmd).stop_src); if err != 0 { return 2; }
    err |= comedi_check_trigger_arg_is(&mut (*cmd).start_arg, 0);
    err |= comedi_check_trigger_arg_min(&mut (*cmd).scan_begin_arg, 1_000_000);
    err |= comedi_check_trigger_arg_max(&mut (*cmd).scan_begin_arg, 1_000_000_000);
    (*cmd).convert_arg = if (*cmd).convert_arg >= 17500 { 20000 } else if (*cmd).convert_arg >= 12500 { 15000 } else if (*cmd).convert_arg >= 7500 { 10000 } else { 5000 };
    err |= comedi_check_trigger_arg_is(&mut (*cmd).scan_end_arg, (*cmd).chanlist_len);
    if (*cmd).stop_src == TRIG_COUNT { err |= comedi_check_trigger_arg_min(&mut (*cmd).stop_arg, 1); } else { err |= comedi_check_trigger_arg_is(&mut (*cmd).stop_arg, 0); }
    if err != 0 { return 3; }
    err |= comedi_check_trigger_arg_min(&mut (*cmd).scan_begin_arg, (*cmd).convert_arg * (*cmd).scan_end_arg);
    if err != 0 { return 4; }
    if !(*cmd).chanlist.is_null() && (*cmd).chanlist_len > 0 { err |= dmm32at_ai_check_chanlist(dev, s, cmd); }
    if err != 0 { 5 } else { 0 }
}

unsafe fn dmm32at_setaitimer(dev: *mut comedi_device, nansec: c_uint) {
    let lo1: u8 = 200; let both2: u16 = (nansec / 20000) as u16;
    outb(0, (*dev).iobase + DMM32AT_CTRDIO_CFG_REG as c_ulong);
    let mut flags = 0; spin_lock_irqsave(&mut (*dev).spinlock, &mut flags);
    outb(0, (*dev).iobase + DMM32AT_CTRL_REG as c_ulong);
    outb(DMM32AT_CLKCT1, (*dev).iobase + DMM32AT_CLKCT as c_ulong); outb(lo1, (*dev).iobase + DMM32AT_CLK1 as c_ulong);
    outb(DMM32AT_CLKCT2, (*dev).iobase + DMM32AT_CLKCT as c_ulong); outb((both2 & 0xff) as u8, (*dev).iobase + DMM32AT_CLK2 as c_ulong); outb((both2 >> 8) as u8, (*dev).iobase + DMM32AT_CLK2 as c_ulong);
    spin_unlock_irqrestore(&mut (*dev).spinlock, flags);
    outb(DMM32AT_INTCLK_ADINT | DMM32AT_INTCLK_CLKEN | DMM32AT_INTCLK_CLKSEL, (*dev).iobase + DMM32AT_INTCLK_REG as c_ulong);
}

unsafe extern "C" fn dmm32at_ai_cmd(dev: *mut comedi_device, s: *mut comedi_subdevice) -> c_int {
    let cmd = &mut (*(*s).async_).cmd;
    dmm32at_ai_set_chanspec(dev, s, cmd.chanlist[0], cmd.chanlist_len as c_int);
    let mut flags = 0; spin_lock_irqsave(&mut (*dev).spinlock, &mut flags); outb(DMM32AT_CTRL_INTRST, (*dev).iobase + DMM32AT_CTRL_REG as c_ulong); spin_unlock_irqrestore(&mut (*dev).spinlock, flags);
    let ret = comedi_timeout(dev, s, core::ptr::null_mut(), Some(dmm32at_ai_status), DMM32AT_AI_READBACK_REG as c_ulong); if ret != 0 { return ret; }
    if cmd.stop_src == TRIG_NONE || cmd.stop_arg > 1 { dmm32at_setaitimer(dev, cmd.scan_begin_arg); } else { outb(DMM32AT_INTCLK_ADINT, (*dev).iobase + DMM32AT_INTCLK_REG as c_ulong); outb(0xff, (*dev).iobase + DMM32AT_AI_START_CONV_REG as c_ulong); } 0
}

unsafe extern "C" fn dmm32at_ai_cancel(dev: *mut comedi_device, _s: *mut comedi_subdevice) -> c_int { outb(0, (*dev).iobase + DMM32AT_INTCLK_REG as c_ulong); 0 }

unsafe extern "C" fn dmm32at_isr(_irq: c_int, d: *mut c_void) -> irqreturn_t {
    let dev = d as *mut comedi_device; if (*dev).attached == 0 { dev_err((*dev).class_dev, "spurious interrupt\0"); return IRQ_HANDLED; }
    let intstat = inb((*dev).iobase + DMM32AT_INTCLK_REG as c_ulong);
    if intstat & DMM32AT_INTCLK_ADINT != 0 { let s = (*dev).read_subdev; let cmd = &mut (*(*s).async_).cmd; for _ in 0..cmd.chanlist_len { let mut val = dmm32at_ai_get_sample(dev, s) as u16; comedi_buf_write_samples(s, &mut val, 1); } if cmd.stop_src == TRIG_COUNT && (*(*s).async_).scans_done >= cmd.stop_arg { (*(*s).async_).events |= COMEDI_CB_EOA; } comedi_handle_events(dev, s); }
    spin_lock(&mut (*dev).spinlock); outb(DMM32AT_CTRL_INTRST, (*dev).iobase + DMM32AT_CTRL_REG as c_ulong); spin_unlock(&mut (*dev).spinlock); IRQ_HANDLED
}

unsafe extern "C" fn dmm32at_ao_eoc(dev: *mut comedi_device, _s: *mut comedi_subdevice, _insn: *mut comedi_insn, _context: c_ulong) -> c_int { if inb((*dev).iobase + DMM32AT_AUX_DI_REG as c_ulong) & DMM32AT_AUX_DI_DACBUSY == 0 { 0 } else { -EBUSY } }

unsafe extern "C" fn dmm32at_ao_insn_write(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut c_uint) -> c_int {
    let chan = CR_CHAN((*insn).chanspec); for i in 0..(*insn).n as isize { let val = *data.offset(i); outb((val & 0xff) as u8, (*dev).iobase + DMM32AT_AO_LSB_REG as c_ulong); outb(((val >> 8) | (chan << 6)) as u8, (*dev).iobase + DMM32AT_AO_MSB_REG as c_ulong); let ret = comedi_timeout(dev, s, insn, Some(dmm32at_ao_eoc), 0); if ret != 0 { return ret; } inb((*dev).iobase + DMM32AT_AO_MSB_REG as c_ulong); (*s).readback[chan as usize] = val; } (*insn).n as c_int
}

unsafe extern "C" fn dmm32at_8255_io(dev: *mut comedi_device, dir: c_int, port: c_int, data: c_int, regbase: c_ulong) -> c_int { let mut flags = 0; spin_lock_irqsave(&mut (*dev).spinlock, &mut flags); outb(1, (*dev).iobase + DMM32AT_CTRL_REG as c_ulong); let ret = if dir != 0 { outb(data as u8, (*dev).iobase + regbase + port as c_ulong); 0 } else { inb((*dev).iobase + regbase + port as c_ulong) as c_int }; spin_unlock_irqrestore(&mut (*dev).spinlock, flags); ret }

unsafe fn dmm32at_reset(dev: *mut comedi_device) -> c_int {
    outb(DMM32AT_CTRL_RESETA, (*dev).iobase + DMM32AT_CTRL_REG as c_ulong); usleep_range(1000, 3000);
    outb(0, (*dev).iobase + DMM32AT_FIFO_CTRL_REG as c_ulong); outb(0, (*dev).iobase + DMM32AT_INTCLK_REG as c_ulong);
    outb(0x80, (*dev).iobase + DMM32AT_AI_LO_CHAN_REG as c_ulong); outb(0xff, (*dev).iobase + DMM32AT_AI_HI_CHAN_REG as c_ulong); outb(DMM32AT_RANGE_U10, (*dev).iobase + DMM32AT_AI_CFG_REG as c_ulong); usleep_range(100, 200);
    let ailo = inb((*dev).iobase + DMM32AT_AI_LO_CHAN_REG as c_ulong); let aihi = inb((*dev).iobase + DMM32AT_AI_HI_CHAN_REG as c_ulong); let fifostat = inb((*dev).iobase + DMM32AT_FIFO_STATUS_REG as c_ulong); let aistat = inb((*dev).iobase + DMM32AT_AI_STATUS_REG as c_ulong); let intstat = inb((*dev).iobase + DMM32AT_INTCLK_REG as c_ulong); let airback = inb((*dev).iobase + DMM32AT_AI_READBACK_REG as c_ulong);
    if ailo != 0 || aihi != 0x1f || fifostat != DMM32AT_FIFO_STATUS_EF || aistat != (DMM32AT_AI_STATUS_SD1 | DMM32AT_AI_STATUS_SD0) || intstat != 0 || airback != 0x0c { -EIO } else { 0 }
}

unsafe fn dmm32at_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> c_int {
    let iobase = (*it).options[0]; let valid = matches!(iobase, 0x100 | 0x140 | 0x180 | 0x200 | 0x280 | 0x300 | 0x340 | 0x380);
    if !valid { dev_err((*dev).class_dev, "unsupported base address %#x\0", iobase); return -EINVAL; }
    let mut ret = comedi_request_region(dev, iobase, 0x10); if ret != 0 { return ret; }
    ret = dmm32at_reset(dev); if ret != 0 { dev_err((*dev).class_dev, "board detection failed\0"); return ret; }
    if (*it).options[1] != 0 { ret = request_irq((*it).options[1], Some(dmm32at_isr), 0, (*dev).board_name, dev as *mut c_void); if ret == 0 { (*dev).irq = (*it).options[1]; } }
    ret = comedi_alloc_subdevices(dev, 3); if ret != 0 { return ret; }
    let s = &mut (*dev).subdevices[0]; s.type_ = COMEDI_SUBD_AI; s.subdev_flags = SDF_READABLE | SDF_GROUND | SDF_DIFF; s.n_chan = 32; s.maxdata = 0xffff; s.range_table = &DMM32AT_AIRANGES; s.insn_read = Some(dmm32at_ai_insn_read);
    if (*dev).irq != 0 { (*dev).read_subdev = s; s.subdev_flags |= SDF_CMD_READ; s.len_chanlist = s.n_chan; s.do_cmd = Some(dmm32at_ai_cmd); s.do_cmdtest = Some(dmm32at_ai_cmdtest); s.cancel = Some(dmm32at_ai_cancel); }
    let s = &mut (*dev).subdevices[1]; s.type_ = COMEDI_SUBD_AO; s.subdev_flags = SDF_WRITABLE; s.n_chan = 4; s.maxdata = 0x0fff; s.range_table = &DMM32AT_AORANGES; s.insn_write = Some(dmm32at_ao_insn_write); ret = comedi_alloc_subdev_readback(s); if ret != 0 { return ret; }
    subdev_8255_cb_init(dev, &mut (*dev).subdevices[2], Some(dmm32at_8255_io), DMM32AT_8255_IOBASE as c_ulong)
}

// Equivalent module registration metadata: driver name "dmm32at", attach=dmm32at_attach,
// detach=comedi_legacy_detach, author="Comedi https://www.comedi.org",
// description="Comedi: Diamond Systems Diamond-MM-32-AT", license="GPL".

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
