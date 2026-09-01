// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *     and (c) 1999 Steve Ratcliffe <steve@parabola.demon.co.uk>
 *  Copyright (C) 1999-2000 Takashi Iwai <tiwai@suse.de>
 *
 *  Routines for control of EMU8000 chip
 */

/*
 * Dependencies originally supplied by:
 * linux/wait.h, linux/sched/signal.h, linux/slab.h, linux/ioport.h,
 * linux/export.h, linux/delay.h, linux/io.h, linux/string.h,
 * sound/core.h, sound/emu8000.h, sound/emu8000_reg.h, linux/uaccess.h,
 * linux/init.h, sound/control.h, sound/initval.h.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

/*
 * External types, constants, and helpers are supplied by the surrounding
 * translated kernel/sound tree.
 */
extern "C" {
    static current: *mut c_void;

    fn outw(value: u16, port: c_uint);
    fn inw(port: c_uint) -> u16;
    fn outb(value: u8, port: c_uint);
    fn schedule_timeout_interruptible(timeout: c_long) -> c_long;
    fn signal_pending(task: *mut c_void) -> c_int;
    fn msleep(msecs: c_uint);
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> c_ulong;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn devm_kzalloc(dev: *mut c_void, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_request_region(dev: *mut c_void, start: c_ulong, n: c_ulong, name: *const c_char) -> *mut c_void;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_remove(card: *mut snd_card, kcontrol: *mut snd_kcontrol);
    fn snd_seq_device_new(
        card: *mut snd_card,
        device: c_int,
        id: *const c_char,
        argsize: c_int,
        result: *mut *mut snd_seq_device,
    ) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn dev_dbg(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn snd_BUG_ON(cond: bool) -> bool;
    fn spin_lock_init(lock: *mut c_void);
    fn spin_lock_irqsave(lock: *mut c_void) -> c_ulong;
    fn spin_unlock_irqrestore(lock: *mut c_void, flags: c_ulong);

    fn EMU8000_PTR(emu: *mut snd_emu8000) -> c_uint;
    fn EMU8000_DATA0(emu: *mut snd_emu8000) -> c_uint;
    fn EMU8000_DATA1(emu: *mut snd_emu8000) -> c_uint;
    fn EMU8000_DATA2(emu: *mut snd_emu8000) -> c_uint;
    fn EMU8000_CMD(cmd: c_int, ch: c_int) -> c_uint;
    fn SNDRV_SEQ_DEVICE_ARGPTR(awe: *mut snd_seq_device) -> *mut c_void;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_emu8000;

    fn EMU8000_CCCA_WRITE(emu: *mut snd_emu8000, ch: c_int, val: c_uint);
    fn EMU8000_DCYSUSV_WRITE(emu: *mut snd_emu8000, ch: c_int, val: c_uint);
    fn EMU8000_VTFT_WRITE(emu: *mut snd_emu8000, ch: c_int, val: c_uint);
    fn EMU8000_CVCF_WRITE(emu: *mut snd_emu8000, ch: c_int, val: c_uint);
    fn EMU8000_PTRX_WRITE(emu: *mut snd_emu8000, ch: c_int, val: c_uint);
    fn EMU8000_CPF_WRITE(emu: *mut snd_emu8000, ch: c_int, val: c_uint);
    fn EMU8000_PSST_WRITE(emu: *mut snd_emu8000, ch: c_int, val: c_uint);
    fn EMU8000_CSL_WRITE(emu: *mut snd_emu8000, ch: c_int, val: c_uint);
    fn EMU8000_SMALR_READ(emu: *mut snd_emu8000) -> c_uint;
    fn EMU8000_SMALW_READ(emu: *mut snd_emu8000) -> c_uint;
    fn EMU8000_HWCF1_WRITE(emu: *mut snd_emu8000, val: c_uint);
    fn EMU8000_HWCF2_WRITE(emu: *mut snd_emu8000, val: c_uint);
    fn EMU8000_HWCF3_WRITE(emu: *mut snd_emu8000, val: c_uint);
    fn EMU8000_HWCF1_READ(emu: *mut snd_emu8000) -> c_uint;
    fn EMU8000_HWCF2_READ(emu: *mut snd_emu8000) -> c_uint;
    fn EMU8000_ENVVOL_WRITE(emu: *mut snd_emu8000, ch: c_int, val: c_uint);
    fn EMU8000_ENVVAL_WRITE(emu: *mut snd_emu8000, ch: c_int, val: c_uint);
    fn EMU8000_DCYSUS_WRITE(emu: *mut snd_emu8000, ch: c_int, val: c_uint);
    fn EMU8000_ATKHLDV_WRITE(emu: *mut snd_emu8000, ch: c_int, val: c_uint);
    fn EMU8000_LFO1VAL_WRITE(emu: *mut snd_emu8000, ch: c_int, val: c_uint);
    fn EMU8000_ATKHLD_WRITE(emu: *mut snd_emu8000, ch: c_int, val: c_uint);
    fn EMU8000_LFO2VAL_WRITE(emu: *mut snd_emu8000, ch: c_int, val: c_uint);
    fn EMU8000_IP_WRITE(emu: *mut snd_emu8000, ch: c_int, val: c_uint);
    fn EMU8000_IFATN_WRITE(emu: *mut snd_emu8000, ch: c_int, val: c_uint);
    fn EMU8000_PEFE_WRITE(emu: *mut snd_emu8000, ch: c_int, val: c_uint);
    fn EMU8000_FMMOD_WRITE(emu: *mut snd_emu8000, ch: c_int, val: c_uint);
    fn EMU8000_TREMFRQ_WRITE(emu: *mut snd_emu8000, ch: c_int, val: c_uint);
    fn EMU8000_FM2FRQ2_WRITE(emu: *mut snd_emu8000, ch: c_int, val: c_uint);
    fn EMU8000_SMALR_WRITE(emu: *mut snd_emu8000, val: c_uint);
    fn EMU8000_SMARR_WRITE(emu: *mut snd_emu8000, val: c_uint);
    fn EMU8000_SMALW_WRITE(emu: *mut snd_emu8000, val: c_uint);
    fn EMU8000_SMARW_WRITE(emu: *mut snd_emu8000, val: c_uint);
    fn EMU8000_INIT1_WRITE(emu: *mut snd_emu8000, ch: c_int, val: u16);
    fn EMU8000_INIT2_WRITE(emu: *mut snd_emu8000, ch: c_int, val: u16);
    fn EMU8000_INIT3_WRITE(emu: *mut snd_emu8000, ch: c_int, val: u16);
    fn EMU8000_INIT4_WRITE(emu: *mut snd_emu8000, ch: c_int, val: u16);
    fn EMU8000_HWCF4_WRITE(emu: *mut snd_emu8000, val: c_uint);
    fn EMU8000_HWCF5_WRITE(emu: *mut snd_emu8000, val: c_uint);
    fn EMU8000_HWCF6_WRITE(emu: *mut snd_emu8000, val: c_uint);
    fn EMU8000_HWCF7_WRITE(emu: *mut snd_emu8000, val: c_uint);
    fn EMU8000_SMLD_WRITE(emu: *mut snd_emu8000, val: u16);
    fn EMU8000_SMLD_READ(emu: *mut snd_emu8000) -> u16;
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut c_void,
}

#[repr(C)]
pub struct snd_seq_device {
    pub name: [c_char; 80],
}

#[repr(C)]
pub struct snd_emu8000 {
    pub reg_lock: c_void,
    pub control_lock: c_void,
    pub last_reg: c_uint,
    pub index: c_int,
    pub port1: c_ulong,
    pub port2: c_ulong,
    pub port3: c_ulong,
    pub mem_size: c_int,
    pub dram_checked: c_int,
    pub card: *mut snd_card,
    pub seq_ports: c_int,
    pub bass_level: c_int,
    pub treble_level: c_int,
    pub chorus_mode: c_int,
    pub reverb_mode: c_int,
    pub fm_chorus_depth: c_uint,
    pub fm_reverb_depth: c_uint,
    pub controls: [*mut snd_kcontrol; EMU8000_NUM_CONTROLS],
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_integer {
    pub min: c_long,
    pub max: c_long,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}

const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const EFAULT: c_int = 14;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;
const GFP_KERNEL: c_uint = 0;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 2;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const EMU8000_RAM_RIGHT: c_int = 0x100;
const EMU8000_RAM_MODE_MASK: c_int = 0xff;
const EMU8000_RAM_CLOSE: c_int = 0;
const EMU8000_RAM_WRITE: c_int = 1;
const EMU8000_RAM_READ: c_int = 2;
const EMU8000_CHANNELS: c_int = 32;
const EMU8000_DRAM_VOICES: c_int = 30;
const EMU8000_DRAM_OFFSET: c_int = 0x200000;
const EMU8000_MAX_DRAM: c_int = 28 * 1024 * 1024;
const EMU8000_NUM_CONTROLS: usize = 6;
const SNDRV_SEQ_DEV_ID_EMU8000: *const c_char = b"emu8000\0".as_ptr() as *const c_char;

/*
 * emu8000 register controls
 */

/*
 * The following routines read and write registers on the emu8000.  They
 * should always be called via the EMU8000*READ/WRITE macros and never
 * directly.  The macros handle the port number and command word.
 */
/* Write a word */
#[no_mangle]
pub unsafe extern "C" fn snd_emu8000_poke(
    emu: *mut snd_emu8000,
    port: c_uint,
    reg: c_uint,
    val: c_uint,
) {
    let flags = spin_lock_irqsave(&mut (*emu).reg_lock as *mut _ as *mut c_void);
    if reg != (*emu).last_reg {
        outw(reg as u16, EMU8000_PTR(emu)); /* Set register */
        (*emu).last_reg = reg;
    }
    outw(val as u16, port); /* Send data */
    spin_unlock_irqrestore(&mut (*emu).reg_lock as *mut _ as *mut c_void, flags);
}

/* Read a word */
#[no_mangle]
pub unsafe extern "C" fn snd_emu8000_peek(
    emu: *mut snd_emu8000,
    port: c_uint,
    reg: c_uint,
) -> u16 {
    let flags = spin_lock_irqsave(&mut (*emu).reg_lock as *mut _ as *mut c_void);
    if reg != (*emu).last_reg {
        outw(reg as u16, EMU8000_PTR(emu)); /* Set register */
        (*emu).last_reg = reg;
    }
    let ret = inw(port); /* Read data */
    spin_unlock_irqrestore(&mut (*emu).reg_lock as *mut _ as *mut c_void, flags);
    ret
}

/* Write a double word */
#[no_mangle]
pub unsafe extern "C" fn snd_emu8000_poke_dw(
    emu: *mut snd_emu8000,
    port: c_uint,
    reg: c_uint,
    val: c_uint,
) {
    let flags = spin_lock_irqsave(&mut (*emu).reg_lock as *mut _ as *mut c_void);
    if reg != (*emu).last_reg {
        outw(reg as u16, EMU8000_PTR(emu)); /* Set register */
        (*emu).last_reg = reg;
    }
    outw(val as u16, port); /* Send low word of data */
    outw((val >> 16) as u16, port.wrapping_add(2)); /* Send high word of data */
    spin_unlock_irqrestore(&mut (*emu).reg_lock as *mut _ as *mut c_void, flags);
}

/* Read a double word */
#[no_mangle]
pub unsafe extern "C" fn snd_emu8000_peek_dw(
    emu: *mut snd_emu8000,
    port: c_uint,
    reg: c_uint,
) -> c_uint {
    let low: u16;

    let flags = spin_lock_irqsave(&mut (*emu).reg_lock as *mut _ as *mut c_void);
    if reg != (*emu).last_reg {
        outw(reg as u16, EMU8000_PTR(emu)); /* Set register */
        (*emu).last_reg = reg;
    }
    low = inw(port); /* Read low word of data */
    let ret = (low as c_uint).wrapping_add((inw(port.wrapping_add(2)) as c_uint) << 16);
    spin_unlock_irqrestore(&mut (*emu).reg_lock as *mut _ as *mut c_void, flags);
    ret
}

/*
 * Set up / close a channel to be used for DMA.
 */
/*exported*/
#[no_mangle]
pub unsafe extern "C" fn snd_emu8000_dma_chan(emu: *mut snd_emu8000, ch: c_int, mut mode: c_int) {
    let right_bit: c_uint = if (mode & EMU8000_RAM_RIGHT) != 0 { 0x01000000 } else { 0 };
    mode &= EMU8000_RAM_MODE_MASK;
    if mode == EMU8000_RAM_CLOSE {
        EMU8000_CCCA_WRITE(emu, ch, 0);
        EMU8000_DCYSUSV_WRITE(emu, ch, 0x807F);
        return;
    }
    EMU8000_DCYSUSV_WRITE(emu, ch, 0x80);
    EMU8000_VTFT_WRITE(emu, ch, 0);
    EMU8000_CVCF_WRITE(emu, ch, 0);
    EMU8000_PTRX_WRITE(emu, ch, 0x40000000);
    EMU8000_CPF_WRITE(emu, ch, 0x40000000);
    EMU8000_PSST_WRITE(emu, ch, 0);
    EMU8000_CSL_WRITE(emu, ch, 0);
    if mode == EMU8000_RAM_WRITE {
        /* DMA write */
        EMU8000_CCCA_WRITE(emu, ch, 0x06000000 | right_bit);
    } else {
        /* DMA read */
        EMU8000_CCCA_WRITE(emu, ch, 0x04000000 | right_bit);
    }
}

unsafe extern "C" fn snd_emu8000_read_wait(emu: *mut snd_emu8000) {
    while (EMU8000_SMALR_READ(emu) & 0x80000000) != 0 {
        schedule_timeout_interruptible(1);
        if signal_pending(current) != 0 {
            break;
        }
    }
}

unsafe extern "C" fn snd_emu8000_write_wait(emu: *mut snd_emu8000) {
    while (EMU8000_SMALW_READ(emu) & 0x80000000) != 0 {
        schedule_timeout_interruptible(1);
        if signal_pending(current) != 0 {
            break;
        }
    }
}

/*
 * detect a card at the given port
 */
unsafe extern "C" fn snd_emu8000_detect(emu: *mut snd_emu8000) -> c_int {
    /* Initialise */
    EMU8000_HWCF1_WRITE(emu, 0x0059);
    EMU8000_HWCF2_WRITE(emu, 0x0020);
    EMU8000_HWCF3_WRITE(emu, 0x0000);
    /* Check for a recognisable emu8000 */
    /*
    if ((EMU8000_U1_READ(emu) & 0x000f) != 0x000c)
        return -ENODEV;
        */
    if (EMU8000_HWCF1_READ(emu) & 0x007e) != 0x0058 {
        return -ENODEV;
    }
    if (EMU8000_HWCF2_READ(emu) & 0x0003) != 0x0003 {
        return -ENODEV;
    }

    dev_dbg(
        (*(*emu).card).dev,
        b"EMU8000 [0x%lx]: Synth chip found\n\0".as_ptr() as *const c_char,
        (*emu).port1,
    );
    0
}

/*
 * intiailize audio channels
 */
unsafe extern "C" fn init_audio(emu: *mut snd_emu8000) {
    let mut ch: c_int;

    /* turn off envelope engines */
    ch = 0;
    while ch < EMU8000_CHANNELS {
        EMU8000_DCYSUSV_WRITE(emu, ch, 0x80);
        ch += 1;
    }

    /* reset all other parameters to zero */
    ch = 0;
    while ch < EMU8000_CHANNELS {
        EMU8000_ENVVOL_WRITE(emu, ch, 0);
        EMU8000_ENVVAL_WRITE(emu, ch, 0);
        EMU8000_DCYSUS_WRITE(emu, ch, 0);
        EMU8000_ATKHLDV_WRITE(emu, ch, 0);
        EMU8000_LFO1VAL_WRITE(emu, ch, 0);
        EMU8000_ATKHLD_WRITE(emu, ch, 0);
        EMU8000_LFO2VAL_WRITE(emu, ch, 0);
        EMU8000_IP_WRITE(emu, ch, 0);
        EMU8000_IFATN_WRITE(emu, ch, 0);
        EMU8000_PEFE_WRITE(emu, ch, 0);
        EMU8000_FMMOD_WRITE(emu, ch, 0);
        EMU8000_TREMFRQ_WRITE(emu, ch, 0);
        EMU8000_FM2FRQ2_WRITE(emu, ch, 0);
        EMU8000_PTRX_WRITE(emu, ch, 0);
        EMU8000_VTFT_WRITE(emu, ch, 0);
        EMU8000_PSST_WRITE(emu, ch, 0);
        EMU8000_CSL_WRITE(emu, ch, 0);
        EMU8000_CCCA_WRITE(emu, ch, 0);
        ch += 1;
    }

    ch = 0;
    while ch < EMU8000_CHANNELS {
        EMU8000_CPF_WRITE(emu, ch, 0);
        EMU8000_CVCF_WRITE(emu, ch, 0);
        ch += 1;
    }
}

/*
 * initialize DMA address
 */
unsafe extern "C" fn init_dma(emu: *mut snd_emu8000) {
    EMU8000_SMALR_WRITE(emu, 0);
    EMU8000_SMARR_WRITE(emu, 0);
    EMU8000_SMALW_WRITE(emu, 0);
    EMU8000_SMARW_WRITE(emu, 0);
}

/*
 * initialization arrays; from ADIP
 */
static INIT1: [u16; 128] = [
    0x03ff, 0x0030, 0x07ff, 0x0130, 0x0bff, 0x0230, 0x0fff, 0x0330,
    0x13ff, 0x0430, 0x17ff, 0x0530, 0x1bff, 0x0630, 0x1fff, 0x0730,
    0x23ff, 0x0830, 0x27ff, 0x0930, 0x2bff, 0x0a30, 0x2fff, 0x0b30,
    0x33ff, 0x0c30, 0x37ff, 0x0d30, 0x3bff, 0x0e30, 0x3fff, 0x0f30,
    0x43ff, 0x0030, 0x47ff, 0x0130, 0x4bff, 0x0230, 0x4fff, 0x0330,
    0x53ff, 0x0430, 0x57ff, 0x0530, 0x5bff, 0x0630, 0x5fff, 0x0730,
    0x63ff, 0x0830, 0x67ff, 0x0930, 0x6bff, 0x0a30, 0x6fff, 0x0b30,
    0x73ff, 0x0c30, 0x77ff, 0x0d30, 0x7bff, 0x0e30, 0x7fff, 0x0f30,
    0x83ff, 0x0030, 0x87ff, 0x0130, 0x8bff, 0x0230, 0x8fff, 0x0330,
    0x93ff, 0x0430, 0x97ff, 0x0530, 0x9bff, 0x0630, 0x9fff, 0x0730,
    0xa3ff, 0x0830, 0xa7ff, 0x0930, 0xabff, 0x0a30, 0xafff, 0x0b30,
    0xb3ff, 0x0c30, 0xb7ff, 0x0d30, 0xbbff, 0x0e30, 0xbfff, 0x0f30,
    0xc3ff, 0x0030, 0xc7ff, 0x0130, 0xcbff, 0x0230, 0xcfff, 0x0330,
    0xd3ff, 0x0430, 0xd7ff, 0x0530, 0xdbff, 0x0630, 0xdfff, 0x0730,
    0xe3ff, 0x0830, 0xe7ff, 0x0930, 0xebff, 0x0a30, 0xefff, 0x0b30,
    0xf3ff, 0x0c30, 0xf7ff, 0x0d30, 0xfbff, 0x0e30, 0xffff, 0x0f30,
];

static INIT2: [u16; 128] = [
    0x03ff, 0x8030, 0x07ff, 0x8130, 0x0bff, 0x8230, 0x0fff, 0x8330,
    0x13ff, 0x8430, 0x17ff, 0x8530, 0x1bff, 0x8630, 0x1fff, 0x8730,
    0x23ff, 0x8830, 0x27ff, 0x8930, 0x2bff, 0x8a30, 0x2fff, 0x8b30,
    0x33ff, 0x8c30, 0x37ff, 0x8d30, 0x3bff, 0x8e30, 0x3fff, 0x8f30,
    0x43ff, 0x8030, 0x47ff, 0x8130, 0x4bff, 0x8230, 0x4fff, 0x8330,
    0x53ff, 0x8430, 0x57ff, 0x8530, 0x5bff, 0x8630, 0x5fff, 0x8730,
    0x63ff, 0x8830, 0x67ff, 0x8930, 0x6bff, 0x8a30, 0x6fff, 0x8b30,
    0x73ff, 0x8c30, 0x77ff, 0x8d30, 0x7bff, 0x8e30, 0x7fff, 0x8f30,
    0x83ff, 0x8030, 0x87ff, 0x8130, 0x8bff, 0x8230, 0x8fff, 0x8330,
    0x93ff, 0x8430, 0x97ff, 0x8530, 0x9bff, 0x8630, 0x9fff, 0x8730,
    0xa3ff, 0x8830, 0xa7ff, 0x8930, 0xabff, 0x8a30, 0xafff, 0x8b30,
    0xb3ff, 0x8c30, 0xb7ff, 0x8d30, 0xbbff, 0x8e30, 0xbfff, 0x8f30,
    0xc3ff, 0x8030, 0xc7ff, 0x8130, 0xcbff, 0x8230, 0xcfff, 0x8330,
    0xd3ff, 0x8430, 0xd7ff, 0x8530, 0xdbff, 0x8630, 0xdfff, 0x8730,
    0xe3ff, 0x8830, 0xe7ff, 0x8930, 0xebff, 0x8a30, 0xefff, 0x8b30,
    0xf3ff, 0x8c30, 0xf7ff, 0x8d30, 0xfbff, 0x8e30, 0xffff, 0x8f30,
];

static INIT3: [u16; 128] = [
    0x0C10, 0x8470, 0x14FE, 0xB488, 0x167F, 0xA470, 0x18E7, 0x84B5,
    0x1B6E, 0x842A, 0x1F1D, 0x852A, 0x0DA3, 0x8F7C, 0x167E, 0xF254,
    0x0000, 0x842A, 0x0001, 0x852A, 0x18E6, 0x8BAA, 0x1B6D, 0xF234,
    0x229F, 0x8429, 0x2746, 0x8529, 0x1F1C, 0x86E7, 0x229E, 0xF224,
    0x0DA4, 0x8429, 0x2C29, 0x8529, 0x2745, 0x87F6, 0x2C28, 0xF254,
    0x383B, 0x8428, 0x320F, 0x8528, 0x320E, 0x8F02, 0x1341, 0xF264,
    0x3EB6, 0x8428, 0x3EB9, 0x8528, 0x383A, 0x8FA9, 0x3EB5, 0xF294,
    0x3EB7, 0x8474, 0x3EBA, 0x8575, 0x3EB8, 0xC4C3, 0x3EBB, 0xC5C3,
    0x0000, 0xA404, 0x0001, 0xA504, 0x141F, 0x8671, 0x14FD, 0x8287,
    0x3EBC, 0xE610, 0x3EC8, 0x8C7B, 0x031A, 0x87E6, 0x3EC8, 0x86F7,
    0x3EC0, 0x821E, 0x3EBE, 0xD208, 0x3EBD, 0x821F, 0x3ECA, 0x8386,
    0x3EC1, 0x8C03, 0x3EC9, 0x831E, 0x3ECA, 0x8C4C, 0x3EBF, 0x8C55,
    0x3EC9, 0xC208, 0x3EC4, 0xBC84, 0x3EC8, 0x8EAD, 0x3EC8, 0xD308,
    0x3EC2, 0x8F7E, 0x3ECB, 0x8219, 0x3ECB, 0xD26E, 0x3EC5, 0x831F,
    0x3EC6, 0xC308, 0x3EC3, 0xB2FF, 0x3EC9, 0x8265, 0x3EC9, 0x8319,
    0x1342, 0xD36E, 0x3EC7, 0xB3FF, 0x0000, 0x8365, 0x1420, 0x9570,
];

static INIT4: [u16; 128] = [
    0x0C10, 0x8470, 0x14FE, 0xB488, 0x167F, 0xA470, 0x18E7, 0x84B5,
    0x1B6E, 0x842A, 0x1F1D, 0x852A, 0x0DA3, 0x0F7C, 0x167E, 0x7254,
    0x0000, 0x842A, 0x0001, 0x852A, 0x18E6, 0x0BAA, 0x1B6D, 0x7234,
    0x229F, 0x8429, 0x2746, 0x8529, 0x1F1C, 0x06E7, 0x229E, 0x7224,
    0x0DA4, 0x8429, 0x2C29, 0x8529, 0x2745, 0x07F6, 0x2C28, 0x7254,
    0x383B, 0x8428, 0x320F, 0x8528, 0x320E, 0x0F02, 0x1341, 0x7264,
    0x3EB6, 0x8428, 0x3EB9, 0x8528, 0x383A, 0x0FA9, 0x3EB5, 0x7294,
    0x3EB7, 0x8474, 0x3EBA, 0x8575, 0x3EB8, 0x44C3, 0x3EBB, 0x45C3,
    0x0000, 0xA404, 0x0001, 0xA504, 0x141F, 0x0671, 0x14FD, 0x0287,
    0x3EBC, 0xE610, 0x3EC8, 0x0C7B, 0x031A, 0x07E6, 0x3EC8, 0x86F7,
    0x3EC0, 0x821E, 0x3EBE, 0xD208, 0x3EBD, 0x021F, 0x3ECA, 0x0386,
    0x3EC1, 0x0C03, 0x3EC9, 0x031E, 0x3ECA, 0x8C4C, 0x3EBF, 0x0C55,
    0x3EC9, 0xC208, 0x3EC4, 0xBC84, 0x3EC8, 0x0EAD, 0x3EC8, 0xD308,
    0x3EC2, 0x8F7E, 0x3ECB, 0x0219, 0x3ECB, 0xD26E, 0x3EC5, 0x031F,
    0x3EC6, 0xC308, 0x3EC3, 0x32FF, 0x3EC9, 0x0265, 0x3EC9, 0x8319,
    0x1342, 0xD36E, 0x3EC7, 0x33FF, 0x0000, 0x8365, 0x1420, 0x9570,
];

/* send an initialization array
 * Taken from the oss driver, not obvious from the doc how this
 * is meant to work
 */
unsafe extern "C" fn send_array(emu: *mut snd_emu8000, data: *const u16, size: c_int) {
    let mut i: c_int;
    let mut p: *const u16;

    p = data;
    i = 0;
    while i < size {
        EMU8000_INIT1_WRITE(emu, i, *p);
        i += 1;
        p = p.add(1);
    }
    i = 0;
    while i < size {
        EMU8000_INIT2_WRITE(emu, i, *p);
        i += 1;
        p = p.add(1);
    }
    i = 0;
    while i < size {
        EMU8000_INIT3_WRITE(emu, i, *p);
        i += 1;
        p = p.add(1);
    }
    i = 0;
    while i < size {
        EMU8000_INIT4_WRITE(emu, i, *p);
        i += 1;
        p = p.add(1);
    }
}

/*
 * Send initialization arrays to start up, this just follows the
 * initialisation sequence in the adip.
 */
unsafe extern "C" fn init_arrays(emu: *mut snd_emu8000) {
    send_array(emu, INIT1.as_ptr(), (INIT1.len() / 4) as c_int);

    msleep((1024 * 1000) / 44100); /* wait for 1024 clocks */
    send_array(emu, INIT2.as_ptr(), (INIT2.len() / 4) as c_int);
    send_array(emu, INIT3.as_ptr(), (INIT3.len() / 4) as c_int);

    EMU8000_HWCF4_WRITE(emu, 0);
    EMU8000_HWCF5_WRITE(emu, 0x83);
    EMU8000_HWCF6_WRITE(emu, 0x8000);

    send_array(emu, INIT4.as_ptr(), (INIT4.len() / 4) as c_int);
}

const UNIQUE_ID1: u16 = 0xa5b9;
const UNIQUE_ID2: u16 = 0x9d53;

/*
 * Size the onboard memory.
 * This is written so as not to need arbitrary delays after the write. It
 * seems that the only way to do this is to use the one channel and keep
 * reallocating between read and write.
 */
unsafe extern "C" fn size_dram(emu: *mut snd_emu8000) {
    let mut i: c_int;
    let mut size: c_int;

    if (*emu).dram_checked != 0 {
        return;
    }

    size = 0;

    /* write out a magic number */
    snd_emu8000_dma_chan(emu, 0, EMU8000_RAM_WRITE);
    snd_emu8000_dma_chan(emu, 1, EMU8000_RAM_READ);
    EMU8000_SMALW_WRITE(emu, EMU8000_DRAM_OFFSET as c_uint);
    EMU8000_SMLD_WRITE(emu, UNIQUE_ID1);
    snd_emu8000_init_fm(emu); /* This must really be here and not 2 lines back even */
    snd_emu8000_write_wait(emu);

    /*
     * Detect first 512 KiB.  If a write succeeds at the beginning of a
     * 512 KiB page we assume that the whole page is there.
     */
    EMU8000_SMALR_WRITE(emu, EMU8000_DRAM_OFFSET as c_uint);
    EMU8000_SMLD_READ(emu); /* discard stale data  */
    if EMU8000_SMLD_READ(emu) != UNIQUE_ID1 {
        /* No RAM */
    } else {
        snd_emu8000_read_wait(emu);

        size = 512 * 1024;
        while size < EMU8000_MAX_DRAM {
            /* Write a unique data on the test address.
             * if the address is out of range, the data is written on
             * 0x200000(=EMU8000_DRAM_OFFSET).  Then the id word is
             * changed by this data.
             */
            /*snd_emu8000_dma_chan(emu, 0, EMU8000_RAM_WRITE);*/
            EMU8000_SMALW_WRITE(emu, (EMU8000_DRAM_OFFSET + (size >> 1)) as c_uint);
            EMU8000_SMLD_WRITE(emu, UNIQUE_ID2);
            snd_emu8000_write_wait(emu);

            /*
             * read the data on the just written DRAM address
             * if not the same then we have reached the end of ram.
             */
            /*snd_emu8000_dma_chan(emu, 0, EMU8000_RAM_READ);*/
            EMU8000_SMALR_WRITE(emu, (EMU8000_DRAM_OFFSET + (size >> 1)) as c_uint);
            /*snd_emu8000_read_wait(emu);*/
            EMU8000_SMLD_READ(emu); /* discard stale data  */
            if EMU8000_SMLD_READ(emu) != UNIQUE_ID2 {
                break; /* no memory at this address */
            }
            snd_emu8000_read_wait(emu);

            /*
             * If it is the same it could be that the address just
             * wraps back to the beginning; so check to see if the
             * initial value has been overwritten.
             */
            EMU8000_SMALR_WRITE(emu, EMU8000_DRAM_OFFSET as c_uint);
            EMU8000_SMLD_READ(emu); /* discard stale data  */
            if EMU8000_SMLD_READ(emu) != UNIQUE_ID1 {
                break; /* we must have wrapped around */
            }
            snd_emu8000_read_wait(emu);

            /* Otherwise, it's valid memory. */
            size += 512 * 1024;
        }
    }

    /* wait until FULL bit in SMAxW register is false */
    i = 0;
    while i < 10000 {
        if (EMU8000_SMALW_READ(emu) & 0x80000000) == 0 {
            break;
        }
        schedule_timeout_interruptible(1);
        if signal_pending(current) != 0 {
            break;
        }
        i += 1;
    }
    snd_emu8000_dma_chan(emu, 0, EMU8000_RAM_CLOSE);
    snd_emu8000_dma_chan(emu, 1, EMU8000_RAM_CLOSE);

    pr_info(
        b"EMU8000 [0x%lx]: %d KiB on-board DRAM detected\n\0".as_ptr() as *const c_char,
        (*emu).port1,
        size / 1024,
    );

    (*emu).mem_size = size;
    (*emu).dram_checked = 1;
}

/*
 * Initiailise the FM section.  You have to do this to use sample RAM
 * and therefore lose 2 voices.
 */
/*exported*/
#[no_mangle]
pub unsafe extern "C" fn snd_emu8000_init_fm(emu: *mut snd_emu8000) {
    /* Initialize the last two channels for DRAM refresh and producing
       the reverb and chorus effects for Yamaha OPL-3 synthesizer */

    /* 31: FM left channel, 0xffffe0-0xffffe8 */
    EMU8000_DCYSUSV_WRITE(emu, 30, 0x80);
    EMU8000_PSST_WRITE(emu, 30, 0xFFFFFFE0); /* full left */
    EMU8000_CSL_WRITE(emu, 30, 0x00FFFFE8 | ((*emu).fm_chorus_depth << 24));
    EMU8000_PTRX_WRITE(emu, 30, (*emu).fm_reverb_depth << 8);
    EMU8000_CPF_WRITE(emu, 30, 0);
    EMU8000_CCCA_WRITE(emu, 30, 0x00FFFFE3);

    /* 32: FM right channel, 0xfffff0-0xfffff8 */
    EMU8000_DCYSUSV_WRITE(emu, 31, 0x80);
    EMU8000_PSST_WRITE(emu, 31, 0x00FFFFF0); /* full right */
    EMU8000_CSL_WRITE(emu, 31, 0x00FFFFF8 | ((*emu).fm_chorus_depth << 24));
    EMU8000_PTRX_WRITE(emu, 31, (*emu).fm_reverb_depth << 8);
    EMU8000_CPF_WRITE(emu, 31, 0x8000);
    EMU8000_CCCA_WRITE(emu, 31, 0x00FFFFF3);

    snd_emu8000_poke(emu, EMU8000_DATA0(emu), EMU8000_CMD(1, 30), 0);

    {
        let flags = spin_lock_irqsave(&mut (*emu).reg_lock as *mut _ as *mut c_void);
        while (inw(EMU8000_PTR(emu)) & 0x1000) == 0 {}
        while (inw(EMU8000_PTR(emu)) & 0x1000) != 0 {}
        spin_unlock_irqrestore(&mut (*emu).reg_lock as *mut _ as *mut c_void, flags);
    }
    snd_emu8000_poke(emu, EMU8000_DATA0(emu), EMU8000_CMD(1, 30), 0x4828);
    /* this is really odd part.. */
    outb(0x3C, EMU8000_PTR(emu));
    outb(0, EMU8000_DATA1(emu));

    /* skew volume & cutoff */
    EMU8000_VTFT_WRITE(emu, 30, 0x8000FFFF);
    EMU8000_VTFT_WRITE(emu, 31, 0x8000FFFF);
}

/*
 * The main initialization routine.
 */
unsafe extern "C" fn snd_emu8000_init_hw(emu: *mut snd_emu8000) {
    let mut i: c_int;

    (*emu).last_reg = 0xffff; /* reset the last register index */

    /* initialize hardware configuration */
    EMU8000_HWCF1_WRITE(emu, 0x0059);
    EMU8000_HWCF2_WRITE(emu, 0x0020);

    /* disable audio; this seems to reduce a clicking noise a bit.. */
    EMU8000_HWCF3_WRITE(emu, 0);

    /* initialize audio channels */
    init_audio(emu);

    /* initialize DMA */
    init_dma(emu);

    /* initialize init arrays */
    init_arrays(emu);

    /*
     * Initialize the FM section of the AWE32, this is needed
     * for DRAM refresh as well
     */
    snd_emu8000_init_fm(emu);

    /* terminate all voices */
    i = 0;
    while i < EMU8000_DRAM_VOICES {
        EMU8000_DCYSUSV_WRITE(emu, 0, 0x807F);
        i += 1;
    }

    /* check DRAM memory size */
    size_dram(emu);

    /* enable audio */
    EMU8000_HWCF3_WRITE(emu, 0x4);

    /* set equzlier, chorus and reverb modes */
    snd_emu8000_update_equalizer(emu);
    snd_emu8000_update_chorus_mode(emu);
    snd_emu8000_update_reverb_mode(emu);
}

/*----------------------------------------------------------------
 * Bass/Treble Equalizer
 *----------------------------------------------------------------*/

static BASS_PARM: [[u16; 3]; 12] = [
    [0xD26A, 0xD36A, 0x0000], /* -12 dB */
    [0xD25B, 0xD35B, 0x0000], /*  -8 */
    [0xD24C, 0xD34C, 0x0000], /*  -6 */
    [0xD23D, 0xD33D, 0x0000], /*  -4 */
    [0xD21F, 0xD31F, 0x0000], /*  -2 */
    [0xC208, 0xC308, 0x0001], /*   0 (HW default) */
    [0xC219, 0xC319, 0x0001], /*  +2 */
    [0xC22A, 0xC32A, 0x0001], /*  +4 */
    [0xC24C, 0xC34C, 0x0001], /*  +6 */
    [0xC26E, 0xC36E, 0x0001], /*  +8 */
    [0xC248, 0xC384, 0x0002], /* +10 */
    [0xC26A, 0xC36A, 0x0002], /* +12 dB */
];

static TREBLE_PARM: [[u16; 9]; 12] = [
    [0x821E, 0xC26A, 0x031E, 0xC36A, 0x021E, 0xD208, 0x831E, 0xD308, 0x0001], /* -12 dB */
    [0x821E, 0xC25B, 0x031E, 0xC35B, 0x021E, 0xD208, 0x831E, 0xD308, 0x0001],
    [0x821E, 0xC24C, 0x031E, 0xC34C, 0x021E, 0xD208, 0x831E, 0xD308, 0x0001],
    [0x821E, 0xC23D, 0x031E, 0xC33D, 0x021E, 0xD208, 0x831E, 0xD308, 0x0001],
    [0x821E, 0xC21F, 0x031E, 0xC31F, 0x021E, 0xD208, 0x831E, 0xD308, 0x0001],
    [0x821E, 0xD208, 0x031E, 0xD308, 0x021E, 0xD208, 0x831E, 0xD308, 0x0002],
    [0x821E, 0xD208, 0x031E, 0xD308, 0x021D, 0xD219, 0x831D, 0xD319, 0x0002],
    [0x821E, 0xD208, 0x031E, 0xD308, 0x021C, 0xD22A, 0x831C, 0xD32A, 0x0002],
    [0x821E, 0xD208, 0x031E, 0xD308, 0x021A, 0xD24C, 0x831A, 0xD34C, 0x0002],
    [0x821E, 0xD208, 0x031E, 0xD308, 0x0219, 0xD26E, 0x8319, 0xD36E, 0x0002], /* +8 (HW default) */
    [0x821D, 0xD219, 0x031D, 0xD319, 0x0219, 0xD26E, 0x8319, 0xD36E, 0x0002],
    [0x821C, 0xD22A, 0x031C, 0xD32A, 0x0219, 0xD26E, 0x8319, 0xD36E, 0x0002], /* +12 dB */
];

/*
 * set Emu8000 digital equalizer; from 0 to 11 [-12dB - 12dB]
 */
/*exported*/
#[no_mangle]
pub unsafe extern "C" fn snd_emu8000_update_equalizer(emu: *mut snd_emu8000) {
    let w: u16;
    let bass: c_int = (*emu).bass_level;
    let treble: c_int = (*emu).treble_level;

    if bass < 0 || bass > 11 || treble < 0 || treble > 11 {
        return;
    }
    EMU8000_INIT4_WRITE(emu, 0x01, BASS_PARM[bass as usize][0]);
    EMU8000_INIT4_WRITE(emu, 0x11, BASS_PARM[bass as usize][1]);
    EMU8000_INIT3_WRITE(emu, 0x11, TREBLE_PARM[treble as usize][0]);
    EMU8000_INIT3_WRITE(emu, 0x13, TREBLE_PARM[treble as usize][1]);
    EMU8000_INIT3_WRITE(emu, 0x1b, TREBLE_PARM[treble as usize][2]);
    EMU8000_INIT4_WRITE(emu, 0x07, TREBLE_PARM[treble as usize][3]);
    EMU8000_INIT4_WRITE(emu, 0x0b, TREBLE_PARM[treble as usize][4]);
    EMU8000_INIT4_WRITE(emu, 0x0d, TREBLE_PARM[treble as usize][5]);
    EMU8000_INIT4_WRITE(emu, 0x17, TREBLE_PARM[treble as usize][6]);
    EMU8000_INIT4_WRITE(emu, 0x19, TREBLE_PARM[treble as usize][7]);
    w = BASS_PARM[bass as usize][2].wrapping_add(TREBLE_PARM[treble as usize][8]);
    EMU8000_INIT4_WRITE(emu, 0x15, w.wrapping_add(0x0262));
    EMU8000_INIT4_WRITE(emu, 0x1d, w.wrapping_add(0x8362));
}

/*----------------------------------------------------------------
 * Chorus mode control
 *----------------------------------------------------------------*/

/*
 * chorus mode parameters
 */
const SNDRV_EMU8000_CHORUS_1: c_int = 0;
const SNDRV_EMU8000_CHORUS_2: c_int = 1;
const SNDRV_EMU8000_CHORUS_3: c_int = 2;
const SNDRV_EMU8000_CHORUS_4: c_int = 3;
const SNDRV_EMU8000_CHORUS_FEEDBACK: c_int = 4;
const SNDRV_EMU8000_CHORUS_FLANGER: c_int = 5;
const SNDRV_EMU8000_CHORUS_SHORTDELAY: c_int = 6;
const SNDRV_EMU8000_CHORUS_SHORTDELAY2: c_int = 7;
const SNDRV_EMU8000_CHORUS_PREDEFINED: c_int = 8;
/* user can define chorus modes up to 32 */
const SNDRV_EMU8000_CHORUS_NUMBERS: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct soundfont_chorus_fx {
    pub feedback: u16,     /* feedback level (0xE600-0xE6FF) */
    pub delay_offset: u16, /* delay (0-0x0DA3) [1/44100 sec] */
    pub lfo_depth: u16,    /* LFO depth (0xBC00-0xBCFF) */
    pub delay: c_uint,     /* right delay (0-0xFFFFFFFF) [1/256/44100 sec] */
    pub lfo_freq: c_uint,  /* LFO freq LFO freq (0-0xFFFFFFFF) */
}

/* 5 parameters for each chorus mode; 3 x 16bit, 2 x 32bit */
static mut CHORUS_DEFINED: [c_char; SNDRV_EMU8000_CHORUS_NUMBERS as usize] =
    [0; SNDRV_EMU8000_CHORUS_NUMBERS as usize];
static mut CHORUS_PARM: [soundfont_chorus_fx; SNDRV_EMU8000_CHORUS_NUMBERS as usize] = [
    soundfont_chorus_fx { feedback: 0xE600, delay_offset: 0x03F6, lfo_depth: 0xBC2C, delay: 0x00000000, lfo_freq: 0x0000006D }, /* chorus 1 */
    soundfont_chorus_fx { feedback: 0xE608, delay_offset: 0x031A, lfo_depth: 0xBC6E, delay: 0x00000000, lfo_freq: 0x0000017C }, /* chorus 2 */
    soundfont_chorus_fx { feedback: 0xE610, delay_offset: 0x031A, lfo_depth: 0xBC84, delay: 0x00000000, lfo_freq: 0x00000083 }, /* chorus 3 */
    soundfont_chorus_fx { feedback: 0xE620, delay_offset: 0x0269, lfo_depth: 0xBC6E, delay: 0x00000000, lfo_freq: 0x0000017C }, /* chorus 4 */
    soundfont_chorus_fx { feedback: 0xE680, delay_offset: 0x04D3, lfo_depth: 0xBCA6, delay: 0x00000000, lfo_freq: 0x0000005B }, /* feedback */
    soundfont_chorus_fx { feedback: 0xE6E0, delay_offset: 0x044E, lfo_depth: 0xBC37, delay: 0x00000000, lfo_freq: 0x00000026 }, /* flanger */
    soundfont_chorus_fx { feedback: 0xE600, delay_offset: 0x0B06, lfo_depth: 0xBC00, delay: 0x0006E000, lfo_freq: 0x00000083 }, /* short delay */
    soundfont_chorus_fx { feedback: 0xE6C0, delay_offset: 0x0B06, lfo_depth: 0xBC00, delay: 0x0006E000, lfo_freq: 0x00000083 }, /* short delay + feedback */
    soundfont_chorus_fx { feedback: 0, delay_offset: 0, lfo_depth: 0, delay: 0, lfo_freq: 0 },
    soundfont_chorus_fx { feedback: 0, delay_offset: 0, lfo_depth: 0, delay: 0, lfo_freq: 0 },
    soundfont_chorus_fx { feedback: 0, delay_offset: 0, lfo_depth: 0, delay: 0, lfo_freq: 0 },
    soundfont_chorus_fx { feedback: 0, delay_offset: 0, lfo_depth: 0, delay: 0, lfo_freq: 0 },
    soundfont_chorus_fx { feedback: 0, delay_offset: 0, lfo_depth: 0, delay: 0, lfo_freq: 0 },
    soundfont_chorus_fx { feedback: 0, delay_offset: 0, lfo_depth: 0, delay: 0, lfo_freq: 0 },
    soundfont_chorus_fx { feedback: 0, delay_offset: 0, lfo_depth: 0, delay: 0, lfo_freq: 0 },
    soundfont_chorus_fx { feedback: 0, delay_offset: 0, lfo_depth: 0, delay: 0, lfo_freq: 0 },
    soundfont_chorus_fx { feedback: 0, delay_offset: 0, lfo_depth: 0, delay: 0, lfo_freq: 0 },
    soundfont_chorus_fx { feedback: 0, delay_offset: 0, lfo_depth: 0, delay: 0, lfo_freq: 0 },
    soundfont_chorus_fx { feedback: 0, delay_offset: 0, lfo_depth: 0, delay: 0, lfo_freq: 0 },
    soundfont_chorus_fx { feedback: 0, delay_offset: 0, lfo_depth: 0, delay: 0, lfo_freq: 0 },
    soundfont_chorus_fx { feedback: 0, delay_offset: 0, lfo_depth: 0, delay: 0, lfo_freq: 0 },
    soundfont_chorus_fx { feedback: 0, delay_offset: 0, lfo_depth: 0, delay: 0, lfo_freq: 0 },
    soundfont_chorus_fx { feedback: 0, delay_offset: 0, lfo_depth: 0, delay: 0, lfo_freq: 0 },
    soundfont_chorus_fx { feedback: 0, delay_offset: 0, lfo_depth: 0, delay: 0, lfo_freq: 0 },
    soundfont_chorus_fx { feedback: 0, delay_offset: 0, lfo_depth: 0, delay: 0, lfo_freq: 0 },
    soundfont_chorus_fx { feedback: 0, delay_offset: 0, lfo_depth: 0, delay: 0, lfo_freq: 0 },
    soundfont_chorus_fx { feedback: 0, delay_offset: 0, lfo_depth: 0, delay: 0, lfo_freq: 0 },
    soundfont_chorus_fx { feedback: 0, delay_offset: 0, lfo_depth: 0, delay: 0, lfo_freq: 0 },
    soundfont_chorus_fx { feedback: 0, delay_offset: 0, lfo_depth: 0, delay: 0, lfo_freq: 0 },
    soundfont_chorus_fx { feedback: 0, delay_offset: 0, lfo_depth: 0, delay: 0, lfo_freq: 0 },
    soundfont_chorus_fx { feedback: 0, delay_offset: 0, lfo_depth: 0, delay: 0, lfo_freq: 0 },
    soundfont_chorus_fx { feedback: 0, delay_offset: 0, lfo_depth: 0, delay: 0, lfo_freq: 0 },
];

/*exported*/
#[no_mangle]
pub unsafe extern "C" fn snd_emu8000_load_chorus_fx(
    emu: *mut snd_emu8000,
    mode: c_int,
    buf: *const c_void,
    len: c_long,
) -> c_int {
    let mut rec = soundfont_chorus_fx {
        feedback: 0,
        delay_offset: 0,
        lfo_depth: 0,
        delay: 0,
        lfo_freq: 0,
    };
    if mode < SNDRV_EMU8000_CHORUS_PREDEFINED || mode >= SNDRV_EMU8000_CHORUS_NUMBERS {
        dev_warn(
            (*(*emu).card).dev,
            b"invalid chorus mode %d for uploading\n\0".as_ptr() as *const c_char,
            mode,
        );
        return -EINVAL;
    }
    if len < size_of::<soundfont_chorus_fx>() as c_long
        || copy_from_user(
            &mut rec as *mut _ as *mut c_void,
            buf,
            size_of::<soundfont_chorus_fx>(),
        ) != 0
    {
        return -EFAULT;
    }
    CHORUS_PARM[mode as usize] = rec;
    CHORUS_DEFINED[mode as usize] = 1;
    0
}

/*exported*/
#[no_mangle]
pub unsafe extern "C" fn snd_emu8000_update_chorus_mode(emu: *mut snd_emu8000) {
    let effect: c_int = (*emu).chorus_mode;
    if effect < 0
        || effect >= SNDRV_EMU8000_CHORUS_NUMBERS
        || (effect >= SNDRV_EMU8000_CHORUS_PREDEFINED && CHORUS_DEFINED[effect as usize] == 0)
    {
        return;
    }
    EMU8000_INIT3_WRITE(emu, 0x09, CHORUS_PARM[effect as usize].feedback);
    EMU8000_INIT3_WRITE(emu, 0x0c, CHORUS_PARM[effect as usize].delay_offset);
    EMU8000_INIT4_WRITE(emu, 0x03, CHORUS_PARM[effect as usize].lfo_depth);
    EMU8000_HWCF4_WRITE(emu, CHORUS_PARM[effect as usize].delay);
    EMU8000_HWCF5_WRITE(emu, CHORUS_PARM[effect as usize].lfo_freq);
    EMU8000_HWCF6_WRITE(emu, 0x8000);
    EMU8000_HWCF7_WRITE(emu, 0x0000);
}

/*----------------------------------------------------------------
 * Reverb mode control
 *----------------------------------------------------------------*/

/*
 * reverb mode parameters
 */
const SNDRV_EMU8000_REVERB_ROOM1: c_int = 0;
const SNDRV_EMU8000_REVERB_ROOM2: c_int = 1;
const SNDRV_EMU8000_REVERB_ROOM3: c_int = 2;
const SNDRV_EMU8000_REVERB_HALL1: c_int = 3;
const SNDRV_EMU8000_REVERB_HALL2: c_int = 4;
const SNDRV_EMU8000_REVERB_PLATE: c_int = 5;
const SNDRV_EMU8000_REVERB_DELAY: c_int = 6;
const SNDRV_EMU8000_REVERB_PANNINGDELAY: c_int = 7;
const SNDRV_EMU8000_REVERB_PREDEFINED: c_int = 8;
/* user can define reverb modes up to 32 */
const SNDRV_EMU8000_REVERB_NUMBERS: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct soundfont_reverb_fx {
    pub parms: [u16; 28],
}

/* reverb mode settings; write the following 28 data of 16 bit length
 *   on the corresponding ports in the reverb_cmds array
 */
static mut REVERB_DEFINED: [c_char; SNDRV_EMU8000_CHORUS_NUMBERS as usize] =
    [0; SNDRV_EMU8000_CHORUS_NUMBERS as usize];
static mut REVERB_PARM: [soundfont_reverb_fx; SNDRV_EMU8000_REVERB_NUMBERS as usize] = [
    soundfont_reverb_fx { parms: [0xB488, 0xA450, 0x9550, 0x84B5, 0x383A, 0x3EB5, 0x72F4, 0x72A4, 0x7254, 0x7204, 0x7204, 0x7204, 0x4416, 0x4516, 0xA490, 0xA590, 0x842A, 0x852A, 0x842A, 0x852A, 0x8429, 0x8529, 0x8429, 0x8529, 0x8428, 0x8528, 0x8428, 0x8528] },
    soundfont_reverb_fx { parms: [0xB488, 0xA458, 0x9558, 0x84B5, 0x383A, 0x3EB5, 0x7284, 0x7254, 0x7224, 0x7224, 0x7254, 0x7284, 0x4448, 0x4548, 0xA440, 0xA540, 0x842A, 0x852A, 0x842A, 0x852A, 0x8429, 0x8529, 0x8429, 0x8529, 0x8428, 0x8528, 0x8428, 0x8528] },
    soundfont_reverb_fx { parms: [0xB488, 0xA460, 0x9560, 0x84B5, 0x383A, 0x3EB5, 0x7284, 0x7254, 0x7224, 0x7224, 0x7254, 0x7284, 0x4416, 0x4516, 0xA490, 0xA590, 0x842C, 0x852C, 0x842C, 0x852C, 0x842B, 0x852B, 0x842B, 0x852B, 0x842A, 0x852A, 0x842A, 0x852A] },
    soundfont_reverb_fx { parms: [0xB488, 0xA470, 0x9570, 0x84B5, 0x383A, 0x3EB5, 0x7284, 0x7254, 0x7224, 0x7224, 0x7254, 0x7284, 0x4448, 0x4548, 0xA440, 0xA540, 0x842B, 0x852B, 0x842B, 0x852B, 0x842A, 0x852A, 0x842A, 0x852A, 0x8429, 0x8529, 0x8429, 0x8529] },
    soundfont_reverb_fx { parms: [0xB488, 0xA470, 0x9570, 0x84B5, 0x383A, 0x3EB5, 0x7254, 0x7234, 0x7224, 0x7254, 0x7264, 0x7294, 0x44C3, 0x45C3, 0xA404, 0xA504, 0x842A, 0x852A, 0x842A, 0x852A, 0x8429, 0x8529, 0x8429, 0x8529, 0x8428, 0x8528, 0x8428, 0x8528] },
    soundfont_reverb_fx { parms: [0xB4FF, 0xA470, 0x9570, 0x84B5, 0x383A, 0x3EB5, 0x7234, 0x7234, 0x7234, 0x7234, 0x7234, 0x7234, 0x4448, 0x4548, 0xA440, 0xA540, 0x842A, 0x852A, 0x842A, 0x852A, 0x8429, 0x8529, 0x8429, 0x8529, 0x8428, 0x8528, 0x8428, 0x8528] },
    soundfont_reverb_fx { parms: [0xB4FF, 0xA470, 0x9500, 0x84B5, 0x333A, 0x39B5, 0x7204, 0x7204, 0x7204, 0x7204, 0x7204, 0x72F4, 0x4400, 0x4500, 0xA4FF, 0xA5FF, 0x8420, 0x8520, 0x8420, 0x8520, 0x8420, 0x8520, 0x8420, 0x8520, 0x8420, 0x8520, 0x8420, 0x8520] },
    soundfont_reverb_fx { parms: [0xB4FF, 0xA490, 0x9590, 0x8474, 0x333A, 0x39B5, 0x7204, 0x7204, 0x7204, 0x7204, 0x7204, 0x72F4, 0x4400, 0x4500, 0xA4FF, 0xA5FF, 0x8420, 0x8520, 0x8420, 0x8520, 0x8420, 0x8520, 0x8420, 0x8520, 0x8420, 0x8520, 0x8420, 0x8520] },
    soundfont_reverb_fx { parms: [0; 28] }, soundfont_reverb_fx { parms: [0; 28] },
    soundfont_reverb_fx { parms: [0; 28] }, soundfont_reverb_fx { parms: [0; 28] },
    soundfont_reverb_fx { parms: [0; 28] }, soundfont_reverb_fx { parms: [0; 28] },
    soundfont_reverb_fx { parms: [0; 28] }, soundfont_reverb_fx { parms: [0; 28] },
    soundfont_reverb_fx { parms: [0; 28] }, soundfont_reverb_fx { parms: [0; 28] },
    soundfont_reverb_fx { parms: [0; 28] }, soundfont_reverb_fx { parms: [0; 28] },
    soundfont_reverb_fx { parms: [0; 28] }, soundfont_reverb_fx { parms: [0; 28] },
    soundfont_reverb_fx { parms: [0; 28] }, soundfont_reverb_fx { parms: [0; 28] },
    soundfont_reverb_fx { parms: [0; 28] }, soundfont_reverb_fx { parms: [0; 28] },
    soundfont_reverb_fx { parms: [0; 28] }, soundfont_reverb_fx { parms: [0; 28] },
    soundfont_reverb_fx { parms: [0; 28] }, soundfont_reverb_fx { parms: [0; 28] },
    soundfont_reverb_fx { parms: [0; 28] }, soundfont_reverb_fx { parms: [0; 28] },
    soundfont_reverb_fx { parms: [0; 28] }, soundfont_reverb_fx { parms: [0; 28] },
];

const DATA1: u16 = 0;
const DATA2: u16 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
struct reverb_cmd_pair {
    cmd: u16,
    port: u16,
}

unsafe fn awe_init1(c: c_int) -> reverb_cmd_pair {
    reverb_cmd_pair { cmd: EMU8000_CMD(2, c) as u16, port: DATA1 }
}
unsafe fn awe_init2(c: c_int) -> reverb_cmd_pair {
    reverb_cmd_pair { cmd: EMU8000_CMD(2, c) as u16, port: DATA2 }
}
unsafe fn awe_init3(c: c_int) -> reverb_cmd_pair {
    reverb_cmd_pair { cmd: EMU8000_CMD(3, c) as u16, port: DATA1 }
}
unsafe fn awe_init4(c: c_int) -> reverb_cmd_pair {
    reverb_cmd_pair { cmd: EMU8000_CMD(3, c) as u16, port: DATA2 }
}

static mut REVERB_CMDS: [reverb_cmd_pair; 28] = [reverb_cmd_pair { cmd: 0, port: 0 }; 28];

unsafe fn init_reverb_cmds_once() {
    REVERB_CMDS = [
        awe_init1(0x03), awe_init1(0x05), awe_init4(0x1F), awe_init1(0x07),
        awe_init2(0x14), awe_init2(0x16), awe_init1(0x0F), awe_init1(0x17),
        awe_init1(0x1F), awe_init2(0x07), awe_init2(0x0F), awe_init2(0x17),
        awe_init2(0x1D), awe_init2(0x1F), awe_init3(0x01), awe_init3(0x03),
        awe_init1(0x09), awe_init1(0x0B), awe_init1(0x11), awe_init1(0x13),
        awe_init1(0x19), awe_init1(0x1B), awe_init2(0x01), awe_init2(0x03),
        awe_init2(0x09), awe_init2(0x0B), awe_init2(0x11), awe_init2(0x13),
    ];
}

/*exported*/
#[no_mangle]
pub unsafe extern "C" fn snd_emu8000_load_reverb_fx(
    emu: *mut snd_emu8000,
    mode: c_int,
    buf: *const c_void,
    len: c_long,
) -> c_int {
    let mut rec = soundfont_reverb_fx { parms: [0; 28] };

    if mode < SNDRV_EMU8000_REVERB_PREDEFINED || mode >= SNDRV_EMU8000_REVERB_NUMBERS {
        dev_warn(
            (*(*emu).card).dev,
            b"invalid reverb mode %d for uploading\n\0".as_ptr() as *const c_char,
            mode,
        );
        return -EINVAL;
    }
    if len < size_of::<soundfont_reverb_fx>() as c_long
        || copy_from_user(
            &mut rec as *mut _ as *mut c_void,
            buf,
            size_of::<soundfont_reverb_fx>(),
        ) != 0
    {
        return -EFAULT;
    }
    REVERB_PARM[mode as usize] = rec;
    REVERB_DEFINED[mode as usize] = 1;
    0
}

/*exported*/
#[no_mangle]
pub unsafe extern "C" fn snd_emu8000_update_reverb_mode(emu: *mut snd_emu8000) {
    let effect: c_int = (*emu).reverb_mode;
    let mut i: c_int;

    if effect < 0
        || effect >= SNDRV_EMU8000_REVERB_NUMBERS
        || (effect >= SNDRV_EMU8000_REVERB_PREDEFINED && REVERB_DEFINED[effect as usize] == 0)
    {
        return;
    }
    init_reverb_cmds_once();
    i = 0;
    while i < 28 {
        let port: c_uint;
        if REVERB_CMDS[i as usize].port == DATA1 {
            port = EMU8000_DATA1(emu);
        } else {
            port = EMU8000_DATA2(emu);
        }
        snd_emu8000_poke(
            emu,
            port,
            REVERB_CMDS[i as usize].cmd as c_uint,
            REVERB_PARM[effect as usize].parms[i as usize] as c_uint,
        );
        i += 1;
    }
}

/*----------------------------------------------------------------
 * mixer interface
 *----------------------------------------------------------------*/

/*
 * bass/treble
 */
unsafe extern "C" fn mixer_bass_treble_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 11;
    0
}

unsafe extern "C" fn mixer_bass_treble_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let emu: *mut snd_emu8000 = snd_kcontrol_chip(kcontrol);

    (*ucontrol).value.integer.value[0] = if (*kcontrol).private_value != 0 {
        (*emu).treble_level as c_long
    } else {
        (*emu).bass_level as c_long
    };
    0
}

unsafe extern "C" fn mixer_bass_treble_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let emu: *mut snd_emu8000 = snd_kcontrol_chip(kcontrol);
    let change: c_int;
    let val1: u16;

    val1 = ((*ucontrol).value.integer.value[0] % 12) as u16;
    {
        let flags = spin_lock_irqsave(&mut (*emu).control_lock as *mut _ as *mut c_void);
        if (*kcontrol).private_value != 0 {
            change = (val1 as c_int != (*emu).treble_level) as c_int;
            (*emu).treble_level = val1 as c_int;
        } else {
            change = (val1 as c_int != (*emu).bass_level) as c_int;
            (*emu).bass_level = val1 as c_int;
        }
        spin_unlock_irqrestore(&mut (*emu).control_lock as *mut _ as *mut c_void, flags);
    }
    snd_emu8000_update_equalizer(emu);
    change
}

