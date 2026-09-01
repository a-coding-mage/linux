// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2022, Analog Devices Inc.

// Translated from soc/codecs/max98388.c.
// Linux/kernel and ALSA symbols included by the C file are external dependencies.

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut c_void,
}
#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}
#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}
#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}
#[repr(C)]
pub struct max98388_priv {
    pub regmap: *mut regmap,
    pub reset_gpio: *mut gpio_desc,
    pub v_slot: c_uint,
    pub i_slot: c_uint,
    pub ch_size: c_int,
    pub tdm_mode: bool,
    pub interleave_mode: bool,
}

unsafe extern "C" {
    static MAX98388_R2000_SW_RESET: c_uint;
    static MAX98388_R2001_INT_RAW1: c_uint;
    static MAX98388_R2002_INT_RAW2: c_uint;
    static MAX98388_R2004_INT_STATE1: c_uint;
    static MAX98388_R2005_INT_STATE2: c_uint;
    static MAX98388_R2020_THERM_WARN_THRESH: c_uint;
    static MAX98388_R2031_SPK_MON_THRESH: c_uint;
    static MAX98388_R2032_SPK_MON_LD_SEL: c_uint;
    static MAX98388_R2033_SPK_MON_DURATION: c_uint;
    static MAX98388_R2037_ERR_MON_CTRL: c_uint;
    static MAX98388_R2040_PCM_MODE_CFG: c_uint;
    static MAX98388_R2041_PCM_CLK_SETUP: c_uint;
    static MAX98388_R2042_PCM_SR_SETUP: c_uint;
    static MAX98388_R2044_PCM_TX_CTRL1: c_uint;
    static MAX98388_R2045_PCM_TX_CTRL2: c_uint;
    static MAX98388_R2050_PCM_TX_HIZ_CTRL1: c_uint;
    static MAX98388_R2051_PCM_TX_HIZ_CTRL2: c_uint;
    static MAX98388_R2052_PCM_TX_HIZ_CTRL3: c_uint;
    static MAX98388_R2053_PCM_TX_HIZ_CTRL4: c_uint;
    static MAX98388_R2054_PCM_TX_HIZ_CTRL5: c_uint;
    static MAX98388_R2055_PCM_TX_HIZ_CTRL6: c_uint;
    static MAX98388_R2056_PCM_TX_HIZ_CTRL7: c_uint;
    static MAX98388_R2057_PCM_TX_HIZ_CTRL8: c_uint;
    static MAX98388_R2058_PCM_RX_SRC1: c_uint;
    static MAX98388_R2059_PCM_RX_SRC2: c_uint;
    static MAX98388_R205C_PCM_TX_DRIVE_STRENGTH: c_uint;
    static MAX98388_R205D_PCM_TX_SRC_EN: c_uint;
    static MAX98388_R205E_PCM_RX_EN: c_uint;
    static MAX98388_R205F_PCM_TX_EN: c_uint;
    static MAX98388_R2090_SPK_CH_VOL_CTRL: c_uint;
    static MAX98388_R2091_SPK_CH_CFG: c_uint;
    static MAX98388_R2092_SPK_AMP_OUT_CFG: c_uint;
    static MAX98388_R2093_SPK_AMP_SSM_CFG: c_uint;
    static MAX98388_R2094_SPK_AMP_ER_CTRL: c_uint;
    static MAX98388_R209E_SPK_CH_PINK_NOISE_EN: c_uint;
    static MAX98388_R209F_SPK_CH_AMP_EN: c_uint;
    static MAX98388_R20A0_IV_DATA_DSP_CTRL: c_uint;
    static MAX98388_R20A7_IV_DATA_EN: c_uint;
    static MAX98388_R20E0_BP_ALC_THRESH: c_uint;
    static MAX98388_R20E1_BP_ALC_RATES: c_uint;
    static MAX98388_R20E2_BP_ALC_ATTEN: c_uint;
    static MAX98388_R20E3_BP_ALC_REL: c_uint;
    static MAX98388_R20E4_BP_ALC_MUTE: c_uint;
    static MAX98388_R20EE_BP_INF_HOLD_REL: c_uint;
    static MAX98388_R20EF_BP_ALC_EN: c_uint;
    static MAX98388_R210E_AUTO_RESTART: c_uint;
    static MAX98388_R210F_GLOBAL_EN: c_uint;
    static MAX98388_R22FF_REV_ID: c_uint;

    static MAX98388_SOFT_RESET: c_uint;
    static MAX98388_PCM_TO_SPK_MONOMIX_CFG_SHIFT: c_uint;
    static MAX98388_ALC_MAX_ATTEN_SHIFT: c_uint;
    static MAX98388_THERM_WARN_THRESH_SHIFT: c_uint;
    static MAX98388_THERM_SHDN_THRESH_SHIFT: c_uint;
    static MAX98388_ALC_THRESH_SHIFT: c_uint;
    static MAX98388_ALC_ATTACK_RATE_SHIFT: c_uint;
    static MAX98388_ALC_RELEASE_RATE_SHIFT: c_uint;
    static MAX98388_ALC_DEBOUNCE_TIME_SHIFT: c_uint;
    static MAX98388_ALC_MUTE_DELAY_SHIFT: c_uint;
    static MAX98388_SPKMON_DURATION_SHIFT: c_uint;
    static MAX98388_SPKMON_THRESH_SHIFT: c_uint;
    static MAX98388_SPKMON_LOAD_SHIFT: c_uint;
    static MAX98388_EDGE_RATE_FALL_SHIFT: c_uint;
    static MAX98388_EDGE_RATE_RISE_SHIFT: c_uint;
    static MAX98388_SPK_AMP_SSM_MOD_SHIFT: c_uint;
    static MAX98388_SPK_CFG_VOL_RMPUP_SHIFT: c_uint;
    static MAX98388_SPK_CFG_VOL_RMPDN_SHIFT: c_uint;
    static MAX98388_SPK_AMP_OUT_MODE_SHIFT: c_uint;
    static MAX98388_OVC_AUTORESTART_SHIFT: c_uint;
    static MAX98388_THERM_AUTORESTART_SHIFT: c_uint;
    static MAX98388_PVDD_UVLO_AUTORESTART_SHIFT: c_uint;
    static MAX98388_CMON_AUTORESTART_SHIFT: c_uint;
    static MAX98388_CLOCK_MON_SHIFT: c_uint;
    static MAX98388_PINK_NOISE_GEN_SHIFT: c_uint;
    static MAX98388_SPK_CFG_DITH_EN_SHIFT: c_uint;
    static MAX98388_AMP_DSP_CTRL_DITH_SHIFT: c_uint;
    static MAX98388_SPK_CFG_DCBLK_SHIFT: c_uint;
    static MAX98388_AMP_DSP_CTRL_VOL_DCBLK_SHIFT: c_uint;
    static MAX98388_AMP_DSP_CTRL_CUR_DCBLK_SHIFT: c_uint;
    static MAX98388_ALC_UNMUTE_RAMP_EN_SHIFT: c_uint;
    static MAX98388_ALC_MUTE_RAMP_EN_SHIFT: c_uint;
    static MAX98388_ALC_MUTE_EN_SHIFT: c_uint;
    static MAX98388_SPK_MON_SHIFT: c_uint;
    static MAX98388_SPK_AMP_SSM_EN_SHIFT: c_uint;
    static MAX98388_PCM_TX_CH_INTERLEAVE_MASK: c_uint;
    static MAX98388_SPK_EN_MASK: c_uint;
    static MAX98388_PCM_MODE_CFG_PCM_BCLKEDGE: c_uint;
    static MAX98388_PCM_MODE_CFG_FORMAT_MASK: c_uint;
    static MAX98388_PCM_MODE_CFG_FORMAT_SHIFT: c_uint;
    static MAX98388_PCM_FORMAT_I2S: c_uint;
    static MAX98388_PCM_FORMAT_LJ: c_uint;
    static MAX98388_PCM_FORMAT_TDM_MODE1: c_uint;
    static MAX98388_PCM_FORMAT_TDM_MODE0: c_uint;
    static MAX98388_PCM_CLK_SETUP_BSEL_MASK: c_uint;
    static MAX98388_PCM_MODE_CFG_CHANSZ_16: c_uint;
    static MAX98388_PCM_MODE_CFG_CHANSZ_24: c_uint;
    static MAX98388_PCM_MODE_CFG_CHANSZ_32: c_uint;
    static MAX98388_PCM_MODE_CFG_CHANSZ_MASK: c_uint;
    static MAX98388_PCM_SR_8000: c_uint;
    static MAX98388_PCM_SR_11025: c_uint;
    static MAX98388_PCM_SR_12000: c_uint;
    static MAX98388_PCM_SR_16000: c_uint;
    static MAX98388_PCM_SR_22050: c_uint;
    static MAX98388_PCM_SR_24000: c_uint;
    static MAX98388_PCM_SR_32000: c_uint;
    static MAX98388_PCM_SR_44100: c_uint;
    static MAX98388_PCM_SR_48000: c_uint;
    static MAX98388_PCM_SR_88200: c_uint;
    static MAX98388_PCM_SR_96000: c_uint;
    static MAX98388_PCM_SR_MASK: c_uint;
    static MAX98388_PCM_SR_IV_MASK: c_uint;
    static MAX98388_PCM_SR_IV_SHIFT: c_uint;
    static MAX98388_RX_SRC_CH0_SHIFT: c_uint;
    static MAX98388_RX_SRC_CH1_SHIFT: c_uint;

    static SND_SOC_DAPM_POST_PMU: c_int;
    static SND_SOC_DAPM_PRE_PMD: c_int;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static GFP_KERNEL: c_uint;
    static GPIOD_OUT_HIGH: c_uint;

    fn snd_soc_dapm_to_component(dapm: *mut c_void) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_int) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn device_property_read_u32(dev: *mut device, propname: *const c_char, val: *mut c_int) -> c_int;
    fn device_property_read_bool(dev: *mut device, propname: *const c_char) -> bool;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const c_void) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const c_void,
        dai_drv: *mut c_void,
        num_dai: usize,
    ) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

