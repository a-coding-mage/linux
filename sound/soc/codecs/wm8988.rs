// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8988.c -- WM8988 ALSA SoC audio driver
 *
 * Copyright 2009 Wolfson Microelectronics plc
 * Copyright 2005 Openedhand Ltd.
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

extern "C" {
    type device;
    type regmap;
    type snd_pcm_hw_constraint_list;
    type snd_soc_dapm_widget;
    type snd_kcontrol;
    type snd_soc_component;
    type snd_soc_dapm_context;
    type snd_soc_dai;
    type snd_pcm_substream;
    type snd_pcm_hw_params;
    type spi_device;
    type i2c_client;

    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_widget) -> *mut snd_soc_component;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn msleep(msecs: c_uint);
    fn snd_pcm_hw_constraint_list(runtime: *mut c_void, cond: c_uint, var: c_uint, l: *const snd_pcm_hw_constraint_list) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn printk(fmt: *const c_char, ...);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn spi_set_drvdata(spi: *mut spi_device, data: *mut c_void);
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn i2c_add_driver(driver: *mut i2c_driver) -> c_int;
    fn i2c_del_driver(driver: *mut i2c_driver);
    fn spi_register_driver(driver: *mut spi_driver) -> c_int;
    fn spi_unregister_driver(driver: *mut spi_driver);
}

#[repr(C)]
struct reg_default {
    reg: c_uint,
    def: c_uint,
}

#[repr(C)]
struct wm8988_priv {
    regmap: *mut regmap,
    sysclk: c_uint,
    sysclk_constraints: *const snd_pcm_hw_constraint_list,
}

#[repr(C)]
struct _coeff_div {
    mclk: u32,
    rate: u32,
    fs: u16,
    sr_usb: u8,
}

impl _coeff_div {
    const fn new(mclk: u32, rate: u32, fs: u16, sr: u8, usb: u8) -> Self {
        Self { mclk, rate, fs, sr_usb: (sr & 0x1f) | ((usb & 0x01) << 5) }
    }

    fn sr(&self) -> u8 {
        self.sr_usb & 0x1f
    }

    fn usb(&self) -> u8 {
        (self.sr_usb >> 5) & 0x01
    }
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

static wm8988_reg_defaults: [reg_default; 33] = [
    reg_default { reg: 0, def: 0x0097 },
    reg_default { reg: 1, def: 0x0097 },
    reg_default { reg: 2, def: 0x0079 },
    reg_default { reg: 3, def: 0x0079 },
    reg_default { reg: 5, def: 0x0008 },
    reg_default { reg: 7, def: 0x000a },
    reg_default { reg: 8, def: 0x0000 },
    reg_default { reg: 10, def: 0x00ff },
    reg_default { reg: 11, def: 0x00ff },
    reg_default { reg: 12, def: 0x000f },
    reg_default { reg: 13, def: 0x000f },
    reg_default { reg: 16, def: 0x0000 },
    reg_default { reg: 17, def: 0x007b },
    reg_default { reg: 18, def: 0x0000 },
    reg_default { reg: 19, def: 0x0032 },
    reg_default { reg: 20, def: 0x0000 },
    reg_default { reg: 21, def: 0x00c3 },
    reg_default { reg: 22, def: 0x00c3 },
    reg_default { reg: 23, def: 0x00c0 },
    reg_default { reg: 24, def: 0x0000 },
    reg_default { reg: 25, def: 0x0000 },
    reg_default { reg: 26, def: 0x0000 },
    reg_default { reg: 27, def: 0x0000 },
    reg_default { reg: 31, def: 0x0000 },
    reg_default { reg: 32, def: 0x0000 },
    reg_default { reg: 33, def: 0x0000 },
    reg_default { reg: 34, def: 0x0050 },
    reg_default { reg: 35, def: 0x0050 },
    reg_default { reg: 36, def: 0x0050 },
    reg_default { reg: 37, def: 0x0050 },
    reg_default { reg: 40, def: 0x0079 },
    reg_default { reg: 41, def: 0x0079 },
    reg_default { reg: 42, def: 0x0079 },
];

unsafe extern "C" fn wm8988_writeable(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        WM8988_LINVOL | WM8988_RINVOL | WM8988_LOUT1V | WM8988_ROUT1V |
        WM8988_ADCDAC | WM8988_IFACE | WM8988_SRATE | WM8988_LDAC |
        WM8988_RDAC | WM8988_BASS | WM8988_TREBLE | WM8988_RESET |
        WM8988_3D | WM8988_ALC1 | WM8988_ALC2 | WM8988_ALC3 |
        WM8988_NGATE | WM8988_LADC | WM8988_RADC | WM8988_ADCTL1 |
        WM8988_ADCTL2 | WM8988_PWR1 | WM8988_PWR2 | WM8988_ADCTL3 |
        WM8988_ADCIN | WM8988_LADCIN | WM8988_RADCIN | WM8988_LOUTM1 |
        WM8988_LOUTM2 | WM8988_ROUTM1 | WM8988_ROUTM2 | WM8988_LOUT2V |
        WM8988_ROUT2V | WM8988_LPPB => true,
        _ => false,
    }
}

macro_rules! wm8988_reset {
    ($c:expr) => {
        snd_soc_component_write($c, WM8988_RESET, 0)
    };
}

