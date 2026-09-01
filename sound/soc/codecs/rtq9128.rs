// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2023 Richtek Technology Corp.
//
// Author: ChiYuan Huang <cy_huang@richtek.com>
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;
type bool_ = bool;
type u8 = u8;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}
#[repr(C)]
pub struct gpio_desc {
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
pub struct snd_pcm_substream {
    _private: [u8; 0],
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
pub struct snd_soc_component {
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub name: *const c_char,
    pub dapm: *mut snd_soc_dapm_context,
}
#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
    pub component: *mut snd_soc_component,
}
#[repr(C)]
pub struct regmap_bus {
    pub write: Option<unsafe extern "C" fn(*mut c_void, *const c_void, size_t) -> c_int>,
    pub read: Option<
        unsafe extern "C" fn(*mut c_void, *const c_void, size_t, *mut c_void, size_t) -> c_int,
    >,
    pub max_raw_read: c_uint,
    pub max_raw_write: c_uint,
}
#[repr(C)]
pub struct regmap_config {
    pub name: *const c_char,
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub val_format_endian: c_uint,
    pub cache_type: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub num_reg_defaults_raw: c_uint,
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
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}
#[repr(C)]
pub struct snd_soc_dapm_widget_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_desc,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_tdm_slot:
        Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int,
    >,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub no_capture_mute: c_uint,
}
#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub rates: c_uint,
    pub formats: u64,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
    pub symmetric_sample_bits: c_uint,
}
#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}
#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}
#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}
#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
}

const EINVAL: c_int = 22;
const EIO: c_int = 5;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_HIGH: c_uint = 1;
const REGMAP_ENDIAN_BIG: c_uint = 1;
const REGCACHE_MAPLE: c_uint = 0;
const SND_SOC_NOPM: c_int = -1;
const SND_SOC_DAPM_POST_PMU: c_int = 0x1;
const SND_SOC_DAPM_POST_PMD: c_int = 0x2;
const SND_SOC_DAIFMT_MASTER_MASK: c_uint = 0;
const SND_SOC_DAIFMT_BC_FC: c_uint = 0;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 2;
const SND_SOC_DAIFMT_RIGHT_J: c_uint = 3;
const SND_SOC_DAIFMT_DSP_A: c_uint = 4;
const SND_SOC_DAIFMT_DSP_B: c_uint = 5;
const SNDRV_PCM_RATE_8000_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_S18_3LE: u64 = 1 << 1;
const SNDRV_PCM_FMTBIT_S20_LE: u64 = 1 << 2;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 3;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 4;

const fn BIT(n: c_uint) -> c_uint {
    1u32 << n
}
const fn GENMASK(h: c_uint, l: c_uint) -> c_uint {
    (!0u32 >> (31 - h)) & (!0u32 << l)
}
const fn FIELD_GET(mask: c_uint, reg: c_uint) -> c_uint {
    (reg & mask) >> mask.trailing_zeros()
}
const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> c_uint {
    N as c_uint
}

