// SPDX-License-Identifier: GPL-2.0+
//
// soc-compress.c  --  ALSA SoC Compress
//
// Copyright (C) 2012 Intel Corp.
//
// Authors: Namarta Kohli <namartax.kohli@intel.com>
//          Ramesh Babu K V <ramesh.babu@linux.intel.com>
//          Vinod Koul <vinod.koul@linux.intel.com>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_int = 0;

const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SND_COMPRESS_PLAYBACK: c_int = 0;
const SND_COMPRESS_CAPTURE: c_int = 1;

const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;

const SND_COMPR_TRIGGER_DRAIN: c_int = 7;
const SND_COMPR_TRIGGER_PARTIAL_DRAIN: c_int = 8;

const SND_SOC_DPCM_UPDATE_NO: c_int = 0;
const SND_SOC_DPCM_UPDATE_FE: c_int = 1;

const SND_SOC_DPCM_STATE_OPEN: c_int = 1;
const SND_SOC_DPCM_STATE_CLOSE: c_int = 2;
const SND_SOC_DPCM_STATE_PREPARE: c_int = 3;
const SND_SOC_DPCM_STATE_START: c_int = 4;
const SND_SOC_DPCM_STATE_STOP: c_int = 5;
const SND_SOC_DPCM_STATE_PAUSED: c_int = 6;

const SND_SOC_DPCM_LINK_STATE_FREE: c_int = 0;

const SND_SOC_DAPM_STREAM_START: c_int = 1;
const SND_SOC_DAPM_STREAM_STOP: c_int = 0;

#[repr(C)]
pub struct snd_compr_stream {
    pub direction: c_int,
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub dai_link: *mut snd_soc_dai_link,
    pub dpcm: [snd_soc_dpcm_runtime; 2],
    pub pcm: *mut snd_pcm,
    pub id: c_int,
    pub fe_compr: c_int,
    pub pop_wait: c_int,
    pub delayed_work: delayed_work,
    pub close_delayed_work_func: Option<unsafe extern "C" fn(*mut work_struct)>,
    pub compr: *mut snd_compr,
}

#[repr(C)]
pub struct snd_soc_dpcm_runtime {
    pub runtime_update: c_int,
    pub state: c_int,
    pub hw_params: snd_pcm_hw_params,
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
    pub snd_card: *mut snd_card,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub num_cpus: c_int,
    pub num_codecs: c_int,
    pub dynamic: c_int,
    pub capture_only: c_int,
    pub playback_only: c_int,
    pub nonatomic: c_int,
    pub stream_name: *const c_char,
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_component {
    pub driver: *mut snd_soc_component_driver,
    pub dev: *mut device,
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub compress_ops: *mut snd_compr_ops,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub name: *const c_char,
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_dpcm {
    pub state: c_int,
}

#[repr(C)]
pub struct snd_soc_dapm_widget_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_compr {
    pub ops: *mut snd_compr_ops,
    pub private_data: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct snd_compr_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_compr_stream) -> c_int>,
    pub free: Option<unsafe extern "C" fn(*mut snd_compr_stream) -> c_int>,
    pub set_params:
        Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_params) -> c_int>,
    pub get_params: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_codec) -> c_int>,
    pub set_metadata:
        Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_metadata) -> c_int>,
    pub get_metadata:
        Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_metadata) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_compr_stream, c_int) -> c_int>,
    pub pointer:
        Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_tstamp64) -> c_int>,
    pub ack: Option<unsafe extern "C" fn(*mut snd_compr_stream, size_t) -> c_int>,
    pub get_caps: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_caps) -> c_int>,
    pub get_codec_caps:
        Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_codec_caps) -> c_int>,
    pub copy: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut c_char, size_t) -> c_int>,
}

#[repr(C)]
pub struct snd_pcm {
    pub streams: [snd_pcm_str; 2],
    pub nonatomic: c_int,
}

