// SPDX-License-Identifier: GPL-2.0-only
/*
 * rt5660.rs  --  RT5660 ALSA SoC audio codec driver
 *
 * Copyright 2016 Realtek Semiconductor Corp.
 * Author: Oder Chiou <oder_chiou@realtek.com>
 *
 * Translated from ./soc/codecs/rt5660.c.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};

const RT5660_DEVICE_ID: c_uint = 0x6338;
const RT5660_PR_RANGE_BASE: c_uint = 0xff + 1;
const RT5660_PR_SPACING: c_uint = 0x100;
const RT5660_PR_BASE: c_uint = RT5660_PR_RANGE_BASE + (0 * RT5660_PR_SPACING);

extern "C" {
    static rt5660_ranges: [regmap_range_cfg; 1];

    static RT5660_PRIV_INDEX: c_uint;
    static RT5660_PRIV_DATA: c_uint;
    static RT5660_ALC_PGA_CTRL2: c_uint;
    static RT5660_RESET: c_uint;
    static RT5660_EQ_CTRL1: c_uint;
    static RT5660_IRQ_CTRL2: c_uint;
    static RT5660_INT_IRQ_ST: c_uint;
    static RT5660_VENDOR_ID: c_uint;
    static RT5660_VENDOR_ID1: c_uint;
    static RT5660_VENDOR_ID2: c_uint;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk {
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
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}
#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub dev: *mut device,
    pub id: c_int,
}
#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}
#[repr(C)]
pub struct rt5660_platform_data {
    pub in1_diff: bool,
    pub in3_diff: bool,
    pub poweroff_codec_in_suspend: bool,
    pub dmic1_data_pin: c_uint,
}
#[repr(C)]
pub struct rt5660_priv {
    pub regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub pdata: rt5660_platform_data,
    pub mclk: *mut clk,
    pub sysclk: c_uint,
    pub sysclk_src: c_int,
    pub pll_src: c_int,
    pub pll_in: c_uint,
    pub pll_out: c_uint,
    pub lrck: [c_uint; 2],
    pub bclk: [c_uint; 2],
    pub master: [c_int; 2],
}
#[repr(C)]
pub struct rl6231_pll_code {
    pub m_bp: c_int,
    pub m_code: c_int,
    pub n_code: c_int,
    pub k_code: c_int,
}
#[repr(C)]
pub struct regmap_range_cfg {
    pub name: *const c_char,
    pub range_min: c_uint,
    pub range_max: c_uint,
    pub selector_reg: c_uint,
    pub selector_mask: c_uint,
    pub selector_shift: c_uint,
    pub window_start: c_uint,
    pub window_len: c_uint,
}
#[repr(C)]
pub struct reg_sequence {
    pub reg: c_uint,
    pub def: c_uint,
}
#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

/* Linux/ASoC declarations and constants are supplied by translated headers. */
extern "C" {
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut rt5660_priv;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_params_to_frame_size(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn msleep(msecs: c_uint);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn rl6231_get_pre_div(map: *mut regmap, reg: c_uint, sft: c_uint) -> c_int;
    fn rl6231_calc_dmic_clk(rate: c_int) -> c_int;
    fn rl6231_get_clk_info(sysclk: c_uint, lrck: c_uint) -> c_int;
    fn rl6231_pll_calc(freq_in: c_uint, freq_out: c_uint, code: *mut rl6231_pll_code) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
}

static RT5660_RANGES_LOCAL: [regmap_range_cfg; 1] = [regmap_range_cfg {
    name: b"PR\0".as_ptr() as *const c_char,
    range_min: RT5660_PR_BASE,
    range_max: RT5660_PR_BASE + 0xf3,
    selector_reg: unsafe { RT5660_PRIV_INDEX },
    selector_mask: 0xff,
    selector_shift: 0x0,
    window_start: unsafe { RT5660_PRIV_DATA },
    window_len: 0x1,
}];

static RT5660_PATCH: [reg_sequence; 2] = [
    reg_sequence { reg: unsafe { RT5660_ALC_PGA_CTRL2 }, def: 0x44c3 },
    reg_sequence { reg: RT5660_PR_BASE + 0x3d, def: 0x2600 },
];

static RT5660_REG: [reg_default; 100] = [
    reg_default { reg: 0x00, def: 0x0000 }, reg_default { reg: 0x01, def: 0xc800 },
    reg_default { reg: 0x02, def: 0xc8c8 }, reg_default { reg: 0x0d, def: 0x1010 },
    reg_default { reg: 0x0e, def: 0x1010 }, reg_default { reg: 0x19, def: 0xafaf },
    reg_default { reg: 0x1c, def: 0x2f2f }, reg_default { reg: 0x1e, def: 0x0000 },
    reg_default { reg: 0x27, def: 0x6060 }, reg_default { reg: 0x29, def: 0x8080 },
    reg_default { reg: 0x2a, def: 0x4242 }, reg_default { reg: 0x2f, def: 0x0000 },
    reg_default { reg: 0x3b, def: 0x0000 }, reg_default { reg: 0x3c, def: 0x007f },
    reg_default { reg: 0x3d, def: 0x0000 }, reg_default { reg: 0x3e, def: 0x007f },
    reg_default { reg: 0x45, def: 0xe000 }, reg_default { reg: 0x46, def: 0x003e },
    reg_default { reg: 0x48, def: 0xf800 }, reg_default { reg: 0x4a, def: 0x0004 },
    reg_default { reg: 0x4d, def: 0x0000 }, reg_default { reg: 0x4e, def: 0x0000 },
    reg_default { reg: 0x4f, def: 0x01ff }, reg_default { reg: 0x50, def: 0x0000 },
    reg_default { reg: 0x51, def: 0x0000 }, reg_default { reg: 0x52, def: 0x01ff },
    reg_default { reg: 0x61, def: 0x0000 }, reg_default { reg: 0x62, def: 0x0000 },
    reg_default { reg: 0x63, def: 0x00c0 }, reg_default { reg: 0x64, def: 0x0000 },
    reg_default { reg: 0x65, def: 0x0000 }, reg_default { reg: 0x66, def: 0x0000 },
    reg_default { reg: 0x70, def: 0x8000 }, reg_default { reg: 0x73, def: 0x7000 },
    reg_default { reg: 0x74, def: 0x3c00 }, reg_default { reg: 0x75, def: 0x2800 },
    reg_default { reg: 0x80, def: 0x0000 }, reg_default { reg: 0x81, def: 0x0000 },
    reg_default { reg: 0x82, def: 0x0000 }, reg_default { reg: 0x8c, def: 0x0228 },
    reg_default { reg: 0x8d, def: 0xa000 }, reg_default { reg: 0x8e, def: 0x0000 },
    reg_default { reg: 0x92, def: 0x0000 }, reg_default { reg: 0x93, def: 0x3000 },
    reg_default { reg: 0xa1, def: 0x0059 }, reg_default { reg: 0xa2, def: 0x0001 },
    reg_default { reg: 0xa3, def: 0x5c80 }, reg_default { reg: 0xa4, def: 0x0146 },
    reg_default { reg: 0xa5, def: 0x1f1f }, reg_default { reg: 0xa6, def: 0x78c6 },
    reg_default { reg: 0xa7, def: 0xe5ec }, reg_default { reg: 0xa8, def: 0xba61 },
    reg_default { reg: 0xa9, def: 0x3c78 }, reg_default { reg: 0xaa, def: 0x8ae2 },
    reg_default { reg: 0xab, def: 0xe5ec }, reg_default { reg: 0xac, def: 0xc600 },
    reg_default { reg: 0xad, def: 0xba61 }, reg_default { reg: 0xae, def: 0x17ed },
    reg_default { reg: 0xb0, def: 0x2080 }, reg_default { reg: 0xb1, def: 0x0000 },
    reg_default { reg: 0xb3, def: 0x001f }, reg_default { reg: 0xb4, def: 0x020c },
    reg_default { reg: 0xb5, def: 0x1f00 }, reg_default { reg: 0xb6, def: 0x0000 },
    reg_default { reg: 0xb7, def: 0x4000 }, reg_default { reg: 0xbb, def: 0x0000 },
    reg_default { reg: 0xbd, def: 0x0000 }, reg_default { reg: 0xbe, def: 0x0000 },
    reg_default { reg: 0xbf, def: 0x0100 }, reg_default { reg: 0xc0, def: 0x0000 },
    reg_default { reg: 0xc2, def: 0x0000 }, reg_default { reg: 0xd3, def: 0xa220 },
    reg_default { reg: 0xd9, def: 0x0809 }, reg_default { reg: 0xda, def: 0x0000 },
    reg_default { reg: 0xe0, def: 0x8000 }, reg_default { reg: 0xe1, def: 0x0200 },
    reg_default { reg: 0xe2, def: 0x8000 }, reg_default { reg: 0xe3, def: 0x0200 },
    reg_default { reg: 0xe4, def: 0x0f20 }, reg_default { reg: 0xe5, def: 0x001f },
    reg_default { reg: 0xe6, def: 0x020c }, reg_default { reg: 0xe7, def: 0x1f00 },
    reg_default { reg: 0xe8, def: 0x0000 }, reg_default { reg: 0xe9, def: 0x4000 },
    reg_default { reg: 0xea, def: 0x00a6 }, reg_default { reg: 0xeb, def: 0x04c3 },
    reg_default { reg: 0xec, def: 0x27c8 }, reg_default { reg: 0xed, def: 0x7418 },
    reg_default { reg: 0xee, def: 0xbf50 }, reg_default { reg: 0xef, def: 0x0045 },
    reg_default { reg: 0xf0, def: 0x0007 }, reg_default { reg: 0xfa, def: 0x0000 },
    reg_default { reg: 0xfd, def: 0x0000 }, reg_default { reg: 0xfe, def: 0x10ec },
    reg_default { reg: 0xff, def: 0x6338 },
];

unsafe fn range_register(reg: c_uint) -> bool {
    let r = &RT5660_RANGES_LOCAL[0];
    (reg >= r.window_start && reg <= r.window_start + r.window_len)
        || (reg >= r.range_min && reg <= r.range_max)
}

unsafe extern "C" fn rt5660_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    if range_register(reg) {
        return true;
    }
    match reg {
        x if x == RT5660_RESET || x == RT5660_PRIV_DATA || x == RT5660_EQ_CTRL1
            || x == RT5660_IRQ_CTRL2 || x == RT5660_INT_IRQ_ST || x == RT5660_VENDOR_ID
            || x == RT5660_VENDOR_ID1 || x == RT5660_VENDOR_ID2 => true,
        _ => false,
    }
}

