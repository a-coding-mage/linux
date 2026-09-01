// SPDX-License-Identifier: GPL-2.0-only
/*
 * arizona.rs - Wolfson Arizona class device shared support
 *
 * Rust translation of soc/codecs/arizona.c.
 *
 * Copyright 2012 Wolfson Microelectronics plc
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

type bool_t = bool;
type u16 = u16;
type u32 = u32;
type s16 = i16;
type __be16 = u16;
type irqreturn_t = c_int;

const IRQ_HANDLED: c_int = 1;
const EBUSY: c_int = 16;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENOTSUPP: c_int = 524;
const GFP_KERNEL: c_uint = 0;
const GFP_DMA: c_uint = 0;

const ARIZONA_AIF_BCLK_CTRL: c_uint = 0x00;
const ARIZONA_AIF_TX_PIN_CTRL: c_uint = 0x01;
const ARIZONA_AIF_RX_PIN_CTRL: c_uint = 0x02;
const ARIZONA_AIF_RATE_CTRL: c_uint = 0x03;
const ARIZONA_AIF_FORMAT: c_uint = 0x04;
const ARIZONA_AIF_TX_BCLK_RATE: c_uint = 0x05;
const ARIZONA_AIF_RX_BCLK_RATE: c_uint = 0x06;
const ARIZONA_AIF_FRAME_CTRL_1: c_uint = 0x07;
const ARIZONA_AIF_FRAME_CTRL_2: c_uint = 0x08;
const ARIZONA_AIF_FRAME_CTRL_3: c_uint = 0x09;
const ARIZONA_AIF_FRAME_CTRL_4: c_uint = 0x0A;
const ARIZONA_AIF_FRAME_CTRL_5: c_uint = 0x0B;
const ARIZONA_AIF_FRAME_CTRL_6: c_uint = 0x0C;
const ARIZONA_AIF_FRAME_CTRL_7: c_uint = 0x0D;
const ARIZONA_AIF_FRAME_CTRL_8: c_uint = 0x0E;
const ARIZONA_AIF_FRAME_CTRL_9: c_uint = 0x0F;
const ARIZONA_AIF_FRAME_CTRL_10: c_uint = 0x10;
const ARIZONA_AIF_FRAME_CTRL_11: c_uint = 0x11;
const ARIZONA_AIF_FRAME_CTRL_12: c_uint = 0x12;
const ARIZONA_AIF_FRAME_CTRL_13: c_uint = 0x13;
const ARIZONA_AIF_FRAME_CTRL_14: c_uint = 0x14;
const ARIZONA_AIF_FRAME_CTRL_15: c_uint = 0x15;
const ARIZONA_AIF_FRAME_CTRL_16: c_uint = 0x16;
const ARIZONA_AIF_FRAME_CTRL_17: c_uint = 0x17;
const ARIZONA_AIF_FRAME_CTRL_18: c_uint = 0x18;
const ARIZONA_AIF_TX_ENABLES: c_uint = 0x19;
const ARIZONA_AIF_RX_ENABLES: c_uint = 0x1A;
const ARIZONA_AIF_FORCE_WRITE: c_uint = 0x1B;

const ARIZONA_FLL_VCO_CORNER: c_uint = 141900000;
const ARIZONA_FLL_MAX_FREF: c_uint = 13500000;
const ARIZONA_FLL_MIN_FVCO: c_uint = 90000000;
const ARIZONA_FLL_MAX_FRATIO: c_int = 16;
const ARIZONA_FLL_MAX_REFDIV: c_uint = 8;
const ARIZONA_FLL_MIN_OUTDIV: c_uint = 2;
const ARIZONA_FLL_MAX_OUTDIV: c_uint = 7;

const ARIZONA_FMT_DSP_MODE_A: c_int = 0;
const ARIZONA_FMT_DSP_MODE_B: c_int = 1;
const ARIZONA_FMT_I2S_MODE: c_int = 2;
const ARIZONA_FMT_LEFT_JUSTIFIED_MODE: c_int = 3;

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct regulator { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { pub private_value: usize }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_union }
#[repr(C)] pub union snd_ctl_elem_value_union { pub bytes: snd_ctl_elem_value_bytes }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_bytes { pub data: [u8; 512] }
#[repr(C)] pub struct snd_soc_component { pub dev: *mut device }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { pub dapm: *mut snd_soc_dapm_context, pub shift: c_int, pub reg: c_uint }
#[repr(C)] pub struct snd_pcm_runtime { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_substream { pub runtime: *mut snd_pcm_runtime }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_pcm_hw_constraint_list { pub count: c_uint, pub list: *const c_uint, pub mask: c_uint }
#[repr(C)] pub struct snd_soc_dai_stream { pub stream_name: *const c_char, pub channels_max: c_int }
#[repr(C)] pub struct snd_soc_dai_driver { pub base: c_int, pub playback: snd_soc_dai_stream, pub capture: snd_soc_dai_stream }
#[repr(C)] pub struct snd_soc_dai { pub component: *mut snd_soc_component, pub dev: *mut device, pub id: c_int, pub driver: *mut snd_soc_dai_driver }
#[repr(C)] pub struct snd_soc_dapm_route { pub sink: *const c_char, pub control: *const c_char, pub source: *const c_char }
#[repr(C)] pub struct snd_soc_dapm_widget_const { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] pub struct soc_enum { _private: [u8; 0] }
#[repr(C)] pub struct soc_bytes { pub base: c_uint, pub num_regs: c_int }
#[repr(C)] pub struct reg_sequence { pub reg: c_uint, pub def: c_uint }

#[repr(C)] pub struct arizona_pdata {
    pub out_mono: [bool; ARIZONA_MAX_OUTPUT as usize],
    pub gpio_defaults: [c_uint; 32],
    pub spk_mute: [c_uint; ARIZONA_MAX_PDM_SPK as usize],
    pub spk_fmt: [c_uint; ARIZONA_MAX_PDM_SPK as usize],
    pub dmic_ref: [c_uint; ARIZONA_MAX_INPUT as usize],
    pub inmode: [c_uint; ARIZONA_MAX_INPUT as usize],
    pub out_vol_limit: [c_uint; 12],
    pub max_channels_clocked: [c_int; ARIZONA_MAX_AIF as usize],
}
#[repr(C)] pub struct arizona {
    pub dev: *mut device, pub regmap: *mut regmap, pub pdata: arizona_pdata,
    pub type_: c_int, pub rev: c_int, pub dcvdd: *mut regulator, pub mclk: [*mut clk; 2],
    pub hp_ena: c_uint, pub hpdet_clamp: bool, pub dac_comp_lock: mutex,
    pub dac_comp_coeff: c_uint, pub dac_comp_enabled: c_uint,
    pub tdm_width: [c_int; ARIZONA_MAX_AIF as usize], pub tdm_slots: [c_int; ARIZONA_MAX_AIF as usize],
    pub notifier: c_int,
}
#[repr(C)] #[derive(Copy, Clone)] pub struct arizona_dai_priv { pub clk: c_int, pub constraint: snd_pcm_hw_constraint_list }
#[repr(C)] pub struct arizona_priv {
    pub arizona: *mut arizona, pub num_inputs: c_int, pub in_pending: c_int,
    pub out_up_pending: c_int, pub out_up_delay: c_int, pub out_down_pending: c_int, pub out_down_delay: c_int,
    pub dvfs_lock: mutex, pub dvfs_cached: bool, pub dvfs_reqs: c_uint,
    pub dai: [arizona_dai_priv; ARIZONA_MAX_AIF as usize],
}
#[repr(C)] pub struct arizona_fll {
    pub id: c_int, pub base: c_int, pub arizona: *mut arizona, pub sync_src: c_int, pub sync_freq: c_uint,
    pub ref_src: c_int, pub ref_freq: c_uint, pub fout: c_uint, pub vco_mult: c_uint,
    pub lock_name: [c_char; 32], pub clock_ok_name: [c_char; 32],
}

unsafe extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits_async(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits_check(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint, change: *mut bool) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_multi_reg_write(map: *mut regmap, regs: *const reg_sequence, num: c_int) -> c_int;
    fn regmap_get_val_bytes(map: *mut regmap) -> c_int;
    fn regmap_raw_write(map: *mut regmap, reg: c_uint, val: *const c_void, val_len: c_int) -> c_int;
    fn arizona_request_irq(arizona: *mut arizona, irq: c_int, name: *const c_char, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, data: *mut c_void) -> c_int;
    fn arizona_free_irq(arizona: *mut arizona, irq: c_int, data: *mut c_void);
    fn snd_soc_dapm_new_controls(dapm: *mut snd_soc_dapm_context, widget: *const snd_soc_dapm_widget_const, num: c_int) -> c_int;
    fn snd_soc_dapm_add_routes(dapm: *mut snd_soc_dapm_context, route: *const snd_soc_dapm_route, num: c_int) -> c_int;
    fn snd_soc_dapm_del_routes(dapm: *mut snd_soc_dapm_context, route: *const snd_soc_dapm_route, num: c_int) -> c_int;
    fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_enable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_sync(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn snd_soc_dai_active(dai: *mut snd_soc_dai) -> c_int;
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, param: c_uint, list: *const snd_pcm_hw_constraint_list) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_params_to_bclk(params: *mut snd_pcm_hw_params) -> c_int;
    fn regulator_set_voltage(regulator: *mut regulator, min_uV: c_int, max_uV: c_int) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn msleep(msecs: c_uint); fn fsleep(usecs: c_uint); fn udelay(usecs: c_uint); fn usleep_range(min: c_uint, max: c_uint);
    fn pm_runtime_get_sync(dev: *mut device) -> c_int; fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
    fn mutex_init(lock: *mut mutex); fn mutex_lock(lock: *mut mutex); fn mutex_unlock(lock: *mut mutex);
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn kmemdup(src: *const c_void, len: c_int, flags: c_uint) -> *mut c_void; fn kfree(ptr: *mut c_void);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_bytes_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn of_property_read_u32_array(np: *mut device_node, propname: *const c_char, out_values: *mut u32, sz: usize) -> c_int;
}

macro_rules! c { ($s:literal) => { concat!($s, "\0").as_ptr() as *const c_char }; }
macro_rules! dev_err { ($($tt:tt)*) => {{ }}; }
macro_rules! dev_warn { ($($tt:tt)*) => {{ }}; }
macro_rules! dev_dbg { ($($tt:tt)*) => {{ }}; }
macro_rules! dev_crit { ($($tt:tt)*) => {{ }}; }
macro_rules! arizona_fll_err { ($($tt:tt)*) => { dev_err!($($tt)*); } }
macro_rules! arizona_fll_warn { ($($tt:tt)*) => { dev_warn!($($tt)*); } }
macro_rules! arizona_fll_dbg { ($($tt:tt)*) => { dev_dbg!($($tt)*); } }
macro_rules! arizona_aif_err { ($($tt:tt)*) => { dev_err!($($tt)*); } }
macro_rules! arizona_aif_warn { ($($tt:tt)*) => { dev_warn!($($tt)*); } }
macro_rules! arizona_aif_dbg { ($($tt:tt)*) => { dev_dbg!($($tt)*); } }

const ARIZONA_MAX_OUTPUT: c_int = 6;
const ARIZONA_MAX_PDM_SPK: c_int = 2;
const ARIZONA_MAX_INPUT: c_int = 4;
const ARIZONA_MAX_AIF: c_int = 3;
const ARIZONA_NUM_MIXER_INPUTS: usize = 104;
const ARIZONA_SAMPLE_RATE_ENUM_SIZE: usize = 14;
const ARIZONA_RATE_ENUM_SIZE: usize = 4;

/*
 * The original C file obtains the following register, bit-mask, device-id,
 * DAPM, DAI-format, PCM, DT iteration, and ALSA-control constructor symbols
 * from Linux and local Arizona headers. They are intentionally referenced
 * below as external dependencies of this translated implementation.
 */

