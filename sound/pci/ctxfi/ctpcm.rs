// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
 *
 * @File	ctpcm.c
 *
 * @Brief
 * This file contains the definition of the pcm device functions.
 *
 * @Author	Liu Chun
 * @Date 	Apr 2 2008
 */

/*
 * C dependencies removed from executable Rust:
 *   #include "ctpcm.h"
 *   #include "cttimer.h"
 *   #include <linux/slab.h>
 *   #include <sound/pcm.h>
 *
 * The types, constants, and functions referenced below are supplied by those
 * dependencies in the original repository.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const ENOMEM: c_int = 12;
const UINT_MAX: c_uint = c_uint::MAX;

extern "C" {
    static snd_pcm_std_chmaps: *const snd_pcm_chmap_elem;

    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut ct_atc;
    fn ct_timer_instance_free(timer: *mut c_void);
    fn ct_timer_instance_new(timer: *mut c_void, data: *mut ct_atc_pcm) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, var: c_int) -> c_int;
    fn snd_pcm_hw_constraint_minmax(
        runtime: *mut snd_pcm_runtime,
        var: c_int,
        min: c_uint,
        max: c_uint,
    ) -> c_int;
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, size: c_ulong) -> snd_pcm_uframes_t;
    fn snd_pcm_new(
        card: *mut snd_card,
        id: *const c_char,
        device: CTALSADEVS,
        playback_count: c_int,
        capture_count: c_int,
        rpcm: *mut *mut snd_pcm,
    ) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        type_: c_int,
        data: *mut c_void,
        size: usize,
        max: usize,
    );
    fn snd_pcm_add_chmap_ctls(
        pcm: *mut snd_pcm,
        stream: c_int,
        chmap: *const snd_pcm_chmap_elem,
        max_channels: c_int,
        private_value: c_ulong,
        info: *mut c_void,
    ) -> c_int;
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: c_uint,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub buffer_bytes_max: usize,
    pub period_bytes_min: usize,
    pub period_bytes_max: usize,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub fifo_size: usize,
}

#[repr(C)]
pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub ioctl: Option<unsafe extern "C" fn() -> c_int>,
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}

#[repr(C)]
pub struct snd_pcm_chmap_elem {
    pub channels: c_uint,
    pub map: [c_uint; 8],
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw: snd_pcm_hardware,
    pub buffer_size: snd_pcm_uframes_t,
    pub private_data: *mut ct_atc_pcm,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_pcm_runtime)>,
}

#[repr(C)]
pub struct snd_pcm {
    pub private_data: *mut c_void,
    pub info_flags: c_uint,
    pub dev_subclass: c_uint,
    pub name: [c_char; 80],
    pub device: CTALSADEVS,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut c_void,
}

#[repr(C)]
pub struct pci_dev {
    pub dev: c_void,
}

#[repr(C)]
pub struct ct_atc_pcm {
    pub started: c_int,
    pub substream: *mut snd_pcm_substream,
    pub interrupt: Option<unsafe extern "C" fn(*mut ct_atc_pcm)>,
    pub timer: *mut c_void,
}

#[repr(C)]
pub struct ct_atc {
    pub card: *mut snd_card,
    pub pci: *mut pci_dev,
    pub timer: *mut c_void,
    pub rsr: c_uint,
    pub msr: c_uint,
    pub pcms: [*mut snd_pcm; 8],
    pub pcm_release_resources: unsafe extern "C" fn(*mut ct_atc, *mut ct_atc_pcm),
    pub spdif_out_passthru: unsafe extern "C" fn(*mut ct_atc, c_int),
    pub spdif_passthru_playback_prepare: unsafe extern "C" fn(*mut ct_atc, *mut ct_atc_pcm) -> c_int,
    pub pcm_playback_prepare: unsafe extern "C" fn(*mut ct_atc, *mut ct_atc_pcm) -> c_int,
    pub pcm_playback_start: unsafe extern "C" fn(*mut ct_atc, *mut ct_atc_pcm),
    pub pcm_playback_stop: unsafe extern "C" fn(*mut ct_atc, *mut ct_atc_pcm),
    pub pcm_playback_position: unsafe extern "C" fn(*mut ct_atc, *mut ct_atc_pcm) -> c_ulong,
    pub pcm_capture_prepare: unsafe extern "C" fn(*mut ct_atc, *mut ct_atc_pcm) -> c_int,
    pub pcm_capture_start: unsafe extern "C" fn(*mut ct_atc, *mut ct_atc_pcm),
    pub pcm_capture_stop: unsafe extern "C" fn(*mut ct_atc, *mut ct_atc_pcm),
    pub pcm_capture_position: unsafe extern "C" fn(*mut ct_atc, *mut ct_atc_pcm) -> c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CTALSADEVS {
    FRONT = 0,
    SURROUND = 1,
    CLFE = 2,
    SIDE = 3,
    IEC958 = 4,
}

pub type snd_pcm_uframes_t = c_ulong;

const GFP_KERNEL: c_uint = 0;
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 0;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 1;
const SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint = 1 << 2;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 3;
const SNDRV_PCM_INFO_PAUSE: c_uint = 1 << 4;
const SNDRV_PCM_FMTBIT_U8: c_uint = 1 << 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 1 << 1;
const SNDRV_PCM_FMTBIT_S24_3LE: c_uint = 1 << 2;
const SNDRV_PCM_FMTBIT_S32_LE: c_uint = 1 << 3;
const SNDRV_PCM_FMTBIT_FLOAT_LE: c_uint = 1 << 4;
const SNDRV_PCM_RATE_CONTINUOUS: c_uint = 1 << 0;
const SNDRV_PCM_RATE_8000_192000: c_uint = 1 << 1;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 2;
const SNDRV_PCM_RATE_44100: c_uint = 1 << 3;
const SNDRV_PCM_RATE_32000: c_uint = 1 << 4;
const SNDRV_PCM_RATE_8000_96000: c_uint = 1 << 5;
const SNDRV_PCM_HW_PARAM_PERIODS: c_int = 0;
const SNDRV_PCM_HW_PARAM_BUFFER_BYTES: c_int = 1;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 2;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 5;
const SNDRV_CHMAP_MONO: c_uint = 1;
const SNDRV_CHMAP_RL: c_uint = 2;
const SNDRV_CHMAP_RR: c_uint = 3;
const SNDRV_CHMAP_FC: c_uint = 4;
const SNDRV_CHMAP_LFE: c_uint = 5;
const SNDRV_CHMAP_SL: c_uint = 6;
const SNDRV_CHMAP_SR: c_uint = 7;
const SNDRV_PCM_SUBCLASS_GENERIC_MIX: c_uint = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_DMA_TYPE_DEV_SG: c_int = 0;

/* Hardware descriptions for playback */
static ct_pcm_playback_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_PAUSE,
    formats: SNDRV_PCM_FMTBIT_U8
        | SNDRV_PCM_FMTBIT_S16_LE
        | SNDRV_PCM_FMTBIT_S24_3LE
        | SNDRV_PCM_FMTBIT_S32_LE
        | SNDRV_PCM_FMTBIT_FLOAT_LE,
    rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_192000,
    rate_min: 8000,
    rate_max: 192000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 128 * 1024,
    period_bytes_min: 64,
    period_bytes_max: 128 * 1024,
    periods_min: 2,
    periods_max: 1024,
    fifo_size: 0,
};

