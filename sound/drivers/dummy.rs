// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Dummy soundcard
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_ulonglong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type bool_t = bool;
type u32 = c_uint;
type u64 = c_ulonglong;
type size_t = usize;
type snd_pcm_uframes_t = c_ulong;
type snd_pcm_format_t = c_int;
type ktime_t = i64;
type spinlock_t = c_ulong;
type atomic_t = c_int;

const MAX_PCM_DEVICES: c_int = 4;
const MAX_PCM_SUBSTREAMS: c_int = 128;
const MAX_MIDI_DEVICES: c_int = 2;

/* defaults */
const MAX_BUFFER_SIZE: size_t = 64 * 1024;
const MIN_PERIOD_SIZE: size_t = 64;
const MAX_PERIOD_SIZE: size_t = MAX_BUFFER_SIZE;
const USE_FORMATS: u64 = SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE;
const USE_RATE: c_uint = SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000;
const USE_RATE_MIN: c_uint = 5500;
const USE_RATE_MAX: c_uint = 48000;
const USE_CHANNELS_MIN: c_uint = 1;
const USE_CHANNELS_MAX: c_uint = 2;
const USE_PERIODS_MIN: c_uint = 1;
const USE_PERIODS_MAX: c_uint = 1024;
const USE_MIXER_VOLUME_LEVEL_MIN: c_int = -50;
const USE_MIXER_VOLUME_LEVEL_MAX: c_int = 100;

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX;
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR;
static mut enable: [bool_t; SNDRV_CARDS] = {
    let mut a = [false; SNDRV_CARDS];
    a[0] = true;
    a
};
static mut model: [*mut c_char; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS];
static mut pcm_devs: [c_int; SNDRV_CARDS] = [1; SNDRV_CARDS];
static mut pcm_substreams: [c_int; SNDRV_CARDS] = [8; SNDRV_CARDS];
// static mut midi_devs: [c_int; SNDRV_CARDS] = [2; SNDRV_CARDS];
static mut mixer_volume_level_min: c_int = USE_MIXER_VOLUME_LEVEL_MIN;
static mut mixer_volume_level_max: c_int = USE_MIXER_VOLUME_LEVEL_MAX;
/* CONFIG_HIGH_RES_TIMERS */
static mut hrtimer: bool_t = true;
static mut fake_buffer: bool_t = true;

/* module_param_array/module_param/MODULE_PARM_DESC declarations are Linux module metadata. */

static mut devices: [*mut platform_device; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS];

const MIXER_ADDR_MASTER: usize = 0;
const MIXER_ADDR_LINE: usize = 1;
const MIXER_ADDR_MIC: usize = 2;
const MIXER_ADDR_SYNTH: usize = 3;
const MIXER_ADDR_CD: usize = 4;
const MIXER_ADDR_LAST: usize = 4;

#[repr(C)]
struct dummy_timer_ops {
    create: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    free: Option<unsafe extern "C" fn(*mut snd_pcm_substream)>,
    prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    start: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    stop: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    sync_stop: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}

unsafe fn get_dummy_ops(substream: *mut snd_pcm_substream) -> *mut *const dummy_timer_ops {
    (*(*substream).runtime).private_data as *mut *const dummy_timer_ops
}

#[repr(C)]
struct dummy_model {
    name: *const c_char,
    playback_constraints: Option<unsafe extern "C" fn(*mut snd_pcm_runtime) -> c_int>,
    capture_constraints: Option<unsafe extern "C" fn(*mut snd_pcm_runtime) -> c_int>,
    formats: u64,
    buffer_bytes_max: size_t,
    period_bytes_min: size_t,
    period_bytes_max: size_t,
    periods_min: c_uint,
    periods_max: c_uint,
    rates: c_uint,
    rate_min: c_uint,
    rate_max: c_uint,
    channels_min: c_uint,
    channels_max: c_uint,
}

#[repr(C)]
struct snd_dummy {
    card: *mut snd_card,
    model: *const dummy_model,
    pcm: *mut snd_pcm,
    pcm_hw: snd_pcm_hardware,
    mixer_lock: spinlock_t,
    mixer_volume: [[c_int; 2]; MIXER_ADDR_LAST + 1],
    capture_source: [[c_int; 2]; MIXER_ADDR_LAST + 1],
    iobox: c_int,
    cd_volume_ctl: *mut snd_kcontrol,
    cd_switch_ctl: *mut snd_kcontrol,
}

/*
 * card models
 */

unsafe extern "C" fn emu10k1_playback_constraints(runtime: *mut snd_pcm_runtime) -> c_int {
    let mut err: c_int;
    err = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if err < 0 {
        return err;
    }
    err = snd_pcm_hw_constraint_minmax(runtime, SNDRV_PCM_HW_PARAM_BUFFER_BYTES, 256, UINT_MAX);
    if err < 0 {
        return err;
    }
    0
}

static model_emu10k1: dummy_model = dummy_model {
    name: c"emu10k1".as_ptr(),
    playback_constraints: Some(emu10k1_playback_constraints),
    capture_constraints: None,
    formats: 0,
    buffer_bytes_max: 128 * 1024,
    period_bytes_min: 0,
    period_bytes_max: 0,
    periods_min: 0,
    periods_max: 0,
    rates: 0,
    rate_min: 0,
    rate_max: 0,
    channels_min: 0,
    channels_max: 0,
};

static model_rme9652: dummy_model = dummy_model {
    name: c"rme9652".as_ptr(),
    playback_constraints: None,
    capture_constraints: None,
    formats: SNDRV_PCM_FMTBIT_S32_LE,
    buffer_bytes_max: 26 * 64 * 1024,
    period_bytes_min: 0,
    period_bytes_max: 0,
    periods_min: 2,
    periods_max: 2,
    rates: 0,
    rate_min: 0,
    rate_max: 0,
    channels_min: 26,
    channels_max: 26,
};

static model_ice1712: dummy_model = dummy_model {
    name: c"ice1712".as_ptr(),
    playback_constraints: None,
    capture_constraints: None,
    formats: SNDRV_PCM_FMTBIT_S32_LE,
    buffer_bytes_max: 256 * 1024,
    period_bytes_min: 0,
    period_bytes_max: 0,
    periods_min: 1,
    periods_max: 1024,
    rates: 0,
    rate_min: 0,
    rate_max: 0,
    channels_min: 10,
    channels_max: 10,
};

static model_uda1341: dummy_model = dummy_model {
    name: c"uda1341".as_ptr(),
    playback_constraints: None,
    capture_constraints: None,
    formats: SNDRV_PCM_FMTBIT_S16_LE,
    buffer_bytes_max: 16380,
    period_bytes_min: 0,
    period_bytes_max: 0,
    periods_min: 2,
    periods_max: 255,
    rates: 0,
    rate_min: 0,
    rate_max: 0,
    channels_min: 2,
    channels_max: 2,
};

static model_ac97: dummy_model = dummy_model {
    name: c"ac97".as_ptr(),
    playback_constraints: None,
    capture_constraints: None,
    formats: SNDRV_PCM_FMTBIT_S16_LE,
    buffer_bytes_max: 0,
    period_bytes_min: 0,
    period_bytes_max: 0,
    periods_min: 0,
    periods_max: 0,
    rates: SNDRV_PCM_RATE_48000,
    rate_min: 48000,
    rate_max: 48000,
    channels_min: 2,
    channels_max: 2,
};

