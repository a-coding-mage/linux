// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  Routines for control of ESS ES1688/688/488 chip
 */

/* Translated from Linux kernel C. Original includes:
 * linux/init.h, linux/interrupt.h, linux/delay.h, linux/slab.h,
 * linux/ioport.h, linux/module.h, linux/io.h, sound/core.h,
 * sound/es1688.h, sound/initval.h, asm/dma.h.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type size_t = usize;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_int;

extern "C" {
    fn inb(port: c_ulong) -> u8;
    fn outb(value: u8, port: c_ulong);
    fn udelay(usecs: c_uint);
    fn request_irq(irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, dev: *mut c_void) -> c_int;
    fn free_irq(irq: c_int, dev: *mut c_void);
    fn request_dma(dma: c_int, name: *const c_char) -> c_int;
    fn free_dma(dma: c_int);
    fn disable_dma(dma: c_int);
    fn request_region(start: c_ulong, n: c_ulong, name: *const c_char) -> *mut resource;
    fn release_and_free_resource(res: *mut resource);
    fn snd_dma_program(dma: c_int, addr: c_ulong, size: c_uint, mode: c_int);
    fn snd_dma_pointer(dma: c_int, size: c_uint) -> size_t;
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut snd_es1688;
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: size_t) -> snd_pcm_uframes_t;
    fn snd_pcm_hw_constraint_ratnums(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_uint, r: *const snd_pcm_hw_constraint_ratnums) -> c_int;
    fn snd_device_new(card: *mut snd_card, ty: c_int, data: *mut c_void, ops: *const snd_device_ops) -> c_int;
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback_count: c_int, capture_count: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, stream: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, ty: c_int, dev: *mut device, min: size_t, max: size_t);
    fn strscpy(dst: *mut c_char, src: *const c_char) -> size_t;
    fn snd_ctl_enum_info(uinfo: *mut snd_ctl_elem_info, channels: c_uint, items: c_uint, texts: *const *const c_char) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_es1688;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn snd_BUG_ON(cond: bool) -> bool;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn spin_lock_init(lock: *mut spinlock_t);
}

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct resource { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { pub private_value: c_ulong }
#[repr(C)] pub struct snd_device { pub device_data: *mut c_void }

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub sync_irq: c_int,
    pub mixername: *mut c_char,
}

#[repr(C)]
pub struct snd_es1688 {
    pub card: *mut snd_card,
    pub irq: c_int,
    pub dma8: c_int,
    pub hardware: c_ushort,
    pub res_port: *mut resource,
    pub reg_lock: spinlock_t,
    pub mixer_lock: spinlock_t,
    pub port: c_ulong,
    pub mpu_port: c_ulong,
    pub mpu_irq: c_int,
    pub version: c_uint,
    pub trigger_value: u8,
    pub dma_size: c_uint,
    pub playback_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
    pub pcm: *mut snd_pcm,
}

type c_ushort = u16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ratnum {
    pub num: c_uint,
    pub den_min: c_uint,
    pub den_max: c_uint,
    pub den_step: c_uint,
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_ratnums {
    pub nrats: c_uint,
    pub rats: *const snd_ratnum,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: c_uint,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub buffer_bytes_max: c_uint,
    pub period_bytes_min: c_uint,
    pub period_bytes_max: c_uint,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub fifo_size: c_uint,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub rate_num: c_uint,
    pub rate_den: c_uint,
    pub rate: c_uint,
    pub channels: c_uint,
    pub format: c_int,
    pub dma_addr: c_ulong,
    pub hw: snd_pcm_hardware,
}

#[repr(C)] pub struct snd_pcm_substream { pub runtime: *mut snd_pcm_runtime }
#[repr(C)] pub struct snd_pcm { pub private_data: *mut c_void, pub info_flags: c_uint, pub name: *mut c_char }

#[repr(C)]
pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}

#[repr(C)]
pub struct snd_device_ops {
    pub dev_free: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_int,
    pub name: *const c_char,
    pub index: c_uint,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: c_ulong,
}

#[repr(C)] pub struct snd_ctl_elem_info { pub type_: c_int, pub count: c_uint, pub value: snd_ctl_elem_info_value }
#[repr(C)] pub struct snd_ctl_elem_info_value { pub integer: snd_ctl_elem_info_integer }
#[repr(C)] pub struct snd_ctl_elem_info_integer { pub min: c_long, pub max: c_long }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_value }
#[repr(C)] pub struct snd_ctl_elem_value_value { pub integer: snd_ctl_elem_value_integer, pub enumerated: snd_ctl_elem_value_enumerated }
#[repr(C)] pub struct snd_ctl_elem_value_integer { pub value: [c_long; 2] }
#[repr(C)] pub struct snd_ctl_elem_value_enumerated { pub item: [c_uint; 2] }
type c_long = isize;

const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const EAGAIN: c_int = 11;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;
const IRQ_HANDLED: irqreturn_t = 1;

const SNDRV_PCM_TRIGGER_STOP: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 1;
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 0;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 1;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 2;
const SNDRV_PCM_FMTBIT_U8: c_uint = 1 << 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 1 << 1;
const SNDRV_PCM_RATE_CONTINUOUS: c_uint = 1 << 0;
const SNDRV_PCM_RATE_8000_48000: c_uint = 1 << 1;
const SNDRV_PCM_HW_PARAM_RATE: c_uint = 0;
const SNDRV_DEV_LOWLEVEL: c_int = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_INFO_HALF_DUPLEX: c_uint = 1 << 3;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_int = 0;
const SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_int = 1;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_int = 2;
const DMA_MODE_WRITE: c_int = 0x48;
const DMA_MODE_READ: c_int = 0x44;
const DMA_AUTOINIT: c_int = 0x10;

const ES1688_HW_UNDEF: c_ushort = 0xffff;
const ES1688_HW_688: c_ushort = 0x0688;
const ES1688_DSP_CMD_SPKON: u8 = 0xd1;
const ES1688_DSP_CMD_SPKOFF: u8 = 0xd3;

const STATUS: c_ulong = 0x0c;
const COMMAND: c_ulong = 0x0c;
const DATA_AVAIL: c_ulong = 0x0e;
const READ: c_ulong = 0x0a;
const MIXER_ADDR: c_ulong = 0x04;
const MIXER_DATA: c_ulong = 0x05;
const RESET: c_ulong = 0x06;
const ENABLE0: c_ulong = 0x00;
const ENABLE1: c_ulong = 0x01;
const ENABLE2: c_ulong = 0x02;

const ES1688_MASTER_DEV: u8 = 0x32;
const ES1688_PCM_DEV: u8 = 0x14;
const ES1688_LINE_DEV: u8 = 0x3e;
const ES1688_CD_DEV: u8 = 0x38;
const ES1688_FM_DEV: u8 = 0x36;
const ES1688_MIC_DEV: u8 = 0x1a;
const ES1688_AUX_DEV: u8 = 0x3a;
const ES1688_SPEAKER_DEV: u8 = 0x3c;
const ES1688_RECLEV_DEV: u8 = 0xb4;
const ES1688_REC_DEV: u8 = 0x1c;

#[inline]
unsafe fn ES1688P(chip: *mut snd_es1688, x: c_ulong) -> c_ulong {
    (*chip).port.wrapping_add(x)
}

unsafe fn snd_es1688_dsp_command(chip: *mut snd_es1688, val: u8) -> c_int {
    let mut i: c_int = 10000;
    while i != 0 {
        if (inb(ES1688P(chip, STATUS)) & 0x80) == 0 {
            outb(val, ES1688P(chip, COMMAND));
            return 1;
        }
        i -= 1;
    }
    dev_dbg((*(*chip).card).dev, b"%s: timeout (0x%x)\n\0".as_ptr() as *const c_char, b"snd_es1688_dsp_command\0".as_ptr(), val as c_int);
    0
}

unsafe fn snd_es1688_dsp_get_byte(chip: *mut snd_es1688) -> c_int {
    let mut i: c_int = 1000;
    while i != 0 {
        if (inb(ES1688P(chip, DATA_AVAIL)) & 0x80) != 0 {
            return inb(ES1688P(chip, READ)) as c_int;
        }
        i -= 1;
    }
    dev_dbg((*(*chip).card).dev, b"es1688 get byte failed: 0x%lx = 0x%x!!!\n\0".as_ptr() as *const c_char,
            ES1688P(chip, DATA_AVAIL), inb(ES1688P(chip, DATA_AVAIL)) as c_int);
    -ENODEV
}

unsafe fn snd_es1688_write(chip: *mut snd_es1688, reg: u8, data: u8) -> c_int {
    if snd_es1688_dsp_command(chip, reg) == 0 {
        return 0;
    }
    snd_es1688_dsp_command(chip, data)
}

unsafe fn snd_es1688_read(chip: *mut snd_es1688, reg: u8) -> c_int {
    /* Read a byte from an extended mode register of ES1688 */
    if snd_es1688_dsp_command(chip, 0xc0) == 0 {
        return -1;
    }
    if snd_es1688_dsp_command(chip, reg) == 0 {
        return -1;
    }
    snd_es1688_dsp_get_byte(chip)
}

