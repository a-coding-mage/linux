/* SPDX-License-Identifier: GPL-2.0 */
/*
 * The MIPI SDCA specification is available for public downloads at
 * https://www.mipi.org/mipi-sdca-v1-0-download
 *
 * Copyright (C) 2025 Cirrus Logic, Inc. and
 *                    Cirrus Logic International Semiconductor Ltd.
 */

// C header dependencies and build-time header guards are intentionally omitted.

pub enum device {}
pub enum regmap {}
pub enum sdca_function_data {}
pub enum sdca_pde_delay {}
pub enum snd_ctl_elem_value {}
pub enum snd_kcontrol {}
pub enum snd_kcontrol_new {}
pub enum snd_pcm_hw_params {}
pub enum snd_pcm_substream {}
pub enum snd_soc_component_driver {}
pub enum snd_soc_dai {}
pub enum snd_soc_dai_driver {}
pub enum snd_soc_dai_ops {}
pub enum snd_soc_dapm_route {}
pub enum snd_soc_dapm_widget {}
pub enum snd_soc_pcm_stream {}
pub enum sdca_entity {}

/* convenient macro to handle the mono volume in 7.8 fixed format representation */
#[macro_export]
macro_rules! SDCA_SINGLE_Q78_TLV {
    ($xname:expr, $xreg:expr, $xmin:expr, $xmax:expr, $xstep:expr, $tlv_array:expr) => {{
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: $xname,
            access: SNDRV_CTL_ELEM_ACCESS_TLV_READ | SNDRV_CTL_ELEM_ACCESS_READWRITE,
            tlv: snd_kcontrol_new_tlv { p: $tlv_array },
            info: Some(snd_soc_info_volsw),
            get: Some(sdca_asoc_q78_get_volsw),
            put: Some(sdca_asoc_q78_put_volsw),
            private_value: (&snd_soc_mixer_control {
                reg: $xreg,
                rreg: $xreg,
                min: $xmin,
                max: $xmax,
                shift: $xstep,
                rshift: $xstep,
                sign_bit: 15,
            }) as *const snd_soc_mixer_control as usize,
        }
    }};
}

/* convenient macro for stereo volume in 7.8 fixed format with separate registers for L/R */
#[macro_export]
macro_rules! SDCA_DOUBLE_Q78_TLV {
    ($xname:expr, $xreg_l:expr, $xreg_r:expr, $xmin:expr, $xmax:expr, $xstep:expr, $tlv_array:expr) => {{
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: $xname,
            access: SNDRV_CTL_ELEM_ACCESS_TLV_READ | SNDRV_CTL_ELEM_ACCESS_READWRITE,
            tlv: snd_kcontrol_new_tlv { p: $tlv_array },
            info: Some(snd_soc_info_volsw),
            get: Some(sdca_asoc_q78_get_volsw),
            put: Some(sdca_asoc_q78_put_volsw),
            private_value: (&snd_soc_mixer_control {
                reg: $xreg_l,
                rreg: $xreg_r,
                min: $xmin,
                max: $xmax,
                shift: $xstep,
                rshift: $xstep,
                sign_bit: 15,
            }) as *const snd_soc_mixer_control as usize,
        }
    }};
}

unsafe extern "C" {
    pub fn sdca_asoc_count_component(dev: *mut device, function: *mut sdca_function_data,
        num_widgets: *mut i32, num_routes: *mut i32, num_controls: *mut i32, num_dais: *mut i32) -> i32;
    pub fn sdca_asoc_populate_dapm(dev: *mut device, function: *mut sdca_function_data,
        widgets: *mut snd_soc_dapm_widget, routes: *mut snd_soc_dapm_route) -> i32;
    pub fn sdca_asoc_populate_controls(dev: *mut device, function: *mut sdca_function_data,
        kctl: *mut snd_kcontrol_new) -> i32;
    pub fn sdca_asoc_populate_dais(dev: *mut device, function: *mut sdca_function_data,
        dais: *mut snd_soc_dai_driver, ops: *const snd_soc_dai_ops) -> i32;
    pub fn sdca_asoc_populate_component(dev: *mut device, function: *mut sdca_function_data,
        component_drv: *mut snd_soc_component_driver, dai_drv: *mut *mut snd_soc_dai_driver,
        num_dai_drv: *mut i32, ops: *const snd_soc_dai_ops) -> i32;
    pub fn sdca_asoc_populate_rate_format(dev: *mut device, function: *mut sdca_function_data,
        entity: *mut sdca_entity, stream: *mut snd_soc_pcm_stream) -> i32;
    pub fn sdca_asoc_set_constraints(dev: *mut device, regmap: *mut regmap,
        function: *mut sdca_function_data, substream: *mut snd_pcm_substream,
        dai: *mut snd_soc_dai) -> i32;
    pub fn sdca_asoc_free_constraints(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai);
    pub fn sdca_asoc_get_port(dev: *mut device, regmap: *mut regmap,
        function: *mut sdca_function_data, dai: *mut snd_soc_dai) -> i32;
    pub fn sdca_asoc_hw_params(dev: *mut device, regmap: *mut regmap,
        function: *mut sdca_function_data, substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> i32;
    pub fn sdca_asoc_q78_put_volsw(kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value) -> i32;
    pub fn sdca_asoc_q78_get_volsw(kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value) -> i32;
    pub fn sdca_asoc_pde_poll_actual_ps(regmap: *mut regmap, function_id: i32, entity_id: i32,
        from_ps: i32, to_ps: i32, pde_delays: *const sdca_pde_delay, num_delays: i32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
