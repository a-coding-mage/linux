// SPDX-License-Identifier: GPL-2.0+
/* Direct Rust translation of s526.c. External Linux/Comedi symbols are supplied by dependencies. */

const S526_TIMER_REG: u32 = 0x00;
const fn s526_timer_load(x: u32) -> u32 { (x & 0xff) << 8 }
const fn s526_timer_mode(x: u32) -> u32 { x << 1 }
const S526_TIMER_MANUAL: u32 = s526_timer_mode(0);
const S526_TIMER_AUTO: u32 = s526_timer_mode(1);
const S526_TIMER_RESTART: u32 = 1 << 0;
const S526_WDOG_REG: u32 = 0x02;
const S526_WDOG_INVERTED: u32 = 1 << 4;
const S526_WDOG_ENA: u32 = 1 << 3;
const fn s526_wdog_interval(x: u32) -> u32 { (x & 0x7) << 0 }
const S526_AO_CTRL_REG: u32 = 0x04;
const S526_AO_CTRL_RESET: u32 = 1 << 3;
const fn s526_ao_ctrl_chan(x: u32) -> u32 { (x & 0x3) << 1 }
const S526_AO_CTRL_START: u32 = 1 << 0;
const S526_AI_CTRL_REG: u32 = 0x06;
const S526_AI_CTRL_DELAY: u32 = 1 << 15;
const fn s526_ai_ctrl_conv(x: u32) -> u32 { 1 << (5 + (x & 0x9)) }
const fn s526_ai_ctrl_read(x: u32) -> u32 { (x & 0xf) << 1 }
const S526_AI_CTRL_START: u32 = 1 << 0;
const S526_AO_REG: u32 = 0x08;
const S526_AI_REG: u32 = 0x08;
const S526_DIO_CTRL_REG: u32 = 0x0a;
const S526_DIO_CTRL_GRP2_OUT: u32 = 1 << 11;
const S526_DIO_CTRL_GRP1_OUT: u32 = 1 << 10;
const S526_INT_STATUS_REG: u32 = 0x0e;
const S526_INT_AI: u32 = 1 << 2;
const S526_INT_AO: u32 = 1 << 1;
const fn s526_gpct_lsb_reg(x: u32) -> u32 { 0x12 + x * 8 }
const fn s526_gpct_msb_reg(x: u32) -> u32 { 0x14 + x * 8 }
const fn s526_gpct_mode_reg(x: u32) -> u32 { 0x16 + x * 8 }
const fn s526_gpct_mode_pr_select(x: u32) -> u32 { x << 14 }
const S526_GPCT_MODE_PR_SELECT_MASK: u32 = s526_gpct_mode_pr_select(0x1);
const S526_GPCT_MODE_PR_SELECT_PR0: u32 = s526_gpct_mode_pr_select(0);
const S526_GPCT_MODE_PR_SELECT_PR1: u32 = s526_gpct_mode_pr_select(1);
const fn s526_gpct_mode_autoload(x: u32) -> u32 { x << 2 }
const S526_GPCT_MODE_AUTOLOAD_MASK: u32 = s526_gpct_mode_autoload(0x7);
const S526_GPCT_MODE_AUTOLOAD_NONE: u32 = s526_gpct_mode_autoload(0);
const fn s526_gpct_ctrl_reg(x: u32) -> u32 { 0x18 + x * 8 }
const S526_GPCT_CTRL_CT_RESET: u32 = 1 << 15;

#[repr(C)]
pub struct s526_private {
    pub gpct_config: [u32; 4],
    pub ai_ctrl: u16,
}

unsafe fn s526_gpct_write(dev: *mut comedi_device, chan: u32, val: u32) {
    outw((val >> 16) & 0xffff, (*dev).iobase + s526_gpct_msb_reg(chan));
    outw(val & 0xffff, (*dev).iobase + s526_gpct_lsb_reg(chan));
}

unsafe fn s526_gpct_read(dev: *mut comedi_device, chan: u32) -> u32 {
    let mut val = inw((*dev).iobase + s526_gpct_lsb_reg(chan)) & 0xffff;
    val |= (inw((*dev).iobase + s526_gpct_msb_reg(chan)) & 0xff) << 16;
    val
}

