// SPDX-License-Identifier: GPL-2.0-only
//
// rt1017-sdca-sdw.c -- rt1017 SDCA ALSA SoC amplifier audio driver
//
// Copyright(c) 2023 Realtek Semiconductor Corp.
//
// Translated from C. Linux, ALSA SoC, SoundWire, regmap, and local
// rt1017-sdca-sdw.h definitions are external dependencies.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u32 = u32;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_slave {
    pub dev: device,
    pub prop: sdw_slave_prop,
    pub bus: *mut sdw_bus,
}

#[repr(C)]
pub struct sdw_slave_prop {
    pub scp_int1_mask: c_uint,
    pub quirks: c_uint,
    pub paging_support: bool,
    pub source_ports: u32,
    pub sink_ports: u32,
    pub src_dpn_prop: *mut sdw_dpn_prop,
    pub sink_dpn_prop: *mut sdw_dpn_prop,
    pub clk_stop_timeout: c_uint,
}

#[repr(C)]
pub struct sdw_dpn_prop {
    pub num: u32,
    pub type_: c_uint,
    pub simple_ch_prep_sm: bool,
    pub ch_prep_timeout: c_uint,
}

#[repr(C)]
pub struct rt1017_sdca_priv {
    pub sdw_slave: *mut sdw_slave,
    pub regmap: *mut regmap,
    pub hw_init: bool,
    pub first_hw_init: bool,
}

