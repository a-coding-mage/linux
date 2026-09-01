// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8731.c  --  WM8731 ALSA SoC Audio driver
 *
 * Copyright 2005 Openedhand Ltd.
 * Copyright 2006-12 Wolfson Microelectronics, plc
 *
 * Author: Richard Purdie <richard@openedhand.com>
 *
 * Based on wm8753.c by Liam Girdwood
 */

/* C dependencies removed from executable Rust:
 * linux/cleanup.h, linux/module.h, linux/moduleparam.h, linux/init.h,
 * linux/delay.h, linux/pm.h, linux/slab.h, linux/regmap.h,
 * linux/regulator/consumer.h, linux/clk.h, sound/core.h, sound/pcm.h,
 * sound/pcm_params.h, sound/soc.h, sound/initval.h, sound/tlv.h, wm8731.h.
 */

type c_char = i8;
type c_int = i32;
type c_uint = u32;
type bool_ = bool;
type u8 = core::ffi::c_uchar;
type u16 = core::ffi::c_ushort;
type u32 = core::ffi::c_uint;

const EINVAL: c_int = 22;
const ENOENT: c_int = 2;

extern "C" {
    static WM8731_NUM_SUPPLIES: usize;
    static WM8731_RESET: c_uint;
    static WM8731_APANA: c_uint;
    static WM8731_APDIGI: c_uint;
    static WM8731_ACTIVE: c_uint;
    static WM8731_PWR: c_uint;
    static WM8731_LOUT1V: c_uint;
    static WM8731_ROUT1V: c_uint;
    static WM8731_LINVOL: c_uint;
    static WM8731_RINVOL: c_uint;
    static WM8731_IFACE: c_uint;
    static WM8731_SRATE: c_uint;
    static WM8731_SYSCLK_XTAL: c_int;
    static WM8731_SYSCLK_MCLK: c_int;
    static SND_SOC_NOPM: c_int;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
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
    static SNDRV_PCM_HW_PARAM_RATE: c_uint;
    static SNDRV_PCM_RATE_8000_96000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S20_3LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_uint;
    static REGCACHE_MAPLE: c_int;
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
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
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
pub struct snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}
#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
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
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
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
pub struct snd_soc_bias_level {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
}
#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *const c_uint,
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
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
    pub connected: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_soc_dapm_widget) -> c_int>,
}
#[repr(C)]
pub struct snd_soc_dapm_widget_layout {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub no_capture_mute: c_uint,
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
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_layout,
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
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub cache_type: c_int,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
}
#[repr(C)]
pub struct wm8731_priv {
    pub mclk: *mut clk,
    pub lock: mutex,
    pub supplies: [regulator_bulk_data; 4],
    pub regmap: *mut regmap,
    pub deemph: c_uint,
    pub playback_fs: c_int,
    pub sysclk_type: c_int,
    pub sysclk: c_uint,
    pub constraints: *const snd_pcm_hw_constraint_list,
}

