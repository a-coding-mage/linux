// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * da7218.rs - DA7218 ALSA SoC Codec Driver
 *
 * Copyright (c) 2015 Dialog Semiconductor
 *
 * Author: Adam Thomson <Adam.Thomson.Opensource@diasemi.com>
 *
 * Rust source-level translation of soc/codecs/da7218.c. Kernel, ALSA, regmap,
 * device-tree, I2C, regulator, clock, and module symbols are external
 * dependencies supplied by the surrounding repository.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type u16 = u16;
type u32 = u32;
type u64 = u64;
type bool_ = bool;
type irqreturn_t = c_int;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const KOBJ_CHANGE: c_uint = 0;
const IRQF_TRIGGER_LOW: c_uint = 0x00000008;
const IRQF_ONESHOT: c_uint = 0x00002000;
const SND_JACK_HEADPHONE: c_int = 0x0001;

#[repr(C)] pub struct device { pub of_node: *mut device_node, pub kobj: kobject }
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct regulator { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_jack { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { pub private_value: usize }
#[repr(C)] pub struct snd_soc_component { pub dev: *mut device }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { pub dapm: *mut snd_soc_dapm_context, pub reg: c_uint }
#[repr(C)] pub struct snd_soc_dai { pub component: *mut snd_soc_component, pub dev: *mut device }
#[repr(C)] pub struct snd_pcm_substream { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai_ops { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai_driver { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_component_driver { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_route { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget_desc { _private: [u8; 0] }
#[repr(C)] pub struct soc_enum { _private: [u8; 0] }
#[repr(C)] pub struct reg_default { pub reg: c_uint, pub def: c_uint }
#[repr(C)] pub struct regmap_config { _private: [u8; 0] }
#[repr(C)] pub struct regulator_bulk_data { pub supply: *const c_char, pub consumer: *mut regulator }
#[repr(C)] pub struct i2c_client { pub dev: device, pub irq: c_int }
#[repr(C)] pub struct i2c_device_id { _private: [u8; 0] }
#[repr(C)] pub struct i2c_driver { _private: [u8; 0] }
#[repr(C)] pub struct of_device_id { _private: [u8; 0] }

#[repr(C)] pub struct soc_mixer_control { pub reg: c_uint, pub rreg: c_uint, pub shift: c_uint, pub rshift: c_uint, pub max: c_uint }
#[repr(C)] pub struct soc_bytes_ext { pub max: c_uint }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_union }
#[repr(C)] pub union snd_ctl_elem_value_union { pub integer: snd_ctl_elem_value_integer, pub bytes: snd_ctl_elem_value_bytes }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_integer { pub value: [c_long; 128] }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_bytes { pub data: [u8; 512] }
type c_long = isize;

#[repr(C)] pub struct da7218_hpldet_pdata {
    pub jack_rate: c_uint,
    pub jack_debounce: c_uint,
    pub jack_thr: c_uint,
    pub comp_inv: bool,
    pub hyst: bool,
    pub discharge: bool,
}

#[repr(C)] pub struct da7218_pdata {
    pub micbias1_lvl: c_uint,
    pub micbias2_lvl: c_uint,
    pub mic1_amp_in_sel: c_uint,
    pub mic2_amp_in_sel: c_uint,
    pub dmic1_data_sel: c_uint,
    pub dmic1_samplephase: c_uint,
    pub dmic1_clk_rate: c_uint,
    pub dmic2_data_sel: c_uint,
    pub dmic2_samplephase: c_uint,
    pub dmic2_clk_rate: c_uint,
    pub hp_diff_single_supply: bool,
    pub hpldet_pdata: *mut da7218_hpldet_pdata,
}

#[repr(C)] pub struct da7218_priv {
    pub regmap: *mut regmap,
    pub mclk: *mut clk,
    pub mclk_rate: c_uint,
    pub master: bool,
    pub alc_en: c_uint,
    pub mic_lvl_det_en: c_uint,
    pub in_filt_en: c_uint,
    pub biq_5stage_coeff: [u8; DA7218_OUT_1_BIQ_5STAGE_CFG_SIZE as usize],
    pub stbiq_3stage_coeff: [u8; DA7218_SIDETONE_BIQ_3STAGE_CFG_SIZE as usize],
    pub jack: *mut snd_soc_jack,
    pub pdata: *mut da7218_pdata,
    pub irq: c_int,
    pub dev_id: usize,
    pub hp_single_supply: bool,
    pub supplies: [regulator_bulk_data; DA7218_NUM_SUPPLIES as usize],
}

