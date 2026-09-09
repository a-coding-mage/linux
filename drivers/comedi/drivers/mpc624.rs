// SPDX-License-Identifier: GPL-2.0+
/*
 * mpc624.c
 * Hardware driver for a Micro/sys inc. MPC-624 PC/104 board
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 2000 David A. Schleef <ds@schleef.org>
 */

/*
 * Driver: mpc624
 * Description: Micro/sys MPC-624 PC/104 board
 * Devices: [Micro/sys] MPC-624 (mpc624)
 * Author: Stanislaw Raczynski <sraczynski@op.pl>
 * Updated: Thu, 15 Sep 2005 12:01:18 +0200
 * Status: working
 *
 * The Micro/sys MPC-624 board is based on the LTC2440 24-bit sigma-delta
 * ADC chip.
 *
 * Subdevices supported by the driver:
 * - Analog In:   supported
 * - Digital I/O: not supported
 * - LEDs:        not supported
 * - EEPROM:      not supported
 *
 * Configuration Options:
 *   [0] - I/O base address
 *   [1] - conversion rate
 *   [2] - voltage range
 */

const MPC624_MASTER_CONTROL: u8 = 0;
const MPC624_GNMUXCH: u8 = 1;
const MPC624_ADC: u8 = 2;
const MPC624_EE: u8 = 3;
const MPC624_LEDS: u8 = 4;
const MPC624_DIO: u8 = 5;
const MPC624_IRQ_MASK: u8 = 6;

const MPC624_ADBUSY: u8 = 1 << 5;
const MPC624_ADSDO: u8 = 1 << 4;
const MPC624_ADFO: u8 = 1 << 3;
const MPC624_ADCS: u8 = 1 << 2;
const MPC624_ADSCK: u8 = 1 << 1;
const MPC624_ADSDI: u8 = 1;

const MPC624_EOC_BIT: u32 = 1 << 31;
const MPC624_DMY_BIT: u32 = 1 << 30;
const MPC624_SGN_BIT: u32 = 1 << 29;

const fn mpc624_osr(x: u32) -> u32 { (x & 0x1f) << 27 }
const MPC624_SPEED_3_52_KHZ: u32 = mpc624_osr(0x11);
const MPC624_SPEED_1_76_KHZ: u32 = mpc624_osr(0x12);
const MPC624_SPEED_880_HZ: u32 = mpc624_osr(0x13);
const MPC624_SPEED_440_HZ: u32 = mpc624_osr(0x14);
const MPC624_SPEED_220_HZ: u32 = mpc624_osr(0x15);
const MPC624_SPEED_110_HZ: u32 = mpc624_osr(0x16);
const MPC624_SPEED_55_HZ: u32 = mpc624_osr(0x17);
const MPC624_SPEED_27_5_HZ: u32 = mpc624_osr(0x18);
const MPC624_SPEED_13_75_HZ: u32 = mpc624_osr(0x19);
const MPC624_SPEED_6_875_HZ: u32 = mpc624_osr(0x1f);

#[repr(C)]
struct mpc624_private { ai_speed: u32 }

static range_mpc624_bipolar1: comedi_lrange = comedi_lrange { length: 1, range: [BIP_RANGE(2.02)] };
static range_mpc624_bipolar10: comedi_lrange = comedi_lrange { length: 1, range: [BIP_RANGE(20.2)] };

