// SPDX-License-Identifier: GPL-2.0-only
/*
 * digi00x-pcm.c - a part of driver for Digidesign Digi 002/003 family
 *
 * Copyright (c) 2014-2015 Takashi Sakamoto
 */

// Translated from C source. Dependency declarations correspond to symbols
// provided by digi00x.h and the surrounding ALSA/firewire kernel code.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of_val;
use core::ptr;

const UINT_MAX: c_uint = c_uint::MAX;

extern "C" {
    static snd_dg00x_stream_pcm_channels: [c_uint; SND_DG00X_RATE_COUNT as usize];
    static snd_dg00x_stream_rates: [c_uint; SND_DG00X_RATE_COUNT as usize];

    fn hw_param_interval(
        params: *mut snd_pcm_hw_params,
        var: c_int,
    ) -> *mut snd_interval;
    fn hw_param_interval_c(
        params: *mut snd_pcm_hw_params,
        var: c_int,
    ) -> *const snd_interval;
    fn snd_interval_test(i: *const snd_interval, val: c_uint) -> bool;
    fn snd_interval_refine(i: *mut snd_interval, v: *const snd_interval) -> c_int;

    fn snd_pcm_limit_hw_rates(runtime: *mut snd_pcm_runtime) -> c_int;
    fn snd_pcm_hw_rule_add(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_int,
        func: Option<
            unsafe extern "C" fn(
                params: *mut snd_pcm_hw_params,
                rule: *mut snd_pcm_hw_rule,
            ) -> c_int,
        >,
        private: *mut c_void,
        dep: c_int,
        last: c_int,
    ) -> c_int;
    fn amdtp_dot_add_pcm_hw_constraints(
        s: *mut amdtp_stream,
        runtime: *mut snd_pcm_runtime,
    ) -> c_int;

    fn snd_dg00x_stream_lock_try(dg00x: *mut snd_dg00x) -> c_int;
    fn snd_dg00x_stream_lock_release(dg00x: *mut snd_dg00x);
    fn snd_dg00x_stream_get_clock(
        dg00x: *mut snd_dg00x,
        clock: *mut snd_dg00x_clock,
    ) -> c_int;
    fn snd_dg00x_stream_check_external_clock(
        dg00x: *mut snd_dg00x,
        detect: *mut bool,
    ) -> c_int;
    fn snd_dg00x_stream_get_external_rate(
        dg00x: *mut snd_dg00x,
        rate: *mut c_uint,
    ) -> c_int;
    fn snd_pcm_hw_constraint_minmax(
        runtime: *mut snd_pcm_runtime,
        var: c_int,
        min: c_uint,
        max: c_uint,
    ) -> c_int;
    fn snd_pcm_set_sync(substream: *mut snd_pcm_substream);
    fn snd_dg00x_stream_reserve_duplex(
        dg00x: *mut snd_dg00x,
        rate: c_uint,
        frames_per_period: c_uint,
        frames_per_buffer: c_uint,
    ) -> c_int;
    fn snd_dg00x_stream_stop_duplex(dg00x: *mut snd_dg00x);
    fn snd_dg00x_stream_start_duplex(dg00x: *mut snd_dg00x) -> c_int;

    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_period_size(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_buffer_size(params: *mut snd_pcm_hw_params) -> c_uint;

    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);

    fn amdtp_stream_pcm_prepare(s: *mut amdtp_stream);
    fn amdtp_dot_reset(s: *mut amdtp_stream);
    fn amdtp_stream_pcm_trigger(
        s: *mut amdtp_stream,
        substream: *mut snd_pcm_substream,
    );
    fn amdtp_domain_stream_pcm_pointer(
        d: *mut amdtp_domain,
        s: *mut amdtp_stream,
    ) -> snd_pcm_uframes_t;
    fn amdtp_domain_stream_pcm_ack(
        d: *mut amdtp_domain,
        s: *mut amdtp_stream,
    ) -> c_int;

    fn snd_pcm_new(
        card: *mut snd_card,
        id: *const c_char,
        device: c_int,
        playback_count: c_int,
        capture_count: c_int,
        rpcm: *mut *mut snd_pcm,
    ) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        type_: c_int,
        data: *mut c_void,
        size: usize,
        max: usize,
    );
}