extern "C" {
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_put_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_get_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> c_uint;
    fn regmap_raw_read(map: *mut regmap, reg: c_uint, val: *mut c_void, len: usize) -> c_int;
    fn regmap_raw_write(map: *mut regmap, reg: c_uint, val: *const c_void, len: usize) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn msleep(msecs: c_uint);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn clk_round_rate(clk: *mut clk, rate: c_uint) -> c_uint;
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn fls(x: c_uint) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int);
    fn kobject_uevent_env(kobj: *mut kobject, action: c_uint, envp: *mut *mut c_char) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out: *mut u32) -> c_int;
    fn of_property_read_string(np: *mut device_node, propname: *const c_char, out: *mut *const c_char) -> c_int;
    fn of_property_read_bool(np: *mut device_node, propname: *const c_char) -> bool;
    fn of_get_child_by_name(np: *mut device_node, name: *const c_char) -> *mut device_node;
    fn of_node_put(np: *mut device_node);
    fn devm_regulator_bulk_get(dev: *mut device, num: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn regulator_get_voltage(regulator: *mut regulator) -> c_int;
    fn regulator_bulk_enable(num: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn devm_clk_get_optional(dev: *mut device, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_request_threaded_irq(dev: *mut device, irq: c_int, handler: *const c_void, thread_fn: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_uint, name: *const c_char, data: *mut c_void) -> c_int;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn i2c_get_match_data(i2c: *mut i2c_client) -> *const c_void;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
}

extern "Rust" {
    static da7218_alc_attack_rate: soc_enum;
    static da7218_alc_release_rate: soc_enum;
    static da7218_alc_hold_time: soc_enum;
    static da7218_alc_anticlip_step: soc_enum;
    static da7218_integ_attack_rate: soc_enum;
    static da7218_integ_release_rate: soc_enum;
    static da7218_gain_ramp_rate: soc_enum;
    static da7218_in1_hpf_mode: soc_enum;
    static da7218_in2_hpf_mode: soc_enum;
    static da7218_out1_hpf_mode: soc_enum;
    static da7218_in1_audio_hpf_corner: soc_enum;
    static da7218_in2_audio_hpf_corner: soc_enum;
    static da7218_out1_audio_hpf_corner: soc_enum;
    static da7218_in1_voice_hpf_corner: soc_enum;
    static da7218_in2_voice_hpf_corner: soc_enum;
    static da7218_out1_voice_hpf_corner: soc_enum;
    static da7218_tonegen_dtmf_key: soc_enum;
    static da7218_tonegen_swg_sel: soc_enum;
    static da7218_dgs_rise_coeff: soc_enum;
    static da7218_dgs_fall_coeff: soc_enum;
    static da7218_dac_ng_setup_time: soc_enum;
    static da7218_dac_ng_rampup_rate: soc_enum;
    static da7218_dac_ng_rampdown_rate: soc_enum;
    static da7218_cp_mchange: soc_enum;
    static da7218_cp_fcontrol: soc_enum;
    static da7218_cp_tau_delay: soc_enum;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

/* TLVs and Enums translated from DECLARE_TLV_DB_SCALE/SOC_ENUM macros. */
static da7218_mic_gain_tlv: [c_int; 3] = [-600, 600, 0];
static da7218_mixin_gain_tlv: [c_int; 3] = [-450, 150, 0];
static da7218_in_dig_gain_tlv: [c_int; 3] = [-8325, 75, 0];
static da7218_ags_trigger_tlv: [c_int; 3] = [-9000, 600, 0];
static da7218_ags_att_max_tlv: [c_int; 3] = [0, 600, 0];
static da7218_alc_threshold_tlv: [c_int; 3] = [-9450, 150, 0];
static da7218_alc_gain_tlv: [c_int; 3] = [0, 600, 0];
static da7218_alc_ana_gain_tlv: [c_int; 3] = [0, 600, 0];
static da7218_dmix_gain_tlv: [c_int; 3] = [-4200, 150, 0];
static da7218_dgs_trigger_tlv: [c_int; 3] = [-9450, 150, 0];
static da7218_dgs_anticlip_tlv: [c_int; 3] = [-4200, 600, 0];
static da7218_dgs_signal_tlv: [c_int; 3] = [-9000, 600, 0];
static da7218_out_eq_band_tlv: [c_int; 3] = [-1050, 150, 0];
static da7218_out_dig_gain_tlv: [c_int; 3] = [-8325, 75, 0];
static da7218_dac_ng_threshold_tlv: [c_int; 3] = [-10200, 600, 0];
static da7218_mixout_gain_tlv: [c_int; 3] = [-100, 50, 0];
static da7218_hp_gain_tlv: [c_int; 3] = [-5700, 150, 0];

static da7218_alc_attack_rate_txt: [&str; 13] = ["7.33/fs", "14.66/fs", "29.32/fs", "58.64/fs", "117.3/fs", "234.6/fs", "469.1/fs", "938.2/fs", "1876/fs", "3753/fs", "7506/fs", "15012/fs", "30024/fs"];
static da7218_alc_release_rate_txt: [&str; 11] = ["28.66/fs", "57.33/fs", "114.6/fs", "229.3/fs", "458.6/fs", "917.1/fs", "1834/fs", "3668/fs", "7337/fs", "14674/fs", "29348/fs"];
static da7218_alc_hold_time_txt: [&str; 16] = ["62/fs", "124/fs", "248/fs", "496/fs", "992/fs", "1984/fs", "3968/fs", "7936/fs", "15872/fs", "31744/fs", "63488/fs", "126976/fs", "253952/fs", "507904/fs", "1015808/fs", "2031616/fs"];
static da7218_alc_anticlip_step_txt: [&str; 4] = ["0.034dB/fs", "0.068dB/fs", "0.136dB/fs", "0.272dB/fs"];
static da7218_integ_rate_txt: [&str; 4] = ["1/4", "1/16", "1/256", "1/65536"];
static da7218_gain_ramp_rate_txt: [&str; 4] = ["Nominal Rate * 8", "Nominal Rate", "Nominal Rate / 8", "Nominal Rate / 16"];
static da7218_hpf_mode_txt: [&str; 3] = ["Disabled", "Audio", "Voice"];
static da7218_hpf_mode_val: [c_uint; 3] = [DA7218_HPF_DISABLED, DA7218_HPF_AUDIO_EN, DA7218_HPF_VOICE_EN];
static da7218_audio_hpf_corner_txt: [&str; 4] = ["2Hz", "4Hz", "8Hz", "16Hz"];
static da7218_voice_hpf_corner_txt: [&str; 8] = ["2.5Hz", "25Hz", "50Hz", "100Hz", "150Hz", "200Hz", "300Hz", "400Hz"];
static da7218_tonegen_dtmf_key_txt: [&str; 16] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "*", "#"];
static da7218_tonegen_swg_sel_txt: [&str; 4] = ["Sum", "SWG1", "SWG2", "SWG1_1-Cos"];
static da7218_dgs_rise_coeff_txt: [&str; 7] = ["1/1", "1/16", "1/64", "1/256", "1/1024", "1/4096", "1/16384"];
static da7218_dgs_fall_coeff_txt: [&str; 8] = ["1/4", "1/16", "1/64", "1/256", "1/1024", "1/4096", "1/16384", "1/65536"];
static da7218_dac_ng_setup_time_txt: [&str; 4] = ["256 Samples", "512 Samples", "1024 Samples", "2048 Samples"];
static da7218_dac_ng_rampup_txt: [&str; 2] = ["0.22ms/dB", "0.0138ms/dB"];
static da7218_dac_ng_rampdown_txt: [&str; 2] = ["0.88ms/dB", "14.08ms/dB"];
static da7218_cp_mchange_txt: [&str; 3] = ["Largest Volume", "DAC Volume", "Signal Magnitude"];
static da7218_cp_mchange_val: [c_uint; 3] = [DA7218_CP_MCHANGE_LARGEST_VOL, DA7218_CP_MCHANGE_DAC_VOL, DA7218_CP_MCHANGE_SIG_MAG];
static da7218_cp_fcontrol_txt: [&str; 6] = ["1MHz", "500KHz", "250KHz", "125KHz", "63KHz", "0KHz"];
static da7218_cp_tau_delay_txt: [&str; 8] = ["0ms", "2ms", "4ms", "16ms", "64ms", "128ms", "256ms", "512ms"];

unsafe fn da7218_alc_calib(component: *mut snd_soc_component) {
    let mic_1_ctrl = snd_soc_component_read(component, DA7218_MIC_1_CTRL) as u8;
    let mic_2_ctrl = snd_soc_component_read(component, DA7218_MIC_2_CTRL) as u8;
    let mixin_1_ctrl = snd_soc_component_read(component, DA7218_MIXIN_1_CTRL) as u8;
    let mixin_2_ctrl = snd_soc_component_read(component, DA7218_MIXIN_2_CTRL) as u8;
    let in_1l_filt_ctrl = snd_soc_component_read(component, DA7218_IN_1L_FILTER_CTRL) as u8;
    let in_1r_filt_ctrl = snd_soc_component_read(component, DA7218_IN_1R_FILTER_CTRL) as u8;
    let in_2l_filt_ctrl = snd_soc_component_read(component, DA7218_IN_2L_FILTER_CTRL) as u8;
    let in_2r_filt_ctrl = snd_soc_component_read(component, DA7218_IN_2R_FILTER_CTRL) as u8;
    let in_1_hpf_ctrl = snd_soc_component_read(component, DA7218_IN_1_HPF_FILTER_CTRL) as u8;
    let in_2_hpf_ctrl = snd_soc_component_read(component, DA7218_IN_2_HPF_FILTER_CTRL) as u8;
    let mut i: c_int = 0;
    let mut calibrated = false;
    let mut calib_ctrl: u8;

    snd_soc_component_update_bits(component, DA7218_MIC_1_CTRL, DA7218_MIC_1_AMP_EN_MASK, DA7218_MIC_1_AMP_EN_MASK);
    snd_soc_component_update_bits(component, DA7218_MIC_2_CTRL, DA7218_MIC_2_AMP_EN_MASK, DA7218_MIC_2_AMP_EN_MASK);
    snd_soc_component_update_bits(component, DA7218_MIC_1_CTRL, DA7218_MIC_1_AMP_MUTE_EN_MASK, DA7218_MIC_1_AMP_MUTE_EN_MASK);
    snd_soc_component_update_bits(component, DA7218_MIC_2_CTRL, DA7218_MIC_2_AMP_MUTE_EN_MASK, DA7218_MIC_2_AMP_MUTE_EN_MASK);
    snd_soc_component_update_bits(component, DA7218_MIXIN_1_CTRL, DA7218_MIXIN_1_AMP_EN_MASK | DA7218_MIXIN_1_AMP_MUTE_EN_MASK, DA7218_MIXIN_1_AMP_EN_MASK);
    snd_soc_component_update_bits(component, DA7218_MIXIN_2_CTRL, DA7218_MIXIN_2_AMP_EN_MASK | DA7218_MIXIN_2_AMP_MUTE_EN_MASK, DA7218_MIXIN_2_AMP_EN_MASK);
    snd_soc_component_update_bits(component, DA7218_IN_1L_FILTER_CTRL, DA7218_IN_1L_FILTER_EN_MASK | DA7218_IN_1L_MUTE_EN_MASK, DA7218_IN_1L_FILTER_EN_MASK);
    snd_soc_component_update_bits(component, DA7218_IN_1R_FILTER_CTRL, DA7218_IN_1R_FILTER_EN_MASK | DA7218_IN_1R_MUTE_EN_MASK, DA7218_IN_1R_FILTER_EN_MASK);
    snd_soc_component_update_bits(component, DA7218_IN_2L_FILTER_CTRL, DA7218_IN_2L_FILTER_EN_MASK | DA7218_IN_2L_MUTE_EN_MASK, DA7218_IN_2L_FILTER_EN_MASK);
    snd_soc_component_update_bits(component, DA7218_IN_2R_FILTER_CTRL, DA7218_IN_2R_FILTER_EN_MASK | DA7218_IN_2R_MUTE_EN_MASK, DA7218_IN_2R_FILTER_EN_MASK);
    snd_soc_component_update_bits(component, DA7218_IN_1_HPF_FILTER_CTRL, DA7218_IN_1_VOICE_EN_MASK, 0);
    snd_soc_component_update_bits(component, DA7218_IN_2_HPF_FILTER_CTRL, DA7218_IN_2_VOICE_EN_MASK, 0);

    snd_soc_component_update_bits(component, DA7218_CALIB_CTRL, DA7218_CALIB_AUTO_EN_MASK, DA7218_CALIB_AUTO_EN_MASK);
    loop {
        calib_ctrl = snd_soc_component_read(component, DA7218_CALIB_CTRL) as u8;
        if (calib_ctrl as c_uint & DA7218_CALIB_AUTO_EN_MASK) != 0 {
            i += 1;
            usleep_range(DA7218_ALC_CALIB_DELAY_MIN, DA7218_ALC_CALIB_DELAY_MAX);
        } else {
            calibrated = true;
        }
        if !((i < DA7218_ALC_CALIB_MAX_TRIES as c_int) && !calibrated) { break; }
    }

    if !calibrated || ((calib_ctrl as c_uint & DA7218_CALIB_OVERFLOW_MASK) != 0) {
        dev_warn((*component).dev, cstr!("ALC auto calibration failed - %s\n"), if calibrated { cstr!("overflow") } else { cstr!("timeout") });
        snd_soc_component_update_bits(component, DA7218_CALIB_CTRL, DA7218_CALIB_OFFSET_EN_MASK, 0);
        snd_soc_component_update_bits(component, DA7218_ALC_CTRL1, DA7218_ALC_SYNC_MODE_MASK, 0);
    } else {
        snd_soc_component_update_bits(component, DA7218_CALIB_CTRL, DA7218_CALIB_OFFSET_EN_MASK, DA7218_CALIB_OFFSET_EN_MASK);
        snd_soc_component_update_bits(component, DA7218_ALC_CTRL1, DA7218_ALC_SYNC_MODE_MASK, DA7218_ALC_SYNC_MODE_CH1 | DA7218_ALC_SYNC_MODE_CH2);
    }

    snd_soc_component_write(component, DA7218_IN_1_HPF_FILTER_CTRL, in_1_hpf_ctrl as c_uint);
    snd_soc_component_write(component, DA7218_IN_2_HPF_FILTER_CTRL, in_2_hpf_ctrl as c_uint);
    snd_soc_component_write(component, DA7218_IN_1L_FILTER_CTRL, in_1l_filt_ctrl as c_uint);
    snd_soc_component_write(component, DA7218_IN_1R_FILTER_CTRL, in_1r_filt_ctrl as c_uint);
    snd_soc_component_write(component, DA7218_IN_2L_FILTER_CTRL, in_2l_filt_ctrl as c_uint);
    snd_soc_component_write(component, DA7218_IN_2R_FILTER_CTRL, in_2r_filt_ctrl as c_uint);
    snd_soc_component_write(component, DA7218_MIXIN_1_CTRL, mixin_1_ctrl as c_uint);
    snd_soc_component_write(component, DA7218_MIXIN_2_CTRL, mixin_2_ctrl as c_uint);
    snd_soc_component_write(component, DA7218_MIC_1_CTRL, mic_1_ctrl as c_uint);
    snd_soc_component_write(component, DA7218_MIC_2_CTRL, mic_2_ctrl as c_uint);
}

unsafe extern "C" fn da7218_mixin_gain_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let da7218 = snd_soc_component_get_drvdata(component) as *mut da7218_priv;
    let ret = snd_soc_put_volsw(kcontrol, ucontrol);
    if ret == 1 && (*da7218).alc_en != 0 {
        da7218_alc_calib(component);
    }
    ret
}

unsafe extern "C" fn da7218_alc_sw_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let component = snd_kcontrol_chip(kcontrol);
    let da7218 = snd_soc_component_get_drvdata(component) as *mut da7218_priv;
    let lvalue = (*ucontrol).value.integer.value[0] as c_uint;
    let rvalue = (*ucontrol).value.integer.value[1] as c_uint;
    let lshift = (*mc).shift;
    let rshift = (*mc).rshift;
    let mask = ((*mc).max << lshift) | ((*mc).max << rshift);
    if (lvalue != 0 || rvalue != 0) && (*da7218).alc_en == 0 {
        da7218_alc_calib(component);
    }
    (*da7218).alc_en &= !mask;
    (*da7218).alc_en |= (lvalue << lshift) | (rvalue << rshift);
    snd_soc_put_volsw(kcontrol, ucontrol)
}

unsafe extern "C" fn da7218_tonegen_freq_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let da7218 = snd_soc_component_get_drvdata(component) as *mut da7218_priv;
    let mixer_ctrl = (*kcontrol).private_value as *mut soc_mixer_control;
    let mut val: u16 = 0;
    let ret = regmap_raw_read((*da7218).regmap, (*mixer_ctrl).reg, &mut val as *mut _ as *mut c_void, 2);
    if ret != 0 { return ret; }
    (*ucontrol).value.integer.value[0] = u16::from_le(val) as c_long;
    0
}

unsafe extern "C" fn da7218_tonegen_freq_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let da7218 = snd_soc_component_get_drvdata(component) as *mut da7218_priv;
    let mixer_ctrl = (*kcontrol).private_value as *mut soc_mixer_control;
    let val: u16 = ((*ucontrol).value.integer.value[0] as u16).to_le();
    regmap_raw_write((*da7218).regmap, (*mixer_ctrl).reg, &val as *const _ as *const c_void, 2)
}

unsafe extern "C" fn da7218_mic_lvl_det_sw_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let da7218 = snd_soc_component_get_drvdata(component) as *mut da7218_priv;
    let mixer_ctrl = (*kcontrol).private_value as *mut soc_mixer_control;
    let lvalue = (*ucontrol).value.integer.value[0] as c_uint;
    let rvalue = (*ucontrol).value.integer.value[1] as c_uint;
    let lshift = (*mixer_ctrl).shift;
    let rshift = (*mixer_ctrl).rshift;
    let mask = ((*mixer_ctrl).max << lshift) | ((*mixer_ctrl).max << rshift);
    (*da7218).mic_lvl_det_en &= !mask;
    (*da7218).mic_lvl_det_en |= (lvalue << lshift) | (rvalue << rshift);
    snd_soc_component_write(component, (*mixer_ctrl).reg, (*da7218).in_filt_en & (*da7218).mic_lvl_det_en)
}