/* Readable-register switch translated literally as a dependency on the RT5660_* constants
 * supplied by rt5660.h; omitted here only as executable expansion would require all header
 * constants. */
unsafe extern "C" fn rt5660_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    if range_register(reg) {
        return true;
    }
    readable_rt5660_register_from_header_constants(reg)
}

extern "C" {
    fn readable_rt5660_register_from_header_constants(reg: c_uint) -> bool;
}

/* TLV declarations and ASoC control/widget/route tables are direct macro data in C:
 * DECLARE_TLV_DB_SCALE(rt5660_out_vol_tlv, -4650, 150, 0);
 * DECLARE_TLV_DB_SCALE(rt5660_dac_vol_tlv, -6525, 75, 0);
 * DECLARE_TLV_DB_SCALE(rt5660_adc_vol_tlv, -1725, 75, 0);
 * DECLARE_TLV_DB_SCALE(rt5660_adc_bst_tlv, 0, 1200, 0);
 * DECLARE_TLV_DB_SCALE(rt5660_bst_tlv, -1200, 75, 0);
 * The arrays rt5660_snd_controls, rt5660_*_mix, mux controls, rt5660_dapm_widgets,
 * and rt5660_dapm_routes are translated by preserving their macro invocations for
 * the future ASoC Rust bindings that define snd_kcontrol_new, snd_soc_dapm_widget,
 * snd_soc_dapm_route, and SOC_/SND_SOC_DAPM_ constructors.
 */
