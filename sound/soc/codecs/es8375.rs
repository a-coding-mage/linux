// SPDX-License-Identifier: GPL-2.0-only
/*
 * es8375.rs  --  ES8375 ALSA SoC Audio Codec
 *
 * Copyright Everest Semiconductor Co., Ltd
 *
 * Authors:  Michael Zhang (zhangyi@everest-semi.com)
 *
 * Rust translation of soc/codecs/es8375.c. C include dependencies are expected
 * to be supplied by the surrounding kernel/ASoC binding layer.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type u8 = u8;
type u16 = u16;
type u32 = u32;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_MAPLE: c_uint = 0;

/* Constants from es8375.h and Linux/ASoC headers are external dependencies. */
extern "C" {
    static ES8375_ADC2: c_uint;
    static ES8375_DAC2: c_uint;
    static ES8375_ADC_AUTOMUTE: c_uint;
    static ES8375_DAC_AUTOMUTE: c_uint;
    static ES8375_ADC1: c_uint;
    static ES8375_HPF1: c_uint;
    static ES8375_ADC_OSR_GAIN: c_uint;
    static ES8375_ADC_VOLUME: c_uint;
    static ES8375_DAC1: c_uint;
    static ES8375_DAC_VOLUME: c_uint;
    static ES8375_DAC_VPPSCALE: c_uint;
    static ES8375_DAC_AUTOMUTE1: c_uint;
    static ES8375_SDP: c_uint;
    static ES8375_SDP2: c_uint;
    static ES8375_MCLK_SEL: c_uint;
    static ES8375_CLK_MGR2: c_uint;
    static ES8375_CLK_MGR3: c_uint;
    static ES8375_CLK_MGR4: c_uint;
    static ES8375_CLK_MGR5: c_uint;
    static ES8375_CLK_MGR6: c_uint;
    static ES8375_CLK_MGR7: c_uint;
    static ES8375_CLK_MGR8: c_uint;
    static ES8375_CLK_MGR9: c_uint;
    static ES8375_CLK_MGR10: c_uint;
    static ES8375_CLK_MGR11: c_uint;
    static ES8375_RESET1: c_uint;
    static ES8375_CSM1: c_uint;
    static ES8375_CSM2: c_uint;
    static ES8375_DIV_SPKCLK: c_uint;
    static ES8375_VMID_CHARGE2: c_uint;
    static ES8375_VMID_CHARGE3: c_uint;
    static ES8375_DAC_CAL: c_uint;
    static ES8375_ANALOG_SPK1: c_uint;
    static ES8375_ANALOG_SPK2: c_uint;
    static ES8375_VMID_SEL: c_uint;
    static ES8375_ANALOG1: c_uint;
    static ES8375_ANALOG2: c_uint;
    static ES8375_ANALOG3: c_uint;
    static ES8375_SYS_CTRL2: c_uint;
    static ES8375_DAC_OTP: c_uint;
    static ES8375_CHIP_VERSION: c_uint;
    static ES8375_CHIP_ID0: c_uint;
    static ES8375_CHIP_ID1: c_uint;
    static ES8375_SPK_OFFSET: c_uint;
    static ES8375_FLAGS2: c_uint;
    static ES8375_REG_MAX: c_uint;

    static ES8375_ADC_OSR_GAIN_MAX: c_uint;
    static ES8375_DMIC_GAIN_MAX: c_uint;
    static ES8375_ADC_VOLUME_MAX: c_uint;
    static ES8375_AUTOMUTE_NG_MAX: c_uint;
    static ES8375_ADC_AUTOMUTE_ATTN_MAX: c_uint;
    static ES8375_DAC_VOLUME_MAX: c_uint;
    static ES8375_DAC_VPPSCALE_MAX: c_uint;
    static ES8375_DAC_AUTOMUTE_ATTN_MAX: c_uint;
    static ES8375_BCLK_PIN: u8;
    static ES8375_MCLK_SOURCE: u8;
    static ES8375_1V8: u8;
    static ES8375_3V3: u8;
    static ES8375_SUPPLY_VD: usize;

    static ADC_RAMPRATE_SHIFT_0: c_uint;
    static DAC_RAMPRATE_SHIFT_0: c_uint;
    static ADC_AUTOMUTE_WS_SHIFT_3: c_uint;
    static DAC_AUTOMUTE_WS_SHIFT_5: c_uint;
    static DMIC_POL_SHIFT_4: c_uint;
    static ADC_HPF_SHIFT_5: c_uint;
    static ADC_SRC_SHIFT_7: c_uint;
    static ADC_OSR_GAIN_SHIFT_0: c_uint;
    static ADC_INV_SHIFT_6: c_uint;
    static ADC_RAMCLR_SHIFT_5: c_uint;
    static DMIC_GAIN_SHIFT_2: c_uint;
    static ADC_VOLUME_SHIFT_0: c_uint;
    static ADC_AUTOMUTE_SHIFT_7: c_uint;
    static ADC_AUTOMUTE_NG_SHIFT_0: c_uint;
    static ADC_AUTOMUTE_ATTN_SHIFT_0: c_uint;
    static DAC_DSMMUTE_SHIFT_7: c_uint;
    static DAC_DEMMUTE_SHIFT_6: c_uint;
    static DAC_INV_SHIFT_5: c_uint;
    static DAC_RAMCLR_SHIFT_4: c_uint;
    static DAC_VOLUME_SHIFT_0: c_uint;
    static DAC_VPPSCALE_SHIFT_0: c_uint;
    static DAC_AUTOMUTE_EN_SHIFT_7: c_uint;
    static DAC_AUTOMUTE_NG_SHIFT_0: c_uint;
    static DAC_AUTOMUTE_ATTN_SHIFT_0: c_uint;
    static ES8375_ADC_P2S_MUTE_SHIFT_5: c_uint;

    static SND_SOC_NOPM: c_uint;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_FORMAT_S16_LE: c_int;
    static SNDRV_PCM_FORMAT_S20_3LE: c_int;
    static SNDRV_PCM_FORMAT_S24_LE: c_int;
    static SNDRV_PCM_FORMAT_S32_LE: c_int;
    static SNDRV_PCM_RATE_8000_96000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S20_3LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_3LE: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_uint;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBC_CFP: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SND_SOC_BIAS_ON: snd_soc_bias_level;
    static SND_SOC_BIAS_PREPARE: snd_soc_bias_level;
    static SND_SOC_BIAS_STANDBY: snd_soc_bias_level;
    static SND_SOC_BIAS_OFF: snd_soc_bias_level;
}

