// SPDX-License-Identifier: GPL-2.0
/*
 * ak4619.c -- Asahi Kasei ALSA SoC Audio driver
 *
 * Copyright (C) 2023 Renesas Electronics Corporation
 * Khanh Le <khanh.le.xr@renesas.com>
 *
 * Based on ak4613.c by Kuninori Morimoto
 * Based on da7213.c by Adam Thomson
 * Based on ak4641.c by Harald Welte
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const fn BIT(n: u32) -> u32 {
    1u32 << n
}

/*
 * Registers
 */

const PWR_MGMT: u32 = 0x00; /* Power Management */
const AU_IFF1: u32 = 0x01; /* Audio I/F Format */
const AU_IFF2: u32 = 0x02; /* Audio I/F Format (Extended) */
const SYS_CLK: u32 = 0x03; /* System Clock Setting */
const MIC_AMP1: u32 = 0x04; /* MIC AMP Gain 1 */
const MIC_AMP2: u32 = 0x05; /* MIC AMP Gain 2 */
const LADC1: u32 = 0x06; /* ADC1 Lch Digital Volume */
const RADC1: u32 = 0x07; /* ADC1 Rch Digital Volume */
const LADC2: u32 = 0x08; /* ADC2 Lch Digital Volume */
const RADC2: u32 = 0x09; /* ADC2 Rch Digital Volume */
const ADC_DF: u32 = 0x0a; /* ADC Digital Filter Setting */
const ADC_AI: u32 = 0x0b; /* ADC Analog Input Setting */
const ADC_MHPF: u32 = 0x0D; /* ADC Mute & HPF Control */
const LDAC1: u32 = 0x0E; /* DAC1 Lch Digital Volume */
const RDAC1: u32 = 0x0F; /* DAC1 Rch Digital Volume */
const LDAC2: u32 = 0x10; /* DAC2 Lch Digital Volume */
const RDAC2: u32 = 0x11; /* DAC2 Rch Digital Volume */
const DAC_IS: u32 = 0x12; /* DAC Input Select Setting */
const DAC_DEMP: u32 = 0x13; /* DAC De-Emphasis Setting */
const DAC_MF: u32 = 0x14; /* DAC Mute & Filter Setting */

/*
 * Bit fields
 */

/* Power Management */
const PMAD2: u8 = BIT(5) as u8;
const PMAD1: u8 = BIT(4) as u8;
const PMDA2: u8 = BIT(2) as u8;
const PMDA1: u8 = BIT(1) as u8;
const RSTN: u8 = BIT(0) as u8;

/* Audio_I/F Format */
const DCF_STEREO_I2S: u8 = (0x0 << 4) as u8;
const DCF_STEREO_MSB: u8 = (0x5 << 4) as u8;
const DCF_PCM_SF: u8 = (0x6 << 4) as u8;
const DCF_PCM_LF: u8 = (0x7 << 4) as u8;
const DSL_32: u8 = (0x3 << 2) as u8;
const DCF_MASK: u32 = 0x7 << 4;
const DSL_MASK: u32 = 0x3 << 2;
const BCKP: u8 = BIT(1) as u8;

/* Audio_I/F Format (Extended) */
const DIDL_24: u8 = (0x0 << 2) as u8;
const DIDL_20: u8 = (0x1 << 2) as u8;
const DIDL_16: u8 = (0x2 << 2) as u8;
const DIDL_32: u8 = (0x3 << 2) as u8;
const DODL_24: u8 = (0x0 << 0) as u8;
const DODL_20: u8 = (0x1 << 0) as u8;
const DODL_16: u8 = (0x2 << 0) as u8;
const DIDL_MASK: u32 = 0x3 << 2;
const DODL_MASK: u32 = 0x3 << 0;
const SLOT: u8 = BIT(4) as u8;

/* System Clock Setting */
const FS_MASK: u32 = 0x7;

/* MIC AMP Gain */
const MGNL_SHIFT: u32 = 4;
const MGNR_SHIFT: u32 = 0;
const MGN_MAX: u32 = 0xB;

/* ADC Digital Volume */
const VOLAD_SHIFT: u32 = 0;
const VOLAD_MAX: u32 = 0xFF;

/* ADC Digital Filter Setting */
const AD1SL_SHIFT: u32 = 0;
const AD2SL_SHIFT: u32 = 4;

/* Analog Input Select */
const AD1LSEL_SHIFT: u32 = 6;
const AD1RSEL_SHIFT: u32 = 4;
const AD2LSEL_SHIFT: u32 = 2;
const AD2RSEL_SHIFT: u32 = 0;

/* ADC Mute & HPF Control */
const ATSPAD_SHIFT: u32 = 7;
const AD1MUTE_SHIFT: u32 = 5;
const AD2MUTE_SHIFT: u32 = 6;
const AD1MUTE_MAX: u32 = 1;
const AD2MUTE_MAX: u32 = 1;
const AD1MUTE_EN: u32 = BIT(5);
const AD2MUTE_EN: u32 = BIT(6);
const AD1HPFN_SHIFT: u32 = 1;
const AD1HPFN_MAX: u32 = 1;
const AD2HPFN_SHIFT: u32 = 2;
const AD2HPFN_MAX: u32 = 1;

