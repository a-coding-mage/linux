/* SPDX-License-Identifier: (GPL-2.0 OR MIT)
 *
 * Copyright (c) 2018 Baylibre SAS.
 * Author: Jerome Brunet <jbrunet@baylibre.com>
 */

/* Rust translation of declarations from axg-tdm-formatter.h.
 *
 * C header guard and include syntax removed.
 * Dependency intent preserved: C included "axg-tdm.h", which supplies
 * axg_tdm_stream and may supply snd_soc_component_driver/regmap_config.
 */

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct axg_tdm_formatter_hw {
    pub skew_offset: ::std::os::raw::c_uint,
}

#[repr(C)]
pub struct axg_tdm_formatter_ops {
    pub get_stream: Option<
        unsafe extern "C" fn(w: *mut snd_soc_dapm_widget) -> *mut axg_tdm_stream,
    >,
    pub enable: Option<unsafe extern "C" fn(map: *mut regmap)>,
    pub disable: Option<unsafe extern "C" fn(map: *mut regmap)>,
    pub prepare: Option<
        unsafe extern "C" fn(
            map: *mut regmap,
            quirks: *const axg_tdm_formatter_hw,
            ts: *mut axg_tdm_stream,
        ) -> ::std::os::raw::c_int,
    >,
}

#[repr(C)]
pub struct axg_tdm_formatter_driver {
    pub component_drv: *const snd_soc_component_driver,
    pub regmap_cfg: *const regmap_config,
    pub ops: *const axg_tdm_formatter_ops,
    pub quirks: *const axg_tdm_formatter_hw,
}

unsafe extern "C" {
    pub fn axg_tdm_formatter_set_channel_masks(
        map: *mut regmap,
        ts: *mut axg_tdm_stream,
        offset: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;

    pub fn axg_tdm_formatter_event(
        w: *mut snd_soc_dapm_widget,
        control: *mut snd_kcontrol,
        event: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn axg_tdm_formatter_probe(pdev: *mut platform_device) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