static bass_boost_txt: [*const c_char; 2] = [c"Linear Control".as_ptr(), c"Adaptive Boost".as_ptr()];
static bass_boost: soc_enum = SOC_ENUM_SINGLE_DECL!(WM8988_BASS, 7, bass_boost_txt);
static bass_filter_txt: [*const c_char; 2] = [c"130Hz @ 48kHz".as_ptr(), c"200Hz @ 48kHz".as_ptr()];
static bass_filter: soc_enum = SOC_ENUM_SINGLE_DECL!(WM8988_BASS, 6, bass_filter_txt);
static treble_txt: [*const c_char; 2] = [c"8kHz".as_ptr(), c"4kHz".as_ptr()];
static treble: soc_enum = SOC_ENUM_SINGLE_DECL!(WM8988_TREBLE, 6, treble_txt);
static stereo_3d_lc_txt: [*const c_char; 2] = [c"200Hz".as_ptr(), c"500Hz".as_ptr()];
static stereo_3d_lc: soc_enum = SOC_ENUM_SINGLE_DECL!(WM8988_3D, 5, stereo_3d_lc_txt);
static stereo_3d_uc_txt: [*const c_char; 2] = [c"2.2kHz".as_ptr(), c"1.5kHz".as_ptr()];
static stereo_3d_uc: soc_enum = SOC_ENUM_SINGLE_DECL!(WM8988_3D, 6, stereo_3d_uc_txt);
static stereo_3d_func_txt: [*const c_char; 2] = [c"Capture".as_ptr(), c"Playback".as_ptr()];
static stereo_3d_func: soc_enum = SOC_ENUM_SINGLE_DECL!(WM8988_3D, 7, stereo_3d_func_txt);
static alc_func_txt: [*const c_char; 4] = [c"Off".as_ptr(), c"Right".as_ptr(), c"Left".as_ptr(), c"Stereo".as_ptr()];
static alc_func: soc_enum = SOC_ENUM_SINGLE_DECL!(WM8988_ALC1, 7, alc_func_txt);
static ng_type_txt: [*const c_char; 2] = [c"Constant PGA Gain".as_ptr(), c"Mute ADC Output".as_ptr()];
static ng_type: soc_enum = SOC_ENUM_SINGLE_DECL!(WM8988_NGATE, 1, ng_type_txt);
static deemph_txt: [*const c_char; 4] = [c"None".as_ptr(), c"32Khz".as_ptr(), c"44.1Khz".as_ptr(), c"48Khz".as_ptr()];
static deemph: soc_enum = SOC_ENUM_SINGLE_DECL!(WM8988_ADCDAC, 1, deemph_txt);
static adcpol_txt: [*const c_char; 4] = [c"Normal".as_ptr(), c"L Invert".as_ptr(), c"R Invert".as_ptr(), c"L + R Invert".as_ptr()];
static adcpol: soc_enum = SOC_ENUM_SINGLE_DECL!(WM8988_ADCDAC, 5, adcpol_txt);

static pga_tlv: [c_uint; 0] = DECLARE_TLV_DB_SCALE!(-1725, 75, 0);
static adc_tlv: [c_uint; 0] = DECLARE_TLV_DB_SCALE!(-9750, 50, 1);
static dac_tlv: [c_uint; 0] = DECLARE_TLV_DB_SCALE!(-12750, 50, 1);
static out_tlv: [c_uint; 0] = DECLARE_TLV_DB_SCALE!(-12100, 100, 1);
static bypass_tlv: [c_uint; 0] = DECLARE_TLV_DB_SCALE!(-1500, 300, 0);