/* DAC Digital Volume */
const VOLDA_SHIFT: u32 = 0;
const VOLDA_MAX: u32 = 0xFF;

/* DAC Input Select Setting */
const DAC1SEL_SHIFT: u32 = 0;
const DAC2SEL_SHIFT: u32 = 2;

/* DAC De-Emphasis Setting */
const DEM1_32000: u8 = (0x3 << 0) as u8;
const DEM1_44100: u8 = (0x0 << 0) as u8;
const DEM1_48000: u8 = (0x2 << 0) as u8;
const DEM1_OFF: u8 = (0x1 << 0) as u8;
const DEM2_32000: u8 = (0x3 << 2) as u8;
const DEM2_44100: u8 = (0x0 << 2) as u8;
const DEM2_48000: u8 = (0x2 << 2) as u8;
const DEM2_OFF: u8 = (0x1 << 2) as u8;
const DEM1_MASK: u32 = 0x3 << 0;
const DEM2_MASK: u32 = 0x3 << 2;
const DEM1_SHIFT: u32 = 0;
const DEM2_SHIFT: u32 = 2;

/* DAC Mute & Filter Setting */
const DA1MUTE_SHIFT: u32 = 4;
const DA1MUTE_MAX: u32 = 1;
const DA2MUTE_SHIFT: u32 = 5;
const DA2MUTE_MAX: u32 = 1;
const DA1MUTE_EN: u32 = BIT(4);
const DA2MUTE_EN: u32 = BIT(5);
const ATSPDA_SHIFT: u32 = 7;
const DA1SL_SHIFT: u32 = 0;
const DA2SL_SHIFT: u32 = 2;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: gfp_t = 0;
const SND_SOC_NOPM: c_int = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const REGCACHE_MAPLE: c_uint = 0;
const SND_SOC_BIAS_ON: snd_soc_bias_level = 0;
const SND_SOC_BIAS_PREPARE: snd_soc_bias_level = 1;
const SND_SOC_BIAS_STANDBY: snd_soc_bias_level = 2;
const SND_SOC_BIAS_OFF: snd_soc_bias_level = 3;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0;
const SND_SOC_DAIFMT_NB_IF: c_uint = 0;
const SND_SOC_DAIFMT_IB_IF: c_uint = 0;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 0;
const SND_SOC_DAIFMT_DSP_A: c_uint = 0;
const SND_SOC_DAIFMT_DSP_B: c_uint = 0;
const SND_SOC_DAIFMT_MASTER_MASK: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const SND_SOC_POSSIBLE_DAIFMT_I2S: u64 = 0;
const SND_SOC_POSSIBLE_DAIFMT_LEFT_J: u64 = 0;
const SND_SOC_POSSIBLE_DAIFMT_DSP_A: u64 = 0;
const SND_SOC_POSSIBLE_DAIFMT_DSP_B: u64 = 0;
const SNDRV_PCM_RATE_8000_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S20_LE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S24_LE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S32_LE: c_uint = 0;

type gfp_t = c_uint;
type kernel_ulong_t = c_ulong;
type snd_soc_bias_level = c_uint;

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: *const c_char,
    pub driver_data: kernel_ulong_t,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub list: *const c_uint,
    pub mask: c_uint,
    pub count: c_uint,
}

#[repr(C)]
pub struct ak4619_priv {
    pub regmap: *mut regmap,
    pub constraint: snd_pcm_hw_constraint_list,
    pub deemph_en: c_int,
    pub playback_rate: c_uint,
    pub sysclk: c_uint,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}

type c_long = core::ffi::c_long;

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub auto_selectable_formats: *mut u64,
    pub num_auto_selectable_formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub idle_bias_on: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub cache_type: c_uint,
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
    pub symmetric_rate: c_uint,
}

unsafe extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_pcm_hw_constraint_list(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_uint,
        constraint: *mut snd_pcm_hw_constraint_list,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: gfp_t) -> *mut c_void;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

/*
 * DAC Volume
 *
 * max : 0x00 : +12.0 dB
 *	( 0.5 dB step )
 * min : 0xFE : -115.0 dB
 * mute: 0xFF
 */
// static const DECLARE_TLV_DB_SCALE(dac_tlv, -11550, 50, 1);

/*
 * MIC Volume
 *
 * max : 0x0B : +27.0 dB
 *	( 3 dB step )
 * min: 0x00 : -6.0 dB
 */
// static const DECLARE_TLV_DB_SCALE(mic_tlv, -600, 300, 0);

/*
 * ADC Volume
 *
 * max : 0x00 : +24.0 dB
 *	( 0.5 dB step )
 * min : 0xFE : -103.0 dB
 * mute: 0xFF
 */
// static const DECLARE_TLV_DB_SCALE(adc_tlv, -10350, 50, 1);