#[repr(C)]
pub struct snd_pcm_str {
    pub substream: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_compr_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_codec {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_compr_tstamp64 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_compr_metadata {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_compr_caps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_compr_codec_caps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct delayed_work {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dpcm_mutex_lock(rtd: *mut snd_soc_pcm_runtime);
    fn snd_soc_dpcm_mutex_unlock(rtd: *mut snd_soc_pcm_runtime);
    fn snd_soc_card_mutex_lock(card: *mut snd_soc_card);
    fn snd_soc_card_mutex_unlock(card: *mut snd_soc_card);
    fn snd_soc_runtime_activate(rtd: *mut snd_soc_pcm_runtime, stream: c_int);
    fn snd_soc_runtime_deactivate(rtd: *mut snd_soc_pcm_runtime, stream: c_int);
    fn snd_soc_dai_digital_mute(dai: *mut snd_soc_dai, mute: c_int, stream: c_int) -> c_int;
    fn snd_soc_dai_active(dai: *mut snd_soc_dai) -> c_int;
    fn soc_pcm_set_dai_params(dai: *mut snd_soc_dai, params: *mut c_void);
    fn snd_soc_link_compr_shutdown(cstream: *mut snd_compr_stream, rollback: c_int);
    fn snd_soc_link_compr_startup(cstream: *mut snd_compr_stream) -> c_int;
    fn snd_soc_link_compr_set_params(cstream: *mut snd_compr_stream) -> c_int;
    fn snd_soc_dai_compr_shutdown(
        dai: *mut snd_soc_dai,
        cstream: *mut snd_compr_stream,
        rollback: c_int,
    );
    fn snd_soc_dai_compr_startup(
        dai: *mut snd_soc_dai,
        cstream: *mut snd_compr_stream,
    ) -> c_int;
    fn snd_soc_dai_compr_trigger(
        dai: *mut snd_soc_dai,
        cstream: *mut snd_compr_stream,
        cmd: c_int,
    ) -> c_int;
    fn snd_soc_dai_compr_set_params(
        dai: *mut snd_soc_dai,
        cstream: *mut snd_compr_stream,
        params: *mut snd_compr_params,
    ) -> c_int;
    fn snd_soc_dai_compr_get_params(
        dai: *mut snd_soc_dai,
        cstream: *mut snd_compr_stream,
        params: *mut snd_codec,
    ) -> c_int;
    fn snd_soc_dai_compr_ack(
        dai: *mut snd_soc_dai,
        cstream: *mut snd_compr_stream,
        bytes: size_t,
    ) -> c_int;
    fn snd_soc_dai_compr_pointer(
        dai: *mut snd_soc_dai,
        cstream: *mut snd_compr_stream,
        tstamp: *mut snd_compr_tstamp64,
    ) -> c_int;
    fn snd_soc_dai_compr_set_metadata(
        dai: *mut snd_soc_dai,
        cstream: *mut snd_compr_stream,
        metadata: *mut snd_compr_metadata,
    ) -> c_int;
    fn snd_soc_dai_compr_get_metadata(
        dai: *mut snd_soc_dai,
        cstream: *mut snd_compr_stream,
        metadata: *mut snd_compr_metadata,
    ) -> c_int;
    fn snd_soc_component_module_get_when_open(
        component: *mut snd_soc_component,
        cstream: *mut snd_compr_stream,
    ) -> c_int;
    fn snd_soc_component_module_put_when_close(
        component: *mut snd_soc_component,
        cstream: *mut snd_compr_stream,
        rollback: c_int,
    );
    fn snd_soc_component_compr_open(
        component: *mut snd_soc_component,
        cstream: *mut snd_compr_stream,
    ) -> c_int;
    fn snd_soc_component_compr_free(
        component: *mut snd_soc_component,
        cstream: *mut snd_compr_stream,
        rollback: c_int,
    );
    fn snd_soc_component_compr_trigger(cstream: *mut snd_compr_stream, cmd: c_int) -> c_int;
    fn snd_soc_component_compr_set_params(
        cstream: *mut snd_compr_stream,
        params: *mut snd_compr_params,
    ) -> c_int;
    fn snd_soc_component_compr_get_params(
        cstream: *mut snd_compr_stream,
        params: *mut snd_codec,
    ) -> c_int;
    fn snd_soc_component_compr_ack(cstream: *mut snd_compr_stream, bytes: size_t) -> c_int;
    fn snd_soc_component_compr_pointer(
        cstream: *mut snd_compr_stream,
        tstamp: *mut snd_compr_tstamp64,
    ) -> c_int;
    fn snd_soc_component_compr_set_metadata(
        cstream: *mut snd_compr_stream,
        metadata: *mut snd_compr_metadata,
    ) -> c_int;
    fn snd_soc_component_compr_get_metadata(
        cstream: *mut snd_compr_stream,
        metadata: *mut snd_compr_metadata,
    ) -> c_int;
    fn snd_soc_component_compr_get_caps(
        cstream: *mut snd_compr_stream,
        caps: *mut snd_compr_caps,
    ) -> c_int;
    fn snd_soc_component_compr_get_codec_caps(
        cstream: *mut snd_compr_stream,
        caps: *mut snd_compr_codec_caps,
    ) -> c_int;
    fn snd_soc_component_compr_copy(
        cstream: *mut snd_compr_stream,
        buf: *mut c_char,
        count: size_t,
    ) -> c_int;
    fn snd_soc_pcm_component_pm_runtime_get(
        rtd: *mut snd_soc_pcm_runtime,
        cstream: *mut snd_compr_stream,
    ) -> c_int;
    fn snd_soc_pcm_component_pm_runtime_put(
        rtd: *mut snd_soc_pcm_runtime,
        cstream: *mut snd_compr_stream,
        rollback: c_int,
    );
    fn snd_soc_dapm_stream_stop(rtd: *mut snd_soc_pcm_runtime, stream: c_int);
    fn snd_soc_dapm_stream_event(
        rtd: *mut snd_soc_pcm_runtime,
        stream: c_int,
        event: c_int,
    );
    fn dpcm_path_get(
        fe: *mut snd_soc_pcm_runtime,
        stream: c_int,
        list: *mut *mut snd_soc_dapm_widget_list,
    ) -> c_int;
    fn dpcm_path_put(list: *mut *mut snd_soc_dapm_widget_list);
    fn dpcm_add_paths(
        fe: *mut snd_soc_pcm_runtime,
        stream: c_int,
        list: *mut *mut snd_soc_dapm_widget_list,
    );
    fn dpcm_be_dai_startup(fe: *mut snd_soc_pcm_runtime, stream: c_int) -> c_int;
    fn dpcm_be_disconnect(fe: *mut snd_soc_pcm_runtime, stream: c_int);
    fn dpcm_clear_pending_state(fe: *mut snd_soc_pcm_runtime, stream: c_int);
    fn dpcm_be_dai_hw_free(fe: *mut snd_soc_pcm_runtime, stream: c_int);
    fn dpcm_be_dai_shutdown(fe: *mut snd_soc_pcm_runtime, stream: c_int);
    fn dpcm_dapm_stream_event(fe: *mut snd_soc_pcm_runtime, stream: c_int, event: c_int);
    fn dpcm_be_dai_trigger(
        fe: *mut snd_soc_pcm_runtime,
        stream: c_int,
        cmd: c_int,
    ) -> c_int;
    fn dpcm_be_dai_hw_params(fe: *mut snd_soc_pcm_runtime, stream: c_int) -> c_int;
    fn dpcm_be_dai_prepare(fe: *mut snd_soc_pcm_runtime, stream: c_int) -> c_int;
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_int) -> *mut c_void;
    fn snd_pcm_new_internal(
        card: *mut snd_card,
        id: *const c_char,
        device: c_int,
        playback_count: c_int,
        capture_count: c_int,
        rpcm: *mut *mut snd_pcm,
    ) -> c_int;
    fn snd_compress_new(
        card: *mut snd_card,
        device: c_int,
        dirn: c_int,
        id: *const c_char,
        compr: *mut snd_compr,
    ) -> c_int;
    fn snd_soc_dai_stream_valid(dai: *mut snd_soc_dai, stream: c_int) -> c_int;
    fn snd_soc_close_delayed_work(work: *mut work_struct);
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

// Iterator bodies for for_each_rtd_components() and for_each_dpcm_be() are
// supplied by external SoC macros in the original C headers.
unsafe fn for_each_rtd_components<F>(_rtd: *mut snd_soc_pcm_runtime, _f: F)
where
    F: FnMut(c_int, *mut snd_soc_component) -> bool,
{
    todo!("external macro for_each_rtd_components");
}

unsafe fn for_each_dpcm_be<F>(_fe: *mut snd_soc_pcm_runtime, _stream: c_int, _f: F)
where
    F: FnMut(*mut snd_soc_dpcm) -> bool,
{
    todo!("external macro for_each_dpcm_be");
}

unsafe extern "C" fn snd_soc_compr_components_open(cstream: *mut snd_compr_stream) -> c_int {
    let rtd = (*cstream).private_data as *mut snd_soc_pcm_runtime;
    let mut ret: c_int = 0;

    for_each_rtd_components(rtd, |_, component| {
        ret = snd_soc_component_module_get_when_open(component, cstream);
        if ret < 0 {
            return false;
        }

        ret = snd_soc_component_compr_open(component, cstream);
        if ret < 0 {
            return false;
        }

        true
    });

    ret
}

unsafe extern "C" fn snd_soc_compr_components_free(
    cstream: *mut snd_compr_stream,
    rollback: c_int,
) {
    let rtd = (*cstream).private_data as *mut snd_soc_pcm_runtime;

    for_each_rtd_components(rtd, |_, component| {
        snd_soc_component_compr_free(component, cstream, rollback);
        snd_soc_component_module_put_when_close(component, cstream, rollback);
        true
    });
}

unsafe extern "C" fn soc_compr_clean(
    cstream: *mut snd_compr_stream,
    rollback: c_int,
) -> c_int {
    let rtd = (*cstream).private_data as *mut snd_soc_pcm_runtime;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let stream = (*cstream).direction; /* SND_COMPRESS_xxx is same as SNDRV_PCM_STREAM_xxx */

    snd_soc_dpcm_mutex_lock(rtd);

    if rollback == 0 {
        snd_soc_runtime_deactivate(rtd, stream);
    }

    snd_soc_dai_digital_mute(codec_dai, 1, stream);

    if snd_soc_dai_active(cpu_dai) == 0 {
        soc_pcm_set_dai_params(cpu_dai, ptr::null_mut());
    }

    if snd_soc_dai_active(codec_dai) == 0 {
        soc_pcm_set_dai_params(codec_dai, ptr::null_mut());
    }

    snd_soc_link_compr_shutdown(cstream, rollback);

    snd_soc_compr_components_free(cstream, rollback);

    snd_soc_dai_compr_shutdown(cpu_dai, cstream, rollback);

    if rollback == 0 {
        snd_soc_dapm_stream_stop(rtd, stream);
    }

    snd_soc_dpcm_mutex_unlock(rtd);

    snd_soc_pcm_component_pm_runtime_put(rtd, cstream, rollback);

    0
}

unsafe extern "C" fn soc_compr_free(cstream: *mut snd_compr_stream) -> c_int {
    soc_compr_clean(cstream, 0)
}

unsafe extern "C" fn soc_compr_open(cstream: *mut snd_compr_stream) -> c_int {
    let rtd = (*cstream).private_data as *mut snd_soc_pcm_runtime;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let stream = (*cstream).direction; /* SND_COMPRESS_xxx is same as SNDRV_PCM_STREAM_xxx */
    let mut ret: c_int;

    ret = snd_soc_pcm_component_pm_runtime_get(rtd, cstream);
    if ret < 0 {
        if ret < 0 {
            soc_compr_clean(cstream, 1);
        }
        return ret;
    }

    snd_soc_dpcm_mutex_lock(rtd);

    ret = snd_soc_dai_compr_startup(cpu_dai, cstream);
    if ret >= 0 {
        ret = snd_soc_compr_components_open(cstream);
    }
    if ret >= 0 {
        ret = snd_soc_link_compr_startup(cstream);
    }
    if ret >= 0 {
        snd_soc_runtime_activate(rtd, stream);
    }

    snd_soc_dpcm_mutex_unlock(rtd);
    if ret < 0 {
        soc_compr_clean(cstream, 1);
    }

    ret
}

unsafe extern "C" fn soc_compr_open_fe(cstream: *mut snd_compr_stream) -> c_int {
    let fe = (*cstream).private_data as *mut snd_soc_pcm_runtime;
    let cpu_dai = snd_soc_rtd_to_cpu(fe, 0);
    let mut dpcm: *mut snd_soc_dpcm;
    let mut list: *mut snd_soc_dapm_widget_list = ptr::null_mut();
    let stream = (*cstream).direction; /* SND_COMPRESS_xxx is same as SNDRV_PCM_STREAM_xxx */
    let mut ret: c_int;

    snd_soc_card_mutex_lock((*fe).card);

    ret = dpcm_path_get(fe, stream, &mut list);
    if ret < 0 {
        (*fe).dpcm[stream as usize].runtime_update = SND_SOC_DPCM_UPDATE_NO;
        snd_soc_card_mutex_unlock((*fe).card);
        return ret;
    }

    snd_soc_dpcm_mutex_lock(fe);

    /* calculate valid and active FE <-> BE dpcms */
    dpcm_add_paths(fe, stream, &mut list);

    (*fe).dpcm[stream as usize].runtime_update = SND_SOC_DPCM_UPDATE_FE;

    ret = dpcm_be_dai_startup(fe, stream);
    if ret < 0 {
        /* clean up all links */
        for_each_dpcm_be(fe, stream, |entry| {
            dpcm = entry;
            (*dpcm).state = SND_SOC_DPCM_LINK_STATE_FREE;
            true
        });

        dpcm_be_disconnect(fe, stream);
        dpcm_path_put(&mut list);
        snd_soc_dpcm_mutex_unlock(fe);
        (*fe).dpcm[stream as usize].runtime_update = SND_SOC_DPCM_UPDATE_NO;
        snd_soc_card_mutex_unlock((*fe).card);
        return ret;
    }

    ret = snd_soc_dai_compr_startup(cpu_dai, cstream);
    if ret < 0 {
        dpcm_path_put(&mut list);
        snd_soc_dpcm_mutex_unlock(fe);
        (*fe).dpcm[stream as usize].runtime_update = SND_SOC_DPCM_UPDATE_NO;
        snd_soc_card_mutex_unlock((*fe).card);
        return ret;
    }

    ret = snd_soc_compr_components_open(cstream);
    if ret < 0 {
        snd_soc_dai_compr_shutdown(cpu_dai, cstream, 1);
        dpcm_path_put(&mut list);
        snd_soc_dpcm_mutex_unlock(fe);
        (*fe).dpcm[stream as usize].runtime_update = SND_SOC_DPCM_UPDATE_NO;
        snd_soc_card_mutex_unlock((*fe).card);
        return ret;
    }

    ret = snd_soc_link_compr_startup(cstream);
    if ret < 0 {
        snd_soc_compr_components_free(cstream, 1);
        snd_soc_dai_compr_shutdown(cpu_dai, cstream, 1);
        dpcm_path_put(&mut list);
        snd_soc_dpcm_mutex_unlock(fe);
        (*fe).dpcm[stream as usize].runtime_update = SND_SOC_DPCM_UPDATE_NO;
        snd_soc_card_mutex_unlock((*fe).card);
        return ret;
    }

    dpcm_clear_pending_state(fe, stream);
    dpcm_path_put(&mut list);

    (*fe).dpcm[stream as usize].state = SND_SOC_DPCM_STATE_OPEN;
    (*fe).dpcm[stream as usize].runtime_update = SND_SOC_DPCM_UPDATE_NO;

    snd_soc_runtime_activate(fe, stream);
    snd_soc_dpcm_mutex_unlock(fe);

    snd_soc_card_mutex_unlock((*fe).card);

    0
}

unsafe extern "C" fn soc_compr_free_fe(cstream: *mut snd_compr_stream) -> c_int {
    let fe = (*cstream).private_data as *mut snd_soc_pcm_runtime;
    let cpu_dai = snd_soc_rtd_to_cpu(fe, 0);
    let mut dpcm: *mut snd_soc_dpcm;
    let stream = (*cstream).direction; /* SND_COMPRESS_xxx is same as SNDRV_PCM_STREAM_xxx */

    snd_soc_card_mutex_lock((*fe).card);

    snd_soc_dpcm_mutex_lock(fe);
    snd_soc_runtime_deactivate(fe, stream);

    (*fe).dpcm[stream as usize].runtime_update = SND_SOC_DPCM_UPDATE_FE;

    dpcm_be_dai_hw_free(fe, stream);

    dpcm_be_dai_shutdown(fe, stream);

    /* mark FE's links ready to prune */
    for_each_dpcm_be(fe, stream, |entry| {
        dpcm = entry;
        (*dpcm).state = SND_SOC_DPCM_LINK_STATE_FREE;
        true
    });

    dpcm_dapm_stream_event(fe, stream, SND_SOC_DAPM_STREAM_STOP);

    (*fe).dpcm[stream as usize].state = SND_SOC_DPCM_STATE_CLOSE;
    (*fe).dpcm[stream as usize].runtime_update = SND_SOC_DPCM_UPDATE_NO;

    dpcm_be_disconnect(fe, stream);

    snd_soc_dpcm_mutex_unlock(fe);

    snd_soc_link_compr_shutdown(cstream, 0);

    snd_soc_compr_components_free(cstream, 0);

    snd_soc_dai_compr_shutdown(cpu_dai, cstream, 0);

    snd_soc_card_mutex_unlock((*fe).card);

    0
}

unsafe extern "C" fn soc_compr_trigger(cstream: *mut snd_compr_stream, cmd: c_int) -> c_int {
    let rtd = (*cstream).private_data as *mut snd_soc_pcm_runtime;
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let stream = (*cstream).direction; /* SND_COMPRESS_xxx is same as SNDRV_PCM_STREAM_xxx */
    let mut ret: c_int;

    snd_soc_dpcm_mutex_lock(rtd);

    ret = snd_soc_component_compr_trigger(cstream, cmd);
    if ret >= 0 {
        ret = snd_soc_dai_compr_trigger(cpu_dai, cstream, cmd);
    }
    if ret >= 0 {
        match cmd {
            SNDRV_PCM_TRIGGER_START => {
                snd_soc_dai_digital_mute(codec_dai, 0, stream);
            }
            SNDRV_PCM_TRIGGER_STOP => {
                snd_soc_dai_digital_mute(codec_dai, 1, stream);
            }
            _ => {}
        }
    }

    snd_soc_dpcm_mutex_unlock(rtd);
    ret
}

unsafe extern "C" fn soc_compr_trigger_fe(
    cstream: *mut snd_compr_stream,
    cmd: c_int,
) -> c_int {
    let fe = (*cstream).private_data as *mut snd_soc_pcm_runtime;
    let cpu_dai = snd_soc_rtd_to_cpu(fe, 0);
    let stream = (*cstream).direction; /* SND_COMPRESS_xxx is same as SNDRV_PCM_STREAM_xxx */
    let mut ret: c_int;

    if cmd == SND_COMPR_TRIGGER_PARTIAL_DRAIN || cmd == SND_COMPR_TRIGGER_DRAIN {
        return snd_soc_component_compr_trigger(cstream, cmd);
    }

    snd_soc_card_mutex_lock((*fe).card);

    ret = snd_soc_dai_compr_trigger(cpu_dai, cstream, cmd);
    if ret >= 0 {
        ret = snd_soc_component_compr_trigger(cstream, cmd);
    }
    if ret >= 0 {
        (*fe).dpcm[stream as usize].runtime_update = SND_SOC_DPCM_UPDATE_FE;

        ret = dpcm_be_dai_trigger(fe, stream, cmd);

        match cmd {
            SNDRV_PCM_TRIGGER_START
            | SNDRV_PCM_TRIGGER_RESUME
            | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
                (*fe).dpcm[stream as usize].state = SND_SOC_DPCM_STATE_START;
            }
            SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
                (*fe).dpcm[stream as usize].state = SND_SOC_DPCM_STATE_STOP;
            }
            SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
                (*fe).dpcm[stream as usize].state = SND_SOC_DPCM_STATE_PAUSED;
            }
            _ => {}
        }
    }

