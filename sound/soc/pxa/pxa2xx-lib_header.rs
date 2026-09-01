/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies from the original C header:
// #include <uapi/sound/asound.h>
// #include <linux/platform_device.h>

use core::ffi::{c_int, c_ushort};

/* PCM */
#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

// Provided by <uapi/sound/asound.h> in the original C source.
// type snd_pcm_uframes_t = ...;

unsafe extern "C" {
    pub fn pxa2xx_soc_pcm_new(
        component: *mut snd_soc_component,
        rtd: *mut snd_soc_pcm_runtime,
    ) -> c_int;
    pub fn pxa2xx_soc_pcm_open(
        component: *mut snd_soc_component,
        substream: *mut snd_pcm_substream,
    ) -> c_int;
    pub fn pxa2xx_soc_pcm_close(
        component: *mut snd_soc_component,
        substream: *mut snd_pcm_substream,
    ) -> c_int;
    pub fn pxa2xx_soc_pcm_hw_params(
        component: *mut snd_soc_component,
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
    ) -> c_int;
    pub fn pxa2xx_soc_pcm_prepare(
        component: *mut snd_soc_component,
        substream: *mut snd_pcm_substream,
    ) -> c_int;
    pub fn pxa2xx_soc_pcm_trigger(
        component: *mut snd_soc_component,
        substream: *mut snd_pcm_substream,
        cmd: c_int,
    ) -> c_int;
    pub fn pxa2xx_soc_pcm_pointer(
        component: *mut snd_soc_component,
        substream: *mut snd_pcm_substream,
    ) -> snd_pcm_uframes_t;
}

/* AC97 */
unsafe extern "C" {
    pub fn pxa2xx_ac97_read(slot: c_int, reg: c_ushort) -> c_int;
    pub fn pxa2xx_ac97_write(slot: c_int, reg: c_ushort, val: c_ushort) -> c_int;

    pub fn pxa2xx_ac97_try_warm_reset() -> bool;
    pub fn pxa2xx_ac97_try_cold_reset() -> bool;
    pub fn pxa2xx_ac97_finish_reset();

    pub fn pxa2xx_ac97_hw_suspend() -> c_int;
    pub fn pxa2xx_ac97_hw_resume() -> c_int;
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn pxa2xx_ac97_hw_probe(dev: *mut platform_device) -> c_int;
    pub fn pxa2xx_ac97_hw_remove(dev: *mut platform_device);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