/* ADC & DAC Volume Level Transition Time select */
static ak4619_vol_trans_txt: [*const c_char; 2] = [c"4/fs".as_ptr(), c"16/fs".as_ptr()];

// static SOC_ENUM_SINGLE_DECL(ak4619_adc_vol_trans, ADC_MHPF, ATSPAD_SHIFT, ak4619_vol_trans_txt);
// static SOC_ENUM_SINGLE_DECL(ak4619_dac_vol_trans, DAC_MF,   ATSPDA_SHIFT, ak4619_vol_trans_txt);

/* ADC Digital Filter select */
static ak4619_adc_digi_fil_txt: [*const c_char; 5] = [
    c"Sharp Roll-Off Filter".as_ptr(),
    c"Slow Roll-Off Filter".as_ptr(),
    c"Short Delay Sharp Roll-Off Filter".as_ptr(),
    c"Short Delay Slow Roll-Off Filter".as_ptr(),
    c"Voice Filter".as_ptr(),
];

// static SOC_ENUM_SINGLE_DECL(ak4619_adc_1_digi_fil, ADC_DF, AD1SL_SHIFT, ak4619_adc_digi_fil_txt);
// static SOC_ENUM_SINGLE_DECL(ak4619_adc_2_digi_fil, ADC_DF, AD2SL_SHIFT, ak4619_adc_digi_fil_txt);

/* DAC De-Emphasis Filter select */
static ak4619_dac_de_emp_txt: [*const c_char; 4] =
    [c"44.1kHz".as_ptr(), c"OFF".as_ptr(), c"48kHz".as_ptr(), c"32kHz".as_ptr()];

// static SOC_ENUM_SINGLE_DECL(ak4619_dac_1_de_emp, DAC_DEMP, DEM1_SHIFT, ak4619_dac_de_emp_txt);
// static SOC_ENUM_SINGLE_DECL(ak4619_dac_2_de_emp, DAC_DEMP, DEM2_SHIFT, ak4619_dac_de_emp_txt);

/* DAC Digital Filter select */
static ak4619_dac_digi_fil_txt: [*const c_char; 4] = [
    c"Sharp Roll-Off Filter".as_ptr(),
    c"Slow Roll-Off Filter".as_ptr(),
    c"Short Delay Sharp Roll-Off Filter".as_ptr(),
    c"Short Delay Slow Roll-Off Filter".as_ptr(),
];

// static SOC_ENUM_SINGLE_DECL(ak4619_dac_1_digi_fil, DAC_MF, DA1SL_SHIFT, ak4619_dac_digi_fil_txt);
// static SOC_ENUM_SINGLE_DECL(ak4619_dac_2_digi_fil, DAC_MF, DA2SL_SHIFT, ak4619_dac_digi_fil_txt);

/*
 * Control functions
 */

unsafe extern "C" fn ak4619_set_deemph(component: *mut snd_soc_component) {
    let ak4619 = snd_soc_component_get_drvdata(component) as *mut ak4619_priv;
    let mut dem: u8 = 0;

    if (*ak4619).deemph_en == 0 {
        return;
    }

    match (*ak4619).playback_rate {
        32000 => {
            dem |= DEM1_32000 | DEM2_32000;
        }
        44100 => {
            dem |= DEM1_44100 | DEM2_44100;
        }
        48000 => {
            dem |= DEM1_48000 | DEM2_48000;
        }
        _ => {
            dem |= DEM1_OFF | DEM2_OFF;
        }
    }
    snd_soc_component_update_bits(component, DAC_DEMP, DEM1_MASK | DEM2_MASK, dem as c_uint);
}

unsafe extern "C" fn ak4619_put_deemph(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let ak4619 = snd_soc_component_get_drvdata(component) as *mut ak4619_priv;
    let deemph_en = (*ucontrol).value.integer.value[0] as c_int;
    let mut ret: c_int = 0;

    match deemph_en {
        0 | 1 => {}
        _ => return -EINVAL,
    }

    if (*ak4619).deemph_en != deemph_en {
        ret = 1; /* The value changed */
    }

    (*ak4619).deemph_en = deemph_en;
    ak4619_set_deemph(component);

    ret
}

unsafe extern "C" fn ak4619_get_deemph(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let ak4619 = snd_soc_component_get_drvdata(component) as *mut ak4619_priv;

    (*ucontrol).value.integer.value[0] = (*ak4619).deemph_en as c_long;

    0
}

/*
 * KControls
 */