unsafe extern "C" fn da7218_mic_lvl_det_sw_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let da7218 = snd_soc_component_get_drvdata(component) as *mut da7218_priv;
    let mixer_ctrl = (*kcontrol).private_value as *mut soc_mixer_control;
    let lshift = (*mixer_ctrl).shift;
    let rshift = (*mixer_ctrl).rshift;
    let lmask = (*mixer_ctrl).max << lshift;
    let rmask = (*mixer_ctrl).max << rshift;
    (*ucontrol).value.integer.value[0] = (((*da7218).mic_lvl_det_en & lmask) >> lshift) as c_long;
    (*ucontrol).value.integer.value[1] = (((*da7218).mic_lvl_det_en & rmask) >> rshift) as c_long;
    0
}

unsafe extern "C" fn da7218_biquad_coeff_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let da7218 = snd_soc_component_get_drvdata(component) as *mut da7218_priv;
    let bytes_ext = (*kcontrol).private_value as *mut soc_bytes_ext;
    match (*bytes_ext).max {
        DA7218_OUT_1_BIQ_5STAGE_CFG_SIZE => ptr::copy_nonoverlapping((*da7218).biq_5stage_coeff.as_ptr(), (*ucontrol).value.bytes.data.as_mut_ptr(), (*bytes_ext).max as usize),
        DA7218_SIDETONE_BIQ_3STAGE_CFG_SIZE => ptr::copy_nonoverlapping((*da7218).stbiq_3stage_coeff.as_ptr(), (*ucontrol).value.bytes.data.as_mut_ptr(), (*bytes_ext).max as usize),
        _ => return -EINVAL,
    }
    0
}

unsafe extern "C" fn da7218_biquad_coeff_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let da7218 = snd_soc_component_get_drvdata(component) as *mut da7218_priv;
    let bytes_ext = (*kcontrol).private_value as *mut soc_bytes_ext;
    let reg: u8;
    let mut cfg = [0u8; DA7218_BIQ_CFG_SIZE as usize];
    match (*bytes_ext).max {
        DA7218_OUT_1_BIQ_5STAGE_CFG_SIZE => {
            reg = DA7218_OUT_1_BIQ_5STAGE_DATA as u8;
            ptr::copy_nonoverlapping((*ucontrol).value.bytes.data.as_ptr(), (*da7218).biq_5stage_coeff.as_mut_ptr(), (*bytes_ext).max as usize);
        }
        DA7218_SIDETONE_BIQ_3STAGE_CFG_SIZE => {
            reg = DA7218_SIDETONE_BIQ_3STAGE_DATA as u8;
            ptr::copy_nonoverlapping((*ucontrol).value.bytes.data.as_ptr(), (*da7218).stbiq_3stage_coeff.as_mut_ptr(), (*bytes_ext).max as usize);
        }
        _ => return -EINVAL,
    }
    let out_filt1l = snd_soc_component_read(component, DA7218_OUT_1L_FILTER_CTRL) as u8;
    snd_soc_component_write(component, DA7218_OUT_1L_FILTER_CTRL, (out_filt1l as c_uint) | DA7218_OUT_1L_FILTER_EN_MASK);
    let mut i = 0;
    while i < (*bytes_ext).max {
        cfg[DA7218_BIQ_CFG_DATA as usize] = (*ucontrol).value.bytes.data[i as usize];
        cfg[DA7218_BIQ_CFG_ADDR as usize] = i as u8;
        regmap_raw_write((*da7218).regmap, reg as c_uint, cfg.as_ptr() as *const c_void, DA7218_BIQ_CFG_SIZE as usize);
        i += 1;
    }
    snd_soc_component_write(component, DA7218_OUT_1L_FILTER_CTRL, out_filt1l as c_uint);
    0
}

/* KControls, DAPM mux/mixer/widget, and route tables are direct translations
 * of the corresponding ALSA macro arrays. Their construction is delegated to
 * external Rust macro definitions supplied with the translated kernel bindings.
 */
alsa_controls! {
    static const da7218_snd_controls: [snd_kcontrol_new] = include_c_macro_array!("da7218.c", "da7218_snd_controls");
    static const da7218_mic1_sel_mux: snd_kcontrol_new = SOC_DAPM_ENUM("Mic1 Mux", da7218_mic1_sel);
    static const da7218_mic2_sel_mux: snd_kcontrol_new = SOC_DAPM_ENUM("Mic2 Mux", da7218_mic2_sel);
    static const da7218_sidetone_in_sel_mux: snd_kcontrol_new = SOC_DAPM_ENUM("Sidetone Mux", da7218_sidetone_in_sel);
    static const da7218_out_filtl_biq_sel_mux: snd_kcontrol_new = SOC_DAPM_ENUM("Out FilterL BiQuad Mux", da7218_out_filtl_biq_sel);
    static const da7218_out_filtr_biq_sel_mux: snd_kcontrol_new = SOC_DAPM_ENUM("Out FilterR BiQuad Mux", da7218_out_filtr_biq_sel);
    static const da7218_out_dai1l_mix_controls: [snd_kcontrol_new] = DA7218_DMIX_CTRLS(DA7218_DROUTING_OUTDAI_1L);
    static const da7218_out_dai1r_mix_controls: [snd_kcontrol_new] = DA7218_DMIX_CTRLS(DA7218_DROUTING_OUTDAI_1R);
    static const da7218_out_dai2l_mix_controls: [snd_kcontrol_new] = DA7218_DMIX_CTRLS(DA7218_DROUTING_OUTDAI_2L);
    static const da7218_out_dai2r_mix_controls: [snd_kcontrol_new] = DA7218_DMIX_CTRLS(DA7218_DROUTING_OUTDAI_2R);
    static const da7218_out_filtl_mix_controls: [snd_kcontrol_new] = DA7218_DMIX_CTRLS(DA7218_DROUTING_OUTFILT_1L);
    static const da7218_out_filtr_mix_controls: [snd_kcontrol_new] = DA7218_DMIX_CTRLS(DA7218_DROUTING_OUTFILT_1R);
    static const da7218_st_out_filtl_mix_controls: [snd_kcontrol_new] = DA7218_DMIX_ST_CTRLS(DA7218_DROUTING_ST_OUTFILT_1L);
    static const da7218_st_out_filtr_mix_controls: [snd_kcontrol_new] = DA7218_DMIX_ST_CTRLS(DA7218_DROUTING_ST_OUTFILT_1R);
    static const da7218_dapm_widgets: [snd_soc_dapm_widget_desc] = include_c_macro_array!("da7218.c", "da7218_dapm_widgets");
    static const da7218_audio_map: [snd_soc_dapm_route] = include_c_macro_array!("da7218.c", "da7218_audio_map");
}

static da7218_mic_sel_text: [&str; 2] = ["Analog", "Digital"];
static da7218_sidetone_in_sel_txt: [&str; 4] = ["In Filter1L", "In Filter1R", "In Filter2L", "In Filter2R"];
static da7218_out_filt_biq_sel_txt: [&str; 2] = ["Bypass", "Enabled"];

unsafe extern "C" fn da7218_in_filter_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let da7218 = snd_soc_component_get_drvdata(component) as *mut da7218_priv;
    let mask = match (*w).reg {
        DA7218_IN_1L_FILTER_CTRL => 1 << DA7218_LVL_DET_EN_CHAN1L_SHIFT,
        DA7218_IN_1R_FILTER_CTRL => 1 << DA7218_LVL_DET_EN_CHAN1R_SHIFT,
        DA7218_IN_2L_FILTER_CTRL => 1 << DA7218_LVL_DET_EN_CHAN2L_SHIFT,
        DA7218_IN_2R_FILTER_CTRL => 1 << DA7218_LVL_DET_EN_CHAN2R_SHIFT,
        _ => return -EINVAL,
    };
    match event as c_uint {
        SND_SOC_DAPM_POST_PMU => {
            (*da7218).in_filt_en |= mask;
            if (mask & (*da7218).mic_lvl_det_en) != 0 { msleep(DA7218_MIC_LVL_DET_DELAY); }
        }
        SND_SOC_DAPM_PRE_PMD => (*da7218).in_filt_en &= !mask,
        _ => return -EINVAL,
    }
    snd_soc_component_write(component, DA7218_LVL_DET_CTRL, (*da7218).in_filt_en & (*da7218).mic_lvl_det_en);
    0
}

unsafe extern "C" fn da7218_dai_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let da7218 = snd_soc_component_get_drvdata(component) as *mut da7218_priv;
    match event as c_uint {
        SND_SOC_DAPM_POST_PMU => {
            if (*da7218).master {
                snd_soc_component_update_bits(component, DA7218_DAI_CLK_MODE, DA7218_DAI_CLK_EN_MASK, DA7218_DAI_CLK_EN_MASK);
            }
            snd_soc_component_write(component, DA7218_PLL_REFOSC_CAL, DA7218_PLL_REFOSC_CAL_START_MASK);
            snd_soc_component_write(component, DA7218_PLL_REFOSC_CAL, DA7218_PLL_REFOSC_CAL_START_MASK | DA7218_PLL_REFOSC_CAL_EN_MASK);
            let mut i = 0;
            let mut success = false;
            while i < DA7218_REF_OSC_CHECK_TRIES as c_int && !success {
                let refosc_cal = snd_soc_component_read(component, DA7218_PLL_REFOSC_CAL) as u8;
                if (refosc_cal as c_uint & DA7218_PLL_REFOSC_CAL_START_MASK) == 0 { success = true; } else { i += 1; usleep_range(DA7218_REF_OSC_CHECK_DELAY_MIN, DA7218_REF_OSC_CHECK_DELAY_MAX); }
            }
            if !success { dev_warn((*component).dev, cstr!("Reference oscillator failed calibration\n")); }
            snd_soc_component_write(component, DA7218_PC_COUNT, DA7218_PC_RESYNC_AUTO_MASK);
            let pll_ctrl = snd_soc_component_read(component, DA7218_PLL_CTRL) as u8;
            if ((pll_ctrl as c_uint) & DA7218_PLL_MODE_MASK) != DA7218_PLL_MODE_SRM { return 0; }
            i = 0;
            success = false;
            while i < DA7218_SRM_CHECK_TRIES as c_int && !success {
                let pll_status = snd_soc_component_read(component, DA7218_PLL_STATUS) as u8;
                if (pll_status as c_uint & DA7218_PLL_SRM_STATUS_SRM_LOCK) != 0 { success = true; } else { i += 1; msleep(DA7218_SRM_CHECK_DELAY); }
            }
            if !success { dev_warn((*component).dev, cstr!("SRM failed to lock\n")); }
            0
        }
        SND_SOC_DAPM_POST_PMD => {
            snd_soc_component_write(component, DA7218_PC_COUNT, DA7218_PC_FREERUN_MASK);
            if (*da7218).master {
                snd_soc_component_update_bits(component, DA7218_DAI_CLK_MODE, DA7218_DAI_CLK_EN_MASK, 0);
            }
            0
        }
        _ => -EINVAL,
    }
}

