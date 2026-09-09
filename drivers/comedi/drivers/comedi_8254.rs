// SPDX-License-Identifier: GPL-2.0+
/*
 * comedi_8254.c
 * Generic 8254 timer/counter support
 * Copyright (C) 2014 H Hartley Sweeten <hsweeten@visionengravers.com>
 *
 * Based on 8253.h and various subdevice implementations in comedi drivers.
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 2000 David A. Schleef <ds@schleef.org>
 */

// C headers and build-time CONFIG_HAS_IOPORT condition are supplied externally.

#[cfg(CONFIG_HAS_IOPORT)]
unsafe fn i8254_io8_cb(i8254: *mut comedi_8254, dir: i32, reg: u32, val: u32) -> u32 {
    let iobase = (*i8254).context;
    let reg_offset = (reg * I8254_IO8) << (*i8254).regshift;
    if dir != 0 { outb(val, iobase + reg_offset as u64); 0 } else { inb(iobase + reg_offset as u64) }
}

#[cfg(CONFIG_HAS_IOPORT)]
unsafe fn i8254_io16_cb(i8254: *mut comedi_8254, dir: i32, reg: u32, val: u32) -> u32 {
    let iobase = (*i8254).context;
    let reg_offset = (reg * I8254_IO16) << (*i8254).regshift;
    if dir != 0 { outw(val, iobase + reg_offset as u64); 0 } else { inw(iobase + reg_offset as u64) }
}

#[cfg(CONFIG_HAS_IOPORT)]
unsafe fn i8254_io32_cb(i8254: *mut comedi_8254, dir: i32, reg: u32, val: u32) -> u32 {
    let iobase = (*i8254).context;
    let reg_offset = (reg * I8254_IO32) << (*i8254).regshift;
    if dir != 0 { outl(val, iobase + reg_offset as u64); 0 } else { inl(iobase + reg_offset as u64) }
}

unsafe fn i8254_mmio8_cb(i8254: *mut comedi_8254, dir: i32, reg: u32, val: u32) -> u32 {
    let mmiobase = (*i8254).context as *mut u8;
    let reg_offset = (reg * I8254_IO8) << (*i8254).regshift;
    if dir != 0 { writeb(val, mmiobase.add(reg_offset as usize)); 0 } else { readb(mmiobase.add(reg_offset as usize)) }
}

unsafe fn i8254_mmio16_cb(i8254: *mut comedi_8254, dir: i32, reg: u32, val: u32) -> u32 {
    let mmiobase = (*i8254).context as *mut u8;
    let reg_offset = (reg * I8254_IO16) << (*i8254).regshift;
    if dir != 0 { writew(val, mmiobase.add(reg_offset as usize)); 0 } else { readw(mmiobase.add(reg_offset as usize)) }
}

unsafe fn i8254_mmio32_cb(i8254: *mut comedi_8254, dir: i32, reg: u32, val: u32) -> u32 {
    let mmiobase = (*i8254).context as *mut u8;
    let reg_offset = (reg * I8254_IO32) << (*i8254).regshift;
    if dir != 0 { writel(val, mmiobase.add(reg_offset as usize)); 0 } else { readl(mmiobase.add(reg_offset as usize)) }
}

unsafe fn __i8254_read(i8254: *mut comedi_8254, reg: u32) -> u32 { 0xff & ((*i8254).iocb)(i8254, 0, reg, 0) }
unsafe fn __i8254_write(i8254: *mut comedi_8254, val: u32, reg: u32) { ((*i8254).iocb)(i8254, 1, reg, val); }

pub unsafe fn comedi_8254_status(i8254: *mut comedi_8254, counter: u32) -> u32 {
    if counter > 2 { return 0; }
    let cmd = I8254_CTRL_READBACK_STATUS | I8254_CTRL_READBACK_SEL_CTR(counter);
    __i8254_write(i8254, cmd, I8254_CTRL_REG);
    __i8254_read(i8254, counter)
}

pub unsafe fn comedi_8254_read(i8254: *mut comedi_8254, counter: u32) -> u32 {
    if counter > 2 { return 0; }
    __i8254_write(i8254, I8254_CTRL_SEL_CTR(counter) | I8254_CTRL_LATCH, I8254_CTRL_REG);
    let mut val = __i8254_read(i8254, counter);
    val |= __i8254_read(i8254, counter) << 8;
    val
}

pub unsafe fn comedi_8254_write(i8254: *mut comedi_8254, counter: u32, val: u32) {
    if counter > 2 || val > 0xffff { return; }
    __i8254_write(i8254, val & 0xff, counter);
    __i8254_write(i8254, (val >> 8) & 0xff, counter);
}

