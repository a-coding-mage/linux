// SPDX-License-Identifier: GPL-2.0-only
/*
 * bebob_pcm.c - a part of driver for BeBoB based devices
 *
 * Copyright (c) 2013-2014 Takashi Sakamoto
 */

// Rust translation of ./bebob.h dependencies is supplied by the surrounding driver.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const UINT_MAX: c_uint = c_uint::MAX;

unsafe fn hw_rule_rate(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let formations = (*rule).private as *mut snd_bebob_stream_formation;
    let r = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    let c = hw_param_interval_c(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let mut t = snd_interval {
        min: UINT_MAX,
        max: 0,
        integer: 1,
        ..Default::default()
    };
    let mut i: c_uint = 0;

    while i < SND_BEBOB_STRM_FMT_ENTRIES {
        /* entry is invalid */
        if (*formations.add(i as usize)).pcm == 0 {
            i += 1;
            continue;
        }

        if snd_interval_test(c, (*formations.add(i as usize)).pcm) == 0 {
            i += 1;
            continue;
        }

        t.min = min(t.min, snd_bebob_rate_table[i as usize]);
        t.max = max(t.max, snd_bebob_rate_table[i as usize]);

        i += 1;
    }
    snd_interval_refine(r, &mut t)
}

unsafe fn hw_rule_channels(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let formations = (*rule).private as *mut snd_bebob_stream_formation;
    let c = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let r = hw_param_interval_c(params, SNDRV_PCM_HW_PARAM_RATE);
    let mut t = snd_interval {
        min: UINT_MAX,
        max: 0,
        integer: 1,
        ..Default::default()
    };

    let mut i: c_uint = 0;

    while i < SND_BEBOB_STRM_FMT_ENTRIES {
        /* entry is invalid */
        if (*formations.add(i as usize)).pcm == 0 {
            i += 1;
            continue;
        }

        if snd_interval_test(r, snd_bebob_rate_table[i as usize]) == 0 {
            i += 1;
            continue;
        }

        t.min = min(t.min, (*formations.add(i as usize)).pcm);
        t.max = max(t.max, (*formations.add(i as usize)).pcm);

        i += 1;
    }

    snd_interval_refine(c, &mut t)
}

unsafe fn limit_channels_and_rates(
    hw: *mut snd_pcm_hardware,
    formations: *mut snd_bebob_stream_formation,
) {
    let mut i: c_uint = 0;

    (*hw).channels_min = UINT_MAX;
    (*hw).channels_max = 0;

    (*hw).rate_min = UINT_MAX;
    (*hw).rate_max = 0;
    (*hw).rates = 0;

    while i < SND_BEBOB_STRM_FMT_ENTRIES {
        /* entry has no PCM channels */
        if (*formations.add(i as usize)).pcm == 0 {
            i += 1;
            continue;
        }

        (*hw).channels_min = min((*hw).channels_min, (*formations.add(i as usize)).pcm);
        (*hw).channels_max = max((*hw).channels_max, (*formations.add(i as usize)).pcm);

        (*hw).rate_min = min((*hw).rate_min, snd_bebob_rate_table[i as usize]);
        (*hw).rate_max = max((*hw).rate_max, snd_bebob_rate_table[i as usize]);
        (*hw).rates |= snd_pcm_rate_to_rate_bit(snd_bebob_rate_table[i as usize]);

        i += 1;
    }
}

unsafe fn pcm_init_hw_params(
    bebob: *mut snd_bebob,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let runtime = (*substream).runtime;
    let s: *mut amdtp_stream;
    let formations: *mut snd_bebob_stream_formation;
    let mut err: c_int;

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        (*runtime).hw.formats = AM824_IN_PCM_FORMAT_BITS;
        s = &mut (*bebob).tx_stream;
        formations = (*bebob).tx_stream_formations.as_mut_ptr();
    } else {
        (*runtime).hw.formats = AM824_OUT_PCM_FORMAT_BITS;
        s = &mut (*bebob).rx_stream;
        formations = (*bebob).rx_stream_formations.as_mut_ptr();
    }

    limit_channels_and_rates(&mut (*runtime).hw, formations);

    err = snd_pcm_hw_rule_add(
        runtime,
        0,
        SNDRV_PCM_HW_PARAM_CHANNELS,
        Some(hw_rule_channels),
        formations as *mut c_void,
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
        Some(hw_rule_rate),
        formations as *mut c_void,
        SNDRV_PCM_HW_PARAM_CHANNELS,
        -1,
    );
    if err < 0 {
        return err;
    }

    err = amdtp_am824_add_pcm_hw_constraints(s, runtime);
    err
}

