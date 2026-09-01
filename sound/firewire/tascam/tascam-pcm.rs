// SPDX-License-Identifier: GPL-2.0-only
/*
 * tascam-pcm.c - a part of driver for TASCAM FireWire series
 *
 * Copyright (c) 2015 Takashi Sakamoto
 */

// C dependency intent: #include "tascam.h"
use crate::*;

unsafe fn pcm_init_hw_params(
    tscm: *mut snd_tscm,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let hw: *mut snd_pcm_hardware = &mut (*runtime).hw;
    let stream: *mut amdtp_stream;
    let mut pcm_channels: c_uint;

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        (*runtime).hw.formats = SNDRV_PCM_FMTBIT_S32;
        stream = &mut (*tscm).tx_stream;
        pcm_channels = (*(*tscm).spec).pcm_capture_analog_channels;
    } else {
        (*runtime).hw.formats = SNDRV_PCM_FMTBIT_S32;
        stream = &mut (*tscm).rx_stream;
        pcm_channels = (*(*tscm).spec).pcm_playback_analog_channels;
    }

    if (*(*tscm).spec).has_adat {
        pcm_channels += 8;
    }
    if (*(*tscm).spec).has_spdif {
        pcm_channels += 2;
    }
    (*runtime).hw.channels_max = pcm_channels;
    (*runtime).hw.channels_min = (*runtime).hw.channels_max;

    (*hw).rates = SNDRV_PCM_RATE_44100
        | SNDRV_PCM_RATE_48000
        | SNDRV_PCM_RATE_88200
        | SNDRV_PCM_RATE_96000;
    snd_pcm_limit_hw_rates(runtime);

    amdtp_tscm_add_pcm_hw_constraints(stream, runtime)
}

unsafe fn pcm_open(substream: *mut snd_pcm_substream) -> c_int {
    let tscm: *mut snd_tscm = (*substream).private_data as *mut snd_tscm;
    let d: *mut amdtp_domain = &mut (*tscm).domain;
    let mut clock: snd_tscm_clock = core::mem::zeroed();
    let mut err: c_int;

    err = snd_tscm_stream_lock_try(tscm);
    if err < 0 {
        return err;
    }

    err = pcm_init_hw_params(tscm, substream);
    if err < 0 {
        snd_tscm_stream_lock_release(tscm);
        return err;
    }

    err = snd_tscm_stream_get_clock(tscm, &mut clock);
    if err < 0 {
        snd_tscm_stream_lock_release(tscm);
        return err;
    }

    // C source used scoped_guard(mutex, &tscm->mutex) for this critical section.
    scoped_guard_mutex(&mut (*tscm).mutex, || {
        // When source of clock is not internal or any stream is reserved for
        // transmission of PCM frames, the available sampling rate is limited
        // at current one.
        if clock != SND_TSCM_CLOCK_INTERNAL || (*tscm).substreams_counter > 0 {
            let frames_per_period: c_uint = (*d).events_per_period;
            let frames_per_buffer: c_uint = (*d).events_per_buffer;
            let mut rate: c_uint = 0;

            err = snd_tscm_stream_get_rate(tscm, &mut rate);
            if err < 0 {
                return;
            }
            (*(*substream).runtime).hw.rate_min = rate;
            (*(*substream).runtime).hw.rate_max = rate;

            err = snd_pcm_hw_constraint_minmax(
                (*substream).runtime,
                SNDRV_PCM_HW_PARAM_PERIOD_SIZE,
                frames_per_period,
                frames_per_period,
            );
            if err < 0 {
                return;
            }

            err = snd_pcm_hw_constraint_minmax(
                (*substream).runtime,
                SNDRV_PCM_HW_PARAM_BUFFER_SIZE,
                frames_per_buffer,
                frames_per_buffer,
            );
        }
    });
    if err < 0 {
        snd_tscm_stream_lock_release(tscm);
        return err;
    }

    snd_pcm_set_sync(substream);

    0
}

unsafe fn pcm_close(substream: *mut snd_pcm_substream) -> c_int {
    let tscm: *mut snd_tscm = (*substream).private_data as *mut snd_tscm;

    snd_tscm_stream_lock_release(tscm);

    0
}

unsafe fn pcm_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let tscm: *mut snd_tscm = (*substream).private_data as *mut snd_tscm;
    let mut err: c_int = 0;

    if (*(*substream).runtime).state == SNDRV_PCM_STATE_OPEN {
        let rate: c_uint = params_rate(hw_params);
        let frames_per_period: c_uint = params_period_size(hw_params);
        let frames_per_buffer: c_uint = params_buffer_size(hw_params);

        // C source used guard(mutex)(&tscm->mutex).
        guard_mutex(&mut (*tscm).mutex);
        err = snd_tscm_stream_reserve_duplex(tscm, rate, frames_per_period, frames_per_buffer);
        if err >= 0 {
            (*tscm).substreams_counter += 1;
        }
    }

    err
}