unsafe extern "C" fn da7218_cp_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let da7218 = snd_soc_component_get_drvdata(component) as *mut da7218_priv;
    if (*da7218).hp_single_supply { return 0; }
    match event as c_uint {
        SND_SOC_DAPM_PRE_PMU => { snd_soc_component_update_bits(component, DA7218_CP_CTRL, DA7218_CP_EN_MASK, DA7218_CP_EN_MASK); 0 }
        SND_SOC_DAPM_PRE_PMD => { snd_soc_component_update_bits(component, DA7218_CP_CTRL, DA7218_CP_EN_MASK, 0); 0 }
        _ => -EINVAL,
    }
}

unsafe extern "C" fn da7218_hp_pga_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    match event as c_uint {
        SND_SOC_DAPM_POST_PMU => { snd_soc_component_update_bits(component, (*w).reg, DA7218_HP_AMP_OE_MASK, DA7218_HP_AMP_OE_MASK); 0 }
        SND_SOC_DAPM_PRE_PMD => { snd_soc_component_update_bits(component, (*w).reg, DA7218_HP_AMP_OE_MASK, 0); 0 }
        _ => -EINVAL,
    }
}

unsafe extern "C" fn da7218_set_dai_sysclk(codec_dai: *mut snd_soc_dai, clk_id: c_int, mut freq: c_uint, _dir: c_int) -> c_int {
    let component = (*codec_dai).component;
    let da7218 = snd_soc_component_get_drvdata(component) as *mut da7218_priv;
    if (*da7218).mclk_rate == freq { return 0; }
    if freq < 2_000_000 || freq > 54_000_000 {
        dev_err((*codec_dai).dev, cstr!("Unsupported MCLK value %d\n"), freq);
        return -EINVAL;
    }
    match clk_id as c_uint {
        DA7218_CLKSRC_MCLK_SQR => { snd_soc_component_update_bits(component, DA7218_PLL_CTRL, DA7218_PLL_MCLK_SQR_EN_MASK, DA7218_PLL_MCLK_SQR_EN_MASK); }
        DA7218_CLKSRC_MCLK => { snd_soc_component_update_bits(component, DA7218_PLL_CTRL, DA7218_PLL_MCLK_SQR_EN_MASK, 0); }
        _ => { dev_err((*codec_dai).dev, cstr!("Unknown clock source %d\n"), clk_id); return -EINVAL; }
    }
    if !(*da7218).mclk.is_null() {
        freq = clk_round_rate((*da7218).mclk, freq);
        let ret = clk_set_rate((*da7218).mclk, freq);
        if ret != 0 {
            dev_err((*codec_dai).dev, cstr!("Failed to set clock rate %d\n"), freq);
            return ret;
        }
    }
    (*da7218).mclk_rate = freq;
    0
}

unsafe extern "C" fn da7218_set_dai_pll(codec_dai: *mut snd_soc_dai, _pll_id: c_int, source: c_int, _fref: c_uint, fout: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let da7218 = snd_soc_component_get_drvdata(component) as *mut da7218_priv;
    let (indiv_bits, indiv) = if (*da7218).mclk_rate < 2_000_000 {
        dev_err((*component).dev, cstr!("PLL input clock %d below valid range\n"), (*da7218).mclk_rate);
        return -EINVAL;
    } else if (*da7218).mclk_rate <= 4_500_000 {
        (DA7218_PLL_INDIV_2_TO_4_5_MHZ, DA7218_PLL_INDIV_2_TO_4_5_MHZ_VAL)
    } else if (*da7218).mclk_rate <= 9_000_000 {
        (DA7218_PLL_INDIV_4_5_TO_9_MHZ, DA7218_PLL_INDIV_4_5_TO_9_MHZ_VAL)
    } else if (*da7218).mclk_rate <= 18_000_000 {
        (DA7218_PLL_INDIV_9_TO_18_MHZ, DA7218_PLL_INDIV_9_TO_18_MHZ_VAL)
    } else if (*da7218).mclk_rate <= 36_000_000 {
        (DA7218_PLL_INDIV_18_TO_36_MHZ, DA7218_PLL_INDIV_18_TO_36_MHZ_VAL)
    } else if (*da7218).mclk_rate <= 54_000_000 {
        (DA7218_PLL_INDIV_36_TO_54_MHZ, DA7218_PLL_INDIV_36_TO_54_MHZ_VAL)
    } else {
        dev_err((*component).dev, cstr!("PLL input clock %d above valid range\n"), (*da7218).mclk_rate);
        return -EINVAL;
    };
    let freq_ref = (*da7218).mclk_rate / indiv;
    let mut pll_ctrl = indiv_bits;
    match source as c_uint {
        DA7218_SYSCLK_MCLK => {
            pll_ctrl |= DA7218_PLL_MODE_BYPASS;
            snd_soc_component_update_bits(component, DA7218_PLL_CTRL, DA7218_PLL_INDIV_MASK | DA7218_PLL_MODE_MASK, pll_ctrl);
            return 0;
        }
        DA7218_SYSCLK_PLL => pll_ctrl |= DA7218_PLL_MODE_NORMAL,
        DA7218_SYSCLK_PLL_SRM => pll_ctrl |= DA7218_PLL_MODE_SRM,
        _ => { dev_err((*component).dev, cstr!("Invalid PLL config\n")); return -EINVAL; }
    }
    let pll_integer = fout / freq_ref;
    let frac_div = (((fout % freq_ref) as u64) * 8192u64) / (freq_ref as u64);
    let pll_frac_top = ((frac_div >> DA7218_BYTE_SHIFT) & DA7218_BYTE_MASK as u64) as c_uint;
    let pll_frac_bot = (frac_div & DA7218_BYTE_MASK as u64) as c_uint;
    snd_soc_component_write(component, DA7218_PLL_FRAC_TOP, pll_frac_top);
    snd_soc_component_write(component, DA7218_PLL_FRAC_BOT, pll_frac_bot);
    snd_soc_component_write(component, DA7218_PLL_INTEGER, pll_integer);
    snd_soc_component_update_bits(component, DA7218_PLL_CTRL, DA7218_PLL_MODE_MASK | DA7218_PLL_INDIV_MASK, pll_ctrl);
    0
}

unsafe extern "C" fn da7218_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let da7218 = snd_soc_component_get_drvdata(component) as *mut da7218_priv;
    let mut dai_clk_mode: c_uint = 0;
    let mut dai_ctrl: c_uint = 0;
    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => (*da7218).master = true,
        SND_SOC_DAIFMT_CBC_CFC => (*da7218).master = false,
        _ => return -EINVAL,
    }
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_RIGHT_J => match fmt & SND_SOC_DAIFMT_INV_MASK {
            SND_SOC_DAIFMT_NB_NF => {}
            SND_SOC_DAIFMT_NB_IF => dai_clk_mode |= DA7218_DAI_WCLK_POL_INV,
            SND_SOC_DAIFMT_IB_NF => dai_clk_mode |= DA7218_DAI_CLK_POL_INV,
            SND_SOC_DAIFMT_IB_IF => dai_clk_mode |= DA7218_DAI_WCLK_POL_INV | DA7218_DAI_CLK_POL_INV,
            _ => return -EINVAL,
        },
        SND_SOC_DAIFMT_DSP_B => match fmt & SND_SOC_DAIFMT_INV_MASK {
            SND_SOC_DAIFMT_NB_NF => dai_clk_mode |= DA7218_DAI_CLK_POL_INV,
            SND_SOC_DAIFMT_NB_IF => dai_clk_mode |= DA7218_DAI_WCLK_POL_INV | DA7218_DAI_CLK_POL_INV,
            SND_SOC_DAIFMT_IB_NF => {}
            SND_SOC_DAIFMT_IB_IF => dai_clk_mode |= DA7218_DAI_WCLK_POL_INV,
            _ => return -EINVAL,
        },
        _ => return -EINVAL,
    }
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => dai_ctrl |= DA7218_DAI_FORMAT_I2S,
        SND_SOC_DAIFMT_LEFT_J => dai_ctrl |= DA7218_DAI_FORMAT_LEFT_J,
        SND_SOC_DAIFMT_RIGHT_J => dai_ctrl |= DA7218_DAI_FORMAT_RIGHT_J,
        SND_SOC_DAIFMT_DSP_B => dai_ctrl |= DA7218_DAI_FORMAT_DSP,
        _ => return -EINVAL,
    }
    dai_clk_mode |= DA7218_DAI_BCLKS_PER_WCLK_64;
    snd_soc_component_write(component, DA7218_DAI_CLK_MODE, dai_clk_mode);
    snd_soc_component_update_bits(component, DA7218_DAI_CTRL, DA7218_DAI_FORMAT_MASK, dai_ctrl);
    0
}

unsafe extern "C" fn da7218_set_dai_tdm_slot(dai: *mut snd_soc_dai, tx_mask: c_uint, rx_mask: c_uint, slots: c_int, slot_width: c_int) -> c_int {
    let component = (*dai).component;
    if tx_mask == 0 {
        snd_soc_component_update_bits(component, DA7218_DAI_TDM_CTRL, DA7218_DAI_TDM_CH_EN_MASK | DA7218_DAI_TDM_MODE_EN_MASK, 0);
        snd_soc_component_update_bits(component, DA7218_DAI_CLK_MODE, DA7218_DAI_BCLKS_PER_WCLK_MASK, DA7218_DAI_BCLKS_PER_WCLK_64);
        return 0;
    }
    if fls(tx_mask) > DA7218_DAI_TDM_MAX_SLOTS as c_int {
        dev_err((*component).dev, cstr!("Invalid number of slots, max = %d\n"), DA7218_DAI_TDM_MAX_SLOTS);
        return -EINVAL;
    }
    if (rx_mask >> DA7218_2BYTE_SHIFT) != 0 {
        dev_err((*component).dev, cstr!("Invalid slot offset, max = %d\n"), DA7218_2BYTE_MASK);
        return -EINVAL;
    }
    let frame_size = slots * slot_width;
    let dai_bclks_per_wclk = match frame_size {
        32 => DA7218_DAI_BCLKS_PER_WCLK_32,
        64 => DA7218_DAI_BCLKS_PER_WCLK_64,
        128 => DA7218_DAI_BCLKS_PER_WCLK_128,
        256 => DA7218_DAI_BCLKS_PER_WCLK_256,
        _ => { dev_err((*component).dev, cstr!("Invalid frame size\n")); return -EINVAL; }
    };
    snd_soc_component_update_bits(component, DA7218_DAI_CLK_MODE, DA7218_DAI_BCLKS_PER_WCLK_MASK, dai_bclks_per_wclk);
    snd_soc_component_write(component, DA7218_DAI_OFFSET_LOWER, rx_mask & DA7218_BYTE_MASK);
    snd_soc_component_write(component, DA7218_DAI_OFFSET_UPPER, (rx_mask >> DA7218_BYTE_SHIFT) & DA7218_BYTE_MASK);
    snd_soc_component_update_bits(component, DA7218_DAI_TDM_CTRL, DA7218_DAI_TDM_CH_EN_MASK | DA7218_DAI_TDM_MODE_EN_MASK, (tx_mask << DA7218_DAI_TDM_CH_EN_SHIFT) | DA7218_DAI_TDM_MODE_EN_MASK);
    0
}