unsafe fn pcm_open(substream: *mut snd_pcm_substream) -> c_int {
    let bebob = (*substream).private_data as *mut snd_bebob;
    let spec = (*(*bebob).spec).rate;
    let d = &mut (*bebob).domain as *mut amdtp_domain;
    let mut src: snd_bebob_clock_type = core::mem::zeroed();
    let mut err: c_int;

    err = snd_bebob_stream_lock_try(bebob);
    if err < 0 {
        return err;
    }

    err = pcm_init_hw_params(bebob, substream);
    if err < 0 {
        snd_bebob_stream_lock_release(bebob);
        return err;
    }

    err = snd_bebob_stream_get_clock_src(bebob, &mut src);
    if err < 0 {
        snd_bebob_stream_lock_release(bebob);
        return err;
    }

    mutex_lock(&mut (*bebob).mutex);
    // When source of clock is not internal or any stream is reserved for
    // transmission of PCM frames, the available sampling rate is limited
    // at current one.
    if src == SND_BEBOB_CLOCK_TYPE_EXTERNAL
        || ((*bebob).substreams_counter > 0 && (*d).events_per_period > 0)
    {
        let frames_per_period = (*d).events_per_period;
        let frames_per_buffer = (*d).events_per_buffer;
        let mut sampling_rate: c_uint = 0;

        err = ((*spec).get.unwrap())(bebob, &mut sampling_rate);
        if err < 0 {
            dev_err(
                &mut (*(*bebob).unit).device,
                b"fail to get sampling rate: %d\n\0".as_ptr() as *const c_char,
                err,
            );
            mutex_unlock(&mut (*bebob).mutex);
            snd_bebob_stream_lock_release(bebob);
            return err;
        }

        (*(*substream).runtime).hw.rate_min = sampling_rate;
        (*(*substream).runtime).hw.rate_max = sampling_rate;

        if frames_per_period > 0 {
            err = snd_pcm_hw_constraint_minmax(
                (*substream).runtime,
                SNDRV_PCM_HW_PARAM_PERIOD_SIZE,
                frames_per_period,
                frames_per_period,
            );
            if err < 0 {
                mutex_unlock(&mut (*bebob).mutex);
                snd_bebob_stream_lock_release(bebob);
                return err;
            }

            err = snd_pcm_hw_constraint_minmax(
                (*substream).runtime,
                SNDRV_PCM_HW_PARAM_BUFFER_SIZE,
                frames_per_buffer,
                frames_per_buffer,
            );
            if err < 0 {
                mutex_unlock(&mut (*bebob).mutex);
                snd_bebob_stream_lock_release(bebob);
                return err;
            }
        }
    }
    mutex_unlock(&mut (*bebob).mutex);

    snd_pcm_set_sync(substream);

    0
}

unsafe fn pcm_close(substream: *mut snd_pcm_substream) -> c_int {
    let bebob = (*substream).private_data as *mut snd_bebob;
    snd_bebob_stream_lock_release(bebob);
    0
}

unsafe fn pcm_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let bebob = (*substream).private_data as *mut snd_bebob;
    let mut err: c_int = 0;

    if (*(*substream).runtime).state == SNDRV_PCM_STATE_OPEN {
        let rate = params_rate(hw_params);
        let frames_per_period = params_period_size(hw_params);
        let frames_per_buffer = params_buffer_size(hw_params);

        mutex_lock(&mut (*bebob).mutex);
        err = snd_bebob_stream_reserve_duplex(
            bebob,
            rate,
            frames_per_period,
            frames_per_buffer,
        );
        if err >= 0 {
            (*bebob).substreams_counter += 1;
        }
        mutex_unlock(&mut (*bebob).mutex);
    }

    err
}

