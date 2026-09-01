// SPDX-License-Identifier: GPL-2.0
//
// ak4642.c  --  AK4642/AK4643 ALSA Soc Audio driver
//
// Copyright (C) 2009 Renesas Solutions Corp.
// Kuninori Morimoto <morimoto.kuninori@renesas.com>
//
// Based on wm8731.c by Richard Purdie
// Based on ak4535.c by Richard Purdie
// Based on wm8753.c by Liam Girdwood

/* ** CAUTION **
 *
 * This is very simple driver.
 * It can use headphone output / stereo input only
 *
 * AK4642 is tested.
 * AK4643 is tested.
 * AK4648 is tested.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

const PW_MGMT1: c_uint = 0x00;
const PW_MGMT2: c_uint = 0x01;
const SG_SL1: c_uint = 0x02;
const SG_SL2: c_uint = 0x03;
const MD_CTL1: c_uint = 0x04;
const MD_CTL2: c_uint = 0x05;
const TIMER: c_uint = 0x06;
const ALC_CTL1: c_uint = 0x07;
const ALC_CTL2: c_uint = 0x08;
const L_IVC: c_uint = 0x09;
const L_DVC: c_uint = 0x0a;
const ALC_CTL3: c_uint = 0x0b;
const R_IVC: c_uint = 0x0c;
const R_DVC: c_uint = 0x0d;
const MD_CTL3: c_uint = 0x0e;
const MD_CTL4: c_uint = 0x0f;
const PW_MGMT3: c_uint = 0x10;
const DF_S: c_uint = 0x11;
const FIL3_0: c_uint = 0x12;
const FIL3_1: c_uint = 0x13;
const FIL3_2: c_uint = 0x14;
const FIL3_3: c_uint = 0x15;
const EQ_0: c_uint = 0x16;
const EQ_1: c_uint = 0x17;
const EQ_2: c_uint = 0x18;
const EQ_3: c_uint = 0x19;
const EQ_4: c_uint = 0x1a;
const EQ_5: c_uint = 0x1b;
const FIL1_0: c_uint = 0x1c;
const FIL1_1: c_uint = 0x1d;
const FIL1_2: c_uint = 0x1e;
const FIL1_3: c_uint = 0x1f; /* The maximum valid register for ak4642 */
const PW_MGMT4: c_uint = 0x20;
const MD_CTL5: c_uint = 0x21;
const LO_MS: c_uint = 0x22;
const HP_MS: c_uint = 0x23;
const SPK_MS: c_uint = 0x24; /* The maximum valid register for ak4643 */
const EQ_FBEQAB: c_uint = 0x25;
const EQ_FBEQCD: c_uint = 0x26;
const EQ_FBEQE: c_uint = 0x27; /* The maximum valid register for ak4648 */

/* PW_MGMT1*/
const PMVCM: u8 = 1 << 6; /* VCOM Power Management */
const PMMIN: u8 = 1 << 5; /* MIN Input Power Management */
const PMDAC: u8 = 1 << 2; /* DAC Power Management */
const PMADL: u8 = 1 << 0; /* MIC Amp Lch and ADC Lch Power Management */

/* PW_MGMT2 */
const HPMTN: u8 = 1 << 6;
const PMHPL: u8 = 1 << 5;
const PMHPR: u8 = 1 << 4;
const MS: u8 = 1 << 3; /* master/slave select */
const MCKO: u8 = 1 << 1;
const PMPLL: u8 = 1 << 0;

const PMHP_MASK: u8 = PMHPL | PMHPR;
const PMHP: u8 = PMHP_MASK;

/* PW_MGMT3 */
const PMADR: u8 = 1 << 0; /* MIC L / ADC R Power Management */

/* SG_SL1 */
const MINS: u8 = 1 << 6; /* Switch from MIN to Speaker */
const DACL: u8 = 1 << 4; /* Switch from DAC to Stereo or Receiver */
const PMMP: u8 = 1 << 2; /* MPWR pin Power Management */
const MGAIN0: u8 = 1 << 0; /* MIC amp gain*/

/* SG_SL2 */
const LOPS: u8 = 1 << 6; /* Stero Line-out Power Save Mode */

/* TIMER */
const fn ZTM(param: u8) -> u8 {
    (param & 0x3) << 4
}
const fn WTM(param: u8) -> u8 {
    ((param & 0x4) << 4) | ((param & 0x3) << 2)
}

