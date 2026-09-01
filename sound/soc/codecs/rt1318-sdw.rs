// SPDX-License-Identifier: GPL-2.0-only
//
// rt1318-sdw.c -- rt1318 SDCA ALSA SoC amplifier audio driver
//
// Copyright(c) 2022 Realtek Semiconductor Corp.
//
// Rust translation of the isolated C implementation source. Linux, SoundWire,
// ASoC, regmap, and rt1318-sdw.h items are external dependencies.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct regmap { _private: [u8; 0] }
#[repr(C)]
pub struct sdw_slave { pub dev: device, pub prop: sdw_slave_prop }
#[repr(C)]
pub struct sdw_device_id { _private: [u8; 0] }
#[repr(C)]
pub struct snd_soc_dapm_widget { pub dapm: *mut snd_soc_dapm_context }
#[repr(C)]
pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)]
pub struct snd_kcontrol { _private: [u8; 0] }
#[repr(C)]
pub struct snd_soc_component { pub dev: *mut device }
#[repr(C)]
pub struct snd_soc_dai { pub component: *mut snd_soc_component, pub dev: *mut device, pub name: *const c_char }
#[repr(C)]
pub struct snd_pcm_substream { pub stream: c_int }
#[repr(C)]
pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)]
pub struct sdw_stream_runtime { _private: [u8; 0] }

#[repr(C)]
pub struct reg_sequence { pub reg: c_uint, pub def: c_uint }
#[repr(C)]
pub struct reg_default { pub reg: c_uint, pub def: c_uint }

#[repr(C)]
pub struct sdw_dpn_prop {
    pub num: u32,
    pub type_: c_int,
    pub simple_ch_prep_sm: bool,
    pub ch_prep_timeout: c_int,
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
    pub clk_stop_timeout: c_int,
}

#[repr(C)]
pub struct rt1318_sdw_priv {
    pub sdw_slave: *mut sdw_slave,
    pub regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub hw_init: bool,
    pub first_hw_init: bool,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub cache_type: c_int,
    pub use_single_read: bool,
    pub use_single_write: bool,
}

#[repr(C)]
pub struct sdw_stream_config {
    pub frame_rate: c_uint,
    pub ch_count: c_int,
    pub bps: c_int,
    pub direction: sdw_data_direction,
}

#[repr(C)]
pub struct sdw_port_config { pub ch_mask: c_int, pub num: c_int }

pub type sdw_data_direction = c_int;
pub type sdw_slave_status = c_int;

#[repr(C)]
pub struct sdw_slave_ops {
    pub read_prop: Option<unsafe extern "C" fn(*mut sdw_slave) -> c_int>,
    pub update_status: Option<unsafe extern "C" fn(*mut sdw_slave, sdw_slave_status) -> c_int>,
}

#[repr(C)]
pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)]
pub struct snd_soc_dapm_widget_def { _private: [u8; 0] }
#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_def,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub endianness: c_int,
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
pub struct dev_pm_ops { _private: [u8; 0] }
#[repr(C)]
pub struct sdw_driver_inner { pub name: *const c_char, pub pm: *const dev_pm_ops }
#[repr(C)]
pub struct sdw_driver {
    pub driver: sdw_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut sdw_slave, *const sdw_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut sdw_slave)>,
    pub ops: *const sdw_slave_ops,
    pub id_table: *const sdw_device_id,
}

extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_cache_bypass(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regmap_multi_reg_write(map: *mut regmap, regs: *const reg_sequence, num_regs: c_int) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_get_noresume(dev: *mut device);
    fn pm_runtime_put_autosuspend(dev: *mut device);
    fn pm_runtime_resume(dev: *mut device) -> c_int;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
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
    fn devm_snd_soc_register_component(dev: *mut device, driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn devm_regmap_init_sdw(slave: *mut sdw_slave, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_to_sdw_dev(dev: *mut device) -> *mut sdw_slave;
    fn sdw_slave_wait_for_init(slave: *mut sdw_slave, timeout: c_int) -> c_int;
}

extern "Rust" {
    fn SDW_SDCA_CTL(func: c_uint, entity: c_uint, control: c_uint, channel: c_uint) -> c_uint;
}

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EACCES: c_int = 13;
const REGCACHE_MAPLE: c_int = 0;
const SDW_SCP_INT1_BUS_CLASH: c_uint = 0;
const SDW_SCP_INT1_PARITY: c_uint = 0;
const SDW_SLAVE_QUIRKS_INVALID_INITIAL_PARITY: c_uint = 0;
const SDW_DPN_FULL: c_int = 0;
const SDW_SLAVE_UNATTACHED: sdw_slave_status = 0;
const SDW_SLAVE_ATTACHED: sdw_slave_status = 1;
const SND_SOC_DAPM_POST_PMU: c_int = 0;
const SND_SOC_DAPM_PRE_PMD: c_int = 0;
const SND_SOC_NOPM: c_int = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SDW_DATA_DIR_RX: sdw_data_direction = 0;
const SDW_DATA_DIR_TX: sdw_data_direction = 1;
const SNDRV_PCM_RATE_16000: c_uint = 0;
const SNDRV_PCM_RATE_32000: c_uint = 0;
const SNDRV_PCM_RATE_44100: c_uint = 0;
const SNDRV_PCM_RATE_48000: c_uint = 0;
const SNDRV_PCM_RATE_96000: c_uint = 0;
const SNDRV_PCM_RATE_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 0;
const SNDRV_PCM_FMTBIT_S24_LE: c_ulong = 0;
const SNDRV_PCM_FMTBIT_S32_LE: c_ulong = 0;
const FUNC_NUM_SMART_AMP: c_uint = 0;
const RT1318_SDCA_ENT_FU21: c_uint = 0;
const RT1318_SDCA_CTL_FU_MUTE: c_uint = 0;
const CH_L: c_uint = 0;
const CH_R: c_uint = 0;
const RT1318_SDCA_ENT_PDE23: c_uint = 0;
const RT1318_SDCA_CTL_REQ_POWER_STATE: c_uint = 0;
const RT1318_SDCA_ENT_UDMPU21: c_uint = 0;
const RT1318_SDCA_CTL_UDMPU_CLUSTER: c_uint = 0;
const RT1318_SDCA_ENT_CS21: c_uint = 0;
const RT1318_SDCA_CTL_SAMPLE_FREQ_INDEX: c_uint = 0;
const RT1318_SDCA_ENT_SAPU: c_uint = 0;
const RT1318_SDCA_CTL_SAPU_PROTECTION_MODE: c_uint = 0;
const RT1318_SDCA_CTL_SAPU_PROTECTION_STATUS: c_uint = 0;
const RT1318_SDCA_RATE_16000HZ: c_uint = 0;
const RT1318_SDCA_RATE_32000HZ: c_uint = 0;
const RT1318_SDCA_RATE_44100HZ: c_uint = 0;
const RT1318_SDCA_RATE_48000HZ: c_uint = 0;
const RT1318_SDCA_RATE_96000HZ: c_uint = 0;
const RT1318_SDCA_RATE_192000HZ: c_uint = 0;

const fn BIT(n: u32) -> u32 { 1u32 << n }
const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize { N }

static rt1318_blind_write: [reg_sequence; 132] = [
    reg_sequence { reg: 0xc001, def: 0x43 }, reg_sequence { reg: 0xc003, def: 0xa2 },
    reg_sequence { reg: 0xc004, def: 0x44 }, reg_sequence { reg: 0xc005, def: 0x44 },
    reg_sequence { reg: 0xc006, def: 0x33 }, reg_sequence { reg: 0xc007, def: 0x64 },
    reg_sequence { reg: 0xc320, def: 0x20 }, reg_sequence { reg: 0xf203, def: 0x18 },
    reg_sequence { reg: 0xf211, def: 0x00 }, reg_sequence { reg: 0xf212, def: 0x26 },
    reg_sequence { reg: 0xf20d, def: 0x17 }, reg_sequence { reg: 0xf214, def: 0x06 },
    reg_sequence { reg: 0xf20e, def: 0x00 }, reg_sequence { reg: 0xf223, def: 0x7f },
    reg_sequence { reg: 0xf224, def: 0xdb }, reg_sequence { reg: 0xf225, def: 0xee },
    reg_sequence { reg: 0xf226, def: 0x3f }, reg_sequence { reg: 0xf227, def: 0x0f },
    reg_sequence { reg: 0xf21a, def: 0x78 }, reg_sequence { reg: 0xf242, def: 0x3c },
    reg_sequence { reg: 0xc321, def: 0x0b }, reg_sequence { reg: 0xc200, def: 0xd8 },
    reg_sequence { reg: 0xc201, def: 0x27 }, reg_sequence { reg: 0xc202, def: 0x0f },
    reg_sequence { reg: 0xf800, def: 0x20 }, reg_sequence { reg: 0xdf00, def: 0x10 },
    reg_sequence { reg: 0xdf5f, def: 0x01 }, reg_sequence { reg: 0xdf60, def: 0xa7 },
    reg_sequence { reg: 0xc400, def: 0x0e }, reg_sequence { reg: 0xc401, def: 0x43 },
    reg_sequence { reg: 0xc402, def: 0xe0 }, reg_sequence { reg: 0xc403, def: 0x00 },
    reg_sequence { reg: 0xc404, def: 0x4c }, reg_sequence { reg: 0xc407, def: 0x02 },
    reg_sequence { reg: 0xc408, def: 0x3f }, reg_sequence { reg: 0xc300, def: 0x01 },
    reg_sequence { reg: 0xc206, def: 0x78 }, reg_sequence { reg: 0xc203, def: 0x84 },
    reg_sequence { reg: 0xc120, def: 0xc0 }, reg_sequence { reg: 0xc121, def: 0x03 },
    reg_sequence { reg: 0xe000, def: 0x88 }, reg_sequence { reg: 0xc321, def: 0x09 },
    reg_sequence { reg: 0xc322, def: 0x01 }, reg_sequence { reg: 0xe706, def: 0x0f },
    reg_sequence { reg: 0xe707, def: 0x30 }, reg_sequence { reg: 0xe806, def: 0x0f },
    reg_sequence { reg: 0xe807, def: 0x30 }, reg_sequence { reg: 0xed00, def: 0xb0 },
    reg_sequence { reg: 0xce04, def: 0x02 }, reg_sequence { reg: 0xce05, def: 0x63 },
    reg_sequence { reg: 0xce06, def: 0x68 }, reg_sequence { reg: 0xce07, def: 0x07 },
    reg_sequence { reg: 0xcf04, def: 0x02 }, reg_sequence { reg: 0xcf05, def: 0x63 },
    reg_sequence { reg: 0xcf06, def: 0x68 }, reg_sequence { reg: 0xcf07, def: 0x07 },
    reg_sequence { reg: 0xce60, def: 0xe3 }, reg_sequence { reg: 0xc130, def: 0x51 },
    reg_sequence { reg: 0xf102, def: 0x00 }, reg_sequence { reg: 0xf103, def: 0x00 },
    reg_sequence { reg: 0xf104, def: 0xf5 }, reg_sequence { reg: 0xf105, def: 0x06 },
    reg_sequence { reg: 0xf109, def: 0x9b }, reg_sequence { reg: 0xf10a, def: 0x0b },
    reg_sequence { reg: 0xf10b, def: 0x4c }, reg_sequence { reg: 0xf10b, def: 0x5c },
    reg_sequence { reg: 0xf102, def: 0x00 }, reg_sequence { reg: 0xf103, def: 0x00 },
    reg_sequence { reg: 0xf104, def: 0xf5 }, reg_sequence { reg: 0xf105, def: 0x0b },
    reg_sequence { reg: 0xf109, def: 0x03 }, reg_sequence { reg: 0xf10a, def: 0x0b },
    reg_sequence { reg: 0xf10b, def: 0x4c }, reg_sequence { reg: 0xf10b, def: 0x5c },
    reg_sequence { reg: 0xf102, def: 0x00 }, reg_sequence { reg: 0xf103, def: 0x00 },
    reg_sequence { reg: 0xf104, def: 0xf5 }, reg_sequence { reg: 0xf105, def: 0x0c },
    reg_sequence { reg: 0xf109, def: 0x7f }, reg_sequence { reg: 0xf10a, def: 0x0b },
    reg_sequence { reg: 0xf10b, def: 0x4c }, reg_sequence { reg: 0xf10b, def: 0x5c },
    reg_sequence { reg: 0xe604, def: 0x00 }, reg_sequence { reg: 0xdb00, def: 0x0c },
    reg_sequence { reg: 0xdd00, def: 0x0c }, reg_sequence { reg: 0xdc19, def: 0x00 },
    reg_sequence { reg: 0xdc1a, def: 0xff }, reg_sequence { reg: 0xdc1b, def: 0xff },
    reg_sequence { reg: 0xdc1c, def: 0xff }, reg_sequence { reg: 0xdc1d, def: 0x00 },
    reg_sequence { reg: 0xdc1e, def: 0x00 }, reg_sequence { reg: 0xdc1f, def: 0x00 },
    reg_sequence { reg: 0xdc20, def: 0xff }, reg_sequence { reg: 0xde19, def: 0x00 },
    reg_sequence { reg: 0xde1a, def: 0xff }, reg_sequence { reg: 0xde1b, def: 0xff },
    reg_sequence { reg: 0xde1c, def: 0xff }, reg_sequence { reg: 0xde1d, def: 0x00 },
    reg_sequence { reg: 0xde1e, def: 0x00 }, reg_sequence { reg: 0xde1f, def: 0x00 },
    reg_sequence { reg: 0xde20, def: 0xff }, reg_sequence { reg: 0xdb32, def: 0x00 },
    reg_sequence { reg: 0xdd32, def: 0x00 }, reg_sequence { reg: 0xdb33, def: 0x0a },
    reg_sequence { reg: 0xdd33, def: 0x0a }, reg_sequence { reg: 0xdb34, def: 0x1a },
    reg_sequence { reg: 0xdd34, def: 0x1a }, reg_sequence { reg: 0xdb17, def: 0xef },
    reg_sequence { reg: 0xdd17, def: 0xef }, reg_sequence { reg: 0xdba7, def: 0x00 },
    reg_sequence { reg: 0xdba8, def: 0x64 }, reg_sequence { reg: 0xdda7, def: 0x00 },
    reg_sequence { reg: 0xdda8, def: 0x64 }, reg_sequence { reg: 0xdb19, def: 0x40 },
    reg_sequence { reg: 0xdd19, def: 0x40 }, reg_sequence { reg: 0xdb00, def: 0x4c },
    reg_sequence { reg: 0xdb01, def: 0x79 }, reg_sequence { reg: 0xdd01, def: 0x79 },
    reg_sequence { reg: 0xdb04, def: 0x05 }, reg_sequence { reg: 0xdb05, def: 0x03 },
    reg_sequence { reg: 0xdd04, def: 0x05 }, reg_sequence { reg: 0xdd05, def: 0x03 },
    reg_sequence { reg: 0xdbbb, def: 0x09 }, reg_sequence { reg: 0xdbbc, def: 0x30 },
    reg_sequence { reg: 0xdbbd, def: 0xf0 }, reg_sequence { reg: 0xdbbe, def: 0xf1 },
    reg_sequence { reg: 0xddbb, def: 0x09 }, reg_sequence { reg: 0xddbc, def: 0x30 },
    reg_sequence { reg: 0xddbd, def: 0xf0 }, reg_sequence { reg: 0xddbe, def: 0xf1 },
    reg_sequence { reg: 0xdb01, def: 0x79 }, reg_sequence { reg: 0xdd01, def: 0x79 },
    reg_sequence { reg: 0xdc52, def: 0xef }, reg_sequence { reg: 0xde52, def: 0xef },
    reg_sequence { reg: 0x2f55, def: 0x22 },
];

static rt1318_reg_defaults: [reg_default; 82] = [
    reg_default { reg: 0x3000, def: 0x00 }, reg_default { reg: 0x3004, def: 0x01 },
    reg_default { reg: 0x3005, def: 0x23 }, reg_default { reg: 0x3202, def: 0x00 },
    reg_default { reg: 0x3203, def: 0x01 }, reg_default { reg: 0x3206, def: 0x00 },
    reg_default { reg: 0xc000, def: 0x00 }, reg_default { reg: 0xc001, def: 0x43 },
    reg_default { reg: 0xc003, def: 0x22 }, reg_default { reg: 0xc004, def: 0x44 },
    reg_default { reg: 0xc005, def: 0x44 }, reg_default { reg: 0xc006, def: 0x33 },
    reg_default { reg: 0xc007, def: 0x64 }, reg_default { reg: 0xc008, def: 0x05 },
    reg_default { reg: 0xc00a, def: 0xfc }, reg_default { reg: 0xc00b, def: 0x0f },
    reg_default { reg: 0xc00c, def: 0x0e }, reg_default { reg: 0xc00d, def: 0xef },
    reg_default { reg: 0xc00e, def: 0xe5 }, reg_default { reg: 0xc00f, def: 0xff },
    reg_default { reg: 0xc120, def: 0xc0 }, reg_default { reg: 0xc121, def: 0x00 },
    reg_default { reg: 0xc122, def: 0x00 }, reg_default { reg: 0xc123, def: 0x14 },
    reg_default { reg: 0xc125, def: 0x00 }, reg_default { reg: 0xc200, def: 0x00 },
    reg_default { reg: 0xc201, def: 0x00 }, reg_default { reg: 0xc202, def: 0x00 },
    reg_default { reg: 0xc203, def: 0x04 }, reg_default { reg: 0xc204, def: 0x00 },
    reg_default { reg: 0xc205, def: 0x00 }, reg_default { reg: 0xc206, def: 0x68 },
    reg_default { reg: 0xc207, def: 0x70 }, reg_default { reg: 0xc208, def: 0x00 },
    reg_default { reg: 0xc20a, def: 0x00 }, reg_default { reg: 0xc20b, def: 0x01 },
    reg_default { reg: 0xc20c, def: 0x7f }, reg_default { reg: 0xc20d, def: 0x01 },
    reg_default { reg: 0xc20e, def: 0x7f }, reg_default { reg: 0xc300, def: 0x00 },
    reg_default { reg: 0xc301, def: 0x00 }, reg_default { reg: 0xc303, def: 0x80 },
    reg_default { reg: 0xc320, def: 0x00 }, reg_default { reg: 0xc321, def: 0x09 },
    reg_default { reg: 0xc322, def: 0x02 }, reg_default { reg: 0xc410, def: 0x04 },
    reg_default { reg: 0xc430, def: 0x00 }, reg_default { reg: 0xc431, def: 0x00 },
    reg_default { reg: 0xca00, def: 0x10 }, reg_default { reg: 0xca01, def: 0x00 },
    reg_default { reg: 0xca02, def: 0x0b }, reg_default { reg: 0xca10, def: 0x10 },
    reg_default { reg: 0xca11, def: 0x00 }, reg_default { reg: 0xca12, def: 0x0b },
    reg_default { reg: 0xdd93, def: 0x00 }, reg_default { reg: 0xdd94, def: 0x64 },
    reg_default { reg: 0xe300, def: 0xa0 }, reg_default { reg: 0xed00, def: 0x80 },
    reg_default { reg: 0xed01, def: 0x0f }, reg_default { reg: 0xed02, def: 0xff },
    reg_default { reg: 0xed03, def: 0x00 }, reg_default { reg: 0xed04, def: 0x00 },
    reg_default { reg: 0xed05, def: 0x0f }, reg_default { reg: 0xed06, def: 0xff },
    reg_default { reg: 0xf010, def: 0x10 }, reg_default { reg: 0xf011, def: 0xec },
    reg_default { reg: 0xf012, def: 0x68 }, reg_default { reg: 0xf013, def: 0x21 },
    reg_default { reg: 0xf800, def: 0x00 }, reg_default { reg: 0xf801, def: 0x12 },
    reg_default { reg: 0xf802, def: 0xe0 }, reg_default { reg: 0xf803, def: 0x2f },
    reg_default { reg: 0xf804, def: 0x00 }, reg_default { reg: 0xf805, def: 0x00 },
    reg_default { reg: 0xf806, def: 0x07 }, reg_default { reg: 0xf807, def: 0xff },
    reg_default { reg: unsafe { SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1318_SDCA_ENT_FU21, RT1318_SDCA_CTL_FU_MUTE, CH_L) }, def: 0x01 },
    reg_default { reg: unsafe { SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1318_SDCA_ENT_FU21, RT1318_SDCA_CTL_FU_MUTE, CH_R) }, def: 0x01 },
    reg_default { reg: unsafe { SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1318_SDCA_ENT_PDE23, RT1318_SDCA_CTL_REQ_POWER_STATE, 0) }, def: 0x03 },
    reg_default { reg: unsafe { SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1318_SDCA_ENT_UDMPU21, RT1318_SDCA_CTL_UDMPU_CLUSTER, 0) }, def: 0x00 },
    reg_default { reg: unsafe { SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1318_SDCA_ENT_CS21, RT1318_SDCA_CTL_SAMPLE_FREQ_INDEX, 0) }, def: 0x09 },
];

