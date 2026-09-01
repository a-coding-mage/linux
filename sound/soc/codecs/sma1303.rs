// SPDX-License-Identifier: GPL-2.0-or-later
//
// sma1303.c -- sma1303 ALSA SoC Audio driver
//
// Copyright 2023 Iron Device Corporation
//
// Auther: Gyuhwa Park <gyuhwa.park@irondevice.com>
//         Kiseok Jo <kiseok.jo@irondevice.com>
//
// Rust translation of the original implementation source. Linux/ALSA/regmap
// declarations and register constants are external dependencies supplied by
// the surrounding repository, corresponding to the original includes:
// linux/module.h, moduleparam.h, kernel.h, init.h, delay.h, pm.h, i2c.h,
// regmap.h, slab.h, asm/div64.h, sound/core.h, pcm.h, pcm_params.h, soc.h,
// initval.h, tlv.h, and "sma1303.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::ptr;

const CHECK_PERIOD_TIME: c_long = 1; /* sec per HZ */
const MAX_CONTROL_NAME: usize = 48;

type ssize_t = isize;
type size_t = usize;
type bool_ = bool;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct kobject {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct attribute_group {
    pub attrs: *mut *mut attribute,
}
#[repr(C)]
pub struct attribute {
    _private: [u8; 0],
}
#[repr(C)]
pub struct delayed_work {
    pub work: work_struct,
}
#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
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
#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
    pub kcontrols: *mut *mut snd_kcontrol,
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}
#[repr(C)]
pub struct soc_enum {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget_init {
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
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
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
    pub id: c_int,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_init,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
}
#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub cache_type: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
}
#[repr(C)]
pub struct device_attribute {
    pub attr: attribute,
}
#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}
#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
}
#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}
#[repr(C)]
pub struct i2c_driver_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}
#[repr(C)]
pub struct i2c_driver {
    pub driver: i2c_driver_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
    pub id_table: *const i2c_device_id,
}

unsafe extern "C" {
    static mut system_freezable_wq: *mut c_void;
    static mut dev_attr_check_fault_period: device_attribute;
    static mut dev_attr_check_fault_status: device_attribute;

    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits_check(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint, change: *mut bool) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_dapm_kcontrol_get_value(kcontrol: *mut snd_kcontrol) -> c_uint;
    fn queue_delayed_work(wq: *mut c_void, work: *mut delayed_work, delay: c_long) -> bool;
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool;
    fn msleep(ms: c_uint);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_physical_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_sync(dapm: *mut snd_soc_dapm_context);
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> ssize_t;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn kstrtol(buf: *const c_char, base: c_uint, res: *mut c_long) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut c_void;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn sysfs_create_group(kobj: *mut kobject, grp: *mut attribute_group) -> c_int;
    fn sysfs_remove_group(kobj: *mut kobject, grp: *mut attribute_group);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_crit(dev: *mut device, fmt: *const c_char, ...);
}

unsafe extern "C" {
    static SMA1303_FF_DEVICE_INDEX: c_uint;
    static SMA1303_00_SYSTEM_CTRL: c_uint;
    static SMA1303_04_INPUT1_CTRL4: c_uint;
    static SMA1303_09_OUTPUT_CTRL: c_uint;
    static SMA1303_0A_SPK_VOL: c_uint;
    static SMA1303_0E_MUTE_VOL_CTRL: c_uint;
    static SMA1303_10_SYSTEM_CTRL1: c_uint;
    static SMA1303_11_SYSTEM_CTRL2: c_uint;
    static SMA1303_12_SYSTEM_CTRL3: c_uint;
    static SMA1303_14_MODULATOR: c_uint;
    static SMA1303_1B_BASS_SPK7: c_uint;
    static SMA1303_23_COMP_LIM1: c_uint;
    static SMA1303_26_COMP_LIM4: c_uint;
    static SMA1303_33_SDM_CTRL: c_uint;
    static SMA1303_34_OTP_DATA1: c_uint;
    static SMA1303_36_PROTECTION: c_uint;
    static SMA1303_37_SLOPE_CTRL: c_uint;
    static SMA1303_38_OTP_TRM0: c_uint;
    static SMA1303_3B_TEST1: c_uint;
    static SMA1303_3F_ATEST2: c_uint;
    static SMA1303_8B_PLL_POST_N: c_uint;
    static SMA1303_8C_PLL_N: c_uint;
    static SMA1303_8D_PLL_A_SETTING: c_uint;
    static SMA1303_8E_PLL_CTRL: c_uint;
    static SMA1303_8F_PLL_P_CP: c_uint;
    static SMA1303_90_POSTSCALER: c_uint;
    static SMA1303_92_FDPEC_CTRL: c_uint;
    static SMA1303_94_BOOST_CTRL1: c_uint;
    static SMA1303_97_BOOST_CTRL4: c_uint;
    static SMA1303_A0_PAD_CTRL0: c_uint;
    static SMA1303_A2_TOP_MAN1: c_uint;
    static SMA1303_A3_TOP_MAN2: c_uint;
    static SMA1303_A4_TOP_MAN3: c_uint;
    static SMA1303_A5_TDM1: c_uint;
    static SMA1303_A6_TDM2: c_uint;
    static SMA1303_A7_CLK_MON: c_uint;
    static SMA1303_FA_STATUS1: c_uint;
    static SMA1303_FB_STATUS2: c_uint;
    static SMA1303_PLL_PD2_MASK: c_uint;
    static SMA1303_PLL_OPERATION2: c_uint;
    static SMA1303_PLL_PD2: c_uint;
    static SMA1303_POWER_MASK: c_uint;
    static SMA1303_POWER_ON: c_uint;
    static SMA1303_POWER_OFF: c_uint;
    static SMA1303_MONO: c_uint;
    static SMA1303_STEREO: c_uint;
    static SMA1303_SPK_MODE_MASK: c_uint;
    static SMA1303_SPK_MONO: c_uint;
    static SMA1303_SPK_STEREO: c_uint;
    static SMA1303_SPK_OFF: c_uint;
    static SMA1303_MONOMIX_MASK: c_uint;
    static SMA1303_MONOMIX_ON: c_uint;
    static SMA1303_MONOMIX_OFF: c_uint;
    static SMA1303_LR_DATA_SW_MASK: c_uint;
    static SMA1303_LR_DATA_SW_NORMAL: c_uint;
    static SMA1303_LR_DATA_SW_SWAP: c_uint;
    static SMA1303_TEST_CLKO_EN_MASK: c_uint;
    static SMA1303_NORMAL_SDO: c_uint;
    static SMA1303_CLK_OUT_SDO: c_uint;
    static SMA1303_PORT_OUT_SEL_MASK: c_uint;
    static SMA1303_OUT_SEL_DISABLE: c_uint;
    static SMA1303_FORMAT_CONVERTER: c_uint;
    static SMA1303_MIXER_OUTPUT: c_uint;
    static SMA1303_SPEAKER_PATH: c_uint;
    static SMA1303_POSTSCALER_OUTPUT: c_uint;
    static SMA1303_MON_OSC_PLL_MASK: c_uint;
    static SMA1303_PLL_SDO: c_uint;
    static SMA1303_OSC_SDO: c_uint;
    static SMA1303_PORT_CONFIG_MASK: c_uint;
    static SMA1303_OUTPUT_PORT_ENABLE: c_uint;
    static SMA1303_INPUT_PORT_ONLY: c_uint;
    static SMA1303_SDO_OUTPUT_MASK: c_uint;
    static SMA1303_NORMAL_OUT: c_uint;
    static SMA1303_HIGH_Z_OUT: c_uint;
    static SMA1303_BYP_POST_MASK: c_uint;
    static SMA1303_EN_POST_SCALER: c_uint;
    static SMA1303_BYP_POST_SCALER: c_uint;
    static SMA1303_PLL_CLKIN_MCLK: c_uint;
    static SMA1303_PLL_CLKIN_BCLK: c_uint;
    static SMA1303_PLL_PD_MASK: c_uint;
    static SMA1303_PLL_REF_CLK_MASK: c_uint;
    static SMA1303_PLL_OPERATION: c_uint;
    static SMA1303_PLL_SCK: c_uint;
    static SMA1303_DAC_DN_CONV_MASK: c_uint;
    static SMA1303_DAC_DN_CONV_DISABLE: c_uint;
    static SMA1303_DAC_DN_CONV_ENABLE: c_uint;
    static SMA1303_01_INPUT1_CTRL1: c_uint;
    static SMA1303_LEFTPOL_MASK: c_uint;
    static SMA1303_LOW_FIRST_CH: c_uint;
    static SMA1303_HIGH_FIRST_CH: c_uint;
    static SMA1303_SCK_RATE_MASK: c_uint;
    static SMA1303_SCK_32FS: c_uint;
    static SMA1303_SCK_64FS: c_uint;
    static SMA1303_I2S_MODE_MASK: c_uint;
    static SMA1303_STANDARD_I2S: c_uint;
    static SMA1303_LJ: c_uint;
    static SMA1303_RJ_16BIT: c_uint;
    static SMA1303_RJ_24BIT: c_uint;
    static SMA1303_O_FORMAT_MASK: c_uint;
    static SMA1303_O_FMT_I2S: c_uint;
    static SMA1303_O_FMT_LJ: c_uint;
    static SMA1303_O_FMT_TDM: c_uint;
    static SMA1303_EXTERNAL_CLOCK_19_2: c_uint;
    static SMA1303_EXTERNAL_CLOCK_24_576: c_uint;
    static SMA1303_SPK_MUTE_MASK: c_uint;
    static SMA1303_SPK_MUTE: c_uint;
    static SMA1303_SPK_UNMUTE: c_uint;
    static SMA1303_CONTROLLER_DEVICE_MASK: c_uint;
    static SMA1303_DEVICE_MODE: c_uint;
    static SMA1303_CONTROLLER_MODE: c_uint;
    static SMA1303_SCK_RISING_MASK: c_uint;
    static SMA1303_SCK_RISING_EDGE: c_uint;
    static SMA1303_TDM_DL_MASK: c_uint;
    static SMA1303_TDM_DL_16: c_uint;
    static SMA1303_TDM_DL_32: c_uint;
    static SMA1303_TDM_N_SLOT_MASK: c_uint;
    static SMA1303_TDM_N_SLOT_4: c_uint;
    static SMA1303_TDM_N_SLOT_8: c_uint;
    static SMA1303_TDM_SLOT1_RX_POS_MASK: c_uint;
    static SMA1303_TDM_CLK_POL_MASK: c_uint;
    static SMA1303_TDM_CLK_POL_RISE: c_uint;
    static SMA1303_TDM_TX_MODE_MASK: c_uint;
    static SMA1303_TDM_TX_MONO: c_uint;
    static SMA1303_TDM_SLOT1_TX_POS_MASK: c_uint;
    static SMA1303_OT1_OK_STATUS: c_uint;
    static SMA1303_OT2_OK_STATUS: c_uint;
    static SMA1303_OCP_SPK_STATUS: c_uint;
    static SMA1303_OCP_BST_STATUS: c_uint;
    static SMA1303_CLK_MON_STATUS: c_uint;
    static SMA1303_UVLO_BST_STATUS: c_uint;
    static SMA1303_RESETBYI2C_MASK: c_uint;
    static SMA1303_RESETBYI2C_RESET: c_uint;
    static SMA1303_DEVICE_ID: c_uint;
    static SMA1303_REV_NUM_STATUS: c_uint;
    static SMA1303_REV_NUM_TV0: c_uint;
    static SMA1303_REV_NUM_TV1: c_uint;
    static SMA1303_OTP_STAT_OK_0: c_uint;
    static SMA1303_OTP_STAT_OK_1: c_uint;
    static SMA1303_I2C_RETRY_COUNT: c_int;
    static SND_SOC_DAPM_PRE_PMU: c_int;
    static SND_SOC_DAPM_POST_PMD: c_int;
    static SND_SOC_DAPM_POST_PMU: c_int;
    static SND_SOC_DAPM_PRE_PMD: c_int;
    static SND_SOC_NOPM: c_int;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_MASTER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static SNDRV_PCM_FORMAT_S16_LE: c_int;
    static SNDRV_PCM_FORMAT_S24_LE: c_int;
    static SNDRV_PCM_FORMAT_S32_LE: c_int;
    static SNDRV_PCM_RATE_8000_192000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_uint;
    static HZ: c_long;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static GFP_KERNEL: c_uint;
    static REGCACHE_NONE: c_uint;
}