unsafe extern "C" {
    fn to_i2c_client(dev: *mut device) -> *mut i2c_client;
    fn i2c_smbus_write_i2c_block_data(
        client: *mut i2c_client,
        command: u8,
        length: u8,
        values: *const u8,
    ) -> c_int;
    fn i2c_smbus_read_i2c_block_data(
        client: *mut i2c_client,
        command: u8,
        length: u8,
        values: *mut u8,
    ) -> c_int;
    fn i2c_smbus_write_byte_data(client: *mut i2c_client, command: u8, value: u8) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(comp: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_dapm_widget_name_cmp(w: *mut snd_soc_dapm_widget, s: *const c_char) -> c_int;
    fn snd_soc_component_write_field(
        comp: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_write(comp: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_update_bits(
        comp: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_read(comp: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn msleep(msecs: c_uint);
    fn usleep_range(min: c_uint, max: c_uint);
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_put(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn devm_pm_runtime_enable(dev: *mut device) -> c_int;
    fn params_width(param: *mut snd_pcm_hw_params) -> c_uint;
    fn params_physical_width(param: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(param: *mut snd_pcm_hw_params) -> c_uint;
    fn hweight_long(w: c_uint) -> c_uint;
    fn fls(x: c_uint) -> c_int;
    fn ffs(x: c_uint) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint)
        -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn device_property_read_bool(dev: *mut device, propname: *const c_char) -> bool_;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_regmap_init(
        dev: *mut device,
        bus: *const regmap_bus,
        bus_context: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_get_regmap(dev: *mut device, name: *const c_char) -> *mut regmap;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
}

const RTQ9128_REG_SDI_SEL: c_uint = 0x00;
const RTQ9128_REG_SDO_SEL: c_uint = 0x01;
const RTQ9128_REG_I2S_OPT: c_uint = 0x02;
const RTQ9128_REG_MISC: c_uint = 0x03;
const RTQ9128_REG_STATE_CTRL: c_uint = 0x04;
const RTQ9128_REG_PLLTRI_GEN1: c_uint = 0x05;
const RTQ9128_REG_PLLTRI_GEN2: c_uint = 0x06;
const RTQ9128_REG_PWM_SS_OPT: c_uint = 0x07;
const RTQ9128_REG_DSP_EN: c_uint = 0x08;
const RTQ9128_REG_TDM_TX_CH1: c_uint = 0x21;
const RTQ9128_REG_TDM_RX_CH1: c_uint = 0x25;
const RTQ9128_REG_MS_VOL: c_uint = 0x30;
const RTQ9128_REG_CH1_VOL: c_uint = 0x31;
const RTQ9128_REG_CH2_VOL: c_uint = 0x32;
const RTQ9128_REG_CH3_VOL: c_uint = 0x33;
const RTQ9128_REG_CH4_VOL: c_uint = 0x34;
const RTQ9128_REG_PROT_OPT: c_uint = 0x71;
const RTQ9128_REG_EFUSE_DATA: c_uint = 0xE0;
const RTQ9128_REG_VENDOR_ID: c_uint = 0xF9;

const RTQ9154_REG_CH1_VOL: c_uint = 0x34;
const RTQ9154_REG_CH2_VOL: c_uint = 0x33;
const RTQ9154_REG_CH3_VOL: c_uint = 0x32;
const RTQ9154_REG_CH4_VOL: c_uint = 0x31;
const RTQ9154_REG_AUTOULQM: c_uint = 0xAD;

const RTQ9128_CHSTAT_VAL_MASK: c_uint = GENMASK(1, 0);
const RTQ9128_DOLEN_MASK: c_uint = GENMASK(7, 6);
const RTQ9128_TDMSRCIN_MASK: c_uint = GENMASK(5, 4);
const RTQ9128_AUDBIT_MASK: c_uint = GENMASK(5, 4);
const RTQ9128_AUDFMT_MASK: c_uint = GENMASK(3, 0);
const RTQ9128_MSMUTE_MASK: c_uint = BIT(0);
const RTQ9128_DIE_CHECK_MASK: c_uint = GENMASK(4, 0);
const RTQ9128_VENDOR_ID_MASK: c_uint = GENMASK(19, 8);
const RTQ9128_MODEL_ID_MASK: c_uint = GENMASK(7, 4);

const RTQ9128_SOFT_RESET_VAL: c_uint = 0x80;
const RTQ9128_VENDOR_ID_VAL: c_uint = 0x470;
const RTQ9128_ALLCH_HIZ_VAL: c_uint = 0x55;
const RTQ9128_ALLCH_ULQM_VAL: c_uint = 0xFF;
const RTQ9128_TKA470B_VAL: c_uint = 0;
const RTQ9128_RTQ9128DH_VAL: c_uint = 0x0F;
const RTQ9128_RTQ9128DL_VAL: c_uint = 0x10;
const RTQ9154_MODEL_ID: c_uint = 0x08;

const RTQ9154_AUTOULQM_VAL: c_uint = 0x82;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum rtq9128_chip_model {
    CHIP_MODEL_RTQ9128 = 0,
    CHIP_MODEL_RTQ9154,
    CHIP_MODEL_MAX,
}

#[repr(C)]
struct rtq9128_data {
    enable: *mut gpio_desc,
    daifmt: c_uint,
    tdm_slots: c_int,
    tdm_slot_width: c_int,
    tdm_input_data2_select: bool_,
    chip_model: rtq9128_chip_model,
}

#[repr(C)]
struct rtq9128_init_reg {
    reg: c_uint,
    val: c_uint,
}

unsafe extern "C" fn rtq9128_get_reg_size(reg: c_uint) -> c_int {
    match reg {
        0x5C..=0x6F | 0x98..=0x9F | 0xC0..=0xC3 | 0xC8..=0xCF | 0xDF..=0xE5 | 0xF9 => 4,
        0x40..=0x4F => 3,
        0x30..=0x35 | 0x8C..=0x97 | 0xC4..=0xC7 | 0xD7..=0xDA => 2,
        _ => 1,
    }
}

unsafe extern "C" fn rtq9128_i2c_write(
    context: *mut c_void,
    data: *const c_void,
    count: size_t,
) -> c_int {
    let dev = context as *mut device;
    let i2c = to_i2c_client(dev);
    let reg = *(data as *const u8);
    let rg_size: c_int;

    if count != 5 {
        dev_err(dev, c"Invalid write for data length (%d)\n".as_ptr(), count as c_int);
        return -EINVAL;
    }

    rg_size = rtq9128_get_reg_size(reg as c_uint);
    i2c_smbus_write_i2c_block_data(
        i2c,
        reg,
        rg_size as u8,
        (data as *const u8).add(count - rg_size as size_t),
    )
}

unsafe extern "C" fn rtq9128_i2c_read(
    context: *mut c_void,
    reg_buf: *const c_void,
    reg_size: size_t,
    val_buf: *mut c_void,
    val_size: size_t,
) -> c_int {
    let dev = context as *mut device;
    let i2c = to_i2c_client(dev);
    let reg = *(reg_buf as *const u8);
    let mut data_tmp: [u8; 4] = [0; 4];
    let rg_size: c_int;
    let ret: c_int;

    if reg_size != 1 || val_size != 4 {
        dev_err(
            dev,
            c"Invalid read for reg_size (%d) or val_size (%d)\n".as_ptr(),
            reg_size as c_int,
            val_size as c_int,
        );
        return -EINVAL;
    }

    rg_size = rtq9128_get_reg_size(reg as c_uint);
    ret = i2c_smbus_read_i2c_block_data(i2c, reg, rg_size as u8, data_tmp.as_mut_ptr());
    if ret < 0 {
        return ret;
    } else if ret != rg_size {
        return -EIO;
    }

    memset(val_buf, 0, val_size - rg_size as size_t);
    memcpy(
        (val_buf as *mut u8).add(val_size - rg_size as size_t) as *mut c_void,
        data_tmp.as_ptr() as *const c_void,
        rg_size as size_t,
    );

    0
}

static rtq9128_regmap_bus: regmap_bus = regmap_bus {
    write: Some(rtq9128_i2c_write),
    read: Some(rtq9128_i2c_read),
    max_raw_read: 4,
    max_raw_write: 4,
};

unsafe extern "C" fn rtq9128_is_readable_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        0x00..=0x2B
        | 0x30..=0x35
        | 0x40..=0x56
        | 0x5C..=0x76
        | 0x80..=0xAD
        | 0xB0..=0xBA
        | 0xC0..=0xE5
        | 0xF0..=0xFB => true,
        _ => false,
    }
}

unsafe extern "C" fn rtq9128_is_writeable_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        0x00..=0x1F
        | 0x21..=0x2B
        | 0x30..=0x35
        | 0x40..=0x56
        | 0x5C..=0x76
        | 0x80..=0x8B
        | 0xA0..=0xAD
        | 0xB0..=0xBA
        | 0xC0
        | 0xD0..=0xDE
        | 0xE0..=0xE5
        | 0xF0..=0xF3
        | 0xF6..=0xF8
        | 0xFA..=0xFB => true,
        _ => false,
    }
}

unsafe extern "C" fn rtq9128_is_volatile_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        0x0F..=0x17
        | 0x20
        | 0x53
        | 0x55
        | 0x5C..=0x6F
        | 0x8C..=0x9F
        | 0xC0..=0xCF
        | 0xDF
        | 0xF0..=0xF1
        | 0xF4..=0xF5 => true,
        _ => false,
    }
}

static rtq9128_regmap_config: regmap_config = regmap_config {
    name: c"rtq9128".as_ptr(),
    reg_bits: 8,
    val_bits: 32,
    val_format_endian: REGMAP_ENDIAN_BIG,
    cache_type: REGCACHE_MAPLE,
    readable_reg: Some(rtq9128_is_readable_reg),
    writeable_reg: Some(rtq9128_is_writeable_reg),
    volatile_reg: Some(rtq9128_is_volatile_reg),
    num_reg_defaults_raw: RTQ9128_REG_VENDOR_ID + 1,
};

// static const DECLARE_TLV_DB_SCALE(dig_tlv, -10375, 25, 0);
static dig_tlv: [c_uint; 4] = [0, (-10375i32) as c_uint, 25, 0];

// static const DECLARE_TLV_DB_RANGE(spkgain_tlv, ...);
static spkgain_tlv: [c_uint; 8] = [0, 3, (-600i32) as c_uint, 600, 0, 4, 5, 1500];

static source_select_text: [*const c_char; 4] =
    [c"CH1".as_ptr(), c"CH2".as_ptr(), c"CH3".as_ptr(), c"CH4".as_ptr()];
static pwmfreq_select_text: [*const c_char; 5] =
    [c"8fs".as_ptr(), c"10fs".as_ptr(), c"40fs".as_ptr(), c"44fs".as_ptr(), c"48fs".as_ptr()];
static phase_select_text: [*const c_char; 8] = [
    c"0 degree".as_ptr(),
    c"45 degree".as_ptr(),
    c"90 degree".as_ptr(),
    c"135 degree".as_ptr(),
    c"180 degree".as_ptr(),
    c"225 degree".as_ptr(),
    c"270 degree".as_ptr(),
    c"315 degree".as_ptr(),
];
static dvdduv_select_text: [*const c_char; 4] =
    [c"1P4V".as_ptr(), c"1P5V".as_ptr(), c"2P1V".as_ptr(), c"2P3V".as_ptr()];

// The following ASoC macro-built control and enum objects preserve the original declarations.
unsafe extern "C" {
    static rtq9128_ch1_si_enum: soc_enum;
    static rtq9128_ch2_si_enum: soc_enum;
    static rtq9128_ch3_si_enum: soc_enum;
    static rtq9128_ch4_si_enum: soc_enum;
    static rtq9128_pwm_freq_enum: soc_enum;
    static rtq9128_out2_phase_enum: soc_enum;
    static rtq9128_out3_phase_enum: soc_enum;
    static rtq9128_out4_phase_enum: soc_enum;
    static rtq9154_ch1_si_enum: soc_enum;
    static rtq9154_ch2_si_enum: soc_enum;
    static rtq9154_ch3_si_enum: soc_enum;
    static rtq9154_ch4_si_enum: soc_enum;
    static rtq9154_out1_phase_enum: soc_enum;
    static rtq9154_out2_phase_enum: soc_enum;
    static rtq9154_out3_phase_enum: soc_enum;
    static rtq9128_dvdduv_select_enum: soc_enum;
}

/*
 * In general usage, DVDD could be 1P8V, 3P0V or 3P3V.
 * This DVDD undervoltage protection is to prevent from the abnormal power
 * lose case while the amplifier is operating. Due to the different DVDD
 * application, treat this threshold as a user choosable option.
 */
// SOC_SINGLE_TLV/SOC_SINGLE/SOC_ENUM initializers are external ASoC macro data in Rust form.
static rtq9128_snd_ctrls: [snd_kcontrol_new; 19] = unsafe { core::mem::zeroed() };
static rtq9154_snd_ctrls: [snd_kcontrol_new; 19] = unsafe { core::mem::zeroed() };

unsafe extern "C" fn rtq9128_dac_power_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let comp = snd_soc_dapm_to_component((*w).dapm);
    let data = snd_soc_component_get_drvdata(comp) as *mut rtq9128_data;
    let mut shift: c_uint;
    let mask: c_uint;
    let ret: c_int;

    dev_dbg((*comp).dev, c"%s: %s event %d\n".as_ptr(), c"rtq9128_dac_power_event".as_ptr(), (*w).name, event);

    if snd_soc_dapm_widget_name_cmp(w, c"DAC1".as_ptr()) == 0 {
        shift = 6;
    } else if snd_soc_dapm_widget_name_cmp(w, c"DAC2".as_ptr()) == 0 {
        shift = 4;
    } else if snd_soc_dapm_widget_name_cmp(w, c"DAC3".as_ptr()) == 0 {
        shift = 2;
    } else {
        shift = 0;
    }

    /* Compared to RTQ9128, RTQ9154 use the reverse order for DACx bitfield location */
    if (*data).chip_model == rtq9128_chip_model::CHIP_MODEL_RTQ9154 {
        shift = 6 - shift;
    }

    mask = RTQ9128_CHSTAT_VAL_MASK << shift;

    /* Turn channel state to Normal or HiZ */
    ret = snd_soc_component_write_field(
        comp,
        RTQ9128_REG_STATE_CTRL,
        mask,
        (event != SND_SOC_DAPM_POST_PMU) as c_uint,
    );
    if ret < 0 {
        return ret;
    }

    /*
     * For each channel turns on, HW will trigger DC load detect and DC
     * offset calibration, the time is needed for all the actions done.
     */
    if event == SND_SOC_DAPM_POST_PMU {
        msleep(25);
    }

    0
}

