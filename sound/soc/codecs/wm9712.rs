// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * wm9712.c  --  ALSA Soc WM9712 codec support
 *
 * Copyright 2006-12 Wolfson Microelectronics PLC.
 * Author: Liam Girdwood <lrg@slimlogic.co.uk>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const WM9712_VENDOR_ID: c_uint = 0x574d4c12;
const WM9712_VENDOR_ID_MASK: c_uint = 0xffffffff;

#[repr(C)]
pub struct snd_ac97 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wm97xx_platform_data {
    pub ac97: *mut snd_ac97,
    pub regmap: *mut regmap,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: usize,
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
    pub value: [i64; 128],
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
pub struct soc_mixer_control {
    pub shift: c_uint,
}

#[repr(C)]
pub struct snd_soc_dapm_update {
    pub kcontrol: *mut snd_kcontrol,
    pub reg: c_uint,
    pub mask: c_uint,
    pub val: c_uint,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub rate: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub reg_stride: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub cache_type: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: usize,
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
pub struct snd_soc_dapm_widget {
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
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
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
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: usize,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: usize,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: usize,
    pub suspend_bias_off: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON,
    SND_SOC_BIAS_PREPARE,
    SND_SOC_BIAS_STANDBY,
    SND_SOC_BIAS_OFF,
}

#[repr(C)]
pub struct wm9712_priv {
    pub ac97: *mut snd_ac97,
    pub hp_mixer: [c_uint; 2],
    pub lock: mutex,
    pub mfd_pdata: *mut wm97xx_platform_data,
}

extern "C" {
    static mut AC97_REC_GAIN: c_uint;
    static mut AC97_PCI_SVID: c_uint;
    static mut AC97_VIDEO: c_uint;
    static mut AC97_AUX: c_uint;
    static mut AC97_REC_SEL: c_uint;
    static mut AC97_MASTER_TONE: c_uint;
    static mut AC97_MIC: c_uint;
    static mut AC97_MASTER: c_uint;
    static mut AC97_HEADPHONE: c_uint;
    static mut AC97_PCM: c_uint;
    static mut AC97_MASTER_MONO: c_uint;
    static mut AC97_CODEC_CLASS_REV: c_uint;
    static mut AC97_PC_BEEP: c_uint;
    static mut AC97_CD: c_uint;
    static mut AC97_PHONE: c_uint;
    static mut AC97_LINE: c_uint;
    static mut AC97_3D_CONTROL: c_uint;
    static mut AC97_INT_PAGING: c_uint;
    static mut AC97_EXTENDED_STATUS: c_uint;
    static mut AC97_PCM_FRONT_DAC_RATE: c_uint;
    static mut AC97_PCM_LR_ADC_RATE: c_uint;
    static mut AC97_PCI_SID: c_uint;
    static mut AC97_PCM_SURR_DAC_RATE: c_uint;
    static mut AC97_POWERDOWN: c_uint;
    static mut AC97_EXTENDED_MSTATUS: c_uint;
    static mut REGCACHE_MAPLE: c_uint;
    static mut SND_SOC_NOPM: c_uint;
    static mut SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static mut SNDRV_PCM_RATE_8000: c_uint;
    static mut SNDRV_PCM_RATE_11025: c_uint;
    static mut SNDRV_PCM_RATE_22050: c_uint;
    static mut SNDRV_PCM_RATE_44100: c_uint;
    static mut SNDRV_PCM_RATE_48000: c_uint;
    static mut SND_SOC_STD_AC97_FMTS: c_uint;
    static mut GFP_KERNEL: c_uint;

    fn regmap_ac97_default_volatile(dev: *mut device, reg: c_uint) -> bool;
    fn snd_soc_dapm_kcontrol_to_dapm(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dapm_mixer_update_power(
        dapm: *mut snd_soc_dapm_context,
        kcontrol: *mut snd_kcontrol,
        val: c_uint,
        update: *mut snd_soc_dapm_update,
    );
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_ac97_reset(ac97: *mut snd_ac97, try_warm: bool, id: c_uint, id_mask: c_uint) -> c_int;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_force_bias_level(dapm: *mut snd_soc_dapm_context, level: snd_soc_bias_level);
    fn snd_soc_component_cache_sync(component: *mut snd_soc_component) -> c_int;
    fn snd_soc_new_ac97_component(
        component: *mut snd_soc_component,
        id: c_uint,
        id_mask: c_uint,
    ) -> *mut snd_ac97;
    fn regmap_init_ac97(ac97: *mut snd_ac97, config: *const regmap_config) -> *mut regmap;
    fn snd_soc_free_ac97_component(ac97: *mut snd_ac97);
    fn snd_soc_component_init_regmap(component: *mut snd_soc_component, regmap: *mut regmap);
    fn snd_soc_component_exit_regmap(component: *mut snd_soc_component);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn mutex_init(lock: *mut mutex);
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: usize,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

extern "Rust" {
    fn IS_ENABLED(option: c_uint) -> bool;
    static CONFIG_SND_SOC_AC97_BUS: c_uint;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
}

const ENODEV: c_int = 19;
const ENXIO: c_int = 6;
const ENOMEM: c_int = 12;

macro_rules! c_str {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

static WM9712_REG_DEFAULTS: [reg_default; 36] = [
    reg_default { reg: 0x02, def: 0x8000 },
    reg_default { reg: 0x04, def: 0x8000 },
    reg_default { reg: 0x06, def: 0x8000 },
    reg_default { reg: 0x08, def: 0x0f0f },
    reg_default { reg: 0x0a, def: 0xaaa0 },
    reg_default { reg: 0x0c, def: 0xc008 },
    reg_default { reg: 0x0e, def: 0x6808 },
    reg_default { reg: 0x10, def: 0xe808 },
    reg_default { reg: 0x12, def: 0xaaa0 },
    reg_default { reg: 0x14, def: 0xad00 },
    reg_default { reg: 0x16, def: 0x8000 },
    reg_default { reg: 0x18, def: 0xe808 },
    reg_default { reg: 0x1a, def: 0x3000 },
    reg_default { reg: 0x1c, def: 0x8000 },
    reg_default { reg: 0x20, def: 0x0000 },
    reg_default { reg: 0x22, def: 0x0000 },
    reg_default { reg: 0x26, def: 0x000f },
    reg_default { reg: 0x28, def: 0x0605 },
    reg_default { reg: 0x2a, def: 0x0410 },
    reg_default { reg: 0x2c, def: 0xbb80 },
    reg_default { reg: 0x2e, def: 0xbb80 },
    reg_default { reg: 0x32, def: 0xbb80 },
    reg_default { reg: 0x34, def: 0x2000 },
    reg_default { reg: 0x4c, def: 0xf83e },
    reg_default { reg: 0x4e, def: 0xffff },
    reg_default { reg: 0x50, def: 0x0000 },
    reg_default { reg: 0x52, def: 0x0000 },
    reg_default { reg: 0x56, def: 0xf83e },
    reg_default { reg: 0x58, def: 0x0008 },
    reg_default { reg: 0x5c, def: 0x0000 },
    reg_default { reg: 0x60, def: 0xb032 },
    reg_default { reg: 0x62, def: 0x3e00 },
    reg_default { reg: 0x64, def: 0x0000 },
    reg_default { reg: 0x76, def: 0x0006 },
    reg_default { reg: 0x78, def: 0x0001 },
    reg_default { reg: 0x7a, def: 0x0000 },
];

unsafe extern "C" fn wm9712_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    if reg == AC97_REC_GAIN {
        true
    } else {
        regmap_ac97_default_volatile(dev, reg)
    }
}

static WM9712_REGMAP_CONFIG: regmap_config = regmap_config {
    reg_bits: 16,
    reg_stride: 2,
    val_bits: 16,
    max_register: 0x7e,
    cache_type: unsafe { REGCACHE_MAPLE },
    volatile_reg: Some(wm9712_volatile_reg),
    reg_defaults: WM9712_REG_DEFAULTS.as_ptr(),
    num_reg_defaults: WM9712_REG_DEFAULTS.len(),
};

const HPL_MIXER: c_uint = 0x0;
const HPR_MIXER: c_uint = 0x1;

static WM9712_ALC_SELECT: [*const c_char; 4] = [c_str!("None"), c_str!("Left"), c_str!("Right"), c_str!("Stereo")];
static WM9712_ALC_MUX: [*const c_char; 4] = [c_str!("Stereo"), c_str!("Left"), c_str!("Right"), c_str!("None")];
static WM9712_OUT3_SRC: [*const c_char; 4] = [c_str!("Left"), c_str!("VREF"), c_str!("Left + Right"), c_str!("Mono")];
static WM9712_SPK_SRC: [*const c_char; 2] = [c_str!("Speaker Mix"), c_str!("Headphone Mix")];
static WM9712_REC_ADC: [*const c_char; 4] = [c_str!("Stereo"), c_str!("Left"), c_str!("Right"), c_str!("Mute")];
static WM9712_BASE: [*const c_char; 2] = [c_str!("Linear Control"), c_str!("Adaptive Boost")];
static WM9712_REC_GAIN: [*const c_char; 2] = [c_str!("+1.5dB Steps"), c_str!("+0.75dB Steps")];
static WM9712_MIC: [*const c_char; 4] = [c_str!("Mic 1"), c_str!("Differential"), c_str!("Mic 2"), c_str!("Stereo")];
static WM9712_REC_SEL: [*const c_char; 8] = [
    c_str!("Mic"),
    c_str!("NC"),
    c_str!("NC"),
    c_str!("Speaker Mixer"),
    c_str!("Line"),
    c_str!("Headphone Mixer"),
    c_str!("Phone Mixer"),
    c_str!("Phone"),
];
static WM9712_NG_TYPE: [*const c_char; 2] = [c_str!("Constant Gain"), c_str!("Mute")];
static WM9712_DIFF_SEL: [*const c_char; 2] = [c_str!("Mic"), c_str!("Line")];

static MAIN_TLV: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-3450, 150, 0);
static BOOST_TLV: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(0, 2000, 0);

static WM9712_ENUM: [soc_enum; 12] = [
    SOC_ENUM_SINGLE!(AC97_PCI_SVID, 14, 4, WM9712_ALC_SELECT),
    SOC_ENUM_SINGLE!(AC97_VIDEO, 12, 4, WM9712_ALC_MUX),
    SOC_ENUM_SINGLE!(AC97_AUX, 9, 4, WM9712_OUT3_SRC),
    SOC_ENUM_SINGLE!(AC97_AUX, 8, 2, WM9712_SPK_SRC),
    SOC_ENUM_SINGLE!(AC97_REC_SEL, 12, 4, WM9712_REC_ADC),
    SOC_ENUM_SINGLE!(AC97_MASTER_TONE, 15, 2, WM9712_BASE),
    SOC_ENUM_DOUBLE!(AC97_REC_GAIN, 14, 6, 2, WM9712_REC_GAIN),
    SOC_ENUM_SINGLE!(AC97_MIC, 5, 4, WM9712_MIC),
    SOC_ENUM_SINGLE!(AC97_REC_SEL, 8, 8, WM9712_REC_SEL),
    SOC_ENUM_SINGLE!(AC97_REC_SEL, 0, 8, WM9712_REC_SEL),
    SOC_ENUM_SINGLE!(AC97_PCI_SVID, 5, 2, WM9712_NG_TYPE),
    SOC_ENUM_SINGLE!(0x5c, 8, 2, WM9712_DIFF_SEL),
];

static WM9712_SND_AC97_CONTROLS: [snd_kcontrol_new; 52] = [
    SOC_DOUBLE!("Speaker Playback Volume", AC97_MASTER, 8, 0, 31, 1),
    SOC_SINGLE!("Speaker Playback Switch", AC97_MASTER, 15, 1, 1),
    SOC_DOUBLE!("Headphone Playback Volume", AC97_HEADPHONE, 8, 0, 31, 1),
    SOC_SINGLE!("Headphone Playback Switch", AC97_HEADPHONE, 15, 1, 1),
    SOC_DOUBLE!("PCM Playback Volume", AC97_PCM, 8, 0, 31, 1),
    SOC_SINGLE!("Speaker Playback ZC Switch", AC97_MASTER, 7, 1, 0),
    SOC_SINGLE!("Speaker Playback Invert Switch", AC97_MASTER, 6, 1, 0),
    SOC_SINGLE!("Headphone Playback ZC Switch", AC97_HEADPHONE, 7, 1, 0),
    SOC_SINGLE!("Mono Playback ZC Switch", AC97_MASTER_MONO, 7, 1, 0),
    SOC_SINGLE!("Mono Playback Volume", AC97_MASTER_MONO, 0, 31, 1),
    SOC_SINGLE!("Mono Playback Switch", AC97_MASTER_MONO, 15, 1, 1),
    SOC_SINGLE!("ALC Target Volume", AC97_CODEC_CLASS_REV, 12, 15, 0),
    SOC_SINGLE!("ALC Hold Time", AC97_CODEC_CLASS_REV, 8, 15, 0),
    SOC_SINGLE!("ALC Decay Time", AC97_CODEC_CLASS_REV, 4, 15, 0),
    SOC_SINGLE!("ALC Attack Time", AC97_CODEC_CLASS_REV, 0, 15, 0),
    SOC_ENUM!("ALC Function", WM9712_ENUM[0]),
    SOC_SINGLE!("ALC Max Volume", AC97_PCI_SVID, 11, 7, 0),
    SOC_SINGLE!("ALC ZC Timeout", AC97_PCI_SVID, 9, 3, 1),
    SOC_SINGLE!("ALC ZC Switch", AC97_PCI_SVID, 8, 1, 0),
    SOC_SINGLE!("ALC NG Switch", AC97_PCI_SVID, 7, 1, 0),
    SOC_ENUM!("ALC NG Type", WM9712_ENUM[10]),
    SOC_SINGLE!("ALC NG Threshold", AC97_PCI_SVID, 0, 31, 1),
    SOC_SINGLE!("Mic Headphone  Volume", AC97_VIDEO, 12, 7, 1),
    SOC_SINGLE!("ALC Headphone Volume", AC97_VIDEO, 7, 7, 1),
    SOC_SINGLE!("Out3 Switch", AC97_AUX, 15, 1, 1),
    SOC_SINGLE!("Out3 ZC Switch", AC97_AUX, 7, 1, 1),
    SOC_SINGLE!("Out3 Volume", AC97_AUX, 0, 31, 1),
    SOC_SINGLE!("PCBeep Bypass Headphone Volume", AC97_PC_BEEP, 12, 7, 1),
    SOC_SINGLE!("PCBeep Bypass Speaker Volume", AC97_PC_BEEP, 8, 7, 1),
    SOC_SINGLE!("PCBeep Bypass Phone Volume", AC97_PC_BEEP, 4, 7, 1),
    SOC_SINGLE!("Aux Playback Headphone Volume", AC97_CD, 12, 7, 1),
    SOC_SINGLE!("Aux Playback Speaker Volume", AC97_CD, 8, 7, 1),
    SOC_SINGLE!("Aux Playback Phone Volume", AC97_CD, 4, 7, 1),
    SOC_SINGLE!("Phone Volume", AC97_PHONE, 0, 15, 1),
    SOC_DOUBLE!("Line Capture Volume", AC97_LINE, 8, 0, 31, 1),
    SOC_SINGLE_TLV!("Capture Boost Switch", AC97_REC_SEL, 14, 1, 0, MAIN_TLV),
    SOC_SINGLE_TLV!("Capture to Phone Boost Switch", AC97_REC_SEL, 11, 1, 1, BOOST_TLV),
    SOC_SINGLE!("3D Upper Cut-off Switch", AC97_3D_CONTROL, 5, 1, 1),
    SOC_SINGLE!("3D Lower Cut-off Switch", AC97_3D_CONTROL, 4, 1, 1),
    SOC_SINGLE!("3D Playback Volume", AC97_3D_CONTROL, 0, 15, 0),
    SOC_ENUM!("Bass Control", WM9712_ENUM[5]),
    SOC_SINGLE!("Bass Cut-off Switch", AC97_MASTER_TONE, 12, 1, 1),
    SOC_SINGLE!("Tone Cut-off Switch", AC97_MASTER_TONE, 4, 1, 1),
    SOC_SINGLE!("Playback Attenuate (-6dB) Switch", AC97_MASTER_TONE, 6, 1, 0),
    SOC_SINGLE!("Bass Volume", AC97_MASTER_TONE, 8, 15, 1),
    SOC_SINGLE!("Treble Volume", AC97_MASTER_TONE, 0, 15, 1),
    SOC_SINGLE!("Capture Switch", AC97_REC_GAIN, 15, 1, 1),
    SOC_ENUM!("Capture Volume Steps", WM9712_ENUM[6]),
    SOC_DOUBLE!("Capture Volume", AC97_REC_GAIN, 8, 0, 63, 0),
    SOC_SINGLE!("Capture ZC Switch", AC97_REC_GAIN, 7, 1, 0),
    SOC_SINGLE_TLV!("Mic 1 Volume", AC97_MIC, 8, 31, 1, MAIN_TLV),
    SOC_SINGLE_TLV!("Mic 2 Volume", AC97_MIC, 0, 31, 1, MAIN_TLV),
    SOC_SINGLE_TLV!("Mic Boost Volume", AC97_MIC, 7, 1, 0, BOOST_TLV),
];

static WM9712_MIXER_MUTE_REGS: [c_uint; 6] = unsafe {
    [AC97_VIDEO, AC97_PCM, AC97_LINE, AC97_PHONE, AC97_CD, AC97_PC_BEEP]
};

/* We have to create a fake left and right HP mixers because
 * the codec only has a single control that is shared by both channels.
 * This makes it impossible to determine the audio path.
 */
unsafe extern "C" fn wm9712_hp_mixer_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let component = snd_soc_dapm_to_component(dapm);
    let wm9712 = snd_soc_component_get_drvdata(component) as *mut wm9712_priv;
    let val = (*ucontrol).value.integer.value[0] as c_uint;
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let mixer: c_uint;
    let mask: c_uint;
    let shift: c_uint;
    let old: c_uint;
    let mut update: snd_soc_dapm_update = snd_soc_dapm_update {
        kcontrol: ptr::null_mut(),
        reg: 0,
        mask: 0,
        val: 0,
    };
    let change: bool;

    mixer = (*mc).shift >> 8;
    shift = (*mc).shift & 0xff;
    mask = 1u32 << shift;

    /* guard(mutex)(&wm9712->lock); */
    old = (*wm9712).hp_mixer[mixer as usize];
    if (*ucontrol).value.integer.value[0] != 0 {
        (*wm9712).hp_mixer[mixer as usize] |= mask;
    } else {
        (*wm9712).hp_mixer[mixer as usize] &= !mask;
    }

    change = old != (*wm9712).hp_mixer[mixer as usize];

    if change {
        update.kcontrol = kcontrol;
        update.reg = WM9712_MIXER_MUTE_REGS[shift as usize];
        update.mask = 0x8000;
        if ((*wm9712).hp_mixer[0] & mask) != 0 || ((*wm9712).hp_mixer[1] & mask) != 0 {
            update.val = 0x0;
        } else {
            update.val = 0x8000;
        }

        snd_soc_dapm_mixer_update_power(dapm, kcontrol, val, &mut update);
    }

    change as c_int
}

unsafe extern "C" fn wm9712_hp_mixer_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let component = snd_soc_dapm_to_component(dapm);
    let wm9712 = snd_soc_component_get_drvdata(component) as *mut wm9712_priv;
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let shift: c_uint;
    let mixer: c_uint;