#[repr(C)]
#[derive(Copy, Clone)]
enum sma1303_type {
    SMA1303,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sma1303_pll_match {
    input_clk_name: *mut c_char,
    output_clk_name: *mut c_char,
    input_clk: c_uint,
    post_n: c_uint,
    n: c_uint,
    vco: c_uint,
    p_cp: c_uint,
}

#[repr(C)]
struct sma1303_priv {
    devtype: sma1303_type,
    attr_grp: *mut attribute_group,
    check_fault_work: delayed_work,
    dev: *mut device,
    kobj: *mut kobject,
    regmap: *mut regmap,
    pll_matches: *mut sma1303_pll_match,
    amp_power_status: bool,
    force_mute_status: bool,
    num_of_pll_matches: c_int,
    retry_cnt: c_int,
    amp_mode: c_uint,
    cur_vol: c_uint,
    format: c_uint,
    frame_size: c_uint,
    init_vol: c_uint,
    last_bclk: c_uint,
    last_ocp_val: c_uint,
    last_over_temp: c_uint,
    rev_num: c_uint,
    sys_clk_id: c_uint,
    tdm_slot_rx: c_uint,
    tdm_slot_tx: c_uint,
    tsdw_cnt: c_uint,
    check_fault_period: c_long,
    check_fault_status: c_long,
}

static mut sma1303_pll_matches: [sma1303_pll_match; 7] = [
    sma1303_pll_match { input_clk_name: c"1.411MHz".as_ptr() as *mut c_char, output_clk_name: c"24.595MHz".as_ptr() as *mut c_char, input_clk: 1411200, post_n: 0x07, n: 0xF4, vco: 0x8B, p_cp: 0x03 },
    sma1303_pll_match { input_clk_name: c"1.536MHz".as_ptr() as *mut c_char, output_clk_name: c"24.576MHz".as_ptr() as *mut c_char, input_clk: 1536000, post_n: 0x07, n: 0xE0, vco: 0x8B, p_cp: 0x03 },
    sma1303_pll_match { input_clk_name: c"3.072MHz".as_ptr() as *mut c_char, output_clk_name: c"24.576MHz".as_ptr() as *mut c_char, input_clk: 3072000, post_n: 0x07, n: 0x70, vco: 0x8B, p_cp: 0x03 },
    sma1303_pll_match { input_clk_name: c"6.144MHz".as_ptr() as *mut c_char, output_clk_name: c"24.576MHz".as_ptr() as *mut c_char, input_clk: 6144000, post_n: 0x07, n: 0x70, vco: 0x8B, p_cp: 0x07 },
    sma1303_pll_match { input_clk_name: c"12.288MHz".as_ptr() as *mut c_char, output_clk_name: c"24.576MHz".as_ptr() as *mut c_char, input_clk: 12288000, post_n: 0x07, n: 0x70, vco: 0x8B, p_cp: 0x0B },
    sma1303_pll_match { input_clk_name: c"19.2MHz".as_ptr() as *mut c_char, output_clk_name: c"24.343MHz".as_ptr() as *mut c_char, input_clk: 19200000, post_n: 0x07, n: 0x47, vco: 0x8B, p_cp: 0x0A },
    sma1303_pll_match { input_clk_name: c"24.576MHz".as_ptr() as *mut c_char, output_clk_name: c"24.576MHz".as_ptr() as *mut c_char, input_clk: 24576000, post_n: 0x07, n: 0x70, vco: 0x8B, p_cp: 0x0F },
];

static sma1303_reg_def: [reg_default; 54] = [
    reg_default { reg: 0x00, def: 0x80 }, reg_default { reg: 0x01, def: 0x00 },
    reg_default { reg: 0x02, def: 0x00 }, reg_default { reg: 0x03, def: 0x11 },
    reg_default { reg: 0x04, def: 0x17 }, reg_default { reg: 0x09, def: 0x00 },
    reg_default { reg: 0x0A, def: 0x31 }, reg_default { reg: 0x0B, def: 0x98 },
    reg_default { reg: 0x0C, def: 0x84 }, reg_default { reg: 0x0D, def: 0x07 },
    reg_default { reg: 0x0E, def: 0x3F }, reg_default { reg: 0x10, def: 0x00 },
    reg_default { reg: 0x11, def: 0x00 }, reg_default { reg: 0x12, def: 0x00 },
    reg_default { reg: 0x14, def: 0x5C }, reg_default { reg: 0x15, def: 0x01 },
    reg_default { reg: 0x16, def: 0x0F }, reg_default { reg: 0x17, def: 0x0F },
    reg_default { reg: 0x18, def: 0x0F }, reg_default { reg: 0x19, def: 0x00 },
    reg_default { reg: 0x1A, def: 0x00 }, reg_default { reg: 0x1B, def: 0x00 },
    reg_default { reg: 0x23, def: 0x19 }, reg_default { reg: 0x24, def: 0x00 },
    reg_default { reg: 0x25, def: 0x00 }, reg_default { reg: 0x26, def: 0x04 },
    reg_default { reg: 0x33, def: 0x00 }, reg_default { reg: 0x36, def: 0x92 },
    reg_default { reg: 0x37, def: 0x27 }, reg_default { reg: 0x3B, def: 0x5A },
    reg_default { reg: 0x3C, def: 0x20 }, reg_default { reg: 0x3D, def: 0x00 },
    reg_default { reg: 0x3E, def: 0x03 }, reg_default { reg: 0x3F, def: 0x0C },
    reg_default { reg: 0x8B, def: 0x07 }, reg_default { reg: 0x8C, def: 0x70 },
    reg_default { reg: 0x8D, def: 0x8B }, reg_default { reg: 0x8E, def: 0x6F },
    reg_default { reg: 0x8F, def: 0x03 }, reg_default { reg: 0x90, def: 0x26 },
    reg_default { reg: 0x91, def: 0x42 }, reg_default { reg: 0x92, def: 0xE0 },
    reg_default { reg: 0x94, def: 0x35 }, reg_default { reg: 0x95, def: 0x0C },
    reg_default { reg: 0x96, def: 0x42 }, reg_default { reg: 0x97, def: 0x95 },
    reg_default { reg: 0xA0, def: 0x00 }, reg_default { reg: 0xA1, def: 0x3B },
    reg_default { reg: 0xA2, def: 0xC8 }, reg_default { reg: 0xA3, def: 0x28 },
    reg_default { reg: 0xA4, def: 0x40 }, reg_default { reg: 0xA5, def: 0x01 },
    reg_default { reg: 0xA6, def: 0x41 }, reg_default { reg: 0xA7, def: 0x00 },
];

unsafe extern "C" fn sma1303_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    if reg > SMA1303_FF_DEVICE_INDEX { return false; }
    ((reg >= SMA1303_00_SYSTEM_CTRL && reg <= SMA1303_04_INPUT1_CTRL4) ||
     (reg >= SMA1303_09_OUTPUT_CTRL && reg <= SMA1303_0E_MUTE_VOL_CTRL) ||
     (reg >= SMA1303_10_SYSTEM_CTRL1 && reg <= SMA1303_12_SYSTEM_CTRL3) ||
     (reg >= SMA1303_14_MODULATOR && reg <= SMA1303_1B_BASS_SPK7) ||
     (reg >= SMA1303_23_COMP_LIM1 && reg <= SMA1303_26_COMP_LIM4) ||
     (reg >= SMA1303_33_SDM_CTRL && reg <= SMA1303_34_OTP_DATA1) ||
     (reg >= SMA1303_36_PROTECTION && reg <= SMA1303_38_OTP_TRM0) ||
     (reg >= SMA1303_3B_TEST1 && reg <= SMA1303_3F_ATEST2) ||
     (reg >= SMA1303_8B_PLL_POST_N && reg <= SMA1303_92_FDPEC_CTRL) ||
     (reg >= SMA1303_94_BOOST_CTRL1 && reg <= SMA1303_97_BOOST_CTRL4) ||
     (reg >= SMA1303_A0_PAD_CTRL0 && reg <= SMA1303_A7_CLK_MON) ||
     (reg >= SMA1303_FA_STATUS1 && reg <= SMA1303_FB_STATUS2) ||
     reg == SMA1303_FF_DEVICE_INDEX)
}

