/* SPDX-License-Identifier: (GPL-2.0 OR MIT) */
/*
 * Copyright (c) 2020 BayLibre, SAS.
 * Author: Jerome Brunet <jbrunet@baylibre.com>
 */

use core::ffi::{c_int, c_uint, c_ulong};

#[repr(C)]
pub struct snd_pcm_hardware {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component_driver {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_ops {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _unused: [u8; 0],
}

pub type snd_pcm_uframes_t = c_ulong;

#[repr(C)]
pub struct aiu_fifo {
    pub pcm: *const snd_pcm_hardware,
    pub mem_offset: c_uint,
    pub fifo_block: c_uint,
    pub pclk: *mut clk,
    pub irq: c_int,
}

unsafe extern "C" {
    pub fn aiu_fifo_dai_probe(dai: *mut snd_soc_dai) -> c_int;
    pub fn aiu_fifo_dai_remove(dai: *mut snd_soc_dai) -> c_int;

    pub fn aiu_fifo_pointer(
        component: *mut snd_soc_component,
        substream: *mut snd_pcm_substream,
    ) -> snd_pcm_uframes_t;

    pub fn aiu_fifo_trigger(
        substream: *mut snd_pcm_substream,
        cmd: c_int,
        dai: *mut snd_soc_dai,
    ) -> c_int;
    pub fn aiu_fifo_prepare(
        substream: *mut snd_pcm_substream,
        dai: *mut snd_soc_dai,
    ) -> c_int;
    pub fn aiu_fifo_hw_params(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        dai: *mut snd_soc_dai,
    ) -> c_int;
    pub fn aiu_fifo_startup(
        substream: *mut snd_pcm_substream,
        dai: *mut snd_soc_dai,
    ) -> c_int;
    pub fn aiu_fifo_shutdown(
        substream: *mut snd_pcm_substream,
        dai: *mut snd_soc_dai,
    );
    pub fn aiu_fifo_pcm_new(
        rtd: *mut snd_soc_pcm_runtime,
        dai: *mut snd_soc_dai,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
