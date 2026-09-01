// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for NeoMagic 256AV and 256ZX chipsets.
 * Copyright (c) 2000 by Takashi Iwai <tiwai@suse.de>
 *
 * Based on nm256_audio.c OSS driver in linux kernel.
 * The original author of OSS nm256 driver wishes to remain anonymous,
 * so I just put my acknoledgment to him/her here.
 * The original author's web page is found at
 *	http://www.uglx.org/sony.html
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type u8 = u8;
type u16 = u16;
type u32 = u32;
type bool_t = bool;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_int;
type irq_handler_t = Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>;

#[repr(C)] pub struct snd_card { pub dev: *mut c_void, pub driver: [c_char; 16], pub shortname: [c_char; 32], pub longname: [c_char; 80], pub mixername: [c_char; 80], pub sync_irq: c_int, pub private_data: *mut c_void, pub private_free: Option<unsafe extern "C" fn(*mut snd_card)> }
#[repr(C)] pub struct snd_pcm_substream { pub runtime: *mut snd_pcm_runtime, pub stream: c_int }
#[repr(C)] pub struct snd_pcm_runtime { pub rate: c_uint, pub format: c_int, pub channels: c_uint, pub private_data: *mut c_void, pub buffer_size: snd_pcm_uframes_t, pub period_size: snd_pcm_uframes_t, pub periods: c_int, pub dma_area: *mut c_void, pub dma_addr: c_ulong, pub dma_bytes: c_int, pub hw: snd_pcm_hardware }
#[repr(C)] pub struct snd_pcm { pub private_data: *mut c_void, pub info_flags: c_int }
#[repr(C)] pub struct snd_ac97 { pub private_data: *mut c_void, pub id: u32 }
#[repr(C)] pub struct snd_ac97_bus { pub no_vra: c_int }
#[repr(C)] pub struct pci_dev { pub dev: c_void, pub irq: c_int, pub device: c_uint }
#[repr(C)] pub struct pci_device_id { pub vendor: c_uint, pub device: c_uint, pub subvendor: c_uint, pub subdevice: c_uint, pub class: c_uint, pub class_mask: c_uint, pub driver_data: c_ulong }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct iov_iter { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *const c_uint,
    pub mask: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: u64,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub buffer_bytes_max: c_uint,
    pub period_bytes_min: c_uint,
    pub period_bytes_max: c_uint,
}

#[repr(C)]
pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    pub copy: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, c_ulong, *mut iov_iter, c_ulong) -> c_int>,
    pub fill_silence: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, c_ulong, c_ulong) -> c_int>,
    pub mmap: Option<unsafe extern "C" fn() -> c_int>,
}

#[repr(C)] pub struct snd_ac97_res_table { pub reg: u16, pub bits: u16 }
#[repr(C)] pub struct snd_ac97_bus_ops { pub reset: Option<unsafe extern "C" fn(*mut snd_ac97)>, pub write: Option<unsafe extern "C" fn(*mut snd_ac97, u16, u16)>, pub read: Option<unsafe extern "C" fn(*mut snd_ac97, u16) -> u16> }
#[repr(C)] pub struct snd_ac97_template { pub scaps: c_uint, pub private_data: *mut c_void, pub res_table: *const snd_ac97_res_table }
#[repr(C)] pub struct snd_pci_quirk { pub subvendor: c_uint, pub subdevice: c_uint, pub name: *const c_char, pub value: c_int }
#[repr(C)] pub struct dev_pm_ops { _private: [u8; 0] }
#[repr(C)] pub struct pci_driver { pub name: *const c_char, pub id_table: *const pci_device_id, pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>, pub driver: pci_driver_inner }
#[repr(C)] pub struct pci_driver_inner { pub pm: *const dev_pm_ops }

const CARD_NAME: &[u8] = b"NeoMagic 256AV/ZX\0";
const DRIVER_NAME: &[u8] = b"NM256\0";

static mut index: c_int = SNDRV_DEFAULT_IDX1;
static mut id: *mut c_char = SNDRV_DEFAULT_STR1 as *mut c_char;
static mut playback_bufsize: c_int = 16;
static mut capture_bufsize: c_int = 16;
static mut force_ac97: bool = false;
static mut buffer_top: c_int = 0;
static mut use_cache: bool = false;
static mut vaio_hack: bool = false;
static mut reset_workaround: bool = false;
static mut reset_workaround_2: bool = false;
static mut enable: bool = false;

const NM_SIGNATURE: u32 = 0x4e4d0000;
const NM_SIG_MASK: u32 = 0xffff0000;
const NM_PORT2_SIZE: c_ulong = 4096;
const NM_MIXER_OFFSET: c_int = 0x600;
const NM_MAX_PLAYBACK_COEF_SIZE: u32 = 0x5000;
const NM_MAX_RECORD_COEF_SIZE: u32 = 0x1260;
const NM_INT_REG: c_int = 0xa04;
const NM_PLAYBACK_INT: u32 = 0x40;
const NM_RECORD_INT: u32 = 0x100;
const NM_MISC_INT_1: u32 = 0x4000;
const NM_MISC_INT_2: u32 = 0x1;
const NM_MIXER_STATUS_OFFSET: c_int = 0xa04;
const NM_MIXER_READY_MASK: c_int = 0x0800;
const NM_MIXER_PRESENCE: c_int = 0xa06;
const NM_PRESENCE_MASK: c_int = 0x0050;
const NM_PRESENCE_VALUE: c_int = 0x0040;
const NM2_PLAYBACK_INT: u32 = 0x10000;
const NM2_RECORD_INT: u32 = 0x80000;
const NM2_MISC_INT_1: u32 = 0x8;
const NM2_MISC_INT_2: u32 = 0x2;
const NM2_MIXER_STATUS_OFFSET: c_int = 0xa06;
const NM2_MIXER_READY_MASK: c_int = 0x0800;
const NM_PLAYBACK_REG_OFFSET: c_int = 0x0;
const NM_RECORD_REG_OFFSET: c_int = 0x200;
const NM_RATE_REG_OFFSET: c_int = 2;
const NM_RATE_STEREO: u8 = 1;
const NM_RATE_BITS_16: u8 = 2;
const NM_RATE_MASK: u8 = 0xf0;
const NM_PLAYBACK_ENABLE_REG: c_int = NM_PLAYBACK_REG_OFFSET + 0x1;
const NM_PLAYBACK_ENABLE_FLAG: u8 = 1;
const NM_PLAYBACK_ONESHOT: u8 = 2;
const NM_PLAYBACK_FREERUN: u8 = 4;
const NM_AUDIO_MUTE_REG: c_int = NM_PLAYBACK_REG_OFFSET + 0x18;
const NM_AUDIO_MUTE_LEFT: u16 = 0x8000;
const NM_AUDIO_MUTE_RIGHT: u16 = 0x0080;
const NM_RECORD_ENABLE_REG: c_int = NM_RECORD_REG_OFFSET + 0;
const NM_RECORD_ENABLE_FLAG: u8 = 1;
const NM_RECORD_FREERUN: u8 = 2;
const NM_COEFF_START_OFFSET: c_int = 0x1c;
const NM_COEFF_END_OFFSET: c_int = 0x20;
const NM_RBUFFER_START: c_int = NM_RECORD_REG_OFFSET + 0x4;
const NM_RBUFFER_END: c_int = NM_RECORD_REG_OFFSET + 0x10;
const NM_RBUFFER_WMARK: c_int = NM_RECORD_REG_OFFSET + 0xc;
const NM_RBUFFER_CURRP: c_int = NM_RECORD_REG_OFFSET + 0x8;
const NM_PBUFFER_START: c_int = NM_PLAYBACK_REG_OFFSET + 0x4;
const NM_PBUFFER_END: c_int = NM_PLAYBACK_REG_OFFSET + 0x14;
const NM_PBUFFER_WMARK: c_int = NM_PLAYBACK_REG_OFFSET + 0xc;
const NM_PBUFFER_CURRP: c_int = NM_PLAYBACK_REG_OFFSET + 0x8;