static rtq9128_dapm_widgets: [snd_soc_dapm_widget_desc; 8] = unsafe { core::mem::zeroed() };

static rtq9128_dapm_routes: [snd_soc_dapm_route; 12] = [
    snd_soc_dapm_route { sink: c"DAC1".as_ptr(), control: ptr::null(), source: c"Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC2".as_ptr(), control: ptr::null(), source: c"Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC3".as_ptr(), control: ptr::null(), source: c"Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC4".as_ptr(), control: ptr::null(), source: c"Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT1".as_ptr(), control: ptr::null(), source: c"DAC1".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT2".as_ptr(), control: ptr::null(), source: c"DAC2".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT3".as_ptr(), control: ptr::null(), source: c"DAC3".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT4".as_ptr(), control: ptr::null(), source: c"DAC4".as_ptr() },
    snd_soc_dapm_route { sink: c"Capture".as_ptr(), control: ptr::null(), source: c"DAC1".as_ptr() },
    snd_soc_dapm_route { sink: c"Capture".as_ptr(), control: ptr::null(), source: c"DAC2".as_ptr() },
    snd_soc_dapm_route { sink: c"Capture".as_ptr(), control: ptr::null(), source: c"DAC3".as_ptr() },
    snd_soc_dapm_route { sink: c"Capture".as_ptr(), control: ptr::null(), source: c"DAC4".as_ptr() },
];

static rtq9128_tka470b_tables: [rtq9128_init_reg; 13] = [
    rtq9128_init_reg { reg: 0xA0, val: 0xEF },
    rtq9128_init_reg { reg: 0x0D, val: 0x00 },
    rtq9128_init_reg { reg: 0x03, val: 0x45 },
    rtq9128_init_reg { reg: 0x05, val: 0x31 },
    rtq9128_init_reg { reg: 0x06, val: 0x23 },
    rtq9128_init_reg { reg: 0x70, val: 0x11 },
    rtq9128_init_reg { reg: 0x75, val: 0x1F },
    rtq9128_init_reg { reg: 0xB6, val: 0x03 },
    rtq9128_init_reg { reg: 0xB9, val: 0x03 },
    rtq9128_init_reg { reg: 0xB8, val: 0x03 },
    rtq9128_init_reg { reg: 0xC1, val: 0xFF },
    rtq9128_init_reg { reg: 0xF8, val: 0x72 },
    rtq9128_init_reg { reg: 0x30, val: 0x180 },
];

static rtq9128_dh_tables: [rtq9128_init_reg; 8] = [
    rtq9128_init_reg { reg: 0x0F, val: 0x00 },
    rtq9128_init_reg { reg: 0x03, val: 0x4D },
    rtq9128_init_reg { reg: 0xB2, val: 0xFF },
    rtq9128_init_reg { reg: 0xB3, val: 0xFF },
    rtq9128_init_reg { reg: 0x30, val: 0x180 },
    rtq9128_init_reg { reg: 0x8A, val: 0x55 },
    rtq9128_init_reg { reg: 0x72, val: 0x00 },
    rtq9128_init_reg { reg: 0xB1, val: 0xE3 },
];

static rtq9128_dl_tables: [rtq9128_init_reg; 6] = [
    rtq9128_init_reg { reg: 0x0F, val: 0x00 },
    rtq9128_init_reg { reg: 0x03, val: 0x4D },
    rtq9128_init_reg { reg: 0x30, val: 0x180 },
    rtq9128_init_reg { reg: 0x8A, val: 0x55 },
    rtq9128_init_reg { reg: 0x72, val: 0x00 },
    rtq9128_init_reg { reg: 0xB1, val: 0xE3 },
];

unsafe extern "C" fn rtq9128_component_probe(comp: *mut snd_soc_component) -> c_int {
    let data = snd_soc_component_get_drvdata(comp) as *mut rtq9128_data;
    let table: *const rtq9128_init_reg;
    let table_size: size_t;
    let mut val: c_uint;
    let mut i: size_t;
    let mut ret: c_int;

    ret = pm_runtime_resume_and_get((*comp).dev);
    if ret < 0 {
        dev_err((*comp).dev, c"Failed to resume device (%d)\n".as_ptr(), ret);
        return ret;
    }

    val = snd_soc_component_read(comp, RTQ9128_REG_EFUSE_DATA);

    match FIELD_GET(RTQ9128_DIE_CHECK_MASK, val) {
        RTQ9128_TKA470B_VAL => {
            table = rtq9128_tka470b_tables.as_ptr();
            table_size = rtq9128_tka470b_tables.len();
        }
        RTQ9128_RTQ9128DH_VAL => {
            table = rtq9128_dh_tables.as_ptr();
            table_size = rtq9128_dh_tables.len();
        }
        _ => {
            table = rtq9128_dl_tables.as_ptr();
            table_size = rtq9128_dl_tables.len();
        }
    }

    i = 0;
    while i < table_size {
        let curr = table.add(i);
        ret = snd_soc_component_write(comp, (*curr).reg, (*curr).val);
        if ret < 0 {
            return ret;
        }
        i += 1;
    }

    if (*data).chip_model == rtq9128_chip_model::CHIP_MODEL_RTQ9154 {
        /* Enable RTQ9154 Specific AUTO ULQM feature */
        ret = snd_soc_component_write(comp, RTQ9154_REG_AUTOULQM, RTQ9154_AUTOULQM_VAL);
        if ret < 0 {
            return ret;
        }
    }

    pm_runtime_mark_last_busy((*comp).dev);
    pm_runtime_put((*comp).dev);

    0
}

static rtq9128_comp_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(rtq9128_component_probe),
    controls: rtq9128_snd_ctrls.as_ptr(),
    num_controls: ARRAY_SIZE(&rtq9128_snd_ctrls),
    dapm_widgets: rtq9128_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE(&rtq9128_dapm_widgets),
    dapm_routes: rtq9128_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE(&rtq9128_dapm_routes),
    use_pmdown_time: 1,
    endianness: 1,
};

