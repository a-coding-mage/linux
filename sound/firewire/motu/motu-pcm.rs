// SPDX-License-Identifier: GPL-2.0-only
/*
 * motu-pcm.c - a part of driver for MOTU FireWire series
 *
 * Copyright (c) 2015-2017 Takashi Sakamoto <o-takashi@sakamocchi.jp>
 */

// C dependencies: <sound/pcm_params.h>, "motu.h".

unsafe fn motu_rate_constraint(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let formats = (*rule).private as *mut snd_motu_packet_format;

    let c = hw_param_interval_c(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let r = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    let mut rates = snd_interval {
        min: UINT_MAX,
        max: 0,
        integer: 1,
        ..Default::default()
    };
    let mut i: c_uint;
    let mut pcm_channels: c_uint;
    let mut rate: c_uint;
    let mut mode: c_uint;

    i = 0;
    while i < ARRAY_SIZE(snd_motu_clock_rates) as c_uint {
        rate = snd_motu_clock_rates[i as usize];
        mode = i / 2;

        pcm_channels = (*formats).pcm_chunks[mode as usize];
        if snd_interval_test(c, pcm_channels) == 0 {
            i += 1;
            continue;
        }

        rates.min = min(rates.min, rate);
        rates.max = max(rates.max, rate);
        i += 1;
    }

    snd_interval_refine(r, &mut rates)
}

unsafe fn motu_channels_constraint(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let formats = (*rule).private as *mut snd_motu_packet_format;

    let r = hw_param_interval_c(params, SNDRV_PCM_HW_PARAM_RATE);
    let c = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let mut channels = snd_interval {
        min: UINT_MAX,
        max: 0,
        integer: 1,
        ..Default::default()
    };
    let mut i: c_uint;
    let mut pcm_channels: c_uint;
    let mut rate: c_uint;
    let mut mode: c_uint;

    i = 0;
    while i < ARRAY_SIZE(snd_motu_clock_rates) as c_uint {
        rate = snd_motu_clock_rates[i as usize];
        mode = i / 2;

        if snd_interval_test(r, rate) == 0 {
            i += 1;
            continue;
        }

        pcm_channels = (*formats).pcm_chunks[mode as usize];
        channels.min = min(channels.min, pcm_channels);
        channels.max = max(channels.max, pcm_channels);
        i += 1;
    }

    snd_interval_refine(c, &mut channels)
}

unsafe fn limit_channels_and_rates(
    motu: *mut snd_motu,
    runtime: *mut snd_pcm_runtime,
    formats: *mut snd_motu_packet_format,
) {
    let hw = &mut (*runtime).hw as *mut snd_pcm_hardware;
    let mut i: c_uint;
    let mut pcm_channels: c_uint;
    let mut rate: c_uint;
    let mut mode: c_uint;

    (*hw).channels_min = UINT_MAX;
    (*hw).channels_max = 0;

    i = 0;
    while i < ARRAY_SIZE(snd_motu_clock_rates) as c_uint {
        rate = snd_motu_clock_rates[i as usize];
        mode = i / 2;

        pcm_channels = (*formats).pcm_chunks[mode as usize];
        if pcm_channels == 0 {
            i += 1;
            continue;
        }

        (*hw).rates |= snd_pcm_rate_to_rate_bit(rate);
        (*hw).channels_min = min((*hw).channels_min, pcm_channels);
        (*hw).channels_max = max((*hw).channels_max, pcm_channels);
        i += 1;
    }

    snd_pcm_limit_hw_rates(runtime);
}

unsafe fn init_hw_info(
    motu: *mut snd_motu,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let runtime = (*substream).runtime;
    let hw = &mut (*runtime).hw as *mut snd_pcm_hardware;
    let stream: *mut amdtp_stream;
    let formats: *mut snd_motu_packet_format;
    let mut err: c_int;

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        (*hw).formats = SNDRV_PCM_FMTBIT_S32;
        stream = &mut (*motu).tx_stream;
        formats = &mut (*motu).tx_packet_formats;
    } else {
        (*hw).formats = SNDRV_PCM_FMTBIT_S32;
        stream = &mut (*motu).rx_stream;
        formats = &mut (*motu).rx_packet_formats;
    }

    limit_channels_and_rates(motu, runtime, formats);

    err = snd_pcm_hw_rule_add(
        runtime,
        0,
        SNDRV_PCM_HW_PARAM_RATE,
        Some(motu_rate_constraint),
        formats as *mut c_void,
        SNDRV_PCM_HW_PARAM_CHANNELS,
        -1,
    );
    if err < 0 {
        return err;
    }
    err = snd_pcm_hw_rule_add(
        runtime,
        0,
        SNDRV_PCM_HW_PARAM_CHANNELS,
        Some(motu_channels_constraint),
        formats as *mut c_void,
        SNDRV_PCM_HW_PARAM_RATE,
        -1,
    );
    if err < 0 {
        return err;
    }

    amdtp_motu_add_pcm_hw_constraints(stream, runtime)
}

