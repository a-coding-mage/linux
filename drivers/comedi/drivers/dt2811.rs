// SPDX-License-Identifier: GPL-2.0+
/*
 * Comedi driver for Data Translation DT2811
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) David A. Schleef <ds@schleef.org>
 */

// Linux/comedi dependencies supplied by the surrounding translation unit.

const DT2811_ADCSR_REG: u32 = 0x00;
const DT2811_ADCSR_ADDONE: u32 = 1 << 7;
const DT2811_ADCSR_ADERROR: u32 = 1 << 6;
const DT2811_ADCSR_ADBUSY: u32 = 1 << 5;
const DT2811_ADCSR_CLRERROR: u32 = 1 << 4;
const DT2811_ADCSR_DMAENB: u32 = 1 << 3;
const DT2811_ADCSR_INTENB: u32 = 1 << 2;
const fn dt2811_adcsr_admode(x: u32) -> u32 { (x & 0x3) << 0 }

const DT2811_ADGCR_REG: u32 = 0x01;
const fn dt2811_adgcr_gain(x: u32) -> u32 { (x & 0x3) << 6 }
const fn dt2811_adgcr_chan(x: u32) -> u32 { (x & 0xf) << 0 }

const DT2811_ADDATA_LO_REG: u32 = 0x02;
const DT2811_ADDATA_HI_REG: u32 = 0x03;
const fn dt2811_dadata_lo_reg(x: u32) -> u32 { 0x02 + x * 2 }
const fn dt2811_dadata_hi_reg(x: u32) -> u32 { 0x03 + x * 2 }
const DT2811_DI_REG: u32 = 0x06;
const DT2811_DO_REG: u32 = 0x06;
const DT2811_TMRCTR_REG: u32 = 0x07;
const fn dt2811_tmrctr_mantissa(x: u32) -> u32 { (x & 0x7) << 3 }
const fn dt2811_tmrctr_exponent(x: u32) -> u32 { (x & 0x7) << 0 }
const DT2811_OSC_BASE: u64 = 1666;

static DT2811_CLK_DIVIDERS: [u32; 8] = [1, 10, 2, 3, 4, 5, 6, 12];
static DT2811_CLK_MULTIPLIERS: [u32; 8] = [1, 10, 100, 1000, 10000, 100000, 1000000, 10000000];

static DT2811_PGH_AI_RANGES: comedi_lrange = comedi_lrange {
    length: 12,
    range: [
        BIP_RANGE!(5), BIP_RANGE!(2.5), BIP_RANGE!(1.25), BIP_RANGE!(0.625),
        BIP_RANGE!(2.5), BIP_RANGE!(1.25), BIP_RANGE!(0.625), BIP_RANGE!(0.3125),
        UNI_RANGE!(5), UNI_RANGE!(2.5), UNI_RANGE!(1.25), UNI_RANGE!(0.625),
    ],
};
static DT2811_PGL_AI_RANGES: comedi_lrange = comedi_lrange {
    length: 12,
    range: [
        BIP_RANGE!(5), BIP_RANGE!(0.5), BIP_RANGE!(0.05), BIP_RANGE!(0.01),
        BIP_RANGE!(2.5), BIP_RANGE!(0.25), BIP_RANGE!(0.025), BIP_RANGE!(0.005),
        UNI_RANGE!(5), UNI_RANGE!(0.5), UNI_RANGE!(0.05), UNI_RANGE!(0.01),
    ],
};
static DT2811_AO_RANGES: comedi_lrange = comedi_lrange {
    length: 3,
    range: [BIP_RANGE!(5), BIP_RANGE!(2.5), UNI_RANGE!(5)],
};

#[repr(C)]
struct dt2811_board { name: *const c_char, is_pgh: u32 }
static DT2811_BOARDS: [dt2811_board; 2] = [
    dt2811_board { name: c"dt2811-pgh".as_ptr(), is_pgh: 1 },
    dt2811_board { name: c"dt2811-pgl".as_ptr(), is_pgh: 0 },
];

#[repr(C)]
struct dt2811_private { ai_divisor: u32 }

unsafe fn dt2811_ai_read_sample(dev: *mut comedi_device, s: *mut comedi_subdevice) -> u32 {
    let val = inb((*dev).iobase + DT2811_ADDATA_LO_REG)
        | (inb((*dev).iobase + DT2811_ADDATA_HI_REG) << 8);
    val & (*s).maxdata
}