static ct_spdif_passthru_playback_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_PAUSE,
    formats: SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_32000,
    rate_min: 32000,
    rate_max: 48000,
    channels_min: 2,
    channels_max: 2,
    buffer_bytes_max: 128 * 1024,
    period_bytes_min: 64,
    period_bytes_max: 128 * 1024,
    periods_min: 2,
    periods_max: 1024,
    fifo_size: 0,
};

/* Hardware descriptions for capture */
static ct_pcm_capture_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_U8
        | SNDRV_PCM_FMTBIT_S16_LE
        | SNDRV_PCM_FMTBIT_S24_3LE
        | SNDRV_PCM_FMTBIT_S32_LE
        | SNDRV_PCM_FMTBIT_FLOAT_LE,
    rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_96000,
    rate_min: 8000,
    rate_max: 96000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 128 * 1024,
    period_bytes_min: 384,
    period_bytes_max: 64 * 1024,
    periods_min: 2,
    periods_max: 1024,
    fifo_size: 0,
};

unsafe extern "C" fn ct_atc_pcm_interrupt(atc_pcm: *mut ct_atc_pcm) {
    let apcm: *mut ct_atc_pcm = atc_pcm;

    if (*apcm).substream.is_null() {
        return;
    }

    snd_pcm_period_elapsed((*apcm).substream);
}