#[repr(C)] struct regmap { _private: [u8; 0] }
#[repr(C)] struct clk { _private: [u8; 0] }
#[repr(C)] struct device { _private: [u8; 0] }
#[repr(C)] struct snd_pcm_substream { _private: [u8; 0] }
#[repr(C)] struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] struct snd_soc_component { dev: *mut device }
#[repr(C)] struct snd_soc_dai { component: *mut snd_soc_component }
#[repr(C)] struct i2c_client { dev: device, addr: c_uint }
#[repr(C)] struct regulator { _private: [u8; 0] }
#[repr(C)] struct regulator_bulk_data { supply: *const c_char, consumer: *mut regulator }
#[repr(C)] struct soc_enum { _private: [u8; 0] }
#[repr(C)] struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] struct snd_soc_dapm_widget { _private: [u8; 0] }
#[repr(C)] struct snd_soc_dapm_route { sink: *const c_char, control: *const c_char, source: *const c_char }
#[repr(C)] struct snd_soc_dai_ops { hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>, mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>, set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>, set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int> }
#[repr(C)] struct snd_soc_pcm_stream { stream_name: *const c_char, channels_min: c_uint, channels_max: c_uint, rates: c_uint, formats: c_uint }
#[repr(C)] struct snd_soc_dai_driver { name: *const c_char, playback: snd_soc_pcm_stream, capture: snd_soc_pcm_stream, ops: *const snd_soc_dai_ops, symmetric_rate: c_uint }
#[repr(C)] struct snd_soc_component_driver { probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>, suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>, resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>, set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>, controls: *const snd_kcontrol_new, num_controls: usize, dapm_widgets: *const snd_soc_dapm_widget, num_dapm_widgets: usize, dapm_routes: *const snd_soc_dapm_route, num_dapm_routes: usize, idle_bias_on: c_uint, suspend_bias_off: c_uint }
#[repr(C)] struct regmap_config { reg_bits: c_uint, val_bits: c_uint, max_register: c_uint, cache_type: c_uint, use_single_read: bool, use_single_write: bool, writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool> }
#[repr(C)] struct i2c_device_id { name: [c_char; 20], driver_data: usize }
#[repr(C)] struct acpi_device_id { id: [c_char; 16], driver_data: usize }
#[repr(C)] struct of_device_id { compatible: *const c_char }
#[repr(C)] struct device_driver { name: *const c_char, of_match_table: *const of_device_id, acpi_match_table: *const acpi_device_id }
#[repr(C)] struct i2c_driver { driver: device_driver, shutdown: Option<unsafe extern "C" fn(*mut i2c_client)>, probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>, id_table: *const i2c_device_id }
type snd_soc_bias_level = c_uint;

#[repr(C)]
struct es8375_priv {
    regmap: *mut regmap,
    mclk: *mut clk,
    core_supply: [regulator_bulk_data; 2],
    mclk_freq: c_uint,
    mastermode: c_int,
    mclk_src: u8,
    vddd: u8,
    bias_level: snd_soc_bias_level,
}

