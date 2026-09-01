// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Support for Digigram Lola PCI-e boards
 *
 *  Copyright (c) 2011 Takashi Iwai <tiwai@suse.de>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const LOLA_MAX_BDL_ENTRIES: c_int = 8;
const LOLA_MAX_BUF_SIZE: c_uint = 1024 * 1024 * 1024;
const LOLA_BDL_ENTRY_SIZE: usize = 16 * 16;

type bool_ = bool;
type u8 = u8;
type u32 = u32;
type __le32 = u32;
type dma_addr_t = u64;
type snd_pcm_uframes_t = c_ulong;

#[repr(C)]
pub struct lola {
    pub pcm: [lola_pcm; 2],
    pub granularity: c_uint,
    pub card: *mut snd_card,
    pub open_mutex: mutex,
    pub sample_rate: c_uint,
    pub sample_rate_min: c_uint,
    pub sample_rate_max: c_uint,
    pub ref_count_rate: c_int,
    pub reg_lock: spinlock_t,
    pub pci: *mut pci_dev,
    pub input_src_caps_mask: c_uint,
}

#[repr(C)]
pub struct lola_pcm {
    pub streams: [lola_stream; 32],
    pub num_streams: u8,
    pub bdl: *mut snd_dma_buffer,
}

#[repr(C)]
pub struct lola_stream {
    pub nid: c_int,
    pub index: c_int,
    pub dsd: c_int,
    pub opened: c_int,
    pub prepared: c_int,
    pub paused: c_int,
    pub running: c_uint,
    pub can_float: bool_,
    pub substream: *mut snd_pcm_substream,
    pub master: *mut lola_stream,
    pub bufsize: c_uint,
    pub period_bytes: c_uint,
    pub format_verb: c_uint,
    pub frags: c_int,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
    pub number: c_uint,
    pub runtime: *mut snd_pcm_runtime,
    pub pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw: snd_pcm_hardware,
    pub format: c_int,
    pub channels: c_uint,
    pub rate: c_uint,
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
    pub buffer_bytes_max: c_uint,
    pub period_bytes_min: c_uint,
    pub period_bytes_max: c_uint,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub fifo_size: c_uint,
}

#[repr(C)]
pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}

#[repr(C)]
pub struct snd_dma_buffer {
    pub area: *mut c_void,
    pub addr: dma_addr_t,
}

#[repr(C)]
pub struct snd_pcm {
    pub card: *mut snd_card,
    pub name: [c_char; 80],
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

extern "C" {
    static mut jiffies: c_ulong;

    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut lola;
    fn lola_readl(chip: *mut lola, bar: c_int, reg: c_int) -> c_uint;
    fn lola_dsd_read(chip: *mut lola, dsd: c_int, reg: c_int) -> c_uint;
    fn lola_dsd_write(chip: *mut lola, dsd: c_int, reg: c_int, val: c_uint);
    fn msecs_to_jiffies(m: c_uint) -> c_ulong;
    fn time_before(a: c_ulong, b: c_ulong) -> bool_;
    fn msleep(msecs: c_uint);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn snd_pcm_group_for_each_entry(
        current: *mut snd_pcm_substream,
        substream: *mut snd_pcm_substream,
    ) -> *mut snd_pcm_substream;
    fn snd_pcm_trigger_done(s: *mut snd_pcm_substream, master: *mut snd_pcm_substream);
    fn snd_pcm_stream_linked(substream: *mut snd_pcm_substream) -> bool_;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, param: c_int) -> c_int;
    fn snd_pcm_hw_constraint_step(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        param: c_int,
        step: c_uint,
    ) -> c_int;
    fn snd_pcm_sgbuf_get_addr(substream: *mut snd_pcm_substream, ofs: c_int) -> dma_addr_t;
    fn snd_pcm_sgbuf_get_chunk_size(
        substream: *mut snd_pcm_substream,
        ofs: c_int,
        size: c_int,
    ) -> c_int;
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, size: c_uint) -> snd_pcm_uframes_t;
    fn lola_codec_read(
        chip: *mut lola,
        nid: c_int,
        verb: c_uint,
        data: c_uint,
        extdata: c_uint,
        val: *mut c_uint,
        extval: *mut c_void,
    ) -> c_int;
    fn lola_set_sample_rate(chip: *mut lola, rate: c_uint) -> c_int;
    fn lola_read_param(chip: *mut lola, nid: c_int, param: c_uint, val: *mut c_uint) -> c_int;
    fn snd_devm_alloc_pages(dev: *mut device, ty: c_int, size: usize) -> *mut snd_dma_buffer;
    fn snd_pcm_new(
        card: *mut snd_card,
        id: *const c_char,
        device: c_int,
        playback_count: c_int,
        capture_count: c_int,
        rpcm: *mut *mut snd_pcm,
    ) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        ty: c_int,
        dev: *mut device,
        size: usize,
        max: usize,
    );
}