unsafe fn reg_default_entry(reg: c_uint, def: c_uint) -> reg_default {
    reg_default { reg, def }
}

static mut max98388_reg: [reg_default; 47] = unsafe {
    [
        reg_default_entry(MAX98388_R2000_SW_RESET, 0x00),
        reg_default_entry(MAX98388_R2001_INT_RAW1, 0x00),
        reg_default_entry(MAX98388_R2002_INT_RAW2, 0x00),
        reg_default_entry(MAX98388_R2004_INT_STATE1, 0x00),
        reg_default_entry(MAX98388_R2005_INT_STATE2, 0x00),
        reg_default_entry(MAX98388_R2020_THERM_WARN_THRESH, 0x0A),
        reg_default_entry(MAX98388_R2031_SPK_MON_THRESH, 0x58),
        reg_default_entry(MAX98388_R2032_SPK_MON_LD_SEL, 0x08),
        reg_default_entry(MAX98388_R2033_SPK_MON_DURATION, 0x02),
        reg_default_entry(MAX98388_R2037_ERR_MON_CTRL, 0x01),
        reg_default_entry(MAX98388_R2040_PCM_MODE_CFG, 0xC0),
        reg_default_entry(MAX98388_R2041_PCM_CLK_SETUP, 0x04),
        reg_default_entry(MAX98388_R2042_PCM_SR_SETUP, 0x88),
        reg_default_entry(MAX98388_R2044_PCM_TX_CTRL1, 0x00),
        reg_default_entry(MAX98388_R2045_PCM_TX_CTRL2, 0x00),
        reg_default_entry(MAX98388_R2050_PCM_TX_HIZ_CTRL1, 0xFF),
        reg_default_entry(MAX98388_R2051_PCM_TX_HIZ_CTRL2, 0xFF),
        reg_default_entry(MAX98388_R2052_PCM_TX_HIZ_CTRL3, 0xFF),
        reg_default_entry(MAX98388_R2053_PCM_TX_HIZ_CTRL4, 0xFF),
        reg_default_entry(MAX98388_R2054_PCM_TX_HIZ_CTRL5, 0xFF),
        reg_default_entry(MAX98388_R2055_PCM_TX_HIZ_CTRL6, 0xFF),
        reg_default_entry(MAX98388_R2056_PCM_TX_HIZ_CTRL7, 0xFF),
        reg_default_entry(MAX98388_R2057_PCM_TX_HIZ_CTRL8, 0xFF),
        reg_default_entry(MAX98388_R2058_PCM_RX_SRC1, 0x00),
        reg_default_entry(MAX98388_R2059_PCM_RX_SRC2, 0x01),
        reg_default_entry(MAX98388_R205C_PCM_TX_DRIVE_STRENGTH, 0x00),
        reg_default_entry(MAX98388_R205D_PCM_TX_SRC_EN, 0x00),
        reg_default_entry(MAX98388_R205E_PCM_RX_EN, 0x00),
        reg_default_entry(MAX98388_R205F_PCM_TX_EN, 0x00),
        reg_default_entry(MAX98388_R2090_SPK_CH_VOL_CTRL, 0x00),
        reg_default_entry(MAX98388_R2091_SPK_CH_CFG, 0x02),
        reg_default_entry(MAX98388_R2092_SPK_AMP_OUT_CFG, 0x03),
        reg_default_entry(MAX98388_R2093_SPK_AMP_SSM_CFG, 0x01),
        reg_default_entry(MAX98388_R2094_SPK_AMP_ER_CTRL, 0x00),
        reg_default_entry(MAX98388_R209E_SPK_CH_PINK_NOISE_EN, 0x00),
        reg_default_entry(MAX98388_R209F_SPK_CH_AMP_EN, 0x00),
        reg_default_entry(MAX98388_R20A0_IV_DATA_DSP_CTRL, 0x10),
        reg_default_entry(MAX98388_R20A7_IV_DATA_EN, 0x00),
        reg_default_entry(MAX98388_R20E0_BP_ALC_THRESH, 0x04),
        reg_default_entry(MAX98388_R20E1_BP_ALC_RATES, 0x20),
        reg_default_entry(MAX98388_R20E2_BP_ALC_ATTEN, 0x06),
        reg_default_entry(MAX98388_R20E3_BP_ALC_REL, 0x02),
        reg_default_entry(MAX98388_R20E4_BP_ALC_MUTE, 0x33),
        reg_default_entry(MAX98388_R20EE_BP_INF_HOLD_REL, 0x00),
        reg_default_entry(MAX98388_R20EF_BP_ALC_EN, 0x00),
        reg_default_entry(MAX98388_R210E_AUTO_RESTART, 0x00),
        reg_default_entry(MAX98388_R210F_GLOBAL_EN, 0x00),
        reg_default_entry(MAX98388_R22FF_REV_ID, 0x00),
    ]
};

