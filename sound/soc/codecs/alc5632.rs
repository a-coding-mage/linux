// SPDX-License-Identifier: GPL-2.0-only
/*
* alc5632.c  --  ALC5632 ALSA SoC Audio Codec
*
* Copyright (C) 2011 The AC100 Kernel Team <ac100@lists.lauchpad.net>
*
* Authors:  Leon Romanovsky <leon@leon.nu>
*           Andrey Danin <danindrey@mail.ru>
*           Ilya Petrov <ilya.muromec@gmail.com>
*           Marc Dietrich <marvin24@gmx.de>
*
* Based on alc5623.c by Arnaud Patard
*/

// C dependencies:
// linux/module.h, linux/kernel.h, linux/init.h, linux/delay.h, linux/pm.h,
// linux/i2c.h, linux/slab.h, linux/regmap.h, sound/core.h, sound/pcm.h,
// sound/pcm_params.h, sound/tlv.h, sound/soc.h, sound/initval.h, alc5632.h

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
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
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_component {
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
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: *const c_char,
    pub driver_data: usize,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct i2c_driver {
    _private: [u8; 0],
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
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_pll:
        Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int, c_uint, c_uint) -> c_int>,
    pub no_capture_mute: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub rates: c_uint,
    pub formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
}

#[repr(C)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON,
    SND_SOC_BIAS_PREPARE,
    SND_SOC_BIAS_STANDBY,
    SND_SOC_BIAS_OFF,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub set_bias_level:
        Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_desc,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub suspend_bias_off: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub cache_type: c_uint,
}

#[repr(C)]
struct alc5632_priv {
    regmap: *mut regmap,
    id: u8,
    sysclk: c_uint,
}

unsafe extern "C" {
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_write(
        component: *mut snd_soc_component,
        reg: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_add_component_controls(
        component: *mut snd_soc_component,
        controls: *const snd_kcontrol_new,
        num_controls: c_uint,
    ) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn msleep(msecs: c_uint);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn i2c_get_match_data(client: *mut i2c_client) -> *const c_void;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EIO: c_int = 5;
const GFP_KERNEL: c_uint = 0;

const fn array_size<T, const N: usize>(_: &[T; N]) -> c_uint {
    N as c_uint
}

/*
 * ALC5632 register cache
 */
static alc5632_reg_defaults: [reg_default; 38] = [
    reg_default { reg: 2, def: 0x8080 },   /* R2   - Speaker Output Volume */
    reg_default { reg: 4, def: 0x8080 },   /* R4   - Headphone Output Volume */
    reg_default { reg: 6, def: 0x8080 },   /* R6   - AUXOUT Volume */
    reg_default { reg: 8, def: 0xC800 },   /* R8   - Phone Input */
    reg_default { reg: 10, def: 0xE808 },  /* R10  - LINE_IN Volume */
    reg_default { reg: 12, def: 0x1010 },  /* R12  - STEREO DAC Input Volume */
    reg_default { reg: 14, def: 0x0808 },  /* R14  - MIC Input Volume */
    reg_default { reg: 16, def: 0xEE0F },  /* R16  - Stereo DAC and MIC Routing Control */
    reg_default { reg: 18, def: 0xCBCB },  /* R18  - ADC Record Gain */
    reg_default { reg: 20, def: 0x7F7F },  /* R20  - ADC Record Mixer Control */
    reg_default { reg: 24, def: 0xE010 },  /* R24  - Voice DAC Volume */
    reg_default { reg: 28, def: 0x8008 },  /* R28  - Output Mixer Control */
    reg_default { reg: 34, def: 0x0000 },  /* R34  - Microphone Control */
    reg_default { reg: 36, def: 0x00C0 },  /* R36  - Codec Digital MIC/Digital Boost Control */
    reg_default { reg: 46, def: 0x0000 },  /* R46  - Stereo DAC/Voice DAC/Stereo ADC Function Select */
    reg_default { reg: 52, def: 0x8000 },  /* R52  - Main Serial Data Port Control (Stereo I2S) */
    reg_default { reg: 54, def: 0x0000 },  /* R54  - Extend Serial Data Port Control (VoDAC_I2S/PCM) */
    reg_default { reg: 58, def: 0x0000 },  /* R58  - Power Management Addition 1 */
    reg_default { reg: 60, def: 0x0000 },  /* R60  - Power Management Addition 2 */
    reg_default { reg: 62, def: 0x8000 },  /* R62  - Power Management Addition 3 */
    reg_default { reg: 64, def: 0x0C0A },  /* R64  - General Purpose Control Register 1 */
    reg_default { reg: 66, def: 0x0000 },  /* R66  - General Purpose Control Register 2 */
    reg_default { reg: 68, def: 0x0000 },  /* R68  - PLL1 Control */
    reg_default { reg: 70, def: 0x0000 },  /* R70  - PLL2 Control */
    reg_default { reg: 76, def: 0xBE3E },  /* R76  - GPIO Pin Configuration */
    reg_default { reg: 78, def: 0xBE3E },  /* R78  - GPIO Pin Polarity */
    reg_default { reg: 80, def: 0x0000 },  /* R80  - GPIO Pin Sticky */
    reg_default { reg: 82, def: 0x0000 },  /* R82  - GPIO Pin Wake Up */
    reg_default { reg: 86, def: 0x0000 },  /* R86  - Pin Sharing */
    reg_default { reg: 90, def: 0x0009 },  /* R90  - Soft Volume Control Setting */
    reg_default { reg: 92, def: 0x0000 },  /* R92  - GPIO_Output Pin Control */
    reg_default { reg: 94, def: 0x3000 },  /* R94  - MISC Control */
    reg_default { reg: 96, def: 0x3075 },  /* R96  - Stereo DAC Clock Control_1 */
    reg_default { reg: 98, def: 0x1010 },  /* R98  - Stereo DAC Clock Control_2 */
    reg_default { reg: 100, def: 0x3110 }, /* R100 - VoDAC_PCM Clock Control_1 */
    reg_default { reg: 104, def: 0x0553 }, /* R104 - Pseudo Stereo and Spatial Effect Block Control */
    reg_default { reg: 106, def: 0x0000 }, /* R106 - Private Register Address */
    reg_default { reg: 0, def: 0 },
];

unsafe extern "C" fn alc5632_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        ALC5632_RESET
        | ALC5632_PWR_DOWN_CTRL_STATUS
        | ALC5632_GPIO_PIN_STATUS
        | ALC5632_OVER_CURR_STATUS
        | ALC5632_HID_CTRL_DATA
        | ALC5632_EQ_CTRL
        | ALC5632_VENDOR_ID1
        | ALC5632_VENDOR_ID2 => true,
        _ => false,
    }
}

unsafe fn alc5632_reset(map: *mut regmap) -> c_int {
    unsafe { regmap_write(map, ALC5632_RESET, 0x59B4) }
}

unsafe extern "C" fn amp_mixer_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = unsafe { snd_soc_dapm_to_component((*w).dapm) };

    /* to power-on/off class-d amp generators/speaker */
    /* need to write to 'index-46h' register :        */
    /* so write index num (here 0x46) to reg 0x6a     */
    /* and then 0xffff/0 to reg 0x6c                  */
    unsafe {
        snd_soc_component_write(component, ALC5632_HID_CTRL_INDEX, 0x46);
    }

    match event {
        SND_SOC_DAPM_PRE_PMU => unsafe {
            snd_soc_component_write(component, ALC5632_HID_CTRL_DATA, 0xFFFF);
        },
        SND_SOC_DAPM_POST_PMD => unsafe {
            snd_soc_component_write(component, ALC5632_HID_CTRL_DATA, 0);
        },
        _ => {}
    }

    0
}

/*
 * ALC5632 Controls
 */

