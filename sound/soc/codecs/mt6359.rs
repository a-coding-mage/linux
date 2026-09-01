// SPDX-License-Identifier: GPL-2.0
//
// mt6359.c  --  mt6359 ALSA SoC audio codec driver
//
// Copyright (c) 2020 MediaTek Inc.
// Author: KaiChieh Chuang <kaichieh.chuang@mediatek.com>
//
// Rust source-level translation of ./mt6359.c.  Kernel/ASoC declarations,
// constants, structures, and macros referenced here are expected from the
// translated dependencies corresponding to the original C includes and
// "mt6359.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

extern "C" {
    fn regmap_update_bits(regmap: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_write(regmap: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(regmap: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut mt6359_priv;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_put_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_get_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_volsw_is_stereo(mc: *mut soc_mixer_control) -> bool;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_dapm_kcontrol_get_value(kcontrol: *mut snd_kcontrol) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_component_init_regmap(cmpnt: *mut snd_soc_component, regmap: *mut regmap);
    fn of_get_child_by_name(node: *mut device_node, name: *const c_char) -> *mut device_node;
    fn of_property_read_u32(np: *mut device_node, prop: *const c_char, out: *mut c_uint) -> c_int;
    fn of_node_put(np: *mut device_node);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_driver: *mut snd_soc_dai_driver,
        num_dai: usize,
    ) -> c_int;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }

#[repr(C)]
pub struct device {
    pub parent: *mut device,
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct mt6397_chip {
    pub regmap: *mut regmap,
}

#[repr(C)]
pub struct snd_soc_component {
    pub regmap: *mut regmap,
}

#[repr(C)]
pub struct snd_kcontrol_id {
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub id: snd_kcontrol_id,
    pub private_value: usize,
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_int; 128],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: core::mem::ManuallyDrop<snd_ctl_elem_value_integer>,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct soc_mixer_control {
    pub reg: c_uint,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
    pub kcontrols: *mut *mut snd_kcontrol,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
    pub number: c_int,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub id: c_int,
    pub component: *mut snd_soc_component,
}

#[repr(C)] pub struct snd_soc_dai_ops { pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>, pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>, pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)> }
#[repr(C)] pub struct snd_soc_pcm_stream { pub stream_name: *const c_char, pub channels_min: c_uint, pub channels_max: c_uint, pub rates: c_uint, pub formats: c_uint }
#[repr(C)] pub struct snd_soc_dai_driver { pub id: c_int, pub name: *const c_char, pub playback: snd_soc_pcm_stream, pub capture: snd_soc_pcm_stream, pub ops: *const snd_soc_dai_ops }
#[repr(C)] pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget_desc { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_route { pub sink: *const c_char, pub control: *const c_char, pub source: *const c_char, pub connected: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_soc_dapm_widget) -> c_int> }
#[repr(C)] pub struct snd_soc_component_driver { pub name: *const c_char, pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>, pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>, pub controls: *const snd_kcontrol_new, pub num_controls: usize, pub dapm_widgets: *const snd_soc_dapm_widget_desc, pub num_dapm_widgets: usize, pub dapm_routes: *const snd_soc_dapm_route, pub num_dapm_routes: usize, pub endianness: c_uint }
#[repr(C)] pub struct device_driver { pub name: *const c_char }
#[repr(C)] pub struct platform_driver { pub driver: device_driver, pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int> }

#[repr(C)]
pub struct mt6359_priv {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub mtkaif_protocol: c_int,
    pub hp_hifi_mode: c_int,
    pub dmic_one_wire_mode: c_uint,
    pub ana_gain: [c_int; 32],
    pub dl_rate: [c_uint; 8],
    pub ul_rate: [c_uint; 8],
    pub dev_counter: [c_int; 8],
    pub mux_select: [c_uint; 16],
}

macro_rules! cstr { ($s:literal) => { concat!($s, "\0").as_ptr() as *const c_char }; }
macro_rules! ARRAY_SIZE { ($a:expr) => { $a.len() }; }
macro_rules! EXPORT_SYMBOL_GPL { ($name:ident) => {}; }
macro_rules! module_platform_driver { ($driver:ident) => {}; }
macro_rules! MODULE_DESCRIPTION { ($s:literal) => {}; }
macro_rules! MODULE_AUTHOR { ($s:literal) => {}; }
macro_rules! MODULE_LICENSE { ($s:literal) => {}; }

extern "C" {
    static CODEC_MT6359_NAME: c_char;
}

unsafe fn IS_DCC_BASE(mic_type: c_uint) -> bool {
    mic_type == MIC_TYPE_MUX_DCC_ECM_DIFF as c_uint || mic_type == MIC_TYPE_MUX_DCC_ECM_SINGLE as c_uint
}

unsafe extern "C" fn mt6359_set_gpio_smt(priv_: *mut mt6359_priv) {
    /* set gpio SMT mode */
    regmap_update_bits((*priv_).regmap, MT6359_SMT_CON1, 0x3ff0, 0x3ff0);
}

unsafe extern "C" fn mt6359_set_gpio_driving(priv_: *mut mt6359_priv) {
    /* 8:4mA(default), a:8mA, c:12mA, e:16mA */
    regmap_update_bits((*priv_).regmap, MT6359_DRV_CON2, 0xffff, 0x8888);
    regmap_update_bits((*priv_).regmap, MT6359_DRV_CON3, 0xffff, 0x8888);
    regmap_update_bits((*priv_).regmap, MT6359_DRV_CON4, 0x00ff, 0x88);
}

unsafe extern "C" fn mt6359_set_playback_gpio(priv_: *mut mt6359_priv) {
    /* set gpio mosi mode, clk / data mosi */
    regmap_write((*priv_).regmap, MT6359_GPIO_MODE2_CLR, 0x0ffe);
    regmap_write((*priv_).regmap, MT6359_GPIO_MODE2_SET, 0x0249);
    /* sync mosi */
    regmap_write((*priv_).regmap, MT6359_GPIO_MODE3_CLR, 0x6);
    regmap_write((*priv_).regmap, MT6359_GPIO_MODE3_SET, 0x1);
}

unsafe extern "C" fn mt6359_reset_playback_gpio(priv_: *mut mt6359_priv) {
    /* set pad_aud_*_mosi to GPIO mode and dir input
     * reason:
     * pad_aud_dat_mosi*, because the pin is used as boot strap
     * don't clean clk/sync, for mtkaif protocol 2
     */
    regmap_write((*priv_).regmap, MT6359_GPIO_MODE2_CLR, 0x0ff8);
    regmap_update_bits((*priv_).regmap, MT6359_GPIO_DIR0, 0x7 << 9, 0x0);
}

unsafe extern "C" fn mt6359_set_capture_gpio(priv_: *mut mt6359_priv) {
    /* set gpio miso mode */
    regmap_write((*priv_).regmap, MT6359_GPIO_MODE3_CLR, 0x0e00);
    regmap_write((*priv_).regmap, MT6359_GPIO_MODE3_SET, 0x0200);
    regmap_write((*priv_).regmap, MT6359_GPIO_MODE4_CLR, 0x003f);
    regmap_write((*priv_).regmap, MT6359_GPIO_MODE4_SET, 0x0009);
}

unsafe extern "C" fn mt6359_reset_capture_gpio(priv_: *mut mt6359_priv) {
    /* set pad_aud_*_miso to GPIO mode and dir input
     * reason:
     * pad_aud_clk_miso, because when playback only the miso_clk
     * will also have 26m, so will have power leak
     * pad_aud_dat_miso*, because the pin is used as boot strap
     */
    regmap_write((*priv_).regmap, MT6359_GPIO_MODE3_CLR, 0x0e00);
    regmap_write((*priv_).regmap, MT6359_GPIO_MODE4_CLR, 0x003f);
    regmap_update_bits((*priv_).regmap, MT6359_GPIO_DIR0, 0x7 << 13, 0x0);
    regmap_update_bits((*priv_).regmap, MT6359_GPIO_DIR1, 0x3 << 0, 0x0);
}

/* use only when doing mtkaif calibraiton at the boot time */
unsafe extern "C" fn mt6359_set_dcxo(priv_: *mut mt6359_priv, enable: bool) {
    regmap_update_bits((*priv_).regmap, MT6359_DCXO_CW12, 0x1 << RG_XO_AUDIO_EN_M_SFT, ((if enable { 1 } else { 0 }) << RG_XO_AUDIO_EN_M_SFT) as c_uint);
}

/* use only when doing mtkaif calibraiton at the boot time */
unsafe extern "C" fn mt6359_set_clksq(priv_: *mut mt6359_priv, enable: bool) {
    /* Enable/disable CLKSQ 26MHz */
    regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON23, RG_CLKSQ_EN_MASK_SFT, ((if enable { 1 } else { 0 }) << RG_CLKSQ_EN_SFT) as c_uint);
}

/* use only when doing mtkaif calibraiton at the boot time */
unsafe extern "C" fn mt6359_set_aud_global_bias(priv_: *mut mt6359_priv, enable: bool) {
    regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON13, RG_AUDGLB_PWRDN_VA32_MASK_SFT, ((if enable { 0 } else { 1 }) << RG_AUDGLB_PWRDN_VA32_SFT) as c_uint);
}

/* use only when doing mtkaif calibraiton at the boot time */
unsafe extern "C" fn mt6359_set_topck(priv_: *mut mt6359_priv, enable: bool) {
    regmap_update_bits((*priv_).regmap, MT6359_AUD_TOP_CKPDN_CON0, 0x0066, if enable { 0x0 } else { 0x66 });
}

unsafe extern "C" fn mt6359_set_decoder_clk(priv_: *mut mt6359_priv, enable: bool) {
    regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON13, RG_RSTB_DECODER_VA32_MASK_SFT, ((if enable { 1 } else { 0 }) << RG_RSTB_DECODER_VA32_SFT) as c_uint);
}

