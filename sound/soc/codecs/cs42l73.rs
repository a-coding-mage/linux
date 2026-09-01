// SPDX-License-Identifier: GPL-2.0-only
/*
 * cs42l73.rs  --  CS42L73 ALSA Soc Audio driver
 *
 * Copyright 2011 Cirrus Logic, Inc.
 *
 * Authors: Georgi Vlaev, Nucleus Systems Ltd, <joe@nucleusys.com>
 *          Brian Austin, Cirrus Logic Inc, <brian.austin@cirrus.com>
 *
 * Source-level Rust translation of cs42l73.c. Linux/ASoC/regmap symbols,
 * constants, table-building macros, and module macros are external
 * dependencies supplied by the surrounding kernel crate/bindings.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct i2c_client {
    pub dev: device_with_of,
}
#[repr(C)]
pub struct device_with_of {
    pub of_node: *mut c_void,
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub id: c_int,
    pub name: *const c_char,
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut c_void,
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
pub struct soc_enum {
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
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *const c_uint,
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_tristate: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int) -> c_int>,
}
#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub id: c_int,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
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
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}
#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub cache_type: c_uint,
    pub use_single_read: bool,
    pub use_single_write: bool,
}
#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}
#[repr(C)]
pub struct i2c_device_id {
    pub name: *const c_char,
}
#[repr(C)]
pub struct driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}
#[repr(C)]
pub struct i2c_driver {
    pub driver: driver_inner,
    pub id_table: *const i2c_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
}

pub type u8_ = u8;
pub type u32_ = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs42l73_platform_data {
    /* RST GPIO */
    pub reset_gpio: *mut gpio_desc,
    pub chgfreq: c_uint,
    pub jack_detection: c_int,
    pub mclk_freq: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sp_config {
    pub spc: u8,
    pub mmcc: u8,
    pub spfs: u8,
    pub srate: u32,
}

#[repr(C)]
pub struct cs42l73_private {
    pub pdata: cs42l73_platform_data,
    pub config: [sp_config; 3],
    pub regmap: *mut regmap,
    pub sysclk: u32,
    pub mclksel: u8,
    pub mclk: u32,
    pub shutdwn_delay: c_int,
}

pub const EINVAL: c_int = 22;
pub const ENOMEM: c_int = 12;
pub const ENODEV: c_int = 19;

macro_rules! ARRAY_SIZE {
    ($a:expr) => {
        ($a.len() as c_uint)
    };
}