unsafe fn pcm_open(substream: *mut snd_pcm_substream) -> c_int {
    let motu = (*substream).private_data as *mut snd_motu;
    let d = &mut (*motu).domain as *mut amdtp_domain;
    let mut src: snd_motu_clock_source = core::mem::zeroed();
    let mut err: c_int;

    err = snd_motu_stream_lock_try(motu);
    if err < 0 {
        return err;
    }

    // Original C uses scoped_guard(mutex, &motu->mutex) for this block.
    mutex_lock(&mut (*motu).mutex);
    err = snd_motu_stream_cache_packet_formats(motu);
    if err < 0 {
        mutex_unlock(&mut (*motu).mutex);
        snd_motu_stream_lock_release(motu);
        return err;
    }

    err = init_hw_info(motu, substream);
    if err < 0 {
        mutex_unlock(&mut (*motu).mutex);
        snd_motu_stream_lock_release(motu);
        return err;
    }

    err = snd_motu_protocol_get_clock_source(motu, &mut src);
    if err < 0 {
        mutex_unlock(&mut (*motu).mutex);
        snd_motu_stream_lock_release(motu);
        return err;
    }

    // When source of clock is not internal or any stream is reserved for
    // transmission of PCM frames, the available sampling rate is limited
    // at current one.
    if (src != SND_MOTU_CLOCK_SOURCE_INTERNAL && src != SND_MOTU_CLOCK_SOURCE_SPH)
        || ((*motu).substreams_counter > 0 && (*d).events_per_period > 0)
    {
        let frames_per_period = (*d).events_per_period;
        let frames_per_buffer = (*d).events_per_buffer;
        let mut rate: c_uint = 0;

        err = snd_motu_protocol_get_clock_rate(motu, &mut rate);
        if err < 0 {
            mutex_unlock(&mut (*motu).mutex);
            snd_motu_stream_lock_release(motu);
            return err;
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
                mutex_unlock(&mut (*motu).mutex);
                snd_motu_stream_lock_release(motu);
                return err;
            }

            err = snd_pcm_hw_constraint_minmax(
                (*substream).runtime,
                SNDRV_PCM_HW_PARAM_BUFFER_SIZE,
                frames_per_buffer,
                frames_per_buffer,
            );
            if err < 0 {
                mutex_unlock(&mut (*motu).mutex);
                snd_motu_stream_lock_release(motu);
                return err;
            }
        }
    }
    mutex_unlock(&mut (*motu).mutex);

    snd_pcm_set_sync(substream);

    0
}

unsafe fn pcm_close(substream: *mut snd_pcm_substream) -> c_int {
    let motu = (*substream).private_data as *mut snd_motu;

    snd_motu_stream_lock_release(motu);

    0
}

unsafe fn pcm_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let motu = (*substream).private_data as *mut snd_motu;
    let mut err: c_int = 0;

    if (*(*substream).runtime).state == SNDRV_PCM_STATE_OPEN {
        let rate = params_rate(hw_params);
        let frames_per_period = params_period_size(hw_params);
        let frames_per_buffer = params_buffer_size(hw_params);

        mutex_lock(&mut (*motu).mutex);
        err = snd_motu_stream_reserve_duplex(
            motu,
            rate,
            frames_per_period,
            frames_per_buffer,
        );
        if err >= 0 {
            (*motu).substreams_counter += 1;
        }
        mutex_unlock(&mut (*motu).mutex);
    }

    err
}

unsafe fn pcm_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let motu = (*substream).private_data as *mut snd_motu;

    mutex_lock(&mut (*motu).mutex);

    if (*(*substream).runtime).state != SNDRV_PCM_STATE_OPEN {
        (*motu).substreams_counter -= 1;
    }

    snd_motu_stream_stop_duplex(motu);
    mutex_unlock(&mut (*motu).mutex);

    0
}