#[no_mangle]
pub unsafe extern "C" fn snd_es1688_mixer_write(chip: *mut snd_es1688, reg: u8, data: u8) {
    outb(reg, ES1688P(chip, MIXER_ADDR));
    udelay(10);
    outb(data, ES1688P(chip, MIXER_DATA));
    udelay(10);
}

unsafe fn snd_es1688_mixer_read(chip: *mut snd_es1688, reg: u8) -> u8 {
    let result: u8;
    outb(reg, ES1688P(chip, MIXER_ADDR));
    udelay(10);
    result = inb(ES1688P(chip, MIXER_DATA));
    udelay(10);
    result
}

#[no_mangle]
pub unsafe extern "C" fn snd_es1688_reset(chip: *mut snd_es1688) -> c_int {
    outb(3, ES1688P(chip, RESET)); /* valid only for ESS chips, SB -> 1 */
    udelay(10);
    outb(0, ES1688P(chip, RESET));
    udelay(30);
    let mut i: c_int = 0;
    while i < 1000 && (inb(ES1688P(chip, DATA_AVAIL)) & 0x80) == 0 {
        i += 1;
    }
    if inb(ES1688P(chip, READ)) != 0xaa {
        dev_dbg((*(*chip).card).dev, b"ess_reset at 0x%lx: failed!!!\n\0".as_ptr() as *const c_char, (*chip).port);
        return -ENODEV;
    }
    snd_es1688_dsp_command(chip, 0xc6); /* enable extended mode */
    0
}