#[repr(C)]
pub struct nm256_stream {
    pub chip: *mut nm256,
    pub substream: *mut snd_pcm_substream,
    pub running: c_int,
    pub suspended: c_int,
    pub buf: u32,
    pub bufsize: c_int,
    pub bufptr: *mut c_void,
    pub bufptr_addr: c_ulong,
    pub dma_size: c_int,
    pub period_size: c_int,
    pub periods: c_int,
    pub shift: c_int,
    pub cur_period: c_int,
}

#[repr(C)]
pub struct nm256 {
    pub card: *mut snd_card,
    pub cport: *mut c_void,
    pub cport_addr: c_ulong,
    pub buffer: *mut c_void,
    pub buffer_addr: c_ulong,
    pub buffer_start: u32,
    pub buffer_end: u32,
    pub buffer_size: u32,
    pub all_coeff_buf: u32,
    pub coeff_buf: [u32; 2],
    pub coeffs_current: c_uint,
    pub use_cache: c_uint,
    pub reset_workaround: c_uint,
    pub reset_workaround_2: c_uint,
    pub in_resume: c_uint,
    pub mixer_base: c_int,
    pub mixer_status_offset: c_int,
    pub mixer_status_mask: c_int,
    pub irq: c_int,
    pub irq_acks: c_int,
    pub interrupt: irq_handler_t,
    pub badintrcount: c_int,
    pub irq_mutex: mutex,
    pub streams: [nm256_stream; 2],
    pub ac97: *mut snd_ac97,
    pub ac97_regs: *mut u16,
    pub pcm: *mut snd_pcm,
    pub pci: *mut pci_dev,
    pub reg_lock: spinlock_t,
}

// include coefficient table: nm256_coef.c
extern "C" {
    static coefficients: [u8; 0];
    static coefficient_sizes: [u16; 0];
    static NM_TOTAL_COEFF_COUNT: c_uint;
}

static snd_nm256_ids: [pci_device_id; 4] = [
    pci_vdevice(PCI_VENDOR_ID_NEOMAGIC, PCI_DEVICE_ID_NEOMAGIC_NM256AV_AUDIO),
    pci_vdevice(PCI_VENDOR_ID_NEOMAGIC, PCI_DEVICE_ID_NEOMAGIC_NM256ZX_AUDIO),
    pci_vdevice(PCI_VENDOR_ID_NEOMAGIC, PCI_DEVICE_ID_NEOMAGIC_NM256XL_PLUS_AUDIO),
    pci_device_id { vendor: 0, device: 0, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: 0 },
];

const fn pci_vdevice(vendor: c_uint, device: c_uint) -> pci_device_id {
    pci_device_id { vendor, device, subvendor: PCI_ANY_ID, subdevice: PCI_ANY_ID, class: 0, class_mask: 0, driver_data: 0 }
}

unsafe fn snd_nm256_readb(chip: *mut nm256, offset: c_int) -> u8 {
    readb(((*chip).cport as *mut u8).offset(offset as isize))
}

unsafe fn snd_nm256_readw(chip: *mut nm256, offset: c_int) -> u16 {
    readw(((*chip).cport as *mut u8).offset(offset as isize) as *mut c_void)
}

unsafe fn snd_nm256_readl(chip: *mut nm256, offset: c_int) -> u32 {
    readl(((*chip).cport as *mut u8).offset(offset as isize) as *mut c_void)
}

unsafe fn snd_nm256_writeb(chip: *mut nm256, offset: c_int, val: u8) {
    writeb(val, ((*chip).cport as *mut u8).offset(offset as isize));
}

unsafe fn snd_nm256_writew(chip: *mut nm256, offset: c_int, val: u16) {
    writew(val, ((*chip).cport as *mut u8).offset(offset as isize) as *mut c_void);
}

unsafe fn snd_nm256_writel(chip: *mut nm256, offset: c_int, val: u32) {
    writel(val, ((*chip).cport as *mut u8).offset(offset as isize) as *mut c_void);
}

unsafe fn NM_ACK_INT(chip: *mut nm256, X: u32) { snd_nm256_writew(chip, NM_INT_REG, (X << 1) as u16); }
unsafe fn NM2_ACK_INT(chip: *mut nm256, X: u32) { snd_nm256_writel(chip, NM_INT_REG, X); }

unsafe fn snd_nm256_write_buffer(chip: *mut nm256, src: *const c_void, mut offset: c_int, size: c_int) {
    offset -= (*chip).buffer_start as c_int;
    // CONFIG_SND_DEBUG: validate offset against chip->buffer_size before writing.
    memcpy_toio(((*chip).buffer as *mut u8).offset(offset as isize) as *mut c_void, src, size as c_ulong);
}

unsafe fn snd_nm256_get_start_offset(mut which: c_int) -> u16 {
    let mut offset: u16 = 0;
    while { let old = which; which -= 1; old > 0 } {
        offset = offset.wrapping_add(*coefficient_sizes.as_ptr().offset(which as isize));
    }
    offset
}

unsafe fn snd_nm256_load_one_coefficient(chip: *mut nm256, stream: c_int, port: u32, which: c_int) {
    let coeff_buf = (*chip).coeff_buf[stream as usize];
    let offset = snd_nm256_get_start_offset(which);
    let mut size = *coefficient_sizes.as_ptr().offset(which as isize);
    snd_nm256_write_buffer(chip, coefficients.as_ptr().offset(offset as isize) as *const c_void, coeff_buf as c_int, size as c_int);
    snd_nm256_writel(chip, port as c_int, coeff_buf);
    /* ???  Record seems to behave differently than playback.  */
    if stream == SNDRV_PCM_STREAM_PLAYBACK {
        size = size.wrapping_sub(1);
    }
    snd_nm256_writel(chip, port as c_int + 4, coeff_buf.wrapping_add(size as u32));
}

unsafe fn snd_nm256_load_coefficient(chip: *mut nm256, stream: c_int, mut number: c_int) {
    /* The enable register for the specified engine.  */
    let poffset = if stream == SNDRV_PCM_STREAM_CAPTURE { NM_RECORD_ENABLE_REG } else { NM_PLAYBACK_ENABLE_REG };
    let mut addr = NM_COEFF_START_OFFSET;
    addr += if stream == SNDRV_PCM_STREAM_CAPTURE { NM_RECORD_REG_OFFSET } else { NM_PLAYBACK_REG_OFFSET };
    if snd_nm256_readb(chip, poffset) & 1 != 0 {
        dev_dbg((*(*chip).card).dev, c"NM256: Engine was enabled while loading coefficients!\n".as_ptr());
        return;
    }
    /* The recording engine uses coefficient values 8-15.  */
    number &= 7;
    if stream == SNDRV_PCM_STREAM_CAPTURE { number += 8; }
    if (*chip).use_cache == 0 {
        snd_nm256_load_one_coefficient(chip, stream, addr as u32, number);
        return;
    }
    if (*chip).coeffs_current == 0 {
        snd_nm256_write_buffer(chip, coefficients.as_ptr() as *const c_void, (*chip).all_coeff_buf as c_int, (NM_TOTAL_COEFF_COUNT * 4) as c_int);
        (*chip).coeffs_current = 1;
    } else {
        let base = (*chip).all_coeff_buf;
        let offset = snd_nm256_get_start_offset(number) as u32;
        let mut end_offset = offset + *coefficient_sizes.as_ptr().offset(number as isize) as u32;
        snd_nm256_writel(chip, addr, base + offset);
        if stream == SNDRV_PCM_STREAM_PLAYBACK { end_offset -= 1; }
        snd_nm256_writel(chip, addr + 4, base + end_offset);
    }
}

static samplerates: [c_uint; 8] = [8000, 11025, 16000, 22050, 24000, 32000, 44100, 48000];
static constraints_rates: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list { count: 8, list: samplerates.as_ptr(), mask: 0 };