extern "C" {
    static SNDRV_PCM_INFO_MMAP: c_uint;
    static SNDRV_PCM_INFO_INTERLEAVED: c_uint;
    static SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint;
    static SNDRV_PCM_INFO_MMAP_VALID: c_uint;
    static SNDRV_PCM_INFO_PAUSE: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static SNDRV_PCM_FMTBIT_FLOAT_LE: u64;
    static SNDRV_PCM_RATE_8000_192000: c_uint;
    static SNDRV_PCM_FORMAT_S16_LE: c_int;
    static SNDRV_PCM_FORMAT_S24_LE: c_int;
    static SNDRV_PCM_FORMAT_S32_LE: c_int;
    static SNDRV_PCM_FORMAT_FLOAT_LE: c_int;
    static SNDRV_PCM_HW_PARAM_PERIODS: c_int;
    static SNDRV_PCM_HW_PARAM_BUFFER_SIZE: c_int;
    static SNDRV_PCM_HW_PARAM_PERIOD_SIZE: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static SNDRV_DMA_TYPE_DEV: c_int;
    static SNDRV_DMA_TYPE_DEV_SG: c_int;
    static PAGE_SIZE: usize;
    static BAR1: c_int;
    static LRC: c_int;
    static STS: c_int;
    static CTL: c_int;
    static LVI: c_int;
    static BDPU: c_int;
    static BDPL: c_int;
    static LPIB: c_int;
    static LOLA_DSD_STS_DESE: c_uint;
    static LOLA_DSD_STS_BCIS: c_uint;
    static LOLA_DSD_STS_FIFORDY: c_uint;
    static LOLA_DSD_CTL_SRUN: c_uint;
    static LOLA_DSD_CTL_IOCE: c_uint;
    static LOLA_DSD_CTL_DEIE: c_uint;
    static LOLA_DSD_CTL_VLRCV: c_uint;
    static LOLA_DSD_CTL_SRST: c_uint;
    static LOLA_VERB_SET_STREAM_FORMAT: c_uint;
    static LOLA_VERB_SET_CHANNEL_STREAMID: c_uint;
    static LOLA_PAR_AUDIO_WIDGET_CAP: c_uint;
    static LOLA_PAR_STREAM_FORMATS: c_uint;
    static PLAY: c_int;
    static MAX_STREAM_IN_COUNT: c_int;
    static EBUSY: c_int;
    static EIO: c_int;
    static EINVAL: c_int;
    static ENOMEM: c_int;
}

unsafe fn upper_32_bits(n: dma_addr_t) -> u32 {
    (n >> 32) as u32
}

unsafe fn cpu_to_le32(n: u32) -> __le32 {
    n.to_le()
}

unsafe fn lola_get_pcm(substream: *mut snd_pcm_substream) -> *mut lola_pcm {
    let chip = snd_pcm_substream_chip(substream);
    &mut (*chip).pcm[(*substream).stream as usize]
}

unsafe fn lola_get_stream(substream: *mut snd_pcm_substream) -> *mut lola_stream {
    let pcm = lola_get_pcm(substream);
    let idx = (*substream).number;
    &mut (*pcm).streams[idx as usize]
}

unsafe fn lola_get_lrc(chip: *mut lola) -> c_uint {
    lola_readl(chip, BAR1, LRC)
}

unsafe fn lola_get_tstamp(chip: *mut lola, quick_no_sync: bool_) -> c_uint {
    let mut tstamp = lola_get_lrc(chip) >> 8;
    if (*chip).granularity != 0 {
        let wait_banks = if quick_no_sync { 0 } else { 8 };
        tstamp = tstamp.wrapping_add((wait_banks + 1) * (*chip).granularity - 1);
        tstamp = tstamp.wrapping_sub(tstamp % (*chip).granularity);
    }
    tstamp << 8
}

