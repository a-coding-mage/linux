// SPDX-License-Identifier: GPL-2.0-only
/*
 * ff-pcm.c - a part of driver for RME Fireface series
 *
 * Copyright (c) 2015-2017 Takashi Sakamoto
 */

// Translated from C implementation source. Dependencies originally supplied by
// "ff.h" and kernel/ALSA headers are declared here as external items.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const UINT_MAX: c_uint = c_uint::MAX;
const EIO: c_int = 5;
const EINVAL: c_int = 22;

type snd_pcm_uframes_t = c_uint;

const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 1;
const SNDRV_PCM_HW_PARAM_PERIOD_SIZE: c_int = 2;
const SNDRV_PCM_HW_PARAM_BUFFER_SIZE: c_int = 3;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_STATE_OPEN: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_FMTBIT_S32: c_uint = 1 << 10;
const SNDRV_DMA_TYPE_VMALLOC: c_int = 0;
const SND_FF_CLOCK_SRC_INTERNAL: snd_ff_clock_src = 0;
const CIP_SFC_COUNT: usize = 8;

type snd_ff_stream_mode = c_int;
type snd_ff_clock_src = c_int;

#[repr(C)]
pub struct snd_interval {
    pub min: c_uint,
    pub max: c_uint,
    pub openmin: c_uint,
    pub openmax: c_uint,
    pub integer: c_uint,
    pub empty: c_uint,
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
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw: snd_pcm_hardware,
    pub state: c_int,
    pub rate: c_uint,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_rule {
    pub private: *mut c_void,
}

#[repr(C)]
pub struct amdtp_stream {
    pub sfc: c_uint,
}

#[repr(C)]
pub struct amdtp_domain {
    pub events_per_period: c_uint,
    pub events_per_buffer: c_uint,
}

#[repr(C)]
pub struct snd_card {
    pub driver: *const c_char,
    pub shortname: *const c_char,
}

#[repr(C)]
pub struct snd_pcm {
    pub private_data: *mut c_void,
    pub nonatomic: bool,
    pub name: [c_char; 80],
}

#[repr(C)]
pub struct snd_ff_protocol {
    pub get_clock:
        Option<unsafe extern "C" fn(*mut snd_ff, *mut c_uint, *mut snd_ff_clock_src) -> c_int>,
}

#[repr(C)]
pub struct snd_ff_spec {
    pub pcm_capture_channels: *const c_uint,
    pub pcm_playback_channels: *const c_uint,
    pub protocol: *const snd_ff_protocol,
}

#[repr(C)]
pub struct snd_ff {
    pub tx_stream: amdtp_stream,
    pub rx_stream: amdtp_stream,
    pub domain: amdtp_domain,
    pub spec: *const snd_ff_spec,
    pub mutex: c_void,
    pub substreams_counter: c_uint,
    pub card: *mut snd_card,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: c_int,
    pub private_data: *mut c_void,
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
    pub ack: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
}

unsafe extern "C" {
    static amdtp_rate_table: [c_uint; CIP_SFC_COUNT];

    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn hw_param_interval_c(params: *mut snd_pcm_hw_params, var: c_int) -> *const snd_interval;
    fn snd_interval_test(i: *const snd_interval, val: c_uint) -> bool;
    fn snd_interval_refine(i: *mut snd_interval, v: *const snd_interval) -> c_int;
    fn snd_ff_stream_get_multiplier_mode(rate_index: c_uint, mode: *mut snd_ff_stream_mode)
        -> c_int;
    fn snd_pcm_rate_to_rate_bit(rate: c_uint) -> c_uint;
    fn snd_pcm_hw_rule_add(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_int,
        func: unsafe extern "C" fn(*mut snd_pcm_hw_params, *mut snd_pcm_hw_rule) -> c_int,
        private: *mut c_void,
        dep: c_int,
        sentinel: c_int,
    ) -> c_int;
    fn amdtp_ff_add_pcm_hw_constraints(
        s: *mut amdtp_stream,
        runtime: *mut snd_pcm_runtime,
    ) -> c_int;
    fn snd_ff_stream_lock_try(ff: *mut snd_ff) -> c_int;
    fn snd_ff_stream_lock_release(ff: *mut snd_ff);
    fn snd_pcm_hw_constraint_minmax(
        runtime: *mut snd_pcm_runtime,
        var: c_int,
        min: c_uint,
        max: c_uint,
    ) -> c_int;
    fn snd_pcm_set_sync(substream: *mut snd_pcm_substream);
    fn snd_ff_stream_reserve_duplex(
        ff: *mut snd_ff,
        rate: c_uint,
        frames_per_period: c_uint,
        frames_per_buffer: c_uint,
    ) -> c_int;
    fn snd_ff_stream_stop_duplex(ff: *mut snd_ff);
    fn snd_ff_stream_start_duplex(ff: *mut snd_ff, rate: c_uint) -> c_int;
    fn amdtp_stream_pcm_prepare(s: *mut amdtp_stream);
    fn amdtp_stream_pcm_trigger(s: *mut amdtp_stream, substream: *mut snd_pcm_substream);
    fn amdtp_domain_stream_pcm_pointer(
        d: *mut amdtp_domain,
        s: *mut amdtp_stream,
    ) -> snd_pcm_uframes_t;
    fn amdtp_domain_stream_pcm_ack(d: *mut amdtp_domain, s: *mut amdtp_stream) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_period_size(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_buffer_size(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_pcm_new(
        card: *mut snd_card,
        id: *const c_char,
        device: c_int,
        playback_count: c_int,
        capture_count: c_int,
        rpcm: *mut *mut snd_pcm,
    ) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        type_: c_int,
        data: *mut c_void,
        size: usize,
        max: usize,
    );
}

#[inline]
unsafe fn pcm_channels_at(pcm_channels: *const c_uint, mode: snd_ff_stream_mode) -> c_uint {
    *pcm_channels.add(mode as usize)
}

unsafe extern "C" fn hw_rule_rate(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let pcm_channels = (*rule).private as *const c_uint;
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

    for i in 0..CIP_SFC_COUNT {
        let mut mode: snd_ff_stream_mode = 0;
        let err = snd_ff_stream_get_multiplier_mode(i as c_uint, &mut mode);
        if err < 0 {
            continue;
        }

        if !snd_interval_test(c, pcm_channels_at(pcm_channels, mode)) {
            continue;
        }

        t.min = t.min.min(amdtp_rate_table[i]);
        t.max = t.max.max(amdtp_rate_table[i]);
    }

    snd_interval_refine(r, &t)
}

unsafe extern "C" fn hw_rule_channels(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let pcm_channels = (*rule).private as *const c_uint;
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

    for i in 0..CIP_SFC_COUNT {
        let mut mode: snd_ff_stream_mode = 0;
        let err = snd_ff_stream_get_multiplier_mode(i as c_uint, &mut mode);
        if err < 0 {
            continue;
        }

        if !snd_interval_test(r, amdtp_rate_table[i]) {
            continue;
        }

        t.min = t.min.min(pcm_channels_at(pcm_channels, mode));
        t.max = t.max.max(pcm_channels_at(pcm_channels, mode));
    }

    snd_interval_refine(c, &t)
}

unsafe fn limit_channels_and_rates(hw: *mut snd_pcm_hardware, pcm_channels: *const c_uint) {
    (*hw).channels_min = UINT_MAX;
    (*hw).channels_max = 0;
    (*hw).rate_min = UINT_MAX;
    (*hw).rate_max = 0;

    for i in 0..CIP_SFC_COUNT {
        let mut mode: snd_ff_stream_mode = 0;
        let err = snd_ff_stream_get_multiplier_mode(i as c_uint, &mut mode);
        if err < 0 {
            continue;
        }

        let channels = pcm_channels_at(pcm_channels, mode);
        if pcm_channels_at(pcm_channels, mode) == 0 {
            continue;
        }
        (*hw).channels_min = (*hw).channels_min.min(channels);
        (*hw).channels_max = (*hw).channels_max.max(channels);

        let rate = amdtp_rate_table[i];
        (*hw).rates |= snd_pcm_rate_to_rate_bit(rate);
        (*hw).rate_min = (*hw).rate_min.min(rate);
        (*hw).rate_max = (*hw).rate_max.max(rate);
    }
}

unsafe fn pcm_init_hw_params(
    ff: *mut snd_ff,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let runtime = (*substream).runtime;
    let s: *mut amdtp_stream;
    let pcm_channels: *const c_uint;

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        (*runtime).hw.formats = SNDRV_PCM_FMTBIT_S32;
        s = &mut (*ff).tx_stream;
        pcm_channels = (*(*ff).spec).pcm_capture_channels;
    } else {
        (*runtime).hw.formats = SNDRV_PCM_FMTBIT_S32;
        s = &mut (*ff).rx_stream;
        pcm_channels = (*(*ff).spec).pcm_playback_channels;
    }

    limit_channels_and_rates(&mut (*runtime).hw, pcm_channels);

    let mut err = snd_pcm_hw_rule_add(
        runtime,
        0,
        SNDRV_PCM_HW_PARAM_CHANNELS,
        hw_rule_channels,
        pcm_channels as *mut c_void,
        SNDRV_PCM_HW_PARAM_RATE,
        -1,
    );
    if err < 0 {
        return err;
    }

    err = snd_pcm_hw_rule_add(
        runtime,
        0,
        SNDRV_PCM_HW_PARAM_RATE,
        hw_rule_rate,
        pcm_channels as *mut c_void,
        SNDRV_PCM_HW_PARAM_CHANNELS,
        -1,
    );
    if err < 0 {
        return err;
    }

    amdtp_ff_add_pcm_hw_constraints(s, runtime)
}

unsafe extern "C" fn pcm_open(substream: *mut snd_pcm_substream) -> c_int {
    let ff = (*substream).private_data as *mut snd_ff;
    let d = &mut (*ff).domain as *mut amdtp_domain;
    let mut rate: c_uint = 0;
    let mut src: snd_ff_clock_src = 0;

    let mut err = snd_ff_stream_lock_try(ff);
    if err < 0 {
        return err;
    }

    err = pcm_init_hw_params(ff, substream);
    if err < 0 {
        snd_ff_stream_lock_release(ff);
        return err;
    }

    err = ((*(*(*ff).spec).protocol).get_clock.unwrap())(ff, &mut rate, &mut src);
    if err < 0 {
        snd_ff_stream_lock_release(ff);
        return err;
    }

    // Original C used scoped_guard(mutex, &ff->mutex) for this block.
    {
        // When source of clock is not internal or any stream is reserved for
        // transmission of PCM frames, the available sampling rate is limited
        // at current one.
        if src != SND_FF_CLOCK_SRC_INTERNAL {
            let mut i = 0usize;
            while i < CIP_SFC_COUNT {
                if amdtp_rate_table[i] == rate {
                    break;
                }
                i += 1;
            }

            // The unit is configured at sampling frequency which packet
            // streaming engine can't support.
            if i >= CIP_SFC_COUNT {
                err = -EIO;
                snd_ff_stream_lock_release(ff);
                return err;
            }

            (*(*substream).runtime).hw.rate_min = rate;
            (*(*substream).runtime).hw.rate_max = rate;
        } else if (*ff).substreams_counter > 0 {
            let frames_per_period = (*d).events_per_period;
            let frames_per_buffer = (*d).events_per_buffer;

            rate = amdtp_rate_table[(*ff).rx_stream.sfc as usize];
            (*(*substream).runtime).hw.rate_min = rate;
            (*(*substream).runtime).hw.rate_max = rate;

            err = snd_pcm_hw_constraint_minmax(
                (*substream).runtime,
                SNDRV_PCM_HW_PARAM_PERIOD_SIZE,
                frames_per_period,
                frames_per_period,
            );
            if err < 0 {
                snd_ff_stream_lock_release(ff);
                return err;
            }

            err = snd_pcm_hw_constraint_minmax(
                (*substream).runtime,
                SNDRV_PCM_HW_PARAM_BUFFER_SIZE,
                frames_per_buffer,
                frames_per_buffer,
            );
            if err < 0 {
                snd_ff_stream_lock_release(ff);
                return err;
            }
        }
    }

    snd_pcm_set_sync(substream);

    0
}

unsafe extern "C" fn pcm_close(substream: *mut snd_pcm_substream) -> c_int {
    let ff = (*substream).private_data as *mut snd_ff;

    snd_ff_stream_lock_release(ff);

    0
}

unsafe extern "C" fn pcm_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let ff = (*substream).private_data as *mut snd_ff;
    let mut err = 0;

