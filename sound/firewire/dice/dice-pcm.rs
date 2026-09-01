// SPDX-License-Identifier: GPL-2.0-only
/*
 * dice_pcm.c - a part of driver for DICE based devices
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 * Copyright (c) 2014 Takashi Sakamoto <o-takashi@sakamocchi.jp>
 */

// Translated from C source that included "dice.h"; external kernel/driver
// definitions are declared here only as dependencies.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type bool_ = bool;
type snd_pcm_uframes_t = c_uint;

const UINT_MAX: c_uint = c_uint::MAX;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 1;
const SNDRV_PCM_HW_PARAM_PERIOD_SIZE: c_int = 2;
const SNDRV_PCM_HW_PARAM_BUFFER_SIZE: c_int = 3;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 1;
const SNDRV_PCM_STATE_OPEN: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const EINVAL: c_int = 22;
const AMDTP_IN_STREAM: amdtp_stream_direction = 0;
const AMDTP_OUT_STREAM: amdtp_stream_direction = 1;
const AM824_IN_PCM_FORMAT_BITS: c_uint = 0;
const AM824_OUT_PCM_FORMAT_BITS: c_uint = 0;
const CLOCK_SOURCE_AES1: c_uint = 0;
const CLOCK_SOURCE_AES2: c_uint = 1;
const CLOCK_SOURCE_AES3: c_uint = 2;
const CLOCK_SOURCE_AES4: c_uint = 3;
const CLOCK_SOURCE_AES_ANY: c_uint = 4;
const CLOCK_SOURCE_ADAT: c_uint = 5;
const CLOCK_SOURCE_TDIF: c_uint = 6;
const CLOCK_SOURCE_WC: c_uint = 7;
const MAX_STREAMS: usize = 0;
const SND_DICE_RATE_MODE_COUNT: usize = 0;
const SNDRV_DMA_TYPE_VMALLOC: c_int = 0;

type snd_dice_rate_mode = usize;
type amdtp_stream_direction = c_int;

#[repr(C)]
pub struct snd_interval {
    min: c_uint,
    max: c_uint,
    integer: c_uint,
}

#[repr(C)]
pub struct snd_pcm_hardware {
    formats: c_uint,
    rates: c_uint,
    rate_min: c_uint,
    rate_max: c_uint,
    channels_min: c_uint,
    channels_max: c_uint,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    hw: snd_pcm_hardware,
    state: c_int,
}

#[repr(C)]
pub struct snd_pcm {
    device: c_uint,
    private_data: *mut c_void,
    nonatomic: bool_,
    name: *mut c_char,
}