static model_ca0106: dummy_model = dummy_model {
    name: c"ca0106".as_ptr(),
    playback_constraints: None,
    capture_constraints: None,
    formats: SNDRV_PCM_FMTBIT_S16_LE,
    buffer_bytes_max: (65536 - 64) * 8,
    period_bytes_min: 0,
    period_bytes_max: 65536 - 64,
    periods_min: 2,
    periods_max: 8,
    rates: SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000,
    rate_min: 48000,
    rate_max: 192000,
    channels_min: 2,
    channels_max: 2,
};

static dummy_models: [*const dummy_model; 7] = [
    &model_emu10k1,
    &model_rme9652,
    &model_ice1712,
    &model_uda1341,
    &model_ac97,
    &model_ca0106,
    ptr::null(),
];

/*
 * system timer interface
 */

#[repr(C)]
struct dummy_systimer_pcm {
    /* ops must be the first item */
    timer_ops: *const dummy_timer_ops,
    lock: spinlock_t,
    timer: timer_list,
    base_time: c_ulong,
    frac_pos: c_uint, /* fractional sample position (based HZ) */
    frac_period_rest: c_uint,
    frac_buffer_size: c_uint, /* buffer_size * HZ */
    frac_period_size: c_uint, /* period_size * HZ */
    rate: c_uint,
    elapsed: c_int,
    substream: *mut snd_pcm_substream,
}

unsafe fn dummy_systimer_rearm(dpcm: *mut dummy_systimer_pcm) {
    mod_timer(&mut (*dpcm).timer, jiffies + DIV_ROUND_UP((*dpcm).frac_period_rest, (*dpcm).rate) as c_ulong);
}

unsafe fn dummy_systimer_update(dpcm: *mut dummy_systimer_pcm) {
    let mut delta: c_ulong;

    delta = jiffies.wrapping_sub((*dpcm).base_time);
    if delta == 0 {
        return;
    }
    (*dpcm).base_time = (*dpcm).base_time.wrapping_add(delta);
    delta = delta.wrapping_mul((*dpcm).rate as c_ulong);
    (*dpcm).frac_pos = (*dpcm).frac_pos.wrapping_add(delta as c_uint);
    while (*dpcm).frac_pos >= (*dpcm).frac_buffer_size {
        (*dpcm).frac_pos = (*dpcm).frac_pos.wrapping_sub((*dpcm).frac_buffer_size);
    }
    while (*dpcm).frac_period_rest as c_ulong <= delta {
        (*dpcm).elapsed += 1;
        (*dpcm).frac_period_rest = (*dpcm).frac_period_rest.wrapping_add((*dpcm).frac_period_size);
    }
    (*dpcm).frac_period_rest = (*dpcm).frac_period_rest.wrapping_sub(delta as c_uint);
}

unsafe extern "C" fn dummy_systimer_start(substream: *mut snd_pcm_substream) -> c_int {
    let dpcm = (*(*substream).runtime).private_data as *mut dummy_systimer_pcm;

    spin_lock(&mut (*dpcm).lock);
    (*dpcm).base_time = jiffies;
    dummy_systimer_rearm(dpcm);
    spin_unlock(&mut (*dpcm).lock);
    0
}

unsafe extern "C" fn dummy_systimer_stop(substream: *mut snd_pcm_substream) -> c_int {
    let dpcm = (*(*substream).runtime).private_data as *mut dummy_systimer_pcm;

    spin_lock(&mut (*dpcm).lock);
    timer_delete(&mut (*dpcm).timer);
    spin_unlock(&mut (*dpcm).lock);
    0
}

unsafe extern "C" fn dummy_systimer_sync_stop(substream: *mut snd_pcm_substream) -> c_int {
    let dpcm = (*(*substream).runtime).private_data as *mut dummy_systimer_pcm;

    timer_delete_sync(&mut (*dpcm).timer);
    0
}

unsafe extern "C" fn dummy_systimer_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let dpcm = (*runtime).private_data as *mut dummy_systimer_pcm;

    (*dpcm).frac_pos = 0;
    (*dpcm).rate = (*runtime).rate;
    (*dpcm).frac_buffer_size = (*runtime).buffer_size * HZ;
    (*dpcm).frac_period_size = (*runtime).period_size * HZ;
    (*dpcm).frac_period_rest = (*dpcm).frac_period_size;
    (*dpcm).elapsed = 0;

    0
}

unsafe extern "C" fn dummy_systimer_callback(t: *mut timer_list) {
    let dpcm = container_of_timer(t);
    let mut elapsed: c_int = 0;

    spin_lock_irqsave(&mut (*dpcm).lock);
    dummy_systimer_update(dpcm);
    dummy_systimer_rearm(dpcm);
    elapsed = (*dpcm).elapsed;
    (*dpcm).elapsed = 0;
    spin_unlock_irqrestore(&mut (*dpcm).lock);
    if elapsed != 0 {
        snd_pcm_period_elapsed((*dpcm).substream);
    }
}

unsafe extern "C" fn dummy_systimer_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let dpcm = (*(*substream).runtime).private_data as *mut dummy_systimer_pcm;

    spin_lock(&mut (*dpcm).lock);
    dummy_systimer_update(dpcm);
    let ret = ((*dpcm).frac_pos / HZ) as snd_pcm_uframes_t;
    spin_unlock(&mut (*dpcm).lock);
    ret
}

unsafe extern "C" fn dummy_systimer_create(substream: *mut snd_pcm_substream) -> c_int {
    let dpcm: *mut dummy_systimer_pcm;

    dpcm = kzalloc_obj::<dummy_systimer_pcm>();
    if dpcm.is_null() {
        return -ENOMEM;
    }
    (*(*substream).runtime).private_data = dpcm as *mut c_void;
    timer_setup(&mut (*dpcm).timer, Some(dummy_systimer_callback), 0);
    spin_lock_init(&mut (*dpcm).lock);
    (*dpcm).substream = substream;
    0
}

unsafe extern "C" fn dummy_systimer_free(substream: *mut snd_pcm_substream) {
    let dpcm = (*(*substream).runtime).private_data as *mut dummy_systimer_pcm;

    timer_shutdown_sync(&mut (*dpcm).timer);
    kfree(dpcm as *mut c_void);
}

static dummy_systimer_ops: dummy_timer_ops = dummy_timer_ops {
    create: Some(dummy_systimer_create),
    free: Some(dummy_systimer_free),
    prepare: Some(dummy_systimer_prepare),
    start: Some(dummy_systimer_start),
    stop: Some(dummy_systimer_stop),
    sync_stop: Some(dummy_systimer_sync_stop),
    pointer: Some(dummy_systimer_pointer),
};

/* CONFIG_HIGH_RES_TIMERS */
/*
 * hrtimer interface
 */

#[repr(C)]
struct dummy_hrtimer_pcm {
    /* ops must be the first item */
    timer_ops: *const dummy_timer_ops,
    base_time: ktime_t,
    period_time: ktime_t,
    running: atomic_t,
    timer: hrtimer,
    substream: *mut snd_pcm_substream,
}

unsafe extern "C" fn dummy_hrtimer_callback(timer: *mut hrtimer) -> hrtimer_restart {
    let dpcm: *mut dummy_hrtimer_pcm;

    dpcm = container_of_hrtimer(timer);
    if atomic_read(&mut (*dpcm).running) == 0 {
        return HRTIMER_NORESTART;
    }
    /*
     * In cases of XRUN and draining, this calls .trigger to stop PCM
     * substream.
     */
    snd_pcm_period_elapsed((*dpcm).substream);
    if atomic_read(&mut (*dpcm).running) == 0 {
        return HRTIMER_NORESTART;
    }

    hrtimer_forward_now(timer, (*dpcm).period_time);
    HRTIMER_RESTART
}