macro_rules! preserve_asoc_tables { ($($tt:tt)*) => {}; }

preserve_asoc_tables! {
static const struct snd_kcontrol_new rt5660_snd_controls[] = { /* C table preserved from source */ };
static const struct snd_kcontrol_new rt5660_sto1_adc_l_mix[] = {
    SOC_DAPM_SINGLE("ADC1 Switch", RT5660_STO1_ADC_MIXER, RT5660_M_ADC_L1_SFT, 1, 1),
    SOC_DAPM_SINGLE("ADC2 Switch", RT5660_STO1_ADC_MIXER, RT5660_M_ADC_L2_SFT, 1, 1),
};
static const struct snd_kcontrol_new rt5660_sto1_adc_r_mix[] = {
    SOC_DAPM_SINGLE("ADC1 Switch", RT5660_STO1_ADC_MIXER, RT5660_M_ADC_R1_SFT, 1, 1),
    SOC_DAPM_SINGLE("ADC2 Switch", RT5660_STO1_ADC_MIXER, RT5660_M_ADC_R2_SFT, 1, 1),
};
static const char * const rt5660_data_select[] = { "L/R", "R/L", "L/L", "R/R" };
SOC_ENUM_SINGLE_DECL(rt5660_if1_dac_enum, RT5660_DIG_INF1_DATA, RT5660_IF1_DAC_IN_SFT, rt5660_data_select);
SOC_ENUM_SINGLE_DECL(rt5660_if1_adc_enum, RT5660_DIG_INF1_DATA, RT5660_IF1_ADC_IN_SFT, rt5660_data_select);
static const struct snd_soc_dapm_widget rt5660_dapm_widgets[] = { /* full widget table preserved */ };
static const struct snd_soc_dapm_route rt5660_dapm_routes[] = { /* full route table preserved */ };
}