unsafe extern "C" fn rt1318_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        0x2f55 | 0x3000 | 0x3206 | 0xc410 | 0xe300 => true,
        0x3004..=0x3005 | 0x3202..=0x3203 | 0xc000..=0xc00f |
        0xc120..=0xc125 | 0xc200..=0xc20e | 0xc300..=0xc303 |
        0xc320..=0xc322 | 0xc430..=0xc431 | 0xca00..=0xca02 |
        0xca10..=0xca12 | 0xcb00..=0xcb0b | 0xcc00..=0xcce5 |
        0xcd00..=0xcde5 | 0xce00..=0xce6a | 0xcf00..=0xcf53 |
        0xd000..=0xd0cc | 0xd100..=0xd1b9 | 0xdb00..=0xdc53 |
        0xdd00..=0xde53 | 0xdf00..=0xdf6b | 0xeb00..=0xebcc |
        0xec00..=0xecb9 | 0xed00..=0xed06 | 0xf010..=0xf014 |
        0xf800..=0xf807 => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1318_SDCA_ENT_UDMPU21, RT1318_SDCA_CTL_UDMPU_CLUSTER, 0) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1318_SDCA_ENT_FU21, RT1318_SDCA_CTL_FU_MUTE, CH_L) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1318_SDCA_ENT_FU21, RT1318_SDCA_CTL_FU_MUTE, CH_R) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1318_SDCA_ENT_PDE23, RT1318_SDCA_CTL_REQ_POWER_STATE, 0) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1318_SDCA_ENT_CS21, RT1318_SDCA_CTL_SAMPLE_FREQ_INDEX, 0) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1318_SDCA_ENT_SAPU, RT1318_SDCA_CTL_SAPU_PROTECTION_MODE, 0) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1318_SDCA_ENT_SAPU, RT1318_SDCA_CTL_SAPU_PROTECTION_STATUS, 0) => true,
        _ => false,
    }
}

