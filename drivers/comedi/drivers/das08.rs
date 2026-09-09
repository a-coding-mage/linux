// SPDX-License-Identifier: GPL-2.0+
/*
 * comedi/drivers/das08.c
 * comedi module for common DAS08 support (used by ISA/PCI/PCMCIA drivers)
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 2000 David A. Schleef <ds@schleef.org>
 * Copyright (C) 2001,2002,2003 Frank Mori Hess <fmhess@users.sourceforge.net>
 * Copyright (C) 2004 Salvador E. Tropea <set@users.sf.net> <set@ieee.org>
 */

// Kernel and DAS08 header dependencies are supplied by the surrounding tree.

const DAS08_AI_LSB_REG: u32 = 0x00;
const DAS08_AI_MSB_REG: u32 = 0x01;
const DAS08_AI_TRIG_REG: u32 = 0x01;
const DAS08_STATUS_REG: u32 = 0x02;
const DAS08_STATUS_AI_BUSY: u32 = 1 << 7;
const DAS08_STATUS_IRQ: u32 = 1 << 3;
const DAS08_CONTROL_REG: u32 = 0x02;
const DAS08_CONTROL_MUX_MASK: u32 = 0x7;
const DAS08_CONTROL_INTE: u32 = 1 << 3;
const DAS08_CONTROL_DO_MASK: u32 = 0xf0;
const DAS08_GAIN_REG: u32 = 0x03;
const DAS08JR_DI_REG: u32 = 0x03;
const DAS08JR_DO_REG: u32 = 0x03;
const DAS08JR_AO_UPDATE_REG: u32 = 0x03;
const DAS08AOX_AO_UPDATE_REG: u32 = 0x08;

#[inline]
const fn das08_status_di(x: u32) -> u32 { (x & 0x70) >> 4 }
#[inline]
const fn das08_control_mux(x: u32) -> u32 { x & DAS08_CONTROL_MUX_MASK }
#[inline]
const fn das08_control_do(x: u32) -> u32 { (x << 4) & DAS08_CONTROL_DO_MASK }
#[inline]
const fn das08jr_ao_lsb_reg(x: u32) -> u32 { if x != 0 { 0x06 } else { 0x04 } }
#[inline]
const fn das08jr_ao_msb_reg(x: u32) -> u32 { if x != 0 { 0x07 } else { 0x05 } }
#[inline]
const fn das08aox_ao_lsb_reg(x: u32) -> u32 { if x != 0 { 0x0a } else { 0x08 } }
#[inline]
const fn das08aox_ao_msb_reg(x: u32) -> u32 { if x != 0 { 0x0b } else { 0x09 } }

static DAS08_PGL_AI_RANGE: comedi_lrange = comedi_lrange { length: 9, range: [
    BIP_RANGE(10), BIP_RANGE(5), BIP_RANGE(2.5), BIP_RANGE(1.25), BIP_RANGE(0.625),
    UNI_RANGE(10), UNI_RANGE(5), UNI_RANGE(2.5), UNI_RANGE(1.25)
] };
static DAS08_PGH_AI_RANGE: comedi_lrange = comedi_lrange { length: 12, range: [
    BIP_RANGE(10), BIP_RANGE(5), BIP_RANGE(1), BIP_RANGE(0.5), BIP_RANGE(0.1),
    BIP_RANGE(0.05), BIP_RANGE(0.01), BIP_RANGE(0.005), UNI_RANGE(10), UNI_RANGE(1),
    UNI_RANGE(0.1), UNI_RANGE(0.01)
] };
static DAS08_PGM_AI_RANGE: comedi_lrange = comedi_lrange { length: 9, range: [
    BIP_RANGE(10), BIP_RANGE(5), BIP_RANGE(0.5), BIP_RANGE(0.05), BIP_RANGE(0.01),
    UNI_RANGE(10), UNI_RANGE(1), UNI_RANGE(0.1), UNI_RANGE(0.01)
] };

static DAS08_PGH_AI_GAINLIST: [i32; 12] = [8, 0, 10, 2, 12, 4, 14, 6, 1, 3, 5, 7];
static DAS08_PGL_AI_GAINLIST: [i32; 9] = [8, 0, 2, 4, 6, 1, 3, 5, 7];
static DAS08_PGM_AI_GAINLIST: [i32; 9] = [8, 0, 10, 12, 14, 9, 11, 13, 15];

static DAS08_AI_LRANGES: [*const comedi_lrange; 5] = [
    &range_unknown, &range_bipolar5, &DAS08_PGH_AI_RANGE,
    &DAS08_PGL_AI_RANGE, &DAS08_PGM_AI_RANGE,
];
static DAS08_AI_GAINLISTS: [*const i32; 5] = [
    core::ptr::null(), core::ptr::null(), DAS08_PGH_AI_GAINLIST.as_ptr(),
    DAS08_PGL_AI_GAINLIST.as_ptr(), DAS08_PGM_AI_GAINLIST.as_ptr(),
];