static rtq9154_comp_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(rtq9128_component_probe),
    controls: rtq9154_snd_ctrls.as_ptr(),
    num_controls: ARRAY_SIZE(&rtq9154_snd_ctrls),
    dapm_widgets: rtq9128_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE(&rtq9128_dapm_widgets),
    dapm_routes: rtq9128_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE(&rtq9128_dapm_routes),
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn rtq9128_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let data = snd_soc_dai_get_drvdata(dai) as *mut rtq9128_data;
    let dev = (*dai).dev;

    dev_dbg(dev, c"%s: fmt 0x%8x\n".as_ptr(), c"rtq9128_dai_set_fmt".as_ptr(), fmt);

    /* Only support bitclock & framesync as consumer */
    if (fmt & SND_SOC_DAIFMT_MASTER_MASK) != SND_SOC_DAIFMT_BC_FC {
        dev_err(dev, c"Only support BCK and LRCK as consumer\n".as_ptr());
        return -EINVAL;
    }

    /* Store here and will be used in runtime hw_params for DAI format setting */
    (*data).daifmt = fmt;

    0
}

unsafe extern "C" fn rtq9128_dai_set_tdm_slot(
    dai: *mut snd_soc_dai,
    tx_mask: c_uint,
    rx_mask: c_uint,
    slots: c_int,
    slot_width: c_int,
) -> c_int {
    let data = snd_soc_dai_get_drvdata(dai) as *mut rtq9128_data;
    let comp = (*dai).component;
    let dev = (*dai).dev;
    let mut mask: c_uint;
    let mut start_loc: c_uint;
    let srcin_select: c_uint;
    let mut i: c_int;
    let frame_length: c_int;
    let mut ret: c_int;

    dev_dbg(
        dev,
        c"%s: slot %d slot_width %d, tx/rx mask 0x%x 0x%x\n".as_ptr(),
        c"rtq9128_dai_set_tdm_slot".as_ptr(),
        slots,
        slot_width,
        tx_mask,
        rx_mask,
    );

    if slots <= 0 || slot_width <= 0 || slot_width % 8 != 0 {
        dev_err(dev, c"Invalid slot numbers (%d) or width (%d)\n".as_ptr(), slots, slot_width);
        return -EINVAL;
    }

    /* HW supported maximum frame length 512 */
    frame_length = slots * slot_width;
    if frame_length > 512 {
        dev_err(dev, c"frame length exceed the maximum (%d)\n".as_ptr(), frame_length);
        return -EINVAL;
    }

    if rx_mask == 0
        || hweight_long(tx_mask) > slots as c_uint
        || hweight_long(rx_mask) > slots as c_uint
        || fls(tx_mask) > slots
        || fls(rx_mask) > slots
    {
        dev_err(dev, c"Invalid tx/rx mask (0x%x/0x%x)\n".as_ptr(), tx_mask, rx_mask);
        return -EINVAL;
    }

    mask = tx_mask;
    i = 0;
    while i < 4 && mask != 0 {
        start_loc = ((ffs(mask) - 1) * slot_width / 8) as c_uint;
        mask &= !BIT((ffs(mask) - 1) as c_uint);

        ret = snd_soc_component_write(comp, RTQ9128_REG_TDM_TX_CH1 + i as c_uint, start_loc);
        if ret < 0 {
            dev_err(dev, c"Failed to assign tx_loc %d (%d)\n".as_ptr(), i, ret);
            return ret;
        }
        i += 1;
    }

    mask = rx_mask;
    i = 0;
    while i < 4 && mask != 0 {
        start_loc = ((ffs(mask) - 1) * slot_width / 8) as c_uint;
        mask &= !BIT((ffs(mask) - 1) as c_uint);

        ret = snd_soc_component_write(comp, RTQ9128_REG_TDM_RX_CH1 + i as c_uint, start_loc);
        if ret < 0 {
            dev_err(dev, c"Failed to assign rx_loc %d (%d)\n".as_ptr(), i, ret);
            return ret;
        }
        i += 1;
    }

    srcin_select = if (*data).tdm_input_data2_select {
        RTQ9128_TDMSRCIN_MASK
    } else {
        0
    };
    ret = snd_soc_component_update_bits(
        comp,
        RTQ9128_REG_SDO_SEL,
        RTQ9128_TDMSRCIN_MASK,
        srcin_select,
    );
    if ret < 0 {
        dev_err(dev, c"Failed to configure TDM source input select\n".as_ptr());
        return ret;
    }

    (*data).tdm_slots = slots;
    (*data).tdm_slot_width = slot_width;

    0
}