#[repr(C)]
pub struct reg_sequence {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct reg_default {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: usize,
    pub cache_type: c_uint,
    pub use_single_read: bool,
    pub use_single_write: bool,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
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
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dapm_widget_def {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct sdw_slave_ops {
    pub read_prop: Option<unsafe extern "C" fn(*mut sdw_slave) -> c_int>,
    pub update_status: Option<unsafe extern "C" fn(*mut sdw_slave, sdw_slave_status) -> c_int>,
}

pub type sdw_slave_status = c_uint;

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_def,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub dev: *mut device,
    pub name: *const c_char,
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
pub struct sdw_stream_config {
    pub frame_rate: c_uint,
    pub ch_count: c_int,
    pub bps: c_int,
    pub direction: sdw_data_direction,
}

pub type sdw_data_direction = c_uint;

#[repr(C)]
pub struct sdw_port_config {
    pub ch_mask: c_int,
    pub num: c_int,
}

#[repr(C)]
pub struct sdw_stream_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub set_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, *mut c_void, c_int) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_ulong,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct sdw_device_id {
    pub mfg_id: c_uint,
    pub part_id: c_uint,
    pub sdw_version: c_uint,
    pub class_id: c_uint,
    pub unique_id: c_uint,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_idle: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct sdw_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut sdw_slave, *const sdw_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut sdw_slave)>,
    pub ops: *const sdw_slave_ops,
    pub id_table: *const sdw_device_id,
}

extern "C" {
    static rt1017_sdca_reg_defaults: [reg_default; 0];

    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_cache_bypass(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_get_noresume(dev: *mut device);
    fn pm_runtime_put_autosuspend(dev: *mut device);
    fn pm_runtime_resume(dev: *mut device) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_multi_reg_write(map: *mut regmap, regs: *const reg_sequence, num_regs: c_int) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dai_dma_data_set(dai: *mut snd_soc_dai, direction: c_int, data: *mut c_void);
    fn snd_soc_dai_set_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream, data: *mut c_void);
    fn snd_soc_dai_get_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream) -> *mut sdw_stream_runtime;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn sdw_stream_add_slave(slave: *mut sdw_slave, stream_config: *mut sdw_stream_config, port_config: *mut sdw_port_config, num_ports: c_int, stream: *mut sdw_stream_runtime) -> c_int;
    fn sdw_stream_remove_slave(slave: *mut sdw_slave, stream: *mut sdw_stream_runtime);
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn devm_regmap_init_sdw(slave: *mut sdw_slave, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_to_sdw_dev(dev: *mut device) -> *mut sdw_slave;
    fn sdw_slave_wait_for_init(slave: *mut sdw_slave, timeout: c_int) -> c_int;
    fn sdw_show_ping_status(bus: *mut sdw_bus, sync: bool);
}

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EACCES: c_int = 13;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_MAPLE: c_uint = 0;
const SDW_SCP_INT1_BUS_CLASH: c_uint = 0;
const SDW_SCP_INT1_PARITY: c_uint = 0;
const SDW_SLAVE_QUIRKS_INVALID_INITIAL_PARITY: c_uint = 0;
const SDW_DPN_FULL: c_uint = 0;
const SDW_SLAVE_UNATTACHED: sdw_slave_status = 0;
const SDW_SLAVE_ATTACHED: sdw_slave_status = 1;
const SND_SOC_DAPM_POST_PMU: c_int = 0x1;
const SND_SOC_DAPM_PRE_PMD: c_int = 0x2;
const SND_SOC_DAPM_PRE_PMU: c_int = 0x4;
const SND_SOC_DAPM_POST_PMD: c_int = 0x8;
const SND_SOC_NOPM: c_int = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SDW_DATA_DIR_RX: sdw_data_direction = 0;
const SDW_DATA_DIR_TX: sdw_data_direction = 1;
const SNDRV_PCM_RATE_44100: c_uint = 0;
const SNDRV_PCM_RATE_48000: c_uint = 0;
const SNDRV_PCM_RATE_96000: c_uint = 0;
const SNDRV_PCM_RATE_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S20_3LE: c_ulong = 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 0;
const SNDRV_PCM_FMTBIT_S24_LE: c_ulong = 0;
const FUNC_NUM_SMART_AMP: c_uint = 0;
const RT1017_SDCA_ENT_UDMPU21: c_uint = 0;
const RT1017_SDCA_ENT_FU: c_uint = 0;
const RT1017_SDCA_ENT_XU22: c_uint = 0;
const RT1017_SDCA_ENT_SAPU29: c_uint = 0;
const RT1017_SDCA_ENT_CS21: c_uint = 0;
const RT1017_SDCA_ENT_PDE23: c_uint = 0;
const RT1017_SDCA_ENT_PDE22: c_uint = 0;
const RT1017_SDCA_CTL_UDMPU_CLUSTER: c_uint = 0;
const RT1017_SDCA_CTL_FU_MUTE: c_uint = 0;
const RT1017_SDCA_CTL_BYPASS: c_uint = 0;
const RT1017_SDCA_CTL_PROT_STAT: c_uint = 0;
const RT1017_SDCA_CTL_FS_INDEX: c_uint = 0;
const RT1017_SDCA_CTL_REQ_POWER_STATE: c_uint = 0;
const RT1017_PWM_TRIM_1: c_uint = 0;
const RT1017_PWM_FREQ_CTL_SRC_SEL_MASK: c_uint = 0;
const RT1017_PWM_FREQ_CTL_SRC_SEL_REG: c_uint = 0;
const RT1017_CLASSD_INT_1: c_uint = 0;
const RT1017_SDCA_RATE_44100HZ: c_uint = 0;
const RT1017_SDCA_RATE_48000HZ: c_uint = 0;
const RT1017_SDCA_RATE_96000HZ: c_uint = 0;
const RT1017_SDCA_RATE_192000HZ: c_uint = 0;

const fn BIT(n: u32) -> u32 {
    1u32 << n
}

const fn SDW_SDCA_CTL(func: c_uint, ent: c_uint, ctl: c_uint, ch: c_uint) -> c_uint {
    (func << 24) | (ent << 16) | (ctl << 8) | ch
}

unsafe extern "C" fn rt1017_sdca_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        0x2f55 | 0x3206 | 0xc000 | 0xc001 | 0xc022 | 0xc030 | 0xc104 | 0xc10b | 0xc10c
        | 0xc110 | 0xc112 | 0xc300 | 0xc301 | 0xc318 | 0xc331 | 0xc340 | 0xc500
        | 0xc502 | 0xc504 | 0xc507 | 0xc509 | 0xc510 | 0xc512 | 0xc518 | 0xc51b
        | 0xc51d | 0xc520 | 0xc600 | 0xc602 | 0xc612 | 0xc622 | 0xc632 | 0xc642
        | 0xc651 | 0xca00 | 0xcb00 | 0xcc00 | 0xcc02 | 0xd017 | 0xd101 | 0xd20c
        | 0xd300 | 0xd370 | 0xd500 | 0xdb14 => true,
        0xc325..=0xc328 | 0xc350..=0xc351 | 0xc540..=0xc542 | 0xc550..=0xc552
        | 0xca09..=0xca0c | 0xca0e..=0xca0f | 0xca10..=0xca11 | 0xca16..=0xca17
        | 0xd01a..=0xd01c | 0xd545..=0xd548 | 0xd5a5..=0xd5a8 | 0xd5aa..=0xd5ad
        | 0xda04..=0xda07 | 0xda09..=0xda0a | 0xda0c..=0xda0f | 0xda11..=0xda14
        | 0xda16..=0xda19 | 0xdab6..=0xdabb | 0xdb09..=0xdb0a => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1017_SDCA_ENT_UDMPU21, RT1017_SDCA_CTL_UDMPU_CLUSTER, 0)
            || x == SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1017_SDCA_ENT_FU, RT1017_SDCA_CTL_FU_MUTE, 0x01)
            || x == SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1017_SDCA_ENT_XU22, RT1017_SDCA_CTL_BYPASS, 0)
            || x == SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1017_SDCA_ENT_SAPU29, RT1017_SDCA_CTL_PROT_STAT, 0)
            || x == SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1017_SDCA_ENT_CS21, RT1017_SDCA_CTL_FS_INDEX, 0)
            || x == SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1017_SDCA_ENT_PDE23, RT1017_SDCA_CTL_REQ_POWER_STATE, 0)
            || x == SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1017_SDCA_ENT_PDE22, RT1017_SDCA_CTL_REQ_POWER_STATE, 0) => true,
        _ => false,
    }
}