unsafe fn mpc624_ai_get_sample(dev: *mut comedi_device, _s: *mut comedi_subdevice) -> u32 {
    let devpriv = (*dev).private as *mut mpc624_private;
    let mut data_out = (*devpriv).ai_speed;
    let mut data_in: u32 = 0;
    udelay(1);
    for _i in 0..32 {
        outb(0, (*dev).iobase + MPC624_ADC as u16); udelay(1);
        let bit = if (data_out & (1 << 31)) != 0 { MPC624_ADSDI } else { 0 };
        outb(bit, (*dev).iobase + MPC624_ADC as u16); udelay(1);
        outb(MPC624_ADSCK | bit, (*dev).iobase + MPC624_ADC as u16); udelay(1);
        data_in <<= 1;
        data_in |= ((inb((*dev).iobase + MPC624_ADC as u16) & MPC624_ADSDO) >> 4) as u32;
        udelay(1); data_out <<= 1;
    }
    if data_in & MPC624_EOC_BIT != 0 { dev_dbg((*dev).class_dev, "EOC bit is set!"); }
    if data_in & MPC624_DMY_BIT != 0 { dev_dbg((*dev).class_dev, "DMY bit is set!"); }
    if data_in & MPC624_SGN_BIT != 0 { data_in &= 0x3fffffff; }
    else { data_in |= MPC624_SGN_BIT; data_in = !data_in; data_in = data_in.wrapping_add(1); data_in &= !(MPC624_EOC_BIT | MPC624_DMY_BIT); data_in = 0x20000000u32.wrapping_sub(data_in); }
    data_in
}

unsafe fn mpc624_ai_eoc(dev: *mut comedi_device, _s: *mut comedi_subdevice, _insn: *mut comedi_insn, _context: u64) -> i32 {
    let status = inb((*dev).iobase + MPC624_ADC as u16);
    if status & MPC624_ADBUSY == 0 { 0 } else { -EBUSY }
}

unsafe fn mpc624_ai_insn_read(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    outb((*insn).chanspec as u8, (*dev).iobase + MPC624_GNMUXCH as u16);
    for i in 0..(*insn).n {
        outb(MPC624_ADSCK, (*dev).iobase + MPC624_ADC as u16); udelay(1);
        outb(MPC624_ADCS | MPC624_ADSCK, (*dev).iobase + MPC624_ADC as u16); udelay(1);
        outb(0, (*dev).iobase + MPC624_ADC as u16); udelay(1);
        let ret = comedi_timeout(dev, s, insn, Some(mpc624_ai_eoc), 0);
        if ret != 0 { return ret; }
        *data.add(i as usize) = mpc624_ai_get_sample(dev, s);
    }
    (*insn).n as i32
}

unsafe fn mpc624_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> i32 {
    let ret = comedi_check_request_region(dev, (*it).options[0], 0x10, 0, 0x3ff, 16);
    if ret != 0 { return ret; }
    let devpriv = comedi_alloc_devpriv(dev, core::mem::size_of::<mpc624_private>()) as *mut mpc624_private;
    if devpriv.is_null() { return -ENOMEM; }
    (*devpriv).ai_speed = match (*it).options[1] { 0=>MPC624_SPEED_3_52_KHZ,1=>MPC624_SPEED_1_76_KHZ,2=>MPC624_SPEED_880_HZ,3=>MPC624_SPEED_440_HZ,4=>MPC624_SPEED_220_HZ,5=>MPC624_SPEED_110_HZ,6=>MPC624_SPEED_55_HZ,7=>MPC624_SPEED_27_5_HZ,8=>MPC624_SPEED_13_75_HZ,9=>MPC624_SPEED_6_875_HZ,_=>MPC624_SPEED_3_52_KHZ };
    let ret = comedi_alloc_subdevices(dev, 1); if ret != 0 { return ret; }
    let s = (*dev).subdevices;
    (*s).type_ = COMEDI_SUBD_AI; (*s).subdev_flags = SDF_READABLE | SDF_DIFF; (*s).n_chan = 4; (*s).maxdata = 0x3fffffff;
    (*s).range_table = if (*it).options[1] == 0 { &range_mpc624_bipolar1 } else { &range_mpc624_bipolar10 };
    (*s).insn_read = Some(mpc624_ai_insn_read);
    0
}

static mut mpc624_driver: comedi_driver = comedi_driver { driver_name: "mpc624", module: THIS_MODULE, attach: Some(mpc624_attach), detach: Some(comedi_legacy_detach) };
module_comedi_driver!(mpc624_driver);
MODULE_AUTHOR!("Comedi https://www.comedi.org");
MODULE_DESCRIPTION!("Comedi driver for Micro/sys MPC-624 PC/104 board");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