/* -34.5db min scale, 1.5db steps, no mute */
static vol_tlv: &[c_uint] = DECLARE_TLV_DB_SCALE!(vol_tlv, -3450, 150, 0);
/* -46.5db min scale, 1.5db steps, no mute */
static hp_tlv: &[c_uint] = DECLARE_TLV_DB_SCALE!(hp_tlv, -4650, 150, 0);
/* -16.5db min scale, 1.5db steps, no mute */
static adc_rec_tlv: &[c_uint] = DECLARE_TLV_DB_SCALE!(adc_rec_tlv, -1650, 150, 0);
static boost_tlv: &[c_uint] = DECLARE_TLV_DB_RANGE!(
    boost_tlv,
    0,
    1,
    TLV_DB_SCALE_ITEM!(0, 2000, 0),
    1,
    3,
    TLV_DB_SCALE_ITEM!(2000, 1000, 0)
);
/* 0db min scale, 6 db steps, no mute */
static dig_tlv: &[c_uint] = DECLARE_TLV_DB_SCALE!(dig_tlv, 0, 600, 0);
/* 0db min scalem 0.75db steps, no mute */
static vdac_tlv: &[c_uint] = DECLARE_TLV_DB_SCALE!(vdac_tlv, -3525, 75, 0);

static alc5632_vol_snd_controls: &[snd_kcontrol_new] = &[
    /* left starts at bit 8, right at bit 0 */
    /* 31 steps (5 bit), -46.5db scale */
    SOC_DOUBLE_TLV!("Speaker Playback Volume", ALC5632_SPK_OUT_VOL, 8, 0, 31, 1, hp_tlv),
    /* bit 15 mutes left, bit 7 right */
    SOC_DOUBLE!("Speaker Playback Switch", ALC5632_SPK_OUT_VOL, 15, 7, 1, 1),
    SOC_DOUBLE_TLV!("Headphone Playback Volume", ALC5632_HP_OUT_VOL, 8, 0, 31, 1, hp_tlv),
    SOC_DOUBLE!("Headphone Playback Switch", ALC5632_HP_OUT_VOL, 15, 7, 1, 1),
];

static alc5632_snd_controls: &[snd_kcontrol_new] = &[
    SOC_DOUBLE_TLV!("Auxout Playback Volume", ALC5632_AUX_OUT_VOL, 8, 0, 31, 1, hp_tlv),
    SOC_DOUBLE!("Auxout Playback Switch", ALC5632_AUX_OUT_VOL, 15, 7, 1, 1),
    SOC_SINGLE_TLV!("Voice DAC Playback Volume", ALC5632_VOICE_DAC_VOL, 0, 63, 0, vdac_tlv),
    SOC_SINGLE!("Voice DAC Playback Switch", ALC5632_VOICE_DAC_VOL, 12, 1, 1),
    SOC_SINGLE_TLV!("Phone Playback Volume", ALC5632_PHONE_IN_VOL, 8, 31, 1, vol_tlv),
    SOC_DOUBLE_TLV!("LineIn Playback Volume", ALC5632_LINE_IN_VOL, 8, 0, 31, 1, vol_tlv),
    SOC_DOUBLE_TLV!("Master Playback Volume", ALC5632_STEREO_DAC_IN_VOL, 8, 0, 63, 1, vdac_tlv),
    SOC_DOUBLE!("Master Playback Switch", ALC5632_STEREO_DAC_IN_VOL, 15, 7, 1, 1),
    SOC_SINGLE_TLV!("Mic1 Playback Volume", ALC5632_MIC_VOL, 8, 31, 1, vol_tlv),
    SOC_SINGLE_TLV!("Mic2 Playback Volume", ALC5632_MIC_VOL, 0, 31, 1, vol_tlv),
    SOC_DOUBLE_TLV!("Rec Capture Volume", ALC5632_ADC_REC_GAIN, 8, 0, 31, 0, adc_rec_tlv),
    SOC_SINGLE_TLV!("Mic 1 Boost Volume", ALC5632_MIC_CTRL, 10, 3, 0, boost_tlv),
    SOC_SINGLE_TLV!("Mic 2 Boost Volume", ALC5632_MIC_CTRL, 8, 3, 0, boost_tlv),
    SOC_SINGLE_TLV!("DMIC Boost Capture Volume", ALC5632_DIGI_BOOST_CTRL, 0, 7, 0, dig_tlv),
    SOC_SINGLE!("DMIC En Capture Switch", ALC5632_DIGI_BOOST_CTRL, 15, 1, 0),
    SOC_SINGLE!("DMIC PreFilter Capture Switch", ALC5632_DIGI_BOOST_CTRL, 12, 1, 0),
];

// DAPM controls/widgets/routes are macro-created kernel data in C.
static alc5632_hp_mixer_controls: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("LI2HP Playback Switch", ALC5632_LINE_IN_VOL, 15, 1, 1),
    SOC_DAPM_SINGLE!("PHONE2HP Playback Switch", ALC5632_PHONE_IN_VOL, 15, 1, 1),
    SOC_DAPM_SINGLE!("MIC12HP Playback Switch", ALC5632_MIC_ROUTING_CTRL, 15, 1, 1),
    SOC_DAPM_SINGLE!("MIC22HP Playback Switch", ALC5632_MIC_ROUTING_CTRL, 11, 1, 1),
    SOC_DAPM_SINGLE!("VOICE2HP Playback Switch", ALC5632_VOICE_DAC_VOL, 15, 1, 1),
];

static alc5632_hpl_mixer_controls: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("ADC2HP_L Playback Switch", ALC5632_ADC_REC_GAIN, 15, 1, 1),
    SOC_DAPM_SINGLE!("DACL2HP Playback Switch", ALC5632_MIC_ROUTING_CTRL, 3, 1, 1),
];

static alc5632_hpr_mixer_controls: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("ADC2HP_R Playback Switch", ALC5632_ADC_REC_GAIN, 7, 1, 1),
    SOC_DAPM_SINGLE!("DACR2HP Playback Switch", ALC5632_MIC_ROUTING_CTRL, 2, 1, 1),
];

static alc5632_mono_mixer_controls: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("ADC2MONO_L Playback Switch", ALC5632_ADC_REC_GAIN, 14, 1, 1),
    SOC_DAPM_SINGLE!("ADC2MONO_R Playback Switch", ALC5632_ADC_REC_GAIN, 6, 1, 1),
    SOC_DAPM_SINGLE!("LI2MONO Playback Switch", ALC5632_LINE_IN_VOL, 13, 1, 1),
    SOC_DAPM_SINGLE!("MIC12MONO Playback Switch", ALC5632_MIC_ROUTING_CTRL, 13, 1, 1),
    SOC_DAPM_SINGLE!("MIC22MONO Playback Switch", ALC5632_MIC_ROUTING_CTRL, 9, 1, 1),
    SOC_DAPM_SINGLE!("DAC2MONO Playback Switch", ALC5632_MIC_ROUTING_CTRL, 0, 1, 1),
    SOC_DAPM_SINGLE!("VOICE2MONO Playback Switch", ALC5632_VOICE_DAC_VOL, 13, 1, 1),
];

static alc5632_speaker_mixer_controls: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("LI2SPK Playback Switch", ALC5632_LINE_IN_VOL, 14, 1, 1),
    SOC_DAPM_SINGLE!("PHONE2SPK Playback Switch", ALC5632_PHONE_IN_VOL, 14, 1, 1),
    SOC_DAPM_SINGLE!("MIC12SPK Playback Switch", ALC5632_MIC_ROUTING_CTRL, 14, 1, 1),
    SOC_DAPM_SINGLE!("MIC22SPK Playback Switch", ALC5632_MIC_ROUTING_CTRL, 10, 1, 1),
    SOC_DAPM_SINGLE!("DAC2SPK Playback Switch", ALC5632_MIC_ROUTING_CTRL, 1, 1, 1),
    SOC_DAPM_SINGLE!("VOICE2SPK Playback Switch", ALC5632_VOICE_DAC_VOL, 14, 1, 1),
];