unsafe extern "C" fn rt1017_sdca_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        0x2f55 | 0xc000 | 0xc022 | 0xc351 | 0xc518 => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1017_SDCA_ENT_SAPU29, RT1017_SDCA_CTL_PROT_STAT, 0) => true,
        _ => false,
    }
}

static rt1017_blind_write: [reg_sequence; 114] = [
    reg_sequence { reg: 0xc001, def: 0x43 },
    reg_sequence { reg: 0x2f55, def: 0x02 },
    reg_sequence { reg: 0x3206, def: 0x80 },
    reg_sequence { reg: 0x005f, def: 0x7f },
    reg_sequence { reg: 0xd101, def: 0xa0 },
    reg_sequence { reg: 0xc112, def: 0xc0 },
    reg_sequence { reg: 0xc104, def: 0xaa },
    reg_sequence { reg: 0xc110, def: 0x59 },
    reg_sequence { reg: 0xc112, def: 0xc0 },
    reg_sequence { reg: 0xc340, def: 0x80 },
    reg_sequence { reg: 0xd017, def: 0x2c },
    reg_sequence { reg: 0xd01a, def: 0xc8 },
    reg_sequence { reg: 0xd01b, def: 0xcf },
    reg_sequence { reg: 0xd01c, def: 0x0c },
    reg_sequence { reg: 0xd20c, def: 0x14 },
    reg_sequence { reg: 0xdb09, def: 0x0f },
    reg_sequence { reg: 0xdb0a, def: 0x7f },
    reg_sequence { reg: 0xdb14, def: 0x03 },
    reg_sequence { reg: 0xcb00, def: 0x31 },
    reg_sequence { reg: 0xc318, def: 0x44 },
    reg_sequence { reg: 0xc325, def: 0xce },
    reg_sequence { reg: 0xc326, def: 0x13 },
    reg_sequence { reg: 0xc327, def: 0x5f },
    reg_sequence { reg: 0xc328, def: 0xf3 },
    reg_sequence { reg: 0xc350, def: 0xe1 },
    reg_sequence { reg: 0xc351, def: 0x88 },
    reg_sequence { reg: 0xc030, def: 0x14 },
    reg_sequence { reg: 0xc331, def: 0xf2 },
    reg_sequence { reg: 0xc551, def: 0x0f },
    reg_sequence { reg: 0xc552, def: 0xff },
    reg_sequence { reg: 0xc651, def: 0xc0 },
    reg_sequence { reg: 0xc550, def: 0xd0 },
    reg_sequence { reg: 0xc612, def: 0x00 },
    reg_sequence { reg: 0xc622, def: 0x00 },
    reg_sequence { reg: 0xc632, def: 0x00 },
    reg_sequence { reg: 0xc642, def: 0x00 },
    reg_sequence { reg: 0xc602, def: 0xf0 },
    reg_sequence { reg: 0xc600, def: 0xd0 },
    reg_sequence { reg: 0xcc02, def: 0x78 },
    reg_sequence { reg: 0xcc00, def: 0x90 },
    reg_sequence { reg: 0xc300, def: 0x3f },
    reg_sequence { reg: 0xc301, def: 0x1d },
    reg_sequence { reg: 0xc10b, def: 0x2e },
    reg_sequence { reg: 0xc10c, def: 0x36 },
    reg_sequence { reg: 0xd5a5, def: 0x00 },
    reg_sequence { reg: 0xd5a6, def: 0x6a },
    reg_sequence { reg: 0xd5a7, def: 0xaa },
    reg_sequence { reg: 0xd5a8, def: 0xaa },
    reg_sequence { reg: 0xd5aa, def: 0x00 },
    reg_sequence { reg: 0xd5ab, def: 0x16 },
    reg_sequence { reg: 0xd5ac, def: 0xdb },
    reg_sequence { reg: 0xd5ad, def: 0x6d },
    reg_sequence { reg: 0xd545, def: 0x09 },
    reg_sequence { reg: 0xd546, def: 0x30 },
    reg_sequence { reg: 0xd547, def: 0xf0 },
    reg_sequence { reg: 0xd548, def: 0xf0 },
    reg_sequence { reg: 0xd500, def: 0x20 },
    reg_sequence { reg: 0xc504, def: 0x3f },
    reg_sequence { reg: 0xc540, def: 0x00 },
    reg_sequence { reg: 0xc541, def: 0x0a },
    reg_sequence { reg: 0xc542, def: 0x1a },
    reg_sequence { reg: 0xc512, def: 0x00 },
    reg_sequence { reg: 0xc520, def: 0x40 },
    reg_sequence { reg: 0xc51b, def: 0x7f },
    reg_sequence { reg: 0xc51d, def: 0x0f },
    reg_sequence { reg: 0xc500, def: 0x40 },
    reg_sequence { reg: 0xc502, def: 0xde },
    reg_sequence { reg: 0xc507, def: 0x05 },
    reg_sequence { reg: 0xc509, def: 0x05 },
    reg_sequence { reg: 0xc510, def: 0x40 },
    reg_sequence { reg: 0xc518, def: 0xc0 },
    reg_sequence { reg: 0xc500, def: 0xc0 },
    reg_sequence { reg: 0xda0c, def: 0x00 },
    reg_sequence { reg: 0xda0d, def: 0x0b },
    reg_sequence { reg: 0xda0e, def: 0x55 },
    reg_sequence { reg: 0xda0f, def: 0x55 },
    reg_sequence { reg: 0xda04, def: 0x00 },
    reg_sequence { reg: 0xda05, def: 0x51 },
    reg_sequence { reg: 0xda06, def: 0xeb },
    reg_sequence { reg: 0xda07, def: 0x85 },
    reg_sequence { reg: 0xca16, def: 0x0f },
    reg_sequence { reg: 0xca17, def: 0x00 },
    reg_sequence { reg: 0xda09, def: 0x5d },
    reg_sequence { reg: 0xda0a, def: 0xc0 },
    reg_sequence { reg: 0xda11, def: 0x26 },
    reg_sequence { reg: 0xda12, def: 0x66 },
    reg_sequence { reg: 0xda13, def: 0x66 },
    reg_sequence { reg: 0xda14, def: 0x66 },
    reg_sequence { reg: 0xda16, def: 0x79 },
    reg_sequence { reg: 0xda17, def: 0x99 },
    reg_sequence { reg: 0xda18, def: 0x99 },
    reg_sequence { reg: 0xda19, def: 0x99 },
    reg_sequence { reg: 0xca09, def: 0x00 },
    reg_sequence { reg: 0xca0a, def: 0x07 },
    reg_sequence { reg: 0xca0b, def: 0x89 },
    reg_sequence { reg: 0xca0c, def: 0x61 },
    reg_sequence { reg: 0xca0e, def: 0x00 },
    reg_sequence { reg: 0xca0f, def: 0x03 },
    reg_sequence { reg: 0xca10, def: 0xc4 },
    reg_sequence { reg: 0xca11, def: 0xb0 },
    reg_sequence { reg: 0xdab6, def: 0x00 },
    reg_sequence { reg: 0xdab7, def: 0x01 },
    reg_sequence { reg: 0xdab8, def: 0x00 },
    reg_sequence { reg: 0xdab9, def: 0x00 },
    reg_sequence { reg: 0xdaba, def: 0x00 },
    reg_sequence { reg: 0xdabb, def: 0x00 },
    reg_sequence { reg: 0xd017, def: 0x0e },
    reg_sequence { reg: 0xca00, def: 0xcd },
    reg_sequence { reg: 0xc022, def: 0x84 },
];