/* ALC_CTL1 */
const ALC: u8 = 1 << 5; /* ALC Enable */
const LMTH0: u8 = 1 << 0; /* ALC Limiter / Recovery Level */

/* MD_CTL1 */
const PLL3: u8 = 1 << 7;
const PLL2: u8 = 1 << 6;
const PLL1: u8 = 1 << 5;
const PLL0: u8 = 1 << 4;
const PLL_MASK: u8 = PLL3 | PLL2 | PLL1 | PLL0;

const BCKO_MASK: u8 = 1 << 3;
const BCKO_64: u8 = BCKO_MASK;

const DIF_MASK: u8 = 3 << 0;
const DSP: u8 = 0 << 0;
const RIGHT_J: u8 = 1 << 0;
const LEFT_J: u8 = 2 << 0;
const I2S: u8 = 3 << 0;

/* MD_CTL2 */
const fn FSs(val: c_int) -> u8 {
    (((val & 0x7) << 0) | ((val & 0x8) << 2)) as u8
}
const fn PSs(val: c_int) -> u8 {
    ((val & 0x3) << 6) as u8
}

/* MD_CTL3 */
const BST1: u8 = 1 << 3;

/* MD_CTL4 */
const DACH: u8 = 1 << 0;

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
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
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
pub struct fwnode_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    pub name: *const c_char,
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
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
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
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
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
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

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_desc,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub idle_bias_on: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_OFF = 0,
}

#[repr(C)]
pub struct ak4642_drvdata {
    pub regmap_config: *const regmap_config,
    pub extended_frequencies: c_int,
}

#[repr(C)]
pub struct ak4642_priv {
    pub drvdata: *const ak4642_drvdata,
    pub mcko: *mut clk,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: *const c_char,
    pub driver_data: c_ulong,
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

extern "C" {
    static out_tlv: [c_uint; 0];
    static ak4642_snd_controls: [snd_kcontrol_new; 3];
    static ak4642_headphone_control: snd_kcontrol_new;
    static ak4642_lout_mixer_controls: [snd_kcontrol_new; 1];
    static ak4642_dapm_widgets: [snd_soc_dapm_widget_desc; 8];

    static SND_SOC_DAPM_PRE_PMD: c_int;
    static SND_SOC_DAPM_PRE_PMU: c_int;
    static SND_SOC_DAPM_POST_PMU: c_int;
    static SND_SOC_DAPM_POST_PMD: c_int;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_NOPM: c_int;
    static SNDRV_PCM_RATE_8000_48000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static REGCACHE_RBTREE: c_uint;
    static GFP_KERNEL: c_uint;

    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: u8, val: u8) -> c_int;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn msleep(msecs: c_uint);
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn clk_get_rate(clk: *mut clk) -> u32;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn dev_get_regmap(dev: *mut device, name: *const c_char) -> *mut regmap;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn dev_fwnode(dev: *mut device) -> *mut fwnode_handle;
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out_value: *mut u32) -> c_int;
    fn of_property_read_bool(np: *mut device_node, propname: *const c_char) -> bool;
    fn of_clk_get_parent_name(np: *mut device_node, index: c_int) -> *const c_char;
    fn of_property_read_string(np: *mut device_node, propname: *const c_char, out_string: *mut *const c_char) -> c_int;
    fn clk_register_fixed_rate(
        dev: *mut device,
        name: *const c_char,
        parent_name: *const c_char,
        flags: c_ulong,
        fixed_rate: c_uint,
    ) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn of_clk_add_provider(
        np: *mut device_node,
        clk_src_get: *const c_void,
        data: *mut c_void,
    ) -> c_int;
    static of_clk_src_simple_get: c_void;
    fn i2c_get_match_data(client: *mut i2c_client) -> *const c_void;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

/*
 * Playback Volume (table 39)
 *
 * max : 0x00 : +12.0 dB
 *       ( 0.5 dB step )
 * min : 0xFE : -115.0 dB
 * mute: 0xFF
 */
// static const DECLARE_TLV_DB_SCALE(out_tlv, -11550, 50, 1);

