// SPDX-License-Identifier: GPL-2.0
/*
 * PC-Speaker driver for Linux
 *
 * Copyright (C) 1993-1997  Michael Beck
 * Copyright (C) 1997-2001  David Woodhouse
 * Copyright (C) 2001-2008  Stas Sergeev
 */

// Translated from drivers/pcsp/pcsp_lib.c. Kernel, ALSA, and pcsp.h symbols are
// expected to be supplied by the surrounding translated repository.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type bool_ = bool;
type u8 = core::primitive::u8;
type u64 = core::primitive::u64;
type size_t = usize;
type snd_pcm_uframes_t = c_ulong;

const DMIX_WANTS_S16: c_int = 1;

static mut nforce_wa: bool_ = false;

// module_param(nforce_wa, bool, 0444);
// MODULE_PARM_DESC(nforce_wa, "Apply NForce chipset workaround (expect bad sound)");

#[repr(C)]
pub struct atomic_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct raw_spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hrtimer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut c_void,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub dma_area: *mut u8,
    pub format: c_int,
    pub periods: c_uint,
    pub hw: snd_pcm_hardware,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm {
    pub private_data: *mut c_void,
    pub info_flags: c_uint,
    pub name: *mut c_char,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
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
    pub buffer_bytes_max: size_t,
    pub period_bytes_min: size_t,
    pub period_bytes_max: size_t,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub fifo_size: size_t,
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
pub struct snd_pcsp {
    pub timer_active: atomic_t,
    pub playback_substream: *mut snd_pcm_substream,
    pub thalf: c_int,
    pub val61: u8,
    pub ns_rem: u64,
    pub enable: c_int,
    pub playback_ptr: size_t,
    pub period_ptr: size_t,
    pub fmt_size: c_int,
    pub is_signed: c_int,
    pub substream_lock: spinlock_t,
    pub card: *mut snd_card,
    pub timer: hrtimer,
    pub pcm: *mut snd_pcm,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum hrtimer_restart {
    HRTIMER_NORESTART = 0,
    HRTIMER_RESTART = 1,
}

extern "C" {
    static mut pcsp_chip: snd_pcsp;
    static mut i8253_lock: raw_spinlock_t;
    static mut system_highpri_wq: *mut c_void;

    static SNDRV_PCM_INFO_INTERLEAVED: c_uint;
    static SNDRV_PCM_INFO_HALF_DUPLEX: c_uint;
    static SNDRV_PCM_INFO_MMAP: c_uint;
    static SNDRV_PCM_INFO_MMAP_VALID: c_uint;
    static SNDRV_PCM_FMTBIT_U8: u64;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_RATE_KNOT: c_uint;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_DMA_TYPE_CONTINUOUS: c_int;
    static PCSP_DEFAULT_SRATE: c_uint;
    static PCSP_BUFFER_SIZE: size_t;
    static PCSP_MAX_PERIOD_SIZE: size_t;
    static PCSP_MAX_PERIODS: c_uint;
    static EIO: c_int;
    static EINVAL: c_int;
    static EBUSY: c_int;
    static HRTIMER_MODE_REL: c_int;

    fn atomic_read(v: *const atomic_t) -> c_int;
    fn atomic_set(v: *mut atomic_t, i: c_int);
    fn outb(value: u8, port: u16);
    fn outb_p(value: u8, port: u16);
    fn inb(port: u16) -> u8;
    fn raw_spin_lock(lock: *mut raw_spinlock_t);
    fn raw_spin_unlock(lock: *mut raw_spinlock_t);
    fn raw_spin_lock_irqsave(lock: *mut raw_spinlock_t, flags: *mut c_ulong);
    fn raw_spin_unlock_irqrestore(lock: *mut raw_spinlock_t, flags: c_ulong);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn local_irq_disable();
    fn local_irq_enable();
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> size_t;
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> size_t;
    fn snd_pcm_format_physical_width(format: c_int) -> c_int;
    fn snd_pcm_format_signed(format: c_int) -> c_int;
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut snd_pcsp;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: c_uint) -> snd_pcm_uframes_t;
    fn hrtimer_forward_now(timer: *mut hrtimer, time: c_void);
    fn ns_to_ktime(ns: u64) -> c_void;
    fn hrtimer_start(timer: *mut hrtimer, time: c_int, mode: c_int);
    fn hrtimer_cancel(timer: *mut hrtimer);
    fn cancel_work_sync(work: *mut work_struct);
    fn queue_work(wq: *mut c_void, work: *mut work_struct) -> bool_;
    fn dev_warn(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut c_void, fmt: *const c_char, ...);
    fn snd_pcm_new(
        card: *mut snd_card,
        id: *const c_char,
        device: c_int,
        playback_count: c_int,
        capture_count: c_int,
        rpcm: *mut *mut snd_pcm,
    ) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops);
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        type_: c_int,
        data: *mut c_void,
        min: size_t,
        max: size_t,
    );

    fn CUR_DIV() -> c_int;
    fn PCSP_PERIOD_NS() -> u64;
    fn PCSP_CALC_NS(timer_cnt: u8) -> u64;
    fn PCSP_INDEX_INC() -> size_t;
}