unsafe extern "C" fn rt1318_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        0x2f55 | 0xc000 | 0xc301 | 0xc410 | 0xdb06 | 0xdb12 | 0xdb35 |
        0xdb37 | 0xdd0b | 0xdd12 | 0xdd35 | 0xea01 | 0xebc5 | 0xebc8 => true,
        0x3000..=0x3001 | 0xc430..=0xc431 | 0xdb1d..=0xdb1f |
        0xdb8a..=0xdb92 | 0xdbc5..=0xdbc8 | 0xdc2b..=0xdc49 |
        0xdd1d..=0xdd1f | 0xdd8a..=0xdd92 | 0xddc5..=0xddc8 |
        0xde2b..=0xde44 | 0xdf4a..=0xdf55 | 0xe224..=0xe23b |
        0xebcb..=0xebcc | 0xed03..=0xed06 | 0xf010..=0xf014 => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1318_SDCA_ENT_SAPU, RT1318_SDCA_CTL_SAPU_PROTECTION_MODE, 0) => true,
        x if x == SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1318_SDCA_ENT_SAPU, RT1318_SDCA_CTL_SAPU_PROTECTION_STATUS, 0) => true,
        _ => false,
    }
}

static rt1318_sdw_regmap: regmap_config = regmap_config {
    reg_bits: 32,
    val_bits: 8,
    readable_reg: Some(rt1318_readable_register),
    volatile_reg: Some(rt1318_volatile_register),
    max_register: 0x41081488,
    reg_defaults: rt1318_reg_defaults.as_ptr(),
    num_reg_defaults: ARRAY_SIZE(&rt1318_reg_defaults) as c_uint,
    cache_type: REGCACHE_MAPLE,
    use_single_read: true,
    use_single_write: true,
};