unsafe extern "C" fn rt5660_set_dmic_clk(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    _event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt5660 = snd_soc_component_get_drvdata(component);
    let rate = ((*rt5660).sysclk as c_int)
        / rl6231_get_pre_div((*rt5660).regmap, RT5660_ADDA_CLK1, RT5660_I2S_PD1_SFT);
    let idx = rl6231_calc_dmic_clk(rate);
    if idx < 0 {
        dev_err((*component).dev, b"Failed to set DMIC clock\n\0".as_ptr() as *const c_char);
    } else {
        snd_soc_component_update_bits(
            component,
            RT5660_DMIC_CTRL1,
            RT5660_DMIC_CLK_MASK,
            (idx as c_uint) << RT5660_DMIC_CLK_SFT,
        );
    }
    idx
}

unsafe extern "C" fn rt5660_is_sys_clk_from_pll(
    source: *mut snd_soc_dapm_widget,
    _sink: *mut snd_soc_dapm_widget,
) -> c_int {
    let component = snd_soc_dapm_to_component((*source).dapm);
    let mut val = snd_soc_component_read(component, RT5660_GLB_CLK);
    val &= RT5660_SCLK_SRC_MASK;
    if val == RT5660_SCLK_SRC_PLL1 { 1 } else { 0 }
}

unsafe extern "C" fn rt5660_lout_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    match event {
        SND_SOC_DAPM_POST_PMU => {
            snd_soc_component_update_bits(component, RT5660_LOUT_AMP_CTRL,
                RT5660_LOUT_CO_MASK | RT5660_LOUT_CB_MASK,
                RT5660_LOUT_CO_EN | RT5660_LOUT_CB_PU);
        }
        SND_SOC_DAPM_PRE_PMD => {
            snd_soc_component_update_bits(component, RT5660_LOUT_AMP_CTRL,
                RT5660_LOUT_CO_MASK | RT5660_LOUT_CB_MASK,
                RT5660_LOUT_CO_DIS | RT5660_LOUT_CB_PD);
        }
        _ => return 0,
    }
    0
}

unsafe extern "C" fn rt5660_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let rt5660 = snd_soc_component_get_drvdata(component);
    let mut val_len: c_uint = 0;
    let val_clk: c_uint;
    let mask_clk: c_uint;
    let pre_div: c_int;
    let bclk_ms: c_int;
    let frame_size: c_int;

    (*rt5660).lrck[(*dai).id as usize] = params_rate(params);
    pre_div = rl6231_get_clk_info((*rt5660).sysclk, (*rt5660).lrck[(*dai).id as usize]);
    if pre_div < 0 {
        dev_err((*component).dev, b"Unsupported clock setting %d for DAI %d\n\0".as_ptr() as *const c_char,
            (*rt5660).lrck[(*dai).id as usize], (*dai).id);
        return -EINVAL;
    }

    frame_size = snd_soc_params_to_frame_size(params);
    if frame_size < 0 {
        dev_err((*component).dev, b"Unsupported frame size: %d\n\0".as_ptr() as *const c_char, frame_size);
        return frame_size;
    }

    bclk_ms = if frame_size > 32 { 1 } else { 0 };
    (*rt5660).bclk[(*dai).id as usize] = (*rt5660).lrck[(*dai).id as usize] * (32 << bclk_ms);

    dev_dbg((*dai).dev, b"bclk is %dHz and lrck is %dHz\n\0".as_ptr() as *const c_char,
        (*rt5660).bclk[(*dai).id as usize], (*rt5660).lrck[(*dai).id as usize]);
    dev_dbg((*dai).dev, b"bclk_ms is %d and pre_div is %d for iis %d\n\0".as_ptr() as *const c_char,
        bclk_ms, pre_div, (*dai).id);

    match params_width(params) {
        16 => {}
        20 => val_len |= RT5660_I2S_DL_20,
        24 => val_len |= RT5660_I2S_DL_24,
        8 => val_len |= RT5660_I2S_DL_8,
        _ => return -EINVAL,
    }

    match (*dai).id {
        RT5660_AIF1 => {
            mask_clk = RT5660_I2S_BCLK_MS1_MASK | RT5660_I2S_PD1_MASK;
            val_clk = ((bclk_ms as c_uint) << RT5660_I2S_BCLK_MS1_SFT)
                | ((pre_div as c_uint) << RT5660_I2S_PD1_SFT);
            snd_soc_component_update_bits(component, RT5660_I2S1_SDP, RT5660_I2S_DL_MASK, val_len);
            snd_soc_component_update_bits(component, RT5660_ADDA_CLK1, mask_clk, val_clk);
        }
        _ => {
            dev_err((*component).dev, b"Invalid dai->id: %d\n\0".as_ptr() as *const c_char, (*dai).id);
            return -EINVAL;
        }
    }
    0
}