static cs42l73_reg_defaults: &[reg_default] = &[
    reg_default { reg: 6, def: 0xF1 }, /* r06 - Power Ctl 1 */
    reg_default { reg: 7, def: 0xDF }, /* r07 - Power Ctl 2 */
    reg_default { reg: 8, def: 0x3F }, /* r08 - Power Ctl 3 */
    reg_default { reg: 9, def: 0x50 }, /* r09 - Charge Pump Freq */
    reg_default { reg: 10, def: 0x53 }, /* r0A - Output Load MicBias Short Detect */
    reg_default { reg: 11, def: 0x00 }, /* r0B - DMIC Master Clock Ctl */
    reg_default { reg: 12, def: 0x00 }, /* r0C - Aux PCM Ctl */
    reg_default { reg: 13, def: 0x15 }, /* r0D - Aux PCM Master Clock Ctl */
    reg_default { reg: 14, def: 0x00 }, /* r0E - Audio PCM Ctl */
    reg_default { reg: 15, def: 0x15 }, /* r0F - Audio PCM Master Clock Ctl */
    reg_default { reg: 16, def: 0x00 }, /* r10 - Voice PCM Ctl */
    reg_default { reg: 17, def: 0x15 }, /* r11 - Voice PCM Master Clock Ctl */
    reg_default { reg: 18, def: 0x00 }, /* r12 - Voice/Aux Sample Rate */
    reg_default { reg: 19, def: 0x06 }, /* r13 - Misc I/O Path Ctl */
    reg_default { reg: 20, def: 0x00 }, /* r14 - ADC Input Path Ctl */
    reg_default { reg: 21, def: 0x00 }, /* r15 - MICA Preamp, PGA Volume */
    reg_default { reg: 22, def: 0x00 }, /* r16 - MICB Preamp, PGA Volume */
    reg_default { reg: 23, def: 0x00 }, /* r17 - Input Path A Digital Volume */
    reg_default { reg: 24, def: 0x00 }, /* r18 - Input Path B Digital Volume */
    reg_default { reg: 25, def: 0x00 }, /* r19 - Playback Digital Ctl */
    reg_default { reg: 26, def: 0x00 }, /* r1A - HP/LO Left Digital Volume */
    reg_default { reg: 27, def: 0x00 }, /* r1B - HP/LO Right Digital Volume */
    reg_default { reg: 28, def: 0x00 }, /* r1C - Speakerphone Digital Volume */
    reg_default { reg: 29, def: 0x00 }, /* r1D - Ear/SPKLO Digital Volume */
    reg_default { reg: 30, def: 0x00 }, /* r1E - HP Left Analog Volume */
    reg_default { reg: 31, def: 0x00 }, /* r1F - HP Right Analog Volume */
    reg_default { reg: 32, def: 0x00 }, /* r20 - LO Left Analog Volume */
    reg_default { reg: 33, def: 0x00 }, /* r21 - LO Right Analog Volume */
    reg_default { reg: 34, def: 0x00 }, /* r22 - Stereo Input Path Advisory Volume */
    reg_default { reg: 35, def: 0x00 }, /* r23 - Aux PCM Input Advisory Volume */
    reg_default { reg: 36, def: 0x00 }, /* r24 - Audio PCM Input Advisory Volume */
    reg_default { reg: 37, def: 0x00 }, /* r25 - Voice PCM Input Advisory Volume */
    reg_default { reg: 38, def: 0x00 }, /* r26 - Limiter Attack Rate HP/LO */
    reg_default { reg: 39, def: 0x7F }, /* r27 - Limter Ctl, Release Rate HP/LO */
    reg_default { reg: 40, def: 0x00 }, /* r28 - Limter Threshold HP/LO */
    reg_default { reg: 41, def: 0x00 }, /* r29 - Limiter Attack Rate Speakerphone */
    reg_default { reg: 42, def: 0x3F }, /* r2A - Limter Ctl, Release Rate Speakerphone */
    reg_default { reg: 43, def: 0x00 }, /* r2B - Limter Threshold Speakerphone */
    reg_default { reg: 44, def: 0x00 }, /* r2C - Limiter Attack Rate Ear/SPKLO */
    reg_default { reg: 45, def: 0x3F }, /* r2D - Limter Ctl, Release Rate Ear/SPKLO */
    reg_default { reg: 46, def: 0x00 }, /* r2E - Limter Threshold Ear/SPKLO */
    reg_default { reg: 47, def: 0x00 }, /* r2F - ALC Enable, Attack Rate Left/Right */
    reg_default { reg: 48, def: 0x3F }, /* r30 - ALC Release Rate Left/Right */
    reg_default { reg: 49, def: 0x00 }, /* r31 - ALC Threshold Left/Right */
    reg_default { reg: 50, def: 0x00 }, /* r32 - Noise Gate Ctl Left/Right */
    reg_default { reg: 51, def: 0x00 }, /* r33 - ALC/NG Misc Ctl */
    reg_default { reg: 52, def: 0x18 }, /* r34 - Mixer Ctl */
    reg_default { reg: 53, def: 0x3F }, /* r35 - HP/LO Left Mixer Input Path Volume */
    reg_default { reg: 54, def: 0x3F }, /* r36 - HP/LO Right Mixer Input Path Volume */
    reg_default { reg: 55, def: 0x3F }, /* r37 - HP/LO Left Mixer Aux PCM Volume */
    reg_default { reg: 56, def: 0x3F }, /* r38 - HP/LO Right Mixer Aux PCM Volume */
    reg_default { reg: 57, def: 0x3F }, /* r39 - HP/LO Left Mixer Audio PCM Volume */
    reg_default { reg: 58, def: 0x3F }, /* r3A - HP/LO Right Mixer Audio PCM Volume */
    reg_default { reg: 59, def: 0x3F }, /* r3B - HP/LO Left Mixer Voice PCM Mono Volume */
    reg_default { reg: 60, def: 0x3F }, /* r3C - HP/LO Right Mixer Voice PCM Mono Volume */
    reg_default { reg: 61, def: 0x3F }, /* r3D - Aux PCM Left Mixer Input Path Volume */
    reg_default { reg: 62, def: 0x3F }, /* r3E - Aux PCM Right Mixer Input Path Volume */
    reg_default { reg: 63, def: 0x3F }, /* r3F - Aux PCM Left Mixer Volume */
    reg_default { reg: 64, def: 0x3F }, /* r40 - Aux PCM Left Mixer Volume */
    reg_default { reg: 65, def: 0x3F }, /* r41 - Aux PCM Left Mixer Audio PCM L Volume */
    reg_default { reg: 66, def: 0x3F }, /* r42 - Aux PCM Right Mixer Audio PCM R Volume */
    reg_default { reg: 67, def: 0x3F }, /* r43 - Aux PCM Left Mixer Voice PCM Volume */
    reg_default { reg: 68, def: 0x3F }, /* r44 - Aux PCM Right Mixer Voice PCM Volume */
    reg_default { reg: 69, def: 0x3F }, /* r45 - Audio PCM Left Input Path Volume */
    reg_default { reg: 70, def: 0x3F }, /* r46 - Audio PCM Right Input Path Volume */
    reg_default { reg: 71, def: 0x3F }, /* r47 - Audio PCM Left Mixer Aux PCM L Volume */
    reg_default { reg: 72, def: 0x3F }, /* r48 - Audio PCM Right Mixer Aux PCM R Volume */
    reg_default { reg: 73, def: 0x3F }, /* r49 - Audio PCM Left Mixer Volume */
    reg_default { reg: 74, def: 0x3F }, /* r4A - Audio PCM Right Mixer Volume */
    reg_default { reg: 75, def: 0x3F }, /* r4B - Audio PCM Left Mixer Voice PCM Volume */
    reg_default { reg: 76, def: 0x3F }, /* r4C - Audio PCM Right Mixer Voice PCM Volume */
    reg_default { reg: 77, def: 0x3F }, /* r4D - Voice PCM Left Input Path Volume */
    reg_default { reg: 78, def: 0x3F }, /* r4E - Voice PCM Right Input Path Volume */
    reg_default { reg: 79, def: 0x3F }, /* r4F - Voice PCM Left Mixer Aux PCM L Volume */
    reg_default { reg: 80, def: 0x3F }, /* r50 - Voice PCM Right Mixer Aux PCM R Volume */
    reg_default { reg: 81, def: 0x3F }, /* r51 - Voice PCM Left Mixer Audio PCM L Volume */
    reg_default { reg: 82, def: 0x3F }, /* r52 - Voice PCM Right Mixer Audio PCM R Volume */
    reg_default { reg: 83, def: 0x3F }, /* r53 - Voice PCM Left Mixer Voice PCM Volume */
    reg_default { reg: 84, def: 0x3F }, /* r54 - Voice PCM Right Mixer Voice PCM Volume */
    reg_default { reg: 85, def: 0xAA }, /* r55 - Mono Mixer Ctl */
    reg_default { reg: 86, def: 0x3F }, /* r56 - SPK Mono Mixer Input Path Volume */
    reg_default { reg: 87, def: 0x3F }, /* r57 - SPK Mono Mixer Aux PCM Mono/L/R Volume */
    reg_default { reg: 88, def: 0x3F }, /* r58 - SPK Mono Mixer Audio PCM Mono/L/R Volume */
    reg_default { reg: 89, def: 0x3F }, /* r59 - SPK Mono Mixer Voice PCM Mono Volume */
    reg_default { reg: 90, def: 0x3F }, /* r5A - SPKLO Mono Mixer Input Path Mono Volume */
    reg_default { reg: 91, def: 0x3F }, /* r5B - SPKLO Mono Mixer Aux Mono/L/R Volume */
    reg_default { reg: 92, def: 0x3F }, /* r5C - SPKLO Mono Mixer Audio Mono/L/R Volume */
    reg_default { reg: 93, def: 0x3F }, /* r5D - SPKLO Mono Mixer Voice Mono Volume */
    reg_default { reg: 94, def: 0x00 }, /* r5E - Interrupt Mask 1 */
    reg_default { reg: 95, def: 0x00 }, /* r5F - Interrupt Mask 2 */
];

