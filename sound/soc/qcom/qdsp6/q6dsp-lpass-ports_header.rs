/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Rust translation of q6dsp-lpass-ports.h.
 *
 * C header guard removed:
 *   __Q6DSP_AUDIO_PORTS_H__
 *
 * External kernel types expected from surrounding bindings:
 *   struct snd_soc_dai;
 *   struct snd_soc_dai_ops;
 *   struct snd_soc_dai_driver;
 *   struct device;
 *   struct snd_soc_component;
 *   struct of_phandle_args;
 */

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct q6dsp_audio_port_dai_driver_config {
    pub probe: Option<unsafe extern "C" fn(dai: *mut snd_soc_dai) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(dai: *mut snd_soc_dai) -> c_int>,
    pub q6hdmi_ops: *const snd_soc_dai_ops,
    pub q6slim_ops: *const snd_soc_dai_ops,
    pub q6i2s_ops: *const snd_soc_dai_ops,
    pub q6tdm_ops: *const snd_soc_dai_ops,
    pub q6dma_ops: *const snd_soc_dai_ops,
    pub q6usb_ops: *const snd_soc_dai_ops,
}

unsafe extern "C" {
    pub fn q6dsp_audio_ports_set_config(
        dev: *mut device,
        cfg: *mut q6dsp_audio_port_dai_driver_config,
        num_dais: *mut c_int,
    ) -> *mut snd_soc_dai_driver;

    pub fn q6dsp_audio_ports_of_xlate_dai_name(
        component: *mut snd_soc_component,
        args: *const of_phandle_args,
        dai_name: *mut *const c_char,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
