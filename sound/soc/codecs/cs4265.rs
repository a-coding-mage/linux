// SPDX-License-Identifier: GPL-2.0-only
/*
 * cs4265.c -- CS4265 ALSA SoC audio driver
 *
 * Copyright 2014 Cirrus Logic, Inc.
 *
 * Author: Paul Handrigan <paul.handrigan@cirrus.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type bool_ = bool;
type u8 = u8;
type u32 = u32;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;

extern "C" {
    static CS4265_PWRCTL: c_uint;
    static CS4265_DAC_CTL: c_uint;
    static CS4265_ADC_CTL: c_uint;
    static CS4265_MCLK_FREQ: c_uint;
    static CS4265_SIG_SEL: c_uint;
    static CS4265_CHB_PGA_CTL: c_uint;
    static CS4265_CHA_PGA_CTL: c_uint;
    static CS4265_ADC_CTL2: c_uint;
    static CS4265_DAC_CHA_VOL: c_uint;
    static CS4265_DAC_CHB_VOL: c_uint;
    static CS4265_DAC_CTL2: c_uint;
    static CS4265_INT_MASK: c_uint;
    static CS4265_STATUS_MODE_MSB: c_uint;
    static CS4265_STATUS_MODE_LSB: c_uint;
    static CS4265_SPDIF_CTL1: c_uint;
    static CS4265_SPDIF_CTL2: c_uint;
    static CS4265_CHIP_ID: c_uint;
    static CS4265_MAX_REGISTER: c_uint;
    static CS4265_INT_STATUS: c_uint;
    static CS4265_C_DATA_BUFF: c_uint;
    static CS4265_ADC_MASTER: c_uint;
    static CS4265_DAC_CTL_MUTE: c_uint;
    static CS4265_SPDIF_CTL2_MUTE: c_uint;
    static CS4265_ADC_FM: c_uint;
    static CS4265_MCLK_FREQ_MASK: c_uint;
    static CS4265_DAC_CTL_DIF: c_uint;
    static CS4265_ADC_DIF: c_uint;
    static CS4265_SPDIF_CTL2_DIF: c_uint;
    static CS4265_PWRCTL_PDN: c_uint;
    static CS4265_CHIP_ID_MASK: c_uint;
    static CS4265_CHIP_ID_VAL: c_uint;
    static CS4265_REV_ID_MASK: c_uint;

    static SND_SOC_NOPM: c_uint;
    static SND_SOC_DAIFMT_MASTER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SNDRV_PCM_STREAM_CAPTURE: c_uint;
    static SNDRV_PCM_RATE_32000: c_uint;
    static SNDRV_PCM_RATE_44100: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_RATE_64000: c_uint;
    static SNDRV_PCM_RATE_88200: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_RATE_176400: c_uint;
    static SNDRV_PCM_RATE_192000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_U16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    static SNDRV_PCM_FMTBIT_U24_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_uint;
    static SNDRV_PCM_FMTBIT_U32_LE: c_uint;
    static REGCACHE_MAPLE: c_uint;
    static GFP_KERNEL: c_uint;
    static GPIOD_OUT_LOW: c_uint;
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
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
pub struct snd_pcm_substream {
    pub stream: c_uint,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct cs4265_private {
    pub regmap: *mut regmap,
    pub reset_gpio: *mut gpio_desc,
    pub format: u8,
    pub sysclk: u32,
}

static cs4265_reg_defaults: [reg_default; 16] = unsafe {
    [
        reg_default { reg: CS4265_PWRCTL, def: 0x0F },
        reg_default { reg: CS4265_DAC_CTL, def: 0x08 },
        reg_default { reg: CS4265_ADC_CTL, def: 0x00 },
        reg_default { reg: CS4265_MCLK_FREQ, def: 0x00 },
        reg_default { reg: CS4265_SIG_SEL, def: 0x40 },
        reg_default { reg: CS4265_CHB_PGA_CTL, def: 0x00 },
        reg_default { reg: CS4265_CHA_PGA_CTL, def: 0x00 },
        reg_default { reg: CS4265_ADC_CTL2, def: 0x19 },
        reg_default { reg: CS4265_DAC_CHA_VOL, def: 0x00 },
        reg_default { reg: CS4265_DAC_CHB_VOL, def: 0x00 },
        reg_default { reg: CS4265_DAC_CTL2, def: 0xC0 },
        reg_default { reg: CS4265_INT_MASK, def: 0x00 },
        reg_default { reg: CS4265_STATUS_MODE_MSB, def: 0x00 },
        reg_default { reg: CS4265_STATUS_MODE_LSB, def: 0x00 },
        reg_default { reg: CS4265_SPDIF_CTL1, def: 0x00 },
        reg_default { reg: CS4265_SPDIF_CTL2, def: 0x00 },
    ]
};

unsafe extern "C" fn cs4265_readable_register(_dev: *mut device, reg: c_uint) -> bool_ {
    if reg >= CS4265_CHIP_ID && reg <= CS4265_MAX_REGISTER {
        true
    } else {
        false
    }
}

unsafe extern "C" fn cs4265_volatile_register(_dev: *mut device, reg: c_uint) -> bool_ {
    if reg == CS4265_INT_STATUS {
        true
    } else {
        false
    }
}

/* static DECLARE_TLV_DB_SCALE(pga_tlv, -1200, 50, 0); */
/* static DECLARE_TLV_DB_SCALE(dac_tlv, -12750, 50, 0); */