// static const struct snd_kcontrol_new ak4619_snd_controls[] = {
// 	SOC_DOUBLE_R_TLV("DAC 1 Volume", LDAC1, RDAC1, VOLDA_SHIFT, VOLDA_MAX, 1, dac_tlv),
// 	SOC_DOUBLE_R_TLV("DAC 2 Volume", LDAC2, RDAC2, VOLDA_SHIFT, VOLDA_MAX, 1, dac_tlv),
// 	SOC_DOUBLE_R_TLV("ADC 1 Volume", LADC1, RADC1, VOLAD_SHIFT, VOLAD_MAX, 1, adc_tlv),
// 	SOC_DOUBLE_R_TLV("ADC 2 Volume", LADC2, RADC2, VOLAD_SHIFT, VOLAD_MAX, 1, adc_tlv),
// 	SOC_DOUBLE_TLV("Mic 1 Volume", MIC_AMP1, MGNL_SHIFT, MGNR_SHIFT, MGN_MAX, 0, mic_tlv),
// 	SOC_DOUBLE_TLV("Mic 2 Volume", MIC_AMP2, MGNL_SHIFT, MGNR_SHIFT, MGN_MAX, 0, mic_tlv),
// 	SOC_ENUM("ADC Volume Level Transition Time", ak4619_adc_vol_trans),
// 	SOC_ENUM("DAC Volume Level Transition Time", ak4619_dac_vol_trans),
// 	SOC_SINGLE("DAC 1 Switch", DAC_MF, DA1MUTE_SHIFT, DA1MUTE_MAX, 1),
// 	SOC_SINGLE("DAC 2 Switch", DAC_MF, DA2MUTE_SHIFT, DA2MUTE_MAX, 1),
// 	SOC_SINGLE("ADC 1 Switch", ADC_MHPF, AD1MUTE_SHIFT, AD1MUTE_MAX, 1),
// 	SOC_SINGLE("ADC 2 Switch", ADC_MHPF, AD2MUTE_SHIFT, AD2MUTE_MAX, 1),
// 	SOC_ENUM("ADC 1 Digital Filter", ak4619_adc_1_digi_fil),
// 	SOC_ENUM("ADC 2 Digital Filter", ak4619_adc_2_digi_fil),
// 	SOC_SINGLE("ADC 1 HPF", ADC_MHPF, AD1HPFN_SHIFT, AD1HPFN_MAX, 1),
// 	SOC_SINGLE("ADC 2 HPF", ADC_MHPF, AD2HPFN_SHIFT, AD2HPFN_MAX, 1),
// 	SOC_ENUM("DAC 1 De-Emphasis Filter", ak4619_dac_1_de_emp),
// 	SOC_ENUM("DAC 2 De-Emphasis Filter", ak4619_dac_2_de_emp),
// 	SOC_ENUM("DAC 1 Digital Filter", ak4619_dac_1_digi_fil),
// 	SOC_ENUM("DAC 2 Digital Filter", ak4619_dac_2_digi_fil),
// 	SOC_SINGLE_BOOL_EXT("Playback De-Emphasis Switch", 0, ak4619_get_deemph, ak4619_put_deemph),
// };
static ak4619_snd_controls: [snd_kcontrol_new; 0] = [];

/*
 * DAPM
 */

/* Analog input mode */
static ak4619_analog_in_txt: [*const c_char; 4] = [
    c"Differential".as_ptr(),
    c"Single-Ended1".as_ptr(),
    c"Single-Ended2".as_ptr(),
    c"Pseudo Differential".as_ptr(),
];

// static SOC_ENUM_SINGLE_DECL(ak4619_ad_1_left_in,  ADC_AI, AD1LSEL_SHIFT, ak4619_analog_in_txt);
// static SOC_ENUM_SINGLE_DECL(ak4619_ad_1_right_in, ADC_AI, AD1RSEL_SHIFT, ak4619_analog_in_txt);
// static SOC_ENUM_SINGLE_DECL(ak4619_ad_2_left_in,  ADC_AI, AD2LSEL_SHIFT, ak4619_analog_in_txt);
// static SOC_ENUM_SINGLE_DECL(ak4619_ad_2_right_in, ADC_AI, AD2RSEL_SHIFT, ak4619_analog_in_txt);

// static const struct snd_kcontrol_new ak4619_ad_1_left_in_mux =
// 	SOC_DAPM_ENUM("Analog Input 1 Left MUX",  ak4619_ad_1_left_in);
// static const struct snd_kcontrol_new ak4619_ad_1_right_in_mux =
// 	SOC_DAPM_ENUM("Analog Input 1 Right MUX", ak4619_ad_1_right_in);
// static const struct snd_kcontrol_new ak4619_ad_2_left_in_mux =
// 	SOC_DAPM_ENUM("Analog Input 2 Left MUX",  ak4619_ad_2_left_in);
// static const struct snd_kcontrol_new ak4619_ad_2_right_in_mux =
// 	SOC_DAPM_ENUM("Analog Input 2 Right MUX", ak4619_ad_2_right_in);
static ak4619_ad_1_left_in_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static ak4619_ad_1_right_in_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static ak4619_ad_2_left_in_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static ak4619_ad_2_right_in_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };

/* DAC source mux */
static ak4619_dac_in_txt: [*const c_char; 4] =
    [c"SDIN1".as_ptr(), c"SDIN2".as_ptr(), c"SDOUT1".as_ptr(), c"SDOUT2".as_ptr()];

// static SOC_ENUM_SINGLE_DECL(ak4619_dac_1_in, DAC_IS, DAC1SEL_SHIFT, ak4619_dac_in_txt);
// static SOC_ENUM_SINGLE_DECL(ak4619_dac_2_in, DAC_IS, DAC2SEL_SHIFT, ak4619_dac_in_txt);