unsafe extern "C" fn sma1303_writeable_register(_dev: *mut device, reg: c_uint) -> bool {
    if reg > SMA1303_FF_DEVICE_INDEX { return false; }
    ((reg >= SMA1303_00_SYSTEM_CTRL && reg <= SMA1303_04_INPUT1_CTRL4) ||
     (reg >= SMA1303_09_OUTPUT_CTRL && reg <= SMA1303_0E_MUTE_VOL_CTRL) ||
     (reg >= SMA1303_10_SYSTEM_CTRL1 && reg <= SMA1303_12_SYSTEM_CTRL3) ||
     (reg >= SMA1303_14_MODULATOR && reg <= SMA1303_1B_BASS_SPK7) ||
     (reg >= SMA1303_23_COMP_LIM1 && reg <= SMA1303_26_COMP_LIM4) ||
     reg == SMA1303_33_SDM_CTRL ||
     (reg >= SMA1303_36_PROTECTION && reg <= SMA1303_37_SLOPE_CTRL) ||
     (reg >= SMA1303_3B_TEST1 && reg <= SMA1303_3F_ATEST2) ||
     (reg >= SMA1303_8B_PLL_POST_N && reg <= SMA1303_92_FDPEC_CTRL) ||
     (reg >= SMA1303_94_BOOST_CTRL1 && reg <= SMA1303_97_BOOST_CTRL4) ||
     (reg >= SMA1303_A0_PAD_CTRL0 && reg <= SMA1303_A7_CLK_MON))
}

unsafe extern "C" fn sma1303_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    (reg >= SMA1303_FA_STATUS1 && reg <= SMA1303_FB_STATUS2) || reg == SMA1303_FF_DEVICE_INDEX
}

// static const DECLARE_TLV_DB_SCALE(sma1303_spk_tlv, -6000, 50, 0);
static sma1303_spk_tlv: [c_uint; 4] = [0, (-6000i32) as c_uint, 50, 0];

unsafe fn neg_errno(errno: c_int) -> c_int { -errno }

unsafe extern "C" fn sma1303_regmap_write(sma1303: *mut sma1303_priv, reg: c_uint, val: c_uint) -> c_int {
    let mut ret: c_int = 0;
    let mut cnt: c_int = (*sma1303).retry_cnt;
    while cnt != 0 {
        cnt -= 1;
        ret = regmap_write((*sma1303).regmap, reg, val);
        if ret < 0 {
            dev_err((*sma1303).dev, c"Failed to write [0x%02X]\n".as_ptr(), reg);
        } else { break; }
    }
    ret
}

unsafe extern "C" fn sma1303_regmap_update_bits(sma1303: *mut sma1303_priv, reg: c_uint, mask: c_uint, val: c_uint, change: *mut bool) -> c_int {
    let mut ret: c_int = 0;
    let mut cnt: c_int = (*sma1303).retry_cnt;
    while cnt != 0 {
        cnt -= 1;
        ret = regmap_update_bits_check((*sma1303).regmap, reg, mask, val, change);
        if ret < 0 {
            dev_err((*sma1303).dev, c"Failed to update [0x%02X]\n".as_ptr(), reg);
        } else { break; }
    }
    ret
}

unsafe extern "C" fn sma1303_regmap_read(sma1303: *mut sma1303_priv, reg: c_uint, val: *mut c_uint) -> c_int {
    let mut ret: c_int = 0;
    let mut cnt: c_int = (*sma1303).retry_cnt;
    while cnt != 0 {
        cnt -= 1;
        ret = regmap_read((*sma1303).regmap, reg, val);
        if ret < 0 {
            dev_err((*sma1303).dev, c"Failed to read [0x%02X]\n".as_ptr(), reg);
        } else { break; }
    }
    ret
}

static sma1303_aif_in_source_text: [*const c_char; 3] = [c"Mono".as_ptr(), c"Left".as_ptr(), c"Right".as_ptr()];
static sma1303_aif_out_source_text: [*const c_char; 7] = [
    c"Disable".as_ptr(), c"After_FmtC".as_ptr(), c"After_Mixer".as_ptr(),
    c"After_DSP".as_ptr(), c"After_Post".as_ptr(), c"Clk_PLL".as_ptr(), c"Clk_OSC".as_ptr(),
];
static sma1303_tdm_slot_text: [*const c_char; 8] = [
    c"Slot0".as_ptr(), c"Slot1".as_ptr(), c"Slot2".as_ptr(), c"Slot3".as_ptr(),
    c"Slot4".as_ptr(), c"Slot5".as_ptr(), c"Slot6".as_ptr(), c"Slot7".as_ptr(),
];

// SOC_ENUM_SINGLE_EXT declarations depend on ALSA macro layout.
static sma1303_aif_in_source_enum: soc_enum = soc_enum { _private: [] };
static sma1303_aif_out_source_enum: soc_enum = soc_enum { _private: [] };
static sma1303_tdm_slot_enum: soc_enum = soc_enum { _private: [] };

unsafe extern "C" fn sma1303_force_mute_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let sma1303 = snd_soc_component_get_drvdata(component) as *mut sma1303_priv;
    (*ucontrol).value.integer.value[0] = (*sma1303).force_mute_status as c_int as c_long;
    dev_dbg((*sma1303).dev, c"%s : Force Mute %s\n".as_ptr(), c"sma1303_force_mute_get".as_ptr(), if (*sma1303).force_mute_status { c"ON".as_ptr() } else { c"OFF".as_ptr() });
    0
}

unsafe extern "C" fn sma1303_force_mute_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let sma1303 = snd_soc_component_get_drvdata(component) as *mut sma1303_priv;
    let mut change = false;
    let val = (*ucontrol).value.integer.value[0] != 0;
    if (*sma1303).force_mute_status == val {
        change = false;
    } else {
        change = true;
        (*sma1303).force_mute_status = val;
    }
    dev_dbg((*sma1303).dev, c"%s : Force Mute %s\n".as_ptr(), c"sma1303_force_mute_put".as_ptr(), if (*sma1303).force_mute_status { c"ON".as_ptr() } else { c"OFF".as_ptr() });
    change as c_int
}

unsafe extern "C" fn sma1303_postscaler_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let sma1303 = snd_soc_component_get_drvdata(component) as *mut sma1303_priv;
    let mut val: c_uint = 0;
    let ret = sma1303_regmap_read(sma1303, SMA1303_90_POSTSCALER, &mut val);
    if ret < 0 { return neg_errno(EINVAL); }
    (*ucontrol).value.integer.value[0] = ((val & 0x7E) >> 1) as c_long;
    0
}

unsafe extern "C" fn sma1303_postscaler_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let sma1303 = snd_soc_component_get_drvdata(component) as *mut sma1303_priv;
    let val = (*ucontrol).value.integer.value[0] as c_int;
    let mut change = false;
    let ret = sma1303_regmap_update_bits(sma1303, SMA1303_90_POSTSCALER, 0x7E, (val << 1) as c_uint, &mut change);
    if ret < 0 { return neg_errno(EINVAL); }
    change as c_int
}

unsafe extern "C" fn sma1303_tdm_slot_rx_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let sma1303 = snd_soc_component_get_drvdata(component) as *mut sma1303_priv;
    let mut val: c_uint = 0;
    let ret = sma1303_regmap_read(sma1303, SMA1303_A5_TDM1, &mut val);
    if ret < 0 { return neg_errno(EINVAL); }
    (*ucontrol).value.integer.value[0] = ((val & 0x38) >> 3) as c_long;
    (*sma1303).tdm_slot_rx = (*ucontrol).value.integer.value[0] as c_uint;
    0
}

unsafe extern "C" fn sma1303_tdm_slot_rx_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let sma1303 = snd_soc_component_get_drvdata(component) as *mut sma1303_priv;
    let val = (*ucontrol).value.integer.value[0] as c_int;
    let mut change = false;
    let ret = sma1303_regmap_update_bits(sma1303, SMA1303_A5_TDM1, 0x38, (val << 3) as c_uint, &mut change);
    if ret < 0 { return neg_errno(EINVAL); }
    change as c_int
}

unsafe extern "C" fn sma1303_tdm_slot_tx_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let sma1303 = snd_soc_component_get_drvdata(component) as *mut sma1303_priv;
    let mut val: c_uint = 0;
    let ret = sma1303_regmap_read(sma1303, SMA1303_A6_TDM2, &mut val);
    if ret < 0 { return neg_errno(EINVAL); }
    (*ucontrol).value.integer.value[0] = ((val & 0x38) >> 3) as c_long;
    (*sma1303).tdm_slot_tx = (*ucontrol).value.integer.value[0] as c_uint;
    0
}

unsafe extern "C" fn sma1303_tdm_slot_tx_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let sma1303 = snd_soc_component_get_drvdata(component) as *mut sma1303_priv;
    let val = (*ucontrol).value.integer.value[0] as c_int;
    let mut change = false;
    let ret = sma1303_regmap_update_bits(sma1303, SMA1303_A6_TDM2, 0x38, (val << 3) as c_uint, &mut change);
    if ret < 0 { return neg_errno(EINVAL); }
    change as c_int
}

unsafe extern "C" fn sma1303_startup(component: *mut snd_soc_component) -> c_int {
    let sma1303 = snd_soc_component_get_drvdata(component) as *mut sma1303_priv;
    let mut change = false;
    let mut temp = false;
    sma1303_regmap_update_bits(sma1303, SMA1303_8E_PLL_CTRL, SMA1303_PLL_PD2_MASK, SMA1303_PLL_OPERATION2, &mut temp);
    if temp { change = true; }
    sma1303_regmap_update_bits(sma1303, SMA1303_00_SYSTEM_CTRL, SMA1303_POWER_MASK, SMA1303_POWER_ON, &mut temp);
    if temp { change = true; }
    if (*sma1303).amp_mode == SMA1303_MONO {
        sma1303_regmap_update_bits(sma1303, SMA1303_10_SYSTEM_CTRL1, SMA1303_SPK_MODE_MASK, SMA1303_SPK_MONO, &mut temp);
    } else {
        sma1303_regmap_update_bits(sma1303, SMA1303_10_SYSTEM_CTRL1, SMA1303_SPK_MODE_MASK, SMA1303_SPK_STEREO, &mut temp);
    }
    if temp { change = true; }
    if (*sma1303).check_fault_status != 0 {
        let delay = if (*sma1303).check_fault_period > 0 { (*sma1303).check_fault_period * HZ } else { CHECK_PERIOD_TIME * HZ };
        queue_delayed_work(system_freezable_wq, &mut (*sma1303).check_fault_work, delay);
    }
    (*sma1303).amp_power_status = true;
    change as c_int
}