const RT1017_MAX_REG_NUM: c_uint = 0x4108ffff;

static rt1017_sdca_regmap: regmap_config = regmap_config {
    reg_bits: 32,
    val_bits: 8,
    readable_reg: Some(rt1017_sdca_readable_register),
    volatile_reg: Some(rt1017_sdca_volatile_register),
    max_register: RT1017_MAX_REG_NUM,
    reg_defaults: unsafe { rt1017_sdca_reg_defaults.as_ptr() },
    num_reg_defaults: 0,
    cache_type: REGCACHE_MAPLE,
    use_single_read: true,
    use_single_write: true,
};

unsafe extern "C" fn rt1017_sdca_read_prop(slave: *mut sdw_slave) -> c_int {
    let prop = &mut (*slave).prop as *mut sdw_slave_prop;
    let mut i: c_int;
    let mut j: c_int;
    let mut bit: u32;
    let mut addr: c_ulong;
    let mut dpn: *mut sdw_dpn_prop;

    (*prop).scp_int1_mask = SDW_SCP_INT1_BUS_CLASH | SDW_SCP_INT1_PARITY;
    (*prop).quirks = SDW_SLAVE_QUIRKS_INVALID_INITIAL_PARITY;
    (*prop).paging_support = true;

    /*
     * first we need to allocate memory for set bits in port lists
     * port = 1 for AMP playback
     * port = 2 for IV capture
     */
    (*prop).source_ports = BIT(2); /* BITMAP: 00000100 */
    (*prop).sink_ports = BIT(1); /* BITMAP: 00000010 */

    let mut nval = (*prop).source_ports.count_ones() as c_int;
    (*prop).src_dpn_prop = devm_kcalloc(&mut (*slave).dev, nval as usize, size_of::<sdw_dpn_prop>(), GFP_KERNEL) as *mut sdw_dpn_prop;
    if (*prop).src_dpn_prop.is_null() {
        return -ENOMEM;
    }

    i = 0;
    dpn = (*prop).src_dpn_prop;
    addr = (*prop).source_ports as c_ulong;
    bit = 0;
    while bit < 32 {
        if (addr & (1c_ulong << bit)) != 0 {
            (*dpn.add(i as usize)).num = bit;
            (*dpn.add(i as usize)).type_ = SDW_DPN_FULL;
            (*dpn.add(i as usize)).simple_ch_prep_sm = true;
            (*dpn.add(i as usize)).ch_prep_timeout = 10;
            i += 1;
        }
        bit += 1;
    }

    /* do this again for sink now */
    nval = (*prop).sink_ports.count_ones() as c_int;
    (*prop).sink_dpn_prop = devm_kcalloc(&mut (*slave).dev, nval as usize, size_of::<sdw_dpn_prop>(), GFP_KERNEL) as *mut sdw_dpn_prop;
    if (*prop).sink_dpn_prop.is_null() {
        return -ENOMEM;
    }

    j = 0;
    dpn = (*prop).sink_dpn_prop;
    addr = (*prop).sink_ports as c_ulong;
    bit = 0;
    while bit < 32 {
        if (addr & (1c_ulong << bit)) != 0 {
            (*dpn.add(j as usize)).num = bit;
            (*dpn.add(j as usize)).type_ = SDW_DPN_FULL;
            (*dpn.add(j as usize)).simple_ch_prep_sm = true;
            (*dpn.add(j as usize)).ch_prep_timeout = 10;
            j += 1;
        }
        bit += 1;
    }

    /* set the timeout values */
    (*prop).clk_stop_timeout = 64;

    0
}