unsafe extern "C" fn rt1318_read_prop(slave: *mut sdw_slave) -> c_int {
    let prop = &mut (*slave).prop;
    let mut i: c_int;
    let mut j: c_int;
    let mut bit: u32;
    let mut addr: c_ulong;
    let mut dpn: *mut sdw_dpn_prop;

    prop.scp_int1_mask = SDW_SCP_INT1_BUS_CLASH | SDW_SCP_INT1_PARITY;
    prop.quirks = SDW_SLAVE_QUIRKS_INVALID_INITIAL_PARITY;
    prop.paging_support = true;

    /* first we need to allocate memory for set bits in port lists */
    prop.source_ports = BIT(2);
    prop.sink_ports = BIT(1);

    let mut nval = prop.source_ports.count_ones() as c_int;
    prop.src_dpn_prop = devm_kcalloc(&mut (*slave).dev, nval as usize,
        size_of::<sdw_dpn_prop>(), GFP_KERNEL) as *mut sdw_dpn_prop;
    if prop.src_dpn_prop.is_null() {
        return -ENOMEM;
    }

    i = 0;
    dpn = prop.src_dpn_prop;
    addr = prop.source_ports as c_ulong;
    bit = 0;
    while bit < 32 {
        if (addr & (1 as c_ulong).wrapping_shl(bit)) != 0 {
            (*dpn.offset(i as isize)).num = bit;
            (*dpn.offset(i as isize)).type_ = SDW_DPN_FULL;
            (*dpn.offset(i as isize)).simple_ch_prep_sm = true;
            (*dpn.offset(i as isize)).ch_prep_timeout = 10;
            i += 1;
        }
        bit += 1;
    }

    /* do this again for sink now */
    nval = prop.sink_ports.count_ones() as c_int;
    prop.sink_dpn_prop = devm_kcalloc(&mut (*slave).dev, nval as usize,
        size_of::<sdw_dpn_prop>(), GFP_KERNEL) as *mut sdw_dpn_prop;
    if prop.sink_dpn_prop.is_null() {
        return -ENOMEM;
    }

    j = 0;
    dpn = prop.sink_dpn_prop;
    addr = prop.sink_ports as c_ulong;
    bit = 0;
    while bit < 32 {
        if (addr & (1 as c_ulong).wrapping_shl(bit)) != 0 {
            (*dpn.offset(j as isize)).num = bit;
            (*dpn.offset(j as isize)).type_ = SDW_DPN_FULL;
            (*dpn.offset(j as isize)).simple_ch_prep_sm = true;
            (*dpn.offset(j as isize)).ch_prep_timeout = 10;
            j += 1;
        }
        bit += 1;
    }

    /* set the timeout values */
    prop.clk_stop_timeout = 20;

    0
}

