// SPDX-License-Identifier: GPL-2.0-only
/*
 * oxfw_pcm.c - a part of driver for OXFW970/971 based devices
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 */

// Translated from C. The original file included "oxfw.h"; its declarations are
// expected to be supplied by the surrounding Rust translation.

use core::ptr;

unsafe extern "C" {
    fn hw_param_interval(
        params: *mut snd_pcm_hw_params,
        var: ::core::ffi::c_uint,
    ) -> *mut snd_interval;
    fn hw_param_interval_c(
        params: *mut snd_pcm_hw_params,
        var: ::core::ffi::c_uint,
    ) -> *const snd_interval;
    fn snd_oxfw_stream_parse_format(
        format: *mut u8,
        formation: *mut snd_oxfw_stream_formation,
    ) -> ::core::ffi::c_int;
    fn snd_interval_test(i: *const snd_interval, val: ::core::ffi::c_uint) -> bool;
    fn snd_interval_refine(
        i: *mut snd_interval,
        v: *const snd_interval,
    ) -> ::core::ffi::c_int;
    fn snd_interval_list(
        i: *mut snd_interval,
        count: ::core::ffi::c_uint,
        list: *mut ::core::ffi::c_uint,
        mask: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn snd_pcm_rate_to_rate_bit(rate: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    fn snd_pcm_hw_rule_add(
        runtime: *mut snd_pcm_runtime,
        cond: ::core::ffi::c_uint,
        var: ::core::ffi::c_int,
        func: unsafe extern "C" fn(*mut snd_pcm_hw_params, *mut snd_pcm_hw_rule) -> ::core::ffi::c_int,
        private: *mut *mut u8,
        dep: ::core::ffi::c_int,
        last: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn amdtp_am824_add_pcm_hw_constraints(
        stream: *mut amdtp_stream,
        runtime: *mut snd_pcm_runtime,
    ) -> ::core::ffi::c_int;
    fn snd_oxfw_stream_get_current_formation(
        oxfw: *mut snd_oxfw,
        dir: avc_general_plug_dir,
        formation: *mut snd_oxfw_stream_formation,
    ) -> ::core::ffi::c_int;
    fn snd_oxfw_stream_lock_try(oxfw: *mut snd_oxfw) -> ::core::ffi::c_int;
    fn snd_oxfw_stream_lock_release(oxfw: *mut snd_oxfw);
    fn snd_pcm_hw_constraint_minmax(
        runtime: *mut snd_pcm_runtime,
        var: ::core::ffi::c_uint,
        min: ::core::ffi::c_uint,
        max: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn snd_pcm_set_sync(substream: *mut snd_pcm_substream);
    fn params_rate(params: *mut snd_pcm_hw_params) -> ::core::ffi::c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> ::core::ffi::c_uint;
    fn params_period_size(params: *mut snd_pcm_hw_params) -> ::core::ffi::c_uint;
    fn params_buffer_size(params: *mut snd_pcm_hw_params) -> ::core::ffi::c_uint;
    fn snd_oxfw_stream_reserve_duplex(
        oxfw: *mut snd_oxfw,
        stream: *mut amdtp_stream,
        rate: ::core::ffi::c_uint,
        channels: ::core::ffi::c_uint,
        frames_per_period: ::core::ffi::c_uint,
        frames_per_buffer: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn snd_oxfw_stream_stop_duplex(oxfw: *mut snd_oxfw);
    fn snd_oxfw_stream_start_duplex(oxfw: *mut snd_oxfw) -> ::core::ffi::c_int;
    fn amdtp_stream_pcm_prepare(stream: *mut amdtp_stream);
    fn amdtp_stream_pcm_trigger(stream: *mut amdtp_stream, pcm: *mut snd_pcm_substream);
    fn amdtp_domain_stream_pcm_pointer(
        d: *mut amdtp_domain,
        s: *mut amdtp_stream,
    ) -> snd_pcm_uframes_t;
    fn amdtp_domain_stream_pcm_ack(
        d: *mut amdtp_domain,
        s: *mut amdtp_stream,
    ) -> ::core::ffi::c_int;
    fn snd_pcm_new(
        card: *mut snd_card,
        id: *const ::core::ffi::c_char,
        device: ::core::ffi::c_int,
        playback_count: ::core::ffi::c_int,
        capture_count: ::core::ffi::c_int,
        rpcm: *mut *mut snd_pcm,
    ) -> ::core::ffi::c_int;
    fn strscpy(dst: *mut ::core::ffi::c_char, src: *const ::core::ffi::c_char) -> isize;
    fn snd_pcm_set_ops(
        pcm: *mut snd_pcm,
        direction: ::core::ffi::c_int,
        ops: *const snd_pcm_ops,
    );
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        ty: ::core::ffi::c_int,
        data: *mut ::core::ffi::c_void,
        min: usize,
        max: usize,
    );
}

unsafe extern "C" fn hw_rule_rate(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> ::core::ffi::c_int {
    let formats = (*rule).private as *mut *mut u8;
    let r = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    let c = hw_param_interval_c(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let mut t = snd_interval {
        min: ::core::ffi::c_uint::MAX,
        max: 0,
        integer: 1,
        ..core::mem::zeroed()
    };
    let mut formation: snd_oxfw_stream_formation = core::mem::zeroed();

    let mut i = 0;
    while i < SND_OXFW_STREAM_FORMAT_ENTRIES {
        if (*formats.add(i as usize)).is_null() {
            i += 1;
            continue;
        }

        let err = snd_oxfw_stream_parse_format(*formats.add(i as usize), &mut formation);
        if err < 0 {
            i += 1;
            continue;
        }
        if !snd_interval_test(c, formation.pcm) {
            i += 1;
            continue;
        }

        t.min = core::cmp::min(t.min, formation.rate);
        t.max = core::cmp::max(t.max, formation.rate);

        i += 1;
    }

    snd_interval_refine(r, &t)
}

unsafe extern "C" fn hw_rule_channels(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> ::core::ffi::c_int {
    let formats = (*rule).private as *mut *mut u8;
    let c = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let r = hw_param_interval_c(params, SNDRV_PCM_HW_PARAM_RATE);
    let mut formation: snd_oxfw_stream_formation = core::mem::zeroed();
    let mut list = [0 as ::core::ffi::c_uint; SND_OXFW_STREAM_FORMAT_ENTRIES as usize];

    let mut count: ::core::ffi::c_uint = 0;
    let mut i = 0;
    while i < SND_OXFW_STREAM_FORMAT_ENTRIES {
        if (*formats.add(i as usize)).is_null() {
            break;
        }

        let err = snd_oxfw_stream_parse_format(*formats.add(i as usize), &mut formation);
        if err < 0 {
            i += 1;
            continue;
        }
        if !snd_interval_test(r, formation.rate) {
            i += 1;
            continue;
        }
        if list[count as usize] == formation.pcm {
            i += 1;
            continue;
        }

        let mut j = 0;
        while j < list.len() {
            if list[j] == formation.pcm {
                break;
            }
            j += 1;
        }
        if j == list.len() {
            list[count as usize] = formation.pcm;
            count += 1;
            if count as usize == list.len() {
                break;
            }
        }

        i += 1;
    }

    snd_interval_list(c, count, list.as_mut_ptr(), 0)
}

unsafe fn limit_channels_and_rates(hw: *mut snd_pcm_hardware, formats: *mut *mut u8) {
    let mut formation: snd_oxfw_stream_formation = core::mem::zeroed();

    (*hw).channels_min = ::core::ffi::c_uint::MAX;
    (*hw).channels_max = 0;

    (*hw).rate_min = ::core::ffi::c_uint::MAX;
    (*hw).rate_max = 0;
    (*hw).rates = 0;

    let mut i = 0;
    while i < SND_OXFW_STREAM_FORMAT_ENTRIES {
        if (*formats.add(i as usize)).is_null() {
            break;
        }

        let err = snd_oxfw_stream_parse_format(*formats.add(i as usize), &mut formation);
        if err < 0 {
            i += 1;
            continue;
        }

        (*hw).channels_min = core::cmp::min((*hw).channels_min, formation.pcm);
        (*hw).channels_max = core::cmp::max((*hw).channels_max, formation.pcm);

        (*hw).rate_min = core::cmp::min((*hw).rate_min, formation.rate);
        (*hw).rate_max = core::cmp::max((*hw).rate_max, formation.rate);
        (*hw).rates |= snd_pcm_rate_to_rate_bit(formation.rate);

        i += 1;
    }
}

unsafe fn init_hw_params(
    oxfw: *mut snd_oxfw,
    substream: *mut snd_pcm_substream,
) -> ::core::ffi::c_int {
    let runtime = (*substream).runtime;
    let formats: *mut *mut u8;
    let stream: *mut amdtp_stream;

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        (*runtime).hw.formats = AM824_IN_PCM_FORMAT_BITS;
        stream = &mut (*oxfw).tx_stream;
        formats = (*oxfw).tx_stream_formats.as_mut_ptr();
    } else {
        (*runtime).hw.formats = AM824_OUT_PCM_FORMAT_BITS;
        stream = &mut (*oxfw).rx_stream;
        formats = (*oxfw).rx_stream_formats.as_mut_ptr();
    }

    limit_channels_and_rates(&mut (*runtime).hw, formats);

    let mut err = snd_pcm_hw_rule_add(
        runtime,
        0,
        SNDRV_PCM_HW_PARAM_CHANNELS,
        hw_rule_channels,
        formats,
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
        formats,
        SNDRV_PCM_HW_PARAM_CHANNELS,
        -1,
    );
    if err < 0 {
        return err;
    }

    amdtp_am824_add_pcm_hw_constraints(stream, runtime)
}

unsafe fn limit_to_current_params(substream: *mut snd_pcm_substream) -> ::core::ffi::c_int {
    let oxfw = (*substream).private_data as *mut snd_oxfw;
    let mut formation: snd_oxfw_stream_formation = core::mem::zeroed();
    let dir: avc_general_plug_dir;

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        dir = AVC_GENERAL_PLUG_DIR_OUT;
    } else {
        dir = AVC_GENERAL_PLUG_DIR_IN;
    }

    let err = snd_oxfw_stream_get_current_formation(oxfw, dir, &mut formation);
    if err < 0 {
        return err;
    }

    (*(*substream).runtime).hw.channels_min = formation.pcm;
    (*(*substream).runtime).hw.channels_max = formation.pcm;
    (*(*substream).runtime).hw.rate_min = formation.rate;
    (*(*substream).runtime).hw.rate_max = formation.rate;
    err
}

unsafe extern "C" fn pcm_open(substream: *mut snd_pcm_substream) -> ::core::ffi::c_int {
    let oxfw = (*substream).private_data as *mut snd_oxfw;
    let d = &mut (*oxfw).domain as *mut amdtp_domain;
    let mut err: ::core::ffi::c_int;

    err = snd_oxfw_stream_lock_try(oxfw);
    if err < 0 {
        return err;
    }

    err = init_hw_params(oxfw, substream);
    if err < 0 {
        snd_oxfw_stream_lock_release(oxfw);
        return err;
    }

    {
        let _guard = mutex_lock(&mut (*oxfw).mutex);
        // When source of clock is not internal or any stream is reserved for
        // transmission of PCM frames, the available sampling rate is limited
        // at current one.
        if (*oxfw).substreams_count > 0 && (*d).events_per_period > 0 {
            let frames_per_period = (*d).events_per_period;
            let frames_per_buffer = (*d).events_per_buffer;

            err = limit_to_current_params(substream);
            if err < 0 {
                snd_oxfw_stream_lock_release(oxfw);
                return err;
            }

            if frames_per_period > 0 {
                err = snd_pcm_hw_constraint_minmax(
                    (*substream).runtime,
                    SNDRV_PCM_HW_PARAM_PERIOD_SIZE,
                    frames_per_period,
                    frames_per_period,
                );
                if err < 0 {
                    snd_oxfw_stream_lock_release(oxfw);
                    return err;
                }

                err = snd_pcm_hw_constraint_minmax(
                    (*substream).runtime,
                    SNDRV_PCM_HW_PARAM_BUFFER_SIZE,
                    frames_per_buffer,
                    frames_per_buffer,
                );
                if err < 0 {
                    snd_oxfw_stream_lock_release(oxfw);
                    return err;
                }
            }
        }
    }

    snd_pcm_set_sync(substream);

    0
}

unsafe extern "C" fn pcm_close(substream: *mut snd_pcm_substream) -> ::core::ffi::c_int {
    let oxfw = (*substream).private_data as *mut snd_oxfw;

    snd_oxfw_stream_lock_release(oxfw);
    0
}

unsafe extern "C" fn pcm_capture_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> ::core::ffi::c_int {
    let oxfw = (*substream).private_data as *mut snd_oxfw;
    let mut err = 0;

    if (*(*substream).runtime).state == SNDRV_PCM_STATE_OPEN {
        let rate = params_rate(hw_params);
        let channels = params_channels(hw_params);
        let frames_per_period = params_period_size(hw_params);
        let frames_per_buffer = params_buffer_size(hw_params);

        let _guard = mutex_lock(&mut (*oxfw).mutex);
        err = snd_oxfw_stream_reserve_duplex(
            oxfw,
            &mut (*oxfw).tx_stream,
            rate,
            channels,
            frames_per_period,
            frames_per_buffer,
        );
        if err >= 0 {
            (*oxfw).substreams_count += 1;
        }
    }

    err
}

unsafe extern "C" fn pcm_playback_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> ::core::ffi::c_int {
    let oxfw = (*substream).private_data as *mut snd_oxfw;
    let mut err = 0;

    if (*(*substream).runtime).state == SNDRV_PCM_STATE_OPEN {
        let rate = params_rate(hw_params);
        let channels = params_channels(hw_params);
        let frames_per_period = params_period_size(hw_params);
        let frames_per_buffer = params_buffer_size(hw_params);

        let _guard = mutex_lock(&mut (*oxfw).mutex);
        err = snd_oxfw_stream_reserve_duplex(
            oxfw,
            &mut (*oxfw).rx_stream,
            rate,
            channels,
            frames_per_period,
            frames_per_buffer,
        );
        if err >= 0 {
            (*oxfw).substreams_count += 1;
        }
    }

    err
}

unsafe extern "C" fn pcm_capture_hw_free(
    substream: *mut snd_pcm_substream,
) -> ::core::ffi::c_int {
    let oxfw = (*substream).private_data as *mut snd_oxfw;

    let _guard = mutex_lock(&mut (*oxfw).mutex);

    if (*(*substream).runtime).state != SNDRV_PCM_STATE_OPEN {
        (*oxfw).substreams_count -= 1;
    }

    snd_oxfw_stream_stop_duplex(oxfw);

    0
}

unsafe extern "C" fn pcm_playback_hw_free(
    substream: *mut snd_pcm_substream,
) -> ::core::ffi::c_int {
    let oxfw = (*substream).private_data as *mut snd_oxfw;

    let _guard = mutex_lock(&mut (*oxfw).mutex);

    if (*(*substream).runtime).state != SNDRV_PCM_STATE_OPEN {
        (*oxfw).substreams_count -= 1;
    }

    snd_oxfw_stream_stop_duplex(oxfw);

    0
}

unsafe extern "C" fn pcm_capture_prepare(
    substream: *mut snd_pcm_substream,
) -> ::core::ffi::c_int {
    let oxfw = (*substream).private_data as *mut snd_oxfw;
    let err: ::core::ffi::c_int;

    {
        let _guard = mutex_lock(&mut (*oxfw).mutex);
        err = snd_oxfw_stream_start_duplex(oxfw);
        if err < 0 {
            return err;
        }
    }

    amdtp_stream_pcm_prepare(&mut (*oxfw).tx_stream);
    0
}

unsafe extern "C" fn pcm_playback_prepare(
    substream: *mut snd_pcm_substream,
) -> ::core::ffi::c_int {
    let oxfw = (*substream).private_data as *mut snd_oxfw;
    let err: ::core::ffi::c_int;

    {
        let _guard = mutex_lock(&mut (*oxfw).mutex);
        err = snd_oxfw_stream_start_duplex(oxfw);
        if err < 0 {
            return err;
        }
    }

    amdtp_stream_pcm_prepare(&mut (*oxfw).rx_stream);
    0
}

unsafe extern "C" fn pcm_capture_trigger(
    substream: *mut snd_pcm_substream,
    cmd: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let oxfw = (*substream).private_data as *mut snd_oxfw;
    let pcm: *mut snd_pcm_substream;

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            pcm = substream;
        }
        SNDRV_PCM_TRIGGER_STOP => {
            pcm = ptr::null_mut();
        }
        _ => {
            return -EINVAL;
        }
    }
    amdtp_stream_pcm_trigger(&mut (*oxfw).tx_stream, pcm);
    0
}

unsafe extern "C" fn pcm_playback_trigger(
    substream: *mut snd_pcm_substream,
    cmd: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let oxfw = (*substream).private_data as *mut snd_oxfw;
    let pcm: *mut snd_pcm_substream;

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            pcm = substream;
        }
        SNDRV_PCM_TRIGGER_STOP => {
            pcm = ptr::null_mut();
        }
        _ => {
            return -EINVAL;
        }
    }
    amdtp_stream_pcm_trigger(&mut (*oxfw).rx_stream, pcm);
    0
}

unsafe extern "C" fn pcm_capture_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let oxfw = (*substream).private_data as *mut snd_oxfw;

    amdtp_domain_stream_pcm_pointer(&mut (*oxfw).domain, &mut (*oxfw).tx_stream)
}