unsafe extern "C" fn mt6359_mtkaif_tx_enable(priv_: *mut mt6359_priv) {
    match (*priv_).mtkaif_protocol {
        MT6359_MTKAIF_PROTOCOL_2_CLK_P2 => {
            /* MTKAIF TX format setting */
            regmap_update_bits((*priv_).regmap, MT6359_AFE_ADDA_MTKAIF_CFG0, 0xffff, 0x0210);
            /* enable aud_pad TX fifos */
            regmap_update_bits((*priv_).regmap, MT6359_AFE_AUD_PAD_TOP, 0xff00, 0x3800);
            regmap_update_bits((*priv_).regmap, MT6359_AFE_AUD_PAD_TOP, 0xff00, 0x3900);
        }
        MT6359_MTKAIF_PROTOCOL_2 => {
            regmap_update_bits((*priv_).regmap, MT6359_AFE_ADDA_MTKAIF_CFG0, 0xffff, 0x0210);
            regmap_update_bits((*priv_).regmap, MT6359_AFE_AUD_PAD_TOP, 0xff00, 0x3100);
        }
        _ => {
            regmap_update_bits((*priv_).regmap, MT6359_AFE_ADDA_MTKAIF_CFG0, 0xffff, 0x0000);
            regmap_update_bits((*priv_).regmap, MT6359_AFE_AUD_PAD_TOP, 0xff00, 0x3100);
        }
    }
}

unsafe extern "C" fn mt6359_mtkaif_tx_disable(priv_: *mut mt6359_priv) {
    /* disable aud_pad TX fifos */
    regmap_update_bits((*priv_).regmap, MT6359_AFE_AUD_PAD_TOP, 0xff00, 0x3000);
}

#[no_mangle]
pub unsafe extern "C" fn mt6359_set_mtkaif_protocol(cmpnt: *mut snd_soc_component, mtkaif_protocol: c_int) {
    let priv_ = snd_soc_component_get_drvdata(cmpnt);
    (*priv_).mtkaif_protocol = mtkaif_protocol;
}
EXPORT_SYMBOL_GPL!(mt6359_set_mtkaif_protocol);

#[no_mangle]
pub unsafe extern "C" fn mt6359_mtkaif_calibration_enable(cmpnt: *mut snd_soc_component) {
    let priv_ = snd_soc_component_get_drvdata(cmpnt);
    mt6359_set_playback_gpio(priv_);
    mt6359_set_capture_gpio(priv_);
    mt6359_mtkaif_tx_enable(priv_);
    mt6359_set_dcxo(priv_, true);
    mt6359_set_aud_global_bias(priv_, true);
    mt6359_set_clksq(priv_, true);
    mt6359_set_topck(priv_, true);
    /* set dat_miso_loopback on */
    regmap_update_bits((*priv_).regmap, MT6359_AUDIO_DIG_CFG, RG_AUD_PAD_TOP_DAT_MISO2_LOOPBACK_MASK_SFT, 1 << RG_AUD_PAD_TOP_DAT_MISO2_LOOPBACK_SFT);
    regmap_update_bits((*priv_).regmap, MT6359_AUDIO_DIG_CFG, RG_AUD_PAD_TOP_DAT_MISO_LOOPBACK_MASK_SFT, 1 << RG_AUD_PAD_TOP_DAT_MISO_LOOPBACK_SFT);
    regmap_update_bits((*priv_).regmap, MT6359_AUDIO_DIG_CFG1, RG_AUD_PAD_TOP_DAT_MISO3_LOOPBACK_MASK_SFT, 1 << RG_AUD_PAD_TOP_DAT_MISO3_LOOPBACK_SFT);
}
EXPORT_SYMBOL_GPL!(mt6359_mtkaif_calibration_enable);

#[no_mangle]
pub unsafe extern "C" fn mt6359_mtkaif_calibration_disable(cmpnt: *mut snd_soc_component) {
    let priv_ = snd_soc_component_get_drvdata(cmpnt);
    /* set dat_miso_loopback off */
    regmap_update_bits((*priv_).regmap, MT6359_AUDIO_DIG_CFG, RG_AUD_PAD_TOP_DAT_MISO2_LOOPBACK_MASK_SFT, 0 << RG_AUD_PAD_TOP_DAT_MISO2_LOOPBACK_SFT);
    regmap_update_bits((*priv_).regmap, MT6359_AUDIO_DIG_CFG, RG_AUD_PAD_TOP_DAT_MISO_LOOPBACK_MASK_SFT, 0 << RG_AUD_PAD_TOP_DAT_MISO_LOOPBACK_SFT);
    regmap_update_bits((*priv_).regmap, MT6359_AUDIO_DIG_CFG1, RG_AUD_PAD_TOP_DAT_MISO3_LOOPBACK_MASK_SFT, 0 << RG_AUD_PAD_TOP_DAT_MISO3_LOOPBACK_SFT);
    mt6359_set_topck(priv_, false);
    mt6359_set_clksq(priv_, false);
    mt6359_set_aud_global_bias(priv_, false);
    mt6359_set_dcxo(priv_, false);
    mt6359_mtkaif_tx_disable(priv_);
    mt6359_reset_playback_gpio(priv_);
    mt6359_reset_capture_gpio(priv_);
}
EXPORT_SYMBOL_GPL!(mt6359_mtkaif_calibration_disable);

#[no_mangle]
pub unsafe extern "C" fn mt6359_set_mtkaif_calibration_phase(cmpnt: *mut snd_soc_component, phase_1: c_int, phase_2: c_int, phase_3: c_int) {
    let priv_ = snd_soc_component_get_drvdata(cmpnt);
    regmap_update_bits((*priv_).regmap, MT6359_AUDIO_DIG_CFG, RG_AUD_PAD_TOP_PHASE_MODE_MASK_SFT, (phase_1 << RG_AUD_PAD_TOP_PHASE_MODE_SFT) as c_uint);
    regmap_update_bits((*priv_).regmap, MT6359_AUDIO_DIG_CFG, RG_AUD_PAD_TOP_PHASE_MODE2_MASK_SFT, (phase_2 << RG_AUD_PAD_TOP_PHASE_MODE2_SFT) as c_uint);
    regmap_update_bits((*priv_).regmap, MT6359_AUDIO_DIG_CFG1, RG_AUD_PAD_TOP_PHASE_MODE3_MASK_SFT, (phase_3 << RG_AUD_PAD_TOP_PHASE_MODE3_SFT) as c_uint);
}
EXPORT_SYMBOL_GPL!(mt6359_set_mtkaif_calibration_phase);

unsafe extern "C" fn zcd_disable(priv_: *mut mt6359_priv) {
    regmap_write((*priv_).regmap, MT6359_ZCD_CON0, 0x0000);
}

unsafe extern "C" fn hp_main_output_ramp(priv_: *mut mt6359_priv, up: bool) {
    let target: c_int = 7;
    let mut i: c_int = 0;
    while i <= target {
        let stage = if up { i } else { target - i };
        regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON1, RG_HPLOUTSTGCTRL_VAUDP32_MASK_SFT, (stage << RG_HPLOUTSTGCTRL_VAUDP32_SFT) as c_uint);
        regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON1, RG_HPROUTSTGCTRL_VAUDP32_MASK_SFT, (stage << RG_HPROUTSTGCTRL_VAUDP32_SFT) as c_uint);
        usleep_range(600, 650);
        i += 1;
    }
}

unsafe extern "C" fn hp_aux_feedback_loop_gain_ramp(priv_: *mut mt6359_priv, up: bool) {
    let target: c_int = 0xf;
    let mut i: c_int = 0;
    while i <= target {
        let stage = if up { i } else { target - i };
        regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON9, 0xf << 12, (stage << 12) as c_uint);
        usleep_range(600, 650);
        i += 1;
    }
}

unsafe extern "C" fn hp_in_pair_current(priv_: *mut mt6359_priv, increase: bool) {
    let target: c_int = 0x3;
    if (*priv_).hp_hifi_mode != 0 {
        let mut i: c_int = 0;
        while i <= target {
            let stage = if increase { i } else { target - i };
            regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON10, 0x3 << 3, (stage << 3) as c_uint);
            usleep_range(100, 150);
            i += 1;
        }
    }
}

unsafe extern "C" fn hp_pull_down(priv_: *mut mt6359_priv, enable: bool) {
    if enable {
        let mut i: c_int = 0;
        while i <= 0x7 {
            regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON2, RG_HPPSHORT2VCM_VAUDP32_MASK_SFT, (i << RG_HPPSHORT2VCM_VAUDP32_SFT) as c_uint);
            usleep_range(100, 150);
            i += 1;
        }
    } else {
        let mut i: c_int = 0x7;
        loop {
            regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON2, RG_HPPSHORT2VCM_VAUDP32_MASK_SFT, (i << RG_HPPSHORT2VCM_VAUDP32_SFT) as c_uint);
            usleep_range(100, 150);
            if i == 0 { break; }
            i -= 1;
        }
    }
}

unsafe extern "C" fn is_valid_hp_pga_idx(reg_idx: c_int) -> bool {
    (reg_idx >= DL_GAIN_8DB && reg_idx <= DL_GAIN_N_22DB) || reg_idx == DL_GAIN_N_40DB
}

unsafe extern "C" fn headset_volume_ramp(priv_: *mut mt6359_priv, from: c_int, to: c_int) {
    let mut offset: c_int = 0;
    let mut count: c_int = 1;
    let mut reg_idx: c_int;
    if !is_valid_hp_pga_idx(from) || !is_valid_hp_pga_idx(to) {
        dev_warn((*priv_).dev, cstr!("%s(), volume index is not valid, from %d, to %d\n"), cstr!("headset_volume_ramp"), from, to);
        return;
    }
    dev_dbg((*priv_).dev, cstr!("%s(), from %d, to %d\n"), cstr!("headset_volume_ramp"), from, to);
    if to > from { offset = to - from; } else { offset = from - to; }
    while offset > 0 {
        if to > from { reg_idx = from + count; } else { reg_idx = from - count; }
        if is_valid_hp_pga_idx(reg_idx) {
            regmap_update_bits((*priv_).regmap, MT6359_ZCD_CON2, DL_GAIN_REG_MASK, ((reg_idx << 7) | reg_idx) as c_uint);
            usleep_range(600, 650);
        }
        offset -= 1;
        count += 1;
    }
}