// static DECLARE_WORK(pcsp_pcm_work, pcsp_call_pcm_elapsed);
static mut pcsp_pcm_work: work_struct = work_struct { _private: [] };

unsafe fn c_str(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

/*
 * Call snd_pcm_period_elapsed in a work
 * This avoids spinlock messes and long-running irq contexts
 */
unsafe extern "C" fn pcsp_call_pcm_elapsed(_work: *mut work_struct) {
    if atomic_read(&raw const pcsp_chip.timer_active) != 0 {
        let substream: *mut snd_pcm_substream;
        substream = pcsp_chip.playback_substream;
        if !substream.is_null() {
            snd_pcm_period_elapsed(substream);
        }
    }
}

/* write the port and returns the next expire time in ns;
 * called at the trigger-start and in hrtimer callback
 */
unsafe fn pcsp_timer_update(chip: *mut snd_pcsp) -> u64 {
    let mut timer_cnt: u8;
    let mut val: u8;
    let ns: u64;
    let substream: *mut snd_pcm_substream;
    let runtime: *mut snd_pcm_runtime;
    let mut flags: c_ulong = 0;

    if (*chip).thalf != 0 {
        outb((*chip).val61, 0x61);
        (*chip).thalf = 0;
        return (*chip).ns_rem;
    }

    substream = (*chip).playback_substream;
    if substream.is_null() {
        return 0;
    }

    runtime = (*substream).runtime;
    /* assume it is mono! */
    val = *(*runtime)
        .dma_area
        .add((*chip).playback_ptr + (*chip).fmt_size as size_t - 1);
    if (*chip).is_signed != 0 {
        val ^= 0x80;
    }
    timer_cnt = ((val as c_int * CUR_DIV()) / 256) as u8;

    if timer_cnt != 0 && (*chip).enable != 0 {
        raw_spin_lock_irqsave(&raw mut i8253_lock, &mut flags);
        if !nforce_wa {
            outb_p((*chip).val61, 0x61);
            outb_p(timer_cnt, 0x42);
            outb((*chip).val61 ^ 1, 0x61);
        } else {
            outb((*chip).val61 ^ 2, 0x61);
            (*chip).thalf = 1;
        }
        raw_spin_unlock_irqrestore(&raw mut i8253_lock, flags);
    }

    (*chip).ns_rem = PCSP_PERIOD_NS();
    ns = if (*chip).thalf != 0 {
        PCSP_CALC_NS(timer_cnt)
    } else {
        (*chip).ns_rem
    };
    (*chip).ns_rem = (*chip).ns_rem.wrapping_sub(ns);
    ns
}

unsafe fn pcsp_pointer_update(chip: *mut snd_pcsp) {
    let substream: *mut snd_pcm_substream;
    let period_bytes: size_t;
    let buffer_bytes: size_t;
    let mut periods_elapsed: c_int;
    let mut flags: c_ulong = 0;

    /* update the playback position */
    substream = (*chip).playback_substream;
    if substream.is_null() {
        return;
    }

    period_bytes = snd_pcm_lib_period_bytes(substream);
    buffer_bytes = snd_pcm_lib_buffer_bytes(substream);

    spin_lock_irqsave(&mut (*chip).substream_lock, &mut flags);
    (*chip).playback_ptr = (*chip)
        .playback_ptr
        .wrapping_add(PCSP_INDEX_INC().wrapping_mul((*chip).fmt_size as size_t));
    periods_elapsed = (*chip).playback_ptr.wrapping_sub((*chip).period_ptr) as c_int;
    if periods_elapsed < 0 {
        // #if PCSP_DEBUG
        // dev_dbg(chip->card->dev,
        //     "PCSP: buffer_bytes mod period_bytes != 0 ? (%zi %zi %zi)\n",
        //     chip->playback_ptr, period_bytes, buffer_bytes);
        // #endif
        periods_elapsed = periods_elapsed.wrapping_add(buffer_bytes as c_int);
    }
    periods_elapsed /= period_bytes as c_int;
    /* wrap the pointer _before_ calling snd_pcm_period_elapsed(),
     * or ALSA will BUG on us. */
    (*chip).playback_ptr %= buffer_bytes;

    if periods_elapsed != 0 {
        (*chip).period_ptr = (*chip)
            .period_ptr
            .wrapping_add((periods_elapsed as size_t).wrapping_mul(period_bytes));
        (*chip).period_ptr %= buffer_bytes;
        queue_work(system_highpri_wq, &raw mut pcsp_pcm_work);
    }
    spin_unlock_irqrestore(&mut (*chip).substream_lock, flags);
}

#[no_mangle]
pub unsafe extern "C" fn pcsp_do_timer(handle: *mut hrtimer) -> hrtimer_restart {
    // container_of(handle, struct snd_pcsp, timer)
    let chip = (handle as *mut u8).sub(core::mem::offset_of!(snd_pcsp, timer)) as *mut snd_pcsp;
    let pointer_update: c_int;
    let ns: u64;

    if atomic_read(&(*chip).timer_active) == 0 || (*chip).playback_substream.is_null() {
        return hrtimer_restart::HRTIMER_NORESTART;
    }

    pointer_update = ((*chip).thalf == 0) as c_int;
    ns = pcsp_timer_update(chip);
    if ns == 0 {
        dev_warn((*(*chip).card).dev, c_str(b"PCSP: unexpected stop\n\0"));
        return hrtimer_restart::HRTIMER_NORESTART;
    }

    if pointer_update != 0 {
        pcsp_pointer_update(chip);
    }

    hrtimer_forward_now(handle, ns_to_ktime(ns));

    hrtimer_restart::HRTIMER_RESTART
}

unsafe fn pcsp_start_playing(chip: *mut snd_pcsp) -> c_int {
    // #if PCSP_DEBUG
    // dev_dbg(chip->card->dev, "PCSP: start_playing called\n");
    // #endif
    if atomic_read(&(*chip).timer_active) != 0 {
        dev_err((*(*chip).card).dev, c_str(b"PCSP: Timer already active\n\0"));
        return -EIO;
    }

    raw_spin_lock(&raw mut i8253_lock);
    (*chip).val61 = inb(0x61) | 0x03;
    outb_p(0x92, 0x43); /* binary, mode 1, LSB only, ch 2 */
    raw_spin_unlock(&raw mut i8253_lock);
    atomic_set(&mut (*chip).timer_active, 1);
    (*chip).thalf = 0;

    hrtimer_start(&mut pcsp_chip.timer, 0, HRTIMER_MODE_REL);
    0
}

unsafe fn pcsp_stop_playing(chip: *mut snd_pcsp) {
    // #if PCSP_DEBUG
    // dev_dbg(chip->card->dev, "PCSP: stop_playing called\n");
    // #endif
    if atomic_read(&(*chip).timer_active) == 0 {
        return;
    }

    atomic_set(&mut (*chip).timer_active, 0);
    raw_spin_lock(&raw mut i8253_lock);
    /* restore the timer */
    outb_p(0xb6, 0x43); /* binary, mode 3, LSB/MSB, ch 2 */
    outb((*chip).val61 & 0xFC, 0x61);
    raw_spin_unlock(&raw mut i8253_lock);
}

/*
 * Force to stop and sync the stream
 */
#[no_mangle]
pub unsafe extern "C" fn pcsp_sync_stop(chip: *mut snd_pcsp) {
    local_irq_disable();
    pcsp_stop_playing(chip);
    local_irq_enable();
    hrtimer_cancel(&mut (*chip).timer);
    cancel_work_sync(&raw mut pcsp_pcm_work);
}

unsafe extern "C" fn snd_pcsp_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut snd_pcsp = snd_pcm_substream_chip(substream);
    // #if PCSP_DEBUG
    // dev_dbg(chip->card->dev, "PCSP: close called\n");
    // #endif
    pcsp_sync_stop(chip);
    (*chip).playback_substream = core::ptr::null_mut();
    0
}