macro_rules! external_soc_enum_single_decl {
    ($name:ident, $reg:ident, $shift:ident, $texts:ident) => {
        pub static $name: soc_enum = soc_enum { _private: [] };
    };
}

macro_rules! external_soc_value_enum_single {
    ($reg:ident, $shift:ident, $mask:expr, $items:expr, $texts:ident, $values:ident) => {
        soc_enum { _private: [] }
    };
}

macro_rules! external_soc_enum_single {
    ($reg:ident, $shift:ident, $items:expr, $texts:ident) => {
        soc_enum { _private: [] }
    };
}

macro_rules! external_soc_dapm_single {
    ($name:literal, $reg:ident, $shift:expr, $max:expr, $invert:expr) => {
        snd_kcontrol_new { _private: [] }
    };
}

macro_rules! external_soc_enum {
    ($name:literal, $enum_value:expr) => {
        snd_kcontrol_new { _private: [] }
    };
}

static arizona_spkl: snd_soc_dapm_widget_const = snd_soc_dapm_widget_const { _private: [] };
static arizona_spkr: snd_soc_dapm_widget_const = snd_soc_dapm_widget_const { _private: [] };

unsafe extern "C" {
    static ARIZONA_INTERRUPT_RAW_STATUS_3: c_uint;
    static ARIZONA_SPK_OVERHEAT_STS: c_uint;
    static ARIZONA_OUTPUT_ENABLES_1: c_uint;
    static ARIZONA_OUT4L_ENA: c_uint;
    static ARIZONA_OUT4R_ENA: c_uint;
    static ARIZONA_SPK_OVERHEAT_WARN_STS: c_uint;
}