unsafe extern "C" fn mt6359_put_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let priv_ = snd_soc_component_get_drvdata(component);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let mut reg: c_uint = 0;
    let index = (*(*ucontrol).value.integer).value[0];
    let mut orig_gain = [0 as c_int; 2];
    let mut new_gain = [0 as c_int; 2];
    match (*mc).reg {
        MT6359_ZCD_CON2 => { orig_gain[0] = (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_HPOUTL as usize]; orig_gain[1] = (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_HPOUTR as usize]; }
        MT6359_ZCD_CON1 => { orig_gain[0] = (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_LINEOUTL as usize]; orig_gain[1] = (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_LINEOUTR as usize]; }
        MT6359_ZCD_CON3 => { orig_gain[0] = (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_HSOUTL as usize]; }
        MT6359_AUDENC_ANA_CON0 => { orig_gain[0] = (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_MICAMP1 as usize]; }
        MT6359_AUDENC_ANA_CON1 => { orig_gain[0] = (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_MICAMP2 as usize]; }
        MT6359_AUDENC_ANA_CON2 => { orig_gain[0] = (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_MICAMP3 as usize]; }
        _ => return -EINVAL,
    }
    let mut ret = snd_soc_put_volsw(kcontrol, ucontrol);
    if ret < 0 { return ret; }
    match (*mc).reg {
        MT6359_ZCD_CON2 => {
            regmap_read((*priv_).regmap, MT6359_ZCD_CON2, &mut reg);
            (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_HPOUTL as usize] = ((reg >> RG_AUDHPLGAIN_SFT) & RG_AUDHPLGAIN_MASK) as c_int;
            (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_HPOUTR as usize] = ((reg >> RG_AUDHPRGAIN_SFT) & RG_AUDHPRGAIN_MASK) as c_int;
            new_gain[0] = (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_HPOUTL as usize]; new_gain[1] = (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_HPOUTR as usize];
        }
        MT6359_ZCD_CON1 => {
            regmap_read((*priv_).regmap, MT6359_ZCD_CON1, &mut reg);
            (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_LINEOUTL as usize] = ((reg >> RG_AUDLOLGAIN_SFT) & RG_AUDLOLGAIN_MASK) as c_int;
            (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_LINEOUTR as usize] = ((reg >> RG_AUDLORGAIN_SFT) & RG_AUDLORGAIN_MASK) as c_int;
            new_gain[0] = (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_LINEOUTL as usize]; new_gain[1] = (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_LINEOUTR as usize];
        }
        MT6359_ZCD_CON3 => {
            regmap_read((*priv_).regmap, MT6359_ZCD_CON3, &mut reg);
            (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_HSOUTL as usize] = ((reg >> RG_AUDHSGAIN_SFT) & RG_AUDHSGAIN_MASK) as c_int;
            new_gain[0] = (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_HSOUTL as usize];
        }
        MT6359_AUDENC_ANA_CON0 => {
            regmap_read((*priv_).regmap, MT6359_AUDENC_ANA_CON0, &mut reg);
            (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_MICAMP1 as usize] = ((reg >> RG_AUDPREAMPLGAIN_SFT) & RG_AUDPREAMPLGAIN_MASK) as c_int;
            new_gain[0] = (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_MICAMP1 as usize];
        }
        MT6359_AUDENC_ANA_CON1 => {
            regmap_read((*priv_).regmap, MT6359_AUDENC_ANA_CON1, &mut reg);
            (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_MICAMP2 as usize] = ((reg >> RG_AUDPREAMPRGAIN_SFT) & RG_AUDPREAMPRGAIN_MASK) as c_int;
            new_gain[0] = (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_MICAMP2 as usize];
        }
        MT6359_AUDENC_ANA_CON2 => {
            regmap_read((*priv_).regmap, MT6359_AUDENC_ANA_CON2, &mut reg);
            (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_MICAMP3 as usize] = ((reg >> RG_AUDPREAMP3GAIN_SFT) & RG_AUDPREAMP3GAIN_MASK) as c_int;
            new_gain[0] = (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_MICAMP3 as usize];
        }
        _ => {}
    }
    ret = 0;
    if orig_gain[0] != new_gain[0] { ret = 1; } else if snd_soc_volsw_is_stereo(mc) && orig_gain[1] != new_gain[1] { ret = 1; }
    dev_dbg((*priv_).dev, cstr!("%s(), name %s, reg(0x%x) = 0x%x, set index = %x\n"), cstr!("mt6359_put_volsw"), (*kcontrol).id.name, (*mc).reg, reg, index);
    ret
}

unsafe extern "C" fn mt6359_get_playback_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let priv_ = snd_soc_component_get_drvdata(component);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    match (*mc).reg {
        MT6359_ZCD_CON2 => { (*(*ucontrol).value.integer).value[0] = (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_HPOUTL as usize]; (*(*ucontrol).value.integer).value[1] = (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_HPOUTR as usize]; }
        MT6359_ZCD_CON1 => { (*(*ucontrol).value.integer).value[0] = (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_LINEOUTL as usize]; (*(*ucontrol).value.integer).value[1] = (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_LINEOUTR as usize]; }
        MT6359_ZCD_CON3 => { (*(*ucontrol).value.integer).value[0] = (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_HSOUTL as usize]; }
        _ => return -EINVAL,
    }
    0
}

/* MUX tables and ALSA macro declarations from the C source are preserved here as
 * Rust-side static data plus dependency macro intent.  The concrete
 * snd_kcontrol_new/snd_soc_enum layouts are supplied by ASoC dependencies.
 */
static lo_in_mux_map: [&[u8]; 4] = [b"Open\0", b"Playback_L_DAC\0", b"Playback\0", b"Test Mode\0"];
static hp_in_mux_map: [&[u8]; 5] = [b"Open\0", b"LoudSPK Playback\0", b"Audio Playback\0", b"Test Mode\0", b"HP Impedance\0"];
static rcv_in_mux_map: [&[u8]; 4] = [b"Open\0", b"Mute\0", b"Voice Playback\0", b"Test Mode\0"];
static dac_in_mux_map: [&[u8]; 2] = [b"Normal Path\0", b"Sgen\0"];
static mut dac_in_mux_map_value: [c_int; 2] = [0x0, 0x1];
static ul_src_mux_map: [&[u8]; 2] = [b"AMIC\0", b"DMIC\0"];
static mut ul_src_mux_map_value: [c_int; 2] = [UL_SRC_MUX_AMIC, UL_SRC_MUX_DMIC];
static miso_mux_map: [&[u8]; 4] = [b"UL1_CH1\0", b"UL1_CH2\0", b"UL2_CH1\0", b"UL2_CH2\0"];
static mut miso_mux_map_value: [c_int; 4] = [MISO_MUX_UL1_CH1, MISO_MUX_UL1_CH2, MISO_MUX_UL2_CH1, MISO_MUX_UL2_CH2];
static dmic_mux_map: [&[u8]; 4] = [b"DMIC_DATA0\0", b"DMIC_DATA1_L\0", b"DMIC_DATA1_L_1\0", b"DMIC_DATA1_R\0"];
static mut dmic_mux_map_value: [c_int; 4] = [DMIC_MUX_DMIC_DATA0, DMIC_MUX_DMIC_DATA1_L, DMIC_MUX_DMIC_DATA1_L_1, DMIC_MUX_DMIC_DATA1_R];
static adc_left_mux_map: [&[u8]; 4] = [b"Idle\0", b"AIN0\0", b"Left Preamplifier\0", b"Idle_1\0"];
static adc_right_mux_map: [&[u8]; 4] = [b"Idle\0", b"AIN0\0", b"Right Preamplifier\0", b"Idle_1\0"];
static adc_3_mux_map: [&[u8]; 4] = [b"Idle\0", b"AIN0\0", b"Preamplifier\0", b"Idle_1\0"];
static mut adc_mux_map_value: [c_int; 4] = [ADC_MUX_IDLE, ADC_MUX_AIN0, ADC_MUX_PREAMPLIFIER, ADC_MUX_IDLE1];
static pga_l_mux_map: [&[u8]; 3] = [b"None\0", b"AIN0\0", b"AIN1\0"];
static mut pga_l_mux_map_value: [c_int; 3] = [PGA_L_MUX_NONE, PGA_L_MUX_AIN0, PGA_L_MUX_AIN1];
static pga_r_mux_map: [&[u8]; 4] = [b"None\0", b"AIN2\0", b"AIN3\0", b"AIN0\0"];
static mut pga_r_mux_map_value: [c_int; 4] = [PGA_R_MUX_NONE, PGA_R_MUX_AIN2, PGA_R_MUX_AIN3, PGA_R_MUX_AIN0];
static pga_3_mux_map: [&[u8]; 3] = [b"None\0", b"AIN3\0", b"AIN2\0"];
static mut pga_3_mux_map_value: [c_int; 3] = [PGA_3_MUX_NONE, PGA_3_MUX_AIN3, PGA_3_MUX_AIN2];

unsafe extern "C" fn mt_sgen_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt);
    dev_dbg((*priv_).dev, cstr!("%s(), event = 0x%x\n"), cstr!("mt_sgen_event"), event);
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            regmap_write((*priv_).regmap, MT6359_AFUNC_AUD_CON2, 0x0006);
            regmap_write((*priv_).regmap, MT6359_AFUNC_AUD_CON0, 0xcba1);
            regmap_write((*priv_).regmap, MT6359_AFUNC_AUD_CON2, 0x0003);
            regmap_write((*priv_).regmap, MT6359_AFUNC_AUD_CON2, 0x000b);
            regmap_update_bits((*priv_).regmap, MT6359_AFE_SGEN_CFG0, 0xff3f, 0x0000);
            regmap_update_bits((*priv_).regmap, MT6359_AFE_SGEN_CFG1, 0xffff, 0x0001);
        }
        SND_SOC_DAPM_POST_PMD => {
            regmap_write((*priv_).regmap, MT6359_AFUNC_AUD_CON2, 0x0000);
            regmap_write((*priv_).regmap, MT6359_AFUNC_AUD_CON0, 0xcba0);
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn mtk_hp_enable(priv_: *mut mt6359_priv) {
    if (*priv_).hp_hifi_mode != 0 {
        regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON11, DRBIAS_HP_MASK_SFT, DRBIAS_6UA << DRBIAS_HP_SFT);
        regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON12, IBIAS_ZCD_MASK_SFT, IBIAS_ZCD_4UA << IBIAS_ZCD_SFT);
        regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON12, IBIAS_HP_MASK_SFT, IBIAS_5UA << IBIAS_HP_SFT);
    } else {
        regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON11, DRBIAS_HP_MASK_SFT, DRBIAS_5UA << DRBIAS_HP_SFT);
        regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON12, IBIAS_ZCD_MASK_SFT, IBIAS_ZCD_3UA << IBIAS_ZCD_SFT);
        regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON12, IBIAS_HP_MASK_SFT, IBIAS_4UA << IBIAS_HP_SFT);
    }
    regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON10, 0x0087);
    if (*priv_).dl_rate[MT6359_AIF_1 as usize] >= 96000 {
        regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON4, RG_AUDHPHFCOMPBUFGAINSEL_VAUDP32_MASK_SFT, 0x1 << RG_AUDHPHFCOMPBUFGAINSEL_VAUDP32_SFT);
    } else {
        regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON4, 0x0000);
    }
    regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON2, 0xf133);
    regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON1, 0x000c);
    regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON1, 0x003c);
    regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON9, 0x0c00);
    regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON0, 0x30c0);
    regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON0, 0x30f0);
    regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON1, 0x00fc);
    hp_in_pair_current(priv_, true);
    regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON9, 0x0e00);
    regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON9, 0x0200);
    regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON1, 0x00ff);
    hp_main_output_ramp(priv_, true);
    hp_aux_feedback_loop_gain_ramp(priv_, true);
    regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON1, 0x77cf);
    headset_volume_ramp(priv_, DL_GAIN_N_22DB, (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_HPOUTL as usize]);
    regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON1, 0x77c3);
    regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON1, 0x7703);
    usleep_range(100, 120);
    mt6359_set_decoder_clk(priv_, true);
    regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON0, 0x30ff);
    if (*priv_).hp_hifi_mode != 0 { regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON9, 0xf201); } else { regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON9, 0xf200); }
    usleep_range(100, 120);
    regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON0, 0x32ff);
    regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON0, 0x3aff);
    hp_pull_down(priv_, false);
}