// static const struct snd_kcontrol_new ak4619_dac_1_in_mux =
// 	SOC_DAPM_ENUM("DAC 1 Source MUX", ak4619_dac_1_in);
// static const struct snd_kcontrol_new ak4619_dac_2_in_mux =
// 	SOC_DAPM_ENUM("DAC 2 Source MUX", ak4619_dac_2_in);
static ak4619_dac_1_in_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static ak4619_dac_2_in_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };

// static const struct snd_soc_dapm_widget ak4619_dapm_widgets[] = { ... };
static ak4619_dapm_widgets: [snd_soc_dapm_widget; 0] = [];

static ak4619_intercon: [snd_soc_dapm_route; 54] = [
    /* Dest       Connecting Widget    Source */
    /* Output path */
    snd_soc_dapm_route { sink: c"AOUT1L".as_ptr(), control: core::ptr::null(), source: c"DAC1".as_ptr() },
    snd_soc_dapm_route { sink: c"AOUT2L".as_ptr(), control: core::ptr::null(), source: c"DAC2".as_ptr() },
    snd_soc_dapm_route { sink: c"AOUT1R".as_ptr(), control: core::ptr::null(), source: c"DAC1".as_ptr() },
    snd_soc_dapm_route { sink: c"AOUT2R".as_ptr(), control: core::ptr::null(), source: c"DAC2".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC1".as_ptr(), control: core::ptr::null(), source: c"DAC 1 Source MUX".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC2".as_ptr(), control: core::ptr::null(), source: c"DAC 2 Source MUX".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC 1 Source MUX".as_ptr(), control: c"SDIN1".as_ptr(), source: c"SDIN1".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC 1 Source MUX".as_ptr(), control: c"SDIN2".as_ptr(), source: c"SDIN2".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC 1 Source MUX".as_ptr(), control: c"SDOUT1".as_ptr(), source: c"SDOUT1".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC 1 Source MUX".as_ptr(), control: c"SDOUT2".as_ptr(), source: c"SDOUT2".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC 2 Source MUX".as_ptr(), control: c"SDIN1".as_ptr(), source: c"SDIN1".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC 2 Source MUX".as_ptr(), control: c"SDIN2".as_ptr(), source: c"SDIN2".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC 2 Source MUX".as_ptr(), control: c"SDOUT1".as_ptr(), source: c"SDOUT1".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC 2 Source MUX".as_ptr(), control: c"SDOUT2".as_ptr(), source: c"SDOUT2".as_ptr() },
    /* Input path */
    snd_soc_dapm_route { sink: c"SDOUT1".as_ptr(), control: core::ptr::null(), source: c"ADC1".as_ptr() },
    snd_soc_dapm_route { sink: c"SDOUT2".as_ptr(), control: core::ptr::null(), source: c"ADC2".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC1".as_ptr(), control: core::ptr::null(), source: c"Analog Input 1 Left MUX".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC1".as_ptr(), control: core::ptr::null(), source: c"Analog Input 1 Right MUX".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC2".as_ptr(), control: core::ptr::null(), source: c"Analog Input 2 Left MUX".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC2".as_ptr(), control: core::ptr::null(), source: c"Analog Input 2 Right MUX".as_ptr() },
    snd_soc_dapm_route { sink: c"Analog Input 1 Left MUX".as_ptr(), control: c"Differential".as_ptr(), source: c"MIC1L".as_ptr() },
    snd_soc_dapm_route { sink: c"Analog Input 1 Left MUX".as_ptr(), control: c"Single-Ended1".as_ptr(), source: c"MIC1L".as_ptr() },
    snd_soc_dapm_route { sink: c"Analog Input 1 Left MUX".as_ptr(), control: c"Single-Ended2".as_ptr(), source: c"MIC1L".as_ptr() },
    snd_soc_dapm_route { sink: c"Analog Input 1 Left MUX".as_ptr(), control: c"Pseudo Differential".as_ptr(), source: c"MIC1L".as_ptr() },
    snd_soc_dapm_route { sink: c"Analog Input 1 Right MUX".as_ptr(), control: c"Differential".as_ptr(), source: c"MIC1R".as_ptr() },
    snd_soc_dapm_route { sink: c"Analog Input 1 Right MUX".as_ptr(), control: c"Single-Ended1".as_ptr(), source: c"MIC1R".as_ptr() },
    snd_soc_dapm_route { sink: c"Analog Input 1 Right MUX".as_ptr(), control: c"Single-Ended2".as_ptr(), source: c"MIC1R".as_ptr() },
    snd_soc_dapm_route { sink: c"Analog Input 1 Right MUX".as_ptr(), control: c"Pseudo Differential".as_ptr(), source: c"MIC1R".as_ptr() },
    snd_soc_dapm_route { sink: c"Analog Input 2 Left MUX".as_ptr(), control: c"Differential".as_ptr(), source: c"MIC2L".as_ptr() },
    snd_soc_dapm_route { sink: c"Analog Input 2 Left MUX".as_ptr(), control: c"Single-Ended1".as_ptr(), source: c"MIC2L".as_ptr() },
    snd_soc_dapm_route { sink: c"Analog Input 2 Left MUX".as_ptr(), control: c"Single-Ended2".as_ptr(), source: c"MIC2L".as_ptr() },
    snd_soc_dapm_route { sink: c"Analog Input 2 Left MUX".as_ptr(), control: c"Pseudo Differential".as_ptr(), source: c"MIC2L".as_ptr() },
    snd_soc_dapm_route { sink: c"Analog Input 2 Right MUX".as_ptr(), control: c"Differential".as_ptr(), source: c"MIC2R".as_ptr() },
    snd_soc_dapm_route { sink: c"Analog Input 2 Right MUX".as_ptr(), control: c"Single-Ended1".as_ptr(), source: c"MIC2R".as_ptr() },
    snd_soc_dapm_route { sink: c"Analog Input 2 Right MUX".as_ptr(), control: c"Single-Ended2".as_ptr(), source: c"MIC2R".as_ptr() },
    snd_soc_dapm_route { sink: c"Analog Input 2 Right MUX".as_ptr(), control: c"Pseudo Differential".as_ptr(), source: c"MIC2R".as_ptr() },
    snd_soc_dapm_route { sink: c"MIC1L".as_ptr(), control: core::ptr::null(), source: c"AIN1L".as_ptr() },
    snd_soc_dapm_route { sink: c"MIC1L".as_ptr(), control: core::ptr::null(), source: c"AIN2L".as_ptr() },
    snd_soc_dapm_route { sink: c"MIC1R".as_ptr(), control: core::ptr::null(), source: c"AIN1R".as_ptr() },
    snd_soc_dapm_route { sink: c"MIC1R".as_ptr(), control: core::ptr::null(), source: c"AIN2R".as_ptr() },
    snd_soc_dapm_route { sink: c"MIC2L".as_ptr(), control: core::ptr::null(), source: c"AIN4L".as_ptr() },
    snd_soc_dapm_route { sink: c"MIC2L".as_ptr(), control: core::ptr::null(), source: c"AIN5L".as_ptr() },
    snd_soc_dapm_route { sink: c"MIC2R".as_ptr(), control: core::ptr::null(), source: c"AIN4R".as_ptr() },
    snd_soc_dapm_route { sink: c"MIC2R".as_ptr(), control: core::ptr::null(), source: c"AIN5R".as_ptr() },
];