unsafe extern "C" fn snd_pcsp_playback_hw_params(
    substream: *mut snd_pcm_substream,
    _hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let chip: *mut snd_pcsp = snd_pcm_substream_chip(substream);
    pcsp_sync_stop(chip);
    0
}

unsafe extern "C" fn snd_pcsp_playback_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut snd_pcsp = snd_pcm_substream_chip(substream);
    // #if PCSP_DEBUG
    // dev_dbg(chip->card->dev, "PCSP: hw_free called\n");
    // #endif
    pcsp_sync_stop(chip);
    0
}

unsafe extern "C" fn snd_pcsp_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut snd_pcsp = snd_pcm_substream_chip(substream);
    pcsp_sync_stop(chip);
    (*chip).playback_ptr = 0;
    (*chip).period_ptr = 0;
    (*chip).fmt_size = snd_pcm_format_physical_width((*(*substream).runtime).format) >> 3;
    (*chip).is_signed = snd_pcm_format_signed((*(*substream).runtime).format);
    // #if PCSP_DEBUG
    // dev_dbg(chip->card->dev, "PCSP: prepare called, size=%zi psize=%zi f=%zi f1=%i fsize=%i\n",
    //     snd_pcm_lib_buffer_bytes(substream),
    //     snd_pcm_lib_period_bytes(substream),
    //     snd_pcm_lib_buffer_bytes(substream) /
    //     snd_pcm_lib_period_bytes(substream),
    //     substream->runtime->periods,
    //     chip->fmt_size);
    // #endif
    0
}