const fn cstr(bytes: &'static [u8]) -> *const c_char { bytes.as_ptr() as *const c_char }

static es8375_core_supplies: [*const c_char; 2] = [
    cstr(b"vddd\0"),
    cstr(b"vdda\0"),
];

/* DECLARE_TLV_DB_SCALE entries from C:
 * es8375_adc_osr_gain_tlv(-3100,100,0), es8375_adc_volume_tlv(-9550,50,0),
 * es8375_adc_automute_attn_tlv(0,100,0), es8375_adc_dmic_volume_tlv(0,600,0),
 * es8375_dac_volume_tlv(-9550,50,0), es8375_dac_vppscale_tlv(-388,12,0),
 * es8375_dac_automute_attn_tlv(0,400,0), es8375_automute_ng_tlv(-9600,600,0).
 */
static es8375_adc_osr_gain_tlv: [c_uint; 4] = [0, (-3100i32) as c_uint, 100, 0];
static es8375_adc_volume_tlv: [c_uint; 4] = [0, (-9550i32) as c_uint, 50, 0];
static es8375_adc_automute_attn_tlv: [c_uint; 4] = [0, 0, 100, 0];
static es8375_adc_dmic_volume_tlv: [c_uint; 4] = [0, 0, 600, 0];
static es8375_dac_volume_tlv: [c_uint; 4] = [0, (-9550i32) as c_uint, 50, 0];
static es8375_dac_vppscale_tlv: [c_uint; 4] = [0, (-388i32) as c_uint, 12, 0];
static es8375_dac_automute_attn_tlv: [c_uint; 4] = [0, 0, 400, 0];
static es8375_automute_ng_tlv: [c_uint; 4] = [0, (-9600i32) as c_uint, 600, 0];

static es8375_ramprate_txt: [*const c_char; 9] = [
    cstr(b"0.125dB/LRCK\0"), cstr(b"0.125dB/2LRCK\0"), cstr(b"0.125dB/4LRCK\0"),
    cstr(b"0.125dB/8LRCK\0"), cstr(b"0.125dB/16LRCK\0"), cstr(b"0.125dB/32LRCK\0"),
    cstr(b"0.125dB/64LRCK\0"), cstr(b"0.125dB/128LRCK\0"), cstr(b"disable softramp\0"),
];
static es8375_automute_ws_txt: [*const c_char; 8] = [
    cstr(b"256 samples\0"), cstr(b"512 samples\0"), cstr(b"1024 samples\0"), cstr(b"2048 samples\0"),
    cstr(b"4096 samples\0"), cstr(b"8192 samples\0"), cstr(b"16384 samples\0"), cstr(b"32768 samples\0"),
];
static es8375_dmic_pol_txt: [*const c_char; 2] = [cstr(b"Low\0"), cstr(b"High\0")];
static es8375_adc_hpf_txt: [*const c_char; 2] = [cstr(b"Freeze Offset\0"), cstr(b"Dynamic HPF\0")];
static es8375_dmic_mux_txt: [*const c_char; 2] = [cstr(b"AMIC\0"), cstr(b"DMIC\0")];

/* SOC_ENUM/SOC_* and SND_SOC_DAPM_* macro-created data are represented as
 * zeroed C-layout placeholders; the original macro arguments are preserved in
 * the translated names/comments above and the externally visible arrays below.
 */
static es8375_dmic_mux_enum: soc_enum = soc_enum { _private: [] };
static es8375_dmic_mux_controls: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static es8375_snd_controls: [snd_kcontrol_new; 22] = [snd_kcontrol_new { _private: [] }; 22];
static es8375_dapm_widgets: [snd_soc_dapm_widget; 9] = [snd_soc_dapm_widget { _private: [] }; 9];
static es8375_dapm_routes: [snd_soc_dapm_route; 7] = [
    snd_soc_dapm_route { sink: cstr(b"ADC MUX\0"), control: cstr(b"AMIC\0"), source: cstr(b"MIC1\0") },
    snd_soc_dapm_route { sink: cstr(b"ADC MUX\0"), control: cstr(b"DMIC\0"), source: cstr(b"DMIC\0") },
    snd_soc_dapm_route { sink: cstr(b"PGA\0"), control: core::ptr::null(), source: cstr(b"ADC MUX\0") },
    snd_soc_dapm_route { sink: cstr(b"Mono ADC\0"), control: core::ptr::null(), source: cstr(b"PGA\0") },
    snd_soc_dapm_route { sink: cstr(b"AIF1TX\0"), control: core::ptr::null(), source: cstr(b"Mono ADC\0") },
    snd_soc_dapm_route { sink: cstr(b"Mono DAC\0"), control: core::ptr::null(), source: cstr(b"AIF1RX\0") },
    snd_soc_dapm_route { sink: cstr(b"OUT\0"), control: core::ptr::null(), source: cstr(b"Mono DAC\0") },
];

#[repr(C)]
struct _coeff_div {
    mclk_lrck_ratio: u16,
    mclk: u32,
    rate: u32,
    Reg0x04: u8,
    Reg0x05: u8,
    Reg0x06: u8,
    Reg0x07: u8,
    Reg0x08: u8,
    Reg0x09: u8,
    Reg0x0A: u8,
    Reg0x0B: u8,
    Reg0x19: u8,
    dvdd_vol: u8,
    dmic_sel: u8,
}

static coeff_div: [_coeff_div; 61] = [
    _coeff_div { mclk_lrck_ratio: 32, mclk: 256000, rate: 8000, Reg0x04: 0x05, Reg0x05: 0x34, Reg0x06: 0xDD, Reg0x07: 0x55, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x95, Reg0x0B: 0x00, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 32, mclk: 512000, rate: 16000, Reg0x04: 0x05, Reg0x05: 0x34, Reg0x06: 0xDD, Reg0x07: 0x55, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x94, Reg0x0B: 0x00, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 32, mclk: 1536000, rate: 48000, Reg0x04: 0x05, Reg0x05: 0x33, Reg0x06: 0xD5, Reg0x07: 0x55, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x93, Reg0x0B: 0x00, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 36, mclk: 288000, rate: 8000, Reg0x04: 0x05, Reg0x05: 0x34, Reg0x06: 0xDD, Reg0x07: 0x55, Reg0x08: 0x23, Reg0x09: 0x08, Reg0x0A: 0x95, Reg0x0B: 0x00, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 36, mclk: 576000, rate: 16000, Reg0x04: 0x05, Reg0x05: 0x34, Reg0x06: 0xDD, Reg0x07: 0x55, Reg0x08: 0x23, Reg0x09: 0x08, Reg0x0A: 0x94, Reg0x0B: 0x00, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 36, mclk: 1728000, rate: 48000, Reg0x04: 0x05, Reg0x05: 0x33, Reg0x06: 0xD5, Reg0x07: 0x55, Reg0x08: 0x23, Reg0x09: 0x08, Reg0x0A: 0x93, Reg0x0B: 0x00, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 48, mclk: 384000, rate: 8000, Reg0x04: 0x05, Reg0x05: 0x14, Reg0x06: 0x5D, Reg0x07: 0x55, Reg0x08: 0x17, Reg0x09: 0x20, Reg0x0A: 0x94, Reg0x0B: 0x00, Reg0x19: 0x28, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 48, mclk: 768000, rate: 16000, Reg0x04: 0x05, Reg0x05: 0x14, Reg0x06: 0x5D, Reg0x07: 0x55, Reg0x08: 0x17, Reg0x09: 0x20, Reg0x0A: 0x94, Reg0x0B: 0x00, Reg0x19: 0x28, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 48, mclk: 2304000, rate: 48000, Reg0x04: 0x05, Reg0x05: 0x11, Reg0x06: 0x53, Reg0x07: 0x55, Reg0x08: 0x17, Reg0x09: 0x20, Reg0x0A: 0x92, Reg0x0B: 0x00, Reg0x19: 0x28, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 50, mclk: 400000, rate: 8000, Reg0x04: 0x05, Reg0x05: 0x14, Reg0x06: 0x5D, Reg0x07: 0x55, Reg0x08: 0x18, Reg0x09: 0x24, Reg0x0A: 0x94, Reg0x0B: 0x00, Reg0x19: 0x27, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 50, mclk: 800000, rate: 16000, Reg0x04: 0x05, Reg0x05: 0x14, Reg0x06: 0x5D, Reg0x07: 0x55, Reg0x08: 0x18, Reg0x09: 0x24, Reg0x0A: 0x94, Reg0x0B: 0x00, Reg0x19: 0x27, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 50, mclk: 2400000, rate: 48000, Reg0x04: 0x05, Reg0x05: 0x11, Reg0x06: 0x53, Reg0x07: 0x55, Reg0x08: 0x18, Reg0x09: 0x24, Reg0x0A: 0x92, Reg0x0B: 0x00, Reg0x19: 0x27, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 64, mclk: 512000, rate: 8000, Reg0x04: 0x05, Reg0x05: 0x14, Reg0x06: 0x5D, Reg0x07: 0x33, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x94, Reg0x0B: 0x00, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 64, mclk: 1024000, rate: 16000, Reg0x04: 0x05, Reg0x05: 0x13, Reg0x06: 0x55, Reg0x07: 0x33, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x93, Reg0x0B: 0x00, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 64, mclk: 3072000, rate: 48000, Reg0x04: 0x05, Reg0x05: 0x11, Reg0x06: 0x53, Reg0x07: 0x33, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x92, Reg0x0B: 0x00, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 72, mclk: 576000, rate: 8000, Reg0x04: 0x05, Reg0x05: 0x14, Reg0x06: 0x5D, Reg0x07: 0x33, Reg0x08: 0x23, Reg0x09: 0x08, Reg0x0A: 0x94, Reg0x0B: 0x00, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 72, mclk: 1152000, rate: 16000, Reg0x04: 0x05, Reg0x05: 0x13, Reg0x06: 0x55, Reg0x07: 0x33, Reg0x08: 0x23, Reg0x09: 0x08, Reg0x0A: 0x93, Reg0x0B: 0x00, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 72, mclk: 3456000, rate: 48000, Reg0x04: 0x05, Reg0x05: 0x11, Reg0x06: 0x53, Reg0x07: 0x33, Reg0x08: 0x23, Reg0x09: 0x08, Reg0x0A: 0x92, Reg0x0B: 0x00, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 96, mclk: 768000, rate: 8000, Reg0x04: 0x15, Reg0x05: 0x34, Reg0x06: 0xDD, Reg0x07: 0x55, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x94, Reg0x0B: 0x00, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 96, mclk: 1536000, rate: 16000, Reg0x04: 0x15, Reg0x05: 0x34, Reg0x06: 0xDD, Reg0x07: 0x55, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x93, Reg0x0B: 0x00, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 96, mclk: 4608000, rate: 48000, Reg0x04: 0x15, Reg0x05: 0x33, Reg0x06: 0xD5, Reg0x07: 0x55, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x92, Reg0x0B: 0x00, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 100, mclk: 800000, rate: 8000, Reg0x04: 0x05, Reg0x05: 0x03, Reg0x06: 0x35, Reg0x07: 0x33, Reg0x08: 0x18, Reg0x09: 0x24, Reg0x0A: 0x94, Reg0x0B: 0x00, Reg0x19: 0x27, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 100, mclk: 1600000, rate: 16000, Reg0x04: 0x05, Reg0x05: 0x03, Reg0x06: 0x35, Reg0x07: 0x33, Reg0x08: 0x18, Reg0x09: 0x24, Reg0x0A: 0x93, Reg0x0B: 0x00, Reg0x19: 0x27, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 100, mclk: 4800000, rate: 48000, Reg0x04: 0x03, Reg0x05: 0x00, Reg0x06: 0x31, Reg0x07: 0x33, Reg0x08: 0x18, Reg0x09: 0x24, Reg0x0A: 0x92, Reg0x0B: 0x00, Reg0x19: 0x27, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 128, mclk: 1024000, rate: 8000, Reg0x04: 0x05, Reg0x05: 0x03, Reg0x06: 0x35, Reg0x07: 0x11, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x93, Reg0x0B: 0x01, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 128, mclk: 2048000, rate: 16000, Reg0x04: 0x03, Reg0x05: 0x01, Reg0x06: 0x33, Reg0x07: 0x11, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x92, Reg0x0B: 0x01, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 128, mclk: 6144000, rate: 48000, Reg0x04: 0x03, Reg0x05: 0x00, Reg0x06: 0x31, Reg0x07: 0x11, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x92, Reg0x0B: 0x01, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 144, mclk: 1152000, rate: 8000, Reg0x04: 0x05, Reg0x05: 0x03, Reg0x06: 0x35, Reg0x07: 0x11, Reg0x08: 0x23, Reg0x09: 0x08, Reg0x0A: 0x93, Reg0x0B: 0x01, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 144, mclk: 2304000, rate: 16000, Reg0x04: 0x03, Reg0x05: 0x01, Reg0x06: 0x33, Reg0x07: 0x11, Reg0x08: 0x23, Reg0x09: 0x08, Reg0x0A: 0x92, Reg0x0B: 0x01, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 144, mclk: 6912000, rate: 48000, Reg0x04: 0x03, Reg0x05: 0x00, Reg0x06: 0x31, Reg0x07: 0x11, Reg0x08: 0x23, Reg0x09: 0x08, Reg0x0A: 0x92, Reg0x0B: 0x01, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 192, mclk: 1536000, rate: 8000, Reg0x04: 0x15, Reg0x05: 0x14, Reg0x06: 0x5D, Reg0x07: 0x33, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x93, Reg0x0B: 0x02, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 192, mclk: 3072000, rate: 16000, Reg0x04: 0x15, Reg0x05: 0x13, Reg0x06: 0x55, Reg0x07: 0x33, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x92, Reg0x0B: 0x02, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 192, mclk: 9216000, rate: 48000, Reg0x04: 0x15, Reg0x05: 0x11, Reg0x06: 0x53, Reg0x07: 0x33, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x92, Reg0x0B: 0x02, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 250, mclk: 12000000, rate: 48000, Reg0x04: 0x25, Reg0x05: 0x11, Reg0x06: 0x53, Reg0x07: 0x55, Reg0x08: 0x18, Reg0x09: 0x24, Reg0x0A: 0x92, Reg0x0B: 0x04, Reg0x19: 0x27, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 256, mclk: 2048000, rate: 8000, Reg0x04: 0x0D, Reg0x05: 0x03, Reg0x06: 0x35, Reg0x07: 0x11, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x92, Reg0x0B: 0x03, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 256, mclk: 4096000, rate: 16000, Reg0x04: 0x0B, Reg0x05: 0x01, Reg0x06: 0x33, Reg0x07: 0x11, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x92, Reg0x0B: 0x03, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 256, mclk: 12288000, rate: 48000, Reg0x04: 0x0B, Reg0x05: 0x00, Reg0x06: 0x31, Reg0x07: 0x11, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x92, Reg0x0B: 0x03, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 384, mclk: 3072000, rate: 8000, Reg0x04: 0x15, Reg0x05: 0x03, Reg0x06: 0x35, Reg0x07: 0x11, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x92, Reg0x0B: 0x05, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 384, mclk: 6144000, rate: 16000, Reg0x04: 0x13, Reg0x05: 0x01, Reg0x06: 0x33, Reg0x07: 0x11, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x92, Reg0x0B: 0x05, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 384, mclk: 18432000, rate: 48000, Reg0x04: 0x13, Reg0x05: 0x00, Reg0x06: 0x31, Reg0x07: 0x11, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x92, Reg0x0B: 0x05, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 400, mclk: 19200000, rate: 48000, Reg0x04: 0x1B, Reg0x05: 0x00, Reg0x06: 0x31, Reg0x07: 0x33, Reg0x08: 0x18, Reg0x09: 0x24, Reg0x0A: 0x92, Reg0x0B: 0x04, Reg0x19: 0x27, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 500, mclk: 24000000, rate: 48000, Reg0x04: 0x23, Reg0x05: 0x00, Reg0x06: 0x31, Reg0x07: 0x33, Reg0x08: 0x18, Reg0x09: 0x24, Reg0x0A: 0x92, Reg0x0B: 0x04, Reg0x19: 0x27, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 512, mclk: 4096000, rate: 8000, Reg0x04: 0x1D, Reg0x05: 0x03, Reg0x06: 0x35, Reg0x07: 0x11, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x92, Reg0x0B: 0x07, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 512, mclk: 8192000, rate: 16000, Reg0x04: 0x1B, Reg0x05: 0x01, Reg0x06: 0x33, Reg0x07: 0x11, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x92, Reg0x0B: 0x07, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 512, mclk: 24576000, rate: 48000, Reg0x04: 0x1B, Reg0x05: 0x00, Reg0x06: 0x31, Reg0x07: 0x11, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x92, Reg0x0B: 0x07, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 768, mclk: 6144000, rate: 8000, Reg0x04: 0x2D, Reg0x05: 0x03, Reg0x06: 0x35, Reg0x07: 0x11, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x92, Reg0x0B: 0x0B, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 768, mclk: 12288000, rate: 16000, Reg0x04: 0x2B, Reg0x05: 0x01, Reg0x06: 0x33, Reg0x07: 0x11, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x92, Reg0x0B: 0x0B, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 1024, mclk: 8192000, rate: 8000, Reg0x04: 0x3D, Reg0x05: 0x03, Reg0x06: 0x35, Reg0x07: 0x11, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x92, Reg0x0B: 0x0F, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 1024, mclk: 16384000, rate: 16000, Reg0x04: 0x3B, Reg0x05: 0x01, Reg0x06: 0x33, Reg0x07: 0x11, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x92, Reg0x0B: 0x0F, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 1152, mclk: 9216000, rate: 8000, Reg0x04: 0x45, Reg0x05: 0x03, Reg0x06: 0x35, Reg0x07: 0x11, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x92, Reg0x0B: 0x0F, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 1152, mclk: 18432000, rate: 16000, Reg0x04: 0x43, Reg0x05: 0x01, Reg0x06: 0x33, Reg0x07: 0x11, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x92, Reg0x0B: 0x0F, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 1200, mclk: 9600000, rate: 8000, Reg0x04: 0x5D, Reg0x05: 0x03, Reg0x06: 0x35, Reg0x07: 0x33, Reg0x08: 0x18, Reg0x09: 0x24, Reg0x0A: 0x92, Reg0x0B: 0x11, Reg0x19: 0x27, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 1200, mclk: 19200000, rate: 16000, Reg0x04: 0x5D, Reg0x05: 0x03, Reg0x06: 0x35, Reg0x07: 0x33, Reg0x08: 0x18, Reg0x09: 0x24, Reg0x0A: 0x92, Reg0x0B: 0x11, Reg0x19: 0x27, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 1536, mclk: 12288000, rate: 8000, Reg0x04: 0x5D, Reg0x05: 0x03, Reg0x06: 0x35, Reg0x07: 0x11, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x92, Reg0x0B: 0x17, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 1536, mclk: 24576000, rate: 16000, Reg0x04: 0x5B, Reg0x05: 0x01, Reg0x06: 0x33, Reg0x07: 0x11, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x92, Reg0x0B: 0x17, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 2048, mclk: 16384000, rate: 8000, Reg0x04: 0x7D, Reg0x05: 0x03, Reg0x06: 0x35, Reg0x07: 0x11, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x92, Reg0x0B: 0x1F, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 2304, mclk: 18432000, rate: 8000, Reg0x04: 0x8D, Reg0x05: 0x03, Reg0x06: 0x35, Reg0x07: 0x11, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x92, Reg0x0B: 0x23, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 2400, mclk: 19200000, rate: 8000, Reg0x04: 0xBD, Reg0x05: 0x03, Reg0x06: 0x35, Reg0x07: 0x33, Reg0x08: 0x18, Reg0x09: 0x24, Reg0x0A: 0x92, Reg0x0B: 0x25, Reg0x19: 0x27, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 3072, mclk: 24576000, rate: 8000, Reg0x04: 0xBD, Reg0x05: 0x03, Reg0x06: 0x35, Reg0x07: 0x11, Reg0x08: 0x1F, Reg0x09: 0x00, Reg0x0A: 0x92, Reg0x0B: 0x2F, Reg0x19: 0x1F, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 32, mclk: 3072000, rate: 96000, Reg0x04: 0x05, Reg0x05: 0x11, Reg0x06: 0x53, Reg0x07: 0x55, Reg0x08: 0x0F, Reg0x09: 0x00, Reg0x0A: 0x92, Reg0x0B: 0x00, Reg0x19: 0x37, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 64, mclk: 6144000, rate: 96000, Reg0x04: 0x03, Reg0x05: 0x00, Reg0x06: 0x31, Reg0x07: 0x33, Reg0x08: 0x0F, Reg0x09: 0x00, Reg0x0A: 0x92, Reg0x0B: 0x00, Reg0x19: 0x37, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 96, mclk: 9216000, rate: 96000, Reg0x04: 0x15, Reg0x05: 0x11, Reg0x06: 0x53, Reg0x07: 0x55, Reg0x08: 0x0F, Reg0x09: 0x00, Reg0x0A: 0x92, Reg0x0B: 0x00, Reg0x19: 0x37, dvdd_vol: 2, dmic_sel: 2 },
    _coeff_div { mclk_lrck_ratio: 128, mclk: 12288000, rate: 96000, Reg0x04: 0x0B, Reg0x05: 0x00, Reg0x06: 0x31, Reg0x07: 0x33, Reg0x08: 0x0F, Reg0x09: 0x00, Reg0x0A: 0x92, Reg0x0B: 0x01, Reg0x19: 0x37, dvdd_vol: 2, dmic_sel: 2 },
];

extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regulator_get_voltage(regulator: *mut regulator) -> c_int;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: isize, fmt: *const c_char, ...) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn msleep(msecs: c_uint);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_cache_bypass(map: *mut regmap, enable: bool);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn device_property_read_u8(dev: *mut device, propname: *const c_char, val: *mut u8) -> c_int;
    fn devm_regulator_bulk_get(dev: *mut device, num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn regulator_bulk_enable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut c_void;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *mut snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> isize;
    fn of_match_ptr(ptr: *const of_device_id) -> *const of_device_id;
    fn ACPI_PTR(ptr: *const acpi_device_id) -> *const acpi_device_id;
}