#[repr(C)]
pub struct snd_pcm_substream {
    private_data: *mut snd_dice,
    pcm: *mut snd_pcm,
    runtime: *mut snd_pcm_runtime,
    stream: c_int,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_rule {
    private: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct amdtp_domain {
    events_per_period: c_uint,
    events_per_buffer: c_uint,
}

#[repr(C)]
pub struct amdtp_stream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    shortname: *const c_char,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_dice {
    tx_pcm_chs: [[c_uint; SND_DICE_RATE_MODE_COUNT]; MAX_STREAMS],
    rx_pcm_chs: [[c_uint; SND_DICE_RATE_MODE_COUNT]; MAX_STREAMS],
    tx_stream: [amdtp_stream; MAX_STREAMS],
    rx_stream: [amdtp_stream; MAX_STREAMS],
    domain: amdtp_domain,
    mutex: mutex,
    substreams_counter: c_uint,
    disable_double_pcm_frames: bool_,
    card: *mut snd_card,
}

#[repr(C)]
pub struct snd_pcm_ops {
    open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    ack: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
}

unsafe extern "C" {
    static snd_dice_rates: [c_uint; 0];

    fn hw_param_interval_c(
        params: *mut snd_pcm_hw_params,
        var: c_int,
    ) -> *const snd_interval;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn snd_dice_stream_get_rate_mode(
        dice: *mut snd_dice,
        rate: c_uint,
        mode: *mut snd_dice_rate_mode,
    ) -> c_int;
    fn snd_interval_test(i: *const snd_interval, val: c_uint) -> c_int;
    fn snd_interval_refine(i: *mut snd_interval, v: *const snd_interval) -> c_int;
    fn snd_pcm_rate_to_rate_bit(rate: c_uint) -> c_uint;
    fn snd_pcm_limit_hw_rates(runtime: *mut snd_pcm_runtime);
    fn snd_pcm_hw_rule_add(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_int,
        func: unsafe extern "C" fn(*mut snd_pcm_hw_params, *mut snd_pcm_hw_rule) -> c_int,
        private: *mut snd_pcm_substream,
        dep: c_int,
        last: c_int,
    ) -> c_int;
    fn amdtp_am824_add_pcm_hw_constraints(
        stream: *mut amdtp_stream,
        runtime: *mut snd_pcm_runtime,
    ) -> c_int;
    fn snd_dice_stream_lock_try(dice: *mut snd_dice) -> c_int;
    fn snd_dice_stream_lock_release(dice: *mut snd_dice);
    fn snd_dice_transaction_get_clock_source(dice: *mut snd_dice, source: *mut c_uint) -> c_int;
    fn snd_dice_transaction_get_rate(dice: *mut snd_dice, rate: *mut c_uint) -> c_int;
    fn snd_pcm_hw_constraint_minmax(
        runtime: *mut snd_pcm_runtime,
        var: c_int,
        min: c_uint,
        max: c_uint,
    ) -> c_int;
    fn snd_pcm_set_sync(substream: *mut snd_pcm_substream);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_period_size(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_buffer_size(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_dice_stream_reserve_duplex(
        dice: *mut snd_dice,
        rate: c_uint,
        events_per_period: c_uint,
        events_per_buffer: c_uint,
    ) -> c_int;
    fn snd_dice_stream_stop_duplex(dice: *mut snd_dice);
    fn snd_dice_stream_start_duplex(dice: *mut snd_dice) -> c_int;
    fn amdtp_stream_pcm_prepare(stream: *mut amdtp_stream);
    fn amdtp_stream_pcm_trigger(stream: *mut amdtp_stream, substream: *mut snd_pcm_substream);
    fn amdtp_domain_stream_pcm_pointer(
        domain: *mut amdtp_domain,
        stream: *mut amdtp_stream,
    ) -> snd_pcm_uframes_t;
    fn amdtp_domain_stream_pcm_ack(domain: *mut amdtp_domain, stream: *mut amdtp_stream)
        -> c_int;
    fn snd_pcm_new(
        card: *mut snd_card,
        id: *const c_char,
        device: c_int,
        playback_count: c_uint,
        capture_count: c_uint,
        rpcm: *mut *mut snd_pcm,
    ) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        type_: c_int,
        data: *mut c_void,
        size: usize,
        max: usize,
    );
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
}

struct MutexGuard {
    mutex: *mut mutex,
}

impl MutexGuard {
    unsafe fn new(mutex: *mut mutex) -> Self {
        unsafe {
            mutex_lock(mutex);
        }
        Self { mutex }
    }
}

impl Drop for MutexGuard {
    fn drop(&mut self) {
        unsafe {
            mutex_unlock(self.mutex);
        }
    }
}

unsafe extern "C" fn dice_rate_constraint(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> c_int {
    unsafe {
        let substream = (*rule).private;
        let dice = (*substream).private_data;
        let index = (*(*substream).pcm).device as usize;
        let c = hw_param_interval_c(params, SNDRV_PCM_HW_PARAM_CHANNELS);
        let r = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
        let mut rates = snd_interval {
            min: UINT_MAX,
            max: 0,
            integer: 1,
        };
        let pcm_channels: *mut c_uint;
        let mut mode: snd_dice_rate_mode = 0;

        if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
            pcm_channels = (*dice).tx_pcm_chs[index].as_mut_ptr();
        } else {
            pcm_channels = (*dice).rx_pcm_chs[index].as_mut_ptr();
        }

        let mut i = 0usize;
        while i < snd_dice_rates.len() {
            let rate = snd_dice_rates[i];
            if snd_dice_stream_get_rate_mode(dice, rate, &mut mode) < 0 {
                i += 1;
                continue;
            }

            if snd_interval_test(c, *pcm_channels.add(mode)) == 0 {
                i += 1;
                continue;
            }

            rates.min = rates.min.min(rate);
            rates.max = rates.max.max(rate);
            i += 1;
        }

        snd_interval_refine(r, &rates)
    }
}

unsafe extern "C" fn dice_channels_constraint(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> c_int {
    unsafe {
        let substream = (*rule).private;
        let dice = (*substream).private_data;
        let index = (*(*substream).pcm).device as usize;
        let r = hw_param_interval_c(params, SNDRV_PCM_HW_PARAM_RATE);
        let c = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
        let mut channels = snd_interval {
            min: UINT_MAX,
            max: 0,
            integer: 1,
        };
        let pcm_channels: *mut c_uint;
        let mut mode: snd_dice_rate_mode = 0;

        if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
            pcm_channels = (*dice).tx_pcm_chs[index].as_mut_ptr();
        } else {
            pcm_channels = (*dice).rx_pcm_chs[index].as_mut_ptr();
        }

        let mut i = 0usize;
        while i < snd_dice_rates.len() {
            let rate = snd_dice_rates[i];
            if snd_dice_stream_get_rate_mode(dice, rate, &mut mode) < 0 {
                i += 1;
                continue;
            }

            if snd_interval_test(r, rate) == 0 {
                i += 1;
                continue;
            }

            channels.min = channels.min.min(*pcm_channels.add(mode));
            channels.max = channels.max.max(*pcm_channels.add(mode));
            i += 1;
        }

        snd_interval_refine(c, &channels)
    }
}

unsafe extern "C" fn limit_channels_and_rates(
    dice: *mut snd_dice,
    runtime: *mut snd_pcm_runtime,
    dir: amdtp_stream_direction,
    index: c_uint,
) -> c_int {
    unsafe {
        let hw = &mut (*runtime).hw as *mut snd_pcm_hardware;
        let pcm_channels: *mut c_uint;

        if dir == AMDTP_IN_STREAM {
            pcm_channels = (*dice).tx_pcm_chs[index as usize].as_mut_ptr();
        } else {
            pcm_channels = (*dice).rx_pcm_chs[index as usize].as_mut_ptr();
        }

        (*hw).channels_min = UINT_MAX;
        (*hw).channels_max = 0;

        let mut i = 0usize;
        while i < snd_dice_rates.len() {
            let mut mode: snd_dice_rate_mode = 0;
            let rate = snd_dice_rates[i];
            if snd_dice_stream_get_rate_mode(dice, rate, &mut mode) < 0 {
                i += 1;
                continue;
            }
            (*hw).rates |= snd_pcm_rate_to_rate_bit(rate);

            let channels = *pcm_channels.add(mode);
            if channels == 0 {
                i += 1;
                continue;
            }
            (*hw).channels_min = (*hw).channels_min.min(channels);
            (*hw).channels_max = (*hw).channels_max.max(channels);
            i += 1;
        }

        snd_pcm_limit_hw_rates(runtime);

        0
    }
}

unsafe extern "C" fn init_hw_info(
    dice: *mut snd_dice,
    substream: *mut snd_pcm_substream,
) -> c_int {
    unsafe {
        let runtime = (*substream).runtime;
        let hw = &mut (*runtime).hw as *mut snd_pcm_hardware;
        let index = (*(*substream).pcm).device;
        let dir: amdtp_stream_direction;
        let stream: *mut amdtp_stream;
        let mut err: c_int;

        if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
            (*hw).formats = AM824_IN_PCM_FORMAT_BITS;
            dir = AMDTP_IN_STREAM;
            stream = (*dice).tx_stream.as_mut_ptr().add(index as usize);
        } else {
            (*hw).formats = AM824_OUT_PCM_FORMAT_BITS;
            dir = AMDTP_OUT_STREAM;
            stream = (*dice).rx_stream.as_mut_ptr().add(index as usize);
        }

        err = limit_channels_and_rates(dice, (*substream).runtime, dir, index);
        if err < 0 {
            return err;
        }

        err = snd_pcm_hw_rule_add(
            runtime,
            0,
            SNDRV_PCM_HW_PARAM_RATE,
            dice_rate_constraint,
            substream,
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
            dice_channels_constraint,
            substream,
            SNDRV_PCM_HW_PARAM_RATE,
            -1,
        );
        if err < 0 {
            return err;
        }

        amdtp_am824_add_pcm_hw_constraints(stream, runtime)
    }
}

unsafe extern "C" fn pcm_open(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let dice = (*substream).private_data;
        let d = &mut (*dice).domain as *mut amdtp_domain;
        let mut source: c_uint = 0;
        let internal: bool;
        let mut err: c_int;

        err = snd_dice_stream_lock_try(dice);
        if err < 0 {
            return err;
        }

        err = init_hw_info(dice, substream);
        if err < 0 {
            snd_dice_stream_lock_release(dice);
            return err;
        }

        err = snd_dice_transaction_get_clock_source(dice, &mut source);
        if err < 0 {
            snd_dice_stream_lock_release(dice);
            return err;
        }
        match source {
            CLOCK_SOURCE_AES1 | CLOCK_SOURCE_AES2 | CLOCK_SOURCE_AES3 | CLOCK_SOURCE_AES4
            | CLOCK_SOURCE_AES_ANY | CLOCK_SOURCE_ADAT | CLOCK_SOURCE_TDIF | CLOCK_SOURCE_WC => {
                internal = false;
            }
            _ => {
                internal = true;
            }
        }

        {
            let _guard = MutexGuard::new(&mut (*dice).mutex);
            // When source of clock is not internal or any stream is reserved for
            // transmission of PCM frames, the available sampling rate is limited
            // at current one.
            if !internal || ((*dice).substreams_counter > 0 && (*d).events_per_period > 0) {
                let mut frames_per_period = (*d).events_per_period;
                let mut frames_per_buffer = (*d).events_per_buffer;
                let mut rate: c_uint = 0;

                err = snd_dice_transaction_get_rate(dice, &mut rate);
                if err < 0 {
                    snd_dice_stream_lock_release(dice);
                    return err;
                }

                (*(*substream).runtime).hw.rate_min = rate;
                (*(*substream).runtime).hw.rate_max = rate;

                if frames_per_period > 0 {
                    // For double_pcm_frame quirk.
                    if rate > 96000 && !(*dice).disable_double_pcm_frames {
                        frames_per_period *= 2;
                        frames_per_buffer *= 2;
                    }

                    err = snd_pcm_hw_constraint_minmax(
                        (*substream).runtime,
                        SNDRV_PCM_HW_PARAM_PERIOD_SIZE,
                        frames_per_period,
                        frames_per_period,
                    );
                    if err < 0 {
                        snd_dice_stream_lock_release(dice);
                        return err;
                    }

                    err = snd_pcm_hw_constraint_minmax(
                        (*substream).runtime,
                        SNDRV_PCM_HW_PARAM_BUFFER_SIZE,
                        frames_per_buffer,
                        frames_per_buffer,
                    );
                    if err < 0 {
                        snd_dice_stream_lock_release(dice);
                        return err;
                    }
                }
            }
        }

        snd_pcm_set_sync(substream);

        0
    }
}

unsafe extern "C" fn pcm_close(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let dice = (*substream).private_data;

        snd_dice_stream_lock_release(dice);

        0
    }
}