unsafe extern "C" fn sma1303_shutdown(component: *mut snd_soc_component) -> c_int {
    let sma1303 = snd_soc_component_get_drvdata(component) as *mut sma1303_priv;
    let mut change = false;
    let mut temp = false;
    cancel_delayed_work_sync(&mut (*sma1303).check_fault_work);
    sma1303_regmap_update_bits(sma1303, SMA1303_10_SYSTEM_CTRL1, SMA1303_SPK_MODE_MASK, SMA1303_SPK_OFF, &mut temp);
    if temp { change = true; }
    sma1303_regmap_update_bits(sma1303, SMA1303_00_SYSTEM_CTRL, SMA1303_POWER_MASK, SMA1303_POWER_OFF, &mut temp);
    if temp { change = true; }
    sma1303_regmap_update_bits(sma1303, SMA1303_8E_PLL_CTRL, SMA1303_PLL_PD2_MASK, SMA1303_PLL_PD2, &mut temp);
    if temp { change = true; }
    (*sma1303).amp_power_status = false;
    change as c_int
}

unsafe fn add_update(ret: &mut c_int, sma1303: *mut sma1303_priv, reg: c_uint, mask: c_uint, val: c_uint, change: *mut bool) {
    *ret += sma1303_regmap_update_bits(sma1303, reg, mask, val, change);
}

unsafe extern "C" fn sma1303_aif_in_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let sma1303 = snd_soc_component_get_drvdata(component) as *mut sma1303_priv;
    let mux = snd_soc_dapm_kcontrol_get_value(*(*w).kcontrols);
    let mut ret = 0;
    let mut change = false;
    let mut temp = false;
    if event == SND_SOC_DAPM_PRE_PMU {
        match mux {
            0 => { add_update(&mut ret, sma1303, SMA1303_11_SYSTEM_CTRL2, SMA1303_MONOMIX_MASK, SMA1303_MONOMIX_ON, &mut change); (*sma1303).amp_mode = SMA1303_MONO; }
            1 => {
                add_update(&mut ret, sma1303, SMA1303_11_SYSTEM_CTRL2, SMA1303_MONOMIX_MASK, SMA1303_MONOMIX_OFF, &mut temp); if temp { change = true; }
                add_update(&mut ret, sma1303, SMA1303_11_SYSTEM_CTRL2, SMA1303_LR_DATA_SW_MASK, SMA1303_LR_DATA_SW_NORMAL, &mut temp); if temp { change = true; }
                (*sma1303).amp_mode = SMA1303_STEREO;
            }
            2 => {
                add_update(&mut ret, sma1303, SMA1303_11_SYSTEM_CTRL2, SMA1303_MONOMIX_MASK, SMA1303_MONOMIX_OFF, &mut temp); if temp { change = true; }
                add_update(&mut ret, sma1303, SMA1303_11_SYSTEM_CTRL2, SMA1303_LR_DATA_SW_MASK, SMA1303_LR_DATA_SW_SWAP, &mut temp); if temp { change = true; }
                (*sma1303).amp_mode = SMA1303_STEREO;
            }
            _ => { dev_err((*sma1303).dev, c"%s : Invalid value (%d)\n".as_ptr(), c"sma1303_aif_in_event".as_ptr(), mux); return neg_errno(EINVAL); }
        }
        dev_dbg((*sma1303).dev, c"%s : Source : %s\n".as_ptr(), c"sma1303_aif_in_event".as_ptr(), sma1303_aif_in_source_text[mux as usize]);
    }
    if ret < 0 { return neg_errno(EINVAL); }
    change as c_int
}

unsafe extern "C" fn sma1303_aif_out_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let sma1303 = snd_soc_component_get_drvdata(component) as *mut sma1303_priv;
    let mux = snd_soc_dapm_kcontrol_get_value(*(*w).kcontrols);
    let mut ret = 0;
    let mut change = false;
    let mut temp = false;
    if event == SND_SOC_DAPM_PRE_PMU {
        match mux {
            0 => { add_update(&mut ret, sma1303, SMA1303_A3_TOP_MAN2, SMA1303_TEST_CLKO_EN_MASK, SMA1303_NORMAL_SDO, &mut temp); if temp { change = true; } add_update(&mut ret, sma1303, SMA1303_09_OUTPUT_CTRL, SMA1303_PORT_OUT_SEL_MASK, SMA1303_OUT_SEL_DISABLE, &mut temp); if temp { change = true; } }
            1 => { add_update(&mut ret, sma1303, SMA1303_A3_TOP_MAN2, SMA1303_TEST_CLKO_EN_MASK, SMA1303_NORMAL_SDO, &mut temp); if temp { change = true; } add_update(&mut ret, sma1303, SMA1303_09_OUTPUT_CTRL, SMA1303_PORT_OUT_SEL_MASK, SMA1303_FORMAT_CONVERTER, &mut temp); if temp { change = true; } }
            2 => { add_update(&mut ret, sma1303, SMA1303_A3_TOP_MAN2, SMA1303_TEST_CLKO_EN_MASK, SMA1303_NORMAL_SDO, &mut temp); if temp { change = true; } add_update(&mut ret, sma1303, SMA1303_09_OUTPUT_CTRL, SMA1303_PORT_OUT_SEL_MASK, SMA1303_MIXER_OUTPUT, &mut temp); if temp { change = true; } }
            3 => { add_update(&mut ret, sma1303, SMA1303_A3_TOP_MAN2, SMA1303_TEST_CLKO_EN_MASK, SMA1303_NORMAL_SDO, &mut temp); if temp { change = true; } add_update(&mut ret, sma1303, SMA1303_09_OUTPUT_CTRL, SMA1303_PORT_OUT_SEL_MASK, SMA1303_SPEAKER_PATH, &mut temp); if temp { change = true; } }
            4 => { add_update(&mut ret, sma1303, SMA1303_A3_TOP_MAN2, SMA1303_TEST_CLKO_EN_MASK, SMA1303_NORMAL_SDO, &mut temp); if temp { change = true; } add_update(&mut ret, sma1303, SMA1303_09_OUTPUT_CTRL, SMA1303_PORT_OUT_SEL_MASK, SMA1303_POSTSCALER_OUTPUT, &mut temp); if temp { change = true; } }
            5 => { add_update(&mut ret, sma1303, SMA1303_A3_TOP_MAN2, SMA1303_TEST_CLKO_EN_MASK, SMA1303_CLK_OUT_SDO, &mut temp); if temp { change = true; } add_update(&mut ret, sma1303, SMA1303_A3_TOP_MAN2, SMA1303_MON_OSC_PLL_MASK, SMA1303_PLL_SDO, &mut temp); if temp { change = true; } }
            6 => { add_update(&mut ret, sma1303, SMA1303_A3_TOP_MAN2, SMA1303_TEST_CLKO_EN_MASK, SMA1303_CLK_OUT_SDO, &mut temp); if temp { change = true; } add_update(&mut ret, sma1303, SMA1303_A3_TOP_MAN2, SMA1303_MON_OSC_PLL_MASK, SMA1303_OSC_SDO, &mut temp); if temp { change = true; } }
            _ => { dev_err((*sma1303).dev, c"%s : Invalid value (%d)\n".as_ptr(), c"sma1303_aif_out_event".as_ptr(), mux); return neg_errno(EINVAL); }
        }
        dev_dbg((*sma1303).dev, c"%s : Source : %s\n".as_ptr(), c"sma1303_aif_out_event".as_ptr(), sma1303_aif_out_source_text[mux as usize]);
    }
    if ret < 0 { return neg_errno(EINVAL); }
    change as c_int
}

unsafe extern "C" fn sma1303_sdo_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let sma1303 = snd_soc_component_get_drvdata(component) as *mut sma1303_priv;
    let mut ret = 0;
    let mut change = false;
    let mut temp = false;
    if event == SND_SOC_DAPM_PRE_PMU {
        dev_dbg((*sma1303).dev, c"%s : SND_SOC_DAPM_PRE_PMU\n".as_ptr(), c"sma1303_sdo_event".as_ptr());
        add_update(&mut ret, sma1303, SMA1303_09_OUTPUT_CTRL, SMA1303_PORT_CONFIG_MASK, SMA1303_OUTPUT_PORT_ENABLE, &mut temp);
        if temp { change = true; }
        add_update(&mut ret, sma1303, SMA1303_A3_TOP_MAN2, SMA1303_SDO_OUTPUT_MASK, SMA1303_NORMAL_OUT, &mut temp);
        if temp { change = true; }
    } else if event == SND_SOC_DAPM_POST_PMD {
        dev_dbg((*sma1303).dev, c"%s : SND_SOC_DAPM_POST_PMD\n".as_ptr(), c"sma1303_sdo_event".as_ptr());
        add_update(&mut ret, sma1303, SMA1303_09_OUTPUT_CTRL, SMA1303_PORT_CONFIG_MASK, SMA1303_INPUT_PORT_ONLY, &mut temp);
        if temp { change = true; }
        add_update(&mut ret, sma1303, SMA1303_A3_TOP_MAN2, SMA1303_SDO_OUTPUT_MASK, SMA1303_HIGH_Z_OUT, &mut temp);
        if temp { change = true; }
    }
    if ret < 0 { return neg_errno(EINVAL); }
    change as c_int
}