unsafe fn snd_nm256_fixed_rate(rate: c_uint) -> c_int {
    let mut i: usize = 0;
    while i < samplerates.len() {
        if rate == samplerates[i] { return i as c_int; }
        i += 1;
    }
    snd_BUG();
    0
}

unsafe fn snd_nm256_set_format(chip: *mut nm256, s: *mut nm256_stream, substream: *mut snd_pcm_substream) {
    let runtime = (*substream).runtime;
    let rate_index = snd_nm256_fixed_rate((*runtime).rate);
    let mut ratebits: u8 = ((rate_index << 4) as u8) & NM_RATE_MASK;
    (*s).shift = 0;
    if snd_pcm_format_width((*runtime).format) == 16 {
        ratebits |= NM_RATE_BITS_16;
        (*s).shift += 1;
    }
    if (*runtime).channels > 1 {
        ratebits |= NM_RATE_STEREO;
        (*s).shift += 1;
    }
    (*runtime).rate = samplerates[rate_index as usize];
    match (*substream).stream {
        SNDRV_PCM_STREAM_PLAYBACK => {
            snd_nm256_load_coefficient(chip, 0, rate_index);
            snd_nm256_writeb(chip, NM_PLAYBACK_REG_OFFSET + NM_RATE_REG_OFFSET, ratebits);
        }
        SNDRV_PCM_STREAM_CAPTURE => {
            snd_nm256_load_coefficient(chip, 1, rate_index);
            snd_nm256_writeb(chip, NM_RECORD_REG_OFFSET + NM_RATE_REG_OFFSET, ratebits);
        }
        _ => {}
    }
}

unsafe fn snd_nm256_acquire_irq(chip: *mut nm256) -> c_int {
    mutex_lock(&mut (*chip).irq_mutex);
    if (*chip).irq < 0 {
        if request_irq((*(*chip).pci).irq, (*chip).interrupt, IRQF_SHARED, KBUILD_MODNAME, chip as *mut c_void) != 0 {
            dev_err((*(*chip).card).dev, c"unable to grab IRQ %d\n".as_ptr(), (*(*chip).pci).irq);
            mutex_unlock(&mut (*chip).irq_mutex);
            return -EBUSY;
        }
        (*chip).irq = (*(*chip).pci).irq;
        (*(*chip).card).sync_irq = (*chip).irq;
    }
    (*chip).irq_acks += 1;
    mutex_unlock(&mut (*chip).irq_mutex);
    0
}

unsafe fn snd_nm256_release_irq(chip: *mut nm256) {
    mutex_lock(&mut (*chip).irq_mutex);
    if (*chip).irq_acks > 0 { (*chip).irq_acks -= 1; }
    if (*chip).irq_acks == 0 && (*chip).irq >= 0 {
        free_irq((*chip).irq, chip as *mut c_void);
        (*chip).irq = -1;
        (*(*chip).card).sync_irq = -1;
    }
    mutex_unlock(&mut (*chip).irq_mutex);
}

unsafe fn snd_nm256_pcm_mark(chip: *mut nm256, s: *mut nm256_stream, reg: c_int) {
    (*s).cur_period += 1;
    (*s).cur_period %= (*s).periods;
    snd_nm256_writel(chip, reg, (*s).buf + ((*s).cur_period * (*s).period_size) as u32);
}
unsafe fn snd_nm256_playback_mark(chip: *mut nm256, s: *mut nm256_stream) { snd_nm256_pcm_mark(chip, s, NM_PBUFFER_WMARK); }
unsafe fn snd_nm256_capture_mark(chip: *mut nm256, s: *mut nm256_stream) { snd_nm256_pcm_mark(chip, s, NM_RBUFFER_WMARK); }

unsafe fn snd_nm256_playback_start(chip: *mut nm256, s: *mut nm256_stream, _substream: *mut snd_pcm_substream) {
    snd_nm256_writel(chip, NM_PBUFFER_START, (*s).buf);
    snd_nm256_writel(chip, NM_PBUFFER_END, (*s).buf + (*s).dma_size as u32 - (1u32 << (*s).shift));
    snd_nm256_writel(chip, NM_PBUFFER_CURRP, (*s).buf);
    snd_nm256_playback_mark(chip, s);
    snd_nm256_writeb(chip, NM_PLAYBACK_ENABLE_REG, NM_PLAYBACK_ENABLE_FLAG | NM_PLAYBACK_FREERUN);
    snd_nm256_writew(chip, NM_AUDIO_MUTE_REG, 0x0);
}

unsafe fn snd_nm256_capture_start(chip: *mut nm256, s: *mut nm256_stream, _substream: *mut snd_pcm_substream) {
    snd_nm256_writel(chip, NM_RBUFFER_START, (*s).buf);
    snd_nm256_writel(chip, NM_RBUFFER_END, (*s).buf + (*s).dma_size as u32);
    snd_nm256_writel(chip, NM_RBUFFER_CURRP, (*s).buf);
    snd_nm256_capture_mark(chip, s);
    snd_nm256_writeb(chip, NM_RECORD_ENABLE_REG, NM_RECORD_ENABLE_FLAG | NM_RECORD_FREERUN);
}

unsafe fn snd_nm256_playback_stop(chip: *mut nm256) {
    snd_nm256_writew(chip, NM_AUDIO_MUTE_REG, NM_AUDIO_MUTE_LEFT | NM_AUDIO_MUTE_RIGHT);
    snd_nm256_writeb(chip, NM_PLAYBACK_ENABLE_REG, 0);
}

unsafe fn snd_nm256_capture_stop(chip: *mut nm256) {
    snd_nm256_writeb(chip, NM_RECORD_ENABLE_REG, 0);
}

unsafe extern "C" fn snd_nm256_playback_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip = snd_pcm_substream_chip(substream) as *mut nm256;
    let s = (*(*substream).runtime).private_data as *mut nm256_stream;
    if snd_BUG_ON(s.is_null() as c_int) != 0 { return -ENXIO; }
    spin_lock(&mut (*chip).reg_lock);
    match cmd {
        SNDRV_PCM_TRIGGER_RESUME => { (*s).suspended = 0; if (*s).running == 0 { snd_nm256_playback_start(chip, s, substream); (*s).running = 1; } }
        SNDRV_PCM_TRIGGER_START => { if (*s).running == 0 { snd_nm256_playback_start(chip, s, substream); (*s).running = 1; } }
        SNDRV_PCM_TRIGGER_SUSPEND => { (*s).suspended = 1; if (*s).running != 0 { snd_nm256_playback_stop(chip); (*s).running = 0; } }
        SNDRV_PCM_TRIGGER_STOP => { if (*s).running != 0 { snd_nm256_playback_stop(chip); (*s).running = 0; } }
        _ => { spin_unlock(&mut (*chip).reg_lock); return -EINVAL; }
    }
    spin_unlock(&mut (*chip).reg_lock);
    0
}

unsafe extern "C" fn snd_nm256_capture_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip = snd_pcm_substream_chip(substream) as *mut nm256;
    let s = (*(*substream).runtime).private_data as *mut nm256_stream;
    if snd_BUG_ON(s.is_null() as c_int) != 0 { return -ENXIO; }
    spin_lock(&mut (*chip).reg_lock);
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => { if (*s).running == 0 { snd_nm256_capture_start(chip, s, substream); (*s).running = 1; } }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => { if (*s).running != 0 { snd_nm256_capture_stop(chip); (*s).running = 0; } }
        _ => { spin_unlock(&mut (*chip).reg_lock); return -EINVAL; }
    }
    spin_unlock(&mut (*chip).reg_lock);
    0
}

