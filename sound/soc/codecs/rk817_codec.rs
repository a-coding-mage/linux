// SPDX-License-Identifier: GPL-2.0
//
// rk817 ALSA SoC Audio driver
//
// Copyright (c) 2018, Fuzhou Rockchip Electronics Co., Ltd All rights reserved.
//
// Translated from ./rk817_codec.c. C include dependencies are expected to be
// supplied by the surrounding kernel Rust binding environment.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct rk808 {
    pub regmap: *mut c_void,
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
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
pub struct device {
    pub parent: *mut device,
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub no_capture_mute: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub set_pll:
        Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int, c_uint, c_uint) -> c_int>,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

#[repr(C)]
pub struct rk817_codec_priv {
    pub component: *mut snd_soc_component,
    pub rk808: *mut rk808,
    pub mclk: *mut clk,
    pub stereo_sysclk: c_uint,
    pub mic_in_differential: bool,
}

unsafe extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_init_regmap(component: *mut snd_soc_component, regmap: *mut c_void);
    fn snd_soc_component_exit_regmap(component: *mut snd_soc_component);
    fn snd_soc_component_set_pll(
        component: *mut snd_soc_component,
        pll_id: c_int,
        source: c_int,
        freq_in: c_uint,
        freq_out: c_uint,
    ) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn of_get_child_by_name(node: *mut device_node, name: *const c_char) -> *mut device_node;
    fn of_property_read_bool(node: *mut device_node, propname: *const c_char) -> bool;
    fn of_node_put(node: *mut device_node);
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...) -> c_int;
}

unsafe extern "C" {
    static RK817_CODEC_DDAC_POPD_DACST: c_uint;
    static RK817_CODEC_DDAC_SR_LMT0: c_uint;
    static RK817_CODEC_DADC_SR_ACL0: c_uint;
    static RK817_CODEC_DTOP_VUCTIME: c_uint;
    static RK817_CODEC_AMIC_CFG0: c_uint;
    static MIC_DIFF_MASK: c_uint;
    static MIC_DIFF_EN: c_uint;
    static RK817_CODEC_APLL_CFG1: c_uint;
    static RK817_CODEC_APLL_CFG2: c_uint;
    static RK817_CODEC_APLL_CFG3: c_uint;
    static RK817_CODEC_APLL_CFG4: c_uint;
    static RK817_CODEC_APLL_CFG5: c_uint;
    static RK817_CODEC_DDAC_VOLL: c_uint;
    static RK817_CODEC_DDAC_VOLR: c_uint;
    static RK817_CODEC_DADC_VOLL: c_uint;
    static RK817_CODEC_DADC_VOLR: c_uint;
    static RK817_CODEC_DMIC_PGA_GAIN: c_uint;
    static RK817_CODEC_AREF_RTCFG1: c_uint;
    static RK817_CODEC_DI2S_RXCMD_TSD: c_uint;
    static RK817_CODEC_DTOP_DIGEN_CLKE: c_uint;
    static RK817_CODEC_DI2S_TXCR3_TXCMD: c_uint;
    static RK817_CODEC_AADC_CFG0: c_uint;
    static RK817_CODEC_ADAC_CFG1: c_uint;
    static RK817_CODEC_DDAC_MUTE_MIXCTL: c_uint;
    static RK817_CODEC_ACLASSD_CFG1: c_uint;
    static RK817_CODEC_ACLASSD_CFG2: c_uint;
    static RK817_CODEC_AHP_CP: c_uint;
    static RK817_CODEC_AHP_CFG0: c_uint;
    static SND_SOC_NOPM: c_uint;
    static SND_SOC_DAIFMT_MASTER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static RK817_I2S_MODE_SLV: c_uint;
    static RK817_I2S_MODE_MST: c_uint;
    static RK817_I2S_MODE_MASK: c_uint;
    static RK817_CODEC_DI2S_CKM: c_uint;
    static SNDRV_PCM_FORMAT_S16_LE: c_int;
    static SNDRV_PCM_FORMAT_S24_LE: c_int;
    static SNDRV_PCM_FORMAT_S32_LE: c_int;
    static RK817_CODEC_DI2S_RXCR2: c_uint;
    static RK817_CODEC_DI2S_TXCR2: c_uint;
    static VDW_RX_16BITS: c_uint;
    static VDW_TX_16BITS: c_uint;
    static VDW_RX_24BITS: c_uint;
    static VDW_TX_24BITS: c_uint;
    static DACMT_MASK: c_uint;
    static DACMT_ENABLE: c_uint;
    static DACMT_DISABLE: c_uint;
    static RK817_CODEC_DTOP_LPT_SRST: c_uint;
    static SNDRV_PCM_RATE_8000: c_uint;
    static SNDRV_PCM_RATE_16000: c_uint;
    static SNDRV_PCM_RATE_32000: c_uint;
    static SNDRV_PCM_RATE_44100: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S20_3LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_uint;
    static GFP_KERNEL: c_uint;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static ENXIO: c_int;
}