static ak4619_reg_defaults: [reg_default; 20] = [
    reg_default { reg: PWR_MGMT, def: 0x00 },
    reg_default { reg: AU_IFF1, def: 0x0C },
    reg_default { reg: AU_IFF2, def: 0x0C },
    reg_default { reg: SYS_CLK, def: 0x00 },
    reg_default { reg: MIC_AMP1, def: 0x22 },
    reg_default { reg: MIC_AMP2, def: 0x22 },
    reg_default { reg: LADC1, def: 0x30 },
    reg_default { reg: RADC1, def: 0x30 },
    reg_default { reg: LADC2, def: 0x30 },
    reg_default { reg: RADC2, def: 0x30 },
    reg_default { reg: ADC_DF, def: 0x00 },
    reg_default { reg: ADC_AI, def: 0x00 },
    reg_default { reg: ADC_MHPF, def: 0x00 },
    reg_default { reg: LDAC1, def: 0x18 },
    reg_default { reg: RDAC1, def: 0x18 },
    reg_default { reg: LDAC2, def: 0x18 },
    reg_default { reg: RDAC2, def: 0x18 },
    reg_default { reg: DAC_IS, def: 0x04 },
    reg_default { reg: DAC_DEMP, def: 0x05 },
    reg_default { reg: DAC_MF, def: 0x0A },
];

unsafe extern "C" fn ak4619_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let mut pwr_ctrl: u8 = 0;

    match level {
        SND_SOC_BIAS_ON => {
            pwr_ctrl |= RSTN;
            pwr_ctrl |= PMAD1 | PMAD2 | PMDA1 | PMDA2;
        }
        SND_SOC_BIAS_PREPARE => {
            pwr_ctrl |= PMAD1 | PMAD2 | PMDA1 | PMDA2;
        }
        SND_SOC_BIAS_STANDBY | SND_SOC_BIAS_OFF | _ => {}
    }

    snd_soc_component_write(component, PWR_MGMT, pwr_ctrl as c_uint);

    0
}