unsafe extern "C" fn rt1017_sdca_io_init(dev: *mut device, slave: *mut sdw_slave) -> c_int {
    let rt1017 = dev_get_drvdata(dev) as *mut rt1017_sdca_priv;

    if (*rt1017).hw_init {
        return 0;
    }

    if (*rt1017).first_hw_init {
        regcache_cache_only((*rt1017).regmap, false);
        regcache_cache_bypass((*rt1017).regmap, true);
    } else {
        /*
         * PM runtime is only enabled when a Slave reports as Attached
         */

        /* set autosuspend parameters */
        pm_runtime_set_autosuspend_delay(&mut (*slave).dev, 3000);
        pm_runtime_use_autosuspend(&mut (*slave).dev);

        /* update count of parent 'active' children */
        pm_runtime_set_active(&mut (*slave).dev);

        /* make sure the device does not suspend immediately */
        pm_runtime_mark_last_busy(&mut (*slave).dev);

        pm_runtime_enable(&mut (*slave).dev);
    }

    pm_runtime_get_noresume(&mut (*slave).dev);

    /* sw reset */
    regmap_write((*rt1017).regmap, 0xc000, 0x02);

    /* initial settings - blind write */
    regmap_multi_reg_write((*rt1017).regmap, rt1017_blind_write.as_ptr(), rt1017_blind_write.len() as c_int);

    if (*rt1017).first_hw_init {
        regcache_cache_bypass((*rt1017).regmap, false);
        regcache_mark_dirty((*rt1017).regmap);
    } else {
        (*rt1017).first_hw_init = true;
    }

    /* Mark Slave initialization complete */
    (*rt1017).hw_init = true;

    pm_runtime_put_autosuspend(&mut (*slave).dev);

    /* dev_dbg(&slave->dev, "hw_init complete\n"); */
    0
}

unsafe extern "C" fn rt1017_sdca_update_status(slave: *mut sdw_slave, status: sdw_slave_status) -> c_int {
    let rt1017 = dev_get_drvdata(&mut (*slave).dev) as *mut rt1017_sdca_priv;

    if status == SDW_SLAVE_UNATTACHED {
        (*rt1017).hw_init = false;
    }

    /*
     * Perform initialization only if slave status is present and
     * hw_init flag is false
     */
    if (*rt1017).hw_init || status != SDW_SLAVE_ATTACHED {
        return 0;
    }

    /* perform I/O transfers required for Slave initialization */
    rt1017_sdca_io_init(&mut (*slave).dev, slave)
}

static rt1017_rx_data_ch_select: [*const c_char; 11] = [
    b"Bypass\0".as_ptr() as *const c_char,
    b"CN1\0".as_ptr() as *const c_char,
    b"CN2\0".as_ptr() as *const c_char,
    b"CN3\0".as_ptr() as *const c_char,
    b"CN4\0".as_ptr() as *const c_char,
    b"(1+2)/2\0".as_ptr() as *const c_char,
    b"(1+3)/2\0".as_ptr() as *const c_char,
    b"(1+4)/2\0".as_ptr() as *const c_char,
    b"(2+3)/2\0".as_ptr() as *const c_char,
    b"(2+4)/2\0".as_ptr() as *const c_char,
    b"(3+4)/2\0".as_ptr() as *const c_char,
];