unsafe fn das08_ai_eoc(dev: *mut comedi_device, _s: *mut comedi_subdevice,
                       _insn: *mut comedi_insn, _context: c_ulong) -> c_int {
    let status = inb((*dev).iobase + DAS08_STATUS_REG as c_ulong);
    if status & DAS08_STATUS_AI_BUSY as u8 == 0 { 0 } else { -EBUSY }
}

unsafe fn das08_ai_insn_read(dev: *mut comedi_device, s: *mut comedi_subdevice,
                             insn: *mut comedi_insn, data: *mut c_uint) -> c_int {
    let board = (*dev).board_ptr as *const das08_board_struct;
    let devpriv = (*dev).private as *mut das08_private_struct;
    let chan = CR_CHAN((*insn).chanspec);
    inb((*dev).iobase + DAS08_AI_LSB_REG as c_ulong);
    inb((*dev).iobase + DAS08_AI_MSB_REG as c_ulong);
    spin_lock(&mut (*dev).spinlock);
    (*devpriv).do_mux_bits &= !DAS08_CONTROL_MUX_MASK;
    (*devpriv).do_mux_bits |= das08_control_mux(chan);
    outb((*devpriv).do_mux_bits as u8, (*dev).iobase + DAS08_CONTROL_REG as c_ulong);
    spin_unlock(&mut (*dev).spinlock);
    if !(*devpriv).pg_gainlist.is_null() {
        let range = CR_RANGE((*insn).chanspec);
        outb(*(*devpriv).pg_gainlist.add(range as usize) as u8,
             (*dev).iobase + DAS08_GAIN_REG as c_ulong);
    }
    let mut n = 0;
    while n < (*insn).n {
        if (*board).ai_nbits == 16 && inb((*dev).iobase + DAS08_AI_MSB_REG as c_ulong) & 0x80 != 0 {
            dev_info((*dev).class_dev, "over-range\n");
        }
        outb_p(0, (*dev).iobase + DAS08_AI_TRIG_REG as c_ulong);
        let ret = comedi_timeout(dev, s, insn, das08_ai_eoc, 0);
        if ret != 0 { return ret; }
        let msb = inb((*dev).iobase + DAS08_AI_MSB_REG as c_ulong) as c_int;
        let lsb = inb((*dev).iobase + DAS08_AI_LSB_REG as c_ulong) as c_int;
        *data.add(n as usize) = if (*board).ai_encoding == das08_encode12 {
            ((lsb >> 4) | (msb << 4)) as c_uint
        } else if (*board).ai_encoding == das08_pcm_encode12 {
            ((msb << 8) + lsb) as c_uint
        } else if (*board).ai_encoding == das08_encode16 {
            let magnitude = (lsb | ((msb & 0x7f) << 8)) as c_uint;
            if msb & 0x80 != 0 { (1 << 15) + magnitude } else { (1 << 15) - magnitude }
        } else {
            dev_err((*dev).class_dev, "bug! unknown ai encoding\n");
            return -1;
        };
        n += 1;
    }
    n as c_int
}

unsafe fn das08_di_insn_bits(dev: *mut comedi_device, _s: *mut comedi_subdevice,
                             insn: *mut comedi_insn, data: *mut c_uint) -> c_int {
    *data = 0;
    *data.add(1) = das08_status_di(inb((*dev).iobase + DAS08_STATUS_REG as c_ulong) as u32);
    (*insn).n as c_int
}

unsafe fn das08_do_insn_bits(dev: *mut comedi_device, s: *mut comedi_subdevice,
                             insn: *mut comedi_insn, data: *mut c_uint) -> c_int {
    let devpriv = (*dev).private as *mut das08_private_struct;
    if comedi_dio_update_state(s, data) != 0 {
        spin_lock(&mut (*dev).spinlock);
        (*devpriv).do_mux_bits &= !DAS08_CONTROL_DO_MASK;
        (*devpriv).do_mux_bits |= das08_control_do((*s).state);
        outb((*devpriv).do_mux_bits as u8, (*dev).iobase + DAS08_CONTROL_REG as c_ulong);
        spin_unlock(&mut (*dev).spinlock);
    }
    *data.add(1) = (*s).state;
    (*insn).n as c_int
}

unsafe fn das08jr_di_insn_bits(dev: *mut comedi_device, _s: *mut comedi_subdevice,
                               insn: *mut comedi_insn, data: *mut c_uint) -> c_int {
    *data = 0; *data.add(1) = inb((*dev).iobase + DAS08JR_DI_REG as c_ulong) as c_uint;
    (*insn).n as c_int
}

unsafe fn das08jr_do_insn_bits(dev: *mut comedi_device, s: *mut comedi_subdevice,
                               insn: *mut comedi_insn, data: *mut c_uint) -> c_int {
    if comedi_dio_update_state(s, data) != 0 { outb((*s).state as u8, (*dev).iobase + DAS08JR_DO_REG as c_ulong); }
    *data.add(1) = (*s).state; (*insn).n as c_int
}