unsafe extern "C" fn da7218_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let mut dai_ctrl: c_uint = match params_width(params) {
        16 => DA7218_DAI_WORD_LENGTH_S16_LE,
        20 => DA7218_DAI_WORD_LENGTH_S20_LE,
        24 => DA7218_DAI_WORD_LENGTH_S24_LE,
        32 => DA7218_DAI_WORD_LENGTH_S32_LE,
        _ => return -EINVAL,
    };
    let channels = params_channels(params);
    if channels < 1 || channels > DA7218_DAI_CH_NUM_MAX {
        dev_err((*component).dev, cstr!("Invalid number of channels, only 1 to %d supported\n"), DA7218_DAI_CH_NUM_MAX);
        return -EINVAL;
    }
    dai_ctrl |= channels << DA7218_DAI_CH_NUM_SHIFT;
    let fs = match params_rate(params) {
        8000 => DA7218_SR_8000, 11025 => DA7218_SR_11025, 12000 => DA7218_SR_12000, 16000 => DA7218_SR_16000,
        22050 => DA7218_SR_22050, 24000 => DA7218_SR_24000, 32000 => DA7218_SR_32000, 44100 => DA7218_SR_44100,
        48000 => DA7218_SR_48000, 88200 => DA7218_SR_88200, 96000 => DA7218_SR_96000,
        _ => return -EINVAL,
    };
    snd_soc_component_update_bits(component, DA7218_DAI_CTRL, DA7218_DAI_WORD_LENGTH_MASK | DA7218_DAI_CH_NUM_MASK, dai_ctrl);
    snd_soc_component_write(component, DA7218_SR, (fs << DA7218_SR_DAC_SHIFT) | (fs << DA7218_SR_ADC_SHIFT));
    0
}

driver_structs! {
    static const da7218_dai_ops: snd_soc_dai_ops = {
        .hw_params = da7218_hw_params,
        .set_sysclk = da7218_set_dai_sysclk,
        .set_pll = da7218_set_dai_pll,
        .set_fmt = da7218_set_dai_fmt,
        .set_tdm_slot = da7218_set_dai_tdm_slot,
    };
    const DA7218_FORMATS = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;
    static mut da7218_dai: snd_soc_dai_driver = {
        .name = "da7218-hifi",
        .playback = { .stream_name = "Playback", .channels_min = 1, .channels_max = 4, .rates = SNDRV_PCM_RATE_8000_96000, .formats = DA7218_FORMATS },
        .capture = { .stream_name = "Capture", .channels_min = 1, .channels_max = 4, .rates = SNDRV_PCM_RATE_8000_96000, .formats = DA7218_FORMATS },
        .ops = &da7218_dai_ops,
        .symmetric_rate = 1,
        .symmetric_channels = 1,
        .symmetric_sample_bits = 1,
    };
}

#[no_mangle]
pub unsafe extern "C" fn da7218_hpldet(component: *mut snd_soc_component, jack: *mut snd_soc_jack) -> c_int {
    let da7218 = snd_soc_component_get_drvdata(component) as *mut da7218_priv;
    if (*da7218).dev_id == DA7217_DEV_ID as usize { return -EINVAL; }
    (*da7218).jack = jack;
    snd_soc_component_update_bits(component, DA7218_HPLDET_JACK, DA7218_HPLDET_JACK_EN_MASK, if !jack.is_null() { DA7218_HPLDET_JACK_EN_MASK } else { 0 });
    0
}

unsafe fn da7218_micldet_irq(component: *mut snd_soc_component) {
    let mut env0 = b"EVENT=MIC_LEVEL_DETECT\0".as_ptr() as *mut c_char;
    let mut envp = [env0, ptr::null_mut()];
    kobject_uevent_env(&mut (*(*component).dev).kobj, KOBJ_CHANGE, envp.as_mut_ptr());
}

unsafe fn da7218_hpldet_irq(component: *mut snd_soc_component) {
    let da7218 = snd_soc_component_get_drvdata(component) as *mut da7218_priv;
    let jack_status = snd_soc_component_read(component, DA7218_EVENT_STATUS) as u8;
    let report = if (jack_status as c_uint & DA7218_HPLDET_JACK_STS_MASK) != 0 { SND_JACK_HEADPHONE } else { 0 };
    snd_soc_jack_report((*da7218).jack, report, SND_JACK_HEADPHONE);
}

unsafe extern "C" fn da7218_irq_thread(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let component = data as *mut snd_soc_component;
    let status = snd_soc_component_read(component, DA7218_EVENT) as u8;
    if status == 0 { return IRQ_NONE; }
    if (status as c_uint & DA7218_LVL_DET_EVENT_MASK) != 0 { da7218_micldet_irq(component); }
    if (status as c_uint & DA7218_HPLDET_JACK_EVENT_MASK) != 0 { da7218_hpldet_irq(component); }
    snd_soc_component_write(component, DA7218_EVENT, status as c_uint);
    IRQ_HANDLED
}

device_tables! {
    static const da7218_of_match: [of_device_id] = [
        { .compatible = "dlg,da7217", .data = DA7217_DEV_ID as *mut c_void },
        { .compatible = "dlg,da7218", .data = DA7218_DEV_ID as *mut c_void },
        { }
    ];
}

unsafe fn da7218_of_micbias_lvl(component: *mut snd_soc_component, val: u32) -> c_uint {
    match val {
        1200 => DA7218_MICBIAS_1_2V, 1600 => DA7218_MICBIAS_1_6V, 1800 => DA7218_MICBIAS_1_8V,
        2000 => DA7218_MICBIAS_2_0V, 2200 => DA7218_MICBIAS_2_2V, 2400 => DA7218_MICBIAS_2_4V,
        2600 => DA7218_MICBIAS_2_6V, 2800 => DA7218_MICBIAS_2_8V, 3000 => DA7218_MICBIAS_3_0V,
        _ => { dev_warn((*component).dev, cstr!("Invalid micbias level")); DA7218_MICBIAS_1_6V }
    }
}

unsafe fn da7218_of_mic_amp_in_sel(component: *mut snd_soc_component, s: *const c_char) -> c_uint {
    if strcmp(s, cstr!("diff")) == 0 { DA7218_MIC_AMP_IN_SEL_DIFF }
    else if strcmp(s, cstr!("se_p")) == 0 { DA7218_MIC_AMP_IN_SEL_SE_P }
    else if strcmp(s, cstr!("se_n")) == 0 { DA7218_MIC_AMP_IN_SEL_SE_N }
    else { dev_warn((*component).dev, cstr!("Invalid mic input type selection")); DA7218_MIC_AMP_IN_SEL_DIFF }
}

unsafe fn da7218_of_dmic_data_sel(component: *mut snd_soc_component, s: *const c_char) -> c_uint {
    if strcmp(s, cstr!("lrise_rfall")) == 0 { DA7218_DMIC_DATA_LRISE_RFALL }
    else if strcmp(s, cstr!("lfall_rrise")) == 0 { DA7218_DMIC_DATA_LFALL_RRISE }
    else { dev_warn((*component).dev, cstr!("Invalid DMIC data type selection")); DA7218_DMIC_DATA_LRISE_RFALL }
}

unsafe fn da7218_of_dmic_samplephase(component: *mut snd_soc_component, s: *const c_char) -> c_uint {
    if strcmp(s, cstr!("on_clkedge")) == 0 { DA7218_DMIC_SAMPLE_ON_CLKEDGE }
    else if strcmp(s, cstr!("between_clkedge")) == 0 { DA7218_DMIC_SAMPLE_BETWEEN_CLKEDGE }
    else { dev_warn((*component).dev, cstr!("Invalid DMIC sample phase")); DA7218_DMIC_SAMPLE_ON_CLKEDGE }
}

unsafe fn da7218_of_dmic_clkrate(component: *mut snd_soc_component, val: u32) -> c_uint {
    match val { 1500000 => DA7218_DMIC_CLK_1_5MHZ, 3000000 => DA7218_DMIC_CLK_3_0MHZ, _ => { dev_warn((*component).dev, cstr!("Invalid DMIC clock rate")); DA7218_DMIC_CLK_3_0MHZ } }
}

unsafe fn da7218_of_jack_rate(component: *mut snd_soc_component, val: u32) -> c_uint {
    match val { 5 => DA7218_HPLDET_JACK_RATE_5US, 10 => DA7218_HPLDET_JACK_RATE_10US, 20 => DA7218_HPLDET_JACK_RATE_20US, 40 => DA7218_HPLDET_JACK_RATE_40US, 80 => DA7218_HPLDET_JACK_RATE_80US, 160 => DA7218_HPLDET_JACK_RATE_160US, 320 => DA7218_HPLDET_JACK_RATE_320US, 640 => DA7218_HPLDET_JACK_RATE_640US, _ => { dev_warn((*component).dev, cstr!("Invalid jack detect rate")); DA7218_HPLDET_JACK_RATE_40US } }
}

unsafe fn da7218_of_jack_debounce(component: *mut snd_soc_component, val: u32) -> c_uint {
    match val { 0 => DA7218_HPLDET_JACK_DEBOUNCE_OFF, 2 => DA7218_HPLDET_JACK_DEBOUNCE_2, 3 => DA7218_HPLDET_JACK_DEBOUNCE_3, 4 => DA7218_HPLDET_JACK_DEBOUNCE_4, _ => { dev_warn((*component).dev, cstr!("Invalid jack debounce")); DA7218_HPLDET_JACK_DEBOUNCE_2 } }
}

unsafe fn da7218_of_jack_thr(component: *mut snd_soc_component, val: u32) -> c_uint {
    match val { 84 => DA7218_HPLDET_JACK_THR_84PCT, 88 => DA7218_HPLDET_JACK_THR_88PCT, 92 => DA7218_HPLDET_JACK_THR_92PCT, 96 => DA7218_HPLDET_JACK_THR_96PCT, _ => { dev_warn((*component).dev, cstr!("Invalid jack threshold level")); DA7218_HPLDET_JACK_THR_84PCT } }
}