type snd_pcm_uframes_t = c_uint;
type snd_dg00x_clock = c_int;

const SND_DG00X_RATE_COUNT: c_int = 4;
const SND_DG00X_CLOCK_INTERNAL: snd_dg00x_clock = 0;

const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 1;
const SNDRV_PCM_HW_PARAM_PERIOD_SIZE: c_int = 2;
const SNDRV_PCM_HW_PARAM_BUFFER_SIZE: c_int = 3;

const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;

const SNDRV_PCM_STATE_OPEN: c_int = 0;

const SNDRV_PCM_FMTBIT_S32: c_uint = 1 << 10;
const SNDRV_PCM_RATE_44100: c_uint = 1 << 6;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 7;
const SNDRV_PCM_RATE_88200: c_uint = 1 << 8;
const SNDRV_PCM_RATE_96000: c_uint = 1 << 9;

const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;

const SNDRV_DMA_TYPE_VMALLOC: c_int = 0;

const EBUSY: c_int = 16;
const EINVAL: c_int = 22;

#[repr(C)]
struct snd_interval {
    min: c_uint,
    max: c_uint,
    openmin: c_uint,
    openmax: c_uint,
    integer: c_uint,
    empty: c_uint,
}

#[repr(C)]
struct snd_pcm_hardware {
    info: c_uint,
    formats: c_uint,
    rates: c_uint,
    rate_min: c_uint,
    rate_max: c_uint,
    channels_min: c_uint,
    channels_max: c_uint,
}

#[repr(C)]
struct snd_pcm_runtime {
    hw: snd_pcm_hardware,
    state: c_int,
}

#[repr(C)]
struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
    private_data: *mut snd_dg00x,
    stream: c_int,
}

#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_hw_rule {
    _private: [u8; 0],
}

#[repr(C)]
struct amdtp_stream {
    _private: [u8; 0],
}

#[repr(C)]
struct amdtp_domain {
    events_per_period: c_uint,
    events_per_buffer: c_uint,
}

#[repr(C)]
struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_card {
    driver: *const c_char,
    shortname: *const c_char,
}

#[repr(C)]
struct snd_pcm {
    private_data: *mut c_void,
    nonatomic: bool,
    name: [c_char; 80],
}

#[repr(C)]
struct snd_dg00x {
    domain: amdtp_domain,
    tx_stream: amdtp_stream,
    rx_stream: amdtp_stream,
    mutex: mutex,
    substreams_counter: c_uint,
    card: *mut snd_card,
}