unsafe extern "C" fn snd_pcsp_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip: *mut snd_pcsp = snd_pcm_substream_chip(substream);
    // #if PCSP_DEBUG
    // dev_dbg(chip->card->dev, "PCSP: trigger called\n");
    // #endif
    if cmd == SNDRV_PCM_TRIGGER_START || cmd == SNDRV_PCM_TRIGGER_RESUME {
        return pcsp_start_playing(chip);
    }
    if cmd == SNDRV_PCM_TRIGGER_STOP || cmd == SNDRV_PCM_TRIGGER_SUSPEND {
        pcsp_stop_playing(chip);
    } else {
        return -EINVAL;
    }
    0
}

unsafe extern "C" fn snd_pcsp_playback_pointer(
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let chip: *mut snd_pcsp = snd_pcm_substream_chip(substream);
    let pos: c_uint;
    spin_lock(&mut (*chip).substream_lock);
    pos = (*chip).playback_ptr as c_uint;
    spin_unlock(&mut (*chip).substream_lock);
    bytes_to_frames((*substream).runtime, pos)
}

static snd_pcsp_playback: snd_pcm_hardware = snd_pcm_hardware {
    info: 0,         // SNDRV_PCM_INFO_* bitmask, initialized by surrounding translated constants.
    formats: 0,      // SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE when DMIX_WANTS_S16.
    rates: 0,        // SNDRV_PCM_RATE_KNOT.
    rate_min: 0,     // PCSP_DEFAULT_SRATE.
    rate_max: 0,     // PCSP_DEFAULT_SRATE.
    channels_min: 1,
    channels_max: 1,
    buffer_bytes_max: 0, // PCSP_BUFFER_SIZE.
    period_bytes_min: 64,
    period_bytes_max: 0, // PCSP_MAX_PERIOD_SIZE.
    periods_min: 2,
    periods_max: 0, // PCSP_MAX_PERIODS.
    fifo_size: 0,
};

unsafe extern "C" fn snd_pcsp_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut snd_pcsp = snd_pcm_substream_chip(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    // #if PCSP_DEBUG
    // dev_dbg(chip->card->dev, "PCSP: open called\n");
    // #endif
    if atomic_read(&(*chip).timer_active) != 0 {
        dev_err((*(*chip).card).dev, c_str(b"PCSP: still active!!\n\0"));
        return -EBUSY;
    }
    (*runtime).hw = snd_pcsp_playback;
    (*chip).playback_substream = substream;
    0
}

static snd_pcsp_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_pcsp_playback_open),
    close: Some(snd_pcsp_playback_close),
    hw_params: Some(snd_pcsp_playback_hw_params),
    hw_free: Some(snd_pcsp_playback_hw_free),
    prepare: Some(snd_pcsp_playback_prepare),
    trigger: Some(snd_pcsp_trigger),
    pointer: Some(snd_pcsp_playback_pointer),
};

#[no_mangle]
pub unsafe extern "C" fn snd_pcsp_new_pcm(chip: *mut snd_pcsp) -> c_int {
    let mut err: c_int;

    err = snd_pcm_new(
        (*chip).card,
        c_str(b"pcspeaker\0"),
        0,
        1,
        0,
        &mut (*chip).pcm,
    );
    if err < 0 {
        return err;
    }

    snd_pcm_set_ops(
        (*chip).pcm,
        SNDRV_PCM_STREAM_PLAYBACK,
        &snd_pcsp_playback_ops,
    );

    (*(*chip).pcm).private_data = chip as *mut c_void;
    (*(*chip).pcm).info_flags = SNDRV_PCM_INFO_HALF_DUPLEX;
    strscpy((*(*chip).pcm).name, c_str(b"pcsp\0"));

    snd_pcm_set_managed_buffer_all(
        (*chip).pcm,
        SNDRV_DMA_TYPE_CONTINUOUS,
        core::ptr::null_mut(),
        PCSP_BUFFER_SIZE,
        PCSP_BUFFER_SIZE,
    );

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