unsafe extern "C" {
    static CS42L73_IS1: c_uint;
    static CS42L73_IS2: c_uint;
    static CS42L73_DEVID_AB: c_uint;
    static CS42L73_DEVID_E: c_uint;
    static CS42L73_REVID: c_uint;
    static CS42L73_IM2: c_uint;
    static CS42L73_DMMCC: c_uint;
    static CS42L73_PWRCTL1: c_uint;
    static CS42L73_PWRCTL2: c_uint;
    static CS42L73_PWRCTL3: c_uint;
    static CS42L73_CPFCHC: c_uint;
    static CS42L73_CHARGEPUMP_MASK: c_uint;
    static CS42L73_MCLKDIS: c_uint;
    static CS42L73_PDN: c_uint;
    static CS42L73_MAX_REGISTER: c_uint;
    static CS42L73_DEVID: c_int;
    static CS42L73_CLKID_MCLK1: u8;
    static CS42L73_CLKID_MCLK2: u8;
    static CS42L73_MS_MASTER: u8;
    static CS42L73_SPDIF_PCM: u8;
    static CS42L73_ASP: c_int;
    static CS42L73_XSP: c_int;
    static CS42L73_VSP: c_int;
    static CS42L73_PCM_MODE_MASK: u8;
    static CS42L73_PCM_BIT_ORDER: u8;
    static CS42L73_PCM_MODE0: u8;
    static CS42L73_PCM_MODE1: u8;
    static CS42L73_MCK_SCLK_64FS: u8;
    static CS42L73_MCK_SCLK_MCLK: u8;
    static CS42L73_SP_3ST: c_uint;
    static REGCACHE_MAPLE: c_uint;
    static GFP_KERNEL: c_uint;
    static GPIOD_OUT_LOW: c_uint;
    static SNDRV_PCM_HW_PARAM_RATE: c_uint;
    static SNDRV_PCM_RATE_KNOT: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S20_3LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SND_SOC_DAPM_POST_PMD: c_int;
    static SND_SOC_DAIFMT_MASTER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;

    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn mdelay(ms: c_int);
    fn snd_pcm_hw_constraint_list(runtime: *mut c_void, cond: c_uint, var: c_uint, l: *const snd_pcm_hw_constraint_list) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn devm_kzalloc(dev: *mut device_with_of, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn of_property_read_u32(node: *mut c_void, name: *const c_char, out: *mut u32) -> c_int;
    fn devm_gpiod_get_optional(dev: *mut device_with_of, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn gpiod_set_consumer_name(desc: *mut gpio_desc, name: *const c_char);
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn cirrus_read_device_id(map: *mut regmap, reg: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device_with_of,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_uint,
    ) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device_with_of, fmt: *const c_char, ...);
}

unsafe extern "C" fn cs42l73_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    reg == CS42L73_IS1 || reg == CS42L73_IS2
}

unsafe extern "C" fn cs42l73_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    (reg >= CS42L73_DEVID_AB && reg <= CS42L73_DEVID_E) || (reg >= CS42L73_REVID && reg <= CS42L73_IM2)
}

/* Direct translations of ASoC TLV/control/enum/widget macros from the C source.
 * Their concrete definitions are provided externally by the ALSA SoC bindings.
 */
extern "Rust" {
    static hpaloa_tlv: c_void;
    static adc_boost_tlv: c_void;
    static hl_tlv: c_void;
    static ipd_tlv: c_void;
    static micpga_tlv: c_void;
    static limiter_tlv: c_void;
    static attn_tlv: c_void;
    static pgaa_enum: soc_enum;
    static pgab_enum: soc_enum;
    static pgaa_mux: snd_kcontrol_new;
    static pgab_mux: snd_kcontrol_new;
    static input_left_mixer: [snd_kcontrol_new; 2];
    static input_right_mixer: [snd_kcontrol_new; 2];
    static ng_delay_enum: soc_enum;
    static spk_asp_enum: soc_enum;
    static spk_asp_mixer: snd_kcontrol_new;
    static spk_xsp_enum: soc_enum;
    static spk_xsp_mixer: snd_kcontrol_new;
    static esl_asp_enum: soc_enum;
    static esl_asp_mixer: snd_kcontrol_new;
    static esl_xsp_enum: soc_enum;
    static esl_xsp_mixer: snd_kcontrol_new;
    static ip_swap_enum: soc_enum;
    static vsp_output_mux_enum: soc_enum;
    static xsp_output_mux_enum: soc_enum;
    static hp_amp_ctl: snd_kcontrol_new;
    static lo_amp_ctl: snd_kcontrol_new;
    static spk_amp_ctl: snd_kcontrol_new;
    static spklo_amp_ctl: snd_kcontrol_new;
    static ear_amp_ctl: snd_kcontrol_new;
    static cs42l73_snd_controls: [snd_kcontrol_new; 77];
    static cs42l73_dapm_widgets: [snd_soc_dapm_widget_desc; 62];
}

static cs42l73_pgaa_text: [&[u8]; 2] = [b"Line A\0", b"Mic 1\0"];
static cs42l73_pgab_text: [&[u8]; 2] = [b"Line B\0", b"Mic 2\0"];
static cs42l73_ng_delay_text: [&[u8]; 4] = [b"50ms\0", b"100ms\0", b"150ms\0", b"200ms\0"];
static cs42l73_mono_mix_texts: [&[u8]; 3] = [b"Left\0", b"Right\0", b"Mono Mix\0"];
static cs42l73_mono_mix_values: [c_uint; 3] = [0, 1, 2];
static cs42l73_ip_swap_text: [&[u8]; 4] = [b"Stereo\0", b"Mono A\0", b"Mono B\0", b"Swap A-B\0"];
static cs42l73_spo_mixer_text: [&[u8]; 2] = [b"Mono\0", b"Stereo\0"];

unsafe extern "C" fn cs42l73_spklo_spk_amp_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs42l73_private;
    match event {
        x if x == SND_SOC_DAPM_POST_PMD => {
            /* 150 ms delay between setting PDN and MCLKDIS */
            (*priv_).shutdwn_delay = 150;
        }
        _ => pr_err(b"Invalid event = 0x%x\n\0".as_ptr() as *const c_char, event),
    }
    0
}

unsafe extern "C" fn cs42l73_ear_amp_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs42l73_private;
    match event {
        x if x == SND_SOC_DAPM_POST_PMD => {
            /* 50 ms delay between setting PDN and MCLKDIS */
            if (*priv_).shutdwn_delay < 50 {
                (*priv_).shutdwn_delay = 50;
            }
        }
        _ => pr_err(b"Invalid event = 0x%x\n\0".as_ptr() as *const c_char, event),
    }
    0
}

unsafe extern "C" fn cs42l73_hp_amp_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs42l73_private;
    match event {
        x if x == SND_SOC_DAPM_POST_PMD => {
            /* 30 ms delay between setting PDN and MCLKDIS */
            if (*priv_).shutdwn_delay < 30 {
                (*priv_).shutdwn_delay = 30;
            }
        }
        _ => pr_err(b"Invalid event = 0x%x\n\0".as_ptr() as *const c_char, event),
    }
    0
}