    if (*(*substream).runtime).state == SNDRV_PCM_STATE_OPEN {
        let rate = params_rate(hw_params);
        let frames_per_period = params_period_size(hw_params);
        let frames_per_buffer = params_buffer_size(hw_params);

        // Original C used guard(mutex)(&ff->mutex).
        err = snd_ff_stream_reserve_duplex(ff, rate, frames_per_period, frames_per_buffer);
        if err >= 0 {
            (*ff).substreams_counter += 1;
        }
    }

    err
}

unsafe extern "C" fn pcm_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let ff = (*substream).private_data as *mut snd_ff;

    // Original C used guard(mutex)(&ff->mutex).

    if (*(*substream).runtime).state != SNDRV_PCM_STATE_OPEN {
        (*ff).substreams_counter -= 1;
    }

    snd_ff_stream_stop_duplex(ff);

    0
}

unsafe extern "C" fn pcm_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let ff = (*substream).private_data as *mut snd_ff;
    let runtime = (*substream).runtime;

    // Original C used guard(mutex)(&ff->mutex).

    let err = snd_ff_stream_start_duplex(ff, (*runtime).rate);
    if err >= 0 {
        amdtp_stream_pcm_prepare(&mut (*ff).tx_stream);
    }

    err
}

unsafe extern "C" fn pcm_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let ff = (*substream).private_data as *mut snd_ff;
    let runtime = (*substream).runtime;

    // Original C used guard(mutex)(&ff->mutex).

    let err = snd_ff_stream_start_duplex(ff, (*runtime).rate);
    if err >= 0 {
        amdtp_stream_pcm_prepare(&mut (*ff).rx_stream);
    }

    err
}

