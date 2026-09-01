// SPDX-License-Identifier: GPL-2.0
//
// ALSA SoC Texas Instruments PCM6240 Family Audio ADC/DAC Device
//
// Copyright (C) 2022 - 2024 Texas Instruments Incorporated
// https://www.ti.com
//
// The PCM6240 driver implements a flexible and configurable
// algo coefficient setting for one, two, or even multiple
// PCM6240 Family chips.
//
// Author: Shenghao Ding <shenghao-ding@ti.com>
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

// C includes removed. This file depends on the Rust forms of the Linux kernel,
// ALSA SoC, regmap, firmware, GPIO, I2C, and local "pcm6240.h" declarations.

type u8_ = u8;
type u16_ = u16;
type u32_ = u32;
type u64_ = u64;

#[repr(C)]
pub struct i2c_device_id {
    pub name: *const c_char,
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct pcmdevice_mixer_control {
    pub shift: c_uint,
    pub reg: c_uint,
    pub max: c_uint,
    pub invert: c_uint,
    pub dev_no: c_uint,
}

#[repr(C)]
pub struct pcmdev_ctrl_info {
    pub gain: *const c_uint,
    pub pcmdev_ctrl: *const pcmdevice_mixer_control,
    pub ctrl_array_size: c_int,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub pcmdev_ctrl_name_id: c_int,
}

#[repr(C)]
pub struct device { pub of_node: *mut device_node }
#[repr(C)]
pub struct device_node { _priv: [u8; 0] }
#[repr(C)]
pub struct regmap { _priv: [u8; 0] }
#[repr(C)]
pub struct gpio_desc { _priv: [u8; 0] }
#[repr(C)]
pub struct mutex { _priv: [u8; 0] }
#[repr(C)]
pub struct i2c_adapter { pub nr: c_int }
#[repr(C)]
pub struct i2c_client { pub addr: u16_, pub dev: device, pub adapter: *mut i2c_adapter }
#[repr(C)]
pub struct firmware { pub size: usize, pub data: *const u8_ }

#[repr(C)]
pub struct snd_soc_component {
    pub name_prefix: *const c_char,
}
#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}
#[repr(C)]
pub struct snd_pcm_substream { _priv: [u8; 0] }
#[repr(C)]
pub struct snd_pcm_hw_params { _priv: [u8; 0] }
#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
}
#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}
#[repr(C)]
pub union snd_ctl_elem_info_value { pub integer: snd_ctl_elem_info_integer }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_integer { pub min: i64, pub max: i64 }
#[repr(C)]
pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_value }
#[repr(C)]
pub union snd_ctl_elem_value_value { pub integer: snd_ctl_elem_value_integer }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer { pub value: [i64; 128] }

#[repr(C)]
pub union snd_kcontrol_new_tlv { pub p: *const c_uint }
#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *mut c_char,
    pub access: c_uint,
    pub tlv: snd_kcontrol_new_tlv,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_soc_dapm_widget { _priv: [u8; 0] }
#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
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
pub struct snd_soc_dai_ops {
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
}
#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64_,
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub capture: snd_soc_pcm_stream,
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
}
#[repr(C)]
pub struct regmap_range_cfg {
    pub range_min: c_uint,
    pub range_max: c_uint,
    pub selector_reg: c_uint,
    pub selector_mask: c_uint,
    pub selector_shift: c_uint,
    pub window_start: c_uint,
    pub window_len: c_uint,
}
#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub cache_type: c_uint,
    pub ranges: *const regmap_range_cfg,
    pub num_ranges: c_uint,
    pub max_register: c_uint,
}
#[repr(C)]
pub struct of_device_id { pub compatible: *const c_char }
#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}
#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
    pub id_table: *const i2c_device_id,
}

#[repr(C)]
pub struct pcmdevice_regbin_hdr {
    pub img_sz: c_uint,
    pub checksum: c_uint,
    pub binary_version_num: c_uint,
    pub drv_fw_version: c_uint,
    pub plat_type: u8_,
    pub dev_family: u8_,
    pub reserve: u8_,
    pub ndev: u8_,
    pub devs: [u8_; PCMDEVICE_MAX_REGBIN_DEVICES as usize],
    pub nconfig: c_uint,
    pub config_size: [c_uint; PCMDEVICE_CONFIG_SUM as usize],
}
#[repr(C)]
pub struct pcmdevice_block_data {
    pub dev_idx: u8_,
    pub block_type: u8_,
    pub yram_checksum: u16_,
    pub block_size: c_uint,
    pub n_subblks: c_uint,
    pub regdata: *mut u8_,
}
#[repr(C)]
pub struct pcmdevice_config_info {
    pub nblocks: c_uint,
    pub real_nblocks: c_uint,
    pub active_dev: c_uint,
    pub cfg_name: [c_char; 64],
    pub blk_data: *mut *mut pcmdevice_block_data,
}
#[repr(C)]
pub struct pcmdevice_regbin {
    pub fw_hdr: pcmdevice_regbin_hdr,
    pub cfg_info: *mut *mut pcmdevice_config_info,
    pub ncfgs: c_int,
}
#[repr(C)]
pub struct pcmdevice_priv {
    pub client: *mut i2c_client,
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub component: *mut snd_soc_component,
    pub codec_lock: mutex,
    pub addr: [u16_; PCMDEVICE_MAX_I2C_DEVICES as usize],
    pub ndev: c_int,
    pub chip_id: c_uint,
    pub dev_name: [c_char; 32],
    pub upper_dev_name: [c_char; 32],
    pub bin_name: [c_char; PCMDEVICE_BIN_FILENAME_LEN as usize],
    pub regbin: pcmdevice_regbin,
    pub cur_conf: c_int,
    pub fw_state: c_int,
    pub irq: c_int,
    pub hw_rst: *mut gpio_desc,
}

extern "C" {
    static ADC3120: c_uint; static ADC5120: c_uint; static ADC6120: c_uint; static DIX4192: c_uint;
    static PCM1690: c_uint; static PCM3120: c_uint; static PCM3140: c_uint; static PCM5120: c_uint;
    static PCM5140: c_uint; static PCM6120: c_uint; static PCM6140: c_uint; static PCM6240: c_uint;
    static PCM6260: c_uint; static PCM9211: c_uint; static PCMD3140: c_uint; static PCMD3180: c_uint;
    static PCMD512X: c_uint; static TAA5212: c_uint; static TAA5412: c_uint; static TAD5212: c_uint;
    static TAD5412: c_uint; static MAX_DEVICE: c_uint;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut pcmdevice_priv;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut pcmdevice_priv;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_bulk_write(map: *mut regmap, reg: c_uint, data: *mut u8_, len: c_uint) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn kmemdup(src: *const c_void, len: usize, flags: c_uint) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kmemdup(dev: *mut device, src: *const c_void, len: usize, flags: c_uint) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, len: usize) -> *mut c_void;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_add_component_controls(comp: *mut snd_soc_component, controls: *mut snd_kcontrol_new, num: c_uint) -> c_int;
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, dev: *mut device) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_params_to_bclk(params: *mut snd_pcm_hw_params) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn free_irq(irq: c_int, dev_id: *mut c_void);
    fn mutex_init(lock: *mut mutex);
    fn mutex_destroy(lock: *mut mutex);
    fn i2c_get_match_data(i2c: *mut i2c_client) -> *const c_void;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn i2c_get_clientdata(i2c: *mut i2c_client) -> *mut pcmdevice_priv;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, cfg: *const regmap_config) -> *mut regmap;
    fn of_property_read_reg(np: *mut device_node, index: c_int, addr: *mut u64_, size: *mut u64_) -> c_int;
    fn of_irq_get(np: *mut device_node, index: c_int) -> c_int;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn devm_snd_soc_register_component(dev: *mut device, driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char, size: usize) -> isize;
    fn toupper(c: c_int) -> c_int;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 1;
const SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint = 1 << 2;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 3;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const SNDRV_CTL_ELEM_ID_NAME_MAXLEN: usize = 44;
const REGCACHE_MAPLE: c_uint = 7;
const GPIOD_OUT_HIGH: c_uint = 1;
const SND_SOC_NOPM: c_int = -1;
const PCMDEVICE_FW_LOAD_FAILED: c_int = -1;

extern "C" {
    static PCMDEVICE_MAX_I2C_DEVICES: c_uint;
    static PCMDEVICE_MAX_REGBIN_DEVICES: c_uint;
    static PCMDEVICE_CONFIG_SUM: c_uint;
    static PCMDEVICE_BIN_FILENAME_LEN: c_uint;
    static PCMDEVICE_PAGE_SELECT: c_uint;
    static PCMDEVICE_REG_SWRESET: c_uint;
    static PCMDEVICE_REG_SWRESET_RESET: c_uint;
    static PCMDEVICE_MAX_CHANNELS: c_uint;
    static PCMDEVICE_RATES: c_uint;
    static PCMDEVICE_FORMATS: u64_;
}

macro_rules! cstr { ($s:literal) => { concat!($s, "\0").as_ptr() as *const c_char }; }
macro_rules! array_size { ($a:expr) => { $a.len() as c_int }; }
macro_rules! BIT { ($x:expr) => { 1u32 << ($x) }; }
fn fls(x: c_uint) -> c_uint { if x == 0 { 0 } else { c_uint::BITS - x.leading_zeros() } }
fn clamp(v: c_int, lo: c_int, hi: c_int) -> c_int { if v < lo { lo } else if v > hi { hi } else { v } }
unsafe fn get_unaligned_be16(p: *const u8_) -> u16_ { ((*p as u16_) << 8) | (*p.add(1) as u16_) }
unsafe fn get_unaligned_be32(p: *const u8_) -> u32_ {
    ((*p as u32_) << 24) | ((*p.add(1) as u32_) << 16) | ((*p.add(2) as u32_) << 8) | (*p.add(3) as u32_)
}
fn PCMDEVICE_REG(page: u8_, reg: u8_) -> c_uint { ((page as c_uint) * 128) + reg as c_uint }
fn IS_ERR<T>(p: *mut T) -> bool { (p as isize) < 0 && (p as isize) > -4096 }
fn PTR_ERR<T>(p: *mut T) -> c_int { p as isize as c_int }
fn IS_ENABLED_CONFIG_OF() -> bool { true }
fn of_match_ptr(p: *const of_device_id) -> *const of_device_id { p }