unsafe fn snd_es1688_probe(chip: *mut snd_es1688) -> c_int {
    let mut major: u16 = 0;
    let mut minor: u16 = 0;
    let mut i: c_int;

    /*
     *  initialization sequence
     */
    inb(ES1688P(chip, ENABLE1)); /* ENABLE1 */
    inb(ES1688P(chip, ENABLE1)); /* ENABLE1 */
    inb(ES1688P(chip, ENABLE1)); /* ENABLE1 */
    inb(ES1688P(chip, ENABLE2)); /* ENABLE2 */
    inb(ES1688P(chip, ENABLE1)); /* ENABLE1 */
    inb(ES1688P(chip, ENABLE2)); /* ENABLE2 */
    inb(ES1688P(chip, ENABLE1)); /* ENABLE1 */
    inb(ES1688P(chip, ENABLE1)); /* ENABLE1 */
    inb(ES1688P(chip, ENABLE2)); /* ENABLE2 */
    inb(ES1688P(chip, ENABLE1)); /* ENABLE1 */
    inb(ES1688P(chip, ENABLE0)); /* ENABLE0 */

    if snd_es1688_reset(chip) < 0 {
        dev_dbg((*(*chip).card).dev, b"ESS: [0x%lx] reset failed... 0x%x\n\0".as_ptr() as *const c_char,
                (*chip).port, inb(ES1688P(chip, READ)) as c_int);
        return -ENODEV;
    }
    snd_es1688_dsp_command(chip, 0xe7); /* return identification */

    i = 1000;
    while i != 0 {
        if (inb(ES1688P(chip, DATA_AVAIL)) & 0x80) != 0 {
            if major == 0 {
                major = inb(ES1688P(chip, READ)) as u16;
            } else {
                minor = inb(ES1688P(chip, READ)) as u16;
            }
        }
        i -= 1;
    }

    dev_dbg((*(*chip).card).dev, b"ESS: [0x%lx] found.. major = 0x%x, minor = 0x%x\n\0".as_ptr() as *const c_char,
            (*chip).port, major as c_int, minor as c_int);
    (*chip).version = (((major as c_uint) << 8) | minor as c_uint) as c_uint;
    if (*chip).version == 0 {
        return -ENODEV; /* probably SB */
    }

    match (*chip).version & 0xfff0 {
        0x4880 => {
            dev_err((*(*chip).card).dev, b"[0x%lx] ESS: AudioDrive ES488 detected, but driver is in another place\n\0".as_ptr() as *const c_char, (*chip).port);
            return -ENODEV;
        }
        0x6880 => {}
        _ => {
            dev_err((*(*chip).card).dev, b"[0x%lx] ESS: unknown AudioDrive chip with version 0x%x (Jazz16 soundcard?)\n\0".as_ptr() as *const c_char,
                    (*chip).port, (*chip).version);
            return -ENODEV;
        }
    }

    snd_es1688_write(chip, 0xb1, 0x10); /* disable IRQ */
    snd_es1688_write(chip, 0xb2, 0x00); /* disable DMA */

    /* enable joystick, but disable OPL3 */
    snd_es1688_mixer_write(chip, 0x40, 0x01);
    0
}

unsafe fn snd_es1688_init(chip: *mut snd_es1688, enable: c_int) -> c_int {
    static IRQS: [c_int; 16] = [-1, -1, 0, -1, -1, 1, -1, 2, -1, 0, 3, -1, -1, -1, -1, -1];
    let mut cfg: c_int;
    let irq_bits: c_int;
    let dma: c_int;
    let mut dma_bits: c_int;
    let mut tmp: c_int;
    let mut tmp1: c_int;

    /* ok.. setup MPU-401 port and joystick and OPL3 */
    cfg = 0x01; /* enable joystick, but disable OPL3 */
    if enable != 0 && (*chip).mpu_port >= 0x300 && (*chip).mpu_irq > 0 && (*chip).hardware != ES1688_HW_688 {
        tmp = (((*chip).mpu_port & 0x0f0) >> 4) as c_int;
        if tmp <= 3 {
            tmp1 = match (*chip).mpu_irq {
                9 => 4,
                5 => 5,
                7 => 6,
                10 => 7,
                _ => 0,
            };
            if tmp1 != 0 {
                cfg |= (tmp << 3) | (tmp1 << 5);
            }
        }
    }
    snd_es1688_mixer_write(chip, 0x40, cfg as u8);
    /* --- */
    snd_es1688_read(chip, 0xb1);
    snd_es1688_read(chip, 0xb2);
    if enable != 0 {
        cfg = 0xf0; /* enable only DMA counter interrupt */
        irq_bits = IRQS[((*chip).irq & 0x0f) as usize];
        if irq_bits < 0 {
            dev_err((*(*chip).card).dev, b"[0x%lx] ESS: bad IRQ %d for ES1688 chip!!\n\0".as_ptr() as *const c_char,
                    (*chip).port, (*chip).irq);
            /* #if 0: irq_bits = 0; cfg = 0x10; */
            return -EINVAL;
        }
        snd_es1688_write(chip, 0xb1, (cfg | (irq_bits << 2)) as u8);
        cfg = 0xf0; /* extended mode DMA enable */
        dma = (*chip).dma8;
        if dma > 3 || dma == 2 {
            dev_err((*(*chip).card).dev, b"[0x%lx] ESS: bad DMA channel %d for ES1688 chip!!\n\0".as_ptr() as *const c_char,
                    (*chip).port, dma);
            /* #if 0: dma_bits = 0; cfg = 0x00; disable all DMA */
            return -EINVAL;
        } else {
            dma_bits = dma;
            if dma != 3 {
                dma_bits += 1;
            }
        }
        snd_es1688_write(chip, 0xb2, (cfg | (dma_bits << 2)) as u8);
    } else {
        snd_es1688_write(chip, 0xb1, 0x10); /* disable IRQ */
        snd_es1688_write(chip, 0xb2, 0x00); /* disable DMA */
    }
    snd_es1688_read(chip, 0xb1);
    snd_es1688_read(chip, 0xb2);
    snd_es1688_reset(chip);
    0
}

/*

 */

static CLOCKS: [snd_ratnum; 2] = [
    snd_ratnum { num: 795444, den_min: 1, den_max: 128, den_step: 1 },
    snd_ratnum { num: 397722, den_min: 1, den_max: 128, den_step: 1 },
];