unsafe extern "C" fn max98388_dac_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let max98388 = snd_soc_component_get_drvdata(component) as *mut max98388_priv;

    if event == SND_SOC_DAPM_POST_PMU {
        regmap_write((*max98388).regmap, MAX98388_R210F_GLOBAL_EN, 1);
        usleep_range(30000, 31000);
    } else if event == SND_SOC_DAPM_PRE_PMD {
        regmap_write((*max98388).regmap, MAX98388_R210F_GLOBAL_EN, 0);
        usleep_range(30000, 31000);
        (*max98388).tdm_mode = false;
    } else {
        return 0;
    }
    0
}

static max98388_monomix_switch_text: [&[u8]; 3] = [b"Left\0", b"Right\0", b"LeftRight\0"];

// SOC_ENUM_SINGLE, SOC_DAPM_ENUM, SOC_DAPM_SINGLE, SND_SOC_DAPM_* widget,
// DECLARE_TLV_DB_SCALE, SOC_ENUM_SINGLE_DECL, SOC_SINGLE*, SOC_ENUM and route
// initializers depend on ALSA C macro-expanded struct layouts. Their source
// definitions are preserved below as translation notes for the generated Rust item
// equivalents:
// dai_sel_enum = SOC_ENUM_SINGLE(MAX98388_R2058_PCM_RX_SRC1,
//     MAX98388_PCM_TO_SPK_MONOMIX_CFG_SHIFT, 3, max98388_monomix_switch_text);
// max98388_dai_controls = SOC_DAPM_ENUM("DAI Sel", dai_sel_enum);
// max98388_vi_control = SOC_DAPM_SINGLE("Switch", MAX98388_R205F_PCM_TX_EN, 0, 1, 0);
// max98388_dapm_widgets[] = { SND_SOC_DAPM_DAC_E(...), SND_SOC_DAPM_MUX(...),
//     SND_SOC_DAPM_OUTPUT(...), SND_SOC_DAPM_AIF_OUT(...), SND_SOC_DAPM_ADC(...),
//     SND_SOC_DAPM_SWITCH(...), SND_SOC_DAPM_SIGGEN(...) };
// max98388_digital_tlv = DECLARE_TLV_DB_SCALE(-6350, 50, 1);
// max98388_amp_gain_tlv = DECLARE_TLV_DB_SCALE(-300, 300, 0);