static digital_input_mux_text: [*const c_char; 2] = [b"SDIN1\0".as_ptr().cast(), b"SDIN2\0".as_ptr().cast()];
/* static SOC_ENUM_SINGLE_DECL(digital_input_mux_enum, CS4265_SIG_SEL, 7, digital_input_mux_text); */
/* static const struct snd_kcontrol_new digital_input_mux = SOC_DAPM_ENUM("Digital Input Mux", digital_input_mux_enum); */

static mic_linein_text: [*const c_char; 2] = [b"MIC\0".as_ptr().cast(), b"LINEIN\0".as_ptr().cast()];
/* static SOC_ENUM_SINGLE_DECL(mic_linein_enum, CS4265_ADC_CTL2, 0, mic_linein_text); */

static cam_mode_text: [*const c_char; 2] = [b"One Byte\0".as_ptr().cast(), b"Two Byte\0".as_ptr().cast()];
/* static SOC_ENUM_SINGLE_DECL(cam_mode_enum, CS4265_SPDIF_CTL1, 5, cam_mode_text); */

static cam_mono_stereo_text: [*const c_char; 2] = [b"Stereo\0".as_ptr().cast(), b"Mono\0".as_ptr().cast()];
/* static SOC_ENUM_SINGLE_DECL(spdif_mono_stereo_enum, CS4265_SPDIF_CTL2, 2, cam_mono_stereo_text); */

static mono_select_text: [*const c_char; 2] = [b"Channel A\0".as_ptr().cast(), b"Channel B\0".as_ptr().cast()];
/* static SOC_ENUM_SINGLE_DECL(spdif_mono_select_enum, CS4265_SPDIF_CTL2, 0, mono_select_text); */

