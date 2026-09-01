/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ALSA SoC TWL6040 codec driver
 *
 * Author:	Misael Lopez Cruz <x0052729@ti.com>
 */

/* Dependencies from included kernel headers in the original C context. */
#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum twl6040_trim {
    TWL6040_TRIM_TRIM1 = 0,
    TWL6040_TRIM_TRIM2,
    TWL6040_TRIM_TRIM3,
    TWL6040_TRIM_HSOTRIM,
    TWL6040_TRIM_HFOTRIM,
    TWL6040_TRIM_INVAL,
}

pub const fn TWL6040_HSF_TRIM_LEFT(x: i32) -> i32 {
    x & 0x0f
}

pub const fn TWL6040_HSF_TRIM_RIGHT(x: i32) -> i32 {
    (x >> 4) & 0x0f
}

unsafe extern "C" {
    pub fn twl6040_get_dl1_gain(component: *mut snd_soc_component) -> ::core::ffi::c_int;
    pub fn twl6040_hs_jack_detect(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        report: ::core::ffi::c_int,
    );
    pub fn twl6040_get_clk_id(component: *mut snd_soc_component) -> ::core::ffi::c_int;
    pub fn twl6040_get_trim_value(
        component: *mut snd_soc_component,
        trim: twl6040_trim,
    ) -> ::core::ffi::c_int;
    pub fn twl6040_get_hs_step_size(component: *mut snd_soc_component) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