unsafe extern "C" fn ak4619_dai_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let ak4619 = snd_soc_component_get_drvdata(component) as *mut ak4619_priv;
    let width: c_uint;
    let rate: c_uint;
    let fs: c_uint;
    let is_play = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    let mut dai_ctrl: u8 = 0;
    let mut clk_mode: u8 = 0;

    width = params_width(params);
    match width {
        16 => dai_ctrl |= if is_play { DIDL_16 } else { DODL_16 },
        20 => dai_ctrl |= if is_play { DIDL_20 } else { DODL_20 },
        24 => dai_ctrl |= if is_play { DIDL_24 } else { DODL_24 },
        32 => {
            if is_play {
                dai_ctrl |= DIDL_32;
            } else {
                return -EINVAL;
            }
        }
        _ => return -EINVAL,
    }

    rate = params_rate(params);
    if rate != 0 {
        fs = (*ak4619).sysclk / rate;
    } else {
        return -EINVAL;
    }

    match rate {
        8000 | 11025 | 12000 | 16000 | 22050 | 24000 | 32000 | 44100 | 48000 => {
            match fs {
                256 => clk_mode |= (0x0 << 0) as u8,
                384 => clk_mode |= (0x2 << 0) as u8,
                512 => clk_mode |= (0x3 << 0) as u8,
                _ => return -EINVAL,
            }
        }
        64000 | 88200 | 96000 => {
            if fs == 256 {
                clk_mode |= (0x1 << 0) as u8;
            } else {
                return -EINVAL;
            }
        }
        176400 | 192000 => {
            if fs == 128 {
                clk_mode |= (0x4 << 0) as u8;
            } else {
                return -EINVAL;
            }
        }
        _ => return -EINVAL,
    }

    snd_soc_component_update_bits(component, SYS_CLK, FS_MASK, clk_mode as c_uint);
    snd_soc_component_update_bits(
        component,
        AU_IFF2,
        if is_play { DIDL_MASK } else { DODL_MASK },
        dai_ctrl as c_uint,
    );

    if is_play {
        (*ak4619).playback_rate = rate;
        ak4619_set_deemph(component);
    }

    0
}

unsafe extern "C" fn ak4619_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let mut dai_fmt1: u8 = 0;
    let mut dai_fmt2: u8 = 0;

    /* Set clock normal/inverted */
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_NF => dai_fmt1 |= BCKP,
        SND_SOC_DAIFMT_NB_IF | SND_SOC_DAIFMT_IB_IF | _ => return -EINVAL,
    }

    /* Only Stereo modes are supported */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => dai_fmt1 |= DCF_STEREO_I2S,
        SND_SOC_DAIFMT_LEFT_J => dai_fmt1 |= DCF_STEREO_MSB,
        SND_SOC_DAIFMT_DSP_A => {
            /* L data MSB after FRM LRC */
            dai_fmt1 |= DCF_PCM_SF;
            dai_fmt2 |= SLOT;
        }
        SND_SOC_DAIFMT_DSP_B => {
            /* L data MSB during FRM LRC */
            dai_fmt1 |= DCF_PCM_LF;
            dai_fmt2 |= SLOT;
        }
        _ => return -EINVAL,
    }

    /* Only slave mode is support */
    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }

    /* By default only 64 BICK per LRCLK is supported */
    dai_fmt1 |= DSL_32;

    snd_soc_component_update_bits(
        component,
        AU_IFF1,
        DCF_MASK | DSL_MASK | BCKP as c_uint,
        dai_fmt1 as c_uint,
    );
    snd_soc_component_update_bits(component, AU_IFF2, SLOT as c_uint, dai_fmt2 as c_uint);

    0
}

unsafe extern "C" fn ak4619_dai_set_sysclk(
    codec_dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = (*codec_dai).component;
    let ak4619 = snd_soc_component_get_drvdata(component) as *mut ak4619_priv;

    (*ak4619).sysclk = freq;

    0
}

unsafe extern "C" fn ak4619_dai_mute(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    let component = (*dai).component;

    snd_soc_component_update_bits(
        component,
        DAC_MF,
        DA1MUTE_EN,
        if mute != 0 { DA1MUTE_EN } else { 0 },
    );
    snd_soc_component_update_bits(
        component,
        DAC_MF,
        DA2MUTE_EN,
        if mute != 0 { DA2MUTE_EN } else { 0 },
    );

    0
}

unsafe extern "C" fn ak4619_hw_constraints(
    ak4619: *mut ak4619_priv,
    runtime: *mut snd_pcm_runtime,
) {
    let constraint = &mut (*ak4619).constraint as *mut snd_pcm_hw_constraint_list;
    let mut ak4619_rate_mask: c_int = 0;
    let mut fs: c_uint;
    let mut i: c_int;
    static ak4619_sr: [c_uint; 14] = [
        8000, 11025, 12000, 16000, 22050, 24000, 32000, 44100, 48000, 64000, 88200, 96000, 176400,
        192000,
    ];

    /*
     *	[8kHz - 48kHz]		: 256fs, 384fs or 512fs
     *	[64kHz - 96kHz]		: 256fs
     *	[176.4kHz, 192kHz]	: 128fs
     */

    i = 0;
    while i < ak4619_sr.len() as c_int {
        fs = (*ak4619).sysclk / ak4619_sr[i as usize];

        match fs {
            512 | 384 | 256 => {
                ak4619_rate_mask |= 1 << i;
            }
            128 => match i {
                x if x == (ak4619_sr.len() as c_int - 1) => {
                    ak4619_rate_mask |= 1 << i;
                }
                x if x == (ak4619_sr.len() as c_int - 2) => {
                    ak4619_rate_mask |= 1 << i;
                }
                _ => {}
            },
            _ => {}
        }
        i += 1;
    }

    (*constraint).list = ak4619_sr.as_ptr();
    (*constraint).mask = ak4619_rate_mask as c_uint;
    (*constraint).count = ak4619_sr.len() as c_uint;

    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, constraint);
}