unsafe extern "C" fn pcm_capture_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let ff = (*substream).private_data as *mut snd_ff;

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            amdtp_stream_pcm_trigger(&mut (*ff).tx_stream, substream);
        }
        SNDRV_PCM_TRIGGER_STOP => {
            amdtp_stream_pcm_trigger(&mut (*ff).tx_stream, ptr::null_mut());
        }
        _ => {
            return -EINVAL;
        }
    }

    0
}

unsafe extern "C" fn pcm_playback_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let ff = (*substream).private_data as *mut snd_ff;

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            amdtp_stream_pcm_trigger(&mut (*ff).rx_stream, substream);
        }
        SNDRV_PCM_TRIGGER_STOP => {
            amdtp_stream_pcm_trigger(&mut (*ff).rx_stream, ptr::null_mut());
        }
        _ => {
            return -EINVAL;
        }
    }

    0
}

unsafe extern "C" fn pcm_capture_pointer(
    sbstrm: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let ff = (*sbstrm).private_data as *mut snd_ff;

    amdtp_domain_stream_pcm_pointer(&mut (*ff).domain, &mut (*ff).tx_stream)
}

unsafe extern "C" fn pcm_playback_pointer(
    sbstrm: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let ff = (*sbstrm).private_data as *mut snd_ff;

    amdtp_domain_stream_pcm_pointer(&mut (*ff).domain, &mut (*ff).rx_stream)
}

