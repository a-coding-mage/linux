// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  I/O routines for GF1/InterWave synthesizer chips
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

pub type c_uchar = u8;
pub type c_ushort = u16;
pub type c_uint = u32;
pub type c_int = i32;
pub type c_short = i16;
pub type c_ulong = u64;

#[repr(C)]
pub struct snd_gus_card {
    pub gf1: snd_gus_card_gf1,
    pub reg_lock: spinlock_t,
    pub interwave: c_int,
    pub card: *mut snd_card,
}

#[repr(C)]
pub struct snd_gus_card_gf1 {
    pub reg_regsel: c_ulong,
    pub reg_data8: c_ulong,
    pub reg_data16: c_ulong,
    pub reg_timerctrl: c_ulong,
    pub reg_timerdata: c_ulong,
    pub reg_dram: c_ulong,
    pub enh_mode: c_int,
    pub active_voices: c_ushort,
    pub playback_freq: c_uint,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_irqsave_guard {
    _private: [u8; 0],
}

extern "C" {
    fn mb();
    fn inb(port: c_ulong) -> c_uchar;
    fn inw(port: c_ulong) -> c_ushort;
    fn outb(value: c_uchar, port: c_ulong);
    fn outw(value: c_ushort, port: c_ulong);
    fn udelay(usecs: c_ulong);
    fn GUSP(gus: *mut snd_gus_card, reg: c_int) -> c_ulong;
    fn guard_spinlock_irqsave(lock: *mut spinlock_t) -> spinlock_irqsave_guard;
    fn dev_dbg(dev: *mut device, fmt: *const c_uchar, ...);

    static SNDRV_GF1_VB_UPPER_ADDRESS: c_uchar;
    static SNDRV_GF1_GW_DRAM_IO_LOW: c_uchar;
    static SNDRV_GF1_GB_DRAM_IO_HIGH: c_uchar;
    static SNDRV_GF1_GW_DRAM_IO16: c_uchar;
    static SNDRV_GF1_GB_ACTIVE_VOICES: c_uchar;
    static DRAM: c_int;
    static GF1DATALOW: c_int;
}

pub unsafe fn snd_gf1_delay(gus: *mut snd_gus_card) {
    let mut i: c_int;

    i = 0;
    while i < 6 {
        mb();
        inb(GUSP(gus, DRAM));
        i += 1;
    }
}

/*
 *  =======================================================================
 */

/*
 *  ok.. stop of control registers (wave & ramp) need some special things..
 *       big UltraClick (tm) elimination...
 */

#[inline]
unsafe fn __snd_gf1_ctrl_stop(gus: *mut snd_gus_card, reg: c_uchar) {
    let value: c_uchar;

    outb(reg | 0x80, (*gus).gf1.reg_regsel);
    mb();
    value = inb((*gus).gf1.reg_data8);
    mb();
    outb(reg, (*gus).gf1.reg_regsel);
    mb();
    outb((value | 0x03) & !(0x80 | 0x20), (*gus).gf1.reg_data8);
    mb();
}

#[inline]
unsafe fn __snd_gf1_write8(gus: *mut snd_gus_card, reg: c_uchar, data: c_uchar) {
    outb(reg, (*gus).gf1.reg_regsel);
    mb();
    outb(data, (*gus).gf1.reg_data8);
    mb();
}

#[inline]
unsafe fn __snd_gf1_look8(gus: *mut snd_gus_card, reg: c_uchar) -> c_uchar {
    outb(reg, (*gus).gf1.reg_regsel);
    mb();
    inb((*gus).gf1.reg_data8)
}

#[inline]
unsafe fn __snd_gf1_write16(gus: *mut snd_gus_card, reg: c_uchar, data: c_uint) {
    outb(reg, (*gus).gf1.reg_regsel);
    mb();
    outw(data as c_ushort, (*gus).gf1.reg_data16);
    mb();
}

#[inline]
unsafe fn __snd_gf1_look16(gus: *mut snd_gus_card, reg: c_uchar) -> c_ushort {
    outb(reg, (*gus).gf1.reg_regsel);
    mb();
    inw((*gus).gf1.reg_data16)
}

#[inline]
unsafe fn __snd_gf1_adlib_write(gus: *mut snd_gus_card, reg: c_uchar, data: c_uchar) {
    outb(reg, (*gus).gf1.reg_timerctrl);
    inb((*gus).gf1.reg_timerctrl);
    inb((*gus).gf1.reg_timerctrl);
    outb(data, (*gus).gf1.reg_timerdata);
    inb((*gus).gf1.reg_timerctrl);
    inb((*gus).gf1.reg_timerctrl);
}

#[inline]
unsafe fn __snd_gf1_write_addr(
    gus: *mut snd_gus_card,
    reg: c_uchar,
    mut addr: c_uint,
    w_16bit: c_int,
) {
    if (*gus).gf1.enh_mode != 0 {
        if w_16bit != 0 {
            addr = ((addr >> 1) & !0x0000000f) | (addr & 0x0000000f);
        }
        __snd_gf1_write8(
            gus,
            SNDRV_GF1_VB_UPPER_ADDRESS,
            ((addr >> 26) & 0x03) as c_uchar,
        );
    } else if w_16bit != 0 {
        addr = (addr & 0x00c0000f) | ((addr & 0x003ffff0) >> 1);
    }
    __snd_gf1_write16(gus, reg, (addr >> 11) as c_ushort as c_uint);
    __snd_gf1_write16(gus, reg.wrapping_add(1), (addr << 5) as c_ushort as c_uint);
}

#[inline]
unsafe fn __snd_gf1_read_addr(gus: *mut snd_gus_card, reg: c_uchar, w_16bit: c_short) -> c_uint {
    let mut res: c_uint;

    res = ((__snd_gf1_look16(gus, reg | 0x80) as c_uint) << 11) & 0xfff800;
    res |= ((__snd_gf1_look16(gus, reg.wrapping_add(1) | 0x80) as c_uint) >> 5) & 0x0007ff;
    if (*gus).gf1.enh_mode != 0 {
        res |= (__snd_gf1_look8(gus, SNDRV_GF1_VB_UPPER_ADDRESS | 0x80) as c_uint) << 26;
        if w_16bit != 0 {
            res = ((res << 1) & 0xffffffe0) | (res & 0x0000000f);
        }
    } else if w_16bit != 0 {
        res = ((res & 0x001ffff0) << 1) | (res & 0x00c0000f);
    }
    res
}

/*
 *  =======================================================================
 */

pub unsafe fn snd_gf1_ctrl_stop(gus: *mut snd_gus_card, reg: c_uchar) {
    __snd_gf1_ctrl_stop(gus, reg);
}

pub unsafe fn snd_gf1_write8(gus: *mut snd_gus_card, reg: c_uchar, data: c_uchar) {
    __snd_gf1_write8(gus, reg, data);
}

pub unsafe fn snd_gf1_look8(gus: *mut snd_gus_card, reg: c_uchar) -> c_uchar {
    __snd_gf1_look8(gus, reg)
}

pub unsafe fn snd_gf1_write16(gus: *mut snd_gus_card, reg: c_uchar, data: c_uint) {
    __snd_gf1_write16(gus, reg, data);
}

pub unsafe fn snd_gf1_look16(gus: *mut snd_gus_card, reg: c_uchar) -> c_ushort {
    __snd_gf1_look16(gus, reg)
}

pub unsafe fn snd_gf1_adlib_write(gus: *mut snd_gus_card, reg: c_uchar, data: c_uchar) {
    __snd_gf1_adlib_write(gus, reg, data);
}

pub unsafe fn snd_gf1_write_addr(
    gus: *mut snd_gus_card,
    reg: c_uchar,
    addr: c_uint,
    w_16bit: c_short,
) {
    __snd_gf1_write_addr(gus, reg, addr, w_16bit as c_int);
}

pub unsafe fn snd_gf1_read_addr(gus: *mut snd_gus_card, reg: c_uchar, w_16bit: c_short) -> c_uint {
    __snd_gf1_read_addr(gus, reg, w_16bit)
}

/*

 */

pub unsafe fn snd_gf1_i_ctrl_stop(gus: *mut snd_gus_card, reg: c_uchar) {
    let _guard = guard_spinlock_irqsave(&mut (*gus).reg_lock);
    __snd_gf1_ctrl_stop(gus, reg);
}

pub unsafe fn snd_gf1_i_write8(gus: *mut snd_gus_card, reg: c_uchar, data: c_uchar) {
    let _guard = guard_spinlock_irqsave(&mut (*gus).reg_lock);
    __snd_gf1_write8(gus, reg, data);
}

pub unsafe fn snd_gf1_i_look8(gus: *mut snd_gus_card, reg: c_uchar) -> c_uchar {
    let _guard = guard_spinlock_irqsave(&mut (*gus).reg_lock);
    __snd_gf1_look8(gus, reg)
}

pub unsafe fn snd_gf1_i_write16(gus: *mut snd_gus_card, reg: c_uchar, data: c_uint) {
    let _guard = guard_spinlock_irqsave(&mut (*gus).reg_lock);
    __snd_gf1_write16(gus, reg, data);
}

pub unsafe fn snd_gf1_i_look16(gus: *mut snd_gus_card, reg: c_uchar) -> c_ushort {
    let _guard = guard_spinlock_irqsave(&mut (*gus).reg_lock);
    __snd_gf1_look16(gus, reg)
}

pub unsafe fn snd_gf1_dram_addr(gus: *mut snd_gus_card, addr: c_uint) {
    outb(0x43, (*gus).gf1.reg_regsel);
    mb();
    outw(addr as c_ushort, (*gus).gf1.reg_data16);
    mb();
    outb(0x44, (*gus).gf1.reg_regsel);
    mb();
    outb((addr >> 16) as c_uchar, (*gus).gf1.reg_data8);
    mb();
}

pub unsafe fn snd_gf1_poke(gus: *mut snd_gus_card, addr: c_uint, data: c_uchar) {
    let _guard = guard_spinlock_irqsave(&mut (*gus).reg_lock);
    outb(SNDRV_GF1_GW_DRAM_IO_LOW, (*gus).gf1.reg_regsel);
    mb();
    outw(addr as c_ushort, (*gus).gf1.reg_data16);
    mb();
    outb(SNDRV_GF1_GB_DRAM_IO_HIGH, (*gus).gf1.reg_regsel);
    mb();
    outb((addr >> 16) as c_uchar, (*gus).gf1.reg_data8);
    mb();
    outb(data, (*gus).gf1.reg_dram);
}

pub unsafe fn snd_gf1_peek(gus: *mut snd_gus_card, addr: c_uint) -> c_uchar {
    let _guard = guard_spinlock_irqsave(&mut (*gus).reg_lock);
    outb(SNDRV_GF1_GW_DRAM_IO_LOW, (*gus).gf1.reg_regsel);
    mb();
    outw(addr as c_ushort, (*gus).gf1.reg_data16);
    mb();
    outb(SNDRV_GF1_GB_DRAM_IO_HIGH, (*gus).gf1.reg_regsel);
    mb();
    outb((addr >> 16) as c_uchar, (*gus).gf1.reg_data8);
    mb();
    inb((*gus).gf1.reg_dram)
}

/*
 * Original C kept the following DRAM word helpers under `#if 0`.
 * They are translated but disabled for the same build-time intent.
 */
#[cfg(any())]
pub unsafe fn snd_gf1_pokew(gus: *mut snd_gus_card, addr: c_uint, data: c_ushort) {
    if (*gus).interwave == 0 {
        dev_dbg((*(*gus).card).dev, b"%s - GF1!!!\n\0".as_ptr(), b"snd_gf1_pokew\0".as_ptr());
    }
    let _guard = guard_spinlock_irqsave(&mut (*gus).reg_lock);
    outb(SNDRV_GF1_GW_DRAM_IO_LOW, (*gus).gf1.reg_regsel);
    mb();
    outw(addr as c_ushort, (*gus).gf1.reg_data16);
    mb();
    outb(SNDRV_GF1_GB_DRAM_IO_HIGH, (*gus).gf1.reg_regsel);
    mb();
    outb((addr >> 16) as c_uchar, (*gus).gf1.reg_data8);
    mb();
    outb(SNDRV_GF1_GW_DRAM_IO16, (*gus).gf1.reg_regsel);
    mb();
    outw(data, (*gus).gf1.reg_data16);
}

#[cfg(any())]
pub unsafe fn snd_gf1_peekw(gus: *mut snd_gus_card, addr: c_uint) -> c_ushort {
    if (*gus).interwave == 0 {
        dev_dbg((*(*gus).card).dev, b"%s - GF1!!!\n\0".as_ptr(), b"snd_gf1_peekw\0".as_ptr());
    }
    let _guard = guard_spinlock_irqsave(&mut (*gus).reg_lock);
    outb(SNDRV_GF1_GW_DRAM_IO_LOW, (*gus).gf1.reg_regsel);
    mb();
    outw(addr as c_ushort, (*gus).gf1.reg_data16);
    mb();
    outb(SNDRV_GF1_GB_DRAM_IO_HIGH, (*gus).gf1.reg_regsel);
    mb();
    outb((addr >> 16) as c_uchar, (*gus).gf1.reg_data8);
    mb();
    outb(SNDRV_GF1_GW_DRAM_IO16, (*gus).gf1.reg_regsel);
    mb();
    inw((*gus).gf1.reg_data16)
}

#[cfg(any())]
pub unsafe fn snd_gf1_dram_setmem(
    gus: *mut snd_gus_card,
    mut addr: c_uint,
    value: c_ushort,
    mut count: c_uint,
) {
    let port: c_ulong;

    if (*gus).interwave == 0 {
        dev_dbg(
            (*(*gus).card).dev,
            b"%s - GF1!!!\n\0".as_ptr(),
            b"snd_gf1_dram_setmem\0".as_ptr(),
        );
    }
    addr &= !1;
    count >>= 1;
    port = GUSP(gus, GF1DATALOW);
    let _guard = guard_spinlock_irqsave(&mut (*gus).reg_lock);
    outb(SNDRV_GF1_GW_DRAM_IO_LOW, (*gus).gf1.reg_regsel);
    mb();
    outw(addr as c_ushort, (*gus).gf1.reg_data16);
    mb();
    outb(SNDRV_GF1_GB_DRAM_IO_HIGH, (*gus).gf1.reg_regsel);
    mb();
    outb((addr >> 16) as c_uchar, (*gus).gf1.reg_data8);
    mb();
    outb(SNDRV_GF1_GW_DRAM_IO16, (*gus).gf1.reg_regsel);
    while count != 0 {
        count = count.wrapping_sub(1);
        outw(value, port);
    }
}

pub unsafe fn snd_gf1_select_active_voices(gus: *mut snd_gus_card) {
    let mut voices: c_ushort;

    static voices_tbl: [c_ushort; 32 - 14 + 1] = [
        44100, 41160, 38587, 36317, 34300, 32494, 30870, 29400, 28063, 26843, 25725, 24696, 23746,
        22866, 22050, 21289, 20580, 19916, 19293,
    ];

    voices = (*gus).gf1.active_voices;
    if voices > 32 {
        voices = 32;
    }
    if voices < 14 {
        voices = 14;
    }
    if (*gus).gf1.enh_mode != 0 {
        voices = 32;
    }
    (*gus).gf1.active_voices = voices;
    (*gus).gf1.playback_freq = if (*gus).gf1.enh_mode != 0 {
        44100
    } else {
        voices_tbl[(voices - 14) as usize] as c_uint
    };
    if (*gus).gf1.enh_mode == 0 {
        snd_gf1_i_write8(gus, SNDRV_GF1_GB_ACTIVE_VOICES, 0xc0 | ((voices - 1) as c_uchar));
        udelay(100);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