unsafe extern "C" fn pcm_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    unsafe {
        let dice = (*substream).private_data;
        let mut err: c_int = 0;

        if (*(*substream).runtime).state == SNDRV_PCM_STATE_OPEN {
            let rate = params_rate(hw_params);
            let mut events_per_period = params_period_size(hw_params);
            let mut events_per_buffer = params_buffer_size(hw_params);

            let _guard = MutexGuard::new(&mut (*dice).mutex);
            // For double_pcm_frame quirk.
            if rate > 96000 && !(*dice).disable_double_pcm_frames {
                events_per_period /= 2;
                events_per_buffer /= 2;
            }
            err = snd_dice_stream_reserve_duplex(
                dice,
                rate,
                events_per_period,
                events_per_buffer,
            );
            if err >= 0 {
                (*dice).substreams_counter += 1;
            }
        }

        err
    }
}

unsafe extern "C" fn pcm_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let dice = (*substream).private_data;

        let _guard = MutexGuard::new(&mut (*dice).mutex);

        if (*(*substream).runtime).state != SNDRV_PCM_STATE_OPEN {
            (*dice).substreams_counter -= 1;
        }

        snd_dice_stream_stop_duplex(dice);

        0
    }
}

unsafe extern "C" fn capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let dice = (*substream).private_data;
        let stream = (*dice)
            .tx_stream
            .as_mut_ptr()
            .add((*(*substream).pcm).device as usize);
        let err: c_int;

        {
            let _guard = MutexGuard::new(&mut (*dice).mutex);
            err = snd_dice_stream_start_duplex(dice);
        }
        if err >= 0 {
            amdtp_stream_pcm_prepare(stream);
        }

        0
    }
}