/* clear any pending interrupt status */
unsafe fn lola_stream_clear_pending_irq(chip: *mut lola, str_: *mut lola_stream) {
    let mut val = lola_dsd_read(chip, (*str_).dsd, STS);
    val &= LOLA_DSD_STS_DESE | LOLA_DSD_STS_BCIS;
    if val != 0 {
        lola_dsd_write(chip, (*str_).dsd, STS, val);
    }
}

unsafe fn lola_stream_start(chip: *mut lola, str_: *mut lola_stream, tstamp: c_uint) {
    lola_stream_clear_pending_irq(chip, str_);
    lola_dsd_write(
        chip,
        (*str_).dsd,
        CTL,
        LOLA_DSD_CTL_SRUN
            | LOLA_DSD_CTL_IOCE
            | LOLA_DSD_CTL_DEIE
            | LOLA_DSD_CTL_VLRCV
            | tstamp,
    );
}

unsafe fn lola_stream_stop(chip: *mut lola, str_: *mut lola_stream, tstamp: c_uint) {
    lola_dsd_write(
        chip,
        (*str_).dsd,
        CTL,
        LOLA_DSD_CTL_IOCE | LOLA_DSD_CTL_DEIE | LOLA_DSD_CTL_VLRCV | tstamp,
    );
    lola_stream_clear_pending_irq(chip, str_);
}

unsafe fn wait_for_srst_clear(chip: *mut lola, str_: *mut lola_stream) {
    let end_time = jiffies.wrapping_add(msecs_to_jiffies(200));
    while time_before(jiffies, end_time) {
        let val = lola_dsd_read(chip, (*str_).dsd, CTL);
        if (val & LOLA_DSD_CTL_SRST) == 0 {
            return;
        }
        msleep(1);
    }
    dev_warn(
        (*(*chip).card).dev,
        b"SRST not clear (stream %d)\n\0".as_ptr() as *const c_char,
        (*str_).dsd,
    );
}

unsafe fn lola_stream_wait_for_fifo(
    chip: *mut lola,
    str_: *mut lola_stream,
    ready: bool_,
) -> c_int {
    let val = if ready { LOLA_DSD_STS_FIFORDY } else { 0 };
    let end_time = jiffies.wrapping_add(msecs_to_jiffies(200));
    while time_before(jiffies, end_time) {
        let reg = lola_dsd_read(chip, (*str_).dsd, STS);
        if (reg & LOLA_DSD_STS_FIFORDY) == val {
            return 0;
        }
        msleep(1);
    }
    dev_warn(
        (*(*chip).card).dev,
        b"FIFO not ready (stream %d)\n\0".as_ptr() as *const c_char,
        (*str_).dsd,
    );
    -EIO
}

/* sync for FIFO ready/empty for all linked streams;
 * clear paused flag when FIFO gets ready again
 */
unsafe fn lola_sync_wait_for_fifo(
    chip: *mut lola,
    substream: *mut snd_pcm_substream,
    ready: bool_,
) -> c_int {
    let val = if ready { LOLA_DSD_STS_FIFORDY } else { 0 };
    let end_time = jiffies.wrapping_add(msecs_to_jiffies(200));
    let mut pending: c_int = 0;

    while time_before(jiffies, end_time) {
        pending = 0;
        let mut s = snd_pcm_group_for_each_entry(core::ptr::null_mut(), substream);
        while !s.is_null() {
            if (*(*s).pcm).card != (*(*substream).pcm).card {
                s = snd_pcm_group_for_each_entry(s, substream);
                continue;
            }
            let str_ = lola_get_stream(s);
            if (*str_).prepared != 0 && (*str_).paused != 0 {
                let reg = lola_dsd_read(chip, (*str_).dsd, STS);
                if (reg & LOLA_DSD_STS_FIFORDY) != val {
                    pending = (*str_).dsd + 1;
                    break;
                }
                if ready {
                    (*str_).paused = 0;
                }
            }
            s = snd_pcm_group_for_each_entry(s, substream);
        }
        if pending == 0 {
            return 0;
        }
        msleep(1);
    }
    dev_warn(
        (*(*chip).card).dev,
        b"FIFO not ready (pending %d)\n\0".as_ptr() as *const c_char,
        pending - 1,
    );
    -EIO
}

