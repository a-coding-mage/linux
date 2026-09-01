// SPDX-License-Identifier: GPL-2.0-only
/*
 * fireworks_pcm.c - a part of driver for Fireworks based devices
 *
 * Copyright (c) 2009-2010 Clemens Ladisch
 * Copyright (c) 2013-2014 Takashi Sakamoto
 */

// C dependency intent: #include "./fireworks.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type snd_pcm_uframes_t = u64;

const UINT_MAX: c_uint = c_uint::MAX;
const EINVAL: c_int = 22;

extern "C" {
    static AM824_IN_PCM_FORMAT_BITS: u64;
    static AM824_OUT_PCM_FORMAT_BITS: u64;
    static SNDRV_PCM_HW_PARAM_RATE: c_int;
    static SNDRV_PCM_HW_PARAM_CHANNELS: c_int;
    static SNDRV_PCM_HW_PARAM_PERIOD_SIZE: c_int;
    static SNDRV_PCM_HW_PARAM_BUFFER_SIZE: c_int;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_STATE_OPEN: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SND_EFW_CLOCK_SOURCE_INTERNAL: snd_efw_clock_source;
    static SNDRV_DMA_TYPE_VMALLOC: c_int;

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
    fn snd_pcm_limit_hw_rates(runtime: *mut snd_pcm_runtime);
    fn snd_pcm_hw_rule_add(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_int,
        func: unsafe extern "C" fn(*mut snd_pcm_hw_params, *mut snd_pcm_hw_rule) -> c_int,
        private: *mut c_uint,
        dep: c_int,
        last: c_int,
    ) -> c_int;
    fn amdtp_am824_add_pcm_hw_constraints(
        s: *mut amdtp_stream,
        runtime: *mut snd_pcm_runtime,
    ) -> c_int;
    fn snd_efw_stream_lock_try(efw: *mut snd_efw) -> c_int;
    fn snd_efw_stream_lock_release(efw: *mut snd_efw);
    fn snd_efw_command_get_clock_source(
        efw: *mut snd_efw,
        clock_source: *mut snd_efw_clock_source,
    ) -> c_int;
    fn snd_efw_command_get_sampling_rate(
        efw: *mut snd_efw,
        sampling_rate: *mut c_uint,
    ) -> c_int;
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
    fn snd_efw_stream_reserve_duplex(
        efw: *mut snd_efw,
        rate: c_uint,
        frames_per_period: c_uint,
        frames_per_buffer: c_uint,
    ) -> c_int;
    fn snd_efw_stream_stop_duplex(efw: *mut snd_efw);
    fn snd_efw_stream_start_duplex(efw: *mut snd_efw) -> c_int;
    fn amdtp_stream_pcm_prepare(s: *mut amdtp_stream);
    fn amdtp_stream_pcm_trigger(s: *mut amdtp_stream, substream: *mut snd_pcm_substream);
    fn amdtp_domain_stream_pcm_pointer(
        d: *mut amdtp_domain,
        s: *mut amdtp_stream,
    ) -> snd_pcm_uframes_t;
    fn amdtp_domain_stream_pcm_ack(d: *mut amdtp_domain, s: *mut amdtp_stream) -> c_int;
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
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
}

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
    pub info: u64,
    pub formats: u64,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub state: c_int,
    pub hw: snd_pcm_hardware,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub private_data: *mut c_void,
    pub stream: c_int,
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
pub struct snd_card {
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
}

#[repr(C)]
pub struct snd_pcm {
    pub private_data: *mut c_void,
    pub nonatomic: bool,
    pub name: [c_char; 80],
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
    pub ack: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
}