unsafe extern "C" fn rt1318_io_init(dev: *mut device, slave: *mut sdw_slave) -> c_int {
    let rt1318 = dev_get_drvdata(dev) as *mut rt1318_sdw_priv;

    if (*rt1318).hw_init {
        return 0;
    }

    regcache_cache_only((*rt1318).regmap, false);
    if (*rt1318).first_hw_init {
        regcache_cache_bypass((*rt1318).regmap, true);
    } else {
        /*
         * PM runtime status is marked as 'active' only when a Slave reports as Attached
         */
        /* update count of parent 'active' children */
        pm_runtime_set_active(&mut (*slave).dev);
    }

    pm_runtime_get_noresume(&mut (*slave).dev);

    /* blind write */
    regmap_multi_reg_write((*rt1318).regmap, rt1318_blind_write.as_ptr(),
        ARRAY_SIZE(&rt1318_blind_write) as c_int);

    if (*rt1318).first_hw_init {
        regcache_cache_bypass((*rt1318).regmap, false);
        regcache_mark_dirty((*rt1318).regmap);
    }

    /* Mark Slave initialization complete */
    (*rt1318).first_hw_init = true;
    (*rt1318).hw_init = true;

    pm_runtime_put_autosuspend(&mut (*slave).dev);

    0
}

unsafe extern "C" fn rt1318_update_status(slave: *mut sdw_slave, status: sdw_slave_status) -> c_int {
    let rt1318 = dev_get_drvdata(&mut (*slave).dev) as *mut rt1318_sdw_priv;

    if status == SDW_SLAVE_UNATTACHED {
        (*rt1318).hw_init = false;
    }

    /*
     * Perform initialization only if slave status is present and
     * hw_init flag is false
     */
    if (*rt1318).hw_init || status != SDW_SLAVE_ATTACHED {
        return 0;
    }

    /* perform I/O transfers required for Slave initialization */
    rt1318_io_init(&mut (*slave).dev, slave)
}

unsafe extern "C" fn rt1318_classd_event(w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt1318 = snd_soc_component_get_drvdata(component) as *mut rt1318_sdw_priv;
    let ps0: u8 = 0x0;
    let ps3: u8 = 0x3;

    match event {
        SND_SOC_DAPM_POST_PMU => {
            regmap_write((*rt1318).regmap,
                SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1318_SDCA_ENT_PDE23,
                    RT1318_SDCA_CTL_REQ_POWER_STATE, 0),
                ps0 as c_uint);
        }
        SND_SOC_DAPM_PRE_PMD => {
            regmap_write((*rt1318).regmap,
                SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1318_SDCA_ENT_PDE23,
                    RT1318_SDCA_CTL_REQ_POWER_STATE, 0),
                ps3 as c_uint);
        }
        _ => {}
    }

    0
}

static rt1318_rx_data_ch_select: [*const c_char; 10] = [
    b"L,R\0".as_ptr() as *const c_char,
    b"L,L\0".as_ptr() as *const c_char,
    b"L,R\0".as_ptr() as *const c_char,
    b"L,L+R\0".as_ptr() as *const c_char,
    b"R,L\0".as_ptr() as *const c_char,
    b"R,R\0".as_ptr() as *const c_char,
    b"R,L+R\0".as_ptr() as *const c_char,
    b"L+R,L\0".as_ptr() as *const c_char,
    b"L+R,R\0".as_ptr() as *const c_char,
    b"L+R,L+R\0".as_ptr() as *const c_char,
];

// static SOC_ENUM_SINGLE_DECL(rt1318_rx_data_ch_enum,
//     SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1318_SDCA_ENT_UDMPU21, RT1318_SDCA_CTL_UDMPU_CLUSTER, 0), 0,
//     rt1318_rx_data_ch_select);
// static const struct snd_kcontrol_new rt1318_snd_controls[] = {
//     /* UDMPU Cluster Selection */
//     SOC_ENUM("RX Channel Select", rt1318_rx_data_ch_enum),
// };
static rt1318_snd_controls: [snd_kcontrol_new; 1] = [
    unsafe { core::mem::zeroed() },
];

// static const struct snd_kcontrol_new rt1318_sto_dac =
//     SOC_DAPM_DOUBLE_R("Switch",
//         SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1318_SDCA_ENT_FU21, RT1318_SDCA_CTL_FU_MUTE, CH_L),
//         SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1318_SDCA_ENT_FU21, RT1318_SDCA_CTL_FU_MUTE, CH_R),
//         0, 1, 1);
static rt1318_sto_dac: snd_kcontrol_new = unsafe { core::mem::zeroed() };

// static const struct snd_soc_dapm_widget rt1318_dapm_widgets[] = {
//     /* Audio Interface */
//     SND_SOC_DAPM_AIF_IN("DP1RX", "DP1 Playback", 0, SND_SOC_NOPM, 0, 0),
//     SND_SOC_DAPM_AIF_OUT("DP2TX", "DP2 Capture", 0, SND_SOC_NOPM, 0, 0),
//     /* Digital Interface */
//     SND_SOC_DAPM_SWITCH("DAC", SND_SOC_NOPM, 0, 0, &rt1318_sto_dac),
//     /* Output */
//     SND_SOC_DAPM_PGA_E("CLASS D", SND_SOC_NOPM, 0, 0, NULL, 0,
//         rt1318_classd_event, SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMU),
//     SND_SOC_DAPM_OUTPUT("SPOL"),
//     SND_SOC_DAPM_OUTPUT("SPOR"),
//     /* Input */
//     SND_SOC_DAPM_PGA("FB Data", SND_SOC_NOPM, 0, 0, NULL, 0),
//     SND_SOC_DAPM_SIGGEN("FB Gen"),
// };
static rt1318_dapm_widgets: [snd_soc_dapm_widget_def; 8] = unsafe { core::mem::zeroed() };