/* Left Record Mixer */
static alc5632_captureL_mixer_controls: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("MIC12REC_L Capture Switch", ALC5632_ADC_REC_MIXER, 14, 1, 1),
    SOC_DAPM_SINGLE!("MIC22REC_L Capture Switch", ALC5632_ADC_REC_MIXER, 13, 1, 1),
    SOC_DAPM_SINGLE!("LIL2REC Capture Switch", ALC5632_ADC_REC_MIXER, 12, 1, 1),
    SOC_DAPM_SINGLE!("PH2REC_L Capture Switch", ALC5632_ADC_REC_MIXER, 11, 1, 1),
    SOC_DAPM_SINGLE!("HPL2REC Capture Switch", ALC5632_ADC_REC_MIXER, 10, 1, 1),
    SOC_DAPM_SINGLE!("SPK2REC_L Capture Switch", ALC5632_ADC_REC_MIXER, 9, 1, 1),
    SOC_DAPM_SINGLE!("MONO2REC_L Capture Switch", ALC5632_ADC_REC_MIXER, 8, 1, 1),
];

/* Right Record Mixer */
static alc5632_captureR_mixer_controls: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("MIC12REC_R Capture Switch", ALC5632_ADC_REC_MIXER, 6, 1, 1),
    SOC_DAPM_SINGLE!("MIC22REC_R Capture Switch", ALC5632_ADC_REC_MIXER, 5, 1, 1),
    SOC_DAPM_SINGLE!("LIR2REC Capture Switch", ALC5632_ADC_REC_MIXER, 4, 1, 1),
    SOC_DAPM_SINGLE!("PH2REC_R Capture Switch", ALC5632_ADC_REC_MIXER, 3, 1, 1),
    SOC_DAPM_SINGLE!("HPR2REC Capture Switch", ALC5632_ADC_REC_MIXER, 2, 1, 1),
    SOC_DAPM_SINGLE!("SPK2REC_R Capture Switch", ALC5632_ADC_REC_MIXER, 1, 1, 1),
    SOC_DAPM_SINGLE!("MONO2REC_R Capture Switch", ALC5632_ADC_REC_MIXER, 0, 1, 1),
];

/* Dmic Mixer */
static alc5632_dmicl_mixer_controls: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("DMICL2ADC Capture Switch", ALC5632_DIGI_BOOST_CTRL, 7, 1, 1),
];
static alc5632_dmicr_mixer_controls: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("DMICR2ADC Capture Switch", ALC5632_DIGI_BOOST_CTRL, 6, 1, 1),
];

static alc5632_spk_n_sour_sel: [&[u8]; 4] = [b"RN/-R\0", b"RP/+R\0", b"LN/-R\0", b"Mute\0"];
static alc5632_hpl_out_input_sel: [&[u8]; 2] = [b"Vmid\0", b"HP Left Mix\0"];
static alc5632_hpr_out_input_sel: [&[u8]; 2] = [b"Vmid\0", b"HP Right Mix\0"];
static alc5632_spkout_input_sel: [&[u8]; 4] =
    [b"Vmid\0", b"HPOut Mix\0", b"Speaker Mix\0", b"Mono Mix\0"];
static alc5632_aux_out_input_sel: [&[u8]; 4] =
    [b"Vmid\0", b"HPOut Mix\0", b"Speaker Mix\0", b"Mono Mix\0"];
static alc5632_adcr_func_sel: [&[u8]; 2] = [b"Stereo ADC\0", b"Voice ADC\0"];
static alc5632_i2s_out_sel: [&[u8]; 2] = [b"ADC LR\0", b"Voice Stereo Digital\0"];

static alc5632_aux_out_input_enum: _ = SOC_ENUM_SINGLE_DECL!(
    alc5632_aux_out_input_enum,
    ALC5632_OUTPUT_MIXER_CTRL,
    6,
    alc5632_aux_out_input_sel
);
static alc5632_auxout_mux_controls: snd_kcontrol_new =
    SOC_DAPM_ENUM!("AuxOut Mux", alc5632_aux_out_input_enum);

static alc5632_spkout_input_enum: _ = SOC_ENUM_SINGLE_DECL!(
    alc5632_spkout_input_enum,
    ALC5632_OUTPUT_MIXER_CTRL,
    10,
    alc5632_spkout_input_sel
);
static alc5632_spkout_mux_controls: snd_kcontrol_new =
    SOC_DAPM_ENUM!("SpeakerOut Mux", alc5632_spkout_input_enum);

static alc5632_hpl_out_input_enum: _ = SOC_ENUM_SINGLE_DECL!(
    alc5632_hpl_out_input_enum,
    ALC5632_OUTPUT_MIXER_CTRL,
    9,
    alc5632_hpl_out_input_sel
);
static alc5632_hpl_out_mux_controls: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Left Headphone Mux", alc5632_hpl_out_input_enum);

static alc5632_hpr_out_input_enum: _ = SOC_ENUM_SINGLE_DECL!(
    alc5632_hpr_out_input_enum,
    ALC5632_OUTPUT_MIXER_CTRL,
    8,
    alc5632_hpr_out_input_sel
);
static alc5632_hpr_out_mux_controls: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Right Headphone Mux", alc5632_hpr_out_input_enum);

static alc5632_spk_n_sour_enum: _ = SOC_ENUM_SINGLE_DECL!(
    alc5632_spk_n_sour_enum,
    ALC5632_OUTPUT_MIXER_CTRL,
    14,
    alc5632_spk_n_sour_sel
);
static alc5632_spkoutn_mux_controls: snd_kcontrol_new =
    SOC_DAPM_ENUM!("SpeakerOut N Mux", alc5632_spk_n_sour_enum);

/* speaker amplifier */
static alc5632_amp_names: [&[u8]; 2] = [b"AB Amp\0", b"D Amp\0"];
static alc5632_amp_enum: _ =
    SOC_ENUM_SINGLE_DECL!(alc5632_amp_enum, ALC5632_OUTPUT_MIXER_CTRL, 13, alc5632_amp_names);
static alc5632_amp_mux_controls: snd_kcontrol_new =
    SOC_DAPM_ENUM!("AB-D Amp Mux", alc5632_amp_enum);

static alc5632_adcr_func_enum: _ =
    SOC_ENUM_SINGLE_DECL!(alc5632_adcr_func_enum, ALC5632_DAC_FUNC_SELECT, 5, alc5632_adcr_func_sel);
static alc5632_adcr_func_controls: snd_kcontrol_new =
    SOC_DAPM_ENUM!("ADCR Mux", alc5632_adcr_func_enum);

static alc5632_i2s_out_enum: _ =
    SOC_ENUM_SINGLE_DECL!(alc5632_i2s_out_enum, ALC5632_I2S_OUT_CTL, 5, alc5632_i2s_out_sel);
static alc5632_i2s_out_controls: snd_kcontrol_new =
    SOC_DAPM_ENUM!("I2SOut Mux", alc5632_i2s_out_enum);

