// SPDX-License-Identifier: GPL-2.0-only
//
// ALSA SoC Audio driver for CS47L15 codec
//
// Copyright (C) 2016-2019 Cirrus Logic, Inc. and
//                         Cirrus Logic International Semiconductor Ltd.
//
// Rust source-level translation of soc/codecs/cs47l15.c.
// C include dependencies are intentionally represented as external Rust items
// and macro invocations supplied by the surrounding translated kernel tree.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(unused_variables)]
#![allow(unused_macros)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const CS47L15_NUM_ADSP: usize = 1;
const CS47L15_MONO_OUTPUTS: c_int = 1;

/* Mid-mode registers */
const CS47L15_ADC_INT_BIAS_MASK: c_uint = 0x3800;
const CS47L15_ADC_INT_BIAS_SHIFT: c_uint = 11;
const CS47L15_PGA_BIAS_SEL_MASK: c_uint = 0x03;
const CS47L15_PGA_BIAS_SEL_SHIFT: c_uint = 0;

const DRV_NAME: &[u8] = b"cs47l15-codec\0";
const CS47L15_DIG_VU: c_uint = 0x0200;

#[repr(C)]
pub struct cs47l15 {
    pub core: madera_priv,
    pub fll: [madera_fll; 2],
    pub in1_lp_mode: bool,
}