static MIXER_BASS_CONTROL: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"Synth Tone Control - Bass\0".as_ptr() as *const c_char,
    info: Some(mixer_bass_treble_info),
    get: Some(mixer_bass_treble_get),
    put: Some(mixer_bass_treble_put),
    private_value: 0,
};

static MIXER_TREBLE_CONTROL: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"Synth Tone Control - Treble\0".as_ptr() as *const c_char,
    info: Some(mixer_bass_treble_info),
    get: Some(mixer_bass_treble_get),
    put: Some(mixer_bass_treble_put),
    private_value: 1,
};

/*
 * chorus/reverb mode
 */
unsafe extern "C" fn mixer_chorus_reverb_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = if (*kcontrol).private_value != 0 {
        (SNDRV_EMU8000_CHORUS_NUMBERS - 1) as c_long
    } else {
        (SNDRV_EMU8000_REVERB_NUMBERS - 1) as c_long
    };
    0
}

unsafe extern "C" fn mixer_chorus_reverb_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let emu: *mut snd_emu8000 = snd_kcontrol_chip(kcontrol);

    (*ucontrol).value.integer.value[0] = if (*kcontrol).private_value != 0 {
        (*emu).chorus_mode as c_long
    } else {
        (*emu).reverb_mode as c_long
    };
    0
}