unsafe extern "C" fn pcm_playback_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let oxfw = (*substream).private_data as *mut snd_oxfw;

    amdtp_domain_stream_pcm_pointer(&mut (*oxfw).domain, &mut (*oxfw).rx_stream)
}

unsafe extern "C" fn pcm_capture_ack(substream: *mut snd_pcm_substream) -> ::core::ffi::c_int {
    let oxfw = (*substream).private_data as *mut snd_oxfw;

    amdtp_domain_stream_pcm_ack(&mut (*oxfw).domain, &mut (*oxfw).tx_stream)
}

unsafe extern "C" fn pcm_playback_ack(substream: *mut snd_pcm_substream) -> ::core::ffi::c_int {
    let oxfw = (*substream).private_data as *mut snd_oxfw;

    amdtp_domain_stream_pcm_ack(&mut (*oxfw).domain, &mut (*oxfw).rx_stream)
}

#[no_mangle]
pub unsafe extern "C" fn snd_oxfw_create_pcm(oxfw: *mut snd_oxfw) -> ::core::ffi::c_int {
    static CAPTURE_OPS: snd_pcm_ops = snd_pcm_ops {
        open: Some(pcm_open),
        close: Some(pcm_close),
        hw_params: Some(pcm_capture_hw_params),
        hw_free: Some(pcm_capture_hw_free),
        prepare: Some(pcm_capture_prepare),
        trigger: Some(pcm_capture_trigger),
        pointer: Some(pcm_capture_pointer),
        ack: Some(pcm_capture_ack),
        ..snd_pcm_ops::ZERO
    };
    static PLAYBACK_OPS: snd_pcm_ops = snd_pcm_ops {
        open: Some(pcm_open),
        close: Some(pcm_close),
        hw_params: Some(pcm_playback_hw_params),
        hw_free: Some(pcm_playback_hw_free),
        prepare: Some(pcm_playback_prepare),
        trigger: Some(pcm_playback_trigger),
        pointer: Some(pcm_playback_pointer),
        ack: Some(pcm_playback_ack),
        ..snd_pcm_ops::ZERO
    };
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let mut cap: ::core::ffi::c_uint = 0;

    if (*oxfw).has_output {
        cap = 1;
    }

    let err = snd_pcm_new(
        (*oxfw).card,
        (*(*oxfw).card).driver.as_ptr(),
        0,
        1,
        cap as ::core::ffi::c_int,
        &mut pcm,
    );
    if err < 0 {
        return err;
    }

    (*pcm).private_data = oxfw as *mut ::core::ffi::c_void;
    (*pcm).nonatomic = true;
    strscpy((*pcm).name.as_mut_ptr(), (*(*oxfw).card).shortname.as_ptr());
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &PLAYBACK_OPS);
    if cap > 0 {
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &CAPTURE_OPS);
    }
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_VMALLOC, ptr::null_mut(), 0, 0);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