unsafe extern "C" fn sma1303_post_scaler_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let sma1303 = snd_soc_component_get_drvdata(component) as *mut sma1303_priv;
    let mut ret = 0;
    let mut change = false;
    if event == SND_SOC_DAPM_PRE_PMU {
        dev_dbg((*sma1303).dev, c"%s : SND_SOC_DAPM_PRE_PMU\n".as_ptr(), c"sma1303_post_scaler_event".as_ptr());
        add_update(&mut ret, sma1303, SMA1303_90_POSTSCALER, SMA1303_BYP_POST_MASK, SMA1303_EN_POST_SCALER, &mut change);
    } else if event == SND_SOC_DAPM_POST_PMD {
        dev_dbg((*sma1303).dev, c"%s : SND_SOC_DAPM_POST_PMD\n".as_ptr(), c"sma1303_post_scaler_event".as_ptr());
        add_update(&mut ret, sma1303, SMA1303_90_POSTSCALER, SMA1303_BYP_POST_MASK, SMA1303_BYP_POST_SCALER, &mut change);
    }
    if ret < 0 { return neg_errno(EINVAL); }
    change as c_int
}

unsafe extern "C" fn sma1303_power_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let sma1303 = snd_soc_component_get_drvdata(component) as *mut sma1303_priv;
    let mut ret = 0;
    if event == SND_SOC_DAPM_POST_PMU {
        dev_dbg((*sma1303).dev, c"%s : SND_SOC_DAPM_POST_PMU\n".as_ptr(), c"sma1303_power_event".as_ptr());
        ret = sma1303_startup(component);
    } else if event == SND_SOC_DAPM_PRE_PMD {
        dev_dbg((*sma1303).dev, c"%s : SND_SOC_DAPM_PRE_PMD\n".as_ptr(), c"sma1303_power_event".as_ptr());
        ret = sma1303_shutdown(component);
    }
    ret
}

// ALSA control/widget macros translated as preserved declarations. Their exact
// struct initializers are generated by SOC_* and SND_SOC_DAPM_* macros.
static sma1303_aif_in_source_control: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static sma1303_aif_out_source_control: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static sma1303_sdo_control: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static sma1303_post_scaler_control: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static sma1303_enable_control: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static sma1303_snd_controls: [snd_kcontrol_new; 5] = [
    snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];
static sma1303_dapm_widgets: [snd_soc_dapm_widget_init; 11] = [
    snd_soc_dapm_widget_init { _private: [] }, snd_soc_dapm_widget_init { _private: [] },
    snd_soc_dapm_widget_init { _private: [] }, snd_soc_dapm_widget_init { _private: [] },
    snd_soc_dapm_widget_init { _private: [] }, snd_soc_dapm_widget_init { _private: [] },
    snd_soc_dapm_widget_init { _private: [] }, snd_soc_dapm_widget_init { _private: [] },
    snd_soc_dapm_widget_init { _private: [] }, snd_soc_dapm_widget_init { _private: [] },
    snd_soc_dapm_widget_init { _private: [] },
];

static sma1303_audio_map: [snd_soc_dapm_route; 22] = [
    snd_soc_dapm_route { sink: c"AIF IN Source".as_ptr(), control: c"Mono".as_ptr(), source: c"AIF IN".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF IN Source".as_ptr(), control: c"Left".as_ptr(), source: c"AIF IN".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF IN Source".as_ptr(), control: c"Right".as_ptr(), source: c"AIF IN".as_ptr() },
    snd_soc_dapm_route { sink: c"SDO Enable".as_ptr(), control: c"Switch".as_ptr(), source: c"AIF IN".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF OUT Source".as_ptr(), control: c"Disable".as_ptr(), source: c"SDO Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF OUT Source".as_ptr(), control: c"After_FmtC".as_ptr(), source: c"SDO Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF OUT Source".as_ptr(), control: c"After_Mixer".as_ptr(), source: c"SDO Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF OUT Source".as_ptr(), control: c"After_DSP".as_ptr(), source: c"SDO Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF OUT Source".as_ptr(), control: c"After_Post".as_ptr(), source: c"SDO Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF OUT Source".as_ptr(), control: c"Clk_PLL".as_ptr(), source: c"SDO Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF OUT Source".as_ptr(), control: c"Clk_OSC".as_ptr(), source: c"SDO Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"Entry".as_ptr(), control: ptr::null(), source: c"AIF OUT Source".as_ptr() },
    snd_soc_dapm_route { sink: c"Entry".as_ptr(), control: ptr::null(), source: c"AIF IN Source".as_ptr() },
    snd_soc_dapm_route { sink: c"Post Scaler".as_ptr(), control: c"Switch".as_ptr(), source: c"Entry".as_ptr() },
    snd_soc_dapm_route { sink: c"AMP Power".as_ptr(), control: ptr::null(), source: c"Entry".as_ptr() },
    snd_soc_dapm_route { sink: c"AMP Power".as_ptr(), control: ptr::null(), source: c"Entry".as_ptr() },
    snd_soc_dapm_route { sink: c"AMP Enable".as_ptr(), control: c"Switch".as_ptr(), source: c"AMP Power".as_ptr() },
    snd_soc_dapm_route { sink: c"SPK".as_ptr(), control: ptr::null(), source: c"AMP Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF OUT".as_ptr(), control: ptr::null(), source: c"AMP Enable".as_ptr() },
    snd_soc_dapm_route { sink: ptr::null(), control: ptr::null(), source: ptr::null() },
    snd_soc_dapm_route { sink: ptr::null(), control: ptr::null(), source: ptr::null() },
    snd_soc_dapm_route { sink: ptr::null(), control: ptr::null(), source: ptr::null() },
];

unsafe extern "C" fn sma1303_setup_pll(component: *mut snd_soc_component, bclk: c_uint) -> c_int {
    let sma1303 = snd_soc_component_get_drvdata(component) as *mut sma1303_priv;
    let mut i: c_int = 0;
    let mut ret: c_int = 0;
    dev_dbg((*component).dev, c"%s : BCLK = %dHz\n".as_ptr(), c"sma1303_setup_pll".as_ptr(), bclk);
    if (*sma1303).sys_clk_id == SMA1303_PLL_CLKIN_MCLK {
        dev_dbg((*component).dev, c"%s : MCLK is not supported\n".as_ptr(), c"sma1303_setup_pll".as_ptr());
    } else if (*sma1303).sys_clk_id == SMA1303_PLL_CLKIN_BCLK {
        while i < (*sma1303).num_of_pll_matches {
            if (*(*sma1303).pll_matches.add(i as usize)).input_clk == bclk { break; }
            i += 1;
        }
        if i == (*sma1303).num_of_pll_matches {
            dev_dbg((*component).dev, c"%s : No matching value between pll table and SCK\n".as_ptr(), c"sma1303_setup_pll".as_ptr());
            return neg_errno(EINVAL);
        }
        ret += sma1303_regmap_update_bits(sma1303, SMA1303_A2_TOP_MAN1, SMA1303_PLL_PD_MASK | SMA1303_PLL_REF_CLK_MASK, SMA1303_PLL_OPERATION | SMA1303_PLL_SCK, ptr::null_mut());
    }
    ret += sma1303_regmap_write(sma1303, SMA1303_8B_PLL_POST_N, (*(*sma1303).pll_matches.add(i as usize)).post_n);
    ret += sma1303_regmap_write(sma1303, SMA1303_8C_PLL_N, (*(*sma1303).pll_matches.add(i as usize)).n);
    ret += sma1303_regmap_write(sma1303, SMA1303_8D_PLL_A_SETTING, (*(*sma1303).pll_matches.add(i as usize)).vco);
    ret += sma1303_regmap_write(sma1303, SMA1303_8F_PLL_P_CP, (*(*sma1303).pll_matches.add(i as usize)).p_cp);
    if ret < 0 { return neg_errno(EINVAL); }
    0
}