unsafe extern "C" fn mixer_chorus_reverb_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let emu: *mut snd_emu8000 = snd_kcontrol_chip(kcontrol);
    let change: c_int;
    let val1: u16;

    {
        let flags = spin_lock_irqsave(&mut (*emu).control_lock as *mut _ as *mut c_void);
        if (*kcontrol).private_value != 0 {
            val1 = ((*ucontrol).value.integer.value[0] % SNDRV_EMU8000_CHORUS_NUMBERS as c_long) as u16;
            change = (val1 as c_int != (*emu).chorus_mode) as c_int;
            (*emu).chorus_mode = val1 as c_int;
        } else {
            val1 = ((*ucontrol).value.integer.value[0] % SNDRV_EMU8000_REVERB_NUMBERS as c_long) as u16;
            change = (val1 as c_int != (*emu).reverb_mode) as c_int;
            (*emu).reverb_mode = val1 as c_int;
        }
        spin_unlock_irqrestore(&mut (*emu).control_lock as *mut _ as *mut c_void, flags);
    }
    if change != 0 {
        if (*kcontrol).private_value != 0 {
            snd_emu8000_update_chorus_mode(emu);
        } else {
            snd_emu8000_update_reverb_mode(emu);
        }
    }
    change
}

static MIXER_CHORUS_MODE_CONTROL: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"Chorus Mode\0".as_ptr() as *const c_char,
    info: Some(mixer_chorus_reverb_info),
    get: Some(mixer_chorus_reverb_get),
    put: Some(mixer_chorus_reverb_put),
    private_value: 1,
};

