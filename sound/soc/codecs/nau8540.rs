// SPDX-License-Identifier: GPL-2.0-only
/*
 * NAU85L40 ALSA SoC audio driver
 *
 * Copyright 2016 Nuvoton Technology Corp.
 * Author: John Hsu <KCHSU0@nuvoton.com>
 */

// Translated from Linux kernel C. Header-provided types, constants, and macros
// from linux/*, sound/*, and "nau8540.h" are expected as external dependencies.

use core::ffi::{c_char, c_int, c_uint, c_void};

const NAU_FREF_MAX: c_uint = 13500000;
const NAU_FVCO_MAX: u64 = 100000000;
const NAU_FVCO_MIN: u64 = 90000000;

/* the maximum frequency of CLK_ADC */
const CLK_ADC_MAX: c_uint = 6144000;

const NAU8540_RATES: c_uint = SNDRV_PCM_RATE_8000_48000;
const NAU8540_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE |
    SNDRV_PCM_FMTBIT_S24_3LE | SNDRV_PCM_FMTBIT_S32_LE;

extern "C" {
    fn regmap_update_bits(regmap: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_write(regmap: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(regmap: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn msleep(msecs: c_uint);
    fn mdelay(msecs: c_uint);
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_pcm_hw_constraint_minmax(runtime: *mut snd_pcm_runtime, var: c_uint, min: c_uint, max: c_uint) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

#[repr(C)]
pub struct regmap { _private: [u8; 0] }
#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct snd_kcontrol { _private: [u8; 0] }
#[repr(C)]
pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)]
pub struct snd_pcm_runtime { _private: [u8; 0] }
#[repr(C)]
pub struct snd_soc_dapm_context { _private: [u8; 0] }

#[repr(C)]
pub struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_soc_component { _private: [u8; 0] }

#[repr(C)]
pub struct snd_soc_dai {
    component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
pub struct i2c_client {
    dev: device,
}

#[repr(C)]
pub struct nau8540 {
    regmap: *mut regmap,
    dev: *mut device,
}

#[repr(C)]
pub struct nau8540_fll_attr {
    param: c_uint,
    val: c_uint,
}

#[repr(C)]
pub struct nau8540_osr_attr {
    osr: c_uint,
    clk_src: c_uint,
}

#[repr(C)]
pub struct nau8540_fll {
    mclk_src: c_uint,
    ratio: c_uint,
    fll_frac: c_uint,
    fll_int: c_uint,
    clk_ref_div: c_uint,
}

/* scaling for mclk from sysclk_src output */
static mclk_src_scaling: [nau8540_fll_attr; 10] = [
    nau8540_fll_attr { param: 1, val: 0x0 },
    nau8540_fll_attr { param: 2, val: 0x2 },
    nau8540_fll_attr { param: 4, val: 0x3 },
    nau8540_fll_attr { param: 8, val: 0x4 },
    nau8540_fll_attr { param: 16, val: 0x5 },
    nau8540_fll_attr { param: 32, val: 0x6 },
    nau8540_fll_attr { param: 3, val: 0x7 },
    nau8540_fll_attr { param: 6, val: 0xa },
    nau8540_fll_attr { param: 12, val: 0xb },
    nau8540_fll_attr { param: 24, val: 0xc },
];

/* ratio for input clk freq */
static fll_ratio: [nau8540_fll_attr; 7] = [
    nau8540_fll_attr { param: 512000, val: 0x01 },
    nau8540_fll_attr { param: 256000, val: 0x02 },
    nau8540_fll_attr { param: 128000, val: 0x04 },
    nau8540_fll_attr { param: 64000, val: 0x08 },
    nau8540_fll_attr { param: 32000, val: 0x10 },
    nau8540_fll_attr { param: 8000, val: 0x20 },
    nau8540_fll_attr { param: 4000, val: 0x40 },
];

static fll_pre_scalar: [nau8540_fll_attr; 4] = [
    nau8540_fll_attr { param: 1, val: 0x0 },
    nau8540_fll_attr { param: 2, val: 0x1 },
    nau8540_fll_attr { param: 4, val: 0x2 },
    nau8540_fll_attr { param: 8, val: 0x3 },
];

/* over sampling rate */
static osr_adc_sel: [nau8540_osr_attr; 4] = [
    nau8540_osr_attr { osr: 32, clk_src: 3 },  /* OSR 32, SRC 1/8 */
    nau8540_osr_attr { osr: 64, clk_src: 2 },  /* OSR 64, SRC 1/4 */
    nau8540_osr_attr { osr: 128, clk_src: 1 }, /* OSR 128, SRC 1/2 */
    nau8540_osr_attr { osr: 256, clk_src: 0 }, /* OSR 256, SRC 1 */
];

static nau8540_reg_defaults: [reg_default; 52] = [
    reg_default { reg: NAU8540_REG_POWER_MANAGEMENT, def: 0x0000 },
    reg_default { reg: NAU8540_REG_CLOCK_CTRL, def: 0x0000 },
    reg_default { reg: NAU8540_REG_CLOCK_SRC, def: 0x0000 },
    reg_default { reg: NAU8540_REG_FLL1, def: 0x0001 },
    reg_default { reg: NAU8540_REG_FLL2, def: 0x3126 },
    reg_default { reg: NAU8540_REG_FLL3, def: 0x0008 },
    reg_default { reg: NAU8540_REG_FLL4, def: 0x0010 },
    reg_default { reg: NAU8540_REG_FLL5, def: 0xC000 },
    reg_default { reg: NAU8540_REG_FLL6, def: 0x6000 },
    reg_default { reg: NAU8540_REG_FLL_VCO_RSV, def: 0xF13C },
    reg_default { reg: NAU8540_REG_PCM_CTRL0, def: 0x000B },
    reg_default { reg: NAU8540_REG_PCM_CTRL1, def: 0x3010 },
    reg_default { reg: NAU8540_REG_PCM_CTRL2, def: 0x0800 },
    reg_default { reg: NAU8540_REG_PCM_CTRL3, def: 0x0000 },
    reg_default { reg: NAU8540_REG_PCM_CTRL4, def: 0x000F },
    reg_default { reg: NAU8540_REG_ALC_CONTROL_1, def: 0x0000 },
    reg_default { reg: NAU8540_REG_ALC_CONTROL_2, def: 0x700B },
    reg_default { reg: NAU8540_REG_ALC_CONTROL_3, def: 0x0022 },
    reg_default { reg: NAU8540_REG_ALC_CONTROL_4, def: 0x1010 },
    reg_default { reg: NAU8540_REG_ALC_CONTROL_5, def: 0x1010 },
    reg_default { reg: NAU8540_REG_NOTCH_FIL1_CH1, def: 0x0000 },
    reg_default { reg: NAU8540_REG_NOTCH_FIL2_CH1, def: 0x0000 },
    reg_default { reg: NAU8540_REG_NOTCH_FIL1_CH2, def: 0x0000 },
    reg_default { reg: NAU8540_REG_NOTCH_FIL2_CH2, def: 0x0000 },
    reg_default { reg: NAU8540_REG_NOTCH_FIL1_CH3, def: 0x0000 },
    reg_default { reg: NAU8540_REG_NOTCH_FIL2_CH3, def: 0x0000 },
    reg_default { reg: NAU8540_REG_NOTCH_FIL1_CH4, def: 0x0000 },
    reg_default { reg: NAU8540_REG_NOTCH_FIL2_CH4, def: 0x0000 },
    reg_default { reg: NAU8540_REG_HPF_FILTER_CH12, def: 0x0000 },
    reg_default { reg: NAU8540_REG_HPF_FILTER_CH34, def: 0x0000 },
    reg_default { reg: NAU8540_REG_ADC_SAMPLE_RATE, def: 0x0002 },
    reg_default { reg: NAU8540_REG_DIGITAL_GAIN_CH1, def: 0x0400 },
    reg_default { reg: NAU8540_REG_DIGITAL_GAIN_CH2, def: 0x0400 },
    reg_default { reg: NAU8540_REG_DIGITAL_GAIN_CH3, def: 0x0400 },
    reg_default { reg: NAU8540_REG_DIGITAL_GAIN_CH4, def: 0x0400 },
    reg_default { reg: NAU8540_REG_DIGITAL_MUX, def: 0x00E4 },
    reg_default { reg: NAU8540_REG_GPIO_CTRL, def: 0x0000 },
    reg_default { reg: NAU8540_REG_MISC_CTRL, def: 0x0000 },
    reg_default { reg: NAU8540_REG_I2C_CTRL, def: 0xEFFF },
    reg_default { reg: NAU8540_REG_VMID_CTRL, def: 0x0000 },
    reg_default { reg: NAU8540_REG_MUTE, def: 0x0000 },
    reg_default { reg: NAU8540_REG_ANALOG_ADC1, def: 0x0011 },
    reg_default { reg: NAU8540_REG_ANALOG_ADC2, def: 0x0020 },
    reg_default { reg: NAU8540_REG_ANALOG_PWR, def: 0x0000 },
    reg_default { reg: NAU8540_REG_MIC_BIAS, def: 0x0004 },
    reg_default { reg: NAU8540_REG_REFERENCE, def: 0x0000 },
    reg_default { reg: NAU8540_REG_FEPGA1, def: 0x0000 },
    reg_default { reg: NAU8540_REG_FEPGA2, def: 0x0000 },
    reg_default { reg: NAU8540_REG_FEPGA3, def: 0x0101 },
    reg_default { reg: NAU8540_REG_FEPGA4, def: 0x0101 },
    reg_default { reg: NAU8540_REG_PWR, def: 0x0000 },
];

unsafe extern "C" fn nau8540_readable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        NAU8540_REG_POWER_MANAGEMENT..=NAU8540_REG_FLL_VCO_RSV |
        NAU8540_REG_PCM_CTRL0..=NAU8540_REG_PCM_CTRL4 |
        NAU8540_REG_ALC_CONTROL_1..=NAU8540_REG_ALC_CONTROL_5 |
        NAU8540_REG_ALC_GAIN_CH12..=NAU8540_REG_ADC_SAMPLE_RATE |
        NAU8540_REG_DIGITAL_GAIN_CH1..=NAU8540_REG_DIGITAL_MUX |
        NAU8540_REG_P2P_CH1..=NAU8540_REG_I2C_CTRL |
        NAU8540_REG_I2C_DEVICE_ID |
        NAU8540_REG_VMID_CTRL..=NAU8540_REG_MUTE |
        NAU8540_REG_ANALOG_ADC1..=NAU8540_REG_PWR => true,
        _ => false,
    }
}

unsafe extern "C" fn nau8540_writeable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        NAU8540_REG_SW_RESET..=NAU8540_REG_FLL_VCO_RSV |
        NAU8540_REG_PCM_CTRL0..=NAU8540_REG_PCM_CTRL4 |
        NAU8540_REG_ALC_CONTROL_1..=NAU8540_REG_ALC_CONTROL_5 |
        NAU8540_REG_NOTCH_FIL1_CH1..=NAU8540_REG_ADC_SAMPLE_RATE |
        NAU8540_REG_DIGITAL_GAIN_CH1..=NAU8540_REG_DIGITAL_MUX |
        NAU8540_REG_GPIO_CTRL..=NAU8540_REG_I2C_CTRL |
        NAU8540_REG_RST |
        NAU8540_REG_VMID_CTRL..=NAU8540_REG_MUTE |
        NAU8540_REG_ANALOG_ADC1..=NAU8540_REG_PWR => true,
        _ => false,
    }
}

