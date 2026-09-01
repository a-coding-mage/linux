// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2019 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>

// C dependencies translated from:
// <linux/module.h>
// <sound/pcm_params.h>
// <sound/soc.h>
// <sound/soc-dai.h>
// "meson-codec-glue.h"

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};

extern "C" {
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut c_void;
    fn snd_soc_dai_dma_data_set_playback(dai: *mut snd_soc_dai, data: *mut meson_codec_glue_input);
    fn snd_soc_dai_dma_data_get_playback(dai: *mut snd_soc_dai) -> *mut meson_codec_glue_input;
    fn snd_pcm_rate_to_rate_bit(rate: c_uint) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_dai_get_widget_capture(dai: *mut snd_soc_dai) -> *mut snd_soc_dapm_widget;
    fn snd_soc_runtime_set_dai_fmt(rtd: *mut snd_soc_pcm_runtime, fmt: c_uint) -> c_int;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn kfree(ptr: *mut c_void);
}

extern "C" {
    static snd_soc_dapm_dai_in: c_int;
}

const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_path {
    pub source: *mut snd_soc_dapm_widget,
    pub connect: c_int,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
    pub priv_: *mut c_void,
    pub id: c_int,
}

#[repr(C)]
pub struct snd_soc_dai_driver_playback {
    pub sig_bits: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub playback: snd_soc_dai_driver_playback,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub driver: *mut snd_soc_dai_driver,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub formats: c_ulonglong,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub sig_bits: c_uint,
}

#[repr(C)]
pub struct meson_codec_glue_input {
    pub params: snd_soc_pcm_stream,
    pub fmt: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub c2c_params: *mut snd_soc_pcm_stream,
    pub num_c2c_params: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dai_link: *mut snd_soc_dai_link,
}

extern "C" {
    fn snd_soc_dapm_widget_get_source_path(
        w: *mut snd_soc_dapm_widget,
        cursor: *mut *mut snd_soc_dapm_path,
    ) -> c_int;
}

#[inline]
unsafe fn WARN_ON(condition: bool) -> bool {
    condition
}

#[inline]
unsafe fn kzalloc_obj_meson_codec_glue_input() -> *mut meson_codec_glue_input {
    extern "C" {
        fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    }

    const GFP_KERNEL: c_uint = 0;

    kzalloc(core::mem::size_of::<meson_codec_glue_input>(), GFP_KERNEL) as *mut meson_codec_glue_input
}

unsafe fn meson_codec_glue_get_input(w: *mut snd_soc_dapm_widget) -> *mut snd_soc_dapm_widget {
    let mut p: *mut snd_soc_dapm_path = core::ptr::null_mut();
    let mut in_: *mut snd_soc_dapm_widget;

    while snd_soc_dapm_widget_get_source_path(w, &mut p) != 0 {
        if (*p).connect == 0 {
            continue;
        }

        /* Check that we still are in the same component */
        if snd_soc_dapm_to_component((*w).dapm) != snd_soc_dapm_to_component((*(*p).source).dapm) {
            continue;
        }

        if (*(*p).source).id == snd_soc_dapm_dai_in {
            return (*p).source;
        }

        in_ = meson_codec_glue_get_input((*p).source);
        if !in_.is_null() {
            return in_;
        }
    }

    core::ptr::null_mut()
}

unsafe fn meson_codec_glue_input_set_data(
    dai: *mut snd_soc_dai,
    data: *mut meson_codec_glue_input,
) {
    snd_soc_dai_dma_data_set_playback(dai, data);
}

#[no_mangle]
pub unsafe extern "C" fn meson_codec_glue_input_get_data(
    dai: *mut snd_soc_dai,
) -> *mut meson_codec_glue_input {
    snd_soc_dai_dma_data_get_playback(dai)
}

unsafe fn meson_codec_glue_output_get_input_data(
    w: *mut snd_soc_dapm_widget,
) -> *mut meson_codec_glue_input {
    let in_: *mut snd_soc_dapm_widget = meson_codec_glue_get_input(w);
    let dai: *mut snd_soc_dai;

    if WARN_ON(in_.is_null()) {
        return core::ptr::null_mut();
    }

    dai = (*in_).priv_ as *mut snd_soc_dai;

    meson_codec_glue_input_get_data(dai)
}

#[no_mangle]
pub unsafe extern "C" fn meson_codec_glue_input_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let data: *mut meson_codec_glue_input = meson_codec_glue_input_get_data(dai);

    (*data).params.rates = snd_pcm_rate_to_rate_bit(params_rate(params));
    (*data).params.rate_min = params_rate(params);
    (*data).params.rate_max = params_rate(params);
    (*data).params.formats = 1_u64.wrapping_shl(params_format(params)) as c_ulonglong;
    (*data).params.channels_min = params_channels(params);
    (*data).params.channels_max = params_channels(params);
    (*data).params.sig_bits = (*(*dai).driver).playback.sig_bits;

    0
}

#[no_mangle]
pub unsafe extern "C" fn meson_codec_glue_input_set_fmt(
    dai: *mut snd_soc_dai,
    fmt: c_uint,
) -> c_int {
    let data: *mut meson_codec_glue_input = meson_codec_glue_input_get_data(dai);

    /* Save the source stream format for the downstream link */
    (*data).fmt = fmt;
    0
}

#[no_mangle]
pub unsafe extern "C" fn meson_codec_glue_output_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let w: *mut snd_soc_dapm_widget = snd_soc_dai_get_widget_capture(dai);
    let in_data: *mut meson_codec_glue_input = meson_codec_glue_output_get_input_data(w);

    if in_data.is_null() {
        return -ENODEV;
    }

    if WARN_ON((*(*rtd).dai_link).c2c_params.is_null()) {
        dev_warn((*dai).dev, c"codec2codec link expected\n".as_ptr());
        return -EINVAL;
    }

    /* Replace link params with the input params */
    (*(*rtd).dai_link).c2c_params = &mut (*in_data).params;
    (*(*rtd).dai_link).num_c2c_params = 1;

    snd_soc_runtime_set_dai_fmt(rtd, (*in_data).fmt)
}

#[no_mangle]
pub unsafe extern "C" fn meson_codec_glue_input_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let data: *mut meson_codec_glue_input;

    data = kzalloc_obj_meson_codec_glue_input();
    if data.is_null() {
        return -ENOMEM;
    }

    meson_codec_glue_input_set_data(dai, data);
    0
}

#[no_mangle]
pub unsafe extern "C" fn meson_codec_glue_input_dai_remove(dai: *mut snd_soc_dai) -> c_int {
    let data: *mut meson_codec_glue_input = meson_codec_glue_input_get_data(dai);

    kfree(data as *mut c_void);
    0
}

// MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>");
// MODULE_DESCRIPTION("Amlogic Codec Glue Helpers");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