static HW_CONSTRAINTS_CLOCKS: snd_pcm_hw_constraint_ratnums = snd_pcm_hw_constraint_ratnums {
    nrats: 2,
    rats: CLOCKS.as_ptr(),
};

unsafe fn snd_es1688_set_rate(chip: *mut snd_es1688, substream: *mut snd_pcm_substream) {
    let runtime = (*substream).runtime;
    let bits: c_uint;
    let divider: c_uint;

    if (*runtime).rate_num == CLOCKS[0].num {
        bits = 256 - (*runtime).rate_den;
    } else {
        bits = 128 - (*runtime).rate_den;
    }
    /* set filter register */
    divider = 256 - 7160000 * 20 / (8 * 82 * (*runtime).rate);
    /* write result to hardware */
    snd_es1688_write(chip, 0xa1, bits as u8);
    snd_es1688_write(chip, 0xa2, divider as u8);
}

unsafe fn snd_es1688_trigger(chip: *mut snd_es1688, cmd: c_int, mut value: u8) -> c_int {
    let val: c_int;

    if cmd == SNDRV_PCM_TRIGGER_STOP {
        value = 0x00;
    } else if cmd != SNDRV_PCM_TRIGGER_START {
        return -EINVAL;
    }
    (*chip).trigger_value = value;
    val = snd_es1688_read(chip, 0xb8);
    if val < 0 || (val & 0x0f) == value as c_int {
        return -EINVAL; /* something is wrong */
    }
    /* #if 0: debug trigger value and DMA pointer */
    snd_es1688_write(chip, 0xb8, ((val & 0xf0) | value as c_int) as u8);
    0
}

unsafe extern "C" fn snd_es1688_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let size = snd_pcm_lib_buffer_bytes(substream);
    let mut count = snd_pcm_lib_period_bytes(substream);

    (*chip).dma_size = size;
    snd_es1688_reset(chip);
    snd_es1688_set_rate(chip, substream);
    snd_es1688_write(chip, 0xb8, 4); /* auto init DMA mode */
    snd_es1688_write(chip, 0xa8, ((snd_es1688_read(chip, 0xa8) & !0x03) | (3 - (*runtime).channels as c_int)) as u8);
    snd_es1688_write(chip, 0xb9, 2); /* demand mode (4 bytes/request) */
    if (*runtime).channels == 1 {
        if snd_pcm_format_width((*runtime).format) == 8 {
            /* 8. bit mono */
            snd_es1688_write(chip, 0xb6, 0x80);
            snd_es1688_write(chip, 0xb7, 0x51);
            snd_es1688_write(chip, 0xb7, 0xd0);
        } else {
            /* 16. bit mono */
            snd_es1688_write(chip, 0xb6, 0x00);
            snd_es1688_write(chip, 0xb7, 0x71);
            snd_es1688_write(chip, 0xb7, 0xf4);
        }
    } else if snd_pcm_format_width((*runtime).format) == 8 {
        /* 8. bit stereo */
        snd_es1688_write(chip, 0xb6, 0x80);
        snd_es1688_write(chip, 0xb7, 0x51);
        snd_es1688_write(chip, 0xb7, 0x98);
    } else {
        /* 16. bit stereo */
        snd_es1688_write(chip, 0xb6, 0x00);
        snd_es1688_write(chip, 0xb7, 0x71);
        snd_es1688_write(chip, 0xb7, 0xbc);
    }
    snd_es1688_write(chip, 0xb1, ((snd_es1688_read(chip, 0xb1) & 0x0f) | 0x50) as u8);
    snd_es1688_write(chip, 0xb2, ((snd_es1688_read(chip, 0xb2) & 0x0f) | 0x50) as u8);
    snd_es1688_dsp_command(chip, ES1688_DSP_CMD_SPKON);
    /* --- */
    count = (0u32).wrapping_sub(count);
    snd_dma_program((*chip).dma8, (*runtime).dma_addr, size, DMA_MODE_WRITE | DMA_AUTOINIT);
    snd_es1688_write(chip, 0xa4, count as u8);
    snd_es1688_write(chip, 0xa5, (count >> 8) as u8);
    0
}

unsafe extern "C" fn snd_es1688_playback_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    snd_es1688_trigger(chip, cmd, 0x05)
}

unsafe extern "C" fn snd_es1688_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let size = snd_pcm_lib_buffer_bytes(substream);
    let mut count = snd_pcm_lib_period_bytes(substream);

    (*chip).dma_size = size;
    snd_es1688_reset(chip);
    snd_es1688_set_rate(chip, substream);
    snd_es1688_dsp_command(chip, ES1688_DSP_CMD_SPKOFF);
    snd_es1688_write(chip, 0xb8, 0x0e); /* auto init DMA mode */
    snd_es1688_write(chip, 0xa8, ((snd_es1688_read(chip, 0xa8) & !0x03) | (3 - (*runtime).channels as c_int)) as u8);
    snd_es1688_write(chip, 0xb9, 2); /* demand mode (4 bytes/request) */
    if (*runtime).channels == 1 {
        if snd_pcm_format_width((*runtime).format) == 8 {
            /* 8. bit mono */
            snd_es1688_write(chip, 0xb7, 0x51);
            snd_es1688_write(chip, 0xb7, 0xd0);
        } else {
            /* 16. bit mono */
            snd_es1688_write(chip, 0xb7, 0x71);
            snd_es1688_write(chip, 0xb7, 0xf4);
        }
    } else if snd_pcm_format_width((*runtime).format) == 8 {
        /* 8. bit stereo */
        snd_es1688_write(chip, 0xb7, 0x51);
        snd_es1688_write(chip, 0xb7, 0x98);
    } else {
        /* 16. bit stereo */
        snd_es1688_write(chip, 0xb7, 0x71);
        snd_es1688_write(chip, 0xb7, 0xbc);
    }
    snd_es1688_write(chip, 0xb1, ((snd_es1688_read(chip, 0xb1) & 0x0f) | 0x50) as u8);
    snd_es1688_write(chip, 0xb2, ((snd_es1688_read(chip, 0xb2) & 0x0f) | 0x50) as u8);
    /* --- */
    count = (0u32).wrapping_sub(count);
    snd_dma_program((*chip).dma8, (*runtime).dma_addr, size, DMA_MODE_READ | DMA_AUTOINIT);
    snd_es1688_write(chip, 0xa4, count as u8);
    snd_es1688_write(chip, 0xa5, (count >> 8) as u8);
    0
}