pub static pcmdevice_i2c_id: [i2c_device_id; 22] = [
    i2c_device_id { name: cstr!("adc3120"), driver_data: unsafe { ADC3120 as c_ulong } },
    i2c_device_id { name: cstr!("adc5120"), driver_data: unsafe { ADC5120 as c_ulong } },
    i2c_device_id { name: cstr!("adc6120"), driver_data: unsafe { ADC6120 as c_ulong } },
    i2c_device_id { name: cstr!("dix4192"), driver_data: unsafe { DIX4192 as c_ulong } },
    i2c_device_id { name: cstr!("pcm1690"), driver_data: unsafe { PCM1690 as c_ulong } },
    i2c_device_id { name: cstr!("pcm3120"), driver_data: unsafe { PCM3120 as c_ulong } },
    i2c_device_id { name: cstr!("pcm3140"), driver_data: unsafe { PCM3140 as c_ulong } },
    i2c_device_id { name: cstr!("pcm5120"), driver_data: unsafe { PCM5120 as c_ulong } },
    i2c_device_id { name: cstr!("pcm5140"), driver_data: unsafe { PCM5140 as c_ulong } },
    i2c_device_id { name: cstr!("pcm6120"), driver_data: unsafe { PCM6120 as c_ulong } },
    i2c_device_id { name: cstr!("pcm6140"), driver_data: unsafe { PCM6140 as c_ulong } },
    i2c_device_id { name: cstr!("pcm6240"), driver_data: unsafe { PCM6240 as c_ulong } },
    i2c_device_id { name: cstr!("pcm6260"), driver_data: unsafe { PCM6260 as c_ulong } },
    i2c_device_id { name: cstr!("pcm9211"), driver_data: unsafe { PCM9211 as c_ulong } },
    i2c_device_id { name: cstr!("pcmd3140"), driver_data: unsafe { PCMD3140 as c_ulong } },
    i2c_device_id { name: cstr!("pcmd3180"), driver_data: unsafe { PCMD3180 as c_ulong } },
    i2c_device_id { name: cstr!("pcmd512x"), driver_data: unsafe { PCMD512X as c_ulong } },
    i2c_device_id { name: cstr!("taa5212"), driver_data: unsafe { TAA5212 as c_ulong } },
    i2c_device_id { name: cstr!("taa5412"), driver_data: unsafe { TAA5412 as c_ulong } },
    i2c_device_id { name: cstr!("tad5212"), driver_data: unsafe { TAD5212 as c_ulong } },
    i2c_device_id { name: cstr!("tad5412"), driver_data: unsafe { TAD5412 as c_ulong } },
    i2c_device_id { name: ptr::null(), driver_data: 0 },
];
// MODULE_DEVICE_TABLE(i2c, pcmdevice_i2c_id);

static pcmdev_ctrl_name: [*const c_char; 3] = [
    cstr!("%s i2c%d Dev%d Ch%d Ana Volume"),
    cstr!("%s i2c%d Dev%d Ch%d Digi Volume"),
    cstr!("%s i2c%d Dev%d Ch%d Fine Volume"),
];

extern "C" {
    static ADC5120_REG_CH1_ANALOG_GAIN: c_uint; static ADC5120_REG_CH2_ANALOG_GAIN: c_uint;
    static ADC5120_REG_CH1_DIGITAL_GAIN: c_uint; static ADC5120_REG_CH2_DIGITAL_GAIN: c_uint;
    static PCM1690_REG_CH1_DIGITAL_GAIN: c_uint; static PCM1690_REG_CH2_DIGITAL_GAIN: c_uint; static PCM1690_REG_CH3_DIGITAL_GAIN: c_uint; static PCM1690_REG_CH4_DIGITAL_GAIN: c_uint;
    static PCM1690_REG_CH5_DIGITAL_GAIN: c_uint; static PCM1690_REG_CH6_DIGITAL_GAIN: c_uint; static PCM1690_REG_CH7_DIGITAL_GAIN: c_uint; static PCM1690_REG_CH8_DIGITAL_GAIN: c_uint;
    static PCM6240_REG_CH1_ANALOG_GAIN: c_uint; static PCM6240_REG_CH2_ANALOG_GAIN: c_uint; static PCM6240_REG_CH3_ANALOG_GAIN: c_uint; static PCM6240_REG_CH4_ANALOG_GAIN: c_uint;
    static PCM6240_REG_CH1_DIGITAL_GAIN: c_uint; static PCM6240_REG_CH2_DIGITAL_GAIN: c_uint; static PCM6240_REG_CH3_DIGITAL_GAIN: c_uint; static PCM6240_REG_CH4_DIGITAL_GAIN: c_uint;
    static PCM6260_REG_CH1_ANALOG_GAIN: c_uint; static PCM6260_REG_CH2_ANALOG_GAIN: c_uint; static PCM6260_REG_CH3_ANALOG_GAIN: c_uint; static PCM6260_REG_CH4_ANALOG_GAIN: c_uint; static PCM6260_REG_CH5_ANALOG_GAIN: c_uint; static PCM6260_REG_CH6_ANALOG_GAIN: c_uint;
    static PCM6260_REG_CH1_DIGITAL_GAIN: c_uint; static PCM6260_REG_CH2_DIGITAL_GAIN: c_uint; static PCM6260_REG_CH3_DIGITAL_GAIN: c_uint; static PCM6260_REG_CH4_DIGITAL_GAIN: c_uint; static PCM6260_REG_CH5_DIGITAL_GAIN: c_uint; static PCM6260_REG_CH6_DIGITAL_GAIN: c_uint;
    static PCM9211_REG_CH1_DIGITAL_GAIN: c_uint; static PCM9211_REG_CH2_DIGITAL_GAIN: c_uint;
    static PCMD3140_REG_CH1_DIGITAL_GAIN: c_uint; static PCMD3140_REG_CH2_DIGITAL_GAIN: c_uint; static PCMD3140_REG_CH3_DIGITAL_GAIN: c_uint; static PCMD3140_REG_CH4_DIGITAL_GAIN: c_uint;
    static PCMD3140_REG_CH1_FINE_GAIN: c_uint; static PCMD3140_REG_CH2_FINE_GAIN: c_uint; static PCMD3140_REG_CH3_FINE_GAIN: c_uint; static PCMD3140_REG_CH4_FINE_GAIN: c_uint;
    static PCMD3180_REG_CH1_DIGITAL_GAIN: c_uint; static PCMD3180_REG_CH2_DIGITAL_GAIN: c_uint; static PCMD3180_REG_CH3_DIGITAL_GAIN: c_uint; static PCMD3180_REG_CH4_DIGITAL_GAIN: c_uint; static PCMD3180_REG_CH5_DIGITAL_GAIN: c_uint; static PCMD3180_REG_CH6_DIGITAL_GAIN: c_uint; static PCMD3180_REG_CH7_DIGITAL_GAIN: c_uint; static PCMD3180_REG_CH8_DIGITAL_GAIN: c_uint;
    static PCMD3180_REG_CH1_FINE_GAIN: c_uint; static PCMD3180_REG_CH2_FINE_GAIN: c_uint; static PCMD3180_REG_CH3_FINE_GAIN: c_uint; static PCMD3180_REG_CH4_FINE_GAIN: c_uint; static PCMD3180_REG_CH5_FINE_GAIN: c_uint; static PCMD3180_REG_CH6_FINE_GAIN: c_uint; static PCMD3180_REG_CH7_FINE_GAIN: c_uint; static PCMD3180_REG_CH8_FINE_GAIN: c_uint;
    static TAA5412_REG_CH1_DIGITAL_VOLUME: c_uint; static TAA5412_REG_CH2_DIGITAL_VOLUME: c_uint; static TAA5412_REG_CH3_DIGITAL_VOLUME: c_uint; static TAA5412_REG_CH4_DIGITAL_VOLUME: c_uint;
    static TAA5412_REG_CH1_FINE_GAIN: c_uint; static TAA5412_REG_CH2_FINE_GAIN: c_uint; static TAA5412_REG_CH3_FINE_GAIN: c_uint; static TAA5412_REG_CH4_FINE_GAIN: c_uint;
}