/*
 * The following ALSA control, DAPM widget, and route tables are generated in C
 * through sound/soc.h macros. Their source-level contents are preserved here as
 * Rust comments because the macro-expanded struct layouts are external.
 *
 * static const struct snd_kcontrol_new mic_linein_mux =
 *     SOC_DAPM_ENUM("ADC Input Capture Mux", mic_linein_enum);
 * static const struct snd_kcontrol_new loopback_ctl =
 *     SOC_DAPM_SINGLE("Switch", CS4265_SIG_SEL, 1, 1, 0);
 * static const struct snd_kcontrol_new spdif_switch =
 *     SOC_DAPM_SINGLE("Switch", SND_SOC_NOPM, 0, 0, 0);
 * static const struct snd_kcontrol_new dac_switch =
 *     SOC_DAPM_SINGLE("Switch", CS4265_PWRCTL, 1, 1, 0);
 *
 * cs4265_snd_controls[]:
 * SOC_DOUBLE_R_SX_TLV("PGA Volume", CS4265_CHA_PGA_CTL, CS4265_CHB_PGA_CTL, 0, 0x28, 0x30, pga_tlv)
 * SOC_DOUBLE_R_TLV("DAC Volume", CS4265_DAC_CHA_VOL, CS4265_DAC_CHB_VOL, 0, 0xFF, 1, dac_tlv)
 * SOC_SINGLE("De-emp 44.1kHz Switch", CS4265_DAC_CTL, 1, 1, 0)
 * SOC_SINGLE("DAC INV Switch", CS4265_DAC_CTL2, 5, 1, 0)
 * SOC_SINGLE("DAC Zero Cross Switch", CS4265_DAC_CTL2, 6, 1, 0)
 * SOC_SINGLE("DAC Soft Ramp Switch", CS4265_DAC_CTL2, 7, 1, 0)
 * SOC_SINGLE("ADC HPF Switch", CS4265_ADC_CTL, 1, 1, 0)
 * SOC_SINGLE("ADC Zero Cross Switch", CS4265_ADC_CTL2, 3, 1, 1)
 * SOC_SINGLE("ADC Soft Ramp Switch", CS4265_ADC_CTL2, 7, 1, 0)
 * SOC_SINGLE("E to F Buffer Disable Switch", CS4265_SPDIF_CTL1, 6, 1, 0)
 * SOC_ENUM("C Data Access", cam_mode_enum)
 * SOC_SINGLE("Validity Bit Control Switch", CS4265_SPDIF_CTL2, 3, 1, 0)
 * SOC_ENUM("SPDIF Mono/Stereo", spdif_mono_stereo_enum)
 * SOC_SINGLE("MMTLR Data Switch", CS4265_SPDIF_CTL2, 0, 1, 0)
 * SOC_ENUM("Mono Channel Select", spdif_mono_select_enum)
 * SND_SOC_BYTES("C Data Buffer", CS4265_C_DATA_BUFF, 24)
 *
 * cs4265_dapm_widgets[]:
 * SND_SOC_DAPM_INPUT("LINEINL")
 * SND_SOC_DAPM_INPUT("LINEINR")
 * SND_SOC_DAPM_INPUT("MICL")
 * SND_SOC_DAPM_INPUT("MICR")
 * SND_SOC_DAPM_AIF_OUT("DOUT", NULL, 0, SND_SOC_NOPM, 0, 0)
 * SND_SOC_DAPM_AIF_OUT("SPDIFOUT", NULL, 0, SND_SOC_NOPM, 0, 0)
 * SND_SOC_DAPM_MUX("ADC Mux", SND_SOC_NOPM, 0, 0, &mic_linein_mux)
 * SND_SOC_DAPM_ADC("ADC", NULL, CS4265_PWRCTL, 2, 1)
 * SND_SOC_DAPM_PGA("Pre-amp MIC", CS4265_PWRCTL, 3, 1, NULL, 0)
 * SND_SOC_DAPM_MUX("Input Mux", SND_SOC_NOPM, 0, 0, &digital_input_mux)
 * SND_SOC_DAPM_MIXER("SDIN1 Input Mixer", SND_SOC_NOPM, 0, 0, NULL, 0)
 * SND_SOC_DAPM_MIXER("SDIN2 Input Mixer", SND_SOC_NOPM, 0, 0, NULL, 0)
 * SND_SOC_DAPM_MIXER("SPDIF Transmitter", SND_SOC_NOPM, 0, 0, NULL, 0)
 * SND_SOC_DAPM_SWITCH("Loopback", SND_SOC_NOPM, 0, 0, &loopback_ctl)
 * SND_SOC_DAPM_SWITCH("SPDIF", CS4265_SPDIF_CTL2, 5, 1, &spdif_switch)
 * SND_SOC_DAPM_SWITCH("DAC", CS4265_PWRCTL, 1, 1, &dac_switch)
 * SND_SOC_DAPM_AIF_IN("DIN1", NULL, 0, SND_SOC_NOPM, 0, 0)
 * SND_SOC_DAPM_AIF_IN("DIN2", NULL, 0, SND_SOC_NOPM, 0, 0)
 * SND_SOC_DAPM_AIF_IN("TXIN", NULL, 0, CS4265_SPDIF_CTL2, 5, 1)
 * SND_SOC_DAPM_OUTPUT("LINEOUTL")
 * SND_SOC_DAPM_OUTPUT("LINEOUTR")
 *
 * cs4265_audio_map[]:
 * {"DIN1", NULL, "DAI1 Playback"}, {"DIN2", NULL, "DAI2 Playback"},
 * {"SDIN1 Input Mixer", NULL, "DIN1"}, {"SDIN2 Input Mixer", NULL, "DIN2"},
 * {"Input Mux", "SDIN1", "SDIN1 Input Mixer"}, {"Input Mux", "SDIN2", "SDIN2 Input Mixer"},
 * {"DAC", "Switch", "Input Mux"}, {"SPDIF", "Switch", "Input Mux"},
 * {"LINEOUTL", NULL, "DAC"}, {"LINEOUTR", NULL, "DAC"}, {"SPDIFOUT", NULL, "SPDIF"},
 * {"Pre-amp MIC", NULL, "MICL"}, {"Pre-amp MIC", NULL, "MICR"},
 * {"ADC Mux", "MIC", "Pre-amp MIC"}, {"ADC Mux", "LINEIN", "LINEINL"},
 * {"ADC Mux", "LINEIN", "LINEINR"}, {"ADC", NULL, "ADC Mux"},
 * {"DOUT", NULL, "ADC"}, {"DAI1 Capture", NULL, "DOUT"}, {"DAI2 Capture", NULL, "DOUT"},
 * Loopback: {"Loopback", "Switch", "ADC"}, {"DAC", NULL, "Loopback"}
 */