unsafe extern "C" fn ct_atc_pcm_free_substream(runtime: *mut snd_pcm_runtime) {
    let apcm: *mut ct_atc_pcm = (*runtime).private_data;
    let atc: *mut ct_atc = snd_pcm_substream_chip((*apcm).substream);

    ((*atc).pcm_release_resources)(atc, apcm);
    ct_timer_instance_free((*apcm).timer);
    kfree(apcm as *mut c_void);
    (*runtime).private_data = core::ptr::null_mut();
}

/* pcm playback operations */
unsafe extern "C" fn ct_pcm_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    let atc: *mut ct_atc = snd_pcm_substream_chip(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let apcm: *mut ct_atc_pcm;
    let mut err: c_int;

    apcm = kzalloc(core::mem::size_of::<ct_atc_pcm>(), GFP_KERNEL) as *mut ct_atc_pcm;
    if apcm.is_null() {
        return -ENOMEM;
    }

    (*apcm).substream = substream;
    (*apcm).interrupt = Some(ct_atc_pcm_interrupt);
    if CTALSADEVS::IEC958 == (*(*substream).pcm).device {
        (*runtime).hw = ct_spdif_passthru_playback_hw;
        ((*atc).spdif_out_passthru)(atc, 1);
    } else {
        (*runtime).hw = ct_pcm_playback_hw;
        if CTALSADEVS::FRONT == (*(*substream).pcm).device {
            (*runtime).hw.channels_max = 8;
        }
    }

    err = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if err < 0 {
        kfree(apcm as *mut c_void);
        return err;
    }

    err = snd_pcm_hw_constraint_minmax(
        runtime,
        SNDRV_PCM_HW_PARAM_BUFFER_BYTES,
        1024,
        UINT_MAX,
    );
    if err < 0 {
        kfree(apcm as *mut c_void);
        return err;
    }

    (*apcm).timer = ct_timer_instance_new((*atc).timer, apcm);
    if (*apcm).timer.is_null() {
        err = -ENOMEM;
        kfree(apcm as *mut c_void);
        return err;
    }
    (*runtime).private_data = apcm;
    (*runtime).private_free = Some(ct_atc_pcm_free_substream);

    0
}

unsafe extern "C" fn ct_pcm_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    let atc: *mut ct_atc = snd_pcm_substream_chip(substream);

    /* TODO: Notify mixer inactive. */
    if CTALSADEVS::IEC958 == (*(*substream).pcm).device {
        ((*atc).spdif_out_passthru)(atc, 0);
    }

    /* The ct_atc_pcm object will be freed by runtime->private_free */

    0
}

unsafe extern "C" fn ct_pcm_hw_params(
    substream: *mut snd_pcm_substream,
    _hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let atc: *mut ct_atc = snd_pcm_substream_chip(substream);
    let apcm: *mut ct_atc_pcm = (*(*substream).runtime).private_data;

    /* clear previous resources */
    ((*atc).pcm_release_resources)(atc, apcm);
    0
}

unsafe extern "C" fn ct_pcm_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let atc: *mut ct_atc = snd_pcm_substream_chip(substream);
    let apcm: *mut ct_atc_pcm = (*(*substream).runtime).private_data;

    /* clear previous resources */
    ((*atc).pcm_release_resources)(atc, apcm);
    0
}

unsafe extern "C" fn ct_pcm_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let err: c_int;
    let atc: *mut ct_atc = snd_pcm_substream_chip(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let apcm: *mut ct_atc_pcm = (*runtime).private_data;

    if CTALSADEVS::IEC958 == (*(*substream).pcm).device {
        err = ((*atc).spdif_passthru_playback_prepare)(atc, apcm);
    } else {
        err = ((*atc).pcm_playback_prepare)(atc, apcm);
    }

    if err < 0 {
        dev_err(
            (*(*atc).card).dev,
            b"Preparing pcm playback failed!!!\n\0".as_ptr() as *const c_char,
        );
        return err;
    }

    0
}