/*
 * This sets the codec up with the values defined in the default implementation including the APLL
 * from the Rockchip vendor kernel. I do not know if these values are universal despite differing
 * from the default values defined above and taken from the datasheet, or implementation specific.
 * I don't have another implementation to compare from the Rockchip sources. Hard-coding for now.
 * Additionally, I do not know according to the documentation the units accepted for the clock
 * values, so for the moment those are left unvalidated.
 */
unsafe extern "C" fn rk817_init(component: *mut snd_soc_component) -> c_int {
    let rk817 = snd_soc_component_get_drvdata(component) as *mut rk817_codec_priv;

    snd_soc_component_write(component, RK817_CODEC_DDAC_POPD_DACST, 0x02);
    snd_soc_component_write(component, RK817_CODEC_DDAC_SR_LMT0, 0x02);
    snd_soc_component_write(component, RK817_CODEC_DADC_SR_ACL0, 0x02);
    snd_soc_component_write(component, RK817_CODEC_DTOP_VUCTIME, 0xf4);
    if (*rk817).mic_in_differential {
        snd_soc_component_update_bits(
            component,
            RK817_CODEC_AMIC_CFG0,
            MIC_DIFF_MASK,
            MIC_DIFF_EN,
        );
    }

    0
}

unsafe extern "C" fn rk817_set_component_pll(
    component: *mut snd_soc_component,
    _pll_id: c_int,
    _source: c_int,
    _freq_in: c_uint,
    _freq_out: c_uint,
) -> c_int {
    /* Set resistor value and charge pump current for PLL. */
    snd_soc_component_write(component, RK817_CODEC_APLL_CFG1, 0x58);
    /* Set the PLL feedback clock divide value (values not documented). */
    snd_soc_component_write(component, RK817_CODEC_APLL_CFG2, 0x2d);
    /* Set the PLL pre-divide value (values not documented). */
    snd_soc_component_write(component, RK817_CODEC_APLL_CFG3, 0x0c);
    /* Set the PLL VCO output clock divide and PLL divided ratio of PLL High Clk (values not
     * documented).
     */
    snd_soc_component_write(component, RK817_CODEC_APLL_CFG4, 0xa5);

    0
}

/*
 * DDAC/DADC L/R volume setting
 * 0db~-95db, 0.375db/step, for example:
 * 0x00: 0dB
 * 0xff: -95dB
 */
static rk817_vol_tlv: [c_uint; 2] = [-9500i32 as c_uint, 0];

/*
 * PGA GAIN L/R volume setting
 * 27db~-18db, 3db/step, for example:
 * 0x0: -18dB
 * 0xf: 27dB
 */
static rk817_gain_tlv: [c_uint; 2] = [-1800i32 as c_uint, 2700];

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

// The following ASoC table constructors are C macros in the original source.
// They are preserved as Rust macro invocations for the surrounding bindings.
static rk817_volume_controls: [snd_kcontrol_new; 3] = [
    SOC_DOUBLE_R_RANGE_TLV!(
        "Master Playback Volume",
        RK817_CODEC_DDAC_VOLL,
        RK817_CODEC_DDAC_VOLR,
        0,
        0x00,
        0xff,
        1,
        rk817_vol_tlv
    ),
    SOC_DOUBLE_R_RANGE_TLV!(
        "Master Capture Volume",
        RK817_CODEC_DADC_VOLL,
        RK817_CODEC_DADC_VOLR,
        0,
        0x00,
        0xff,
        1,
        rk817_vol_tlv
    ),
    SOC_DOUBLE_TLV!("Mic Capture Gain", RK817_CODEC_DMIC_PGA_GAIN, 4, 0, 0xf, 0, rk817_gain_tlv),
];