extern "C" {
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regcache_mark_dirty(map: *mut regmap);
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut wm8731_priv;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_sync(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_uint, l: *const snd_pcm_hw_constraint_list) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn regulator_bulk_enable(num_consumers: c_uint, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num_consumers: c_uint, consumers: *mut regulator_bulk_data) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_regulator_bulk_get(dev: *mut device, num_consumers: c_uint, consumers: *mut regulator_bulk_data) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool_;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> c_int;
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

const fn array_size<T, const N: usize>(_: &[T; N]) -> c_uint {
    N as c_uint
}

unsafe fn abs_int(x: c_int) -> c_int {
    if x < 0 { -x } else { x }
}

static WM8731_SUPPLY_NAMES: [*const c_char; 4] = [
    b"AVDD\0".as_ptr() as *const c_char,
    b"HPVDD\0".as_ptr() as *const c_char,
    b"DCVDD\0".as_ptr() as *const c_char,
    b"DBVDD\0".as_ptr() as *const c_char,
];

/*
 * wm8731 register cache
 */
static WM8731_REG_DEFAULTS: [reg_default; 10] = [
    reg_default { reg: 0, def: 0x0097 },
    reg_default { reg: 1, def: 0x0097 },
    reg_default { reg: 2, def: 0x0079 },
    reg_default { reg: 3, def: 0x0079 },
    reg_default { reg: 4, def: 0x000a },
    reg_default { reg: 5, def: 0x0008 },
    reg_default { reg: 6, def: 0x009f },
    reg_default { reg: 7, def: 0x000a },
    reg_default { reg: 8, def: 0x0000 },
    reg_default { reg: 9, def: 0x0000 },
];

unsafe extern "C" fn wm8731_volatile(_dev: *mut device, reg: c_uint) -> bool_ {
    reg == WM8731_RESET
}

unsafe fn wm8731_reset(m: *mut regmap) -> c_int {
    regmap_write(m, WM8731_RESET, 0)
}

static WM8731_INPUT_SELECT: [*const c_char; 2] = [
    b"Line In\0".as_ptr() as *const c_char,
    b"Mic\0".as_ptr() as *const c_char,
];

/* static SOC_ENUM_SINGLE_DECL(wm8731_insel_enum, WM8731_APANA, 2, wm8731_input_select); */

static WM8731_DEEMPH: [c_int; 4] = [0, 32000, 44100, 48000];

unsafe fn wm8731_set_deemph(component: *mut snd_soc_component) -> c_int {
    let wm8731 = snd_soc_component_get_drvdata(component);
    let mut val: c_int;
    let mut best: c_int;

    /* If we're using deemphasis select the nearest available sample
     * rate.
     */
    if (*wm8731).deemph != 0 {
        best = 1;
        let mut i = 2usize;
        while i < WM8731_DEEMPH.len() {
            if abs_int(WM8731_DEEMPH[i] - (*wm8731).playback_fs)
                < abs_int(WM8731_DEEMPH[best as usize] - (*wm8731).playback_fs)
            {
                best = i as c_int;
            }
            i += 1;
        }

        val = best << 1;
    } else {
        best = 0;
        val = 0;
    }

    dev_dbg(
        (*component).dev,
        b"Set deemphasis %d (%dHz)\n\0".as_ptr() as *const c_char,
        best,
        WM8731_DEEMPH[best as usize],
    );

    snd_soc_component_update_bits(component, WM8731_APDIGI, 0x6, val as c_uint)
}

unsafe extern "C" fn wm8731_get_deemph(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let wm8731 = snd_soc_component_get_drvdata(component);

    (*ucontrol).value.integer.value[0] = (*wm8731).deemph as i64;

    0
}

unsafe extern "C" fn wm8731_put_deemph(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let wm8731 = snd_soc_component_get_drvdata(component);
    let deemph = (*ucontrol).value.integer.value[0] as c_uint;

    if deemph > 1 {
        return -EINVAL;
    }

    mutex_lock(core::ptr::addr_of_mut!((*wm8731).lock));
    if (*wm8731).deemph != deemph {
        (*wm8731).deemph = deemph;

        wm8731_set_deemph(component);

        mutex_unlock(core::ptr::addr_of_mut!((*wm8731).lock));
        return 1;
    }
    mutex_unlock(core::ptr::addr_of_mut!((*wm8731).lock));

    0
}

/* static const DECLARE_TLV_DB_SCALE(in_tlv, -3450, 150, 0); */
/* static const DECLARE_TLV_DB_SCALE(sidetone_tlv, -1500, 300, 0); */
/* static const DECLARE_TLV_DB_SCALE(out_tlv, -12100, 100, 1); */
/* static const DECLARE_TLV_DB_SCALE(mic_tlv, 0, 2000, 0); */

/* snd_kcontrol_new initializers are macro-defined in ASoC headers. */
static WM8731_SND_CONTROLS: [snd_kcontrol_new; 11] = unsafe { core::mem::zeroed() };

/* Output Mixer */
static WM8731_OUTPUT_MIXER_CONTROLS: [snd_kcontrol_new; 3] = unsafe { core::mem::zeroed() };

/* Input mux */
static WM8731_INPUT_MUX_CONTROLS: snd_kcontrol_new = unsafe { core::mem::zeroed() };

static WM8731_DAPM_WIDGETS: [snd_soc_dapm_widget_layout; 15] = unsafe { core::mem::zeroed() };

unsafe extern "C" fn wm8731_check_osc(
    source: *mut snd_soc_dapm_widget,
    _sink: *mut snd_soc_dapm_widget,
) -> c_int {
    let component = snd_soc_dapm_to_component((*source).dapm);
    let wm8731 = snd_soc_component_get_drvdata(component);

    ((*wm8731).sysclk_type == WM8731_SYSCLK_XTAL) as c_int
}

static WM8731_INTERCON: [snd_soc_dapm_route; 21] = [
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"OSC\0".as_ptr() as *const c_char, connected: Some(wm8731_check_osc) },
    snd_soc_dapm_route { sink: b"ADC\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"OSC\0".as_ptr() as *const c_char, connected: Some(wm8731_check_osc) },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"ACTIVE\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"ADC\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"ACTIVE\0".as_ptr() as *const c_char, connected: None },

    /* output mixer */
    snd_soc_dapm_route { sink: b"Output Mixer\0".as_ptr() as *const c_char, control: b"Line Bypass Switch\0".as_ptr() as *const c_char, source: b"Line Input\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"Output Mixer\0".as_ptr() as *const c_char, control: b"HiFi Playback Switch\0".as_ptr() as *const c_char, source: b"DAC\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"Output Mixer\0".as_ptr() as *const c_char, control: b"Mic Sidetone Switch\0".as_ptr() as *const c_char, source: b"Mic Bias\0".as_ptr() as *const c_char, connected: None },

    /* outputs */
    snd_soc_dapm_route { sink: b"RHPOUT\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Output Mixer\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"ROUT\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Output Mixer\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"LHPOUT\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Output Mixer\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"LOUT\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Output Mixer\0".as_ptr() as *const c_char, connected: None },

    /* input mux */
    snd_soc_dapm_route { sink: b"Input Mux\0".as_ptr() as *const c_char, control: b"Line In\0".as_ptr() as *const c_char, source: b"Line Input\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"Input Mux\0".as_ptr() as *const c_char, control: b"Mic\0".as_ptr() as *const c_char, source: b"Mic Bias\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"ADC\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Input Mux\0".as_ptr() as *const c_char, connected: None },

    /* inputs */
    snd_soc_dapm_route { sink: b"Line Input\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"LLINEIN\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"Line Input\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"RLINEIN\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"Mic Bias\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"MICIN\0".as_ptr() as *const c_char, connected: None },
];

#[repr(C)]
struct _coeff_div {
    mclk: u32,
    rate: u32,
    fs: u16,
    sr: u8,
    bosr: u8,
    usb: u8,
}

/* codec mclk clock divider coefficients */
static COEFF_DIV: [_coeff_div; 21] = [
    /* 48k */
    _coeff_div { mclk: 12288000, rate: 48000, fs: 256, sr: 0x0, bosr: 0x0, usb: 0x0 },
    _coeff_div { mclk: 18432000, rate: 48000, fs: 384, sr: 0x0, bosr: 0x1, usb: 0x0 },
    _coeff_div { mclk: 12000000, rate: 48000, fs: 250, sr: 0x0, bosr: 0x0, usb: 0x1 },

    /* 32k */
    _coeff_div { mclk: 12288000, rate: 32000, fs: 384, sr: 0x6, bosr: 0x0, usb: 0x0 },
    _coeff_div { mclk: 18432000, rate: 32000, fs: 576, sr: 0x6, bosr: 0x1, usb: 0x0 },
    _coeff_div { mclk: 12000000, rate: 32000, fs: 375, sr: 0x6, bosr: 0x0, usb: 0x1 },

    /* 8k */
    _coeff_div { mclk: 12288000, rate: 8000, fs: 1536, sr: 0x3, bosr: 0x0, usb: 0x0 },
    _coeff_div { mclk: 18432000, rate: 8000, fs: 2304, sr: 0x3, bosr: 0x1, usb: 0x0 },
    _coeff_div { mclk: 11289600, rate: 8000, fs: 1408, sr: 0xb, bosr: 0x0, usb: 0x0 },
    _coeff_div { mclk: 16934400, rate: 8000, fs: 2112, sr: 0xb, bosr: 0x1, usb: 0x0 },
    _coeff_div { mclk: 12000000, rate: 8000, fs: 1500, sr: 0x3, bosr: 0x0, usb: 0x1 },

    /* 96k */
    _coeff_div { mclk: 12288000, rate: 96000, fs: 128, sr: 0x7, bosr: 0x0, usb: 0x0 },
    _coeff_div { mclk: 18432000, rate: 96000, fs: 192, sr: 0x7, bosr: 0x1, usb: 0x0 },
    _coeff_div { mclk: 12000000, rate: 96000, fs: 125, sr: 0x7, bosr: 0x0, usb: 0x1 },

    /* 44.1k */
    _coeff_div { mclk: 11289600, rate: 44100, fs: 256, sr: 0x8, bosr: 0x0, usb: 0x0 },
    _coeff_div { mclk: 16934400, rate: 44100, fs: 384, sr: 0x8, bosr: 0x1, usb: 0x0 },
    _coeff_div { mclk: 12000000, rate: 44100, fs: 272, sr: 0x8, bosr: 0x1, usb: 0x1 },

    /* 88.2k */
    _coeff_div { mclk: 11289600, rate: 88200, fs: 128, sr: 0xf, bosr: 0x0, usb: 0x0 },
    _coeff_div { mclk: 16934400, rate: 88200, fs: 192, sr: 0xf, bosr: 0x1, usb: 0x0 },
    _coeff_div { mclk: 12000000, rate: 88200, fs: 136, sr: 0xf, bosr: 0x1, usb: 0x1 },
];

/* rates constraints */
static WM8731_RATES_12000000: [c_uint; 6] = [8000, 32000, 44100, 48000, 96000, 88200];

static WM8731_RATES_12288000_18432000: [c_uint; 4] = [8000, 32000, 48000, 96000];

static WM8731_RATES_11289600_16934400: [c_uint; 3] = [8000, 44100, 88200];

static WM8731_CONSTRAINTS_12000000: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    list: WM8731_RATES_12000000.as_ptr(),
    count: 6,
};

static WM8731_CONSTRAINTS_12288000_18432000: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    list: WM8731_RATES_12288000_18432000.as_ptr(),
    count: 4,
};

static WM8731_CONSTRAINTS_11289600_16934400: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    list: WM8731_RATES_11289600_16934400.as_ptr(),
    count: 3,
};