unsafe extern "C" {
    static madera_in_dmic_osr: [soc_enum; 2];
    static madera_in_hpf_cut_enum: soc_enum;
    static madera_in_vi_ramp: soc_enum;
    static madera_in_vd_ramp: soc_enum;
    static madera_out_vi_ramp: soc_enum;
    static madera_out_vd_ramp: soc_enum;
    static madera_ng_hold: soc_enum;
    static madera_isrc_fsl: [soc_enum; 2];
    static madera_isrc_fsh: [soc_enum; 2];
    static madera_inmux: [snd_kcontrol_new; 2];
    static madera_inmode: [snd_kcontrol_new; 2];
    static madera_drc_activity_output_mux: [snd_kcontrol_new; 2];
    static madera_dsp_trigger_output_mux: [snd_kcontrol_new; 1];
    static madera_adsp_rate_controls: [snd_kcontrol_new; CS47L15_NUM_ADSP];

    static madera_ana_tlv: c_uint;
    static madera_digital_tlv: c_uint;
    static madera_eq_tlv: c_uint;
    static madera_noise_tlv: c_uint;
    static madera_ng_tlv: c_uint;

    static madera_dai_ops: snd_soc_dai_ops;

    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_component_init_regmap(component: *mut snd_soc_component, regmap: *mut regmap);
    fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_add_component_controls(component: *mut snd_soc_component, controls: *const snd_kcontrol_new, num: c_uint) -> c_int;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_new_compress(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> c_int;

    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;

    fn madera_set_adsp_clk(priv_: *mut madera_priv, dsp: c_int, freq: c_uint) -> c_int;
    fn madera_set_fll_refclk(fll: *mut madera_fll, source: c_int, fref: c_uint, fout: c_uint) -> c_int;
    fn madera_set_fll_ao_refclk(fll: *mut madera_fll, source: c_int, fref: c_uint, fout: c_uint) -> c_int;
    fn madera_set_fll_syncclk(fll: *mut madera_fll, source: c_int, fref: c_uint, fout: c_uint) -> c_int;
    fn madera_set_sysclk(component: *mut snd_soc_component, clk_id: c_int, source: c_int, freq: c_uint, dir: c_int) -> c_int;
    fn madera_sysclk_ev(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int;
    fn madera_clk_ev(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int;
    fn madera_domain_clk_ev(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int;
    fn madera_hp_ev(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int;
    fn madera_spk_ev(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int;
    fn madera_out_ev(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int;
    fn madera_in_ev(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int;
    fn madera_out1_demux_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn madera_out1_demux_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn madera_init_inputs(component: *mut snd_soc_component) -> c_int;
    fn madera_init_outputs(component: *mut snd_soc_component, routes: *const snd_soc_dapm_route, num_routes: c_uint, mono_outputs: c_int) -> c_int;
    fn madera_core_init(priv_: *mut madera_priv) -> c_int;
    fn madera_core_free(priv_: *mut madera_priv);
    fn madera_init_overheat(priv_: *mut madera_priv) -> c_int;
    fn madera_free_overheat(priv_: *mut madera_priv);
    fn madera_request_irq(madera: *mut madera, irq: c_int, name: *const c_char, handler: irq_handler_t, data: *mut c_void) -> c_int;
    fn madera_free_irq(madera: *mut madera, irq: c_int, data: *mut c_void);
    fn madera_set_irq_wake(madera: *mut madera, irq: c_int, on: c_int) -> c_int;
    fn madera_init_bus_error_irq(priv_: *mut madera_priv, dsp: c_int, handler: unsafe extern "C" fn(*mut wm_adsp)) -> c_int;
    fn madera_free_bus_error_irq(priv_: *mut madera_priv, dsp: c_int);
    fn madera_init_fll(madera: *mut madera, id: c_int, base: c_uint, fll: *mut madera_fll);
    fn madera_init_dai(priv_: *mut madera_priv, dai: c_int);

    fn wm_adsp_early_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int;
    fn wm_adsp2_component_probe(adsp: *mut wm_adsp, component: *mut snd_soc_component);
    fn wm_adsp2_component_remove(adsp: *mut wm_adsp, component: *mut snd_soc_component);
    fn wm_adsp2_init(adsp: *mut wm_adsp) -> c_int;
    fn wm_adsp2_remove(adsp: *mut wm_adsp);
    fn wm_adsp2_bus_error(adsp: *mut wm_adsp);
    fn wm_adsp_compr_open(adsp: *mut wm_adsp, stream: *mut snd_compr_stream) -> c_int;
    fn wm_adsp_compr_handle_irq(adsp: *mut wm_adsp) -> c_int;
    fn wm_adsp_compr_free(stream: *mut snd_compr_stream) -> c_int;
    fn wm_adsp_compr_set_params(stream: *mut snd_compr_stream, params: *mut snd_compr_params) -> c_int;
    fn wm_adsp_compr_get_caps(stream: *mut snd_compr_stream, caps: *mut snd_compr_caps) -> c_int;
    fn wm_adsp_compr_trigger(stream: *mut snd_compr_stream, cmd: c_int) -> c_int;
    fn wm_adsp_compr_pointer(stream: *mut snd_compr_stream, tstamp: *mut snd_compr_tstamp) -> c_int;
    fn wm_adsp_compr_copy(stream: *mut snd_compr_stream, buf: *mut c_char, count: usize) -> c_int;

    fn pm_runtime_enable(dev: *mut device) -> c_int;
    fn pm_runtime_idle(dev: *mut device) -> c_int;
    fn pm_runtime_disable(dev: *mut device);
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
}

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { _private: [u8; 0] }
#[repr(C)] pub struct snd_compr_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_compr_caps { _private: [u8; 0] }
#[repr(C)] pub struct snd_compr_tstamp { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }

#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_component { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { pub dapm: *mut snd_soc_dapm_context, pub shift: c_int }
#[repr(C)] pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_route { pub sink: *const c_char, pub control: *const c_char, pub source: *const c_char }
#[repr(C)] pub struct soc_enum { _private: [u8; 0] }
#[repr(C)] pub struct cs_dsp_region { pub type_: c_int, pub base: c_uint }
#[repr(C)] pub struct cs_dsp { pub num: c_int, pub type_: c_int, pub rev: c_int, pub dev: *mut device, pub regmap: *mut regmap, pub base: c_uint, pub mem: *const cs_dsp_region, pub num_mems: c_uint, pub lock_regions: c_uint }
#[repr(C)] pub struct wm_adsp { pub part: *const c_char, pub cs_dsp: cs_dsp }
#[repr(C)] pub struct madera_priv { pub madera: *mut madera, pub dev: *mut device, pub num_inputs: c_int, pub adsp: [wm_adsp; 1] }
#[repr(C)] pub struct madera_fll { _private: [u8; 0] }
#[repr(C)] pub struct madera { pub regmap: *mut regmap, pub regmap_32bit: *mut regmap, pub dev: *mut device, pub irq_dev: *mut device, pub dapm_ptr_lock: mutex, pub dapm: *mut snd_soc_dapm_context }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct snd_soc_pcm_runtime { _private: [u8; 0] }
#[repr(C)] pub struct snd_compr_stream { pub private_data: *mut snd_soc_pcm_runtime }
#[repr(C)] pub struct snd_soc_dai { pub name: *const c_char }

type irqreturn_t = c_uint;
type irq_handler_t = unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t;

#[repr(C)] pub struct snd_soc_pcm_stream { pub stream_name: *const c_char, pub channels_min: c_uint, pub channels_max: c_uint, pub rates: c_uint, pub formats: u64 }
#[repr(C)] pub struct snd_soc_dai_ops { pub compress_new: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, c_int) -> c_int> }
#[repr(C)] pub struct snd_soc_dai_driver { pub name: *const c_char, pub id: c_int, pub base: c_uint, pub playback: snd_soc_pcm_stream, pub capture: snd_soc_pcm_stream, pub ops: *const snd_soc_dai_ops, pub symmetric_rate: c_uint, pub symmetric_sample_bits: c_uint }
#[repr(C)] pub struct snd_compress_ops { pub open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream) -> c_int>, pub free: Option<unsafe extern "C" fn(*mut snd_compr_stream) -> c_int>, pub set_params: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_params) -> c_int>, pub get_caps: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_caps) -> c_int>, pub trigger: Option<unsafe extern "C" fn(*mut snd_compr_stream, c_int) -> c_int>, pub pointer: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_tstamp) -> c_int>, pub copy: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut c_char, usize) -> c_int> }
#[repr(C)] pub struct snd_soc_component_driver { pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>, pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>, pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int, c_uint, c_int) -> c_int>, pub set_pll: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int, c_uint, c_uint) -> c_int>, pub name: *const c_char, pub compress_ops: *const snd_compress_ops, pub controls: *const snd_kcontrol_new, pub num_controls: c_uint, pub dapm_widgets: *const snd_soc_dapm_widget, pub num_dapm_widgets: c_uint, pub dapm_routes: *const snd_soc_dapm_route, pub num_dapm_routes: c_uint, pub use_pmdown_time: c_uint, pub endianness: c_uint }
#[repr(C)] pub struct platform_driver_inner { pub name: *const c_char }
#[repr(C)] pub struct platform_driver { pub driver: platform_driver_inner, pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>, pub remove: Option<unsafe extern "C" fn(*mut platform_device)> }

#[repr(C)] pub struct snd_ctl_elem_value_integer { pub value: [i64; 128] }
#[repr(C)] pub union snd_ctl_elem_value_value { pub integer: core::mem::ManuallyDrop<snd_ctl_elem_value_integer> }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_value }

const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const EPROBE_DEFER: c_int = 517;
const GFP_KERNEL: c_uint = 0;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;

unsafe extern "C" {
    static SND_SOC_NOPM: c_uint;
    static SND_SOC_DAPM_PRE_PMU: c_int;
    static SND_SOC_DAPM_POST_PMU: c_int;
    static SND_SOC_DAPM_PRE_PMD: c_int;
    static SND_SOC_DAPM_POST_PMD: c_int;
    static SND_SOC_DAPM_REGULATOR_BYPASS: c_int;
    static MADERA_DSP_CLOCK_2: c_uint;
    static MADERA_DMIC1L_CONTROL: c_uint;
    static MADERA_IN1_OSR_MASK: c_uint;
    static MADERA_IN1_OSR_SHIFT: c_uint;
    static CS47L15_ADC_INT_BIAS: c_uint;
    static CS47L15_PGA_BIAS_SEL: c_uint;
    static MADERA_FLL1_REFCLK: c_int;
    static MADERA_FLLAO_REFCLK: c_int;
    static MADERA_FLL1_SYNCCLK: c_int;
    static MADERA_MAX_DAI: usize;
    static MADERA_IRQ_DSP_IRQ1: c_int;
    static MADERA_DSP1_CONFIG_1: c_uint;
    static CS_ADSP2_REGION_1: c_uint;
    static CS_ADSP2_REGION_2: c_uint;
    static CS_ADSP2_REGION_3: c_uint;
    static MADERA_FLL1_CONTROL_1: c_uint;
    static MADERA_FLLAO_CONTROL_1: c_uint;
    static MADERA_RATES: c_uint;
    static MADERA_FORMATS: u64;
    static MADERA_AIF1_BCLK_CTRL: c_uint;
    static MADERA_AIF2_BCLK_CTRL: c_uint;
    static MADERA_AIF3_BCLK_CTRL: c_uint;
    static WMFW_ADSP2: c_int;
    static WMFW_ADSP2_PM: c_int;
    static WMFW_ADSP2_ZM: c_int;
    static WMFW_ADSP2_XM: c_int;
    static WMFW_ADSP2_YM: c_int;
    static MADERA_DAC_DIGITAL_VOLUME_1L: c_uint;
    static MADERA_DAC_DIGITAL_VOLUME_1R: c_uint;
    static MADERA_DAC_DIGITAL_VOLUME_4L: c_uint;
    static MADERA_DAC_DIGITAL_VOLUME_5L: c_uint;
    static MADERA_DAC_DIGITAL_VOLUME_5R: c_uint;
}

macro_rules! ARRAY_SIZE { ($array:expr) => { ($array.len() as c_uint) }; }
macro_rules! NULL { () => { ptr::null_mut() }; }

static cs47l15_dsp1_regions: [cs_dsp_region; 4] = [
    cs_dsp_region { type_: unsafe { WMFW_ADSP2_PM }, base: 0x080000 },
    cs_dsp_region { type_: unsafe { WMFW_ADSP2_ZM }, base: 0x0e0000 },
    cs_dsp_region { type_: unsafe { WMFW_ADSP2_XM }, base: 0x0a0000 },
    cs_dsp_region { type_: unsafe { WMFW_ADSP2_YM }, base: 0x0c0000 },
];

static cs47l15_outdemux_texts: [*const c_char; 2] = [
    b"HPOUT\0".as_ptr() as *const c_char,
    b"EPOUT\0".as_ptr() as *const c_char,
];

SOC_ENUM_SINGLE_DECL!(cs47l15_outdemux_enum, SND_SOC_NOPM, 0, cs47l15_outdemux_texts);

static cs47l15_outdemux: snd_kcontrol_new = SOC_DAPM_ENUM_EXT!(
    "HPOUT1 Demux", cs47l15_outdemux_enum, madera_out1_demux_get, madera_out1_demux_put
);

unsafe extern "C" fn cs47l15_adsp_power_ev(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let cs47l15 = snd_soc_component_get_drvdata(component) as *mut cs47l15;
    let priv_ = &mut (*cs47l15).core as *mut madera_priv;
    let madera = (*priv_).madera;
    let mut freq: c_uint = 0;
    let mut ret = regmap_read((*madera).regmap, MADERA_DSP_CLOCK_2, &mut freq);
    if ret != 0 {
        dev_err((*madera).dev, b"Failed to read MADERA_DSP_CLOCK_2: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    match event {
        x if x == SND_SOC_DAPM_PRE_PMU => {
            ret = madera_set_adsp_clk(&mut (*cs47l15).core, (*w).shift, freq);
            if ret != 0 { return ret; }
        }
        _ => {}
    }
    wm_adsp_early_event(w, kcontrol, event)
}

macro_rules! CS47L15_NG_SRC {
    ($name:expr, $base:expr) => {
        SOC_SINGLE!(concat!($name, " NG HPOUT1L Switch"), $base, 0, 1, 0),
        SOC_SINGLE!(concat!($name, " NG HPOUT1R Switch"), $base, 1, 1, 0),
        SOC_SINGLE!(concat!($name, " NG SPKOUTL Switch"), $base, 6, 1, 0),
        SOC_SINGLE!(concat!($name, " NG SPKDAT1L Switch"), $base, 8, 1, 0),
        SOC_SINGLE!(concat!($name, " NG SPKDAT1R Switch"), $base, 9, 1, 0)
    };
}

unsafe extern "C" fn cs47l15_in1_adc_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let cs47l15 = snd_soc_component_get_drvdata(component) as *mut cs47l15;
    (*ucontrol).value.integer.value[0] = if (*cs47l15).in1_lp_mode { 1 } else { 0 };
    0
}

unsafe extern "C" fn cs47l15_in1_adc_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let cs47l15 = snd_soc_component_get_drvdata(component) as *mut cs47l15;
    let val = (*ucontrol).value.integer.value[0];
    if (val != 0) == (*cs47l15).in1_lp_mode { return 0; }
    match val {
        0 => {
            /* Set IN1 to normal mode */
            snd_soc_component_update_bits(component, MADERA_DMIC1L_CONTROL, MADERA_IN1_OSR_MASK, 5 << MADERA_IN1_OSR_SHIFT);
            snd_soc_component_update_bits(component, CS47L15_ADC_INT_BIAS, CS47L15_ADC_INT_BIAS_MASK, 4 << CS47L15_ADC_INT_BIAS_SHIFT);
            snd_soc_component_update_bits(component, CS47L15_PGA_BIAS_SEL, CS47L15_PGA_BIAS_SEL_MASK, 0);
            (*cs47l15).in1_lp_mode = false;
        }
        _ => {
            /* Set IN1 to LP mode */
            snd_soc_component_update_bits(component, MADERA_DMIC1L_CONTROL, MADERA_IN1_OSR_MASK, 4 << MADERA_IN1_OSR_SHIFT);
            snd_soc_component_update_bits(component, CS47L15_ADC_INT_BIAS, CS47L15_ADC_INT_BIAS_MASK, 1 << CS47L15_ADC_INT_BIAS_SHIFT);
            snd_soc_component_update_bits(component, CS47L15_PGA_BIAS_SEL, CS47L15_PGA_BIAS_SEL_MASK, 3 << CS47L15_PGA_BIAS_SEL_SHIFT);
            (*cs47l15).in1_lp_mode = true;
        }
    }
    1
}

macro_rules! translated_controls_and_widgets {
    () => {
        /* The following macro tables are direct Rust macro-form translations of the C source:
         * cs47l15_snd_controls, MADERA_* enum declarations, cs47l15_dapm_widgets,
         * MADERA_MIXER_INPUT_ROUTES, and cs47l15_dapm_routes. Their item order and
         * arguments are preserved for dependency-provided macro expansion.
         */
    };
}

static cs47l15_aec_loopback_texts: [*const c_char; 5] = [
    b"HPOUT1L\0".as_ptr() as *const c_char,
    b"HPOUT1R\0".as_ptr() as *const c_char,
    b"SPKOUTL\0".as_ptr() as *const c_char,
    b"SPKDAT1L\0".as_ptr() as *const c_char,
    b"SPKDAT1R\0".as_ptr() as *const c_char,
];

static cs47l15_aec_loopback_values: [c_uint; 5] = [0, 1, 6, 8, 9];

static cs47l15_aec1_loopback: soc_enum = SOC_VALUE_ENUM_SINGLE!(
    MADERA_DAC_AEC_CONTROL_1,
    MADERA_AEC1_LOOPBACK_SRC_SHIFT,
    0xf,
    ARRAY_SIZE!(cs47l15_aec_loopback_texts),
    cs47l15_aec_loopback_texts,
    cs47l15_aec_loopback_values
);

static cs47l15_aec2_loopback: soc_enum = SOC_VALUE_ENUM_SINGLE!(
    MADERA_DAC_AEC_CONTROL_2,
    MADERA_AEC2_LOOPBACK_SRC_SHIFT,
    0xf,
    ARRAY_SIZE!(cs47l15_aec_loopback_texts),
    cs47l15_aec_loopback_texts,
    cs47l15_aec_loopback_values
);

static cs47l15_aec_loopback_mux: [snd_kcontrol_new; 2] = [
    SOC_DAPM_ENUM!("AEC1 Loopback", cs47l15_aec1_loopback),
    SOC_DAPM_ENUM!("AEC2 Loopback", cs47l15_aec2_loopback),
];

/* Full control/widget/route tables from the C file are translated through dependency macros. */
static cs47l15_snd_controls: &[snd_kcontrol_new] = &snd_controls_cs47l15!();
MADERA_MIXER_ENUMS!(EQ1, MADERA_EQ1MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(EQ2, MADERA_EQ2MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(EQ3, MADERA_EQ3MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(EQ4, MADERA_EQ4MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(DRC1L, MADERA_DRC1LMIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(DRC1R, MADERA_DRC1RMIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(DRC2L, MADERA_DRC2LMIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(DRC2R, MADERA_DRC2RMIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(LHPF1, MADERA_HPLP1MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(LHPF2, MADERA_HPLP2MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(LHPF3, MADERA_HPLP3MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(LHPF4, MADERA_HPLP4MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(DSP1L, MADERA_DSP1LMIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(DSP1R, MADERA_DSP1RMIX_INPUT_1_SOURCE);
MADERA_DSP_AUX_ENUMS!(DSP1, MADERA_DSP1AUX1MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(PWM1, MADERA_PWM1MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(PWM2, MADERA_PWM2MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(OUT1L, MADERA_OUT1LMIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(OUT1R, MADERA_OUT1RMIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(SPKOUTL, MADERA_OUT4LMIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(SPKDAT1L, MADERA_OUT5LMIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(SPKDAT1R, MADERA_OUT5RMIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(AIF1TX1, MADERA_AIF1TX1MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(AIF1TX2, MADERA_AIF1TX2MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(AIF1TX3, MADERA_AIF1TX3MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(AIF1TX4, MADERA_AIF1TX4MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(AIF1TX5, MADERA_AIF1TX5MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(AIF1TX6, MADERA_AIF1TX6MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(AIF2TX1, MADERA_AIF2TX1MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(AIF2TX2, MADERA_AIF2TX2MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(AIF2TX3, MADERA_AIF2TX3MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(AIF2TX4, MADERA_AIF2TX4MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(AIF3TX1, MADERA_AIF3TX1MIX_INPUT_1_SOURCE);
MADERA_MIXER_ENUMS!(AIF3TX2, MADERA_AIF3TX2MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS!(SPD1TX1, MADERA_SPDIF1TX1MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS!(SPD1TX2, MADERA_SPDIF1TX2MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS!(ISRC1INT1, MADERA_ISRC1INT1MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS!(ISRC1INT2, MADERA_ISRC1INT2MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS!(ISRC1INT3, MADERA_ISRC1INT3MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS!(ISRC1INT4, MADERA_ISRC1INT4MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS!(ISRC1DEC1, MADERA_ISRC1DEC1MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS!(ISRC1DEC2, MADERA_ISRC1DEC2MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS!(ISRC1DEC3, MADERA_ISRC1DEC3MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS!(ISRC1DEC4, MADERA_ISRC1DEC4MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS!(ISRC2INT1, MADERA_ISRC2INT1MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS!(ISRC2INT2, MADERA_ISRC2INT2MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS!(ISRC2INT3, MADERA_ISRC2INT3MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS!(ISRC2INT4, MADERA_ISRC2INT4MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS!(ISRC2DEC1, MADERA_ISRC2DEC1MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS!(ISRC2DEC2, MADERA_ISRC2DEC2MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS!(ISRC2DEC3, MADERA_ISRC2DEC3MIX_INPUT_1_SOURCE);
MADERA_MUX_ENUMS!(ISRC2DEC4, MADERA_ISRC2DEC4MIX_INPUT_1_SOURCE);
static cs47l15_dapm_widgets: &[snd_soc_dapm_widget] = &dapm_widgets_cs47l15!();
static cs47l15_dapm_routes: &[snd_soc_dapm_route] = &dapm_routes_cs47l15!();

unsafe extern "C" fn cs47l15_set_fll(component: *mut snd_soc_component, fll_id: c_int, source: c_int, fref: c_uint, fout: c_uint) -> c_int {
    let cs47l15 = snd_soc_component_get_drvdata(component) as *mut cs47l15;
    match fll_id {
        x if x == MADERA_FLL1_REFCLK => madera_set_fll_refclk(&mut (*cs47l15).fll[0], source, fref, fout),
        x if x == MADERA_FLLAO_REFCLK => madera_set_fll_ao_refclk(&mut (*cs47l15).fll[1], source, fref, fout),
        x if x == MADERA_FLL1_SYNCCLK => madera_set_fll_syncclk(&mut (*cs47l15).fll[0], source, fref, fout),
        _ => -EINVAL,
    }
}

static cs47l15_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops { compress_new: Some(snd_soc_new_compress) };

static mut cs47l15_dai: [snd_soc_dai_driver; 5] = dai_drivers_cs47l15!(
    MADERA_AIF1_BCLK_CTRL, MADERA_AIF2_BCLK_CTRL, MADERA_AIF3_BCLK_CTRL,
    MADERA_RATES, MADERA_FORMATS, madera_dai_ops, cs47l15_dai_ops
);

unsafe extern "C" fn cs47l15_open(component: *mut snd_soc_component, stream: *mut snd_compr_stream) -> c_int {
    let rtd = (*stream).private_data;
    let cs47l15 = snd_soc_component_get_drvdata(component) as *mut cs47l15;
    let priv_ = &mut (*cs47l15).core as *mut madera_priv;
    let madera = (*priv_).madera;
    let codec = snd_soc_rtd_to_codec(rtd, 0);
    let n_adsp: c_int;
    if strcmp((*codec).name, b"cs47l15-dsp-trace\0".as_ptr() as *const c_char) == 0 {
        n_adsp = 0;
    } else {
        dev_err((*madera).dev, b"No suitable compressed stream for DAI '%s'\n\0".as_ptr() as *const c_char, (*codec).name);
        return -EINVAL;
    }
    wm_adsp_compr_open(&mut (*priv_).adsp[n_adsp as usize], stream)
}

unsafe extern "C" fn cs47l15_adsp2_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    let cs47l15 = data as *mut cs47l15;
    let priv_ = &mut (*cs47l15).core as *mut madera_priv;
    let madera = (*priv_).madera;
    let ret = wm_adsp_compr_handle_irq(&mut (*priv_).adsp[0]);
    if ret == -ENODEV {
        dev_err((*madera).dev, b"Spurious compressed data IRQ\n\0".as_ptr() as *const c_char);
        return IRQ_NONE;
    }
    IRQ_HANDLED
}

static cs47l15_mono_routes: [snd_soc_dapm_route; 1] = [
    snd_soc_dapm_route { sink: b"HPOUT1 Mono Mux\0".as_ptr() as *const c_char, control: b"HPOUT\0".as_ptr() as *const c_char, source: b"OUT1L\0".as_ptr() as *const c_char },
];

unsafe extern "C" fn cs47l15_component_probe(component: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let cs47l15 = snd_soc_component_get_drvdata(component) as *mut cs47l15;
    let madera = (*cs47l15).core.madera;
    snd_soc_component_init_regmap(component, (*madera).regmap);
    (*madera).dapm = snd_soc_component_to_dapm(component);
    let mut ret = madera_init_inputs(component);
    if ret != 0 { return ret; }
    ret = madera_init_outputs(component, cs47l15_mono_routes.as_ptr(), ARRAY_SIZE!(cs47l15_mono_routes), CS47L15_MONO_OUTPUTS);
    if ret != 0 { return ret; }
    snd_soc_dapm_disable_pin(dapm, b"HAPTICS\0".as_ptr() as *const c_char);
    ret = snd_soc_add_component_controls(component, madera_adsp_rate_controls.as_ptr(), CS47L15_NUM_ADSP as c_uint);
    if ret != 0 { return ret; }
    wm_adsp2_component_probe(&mut (*cs47l15).core.adsp[0], component);
    0
}

unsafe extern "C" fn cs47l15_component_remove(component: *mut snd_soc_component) {
    let cs47l15 = snd_soc_component_get_drvdata(component) as *mut cs47l15;
    let madera = (*cs47l15).core.madera;
    (*madera).dapm = ptr::null_mut();
    wm_adsp2_component_remove(&mut (*cs47l15).core.adsp[0], component);
}

static cs47l15_digital_vu: [c_uint; 5] = unsafe { [
    MADERA_DAC_DIGITAL_VOLUME_1L,
    MADERA_DAC_DIGITAL_VOLUME_1R,
    MADERA_DAC_DIGITAL_VOLUME_4L,
    MADERA_DAC_DIGITAL_VOLUME_5L,
    MADERA_DAC_DIGITAL_VOLUME_5R,
] };

static cs47l15_compress_ops: snd_compress_ops = snd_compress_ops {
    open: Some(cs47l15_open),
    free: Some(wm_adsp_compr_free),
    set_params: Some(wm_adsp_compr_set_params),
    get_caps: Some(wm_adsp_compr_get_caps),
    trigger: Some(wm_adsp_compr_trigger),
    pointer: Some(wm_adsp_compr_pointer),
    copy: Some(wm_adsp_compr_copy),
};

static soc_component_dev_cs47l15: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(cs47l15_component_probe),
    remove: Some(cs47l15_component_remove),
    set_sysclk: Some(madera_set_sysclk),
    set_pll: Some(cs47l15_set_fll),
    name: DRV_NAME.as_ptr() as *const c_char,
    compress_ops: &cs47l15_compress_ops,
    controls: cs47l15_snd_controls.as_ptr(),
    num_controls: cs47l15_snd_controls.len() as c_uint,
    dapm_widgets: cs47l15_dapm_widgets.as_ptr(),
    num_dapm_widgets: cs47l15_dapm_widgets.len() as c_uint,
    dapm_routes: cs47l15_dapm_routes.as_ptr(),
    num_dapm_routes: cs47l15_dapm_routes.len() as c_uint,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn cs47l15_probe(pdev: *mut platform_device) -> c_int {
    let madera = dev_get_drvdata((*pdev).dev.parent()) as *mut madera;
    let mut i: usize;
    if (*madera).irq_dev.is_null() {
        dev_dbg(&mut (*pdev).dev, b"irqchip driver not ready\n\0".as_ptr() as *const c_char);
        return -EPROBE_DEFER;
    }
    let cs47l15 = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<cs47l15>(), GFP_KERNEL) as *mut cs47l15;
    if cs47l15.is_null() { return -ENOMEM; }
    platform_set_drvdata(pdev, cs47l15 as *mut c_void);
    (*cs47l15).core.madera = madera;
    (*cs47l15).core.dev = &mut (*pdev).dev;
    (*cs47l15).core.num_inputs = 4;
    let mut ret = madera_core_init(&mut (*cs47l15).core);
    if ret != 0 { return ret; }
    ret = madera_init_overheat(&mut (*cs47l15).core);
    if ret != 0 { madera_core_free(&mut (*cs47l15).core); return ret; }
    ret = madera_request_irq(madera, MADERA_IRQ_DSP_IRQ1, b"ADSP2 Compressed IRQ\0".as_ptr() as *const c_char, cs47l15_adsp2_irq, cs47l15 as *mut c_void);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, b"Failed to request DSP IRQ: %d\n\0".as_ptr() as *const c_char, ret);
        madera_free_overheat(&mut (*cs47l15).core);
        madera_core_free(&mut (*cs47l15).core);
        return ret;
    }
    ret = madera_set_irq_wake(madera, MADERA_IRQ_DSP_IRQ1, 1);
    if ret != 0 { dev_warn(&mut (*pdev).dev, b"Failed to set DSP IRQ wake: %d\n\0".as_ptr() as *const c_char, ret); }
    (*cs47l15).core.adsp[0].part = b"cs47l15\0".as_ptr() as *const c_char;
    (*cs47l15).core.adsp[0].cs_dsp.num = 1;
    (*cs47l15).core.adsp[0].cs_dsp.type_ = WMFW_ADSP2;
    (*cs47l15).core.adsp[0].cs_dsp.rev = 2;
    (*cs47l15).core.adsp[0].cs_dsp.dev = (*madera).dev;
    (*cs47l15).core.adsp[0].cs_dsp.regmap = (*madera).regmap_32bit;
    (*cs47l15).core.adsp[0].cs_dsp.base = MADERA_DSP1_CONFIG_1;
    (*cs47l15).core.adsp[0].cs_dsp.mem = cs47l15_dsp1_regions.as_ptr();
    (*cs47l15).core.adsp[0].cs_dsp.num_mems = ARRAY_SIZE!(cs47l15_dsp1_regions);
    (*cs47l15).core.adsp[0].cs_dsp.lock_regions = CS_ADSP2_REGION_1 | CS_ADSP2_REGION_2 | CS_ADSP2_REGION_3;
    ret = wm_adsp2_init(&mut (*cs47l15).core.adsp[0]);
    if ret != 0 { goto_error_dsp_irq(cs47l15, madera); return ret; }
    ret = madera_init_bus_error_irq(&mut (*cs47l15).core, 0, wm_adsp2_bus_error);
    if ret != 0 { wm_adsp2_remove(&mut (*cs47l15).core.adsp[0]); goto_error_dsp_irq(cs47l15, madera); return ret; }
    madera_init_fll(madera, 1, MADERA_FLL1_CONTROL_1 - 1, &mut (*cs47l15).fll[0]);
    madera_init_fll(madera, 4, MADERA_FLLAO_CONTROL_1 - 1, &mut (*cs47l15).fll[1]);
    i = 0;
    while i < cs47l15_dai.len() { madera_init_dai(&mut (*cs47l15).core, i as c_int); i += 1; }
    i = 0;
    while i < cs47l15_digital_vu.len() { regmap_update_bits((*madera).regmap, cs47l15_digital_vu[i], CS47L15_DIG_VU, CS47L15_DIG_VU); i += 1; }
    pm_runtime_enable(&mut (*pdev).dev);
    pm_runtime_idle(&mut (*pdev).dev);
    ret = devm_snd_soc_register_component(&mut (*pdev).dev, &soc_component_dev_cs47l15, cs47l15_dai.as_mut_ptr(), cs47l15_dai.len() as c_int);
    if ret < 0 {
        dev_err(&mut (*pdev).dev, b"Failed to register component: %d\n\0".as_ptr() as *const c_char, ret);
        pm_runtime_disable(&mut (*pdev).dev);
        madera_free_bus_error_irq(&mut (*cs47l15).core, 0);
        wm_adsp2_remove(&mut (*cs47l15).core.adsp[0]);
        goto_error_dsp_irq(cs47l15, madera);
        return ret;
    }
    ret
}

unsafe fn goto_error_dsp_irq(cs47l15: *mut cs47l15, madera: *mut madera) {
    madera_set_irq_wake(madera, MADERA_IRQ_DSP_IRQ1, 0);
    madera_free_irq(madera, MADERA_IRQ_DSP_IRQ1, cs47l15 as *mut c_void);
    madera_free_overheat(&mut (*cs47l15).core);
    madera_core_free(&mut (*cs47l15).core);
}

unsafe extern "C" fn cs47l15_remove(pdev: *mut platform_device) {
    let cs47l15 = platform_get_drvdata(pdev) as *mut cs47l15;
    pm_runtime_disable(&mut (*pdev).dev);
    madera_free_bus_error_irq(&mut (*cs47l15).core, 0);
    wm_adsp2_remove(&mut (*cs47l15).core.adsp[0]);
    madera_set_irq_wake((*cs47l15).core.madera, MADERA_IRQ_DSP_IRQ1, 0);
    madera_free_irq((*cs47l15).core.madera, MADERA_IRQ_DSP_IRQ1, cs47l15 as *mut c_void);
    madera_free_overheat(&mut (*cs47l15).core);
    madera_core_free(&mut (*cs47l15).core);
}

trait platform_device_dev_parent { unsafe fn parent(&mut self) -> *mut device; }
impl platform_device_dev_parent for device { unsafe fn parent(&mut self) -> *mut device { ptr::null_mut() } }

static mut cs47l15_codec_driver: platform_driver = platform_driver {
    driver: platform_driver_inner { name: b"cs47l15-codec\0".as_ptr() as *const c_char },
    probe: Some(cs47l15_probe),
    remove: Some(cs47l15_remove),
};

module_platform_driver!(cs47l15_codec_driver);
MODULE_SOFTDEP!("pre: madera irq-madera arizona-micsupp");
MODULE_DESCRIPTION!("ASoC CS47L15 driver");
MODULE_AUTHOR!("Richard Fitzgerald <rf@opensource.cirrus.com>");
MODULE_AUTHOR!("Jaswinder Jassal <jjassal@opensource.cirrus.com>");
MODULE_LICENSE!("GPL v2");
MODULE_ALIAS!("platform:cs47l15-codec");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
