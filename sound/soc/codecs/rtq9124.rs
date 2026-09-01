// SPDX-License-Identifier: GPL-2.0-only
//
// rtq9124.c -- RTQ9124 ALSA SoC Codec driver
//
// Author: ChiYuan Huang <cy_huang@richtek.com>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type size_t = usize;
type u8 = u8;
type u32 = u32;
type __be16 = u16;
type __be32 = u32;

const fn BIT(n: c_uint) -> c_uint {
    1u32 << n
}

const fn GENMASK(h: c_uint, l: c_uint) -> c_uint {
    let high = if h >= 31 { u32::MAX } else { (1u32 << (h + 1)) - 1 };
    let low = if l == 0 { 0 } else { (1u32 << l) - 1 };
    high & !low
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

const RTQ9124_REG_SDI_SEL: c_uint = 0x00;
const RTQ9124_REG_SDO_SEL: c_uint = 0x01;
const RTQ9124_REG_I2S_OPT: c_uint = 0x02;
const RTQ9124_REG_AMP_OPT: c_uint = 0x03;
const RTQ9124_REG_STATE_CTRL: c_uint = 0x04;
const RTQ9124_REG_PWM_PHASE: c_uint = 0x05;
const RTQ9124_REG_SIL_CTRL: c_uint = 0x06;
const RTQ9124_REG_PWM_SS_OPT: c_uint = 0x07;
const RTQ9124_REG_ERR_INT_0: c_uint = 0x10;
const RTQ9124_REG_ERR_MASK6: c_uint = 0x26;
const RTQ9124_REG_TDM_TX_CH0: c_uint = 0x32;
const RTQ9124_REG_TDM_RX_CH0: c_uint = 0x34;
const RTQ9124_REG_VOL_OPT: c_uint = 0x38;
const RTQ9124_REG_DCR_TH: c_uint = 0x4B;
const RTQ9124_REG_ERR_TH: c_uint = 0x4C;
const RTQ9124_REG_PROT_EN: c_uint = 0x5B;
const RTQ9124_REG_PRJ_CODE: c_uint = 0xF9;

const RTQ9124_MASK_CS_DATA_INV: c_uint = BIT(9);
const RTQ9124_MASK_VDDIO_SDO_SEL: c_uint = BIT(8);
const RTQ9124_MASK_AUD_BITS: c_uint = GENMASK(5, 4);
const RTQ9124_MASK_AUD_FMT: c_uint = GENMASK(3, 0);
const RTQ9124_MASK_CH_STATE: c_uint = GENMASK(1, 0);
const RTQ9124_MASK_SF_RESET: c_uint = BIT(15);

const RTQ9124_FIXED_VENID: c_int = 0x9124;

const EINVAL: c_int = 22;
const EIO: c_int = 5;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_HIGH: c_uint = 0;
const REGCACHE_MAPLE: c_uint = 0;
const SND_SOC_DAPM_POST_PMU: c_int = 1;
const SND_SOC_DAPM_POST_PMD: c_int = 2;
const SND_SOC_NOPM: c_int = -1;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 2;
const SND_SOC_DAIFMT_RIGHT_J: c_uint = 3;
const SND_SOC_DAIFMT_DSP_A: c_uint = 4;
const SND_SOC_DAIFMT_DSP_B: c_uint = 5;
const SNDRV_PCM_FMTBIT_S16: c_uint = 1 << 0;
const SNDRV_PCM_FMTBIT_S24: c_uint = 1 << 1;
const SNDRV_PCM_FMTBIT_S32: c_uint = 1 << 2;
const SNDRV_PCM_RATE_8000_48000: c_uint = 1 << 0;
const SNDRV_PCM_RATE_24000: c_uint = 1 << 1;
const SNDRV_PCM_RATE_88200: c_uint = 1 << 2;
const SNDRV_PCM_RATE_96000: c_uint = 1 << 3;
const SNDRV_PCM_RATE_192000: c_uint = 1 << 4;

#[repr(C)]
struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_kcontrol {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_soc_component {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_pcm_substream {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct i2c_client {
    dev: device,
}

#[repr(C)]
struct snd_soc_dapm_widget {
    dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
struct snd_soc_dai {
    component: *mut snd_soc_component,
    dev: *mut device,
}

#[repr(C)]
struct rtq9124_priv {
    enable: *mut gpio_desc,
    dai_fmt: c_uint,
    tdm_slots: c_int,
    tdm_slot_width: c_int,
}

#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
struct soc_enum {
    reg: c_uint,
    shift_l: c_uint,
    shift_r: c_uint,
    items: c_uint,
    texts: *const *const c_char,
}

#[repr(C)]
struct snd_kcontrol_new {
    name: *const c_char,
    reg: c_uint,
    shift: c_uint,
    max: c_uint,
    invert: c_uint,
    tlv: *const c_uint,
    enum_: *const soc_enum,
}

#[repr(C)]
struct snd_soc_component_driver {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    controls: *const snd_kcontrol_new,
    num_controls: usize,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: usize,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: usize,
    use_pmdown_time: c_uint,
    endianness: c_uint,
}

#[repr(C)]
struct snd_soc_dai_ops {
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    set_tdm_slot:
        Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int,
    >,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    formats: c_uint,
    rates: c_uint,
    rate_min: c_uint,
    rate_max: c_uint,
    channels_min: c_uint,
    channels_max: c_uint,
}

#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    playback: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
    symmetric_rate: c_uint,
    symmetric_sample_bits: c_uint,
}

#[repr(C)]
struct regmap_config {
    name: *const c_char,
    reg_bits: c_uint,
    val_bits: c_uint,
    read: Option<unsafe extern "C" fn(*mut c_void, *const c_void, size_t, *mut c_void, size_t) -> c_int>,
    write: Option<unsafe extern "C" fn(*mut c_void, *const c_void, size_t) -> c_int>,
    readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    cache_type: c_uint,
    num_reg_defaults_raw: c_uint,
    use_single_read: c_uint,
    use_single_write: c_uint,
}

#[repr(C)]
struct reg_sequence {
    reg: c_uint,
    def: c_uint,
}

#[repr(C)]
struct dev_pm_ops {
    runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    runtime_idle: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
struct device_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
    pm: *const dev_pm_ops,
}

#[repr(C)]
struct i2c_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
}

unsafe extern "C" {
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_write(comp: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_write_field(
        comp: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn params_width(param: *mut snd_pcm_hw_params) -> c_uint;
    fn params_physical_width(param: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(param: *mut snd_pcm_hw_params) -> c_uint;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn i2c_smbus_read_word_swapped(i2c: *mut i2c_client, command: u8) -> c_int;
    fn i2c_smbus_write_word_swapped(i2c: *mut i2c_client, command: u8, value: u16) -> c_int;
    fn i2c_smbus_read_i2c_block_data(
        i2c: *mut i2c_client,
        command: u8,
        length: u8,
        values: *mut c_void,
    ) -> c_int;
    fn i2c_smbus_write_i2c_block_data(
        i2c: *mut i2c_client,
        command: u8,
        length: u8,
        values: *mut u8,
    ) -> c_int;
    fn devm_regmap_init(
        dev: *mut device,
        bus: *const c_void,
        bus_context: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn regmap_register_patch(regmap: *mut regmap, regs: *const reg_sequence, num_regs: c_int) -> c_int;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn devm_pm_runtime_enable(dev: *mut device) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_get_regmap(dev: *mut device, name: *const c_char) -> *mut regmap;
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regcache_mark_dirty(map: *mut regmap);
    fn gpiod_set_value(desc: *mut gpio_desc, value: c_int);
    fn regcache_sync(map: *mut regmap) -> c_int;
}

unsafe fn dev_dbg(_dev: *mut device, _fmt: *const c_char) {}
unsafe fn dev_err(_dev: *mut device, _fmt: *const c_char) {}
unsafe fn dev_err_probe(_dev: *mut device, err: c_int, _fmt: *const c_char) -> c_int {
    err
}

const fn FIELD_GET(mask: c_uint, reg: c_uint) -> c_uint {
    if mask == 0 {
        reg
    } else {
        (reg & mask) >> mask.trailing_zeros()
    }
}

fn hweight_long(v: c_uint) -> c_uint {
    v.count_ones()
}

fn fls(v: c_uint) -> c_uint {
    if v == 0 {
        0
    } else {
        c_uint::BITS - v.leading_zeros()
    }
}

fn ffs(v: c_uint) -> c_uint {
    if v == 0 {
        0
    } else {
        v.trailing_zeros() + 1
    }
}

unsafe extern "C" fn rtq9124_enable_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let comp = snd_soc_dapm_to_component((*w).dapm);
    let mut i: c_uint;
    let chan_state: c_uint;

    match event {
        SND_SOC_DAPM_POST_PMU => {
            /* Change state to normal */
            chan_state = 0;
        }
        SND_SOC_DAPM_POST_PMD => {
            /* Change state to HiZ */
            chan_state = 1;
        }
        _ => return -EINVAL,
    }

    /* Before amp turn on, clear old events first */
    i = 0;
    while chan_state == 0 && i < 8 {
        snd_soc_component_write(comp, RTQ9124_REG_ERR_INT_0 + i, 0xffff);
        i += 1;
    }

    snd_soc_component_write_field(
        comp,
        RTQ9124_REG_STATE_CTRL,
        RTQ9124_MASK_CH_STATE,
        chan_state,
    );

    0
}

static rtq9124_dapm_widgets: [snd_soc_dapm_widget; 2] = [
    /* SND_SOC_DAPM_OUTPUT("SPK") */
    snd_soc_dapm_widget { dapm: ptr::null_mut() },
    /* SND_SOC_DAPM_OUT_DRV_E("Amp Drv", SND_SOC_NOPM, 0, 0, NULL, 0,
     *                         rtq9124_enable_event,
     *                         SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD)
     */
    snd_soc_dapm_widget { dapm: ptr::null_mut() },
];

static rtq9124_dapm_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: b"Amp Drv\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"HiFi Playback\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"SPK\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Amp Drv\0".as_ptr() as *const c_char,
    },
];

static dig_tlv: [c_uint; 4] = [0, (-10375i32) as c_uint, 25, 0];
static ana_tlv: [c_uint; 8] = [0, 3, (-600i32) as c_uint, 600, 0, 4, 6, 1400];
static i2sch_text: [*const c_char; 4] = [
    b"(L+R)/2\0".as_ptr() as *const c_char,
    b"LCH\0".as_ptr() as *const c_char,
    b"RCH\0".as_ptr() as *const c_char,
    b"(L+R)/2\0".as_ptr() as *const c_char,
];
static rtq9124_i2sch_select_enum: soc_enum = soc_enum {
    reg: RTQ9124_REG_SDI_SEL,
    shift_l: 0,
    shift_r: 0,
    items: ARRAY_SIZE(&i2sch_text) as c_uint,
    texts: i2sch_text.as_ptr(),
};
static sdo_vsel_text: [*const c_char; 2] = [
    b"1.8V\0".as_ptr() as *const c_char,
    b"3.3V\0".as_ptr() as *const c_char,
];
static rtq9124_sdo_vselect_enum: soc_enum = soc_enum {
    reg: RTQ9124_REG_SDO_SEL,
    shift_l: 8,
    shift_r: 8,
    items: ARRAY_SIZE(&sdo_vsel_text) as c_uint,
    texts: sdo_vsel_text.as_ptr(),
};
static pwmfreq_text: [*const c_char; 5] = [
    b"8*fs\0".as_ptr() as *const c_char,
    b"10*fs\0".as_ptr() as *const c_char,
    b"40*fs\0".as_ptr() as *const c_char,
    b"44*fs\0".as_ptr() as *const c_char,
    b"48*fs\0".as_ptr() as *const c_char,
];
static rtq9124_pwm_freq_enum: soc_enum = soc_enum {
    reg: RTQ9124_REG_AMP_OPT,
    shift_l: 4,
    shift_r: 4,
    items: ARRAY_SIZE(&pwmfreq_text) as c_uint,
    texts: pwmfreq_text.as_ptr(),
};
static out_angle_text: [*const c_char; 8] = [
    b"0\0".as_ptr() as *const c_char,
    b"45\0".as_ptr() as *const c_char,
    b"90\0".as_ptr() as *const c_char,
    b"135\0".as_ptr() as *const c_char,
    b"180\0".as_ptr() as *const c_char,
    b"225\0".as_ptr() as *const c_char,
    b"270\0".as_ptr() as *const c_char,
    b"315\0".as_ptr() as *const c_char,
];
static rtq9124_out_angle_enum: soc_enum = soc_enum {
    reg: RTQ9124_REG_PWM_PHASE,
    shift_l: 0,
    shift_r: 0,
    items: ARRAY_SIZE(&out_angle_text) as c_uint,
    texts: out_angle_text.as_ptr(),
};
static sdo_select_text: [*const c_char; 9] = [
    b"None\0".as_ptr() as *const c_char,
    b"I2S DataI\0".as_ptr() as *const c_char,
    b"Interface\0".as_ptr() as *const c_char,
    b"DSP\0".as_ptr() as *const c_char,
    b"DF\0".as_ptr() as *const c_char,
    b"ISense\0".as_ptr() as *const c_char,
    b"ACLoad Cos\0".as_ptr() as *const c_char,
    b"ACLoad Sin\0".as_ptr() as *const c_char,
    b"DCR\0".as_ptr() as *const c_char,
];
static rtq9124_sdo_select_enum: soc_enum = soc_enum {
    reg: RTQ9124_REG_SDO_SEL,
    shift_l: 4,
    shift_r: 0,
    items: ARRAY_SIZE(&sdo_select_text) as c_uint,
    texts: sdo_select_text.as_ptr(),
};
static ulqm_dcvt_text: [*const c_char; 4] = [
    b"Disable\0".as_ptr() as *const c_char,
    b"DC\0".as_ptr() as *const c_char,
    b"VT\0".as_ptr() as *const c_char,
    b"DC+VT\0".as_ptr() as *const c_char,
];
static rtq9124_ulqm_dcvt_select_enum: soc_enum = soc_enum {
    reg: RTQ9124_REG_STATE_CTRL,
    shift_l: 10,
    shift_r: 10,
    items: ARRAY_SIZE(&ulqm_dcvt_text) as c_uint,
    texts: ulqm_dcvt_text.as_ptr(),
};

static rtq9124_controls: [snd_kcontrol_new; 10] = [
    snd_kcontrol_new { name: b"Master Volume\0".as_ptr() as *const c_char, reg: RTQ9124_REG_VOL_OPT, shift: 2, max: 511, invert: 1, tlv: dig_tlv.as_ptr(), enum_: ptr::null() },
    snd_kcontrol_new { name: b"Speaker Volume\0".as_ptr() as *const c_char, reg: RTQ9124_REG_AMP_OPT, shift: 0, max: 6, invert: 0, tlv: ana_tlv.as_ptr(), enum_: ptr::null() },
    snd_kcontrol_new { name: b"I2S CH Select\0".as_ptr() as *const c_char, reg: 0, shift: 0, max: 0, invert: 0, tlv: ptr::null(), enum_: &rtq9124_i2sch_select_enum },
    snd_kcontrol_new { name: b"SDO VDDIO Select\0".as_ptr() as *const c_char, reg: 0, shift: 0, max: 0, invert: 0, tlv: ptr::null(), enum_: &rtq9124_sdo_vselect_enum },
    snd_kcontrol_new { name: b"PWM Frequency Select\0".as_ptr() as *const c_char, reg: 0, shift: 0, max: 0, invert: 0, tlv: ptr::null(), enum_: &rtq9124_pwm_freq_enum },
    snd_kcontrol_new { name: b"PWM Output Phase Select\0".as_ptr() as *const c_char, reg: 0, shift: 0, max: 0, invert: 0, tlv: ptr::null(), enum_: &rtq9124_out_angle_enum },
    snd_kcontrol_new { name: b"SDO Select\0".as_ptr() as *const c_char, reg: 0, shift: 0, max: 0, invert: 0, tlv: ptr::null(), enum_: &rtq9124_sdo_select_enum },
    snd_kcontrol_new { name: b"ULQM DCVT Select\0".as_ptr() as *const c_char, reg: 0, shift: 0, max: 0, invert: 0, tlv: ptr::null(), enum_: &rtq9124_ulqm_dcvt_select_enum },
    snd_kcontrol_new { name: b"Silence Detect Enable Switch\0".as_ptr() as *const c_char, reg: RTQ9124_REG_SIL_CTRL, shift: 7, max: 1, invert: 0, tlv: ptr::null(), enum_: ptr::null() },
    snd_kcontrol_new { name: b"Spread Spectrum Enable Switch\0".as_ptr() as *const c_char, reg: RTQ9124_REG_PWM_SS_OPT, shift: 7, max: 1, invert: 0, tlv: ptr::null(), enum_: ptr::null() },
];

unsafe extern "C" fn rtq9124_comp_probe(comp: *mut snd_soc_component) -> c_int {
    /* CS Data INV */
    snd_soc_component_write_field(comp, RTQ9124_REG_SDO_SEL, RTQ9124_MASK_CS_DATA_INV, 1);

    /* RTLD */
    snd_soc_component_write(comp, RTQ9124_REG_DCR_TH, 0x5e30);
    snd_soc_component_write(comp, RTQ9124_REG_ERR_TH, 0x3ff);
    snd_soc_component_write(comp, RTQ9124_REG_PROT_EN, 0x3fc);
    snd_soc_component_write(comp, RTQ9124_REG_ERR_MASK6, 0);

    0
}

static rtq9124_comp_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(rtq9124_comp_probe),
    controls: rtq9124_controls.as_ptr(),
    num_controls: ARRAY_SIZE(&rtq9124_controls),
    dapm_widgets: rtq9124_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE(&rtq9124_dapm_widgets),
    dapm_routes: rtq9124_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE(&rtq9124_dapm_routes),
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn rtq9124_dai_set_format(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let rtq9124 = snd_soc_dai_get_drvdata(dai) as *mut rtq9124_priv;

    (*rtq9124).dai_fmt = fmt;
    0
}

unsafe extern "C" fn rtq9124_dai_set_tdm_slot(
    dai: *mut snd_soc_dai,
    mut tx_mask: c_uint,
    rx_mask: c_uint,
    slots: c_int,
    slot_width: c_int,
) -> c_int {
    let rtq9124 = snd_soc_dai_get_drvdata(dai) as *mut rtq9124_priv;
    let comp = (*dai).component;
    let dev = (*dai).dev;
    let mut byte_loc: c_uint;
    let mut i: c_uint;

    dev_dbg(dev, b"(slots, slot_width) = (%d, %d), (txmask, rxmask) = 0x%x, 0x%x\n\0".as_ptr() as *const c_char);

    if slots <= 0 || slots > 16 || slot_width <= 0 || slots % 2 != 0 || slot_width % 8 != 0 {
        dev_err(dev, b"Invalid slot parameter (%d, %d)\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    if tx_mask != 0 && (hweight_long(tx_mask) > 2 || fls(tx_mask) > slots as c_uint) {
        dev_err(dev, b"Invalid tx_mask 0x%08x, slots = %d\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    if rx_mask == 0 || hweight_long(rx_mask) > 1 || fls(rx_mask) > slots as c_uint {
        dev_err(dev, b"Invalid rx_mask 0x%08x, slots = %d\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    /* Configure tx channel data location */
    i = 0;
    while tx_mask != 0 {
        byte_loc = (ffs(tx_mask) - 1) * slot_width as c_uint / 8;
        snd_soc_component_write(comp, RTQ9124_REG_TDM_TX_CH0 + i, byte_loc);
        tx_mask ^= BIT(ffs(tx_mask) - 1);
        i += 1;
    }

    /* Configure rx channel data location */
    byte_loc = (ffs(rx_mask) - 1) * slot_width as c_uint / 8;
    snd_soc_component_write(comp, RTQ9124_REG_TDM_RX_CH0, byte_loc);

    (*rtq9124).tdm_slots = slots;
    (*rtq9124).tdm_slot_width = slot_width;

    0
}

unsafe extern "C" fn rtq9124_dai_hw_params(
    _substream: *mut snd_pcm_substream,
    param: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let rtq9124 = snd_soc_dai_get_drvdata(dai) as *mut rtq9124_priv;
    let comp = (*dai).component;
    let mut fmtval: c_uint;
    let width: c_uint;
    let slot_width: c_uint;
    let bitrate: c_uint;
    let dev = (*dai).dev;
    let audfmt: c_uint;
    let audbit: c_uint;

    fmtval = FIELD_GET(SND_SOC_DAIFMT_FORMAT_MASK, (*rtq9124).dai_fmt);
    if (*rtq9124).tdm_slots != 0
        && fmtval != SND_SOC_DAIFMT_DSP_A
        && fmtval != SND_SOC_DAIFMT_DSP_B
    {
        dev_err(dev, b"TDM only can support DSP_A or DSP_B format\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    match fmtval {
        SND_SOC_DAIFMT_I2S => audfmt = 0,
        SND_SOC_DAIFMT_LEFT_J => audfmt = 1,
        SND_SOC_DAIFMT_RIGHT_J => audfmt = 2,
        SND_SOC_DAIFMT_DSP_B => audfmt = if (*rtq9124).tdm_slots != 0 { 7 } else { 3 },
        SND_SOC_DAIFMT_DSP_A => audfmt = if (*rtq9124).tdm_slots != 0 { 15 } else { 11 },
        _ => {
            dev_err(dev, b"Unsupported format %d\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    width = params_width(param);
    match width {
        16 => audbit = 0,
        20 => audbit = 1,
        24 | 32 => audbit = 3,
        _ => {
            dev_err(dev, b"Unsupported width %d\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    if (*rtq9124).tdm_slots != 0 {
        slot_width = params_physical_width(param);
        if slot_width > (*rtq9124).tdm_slot_width as c_uint {
            dev_err(dev, b"Slot width is larger than TDM slot width\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }

        bitrate = ((*rtq9124).tdm_slots as c_uint)
            .wrapping_mul((*rtq9124).tdm_slot_width as c_uint)
            .wrapping_mul(params_rate(param));
        if bitrate > 24576000 {
            dev_err(dev, b"Bitrate exceed the internal PLL 24.576MHz (%d)\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    snd_soc_component_write_field(comp, RTQ9124_REG_I2S_OPT, RTQ9124_MASK_AUD_FMT, audfmt);
    snd_soc_component_write_field(comp, RTQ9124_REG_I2S_OPT, RTQ9124_MASK_AUD_BITS, audbit);

    0
}

static rtq9124_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(rtq9124_dai_set_format),
    set_tdm_slot: Some(rtq9124_dai_set_tdm_slot),
    hw_params: Some(rtq9124_dai_hw_params),
};

static mut rtq9124_dai_driver: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"HiFi\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"HiFi Playback\0".as_ptr() as *const c_char,
        formats: SNDRV_PCM_FMTBIT_S16 | SNDRV_PCM_FMTBIT_S24 | SNDRV_PCM_FMTBIT_S32,
        rates: SNDRV_PCM_RATE_8000_48000
            | SNDRV_PCM_RATE_24000
            | SNDRV_PCM_RATE_88200
            | SNDRV_PCM_RATE_96000
            | SNDRV_PCM_RATE_192000,
        rate_min: 8000,
        rate_max: 192000,
        channels_min: 1,
        channels_max: 2,
    },
    ops: &rtq9124_dai_ops,
    symmetric_rate: 1,
    symmetric_sample_bits: 1,
};

unsafe extern "C" fn rtq9124_readable_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        0x00..=0x17
        | 0x20..=0x27
        | 0x30..=0x3D
        | 0x40..=0x68
        | 0x80..=0xBC
        | 0xC0..=0xDE
        | 0xE0..=0xE7
        | 0xF0..=0xFD => true,
        _ => false,
    }
}

unsafe extern "C" fn rtq9124_writeable_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        0x00..=0x09
        | 0x0C..=0x0E
        | 0x10..=0x17
        | 0x20..=0x27
        | 0x30
        | 0x32..=0x3D
        | 0x40..=0x4E
        | 0x50..=0x68
        | 0x80..=0xBC
        | 0xC0..=0xDE
        | 0xE0..=0xE7
        | 0xF0..=0xFD => true,
        _ => false,
    }
}

unsafe extern "C" fn rtq9124_volatile_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        0x0A..=0x0B
        | 0x0F..=0x17
        | 0x31
        | 0x4F
        | 0x51
        | 0x53..=0x57
        | 0x80..=0xBC
        | 0xC0..=0xDE
        | 0xE0..=0xE7
        | 0xF0..=0xFD => true,
        _ => false,
    }
}

fn rtq9124_get_reg_len(reg: c_uint) -> u8 {
    if reg >= 0x40 && reg <= 0x47 {
        4
    } else {
        2
    }
}

unsafe extern "C" fn rtq9124_regmap_read(
    context: *mut c_void,
    reg_buf: *const c_void,
    _reg_size: size_t,
    val_buf: *mut c_void,
    _val_size: size_t,
) -> c_int {
    let i2c = context as *mut i2c_client;
    let reg = *(reg_buf as *const u8);
    let size = rtq9124_get_reg_len(reg as c_uint);
    let val = val_buf as *mut u32;
    let ret: c_int;

    ret = i2c_smbus_read_i2c_block_data(i2c, reg, size, val_buf);
    if ret < 0 {
        return ret;
    } else if ret != size as c_int {
        return -EIO;
    }

    *val = if size == 4 {
        u32::from_be(*(val_buf as *const u32))
    } else {
        u16::from_be(*(val_buf as *const u16)) as u32
    };

    0
}

unsafe extern "C" fn rtq9124_regmap_write(
    context: *mut c_void,
    data: *const c_void,
    _count: size_t,
) -> c_int {
    let i2c = context as *mut i2c_client;
    let reg = *(data as *const u8);
    let vbuf: *mut u8;
    let size = rtq9124_get_reg_len(reg as c_uint);
    let val16: __be16 = (*(data.add(1) as *const u16)).to_be();
    let val32: __be32 = (*(data.add(1) as *const u32)).to_be();

    vbuf = if size == 4 {
        &val32 as *const __be32 as *mut u8
    } else {
        &val16 as *const __be16 as *mut u8
    };
    i2c_smbus_write_i2c_block_data(i2c, reg, size, vbuf)
}

static rtq9124_regmap_config: regmap_config = regmap_config {
    name: b"rtq9124\0".as_ptr() as *const c_char,
    reg_bits: 8,
    val_bits: 32,
    read: Some(rtq9124_regmap_read),
    write: Some(rtq9124_regmap_write),
    readable_reg: Some(rtq9124_readable_reg),
    writeable_reg: Some(rtq9124_writeable_reg),
    volatile_reg: Some(rtq9124_volatile_reg),
    cache_type: REGCACHE_MAPLE,
    num_reg_defaults_raw: 0xFD + 1,
    use_single_read: 1,
    use_single_write: 1,
};

static rtq9124_init_regs: [reg_sequence; 3] = [
    reg_sequence { reg: 0xfb, def: 0x0065 },
    reg_sequence { reg: 0x93, def: 0x2000 },
    reg_sequence { reg: 0xfb, def: 0x0000 },
];

unsafe extern "C" fn rtq9124_probe(i2c: *mut i2c_client) -> c_int {
    let dev = &mut (*i2c).dev as *mut device;
    let rtq9124: *mut rtq9124_priv;
    let regmap: *mut regmap;
    let mut ret: c_int;

    rtq9124 = devm_kzalloc(dev, size_of::<rtq9124_priv>(), GFP_KERNEL) as *mut rtq9124_priv;
    if rtq9124.is_null() {
        return -ENOMEM;
    }

    (*rtq9124).enable =
        devm_gpiod_get_optional(dev, b"enable\0".as_ptr() as *const c_char, GPIOD_OUT_HIGH);
    if IS_ERR((*rtq9124).enable as *const c_void) {
        return PTR_ERR((*rtq9124).enable as *const c_void);
    } else if !(*rtq9124).enable.is_null() {
        usleep_range(6000, 7000);
    } else {
        dev_dbg(dev, b"No 'enable' GPIO specified, treat it as default on\n\0".as_ptr() as *const c_char);
    }

    /* Check vendor id information */
    ret = i2c_smbus_read_word_swapped(i2c, RTQ9124_REG_PRJ_CODE as u8);
    if ret < 0 {
        return dev_err_probe(dev, ret, b"Failed to read project code\n\0".as_ptr() as *const c_char);
    } else if ret != RTQ9124_FIXED_VENID {
        return dev_err_probe(dev, -ENODEV, b"Incorrect project-code 0x%04x\n\0".as_ptr() as *const c_char);
    }

    /* Trigger RG reset before regmap init */
    ret = i2c_smbus_write_word_swapped(
        i2c,
        RTQ9124_REG_STATE_CTRL as u8,
        RTQ9124_MASK_SF_RESET as u16,
    );
    if ret != 0 {
        return dev_err_probe(dev, ret, b"Failed to trigger RG reset\n\0".as_ptr() as *const c_char);
    }

    /* Need to wait 10ms for the reset to complete */
    usleep_range(10000, 11000);

    regmap = devm_regmap_init(
        dev,
        ptr::null(),
        i2c as *mut c_void,
        &rtq9124_regmap_config,
    );
    if IS_ERR(regmap as *const c_void) {
        return dev_err_probe(dev, PTR_ERR(regmap as *const c_void), b"Failed to init regmap\n\0".as_ptr() as *const c_char);
    }

    ret = regmap_register_patch(
        regmap,
        rtq9124_init_regs.as_ptr(),
        ARRAY_SIZE(&rtq9124_init_regs) as c_int,
    );
    if ret != 0 {
        return dev_err_probe(dev, ret, b"Failed to register regmap patch\n\0".as_ptr() as *const c_char);
    }

    i2c_set_clientdata(i2c, rtq9124 as *mut c_void);

    pm_runtime_set_autosuspend_delay(dev, 1000);
    pm_runtime_use_autosuspend(dev);
    pm_runtime_set_active(dev);
    ret = devm_pm_runtime_enable(dev);
    if ret != 0 {
        return dev_err_probe(dev, ret, b"Failed to enable pm runtime\n\0".as_ptr() as *const c_char);
    }

    devm_snd_soc_register_component(
        dev,
        &rtq9124_comp_driver,
        &mut rtq9124_dai_driver,
        1,
    )
}

/* CONFIG_PM */
unsafe extern "C" fn rtq9124_runtime_suspend(dev: *mut device) -> c_int {
    let rtq9124 = dev_get_drvdata(dev) as *mut rtq9124_priv;
    let regmap = dev_get_regmap(dev, ptr::null());

    if !(*rtq9124).enable.is_null() {
        regcache_cache_only(regmap, true);
        regcache_mark_dirty(regmap);
        gpiod_set_value((*rtq9124).enable, 0);
    }

    0
}

unsafe extern "C" fn rtq9124_runtime_resume(dev: *mut device) -> c_int {
    let rtq9124 = dev_get_drvdata(dev) as *mut rtq9124_priv;
    let regmap = dev_get_regmap(dev, ptr::null());
    let ret: c_int;

    if !(*rtq9124).enable.is_null() {
        gpiod_set_value((*rtq9124).enable, 1);
        usleep_range(6000, 7000);

        regcache_cache_only(regmap, false);
        ret = regcache_sync(regmap);
        if ret != 0 {
            return ret;
        }
    }

    0
}

static rtq9124_dev_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(rtq9124_runtime_suspend),
    runtime_resume: Some(rtq9124_runtime_resume),
    runtime_idle: None,
};

/* CONFIG_OF */
static rtq9124_device_id: [of_device_id; 2] = [
    of_device_id {
        compatible: b"richtek,rtq9124\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, rtq9124_device_id); */

static mut rtq9124_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"rtq9124\0".as_ptr() as *const c_char,
        of_match_table: rtq9124_device_id.as_ptr(),
        pm: &rtq9124_dev_pm_ops,
    },
    probe: Some(rtq9124_probe),
};
/* module_i2c_driver(rtq9124_driver); */

/* MODULE_AUTHOR("ChiYuan Huang <cy_huang@richtek.com>"); */
/* MODULE_DESCRIPTION("ASoC RTQ9124 Driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