    (*fe).dpcm[stream as usize].runtime_update = SND_SOC_DPCM_UPDATE_NO;
    snd_soc_card_mutex_unlock((*fe).card);
    ret
}

unsafe extern "C" fn soc_compr_set_params(
    cstream: *mut snd_compr_stream,
    params: *mut snd_compr_params,
) -> c_int {
    let rtd = (*cstream).private_data as *mut snd_soc_pcm_runtime;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let stream = (*cstream).direction; /* SND_COMPRESS_xxx is same as SNDRV_PCM_STREAM_xxx */
    let mut ret: c_int;

    snd_soc_dpcm_mutex_lock(rtd);

    /*
     * First we call set_params for the CPU DAI, then the component
     * driver this should configure the SoC side. If the machine has
     * compressed ops then we call that as well. The expectation is
     * that these callbacks will configure everything for this compress
     * path, like configuring a PCM port for a CODEC.
     */
    ret = snd_soc_dai_compr_set_params(cpu_dai, cstream, params);
    if ret >= 0 {
        ret = snd_soc_component_compr_set_params(cstream, params);
    }
    if ret >= 0 {
        ret = snd_soc_link_compr_set_params(cstream);
    }
    if ret < 0 {
        snd_soc_dpcm_mutex_unlock(rtd);
        return ret;
    }

    snd_soc_dapm_stream_event(rtd, stream, SND_SOC_DAPM_STREAM_START);

    /* cancel any delayed stream shutdown that is pending */
    (*rtd).pop_wait = 0;
    snd_soc_dpcm_mutex_unlock(rtd);

    cancel_delayed_work_sync(&mut (*rtd).delayed_work);

    0
}