macro_rules! mc { ($shift:expr, $reg:expr, $max:expr, $invert:expr) => { pcmdevice_mixer_control { shift: $shift, reg: unsafe { $reg }, max: $max, invert: $invert, dev_no: 0 } }; }
static adc5120_analog_gain_ctl: [pcmdevice_mixer_control; 2] = [mc!(1, ADC5120_REG_CH1_ANALOG_GAIN, 0x54, 0), mc!(1, ADC5120_REG_CH2_ANALOG_GAIN, 0x54, 0)];
static adc5120_digi_gain_ctl: [pcmdevice_mixer_control; 2] = [mc!(0, ADC5120_REG_CH1_DIGITAL_GAIN, 0xff, 0), mc!(0, ADC5120_REG_CH2_DIGITAL_GAIN, 0xff, 0)];
static pcm1690_digi_gain_ctl: [pcmdevice_mixer_control; 8] = [mc!(0, PCM1690_REG_CH1_DIGITAL_GAIN, 0xff, 0), mc!(0, PCM1690_REG_CH2_DIGITAL_GAIN, 0xff, 0), mc!(0, PCM1690_REG_CH3_DIGITAL_GAIN, 0xff, 0), mc!(0, PCM1690_REG_CH4_DIGITAL_GAIN, 0xff, 0), mc!(0, PCM1690_REG_CH5_DIGITAL_GAIN, 0xff, 0), mc!(0, PCM1690_REG_CH6_DIGITAL_GAIN, 0xff, 0), mc!(0, PCM1690_REG_CH7_DIGITAL_GAIN, 0xff, 0), mc!(0, PCM1690_REG_CH8_DIGITAL_GAIN, 0xff, 0)];
static pcm6240_analog_gain_ctl: [pcmdevice_mixer_control; 4] = [mc!(2, PCM6240_REG_CH1_ANALOG_GAIN, 0x42, 0), mc!(2, PCM6240_REG_CH2_ANALOG_GAIN, 0x42, 0), mc!(2, PCM6240_REG_CH3_ANALOG_GAIN, 0x42, 0), mc!(2, PCM6240_REG_CH4_ANALOG_GAIN, 0x42, 0)];
static pcm6240_digi_gain_ctl: [pcmdevice_mixer_control; 4] = [mc!(0, PCM6240_REG_CH1_DIGITAL_GAIN, 0xff, 0), mc!(0, PCM6240_REG_CH2_DIGITAL_GAIN, 0xff, 0), mc!(0, PCM6240_REG_CH3_DIGITAL_GAIN, 0xff, 0), mc!(0, PCM6240_REG_CH4_DIGITAL_GAIN, 0xff, 0)];
static pcm6260_analog_gain_ctl: [pcmdevice_mixer_control; 6] = [mc!(2, PCM6260_REG_CH1_ANALOG_GAIN, 0x42, 0), mc!(2, PCM6260_REG_CH2_ANALOG_GAIN, 0x42, 0), mc!(2, PCM6260_REG_CH3_ANALOG_GAIN, 0x42, 0), mc!(2, PCM6260_REG_CH4_ANALOG_GAIN, 0x42, 0), mc!(2, PCM6260_REG_CH5_ANALOG_GAIN, 0x42, 0), mc!(2, PCM6260_REG_CH6_ANALOG_GAIN, 0x42, 0)];
static pcm6260_digi_gain_ctl: [pcmdevice_mixer_control; 6] = [mc!(0, PCM6260_REG_CH1_DIGITAL_GAIN, 0xff, 0), mc!(0, PCM6260_REG_CH2_DIGITAL_GAIN, 0xff, 0), mc!(0, PCM6260_REG_CH3_DIGITAL_GAIN, 0xff, 0), mc!(0, PCM6260_REG_CH4_DIGITAL_GAIN, 0xff, 0), mc!(0, PCM6260_REG_CH5_DIGITAL_GAIN, 0xff, 0), mc!(0, PCM6260_REG_CH6_DIGITAL_GAIN, 0xff, 0)];
static pcm9211_digi_gain_ctl: [pcmdevice_mixer_control; 2] = [mc!(0, PCM9211_REG_CH1_DIGITAL_GAIN, 0xff, 0), mc!(0, PCM9211_REG_CH2_DIGITAL_GAIN, 0xff, 0)];
static pcmd3140_digi_gain_ctl: [pcmdevice_mixer_control; 4] = [mc!(0, PCMD3140_REG_CH1_DIGITAL_GAIN, 0xff, 0), mc!(0, PCMD3140_REG_CH2_DIGITAL_GAIN, 0xff, 0), mc!(0, PCMD3140_REG_CH3_DIGITAL_GAIN, 0xff, 0), mc!(0, PCMD3140_REG_CH4_DIGITAL_GAIN, 0xff, 0)];
static pcmd3140_fine_gain_ctl: [pcmdevice_mixer_control; 4] = [mc!(4, PCMD3140_REG_CH1_FINE_GAIN, 0xf, 0), mc!(4, PCMD3140_REG_CH2_FINE_GAIN, 0xf, 0), mc!(4, PCMD3140_REG_CH3_FINE_GAIN, 0xf, 0), mc!(4, PCMD3140_REG_CH4_FINE_GAIN, 0xf, 0)];
static pcmd3180_digi_gain_ctl: [pcmdevice_mixer_control; 8] = [mc!(0, PCMD3180_REG_CH1_DIGITAL_GAIN, 0xff, 0), mc!(0, PCMD3180_REG_CH2_DIGITAL_GAIN, 0xff, 0), mc!(0, PCMD3180_REG_CH3_DIGITAL_GAIN, 0xff, 0), mc!(0, PCMD3180_REG_CH4_DIGITAL_GAIN, 0xff, 0), mc!(0, PCMD3180_REG_CH5_DIGITAL_GAIN, 0xff, 0), mc!(0, PCMD3180_REG_CH6_DIGITAL_GAIN, 0xff, 0), mc!(0, PCMD3180_REG_CH7_DIGITAL_GAIN, 0xff, 0), mc!(0, PCMD3180_REG_CH8_DIGITAL_GAIN, 0xff, 0)];
static pcmd3180_fine_gain_ctl: [pcmdevice_mixer_control; 8] = [mc!(4, PCMD3180_REG_CH1_FINE_GAIN, 0xf, 0), mc!(4, PCMD3180_REG_CH2_FINE_GAIN, 0xf, 0), mc!(4, PCMD3180_REG_CH3_FINE_GAIN, 0xf, 0), mc!(4, PCMD3180_REG_CH4_FINE_GAIN, 0xf, 0), mc!(4, PCMD3180_REG_CH5_FINE_GAIN, 0xf, 0), mc!(4, PCMD3180_REG_CH6_FINE_GAIN, 0xf, 0), mc!(4, PCMD3180_REG_CH7_FINE_GAIN, 0xf, 0), mc!(4, PCMD3180_REG_CH8_FINE_GAIN, 0xf, 0)];
static taa5412_digi_vol_ctl: [pcmdevice_mixer_control; 4] = [mc!(0, TAA5412_REG_CH1_DIGITAL_VOLUME, 0xff, 0), mc!(0, TAA5412_REG_CH2_DIGITAL_VOLUME, 0xff, 0), mc!(0, TAA5412_REG_CH3_DIGITAL_VOLUME, 0xff, 0), mc!(0, TAA5412_REG_CH4_DIGITAL_VOLUME, 0xff, 0)];
static taa5412_fine_gain_ctl_arr: [pcmdevice_mixer_control; 4] = [mc!(4, TAA5412_REG_CH1_FINE_GAIN, 0xf, 0), mc!(4, TAA5412_REG_CH2_FINE_GAIN, 0xf, 0), mc!(4, TAA5412_REG_CH3_FINE_GAIN, 0xf, 4), mc!(0, TAA5412_REG_CH4_FINE_GAIN, 0xf, 4)];

// DECLARE_TLV_DB_* translated as TLV data placeholders retaining names and ranges.
static pcmd3140_dig_gain_tlv: [c_uint; 3] = [0, (-10000i32) as c_uint, 2700];
static pcm1690_fine_dig_gain_tlv: [c_uint; 3] = [0, (-12750i32) as c_uint, 0];
static pcm1690_dig_gain_tlv: [c_uint; 3] = [0, (-25500i32) as c_uint, 0];
static pcm9211_dig_gain_tlv: [c_uint; 3] = [0, (-11450i32) as c_uint, 2000];
static adc5120_fgain_tlv: [c_uint; 3] = [0, (-10050i32) as c_uint, 2700];
static adc5120_chgain_tlv: [c_uint; 3] = [1, 0, 4200];
static pcm6260_fgain_tlv: [c_uint; 3] = [0, (-10000i32) as c_uint, 2700];
static pcm6260_chgain_tlv: [c_uint; 3] = [1, 0, 4200];
static taa5412_dig_vol_tlv: [c_uint; 3] = [0, (-8050i32) as c_uint, 4700];
static taa5412_fine_gain_tlv: [c_uint; 3] = [1, (-80i32) as c_uint, 70];

extern "C" {
    static PCMDEV_GENERIC_VOL_CTRL: c_int;
    static PCMDEV_PCM1690_VOL_CTRL: c_int;
    static PCMDEV_PCM1690_FINE_VOL_CTRL: c_int;
    static PCM1690_REG_MODE_CTRL: c_uint;
    static PCM1690_REG_MODE_CTRL_DAMS_MSK: c_uint;
    static PCM1690_REG_MODE_CTRL_DAMS_WIDE_RANGE: c_uint;
    static PCM1690_REG_MODE_CTRL_DAMS_FINE_STEP: c_uint;
    static PCMDEVICE_BIN_BLK_PRE_POWER_UP: u8_;
    static PCMDEVICE_BIN_BLK_PRE_SHUTDOWN: u8_;
    static PCMDEVICE_CMD_SING_W: u8_;
    static PCMDEVICE_CMD_BURST: u8_;
    static PCMDEVICE_CMD_DELAY: u8_;
    static PCMDEVICE_CMD_FIELD_W: u8_;
    static PCM9211_REG_SW_CTRL: c_uint;
    static PCM9211_REG_SW_CTRL_MRST_MSK: c_uint;
    static PCM9211_REG_SW_CTRL_MRST: c_uint;
}

unsafe extern "C" fn pcmdev_change_dev(pcm_priv: *mut pcmdevice_priv, dev_no: u16_) -> c_int {
    let client = (*pcm_priv).client;
    let map = (*pcm_priv).regmap;
    if (*client).addr == (*pcm_priv).addr[dev_no as usize] { return 0; }
    (*client).addr = (*pcm_priv).addr[dev_no as usize];
    let ret = regmap_write(map, PCMDEVICE_PAGE_SELECT, 0);
    if ret < 0 { dev_err((*pcm_priv).dev, cstr!("%s: err = %d\n"), cstr!("pcmdev_change_dev"), ret); }
    ret
}

unsafe extern "C" fn pcmdev_dev_read(pcm_dev: *mut pcmdevice_priv, dev_no: c_uint, reg: c_uint, val: *mut c_uint) -> c_int {
    if dev_no >= (*pcm_dev).ndev as c_uint {
        dev_err((*pcm_dev).dev, cstr!("%s: no such channel(%d)\n"), cstr!("pcmdev_dev_read"), dev_no);
        return -EINVAL;
    }
    let ret = pcmdev_change_dev(pcm_dev, dev_no as u16_);
    if ret < 0 {
        dev_err((*pcm_dev).dev, cstr!("%s: chg dev err = %d\n"), cstr!("pcmdev_dev_read"), ret);
        return ret;
    }
    let ret = regmap_read((*pcm_dev).regmap, reg, val);
    if ret < 0 { dev_err((*pcm_dev).dev, cstr!("%s: err = %d\n"), cstr!("pcmdev_dev_read"), ret); }
    ret
}

unsafe extern "C" fn pcmdev_dev_update_bits(pcm_dev: *mut pcmdevice_priv, dev_no: c_uint, reg: c_uint, mask: c_uint, value: c_uint) -> c_int {
    if dev_no >= (*pcm_dev).ndev as c_uint {
        dev_err((*pcm_dev).dev, cstr!("%s: no such channel(%d)\n"), cstr!("pcmdev_dev_update_bits"), dev_no);
        return -EINVAL;
    }
    let ret = pcmdev_change_dev(pcm_dev, dev_no as u16_);
    if ret < 0 {
        dev_err((*pcm_dev).dev, cstr!("%s: chg dev err = %d\n"), cstr!("pcmdev_dev_update_bits"), ret);
        return ret;
    }
    let ret = regmap_update_bits((*pcm_dev).regmap, reg, mask, value);
    if ret < 0 { dev_err((*pcm_dev).dev, cstr!("%s: update_bits err=%d\n"), cstr!("pcmdev_dev_update_bits"), ret); }
    ret
}