unsafe extern "C" fn snd_nm256_pcm_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream) as *mut nm256;
    let runtime = (*substream).runtime;
    let s = (*runtime).private_data as *mut nm256_stream;
    if snd_BUG_ON(s.is_null() as c_int) != 0 { return -ENXIO; }
    (*s).dma_size = frames_to_bytes(runtime, (*runtime).buffer_size);
    (*s).period_size = frames_to_bytes(runtime, (*runtime).period_size);
    (*s).periods = (*runtime).periods;
    (*s).cur_period = 0;
    spin_lock_irq(&mut (*chip).reg_lock);
    (*s).running = 0;
    snd_nm256_set_format(chip, s, substream);
    spin_unlock_irq(&mut (*chip).reg_lock);
    0
}

unsafe extern "C" fn snd_nm256_playback_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream) as *mut nm256;
    let s = (*(*substream).runtime).private_data as *mut nm256_stream;
    if snd_BUG_ON(s.is_null() as c_int) != 0 { return 0; }
    let mut curp = snd_nm256_readl(chip, NM_PBUFFER_CURRP) as c_ulong - (*s).buf as c_ulong;
    curp %= (*s).dma_size as c_ulong;
    bytes_to_frames((*substream).runtime, curp)
}

unsafe extern "C" fn snd_nm256_capture_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream) as *mut nm256;
    let s = (*(*substream).runtime).private_data as *mut nm256_stream;
    if snd_BUG_ON(s.is_null() as c_int) != 0 { return 0; }
    let mut curp = snd_nm256_readl(chip, NM_RBUFFER_CURRP) as c_ulong - (*s).buf as c_ulong;
    curp %= (*s).dma_size as c_ulong;
    bytes_to_frames((*substream).runtime, curp)
}

/* Remapped I/O space can be accessible as pointer on i386 */
/* This might be changed in the future */
#[cfg(not(target_arch = "x86"))]
unsafe extern "C" fn snd_nm256_playback_silence(substream: *mut snd_pcm_substream, _channel: c_int, pos: c_ulong, count: c_ulong) -> c_int {
    let runtime = (*substream).runtime;
    let s = (*runtime).private_data as *mut nm256_stream;
    memset_io(((*s).bufptr as *mut u8).offset(pos as isize) as *mut c_void, 0, count);
    0
}

#[cfg(not(target_arch = "x86"))]
unsafe extern "C" fn snd_nm256_playback_copy(substream: *mut snd_pcm_substream, _channel: c_int, pos: c_ulong, src: *mut iov_iter, count: c_ulong) -> c_int {
    let runtime = (*substream).runtime;
    let s = (*runtime).private_data as *mut nm256_stream;
    if copy_from_iter_toio(((*s).bufptr as *mut u8).offset(pos as isize) as *mut c_void, count, src) != count { return -EFAULT; }
    0
}

#[cfg(not(target_arch = "x86"))]
unsafe extern "C" fn snd_nm256_capture_copy(substream: *mut snd_pcm_substream, _channel: c_int, pos: c_ulong, dst: *mut iov_iter, count: c_ulong) -> c_int {
    let runtime = (*substream).runtime;
    let s = (*runtime).private_data as *mut nm256_stream;
    if copy_to_iter_fromio(((*s).bufptr as *mut u8).offset(pos as isize) as *mut c_void, count, dst) != count { return -EFAULT; }
    0
}

unsafe fn snd_nm256_playback_update(chip: *mut nm256) {
    let s = &mut (*chip).streams[SNDRV_PCM_STREAM_PLAYBACK as usize] as *mut nm256_stream;
    if (*s).running != 0 && !(*s).substream.is_null() {
        spin_unlock(&mut (*chip).reg_lock);
        snd_pcm_period_elapsed((*s).substream);
        spin_lock(&mut (*chip).reg_lock);
        snd_nm256_playback_mark(chip, s);
    }
}

unsafe fn snd_nm256_capture_update(chip: *mut nm256) {
    let s = &mut (*chip).streams[SNDRV_PCM_STREAM_CAPTURE as usize] as *mut nm256_stream;
    if (*s).running != 0 && !(*s).substream.is_null() {
        spin_unlock(&mut (*chip).reg_lock);
        snd_pcm_period_elapsed((*s).substream);
        spin_lock(&mut (*chip).reg_lock);
        snd_nm256_capture_mark(chip, s);
    }
}

static snd_nm256_playback: snd_pcm_hardware = snd_pcm_hardware { info: SNDRV_PCM_INFO_MMAP_IOMEM | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_RESUME, formats: SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE, rates: SNDRV_PCM_RATE_KNOT | SNDRV_PCM_RATE_8000_48000, rate_min: 8000, rate_max: 48000, channels_min: 1, channels_max: 2, periods_min: 2, periods_max: 1024, buffer_bytes_max: 128 * 1024, period_bytes_min: 256, period_bytes_max: 128 * 1024 };
static snd_nm256_capture: snd_pcm_hardware = snd_pcm_hardware { info: SNDRV_PCM_INFO_MMAP_IOMEM | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_RESUME, formats: SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE, rates: SNDRV_PCM_RATE_KNOT | SNDRV_PCM_RATE_8000_48000, rate_min: 8000, rate_max: 48000, channels_min: 1, channels_max: 2, periods_min: 2, periods_max: 1024, buffer_bytes_max: 128 * 1024, period_bytes_min: 256, period_bytes_max: 128 * 1024 };

unsafe extern "C" fn snd_nm256_pcm_hw_params(substream: *mut snd_pcm_substream, hw_params: *mut snd_pcm_hw_params) -> c_int {
    /* area and addr are already set and unchanged */
    (*(*substream).runtime).dma_bytes = params_buffer_bytes(hw_params);
    0
}

unsafe fn snd_nm256_setup_stream(chip: *mut nm256, s: *mut nm256_stream, substream: *mut snd_pcm_substream, hw_ptr: *const snd_pcm_hardware) {
    let runtime = (*substream).runtime;
    (*s).running = 0;
    (*runtime).hw = *hw_ptr;
    (*runtime).hw.buffer_bytes_max = (*s).bufsize as c_uint;
    (*runtime).hw.period_bytes_max = ((*s).bufsize / 2) as c_uint;
    (*runtime).dma_area = (*s).bufptr;
    (*runtime).dma_addr = (*s).bufptr_addr;
    (*runtime).dma_bytes = (*s).bufsize;
    (*runtime).private_data = s as *mut c_void;
    (*s).substream = substream;
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &constraints_rates);
}

unsafe extern "C" fn snd_nm256_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream) as *mut nm256;
    if snd_nm256_acquire_irq(chip) < 0 { return -EBUSY; }
    snd_nm256_setup_stream(chip, &mut (*chip).streams[SNDRV_PCM_STREAM_PLAYBACK as usize], substream, &snd_nm256_playback);
    0
}

unsafe extern "C" fn snd_nm256_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream) as *mut nm256;
    if snd_nm256_acquire_irq(chip) < 0 { return -EBUSY; }
    snd_nm256_setup_stream(chip, &mut (*chip).streams[SNDRV_PCM_STREAM_CAPTURE as usize], substream, &snd_nm256_capture);
    0
}

unsafe extern "C" fn snd_nm256_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream) as *mut nm256;
    snd_nm256_release_irq(chip);
    0
}

unsafe extern "C" fn snd_nm256_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream) as *mut nm256;
    snd_nm256_release_irq(chip);
    0
}

static snd_nm256_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_nm256_playback_open), close: Some(snd_nm256_playback_close), hw_params: Some(snd_nm256_pcm_hw_params), prepare: Some(snd_nm256_pcm_prepare), trigger: Some(snd_nm256_playback_trigger), pointer: Some(snd_nm256_playback_pointer),
    #[cfg(not(target_arch = "x86"))] copy: Some(snd_nm256_playback_copy),
    #[cfg(target_arch = "x86")] copy: None,
    #[cfg(not(target_arch = "x86"))] fill_silence: Some(snd_nm256_playback_silence),
    #[cfg(target_arch = "x86")] fill_silence: None,
    mmap: Some(snd_pcm_lib_mmap_iomem),
};