unsafe fn get_coeff(vddd: u8, dmic: u8, mclk: c_int, rate: c_int) -> c_int {
    let mut i: usize = 0;
    while i < coeff_div.len() {
        if coeff_div[i].rate == rate as u32 && coeff_div[i].mclk == mclk as u32 {
            let mut vddd_det: u8 = (!(coeff_div[i].dvdd_vol ^ vddd)) & 0x01;
            let mut dmic_det: u8 = (!(coeff_div[i].dmic_sel ^ dmic)) & 0x01;
            vddd_det |= (!(coeff_div[i].dvdd_vol % 2)) & 0x01;
            dmic_det |= (!(coeff_div[i].dmic_sel % 2)) & 0x01;

            if vddd_det != 0 && dmic_det != 0 {
                return i as c_int;
            }
        }
        i += 1;
    }
    -EINVAL
}

unsafe extern "C" fn es8375_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let es8375 = snd_soc_component_get_drvdata(component) as *mut es8375_priv;
    let par_width = params_width(params);
    let mut dmic_enable: u8;
    let mut iface: u8 = 0;
    let mut regv: c_uint = 0;
    let coeff: c_int;
    let mut ret: c_int;

    if (*es8375).mclk_src == ES8375_BCLK_PIN {
        regmap_update_bits((*es8375).regmap, ES8375_MCLK_SEL, 0x80, 0x80);
        (*es8375).mclk_freq = (2u32).wrapping_mul(par_width as c_uint).wrapping_mul(params_rate(params));
    }

    regmap_read((*es8375).regmap, ES8375_ADC1, &mut regv);
    dmic_enable = ((regv >> 7) & 0x01) as u8;

    ret = regulator_get_voltage((*es8375).core_supply[ES8375_SUPPLY_VD].consumer);
    match ret {
        1800000..=2000000 => (*es8375).vddd = ES8375_1V8,
        2500000..=3300000 => (*es8375).vddd = ES8375_3V3,
        _ => (*es8375).vddd = ES8375_3V3,
    }

    coeff = get_coeff((*es8375).vddd, dmic_enable, (*es8375).mclk_freq as c_int, params_rate(params) as c_int);
    if coeff < 0 {
        dev_warn((*component).dev, cstr(b"Clock coefficients do not match\0"));
        return coeff;
    }
    let c = coeff as usize;
    regmap_write((*es8375).regmap, ES8375_CLK_MGR4, coeff_div[c].Reg0x04 as c_uint);
    regmap_write((*es8375).regmap, ES8375_CLK_MGR5, coeff_div[c].Reg0x05 as c_uint);
    regmap_write((*es8375).regmap, ES8375_CLK_MGR6, coeff_div[c].Reg0x06 as c_uint);
    regmap_write((*es8375).regmap, ES8375_CLK_MGR7, coeff_div[c].Reg0x07 as c_uint);
    regmap_write((*es8375).regmap, ES8375_CLK_MGR8, coeff_div[c].Reg0x08 as c_uint);
    regmap_write((*es8375).regmap, ES8375_CLK_MGR9, coeff_div[c].Reg0x09 as c_uint);
    regmap_write((*es8375).regmap, ES8375_CLK_MGR10, coeff_div[c].Reg0x0A as c_uint);
    regmap_write((*es8375).regmap, ES8375_CLK_MGR11, coeff_div[c].Reg0x0B as c_uint);
    regmap_write((*es8375).regmap, ES8375_ADC_OSR_GAIN, coeff_div[c].Reg0x19 as c_uint);

    match params_format(params) {
        x if x == SNDRV_PCM_FORMAT_S16_LE => iface |= 0x0c,
        x if x == SNDRV_PCM_FORMAT_S20_3LE => iface |= 0x04,
        x if x == SNDRV_PCM_FORMAT_S24_LE => {}
        x if x == SNDRV_PCM_FORMAT_S32_LE => iface |= 0x10,
        _ => {}
    }

    regmap_update_bits((*es8375).regmap, ES8375_SDP, 0x1c, iface as c_uint);
    0
}