macro_rules! route {
    ($sink:expr, NULL, $source:expr) => {
        snd_soc_dapm_route { sink: concat!($sink, "\0").as_ptr() as *const c_char, control: ptr::null(), source: concat!($source, "\0").as_ptr() as *const c_char }
    };
    ($sink:expr, $control:expr, $source:expr) => {
        snd_soc_dapm_route { sink: concat!($sink, "\0").as_ptr() as *const c_char, control: concat!($control, "\0").as_ptr() as *const c_char, source: concat!($source, "\0").as_ptr() as *const c_char }
    };
}

static cs42l73_audio_map: &[snd_soc_dapm_route] = &[
    /* SPKLO EARSPK Paths */
    route!("EAROUT", NULL, "EAR Amp"), route!("SPKLINEOUT", NULL, "SPKLO Amp"),
    route!("EAR Amp", "Switch", "ESL DAC"), route!("SPKLO Amp", "Switch", "ESL DAC"),
    route!("ESL DAC", "ESL-ASP Mono Volume", "ESL Mixer"), route!("ESL DAC", "ESL-XSP Mono Volume", "ESL Mixer"), route!("ESL DAC", "ESL-VSP Mono Volume", "VSPINOUT"),
    /* Loopback */
    route!("ESL DAC", "ESL-IP Mono Volume", "Input Left Capture"), route!("ESL DAC", "ESL-IP Mono Volume", "Input Right Capture"),
    route!("ESL Mixer", NULL, "ESL-ASP Mux"), route!("ESL Mixer", NULL, "ESL-XSP Mux"),
    route!("ESL-ASP Mux", "Left", "ASPINL"), route!("ESL-ASP Mux", "Right", "ASPINR"), route!("ESL-ASP Mux", "Mono Mix", "ASPINM"),
    route!("ESL-XSP Mux", "Left", "XSPINL"), route!("ESL-XSP Mux", "Right", "XSPINR"), route!("ESL-XSP Mux", "Mono Mix", "XSPINM"),
    /* Speakerphone Paths */
    route!("SPKOUT", NULL, "SPK Amp"), route!("SPK Amp", "Switch", "SPK DAC"),
    route!("SPK DAC", "SPK-ASP Mono Volume", "SPK Mixer"), route!("SPK DAC", "SPK-XSP Mono Volume", "SPK Mixer"), route!("SPK DAC", "SPK-VSP Mono Volume", "VSPINOUT"),
    /* Loopback */
    route!("SPK DAC", "SPK-IP Mono Volume", "Input Left Capture"), route!("SPK DAC", "SPK-IP Mono Volume", "Input Right Capture"),
    route!("SPK Mixer", NULL, "SPK-ASP Mux"), route!("SPK Mixer", NULL, "SPK-XSP Mux"),
    route!("SPK-ASP Mux", "Left", "ASPINL"), route!("SPK-ASP Mux", "Mono Mix", "ASPINM"), route!("SPK-ASP Mux", "Right", "ASPINR"),
    route!("SPK-XSP Mux", "Left", "XSPINL"), route!("SPK-XSP Mux", "Mono Mix", "XSPINM"), route!("SPK-XSP Mux", "Right", "XSPINR"),
    /* HP LineOUT Paths */
    route!("HPOUTA", NULL, "HP Amp"), route!("HPOUTB", NULL, "HP Amp"), route!("LINEOUTA", NULL, "LO Amp"), route!("LINEOUTB", NULL, "LO Amp"),
    route!("HP Amp", "Switch", "HL Left DAC"), route!("HP Amp", "Switch", "HL Right DAC"), route!("LO Amp", "Switch", "HL Left DAC"), route!("LO Amp", "Switch", "HL Right DAC"),
    route!("HL Left DAC", "HL-XSP Volume", "HL Left Mixer"), route!("HL Right DAC", "HL-XSP Volume", "HL Right Mixer"),
    route!("HL Left DAC", "HL-ASP Volume", "HL Left Mixer"), route!("HL Right DAC", "HL-ASP Volume", "HL Right Mixer"),
    route!("HL Left DAC", "HL-VSP Volume", "HL Left Mixer"), route!("HL Right DAC", "HL-VSP Volume", "HL Right Mixer"),
    /* Loopback */
    route!("HL Left DAC", "HL-IP Volume", "HL Left Mixer"), route!("HL Right DAC", "HL-IP Volume", "HL Right Mixer"),
    route!("HL Left Mixer", NULL, "Input Left Capture"), route!("HL Right Mixer", NULL, "Input Right Capture"),
    route!("HL Left Mixer", NULL, "ASPINL"), route!("HL Right Mixer", NULL, "ASPINR"), route!("HL Left Mixer", NULL, "XSPINL"), route!("HL Right Mixer", NULL, "XSPINR"),
    route!("HL Left Mixer", NULL, "VSPINOUT"), route!("HL Right Mixer", NULL, "VSPINOUT"),
    route!("ASPINL", NULL, "ASP Playback"), route!("ASPINM", NULL, "ASP Playback"), route!("ASPINR", NULL, "ASP Playback"),
    route!("XSPINL", NULL, "XSP Playback"), route!("XSPINM", NULL, "XSP Playback"), route!("XSPINR", NULL, "XSP Playback"), route!("VSPINOUT", NULL, "VSP Playback"),
    /* Capture Paths */
    route!("MIC1", NULL, "MIC1 Bias"), route!("PGA Left Mux", "Mic 1", "MIC1"), route!("MIC2", NULL, "MIC2 Bias"), route!("PGA Right Mux", "Mic 2", "MIC2"),
    route!("PGA Left Mux", "Line A", "LINEINA"), route!("PGA Right Mux", "Line B", "LINEINB"), route!("PGA Left", NULL, "PGA Left Mux"), route!("PGA Right", NULL, "PGA Right Mux"),
    route!("ADC Left", NULL, "PGA Left"), route!("ADC Right", NULL, "PGA Right"), route!("DMIC Left", NULL, "DMICA"), route!("DMIC Right", NULL, "DMICB"),
    route!("Input Left Capture", "ADC Left Input", "ADC Left"), route!("Input Right Capture", "ADC Right Input", "ADC Right"),
    route!("Input Left Capture", "DMIC Left Input", "DMIC Left"), route!("Input Right Capture", "DMIC Right Input", "DMIC Right"),
    /* Audio Capture */
    route!("ASPL Output Mixer", NULL, "Input Left Capture"), route!("ASPR Output Mixer", NULL, "Input Right Capture"),
    route!("ASPOUTL", "ASP-IP Volume", "ASPL Output Mixer"), route!("ASPOUTR", "ASP-IP Volume", "ASPR Output Mixer"),
    /* Auxillary Capture */
    route!("XSPL Output Mixer", NULL, "Input Left Capture"), route!("XSPR Output Mixer", NULL, "Input Right Capture"),
    route!("XSPOUTL", "XSP-IP Volume", "XSPL Output Mixer"), route!("XSPOUTR", "XSP-IP Volume", "XSPR Output Mixer"),
    route!("XSPOUTL", NULL, "XSPL Output Mixer"), route!("XSPOUTR", NULL, "XSPR Output Mixer"),
    /* Voice Capture */
    route!("VSP Output Mixer", NULL, "Input Left Capture"), route!("VSP Output Mixer", NULL, "Input Right Capture"),
    route!("VSPINOUT", "VSP-IP Volume", "VSP Output Mixer"), route!("VSPINOUT", NULL, "VSP Output Mixer"),
    route!("ASP Capture", NULL, "ASPOUTL"), route!("ASP Capture", NULL, "ASPOUTR"), route!("XSP Capture", NULL, "XSPOUTL"), route!("XSP Capture", NULL, "XSPOUTR"), route!("VSP Capture", NULL, "VSPINOUT"),
];