unsafe extern "C" fn snd_es1688_capture_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    snd_es1688_trigger(chip, cmd, 0x0f)
}

unsafe extern "C" fn snd_es1688_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let chip = dev_id as *mut snd_es1688;

    if (*chip).trigger_value == 0x05 { /* ok.. playback is active */
        snd_pcm_period_elapsed((*chip).playback_substream);
    }
    if (*chip).trigger_value == 0x0f { /* ok.. capture is active */
        snd_pcm_period_elapsed((*chip).capture_substream);
    }

    inb(ES1688P(chip, DATA_AVAIL)); /* ack interrupt */
    IRQ_HANDLED
}

unsafe extern "C" fn snd_es1688_playback_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream);
    let ptr: size_t;

    if (*chip).trigger_value != 0x05 {
        return 0;
    }
    ptr = snd_dma_pointer((*chip).dma8, (*chip).dma_size);
    bytes_to_frames((*substream).runtime, ptr)
}

unsafe extern "C" fn snd_es1688_capture_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream);
    let ptr: size_t;

    if (*chip).trigger_value != 0x0f {
        return 0;
    }
    ptr = snd_dma_pointer((*chip).dma8, (*chip).dma_size);
    bytes_to_frames((*substream).runtime, ptr)
}

/*

 */

static SND_ES1688_PLAYBACK: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,
    rate_min: 4000,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 65536,
    period_bytes_min: 64,
    period_bytes_max: 65536,
    periods_min: 1,
    periods_max: 1024,
    fifo_size: 0,
};

static SND_ES1688_CAPTURE: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,
    rate_min: 4000,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 65536,
    period_bytes_min: 64,
    period_bytes_max: 65536,
    periods_min: 1,
    periods_max: 1024,
    fifo_size: 0,
};

/*

 */

unsafe extern "C" fn snd_es1688_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;

    if !(*chip).capture_substream.is_null() {
        return -EAGAIN;
    }
    (*chip).playback_substream = substream;
    (*runtime).hw = SND_ES1688_PLAYBACK;
    snd_pcm_hw_constraint_ratnums(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &HW_CONSTRAINTS_CLOCKS);
    0
}

unsafe extern "C" fn snd_es1688_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;

    if !(*chip).playback_substream.is_null() {
        return -EAGAIN;
    }
    (*chip).capture_substream = substream;
    (*runtime).hw = SND_ES1688_CAPTURE;
    snd_pcm_hw_constraint_ratnums(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &HW_CONSTRAINTS_CLOCKS);
    0
}

unsafe extern "C" fn snd_es1688_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    (*chip).playback_substream = core::ptr::null_mut();
    0
}

unsafe extern "C" fn snd_es1688_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    (*chip).capture_substream = core::ptr::null_mut();
    0
}

unsafe fn snd_es1688_free(chip: *mut snd_es1688) -> c_int {
    if (*chip).hardware != ES1688_HW_UNDEF {
        snd_es1688_init(chip, 0);
    }
    release_and_free_resource((*chip).res_port);
    if (*chip).irq >= 0 {
        free_irq((*chip).irq, chip as *mut c_void);
    }
    if (*chip).dma8 >= 0 {
        disable_dma((*chip).dma8);
        free_dma((*chip).dma8);
    }
    0
}

unsafe extern "C" fn snd_es1688_dev_free(device: *mut snd_device) -> c_int {
    let chip = (*device).device_data as *mut snd_es1688;
    snd_es1688_free(chip)
}