unsafe extern "C" fn es8375_set_sysclk(dai: *mut snd_soc_dai, _clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component = (*dai).component;
    let es8375 = snd_soc_component_get_drvdata(component) as *mut es8375_priv;
    (*es8375).mclk_freq = freq;
    0
}

unsafe extern "C" fn es8375_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let es8375 = snd_soc_component_get_drvdata(component) as *mut es8375_priv;
    let mut iface: c_uint = 0;
    let mut codeciface: c_uint = 0;

    regmap_read((*es8375).regmap, ES8375_SDP, &mut codeciface);

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        x if x == SND_SOC_DAIFMT_CBC_CFP => {
            (*es8375).mastermode = 1;
            regmap_update_bits((*es8375).regmap, ES8375_RESET1, 0x80, 0x80);
        }
        x if x == SND_SOC_DAIFMT_CBC_CFC => {
            (*es8375).mastermode = 0;
            regmap_update_bits((*es8375).regmap, ES8375_RESET1, 0x80, 0x00);
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => codeciface &= 0xFC,
        x if x == SND_SOC_DAIFMT_LEFT_J => { codeciface &= 0xFC; codeciface |= 0x01; }
        x if x == SND_SOC_DAIFMT_DSP_A => { codeciface &= 0xDC; codeciface |= 0x03; }
        x if x == SND_SOC_DAIFMT_DSP_B => { codeciface &= 0xDC; codeciface |= 0x23; }
        _ => return -EINVAL,
    }

    regmap_read((*es8375).regmap, ES8375_CLK_MGR3, &mut iface);

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF => { iface &= 0xFE; codeciface &= 0xDF; }
        x if x == SND_SOC_DAIFMT_IB_IF => { iface |= 0x01; codeciface |= 0x20; }
        x if x == SND_SOC_DAIFMT_IB_NF => { iface |= 0x01; codeciface &= 0xDF; }
        x if x == SND_SOC_DAIFMT_NB_IF => { iface &= 0xFE; codeciface |= 0x20; }
        _ => return -EINVAL,
    }

    regmap_write((*es8375).regmap, ES8375_CLK_MGR3, iface);
    regmap_write((*es8375).regmap, ES8375_SDP, codeciface);
    0
}