static max98388_alc_max_atten_text: [&[u8]; 16] = [
    b"0dBFS\0", b"-1dBFS\0", b"-2dBFS\0", b"-3dBFS\0", b"-4dBFS\0", b"-5dBFS\0",
    b"-6dBFS\0", b"-7dBFS\0", b"-8dBFS\0", b"-9dBFS\0", b"-10dBFS\0", b"-11dBFS\0",
    b"-12dBFS\0", b"-13dBFS\0", b"-14dBFS\0", b"-15dBFS\0",
];
static max98388_thermal_warn_text: [&[u8]; 4] = [b"95C\0", b"105C\0", b"115C\0", b"125C\0"];
static max98388_thermal_shutdown_text: [&[u8]; 4] = [b"135C\0", b"145C\0", b"155C\0", b"165C\0"];
static max98388_alc_thresh_single_text: [&[u8]; 16] = [
    b"3.625V\0", b"3.550V\0", b"3.475V\0", b"3.400V\0", b"3.325V\0", b"3.250V\0",
    b"3.175V\0", b"3.100V\0", b"3.025V\0", b"2.950V\0", b"2.875V\0", b"2.800V\0",
    b"2.725V\0", b"2.650V\0", b"2.575V\0", b"2.500V\0",
];
static max98388_alc_attack_rate_text: [&[u8]; 16] = [
    b"0\0", b"10us\0", b"20us\0", b"40us\0", b"80us\0", b"160us\0", b"320us\0",
    b"640us\0", b"1.28ms\0", b"2.56ms\0", b"5.12ms\0", b"10.24ms\0",
    b"20.48ms\0", b"40.96ms\0", b"81.92ms\0", b"163.84ms\0",
];
static max98388_alc_release_rate_text: [&[u8]; 16] = [
    b"20us\0", b"40us\0", b"80us\0", b"160us\0", b"320us\0", b"640us\0",
    b"1.28ms\0", b"2.56ms\0", b"5.12ms\0", b"10.24ms\0", b"20.48ms\0",
    b"40.96ms\0", b"81.92ms\0", b"163.84ms\0", b"327.68ms\0", b"655.36ms\0",
];
static max98388_alc_debounce_text: [&[u8]; 8] = [
    b"0.01ms\0", b"0.1ms\0", b"1ms\0", b"10ms\0", b"100ms\0", b"250ms\0",
    b"500ms\0", b"hold\0",
];
static max98388_alc_mute_delay_text: [&[u8]; 8] = [
    b"0.01ms\0", b"0.05ms\0", b"0.1ms\0", b"0.5ms\0", b"1ms\0", b"5ms\0",
    b"25ms\0", b"250ms\0",
];
static max98388_spkmon_duration_text: [&[u8]; 16] = [
    b"10ms\0", b"25ms\0", b"50ms\0", b"75ms\0", b"100ms\0", b"200ms\0",
    b"300ms\0", b"400ms\0", b"500ms\0", b"600ms\0", b"700ms\0", b"800ms\0",
    b"900ms\0", b"1000ms\0", b"1100ms\0", b"1200ms\0",
];
static max98388_spkmon_thresh_text: [&[u8]; 128] = [
    b"0.03V\0", b"0.06V\0", b"0.09V\0", b"0.12V\0", b"0.15V\0", b"0.18V\0", b"0.20V\0", b"0.23V\0",
    b"0.26V\0", b"0.29V\0", b"0.32V\0", b"0.35V\0", b"0.38V\0", b"0.41V\0", b"0.44V\0", b"0.47V\0",
    b"0.50V\0", b"0.53V\0", b"0.56V\0", b"0.58V\0", b"0.61V\0", b"0.64V\0", b"0.67V\0", b"0.70V\0",
    b"0.73V\0", b"0.76V\0", b"0.79V\0", b"0.82V\0", b"0.85V\0", b"0.88V\0", b"0.91V\0", b"0.94V\0",
    b"0.96V\0", b"0.99V\0", b"1.02V\0", b"1.05V\0", b"1.08V\0", b"1.11V\0", b"1.14V\0", b"1.17V\0",
    b"1.20V\0", b"1.23V\0", b"1.26V\0", b"1.29V\0", b"1.32V\0", b"1.35V\0", b"1.37V\0", b"1.40V\0",
    b"1.43V\0", b"1.46V\0", b"1.49V\0", b"1.52V\0", b"1.55V\0", b"1.58V\0", b"1.61V\0", b"1.64V\0",
    b"1.67V\0", b"1.70V\0", b"1.73V\0", b"1.75V\0", b"1.78V\0", b"1.81V\0", b"1.84V\0", b"1.87V\0",
    b"1.90V\0", b"1.93V\0", b"1.96V\0", b"1.99V\0", b"2.02V\0", b"2.05V\0", b"2.08V\0", b"2.11V\0",
    b"2.13V\0", b"2.16V\0", b"2.19V\0", b"2.22V\0", b"2.25V\0", b"2.28V\0", b"2.31V\0", b"2.34V\0",
    b"2.37V\0", b"2.40V\0", b"2.43V\0", b"2.46V\0", b"2.49V\0", b"2.51V\0", b"2.54V\0", b"2.57V\0",
    b"2.60V\0", b"2.63V\0", b"2.66V\0", b"2.69V\0", b"2.72V\0", b"2.75V\0", b"2.78V\0", b"2.81V\0",
    b"2.84V\0", b"2.87V\0", b"2.89V\0", b"2.92V\0", b"2.95V\0", b"2.98V\0", b"3.01V\0", b"3.04V\0",
    b"3.07V\0", b"3.10V\0", b"3.13V\0", b"3.16V\0", b"3.19V\0", b"3.22V\0", b"3.25V\0", b"3.27V\0",
    b"3.30V\0", b"3.33V\0", b"3.36V\0", b"3.39V\0", b"3.42V\0", b"3.45V\0", b"3.48V\0", b"3.51V\0",
    b"3.54V\0", b"3.57V\0", b"3.60V\0", b"3.63V\0", b"3.66V\0", b"3.68V\0", b"3.71V\0", b"3.74V\0",
];
static max98388_spkmon_load_text: [&[u8]; 128] = [
    b"2.00ohm\0", b"2.25ohm\0", b"2.50ohm\0", b"2.75ohm\0", b"3.00ohm\0", b"3.25ohm\0",
    b"3.50ohm\0", b"3.75ohm\0", b"4.00ohm\0", b"4.25ohm\0", b"4.50ohm\0", b"4.75ohm\0",
    b"5.00ohm\0", b"5.25ohm\0", b"5.50ohm\0", b"5.75ohm\0", b"6.00ohm\0", b"6.25ohm\0",
    b"6.50ohm\0", b"6.75ohm\0", b"7.00ohm\0", b"7.25ohm\0", b"7.50ohm\0", b"7.75ohm\0",
    b"8.00ohm\0", b"8.25ohm\0", b"8.50ohm\0", b"8.75ohm\0", b"9.00ohm\0", b"9.25ohm\0",
    b"9.50ohm\0", b"9.75ohm\0", b"10.00ohm\0", b"10.25ohm\0", b"10.50ohm\0", b"10.75ohm\0",
    b"11.00ohm\0", b"11.25ohm\0", b"11.50ohm\0", b"11.75ohm\0", b"12.00ohm\0", b"12.25ohm\0",
    b"12.50ohm\0", b"12.75ohm\0", b"13.00ohm\0", b"13.25ohm\0", b"13.50ohm\0", b"13.75ohm\0",
    b"14.00ohm\0", b"14.25ohm\0", b"14.50ohm\0", b"14.75ohm\0", b"15.00ohm\0", b"15.25ohm\0",
    b"15.50ohm\0", b"15.75ohm\0", b"16.00ohm\0", b"16.25ohm\0", b"16.50ohm\0", b"16.75ohm\0",
    b"17.00ohm\0", b"17.25ohm\0", b"17.50ohm\0", b"17.75ohm\0", b"18.00ohm\0", b"18.25ohm\0",
    b"18.50ohm\0", b"18.75ohm\0", b"19.00ohm\0", b"19.25ohm\0", b"19.50ohm\0", b"19.75ohm\0",
    b"20.00ohm\0", b"20.25ohm\0", b"20.50ohm\0", b"20.75ohm\0", b"21.00ohm\0", b"21.25ohm\0",
    b"21.50ohm\0", b"21.75ohm\0", b"22.00ohm\0", b"22.25ohm\0", b"22.50ohm\0", b"22.75ohm\0",
    b"23.00ohm\0", b"23.25ohm\0", b"23.50ohm\0", b"23.75ohm\0", b"24.00ohm\0", b"24.25ohm\0",
    b"24.50ohm\0", b"24.75ohm\0", b"25.00ohm\0", b"25.25ohm\0", b"25.50ohm\0", b"25.75ohm\0",
    b"26.00ohm\0", b"26.25ohm\0", b"26.50ohm\0", b"26.75ohm\0", b"27.00ohm\0", b"27.25ohm\0",
    b"27.50ohm\0", b"27.75ohm\0", b"28.00ohm\0", b"28.25ohm\0", b"28.50ohm\0", b"28.75ohm\0",
    b"29.00ohm\0", b"29.25ohm\0", b"29.50ohm\0", b"29.75ohm\0", b"30.00ohm\0", b"30.25ohm\0",
    b"30.50ohm\0", b"30.75ohm\0", b"31.00ohm\0", b"31.25ohm\0", b"31.50ohm\0", b"31.75ohm\0",
    b"32.00ohm\0", b"32.25ohm\0", b"32.50ohm\0", b"32.75ohm\0", b"33.00ohm\0", b"33.25ohm\0",
    b"33.50ohm\0", b"33.75ohm\0",
];
static max98388_edge_rate_text: [&[u8]; 4] = [b"Normal\0", b"Reduced\0", b"Maximum\0", b"Increased\0"];
static max98388_ssm_mod_text: [&[u8]; 4] = [b"1.5%\0", b"3.0%\0", b"4.5%\0", b"6.0%\0"];