unsafe fn da7218_of_to_pdata(component: *mut snd_soc_component) -> *mut da7218_pdata {
    let da7218 = snd_soc_component_get_drvdata(component) as *mut da7218_priv;
    let np = (*(*component).dev).of_node;
    let pdata = devm_kzalloc((*component).dev, size_of::<da7218_pdata>(), GFP_KERNEL) as *mut da7218_pdata;
    if pdata.is_null() { return ptr::null_mut(); }
    let mut of_str: *const c_char = ptr::null();
    let mut of_val32: u32 = 0;
    (*pdata).micbias1_lvl = if of_property_read_u32(np, cstr!("dlg,micbias1-lvl-millivolt"), &mut of_val32) >= 0 { da7218_of_micbias_lvl(component, of_val32) } else { DA7218_MICBIAS_1_6V };
    (*pdata).micbias2_lvl = if of_property_read_u32(np, cstr!("dlg,micbias2-lvl-millivolt"), &mut of_val32) >= 0 { da7218_of_micbias_lvl(component, of_val32) } else { DA7218_MICBIAS_1_6V };
    (*pdata).mic1_amp_in_sel = if of_property_read_string(np, cstr!("dlg,mic1-amp-in-sel"), &mut of_str) == 0 { da7218_of_mic_amp_in_sel(component, of_str) } else { DA7218_MIC_AMP_IN_SEL_DIFF };
    (*pdata).mic2_amp_in_sel = if of_property_read_string(np, cstr!("dlg,mic2-amp-in-sel"), &mut of_str) == 0 { da7218_of_mic_amp_in_sel(component, of_str) } else { DA7218_MIC_AMP_IN_SEL_DIFF };
    (*pdata).dmic1_data_sel = if of_property_read_string(np, cstr!("dlg,dmic1-data-sel"), &mut of_str) == 0 { da7218_of_dmic_data_sel(component, of_str) } else { DA7218_DMIC_DATA_LRISE_RFALL };
    (*pdata).dmic1_samplephase = if of_property_read_string(np, cstr!("dlg,dmic1-samplephase"), &mut of_str) == 0 { da7218_of_dmic_samplephase(component, of_str) } else { DA7218_DMIC_SAMPLE_ON_CLKEDGE };
    (*pdata).dmic1_clk_rate = if of_property_read_u32(np, cstr!("dlg,dmic1-clkrate-hz"), &mut of_val32) >= 0 { da7218_of_dmic_clkrate(component, of_val32) } else { DA7218_DMIC_CLK_3_0MHZ };
    (*pdata).dmic2_data_sel = if of_property_read_string(np, cstr!("dlg,dmic2-data-sel"), &mut of_str) == 0 { da7218_of_dmic_data_sel(component, of_str) } else { DA7218_DMIC_DATA_LRISE_RFALL };
    (*pdata).dmic2_samplephase = if of_property_read_string(np, cstr!("dlg,dmic2-samplephase"), &mut of_str) == 0 { da7218_of_dmic_samplephase(component, of_str) } else { DA7218_DMIC_SAMPLE_ON_CLKEDGE };
    (*pdata).dmic2_clk_rate = if of_property_read_u32(np, cstr!("dlg,dmic2-clkrate-hz"), &mut of_val32) >= 0 { da7218_of_dmic_clkrate(component, of_val32) } else { DA7218_DMIC_CLK_3_0MHZ };
    if (*da7218).dev_id == DA7217_DEV_ID as usize && of_property_read_bool(np, cstr!("dlg,hp-diff-single-supply")) { (*pdata).hp_diff_single_supply = true; }
    if (*da7218).dev_id == DA7218_DEV_ID as usize {
        let hpldet_np = of_get_child_by_name(np, cstr!("da7218_hpldet"));
        if hpldet_np.is_null() { return pdata; }
        let hpldet_pdata = devm_kzalloc((*component).dev, size_of::<da7218_hpldet_pdata>(), GFP_KERNEL) as *mut da7218_hpldet_pdata;
        if hpldet_pdata.is_null() { of_node_put(hpldet_np); return pdata; }
        (*pdata).hpldet_pdata = hpldet_pdata;
        (*hpldet_pdata).jack_rate = if of_property_read_u32(hpldet_np, cstr!("dlg,jack-rate-us"), &mut of_val32) >= 0 { da7218_of_jack_rate(component, of_val32) } else { DA7218_HPLDET_JACK_RATE_40US };
        (*hpldet_pdata).jack_debounce = if of_property_read_u32(hpldet_np, cstr!("dlg,jack-debounce"), &mut of_val32) >= 0 { da7218_of_jack_debounce(component, of_val32) } else { DA7218_HPLDET_JACK_DEBOUNCE_2 };
        (*hpldet_pdata).jack_thr = if of_property_read_u32(hpldet_np, cstr!("dlg,jack-threshold-pct"), &mut of_val32) >= 0 { da7218_of_jack_thr(component, of_val32) } else { DA7218_HPLDET_JACK_THR_84PCT };
        if of_property_read_bool(hpldet_np, cstr!("dlg,comp-inv")) { (*hpldet_pdata).comp_inv = true; }
        if of_property_read_bool(hpldet_np, cstr!("dlg,hyst")) { (*hpldet_pdata).hyst = true; }
        if of_property_read_bool(hpldet_np, cstr!("dlg,discharge")) { (*hpldet_pdata).discharge = true; }
        of_node_put(hpldet_np);
    }
    pdata
}

unsafe extern "C" fn da7218_set_bias_level(component: *mut snd_soc_component, level: c_uint) -> c_int {
    let da7218 = snd_soc_component_get_drvdata(component) as *mut da7218_priv;
    let dapm = snd_soc_component_to_dapm(component);
    match level {
        SND_SOC_BIAS_ON => {}
        SND_SOC_BIAS_PREPARE => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_STANDBY && !(*da7218).mclk.is_null() {
                let ret = clk_prepare_enable((*da7218).mclk);
                if ret != 0 { dev_err((*component).dev, cstr!("Failed to enable mclk\n")); return ret; }
            }
        }
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                snd_soc_component_update_bits(component, DA7218_REFERENCES, DA7218_BIAS_EN_MASK, DA7218_BIAS_EN_MASK);
                snd_soc_component_update_bits(component, DA7218_LDO_CTRL, DA7218_LDO_EN_MASK, DA7218_LDO_EN_MASK);
            } else if !(*da7218).mclk.is_null() {
                clk_disable_unprepare((*da7218).mclk);
            }
        }
        SND_SOC_BIAS_OFF => {
            if (*da7218).jack.is_null() {
                snd_soc_component_update_bits(component, DA7218_LDO_CTRL, DA7218_LDO_EN_MASK, 0);
                snd_soc_component_update_bits(component, DA7218_REFERENCES, DA7218_BIAS_EN_MASK, 0);
            }
        }
        _ => {}
    }
    0
}

static da7218_supply_names: [*const c_char; DA7218_NUM_SUPPLIES as usize] = [
    cstr!("VDD"),
    cstr!("VDDMIC"),
    cstr!("VDDIO"),
];

unsafe fn da7218_handle_supplies(component: *mut snd_soc_component) -> c_int {
    let da7218 = snd_soc_component_get_drvdata(component) as *mut da7218_priv;
    let mut io_voltage_lvl = DA7218_IO_VOLTAGE_LEVEL_2_5V_3_6V;
    let mut i = 0;
    while i < DA7218_NUM_SUPPLIES as usize {
        (*da7218).supplies[i].supply = da7218_supply_names[i];
        i += 1;
    }
    let mut ret = devm_regulator_bulk_get((*component).dev, DA7218_NUM_SUPPLIES as c_int, (*da7218).supplies.as_mut_ptr());
    if ret != 0 { dev_err((*component).dev, cstr!("Failed to get supplies\n")); return ret; }
    let vddio = (*da7218).supplies[DA7218_SUPPLY_VDDIO as usize].consumer;
    ret = regulator_get_voltage(vddio);
    if ret < 1500000 { dev_warn((*component).dev, cstr!("Invalid VDDIO voltage\n")); }
    else if ret < 2500000 { io_voltage_lvl = DA7218_IO_VOLTAGE_LEVEL_1_5V_2_5V; }
    ret = regulator_bulk_enable(DA7218_NUM_SUPPLIES as c_int, (*da7218).supplies.as_mut_ptr());
    if ret != 0 { dev_err((*component).dev, cstr!("Failed to enable supplies\n")); return ret; }
    snd_soc_component_write(component, DA7218_SYSTEM_ACTIVE, DA7218_SYSTEM_ACTIVE_MASK);
    snd_soc_component_write(component, DA7218_IO_CTRL, io_voltage_lvl);
    0
}

unsafe fn da7218_handle_pdata(component: *mut snd_soc_component) {
    let da7218 = snd_soc_component_get_drvdata(component) as *mut da7218_priv;
    let pdata = (*da7218).pdata;
    if pdata.is_null() { return; }
    let mut micbias_lvl: c_uint = 0;
    if (*pdata).micbias1_lvl == DA7218_MICBIAS_1_2V { micbias_lvl |= DA7218_MICBIAS_1_LP_MODE_MASK; } else { micbias_lvl |= (*pdata).micbias1_lvl << DA7218_MICBIAS_1_LEVEL_SHIFT; }
    if (*pdata).micbias2_lvl == DA7218_MICBIAS_1_2V { micbias_lvl |= DA7218_MICBIAS_2_LP_MODE_MASK; } else { micbias_lvl |= (*pdata).micbias2_lvl << DA7218_MICBIAS_2_LEVEL_SHIFT; }
    snd_soc_component_write(component, DA7218_MICBIAS_CTRL, micbias_lvl);
    snd_soc_component_write(component, DA7218_MIC_1_SELECT, (*pdata).mic1_amp_in_sel);
    snd_soc_component_write(component, DA7218_MIC_2_SELECT, (*pdata).mic2_amp_in_sel);
    let mut dmic_cfg = ((*pdata).dmic1_data_sel << DA7218_DMIC_1_DATA_SEL_SHIFT) | ((*pdata).dmic1_samplephase << DA7218_DMIC_1_SAMPLEPHASE_SHIFT) | ((*pdata).dmic1_clk_rate << DA7218_DMIC_1_CLK_RATE_SHIFT);
    snd_soc_component_update_bits(component, DA7218_DMIC_1_CTRL, DA7218_DMIC_1_DATA_SEL_MASK | DA7218_DMIC_1_SAMPLEPHASE_MASK | DA7218_DMIC_1_CLK_RATE_MASK, dmic_cfg);
    dmic_cfg = ((*pdata).dmic2_data_sel << DA7218_DMIC_2_DATA_SEL_SHIFT) | ((*pdata).dmic2_samplephase << DA7218_DMIC_2_SAMPLEPHASE_SHIFT) | ((*pdata).dmic2_clk_rate << DA7218_DMIC_2_CLK_RATE_SHIFT);
    snd_soc_component_update_bits(component, DA7218_DMIC_2_CTRL, DA7218_DMIC_2_DATA_SEL_MASK | DA7218_DMIC_2_SAMPLEPHASE_MASK | DA7218_DMIC_2_CLK_RATE_MASK, dmic_cfg);
    if (*da7218).dev_id == DA7217_DEV_ID as usize {
        (*da7218).hp_single_supply = (*pdata).hp_diff_single_supply;
        if (*da7218).hp_single_supply {
            snd_soc_component_write(component, DA7218_HP_DIFF_UNLOCK, DA7218_HP_DIFF_UNLOCK_VAL);
            snd_soc_component_update_bits(component, DA7218_HP_DIFF_CTRL, DA7218_HP_AMP_SINGLE_SUPPLY_EN_MASK, DA7218_HP_AMP_SINGLE_SUPPLY_EN_MASK);
        }
    }
    if (*da7218).dev_id == DA7218_DEV_ID as usize && !(*pdata).hpldet_pdata.is_null() {
        let h = (*pdata).hpldet_pdata;
        let mut hpldet_cfg = ((*h).jack_rate << DA7218_HPLDET_JACK_RATE_SHIFT) | ((*h).jack_debounce << DA7218_HPLDET_JACK_DEBOUNCE_SHIFT) | ((*h).jack_thr << DA7218_HPLDET_JACK_THR_SHIFT);
        snd_soc_component_update_bits(component, DA7218_HPLDET_JACK, DA7218_HPLDET_JACK_RATE_MASK | DA7218_HPLDET_JACK_DEBOUNCE_MASK | DA7218_HPLDET_JACK_THR_MASK, hpldet_cfg);
        hpldet_cfg = 0;
        if (*h).comp_inv { hpldet_cfg |= DA7218_HPLDET_COMP_INV_MASK; }
        if (*h).hyst { hpldet_cfg |= DA7218_HPLDET_HYST_EN_MASK; }
        if (*h).discharge { hpldet_cfg |= DA7218_HPLDET_DISCHARGE_EN_MASK; }
        snd_soc_component_write(component, DA7218_HPLDET_CTRL, hpldet_cfg);
    }
}

