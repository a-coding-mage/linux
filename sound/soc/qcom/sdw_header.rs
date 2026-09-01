// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018, The Linux Foundation. All rights reserved.

// C dependency: <linux/soundwire/sdw.h>

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_stream_runtime {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn qcom_snd_sdw_startup(substream: *mut snd_pcm_substream) -> core::ffi::c_int;
    pub fn qcom_snd_sdw_shutdown(substream: *mut snd_pcm_substream);
    pub fn qcom_snd_sdw_prepare(
        substream: *mut snd_pcm_substream,
        stream_prepared: *mut bool,
    ) -> core::ffi::c_int;
    pub fn qcom_snd_sdw_get_stream(
        stream: *mut snd_pcm_substream,
    ) -> *mut sdw_stream_runtime;
    pub fn qcom_snd_sdw_hw_free(
        substream: *mut snd_pcm_substream,
        stream_prepared: *mut bool,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
