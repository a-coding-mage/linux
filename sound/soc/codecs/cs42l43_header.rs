/* SPDX-License-Identifier: GPL-2.0 */
/*
 * CS42L43 CODEC driver internal data
 *
 * Copyright (C) 2022-2023 Cirrus Logic, Inc. and
 *                         Cirrus Logic International Semiconductor Ltd.
 */

/* C header dependencies:
 * linux/completion.h, linux/mutex.h, linux/types.h, linux/workqueue.h,
 * sound/pcm.h
 */

pub const CS42L43_INTERNAL_SYSCLK: u32 = 24576000;
pub const CS42L43_DEFAULT_SLOTS: u32 = 0x3F;

pub const CS42L43_PLL_TIMEOUT_MS: u32 = 200;
pub const CS42L43_SPK_TIMEOUT_MS: u32 = 100;
pub const CS42L43_HP_TIMEOUT_MS: u32 = 2000;
pub const CS42L43_LOAD_TIMEOUT_MS: u32 = 1000;

pub const CS42L43_HP_ILIMIT_BACKOFF_MS: u32 = 1000;
pub const CS42L43_HP_ILIMIT_DECAY_MS: u32 = 300;
pub const CS42L43_HP_ILIMIT_MAX_COUNT: u32 = 4;

pub const CS42L43_ASP_MAX_CHANNELS: usize = 6;
pub const CS42L43_N_EQ_COEFFS: usize = 15;

pub const CS42L43_N_BUTTONS: usize = 6;

pub const EINVAL: i32 = 22;

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cs42l43 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct irq_domain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct delayed_work {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    _private: [u8; 0],
}

#[repr(C)]
pub struct soc_enum {
    _private: [u8; 0],
}

pub type irqreturn_t = ::core::ffi::c_int;

#[repr(C)]
pub struct cs42l43_codec {
    pub dev: *mut device,
    pub core: *mut cs42l43,
    pub component: *mut snd_soc_component,
    pub dom: *mut irq_domain,
    pub shutter_irqs: [::core::ffi::c_uint; 4],

    pub mclk: *mut clk,

    pub n_slots: ::core::ffi::c_int,
    pub slot_width: ::core::ffi::c_int,
    pub tx_slots: [::core::ffi::c_int; CS42L43_ASP_MAX_CHANNELS],
    pub rx_slots: [::core::ffi::c_int; CS42L43_ASP_MAX_CHANNELS],
    pub constraint: snd_pcm_hw_constraint_list,

    pub eq_coeffs: [u32; CS42L43_N_EQ_COEFFS],

    pub refclk_src: ::core::ffi::c_uint,
    pub refclk_freq: ::core::ffi::c_uint,
    pub pll_ready: completion,

    pub decim_cache: [::core::ffi::c_uint; 6],
    pub adc_ena: ::core::ffi::c_uint,
    pub hp_ena: ::core::ffi::c_uint,

    pub hp_startup: completion,
    pub hp_shutdown: completion,
    pub spkr_shutdown: completion,
    pub spkl_shutdown: completion,
    pub spkr_startup: completion,
    pub spkl_startup: completion,
    // Lock to ensure speaker VU updates don't clash
    pub spk_vu_lock: mutex,

    // Lock for all jack detect operations
    pub jack_lock: mutex,
    pub jack_hp: *mut snd_soc_jack,

    pub use_ring_sense: bool,
    pub tip_debounce_ms: ::core::ffi::c_uint,
    pub tip_fall_db_ms: ::core::ffi::c_uint,
    pub tip_rise_db_ms: ::core::ffi::c_uint,
    pub bias_low: ::core::ffi::c_uint,
    pub bias_sense_ua: ::core::ffi::c_uint,
    pub bias_ramp_ms: ::core::ffi::c_uint,
    pub detect_us: ::core::ffi::c_uint,
    pub buttons: [::core::ffi::c_uint; CS42L43_N_BUTTONS],

    pub tip_sense_work: delayed_work,
    pub bias_sense_timeout: delayed_work,
    pub type_detect: completion,
    pub load_detect: completion,

    pub load_detect_running: bool,
    pub button_detect_running: bool,
    pub jack_present: bool,
    pub jack_override: ::core::ffi::c_int,
    pub suspend_jack_debounce: bool,

    pub hp_ilimit_clear_work: delayed_work,
    pub hp_ilimited: bool,
    pub hp_ilimit_count: ::core::ffi::c_int,

    pub kctl: [*mut snd_kcontrol; 7],
}

/* C conditional: #if IS_REACHABLE(CONFIG_SND_SOC_CS42L43_SDW) */
#[cfg(CONFIG_SND_SOC_CS42L43_SDW_REACHABLE)]
unsafe extern "C" {
    pub fn cs42l43_sdw_add_peripheral(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        dai: *mut snd_soc_dai,
    ) -> ::core::ffi::c_int;
    pub fn cs42l43_sdw_remove_peripheral(
        substream: *mut snd_pcm_substream,
        dai: *mut snd_soc_dai,
    ) -> ::core::ffi::c_int;
    pub fn cs42l43_sdw_set_stream(
        dai: *mut snd_soc_dai,
        sdw_stream: *mut ::core::ffi::c_void,
        direction: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

/* C conditional: #else of IS_REACHABLE(CONFIG_SND_SOC_CS42L43_SDW) */
#[cfg(not(CONFIG_SND_SOC_CS42L43_SDW_REACHABLE))]
#[inline]
pub unsafe extern "C" fn cs42l43_sdw_add_peripheral(
    _substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
    _dai: *mut snd_soc_dai,
) -> ::core::ffi::c_int {
    -EINVAL
}

#[cfg(not(CONFIG_SND_SOC_CS42L43_SDW_REACHABLE))]
pub const cs42l43_sdw_remove_peripheral: Option<
    unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> ::core::ffi::c_int,
> = None;

#[cfg(not(CONFIG_SND_SOC_CS42L43_SDW_REACHABLE))]
pub const cs42l43_sdw_set_stream: Option<
    unsafe extern "C" fn(
        *mut snd_soc_dai,
        *mut ::core::ffi::c_void,
        ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
> = None;

unsafe extern "C" {
    pub fn cs42l43_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        d: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    pub fn cs42l43_bias_sense_timeout(work: *mut work_struct);
    pub fn cs42l43_clear_jack(priv_: *mut cs42l43_codec);
    pub fn cs42l43_tip_sense_work(work: *mut work_struct);
    pub fn cs42l43_bias_detect_clamp(
        irq: ::core::ffi::c_int,
        data: *mut ::core::ffi::c_void,
    ) -> irqreturn_t;
    pub fn cs42l43_button_press(
        irq: ::core::ffi::c_int,
        data: *mut ::core::ffi::c_void,
    ) -> irqreturn_t;
    pub fn cs42l43_button_release(
        irq: ::core::ffi::c_int,
        data: *mut ::core::ffi::c_void,
    ) -> irqreturn_t;
    pub fn cs42l43_tip_sense(
        irq: ::core::ffi::c_int,
        data: *mut ::core::ffi::c_void,
    ) -> irqreturn_t;
    pub fn cs42l43_jack_get(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> ::core::ffi::c_int;
    pub fn cs42l43_jack_put(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> ::core::ffi::c_int;

    pub static cs42l43_jack_enum: soc_enum;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