unsafe extern "C" fn mtk_hp_disable(priv_: *mut mt6359_priv) {
    hp_pull_down(priv_, true);
    regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON0, 0x0f00, 0x0000);
    regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON9, 0x0001, 0x0000);
    regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON0, 0x000f, 0x0000);
    mt6359_set_decoder_clk(priv_, false);
    regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON1, 0x77c3);
    regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON1, 0x77cf);
    headset_volume_ramp(priv_, (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_HPOUTL as usize], DL_GAIN_N_22DB);
    regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON1, 0x77ff);
    hp_aux_feedback_loop_gain_ramp(priv_, false);
    hp_main_output_ramp(priv_, false);
    regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON1, 0x3, 0x0);
    regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON9, 0x0e01);
    regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON9, 0x0c01);
    hp_in_pair_current(priv_, false);
    regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON1, 0x3 << 6, 0x0);
    regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON0, 0x3 << 4, 0x0);
    regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON0, 0x3 << 6, 0x0);
    regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON9, 0x201);
    regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON1, 0x3 << 4, 0x0);
    regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON1, 0x3 << 2, 0x0);
}

unsafe extern "C" fn mt_hp_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt);
    let mux = snd_soc_dapm_kcontrol_get_value(*(*w).kcontrols);
    let device = DEVICE_HP;
    dev_dbg((*priv_).dev, cstr!("%s(), event 0x%x, dev_counter[DEV_HP] %d, mux %u\n"), cstr!("mt_hp_event"), event, (*priv_).dev_counter[device as usize], mux);
    match event {
        SND_SOC_DAPM_PRE_PMU => { (*priv_).dev_counter[device as usize] += 1; if mux == HP_MUX_HP as c_uint { mtk_hp_enable(priv_); } }
        SND_SOC_DAPM_PRE_PMD => { (*priv_).dev_counter[device as usize] -= 1; if mux == HP_MUX_HP as c_uint { mtk_hp_disable(priv_); } }
        _ => {}
    }
    0
}

unsafe extern "C" fn mt_rcv_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt);
    dev_dbg((*priv_).dev, cstr!("%s(), event 0x%x, mux %u\n"), cstr!("mt_rcv_event"), event, snd_soc_dapm_kcontrol_get_value(*(*w).kcontrols));
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON6, 0x0010);
            regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON11, DRBIAS_HS_MASK_SFT, DRBIAS_6UA << DRBIAS_HS_SFT);
            regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON12, IBIAS_ZCD_MASK_SFT, IBIAS_ZCD_4UA << IBIAS_ZCD_SFT);
            regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON12, IBIAS_HS_MASK_SFT, IBIAS_5UA << IBIAS_HS_SFT);
            regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON6, 0x0090);
            regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON10, 0x7000);
            regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON6, 0x0092);
            regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON6, 0x0093);
            regmap_write((*priv_).regmap, MT6359_ZCD_CON3, (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_HSOUTL as usize] as c_uint);
            mt6359_set_decoder_clk(priv_, true);
            regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON0, 0x0009);
            regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON9, 0x0001);
            regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON6, 0x009b);
        }
        SND_SOC_DAPM_PRE_PMD => {
            regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON6, RG_AUDHSMUXINPUTSEL_VAUDP32_MASK_SFT, RCV_MUX_OPEN as c_uint);
            regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON0, 0x000f, 0x0000);
            mt6359_set_decoder_clk(priv_, false);
            regmap_write((*priv_).regmap, MT6359_ZCD_CON3, DL_GAIN_N_40DB as c_uint);
            regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON6, RG_AUDHSPWRUP_VAUDP32_MASK_SFT, 0x0);
            regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON6, RG_AUDHSPWRUP_IBIAS_VAUDP32_MASK_SFT, 0x0);
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn mt_lo_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt);
    let mux = snd_soc_dapm_kcontrol_get_value(*(*w).kcontrols);
    dev_dbg((*priv_).dev, cstr!("%s(), event 0x%x, mux %u\n"), cstr!("mt_lo_event"), event, mux);
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON7, 0x0010);
            regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON11, DRBIAS_LO_MASK_SFT, DRBIAS_6UA << DRBIAS_LO_SFT);
            if (*priv_).dev_counter[DEVICE_HP as usize] == 0 { regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON12, IBIAS_ZCD_MASK_SFT, IBIAS_ZCD_4UA << IBIAS_ZCD_SFT); }
            regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON12, IBIAS_LO_MASK_SFT, IBIAS_5UA << IBIAS_LO_SFT);
            regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON7, 0x0110);
            regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON7, 0x0112);
            regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON7, 0x0113);
            regmap_write((*priv_).regmap, MT6359_ZCD_CON1, (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_LINEOUTL as usize] as c_uint);
            mt6359_set_decoder_clk(priv_, true);
            if mux == LO_MUX_L_DAC as c_uint {
                if (*priv_).dev_counter[DEVICE_HP as usize] > 0 {
                    dev_info((*priv_).dev, cstr!("%s(), can not enable DAC, hp count %d\n"), cstr!("mt_lo_event"), (*priv_).dev_counter[DEVICE_HP as usize]);
                } else {
                    regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON0, 0x3009);
                    regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON9, 0xf200);
                    usleep_range(100, 120);
                    regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON7, 0x0117);
                }
            } else if mux == LO_MUX_3RD_DAC as c_uint {
                regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON7, 0x3113);
                if (*priv_).dev_counter[DEVICE_HP as usize] == 0 { regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON9, 0x0001); }
                regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON7, 0x311b);
            }
        }
        SND_SOC_DAPM_PRE_PMD => {
            regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON7, RG_AUDLOLMUXINPUTSEL_VAUDP32_MASK_SFT, LO_MUX_OPEN as c_uint);
            regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON0, 0x000f, 0x0000);
            if mux == LO_MUX_L_DAC as c_uint {
                regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON0, 0x3 << 4, 0x0);
                regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON0, 0x3 << 6, 0x0);
            }
            mt6359_set_decoder_clk(priv_, false);
            regmap_write((*priv_).regmap, MT6359_ZCD_CON1, DL_GAIN_N_40DB as c_uint);
            regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON7, RG_AUDLOLPWRUP_VAUDP32_MASK_SFT, 0x0);
            regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON7, RG_AUDLOLPWRUP_IBIAS_VAUDP32_MASK_SFT, 0x0);
        }
        _ => {}
    }
    0
}

macro_rules! simple_event {
    ($name:ident, $body:block) => {
        unsafe extern "C" fn $name(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int $body
    };
}

simple_event!(mt_adc_clk_gen_event, {{
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt);
    dev_dbg((*priv_).dev, cstr!("%s(), event 0x%x\n"), cstr!("mt_adc_clk_gen_event"), event);
    match event {
        SND_SOC_DAPM_POST_PMU => {
            regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON5, RG_AUDADCCLKRSTB_MASK_SFT, 0x1 << RG_AUDADCCLKRSTB_SFT);
            regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON5, RG_AUDADCCLKSOURCE_MASK_SFT, 0x0);
            regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON5, RG_AUDADCCLKSEL_MASK_SFT, 0x0);
            regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON5, RG_AUDADCCLKGENMODE_MASK_SFT, 0x1 << RG_AUDADCCLKGENMODE_SFT);
        }
        SND_SOC_DAPM_PRE_PMD => {
            regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON5, RG_AUDADCCLKSOURCE_MASK_SFT, 0x0);
            regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON5, RG_AUDADCCLKSEL_MASK_SFT, 0x0);
            regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON5, RG_AUDADCCLKGENMODE_MASK_SFT, 0x0);
            regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON5, RG_AUDADCCLKRSTB_MASK_SFT, 0x0);
        }
        _ => {}
    }
    0
}});

simple_event!(mt_dcc_clk_event, {{
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt);
    dev_dbg((*priv_).dev, cstr!("%s(), event 0x%x\n"), cstr!("mt_dcc_clk_event"), event);
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            regmap_update_bits((*priv_).regmap, MT6359_AFE_DCCLK_CFG0, 0xfff7, 0x2062);
            regmap_update_bits((*priv_).regmap, MT6359_AFE_DCCLK_CFG0, 0xfff7, 0x2060);
            regmap_update_bits((*priv_).regmap, MT6359_AFE_DCCLK_CFG0, 0xfff7, 0x2061);
            regmap_write((*priv_).regmap, MT6359_AFE_DCCLK_CFG1, 0x0100);
        }
        SND_SOC_DAPM_POST_PMD => {
            regmap_update_bits((*priv_).regmap, MT6359_AFE_DCCLK_CFG0, 0xfff7, 0x2060);
            regmap_update_bits((*priv_).regmap, MT6359_AFE_DCCLK_CFG0, 0xfff7, 0x2062);
        }
        _ => {}
    }
    0
}});