static wm8988_snd_controls: [snd_kcontrol_new; 36] = [
    SOC_ENUM!(c"Bass Boost".as_ptr(), bass_boost),
    SOC_ENUM!(c"Bass Filter".as_ptr(), bass_filter),
    SOC_SINGLE!(c"Bass Volume".as_ptr(), WM8988_BASS, 0, 15, 1),
    SOC_SINGLE!(c"Treble Volume".as_ptr(), WM8988_TREBLE, 0, 15, 0),
    SOC_ENUM!(c"Treble Cut-off".as_ptr(), treble),
    SOC_SINGLE!(c"3D Switch".as_ptr(), WM8988_3D, 0, 1, 0),
    SOC_SINGLE!(c"3D Volume".as_ptr(), WM8988_3D, 1, 15, 0),
    SOC_ENUM!(c"3D Lower Cut-off".as_ptr(), stereo_3d_lc),
    SOC_ENUM!(c"3D Upper Cut-off".as_ptr(), stereo_3d_uc),
    SOC_ENUM!(c"3D Mode".as_ptr(), stereo_3d_func),
    SOC_SINGLE!(c"ALC Capture Target Volume".as_ptr(), WM8988_ALC1, 0, 7, 0),
    SOC_SINGLE!(c"ALC Capture Max Volume".as_ptr(), WM8988_ALC1, 4, 7, 0),
    SOC_ENUM!(c"ALC Capture Function".as_ptr(), alc_func),
    SOC_SINGLE!(c"ALC Capture ZC Switch".as_ptr(), WM8988_ALC2, 7, 1, 0),
    SOC_SINGLE!(c"ALC Capture Hold Time".as_ptr(), WM8988_ALC2, 0, 15, 0),
    SOC_SINGLE!(c"ALC Capture Decay Time".as_ptr(), WM8988_ALC3, 4, 15, 0),
    SOC_SINGLE!(c"ALC Capture Attack Time".as_ptr(), WM8988_ALC3, 0, 15, 0),
    SOC_SINGLE!(c"ALC Capture NG Threshold".as_ptr(), WM8988_NGATE, 3, 31, 0),
    SOC_ENUM!(c"ALC Capture NG Type".as_ptr(), ng_type),
    SOC_SINGLE!(c"ALC Capture NG Switch".as_ptr(), WM8988_NGATE, 0, 1, 0),
    SOC_SINGLE!(c"ZC Timeout Switch".as_ptr(), WM8988_ADCTL1, 0, 1, 0),
    SOC_DOUBLE_R_TLV!(c"Capture Digital Volume".as_ptr(), WM8988_LADC, WM8988_RADC, 0, 255, 0, adc_tlv),
    SOC_DOUBLE_R_TLV!(c"Capture Volume".as_ptr(), WM8988_LINVOL, WM8988_RINVOL, 0, 63, 0, pga_tlv),
    SOC_DOUBLE_R!(c"Capture ZC Switch".as_ptr(), WM8988_LINVOL, WM8988_RINVOL, 6, 1, 0),
    SOC_DOUBLE_R!(c"Capture Switch".as_ptr(), WM8988_LINVOL, WM8988_RINVOL, 7, 1, 1),
    SOC_ENUM!(c"Playback De-emphasis".as_ptr(), deemph),
    SOC_ENUM!(c"Capture Polarity".as_ptr(), adcpol),
    SOC_SINGLE!(c"Playback 6dB Attenuate".as_ptr(), WM8988_ADCDAC, 7, 1, 0),
    SOC_SINGLE!(c"Capture 6dB Attenuate".as_ptr(), WM8988_ADCDAC, 8, 1, 0),
    SOC_DOUBLE_R_TLV!(c"PCM Volume".as_ptr(), WM8988_LDAC, WM8988_RDAC, 0, 255, 0, dac_tlv),
    SOC_SINGLE_TLV!(c"Left Mixer Left Bypass Volume".as_ptr(), WM8988_LOUTM1, 4, 7, 1, bypass_tlv),
    SOC_SINGLE_TLV!(c"Left Mixer Right Bypass Volume".as_ptr(), WM8988_LOUTM2, 4, 7, 1, bypass_tlv),
    SOC_SINGLE_TLV!(c"Right Mixer Left Bypass Volume".as_ptr(), WM8988_ROUTM1, 4, 7, 1, bypass_tlv),
    SOC_SINGLE_TLV!(c"Right Mixer Right Bypass Volume".as_ptr(), WM8988_ROUTM2, 4, 7, 1, bypass_tlv),
    SOC_DOUBLE_R!(c"Output 1 Playback ZC Switch".as_ptr(), WM8988_LOUT1V, WM8988_ROUT1V, 7, 1, 0),
    SOC_DOUBLE_R_TLV!(c"Output 1 Playback Volume".as_ptr(), WM8988_LOUT1V, WM8988_ROUT1V, 0, 127, 0, out_tlv),
    SOC_DOUBLE_R!(c"Output 2 Playback ZC Switch".as_ptr(), WM8988_LOUT2V, WM8988_ROUT2V, 7, 1, 0),
    SOC_DOUBLE_R_TLV!(c"Output 2 Playback Volume".as_ptr(), WM8988_LOUT2V, WM8988_ROUT2V, 0, 127, 0, out_tlv),
];

unsafe extern "C" fn wm8988_lrc_control(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, _event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let mut adctl2 = snd_soc_component_read(component, WM8988_ADCTL2) as u16;

    /* Use the DAC to gate LRC if active, otherwise use ADC */
    if (snd_soc_component_read(component, WM8988_PWR2) & 0x180) != 0 {
        adctl2 &= !0x4;
    } else {
        adctl2 |= 0x4;
    }

    snd_soc_component_write(component, WM8988_ADCTL2, adctl2 as c_uint)
}

static wm8988_line_texts: [*const c_char; 4] = [c"Line 1".as_ptr(), c"Line 2".as_ptr(), c"PGA".as_ptr(), c"Differential".as_ptr()];
static wm8988_line_values: [c_uint; 4] = [0, 1, 3, 4];
static wm8988_lline_enum: soc_enum = SOC_VALUE_ENUM_SINGLE!(WM8988_LOUTM1, 0, 7, wm8988_line_texts.len(), wm8988_line_texts, wm8988_line_values);
static wm8988_left_line_controls: snd_kcontrol_new = SOC_DAPM_ENUM!(c"Route".as_ptr(), wm8988_lline_enum);
static wm8988_rline_enum: soc_enum = SOC_VALUE_ENUM_SINGLE!(WM8988_ROUTM1, 0, 7, wm8988_line_texts.len(), wm8988_line_texts, wm8988_line_values);
static wm8988_right_line_controls: snd_kcontrol_new = SOC_DAPM_ENUM!(c"Route".as_ptr(), wm8988_rline_enum);

static wm8988_left_mixer_controls: [snd_kcontrol_new; 4] = [
    SOC_DAPM_SINGLE!(c"Playback Switch".as_ptr(), WM8988_LOUTM1, 8, 1, 0),
    SOC_DAPM_SINGLE!(c"Left Bypass Switch".as_ptr(), WM8988_LOUTM1, 7, 1, 0),
    SOC_DAPM_SINGLE!(c"Right Playback Switch".as_ptr(), WM8988_LOUTM2, 8, 1, 0),
    SOC_DAPM_SINGLE!(c"Right Bypass Switch".as_ptr(), WM8988_LOUTM2, 7, 1, 0),
];