unsafe extern "C" fn nau8540_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        NAU8540_REG_SW_RESET |
        NAU8540_REG_ALC_GAIN_CH12..=NAU8540_REG_ALC_STATUS |
        NAU8540_REG_P2P_CH1..=NAU8540_REG_PEAK_CH4 |
        NAU8540_REG_I2C_DEVICE_ID |
        NAU8540_REG_RST => true,
        _ => false,
    }
}

static adc_vol_tlv: [c_uint; 4] = DECLARE_TLV_DB_MINMAX!(-12800, 3600);
static fepga_gain_tlv: [c_uint; 4] = DECLARE_TLV_DB_MINMAX!(-100, 3600);

static nau8540_snd_controls: [snd_kcontrol_new; 8] = [
    SOC_SINGLE_TLV!("Mic1 Volume", NAU8540_REG_DIGITAL_GAIN_CH1, 0, 0x520, 0, adc_vol_tlv),
    SOC_SINGLE_TLV!("Mic2 Volume", NAU8540_REG_DIGITAL_GAIN_CH2, 0, 0x520, 0, adc_vol_tlv),
    SOC_SINGLE_TLV!("Mic3 Volume", NAU8540_REG_DIGITAL_GAIN_CH3, 0, 0x520, 0, adc_vol_tlv),
    SOC_SINGLE_TLV!("Mic4 Volume", NAU8540_REG_DIGITAL_GAIN_CH4, 0, 0x520, 0, adc_vol_tlv),
    SOC_SINGLE_TLV!("Frontend PGA1 Volume", NAU8540_REG_FEPGA3, 0, 0x25, 0, fepga_gain_tlv),
    SOC_SINGLE_TLV!("Frontend PGA2 Volume", NAU8540_REG_FEPGA3, 8, 0x25, 0, fepga_gain_tlv),
    SOC_SINGLE_TLV!("Frontend PGA3 Volume", NAU8540_REG_FEPGA4, 0, 0x25, 0, fepga_gain_tlv),
    SOC_SINGLE_TLV!("Frontend PGA4 Volume", NAU8540_REG_FEPGA4, 8, 0x25, 0, fepga_gain_tlv),
];