unsafe extern "C" fn pcm_capture_ack(substream: *mut snd_pcm_substream) -> c_int {
    let ff = (*substream).private_data as *mut snd_ff;

    amdtp_domain_stream_pcm_ack(&mut (*ff).domain, &mut (*ff).tx_stream)
}

unsafe extern "C" fn pcm_playback_ack(substream: *mut snd_pcm_substream) -> c_int {
    let ff = (*substream).private_data as *mut snd_ff;

    amdtp_domain_stream_pcm_ack(&mut (*ff).domain, &mut (*ff).rx_stream)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ff_create_pcm_devices(ff: *mut snd_ff) -> c_int {
    static PCM_CAPTURE_OPS: snd_pcm_ops = snd_pcm_ops {
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
    static PCM_PLAYBACK_OPS: snd_pcm_ops = snd_pcm_ops {
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

    let err = snd_pcm_new((*ff).card, (*(*ff).card).driver, 0, 1, 1, &mut pcm);
    if err < 0 {
        return err;
    }

    (*pcm).private_data = ff as *mut c_void;
    (*pcm).nonatomic = true;
    snprintf(
        (*pcm).name.as_mut_ptr(),
        (*pcm).name.len(),
        c"%s PCM".as_ptr(),
        (*(*ff).card).shortname,
    );
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &PCM_PLAYBACK_OPS);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &PCM_CAPTURE_OPS);
    snd_pcm_set_managed_buffer_all(
        pcm,
        SNDRV_DMA_TYPE_VMALLOC,
        ptr::null_mut(),
        0,
        0,
    );

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