unsafe fn capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let motu = (*substream).private_data as *mut snd_motu;
    let mut err: c_int;

    // Original C uses scoped_guard(mutex, &motu->mutex) for this block.
    mutex_lock(&mut (*motu).mutex);
    err = snd_motu_stream_start_duplex(motu);
    mutex_unlock(&mut (*motu).mutex);

    if err >= 0 {
        amdtp_stream_pcm_prepare(&mut (*motu).tx_stream);
    }

    0
}

unsafe fn playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let motu = (*substream).private_data as *mut snd_motu;
    let mut err: c_int;

    // Original C uses scoped_guard(mutex, &motu->mutex) for this block.
    mutex_lock(&mut (*motu).mutex);
    err = snd_motu_stream_start_duplex(motu);
    mutex_unlock(&mut (*motu).mutex);

    if err >= 0 {
        amdtp_stream_pcm_prepare(&mut (*motu).rx_stream);
    }

    err
}

unsafe fn capture_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let motu = (*substream).private_data as *mut snd_motu;

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            amdtp_stream_pcm_trigger(&mut (*motu).tx_stream, substream);
        }
        SNDRV_PCM_TRIGGER_STOP => {
            amdtp_stream_pcm_trigger(&mut (*motu).tx_stream, core::ptr::null_mut());
        }
        _ => {
            return -EINVAL;
        }
    }

    0
}

unsafe fn playback_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let motu = (*substream).private_data as *mut snd_motu;

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            amdtp_stream_pcm_trigger(&mut (*motu).rx_stream, substream);
        }
        SNDRV_PCM_TRIGGER_STOP => {
            amdtp_stream_pcm_trigger(&mut (*motu).rx_stream, core::ptr::null_mut());
        }
        _ => {
            return -EINVAL;
        }
    }

    0
}

unsafe fn capture_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let motu = (*substream).private_data as *mut snd_motu;

    amdtp_domain_stream_pcm_pointer(&mut (*motu).domain, &mut (*motu).tx_stream)
}

unsafe fn playback_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let motu = (*substream).private_data as *mut snd_motu;

    amdtp_domain_stream_pcm_pointer(&mut (*motu).domain, &mut (*motu).rx_stream)
}

unsafe fn capture_ack(substream: *mut snd_pcm_substream) -> c_int {
    let motu = (*substream).private_data as *mut snd_motu;

    amdtp_domain_stream_pcm_ack(&mut (*motu).domain, &mut (*motu).tx_stream)
}

unsafe fn playback_ack(substream: *mut snd_pcm_substream) -> c_int {
    let motu = (*substream).private_data as *mut snd_motu;

    amdtp_domain_stream_pcm_ack(&mut (*motu).domain, &mut (*motu).rx_stream)
}

pub unsafe fn snd_motu_create_pcm_devices(motu: *mut snd_motu) -> c_int {
    static CAPTURE_OPS: snd_pcm_ops = snd_pcm_ops {
        open: Some(pcm_open),
        close: Some(pcm_close),
        hw_params: Some(pcm_hw_params),
        hw_free: Some(pcm_hw_free),
        prepare: Some(capture_prepare),
        trigger: Some(capture_trigger),
        pointer: Some(capture_pointer),
        ack: Some(capture_ack),
    };
    static PLAYBACK_OPS: snd_pcm_ops = snd_pcm_ops {
        open: Some(pcm_open),
        close: Some(pcm_close),
        hw_params: Some(pcm_hw_params),
        hw_free: Some(pcm_hw_free),
        prepare: Some(playback_prepare),
        trigger: Some(playback_trigger),
        pointer: Some(playback_pointer),
        ack: Some(playback_ack),
    };
    let mut pcm: *mut snd_pcm = core::ptr::null_mut();
    let mut err: c_int;

    err = snd_pcm_new(
        (*motu).card,
        (*(*motu).card).driver,
        0,
        1,
        1,
        &mut pcm,
    );
    if err < 0 {
        return err;
    }
    (*pcm).private_data = motu as *mut c_void;
    (*pcm).nonatomic = true;
    strscpy((*pcm).name, (*(*motu).card).shortname);

    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &CAPTURE_OPS);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &PLAYBACK_OPS);
    snd_pcm_set_managed_buffer_all(
        pcm,
        SNDRV_DMA_TYPE_VMALLOC,
        core::ptr::null_mut(),
        0,
        0,
    );

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
