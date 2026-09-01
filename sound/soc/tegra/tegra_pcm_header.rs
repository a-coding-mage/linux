/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * tegra_pcm.h - Definitions for Tegra PCM driver
 *
 * Author: Stephen Warren <swarren@nvidia.com>
 * Copyright (C) 2010,2012 - NVIDIA, Inc.
 *
 * Based on code copyright/by:
 *
 * Copyright (c) 2009-2010, NVIDIA Corporation.
 * Scott Peterson <speterson@nvidia.com>
 *
 * Copyright (C) 2010 Google, Inc.
 * Iliyan Malchev <malchev@google.com>
 */

/* Depends on declarations from <sound/dmaengine_pcm.h> and <sound/asound.h>. */

unsafe extern "C" {
    pub fn tegra_pcm_new(
        component: *mut snd_soc_component,
        rtd: *mut snd_soc_pcm_runtime,
    ) -> ::core::ffi::c_int;

    pub fn tegra_pcm_open(
        component: *mut snd_soc_component,
        substream: *mut snd_pcm_substream,
    ) -> ::core::ffi::c_int;

    pub fn tegra_pcm_close(
        component: *mut snd_soc_component,
        substream: *mut snd_pcm_substream,
    ) -> ::core::ffi::c_int;

    pub fn tegra_pcm_hw_params(
        component: *mut snd_soc_component,
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
    ) -> ::core::ffi::c_int;

    pub fn tegra_pcm_pointer(
        component: *mut snd_soc_component,
        substream: *mut snd_pcm_substream,
    ) -> snd_pcm_uframes_t;

    pub fn tegra_pcm_platform_register(dev: *mut device) -> ::core::ffi::c_int;

    pub fn devm_tegra_pcm_platform_register(dev: *mut device) -> ::core::ffi::c_int;

    pub fn tegra_pcm_platform_register_with_chan_names(
        dev: *mut device,
        config: *mut snd_dmaengine_pcm_config,
        txdmachan: *mut ::core::ffi::c_char,
        rxdmachan: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;

    pub fn tegra_pcm_platform_unregister(dev: *mut device);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