static MIXER_REVERB_MODE_CONTROL: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"Reverb Mode\0".as_ptr() as *const c_char,
    info: Some(mixer_chorus_reverb_info),
    get: Some(mixer_chorus_reverb_get),
    put: Some(mixer_chorus_reverb_put),
    private_value: 0,
};

/*
 * FM OPL3 chorus/reverb depth
 */
unsafe extern "C" fn mixer_fm_depth_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 255;
    0
}

unsafe extern "C" fn mixer_fm_depth_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let emu: *mut snd_emu8000 = snd_kcontrol_chip(kcontrol);

    (*ucontrol).value.integer.value[0] = if (*kcontrol).private_value != 0 {
        (*emu).fm_chorus_depth as c_long
    } else {
        (*emu).fm_reverb_depth as c_long
    };
    0
}

unsafe extern "C" fn mixer_fm_depth_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let emu: *mut snd_emu8000 = snd_kcontrol_chip(kcontrol);
    let change: c_int;
    let val1: u16;

    val1 = ((*ucontrol).value.integer.value[0] % 256) as u16;
    {
        let flags = spin_lock_irqsave(&mut (*emu).control_lock as *mut _ as *mut c_void);
        if (*kcontrol).private_value != 0 {
            change = (val1 as c_uint != (*emu).fm_chorus_depth) as c_int;
            (*emu).fm_chorus_depth = val1 as c_uint;
        } else {
            change = (val1 as c_uint != (*emu).fm_reverb_depth) as c_int;
            (*emu).fm_reverb_depth = val1 as c_uint;
        }
        spin_unlock_irqrestore(&mut (*emu).control_lock as *mut _ as *mut c_void, flags);
    }
    if change != 0 {
        snd_emu8000_init_fm(emu);
    }
    change
}