unsafe fn snd_es1688_chip_id(chip: *mut snd_es1688) -> *const c_char {
    static mut TMP: [c_char; 16] = [0; 16];
    sprintf(TMP.as_mut_ptr(), b"ES%s688 rev %i\0".as_ptr() as *const c_char,
            if (*chip).hardware == ES1688_HW_688 { b"\0".as_ptr() } else { b"1\0".as_ptr() },
            ((*chip).version & 0x0f) as c_int);
    TMP.as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn snd_es1688_create(card: *mut snd_card, chip: *mut snd_es1688, port: c_ulong, mut mpu_port: c_ulong, irq: c_int, mpu_irq: c_int, dma8: c_int, hardware: c_ushort) -> c_int {
    static OPS: snd_device_ops = snd_device_ops { dev_free: Some(snd_es1688_dev_free) };
    let mut err: c_int;

    if chip.is_null() {
        return -ENOMEM;
    }
    (*chip).card = card;
    (*chip).irq = -1;
    (*chip).dma8 = -1;
    (*chip).hardware = ES1688_HW_UNDEF;

    (*chip).res_port = request_region(port + 4, 12, b"ES1688\0".as_ptr() as *const c_char);
    if (*chip).res_port.is_null() {
        dev_err((*card).dev, b"es1688: can't grab port 0x%lx\n\0".as_ptr() as *const c_char, port + 4);
        err = -EBUSY;
        return { snd_es1688_free(chip); err };
    }

    err = request_irq(irq, snd_es1688_interrupt, 0, b"ES1688\0".as_ptr() as *const c_char, chip as *mut c_void);
    if err < 0 {
        dev_err((*card).dev, b"es1688: can't grab IRQ %d\n\0".as_ptr() as *const c_char, irq);
        return { snd_es1688_free(chip); err };
    }

    (*chip).irq = irq;
    (*card).sync_irq = (*chip).irq;
    err = request_dma(dma8, b"ES1688\0".as_ptr() as *const c_char);
    if err < 0 {
        dev_err((*card).dev, b"es1688: can't grab DMA8 %d\n\0".as_ptr() as *const c_char, dma8);
        return { snd_es1688_free(chip); err };
    }
    (*chip).dma8 = dma8;

    spin_lock_init(&mut (*chip).reg_lock);
    spin_lock_init(&mut (*chip).mixer_lock);
    (*chip).port = port;
    mpu_port &= !0x000f;
    if mpu_port < 0x300 || mpu_port > 0x330 {
        mpu_port = 0;
    }
    (*chip).mpu_port = mpu_port;
    (*chip).mpu_irq = mpu_irq;
    (*chip).hardware = hardware;

    err = snd_es1688_probe(chip);
    if err < 0 {
        return { snd_es1688_free(chip); err };
    }

    err = snd_es1688_init(chip, 1);
    if err < 0 {
        return { snd_es1688_free(chip); err };
    }

    /* Register device */
    err = snd_device_new(card, SNDRV_DEV_LOWLEVEL, chip as *mut c_void, &OPS);
    if err != 0 {
        snd_es1688_free(chip);
    }
    err
}

static SND_ES1688_PLAYBACK_OPS: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_es1688_playback_open),
    close: Some(snd_es1688_playback_close),
    prepare: Some(snd_es1688_playback_prepare),
    trigger: Some(snd_es1688_playback_trigger),
    pointer: Some(snd_es1688_playback_pointer),
};

static SND_ES1688_CAPTURE_OPS: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_es1688_capture_open),
    close: Some(snd_es1688_capture_close),
    prepare: Some(snd_es1688_capture_prepare),
    trigger: Some(snd_es1688_capture_trigger),
    pointer: Some(snd_es1688_capture_pointer),
};

#[no_mangle]
pub unsafe extern "C" fn snd_es1688_pcm(card: *mut snd_card, chip: *mut snd_es1688, device: c_int) -> c_int {
    let mut pcm: *mut snd_pcm = core::ptr::null_mut();
    let mut err: c_int;

    err = snd_pcm_new(card, b"ESx688\0".as_ptr() as *const c_char, device, 1, 1, &mut pcm);
    if err < 0 {
        return err;
    }

    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &SND_ES1688_PLAYBACK_OPS);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &SND_ES1688_CAPTURE_OPS);

    (*pcm).private_data = chip as *mut c_void;
    (*pcm).info_flags = SNDRV_PCM_INFO_HALF_DUPLEX;
    strscpy((*pcm).name, snd_es1688_chip_id(chip));
    (*chip).pcm = pcm;

    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, (*card).dev, 64 * 1024, 64 * 1024);
    0
}

/*
 *  MIXER part
 */

unsafe extern "C" fn snd_es1688_info_mux(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    static TEXT0: &[u8] = b"Mic\0";
    static TEXT1: &[u8] = b"Mic Master\0";
    static TEXT2: &[u8] = b"CD\0";
    static TEXT3: &[u8] = b"AOUT\0";
    static TEXT4: &[u8] = b"Mic1\0";
    static TEXT5: &[u8] = b"Mix\0";
    static TEXT6: &[u8] = b"Line\0";
    static TEXT7: &[u8] = b"Master\0";
    let texts: [*const c_char; 8] = [
        TEXT0.as_ptr() as *const c_char, TEXT1.as_ptr() as *const c_char,
        TEXT2.as_ptr() as *const c_char, TEXT3.as_ptr() as *const c_char,
        TEXT4.as_ptr() as *const c_char, TEXT5.as_ptr() as *const c_char,
        TEXT6.as_ptr() as *const c_char, TEXT7.as_ptr() as *const c_char,
    ];

    snd_ctl_enum_info(uinfo, 1, 8, texts.as_ptr())
}

unsafe extern "C" fn snd_es1688_get_mux(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    (*ucontrol).value.enumerated.item[0] = (snd_es1688_mixer_read(chip, ES1688_REC_DEV) & 7) as c_uint;
    0
}

unsafe extern "C" fn snd_es1688_put_mux(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let oval: u8;
    let nval: u8;
    let change: c_int;

    if (*ucontrol).value.enumerated.item[0] > 8 {
        return -EINVAL;
    }
    oval = snd_es1688_mixer_read(chip, ES1688_REC_DEV);
    nval = (((*ucontrol).value.enumerated.item[0] & 7) as u8) | (oval & !15);
    change = (nval != oval) as c_int;
    if change != 0 {
        snd_es1688_mixer_write(chip, ES1688_REC_DEV, nval);
    }
    change
}

const fn ES1688_SINGLE_VALUE(reg: u8, shift: c_ulong, mask: c_ulong, invert: c_ulong) -> c_ulong {
    reg as c_ulong | (shift << 8) | (mask << 16) | (invert << 24)
}

unsafe extern "C" fn snd_es1688_info_single(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let mask = (((*kcontrol).private_value >> 16) & 0xff) as c_int;

    (*uinfo).type_ = if mask == 1 { SNDRV_CTL_ELEM_TYPE_BOOLEAN } else { SNDRV_CTL_ELEM_TYPE_INTEGER };
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = mask as c_long;
    0
}