unsafe extern "C" fn es8375_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let es8375 = snd_soc_component_get_drvdata(component) as *mut es8375_priv;
    let ret: c_int;
    if level == SND_SOC_BIAS_ON {
        ret = clk_prepare_enable((*es8375).mclk);
        if ret != 0 {
            dev_err((*component).dev, cstr(b"unable to prepare mclk\n\0"));
            return ret;
        }
        regmap_write((*es8375).regmap, ES8375_CSM1, 0xA6);
    } else if level == SND_SOC_BIAS_PREPARE {
    } else if level == SND_SOC_BIAS_STANDBY {
        regmap_write((*es8375).regmap, ES8375_CSM1, 0x96);
        clk_disable_unprepare((*es8375).mclk);
    } else if level == SND_SOC_BIAS_OFF {
    }
    0
}

unsafe extern "C" fn es8375_mute(dai: *mut snd_soc_dai, mute: c_int, stream: c_int) -> c_int {
    let component = (*dai).component;
    let es8375 = snd_soc_component_get_drvdata(component) as *mut es8375_priv;
    if mute != 0 {
        if stream == SNDRV_PCM_STREAM_PLAYBACK {
            regmap_update_bits((*es8375).regmap, ES8375_SDP, 0x40, 0x40);
        } else {
            regmap_update_bits((*es8375).regmap, ES8375_SDP2, 0x20, 0x20);
        }
    } else if stream == SNDRV_PCM_STREAM_PLAYBACK {
        regmap_update_bits((*es8375).regmap, ES8375_SDP, 0x40, 0x00);
    } else {
        regmap_update_bits((*es8375).regmap, ES8375_SDP2, 0x20, 0x00);
    }
    0
}

unsafe fn es8375_RATES() -> c_uint { SNDRV_PCM_RATE_8000_96000 }
unsafe fn es8375_FORMATS() -> c_uint {
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE |
    SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S24_3LE | SNDRV_PCM_FMTBIT_S32_LE
}