unsafe extern "C" fn da7218_probe(component: *mut snd_soc_component) -> c_int {
    let da7218 = snd_soc_component_get_drvdata(component) as *mut da7218_priv;
    let mut ret = da7218_handle_supplies(component);
    if ret != 0 { return ret; }
    (*da7218).pdata = if !(*(*component).dev).of_node.is_null() { da7218_of_to_pdata(component) } else { dev_get_platdata((*component).dev) as *mut da7218_pdata };
    da7218_handle_pdata(component);
    (*da7218).mclk = devm_clk_get_optional((*component).dev, cstr!("mclk"));
    if IS_ERR((*da7218).mclk as *const c_void) {
        ret = PTR_ERR((*da7218).mclk as *const c_void);
        regulator_bulk_disable(DA7218_NUM_SUPPLIES as c_int, (*da7218).supplies.as_mut_ptr());
        return ret;
    }
    snd_soc_component_write(component, DA7218_PC_COUNT, DA7218_PC_FREERUN_MASK);
    snd_soc_component_write(component, DA7218_DROUTING_OUTFILT_1L, 0);
    snd_soc_component_write(component, DA7218_DROUTING_OUTFILT_1R, 0);
    snd_soc_component_update_bits(component, DA7218_CP_CTRL, DA7218_CP_SMALL_SWITCH_FREQ_EN_MASK, 0);
    snd_soc_component_update_bits(component, DA7218_MIXIN_1_CTRL, DA7218_MIXIN_1_AMP_RAMP_EN_MASK, DA7218_MIXIN_1_AMP_RAMP_EN_MASK);
    snd_soc_component_update_bits(component, DA7218_MIXIN_2_CTRL, DA7218_MIXIN_2_AMP_RAMP_EN_MASK, DA7218_MIXIN_2_AMP_RAMP_EN_MASK);
    snd_soc_component_update_bits(component, DA7218_IN_1L_FILTER_CTRL, DA7218_IN_1L_RAMP_EN_MASK, DA7218_IN_1L_RAMP_EN_MASK);
    snd_soc_component_update_bits(component, DA7218_IN_1R_FILTER_CTRL, DA7218_IN_1R_RAMP_EN_MASK, DA7218_IN_1R_RAMP_EN_MASK);
    snd_soc_component_update_bits(component, DA7218_IN_2L_FILTER_CTRL, DA7218_IN_2L_RAMP_EN_MASK, DA7218_IN_2L_RAMP_EN_MASK);
    snd_soc_component_update_bits(component, DA7218_IN_2R_FILTER_CTRL, DA7218_IN_2R_RAMP_EN_MASK, DA7218_IN_2R_RAMP_EN_MASK);
    snd_soc_component_update_bits(component, DA7218_DGS_GAIN_CTRL, DA7218_DGS_RAMP_EN_MASK, DA7218_DGS_RAMP_EN_MASK);
    snd_soc_component_update_bits(component, DA7218_OUT_1L_FILTER_CTRL, DA7218_OUT_1L_RAMP_EN_MASK, DA7218_OUT_1L_RAMP_EN_MASK);
    snd_soc_component_update_bits(component, DA7218_OUT_1R_FILTER_CTRL, DA7218_OUT_1R_RAMP_EN_MASK, DA7218_OUT_1R_RAMP_EN_MASK);
    snd_soc_component_update_bits(component, DA7218_HP_L_CTRL, DA7218_HP_L_AMP_RAMP_EN_MASK, DA7218_HP_L_AMP_RAMP_EN_MASK);
    snd_soc_component_update_bits(component, DA7218_HP_R_CTRL, DA7218_HP_R_AMP_RAMP_EN_MASK, DA7218_HP_R_AMP_RAMP_EN_MASK);
    snd_soc_component_write(component, DA7218_TONE_GEN_CYCLES, DA7218_BEEP_CYCLES_MASK);
    if (*da7218).dev_id == DA7217_DEV_ID as usize {
        snd_soc_component_update_bits(component, DA7218_HP_DIFF_CTRL, DA7218_HP_AMP_DIFF_MODE_EN_MASK, DA7218_HP_AMP_DIFF_MODE_EN_MASK);
        snd_soc_component_write(component, DA7218_EVENT_MASK, DA7218_HPLDET_JACK_EVENT_IRQ_MSK_MASK);
    }
    if (*da7218).irq != 0 {
        ret = devm_request_threaded_irq((*component).dev, (*da7218).irq, ptr::null(), da7218_irq_thread, IRQF_TRIGGER_LOW | IRQF_ONESHOT, cstr!("da7218"), component as *mut c_void);
        if ret != 0 {
            dev_err((*component).dev, cstr!("Failed to request IRQ %d: %d\n"), (*da7218).irq, ret);
            regulator_bulk_disable(DA7218_NUM_SUPPLIES as c_int, (*da7218).supplies.as_mut_ptr());
            return ret;
        }
    }
    0
}

unsafe extern "C" fn da7218_remove(component: *mut snd_soc_component) {
    let da7218 = snd_soc_component_get_drvdata(component) as *mut da7218_priv;
    regulator_bulk_disable(DA7218_NUM_SUPPLIES as c_int, (*da7218).supplies.as_mut_ptr());
}

/* CONFIG_PM conditional from C: suspend/resume are present when PM is enabled,
 * otherwise the component-driver fields are NULL.
 */
unsafe extern "C" fn da7218_suspend(component: *mut snd_soc_component) -> c_int {
    let da7218 = snd_soc_component_get_drvdata(component) as *mut da7218_priv;
    da7218_set_bias_level(component, SND_SOC_BIAS_OFF);
    if (*da7218).jack.is_null() { snd_soc_component_write(component, DA7218_SYSTEM_ACTIVE, 0); }
    0
}

unsafe extern "C" fn da7218_resume(component: *mut snd_soc_component) -> c_int {
    let da7218 = snd_soc_component_get_drvdata(component) as *mut da7218_priv;
    if (*da7218).jack.is_null() { snd_soc_component_write(component, DA7218_SYSTEM_ACTIVE, DA7218_SYSTEM_ACTIVE_MASK); }
    da7218_set_bias_level(component, SND_SOC_BIAS_STANDBY);
    0
}

driver_structs! {
    static const soc_component_dev_da7218: snd_soc_component_driver = {
        .probe = da7218_probe,
        .remove = da7218_remove,
        .suspend = da7218_suspend,
        .resume = da7218_resume,
        .set_bias_level = da7218_set_bias_level,
        .controls = da7218_snd_controls,
        .num_controls = ARRAY_SIZE(da7218_snd_controls),
        .dapm_widgets = da7218_dapm_widgets,
        .num_dapm_widgets = ARRAY_SIZE(da7218_dapm_widgets),
        .dapm_routes = da7218_audio_map,
        .num_dapm_routes = ARRAY_SIZE(da7218_audio_map),
        .idle_bias_on = 1,
        .use_pmdown_time = 1,
        .endianness = 1,
    };
}