// static const struct snd_kcontrol_new ak4642_snd_controls[] = {
// 	SOC_DOUBLE_R_TLV("Digital Playback Volume", L_DVC, R_DVC,
// 			 0, 0xFF, 1, out_tlv),
// 	SOC_SINGLE("ALC Capture Switch", ALC_CTL1, 5, 1, 0),
// 	SOC_SINGLE("ALC Capture ZC Switch", ALC_CTL1, 4, 1, 1),
// };
//
// static const struct snd_kcontrol_new ak4642_headphone_control =
// 	SOC_DAPM_SINGLE("Switch", PW_MGMT2, 6, 1, 0);
//
// static const struct snd_kcontrol_new ak4642_lout_mixer_controls[] = {
// 	SOC_DAPM_SINGLE("DACL", SG_SL1, 4, 1, 0),
// };

/* event handlers */
unsafe extern "C" fn ak4642_lout_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);

    if event == SND_SOC_DAPM_PRE_PMD || event == SND_SOC_DAPM_PRE_PMU {
        /* Power save mode ON */
        snd_soc_component_update_bits(component, SG_SL2, LOPS, LOPS);
    } else if event == SND_SOC_DAPM_POST_PMU || event == SND_SOC_DAPM_POST_PMD {
        /* Power save mode OFF */
        msleep(300);
        snd_soc_component_update_bits(component, SG_SL2, LOPS, 0);
    }

    0
}

// static const struct snd_soc_dapm_widget ak4642_dapm_widgets[] = {
// 	/* Outputs */
// 	SND_SOC_DAPM_OUTPUT("HPOUTL"),
// 	SND_SOC_DAPM_OUTPUT("HPOUTR"),
// 	SND_SOC_DAPM_OUTPUT("LINEOUT"),
// 	SND_SOC_DAPM_PGA("HPL Out", PW_MGMT2, 5, 0, NULL, 0),
// 	SND_SOC_DAPM_PGA("HPR Out", PW_MGMT2, 4, 0, NULL, 0),
// 	SND_SOC_DAPM_SWITCH("Headphone Enable", SND_SOC_NOPM, 0, 0,
// 			    &ak4642_headphone_control),
// 	SND_SOC_DAPM_PGA("DACH", MD_CTL4, 0, 0, NULL, 0),
// 	SND_SOC_DAPM_MIXER_E("LINEOUT Mixer", PW_MGMT1, 3, 0,
// 			   &ak4642_lout_mixer_controls[0],
// 			   ARRAY_SIZE(ak4642_lout_mixer_controls),
// 			   ak4642_lout_event,
// 			   SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU |
// 			   SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD),
// 	/* DAC */
// 	SND_SOC_DAPM_DAC("DAC", NULL, PW_MGMT1, 2, 0),
// };