unsafe fn s526_gpct_rinsn(dev: *mut comedi_device, _s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let chan = CR_CHAN((*insn).chanspec);
    for i in 0..(*insn).n { *data.add(i as usize) = s526_gpct_read(dev, chan); }
    (*insn).n as i32
}

unsafe fn s526_gpct_insn_config(dev: *mut comedi_device, _s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let p = (*dev).private as *mut s526_private;
    let chan = CR_CHAN((*insn).chanspec);
    let typ = *data;
    (*p).gpct_config[chan as usize] = typ;
    match typ {
        INSN_CONFIG_GPCT_QUADRATURE_ENCODER => {
            let val = *data.add(1) & 0xffff;
            outw(val, (*dev).iobase + s526_gpct_mode_reg(chan));
            if (val & S526_GPCT_MODE_AUTOLOAD_MASK) == S526_GPCT_MODE_AUTOLOAD_NONE {
                outw(S526_GPCT_CTRL_CT_RESET, (*dev).iobase + s526_gpct_ctrl_reg(chan));
            }
        }
        INSN_CONFIG_GPCT_SINGLE_PULSE_GENERATOR | INSN_CONFIG_GPCT_PULSE_TRAIN_GENERATOR => {
            let mut val = *data.add(1) & 0xffff;
            val = (val & !S526_GPCT_MODE_PR_SELECT_MASK) | S526_GPCT_MODE_PR_SELECT_PR0;
            outw(val, (*dev).iobase + s526_gpct_mode_reg(chan));
            s526_gpct_write(dev, chan, *data.add(2));
            val = (*data.add(1) & 0xffff & !S526_GPCT_MODE_PR_SELECT_MASK) | S526_GPCT_MODE_PR_SELECT_PR1;
            outw(val, (*dev).iobase + s526_gpct_mode_reg(chan));
            s526_gpct_write(dev, chan, *data.add(3));
            let ctrl = *data.add(4);
            if ctrl != 0 { outw(ctrl & 0xffff, (*dev).iobase + s526_gpct_ctrl_reg(chan)); }
        }
        _ => return -EINVAL,
    }
    (*insn).n as i32
}

unsafe fn s526_gpct_winsn(dev: *mut comedi_device, _s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let p = (*dev).private as *mut s526_private;
    let chan = CR_CHAN((*insn).chanspec);
    let _ = inw((*dev).iobase + s526_gpct_mode_reg(chan));
    match (*p).gpct_config[chan as usize] {
        INSN_CONFIG_GPCT_PULSE_TRAIN_GENERATOR => {
            if *data.add(1) <= *data || *data == 0 { return -EINVAL; }
            s526_gpct_write(dev, chan, *data);
        }
        INSN_CONFIG_GPCT_QUADRATURE_ENCODER | INSN_CONFIG_GPCT_SINGLE_PULSE_GENERATOR => s526_gpct_write(dev, chan, *data),
        _ => return -EINVAL,
    }
    (*insn).n as i32
}

unsafe fn s526_eoc(dev: *mut comedi_device, _s: *mut comedi_subdevice, _insn: *mut comedi_insn, context: usize) -> i32 {
    let status = inw((*dev).iobase + S526_INT_STATUS_REG);
    if status & context as u32 != 0 { outw(context as u32, (*dev).iobase + S526_INT_STATUS_REG); 0 } else { -EBUSY }
}

unsafe fn s526_ai_insn_read(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let p = (*dev).private as *mut s526_private;
    let chan = CR_CHAN((*insn).chanspec);
    let mut ctrl = s526_ai_ctrl_conv(chan) | s526_ai_ctrl_read(chan) | S526_AI_CTRL_START;
    if ctrl != (*p).ai_ctrl { (*p).ai_ctrl = ctrl as u16; ctrl |= S526_AI_CTRL_DELAY; }
    for i in 0..(*insn).n {
        outw(ctrl, (*dev).iobase + S526_AI_CTRL_REG); ctrl &= !S526_AI_CTRL_DELAY;
        let ret = comedi_timeout(dev, s, insn, Some(s526_eoc), S526_INT_AI);
        if ret != 0 { return ret; }
        *data.add(i as usize) = comedi_offset_munge(s, inw((*dev).iobase + S526_AI_REG));
    }
    (*insn).n as i32
}