unsafe extern "C" fn ct_pcm_playback_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let atc: *mut ct_atc = snd_pcm_substream_chip(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let apcm: *mut ct_atc_pcm = (*runtime).private_data;

    match cmd {
        SNDRV_PCM_TRIGGER_START
        | SNDRV_PCM_TRIGGER_RESUME
        | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            ((*atc).pcm_playback_start)(atc, apcm);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            ((*atc).pcm_playback_stop)(atc, apcm);
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn ct_pcm_playback_pointer(
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let mut position: c_ulong;
    let atc: *mut ct_atc = snd_pcm_substream_chip(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let apcm: *mut ct_atc_pcm = (*runtime).private_data;

    /* Read out playback position */
    position = ((*atc).pcm_playback_position)(atc, apcm);
    position = bytes_to_frames(runtime, position);
    if position >= (*runtime).buffer_size {
        position = 0;
    }
    position
}

/* pcm capture operations */
unsafe extern "C" fn ct_pcm_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    let atc: *mut ct_atc = snd_pcm_substream_chip(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let apcm: *mut ct_atc_pcm;
    let mut err: c_int;

    apcm = kzalloc(core::mem::size_of::<ct_atc_pcm>(), GFP_KERNEL) as *mut ct_atc_pcm;
    if apcm.is_null() {
        return -ENOMEM;
    }

    (*apcm).started = 0;
    (*apcm).substream = substream;
    (*apcm).interrupt = Some(ct_atc_pcm_interrupt);
    (*runtime).hw = ct_pcm_capture_hw;
    (*runtime).hw.rate_max = (*atc).rsr * (*atc).msr;

    err = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if err < 0 {
        kfree(apcm as *mut c_void);
        return err;
    }

    err = snd_pcm_hw_constraint_minmax(
        runtime,
        SNDRV_PCM_HW_PARAM_BUFFER_BYTES,
        1024,
        UINT_MAX,
    );
    if err < 0 {
        kfree(apcm as *mut c_void);
        return err;
    }

    (*apcm).timer = ct_timer_instance_new((*atc).timer, apcm);
    if (*apcm).timer.is_null() {
        err = -ENOMEM;
        kfree(apcm as *mut c_void);
        return err;
    }
    (*runtime).private_data = apcm;
    (*runtime).private_free = Some(ct_atc_pcm_free_substream);

    0
}

unsafe extern "C" fn ct_pcm_capture_close(_substream: *mut snd_pcm_substream) -> c_int {
    /* The ct_atc_pcm object will be freed by runtime->private_free */
    /* TODO: Notify mixer inactive. */
    0
}

unsafe extern "C" fn ct_pcm_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let err: c_int;
    let atc: *mut ct_atc = snd_pcm_substream_chip(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let apcm: *mut ct_atc_pcm = (*runtime).private_data;

    err = ((*atc).pcm_capture_prepare)(atc, apcm);
    if err < 0 {
        dev_err(
            (*(*atc).card).dev,
            b"Preparing pcm capture failed!!!\n\0".as_ptr() as *const c_char,
        );
        return err;
    }

    0
}

unsafe extern "C" fn ct_pcm_capture_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let atc: *mut ct_atc = snd_pcm_substream_chip(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let apcm: *mut ct_atc_pcm = (*runtime).private_data;

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            ((*atc).pcm_capture_start)(atc, apcm);
        }
        SNDRV_PCM_TRIGGER_STOP => {
            ((*atc).pcm_capture_stop)(atc, apcm);
        }
        _ => {
            ((*atc).pcm_capture_stop)(atc, apcm);
        }
    }

    0
}

unsafe extern "C" fn ct_pcm_capture_pointer(
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let mut position: c_ulong;
    let atc: *mut ct_atc = snd_pcm_substream_chip(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let apcm: *mut ct_atc_pcm = (*runtime).private_data;

    /* Read out playback position */
    position = ((*atc).pcm_capture_position)(atc, apcm);
    position = bytes_to_frames(runtime, position);
    if position >= (*runtime).buffer_size {
        position = 0;
    }
    position
}

/* PCM operators for playback */
static ct_pcm_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(ct_pcm_playback_open),
    close: Some(ct_pcm_playback_close),
    ioctl: None,
    hw_params: Some(ct_pcm_hw_params),
    hw_free: Some(ct_pcm_hw_free),
    prepare: Some(ct_pcm_playback_prepare),
    trigger: Some(ct_pcm_playback_trigger),
    pointer: Some(ct_pcm_playback_pointer),
};

/* PCM operators for capture */
static ct_pcm_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(ct_pcm_capture_open),
    close: Some(ct_pcm_capture_close),
    ioctl: None,
    hw_params: Some(ct_pcm_hw_params),
    hw_free: Some(ct_pcm_hw_free),
    prepare: Some(ct_pcm_capture_prepare),
    trigger: Some(ct_pcm_capture_trigger),
    pointer: Some(ct_pcm_capture_pointer),
};