unsafe extern "C" fn sma1303_dai_hw_params_amp(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let sma1303 = snd_soc_component_get_drvdata(component) as *mut sma1303_priv;
    let bclk: c_uint = if (*sma1303).format == SND_SOC_DAIFMT_DSP_A {
        params_rate(params) * (*sma1303).frame_size
    } else {
        params_rate(params) * params_physical_width(params) * params_channels(params)
    };
    let mut ret = 0;
    dev_dbg((*component).dev, c"%s : rate = %d : bit size = %d : channel = %d\n".as_ptr(), c"sma1303_dai_hw_params_amp".as_ptr(), params_rate(params), params_width(params), params_channels(params));
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        if (*sma1303).sys_clk_id == SMA1303_PLL_CLKIN_BCLK && (*sma1303).last_bclk != bclk {
            sma1303_setup_pll(component, bclk);
            (*sma1303).last_bclk = bclk;
        }
        match params_rate(params) {
            8000 | 12000 | 16000 | 24000 | 32000 | 44100 | 48000 | 96000 => {
                ret += sma1303_regmap_update_bits(sma1303, SMA1303_A2_TOP_MAN1, SMA1303_DAC_DN_CONV_MASK, SMA1303_DAC_DN_CONV_DISABLE, ptr::null_mut());
                ret += sma1303_regmap_update_bits(sma1303, SMA1303_01_INPUT1_CTRL1, SMA1303_LEFTPOL_MASK, SMA1303_LOW_FIRST_CH, ptr::null_mut());
            }
            192000 => {
                ret += sma1303_regmap_update_bits(sma1303, SMA1303_A2_TOP_MAN1, SMA1303_DAC_DN_CONV_MASK, SMA1303_DAC_DN_CONV_ENABLE, ptr::null_mut());
                ret += sma1303_regmap_update_bits(sma1303, SMA1303_01_INPUT1_CTRL1, SMA1303_LEFTPOL_MASK, SMA1303_HIGH_FIRST_CH, ptr::null_mut());
            }
            _ => { dev_err((*component).dev, c"%s not support rate : %d\n".as_ptr(), c"sma1303_dai_hw_params_amp".as_ptr(), params_rate(params)); return neg_errno(EINVAL); }
        }
    } else {
        match params_format(params) {
            x if x == SNDRV_PCM_FORMAT_S16_LE => { dev_dbg((*component).dev, c"%s set format SNDRV_PCM_FORMAT_S16_LE\n".as_ptr(), c"sma1303_dai_hw_params_amp".as_ptr()); ret += sma1303_regmap_update_bits(sma1303, SMA1303_A4_TOP_MAN3, SMA1303_SCK_RATE_MASK, SMA1303_SCK_32FS, ptr::null_mut()); }
            x if x == SNDRV_PCM_FORMAT_S24_LE => { dev_dbg((*component).dev, c"%s set format SNDRV_PCM_FORMAT_S24_LE\n".as_ptr(), c"sma1303_dai_hw_params_amp".as_ptr()); ret += sma1303_regmap_update_bits(sma1303, SMA1303_A4_TOP_MAN3, SMA1303_SCK_RATE_MASK, SMA1303_SCK_64FS, ptr::null_mut()); }
            x if x == SNDRV_PCM_FORMAT_S32_LE => { dev_dbg((*component).dev, c"%s set format SNDRV_PCM_FORMAT_S32_LE\n".as_ptr(), c"sma1303_dai_hw_params_amp".as_ptr()); ret += sma1303_regmap_update_bits(sma1303, SMA1303_A4_TOP_MAN3, SMA1303_SCK_RATE_MASK, SMA1303_SCK_64FS, ptr::null_mut()); }
            _ => { dev_err((*component).dev, c"%s not support data bit : %d\n".as_ptr(), c"sma1303_dai_hw_params_amp".as_ptr(), params_format(params)); return neg_errno(EINVAL); }
        }
    }
    if (*sma1303).format == SND_SOC_DAIFMT_I2S {
        ret += sma1303_regmap_update_bits(sma1303, SMA1303_01_INPUT1_CTRL1, SMA1303_I2S_MODE_MASK, SMA1303_STANDARD_I2S, ptr::null_mut());
        ret += sma1303_regmap_update_bits(sma1303, SMA1303_A4_TOP_MAN3, SMA1303_O_FORMAT_MASK, SMA1303_O_FMT_I2S, ptr::null_mut());
    } else if (*sma1303).format == SND_SOC_DAIFMT_LEFT_J {
        ret += sma1303_regmap_update_bits(sma1303, SMA1303_01_INPUT1_CTRL1, SMA1303_I2S_MODE_MASK, SMA1303_LJ, ptr::null_mut());
        ret += sma1303_regmap_update_bits(sma1303, SMA1303_A4_TOP_MAN3, SMA1303_O_FORMAT_MASK, SMA1303_O_FMT_LJ, ptr::null_mut());
    } else if (*sma1303).format == SND_SOC_DAIFMT_RIGHT_J {
        match params_width(params) {
            16 => ret += sma1303_regmap_update_bits(sma1303, SMA1303_01_INPUT1_CTRL1, SMA1303_I2S_MODE_MASK, SMA1303_RJ_16BIT, ptr::null_mut()),
            24 | 32 => ret += sma1303_regmap_update_bits(sma1303, SMA1303_01_INPUT1_CTRL1, SMA1303_I2S_MODE_MASK, SMA1303_RJ_24BIT, ptr::null_mut()),
            _ => {}
        }
    } else if (*sma1303).format == SND_SOC_DAIFMT_DSP_A {
        ret += sma1303_regmap_update_bits(sma1303, SMA1303_01_INPUT1_CTRL1, SMA1303_I2S_MODE_MASK, SMA1303_STANDARD_I2S, ptr::null_mut());
        ret += sma1303_regmap_update_bits(sma1303, SMA1303_A4_TOP_MAN3, SMA1303_O_FORMAT_MASK, SMA1303_O_FMT_TDM, ptr::null_mut());
    }
    match params_width(params) {
        16 | 24 | 32 => {}
        _ => { dev_err((*component).dev, c"%s not support data bit : %d\n".as_ptr(), c"sma1303_dai_hw_params_amp".as_ptr(), params_format(params)); return neg_errno(EINVAL); }
    }
    if ret < 0 { return neg_errno(EINVAL); }
    0
}

unsafe extern "C" fn sma1303_dai_set_sysclk_amp(dai: *mut snd_soc_dai, clk_id: c_int, _freq: c_uint, _dir: c_int) -> c_int {
    let component = (*dai).component;
    let sma1303 = snd_soc_component_get_drvdata(component) as *mut sma1303_priv;
    let id = clk_id as c_uint;
    if id == SMA1303_EXTERNAL_CLOCK_19_2 || id == SMA1303_EXTERNAL_CLOCK_24_576 || id == SMA1303_PLL_CLKIN_MCLK || id == SMA1303_PLL_CLKIN_BCLK {
        (*sma1303).sys_clk_id = clk_id as c_uint;
        0
    } else {
        dev_err((*component).dev, c"Invalid clk id: %d\n".as_ptr(), clk_id);
        neg_errno(EINVAL)
    }
}

unsafe extern "C" fn sma1303_dai_mute(dai: *mut snd_soc_dai, mute: c_int, stream: c_int) -> c_int {
    let component = (*dai).component;
    let sma1303 = snd_soc_component_get_drvdata(component) as *mut sma1303_priv;
    let mut ret = 0;
    if stream == SNDRV_PCM_STREAM_CAPTURE { return ret; }
    if mute != 0 {
        dev_dbg((*component).dev, c"%s : %s\n".as_ptr(), c"sma1303_dai_mute".as_ptr(), c"MUTE".as_ptr());
        ret += sma1303_regmap_update_bits(sma1303, SMA1303_0E_MUTE_VOL_CTRL, SMA1303_SPK_MUTE_MASK, SMA1303_SPK_MUTE, ptr::null_mut());
        /* Need to wait time for mute slope */
        msleep(55);
    } else if !(*sma1303).force_mute_status {
        dev_dbg((*component).dev, c"%s : %s\n".as_ptr(), c"sma1303_dai_mute".as_ptr(), c"UNMUTE".as_ptr());
        ret += sma1303_regmap_update_bits(sma1303, SMA1303_0E_MUTE_VOL_CTRL, SMA1303_SPK_MUTE_MASK, SMA1303_SPK_UNMUTE, ptr::null_mut());
    } else {
        dev_dbg((*sma1303).dev, c"%s : FORCE MUTE!!!\n".as_ptr(), c"sma1303_dai_mute".as_ptr());
    }
    if ret < 0 { return neg_errno(EINVAL); }
    0
}

unsafe extern "C" fn sma1303_dai_set_fmt_amp(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let sma1303 = snd_soc_component_get_drvdata(component) as *mut sma1303_priv;
    let mut ret = 0;
    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        x if x == SND_SOC_DAIFMT_CBC_CFC => {
            dev_dbg((*component).dev, c"%s : %s\n".as_ptr(), c"sma1303_dai_set_fmt_amp".as_ptr(), c"I2S/TDM Device mode".as_ptr());
            ret += sma1303_regmap_update_bits(sma1303, SMA1303_01_INPUT1_CTRL1, SMA1303_CONTROLLER_DEVICE_MASK, SMA1303_DEVICE_MODE, ptr::null_mut());
        }
        x if x == SND_SOC_DAIFMT_CBP_CFP => {
            dev_dbg((*component).dev, c"%s : %s\n".as_ptr(), c"sma1303_dai_set_fmt_amp".as_ptr(), c"I2S/TDM Controller mode".as_ptr());
            ret += sma1303_regmap_update_bits(sma1303, SMA1303_01_INPUT1_CTRL1, SMA1303_CONTROLLER_DEVICE_MASK, SMA1303_CONTROLLER_MODE, ptr::null_mut());
        }
        _ => { dev_err((*component).dev, c"Unsupported Controller/Device : 0x%x\n".as_ptr(), fmt); return neg_errno(EINVAL); }
    }
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S || x == SND_SOC_DAIFMT_RIGHT_J || x == SND_SOC_DAIFMT_LEFT_J || x == SND_SOC_DAIFMT_DSP_A || x == SND_SOC_DAIFMT_DSP_B => {
            (*sma1303).format = fmt & SND_SOC_DAIFMT_FORMAT_MASK;
        }
        _ => { dev_err((*component).dev, c"Unsupported Audio Interface Format : 0x%x\n".as_ptr(), fmt); return neg_errno(EINVAL); }
    }
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_IB_NF => {
            dev_dbg((*component).dev, c"%s : %s\n".as_ptr(), c"sma1303_dai_set_fmt_amp".as_ptr(), c"Invert BCLK + Normal Frame".as_ptr());
            ret += sma1303_regmap_update_bits(sma1303, SMA1303_01_INPUT1_CTRL1, SMA1303_SCK_RISING_MASK, SMA1303_SCK_RISING_EDGE, ptr::null_mut());
        }
        x if x == SND_SOC_DAIFMT_IB_IF => {
            dev_dbg((*component).dev, c"%s : %s\n".as_ptr(), c"sma1303_dai_set_fmt_amp".as_ptr(), c"Invert BCLK + Invert Frame".as_ptr());
            ret += sma1303_regmap_update_bits(sma1303, SMA1303_01_INPUT1_CTRL1, SMA1303_LEFTPOL_MASK | SMA1303_SCK_RISING_MASK, SMA1303_HIGH_FIRST_CH | SMA1303_SCK_RISING_EDGE, ptr::null_mut());
        }
        x if x == SND_SOC_DAIFMT_NB_IF => {
            dev_dbg((*component).dev, c"%s : %s\n".as_ptr(), c"sma1303_dai_set_fmt_amp".as_ptr(), c"Normal BCLK + Invert Frame".as_ptr());
            ret += sma1303_regmap_update_bits(sma1303, SMA1303_01_INPUT1_CTRL1, SMA1303_LEFTPOL_MASK, SMA1303_HIGH_FIRST_CH, ptr::null_mut());
        }
        x if x == SND_SOC_DAIFMT_NB_NF => {
            dev_dbg((*component).dev, c"%s : %s\n".as_ptr(), c"sma1303_dai_set_fmt_amp".as_ptr(), c"Normal BCLK + Normal Frame".as_ptr());
        }
        _ => { dev_err((*component).dev, c"Unsupported Bit & Frameclock : 0x%x\n".as_ptr(), fmt); return neg_errno(EINVAL); }
    }
    if ret < 0 { return neg_errno(EINVAL); }
    0
}