unsafe extern "C" fn max98388_reset(max98388: *mut max98388_priv, dev: *mut device) {
    let mut ret: c_int;
    let mut reg: c_int = 0;
    let mut count: c_int;

    /* Software Reset */
    ret = regmap_update_bits(
        (*max98388).regmap,
        MAX98388_R2000_SW_RESET,
        MAX98388_SOFT_RESET,
        MAX98388_SOFT_RESET,
    );
    if ret != 0 {
        dev_err(dev, b"Reset command failed. (ret:%d)\n\0".as_ptr() as *const c_char, ret);
    }

    count = 0;
    while count < 3 {
        usleep_range(10000, 11000);
        /* Software Reset Verification */
        ret = regmap_read((*max98388).regmap, MAX98388_R22FF_REV_ID, &mut reg);
        if ret == 0 {
            dev_info(dev, b"Reset completed (retry:%d)\n\0".as_ptr() as *const c_char, count);
            return;
        }
        count += 1;
    }
    dev_err(dev, b"Reset failed. (ret:%d)\n\0".as_ptr() as *const c_char, ret);
}

unsafe extern "C" fn max98388_probe(component: *mut snd_soc_component) -> c_int {
    let max98388 = snd_soc_component_get_drvdata(component) as *mut max98388_priv;

    /* Software Reset */
    max98388_reset(max98388, (*component).dev);

    /* General channel source configuration */
    regmap_write((*max98388).regmap, MAX98388_R2059_PCM_RX_SRC2, 0x10);

    /* Enable DC blocker */
    regmap_write((*max98388).regmap, MAX98388_R2091_SPK_CH_CFG, 0x1);
    /* Enable IMON VMON DC blocker */
    regmap_write((*max98388).regmap, MAX98388_R20A0_IV_DATA_DSP_CTRL, 0x3);
    /* TX slot configuration */
    regmap_write((*max98388).regmap, MAX98388_R2044_PCM_TX_CTRL1, (*max98388).v_slot);

    regmap_write((*max98388).regmap, MAX98388_R2045_PCM_TX_CTRL2, (*max98388).i_slot);
    /* Enable Auto-restart behavior by default */
    regmap_write((*max98388).regmap, MAX98388_R210E_AUTO_RESTART, 0xF);
    /* Set interleave mode */
    if (*max98388).interleave_mode {
        regmap_update_bits(
            (*max98388).regmap,
            MAX98388_R2040_PCM_MODE_CFG,
            MAX98388_PCM_TX_CH_INTERLEAVE_MASK,
            MAX98388_PCM_TX_CH_INTERLEAVE_MASK,
        );
    }

    /* Speaker Amplifier Channel Enable */
    regmap_update_bits(
        (*max98388).regmap,
        MAX98388_R209F_SPK_CH_AMP_EN,
        MAX98388_SPK_EN_MASK,
        1,
    );

    0
}

unsafe extern "C" fn max98388_dai_set_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let max98388 = snd_soc_component_get_drvdata(component) as *mut max98388_priv;
    let mut format: c_uint = 0;
    let mut invert: c_uint = 0;

    dev_dbg((*component).dev, b"%s: fmt 0x%08X\n\0".as_ptr() as *const c_char, b"max98388_dai_set_fmt\0".as_ptr(), fmt);

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF => {}
        x if x == SND_SOC_DAIFMT_IB_NF => {
            invert = MAX98388_PCM_MODE_CFG_PCM_BCLKEDGE;
        }
        _ => {
            dev_err((*component).dev, b"DAI invert mode unsupported\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    regmap_update_bits(
        (*max98388).regmap,
        MAX98388_R2041_PCM_CLK_SETUP,
        MAX98388_PCM_MODE_CFG_PCM_BCLKEDGE,
        invert,
    );

    /* interface format */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => format = MAX98388_PCM_FORMAT_I2S,
        x if x == SND_SOC_DAIFMT_LEFT_J => format = MAX98388_PCM_FORMAT_LJ,
        x if x == SND_SOC_DAIFMT_DSP_A => format = MAX98388_PCM_FORMAT_TDM_MODE1,
        x if x == SND_SOC_DAIFMT_DSP_B => format = MAX98388_PCM_FORMAT_TDM_MODE0,
        _ => return -EINVAL,
    }

    regmap_update_bits(
        (*max98388).regmap,
        MAX98388_R2040_PCM_MODE_CFG,
        MAX98388_PCM_MODE_CFG_FORMAT_MASK,
        format << MAX98388_PCM_MODE_CFG_FORMAT_SHIFT,
    );

    0
}