unsafe extern "C" fn dummy_hrtimer_start(substream: *mut snd_pcm_substream) -> c_int {
    let dpcm = (*(*substream).runtime).private_data as *mut dummy_hrtimer_pcm;

    (*dpcm).base_time = hrtimer_cb_get_time(&mut (*dpcm).timer);
    hrtimer_start(&mut (*dpcm).timer, (*dpcm).period_time, HRTIMER_MODE_REL_SOFT);
    atomic_set(&mut (*dpcm).running, 1);
    0
}

unsafe extern "C" fn dummy_hrtimer_stop(substream: *mut snd_pcm_substream) -> c_int {
    let dpcm = (*(*substream).runtime).private_data as *mut dummy_hrtimer_pcm;

    atomic_set(&mut (*dpcm).running, 0);
    if hrtimer_callback_running(&mut (*dpcm).timer) == 0 {
        hrtimer_cancel(&mut (*dpcm).timer);
    }
    0
}

unsafe extern "C" fn dummy_hrtimer_sync_stop(substream: *mut snd_pcm_substream) -> c_int {
    let dpcm = (*(*substream).runtime).private_data as *mut dummy_hrtimer_pcm;

    hrtimer_cancel(&mut (*dpcm).timer);
    0
}

unsafe extern "C" fn dummy_hrtimer_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let runtime = (*substream).runtime;
    let dpcm = (*runtime).private_data as *mut dummy_hrtimer_pcm;
    let mut delta: u64;
    let mut pos: u32 = 0;

    delta = ktime_us_delta(hrtimer_cb_get_time(&mut (*dpcm).timer), (*dpcm).base_time) as u64;
    delta = div_u64(delta.wrapping_mul((*runtime).rate as u64).wrapping_add(999999), 1000000);
    div_u64_rem(delta, (*runtime).buffer_size as u64, &mut pos);
    pos as snd_pcm_uframes_t
}

unsafe extern "C" fn dummy_hrtimer_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let dpcm = (*runtime).private_data as *mut dummy_hrtimer_pcm;
    let mut period: c_uint;
    let rate: c_uint;
    let sec: c_long;
    let nsecs: c_ulong;

    period = (*runtime).period_size;
    rate = (*runtime).rate;
    sec = (period / rate) as c_long;
    period %= rate;
    nsecs = div_u64((period as u64).wrapping_mul(1000000000).wrapping_add(rate as u64).wrapping_sub(1), rate as u64) as c_ulong;
    (*dpcm).period_time = ktime_set(sec, nsecs);

    0
}

unsafe extern "C" fn dummy_hrtimer_create(substream: *mut snd_pcm_substream) -> c_int {
    let dpcm: *mut dummy_hrtimer_pcm;

    dpcm = kzalloc_obj::<dummy_hrtimer_pcm>();
    if dpcm.is_null() {
        return -ENOMEM;
    }
    (*(*substream).runtime).private_data = dpcm as *mut c_void;
    hrtimer_setup(&mut (*dpcm).timer, Some(dummy_hrtimer_callback), CLOCK_MONOTONIC, HRTIMER_MODE_REL_SOFT);
    (*dpcm).substream = substream;
    atomic_set(&mut (*dpcm).running, 0);
    0
}

unsafe extern "C" fn dummy_hrtimer_free(substream: *mut snd_pcm_substream) {
    let dpcm = (*(*substream).runtime).private_data as *mut dummy_hrtimer_pcm;

    kfree(dpcm as *mut c_void);
}

static dummy_hrtimer_ops: dummy_timer_ops = dummy_timer_ops {
    create: Some(dummy_hrtimer_create),
    free: Some(dummy_hrtimer_free),
    prepare: Some(dummy_hrtimer_prepare),
    start: Some(dummy_hrtimer_start),
    stop: Some(dummy_hrtimer_stop),
    sync_stop: Some(dummy_hrtimer_sync_stop),
    pointer: Some(dummy_hrtimer_pointer),
};

/*
 * PCM interface
 */

unsafe extern "C" fn dummy_pcm_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => ((*(*get_dummy_ops(substream))).start.unwrap())(substream),
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => ((*(*get_dummy_ops(substream))).stop.unwrap())(substream),
        _ => -EINVAL,
    }
}

unsafe extern "C" fn dummy_pcm_sync_stop(substream: *mut snd_pcm_substream) -> c_int {
    if let Some(sync_stop) = (*(*get_dummy_ops(substream))).sync_stop {
        return sync_stop(substream);
    }
    0
}

unsafe extern "C" fn dummy_pcm_prepare(substream: *mut snd_pcm_substream) -> c_int {
    ((*(*get_dummy_ops(substream))).prepare.unwrap())(substream)
}

unsafe extern "C" fn dummy_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    ((*(*get_dummy_ops(substream))).pointer.unwrap())(substream)
}

static dummy_pcm_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_RESUME | SNDRV_PCM_INFO_MMAP_VALID,
    formats: USE_FORMATS,
    rates: USE_RATE,
    rate_min: USE_RATE_MIN,
    rate_max: USE_RATE_MAX,
    channels_min: USE_CHANNELS_MIN,
    channels_max: USE_CHANNELS_MAX,
    buffer_bytes_max: MAX_BUFFER_SIZE,
    period_bytes_min: MIN_PERIOD_SIZE,
    period_bytes_max: MAX_PERIOD_SIZE,
    periods_min: USE_PERIODS_MIN,
    periods_max: USE_PERIODS_MAX,
    fifo_size: 0,
};

unsafe extern "C" fn dummy_pcm_hw_params(substream: *mut snd_pcm_substream, hw_params: *mut snd_pcm_hw_params) -> c_int {
    if fake_buffer {
        /* runtime->dma_bytes has to be set manually to allow mmap */
        (*(*substream).runtime).dma_bytes = params_buffer_bytes(hw_params);
        return 0;
    }
    0
}

unsafe extern "C" fn dummy_pcm_open(substream: *mut snd_pcm_substream) -> c_int {
    let dummy = snd_pcm_substream_chip(substream) as *mut snd_dummy;
    let model = (*dummy).model;
    let runtime = (*substream).runtime;
    let mut ops: *const dummy_timer_ops;
    let mut err: c_int;

    ops = &dummy_systimer_ops;
    /* CONFIG_HIGH_RES_TIMERS */
    if hrtimer {
        ops = &dummy_hrtimer_ops;
    }

    err = ((*ops).create.unwrap())(substream);
    if err < 0 {
        return err;
    }
    *get_dummy_ops(substream) = ops;

    (*runtime).hw = (*dummy).pcm_hw;
    if (*(*substream).pcm).device & 1 != 0 {
        (*runtime).hw.info &= !SNDRV_PCM_INFO_INTERLEAVED;
        (*runtime).hw.info |= SNDRV_PCM_INFO_NONINTERLEAVED;
    }
    if (*(*substream).pcm).device & 2 != 0 {
        (*runtime).hw.info &= !(SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID);
    }

    if model.is_null() {
        return 0;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        if let Some(playback_constraints) = (*model).playback_constraints {
            err = playback_constraints((*substream).runtime);
        }
    } else if let Some(capture_constraints) = (*model).capture_constraints {
        err = capture_constraints((*substream).runtime);
    }
    if err < 0 {
        ((*(*get_dummy_ops(substream))).free.unwrap())(substream);
        return err;
    }
    0
}