unsafe extern "C" fn playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let dice = (*substream).private_data;
        let stream = (*dice)
            .rx_stream
            .as_mut_ptr()
            .add((*(*substream).pcm).device as usize);
        let err: c_int;

        {
            let _guard = MutexGuard::new(&mut (*dice).mutex);
            err = snd_dice_stream_start_duplex(dice);
        }
        if err >= 0 {
            amdtp_stream_pcm_prepare(stream);
        }

        err
    }
}

unsafe extern "C" fn capture_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    unsafe {
        let dice = (*substream).private_data;
        let stream = (*dice)
            .tx_stream
            .as_mut_ptr()
            .add((*(*substream).pcm).device as usize);

        match cmd {
            SNDRV_PCM_TRIGGER_START => {
                amdtp_stream_pcm_trigger(stream, substream);
            }
            SNDRV_PCM_TRIGGER_STOP => {
                amdtp_stream_pcm_trigger(stream, ptr::null_mut());
            }
            _ => {
                return -EINVAL;
            }
        }

        0
    }
}

unsafe extern "C" fn playback_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    unsafe {
        let dice = (*substream).private_data;
        let stream = (*dice)
            .rx_stream
            .as_mut_ptr()
            .add((*(*substream).pcm).device as usize);

        match cmd {
            SNDRV_PCM_TRIGGER_START => {
                amdtp_stream_pcm_trigger(stream, substream);
            }
            SNDRV_PCM_TRIGGER_STOP => {
                amdtp_stream_pcm_trigger(stream, ptr::null_mut());
            }
            _ => {
                return -EINVAL;
            }
        }

        0
    }
}