#[repr(C)]
pub struct cs4265_clk_para {
    pub mclk: u32,
    pub rate: u32,
    pub fm_mode: u8, /* values 1, 2, or 4 */
    pub mclkdiv: u8,
}

static clk_map_table: [cs4265_clk_para; 45] = [
    /*32k*/
    cs4265_clk_para { mclk: 8192000, rate: 32000, fm_mode: 0, mclkdiv: 0 },
    cs4265_clk_para { mclk: 12288000, rate: 32000, fm_mode: 0, mclkdiv: 1 },
    cs4265_clk_para { mclk: 16384000, rate: 32000, fm_mode: 0, mclkdiv: 2 },
    cs4265_clk_para { mclk: 24576000, rate: 32000, fm_mode: 0, mclkdiv: 3 },
    cs4265_clk_para { mclk: 32768000, rate: 32000, fm_mode: 0, mclkdiv: 4 },
    /*44.1k*/
    cs4265_clk_para { mclk: 11289600, rate: 44100, fm_mode: 0, mclkdiv: 0 },
    cs4265_clk_para { mclk: 16934400, rate: 44100, fm_mode: 0, mclkdiv: 1 },
    cs4265_clk_para { mclk: 22579200, rate: 44100, fm_mode: 0, mclkdiv: 2 },
    cs4265_clk_para { mclk: 33868000, rate: 44100, fm_mode: 0, mclkdiv: 3 },
    cs4265_clk_para { mclk: 45158400, rate: 44100, fm_mode: 0, mclkdiv: 4 },
    /*48k*/
    cs4265_clk_para { mclk: 12288000, rate: 48000, fm_mode: 0, mclkdiv: 0 },
    cs4265_clk_para { mclk: 18432000, rate: 48000, fm_mode: 0, mclkdiv: 1 },
    cs4265_clk_para { mclk: 24576000, rate: 48000, fm_mode: 0, mclkdiv: 2 },
    cs4265_clk_para { mclk: 36864000, rate: 48000, fm_mode: 0, mclkdiv: 3 },
    cs4265_clk_para { mclk: 49152000, rate: 48000, fm_mode: 0, mclkdiv: 4 },
    /*64k*/
    cs4265_clk_para { mclk: 8192000, rate: 64000, fm_mode: 1, mclkdiv: 0 },
    cs4265_clk_para { mclk: 12288000, rate: 64000, fm_mode: 1, mclkdiv: 1 },
    cs4265_clk_para { mclk: 16934400, rate: 64000, fm_mode: 1, mclkdiv: 2 },
    cs4265_clk_para { mclk: 24576000, rate: 64000, fm_mode: 1, mclkdiv: 3 },
    cs4265_clk_para { mclk: 32768000, rate: 64000, fm_mode: 1, mclkdiv: 4 },
    /* 88.2k */
    cs4265_clk_para { mclk: 11289600, rate: 88200, fm_mode: 1, mclkdiv: 0 },
    cs4265_clk_para { mclk: 16934400, rate: 88200, fm_mode: 1, mclkdiv: 1 },
    cs4265_clk_para { mclk: 22579200, rate: 88200, fm_mode: 1, mclkdiv: 2 },
    cs4265_clk_para { mclk: 33868000, rate: 88200, fm_mode: 1, mclkdiv: 3 },
    cs4265_clk_para { mclk: 45158400, rate: 88200, fm_mode: 1, mclkdiv: 4 },
    /* 96k */
    cs4265_clk_para { mclk: 12288000, rate: 96000, fm_mode: 1, mclkdiv: 0 },
    cs4265_clk_para { mclk: 18432000, rate: 96000, fm_mode: 1, mclkdiv: 1 },
    cs4265_clk_para { mclk: 24576000, rate: 96000, fm_mode: 1, mclkdiv: 2 },
    cs4265_clk_para { mclk: 36864000, rate: 96000, fm_mode: 1, mclkdiv: 3 },
    cs4265_clk_para { mclk: 49152000, rate: 96000, fm_mode: 1, mclkdiv: 4 },
    /* 128k */
    cs4265_clk_para { mclk: 8192000, rate: 128000, fm_mode: 2, mclkdiv: 0 },
    cs4265_clk_para { mclk: 12288000, rate: 128000, fm_mode: 2, mclkdiv: 1 },
    cs4265_clk_para { mclk: 16934400, rate: 128000, fm_mode: 2, mclkdiv: 2 },
    cs4265_clk_para { mclk: 24576000, rate: 128000, fm_mode: 2, mclkdiv: 3 },
    cs4265_clk_para { mclk: 32768000, rate: 128000, fm_mode: 2, mclkdiv: 4 },
    /* 176.4k */
    cs4265_clk_para { mclk: 11289600, rate: 176400, fm_mode: 2, mclkdiv: 0 },
    cs4265_clk_para { mclk: 16934400, rate: 176400, fm_mode: 2, mclkdiv: 1 },
    cs4265_clk_para { mclk: 22579200, rate: 176400, fm_mode: 2, mclkdiv: 2 },
    cs4265_clk_para { mclk: 33868000, rate: 176400, fm_mode: 2, mclkdiv: 3 },
    cs4265_clk_para { mclk: 49152000, rate: 176400, fm_mode: 2, mclkdiv: 4 },
    /* 192k */
    cs4265_clk_para { mclk: 12288000, rate: 192000, fm_mode: 2, mclkdiv: 0 },
    cs4265_clk_para { mclk: 18432000, rate: 192000, fm_mode: 2, mclkdiv: 1 },
    cs4265_clk_para { mclk: 24576000, rate: 192000, fm_mode: 2, mclkdiv: 2 },
    cs4265_clk_para { mclk: 36864000, rate: 192000, fm_mode: 2, mclkdiv: 3 },
    cs4265_clk_para { mclk: 49152000, rate: 192000, fm_mode: 2, mclkdiv: 4 },
];