static adc_channel: [*const c_char; 4] = [
    c"ADC channel 1".as_ptr(),
    c"ADC channel 2".as_ptr(),
    c"ADC channel 3".as_ptr(),
    c"ADC channel 4".as_ptr(),
];

static digital_ch4_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(NAU8540_REG_DIGITAL_MUX, 6, adc_channel);
static digital_ch4_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("Digital CH4 Select", digital_ch4_enum);
static digital_ch3_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(NAU8540_REG_DIGITAL_MUX, 4, adc_channel);
static digital_ch3_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("Digital CH3 Select", digital_ch3_enum);
static digital_ch2_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(NAU8540_REG_DIGITAL_MUX, 2, adc_channel);
static digital_ch2_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("Digital CH2 Select", digital_ch2_enum);
static digital_ch1_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(NAU8540_REG_DIGITAL_MUX, 0, adc_channel);
static digital_ch1_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("Digital CH1 Select", digital_ch1_enum);

unsafe extern "C" fn nau8540_fepga_event(w: *mut snd_soc_dapm_widget, _k: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let nau8540 = snd_soc_component_get_drvdata(component) as *mut nau8540;

    match event {
        SND_SOC_DAPM_POST_PMU => {
            regmap_update_bits((*nau8540).regmap, NAU8540_REG_FEPGA2,
                NAU8540_ACDC_CTL_MASK,
                NAU8540_ACDC_CTL_MIC1P_VREF |
                NAU8540_ACDC_CTL_MIC1N_VREF | NAU8540_ACDC_CTL_MIC2P_VREF |
                NAU8540_ACDC_CTL_MIC2N_VREF | NAU8540_ACDC_CTL_MIC3P_VREF |
                NAU8540_ACDC_CTL_MIC3N_VREF | NAU8540_ACDC_CTL_MIC4P_VREF |
                NAU8540_ACDC_CTL_MIC4N_VREF);
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn nau8540_precharge_event(w: *mut snd_soc_dapm_widget, _k: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let nau8540 = snd_soc_component_get_drvdata(component) as *mut nau8540;

    match event {
        SND_SOC_DAPM_POST_PMU => {
            regmap_update_bits((*nau8540).regmap, NAU8540_REG_REFERENCE,
                NAU8540_DISCHRG_EN, NAU8540_DISCHRG_EN);
            msleep(40);
            regmap_update_bits((*nau8540).regmap, NAU8540_REG_REFERENCE,
                NAU8540_DISCHRG_EN, 0);
            regmap_update_bits((*nau8540).regmap, NAU8540_REG_FEPGA2,
                NAU8540_ACDC_CTL_MASK, 0);
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn adc_power_control(w: *mut snd_soc_dapm_widget, _k: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let nau8540 = snd_soc_component_get_drvdata(component) as *mut nau8540;

    if SND_SOC_DAPM_EVENT_ON!(event) {
        msleep(160);
        /* DO12 and DO34 pad output enable */
        regmap_update_bits((*nau8540).regmap, NAU8540_REG_POWER_MANAGEMENT,
            NAU8540_ADC_ALL_EN, NAU8540_ADC_ALL_EN);
        regmap_update_bits((*nau8540).regmap, NAU8540_REG_PCM_CTRL1,
            NAU8540_I2S_DO12_TRI, 0);
        regmap_update_bits((*nau8540).regmap, NAU8540_REG_PCM_CTRL2,
            NAU8540_I2S_DO34_TRI, 0);
    } else if SND_SOC_DAPM_EVENT_OFF!(event) {
        regmap_update_bits((*nau8540).regmap, NAU8540_REG_PCM_CTRL1,
            NAU8540_I2S_DO12_TRI, NAU8540_I2S_DO12_TRI);
        regmap_update_bits((*nau8540).regmap, NAU8540_REG_PCM_CTRL2,
            NAU8540_I2S_DO34_TRI, NAU8540_I2S_DO34_TRI);
        regmap_update_bits((*nau8540).regmap, NAU8540_REG_POWER_MANAGEMENT,
            NAU8540_ADC_ALL_EN, 0);
    }
    0
}

unsafe extern "C" fn aiftx_power_control(w: *mut snd_soc_dapm_widget, _k: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let nau8540 = snd_soc_component_get_drvdata(component) as *mut nau8540;

    if SND_SOC_DAPM_EVENT_OFF!(event) {
        regmap_write((*nau8540).regmap, NAU8540_REG_RST, 0x0001);
        regmap_write((*nau8540).regmap, NAU8540_REG_RST, 0x0000);
    }
    0
}

static nau8540_dapm_widgets: [snd_soc_dapm_widget; 22] = [
    SND_SOC_DAPM_SUPPLY!("MICBIAS2", NAU8540_REG_MIC_BIAS, 11, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("MICBIAS1", NAU8540_REG_MIC_BIAS, 10, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_INPUT!("MIC1"),
    SND_SOC_DAPM_INPUT!("MIC2"),
    SND_SOC_DAPM_INPUT!("MIC3"),
    SND_SOC_DAPM_INPUT!("MIC4"),
    SND_SOC_DAPM_PGA_S!("Frontend PGA1", 0, NAU8540_REG_PWR, 12, 0, nau8540_fepga_event, SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_PGA_S!("Frontend PGA2", 0, NAU8540_REG_PWR, 13, 0, nau8540_fepga_event, SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_PGA_S!("Frontend PGA3", 0, NAU8540_REG_PWR, 14, 0, nau8540_fepga_event, SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_PGA_S!("Frontend PGA4", 0, NAU8540_REG_PWR, 15, 0, nau8540_fepga_event, SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_PGA_S!("Precharge", 1, SND_SOC_NOPM, 0, 0, nau8540_precharge_event, SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_PGA_S!("ADC CH1", 2, NAU8540_REG_ANALOG_PWR, 0, 0, adc_power_control, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_PGA_S!("ADC CH2", 2, NAU8540_REG_ANALOG_PWR, 1, 0, adc_power_control, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_PGA_S!("ADC CH3", 2, NAU8540_REG_ANALOG_PWR, 2, 0, adc_power_control, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_PGA_S!("ADC CH4", 2, NAU8540_REG_ANALOG_PWR, 3, 0, adc_power_control, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_MUX!("Digital CH4 Mux", SND_SOC_NOPM, 0, 0, &digital_ch4_mux),
    SND_SOC_DAPM_MUX!("Digital CH3 Mux", SND_SOC_NOPM, 0, 0, &digital_ch3_mux),
    SND_SOC_DAPM_MUX!("Digital CH2 Mux", SND_SOC_NOPM, 0, 0, &digital_ch2_mux),
    SND_SOC_DAPM_MUX!("Digital CH1 Mux", SND_SOC_NOPM, 0, 0, &digital_ch1_mux),
    SND_SOC_DAPM_AIF_OUT_E!("AIFTX", "Capture", 0, SND_SOC_NOPM, 0, 0, aiftx_power_control, SND_SOC_DAPM_POST_PMD),
];

static nau8540_dapm_routes: [snd_soc_dapm_route; 36] = [
    snd_soc_dapm_route { sink: c"Frontend PGA1".as_ptr(), control: core::ptr::null(), source: c"MIC1".as_ptr() },
    snd_soc_dapm_route { sink: c"Frontend PGA2".as_ptr(), control: core::ptr::null(), source: c"MIC2".as_ptr() },
    snd_soc_dapm_route { sink: c"Frontend PGA3".as_ptr(), control: core::ptr::null(), source: c"MIC3".as_ptr() },
    snd_soc_dapm_route { sink: c"Frontend PGA4".as_ptr(), control: core::ptr::null(), source: c"MIC4".as_ptr() },
    snd_soc_dapm_route { sink: c"Precharge".as_ptr(), control: core::ptr::null(), source: c"Frontend PGA1".as_ptr() },
    snd_soc_dapm_route { sink: c"Precharge".as_ptr(), control: core::ptr::null(), source: c"Frontend PGA2".as_ptr() },
    snd_soc_dapm_route { sink: c"Precharge".as_ptr(), control: core::ptr::null(), source: c"Frontend PGA3".as_ptr() },
    snd_soc_dapm_route { sink: c"Precharge".as_ptr(), control: core::ptr::null(), source: c"Frontend PGA4".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC CH1".as_ptr(), control: core::ptr::null(), source: c"Precharge".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC CH2".as_ptr(), control: core::ptr::null(), source: c"Precharge".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC CH3".as_ptr(), control: core::ptr::null(), source: c"Precharge".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC CH4".as_ptr(), control: core::ptr::null(), source: c"Precharge".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC CH1".as_ptr(), control: core::ptr::null(), source: c"MICBIAS1".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC CH2".as_ptr(), control: core::ptr::null(), source: c"MICBIAS1".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC CH3".as_ptr(), control: core::ptr::null(), source: c"MICBIAS2".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC CH4".as_ptr(), control: core::ptr::null(), source: c"MICBIAS2".as_ptr() },
    snd_soc_dapm_route { sink: c"Digital CH1 Mux".as_ptr(), control: c"ADC channel 1".as_ptr(), source: c"ADC CH1".as_ptr() },
    snd_soc_dapm_route { sink: c"Digital CH1 Mux".as_ptr(), control: c"ADC channel 2".as_ptr(), source: c"ADC CH2".as_ptr() },
    snd_soc_dapm_route { sink: c"Digital CH1 Mux".as_ptr(), control: c"ADC channel 3".as_ptr(), source: c"ADC CH3".as_ptr() },
    snd_soc_dapm_route { sink: c"Digital CH1 Mux".as_ptr(), control: c"ADC channel 4".as_ptr(), source: c"ADC CH4".as_ptr() },
    snd_soc_dapm_route { sink: c"Digital CH2 Mux".as_ptr(), control: c"ADC channel 1".as_ptr(), source: c"ADC CH1".as_ptr() },
    snd_soc_dapm_route { sink: c"Digital CH2 Mux".as_ptr(), control: c"ADC channel 2".as_ptr(), source: c"ADC CH2".as_ptr() },
    snd_soc_dapm_route { sink: c"Digital CH2 Mux".as_ptr(), control: c"ADC channel 3".as_ptr(), source: c"ADC CH3".as_ptr() },
    snd_soc_dapm_route { sink: c"Digital CH2 Mux".as_ptr(), control: c"ADC channel 4".as_ptr(), source: c"ADC CH4".as_ptr() },
    snd_soc_dapm_route { sink: c"Digital CH3 Mux".as_ptr(), control: c"ADC channel 1".as_ptr(), source: c"ADC CH1".as_ptr() },
    snd_soc_dapm_route { sink: c"Digital CH3 Mux".as_ptr(), control: c"ADC channel 2".as_ptr(), source: c"ADC CH2".as_ptr() },
    snd_soc_dapm_route { sink: c"Digital CH3 Mux".as_ptr(), control: c"ADC channel 3".as_ptr(), source: c"ADC CH3".as_ptr() },
    snd_soc_dapm_route { sink: c"Digital CH3 Mux".as_ptr(), control: c"ADC channel 4".as_ptr(), source: c"ADC CH4".as_ptr() },
    snd_soc_dapm_route { sink: c"Digital CH4 Mux".as_ptr(), control: c"ADC channel 1".as_ptr(), source: c"ADC CH1".as_ptr() },
    snd_soc_dapm_route { sink: c"Digital CH4 Mux".as_ptr(), control: c"ADC channel 2".as_ptr(), source: c"ADC CH2".as_ptr() },
    snd_soc_dapm_route { sink: c"Digital CH4 Mux".as_ptr(), control: c"ADC channel 3".as_ptr(), source: c"ADC CH3".as_ptr() },
    snd_soc_dapm_route { sink: c"Digital CH4 Mux".as_ptr(), control: c"ADC channel 4".as_ptr(), source: c"ADC CH4".as_ptr() },
    snd_soc_dapm_route { sink: c"AIFTX".as_ptr(), control: core::ptr::null(), source: c"Digital CH1 Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"AIFTX".as_ptr(), control: core::ptr::null(), source: c"Digital CH2 Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"AIFTX".as_ptr(), control: core::ptr::null(), source: c"Digital CH3 Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"AIFTX".as_ptr(), control: core::ptr::null(), source: c"Digital CH4 Mux".as_ptr() },
];

unsafe fn nau8540_get_osr(nau8540: *mut nau8540) -> *const nau8540_osr_attr {
    let mut osr: c_uint = 0;

    regmap_read((*nau8540).regmap, NAU8540_REG_ADC_SAMPLE_RATE, &mut osr);
    osr &= NAU8540_ADC_OSR_MASK;
    if osr as usize >= osr_adc_sel.len() {
        return core::ptr::null();
    }
    &osr_adc_sel[osr as usize]
}

unsafe extern "C" fn nau8540_dai_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let nau8540 = snd_soc_component_get_drvdata(component) as *mut nau8540;
    let osr = nau8540_get_osr(nau8540);

    if osr.is_null() || (*osr).osr == 0 {
        return -EINVAL;
    }

    snd_pcm_hw_constraint_minmax((*substream).runtime,
        SNDRV_PCM_HW_PARAM_RATE, 0, CLK_ADC_MAX / (*osr).osr)
}

unsafe extern "C" fn nau8540_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let nau8540 = snd_soc_component_get_drvdata(component) as *mut nau8540;
    let mut val_len: c_uint = 0;

    /* CLK_ADC = OSR * FS
     * ADC clock frequency is defined as Over Sampling Rate (OSR)
     * multiplied by the audio sample rate (Fs). Note that the OSR and Fs
     * values must be selected such that the maximum frequency is less
     * than 6.144 MHz.
     */
    let osr = nau8540_get_osr(nau8540);
    if osr.is_null() || (*osr).osr == 0 {
        return -EINVAL;
    }
    if params_rate(params).wrapping_mul((*osr).osr) > CLK_ADC_MAX {
        return -EINVAL;
    }
    regmap_update_bits((*nau8540).regmap, NAU8540_REG_CLOCK_SRC,
        NAU8540_CLK_ADC_SRC_MASK,
        (*osr).clk_src << NAU8540_CLK_ADC_SRC_SFT);

    match params_width(params) {
        16 => val_len |= NAU8540_I2S_DL_16,
        20 => val_len |= NAU8540_I2S_DL_20,
        24 => val_len |= NAU8540_I2S_DL_24,
        32 => val_len |= NAU8540_I2S_DL_32,
        _ => return -EINVAL,
    }

    regmap_update_bits((*nau8540).regmap, NAU8540_REG_PCM_CTRL0,
        NAU8540_I2S_DL_MASK, val_len);

    0
}

unsafe extern "C" fn nau8540_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let nau8540 = snd_soc_component_get_drvdata(component) as *mut nau8540;
    let mut ctrl1_val: c_uint = 0;
    let mut ctrl2_val: c_uint = 0;

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => ctrl2_val |= NAU8540_I2S_MS_MASTER,
        SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_NF => ctrl1_val |= NAU8540_I2S_BP_INV,
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => ctrl1_val |= NAU8540_I2S_DF_I2S,
        SND_SOC_DAIFMT_LEFT_J => ctrl1_val |= NAU8540_I2S_DF_LEFT,
        SND_SOC_DAIFMT_RIGHT_J => ctrl1_val |= NAU8540_I2S_DF_RIGTH,
        SND_SOC_DAIFMT_DSP_A => ctrl1_val |= NAU8540_I2S_DF_PCM_AB,
        SND_SOC_DAIFMT_DSP_B => {
            ctrl1_val |= NAU8540_I2S_DF_PCM_AB;
            ctrl1_val |= NAU8540_I2S_PCMB_EN;
        }
        _ => return -EINVAL,
    }

    regmap_update_bits((*nau8540).regmap, NAU8540_REG_PCM_CTRL0,
        NAU8540_I2S_DL_MASK | NAU8540_I2S_DF_MASK |
        NAU8540_I2S_BP_INV | NAU8540_I2S_PCMB_EN, ctrl1_val);
    regmap_update_bits((*nau8540).regmap, NAU8540_REG_PCM_CTRL1,
        NAU8540_I2S_MS_MASK | NAU8540_I2S_DO12_OE, ctrl2_val);
    regmap_update_bits((*nau8540).regmap, NAU8540_REG_PCM_CTRL2,
        NAU8540_I2S_DO34_OE, 0);

    0
}

/**
 * nau8540_set_tdm_slot - configure DAI TX TDM.
 * @dai: DAI
 * @tx_mask: bitmask representing active TX slots. Ex.
 *                 0xf for normal 4 channel TDM.
 *                 0xf0 for shifted 4 channel TDM
 * @rx_mask: no used.
 * @slots: Number of slots in use.
 * @slot_width: Width in bits for each slot.
 *
 * Configures a DAI for TDM operation. Only support 4 slots TDM.
 */
unsafe extern "C" fn nau8540_set_tdm_slot(
    dai: *mut snd_soc_dai,
    tx_mask: c_uint,
    _rx_mask: c_uint,
    slots: c_int,
    slot_width: c_int,
) -> c_int {
    let component = (*dai).component;
    let nau8540 = snd_soc_component_get_drvdata(component) as *mut nau8540;
    let mut ctrl2_val: c_uint = 0;
    let mut ctrl4_val: c_uint = 0;

    if slots > 4 || ((tx_mask & 0xf0) != 0 && (tx_mask & 0xf) != 0) {
        return -EINVAL;
    }

    ctrl4_val |= NAU8540_TDM_MODE | NAU8540_TDM_OFFSET_EN;
    if (tx_mask & 0xf0) != 0 {
        ctrl2_val = (4 * slot_width) as c_uint;
        ctrl4_val |= tx_mask >> 4;
    } else {
        ctrl4_val |= tx_mask;
    }
    regmap_update_bits((*nau8540).regmap, NAU8540_REG_PCM_CTRL4,
        NAU8540_TDM_MODE | NAU8540_TDM_OFFSET_EN |
        NAU8540_TDM_TX_MASK, ctrl4_val);
    regmap_update_bits((*nau8540).regmap, NAU8540_REG_PCM_CTRL1,
        NAU8540_I2S_DO12_OE, NAU8540_I2S_DO12_OE);
    regmap_update_bits((*nau8540).regmap, NAU8540_REG_PCM_CTRL2,
        NAU8540_I2S_DO34_OE | NAU8540_I2S_TSLOT_L_MASK,
        NAU8540_I2S_DO34_OE | ctrl2_val);

    0
}

unsafe extern "C" fn nau8540_dai_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let nau8540 = snd_soc_component_get_drvdata(component) as *mut nau8540;
    let regmap = (*nau8540).regmap;
    let mut val: c_uint = 0;
    let mut ret: c_int = 0;

    /* Reading the peak data to detect abnormal data in the ADC channel.
     * If abnormal data happens, the driver takes recovery actions to
     * refresh the ADC channel.
     */
    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            regmap_update_bits(regmap, NAU8540_REG_CLOCK_CTRL,
                NAU8540_CLK_AGC_EN, NAU8540_CLK_AGC_EN);
            regmap_update_bits(regmap, NAU8540_REG_ALC_CONTROL_3,
                NAU8540_ALC_CH_ALL_EN, NAU8540_ALC_CH_ALL_EN);

            regmap_read(regmap, NAU8540_REG_PEAK_CH1, &mut val);
            dev_dbg!((*nau8540).dev, "1.ADC CH1 peak data %x", val);
            if val == 0 {
                regmap_update_bits(regmap, NAU8540_REG_MUTE,
                    NAU8540_PGA_CH_ALL_MUTE, NAU8540_PGA_CH_ALL_MUTE);
                regmap_update_bits(regmap, NAU8540_REG_MUTE,
                    NAU8540_PGA_CH_ALL_MUTE, 0);
                regmap_write(regmap, NAU8540_REG_RST, 0x1);
                regmap_write(regmap, NAU8540_REG_RST, 0);
                regmap_read(regmap, NAU8540_REG_PEAK_CH1, &mut val);
                dev_dbg!((*nau8540).dev, "2.ADC CH1 peak data %x", val);
                if val == 0 {
                    dev_err!((*nau8540).dev, "Channel recovery failed!!");
                    ret = -EIO;
                }
            }
            regmap_update_bits(regmap, NAU8540_REG_CLOCK_CTRL,
                NAU8540_CLK_AGC_EN, 0);
            regmap_update_bits(regmap, NAU8540_REG_ALC_CONTROL_3,
                NAU8540_ALC_CH_ALL_EN, 0);
        }
        _ => {}
    }

    ret
}

static nau8540_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(nau8540_dai_startup),
    hw_params: Some(nau8540_hw_params),
    set_fmt: Some(nau8540_set_fmt),
    set_tdm_slot: Some(nau8540_set_tdm_slot),
    trigger: Some(nau8540_dai_trigger),
};

static mut nau8540_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"nau8540-hifi".as_ptr(),
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 1,
        channels_max: 4,
        rates: NAU8540_RATES,
        formats: NAU8540_FORMATS,
    },
    ops: &nau8540_dai_ops,
};

/**
 * nau8540_calc_fll_param - Calculate FLL parameters.
 * @fll_in: external clock provided to codec.
 * @fs: sampling rate.
 * @fll_param: Pointer to structure of FLL parameters.
 *
 * Calculate FLL parameters to configure codec.
 *
 * Returns 0 for success or negative error code.
 */
unsafe fn nau8540_calc_fll_param(fll_in: c_uint, fs: c_uint, fll_param: *mut nau8540_fll) -> c_int {
    let mut fref: c_uint = 0;
    let mut fvco: u64;
    let mut fvco_max: u64;
    let mut fvco_sel: usize;

    /* Ensure the reference clock frequency (FREF) is <= 13.5MHz by dividing
     * freq_in by 1, 2, 4, or 8 using FLL pre-scalar.
     * FREF = freq_in / NAU8540_FLL_REF_DIV_MASK
     */
    let mut i = 0usize;
    while i < fll_pre_scalar.len() {
        fref = fll_in / fll_pre_scalar[i].param;
        if fref <= NAU_FREF_MAX {
            break;
        }
        i += 1;
    }
    if i == fll_pre_scalar.len() {
        return -EINVAL;
    }
    (*fll_param).clk_ref_div = fll_pre_scalar[i].val;

    /* Choose the FLL ratio based on FREF */
    i = 0;
    while i < fll_ratio.len() {
        if fref >= fll_ratio[i].param {
            break;
        }
        i += 1;
    }
    if i == fll_ratio.len() {
        return -EINVAL;
    }
    (*fll_param).ratio = fll_ratio[i].val;

    /* Calculate the frequency of DCO (FDCO) given freq_out = 256 * Fs.
     * FDCO must be within the 90MHz - 124MHz or the FFL cannot be
     * guaranteed across the full range of operation.
     * FDCO = freq_out * 2 * mclk_src_scaling
     */
    fvco_max = 0;
    fvco_sel = mclk_src_scaling.len();
    i = 0;
    while i < mclk_src_scaling.len() {
        fvco = 256u64
            .wrapping_mul(fs as u64)
            .wrapping_mul(2)
            .wrapping_mul(mclk_src_scaling[i].param as u64);
        if fvco > NAU_FVCO_MIN && fvco < NAU_FVCO_MAX && fvco_max < fvco {
            fvco_max = fvco;
            fvco_sel = i;
        }
        i += 1;
    }
    if mclk_src_scaling.len() == fvco_sel {
        return -EINVAL;
    }
    (*fll_param).mclk_src = mclk_src_scaling[fvco_sel].val;

    /* Calculate the FLL 10-bit integer input and the FLL 16-bit fractional
     * input based on FDCO, FREF and FLL ratio.
     */
    fvco = (fvco_max << 16) / (fref as u64 * (*fll_param).ratio as u64);
    (*fll_param).fll_int = ((fvco >> 16) & 0x3FF) as c_uint;
    (*fll_param).fll_frac = (fvco & 0xFFFF) as c_uint;
    0
}

unsafe fn nau8540_fll_apply(regmap: *mut regmap, fll_param: *mut nau8540_fll) {
    regmap_update_bits(regmap, NAU8540_REG_CLOCK_SRC,
        NAU8540_CLK_SRC_MASK | NAU8540_CLK_MCLK_SRC_MASK,
        NAU8540_CLK_SRC_MCLK | (*fll_param).mclk_src);
    regmap_update_bits(regmap, NAU8540_REG_FLL1,
        NAU8540_FLL_RATIO_MASK | NAU8540_ICTRL_LATCH_MASK,
        (*fll_param).ratio | (0x6 << NAU8540_ICTRL_LATCH_SFT));
    /* FLL 16-bit fractional input */
    regmap_write(regmap, NAU8540_REG_FLL2, (*fll_param).fll_frac);
    /* FLL 10-bit integer input */
    regmap_update_bits(regmap, NAU8540_REG_FLL3,
        NAU8540_FLL_INTEGER_MASK, (*fll_param).fll_int);
    /* FLL pre-scaler */
    regmap_update_bits(regmap, NAU8540_REG_FLL4,
        NAU8540_FLL_REF_DIV_MASK,
        (*fll_param).clk_ref_div << NAU8540_FLL_REF_DIV_SFT);
    regmap_update_bits(regmap, NAU8540_REG_FLL5,
        NAU8540_FLL_CLK_SW_MASK, NAU8540_FLL_CLK_SW_REF);
    regmap_update_bits(regmap, NAU8540_REG_FLL6, NAU8540_DCO_EN, 0);
    if (*fll_param).fll_frac != 0 {
        regmap_update_bits(regmap, NAU8540_REG_FLL5,
            NAU8540_FLL_PDB_DAC_EN | NAU8540_FLL_LOOP_FTR_EN |
            NAU8540_FLL_FTR_SW_MASK,
            NAU8540_FLL_PDB_DAC_EN | NAU8540_FLL_LOOP_FTR_EN |
            NAU8540_FLL_FTR_SW_FILTER);
        regmap_update_bits(regmap, NAU8540_REG_FLL6,
            NAU8540_SDM_EN | NAU8540_CUTOFF500,
            NAU8540_SDM_EN | NAU8540_CUTOFF500);
    } else {
        regmap_update_bits(regmap, NAU8540_REG_FLL5,
            NAU8540_FLL_PDB_DAC_EN | NAU8540_FLL_LOOP_FTR_EN |
            NAU8540_FLL_FTR_SW_MASK, NAU8540_FLL_FTR_SW_ACCU);
        regmap_update_bits(regmap, NAU8540_REG_FLL6,
            NAU8540_SDM_EN | NAU8540_CUTOFF500, 0);
    }
}

/* freq_out must be 256*Fs in order to achieve the best performance */
unsafe extern "C" fn nau8540_set_pll(
    component: *mut snd_soc_component,
    pll_id: c_int,
    _source: c_int,
    freq_in: c_uint,
    freq_out: c_uint,
) -> c_int {
    let nau8540 = snd_soc_component_get_drvdata(component) as *mut nau8540;
    let mut fll_param = core::mem::MaybeUninit::<nau8540_fll>::zeroed().assume_init();
    let ret: c_int;
    let fs: c_int;

    match pll_id {
        NAU8540_CLK_FLL_MCLK => {
            regmap_update_bits((*nau8540).regmap, NAU8540_REG_FLL3,
                NAU8540_FLL_CLK_SRC_MASK | NAU8540_GAIN_ERR_MASK,
                NAU8540_FLL_CLK_SRC_MCLK | 0);
        }
        NAU8540_CLK_FLL_BLK => {
            regmap_update_bits((*nau8540).regmap, NAU8540_REG_FLL3,
                NAU8540_FLL_CLK_SRC_MASK | NAU8540_GAIN_ERR_MASK,
                NAU8540_FLL_CLK_SRC_BLK | (0xf << NAU8540_GAIN_ERR_SFT));
        }
        NAU8540_CLK_FLL_FS => {
            regmap_update_bits((*nau8540).regmap, NAU8540_REG_FLL3,
                NAU8540_FLL_CLK_SRC_MASK | NAU8540_GAIN_ERR_MASK,
                NAU8540_FLL_CLK_SRC_FS | (0xf << NAU8540_GAIN_ERR_SFT));
        }
        _ => {
            dev_err!((*nau8540).dev, "Invalid clock id (%d)\n", pll_id);
            return -EINVAL;
        }
    }
    dev_dbg!((*nau8540).dev, "Sysclk is %dHz and clock id is %d\n", freq_out, pll_id);

    fs = (freq_out / 256) as c_int;
    ret = nau8540_calc_fll_param(freq_in, fs as c_uint, &mut fll_param);
    if ret < 0 {
        dev_err!((*nau8540).dev, "Unsupported input clock %d\n", freq_in);
        return ret;
    }
    dev_dbg!((*nau8540).dev,
        "mclk_src=%x ratio=%x fll_frac=%x fll_int=%x clk_ref_div=%x\n",
        fll_param.mclk_src, fll_param.ratio, fll_param.fll_frac,
        fll_param.fll_int, fll_param.clk_ref_div);

    nau8540_fll_apply((*nau8540).regmap, &mut fll_param);
    mdelay(2);
    regmap_update_bits((*nau8540).regmap, NAU8540_REG_CLOCK_SRC,
        NAU8540_CLK_SRC_MASK, NAU8540_CLK_SRC_VCO);

    0
}

unsafe extern "C" fn nau8540_set_sysclk(
    component: *mut snd_soc_component,
    clk_id: c_int,
    _source: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let nau8540 = snd_soc_component_get_drvdata(component) as *mut nau8540;

    match clk_id {
        NAU8540_CLK_DIS | NAU8540_CLK_MCLK => {
            regmap_update_bits((*nau8540).regmap, NAU8540_REG_CLOCK_SRC,
                NAU8540_CLK_SRC_MASK, NAU8540_CLK_SRC_MCLK);
            regmap_update_bits((*nau8540).regmap, NAU8540_REG_FLL6,
                NAU8540_DCO_EN, 0);
        }
        NAU8540_CLK_INTERNAL => {
            regmap_update_bits((*nau8540).regmap, NAU8540_REG_FLL6,
                NAU8540_DCO_EN, NAU8540_DCO_EN);
            regmap_update_bits((*nau8540).regmap, NAU8540_REG_CLOCK_SRC,
                NAU8540_CLK_SRC_MASK, NAU8540_CLK_SRC_VCO);
        }
        _ => {
            dev_err!((*nau8540).dev, "Invalid clock id (%d)\n", clk_id);
            return -EINVAL;
        }
    }

    dev_dbg!((*nau8540).dev, "Sysclk is %dHz and clock id is %d\n", freq, clk_id);

    0
}

unsafe fn nau8540_reset_chip(regmap: *mut regmap) {
    regmap_write(regmap, NAU8540_REG_SW_RESET, 0x00);
    regmap_write(regmap, NAU8540_REG_SW_RESET, 0x00);
}

unsafe fn nau8540_init_regs(nau8540: *mut nau8540) {
    let regmap = (*nau8540).regmap;

    /* Enable Bias/VMID/VMID Tieoff */
    regmap_update_bits(regmap, NAU8540_REG_VMID_CTRL,
        NAU8540_VMID_EN | NAU8540_VMID_SEL_MASK,
        NAU8540_VMID_EN | (0x2 << NAU8540_VMID_SEL_SFT));
    regmap_update_bits(regmap, NAU8540_REG_REFERENCE,
        NAU8540_PRECHARGE_DIS | NAU8540_GLOBAL_BIAS_EN,
        NAU8540_PRECHARGE_DIS | NAU8540_GLOBAL_BIAS_EN);
    mdelay(2);
    regmap_update_bits(regmap, NAU8540_REG_MIC_BIAS,
        NAU8540_PU_PRE, NAU8540_PU_PRE);
    regmap_update_bits(regmap, NAU8540_REG_CLOCK_CTRL,
        NAU8540_CLK_ADC_EN | NAU8540_CLK_I2S_EN,
        NAU8540_CLK_ADC_EN | NAU8540_CLK_I2S_EN);
    /* ADC OSR selection, CLK_ADC = Fs * OSR;
     * Channel time alignment enable.
     */
    regmap_update_bits(regmap, NAU8540_REG_ADC_SAMPLE_RATE,
        NAU8540_CH_SYNC | NAU8540_ADC_OSR_MASK,
        NAU8540_CH_SYNC | NAU8540_ADC_OSR_64);
    /* PGA input mode selection */
    regmap_update_bits(regmap, NAU8540_REG_FEPGA1,
        NAU8540_FEPGA1_MODCH2_SHT | NAU8540_FEPGA1_MODCH1_SHT,
        NAU8540_FEPGA1_MODCH2_SHT | NAU8540_FEPGA1_MODCH1_SHT);
    regmap_update_bits(regmap, NAU8540_REG_FEPGA2,
        NAU8540_FEPGA2_MODCH4_SHT | NAU8540_FEPGA2_MODCH3_SHT,
        NAU8540_FEPGA2_MODCH4_SHT | NAU8540_FEPGA2_MODCH3_SHT);
    /* DO12 and DO34 pad output disable */
    regmap_update_bits(regmap, NAU8540_REG_PCM_CTRL1,
        NAU8540_I2S_DO12_TRI, NAU8540_I2S_DO12_TRI);
    regmap_update_bits(regmap, NAU8540_REG_PCM_CTRL2,
        NAU8540_I2S_DO34_TRI, NAU8540_I2S_DO34_TRI);
}

unsafe extern "C" fn nau8540_suspend(component: *mut snd_soc_component) -> c_int {
    let nau8540 = snd_soc_component_get_drvdata(component) as *mut nau8540;

    regcache_cache_only((*nau8540).regmap, true);
    regcache_mark_dirty((*nau8540).regmap);

    0
}

unsafe extern "C" fn nau8540_resume(component: *mut snd_soc_component) -> c_int {
    let nau8540 = snd_soc_component_get_drvdata(component) as *mut nau8540;

    regcache_cache_only((*nau8540).regmap, false);
    regcache_sync((*nau8540).regmap);

    0
}

static nau8540_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    set_sysclk: Some(nau8540_set_sysclk),
    set_pll: Some(nau8540_set_pll),
    suspend: Some(nau8540_suspend),
    resume: Some(nau8540_resume),
    controls: nau8540_snd_controls.as_ptr(),
    num_controls: nau8540_snd_controls.len() as c_uint,
    dapm_widgets: nau8540_dapm_widgets.as_ptr(),
    num_dapm_widgets: nau8540_dapm_widgets.len() as c_uint,
    dapm_routes: nau8540_dapm_routes.as_ptr(),
    num_dapm_routes: nau8540_dapm_routes.len() as c_uint,
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static nau8540_regmap_config: regmap_config = regmap_config {
    val_bits: 16,
    reg_bits: 16,
    max_register: NAU8540_REG_MAX,
    readable_reg: Some(nau8540_readable_reg),
    writeable_reg: Some(nau8540_writeable_reg),
    volatile_reg: Some(nau8540_volatile_reg),
    cache_type: REGCACHE_RBTREE,
    reg_defaults: nau8540_reg_defaults.as_ptr(),
    num_reg_defaults: nau8540_reg_defaults.len() as c_uint,
};

unsafe extern "C" fn nau8540_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let dev = &mut (*i2c).dev as *mut device;
    let mut nau8540 = dev_get_platdata(dev) as *mut nau8540;
    let mut value: c_int = 0;

    if nau8540.is_null() {
        nau8540 = devm_kzalloc(dev, core::mem::size_of::<nau8540>(), GFP_KERNEL) as *mut nau8540;
        if nau8540.is_null() {
            return -ENOMEM;
        }
    }
    i2c_set_clientdata(i2c, nau8540 as *mut c_void);

    (*nau8540).regmap = devm_regmap_init_i2c(i2c, &nau8540_regmap_config);
    if IS_ERR((*nau8540).regmap as *const c_void) {
        return PTR_ERR((*nau8540).regmap as *const c_void);
    }
    let ret = regmap_read((*nau8540).regmap, NAU8540_REG_I2C_DEVICE_ID, &mut value as *mut c_int as *mut c_uint);
    if ret < 0 {
        dev_err!(dev, "Failed to read device id from the NAU85L40: %d\n", ret);
        return ret;
    }

    (*nau8540).dev = dev;
    nau8540_reset_chip((*nau8540).regmap);
    nau8540_init_regs(nau8540);

    devm_snd_soc_register_component(dev,
        &nau8540_component_driver, &mut nau8540_dai, 1)
}

static nau8540_i2c_ids: [i2c_device_id; 2] = [
    i2c_device_id { name: *b"nau8540\0" },
    i2c_device_id { name: [0; 8] },
];
MODULE_DEVICE_TABLE!(i2c, nau8540_i2c_ids);

/* CONFIG_OF: Open Firmware device match table. */
static nau8540_of_ids: [of_device_id; 2] = [
    of_device_id { compatible: c"nuvoton,nau8540".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];
MODULE_DEVICE_TABLE!(of, nau8540_of_ids);

static mut nau8540_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"nau8540".as_ptr(),
        of_match_table: of_match_ptr!(nau8540_of_ids),
    },
    probe: Some(nau8540_i2c_probe),
    id_table: nau8540_i2c_ids.as_ptr(),
};
module_i2c_driver!(nau8540_i2c_driver);

MODULE_DESCRIPTION!("ASoC NAU85L40 driver");
MODULE_AUTHOR!("John Hsu <KCHSU0@nuvoton.com>");
MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