#[repr(C)]
pub struct amdtp_stream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct amdtp_domain {
    pub events_per_period: c_uint,
    pub events_per_buffer: c_uint,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct snd_efw_clock_source(pub c_int);

#[repr(C)]
pub struct snd_efw {
    pub tx_stream: amdtp_stream,
    pub rx_stream: amdtp_stream,
    pub domain: amdtp_domain,
    pub pcm_capture_channels: *mut c_uint,
    pub pcm_playback_channels: *mut c_uint,
    pub supported_sampling_rate: c_uint,
    pub substreams_counter: c_uint,
    pub mutex: mutex,
    pub card: *mut snd_card,
}

struct MutexGuard {
    mutex: *mut mutex,
}

impl MutexGuard {
    unsafe fn new(mutex: *mut mutex) -> Self {
        unsafe { mutex_lock(mutex) };
        Self { mutex }
    }
}

impl Drop for MutexGuard {
    fn drop(&mut self) {
        unsafe { mutex_unlock(self.mutex) };
    }
}

/*
 * NOTE:
 * Fireworks changes its AMDTP channels for PCM data according to its sampling
 * rate. There are three modes. Here _XX is either _rx or _tx.
 *  0:  32.0- 48.0 kHz then snd_efw_hwinfo.amdtp_XX_pcm_channels applied
 *  1:  88.2- 96.0 kHz then snd_efw_hwinfo.amdtp_XX_pcm_channels_2x applied
 *  2: 176.4-192.0 kHz then snd_efw_hwinfo.amdtp_XX_pcm_channels_4x applied
 *
 * The number of PCM channels for analog input and output are always fixed but
 * the number of PCM channels for digital input and output are differed.
 *
 * Additionally, according to "AudioFire Owner's Manual Version 2.2", in some
 * model, the number of PCM channels for digital input has more restriction
 * depending on which digital interface is selected.
 *  - S/PDIF coaxial and optical	: use input 1-2
 *  - ADAT optical at 32.0-48.0 kHz	: use input 1-8
 *  - ADAT optical at 88.2-96.0 kHz	: use input 1-4 (S/MUX format)
 *
 * The data in AMDTP channels for blank PCM channels are zero.
 */
static freq_table: [c_uint; 7] = [
    /* multiplier mode 0 */
    32000, 44100, 48000,
    /* multiplier mode 1 */
    88200, 96000,
    /* multiplier mode 2 */
    176400, 192000,
];

#[inline]
fn get_multiplier_mode_with_index(index: c_uint) -> c_uint {
    (((index as c_int) - 1) / 2) as c_uint
}

#[no_mangle]
pub unsafe extern "C" fn snd_efw_get_multiplier_mode(
    sampling_rate: c_uint,
    mode: *mut c_uint,
) -> c_int {
    let mut i: c_uint = 0;

    while (i as usize) < freq_table.len() {
        if freq_table[i as usize] == sampling_rate {
            unsafe {
                *mode = get_multiplier_mode_with_index(i);
            }
            return 0;
        }
        i += 1;
    }

    -EINVAL
}

unsafe extern "C" fn hw_rule_rate(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let pcm_channels = unsafe { (*rule).private as *mut c_uint };
    let r = unsafe { hw_param_interval(params, unsafe { SNDRV_PCM_HW_PARAM_RATE }) };
    let c = unsafe { hw_param_interval_c(params, unsafe { SNDRV_PCM_HW_PARAM_CHANNELS }) };
    let mut t = snd_interval {
        min: UINT_MAX,
        max: 0,
        openmin: 0,
        openmax: 0,
        integer: 1,
        empty: 0,
    };
    let mut i: c_uint = 0;

    while (i as usize) < freq_table.len() {
        let mode = get_multiplier_mode_with_index(i);
        if unsafe { !snd_interval_test(c, *pcm_channels.add(mode as usize)) } {
            i += 1;
            continue;
        }

        t.min = core::cmp::min(t.min, freq_table[i as usize]);
        t.max = core::cmp::max(t.max, freq_table[i as usize]);
        i += 1;
    }

    unsafe { snd_interval_refine(r, &t) }
}

unsafe extern "C" fn hw_rule_channels(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let pcm_channels = unsafe { (*rule).private as *mut c_uint };
    let c = unsafe { hw_param_interval(params, unsafe { SNDRV_PCM_HW_PARAM_CHANNELS }) };
    let r = unsafe { hw_param_interval_c(params, unsafe { SNDRV_PCM_HW_PARAM_RATE }) };
    let mut t = snd_interval {
        min: UINT_MAX,
        max: 0,
        openmin: 0,
        openmax: 0,
        integer: 1,
        empty: 0,
    };
    let mut i: c_uint = 0;

    while (i as usize) < freq_table.len() {
        let mode = get_multiplier_mode_with_index(i);
        if unsafe { !snd_interval_test(r, freq_table[i as usize]) } {
            i += 1;
            continue;
        }

        t.min = core::cmp::min(t.min, unsafe { *pcm_channels.add(mode as usize) });
        t.max = core::cmp::max(t.max, unsafe { *pcm_channels.add(mode as usize) });
        i += 1;
    }

    unsafe { snd_interval_refine(c, &t) }
}

unsafe fn limit_channels(hw: *mut snd_pcm_hardware, pcm_channels: *mut c_uint) {
    let mut i: c_uint = 0;

    unsafe {
        (*hw).channels_min = UINT_MAX;
        (*hw).channels_max = 0;
    }

    while (i as usize) < freq_table.len() {
        let mode = get_multiplier_mode_with_index(i);
        unsafe {
            if *pcm_channels.add(mode as usize) == 0 {
                i += 1;
                continue;
            }

            (*hw).channels_min =
                core::cmp::min((*hw).channels_min, *pcm_channels.add(mode as usize));
            (*hw).channels_max =
                core::cmp::max((*hw).channels_max, *pcm_channels.add(mode as usize));
        }
        i += 1;
    }
}

unsafe fn pcm_init_hw_params(
    efw: *mut snd_efw,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let runtime = unsafe { (*substream).runtime };
    let s: *mut amdtp_stream;
    let pcm_channels: *mut c_uint;
    let mut err: c_int;

    unsafe {
        if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
            (*runtime).hw.formats = AM824_IN_PCM_FORMAT_BITS;
            s = &mut (*efw).tx_stream;
            pcm_channels = (*efw).pcm_capture_channels;
        } else {
            (*runtime).hw.formats = AM824_OUT_PCM_FORMAT_BITS;
            s = &mut (*efw).rx_stream;
            pcm_channels = (*efw).pcm_playback_channels;
        }
    }