static wm8988_right_mixer_controls: [snd_kcontrol_new; 4] = [
    SOC_DAPM_SINGLE!(c"Left Playback Switch".as_ptr(), WM8988_ROUTM1, 8, 1, 0),
    SOC_DAPM_SINGLE!(c"Left Bypass Switch".as_ptr(), WM8988_ROUTM1, 7, 1, 0),
    SOC_DAPM_SINGLE!(c"Playback Switch".as_ptr(), WM8988_ROUTM2, 8, 1, 0),
    SOC_DAPM_SINGLE!(c"Right Bypass Switch".as_ptr(), WM8988_ROUTM2, 7, 1, 0),
];

static wm8988_pga_sel: [*const c_char; 3] = [c"Line 1".as_ptr(), c"Line 2".as_ptr(), c"Differential".as_ptr()];
static wm8988_pga_val: [c_uint; 3] = [0, 1, 3];
static wm8988_lpga_enum: soc_enum = SOC_VALUE_ENUM_SINGLE!(WM8988_LADCIN, 6, 3, wm8988_pga_sel.len(), wm8988_pga_sel, wm8988_pga_val);
static wm8988_left_pga_controls: snd_kcontrol_new = SOC_DAPM_ENUM!(c"Route".as_ptr(), wm8988_lpga_enum);
static wm8988_rpga_enum: soc_enum = SOC_VALUE_ENUM_SINGLE!(WM8988_RADCIN, 6, 3, wm8988_pga_sel.len(), wm8988_pga_sel, wm8988_pga_val);
static wm8988_right_pga_controls: snd_kcontrol_new = SOC_DAPM_ENUM!(c"Route".as_ptr(), wm8988_rpga_enum);

static wm8988_diff_sel: [*const c_char; 2] = [c"Line 1".as_ptr(), c"Line 2".as_ptr()];
static diffmux: soc_enum = SOC_ENUM_SINGLE_DECL!(WM8988_ADCIN, 8, wm8988_diff_sel);
static wm8988_diffmux_controls: snd_kcontrol_new = SOC_DAPM_ENUM!(c"Route".as_ptr(), diffmux);

static wm8988_mono_mux: [*const c_char; 4] = [c"Stereo".as_ptr(), c"Mono (Left)".as_ptr(), c"Mono (Right)".as_ptr(), c"Digital Mono".as_ptr()];
static monomux: soc_enum = SOC_ENUM_SINGLE_DECL!(WM8988_ADCIN, 6, wm8988_mono_mux);
static wm8988_monomux_controls: snd_kcontrol_new = SOC_DAPM_ENUM!(c"Route".as_ptr(), monomux);

static wm8988_dapm_widgets: [snd_soc_dapm_widget; 26] = [
    SND_SOC_DAPM_SUPPLY!(c"Mic Bias".as_ptr(), WM8988_PWR1, 1, 0, ptr::null(), 0),
    SND_SOC_DAPM_MUX!(c"Differential Mux".as_ptr(), SND_SOC_NOPM, 0, 0, &wm8988_diffmux_controls),
    SND_SOC_DAPM_MUX!(c"Left ADC Mux".as_ptr(), SND_SOC_NOPM, 0, 0, &wm8988_monomux_controls),
    SND_SOC_DAPM_MUX!(c"Right ADC Mux".as_ptr(), SND_SOC_NOPM, 0, 0, &wm8988_monomux_controls),
    SND_SOC_DAPM_MUX!(c"Left PGA Mux".as_ptr(), WM8988_PWR1, 5, 0, &wm8988_left_pga_controls),
    SND_SOC_DAPM_MUX!(c"Right PGA Mux".as_ptr(), WM8988_PWR1, 4, 0, &wm8988_right_pga_controls),
    SND_SOC_DAPM_MUX!(c"Left Line Mux".as_ptr(), SND_SOC_NOPM, 0, 0, &wm8988_left_line_controls),
    SND_SOC_DAPM_MUX!(c"Right Line Mux".as_ptr(), SND_SOC_NOPM, 0, 0, &wm8988_right_line_controls),
    SND_SOC_DAPM_ADC!(c"Right ADC".as_ptr(), c"Right Capture".as_ptr(), WM8988_PWR1, 2, 0),
    SND_SOC_DAPM_ADC!(c"Left ADC".as_ptr(), c"Left Capture".as_ptr(), WM8988_PWR1, 3, 0),
    SND_SOC_DAPM_DAC!(c"Right DAC".as_ptr(), c"Right Playback".as_ptr(), WM8988_PWR2, 7, 0),
    SND_SOC_DAPM_DAC!(c"Left DAC".as_ptr(), c"Left Playback".as_ptr(), WM8988_PWR2, 8, 0),
    SND_SOC_DAPM_MIXER!(c"Left Mixer".as_ptr(), SND_SOC_NOPM, 0, 0, &wm8988_left_mixer_controls[0], wm8988_left_mixer_controls.len()),
    SND_SOC_DAPM_MIXER!(c"Right Mixer".as_ptr(), SND_SOC_NOPM, 0, 0, &wm8988_right_mixer_controls[0], wm8988_right_mixer_controls.len()),
    SND_SOC_DAPM_PGA!(c"Right Out 2".as_ptr(), WM8988_PWR2, 3, 0, ptr::null(), 0),
    SND_SOC_DAPM_PGA!(c"Left Out 2".as_ptr(), WM8988_PWR2, 4, 0, ptr::null(), 0),
    SND_SOC_DAPM_PGA!(c"Right Out 1".as_ptr(), WM8988_PWR2, 5, 0, ptr::null(), 0),
    SND_SOC_DAPM_PGA!(c"Left Out 1".as_ptr(), WM8988_PWR2, 6, 0, ptr::null(), 0),
    SND_SOC_DAPM_POST!(c"LRC control".as_ptr(), wm8988_lrc_control),
    SND_SOC_DAPM_OUTPUT!(c"LOUT1".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"ROUT1".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"LOUT2".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"ROUT2".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"VREF".as_ptr()),
    SND_SOC_DAPM_INPUT!(c"LINPUT1".as_ptr()),
    SND_SOC_DAPM_INPUT!(c"LINPUT2".as_ptr()),
    SND_SOC_DAPM_INPUT!(c"RINPUT1".as_ptr()),
    SND_SOC_DAPM_INPUT!(c"RINPUT2".as_ptr()),
];