/* BCLKs per LRCLK */
static bclk_sel_table: [c_int; 10] = [32, 48, 64, 96, 128, 192, 256, 384, 512, 320];

fn max98388_get_bclk_sel(bclk: c_int) -> c_int {
    /* match BCLKs per LRCLK */
    for i in 0..bclk_sel_table.len() {
        if bclk_sel_table[i] == bclk {
            return i as c_int + 2;
        }
    }
    0
}

unsafe extern "C" fn max98388_set_clock(
    component: *mut snd_soc_component,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let max98388 = snd_soc_component_get_drvdata(component) as *mut max98388_priv;
    /* BCLK/LRCLK ratio calculation */
    let blr_clk_ratio: c_int = params_channels(params) * (*max98388).ch_size;
    let value: c_int;

    if !(*max98388).tdm_mode {
        /* BCLK configuration */
        value = max98388_get_bclk_sel(blr_clk_ratio);
        if value == 0 {
            dev_err(
                (*component).dev,
                b"format unsupported %d\n\0".as_ptr() as *const c_char,
                params_format(params),
            );
            return -EINVAL;
        }

        regmap_update_bits(
            (*max98388).regmap,
            MAX98388_R2041_PCM_CLK_SETUP,
            MAX98388_PCM_CLK_SETUP_BSEL_MASK,
            value as c_uint,
        );
    }
    0
}

unsafe extern "C" fn max98388_dai_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let max98388 = snd_soc_component_get_drvdata(component) as *mut max98388_priv;
    let mut sampling_rate: c_uint = 0;
    let mut chan_sz: c_uint = 0;
    let mut ret: c_int;
    let mut reg: c_int = 0;
    let mut status: c_int = 0;

    /* pcm mode configuration */
    match snd_pcm_format_width(params_format(params)) {
        16 => chan_sz = MAX98388_PCM_MODE_CFG_CHANSZ_16,
        24 => chan_sz = MAX98388_PCM_MODE_CFG_CHANSZ_24,
        32 => chan_sz = MAX98388_PCM_MODE_CFG_CHANSZ_32,
        _ => {
            dev_err(
                (*component).dev,
                b"format unsupported %d\n\0".as_ptr() as *const c_char,
                params_format(params),
            );
            return -EINVAL;
        }
    }

    (*max98388).ch_size = snd_pcm_format_width(params_format(params));

    ret = regmap_read((*max98388).regmap, MAX98388_R2040_PCM_MODE_CFG, &mut reg);
    if ret < 0 {
        return -EINVAL;
    }

    /* GLOBAL_EN OFF prior to the channel size re-configure */
    if chan_sz != ((reg as c_uint) & MAX98388_PCM_MODE_CFG_CHANSZ_MASK) {
        ret = regmap_read((*max98388).regmap, MAX98388_R210F_GLOBAL_EN, &mut status);
        if ret < 0 {
            return -EINVAL;
        }

        if status != 0 {
            regmap_write((*max98388).regmap, MAX98388_R210F_GLOBAL_EN, 0);
            usleep_range(30000, 31000);
        }
        regmap_update_bits(
            (*max98388).regmap,
            MAX98388_R2040_PCM_MODE_CFG,
            MAX98388_PCM_MODE_CFG_CHANSZ_MASK,
            chan_sz,
        );
    }
    dev_dbg(
        (*component).dev,
        b"format supported %d\0".as_ptr() as *const c_char,
        params_format(params),
    );

    /* sampling rate configuration */
    match params_rate(params) {
        8000 => sampling_rate = MAX98388_PCM_SR_8000,
        11025 => sampling_rate = MAX98388_PCM_SR_11025,
        12000 => sampling_rate = MAX98388_PCM_SR_12000,
        16000 => sampling_rate = MAX98388_PCM_SR_16000,
        22050 => sampling_rate = MAX98388_PCM_SR_22050,
        24000 => sampling_rate = MAX98388_PCM_SR_24000,
        32000 => sampling_rate = MAX98388_PCM_SR_32000,
        44100 => sampling_rate = MAX98388_PCM_SR_44100,
        48000 => sampling_rate = MAX98388_PCM_SR_48000,
        88200 => sampling_rate = MAX98388_PCM_SR_88200,
        96000 => sampling_rate = MAX98388_PCM_SR_96000,
        _ => {
            dev_err(
                (*component).dev,
                b"rate %d not supported\n\0".as_ptr() as *const c_char,
                params_rate(params),
            );
            return -EINVAL;
        }
    }

    /* set DAI_SR to correct LRCLK frequency */
    regmap_update_bits(
        (*max98388).regmap,
        MAX98388_R2042_PCM_SR_SETUP,
        MAX98388_PCM_SR_MASK,
        sampling_rate,
    );

    /* set sampling rate of IV */
    if (*max98388).interleave_mode && sampling_rate > MAX98388_PCM_SR_16000 {
        regmap_update_bits(
            (*max98388).regmap,
            MAX98388_R2042_PCM_SR_SETUP,
            MAX98388_PCM_SR_IV_MASK,
            (sampling_rate - 3) << MAX98388_PCM_SR_IV_SHIFT,
        );
    } else {
        regmap_update_bits(
            (*max98388).regmap,
            MAX98388_R2042_PCM_SR_SETUP,
            MAX98388_PCM_SR_IV_MASK,
            sampling_rate << MAX98388_PCM_SR_IV_SHIFT,
        );
    }

    ret = max98388_set_clock(component, params);

    if status != 0 {
        regmap_write((*max98388).regmap, MAX98388_R210F_GLOBAL_EN, 1);
        usleep_range(30000, 31000);
    }

    ret
}

const MAX_NUM_SLOTS: c_int = 16;
const MAX_NUM_CH: c_int = 2;