unsafe extern "C" fn rtq9128_dai_hw_params(
    _stream: *mut snd_pcm_substream,
    param: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let data = snd_soc_dai_get_drvdata(dai) as *mut rtq9128_data;
    let mut width: c_uint;
    let mut slot_width: c_uint;
    let bitrate: c_uint;
    let audbit: c_uint;
    let dolen: c_uint;
    let comp = (*dai).component;
    let dev = (*dai).dev;
    let fmtval: c_uint;
    let audfmt: c_uint;
    let mut ret: c_int;

    dev_dbg(dev, c"%s: width %d\n".as_ptr(), c"rtq9128_dai_hw_params".as_ptr(), params_width(param));

    fmtval = FIELD_GET(SND_SOC_DAIFMT_FORMAT_MASK, (*data).daifmt);
    if (*data).tdm_slots != 0 && fmtval != SND_SOC_DAIFMT_DSP_A && fmtval != SND_SOC_DAIFMT_DSP_B {
        dev_err(dev, c"TDM is used, format only support DSP_A or DSP_B\n".as_ptr());
        return -EINVAL;
    }

    audfmt = match fmtval {
        SND_SOC_DAIFMT_I2S => 8,
        SND_SOC_DAIFMT_LEFT_J => 9,
        SND_SOC_DAIFMT_RIGHT_J => 10,
        SND_SOC_DAIFMT_DSP_A => {
            if (*data).tdm_slots != 0 { 12 } else { 11 }
        }
        SND_SOC_DAIFMT_DSP_B => {
            if (*data).tdm_slots != 0 { 4 } else { 3 }
        }
        _ => {
            dev_err(dev, c"Unsupported format 0x%8x\n".as_ptr(), fmtval);
            return -EINVAL;
        }
    };

    width = params_width(param);
    audbit = match width {
        16 => 0,
        18 => 1,
        20 => 2,
        24 | 32 => 3,
        _ => {
            dev_err(dev, c"Unsupported width (%d)\n".as_ptr(), width);
            return -EINVAL;
        }
    };

    slot_width = params_physical_width(param);

    if (*data).tdm_slots != 0 {
        if slot_width > (*data).tdm_slot_width as c_uint {
            dev_err(dev, c"slot width is larger than TDM slot width\n".as_ptr());
            return -EINVAL;
        }

        /* Check BCK not exceed the maximum supported rate 24.576MHz */
        bitrate = ((*data).tdm_slots as c_uint)
            .wrapping_mul((*data).tdm_slot_width as c_uint)
            .wrapping_mul(params_rate(param));
        if bitrate > 24576000 {
            dev_err(dev, c"bitrate exceed the maximum (%d)\n".as_ptr(), bitrate);
            return -EINVAL;
        }

        /* If TDM is used, configure slot width as TDM slot witdh */
        slot_width = (*data).tdm_slot_width as c_uint;
    }

    dolen = match slot_width {
        16 => 0,
        24 => 1,
        32 => 2,
        _ => {
            dev_err(dev, c"Unsupported slot width (%d)\n".as_ptr(), slot_width);
            return -EINVAL;
        }
    };

    ret = snd_soc_component_write_field(comp, RTQ9128_REG_I2S_OPT, RTQ9128_AUDFMT_MASK, audfmt);
    if ret < 0 {
        return ret;
    }

    ret = snd_soc_component_write_field(comp, RTQ9128_REG_I2S_OPT, RTQ9128_AUDBIT_MASK, audbit);
    if ret < 0 {
        return ret;
    }

    ret = snd_soc_component_write_field(comp, RTQ9128_REG_SDO_SEL, RTQ9128_DOLEN_MASK, dolen);
    if ret < 0 { ret } else { 0 }
}