/* Since the speaker output and L headphone pin are internally the same, make audio path mutually
 * exclusive with a mux.
 */
static dac_mux_text: [*const c_char; 2] = [cstr!("HP"), cstr!("SPK")];

SOC_ENUM_SINGLE_VIRT_DECL!(dac_enum, dac_mux_text);

static dac_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("Playback Mux", dac_enum);

static rk817_dapm_widgets: [snd_soc_dapm_widget; 55] = [
    /* capture/playback common */
    SND_SOC_DAPM_SUPPLY!("LDO Regulator", RK817_CODEC_AREF_RTCFG1, 6, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("IBIAS Block", RK817_CODEC_AREF_RTCFG1, 2, 1, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("VAvg Buffer", RK817_CODEC_AREF_RTCFG1, 1, 1, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("PLL Power", RK817_CODEC_APLL_CFG5, 0, 1, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("I2S TX1 Transfer Start", RK817_CODEC_DI2S_RXCMD_TSD, 5, 0, ptr::null(), 0),
    /* capture path common */
    SND_SOC_DAPM_SUPPLY!("ADC Clock", RK817_CODEC_DTOP_DIGEN_CLKE, 7, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("I2S TX Clock", RK817_CODEC_DTOP_DIGEN_CLKE, 6, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("ADC Channel Enable", RK817_CODEC_DTOP_DIGEN_CLKE, 5, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("I2S TX Channel Enable", RK817_CODEC_DTOP_DIGEN_CLKE, 4, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("MIC Power On", RK817_CODEC_AMIC_CFG0, 6, 1, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("I2S TX3 Transfer Start", RK817_CODEC_DI2S_TXCR3_TXCMD, 7, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("I2S TX3 Right Justified", RK817_CODEC_DI2S_TXCR3_TXCMD, 3, 0, ptr::null(), 0),
    /* capture path L */
    SND_SOC_DAPM_ADC!("ADC L", "Capture", RK817_CODEC_AADC_CFG0, 7, 1),
    SND_SOC_DAPM_SUPPLY!("PGA L Power On", RK817_CODEC_AMIC_CFG0, 5, 1, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Mic Boost L1", RK817_CODEC_AMIC_CFG0, 3, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Mic Boost L2", RK817_CODEC_AMIC_CFG0, 2, 0, ptr::null(), 0),
    /* capture path R */
    SND_SOC_DAPM_ADC!("ADC R", "Capture", RK817_CODEC_AADC_CFG0, 6, 1),
    SND_SOC_DAPM_SUPPLY!("PGA R Power On", RK817_CODEC_AMIC_CFG0, 4, 1, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Mic Boost R1", RK817_CODEC_AMIC_CFG0, 3, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Mic Boost R2", RK817_CODEC_AMIC_CFG0, 3, 0, ptr::null(), 0),
    /* playback path common */
    SND_SOC_DAPM_SUPPLY!("DAC Clock", RK817_CODEC_DTOP_DIGEN_CLKE, 3, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("I2S RX Clock", RK817_CODEC_DTOP_DIGEN_CLKE, 2, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("DAC Channel Enable", RK817_CODEC_DTOP_DIGEN_CLKE, 1, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("I2S RX Channel Enable", RK817_CODEC_DTOP_DIGEN_CLKE, 0, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("DAC Bias", RK817_CODEC_ADAC_CFG1, 3, 1, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("DAC Mute Off", RK817_CODEC_DDAC_MUTE_MIXCTL, 0, 1, ptr::null(), 0),
    /* playback path speaker */
    SND_SOC_DAPM_SUPPLY!("Class D Mode", RK817_CODEC_DDAC_MUTE_MIXCTL, 4, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("High Pass Filter", RK817_CODEC_DDAC_MUTE_MIXCTL, 7, 0, ptr::null(), 0),
    SND_SOC_DAPM_DAC!("SPK DAC", "Playback", RK817_CODEC_ADAC_CFG1, 2, 1),
    SND_SOC_DAPM_SUPPLY!("Enable Class D", RK817_CODEC_ACLASSD_CFG1, 7, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Disable Class D Mute Ramp", RK817_CODEC_ACLASSD_CFG1, 6, 1, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Class D Mute Rate 1", RK817_CODEC_ACLASSD_CFG1, 3, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Class D Mute Rate 2", RK817_CODEC_ACLASSD_CFG1, 2, 1, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Class D OCPP 2", RK817_CODEC_ACLASSD_CFG2, 5, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Class D OCPP 3", RK817_CODEC_ACLASSD_CFG2, 4, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Class D OCPN 2", RK817_CODEC_ACLASSD_CFG2, 1, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Class D OCPN 3", RK817_CODEC_ACLASSD_CFG2, 0, 0, ptr::null(), 0),
    /* playback path headphones */
    SND_SOC_DAPM_SUPPLY!("Headphone Charge Pump", RK817_CODEC_AHP_CP, 4, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Headphone CP Discharge LDO", RK817_CODEC_AHP_CP, 3, 1, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Headphone OStage", RK817_CODEC_AHP_CFG0, 6, 1, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Headphone Pre Amp", RK817_CODEC_AHP_CFG0, 5, 1, ptr::null(), 0),
    SND_SOC_DAPM_DAC!("DAC L", "Playback", RK817_CODEC_ADAC_CFG1, 1, 1),
    SND_SOC_DAPM_DAC!("DAC R", "Playback", RK817_CODEC_ADAC_CFG1, 0, 1),
    /* Mux for input/output path selection */
    SND_SOC_DAPM_MUX!("Playback Mux", SND_SOC_NOPM, 1, 0, &dac_mux),
    /* Pins for Simple Card Bindings */
    SND_SOC_DAPM_INPUT!("MICL"),
    SND_SOC_DAPM_INPUT!("MICR"),
    SND_SOC_DAPM_OUTPUT!("HPOL"),
    SND_SOC_DAPM_OUTPUT!("HPOR"),
    SND_SOC_DAPM_OUTPUT!("SPKO"),
];

static rk817_dapm_routes: [snd_soc_dapm_route; 91] = [
    /* capture path */
    /* left mic */
    route!("ADC L", ptr::null(), "LDO Regulator"),
    route!("ADC L", ptr::null(), "IBIAS Block"),
    route!("ADC L", ptr::null(), "VAvg Buffer"),
    route!("ADC L", ptr::null(), "PLL Power"),
    route!("ADC L", ptr::null(), "ADC Clock"),
    route!("ADC L", ptr::null(), "I2S TX Clock"),
    route!("ADC L", ptr::null(), "ADC Channel Enable"),
    route!("ADC L", ptr::null(), "I2S TX Channel Enable"),
    route!("ADC L", ptr::null(), "I2S TX1 Transfer Start"),
    route!("MICL", ptr::null(), "MIC Power On"),
    route!("MICL", ptr::null(), "PGA L Power On"),
    route!("MICL", ptr::null(), "Mic Boost L1"),
    route!("MICL", ptr::null(), "Mic Boost L2"),
    route!("MICL", ptr::null(), "I2S TX3 Transfer Start"),
    route!("MICL", ptr::null(), "I2S TX3 Right Justified"),
    route!("ADC L", ptr::null(), "MICL"),
    /* right mic */
    route!("ADC R", ptr::null(), "LDO Regulator"),
    route!("ADC R", ptr::null(), "IBIAS Block"),
    route!("ADC R", ptr::null(), "VAvg Buffer"),
    route!("ADC R", ptr::null(), "PLL Power"),
    route!("ADC R", ptr::null(), "ADC Clock"),
    route!("ADC R", ptr::null(), "I2S TX Clock"),
    route!("ADC R", ptr::null(), "ADC Channel Enable"),
    route!("ADC R", ptr::null(), "I2S TX Channel Enable"),
    route!("ADC R", ptr::null(), "I2S TX1 Transfer Start"),
    route!("MICR", ptr::null(), "MIC Power On"),
    route!("MICR", ptr::null(), "PGA R Power On"),
    route!("MICR", ptr::null(), "Mic Boost R1"),
    route!("MICR", ptr::null(), "Mic Boost R2"),
    route!("MICR", ptr::null(), "I2S TX3 Transfer Start"),
    route!("MICR", ptr::null(), "I2S TX3 Right Justified"),
    route!("ADC R", ptr::null(), "MICR"),
    /* playback path */
    /* speaker path */
    route!("SPK DAC", ptr::null(), "LDO Regulator"),
    route!("SPK DAC", ptr::null(), "IBIAS Block"),
    route!("SPK DAC", ptr::null(), "VAvg Buffer"),
    route!("SPK DAC", ptr::null(), "PLL Power"),
    route!("SPK DAC", ptr::null(), "I2S TX1 Transfer Start"),
    route!("SPK DAC", ptr::null(), "DAC Clock"),
    route!("SPK DAC", ptr::null(), "I2S RX Clock"),
    route!("SPK DAC", ptr::null(), "DAC Channel Enable"),
    route!("SPK DAC", ptr::null(), "I2S RX Channel Enable"),
    route!("SPK DAC", ptr::null(), "Class D Mode"),
    route!("SPK DAC", ptr::null(), "DAC Bias"),
    route!("SPK DAC", ptr::null(), "DAC Mute Off"),
    route!("SPK DAC", ptr::null(), "Enable Class D"),
    route!("SPK DAC", ptr::null(), "Disable Class D Mute Ramp"),
    route!("SPK DAC", ptr::null(), "Class D Mute Rate 1"),
    route!("SPK DAC", ptr::null(), "Class D Mute Rate 2"),
    route!("SPK DAC", ptr::null(), "Class D OCPP 2"),
    route!("SPK DAC", ptr::null(), "Class D OCPP 3"),
    route!("SPK DAC", ptr::null(), "Class D OCPN 2"),
    route!("SPK DAC", ptr::null(), "Class D OCPN 3"),
    route!("SPK DAC", ptr::null(), "High Pass Filter"),
    /* headphone path L */
    route!("DAC L", ptr::null(), "LDO Regulator"),
    route!("DAC L", ptr::null(), "IBIAS Block"),
    route!("DAC L", ptr::null(), "VAvg Buffer"),
    route!("DAC L", ptr::null(), "PLL Power"),
    route!("DAC L", ptr::null(), "I2S TX1 Transfer Start"),
    route!("DAC L", ptr::null(), "DAC Clock"),
    route!("DAC L", ptr::null(), "I2S RX Clock"),
    route!("DAC L", ptr::null(), "DAC Channel Enable"),
    route!("DAC L", ptr::null(), "I2S RX Channel Enable"),
    route!("DAC L", ptr::null(), "DAC Bias"),
    route!("DAC L", ptr::null(), "DAC Mute Off"),
    route!("DAC L", ptr::null(), "Headphone Charge Pump"),
    route!("DAC L", ptr::null(), "Headphone CP Discharge LDO"),
    route!("DAC L", ptr::null(), "Headphone OStage"),
    route!("DAC L", ptr::null(), "Headphone Pre Amp"),
    /* headphone path R */
    route!("DAC R", ptr::null(), "LDO Regulator"),
    route!("DAC R", ptr::null(), "IBIAS Block"),
    route!("DAC R", ptr::null(), "VAvg Buffer"),
    route!("DAC R", ptr::null(), "PLL Power"),
    route!("DAC R", ptr::null(), "I2S TX1 Transfer Start"),
    route!("DAC R", ptr::null(), "DAC Clock"),
    route!("DAC R", ptr::null(), "I2S RX Clock"),
    route!("DAC R", ptr::null(), "DAC Channel Enable"),
    route!("DAC R", ptr::null(), "I2S RX Channel Enable"),
    route!("DAC R", ptr::null(), "DAC Bias"),
    route!("DAC R", ptr::null(), "DAC Mute Off"),
    route!("DAC R", ptr::null(), "Headphone Charge Pump"),
    route!("DAC R", ptr::null(), "Headphone CP Discharge LDO"),
    route!("DAC R", ptr::null(), "Headphone OStage"),
    route!("DAC R", ptr::null(), "Headphone Pre Amp"),
    /* mux path for output selection */
    route!("Playback Mux", "HP", "DAC L"),
    route!("Playback Mux", "HP", "DAC R"),
    route!("Playback Mux", "SPK", "SPK DAC"),
    route!("SPKO", ptr::null(), "Playback Mux"),
    route!("HPOL", ptr::null(), "Playback Mux"),
    route!("HPOR", ptr::null(), "Playback Mux"),
];

macro_rules! route {
    ($sink:literal, ptr::null(), $source:literal) => {
        snd_soc_dapm_route {
            sink: cstr!($sink),
            control: ptr::null(),
            source: cstr!($source),
        }
    };
    ($sink:literal, $control:literal, $source:literal) => {
        snd_soc_dapm_route {
            sink: cstr!($sink),
            control: cstr!($control),
            source: cstr!($source),
        }
    };
}

unsafe extern "C" fn rk817_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = (*codec_dai).component;
    let rk817 = snd_soc_component_get_drvdata(component) as *mut rk817_codec_priv;

    (*rk817).stereo_sysclk = freq;

    0
}

unsafe extern "C" fn rk817_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let mut i2s_mst: c_uint = 0;

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        x if x == SND_SOC_DAIFMT_CBC_CFC => {
            i2s_mst |= RK817_I2S_MODE_SLV;
        }
        x if x == SND_SOC_DAIFMT_CBP_CFP => {
            i2s_mst |= RK817_I2S_MODE_MST;
        }
        _ => {
            dev_err((*component).dev, cstr!("%s : set master mask failed!\n"), cstr!("rk817_set_dai_fmt"));
            return -EINVAL;
        }
    }

    snd_soc_component_update_bits(component, RK817_CODEC_DI2S_CKM, RK817_I2S_MODE_MASK, i2s_mst);

    0
}

unsafe extern "C" fn rk817_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;

    match params_format(params) {
        x if x == SNDRV_PCM_FORMAT_S16_LE => {
            snd_soc_component_write(component, RK817_CODEC_DI2S_RXCR2, VDW_RX_16BITS);
            snd_soc_component_write(component, RK817_CODEC_DI2S_TXCR2, VDW_TX_16BITS);
        }
        x if x == SNDRV_PCM_FORMAT_S24_LE || x == SNDRV_PCM_FORMAT_S32_LE => {
            snd_soc_component_write(component, RK817_CODEC_DI2S_RXCR2, VDW_RX_24BITS);
            snd_soc_component_write(component, RK817_CODEC_DI2S_TXCR2, VDW_TX_24BITS);
        }
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn rk817_digital_mute(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _stream: c_int,
) -> c_int {
    let component = (*dai).component;

    if mute != 0 {
        snd_soc_component_update_bits(
            component,
            RK817_CODEC_DDAC_MUTE_MIXCTL,
            DACMT_MASK,
            DACMT_ENABLE,
        );
    } else {
        snd_soc_component_update_bits(
            component,
            RK817_CODEC_DDAC_MUTE_MIXCTL,
            DACMT_MASK,
            DACMT_DISABLE,
        );
    }

    0
}

fn RK817_PLAYBACK_RATES() -> c_uint {
    unsafe {
        SNDRV_PCM_RATE_8000
            | SNDRV_PCM_RATE_16000
            | SNDRV_PCM_RATE_32000
            | SNDRV_PCM_RATE_44100
            | SNDRV_PCM_RATE_48000
            | SNDRV_PCM_RATE_96000
    }
}

fn RK817_CAPTURE_RATES() -> c_uint {
    unsafe {
        SNDRV_PCM_RATE_8000
            | SNDRV_PCM_RATE_16000
            | SNDRV_PCM_RATE_32000
            | SNDRV_PCM_RATE_44100
            | SNDRV_PCM_RATE_48000
            | SNDRV_PCM_RATE_96000
    }
}

fn RK817_FORMATS() -> c_uint {
    unsafe {
        SNDRV_PCM_FMTBIT_S16_LE
            | SNDRV_PCM_FMTBIT_S20_3LE
            | SNDRV_PCM_FMTBIT_S24_LE
            | SNDRV_PCM_FMTBIT_S32_LE
    }
}

static rk817_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(rk817_hw_params),
    set_fmt: Some(rk817_set_dai_fmt),
    set_sysclk: Some(rk817_set_dai_sysclk),
    mute_stream: Some(rk817_digital_mute),
    no_capture_mute: 1,
};

static mut rk817_dai: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: cstr!("rk817-hifi"),
    playback: snd_soc_pcm_stream {
        stream_name: cstr!("Playback"),
        channels_min: 2,
        channels_max: 8,
        rates: RK817_PLAYBACK_RATES(),
        formats: RK817_FORMATS(),
    },
    capture: snd_soc_pcm_stream {
        stream_name: cstr!("Capture"),
        channels_min: 1,
        channels_max: 2,
        rates: RK817_CAPTURE_RATES(),
        formats: RK817_FORMATS(),
    },
    ops: &rk817_dai_ops,
}];

unsafe extern "C" fn rk817_probe(component: *mut snd_soc_component) -> c_int {
    let rk817 = snd_soc_component_get_drvdata(component) as *mut rk817_codec_priv;
    let rk808 = dev_get_drvdata((*(*component).dev).parent) as *mut rk808;

    snd_soc_component_init_regmap(component, (*rk808).regmap);
    (*rk817).component = component;

    snd_soc_component_write(component, RK817_CODEC_DTOP_LPT_SRST, 0x40);

    rk817_init(component);

    /* setting initial pll values so that we can continue to leverage simple-audio-card.
     * The values aren't important since no parameters are used.
     */
    snd_soc_component_set_pll(component, 0, 0, 0, 0);

    0
}

unsafe extern "C" fn rk817_remove(component: *mut snd_soc_component) {
    snd_soc_component_exit_regmap(component);
}

static soc_codec_dev_rk817: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(rk817_probe),
    remove: Some(rk817_remove),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
    controls: rk817_volume_controls.as_ptr(),
    num_controls: rk817_volume_controls.len() as c_uint,
    dapm_routes: rk817_dapm_routes.as_ptr(),
    num_dapm_routes: rk817_dapm_routes.len() as c_uint,
    dapm_widgets: rk817_dapm_widgets.as_ptr(),
    num_dapm_widgets: rk817_dapm_widgets.len() as c_uint,
    set_pll: Some(rk817_set_component_pll),
};

unsafe extern "C" fn rk817_codec_parse_dt_property(
    dev: *mut device,
    rk817: *mut rk817_codec_priv,
) {
    let mut node: *mut device_node;

    node = of_get_child_by_name((*(*dev).parent).of_node, cstr!("codec"));
    if node.is_null() {
        dev_dbg(dev, cstr!("%s() Can not get child: codec\n"), cstr!("rk817_codec_parse_dt_property"));
    }

    (*rk817).mic_in_differential =
        of_property_read_bool(node, cstr!("rockchip,mic-in-differential"));

    of_node_put(node);
}

unsafe extern "C" fn rk817_platform_probe(pdev: *mut platform_device) -> c_int {
    let rk808 = dev_get_drvdata((*pdev).dev.parent) as *mut rk808;
    let rk817_codec_data: *mut rk817_codec_priv;
    let mut ret: c_int;

    rk817_codec_data = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<rk817_codec_priv>(),
        GFP_KERNEL,
    ) as *mut rk817_codec_priv;
    if rk817_codec_data.is_null() {
        return -ENOMEM;
    }

    platform_set_drvdata(pdev, rk817_codec_data as *mut c_void);

    (*rk817_codec_data).rk808 = rk808;

    rk817_codec_parse_dt_property(&mut (*pdev).dev, rk817_codec_data);

    (*rk817_codec_data).mclk = devm_clk_get((*pdev).dev.parent, cstr!("mclk"));
    if IS_ERR((*rk817_codec_data).mclk as *const c_void) {
        dev_dbg(&mut (*pdev).dev, cstr!("Unable to get mclk\n"));
        ret = -ENXIO;
        return ret;
    }

    ret = clk_prepare_enable((*rk817_codec_data).mclk);
    if ret < 0 {
        dev_err(
            &mut (*pdev).dev,
            cstr!("%s() clock prepare error %d\n"),
            cstr!("rk817_platform_probe"),
            ret,
        );
        return ret;
    }

    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &soc_codec_dev_rk817,
        rk817_dai.as_mut_ptr(),
        rk817_dai.len() as c_int,
    );
    if ret < 0 {
        dev_err(
            &mut (*pdev).dev,
            cstr!("%s() register codec error %d\n"),
            cstr!("rk817_platform_probe"),
            ret,
        );
        clk_disable_unprepare((*rk817_codec_data).mclk);
        return ret;
    }

    0
}

unsafe extern "C" fn rk817_platform_remove(pdev: *mut platform_device) {
    let rk817 = platform_get_drvdata(pdev) as *mut rk817_codec_priv;

    clk_disable_unprepare((*rk817).mclk);
}

static mut rk817_codec_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: cstr!("rk817-codec"),
    },
    probe: Some(rk817_platform_probe),
    remove: Some(rk817_platform_remove),
};

module_platform_driver!(rk817_codec_driver);

MODULE_DESCRIPTION!("ASoC RK817 codec driver");
MODULE_AUTHOR!("binyuan <kevan.lan@rock-chips.com>");
MODULE_LICENSE!("GPL v2");
MODULE_ALIAS!("platform:rk817-codec");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