static surround_map: [snd_pcm_chmap_elem; 3] = [
    snd_pcm_chmap_elem {
        channels: 1,
        map: [SNDRV_CHMAP_MONO, 0, 0, 0, 0, 0, 0, 0],
    },
    snd_pcm_chmap_elem {
        channels: 2,
        map: [SNDRV_CHMAP_RL, SNDRV_CHMAP_RR, 0, 0, 0, 0, 0, 0],
    },
    snd_pcm_chmap_elem {
        channels: 0,
        map: [0; 8],
    },
];

static clfe_map: [snd_pcm_chmap_elem; 3] = [
    snd_pcm_chmap_elem {
        channels: 1,
        map: [SNDRV_CHMAP_MONO, 0, 0, 0, 0, 0, 0, 0],
    },
    snd_pcm_chmap_elem {
        channels: 2,
        map: [SNDRV_CHMAP_FC, SNDRV_CHMAP_LFE, 0, 0, 0, 0, 0, 0],
    },
    snd_pcm_chmap_elem {
        channels: 0,
        map: [0; 8],
    },
];

static side_map: [snd_pcm_chmap_elem; 3] = [
    snd_pcm_chmap_elem {
        channels: 1,
        map: [SNDRV_CHMAP_MONO, 0, 0, 0, 0, 0, 0, 0],
    },
    snd_pcm_chmap_elem {
        channels: 2,
        map: [SNDRV_CHMAP_SL, SNDRV_CHMAP_SR, 0, 0, 0, 0, 0, 0],
    },
    snd_pcm_chmap_elem {
        channels: 0,
        map: [0; 8],
    },
];

/* Create ALSA pcm device */
#[no_mangle]
pub unsafe extern "C" fn ct_alsa_pcm_create(
    atc: *mut ct_atc,
    device: CTALSADEVS,
    device_name: *const c_char,
) -> c_int {
    let mut pcm: *mut snd_pcm = core::ptr::null_mut();
    let map: *const snd_pcm_chmap_elem;
    let mut chs: c_int;
    let mut err: c_int;
    let playback_count: c_int;
    let capture_count: c_int;

    playback_count = if CTALSADEVS::IEC958 == device { 1 } else { 256 };
    capture_count = if CTALSADEVS::FRONT == device { 1 } else { 0 };
    err = snd_pcm_new(
        (*atc).card,
        b"ctxfi\0".as_ptr() as *const c_char,
        device,
        playback_count,
        capture_count,
        &mut pcm,
    );
    if err < 0 {
        dev_err(
            (*(*atc).card).dev,
            b"snd_pcm_new failed!! Err=%d\n\0".as_ptr() as *const c_char,
            err,
        );
        return err;
    }

    (*pcm).private_data = atc as *mut c_void;
    (*pcm).info_flags = 0;
    (*pcm).dev_subclass = SNDRV_PCM_SUBCLASS_GENERIC_MIX;
    strscpy(
        (*pcm).name.as_mut_ptr(),
        device_name,
        core::mem::size_of_val(&(*pcm).name),
    );

    snd_pcm_set_ops(
        pcm,
        SNDRV_PCM_STREAM_PLAYBACK,
        &ct_pcm_playback_ops as *const snd_pcm_ops,
    );

    if CTALSADEVS::FRONT == device {
        snd_pcm_set_ops(
            pcm,
            SNDRV_PCM_STREAM_CAPTURE,
            &ct_pcm_capture_ops as *const snd_pcm_ops,
        );
    }

    snd_pcm_set_managed_buffer_all(
        pcm,
        SNDRV_DMA_TYPE_DEV_SG,
        &mut (*(*atc).pci).dev as *mut c_void,
        128 * 1024,
        128 * 1024,
    );

    chs = 2;
    match device {
        CTALSADEVS::FRONT => {
            chs = 8;
            map = snd_pcm_std_chmaps;
        }
        CTALSADEVS::SURROUND => {
            map = surround_map.as_ptr();
        }
        CTALSADEVS::CLFE => {
            map = clfe_map.as_ptr();
        }
        CTALSADEVS::SIDE => {
            map = side_map.as_ptr();
        }
        _ => {
            map = snd_pcm_std_chmaps;
        }
    }
    err = snd_pcm_add_chmap_ctls(
        pcm,
        SNDRV_PCM_STREAM_PLAYBACK,
        map,
        chs,
        0,
        core::ptr::null_mut(),
    );
    if err < 0 {
        return err;
    }

    /*
     * Original C condition:
     * #ifdef CONFIG_PM_SLEEP
     *	atc->pcms[device] = pcm;
     * #endif
     */
    #[cfg(CONFIG_PM_SLEEP)]
    {
        (*atc).pcms[device as usize] = pcm;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