fn cs4265_get_clk_index(mclk: c_int, rate: c_int) -> c_int {
    let mut i: usize = 0;

    while i < clk_map_table.len() {
        if clk_map_table[i].rate == rate as u32 && clk_map_table[i].mclk == mclk as u32 {
            return i as c_int;
        }
        i += 1;
    }
    -EINVAL
}

extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_info(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn mdelay(msecs: c_uint);
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut c_void;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

unsafe extern "C" fn cs4265_set_sysclk(
    codec_dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = (*codec_dai).component;
    let cs4265 = snd_soc_component_get_drvdata(component) as *mut cs4265_private;
    let mut i: usize = 0;

    if clk_id != 0 {
        dev_err((*component).dev, b"Invalid clk_id %d\n\0".as_ptr().cast(), clk_id);
        return -EINVAL;
    }
    while i < clk_map_table.len() {
        if clk_map_table[i].mclk == freq {
            (*cs4265).sysclk = freq;
            return 0;
        }
        i += 1;
    }
    (*cs4265).sysclk = 0;
    dev_err((*component).dev, b"Invalid freq parameter %d\n\0".as_ptr().cast(), freq);
    -EINVAL
}

unsafe extern "C" fn cs4265_set_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let cs4265 = snd_soc_component_get_drvdata(component) as *mut cs4265_private;
    let mut iface: u8 = 0;

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        x if x == SND_SOC_DAIFMT_CBP_CFP => {
            snd_soc_component_update_bits(component, CS4265_ADC_CTL, CS4265_ADC_MASTER, CS4265_ADC_MASTER);
        }
        x if x == SND_SOC_DAIFMT_CBC_CFC => {
            snd_soc_component_update_bits(component, CS4265_ADC_CTL, CS4265_ADC_MASTER, 0);
        }
        _ => return -EINVAL,
    }

    /* interface format */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => iface |= SND_SOC_DAIFMT_I2S as u8,
        x if x == SND_SOC_DAIFMT_RIGHT_J => iface |= SND_SOC_DAIFMT_RIGHT_J as u8,
        x if x == SND_SOC_DAIFMT_LEFT_J => iface |= SND_SOC_DAIFMT_LEFT_J as u8,
        _ => return -EINVAL,
    }

    (*cs4265).format = iface;
    0
}