unsafe extern "C" fn dt2811_interrupt(_irq: c_int, d: *mut c_void) -> irqreturn_t {
    let dev = d as *mut comedi_device;
    let s = (*dev).read_subdev;
    if !(*dev).attached { return IRQ_NONE; }
    let async_ = (*s).async_;
    let cmd = &mut (*async_).cmd;
    let status = inb((*dev).iobase + DT2811_ADCSR_REG);
    if status & DT2811_ADCSR_ADERROR != 0 {
        (*async_).events |= COMEDI_CB_OVERFLOW;
        outb(status | DT2811_ADCSR_CLRERROR, (*dev).iobase + DT2811_ADCSR_REG);
    }
    if status & DT2811_ADCSR_ADDONE != 0 {
        let val = dt2811_ai_read_sample(dev, s) as u16;
        comedi_buf_write_samples(s, &val as *const u16, 1);
    }
    if cmd.stop_src == TRIG_COUNT && (*async_).scans_done >= cmd.stop_arg {
        (*async_).events |= COMEDI_CB_EOA;
    }
    comedi_handle_events(dev, s);
    IRQ_HANDLED
}

unsafe fn dt2811_ai_cancel(dev: *mut comedi_device, _s: *mut comedi_subdevice) -> c_int {
    outb(dt2811_adcsr_admode(0), (*dev).iobase + DT2811_ADCSR_REG); 0
}

unsafe fn dt2811_ai_set_chanspec(dev: *mut comedi_device, chanspec: u32) {
    let chan = CR_CHAN(chanspec); let range = CR_RANGE(chanspec);
    outb(dt2811_adgcr_chan(chan) | dt2811_adgcr_gain(range), (*dev).iobase + DT2811_ADGCR_REG);
}

unsafe fn dt2811_ai_cmd(dev: *mut comedi_device, s: *mut comedi_subdevice) -> c_int {
    let devpriv = (*dev).private as *mut dt2811_private;
    let cmd = &mut (*(*s).async_).cmd;
    let mode = if cmd.start_src == TRIG_NOW { dt2811_adcsr_admode(1) } else if cmd.convert_src == TRIG_TIMER { dt2811_adcsr_admode(2) } else { dt2811_adcsr_admode(3) };
    outb(mode | DT2811_ADCSR_INTENB, (*dev).iobase + DT2811_ADCSR_REG);
    outb((*devpriv).ai_divisor, (*dev).iobase + DT2811_TMRCTR_REG);
    dt2811_ai_set_chanspec(dev, *cmd.chanlist);
    0
}

unsafe fn dt2811_ns_to_timer(nanosec: *mut u32, flags: u32) -> u32 {
    let mut ns_lo = COMEDI_MIN_SPEED as u64; let mut ns_hi = 0u64;
    let mut divisor_hi = 0; let mut divisor_lo = 0;
    for div_i in 0..8 { for mult_i in 0..8 {
        let ns = DT2811_CLK_DIVIDERS[div_i] as u64 * DT2811_CLK_MULTIPLIERS[mult_i] as u64 * DT2811_OSC_BASE;
        if ns > COMEDI_MIN_SPEED as u64 { continue; }
        let divisor = dt2811_tmrctr_mantissa(div_i as u32) | dt2811_tmrctr_exponent(mult_i as u32);
        if ns <= *nanosec as u64 && ns > ns_hi { ns_hi = ns; divisor_hi = divisor; }
        if ns >= *nanosec as u64 && ns < ns_lo { ns_lo = ns; divisor_lo = divisor; }
    }}
    if ns_lo == COMEDI_MIN_SPEED as u64 { ns_lo = ns_hi; divisor_lo = divisor_hi; }
    if ns_hi == 0 { ns_hi = ns_lo; divisor_hi = divisor_lo; }
    match flags & CMDF_ROUND_MASK { CMDF_ROUND_UP => {*nanosec = ns_lo as u32; divisor_lo}, CMDF_ROUND_DOWN => {*nanosec = ns_hi as u32; divisor_hi}, _ => if ns_hi - *nanosec as u64 < *nanosec as u64 - ns_lo {*nanosec = ns_lo as u32; divisor_lo} else {*nanosec = ns_hi as u32; divisor_hi} }
}