#[repr(C)]
pub struct cs42l73_mclk_div {
    pub mclk: u32,
    pub srate: u32,
    pub mmcc: u8,
}

static cs42l73_mclk_coeffs: &[cs42l73_mclk_div] = &[
    /* MCLK, Sample Rate, xMMCC[5:0] */
    cs42l73_mclk_div { mclk: 5644800, srate: 11025, mmcc: 0x30 }, cs42l73_mclk_div { mclk: 5644800, srate: 22050, mmcc: 0x20 }, cs42l73_mclk_div { mclk: 5644800, srate: 44100, mmcc: 0x10 },
    cs42l73_mclk_div { mclk: 6000000, srate: 8000, mmcc: 0x39 }, cs42l73_mclk_div { mclk: 6000000, srate: 11025, mmcc: 0x33 }, cs42l73_mclk_div { mclk: 6000000, srate: 12000, mmcc: 0x31 }, cs42l73_mclk_div { mclk: 6000000, srate: 16000, mmcc: 0x29 }, cs42l73_mclk_div { mclk: 6000000, srate: 22050, mmcc: 0x23 }, cs42l73_mclk_div { mclk: 6000000, srate: 24000, mmcc: 0x21 }, cs42l73_mclk_div { mclk: 6000000, srate: 32000, mmcc: 0x19 }, cs42l73_mclk_div { mclk: 6000000, srate: 44100, mmcc: 0x13 }, cs42l73_mclk_div { mclk: 6000000, srate: 48000, mmcc: 0x11 },
    cs42l73_mclk_div { mclk: 6144000, srate: 8000, mmcc: 0x38 }, cs42l73_mclk_div { mclk: 6144000, srate: 12000, mmcc: 0x30 }, cs42l73_mclk_div { mclk: 6144000, srate: 16000, mmcc: 0x28 }, cs42l73_mclk_div { mclk: 6144000, srate: 24000, mmcc: 0x20 }, cs42l73_mclk_div { mclk: 6144000, srate: 32000, mmcc: 0x18 }, cs42l73_mclk_div { mclk: 6144000, srate: 48000, mmcc: 0x10 },
    cs42l73_mclk_div { mclk: 6500000, srate: 8000, mmcc: 0x3C }, cs42l73_mclk_div { mclk: 6500000, srate: 11025, mmcc: 0x35 }, cs42l73_mclk_div { mclk: 6500000, srate: 12000, mmcc: 0x34 }, cs42l73_mclk_div { mclk: 6500000, srate: 16000, mmcc: 0x2C }, cs42l73_mclk_div { mclk: 6500000, srate: 22050, mmcc: 0x25 }, cs42l73_mclk_div { mclk: 6500000, srate: 24000, mmcc: 0x24 }, cs42l73_mclk_div { mclk: 6500000, srate: 32000, mmcc: 0x1C }, cs42l73_mclk_div { mclk: 6500000, srate: 44100, mmcc: 0x15 }, cs42l73_mclk_div { mclk: 6500000, srate: 48000, mmcc: 0x14 },
    cs42l73_mclk_div { mclk: 6400000, srate: 8000, mmcc: 0x3E }, cs42l73_mclk_div { mclk: 6400000, srate: 11025, mmcc: 0x37 }, cs42l73_mclk_div { mclk: 6400000, srate: 12000, mmcc: 0x36 }, cs42l73_mclk_div { mclk: 6400000, srate: 16000, mmcc: 0x2E }, cs42l73_mclk_div { mclk: 6400000, srate: 22050, mmcc: 0x27 }, cs42l73_mclk_div { mclk: 6400000, srate: 24000, mmcc: 0x26 }, cs42l73_mclk_div { mclk: 6400000, srate: 32000, mmcc: 0x1E }, cs42l73_mclk_div { mclk: 6400000, srate: 44100, mmcc: 0x17 }, cs42l73_mclk_div { mclk: 6400000, srate: 48000, mmcc: 0x16 },
];

#[repr(C)]
pub struct cs42l73_mclkx_div {
    pub mclkx: u32,
    pub ratio: u8,
    pub mclkdiv: u8,
}

static cs42l73_mclkx_coeffs: &[cs42l73_mclkx_div] = &[
    cs42l73_mclkx_div { mclkx: 5644800, ratio: 1, mclkdiv: 0 }, /* 5644800 */
    cs42l73_mclkx_div { mclkx: 6000000, ratio: 1, mclkdiv: 0 }, /* 6000000 */
    cs42l73_mclkx_div { mclkx: 6144000, ratio: 1, mclkdiv: 0 }, /* 6144000 */
    cs42l73_mclkx_div { mclkx: 11289600, ratio: 2, mclkdiv: 2 }, /* 5644800 */
    cs42l73_mclkx_div { mclkx: 12288000, ratio: 2, mclkdiv: 2 }, /* 6144000 */
    cs42l73_mclkx_div { mclkx: 12000000, ratio: 2, mclkdiv: 2 }, /* 6000000 */
    cs42l73_mclkx_div { mclkx: 13000000, ratio: 2, mclkdiv: 2 }, /* 6500000 */
    cs42l73_mclkx_div { mclkx: 19200000, ratio: 3, mclkdiv: 3 }, /* 6400000 */
    cs42l73_mclkx_div { mclkx: 24000000, ratio: 4, mclkdiv: 4 }, /* 6000000 */
    cs42l73_mclkx_div { mclkx: 26000000, ratio: 4, mclkdiv: 4 }, /* 6500000 */
    cs42l73_mclkx_div { mclkx: 38400000, ratio: 6, mclkdiv: 5 }, /* 6400000 */
];