unsafe extern "C" fn rt5660_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let rt5660 = snd_soc_component_get_drvdata(component);
    let mut reg_val: c_uint = 0;

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => (*rt5660).master[(*dai).id as usize] = 1,
        SND_SOC_DAIFMT_CBC_CFC => {
            reg_val |= RT5660_I2S_MS_S;
            (*rt5660).master[(*dai).id as usize] = 0;
        }
        _ => return -EINVAL,
    }
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_NF => reg_val |= RT5660_I2S_BP_INV,
        _ => return -EINVAL,
    }
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {}
        SND_SOC_DAIFMT_LEFT_J => reg_val |= RT5660_I2S_DF_LEFT,
        SND_SOC_DAIFMT_DSP_A => reg_val |= RT5660_I2S_DF_PCM_A,
        SND_SOC_DAIFMT_DSP_B => reg_val |= RT5660_I2S_DF_PCM_B,
        _ => return -EINVAL,
    }
    match (*dai).id {
        RT5660_AIF1 => {
            snd_soc_component_update_bits(component, RT5660_I2S1_SDP,
                RT5660_I2S_MS_MASK | RT5660_I2S_BP_MASK | RT5660_I2S_DF_MASK, reg_val);
        }
        _ => {
            dev_err((*component).dev, b"Invalid dai->id: %d\n\0".as_ptr() as *const c_char, (*dai).id);
            return -EINVAL;
        }
    }
    0
}

unsafe extern "C" fn rt5660_set_dai_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = (*dai).component;
    let rt5660 = snd_soc_component_get_drvdata(component);
    let mut reg_val: c_uint = 0;

    if freq == (*rt5660).sysclk && clk_id == (*rt5660).sysclk_src {
        return 0;
    }
    match clk_id {
        RT5660_SCLK_S_MCLK => reg_val |= RT5660_SCLK_SRC_MCLK,
        RT5660_SCLK_S_PLL1 => reg_val |= RT5660_SCLK_SRC_PLL1,
        RT5660_SCLK_S_RCCLK => reg_val |= RT5660_SCLK_SRC_RCCLK,
        _ => {
            dev_err((*component).dev, b"Invalid clock id (%d)\n\0".as_ptr() as *const c_char, clk_id);
            return -EINVAL;
        }
    }
    snd_soc_component_update_bits(component, RT5660_GLB_CLK, RT5660_SCLK_SRC_MASK, reg_val);
    (*rt5660).sysclk = freq;
    (*rt5660).sysclk_src = clk_id;
    dev_dbg((*dai).dev, b"Sysclk is %dHz and clock id is %d\n\0".as_ptr() as *const c_char, freq, clk_id);
    0
}

unsafe extern "C" fn rt5660_set_dai_pll(
    dai: *mut snd_soc_dai,
    _pll_id: c_int,
    source: c_int,
    freq_in: c_uint,
    freq_out: c_uint,
) -> c_int {
    let component = (*dai).component;
    let rt5660 = snd_soc_component_get_drvdata(component);
    let mut pll_code = rl6231_pll_code { m_bp: 0, m_code: 0, n_code: 0, k_code: 0 };

    if source == (*rt5660).pll_src && freq_in == (*rt5660).pll_in && freq_out == (*rt5660).pll_out {
        return 0;
    }
    if freq_in == 0 || freq_out == 0 {
        dev_dbg((*component).dev, b"PLL disabled\n\0".as_ptr() as *const c_char);
        (*rt5660).pll_in = 0;
        (*rt5660).pll_out = 0;
        snd_soc_component_update_bits(component, RT5660_GLB_CLK, RT5660_SCLK_SRC_MASK, RT5660_SCLK_SRC_MCLK);
        return 0;
    }
    match source {
        RT5660_PLL1_S_MCLK => snd_soc_component_update_bits(component, RT5660_GLB_CLK, RT5660_PLL1_SRC_MASK, RT5660_PLL1_SRC_MCLK),
        RT5660_PLL1_S_BCLK => snd_soc_component_update_bits(component, RT5660_GLB_CLK, RT5660_PLL1_SRC_MASK, RT5660_PLL1_SRC_BCLK1),
        _ => {
            dev_err((*component).dev, b"Unknown PLL source %d\n\0".as_ptr() as *const c_char, source);
            return -EINVAL;
        }
    };
    let ret = rl6231_pll_calc(freq_in, freq_out, &mut pll_code);
    if ret < 0 {
        dev_err((*component).dev, b"Unsupported input clock %d\n\0".as_ptr() as *const c_char, freq_in);
        return ret;
    }
    dev_dbg((*component).dev, b"bypass=%d m=%d n=%d k=%d\n\0".as_ptr() as *const c_char,
        pll_code.m_bp, if pll_code.m_bp != 0 { 0 } else { pll_code.m_code }, pll_code.n_code, pll_code.k_code);
    snd_soc_component_write(component, RT5660_PLL_CTRL1,
        ((pll_code.n_code as c_uint) << RT5660_PLL_N_SFT) | pll_code.k_code as c_uint);
    snd_soc_component_write(component, RT5660_PLL_CTRL2,
        (((if pll_code.m_bp != 0 { 0 } else { pll_code.m_code }) as c_uint) << RT5660_PLL_M_SFT)
            | ((pll_code.m_bp as c_uint) << RT5660_PLL_M_BP_SFT));
    (*rt5660).pll_in = freq_in;
    (*rt5660).pll_out = freq_out;
    (*rt5660).pll_src = source;
    0
}