/* finish pause - prepare for a new resume */
unsafe fn lola_sync_pause(chip: *mut lola, substream: *mut snd_pcm_substream) {
    lola_sync_wait_for_fifo(chip, substream, false);
    let mut s = snd_pcm_group_for_each_entry(core::ptr::null_mut(), substream);
    while !s.is_null() {
        if (*(*s).pcm).card != (*(*substream).pcm).card {
            s = snd_pcm_group_for_each_entry(s, substream);
            continue;
        }
        let str_ = lola_get_stream(s);
        if (*str_).paused != 0 && (*str_).prepared != 0 {
            lola_dsd_write(
                chip,
                (*str_).dsd,
                CTL,
                LOLA_DSD_CTL_SRUN | LOLA_DSD_CTL_IOCE | LOLA_DSD_CTL_DEIE,
            );
        }
        s = snd_pcm_group_for_each_entry(s, substream);
    }
    lola_sync_wait_for_fifo(chip, substream, true);
}

unsafe fn lola_stream_reset(chip: *mut lola, str_: *mut lola_stream) {
    if (*str_).prepared != 0 {
        if (*str_).paused != 0 {
            lola_sync_pause(chip, (*str_).substream);
        }
        (*str_).prepared = 0;
        lola_dsd_write(chip, (*str_).dsd, CTL, LOLA_DSD_CTL_IOCE | LOLA_DSD_CTL_DEIE);
        lola_stream_wait_for_fifo(chip, str_, false);
        lola_stream_clear_pending_irq(chip, str_);
        lola_dsd_write(chip, (*str_).dsd, CTL, LOLA_DSD_CTL_SRST);
        lola_dsd_write(chip, (*str_).dsd, LVI, 0);
        lola_dsd_write(chip, (*str_).dsd, BDPU, 0);
        lola_dsd_write(chip, (*str_).dsd, BDPL, 0);
        wait_for_srst_clear(chip, str_);
    }
}

unsafe fn lola_pcm_hw() -> snd_pcm_hardware {
    snd_pcm_hardware {
        info: SNDRV_PCM_INFO_MMAP
            | SNDRV_PCM_INFO_INTERLEAVED
            | SNDRV_PCM_INFO_BLOCK_TRANSFER
            | SNDRV_PCM_INFO_MMAP_VALID
            | SNDRV_PCM_INFO_PAUSE,
        formats: SNDRV_PCM_FMTBIT_S16_LE
            | SNDRV_PCM_FMTBIT_S24_LE
            | SNDRV_PCM_FMTBIT_S32_LE
            | SNDRV_PCM_FMTBIT_FLOAT_LE,
        rates: SNDRV_PCM_RATE_8000_192000,
        rate_min: 8000,
        rate_max: 192000,
        channels_min: 1,
        channels_max: 2,
        buffer_bytes_max: LOLA_MAX_BUF_SIZE,
        period_bytes_min: 128,
        period_bytes_max: LOLA_MAX_BUF_SIZE / 2,
        periods_min: 2,
        periods_max: LOLA_MAX_BDL_ENTRIES as c_uint,
        fifo_size: 0,
    }
}

unsafe extern "C" fn lola_pcm_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let pcm = lola_get_pcm(substream);
    let str_ = lola_get_stream(substream);
    let runtime = (*substream).runtime;

    mutex_lock(&mut (*chip).open_mutex);
    if (*str_).opened != 0 {
        mutex_unlock(&mut (*chip).open_mutex);
        return -EBUSY;
    }
    (*str_).substream = substream;
    (*str_).master = core::ptr::null_mut();
    (*str_).opened = 1;
    (*runtime).hw = lola_pcm_hw();
    (*runtime).hw.channels_max = ((*pcm).num_streams as c_int - (*str_).index) as c_uint;
    if (*chip).sample_rate != 0 {
        /* sample rate is locked */
        (*runtime).hw.rate_min = (*chip).sample_rate;
        (*runtime).hw.rate_max = (*chip).sample_rate;
    } else {
        (*runtime).hw.rate_min = (*chip).sample_rate_min;
        (*runtime).hw.rate_max = (*chip).sample_rate_max;
    }
    (*chip).ref_count_rate += 1;
    snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    /* period size = multiple of chip->granularity (8, 16 or 32 frames)*/
    snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_BUFFER_SIZE, (*chip).granularity);
    snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_SIZE, (*chip).granularity);
    mutex_unlock(&mut (*chip).open_mutex);
    0
}