#[inline]
unsafe fn get_coeff(mclk: c_int, rate: c_int) -> c_int {
    let mut i = 0usize;

    while i < COEFF_DIV.len() {
        if COEFF_DIV[i].rate == rate as u32 && COEFF_DIV[i].mclk == mclk as u32 {
            return i as c_int;
        }
        i += 1;
    }
    0
}

unsafe extern "C" fn wm8731_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let wm8731 = snd_soc_component_get_drvdata(component);
    let mut iface: u16 = (snd_soc_component_read(component, WM8731_IFACE) & 0xfff3) as u16;
    let i = get_coeff((*wm8731).sysclk as c_int, params_rate(params)) as usize;
    let srate: u16 = (((COEFF_DIV[i].sr as u16) << 2)
        | ((COEFF_DIV[i].bosr as u16) << 1)
        | COEFF_DIV[i].usb as u16) as u16;

    (*wm8731).playback_fs = params_rate(params);

    snd_soc_component_write(component, WM8731_SRATE, srate as c_uint);

    /* bit size */
    match params_width(params) {
        16 => {}
        20 => {
            iface |= 0x0004;
        }
        24 => {
            iface |= 0x0008;
        }
        32 => {
            iface |= 0x000c;
        }
        _ => {}
    }

    wm8731_set_deemph(component);

    snd_soc_component_write(component, WM8731_IFACE, iface as c_uint);
    0
}