unsafe extern "C" fn capture_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    unsafe {
        let dice = (*substream).private_data;
        let stream = (*dice)
            .tx_stream
            .as_mut_ptr()
            .add((*(*substream).pcm).device as usize);

        amdtp_domain_stream_pcm_pointer(&mut (*dice).domain, stream)
    }
}

unsafe extern "C" fn playback_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    unsafe {
        let dice = (*substream).private_data;
        let stream = (*dice)
            .rx_stream
            .as_mut_ptr()
            .add((*(*substream).pcm).device as usize);

        amdtp_domain_stream_pcm_pointer(&mut (*dice).domain, stream)
    }
}

unsafe extern "C" fn capture_ack(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let dice = (*substream).private_data;
        let stream = (*dice)
            .tx_stream
            .as_mut_ptr()
            .add((*(*substream).pcm).device as usize);

        amdtp_domain_stream_pcm_ack(&mut (*dice).domain, stream)
    }
}

unsafe extern "C" fn playback_ack(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let dice = (*substream).private_data;
        let stream = (*dice)
            .rx_stream
            .as_mut_ptr()
            .add((*(*substream).pcm).device as usize);

        amdtp_domain_stream_pcm_ack(&mut (*dice).domain, stream)
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_dice_create_pcm(dice: *mut snd_dice) -> c_int {
    unsafe {
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
        let mut pcm: *mut snd_pcm = ptr::null_mut();
        let mut capture: c_uint;
        let mut playback: c_uint;
        let mut err: c_int;

        let mut i = 0usize;
        while i < MAX_STREAMS {
            capture = 0;
            playback = 0;
            let mut j = 0usize;
            while j < SND_DICE_RATE_MODE_COUNT {
                if (*dice).tx_pcm_chs[i][j] > 0 {
                    capture = 1;
                }
                if (*dice).rx_pcm_chs[i][j] > 0 {
                    playback = 1;
                }
                j += 1;
            }

            err = snd_pcm_new(
                (*dice).card,
                b"DICE\0".as_ptr() as *const c_char,
                i as c_int,
                playback,
                capture,
                &mut pcm,
            );
            if err < 0 {
                return err;
            }
            (*pcm).private_data = dice as *mut c_void;
            (*pcm).nonatomic = true;
            strscpy((*pcm).name, (*(*dice).card).shortname);

            if capture > 0 {
                snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &CAPTURE_OPS);
            }

            if playback > 0 {
                snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &PLAYBACK_OPS);
            }

            snd_pcm_set_managed_buffer_all(
                pcm,
                SNDRV_DMA_TYPE_VMALLOC,
                ptr::null_mut(),
                0,
                0,
            );

            i += 1;
        }

        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