unsafe extern "C" fn soc_compr_set_params_fe(
    cstream: *mut snd_compr_stream,
    params: *mut snd_compr_params,
) -> c_int {
    let fe = (*cstream).private_data as *mut snd_soc_pcm_runtime;
    let fe_substream = (*(*fe).pcm).streams[(*cstream).direction as usize].substream;
    let cpu_dai = snd_soc_rtd_to_cpu(fe, 0);
    let stream = (*cstream).direction; /* SND_COMPRESS_xxx is same as SNDRV_PCM_STREAM_xxx */
    let mut ret: c_int;

    snd_soc_card_mutex_lock((*fe).card);

    /*
     * Create an empty hw_params for the BE as the machine driver must
     * fix this up to match DSP decoder and ASRC configuration.
     * I.e. machine driver fixup for compressed BE is mandatory.
     */
    memset(
        &mut (*fe).dpcm[(*fe_substream).stream as usize].hw_params as *mut _ as *mut c_void,
        0,
        size_of::<snd_pcm_hw_params>(),
    );

    (*fe).dpcm[stream as usize].runtime_update = SND_SOC_DPCM_UPDATE_FE;

    snd_soc_dpcm_mutex_lock(fe);
    ret = dpcm_be_dai_hw_params(fe, stream);
    snd_soc_dpcm_mutex_unlock(fe);
    if ret < 0 {
        (*fe).dpcm[stream as usize].runtime_update = SND_SOC_DPCM_UPDATE_NO;
        snd_soc_card_mutex_unlock((*fe).card);
        return ret;
    }

    snd_soc_dpcm_mutex_lock(fe);
    ret = dpcm_be_dai_prepare(fe, stream);
    snd_soc_dpcm_mutex_unlock(fe);
    if ret < 0 {
        (*fe).dpcm[stream as usize].runtime_update = SND_SOC_DPCM_UPDATE_NO;
        snd_soc_card_mutex_unlock((*fe).card);
        return ret;
    }

    ret = snd_soc_dai_compr_set_params(cpu_dai, cstream, params);
    if ret >= 0 {
        ret = snd_soc_component_compr_set_params(cstream, params);
    }
    if ret >= 0 {
        ret = snd_soc_link_compr_set_params(cstream);
    }
    if ret >= 0 {
        snd_soc_dpcm_mutex_lock(fe);
        dpcm_dapm_stream_event(fe, stream, SND_SOC_DAPM_STREAM_START);
        snd_soc_dpcm_mutex_unlock(fe);
        (*fe).dpcm[stream as usize].state = SND_SOC_DPCM_STATE_PREPARE;
    }

    (*fe).dpcm[stream as usize].runtime_update = SND_SOC_DPCM_UPDATE_NO;
    snd_soc_card_mutex_unlock((*fe).card);
    ret
}