unsafe fn pcm_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let bebob = (*substream).private_data as *mut snd_bebob;

    mutex_lock(&mut (*bebob).mutex);

    if (*(*substream).runtime).state != SNDRV_PCM_STATE_OPEN {
        (*bebob).substreams_counter -= 1;
    }

    snd_bebob_stream_stop_duplex(bebob);

    mutex_unlock(&mut (*bebob).mutex);

    0
}

unsafe fn pcm_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let bebob = (*substream).private_data as *mut snd_bebob;
    let err: c_int;

    err = snd_bebob_stream_start_duplex(bebob);
    if err >= 0 {
        amdtp_stream_pcm_prepare(&mut (*bebob).tx_stream);
    }

    err
}

unsafe fn pcm_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let bebob = (*substream).private_data as *mut snd_bebob;
    let err: c_int;

    err = snd_bebob_stream_start_duplex(bebob);
    if err >= 0 {
        amdtp_stream_pcm_prepare(&mut (*bebob).rx_stream);
    }

    err
}

unsafe fn pcm_capture_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let bebob = (*substream).private_data as *mut snd_bebob;

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            amdtp_stream_pcm_trigger(&mut (*bebob).tx_stream, substream);
        }
        SNDRV_PCM_TRIGGER_STOP => {
            amdtp_stream_pcm_trigger(&mut (*bebob).tx_stream, ptr::null_mut());
        }
        _ => {
            return -EINVAL;
        }
    }

    0
}

unsafe fn pcm_playback_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let bebob = (*substream).private_data as *mut snd_bebob;

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            amdtp_stream_pcm_trigger(&mut (*bebob).rx_stream, substream);
        }
        SNDRV_PCM_TRIGGER_STOP => {
            amdtp_stream_pcm_trigger(&mut (*bebob).rx_stream, ptr::null_mut());
        }
        _ => {
            return -EINVAL;
        }
    }

    0
}

unsafe fn pcm_capture_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let bebob = (*substream).private_data as *mut snd_bebob;

    amdtp_domain_stream_pcm_pointer(&mut (*bebob).domain, &mut (*bebob).tx_stream)
}

unsafe fn pcm_playback_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let bebob = (*substream).private_data as *mut snd_bebob;

    amdtp_domain_stream_pcm_pointer(&mut (*bebob).domain, &mut (*bebob).rx_stream)
}

unsafe fn pcm_capture_ack(substream: *mut snd_pcm_substream) -> c_int {
    let bebob = (*substream).private_data as *mut snd_bebob;

    amdtp_domain_stream_pcm_ack(&mut (*bebob).domain, &mut (*bebob).tx_stream)
}

unsafe fn pcm_playback_ack(substream: *mut snd_pcm_substream) -> c_int {
    let bebob = (*substream).private_data as *mut snd_bebob;

    amdtp_domain_stream_pcm_ack(&mut (*bebob).domain, &mut (*bebob).rx_stream)
}

pub unsafe fn snd_bebob_create_pcm_devices(bebob: *mut snd_bebob) -> c_int {
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
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let mut err: c_int;

    err = snd_pcm_new(
        (*bebob).card,
        (*(*bebob).card).driver.as_ptr(),
        0,
        1,
        1,
        &mut pcm,
    );
    if err < 0 {
        return err;
    }

    (*pcm).private_data = bebob as *mut c_void;
    (*pcm).nonatomic = true;
    snprintf(
        (*pcm).name.as_mut_ptr(),
        core::mem::size_of_val(&(*pcm).name),
        b"%s PCM\0".as_ptr() as *const c_char,
        (*(*bebob).card).shortname.as_ptr(),
    );
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &PLAYBACK_OPS);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &CAPTURE_OPS);
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_VMALLOC, ptr::null_mut(), 0, 0);

    err
}


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