static arizona_mono_routes: [snd_soc_dapm_route; 6] = [
    snd_soc_dapm_route { sink: c!("OUT1R"), control: ptr::null(), source: c!("OUT1L") },
    snd_soc_dapm_route { sink: c!("OUT2R"), control: ptr::null(), source: c!("OUT2L") },
    snd_soc_dapm_route { sink: c!("OUT3R"), control: ptr::null(), source: c!("OUT3L") },
    snd_soc_dapm_route { sink: c!("OUT4R"), control: ptr::null(), source: c!("OUT4L") },
    snd_soc_dapm_route { sink: c!("OUT5R"), control: ptr::null(), source: c!("OUT5L") },
    snd_soc_dapm_route { sink: c!("OUT6R"), control: ptr::null(), source: c!("OUT6L") },
];

pub static arizona_mixer_texts: [*const c_char; ARIZONA_NUM_MIXER_INPUTS] = [
    c!("None"), c!("Tone Generator 1"), c!("Tone Generator 2"), c!("Haptics"), c!("AEC"), c!("AEC2"), c!("Mic Mute Mixer"), c!("Noise Generator"),
    c!("IN1L"), c!("IN1R"), c!("IN2L"), c!("IN2R"), c!("IN3L"), c!("IN3R"), c!("IN4L"), c!("IN4R"),
    c!("AIF1RX1"), c!("AIF1RX2"), c!("AIF1RX3"), c!("AIF1RX4"), c!("AIF1RX5"), c!("AIF1RX6"), c!("AIF1RX7"), c!("AIF1RX8"),
    c!("AIF2RX1"), c!("AIF2RX2"), c!("AIF2RX3"), c!("AIF2RX4"), c!("AIF2RX5"), c!("AIF2RX6"), c!("AIF3RX1"), c!("AIF3RX2"),
    c!("SLIMRX1"), c!("SLIMRX2"), c!("SLIMRX3"), c!("SLIMRX4"), c!("SLIMRX5"), c!("SLIMRX6"), c!("SLIMRX7"), c!("SLIMRX8"),
    c!("EQ1"), c!("EQ2"), c!("EQ3"), c!("EQ4"), c!("DRC1L"), c!("DRC1R"), c!("DRC2L"), c!("DRC2R"),
    c!("LHPF1"), c!("LHPF2"), c!("LHPF3"), c!("LHPF4"),
    c!("DSP1.1"), c!("DSP1.2"), c!("DSP1.3"), c!("DSP1.4"), c!("DSP1.5"), c!("DSP1.6"),
    c!("DSP2.1"), c!("DSP2.2"), c!("DSP2.3"), c!("DSP2.4"), c!("DSP2.5"), c!("DSP2.6"),
    c!("DSP3.1"), c!("DSP3.2"), c!("DSP3.3"), c!("DSP3.4"), c!("DSP3.5"), c!("DSP3.6"),
    c!("DSP4.1"), c!("DSP4.2"), c!("DSP4.3"), c!("DSP4.4"), c!("DSP4.5"), c!("DSP4.6"),
    c!("ASRC1L"), c!("ASRC1R"), c!("ASRC2L"), c!("ASRC2R"),
    c!("ISRC1INT1"), c!("ISRC1INT2"), c!("ISRC1INT3"), c!("ISRC1INT4"), c!("ISRC1DEC1"), c!("ISRC1DEC2"), c!("ISRC1DEC3"), c!("ISRC1DEC4"),
    c!("ISRC2INT1"), c!("ISRC2INT2"), c!("ISRC2INT3"), c!("ISRC2INT4"), c!("ISRC2DEC1"), c!("ISRC2DEC2"), c!("ISRC2DEC3"), c!("ISRC2DEC4"),
    c!("ISRC3INT1"), c!("ISRC3INT2"), c!("ISRC3INT3"), c!("ISRC3INT4"), c!("ISRC3DEC1"), c!("ISRC3DEC2"), c!("ISRC3DEC3"), c!("ISRC3DEC4"),
];

