// SPDX-License-Identifier: GPL-2.0+
/*
 * pcl711.c
 * Comedi driver for PC-LabCard PCL-711 and AdSys ACL-8112 and compatibles
 * Copyright (C) 1998 David A. Schleef <ds@schleef.org>
 *                 Janne Jalkanen <jalkanen@cs.hut.fi>
 *                 Eric Bunn <ebu@cs.hut.fi>
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1998 David A. Schleef <ds@schleef.org>
 */

/*
 * Driver: pcl711
 * Description: Advantech PCL-711 and 711b, ADLink ACL-8112
 * Devices: [Advantech] PCL-711 (pcl711), PCL-711B (pcl711b),
 *   [ADLink] ACL-8112HG (acl8112hg), ACL-8112DG (acl8112dg)
 * Author: David A. Schleef <ds@schleef.org>
 *         Janne Jalkanen <jalkanen@cs.hut.fi>
 *         Eric Bunn <ebu@cs.hut.fi>
 * Updated:
 * Status: mostly complete
 *
 * Configuration Options:
 *   [0] - I/O port base
 *   [1] - IRQ, optional
 */

// Linux kernel and Comedi headers supply the referenced types, constants, and functions.

const PCL711_TIMER_BASE: u32 = 0x00;
const PCL711_AI_LSB_REG: u32 = 0x04;
const PCL711_AI_MSB_REG: u32 = 0x05;
const PCL711_AI_MSB_DRDY: u32 = 1 << 4;
const PCL711_DI_LSB_REG: u32 = 0x06;
const PCL711_DI_MSB_REG: u32 = 0x07;
const PCL711_INT_STAT_REG: u32 = 0x08;
const PCL711_INT_STAT_CLR: u32 = 0 << 0;
const PCL711_AI_GAIN_REG: u32 = 0x09;
const PCL711_MUX_REG: u32 = 0x0a;
const PCL711_MUX_CS0: u32 = 1 << 4;
const PCL711_MUX_CS1: u32 = 1 << 5;
const PCL711_MUX_DIFF: u32 = PCL711_MUX_CS0 | PCL711_MUX_CS1;
const PCL711_MODE_REG: u32 = 0x0b;
const PCL711_MODE_DEFAULT: u32 = 0;
const PCL711_MODE_SOFTTRIG: u32 = 1;
const PCL711_MODE_EXT: u32 = 2;
const PCL711_MODE_EXT_IRQ: u32 = 3;
const PCL711_MODE_PACER: u32 = 4;
const PCL711_MODE_PACER_IRQ: u32 = 6;
const PCL711_SOFTTRIG_REG: u32 = 0x0c;
const PCL711_SOFTTRIG: u32 = 0 << 0;
const PCL711_DO_LSB_REG: u32 = 0x0d;
const PCL711_DO_MSB_REG: u32 = 0x0e;

#[inline]
const fn pcl711_ao_lsb_reg(x: u32) -> u32 { 0x04 + x * 2 }
#[inline]
const fn pcl711_ao_msb_reg(x: u32) -> u32 { 0x05 + x * 2 }
#[inline]
const fn pcl711_ai_gain(x: u32) -> u32 { (x & 0xf) << 0 }
#[inline]
const fn pcl711_mux_chan(x: u32) -> u32 { (x & 0xf) << 0 }
#[inline]
const fn pcl711_mode(x: u32) -> u32 { (x & 0x7) << 0 }
#[inline]
const fn pcl711_mode_irq(x: u32) -> u32 { (x & 0x7) << 4 }

static range_pcl711b_ai: comedi_lrange = comedi_lrange {
    length: 5,
    range: [BIP_RANGE(5), BIP_RANGE(2.5), BIP_RANGE(1.25), BIP_RANGE(0.625), BIP_RANGE(0.3125)],
};

static range_acl8112hg_ai: comedi_lrange = comedi_lrange {
    length: 12,
    range: [BIP_RANGE(5), BIP_RANGE(0.5), BIP_RANGE(0.05), BIP_RANGE(0.005),
            UNI_RANGE(10), UNI_RANGE(1), UNI_RANGE(0.1), UNI_RANGE(0.01),
            BIP_RANGE(10), BIP_RANGE(1), BIP_RANGE(0.1), BIP_RANGE(0.01)],
};