unsafe extern "C" fn rtq9128_dai_mute_stream(
    dai: *mut snd_soc_dai,
    mute: c_int,
    stream: c_int,
) -> c_int {
    let comp = (*dai).component;
    let dev = (*dai).dev;
    let ret: c_int;

    dev_dbg(
        dev,
        c"%s: mute (%d), stream (%d)\n".as_ptr(),
        c"rtq9128_dai_mute_stream".as_ptr(),
        mute,
        stream,
    );

    ret = snd_soc_component_write_field(
        comp,
        RTQ9128_REG_DSP_EN,
        RTQ9128_MSMUTE_MASK,
        if mute != 0 { 1 } else { 0 },
    );
    if ret < 0 { ret } else { 0 }
}

static rtq9128_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(rtq9128_dai_set_fmt),
    set_tdm_slot: Some(rtq9128_dai_set_tdm_slot),
    hw_params: Some(rtq9128_dai_hw_params),
    mute_stream: Some(rtq9128_dai_mute_stream),
    no_capture_mute: 1,
};

const RTQ9128_FMTS_MASK: u64 = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S18_3LE
    | SNDRV_PCM_FMTBIT_S20_LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE;

static mut rtq9128_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"rtq9128-aif".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        rates: SNDRV_PCM_RATE_8000_192000,
        formats: RTQ9128_FMTS_MASK,
        channels_min: 1,
        channels_max: 4,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        rates: SNDRV_PCM_RATE_8000_192000,
        formats: RTQ9128_FMTS_MASK,
        channels_min: 1,
        channels_max: 4,
    },
    ops: &rtq9128_dai_ops,
    symmetric_rate: 1,
    symmetric_sample_bits: 1,
};