unsafe fn lola_cleanup_slave_streams(pcm: *mut lola_pcm, str_: *mut lola_stream) {
    let mut i = (*str_).index + 1;
    while i < (*pcm).num_streams as c_int {
        let s = &mut (*pcm).streams[i as usize] as *mut lola_stream;
        if (*s).master != str_ {
            break;
        }
        (*s).master = core::ptr::null_mut();
        (*s).opened = 0;
        i += 1;
    }
}

unsafe extern "C" fn lola_pcm_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let str_ = lola_get_stream(substream);

    mutex_lock(&mut (*chip).open_mutex);
    if (*str_).substream == substream {
        (*str_).substream = core::ptr::null_mut();
        (*str_).opened = 0;
    }
    (*chip).ref_count_rate -= 1;
    if (*chip).ref_count_rate == 0 {
        /* release sample rate */
        (*chip).sample_rate = 0;
    }
    mutex_unlock(&mut (*chip).open_mutex);
    0
}

unsafe extern "C" fn lola_pcm_hw_params(
    substream: *mut snd_pcm_substream,
    _hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let str_ = lola_get_stream(substream);

    (*str_).bufsize = 0;
    (*str_).period_bytes = 0;
    (*str_).format_verb = 0;
    0
}

unsafe extern "C" fn lola_pcm_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let pcm = lola_get_pcm(substream);
    let str_ = lola_get_stream(substream);

    mutex_lock(&mut (*chip).open_mutex);
    lola_stream_reset(chip, str_);
    lola_cleanup_slave_streams(pcm, str_);
    mutex_unlock(&mut (*chip).open_mutex);
    0
}

/*
 * set up a BDL entry
 */
unsafe fn setup_bdle(
    substream: *mut snd_pcm_substream,
    str_: *mut lola_stream,
    bdlp: *mut *mut __le32,
    mut ofs: c_int,
    mut size: c_int,
) -> c_int {
    let mut bdl = *bdlp;

    while size > 0 {
        let addr: dma_addr_t;
        let chunk: c_int;

        if (*str_).frags >= LOLA_MAX_BDL_ENTRIES {
            return -EINVAL;
        }

        addr = snd_pcm_sgbuf_get_addr(substream, ofs);
        /* program the address field of the BDL entry */
        *bdl.add(0) = cpu_to_le32(addr as u32);
        *bdl.add(1) = cpu_to_le32(upper_32_bits(addr));
        /* program the size field of the BDL entry */
        chunk = snd_pcm_sgbuf_get_chunk_size(substream, ofs, size);
        *bdl.add(2) = cpu_to_le32(chunk as u32);
        /* program the IOC to enable interrupt
         * only when the whole fragment is processed
         */
        size -= chunk;
        *bdl.add(3) = if size != 0 { 0 } else { cpu_to_le32(0x01) };
        bdl = bdl.add(4);
        (*str_).frags += 1;
        ofs += chunk;
    }
    *bdlp = bdl;
    ofs
}

/*
 * set up BDL entries
 */
unsafe fn lola_setup_periods(
    chip: *mut lola,
    pcm: *mut lola_pcm,
    substream: *mut snd_pcm_substream,
    str_: *mut lola_stream,
) -> c_int {
    let period_bytes: c_int = (*str_).period_bytes as c_int;
    let periods: c_int = (*str_).bufsize as c_int / period_bytes;

    /* program the initial BDL entries */
    let mut bdl =
        ((*(*pcm).bdl).area as *mut u8).add(LOLA_BDL_ENTRY_SIZE * (*str_).index as usize)
            as *mut __le32;
    let mut ofs: c_int = 0;
    (*str_).frags = 0;
    let mut i = 0;
    while i < periods {
        ofs = setup_bdle(substream, str_, &mut bdl, ofs, period_bytes);
        if ofs < 0 {
            dev_err(
                (*(*chip).card).dev,
                b"Too many BDL entries: buffer=%d, period=%d\n\0".as_ptr() as *const c_char,
                (*str_).bufsize,
                period_bytes,
            );
            return -EINVAL;
        }
        i += 1;
    }
    0
}

