// SPDX-License-Identifier: GPL-2.0-only
//
// ALSA SoC Audio driver for CS47L90 codec
//
// Copyright (C) 2015-2019 Cirrus Logic, Inc. and
//                         Cirrus Logic International Semiconductor Ltd.
//
// Source-level Rust translation of soc/codecs/cs47l90.c.
// Linux/ASoC/Madera macro expansions and foreign declarations are supplied by
// the surrounding translated repository.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

pub const DRV_NAME: &[u8] = b"cs47l90-codec\0";
pub const CS47L90_NUM_ADSP: usize = 7;
pub const CS47L90_MONO_OUTPUTS: c_int = 3;
pub const CS47L90_DIG_VU: c_uint = 0x0200;

#[repr(C)]
pub struct cs47l90 {
    pub core: madera_priv,
    pub fll: [madera_fll; 3],
}

#[repr(C)] pub struct madera_priv { pub madera: *mut madera, pub dev: *mut device, pub num_inputs: c_int, pub adsp: [wm_adsp; CS47L90_NUM_ADSP] }
#[repr(C)] pub struct madera_fll { _private: [u8; 0] }
#[repr(C)] pub struct wm_adsp { pub part: *const c_char, pub cs_dsp: cs_dsp }
#[repr(C)] pub struct cs_dsp { pub num: c_int, pub type_: c_int, pub rev: c_int, pub dev: *mut device, pub regmap: *mut regmap, pub base: c_uint, pub mem: *const cs_dsp_region, pub num_mems: usize, pub lock_regions: c_uint }
#[repr(C)] pub struct cs_dsp_region { pub type_: c_uint, pub base: c_uint }
#[repr(C)] pub struct madera { pub regmap: *mut regmap, pub regmap_32bit: *mut regmap, pub dev: *mut device, pub irq_dev: *mut c_void, pub notifier: blocking_notifier_head, pub dapm_ptr_lock: mutex, pub dapm: *mut snd_soc_dapm_context }
#[repr(C)] pub struct device { pub parent: *mut device }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct blocking_notifier_head { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { pub dapm: *mut snd_soc_dapm_context, pub shift: c_int }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_component { _private: [u8; 0] }
#[repr(C)] pub struct snd_compr_stream { pub private_data: *mut c_void }
#[repr(C)] pub struct snd_soc_pcm_runtime { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai { pub name: *const c_char }
#[repr(C)] pub struct madera_voice_trigger_info { pub core_num: c_int }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] pub struct soc_enum { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_route { pub sink: *const c_char, pub control: *const c_char, pub source: *const c_char }
#[repr(C)] pub struct snd_soc_dapm_widget_decl { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai_ops { pub compress_new: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct snd_compress_ops { pub open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream) -> c_int>, pub free: Option<unsafe extern "C" fn()>, pub set_params: Option<unsafe extern "C" fn()>, pub get_caps: Option<unsafe extern "C" fn()>, pub trigger: Option<unsafe extern "C" fn()>, pub pointer: Option<unsafe extern "C" fn()>, pub copy: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct snd_soc_pcm_stream { pub stream_name: *const c_char, pub channels_min: c_uint, pub channels_max: c_uint, pub rates: c_uint, pub formats: u64 }
#[repr(C)] pub struct snd_soc_dai_driver { pub name: *const c_char, pub id: c_int, pub base: c_uint, pub playback: snd_soc_pcm_stream, pub capture: snd_soc_pcm_stream, pub ops: *const snd_soc_dai_ops, pub symmetric_rate: c_uint, pub symmetric_sample_bits: c_uint }
#[repr(C)] pub struct snd_soc_component_driver { _private: [u8; 0] }
#[repr(C)] pub struct platform_driver { _private: [u8; 0] }

unsafe extern "C" {
    static mut madera_dai_ops: snd_soc_dai_ops;
    static mut madera_simple_dai_ops: snd_soc_dai_ops;
    static mut madera_adsp_rate_controls: snd_kcontrol_new;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_component_init_regmap(component: *mut snd_soc_component, regmap: *mut regmap);
    fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_add_component_controls(component: *mut snd_soc_component, controls: *const snd_kcontrol_new, num: c_uint) -> c_int;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_new_compress();
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn devm_snd_soc_register_component(dev: *mut device, cmp: *const snd_soc_component_driver, dai: *mut snd_soc_dai_driver, num: c_int) -> c_int;
    fn madera_set_adsp_clk(priv_: *mut madera_priv, dsp: c_int, freq: c_uint) -> c_int;
    fn wm_adsp_early_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int;
    fn madera_set_fll_refclk(fll: *mut madera_fll, source: c_int, fref: c_uint, fout: c_uint) -> c_int;
    fn madera_set_fll_ao_refclk(fll: *mut madera_fll, source: c_int, fref: c_uint, fout: c_uint) -> c_int;
    fn madera_set_fll_syncclk(fll: *mut madera_fll, source: c_int, fref: c_uint, fout: c_uint) -> c_int;
    fn wm_adsp_compr_open(adsp: *mut wm_adsp, stream: *mut snd_compr_stream) -> c_int;
    fn wm_adsp_compr_handle_irq(adsp: *mut wm_adsp) -> c_int;
    fn blocking_notifier_call_chain(nh: *mut blocking_notifier_head, val: c_ulong, v: *mut c_void) -> c_int;
    fn madera_init_inputs(component: *mut snd_soc_component) -> c_int;
    fn madera_init_outputs(component: *mut snd_soc_component, out: *mut c_void, mono: c_int, max: c_int) -> c_int;
    fn wm_adsp2_component_probe(adsp: *mut wm_adsp, component: *mut snd_soc_component);
    fn wm_adsp2_component_remove(adsp: *mut wm_adsp, component: *mut snd_soc_component);
    fn madera_core_init(priv_: *mut madera_priv) -> c_int;
    fn madera_core_free(priv_: *mut madera_priv);
    fn madera_request_irq(madera: *mut madera, irq: c_int, name: *const c_char, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, data: *mut c_void) -> c_int;
    fn madera_free_irq(madera: *mut madera, irq: c_int, data: *mut c_void);
    fn madera_set_irq_wake(madera: *mut madera, irq: c_int, on: c_int) -> c_int;
    fn wm_adsp2_init(adsp: *mut wm_adsp) -> c_int;
    fn wm_adsp2_remove(adsp: *mut wm_adsp);
    fn wm_adsp2_bus_error();
    fn madera_init_bus_error_irq(priv_: *mut madera_priv, i: c_int, handler: unsafe extern "C" fn()) -> c_int;
    fn madera_free_bus_error_irq(priv_: *mut madera_priv, i: c_int);
    fn madera_init_fll(madera: *mut madera, id: c_int, base: c_uint, fll: *mut madera_fll);
    fn madera_init_dai(priv_: *mut madera_priv, i: c_int);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_idle(dev: *mut device) -> c_int;
    fn pm_runtime_disable(dev: *mut device);
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn wm_adsp_compr_free(); fn wm_adsp_compr_set_params(); fn wm_adsp_compr_get_caps(); fn wm_adsp_compr_trigger(); fn wm_adsp_compr_pointer(); fn wm_adsp_compr_copy();
    fn madera_set_sysclk();
}

type c_ulong = core::ffi::c_ulong;
type irqreturn_t = c_uint;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const EPROBE_DEFER: c_int = 517;
const GFP_KERNEL: c_uint = 0;

// Register, bit, event, format, and macro-expanded table symbols are external
// dependencies of the translated driver, corresponding to Linux, ASoC, Madera,
// and wm_adsp headers included by the C source.
const SND_SOC_DAPM_PRE_PMU: c_int = 0;
const MADERA_NOTIFY_VOICE_TRIGGER: c_ulong = 0;
const WM_ADSP_COMPR_VOICE_TRIGGER: c_int = 0;
const WMFW_ADSP2: c_int = 0;
const CS_ADSP2_REGION_1_9: c_uint = 0;
const MADERA_RATES: c_uint = 0;
const MADERA_FORMATS: u64 = 0;
const MADERA_FLL1_REFCLK: c_int = 0;
const MADERA_FLL2_REFCLK: c_int = 0;
const MADERA_FLLAO_REFCLK: c_int = 0;
const MADERA_FLL1_SYNCCLK: c_int = 0;
const MADERA_FLL2_SYNCCLK: c_int = 0;
const MADERA_IRQ_DSP_IRQ1: c_int = 0;
const MADERA_DSP_CLOCK_2: c_uint = 0;
const MADERA_FLL1_CONTROL_1: c_uint = 1;
const MADERA_FLL2_CONTROL_1: c_uint = 1;
const MADERA_FLLAO_CONTROL_1: c_uint = 1;
const MADERA_MAX_DAI: usize = 16;
const MADERA_DAC_DIGITAL_VOLUME_1L: c_uint = 0;
const MADERA_DAC_DIGITAL_VOLUME_1R: c_uint = 0;
const MADERA_DAC_DIGITAL_VOLUME_2L: c_uint = 0;
const MADERA_DAC_DIGITAL_VOLUME_2R: c_uint = 0;
const MADERA_DAC_DIGITAL_VOLUME_3L: c_uint = 0;
const MADERA_DAC_DIGITAL_VOLUME_3R: c_uint = 0;
const MADERA_DAC_DIGITAL_VOLUME_5L: c_uint = 0;
const MADERA_DAC_DIGITAL_VOLUME_5R: c_uint = 0;
const MADERA_DSP1_CONFIG_1: c_uint = 0; const MADERA_DSP2_CONFIG_1: c_uint = 0; const MADERA_DSP3_CONFIG_1: c_uint = 0; const MADERA_DSP4_CONFIG_1: c_uint = 0; const MADERA_DSP5_CONFIG_1: c_uint = 0; const MADERA_DSP6_CONFIG_1: c_uint = 0; const MADERA_DSP7_CONFIG_1: c_uint = 0;
const MADERA_AIF1_BCLK_CTRL: c_uint = 0; const MADERA_AIF2_BCLK_CTRL: c_uint = 0; const MADERA_AIF3_BCLK_CTRL: c_uint = 0; const MADERA_AIF4_BCLK_CTRL: c_uint = 0;
const WMFW_ADSP2_PM: c_uint = 0; const WMFW_ADSP2_ZM: c_uint = 0; const WMFW_ADSP2_XM: c_uint = 0; const WMFW_ADSP2_YM: c_uint = 0;

static cs47l90_dsp1_regions: [cs_dsp_region; 4] = [cs_dsp_region{type_:WMFW_ADSP2_PM,base:0x080000},cs_dsp_region{type_:WMFW_ADSP2_ZM,base:0x0e0000},cs_dsp_region{type_:WMFW_ADSP2_XM,base:0x0a0000},cs_dsp_region{type_:WMFW_ADSP2_YM,base:0x0c0000}];
static cs47l90_dsp2_regions: [cs_dsp_region; 4] = [cs_dsp_region{type_:WMFW_ADSP2_PM,base:0x100000},cs_dsp_region{type_:WMFW_ADSP2_ZM,base:0x160000},cs_dsp_region{type_:WMFW_ADSP2_XM,base:0x120000},cs_dsp_region{type_:WMFW_ADSP2_YM,base:0x140000}];
static cs47l90_dsp3_regions: [cs_dsp_region; 4] = [cs_dsp_region{type_:WMFW_ADSP2_PM,base:0x180000},cs_dsp_region{type_:WMFW_ADSP2_ZM,base:0x1e0000},cs_dsp_region{type_:WMFW_ADSP2_XM,base:0x1a0000},cs_dsp_region{type_:WMFW_ADSP2_YM,base:0x1c0000}];
static cs47l90_dsp4_regions: [cs_dsp_region; 4] = [cs_dsp_region{type_:WMFW_ADSP2_PM,base:0x200000},cs_dsp_region{type_:WMFW_ADSP2_ZM,base:0x260000},cs_dsp_region{type_:WMFW_ADSP2_XM,base:0x220000},cs_dsp_region{type_:WMFW_ADSP2_YM,base:0x240000}];
static cs47l90_dsp5_regions: [cs_dsp_region; 4] = [cs_dsp_region{type_:WMFW_ADSP2_PM,base:0x280000},cs_dsp_region{type_:WMFW_ADSP2_ZM,base:0x2e0000},cs_dsp_region{type_:WMFW_ADSP2_XM,base:0x2a0000},cs_dsp_region{type_:WMFW_ADSP2_YM,base:0x2c0000}];
static cs47l90_dsp6_regions: [cs_dsp_region; 4] = [cs_dsp_region{type_:WMFW_ADSP2_PM,base:0x300000},cs_dsp_region{type_:WMFW_ADSP2_ZM,base:0x360000},cs_dsp_region{type_:WMFW_ADSP2_XM,base:0x320000},cs_dsp_region{type_:WMFW_ADSP2_YM,base:0x340000}];
static cs47l90_dsp7_regions: [cs_dsp_region; 4] = [cs_dsp_region{type_:WMFW_ADSP2_PM,base:0x380000},cs_dsp_region{type_:WMFW_ADSP2_ZM,base:0x3e0000},cs_dsp_region{type_:WMFW_ADSP2_XM,base:0x3a0000},cs_dsp_region{type_:WMFW_ADSP2_YM,base:0x3c0000}];
static cs47l90_dsp_control_bases: [c_uint; CS47L90_NUM_ADSP] = [MADERA_DSP1_CONFIG_1,MADERA_DSP2_CONFIG_1,MADERA_DSP3_CONFIG_1,MADERA_DSP4_CONFIG_1,MADERA_DSP5_CONFIG_1,MADERA_DSP6_CONFIG_1,MADERA_DSP7_CONFIG_1];

unsafe extern "C" fn cs47l90_adsp_power_ev(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let cs47l90 = snd_soc_component_get_drvdata(component) as *mut cs47l90;
    let priv_ = &mut (*cs47l90).core as *mut madera_priv;
    let madera = (*priv_).madera;
    let mut freq: c_uint = 0;
    let mut ret = regmap_read((*madera).regmap, MADERA_DSP_CLOCK_2, &mut freq);
    if ret != 0 { dev_err((*madera).dev, b"Failed to read MADERA_DSP_CLOCK_2: %d\n\0".as_ptr() as *const c_char, ret); return ret; }
    match event { SND_SOC_DAPM_PRE_PMU => { ret = madera_set_adsp_clk(&mut (*cs47l90).core, (*w).shift, freq); if ret != 0 { return ret; } }, _ => {} }
    wm_adsp_early_event(w, kcontrol, event)
}

static cs47l90_aec_loopback_texts: [*const c_char; 8] = [b"HPOUT1L\0".as_ptr() as *const c_char,b"HPOUT1R\0".as_ptr() as *const c_char,b"HPOUT2L\0".as_ptr() as *const c_char,b"HPOUT2R\0".as_ptr() as *const c_char,b"HPOUT3L\0".as_ptr() as *const c_char,b"HPOUT3R\0".as_ptr() as *const c_char,b"SPKDAT1L\0".as_ptr() as *const c_char,b"SPKDAT1R\0".as_ptr() as *const c_char];
static cs47l90_aec_loopback_values: [c_uint; 8] = [0, 1, 2, 3, 4, 5, 8, 9];
static mut cs47l90_snd_controls: [snd_kcontrol_new; 0] = [];
static mut cs47l90_aec_loopback_mux: [snd_kcontrol_new; 0] = [];
static mut cs47l90_anc_input_mux: [snd_kcontrol_new; 0] = [];
static mut cs47l90_output_anc_src: [snd_kcontrol_new; 0] = [];
static mut cs47l90_dapm_widgets: [snd_soc_dapm_widget_decl; 0] = [];
static mut cs47l90_dapm_routes: [snd_soc_dapm_route; 0] = [];

unsafe extern "C" fn cs47l90_set_fll(component: *mut snd_soc_component, fll_id: c_int, source: c_int, fref: c_uint, fout: c_uint) -> c_int {
    let cs47l90 = snd_soc_component_get_drvdata(component) as *mut cs47l90;
    match fll_id {
        MADERA_FLL1_REFCLK => madera_set_fll_refclk(&mut (*cs47l90).fll[0], source, fref, fout),
        MADERA_FLL2_REFCLK => madera_set_fll_refclk(&mut (*cs47l90).fll[1], source, fref, fout),
        MADERA_FLLAO_REFCLK => madera_set_fll_ao_refclk(&mut (*cs47l90).fll[2], source, fref, fout),
        MADERA_FLL1_SYNCCLK => madera_set_fll_syncclk(&mut (*cs47l90).fll[0], source, fref, fout),
        MADERA_FLL2_SYNCCLK => madera_set_fll_syncclk(&mut (*cs47l90).fll[1], source, fref, fout),
        _ => -EINVAL,
    }
}

static cs47l90_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops { compress_new: Some(snd_soc_new_compress) };
static mut cs47l90_dai: [snd_soc_dai_driver; 11] = [
    snd_soc_dai_driver{name:b"cs47l90-aif1\0".as_ptr() as *const c_char,id:1,base:MADERA_AIF1_BCLK_CTRL,playback:snd_soc_pcm_stream{stream_name:b"AIF1 Playback\0".as_ptr() as *const c_char,channels_min:1,channels_max:8,rates:MADERA_RATES,formats:MADERA_FORMATS},capture:snd_soc_pcm_stream{stream_name:b"AIF1 Capture\0".as_ptr() as *const c_char,channels_min:1,channels_max:8,rates:MADERA_RATES,formats:MADERA_FORMATS},ops:ptr::addr_of!(madera_dai_ops),symmetric_rate:1,symmetric_sample_bits:1},
    snd_soc_dai_driver{name:b"cs47l90-aif2\0".as_ptr() as *const c_char,id:2,base:MADERA_AIF2_BCLK_CTRL,playback:snd_soc_pcm_stream{stream_name:b"AIF2 Playback\0".as_ptr() as *const c_char,channels_min:1,channels_max:8,rates:MADERA_RATES,formats:MADERA_FORMATS},capture:snd_soc_pcm_stream{stream_name:b"AIF2 Capture\0".as_ptr() as *const c_char,channels_min:1,channels_max:8,rates:MADERA_RATES,formats:MADERA_FORMATS},ops:ptr::addr_of!(madera_dai_ops),symmetric_rate:1,symmetric_sample_bits:1},
    snd_soc_dai_driver{name:b"cs47l90-aif3\0".as_ptr() as *const c_char,id:3,base:MADERA_AIF3_BCLK_CTRL,playback:snd_soc_pcm_stream{stream_name:b"AIF3 Playback\0".as_ptr() as *const c_char,channels_min:1,channels_max:2,rates:MADERA_RATES,formats:MADERA_FORMATS},capture:snd_soc_pcm_stream{stream_name:b"AIF3 Capture\0".as_ptr() as *const c_char,channels_min:1,channels_max:2,rates:MADERA_RATES,formats:MADERA_FORMATS},ops:ptr::addr_of!(madera_dai_ops),symmetric_rate:1,symmetric_sample_bits:1},
    snd_soc_dai_driver{name:b"cs47l90-aif4\0".as_ptr() as *const c_char,id:4,base:MADERA_AIF4_BCLK_CTRL,playback:snd_soc_pcm_stream{stream_name:b"AIF4 Playback\0".as_ptr() as *const c_char,channels_min:1,channels_max:2,rates:MADERA_RATES,formats:MADERA_FORMATS},capture:snd_soc_pcm_stream{stream_name:b"AIF4 Capture\0".as_ptr() as *const c_char,channels_min:1,channels_max:2,rates:MADERA_RATES,formats:MADERA_FORMATS},ops:ptr::addr_of!(madera_dai_ops),symmetric_rate:1,symmetric_sample_bits:1},
    snd_soc_dai_driver{name:b"cs47l90-slim1\0".as_ptr() as *const c_char,id:5,base:0,playback:snd_soc_pcm_stream{stream_name:b"Slim1 Playback\0".as_ptr() as *const c_char,channels_min:1,channels_max:4,rates:MADERA_RATES,formats:MADERA_FORMATS},capture:snd_soc_pcm_stream{stream_name:b"Slim1 Capture\0".as_ptr() as *const c_char,channels_min:1,channels_max:4,rates:MADERA_RATES,formats:MADERA_FORMATS},ops:ptr::addr_of!(madera_simple_dai_ops),symmetric_rate:0,symmetric_sample_bits:0},
    snd_soc_dai_driver{name:b"cs47l90-slim2\0".as_ptr() as *const c_char,id:6,base:0,playback:snd_soc_pcm_stream{stream_name:b"Slim2 Playback\0".as_ptr() as *const c_char,channels_min:1,channels_max:2,rates:MADERA_RATES,formats:MADERA_FORMATS},capture:snd_soc_pcm_stream{stream_name:b"Slim2 Capture\0".as_ptr() as *const c_char,channels_min:1,channels_max:2,rates:MADERA_RATES,formats:MADERA_FORMATS},ops:ptr::addr_of!(madera_simple_dai_ops),symmetric_rate:0,symmetric_sample_bits:0},
    snd_soc_dai_driver{name:b"cs47l90-slim3\0".as_ptr() as *const c_char,id:7,base:0,playback:snd_soc_pcm_stream{stream_name:b"Slim3 Playback\0".as_ptr() as *const c_char,channels_min:1,channels_max:2,rates:MADERA_RATES,formats:MADERA_FORMATS},capture:snd_soc_pcm_stream{stream_name:b"Slim3 Capture\0".as_ptr() as *const c_char,channels_min:1,channels_max:2,rates:MADERA_RATES,formats:MADERA_FORMATS},ops:ptr::addr_of!(madera_simple_dai_ops),symmetric_rate:0,symmetric_sample_bits:0},
    snd_soc_dai_driver{name:b"cs47l90-cpu-voicectrl\0".as_ptr() as *const c_char,id:0,base:0,playback:snd_soc_pcm_stream{stream_name:ptr::null(),channels_min:0,channels_max:0,rates:0,formats:0},capture:snd_soc_pcm_stream{stream_name:b"Voice Control CPU\0".as_ptr() as *const c_char,channels_min:1,channels_max:1,rates:MADERA_RATES,formats:MADERA_FORMATS},ops:&cs47l90_dai_ops,symmetric_rate:0,symmetric_sample_bits:0},
    snd_soc_dai_driver{name:b"cs47l90-dsp-voicectrl\0".as_ptr() as *const c_char,id:0,base:0,playback:snd_soc_pcm_stream{stream_name:ptr::null(),channels_min:0,channels_max:0,rates:0,formats:0},capture:snd_soc_pcm_stream{stream_name:b"Voice Control DSP\0".as_ptr() as *const c_char,channels_min:1,channels_max:1,rates:MADERA_RATES,formats:MADERA_FORMATS},ops:ptr::null(),symmetric_rate:0,symmetric_sample_bits:0},
    snd_soc_dai_driver{name:b"cs47l90-cpu-trace\0".as_ptr() as *const c_char,id:0,base:0,playback:snd_soc_pcm_stream{stream_name:ptr::null(),channels_min:0,channels_max:0,rates:0,formats:0},capture:snd_soc_pcm_stream{stream_name:b"Audio Trace CPU\0".as_ptr() as *const c_char,channels_min:1,channels_max:6,rates:MADERA_RATES,formats:MADERA_FORMATS},ops:&cs47l90_dai_ops,symmetric_rate:0,symmetric_sample_bits:0},
    snd_soc_dai_driver{name:b"cs47l90-dsp-trace\0".as_ptr() as *const c_char,id:0,base:0,playback:snd_soc_pcm_stream{stream_name:ptr::null(),channels_min:0,channels_max:0,rates:0,formats:0},capture:snd_soc_pcm_stream{stream_name:b"Audio Trace DSP\0".as_ptr() as *const c_char,channels_min:1,channels_max:6,rates:MADERA_RATES,formats:MADERA_FORMATS},ops:ptr::null(),symmetric_rate:0,symmetric_sample_bits:0},
];