unsafe extern "C" fn soc_compr_get_params(
    cstream: *mut snd_compr_stream,
    params: *mut snd_codec,
) -> c_int {
    let rtd = (*cstream).private_data as *mut snd_soc_pcm_runtime;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut ret: c_int = 0;

    snd_soc_dpcm_mutex_lock(rtd);

    ret = snd_soc_dai_compr_get_params(cpu_dai, cstream, params);
    if ret >= 0 {
        ret = snd_soc_component_compr_get_params(cstream, params);
    }

    snd_soc_dpcm_mutex_unlock(rtd);
    ret
}

unsafe extern "C" fn soc_compr_ack(cstream: *mut snd_compr_stream, bytes: size_t) -> c_int {
    let rtd = (*cstream).private_data as *mut snd_soc_pcm_runtime;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut ret: c_int;

    snd_soc_dpcm_mutex_lock(rtd);

    ret = snd_soc_dai_compr_ack(cpu_dai, cstream, bytes);
    if ret >= 0 {
        ret = snd_soc_component_compr_ack(cstream, bytes);
    }

    snd_soc_dpcm_mutex_unlock(rtd);
    ret
}

unsafe extern "C" fn soc_compr_pointer(
    cstream: *mut snd_compr_stream,
    tstamp: *mut snd_compr_tstamp64,
) -> c_int {
    let rtd = (*cstream).private_data as *mut snd_soc_pcm_runtime;
    let mut ret: c_int;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);

    snd_soc_dpcm_mutex_lock(rtd);

    ret = snd_soc_dai_compr_pointer(cpu_dai, cstream, tstamp);
    if ret >= 0 {
        ret = snd_soc_component_compr_pointer(cstream, tstamp);
    }

    snd_soc_dpcm_mutex_unlock(rtd);
    ret
}

