/* SPDX-License-Identifier: GPL-2.0
 *
 * soc-link.h
 *
 * Copyright (C) 2019 Renesas Electronics Corp.
 * Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
 */

use core::ffi::c_int;

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_compr_stream {
    _private: [u8; 0],
}

extern "C" {
    pub fn snd_soc_link_init(rtd: *mut snd_soc_pcm_runtime) -> c_int;
    pub fn snd_soc_link_exit(rtd: *mut snd_soc_pcm_runtime);
    pub fn snd_soc_link_be_hw_params_fixup(
        rtd: *mut snd_soc_pcm_runtime,
        params: *mut snd_pcm_hw_params,
    ) -> c_int;

    pub fn snd_soc_link_startup(substream: *mut snd_pcm_substream) -> c_int;
    pub fn snd_soc_link_shutdown(substream: *mut snd_pcm_substream, rollback: c_int);
    pub fn snd_soc_link_prepare(substream: *mut snd_pcm_substream) -> c_int;
    pub fn snd_soc_link_hw_params(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
    ) -> c_int;
    pub fn snd_soc_link_hw_free(substream: *mut snd_pcm_substream, rollback: c_int);

    pub fn snd_soc_link_trigger(
        substream: *mut snd_pcm_substream,
        cmd: c_int,
        rollback: c_int,
    ) -> c_int;
    pub fn snd_soc_link_compr_startup(cstream: *mut snd_compr_stream) -> c_int;
    pub fn snd_soc_link_compr_shutdown(cstream: *mut snd_compr_stream, rollback: c_int);
    pub fn snd_soc_link_compr_set_params(cstream: *mut snd_compr_stream) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