unsafe extern "C" fn wm8731_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*dai).component;
    let mute_reg: u16 = (snd_soc_component_read(component, WM8731_APDIGI) & 0xfff7) as u16;

    if mute != 0 {
        snd_soc_component_write(component, WM8731_APDIGI, (mute_reg | 0x8) as c_uint);
    } else {
        snd_soc_component_write(component, WM8731_APDIGI, mute_reg as c_uint);
    }
    0
}

unsafe extern "C" fn wm8731_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = (*codec_dai).component;
    let dapm = snd_soc_component_to_dapm(component);
    let wm8731 = snd_soc_component_get_drvdata(component);

    if clk_id == WM8731_SYSCLK_XTAL || clk_id == WM8731_SYSCLK_MCLK {
        if !(*wm8731).mclk.is_null() && clk_set_rate((*wm8731).mclk, freq) != 0 {
            return -EINVAL;
        }
        (*wm8731).sysclk_type = clk_id;
    } else {
        return -EINVAL;
    }

    match freq {
        0 => {
            (*wm8731).constraints = core::ptr::null();
        }
        12000000 => {
            (*wm8731).constraints = &WM8731_CONSTRAINTS_12000000;
        }
        12288000 | 18432000 => {
            (*wm8731).constraints = &WM8731_CONSTRAINTS_12288000_18432000;
        }
        16934400 | 11289600 => {
            (*wm8731).constraints = &WM8731_CONSTRAINTS_11289600_16934400;
        }
        _ => {
            return -EINVAL;
        }
    }

    (*wm8731).sysclk = freq;

    snd_soc_dapm_sync(dapm);

    0
}