    mixer = (*mc).shift >> 8;
    shift = (*mc).shift & 0xff;

    (*ucontrol).value.integer.value[0] = (((*wm9712).hp_mixer[mixer as usize] >> shift) & 1) as i64;

    0
}

macro_rules! WM9712_HP_MIXER_CTRL {
    ($xname:expr, $xmixer:expr, $xshift:expr) => {
        SOC_SINGLE_EXT!(
            $xname,
            SND_SOC_NOPM,
            (($xmixer) << 8) | ($xshift),
            1,
            0,
            wm9712_hp_mixer_get,
            wm9712_hp_mixer_put
        )
    };
}

/* Left Headphone Mixers */
static WM9712_HPL_MIXER_CONTROLS: [snd_kcontrol_new; 6] = [
    WM9712_HP_MIXER_CTRL!("PCBeep Bypass Switch", HPL_MIXER, 5),
    WM9712_HP_MIXER_CTRL!("Aux Playback Switch", HPL_MIXER, 4),
    WM9712_HP_MIXER_CTRL!("Phone Bypass Switch", HPL_MIXER, 3),
    WM9712_HP_MIXER_CTRL!("Line Bypass Switch", HPL_MIXER, 2),
    WM9712_HP_MIXER_CTRL!("PCM Playback Switch", HPL_MIXER, 1),
    WM9712_HP_MIXER_CTRL!("Mic Sidetone Switch", HPL_MIXER, 0),
];

/* Right Headphone Mixers */
static WM9712_HPR_MIXER_CONTROLS: [snd_kcontrol_new; 6] = [
    WM9712_HP_MIXER_CTRL!("PCBeep Bypass Switch", HPR_MIXER, 5),
    WM9712_HP_MIXER_CTRL!("Aux Playback Switch", HPR_MIXER, 4),
    WM9712_HP_MIXER_CTRL!("Phone Bypass Switch", HPR_MIXER, 3),
    WM9712_HP_MIXER_CTRL!("Line Bypass Switch", HPR_MIXER, 2),
    WM9712_HP_MIXER_CTRL!("PCM Playback Switch", HPR_MIXER, 1),
    WM9712_HP_MIXER_CTRL!("Mic Sidetone Switch", HPR_MIXER, 0),
];

/* Speaker Mixer */
static WM9712_SPEAKER_MIXER_CONTROLS: [snd_kcontrol_new; 5] = [
    SOC_DAPM_SINGLE!("PCBeep Bypass Switch", AC97_PC_BEEP, 11, 1, 1),
    SOC_DAPM_SINGLE!("Aux Playback Switch", AC97_CD, 11, 1, 1),
    SOC_DAPM_SINGLE!("Phone Bypass Switch", AC97_PHONE, 14, 1, 1),
    SOC_DAPM_SINGLE!("Line Bypass Switch", AC97_LINE, 14, 1, 1),
    SOC_DAPM_SINGLE!("PCM Playback Switch", AC97_PCM, 14, 1, 1),
];

/* Phone Mixer */
static WM9712_PHONE_MIXER_CONTROLS: [snd_kcontrol_new; 6] = [
    SOC_DAPM_SINGLE!("PCBeep Bypass Switch", AC97_PC_BEEP, 7, 1, 1),
    SOC_DAPM_SINGLE!("Aux Playback Switch", AC97_CD, 7, 1, 1),
    SOC_DAPM_SINGLE!("Line Bypass Switch", AC97_LINE, 13, 1, 1),
    SOC_DAPM_SINGLE!("PCM Playback Switch", AC97_PCM, 13, 1, 1),
    SOC_DAPM_SINGLE!("Mic 1 Sidetone Switch", AC97_MIC, 14, 1, 1),
    SOC_DAPM_SINGLE!("Mic 2 Sidetone Switch", AC97_MIC, 13, 1, 1),
];

static WM9712_ALC_MUX_CONTROLS: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", WM9712_ENUM[1]);
static WM9712_OUT3_MUX_CONTROLS: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", WM9712_ENUM[2]);
static WM9712_SPK_MUX_CONTROLS: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", WM9712_ENUM[3]);
static WM9712_CAPTURE_PHONE_MUX_CONTROLS: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", WM9712_ENUM[4]);
static WM9712_CAPTURE_SELECTL_CONTROLS: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", WM9712_ENUM[8]);
static WM9712_CAPTURE_SELECTR_CONTROLS: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", WM9712_ENUM[9]);
static WM9712_MIC_SRC_CONTROLS: snd_kcontrol_new = SOC_DAPM_ENUM!("Mic Source Select", WM9712_ENUM[7]);
static WM9712_DIFF_SEL_CONTROLS: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", WM9712_ENUM[11]);

static WM9712_DAPM_WIDGETS: [snd_soc_dapm_widget; 40] = [
    SND_SOC_DAPM_MUX!("ALC Sidetone Mux", SND_SOC_NOPM, 0, 0, &WM9712_ALC_MUX_CONTROLS),
    SND_SOC_DAPM_MUX!("Out3 Mux", SND_SOC_NOPM, 0, 0, &WM9712_OUT3_MUX_CONTROLS),
    SND_SOC_DAPM_MUX!("Speaker Mux", SND_SOC_NOPM, 0, 0, &WM9712_SPK_MUX_CONTROLS),
    SND_SOC_DAPM_MUX!("Capture Phone Mux", SND_SOC_NOPM, 0, 0, &WM9712_CAPTURE_PHONE_MUX_CONTROLS),
    SND_SOC_DAPM_MUX!("Left Capture Select", SND_SOC_NOPM, 0, 0, &WM9712_CAPTURE_SELECTL_CONTROLS),
    SND_SOC_DAPM_MUX!("Right Capture Select", SND_SOC_NOPM, 0, 0, &WM9712_CAPTURE_SELECTR_CONTROLS),
    SND_SOC_DAPM_MUX!("Left Mic Select Source", SND_SOC_NOPM, 0, 0, &WM9712_MIC_SRC_CONTROLS),
    SND_SOC_DAPM_MUX!("Right Mic Select Source", SND_SOC_NOPM, 0, 0, &WM9712_MIC_SRC_CONTROLS),
    SND_SOC_DAPM_MUX!("Differential Source", SND_SOC_NOPM, 0, 0, &WM9712_DIFF_SEL_CONTROLS),
    SND_SOC_DAPM_MIXER!("AC97 Mixer", SND_SOC_NOPM, 0, 0, ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("Left HP Mixer", AC97_INT_PAGING, 9, 1, &WM9712_HPL_MIXER_CONTROLS[0], WM9712_HPL_MIXER_CONTROLS.len()),
    SND_SOC_DAPM_MIXER!("Right HP Mixer", AC97_INT_PAGING, 8, 1, &WM9712_HPR_MIXER_CONTROLS[0], WM9712_HPR_MIXER_CONTROLS.len()),
    SND_SOC_DAPM_MIXER!("Phone Mixer", AC97_INT_PAGING, 6, 1, &WM9712_PHONE_MIXER_CONTROLS[0], WM9712_PHONE_MIXER_CONTROLS.len()),
    SND_SOC_DAPM_MIXER!("Speaker Mixer", AC97_INT_PAGING, 7, 1, &WM9712_SPEAKER_MIXER_CONTROLS[0], WM9712_SPEAKER_MIXER_CONTROLS.len()),
    SND_SOC_DAPM_MIXER!("Mono Mixer", SND_SOC_NOPM, 0, 0, ptr::null(), 0),
    SND_SOC_DAPM_DAC!("Left DAC", "Left HiFi Playback", AC97_INT_PAGING, 14, 1),
    SND_SOC_DAPM_DAC!("Right DAC", "Right HiFi Playback", AC97_INT_PAGING, 13, 1),
    SND_SOC_DAPM_DAC!("Aux DAC", "Aux Playback", SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_ADC!("Left ADC", "Left HiFi Capture", AC97_INT_PAGING, 12, 1),
    SND_SOC_DAPM_ADC!("Right ADC", "Right HiFi Capture", AC97_INT_PAGING, 11, 1),
    SND_SOC_DAPM_PGA!("Headphone PGA", AC97_INT_PAGING, 4, 1, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Speaker PGA", AC97_INT_PAGING, 3, 1, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Out 3 PGA", AC97_INT_PAGING, 5, 1, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Line PGA", AC97_INT_PAGING, 2, 1, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Phone PGA", AC97_INT_PAGING, 1, 1, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Mic PGA", AC97_INT_PAGING, 0, 1, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Differential Mic", SND_SOC_NOPM, 0, 0, ptr::null(), 0),
    SND_SOC_DAPM_MICBIAS!("Mic Bias", AC97_INT_PAGING, 10, 1),
    SND_SOC_DAPM_OUTPUT!("MONOOUT"),
    SND_SOC_DAPM_OUTPUT!("HPOUTL"),
    SND_SOC_DAPM_OUTPUT!("HPOUTR"),
    SND_SOC_DAPM_OUTPUT!("LOUT2"),
    SND_SOC_DAPM_OUTPUT!("ROUT2"),
    SND_SOC_DAPM_OUTPUT!("OUT3"),
    SND_SOC_DAPM_INPUT!("LINEINL"),
    SND_SOC_DAPM_INPUT!("LINEINR"),
    SND_SOC_DAPM_INPUT!("PHONE"),
    SND_SOC_DAPM_INPUT!("PCBEEP"),
    SND_SOC_DAPM_INPUT!("MIC1"),
    SND_SOC_DAPM_INPUT!("MIC2"),
];

macro_rules! route {
    ($sink:expr, NULL, $source:expr) => {
        snd_soc_dapm_route { sink: c_str!($sink), control: ptr::null(), source: c_str!($source) }
    };
    ($sink:expr, $control:expr, $source:expr) => {
        snd_soc_dapm_route { sink: c_str!($sink), control: c_str!($control), source: c_str!($source) }
    };
}

static WM9712_AUDIO_MAP: [snd_soc_dapm_route; 79] = [
    route!("AC97 Mixer", NULL, "Left DAC"),
    route!("AC97 Mixer", NULL, "Right DAC"),
    route!("Left HP Mixer", "PCBeep Bypass Switch", "PCBEEP"),
    route!("Left HP Mixer", "Aux Playback Switch", "Aux DAC"),
    route!("Left HP Mixer", "Phone Bypass Switch", "Phone PGA"),
    route!("Left HP Mixer", "Line Bypass Switch", "Line PGA"),
    route!("Left HP Mixer", "PCM Playback Switch", "Left DAC"),
    route!("Left HP Mixer", "Mic Sidetone Switch", "Mic PGA"),
    route!("Left HP Mixer", NULL, "ALC Sidetone Mux"),
    route!("Right HP Mixer", "PCBeep Bypass Switch", "PCBEEP"),
    route!("Right HP Mixer", "Aux Playback Switch", "Aux DAC"),
    route!("Right HP Mixer", "Phone Bypass Switch", "Phone PGA"),
    route!("Right HP Mixer", "Line Bypass Switch", "Line PGA"),
    route!("Right HP Mixer", "PCM Playback Switch", "Right DAC"),
    route!("Right HP Mixer", "Mic Sidetone Switch", "Mic PGA"),
    route!("Right HP Mixer", NULL, "ALC Sidetone Mux"),
    route!("Speaker Mixer", "PCBeep Bypass Switch", "PCBEEP"),
    route!("Speaker Mixer", "Line Bypass Switch", "Line PGA"),
    route!("Speaker Mixer", "PCM Playback Switch", "AC97 Mixer"),
    route!("Speaker Mixer", "Phone Bypass Switch", "Phone PGA"),
    route!("Speaker Mixer", "Aux Playback Switch", "Aux DAC"),
    route!("Phone Mixer", "PCBeep Bypass Switch", "PCBEEP"),
    route!("Phone Mixer", "Line Bypass Switch", "Line PGA"),
    route!("Phone Mixer", "Aux Playback Switch", "Aux DAC"),
    route!("Phone Mixer", "PCM Playback Switch", "AC97 Mixer"),
    route!("Phone Mixer", "Mic 1 Sidetone Switch", "Mic PGA"),
    route!("Phone Mixer", "Mic 2 Sidetone Switch", "Mic PGA"),
    route!("Line PGA", NULL, "LINEINL"),
    route!("Line PGA", NULL, "LINEINR"),
    route!("Phone PGA", NULL, "PHONE"),
    route!("Mic PGA", NULL, "MIC1"),
    route!("Mic PGA", NULL, "MIC2"),
    route!("Differential Mic", NULL, "MIC1"),
    route!("Differential Mic", NULL, "MIC2"),
    route!("Left Mic Select Source", "Mic 1", "MIC1"),
    route!("Left Mic Select Source", "Mic 2", "MIC2"),
    route!("Left Mic Select Source", "Stereo", "MIC1"),
    route!("Left Mic Select Source", "Differential", "Differential Mic"),
    route!("Right Mic Select Source", "Mic 1", "MIC1"),
    route!("Right Mic Select Source", "Mic 2", "MIC2"),
    route!("Right Mic Select Source", "Stereo", "MIC2"),
    route!("Right Mic Select Source", "Differential", "Differential Mic"),
    route!("Left Capture Select", "Mic", "MIC1"),
    route!("Left Capture Select", "Speaker Mixer", "Speaker Mixer"),
    route!("Left Capture Select", "Line", "LINEINL"),
    route!("Left Capture Select", "Headphone Mixer", "Left HP Mixer"),
    route!("Left Capture Select", "Phone Mixer", "Phone Mixer"),
    route!("Left Capture Select", "Phone", "PHONE"),
    route!("Right Capture Select", "Mic", "MIC2"),
    route!("Right Capture Select", "Speaker Mixer", "Speaker Mixer"),
    route!("Right Capture Select", "Line", "LINEINR"),
    route!("Right Capture Select", "Headphone Mixer", "Right HP Mixer"),
    route!("Right Capture Select", "Phone Mixer", "Phone Mixer"),
    route!("Right Capture Select", "Phone", "PHONE"),
    route!("ALC Sidetone Mux", "Stereo", "Left Capture Select"),
    route!("ALC Sidetone Mux", "Stereo", "Right Capture Select"),
    route!("ALC Sidetone Mux", "Left", "Left Capture Select"),
    route!("ALC Sidetone Mux", "Right", "Right Capture Select"),
    route!("Left ADC", NULL, "Left Capture Select"),
    route!("Right ADC", NULL, "Right Capture Select"),
    route!("MONOOUT", NULL, "Phone Mixer"),
    route!("HPOUTL", NULL, "Headphone PGA"),
    route!("Headphone PGA", NULL, "Left HP Mixer"),
    route!("HPOUTR", NULL, "Headphone PGA"),
    route!("Headphone PGA", NULL, "Right HP Mixer"),
    route!("Mono Mixer", NULL, "Left HP Mixer"),
    route!("Mono Mixer", NULL, "Right HP Mixer"),
    route!("Out3 Mux", "Left", "Left HP Mixer"),
    route!("Out3 Mux", "Mono", "Phone Mixer"),
    route!("Out3 Mux", "Left + Right", "Mono Mixer"),
    route!("Out 3 PGA", NULL, "Out3 Mux"),
    route!("OUT3", NULL, "Out 3 PGA"),
    route!("Speaker Mux", "Speaker Mix", "Speaker Mixer"),
    route!("Speaker Mux", "Headphone Mix", "Mono Mixer"),
    route!("Speaker PGA", NULL, "Speaker Mux"),
    route!("LOUT2", NULL, "Speaker PGA"),
    route!("ROUT2", NULL, "Speaker PGA"),
];

unsafe extern "C" fn ac97_prepare(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let reg: c_uint;
    let runtime = (*substream).runtime;

    snd_soc_component_update_bits(component, AC97_EXTENDED_STATUS, 0x1, 0x1);

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        reg = AC97_PCM_FRONT_DAC_RATE;
    } else {
        reg = AC97_PCM_LR_ADC_RATE;
    }

    snd_soc_component_write(component, reg, (*runtime).rate)
}

unsafe extern "C" fn ac97_aux_prepare(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let runtime = (*substream).runtime;

    snd_soc_component_update_bits(component, AC97_EXTENDED_STATUS, 0x1, 0x1);
    snd_soc_component_update_bits(component, AC97_PCI_SID, 0x8000, 0x8000);

    if (*substream).stream != SNDRV_PCM_STREAM_PLAYBACK {
        return -ENODEV;
    }

    snd_soc_component_write(component, AC97_PCM_SURR_DAC_RATE, (*runtime).rate)
}

static WM9712_DAI_OPS_HIFI: snd_soc_dai_ops = snd_soc_dai_ops {
    prepare: Some(ac97_prepare),
};

static WM9712_DAI_OPS_AUX: snd_soc_dai_ops = snd_soc_dai_ops {
    prepare: Some(ac97_aux_prepare),
};

static mut WM9712_DAI: [snd_soc_dai_driver; 2] = unsafe {
    let wm9712_ac97_rates = SNDRV_PCM_RATE_8000
        | SNDRV_PCM_RATE_11025
        | SNDRV_PCM_RATE_22050
        | SNDRV_PCM_RATE_44100
        | SNDRV_PCM_RATE_48000;
    [
        snd_soc_dai_driver {
            name: c_str!("wm9712-hifi"),
            playback: snd_soc_pcm_stream {
                stream_name: c_str!("HiFi Playback"),
                channels_min: 1,
                channels_max: 2,
                rates: wm9712_ac97_rates,
                formats: SND_SOC_STD_AC97_FMTS,
            },
            capture: snd_soc_pcm_stream {
                stream_name: c_str!("HiFi Capture"),
                channels_min: 1,
                channels_max: 2,
                rates: wm9712_ac97_rates,
                formats: SND_SOC_STD_AC97_FMTS,
            },
            ops: &WM9712_DAI_OPS_HIFI,
        },
        snd_soc_dai_driver {
            name: c_str!("wm9712-aux"),
            playback: snd_soc_pcm_stream {
                stream_name: c_str!("Aux Playback"),
                channels_min: 1,
                channels_max: 1,
                rates: wm9712_ac97_rates,
                formats: SND_SOC_STD_AC97_FMTS,
            },
            capture: snd_soc_pcm_stream {
                stream_name: ptr::null(),
                channels_min: 0,
                channels_max: 0,
                rates: 0,
                formats: 0,
            },
            ops: &WM9712_DAI_OPS_AUX,
        },
    ]
};

unsafe extern "C" fn wm9712_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON | snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {}
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            snd_soc_component_write(component, AC97_POWERDOWN, 0x0000);
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            /* disable everything including AC link */
            snd_soc_component_write(component, AC97_EXTENDED_MSTATUS, 0xffff);
            snd_soc_component_write(component, AC97_POWERDOWN, 0xffff);
        }
    }
    0
}