unsafe fn lola_get_format_verb(substream: *mut snd_pcm_substream) -> c_uint {
    let verb: c_uint;

    if (*(*substream).runtime).format == SNDRV_PCM_FORMAT_S16_LE {
        verb = 0x00000000;
    } else if (*(*substream).runtime).format == SNDRV_PCM_FORMAT_S24_LE {
        verb = 0x00000200;
    } else if (*(*substream).runtime).format == SNDRV_PCM_FORMAT_S32_LE {
        verb = 0x00000300;
    } else if (*(*substream).runtime).format == SNDRV_PCM_FORMAT_FLOAT_LE {
        verb = 0x00001300;
    } else {
        return 0;
    }
    verb | (*(*substream).runtime).channels
}

unsafe fn lola_set_stream_config(
    chip: *mut lola,
    str_: *mut lola_stream,
    channels: c_int,
) -> c_int {
    let mut val: c_uint = 0;

    /* set format info for all channels
     * (with only one command for the first channel)
     */
    let mut err = lola_codec_read(
        chip,
        (*str_).nid,
        LOLA_VERB_SET_STREAM_FORMAT,
        (*str_).format_verb,
        0,
        &mut val,
        core::ptr::null_mut(),
    );
    if err < 0 {
        dev_err(
            (*(*chip).card).dev,
            b"Cannot set stream format 0x%x\n\0".as_ptr() as *const c_char,
            (*str_).format_verb,
        );
        return err;
    }

    /* update stream - channel config */
    let mut i = 0;
    while i < channels {
        let verb = (((*str_).index << 6) | i) as c_uint;
        err = lola_codec_read(
            chip,
            (*str_.add(i as usize)).nid,
            LOLA_VERB_SET_CHANNEL_STREAMID,
            0,
            verb,
            &mut val,
            core::ptr::null_mut(),
        );
        if err < 0 {
            dev_err(
                (*(*chip).card).dev,
                b"Cannot set stream channel %d\n\0".as_ptr() as *const c_char,
                i,
            );
            return err;
        }
        i += 1;
    }
    0
}

/*
 * set up the SD for streaming
 */
unsafe fn lola_setup_controller(
    chip: *mut lola,
    pcm: *mut lola_pcm,
    str_: *mut lola_stream,
) -> c_int {
    let bdl: dma_addr_t;

    if (*str_).prepared != 0 {
        return -EINVAL;
    }

    /* set up BDL */
    bdl = (*(*pcm).bdl).addr + (LOLA_BDL_ENTRY_SIZE * (*str_).index as usize) as dma_addr_t;
    lola_dsd_write(chip, (*str_).dsd, BDPL, bdl as u32);
    lola_dsd_write(chip, (*str_).dsd, BDPU, upper_32_bits(bdl));
    /* program the stream LVI (last valid index) of the BDL */
    lola_dsd_write(chip, (*str_).dsd, LVI, ((*str_).frags - 1) as c_uint);
    lola_stream_clear_pending_irq(chip, str_);

    lola_dsd_write(
        chip,
        (*str_).dsd,
        CTL,
        LOLA_DSD_CTL_IOCE | LOLA_DSD_CTL_DEIE | LOLA_DSD_CTL_SRUN,
    );

    (*str_).prepared = 1;

    lola_stream_wait_for_fifo(chip, str_, true)
}

unsafe extern "C" fn lola_pcm_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let pcm = lola_get_pcm(substream);
    let str_ = lola_get_stream(substream);
    let runtime = (*substream).runtime;

    mutex_lock(&mut (*chip).open_mutex);
    lola_stream_reset(chip, str_);
    lola_cleanup_slave_streams(pcm, str_);
    if (*str_).index + (*runtime).channels as c_int > (*pcm).num_streams as c_int {
        mutex_unlock(&mut (*chip).open_mutex);
        return -EINVAL;
    }
    let mut i = 1;
    while i < (*runtime).channels as c_int {
        (*str_.add(i as usize)).master = str_;
        (*str_.add(i as usize)).opened = 1;
        i += 1;
    }
    mutex_unlock(&mut (*chip).open_mutex);

    let bufsize = snd_pcm_lib_buffer_bytes(substream);
    let period_bytes = snd_pcm_lib_period_bytes(substream);
    let format_verb = lola_get_format_verb(substream);

    (*str_).bufsize = bufsize;
    (*str_).period_bytes = period_bytes;
    (*str_).format_verb = format_verb;

    let mut err = lola_setup_periods(chip, pcm, substream, str_);
    if err < 0 {
        return err;
    }

    err = lola_set_sample_rate(chip, (*runtime).rate);
    if err < 0 {
        return err;
    }
    (*chip).sample_rate = (*runtime).rate; /* sample rate gets locked */

    err = lola_set_stream_config(chip, str_, (*runtime).channels as c_int);
    if err < 0 {
        return err;
    }

    err = lola_setup_controller(chip, pcm, str_);
    if err < 0 {
        lola_stream_reset(chip, str_);
        return err;
    }

    0
}