unsafe extern "C" fn wm8731_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let mut iface: u16 = 0;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        x if x == SND_SOC_DAIFMT_CBP_CFP => {
            iface |= 0x0040;
        }
        x if x == SND_SOC_DAIFMT_CBC_CFC => {}
        _ => {
            return -EINVAL;
        }
    }

    /* interface format */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => {
            iface |= 0x0002;
        }
        x if x == SND_SOC_DAIFMT_RIGHT_J => {}
        x if x == SND_SOC_DAIFMT_LEFT_J => {
            iface |= 0x0001;
        }
        x if x == SND_SOC_DAIFMT_DSP_A => {
            iface |= 0x0013;
        }
        x if x == SND_SOC_DAIFMT_DSP_B => {
            iface |= 0x0003;
        }
        _ => {
            return -EINVAL;
        }
    }

    /* clock inversion */
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF => {}
        x if x == SND_SOC_DAIFMT_IB_IF => {
            iface |= 0x0090;
        }
        x if x == SND_SOC_DAIFMT_IB_NF => {
            iface |= 0x0080;
        }
        x if x == SND_SOC_DAIFMT_NB_IF => {
            iface |= 0x0010;
        }
        _ => {
            return -EINVAL;
        }
    }

    /* set iface */
    snd_soc_component_write(component, WM8731_IFACE, iface as c_uint);
    0
}

unsafe extern "C" fn wm8731_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let wm8731 = snd_soc_component_get_drvdata(component);
    let dapm = snd_soc_component_to_dapm(component);
    let mut ret: c_int;
    let reg: u16;

    if core::mem::transmute_copy::<_, usize>(&level) == core::mem::transmute_copy::<_, usize>(&SND_SOC_BIAS_ON) {
        ret = clk_prepare_enable((*wm8731).mclk);
        if ret != 0 {
            return ret;
        }
    } else if core::mem::transmute_copy::<_, usize>(&level) == core::mem::transmute_copy::<_, usize>(&SND_SOC_BIAS_PREPARE) {
    } else if core::mem::transmute_copy::<_, usize>(&level) == core::mem::transmute_copy::<_, usize>(&SND_SOC_BIAS_STANDBY) {
        if core::mem::transmute_copy::<_, usize>(&snd_soc_dapm_get_bias_level(dapm))
            == core::mem::transmute_copy::<_, usize>(&SND_SOC_BIAS_OFF)
        {
            ret = regulator_bulk_enable(array_size(&(*wm8731).supplies), (*wm8731).supplies.as_mut_ptr());
            if ret != 0 {
                return ret;
            }

            regcache_sync((*wm8731).regmap);
        }

        /* Clear PWROFF, gate CLKOUT, everything else as-is */
        reg = (snd_soc_component_read(component, WM8731_PWR) & 0xff7f) as u16;
        snd_soc_component_write(component, WM8731_PWR, (reg | 0x0040) as c_uint);
    } else if core::mem::transmute_copy::<_, usize>(&level) == core::mem::transmute_copy::<_, usize>(&SND_SOC_BIAS_OFF) {
        clk_disable_unprepare((*wm8731).mclk);
        snd_soc_component_write(component, WM8731_PWR, 0xffff);
        regulator_bulk_disable(array_size(&(*wm8731).supplies), (*wm8731).supplies.as_mut_ptr());
        regcache_mark_dirty((*wm8731).regmap);
    }
    0
}

unsafe extern "C" fn wm8731_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let wm8731 = snd_soc_component_get_drvdata((*dai).component);

    if !(*wm8731).constraints.is_null() {
        snd_pcm_hw_constraint_list(
            (*substream).runtime,
            0,
            SNDRV_PCM_HW_PARAM_RATE,
            (*wm8731).constraints,
        );
    }

    0
}

unsafe fn WM8731_RATES() -> c_uint {
    SNDRV_PCM_RATE_8000_96000
}

unsafe fn WM8731_FORMATS() -> c_uint {
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE
}

static WM8731_DAI_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(wm8731_startup),
    hw_params: Some(wm8731_hw_params),
    mute_stream: Some(wm8731_mute),
    set_sysclk: Some(wm8731_set_dai_sysclk),
    set_fmt: Some(wm8731_set_dai_fmt),
    no_capture_mute: 1,
};

static mut WM8731_DAI: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"wm8731-hifi\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: 0, /* WM8731_RATES */
        formats: 0, /* WM8731_FORMATS */
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: 0, /* WM8731_RATES */
        formats: 0, /* WM8731_FORMATS */
    },
    ops: &WM8731_DAI_OPS,
    symmetric_rate: 1,
};