static es8375_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(es8375_hw_params),
    mute_stream: Some(es8375_mute),
    set_sysclk: Some(es8375_set_sysclk),
    set_fmt: Some(es8375_set_dai_fmt),
};

static mut es8375_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: cstr(b"ES8375 HiFi\0"),
    playback: snd_soc_pcm_stream { stream_name: cstr(b"AIF1 Playback\0"), channels_min: 1, channels_max: 2, rates: 0, formats: 0 },
    capture: snd_soc_pcm_stream { stream_name: cstr(b"AIF1 Capture\0"), channels_min: 1, channels_max: 2, rates: 0, formats: 0 },
    ops: &es8375_ops,
    symmetric_rate: 1,
};

unsafe fn es8375_init(component: *mut snd_soc_component) {
    let es8375 = snd_soc_component_get_drvdata(component) as *mut es8375_priv;
    regmap_write((*es8375).regmap, ES8375_CLK_MGR10, 0x95);
    regmap_write((*es8375).regmap, ES8375_CLK_MGR3, 0x48);
    regmap_write((*es8375).regmap, ES8375_DIV_SPKCLK, 0x18);
    regmap_write((*es8375).regmap, ES8375_CLK_MGR4, 0x02);
    regmap_write((*es8375).regmap, ES8375_CLK_MGR5, 0x05);
    regmap_write((*es8375).regmap, ES8375_CSM1, 0x82);
    regmap_write((*es8375).regmap, ES8375_VMID_CHARGE2, 0x20);
    regmap_write((*es8375).regmap, ES8375_VMID_CHARGE3, 0x20);
    regmap_write((*es8375).regmap, ES8375_DAC_CAL, 0x28);
    regmap_write((*es8375).regmap, ES8375_ANALOG_SPK1, 0xFC);
    regmap_write((*es8375).regmap, ES8375_ANALOG_SPK2, 0xE0);
    regmap_write((*es8375).regmap, ES8375_VMID_SEL, 0xFE);
    regmap_write((*es8375).regmap, ES8375_ANALOG1, 0xB8);
    regmap_write((*es8375).regmap, ES8375_SYS_CTRL2, 0x03);
    regmap_write((*es8375).regmap, ES8375_CLK_MGR2, 0x16);
    regmap_write((*es8375).regmap, ES8375_RESET1, 0x00);
    msleep(80);
    regmap_write((*es8375).regmap, ES8375_CLK_MGR3, 0x00);
    regmap_write((*es8375).regmap, ES8375_CSM1, 0x86);
    regmap_write((*es8375).regmap, ES8375_CLK_MGR4, 0x0B);
    regmap_write((*es8375).regmap, ES8375_CLK_MGR5, 0x00);
    regmap_write((*es8375).regmap, ES8375_CLK_MGR6, 0x31);
    regmap_write((*es8375).regmap, ES8375_CLK_MGR7, 0x11);
    regmap_write((*es8375).regmap, ES8375_CLK_MGR8, 0x1F);
    regmap_write((*es8375).regmap, ES8375_CLK_MGR9, 0x00);
    regmap_write((*es8375).regmap, ES8375_ADC_OSR_GAIN, 0x1F);
    regmap_write((*es8375).regmap, ES8375_ADC2, 0x00);
    regmap_write((*es8375).regmap, ES8375_DAC2, 0x00);
    regmap_write((*es8375).regmap, ES8375_DAC_OTP, 0x88);
    regmap_write((*es8375).regmap, ES8375_ANALOG_SPK2, 0xE7);
    regmap_write((*es8375).regmap, ES8375_ANALOG2, 0xF0);
    regmap_write((*es8375).regmap, ES8375_ANALOG3, 0x40);
    regmap_write((*es8375).regmap, ES8375_CLK_MGR2, 0xFE);
    regmap_update_bits((*es8375).regmap, ES8375_SDP, 0x40, 0x40);
    regmap_update_bits((*es8375).regmap, ES8375_SDP2, 0x20, 0x20);
}

unsafe extern "C" fn es8375_suspend(component: *mut snd_soc_component) -> c_int {
    let es8375 = snd_soc_component_get_drvdata(component) as *mut es8375_priv;
    regmap_write((*es8375).regmap, ES8375_CSM1, 0x96);
    regcache_cache_only((*es8375).regmap, true);
    regcache_mark_dirty((*es8375).regmap);
    0
}

unsafe extern "C" fn es8375_resume(component: *mut snd_soc_component) -> c_int {
    let es8375 = snd_soc_component_get_drvdata(component) as *mut es8375_priv;
    let mut reg: c_uint = 0;
    regcache_cache_only((*es8375).regmap, false);
    regcache_cache_bypass((*es8375).regmap, true);
    regmap_read((*es8375).regmap, ES8375_CLK_MGR2, &mut reg);
    regcache_cache_bypass((*es8375).regmap, false);
    if reg == 0x00 { es8375_init(component); } else { es8375_set_bias_level(component, SND_SOC_BIAS_ON); }
    regcache_sync((*es8375).regmap);
    0
}

unsafe extern "C" fn es8375_codec_probe(component: *mut snd_soc_component) -> c_int {
    let es8375 = snd_soc_component_get_drvdata(component) as *mut es8375_priv;
    (*es8375).mastermode = 0;
    es8375_init(component);
    0
}

unsafe extern "C" fn es8375_writeable_register(_dev: *mut device, reg: c_uint) -> bool {
    if reg == ES8375_CHIP_VERSION || reg == ES8375_CHIP_ID0 || reg == ES8375_CHIP_ID1 ||
       reg == ES8375_SPK_OFFSET || reg == ES8375_FLAGS2 {
        false
    } else {
        true
    }
}

static mut es8375_regmap_config: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: 0,
    cache_type: REGCACHE_MAPLE,
    use_single_read: true,
    use_single_write: true,
    writeable_reg: Some(es8375_writeable_register),
};

static mut es8375_codec_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(es8375_codec_probe),
    suspend: Some(es8375_suspend),
    resume: Some(es8375_resume),
    set_bias_level: Some(es8375_set_bias_level),
    controls: es8375_snd_controls.as_ptr(),
    num_controls: 22,
    dapm_widgets: es8375_dapm_widgets.as_ptr(),
    num_dapm_widgets: 9,
    dapm_routes: es8375_dapm_routes.as_ptr(),
    num_dapm_routes: 7,
    idle_bias_on: 1,
    suspend_bias_off: 1,
};