unsafe extern "C" fn soc_compr_set_metadata(
    cstream: *mut snd_compr_stream,
    metadata: *mut snd_compr_metadata,
) -> c_int {
    let rtd = (*cstream).private_data as *mut snd_soc_pcm_runtime;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut ret: c_int;

    ret = snd_soc_dai_compr_set_metadata(cpu_dai, cstream, metadata);
    if ret < 0 {
        return ret;
    }

    snd_soc_component_compr_set_metadata(cstream, metadata)
}

unsafe extern "C" fn soc_compr_get_metadata(
    cstream: *mut snd_compr_stream,
    metadata: *mut snd_compr_metadata,
) -> c_int {
    let rtd = (*cstream).private_data as *mut snd_soc_pcm_runtime;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut ret: c_int;

    ret = snd_soc_dai_compr_get_metadata(cpu_dai, cstream, metadata);
    if ret < 0 {
        return ret;
    }

    snd_soc_component_compr_get_metadata(cstream, metadata)
}

/* ASoC Compress operations */
static mut soc_compr_ops: snd_compr_ops = snd_compr_ops {
    open: Some(soc_compr_open),
    free: Some(soc_compr_free),
    set_params: Some(soc_compr_set_params),
    set_metadata: Some(soc_compr_set_metadata),
    get_metadata: Some(soc_compr_get_metadata),
    get_params: Some(soc_compr_get_params),
    trigger: Some(soc_compr_trigger),
    pointer: Some(soc_compr_pointer),
    ack: Some(soc_compr_ack),
    get_caps: Some(snd_soc_component_compr_get_caps),
    get_codec_caps: Some(snd_soc_component_compr_get_codec_caps),
    copy: None,
};