static wm8988_dapm_routes: [snd_soc_dapm_route; 49] = [
    route(c"Left Line Mux".as_ptr(), c"Line 1".as_ptr(), c"LINPUT1".as_ptr()),
    route(c"Left Line Mux".as_ptr(), c"Line 2".as_ptr(), c"LINPUT2".as_ptr()),
    route(c"Left Line Mux".as_ptr(), c"PGA".as_ptr(), c"Left PGA Mux".as_ptr()),
    route(c"Left Line Mux".as_ptr(), c"Differential".as_ptr(), c"Differential Mux".as_ptr()),
    route(c"Right Line Mux".as_ptr(), c"Line 1".as_ptr(), c"RINPUT1".as_ptr()),
    route(c"Right Line Mux".as_ptr(), c"Line 2".as_ptr(), c"RINPUT2".as_ptr()),
    route(c"Right Line Mux".as_ptr(), c"PGA".as_ptr(), c"Right PGA Mux".as_ptr()),
    route(c"Right Line Mux".as_ptr(), c"Differential".as_ptr(), c"Differential Mux".as_ptr()),
    route(c"Left PGA Mux".as_ptr(), c"Line 1".as_ptr(), c"LINPUT1".as_ptr()),
    route(c"Left PGA Mux".as_ptr(), c"Line 2".as_ptr(), c"LINPUT2".as_ptr()),
    route(c"Left PGA Mux".as_ptr(), c"Differential".as_ptr(), c"Differential Mux".as_ptr()),
    route(c"Right PGA Mux".as_ptr(), c"Line 1".as_ptr(), c"RINPUT1".as_ptr()),
    route(c"Right PGA Mux".as_ptr(), c"Line 2".as_ptr(), c"RINPUT2".as_ptr()),
    route(c"Right PGA Mux".as_ptr(), c"Differential".as_ptr(), c"Differential Mux".as_ptr()),
    route(c"Differential Mux".as_ptr(), c"Line 1".as_ptr(), c"LINPUT1".as_ptr()),
    route(c"Differential Mux".as_ptr(), c"Line 1".as_ptr(), c"RINPUT1".as_ptr()),
    route(c"Differential Mux".as_ptr(), c"Line 2".as_ptr(), c"LINPUT2".as_ptr()),
    route(c"Differential Mux".as_ptr(), c"Line 2".as_ptr(), c"RINPUT2".as_ptr()),
    route(c"Left ADC Mux".as_ptr(), c"Stereo".as_ptr(), c"Left PGA Mux".as_ptr()),
    route(c"Left ADC Mux".as_ptr(), c"Mono (Left)".as_ptr(), c"Left PGA Mux".as_ptr()),
    route(c"Left ADC Mux".as_ptr(), c"Digital Mono".as_ptr(), c"Left PGA Mux".as_ptr()),
    route(c"Right ADC Mux".as_ptr(), c"Stereo".as_ptr(), c"Right PGA Mux".as_ptr()),
    route(c"Right ADC Mux".as_ptr(), c"Mono (Right)".as_ptr(), c"Right PGA Mux".as_ptr()),
    route(c"Right ADC Mux".as_ptr(), c"Digital Mono".as_ptr(), c"Right PGA Mux".as_ptr()),
    route(c"Left ADC".as_ptr(), ptr::null(), c"Left ADC Mux".as_ptr()),
    route(c"Right ADC".as_ptr(), ptr::null(), c"Right ADC Mux".as_ptr()),
    route(c"Left Line Mux".as_ptr(), c"Line 1".as_ptr(), c"LINPUT1".as_ptr()),
    route(c"Left Line Mux".as_ptr(), c"Line 2".as_ptr(), c"LINPUT2".as_ptr()),
    route(c"Left Line Mux".as_ptr(), c"PGA".as_ptr(), c"Left PGA Mux".as_ptr()),
    route(c"Left Line Mux".as_ptr(), c"Differential".as_ptr(), c"Differential Mux".as_ptr()),
    route(c"Right Line Mux".as_ptr(), c"Line 1".as_ptr(), c"RINPUT1".as_ptr()),
    route(c"Right Line Mux".as_ptr(), c"Line 2".as_ptr(), c"RINPUT2".as_ptr()),
    route(c"Right Line Mux".as_ptr(), c"PGA".as_ptr(), c"Right PGA Mux".as_ptr()),
    route(c"Right Line Mux".as_ptr(), c"Differential".as_ptr(), c"Differential Mux".as_ptr()),
    route(c"Left Mixer".as_ptr(), c"Playback Switch".as_ptr(), c"Left DAC".as_ptr()),
    route(c"Left Mixer".as_ptr(), c"Left Bypass Switch".as_ptr(), c"Left Line Mux".as_ptr()),
    route(c"Left Mixer".as_ptr(), c"Right Playback Switch".as_ptr(), c"Right DAC".as_ptr()),
    route(c"Left Mixer".as_ptr(), c"Right Bypass Switch".as_ptr(), c"Right Line Mux".as_ptr()),
    route(c"Right Mixer".as_ptr(), c"Left Playback Switch".as_ptr(), c"Left DAC".as_ptr()),
    route(c"Right Mixer".as_ptr(), c"Left Bypass Switch".as_ptr(), c"Left Line Mux".as_ptr()),
    route(c"Right Mixer".as_ptr(), c"Playback Switch".as_ptr(), c"Right DAC".as_ptr()),
    route(c"Right Mixer".as_ptr(), c"Right Bypass Switch".as_ptr(), c"Right Line Mux".as_ptr()),
    route(c"Left Out 1".as_ptr(), ptr::null(), c"Left Mixer".as_ptr()),
    route(c"LOUT1".as_ptr(), ptr::null(), c"Left Out 1".as_ptr()),
    route(c"Right Out 1".as_ptr(), ptr::null(), c"Right Mixer".as_ptr()),
    route(c"ROUT1".as_ptr(), ptr::null(), c"Right Out 1".as_ptr()),
    route(c"Left Out 2".as_ptr(), ptr::null(), c"Left Mixer".as_ptr()),
    route(c"LOUT2".as_ptr(), ptr::null(), c"Left Out 2".as_ptr()),
    route(c"Right Out 2".as_ptr(), ptr::null(), c"Right Mixer".as_ptr()),
    route(c"ROUT2".as_ptr(), ptr::null(), c"Right Out 2".as_ptr()),
];