unsafe extern "C" fn pcmdev_get_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value, vol_ctrl_type: c_int) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let pcm_dev = snd_soc_component_get_drvdata(component);
    let mc = (*kcontrol).private_value as *mut pcmdevice_mixer_control;
    let max = (*mc).max;
    let mask = BIT!(fls(max)) - 1;
    let dev_no = (*mc).dev_no;
    let shift = (*mc).shift;
    let reg = (*mc).reg;
    let mut val: c_uint = 0;
    if (*pcm_dev).chip_id == PCM1690 {
        let ret = pcmdev_dev_read(pcm_dev, dev_no, PCM1690_REG_MODE_CTRL, &mut val);
        if ret != 0 {
            dev_err((*pcm_dev).dev, cstr!("%s: read mode err=%d\n"), cstr!("pcmdev_get_volsw"), ret);
            return ret;
        }
        val &= PCM1690_REG_MODE_CTRL_DAMS_MSK;
        if val == 0 && vol_ctrl_type == PCMDEV_PCM1690_VOL_CTRL {
            (*ucontrol).value.integer.value[0] = -25500;
            return ret;
        }
        if val != 0 && vol_ctrl_type == PCMDEV_PCM1690_FINE_VOL_CTRL {
            (*ucontrol).value.integer.value[0] = -12750;
            return ret;
        }
    }
    let ret = pcmdev_dev_read(pcm_dev, dev_no, reg, &mut val);
    if ret != 0 {
        dev_err((*pcm_dev).dev, cstr!("%s: read err=%d\n"), cstr!("pcmdev_get_volsw"), ret);
        return ret;
    }
    val = (val >> shift) & mask;
    val = if val > max { max } else { val };
    val = if (*mc).invert != 0 { max - val } else { val };
    (*ucontrol).value.integer.value[0] = val as i64;
    ret
}
unsafe extern "C" fn pcmdevice_get_volsw(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { pcmdev_get_volsw(k, u, PCMDEV_GENERIC_VOL_CTRL) }
unsafe extern "C" fn pcm1690_get_volsw(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { pcmdev_get_volsw(k, u, PCMDEV_PCM1690_VOL_CTRL) }
unsafe extern "C" fn pcm1690_get_finevolsw(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { pcmdev_get_volsw(k, u, PCMDEV_PCM1690_FINE_VOL_CTRL) }

unsafe extern "C" fn pcmdev_put_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value, vol_ctrl_type: c_int) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let pcm_dev = snd_soc_component_get_drvdata(component);
    let mc = (*kcontrol).private_value as *mut pcmdevice_mixer_control;
    let max = (*mc).max;
    let mask = BIT!(fls(max)) - 1;
    let dev_no = (*mc).dev_no;
    let shift = (*mc).shift;
    let reg = (*mc).reg;
    let mut val = ((*ucontrol).value.integer.value[0] as c_uint) & mask;
    val = if val > max { max } else { val };
    val = if (*mc).invert != 0 { max - val } else { val };
    let mut val_mask = mask << shift;
    val <<= shift;
    if vol_ctrl_type == PCMDEV_PCM1690_VOL_CTRL {
        val_mask |= PCM1690_REG_MODE_CTRL_DAMS_MSK;
        val |= PCM1690_REG_MODE_CTRL_DAMS_WIDE_RANGE;
    } else if vol_ctrl_type == PCMDEV_PCM1690_FINE_VOL_CTRL {
        val_mask |= PCM1690_REG_MODE_CTRL_DAMS_MSK;
        val |= PCM1690_REG_MODE_CTRL_DAMS_FINE_STEP;
    }
    let mut rc = pcmdev_dev_update_bits(pcm_dev, dev_no, reg, val_mask, val);
    if rc < 0 { dev_err((*pcm_dev).dev, cstr!("%s: update_bits err = %d\n"), cstr!("pcmdev_put_volsw"), rc); } else { rc = 1; }
    rc
}
unsafe extern "C" fn pcmdevice_put_volsw(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { pcmdev_put_volsw(k, u, PCMDEV_GENERIC_VOL_CTRL) }
unsafe extern "C" fn pcm1690_put_volsw(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { pcmdev_put_volsw(k, u, PCMDEV_PCM1690_VOL_CTRL) }
unsafe extern "C" fn pcm1690_put_finevolsw(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { pcmdev_put_volsw(k, u, PCMDEV_PCM1690_FINE_VOL_CTRL) }

macro_rules! ctl {
    ($gain:expr, $ctrl:expr, $get:expr, $put:expr, $name_id:expr) => {
        pcmdev_ctrl_info { gain: $gain.as_ptr(), pcmdev_ctrl: $ctrl.as_ptr(), ctrl_array_size: array_size!($ctrl), get: Some($get), put: Some($put), pcmdev_ctrl_name_id: $name_id }
    };
    () => { pcmdev_ctrl_info { gain: ptr::null(), pcmdev_ctrl: ptr::null(), ctrl_array_size: 0, get: None, put: None, pcmdev_ctrl_name_id: 0 } };
}

static pcmdev_gain_ctl_info: [[pcmdev_ctrl_info; 2]; 21] = [
    [ctl!(adc5120_chgain_tlv, adc5120_analog_gain_ctl, pcmdevice_get_volsw, pcmdevice_put_volsw, 0), ctl!(adc5120_fgain_tlv, adc5120_digi_gain_ctl, pcmdevice_get_volsw, pcmdevice_put_volsw, 1)], // ADC3120
    [ctl!(adc5120_chgain_tlv, adc5120_analog_gain_ctl, pcmdevice_get_volsw, pcmdevice_put_volsw, 0), ctl!(adc5120_fgain_tlv, adc5120_digi_gain_ctl, pcmdevice_get_volsw, pcmdevice_put_volsw, 1)], // ADC5120
    [ctl!(adc5120_chgain_tlv, adc5120_analog_gain_ctl, pcmdevice_get_volsw, pcmdevice_put_volsw, 0), ctl!(adc5120_fgain_tlv, adc5120_digi_gain_ctl, pcmdevice_get_volsw, pcmdevice_put_volsw, 1)], // ADC6120
    [ctl!(), ctl!()], // DIX4192
    [ctl!(pcm1690_fine_dig_gain_tlv, pcm1690_digi_gain_ctl, pcm1690_get_volsw, pcm1690_put_volsw, 1), ctl!(pcm1690_dig_gain_tlv, pcm1690_digi_gain_ctl, pcm1690_get_finevolsw, pcm1690_put_finevolsw, 2)], // PCM1690
    [ctl!(adc5120_chgain_tlv, adc5120_analog_gain_ctl, pcmdevice_get_volsw, pcmdevice_put_volsw, 0), ctl!(adc5120_fgain_tlv, adc5120_digi_gain_ctl, pcmdevice_get_volsw, pcmdevice_put_volsw, 1)], // PCM3120
    [ctl!(pcm6260_chgain_tlv, pcm6240_analog_gain_ctl, pcmdevice_get_volsw, pcmdevice_put_volsw, 0), ctl!(pcm6260_fgain_tlv, pcm6240_digi_gain_ctl, pcmdevice_get_volsw, pcmdevice_put_volsw, 1)], // PCM3140
    [ctl!(adc5120_chgain_tlv, adc5120_analog_gain_ctl, pcmdevice_get_volsw, pcmdevice_put_volsw, 0), ctl!(adc5120_fgain_tlv, adc5120_digi_gain_ctl, pcmdevice_get_volsw, pcmdevice_put_volsw, 1)], // PCM5120
    [ctl!(pcm6260_chgain_tlv, pcm6240_analog_gain_ctl, pcmdevice_get_volsw, pcmdevice_put_volsw, 0), ctl!(pcm6260_fgain_tlv, pcm6240_digi_gain_ctl, pcmdevice_get_volsw, pcmdevice_put_volsw, 1)], // PCM5140
    [ctl!(adc5120_chgain_tlv, adc5120_analog_gain_ctl, pcmdevice_get_volsw, pcmdevice_put_volsw, 0), ctl!(adc5120_fgain_tlv, adc5120_digi_gain_ctl, pcmdevice_get_volsw, pcmdevice_put_volsw, 1)], // PCM6120
    [ctl!(pcm6260_chgain_tlv, pcm6240_analog_gain_ctl, pcmdevice_get_volsw, pcmdevice_put_volsw, 0), ctl!(pcm6260_fgain_tlv, pcm6240_digi_gain_ctl, pcmdevice_get_volsw, pcmdevice_put_volsw, 1)], // PCM6140
    [ctl!(pcm6260_chgain_tlv, pcm6240_analog_gain_ctl, pcmdevice_get_volsw, pcmdevice_put_volsw, 0), ctl!(pcm6260_fgain_tlv, pcm6240_digi_gain_ctl, pcmdevice_get_volsw, pcmdevice_put_volsw, 1)], // PCM6240
    [ctl!(pcm6260_chgain_tlv, pcm6260_analog_gain_ctl, pcmdevice_get_volsw, pcmdevice_put_volsw, 0), ctl!(pcm6260_fgain_tlv, pcm6260_digi_gain_ctl, pcmdevice_get_volsw, pcmdevice_put_volsw, 1)], // PCM6260
    [ctl!(), ctl!(pcm9211_dig_gain_tlv, pcm9211_digi_gain_ctl, pcmdevice_get_volsw, pcmdevice_put_volsw, 1)], // PCM9211
    [ctl!(taa5412_fine_gain_tlv, pcmd3140_fine_gain_ctl, pcmdevice_get_volsw, pcmdevice_put_volsw, 2), ctl!(pcmd3140_dig_gain_tlv, pcmd3140_digi_gain_ctl, pcmdevice_get_volsw, pcmdevice_put_volsw, 1)], // PCMD3140
    [ctl!(taa5412_fine_gain_tlv, pcmd3180_fine_gain_ctl, pcmdevice_get_volsw, pcmdevice_put_volsw, 2), ctl!(pcmd3140_dig_gain_tlv, pcmd3180_digi_gain_ctl, pcmdevice_get_volsw, pcmdevice_put_volsw, 1)], // PCMD3180
    [ctl!(), ctl!()], // PCMD512X
    [ctl!(taa5412_fine_gain_tlv, taa5412_fine_gain_ctl_arr, pcmdevice_get_volsw, pcmdevice_put_volsw, 2), ctl!(taa5412_dig_vol_tlv, taa5412_digi_vol_ctl, pcmdevice_get_volsw, pcmdevice_put_volsw, 1)], // TAA5212
    [ctl!(taa5412_fine_gain_tlv, taa5412_fine_gain_ctl_arr, pcmdevice_get_volsw, pcmdevice_put_volsw, 2), ctl!(taa5412_dig_vol_tlv, taa5412_digi_vol_ctl, pcmdevice_get_volsw, pcmdevice_put_volsw, 1)], // TAA5412
    [ctl!(), ctl!()], // TAD5212
    [ctl!(), ctl!()], // TAD5412
];

unsafe extern "C" fn pcmdev_dev_bulk_write(pcm_dev: *mut pcmdevice_priv, dev_no: c_uint, reg: c_uint, data: *mut u8_, len: c_uint) -> c_int {
    if dev_no >= (*pcm_dev).ndev as c_uint {
        dev_err((*pcm_dev).dev, cstr!("%s: no such channel(%d)\n"), cstr!("pcmdev_dev_bulk_write"), dev_no);
        return -EINVAL;
    }
    let ret = pcmdev_change_dev(pcm_dev, dev_no as u16_);
    if ret < 0 {
        dev_err((*pcm_dev).dev, cstr!("%s: chg dev err = %d\n"), cstr!("pcmdev_dev_bulk_write"), ret);
        return ret;
    }
    let ret = regmap_bulk_write((*pcm_dev).regmap, reg, data, len);
    if ret < 0 { dev_err((*pcm_dev).dev, cstr!("%s: bulk_write err = %d\n"), cstr!("pcmdev_dev_bulk_write"), ret); }
    ret
}

unsafe extern "C" fn pcmdev_dev_write(pcm_dev: *mut pcmdevice_priv, dev_no: c_uint, reg: c_uint, value: c_uint) -> c_int {
    if dev_no >= (*pcm_dev).ndev as c_uint {
        dev_err((*pcm_dev).dev, cstr!("%s: no such channel(%d)\n"), cstr!("pcmdev_dev_write"), dev_no);
        return -EINVAL;
    }
    let ret = pcmdev_change_dev(pcm_dev, dev_no as u16_);
    if ret < 0 {
        dev_err((*pcm_dev).dev, cstr!("%s: chg dev err = %d\n"), cstr!("pcmdev_dev_write"), ret);
        return ret;
    }
    let ret = regmap_write((*pcm_dev).regmap, reg, value);
    if ret < 0 { dev_err((*pcm_dev).dev, cstr!("%s: err = %d\n"), cstr!("pcmdev_dev_write"), ret); }
    ret
}

unsafe extern "C" fn pcmdevice_info_profile(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let pcm_dev = snd_soc_component_get_drvdata(codec);
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = core::cmp::max(0, (*pcm_dev).regbin.ncfgs - 1) as i64;
    0
}
unsafe extern "C" fn pcmdevice_get_profile_id(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let pcm_dev = snd_soc_component_get_drvdata(codec);
    (*ucontrol).value.integer.value[0] = (*pcm_dev).cur_conf as i64;
    0
}
unsafe extern "C" fn pcmdevice_set_profile_id(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let pcm_dev = snd_soc_component_get_drvdata(codec);
    let max = (*pcm_dev).regbin.ncfgs - 1;
    let nr_profile = clamp((*ucontrol).value.integer.value[0] as c_int, 0, max);
    if (*pcm_dev).cur_conf != nr_profile { (*pcm_dev).cur_conf = nr_profile; 1 } else { 0 }
}
unsafe extern "C" fn pcmdevice_info_volsw(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let mc = (*kcontrol).private_value as *mut pcmdevice_mixer_control;
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = (*mc).max as i64;
    0
}

unsafe extern "C" fn pcm9211_sw_rst(pcm_dev: *mut pcmdevice_priv) {
    for i in 0..(*pcm_dev).ndev {
        let ret = pcmdev_dev_update_bits(pcm_dev, i as c_uint, PCM9211_REG_SW_CTRL, PCM9211_REG_SW_CTRL_MRST_MSK, PCM9211_REG_SW_CTRL_MRST);
        if ret < 0 { dev_err((*pcm_dev).dev, cstr!("%s: dev %d swreset fail %d\n"), cstr!("pcm9211_sw_rst"), i, ret); }
    }
}
unsafe extern "C" fn pcmdevice_sw_rst(pcm_dev: *mut pcmdevice_priv) {
    for i in 0..(*pcm_dev).ndev {
        let ret = pcmdev_dev_write(pcm_dev, i as c_uint, PCMDEVICE_REG_SWRESET, PCMDEVICE_REG_SWRESET_RESET);
        if ret < 0 { dev_err((*pcm_dev).dev, cstr!("%s: dev %d swreset fail %d\n"), cstr!("pcmdevice_sw_rst"), i, ret); }
    }
}

unsafe extern "C" fn pcmdevice_add_config(ctxt: *mut c_void, config_data: *const u8_, config_size: c_uint, status: *mut c_int) -> *mut pcmdevice_config_info {
    let pcm_dev = ctxt as *mut pcmdevice_priv;
    let mut cfg_info: *mut pcmdevice_config_info = ptr::null_mut();
    let mut cfg_name = [0 as c_char; 64];
    let mut config_offset: c_uint = 0;
    if (*pcm_dev).regbin.fw_hdr.binary_version_num >= 0x105 {
        if config_offset + 64 > config_size {
            *status = -EINVAL;
            dev_err((*pcm_dev).dev, cstr!("%s: cfg_name out of boundary\n"), cstr!("pcmdevice_add_config"));
            return cfg_info;
        }
        memcpy(cfg_name.as_mut_ptr() as *mut c_void, config_data.add(config_offset as usize) as *const c_void, 64);
        config_offset += 64;
    }
    if config_offset + 4 > config_size {
        *status = -EINVAL;
        dev_err((*pcm_dev).dev, cstr!("%s: nblocks out of boundary\n"), cstr!("pcmdevice_add_config"));
        return cfg_info;
    }
    let nblocks = get_unaligned_be32(config_data.add(config_offset as usize));
    config_offset += 4;
    cfg_info = kzalloc(core::mem::size_of::<pcmdevice_config_info>(), GFP_KERNEL) as *mut pcmdevice_config_info;
    if cfg_info.is_null() { *status = -ENOMEM; return cfg_info; }
    (*cfg_info).blk_data = kzalloc(core::mem::size_of::<*mut pcmdevice_block_data>() * nblocks as usize, GFP_KERNEL) as *mut *mut pcmdevice_block_data;
    if (*cfg_info).blk_data.is_null() { *status = -ENOMEM; return cfg_info; }
    (*cfg_info).nblocks = nblocks;
    memcpy((*cfg_info).cfg_name.as_mut_ptr() as *mut c_void, cfg_name.as_ptr() as *const c_void, (*cfg_info).cfg_name.len());
    (*cfg_info).real_nblocks = 0;
    for i in 0..(*cfg_info).nblocks {
        if config_offset + 12 > config_size {
            *status = -EINVAL;
            dev_err((*pcm_dev).dev, cstr!("%s: out of boundary i = %d nblocks = %u\n"), cstr!("pcmdevice_add_config"), i as c_int, (*cfg_info).nblocks);
            break;
        }
        let bk = kzalloc(core::mem::size_of::<pcmdevice_block_data>(), GFP_KERNEL) as *mut pcmdevice_block_data;
        *(*cfg_info).blk_data.add(i as usize) = bk;
        if bk.is_null() { *status = -ENOMEM; break; }
        (*bk).dev_idx = *config_data.add(config_offset as usize); config_offset += 1;
        (*bk).block_type = *config_data.add(config_offset as usize); config_offset += 1;
        if (*bk).block_type == PCMDEVICE_BIN_BLK_PRE_POWER_UP {
            (*cfg_info).active_dev = if (*bk).dev_idx == 0 { (1u32 << (*pcm_dev).ndev) - 1 } else { 1u32 << ((*bk).dev_idx - 1) };
        }
        (*bk).yram_checksum = get_unaligned_be16(config_data.add(config_offset as usize)); config_offset += 2;
        (*bk).block_size = get_unaligned_be32(config_data.add(config_offset as usize)); config_offset += 4;
        (*bk).n_subblks = get_unaligned_be32(config_data.add(config_offset as usize)); config_offset += 4;
        if config_offset + (*bk).block_size > config_size {
            *status = -EINVAL;
            dev_err((*pcm_dev).dev, cstr!("%s: out of boundary: i = %d blks = %u\n"), cstr!("pcmdevice_add_config"), i as c_int, (*cfg_info).nblocks);
            break;
        }
        (*bk).regdata = kmemdup(config_data.add(config_offset as usize) as *const c_void, (*bk).block_size as usize, GFP_KERNEL) as *mut u8_;
        if (*bk).regdata.is_null() { *status = -ENOMEM; return cfg_info; }
        config_offset += (*bk).block_size;
        (*cfg_info).real_nblocks += 1;
    }
    cfg_info
}

unsafe extern "C" fn pcmdev_gain_ctrl_add(pcm_dev: *mut pcmdevice_priv, dev_no: c_int, ctl_id: c_int) -> c_int {
    let adap = (*(*pcm_dev).client).adapter;
    let comp = (*pcm_dev).component;
    let id = (*pcm_dev).chip_id as usize;
    let nr_chn = pcmdev_gain_ctl_info[id][ctl_id as usize].ctrl_array_size;
    if nr_chn == 0 {
        dev_dbg((*pcm_dev).dev, cstr!("%s: no gain ctrl for %s\n"), cstr!("pcmdev_gain_ctrl_add"), (*pcm_dev).dev_name.as_ptr());
        return 0;
    }
    let controls = devm_kcalloc((*pcm_dev).dev, nr_chn as usize, core::mem::size_of::<snd_kcontrol_new>(), GFP_KERNEL) as *mut snd_kcontrol_new;
    if controls.is_null() { return -ENOMEM; }
    let name_id = pcmdev_gain_ctl_info[id][ctl_id as usize].pcmdev_ctrl_name_id;
    let ctrl_name = pcmdev_ctrl_name[name_id as usize];
    let mut mix_index = 0;
    for chn in 1..=nr_chn {
        let name = devm_kzalloc((*pcm_dev).dev, SNDRV_CTL_ELEM_ID_NAME_MAXLEN, GFP_KERNEL) as *mut c_char;
        if name.is_null() { return -ENOMEM; }
        scnprintf(name, SNDRV_CTL_ELEM_ID_NAME_MAXLEN, ctrl_name, (*pcm_dev).upper_dev_name.as_ptr(), (*adap).nr, dev_no, chn);
        (*controls.add(mix_index as usize)).tlv.p = pcmdev_gain_ctl_info[id][ctl_id as usize].gain;
        let src = pcmdev_gain_ctl_info[id][ctl_id as usize].pcmdev_ctrl.add((chn - 1) as usize);
        let ctrl = devm_kmemdup((*pcm_dev).dev, src as *const c_void, core::mem::size_of::<pcmdevice_mixer_control>(), GFP_KERNEL) as *mut pcmdevice_mixer_control;
        if ctrl.is_null() { return -ENOMEM; }
        (*ctrl).dev_no = dev_no as c_uint;
        (*controls.add(mix_index as usize)).private_value = ctrl as c_ulong;
        (*controls.add(mix_index as usize)).name = name;
        (*controls.add(mix_index as usize)).access = SNDRV_CTL_ELEM_ACCESS_TLV_READ | SNDRV_CTL_ELEM_ACCESS_READWRITE;
        (*controls.add(mix_index as usize)).iface = SNDRV_CTL_ELEM_IFACE_MIXER;
        (*controls.add(mix_index as usize)).info = Some(pcmdevice_info_volsw);
        (*controls.add(mix_index as usize)).get = pcmdev_gain_ctl_info[id][ctl_id as usize].get;
        (*controls.add(mix_index as usize)).put = pcmdev_gain_ctl_info[id][ctl_id as usize].put;
        mix_index += 1;
    }
    let ret = snd_soc_add_component_controls(comp, controls, mix_index as c_uint);
    if ret != 0 { dev_err((*pcm_dev).dev, cstr!("%s: add_controls err = %d\n"), cstr!("pcmdev_gain_ctrl_add"), ret); }
    ret
}

unsafe extern "C" fn pcmdev_profile_ctrl_add(pcm_dev: *mut pcmdevice_priv) -> c_int {
    let comp = (*pcm_dev).component;
    let adap = (*(*pcm_dev).client).adapter;
    let ctrl = devm_kzalloc((*pcm_dev).dev, core::mem::size_of::<snd_kcontrol_new>(), GFP_KERNEL) as *mut snd_kcontrol_new;
    if ctrl.is_null() { return -ENOMEM; }
    let name = devm_kzalloc((*pcm_dev).dev, SNDRV_CTL_ELEM_ID_NAME_MAXLEN, GFP_KERNEL) as *mut c_char;
    if name.is_null() { return -ENOMEM; }
    scnprintf(name, SNDRV_CTL_ELEM_ID_NAME_MAXLEN, cstr!("%s i2c%d Profile id"), (*pcm_dev).upper_dev_name.as_ptr(), (*adap).nr);
    (*ctrl).name = name;
    (*ctrl).iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    (*ctrl).info = Some(pcmdevice_info_profile);
    (*ctrl).get = Some(pcmdevice_get_profile_id);
    (*ctrl).put = Some(pcmdevice_set_profile_id);
    let ret = snd_soc_add_component_controls(comp, ctrl, 1);
    if ret != 0 { dev_err((*pcm_dev).dev, cstr!("%s: add_controls err = %d\n"), cstr!("pcmdev_profile_ctrl_add"), ret); }
    ret
}

unsafe extern "C" fn pcmdevice_config_info_remove(ctxt: *mut c_void) {
    let pcm_dev = ctxt as *mut pcmdevice_priv;
    let regbin = &mut (*pcm_dev).regbin as *mut pcmdevice_regbin;
    let cfg_info = (*regbin).cfg_info;
    if cfg_info.is_null() { return; }
    for i in 0..(*regbin).ncfgs {
        let cfg = *cfg_info.add(i as usize);
        if cfg.is_null() { continue; }
        for j in 0..(*cfg).real_nblocks as c_int {
            let blk = *(*cfg).blk_data.add(j as usize);
            if blk.is_null() { continue; }
            kfree((*blk).regdata as *mut c_void);
            kfree(blk as *mut c_void);
        }
        kfree(cfg as *mut c_void);
    }
    kfree(cfg_info as *mut c_void);
}

unsafe extern "C" fn pcmdev_regbin_ready(fmw: *const firmware, ctxt: *mut c_void) -> c_int {
    let pcm_dev = ctxt as *mut pcmdevice_priv;
    let regbin = &mut (*pcm_dev).regbin as *mut pcmdevice_regbin;
    let fw_hdr = &mut (*regbin).fw_hdr as *mut pcmdevice_regbin_hdr;
    let mut total_config_sz: c_uint = 0;
    let mut offset: c_int = 0;
    let mut ret: c_int = 0;
    if fmw.is_null() || (*fmw).data.is_null() {
        dev_err((*pcm_dev).dev, cstr!("%s: failed to read %s\n"), cstr!("pcmdev_regbin_ready"), (*pcm_dev).bin_name.as_ptr());
        (*pcm_dev).fw_state = PCMDEVICE_FW_LOAD_FAILED;
        return -EINVAL;
    }
    let buf = (*fmw).data as *mut u8_;
    (*fw_hdr).img_sz = get_unaligned_be32(buf.add(offset as usize)); offset += 4;
    if (*fw_hdr).img_sz as usize != (*fmw).size {
        dev_err((*pcm_dev).dev, cstr!("%s: file size(%d) not match %u"), cstr!("pcmdev_regbin_ready"), (*fmw).size as c_int, (*fw_hdr).img_sz);
        (*pcm_dev).fw_state = PCMDEVICE_FW_LOAD_FAILED;
        ret = -EINVAL;
        pcmdevice_config_info_remove(pcm_dev as *mut c_void);
        return ret;
    }
    (*fw_hdr).checksum = get_unaligned_be32(buf.add(offset as usize)); offset += 4;
    (*fw_hdr).binary_version_num = get_unaligned_be32(buf.add(offset as usize));
    if (*fw_hdr).binary_version_num < 0x103 {
        dev_err((*pcm_dev).dev, cstr!("%s: bin version 0x%04x is out of date"), cstr!("pcmdev_regbin_ready"), (*fw_hdr).binary_version_num);
        (*pcm_dev).fw_state = PCMDEVICE_FW_LOAD_FAILED;
        ret = -EINVAL;
        pcmdevice_config_info_remove(pcm_dev as *mut c_void);
        return ret;
    }
    offset += 4;
    (*fw_hdr).drv_fw_version = get_unaligned_be32(buf.add(offset as usize)); offset += 8;
    (*fw_hdr).plat_type = *buf.add(offset as usize); offset += 1;
    (*fw_hdr).dev_family = *buf.add(offset as usize); offset += 1;
    (*fw_hdr).reserve = *buf.add(offset as usize); offset += 1;
    (*fw_hdr).ndev = *buf.add(offset as usize); offset += 1;
    if (*fw_hdr).ndev != (*pcm_dev).ndev as u8_ {
        dev_err((*pcm_dev).dev, cstr!("%s: invalid ndev(%u)\n"), cstr!("pcmdev_regbin_ready"), (*fw_hdr).ndev as c_uint);
        (*pcm_dev).fw_state = PCMDEVICE_FW_LOAD_FAILED;
        ret = -EINVAL;
        pcmdevice_config_info_remove(pcm_dev as *mut c_void);
        return ret;
    }
    if offset as c_uint + PCMDEVICE_MAX_REGBIN_DEVICES > (*fw_hdr).img_sz {
        dev_err((*pcm_dev).dev, cstr!("%s: devs out of boundary!\n"), cstr!("pcmdev_regbin_ready"));
        (*pcm_dev).fw_state = PCMDEVICE_FW_LOAD_FAILED;
        ret = -EINVAL;
        pcmdevice_config_info_remove(pcm_dev as *mut c_void);
        return ret;
    }
    for i in 0..PCMDEVICE_MAX_REGBIN_DEVICES as usize { (*fw_hdr).devs[i] = *buf.add(offset as usize); offset += 1; }
    (*fw_hdr).nconfig = get_unaligned_be32(buf.add(offset as usize)); offset += 4;
    for i in 0..PCMDEVICE_CONFIG_SUM as usize {
        (*fw_hdr).config_size[i] = get_unaligned_be32(buf.add(offset as usize)); offset += 4;
        total_config_sz += (*fw_hdr).config_size[i];
    }
    if (*fw_hdr).img_sz - total_config_sz != offset as c_uint {
        dev_err((*pcm_dev).dev, cstr!("%s: bin file error!\n"), cstr!("pcmdev_regbin_ready"));
        (*pcm_dev).fw_state = PCMDEVICE_FW_LOAD_FAILED;
        ret = -EINVAL;
        pcmdevice_config_info_remove(pcm_dev as *mut c_void);
        return ret;
    }
    let cfg_info = kzalloc(core::mem::size_of::<*mut pcmdevice_config_info>() * (*fw_hdr).nconfig as usize, GFP_KERNEL) as *mut *mut pcmdevice_config_info;
    if cfg_info.is_null() {
        (*pcm_dev).fw_state = PCMDEVICE_FW_LOAD_FAILED;
        ret = -ENOMEM;
        pcmdevice_config_info_remove(pcm_dev as *mut c_void);
        return ret;
    }
    (*regbin).cfg_info = cfg_info;
    (*regbin).ncfgs = 0;
    for i in 0..(*fw_hdr).nconfig as c_int {
        *cfg_info.add(i as usize) = pcmdevice_add_config(ctxt, buf.add(offset as usize), (*fw_hdr).config_size[i as usize], &mut ret);
        if ret != 0 {
            if (*regbin).ncfgs == 0 { (*pcm_dev).fw_state = PCMDEVICE_FW_LOAD_FAILED; }
            break;
        }
        offset += (*fw_hdr).config_size[i as usize] as c_int;
        (*regbin).ncfgs += 1;
    }
    if (*pcm_dev).fw_state == PCMDEVICE_FW_LOAD_FAILED {
        dev_err((*pcm_dev).dev, cstr!("%s: remove config due to fw load error!\n"), cstr!("pcmdev_regbin_ready"));
        pcmdevice_config_info_remove(pcm_dev as *mut c_void);
    }
    ret
}

unsafe extern "C" fn pcmdevice_comp_probe(comp: *mut snd_soc_component) -> c_int {
    let pcm_dev = snd_soc_component_get_drvdata(comp);
    let adap = (*(*pcm_dev).client).adapter;
    (*pcm_dev).component = comp;
    for i in 0..(*pcm_dev).ndev {
        for j in 0..2 {
            let ret = pcmdev_gain_ctrl_add(pcm_dev, i, j);
            if ret < 0 { return ret; }
        }
    }
    if !(*comp).name_prefix.is_null() {
        scnprintf((*pcm_dev).bin_name.as_mut_ptr(), PCMDEVICE_BIN_FILENAME_LEN as usize, cstr!("%s.bin"), (*comp).name_prefix);
    } else {
        scnprintf((*pcm_dev).bin_name.as_mut_ptr(), PCMDEVICE_BIN_FILENAME_LEN as usize, cstr!("%s-i2c-%d-%udev.bin"), (*pcm_dev).dev_name.as_ptr(), (*adap).nr, (*pcm_dev).ndev as c_uint);
    }
    let mut fw_entry: *const firmware = ptr::null();
    let ret = request_firmware(&mut fw_entry, (*pcm_dev).bin_name.as_ptr(), (*pcm_dev).dev);
    if ret != 0 {
        dev_err((*pcm_dev).dev, cstr!("%s: request %s err = %d\n"), cstr!("pcmdevice_comp_probe"), (*pcm_dev).bin_name.as_ptr(), ret);
        return ret;
    }
    let ret = pcmdev_regbin_ready(fw_entry, pcm_dev as *mut c_void);
    if ret != 0 {
        dev_err((*pcm_dev).dev, cstr!("%s: %s parse err = %d\n"), cstr!("pcmdevice_comp_probe"), (*pcm_dev).bin_name.as_ptr(), ret);
        return ret;
    }
    pcmdev_profile_ctrl_add(pcm_dev)
}

unsafe extern "C" fn pcmdevice_comp_remove(codec: *mut snd_soc_component) {
    let pcm_dev = snd_soc_component_get_drvdata(codec);
    if pcm_dev.is_null() { return; }
    pcmdevice_config_info_remove(pcm_dev as *mut c_void);
}

static pcmdevice_dapm_widgets: [snd_soc_dapm_widget; 4] = [
    snd_soc_dapm_widget { _priv: [] }, // SND_SOC_DAPM_AIF_IN("ASI", "ASI Playback", 0, SND_SOC_NOPM, 0, 0)
    snd_soc_dapm_widget { _priv: [] }, // SND_SOC_DAPM_AIF_OUT("ASI1 OUT", "ASI1 Capture", 0, SND_SOC_NOPM, 0, 0)
    snd_soc_dapm_widget { _priv: [] }, // SND_SOC_DAPM_OUTPUT("OUT")
    snd_soc_dapm_widget { _priv: [] }, // SND_SOC_DAPM_INPUT("MIC")
];
static pcmdevice_audio_map: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route { sink: cstr!("OUT"), control: ptr::null(), source: cstr!("ASI") },
    snd_soc_dapm_route { sink: cstr!("ASI1 OUT"), control: ptr::null(), source: cstr!("MIC") },
];
static soc_codec_driver_pcmdevice: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(pcmdevice_comp_probe),
    remove: Some(pcmdevice_comp_remove),
    dapm_widgets: pcmdevice_dapm_widgets.as_ptr(),
    num_dapm_widgets: pcmdevice_dapm_widgets.len(),
    dapm_routes: pcmdevice_audio_map.as_ptr(),
    num_dapm_routes: pcmdevice_audio_map.len(),
    suspend_bias_off: 1,
    idle_bias_on: 0,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn pcmdev_single_byte_wr(pcm_dev: *mut pcmdevice_priv, data: *mut u8_, devn: c_int, sublocksize: c_int) -> c_int {
    let len = get_unaligned_be16(data.add(2));
    let mut offset: c_int = 4;
    if offset + 4 * len as c_int > sublocksize {
        dev_err((*pcm_dev).dev, cstr!("%s: dev-%d byt wr out of boundary\n"), cstr!("pcmdev_single_byte_wr"), devn);
        return -EINVAL;
    }
    for _ in 0..len {
        let ret = pcmdev_dev_write(pcm_dev, devn as c_uint, PCMDEVICE_REG(*data.add((offset + 1) as usize), *data.add((offset + 2) as usize)), *data.add((offset + 3) as usize) as c_uint);
        if ret < 0 { dev_err((*pcm_dev).dev, cstr!("%s: dev-%d single write err\n"), cstr!("pcmdev_single_byte_wr"), devn); }
        offset += 4;
    }
    offset
}
unsafe extern "C" fn pcmdev_burst_wr(pcm_dev: *mut pcmdevice_priv, data: *mut u8_, devn: c_int, sublocksize: c_int) -> c_int {
    let len = get_unaligned_be16(data.add(2));
    let mut offset: c_int = 4;
    if offset + 4 + len as c_int > sublocksize {
        dev_err((*pcm_dev).dev, cstr!("%s: dev-%d burst Out of boundary\n"), cstr!("pcmdev_burst_wr"), devn);
        return -EINVAL;
    }
    if len % 4 != 0 {
        dev_err((*pcm_dev).dev, cstr!("%s: dev-%d bst-len(%u) not div by 4\n"), cstr!("pcmdev_burst_wr"), devn, len as c_uint);
        return -EINVAL;
    }
    let ret = pcmdev_dev_bulk_write(pcm_dev, devn as c_uint, PCMDEVICE_REG(*data.add((offset + 1) as usize), *data.add((offset + 2) as usize)), data.add((offset + 4) as usize), len as c_uint);
    if ret < 0 { dev_err((*pcm_dev).dev, cstr!("%s: dev-%d bulk_write err = %d\n"), cstr!("pcmdev_burst_wr"), devn, ret); }
    offset += len as c_int + 4;
    offset
}
unsafe extern "C" fn pcmdev_delay(pcm_dev: *mut pcmdevice_priv, data: *mut u8_, devn: c_int, sublocksize: c_int) -> c_int {
    let mut offset: c_int = 2;
    if offset + 2 > sublocksize {
        dev_err((*pcm_dev).dev, cstr!("%s: dev-%d delay out of boundary\n"), cstr!("pcmdev_delay"), devn);
        return -EINVAL;
    }
    let delay_time = get_unaligned_be16(data.add(2)) as c_uint * 1000;
    usleep_range(delay_time, delay_time + 50);
    offset += 2;
    offset
}
unsafe extern "C" fn pcmdev_bits_wr(pcm_dev: *mut pcmdevice_priv, data: *mut u8_, devn: c_int, sublocksize: c_int) -> c_int {
    let mut offset: c_int = 2;
    if offset + 6 > sublocksize {
        dev_err((*pcm_dev).dev, cstr!("%s: dev-%d bit write out of memory\n"), cstr!("pcmdev_bits_wr"), devn);
        return -EINVAL;
    }
    let ret = pcmdev_dev_update_bits(pcm_dev, devn as c_uint, PCMDEVICE_REG(*data.add((offset + 3) as usize), *data.add((offset + 4) as usize)), *data.add((offset + 1) as usize) as c_uint, *data.add((offset + 5) as usize) as c_uint);
    if ret < 0 { dev_err((*pcm_dev).dev, cstr!("%s: dev-%d update_bits err = %d\n"), cstr!("pcmdev_bits_wr"), devn, ret); }
    offset += 6;
    offset
}

unsafe extern "C" fn pcmdevice_process_block(ctxt: *mut c_void, data: *mut u8_, dev_idx: u8_, sublocksize: c_int) -> c_int {
    let pcm_dev = ctxt as *mut pcmdevice_priv;
    let mut ret = 0;
    let subblk_typ = *data.add(1);
    let (mut devn, dev_end) = if dev_idx != 0 { ((dev_idx - 1) as c_int, dev_idx as c_int) } else { (0, (*pcm_dev).ndev) };
    while devn < dev_end {
        if subblk_typ == PCMDEVICE_CMD_SING_W { ret = pcmdev_single_byte_wr(pcm_dev, data, devn, sublocksize); }
        else if subblk_typ == PCMDEVICE_CMD_BURST { ret = pcmdev_burst_wr(pcm_dev, data, devn, sublocksize); }
        else if subblk_typ == PCMDEVICE_CMD_DELAY { ret = pcmdev_delay(pcm_dev, data, devn, sublocksize); }
        else if subblk_typ == PCMDEVICE_CMD_FIELD_W { ret = pcmdev_bits_wr(pcm_dev, data, devn, sublocksize); }
        if ret < 0 { break; }
        devn += 1;
    }
    ret
}

unsafe extern "C" fn pcmdevice_select_cfg_blk(ctxt: *mut c_void, conf_no: c_int, block_type: u8_) {
    let pcm_dev = ctxt as *mut pcmdevice_priv;
    let regbin = &mut (*pcm_dev).regbin as *mut pcmdevice_regbin;
    let cfg_info = (*regbin).cfg_info;
    if conf_no >= (*regbin).ncfgs || conf_no < 0 || cfg_info.is_null() {
        dev_err((*pcm_dev).dev, cstr!("%s: conf_no should be less than %u\n"), cstr!("pcmdevice_select_cfg_blk"), (*regbin).ncfgs as c_uint);
        return;
    }
    let cfg = *cfg_info.add(conf_no as usize);
    let blk_data = (*cfg).blk_data;
    for j in 0..(*cfg).real_nblocks as c_int {
        let mut length: c_uint = 0;
        if block_type > 5 || block_type < 2 {
            dev_err((*pcm_dev).dev, cstr!("%s: block_type should be out of range\n"), cstr!("pcmdevice_select_cfg_blk"));
            return;
        }
        let blk = *blk_data.add(j as usize);
        if block_type != (*blk).block_type { continue; }
        for _k in 0..(*blk).n_subblks as c_int {
            let ret = pcmdevice_process_block(pcm_dev as *mut c_void, (*blk).regdata.add(length as usize), (*blk).dev_idx, ((*blk).block_size - length) as c_int);
            length = length.wrapping_add(ret as c_uint);
            if (*blk).block_size < length {
                dev_err((*pcm_dev).dev, cstr!("%s: %u %u out of boundary\n"), cstr!("pcmdevice_select_cfg_blk"), length, (*blk).block_size);
                break;
            }
        }
        if length != (*blk).block_size {
            dev_err((*pcm_dev).dev, cstr!("%s: %u %u size is not same\n"), cstr!("pcmdevice_select_cfg_blk"), length, (*blk).block_size);
        }
    }
}

unsafe extern "C" fn pcmdevice_mute(dai: *mut snd_soc_dai, mute: c_int, _stream: c_int) -> c_int {
    let codec = (*dai).component;
    let pcm_dev = snd_soc_component_get_drvdata(codec);
    if (*pcm_dev).fw_state == PCMDEVICE_FW_LOAD_FAILED {
        dev_err((*pcm_dev).dev, cstr!("%s: bin file not loaded\n"), cstr!("pcmdevice_mute"));
        return -EINVAL;
    }
    let block_type = if mute != 0 { PCMDEVICE_BIN_BLK_PRE_SHUTDOWN } else { PCMDEVICE_BIN_BLK_PRE_POWER_UP };
    pcmdevice_select_cfg_blk(pcm_dev as *mut c_void, (*pcm_dev).cur_conf, block_type);
    0
}

unsafe extern "C" fn pcmdevice_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let pcm_dev = snd_soc_dai_get_drvdata(dai);
    let fsrate = params_rate(params);
    let mut ret = 0;
    match fsrate {
        48000 | 44100 => {}
        _ => {
            dev_err((*pcm_dev).dev, cstr!("%s: incorrect sample rate = %u\n"), cstr!("pcmdevice_hw_params"), fsrate);
            return -EINVAL;
        }
    }
    let slot_width = params_width(params);
    match slot_width {
        16 | 20 | 24 | 32 => {}
        _ => {
            dev_err((*pcm_dev).dev, cstr!("%s: incorrect slot width = %u\n"), cstr!("pcmdevice_hw_params"), slot_width);
            return -EINVAL;
        }
    }
    let bclk_rate = snd_soc_params_to_bclk(params);
    if bclk_rate < 0 {
        dev_err((*pcm_dev).dev, cstr!("%s: incorrect bclk rate = %d\n"), cstr!("pcmdevice_hw_params"), bclk_rate);
        ret = bclk_rate;
    }
    ret
}

static pcmdevice_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops { mute_stream: Some(pcmdevice_mute), hw_params: Some(pcmdevice_hw_params) };
static mut pcmdevice_dai_driver: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: cstr!("pcmdevice-codec"),
    capture: snd_soc_pcm_stream { stream_name: cstr!("Capture"), channels_min: 2, channels_max: PCMDEVICE_MAX_CHANNELS, rates: PCMDEVICE_RATES, formats: PCMDEVICE_FORMATS },
    playback: snd_soc_pcm_stream { stream_name: cstr!("Playback"), channels_min: 2, channels_max: PCMDEVICE_MAX_CHANNELS, rates: PCMDEVICE_RATES, formats: PCMDEVICE_FORMATS },
    ops: &pcmdevice_dai_ops,
    symmetric_rate: 1,
}];