const PLAYBACK_MODE: u32 = 0;
const CAPTURE_MODE: u32 = 1;

unsafe extern "C" fn ak4619_dai_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let ak4619 = snd_soc_component_get_drvdata(component) as *mut ak4619_priv;

    ak4619_hw_constraints(ak4619, (*substream).runtime);

    0
}

static mut ak4619_dai_formats: [u64; 2] = [
    /* First Priority */
    SND_SOC_POSSIBLE_DAIFMT_I2S | SND_SOC_POSSIBLE_DAIFMT_LEFT_J,
    /* Second Priority */
    SND_SOC_POSSIBLE_DAIFMT_I2S
        | SND_SOC_POSSIBLE_DAIFMT_LEFT_J
        | SND_SOC_POSSIBLE_DAIFMT_DSP_A
        | SND_SOC_POSSIBLE_DAIFMT_DSP_B,
];

static ak4619_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(ak4619_dai_startup),
    set_sysclk: Some(ak4619_dai_set_sysclk),
    set_fmt: Some(ak4619_dai_set_fmt),
    hw_params: Some(ak4619_dai_hw_params),
    mute_stream: Some(ak4619_dai_mute),
    auto_selectable_formats: unsafe { ak4619_dai_formats.as_mut_ptr() },
    num_auto_selectable_formats: 2,
};

static soc_component_dev_ak4619: snd_soc_component_driver = snd_soc_component_driver {
    set_bias_level: Some(ak4619_set_bias_level),
    controls: ak4619_snd_controls.as_ptr(),
    num_controls: 0,
    dapm_widgets: ak4619_dapm_widgets.as_ptr(),
    num_dapm_widgets: 0,
    dapm_routes: ak4619_intercon.as_ptr(),
    num_dapm_routes: 54,
    idle_bias_on: 1,
    endianness: 1,
};

static ak4619_regmap_cfg: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: 0x14,
    reg_defaults: ak4619_reg_defaults.as_ptr(),
    num_reg_defaults: 20,
    cache_type: REGCACHE_MAPLE,
};

static ak4619_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"asahi-kasei,ak4619".as_ptr(),
        data: &ak4619_regmap_cfg as *const regmap_config as *const c_void,
    },
    of_device_id {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, ak4619_of_match);

static ak4619_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: c"ak4619".as_ptr(),
        driver_data: &ak4619_regmap_cfg as *const regmap_config as kernel_ulong_t,
    },
    i2c_device_id {
        name: core::ptr::null(),
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(i2c, ak4619_i2c_id);

const AK4619_RATES: c_uint = SNDRV_PCM_RATE_8000_192000;

const AK4619_DAC_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S20_LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE;

const AK4619_ADC_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_LE | SNDRV_PCM_FMTBIT_S24_LE;

static mut ak4619_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"ak4619-hifi".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: AK4619_RATES,
        formats: AK4619_DAC_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: AK4619_RATES,
        formats: AK4619_ADC_FORMATS,
    },
    ops: &ak4619_dai_ops,
    symmetric_rate: 1,
};

unsafe extern "C" fn ak4619_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let dev = &mut (*i2c).dev as *mut device;
    let ak4619: *mut ak4619_priv;
    let mut ret: c_int;

    ak4619 = devm_kzalloc(dev, core::mem::size_of::<ak4619_priv>(), GFP_KERNEL) as *mut ak4619_priv;
    if ak4619.is_null() {
        return -ENOMEM;
    }

    i2c_set_clientdata(i2c, ak4619 as *mut c_void);

    (*ak4619).regmap = devm_regmap_init_i2c(i2c, &ak4619_regmap_cfg);
    if IS_ERR((*ak4619).regmap as *const c_void) {
        ret = PTR_ERR((*ak4619).regmap as *const c_void);
        dev_err(dev, c"regmap_init() failed: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = devm_snd_soc_register_component(dev, &soc_component_dev_ak4619, &mut ak4619_dai, 1);
    if ret < 0 {
        dev_err(
            dev,
            c"Failed to register ak4619 component: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    0
}

static ak4619_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"ak4619-codec".as_ptr(),
        of_match_table: ak4619_of_match.as_ptr(),
    },
    probe: Some(ak4619_i2c_probe),
    id_table: ak4619_i2c_id.as_ptr(),
};
// module_i2c_driver(ak4619_i2c_driver);

// MODULE_DESCRIPTION("SoC AK4619 driver");
// MODULE_AUTHOR("Khanh Le <khanh.le.xr@renesas.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