static coeff_div: [_coeff_div; 30] = [
    _coeff_div::new(12288000, 8000, 1536, 0x6, 0x0),
    _coeff_div::new(11289600, 8000, 1408, 0x16, 0x0),
    _coeff_div::new(18432000, 8000, 2304, 0x7, 0x0),
    _coeff_div::new(16934400, 8000, 2112, 0x17, 0x0),
    _coeff_div::new(12000000, 8000, 1500, 0x6, 0x1),
    _coeff_div::new(11289600, 11025, 1024, 0x18, 0x0),
    _coeff_div::new(16934400, 11025, 1536, 0x19, 0x0),
    _coeff_div::new(12000000, 11025, 1088, 0x19, 0x1),
    _coeff_div::new(12288000, 16000, 768, 0xa, 0x0),
    _coeff_div::new(18432000, 16000, 1152, 0xb, 0x0),
    _coeff_div::new(12000000, 16000, 750, 0xa, 0x1),
    _coeff_div::new(11289600, 22050, 512, 0x1a, 0x0),
    _coeff_div::new(16934400, 22050, 768, 0x1b, 0x0),
    _coeff_div::new(12000000, 22050, 544, 0x1b, 0x1),
    _coeff_div::new(12288000, 32000, 384, 0xc, 0x0),
    _coeff_div::new(18432000, 32000, 576, 0xd, 0x0),
    _coeff_div::new(12000000, 32000, 375, 0xa, 0x1),
    _coeff_div::new(11289600, 44100, 256, 0x10, 0x0),
    _coeff_div::new(16934400, 44100, 384, 0x11, 0x0),
    _coeff_div::new(12000000, 44100, 272, 0x11, 0x1),
    _coeff_div::new(12288000, 48000, 256, 0x0, 0x0),
    _coeff_div::new(18432000, 48000, 384, 0x1, 0x0),
    _coeff_div::new(12000000, 48000, 250, 0x0, 0x1),
    _coeff_div::new(11289600, 88200, 128, 0x1e, 0x0),
    _coeff_div::new(16934400, 88200, 192, 0x1f, 0x0),
    _coeff_div::new(12000000, 88200, 136, 0x1f, 0x1),
    _coeff_div::new(12288000, 96000, 128, 0xe, 0x0),
    _coeff_div::new(18432000, 96000, 192, 0xf, 0x0),
    _coeff_div::new(12000000, 96000, 125, 0xe, 0x1),
];

fn get_coeff(mclk: c_int, rate: c_int) -> c_int {
    let mut i = 0usize;
    while i < coeff_div.len() {
        if coeff_div[i].rate == rate as u32 && coeff_div[i].mclk == mclk as u32 {
            return i as c_int;
        }
        i += 1;
    }
    -EINVAL
}

static rates_12288: [c_uint; 7] = [8000, 12000, 16000, 24000, 32000, 48000, 96000];
static constraints_12288: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: rates_12288.len() as c_uint,
    list: rates_12288.as_ptr(),
};

static rates_112896: [c_uint; 4] = [8000, 11025, 22050, 44100];
static constraints_112896: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: rates_112896.len() as c_uint,
    list: rates_112896.as_ptr(),
};

static rates_12: [c_uint; 12] = [8000, 11025, 12000, 16000, 22050, 24000, 32000, 41100, 48000, 48000, 88235, 96000];
static constraints_12: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: rates_12.len() as c_uint,
    list: rates_12.as_ptr(),
};