/* ASoC Dynamic Compress operations */
static mut soc_compr_dyn_ops: snd_compr_ops = snd_compr_ops {
    open: Some(soc_compr_open_fe),
    free: Some(soc_compr_free_fe),
    set_params: Some(soc_compr_set_params_fe),
    get_params: Some(soc_compr_get_params),
    set_metadata: Some(soc_compr_set_metadata),
    get_metadata: Some(soc_compr_get_metadata),
    trigger: Some(soc_compr_trigger_fe),
    pointer: Some(soc_compr_pointer),
    ack: Some(soc_compr_ack),
    get_caps: Some(snd_soc_component_compr_get_caps),
    get_codec_caps: Some(snd_soc_component_compr_get_codec_caps),
    copy: None,
};

/**
 * snd_soc_new_compress - create a new compress.
 *
 * @rtd: The runtime for which we will create compress
 *
 * Return: 0 for success, else error.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_new_compress(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let mut component: *mut snd_soc_component;
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut compr: *mut snd_compr;
    let mut be_pcm: *mut snd_pcm = ptr::null_mut();
    let mut new_name: [c_char; 64] = [0; 64];
    let mut ret: c_int = 0;
    let mut direction: c_int = 0;
    let mut playback: c_int = 0;
    let mut capture: c_int = 0;

    /*
     * make sure these are same value,
     * and then use these as equally
     */
    const _: [(); SNDRV_PCM_STREAM_PLAYBACK as usize] = [(); SND_COMPRESS_PLAYBACK as usize];
    const _: [(); SNDRV_PCM_STREAM_CAPTURE as usize] = [(); SND_COMPRESS_CAPTURE as usize];

    if (*(*rtd).dai_link).num_cpus > 1 || (*(*rtd).dai_link).num_codecs > 1 {
        dev_err(
            (*(*rtd).card).dev,
            b"Compress ASoC: Multi CPU/Codec not supported\n\0".as_ptr() as *const c_char,
        );
        return -EINVAL;
    }

    if codec_dai.is_null() {
        dev_err(
            (*(*rtd).card).dev,
            b"Missing codec\n\0".as_ptr() as *const c_char,
        );
        return -EINVAL;
    }

    /* check client and interface hw capabilities */
    if snd_soc_dai_stream_valid(codec_dai, SNDRV_PCM_STREAM_PLAYBACK) != 0
        && snd_soc_dai_stream_valid(cpu_dai, SNDRV_PCM_STREAM_PLAYBACK) != 0
    {
        playback = 1;
    }
    if snd_soc_dai_stream_valid(codec_dai, SNDRV_PCM_STREAM_CAPTURE) != 0
        && snd_soc_dai_stream_valid(cpu_dai, SNDRV_PCM_STREAM_CAPTURE) != 0
    {
        capture = 1;
    }

    /*
     * Compress devices are unidirectional so only one of the directions
     * should be set, check for that (xor)
     */
    if playback + capture != 1 {
        dev_err(
            (*(*rtd).card).dev,
            b"Compress ASoC: Invalid direction for P %d, C %d\n\0".as_ptr() as *const c_char,
            playback,
            capture,
        );
        return -EINVAL;
    }

    if playback != 0 {
        direction = SND_COMPRESS_PLAYBACK;
    } else {
        direction = SND_COMPRESS_CAPTURE;
    }

    compr = devm_kzalloc(
        (*(*rtd).card).dev,
        size_of::<snd_compr>(),
        GFP_KERNEL,
    ) as *mut snd_compr;
    if compr.is_null() {
        return -ENOMEM;
    }

    (*compr).ops = devm_kzalloc(
        (*(*rtd).card).dev,
        size_of::<snd_compr_ops>(),
        GFP_KERNEL,
    ) as *mut snd_compr_ops;
    if (*compr).ops.is_null() {
        return -ENOMEM;
    }

    if (*(*rtd).dai_link).dynamic != 0 {
        let mut playback: c_int = 1;
        let mut capture: c_int = 1;

        if (*(*rtd).dai_link).capture_only != 0 {
            playback = 0;
        }
        if (*(*rtd).dai_link).playback_only != 0 {
            capture = 0;
        }

        snprintf(
            new_name.as_mut_ptr(),
            size_of::<[c_char; 64]>(),
            b"(%s)\0".as_ptr() as *const c_char,
            (*(*rtd).dai_link).stream_name,
        );

        ret = snd_pcm_new_internal(
            (*(*rtd).card).snd_card,
            new_name.as_ptr(),
            (*rtd).id,
            playback,
            capture,
            &mut be_pcm,
        );
        if ret < 0 {
            dev_err(
                (*(*rtd).card).dev,
                b"Compress ASoC: can't create compressed for %s: %d\n\0".as_ptr()
                    as *const c_char,
                (*(*rtd).dai_link).name,
                ret,
            );
            return ret;
        }

        /* inherit atomicity from DAI link */
        (*be_pcm).nonatomic = (*(*rtd).dai_link).nonatomic;

        (*rtd).pcm = be_pcm;
        (*rtd).fe_compr = 1;
        if playback != 0 {
            (*(*be_pcm).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].substream).private_data =
                rtd as *mut c_void;
        }
        if capture != 0 {
            (*(*be_pcm).streams[SNDRV_PCM_STREAM_CAPTURE as usize].substream).private_data =
                rtd as *mut c_void;
        }
        memcpy(
            (*compr).ops as *mut c_void,
            ptr::addr_of!(soc_compr_dyn_ops) as *const c_void,
            size_of::<snd_compr_ops>(),
        );
    } else {
        snprintf(
            new_name.as_mut_ptr(),
            size_of::<[c_char; 64]>(),
            b"%s %s-%d\0".as_ptr() as *const c_char,
            (*(*rtd).dai_link).stream_name,
            (*codec_dai).name,
            (*rtd).id,
        );

        memcpy(
            (*compr).ops as *mut c_void,
            ptr::addr_of!(soc_compr_ops) as *const c_void,
            size_of::<snd_compr_ops>(),
        );
    }

    for_each_rtd_components(rtd, |_, c| {
        component = c;
        if (*component).driver.is_null()
            || (*(*component).driver).compress_ops.is_null()
            || (*(*(*component).driver).compress_ops).copy.is_none()
        {
            return true;
        }

        (*(*compr).ops).copy = Some(snd_soc_component_compr_copy);
        false
    });

    ret = snd_compress_new(
        (*(*rtd).card).snd_card,
        (*rtd).id,
        direction,
        new_name.as_ptr(),
        compr,
    );
    if ret < 0 {
        component = (*snd_soc_rtd_to_codec(rtd, 0)).component;
        dev_err(
            (*component).dev,
            b"Compress ASoC: can't create compress for codec %s: %d\n\0".as_ptr()
                as *const c_char,
            (*component).name,
            ret,
        );
        return ret;
    }

    /* DAPM dai link stream work */
    (*rtd).close_delayed_work_func = Some(snd_soc_close_delayed_work);

    (*rtd).compr = compr;
    (*compr).private_data = rtd as *mut c_void;

    dev_dbg(
        (*(*rtd).card).dev,
        b"Compress ASoC: %s <-> %s mapping ok\n\0".as_ptr() as *const c_char,
        (*codec_dai).name,
        (*cpu_dai).name,
    );

    0
}

// EXPORT_SYMBOL_GPL(snd_soc_new_compress);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