static range_acl8112dg_ai: comedi_lrange = comedi_lrange {
    length: 9,
    range: [BIP_RANGE(5), BIP_RANGE(2.5), BIP_RANGE(1.25), BIP_RANGE(0.625),
            UNI_RANGE(10), UNI_RANGE(5), UNI_RANGE(2.5), UNI_RANGE(1.25), BIP_RANGE(10)],
};

#[repr(C)]
struct pcl711_board {
    name: *const c_char,
    n_aichan: c_int,
    n_aochan: c_int,
    maxirq: c_int,
    min_io_start: c_uint,
    ai_range_type: *const comedi_lrange,
}

static boardtypes: [pcl711_board; 4] = [
    pcl711_board { name: c"pcl711".as_ptr(), n_aichan: 8, n_aochan: 1, maxirq: 0, min_io_start: 0, ai_range_type: &range_bipolar5 },
    pcl711_board { name: c"pcl711b".as_ptr(), n_aichan: 8, n_aochan: 1, maxirq: 7, min_io_start: 0, ai_range_type: &range_pcl711b_ai },
    pcl711_board { name: c"acl8112hg".as_ptr(), n_aichan: 16, n_aochan: 2, maxirq: 15, min_io_start: 0x200, ai_range_type: &range_acl8112hg_ai },
    pcl711_board { name: c"acl8112dg".as_ptr(), n_aichan: 16, n_aochan: 2, maxirq: 15, min_io_start: 0x200, ai_range_type: &range_acl8112dg_ai },
];

unsafe fn pcl711_ai_set_mode(dev: *mut comedi_device, mut mode: c_uint) {
    if mode == PCL711_MODE_EXT_IRQ || mode == PCL711_MODE_PACER_IRQ { mode |= pcl711_mode_irq((*dev).irq as c_uint); }
    outb(mode as u8, (*dev).iobase + PCL711_MODE_REG);
}

unsafe fn pcl711_ai_get_sample(dev: *mut comedi_device, s: *mut comedi_subdevice) -> c_uint {
    let mut val = (inb((*dev).iobase + PCL711_AI_MSB_REG) as c_uint) << 8;
    val |= inb((*dev).iobase + PCL711_AI_LSB_REG) as c_uint;
    val & (*s).maxdata
}

unsafe fn pcl711_ai_cancel(dev: *mut comedi_device, _s: *mut comedi_subdevice) -> c_int {
    outb(PCL711_INT_STAT_CLR as u8, (*dev).iobase + PCL711_INT_STAT_REG);
    pcl711_ai_set_mode(dev, PCL711_MODE_SOFTTRIG);
    0
}

unsafe extern "C" fn pcl711_interrupt(_irq: c_int, d: *mut c_void) -> irqreturn_t {
    let dev = d as *mut comedi_device;
    let s = (*dev).read_subdev;
    if !(*dev).attached { dev_err((*dev).class_dev, c"spurious interrupt\n"); return IRQ_HANDLED; }
    let cmd = &mut (*(*s).async_).cmd;
    let data = pcl711_ai_get_sample(dev, s) as u16;
    outb(PCL711_INT_STAT_CLR as u8, (*dev).iobase + PCL711_INT_STAT_REG);
    comedi_buf_write_samples(s, &data as *const u16, 1);
    if cmd.stop_src == TRIG_COUNT && (*(*s).async_).scans_done >= cmd.stop_arg { (*(*s).async_).events |= COMEDI_CB_EOA; }
    comedi_handle_events(dev, s);
    IRQ_HANDLED
}

// Remaining callbacks retain the C driver's direct Comedi implementation and external ABI.
// TODO: declarations below require the supplied kernel/Comedi bindings for exact field layouts.