unsafe extern "C" fn wm8988_set_dai_sysclk(codec_dai: *mut snd_soc_dai, _clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component = (*codec_dai).component;
    let wm8988 = snd_soc_component_get_drvdata(component) as *mut wm8988_priv;

    match freq {
        11289600 | 18432000 | 22579200 | 36864000 => {
            (*wm8988).sysclk_constraints = &constraints_112896;
            (*wm8988).sysclk = freq;
            0
        }
        12288000 | 16934400 | 24576000 | 33868800 => {
            (*wm8988).sysclk_constraints = &constraints_12288;
            (*wm8988).sysclk = freq;
            0
        }
        12000000 | 24000000 => {
            (*wm8988).sysclk_constraints = &constraints_12;
            (*wm8988).sysclk = freq;
            0
        }
        _ => -EINVAL,
    }
}

unsafe extern "C" fn wm8988_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let mut iface: u16 = 0;

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => iface = 0x0040,
        SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => iface |= 0x0002,
        SND_SOC_DAIFMT_RIGHT_J => {}
        SND_SOC_DAIFMT_LEFT_J => iface |= 0x0001,
        SND_SOC_DAIFMT_DSP_A => iface |= 0x0003,
        SND_SOC_DAIFMT_DSP_B => iface |= 0x0013,
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_IF => iface |= 0x0090,
        SND_SOC_DAIFMT_IB_NF => iface |= 0x0080,
        SND_SOC_DAIFMT_NB_IF => iface |= 0x0010,
        _ => return -EINVAL,
    }

    snd_soc_component_write(component, WM8988_IFACE, iface as c_uint);
    0
}

unsafe extern "C" fn wm8988_pcm_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let wm8988 = snd_soc_component_get_drvdata(component) as *mut wm8988_priv;

    if (*wm8988).sysclk == 0 {
        dev_err((*component).dev, c"No MCLK configured, call set_sysclk() on init\n".as_ptr());
        return -EINVAL;
    }

    snd_pcm_hw_constraint_list((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_RATE, (*wm8988).sysclk_constraints);
    0
}

unsafe extern "C" fn wm8988_pcm_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let wm8988 = snd_soc_component_get_drvdata(component) as *mut wm8988_priv;
    let mut iface = (snd_soc_component_read(component, WM8988_IFACE) & 0x1f3) as u16;
    let mut srate = (snd_soc_component_read(component, WM8988_SRATE) & 0x180) as u16;
    let mut coeff = get_coeff((*wm8988).sysclk as c_int, params_rate(params) as c_int);

    if coeff < 0 {
        coeff = get_coeff(((*wm8988).sysclk / 2) as c_int, params_rate(params) as c_int);
        srate |= 0x40;
    }
    if coeff < 0 {
        dev_err((*component).dev, c"Unable to configure sample rate %dHz with %dHz MCLK\n".as_ptr(), params_rate(params), (*wm8988).sysclk);
        return coeff;
    }

    match params_width(params) {
        16 => {}
        20 => iface |= 0x0004,
        24 => iface |= 0x0008,
        32 => iface |= 0x000c,
        _ => {}
    }

    snd_soc_component_write(component, WM8988_IFACE, iface as c_uint);
    if coeff >= 0 {
        snd_soc_component_write(
            component,
            WM8988_SRATE,
            (srate as c_uint) | ((coeff_div[coeff as usize].sr() as c_uint) << 1) | (coeff_div[coeff as usize].usb() as c_uint),
        );
    }

    0
}

unsafe extern "C" fn wm8988_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*dai).component;
    let mute_reg = (snd_soc_component_read(component, WM8988_ADCDAC) & 0xfff7) as u16;

    if mute != 0 {
        snd_soc_component_write(component, WM8988_ADCDAC, (mute_reg | 0x8) as c_uint);
    } else {
        snd_soc_component_write(component, WM8988_ADCDAC, mute_reg as c_uint);
    }
    0
}

unsafe extern "C" fn wm8988_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let wm8988 = snd_soc_component_get_drvdata(component) as *mut wm8988_priv;
    let dapm = snd_soc_component_to_dapm(component);
    let pwr_reg = (snd_soc_component_read(component, WM8988_PWR1) & !0x1c1) as u16;

    match level {
        SND_SOC_BIAS_ON => {}
        SND_SOC_BIAS_PREPARE => {
            snd_soc_component_write(component, WM8988_PWR1, (pwr_reg | 0x00c0) as c_uint);
        }
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                regcache_sync((*wm8988).regmap);
                snd_soc_component_write(component, WM8988_PWR1, (pwr_reg | 0x1c1) as c_uint);
                msleep(100);
            }
            snd_soc_component_write(component, WM8988_PWR1, (pwr_reg | 0x0141) as c_uint);
        }
        SND_SOC_BIAS_OFF => {
            snd_soc_component_write(component, WM8988_PWR1, 0x0000);
        }
        _ => {}
    }
    0
}

const WM8988_RATES: c_uint = SNDRV_PCM_RATE_8000_96000;
const WM8988_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE;

static wm8988_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(wm8988_pcm_startup),
    hw_params: Some(wm8988_pcm_hw_params),
    set_fmt: Some(wm8988_set_dai_fmt),
    set_sysclk: Some(wm8988_set_dai_sysclk),
    mute_stream: Some(wm8988_mute),
    no_capture_mute: 1,
};

static mut wm8988_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"wm8988-hifi".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: WM8988_RATES,
        formats: WM8988_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: WM8988_RATES,
        formats: WM8988_FORMATS,
    },
    ops: &wm8988_ops,
    symmetric_rate: 1,
};