static snd_nm256_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_nm256_capture_open), close: Some(snd_nm256_capture_close), hw_params: Some(snd_nm256_pcm_hw_params), prepare: Some(snd_nm256_pcm_prepare), trigger: Some(snd_nm256_capture_trigger), pointer: Some(snd_nm256_capture_pointer),
    #[cfg(not(target_arch = "x86"))] copy: Some(snd_nm256_capture_copy),
    #[cfg(target_arch = "x86")] copy: None,
    fill_silence: None,
    mmap: Some(snd_pcm_lib_mmap_iomem),
};

unsafe fn snd_nm256_pcm(chip: *mut nm256, device: c_int) -> c_int {
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let mut i = 0;
    while i < 2 {
        let s = &mut (*chip).streams[i];
        s.bufptr = ((*chip).buffer as *mut u8).offset((s.buf - (*chip).buffer_start) as isize) as *mut c_void;
        s.bufptr_addr = (*chip).buffer_addr + (s.buf - (*chip).buffer_start) as c_ulong;
        i += 1;
    }
    let err = snd_pcm_new((*chip).card, (*(*chip).card).driver.as_ptr(), device, 1, 1, &mut pcm);
    if err < 0 { return err; }
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_nm256_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_nm256_capture_ops);
    (*pcm).private_data = chip as *mut c_void;
    (*pcm).info_flags = 0;
    (*chip).pcm = pcm;
    0
}

unsafe fn snd_nm256_init_chip(chip: *mut nm256) {
    /* Reset everything. */
    snd_nm256_writeb(chip, 0x0, 0x11);
    snd_nm256_writew(chip, 0x214, 0);
    /* stop sounds.. */
    // snd_nm256_playback_stop(chip);
    // snd_nm256_capture_stop(chip);
}

unsafe fn snd_nm256_intr_check(chip: *mut nm256) -> irqreturn_t {
    (*chip).badintrcount += 1;
    if (*chip).badintrcount - 1 > 1000 {
        if (*chip).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].running != 0 { snd_nm256_playback_stop(chip); }
        if (*chip).streams[SNDRV_PCM_STREAM_CAPTURE as usize].running != 0 { snd_nm256_capture_stop(chip); }
        (*chip).badintrcount = 0;
        return IRQ_HANDLED;
    }
    IRQ_NONE
}

unsafe extern "C" fn snd_nm256_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let chip = dev_id as *mut nm256;
    let mut status = snd_nm256_readw(chip, NM_INT_REG);
    let mut cbyte: u8;
    if status == 0 { return snd_nm256_intr_check(chip); }
    (*chip).badintrcount = 0;
    spin_lock(&mut (*chip).reg_lock);
    if status as u32 & NM_PLAYBACK_INT != 0 { status &= !(NM_PLAYBACK_INT as u16); NM_ACK_INT(chip, NM_PLAYBACK_INT); snd_nm256_playback_update(chip); }
    if status as u32 & NM_RECORD_INT != 0 { status &= !(NM_RECORD_INT as u16); NM_ACK_INT(chip, NM_RECORD_INT); snd_nm256_capture_update(chip); }
    if status as u32 & NM_MISC_INT_1 != 0 { status &= !(NM_MISC_INT_1 as u16); NM_ACK_INT(chip, NM_MISC_INT_1); dev_dbg((*(*chip).card).dev, c"NM256: Got misc interrupt #1\n".as_ptr()); snd_nm256_writew(chip, NM_INT_REG, 0x8000); cbyte = snd_nm256_readb(chip, 0x400); snd_nm256_writeb(chip, 0x400, cbyte | 2); }
    if status as u32 & NM_MISC_INT_2 != 0 { status &= !(NM_MISC_INT_2 as u16); NM_ACK_INT(chip, NM_MISC_INT_2); dev_dbg((*(*chip).card).dev, c"NM256: Got misc interrupt #2\n".as_ptr()); cbyte = snd_nm256_readb(chip, 0x400); snd_nm256_writeb(chip, 0x400, cbyte & !2); }
    if status != 0 { dev_dbg((*(*chip).card).dev, c"NM256: Fire in the hole! Unknown status 0x%x\n".as_ptr(), status as c_uint); NM_ACK_INT(chip, status as u32); }
    spin_unlock(&mut (*chip).reg_lock);
    IRQ_HANDLED
}

unsafe extern "C" fn snd_nm256_interrupt_zx(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let chip = dev_id as *mut nm256;
    let mut status = snd_nm256_readl(chip, NM_INT_REG);
    let mut cbyte: u8;
    if status == 0 { return snd_nm256_intr_check(chip); }
    (*chip).badintrcount = 0;
    spin_lock(&mut (*chip).reg_lock);
    if status & NM2_PLAYBACK_INT != 0 { status &= !NM2_PLAYBACK_INT; NM2_ACK_INT(chip, NM2_PLAYBACK_INT); snd_nm256_playback_update(chip); }
    if status & NM2_RECORD_INT != 0 { status &= !NM2_RECORD_INT; NM2_ACK_INT(chip, NM2_RECORD_INT); snd_nm256_capture_update(chip); }
    if status & NM2_MISC_INT_1 != 0 { status &= !NM2_MISC_INT_1; NM2_ACK_INT(chip, NM2_MISC_INT_1); dev_dbg((*(*chip).card).dev, c"NM256: Got misc interrupt #1\n".as_ptr()); cbyte = snd_nm256_readb(chip, 0x400); snd_nm256_writeb(chip, 0x400, cbyte | 2); }
    if status & NM2_MISC_INT_2 != 0 { status &= !NM2_MISC_INT_2; NM2_ACK_INT(chip, NM2_MISC_INT_2); dev_dbg((*(*chip).card).dev, c"NM256: Got misc interrupt #2\n".as_ptr()); cbyte = snd_nm256_readb(chip, 0x400); snd_nm256_writeb(chip, 0x400, cbyte & !2); }
    if status != 0 { dev_dbg((*(*chip).card).dev, c"NM256: Fire in the hole! Unknown status 0x%x\n".as_ptr(), status); NM2_ACK_INT(chip, status); }
    spin_unlock(&mut (*chip).reg_lock);
    IRQ_HANDLED
}

unsafe fn snd_nm256_ac97_ready(chip: *mut nm256) -> c_int {
    let mut timeout = 10;
    let testaddr = (*chip).mixer_status_offset;
    let testb = (*chip).mixer_status_mask as u16;
    while { let old = timeout; timeout -= 1; old > 0 } {
        if (snd_nm256_readw(chip, testaddr) & testb) == 0 { return 1; }
        udelay(100);
    }
    0
}

#[repr(C)]
#[derive(Copy, Clone)]
struct initialValues { reg: u16, value: u16 }

static nm256_ac97_init_val: [initialValues; 17] = [
    initialValues { reg: AC97_MASTER, value: 0x8000 }, initialValues { reg: AC97_HEADPHONE, value: 0x8000 }, initialValues { reg: AC97_MASTER_MONO, value: 0x8000 }, initialValues { reg: AC97_PC_BEEP, value: 0x8000 }, initialValues { reg: AC97_PHONE, value: 0x8008 }, initialValues { reg: AC97_MIC, value: 0x8000 }, initialValues { reg: AC97_LINE, value: 0x8808 }, initialValues { reg: AC97_CD, value: 0x8808 }, initialValues { reg: AC97_VIDEO, value: 0x8808 }, initialValues { reg: AC97_AUX, value: 0x8808 }, initialValues { reg: AC97_PCM, value: 0x8808 }, initialValues { reg: AC97_REC_SEL, value: 0x0000 }, initialValues { reg: AC97_REC_GAIN, value: 0x0B0B }, initialValues { reg: AC97_GENERAL_PURPOSE, value: 0x0000 }, initialValues { reg: AC97_3D_CONTROL, value: 0x8000 }, initialValues { reg: AC97_VENDOR_ID1, value: 0x8384 }, initialValues { reg: AC97_VENDOR_ID2, value: 0x7609 },
];