unsafe extern "C" fn wm9712_soc_resume(component: *mut snd_soc_component) -> c_int {
    let wm9712 = snd_soc_component_get_drvdata(component) as *mut wm9712_priv;
    let dapm = snd_soc_component_to_dapm(component);
    let mut ret: c_int;

    ret = snd_ac97_reset((*wm9712).ac97, true, WM9712_VENDOR_ID, WM9712_VENDOR_ID_MASK);
    if ret < 0 {
        return ret;
    }

    snd_soc_dapm_force_bias_level(dapm, snd_soc_bias_level::SND_SOC_BIAS_STANDBY);

    if ret == 0 {
        snd_soc_component_cache_sync(component);
    }

    ret
}

unsafe extern "C" fn wm9712_soc_probe(component: *mut snd_soc_component) -> c_int {
    let wm9712 = snd_soc_component_get_drvdata(component) as *mut wm9712_priv;
    let regmap: *mut regmap;

    if !(*wm9712).mfd_pdata.is_null() {
        (*wm9712).ac97 = (*(*wm9712).mfd_pdata).ac97;
        regmap = (*(*wm9712).mfd_pdata).regmap;
    } else if IS_ENABLED(CONFIG_SND_SOC_AC97_BUS) {
        let ret: c_int;

        (*wm9712).ac97 = snd_soc_new_ac97_component(component, WM9712_VENDOR_ID, WM9712_VENDOR_ID_MASK);
        if IS_ERR((*wm9712).ac97 as *const c_void) {
            ret = PTR_ERR((*wm9712).ac97 as *const c_void);
            dev_err((*component).dev, c_str!("Failed to register AC97 codec: %d\n"), ret);
            return ret;
        }

        regmap = regmap_init_ac97((*wm9712).ac97, &WM9712_REGMAP_CONFIG);
        if IS_ERR(regmap as *const c_void) {
            snd_soc_free_ac97_component((*wm9712).ac97);
            return PTR_ERR(regmap as *const c_void);
        }
    } else {
        return -ENXIO;
    }

    snd_soc_component_init_regmap(component, regmap);

    /* set alc mux to none */
    snd_soc_component_update_bits(component, AC97_VIDEO, 0x3000, 0x3000);

    0
}