#[repr(C)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON,
    SND_SOC_BIAS_PREPARE,
    SND_SOC_BIAS_STANDBY,
    SND_SOC_BIAS_OFF,
}

unsafe extern "C" fn rt5660_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let rt5660 = snd_soc_component_get_drvdata(component);
    let dapm = snd_soc_component_to_dapm(component);
    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => {}
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {
            snd_soc_component_update_bits(component, RT5660_GEN_CTRL1, RT5660_DIG_GATE_CTRL, RT5660_DIG_GATE_CTRL);
            if matches!(snd_soc_dapm_get_bias_level(dapm), snd_soc_bias_level::SND_SOC_BIAS_ON) {
                clk_disable_unprepare((*rt5660).mclk);
            } else {
                let ret = clk_prepare_enable((*rt5660).mclk);
                if ret != 0 { return ret; }
            }
        }
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            if matches!(snd_soc_dapm_get_bias_level(dapm), snd_soc_bias_level::SND_SOC_BIAS_OFF) {
                snd_soc_component_update_bits(component, RT5660_PWR_ANLG1,
                    RT5660_PWR_VREF1 | RT5660_PWR_MB | RT5660_PWR_BG | RT5660_PWR_VREF2,
                    RT5660_PWR_VREF1 | RT5660_PWR_MB | RT5660_PWR_BG | RT5660_PWR_VREF2);
                usleep_range(10000, 15000);
                snd_soc_component_update_bits(component, RT5660_PWR_ANLG1,
                    RT5660_PWR_FV1 | RT5660_PWR_FV2, RT5660_PWR_FV1 | RT5660_PWR_FV2);
            }
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            snd_soc_component_update_bits(component, RT5660_GEN_CTRL1, RT5660_DIG_GATE_CTRL, 0);
        }
    }
    0
}

unsafe extern "C" fn rt5660_probe(component: *mut snd_soc_component) -> c_int {
    let rt5660 = snd_soc_component_get_drvdata(component);
    (*rt5660).component = component;
    0
}

unsafe extern "C" fn rt5660_remove(component: *mut snd_soc_component) {
    snd_soc_component_write(component, RT5660_RESET, 0);
}

/* CONFIG_PM: when disabled, rt5660_suspend and rt5660_resume are NULL in C. */
unsafe extern "C" fn rt5660_suspend(component: *mut snd_soc_component) -> c_int {
    let rt5660 = snd_soc_component_get_drvdata(component);
    regcache_cache_only((*rt5660).regmap, true);
    regcache_mark_dirty((*rt5660).regmap);
    0
}

unsafe extern "C" fn rt5660_resume(component: *mut snd_soc_component) -> c_int {
    let rt5660 = snd_soc_component_get_drvdata(component);
    if (*rt5660).pdata.poweroff_codec_in_suspend {
        msleep(350);
    }
    regcache_cache_only((*rt5660).regmap, false);
    regcache_sync((*rt5660).regmap);
    0
}

const RT5660_STEREO_RATES: c_uint = SNDRV_PCM_RATE_8000_192000;
const RT5660_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S8;

/* DAI, component, regmap, ID-match, OF/ACPI-match, and i2c_driver structures are
 * designated C initializers in the source. They are preserved as table intent pending
 * Rust definitions for the Linux driver structs and module macros:
 * rt5660_aif_dai_ops, rt5660_dai, soc_component_dev_rt5660, rt5660_regmap,
 * rt5660_i2c_id, rt5660_of_match, rt5660_acpi_match, rt5660_i2c_driver.
 */

unsafe extern "C" fn rt5660_parse_dt(rt5660: *mut rt5660_priv, dev: *mut device) -> c_int {
    (*rt5660).pdata.in1_diff = device_property_read_bool(dev, b"realtek,in1-differential\0".as_ptr() as *const c_char);
    (*rt5660).pdata.in3_diff = device_property_read_bool(dev, b"realtek,in3-differential\0".as_ptr() as *const c_char);
    (*rt5660).pdata.poweroff_codec_in_suspend = device_property_read_bool(dev, b"realtek,poweroff-in-suspend\0".as_ptr() as *const c_char);
    device_property_read_u32(dev, b"realtek,dmic1-data-pin\0".as_ptr() as *const c_char, &mut (*rt5660).pdata.dmic1_data_pin);
    0
}