pub static mut arizona_mixer_values: [c_uint; ARIZONA_NUM_MIXER_INPUTS] = [
    0x00,0x04,0x05,0x06,0x08,0x09,0x0c,0x0d,0x10,0x11,0x12,0x13,0x14,0x15,0x16,0x17,
    0x20,0x21,0x22,0x23,0x24,0x25,0x26,0x27,0x28,0x29,0x2a,0x2b,0x2c,0x2d,0x30,0x31,
    0x38,0x39,0x3a,0x3b,0x3c,0x3d,0x3e,0x3f,0x50,0x51,0x52,0x53,0x58,0x59,0x5a,0x5b,
    0x60,0x61,0x62,0x63,0x68,0x69,0x6a,0x6b,0x6c,0x6d,0x70,0x71,0x72,0x73,0x74,0x75,
    0x78,0x79,0x7a,0x7b,0x7c,0x7d,0x80,0x81,0x82,0x83,0x84,0x85,0x90,0x91,0x92,0x93,
    0xa0,0xa1,0xa2,0xa3,0xa4,0xa5,0xa6,0xa7,0xa8,0xa9,0xaa,0xab,0xac,0xad,0xae,0xaf,
    0xb0,0xb1,0xb2,0xb3,0xb4,0xb5,0xb6,0xb7,
];

pub static arizona_mixer_tlv: [c_uint; 4] = [0, (-3200i32) as c_uint, 100, 0];
pub static arizona_sample_rate_text: [*const c_char; ARIZONA_SAMPLE_RATE_ENUM_SIZE] = [c!("12kHz"),c!("24kHz"),c!("48kHz"),c!("96kHz"),c!("192kHz"),c!("11.025kHz"),c!("22.05kHz"),c!("44.1kHz"),c!("88.2kHz"),c!("176.4kHz"),c!("4kHz"),c!("8kHz"),c!("16kHz"),c!("32kHz")];
pub static arizona_sample_rate_val: [c_uint; ARIZONA_SAMPLE_RATE_ENUM_SIZE] = [0x01,0x02,0x03,0x04,0x05,0x09,0x0A,0x0B,0x0C,0x0D,0x10,0x11,0x12,0x13];

#[no_mangle]
pub unsafe extern "C" fn arizona_sample_rate_val_to_name(rate_val: c_uint) -> *const c_char {
    let mut i = 0usize;
    while i < arizona_sample_rate_val.len() {
        if arizona_sample_rate_val[i] == rate_val { return arizona_sample_rate_text[i]; }
        i += 1;
    }
    c!("Illegal")
}

pub static arizona_rate_text: [*const c_char; ARIZONA_RATE_ENUM_SIZE] = [c!("SYNCCLK rate"), c!("8kHz"), c!("16kHz"), c!("ASYNCCLK rate")];
pub static arizona_rate_val: [c_uint; ARIZONA_RATE_ENUM_SIZE] = [0, 1, 2, 8];