unsafe extern "C" fn max98388_dai_tdm_slot(
    dai: *mut snd_soc_dai,
    tx_mask: c_uint,
    rx_mask: c_uint,
    slots: c_int,
    slot_width: c_int,
) -> c_int {
    let component = (*dai).component;
    let max98388 = snd_soc_component_get_drvdata(component) as *mut max98388_priv;
    let mut bsel: c_int = 0;
    let mut chan_sz: c_uint = 0;
    let mut mask: c_uint;
    let mut cnt: c_int;
    let mut slot_found: c_int;
    let mut addr: c_uint;
    let mut bits: c_uint;

    if tx_mask == 0 && rx_mask == 0 && slots == 0 && slot_width == 0 {
        (*max98388).tdm_mode = false;
    } else {
        (*max98388).tdm_mode = true;
    }

    /* BCLK configuration */
    bsel = max98388_get_bclk_sel(slots * slot_width);
    if bsel == 0 {
        dev_err(
            (*component).dev,
            b"BCLK %d not supported\n\0".as_ptr() as *const c_char,
            slots * slot_width,
        );
        return -EINVAL;
    }

    regmap_update_bits(
        (*max98388).regmap,
        MAX98388_R2041_PCM_CLK_SETUP,
        MAX98388_PCM_CLK_SETUP_BSEL_MASK,
        bsel as c_uint,
    );

    /* Channel size configuration */
    match slot_width {
        16 => chan_sz = MAX98388_PCM_MODE_CFG_CHANSZ_16,
        24 => chan_sz = MAX98388_PCM_MODE_CFG_CHANSZ_24,
        32 => chan_sz = MAX98388_PCM_MODE_CFG_CHANSZ_32,
        _ => {
            dev_err(
                (*component).dev,
                b"format unsupported %d\n\0".as_ptr() as *const c_char,
                slot_width,
            );
            return -EINVAL;
        }
    }

    regmap_update_bits(
        (*max98388).regmap,
        MAX98388_R2040_PCM_MODE_CFG,
        MAX98388_PCM_MODE_CFG_CHANSZ_MASK,
        chan_sz,
    );

    /* Rx slot configuration */
    slot_found = 0;
    mask = rx_mask;
    cnt = 0;
    while cnt < MAX_NUM_SLOTS {
        if (mask & 0x1) != 0 {
            if slot_found == 0 {
                regmap_update_bits(
                    (*max98388).regmap,
                    MAX98388_R2059_PCM_RX_SRC2,
                    MAX98388_RX_SRC_CH0_SHIFT,
                    cnt as c_uint,
                );
            } else {
                regmap_update_bits(
                    (*max98388).regmap,
                    MAX98388_R2059_PCM_RX_SRC2,
                    MAX98388_RX_SRC_CH1_SHIFT,
                    cnt as c_uint,
                );
            }
            slot_found += 1;
            if slot_found >= MAX_NUM_CH {
                break;
            }
        }
        cnt += 1;
        mask >>= 1;
    }

    /* speaker feedback slot configuration */
    slot_found = 0;
    mask = tx_mask;
    cnt = 0;
    while cnt < MAX_NUM_SLOTS {
        if (mask & 0x1) != 0 {
            addr = MAX98388_R2044_PCM_TX_CTRL1 + (cnt as c_uint / 8);
            bits = (cnt as c_uint) % 8;
            regmap_update_bits((*max98388).regmap, addr, bits, bits);
            slot_found += 1;
            if slot_found >= MAX_NUM_CH {
                break;
            }
        }
        cnt += 1;
        mask >>= 1;
    }

    0
}

// MAX98388_RATES = SNDRV_PCM_RATE_8000_96000
// MAX98388_FORMATS = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE
// max98388_dai_ops = { .set_fmt = max98388_dai_set_fmt,
//     .hw_params = max98388_dai_hw_params, .set_tdm_slot = max98388_dai_tdm_slot };

unsafe extern "C" fn max98388_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    (reg >= MAX98388_R2001_INT_RAW1 && reg <= MAX98388_R2002_INT_RAW2)
        || (reg >= MAX98388_R2004_INT_STATE1 && reg <= MAX98388_R2005_INT_STATE2)
        || reg == MAX98388_R2020_THERM_WARN_THRESH
        || (reg >= MAX98388_R2031_SPK_MON_THRESH && reg <= MAX98388_R2033_SPK_MON_DURATION)
        || reg == MAX98388_R2037_ERR_MON_CTRL
        || (reg >= MAX98388_R2040_PCM_MODE_CFG && reg <= MAX98388_R2042_PCM_SR_SETUP)
        || (reg >= MAX98388_R2044_PCM_TX_CTRL1 && reg <= MAX98388_R2045_PCM_TX_CTRL2)
        || (reg >= MAX98388_R2050_PCM_TX_HIZ_CTRL1 && reg <= MAX98388_R2059_PCM_RX_SRC2)
        || (reg >= MAX98388_R205C_PCM_TX_DRIVE_STRENGTH && reg <= MAX98388_R205F_PCM_TX_EN)
        || (reg >= MAX98388_R2090_SPK_CH_VOL_CTRL && reg <= MAX98388_R2094_SPK_AMP_ER_CTRL)
        || (reg >= MAX98388_R209E_SPK_CH_PINK_NOISE_EN && reg <= MAX98388_R209F_SPK_CH_AMP_EN)
        || reg == MAX98388_R20A0_IV_DATA_DSP_CTRL
        || reg == MAX98388_R20A7_IV_DATA_EN
        || (reg >= MAX98388_R20E0_BP_ALC_THRESH && reg <= MAX98388_R20E4_BP_ALC_MUTE)
        || (reg >= MAX98388_R20EE_BP_INF_HOLD_REL && reg <= MAX98388_R20EF_BP_ALC_EN)
        || reg == MAX98388_R210E_AUTO_RESTART
        || reg == MAX98388_R210F_GLOBAL_EN
        || reg == MAX98388_R22FF_REV_ID
}

unsafe extern "C" fn max98388_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    (reg >= MAX98388_R2001_INT_RAW1 && reg <= MAX98388_R2005_INT_STATE2)
        || reg == MAX98388_R210F_GLOBAL_EN
        || reg == MAX98388_R22FF_REV_ID
}

// max98388_dai[] = { { .name = "max98388-aif1", playback/capture stream data,
//     .ops = &max98388_dai_ops } };