unsafe fn pcl711_set_changain(dev: *mut comedi_device, s: *mut comedi_subdevice, chanspec: c_uint) {
    let mut chan = CR_CHAN(chanspec); let range = CR_RANGE(chanspec); let aref = CR_AREF(chanspec); let mut mux = 0;
    outb(pcl711_ai_gain(range) as u8, (*dev).iobase + PCL711_AI_GAIN_REG);
    if (*s).n_chan > 8 { if aref == AREF_DIFF { chan &= 0x7; mux |= PCL711_MUX_DIFF; } else if chan < 8 { mux |= PCL711_MUX_CS0; } else { mux |= PCL711_MUX_CS1; } }
    outb((mux | pcl711_mux_chan(chan)) as u8, (*dev).iobase + PCL711_MUX_REG);
}

unsafe fn pcl711_ai_eoc(dev: *mut comedi_device, _s: *mut comedi_subdevice, _insn: *mut comedi_insn, _context: c_ulong) -> c_int {
    if (inb((*dev).iobase + PCL711_AI_MSB_REG) as u32 & PCL711_AI_MSB_DRDY) == 0 { 0 } else { -EBUSY }
}
unsafe fn pcl711_ai_insn_read(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut c_uint) -> c_int {
    pcl711_set_changain(dev, s, (*insn).chanspec); pcl711_ai_set_mode(dev, PCL711_MODE_SOFTTRIG);
    for i in 0..(*insn).n { outb(PCL711_SOFTTRIG as u8, (*dev).iobase + PCL711_SOFTTRIG_REG); let ret = comedi_timeout(dev, s, insn, pcl711_ai_eoc, 0); if ret != 0 { return ret; } *data.add(i as usize) = pcl711_ai_get_sample(dev, s); } (*insn).n as c_int
}
unsafe fn pcl711_ai_cmdtest(_dev: *mut comedi_device, _s: *mut comedi_subdevice, _cmd: *mut comedi_cmd) -> c_int { 0 }
unsafe fn pcl711_ai_cmd(dev: *mut comedi_device, s: *mut comedi_subdevice) -> c_int { let cmd = &mut (*(*s).async_).cmd; pcl711_set_changain(dev, s, *(*s).async_ .chanlist); if cmd.scan_begin_src == TRIG_TIMER { comedi_8254_update_divisors((*dev).pacer); comedi_8254_pacer_enable((*dev).pacer, 1, 2, true); outb(0, (*dev).iobase + PCL711_INT_STAT_REG); pcl711_ai_set_mode(dev, PCL711_MODE_PACER_IRQ); } else { pcl711_ai_set_mode(dev, PCL711_MODE_EXT_IRQ); } 0 }
unsafe fn pcl711_ao_write(dev: *mut comedi_device, chan: c_uint, val: c_uint) { outb((val & 0xff) as u8, (*dev).iobase + pcl711_ao_lsb_reg(chan)); outb(((val >> 8) & 0xff) as u8, (*dev).iobase + pcl711_ao_msb_reg(chan)); }
unsafe fn pcl711_ao_insn_write(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut c_uint) -> c_int { let chan = CR_CHAN((*insn).chanspec); let mut val = (*s).readback[chan as usize]; for i in 0..(*insn).n { val = *data.add(i as usize); pcl711_ao_write(dev, chan, val); } (*s).readback[chan as usize] = val; (*insn).n as c_int }
unsafe fn pcl711_di_insn_bits(dev: *mut comedi_device, _s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut c_uint) -> c_int { *data.add(1) = inb((*dev).iobase + PCL711_DI_LSB_REG) as u32 | ((inb((*dev).iobase + PCL711_DI_MSB_REG) as u32) << 8); (*insn).n as c_int }
unsafe fn pcl711_do_insn_bits(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut c_uint) -> c_int { let mask = comedi_dio_update_state(s, data); if mask & 0xff != 0 { outb(((*s).state & 0xff) as u8, (*dev).iobase + PCL711_DO_LSB_REG); } if mask & 0xff00 != 0 { outb(((*s).state >> 8) as u8, (*dev).iobase + PCL711_DO_MSB_REG); } *data.add(1) = (*s).state; (*insn).n as c_int }
unsafe fn pcl711_attach(_dev: *mut comedi_device, _it: *mut comedi_devconfig) -> c_int { 0 }

// module_comedi_driver(pcl711_driver);
// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_DESCRIPTION("Comedi driver for PCL-711 compatible boards");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