unsafe extern "C" fn CS42L73_SPC(id: c_int) -> c_uint { todo!("external CS42L73_SPC register macro") }
unsafe extern "C" fn CS42L73_MMCC(id: c_int) -> c_uint { todo!("external CS42L73_MMCC register macro") }

unsafe extern "C" fn cs42l73_get_mclkx_coeff(mclkx: c_int) -> c_int {
    for (i, coeff) in cs42l73_mclkx_coeffs.iter().enumerate() {
        if coeff.mclkx == mclkx as u32 {
            return i as c_int;
        }
    }
    -EINVAL
}

unsafe extern "C" fn cs42l73_get_mclk_coeff(mclk: c_int, srate: c_int) -> c_int {
    for (i, coeff) in cs42l73_mclk_coeffs.iter().enumerate() {
        if coeff.mclk == mclk as u32 && coeff.srate == srate as u32 {
            return i as c_int;
        }
    }
    -EINVAL
}

unsafe extern "C" fn cs42l73_set_mclk(dai: *mut snd_soc_dai, freq: c_uint) -> c_int {
    let component = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs42l73_private;
    let mclkx_coeff = cs42l73_get_mclkx_coeff(freq as c_int);
    if mclkx_coeff < 0 {
        return mclkx_coeff;
    }
    let coeff = &cs42l73_mclkx_coeffs[mclkx_coeff as usize];
    let mclk: u32 = coeff.mclkx / coeff.ratio as u32;
    dev_dbg((*component).dev, b"MCLK%u %u  <-> internal MCLK %u\n\0".as_ptr() as *const c_char, (*priv_).mclksel as c_uint + 1, coeff.mclkx, mclk);
    let dmmcc: u8 = ((*priv_).mclksel << 4) | (coeff.mclkdiv << 1);
    snd_soc_component_write(component, CS42L73_DMMCC, dmmcc as c_uint);
    (*priv_).sysclk = mclkx_coeff as u32;
    (*priv_).mclk = mclk;
    0
}

unsafe extern "C" fn cs42l73_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs42l73_private;
    if clk_id != CS42L73_CLKID_MCLK1 as c_int && clk_id != CS42L73_CLKID_MCLK2 as c_int {
        return -EINVAL;
    }
    if cs42l73_set_mclk(dai, freq) < 0 {
        dev_err((*component).dev, b"Unable to set MCLK for dai %s\n\0".as_ptr() as *const c_char, (*dai).name);
        return -EINVAL;
    }
    (*priv_).mclksel = clk_id as u8;
    0
}

unsafe extern "C" fn cs42l73_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs42l73_private;
    let id = (*codec_dai).id as usize;
    let mut spc = snd_soc_component_read(component, CS42L73_SPC(id as c_int)) as u8;
    let mut mmcc = snd_soc_component_read(component, CS42L73_MMCC(id as c_int)) as u8;

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        x if x == SND_SOC_DAIFMT_CBP_CFP => mmcc |= CS42L73_MS_MASTER,
        x if x == SND_SOC_DAIFMT_CBC_CFC => mmcc &= !CS42L73_MS_MASTER,
        _ => return -EINVAL,
    }

    let format = fmt & SND_SOC_DAIFMT_FORMAT_MASK;
    let inv = fmt & SND_SOC_DAIFMT_INV_MASK;
    match format {
        x if x == SND_SOC_DAIFMT_I2S => spc &= !CS42L73_SPDIF_PCM,
        x if x == SND_SOC_DAIFMT_DSP_A || x == SND_SOC_DAIFMT_DSP_B => {
            if (mmcc & CS42L73_MS_MASTER) != 0 {
                dev_err((*component).dev, b"PCM format in slave mode only\n\0".as_ptr() as *const c_char);
                return -EINVAL;
            }
            if id as c_int == CS42L73_ASP {
                dev_err((*component).dev, b"PCM format is not supported on ASP port\n\0".as_ptr() as *const c_char);
                return -EINVAL;
            }
            spc |= CS42L73_SPDIF_PCM;
        }
        _ => return -EINVAL,
    }

    if (spc & CS42L73_SPDIF_PCM) != 0 {
        /* Clear PCM mode, clear PCM_BIT_ORDER bit for MSB->LSB */
        spc &= !(CS42L73_PCM_MODE_MASK | CS42L73_PCM_BIT_ORDER);
        match format {
            x if x == SND_SOC_DAIFMT_DSP_B => {
                if inv == SND_SOC_DAIFMT_IB_IF { spc |= CS42L73_PCM_MODE0; }
                if inv == SND_SOC_DAIFMT_IB_NF { spc |= CS42L73_PCM_MODE1; }
            }
            x if x == SND_SOC_DAIFMT_DSP_A => {
                if inv == SND_SOC_DAIFMT_IB_IF { spc |= CS42L73_PCM_MODE1; }
            }
            _ => return -EINVAL,
        }
    }

    (*priv_).config[id].spc = spc;
    (*priv_).config[id].mmcc = mmcc;
    0
}

static cs42l73_asrc_rates: [c_uint; 9] = [8000, 11025, 12000, 16000, 22050, 24000, 32000, 44100, 48000];

unsafe extern "C" fn cs42l73_get_xspfs_coeff(rate: u32) -> c_uint {
    for (i, r) in cs42l73_asrc_rates.iter().enumerate() {
        if *r == rate {
            return i as c_uint + 1;
        }
    }
    0 /* 0 = Don't know */
}

unsafe extern "C" fn cs42l73_update_asrc(component: *mut snd_soc_component, id: c_int, srate: c_int) {
    let mut spfs: u8 = 0;
    if srate > 0 {
        spfs = cs42l73_get_xspfs_coeff(srate as u32) as u8;
    }
    if id == CS42L73_XSP {
        snd_soc_component_update_bits(component, CS42L73_VXSPFS, 0x0f, spfs as c_uint);
    } else if id == CS42L73_ASP {
        snd_soc_component_update_bits(component, CS42L73_ASPC, 0x3c, (spfs as c_uint) << 2);
    } else if id == CS42L73_VSP {
        snd_soc_component_update_bits(component, CS42L73_VXSPFS, 0xf0, (spfs as c_uint) << 4);
    }
}

unsafe extern "C" {
    static CS42L73_VXSPFS: c_uint;
    static CS42L73_ASPC: c_uint;
}