unsafe extern "C" fn dummy_pcm_close(substream: *mut snd_pcm_substream) -> c_int {
    ((*(*get_dummy_ops(substream))).free.unwrap())(substream);
    0
}

/*
 * dummy buffer handling
 */

static mut dummy_page: [*mut c_void; 2] = [ptr::null_mut(); 2];

unsafe fn free_fake_buffer() {
    if fake_buffer {
        let mut i: c_int;
        i = 0;
        while i < 2 {
            if !dummy_page[i as usize].is_null() {
                free_page(dummy_page[i as usize] as c_ulong);
                dummy_page[i as usize] = ptr::null_mut();
            }
            i += 1;
        }
    }
}

unsafe fn alloc_fake_buffer() -> c_int {
    let mut i: c_int;

    if !fake_buffer {
        return 0;
    }
    i = 0;
    while i < 2 {
        dummy_page[i as usize] = get_zeroed_page(GFP_KERNEL) as *mut c_void;
        if dummy_page[i as usize].is_null() {
            free_fake_buffer();
            return -ENOMEM;
        }
        i += 1;
    }
    0
}

unsafe extern "C" fn dummy_pcm_copy(
    _substream: *mut snd_pcm_substream,
    _channel: c_int,
    _pos: c_ulong,
    _iter: *mut iov_iter,
    _bytes: c_ulong,
) -> c_int {
    0 /* do nothing */
}

unsafe extern "C" fn dummy_pcm_silence(
    _substream: *mut snd_pcm_substream,
    _channel: c_int,
    _pos: c_ulong,
    _bytes: c_ulong,
) -> c_int {
    0 /* do nothing */
}

unsafe extern "C" fn dummy_pcm_page(substream: *mut snd_pcm_substream, _offset: c_ulong) -> *mut page {
    virt_to_page(dummy_page[(*substream).stream as usize]) /* the same page */
}

static dummy_pcm_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(dummy_pcm_open),
    close: Some(dummy_pcm_close),
    hw_params: Some(dummy_pcm_hw_params),
    prepare: Some(dummy_pcm_prepare),
    trigger: Some(dummy_pcm_trigger),
    sync_stop: Some(dummy_pcm_sync_stop),
    pointer: Some(dummy_pcm_pointer),
    copy: None,
    fill_silence: None,
    page: None,
};

static dummy_pcm_ops_no_buf: snd_pcm_ops = snd_pcm_ops {
    open: Some(dummy_pcm_open),
    close: Some(dummy_pcm_close),
    hw_params: Some(dummy_pcm_hw_params),
    prepare: Some(dummy_pcm_prepare),
    trigger: Some(dummy_pcm_trigger),
    sync_stop: Some(dummy_pcm_sync_stop),
    pointer: Some(dummy_pcm_pointer),
    copy: Some(dummy_pcm_copy),
    fill_silence: Some(dummy_pcm_silence),
    page: Some(dummy_pcm_page),
};

unsafe fn snd_card_dummy_pcm(dummy: *mut snd_dummy, device: c_int, substreams: c_int) -> c_int {
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let ops: *const snd_pcm_ops;
    let err: c_int;

    err = snd_pcm_new((*dummy).card, c"Dummy PCM".as_ptr(), device, substreams, substreams, &mut pcm);
    if err < 0 {
        return err;
    }
    (*dummy).pcm = pcm;
    if fake_buffer {
        ops = &dummy_pcm_ops_no_buf;
    } else {
        ops = &dummy_pcm_ops;
    }
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, ops);
    (*pcm).private_data = dummy as *mut c_void;
    (*pcm).info_flags = 0;
    strscpy((*pcm).name.as_mut_ptr(), c"Dummy PCM".as_ptr());
    if !fake_buffer {
        snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_CONTINUOUS, ptr::null_mut(), 0, 64 * 1024);
    }
    0
}

/*
 * mixer interface
 */

const db_scale_dummy: [c_uint; 4] = TLV_DB_SCALE(-4500, 30, 0);

unsafe extern "C" fn snd_dummy_volume_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = mixer_volume_level_min as c_long;
    (*uinfo).value.integer.max = mixer_volume_level_max as c_long;
    0
}

unsafe extern "C" fn snd_dummy_volume_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dummy = snd_kcontrol_chip(kcontrol) as *mut snd_dummy;
    let addr = (*kcontrol).private_value as usize;

    spin_lock_irq(&mut (*dummy).mixer_lock);
    (*ucontrol).value.integer.value[0] = (*dummy).mixer_volume[addr][0] as c_long;
    (*ucontrol).value.integer.value[1] = (*dummy).mixer_volume[addr][1] as c_long;
    spin_unlock_irq(&mut (*dummy).mixer_lock);
    0
}

unsafe extern "C" fn snd_dummy_volume_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dummy = snd_kcontrol_chip(kcontrol) as *mut snd_dummy;
    let addr = (*kcontrol).private_value as usize;
    let change: c_int;
    let mut left: c_int;
    let mut right: c_int;

    left = (*ucontrol).value.integer.value[0] as c_int;
    if left < mixer_volume_level_min {
        left = mixer_volume_level_min;
    }
    if left > mixer_volume_level_max {
        left = mixer_volume_level_max;
    }
    right = (*ucontrol).value.integer.value[1] as c_int;
    if right < mixer_volume_level_min {
        right = mixer_volume_level_min;
    }
    if right > mixer_volume_level_max {
        right = mixer_volume_level_max;
    }
    spin_lock_irq(&mut (*dummy).mixer_lock);
    change = ((*dummy).mixer_volume[addr][0] != left || (*dummy).mixer_volume[addr][1] != right) as c_int;
    (*dummy).mixer_volume[addr][0] = left;
    (*dummy).mixer_volume[addr][1] = right;
    spin_unlock_irq(&mut (*dummy).mixer_lock);
    change
}

unsafe extern "C" fn snd_dummy_capsrc_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dummy = snd_kcontrol_chip(kcontrol) as *mut snd_dummy;
    let addr = (*kcontrol).private_value as usize;

    spin_lock_irq(&mut (*dummy).mixer_lock);
    (*ucontrol).value.integer.value[0] = (*dummy).capture_source[addr][0] as c_long;
    (*ucontrol).value.integer.value[1] = (*dummy).capture_source[addr][1] as c_long;
    spin_unlock_irq(&mut (*dummy).mixer_lock);
    0
}

unsafe extern "C" fn snd_dummy_capsrc_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dummy = snd_kcontrol_chip(kcontrol) as *mut snd_dummy;
    let addr = (*kcontrol).private_value as usize;
    let change: c_int;
    let left: c_int;
    let right: c_int;

    left = ((*ucontrol).value.integer.value[0] & 1) as c_int;
    right = ((*ucontrol).value.integer.value[1] & 1) as c_int;
    spin_lock_irq(&mut (*dummy).mixer_lock);
    change = ((*dummy).capture_source[addr][0] != left && (*dummy).capture_source[addr][1] != right) as c_int;
    (*dummy).capture_source[addr][0] = left;
    (*dummy).capture_source[addr][1] = right;
    spin_unlock_irq(&mut (*dummy).mixer_lock);
    change
}

unsafe extern "C" fn snd_dummy_iobox_info(_kcontrol: *mut snd_kcontrol, info: *mut snd_ctl_elem_info) -> c_int {
    static names: [*const c_char; 2] = [c"None".as_ptr(), c"CD Player".as_ptr()];

    snd_ctl_enum_info(info, 1, 2, names.as_ptr())
}