// #ifdef CONFIG_OF
static pcmdevice_of_match: [of_device_id; 22] = [
    of_device_id { compatible: cstr!("ti,adc3120") }, of_device_id { compatible: cstr!("ti,adc5120") },
    of_device_id { compatible: cstr!("ti,adc6120") }, of_device_id { compatible: cstr!("ti,dix4192") },
    of_device_id { compatible: cstr!("ti,pcm1690") }, of_device_id { compatible: cstr!("ti,pcm3120") },
    of_device_id { compatible: cstr!("ti,pcm3140") }, of_device_id { compatible: cstr!("ti,pcm5120") },
    of_device_id { compatible: cstr!("ti,pcm5140") }, of_device_id { compatible: cstr!("ti,pcm6120") },
    of_device_id { compatible: cstr!("ti,pcm6140") }, of_device_id { compatible: cstr!("ti,pcm6240") },
    of_device_id { compatible: cstr!("ti,pcm6260") }, of_device_id { compatible: cstr!("ti,pcm9211") },
    of_device_id { compatible: cstr!("ti,pcmd3140") }, of_device_id { compatible: cstr!("ti,pcmd3180") },
    of_device_id { compatible: cstr!("ti,pcmd512x") }, of_device_id { compatible: cstr!("ti,taa5212") },
    of_device_id { compatible: cstr!("ti,taa5412") }, of_device_id { compatible: cstr!("ti,tad5212") },
    of_device_id { compatible: cstr!("ti,tad5412") }, of_device_id { compatible: ptr::null() },
];
// MODULE_DEVICE_TABLE(of, pcmdevice_of_match);