unsafe extern "C" fn lola_pcm_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let start: c_uint;

    if cmd == SNDRV_PCM_TRIGGER_START
        || cmd == SNDRV_PCM_TRIGGER_PAUSE_RELEASE
        || cmd == SNDRV_PCM_TRIGGER_RESUME
    {
        start = 1;
    } else if cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH
        || cmd == SNDRV_PCM_TRIGGER_SUSPEND
        || cmd == SNDRV_PCM_TRIGGER_STOP
    {
        start = 0;
    } else {
        return -EINVAL;
    }

    /*
     * sample correct synchronization is only needed starting several
     * streams. On stop or if only one stream do as quick as possible
     */
    let sync_streams = start != 0 && snd_pcm_stream_linked(substream);
    let tstamp = lola_get_tstamp(chip, !sync_streams);
    spin_lock(&mut (*chip).reg_lock);
    let mut s = snd_pcm_group_for_each_entry(core::ptr::null_mut(), substream);
    while !s.is_null() {
        if (*(*s).pcm).card != (*(*substream).pcm).card {
            s = snd_pcm_group_for_each_entry(s, substream);
            continue;
        }
        let str_ = lola_get_stream(s);
        if start != 0 {
            lola_stream_start(chip, str_, tstamp);
        } else {
            lola_stream_stop(chip, str_, tstamp);
        }
        (*str_).running = start;
        (*str_).paused = if start == 0 { 1 } else { 0 };
        snd_pcm_trigger_done(s, substream);
        s = snd_pcm_group_for_each_entry(s, substream);
    }
    spin_unlock(&mut (*chip).reg_lock);
    0
}

unsafe extern "C" fn lola_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream);
    let str_ = lola_get_stream(substream);
    let mut pos = lola_dsd_read(chip, (*str_).dsd, LPIB);

    if pos >= (*str_).bufsize {
        pos = 0;
    }
    bytes_to_frames((*substream).runtime, pos)
}

#[no_mangle]
pub unsafe extern "C" fn lola_pcm_update(
    _chip: *mut lola,
    pcm: *mut lola_pcm,
    mut bits: c_uint,
) {
    let num_streams = if (*pcm).num_streams < (*pcm).streams.len() as u8 {
        (*pcm).num_streams
    } else {
        (*pcm).streams.len() as u8
    };

    let mut i: c_int = 0;
    while bits != 0 && i < num_streams as c_int {
        if (bits & (1u32 << i)) != 0 {
            let str_ = &mut (*pcm).streams[i as usize] as *mut lola_stream;
            if !(*str_).substream.is_null() && (*str_).running != 0 {
                snd_pcm_period_elapsed((*str_).substream);
            }
            bits &= !(1u32 << i);
        }
        i += 1;
    }
}

static lola_pcm_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(lola_pcm_open),
    close: Some(lola_pcm_close),
    hw_params: Some(lola_pcm_hw_params),
    hw_free: Some(lola_pcm_hw_free),
    prepare: Some(lola_pcm_prepare),
    trigger: Some(lola_pcm_trigger),
    pointer: Some(lola_pcm_pointer),
};