unsafe extern "C" fn wm8988_probe(component: *mut snd_soc_component) -> c_int {
    let mut ret: c_int = 0;

    ret = wm8988_reset!(component);
    if ret < 0 {
        dev_err((*component).dev, c"Failed to issue reset\n".as_ptr());
        return ret;
    }

    snd_soc_component_update_bits(component, WM8988_RADC, 0x0100, 0x0100);
    snd_soc_component_update_bits(component, WM8988_RDAC, 0x0100, 0x0100);
    snd_soc_component_update_bits(component, WM8988_ROUT1V, 0x0100, 0x0100);
    snd_soc_component_update_bits(component, WM8988_ROUT2V, 0x0100, 0x0100);
    snd_soc_component_update_bits(component, WM8988_RINVOL, 0x0100, 0x0100);

    0
}

static soc_component_dev_wm8988: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wm8988_probe),
    set_bias_level: Some(wm8988_set_bias_level),
    controls: wm8988_snd_controls.as_ptr(),
    num_controls: wm8988_snd_controls.len() as c_uint,
    dapm_widgets: wm8988_dapm_widgets.as_ptr(),
    num_dapm_widgets: wm8988_dapm_widgets.len() as c_uint,
    dapm_routes: wm8988_dapm_routes.as_ptr(),
    num_dapm_routes: wm8988_dapm_routes.len() as c_uint,
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static wm8988_regmap: regmap_config = regmap_config {
    reg_bits: 7,
    val_bits: 9,
    max_register: WM8988_LPPB,
    writeable_reg: Some(wm8988_writeable),
    cache_type: REGCACHE_MAPLE,
    reg_defaults: wm8988_reg_defaults.as_ptr(),
    num_reg_defaults: wm8988_reg_defaults.len() as c_uint,
};

/* C conditional: #if defined(CONFIG_SPI_MASTER) */
unsafe extern "C" fn wm8988_spi_probe(spi: *mut spi_device) -> c_int {
    let wm8988: *mut wm8988_priv;
    let mut ret: c_int;

    wm8988 = devm_kzalloc(&mut (*spi).dev, core::mem::size_of::<wm8988_priv>(), GFP_KERNEL) as *mut wm8988_priv;
    if wm8988.is_null() {
        return -ENOMEM;
    }

    (*wm8988).regmap = devm_regmap_init_spi(spi, &wm8988_regmap);
    if IS_ERR((*wm8988).regmap as *const c_void) {
        ret = PTR_ERR((*wm8988).regmap as *const c_void);
        dev_err(&mut (*spi).dev, c"Failed to init regmap: %d\n".as_ptr(), ret);
        return ret;
    }

    spi_set_drvdata(spi, wm8988 as *mut c_void);

    ret = devm_snd_soc_register_component(&mut (*spi).dev, &soc_component_dev_wm8988, &mut wm8988_dai, 1);
    ret
}

static mut wm8988_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: c"wm8988".as_ptr(),
    },
    probe: Some(wm8988_spi_probe),
};
/* end CONFIG_SPI_MASTER */

/* C conditional: #if IS_ENABLED(CONFIG_I2C) */
unsafe extern "C" fn wm8988_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let wm8988: *mut wm8988_priv;
    let mut ret: c_int;

    wm8988 = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<wm8988_priv>(), GFP_KERNEL) as *mut wm8988_priv;
    if wm8988.is_null() {
        return -ENOMEM;
    }

    i2c_set_clientdata(i2c, wm8988 as *mut c_void);

    (*wm8988).regmap = devm_regmap_init_i2c(i2c, &wm8988_regmap);
    if IS_ERR((*wm8988).regmap as *const c_void) {
        ret = PTR_ERR((*wm8988).regmap as *const c_void);
        dev_err(&mut (*i2c).dev, c"Failed to init regmap: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = devm_snd_soc_register_component(&mut (*i2c).dev, &soc_component_dev_wm8988, &mut wm8988_dai, 1);
    ret
}

static wm8988_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: c"wm8988".as_ptr() },
    i2c_device_id { name: ptr::null() },
];
MODULE_DEVICE_TABLE!(i2c, wm8988_i2c_id);

static mut wm8988_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"wm8988".as_ptr(),
    },
    probe: Some(wm8988_i2c_probe),
    id_table: wm8988_i2c_id.as_ptr(),
};
/* end CONFIG_I2C */

unsafe extern "C" fn wm8988_modinit() -> c_int {
    let mut ret: c_int = 0;

    /* C conditional: #if IS_ENABLED(CONFIG_I2C) */
    ret = i2c_add_driver(&mut wm8988_i2c_driver);
    if ret != 0 {
        printk(c"Failed to register WM8988 I2C driver: %d\n".as_ptr(), ret);
    }

    /* C conditional: #if defined(CONFIG_SPI_MASTER) */
    ret = spi_register_driver(&mut wm8988_spi_driver);
    if ret != 0 {
        printk(c"Failed to register WM8988 SPI driver: %d\n".as_ptr(), ret);
    }

    ret
}
module_init!(wm8988_modinit);

unsafe extern "C" fn wm8988_exit() {
    /* C conditional: #if IS_ENABLED(CONFIG_I2C) */
    i2c_del_driver(&mut wm8988_i2c_driver);
    /* C conditional: #if defined(CONFIG_SPI_MASTER) */
    spi_unregister_driver(&mut wm8988_spi_driver);
}
module_exit!(wm8988_exit);

MODULE_DESCRIPTION!(c"ASoC WM8988 driver".as_ptr());
MODULE_AUTHOR!(c"Mark Brown <broonie@opensource.wolfsonmicro.com>".as_ptr());
MODULE_LICENSE!(c"GPL".as_ptr());

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