#[repr(C)]
struct snd_pcm_ops {
    open: Option<unsafe extern "C" fn(substream: *mut snd_pcm_substream) -> c_int>,
    close: Option<unsafe extern "C" fn(substream: *mut snd_pcm_substream) -> c_int>,
    ioctl: Option<unsafe extern "C" fn() -> c_int>,
    hw_params: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            hw_params: *mut snd_pcm_hw_params,
        ) -> c_int,
    >,
    hw_free: Option<unsafe extern "C" fn(substream: *mut snd_pcm_substream) -> c_int>,
    prepare: Option<unsafe extern "C" fn(substream: *mut snd_pcm_substream) -> c_int>,
    trigger: Option<
        unsafe extern "C" fn(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int,
    >,
    pointer: Option<
        unsafe extern "C" fn(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t,
    >,
    ack: Option<unsafe extern "C" fn(substream: *mut snd_pcm_substream) -> c_int>,
}

unsafe extern "C" fn hw_rule_rate(
    params: *mut snd_pcm_hw_params,
    _rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let r = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    let c = hw_param_interval_c(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let mut t = snd_interval {
        min: UINT_MAX,
        max: 0,
        openmin: 0,
        openmax: 0,
        integer: 1,
        empty: 0,
    };
    let mut i: c_uint = 0;

    while i < SND_DG00X_RATE_COUNT as c_uint {
        if !snd_interval_test(c, snd_dg00x_stream_pcm_channels[i as usize]) {
            i += 1;
            continue;
        }

        t.min = t.min.min(snd_dg00x_stream_rates[i as usize]);
        t.max = t.max.max(snd_dg00x_stream_rates[i as usize]);
        i += 1;
    }

    snd_interval_refine(r, &t)
}

unsafe extern "C" fn hw_rule_channels(
    params: *mut snd_pcm_hw_params,
    _rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let c = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let r = hw_param_interval_c(params, SNDRV_PCM_HW_PARAM_RATE);
    let mut t = snd_interval {
        min: UINT_MAX,
        max: 0,
        openmin: 0,
        openmax: 0,
        integer: 1,
        empty: 0,
    };
    let mut i: c_uint = 0;

    while i < SND_DG00X_RATE_COUNT as c_uint {
        if !snd_interval_test(r, snd_dg00x_stream_rates[i as usize]) {
            i += 1;
            continue;
        }

        t.min = t.min.min(snd_dg00x_stream_pcm_channels[i as usize]);
        t.max = t.max.max(snd_dg00x_stream_pcm_channels[i as usize]);
        i += 1;
    }

    snd_interval_refine(c, &t)
}

unsafe extern "C" fn pcm_init_hw_params(
    dg00x: *mut snd_dg00x,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let runtime = (*substream).runtime;
    let hw = &mut (*runtime).hw as *mut snd_pcm_hardware;
    let s: *mut amdtp_stream;
    let mut err: c_int;

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        (*(*substream).runtime).hw.formats = SNDRV_PCM_FMTBIT_S32;
        s = &mut (*dg00x).tx_stream;
    } else {
        (*(*substream).runtime).hw.formats = SNDRV_PCM_FMTBIT_S32;
        s = &mut (*dg00x).rx_stream;
    }

    (*hw).channels_min = 10;
    (*hw).channels_max = 18;

    (*hw).rates = SNDRV_PCM_RATE_44100
        | SNDRV_PCM_RATE_48000
        | SNDRV_PCM_RATE_88200
        | SNDRV_PCM_RATE_96000;
    snd_pcm_limit_hw_rates(runtime);

    err = snd_pcm_hw_rule_add(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_CHANNELS,
        Some(hw_rule_channels),
        ptr::null_mut(),
        SNDRV_PCM_HW_PARAM_RATE,
        -1,
    );
    if err < 0 {
        return err;
    }

    err = snd_pcm_hw_rule_add(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_RATE,
        Some(hw_rule_rate),
        ptr::null_mut(),
        SNDRV_PCM_HW_PARAM_CHANNELS,
        -1,
    );
    if err < 0 {
        return err;
    }

    amdtp_dot_add_pcm_hw_constraints(s, (*substream).runtime)
}

unsafe extern "C" fn pcm_open(substream: *mut snd_pcm_substream) -> c_int {
    let dg00x = (*substream).private_data;
    let d = &mut (*dg00x).domain as *mut amdtp_domain;
    let mut clock: snd_dg00x_clock = 0;
    let mut detect: bool = false;
    let mut err: c_int;

    err = snd_dg00x_stream_lock_try(dg00x);
    if err < 0 {
        return err;
    }

    err = pcm_init_hw_params(dg00x, substream);
    if err < 0 {
        goto_err_locked(dg00x, err)
    } else {
        err = snd_dg00x_stream_get_clock(dg00x, &mut clock);
        if err < 0 {
            goto_err_locked(dg00x, err)
        } else {
            if clock != SND_DG00X_CLOCK_INTERNAL {
                err = snd_dg00x_stream_check_external_clock(dg00x, &mut detect);
                if err < 0 {
                    return goto_err_locked(dg00x, err);
                }
                if !detect {
                    err = -EBUSY;
                    return goto_err_locked(dg00x, err);
                }
            }

            mutex_lock(&mut (*dg00x).mutex);
            // When source of clock is not internal or any stream is reserved for
            // transmission of PCM frames, the available sampling rate is limited
            // at current one.
            if (clock != SND_DG00X_CLOCK_INTERNAL)
                || ((*dg00x).substreams_counter > 0 && (*d).events_per_period > 0)
            {
                let frames_per_period = (*d).events_per_period;
                let frames_per_buffer = (*d).events_per_buffer;
                let mut rate: c_uint = 0;

                err = snd_dg00x_stream_get_external_rate(dg00x, &mut rate);
                if err < 0 {
                    mutex_unlock(&mut (*dg00x).mutex);
                    return goto_err_locked(dg00x, err);
                }
                (*(*substream).runtime).hw.rate_min = rate;
                (*(*substream).runtime).hw.rate_max = rate;

                if frames_per_period > 0 {
                    err = snd_pcm_hw_constraint_minmax(
                        (*substream).runtime,
                        SNDRV_PCM_HW_PARAM_PERIOD_SIZE,
                        frames_per_period,
                        frames_per_period,
                    );
                    if err < 0 {
                        mutex_unlock(&mut (*dg00x).mutex);
                        return goto_err_locked(dg00x, err);
                    }

                    err = snd_pcm_hw_constraint_minmax(
                        (*substream).runtime,
                        SNDRV_PCM_HW_PARAM_BUFFER_SIZE,
                        frames_per_buffer,
                        frames_per_buffer,
                    );
                    if err < 0 {
                        mutex_unlock(&mut (*dg00x).mutex);
                        return goto_err_locked(dg00x, err);
                    }
                }
            }
            mutex_unlock(&mut (*dg00x).mutex);

            snd_pcm_set_sync(substream);

            0
        }
    }
}

unsafe fn goto_err_locked(dg00x: *mut snd_dg00x, err: c_int) -> c_int {
    snd_dg00x_stream_lock_release(dg00x);
    err
}

unsafe extern "C" fn pcm_close(substream: *mut snd_pcm_substream) -> c_int {
    let dg00x = (*substream).private_data;

    snd_dg00x_stream_lock_release(dg00x);

    0
}

unsafe extern "C" fn pcm_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let dg00x = (*substream).private_data;
    let mut err: c_int = 0;

    if (*(*substream).runtime).state == SNDRV_PCM_STATE_OPEN {
        let rate = params_rate(hw_params);
        let frames_per_period = params_period_size(hw_params);
        let frames_per_buffer = params_buffer_size(hw_params);

        mutex_lock(&mut (*dg00x).mutex);
        err = snd_dg00x_stream_reserve_duplex(
            dg00x,
            rate,
            frames_per_period,
            frames_per_buffer,
        );
        if err >= 0 {
            (*dg00x).substreams_counter += 1;
        }
        mutex_unlock(&mut (*dg00x).mutex);
    }

    err
}