unsafe fn s526_ao_insn_write(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let chan = CR_CHAN((*insn).chanspec);
    let mut ctrl = s526_ao_ctrl_chan(chan);
    let mut val = (*s).readback[chan as usize];
    outw(ctrl, (*dev).iobase + S526_AO_CTRL_REG); ctrl |= S526_AO_CTRL_START;
    for i in 0..(*insn).n { val = *data.add(i as usize); outw(val, (*dev).iobase + S526_AO_REG); outw(ctrl, (*dev).iobase + S526_AO_CTRL_REG); let ret = comedi_timeout(dev, s, insn, Some(s526_eoc), S526_INT_AO); if ret != 0 { return ret; } }
    (*s).readback[chan as usize] = val; (*insn).n as i32
}

unsafe fn s526_dio_insn_bits(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    if comedi_dio_update_state(s, data) != 0 { outw((*s).state, (*dev).iobase + S526_DIO_CTRL_REG); }
    *data.add(1) = inw((*dev).iobase + S526_DIO_CTRL_REG) & 0xff; (*insn).n as i32
}

unsafe fn s526_dio_insn_config(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let chan = CR_CHAN((*insn).chanspec); let mask = if chan < 4 { 0x0f } else { 0xf0 };
    let ret = comedi_dio_insn_config(dev, s, insn, data, mask); if ret != 0 { return ret; }
    if (*s).io_bits & 0x0f != 0 { (*s).state |= S526_DIO_CTRL_GRP1_OUT; } else { (*s).state &= !S526_DIO_CTRL_GRP1_OUT; }
    if (*s).io_bits & 0xf0 != 0 { (*s).state |= S526_DIO_CTRL_GRP2_OUT; } else { (*s).state &= !S526_DIO_CTRL_GRP2_OUT; }
    outw((*s).state, (*dev).iobase + S526_DIO_CTRL_REG); (*insn).n as i32
}

unsafe fn s526_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> i32 {
    let ret = comedi_check_request_region(dev, (*it).options[0], 0x40, 0, 0xffc0, 64); if ret != 0 { return ret; }
    let p = comedi_alloc_devpriv(dev, core::mem::size_of::<s526_private>()); if p.is_null() { return -ENOMEM; }
    let ret = comedi_alloc_subdevices(dev, 4); if ret != 0 { return ret; }
    let s = &mut (*dev).subdevices[0]; s.type_ = COMEDI_SUBD_COUNTER; s.subdev_flags = SDF_READABLE | SDF_WRITABLE | SDF_LSAMPL; s.n_chan = 4; s.maxdata = 0x00ffffff; s.insn_read = Some(s526_gpct_rinsn); s.insn_config = Some(s526_gpct_insn_config); s.insn_write = Some(s526_gpct_winsn);
    let s = &mut (*dev).subdevices[1]; s.type_ = COMEDI_SUBD_AI; s.subdev_flags = SDF_READABLE | SDF_DIFF; s.n_chan = 10; s.maxdata = 0xffff; s.range_table = &range_bipolar10; s.len_chanlist = 16; s.insn_read = Some(s526_ai_insn_read);
    let s = &mut (*dev).subdevices[2]; s.type_ = COMEDI_SUBD_AO; s.subdev_flags = SDF_WRITABLE; s.n_chan = 4; s.maxdata = 0xffff; s.range_table = &range_bipolar10; s.insn_write = Some(s526_ao_insn_write); let ret = comedi_alloc_subdev_readback(s); if ret != 0 { return ret; }
    let s = &mut (*dev).subdevices[3]; s.type_ = COMEDI_SUBD_DIO; s.subdev_flags = SDF_READABLE | SDF_WRITABLE; s.n_chan = 8; s.maxdata = 1; s.range_table = &range_digital; s.insn_bits = Some(s526_dio_insn_bits); s.insn_config = Some(s526_dio_insn_config); 0
}

// Equivalent driver registration and module metadata:
// static struct comedi_driver s526_driver = { .driver_name = "s526", .module = THIS_MODULE, .attach = s526_attach, .detach = comedi_legacy_detach };
// module_comedi_driver(s526_driver);
// MODULE_AUTHOR("Comedi https://www.comedi.org"); MODULE_DESCRIPTION("Comedi low-level driver"); MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