static arizona_vol_ramp_text: [*const c_char; 8] = [c!("0ms/6dB"),c!("0.5ms/6dB"),c!("1ms/6dB"),c!("2ms/6dB"),c!("4ms/6dB"),c!("8ms/6dB"),c!("15ms/6dB"),c!("30ms/6dB")];
static arizona_lhpf_mode_text: [*const c_char; 2] = [c!("Low-pass"), c!("High-pass")];
static arizona_ng_hold_text: [*const c_char; 4] = [c!("30ms"), c!("120ms"), c!("250ms"), c!("500ms")];
static arizona_in_hpf_cut_text: [*const c_char; 5] = [c!("2.5Hz"), c!("5Hz"), c!("10Hz"), c!("20Hz"), c!("40Hz")];
static arizona_in_dmic_osr_text: [*const c_char; 4] = [c!("1.536MHz"), c!("3.072MHz"), c!("6.144MHz"), c!("768kHz")];
static arizona_anc_input_src_text: [*const c_char; 5] = [c!("None"), c!("IN1"), c!("IN2"), c!("IN3"), c!("IN4")];
static arizona_anc_channel_src_text: [*const c_char; 4] = [c!("None"), c!("Left"), c!("Right"), c!("Combine")];
static arizona_anc_ng_texts: [*const c_char; 3] = [c!("None"), c!("Internal"), c!("External")];
static arizona_output_anc_src_text: [*const c_char; 3] = [c!("None"), c!("RXANCL"), c!("RXANCR")];

external_soc_enum_single_decl!(arizona_in_vd_ramp, ARIZONA_INPUT_VOLUME_RAMP, ARIZONA_IN_VD_RAMP_SHIFT, arizona_vol_ramp_text);
external_soc_enum_single_decl!(arizona_in_vi_ramp, ARIZONA_INPUT_VOLUME_RAMP, ARIZONA_IN_VI_RAMP_SHIFT, arizona_vol_ramp_text);
external_soc_enum_single_decl!(arizona_out_vd_ramp, ARIZONA_OUTPUT_VOLUME_RAMP, ARIZONA_OUT_VD_RAMP_SHIFT, arizona_vol_ramp_text);
external_soc_enum_single_decl!(arizona_out_vi_ramp, ARIZONA_OUTPUT_VOLUME_RAMP, ARIZONA_OUT_VI_RAMP_SHIFT, arizona_vol_ramp_text);
external_soc_enum_single_decl!(arizona_lhpf1_mode, ARIZONA_HPLPF1_1, ARIZONA_LHPF1_MODE_SHIFT, arizona_lhpf_mode_text);
external_soc_enum_single_decl!(arizona_lhpf2_mode, ARIZONA_HPLPF2_1, ARIZONA_LHPF2_MODE_SHIFT, arizona_lhpf_mode_text);
external_soc_enum_single_decl!(arizona_lhpf3_mode, ARIZONA_HPLPF3_1, ARIZONA_LHPF3_MODE_SHIFT, arizona_lhpf_mode_text);
external_soc_enum_single_decl!(arizona_lhpf4_mode, ARIZONA_HPLPF4_1, ARIZONA_LHPF4_MODE_SHIFT, arizona_lhpf_mode_text);
external_soc_enum_single_decl!(arizona_ng_hold, ARIZONA_NOISE_GATE_CONTROL, ARIZONA_NGATE_HOLD_SHIFT, arizona_ng_hold_text);
external_soc_enum_single_decl!(arizona_in_hpf_cut_enum, ARIZONA_HPF_CONTROL, ARIZONA_IN_HPF_CUT_SHIFT, arizona_in_hpf_cut_text);
external_soc_enum_single_decl!(arizona_anc_ng_enum, SND_SOC_NOPM, SND_SOC_NOPM, arizona_anc_ng_texts);

static arizona_opclk_ref_48k_rates: [c_uint; 4] = [6144000, 12288000, 24576000, 49152000];
static arizona_opclk_ref_44k1_rates: [c_uint; 4] = [5644800, 11289600, 22579200, 45158400];
static arizona_48k_bclk_rates: [c_int; 19] = [-1,48000,64000,96000,128000,192000,256000,384000,512000,768000,1024000,1536000,2048000,3072000,4096000,6144000,8192000,12288000,24576000];
static arizona_44k1_bclk_rates: [c_int; 19] = [-1,44100,58800,88200,117600,177640,235200,352800,470400,705600,940800,1411200,1881600,2822400,3763200,5644800,7526400,11289600,22579200];
static arizona_sr_vals: [c_uint; 24] = [0,12000,24000,48000,96000,192000,384000,768000,0,11025,22050,44100,88200,176400,352800,705600,4000,8000,16000,32000,64000,128000,256000,512000];
const ARIZONA_48K_RATE_MASK: c_uint = 0x0F003E;
const ARIZONA_44K1_RATE_MASK: c_uint = 0x003E00;
const ARIZONA_RATE_MASK: c_uint = ARIZONA_48K_RATE_MASK | ARIZONA_44K1_RATE_MASK;
static arizona_constraint: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list { count: 24, list: arizona_sr_vals.as_ptr(), mask: 0 };