unsafe extern "C" fn cs47l90_open(component: *mut snd_soc_component, stream: *mut snd_compr_stream) -> c_int {
    let rtd = (*stream).private_data as *mut snd_soc_pcm_runtime;
    let cs47l90 = snd_soc_component_get_drvdata(component) as *mut cs47l90;
    let priv_ = &mut (*cs47l90).core as *mut madera_priv;
    let madera = (*priv_).madera;
    let name = (*snd_soc_rtd_to_codec(rtd, 0)).name;
    let n_adsp: c_int;
    if strcmp(name, b"cs47l90-dsp-voicectrl\0".as_ptr() as *const c_char) == 0 { n_adsp = 5; }
    else if strcmp(name, b"cs47l90-dsp-trace\0".as_ptr() as *const c_char) == 0 { n_adsp = 0; }
    else { dev_err((*madera).dev, b"No suitable compressed stream for DAI '%s'\n\0".as_ptr() as *const c_char, name); return -EINVAL; }
    wm_adsp_compr_open(&mut (*priv_).adsp[n_adsp as usize], stream)
}

unsafe extern "C" fn cs47l90_adsp2_irq(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let cs47l90 = data as *mut cs47l90;
    let priv_ = &mut (*cs47l90).core as *mut madera_priv;
    let madera = (*priv_).madera;
    let mut trig_info = madera_voice_trigger_info { core_num: 0 };
    let mut serviced = 0;
    for i in 0..CS47L90_NUM_ADSP {
        let ret = wm_adsp_compr_handle_irq(&mut (*priv_).adsp[i]);
        if ret != -ENODEV { serviced += 1; }
        if ret == WM_ADSP_COMPR_VOICE_TRIGGER { trig_info.core_num = i as c_int + 1; blocking_notifier_call_chain(&mut (*madera).notifier, MADERA_NOTIFY_VOICE_TRIGGER, &mut trig_info as *mut _ as *mut c_void); }
    }
    if serviced == 0 { dev_err((*madera).dev, b"Spurious compressed data IRQ\n\0".as_ptr() as *const c_char); return IRQ_NONE; }
    IRQ_HANDLED
}

unsafe extern "C" fn cs47l90_component_probe(component: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let cs47l90 = snd_soc_component_get_drvdata(component) as *mut cs47l90;
    let madera = (*cs47l90).core.madera;
    snd_soc_component_init_regmap(component, (*madera).regmap);
    (*madera).dapm = snd_soc_component_to_dapm(component);
    let mut ret = madera_init_inputs(component); if ret != 0 { return ret; }
    ret = madera_init_outputs(component, ptr::null_mut(), CS47L90_MONO_OUTPUTS, CS47L90_MONO_OUTPUTS); if ret != 0 { return ret; }
    snd_soc_dapm_disable_pin(dapm, b"HAPTICS\0".as_ptr() as *const c_char);
    ret = snd_soc_add_component_controls(component, ptr::addr_of!(madera_adsp_rate_controls), CS47L90_NUM_ADSP as c_uint); if ret != 0 { return ret; }
    for i in 0..CS47L90_NUM_ADSP { wm_adsp2_component_probe(&mut (*cs47l90).core.adsp[i], component); }
    0
}

unsafe extern "C" fn cs47l90_component_remove(component: *mut snd_soc_component) {
    let cs47l90 = snd_soc_component_get_drvdata(component) as *mut cs47l90;
    let madera = (*cs47l90).core.madera;
    (*madera).dapm = ptr::null_mut();
    for i in 0..CS47L90_NUM_ADSP { wm_adsp2_component_remove(&mut (*cs47l90).core.adsp[i], component); }
}

static mut cs47l90_digital_vu: [c_uint; 8] = [MADERA_DAC_DIGITAL_VOLUME_1L,MADERA_DAC_DIGITAL_VOLUME_1R,MADERA_DAC_DIGITAL_VOLUME_2L,MADERA_DAC_DIGITAL_VOLUME_2R,MADERA_DAC_DIGITAL_VOLUME_3L,MADERA_DAC_DIGITAL_VOLUME_3R,MADERA_DAC_DIGITAL_VOLUME_5L,MADERA_DAC_DIGITAL_VOLUME_5R];
static cs47l90_compress_ops: snd_compress_ops = snd_compress_ops { open: Some(cs47l90_open), free: Some(wm_adsp_compr_free), set_params: Some(wm_adsp_compr_set_params), get_caps: Some(wm_adsp_compr_get_caps), trigger: Some(wm_adsp_compr_trigger), pointer: Some(wm_adsp_compr_pointer), copy: Some(wm_adsp_compr_copy) };
static soc_component_dev_cs47l90: snd_soc_component_driver = snd_soc_component_driver { _private: [] };

unsafe extern "C" fn cs47l90_probe(pdev: *mut platform_device) -> c_int {
    let madera = dev_get_drvdata((*pdev).dev.parent) as *mut madera;
    let mut ret: c_int;
    if (*madera).irq_dev.is_null() { dev_dbg(&mut (*pdev).dev, b"irqchip driver not ready\n\0".as_ptr() as *const c_char); return -EPROBE_DEFER; }
    let cs47l90 = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<cs47l90>(), GFP_KERNEL) as *mut cs47l90;
    if cs47l90.is_null() { return -ENOMEM; }
    platform_set_drvdata(pdev, cs47l90 as *mut c_void);
    (*cs47l90).core.madera = madera; (*cs47l90).core.dev = &mut (*pdev).dev; (*cs47l90).core.num_inputs = 10;
    ret = madera_core_init(&mut (*cs47l90).core); if ret != 0 { return ret; }
    ret = madera_request_irq(madera, MADERA_IRQ_DSP_IRQ1, b"ADSP2 Compressed IRQ\0".as_ptr() as *const c_char, cs47l90_adsp2_irq, cs47l90 as *mut c_void);
    if ret != 0 { dev_err(&mut (*pdev).dev, b"Failed to request DSP IRQ: %d\n\0".as_ptr() as *const c_char, ret); madera_core_free(&mut (*cs47l90).core); return ret; }
    ret = madera_set_irq_wake(madera, MADERA_IRQ_DSP_IRQ1, 1); if ret != 0 { dev_warn(&mut (*pdev).dev, b"Failed to set DSP IRQ wake: %d\n\0".as_ptr() as *const c_char, ret); }
    for i in 0..CS47L90_NUM_ADSP {
        (*cs47l90).core.adsp[i].part = b"cs47l90\0".as_ptr() as *const c_char;
        (*cs47l90).core.adsp[i].cs_dsp.num = i as c_int + 1; (*cs47l90).core.adsp[i].cs_dsp.type_ = WMFW_ADSP2; (*cs47l90).core.adsp[i].cs_dsp.rev = 2; (*cs47l90).core.adsp[i].cs_dsp.dev = (*madera).dev; (*cs47l90).core.adsp[i].cs_dsp.regmap = (*madera).regmap_32bit;
        (*cs47l90).core.adsp[i].cs_dsp.base = cs47l90_dsp_control_bases[i];
        (*cs47l90).core.adsp[i].cs_dsp.mem = match i {0 => cs47l90_dsp1_regions.as_ptr(),1 => cs47l90_dsp2_regions.as_ptr(),2 => cs47l90_dsp3_regions.as_ptr(),3 => cs47l90_dsp4_regions.as_ptr(),4 => cs47l90_dsp5_regions.as_ptr(),5 => cs47l90_dsp6_regions.as_ptr(),_ => cs47l90_dsp7_regions.as_ptr()};
        (*cs47l90).core.adsp[i].cs_dsp.num_mems = cs47l90_dsp1_regions.len(); (*cs47l90).core.adsp[i].cs_dsp.lock_regions = CS_ADSP2_REGION_1_9;
        ret = wm_adsp2_init(&mut (*cs47l90).core.adsp[i]);
        if ret == 0 { ret = madera_init_bus_error_irq(&mut (*cs47l90).core, i as c_int, wm_adsp2_bus_error); if ret != 0 { wm_adsp2_remove(&mut (*cs47l90).core.adsp[i]); } }
        if ret != 0 { let mut j = i; while j > 0 { j -= 1; madera_free_bus_error_irq(&mut (*cs47l90).core, j as c_int); wm_adsp2_remove(&mut (*cs47l90).core.adsp[j]); } madera_set_irq_wake(madera, MADERA_IRQ_DSP_IRQ1, 0); madera_free_irq(madera, MADERA_IRQ_DSP_IRQ1, cs47l90 as *mut c_void); madera_core_free(&mut (*cs47l90).core); return ret; }
    }
    madera_init_fll(madera, 1, MADERA_FLL1_CONTROL_1 - 1, &mut (*cs47l90).fll[0]); madera_init_fll(madera, 2, MADERA_FLL2_CONTROL_1 - 1, &mut (*cs47l90).fll[1]); madera_init_fll(madera, 4, MADERA_FLLAO_CONTROL_1 - 1, &mut (*cs47l90).fll[2]);
    for i in 0..cs47l90_dai.len() { madera_init_dai(&mut (*cs47l90).core, i as c_int); }
    for i in 0..cs47l90_digital_vu.len() { regmap_update_bits((*madera).regmap, cs47l90_digital_vu[i], CS47L90_DIG_VU, CS47L90_DIG_VU); }
    pm_runtime_enable(&mut (*pdev).dev); pm_runtime_idle(&mut (*pdev).dev);
    ret = devm_snd_soc_register_component(&mut (*pdev).dev, &soc_component_dev_cs47l90, cs47l90_dai.as_mut_ptr(), cs47l90_dai.len() as c_int);
    if ret < 0 { dev_err(&mut (*pdev).dev, b"Failed to register component: %d\n\0".as_ptr() as *const c_char, ret); pm_runtime_disable(&mut (*pdev).dev); for i in 0..CS47L90_NUM_ADSP { madera_free_bus_error_irq(&mut (*cs47l90).core, i as c_int); wm_adsp2_remove(&mut (*cs47l90).core.adsp[i]); } madera_set_irq_wake(madera, MADERA_IRQ_DSP_IRQ1, 0); madera_free_irq(madera, MADERA_IRQ_DSP_IRQ1, cs47l90 as *mut c_void); madera_core_free(&mut (*cs47l90).core); }
    ret
}

unsafe extern "C" fn cs47l90_remove(pdev: *mut platform_device) {
    let cs47l90 = platform_get_drvdata(pdev) as *mut cs47l90;
    pm_runtime_disable(&mut (*pdev).dev);
    for i in 0..CS47L90_NUM_ADSP { madera_free_bus_error_irq(&mut (*cs47l90).core, i as c_int); wm_adsp2_remove(&mut (*cs47l90).core.adsp[i]); }
    madera_set_irq_wake((*cs47l90).core.madera, MADERA_IRQ_DSP_IRQ1, 0); madera_free_irq((*cs47l90).core.madera, MADERA_IRQ_DSP_IRQ1, cs47l90 as *mut c_void); madera_core_free(&mut (*cs47l90).core);
}

static mut cs47l90_codec_driver: platform_driver = platform_driver { _private: [] };

// module_platform_driver(cs47l90_codec_driver);
// MODULE_SOFTDEP("pre: madera irq-madera arizona-micsupp");
// MODULE_DESCRIPTION("ASoC CS47L90 driver");
// MODULE_AUTHOR("Nikesh Oswal <nikesh@opensource.cirrus.com>");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:cs47l90-codec");

/*
 * The original C file contains large static ASoC tables constructed from
 * Linux C macros (SOC_*, SND_SOC_DAPM_*, MADERA_*, WM_ADSP_*). Those macro
 * invocations are preserved below verbatim as source-level translation notes;
 * their Rust executable forms depend on translated definitions of the same
 * external macros/types from the wider repository.
 */
// SPDX-License-Identifier: GPL-2.0-only
//
// ALSA SoC Audio driver for CS47L90 codec
//
// Copyright (C) 2015-2019 Cirrus Logic, Inc. and
//                         Cirrus Logic International Semiconductor Ltd.
//









struct cs47l90 {
	struct madera_priv core;
	struct madera_fll fll[3];
};

static const struct cs_dsp_region cs47l90_dsp1_regions[] = {
	{ .type = WMFW_ADSP2_PM, .base = 0x080000 },
	{ .type = WMFW_ADSP2_ZM, .base = 0x0e0000 },
	{ .type = WMFW_ADSP2_XM, .base = 0x0a0000 },
	{ .type = WMFW_ADSP2_YM, .base = 0x0c0000 },
};

static const struct cs_dsp_region cs47l90_dsp2_regions[] = {
	{ .type = WMFW_ADSP2_PM, .base = 0x100000 },
	{ .type = WMFW_ADSP2_ZM, .base = 0x160000 },
	{ .type = WMFW_ADSP2_XM, .base = 0x120000 },
	{ .type = WMFW_ADSP2_YM, .base = 0x140000 },
};

static const struct cs_dsp_region cs47l90_dsp3_regions[] = {
	{ .type = WMFW_ADSP2_PM, .base = 0x180000 },
	{ .type = WMFW_ADSP2_ZM, .base = 0x1e0000 },
	{ .type = WMFW_ADSP2_XM, .base = 0x1a0000 },
	{ .type = WMFW_ADSP2_YM, .base = 0x1c0000 },
};

static const struct cs_dsp_region cs47l90_dsp4_regions[] = {
	{ .type = WMFW_ADSP2_PM, .base = 0x200000 },
	{ .type = WMFW_ADSP2_ZM, .base = 0x260000 },
	{ .type = WMFW_ADSP2_XM, .base = 0x220000 },
	{ .type = WMFW_ADSP2_YM, .base = 0x240000 },
};

static const struct cs_dsp_region cs47l90_dsp5_regions[] = {
	{ .type = WMFW_ADSP2_PM, .base = 0x280000 },
	{ .type = WMFW_ADSP2_ZM, .base = 0x2e0000 },
	{ .type = WMFW_ADSP2_XM, .base = 0x2a0000 },
	{ .type = WMFW_ADSP2_YM, .base = 0x2c0000 },
};

static const struct cs_dsp_region cs47l90_dsp6_regions[] = {
	{ .type = WMFW_ADSP2_PM, .base = 0x300000 },
	{ .type = WMFW_ADSP2_ZM, .base = 0x360000 },
	{ .type = WMFW_ADSP2_XM, .base = 0x320000 },
	{ .type = WMFW_ADSP2_YM, .base = 0x340000 },
};

static const struct cs_dsp_region cs47l90_dsp7_regions[] = {
	{ .type = WMFW_ADSP2_PM, .base = 0x380000 },
	{ .type = WMFW_ADSP2_ZM, .base = 0x3e0000 },
	{ .type = WMFW_ADSP2_XM, .base = 0x3a0000 },
	{ .type = WMFW_ADSP2_YM, .base = 0x3c0000 },
};

static const struct cs_dsp_region *cs47l90_dsp_regions[] = {
	cs47l90_dsp1_regions,
	cs47l90_dsp2_regions,
	cs47l90_dsp3_regions,
	cs47l90_dsp4_regions,
	cs47l90_dsp5_regions,
	cs47l90_dsp6_regions,
	cs47l90_dsp7_regions,
};

static const int cs47l90_dsp_control_bases[] = {
	MADERA_DSP1_CONFIG_1,
	MADERA_DSP2_CONFIG_1,
	MADERA_DSP3_CONFIG_1,
	MADERA_DSP4_CONFIG_1,
	MADERA_DSP5_CONFIG_1,
	MADERA_DSP6_CONFIG_1,
	MADERA_DSP7_CONFIG_1,
};

static int cs47l90_adsp_power_ev(struct snd_soc_dapm_widget *w,
				 struct snd_kcontrol *kcontrol,
				 int event)
{
	struct snd_soc_component *component =
		snd_soc_dapm_to_component(w->dapm);
	struct cs47l90 *cs47l90 = snd_soc_component_get_drvdata(component);
	struct madera_priv *priv = &cs47l90->core;
	struct madera *madera = priv->madera;
	unsigned int freq;
	int ret;

	ret = regmap_read(madera->regmap, MADERA_DSP_CLOCK_2, &freq);
	if (ret != 0) {
		dev_err(madera->dev,
			"Failed to read MADERA_DSP_CLOCK_2: %d\n", ret);
		return ret;
	}

	switch (event) {
	case SND_SOC_DAPM_PRE_PMU:
		ret = madera_set_adsp_clk(&cs47l90->core, w->shift, freq);
		if (ret)
			return ret;
		break;
	default:
		break;
	}

	return wm_adsp_early_event(w, kcontrol, event);
}

#define CS47L90_NG_SRC(name, base) \
	SOC_SINGLE(name " NG HPOUT1L Switch",  base,  0, 1, 0), \
	SOC_SINGLE(name " NG HPOUT1R Switch",  base,  1, 1, 0), \
	SOC_SINGLE(name " NG HPOUT2L Switch",  base,  2, 1, 0), \
	SOC_SINGLE(name " NG HPOUT2R Switch",  base,  3, 1, 0), \
	SOC_SINGLE(name " NG HPOUT3L Switch",  base,  4, 1, 0), \
	SOC_SINGLE(name " NG HPOUT3R Switch",  base,  5, 1, 0), \
	SOC_SINGLE(name " NG SPKDAT1L Switch", base,  8, 1, 0), \
	SOC_SINGLE(name " NG SPKDAT1R Switch", base,  9, 1, 0)

#define CS47L90_RXANC_INPUT_ROUTES(widget, name) \
	{ widget, NULL, name " NG Mux" }, \
	{ name " NG Internal", NULL, "RXANC NG Clock" }, \
	{ name " NG Internal", NULL, name " Channel" }, \
	{ name " NG External", NULL, "RXANC NG External Clock" }, \
	{ name " NG External", NULL, name " Channel" }, \
	{ name " NG Mux", "None", name " Channel" }, \
	{ name " NG Mux", "Internal", name " NG Internal" }, \
	{ name " NG Mux", "External", name " NG External" }, \
	{ name " Channel", "Left", name " Left Input" }, \
	{ name " Channel", "Combine", name " Left Input" }, \
	{ name " Channel", "Right", name " Right Input" }, \
	{ name " Channel", "Combine", name " Right Input" }, \
	{ name " Left Input", "IN1", "IN1L" }, \
	{ name " Right Input", "IN1", "IN1R" }, \
	{ name " Left Input", "IN2", "IN2L" }, \
	{ name " Right Input", "IN2", "IN2R" }, \
	{ name " Left Input", "IN3", "IN3L" }, \
	{ name " Right Input", "IN3", "IN3R" }, \
	{ name " Left Input", "IN4", "IN4L" }, \
	{ name " Right Input", "IN4", "IN4R" }, \
	{ name " Left Input", "IN5", "IN5L" }, \
	{ name " Right Input", "IN5", "IN5R" }

#define CS47L90_RXANC_OUTPUT_ROUTES(widget, name) \
	{ widget, NULL, name " ANC Source" }, \
	{ name " ANC Source", "RXANCL", "RXANCL" }, \
	{ name " ANC Source", "RXANCR", "RXANCR" }