static ak4642_intercon: [snd_soc_dapm_route; 9] = [
    snd_soc_dapm_route { sink: b"HPOUTL\0".as_ptr() as *const c_char, control: ptr::null(), source: b"HPL Out\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"HPOUTR\0".as_ptr() as *const c_char, control: ptr::null(), source: b"HPR Out\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"LINEOUT\0".as_ptr() as *const c_char, control: ptr::null(), source: b"LINEOUT Mixer\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"HPL Out\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Headphone Enable\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"HPR Out\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Headphone Enable\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headphone Enable\0".as_ptr() as *const c_char, control: b"Switch\0".as_ptr() as *const c_char, source: b"DACH\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DACH\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DAC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"LINEOUT Mixer\0".as_ptr() as *const c_char, control: b"DACL\0".as_ptr() as *const c_char, source: b"DAC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Playback\0".as_ptr() as *const c_char },
];

/*
 * ak4642 register cache
 */
static ak4643_reg: [reg_default; 37] = [
    reg_default { reg: 0, def: 0x00 }, reg_default { reg: 1, def: 0x00 }, reg_default { reg: 2, def: 0x01 }, reg_default { reg: 3, def: 0x00 },
    reg_default { reg: 4, def: 0x02 }, reg_default { reg: 5, def: 0x00 }, reg_default { reg: 6, def: 0x00 }, reg_default { reg: 7, def: 0x00 },
    reg_default { reg: 8, def: 0xe1 }, reg_default { reg: 9, def: 0xe1 }, reg_default { reg: 10, def: 0x18 }, reg_default { reg: 11, def: 0x00 },
    reg_default { reg: 12, def: 0xe1 }, reg_default { reg: 13, def: 0x18 }, reg_default { reg: 14, def: 0x11 }, reg_default { reg: 15, def: 0x08 },
    reg_default { reg: 16, def: 0x00 }, reg_default { reg: 17, def: 0x00 }, reg_default { reg: 18, def: 0x00 }, reg_default { reg: 19, def: 0x00 },
    reg_default { reg: 20, def: 0x00 }, reg_default { reg: 21, def: 0x00 }, reg_default { reg: 22, def: 0x00 }, reg_default { reg: 23, def: 0x00 },
    reg_default { reg: 24, def: 0x00 }, reg_default { reg: 25, def: 0x00 }, reg_default { reg: 26, def: 0x00 }, reg_default { reg: 27, def: 0x00 },
    reg_default { reg: 28, def: 0x00 }, reg_default { reg: 29, def: 0x00 }, reg_default { reg: 30, def: 0x00 }, reg_default { reg: 31, def: 0x00 },
    reg_default { reg: 32, def: 0x00 }, reg_default { reg: 33, def: 0x00 }, reg_default { reg: 34, def: 0x00 }, reg_default { reg: 35, def: 0x00 },
    reg_default { reg: 36, def: 0x00 },
];

/* The default settings for 0x0 ~ 0x1f registers are the same for ak4642
   and ak4643. So we reuse the ak4643 reg_default for ak4642.
   The valid registers for ak4642 are 0x0 ~ 0x1f which is a subset of ak4643,
   so define NUM_AK4642_REG_DEFAULTS for ak4642.
*/
static ak4642_reg: *const reg_default = ak4643_reg.as_ptr();
const NUM_AK4642_REG_DEFAULTS: c_uint = FIL1_3 + 1;

static ak4648_reg: [reg_default; 40] = [
    reg_default { reg: 0, def: 0x00 }, reg_default { reg: 1, def: 0x00 }, reg_default { reg: 2, def: 0x01 }, reg_default { reg: 3, def: 0x00 },
    reg_default { reg: 4, def: 0x02 }, reg_default { reg: 5, def: 0x00 }, reg_default { reg: 6, def: 0x00 }, reg_default { reg: 7, def: 0x00 },
    reg_default { reg: 8, def: 0xe1 }, reg_default { reg: 9, def: 0xe1 }, reg_default { reg: 10, def: 0x18 }, reg_default { reg: 11, def: 0x00 },
    reg_default { reg: 12, def: 0xe1 }, reg_default { reg: 13, def: 0x18 }, reg_default { reg: 14, def: 0x11 }, reg_default { reg: 15, def: 0xb8 },
    reg_default { reg: 16, def: 0x00 }, reg_default { reg: 17, def: 0x00 }, reg_default { reg: 18, def: 0x00 }, reg_default { reg: 19, def: 0x00 },
    reg_default { reg: 20, def: 0x00 }, reg_default { reg: 21, def: 0x00 }, reg_default { reg: 22, def: 0x00 }, reg_default { reg: 23, def: 0x00 },
    reg_default { reg: 24, def: 0x00 }, reg_default { reg: 25, def: 0x00 }, reg_default { reg: 26, def: 0x00 }, reg_default { reg: 27, def: 0x00 },
    reg_default { reg: 28, def: 0x00 }, reg_default { reg: 29, def: 0x00 }, reg_default { reg: 30, def: 0x00 }, reg_default { reg: 31, def: 0x00 },
    reg_default { reg: 32, def: 0x00 }, reg_default { reg: 33, def: 0x00 }, reg_default { reg: 34, def: 0x00 }, reg_default { reg: 35, def: 0x00 },
    reg_default { reg: 36, def: 0x00 }, reg_default { reg: 37, def: 0x88 }, reg_default { reg: 38, def: 0x88 }, reg_default { reg: 39, def: 0x08 },
];

unsafe extern "C" fn ak4642_dai_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let is_play = ((*substream).stream == SNDRV_PCM_STREAM_PLAYBACK) as c_int;
    let component = (*dai).component;

    if is_play != 0 {
        /*
         * start headphone output
         *
         * PLL, Master Mode
         * Audio I/F Format :MSB justified (ADC & DAC)
         * Bass Boost Level : Middle
         *
         * This operation came from example code of
         * "ASAHI KASEI AK4642" (japanese) manual p97.
         */
        snd_soc_component_write(component, L_IVC, 0x91); /* volume */
        snd_soc_component_write(component, R_IVC, 0x91); /* volume */
    } else {
        /*
         * start stereo input
         *
         * PLL Master Mode
         * Audio I/F Format:MSB justified (ADC & DAC)
         * Pre MIC AMP:+20dB
         * MIC Power On
         * ALC setting:Refer to Table 35
         * ALC bit=“1”
         *
         * This operation came from example code of
         * "ASAHI KASEI AK4642" (japanese) manual p94.
         */
        snd_soc_component_update_bits(component, SG_SL1, PMMP | MGAIN0, PMMP | MGAIN0);
        snd_soc_component_write(component, TIMER, (ZTM(0x3) | WTM(0x3)) as c_uint);
        snd_soc_component_write(component, ALC_CTL1, (ALC | LMTH0) as c_uint);
        snd_soc_component_update_bits(component, PW_MGMT1, PMADL, PMADL);
        snd_soc_component_update_bits(component, PW_MGMT3, PMADR, PMADR);
    }

    0
}