unsafe extern "C" fn cs4265_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*dai).component;

    if mute != 0 {
        snd_soc_component_update_bits(component, CS4265_DAC_CTL, CS4265_DAC_CTL_MUTE, CS4265_DAC_CTL_MUTE);
        snd_soc_component_update_bits(component, CS4265_SPDIF_CTL2, CS4265_SPDIF_CTL2_MUTE, CS4265_SPDIF_CTL2_MUTE);
    } else {
        snd_soc_component_update_bits(component, CS4265_DAC_CTL, CS4265_DAC_CTL_MUTE, 0);
        snd_soc_component_update_bits(component, CS4265_SPDIF_CTL2, CS4265_SPDIF_CTL2_MUTE, 0);
    }
    0
}

unsafe extern "C" fn cs4265_pcm_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let cs4265 = snd_soc_component_get_drvdata(component) as *mut cs4265_private;
    let index: c_int;

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE
        && (((*cs4265).format as c_uint & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_RIGHT_J)
    {
        return -EINVAL;
    }

    index = cs4265_get_clk_index((*cs4265).sysclk as c_int, params_rate(params));
    if index >= 0 {
        snd_soc_component_update_bits(
            component,
            CS4265_ADC_CTL,
            CS4265_ADC_FM,
            (clk_map_table[index as usize].fm_mode as c_uint) << 6,
        );
        snd_soc_component_update_bits(
            component,
            CS4265_MCLK_FREQ,
            CS4265_MCLK_FREQ_MASK,
            (clk_map_table[index as usize].mclkdiv as c_uint) << 4,
        );
    } else {
        dev_err((*component).dev, b"can't get correct mclk\n\0".as_ptr().cast());
        return -EINVAL;
    }

    match (*cs4265).format as c_uint & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => {
            snd_soc_component_update_bits(component, CS4265_DAC_CTL, CS4265_DAC_CTL_DIF, 1 << 4);
            snd_soc_component_update_bits(component, CS4265_ADC_CTL, CS4265_ADC_DIF, 1 << 4);
            snd_soc_component_update_bits(component, CS4265_SPDIF_CTL2, CS4265_SPDIF_CTL2_DIF, 1 << 6);
        }
        x if x == SND_SOC_DAIFMT_RIGHT_J => {
            if params_width(params) == 16 {
                snd_soc_component_update_bits(component, CS4265_DAC_CTL, CS4265_DAC_CTL_DIF, 2 << 4);
                snd_soc_component_update_bits(component, CS4265_SPDIF_CTL2, CS4265_SPDIF_CTL2_DIF, 2 << 6);
            } else {
                snd_soc_component_update_bits(component, CS4265_DAC_CTL, CS4265_DAC_CTL_DIF, 3 << 4);
                snd_soc_component_update_bits(component, CS4265_SPDIF_CTL2, CS4265_SPDIF_CTL2_DIF, 3 << 6);
            }
        }
        x if x == SND_SOC_DAIFMT_LEFT_J => {
            snd_soc_component_update_bits(component, CS4265_DAC_CTL, CS4265_DAC_CTL_DIF, 0);
            snd_soc_component_update_bits(component, CS4265_ADC_CTL, CS4265_ADC_DIF, 0);
            snd_soc_component_update_bits(component, CS4265_SPDIF_CTL2, CS4265_SPDIF_CTL2_DIF, 0);
        }
        _ => return -EINVAL,
    }
    0
}

#[repr(C)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON,
    SND_SOC_BIAS_PREPARE,
    SND_SOC_BIAS_STANDBY,
    SND_SOC_BIAS_OFF,
}