unsafe fn dt2811_ai_eoc(dev: *mut comedi_device, _s: *mut comedi_subdevice, _insn: *mut comedi_insn, _context: c_ulong) -> c_int {
    if inb((*dev).iobase + DT2811_ADCSR_REG) & DT2811_ADCSR_ADBUSY == 0 { 0 } else { -EBUSY }
}

unsafe fn dt2811_ai_insn_read(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> c_int {
    for i in 0..(*insn).n { dt2811_ai_set_chanspec(dev, (*insn).chanspec); let ret = comedi_timeout(dev, s, insn, Some(dt2811_ai_eoc), 0); if ret != 0 { return ret; } *data.add(i as usize) = dt2811_ai_read_sample(dev, s); } (*insn).n as c_int
}

unsafe fn dt2811_ao_insn_write(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> c_int {
    let chan = CR_CHAN((*insn).chanspec); let mut val = *(*s).readback.add(chan as usize);
    for i in 0..(*insn).n { val = *data.add(i as usize); outb(val & 0xff, (*dev).iobase + dt2811_dadata_lo_reg(chan)); outb((val >> 8) & 0xff, (*dev).iobase + dt2811_dadata_hi_reg(chan)); }
    *(*s).readback.add(chan as usize) = val; (*insn).n as c_int
}

unsafe fn dt2811_di_insn_bits(dev: *mut comedi_device, _s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> c_int { *data.add(1) = inb((*dev).iobase + DT2811_DI_REG); (*insn).n as c_int }
unsafe fn dt2811_do_insn_bits(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> c_int { if comedi_dio_update_state(s, data) != 0 { outb((*s).state, (*dev).iobase + DT2811_DO_REG); } *data.add(1) = (*s).state; (*insn).n as c_int }

unsafe fn dt2811_reset(dev: *mut comedi_device) { outb(dt2811_adcsr_admode(0), (*dev).iobase + DT2811_ADCSR_REG); usleep_range(100, 1000); inb((*dev).iobase + DT2811_ADDATA_LO_REG); inb((*dev).iobase + DT2811_ADDATA_HI_REG); outb(dt2811_adcsr_admode(0) | DT2811_ADCSR_CLRERROR, (*dev).iobase + DT2811_ADCSR_REG); }

unsafe fn dt2811_ai_cmdtest(dev: *mut comedi_device, s: *mut comedi_subdevice, cmd: *mut comedi_cmd) -> c_int {
    let devpriv = (*dev).private as *mut dt2811_private; let mut err = 0; let mut arg;
    err |= comedi_check_trigger_src(&mut (*cmd).start_src, TRIG_NOW | TRIG_EXT);
    err |= comedi_check_trigger_src(&mut (*cmd).scan_begin_src, TRIG_FOLLOW);
    err |= comedi_check_trigger_src(&mut (*cmd).convert_src, TRIG_TIMER | TRIG_EXT);
    err |= comedi_check_trigger_src(&mut (*cmd).scan_end_src, TRIG_COUNT);
    err |= comedi_check_trigger_src(&mut (*cmd).stop_src, TRIG_COUNT | TRIG_NONE); if err != 0 { return 1; }
    err |= comedi_check_trigger_is_unique((*cmd).start_src); err |= comedi_check_trigger_is_unique((*cmd).convert_src); err |= comedi_check_trigger_is_unique((*cmd).stop_src);
    if (*cmd).convert_src == TRIG_EXT && (*cmd).start_src != TRIG_EXT { err |= -EINVAL; } if err != 0 { return 2; }
    err |= comedi_check_trigger_arg_is(&mut (*cmd).start_arg, 0); err |= comedi_check_trigger_arg_is(&mut (*cmd).scan_begin_arg, 0);
    if (*cmd).convert_src == TRIG_TIMER { err |= comedi_check_trigger_arg_min(&mut (*cmd).convert_arg, 12500); }
    err |= comedi_check_trigger_arg_is(&mut (*cmd).scan_end_arg, (*cmd).chanlist_len); if (*cmd).stop_src == TRIG_COUNT { err |= comedi_check_trigger_arg_min(&mut (*cmd).stop_arg, 1); } else { err |= comedi_check_trigger_arg_is(&mut (*cmd).stop_arg, 0); } if err != 0 { return 3; }
    if (*cmd).convert_src == TRIG_TIMER { arg = (*cmd).convert_arg; (*devpriv).ai_divisor = dt2811_ns_to_timer(&mut arg, (*cmd).flags); err |= comedi_check_trigger_arg_is(&mut (*cmd).convert_arg, arg); } else { (*devpriv).ai_divisor = (*cmd).convert_arg; } if err != 0 { return 4; } 0
}

unsafe fn dt2811_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> c_int {
    let board = (*dev).board_ptr as *const dt2811_board; let p = comedi_alloc_devpriv(dev, core::mem::size_of::<dt2811_private>()) as *mut dt2811_private; if p.is_null() { return -ENOMEM; }
    let mut ret = comedi_check_request_region(dev, (*it).options[0], 8, 0x200, 0x3ff, 8); if ret != 0 { return ret; } dt2811_reset(dev);
    if (*it).options[1] <= 7 && ((1u32 << (*it).options[1]) & 0xac) != 0 { ret = request_irq((*it).options[1], Some(dt2811_interrupt), 0, (*dev).board_name, dev); if ret == 0 { (*dev).irq = (*it).options[1]; } }
    ret = comedi_alloc_subdevices(dev, 4); if ret != 0 { return ret; }
    let s = (*dev).subdevices; (*s.add(0)).type_ = COMEDI_SUBD_AI; (*s.add(0)).subdev_flags = SDF_READABLE | if (*it).options[2] == 1 { SDF_DIFF } else if (*it).options[2] == 2 { SDF_COMMON } else { SDF_GROUND }; (*s.add(0)).n_chan = if (*it).options[2] == 1 { 8 } else { 16 }; (*s.add(0)).maxdata = 0xfff; (*s.add(0)).range_table = if (*board).is_pgh != 0 { &DT2811_PGH_AI_RANGES } else { &DT2811_PGL_AI_RANGES }; (*s.add(0)).insn_read = Some(dt2811_ai_insn_read);
    if (*dev).irq != 0 { (*dev).read_subdev = s; (*s.add(0)).subdev_flags |= SDF_CMD_READ; (*s.add(0)).len_chanlist = 1; (*s.add(0)).do_cmdtest = Some(dt2811_ai_cmdtest); (*s.add(0)).do_cmd = Some(dt2811_ai_cmd); (*s.add(0)).cancel = Some(dt2811_ai_cancel); }
    (*s.add(1)).type_ = COMEDI_SUBD_AO; (*s.add(1)).subdev_flags = SDF_WRITABLE; (*s.add(1)).n_chan = 2; (*s.add(1)).maxdata = 0xfff; (*s.add(1)).range_table = &DT2811_AO_RANGES; (*s.add(1)).insn_write = Some(dt2811_ao_insn_write); ret = comedi_alloc_subdev_readback(s.add(1)); if ret != 0 { return ret; }
    (*s.add(2)).type_ = COMEDI_SUBD_DI; (*s.add(2)).subdev_flags = SDF_READABLE; (*s.add(2)).n_chan = 8; (*s.add(2)).maxdata = 1; (*s.add(2)).range_table = &range_digital; (*s.add(2)).insn_bits = Some(dt2811_di_insn_bits);
    (*s.add(3)).type_ = COMEDI_SUBD_DO; (*s.add(3)).subdev_flags = SDF_WRITABLE; (*s.add(3)).n_chan = 8; (*s.add(3)).maxdata = 1; (*s.add(3)).range_table = &range_digital; (*s.add(3)).insn_bits = Some(dt2811_do_insn_bits); 0
}

static mut DT2811_DRIVER: comedi_driver = comedi_driver { driver_name: c"dt2811".as_ptr(), module: THIS_MODULE, attach: Some(dt2811_attach), detach: Some(comedi_legacy_detach), board_name: DT2811_BOARDS.as_ptr() as *const *const c_char, num_names: DT2811_BOARDS.len(), offset: core::mem::size_of::<dt2811_board>() };
module_comedi_driver!(DT2811_DRIVER);
module_author!("Comedi https://www.comedi.org");
module_description!("Comedi driver for Data Translation DT2811 series boards");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