/* SOC_ENUM_SINGLE_DECL(rt1017_rx_data_ch_enum, SDW_SDCA_CTL(...), 0, rt1017_rx_data_ch_select); */
/* static const struct snd_kcontrol_new rt1017_sdca_controls[] = { SOC_ENUM("RX Channel Select", rt1017_rx_data_ch_enum), }; */
static rt1017_sdca_controls: [snd_kcontrol_new; 0] = [];
/* static const struct snd_kcontrol_new rt1017_sto_dac = SOC_DAPM_SINGLE("Switch", SDW_SDCA_CTL(...), 0, 1, 1); */
static rt1017_sto_dac: snd_kcontrol_new = snd_kcontrol_new { _private: [] };

unsafe extern "C" fn rt1017_sdca_pde23_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt1017 = snd_soc_component_get_drvdata(component) as *mut rt1017_sdca_priv;
    let ps0: u8 = 0x0;
    let ps3: u8 = 0x3;

    match event {
        SND_SOC_DAPM_POST_PMU => {
            regmap_write((*rt1017).regmap, SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1017_SDCA_ENT_PDE23, RT1017_SDCA_CTL_REQ_POWER_STATE, 0), ps0 as c_uint);
        }
        SND_SOC_DAPM_PRE_PMD => {
            regmap_write((*rt1017).regmap, SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1017_SDCA_ENT_PDE23, RT1017_SDCA_CTL_REQ_POWER_STATE, 0), ps3 as c_uint);
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn rt1017_sdca_classd_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt1017 = snd_soc_component_get_drvdata(component) as *mut rt1017_sdca_priv;

    match event {
        SND_SOC_DAPM_POST_PMU => {
            regmap_update_bits((*rt1017).regmap, RT1017_PWM_TRIM_1, RT1017_PWM_FREQ_CTL_SRC_SEL_MASK, RT1017_PWM_FREQ_CTL_SRC_SEL_REG);
            regmap_write((*rt1017).regmap, RT1017_CLASSD_INT_1, 0x10);
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn rt1017_sdca_feedback_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt1017 = snd_soc_component_get_drvdata(component) as *mut rt1017_sdca_priv;

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            regmap_update_bits((*rt1017).regmap, 0xd017, 0x1f, 0x08);
        }
        SND_SOC_DAPM_POST_PMD => {
            regmap_update_bits((*rt1017).regmap, 0xd017, 0x1f, 0x09);
        }
        _ => {}
    }

    0
}

/*
 * static const struct snd_soc_dapm_widget rt1017_sdca_dapm_widgets[] = {
 *     SND_SOC_DAPM_AIF_IN(...), SND_SOC_DAPM_AIF_OUT_E(...),
 *     SND_SOC_DAPM_SWITCH(...), SND_SOC_DAPM_PGA_E(...),
 *     SND_SOC_DAPM_OUTPUT(...), SND_SOC_DAPM_SUPPLY(...),
 *     SND_SOC_DAPM_PGA(...), SND_SOC_DAPM_SIGGEN(...),
 * };
 */
static rt1017_sdca_dapm_widgets: [snd_soc_dapm_widget_def; 0] = [];

static rt1017_sdca_dapm_routes: [snd_soc_dapm_route; 10] = [
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: b"Switch\0".as_ptr() as *const c_char, source: b"DP1RX\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"CLASS D\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DAC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"CLASS D\0".as_ptr() as *const c_char, control: ptr::null(), source: b"PDE23\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SPO\0".as_ptr() as *const c_char, control: ptr::null(), source: b"CLASS D\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"I Sense\0".as_ptr() as *const c_char, control: ptr::null(), source: b"I Gen\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"V Sense\0".as_ptr() as *const c_char, control: ptr::null(), source: b"V Gen\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"I Sense\0".as_ptr() as *const c_char, control: ptr::null(), source: b"PDE23\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"V Sense\0".as_ptr() as *const c_char, control: ptr::null(), source: b"PDE23\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DP2TX\0".as_ptr() as *const c_char, control: ptr::null(), source: b"I Sense\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DP2TX\0".as_ptr() as *const c_char, control: ptr::null(), source: b"V Sense\0".as_ptr() as *const c_char },
];

static rt1017_sdca_slave_ops: sdw_slave_ops = sdw_slave_ops {
    read_prop: Some(rt1017_sdca_read_prop),
    update_status: Some(rt1017_sdca_update_status),
};

unsafe extern "C" fn rt1017_sdca_component_probe(component: *mut snd_soc_component) -> c_int {
    let ret = pm_runtime_resume((*component).dev);
    if ret < 0 && ret != -EACCES {
        return ret;
    }

    0
}