    /* limit rates */
    unsafe {
        (*runtime).hw.rates = (*efw).supported_sampling_rate;
        snd_pcm_limit_hw_rates(runtime);

        limit_channels(&mut (*runtime).hw, pcm_channels);

        err = snd_pcm_hw_rule_add(
            runtime,
            0,
            SNDRV_PCM_HW_PARAM_CHANNELS,
            hw_rule_channels,
            pcm_channels,
            SNDRV_PCM_HW_PARAM_RATE,
            -1,
        );
    }
    if err < 0 {
        return err;
    }

    unsafe {
        err = snd_pcm_hw_rule_add(
            runtime,
            0,
            SNDRV_PCM_HW_PARAM_RATE,
            hw_rule_rate,
            pcm_channels,
            SNDRV_PCM_HW_PARAM_CHANNELS,
            -1,
        );
    }
    if err < 0 {
        return err;
    }

    unsafe { amdtp_am824_add_pcm_hw_constraints(s, runtime) }
}

unsafe extern "C" fn pcm_open(substream: *mut snd_pcm_substream) -> c_int {
    let efw = unsafe { (*substream).private_data as *mut snd_efw };
    let d = unsafe { &mut (*efw).domain as *mut amdtp_domain };
    let mut clock_source = snd_efw_clock_source(0);
    let mut err: c_int;

    err = unsafe { snd_efw_stream_lock_try(efw) };
    if err < 0 {
        return err;
    }

    err = unsafe { pcm_init_hw_params(efw, substream) };
    if err < 0 {
        unsafe { snd_efw_stream_lock_release(efw) };
        return err;
    }

    err = unsafe { snd_efw_command_get_clock_source(efw, &mut clock_source) };
    if err < 0 {
        unsafe { snd_efw_stream_lock_release(efw) };
        return err;
    }

    {
        let _guard = unsafe { MutexGuard::new(&mut (*efw).mutex) };
        // When source of clock is not internal or any stream is reserved for
        // transmission of PCM frames, the available sampling rate is limited
        // at current one.
        if unsafe {
            (clock_source != SND_EFW_CLOCK_SOURCE_INTERNAL)
                || ((*efw).substreams_counter > 0 && (*d).events_per_period > 0)
        } {
            let frames_per_period = unsafe { (*d).events_per_period };
            let frames_per_buffer = unsafe { (*d).events_per_buffer };
            let mut sampling_rate: c_uint = 0;

            err = unsafe { snd_efw_command_get_sampling_rate(efw, &mut sampling_rate) };
            if err < 0 {
                unsafe { snd_efw_stream_lock_release(efw) };
                return err;
            }
            unsafe {
                (*(*substream).runtime).hw.rate_min = sampling_rate;
                (*(*substream).runtime).hw.rate_max = sampling_rate;
            }

            if frames_per_period > 0 {
                err = unsafe {
                    snd_pcm_hw_constraint_minmax(
                        (*substream).runtime,
                        SNDRV_PCM_HW_PARAM_PERIOD_SIZE,
                        frames_per_period,
                        frames_per_period,
                    )
                };
                if err < 0 {
                    unsafe { snd_efw_stream_lock_release(efw) };
                    return err;
                }

                err = unsafe {
                    snd_pcm_hw_constraint_minmax(
                        (*substream).runtime,
                        SNDRV_PCM_HW_PARAM_BUFFER_SIZE,
                        frames_per_buffer,
                        frames_per_buffer,
                    )
                };
                if err < 0 {
                    unsafe { snd_efw_stream_lock_release(efw) };
                    return err;
                }
            }
        }
    }

    unsafe { snd_pcm_set_sync(substream) };

    0
}