unsafe fn nm256_ac97_idx(reg: u16) -> c_int {
    let mut i = 0usize;
    while i < nm256_ac97_init_val.len() {
        if nm256_ac97_init_val[i].reg == reg { return i as c_int; }
        i += 1;
    }
    -1
}

unsafe extern "C" fn snd_nm256_ac97_read(ac97: *mut snd_ac97, reg: u16) -> u16 {
    let chip = (*ac97).private_data as *mut nm256;
    let idx = nm256_ac97_idx(reg);
    if idx < 0 { return 0; }
    *(*chip).ac97_regs.offset(idx as isize)
}

unsafe extern "C" fn snd_nm256_ac97_write(ac97: *mut snd_ac97, reg: u16, val: u16) {
    let chip = (*ac97).private_data as *mut nm256;
    let mut tries = 2;
    let idx = nm256_ac97_idx(reg);
    if idx < 0 { return; }
    let base = (*chip).mixer_base;
    snd_nm256_ac97_ready(chip);
    while { let old = tries; tries -= 1; old > 0 } {
        snd_nm256_writew(chip, base + reg as c_int, val);
        msleep(1);
        if snd_nm256_ac97_ready(chip) != 0 {
            *(*chip).ac97_regs.offset(idx as isize) = val;
            return;
        }
    }
    dev_dbg((*(*chip).card).dev, c"nm256: ac97 codec not ready..\n".as_ptr());
}

static nm256_res_table: [snd_ac97_res_table; 13] = [
    snd_ac97_res_table { reg: AC97_MASTER, bits: 0x1f1f }, snd_ac97_res_table { reg: AC97_HEADPHONE, bits: 0x1f1f }, snd_ac97_res_table { reg: AC97_MASTER_MONO, bits: 0x001f }, snd_ac97_res_table { reg: AC97_PC_BEEP, bits: 0x001f }, snd_ac97_res_table { reg: AC97_PHONE, bits: 0x001f }, snd_ac97_res_table { reg: AC97_MIC, bits: 0x001f }, snd_ac97_res_table { reg: AC97_LINE, bits: 0x1f1f }, snd_ac97_res_table { reg: AC97_CD, bits: 0x1f1f }, snd_ac97_res_table { reg: AC97_VIDEO, bits: 0x1f1f }, snd_ac97_res_table { reg: AC97_AUX, bits: 0x1f1f }, snd_ac97_res_table { reg: AC97_PCM, bits: 0x1f1f }, snd_ac97_res_table { reg: AC97_REC_GAIN, bits: 0x0f0f }, snd_ac97_res_table { reg: 0, bits: 0 },
];

unsafe extern "C" fn snd_nm256_ac97_reset(ac97: *mut snd_ac97) {
    let chip = (*ac97).private_data as *mut nm256;
    snd_nm256_writeb(chip, 0x6c0, 1);
    if (*chip).reset_workaround == 0 { snd_nm256_writeb(chip, 0x6cc, 0x87); }
    if (*chip).reset_workaround_2 == 0 { snd_nm256_writeb(chip, 0x6cc, 0x80); snd_nm256_writeb(chip, 0x6cc, 0x0); }
    if (*chip).in_resume == 0 {
        let mut i = 0usize;
        while i < nm256_ac97_init_val.len() {
            snd_nm256_ac97_write(ac97, nm256_ac97_init_val[i].reg, nm256_ac97_init_val[i].value);
            i += 1;
        }
    }
}

unsafe fn snd_nm256_mixer(chip: *mut nm256) -> c_int {
    let mut pbus: *mut snd_ac97_bus = ptr::null_mut();
    let mut ac97: snd_ac97_template = mem::zeroed();
    static ops: snd_ac97_bus_ops = snd_ac97_bus_ops { reset: Some(snd_nm256_ac97_reset), write: Some(snd_nm256_ac97_write), read: Some(snd_nm256_ac97_read) };
    (*chip).ac97_regs = devm_kcalloc((*(*chip).card).dev, nm256_ac97_init_val.len() as c_ulong, mem::size_of::<u16>() as c_ulong, GFP_KERNEL) as *mut u16;
    if (*chip).ac97_regs.is_null() { return -ENOMEM; }
    let mut err = snd_ac97_bus((*chip).card, 0, &ops, ptr::null_mut(), &mut pbus);
    if err < 0 { return err; }
    ac97.scaps = AC97_SCAP_AUDIO;
    ac97.private_data = chip as *mut c_void;
    ac97.res_table = nm256_res_table.as_ptr();
    (*pbus).no_vra = 1;
    err = snd_ac97_mixer(pbus, &mut ac97, &mut (*chip).ac97);
    if err < 0 { return err; }
    if ((*(*chip).ac97).id & 0xf0000000) == 0 {
        sprintf((*(*chip).card).mixername.as_mut_ptr(), c"%s AC97".as_ptr(), (*(*chip).card).driver.as_ptr());
    }
    0
}

unsafe fn snd_nm256_peek_for_sig(chip: *mut nm256) -> c_int {
    let mut pointer_found = (*chip).buffer_end as c_ulong - 0x1400;
    let temp = ioremap((*chip).buffer_addr + (*chip).buffer_end as c_ulong - 0x400, 16);
    if temp.is_null() {
        dev_err((*(*chip).card).dev, c"Unable to scan for card signature in video RAM\n".as_ptr());
        return -EBUSY;
    }
    let sig = readl(temp);
    if (sig & NM_SIG_MASK) == NM_SIGNATURE {
        let pointer = readl((temp as *mut u8).offset(4) as *mut c_void);
        if pointer == 0xffffffff || pointer < (*chip).buffer_size || pointer > (*chip).buffer_end {
            dev_err((*(*chip).card).dev, c"invalid signature found: 0x%x\n".as_ptr(), pointer);
            iounmap(temp);
            return -ENODEV;
        } else {
            pointer_found = pointer as c_ulong;
            dev_info((*(*chip).card).dev, c"found card signature in video RAM: 0x%x\n".as_ptr(), pointer);
        }
    }
    iounmap(temp);
    (*chip).buffer_end = pointer_found as u32;
    0
}

unsafe extern "C" fn nm256_suspend(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    let chip = (*card).private_data as *mut nm256;
    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    snd_ac97_suspend((*chip).ac97);
    (*chip).coeffs_current = 0;
    0
}

unsafe extern "C" fn nm256_resume(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    let chip = (*card).private_data as *mut nm256;
    (*chip).in_resume = 1;
    snd_nm256_init_chip(chip);
    snd_ac97_resume((*chip).ac97);
    let mut i = 0usize;
    while i < 2 {
        let s = &mut (*chip).streams[i] as *mut nm256_stream;
        if !(*s).substream.is_null() && (*s).suspended != 0 {
            spin_lock_irq(&mut (*chip).reg_lock);
            snd_nm256_set_format(chip, s, (*s).substream);
            spin_unlock_irq(&mut (*chip).reg_lock);
        }
        i += 1;
    }
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    (*chip).in_resume = 0;
    0
}

static nm256_pm: dev_pm_ops = dev_pm_ops { _private: [] };

unsafe extern "C" fn snd_nm256_free(card: *mut snd_card) {
    let chip = (*card).private_data as *mut nm256;
    if (*chip).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].running != 0 { snd_nm256_playback_stop(chip); }
    if (*chip).streams[SNDRV_PCM_STREAM_CAPTURE as usize].running != 0 { snd_nm256_capture_stop(chip); }
}