unsafe extern "C" fn sma1303_dai_set_tdm_slot(dai: *mut snd_soc_dai, _tx_mask: c_uint, _rx_mask: c_uint, slots: c_int, slot_width: c_int) -> c_int {
    let component = (*dai).component;
    let sma1303 = snd_soc_component_get_drvdata(component) as *mut sma1303_priv;
    let mut ret = 0;
    dev_dbg((*component).dev, c"%s : slots = %d, slot_width - %d\n".as_ptr(), c"sma1303_dai_set_tdm_slot".as_ptr(), slots, slot_width);
    (*sma1303).frame_size = (slot_width * slots) as c_uint;
    ret += sma1303_regmap_update_bits(sma1303, SMA1303_A4_TOP_MAN3, SMA1303_O_FORMAT_MASK, SMA1303_O_FMT_TDM, ptr::null_mut());
    match slot_width {
        16 => ret += sma1303_regmap_update_bits(sma1303, SMA1303_A6_TDM2, SMA1303_TDM_DL_MASK, SMA1303_TDM_DL_16, ptr::null_mut()),
        32 => ret += sma1303_regmap_update_bits(sma1303, SMA1303_A6_TDM2, SMA1303_TDM_DL_MASK, SMA1303_TDM_DL_32, ptr::null_mut()),
        _ => dev_err((*component).dev, c"%s not support TDM %d slot_width\n".as_ptr(), c"sma1303_dai_set_tdm_slot".as_ptr(), slot_width),
    }
    match slots {
        4 => ret += sma1303_regmap_update_bits(sma1303, SMA1303_A6_TDM2, SMA1303_TDM_N_SLOT_MASK, SMA1303_TDM_N_SLOT_4, ptr::null_mut()),
        8 => ret += sma1303_regmap_update_bits(sma1303, SMA1303_A6_TDM2, SMA1303_TDM_N_SLOT_MASK, SMA1303_TDM_N_SLOT_8, ptr::null_mut()),
        _ => dev_err((*component).dev, c"%s not support TDM %d slots\n".as_ptr(), c"sma1303_dai_set_tdm_slot".as_ptr(), slots),
    }
    if (*sma1303).tdm_slot_rx < slots as c_uint {
        ret += sma1303_regmap_update_bits(sma1303, SMA1303_A5_TDM1, SMA1303_TDM_SLOT1_RX_POS_MASK, (*sma1303).tdm_slot_rx << 3, ptr::null_mut());
    } else {
        dev_err((*component).dev, c"%s Incorrect tdm-slot-rx %d set\n".as_ptr(), c"sma1303_dai_set_tdm_slot".as_ptr(), (*sma1303).tdm_slot_rx);
    }
    ret += sma1303_regmap_update_bits(sma1303, SMA1303_A5_TDM1, SMA1303_TDM_CLK_POL_MASK, SMA1303_TDM_CLK_POL_RISE, ptr::null_mut());
    ret += sma1303_regmap_update_bits(sma1303, SMA1303_A5_TDM1, SMA1303_TDM_TX_MODE_MASK, SMA1303_TDM_TX_MONO, ptr::null_mut());
    if (*sma1303).tdm_slot_tx < slots as c_uint {
        ret += sma1303_regmap_update_bits(sma1303, SMA1303_A6_TDM2, SMA1303_TDM_SLOT1_TX_POS_MASK, (*sma1303).tdm_slot_tx << 3, ptr::null_mut());
    } else {
        dev_err((*component).dev, c"%s Incorrect tdm-slot-tx %d set\n".as_ptr(), c"sma1303_dai_set_tdm_slot".as_ptr(), (*sma1303).tdm_slot_tx);
    }
    if ret < 0 { return neg_errno(EINVAL); }
    0
}

static sma1303_dai_ops_amp: snd_soc_dai_ops = snd_soc_dai_ops {
    set_sysclk: Some(sma1303_dai_set_sysclk_amp),
    set_fmt: Some(sma1303_dai_set_fmt_amp),
    hw_params: Some(sma1303_dai_hw_params_amp),
    mute_stream: Some(sma1303_dai_mute),
    set_tdm_slot: Some(sma1303_dai_set_tdm_slot),
};

unsafe fn SMA1303_RATES() -> c_uint { SNDRV_PCM_RATE_8000_192000 }
unsafe fn SMA1303_FORMATS() -> c_uint { SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE }

static mut sma1303_dai: [snd_soc_dai_driver; 1] = [
    snd_soc_dai_driver {
        name: c"sma1303-amplifier".as_ptr(),
        id: 0,
        playback: snd_soc_pcm_stream {
            stream_name: c"Playback".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: 0, /* SMA1303_RATES */
            formats: 0, /* SMA1303_FORMATS */
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"Capture".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: 0, /* SMA1303_RATES */
            formats: 0, /* SMA1303_FORMATS */
        },
        ops: &sma1303_dai_ops_amp,
    },
];

unsafe extern "C" fn sma1303_check_fault_worker(work: *mut work_struct) {
    let sma1303 = (work as *mut u8).sub(core::mem::offset_of!(sma1303_priv, check_fault_work) + core::mem::offset_of!(delayed_work, work)) as *mut sma1303_priv;
    let mut ret = 0;
    let mut over_temp: c_uint = 0;
    let mut ocp_val: c_uint = 0;
    let mut uvlo_val: c_uint = 0;
    if (*sma1303).tsdw_cnt != 0 {
        ret = sma1303_regmap_read(sma1303, SMA1303_0A_SPK_VOL, &mut (*sma1303).cur_vol);
    } else {
        ret = sma1303_regmap_read(sma1303, SMA1303_0A_SPK_VOL, &mut (*sma1303).init_vol);
    }
    if ret != 0 { dev_err((*sma1303).dev, c"failed to read SMA1303_0A_SPK_VOL : %d\n".as_ptr(), ret); return; }
    ret = sma1303_regmap_read(sma1303, SMA1303_FA_STATUS1, &mut over_temp);
    if ret != 0 { dev_err((*sma1303).dev, c"failed to read SMA1303_FA_STATUS1 : %d\n".as_ptr(), ret); return; }
    ret = sma1303_regmap_read(sma1303, SMA1303_FB_STATUS2, &mut ocp_val);
    if ret != 0 { dev_err((*sma1303).dev, c"failed to read SMA1303_FB_STATUS2 : %d\n".as_ptr(), ret); return; }
    ret = sma1303_regmap_read(sma1303, SMA1303_FF_DEVICE_INDEX, &mut uvlo_val);
    if ret != 0 { dev_err((*sma1303).dev, c"failed to read SMA1303_FF_DEVICE_INDEX : %d\n".as_ptr(), ret); return; }
    if ((!over_temp) & SMA1303_OT1_OK_STATUS) != 0 {
        dev_crit((*sma1303).dev, c"%s : OT1(Over Temperature Level 1)\n".as_ptr(), c"sma1303_check_fault_worker".as_ptr());
        if (*sma1303).cur_vol + 6 <= 0xFF {
            sma1303_regmap_write(sma1303, SMA1303_0A_SPK_VOL, (*sma1303).cur_vol + 6);
        }
        (*sma1303).tsdw_cnt += 1;
    } else if (*sma1303).tsdw_cnt != 0 {
        sma1303_regmap_write(sma1303, SMA1303_0A_SPK_VOL, (*sma1303).init_vol);
        (*sma1303).tsdw_cnt = 0;
        (*sma1303).cur_vol = (*sma1303).init_vol;
    }
    if ((!over_temp) & SMA1303_OT2_OK_STATUS) != 0 { dev_crit((*sma1303).dev, c"%s : OT2(Over Temperature Level 2)\n".as_ptr(), c"sma1303_check_fault_worker".as_ptr()); }
    if (ocp_val & SMA1303_OCP_SPK_STATUS) != 0 { dev_crit((*sma1303).dev, c"%s : OCP_SPK(Over Current Protect SPK)\n".as_ptr(), c"sma1303_check_fault_worker".as_ptr()); }
    if (ocp_val & SMA1303_OCP_BST_STATUS) != 0 { dev_crit((*sma1303).dev, c"%s : OCP_BST(Over Current Protect Boost)\n".as_ptr(), c"sma1303_check_fault_worker".as_ptr()); }
    if (ocp_val & SMA1303_CLK_MON_STATUS) != 0 && (*sma1303).amp_power_status { dev_crit((*sma1303).dev, c"%s : CLK_FAULT(No clock input)\n".as_ptr(), c"sma1303_check_fault_worker".as_ptr()); }
    if (uvlo_val & SMA1303_UVLO_BST_STATUS) != 0 { dev_crit((*sma1303).dev, c"%s : UVLO(Under Voltage Lock Out)\n".as_ptr(), c"sma1303_check_fault_worker".as_ptr()); }
    if over_temp != (*sma1303).last_over_temp || ocp_val != (*sma1303).last_ocp_val {
        dev_crit((*sma1303).dev, c"Please check AMP status".as_ptr());
        dev_dbg((*sma1303).dev, c"STATUS1=0x%02X : STATUS2=0x%02X\n".as_ptr(), over_temp, ocp_val);
        (*sma1303).last_over_temp = over_temp;
        (*sma1303).last_ocp_val = ocp_val;
    }
    if (*sma1303).check_fault_status != 0 {
        let delay = if (*sma1303).check_fault_period > 0 { (*sma1303).check_fault_period * HZ } else { CHECK_PERIOD_TIME * HZ };
        queue_delayed_work(system_freezable_wq, &mut (*sma1303).check_fault_work, delay);
    }
    if ((!((!over_temp) & SMA1303_OT1_OK_STATUS != 0))
        && (!((!over_temp) & SMA1303_OT2_OK_STATUS != 0))
        && (!(ocp_val & SMA1303_OCP_SPK_STATUS != 0))
        && (!(ocp_val & SMA1303_OCP_BST_STATUS != 0))
        && (!(ocp_val & SMA1303_CLK_MON_STATUS != 0))
        && (!(uvlo_val & SMA1303_UVLO_BST_STATUS != 0))) {
    }
}

unsafe extern "C" fn sma1303_probe(component: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    snd_soc_dapm_sync(dapm);
    0
}

unsafe extern "C" fn sma1303_remove(component: *mut snd_soc_component) {
    let sma1303 = snd_soc_component_get_drvdata(component) as *mut sma1303_priv;
    cancel_delayed_work_sync(&mut (*sma1303).check_fault_work);
}