unsafe extern "C" fn ak4642_dai_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let is_play = ((*substream).stream == SNDRV_PCM_STREAM_PLAYBACK) as c_int;
    let component = (*dai).component;

    if is_play != 0 {
    } else {
        /* stop stereo input */
        snd_soc_component_update_bits(component, PW_MGMT1, PMADL, 0);
        snd_soc_component_update_bits(component, PW_MGMT3, PMADR, 0);
        snd_soc_component_update_bits(component, ALC_CTL1, ALC, 0);
    }
}

unsafe extern "C" fn ak4642_dai_set_sysclk(
    codec_dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = (*codec_dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut ak4642_priv;
    let pll: u8;
    let mut extended_freq: c_int = 0;

    match freq {
        11289600 => {
            pll = PLL2;
        }
        12288000 => {
            pll = PLL2 | PLL0;
        }
        12000000 => {
            pll = PLL2 | PLL1;
        }
        24000000 => {
            pll = PLL2 | PLL1 | PLL0;
        }
        13500000 => {
            pll = PLL3 | PLL2;
        }
        27000000 => {
            pll = PLL3 | PLL2 | PLL0;
        }
        19200000 => {
            pll = PLL3;
            extended_freq = 1;
        }
        13000000 => {
            pll = PLL3 | PLL2 | PLL1;
            extended_freq = 1;
        }
        26000000 => {
            pll = PLL3 | PLL2 | PLL1 | PLL0;
            extended_freq = 1;
        }
        _ => {
            return -EINVAL;
        }
    }

    if extended_freq != 0 && (*(*priv_).drvdata).extended_frequencies == 0 {
        return -EINVAL;
    }

    snd_soc_component_update_bits(component, MD_CTL1, PLL_MASK, pll);

    0
}

unsafe extern "C" fn ak4642_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let mut data: u8;
    let mut bcko: u8;

    data = MCKO | PMPLL; /* use MCKO */
    bcko = 0;

    /* set clocking for audio interface */
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        x if x == SND_SOC_DAIFMT_CBP_CFP => {
            data |= MS;
            bcko = BCKO_64;
        }
        x if x == SND_SOC_DAIFMT_CBC_CFC => {}
        _ => {
            return -EINVAL;
        }
    }
    snd_soc_component_update_bits(component, PW_MGMT2, MS | MCKO | PMPLL, data);
    snd_soc_component_update_bits(component, MD_CTL1, BCKO_MASK, bcko);

    /* format type */
    data = 0;
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_LEFT_J => {
            data = LEFT_J;
        }
        x if x == SND_SOC_DAIFMT_I2S => {
            data = I2S;
        }
        /* FIXME
         * Please add RIGHT_J / DSP support here
         */
        _ => {
            return -EINVAL;
        }
    }
    snd_soc_component_update_bits(component, MD_CTL1, DIF_MASK, data);

    0
}