static alc5632_dapm_widgets: &[snd_soc_dapm_widget_desc] = &[
    /* Muxes */
    SND_SOC_DAPM_MUX!("AuxOut Mux", SND_SOC_NOPM, 0, 0, &alc5632_auxout_mux_controls),
    SND_SOC_DAPM_MUX!("SpeakerOut Mux", SND_SOC_NOPM, 0, 0, &alc5632_spkout_mux_controls),
    SND_SOC_DAPM_MUX!("Left Headphone Mux", SND_SOC_NOPM, 0, 0, &alc5632_hpl_out_mux_controls),
    SND_SOC_DAPM_MUX!("Right Headphone Mux", SND_SOC_NOPM, 0, 0, &alc5632_hpr_out_mux_controls),
    SND_SOC_DAPM_MUX!("SpeakerOut N Mux", SND_SOC_NOPM, 0, 0, &alc5632_spkoutn_mux_controls),
    SND_SOC_DAPM_MUX!("ADCR Mux", SND_SOC_NOPM, 0, 0, &alc5632_adcr_func_controls),
    SND_SOC_DAPM_MUX!("I2SOut Mux", ALC5632_PWR_MANAG_ADD1, 11, 0, &alc5632_i2s_out_controls),
    SND_SOC_DAPM_MIXER!("HP Mix", SND_SOC_NOPM, 0, 0, &alc5632_hp_mixer_controls[0], alc5632_hp_mixer_controls.len()),
    SND_SOC_DAPM_MIXER!("HPR Mix", ALC5632_PWR_MANAG_ADD2, 4, 0, &alc5632_hpr_mixer_controls[0], alc5632_hpr_mixer_controls.len()),
    SND_SOC_DAPM_MIXER!("HPL Mix", ALC5632_PWR_MANAG_ADD2, 5, 0, &alc5632_hpl_mixer_controls[0], alc5632_hpl_mixer_controls.len()),
    SND_SOC_DAPM_MIXER!("HPOut Mix", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("Mono Mix", ALC5632_PWR_MANAG_ADD2, 2, 0, &alc5632_mono_mixer_controls[0], alc5632_mono_mixer_controls.len()),
    SND_SOC_DAPM_MIXER!("Speaker Mix", ALC5632_PWR_MANAG_ADD2, 3, 0, &alc5632_speaker_mixer_controls[0], alc5632_speaker_mixer_controls.len()),
    SND_SOC_DAPM_MIXER!("DMICL Mix", SND_SOC_NOPM, 0, 0, &alc5632_dmicl_mixer_controls[0], alc5632_dmicl_mixer_controls.len()),
    SND_SOC_DAPM_MIXER!("DMICR Mix", SND_SOC_NOPM, 0, 0, &alc5632_dmicr_mixer_controls[0], alc5632_dmicr_mixer_controls.len()),
    SND_SOC_DAPM_MIXER!("Left Capture Mix", ALC5632_PWR_MANAG_ADD2, 1, 0, &alc5632_captureL_mixer_controls[0], alc5632_captureL_mixer_controls.len()),
    SND_SOC_DAPM_MIXER!("Right Capture Mix", ALC5632_PWR_MANAG_ADD2, 0, 0, &alc5632_captureR_mixer_controls[0], alc5632_captureR_mixer_controls.len()),
    SND_SOC_DAPM_AIF_IN!("AIFRXL", "Left HiFi Playback", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!("AIFRXR", "Right HiFi Playback", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("AIFTXL", "Left HiFi Capture", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("AIFTXR", "Right HiFi Capture", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!("VAIFRX", "Voice Playback", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("VAIFTX", "Voice Capture", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_DAC!("Voice DAC", core::ptr::null(), ALC5632_PWR_MANAG_ADD2, 10, 0),
    SND_SOC_DAPM_DAC!("Left DAC", core::ptr::null(), ALC5632_PWR_MANAG_ADD2, 9, 0),
    SND_SOC_DAPM_DAC!("Right DAC", core::ptr::null(), ALC5632_PWR_MANAG_ADD2, 8, 0),
    SND_SOC_DAPM_ADC!("Left ADC", core::ptr::null(), ALC5632_PWR_MANAG_ADD2, 7, 0),
    SND_SOC_DAPM_ADC!("Right ADC", core::ptr::null(), ALC5632_PWR_MANAG_ADD2, 6, 0),
    SND_SOC_DAPM_MIXER!("DAC Left Channel", ALC5632_PWR_MANAG_ADD1, 15, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("DAC Right Channel", ALC5632_PWR_MANAG_ADD1, 14, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("I2S Mix", ALC5632_PWR_MANAG_ADD1, 11, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("Phone Mix", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("Line Mix", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("Voice Mix", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("ADCLR", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Left Headphone", ALC5632_PWR_MANAG_ADD3, 11, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Right Headphone", ALC5632_PWR_MANAG_ADD3, 10, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Left Speaker", ALC5632_PWR_MANAG_ADD3, 13, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Right Speaker", ALC5632_PWR_MANAG_ADD3, 12, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Aux Out", ALC5632_PWR_MANAG_ADD3, 14, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Left LineIn", ALC5632_PWR_MANAG_ADD3, 7, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Right LineIn", ALC5632_PWR_MANAG_ADD3, 6, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Phone", ALC5632_PWR_MANAG_ADD3, 5, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Phone ADMix", ALC5632_PWR_MANAG_ADD3, 4, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("MIC1 PGA", ALC5632_PWR_MANAG_ADD3, 3, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("MIC2 PGA", ALC5632_PWR_MANAG_ADD3, 2, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("MIC1 Pre Amp", ALC5632_PWR_MANAG_ADD3, 1, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("MIC2 Pre Amp", ALC5632_PWR_MANAG_ADD3, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("MICBIAS1", ALC5632_PWR_MANAG_ADD1, 3, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("MICBIAS2", ALC5632_PWR_MANAG_ADD1, 2, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA_E!("D Amp", ALC5632_PWR_MANAG_ADD2, 14, 0, core::ptr::null(), 0, amp_mixer_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_PGA!("AB Amp", ALC5632_PWR_MANAG_ADD2, 15, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_MUX!("AB-D Amp Mux", ALC5632_PWR_MANAG_ADD1, 10, 0, &alc5632_amp_mux_controls),
    SND_SOC_DAPM_OUTPUT!("AUXOUT"),
    SND_SOC_DAPM_OUTPUT!("HPL"),
    SND_SOC_DAPM_OUTPUT!("HPR"),
    SND_SOC_DAPM_OUTPUT!("SPKOUT"),
    SND_SOC_DAPM_OUTPUT!("SPKOUTN"),
    SND_SOC_DAPM_INPUT!("LINEINL"),
    SND_SOC_DAPM_INPUT!("LINEINR"),
    SND_SOC_DAPM_INPUT!("PHONEP"),
    SND_SOC_DAPM_INPUT!("PHONEN"),
    SND_SOC_DAPM_INPUT!("DMICDAT"),
    SND_SOC_DAPM_INPUT!("MIC1"),
    SND_SOC_DAPM_INPUT!("MIC2"),
    SND_SOC_DAPM_VMID!("Vmid"),
];

macro_rules! route {
    ($sink:literal, NULL, $source:literal) => {
        snd_soc_dapm_route {
            sink: concat!($sink, "\0").as_ptr() as *const c_char,
            control: core::ptr::null(),
            source: concat!($source, "\0").as_ptr() as *const c_char,
        }
    };
    ($sink:literal, $control:literal, $source:literal) => {
        snd_soc_dapm_route {
            sink: concat!($sink, "\0").as_ptr() as *const c_char,
            control: concat!($control, "\0").as_ptr() as *const c_char,
            source: concat!($source, "\0").as_ptr() as *const c_char,
        }
    };
}

static alc5632_dapm_routes: &[snd_soc_dapm_route] = &[
    route!("Left DAC", NULL, "AIFRXL"),
    route!("Right DAC", NULL, "AIFRXR"),
    route!("I2S Mix", NULL, "Left DAC"),
    route!("I2S Mix", NULL, "Right DAC"),
    route!("Line Mix", NULL, "Right LineIn"),
    route!("Line Mix", NULL, "Left LineIn"),
    route!("Phone Mix", NULL, "Phone"),
    route!("Phone Mix", NULL, "Phone ADMix"),
    route!("AUXOUT", NULL, "Aux Out"),
    route!("DAC Right Channel", NULL, "I2S Mix"),
    route!("DAC Left Channel", NULL, "I2S Mix"),
    route!("HPL Mix", "ADC2HP_L Playback Switch", "Left Capture Mix"),
    route!("HPL Mix", NULL, "HP Mix"),
    route!("HPR Mix", "ADC2HP_R Playback Switch", "Right Capture Mix"),
    route!("HPR Mix", NULL, "HP Mix"),
    route!("HP Mix", "LI2HP Playback Switch", "Line Mix"),
    route!("HP Mix", "PHONE2HP Playback Switch", "Phone Mix"),
    route!("HP Mix", "MIC12HP Playback Switch", "MIC1 PGA"),
    route!("HP Mix", "MIC22HP Playback Switch", "MIC2 PGA"),
    route!("HP Mix", "VOICE2HP Playback Switch", "Voice Mix"),
    route!("HPR Mix", "DACR2HP Playback Switch", "DAC Right Channel"),
    route!("HPL Mix", "DACL2HP Playback Switch", "DAC Left Channel"),
    route!("HPOut Mix", NULL, "HP Mix"),
    route!("HPOut Mix", NULL, "HPR Mix"),
    route!("HPOut Mix", NULL, "HPL Mix"),
    route!("Speaker Mix", "LI2SPK Playback Switch", "Line Mix"),
    route!("Speaker Mix", "PHONE2SPK Playback Switch", "Phone Mix"),
    route!("Speaker Mix", "MIC12SPK Playback Switch", "MIC1 PGA"),
    route!("Speaker Mix", "MIC22SPK Playback Switch", "MIC2 PGA"),
    route!("Speaker Mix", "DAC2SPK Playback Switch", "DAC Left Channel"),
    route!("Speaker Mix", "VOICE2SPK Playback Switch", "Voice Mix"),
    route!("Mono Mix", "ADC2MONO_L Playback Switch", "Left Capture Mix"),
    route!("Mono Mix", "ADC2MONO_R Playback Switch", "Right Capture Mix"),
    route!("Mono Mix", "LI2MONO Playback Switch", "Line Mix"),
    route!("Mono Mix", "MIC12MONO Playback Switch", "MIC1 PGA"),
    route!("Mono Mix", "MIC22MONO Playback Switch", "MIC2 PGA"),
    route!("Mono Mix", "DAC2MONO Playback Switch", "DAC Left Channel"),
    route!("Mono Mix", "VOICE2MONO Playback Switch", "Voice Mix"),
    route!("Left Capture Mix", "LIL2REC Capture Switch", "LINEINL"),
    route!("Left Capture Mix", "PH2REC_L Capture Switch", "PHONEN"),
    route!("Left Capture Mix", "MIC12REC_L Capture Switch", "MIC1 Pre Amp"),
    route!("Left Capture Mix", "MIC22REC_L Capture Switch", "MIC2 Pre Amp"),
    route!("Left Capture Mix", "HPL2REC Capture Switch", "HPL Mix"),
    route!("Left Capture Mix", "SPK2REC_L Capture Switch", "Speaker Mix"),
    route!("Left Capture Mix", "MONO2REC_L Capture Switch", "Mono Mix"),
    route!("Right Capture Mix", "LIR2REC Capture Switch", "LINEINR"),
    route!("Right Capture Mix", "PH2REC_R Capture Switch", "PHONEP"),
    route!("Right Capture Mix", "MIC12REC_R Capture Switch", "MIC1 Pre Amp"),
    route!("Right Capture Mix", "MIC22REC_R Capture Switch", "MIC2 Pre Amp"),
    route!("Right Capture Mix", "HPR2REC Capture Switch", "HPR Mix"),
    route!("Right Capture Mix", "SPK2REC_R Capture Switch", "Speaker Mix"),
    route!("Right Capture Mix", "MONO2REC_R Capture Switch", "Mono Mix"),
    route!("Left Headphone Mux", "HP Left Mix", "HPL Mix"),
    route!("Left Headphone Mux", "Vmid", "Vmid"),
    route!("Right Headphone Mux", "HP Right Mix", "HPR Mix"),
    route!("Right Headphone Mux", "Vmid", "Vmid"),
    route!("SpeakerOut Mux", "Vmid", "Vmid"),
    route!("SpeakerOut Mux", "HPOut Mix", "HPOut Mix"),
    route!("SpeakerOut Mux", "Speaker Mix", "Speaker Mix"),
    route!("SpeakerOut Mux", "Mono Mix", "Mono Mix"),
    route!("AuxOut Mux", "Vmid", "Vmid"),
    route!("AuxOut Mux", "HPOut Mix", "HPOut Mix"),
    route!("AuxOut Mux", "Speaker Mix", "Speaker Mix"),
    route!("AuxOut Mux", "Mono Mix", "Mono Mix"),
    route!("HPL", NULL, "Left Headphone"),
    route!("Left Headphone", NULL, "Left Headphone Mux"),
    route!("HPR", NULL, "Right Headphone"),
    route!("Right Headphone", NULL, "Right Headphone Mux"),
    route!("Aux Out", NULL, "AuxOut Mux"),
    route!("Left LineIn", NULL, "LINEINL"),
    route!("Right LineIn", NULL, "LINEINR"),
    route!("Phone", NULL, "PHONEP"),
    route!("MIC1 Pre Amp", NULL, "MIC1"),
    route!("MIC2 Pre Amp", NULL, "MIC2"),
    route!("MIC1 PGA", NULL, "MIC1 Pre Amp"),
    route!("MIC2 PGA", NULL, "MIC2 Pre Amp"),
    route!("Left ADC", NULL, "Left Capture Mix"),
    route!("DMICL Mix", "DMICL2ADC Capture Switch", "DMICDAT"),
    route!("Left ADC", NULL, "DMICL Mix"),
    route!("ADCLR", NULL, "Left ADC"),
    route!("Right ADC", NULL, "Right Capture Mix"),
    route!("DMICR Mix", "DMICR2ADC Capture Switch", "DMICDAT"),
    route!("Right ADC", NULL, "DMICR Mix"),
    route!("ADCR Mux", "Stereo ADC", "Right ADC"),
    route!("ADCR Mux", "Voice ADC", "Right ADC"),
    route!("ADCLR", NULL, "ADCR Mux"),
    route!("VAIFTX", NULL, "ADCR Mux"),
    route!("I2SOut Mux", "ADC LR", "ADCLR"),
    route!("I2SOut Mux", "Voice Stereo Digital", "VAIFRX"),
    route!("AIFTXL", NULL, "I2SOut Mux"),
    route!("AIFTXR", NULL, "I2SOut Mux"),
    route!("Voice DAC", NULL, "VAIFRX"),
    route!("Voice Mix", NULL, "Voice DAC"),
    route!("SpeakerOut N Mux", "RN/-R", "Left Speaker"),
    route!("SpeakerOut N Mux", "RP/+R", "Left Speaker"),
    route!("SpeakerOut N Mux", "LN/-R", "Left Speaker"),
    route!("SpeakerOut N Mux", "Mute", "Vmid"),
    route!("SpeakerOut N Mux", "RN/-R", "Right Speaker"),
    route!("SpeakerOut N Mux", "RP/+R", "Right Speaker"),
    route!("SpeakerOut N Mux", "LN/-R", "Right Speaker"),
    route!("SpeakerOut N Mux", "Mute", "Vmid"),
    route!("AB Amp", NULL, "SpeakerOut Mux"),
    route!("D Amp", NULL, "SpeakerOut Mux"),
    route!("AB-D Amp Mux", "AB Amp", "AB Amp"),
    route!("AB-D Amp Mux", "D Amp", "D Amp"),
    route!("Left Speaker", NULL, "AB-D Amp Mux"),
    route!("Right Speaker", NULL, "AB-D Amp Mux"),
    route!("SPKOUT", NULL, "Left Speaker"),
    route!("SPKOUT", NULL, "Right Speaker"),
    route!("SPKOUTN", NULL, "SpeakerOut N Mux"),
];

/* PLL divisors */
#[repr(C)]
struct _pll_div {
    pll_in: u32,
    pll_out: u32,
    regvalue: u16,
}

/* Note : pll code from original alc5632 driver. Not sure of how good it is */
/* useful only for master mode */
static codec_master_pll_div: [_pll_div; 30] = [
    _pll_div { pll_in: 2048000, pll_out: 8192000, regvalue: 0x0ea0 },
    _pll_div { pll_in: 3686400, pll_out: 8192000, regvalue: 0x4e27 },
    _pll_div { pll_in: 12000000, pll_out: 8192000, regvalue: 0x456b },
    _pll_div { pll_in: 13000000, pll_out: 8192000, regvalue: 0x495f },
    _pll_div { pll_in: 13100000, pll_out: 8192000, regvalue: 0x0320 },
    _pll_div { pll_in: 2048000, pll_out: 11289600, regvalue: 0xf637 },
    _pll_div { pll_in: 3686400, pll_out: 11289600, regvalue: 0x2f22 },
    _pll_div { pll_in: 12000000, pll_out: 11289600, regvalue: 0x3e2f },
    _pll_div { pll_in: 13000000, pll_out: 11289600, regvalue: 0x4d5b },
    _pll_div { pll_in: 13100000, pll_out: 11289600, regvalue: 0x363b },
    _pll_div { pll_in: 2048000, pll_out: 16384000, regvalue: 0x1ea0 },
    _pll_div { pll_in: 3686400, pll_out: 16384000, regvalue: 0x9e27 },
    _pll_div { pll_in: 12000000, pll_out: 16384000, regvalue: 0x452b },
    _pll_div { pll_in: 13000000, pll_out: 16384000, regvalue: 0x542f },
    _pll_div { pll_in: 13100000, pll_out: 16384000, regvalue: 0x03a0 },
    _pll_div { pll_in: 2048000, pll_out: 16934400, regvalue: 0xe625 },
    _pll_div { pll_in: 3686400, pll_out: 16934400, regvalue: 0x9126 },
    _pll_div { pll_in: 12000000, pll_out: 16934400, regvalue: 0x4d2c },
    _pll_div { pll_in: 13000000, pll_out: 16934400, regvalue: 0x742f },
    _pll_div { pll_in: 13100000, pll_out: 16934400, regvalue: 0x3c27 },
    _pll_div { pll_in: 2048000, pll_out: 22579200, regvalue: 0x2aa0 },
    _pll_div { pll_in: 3686400, pll_out: 22579200, regvalue: 0x2f20 },
    _pll_div { pll_in: 12000000, pll_out: 22579200, regvalue: 0x7e2f },
    _pll_div { pll_in: 13000000, pll_out: 22579200, regvalue: 0x742f },
    _pll_div { pll_in: 13100000, pll_out: 22579200, regvalue: 0x3c27 },
    _pll_div { pll_in: 2048000, pll_out: 24576000, regvalue: 0x2ea0 },
    _pll_div { pll_in: 3686400, pll_out: 24576000, regvalue: 0xee27 },
    _pll_div { pll_in: 12000000, pll_out: 24576000, regvalue: 0x2915 },
    _pll_div { pll_in: 13000000, pll_out: 24576000, regvalue: 0x772e },
    _pll_div { pll_in: 13100000, pll_out: 24576000, regvalue: 0x0d20 },
];

/* FOUT = MCLK*(N+2)/((M+2)*(K+2))
   N: bit 15:8 (div 2 .. div 257)
   K: bit  6:4 typical 2
   M: bit  3:0 (div 2 .. div 17)

   same as for 5623 - thanks!
*/

static codec_slave_pll_div: [_pll_div; 6] = [
    _pll_div { pll_in: 1024000, pll_out: 16384000, regvalue: 0x3ea0 },
    _pll_div { pll_in: 1411200, pll_out: 22579200, regvalue: 0x3ea0 },
    _pll_div { pll_in: 1536000, pll_out: 24576000, regvalue: 0x3ea0 },
    _pll_div { pll_in: 2048000, pll_out: 16384000, regvalue: 0x1ea0 },
    _pll_div { pll_in: 2822400, pll_out: 22579200, regvalue: 0x1ea0 },
    _pll_div { pll_in: 3072000, pll_out: 24576000, regvalue: 0x1ea0 },
];

unsafe extern "C" fn alc5632_set_dai_pll(
    codec_dai: *mut snd_soc_dai,
    pll_id: c_int,
    _source: c_int,
    freq_in: c_uint,
    freq_out: c_uint,
) -> c_int {
    let component = unsafe { (*codec_dai).component };
    let mut gbl_clk: c_int = 0;
    let mut pll_div: c_int = 0;
    let reg: u16;

    if pll_id < ALC5632_PLL_FR_MCLK || pll_id > ALC5632_PLL_FR_VBCLK {
        return -EINVAL;
    }

    /* Disable PLL power */
    unsafe {
        snd_soc_component_update_bits(component, ALC5632_PWR_MANAG_ADD2, ALC5632_PWR_ADD2_PLL1, 0);
        snd_soc_component_update_bits(component, ALC5632_PWR_MANAG_ADD2, ALC5632_PWR_ADD2_PLL2, 0);
    }

    /* pll is not used in slave mode */
    reg = unsafe { snd_soc_component_read(component, ALC5632_DAI_CONTROL) as u16 };
    if (reg as c_uint & ALC5632_DAI_SDP_SLAVE_MODE) != 0 {
        return 0;
    }

    if freq_in == 0 || freq_out == 0 {
        return 0;
    }

    match pll_id {
        ALC5632_PLL_FR_MCLK => {
            for div in codec_master_pll_div.iter() {
                if div.pll_in == freq_in && div.pll_out == freq_out {
                    /* PLL source from MCLK */
                    pll_div = div.regvalue as c_int;
                    break;
                }
            }
        }
        ALC5632_PLL_FR_BCLK => {
            for div in codec_slave_pll_div.iter() {
                if div.pll_in == freq_in && div.pll_out == freq_out {
                    /* PLL source from Bitclk */
                    gbl_clk = ALC5632_PLL_FR_BCLK;
                    pll_div = div.regvalue as c_int;
                    break;
                }
            }
        }
        ALC5632_PLL_FR_VBCLK => {
            for div in codec_slave_pll_div.iter() {
                if div.pll_in == freq_in && div.pll_out == freq_out {
                    /* PLL source from voice clock */
                    gbl_clk = ALC5632_PLL_FR_VBCLK;
                    pll_div = div.regvalue as c_int;
                    break;
                }
            }
        }
        _ => return -EINVAL,
    }

    if pll_div == 0 {
        return -EINVAL;
    }

    unsafe {
        /* choose MCLK/BCLK/VBCLK */
        snd_soc_component_write(component, ALC5632_GPCR2, gbl_clk as c_uint);
        /* choose PLL1 clock rate */
        snd_soc_component_write(component, ALC5632_PLL1_CTRL, pll_div as c_uint);
        /* enable PLL1 */
        snd_soc_component_update_bits(
            component,
            ALC5632_PWR_MANAG_ADD2,
            ALC5632_PWR_ADD2_PLL1,
            ALC5632_PWR_ADD2_PLL1,
        );
        /* enable PLL2 */
        snd_soc_component_update_bits(
            component,
            ALC5632_PWR_MANAG_ADD2,
            ALC5632_PWR_ADD2_PLL2,
            ALC5632_PWR_ADD2_PLL2,
        );
        /* use PLL1 as main SYSCLK */
        snd_soc_component_update_bits(
            component,
            ALC5632_GPCR1,
            ALC5632_GPCR1_CLK_SYS_SRC_SEL_PLL1,
            ALC5632_GPCR1_CLK_SYS_SRC_SEL_PLL1,
        );
    }

    0
}

#[repr(C)]
struct _coeff_div {
    fs: u16,
    regvalue: u16,
}

/* codec hifi mclk (after PLL) clock divider coefficients */
/* values inspired from column BCLK=32Fs of Appendix A table */
static coeff_div: [_coeff_div; 1] = [_coeff_div { fs: 512 * 1, regvalue: 0x3075 }];

unsafe fn get_coeff(component: *mut snd_soc_component, rate: c_int) -> c_int {
    let alc5632 = unsafe { snd_soc_component_get_drvdata(component) as *mut alc5632_priv };

    for (i, coeff) in coeff_div.iter().enumerate() {
        if (coeff.fs as c_int).wrapping_mul(rate) as c_uint == unsafe { (*alc5632).sysclk } {
            return i as c_int;
        }
    }
    -EINVAL
}

/*
 * Clock after PLL and dividers
 */
unsafe extern "C" fn alc5632_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = unsafe { (*codec_dai).component };
    let alc5632 = unsafe { snd_soc_component_get_drvdata(component) as *mut alc5632_priv };

    match freq {
        4096000 | 8192000 | 11289600 | 12288000 | 16384000 | 16934400 | 18432000
        | 22579200 | 24576000 => {
            unsafe {
                (*alc5632).sysclk = freq;
            }
            0
        }
        _ => -EINVAL,
    }
}

unsafe extern "C" fn alc5632_set_dai_fmt(
    codec_dai: *mut snd_soc_dai,
    fmt: c_uint,
) -> c_int {
    let component = unsafe { (*codec_dai).component };
    let mut iface: u16 = 0;

    /* set audio interface clocking */
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => iface = ALC5632_DAI_SDP_MASTER_MODE as u16,
        SND_SOC_DAIFMT_CBC_CFC => iface = ALC5632_DAI_SDP_SLAVE_MODE as u16,
        _ => return -EINVAL,
    }

    /* interface format */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => iface |= ALC5632_DAI_I2S_DF_I2S as u16,
        SND_SOC_DAIFMT_LEFT_J => iface |= ALC5632_DAI_I2S_DF_LEFT as u16,
        SND_SOC_DAIFMT_DSP_A => iface |= ALC5632_DAI_I2S_DF_PCM_A as u16,
        SND_SOC_DAIFMT_DSP_B => iface |= ALC5632_DAI_I2S_DF_PCM_B as u16,
        _ => return -EINVAL,
    }

    /* clock inversion */
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_IF => iface |= ALC5632_DAI_MAIN_I2S_BCLK_POL_CTRL as u16,
        SND_SOC_DAIFMT_IB_NF => iface |= ALC5632_DAI_MAIN_I2S_BCLK_POL_CTRL as u16,
        SND_SOC_DAIFMT_NB_IF => {}
        _ => return -EINVAL,
    }

    unsafe { snd_soc_component_write(component, ALC5632_DAI_CONTROL, iface as c_uint) }
}

unsafe extern "C" fn alc5632_pcm_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = unsafe { (*dai).component };
    let coeff: c_int;
    let rate: c_int;
    let mut iface: u16;

    iface = unsafe { snd_soc_component_read(component, ALC5632_DAI_CONTROL) as u16 };
    iface &= !(ALC5632_DAI_I2S_DL_MASK as u16);

    /* bit size */
    match unsafe { params_width(params) } {
        16 => iface |= ALC5632_DAI_I2S_DL_16 as u16,
        20 => iface |= ALC5632_DAI_I2S_DL_20 as u16,
        24 => iface |= ALC5632_DAI_I2S_DL_24 as u16,
        _ => return -EINVAL,
    }

    /* set iface & srate */
    unsafe {
        snd_soc_component_write(component, ALC5632_DAI_CONTROL, iface as c_uint);
    }
    rate = unsafe { params_rate(params) };
    coeff = unsafe { get_coeff(component, rate) };
    if coeff < 0 {
        return -EINVAL;
    }

    let coeff_value = coeff_div[coeff as usize].regvalue;
    unsafe {
        snd_soc_component_write(component, ALC5632_DAC_CLK_CTRL1, coeff_value as c_uint);
    }

    0
}

unsafe extern "C" fn alc5632_mute(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    let component = unsafe { (*dai).component };
    let hp_mute: u16 = (ALC5632_MISC_HP_DEPOP_MUTE_L | ALC5632_MISC_HP_DEPOP_MUTE_R) as u16;
    let mut mute_reg: u16 =
        (unsafe { snd_soc_component_read(component, ALC5632_MISC_CTRL) } as u16) & !hp_mute;

    if mute != 0 {
        mute_reg |= hp_mute;
    }

    unsafe { snd_soc_component_write(component, ALC5632_MISC_CTRL, mute_reg as c_uint) }
}

const ALC5632_ADD2_POWER_EN: c_uint = ALC5632_PWR_ADD2_VREF;
const ALC5632_ADD3_POWER_EN: c_uint = ALC5632_PWR_ADD3_MIC1_BOOST_AD;
const ALC5632_ADD1_POWER_EN: c_uint = ALC5632_PWR_ADD1_DAC_REF
    | ALC5632_PWR_ADD1_SOFTGEN_EN
    | ALC5632_PWR_ADD1_HP_OUT_AMP
    | ALC5632_PWR_ADD1_HP_OUT_ENH_AMP
    | ALC5632_PWR_ADD1_MAIN_BIAS;

unsafe fn enable_power_depop(component: *mut snd_soc_component) {
    unsafe {
        snd_soc_component_update_bits(
            component,
            ALC5632_PWR_MANAG_ADD1,
            ALC5632_PWR_ADD1_SOFTGEN_EN,
            ALC5632_PWR_ADD1_SOFTGEN_EN,
        );
        snd_soc_component_update_bits(
            component,
            ALC5632_PWR_MANAG_ADD3,
            ALC5632_ADD3_POWER_EN,
            ALC5632_ADD3_POWER_EN,
        );
        snd_soc_component_update_bits(
            component,
            ALC5632_MISC_CTRL,
            ALC5632_MISC_HP_DEPOP_MODE2_EN,
            ALC5632_MISC_HP_DEPOP_MODE2_EN,
        );

        /* "normal" mode: 0 @ 26 */
        /* set all PR0-7 mixers to 0 */
        snd_soc_component_update_bits(
            component,
            ALC5632_PWR_DOWN_CTRL_STATUS,
            ALC5632_PWR_DOWN_CTRL_STATUS_MASK,
            0,
        );

        msleep(500);

        snd_soc_component_update_bits(
            component,
            ALC5632_PWR_MANAG_ADD2,
            ALC5632_ADD2_POWER_EN,
            ALC5632_ADD2_POWER_EN,
        );
        snd_soc_component_update_bits(
            component,
            ALC5632_PWR_MANAG_ADD1,
            ALC5632_ADD1_POWER_EN,
            ALC5632_ADD1_POWER_EN,
        );

        /* disable HP Depop2 */
        snd_soc_component_update_bits(component, ALC5632_MISC_CTRL, ALC5632_MISC_HP_DEPOP_MODE2_EN, 0);
    }
}

unsafe extern "C" fn alc5632_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => unsafe {
            enable_power_depop(component);
        },
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {}
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => unsafe {
            /* everything off except vref/vmid, */
            snd_soc_component_update_bits(
                component,
                ALC5632_PWR_MANAG_ADD1,
                ALC5632_PWR_MANAG_ADD1_MASK,
                ALC5632_PWR_ADD1_MAIN_BIAS,
            );
            snd_soc_component_update_bits(
                component,
                ALC5632_PWR_MANAG_ADD2,
                ALC5632_PWR_MANAG_ADD2_MASK,
                ALC5632_PWR_ADD2_VREF,
            );
            /* "normal" mode: 0 @ 26 */
            snd_soc_component_update_bits(
                component,
                ALC5632_PWR_DOWN_CTRL_STATUS,
                ALC5632_PWR_DOWN_CTRL_STATUS_MASK,
                0xffff ^ (ALC5632_PWR_VREF_PR3 | ALC5632_PWR_VREF_PR2),
            );
        },
        snd_soc_bias_level::SND_SOC_BIAS_OFF => unsafe {
            /* everything off, dac mute, inactive */
            snd_soc_component_update_bits(component, ALC5632_PWR_MANAG_ADD2, ALC5632_PWR_MANAG_ADD2_MASK, 0);
            snd_soc_component_update_bits(component, ALC5632_PWR_MANAG_ADD3, ALC5632_PWR_MANAG_ADD3_MASK, 0);
            snd_soc_component_update_bits(component, ALC5632_PWR_MANAG_ADD1, ALC5632_PWR_MANAG_ADD1_MASK, 0);
        },
    }
    0
}

const ALC5632_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static alc5632_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(alc5632_pcm_hw_params),
    mute_stream: Some(alc5632_mute),
    set_fmt: Some(alc5632_set_dai_fmt),
    set_sysclk: Some(alc5632_set_dai_sysclk),
    set_pll: Some(alc5632_set_dai_pll),
    no_capture_mute: 1,
};

static mut alc5632_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"alc5632-hifi\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"HiFi Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rate_min: 8000,
        rate_max: 48000,
        rates: SNDRV_PCM_RATE_8000_48000,
        formats: ALC5632_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"HiFi Capture\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rate_min: 8000,
        rate_max: 48000,
        rates: SNDRV_PCM_RATE_8000_48000,
        formats: ALC5632_FORMATS,
    },
    ops: &alc5632_dai_ops,
    symmetric_rate: 1,
};

// CONFIG_PM conditional: C provides alc5632_resume only when CONFIG_PM is set,
// otherwise the resume callback is NULL.
#[cfg(CONFIG_PM)]
unsafe extern "C" fn alc5632_resume(component: *mut snd_soc_component) -> c_int {
    let alc5632 = unsafe { snd_soc_component_get_drvdata(component) as *mut alc5632_priv };

    unsafe {
        regcache_sync((*alc5632).regmap);
    }

    0
}

unsafe extern "C" fn alc5632_probe(component: *mut snd_soc_component) -> c_int {
    let alc5632 = unsafe { snd_soc_component_get_drvdata(component) as *mut alc5632_priv };

    match unsafe { (*alc5632).id as c_int } {
        0x5c => unsafe {
            snd_soc_add_component_controls(
                component,
                alc5632_vol_snd_controls.as_ptr(),
                alc5632_vol_snd_controls.len() as c_uint,
            );
        },
        _ => return -EINVAL,
    }

    0
}

static soc_component_device_alc5632: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(alc5632_probe),
    #[cfg(CONFIG_PM)]
    resume: Some(alc5632_resume),
    #[cfg(not(CONFIG_PM))]
    resume: None,
    set_bias_level: Some(alc5632_set_bias_level),
    controls: alc5632_snd_controls.as_ptr(),
    num_controls: alc5632_snd_controls.len() as c_uint,
    dapm_widgets: alc5632_dapm_widgets.as_ptr(),
    num_dapm_widgets: alc5632_dapm_widgets.len() as c_uint,
    dapm_routes: alc5632_dapm_routes.as_ptr(),
    num_dapm_routes: alc5632_dapm_routes.len() as c_uint,
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static alc5632_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 16,
    max_register: ALC5632_MAX_REGISTER,
    reg_defaults: alc5632_reg_defaults.as_ptr(),
    num_reg_defaults: alc5632_reg_defaults.len() as c_uint,
    volatile_reg: Some(alc5632_volatile_register),
    cache_type: REGCACHE_RBTREE,
};

static alc5632_i2c_table: [i2c_device_id; 2] = [
    i2c_device_id {
        name: b"alc5632\0".as_ptr() as *const c_char,
        driver_data: 0x5c,
    },
    i2c_device_id {
        name: core::ptr::null(),
        driver_data: 0,
    },
];
MODULE_DEVICE_TABLE!(i2c, alc5632_i2c_table);

/*
 * alc5632 2 wire address is determined by A1 pin
 * state during powerup.
 *    low  = 0x1a
 *    high = 0x1b
 */
unsafe extern "C" fn alc5632_i2c_probe(client: *mut i2c_client) -> c_int {
    let alc5632: *mut alc5632_priv;
    let mut ret: c_int;
    let ret1: c_int;
    let ret2: c_int;
    let mut vid1: c_uint = 0;
    let mut vid2: c_uint = 0;
    let matched_id: c_uint;

    alc5632 = unsafe {
        devm_kzalloc(
            &mut (*client).dev,
            core::mem::size_of::<alc5632_priv>(),
            GFP_KERNEL,
        ) as *mut alc5632_priv
    };
    if alc5632.is_null() {
        return -ENOMEM;
    }

    unsafe {
        i2c_set_clientdata(client, alc5632 as *mut c_void);
    }

    unsafe {
        (*alc5632).regmap = devm_regmap_init_i2c(client, &alc5632_regmap);
    }
    if unsafe { IS_ERR((*alc5632).regmap as *const c_void) } {
        ret = unsafe { PTR_ERR((*alc5632).regmap as *const c_void) };
        unsafe {
            dev_err(
                &mut (*client).dev,
                b"regmap_init() failed: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
        }
        return ret;
    }

    ret1 = unsafe { regmap_read((*alc5632).regmap, ALC5632_VENDOR_ID1, &mut vid1) };
    ret2 = unsafe { regmap_read((*alc5632).regmap, ALC5632_VENDOR_ID2, &mut vid2) };
    if ret1 != 0 || ret2 != 0 {
        unsafe {
            dev_err(
                &mut (*client).dev,
                b"Failed to read chip ID: ret1=%d, ret2=%d\n\0".as_ptr() as *const c_char,
                ret1,
                ret2,
            );
        }
        return -EIO;
    }

    vid2 >>= 8;

    matched_id = unsafe { i2c_get_match_data(client) as usize as c_uint };

    if vid1 != 0x10EC || vid2 != matched_id {
        unsafe {
            dev_err(
                &mut (*client).dev,
                b"Device is not a ALC5632: VID1=0x%x, VID2=0x%x\n\0".as_ptr() as *const c_char,
                vid1,
                vid2,
            );
        }
        return -EINVAL;
    }

    ret = unsafe { alc5632_reset((*alc5632).regmap) };
    if ret < 0 {
        unsafe {
            dev_err(
                &mut (*client).dev,
                b"Failed to issue reset\n\0".as_ptr() as *const c_char,
            );
        }
        return ret;
    }

    unsafe {
        (*alc5632).id = vid2 as u8;
    }
    match unsafe { (*alc5632).id as c_int } {
        0x5c => unsafe {
            alc5632_dai.name = b"alc5632-hifi\0".as_ptr() as *const c_char;
        },
        _ => return -EINVAL,
    }

    ret = unsafe {
        devm_snd_soc_register_component(
            &mut (*client).dev,
            &soc_component_device_alc5632,
            &mut alc5632_dai,
            1,
        )
    };

    if ret < 0 {
        unsafe {
            dev_err(
                &mut (*client).dev,
                b"Failed to register component: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
        }
        return ret;
    }

    ret
}

// CONFIG_OF conditional: translated only when CONFIG_OF is enabled in the build.
#[cfg(CONFIG_OF)]
static alc5632_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"realtek,alc5632\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
#[cfg(CONFIG_OF)]
MODULE_DEVICE_TABLE!(of, alc5632_of_match);

/* i2c codec control layer */
static alc5632_i2c_driver: i2c_driver = i2c_driver! {
    driver: {
        name: "alc5632",
        of_match_table: of_match_ptr!(alc5632_of_match),
    },
    probe: alc5632_i2c_probe,
    id_table: alc5632_i2c_table,
};

module_i2c_driver!(alc5632_i2c_driver);

MODULE_DESCRIPTION!("ASoC ALC5632 driver");
MODULE_AUTHOR!("Leon Romanovsky <leon@leon.nu>");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