static MIXER_FM_CHORUS_DEPTH_CONTROL: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"FM Chorus Depth\0".as_ptr() as *const c_char,
    info: Some(mixer_fm_depth_info),
    get: Some(mixer_fm_depth_get),
    put: Some(mixer_fm_depth_put),
    private_value: 1,
};

static MIXER_FM_REVERB_DEPTH_CONTROL: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"FM Reverb Depth\0".as_ptr() as *const c_char,
    info: Some(mixer_fm_depth_info),
    get: Some(mixer_fm_depth_get),
    put: Some(mixer_fm_depth_put),
    private_value: 0,
};

static MIXER_DEFS: [*const snd_kcontrol_new; EMU8000_NUM_CONTROLS] = [
    &MIXER_BASS_CONTROL,
    &MIXER_TREBLE_CONTROL,
    &MIXER_CHORUS_MODE_CONTROL,
    &MIXER_REVERB_MODE_CONTROL,
    &MIXER_FM_CHORUS_DEPTH_CONTROL,
    &MIXER_FM_REVERB_DEPTH_CONTROL,
];

/*
 * create and attach mixer elements for WaveTable treble/bass controls
 */
unsafe extern "C" fn snd_emu8000_create_mixer(
    card: *mut snd_card,
    emu: *mut snd_emu8000,
) -> c_int {
    let mut kctl: *mut snd_kcontrol;
    let mut i: c_int;
    let mut err: c_int = 0;

    if snd_BUG_ON(emu.is_null() || card.is_null()) {
        return -EINVAL;
    }

    spin_lock_init(&mut (*emu).control_lock as *mut _ as *mut c_void);

    memset(
        (*emu).controls.as_mut_ptr() as *mut c_void,
        0,
        size_of::<[*mut snd_kcontrol; EMU8000_NUM_CONTROLS]>(),
    );
    i = 0;
    while i < EMU8000_NUM_CONTROLS as c_int {
        kctl = snd_ctl_new1(MIXER_DEFS[i as usize], emu as *mut c_void);
        err = snd_ctl_add(card, kctl);
        if err < 0 {
            while i < EMU8000_NUM_CONTROLS as c_int {
                snd_ctl_remove(card, (*emu).controls[i as usize]);
                i += 1;
            }
            return err;
        }
        (*emu).controls[i as usize] = kctl;
        i += 1;
    }
    0
}