static sma1303_component: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(sma1303_probe),
    remove: Some(sma1303_remove),
    controls: sma1303_snd_controls.as_ptr(),
    num_controls: sma1303_snd_controls.len() as c_uint,
    dapm_widgets: sma1303_dapm_widgets.as_ptr(),
    num_dapm_widgets: sma1303_dapm_widgets.len() as c_uint,
    dapm_routes: sma1303_audio_map.as_ptr(),
    num_dapm_routes: sma1303_audio_map.len() as c_uint,
};

static sma_i2c_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: 0xFF, /* SMA1303_FF_DEVICE_INDEX */
    readable_reg: Some(sma1303_readable_register),
    writeable_reg: Some(sma1303_writeable_register),
    volatile_reg: Some(sma1303_volatile_register),
    cache_type: 0, /* REGCACHE_NONE */
    reg_defaults: sma1303_reg_def.as_ptr(),
    num_reg_defaults: sma1303_reg_def.len() as c_uint,
};

unsafe extern "C" fn check_fault_period_show(dev: *mut device, _devattr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let sma1303 = dev_get_drvdata(dev) as *mut sma1303_priv;
    sysfs_emit(buf, c"%ld\n".as_ptr(), (*sma1303).check_fault_period)
}

unsafe extern "C" fn check_fault_period_store(dev: *mut device, _devattr: *mut device_attribute, buf: *const c_char, count: size_t) -> ssize_t {
    let sma1303 = dev_get_drvdata(dev) as *mut sma1303_priv;
    let ret = kstrtol(buf, 10, &mut (*sma1303).check_fault_period);
    if ret != 0 { return neg_errno(EINVAL) as ssize_t; }
    count as ssize_t
}

// static DEVICE_ATTR_RW(check_fault_period);

unsafe extern "C" fn check_fault_status_show(dev: *mut device, _devattr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let sma1303 = dev_get_drvdata(dev) as *mut sma1303_priv;
    sysfs_emit(buf, c"%ld\n".as_ptr(), (*sma1303).check_fault_status)
}

unsafe extern "C" fn check_fault_status_store(dev: *mut device, _devattr: *mut device_attribute, buf: *const c_char, count: size_t) -> ssize_t {
    let sma1303 = dev_get_drvdata(dev) as *mut sma1303_priv;
    let ret = kstrtol(buf, 10, &mut (*sma1303).check_fault_status);
    if ret != 0 { return neg_errno(EINVAL) as ssize_t; }
    if (*sma1303).check_fault_status != 0 {
        let delay = if (*sma1303).check_fault_period > 0 { (*sma1303).check_fault_period * HZ } else { CHECK_PERIOD_TIME * HZ };
        queue_delayed_work(system_freezable_wq, &mut (*sma1303).check_fault_work, delay);
    }
    count as ssize_t
}

// static DEVICE_ATTR_RW(check_fault_status);

static mut sma1303_attr: [*mut attribute; 3] = [
    ptr::null_mut(), /* &dev_attr_check_fault_period.attr */
    ptr::null_mut(), /* &dev_attr_check_fault_status.attr */
    ptr::null_mut(),
];

static mut sma1303_attr_group: attribute_group = attribute_group {
    attrs: unsafe { sma1303_attr.as_mut_ptr() },
};

unsafe extern "C" fn sma1303_i2c_probe(client: *mut i2c_client) -> c_int {
    let mut ret: c_int;
    let mut i: c_uint = 0;
    let mut device_info: c_uint = 0;
    let mut status: c_uint = 0;
    let mut otp_stat: c_uint = 0;
    let sma1303 = devm_kzalloc(&mut (*client).dev, core::mem::size_of::<sma1303_priv>(), GFP_KERNEL) as *mut sma1303_priv;
    if sma1303.is_null() { return neg_errno(ENOMEM); }
    (*sma1303).dev = &mut (*client).dev;
    (*sma1303).regmap = devm_regmap_init_i2c(client, &sma_i2c_regmap);
    if IS_ERR((*sma1303).regmap as *const c_void) {
        ret = PTR_ERR((*sma1303).regmap as *const c_void);
        dev_err(&mut (*client).dev, c"Failed to allocate register map: %d\n".as_ptr(), ret);
        return ret;
    }
    ret = sma1303_regmap_read(sma1303, SMA1303_FF_DEVICE_INDEX, &mut device_info);
    if ret != 0 || ((device_info & 0xF8) != SMA1303_DEVICE_ID) {
        dev_err(&mut (*client).dev, c"device initialization error (%d 0x%02X)".as_ptr(), ret, device_info);
    }
    dev_dbg(&mut (*client).dev, c"chip version 0x%02X\n".as_ptr(), device_info);
    ret += sma1303_regmap_update_bits(sma1303, SMA1303_00_SYSTEM_CTRL, SMA1303_RESETBYI2C_MASK, SMA1303_RESETBYI2C_RESET, ptr::null_mut());
    ret += sma1303_regmap_read(sma1303, SMA1303_FF_DEVICE_INDEX, &mut status);
    (*sma1303).rev_num = status & SMA1303_REV_NUM_STATUS;
    if (*sma1303).rev_num == SMA1303_REV_NUM_TV0 {
        dev_dbg(&mut (*client).dev, c"SMA1303 Trimming Version 0\n".as_ptr());
    } else if (*sma1303).rev_num == SMA1303_REV_NUM_TV1 {
        dev_dbg(&mut (*client).dev, c"SMA1303 Trimming Version 1\n".as_ptr());
    }
    ret += sma1303_regmap_read(sma1303, SMA1303_FB_STATUS2, &mut otp_stat);
    if ret < 0 {
        dev_err(&mut (*client).dev, c"failed to read, register: %02X, ret: %d\n".as_ptr(), SMA1303_FF_DEVICE_INDEX, ret);
    }
    if (((*sma1303).rev_num == SMA1303_REV_NUM_TV0) && ((otp_stat & 0x0E) == SMA1303_OTP_STAT_OK_0))
        || (((*sma1303).rev_num != SMA1303_REV_NUM_TV0) && ((otp_stat & 0x0C) == SMA1303_OTP_STAT_OK_1)) {
        dev_dbg(&mut (*client).dev, c"SMA1303 OTP Status Successful\n".as_ptr());
    } else {
        dev_dbg(&mut (*client).dev, c"SMA1303 OTP Status Fail\n".as_ptr());
    }
    while i < sma1303_reg_def.len() as c_uint {
        ret += sma1303_regmap_write(sma1303, sma1303_reg_def[i as usize].reg, sma1303_reg_def[i as usize].def);
        i += 1;
    }
    (*sma1303).amp_mode = SMA1303_MONO;
    (*sma1303).amp_power_status = false;
    (*sma1303).check_fault_period = CHECK_PERIOD_TIME;
    (*sma1303).check_fault_status = true as c_long;
    (*sma1303).force_mute_status = false;
    (*sma1303).init_vol = 0x31;
    (*sma1303).cur_vol = (*sma1303).init_vol;
    (*sma1303).last_bclk = 0;
    (*sma1303).last_ocp_val = 0x08;
    (*sma1303).last_over_temp = 0xC0;
    (*sma1303).tsdw_cnt = 0;
    (*sma1303).retry_cnt = SMA1303_I2C_RETRY_COUNT;
    (*sma1303).tdm_slot_rx = 0;
    (*sma1303).tdm_slot_tx = 0;
    (*sma1303).sys_clk_id = SMA1303_PLL_CLKIN_BCLK;
    (*sma1303).dev = &mut (*client).dev;
    (*sma1303).kobj = ptr::null_mut(); /* &client->dev.kobj, layout supplied externally */
    // INIT_DELAYED_WORK(&sma1303->check_fault_work, sma1303_check_fault_worker);
    i2c_set_clientdata(client, sma1303 as *mut c_void);
    (*sma1303).pll_matches = sma1303_pll_matches.as_mut_ptr();
    (*sma1303).num_of_pll_matches = sma1303_pll_matches.len() as c_int;
    ret = devm_snd_soc_register_component(&mut (*client).dev, &sma1303_component, sma1303_dai.as_mut_ptr(), 1);
    if ret != 0 {
        dev_err(&mut (*client).dev, c"Failed to register component".as_ptr());
        return ret;
    }
    (*sma1303).attr_grp = &mut sma1303_attr_group;
    ret = sysfs_create_group((*sma1303).kobj, (*sma1303).attr_grp);
    if ret != 0 {
        dev_err(&mut (*client).dev, c"failed to create attribute group [%d]\n".as_ptr(), ret);
        (*sma1303).attr_grp = ptr::null_mut();
    }
    ret
}

unsafe extern "C" fn sma1303_i2c_remove(client: *mut i2c_client) {
    let sma1303 = i2c_get_clientdata(client) as *mut sma1303_priv;
    if !(*sma1303).attr_grp.is_null() {
        sysfs_remove_group((*sma1303).kobj, (*sma1303).attr_grp);
    }
    cancel_delayed_work_sync(&mut (*sma1303).check_fault_work);
}

static sma1303_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: [b's' as c_char, b'm' as c_char, b'a' as c_char, b'1' as c_char, b'3' as c_char, b'0' as c_char, b'3' as c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] },
    i2c_device_id { name: [0; 20] },
];
// MODULE_DEVICE_TABLE(i2c, sma1303_i2c_id);

static sma1303_of_match: [of_device_id; 2] = [
    of_device_id { compatible: c"irondevice,sma1303".as_ptr() },
    of_device_id { compatible: ptr::null() },
];
// MODULE_DEVICE_TABLE(of, sma1303_of_match);

static mut sma1303_i2c_driver: i2c_driver = i2c_driver {
    driver: i2c_driver_driver {
        name: c"sma1303".as_ptr(),
        of_match_table: sma1303_of_match.as_ptr(),
    },
    probe: Some(sma1303_i2c_probe),
    remove: Some(sma1303_i2c_remove),
    id_table: sma1303_i2c_id.as_ptr(),
};

// module_i2c_driver(sma1303_i2c_driver);
// MODULE_DESCRIPTION("ALSA SoC SMA1303 driver");
// MODULE_AUTHOR("Gyuhwa Park, <gyuhwa.park@irondevice.com>");
// MODULE_AUTHOR("Kiseok Jo, <kiseok.jo@irondevice.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