unsafe extern "C" fn pcm_close(substream: *mut snd_pcm_substream) -> c_int {
    let efw = unsafe { (*substream).private_data as *mut snd_efw };
    unsafe { snd_efw_stream_lock_release(efw) };
    0
}

unsafe extern "C" fn pcm_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let efw = unsafe { (*substream).private_data as *mut snd_efw };
    let mut err: c_int = 0;

    if unsafe { (*(*substream).runtime).state == SNDRV_PCM_STATE_OPEN } {
        let rate = unsafe { params_rate(hw_params) };
        let frames_per_period = unsafe { params_period_size(hw_params) };
        let frames_per_buffer = unsafe { params_buffer_size(hw_params) };

        let _guard = unsafe { MutexGuard::new(&mut (*efw).mutex) };
        err = unsafe {
            snd_efw_stream_reserve_duplex(efw, rate, frames_per_period, frames_per_buffer)
        };
        if err >= 0 {
            unsafe {
                (*efw).substreams_counter += 1;
            }
        }
    }

    err
}

unsafe extern "C" fn pcm_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let efw = unsafe { (*substream).private_data as *mut snd_efw };

    let _guard = unsafe { MutexGuard::new(&mut (*efw).mutex) };

    if unsafe { (*(*substream).runtime).state != SNDRV_PCM_STATE_OPEN } {
        unsafe {
            (*efw).substreams_counter -= 1;
        }
    }

    unsafe { snd_efw_stream_stop_duplex(efw) };

    0
}

unsafe extern "C" fn pcm_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let efw = unsafe { (*substream).private_data as *mut snd_efw };
    let mut err: c_int;

    err = unsafe { snd_efw_stream_start_duplex(efw) };
    if err >= 0 {
        unsafe { amdtp_stream_pcm_prepare(&mut (*efw).tx_stream) };
    }

    err
}