unsafe extern "C" fn snd_es1688_get_single(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let reg = ((*kcontrol).private_value & 0xff) as u8;
    let shift = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
    let mask = (((*kcontrol).private_value >> 16) & 0xff) as c_long;
    let invert = (((*kcontrol).private_value >> 24) & 0xff) as c_int;

    (*ucontrol).value.integer.value[0] = (((snd_es1688_mixer_read(chip, reg) as c_int) >> shift) as c_long) & mask;
    if invert != 0 {
        (*ucontrol).value.integer.value[0] = mask - (*ucontrol).value.integer.value[0];
    }
    0
}

unsafe extern "C" fn snd_es1688_put_single(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let reg = ((*kcontrol).private_value & 0xff) as u8;
    let shift = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
    let mask = (((*kcontrol).private_value >> 16) & 0xff) as c_long;
    let invert = (((*kcontrol).private_value >> 24) & 0xff) as c_int;
    let change: c_int;
    let oval: u8;
    let mut nval: u8;

    nval = ((*ucontrol).value.integer.value[0] & mask) as u8;
    if invert != 0 {
        nval = (mask as u8).wrapping_sub(nval);
    }
    nval <<= shift;
    oval = snd_es1688_mixer_read(chip, reg);
    nval = (oval & !((mask as u8) << shift)) | nval;
    change = (nval != oval) as c_int;
    if change != 0 {
        snd_es1688_mixer_write(chip, reg, nval);
    }
    change
}

const fn ES1688_DOUBLE_VALUE(left_reg: u8, right_reg: u8, shift_left: c_ulong, shift_right: c_ulong, mask: c_ulong, invert: c_ulong) -> c_ulong {
    left_reg as c_ulong | ((right_reg as c_ulong) << 8) | (shift_left << 16) | (shift_right << 19) | (mask << 24) | (invert << 22)
}

unsafe extern "C" fn snd_es1688_info_double(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let mask = (((*kcontrol).private_value >> 24) & 0xff) as c_int;

    (*uinfo).type_ = if mask == 1 { SNDRV_CTL_ELEM_TYPE_BOOLEAN } else { SNDRV_CTL_ELEM_TYPE_INTEGER };
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = mask as c_long;
    0
}

unsafe extern "C" fn snd_es1688_get_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let left_reg = ((*kcontrol).private_value & 0xff) as u8;
    let right_reg = (((*kcontrol).private_value >> 8) & 0xff) as u8;
    let shift_left = (((*kcontrol).private_value >> 16) & 0x07) as c_int;
    let shift_right = (((*kcontrol).private_value >> 19) & 0x07) as c_int;
    let mask = (((*kcontrol).private_value >> 24) & 0xff) as c_long;
    let invert = (((*kcontrol).private_value >> 22) & 1) as c_int;
    let left: u8;
    let right: u8;

    if left_reg < 0xa0 {
        left = snd_es1688_mixer_read(chip, left_reg);
    } else {
        left = snd_es1688_read(chip, left_reg) as u8;
    }
    if left_reg != right_reg {
        if right_reg < 0xa0 {
            right = snd_es1688_mixer_read(chip, right_reg);
        } else {
            right = snd_es1688_read(chip, right_reg) as u8;
        }
    } else {
        right = left;
    }
    (*ucontrol).value.integer.value[0] = (((left as c_int) >> shift_left) as c_long) & mask;
    (*ucontrol).value.integer.value[1] = (((right as c_int) >> shift_right) as c_long) & mask;
    if invert != 0 {
        (*ucontrol).value.integer.value[0] = mask - (*ucontrol).value.integer.value[0];
        (*ucontrol).value.integer.value[1] = mask - (*ucontrol).value.integer.value[1];
    }
    0
}

unsafe extern "C" fn snd_es1688_put_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let left_reg = ((*kcontrol).private_value & 0xff) as u8;
    let right_reg = (((*kcontrol).private_value >> 8) & 0xff) as u8;
    let shift_left = (((*kcontrol).private_value >> 16) & 0x07) as c_int;
    let shift_right = (((*kcontrol).private_value >> 19) & 0x07) as c_int;
    let mask = (((*kcontrol).private_value >> 24) & 0xff) as u8;
    let invert = (((*kcontrol).private_value >> 22) & 1) as c_int;
    let change: c_int;
    let mut val1: u8;
    let mut val2: u8;
    let oval1: u8;
    let oval2: u8;

    val1 = ((*ucontrol).value.integer.value[0] as u8) & mask;
    val2 = ((*ucontrol).value.integer.value[1] as u8) & mask;
    if invert != 0 {
        val1 = mask.wrapping_sub(val1);
        val2 = mask.wrapping_sub(val2);
    }
    val1 <<= shift_left;
    val2 <<= shift_right;
    if left_reg != right_reg {
        if left_reg < 0xa0 {
            oval1 = snd_es1688_mixer_read(chip, left_reg);
        } else {
            oval1 = snd_es1688_read(chip, left_reg) as u8;
        }
        if right_reg < 0xa0 {
            oval2 = snd_es1688_mixer_read(chip, right_reg);
        } else {
            oval2 = snd_es1688_read(chip, right_reg) as u8;
        }
        val1 = (oval1 & !(mask << shift_left)) | val1;
        val2 = (oval2 & !(mask << shift_right)) | val2;
        change = (val1 != oval1 || val2 != oval2) as c_int;
        if change != 0 {
            if left_reg < 0xa0 {
                snd_es1688_mixer_write(chip, left_reg, val1);
            } else {
                snd_es1688_write(chip, left_reg, val1);
            }
            if right_reg < 0xa0 {
                snd_es1688_mixer_write(chip, right_reg, val1);
            } else {
                snd_es1688_write(chip, right_reg, val1);
            }
        }
    } else {
        if left_reg < 0xa0 {
            oval1 = snd_es1688_mixer_read(chip, left_reg);
        } else {
            oval1 = snd_es1688_read(chip, left_reg) as u8;
        }
        val1 = (oval1 & !((mask << shift_left) | (mask << shift_right))) | val1 | val2;
        change = (val1 != oval1) as c_int;
        if change != 0 {
            if left_reg < 0xa0 {
                snd_es1688_mixer_write(chip, left_reg, val1);
            } else {
                snd_es1688_write(chip, left_reg, val1);
            }
        }
    }
    change
}