unsafe extern "C" fn mt_mic_bias_0_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm); let priv_ = snd_soc_component_get_drvdata(cmpnt); let mic_type = (*priv_).mux_select[MUX_MIC_TYPE_0 as usize];
    dev_dbg((*priv_).dev, cstr!("%s(), event 0x%x, mic_type %d\n"), cstr!("mt_mic_bias_0_event"), event, mic_type);
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            match mic_type as c_int {
                MIC_TYPE_MUX_DCC_ECM_DIFF => regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON15, 0xff00, 0x7700),
                MIC_TYPE_MUX_DCC_ECM_SINGLE => regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON15, 0xff00, 0x1100),
                _ => regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON15, 0xff00, 0x0000),
            };
            regmap_write((*priv_).regmap, MT6359_AUDENC_ANA_CON14, 0x0004);
            regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON15, RG_AUDMICBIAS0VREF_MASK_SFT, MIC_BIAS_1P9 << RG_AUDMICBIAS0VREF_SFT);
            regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON15, RG_AUDMICBIAS0LOWPEN_MASK_SFT, 0 << RG_AUDMICBIAS0LOWPEN_SFT);
        }
        SND_SOC_DAPM_POST_PMD => { regmap_write((*priv_).regmap, MT6359_AUDENC_ANA_CON15, 0x0000); }
        _ => {}
    }
    0
}

unsafe extern "C" fn mt_mic_bias_1_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm); let priv_ = snd_soc_component_get_drvdata(cmpnt); let mic_type = (*priv_).mux_select[MUX_MIC_TYPE_1 as usize];
    dev_dbg((*priv_).dev, cstr!("%s(), event 0x%x, mic_type %d\n"), cstr!("mt_mic_bias_1_event"), event, mic_type);
    if event == SND_SOC_DAPM_PRE_PMU {
        if mic_type as c_int == MIC_TYPE_MUX_DCC_ECM_SINGLE { regmap_write((*priv_).regmap, MT6359_AUDENC_ANA_CON16, 0x0160); } else { regmap_write((*priv_).regmap, MT6359_AUDENC_ANA_CON16, 0x0060); }
        regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON16, RG_AUDMICBIAS1LOWPEN_MASK_SFT, 0 << RG_AUDMICBIAS1LOWPEN_SFT);
    }
    0
}

unsafe extern "C" fn mt_mic_bias_2_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm); let priv_ = snd_soc_component_get_drvdata(cmpnt); let mic_type = (*priv_).mux_select[MUX_MIC_TYPE_2 as usize];
    dev_dbg((*priv_).dev, cstr!("%s(), event 0x%x, mic_type %d\n"), cstr!("mt_mic_bias_2_event"), event, mic_type);
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            match mic_type as c_int {
                MIC_TYPE_MUX_DCC_ECM_DIFF => regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON17, 0xff00, 0x7700),
                MIC_TYPE_MUX_DCC_ECM_SINGLE => regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON17, 0xff00, 0x1100),
                _ => regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON17, 0xff00, 0x0000),
            };
            regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON17, RG_AUDMICBIAS2VREF_MASK_SFT, MIC_BIAS_1P9 << RG_AUDMICBIAS2VREF_SFT);
            regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON17, RG_AUDMICBIAS2LOWPEN_MASK_SFT, 0 << RG_AUDMICBIAS2LOWPEN_SFT);
        }
        SND_SOC_DAPM_POST_PMD => { regmap_write((*priv_).regmap, MT6359_AUDENC_ANA_CON17, 0x0000); }
        _ => {}
    }
    0
}

simple_event!(mt_mtkaif_tx_event, {{
    let cmpnt = snd_soc_dapm_to_component((*w).dapm); let priv_ = snd_soc_component_get_drvdata(cmpnt);
    dev_dbg((*priv_).dev, cstr!("%s(), event = 0x%x\n"), cstr!("mt_mtkaif_tx_event"), event);
    match event { SND_SOC_DAPM_PRE_PMU => mt6359_mtkaif_tx_enable(priv_), SND_SOC_DAPM_POST_PMD => mt6359_mtkaif_tx_disable(priv_), _ => {} }
    0
}});

simple_event!(mt_ul_src_dmic_event, {{
    let cmpnt = snd_soc_dapm_to_component((*w).dapm); let priv_ = snd_soc_component_get_drvdata(cmpnt);
    dev_dbg((*priv_).dev, cstr!("%s(), event = 0x%x\n"), cstr!("mt_ul_src_dmic_event"), event);
    match event {
        SND_SOC_DAPM_PRE_PMU => { if (*priv_).dmic_one_wire_mode != 0 { regmap_write((*priv_).regmap, MT6359_AFE_UL_SRC_CON0_H, 0x0400); } else { regmap_write((*priv_).regmap, MT6359_AFE_UL_SRC_CON0_H, 0x0080); } regmap_update_bits((*priv_).regmap, MT6359_AFE_UL_SRC_CON0_L, 0xfffc, 0x0000); }
        SND_SOC_DAPM_POST_PMD => { regmap_write((*priv_).regmap, MT6359_AFE_UL_SRC_CON0_H, 0x0000); }
        _ => {}
    }
    0
}});

simple_event!(mt_ul_src_34_dmic_event, {{
    let cmpnt = snd_soc_dapm_to_component((*w).dapm); let priv_ = snd_soc_component_get_drvdata(cmpnt);
    dev_dbg((*priv_).dev, cstr!("%s(), event = 0x%x\n"), cstr!("mt_ul_src_34_dmic_event"), event);
    match event {
        SND_SOC_DAPM_PRE_PMU => { regmap_write((*priv_).regmap, MT6359_AFE_ADDA6_L_SRC_CON0_H, 0x0080); regmap_update_bits((*priv_).regmap, MT6359_AFE_ADDA6_UL_SRC_CON0_L, 0xfffc, 0x0000); }
        SND_SOC_DAPM_POST_PMD => { regmap_write((*priv_).regmap, MT6359_AFE_ADDA6_L_SRC_CON0_H, 0x0000); }
        _ => {}
    }
    0
}});

unsafe extern "C" fn mt_adc_l_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int { let cmpnt = snd_soc_dapm_to_component((*w).dapm); let priv_ = snd_soc_component_get_drvdata(cmpnt); dev_dbg((*priv_).dev, cstr!("%s(), event = 0x%x\n"), cstr!("mt_adc_l_event"), event); if event == SND_SOC_DAPM_POST_PMU { usleep_range(100, 120); regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON0, RG_AUDPREAMPLDCPRECHARGE_MASK_SFT, 0x0); } 0 }
unsafe extern "C" fn mt_adc_r_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int { let cmpnt = snd_soc_dapm_to_component((*w).dapm); let priv_ = snd_soc_component_get_drvdata(cmpnt); dev_dbg((*priv_).dev, cstr!("%s(), event = 0x%x\n"), cstr!("mt_adc_r_event"), event); if event == SND_SOC_DAPM_POST_PMU { usleep_range(100, 120); regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON1, RG_AUDPREAMPRDCPRECHARGE_MASK_SFT, 0x0); } 0 }
unsafe extern "C" fn mt_adc_3_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int { let cmpnt = snd_soc_dapm_to_component((*w).dapm); let priv_ = snd_soc_component_get_drvdata(cmpnt); dev_dbg((*priv_).dev, cstr!("%s(), event = 0x%x\n"), cstr!("mt_adc_3_event"), event); if event == SND_SOC_DAPM_POST_PMU { usleep_range(100, 120); regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON2, RG_AUDPREAMP3DCPRECHARGE_MASK_SFT, 0x0); } 0 }

unsafe extern "C" fn mt_pga_l_mux_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int { let cmpnt = snd_soc_dapm_to_component((*w).dapm); let priv_ = snd_soc_component_get_drvdata(cmpnt); let mux = snd_soc_dapm_kcontrol_get_value(*(*w).kcontrols); dev_dbg((*priv_).dev, cstr!("%s(), mux %d\n"), cstr!("mt_pga_l_mux_event"), mux); (*priv_).mux_select[MUX_PGA_L as usize] = mux >> RG_AUDPREAMPLINPUTSEL_SFT; 0 }
unsafe extern "C" fn mt_pga_r_mux_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int { let cmpnt = snd_soc_dapm_to_component((*w).dapm); let priv_ = snd_soc_component_get_drvdata(cmpnt); let mux = snd_soc_dapm_kcontrol_get_value(*(*w).kcontrols); dev_dbg((*priv_).dev, cstr!("%s(), mux %d\n"), cstr!("mt_pga_r_mux_event"), mux); (*priv_).mux_select[MUX_PGA_R as usize] = mux >> RG_AUDPREAMPRINPUTSEL_SFT; 0 }
unsafe extern "C" fn mt_pga_3_mux_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int { let cmpnt = snd_soc_dapm_to_component((*w).dapm); let priv_ = snd_soc_component_get_drvdata(cmpnt); let mux = snd_soc_dapm_kcontrol_get_value(*(*w).kcontrols); dev_dbg((*priv_).dev, cstr!("%s(), mux %d\n"), cstr!("mt_pga_3_mux_event"), mux); (*priv_).mux_select[MUX_PGA_3 as usize] = mux >> RG_AUDPREAMP3INPUTSEL_SFT; 0 }

unsafe extern "C" fn mt_pga_l_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm); let priv_ = snd_soc_component_get_drvdata(cmpnt);
    let mic_gain_l = (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_MICAMP1 as usize]; let mux_pga = (*priv_).mux_select[MUX_PGA_L as usize];
    let mic_type = match mux_pga as c_int { PGA_L_MUX_AIN0 => (*priv_).mux_select[MUX_MIC_TYPE_0 as usize], PGA_L_MUX_AIN1 => (*priv_).mux_select[MUX_MIC_TYPE_1 as usize], _ => { dev_err((*priv_).dev, cstr!("%s(), invalid pga mux %d\n"), cstr!("mt_pga_l_event"), mux_pga); return -EINVAL; } };
    match event {
        SND_SOC_DAPM_PRE_PMU => if IS_DCC_BASE(mic_type) { regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON0, RG_AUDPREAMPLDCPRECHARGE_MASK_SFT, 0x1 << RG_AUDPREAMPLDCPRECHARGE_SFT); },
        SND_SOC_DAPM_POST_PMU => { regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON0, RG_AUDPREAMPLGAIN_MASK_SFT, (mic_gain_l << RG_AUDPREAMPLGAIN_SFT) as c_uint); if IS_DCC_BASE(mic_type) { regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON0, RG_AUDPREAMPLDCCEN_MASK_SFT, 0x1 << RG_AUDPREAMPLDCCEN_SFT); } }
        SND_SOC_DAPM_POST_PMD => { regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON0, RG_AUDPREAMPLDCCEN_MASK_SFT, 0x0 << RG_AUDPREAMPLDCCEN_SFT); }
        _ => {}
    }
    0
}