unsafe extern "C" fn rt1017_sdca_component_remove(component: *mut snd_soc_component) {
    let rt1017 = snd_soc_component_get_drvdata(component) as *mut rt1017_sdca_priv;

    regcache_cache_only((*rt1017).regmap, true);
}

static soc_sdca_component_rt1017: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(rt1017_sdca_component_probe),
    remove: Some(rt1017_sdca_component_remove),
    controls: rt1017_sdca_controls.as_ptr(),
    num_controls: rt1017_sdca_controls.len() as c_uint,
    dapm_widgets: rt1017_sdca_dapm_widgets.as_ptr(),
    num_dapm_widgets: rt1017_sdca_dapm_widgets.len() as c_uint,
    dapm_routes: rt1017_sdca_dapm_routes.as_ptr(),
    num_dapm_routes: rt1017_sdca_dapm_routes.len() as c_uint,
    endianness: 1,
};

unsafe extern "C" fn rt1017_sdca_set_sdw_stream(dai: *mut snd_soc_dai, sdw_stream: *mut c_void, direction: c_int) -> c_int {
    snd_soc_dai_dma_data_set(dai, direction, sdw_stream);

    0
}

unsafe extern "C" fn rt1017_sdca_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    snd_soc_dai_set_dma_data(dai, substream, ptr::null_mut());
}

unsafe extern "C" fn rt1017_sdca_pcm_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let rt1017 = snd_soc_component_get_drvdata(component) as *mut rt1017_sdca_priv;
    let mut stream_config: sdw_stream_config = core::mem::zeroed();
    let mut port_config: sdw_port_config = core::mem::zeroed();
    let direction: sdw_data_direction;
    let port: c_int;
    let num_channels: c_int;
    let ch_mask: c_int;
    let sampling_rate: c_uint;

    /* dev_dbg(dai->dev, "%s %s", __func__, dai->name); */
    let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);

    if sdw_stream.is_null() {
        return -EINVAL;
    }

    if (*rt1017).sdw_slave.is_null() {
        return -EINVAL;
    }

    /* SoundWire specific configuration */
    /* port 1 for playback */
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        direction = SDW_DATA_DIR_RX;
        port = 1;
    } else {
        direction = SDW_DATA_DIR_TX;
        port = 2;
    }

    num_channels = params_channels(params);
    ch_mask = (1 << num_channels) - 1;

    stream_config.frame_rate = params_rate(params);
    stream_config.ch_count = num_channels;
    stream_config.bps = snd_pcm_format_width(params_format(params));
    stream_config.direction = direction;

    port_config.ch_mask = ch_mask;
    port_config.num = port;

    /* dev_dbg(dai->dev, "frame_rate %d, ch_count %d, bps %d, direction %d, ch_mask %d, port: %d\n", ...); */

    let retval = sdw_stream_add_slave((*rt1017).sdw_slave, &mut stream_config, &mut port_config, 1, sdw_stream);
    if retval != 0 {
        /* dev_err(dai->dev, "Unable to configure port\n"); */
        return retval;
    }

    /* sampling rate configuration */
    match params_rate(params) {
        44100 => sampling_rate = RT1017_SDCA_RATE_44100HZ,
        48000 => sampling_rate = RT1017_SDCA_RATE_48000HZ,
        96000 => sampling_rate = RT1017_SDCA_RATE_96000HZ,
        192000 => sampling_rate = RT1017_SDCA_RATE_192000HZ,
        _ => {
            /* dev_err(component->dev, "Rate %d is not supported\n", params_rate(params)); */
            return -EINVAL;
        }
    }

    /* set sampling frequency */
    regmap_write((*rt1017).regmap, SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1017_SDCA_ENT_CS21, RT1017_SDCA_CTL_FS_INDEX, 0), sampling_rate);

    0
}

unsafe extern "C" fn rt1017_sdca_pcm_hw_free(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let rt1017 = snd_soc_component_get_drvdata(component) as *mut rt1017_sdca_priv;
    let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);

    if (*rt1017).sdw_slave.is_null() {
        return -EINVAL;
    }

    sdw_stream_remove_slave((*rt1017).sdw_slave, sdw_stream);
    0
}

static rt1017_sdca_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(rt1017_sdca_pcm_hw_params),
    hw_free: Some(rt1017_sdca_pcm_hw_free),
    set_stream: Some(rt1017_sdca_set_sdw_stream),
    shutdown: Some(rt1017_sdca_shutdown),
};

const RT1017_STEREO_RATES: c_uint = SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000;
const RT1017_FORMATS: c_ulong = SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE;