unsafe extern "C" fn max98388_suspend(dev: *mut device) -> c_int {
    let max98388 = dev_get_drvdata(dev) as *mut max98388_priv;

    regcache_cache_only((*max98388).regmap, true);
    regcache_mark_dirty((*max98388).regmap);

    0
}

unsafe extern "C" fn max98388_resume(dev: *mut device) -> c_int {
    let max98388 = dev_get_drvdata(dev) as *mut max98388_priv;
    let ret: c_int;

    regcache_cache_only((*max98388).regmap, false);
    max98388_reset(max98388, dev);
    ret = regcache_sync((*max98388).regmap);
    if ret != 0 {
        regcache_cache_only((*max98388).regmap, true);
        regcache_mark_dirty((*max98388).regmap);
        return ret;
    }

    0
}

// max98388_pm = { SYSTEM_SLEEP_PM_OPS(max98388_suspend, max98388_resume) };
// max98388_regmap = { .reg_bits = 16, .val_bits = 8,
//     .max_register = MAX98388_R22FF_REV_ID, .reg_defaults = max98388_reg,
//     .num_reg_defaults = ARRAY_SIZE(max98388_reg),
//     .readable_reg = max98388_readable_register,
//     .volatile_reg = max98388_volatile_reg, .cache_type = REGCACHE_RBTREE };
// soc_codec_dev_max98388 = { .probe = max98388_probe, .controls = max98388_snd_controls,
//     .dapm_widgets = max98388_dapm_widgets, .dapm_routes = max98388_audio_map,
//     .use_pmdown_time = 1, .endianness = 1 };

unsafe extern "C" fn max98388_read_deveice_property(
    dev: *mut device,
    max98388: *mut max98388_priv,
) {
    let mut value: c_int = 0;

    if device_property_read_u32(dev, b"adi,vmon-slot-no\0".as_ptr() as *const c_char, &mut value) == 0 {
        (*max98388).v_slot = (value & 0xF) as c_uint;
    } else {
        (*max98388).v_slot = 0;
    }

    if device_property_read_u32(dev, b"adi,imon-slot-no\0".as_ptr() as *const c_char, &mut value) == 0 {
        (*max98388).i_slot = (value & 0xF) as c_uint;
    } else {
        (*max98388).i_slot = 1;
    }

    if device_property_read_bool(dev, b"adi,interleave-mode\0".as_ptr() as *const c_char) {
        (*max98388).interleave_mode = true;
    } else {
        (*max98388).interleave_mode = false;
    }
}

unsafe extern "C" fn max98388_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let mut ret: c_int = 0;
    let mut reg: c_int = 0;

    let mut max98388: *mut max98388_priv = core::ptr::null_mut();

    max98388 = devm_kzalloc(
        &mut (*i2c).dev,
        core::mem::size_of::<max98388_priv>(),
        GFP_KERNEL,
    ) as *mut max98388_priv;
    if max98388.is_null() {
        return -ENOMEM;
    }

    i2c_set_clientdata(i2c, max98388 as *mut c_void);

    /* regmap initialization */
    // The concrete regmap_config object is macro/struct-layout dependent.
    unsafe extern "C" {
        static max98388_regmap: c_void;
        static soc_codec_dev_max98388: c_void;
        static mut max98388_dai: c_void;
    }
    (*max98388).regmap = devm_regmap_init_i2c(i2c, &max98388_regmap as *const c_void);
    if IS_ERR((*max98388).regmap as *const c_void) {
        return dev_err_probe(
            &mut (*i2c).dev,
            PTR_ERR((*max98388).regmap as *const c_void),
            b"Failed to allocate register map.\n\0".as_ptr() as *const c_char,
        );
    }

    /* voltage/current slot & gpio configuration */
    max98388_read_deveice_property(&mut (*i2c).dev, max98388);

    /* Device Reset */
    (*max98388).reset_gpio = devm_gpiod_get_optional(
        &mut (*i2c).dev,
        b"reset\0".as_ptr() as *const c_char,
        GPIOD_OUT_HIGH,
    );
    if IS_ERR((*max98388).reset_gpio as *const c_void) {
        return dev_err_probe(
            &mut (*i2c).dev,
            PTR_ERR((*max98388).reset_gpio as *const c_void),
            b"Unable to request GPIO\n\0".as_ptr() as *const c_char,
        );
    }

    if !(*max98388).reset_gpio.is_null() {
        usleep_range(5000, 6000);
        gpiod_set_value_cansleep((*max98388).reset_gpio, 0);
        /* Wait for the hw reset done */
        usleep_range(5000, 6000);
    }

    /* Read Revision ID */
    ret = regmap_read((*max98388).regmap, MAX98388_R22FF_REV_ID, &mut reg);
    if ret < 0 {
        return dev_err_probe(
            &mut (*i2c).dev,
            ret,
            b"Failed to read the revision ID\n\0".as_ptr() as *const c_char,
        );
    }

    dev_info(
        &mut (*i2c).dev,
        b"MAX98388 revisionID: 0x%02X\n\0".as_ptr() as *const c_char,
        reg,
    );

    /* codec registration */
    ret = devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &soc_codec_dev_max98388 as *const c_void,
        &mut max98388_dai as *mut c_void,
        1,
    );
    if ret < 0 {
        dev_err(
            &mut (*i2c).dev,
            b"Failed to register codec: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
    }

    ret
}

// max98388_i2c_id[] = { { .name = "max98388" }, { } };
// MODULE_DEVICE_TABLE(i2c, max98388_i2c_id);
// max98388_of_match[] = { { .compatible = "adi,max98388" }, { } };
// MODULE_DEVICE_TABLE(of, max98388_of_match);
// max98388_acpi_match[] = { { "ADS8388", 0 }, {} };
// MODULE_DEVICE_TABLE(acpi, max98388_acpi_match);
// max98388_i2c_driver = { .driver = { .name = "max98388",
//     .of_match_table = max98388_of_match,
//     .acpi_match_table = max98388_acpi_match, .pm = pm_sleep_ptr(&max98388_pm) },
//     .probe = max98388_i2c_probe, .id_table = max98388_i2c_id };
// module_i2c_driver(max98388_i2c_driver)
// MODULE_DESCRIPTION("ALSA SoC MAX98388 driver");
// MODULE_AUTHOR("Ryan Lee <ryans.lee@analog.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
