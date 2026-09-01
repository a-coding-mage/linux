// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Sound Core PDAudioCF soundcard
 *
 * Copyright (c) 2003 by Jaroslav Kysela <perex@perex.cz>
 */

// Dependencies originally supplied by:
// <sound/core.h>, "pdaudiocf.h", <sound/initval.h>, <asm/irq_regs.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_int, c_ulong, c_void};

type u8 = u8;
type u16 = u16;
type u32 = u32;
type irqreturn_t = c_int;

const IRQ_HANDLED: irqreturn_t = 1;
const IRQ_WAKE_THREAD: irqreturn_t = 2;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct ak4117 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pdacf {
    pub chip_status: c_int,
    pub port: c_ulong,
    pub card: *mut snd_card,
    pub ak4117: *mut ak4117,
    pub pcm_substream: *mut snd_pcm_substream,
    pub pcm_xor: c_uint,
    pub pcm_sample: c_int,
    pub pcm_little: c_int,
    pub pcm_channels: c_int,
    pub pcm_area: *mut c_void,
    pub pcm_swab: c_int,
    pub pcm_frame: c_int,
    pub pcm_hwptr: c_int,
    pub pcm_tdone: c_int,
    pub pcm_size: c_int,
    pub pcm_period: c_int,
    pub reg_lock: mutex,
}

type c_uint = u32;

unsafe extern "C" {
    static PDAUDIOCF_STAT_IS_STALE: c_int;
    static PDAUDIOCF_STAT_IS_CONFIGURED: c_int;
    static PDAUDIOCF_STAT_IS_SUSPENDED: c_int;
    static PDAUDIOCF_REG_ISR: c_ulong;
    static PDAUDIOCF_IRQLVL: u16;
    static PDAUDIOCF_IRQOVR: u16;
    static PDAUDIOCF_IRQAKM: u16;
    static PDAUDIOCF_REG_MD: c_ulong;
    static PDAUDIOCF_REG_RDP: c_ulong;
    static PDAUDIOCF_REG_WDP: c_ulong;

    fn inw(port: c_ulong) -> u16;
    fn get_irq_regs() -> *mut pt_regs;
    fn snd_ak4117_check_rate_and_errors(ak4117: *mut ak4117, flags: c_int);
    fn dev_err(dev: *mut device, fmt: *const i8, ...);
    fn snd_pcm_running(substream: *mut snd_pcm_substream) -> c_int;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
}

#[inline]
fn swab16(x: u16) -> u16 {
    x.swap_bytes()
}