unsafe extern "C" fn cs4265_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => {}
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {
            snd_soc_component_update_bits(component, CS4265_PWRCTL, CS4265_PWRCTL_PDN, 0);
        }
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            snd_soc_component_update_bits(component, CS4265_PWRCTL, CS4265_PWRCTL_PDN, CS4265_PWRCTL_PDN);
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            snd_soc_component_update_bits(component, CS4265_PWRCTL, CS4265_PWRCTL_PDN, CS4265_PWRCTL_PDN);
        }
    }
    0
}

unsafe fn CS4265_RATES() -> c_uint {
    SNDRV_PCM_RATE_32000
        | SNDRV_PCM_RATE_44100
        | SNDRV_PCM_RATE_48000
        | SNDRV_PCM_RATE_64000
        | SNDRV_PCM_RATE_88200
        | SNDRV_PCM_RATE_96000
        | SNDRV_PCM_RATE_176400
        | SNDRV_PCM_RATE_192000
}

unsafe fn CS4265_FORMATS() -> c_uint {
    SNDRV_PCM_FMTBIT_S16_LE
        | SNDRV_PCM_FMTBIT_U16_LE
        | SNDRV_PCM_FMTBIT_S24_LE
        | SNDRV_PCM_FMTBIT_U24_LE
        | SNDRV_PCM_FMTBIT_S32_LE
        | SNDRV_PCM_FMTBIT_U32_LE
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub no_capture_mute: c_uint,
}

static cs4265_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(cs4265_pcm_hw_params),
    mute_stream: Some(cs4265_mute),
    set_fmt: Some(cs4265_set_fmt),
    set_sysclk: Some(cs4265_set_sysclk),
    no_capture_mute: 1,
};

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

static mut cs4265_dai: [snd_soc_dai_driver; 2] = unsafe {
    [
        snd_soc_dai_driver {
            name: b"cs4265-dai1\0".as_ptr().cast(),
            playback: snd_soc_pcm_stream {
                stream_name: b"DAI1 Playback\0".as_ptr().cast(),
                channels_min: 1,
                channels_max: 2,
                rates: CS4265_RATES(),
                formats: CS4265_FORMATS(),
            },
            capture: snd_soc_pcm_stream {
                stream_name: b"DAI1 Capture\0".as_ptr().cast(),
                channels_min: 1,
                channels_max: 2,
                rates: CS4265_RATES(),
                formats: CS4265_FORMATS(),
            },
            ops: &cs4265_ops,
        },
        snd_soc_dai_driver {
            name: b"cs4265-dai2\0".as_ptr().cast(),
            playback: snd_soc_pcm_stream {
                stream_name: b"DAI2 Playback\0".as_ptr().cast(),
                channels_min: 1,
                channels_max: 2,
                rates: CS4265_RATES(),
                formats: CS4265_FORMATS(),
            },
            capture: snd_soc_pcm_stream {
                stream_name: b"DAI2 Capture\0".as_ptr().cast(),
                channels_min: 1,
                channels_max: 2,
                rates: CS4265_RATES(),
                formats: CS4265_FORMATS(),
            },
            ops: &cs4265_ops,
        },
    ]
};