pub unsafe fn comedi_8254_set_mode(i8254: *mut comedi_8254, counter: u32, mode: u32) -> i32 {
    if counter > 2 || mode > (I8254_MODE5 | I8254_BCD) { return -EINVAL; }
    let byte = I8254_CTRL_SEL_CTR(counter) | I8254_CTRL_LSB_MSB | mode;
    __i8254_write(i8254, byte, I8254_CTRL_REG);
    0
}

pub unsafe fn comedi_8254_load(i8254: *mut comedi_8254, counter: u32, val: u32, mode: u32) -> i32 {
    if counter > 2 || val > 0xffff || mode > (I8254_MODE5 | I8254_BCD) { return -EINVAL; }
    comedi_8254_set_mode(i8254, counter, mode);
    comedi_8254_write(i8254, counter, val);
    0
}

pub unsafe fn comedi_8254_pacer_enable(i8254: *mut comedi_8254, counter1: u32, counter2: u32, enable: bool) {
    if counter1 > 2 || counter2 > 2 || counter1 == counter2 { return; }
    let mode = if enable { I8254_MODE2 | I8254_BINARY } else { I8254_MODE0 | I8254_BINARY };
    comedi_8254_set_mode(i8254, counter1, mode);
    comedi_8254_set_mode(i8254, counter2, mode);
    if enable {
        // Divisors are loaded second counter then first counter to avoid possible issues.
        comedi_8254_write(i8254, counter2, (*i8254).divisor2);
        comedi_8254_write(i8254, counter1, (*i8254).divisor1);
    }
}

pub unsafe fn comedi_8254_update_divisors(i8254: *mut comedi_8254) {
    // masking is done since counter maps zero to 0x10000
    (*i8254).divisor = (*i8254).next_div & 0xffff;
    (*i8254).divisor1 = (*i8254).next_div1 & 0xffff;
    (*i8254).divisor2 = (*i8254).next_div2 & 0xffff;
}

pub unsafe fn comedi_8254_cascade_ns_to_timer(i8254: *mut comedi_8254, nanosec: *mut u32, flags: u32) {
    let mut d1 = if (*i8254).next_div1 != 0 { (*i8254).next_div1 } else { I8254_MAX_COUNT };
    let mut d2 = if (*i8254).next_div2 != 0 { (*i8254).next_div2 } else { I8254_MAX_COUNT };
    let mut div = d1 * d2;
    let mut ns_lub = 0xffffffff; let mut ns_glb = 0; let mut d1_lub = 0; let mut d1_glb = 0; let mut d2_lub = 0; let mut d2_glb = 0;
    let mut start; let mut ns; let mut ns_low; let mut ns_high;
    if div * (*i8254).osc_base == *nanosec && d1 > 1 && d1 <= I8254_MAX_COUNT && d2 > 1 && d2 <= I8254_MAX_COUNT && div > d1 && div > d2 && div * (*i8254).osc_base > div && div * (*i8254).osc_base > (*i8254).osc_base { return; }
    div = *nanosec / (*i8254).osc_base; d2 = I8254_MAX_COUNT; start = div / d2; if start < 2 { start = 2; }
    d1 = start;
    while d1 <= div / d1 + 1 && d1 <= I8254_MAX_COUNT {
        d2 = div / d1;
        while d1 * d2 <= div + d1 + 1 && d2 <= I8254_MAX_COUNT {
            ns = (*i8254).osc_base * d1 * d2;
            if ns <= *nanosec && ns > ns_glb { ns_glb = ns; d1_glb = d1; d2_glb = d2; }
            if ns >= *nanosec && ns < ns_lub { ns_lub = ns; d1_lub = d1; d2_lub = d2; }
            d2 += 1;
        }
        d1 += 1;
    }
    match flags & CMDF_ROUND_MASK {
        CMDF_ROUND_UP => { d1 = d1_lub; d2 = d2_lub; }
        CMDF_ROUND_DOWN => { d1 = d1_glb; d2 = d2_glb; }
        _ => { ns_high = d1_lub * d2_lub * (*i8254).osc_base; ns_low = d1_glb * d2_glb * (*i8254).osc_base; if ns_high - *nanosec < *nanosec - ns_low { d1 = d1_lub; d2 = d2_lub; } else { d1 = d1_glb; d2 = d2_glb; } }
    }
    *nanosec = d1 * d2 * (*i8254).osc_base; (*i8254).next_div1 = d1; (*i8254).next_div2 = d2;
}