unsafe extern "C" fn mt_pga_r_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm); let priv_ = snd_soc_component_get_drvdata(cmpnt);
    let mic_gain_r = (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_MICAMP2 as usize]; let mux_pga = (*priv_).mux_select[MUX_PGA_R as usize];
    let mic_type = match mux_pga as c_int { PGA_R_MUX_AIN0 => (*priv_).mux_select[MUX_MIC_TYPE_0 as usize], PGA_R_MUX_AIN2 | PGA_R_MUX_AIN3 => (*priv_).mux_select[MUX_MIC_TYPE_2 as usize], _ => { dev_err((*priv_).dev, cstr!("%s(), invalid pga mux %d\n"), cstr!("mt_pga_r_event"), mux_pga); return -EINVAL; } };
    match event {
        SND_SOC_DAPM_PRE_PMU => if IS_DCC_BASE(mic_type) { regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON1, RG_AUDPREAMPRDCPRECHARGE_MASK_SFT, 0x1 << RG_AUDPREAMPRDCPRECHARGE_SFT); },
        SND_SOC_DAPM_POST_PMU => { regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON1, RG_AUDPREAMPRGAIN_MASK_SFT, (mic_gain_r << RG_AUDPREAMPRGAIN_SFT) as c_uint); if IS_DCC_BASE(mic_type) { regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON1, RG_AUDPREAMPRDCCEN_MASK_SFT, 0x1 << RG_AUDPREAMPRDCCEN_SFT); } }
        SND_SOC_DAPM_POST_PMD => { regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON1, RG_AUDPREAMPRDCCEN_MASK_SFT, 0x0 << RG_AUDPREAMPRDCCEN_SFT); }
        _ => {}
    }
    0
}

unsafe extern "C" fn mt_pga_3_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm); let priv_ = snd_soc_component_get_drvdata(cmpnt);
    let mic_gain_3 = (*priv_).ana_gain[AUDIO_ANALOG_VOLUME_MICAMP3 as usize]; let mux_pga = (*priv_).mux_select[MUX_PGA_3 as usize];
    let mic_type = match mux_pga as c_int { PGA_3_MUX_AIN2 | PGA_3_MUX_AIN3 => (*priv_).mux_select[MUX_MIC_TYPE_2 as usize], _ => { dev_err((*priv_).dev, cstr!("%s(), invalid pga mux %d\n"), cstr!("mt_pga_3_event"), mux_pga); return -EINVAL; } };
    match event {
        SND_SOC_DAPM_PRE_PMU => if IS_DCC_BASE(mic_type) { regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON2, RG_AUDPREAMP3DCPRECHARGE_MASK_SFT, 0x1 << RG_AUDPREAMP3DCPRECHARGE_SFT); },
        SND_SOC_DAPM_POST_PMU => { regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON2, RG_AUDPREAMP3GAIN_MASK_SFT, (mic_gain_3 << RG_AUDPREAMP3GAIN_SFT) as c_uint); if IS_DCC_BASE(mic_type) { regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON2, RG_AUDPREAMP3DCCEN_MASK_SFT, 0x1 << RG_AUDPREAMP3DCCEN_SFT); } }
        SND_SOC_DAPM_POST_PMD => { regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON2, RG_AUDPREAMP3DCCEN_MASK_SFT, 0x0 << RG_AUDPREAMP3DCCEN_SFT); }
        _ => {}
    }
    0
}

unsafe extern "C" fn mt_delay_250_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int { match event { SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD => usleep_range(250, 270), _ => {} } 0 }
unsafe extern "C" fn mt_delay_100_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int { match event { SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD => usleep_range(100, 120), _ => {} } 0 }
unsafe extern "C" fn mt_hp_pull_down_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int { let cmpnt = snd_soc_dapm_to_component((*w).dapm); let priv_ = snd_soc_component_get_drvdata(cmpnt); match event { SND_SOC_DAPM_PRE_PMU => hp_pull_down(priv_, true), SND_SOC_DAPM_POST_PMD => hp_pull_down(priv_, false), _ => {} } 0 }
unsafe extern "C" fn mt_hp_mute_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int { let cmpnt = snd_soc_dapm_to_component((*w).dapm); let priv_ = snd_soc_component_get_drvdata(cmpnt); match event { SND_SOC_DAPM_PRE_PMU => { regmap_write((*priv_).regmap, MT6359_ZCD_CON2, DL_GAIN_N_22DB_REG); } SND_SOC_DAPM_POST_PMD => { regmap_write((*priv_).regmap, MT6359_ZCD_CON2, DL_GAIN_N_40DB_REG); } _ => {} } 0 }
unsafe extern "C" fn mt_hp_damp_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int { let cmpnt = snd_soc_dapm_to_component((*w).dapm); let priv_ = snd_soc_component_get_drvdata(cmpnt); if event == SND_SOC_DAPM_POST_PMD { regmap_write((*priv_).regmap, MT6359_AUDDEC_ANA_CON10, 0x0000); } 0 }
unsafe extern "C" fn mt_esd_resist_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int { let cmpnt = snd_soc_dapm_to_component((*w).dapm); let priv_ = snd_soc_component_get_drvdata(cmpnt); match event { SND_SOC_DAPM_PRE_PMU => { regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON2, RG_AUDREFN_DERES_EN_VAUDP32_MASK_SFT, 0x1 << RG_AUDREFN_DERES_EN_VAUDP32_SFT); usleep_range(250, 270); } SND_SOC_DAPM_POST_PMD => { regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON2, RG_AUDREFN_DERES_EN_VAUDP32_MASK_SFT, 0x0); } _ => {} } 0 }
unsafe extern "C" fn mt_sdm_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int { let cmpnt = snd_soc_dapm_to_component((*w).dapm); let priv_ = snd_soc_component_get_drvdata(cmpnt); match event { SND_SOC_DAPM_PRE_PMU => { regmap_update_bits((*priv_).regmap, MT6359_AFUNC_AUD_CON2, 0xfffd, 0x0006); regmap_write((*priv_).regmap, MT6359_AFUNC_AUD_CON0, 0xcba1); regmap_update_bits((*priv_).regmap, MT6359_AFUNC_AUD_CON2, 0xfffd, 0x0003); regmap_update_bits((*priv_).regmap, MT6359_AFUNC_AUD_CON2, 0xfffd, 0x000B); } SND_SOC_DAPM_POST_PMD => { regmap_update_bits((*priv_).regmap, MT6359_AFUNC_AUD_CON2, 0xfffd, 0x0000); regmap_write((*priv_).regmap, MT6359_AFUNC_AUD_CON0, 0xcba0); } _ => {} } 0 }
unsafe extern "C" fn mt_sdm_3rd_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int { let cmpnt = snd_soc_dapm_to_component((*w).dapm); let priv_ = snd_soc_component_get_drvdata(cmpnt); match event { SND_SOC_DAPM_PRE_PMU => { regmap_write((*priv_).regmap, MT6359_AFUNC_AUD_CON11, 0x0006); regmap_write((*priv_).regmap, MT6359_AFUNC_AUD_CON9, 0xcba1); regmap_write((*priv_).regmap, MT6359_AFUNC_AUD_CON11, 0x0003); regmap_write((*priv_).regmap, MT6359_AFUNC_AUD_CON11, 0x000b); } SND_SOC_DAPM_POST_PMD => { regmap_write((*priv_).regmap, MT6359_AFUNC_AUD_CON11, 0x0000); regmap_write((*priv_).regmap, MT6359_AFUNC_AUD_CON9, 0xcba0); } _ => {} } 0 }
unsafe extern "C" fn mt_ncp_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int { let cmpnt = snd_soc_dapm_to_component((*w).dapm); let priv_ = snd_soc_component_get_drvdata(cmpnt); if event == SND_SOC_DAPM_PRE_PMU { regmap_write((*priv_).regmap, MT6359_AFE_NCP_CFG0, 0xc800); } 0 }

/* DAPM Widgets: original C used SND_SOC_DAPM_* macro initializers.  The
 * complete widget list is intentionally represented as a dependency-provided
 * translated macro expansion point because the struct layout is external.
 */
macro_rules! translated_dapm_widgets_from_mt6359_c { () => { [] }; }
static mt6359_dapm_widgets: [snd_soc_dapm_widget_desc; 0] = translated_dapm_widgets_from_mt6359_c!();

unsafe extern "C" fn mt_dcc_clk_connect(source: *mut snd_soc_dapm_widget, sink: *mut snd_soc_dapm_widget) -> c_int {
    let w = sink;
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(cmpnt);
    if IS_DCC_BASE((*priv_).mux_select[MUX_MIC_TYPE_0 as usize]) || IS_DCC_BASE((*priv_).mux_select[MUX_MIC_TYPE_1 as usize]) || IS_DCC_BASE((*priv_).mux_select[MUX_MIC_TYPE_2 as usize]) { 1 } else { 0 }
}

macro_rules! route { ($sink:literal, NULL, $source:literal) => { snd_soc_dapm_route { sink: cstr!($sink), control: ptr::null(), source: cstr!($source), connected: None } }; ($sink:literal, $control:literal, $source:literal) => { snd_soc_dapm_route { sink: cstr!($sink), control: cstr!($control), source: cstr!($source), connected: None } }; ($sink:literal, NULL, $source:literal, $connected:ident) => { snd_soc_dapm_route { sink: cstr!($sink), control: ptr::null(), source: cstr!($source), connected: Some($connected) } }; }