unsafe extern "C" fn pcm_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let efw = unsafe { (*substream).private_data as *mut snd_efw };
    let mut err: c_int;

    err = unsafe { snd_efw_stream_start_duplex(efw) };
    if err >= 0 {
        unsafe { amdtp_stream_pcm_prepare(&mut (*efw).rx_stream) };
    }

    err
}

unsafe extern "C" fn pcm_capture_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let efw = unsafe { (*substream).private_data as *mut snd_efw };

    unsafe {
        if cmd == SNDRV_PCM_TRIGGER_START {
            amdtp_stream_pcm_trigger(&mut (*efw).tx_stream, substream);
        } else if cmd == SNDRV_PCM_TRIGGER_STOP {
            amdtp_stream_pcm_trigger(&mut (*efw).tx_stream, ptr::null_mut());
        } else {
            return -EINVAL;
        }
    }

    0
}

unsafe extern "C" fn pcm_playback_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let efw = unsafe { (*substream).private_data as *mut snd_efw };

    unsafe {
        if cmd == SNDRV_PCM_TRIGGER_START {
            amdtp_stream_pcm_trigger(&mut (*efw).rx_stream, substream);
        } else if cmd == SNDRV_PCM_TRIGGER_STOP {
            amdtp_stream_pcm_trigger(&mut (*efw).rx_stream, ptr::null_mut());
        } else {
            return -EINVAL;
        }
    }

    0
}

unsafe extern "C" fn pcm_capture_pointer(
    sbstrm: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let efw = unsafe { (*sbstrm).private_data as *mut snd_efw };

    unsafe { amdtp_domain_stream_pcm_pointer(&mut (*efw).domain, &mut (*efw).tx_stream) }
}

unsafe extern "C" fn pcm_playback_pointer(
    sbstrm: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let efw = unsafe { (*sbstrm).private_data as *mut snd_efw };

    unsafe { amdtp_domain_stream_pcm_pointer(&mut (*efw).domain, &mut (*efw).rx_stream) }
}

unsafe extern "C" fn pcm_capture_ack(substream: *mut snd_pcm_substream) -> c_int {
    let efw = unsafe { (*substream).private_data as *mut snd_efw };

    unsafe { amdtp_domain_stream_pcm_ack(&mut (*efw).domain, &mut (*efw).tx_stream) }
}

unsafe extern "C" fn pcm_playback_ack(substream: *mut snd_pcm_substream) -> c_int {
    let efw = unsafe { (*substream).private_data as *mut snd_efw };

    unsafe { amdtp_domain_stream_pcm_ack(&mut (*efw).domain, &mut (*efw).rx_stream) }
}

#[no_mangle]
pub unsafe extern "C" fn snd_efw_create_pcm_devices(efw: *mut snd_efw) -> c_int {
    static capture_ops: snd_pcm_ops = snd_pcm_ops {
        open: Some(pcm_open),
        close: Some(pcm_close),
        hw_params: Some(pcm_hw_params),
        hw_free: Some(pcm_hw_free),
        prepare: Some(pcm_capture_prepare),
        trigger: Some(pcm_capture_trigger),
        pointer: Some(pcm_capture_pointer),
        ack: Some(pcm_capture_ack),
    };
    static playback_ops: snd_pcm_ops = snd_pcm_ops {
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

    err = unsafe {
        snd_pcm_new(
            (*efw).card,
            (*(*efw).card).driver.as_ptr(),
            0,
            1,
            1,
            &mut pcm,
        )
    };
    if err < 0 {
        return err;
    }

    unsafe {
        (*pcm).private_data = efw as *mut c_void;
        (*pcm).nonatomic = true;
        snprintf(
            (*pcm).name.as_mut_ptr(),
            (*pcm).name.len(),
            b"%s PCM\0".as_ptr() as *const c_char,
            (*(*efw).card).shortname.as_ptr(),
        );
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &playback_ops);
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &capture_ops);
        snd_pcm_set_managed_buffer_all(
            pcm,
            SNDRV_DMA_TYPE_VMALLOC,
            ptr::null_mut(),
            0,
            0,
        );
    }

    err
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