unsafe extern "C" fn ak4642_set_mcko(component: *mut snd_soc_component, frequency: u32) -> c_int {
    static fs_list: [u32; 16] = [
        8000, 12000, 16000, 24000, 7350, 11025, 14700, 22050,
        0, 0, 32000, 48000, 0, 0, 29400, 44100,
    ];
    static ps_list: [u32; 4] = [256, 128, 64, 32];
    let mut ps: c_int;
    let mut fs: c_int;

    ps = 0;
    while (ps as usize) < ps_list.len() {
        fs = 0;
        while (fs as usize) < fs_list.len() {
            if frequency == ps_list[ps as usize].wrapping_mul(fs_list[fs as usize]) {
                snd_soc_component_write(component, MD_CTL2, (PSs(ps) | FSs(fs)) as c_uint);
                return 0;
            }
            fs += 1;
        }
        ps += 1;
    }

    0
}

unsafe extern "C" fn ak4642_dai_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut ak4642_priv;
    let mut rate: u32 = clk_get_rate((*priv_).mcko);

    if rate == 0 {
        rate = params_rate(params).wrapping_mul(256);
    }

    ak4642_set_mcko(component, rate)
}

unsafe extern "C" fn ak4642_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    match level {
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            snd_soc_component_write(component, PW_MGMT1, 0x00);
        }
        _ => {
            snd_soc_component_update_bits(component, PW_MGMT1, PMVCM, PMVCM);
        }
    }

    0
}

static ak4642_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(ak4642_dai_startup),
    shutdown: Some(ak4642_dai_shutdown),
    set_sysclk: Some(ak4642_dai_set_sysclk),
    set_fmt: Some(ak4642_dai_set_fmt),
    hw_params: Some(ak4642_dai_hw_params),
};

static mut ak4642_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"ak4642-hifi\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 2,
        rates: unsafe { SNDRV_PCM_RATE_8000_48000 },
        formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE },
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 2,
        rates: unsafe { SNDRV_PCM_RATE_8000_48000 },
        formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE },
    },
    ops: &ak4642_dai_ops,
    symmetric_rate: 1,
};

unsafe extern "C" fn ak4642_suspend(component: *mut snd_soc_component) -> c_int {
    let regmap = dev_get_regmap((*component).dev, ptr::null());

    regcache_cache_only(regmap, true);
    regcache_mark_dirty(regmap);
    0
}

unsafe extern "C" fn ak4642_resume(component: *mut snd_soc_component) -> c_int {
    let regmap = dev_get_regmap((*component).dev, ptr::null());

    regcache_cache_only(regmap, false);
    regcache_sync(regmap);
    0
}

unsafe extern "C" fn ak4642_probe(component: *mut snd_soc_component) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut ak4642_priv;

    if !(*priv_).mcko.is_null() {
        ak4642_set_mcko(component, clk_get_rate((*priv_).mcko));
    }

    0
}

static soc_component_dev_ak4642: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(ak4642_probe),
    suspend: Some(ak4642_suspend),
    resume: Some(ak4642_resume),
    set_bias_level: Some(ak4642_set_bias_level),
    controls: unsafe { ak4642_snd_controls.as_ptr() },
    num_controls: 3,
    dapm_widgets: unsafe { ak4642_dapm_widgets.as_ptr() },
    num_dapm_widgets: 8,
    dapm_routes: ak4642_intercon.as_ptr(),
    num_dapm_routes: 9,
    idle_bias_on: 1,
    endianness: 1,
};

static ak4642_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: FIL1_3,
    reg_defaults: ak4642_reg,
    num_reg_defaults: NUM_AK4642_REG_DEFAULTS,
    cache_type: unsafe { REGCACHE_RBTREE },
};

static ak4643_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: SPK_MS,
    reg_defaults: ak4643_reg.as_ptr(),
    num_reg_defaults: 37,
    cache_type: unsafe { REGCACHE_RBTREE },
};

static ak4648_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: EQ_FBEQE,
    reg_defaults: ak4648_reg.as_ptr(),
    num_reg_defaults: 40,
    cache_type: unsafe { REGCACHE_RBTREE },
};

static ak4642_drvdata: ak4642_drvdata = ak4642_drvdata {
    regmap_config: &ak4642_regmap,
    extended_frequencies: 0,
};

static ak4643_drvdata: ak4642_drvdata = ak4642_drvdata {
    regmap_config: &ak4643_regmap,
    extended_frequencies: 0,
};

static ak4648_drvdata: ak4642_drvdata = ak4642_drvdata {
    regmap_config: &ak4648_regmap,
    extended_frequencies: 1,
};