unsafe fn snd_nm256_create(card: *mut snd_card, pci: *mut pci_dev) -> c_int {
    let chip = (*card).private_data as *mut nm256;
    let mut err = pcim_enable_device(pci);
    if err < 0 { return err; }
    (*chip).card = card;
    (*chip).pci = pci;
    (*chip).use_cache = use_cache as c_uint;
    spin_lock_init(&mut (*chip).reg_lock);
    (*chip).irq = -1;
    mutex_init(&mut (*chip).irq_mutex);
    (*chip).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].bufsize = playback_bufsize * 1024;
    (*chip).streams[SNDRV_PCM_STREAM_CAPTURE as usize].bufsize = capture_bufsize * 1024;
    (*chip).buffer_addr = pci_resource_start(pci, 0);
    (*chip).cport_addr = pci_resource_start(pci, 1);
    err = pcim_request_all_regions(pci, (*card).driver.as_ptr());
    if err < 0 { return err; }
    (*chip).cport = devm_ioremap(&mut (*pci).dev as *mut c_void, (*chip).cport_addr, NM_PORT2_SIZE);
    if (*chip).cport.is_null() {
        dev_err((*card).dev, c"unable to map control port %lx\n".as_ptr(), (*chip).cport_addr);
        return -ENOMEM;
    }
    if strcmp((*card).driver.as_ptr(), c"NM256AV".as_ptr()) == 0 {
        let pval = snd_nm256_readw(chip, NM_MIXER_PRESENCE) as c_int;
        if (pval & NM_PRESENCE_MASK) != NM_PRESENCE_VALUE {
            if !force_ac97 {
                dev_err((*card).dev, c"no ac97 is found!\n".as_ptr());
                dev_err((*card).dev, c"force the driver to load by passing in the module parameter\n".as_ptr());
                dev_err((*card).dev, c" force_ac97=1\n".as_ptr());
                dev_err((*card).dev, c"or try sb16, opl3sa2, or cs423x drivers instead.\n".as_ptr());
                return -ENXIO;
            }
        }
        (*chip).buffer_end = 2560 * 1024;
        (*chip).interrupt = Some(snd_nm256_interrupt);
        (*chip).mixer_status_offset = NM_MIXER_STATUS_OFFSET;
        (*chip).mixer_status_mask = NM_MIXER_READY_MASK;
    } else {
        if snd_nm256_readb(chip, 0xa0b) != 0 { (*chip).buffer_end = 6144 * 1024; } else { (*chip).buffer_end = 4096 * 1024; }
        (*chip).interrupt = Some(snd_nm256_interrupt_zx);
        (*chip).mixer_status_offset = NM2_MIXER_STATUS_OFFSET;
        (*chip).mixer_status_mask = NM2_MIXER_READY_MASK;
    }
    (*chip).buffer_size = ((*chip).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].bufsize + (*chip).streams[SNDRV_PCM_STREAM_CAPTURE as usize].bufsize) as u32;
    if (*chip).use_cache != 0 { (*chip).buffer_size += NM_TOTAL_COEFF_COUNT * 4; } else { (*chip).buffer_size += NM_MAX_PLAYBACK_COEF_SIZE + NM_MAX_RECORD_COEF_SIZE; }
    if buffer_top >= (*chip).buffer_size as c_int && buffer_top < (*chip).buffer_end as c_int {
        (*chip).buffer_end = buffer_top as u32;
    } else {
        err = snd_nm256_peek_for_sig(chip);
        if err < 0 { return err; }
    }
    (*chip).buffer_start = (*chip).buffer_end - (*chip).buffer_size;
    (*chip).buffer_addr += (*chip).buffer_start as c_ulong;
    dev_info((*card).dev, c"Mapping port 1 from 0x%x - 0x%x\n".as_ptr(), (*chip).buffer_start, (*chip).buffer_end);
    (*chip).buffer = devm_ioremap(&mut (*pci).dev as *mut c_void, (*chip).buffer_addr, (*chip).buffer_size as c_ulong);
    if (*chip).buffer.is_null() {
        dev_err((*card).dev, c"unable to map ring buffer at %lx\n".as_ptr(), (*chip).buffer_addr);
        return -ENOMEM;
    }
    let mut addr = (*chip).buffer_start;
    (*chip).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].buf = addr;
    addr += (*chip).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].bufsize as u32;
    (*chip).streams[SNDRV_PCM_STREAM_CAPTURE as usize].buf = addr;
    addr += (*chip).streams[SNDRV_PCM_STREAM_CAPTURE as usize].bufsize as u32;
    if (*chip).use_cache != 0 {
        (*chip).all_coeff_buf = addr;
    } else {
        (*chip).coeff_buf[SNDRV_PCM_STREAM_PLAYBACK as usize] = addr;
        addr += NM_MAX_PLAYBACK_COEF_SIZE;
        (*chip).coeff_buf[SNDRV_PCM_STREAM_CAPTURE as usize] = addr;
    }
    (*chip).mixer_base = NM_MIXER_OFFSET;
    (*chip).coeffs_current = 0;
    snd_nm256_init_chip(chip);
    // pci_set_master(pci); /* needed? */
    0
}

const NM_IGNORED: c_int = 0;
const NM_RESET_WORKAROUND: c_int = 1;
const NM_RESET_WORKAROUND_2: c_int = 2;

static nm256_quirks: [snd_pci_quirk; 5] = [
    snd_pci_quirk { subvendor: 0x103c, subdevice: 0x0007, name: c"HP omnibook 4150".as_ptr(), value: NM_IGNORED },
    snd_pci_quirk { subvendor: 0x104d, subdevice: 0x8041, name: c"Sony PCG-F305".as_ptr(), value: NM_RESET_WORKAROUND },
    snd_pci_quirk { subvendor: 0x1028, subdevice: 0x0080, name: c"Dell Latitude LS".as_ptr(), value: NM_RESET_WORKAROUND },
    snd_pci_quirk { subvendor: 0x1028, subdevice: 0x0091, name: c"Dell Latitude CSx".as_ptr(), value: NM_RESET_WORKAROUND_2 },
    snd_pci_quirk { subvendor: 0, subdevice: 0, name: ptr::null(), value: 0 },
];

unsafe extern "C" fn snd_nm256_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let mut err: c_int;
    let q = snd_pci_quirk_lookup(pci, nm256_quirks.as_ptr());
    if !q.is_null() {
        dev_dbg(&mut (*pci).dev as *mut c_void, c"Enabled quirk for %s.\n".as_ptr(), snd_pci_quirk_name(q));
        match (*q).value {
            NM_IGNORED => {
                dev_info(&mut (*pci).dev as *mut c_void, c"The device is on the denylist. Loading stopped\n".as_ptr());
                return -ENODEV;
            }
            NM_RESET_WORKAROUND_2 => {
                reset_workaround_2 = true;
                reset_workaround = true;
            }
            NM_RESET_WORKAROUND => { reset_workaround = true; }
            _ => {}
        }
    }
    err = snd_devm_card_new(&mut (*pci).dev as *mut c_void, index, id, THIS_MODULE, mem::size_of::<nm256>() as c_int, &mut card);
    if err < 0 { return err; }
    let chip = (*card).private_data as *mut nm256;
    match (*pci).device {
        PCI_DEVICE_ID_NEOMAGIC_NM256AV_AUDIO => { strscpy((*card).driver.as_mut_ptr(), c"NM256AV".as_ptr()); }
        PCI_DEVICE_ID_NEOMAGIC_NM256ZX_AUDIO => { strscpy((*card).driver.as_mut_ptr(), c"NM256ZX".as_ptr()); }
        PCI_DEVICE_ID_NEOMAGIC_NM256XL_PLUS_AUDIO => { strscpy((*card).driver.as_mut_ptr(), c"NM256XL+".as_ptr()); }
        _ => {
            dev_err(&mut (*pci).dev as *mut c_void, c"invalid device id 0x%x\n".as_ptr(), (*pci).device);
            return -EINVAL;
        }
    }
    if vaio_hack { buffer_top = 0x25a800; }
    if playback_bufsize < 4 { playback_bufsize = 4; }
    if playback_bufsize > 128 { playback_bufsize = 128; }
    if capture_bufsize < 4 { capture_bufsize = 4; }
    if capture_bufsize > 128 { capture_bufsize = 128; }
    err = snd_nm256_create(card, pci);
    if err < 0 { return err; }
    if reset_workaround {
        dev_dbg(&mut (*pci).dev as *mut c_void, c"reset_workaround activated\n".as_ptr());
        (*chip).reset_workaround = 1;
    }
    if reset_workaround_2 {
        dev_dbg(&mut (*pci).dev as *mut c_void, c"reset_workaround_2 activated\n".as_ptr());
        (*chip).reset_workaround_2 = 1;
    }
    err = snd_nm256_pcm(chip, 0);
    if err < 0 { return err; }
    err = snd_nm256_mixer(chip);
    if err < 0 { return err; }
    sprintf((*card).shortname.as_mut_ptr(), c"NeoMagic %s".as_ptr(), (*card).driver.as_ptr());
    sprintf((*card).longname.as_mut_ptr(), c"%s at 0x%lx & 0x%lx, irq %d".as_ptr(), (*card).shortname.as_ptr(), (*chip).buffer_addr, (*chip).cport_addr, (*chip).irq);
    err = snd_card_register(card);
    if err < 0 { return err; }
    (*card).private_free = Some(snd_nm256_free);
    pci_set_drvdata(pci, card as *mut c_void);
    let _ = pci_id;
    0
}