unsafe extern "C" fn rtq9128_probe(i2c: *mut i2c_client) -> c_int {
    let dev = &mut (*i2c).dev as *mut device;
    let data: *mut rtq9128_data;
    let regmap: *mut regmap;
    let mut veninfo: c_uint = 0;
    let venid: c_uint;
    let chip_model: c_uint;
    let comp_drv: *const snd_soc_component_driver;
    let mut ret: c_int;

    data = devm_kzalloc(dev, size_of::<rtq9128_data>(), GFP_KERNEL) as *mut rtq9128_data;
    if data.is_null() {
        return -ENOMEM;
    }

    (*data).enable = devm_gpiod_get_optional(dev, c"enable".as_ptr(), GPIOD_OUT_HIGH);
    if IS_ERR((*data).enable as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*data).enable as *const c_void),
            c"Failed to get 'enable' gpio\n".as_ptr(),
        );
    } else if !(*data).enable.is_null() {
        usleep_range(10000, 11000);
    }

    (*data).tdm_input_data2_select =
        device_property_read_bool(dev, c"richtek,tdm-input-data2-select".as_ptr());

    i2c_set_clientdata(i2c, data as *mut c_void);

    /*
     * Due to the bad design to combine SOFT_RESET bit with other function,
     * directly use generic i2c API to trigger SOFT_RESET.
     */
    ret = i2c_smbus_write_byte_data(i2c, RTQ9128_REG_MISC as u8, RTQ9128_SOFT_RESET_VAL as u8);
    if ret != 0 {
        return dev_err_probe(dev, ret, c"Failed to trigger software reset\n".as_ptr());
    }

    /* After trigger soft reset, have to wait 10ms for digital reset done */
    usleep_range(10000, 11000);

    regmap = devm_regmap_init(
        dev,
        &rtq9128_regmap_bus,
        dev as *mut c_void,
        &rtq9128_regmap_config,
    );
    if IS_ERR(regmap as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR(regmap as *const c_void),
            c"Failed to init regmap\n".as_ptr(),
        );
    }

    ret = regmap_read(regmap, RTQ9128_REG_VENDOR_ID, &mut veninfo);
    if ret != 0 {
        return dev_err_probe(dev, ret, c"Failed to get vendor id\n".as_ptr());
    }

    venid = FIELD_GET(RTQ9128_VENDOR_ID_MASK, veninfo);
    if venid != RTQ9128_VENDOR_ID_VAL {
        return dev_err_probe(dev, -ENODEV, c"Vendor ID not match (0x%x)\n".as_ptr(), venid);
    }

    chip_model = FIELD_GET(RTQ9128_MODEL_ID_MASK, veninfo);
    match chip_model {
        RTQ9154_MODEL_ID => {
            (*data).chip_model = rtq9128_chip_model::CHIP_MODEL_RTQ9154;
            comp_drv = &rtq9154_comp_driver;
        }
        _ => {
            (*data).chip_model = rtq9128_chip_model::CHIP_MODEL_RTQ9128;
            comp_drv = &rtq9128_comp_driver;
        }
    }

    pm_runtime_set_active(dev);
    pm_runtime_mark_last_busy(dev);
    ret = devm_pm_runtime_enable(dev);
    if ret != 0 {
        return dev_err_probe(dev, ret, c"Failed to enable pm runtime\n".as_ptr());
    }

    devm_snd_soc_register_component(dev, comp_drv, &raw mut rtq9128_dai, 1)
}