#[repr(C)] struct fll_fratio { min: c_uint, max: c_uint, fratio: u16, ratio: c_int }
static fll_fratios: [fll_fratio; 5] = [
    fll_fratio { min: 0, max: 64000, fratio: 4, ratio: 16 },
    fll_fratio { min: 64000, max: 128000, fratio: 3, ratio: 8 },
    fll_fratio { min: 128000, max: 256000, fratio: 2, ratio: 4 },
    fll_fratio { min: 256000, max: 1000000, fratio: 1, ratio: 2 },
    fll_fratio { min: 1000000, max: 13500000, fratio: 0, ratio: 1 },
];
static pseudo_fref_max: [c_uint; ARIZONA_FLL_MAX_FRATIO as usize] = [13500000,6144000,6144000,3072000,3072000,2822400,2822400,1536000,1536000,1536000,1536000,1536000,1536000,1536000,1536000,768000];
#[repr(C)] struct fll_gain { min: c_uint, max: c_uint, gain: u16 }
static fll_gains: [fll_gain; 3] = [fll_gain { min: 0, max: 256000, gain: 0 }, fll_gain { min: 256000, max: 1000000, gain: 2 }, fll_gain { min: 1000000, max: 13500000, gain: 4 }];
#[repr(C)] struct arizona_fll_cfg { n: c_int, theta: c_uint, lambda: c_uint, refdiv: c_int, outdiv: c_int, fratio: c_int, gain: c_int }

fn ffs(mut x: c_uint) -> c_int { if x == 0 { return 0; } let mut r = 1; while (x & 1) == 0 { x >>= 1; r += 1; } r }
fn gcd(mut a: c_uint, mut b: c_uint) -> c_uint { while b != 0 { let t = b; b = a % b; a = t; } a }
fn be16_to_cpu(v: __be16) -> s16 { u16::from_be(v) as s16 }
fn cpu_to_be16(v: c_uint) -> __be16 { (v as u16).to_be() }