unsafe extern "C" fn snd_dummy_iobox_get(kcontrol: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    let dummy = snd_kcontrol_chip(kcontrol) as *mut snd_dummy;

    (*value).value.enumerated.item[0] = (*dummy).iobox as c_uint;
    0
}

unsafe extern "C" fn snd_dummy_iobox_put(kcontrol: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    let dummy = snd_kcontrol_chip(kcontrol) as *mut snd_dummy;
    let changed: c_int;

    if (*value).value.enumerated.item[0] > 1 {
        return -EINVAL;
    }

    changed = ((*value).value.enumerated.item[0] as c_int != (*dummy).iobox) as c_int;
    if changed != 0 {
        (*dummy).iobox = (*value).value.enumerated.item[0] as c_int;

        if (*dummy).iobox != 0 {
            (*(*dummy).cd_volume_ctl).vd[0].access &= !SNDRV_CTL_ELEM_ACCESS_INACTIVE;
            (*(*dummy).cd_switch_ctl).vd[0].access &= !SNDRV_CTL_ELEM_ACCESS_INACTIVE;
        } else {
            (*(*dummy).cd_volume_ctl).vd[0].access |= SNDRV_CTL_ELEM_ACCESS_INACTIVE;
            (*(*dummy).cd_switch_ctl).vd[0].access |= SNDRV_CTL_ELEM_ACCESS_INACTIVE;
        }

        snd_ctl_notify((*dummy).card, SNDRV_CTL_EVENT_MASK_INFO, &mut (*(*dummy).cd_volume_ctl).id);
        snd_ctl_notify((*dummy).card, SNDRV_CTL_EVENT_MASK_INFO, &mut (*(*dummy).cd_switch_ctl).id);
    }

    changed
}

const fn DUMMY_VOLUME(xname: *const c_char, xindex: c_uint, addr: usize) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
        name: xname,
        index: xindex,
        info: Some(snd_dummy_volume_info),
        get: Some(snd_dummy_volume_get),
        put: Some(snd_dummy_volume_put),
        private_value: addr as c_ulong,
        tlv: snd_kcontrol_tlv { p: db_scale_dummy.as_ptr() },
    }
}

const fn DUMMY_CAPSRC(xname: *const c_char, xindex: c_uint, addr: usize) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        access: 0,
        name: xname,
        index: xindex,
        info: Some(snd_ctl_boolean_stereo_info),
        get: Some(snd_dummy_capsrc_get),
        put: Some(snd_dummy_capsrc_put),
        private_value: addr as c_ulong,
        tlv: snd_kcontrol_tlv { p: ptr::null() },
    }
}

static snd_dummy_controls: [snd_kcontrol_new; 11] = [
    DUMMY_VOLUME(c"Master Volume".as_ptr(), 0, MIXER_ADDR_MASTER),
    DUMMY_CAPSRC(c"Master Capture Switch".as_ptr(), 0, MIXER_ADDR_MASTER),
    DUMMY_VOLUME(c"Synth Volume".as_ptr(), 0, MIXER_ADDR_SYNTH),
    DUMMY_CAPSRC(c"Synth Capture Switch".as_ptr(), 0, MIXER_ADDR_SYNTH),
    DUMMY_VOLUME(c"Line Volume".as_ptr(), 0, MIXER_ADDR_LINE),
    DUMMY_CAPSRC(c"Line Capture Switch".as_ptr(), 0, MIXER_ADDR_LINE),
    DUMMY_VOLUME(c"Mic Volume".as_ptr(), 0, MIXER_ADDR_MIC),
    DUMMY_CAPSRC(c"Mic Capture Switch".as_ptr(), 0, MIXER_ADDR_MIC),
    DUMMY_VOLUME(c"CD Volume".as_ptr(), 0, MIXER_ADDR_CD),
    DUMMY_CAPSRC(c"CD Capture Switch".as_ptr(), 0, MIXER_ADDR_CD),
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        access: 0,
        name: c"External I/O Box".as_ptr(),
        index: 0,
        info: Some(snd_dummy_iobox_info),
        get: Some(snd_dummy_iobox_get),
        put: Some(snd_dummy_iobox_put),
        private_value: 0,
        tlv: snd_kcontrol_tlv { p: ptr::null() },
    },
];

unsafe fn snd_card_dummy_new_mixer(dummy: *mut snd_dummy) -> c_int {
    let card = (*dummy).card;
    let mut kcontrol: *mut snd_kcontrol;
    let mut idx: c_uint;
    let mut err: c_int;

    spin_lock_init(&mut (*dummy).mixer_lock);
    strscpy((*card).mixername.as_mut_ptr(), c"Dummy Mixer".as_ptr());
    (*dummy).iobox = 1;

    idx = 0;
    while (idx as usize) < snd_dummy_controls.len() {
        kcontrol = snd_ctl_new1(&snd_dummy_controls[idx as usize], dummy as *mut c_void);
        err = snd_ctl_add(card, kcontrol);
        if err < 0 {
            return err;
        }
        if strcmp((*kcontrol).id.name.as_ptr(), c"CD Volume".as_ptr()) == 0 {
            (*dummy).cd_volume_ctl = kcontrol;
        } else if strcmp((*kcontrol).id.name.as_ptr(), c"CD Capture Switch".as_ptr()) == 0 {
            (*dummy).cd_switch_ctl = kcontrol;
        }

        idx += 1;
    }
    0
}

/* CONFIG_SND_DEBUG && CONFIG_SND_PROC_FS */
/*
 * proc interface
 */
unsafe fn print_formats(dummy: *mut snd_dummy, buffer: *mut snd_info_buffer) {
    let mut i: snd_pcm_format_t = 0;

    while pcm_format_valid(i) {
        if (*dummy).pcm_hw.formats & pcm_format_to_bits(i) != 0 {
            snd_iprintf(buffer, c" %s".as_ptr(), snd_pcm_format_name(i));
        }
        i += 1;
    }
}

unsafe fn print_rates(dummy: *mut snd_dummy, buffer: *mut snd_info_buffer) {
    static rates: [c_int; 13] = [
        5512, 8000, 11025, 16000, 22050, 32000, 44100, 48000,
        64000, 88200, 96000, 176400, 192000,
    ];
    let mut i: c_int;

    if (*dummy).pcm_hw.rates & SNDRV_PCM_RATE_CONTINUOUS != 0 {
        snd_iprintf(buffer, c" continuous".as_ptr());
    }
    if (*dummy).pcm_hw.rates & SNDRV_PCM_RATE_KNOT != 0 {
        snd_iprintf(buffer, c" knot".as_ptr());
    }
    i = 0;
    while (i as usize) < rates.len() {
        if (*dummy).pcm_hw.rates & (1 << i) != 0 {
            snd_iprintf(buffer, c" %d".as_ptr(), rates[i as usize]);
        }
        i += 1;
    }
}

unsafe fn get_dummy_int_ptr(dummy: *mut snd_dummy, ofs: c_uint) -> *mut c_uint {
    (&mut (*dummy).pcm_hw as *mut snd_pcm_hardware as *mut c_char).add(ofs as usize) as *mut c_uint
}

unsafe fn get_dummy_ll_ptr(dummy: *mut snd_dummy, ofs: c_uint) -> *mut c_ulonglong {
    (&mut (*dummy).pcm_hw as *mut snd_pcm_hardware as *mut c_char).add(ofs as usize) as *mut c_ulonglong
}

#[repr(C)]
struct dummy_hw_field {
    name: *const c_char,
    format: *const c_char,
    offset: c_uint,
    size: c_uint,
}