static mut rt1017_sdca_dai: [snd_soc_dai_driver; 1] = [
    snd_soc_dai_driver {
        name: b"rt1017-aif\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream {
            stream_name: b"DP1 Playback\0".as_ptr() as *const c_char,
            channels_min: 1,
            channels_max: 1,
            rates: RT1017_STEREO_RATES,
            formats: RT1017_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: b"DP2 Capture\0".as_ptr() as *const c_char,
            channels_min: 1,
            channels_max: 1,
            rates: RT1017_STEREO_RATES,
            formats: RT1017_FORMATS,
        },
        ops: &rt1017_sdca_ops,
    },
];

unsafe extern "C" fn rt1017_sdca_init(dev: *mut device, regmap: *mut regmap, slave: *mut sdw_slave) -> c_int {
    let rt1017: *mut rt1017_sdca_priv;
    let ret: c_int;

    rt1017 = devm_kzalloc(dev, size_of::<rt1017_sdca_priv>(), GFP_KERNEL) as *mut rt1017_sdca_priv;
    if rt1017.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(dev, rt1017 as *mut c_void);
    (*rt1017).sdw_slave = slave;
    (*rt1017).regmap = regmap;

    /*
     * Mark hw_init to false
     * HW init will be performed when device reports present
     */
    (*rt1017).hw_init = false;
    (*rt1017).first_hw_init = false;

    ret = devm_snd_soc_register_component(dev, &soc_sdca_component_rt1017, rt1017_sdca_dai.as_mut_ptr(), rt1017_sdca_dai.len() as c_int);

    ret
}

unsafe extern "C" fn rt1017_sdca_sdw_probe(slave: *mut sdw_slave, _id: *const sdw_device_id) -> c_int {
    let regmap: *mut regmap;

    /* Regmap Initialization */
    regmap = devm_regmap_init_sdw(slave, &rt1017_sdca_regmap);
    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }

    rt1017_sdca_init(&mut (*slave).dev, regmap, slave)
}

unsafe extern "C" fn rt1017_sdca_sdw_remove(slave: *mut sdw_slave) {
    let rt1017 = dev_get_drvdata(&mut (*slave).dev) as *mut rt1017_sdca_priv;

    if (*rt1017).first_hw_init {
        pm_runtime_disable(&mut (*slave).dev);
    }
}

static rt1017_sdca_id: [sdw_device_id; 2] = [
    sdw_device_id { mfg_id: 0x025d, part_id: 0x1017, sdw_version: 0x3, class_id: 0x1, unique_id: 0 },
    sdw_device_id { mfg_id: 0, part_id: 0, sdw_version: 0, class_id: 0, unique_id: 0 },
];
/* MODULE_DEVICE_TABLE(sdw, rt1017_sdca_id); */

unsafe extern "C" fn rt1017_sdca_dev_suspend(dev: *mut device) -> c_int {
    let rt1017 = dev_get_drvdata(dev) as *mut rt1017_sdca_priv;

    if !(*rt1017).hw_init {
        return 0;
    }

    regcache_cache_only((*rt1017).regmap, true);

    0
}

const RT1017_PROBE_TIMEOUT: c_int = 5000;

unsafe extern "C" fn rt1017_sdca_dev_resume(dev: *mut device) -> c_int {
    let slave = dev_to_sdw_dev(dev);
    let rt1017 = dev_get_drvdata(dev) as *mut rt1017_sdca_priv;
    let mut ret: c_int;

    if !(*rt1017).first_hw_init {
        return 0;
    }

    ret = sdw_slave_wait_for_init(slave, RT1017_PROBE_TIMEOUT);
    if ret != 0 {
        sdw_show_ping_status((*slave).bus, true);
        return ret;
    }

    regcache_cache_only((*rt1017).regmap, false);
    ret = regcache_sync((*rt1017).regmap);
    if ret != 0 {
        regcache_cache_only((*rt1017).regmap, true);
        regcache_mark_dirty((*rt1017).regmap);
        return ret;
    }

    0
}

static rt1017_sdca_pm: dev_pm_ops = dev_pm_ops {
    suspend: Some(rt1017_sdca_dev_suspend),
    resume: Some(rt1017_sdca_dev_resume),
    runtime_suspend: Some(rt1017_sdca_dev_suspend),
    runtime_resume: Some(rt1017_sdca_dev_resume),
    runtime_idle: None,
};

static mut rt1017_sdca_sdw_driver: sdw_driver = sdw_driver {
    driver: device_driver {
        name: b"rt1017-sdca\0".as_ptr() as *const c_char,
        pm: &rt1017_sdca_pm,
    },
    probe: Some(rt1017_sdca_sdw_probe),
    remove: Some(rt1017_sdca_sdw_remove),
    ops: &rt1017_sdca_slave_ops,
    id_table: rt1017_sdca_id.as_ptr(),
};
/* module_sdw_driver(rt1017_sdca_sdw_driver); */

/* MODULE_DESCRIPTION("ASoC RT1017 driver SDCA SDW"); */
/* MODULE_AUTHOR("Derek Fang <derek.fang@realtek.com>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