unsafe extern "C" fn pcm_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let dg00x = (*substream).private_data;

    mutex_lock(&mut (*dg00x).mutex);

    if (*(*substream).runtime).state != SNDRV_PCM_STATE_OPEN {
        (*dg00x).substreams_counter -= 1;
    }

    snd_dg00x_stream_stop_duplex(dg00x);

    mutex_unlock(&mut (*dg00x).mutex);

    0
}

unsafe extern "C" fn pcm_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let dg00x = (*substream).private_data;
    let mut err: c_int;

    mutex_lock(&mut (*dg00x).mutex);

    err = snd_dg00x_stream_start_duplex(dg00x);
    if err >= 0 {
        amdtp_stream_pcm_prepare(&mut (*dg00x).tx_stream);
    }

    mutex_unlock(&mut (*dg00x).mutex);

    err
}

unsafe extern "C" fn pcm_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let dg00x = (*substream).private_data;
    let mut err: c_int;

    mutex_lock(&mut (*dg00x).mutex);

    err = snd_dg00x_stream_start_duplex(dg00x);
    if err >= 0 {
        amdtp_stream_pcm_prepare(&mut (*dg00x).rx_stream);
        amdtp_dot_reset(&mut (*dg00x).rx_stream);
    }

    mutex_unlock(&mut (*dg00x).mutex);

    err
}