const fn es1688_single(xname: *const c_char, xindex: c_uint, reg: u8, shift: c_ulong, mask: c_ulong, invert: c_ulong) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: xname,
        index: xindex,
        info: Some(snd_es1688_info_single),
        get: Some(snd_es1688_get_single),
        put: Some(snd_es1688_put_single),
        private_value: ES1688_SINGLE_VALUE(reg, shift, mask, invert),
    }
}

const fn es1688_double(xname: *const c_char, xindex: c_uint, left_reg: u8, right_reg: u8, shift_left: c_ulong, shift_right: c_ulong, mask: c_ulong, invert: c_ulong) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: xname,
        index: xindex,
        info: Some(snd_es1688_info_double),
        get: Some(snd_es1688_get_double),
        put: Some(snd_es1688_put_double),
        private_value: ES1688_DOUBLE_VALUE(left_reg, right_reg, shift_left, shift_right, mask, invert),
    }
}

static SND_ES1688_CONTROLS: [snd_kcontrol_new; 11] = [
    es1688_double(b"Master Playback Volume\0".as_ptr() as *const c_char, 0, ES1688_MASTER_DEV, ES1688_MASTER_DEV, 4, 0, 15, 0),
    es1688_double(b"PCM Playback Volume\0".as_ptr() as *const c_char, 0, ES1688_PCM_DEV, ES1688_PCM_DEV, 4, 0, 15, 0),
    es1688_double(b"Line Playback Volume\0".as_ptr() as *const c_char, 0, ES1688_LINE_DEV, ES1688_LINE_DEV, 4, 0, 15, 0),
    es1688_double(b"CD Playback Volume\0".as_ptr() as *const c_char, 0, ES1688_CD_DEV, ES1688_CD_DEV, 4, 0, 15, 0),
    es1688_double(b"FM Playback Volume\0".as_ptr() as *const c_char, 0, ES1688_FM_DEV, ES1688_FM_DEV, 4, 0, 15, 0),
    es1688_double(b"Mic Playback Volume\0".as_ptr() as *const c_char, 0, ES1688_MIC_DEV, ES1688_MIC_DEV, 4, 0, 15, 0),
    es1688_double(b"Aux Playback Volume\0".as_ptr() as *const c_char, 0, ES1688_AUX_DEV, ES1688_AUX_DEV, 4, 0, 15, 0),
    es1688_single(b"Beep Playback Volume\0".as_ptr() as *const c_char, 0, ES1688_SPEAKER_DEV, 0, 7, 0),
    es1688_double(b"Capture Volume\0".as_ptr() as *const c_char, 0, ES1688_RECLEV_DEV, ES1688_RECLEV_DEV, 4, 0, 15, 0),
    es1688_single(b"Capture Switch\0".as_ptr() as *const c_char, 0, ES1688_REC_DEV, 4, 1, 1),
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Capture Source\0".as_ptr() as *const c_char,
        index: 0,
        info: Some(snd_es1688_info_mux),
        get: Some(snd_es1688_get_mux),
        put: Some(snd_es1688_put_mux),
        private_value: 0,
    },
];

const ES1688_INIT_TABLE_SIZE: usize = SND_ES1688_INIT_TABLE.len();

static SND_ES1688_INIT_TABLE: [[u8; 2]; 10] = [
    [ES1688_MASTER_DEV, 0],
    [ES1688_PCM_DEV, 0],
    [ES1688_LINE_DEV, 0],
    [ES1688_CD_DEV, 0],
    [ES1688_FM_DEV, 0],
    [ES1688_MIC_DEV, 0],
    [ES1688_AUX_DEV, 0],
    [ES1688_SPEAKER_DEV, 0],
    [ES1688_RECLEV_DEV, 0],
    [ES1688_REC_DEV, 0x17],
];

#[no_mangle]
pub unsafe extern "C" fn snd_es1688_mixer(card: *mut snd_card, chip: *mut snd_es1688) -> c_int {
    let mut idx: c_uint;
    let mut err: c_int;
    let mut reg: u8;
    let mut val: u8;

    if snd_BUG_ON(chip.is_null() || card.is_null()) {
        return -EINVAL;
    }

    strscpy((*card).mixername, snd_es1688_chip_id(chip));

    idx = 0;
    while (idx as usize) < SND_ES1688_CONTROLS.len() {
        err = snd_ctl_add(card, snd_ctl_new1(&SND_ES1688_CONTROLS[idx as usize], chip as *mut c_void));
        if err < 0 {
            return err;
        }
        idx += 1;
    }
    idx = 0;
    while (idx as usize) < ES1688_INIT_TABLE_SIZE {
        reg = SND_ES1688_INIT_TABLE[idx as usize][0];
        val = SND_ES1688_INIT_TABLE[idx as usize][1];
        if reg < 0xa0 {
            snd_es1688_mixer_write(chip, reg, val);
        } else {
            snd_es1688_write(chip, reg, val);
        }
        idx += 1;
    }
    0
}

/* EXPORT_SYMBOL(snd_es1688_reset);
 * EXPORT_SYMBOL(snd_es1688_mixer_write);
 * EXPORT_SYMBOL(snd_es1688_create);
 * EXPORT_SYMBOL(snd_es1688_pcm);
 * EXPORT_SYMBOL(snd_es1688_mixer);
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
