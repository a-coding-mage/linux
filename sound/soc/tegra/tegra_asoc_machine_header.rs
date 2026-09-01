/* SPDX-License-Identifier: GPL-2.0-only */

use core::ffi::{c_char, c_int, c_uint};

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack_gpio {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tegra_asoc_data {
    pub mclk_rate: Option<unsafe extern "C" fn(srate: c_uint) -> c_uint>,
    pub codec_dev_name: *const c_char,
    pub hp_jack_name: *const c_char,
    pub card: *mut snd_soc_card,
    pub mclk_id: c_uint,
    pub hp_jack_gpio_active_low: bool,
    pub add_common_dapm_widgets: bool,
    pub add_common_controls: bool,
    pub add_common_snd_ops: bool,
    pub add_headset_jack: bool,
    pub add_mic_jack: bool,
    pub add_hp_jack: bool,
    pub set_ac97: bool,
}

#[repr(C)]
pub struct tegra_machine {
    pub clk_pll_a_out0: *mut clk,
    pub clk_pll_a: *mut clk,
    pub clk_cdev1: *mut clk,
    pub set_baseclock: c_uint,
    pub set_mclk: c_uint,
    pub asoc: *const tegra_asoc_data,
    pub gpiod_ext_mic_en: *mut gpio_desc,
    pub gpiod_int_mic_en: *mut gpio_desc,
    pub gpiod_spkr_en: *mut gpio_desc,
    pub gpiod_mic_det: *mut gpio_desc,
    pub gpiod_ear_sel: *mut gpio_desc,
    pub gpiod_hp_mute: *mut gpio_desc,
    pub gpiod_hp_det: *mut gpio_desc,
    pub mic_jack: *mut snd_soc_jack,
    pub hp_jack_gpio: *mut snd_soc_jack_gpio,
}

unsafe extern "C" {
    pub fn tegra_asoc_machine_probe(pdev: *mut platform_device) -> c_int;
    pub fn tegra_asoc_machine_init(rtd: *mut snd_soc_pcm_runtime) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