unsafe fn pcm_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let tscm: *mut snd_tscm = (*substream).private_data as *mut snd_tscm;

    // C source used guard(mutex)(&tscm->mutex).
    guard_mutex(&mut (*tscm).mutex);

    if (*(*substream).runtime).state != SNDRV_PCM_STATE_OPEN {
        (*tscm).substreams_counter -= 1;
    }

    snd_tscm_stream_stop_duplex(tscm);

    0
}

unsafe fn pcm_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let tscm: *mut snd_tscm = (*substream).private_data as *mut snd_tscm;
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let err: c_int;

    // C source used guard(mutex)(&tscm->mutex).
    guard_mutex(&mut (*tscm).mutex);

    err = snd_tscm_stream_start_duplex(tscm, (*runtime).rate);
    if err >= 0 {
        amdtp_stream_pcm_prepare(&mut (*tscm).tx_stream);
    }

    err
}

unsafe fn pcm_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let tscm: *mut snd_tscm = (*substream).private_data as *mut snd_tscm;
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let err: c_int;

    // C source used guard(mutex)(&tscm->mutex).
    guard_mutex(&mut (*tscm).mutex);

    err = snd_tscm_stream_start_duplex(tscm, (*runtime).rate);
    if err >= 0 {
        amdtp_stream_pcm_prepare(&mut (*tscm).rx_stream);
    }

    err
}

unsafe fn pcm_capture_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let tscm: *mut snd_tscm = (*substream).private_data as *mut snd_tscm;

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            amdtp_stream_pcm_trigger(&mut (*tscm).tx_stream, substream);
        }
        SNDRV_PCM_TRIGGER_STOP => {
            amdtp_stream_pcm_trigger(&mut (*tscm).tx_stream, core::ptr::null_mut());
        }
        _ => {
            return -EINVAL;
        }
    }

    0
}

unsafe fn pcm_playback_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let tscm: *mut snd_tscm = (*substream).private_data as *mut snd_tscm;

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            amdtp_stream_pcm_trigger(&mut (*tscm).rx_stream, substream);
        }
        SNDRV_PCM_TRIGGER_STOP => {
            amdtp_stream_pcm_trigger(&mut (*tscm).rx_stream, core::ptr::null_mut());
        }
        _ => {
            return -EINVAL;
        }
    }

    0
}

unsafe fn pcm_capture_pointer(sbstrm: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let tscm: *mut snd_tscm = (*sbstrm).private_data as *mut snd_tscm;

    amdtp_domain_stream_pcm_pointer(&mut (*tscm).domain, &mut (*tscm).tx_stream)
}

unsafe fn pcm_playback_pointer(sbstrm: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let tscm: *mut snd_tscm = (*sbstrm).private_data as *mut snd_tscm;

    amdtp_domain_stream_pcm_pointer(&mut (*tscm).domain, &mut (*tscm).rx_stream)
}

unsafe fn pcm_capture_ack(substream: *mut snd_pcm_substream) -> c_int {
    let tscm: *mut snd_tscm = (*substream).private_data as *mut snd_tscm;

    amdtp_domain_stream_pcm_ack(&mut (*tscm).domain, &mut (*tscm).tx_stream)
}

unsafe fn pcm_playback_ack(substream: *mut snd_pcm_substream) -> c_int {
    let tscm: *mut snd_tscm = (*substream).private_data as *mut snd_tscm;

    amdtp_domain_stream_pcm_ack(&mut (*tscm).domain, &mut (*tscm).rx_stream)
}

pub unsafe fn snd_tscm_create_pcm_devices(tscm: *mut snd_tscm) -> c_int {
    static CAPTURE_OPS: snd_pcm_ops = snd_pcm_ops {
        open: Some(pcm_open),
        close: Some(pcm_close),
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
        hw_params: Some(pcm_hw_params),
        hw_free: Some(pcm_hw_free),
        prepare: Some(pcm_playback_prepare),
        trigger: Some(pcm_playback_trigger),
        pointer: Some(pcm_playback_pointer),
        ack: Some(pcm_playback_ack),
    };
    let mut pcm: *mut snd_pcm = core::ptr::null_mut();
    let mut err: c_int;

    err = snd_pcm_new(
        (*tscm).card,
        (*(*tscm).card).driver,
        0,
        1,
        1,
        &mut pcm,
    );
    if err < 0 {
        return err;
    }

    (*pcm).private_data = tscm as *mut c_void;
    (*pcm).nonatomic = true;
    snprintf(
        (*pcm).name.as_mut_ptr(),
        core::mem::size_of_val(&(*pcm).name),
        c"%s PCM".as_ptr(),
        (*(*tscm).card).shortname,
    );
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &PLAYBACK_OPS);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &CAPTURE_OPS);
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_VMALLOC, core::ptr::null_mut(), 0, 0);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