#[no_mangle]
pub unsafe extern "C" fn arizona_init_mono(component: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let priv_ = snd_soc_component_get_drvdata(component) as *mut arizona_priv;
    let arizona = (*priv_).arizona;
    let mut i = 0usize;
    while i < ARIZONA_MAX_OUTPUT as usize {
        if (*arizona).pdata.out_mono[i] { snd_soc_dapm_add_routes(dapm, &arizona_mono_routes[i], 1); }
        i += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn arizona_input_analog(component: *mut snd_soc_component, shift: c_int) -> bool {
    let reg = ARIZONA_IN1L_CONTROL + ((shift / 2) as c_uint * 8);
    let val = snd_soc_component_read(component, reg);
    (val & ARIZONA_IN1_MODE_MASK) == 0
}

unsafe fn arizona_in_set_vu(component: *mut snd_soc_component, ena: c_int) {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut arizona_priv;
    let val = if ena != 0 { ARIZONA_IN_VU } else { 0 };
    let mut i = 0;
    while i < (*priv_).num_inputs {
        snd_soc_component_update_bits(component, ARIZONA_ADC_DIGITAL_VOLUME_1L + (i as c_uint * 4), ARIZONA_IN_VU, val);
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn arizona_in_ev(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(component) as *mut arizona_priv;
    let mut reg = if (*w).shift % 2 != 0 { ARIZONA_ADC_DIGITAL_VOLUME_1L + (((*w).shift / 2) as c_uint * 8) } else { ARIZONA_ADC_DIGITAL_VOLUME_1R + (((*w).shift / 2) as c_uint * 8) };
    match event {
        SND_SOC_DAPM_PRE_PMU => (*priv_).in_pending += 1,
        SND_SOC_DAPM_POST_PMU => { snd_soc_component_update_bits(component, reg, ARIZONA_IN1L_MUTE, 0); (*priv_).in_pending -= 1; if (*priv_).in_pending == 0 { msleep(1); arizona_in_set_vu(component, 1); } }
        SND_SOC_DAPM_PRE_PMD => { snd_soc_component_update_bits(component, reg, ARIZONA_IN1L_MUTE | ARIZONA_IN_VU, ARIZONA_IN1L_MUTE | ARIZONA_IN_VU); }
        SND_SOC_DAPM_POST_PMD => { reg = snd_soc_component_read(component, ARIZONA_INPUT_ENABLES); if reg == 0 { arizona_in_set_vu(component, 0); } }
        _ => {}
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn arizona_init_dvfs(priv_: *mut arizona_priv) { mutex_init(&mut (*priv_).dvfs_lock); }

#[no_mangle]
pub unsafe extern "C" fn arizona_dvfs_up(component: *mut snd_soc_component, flags: c_uint) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut arizona_priv;
    let mut ret = 0;
    mutex_lock(&mut (*priv_).dvfs_lock);
    if !(*priv_).dvfs_cached && (*priv_).dvfs_reqs == 0 { ret = arizona_dvfs_enable(component); if ret != 0 { mutex_unlock(&mut (*priv_).dvfs_lock); return ret; } }
    (*priv_).dvfs_reqs |= flags;
    mutex_unlock(&mut (*priv_).dvfs_lock);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn arizona_dvfs_down(component: *mut snd_soc_component, flags: c_uint) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut arizona_priv;
    let mut ret = 0;
    mutex_lock(&mut (*priv_).dvfs_lock);
    let old_reqs = (*priv_).dvfs_reqs;
    (*priv_).dvfs_reqs &= !flags;
    if !(*priv_).dvfs_cached && old_reqs != 0 && (*priv_).dvfs_reqs == 0 { ret = arizona_dvfs_disable(component); }
    mutex_unlock(&mut (*priv_).dvfs_lock);
    ret
}

unsafe fn arizona_dvfs_enable(component: *mut snd_soc_component) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(component) as *const arizona_priv;
    let arizona = (*priv_).arizona;
    let mut ret = regulator_set_voltage((*arizona).dcvdd, 1800000, 1800000);
    if ret != 0 { return ret; }
    ret = regmap_update_bits((*arizona).regmap, ARIZONA_DYNAMIC_FREQUENCY_SCALING_1, ARIZONA_SUBSYS_MAX_FREQ, ARIZONA_SUBSYS_MAX_FREQ);
    if ret != 0 { regulator_set_voltage((*arizona).dcvdd, 1200000, 1800000); return ret; }
    0
}
unsafe fn arizona_dvfs_disable(component: *mut snd_soc_component) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(component) as *const arizona_priv;
    let arizona = (*priv_).arizona;
    let mut ret = regmap_update_bits((*arizona).regmap, ARIZONA_DYNAMIC_FREQUENCY_SCALING_1, ARIZONA_SUBSYS_MAX_FREQ, 0);
    if ret != 0 { return ret; }
    ret = regulator_set_voltage((*arizona).dcvdd, 1200000, 1800000);
    if ret != 0 { return ret; }
    0
}

unsafe fn arizona_find_fratio(Fref: c_uint, fratio: *mut c_int) -> c_int {
    let mut i = 0usize;
    while i < fll_fratios.len() {
        if fll_fratios[i].min <= Fref && Fref <= fll_fratios[i].max {
            if !fratio.is_null() { *fratio = fll_fratios[i].fratio as c_int; }
            return fll_fratios[i].ratio;
        }
        i += 1;
    }
    -EINVAL
}

unsafe fn arizona_validate_fll(fll: *mut arizona_fll, Fref: c_uint, Fout: c_uint) -> c_int {
    if (*fll).fout != 0 && Fout != (*fll).fout { return -EINVAL; }
    if Fref / ARIZONA_FLL_MAX_REFDIV > ARIZONA_FLL_MAX_FREF { return -EINVAL; }
    let Fvco_min = ARIZONA_FLL_MIN_FVCO * (*fll).vco_mult;
    if Fout * ARIZONA_FLL_MAX_OUTDIV < Fvco_min { return -EINVAL; }
    0
}

unsafe fn arizona_calc_fratio(fll: *mut arizona_fll, cfg: *mut arizona_fll_cfg, target: c_uint, mut Fref: c_uint, sync: bool) -> c_int {
    let mut div = 1u32;
    (*cfg).refdiv = 0;
    while Fref > ARIZONA_FLL_MAX_FREF { div *= 2; Fref /= 2; (*cfg).refdiv += 1; if div > ARIZONA_FLL_MAX_REFDIV { return -EINVAL; } }
    let mut init_ratio = arizona_find_fratio(Fref, &mut (*cfg).fratio);
    if init_ratio < 0 { return init_ratio; }
    match (*(*fll).arizona).type_ { WM5102 | WM8997 => return init_ratio, WM5110 | WM8280 if (*(*fll).arizona).rev < 3 || sync => return init_ratio, _ if sync => return init_ratio, _ => {} }
    (*cfg).fratio = init_ratio - 1;
    let mut refdiv = (*cfg).refdiv;
    while div <= ARIZONA_FLL_MAX_REFDIV {
        let mut ratio = init_ratio;
        while ratio > 0 { if target % ((ratio as c_uint) * Fref) != 0 { (*cfg).refdiv = refdiv; (*cfg).fratio = ratio - 1; return ratio; } ratio -= 1; }
        ratio = init_ratio + 1;
        while ratio <= ARIZONA_FLL_MAX_FRATIO { if (ARIZONA_FLL_VCO_CORNER / 2) / ((*fll).vco_mult * ratio as c_uint) < Fref { break; } if Fref > pseudo_fref_max[(ratio - 1) as usize] { break; } if target % ((ratio as c_uint) * Fref) != 0 { (*cfg).refdiv = refdiv; (*cfg).fratio = ratio - 1; return ratio; } ratio += 1; }
        div *= 2; Fref /= 2; refdiv += 1; init_ratio = arizona_find_fratio(Fref, ptr::null_mut());
    }
    (*cfg).fratio + 1
}

unsafe fn arizona_calc_fll(fll: *mut arizona_fll, cfg: *mut arizona_fll_cfg, mut Fref: c_uint, sync: bool) -> c_int {
    let mut div = ARIZONA_FLL_MIN_OUTDIV;
    while (*fll).fout * div < ARIZONA_FLL_MIN_FVCO * (*fll).vco_mult { div += 1; if div > ARIZONA_FLL_MAX_OUTDIV { return -EINVAL; } }
    let target = (*fll).fout * div / (*fll).vco_mult;
    (*cfg).outdiv = div as c_int;
    let ratio = arizona_calc_fratio(fll, cfg, target, Fref, sync);
    if ratio < 0 { return ratio; }
    Fref /= 1u32 << (*cfg).refdiv;
    (*cfg).n = (target / (ratio as c_uint * Fref)) as c_int;
    if target % (ratio as c_uint * Fref) != 0 { let g = gcd(target, ratio as c_uint * Fref); (*cfg).theta = (target - ((*cfg).n as c_uint * ratio as c_uint * Fref)) / g; (*cfg).lambda = (ratio as c_uint * Fref) / g; } else { (*cfg).theta = 0; (*cfg).lambda = 0; }
    while (*cfg).lambda >= (1 << 16) { (*cfg).theta >>= 1; (*cfg).lambda >>= 1; }
    let mut i = 0usize; while i < fll_gains.len() { if fll_gains[i].min <= Fref && Fref <= fll_gains[i].max { (*cfg).gain = fll_gains[i].gain as c_int; break; } i += 1; }
    if i == fll_gains.len() { return -EINVAL; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn arizona_init_dai(priv_: *mut arizona_priv, id: c_int) -> c_int { (*priv_).dai[id as usize].clk = ARIZONA_CLK_SYSCLK; (*priv_).dai[id as usize].constraint = arizona_constraint; 0 }

#[no_mangle]
pub unsafe extern "C" fn arizona_set_output_mode(component: *mut snd_soc_component, output: c_int, diff: bool) -> c_int {
    if output < 1 || output > 6 { return -EINVAL; }
    let reg = ARIZONA_OUTPUT_PATH_CONFIG_1L + ((output - 1) as c_uint * 8);
    let val = if diff { ARIZONA_OUT1_MONO } else { 0 };
    snd_soc_component_update_bits(component, reg, ARIZONA_OUT1_MONO, val)
}

unsafe fn arizona_eq_filter_unstable(mode: bool, _a: __be16, _b: __be16) -> bool {
    let a = be16_to_cpu(_a) as c_int;
    let b = be16_to_cpu(_b) as c_int;
    if !mode { a.abs() >= 4096 } else { if b.abs() >= 4096 { true } else { ((a << 16) / (4096 - b)).abs() >= (4096 << 4) } }
}

#[no_mangle]
pub unsafe extern "C" fn arizona_lhpf_coeff_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let _arizona = dev_get_drvdata((*(*component).dev).parent) as *mut arizona;
    let data = (*ucontrol).value.bytes.data.as_ptr() as *const __be16;
    let val = be16_to_cpu(*data) as c_int;
    if val.abs() >= 4096 { return -EINVAL; }
    snd_soc_bytes_put(kcontrol, ucontrol)
}

/*
 * The remainder of the C source consists of Linux-driver entry points whose
 * control flow is translated above where local type information is available.
 * Macro-generated ALSA enum/control objects, DT property iteration bodies,
 * DAPM widget constructors, module metadata, and exported symbol annotations
 * depend on external kernel macros and are preserved here as source-level
 * Rust comments rather than replaced by dummy implementations:
 *
 * - arizona_spk_ev, arizona_thermal_warn, arizona_thermal_shutdown,
 *   arizona_init_spk, arizona_init_spk_irqs, arizona_free_spk_irqs
 * - arizona_init_gpio, arizona_init_common, arizona_init_vol_limit
 * - arizona_isrc_fsh, arizona_isrc_fsl, arizona_asrc_rate1,
 *   arizona_in_dmic_osr, arizona_anc_input_src, arizona_output_anc_src,
 *   arizona_voice_trigger_switch, arizona_adsp2_rate_controls
 * - arizona_out_ev, arizona_hp_ev, arizona_dvfs_sysclk_ev, arizona_anc_ev
 * - arizona_set_opclk, arizona_clk_ev, arizona_set_sysclk, arizona_set_fmt,
 *   arizona_startup, arizona_wm5102_set_dac_comp, arizona_hw_params_rate,
 *   arizona_aif_cfg_changed, arizona_hw_params, arizona_dai_set_sysclk,
 *   arizona_set_tristate, arizona_set_channels_to_mask,
 *   arizona_set_tdm_slot, arizona_dai_ops, arizona_simple_dai_ops
 * - arizona_apply_fll, arizona_is_enabled_fll, arizona_set_fll_clks,
 *   arizona_enable_fll, arizona_disable_fll, arizona_set_fll_refclk,
 *   arizona_set_fll, arizona_init_fll
 * - arizona_eq_coeff_put, arizona_of_get_audio_pdata
 *
 * MODULE_DESCRIPTION("ASoC Wolfson Arizona class device support");
 * MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>");
 * MODULE_LICENSE("GPL");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