static pcmdevice_ranges: [regmap_range_cfg; 1] = [regmap_range_cfg {
    range_min: 0,
    range_max: 256 * 128,
    selector_reg: PCMDEVICE_PAGE_SELECT,
    selector_mask: 0xff,
    selector_shift: 0,
    window_start: 0,
    window_len: 128,
}];
static pcmdevice_i2c_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    cache_type: REGCACHE_MAPLE,
    ranges: pcmdevice_ranges.as_ptr(),
    num_ranges: pcmdevice_ranges.len() as c_uint,
    max_register: 256 * 128,
};

unsafe extern "C" fn pcmdevice_remove(pcm_dev: *mut pcmdevice_priv) {
    if (*pcm_dev).irq != 0 { free_irq((*pcm_dev).irq, pcm_dev as *mut c_void); }
    mutex_destroy(&mut (*pcm_dev).codec_lock);
}

unsafe extern "C" fn str_to_upper(mut str_: *mut c_char) -> *mut c_char {
    let orig = str_;
    if str_.is_null() { return ptr::null_mut(); }
    while *str_ != 0 {
        *str_ = toupper(*str_ as c_int) as c_char;
        str_ = str_.add(1);
    }
    orig
}

unsafe extern "C" fn pcmdevice_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let pcm_dev = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<pcmdevice_priv>(), GFP_KERNEL) as *mut pcmdevice_priv;
    if pcm_dev.is_null() { return -ENOMEM; }
    (*pcm_dev).chip_id = i2c_get_match_data(i2c) as usize as c_uint;
    (*pcm_dev).dev = &mut (*i2c).dev;
    (*pcm_dev).client = i2c;
    if (*pcm_dev).chip_id >= MAX_DEVICE { (*pcm_dev).chip_id = 0; }
    strscpy((*pcm_dev).dev_name.as_mut_ptr(), pcmdevice_i2c_id[(*pcm_dev).chip_id as usize].name, (*pcm_dev).dev_name.len());
    strscpy((*pcm_dev).upper_dev_name.as_mut_ptr(), pcmdevice_i2c_id[(*pcm_dev).chip_id as usize].name, (*pcm_dev).upper_dev_name.len());
    str_to_upper((*pcm_dev).upper_dev_name.as_mut_ptr());
    (*pcm_dev).regmap = devm_regmap_init_i2c(i2c, &pcmdevice_i2c_regmap);
    let mut ret = 0;
    if IS_ERR((*pcm_dev).regmap) {
        ret = PTR_ERR((*pcm_dev).regmap);
        dev_err(&mut (*i2c).dev, cstr!("%s: failed to allocate register map: %d\n"), cstr!("pcmdevice_i2c_probe"), ret);
    } else {
        i2c_set_clientdata(i2c, pcm_dev as *mut c_void);
        mutex_init(&mut (*pcm_dev).codec_lock);
        let np = (*(*pcm_dev).dev).of_node;
        let mut dev_addrs = [0u32; 8];
        let mut ndev: c_int = 0;
        if IS_ENABLED_CONFIG_OF() {
            let mut addr: u64_ = 0;
            for i in 0..PCMDEVICE_MAX_I2C_DEVICES as c_int {
                if of_property_read_reg(np, i, &mut addr, ptr::null_mut()) != 0 { break; }
                dev_addrs[ndev as usize] = addr as u32;
                ndev += 1;
            }
        } else {
            ndev = 1;
            dev_addrs[0] = (*i2c).addr as u32;
        }
        (*pcm_dev).irq = of_irq_get(np, 0);
        for i in 0..ndev { (*pcm_dev).addr[i as usize] = dev_addrs[i as usize] as u16_; }
        (*pcm_dev).ndev = ndev;
        (*pcm_dev).hw_rst = devm_gpiod_get_optional(&mut (*i2c).dev, cstr!("reset-gpios"), GPIOD_OUT_HIGH);
        if IS_ERR((*pcm_dev).hw_rst) {
            if (*pcm_dev).chip_id == PCM9211 || (*pcm_dev).chip_id == PCM1690 { pcm9211_sw_rst(pcm_dev); } else { pcmdevice_sw_rst(pcm_dev); }
        } else {
            gpiod_set_value_cansleep((*pcm_dev).hw_rst, 0);
            usleep_range(500, 1000);
            gpiod_set_value_cansleep((*pcm_dev).hw_rst, 1);
        }
        if (*pcm_dev).chip_id != PCM1690 {
            if (*pcm_dev).irq != 0 { dev_dbg((*pcm_dev).dev, cstr!("irq = %d"), (*pcm_dev).irq); }
            else { dev_err((*pcm_dev).dev, cstr!("No irq provided\n")); }
        }
        ret = devm_snd_soc_register_component(&mut (*i2c).dev, &soc_codec_driver_pcmdevice, pcmdevice_dai_driver.as_mut_ptr(), pcmdevice_dai_driver.len() as c_int);
        if ret < 0 { dev_err(&mut (*i2c).dev, cstr!("probe register comp failed %d\n"), ret); }
    }
    if ret < 0 { pcmdevice_remove(pcm_dev); }
    ret
}

unsafe extern "C" fn pcmdevice_i2c_remove(i2c: *mut i2c_client) {
    let pcm_dev = i2c_get_clientdata(i2c);
    pcmdevice_remove(pcm_dev);
}

static mut pcmdevice_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: cstr!("pcmdevice-codec"),
        of_match_table: of_match_ptr(pcmdevice_of_match.as_ptr()),
    },
    probe: Some(pcmdevice_i2c_probe),
    remove: Some(pcmdevice_i2c_remove),
    id_table: pcmdevice_i2c_id.as_ptr(),
};
// module_i2c_driver(pcmdevice_i2c_driver);
// MODULE_AUTHOR("Shenghao Ding <shenghao-ding@ti.com>");
// MODULE_DESCRIPTION("ASoC PCM6240 Family Audio ADC/DAC Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