const fn FIELD_ENTRY(name: *const c_char, format: *const c_char, offset: usize, size: usize) -> dummy_hw_field {
    dummy_hw_field {
        name,
        format,
        offset: offset as c_uint,
        size: size as c_uint,
    }
}

static fields: [dummy_hw_field; 11] = [
    FIELD_ENTRY(c"formats".as_ptr(), c"%#llx".as_ptr(), offset_of!(snd_pcm_hardware, formats), size_of::<u64>()),
    FIELD_ENTRY(c"rates".as_ptr(), c"%#x".as_ptr(), offset_of!(snd_pcm_hardware, rates), size_of::<c_uint>()),
    FIELD_ENTRY(c"rate_min".as_ptr(), c"%d".as_ptr(), offset_of!(snd_pcm_hardware, rate_min), size_of::<c_uint>()),
    FIELD_ENTRY(c"rate_max".as_ptr(), c"%d".as_ptr(), offset_of!(snd_pcm_hardware, rate_max), size_of::<c_uint>()),
    FIELD_ENTRY(c"channels_min".as_ptr(), c"%d".as_ptr(), offset_of!(snd_pcm_hardware, channels_min), size_of::<c_uint>()),
    FIELD_ENTRY(c"channels_max".as_ptr(), c"%d".as_ptr(), offset_of!(snd_pcm_hardware, channels_max), size_of::<c_uint>()),
    FIELD_ENTRY(c"buffer_bytes_max".as_ptr(), c"%ld".as_ptr(), offset_of!(snd_pcm_hardware, buffer_bytes_max), size_of::<size_t>()),
    FIELD_ENTRY(c"period_bytes_min".as_ptr(), c"%ld".as_ptr(), offset_of!(snd_pcm_hardware, period_bytes_min), size_of::<size_t>()),
    FIELD_ENTRY(c"period_bytes_max".as_ptr(), c"%ld".as_ptr(), offset_of!(snd_pcm_hardware, period_bytes_max), size_of::<size_t>()),
    FIELD_ENTRY(c"periods_min".as_ptr(), c"%d".as_ptr(), offset_of!(snd_pcm_hardware, periods_min), size_of::<c_uint>()),
    FIELD_ENTRY(c"periods_max".as_ptr(), c"%d".as_ptr(), offset_of!(snd_pcm_hardware, periods_max), size_of::<c_uint>()),
];

unsafe extern "C" fn dummy_proc_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let dummy = (*entry).private_data as *mut snd_dummy;
    let mut i: c_int;

    i = 0;
    while (i as usize) < fields.len() {
        snd_iprintf(buffer, c"%s ".as_ptr(), fields[i as usize].name);
        if fields[i as usize].size as usize == size_of::<c_int>() {
            snd_iprintf(buffer, fields[i as usize].format, *get_dummy_int_ptr(dummy, fields[i as usize].offset));
        } else {
            snd_iprintf(buffer, fields[i as usize].format, *get_dummy_ll_ptr(dummy, fields[i as usize].offset));
        }
        if strcmp(fields[i as usize].name, c"formats".as_ptr()) == 0 {
            print_formats(dummy, buffer);
        } else if strcmp(fields[i as usize].name, c"rates".as_ptr()) == 0 {
            print_rates(dummy, buffer);
        }
        snd_iprintf(buffer, c"\n".as_ptr());
        i += 1;
    }
}

unsafe extern "C" fn dummy_proc_write(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let dummy = (*entry).private_data as *mut snd_dummy;
    let mut line: [c_char; 64] = [0; 64];

    while snd_info_get_line(buffer, line.as_mut_ptr(), size_of::<[c_char; 64]>()) == 0 {
        let mut item: [c_char; 20] = [0; 20];
        let mut ptr_: *const c_char;
        let mut val: c_ulonglong = 0;
        let mut i: c_int;

        ptr_ = snd_info_get_str(item.as_mut_ptr(), line.as_ptr(), size_of::<[c_char; 20]>());
        i = 0;
        while (i as usize) < fields.len() {
            if strcmp(item.as_ptr(), fields[i as usize].name) == 0 {
                break;
            }
            i += 1;
        }
        if (i as usize) >= fields.len() {
            continue;
        }
        snd_info_get_str(item.as_mut_ptr(), ptr_, size_of::<[c_char; 20]>());
        if kstrtoull(item.as_ptr(), 0, &mut val) != 0 {
            continue;
        }
        if fields[i as usize].size as usize == size_of::<c_int>() {
            *get_dummy_int_ptr(dummy, fields[i as usize].offset) = val as c_uint;
        } else {
            *get_dummy_ll_ptr(dummy, fields[i as usize].offset) = val;
        }
    }
}

unsafe fn dummy_proc_init(chip: *mut snd_dummy) {
    snd_card_rw_proc_new((*chip).card, c"dummy_pcm".as_ptr(), chip as *mut c_void, Some(dummy_proc_read), Some(dummy_proc_write));
}

unsafe extern "C" fn snd_dummy_probe(devptr: *mut platform_device) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let mut dummy: *mut snd_dummy;
    let mut m: *const dummy_model = ptr::null();
    let mut mdl: *const *const dummy_model;
    let mut idx: c_int;
    let mut err: c_int;
    let mut dev: c_int = (*devptr).id;

    if dev < 0 || dev >= SNDRV_CARDS as c_int {
        dev_warn(&mut (*devptr).dev, c"Invalid card index %d, using default 0\n".as_ptr(), dev);
        dev = 0;
    }

    err = snd_devm_card_new(
        &mut (*devptr).dev,
        index[dev as usize],
        id[dev as usize],
        THIS_MODULE,
        size_of::<snd_dummy>(),
        &mut card,
    );
    if err < 0 {
        return err;
    }
    dummy = (*card).private_data as *mut snd_dummy;
    (*dummy).card = card;
    mdl = dummy_models.as_ptr();
    while !(*mdl).is_null() && !model[dev as usize].is_null() {
        if strcmp(model[dev as usize], (**mdl).name) == 0 {
            pr_info(c"snd-dummy: Using model '%s' for card %i\n".as_ptr(), (**mdl).name, (*card).number);
            m = *mdl;
            (*dummy).model = *mdl;
            break;
        }
        mdl = mdl.add(1);
    }
    idx = 0;
    while idx < MAX_PCM_DEVICES && idx < pcm_devs[dev as usize] {
        if pcm_substreams[dev as usize] < 1 {
            pcm_substreams[dev as usize] = 1;
        }
        if pcm_substreams[dev as usize] > MAX_PCM_SUBSTREAMS {
            pcm_substreams[dev as usize] = MAX_PCM_SUBSTREAMS;
        }
        err = snd_card_dummy_pcm(dummy, idx, pcm_substreams[dev as usize]);
        if err < 0 {
            return err;
        }
        idx += 1;
    }

    (*dummy).pcm_hw = dummy_pcm_hardware;
    if !m.is_null() {
        if (*m).formats != 0 {
            (*dummy).pcm_hw.formats = (*m).formats;
        }
        if (*m).buffer_bytes_max != 0 {
            (*dummy).pcm_hw.buffer_bytes_max = (*m).buffer_bytes_max;
        }
        if (*m).period_bytes_min != 0 {
            (*dummy).pcm_hw.period_bytes_min = (*m).period_bytes_min;
        }
        if (*m).period_bytes_max != 0 {
            (*dummy).pcm_hw.period_bytes_max = (*m).period_bytes_max;
        }
        if (*m).periods_min != 0 {
            (*dummy).pcm_hw.periods_min = (*m).periods_min;
        }
        if (*m).periods_max != 0 {
            (*dummy).pcm_hw.periods_max = (*m).periods_max;
        }
        if (*m).rates != 0 {
            (*dummy).pcm_hw.rates = (*m).rates;
        }
        if (*m).rate_min != 0 {
            (*dummy).pcm_hw.rate_min = (*m).rate_min;
        }
        if (*m).rate_max != 0 {
            (*dummy).pcm_hw.rate_max = (*m).rate_max;
        }
        if (*m).channels_min != 0 {
            (*dummy).pcm_hw.channels_min = (*m).channels_min;
        }
        if (*m).channels_max != 0 {
            (*dummy).pcm_hw.channels_max = (*m).channels_max;
        }
    }

    if mixer_volume_level_min > mixer_volume_level_max {
        pr_warn(
            c"snd-dummy: Invalid mixer volume level: min=%d, max=%d. Fall back to default value.\n".as_ptr(),
            mixer_volume_level_min,
            mixer_volume_level_max,
        );
        mixer_volume_level_min = USE_MIXER_VOLUME_LEVEL_MIN;
        mixer_volume_level_max = USE_MIXER_VOLUME_LEVEL_MAX;
    }
    err = snd_card_dummy_new_mixer(dummy);
    if err < 0 {
        return err;
    }
    strscpy((*card).driver.as_mut_ptr(), c"Dummy".as_ptr());
    strscpy((*card).shortname.as_mut_ptr(), c"Dummy".as_ptr());
    sprintf((*card).longname.as_mut_ptr(), c"Dummy %i".as_ptr(), dev + 1);

    dummy_proc_init(dummy);

    err = snd_card_register(card);
    if err < 0 {
        return err;
    }
    platform_set_drvdata(devptr, card as *mut c_void);
    0
}