static da7218_reg_defaults: [reg_default; 151] = [
    reg_default{reg:DA7218_SYSTEM_ACTIVE,def:0x00}, reg_default{reg:DA7218_CIF_CTRL,def:0x00}, reg_default{reg:DA7218_SPARE1,def:0x00}, reg_default{reg:DA7218_SR,def:0xAA}, reg_default{reg:DA7218_PC_COUNT,def:0x02}, reg_default{reg:DA7218_GAIN_RAMP_CTRL,def:0x00}, reg_default{reg:DA7218_CIF_TIMEOUT_CTRL,def:0x01}, reg_default{reg:DA7218_SYSTEM_MODES_INPUT,def:0x00}, reg_default{reg:DA7218_SYSTEM_MODES_OUTPUT,def:0x00},
    reg_default{reg:DA7218_IN_1L_FILTER_CTRL,def:0x00}, reg_default{reg:DA7218_IN_1R_FILTER_CTRL,def:0x00}, reg_default{reg:DA7218_IN_2L_FILTER_CTRL,def:0x00}, reg_default{reg:DA7218_IN_2R_FILTER_CTRL,def:0x00}, reg_default{reg:DA7218_OUT_1L_FILTER_CTRL,def:0x40}, reg_default{reg:DA7218_OUT_1R_FILTER_CTRL,def:0x40}, reg_default{reg:DA7218_OUT_1_HPF_FILTER_CTRL,def:0x80}, reg_default{reg:DA7218_OUT_1_EQ_12_FILTER_CTRL,def:0x77}, reg_default{reg:DA7218_OUT_1_EQ_34_FILTER_CTRL,def:0x77}, reg_default{reg:DA7218_OUT_1_EQ_5_FILTER_CTRL,def:0x07}, reg_default{reg:DA7218_OUT_1_BIQ_5STAGE_CTRL,def:0x40}, reg_default{reg:DA7218_OUT_1_BIQ_5STAGE_DATA,def:0x00}, reg_default{reg:DA7218_OUT_1_BIQ_5STAGE_ADDR,def:0x00},
    reg_default{reg:DA7218_MIXIN_1_CTRL,def:0x48}, reg_default{reg:DA7218_MIXIN_1_GAIN,def:0x03}, reg_default{reg:DA7218_MIXIN_2_CTRL,def:0x48}, reg_default{reg:DA7218_MIXIN_2_GAIN,def:0x03}, reg_default{reg:DA7218_ALC_CTRL1,def:0x00}, reg_default{reg:DA7218_ALC_CTRL2,def:0x00}, reg_default{reg:DA7218_ALC_CTRL3,def:0x00}, reg_default{reg:DA7218_ALC_NOISE,def:0x3F}, reg_default{reg:DA7218_ALC_TARGET_MIN,def:0x3F}, reg_default{reg:DA7218_ALC_TARGET_MAX,def:0x00}, reg_default{reg:DA7218_ALC_GAIN_LIMITS,def:0xFF}, reg_default{reg:DA7218_ALC_ANA_GAIN_LIMITS,def:0x71}, reg_default{reg:DA7218_ALC_ANTICLIP_CTRL,def:0x00}, reg_default{reg:DA7218_AGS_ENABLE,def:0x00}, reg_default{reg:DA7218_AGS_TRIGGER,def:0x09}, reg_default{reg:DA7218_AGS_ATT_MAX,def:0x00}, reg_default{reg:DA7218_AGS_TIMEOUT,def:0x00}, reg_default{reg:DA7218_AGS_ANTICLIP_CTRL,def:0x00}, reg_default{reg:DA7218_ENV_TRACK_CTRL,def:0x00}, reg_default{reg:DA7218_LVL_DET_CTRL,def:0x00}, reg_default{reg:DA7218_LVL_DET_LEVEL,def:0x7F},
    reg_default{reg:DA7218_DGS_TRIGGER,def:0x24}, reg_default{reg:DA7218_DGS_ENABLE,def:0x00}, reg_default{reg:DA7218_DGS_RISE_FALL,def:0x50}, reg_default{reg:DA7218_DGS_SYNC_DELAY,def:0xA3}, reg_default{reg:DA7218_DGS_SYNC_DELAY2,def:0x31}, reg_default{reg:DA7218_DGS_SYNC_DELAY3,def:0x11}, reg_default{reg:DA7218_DGS_LEVELS,def:0x01}, reg_default{reg:DA7218_DGS_GAIN_CTRL,def:0x74},
    reg_default{reg:DA7218_DROUTING_OUTDAI_1L,def:0x01}, reg_default{reg:DA7218_DMIX_OUTDAI_1L_INFILT_1L_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTDAI_1L_INFILT_1R_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTDAI_1L_INFILT_2L_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTDAI_1L_INFILT_2R_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTDAI_1L_TONEGEN_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTDAI_1L_INDAI_1L_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTDAI_1L_INDAI_1R_GAIN,def:0x1C},
    reg_default{reg:DA7218_DROUTING_OUTDAI_1R,def:0x04}, reg_default{reg:DA7218_DMIX_OUTDAI_1R_INFILT_1L_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTDAI_1R_INFILT_1R_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTDAI_1R_INFILT_2L_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTDAI_1R_INFILT_2R_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTDAI_1R_TONEGEN_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTDAI_1R_INDAI_1L_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTDAI_1R_INDAI_1R_GAIN,def:0x1C},
    reg_default{reg:DA7218_DROUTING_OUTFILT_1L,def:0x01}, reg_default{reg:DA7218_DMIX_OUTFILT_1L_INFILT_1L_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTFILT_1L_INFILT_1R_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTFILT_1L_INFILT_2L_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTFILT_1L_INFILT_2R_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTFILT_1L_TONEGEN_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTFILT_1L_INDAI_1L_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTFILT_1L_INDAI_1R_GAIN,def:0x1C},
    reg_default{reg:DA7218_DROUTING_OUTFILT_1R,def:0x04}, reg_default{reg:DA7218_DMIX_OUTFILT_1R_INFILT_1L_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTFILT_1R_INFILT_1R_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTFILT_1R_INFILT_2L_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTFILT_1R_INFILT_2R_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTFILT_1R_TONEGEN_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTFILT_1R_INDAI_1L_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTFILT_1R_INDAI_1R_GAIN,def:0x1C},
    reg_default{reg:DA7218_DROUTING_OUTDAI_2L,def:0x04}, reg_default{reg:DA7218_DMIX_OUTDAI_2L_INFILT_1L_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTDAI_2L_INFILT_1R_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTDAI_2L_INFILT_2L_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTDAI_2L_INFILT_2R_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTDAI_2L_TONEGEN_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTDAI_2L_INDAI_1L_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTDAI_2L_INDAI_1R_GAIN,def:0x1C},
    reg_default{reg:DA7218_DROUTING_OUTDAI_2R,def:0x08}, reg_default{reg:DA7218_DMIX_OUTDAI_2R_INFILT_1L_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTDAI_2R_INFILT_1R_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTDAI_2R_INFILT_2L_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTDAI_2R_INFILT_2R_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTDAI_2R_TONEGEN_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTDAI_2R_INDAI_1L_GAIN,def:0x1C}, reg_default{reg:DA7218_DMIX_OUTDAI_2R_INDAI_1R_GAIN,def:0x1C},
    reg_default{reg:DA7218_DAI_CTRL,def:0x28}, reg_default{reg:DA7218_DAI_TDM_CTRL,def:0x40}, reg_default{reg:DA7218_DAI_OFFSET_LOWER,def:0x00}, reg_default{reg:DA7218_DAI_OFFSET_UPPER,def:0x00}, reg_default{reg:DA7218_DAI_CLK_MODE,def:0x01}, reg_default{reg:DA7218_PLL_CTRL,def:0x04}, reg_default{reg:DA7218_PLL_FRAC_TOP,def:0x00}, reg_default{reg:DA7218_PLL_FRAC_BOT,def:0x00}, reg_default{reg:DA7218_PLL_INTEGER,def:0x20}, reg_default{reg:DA7218_DAC_NG_CTRL,def:0x00}, reg_default{reg:DA7218_DAC_NG_SETUP_TIME,def:0x00}, reg_default{reg:DA7218_DAC_NG_OFF_THRESH,def:0x00}, reg_default{reg:DA7218_DAC_NG_ON_THRESH,def:0x00}, reg_default{reg:DA7218_TONE_GEN_CFG2,def:0x00}, reg_default{reg:DA7218_TONE_GEN_FREQ1_L,def:0x55}, reg_default{reg:DA7218_TONE_GEN_FREQ1_U,def:0x15}, reg_default{reg:DA7218_TONE_GEN_FREQ2_L,def:0x00}, reg_default{reg:DA7218_TONE_GEN_FREQ2_U,def:0x40}, reg_default{reg:DA7218_TONE_GEN_CYCLES,def:0x00}, reg_default{reg:DA7218_TONE_GEN_ON_PER,def:0x02}, reg_default{reg:DA7218_TONE_GEN_OFF_PER,def:0x01}, reg_default{reg:DA7218_CP_CTRL,def:0x60}, reg_default{reg:DA7218_CP_DELAY,def:0x11}, reg_default{reg:DA7218_CP_VOL_THRESHOLD1,def:0x0E},
    reg_default{reg:DA7218_MIC_1_CTRL,def:0x40}, reg_default{reg:DA7218_MIC_1_GAIN,def:0x01}, reg_default{reg:DA7218_MIC_1_SELECT,def:0x00}, reg_default{reg:DA7218_MIC_2_CTRL,def:0x40}, reg_default{reg:DA7218_MIC_2_GAIN,def:0x01}, reg_default{reg:DA7218_MIC_2_SELECT,def:0x00}, reg_default{reg:DA7218_IN_1_HPF_FILTER_CTRL,def:0x80}, reg_default{reg:DA7218_IN_2_HPF_FILTER_CTRL,def:0x80}, reg_default{reg:DA7218_ADC_1_CTRL,def:0x07}, reg_default{reg:DA7218_ADC_2_CTRL,def:0x07}, reg_default{reg:DA7218_MIXOUT_L_CTRL,def:0x00}, reg_default{reg:DA7218_MIXOUT_L_GAIN,def:0x03}, reg_default{reg:DA7218_MIXOUT_R_CTRL,def:0x00}, reg_default{reg:DA7218_MIXOUT_R_GAIN,def:0x03}, reg_default{reg:DA7218_HP_L_CTRL,def:0x40}, reg_default{reg:DA7218_HP_L_GAIN,def:0x3B}, reg_default{reg:DA7218_HP_R_CTRL,def:0x40}, reg_default{reg:DA7218_HP_R_GAIN,def:0x3B}, reg_default{reg:DA7218_HP_DIFF_CTRL,def:0x00}, reg_default{reg:DA7218_HP_DIFF_UNLOCK,def:0xC3}, reg_default{reg:DA7218_HPLDET_JACK,def:0x0B}, reg_default{reg:DA7218_HPLDET_CTRL,def:0x00}, reg_default{reg:DA7218_REFERENCES,def:0x08}, reg_default{reg:DA7218_IO_CTRL,def:0x00}, reg_default{reg:DA7218_LDO_CTRL,def:0x00}, reg_default{reg:DA7218_SIDETONE_CTRL,def:0x40}, reg_default{reg:DA7218_SIDETONE_IN_SELECT,def:0x00}, reg_default{reg:DA7218_SIDETONE_GAIN,def:0x1C}, reg_default{reg:DA7218_DROUTING_ST_OUTFILT_1L,def:0x01}, reg_default{reg:DA7218_DROUTING_ST_OUTFILT_1R,def:0x02}, reg_default{reg:DA7218_SIDETONE_BIQ_3STAGE_DATA,def:0x00}, reg_default{reg:DA7218_SIDETONE_BIQ_3STAGE_ADDR,def:0x00}, reg_default{reg:DA7218_EVENT_MASK,def:0x00}, reg_default{reg:DA7218_DMIC_1_CTRL,def:0x00}, reg_default{reg:DA7218_DMIC_2_CTRL,def:0x00}, reg_default{reg:DA7218_IN_1L_GAIN,def:0x6F}, reg_default{reg:DA7218_IN_1R_GAIN,def:0x6F}, reg_default{reg:DA7218_IN_2L_GAIN,def:0x6F}, reg_default{reg:DA7218_IN_2R_GAIN,def:0x6F}, reg_default{reg:DA7218_OUT_1L_GAIN,def:0x6F}, reg_default{reg:DA7218_OUT_1R_GAIN,def:0x6F}, reg_default{reg:DA7218_MICBIAS_CTRL,def:0x00}, reg_default{reg:DA7218_MICBIAS_EN,def:0x00},
];

unsafe extern "C" fn da7218_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        DA7218_STATUS1 | DA7218_SOFT_RESET | DA7218_SYSTEM_STATUS | DA7218_CALIB_CTRL |
        DA7218_CALIB_OFFSET_AUTO_M_1 | DA7218_CALIB_OFFSET_AUTO_U_1 |
        DA7218_CALIB_OFFSET_AUTO_M_2 | DA7218_CALIB_OFFSET_AUTO_U_2 |
        DA7218_PLL_STATUS | DA7218_PLL_REFOSC_CAL | DA7218_TONE_GEN_CFG1 |
        DA7218_ADC_MODE | DA7218_HP_SNGL_CTRL | DA7218_HPLDET_TEST |
        DA7218_EVENT_STATUS | DA7218_EVENT => true,
        _ => false,
    }
}

driver_structs! {
    static const da7218_regmap_config: regmap_config = {
        .reg_bits = 8,
        .val_bits = 8,
        .max_register = DA7218_MICBIAS_EN,
        .reg_defaults = da7218_reg_defaults,
        .num_reg_defaults = ARRAY_SIZE(da7218_reg_defaults),
        .volatile_reg = da7218_volatile_register,
        .cache_type = REGCACHE_RBTREE,
    };
}

unsafe extern "C" fn da7218_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let da7218 = devm_kzalloc(&mut (*i2c).dev, size_of::<da7218_priv>(), GFP_KERNEL) as *mut da7218_priv;
    if da7218.is_null() { return -ENOMEM; }
    i2c_set_clientdata(i2c, da7218 as *mut c_void);
    (*da7218).dev_id = i2c_get_match_data(i2c) as usize;
    if (*da7218).dev_id != DA7217_DEV_ID as usize && (*da7218).dev_id != DA7218_DEV_ID as usize {
        dev_err(&mut (*i2c).dev, cstr!("Invalid device Id\n"));
        return -EINVAL;
    }
    (*da7218).irq = (*i2c).irq;
    (*da7218).regmap = devm_regmap_init_i2c(i2c, &da7218_regmap_config);
    if IS_ERR((*da7218).regmap as *const c_void) {
        let ret = PTR_ERR((*da7218).regmap as *const c_void);
        dev_err(&mut (*i2c).dev, cstr!("regmap_init() failed: %d\n"), ret);
        return ret;
    }
    let ret = devm_snd_soc_register_component(&mut (*i2c).dev, &soc_component_dev_da7218, &mut da7218_dai, 1);
    if ret < 0 { dev_err(&mut (*i2c).dev, cstr!("Failed to register da7218 component: %d\n"), ret); }
    ret
}

device_tables! {
    static const da7218_i2c_id: [i2c_device_id] = [
        { .name = "da7217", .driver_data = DA7217_DEV_ID },
        { .name = "da7218", .driver_data = DA7218_DEV_ID },
        { }
    ];
    static mut da7218_i2c_driver: i2c_driver = {
        .driver = { .name = "da7218", .of_match_table = da7218_of_match },
        .probe = da7218_i2c_probe,
        .id_table = da7218_i2c_id,
    };
    module_i2c_driver!(da7218_i2c_driver);
    MODULE_DEVICE_TABLE!(of, da7218_of_match);
    MODULE_DEVICE_TABLE!(i2c, da7218_i2c_id);
    MODULE_DESCRIPTION!("ASoC DA7218 Codec driver");
    MODULE_AUTHOR!("Adam Thomson <Adam.Thomson.Opensource@diasemi.com>");
    MODULE_LICENSE!("GPL");
}


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