static const struct snd_kcontrol_new cs47l90_snd_controls[] = {
SOC_ENUM("IN1 OSR", madera_in_dmic_osr[0]),
SOC_ENUM("IN2 OSR", madera_in_dmic_osr[1]),
SOC_ENUM("IN3 OSR", madera_in_dmic_osr[2]),
SOC_ENUM("IN4 OSR", madera_in_dmic_osr[3]),
SOC_ENUM("IN5 OSR", madera_in_dmic_osr[4]),

SOC_SINGLE_RANGE_TLV("IN1L Volume", MADERA_IN1L_CONTROL,
		     MADERA_IN1L_PGA_VOL_SHIFT, 0x40, 0x5f, 0, madera_ana_tlv),
SOC_SINGLE_RANGE_TLV("IN1R Volume", MADERA_IN1R_CONTROL,
		     MADERA_IN1R_PGA_VOL_SHIFT, 0x40, 0x5f, 0, madera_ana_tlv),
SOC_SINGLE_RANGE_TLV("IN2L Volume", MADERA_IN2L_CONTROL,
		     MADERA_IN2L_PGA_VOL_SHIFT, 0x40, 0x5f, 0, madera_ana_tlv),
SOC_SINGLE_RANGE_TLV("IN2R Volume", MADERA_IN2R_CONTROL,
		     MADERA_IN2R_PGA_VOL_SHIFT, 0x40, 0x5f, 0, madera_ana_tlv),

SOC_ENUM("IN HPF Cutoff Frequency", madera_in_hpf_cut_enum),

SOC_SINGLE_EXT("IN1L LP Switch", MADERA_ADC_DIGITAL_VOLUME_1L,
	       MADERA_IN1L_LP_MODE_SHIFT, 1, 0,
	       snd_soc_get_volsw, madera_lp_mode_put),
SOC_SINGLE_EXT("IN1R LP Switch", MADERA_ADC_DIGITAL_VOLUME_1R,
	       MADERA_IN1R_LP_MODE_SHIFT, 1, 0,
	       snd_soc_get_volsw, madera_lp_mode_put),
SOC_SINGLE_EXT("IN2L LP Switch", MADERA_ADC_DIGITAL_VOLUME_2L,
	       MADERA_IN2L_LP_MODE_SHIFT, 1, 0,
	       snd_soc_get_volsw, madera_lp_mode_put),
SOC_SINGLE_EXT("IN2R LP Switch", MADERA_ADC_DIGITAL_VOLUME_2R,
	       MADERA_IN2R_LP_MODE_SHIFT, 1, 0,
	       snd_soc_get_volsw, madera_lp_mode_put),

SOC_SINGLE("IN1L HPF Switch", MADERA_IN1L_CONTROL,
	   MADERA_IN1L_HPF_SHIFT, 1, 0),
SOC_SINGLE("IN1R HPF Switch", MADERA_IN1R_CONTROL,
	   MADERA_IN1R_HPF_SHIFT, 1, 0),
SOC_SINGLE("IN2L HPF Switch", MADERA_IN2L_CONTROL,
	   MADERA_IN2L_HPF_SHIFT, 1, 0),
SOC_SINGLE("IN2R HPF Switch", MADERA_IN2R_CONTROL,
	   MADERA_IN2R_HPF_SHIFT, 1, 0),
SOC_SINGLE("IN3L HPF Switch", MADERA_IN3L_CONTROL,
	   MADERA_IN3L_HPF_SHIFT, 1, 0),
SOC_SINGLE("IN3R HPF Switch", MADERA_IN3R_CONTROL,
	   MADERA_IN3R_HPF_SHIFT, 1, 0),
SOC_SINGLE("IN4L HPF Switch", MADERA_IN4L_CONTROL,
	   MADERA_IN4L_HPF_SHIFT, 1, 0),
SOC_SINGLE("IN4R HPF Switch", MADERA_IN4R_CONTROL,
	   MADERA_IN4R_HPF_SHIFT, 1, 0),
SOC_SINGLE("IN5L HPF Switch", MADERA_IN5L_CONTROL,
	   MADERA_IN5L_HPF_SHIFT, 1, 0),
SOC_SINGLE("IN5R HPF Switch", MADERA_IN5R_CONTROL,
	   MADERA_IN5R_HPF_SHIFT, 1, 0),

SOC_SINGLE_TLV("IN1L Digital Volume", MADERA_ADC_DIGITAL_VOLUME_1L,
	       MADERA_IN1L_DIG_VOL_SHIFT, 0xbf, 0, madera_digital_tlv),
SOC_SINGLE_TLV("IN1R Digital Volume", MADERA_ADC_DIGITAL_VOLUME_1R,
	       MADERA_IN1R_DIG_VOL_SHIFT, 0xbf, 0, madera_digital_tlv),
SOC_SINGLE_TLV("IN2L Digital Volume", MADERA_ADC_DIGITAL_VOLUME_2L,
	       MADERA_IN2L_DIG_VOL_SHIFT, 0xbf, 0, madera_digital_tlv),
SOC_SINGLE_TLV("IN2R Digital Volume", MADERA_ADC_DIGITAL_VOLUME_2R,
	       MADERA_IN2R_DIG_VOL_SHIFT, 0xbf, 0, madera_digital_tlv),
SOC_SINGLE_TLV("IN3L Digital Volume", MADERA_ADC_DIGITAL_VOLUME_3L,
	       MADERA_IN3L_DIG_VOL_SHIFT, 0xbf, 0, madera_digital_tlv),
SOC_SINGLE_TLV("IN3R Digital Volume", MADERA_ADC_DIGITAL_VOLUME_3R,
	       MADERA_IN3R_DIG_VOL_SHIFT, 0xbf, 0, madera_digital_tlv),
SOC_SINGLE_TLV("IN4L Digital Volume", MADERA_ADC_DIGITAL_VOLUME_4L,
	       MADERA_IN4L_DIG_VOL_SHIFT, 0xbf, 0, madera_digital_tlv),
SOC_SINGLE_TLV("IN4R Digital Volume", MADERA_ADC_DIGITAL_VOLUME_4R,
	       MADERA_IN4R_DIG_VOL_SHIFT, 0xbf, 0, madera_digital_tlv),
SOC_SINGLE_TLV("IN5L Digital Volume", MADERA_ADC_DIGITAL_VOLUME_5L,
	       MADERA_IN5L_DIG_VOL_SHIFT, 0xbf, 0, madera_digital_tlv),
SOC_SINGLE_TLV("IN5R Digital Volume", MADERA_ADC_DIGITAL_VOLUME_5R,
	       MADERA_IN5R_DIG_VOL_SHIFT, 0xbf, 0, madera_digital_tlv),

SOC_ENUM("Input Ramp Up", madera_in_vi_ramp),
SOC_ENUM("Input Ramp Down", madera_in_vd_ramp),

SND_SOC_BYTES("RXANC Coefficients", MADERA_ANC_COEFF_START,
	      MADERA_ANC_COEFF_END - MADERA_ANC_COEFF_START + 1),
SND_SOC_BYTES("RXANCL Config", MADERA_FCL_FILTER_CONTROL, 1),
SND_SOC_BYTES("RXANCL Coefficients", MADERA_FCL_COEFF_START,
	      MADERA_FCL_COEFF_END - MADERA_FCL_COEFF_START + 1),
SND_SOC_BYTES("RXANCR Config", MADERA_FCR_FILTER_CONTROL, 1),
SND_SOC_BYTES("RXANCR Coefficients", MADERA_FCR_COEFF_START,
	      MADERA_FCR_COEFF_END - MADERA_FCR_COEFF_START + 1),

MADERA_MIXER_CONTROLS("EQ1", MADERA_EQ1MIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("EQ2", MADERA_EQ2MIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("EQ3", MADERA_EQ3MIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("EQ4", MADERA_EQ4MIX_INPUT_1_SOURCE),

MADERA_EQ_CONTROL("EQ1 Coefficients", MADERA_EQ1_2),
SOC_SINGLE_TLV("EQ1 B1 Volume", MADERA_EQ1_1, MADERA_EQ1_B1_GAIN_SHIFT,
	       24, 0, madera_eq_tlv),
SOC_SINGLE_TLV("EQ1 B2 Volume", MADERA_EQ1_1, MADERA_EQ1_B2_GAIN_SHIFT,
	       24, 0, madera_eq_tlv),
SOC_SINGLE_TLV("EQ1 B3 Volume", MADERA_EQ1_1, MADERA_EQ1_B3_GAIN_SHIFT,
	       24, 0, madera_eq_tlv),
SOC_SINGLE_TLV("EQ1 B4 Volume", MADERA_EQ1_2, MADERA_EQ1_B4_GAIN_SHIFT,
	       24, 0, madera_eq_tlv),
SOC_SINGLE_TLV("EQ1 B5 Volume", MADERA_EQ1_2, MADERA_EQ1_B5_GAIN_SHIFT,
	       24, 0, madera_eq_tlv),

MADERA_EQ_CONTROL("EQ2 Coefficients", MADERA_EQ2_2),
SOC_SINGLE_TLV("EQ2 B1 Volume", MADERA_EQ2_1, MADERA_EQ2_B1_GAIN_SHIFT,
	       24, 0, madera_eq_tlv),
SOC_SINGLE_TLV("EQ2 B2 Volume", MADERA_EQ2_1, MADERA_EQ2_B2_GAIN_SHIFT,
	       24, 0, madera_eq_tlv),
SOC_SINGLE_TLV("EQ2 B3 Volume", MADERA_EQ2_1, MADERA_EQ2_B3_GAIN_SHIFT,
	       24, 0, madera_eq_tlv),
SOC_SINGLE_TLV("EQ2 B4 Volume", MADERA_EQ2_2, MADERA_EQ2_B4_GAIN_SHIFT,
	       24, 0, madera_eq_tlv),
SOC_SINGLE_TLV("EQ2 B5 Volume", MADERA_EQ2_2, MADERA_EQ2_B5_GAIN_SHIFT,
	       24, 0, madera_eq_tlv),

MADERA_EQ_CONTROL("EQ3 Coefficients", MADERA_EQ3_2),
SOC_SINGLE_TLV("EQ3 B1 Volume", MADERA_EQ3_1, MADERA_EQ3_B1_GAIN_SHIFT,
	       24, 0, madera_eq_tlv),
SOC_SINGLE_TLV("EQ3 B2 Volume", MADERA_EQ3_1, MADERA_EQ3_B2_GAIN_SHIFT,
	       24, 0, madera_eq_tlv),
SOC_SINGLE_TLV("EQ3 B3 Volume", MADERA_EQ3_1, MADERA_EQ3_B3_GAIN_SHIFT,
	       24, 0, madera_eq_tlv),
SOC_SINGLE_TLV("EQ3 B4 Volume", MADERA_EQ3_2, MADERA_EQ3_B4_GAIN_SHIFT,
	       24, 0, madera_eq_tlv),
SOC_SINGLE_TLV("EQ3 B5 Volume", MADERA_EQ3_2, MADERA_EQ3_B5_GAIN_SHIFT,
	       24, 0, madera_eq_tlv),

MADERA_EQ_CONTROL("EQ4 Coefficients", MADERA_EQ4_2),
SOC_SINGLE_TLV("EQ4 B1 Volume", MADERA_EQ4_1, MADERA_EQ4_B1_GAIN_SHIFT,
	       24, 0, madera_eq_tlv),
SOC_SINGLE_TLV("EQ4 B2 Volume", MADERA_EQ4_1, MADERA_EQ4_B2_GAIN_SHIFT,
	       24, 0, madera_eq_tlv),
SOC_SINGLE_TLV("EQ4 B3 Volume", MADERA_EQ4_1, MADERA_EQ4_B3_GAIN_SHIFT,
	       24, 0, madera_eq_tlv),
SOC_SINGLE_TLV("EQ4 B4 Volume", MADERA_EQ4_2, MADERA_EQ4_B4_GAIN_SHIFT,
	       24, 0, madera_eq_tlv),
SOC_SINGLE_TLV("EQ4 B5 Volume", MADERA_EQ4_2, MADERA_EQ4_B5_GAIN_SHIFT,
	       24, 0, madera_eq_tlv),

MADERA_MIXER_CONTROLS("DRC1L", MADERA_DRC1LMIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("DRC1R", MADERA_DRC1RMIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("DRC2L", MADERA_DRC2LMIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("DRC2R", MADERA_DRC2RMIX_INPUT_1_SOURCE),

SND_SOC_BYTES_MASK("DRC1", MADERA_DRC1_CTRL1, 5,
		   MADERA_DRC1R_ENA | MADERA_DRC1L_ENA),
SND_SOC_BYTES_MASK("DRC2", MADERA_DRC2_CTRL1, 5,
		   MADERA_DRC2R_ENA | MADERA_DRC2L_ENA),

MADERA_MIXER_CONTROLS("LHPF1", MADERA_HPLP1MIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("LHPF2", MADERA_HPLP2MIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("LHPF3", MADERA_HPLP3MIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("LHPF4", MADERA_HPLP4MIX_INPUT_1_SOURCE),

MADERA_LHPF_CONTROL("LHPF1 Coefficients", MADERA_HPLPF1_2),
MADERA_LHPF_CONTROL("LHPF2 Coefficients", MADERA_HPLPF2_2),
MADERA_LHPF_CONTROL("LHPF3 Coefficients", MADERA_HPLPF3_2),
MADERA_LHPF_CONTROL("LHPF4 Coefficients", MADERA_HPLPF4_2),

SOC_ENUM("LHPF1 Mode", madera_lhpf1_mode),
SOC_ENUM("LHPF2 Mode", madera_lhpf2_mode),
SOC_ENUM("LHPF3 Mode", madera_lhpf3_mode),
SOC_ENUM("LHPF4 Mode", madera_lhpf4_mode),

MADERA_RATE_ENUM("ISRC1 FSL", madera_isrc_fsl[0]),
MADERA_RATE_ENUM("ISRC2 FSL", madera_isrc_fsl[1]),
MADERA_RATE_ENUM("ISRC3 FSL", madera_isrc_fsl[2]),
MADERA_RATE_ENUM("ISRC4 FSL", madera_isrc_fsl[3]),
MADERA_RATE_ENUM("ISRC1 FSH", madera_isrc_fsh[0]),
MADERA_RATE_ENUM("ISRC2 FSH", madera_isrc_fsh[1]),
MADERA_RATE_ENUM("ISRC3 FSH", madera_isrc_fsh[2]),
MADERA_RATE_ENUM("ISRC4 FSH", madera_isrc_fsh[3]),
MADERA_RATE_ENUM("ASRC1 Rate 1", madera_asrc1_rate[0]),
MADERA_RATE_ENUM("ASRC1 Rate 2", madera_asrc1_rate[1]),
MADERA_RATE_ENUM("ASRC2 Rate 1", madera_asrc2_rate[0]),
MADERA_RATE_ENUM("ASRC2 Rate 2", madera_asrc2_rate[1]),

WM_ADSP2_PRELOAD_SWITCH("DSP1", 1),
WM_ADSP2_PRELOAD_SWITCH("DSP2", 2),
WM_ADSP2_PRELOAD_SWITCH("DSP3", 3),
WM_ADSP2_PRELOAD_SWITCH("DSP4", 4),
WM_ADSP2_PRELOAD_SWITCH("DSP5", 5),
WM_ADSP2_PRELOAD_SWITCH("DSP6", 6),
WM_ADSP2_PRELOAD_SWITCH("DSP7", 7),

MADERA_MIXER_CONTROLS("DSP1L", MADERA_DSP1LMIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("DSP1R", MADERA_DSP1RMIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("DSP2L", MADERA_DSP2LMIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("DSP2R", MADERA_DSP2RMIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("DSP3L", MADERA_DSP3LMIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("DSP3R", MADERA_DSP3RMIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("DSP4L", MADERA_DSP4LMIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("DSP4R", MADERA_DSP4RMIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("DSP5L", MADERA_DSP5LMIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("DSP5R", MADERA_DSP5RMIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("DSP6L", MADERA_DSP6LMIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("DSP6R", MADERA_DSP6RMIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("DSP7L", MADERA_DSP7LMIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("DSP7R", MADERA_DSP7RMIX_INPUT_1_SOURCE),

SOC_SINGLE_TLV("Noise Generator Volume", MADERA_COMFORT_NOISE_GENERATOR,
	       MADERA_NOISE_GEN_GAIN_SHIFT, 0x16, 0, madera_noise_tlv),

MADERA_MIXER_CONTROLS("HPOUT1L", MADERA_OUT1LMIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("HPOUT1R", MADERA_OUT1RMIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("HPOUT2L", MADERA_OUT2LMIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("HPOUT2R", MADERA_OUT2RMIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("HPOUT3L", MADERA_OUT3LMIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("HPOUT3R", MADERA_OUT3RMIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("SPKDAT1L", MADERA_OUT5LMIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("SPKDAT1R", MADERA_OUT5RMIX_INPUT_1_SOURCE),

SOC_SINGLE("HPOUT1 SC Protect Switch", MADERA_HP1_SHORT_CIRCUIT_CTRL,
	   MADERA_HP1_SC_ENA_SHIFT, 1, 0),
SOC_SINGLE("HPOUT2 SC Protect Switch", MADERA_HP2_SHORT_CIRCUIT_CTRL,
	   MADERA_HP2_SC_ENA_SHIFT, 1, 0),
SOC_SINGLE("HPOUT3 SC Protect Switch", MADERA_HP3_SHORT_CIRCUIT_CTRL,
	   MADERA_HP3_SC_ENA_SHIFT, 1, 0),

SOC_SINGLE("SPKDAT1 High Performance Switch", MADERA_OUTPUT_PATH_CONFIG_5L,
	   MADERA_OUT5_OSR_SHIFT, 1, 0),

SOC_DOUBLE_R("HPOUT1 Digital Switch", MADERA_DAC_DIGITAL_VOLUME_1L,
	     MADERA_DAC_DIGITAL_VOLUME_1R, MADERA_OUT1L_MUTE_SHIFT, 1, 1),
SOC_DOUBLE_R("HPOUT2 Digital Switch", MADERA_DAC_DIGITAL_VOLUME_2L,
	     MADERA_DAC_DIGITAL_VOLUME_2R, MADERA_OUT2L_MUTE_SHIFT, 1, 1),
SOC_DOUBLE_R("HPOUT3 Digital Switch", MADERA_DAC_DIGITAL_VOLUME_3L,
	     MADERA_DAC_DIGITAL_VOLUME_3R, MADERA_OUT3L_MUTE_SHIFT, 1, 1),
SOC_DOUBLE_R("SPKDAT1 Digital Switch", MADERA_DAC_DIGITAL_VOLUME_5L,
	     MADERA_DAC_DIGITAL_VOLUME_5R, MADERA_OUT5L_MUTE_SHIFT, 1, 1),

SOC_DOUBLE_R_TLV("HPOUT1 Digital Volume", MADERA_DAC_DIGITAL_VOLUME_1L,
		 MADERA_DAC_DIGITAL_VOLUME_1R, MADERA_OUT1L_VOL_SHIFT,
		 0xbf, 0, madera_digital_tlv),
SOC_DOUBLE_R_TLV("HPOUT2 Digital Volume", MADERA_DAC_DIGITAL_VOLUME_2L,
		 MADERA_DAC_DIGITAL_VOLUME_2R, MADERA_OUT2L_VOL_SHIFT,
		 0xbf, 0, madera_digital_tlv),
SOC_DOUBLE_R_TLV("HPOUT3 Digital Volume", MADERA_DAC_DIGITAL_VOLUME_3L,
		 MADERA_DAC_DIGITAL_VOLUME_3R, MADERA_OUT3L_VOL_SHIFT,
		 0xbf, 0, madera_digital_tlv),
SOC_DOUBLE_R_TLV("SPKDAT1 Digital Volume", MADERA_DAC_DIGITAL_VOLUME_5L,
		 MADERA_DAC_DIGITAL_VOLUME_5R, MADERA_OUT5L_VOL_SHIFT,
		 0xbf, 0, madera_digital_tlv),

SOC_DOUBLE("SPKDAT1 Switch", MADERA_PDM_SPK1_CTRL_1, MADERA_SPK1L_MUTE_SHIFT,
	   MADERA_SPK1R_MUTE_SHIFT, 1, 1),

SOC_ENUM("Output Ramp Up", madera_out_vi_ramp),
SOC_ENUM("Output Ramp Down", madera_out_vd_ramp),

SOC_SINGLE("Noise Gate Switch", MADERA_NOISE_GATE_CONTROL,
	   MADERA_NGATE_ENA_SHIFT, 1, 0),
SOC_SINGLE_TLV("Noise Gate Threshold Volume", MADERA_NOISE_GATE_CONTROL,
	       MADERA_NGATE_THR_SHIFT, 7, 1, madera_ng_tlv),
SOC_ENUM("Noise Gate Hold", madera_ng_hold),

SOC_ENUM_EXT("DFC1RX Width", madera_dfc_width[0],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC1RX Type", madera_dfc_type[0],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC1TX Width", madera_dfc_width[1],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC1TX Type", madera_dfc_type[1],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC2RX Width", madera_dfc_width[2],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC2RX Type", madera_dfc_type[2],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC2TX Width", madera_dfc_width[3],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC2TX Type", madera_dfc_type[3],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC3RX Width", madera_dfc_width[4],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC3RX Type", madera_dfc_type[4],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC3TX Width", madera_dfc_width[5],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC3TX Type", madera_dfc_type[5],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC4RX Width", madera_dfc_width[6],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC4RX Type", madera_dfc_type[6],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC4TX Width", madera_dfc_width[7],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC4TX Type", madera_dfc_type[7],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC5RX Width", madera_dfc_width[8],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC5RX Type", madera_dfc_type[8],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC5TX Width", madera_dfc_width[9],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC5TX Type", madera_dfc_type[9],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC6RX Width", madera_dfc_width[10],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC6RX Type", madera_dfc_type[10],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC6TX Width", madera_dfc_width[11],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC6TX Type", madera_dfc_type[11],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC7RX Width", madera_dfc_width[12],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC7RX Type", madera_dfc_type[12],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC7TX Width", madera_dfc_width[13],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC7TX Type", madera_dfc_type[13],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC8RX Width", madera_dfc_width[14],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC8RX Type", madera_dfc_type[14],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC8TX Width", madera_dfc_width[15],
	     snd_soc_get_enum_double, madera_dfc_put),
SOC_ENUM_EXT("DFC8TX Type", madera_dfc_type[15],
	     snd_soc_get_enum_double, madera_dfc_put),

CS47L90_NG_SRC("HPOUT1L", MADERA_NOISE_GATE_SELECT_1L),
CS47L90_NG_SRC("HPOUT1R", MADERA_NOISE_GATE_SELECT_1R),
CS47L90_NG_SRC("HPOUT2L", MADERA_NOISE_GATE_SELECT_2L),
CS47L90_NG_SRC("HPOUT2R", MADERA_NOISE_GATE_SELECT_2R),
CS47L90_NG_SRC("HPOUT3L", MADERA_NOISE_GATE_SELECT_3L),
CS47L90_NG_SRC("HPOUT3R", MADERA_NOISE_GATE_SELECT_3R),
CS47L90_NG_SRC("SPKDAT1L", MADERA_NOISE_GATE_SELECT_5L),
CS47L90_NG_SRC("SPKDAT1R", MADERA_NOISE_GATE_SELECT_5R),

MADERA_MIXER_CONTROLS("AIF1TX1", MADERA_AIF1TX1MIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("AIF1TX2", MADERA_AIF1TX2MIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("AIF1TX3", MADERA_AIF1TX3MIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("AIF1TX4", MADERA_AIF1TX4MIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("AIF1TX5", MADERA_AIF1TX5MIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("AIF1TX6", MADERA_AIF1TX6MIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("AIF1TX7", MADERA_AIF1TX7MIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("AIF1TX8", MADERA_AIF1TX8MIX_INPUT_1_SOURCE),

MADERA_MIXER_CONTROLS("AIF2TX1", MADERA_AIF2TX1MIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("AIF2TX2", MADERA_AIF2TX2MIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("AIF2TX3", MADERA_AIF2TX3MIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("AIF2TX4", MADERA_AIF2TX4MIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("AIF2TX5", MADERA_AIF2TX5MIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("AIF2TX6", MADERA_AIF2TX6MIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("AIF2TX7", MADERA_AIF2TX7MIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("AIF2TX8", MADERA_AIF2TX8MIX_INPUT_1_SOURCE),

MADERA_MIXER_CONTROLS("AIF3TX1", MADERA_AIF3TX1MIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("AIF3TX2", MADERA_AIF3TX2MIX_INPUT_1_SOURCE),

MADERA_MIXER_CONTROLS("AIF4TX1", MADERA_AIF4TX1MIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("AIF4TX2", MADERA_AIF4TX2MIX_INPUT_1_SOURCE),

MADERA_MIXER_CONTROLS("SLIMTX1", MADERA_SLIMTX1MIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("SLIMTX2", MADERA_SLIMTX2MIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("SLIMTX3", MADERA_SLIMTX3MIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("SLIMTX4", MADERA_SLIMTX4MIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("SLIMTX5", MADERA_SLIMTX5MIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("SLIMTX6", MADERA_SLIMTX6MIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("SLIMTX7", MADERA_SLIMTX7MIX_INPUT_1_SOURCE),
MADERA_MIXER_CONTROLS("SLIMTX8", MADERA_SLIMTX8MIX_INPUT_1_SOURCE),

MADERA_GAINMUX_CONTROLS("SPDIF1TX1", MADERA_SPDIF1TX1MIX_INPUT_1_SOURCE),
MADERA_GAINMUX_CONTROLS("SPDIF1TX2", MADERA_SPDIF1TX2MIX_INPUT_1_SOURCE),

WM_ADSP_FW_CONTROL("DSP1", 0),
WM_ADSP_FW_CONTROL("DSP2", 1),
WM_ADSP_FW_CONTROL("DSP3", 2),
WM_ADSP_FW_CONTROL("DSP4", 3),
WM_ADSP_FW_CONTROL("DSP5", 4),
WM_ADSP_FW_CONTROL("DSP6", 5),
WM_ADSP_FW_CONTROL("DSP7", 6),
};

MADERA_MIXER_ENUMS(EQ1, MADERA_EQ1MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(EQ2, MADERA_EQ2MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(EQ3, MADERA_EQ3MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(EQ4, MADERA_EQ4MIX_INPUT_1_SOURCE);

MADERA_MIXER_ENUMS(DRC1L, MADERA_DRC1LMIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(DRC1R, MADERA_DRC1RMIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(DRC2L, MADERA_DRC2LMIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(DRC2R, MADERA_DRC2RMIX_INPUT_1_SOURCE);

MADERA_MIXER_ENUMS(LHPF1, MADERA_HPLP1MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(LHPF2, MADERA_HPLP2MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(LHPF3, MADERA_HPLP3MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(LHPF4, MADERA_HPLP4MIX_INPUT_1_SOURCE);

MADERA_MIXER_ENUMS(DSP1L, MADERA_DSP1LMIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(DSP1R, MADERA_DSP1RMIX_INPUT_1_SOURCE);
MADERA_DSP_AUX_ENUMS(DSP1, MADERA_DSP1AUX1MIX_INPUT_1_SOURCE);

MADERA_MIXER_ENUMS(DSP2L, MADERA_DSP2LMIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(DSP2R, MADERA_DSP2RMIX_INPUT_1_SOURCE);
MADERA_DSP_AUX_ENUMS(DSP2, MADERA_DSP2AUX1MIX_INPUT_1_SOURCE);

MADERA_MIXER_ENUMS(DSP3L, MADERA_DSP3LMIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(DSP3R, MADERA_DSP3RMIX_INPUT_1_SOURCE);
MADERA_DSP_AUX_ENUMS(DSP3, MADERA_DSP3AUX1MIX_INPUT_1_SOURCE);

MADERA_MIXER_ENUMS(DSP4L, MADERA_DSP4LMIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(DSP4R, MADERA_DSP4RMIX_INPUT_1_SOURCE);
MADERA_DSP_AUX_ENUMS(DSP4, MADERA_DSP4AUX1MIX_INPUT_1_SOURCE);

MADERA_MIXER_ENUMS(DSP5L, MADERA_DSP5LMIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(DSP5R, MADERA_DSP5RMIX_INPUT_1_SOURCE);
MADERA_DSP_AUX_ENUMS(DSP5, MADERA_DSP5AUX1MIX_INPUT_1_SOURCE);

MADERA_MIXER_ENUMS(DSP6L, MADERA_DSP6LMIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(DSP6R, MADERA_DSP6RMIX_INPUT_1_SOURCE);
MADERA_DSP_AUX_ENUMS(DSP6, MADERA_DSP6AUX1MIX_INPUT_1_SOURCE);

MADERA_MIXER_ENUMS(DSP7L, MADERA_DSP7LMIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(DSP7R, MADERA_DSP7RMIX_INPUT_1_SOURCE);
MADERA_DSP_AUX_ENUMS(DSP7, MADERA_DSP7AUX1MIX_INPUT_1_SOURCE);

MADERA_MIXER_ENUMS(PWM1, MADERA_PWM1MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(PWM2, MADERA_PWM2MIX_INPUT_1_SOURCE);

MADERA_MIXER_ENUMS(OUT1L, MADERA_OUT1LMIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(OUT1R, MADERA_OUT1RMIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(OUT2L, MADERA_OUT2LMIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(OUT2R, MADERA_OUT2RMIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(OUT3L, MADERA_OUT3LMIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(OUT3R, MADERA_OUT3RMIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(SPKDAT1L, MADERA_OUT5LMIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(SPKDAT1R, MADERA_OUT5RMIX_INPUT_1_SOURCE);

MADERA_MIXER_ENUMS(AIF1TX1, MADERA_AIF1TX1MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(AIF1TX2, MADERA_AIF1TX2MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(AIF1TX3, MADERA_AIF1TX3MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(AIF1TX4, MADERA_AIF1TX4MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(AIF1TX5, MADERA_AIF1TX5MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(AIF1TX6, MADERA_AIF1TX6MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(AIF1TX7, MADERA_AIF1TX7MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(AIF1TX8, MADERA_AIF1TX8MIX_INPUT_1_SOURCE);

MADERA_MIXER_ENUMS(AIF2TX1, MADERA_AIF2TX1MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(AIF2TX2, MADERA_AIF2TX2MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(AIF2TX3, MADERA_AIF2TX3MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(AIF2TX4, MADERA_AIF2TX4MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(AIF2TX5, MADERA_AIF2TX5MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(AIF2TX6, MADERA_AIF2TX6MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(AIF2TX7, MADERA_AIF2TX7MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(AIF2TX8, MADERA_AIF2TX8MIX_INPUT_1_SOURCE);

MADERA_MIXER_ENUMS(AIF3TX1, MADERA_AIF3TX1MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(AIF3TX2, MADERA_AIF3TX2MIX_INPUT_1_SOURCE);

MADERA_MIXER_ENUMS(AIF4TX1, MADERA_AIF4TX1MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(AIF4TX2, MADERA_AIF4TX2MIX_INPUT_1_SOURCE);

MADERA_MIXER_ENUMS(SLIMTX1, MADERA_SLIMTX1MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(SLIMTX2, MADERA_SLIMTX2MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(SLIMTX3, MADERA_SLIMTX3MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(SLIMTX4, MADERA_SLIMTX4MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(SLIMTX5, MADERA_SLIMTX5MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(SLIMTX6, MADERA_SLIMTX6MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(SLIMTX7, MADERA_SLIMTX7MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS(SLIMTX8, MADERA_SLIMTX8MIX_INPUT_1_SOURCE);

MADERA_MUX_ENUMS(SPD1TX1, MADERA_SPDIF1TX1MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(SPD1TX2, MADERA_SPDIF1TX2MIX_INPUT_1_SOURCE);

MADERA_MUX_ENUMS(ASRC1IN1L, MADERA_ASRC1_1LMIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(ASRC1IN1R, MADERA_ASRC1_1RMIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(ASRC1IN2L, MADERA_ASRC1_2LMIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(ASRC1IN2R, MADERA_ASRC1_2RMIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(ASRC2IN1L, MADERA_ASRC2_1LMIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(ASRC2IN1R, MADERA_ASRC2_1RMIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(ASRC2IN2L, MADERA_ASRC2_2LMIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(ASRC2IN2R, MADERA_ASRC2_2RMIX_INPUT_1_SOURCE);

MADERA_MUX_ENUMS(ISRC1INT1, MADERA_ISRC1INT1MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(ISRC1INT2, MADERA_ISRC1INT2MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(ISRC1INT3, MADERA_ISRC1INT3MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(ISRC1INT4, MADERA_ISRC1INT4MIX_INPUT_1_SOURCE);

MADERA_MUX_ENUMS(ISRC1DEC1, MADERA_ISRC1DEC1MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(ISRC1DEC2, MADERA_ISRC1DEC2MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(ISRC1DEC3, MADERA_ISRC1DEC3MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(ISRC1DEC4, MADERA_ISRC1DEC4MIX_INPUT_1_SOURCE);

MADERA_MUX_ENUMS(ISRC2INT1, MADERA_ISRC2INT1MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(ISRC2INT2, MADERA_ISRC2INT2MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(ISRC2INT3, MADERA_ISRC2INT3MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(ISRC2INT4, MADERA_ISRC2INT4MIX_INPUT_1_SOURCE);

MADERA_MUX_ENUMS(ISRC2DEC1, MADERA_ISRC2DEC1MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(ISRC2DEC2, MADERA_ISRC2DEC2MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(ISRC2DEC3, MADERA_ISRC2DEC3MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(ISRC2DEC4, MADERA_ISRC2DEC4MIX_INPUT_1_SOURCE);

MADERA_MUX_ENUMS(ISRC3INT1, MADERA_ISRC3INT1MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(ISRC3INT2, MADERA_ISRC3INT2MIX_INPUT_1_SOURCE);

MADERA_MUX_ENUMS(ISRC3DEC1, MADERA_ISRC3DEC1MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(ISRC3DEC2, MADERA_ISRC3DEC2MIX_INPUT_1_SOURCE);

MADERA_MUX_ENUMS(ISRC4INT1, MADERA_ISRC4INT1MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(ISRC4INT2, MADERA_ISRC4INT2MIX_INPUT_1_SOURCE);

MADERA_MUX_ENUMS(ISRC4DEC1, MADERA_ISRC4DEC1MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(ISRC4DEC2, MADERA_ISRC4DEC2MIX_INPUT_1_SOURCE);

MADERA_MUX_ENUMS(DFC1, MADERA_DFC1MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(DFC2, MADERA_DFC2MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(DFC3, MADERA_DFC3MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(DFC4, MADERA_DFC4MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(DFC5, MADERA_DFC5MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(DFC6, MADERA_DFC6MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(DFC7, MADERA_DFC7MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS(DFC8, MADERA_DFC8MIX_INPUT_1_SOURCE);

static const char * const cs47l90_aec_loopback_texts[] = {
	"HPOUT1L", "HPOUT1R", "HPOUT2L", "HPOUT2R", "HPOUT3L", "HPOUT3R",
	"SPKDAT1L", "SPKDAT1R",
};

static const unsigned int cs47l90_aec_loopback_values[] = {
	0, 1, 2, 3, 4, 5, 8, 9,
};

static const struct soc_enum cs47l90_aec1_loopback =
	SOC_VALUE_ENUM_SINGLE(MADERA_DAC_AEC_CONTROL_1,
			      MADERA_AEC1_LOOPBACK_SRC_SHIFT, 0xf,
			      ARRAY_SIZE(cs47l90_aec_loopback_texts),
			      cs47l90_aec_loopback_texts,
			      cs47l90_aec_loopback_values);

static const struct soc_enum cs47l90_aec2_loopback =
	SOC_VALUE_ENUM_SINGLE(MADERA_DAC_AEC_CONTROL_2,
			      MADERA_AEC2_LOOPBACK_SRC_SHIFT, 0xf,
			      ARRAY_SIZE(cs47l90_aec_loopback_texts),
			      cs47l90_aec_loopback_texts,
			      cs47l90_aec_loopback_values);

static const struct snd_kcontrol_new cs47l90_aec_loopback_mux[] = {
	SOC_DAPM_ENUM("AEC1 Loopback", cs47l90_aec1_loopback),
	SOC_DAPM_ENUM("AEC2 Loopback", cs47l90_aec2_loopback),
};

static const struct snd_kcontrol_new cs47l90_anc_input_mux[] = {
	SOC_DAPM_ENUM("RXANCL Input", madera_anc_input_src[0]),
	SOC_DAPM_ENUM("RXANCL Channel", madera_anc_input_src[1]),
	SOC_DAPM_ENUM("RXANCR Input", madera_anc_input_src[2]),
	SOC_DAPM_ENUM("RXANCR Channel", madera_anc_input_src[3]),
};

static const struct snd_kcontrol_new cs47l90_anc_ng_mux =
	SOC_DAPM_ENUM("RXANC NG Source", madera_anc_ng_enum);

static const struct snd_kcontrol_new cs47l90_output_anc_src[] = {
	SOC_DAPM_ENUM("HPOUT1L ANC Source", madera_output_anc_src[0]),
	SOC_DAPM_ENUM("HPOUT1R ANC Source", madera_output_anc_src[1]),
	SOC_DAPM_ENUM("HPOUT2L ANC Source", madera_output_anc_src[2]),
	SOC_DAPM_ENUM("HPOUT2R ANC Source", madera_output_anc_src[3]),
	SOC_DAPM_ENUM("HPOUT3L ANC Source", madera_output_anc_src[4]),
	SOC_DAPM_ENUM("HPOUT3R ANC Source", madera_output_anc_src[0]),
	SOC_DAPM_ENUM("SPKDAT1L ANC Source", madera_output_anc_src[8]),
	SOC_DAPM_ENUM("SPKDAT1R ANC Source", madera_output_anc_src[9]),
};

static const struct snd_soc_dapm_widget cs47l90_dapm_widgets[] = {
SND_SOC_DAPM_SUPPLY("SYSCLK", MADERA_SYSTEM_CLOCK_1, MADERA_SYSCLK_ENA_SHIFT,
		    0, madera_sysclk_ev,
		    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU |
		    SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD),
SND_SOC_DAPM_SUPPLY("ASYNCCLK", MADERA_ASYNC_CLOCK_1,
		    MADERA_ASYNC_CLK_ENA_SHIFT, 0, madera_clk_ev,
		    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
SND_SOC_DAPM_SUPPLY("OPCLK", MADERA_OUTPUT_SYSTEM_CLOCK,
		    MADERA_OPCLK_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_SUPPLY("ASYNCOPCLK", MADERA_OUTPUT_ASYNC_CLOCK,
		    MADERA_OPCLK_ASYNC_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_SUPPLY("DSPCLK", MADERA_DSP_CLOCK_1, MADERA_DSP_CLK_ENA_SHIFT,
		    0, madera_clk_ev,
		    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),

SND_SOC_DAPM_REGULATOR_SUPPLY("DBVDD2", 0, 0),
SND_SOC_DAPM_REGULATOR_SUPPLY("DBVDD3", 0, 0),
SND_SOC_DAPM_REGULATOR_SUPPLY("DBVDD4", 0, 0),
SND_SOC_DAPM_REGULATOR_SUPPLY("CPVDD1", 20, 0),
SND_SOC_DAPM_REGULATOR_SUPPLY("CPVDD2", 20, 0),
SND_SOC_DAPM_REGULATOR_SUPPLY("MICVDD", 0, SND_SOC_DAPM_REGULATOR_BYPASS),

SND_SOC_DAPM_SUPPLY("MICBIAS1", MADERA_MIC_BIAS_CTRL_1,
		    MADERA_MICB1_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_SUPPLY("MICBIAS2", MADERA_MIC_BIAS_CTRL_2,
		    MADERA_MICB1_ENA_SHIFT, 0, NULL, 0),

SND_SOC_DAPM_SUPPLY("MICBIAS1A", MADERA_MIC_BIAS_CTRL_5,
		    MADERA_MICB1A_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_SUPPLY("MICBIAS1B", MADERA_MIC_BIAS_CTRL_5,
		    MADERA_MICB1B_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_SUPPLY("MICBIAS1C", MADERA_MIC_BIAS_CTRL_5,
		    MADERA_MICB1C_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_SUPPLY("MICBIAS1D", MADERA_MIC_BIAS_CTRL_5,
		    MADERA_MICB1D_ENA_SHIFT, 0, NULL, 0),

SND_SOC_DAPM_SUPPLY("MICBIAS2A", MADERA_MIC_BIAS_CTRL_6,
		    MADERA_MICB2A_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_SUPPLY("MICBIAS2B", MADERA_MIC_BIAS_CTRL_6,
		    MADERA_MICB2B_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_SUPPLY("MICBIAS2C", MADERA_MIC_BIAS_CTRL_6,
		    MADERA_MICB2C_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_SUPPLY("MICBIAS2D", MADERA_MIC_BIAS_CTRL_6,
		    MADERA_MICB2D_ENA_SHIFT, 0, NULL, 0),

SND_SOC_DAPM_SUPPLY("FXCLK", SND_SOC_NOPM,
		    MADERA_DOM_GRP_FX, 0,
		    madera_domain_clk_ev,
		    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
SND_SOC_DAPM_SUPPLY("ASRC1CLK", SND_SOC_NOPM,
		    MADERA_DOM_GRP_ASRC1, 0,
		    madera_domain_clk_ev,
		    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
SND_SOC_DAPM_SUPPLY("ASRC2CLK", SND_SOC_NOPM,
		    MADERA_DOM_GRP_ASRC2, 0,
		    madera_domain_clk_ev,
		    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
SND_SOC_DAPM_SUPPLY("ISRC1CLK", SND_SOC_NOPM,
		    MADERA_DOM_GRP_ISRC1, 0,
		    madera_domain_clk_ev,
		    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
SND_SOC_DAPM_SUPPLY("ISRC2CLK", SND_SOC_NOPM,
		    MADERA_DOM_GRP_ISRC2, 0,
		    madera_domain_clk_ev,
		    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
SND_SOC_DAPM_SUPPLY("ISRC3CLK", SND_SOC_NOPM,
		    MADERA_DOM_GRP_ISRC3, 0,
		    madera_domain_clk_ev,
		    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
SND_SOC_DAPM_SUPPLY("ISRC4CLK", SND_SOC_NOPM,
		    MADERA_DOM_GRP_ISRC4, 0,
		    madera_domain_clk_ev,
		    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
SND_SOC_DAPM_SUPPLY("OUTCLK", SND_SOC_NOPM,
		    MADERA_DOM_GRP_OUT, 0,
		    madera_domain_clk_ev,
		    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
SND_SOC_DAPM_SUPPLY("SPDCLK", SND_SOC_NOPM,
		    MADERA_DOM_GRP_SPD, 0,
		    madera_domain_clk_ev,
		    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
SND_SOC_DAPM_SUPPLY("DSP1CLK", SND_SOC_NOPM,
		    MADERA_DOM_GRP_DSP1, 0,
		    madera_domain_clk_ev,
		    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
SND_SOC_DAPM_SUPPLY("DSP2CLK", SND_SOC_NOPM,
		    MADERA_DOM_GRP_DSP2, 0,
		    madera_domain_clk_ev,
		    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
SND_SOC_DAPM_SUPPLY("DSP3CLK", SND_SOC_NOPM,
		    MADERA_DOM_GRP_DSP3, 0,
		    madera_domain_clk_ev,
		    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
SND_SOC_DAPM_SUPPLY("DSP4CLK", SND_SOC_NOPM,
		    MADERA_DOM_GRP_DSP4, 0,
		    madera_domain_clk_ev,
		    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
SND_SOC_DAPM_SUPPLY("DSP5CLK", SND_SOC_NOPM,
		    MADERA_DOM_GRP_DSP5, 0,
		    madera_domain_clk_ev,
		    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
SND_SOC_DAPM_SUPPLY("DSP6CLK", SND_SOC_NOPM,
		    MADERA_DOM_GRP_DSP6, 0,
		    madera_domain_clk_ev,
		    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
SND_SOC_DAPM_SUPPLY("DSP7CLK", SND_SOC_NOPM,
		    MADERA_DOM_GRP_DSP7, 0,
		    madera_domain_clk_ev,
		    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
SND_SOC_DAPM_SUPPLY("AIF1TXCLK", SND_SOC_NOPM,
		    MADERA_DOM_GRP_AIF1, 0,
		    madera_domain_clk_ev,
		    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
SND_SOC_DAPM_SUPPLY("AIF2TXCLK", SND_SOC_NOPM,
		    MADERA_DOM_GRP_AIF2, 0,
		    madera_domain_clk_ev,
		    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
SND_SOC_DAPM_SUPPLY("AIF3TXCLK", SND_SOC_NOPM,
		    MADERA_DOM_GRP_AIF3, 0,
		    madera_domain_clk_ev,
		    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
SND_SOC_DAPM_SUPPLY("AIF4TXCLK", SND_SOC_NOPM,
		    MADERA_DOM_GRP_AIF4, 0,
		    madera_domain_clk_ev,
		    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
SND_SOC_DAPM_SUPPLY("SLIMBUSCLK", SND_SOC_NOPM,
		    MADERA_DOM_GRP_SLIMBUS, 0,
		    madera_domain_clk_ev,
		    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
SND_SOC_DAPM_SUPPLY("PWMCLK", SND_SOC_NOPM,
		    MADERA_DOM_GRP_PWM, 0,
		    madera_domain_clk_ev,
		    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
SND_SOC_DAPM_SUPPLY("DFCCLK", SND_SOC_NOPM,
		    MADERA_DOM_GRP_DFC, 0,
		    madera_domain_clk_ev,
		    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),

SND_SOC_DAPM_SIGGEN("TONE"),
SND_SOC_DAPM_SIGGEN("NOISE"),

SND_SOC_DAPM_INPUT("IN1ALN"),
SND_SOC_DAPM_INPUT("IN1ALP"),
SND_SOC_DAPM_INPUT("IN1BLN"),
SND_SOC_DAPM_INPUT("IN1BLP"),
SND_SOC_DAPM_INPUT("IN1ARN"),
SND_SOC_DAPM_INPUT("IN1ARP"),
SND_SOC_DAPM_INPUT("IN1BRN"),
SND_SOC_DAPM_INPUT("IN1BRP"),
SND_SOC_DAPM_INPUT("IN2ALN"),
SND_SOC_DAPM_INPUT("IN2ALP"),
SND_SOC_DAPM_INPUT("IN2BLN"),
SND_SOC_DAPM_INPUT("IN2BLP"),
SND_SOC_DAPM_INPUT("IN2RN"),
SND_SOC_DAPM_INPUT("IN2RP"),
SND_SOC_DAPM_INPUT("DMICCLK3"),
SND_SOC_DAPM_INPUT("DMICDAT3"),
SND_SOC_DAPM_INPUT("DMICCLK4"),
SND_SOC_DAPM_INPUT("DMICDAT4"),
SND_SOC_DAPM_INPUT("DMICCLK5"),
SND_SOC_DAPM_INPUT("DMICDAT5"),

SND_SOC_DAPM_MUX("IN1L Analog Mux", SND_SOC_NOPM, 0, 0, &madera_inmux[0]),
SND_SOC_DAPM_MUX("IN1R Analog Mux", SND_SOC_NOPM, 0, 0, &madera_inmux[1]),
SND_SOC_DAPM_MUX("IN2L Analog Mux", SND_SOC_NOPM, 0, 0, &madera_inmux[2]),

SND_SOC_DAPM_MUX("IN1L Mode", SND_SOC_NOPM, 0, 0, &madera_inmode[0]),
SND_SOC_DAPM_MUX("IN1R Mode", SND_SOC_NOPM, 0, 0, &madera_inmode[0]),

SND_SOC_DAPM_MUX("IN2L Mode", SND_SOC_NOPM, 0, 0, &madera_inmode[1]),
SND_SOC_DAPM_MUX("IN2R Mode", SND_SOC_NOPM, 0, 0, &madera_inmode[1]),

SND_SOC_DAPM_OUTPUT("DRC1 Signal Activity"),
SND_SOC_DAPM_OUTPUT("DRC2 Signal Activity"),

SND_SOC_DAPM_OUTPUT("DSP Trigger Out"),

SND_SOC_DAPM_PGA("PWM1 Driver", MADERA_PWM_DRIVE_1, MADERA_PWM1_ENA_SHIFT,
		 0, NULL, 0),
SND_SOC_DAPM_PGA("PWM2 Driver", MADERA_PWM_DRIVE_1, MADERA_PWM2_ENA_SHIFT,
		 0, NULL, 0),

SND_SOC_DAPM_SUPPLY("RXANC NG External Clock", SND_SOC_NOPM,
		    MADERA_EXT_NG_SEL_SET_SHIFT, 0, madera_anc_ev,
		    SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
SND_SOC_DAPM_PGA("RXANCL NG External", SND_SOC_NOPM, 0, 0, NULL, 0),
SND_SOC_DAPM_PGA("RXANCR NG External", SND_SOC_NOPM, 0, 0, NULL, 0),

SND_SOC_DAPM_SUPPLY("RXANC NG Clock", SND_SOC_NOPM,
		    MADERA_CLK_NG_ENA_SET_SHIFT, 0, madera_anc_ev,
		    SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
SND_SOC_DAPM_PGA("RXANCL NG Internal", SND_SOC_NOPM, 0, 0, NULL, 0),
SND_SOC_DAPM_PGA("RXANCR NG Internal", SND_SOC_NOPM, 0, 0, NULL, 0),

SND_SOC_DAPM_MUX("RXANCL Left Input", SND_SOC_NOPM, 0, 0,
		 &cs47l90_anc_input_mux[0]),
SND_SOC_DAPM_MUX("RXANCL Right Input", SND_SOC_NOPM, 0, 0,
		 &cs47l90_anc_input_mux[0]),
SND_SOC_DAPM_MUX("RXANCL Channel", SND_SOC_NOPM, 0, 0,
		 &cs47l90_anc_input_mux[1]),
SND_SOC_DAPM_MUX("RXANCL NG Mux", SND_SOC_NOPM, 0, 0, &cs47l90_anc_ng_mux),
SND_SOC_DAPM_MUX("RXANCR Left Input", SND_SOC_NOPM, 0, 0,
		 &cs47l90_anc_input_mux[2]),
SND_SOC_DAPM_MUX("RXANCR Right Input", SND_SOC_NOPM, 0, 0,
		 &cs47l90_anc_input_mux[2]),
SND_SOC_DAPM_MUX("RXANCR Channel", SND_SOC_NOPM, 0, 0,
		 &cs47l90_anc_input_mux[3]),
SND_SOC_DAPM_MUX("RXANCR NG Mux", SND_SOC_NOPM, 0, 0, &cs47l90_anc_ng_mux),

SND_SOC_DAPM_PGA_E("RXANCL", SND_SOC_NOPM, MADERA_CLK_L_ENA_SET_SHIFT,
		   0, NULL, 0, madera_anc_ev,
		   SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
SND_SOC_DAPM_PGA_E("RXANCR", SND_SOC_NOPM, MADERA_CLK_R_ENA_SET_SHIFT,
		   0, NULL, 0, madera_anc_ev,
		   SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),

SND_SOC_DAPM_MUX("HPOUT1L ANC Source", SND_SOC_NOPM, 0, 0,
		 &cs47l90_output_anc_src[0]),
SND_SOC_DAPM_MUX("HPOUT1R ANC Source", SND_SOC_NOPM, 0, 0,
		 &cs47l90_output_anc_src[1]),
SND_SOC_DAPM_MUX("HPOUT2L ANC Source", SND_SOC_NOPM, 0, 0,
		 &cs47l90_output_anc_src[2]),
SND_SOC_DAPM_MUX("HPOUT2R ANC Source", SND_SOC_NOPM, 0, 0,
		 &cs47l90_output_anc_src[3]),
SND_SOC_DAPM_MUX("HPOUT3L ANC Source", SND_SOC_NOPM, 0, 0,
		 &cs47l90_output_anc_src[4]),
SND_SOC_DAPM_MUX("HPOUT3R ANC Source", SND_SOC_NOPM, 0, 0,
		 &cs47l90_output_anc_src[5]),
SND_SOC_DAPM_MUX("SPKDAT1L ANC Source", SND_SOC_NOPM, 0, 0,
		 &cs47l90_output_anc_src[6]),
SND_SOC_DAPM_MUX("SPKDAT1R ANC Source", SND_SOC_NOPM, 0, 0,
		 &cs47l90_output_anc_src[7]),

SND_SOC_DAPM_AIF_OUT("AIF1TX1", NULL, 0,
		     MADERA_AIF1_TX_ENABLES, MADERA_AIF1TX1_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_OUT("AIF1TX2", NULL, 1,
		     MADERA_AIF1_TX_ENABLES, MADERA_AIF1TX2_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_OUT("AIF1TX3", NULL, 2,
		     MADERA_AIF1_TX_ENABLES, MADERA_AIF1TX3_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_OUT("AIF1TX4", NULL, 3,
		     MADERA_AIF1_TX_ENABLES, MADERA_AIF1TX4_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_OUT("AIF1TX5", NULL, 4,
		     MADERA_AIF1_TX_ENABLES, MADERA_AIF1TX5_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_OUT("AIF1TX6", NULL, 5,
		     MADERA_AIF1_TX_ENABLES, MADERA_AIF1TX6_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_OUT("AIF1TX7", NULL, 6,
		     MADERA_AIF1_TX_ENABLES, MADERA_AIF1TX7_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_OUT("AIF1TX8", NULL, 7,
		     MADERA_AIF1_TX_ENABLES, MADERA_AIF1TX8_ENA_SHIFT, 0),

SND_SOC_DAPM_AIF_OUT("AIF2TX1", NULL, 0,
		     MADERA_AIF2_TX_ENABLES, MADERA_AIF2TX1_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_OUT("AIF2TX2", NULL, 1,
		     MADERA_AIF2_TX_ENABLES, MADERA_AIF2TX2_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_OUT("AIF2TX3", NULL, 2,
		     MADERA_AIF2_TX_ENABLES, MADERA_AIF2TX3_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_OUT("AIF2TX4", NULL, 3,
		     MADERA_AIF2_TX_ENABLES, MADERA_AIF2TX4_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_OUT("AIF2TX5", NULL, 4,
		     MADERA_AIF2_TX_ENABLES, MADERA_AIF2TX5_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_OUT("AIF2TX6", NULL, 5,
		     MADERA_AIF2_TX_ENABLES, MADERA_AIF2TX6_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_OUT("AIF2TX7", NULL, 6,
		     MADERA_AIF2_TX_ENABLES, MADERA_AIF2TX7_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_OUT("AIF2TX8", NULL, 7,
		     MADERA_AIF2_TX_ENABLES, MADERA_AIF2TX8_ENA_SHIFT, 0),

SND_SOC_DAPM_AIF_OUT("SLIMTX1", NULL, 0,
		     MADERA_SLIMBUS_TX_CHANNEL_ENABLE,
		     MADERA_SLIMTX1_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_OUT("SLIMTX2", NULL, 1,
		     MADERA_SLIMBUS_TX_CHANNEL_ENABLE,
		     MADERA_SLIMTX2_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_OUT("SLIMTX3", NULL, 2,
		     MADERA_SLIMBUS_TX_CHANNEL_ENABLE,
		     MADERA_SLIMTX3_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_OUT("SLIMTX4", NULL, 3,
		     MADERA_SLIMBUS_TX_CHANNEL_ENABLE,
		     MADERA_SLIMTX4_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_OUT("SLIMTX5", NULL, 4,
		     MADERA_SLIMBUS_TX_CHANNEL_ENABLE,
		     MADERA_SLIMTX5_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_OUT("SLIMTX6", NULL, 5,
		     MADERA_SLIMBUS_TX_CHANNEL_ENABLE,
		     MADERA_SLIMTX6_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_OUT("SLIMTX7", NULL, 6,
		     MADERA_SLIMBUS_TX_CHANNEL_ENABLE,
		     MADERA_SLIMTX7_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_OUT("SLIMTX8", NULL, 7,
		     MADERA_SLIMBUS_TX_CHANNEL_ENABLE,
		     MADERA_SLIMTX8_ENA_SHIFT, 0),

SND_SOC_DAPM_AIF_OUT("AIF3TX1", NULL, 0,
		     MADERA_AIF3_TX_ENABLES, MADERA_AIF3TX1_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_OUT("AIF3TX2", NULL, 1,
		     MADERA_AIF3_TX_ENABLES, MADERA_AIF3TX2_ENA_SHIFT, 0),

SND_SOC_DAPM_AIF_OUT("AIF4TX1", NULL, 0,
		     MADERA_AIF4_TX_ENABLES, MADERA_AIF4TX1_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_OUT("AIF4TX2", NULL, 1,
		     MADERA_AIF4_TX_ENABLES, MADERA_AIF4TX2_ENA_SHIFT, 0),

SND_SOC_DAPM_PGA_E("OUT1L", SND_SOC_NOPM,
		   MADERA_OUT1L_ENA_SHIFT, 0, NULL, 0, madera_hp_ev,
		   SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD |
		   SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
SND_SOC_DAPM_PGA_E("OUT1R", SND_SOC_NOPM,
		   MADERA_OUT1R_ENA_SHIFT, 0, NULL, 0, madera_hp_ev,
		   SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD |
		   SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
SND_SOC_DAPM_PGA_E("OUT2L", SND_SOC_NOPM,
		   MADERA_OUT2L_ENA_SHIFT, 0, NULL, 0, madera_hp_ev,
		   SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD |
		   SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
SND_SOC_DAPM_PGA_E("OUT2R", SND_SOC_NOPM,
		   MADERA_OUT2R_ENA_SHIFT, 0, NULL, 0, madera_hp_ev,
		   SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD |
		   SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
SND_SOC_DAPM_PGA_E("OUT3L", SND_SOC_NOPM,
		   MADERA_OUT3L_ENA_SHIFT, 0, NULL, 0, madera_hp_ev,
		   SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD |
		   SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
SND_SOC_DAPM_PGA_E("OUT3R", SND_SOC_NOPM,
		   MADERA_OUT3R_ENA_SHIFT, 0, NULL, 0, madera_hp_ev,
		   SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD |
		   SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
SND_SOC_DAPM_PGA_E("OUT5L", MADERA_OUTPUT_ENABLES_1,
		   MADERA_OUT5L_ENA_SHIFT, 0, NULL, 0, madera_out_ev,
		   SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMU),
SND_SOC_DAPM_PGA_E("OUT5R", MADERA_OUTPUT_ENABLES_1,
		   MADERA_OUT5R_ENA_SHIFT, 0, NULL, 0, madera_out_ev,
		   SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMU),

SND_SOC_DAPM_PGA("SPD1TX1", MADERA_SPD1_TX_CONTROL,
		 MADERA_SPD1_VAL1_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("SPD1TX2", MADERA_SPD1_TX_CONTROL,
		 MADERA_SPD1_VAL2_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_OUT_DRV("SPD1", MADERA_SPD1_TX_CONTROL,
		     MADERA_SPD1_ENA_SHIFT, 0, NULL, 0),

/*
 * mux_in widgets : arranged in the order of sources
 * specified in MADERA_MIXER_INPUT_ROUTES
 */

SND_SOC_DAPM_PGA("Noise Generator", MADERA_COMFORT_NOISE_GENERATOR,
		 MADERA_NOISE_GEN_ENA_SHIFT, 0, NULL, 0),

SND_SOC_DAPM_PGA("Tone Generator 1", MADERA_TONE_GENERATOR_1,
		 MADERA_TONE1_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("Tone Generator 2", MADERA_TONE_GENERATOR_1,
		 MADERA_TONE2_ENA_SHIFT, 0, NULL, 0),

SND_SOC_DAPM_SIGGEN("HAPTICS"),

SND_SOC_DAPM_MUX("AEC1 Loopback", MADERA_DAC_AEC_CONTROL_1,
		 MADERA_AEC1_LOOPBACK_ENA_SHIFT, 0,
		 &cs47l90_aec_loopback_mux[0]),
SND_SOC_DAPM_MUX("AEC2 Loopback", MADERA_DAC_AEC_CONTROL_2,
		 MADERA_AEC2_LOOPBACK_ENA_SHIFT, 0,
		 &cs47l90_aec_loopback_mux[1]),

SND_SOC_DAPM_PGA_E("IN1L", MADERA_INPUT_ENABLES, MADERA_IN1L_ENA_SHIFT,
		   0, NULL, 0, madera_in_ev,
		   SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD |
		   SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
SND_SOC_DAPM_PGA_E("IN1R", MADERA_INPUT_ENABLES, MADERA_IN1R_ENA_SHIFT,
		   0, NULL, 0, madera_in_ev,
		   SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD |
		   SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
SND_SOC_DAPM_PGA_E("IN2L", MADERA_INPUT_ENABLES, MADERA_IN2L_ENA_SHIFT,
		   0, NULL, 0, madera_in_ev,
		   SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD |
		   SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
SND_SOC_DAPM_PGA_E("IN2R", MADERA_INPUT_ENABLES, MADERA_IN2R_ENA_SHIFT,
		   0, NULL, 0, madera_in_ev,
		   SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD |
		   SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
SND_SOC_DAPM_PGA_E("IN3L", MADERA_INPUT_ENABLES, MADERA_IN3L_ENA_SHIFT,
		   0, NULL, 0, madera_in_ev,
		   SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD |
		   SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
SND_SOC_DAPM_PGA_E("IN3R", MADERA_INPUT_ENABLES, MADERA_IN3R_ENA_SHIFT,
		   0, NULL, 0, madera_in_ev,
		   SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD |
		   SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
SND_SOC_DAPM_PGA_E("IN4L", MADERA_INPUT_ENABLES, MADERA_IN4L_ENA_SHIFT,
		   0, NULL, 0, madera_in_ev,
		   SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD |
		   SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
SND_SOC_DAPM_PGA_E("IN4R", MADERA_INPUT_ENABLES, MADERA_IN4R_ENA_SHIFT,
		   0, NULL, 0, madera_in_ev,
		   SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD |
		   SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
SND_SOC_DAPM_PGA_E("IN5L", MADERA_INPUT_ENABLES, MADERA_IN5L_ENA_SHIFT,
		   0, NULL, 0, madera_in_ev,
		   SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD |
		   SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),
SND_SOC_DAPM_PGA_E("IN5R", MADERA_INPUT_ENABLES, MADERA_IN5R_ENA_SHIFT,
		   0, NULL, 0, madera_in_ev,
		   SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD |
		   SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU),

SND_SOC_DAPM_AIF_IN("AIF1RX1", NULL, 0,
		    MADERA_AIF1_RX_ENABLES, MADERA_AIF1RX1_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_IN("AIF1RX2", NULL, 1,
		    MADERA_AIF1_RX_ENABLES, MADERA_AIF1RX2_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_IN("AIF1RX3", NULL, 2,
		    MADERA_AIF1_RX_ENABLES, MADERA_AIF1RX3_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_IN("AIF1RX4", NULL, 3,
		    MADERA_AIF1_RX_ENABLES, MADERA_AIF1RX4_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_IN("AIF1RX5", NULL, 4,
		    MADERA_AIF1_RX_ENABLES, MADERA_AIF1RX5_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_IN("AIF1RX6", NULL, 5,
		    MADERA_AIF1_RX_ENABLES, MADERA_AIF1RX6_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_IN("AIF1RX7", NULL, 6,
		    MADERA_AIF1_RX_ENABLES, MADERA_AIF1RX7_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_IN("AIF1RX8", NULL, 7,
		    MADERA_AIF1_RX_ENABLES, MADERA_AIF1RX8_ENA_SHIFT, 0),

SND_SOC_DAPM_AIF_IN("AIF2RX1", NULL, 0,
		    MADERA_AIF2_RX_ENABLES, MADERA_AIF2RX1_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_IN("AIF2RX2", NULL, 1,
		    MADERA_AIF2_RX_ENABLES, MADERA_AIF2RX2_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_IN("AIF2RX3", NULL, 2,
		    MADERA_AIF2_RX_ENABLES, MADERA_AIF2RX3_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_IN("AIF2RX4", NULL, 3,
		    MADERA_AIF2_RX_ENABLES, MADERA_AIF2RX4_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_IN("AIF2RX5", NULL, 4,
		    MADERA_AIF2_RX_ENABLES, MADERA_AIF2RX5_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_IN("AIF2RX6", NULL, 5,
		    MADERA_AIF2_RX_ENABLES, MADERA_AIF2RX6_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_IN("AIF2RX7", NULL, 6,
		    MADERA_AIF2_RX_ENABLES, MADERA_AIF2RX7_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_IN("AIF2RX8", NULL, 7,
		    MADERA_AIF2_RX_ENABLES, MADERA_AIF2RX8_ENA_SHIFT, 0),

SND_SOC_DAPM_AIF_IN("AIF3RX1", NULL, 0,
		    MADERA_AIF3_RX_ENABLES, MADERA_AIF3RX1_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_IN("AIF3RX2", NULL, 1,
		    MADERA_AIF3_RX_ENABLES, MADERA_AIF3RX2_ENA_SHIFT, 0),

SND_SOC_DAPM_AIF_IN("AIF4RX1", NULL, 0,
		    MADERA_AIF4_RX_ENABLES, MADERA_AIF4RX1_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_IN("AIF4RX2", NULL, 1,
		    MADERA_AIF4_RX_ENABLES, MADERA_AIF4RX2_ENA_SHIFT, 0),

SND_SOC_DAPM_AIF_IN("SLIMRX1", NULL, 0, MADERA_SLIMBUS_RX_CHANNEL_ENABLE,
		    MADERA_SLIMRX1_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_IN("SLIMRX2", NULL, 1, MADERA_SLIMBUS_RX_CHANNEL_ENABLE,
		    MADERA_SLIMRX2_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_IN("SLIMRX3", NULL, 2, MADERA_SLIMBUS_RX_CHANNEL_ENABLE,
		    MADERA_SLIMRX3_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_IN("SLIMRX4", NULL, 3, MADERA_SLIMBUS_RX_CHANNEL_ENABLE,
		    MADERA_SLIMRX4_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_IN("SLIMRX5", NULL, 4, MADERA_SLIMBUS_RX_CHANNEL_ENABLE,
		    MADERA_SLIMRX5_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_IN("SLIMRX6", NULL, 5, MADERA_SLIMBUS_RX_CHANNEL_ENABLE,
		    MADERA_SLIMRX6_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_IN("SLIMRX7", NULL, 6, MADERA_SLIMBUS_RX_CHANNEL_ENABLE,
		    MADERA_SLIMRX7_ENA_SHIFT, 0),
SND_SOC_DAPM_AIF_IN("SLIMRX8", NULL, 7, MADERA_SLIMBUS_RX_CHANNEL_ENABLE,
		    MADERA_SLIMRX8_ENA_SHIFT, 0),

SND_SOC_DAPM_PGA("EQ1", MADERA_EQ1_1, MADERA_EQ1_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("EQ2", MADERA_EQ2_1, MADERA_EQ2_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("EQ3", MADERA_EQ3_1, MADERA_EQ3_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("EQ4", MADERA_EQ4_1, MADERA_EQ4_ENA_SHIFT, 0, NULL, 0),

SND_SOC_DAPM_PGA("DRC1L", MADERA_DRC1_CTRL1, MADERA_DRC1L_ENA_SHIFT, 0,
		 NULL, 0),
SND_SOC_DAPM_PGA("DRC1R", MADERA_DRC1_CTRL1, MADERA_DRC1R_ENA_SHIFT, 0,
		 NULL, 0),
SND_SOC_DAPM_PGA("DRC2L", MADERA_DRC2_CTRL1, MADERA_DRC2L_ENA_SHIFT, 0,
		 NULL, 0),
SND_SOC_DAPM_PGA("DRC2R", MADERA_DRC2_CTRL1, MADERA_DRC2R_ENA_SHIFT, 0,
		 NULL, 0),

SND_SOC_DAPM_PGA("LHPF1", MADERA_HPLPF1_1, MADERA_LHPF1_ENA_SHIFT, 0,
		 NULL, 0),
SND_SOC_DAPM_PGA("LHPF2", MADERA_HPLPF2_1, MADERA_LHPF2_ENA_SHIFT, 0,
		 NULL, 0),
SND_SOC_DAPM_PGA("LHPF3", MADERA_HPLPF3_1, MADERA_LHPF3_ENA_SHIFT, 0,
		 NULL, 0),
SND_SOC_DAPM_PGA("LHPF4", MADERA_HPLPF4_1, MADERA_LHPF4_ENA_SHIFT, 0,
		 NULL, 0),

SND_SOC_DAPM_PGA("ASRC1IN1L", MADERA_ASRC1_ENABLE,
		 MADERA_ASRC1_IN1L_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("ASRC1IN1R", MADERA_ASRC1_ENABLE,
		 MADERA_ASRC1_IN1R_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("ASRC1IN2L", MADERA_ASRC1_ENABLE,
		 MADERA_ASRC1_IN2L_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("ASRC1IN2R", MADERA_ASRC1_ENABLE,
		 MADERA_ASRC1_IN2R_ENA_SHIFT, 0, NULL, 0),

SND_SOC_DAPM_PGA("ASRC2IN1L", MADERA_ASRC2_ENABLE,
		 MADERA_ASRC2_IN1L_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("ASRC2IN1R", MADERA_ASRC2_ENABLE,
		 MADERA_ASRC2_IN1R_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("ASRC2IN2L", MADERA_ASRC2_ENABLE,
		 MADERA_ASRC2_IN2L_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("ASRC2IN2R", MADERA_ASRC2_ENABLE,
		 MADERA_ASRC2_IN2R_ENA_SHIFT, 0, NULL, 0),

SND_SOC_DAPM_PGA("ISRC1DEC1", MADERA_ISRC_1_CTRL_3,
		 MADERA_ISRC1_DEC1_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("ISRC1DEC2", MADERA_ISRC_1_CTRL_3,
		 MADERA_ISRC1_DEC2_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("ISRC1DEC3", MADERA_ISRC_1_CTRL_3,
		 MADERA_ISRC1_DEC3_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("ISRC1DEC4", MADERA_ISRC_1_CTRL_3,
		 MADERA_ISRC1_DEC4_ENA_SHIFT, 0, NULL, 0),

SND_SOC_DAPM_PGA("ISRC1INT1", MADERA_ISRC_1_CTRL_3,
		 MADERA_ISRC1_INT1_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("ISRC1INT2", MADERA_ISRC_1_CTRL_3,
		 MADERA_ISRC1_INT2_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("ISRC1INT3", MADERA_ISRC_1_CTRL_3,
		 MADERA_ISRC1_INT3_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("ISRC1INT4", MADERA_ISRC_1_CTRL_3,
		 MADERA_ISRC1_INT4_ENA_SHIFT, 0, NULL, 0),

SND_SOC_DAPM_PGA("ISRC2DEC1", MADERA_ISRC_2_CTRL_3,
		 MADERA_ISRC2_DEC1_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("ISRC2DEC2", MADERA_ISRC_2_CTRL_3,
		 MADERA_ISRC2_DEC2_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("ISRC2DEC3", MADERA_ISRC_2_CTRL_3,
		 MADERA_ISRC2_DEC3_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("ISRC2DEC4", MADERA_ISRC_2_CTRL_3,
		 MADERA_ISRC2_DEC4_ENA_SHIFT, 0, NULL, 0),

SND_SOC_DAPM_PGA("ISRC2INT1", MADERA_ISRC_2_CTRL_3,
		 MADERA_ISRC2_INT1_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("ISRC2INT2", MADERA_ISRC_2_CTRL_3,
		 MADERA_ISRC2_INT2_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("ISRC2INT3", MADERA_ISRC_2_CTRL_3,
		 MADERA_ISRC2_INT3_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("ISRC2INT4", MADERA_ISRC_2_CTRL_3,
		 MADERA_ISRC2_INT4_ENA_SHIFT, 0, NULL, 0),

SND_SOC_DAPM_PGA("ISRC3DEC1", MADERA_ISRC_3_CTRL_3,
		 MADERA_ISRC3_DEC1_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("ISRC3DEC2", MADERA_ISRC_3_CTRL_3,
		 MADERA_ISRC3_DEC2_ENA_SHIFT, 0, NULL, 0),

SND_SOC_DAPM_PGA("ISRC3INT1", MADERA_ISRC_3_CTRL_3,
		 MADERA_ISRC3_INT1_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("ISRC3INT2", MADERA_ISRC_3_CTRL_3,
		 MADERA_ISRC3_INT2_ENA_SHIFT, 0, NULL, 0),

SND_SOC_DAPM_PGA("ISRC4DEC1", MADERA_ISRC_4_CTRL_3,
		 MADERA_ISRC4_DEC1_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("ISRC4DEC2", MADERA_ISRC_4_CTRL_3,
		 MADERA_ISRC4_DEC2_ENA_SHIFT, 0, NULL, 0),

SND_SOC_DAPM_PGA("ISRC4INT1", MADERA_ISRC_4_CTRL_3,
		 MADERA_ISRC4_INT1_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("ISRC4INT2", MADERA_ISRC_4_CTRL_3,
		 MADERA_ISRC4_INT2_ENA_SHIFT, 0, NULL, 0),

WM_ADSP2("DSP1", 0, cs47l90_adsp_power_ev),
WM_ADSP2("DSP2", 1, cs47l90_adsp_power_ev),
WM_ADSP2("DSP3", 2, cs47l90_adsp_power_ev),
WM_ADSP2("DSP4", 3, cs47l90_adsp_power_ev),
WM_ADSP2("DSP5", 4, cs47l90_adsp_power_ev),
WM_ADSP2("DSP6", 5, cs47l90_adsp_power_ev),
WM_ADSP2("DSP7", 6, cs47l90_adsp_power_ev),

/* end of ordered widget list */

SND_SOC_DAPM_PGA("DFC1", MADERA_DFC1_CTRL, MADERA_DFC1_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("DFC2", MADERA_DFC2_CTRL, MADERA_DFC1_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("DFC3", MADERA_DFC3_CTRL, MADERA_DFC1_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("DFC4", MADERA_DFC4_CTRL, MADERA_DFC1_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("DFC5", MADERA_DFC5_CTRL, MADERA_DFC1_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("DFC6", MADERA_DFC6_CTRL, MADERA_DFC1_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("DFC7", MADERA_DFC7_CTRL, MADERA_DFC1_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("DFC8", MADERA_DFC8_CTRL, MADERA_DFC1_ENA_SHIFT, 0, NULL, 0),

MADERA_MIXER_WIDGETS(EQ1, "EQ1"),
MADERA_MIXER_WIDGETS(EQ2, "EQ2"),
MADERA_MIXER_WIDGETS(EQ3, "EQ3"),
MADERA_MIXER_WIDGETS(EQ4, "EQ4"),

MADERA_MIXER_WIDGETS(DRC1L, "DRC1L"),
MADERA_MIXER_WIDGETS(DRC1R, "DRC1R"),
MADERA_MIXER_WIDGETS(DRC2L, "DRC2L"),
MADERA_MIXER_WIDGETS(DRC2R, "DRC2R"),

SND_SOC_DAPM_SWITCH("DRC1 Activity Output", SND_SOC_NOPM, 0, 0,
		    &madera_drc_activity_output_mux[0]),
SND_SOC_DAPM_SWITCH("DRC2 Activity Output", SND_SOC_NOPM, 0, 0,
		    &madera_drc_activity_output_mux[1]),

MADERA_MIXER_WIDGETS(LHPF1, "LHPF1"),
MADERA_MIXER_WIDGETS(LHPF2, "LHPF2"),
MADERA_MIXER_WIDGETS(LHPF3, "LHPF3"),
MADERA_MIXER_WIDGETS(LHPF4, "LHPF4"),

MADERA_MIXER_WIDGETS(PWM1, "PWM1"),
MADERA_MIXER_WIDGETS(PWM2, "PWM2"),

MADERA_MIXER_WIDGETS(OUT1L, "HPOUT1L"),
MADERA_MIXER_WIDGETS(OUT1R, "HPOUT1R"),
MADERA_MIXER_WIDGETS(OUT2L, "HPOUT2L"),
MADERA_MIXER_WIDGETS(OUT2R, "HPOUT2R"),
MADERA_MIXER_WIDGETS(OUT3L, "HPOUT3L"),
MADERA_MIXER_WIDGETS(OUT3R, "HPOUT3R"),
MADERA_MIXER_WIDGETS(SPKDAT1L, "SPKDAT1L"),
MADERA_MIXER_WIDGETS(SPKDAT1R, "SPKDAT1R"),

MADERA_MIXER_WIDGETS(AIF1TX1, "AIF1TX1"),
MADERA_MIXER_WIDGETS(AIF1TX2, "AIF1TX2"),
MADERA_MIXER_WIDGETS(AIF1TX3, "AIF1TX3"),
MADERA_MIXER_WIDGETS(AIF1TX4, "AIF1TX4"),
MADERA_MIXER_WIDGETS(AIF1TX5, "AIF1TX5"),
MADERA_MIXER_WIDGETS(AIF1TX6, "AIF1TX6"),
MADERA_MIXER_WIDGETS(AIF1TX7, "AIF1TX7"),
MADERA_MIXER_WIDGETS(AIF1TX8, "AIF1TX8"),

MADERA_MIXER_WIDGETS(AIF2TX1, "AIF2TX1"),
MADERA_MIXER_WIDGETS(AIF2TX2, "AIF2TX2"),
MADERA_MIXER_WIDGETS(AIF2TX3, "AIF2TX3"),
MADERA_MIXER_WIDGETS(AIF2TX4, "AIF2TX4"),
MADERA_MIXER_WIDGETS(AIF2TX5, "AIF2TX5"),
MADERA_MIXER_WIDGETS(AIF2TX6, "AIF2TX6"),
MADERA_MIXER_WIDGETS(AIF2TX7, "AIF2TX7"),
MADERA_MIXER_WIDGETS(AIF2TX8, "AIF2TX8"),

MADERA_MIXER_WIDGETS(AIF3TX1, "AIF3TX1"),
MADERA_MIXER_WIDGETS(AIF3TX2, "AIF3TX2"),

MADERA_MIXER_WIDGETS(AIF4TX1, "AIF4TX1"),
MADERA_MIXER_WIDGETS(AIF4TX2, "AIF4TX2"),

MADERA_MIXER_WIDGETS(SLIMTX1, "SLIMTX1"),
MADERA_MIXER_WIDGETS(SLIMTX2, "SLIMTX2"),
MADERA_MIXER_WIDGETS(SLIMTX3, "SLIMTX3"),
MADERA_MIXER_WIDGETS(SLIMTX4, "SLIMTX4"),
MADERA_MIXER_WIDGETS(SLIMTX5, "SLIMTX5"),
MADERA_MIXER_WIDGETS(SLIMTX6, "SLIMTX6"),
MADERA_MIXER_WIDGETS(SLIMTX7, "SLIMTX7"),
MADERA_MIXER_WIDGETS(SLIMTX8, "SLIMTX8"),

MADERA_MUX_WIDGETS(SPD1TX1, "SPDIF1TX1"),
MADERA_MUX_WIDGETS(SPD1TX2, "SPDIF1TX2"),

MADERA_MUX_WIDGETS(ASRC1IN1L, "ASRC1IN1L"),
MADERA_MUX_WIDGETS(ASRC1IN1R, "ASRC1IN1R"),
MADERA_MUX_WIDGETS(ASRC1IN2L, "ASRC1IN2L"),
MADERA_MUX_WIDGETS(ASRC1IN2R, "ASRC1IN2R"),
MADERA_MUX_WIDGETS(ASRC2IN1L, "ASRC2IN1L"),
MADERA_MUX_WIDGETS(ASRC2IN1R, "ASRC2IN1R"),
MADERA_MUX_WIDGETS(ASRC2IN2L, "ASRC2IN2L"),
MADERA_MUX_WIDGETS(ASRC2IN2R, "ASRC2IN2R"),

MADERA_DSP_WIDGETS(DSP1, "DSP1"),
MADERA_DSP_WIDGETS(DSP2, "DSP2"),
MADERA_DSP_WIDGETS(DSP3, "DSP3"),
MADERA_DSP_WIDGETS(DSP4, "DSP4"),
MADERA_DSP_WIDGETS(DSP5, "DSP5"),
MADERA_DSP_WIDGETS(DSP6, "DSP6"),
MADERA_DSP_WIDGETS(DSP7, "DSP7"),

SND_SOC_DAPM_SWITCH("DSP1 Trigger Output", SND_SOC_NOPM, 0, 0,
		    &madera_dsp_trigger_output_mux[0]),
SND_SOC_DAPM_SWITCH("DSP2 Trigger Output", SND_SOC_NOPM, 0, 0,
		    &madera_dsp_trigger_output_mux[1]),
SND_SOC_DAPM_SWITCH("DSP3 Trigger Output", SND_SOC_NOPM, 0, 0,
		    &madera_dsp_trigger_output_mux[2]),
SND_SOC_DAPM_SWITCH("DSP4 Trigger Output", SND_SOC_NOPM, 0, 0,
		    &madera_dsp_trigger_output_mux[3]),
SND_SOC_DAPM_SWITCH("DSP5 Trigger Output", SND_SOC_NOPM, 0, 0,
		    &madera_dsp_trigger_output_mux[4]),
SND_SOC_DAPM_SWITCH("DSP6 Trigger Output", SND_SOC_NOPM, 0, 0,
		    &madera_dsp_trigger_output_mux[5]),
SND_SOC_DAPM_SWITCH("DSP7 Trigger Output", SND_SOC_NOPM, 0, 0,
		    &madera_dsp_trigger_output_mux[6]),

MADERA_MUX_WIDGETS(ISRC1DEC1, "ISRC1DEC1"),
MADERA_MUX_WIDGETS(ISRC1DEC2, "ISRC1DEC2"),
MADERA_MUX_WIDGETS(ISRC1DEC3, "ISRC1DEC3"),
MADERA_MUX_WIDGETS(ISRC1DEC4, "ISRC1DEC4"),

MADERA_MUX_WIDGETS(ISRC1INT1, "ISRC1INT1"),
MADERA_MUX_WIDGETS(ISRC1INT2, "ISRC1INT2"),
MADERA_MUX_WIDGETS(ISRC1INT3, "ISRC1INT3"),
MADERA_MUX_WIDGETS(ISRC1INT4, "ISRC1INT4"),

MADERA_MUX_WIDGETS(ISRC2DEC1, "ISRC2DEC1"),
MADERA_MUX_WIDGETS(ISRC2DEC2, "ISRC2DEC2"),
MADERA_MUX_WIDGETS(ISRC2DEC3, "ISRC2DEC3"),
MADERA_MUX_WIDGETS(ISRC2DEC4, "ISRC2DEC4"),

MADERA_MUX_WIDGETS(ISRC2INT1, "ISRC2INT1"),
MADERA_MUX_WIDGETS(ISRC2INT2, "ISRC2INT2"),
MADERA_MUX_WIDGETS(ISRC2INT3, "ISRC2INT3"),
MADERA_MUX_WIDGETS(ISRC2INT4, "ISRC2INT4"),

MADERA_MUX_WIDGETS(ISRC3DEC1, "ISRC3DEC1"),
MADERA_MUX_WIDGETS(ISRC3DEC2, "ISRC3DEC2"),

MADERA_MUX_WIDGETS(ISRC3INT1, "ISRC3INT1"),
MADERA_MUX_WIDGETS(ISRC3INT2, "ISRC3INT2"),

MADERA_MUX_WIDGETS(ISRC4DEC1, "ISRC4DEC1"),
MADERA_MUX_WIDGETS(ISRC4DEC2, "ISRC4DEC2"),

MADERA_MUX_WIDGETS(ISRC4INT1, "ISRC4INT1"),
MADERA_MUX_WIDGETS(ISRC4INT2, "ISRC4INT2"),

MADERA_MUX_WIDGETS(DFC1, "DFC1"),
MADERA_MUX_WIDGETS(DFC2, "DFC2"),
MADERA_MUX_WIDGETS(DFC3, "DFC3"),
MADERA_MUX_WIDGETS(DFC4, "DFC4"),
MADERA_MUX_WIDGETS(DFC5, "DFC5"),
MADERA_MUX_WIDGETS(DFC6, "DFC6"),
MADERA_MUX_WIDGETS(DFC7, "DFC7"),
MADERA_MUX_WIDGETS(DFC8, "DFC8"),

SND_SOC_DAPM_OUTPUT("HPOUT1L"),
SND_SOC_DAPM_OUTPUT("HPOUT1R"),
SND_SOC_DAPM_OUTPUT("HPOUT2L"),
SND_SOC_DAPM_OUTPUT("HPOUT2R"),
SND_SOC_DAPM_OUTPUT("HPOUT3L"),
SND_SOC_DAPM_OUTPUT("HPOUT3R"),
SND_SOC_DAPM_OUTPUT("SPKDAT1L"),
SND_SOC_DAPM_OUTPUT("SPKDAT1R"),
SND_SOC_DAPM_OUTPUT("SPDIF1"),

SND_SOC_DAPM_OUTPUT("MICSUPP"),
};

#define MADERA_MIXER_INPUT_ROUTES(name)	\
	{ name, "Noise Generator", "Noise Generator" }, \
	{ name, "Tone Generator 1", "Tone Generator 1" }, \
	{ name, "Tone Generator 2", "Tone Generator 2" }, \
	{ name, "Haptics", "HAPTICS" }, \
	{ name, "AEC1", "AEC1 Loopback" }, \
	{ name, "AEC2", "AEC2 Loopback" }, \
	{ name, "IN1L", "IN1L" }, \
	{ name, "IN1R", "IN1R" }, \
	{ name, "IN2L", "IN2L" }, \
	{ name, "IN2R", "IN2R" }, \
	{ name, "IN3L", "IN3L" }, \
	{ name, "IN3R", "IN3R" }, \
	{ name, "IN4L", "IN4L" }, \
	{ name, "IN4R", "IN4R" }, \
	{ name, "IN5L", "IN5L" }, \
	{ name, "IN5R", "IN5R" }, \
	{ name, "AIF1RX1", "AIF1RX1" }, \
	{ name, "AIF1RX2", "AIF1RX2" }, \
	{ name, "AIF1RX3", "AIF1RX3" }, \
	{ name, "AIF1RX4", "AIF1RX4" }, \
	{ name, "AIF1RX5", "AIF1RX5" }, \
	{ name, "AIF1RX6", "AIF1RX6" }, \
	{ name, "AIF1RX7", "AIF1RX7" }, \
	{ name, "AIF1RX8", "AIF1RX8" }, \
	{ name, "AIF2RX1", "AIF2RX1" }, \
	{ name, "AIF2RX2", "AIF2RX2" }, \
	{ name, "AIF2RX3", "AIF2RX3" }, \
	{ name, "AIF2RX4", "AIF2RX4" }, \
	{ name, "AIF2RX5", "AIF2RX5" }, \
	{ name, "AIF2RX6", "AIF2RX6" }, \
	{ name, "AIF2RX7", "AIF2RX7" }, \
	{ name, "AIF2RX8", "AIF2RX8" }, \
	{ name, "AIF3RX1", "AIF3RX1" }, \
	{ name, "AIF3RX2", "AIF3RX2" }, \
	{ name, "AIF4RX1", "AIF4RX1" }, \
	{ name, "AIF4RX2", "AIF4RX2" }, \
	{ name, "SLIMRX1", "SLIMRX1" }, \
	{ name, "SLIMRX2", "SLIMRX2" }, \
	{ name, "SLIMRX3", "SLIMRX3" }, \
	{ name, "SLIMRX4", "SLIMRX4" }, \
	{ name, "SLIMRX5", "SLIMRX5" }, \
	{ name, "SLIMRX6", "SLIMRX6" }, \
	{ name, "SLIMRX7", "SLIMRX7" }, \
	{ name, "SLIMRX8", "SLIMRX8" }, \
	{ name, "EQ1", "EQ1" }, \
	{ name, "EQ2", "EQ2" }, \
	{ name, "EQ3", "EQ3" }, \
	{ name, "EQ4", "EQ4" }, \
	{ name, "DRC1L", "DRC1L" }, \
	{ name, "DRC1R", "DRC1R" }, \
	{ name, "DRC2L", "DRC2L" }, \
	{ name, "DRC2R", "DRC2R" }, \
	{ name, "LHPF1", "LHPF1" }, \
	{ name, "LHPF2", "LHPF2" }, \
	{ name, "LHPF3", "LHPF3" }, \
	{ name, "LHPF4", "LHPF4" }, \
	{ name, "ASRC1IN1L", "ASRC1IN1L" }, \
	{ name, "ASRC1IN1R", "ASRC1IN1R" }, \
	{ name, "ASRC1IN2L", "ASRC1IN2L" }, \
	{ name, "ASRC1IN2R", "ASRC1IN2R" }, \
	{ name, "ASRC2IN1L", "ASRC2IN1L" }, \
	{ name, "ASRC2IN1R", "ASRC2IN1R" }, \
	{ name, "ASRC2IN2L", "ASRC2IN2L" }, \
	{ name, "ASRC2IN2R", "ASRC2IN2R" }, \
	{ name, "ISRC1DEC1", "ISRC1DEC1" }, \
	{ name, "ISRC1DEC2", "ISRC1DEC2" }, \
	{ name, "ISRC1DEC3", "ISRC1DEC3" }, \
	{ name, "ISRC1DEC4", "ISRC1DEC4" }, \
	{ name, "ISRC1INT1", "ISRC1INT1" }, \
	{ name, "ISRC1INT2", "ISRC1INT2" }, \
	{ name, "ISRC1INT3", "ISRC1INT3" }, \
	{ name, "ISRC1INT4", "ISRC1INT4" }, \
	{ name, "ISRC2DEC1", "ISRC2DEC1" }, \
	{ name, "ISRC2DEC2", "ISRC2DEC2" }, \
	{ name, "ISRC2DEC3", "ISRC2DEC3" }, \
	{ name, "ISRC2DEC4", "ISRC2DEC4" }, \
	{ name, "ISRC2INT1", "ISRC2INT1" }, \
	{ name, "ISRC2INT2", "ISRC2INT2" }, \
	{ name, "ISRC2INT3", "ISRC2INT3" }, \
	{ name, "ISRC2INT4", "ISRC2INT4" }, \
	{ name, "ISRC3DEC1", "ISRC3DEC1" }, \
	{ name, "ISRC3DEC2", "ISRC3DEC2" }, \
	{ name, "ISRC3INT1", "ISRC3INT1" }, \
	{ name, "ISRC3INT2", "ISRC3INT2" }, \
	{ name, "ISRC4DEC1", "ISRC4DEC1" }, \
	{ name, "ISRC4DEC2", "ISRC4DEC2" }, \
	{ name, "ISRC4INT1", "ISRC4INT1" }, \
	{ name, "ISRC4INT2", "ISRC4INT2" }, \
	{ name, "DSP1.1", "DSP1" }, \
	{ name, "DSP1.2", "DSP1" }, \
	{ name, "DSP1.3", "DSP1" }, \
	{ name, "DSP1.4", "DSP1" }, \
	{ name, "DSP1.5", "DSP1" }, \
	{ name, "DSP1.6", "DSP1" }, \
	{ name, "DSP2.1", "DSP2" }, \
	{ name, "DSP2.2", "DSP2" }, \
	{ name, "DSP2.3", "DSP2" }, \
	{ name, "DSP2.4", "DSP2" }, \
	{ name, "DSP2.5", "DSP2" }, \
	{ name, "DSP2.6", "DSP2" }, \
	{ name, "DSP3.1", "DSP3" }, \
	{ name, "DSP3.2", "DSP3" }, \
	{ name, "DSP3.3", "DSP3" }, \
	{ name, "DSP3.4", "DSP3" }, \
	{ name, "DSP3.5", "DSP3" }, \
	{ name, "DSP3.6", "DSP3" }, \
	{ name, "DSP4.1", "DSP4" }, \
	{ name, "DSP4.2", "DSP4" }, \
	{ name, "DSP4.3", "DSP4" }, \
	{ name, "DSP4.4", "DSP4" }, \
	{ name, "DSP4.5", "DSP4" }, \
	{ name, "DSP4.6", "DSP4" }, \
	{ name, "DSP5.1", "DSP5" }, \
	{ name, "DSP5.2", "DSP5" }, \
	{ name, "DSP5.3", "DSP5" }, \
	{ name, "DSP5.4", "DSP5" }, \
	{ name, "DSP5.5", "DSP5" }, \
	{ name, "DSP5.6", "DSP5" }, \
	{ name, "DSP6.1", "DSP6" }, \
	{ name, "DSP6.2", "DSP6" }, \
	{ name, "DSP6.3", "DSP6" }, \
	{ name, "DSP6.4", "DSP6" }, \
	{ name, "DSP6.5", "DSP6" }, \
	{ name, "DSP6.6", "DSP6" }, \
	{ name, "DSP7.1", "DSP7" }, \
	{ name, "DSP7.2", "DSP7" }, \
	{ name, "DSP7.3", "DSP7" }, \
	{ name, "DSP7.4", "DSP7" }, \
	{ name, "DSP7.5", "DSP7" }, \
	{ name, "DSP7.6", "DSP7" }, \
	{ name, "DFC1", "DFC1" }, \
	{ name, "DFC2", "DFC2" }, \
	{ name, "DFC3", "DFC3" }, \
	{ name, "DFC4", "DFC4" }, \
	{ name, "DFC5", "DFC5" }, \
	{ name, "DFC6", "DFC6" }, \
	{ name, "DFC7", "DFC7" }, \
	{ name, "DFC8", "DFC8" }

static const struct snd_soc_dapm_route cs47l90_dapm_routes[] = {
	/* Internal clock domains */
	{ "EQ1", NULL, "FXCLK" },
	{ "EQ2", NULL, "FXCLK" },
	{ "EQ3", NULL, "FXCLK" },
	{ "EQ4", NULL, "FXCLK" },
	{ "DRC1L", NULL, "FXCLK" },
	{ "DRC1R", NULL, "FXCLK" },
	{ "DRC2L", NULL, "FXCLK" },
	{ "DRC2R", NULL, "FXCLK" },
	{ "LHPF1", NULL, "FXCLK" },
	{ "LHPF2", NULL, "FXCLK" },
	{ "LHPF3", NULL, "FXCLK" },
	{ "LHPF4", NULL, "FXCLK" },
	{ "PWM1 Mixer", NULL, "PWMCLK" },
	{ "PWM2 Mixer", NULL, "PWMCLK" },
	{ "OUT1L", NULL, "OUTCLK" },
	{ "OUT1R", NULL, "OUTCLK" },
	{ "OUT2L", NULL, "OUTCLK" },
	{ "OUT2R", NULL, "OUTCLK" },
	{ "OUT3L", NULL, "OUTCLK" },
	{ "OUT3R", NULL, "OUTCLK" },
	{ "OUT5L", NULL, "OUTCLK" },
	{ "OUT5R", NULL, "OUTCLK" },
	{ "AIF1TX1", NULL, "AIF1TXCLK" },
	{ "AIF1TX2", NULL, "AIF1TXCLK" },
	{ "AIF1TX3", NULL, "AIF1TXCLK" },
	{ "AIF1TX4", NULL, "AIF1TXCLK" },
	{ "AIF1TX5", NULL, "AIF1TXCLK" },
	{ "AIF1TX6", NULL, "AIF1TXCLK" },
	{ "AIF1TX7", NULL, "AIF1TXCLK" },
	{ "AIF1TX8", NULL, "AIF1TXCLK" },
	{ "AIF2TX1", NULL, "AIF2TXCLK" },
	{ "AIF2TX2", NULL, "AIF2TXCLK" },
	{ "AIF2TX3", NULL, "AIF2TXCLK" },
	{ "AIF2TX4", NULL, "AIF2TXCLK" },
	{ "AIF2TX5", NULL, "AIF2TXCLK" },
	{ "AIF2TX6", NULL, "AIF2TXCLK" },
	{ "AIF2TX7", NULL, "AIF2TXCLK" },
	{ "AIF2TX8", NULL, "AIF2TXCLK" },
	{ "AIF3TX1", NULL, "AIF3TXCLK" },
	{ "AIF3TX2", NULL, "AIF3TXCLK" },
	{ "AIF4TX1", NULL, "AIF4TXCLK" },
	{ "AIF4TX2", NULL, "AIF4TXCLK" },
	{ "SLIMTX1", NULL, "SLIMBUSCLK" },
	{ "SLIMTX2", NULL, "SLIMBUSCLK" },
	{ "SLIMTX3", NULL, "SLIMBUSCLK" },
	{ "SLIMTX4", NULL, "SLIMBUSCLK" },
	{ "SLIMTX5", NULL, "SLIMBUSCLK" },
	{ "SLIMTX6", NULL, "SLIMBUSCLK" },
	{ "SLIMTX7", NULL, "SLIMBUSCLK" },
	{ "SLIMTX8", NULL, "SLIMBUSCLK" },
	{ "SPD1TX1", NULL, "SPDCLK" },
	{ "SPD1TX2", NULL, "SPDCLK" },
	{ "DSP1", NULL, "DSP1CLK" },
	{ "DSP2", NULL, "DSP2CLK" },
	{ "DSP3", NULL, "DSP3CLK" },
	{ "DSP4", NULL, "DSP4CLK" },
	{ "DSP5", NULL, "DSP5CLK" },
	{ "DSP6", NULL, "DSP6CLK" },
	{ "DSP7", NULL, "DSP7CLK" },
	{ "ISRC1DEC1", NULL, "ISRC1CLK" },
	{ "ISRC1DEC2", NULL, "ISRC1CLK" },
	{ "ISRC1DEC3", NULL, "ISRC1CLK" },
	{ "ISRC1DEC4", NULL, "ISRC1CLK" },
	{ "ISRC1INT1", NULL, "ISRC1CLK" },
	{ "ISRC1INT2", NULL, "ISRC1CLK" },
	{ "ISRC1INT3", NULL, "ISRC1CLK" },
	{ "ISRC1INT4", NULL, "ISRC1CLK" },
	{ "ISRC2DEC1", NULL, "ISRC2CLK" },
	{ "ISRC2DEC2", NULL, "ISRC2CLK" },
	{ "ISRC2DEC3", NULL, "ISRC2CLK" },
	{ "ISRC2DEC4", NULL, "ISRC2CLK" },
	{ "ISRC2INT1", NULL, "ISRC2CLK" },
	{ "ISRC2INT2", NULL, "ISRC2CLK" },
	{ "ISRC2INT3", NULL, "ISRC2CLK" },
	{ "ISRC2INT4", NULL, "ISRC2CLK" },
	{ "ISRC3DEC1", NULL, "ISRC3CLK" },
	{ "ISRC3DEC2", NULL, "ISRC3CLK" },
	{ "ISRC3INT1", NULL, "ISRC3CLK" },
	{ "ISRC3INT2", NULL, "ISRC3CLK" },
	{ "ISRC4DEC1", NULL, "ISRC4CLK" },
	{ "ISRC4DEC2", NULL, "ISRC4CLK" },
	{ "ISRC4INT1", NULL, "ISRC4CLK" },
	{ "ISRC4INT2", NULL, "ISRC4CLK" },
	{ "ASRC1IN1L", NULL, "ASRC1CLK" },
	{ "ASRC1IN1R", NULL, "ASRC1CLK" },
	{ "ASRC1IN2L", NULL, "ASRC1CLK" },
	{ "ASRC1IN2R", NULL, "ASRC1CLK" },
	{ "ASRC2IN1L", NULL, "ASRC2CLK" },
	{ "ASRC2IN1R", NULL, "ASRC2CLK" },
	{ "ASRC2IN2L", NULL, "ASRC2CLK" },
	{ "ASRC2IN2R", NULL, "ASRC2CLK" },
	{ "DFC1", NULL, "DFCCLK" },
	{ "DFC2", NULL, "DFCCLK" },
	{ "DFC3", NULL, "DFCCLK" },
	{ "DFC4", NULL, "DFCCLK" },
	{ "DFC5", NULL, "DFCCLK" },
	{ "DFC6", NULL, "DFCCLK" },
	{ "DFC7", NULL, "DFCCLK" },
	{ "DFC8", NULL, "DFCCLK" },

	{ "AIF2 Capture", NULL, "DBVDD2" },
	{ "AIF2 Playback", NULL, "DBVDD2" },

	{ "AIF3 Capture", NULL, "DBVDD3" },
	{ "AIF3 Playback", NULL, "DBVDD3" },

	{ "AIF4 Capture", NULL, "DBVDD3" },
	{ "AIF4 Playback", NULL, "DBVDD3" },

	{ "OUT1L", NULL, "CPVDD1" },
	{ "OUT1L", NULL, "CPVDD2" },
	{ "OUT1R", NULL, "CPVDD1" },
	{ "OUT1R", NULL, "CPVDD2" },
	{ "OUT2L", NULL, "CPVDD1" },
	{ "OUT2L", NULL, "CPVDD2" },
	{ "OUT2R", NULL, "CPVDD1" },
	{ "OUT2R", NULL, "CPVDD2" },
	{ "OUT3L", NULL, "CPVDD1" },
	{ "OUT3L", NULL, "CPVDD2" },
	{ "OUT3R", NULL, "CPVDD1" },
	{ "OUT3R", NULL, "CPVDD2" },

	{ "OUT1L", NULL, "SYSCLK" },
	{ "OUT1R", NULL, "SYSCLK" },
	{ "OUT2L", NULL, "SYSCLK" },
	{ "OUT2R", NULL, "SYSCLK" },
	{ "OUT3L", NULL, "SYSCLK" },
	{ "OUT3R", NULL, "SYSCLK" },
	{ "OUT5L", NULL, "SYSCLK" },
	{ "OUT5R", NULL, "SYSCLK" },

	{ "SPD1", NULL, "SYSCLK" },
	{ "SPD1", NULL, "SPD1TX1" },
	{ "SPD1", NULL, "SPD1TX2" },

	{ "IN1L", NULL, "SYSCLK" },
	{ "IN1R", NULL, "SYSCLK" },
	{ "IN2L", NULL, "SYSCLK" },
	{ "IN2R", NULL, "SYSCLK" },
	{ "IN3L", NULL, "SYSCLK" },
	{ "IN3R", NULL, "SYSCLK" },
	{ "IN4L", NULL, "SYSCLK" },
	{ "IN4R", NULL, "SYSCLK" },
	{ "IN5L", NULL, "SYSCLK" },
	{ "IN5R", NULL, "SYSCLK" },

	{ "IN3L", NULL, "DBVDD4" },
	{ "IN3R", NULL, "DBVDD4" },
	{ "IN4L", NULL, "DBVDD4" },
	{ "IN4R", NULL, "DBVDD4" },
	{ "IN5L", NULL, "DBVDD4" },
	{ "IN5R", NULL, "DBVDD4" },

	{ "ASRC1IN1L", NULL, "SYSCLK" },
	{ "ASRC1IN1R", NULL, "SYSCLK" },
	{ "ASRC1IN2L", NULL, "SYSCLK" },
	{ "ASRC1IN2R", NULL, "SYSCLK" },
	{ "ASRC2IN1L", NULL, "SYSCLK" },
	{ "ASRC2IN1R", NULL, "SYSCLK" },
	{ "ASRC2IN2L", NULL, "SYSCLK" },
	{ "ASRC2IN2R", NULL, "SYSCLK" },

	{ "ASRC1IN1L", NULL, "ASYNCCLK" },
	{ "ASRC1IN1R", NULL, "ASYNCCLK" },
	{ "ASRC1IN2L", NULL, "ASYNCCLK" },
	{ "ASRC1IN2R", NULL, "ASYNCCLK" },
	{ "ASRC2IN1L", NULL, "ASYNCCLK" },
	{ "ASRC2IN1R", NULL, "ASYNCCLK" },
	{ "ASRC2IN2L", NULL, "ASYNCCLK" },
	{ "ASRC2IN2R", NULL, "ASYNCCLK" },

	{ "MICBIAS1", NULL, "MICVDD" },
	{ "MICBIAS2", NULL, "MICVDD" },

	{ "MICBIAS1A", NULL, "MICBIAS1" },
	{ "MICBIAS1B", NULL, "MICBIAS1" },
	{ "MICBIAS1C", NULL, "MICBIAS1" },
	{ "MICBIAS1D", NULL, "MICBIAS1" },

	{ "MICBIAS2A", NULL, "MICBIAS2" },
	{ "MICBIAS2B", NULL, "MICBIAS2" },
	{ "MICBIAS2C", NULL, "MICBIAS2" },
	{ "MICBIAS2D", NULL, "MICBIAS2" },

	{ "Noise Generator", NULL, "SYSCLK" },
	{ "Tone Generator 1", NULL, "SYSCLK" },
	{ "Tone Generator 2", NULL, "SYSCLK" },

	{ "Noise Generator", NULL, "NOISE" },
	{ "Tone Generator 1", NULL, "TONE" },
	{ "Tone Generator 2", NULL, "TONE" },

	{ "AIF1 Capture", NULL, "AIF1TX1" },
	{ "AIF1 Capture", NULL, "AIF1TX2" },
	{ "AIF1 Capture", NULL, "AIF1TX3" },
	{ "AIF1 Capture", NULL, "AIF1TX4" },
	{ "AIF1 Capture", NULL, "AIF1TX5" },
	{ "AIF1 Capture", NULL, "AIF1TX6" },
	{ "AIF1 Capture", NULL, "AIF1TX7" },
	{ "AIF1 Capture", NULL, "AIF1TX8" },

	{ "AIF1RX1", NULL, "AIF1 Playback" },
	{ "AIF1RX2", NULL, "AIF1 Playback" },
	{ "AIF1RX3", NULL, "AIF1 Playback" },
	{ "AIF1RX4", NULL, "AIF1 Playback" },
	{ "AIF1RX5", NULL, "AIF1 Playback" },
	{ "AIF1RX6", NULL, "AIF1 Playback" },
	{ "AIF1RX7", NULL, "AIF1 Playback" },
	{ "AIF1RX8", NULL, "AIF1 Playback" },

	{ "AIF2 Capture", NULL, "AIF2TX1" },
	{ "AIF2 Capture", NULL, "AIF2TX2" },
	{ "AIF2 Capture", NULL, "AIF2TX3" },
	{ "AIF2 Capture", NULL, "AIF2TX4" },
	{ "AIF2 Capture", NULL, "AIF2TX5" },
	{ "AIF2 Capture", NULL, "AIF2TX6" },
	{ "AIF2 Capture", NULL, "AIF2TX7" },
	{ "AIF2 Capture", NULL, "AIF2TX8" },

	{ "AIF2RX1", NULL, "AIF2 Playback" },
	{ "AIF2RX2", NULL, "AIF2 Playback" },
	{ "AIF2RX3", NULL, "AIF2 Playback" },
	{ "AIF2RX4", NULL, "AIF2 Playback" },
	{ "AIF2RX5", NULL, "AIF2 Playback" },
	{ "AIF2RX6", NULL, "AIF2 Playback" },
	{ "AIF2RX7", NULL, "AIF2 Playback" },
	{ "AIF2RX8", NULL, "AIF2 Playback" },

	{ "AIF3 Capture", NULL, "AIF3TX1" },
	{ "AIF3 Capture", NULL, "AIF3TX2" },

	{ "AIF3RX1", NULL, "AIF3 Playback" },
	{ "AIF3RX2", NULL, "AIF3 Playback" },

	{ "AIF4 Capture", NULL, "AIF4TX1" },
	{ "AIF4 Capture", NULL, "AIF4TX2" },

	{ "AIF4RX1", NULL, "AIF4 Playback" },
	{ "AIF4RX2", NULL, "AIF4 Playback" },

	{ "Slim1 Capture", NULL, "SLIMTX1" },
	{ "Slim1 Capture", NULL, "SLIMTX2" },
	{ "Slim1 Capture", NULL, "SLIMTX3" },
	{ "Slim1 Capture", NULL, "SLIMTX4" },

	{ "SLIMRX1", NULL, "Slim1 Playback" },
	{ "SLIMRX2", NULL, "Slim1 Playback" },
	{ "SLIMRX3", NULL, "Slim1 Playback" },
	{ "SLIMRX4", NULL, "Slim1 Playback" },

	{ "Slim2 Capture", NULL, "SLIMTX5" },
	{ "Slim2 Capture", NULL, "SLIMTX6" },

	{ "SLIMRX5", NULL, "Slim2 Playback" },
	{ "SLIMRX6", NULL, "Slim2 Playback" },

	{ "Slim3 Capture", NULL, "SLIMTX7" },
	{ "Slim3 Capture", NULL, "SLIMTX8" },

	{ "SLIMRX7", NULL, "Slim3 Playback" },
	{ "SLIMRX8", NULL, "Slim3 Playback" },

	{ "AIF1 Playback", NULL, "SYSCLK" },
	{ "AIF2 Playback", NULL, "SYSCLK" },
	{ "AIF3 Playback", NULL, "SYSCLK" },
	{ "AIF4 Playback", NULL, "SYSCLK" },
	{ "Slim1 Playback", NULL, "SYSCLK" },
	{ "Slim2 Playback", NULL, "SYSCLK" },
	{ "Slim3 Playback", NULL, "SYSCLK" },

	{ "AIF1 Capture", NULL, "SYSCLK" },
	{ "AIF2 Capture", NULL, "SYSCLK" },
	{ "AIF3 Capture", NULL, "SYSCLK" },
	{ "AIF4 Capture", NULL, "SYSCLK" },
	{ "Slim1 Capture", NULL, "SYSCLK" },
	{ "Slim2 Capture", NULL, "SYSCLK" },
	{ "Slim3 Capture", NULL, "SYSCLK" },

	{ "Voice Control DSP", NULL, "DSP6" },

	{ "Audio Trace DSP", NULL, "DSP1" },

	{ "IN1L Analog Mux", "A", "IN1ALN" },
	{ "IN1L Analog Mux", "A", "IN1ALP" },
	{ "IN1L Analog Mux", "B", "IN1BLN" },
	{ "IN1L Analog Mux", "B", "IN1BLP" },
	{ "IN1R Analog Mux", "A", "IN1ARN" },
	{ "IN1R Analog Mux", "A", "IN1ARP" },
	{ "IN1R Analog Mux", "B", "IN1BRN" },
	{ "IN1R Analog Mux", "B", "IN1BRP" },

	{ "IN1L Mode", "Analog", "IN1L Analog Mux" },
	{ "IN1R Mode", "Analog", "IN1R Analog Mux" },

	{ "IN1L Mode", "Digital", "IN1ARN" },
	{ "IN1L Mode", "Digital", "IN1ARP" },
	{ "IN1R Mode", "Digital", "IN1ARN" },
	{ "IN1R Mode", "Digital", "IN1ARP" },

	{ "IN1L", NULL, "IN1L Mode" },
	{ "IN1R", NULL, "IN1R Mode" },

	{ "IN2L Analog Mux", "A", "IN2ALN" },
	{ "IN2L Analog Mux", "A", "IN2ALP" },
	{ "IN2L Analog Mux", "B", "IN2BLN" },
	{ "IN2L Analog Mux", "B", "IN2BLP" },

	{ "IN2L Mode", "Analog", "IN2L Analog Mux" },
	{ "IN2R Mode", "Analog", "IN2RN" },
	{ "IN2R Mode", "Analog", "IN2RP" },

	{ "IN2L Mode", "Digital", "IN2ALN" },
	{ "IN2L Mode", "Digital", "IN2ALP" },
	{ "IN2R Mode", "Digital", "IN2ALN" },
	{ "IN2R Mode", "Digital", "IN2ALP" },

	{ "IN2L", NULL, "IN2L Mode" },
	{ "IN2R", NULL, "IN2R Mode" },

	{ "IN3L", NULL, "DMICCLK3" },
	{ "IN3L", NULL, "DMICDAT3" },
	{ "IN3R", NULL, "DMICCLK3" },
	{ "IN3R", NULL, "DMICDAT3" },

	{ "IN4L", NULL, "DMICCLK4" },
	{ "IN4L", NULL, "DMICDAT4" },
	{ "IN4R", NULL, "DMICCLK4" },
	{ "IN4R", NULL, "DMICDAT4" },

	{ "IN5L", NULL, "DMICCLK5" },
	{ "IN5L", NULL, "DMICDAT5" },
	{ "IN5R", NULL, "DMICCLK5" },
	{ "IN5R", NULL, "DMICDAT5" },

	MADERA_MIXER_ROUTES("OUT1L", "HPOUT1L"),
	MADERA_MIXER_ROUTES("OUT1R", "HPOUT1R"),
	MADERA_MIXER_ROUTES("OUT2L", "HPOUT2L"),
	MADERA_MIXER_ROUTES("OUT2R", "HPOUT2R"),
	MADERA_MIXER_ROUTES("OUT3L", "HPOUT3L"),
	MADERA_MIXER_ROUTES("OUT3R", "HPOUT3R"),

	MADERA_MIXER_ROUTES("OUT5L", "SPKDAT1L"),
	MADERA_MIXER_ROUTES("OUT5R", "SPKDAT1R"),

	MADERA_MIXER_ROUTES("PWM1 Driver", "PWM1"),
	MADERA_MIXER_ROUTES("PWM2 Driver", "PWM2"),

	MADERA_MIXER_ROUTES("AIF1TX1", "AIF1TX1"),
	MADERA_MIXER_ROUTES("AIF1TX2", "AIF1TX2"),
	MADERA_MIXER_ROUTES("AIF1TX3", "AIF1TX3"),
	MADERA_MIXER_ROUTES("AIF1TX4", "AIF1TX4"),
	MADERA_MIXER_ROUTES("AIF1TX5", "AIF1TX5"),
	MADERA_MIXER_ROUTES("AIF1TX6", "AIF1TX6"),
	MADERA_MIXER_ROUTES("AIF1TX7", "AIF1TX7"),
	MADERA_MIXER_ROUTES("AIF1TX8", "AIF1TX8"),

	MADERA_MIXER_ROUTES("AIF2TX1", "AIF2TX1"),
	MADERA_MIXER_ROUTES("AIF2TX2", "AIF2TX2"),
	MADERA_MIXER_ROUTES("AIF2TX3", "AIF2TX3"),
	MADERA_MIXER_ROUTES("AIF2TX4", "AIF2TX4"),
	MADERA_MIXER_ROUTES("AIF2TX5", "AIF2TX5"),
	MADERA_MIXER_ROUTES("AIF2TX6", "AIF2TX6"),
	MADERA_MIXER_ROUTES("AIF2TX7", "AIF2TX7"),
	MADERA_MIXER_ROUTES("AIF2TX8", "AIF2TX8"),

	MADERA_MIXER_ROUTES("AIF3TX1", "AIF3TX1"),
	MADERA_MIXER_ROUTES("AIF3TX2", "AIF3TX2"),

	MADERA_MIXER_ROUTES("AIF4TX1", "AIF4TX1"),
	MADERA_MIXER_ROUTES("AIF4TX2", "AIF4TX2"),

	MADERA_MIXER_ROUTES("SLIMTX1", "SLIMTX1"),
	MADERA_MIXER_ROUTES("SLIMTX2", "SLIMTX2"),
	MADERA_MIXER_ROUTES("SLIMTX3", "SLIMTX3"),
	MADERA_MIXER_ROUTES("SLIMTX4", "SLIMTX4"),
	MADERA_MIXER_ROUTES("SLIMTX5", "SLIMTX5"),
	MADERA_MIXER_ROUTES("SLIMTX6", "SLIMTX6"),
	MADERA_MIXER_ROUTES("SLIMTX7", "SLIMTX7"),
	MADERA_MIXER_ROUTES("SLIMTX8", "SLIMTX8"),

	MADERA_MUX_ROUTES("SPD1TX1", "SPDIF1TX1"),
	MADERA_MUX_ROUTES("SPD1TX2", "SPDIF1TX2"),

	MADERA_MIXER_ROUTES("EQ1", "EQ1"),
	MADERA_MIXER_ROUTES("EQ2", "EQ2"),
	MADERA_MIXER_ROUTES("EQ3", "EQ3"),
	MADERA_MIXER_ROUTES("EQ4", "EQ4"),

	MADERA_MIXER_ROUTES("DRC1L", "DRC1L"),
	MADERA_MIXER_ROUTES("DRC1R", "DRC1R"),
	MADERA_MIXER_ROUTES("DRC2L", "DRC2L"),
	MADERA_MIXER_ROUTES("DRC2R", "DRC2R"),

	MADERA_MIXER_ROUTES("LHPF1", "LHPF1"),
	MADERA_MIXER_ROUTES("LHPF2", "LHPF2"),
	MADERA_MIXER_ROUTES("LHPF3", "LHPF3"),
	MADERA_MIXER_ROUTES("LHPF4", "LHPF4"),

	MADERA_MUX_ROUTES("ASRC1IN1L", "ASRC1IN1L"),
	MADERA_MUX_ROUTES("ASRC1IN1R", "ASRC1IN1R"),
	MADERA_MUX_ROUTES("ASRC1IN2L", "ASRC1IN2L"),
	MADERA_MUX_ROUTES("ASRC1IN2R", "ASRC1IN2R"),
	MADERA_MUX_ROUTES("ASRC2IN1L", "ASRC2IN1L"),
	MADERA_MUX_ROUTES("ASRC2IN1R", "ASRC2IN1R"),
	MADERA_MUX_ROUTES("ASRC2IN2L", "ASRC2IN2L"),
	MADERA_MUX_ROUTES("ASRC2IN2R", "ASRC2IN2R"),

	MADERA_DSP_ROUTES("DSP1"),
	MADERA_DSP_ROUTES("DSP2"),
	MADERA_DSP_ROUTES("DSP3"),
	MADERA_DSP_ROUTES("DSP4"),
	MADERA_DSP_ROUTES("DSP5"),
	MADERA_DSP_ROUTES("DSP6"),
	MADERA_DSP_ROUTES("DSP7"),

	{ "DSP Trigger Out", NULL, "DSP1 Trigger Output" },
	{ "DSP Trigger Out", NULL, "DSP2 Trigger Output" },
	{ "DSP Trigger Out", NULL, "DSP3 Trigger Output" },
	{ "DSP Trigger Out", NULL, "DSP4 Trigger Output" },
	{ "DSP Trigger Out", NULL, "DSP5 Trigger Output" },
	{ "DSP Trigger Out", NULL, "DSP6 Trigger Output" },
	{ "DSP Trigger Out", NULL, "DSP7 Trigger Output" },

	{ "DSP1 Trigger Output", "Switch", "DSP1" },
	{ "DSP2 Trigger Output", "Switch", "DSP2" },
	{ "DSP3 Trigger Output", "Switch", "DSP3" },
	{ "DSP4 Trigger Output", "Switch", "DSP4" },
	{ "DSP5 Trigger Output", "Switch", "DSP5" },
	{ "DSP6 Trigger Output", "Switch", "DSP6" },
	{ "DSP7 Trigger Output", "Switch", "DSP7" },

	MADERA_MUX_ROUTES("ISRC1INT1", "ISRC1INT1"),
	MADERA_MUX_ROUTES("ISRC1INT2", "ISRC1INT2"),
	MADERA_MUX_ROUTES("ISRC1INT3", "ISRC1INT3"),
	MADERA_MUX_ROUTES("ISRC1INT4", "ISRC1INT4"),

	MADERA_MUX_ROUTES("ISRC1DEC1", "ISRC1DEC1"),
	MADERA_MUX_ROUTES("ISRC1DEC2", "ISRC1DEC2"),
	MADERA_MUX_ROUTES("ISRC1DEC3", "ISRC1DEC3"),
	MADERA_MUX_ROUTES("ISRC1DEC4", "ISRC1DEC4"),

	MADERA_MUX_ROUTES("ISRC2INT1", "ISRC2INT1"),
	MADERA_MUX_ROUTES("ISRC2INT2", "ISRC2INT2"),
	MADERA_MUX_ROUTES("ISRC2INT3", "ISRC2INT3"),
	MADERA_MUX_ROUTES("ISRC2INT4", "ISRC2INT4"),

	MADERA_MUX_ROUTES("ISRC2DEC1", "ISRC2DEC1"),
	MADERA_MUX_ROUTES("ISRC2DEC2", "ISRC2DEC2"),
	MADERA_MUX_ROUTES("ISRC2DEC3", "ISRC2DEC3"),
	MADERA_MUX_ROUTES("ISRC2DEC4", "ISRC2DEC4"),

	MADERA_MUX_ROUTES("ISRC3INT1", "ISRC3INT1"),
	MADERA_MUX_ROUTES("ISRC3INT2", "ISRC3INT2"),

	MADERA_MUX_ROUTES("ISRC3DEC1", "ISRC3DEC1"),
	MADERA_MUX_ROUTES("ISRC3DEC2", "ISRC3DEC2"),

	MADERA_MUX_ROUTES("ISRC4INT1", "ISRC4INT1"),
	MADERA_MUX_ROUTES("ISRC4INT2", "ISRC4INT2"),

	MADERA_MUX_ROUTES("ISRC4DEC1", "ISRC4DEC1"),
	MADERA_MUX_ROUTES("ISRC4DEC2", "ISRC4DEC2"),

	{ "AEC1 Loopback", "HPOUT1L", "OUT1L" },
	{ "AEC1 Loopback", "HPOUT1R", "OUT1R" },
	{ "AEC2 Loopback", "HPOUT1L", "OUT1L" },
	{ "AEC2 Loopback", "HPOUT1R", "OUT1R" },
	{ "HPOUT1L", NULL, "OUT1L" },
	{ "HPOUT1R", NULL, "OUT1R" },

	{ "AEC1 Loopback", "HPOUT2L", "OUT2L" },
	{ "AEC1 Loopback", "HPOUT2R", "OUT2R" },
	{ "AEC2 Loopback", "HPOUT2L", "OUT2L" },
	{ "AEC2 Loopback", "HPOUT2R", "OUT2R" },
	{ "HPOUT2L", NULL, "OUT2L" },
	{ "HPOUT2R", NULL, "OUT2R" },

	{ "AEC1 Loopback", "HPOUT3L", "OUT3L" },
	{ "AEC1 Loopback", "HPOUT3R", "OUT3R" },
	{ "AEC2 Loopback", "HPOUT3L", "OUT3L" },
	{ "AEC2 Loopback", "HPOUT3R", "OUT3R" },
	{ "HPOUT3L", NULL, "OUT3L" },
	{ "HPOUT3R", NULL, "OUT3R" },

	{ "AEC1 Loopback", "SPKDAT1L", "OUT5L" },
	{ "AEC1 Loopback", "SPKDAT1R", "OUT5R" },
	{ "AEC2 Loopback", "SPKDAT1L", "OUT5L" },
	{ "AEC2 Loopback", "SPKDAT1R", "OUT5R" },
	{ "SPKDAT1L", NULL, "OUT5L" },
	{ "SPKDAT1R", NULL, "OUT5R" },

	CS47L90_RXANC_INPUT_ROUTES("RXANCL", "RXANCL"),
	CS47L90_RXANC_INPUT_ROUTES("RXANCR", "RXANCR"),

	CS47L90_RXANC_OUTPUT_ROUTES("OUT1L", "HPOUT1L"),
	CS47L90_RXANC_OUTPUT_ROUTES("OUT1R", "HPOUT1R"),
	CS47L90_RXANC_OUTPUT_ROUTES("OUT2L", "HPOUT2L"),
	CS47L90_RXANC_OUTPUT_ROUTES("OUT2R", "HPOUT2R"),
	CS47L90_RXANC_OUTPUT_ROUTES("OUT3L", "HPOUT3L"),
	CS47L90_RXANC_OUTPUT_ROUTES("OUT3R", "HPOUT3R"),
	CS47L90_RXANC_OUTPUT_ROUTES("OUT5L", "SPKDAT1L"),
	CS47L90_RXANC_OUTPUT_ROUTES("OUT5R", "SPKDAT1R"),

	{ "SPDIF1", NULL, "SPD1" },

	{ "MICSUPP", NULL, "SYSCLK" },

	{ "DRC1 Signal Activity", NULL, "DRC1 Activity Output" },
	{ "DRC2 Signal Activity", NULL, "DRC2 Activity Output" },
	{ "DRC1 Activity Output", "Switch", "DRC1L" },
	{ "DRC1 Activity Output", "Switch", "DRC1R" },
	{ "DRC2 Activity Output", "Switch", "DRC2L" },
	{ "DRC2 Activity Output", "Switch", "DRC2R" },

	MADERA_MUX_ROUTES("DFC1", "DFC1"),
	MADERA_MUX_ROUTES("DFC2", "DFC2"),
	MADERA_MUX_ROUTES("DFC3", "DFC3"),
	MADERA_MUX_ROUTES("DFC4", "DFC4"),
	MADERA_MUX_ROUTES("DFC5", "DFC5"),
	MADERA_MUX_ROUTES("DFC6", "DFC6"),
	MADERA_MUX_ROUTES("DFC7", "DFC7"),
	MADERA_MUX_ROUTES("DFC8", "DFC8"),
};

static int cs47l90_set_fll(struct snd_soc_component *component, int fll_id,
			   int source, unsigned int fref, unsigned int fout)
{
	struct cs47l90 *cs47l90 = snd_soc_component_get_drvdata(component);

	switch (fll_id) {
	case MADERA_FLL1_REFCLK:
		return madera_set_fll_refclk(&cs47l90->fll[0], source, fref,
					     fout);
	case MADERA_FLL2_REFCLK:
		return madera_set_fll_refclk(&cs47l90->fll[1], source, fref,
					     fout);
	case MADERA_FLLAO_REFCLK:
		return madera_set_fll_ao_refclk(&cs47l90->fll[2], source, fref,
						fout);
	case MADERA_FLL1_SYNCCLK:
		return madera_set_fll_syncclk(&cs47l90->fll[0], source, fref,
					      fout);
	case MADERA_FLL2_SYNCCLK:
		return madera_set_fll_syncclk(&cs47l90->fll[1], source, fref,
					      fout);
	default:
		return -EINVAL;
	}
}

static const struct snd_soc_dai_ops cs47l90_dai_ops = {
	.compress_new = snd_soc_new_compress,
};

static struct snd_soc_dai_driver cs47l90_dai[] = {
	{
		.name = "cs47l90-aif1",
		.id = 1,
		.base = MADERA_AIF1_BCLK_CTRL,
		.playback = {
			.stream_name = "AIF1 Playback",
			.channels_min = 1,
			.channels_max = 8,
			.rates = MADERA_RATES,
			.formats = MADERA_FORMATS,
		},
		.capture = {
			.stream_name = "AIF1 Capture",
			.channels_min = 1,
			.channels_max = 8,
			.rates = MADERA_RATES,
			.formats = MADERA_FORMATS,
		 },
		.ops = &madera_dai_ops,
		.symmetric_rate = 1,
		.symmetric_sample_bits = 1,
	},
	{
		.name = "cs47l90-aif2",
		.id = 2,
		.base = MADERA_AIF2_BCLK_CTRL,
		.playback = {
			.stream_name = "AIF2 Playback",
			.channels_min = 1,
			.channels_max = 8,
			.rates = MADERA_RATES,
			.formats = MADERA_FORMATS,
		},
		.capture = {
			.stream_name = "AIF2 Capture",
			.channels_min = 1,
			.channels_max = 8,
			.rates = MADERA_RATES,
			.formats = MADERA_FORMATS,
		 },
		.ops = &madera_dai_ops,
		.symmetric_rate = 1,
		.symmetric_sample_bits = 1,
	},
	{
		.name = "cs47l90-aif3",
		.id = 3,
		.base = MADERA_AIF3_BCLK_CTRL,
		.playback = {
			.stream_name = "AIF3 Playback",
			.channels_min = 1,
			.channels_max = 2,
			.rates = MADERA_RATES,
			.formats = MADERA_FORMATS,
		},
		.capture = {
			.stream_name = "AIF3 Capture",
			.channels_min = 1,
			.channels_max = 2,
			.rates = MADERA_RATES,
			.formats = MADERA_FORMATS,
		 },
		.ops = &madera_dai_ops,
		.symmetric_rate = 1,
		.symmetric_sample_bits = 1,
	},
	{
		.name = "cs47l90-aif4",
		.id = 4,
		.base = MADERA_AIF4_BCLK_CTRL,
		.playback = {
			.stream_name = "AIF4 Playback",
			.channels_min = 1,
			.channels_max = 2,
			.rates = MADERA_RATES,
			.formats = MADERA_FORMATS,
		},
		.capture = {
			.stream_name = "AIF4 Capture",
			.channels_min = 1,
			.channels_max = 2,
			.rates = MADERA_RATES,
			.formats = MADERA_FORMATS,
		 },
		.ops = &madera_dai_ops,
		.symmetric_rate = 1,
		.symmetric_sample_bits = 1,
	},
	{
		.name = "cs47l90-slim1",
		.id = 5,
		.playback = {
			.stream_name = "Slim1 Playback",
			.channels_min = 1,
			.channels_max = 4,
			.rates = MADERA_RATES,
			.formats = MADERA_FORMATS,
		},
		.capture = {
			.stream_name = "Slim1 Capture",
			.channels_min = 1,
			.channels_max = 4,
			.rates = MADERA_RATES,
			.formats = MADERA_FORMATS,
		 },
		.ops = &madera_simple_dai_ops,
	},
	{
		.name = "cs47l90-slim2",
		.id = 6,
		.playback = {
			.stream_name = "Slim2 Playback",
			.channels_min = 1,
			.channels_max = 2,
			.rates = MADERA_RATES,
			.formats = MADERA_FORMATS,
		},
		.capture = {
			.stream_name = "Slim2 Capture",
			.channels_min = 1,
			.channels_max = 2,
			.rates = MADERA_RATES,
			.formats = MADERA_FORMATS,
		 },
		.ops = &madera_simple_dai_ops,
	},
	{
		.name = "cs47l90-slim3",
		.id = 7,
		.playback = {
			.stream_name = "Slim3 Playback",
			.channels_min = 1,
			.channels_max = 2,
			.rates = MADERA_RATES,
			.formats = MADERA_FORMATS,
		},
		.capture = {
			.stream_name = "Slim3 Capture",
			.channels_min = 1,
			.channels_max = 2,
			.rates = MADERA_RATES,
			.formats = MADERA_FORMATS,
		 },
		.ops = &madera_simple_dai_ops,
	},
	{
		.name = "cs47l90-cpu-voicectrl",
		.capture = {
			.stream_name = "Voice Control CPU",
			.channels_min = 1,
			.channels_max = 1,
			.rates = MADERA_RATES,
			.formats = MADERA_FORMATS,
		},
		.ops = &cs47l90_dai_ops,
	},
	{
		.name = "cs47l90-dsp-voicectrl",
		.capture = {
			.stream_name = "Voice Control DSP",
			.channels_min = 1,
			.channels_max = 1,
			.rates = MADERA_RATES,
			.formats = MADERA_FORMATS,
		},
	},
	{
		.name = "cs47l90-cpu-trace",
		.capture = {
			.stream_name = "Audio Trace CPU",
			.channels_min = 1,
			.channels_max = 6,
			.rates = MADERA_RATES,
			.formats = MADERA_FORMATS,
		},
		.ops = &cs47l90_dai_ops,
	},
	{
		.name = "cs47l90-dsp-trace",
		.capture = {
			.stream_name = "Audio Trace DSP",
			.channels_min = 1,
			.channels_max = 6,
			.rates = MADERA_RATES,
			.formats = MADERA_FORMATS,
		},
	},
};

static int cs47l90_open(struct snd_soc_component *component,
			struct snd_compr_stream *stream)
{
	struct snd_soc_pcm_runtime *rtd = stream->private_data;
	struct cs47l90 *cs47l90 = snd_soc_component_get_drvdata(component);
	struct madera_priv *priv = &cs47l90->core;
	struct madera *madera = priv->madera;
	int n_adsp;

	if (strcmp(snd_soc_rtd_to_codec(rtd, 0)->name, "cs47l90-dsp-voicectrl") == 0) {
		n_adsp = 5;
	} else if (strcmp(snd_soc_rtd_to_codec(rtd, 0)->name, "cs47l90-dsp-trace") == 0) {
		n_adsp = 0;
	} else {
		dev_err(madera->dev,
			"No suitable compressed stream for DAI '%s'\n",
			snd_soc_rtd_to_codec(rtd, 0)->name);
		return -EINVAL;
	}

	return wm_adsp_compr_open(&priv->adsp[n_adsp], stream);
}

static irqreturn_t cs47l90_adsp2_irq(int irq, void *data)
{
	struct cs47l90 *cs47l90 = data;
	struct madera_priv *priv = &cs47l90->core;
	struct madera *madera = priv->madera;
	struct madera_voice_trigger_info trig_info;
	int serviced = 0;
	int i, ret;

	for (i = 0; i < CS47L90_NUM_ADSP; ++i) {
		ret = wm_adsp_compr_handle_irq(&priv->adsp[i]);
		if (ret != -ENODEV)
			serviced++;
		if (ret == WM_ADSP_COMPR_VOICE_TRIGGER) {
			trig_info.core_num = i + 1;
			blocking_notifier_call_chain(&madera->notifier,
						MADERA_NOTIFY_VOICE_TRIGGER,
						&trig_info);
		}
	}

	if (!serviced) {
		dev_err(madera->dev, "Spurious compressed data IRQ\n");
		return IRQ_NONE;
	}

	return IRQ_HANDLED;
}

static int cs47l90_component_probe(struct snd_soc_component *component)
{
	struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(component);
	struct cs47l90 *cs47l90 = snd_soc_component_get_drvdata(component);
	struct madera *madera = cs47l90->core.madera;
	int ret, i;

	snd_soc_component_init_regmap(component, madera->regmap);

	scoped_guard(mutex, &madera->dapm_ptr_lock)
		madera->dapm = snd_soc_component_to_dapm(component);

	ret = madera_init_inputs(component);
	if (ret)
		return ret;

	ret = madera_init_outputs(component, NULL, CS47L90_MONO_OUTPUTS,
				  CS47L90_MONO_OUTPUTS);
	if (ret)
		return ret;

	snd_soc_dapm_disable_pin(dapm, "HAPTICS");

	ret = snd_soc_add_component_controls(component,
					     madera_adsp_rate_controls,
					     CS47L90_NUM_ADSP);
	if (ret)
		return ret;

	for (i = 0; i < CS47L90_NUM_ADSP; i++)
		wm_adsp2_component_probe(&cs47l90->core.adsp[i], component);

	return 0;
}

static void cs47l90_component_remove(struct snd_soc_component *component)
{
	struct cs47l90 *cs47l90 = snd_soc_component_get_drvdata(component);
	struct madera *madera = cs47l90->core.madera;
	int i;

	scoped_guard(mutex, &madera->dapm_ptr_lock)
		madera->dapm = NULL;

	for (i = 0; i < CS47L90_NUM_ADSP; i++)
		wm_adsp2_component_remove(&cs47l90->core.adsp[i], component);
}



static unsigned int cs47l90_digital_vu[] = {
	MADERA_DAC_DIGITAL_VOLUME_1L,
	MADERA_DAC_DIGITAL_VOLUME_1R,
	MADERA_DAC_DIGITAL_VOLUME_2L,
	MADERA_DAC_DIGITAL_VOLUME_2R,
	MADERA_DAC_DIGITAL_VOLUME_3L,
	MADERA_DAC_DIGITAL_VOLUME_3R,
	MADERA_DAC_DIGITAL_VOLUME_5L,
	MADERA_DAC_DIGITAL_VOLUME_5R,
};

static const struct snd_compress_ops cs47l90_compress_ops = {
	.open = &cs47l90_open,
	.free = &wm_adsp_compr_free,
	.set_params = &wm_adsp_compr_set_params,
	.get_caps = &wm_adsp_compr_get_caps,
	.trigger = &wm_adsp_compr_trigger,
	.pointer = &wm_adsp_compr_pointer,
	.copy = &wm_adsp_compr_copy,
};

static const struct snd_soc_component_driver soc_component_dev_cs47l90 = {
	.probe			= &cs47l90_component_probe,
	.remove			= &cs47l90_component_remove,
	.set_sysclk		= &madera_set_sysclk,
	.set_pll		= &cs47l90_set_fll,
	.name			= DRV_NAME,
	.compress_ops		= &cs47l90_compress_ops,
	.controls		= cs47l90_snd_controls,
	.num_controls		= ARRAY_SIZE(cs47l90_snd_controls),
	.dapm_widgets		= cs47l90_dapm_widgets,
	.num_dapm_widgets	= ARRAY_SIZE(cs47l90_dapm_widgets),
	.dapm_routes		= cs47l90_dapm_routes,
	.num_dapm_routes	= ARRAY_SIZE(cs47l90_dapm_routes),
	.use_pmdown_time	= 1,
	.endianness		= 1,
};

static int cs47l90_probe(struct platform_device *pdev)
{
	struct madera *madera = dev_get_drvdata(pdev->dev.parent);
	struct cs47l90 *cs47l90;
	int i, ret;

	BUILD_BUG_ON(ARRAY_SIZE(cs47l90_dai) > MADERA_MAX_DAI);

	/* quick exit if Madera irqchip driver hasn't completed probe */
	if (!madera->irq_dev) {
		dev_dbg(&pdev->dev, "irqchip driver not ready\n");
		return -EPROBE_DEFER;
	}

	cs47l90 = devm_kzalloc(&pdev->dev, sizeof(struct cs47l90),
			       GFP_KERNEL);
	if (!cs47l90)
		return -ENOMEM;

	platform_set_drvdata(pdev, cs47l90);

	cs47l90->core.madera = madera;
	cs47l90->core.dev = &pdev->dev;
	cs47l90->core.num_inputs = 10;

	ret = madera_core_init(&cs47l90->core);
	if (ret)
		return ret;

	ret = madera_request_irq(madera, MADERA_IRQ_DSP_IRQ1,
				 "ADSP2 Compressed IRQ", cs47l90_adsp2_irq,
				 cs47l90);
	if (ret != 0) {
		dev_err(&pdev->dev, "Failed to request DSP IRQ: %d\n", ret);
		goto error_core;
	}

	ret = madera_set_irq_wake(madera, MADERA_IRQ_DSP_IRQ1, 1);
	if (ret)
		dev_warn(&pdev->dev, "Failed to set DSP IRQ wake: %d\n", ret);

	for (i = 0; i < CS47L90_NUM_ADSP; i++) {
		cs47l90->core.adsp[i].part = "cs47l90";
		cs47l90->core.adsp[i].cs_dsp.num = i + 1;
		cs47l90->core.adsp[i].cs_dsp.type = WMFW_ADSP2;
		cs47l90->core.adsp[i].cs_dsp.rev = 2;
		cs47l90->core.adsp[i].cs_dsp.dev = madera->dev;
		cs47l90->core.adsp[i].cs_dsp.regmap = madera->regmap_32bit;

		cs47l90->core.adsp[i].cs_dsp.base = cs47l90_dsp_control_bases[i];
		cs47l90->core.adsp[i].cs_dsp.mem = cs47l90_dsp_regions[i];
		cs47l90->core.adsp[i].cs_dsp.num_mems =
			ARRAY_SIZE(cs47l90_dsp1_regions);

		cs47l90->core.adsp[i].cs_dsp.lock_regions = CS_ADSP2_REGION_1_9;

		ret = wm_adsp2_init(&cs47l90->core.adsp[i]);

		if (ret == 0) {
			ret = madera_init_bus_error_irq(&cs47l90->core, i,
							wm_adsp2_bus_error);
			if (ret != 0)
				wm_adsp2_remove(&cs47l90->core.adsp[i]);
		}

		if (ret) {
			for (--i; i >= 0; --i) {
				madera_free_bus_error_irq(&cs47l90->core, i);
				wm_adsp2_remove(&cs47l90->core.adsp[i]);
			}
			goto error_dsp_irq;
		}
	}

	madera_init_fll(madera, 1, MADERA_FLL1_CONTROL_1 - 1,
			&cs47l90->fll[0]);
	madera_init_fll(madera, 2, MADERA_FLL2_CONTROL_1 - 1,
			&cs47l90->fll[1]);
	madera_init_fll(madera, 4, MADERA_FLLAO_CONTROL_1 - 1,
			&cs47l90->fll[2]);

	for (i = 0; i < ARRAY_SIZE(cs47l90_dai); i++)
		madera_init_dai(&cs47l90->core, i);

	/* Latch volume update bits */
	for (i = 0; i < ARRAY_SIZE(cs47l90_digital_vu); i++)
		regmap_update_bits(madera->regmap, cs47l90_digital_vu[i],
				   CS47L90_DIG_VU, CS47L90_DIG_VU);

	pm_runtime_enable(&pdev->dev);
	pm_runtime_idle(&pdev->dev);

	ret = devm_snd_soc_register_component(&pdev->dev,
					      &soc_component_dev_cs47l90,
					      cs47l90_dai,
					      ARRAY_SIZE(cs47l90_dai));
	if (ret < 0) {
		dev_err(&pdev->dev, "Failed to register component: %d\n", ret);
		goto error_pm_runtime;
	}

	return ret;

error_pm_runtime:
	pm_runtime_disable(&pdev->dev);

	for (i = 0; i < CS47L90_NUM_ADSP; i++) {
		madera_free_bus_error_irq(&cs47l90->core, i);
		wm_adsp2_remove(&cs47l90->core.adsp[i]);
	}
error_dsp_irq:
	madera_set_irq_wake(madera, MADERA_IRQ_DSP_IRQ1, 0);
	madera_free_irq(madera, MADERA_IRQ_DSP_IRQ1, cs47l90);
error_core:
	madera_core_free(&cs47l90->core);

	return ret;
}

static void cs47l90_remove(struct platform_device *pdev)
{
	struct cs47l90 *cs47l90 = platform_get_drvdata(pdev);
	int i;

	pm_runtime_disable(&pdev->dev);

	for (i = 0; i < CS47L90_NUM_ADSP; i++) {
		madera_free_bus_error_irq(&cs47l90->core, i);
		wm_adsp2_remove(&cs47l90->core.adsp[i]);
	}

	madera_set_irq_wake(cs47l90->core.madera, MADERA_IRQ_DSP_IRQ1, 0);
	madera_free_irq(cs47l90->core.madera, MADERA_IRQ_DSP_IRQ1, cs47l90);
	madera_core_free(&cs47l90->core);
}

static struct platform_driver cs47l90_codec_driver = {
	.driver = {
		.name = "cs47l90-codec",
	},
	.probe = &cs47l90_probe,
	.remove = cs47l90_remove,
};

module_platform_driver(cs47l90_codec_driver);

MODULE_SOFTDEP("pre: madera irq-madera arizona-micsupp");
MODULE_DESCRIPTION("ASoC CS47L90 driver");
MODULE_AUTHOR("Nikesh Oswal <nikesh@opensource.cirrus.com>");
MODULE_LICENSE("GPL v2");
MODULE_ALIAS("platform:cs47l90-codec");
*/

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