#[repr(C)]
pub struct snd_soc_component_driver {
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const c_void,
    pub num_controls: c_uint,
    pub dapm_widgets: *const c_void,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const c_void,
    pub num_dapm_routes: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

static soc_component_cs4265: snd_soc_component_driver = snd_soc_component_driver {
    set_bias_level: Some(cs4265_set_bias_level),
    controls: core::ptr::null(),
    num_controls: 16,
    dapm_widgets: core::ptr::null(),
    num_dapm_widgets: 23,
    dapm_routes: core::ptr::null(),
    num_dapm_routes: 23,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub cache_type: c_uint,
}

static cs4265_regmap: regmap_config = unsafe {
    regmap_config {
        reg_bits: 8,
        val_bits: 8,
        max_register: CS4265_MAX_REGISTER,
        reg_defaults: cs4265_reg_defaults.as_ptr(),
        num_reg_defaults: cs4265_reg_defaults.len() as c_uint,
        readable_reg: Some(cs4265_readable_register),
        volatile_reg: Some(cs4265_volatile_register),
        cache_type: REGCACHE_MAPLE,
    }
};

unsafe extern "C" fn cs4265_i2c_probe(i2c_client: *mut i2c_client) -> c_int {
    let cs4265: *mut cs4265_private;
    let mut ret: c_int;
    let mut devid: c_uint = 0;
    let mut reg: c_uint = 0;

    cs4265 = devm_kzalloc(
        &mut (*i2c_client).dev,
        core::mem::size_of::<cs4265_private>(),
        GFP_KERNEL,
    ) as *mut cs4265_private;
    if cs4265.is_null() {
        return -ENOMEM;
    }

    (*cs4265).regmap = devm_regmap_init_i2c(i2c_client, &cs4265_regmap);
    if IS_ERR((*cs4265).regmap.cast()) {
        ret = PTR_ERR((*cs4265).regmap.cast());
        dev_err(&mut (*i2c_client).dev, b"regmap_init() failed: %d\n\0".as_ptr().cast(), ret);
        return ret;
    }

    (*cs4265).reset_gpio =
        devm_gpiod_get_optional(&mut (*i2c_client).dev, b"reset\0".as_ptr().cast(), GPIOD_OUT_LOW);
    if IS_ERR((*cs4265).reset_gpio.cast()) {
        return PTR_ERR((*cs4265).reset_gpio.cast());
    }

    if !(*cs4265).reset_gpio.is_null() {
        mdelay(1);
        gpiod_set_value_cansleep((*cs4265).reset_gpio, 1);
    }

    i2c_set_clientdata(i2c_client, cs4265.cast());

    ret = regmap_read((*cs4265).regmap, CS4265_CHIP_ID, &mut reg);
    if ret != 0 {
        dev_err(&mut (*i2c_client).dev, b"Failed to read chip ID: %d\n\0".as_ptr().cast(), ret);
        return ret;
    }

    devid = reg & CS4265_CHIP_ID_MASK;
    if devid != CS4265_CHIP_ID_VAL {
        ret = -ENODEV;
        dev_err(
            &mut (*i2c_client).dev,
            b"CS4265 Part Number ID: 0x%x Expected: 0x%x\n\0".as_ptr().cast(),
            devid >> 4,
            CS4265_CHIP_ID_VAL >> 4,
        );
        return ret;
    }
    dev_info(
        &mut (*i2c_client).dev,
        b"CS4265 Version %x\n\0".as_ptr().cast(),
        reg & CS4265_REV_ID_MASK,
    );

    regmap_write((*cs4265).regmap, CS4265_PWRCTL, 0x0F);

    devm_snd_soc_register_component(
        &mut (*i2c_client).dev,
        &soc_component_cs4265,
        cs4265_dai.as_mut_ptr(),
        cs4265_dai.len() as c_int,
    )
}

unsafe extern "C" fn cs4265_i2c_remove(i2c: *mut i2c_client) {
    let cs4265 = i2c_get_clientdata(i2c) as *mut cs4265_private;

    if !(*cs4265).reset_gpio.is_null() {
        gpiod_set_value_cansleep((*cs4265).reset_gpio, 0);
    }
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

static cs4265_of_match: [of_device_id; 2] = [
    of_device_id { compatible: b"cirrus,cs4265\0".as_ptr().cast() },
    of_device_id { compatible: core::ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, cs4265_of_match); */

#[repr(C)]
pub struct i2c_device_id {
    pub name: *const c_char,
}

static cs4265_id: [i2c_device_id; 2] = [
    i2c_device_id { name: b"cs4265\0".as_ptr().cast() },
    i2c_device_id { name: core::ptr::null() },
];
/* MODULE_DEVICE_TABLE(i2c, cs4265_id); */

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub id_table: *const i2c_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
}

static cs4265_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"cs4265\0".as_ptr().cast(),
        of_match_table: cs4265_of_match.as_ptr(),
    },
    id_table: cs4265_id.as_ptr(),
    probe: Some(cs4265_i2c_probe),
    remove: Some(cs4265_i2c_remove),
};

/* module_i2c_driver(cs4265_i2c_driver); */

/* MODULE_DESCRIPTION("ASoC CS4265 driver"); */
/* MODULE_AUTHOR("Paul Handrigan, Cirrus Logic Inc, <paul.handrigan@cirrus.com>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