unsafe extern "C" fn pcm_capture_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let dg00x = (*substream).private_data;

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            amdtp_stream_pcm_trigger(&mut (*dg00x).tx_stream, substream);
        }
        SNDRV_PCM_TRIGGER_STOP => {
            amdtp_stream_pcm_trigger(&mut (*dg00x).tx_stream, ptr::null_mut());
        }
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn pcm_playback_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let dg00x = (*substream).private_data;

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            amdtp_stream_pcm_trigger(&mut (*dg00x).rx_stream, substream);
        }
        SNDRV_PCM_TRIGGER_STOP => {
            amdtp_stream_pcm_trigger(&mut (*dg00x).rx_stream, ptr::null_mut());
        }
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn pcm_capture_pointer(
    sbstrm: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let dg00x = (*sbstrm).private_data;

    amdtp_domain_stream_pcm_pointer(&mut (*dg00x).domain, &mut (*dg00x).tx_stream)
}

unsafe extern "C" fn pcm_playback_pointer(
    sbstrm: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let dg00x = (*sbstrm).private_data;

    amdtp_domain_stream_pcm_pointer(&mut (*dg00x).domain, &mut (*dg00x).rx_stream)
}

unsafe extern "C" fn pcm_capture_ack(substream: *mut snd_pcm_substream) -> c_int {
    let dg00x = (*substream).private_data;

    amdtp_domain_stream_pcm_ack(&mut (*dg00x).domain, &mut (*dg00x).tx_stream)
}

unsafe extern "C" fn pcm_playback_ack(substream: *mut snd_pcm_substream) -> c_int {
    let dg00x = (*substream).private_data;

    amdtp_domain_stream_pcm_ack(&mut (*dg00x).domain, &mut (*dg00x).rx_stream)
}

#[no_mangle]
pub unsafe extern "C" fn snd_dg00x_create_pcm_devices(dg00x: *mut snd_dg00x) -> c_int {
    static CAPTURE_OPS: snd_pcm_ops = snd_pcm_ops {
        open: Some(pcm_open),
        close: Some(pcm_close),
        ioctl: None,
        hw_params: Some(pcm_hw_params),
        hw_free: Some(pcm_hw_free),
        prepare: Some(pcm_capture_prepare),
        trigger: Some(pcm_capture_trigger),
        pointer: Some(pcm_capture_pointer),
        ack: Some(pcm_capture_ack),
    };
    static PLAYBACK_OPS: snd_pcm_ops = snd_pcm_ops {
        open: Some(pcm_open),
        close: Some(pcm_close),
        ioctl: None,
        hw_params: Some(pcm_hw_params),
        hw_free: Some(pcm_hw_free),
        prepare: Some(pcm_playback_prepare),
        trigger: Some(pcm_playback_trigger),
        pointer: Some(pcm_playback_pointer),
        ack: Some(pcm_playback_ack),
    };
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let mut err: c_int;

    err = snd_pcm_new((*dg00x).card, (*(*dg00x).card).driver, 0, 1, 1, &mut pcm);
    if err < 0 {
        return err;
    }

    (*pcm).private_data = dg00x as *mut c_void;
    (*pcm).nonatomic = true;
    snprintf(
        (*pcm).name.as_mut_ptr(),
        size_of_val(&(*pcm).name),
        b"%s PCM\0".as_ptr() as *const c_char,
        (*(*dg00x).card).shortname,
    );
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &PLAYBACK_OPS);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &CAPTURE_OPS);
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_VMALLOC, ptr::null_mut(), 0, 0);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