unsafe extern "C" fn snd_dummy_suspend(pdev: *mut device) -> c_int {
    let card = dev_get_drvdata(pdev) as *mut snd_card;

    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    0
}

unsafe extern "C" fn snd_dummy_resume(pdev: *mut device) -> c_int {
    let card = dev_get_drvdata(pdev) as *mut snd_card;

    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    0
}

static snd_dummy_pm: dev_pm_ops = dev_pm_ops {
    suspend: Some(snd_dummy_suspend),
    resume: Some(snd_dummy_resume),
};

const SND_DUMMY_DRIVER: *const c_char = c"snd_dummy".as_ptr();

static mut snd_dummy_driver: platform_driver = platform_driver {
    probe: Some(snd_dummy_probe),
    driver: driver {
        name: SND_DUMMY_DRIVER,
        pm: &snd_dummy_pm,
    },
};

unsafe fn snd_dummy_unregister_all() {
    let mut i: c_int;

    i = 0;
    while (i as usize) < devices.len() {
        platform_device_unregister(devices[i as usize]);
        i += 1;
    }
    platform_driver_unregister(&mut snd_dummy_driver);
    free_fake_buffer();
}

unsafe extern "C" fn alsa_card_dummy_init() -> c_int {
    let mut i: c_int;
    let mut cards: c_int;
    let mut err: c_int;

    err = platform_driver_register(&mut snd_dummy_driver);
    if err < 0 {
        return err;
    }

    err = alloc_fake_buffer();
    if err < 0 {
        platform_driver_unregister(&mut snd_dummy_driver);
        return err;
    }

    cards = 0;
    i = 0;
    while i < SNDRV_CARDS as c_int {
        let device: *mut platform_device;
        if !enable[i as usize] {
            i += 1;
            continue;
        }
        device = platform_device_register_simple(SND_DUMMY_DRIVER, i, ptr::null_mut(), 0);
        if IS_ERR(device as *const c_void) {
            i += 1;
            continue;
        }
        if platform_get_drvdata(device).is_null() {
            platform_device_unregister(device);
            i += 1;
            continue;
        }
        devices[i as usize] = device;
        cards += 1;
        i += 1;
    }
    if cards == 0 {
        /* MODULE */
        pr_err(c"Dummy soundcard not found or device busy\n".as_ptr());
        snd_dummy_unregister_all();
        return -ENODEV;
    }
    0
}

unsafe extern "C" fn alsa_card_dummy_exit() {
    snd_dummy_unregister_all();
}

/* module_init(alsa_card_dummy_init) */
/* module_exit(alsa_card_dummy_exit) */

#[repr(C)]
struct platform_device {
    id: c_int,
    dev: device,
}
#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    driver: driver,
}
#[repr(C)]
struct driver {
    name: *const c_char,
    pm: *const dev_pm_ops,
}
#[repr(C)]
struct dev_pm_ops {
    suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}
#[repr(C)]
struct device {
    _priv: [u8; 0],
}
#[repr(C)]
struct snd_card {
    private_data: *mut c_void,
    number: c_int,
    mixername: [c_char; 80],
    driver: [c_char; 16],
    shortname: [c_char; 32],
    longname: [c_char; 80],
}
#[repr(C)]
struct snd_pcm {
    device: c_int,
    private_data: *mut c_void,
    info_flags: c_uint,
    name: [c_char; 80],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct snd_pcm_hardware {
    info: c_uint,
    formats: u64,
    rates: c_uint,
    rate_min: c_uint,
    rate_max: c_uint,
    channels_min: c_uint,
    channels_max: c_uint,
    buffer_bytes_max: size_t,
    period_bytes_min: size_t,
    period_bytes_max: size_t,
    periods_min: c_uint,
    periods_max: c_uint,
    fifo_size: c_uint,
}
#[repr(C)]
struct snd_pcm_runtime {
    private_data: *mut c_void,
    rate: c_uint,
    buffer_size: c_uint,
    period_size: c_uint,
    dma_bytes: size_t,
    hw: snd_pcm_hardware,
}
#[repr(C)]
struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
    pcm: *mut snd_pcm,
    stream: c_int,
}
#[repr(C)]
struct snd_pcm_hw_params {
    _priv: [u8; 0],
}
#[repr(C)]
struct timer_list {
    _priv: [u8; 0],
}
#[repr(C)]
struct hrtimer {
    _priv: [u8; 0],
}
type hrtimer_restart = c_int;
#[repr(C)]
struct iov_iter {
    _priv: [u8; 0],
}
#[repr(C)]
struct page {
    _priv: [u8; 0],
}
#[repr(C)]
struct snd_kcontrol_new {
    iface: c_uint,
    access: c_uint,
    name: *const c_char,
    index: c_uint,
    info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    private_value: c_ulong,
    tlv: snd_kcontrol_tlv,
}
#[repr(C)]
union snd_kcontrol_tlv {
    p: *const c_uint,
}
#[repr(C)]
struct snd_kcontrol {
    private_value: c_ulong,
    id: snd_ctl_elem_id,
    vd: [snd_kcontrol_volatile; 1],
}
#[repr(C)]
struct snd_kcontrol_volatile {
    access: c_uint,
}
#[repr(C)]
struct snd_ctl_elem_id {
    name: [c_char; 44],
}
#[repr(C)]
struct snd_ctl_elem_info {
    type_: c_uint,
    count: c_uint,
    value: snd_ctl_elem_info_value,
}
#[repr(C)]
union snd_ctl_elem_info_value {
    integer: snd_ctl_elem_info_integer,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct snd_ctl_elem_info_integer {
    min: c_long,
    max: c_long,
}
#[repr(C)]
struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_value,
}
#[repr(C)]
union snd_ctl_elem_value_value {
    integer: snd_ctl_elem_value_integer,
    enumerated: snd_ctl_elem_value_enumerated,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct snd_ctl_elem_value_integer {
    value: [c_long; 2],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct snd_ctl_elem_value_enumerated {
    item: [c_uint; 2],
}
#[repr(C)]
struct snd_pcm_ops {
    open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    sync_stop: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    copy: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, c_ulong, *mut iov_iter, c_ulong) -> c_int>,
    fill_silence: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, c_ulong, c_ulong) -> c_int>,
    page: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_ulong) -> *mut page>,
}
#[repr(C)]
struct snd_info_entry {
    private_data: *mut c_void,
}
#[repr(C)]
struct snd_info_buffer {
    _priv: [u8; 0],
}