unsafe extern "C" fn rtq9128_pm_runtime_suspend(dev: *mut device) -> c_int {
    let data = dev_get_drvdata(dev) as *mut rtq9128_data;
    let regmap = dev_get_regmap(dev, ptr::null());

    /* If 'enable' gpio not specified, change all channels to ultra low quiescent */
    if (*data).enable.is_null() {
        return regmap_write(regmap, RTQ9128_REG_STATE_CTRL, RTQ9128_ALLCH_ULQM_VAL);
    }

    gpiod_set_value_cansleep((*data).enable, 0);

    regcache_cache_only(regmap, true);
    regcache_mark_dirty(regmap);

    0
}

unsafe extern "C" fn rtq9128_pm_runtime_resume(dev: *mut device) -> c_int {
    let data = dev_get_drvdata(dev) as *mut rtq9128_data;
    let regmap = dev_get_regmap(dev, ptr::null());

    /* If 'enable' gpio not specified, change all channels to default Hi-Z */
    if (*data).enable.is_null() {
        return regmap_write(regmap, RTQ9128_REG_STATE_CTRL, RTQ9128_ALLCH_HIZ_VAL);
    }

    gpiod_set_value_cansleep((*data).enable, 1);

    /* Wait digital block to be ready */
    usleep_range(10000, 11000);

    regcache_cache_only(regmap, false);
    regcache_sync(regmap)
}

// static const struct dev_pm_ops rtq9128_pm_ops = { RUNTIME_PM_OPS(...) };
static rtq9128_pm_ops: dev_pm_ops = dev_pm_ops { _private: [] };

static rtq9128_device_table: [of_device_id; 2] = [
    of_device_id { compatible: c"richtek,rtq9128".as_ptr() },
    of_device_id { compatible: ptr::null() },
];
// MODULE_DEVICE_TABLE(of, rtq9128_device_table);

static mut rtq9128_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"rtq9128".as_ptr(),
        of_match_table: rtq9128_device_table.as_ptr(),
        pm: &rtq9128_pm_ops,
    },
    probe: Some(rtq9128_probe),
};
// module_i2c_driver(rtq9128_driver);

// MODULE_AUTHOR("ChiYuan Huang <cy_huang@richtek.com>");
// MODULE_DESCRIPTION("RTQ9128 4CH Audio Amplifier Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