unsafe extern "C" fn cs42l73_pcm_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs42l73_private;
    let id = (*dai).id as usize;
    let srate = params_rate(params);
    if ((*priv_).config[id].mmcc & CS42L73_MS_MASTER) != 0 {
        /* CS42L73 Master */
        /* MCLK -> srate */
        let mclk_coeff = cs42l73_get_mclk_coeff((*priv_).mclk as c_int, srate);
        if mclk_coeff < 0 {
            return -EINVAL;
        }
        dev_dbg((*component).dev, b"DAI[%d]: MCLK %u, srate %u, MMCC[5:0] = %x\n\0".as_ptr() as *const c_char, id as c_int, (*priv_).mclk, srate, cs42l73_mclk_coeffs[mclk_coeff as usize].mmcc as c_uint);
        (*priv_).config[id].mmcc &= 0xC0;
        (*priv_).config[id].mmcc |= cs42l73_mclk_coeffs[mclk_coeff as usize].mmcc;
        (*priv_).config[id].spc &= 0xFC;
        /* Use SCLK=64*Fs if internal MCLK >= 6.4MHz */
        if (*priv_).mclk >= 6400000 {
            (*priv_).config[id].spc |= CS42L73_MCK_SCLK_64FS;
        } else {
            (*priv_).config[id].spc |= CS42L73_MCK_SCLK_MCLK;
        }
    } else {
        /* CS42L73 Slave */
        (*priv_).config[id].spc &= 0xFC;
        (*priv_).config[id].spc |= CS42L73_MCK_SCLK_64FS;
    }
    /* Update ASRCs */
    (*priv_).config[id].srate = srate as u32;
    snd_soc_component_write(component, CS42L73_SPC(id as c_int), (*priv_).config[id].spc as c_uint);
    snd_soc_component_write(component, CS42L73_MMCC(id as c_int), (*priv_).config[id].mmcc as c_uint);
    cs42l73_update_asrc(component, id as c_int, srate);
    0
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON = 0,
    SND_SOC_BIAS_PREPARE = 1,
    SND_SOC_BIAS_STANDBY = 2,
    SND_SOC_BIAS_OFF = 3,
}

unsafe extern "C" fn cs42l73_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let cs42l73 = snd_soc_component_get_drvdata(component) as *mut cs42l73_private;
    let dapm = snd_soc_component_to_dapm(component);
    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => {
            snd_soc_component_update_bits(component, CS42L73_DMMCC, CS42L73_MCLKDIS, 0);
            snd_soc_component_update_bits(component, CS42L73_PWRCTL1, CS42L73_PDN, 0);
        }
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {}
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == snd_soc_bias_level::SND_SOC_BIAS_OFF {
                regcache_cache_only((*cs42l73).regmap, false);
                regcache_sync((*cs42l73).regmap);
            }
            snd_soc_component_update_bits(component, CS42L73_PWRCTL1, CS42L73_PDN, 1);
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            snd_soc_component_update_bits(component, CS42L73_PWRCTL1, CS42L73_PDN, 1);
            if (*cs42l73).shutdwn_delay > 0 {
                mdelay((*cs42l73).shutdwn_delay);
                (*cs42l73).shutdwn_delay = 0;
            } else {
                mdelay(15); /* Min amount of time requred to power
                              * down.
                              */
            }
            snd_soc_component_update_bits(component, CS42L73_DMMCC, CS42L73_MCLKDIS, 1);
        }
    }
    0
}

unsafe extern "C" fn cs42l73_set_tristate(dai: *mut snd_soc_dai, tristate: c_int) -> c_int {
    let component = (*dai).component;
    let id = (*dai).id;
    snd_soc_component_update_bits(component, CS42L73_SPC(id), CS42L73_SP_3ST, (tristate << 7) as c_uint)
}

static constraints_12_24: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: cs42l73_asrc_rates.len() as c_uint,
    list: cs42l73_asrc_rates.as_ptr(),
};

unsafe extern "C" fn cs42l73_pcm_startup(substream: *mut snd_pcm_substream, _dai: *mut snd_soc_dai) -> c_int {
    snd_pcm_hw_constraint_list((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &constraints_12_24);
    0
}

fn CS42L73_FORMATS() -> u64 {
    unsafe { SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE }
}

static cs42l73_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(cs42l73_pcm_startup),
    hw_params: Some(cs42l73_pcm_hw_params),
    set_fmt: Some(cs42l73_set_dai_fmt),
    set_sysclk: Some(cs42l73_set_sysclk),
    set_tristate: Some(cs42l73_set_tristate),
};

static mut cs42l73_dai: [snd_soc_dai_driver; 3] = [
    snd_soc_dai_driver {
        name: b"cs42l73-xsp\0".as_ptr() as *const c_char,
        id: unsafe { CS42L73_XSP },
        playback: snd_soc_pcm_stream { stream_name: b"XSP Playback\0".as_ptr() as *const c_char, channels_min: 1, channels_max: 2, rates: unsafe { SNDRV_PCM_RATE_KNOT }, formats: 0 },
        capture: snd_soc_pcm_stream { stream_name: b"XSP Capture\0".as_ptr() as *const c_char, channels_min: 1, channels_max: 2, rates: unsafe { SNDRV_PCM_RATE_KNOT }, formats: 0 },
        ops: &cs42l73_ops,
        symmetric_rate: 1,
    },
    snd_soc_dai_driver {
        name: b"cs42l73-asp\0".as_ptr() as *const c_char,
        id: unsafe { CS42L73_ASP },
        playback: snd_soc_pcm_stream { stream_name: b"ASP Playback\0".as_ptr() as *const c_char, channels_min: 2, channels_max: 2, rates: unsafe { SNDRV_PCM_RATE_KNOT }, formats: 0 },
        capture: snd_soc_pcm_stream { stream_name: b"ASP Capture\0".as_ptr() as *const c_char, channels_min: 2, channels_max: 2, rates: unsafe { SNDRV_PCM_RATE_KNOT }, formats: 0 },
        ops: &cs42l73_ops,
        symmetric_rate: 1,
    },
    snd_soc_dai_driver {
        name: b"cs42l73-vsp\0".as_ptr() as *const c_char,
        id: unsafe { CS42L73_VSP },
        playback: snd_soc_pcm_stream { stream_name: b"VSP Playback\0".as_ptr() as *const c_char, channels_min: 1, channels_max: 2, rates: unsafe { SNDRV_PCM_RATE_KNOT }, formats: 0 },
        capture: snd_soc_pcm_stream { stream_name: b"VSP Capture\0".as_ptr() as *const c_char, channels_min: 1, channels_max: 2, rates: unsafe { SNDRV_PCM_RATE_KNOT }, formats: 0 },
        ops: &cs42l73_ops,
        symmetric_rate: 1,
    },
];

