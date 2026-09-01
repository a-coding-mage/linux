/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (c) 2018 Baylibre SAS.
 * Author: Jerome Brunet <jbrunet@baylibre.com>
 */

// C dependency: <sound/soc.h>

#[repr(C)]
pub struct meson_codec_glue_input {
    pub params: snd_soc_pcm_stream,
    pub fmt: ::core::ffi::c_uint,
}

/* Input helpers */
unsafe extern "C" {
    pub fn meson_codec_glue_input_get_data(
        dai: *mut snd_soc_dai,
    ) -> *mut meson_codec_glue_input;

    pub fn meson_codec_glue_input_hw_params(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        dai: *mut snd_soc_dai,
    ) -> ::core::ffi::c_int;

    pub fn meson_codec_glue_input_set_fmt(
        dai: *mut snd_soc_dai,
        fmt: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;

    pub fn meson_codec_glue_input_dai_probe(dai: *mut snd_soc_dai) -> ::core::ffi::c_int;
    pub fn meson_codec_glue_input_dai_remove(dai: *mut snd_soc_dai) -> ::core::ffi::c_int;

    /* Output helpers */
    pub fn meson_codec_glue_output_startup(
        substream: *mut snd_pcm_substream,
        dai: *mut snd_soc_dai,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