#[no_mangle]
pub unsafe extern "C" fn lola_create_pcm(chip: *mut lola) -> c_int {
    let mut pcm: *mut snd_pcm = core::ptr::null_mut();
    let mut i: c_int = 0;

    while i < 2 {
        (*chip).pcm[i as usize].bdl = snd_devm_alloc_pages(
            &mut (*(*chip).pci).dev,
            SNDRV_DMA_TYPE_DEV,
            PAGE_SIZE,
        );
        if (*chip).pcm[i as usize].bdl.is_null() {
            return -ENOMEM;
        }
        i += 1;
    }

    let mut err = snd_pcm_new(
        (*chip).card,
        b"Digigram Lola\0".as_ptr() as *const c_char,
        0,
        (*chip).pcm[SNDRV_PCM_STREAM_PLAYBACK as usize].num_streams as c_int,
        (*chip).pcm[SNDRV_PCM_STREAM_CAPTURE as usize].num_streams as c_int,
        &mut pcm,
    );
    if err < 0 {
        return err;
    }
    strscpy(
        (*pcm).name.as_mut_ptr(),
        b"Digigram Lola\0".as_ptr() as *const c_char,
        (*pcm).name.len(),
    );
    (*pcm).private_data = chip as *mut c_void;
    i = 0;
    while i < 2 {
        if (*chip).pcm[i as usize].num_streams != 0 {
            snd_pcm_set_ops(pcm, i, &lola_pcm_ops);
        }
        i += 1;
    }
    /* buffer pre-allocation */
    snd_pcm_set_managed_buffer_all(
        pcm,
        SNDRV_DMA_TYPE_DEV_SG,
        &mut (*(*chip).pci).dev,
        1024 * 64,
        32 * 1024 * 1024,
    );
    0
}

/*
 */

unsafe fn lola_init_stream(
    chip: *mut lola,
    str_: *mut lola_stream,
    idx: c_int,
    nid: c_int,
    dir: c_int,
) -> c_int {
    let mut val: c_uint = 0;

    (*str_).nid = nid;
    (*str_).index = idx;
    (*str_).dsd = idx;
    if dir == PLAY {
        (*str_).dsd += MAX_STREAM_IN_COUNT;
    }
    let mut err = lola_read_param(chip, nid, LOLA_PAR_AUDIO_WIDGET_CAP, &mut val);
    if err < 0 {
        dev_err(
            (*(*chip).card).dev,
            b"Can't read wcaps for 0x%x\n\0".as_ptr() as *const c_char,
            nid,
        );
        return err;
    }
    if dir == PLAY {
        /* test TYPE and bits 0..11 (no test bit9 : Digital = 0/1) */
        if (val & 0x00f00dff) != 0x00000010 {
            dev_err(
                (*(*chip).card).dev,
                b"Invalid wcaps 0x%x for 0x%x\n\0".as_ptr() as *const c_char,
                val,
                nid,
            );
            return -EINVAL;
        }
    } else {
        /* test TYPE and bits 0..11 (no test bit9 : Digital = 0/1)
         * (bug : ignore bit8: Conn list = 0/1)
         */
        if (val & 0x00f00cff) != 0x00100010 {
            dev_err(
                (*(*chip).card).dev,
                b"Invalid wcaps 0x%x for 0x%x\n\0".as_ptr() as *const c_char,
                val,
                nid,
            );
            return -EINVAL;
        }
        /* test bit9:DIGITAL and bit12:SRC_PRESENT*/
        if (val & 0x00001200) == 0x00001200 {
            (*chip).input_src_caps_mask |= 1u32 << idx;
        }
    }

    err = lola_read_param(chip, nid, LOLA_PAR_STREAM_FORMATS, &mut val);
    if err < 0 {
        dev_err(
            (*(*chip).card).dev,
            b"Can't read FORMATS 0x%x\n\0".as_ptr() as *const c_char,
            nid,
        );
        return err;
    }
    val &= 3;
    if val == 3 {
        (*str_).can_float = true;
    }
    if (val & 1) == 0 {
        dev_err(
            (*(*chip).card).dev,
            b"Invalid formats 0x%x for 0x%x\0".as_ptr() as *const c_char,
            val,
            nid,
        );
        return -EINVAL;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn lola_init_pcm(chip: *mut lola, dir: c_int, nidp: *mut c_int) -> c_int {
    let pcm = &mut (*chip).pcm[dir as usize] as *mut lola_pcm;
    let mut nid = *nidp;
    let mut i: c_int = 0;

    while i < (*pcm).num_streams as c_int {
        let err = lola_init_stream(chip, &mut (*pcm).streams[i as usize], i, nid, dir);
        if err < 0 {
            return err;
        }
        i += 1;
        nid += 1;
    }
    *nidp = nid;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