pub unsafe fn comedi_8254_ns_to_timer(i8254: *mut comedi_8254, nanosec: *mut u32, flags: u32) {
    let mut divisor = match flags & CMDF_ROUND_MASK { CMDF_ROUND_UP => DIV_ROUND_UP(*nanosec, (*i8254).osc_base), CMDF_ROUND_DOWN => *nanosec / (*i8254).osc_base, _ => DIV_ROUND_CLOSEST(*nanosec, (*i8254).osc_base) };
    if divisor < 2 { divisor = 2; } if divisor > I8254_MAX_COUNT { divisor = I8254_MAX_COUNT; }
    *nanosec = divisor * (*i8254).osc_base; (*i8254).next_div = divisor;
}

pub unsafe fn comedi_8254_set_busy(i8254: *mut comedi_8254, counter: u32, busy: bool) { if counter < 3 { (*i8254).busy[counter as usize] = busy; } }

unsafe fn comedi_8254_insn_read(_dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let i8254 = (*s).private as *mut comedi_8254; let chan = CR_CHAN((*insn).chanspec);
    if (*i8254).busy[chan as usize] { return -EBUSY; }
    for i in 0..(*insn).n { *data.add(i as usize) = comedi_8254_read(i8254, chan); } (*insn).n as i32
}

unsafe fn comedi_8254_insn_write(_dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let i8254 = (*s).private as *mut comedi_8254; let chan = CR_CHAN((*insn).chanspec);
    if (*i8254).busy[chan as usize] { return -EBUSY; }
    if (*insn).n != 0 { comedi_8254_write(i8254, chan, *data.add((*insn).n as usize - 1)); } (*insn).n as i32
}

unsafe fn comedi_8254_insn_config(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let i8254 = (*s).private as *mut comedi_8254; let chan = CR_CHAN((*insn).chanspec); if (*i8254).busy[chan as usize] { return -EBUSY; }
    let ret = match *data { INSN_CONFIG_RESET => comedi_8254_set_mode(i8254, chan, I8254_MODE0 | I8254_BINARY), INSN_CONFIG_SET_COUNTER_MODE => comedi_8254_set_mode(i8254, chan, *data.add(1)), INSN_CONFIG_8254_READ_STATUS => { *data.add(1) = comedi_8254_status(i8254, chan); 0 }, _ => { if let Some(f) = (*i8254).insn_config { return f(dev, s, insn, data); } return -EINVAL; } }; if ret != 0 { return ret; } (*insn).n as i32
}

pub unsafe fn comedi_8254_subdevice_init(s: *mut comedi_subdevice, i8254: *mut comedi_8254) {
    (*s).type_ = COMEDI_SUBD_COUNTER; (*s).subdev_flags = SDF_READABLE | SDF_WRITABLE; (*s).n_chan = 3; (*s).maxdata = 0xffff; (*s).range_table = &range_unknown; (*s).insn_read = Some(comedi_8254_insn_read); (*s).insn_write = Some(comedi_8254_insn_write); (*s).insn_config = Some(comedi_8254_insn_config); (*s).private = i8254 as *mut _;
}

unsafe fn __i8254_init(iocb: comedi_8254_iocb_fn, context: u64, osc_base: u32, iosize: u32, regshift: u32) -> *mut comedi_8254 {
    if iosize != I8254_IO8 && iosize != I8254_IO16 && iosize != I8254_IO32 || iocb.is_none() { return ERR_PTR(-EINVAL); }
    let i8254 = kzalloc_obj::<comedi_8254>(); if i8254.is_null() { return ERR_PTR(-ENOMEM); }
    (*i8254).iocb = iocb; (*i8254).context = context; (*i8254).iosize = iosize; (*i8254).regshift = regshift; (*i8254).osc_base = if osc_base != 0 { osc_base } else { I8254_OSC_BASE_10MHZ };
    for i in 0..3 { comedi_8254_set_mode(i8254, i, I8254_MODE0 | I8254_BINARY); } i8254
}

#[cfg(CONFIG_HAS_IOPORT)]
pub unsafe fn comedi_8254_io_alloc(iobase: u64, osc_base: u32, iosize: u32, regshift: u32) -> *mut comedi_8254 {
    let iocb = match iosize { I8254_IO8 => Some(i8254_io8_cb), I8254_IO16 => Some(i8254_io16_cb), I8254_IO32 => Some(i8254_io32_cb), _ => return ERR_PTR(-EINVAL) }; __i8254_init(iocb, iobase, osc_base, iosize, regshift)
}

pub unsafe fn comedi_8254_mm_alloc(mmio: *mut u8, osc_base: u32, iosize: u32, regshift: u32) -> *mut comedi_8254 {
    let iocb = match iosize { I8254_IO8 => Some(i8254_mmio8_cb), I8254_IO16 => Some(i8254_mmio16_cb), I8254_IO32 => Some(i8254_mmio32_cb), _ => return ERR_PTR(-EINVAL) }; __i8254_init(iocb, mmio as u64, osc_base, iosize, regshift)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