static nm256_driver: pci_driver = pci_driver {
    name: KBUILD_MODNAME,
    id_table: snd_nm256_ids.as_ptr(),
    probe: Some(snd_nm256_probe),
    driver: pci_driver_inner { pm: &nm256_pm },
};

// module metadata and registration translated from:
// MODULE_AUTHOR("Takashi Iwai <tiwai@suse.de>");
// MODULE_DESCRIPTION("NeoMagic NM256AV/ZX");
// MODULE_LICENSE("GPL");
// MODULE_DEVICE_TABLE(pci, snd_nm256_ids);
// module_pci_driver(nm256_driver);

extern "C" {
    static SNDRV_DEFAULT_STR1: *const c_char;
    static KBUILD_MODNAME: *const c_char;
    static THIS_MODULE: *mut module;

    fn readb(addr: *mut u8) -> u8;
    fn readw(addr: *mut c_void) -> u16;
    fn readl(addr: *mut c_void) -> u32;
    fn writeb(val: u8, addr: *mut u8);
    fn writew(val: u16, addr: *mut c_void);
    fn writel(val: u32, addr: *mut c_void);
    fn memcpy_toio(dst: *mut c_void, src: *const c_void, count: c_ulong);
    fn memset_io(dst: *mut c_void, val: c_int, count: c_ulong);
    fn copy_from_iter_toio(dst: *mut c_void, count: c_ulong, src: *mut iov_iter) -> c_ulong;
    fn copy_to_iter_fromio(src: *mut c_void, count: c_ulong, dst: *mut iov_iter) -> c_ulong;
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_info(dev: *mut c_void, fmt: *const c_char, ...);
    fn snd_BUG();
    fn snd_BUG_ON(cond: c_int) -> c_int;
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn request_irq(irq: c_int, handler: irq_handler_t, flags: c_ulong, name: *const c_char, dev: *mut c_void) -> c_int;
    fn free_irq(irq: c_int, dev: *mut c_void);
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
    fn mutex_init(mutex: *mut mutex);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut c_void;
    fn frames_to_bytes(runtime: *mut snd_pcm_runtime, frames: snd_pcm_uframes_t) -> c_int;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: c_ulong) -> snd_pcm_uframes_t;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn params_buffer_bytes(hw_params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_uint, l: *const snd_pcm_hw_constraint_list) -> c_int;
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback_count: c_int, capture_count: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_lib_mmap_iomem() -> c_int;
    fn udelay(usecs: c_ulong);
    fn msleep(msecs: c_uint);
    fn devm_kcalloc(dev: *mut c_void, n: c_ulong, size: c_ulong, flags: c_uint) -> *mut c_void;
    fn snd_ac97_bus(card: *mut snd_card, num: c_int, ops: *const snd_ac97_bus_ops, private_data: *mut c_void, rbus: *mut *mut snd_ac97_bus) -> c_int;
    fn snd_ac97_mixer(bus: *mut snd_ac97_bus, template: *mut snd_ac97_template, rac97: *mut *mut snd_ac97) -> c_int;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn ioremap(offset: c_ulong, size: c_ulong) -> *mut c_void;
    fn iounmap(addr: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_power_change_state(card: *mut snd_card, state: c_int) -> c_int;
    fn snd_ac97_suspend(ac97: *mut snd_ac97);
    fn snd_ac97_resume(ac97: *mut snd_ac97);
    fn pcim_enable_device(pci: *mut pci_dev) -> c_int;
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> c_ulong;
    fn pcim_request_all_regions(pci: *mut pci_dev, name: *const c_char) -> c_int;
    fn devm_ioremap(dev: *mut c_void, offset: c_ulong, size: c_ulong) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn snd_pci_quirk_lookup(pci: *mut pci_dev, list: *const snd_pci_quirk) -> *const snd_pci_quirk;
    fn snd_pci_quirk_name(q: *const snd_pci_quirk) -> *const c_char;
    fn snd_devm_card_new(dev: *mut c_void, idx: c_int, xid: *mut c_char, module: *mut module, extra_size: c_int, card_ret: *mut *mut snd_card) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> c_long;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
}

type c_long = i64;

const SNDRV_DEFAULT_IDX1: c_int = -1;
const PCI_ANY_ID: c_uint = !0;
const PCI_VENDOR_ID_NEOMAGIC: c_uint = 0x10c8;
const PCI_DEVICE_ID_NEOMAGIC_NM256AV_AUDIO: c_uint = 0x8005;
const PCI_DEVICE_ID_NEOMAGIC_NM256ZX_AUDIO: c_uint = 0x8006;
const PCI_DEVICE_ID_NEOMAGIC_NM256XL_PLUS_AUDIO: c_uint = 0x8016;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const IRQF_SHARED: c_ulong = 0x80;
const EBUSY: c_int = 16;
const ENXIO: c_int = 6;
const EINVAL: c_int = 22;
const EFAULT: c_int = 14;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQ_NONE: irqreturn_t = 0;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 4;
const SNDRV_PCM_TRIGGER_START: c_int = 1;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_STOP: c_int = 0;
const SNDRV_PCM_INFO_MMAP_IOMEM: c_uint = 1 << 0;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 1;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 2;
const SNDRV_PCM_INFO_RESUME: c_uint = 1 << 3;
const SNDRV_PCM_FMTBIT_U8: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 1;
const SNDRV_PCM_RATE_KNOT: c_uint = 1 << 0;
const SNDRV_PCM_RATE_8000_48000: c_uint = 1 << 1;
const SNDRV_PCM_HW_PARAM_RATE: c_uint = 0;
const GFP_KERNEL: c_uint = 0;
const AC97_MASTER: u16 = 0x02;
const AC97_HEADPHONE: u16 = 0x04;
const AC97_MASTER_MONO: u16 = 0x06;
const AC97_PC_BEEP: u16 = 0x0a;
const AC97_PHONE: u16 = 0x0c;
const AC97_MIC: u16 = 0x0e;
const AC97_LINE: u16 = 0x10;
const AC97_CD: u16 = 0x12;
const AC97_VIDEO: u16 = 0x14;
const AC97_AUX: u16 = 0x16;
const AC97_PCM: u16 = 0x18;
const AC97_REC_SEL: u16 = 0x1a;
const AC97_REC_GAIN: u16 = 0x1c;
const AC97_GENERAL_PURPOSE: u16 = 0x20;
const AC97_3D_CONTROL: u16 = 0x22;
const AC97_VENDOR_ID1: u16 = 0x7c;
const AC97_VENDOR_ID2: u16 = 0x7e;
const AC97_SCAP_AUDIO: c_uint = 1;
const SNDRV_CTL_POWER_D3hot: c_int = 3;
const SNDRV_CTL_POWER_D0: c_int = 0;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