static SOC_COMPONENT_DEV_WM8731: snd_soc_component_driver = snd_soc_component_driver {
    set_bias_level: Some(wm8731_set_bias_level),
    controls: WM8731_SND_CONTROLS.as_ptr(),
    num_controls: 11,
    dapm_widgets: WM8731_DAPM_WIDGETS.as_ptr(),
    num_dapm_widgets: 15,
    dapm_routes: WM8731_INTERCON.as_ptr(),
    num_dapm_routes: 21,
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

#[no_mangle]
pub unsafe extern "C" fn wm8731_init(dev: *mut device, wm8731: *mut wm8731_priv) -> c_int {
    let mut ret: c_int = 0;

    (*wm8731).mclk = devm_clk_get(dev, b"mclk\0".as_ptr() as *const c_char);
    if IS_ERR((*wm8731).mclk as *const core::ffi::c_void) {
        ret = PTR_ERR((*wm8731).mclk as *const core::ffi::c_void);
        if ret == -ENOENT {
            (*wm8731).mclk = core::ptr::null_mut();
            dev_warn(dev, b"Assuming static MCLK\n\0".as_ptr() as *const c_char);
        } else {
            dev_err(dev, b"Failed to get MCLK: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
    }

    mutex_init(core::ptr::addr_of_mut!((*wm8731).lock));

    let mut i = 0usize;
    while i < (*wm8731).supplies.len() {
        (*wm8731).supplies[i].supply = WM8731_SUPPLY_NAMES[i];
        i += 1;
    }

    ret = devm_regulator_bulk_get(dev, array_size(&(*wm8731).supplies), (*wm8731).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, b"Failed to request supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = regulator_bulk_enable(array_size(&(*wm8731).supplies), (*wm8731).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, b"Failed to enable supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = wm8731_reset((*wm8731).regmap);
    if ret < 0 {
        dev_err(dev, b"Failed to issue reset: %d\n\0".as_ptr() as *const c_char, ret);
        goto_err_regulator_enable(wm8731, ret);
        return ret;
    }

    /* Clear POWEROFF, keep everything else disabled */
    regmap_write((*wm8731).regmap, WM8731_PWR, 0x7f);

    /* Latch the update bits */
    regmap_update_bits((*wm8731).regmap, WM8731_LOUT1V, 0x100, 0);
    regmap_update_bits((*wm8731).regmap, WM8731_ROUT1V, 0x100, 0);
    regmap_update_bits((*wm8731).regmap, WM8731_LINVOL, 0x100, 0);
    regmap_update_bits((*wm8731).regmap, WM8731_RINVOL, 0x100, 0);

    /* Disable bypass path by default */
    regmap_update_bits((*wm8731).regmap, WM8731_APANA, 0x8, 0);

    regcache_mark_dirty((*wm8731).regmap);

    WM8731_DAI.playback.rates = WM8731_RATES();
    WM8731_DAI.playback.formats = WM8731_FORMATS();
    WM8731_DAI.capture.rates = WM8731_RATES();
    WM8731_DAI.capture.formats = WM8731_FORMATS();

    ret = devm_snd_soc_register_component(dev, &SOC_COMPONENT_DEV_WM8731, core::ptr::addr_of_mut!(WM8731_DAI), 1);
    if ret != 0 {
        dev_err(dev, b"Failed to register CODEC: %d\n\0".as_ptr() as *const c_char, ret);
        goto_err_regulator_enable(wm8731, ret);
        return ret;
    }

    0
}

unsafe fn goto_err_regulator_enable(wm8731: *mut wm8731_priv, ret: c_int) -> c_int {
    /* Regulators will be enabled by bias management */
    regulator_bulk_disable(array_size(&(*wm8731).supplies), (*wm8731).supplies.as_mut_ptr());

    ret
}

/* EXPORT_SYMBOL_GPL(wm8731_init); */

#[no_mangle]
pub static WM8731_REGMAP: regmap_config = regmap_config {
    reg_bits: 7,
    val_bits: 9,

    max_register: unsafe { WM8731_RESET },
    volatile_reg: Some(wm8731_volatile),

    cache_type: unsafe { REGCACHE_MAPLE },
    reg_defaults: WM8731_REG_DEFAULTS.as_ptr(),
    num_reg_defaults: 10,
};
/* EXPORT_SYMBOL_GPL(wm8731_regmap); */

/* MODULE_DESCRIPTION("ASoC WM8731 driver"); */
/* MODULE_AUTHOR("Richard Purdie"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