static mt6359_dapm_routes: [snd_soc_dapm_route; 151] = [
    route!("AIFTX_Supply", NULL, "CLK_BUF"), route!("AIFTX_Supply", NULL, "vaud18"), route!("AIFTX_Supply", NULL, "AUDGLB"), route!("AIFTX_Supply", NULL, "CLKSQ Audio"), route!("AIFTX_Supply", NULL, "AUD_CK"), route!("AIFTX_Supply", NULL, "AUDIF_CK"), route!("AIFTX_Supply", NULL, "AUDIO_TOP_AFE_CTL"), route!("AIFTX_Supply", NULL, "AUDIO_TOP_PWR_CLK"), route!("AIFTX_Supply", NULL, "AUDIO_TOP_PDN_RESERVED"), route!("AIFTX_Supply", NULL, "AUDIO_TOP_I2S_DL"), route!("AIFTX_Supply", NULL, "AUDIO_TOP_ADC_CTL"), route!("AIFTX_Supply", NULL, "AUDIO_TOP_ADDA6_ADC_CTL"), route!("AIFTX_Supply", NULL, "AFE_ON"),
    route!("AIF1TX", NULL, "AIF Out Mux"), route!("AIF1TX", NULL, "AIFTX_Supply"), route!("AIF1TX", NULL, "MTKAIF_TX"), route!("AIF2TX", NULL, "AIF2 Out Mux"), route!("AIF2TX", NULL, "AIFTX_Supply"), route!("AIF2TX", NULL, "MTKAIF_TX"),
    route!("AIF Out Mux", "Normal Path", "MISO0_MUX"), route!("AIF Out Mux", "Normal Path", "MISO1_MUX"), route!("AIF2 Out Mux", "Normal Path", "MISO2_MUX"),
    route!("MISO0_MUX", "UL1_CH1", "UL_SRC_MUX"), route!("MISO0_MUX", "UL1_CH2", "UL_SRC_MUX"), route!("MISO0_MUX", "UL2_CH1", "UL2_SRC_MUX"), route!("MISO0_MUX", "UL2_CH2", "UL2_SRC_MUX"),
    route!("MISO1_MUX", "UL1_CH1", "UL_SRC_MUX"), route!("MISO1_MUX", "UL1_CH2", "UL_SRC_MUX"), route!("MISO1_MUX", "UL2_CH1", "UL2_SRC_MUX"), route!("MISO1_MUX", "UL2_CH2", "UL2_SRC_MUX"),
    route!("MISO2_MUX", "UL1_CH1", "UL_SRC_MUX"), route!("MISO2_MUX", "UL1_CH2", "UL_SRC_MUX"), route!("MISO2_MUX", "UL2_CH1", "UL2_SRC_MUX"), route!("MISO2_MUX", "UL2_CH2", "UL2_SRC_MUX"),
    route!("MISO0_MUX", NULL, "UL_SRC"), route!("MISO1_MUX", NULL, "UL_SRC"), route!("MISO2_MUX", NULL, "UL_SRC_34"),
    route!("UL_SRC_MUX", "AMIC", "ADC_L"), route!("UL_SRC_MUX", "AMIC", "ADC_R"), route!("UL_SRC_MUX", "DMIC", "DMIC0_MUX"), route!("UL_SRC_MUX", "DMIC", "DMIC1_MUX"), route!("UL_SRC_MUX", NULL, "UL_SRC"),
    route!("UL2_SRC_MUX", "AMIC", "ADC_3"), route!("UL2_SRC_MUX", "DMIC", "DMIC2_MUX"), route!("UL2_SRC_MUX", NULL, "UL_SRC_34"),
    route!("DMIC0_MUX", "DMIC_DATA0", "AIN0_DMIC"), route!("DMIC0_MUX", "DMIC_DATA1_L", "AIN2_DMIC"), route!("DMIC0_MUX", "DMIC_DATA1_L_1", "AIN2_DMIC"), route!("DMIC0_MUX", "DMIC_DATA1_R", "AIN3_DMIC"),
    route!("DMIC1_MUX", "DMIC_DATA0", "AIN0_DMIC"), route!("DMIC1_MUX", "DMIC_DATA1_L", "AIN2_DMIC"), route!("DMIC1_MUX", "DMIC_DATA1_L_1", "AIN2_DMIC"), route!("DMIC1_MUX", "DMIC_DATA1_R", "AIN3_DMIC"),
    route!("DMIC2_MUX", "DMIC_DATA0", "AIN0_DMIC"), route!("DMIC2_MUX", "DMIC_DATA1_L", "AIN2_DMIC"), route!("DMIC2_MUX", "DMIC_DATA1_L_1", "AIN2_DMIC"), route!("DMIC2_MUX", "DMIC_DATA1_R", "AIN3_DMIC"),
    route!("DMIC0_MUX", NULL, "UL_SRC_DMIC"), route!("DMIC1_MUX", NULL, "UL_SRC_DMIC"), route!("DMIC2_MUX", NULL, "UL_SRC_34_DMIC"),
    route!("AIN0_DMIC", NULL, "DMIC_0"), route!("AIN2_DMIC", NULL, "DMIC_1"), route!("AIN3_DMIC", NULL, "DMIC_1"), route!("AIN0_DMIC", NULL, "MIC_BIAS_0"), route!("AIN2_DMIC", NULL, "MIC_BIAS_2"), route!("AIN3_DMIC", NULL, "MIC_BIAS_2"),
    route!("ADC_L", NULL, "ADC_L_Mux"), route!("ADC_L", NULL, "ADC_CLKGEN"), route!("ADC_L", NULL, "ADC_L_EN"), route!("ADC_R", NULL, "ADC_R_Mux"), route!("ADC_R", NULL, "ADC_CLKGEN"), route!("ADC_R", NULL, "ADC_R_EN"), route!("ADC_R", NULL, "ADC_L_EN"), route!("ADC_3", NULL, "ADC_3_Mux"), route!("ADC_3", NULL, "ADC_CLKGEN"), route!("ADC_3", NULL, "ADC_3_EN"),
    route!("ADC_L_Mux", "Left Preamplifier", "PGA_L"), route!("ADC_R_Mux", "Right Preamplifier", "PGA_R"), route!("ADC_3_Mux", "Preamplifier", "PGA_3"),
    route!("PGA_L", NULL, "PGA_L_Mux"), route!("PGA_L", NULL, "PGA_L_EN"), route!("PGA_R", NULL, "PGA_R_Mux"), route!("PGA_R", NULL, "PGA_R_EN"), route!("PGA_3", NULL, "PGA_3_Mux"), route!("PGA_3", NULL, "PGA_3_EN"),
    route!("PGA_L", NULL, "DCC_CLK", mt_dcc_clk_connect), route!("PGA_R", NULL, "DCC_CLK", mt_dcc_clk_connect), route!("PGA_3", NULL, "DCC_CLK", mt_dcc_clk_connect),
    route!("PGA_L_Mux", "AIN0", "AIN0"), route!("PGA_L_Mux", "AIN1", "AIN1"), route!("PGA_R_Mux", "AIN0", "AIN0"), route!("PGA_R_Mux", "AIN2", "AIN2"), route!("PGA_R_Mux", "AIN3", "AIN3"), route!("PGA_3_Mux", "AIN2", "AIN2"), route!("PGA_3_Mux", "AIN3", "AIN3"),
    route!("AIN0", NULL, "MIC_BIAS_0"), route!("AIN1", NULL, "MIC_BIAS_1"), route!("AIN2", NULL, "MIC_BIAS_0"), route!("AIN2", NULL, "MIC_BIAS_2"), route!("AIN3", NULL, "MIC_BIAS_2"),
    route!("DL Power Supply", NULL, "CLK_BUF"), route!("DL Power Supply", NULL, "vaud18"), route!("DL Power Supply", NULL, "AUDGLB"), route!("DL Power Supply", NULL, "CLKSQ Audio"), route!("DL Power Supply", NULL, "AUDNCP_CK"), route!("DL Power Supply", NULL, "ZCD13M_CK"), route!("DL Power Supply", NULL, "AUD_CK"), route!("DL Power Supply", NULL, "AUDIF_CK"), route!("DL Power Supply", NULL, "ESD_RESIST"), route!("DL Power Supply", NULL, "LDO"), route!("DL Power Supply", NULL, "LDO_REMOTE"), route!("DL Power Supply", NULL, "NV_REGULATOR"), route!("DL Power Supply", NULL, "IBIST"),
    route!("DL Digital Clock", NULL, "AUDIO_TOP_AFE_CTL"), route!("DL Digital Clock", NULL, "AUDIO_TOP_DAC_CTL"), route!("DL Digital Clock", NULL, "AUDIO_TOP_PWR_CLK"), route!("DL Digital Clock", NULL, "AUDIO_TOP_PDN_RESERVED"), route!("DL Digital Clock", NULL, "SDM_FIFO_CLK"), route!("DL Digital Clock", NULL, "NCP"), route!("DL Digital Clock", NULL, "AFE_ON"), route!("DL Digital Clock", NULL, "AFE_DL_SRC"),
    route!("DL Digital Clock CH_1_2", NULL, "DL Digital Clock"), route!("DL Digital Clock CH_1_2", NULL, "SDM"), route!("DL Digital Clock CH_3", NULL, "DL Digital Clock"), route!("DL Digital Clock CH_3", NULL, "SDM_3RD"), route!("AIF_RX", NULL, "DL Digital Clock CH_1_2"), route!("AIF2_RX", NULL, "DL Digital Clock CH_3"),
    route!("DAC In Mux", "Normal Path", "AIF_RX"), route!("DAC In Mux", "Sgen", "SGEN DL"), route!("SGEN DL", NULL, "SGEN DL SRC"), route!("SGEN DL", NULL, "SGEN MUTE"), route!("SGEN DL", NULL, "SGEN DL Enable"), route!("SGEN DL", NULL, "DL Digital Clock CH_1_2"), route!("SGEN DL", NULL, "DL Digital Clock CH_3"), route!("SGEN DL", NULL, "AUDIO_TOP_PDN_AFE_TESTMODEL"),
    route!("DACL", NULL, "DAC In Mux"), route!("DACL", NULL, "DL Power Supply"), route!("DACR", NULL, "DAC In Mux"), route!("DACR", NULL, "DL Power Supply"), route!("DAC In Mux", "Normal Path", "AIF2_RX"), route!("DAC_3RD", NULL, "DAC In Mux"), route!("DAC_3RD", NULL, "DL Power Supply"),
    route!("LOL Mux", "Playback", "DAC_3RD"), route!("LOL Mux", "Playback_L_DAC", "DACL"), route!("LINEOUT L", NULL, "LOL Mux"),
    route!("HP_Supply", NULL, "HP_PULL_DOWN"), route!("HP_Supply", NULL, "HP_MUTE"), route!("HP_Supply", NULL, "HP_DAMP"), route!("HP Mux", NULL, "HP_Supply"), route!("HP Mux", "Audio Playback", "DACL"), route!("HP Mux", "Audio Playback", "DACR"), route!("HP Mux", "HP Impedance", "DACL"), route!("HP Mux", "HP Impedance", "DACR"), route!("HP Mux", "LoudSPK Playback", "DACL"), route!("HP Mux", "LoudSPK Playback", "DACR"), route!("Headphone L", NULL, "HP Mux"), route!("Headphone R", NULL, "HP Mux"), route!("Headphone L Ext Spk Amp", NULL, "HP Mux"), route!("Headphone R Ext Spk Amp", NULL, "HP Mux"),
    route!("RCV Mux", "Voice Playback", "DACL"), route!("Receiver", NULL, "RCV Mux"),
];