/*
 * initialize and register emu8000 synth device.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_emu8000_new(
    card: *mut snd_card,
    index: c_int,
    port: c_long,
    seq_ports: c_int,
    awe_ret: *mut *mut snd_seq_device,
) -> c_int {
    let mut awe: *mut snd_seq_device;
    let hw: *mut snd_emu8000;
    let err: c_int;

    if !awe_ret.is_null() {
        *awe_ret = ptr::null_mut();
    }

    if seq_ports <= 0 {
        return 0;
    }

    hw = devm_kzalloc((*card).dev, size_of::<snd_emu8000>(), GFP_KERNEL) as *mut snd_emu8000;
    if hw.is_null() {
        return -ENOMEM;
    }
    spin_lock_init(&mut (*hw).reg_lock as *mut _ as *mut c_void);
    (*hw).index = index;
    (*hw).port1 = port as c_ulong;
    (*hw).port2 = (port + 0x400) as c_ulong;
    (*hw).port3 = (port + 0x800) as c_ulong;
    if devm_request_region((*card).dev, (*hw).port1, 4, b"Emu8000-1\0".as_ptr() as *const c_char).is_null()
        || devm_request_region((*card).dev, (*hw).port2, 4, b"Emu8000-2\0".as_ptr() as *const c_char).is_null()
        || devm_request_region((*card).dev, (*hw).port3, 4, b"Emu8000-3\0".as_ptr() as *const c_char).is_null()
    {
        dev_err(
            (*card).dev,
            b"sbawe: can't grab ports 0x%lx, 0x%lx, 0x%lx\n\0".as_ptr() as *const c_char,
            (*hw).port1,
            (*hw).port2,
            (*hw).port3,
        );
        return -EBUSY;
    }
    (*hw).mem_size = 0;
    (*hw).card = card;
    (*hw).seq_ports = seq_ports;
    (*hw).bass_level = 5;
    (*hw).treble_level = 9;
    (*hw).chorus_mode = 2;
    (*hw).reverb_mode = 4;
    (*hw).fm_chorus_depth = 0;
    (*hw).fm_reverb_depth = 0;

    if snd_emu8000_detect(hw) < 0 {
        return -ENODEV;
    }

    snd_emu8000_init_hw(hw);
    err = snd_emu8000_create_mixer(card, hw);
    if err < 0 {
        return err;
    }
    /*
     * Original conditional:
     * #if IS_ENABLED(CONFIG_SND_SEQUENCER)
     */
    awe = ptr::null_mut();
    if snd_seq_device_new(
        card,
        index,
        SNDRV_SEQ_DEV_ID_EMU8000,
        size_of::<*mut snd_emu8000>() as c_int,
        &mut awe,
    ) >= 0
    {
        strscpy((*awe).name.as_mut_ptr(), b"EMU-8000\0".as_ptr() as *const c_char);
        *(SNDRV_SEQ_DEVICE_ARGPTR(awe) as *mut *mut snd_emu8000) = hw;
    }
    /*
     * #else
     * awe = NULL;
     * #endif
     */
    if !awe_ret.is_null() {
        *awe_ret = awe;
    }

    0
}

/*
 * exported stuff
 *
 * EXPORT_SYMBOL(snd_emu8000_poke);
 * EXPORT_SYMBOL(snd_emu8000_peek);
 * EXPORT_SYMBOL(snd_emu8000_poke_dw);
 * EXPORT_SYMBOL(snd_emu8000_peek_dw);
 * EXPORT_SYMBOL(snd_emu8000_dma_chan);
 * EXPORT_SYMBOL(snd_emu8000_init_fm);
 * EXPORT_SYMBOL(snd_emu8000_load_chorus_fx);
 * EXPORT_SYMBOL(snd_emu8000_load_reverb_fx);
 * EXPORT_SYMBOL(snd_emu8000_update_chorus_mode);
 * EXPORT_SYMBOL(snd_emu8000_update_reverb_mode);
 * EXPORT_SYMBOL(snd_emu8000_update_equalizer);
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