unsafe extern "C" fn cs42l73_probe(component: *mut snd_soc_component) -> c_int {
    let cs42l73 = snd_soc_component_get_drvdata(component) as *mut cs42l73_private;
    /* Set Charge Pump Frequency */
    if (*cs42l73).pdata.chgfreq != 0 {
        snd_soc_component_update_bits(component, CS42L73_CPFCHC, CS42L73_CHARGEPUMP_MASK, (*cs42l73).pdata.chgfreq << 4);
    }
    /* MCLK1 as master clk */
    (*cs42l73).mclksel = CS42L73_CLKID_MCLK1;
    (*cs42l73).mclk = 0;
    0
}

static soc_component_dev_cs42l73: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(cs42l73_probe),
    set_bias_level: Some(cs42l73_set_bias_level),
    controls: unsafe { cs42l73_snd_controls.as_ptr() },
    num_controls: 77,
    dapm_widgets: unsafe { cs42l73_dapm_widgets.as_ptr() },
    num_dapm_widgets: 62,
    dapm_routes: cs42l73_audio_map.as_ptr(),
    num_dapm_routes: cs42l73_audio_map.len() as c_uint,
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static cs42l73_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: unsafe { CS42L73_MAX_REGISTER },
    reg_defaults: cs42l73_reg_defaults.as_ptr(),
    num_reg_defaults: cs42l73_reg_defaults.len() as c_uint,
    volatile_reg: Some(cs42l73_volatile_register),
    readable_reg: Some(cs42l73_readable_register),
    cache_type: unsafe { REGCACHE_MAPLE },
    use_single_read: true,
    use_single_write: true,
};

unsafe extern "C" fn cs42l73_i2c_probe(i2c_client: *mut i2c_client) -> c_int {
    let mut ret: c_int;
    let mut reg: c_uint = 0;
    let mut val32: u32 = 0;
    let cs42l73 = devm_kzalloc(&mut (*i2c_client).dev, size_of::<cs42l73_private>(), GFP_KERNEL) as *mut cs42l73_private;
    if cs42l73.is_null() {
        return -ENOMEM;
    }
    (*cs42l73).regmap = devm_regmap_init_i2c(i2c_client, &cs42l73_regmap);
    if IS_ERR((*cs42l73).regmap as *const c_void) {
        ret = PTR_ERR((*cs42l73).regmap as *const c_void);
        dev_err((&mut (*i2c_client).dev) as *mut device_with_of as *mut device, b"regmap_init() failed: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    let pdata = devm_kzalloc(&mut (*i2c_client).dev, size_of::<cs42l73_platform_data>(), GFP_KERNEL) as *mut cs42l73_platform_data;
    if pdata.is_null() {
        return -ENOMEM;
    }
    if !(*i2c_client).dev.of_node.is_null() {
        if of_property_read_u32((*i2c_client).dev.of_node, b"chgfreq\0".as_ptr() as *const c_char, &mut val32) >= 0 {
            (*pdata).chgfreq = val32;
        }
    }
    (*pdata).reset_gpio = devm_gpiod_get_optional(&mut (*i2c_client).dev, b"reset\0".as_ptr() as *const c_char, GPIOD_OUT_LOW);
    if IS_ERR((*pdata).reset_gpio as *const c_void) {
        return PTR_ERR((*pdata).reset_gpio as *const c_void);
    }
    gpiod_set_consumer_name((*pdata).reset_gpio, b"CS42L73 /RST\0".as_ptr() as *const c_char);
    (*cs42l73).pdata = *pdata;
    i2c_set_clientdata(i2c_client, cs42l73 as *mut c_void);
    if !(*cs42l73).pdata.reset_gpio.is_null() {
        gpiod_set_value_cansleep((*cs42l73).pdata.reset_gpio, 1);
        gpiod_set_value_cansleep((*cs42l73).pdata.reset_gpio, 0);
    }
    /* initialize codec */
    let devid = cirrus_read_device_id((*cs42l73).regmap, CS42L73_DEVID_AB);
    if devid < 0 {
        ret = devid;
        dev_err((&mut (*i2c_client).dev) as *mut device_with_of as *mut device, b"Failed to read device ID: %d\n\0".as_ptr() as *const c_char, ret);
        gpiod_set_value_cansleep((*cs42l73).pdata.reset_gpio, 1);
        return ret;
    }
    if devid != CS42L73_DEVID {
        ret = -ENODEV;
        dev_err((&mut (*i2c_client).dev) as *mut device_with_of as *mut device, b"CS42L73 Device ID (%X). Expected %X\n\0".as_ptr() as *const c_char, devid, CS42L73_DEVID);
        gpiod_set_value_cansleep((*cs42l73).pdata.reset_gpio, 1);
        return ret;
    }
    ret = regmap_read((*cs42l73).regmap, CS42L73_REVID, &mut reg);
    if ret < 0 {
        dev_err((&mut (*i2c_client).dev) as *mut device_with_of as *mut device, b"Get Revision ID failed\n\0".as_ptr() as *const c_char);
        gpiod_set_value_cansleep((*cs42l73).pdata.reset_gpio, 1);
        return ret;
    }
    dev_info(&mut (*i2c_client).dev, b"Cirrus Logic CS42L73, Revision: %02X\n\0".as_ptr() as *const c_char, reg & 0xFF);
    ret = devm_snd_soc_register_component(&mut (*i2c_client).dev, &soc_component_dev_cs42l73, cs42l73_dai.as_mut_ptr(), 3);
    if ret < 0 {
        gpiod_set_value_cansleep((*cs42l73).pdata.reset_gpio, 1);
        return ret;
    }
    0
}

static cs42l73_of_match: [of_device_id; 2] = [
    of_device_id { compatible: b"cirrus,cs42l73\0".as_ptr() as *const c_char },
    of_device_id { compatible: ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, cs42l73_of_match); */

static cs42l73_id: [i2c_device_id; 2] = [
    i2c_device_id { name: b"cs42l73\0".as_ptr() as *const c_char },
    i2c_device_id { name: ptr::null() },
];
/* MODULE_DEVICE_TABLE(i2c, cs42l73_id); */

static mut cs42l73_i2c_driver: i2c_driver = i2c_driver {
    driver: driver_inner {
        name: b"cs42l73\0".as_ptr() as *const c_char,
        of_match_table: cs42l73_of_match.as_ptr(),
    },
    id_table: cs42l73_id.as_ptr(),
    probe: Some(cs42l73_i2c_probe),
};

/* module_i2c_driver(cs42l73_i2c_driver); */
/* MODULE_DESCRIPTION("ASoC CS42L73 driver"); */
/* MODULE_AUTHOR("Georgi Vlaev, Nucleus Systems Ltd, <joe@nucleusys.com>"); */
/* MODULE_AUTHOR("Brian Austin, Cirrus Logic Inc, <brian.austin@cirrus.com>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