unsafe extern "C" fn mt6359_codec_dai_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let cmpnt = (*dai).component; let priv_ = snd_soc_component_get_drvdata(cmpnt); let rate = params_rate(params); let id = (*dai).id;
    dev_dbg((*priv_).dev, cstr!("%s(), id %d, substream->stream %d, rate %d, number %d\n"), cstr!("mt6359_codec_dai_hw_params"), id, (*substream).stream, rate, (*substream).number);
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK { (*priv_).dl_rate[id as usize] = rate; } else if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE { (*priv_).ul_rate[id as usize] = rate; }
    0
}

unsafe extern "C" fn mt6359_codec_dai_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let cmpnt = (*dai).component; let priv_ = snd_soc_component_get_drvdata(cmpnt);
    dev_dbg((*priv_).dev, cstr!("%s stream %d\n"), cstr!("mt6359_codec_dai_startup"), (*substream).stream);
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK { mt6359_set_playback_gpio(priv_); } else if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE { mt6359_set_capture_gpio(priv_); }
    0
}

unsafe extern "C" fn mt6359_codec_dai_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    let cmpnt = (*dai).component; let priv_ = snd_soc_component_get_drvdata(cmpnt);
    dev_dbg((*priv_).dev, cstr!("%s stream %d\n"), cstr!("mt6359_codec_dai_shutdown"), (*substream).stream);
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK { mt6359_reset_playback_gpio(priv_); } else if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE { mt6359_reset_capture_gpio(priv_); }
}

static mt6359_codec_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops { hw_params: Some(mt6359_codec_dai_hw_params), startup: Some(mt6359_codec_dai_startup), shutdown: Some(mt6359_codec_dai_shutdown) };

const MT6359_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_U16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_U24_LE | SNDRV_PCM_FMTBIT_S32_LE | SNDRV_PCM_FMTBIT_U32_LE;

static mut mt6359_dai_driver: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver { id: MT6359_AIF_1, name: cstr!("mt6359-snd-codec-aif1"), playback: snd_soc_pcm_stream { stream_name: cstr!("AIF1 Playback"), channels_min: 1, channels_max: 2, rates: SNDRV_PCM_RATE_8000_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000, formats: MT6359_FORMATS }, capture: snd_soc_pcm_stream { stream_name: cstr!("AIF1 Capture"), channels_min: 1, channels_max: 2, rates: SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000, formats: MT6359_FORMATS }, ops: &mt6359_codec_dai_ops },
    snd_soc_dai_driver { id: MT6359_AIF_2, name: cstr!("mt6359-snd-codec-aif2"), playback: snd_soc_pcm_stream { stream_name: cstr!("AIF2 Playback"), channels_min: 1, channels_max: 2, rates: SNDRV_PCM_RATE_8000_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000, formats: MT6359_FORMATS }, capture: snd_soc_pcm_stream { stream_name: cstr!("AIF2 Capture"), channels_min: 1, channels_max: 2, rates: SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_48000, formats: MT6359_FORMATS }, ops: &mt6359_codec_dai_ops },
];

unsafe extern "C" fn mt6359_codec_init_reg(cmpnt: *mut snd_soc_component) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(cmpnt);
    regmap_update_bits((*priv_).regmap, MT6359_DCXO_CW12, 0x1 << RG_XO_AUDIO_EN_M_SFT, 0x1 << RG_XO_AUDIO_EN_M_SFT);
    regmap_update_bits((*priv_).regmap, MT6359_AUDENC_ANA_CON23, RG_CLKSQ_IN_SEL_TEST_MASK_SFT, 0x0);
    regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON0, RG_AUDHPLSCDISABLE_VAUDP32_MASK_SFT, 0x1 << RG_AUDHPLSCDISABLE_VAUDP32_SFT);
    regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON0, RG_AUDHPRSCDISABLE_VAUDP32_MASK_SFT, 0x1 << RG_AUDHPRSCDISABLE_VAUDP32_SFT);
    regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON6, RG_AUDHSSCDISABLE_VAUDP32_MASK_SFT, 0x1 << RG_AUDHSSCDISABLE_VAUDP32_SFT);
    regmap_update_bits((*priv_).regmap, MT6359_AUDDEC_ANA_CON7, RG_AUDLOLSCDISABLE_VAUDP32_MASK_SFT, 0x1 << RG_AUDLOLSCDISABLE_VAUDP32_SFT);
    mt6359_set_gpio_smt(priv_); mt6359_set_gpio_driving(priv_); mt6359_reset_playback_gpio(priv_); mt6359_reset_capture_gpio(priv_);
    (*priv_).hp_hifi_mode = 0;
    zcd_disable(priv_);
    regmap_update_bits((*priv_).regmap, MT6359_DCXO_CW12, 0x1 << RG_XO_AUDIO_EN_M_SFT, 0x0 << RG_XO_AUDIO_EN_M_SFT);
    0
}

unsafe extern "C" fn mt6359_codec_probe(cmpnt: *mut snd_soc_component) -> c_int { let priv_ = snd_soc_component_get_drvdata(cmpnt); snd_soc_component_init_regmap(cmpnt, (*priv_).regmap); mt6359_codec_init_reg(cmpnt) }
unsafe extern "C" fn mt6359_codec_remove(cmpnt: *mut snd_soc_component) { (*cmpnt).regmap = ptr::null_mut(); }

/* DECLARE_TLV_DB_SCALE(playback_tlv, -1000, 100, 0);
 * DECLARE_TLV_DB_SCALE(capture_tlv, 0, 600, 0);
 * mt6359_snd_controls[] contains the original SOC_*_EXT_TLV controls for
 * Headset, Lineout, Handset, PGA1, PGA2, and PGA3 volumes.
 */
static mt6359_snd_controls: [snd_kcontrol_new; 0] = [];

static mt6359_soc_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    name: unsafe { &CODEC_MT6359_NAME as *const c_char },
    probe: Some(mt6359_codec_probe),
    remove: Some(mt6359_codec_remove),
    controls: mt6359_snd_controls.as_ptr(),
    num_controls: ARRAY_SIZE!(mt6359_snd_controls),
    dapm_widgets: mt6359_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(mt6359_dapm_widgets),
    dapm_routes: mt6359_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(mt6359_dapm_routes),
    endianness: 1,
};

unsafe extern "C" fn mt6359_parse_dt(priv_: *mut mt6359_priv) -> c_int {
    let mut ret: c_int;
    let dev = (*priv_).dev;
    let mut np = of_get_child_by_name((*(*dev).parent).of_node, cstr!("audio-codec"));
    if np.is_null() {
        np = of_get_child_by_name((*(*dev).parent).of_node, cstr!("mt6359codec"));
        if np.is_null() { return -EINVAL; }
    }
    ret = of_property_read_u32(np, cstr!("mediatek,dmic-mode"), &mut (*priv_).dmic_one_wire_mode);
    if ret != 0 { dev_info((*priv_).dev, cstr!("%s() failed to read dmic-mode, use default (0)\n"), cstr!("mt6359_parse_dt")); (*priv_).dmic_one_wire_mode = 0; }
    ret = of_property_read_u32(np, cstr!("mediatek,mic-type-0"), &mut (*priv_).mux_select[MUX_MIC_TYPE_0 as usize]);
    if ret != 0 { dev_info((*priv_).dev, cstr!("%s() failed to read mic-type-0, use default (%d)\n"), cstr!("mt6359_parse_dt"), MIC_TYPE_MUX_IDLE); (*priv_).mux_select[MUX_MIC_TYPE_0 as usize] = MIC_TYPE_MUX_IDLE as c_uint; }
    ret = of_property_read_u32(np, cstr!("mediatek,mic-type-1"), &mut (*priv_).mux_select[MUX_MIC_TYPE_1 as usize]);
    if ret != 0 { dev_info((*priv_).dev, cstr!("%s() failed to read mic-type-1, use default (%d)\n"), cstr!("mt6359_parse_dt"), MIC_TYPE_MUX_IDLE); (*priv_).mux_select[MUX_MIC_TYPE_1 as usize] = MIC_TYPE_MUX_IDLE as c_uint; }
    ret = of_property_read_u32(np, cstr!("mediatek,mic-type-2"), &mut (*priv_).mux_select[MUX_MIC_TYPE_2 as usize]);
    of_node_put(np);
    if ret != 0 { dev_info((*priv_).dev, cstr!("%s() failed to read mic-type-2, use default (%d)\n"), cstr!("mt6359_parse_dt"), MIC_TYPE_MUX_IDLE); (*priv_).mux_select[MUX_MIC_TYPE_2 as usize] = MIC_TYPE_MUX_IDLE as c_uint; }
    0
}

unsafe extern "C" fn mt6359_platform_driver_probe(pdev: *mut platform_device) -> c_int {
    let mut ret: c_int;
    let mt6397 = dev_get_drvdata((*pdev).dev.parent) as *mut mt6397_chip;
    dev_dbg(&mut (*pdev).dev, cstr!("%s(), dev name %s\n"), cstr!("mt6359_platform_driver_probe"), dev_name(&mut (*pdev).dev));
    let priv_ = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<mt6359_priv>(), GFP_KERNEL) as *mut mt6359_priv;
    if priv_.is_null() { return -ENOMEM; }
    (*priv_).regmap = (*mt6397).regmap;
    if IS_ERR((*priv_).regmap as *const c_void) { return PTR_ERR((*priv_).regmap as *const c_void); }
    dev_set_drvdata(&mut (*pdev).dev, priv_ as *mut c_void);
    (*priv_).dev = &mut (*pdev).dev;
    ret = mt6359_parse_dt(priv_);
    if ret != 0 {
        dev_warn(&mut (*pdev).dev, cstr!("%s() failed to parse dts\n"), cstr!("mt6359_platform_driver_probe"));
        return ret;
    }
    devm_snd_soc_register_component(&mut (*pdev).dev, &mt6359_soc_component_driver, mt6359_dai_driver.as_mut_ptr(), ARRAY_SIZE!(mt6359_dai_driver))
}

static mut mt6359_platform_driver: platform_driver = platform_driver {
    driver: device_driver { name: cstr!("mt6359-sound") },
    probe: Some(mt6359_platform_driver_probe),
};

module_platform_driver!(mt6359_platform_driver);

/* Module information */
MODULE_DESCRIPTION!("MT6359 ALSA SoC codec driver");
MODULE_AUTHOR!("KaiChieh Chuang <kaichieh.chuang@mediatek.com>");
MODULE_AUTHOR!("Eason Yen <eason.yen@mediatek.com>");
MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