unsafe fn das08_ao_set_data(dev: *mut comedi_device, chan: c_uint, data: c_uint) {
    let board = (*dev).board_ptr as *const das08_board_struct;
    let lsb = (data & 0xff) as u8; let msb = ((data >> 8) & 0xff) as u8;
    if (*board).is_jr {
        outb(lsb, (*dev).iobase + das08jr_ao_lsb_reg(chan) as c_ulong);
        outb(msb, (*dev).iobase + das08jr_ao_msb_reg(chan) as c_ulong);
        inb((*dev).iobase + DAS08JR_AO_UPDATE_REG as c_ulong);
    } else {
        outb(lsb, (*dev).iobase + das08aox_ao_lsb_reg(chan) as c_ulong);
        outb(msb, (*dev).iobase + das08aox_ao_msb_reg(chan) as c_ulong);
        inb((*dev).iobase + DAS08AOX_AO_UPDATE_REG as c_ulong);
    }
}

unsafe fn das08_ao_insn_write(dev: *mut comedi_device, s: *mut comedi_subdevice,
                              insn: *mut comedi_insn, data: *mut c_uint) -> c_int {
    let chan = CR_CHAN((*insn).chanspec) as usize;
    let mut val = *(*s).readback.add(chan);
    for i in 0..(*insn).n as usize { val = *data.add(i); das08_ao_set_data(dev, chan as c_uint, val); }
    *(*s).readback.add(chan) = val; (*insn).n as c_int
}

pub unsafe fn das08_common_attach(dev: *mut comedi_device, iobase: c_ulong) -> c_int {
    let board = (*dev).board_ptr as *const das08_board_struct;
    let devpriv = (*dev).private as *mut das08_private_struct;
    (*dev).iobase = iobase; (*dev).board_name = (*board).name;
    let mut ret = comedi_alloc_subdevices(dev, 6); if ret != 0 { return ret; }
    let s = &mut *(*dev).subdevices.add(0);
    if (*board).ai_nbits != 0 {
        s.type_ = COMEDI_SUBD_AI; s.subdev_flags = SDF_READABLE | SDF_GROUND; s.n_chan = 8;
        s.maxdata = (1 << (*board).ai_nbits) - 1; s.range_table = DAS08_AI_LRANGES[(*board).ai_pg as usize];
        s.insn_read = Some(das08_ai_insn_read); (*devpriv).pg_gainlist = DAS08_AI_GAINLISTS[(*board).ai_pg as usize];
    } else { s.type_ = COMEDI_SUBD_UNUSED; }
    let s = &mut *(*dev).subdevices.add(1);
    if (*board).ao_nbits != 0 {
        s.type_ = COMEDI_SUBD_AO; s.subdev_flags = SDF_WRITABLE; s.n_chan = 2;
        s.maxdata = (1 << (*board).ao_nbits) - 1; s.range_table = &range_bipolar5; s.insn_write = Some(das08_ao_insn_write);
        ret = comedi_alloc_subdev_readback(s); if ret != 0 { return ret; }
        for i in 0..s.n_chan { *s.readback.add(i as usize) = s.maxdata / 2; das08_ao_set_data(dev, i as c_uint, *s.readback.add(i as usize)); }
    } else { s.type_ = COMEDI_SUBD_UNUSED; }
    let s = &mut *(*dev).subdevices.add(2);
    if (*board).di_nchan != 0 { s.type_ = COMEDI_SUBD_DI; s.subdev_flags = SDF_READABLE; s.n_chan = (*board).di_nchan; s.maxdata = 1; s.range_table = &range_digital; s.insn_bits = Some(if (*board).is_jr { das08jr_di_insn_bits } else { das08_di_insn_bits }); } else { s.type_ = COMEDI_SUBD_UNUSED; }
    let s = &mut *(*dev).subdevices.add(3);
    if (*board).do_nchan != 0 { s.type_ = COMEDI_SUBD_DO; s.subdev_flags = SDF_WRITABLE; s.n_chan = (*board).do_nchan; s.maxdata = 1; s.range_table = &range_digital; s.insn_bits = Some(if (*board).is_jr { das08jr_do_insn_bits } else { das08_do_insn_bits }); } else { s.type_ = COMEDI_SUBD_UNUSED; }
    let s = &mut *(*dev).subdevices.add(4);
    if (*board).i8255_offset != 0 { ret = subdev_8255_io_init(dev, s, (*board).i8255_offset); if ret != 0 { return ret; } } else { s.type_ = COMEDI_SUBD_UNUSED; }
    let s = &mut *(*dev).subdevices.add(5);
    if (*board).i8254_offset != 0 { (*dev).pacer = comedi_8254_io_alloc((*dev).iobase + (*board).i8254_offset, 0, I8254_IO8, 0); if IS_ERR((*dev).pacer) { return PTR_ERR((*dev).pacer); } comedi_8254_subdevice_init(s, (*dev).pacer); } else { s.type_ = COMEDI_SUBD_UNUSED; }
    0
}

// EXPORT_SYMBOL_GPL(das08_common_attach);
// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_DESCRIPTION("Comedi common DAS08 support module");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