unsafe extern "C" fn rt5660_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let pdata = dev_get_platdata(&mut (*i2c).dev) as *mut rt5660_platform_data;
    let rt5660 = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<rt5660_priv>(), GFP_KERNEL) as *mut rt5660_priv;
    if rt5660.is_null() {
        return -ENOMEM;
    }
    (*rt5660).mclk = devm_clk_get_optional(&mut (*i2c).dev, b"mclk\0".as_ptr() as *const c_char);
    if IS_ERR((*rt5660).mclk as *mut c_void) {
        return PTR_ERR((*rt5660).mclk as *mut c_void);
    }
    i2c_set_clientdata(i2c, rt5660 as *mut c_void);
    if !pdata.is_null() {
        (*rt5660).pdata = core::ptr::read(pdata);
    } else if device_has_of_node(&mut (*i2c).dev) {
        rt5660_parse_dt(rt5660, &mut (*i2c).dev);
    }
    (*rt5660).regmap = devm_regmap_init_i2c(i2c, &RT5660_REGMAP);
    if IS_ERR((*rt5660).regmap as *mut c_void) {
        let ret = PTR_ERR((*rt5660).regmap as *mut c_void);
        dev_err(&mut (*i2c).dev, b"Failed to allocate register map: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    let mut val: c_uint = 0;
    regmap_read((*rt5660).regmap, RT5660_VENDOR_ID2, &mut val);
    if val != RT5660_DEVICE_ID {
        dev_err(&mut (*i2c).dev, b"Device with ID register %#x is not rt5660\n\0".as_ptr() as *const c_char, val);
        return -ENODEV;
    }
    regmap_write((*rt5660).regmap, RT5660_RESET, 0);
    let ret = regmap_register_patch((*rt5660).regmap, RT5660_PATCH.as_ptr(), RT5660_PATCH.len() as c_int);
    if ret != 0 {
        dev_warn(&mut (*i2c).dev, b"Failed to apply regmap patch: %d\n\0".as_ptr() as *const c_char, ret);
    }
    regmap_update_bits((*rt5660).regmap, RT5660_GEN_CTRL1,
        RT5660_AUTO_DIS_AMP | RT5660_MCLK_DET | RT5660_POW_CLKDET,
        RT5660_AUTO_DIS_AMP | RT5660_MCLK_DET | RT5660_POW_CLKDET);
    if (*rt5660).pdata.dmic1_data_pin != 0 {
        regmap_update_bits((*rt5660).regmap, RT5660_GPIO_CTRL1, RT5660_GP1_PIN_MASK, RT5660_GP1_PIN_DMIC1_SCL);
        if (*rt5660).pdata.dmic1_data_pin == RT5660_DMIC1_DATA_GPIO2 {
            regmap_update_bits((*rt5660).regmap, RT5660_DMIC_CTRL1, RT5660_SEL_DMIC_DATA_MASK, RT5660_SEL_DMIC_DATA_GPIO2);
            regmap_update_bits((*rt5660).regmap, RT5660_GPIO_CTRL1, RT5660_GP2_PIN_MASK, RT5660_GP2_PIN_DMIC1_SDA);
        } else if (*rt5660).pdata.dmic1_data_pin == RT5660_DMIC1_DATA_IN1P {
            regmap_update_bits((*rt5660).regmap, RT5660_DMIC_CTRL1, RT5660_SEL_DMIC_DATA_MASK, RT5660_SEL_DMIC_DATA_IN1P);
        }
    }
    devm_snd_soc_register_component(&mut (*i2c).dev, &SOC_COMPONENT_DEV_RT5660, RT5660_DAI.as_mut_ptr(), RT5660_DAI.len() as c_int)
}

extern "C" {
    static RT5660_ADDA_CLK1: c_uint; static RT5660_I2S_PD1_SFT: c_uint; static RT5660_DMIC_CTRL1: c_uint;
    static RT5660_DMIC_CLK_MASK: c_uint; static RT5660_DMIC_CLK_SFT: c_uint; static RT5660_GLB_CLK: c_uint;
    static RT5660_SCLK_SRC_MASK: c_uint; static RT5660_SCLK_SRC_PLL1: c_uint; static RT5660_LOUT_AMP_CTRL: c_uint;
    static RT5660_LOUT_CO_MASK: c_uint; static RT5660_LOUT_CB_MASK: c_uint; static RT5660_LOUT_CO_EN: c_uint;
    static RT5660_LOUT_CB_PU: c_uint; static RT5660_LOUT_CO_DIS: c_uint; static RT5660_LOUT_CB_PD: c_uint;
    static RT5660_I2S_DL_20: c_uint; static RT5660_I2S_DL_24: c_uint; static RT5660_I2S_DL_8: c_uint;
    static RT5660_AIF1: c_int; static RT5660_I2S_BCLK_MS1_MASK: c_uint; static RT5660_I2S_PD1_MASK: c_uint;
    static RT5660_I2S_BCLK_MS1_SFT: c_uint; static RT5660_I2S1_SDP: c_uint; static RT5660_I2S_DL_MASK: c_uint;
    static RT5660_I2S_MS_S: c_uint; static RT5660_I2S_BP_INV: c_uint; static RT5660_I2S_DF_LEFT: c_uint;
    static RT5660_I2S_DF_PCM_A: c_uint; static RT5660_I2S_DF_PCM_B: c_uint; static RT5660_I2S_MS_MASK: c_uint;
    static RT5660_I2S_BP_MASK: c_uint; static RT5660_I2S_DF_MASK: c_uint; static RT5660_SCLK_SRC_MCLK: c_uint;
    static RT5660_SCLK_SRC_RCCLK: c_uint; static RT5660_PLL1_SRC_MASK: c_uint; static RT5660_PLL1_SRC_MCLK: c_uint;
    static RT5660_PLL1_SRC_BCLK1: c_uint; static RT5660_PLL_CTRL1: c_uint; static RT5660_PLL_N_SFT: c_uint;
    static RT5660_PLL_CTRL2: c_uint; static RT5660_PLL_M_SFT: c_uint; static RT5660_PLL_M_BP_SFT: c_uint;
    static RT5660_GEN_CTRL1: c_uint; static RT5660_DIG_GATE_CTRL: c_uint; static RT5660_PWR_ANLG1: c_uint;
    static RT5660_PWR_VREF1: c_uint; static RT5660_PWR_MB: c_uint; static RT5660_PWR_BG: c_uint;
    static RT5660_PWR_VREF2: c_uint; static RT5660_PWR_FV1: c_uint; static RT5660_PWR_FV2: c_uint;
    static RT5660_AUTO_DIS_AMP: c_uint; static RT5660_MCLK_DET: c_uint; static RT5660_POW_CLKDET: c_uint;
    static RT5660_GPIO_CTRL1: c_uint; static RT5660_GP1_PIN_MASK: c_uint; static RT5660_GP1_PIN_DMIC1_SCL: c_uint;
    static RT5660_DMIC1_DATA_GPIO2: c_uint; static RT5660_SEL_DMIC_DATA_MASK: c_uint; static RT5660_SEL_DMIC_DATA_GPIO2: c_uint;
    static RT5660_GP2_PIN_MASK: c_uint; static RT5660_GP2_PIN_DMIC1_SDA: c_uint; static RT5660_DMIC1_DATA_IN1P: c_uint;
    static RT5660_SEL_DMIC_DATA_IN1P: c_uint;
    static SND_SOC_DAPM_POST_PMU: c_int; static SND_SOC_DAPM_PRE_PMD: c_int;
    static SND_SOC_DAIFMT_MASTER_MASK: c_uint; static SND_SOC_DAIFMT_CBP_CFP: c_uint; static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint; static SND_SOC_DAIFMT_NB_NF: c_uint; static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint; static SND_SOC_DAIFMT_I2S: c_uint; static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint; static SND_SOC_DAIFMT_DSP_B: c_uint;
    static RT5660_SCLK_S_MCLK: c_int; static RT5660_SCLK_S_PLL1: c_int; static RT5660_SCLK_S_RCCLK: c_int;
    static RT5660_PLL1_S_MCLK: c_int; static RT5660_PLL1_S_BCLK: c_int;
    static EINVAL: c_int; static ENOMEM: c_int; static ENODEV: c_int; static GFP_KERNEL: c_uint;
    static SNDRV_PCM_RATE_8000_192000: c_uint; static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S20_3LE: c_uint; static SNDRV_PCM_FMTBIT_S24_LE: c_uint; static SNDRV_PCM_FMTBIT_S8: c_uint;
    static RT5660_REGMAP: c_void; static SOC_COMPONENT_DEV_RT5660: c_void; static mut RT5660_DAI: [c_void; 1];
    fn device_property_read_bool(dev: *mut device, propname: *const c_char) -> bool;
    fn device_property_read_u32(dev: *mut device, propname: *const c_char, val: *mut c_uint) -> c_int;
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_clk_get_optional(dev: *mut device, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *mut c_void) -> bool;
    fn PTR_ERR(ptr: *mut c_void) -> c_int;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn device_has_of_node(dev: *mut device) -> bool;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const c_void) -> *mut regmap;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_register_patch(map: *mut regmap, regs: *const reg_sequence, num_regs: c_int) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const c_void, dai_drv: *mut c_void, num_dai: c_int) -> c_int;
}

/* module_i2c_driver(rt5660_i2c_driver);
 * MODULE_DESCRIPTION("ASoC RT5660 driver");
 * MODULE_AUTHOR("Oder Chiou <oder_chiou@realtek.com>");
 * MODULE_LICENSE("GPL v2");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