static rt1318_dapm_routes: [snd_soc_dapm_route; 6] = [
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: b"Switch\0".as_ptr() as *const c_char, source: b"DP1RX\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"CLASS D\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DAC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SPOL\0".as_ptr() as *const c_char, control: ptr::null(), source: b"CLASS D\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SPOR\0".as_ptr() as *const c_char, control: ptr::null(), source: b"CLASS D\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"FB Data\0".as_ptr() as *const c_char, control: ptr::null(), source: b"FB Gen\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DP2TX\0".as_ptr() as *const c_char, control: ptr::null(), source: b"FB Data\0".as_ptr() as *const c_char },
];

unsafe extern "C" fn rt1318_set_sdw_stream(dai: *mut snd_soc_dai, sdw_stream: *mut c_void,
    direction: c_int) -> c_int {
    snd_soc_dai_dma_data_set(dai, direction, sdw_stream);
    0
}

unsafe extern "C" fn rt1318_sdw_shutdown(substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai) {
    snd_soc_dai_set_dma_data(dai, substream, ptr::null_mut());
}

unsafe extern "C" fn rt1318_sdw_hw_params(substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let rt1318 = snd_soc_component_get_drvdata(component) as *mut rt1318_sdw_priv;
    let mut stream_config: sdw_stream_config = core::mem::zeroed();
    let mut port_config: sdw_port_config = core::mem::zeroed();
    let direction: sdw_data_direction;
    let sdw_stream: *mut sdw_stream_runtime;
    let retval: c_int;
    let port: c_int;
    let num_channels: c_int;
    let ch_mask: c_int;
    let sampling_rate: c_uint;

    sdw_stream = snd_soc_dai_get_dma_data(dai, substream);
    if sdw_stream.is_null() {
        return -EINVAL;
    }

    if (*rt1318).sdw_slave.is_null() {
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
    ch_mask = (1_i32 << num_channels) - 1;

    stream_config.frame_rate = params_rate(params);
    stream_config.ch_count = num_channels;
    stream_config.bps = snd_pcm_format_width(params_format(params));
    stream_config.direction = direction;

    port_config.ch_mask = ch_mask;
    port_config.num = port;

    retval = sdw_stream_add_slave((*rt1318).sdw_slave, &mut stream_config,
        &mut port_config, 1, sdw_stream);
    if retval != 0 {
        return retval;
    }

    /* sampling rate configuration */
    match params_rate(params) {
        16000 => sampling_rate = RT1318_SDCA_RATE_16000HZ,
        32000 => sampling_rate = RT1318_SDCA_RATE_32000HZ,
        44100 => sampling_rate = RT1318_SDCA_RATE_44100HZ,
        48000 => sampling_rate = RT1318_SDCA_RATE_48000HZ,
        96000 => sampling_rate = RT1318_SDCA_RATE_96000HZ,
        192000 => sampling_rate = RT1318_SDCA_RATE_192000HZ,
        _ => return -EINVAL,
    }

    /* set sampling frequency */
    regmap_write((*rt1318).regmap,
        SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, RT1318_SDCA_ENT_CS21, RT1318_SDCA_CTL_SAMPLE_FREQ_INDEX, 0),
        sampling_rate);

    0
}

unsafe extern "C" fn rt1318_sdw_pcm_hw_free(substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let rt1318 = snd_soc_component_get_drvdata(component) as *mut rt1318_sdw_priv;
    let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);

    if (*rt1318).sdw_slave.is_null() {
        return -EINVAL;
    }

    sdw_stream_remove_slave((*rt1318).sdw_slave, sdw_stream);
    0
}

/*
 * slave_ops: callbacks for get_clock_stop_mode, clock_stop and
 * port_prep are not defined for now
 */
static rt1318_slave_ops: sdw_slave_ops = sdw_slave_ops {
    read_prop: Some(rt1318_read_prop),
    update_status: Some(rt1318_update_status),
};

unsafe extern "C" fn rt1318_sdw_component_probe(component: *mut snd_soc_component) -> c_int {
    let ret: c_int;
    let rt1318 = snd_soc_component_get_drvdata(component) as *mut rt1318_sdw_priv;

    (*rt1318).component = component;

    if !(*rt1318).first_hw_init {
        return 0;
    }

    ret = pm_runtime_resume((*component).dev);
    if ret < 0 && ret != -EACCES {
        return ret;
    }

    0
}

static soc_component_sdw_rt1318: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(rt1318_sdw_component_probe),
    controls: rt1318_snd_controls.as_ptr(),
    num_controls: ARRAY_SIZE(&rt1318_snd_controls) as c_uint,
    dapm_widgets: rt1318_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE(&rt1318_dapm_widgets) as c_uint,
    dapm_routes: rt1318_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE(&rt1318_dapm_routes) as c_uint,
    endianness: 1,
};

static rt1318_aif_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(rt1318_sdw_hw_params),
    hw_free: Some(rt1318_sdw_pcm_hw_free),
    set_stream: Some(rt1318_set_sdw_stream),
    shutdown: Some(rt1318_sdw_shutdown),
};

const RT1318_STEREO_RATES: c_uint = SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 |
    SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000;