extern "C" {
    static mut jiffies: c_ulong;
    static THIS_MODULE: *mut c_void;

    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, param: c_int) -> c_int;
    fn snd_pcm_hw_constraint_minmax(runtime: *mut snd_pcm_runtime, param: c_int, min: c_uint, max: c_uint) -> c_int;
    fn mod_timer(timer: *mut timer_list, expires: c_ulong) -> c_int;
    fn timer_delete(timer: *mut timer_list) -> c_int;
    fn timer_delete_sync(timer: *mut timer_list) -> c_int;
    fn timer_shutdown_sync(timer: *mut timer_list);
    fn timer_setup(timer: *mut timer_list, callback: Option<unsafe extern "C" fn(*mut timer_list)>, flags: c_uint);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t);
    fn kfree(ptr: *mut c_void);
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn hrtimer_cb_get_time(timer: *mut hrtimer) -> ktime_t;
    fn hrtimer_start(timer: *mut hrtimer, time: ktime_t, mode: c_int) -> c_int;
    fn hrtimer_callback_running(timer: *mut hrtimer) -> c_int;
    fn hrtimer_cancel(timer: *mut hrtimer) -> c_int;
    fn hrtimer_forward_now(timer: *mut hrtimer, interval: ktime_t) -> u64;
    fn hrtimer_setup(timer: *mut hrtimer, callback: Option<unsafe extern "C" fn(*mut hrtimer) -> hrtimer_restart>, clock_id: c_int, mode: c_int);
    fn atomic_read(v: *mut atomic_t) -> c_int;
    fn atomic_set(v: *mut atomic_t, i: c_int);
    fn ktime_us_delta(later: ktime_t, earlier: ktime_t) -> i64;
    fn ktime_set(secs: c_long, nsecs: c_ulong) -> ktime_t;
    fn div_u64_rem(dividend: u64, divisor: u64, remainder: *mut u32) -> u64;
    fn params_buffer_bytes(hw_params: *mut snd_pcm_hw_params) -> size_t;
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut c_void;
    fn free_page(addr: c_ulong);
    fn get_zeroed_page(flags: c_uint) -> c_ulong;
    fn virt_to_page(addr: *mut c_void) -> *mut page;
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback_count: c_int, capture_count: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops);
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, type_: c_int, data: *mut c_void, size: size_t, max: size_t);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn snd_ctl_boolean_stereo_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn snd_ctl_enum_info(info: *mut snd_ctl_elem_info, channels: c_uint, items: c_uint, names: *const *const c_char) -> c_int;
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *mut snd_ctl_elem_id);
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn pcm_format_to_bits(format: snd_pcm_format_t) -> u64;
    fn snd_pcm_format_name(format: snd_pcm_format_t) -> *const c_char;
    fn snd_info_get_line(buffer: *mut snd_info_buffer, line: *mut c_char, len: usize) -> c_int;
    fn snd_info_get_str(dest: *mut c_char, src: *const c_char, len: usize) -> *const c_char;
    fn kstrtoull(s: *const c_char, base: c_uint, res: *mut c_ulonglong) -> c_int;
    fn snd_card_rw_proc_new(card: *mut snd_card, name: *const c_char, private_data: *mut c_void, read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>, write: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn snd_devm_card_new(parent: *mut device, idx: c_int, xid: *mut c_char, module: *mut c_void, extra_size: usize, card_ret: *mut *mut snd_card) -> c_int;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn platform_device_unregister(pdev: *mut platform_device);
    fn platform_driver_unregister(driver: *mut platform_driver);
    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
    fn platform_device_register_simple(name: *const c_char, id: c_int, res: *mut c_void, num: c_uint) -> *mut platform_device;
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
}

unsafe fn kzalloc_obj<T>() -> *mut T {
    kzalloc(size_of::<T>(), GFP_KERNEL) as *mut T
}

extern "C" {
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
}

fn DIV_ROUND_UP(n: c_uint, d: c_uint) -> c_uint {
    (n + d - 1) / d
}

fn div_u64(n: u64, d: u64) -> u64 {
    n / d
}

const fn TLV_DB_SCALE(min: c_int, step: c_int, mute: c_int) -> [c_uint; 4] {
    [min as c_uint, step as c_uint, mute as c_uint, 0]
}

unsafe fn container_of_timer(t: *mut timer_list) -> *mut dummy_systimer_pcm {
    (t as *mut u8).sub(offset_of!(dummy_systimer_pcm, timer)) as *mut dummy_systimer_pcm
}

unsafe fn container_of_hrtimer(t: *mut hrtimer) -> *mut dummy_hrtimer_pcm {
    (t as *mut u8).sub(offset_of!(dummy_hrtimer_pcm, timer)) as *mut dummy_hrtimer_pcm
}

fn pcm_format_valid(_i: snd_pcm_format_t) -> bool {
    true
}

fn IS_ERR(_ptr: *const c_void) -> bool {
    false
}

const SNDRV_CARDS: usize = 8;
const SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS];
const SNDRV_PCM_FMTBIT_U8: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 1;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 10;
const SNDRV_PCM_RATE_CONTINUOUS: c_uint = 1 << 30;
const SNDRV_PCM_RATE_8000_48000: c_uint = 0x0000_07ff;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 7;
const SNDRV_PCM_RATE_96000: c_uint = 1 << 10;
const SNDRV_PCM_RATE_192000: c_uint = 1 << 12;
const SNDRV_PCM_RATE_KNOT: c_uint = 1 << 31;
const SNDRV_PCM_HW_PARAM_PERIODS: c_int = 0;
const SNDRV_PCM_HW_PARAM_BUFFER_BYTES: c_int = 1;
const UINT_MAX: c_uint = c_uint::MAX;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const HZ: c_uint = 100;
const HRTIMER_NORESTART: hrtimer_restart = 0;
const HRTIMER_RESTART: hrtimer_restart = 1;
const HRTIMER_MODE_REL_SOFT: c_int = 0;
const CLOCK_MONOTONIC: c_int = 1;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 1;
const SNDRV_PCM_TRIGGER_STOP: c_int = 2;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 3;
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 0;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 1;
const SNDRV_PCM_INFO_RESUME: c_uint = 1 << 2;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 3;
const SNDRV_PCM_INFO_NONINTERLEAVED: c_uint = 1 << 4;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const GFP_KERNEL: c_uint = 0;
const SNDRV_DMA_TYPE_CONTINUOUS: c_int = 0;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 3;
const SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint = 1 << 2;
const SNDRV_CTL_ELEM_ACCESS_INACTIVE: c_uint = 1 << 8;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 1;
const SNDRV_CTL_EVENT_MASK_INFO: c_uint = 1;
const SNDRV_CTL_POWER_D3hot: c_int = 3;
const SNDRV_CTL_POWER_D0: c_int = 0;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