#[inline]
fn swab32(x: u32) -> u32 {
    x.swap_bytes()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pdacf_interrupt(_irq: c_int, dev: *mut c_void) -> irqreturn_t {
    let chip = dev as *mut snd_pdacf;
    let mut stat: u16;
    let mut wake_thread = false;

    if ((*chip).chip_status
        & (PDAUDIOCF_STAT_IS_STALE | PDAUDIOCF_STAT_IS_CONFIGURED | PDAUDIOCF_STAT_IS_SUSPENDED))
        != PDAUDIOCF_STAT_IS_CONFIGURED
    {
        return IRQ_HANDLED; /* IRQ_NONE here? */
    }

    stat = inw((*chip).port.wrapping_add(PDAUDIOCF_REG_ISR));
    if (stat & (PDAUDIOCF_IRQLVL | PDAUDIOCF_IRQOVR)) != 0 {
        if (stat & PDAUDIOCF_IRQOVR) != 0 {
            /* should never happen */
            dev_err(
                (*(*chip).card).dev,
                c"PDAUDIOCF SRAM buffer overrun detected!\n".as_ptr(),
            );
        }
        if !(*chip).pcm_substream.is_null() {
            wake_thread = true;
        }
        if (stat & PDAUDIOCF_IRQAKM) == 0 {
            stat |= PDAUDIOCF_IRQAKM; /* check rate */
        }
    }
    if !get_irq_regs().is_null() {
        snd_ak4117_check_rate_and_errors((*chip).ak4117, 0);
    }
    if wake_thread { IRQ_WAKE_THREAD } else { IRQ_HANDLED }
}

#[inline]
unsafe fn pdacf_transfer_mono16(mut dst: *mut u16, xor: u16, mut size: c_uint, rdp_port: c_ulong) {
    while size > 0 {
        size = size.wrapping_sub(1);
        *dst = inw(rdp_port) ^ xor;
        dst = dst.add(1);
        inw(rdp_port);
    }
}

#[inline]
unsafe fn pdacf_transfer_mono32(mut dst: *mut u32, xor: u32, mut size: c_uint, rdp_port: c_ulong) {
    let mut val1: u16;
    let mut val2: u16;

    while size > 0 {
        size = size.wrapping_sub(1);
        val1 = inw(rdp_port);
        val2 = inw(rdp_port);
        inw(rdp_port);
        *dst = ((((val2 as u32) & 0xff) << 24) | ((val1 as u32) << 8)) ^ xor;
        dst = dst.add(1);
    }
}

#[inline]
unsafe fn pdacf_transfer_stereo16(mut dst: *mut u16, xor: u16, mut size: c_uint, rdp_port: c_ulong) {
    while size > 0 {
        size = size.wrapping_sub(1);
        *dst = inw(rdp_port) ^ xor;
        dst = dst.add(1);
        *dst = inw(rdp_port) ^ xor;
        dst = dst.add(1);
    }
}

#[inline]
unsafe fn pdacf_transfer_stereo32(mut dst: *mut u32, xor: u32, mut size: c_uint, rdp_port: c_ulong) {
    let mut val1: u16;
    let mut val2: u16;
    let mut val3: u16;

    while size > 0 {
        size = size.wrapping_sub(1);
        val1 = inw(rdp_port);
        val2 = inw(rdp_port);
        val3 = inw(rdp_port);
        *dst = ((((val2 as u32) & 0xff) << 24) | ((val1 as u32) << 8)) ^ xor;
        dst = dst.add(1);
        *dst = (((val3 as u32) << 16) | ((val2 & 0xff00) as u32)) ^ xor;
        dst = dst.add(1);
    }
}

#[inline]
unsafe fn pdacf_transfer_mono16sw(mut dst: *mut u16, xor: u16, mut size: c_uint, rdp_port: c_ulong) {
    while size > 0 {
        size = size.wrapping_sub(1);
        *dst = swab16(inw(rdp_port) ^ xor);
        dst = dst.add(1);
        inw(rdp_port);
    }
}

#[inline]
unsafe fn pdacf_transfer_mono32sw(mut dst: *mut u32, xor: u32, mut size: c_uint, rdp_port: c_ulong) {
    let mut val1: u16;
    let mut val2: u16;

    while size > 0 {
        size = size.wrapping_sub(1);
        val1 = inw(rdp_port);
        val2 = inw(rdp_port);
        inw(rdp_port);
        *dst = swab32(((((val2 & 0xff) as u32) << 24) | ((val1 as u32) << 8)) ^ xor);
        dst = dst.add(1);
    }
}

#[inline]
unsafe fn pdacf_transfer_stereo16sw(mut dst: *mut u16, xor: u16, mut size: c_uint, rdp_port: c_ulong) {
    while size > 0 {
        size = size.wrapping_sub(1);
        *dst = swab16(inw(rdp_port) ^ xor);
        dst = dst.add(1);
        *dst = swab16(inw(rdp_port) ^ xor);
        dst = dst.add(1);
    }
}

#[inline]
unsafe fn pdacf_transfer_stereo32sw(mut dst: *mut u32, xor: u32, mut size: c_uint, rdp_port: c_ulong) {
    let mut val1: u16;
    let mut val2: u16;
    let mut val3: u16;

    while size > 0 {
        size = size.wrapping_sub(1);
        val1 = inw(rdp_port);
        val2 = inw(rdp_port);
        val3 = inw(rdp_port);
        *dst = swab32(((((val2 & 0xff) as u32) << 24) | ((val1 as u32) << 8)) ^ xor);
        dst = dst.add(1);
        *dst = swab32((((val3 as u32) << 16) | ((val2 & 0xff00) as u32)) ^ xor);
        dst = dst.add(1);
    }
}

#[inline]
unsafe fn pdacf_transfer_mono24le(mut dst: *mut u8, xor: u16, mut size: c_uint, rdp_port: c_ulong) {
    let mut val1: u16;
    let mut val2: u16;
    let mut xval1: u32;

    while size > 0 {
        size = size.wrapping_sub(1);
        val1 = inw(rdp_port);
        val2 = inw(rdp_port);
        inw(rdp_port);
        xval1 = ((((val2 & 0xff) as u32) << 8) | ((val1 as u32) << 16)) ^ (xor as u32);
        *dst = (xval1 >> 8) as u8;
        dst = dst.add(1);
        *dst = (xval1 >> 16) as u8;
        dst = dst.add(1);
        *dst = (xval1 >> 24) as u8;
        dst = dst.add(1);
    }
}

#[inline]
unsafe fn pdacf_transfer_mono24be(mut dst: *mut u8, xor: u16, mut size: c_uint, rdp_port: c_ulong) {
    let mut val1: u16;
    let mut val2: u16;
    let mut xval1: u32;

    while size > 0 {
        size = size.wrapping_sub(1);
        val1 = inw(rdp_port);
        val2 = inw(rdp_port);
        inw(rdp_port);
        xval1 = ((((val2 & 0xff) as u32) << 8) | ((val1 as u32) << 16)) ^ (xor as u32);
        *dst = (xval1 >> 24) as u8;
        dst = dst.add(1);
        *dst = (xval1 >> 16) as u8;
        dst = dst.add(1);
        *dst = (xval1 >> 8) as u8;
        dst = dst.add(1);
    }
}

#[inline]
unsafe fn pdacf_transfer_stereo24le(mut dst: *mut u8, xor: u32, mut size: c_uint, rdp_port: c_ulong) {
    let mut val1: u16;
    let mut val2: u16;
    let mut val3: u16;
    let mut xval1: u32;
    let mut xval2: u32;

    while size > 0 {
        size = size.wrapping_sub(1);
        val1 = inw(rdp_port);
        val2 = inw(rdp_port);
        val3 = inw(rdp_port);
        xval1 = ((((val2 as u32) & 0xff) << 24) | ((val1 as u32) << 8)) ^ xor;
        xval2 = (((val3 as u32) << 16) | ((val2 & 0xff00) as u32)) ^ xor;
        *dst = (xval1 >> 8) as u8;
        dst = dst.add(1);
        *dst = (xval1 >> 16) as u8;
        dst = dst.add(1);
        *dst = (xval1 >> 24) as u8;
        dst = dst.add(1);
        *dst = (xval2 >> 8) as u8;
        dst = dst.add(1);
        *dst = (xval2 >> 16) as u8;
        dst = dst.add(1);
        *dst = (xval2 >> 24) as u8;
        dst = dst.add(1);
    }
}

#[inline]
unsafe fn pdacf_transfer_stereo24be(mut dst: *mut u8, xor: u32, mut size: c_uint, rdp_port: c_ulong) {
    let mut val1: u16;
    let mut val2: u16;
    let mut val3: u16;
    let mut xval1: u32;
    let mut xval2: u32;

    while size > 0 {
        size = size.wrapping_sub(1);
        val1 = inw(rdp_port);
        val2 = inw(rdp_port);
        val3 = inw(rdp_port);
        xval1 = ((((val2 as u32) & 0xff) << 24) | ((val1 as u32) << 8)) ^ xor;
        xval2 = (((val3 as u32) << 16) | ((val2 & 0xff00) as u32)) ^ xor;
        *dst = (xval1 >> 24) as u8;
        dst = dst.add(1);
        *dst = (xval1 >> 16) as u8;
        dst = dst.add(1);
        *dst = (xval1 >> 8) as u8;
        dst = dst.add(1);
        *dst = (xval2 >> 24) as u8;
        dst = dst.add(1);
        *dst = (xval2 >> 16) as u8;
        dst = dst.add(1);
        *dst = (xval2 >> 8) as u8;
        dst = dst.add(1);
    }
}

unsafe fn pdacf_transfer(chip: *mut snd_pdacf, size: c_uint, off: c_uint) {
    let rdp_port: c_ulong = (*chip).port.wrapping_add(PDAUDIOCF_REG_MD);
    let xor: c_uint = (*chip).pcm_xor;

    if (*chip).pcm_sample == 3 {
        if (*chip).pcm_little != 0 {
            if (*chip).pcm_channels == 1 {
                pdacf_transfer_mono24le(
                    ((*chip).pcm_area as *mut u8).add((off * 3) as usize),
                    xor as u16,
                    size,
                    rdp_port,
                );
            } else {
                pdacf_transfer_stereo24le(
                    ((*chip).pcm_area as *mut u8).add((off * 6) as usize),
                    xor,
                    size,
                    rdp_port,
                );
            }
        } else if (*chip).pcm_channels == 1 {
            pdacf_transfer_mono24be(
                ((*chip).pcm_area as *mut u8).add((off * 3) as usize),
                xor as u16,
                size,
                rdp_port,
            );
        } else {
            pdacf_transfer_stereo24be(
                ((*chip).pcm_area as *mut u8).add((off * 6) as usize),
                xor,
                size,
                rdp_port,
            );
        }
        return;
    }
    if (*chip).pcm_swab == 0 {
        if (*chip).pcm_channels == 1 {
            if (*chip).pcm_frame == 2 {
                pdacf_transfer_mono16(((*chip).pcm_area as *mut u16).add(off as usize), xor as u16, size, rdp_port);
            } else {
                pdacf_transfer_mono32(((*chip).pcm_area as *mut u32).add(off as usize), xor, size, rdp_port);
            }
        } else if (*chip).pcm_frame == 2 {
            pdacf_transfer_stereo16(((*chip).pcm_area as *mut u16).add((off * 2) as usize), xor as u16, size, rdp_port);
        } else {
            pdacf_transfer_stereo32(((*chip).pcm_area as *mut u32).add((off * 2) as usize), xor, size, rdp_port);
        }
    } else if (*chip).pcm_channels == 1 {
        if (*chip).pcm_frame == 2 {
            pdacf_transfer_mono16sw(((*chip).pcm_area as *mut u16).add(off as usize), xor as u16, size, rdp_port);
        } else {
            pdacf_transfer_mono32sw(((*chip).pcm_area as *mut u32).add(off as usize), xor, size, rdp_port);
        }
    } else if (*chip).pcm_frame == 2 {
        pdacf_transfer_stereo16sw(((*chip).pcm_area as *mut u16).add((off * 2) as usize), xor as u16, size, rdp_port);
    } else {
        pdacf_transfer_stereo32sw(((*chip).pcm_area as *mut u32).add((off * 2) as usize), xor, size, rdp_port);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pdacf_threaded_irq(_irq: c_int, dev: *mut c_void) -> irqreturn_t {
    let chip = dev as *mut snd_pdacf;
    let mut size: c_int;
    let mut off: c_int;
    let mut cont: c_int;
    let rdp: c_int;
    let wdp: c_int;

    if ((*chip).chip_status & (PDAUDIOCF_STAT_IS_STALE | PDAUDIOCF_STAT_IS_CONFIGURED))
        != PDAUDIOCF_STAT_IS_CONFIGURED
    {
        return IRQ_HANDLED;
    }

    if (*chip).pcm_substream.is_null()
        || (*(*chip).pcm_substream).runtime.is_null()
        || snd_pcm_running((*chip).pcm_substream) == 0
    {
        return IRQ_HANDLED;
    }

    rdp = inw((*chip).port.wrapping_add(PDAUDIOCF_REG_RDP)) as c_int;
    wdp = inw((*chip).port.wrapping_add(PDAUDIOCF_REG_WDP)) as c_int;
    size = wdp - rdp;
    if size < 0 {
        size += 0x10000;
    }
    if size == 0 {
        size = 0x10000;
    }
    size /= (*chip).pcm_frame;
    if size > 64 {
        size -= 32;
    }

    /*
     * Original C kept an alternate implementation under #if 0 here. That
     * disabled code only consumed samples from PDAUDIOCF_REG_MD while updating
     * pcm_hwptr/pcm_tdone; the active #else path below is translated.
     */
    off = (*chip).pcm_hwptr + (*chip).pcm_tdone;
    off %= (*chip).pcm_size;
    (*chip).pcm_tdone += size;
    while size > 0 {
        cont = (*chip).pcm_size - off;
        if cont > size {
            cont = size;
        }
        pdacf_transfer(chip, cont as c_uint, off as c_uint);
        off += cont;
        off %= (*chip).pcm_size;
        size -= cont;
    }

    mutex_lock(&mut (*chip).reg_lock);
    while (*chip).pcm_tdone >= (*chip).pcm_period {
        (*chip).pcm_hwptr += (*chip).pcm_period;
        (*chip).pcm_hwptr %= (*chip).pcm_size;
        (*chip).pcm_tdone -= (*chip).pcm_period;
        mutex_unlock(&mut (*chip).reg_lock);
        snd_pcm_period_elapsed((*chip).pcm_substream);
        mutex_lock(&mut (*chip).reg_lock);
    }
    mutex_unlock(&mut (*chip).reg_lock);
    IRQ_HANDLED
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