const RT1318_FORMATS: c_ulong = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE |
    SNDRV_PCM_FMTBIT_S32_LE;

static mut rt1318_sdw_dai: [snd_soc_dai_driver; 1] = [
    snd_soc_dai_driver {
        name: b"rt1318-aif\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream {
            stream_name: b"DP1 Playback\0".as_ptr() as *const c_char,
            channels_min: 1,
            channels_max: 2,
            rates: RT1318_STEREO_RATES,
            formats: RT1318_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: b"DP2 Capture\0".as_ptr() as *const c_char,
            channels_min: 1,
            channels_max: 2,
            rates: RT1318_STEREO_RATES,
            formats: RT1318_FORMATS,
        },
        ops: &rt1318_aif_dai_ops,
    },
];

unsafe extern "C" fn rt1318_sdw_init(dev: *mut device, regmap: *mut regmap,
    slave: *mut sdw_slave) -> c_int {
    let rt1318: *mut rt1318_sdw_priv;
    let ret: c_int;

    rt1318 = devm_kzalloc(dev, size_of::<rt1318_sdw_priv>(), GFP_KERNEL) as *mut rt1318_sdw_priv;
    if rt1318.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(dev, rt1318 as *mut c_void);
    (*rt1318).sdw_slave = slave;
    (*rt1318).regmap = regmap;

    regcache_cache_only((*rt1318).regmap, true);

    /*
     * Mark hw_init to false
     * HW init will be performed when device reports present
     */
    (*rt1318).hw_init = false;
    (*rt1318).first_hw_init = false;

    ret = devm_snd_soc_register_component(dev,
        &soc_component_sdw_rt1318,
        rt1318_sdw_dai.as_mut_ptr(),
        ARRAY_SIZE(&rt1318_sdw_dai) as c_int);
    if ret < 0 {
        return ret;
    }

    /* set autosuspend parameters */
    pm_runtime_set_autosuspend_delay(dev, 3000);
    pm_runtime_use_autosuspend(dev);

    /* make sure the device does not suspend immediately */
    pm_runtime_mark_last_busy(dev);

    pm_runtime_enable(dev);

    /* important note: the device is NOT tagged as 'active' and will remain
     * 'suspended' until the hardware is enumerated/initialized. This is required
     * to make sure the ASoC framework use of pm_runtime_get_sync() does not silently
     * fail with -EACCESS because of race conditions between card creation and enumeration
     */

    ret
}

unsafe extern "C" fn rt1318_sdw_probe(slave: *mut sdw_slave,
    _id: *const sdw_device_id) -> c_int {
    let regmap: *mut regmap;

    /* Regmap Initialization */
    regmap = devm_regmap_init_sdw(slave, &rt1318_sdw_regmap);
    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }

    rt1318_sdw_init(&mut (*slave).dev, regmap, slave)
}

unsafe extern "C" fn rt1318_sdw_remove(slave: *mut sdw_slave) {
    pm_runtime_disable(&mut (*slave).dev);
}

// static const struct sdw_device_id rt1318_id[] = {
//     SDW_SLAVE_ENTRY_EXT(0x025d, 0x1318, 0x3, 0x1, 0),
//     {},
// };
// MODULE_DEVICE_TABLE(sdw, rt1318_id);
static rt1318_id: [sdw_device_id; 2] = unsafe { core::mem::zeroed() };

unsafe extern "C" fn rt1318_dev_suspend(dev: *mut device) -> c_int {
    let rt1318 = dev_get_drvdata(dev) as *mut rt1318_sdw_priv;

    if !(*rt1318).hw_init {
        return 0;
    }

    regcache_cache_only((*rt1318).regmap, true);
    0
}

const RT1318_PROBE_TIMEOUT: c_int = 5000;

unsafe extern "C" fn rt1318_dev_resume(dev: *mut device) -> c_int {
    let slave = dev_to_sdw_dev(dev);
    let rt1318 = dev_get_drvdata(dev) as *mut rt1318_sdw_priv;
    let mut ret: c_int;

    if !(*rt1318).first_hw_init {
        return 0;
    }

    ret = sdw_slave_wait_for_init(slave, RT1318_PROBE_TIMEOUT);
    if ret != 0 {
        return ret;
    }

    regcache_cache_only((*rt1318).regmap, false);
    ret = regcache_sync((*rt1318).regmap);
    if ret != 0 {
        regcache_cache_only((*rt1318).regmap, true);
        regcache_mark_dirty((*rt1318).regmap);
        return ret;
    }

    0
}

// static const struct dev_pm_ops rt1318_pm = {
//     SYSTEM_SLEEP_PM_OPS(rt1318_dev_suspend, rt1318_dev_resume)
//     RUNTIME_PM_OPS(rt1318_dev_suspend, rt1318_dev_resume, NULL)
// };
static rt1318_pm: dev_pm_ops = unsafe { core::mem::zeroed() };

static mut rt1318_sdw_driver: sdw_driver = sdw_driver {
    driver: sdw_driver_inner {
        name: b"rt1318-sdca\0".as_ptr() as *const c_char,
        pm: &rt1318_pm,
    },
    probe: Some(rt1318_sdw_probe),
    remove: Some(rt1318_sdw_remove),
    ops: &rt1318_slave_ops,
    id_table: rt1318_id.as_ptr(),
};

// module_sdw_driver(rt1318_sdw_driver);
// MODULE_DESCRIPTION("ASoC RT1318 driver SDCA SDW");
// MODULE_AUTHOR("Shuming Fan <shumingf@realtek.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