/* CONFIG_COMMON_CLK */
unsafe extern "C" fn ak4642_of_parse_mcko(dev: *mut device) -> *mut clk {
    let np = (*dev).of_node;
    let mut clk: *mut clk;
    let mut clk_name: *const c_char = (*np).name;
    let mut parent_clk_name: *const c_char = ptr::null();
    let mut rate: u32 = 0;

    if of_property_read_u32(np, b"clock-frequency\0".as_ptr() as *const c_char, &mut rate) != 0 {
        return ptr::null_mut();
    }

    if of_property_read_bool(np, b"clocks\0".as_ptr() as *const c_char) {
        parent_clk_name = of_clk_get_parent_name(np, 0);
    }

    of_property_read_string(
        np,
        b"clock-output-names\0".as_ptr() as *const c_char,
        &mut clk_name,
    );

    clk = clk_register_fixed_rate(dev, clk_name, parent_clk_name, 0, rate);
    if !IS_ERR(clk as *const c_void) {
        of_clk_add_provider(np, &of_clk_src_simple_get as *const c_void, clk as *mut c_void);
    }

    clk
}
/* Without CONFIG_COMMON_CLK: ak4642_of_parse_mcko(d) maps to 0. */

unsafe extern "C" fn ak4642_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let dev = &mut (*i2c).dev as *mut device;
    let mut drvdata: *const ak4642_drvdata;
    let regmap: *mut regmap;
    let priv_: *mut ak4642_priv;
    let mut mcko: *mut clk = ptr::null_mut();

    if !dev_fwnode(dev).is_null() {
        mcko = ak4642_of_parse_mcko(dev);
        if IS_ERR(mcko as *const c_void) {
            mcko = ptr::null_mut();
        }
    }

    drvdata = i2c_get_match_data(i2c) as *const ak4642_drvdata;
    if drvdata.is_null() {
        return dev_err_probe(dev, -EINVAL, b"Unknown device type\n\0".as_ptr() as *const c_char);
    }

    priv_ = devm_kzalloc(dev, core::mem::size_of::<ak4642_priv>(), GFP_KERNEL) as *mut ak4642_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    (*priv_).drvdata = drvdata;
    (*priv_).mcko = mcko;

    i2c_set_clientdata(i2c, priv_ as *mut c_void);

    regmap = devm_regmap_init_i2c(i2c, (*drvdata).regmap_config);
    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }

    devm_snd_soc_register_component(dev, &soc_component_dev_ak4642, &mut ak4642_dai, 1)
}

static ak4642_of_match: [of_device_id; 4] = [
    of_device_id { compatible: b"asahi-kasei,ak4642\0".as_ptr() as *const c_char, data: &ak4642_drvdata as *const _ as *const c_void },
    of_device_id { compatible: b"asahi-kasei,ak4643\0".as_ptr() as *const c_char, data: &ak4643_drvdata as *const _ as *const c_void },
    of_device_id { compatible: b"asahi-kasei,ak4648\0".as_ptr() as *const c_char, data: &ak4648_drvdata as *const _ as *const c_void },
    of_device_id { compatible: ptr::null(), data: ptr::null() },
];
// MODULE_DEVICE_TABLE(of, ak4642_of_match);

static ak4642_i2c_id: [i2c_device_id; 4] = [
    i2c_device_id { name: b"ak4642\0".as_ptr() as *const c_char, driver_data: &ak4642_drvdata as *const _ as c_ulong },
    i2c_device_id { name: b"ak4643\0".as_ptr() as *const c_char, driver_data: &ak4643_drvdata as *const _ as c_ulong },
    i2c_device_id { name: b"ak4648\0".as_ptr() as *const c_char, driver_data: &ak4648_drvdata as *const _ as c_ulong },
    i2c_device_id { name: ptr::null(), driver_data: 0 },
];
// MODULE_DEVICE_TABLE(i2c, ak4642_i2c_id);

static mut ak4642_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"ak4642-codec\0".as_ptr() as *const c_char,
        of_match_table: ak4642_of_match.as_ptr(),
    },
    probe: Some(ak4642_i2c_probe),
    id_table: ak4642_i2c_id.as_ptr(),
};

// module_i2c_driver(ak4642_i2c_driver);
// MODULE_DESCRIPTION("Soc AK4642 driver");
// MODULE_AUTHOR("Kuninori Morimoto <morimoto.kuninori@renesas.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