unsafe extern "C" fn wm9712_soc_remove(component: *mut snd_soc_component) {
    let wm9712 = snd_soc_component_get_drvdata(component) as *mut wm9712_priv;

    if IS_ENABLED(CONFIG_SND_SOC_AC97_BUS) && (*wm9712).mfd_pdata.is_null() {
        snd_soc_component_exit_regmap(component);
        snd_soc_free_ac97_component((*wm9712).ac97);
    }
}

static SOC_COMPONENT_DEV_WM9712: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wm9712_soc_probe),
    remove: Some(wm9712_soc_remove),
    resume: Some(wm9712_soc_resume),
    set_bias_level: Some(wm9712_set_bias_level),
    controls: WM9712_SND_AC97_CONTROLS.as_ptr(),
    num_controls: WM9712_SND_AC97_CONTROLS.len(),
    dapm_widgets: WM9712_DAPM_WIDGETS.as_ptr(),
    num_dapm_widgets: WM9712_DAPM_WIDGETS.len(),
    dapm_routes: WM9712_AUDIO_MAP.as_ptr(),
    num_dapm_routes: WM9712_AUDIO_MAP.len(),
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn wm9712_probe(pdev: *mut platform_device) -> c_int {
    let wm9712: *mut wm9712_priv;

    wm9712 = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<wm9712_priv>(),
        GFP_KERNEL,
    ) as *mut wm9712_priv;
    if wm9712.is_null() {
        return -ENOMEM;
    }

    mutex_init(&mut (*wm9712).lock);

    (*wm9712).mfd_pdata = dev_get_platdata(&mut (*pdev).dev) as *mut wm97xx_platform_data;
    platform_set_drvdata(pdev, wm9712 as *mut c_void);

    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &SOC_COMPONENT_DEV_WM9712,
        WM9712_DAI.as_mut_ptr(),
        WM9712_DAI.len(),
    )
}

static mut WM9712_COMPONENT_DRIVER: platform_driver = platform_driver {
    driver: device_driver {
        name: c_str!("wm9712-codec"),
    },
    probe: Some(wm9712_probe),
};

module_platform_driver!(WM9712_COMPONENT_DRIVER);

MODULE_DESCRIPTION!("ASoC WM9711/WM9712 driver");
MODULE_AUTHOR!("Liam Girdwood");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