unsafe fn es8375_read_device_properities(dev: *mut device, es8375: *mut es8375_priv) -> c_int {
    let mut ret: c_int;
    let mut i: usize;
    ret = device_property_read_u8(dev, cstr(b"everest,mclk-src\0"), &mut (*es8375).mclk_src);
    if ret != 0 { (*es8375).mclk_src = ES8375_MCLK_SOURCE; }
    dev_dbg(dev, cstr(b"mclk-src %x\0"), (*es8375).mclk_src as c_uint);
    i = 0;
    while i < es8375_core_supplies.len() {
        (*es8375).core_supply[i].supply = es8375_core_supplies[i];
        i += 1;
    }
    ret = devm_regulator_bulk_get(dev, es8375_core_supplies.len() as c_int, (*es8375).core_supply.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, cstr(b"Failed to request core supplies %d\n\0"), ret);
        return ret;
    }
    (*es8375).mclk = devm_clk_get(dev, cstr(b"mclk\0"));
    if IS_ERR((*es8375).mclk as *const c_void) {
        return dev_err_probe(dev, PTR_ERR((*es8375).mclk as *const c_void), cstr(b"unable to get mclk\n\0"));
    }
    if (*es8375).mclk.is_null() {
        dev_warn(dev, cstr(b"assuming static mclk\n\0"));
    }
    ret = clk_prepare_enable((*es8375).mclk);
    if ret != 0 {
        dev_err(dev, cstr(b"unable to enable mclk\n\0"));
        return ret;
    }
    ret = regulator_bulk_enable(es8375_core_supplies.len() as c_int, (*es8375).core_supply.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, cstr(b"Failed to enable core supplies: %d\n\0"), ret);
        clk_disable_unprepare((*es8375).mclk);
        return ret;
    }
    0
}

unsafe extern "C" fn es8375_i2c_probe(i2c_client: *mut i2c_client) -> c_int {
    let mut es8375: *mut es8375_priv;
    let dev: *mut device = &mut (*i2c_client).dev;
    let mut ret: c_int;
    let mut val: c_uint = 0;
    es8375 = devm_kzalloc(&mut (*i2c_client).dev, core::mem::size_of::<es8375_priv>(), GFP_KERNEL) as *mut es8375_priv;
    if es8375.is_null() { return -ENOMEM; }
    (*es8375).regmap = devm_regmap_init_i2c(i2c_client, &es8375_regmap_config);
    if IS_ERR((*es8375).regmap as *const c_void) {
        return dev_err_probe(&mut (*i2c_client).dev, PTR_ERR((*es8375).regmap as *const c_void), cstr(b"regmap_init() failed\n\0"));
    }
    i2c_set_clientdata(i2c_client, es8375 as *mut c_void);
    ret = regmap_read((*es8375).regmap, ES8375_CHIP_ID1, &mut val);
    if ret < 0 {
        dev_err(&mut (*i2c_client).dev, cstr(b"failed to read i2c at addr %X\n\0"), (*i2c_client).addr);
        return ret;
    }
    if val != 0x83 {
        dev_err(&mut (*i2c_client).dev, cstr(b"device at addr %X is not an es8375\n\0"), (*i2c_client).addr);
        return -ENODEV;
    }
    ret = regmap_read((*es8375).regmap, ES8375_CHIP_ID0, &mut val);
    if val != 0x75 {
        dev_err(&mut (*i2c_client).dev, cstr(b"device at addr %X is not an es8375\n\0"), (*i2c_client).addr);
        return -ENODEV;
    }
    ret = es8375_read_device_properities(dev, es8375);
    if ret != 0 {
        dev_err(&mut (*i2c_client).dev, cstr(b"get an error from dts info %X\n\0"), ret);
        return ret;
    }
    es8375_dai.playback.rates = es8375_RATES();
    es8375_dai.capture.rates = es8375_RATES();
    es8375_dai.playback.formats = es8375_FORMATS();
    es8375_dai.capture.formats = es8375_FORMATS();
    es8375_regmap_config.max_register = ES8375_REG_MAX;
    devm_snd_soc_register_component(&mut (*i2c_client).dev, &mut es8375_codec_driver, &mut es8375_dai, 1)
}

unsafe extern "C" fn es8375_i2c_shutdown(i2c: *mut i2c_client) {
    let es8375 = i2c_get_clientdata(i2c) as *mut es8375_priv;
    regmap_write((*es8375).regmap, ES8375_CSM1, 0x3C);
    regmap_write((*es8375).regmap, ES8375_CLK_MGR3, 0x48);
    regmap_write((*es8375).regmap, ES8375_CSM2, 0x80);
    regmap_write((*es8375).regmap, ES8375_CSM1, 0x3E);
    regmap_write((*es8375).regmap, ES8375_CLK_MGR10, 0x15);
    regmap_write((*es8375).regmap, ES8375_SYS_CTRL2, 0x0C);
    regmap_write((*es8375).regmap, ES8375_RESET1, 0x00);
    regmap_write((*es8375).regmap, ES8375_CSM2, 0x00);
    regulator_bulk_disable(es8375_core_supplies.len() as c_int, (*es8375).core_supply.as_mut_ptr());
    clk_disable_unprepare((*es8375).mclk);
}

static es8375_id: [i2c_device_id; 2] = [
    i2c_device_id { name: [b'e' as c_char, b's' as c_char, b'8' as c_char, b'3' as c_char, b'7' as c_char, b'5' as c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], driver_data: 0 },
    i2c_device_id { name: [0; 20], driver_data: 0 },
];
/* MODULE_DEVICE_TABLE(i2c, es8375_id); */

/* #ifdef CONFIG_ACPI */
static es8375_acpi_match: [acpi_device_id; 2] = [
    acpi_device_id { id: [b'E' as c_char, b'S' as c_char, b'S' as c_char, b'X' as c_char, b'8' as c_char, b'3' as c_char, b'7' as c_char, b'5' as c_char, 0, 0, 0, 0, 0, 0, 0, 0], driver_data: 0 },
    acpi_device_id { id: [0; 16], driver_data: 0 },
];
/* MODULE_DEVICE_TABLE(acpi, es8375_acpi_match); */

/* #ifdef CONFIG_OF */
static es8375_of_match: [of_device_id; 2] = [
    of_device_id { compatible: cstr(b"everest,es8375\0") },
    of_device_id { compatible: core::ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, es8375_of_match); */

static mut es8375_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: cstr(b"es8375\0"),
        of_match_table: core::ptr::null(),
        acpi_match_table: core::ptr::null(),
    },
    shutdown: Some(es8375_i2c_shutdown),
    probe: Some(es8375_i2c_probe),
    id_table: es8375_id.as_ptr(),
};
/* module_i2c_driver(es8375_i2c_driver);
 * MODULE_DESCRIPTION("ASoC ES8375 driver");
 * MODULE_AUTHOR("Michael Zhang <zhangyi@everest-semi.com>");
 * MODULE_LICENSE("GPL");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
