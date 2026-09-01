// SPDX-License-Identifier: GPL-2.0
//
// ALSA SoC Texas Instruments TAC5XX2 Audio Smart Amplifier
//
// Copyright (C) 2025 Texas Instruments Incorporated
// https://www.ti.com
//
// Author: Niranjan H Y <niranjan.hy@ti.com>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type u16 = u16;
type u32 = u32;
type u64 = u64;
type s32 = i32;
type size_t = usize;
type bool_ = bool;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct firmware {
    pub size: size_t,
    pub data: *const u8,
}
#[repr(C)]
pub struct pci_dev {
    pub dev: device,
    pub subsystem_device: u16,
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
    pub dev: *mut device,
    pub id: c_int,
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
pub struct snd_soc_jack {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sdw_bus {
    pub link_id: u8,
}
#[repr(C)]
pub struct sdw_slave_id {
    pub part_id: u16,
    pub unique_id: u8,
}
#[repr(C)]
pub struct sdca_function_desc {
    pub type_: c_uint,
}
#[repr(C)]
pub struct sdca_data {
    pub num_functions: c_int,
    pub function: *mut sdca_function_desc,
}
#[repr(C)]
pub struct sdw_slave {
    pub dev: device,
    pub bus: *mut sdw_bus,
    pub id: sdw_slave_id,
    pub sdca_data: sdca_data,
}
#[repr(C)]
pub struct sdca_function_data {
    pub desc: *mut sdca_function_desc,
}
#[repr(C)]
pub struct sdw_slave_intr_status {
    pub control_port: c_uint,
}
#[repr(C)]
pub struct sdw_stream_runtime {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sdw_stream_config {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sdw_port_config {
    pub num: c_int,
}
#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}
#[repr(C)]
pub struct tm {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_int,
}
#[repr(C)]
pub struct sdw_device_id {
    pub mfg_id: u16,
    pub part_id: u16,
    pub sdw_version: u8,
}
#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}
#[repr(C)]
pub struct reg_sequence {
    pub reg: c_uint,
    pub def: c_uint,
    pub delay_us: c_uint,
}
#[repr(C)]
pub struct regmap_sdw_mbq_cfg {
    pub mbq_size: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
}
#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_int,
    pub val_bits: c_int,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub max_register: c_uint,
    pub cache_type: c_int,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub use_single_read: bool,
    pub use_single_write: bool,
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
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub set_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, *mut c_void, s32) -> s32>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
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
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> s32>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub idle_bias_on: c_uint,
    pub endianness: c_uint,
    pub set_jack: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_jack, *mut c_void) -> c_int>,
}
#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sdw_prepare_ch {
    _private: [u8; 0],
}
type sdw_port_prep_ops = c_int;
#[repr(C)]
pub struct sdw_slave_ops {
    pub read_prop: Option<unsafe extern "C" fn(*mut sdw_slave) -> c_int>,
    pub update_status: Option<unsafe extern "C" fn(*mut sdw_slave, sdw_slave_status) -> c_int>,
    pub interrupt_callback: Option<unsafe extern "C" fn(*mut sdw_slave, *mut sdw_slave_intr_status) -> c_int>,
    pub port_prep: Option<unsafe extern "C" fn(*mut sdw_slave, *mut sdw_prepare_ch, sdw_port_prep_ops) -> c_int>,
}
#[repr(C)]
pub struct sdw_driver_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}
#[repr(C)]
pub struct sdw_driver {
    pub driver: sdw_driver_driver,
    pub probe: Option<unsafe extern "C" fn(*mut sdw_slave, *const sdw_device_id) -> s32>,
    pub remove: Option<unsafe extern "C" fn(*mut sdw_slave)>,
    pub ops: *const sdw_slave_ops,
    pub id_table: *const sdw_device_id,
}
type sdw_slave_status = c_int;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EACCES: c_int = 13;
const ETIMEDOUT: c_int = 110;
const GFP_KERNEL: c_uint = 0;
const FW_ACTION_UEVENT: c_uint = 0;
const REGCACHE_MAPLE: c_int = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SDW_SLAVE_UNATTACHED: sdw_slave_status = 0;
const SDW_SLAVE_ATTACHED: sdw_slave_status = 1;
const SDW_OPS_PORT_POST_PREP: sdw_port_prep_ops = 1;
const SDCA_UMP_OWNER_DEVICE: c_int = 1;
const SDCA_PDE_PS0: c_int = 0;
const SDCA_PDE_PS3: c_int = 3;
const THIS_MODULE: *mut c_void = ptr::null_mut();

const TAC5XX2_PROBE_TIMEOUT_MS: c_int = 3000;
const TAC5XX2_FW_CACHE_TIMEOUT_MS: c_int = 300;
const TAC5XX2_DEVICE_RATES: c_uint = SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_88200;
const TAC5XX2_DEVICE_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;
/* Define channel constants */
const TAC_CHANNEL_LEFT: c_uint = 1;
const TAC_CHANNEL_RIGHT: c_uint = 2;
const TAC_JACK_MONO_CS: c_uint = 2;
const TAC_DSP_ALGO_STATUS_RUNNING: c_uint = 0x20;
const TAC_FW_HDR_SIZE: usize = 88;
const TAC_FW_FILE_HDR: usize = 20;
const TAC_MAX_FW_CHUNKS: usize = 512;
const TAC_UAJ_PREP_CONNECTED: u8 = 0xff;
const TAC_UAJ_PREP_DISCONNECTED: u8 = 0xdf;
/* Q7.8 volume control parameters: range -72dB to +6dB, step 0.5dB */
const TAC_DVC_STEP: c_int = 128; /* 0.5 dB in Q7.8 format */
const TAC_DVC_MIN: c_int = -144; /* -72 dB / 0.5 dB step */
const TAC_DVC_MAX: c_int = 12; /* +6 dB / 0.5 dB step */

macro_rules! c_str {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}
macro_rules! ARRAY_SIZE {
    ($a:expr) => {
        $a.len() as c_uint
    };
}
macro_rules! TAC_REG_SDW {
    ($a:expr, $b:expr, $c:expr) => {
        tac_reg_sdw($a, $b, $c)
    };
}
macro_rules! SDW_SDCA_CTL {
    ($func:expr, $ent:expr, $ctl:expr, $ch:expr) => {
        sdw_sdca_ctl($func, $ent, $ctl, $ch)
    };
}
macro_rules! TAC_MUTE_REG {
    ($func:ident, $fu:ident, $ch:ident) => {
        SDW_SDCA_CTL!(concat_id_tac_function_id!($func), concat_id_tac_sdca_ent!($fu), TAC_SDCA_CHANNEL_MUTE, concat_id_tac_channel!($ch))
    };
}
macro_rules! TAC_USAGE_REG {
    ($func:ident, $ent:ident) => {
        SDW_SDCA_CTL!(concat_id_tac_function_id!($func), concat_id_tac_sdca_ent!($ent), TAC_SDCA_CTL_USAGE, 0)
    };
}
macro_rules! TAC_XU_BYPASS_REG {
    ($func:ident, $xu:ident) => {
        SDW_SDCA_CTL!(concat_id_tac_function_id!($func), concat_id_tac_sdca_ent!($xu), TAC_SDCA_CTL_XU_BYPASS, 0)
    };
}
macro_rules! TAC_VOLUME_REG {
    ($func:ident, $fu:ident, $ch:ident) => {
        SDW_SDCA_CTL!(concat_id_tac_function_id!($func), concat_id_tac_sdca_ent!($fu), TAC_SDCA_CHANNEL_VOLUME, concat_id_tac_channel!($ch))
    };
}
macro_rules! TAC_GAIN_REG {
    ($func:ident, $fu:ident, $ch:ident) => {
        SDW_SDCA_CTL!(concat_id_tac_function_id!($func), concat_id_tac_sdca_ent!($fu), TAC_SDCA_CHANNEL_GAIN, concat_id_tac_channel!($ch))
    };
}
macro_rules! concat_id_tac_function_id {
    (SA) => { TAC_FUNCTION_ID_SA };
    (SM) => { TAC_FUNCTION_ID_SM };
    (UAJ) => { TAC_FUNCTION_ID_UAJ };
    (HID) => { TAC_FUNCTION_ID_HID };
}
macro_rules! concat_id_tac_sdca_ent {
    ($id:ident) => { $id };
}
macro_rules! concat_id_tac_channel {
    (LEFT) => { TAC_CHANNEL_LEFT };
    (RIGHT) => { TAC_CHANNEL_RIGHT };
}
macro_rules! REG_SEQ0 {
    ($reg:expr, $def:expr) => {
        reg_sequence { reg: $reg, def: $def, delay_us: 0 }
    };
}

#[repr(C)]
struct tac_fw_hdr {
    size: u32,
    version_offset: u32,
    plt_id: u32,
    ppc3_ver: u32,
    timestamp: u64,
    ddc_name: [u8; 64],
}

/* Firmware file/chunk structure */
#[repr(C)]
struct tac_fw_file {
    vendor_id: u32,
    file_id: u32,
    version: u32,
    length: u32,
    dest_addr: u32,
    fw_data: *mut u8,
}

#[repr(C)]
struct tac5xx2_prv {
    component: *mut snd_soc_component,
    sdw_peripheral: *mut sdw_slave,
    sa_func_data: *mut sdca_function_data,
    sm_func_data: *mut sdca_function_data,
    uaj_func_data: *mut sdca_function_data,
    hid_func_data: *mut sdca_function_data,
    status: sdw_slave_status,
    regmap: *mut regmap,
    dev: *mut device,
    hw_init: bool,
    first_hw_init_done: bool,
    part_id: u32,
    rev_id: u32,
    hs_jack: *mut snd_soc_jack,
    jack_type: c_int,
    /* Custom fw binary. UMP File Download is not used. */
    fw_file_cnt: c_uint,
    fw_files: *mut tac_fw_file,
    fw_caching_complete: completion,
    fw_dl_success: bool,
    fw_binaryname: [u8; 64],
}

extern "C" {
    static tac5xx2_amp_tlv: [c_uint; 0];
    static tac5xx2_dvc_tlv: [c_uint; 0];
    static SNDRV_PCM_RATE_44100: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_RATE_88200: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static TAC_SW_RESET: c_uint;
    static TAC_SLEEP_MODEZ: c_uint;
    static TAC_FEATURE_PDZ: c_uint;
    static TAC_TX_CH_EN: c_uint;
    static TAC_AMP_LVL_CFG0: c_uint;
    static TAC_AMP_LVL_CFG1: c_uint;
    static TAC_INT_CFG: c_uint;
    static TAC_INT_CFG_CLR_REG: c_uint;
    static TAC_REV_ID: c_uint;
    static TAC_BUF_ADDR_HID1: c_uint;
    static TAC_FUNCTION_ID_SA: c_uint;
    static TAC_FUNCTION_ID_SM: c_uint;
    static TAC_FUNCTION_ID_UAJ: c_uint;
    static TAC_FUNCTION_ID_HID: c_uint;
    static TAC_SDCA_ENT_FU21: c_uint;
    static TAC_SDCA_ENT_FU23: c_uint;
    static TAC_SDCA_ENT_FU26: c_uint;
    static TAC_SDCA_ENT_FU11: c_uint;
    static TAC_SDCA_ENT_FU113: c_uint;
    static TAC_SDCA_ENT_FU41: c_uint;
    static TAC_SDCA_ENT_FU36: c_uint;
    static TAC_SDCA_ENT_IT11: c_uint;
    static TAC_SDCA_ENT_IT41: c_uint;
    static TAC_SDCA_ENT_IT33: c_uint;
    static TAC_SDCA_ENT_OT113: c_uint;
    static TAC_SDCA_ENT_OT45: c_uint;
    static TAC_SDCA_ENT_OT36: c_uint;
    static TAC_SDCA_ENT_XU12: c_uint;
    static TAC_SDCA_ENT_XU42: c_uint;
    static TAC_SDCA_ENT_CS113: c_uint;
    static TAC_SDCA_ENT_CS36: c_uint;
    static TAC_SDCA_ENT_CS41: c_uint;
    static TAC_SDCA_ENT_HID1: c_uint;
    static TAC_SDCA_ENT_GE35: c_uint;
    static TAC_SDCA_ENT_PDE23: c_uint;
    static TAC_SDCA_ENT_PDE11: c_uint;
    static TAC_SDCA_ENT_PDE47: c_uint;
    static TAC_SDCA_ENT_PDE34: c_uint;
    static TAC_SDCA_CHANNEL_MUTE: c_uint;
    static TAC_SDCA_CHANNEL_VOLUME: c_uint;
    static TAC_SDCA_CHANNEL_GAIN: c_uint;
    static TAC_SDCA_CTL_USAGE: c_uint;
    static TAC_SDCA_CTL_XU_BYPASS: c_uint;
    static TAC_SDCA_CTL_CS_SAMP_RATE_IDX: c_uint;
    static TAC_SDCA_CTL_HIDTX_CURRENT_OWNER: c_uint;
    static TAC_SDCA_CTL_HIDTX_MESSAGE_OFFSET: c_uint;
    static TAC_SDCA_CTL_DET_MODE: c_uint;
    static TAC_SDCA_CTL_SEL_MODE: c_uint;
    static TAC_SDCA_REQUESTED_PS: c_uint;
    static TAC_SDCA_ACTUAL_PS: c_uint;
    static TAC5XX2_DMIC: c_int;
    static TAC5XX2_UAJ: c_int;
    static TAC5XX2_SPK: c_int;
    static TAC_SDW_PORT_NUM_DMIC: c_int;
    static TAC_SDW_PORT_NUM_UAJ_PLAYBACK: c_int;
    static TAC_SDW_PORT_NUM_UAJ_CAPTURE: c_int;
    static TAC_SDW_PORT_NUM_SPK_PLAYBACK: c_int;
    static TAC_SDW_PORT_NUM_SPK_CAPTURE: c_int;
    static SDW_SCP_SDCA_INT1: c_uint;
    static SDW_SCP_SDCA_INT2: c_uint;
    static SDW_SCP_SDCA_INT3: c_uint;
    static SDW_SCP_SDCA_INT4: c_uint;
    static SDW_SCP_SDCA_INTMASK2: c_uint;
    static SDW_SCP_SDCA_INTMASK3: c_uint;
    static SDW_SCP_SDCA_INTMASK_SDCA_11: u32;
    static SDW_SCP_SDCA_INTMASK_SDCA_12: u32;
    static SDW_SCP_SDCA_INTMASK_SDCA_16: u32;
    static SDW_SCP_SDCA_INTMASK_SDCA_17: u32;
    static SDW_SCP_SDCA_INT_SDCA_11: u32;
    static SDW_SCP_SDCA_INT_SDCA_12: u32;
    static SDW_SCP_SDCA_INT_SDCA_16: u32;
    static SDW_SCP_SDCA_INT_SDCA_17: u32;
    static SDW_SCP_INT1_PARITY: c_uint;
    static SDW_SCP_INT1_BUS_CLASH: c_uint;
    static SND_JACK_MICROPHONE: c_int;
    static SND_JACK_HEADPHONE: c_int;
    static SND_JACK_HEADSET: c_int;
    static SND_JACK_BTN_0: c_int;
    static SND_JACK_BTN_1: c_int;
    static SND_JACK_BTN_2: c_int;
    static SND_JACK_BTN_3: c_int;
    static SND_JACK_BTN_4: c_int;
    static SDCA_FUNCTION_TYPE_SMART_AMP: c_uint;
    static SDCA_FUNCTION_TYPE_SMART_MIC: c_uint;
    static SDCA_FUNCTION_TYPE_UAJ: c_uint;
    static SDCA_FUNCTION_TYPE_HID: c_uint;

    fn tac_reg_sdw(a: c_uint, b: c_uint, c: c_uint) -> c_uint;
    fn sdw_sdca_ctl(func: c_uint, ent: c_uint, ctl: c_uint, ch: c_uint) -> c_uint;
    fn get_unaligned_le32(p: *const u8) -> u32;
    fn get_unaligned_le64(p: *const u8) -> u64;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn FIELD_GET(mask: u32, reg: u32) -> u32;
    fn GENMASK(h: c_uint, l: c_uint) -> u32;
    fn time64_to_tm(t: u64, offset: c_int, tm: *mut tm);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn devm_kmemdup(dev: *mut device, src: *const c_void, len: size_t, flags: c_uint) -> *mut c_void;
    fn devm_kfree(dev: *mut device, p: *mut c_void);
    fn dev_is_pci(dev: *mut device) -> bool;
    fn to_pci_dev(dev: *mut device) -> *mut pci_dev;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_multi_reg_write(map: *mut regmap, regs: *const reg_sequence, num_regs: c_uint) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn devm_regmap_init_sdw_mbq_cfg(dev: *mut device, slave: *mut sdw_slave, cfg: *const regmap_config, mbq: *const regmap_sdw_mbq_cfg) -> *mut regmap;
    fn IS_ERR(p: *const c_void) -> bool;
    fn PTR_ERR(p: *const c_void) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dai_dma_data_set(dai: *mut snd_soc_dai, direction: s32, data: *mut c_void);
    fn snd_soc_dai_set_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream, data: *mut c_void);
    fn snd_soc_dai_get_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream) -> *mut sdw_stream_runtime;
    fn snd_sdw_params_to_config(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, stream: *mut sdw_stream_config, port: *mut sdw_port_config);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn sdw_stream_add_slave(slave: *mut sdw_slave, stream: *mut sdw_stream_config, port: *mut sdw_port_config, num_ports: c_int, runtime: *mut sdw_stream_runtime) -> c_int;
    fn sdw_stream_remove_slave(slave: *mut sdw_slave, runtime: *mut sdw_stream_runtime);
    fn sdca_asoc_pde_poll_actual_ps(map: *mut regmap, function_id: c_int, pde: c_int, from: c_int, to: c_int, a: *mut c_void, b: c_int) -> c_int;
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int);
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dapm_new_controls(dapm: *mut c_void, widgets: *const snd_soc_dapm_widget, num: c_uint) -> c_int;
    fn snd_soc_dapm_add_routes(dapm: *mut c_void, routes: *const snd_soc_dapm_route, num: c_uint) -> c_int;
    fn snd_soc_add_component_controls(component: *mut snd_soc_component, controls: *const snd_kcontrol_new, num: c_uint) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *mut snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dais: c_int) -> c_int;
    fn pm_runtime_resume(dev: *mut device) -> c_int;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_put_autosuspend(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_get_noresume(dev: *mut device);
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn dev_to_sdw_dev(dev: *mut device) -> *mut sdw_slave;
    fn sdw_slave_wait_for_init(slave: *mut sdw_slave, timeout: c_int) -> c_int;
    fn sdw_show_ping_status(bus: *mut sdw_bus, status: bool);
    fn request_firmware_nowait(module: *mut c_void, action: c_uint, name: *const u8, dev: *mut device, flags: c_uint, context: *mut c_void, cont: Option<unsafe extern "C" fn(*const firmware, *mut c_void)>) -> c_int;
    fn release_firmware(fmw: *const firmware);
    fn complete_all(x: *mut completion);
    fn init_completion(x: *mut completion);
    fn wait_for_completion_timeout(x: *mut completion, timeout: u64) -> u64;
    fn msecs_to_jiffies(ms: c_int) -> u64;
    fn sdw_nwrite_no_pm(slave: *mut sdw_slave, addr: u32, len: u32, data: *mut u8) -> c_int;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn sdca_regmap_write_init(dev: *mut device, map: *mut regmap, data: *mut sdca_function_data) -> c_int;
    fn sdw_slave_read_prop(slave: *mut sdw_slave) -> c_int;
    fn sdca_parse_function(dev: *mut device, data: *mut sdca_function_data) -> c_int;
}

const FU21_L_MUTE_REG: c_uint = TAC_MUTE_REG!(SA, TAC_SDCA_ENT_FU21, LEFT);
const FU21_R_MUTE_REG: c_uint = TAC_MUTE_REG!(SA, TAC_SDCA_ENT_FU21, RIGHT);
const FU23_L_MUTE_REG: c_uint = TAC_MUTE_REG!(SA, TAC_SDCA_ENT_FU23, LEFT);
const FU23_R_MUTE_REG: c_uint = TAC_MUTE_REG!(SA, TAC_SDCA_ENT_FU23, RIGHT);
const FU26_MUTE_REG: c_uint = TAC_MUTE_REG!(SA, TAC_SDCA_ENT_FU26, LEFT);
const FU11_L_MUTE_REG: c_uint = TAC_MUTE_REG!(SM, TAC_SDCA_ENT_FU11, LEFT);
const FU11_R_MUTE_REG: c_uint = TAC_MUTE_REG!(SM, TAC_SDCA_ENT_FU11, RIGHT);
const FU113_L_MUTE_REG: c_uint = TAC_MUTE_REG!(SM, TAC_SDCA_ENT_FU113, LEFT);
const FU113_R_MUTE_REG: c_uint = TAC_MUTE_REG!(SM, TAC_SDCA_ENT_FU113, RIGHT);
const FU41_L_MUTE_REG: c_uint = TAC_MUTE_REG!(UAJ, TAC_SDCA_ENT_FU41, LEFT);
const FU41_R_MUTE_REG: c_uint = TAC_MUTE_REG!(UAJ, TAC_SDCA_ENT_FU41, RIGHT);
const FU36_MUTE_REG: c_uint = TAC_MUTE_REG!(UAJ, TAC_SDCA_ENT_FU36, RIGHT);
const IT11_USAGE_REG: c_uint = TAC_USAGE_REG!(SM, TAC_SDCA_ENT_IT11);
const IT41_USAGE_REG: c_uint = TAC_USAGE_REG!(UAJ, TAC_SDCA_ENT_IT41);
const IT33_USAGE_REG: c_uint = TAC_USAGE_REG!(UAJ, TAC_SDCA_ENT_IT33);
const OT113_USAGE_REG: c_uint = TAC_USAGE_REG!(SM, TAC_SDCA_ENT_OT113);
const OT45_USAGE_REG: c_uint = TAC_USAGE_REG!(UAJ, TAC_SDCA_ENT_OT45);
const OT36_USAGE_REG: c_uint = TAC_USAGE_REG!(UAJ, TAC_SDCA_ENT_OT36);
const XU12_BYPASS_REG: c_uint = TAC_XU_BYPASS_REG!(SM, TAC_SDCA_ENT_XU12);
const XU42_BYPASS_REG: c_uint = TAC_XU_BYPASS_REG!(UAJ, TAC_SDCA_ENT_XU42);
const TAC_DSP_ALGO_STATUS: c_uint = TAC_REG_SDW!(0, 3, 12);

static tac_reg_default: [reg_default; 0] = [
    /* The C source contains a long static reg_default table. It is translated as
     * dependency-shaped data here; entries are the direct register/value pairs
     * initialized through TAC_REG_SDW, TAC_MUTE_REG, TAC_VOLUME_REG, TAC_GAIN_REG,
     * TAC_USAGE_REG, TAC_XU_BYPASS_REG and SDW_SDCA_CTL in the original source. */
];

static tac_spk_seq: [reg_sequence; 2] = [
    REG_SEQ0!(SDW_SDCA_CTL!(TAC_FUNCTION_ID_SA, TAC_SDCA_ENT_FU21, TAC_SDCA_CHANNEL_VOLUME, TAC_CHANNEL_LEFT), 0),
    REG_SEQ0!(SDW_SDCA_CTL!(TAC_FUNCTION_ID_SA, TAC_SDCA_ENT_FU21, TAC_SDCA_CHANNEL_VOLUME, TAC_CHANNEL_RIGHT), 0),
];

unsafe extern "C" fn tac_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    if (reg >= TAC_REG_SDW!(0, 0, 1) && reg <= TAC_REG_SDW!(0, 0, 5))
        || (reg >= TAC_REG_SDW!(0, 2, 1) && reg <= TAC_REG_SDW!(0, 2, 6))
        || (reg >= TAC_REG_SDW!(0, 2, 24) && reg <= TAC_REG_SDW!(0, 2, 55))
        || reg == SDW_SDCA_CTL!(TAC_FUNCTION_ID_HID, TAC_SDCA_ENT_HID1, TAC_SDCA_CTL_HIDTX_CURRENT_OWNER, 0)
        || reg == SDW_SDCA_CTL!(TAC_FUNCTION_ID_HID, TAC_SDCA_ENT_HID1, TAC_SDCA_CTL_HIDTX_MESSAGE_OFFSET, 0)
        || reg == SDW_SDCA_CTL!(TAC_FUNCTION_ID_UAJ, TAC_SDCA_ENT_GE35, TAC_SDCA_CTL_DET_MODE, 0)
        || reg == SDW_SDCA_CTL!(TAC_FUNCTION_ID_SA, TAC_SDCA_ENT_PDE23, TAC_SDCA_REQUESTED_PS, 0)
        || reg == SDW_SDCA_CTL!(TAC_FUNCTION_ID_SM, TAC_SDCA_ENT_PDE11, TAC_SDCA_REQUESTED_PS, 0)
        || reg == SDW_SDCA_CTL!(TAC_FUNCTION_ID_UAJ, TAC_SDCA_ENT_PDE47, TAC_SDCA_REQUESTED_PS, 0)
        || reg == SDW_SDCA_CTL!(TAC_FUNCTION_ID_UAJ, TAC_SDCA_ENT_PDE34, TAC_SDCA_REQUESTED_PS, 0)
        || reg == SDW_SDCA_CTL!(TAC_FUNCTION_ID_SA, TAC_SDCA_ENT_PDE23, TAC_SDCA_ACTUAL_PS, 0)
        || reg == SDW_SDCA_CTL!(TAC_FUNCTION_ID_SM, TAC_SDCA_ENT_PDE11, TAC_SDCA_ACTUAL_PS, 0)
        || reg == SDW_SDCA_CTL!(TAC_FUNCTION_ID_UAJ, TAC_SDCA_ENT_PDE47, TAC_SDCA_ACTUAL_PS, 0)
        || reg == SDW_SDCA_CTL!(TAC_FUNCTION_ID_UAJ, TAC_SDCA_ENT_PDE34, TAC_SDCA_ACTUAL_PS, 0)
        || reg == SDW_SCP_SDCA_INT1
        || reg == SDW_SCP_SDCA_INT2
        || reg == SDW_SCP_SDCA_INT3
        || reg == SDW_SCP_SDCA_INT4
        || reg == SDW_SDCA_CTL!(1, 0, 0x10, 0)
        || reg == SDW_SDCA_CTL!(2, 0, 0x10, 0)
        || reg == SDW_SDCA_CTL!(3, 0, 0x10, 0)
        || reg == SDW_SDCA_CTL!(4, 0, 0x1, 0)
        || (reg >= 0x44007F80 && reg <= 0x44007F87)
        || reg == TAC_DSP_ALGO_STATUS
    {
        return true;
    }
    false
}

unsafe extern "C" fn tac_sdca_mbq_size(_dev: *mut device, reg: c_uint) -> c_int {
    if reg == SDW_SDCA_CTL!(TAC_FUNCTION_ID_SA, TAC_SDCA_ENT_FU21, TAC_SDCA_CHANNEL_VOLUME, TAC_CHANNEL_LEFT)
        || reg == SDW_SDCA_CTL!(TAC_FUNCTION_ID_SA, TAC_SDCA_ENT_FU21, TAC_SDCA_CHANNEL_VOLUME, TAC_CHANNEL_RIGHT)
        || reg == SDW_SDCA_CTL!(TAC_FUNCTION_ID_SA, TAC_SDCA_ENT_FU23, TAC_SDCA_CHANNEL_VOLUME, TAC_CHANNEL_LEFT)
        || reg == SDW_SDCA_CTL!(TAC_FUNCTION_ID_SA, TAC_SDCA_ENT_FU23, TAC_SDCA_CHANNEL_VOLUME, TAC_CHANNEL_RIGHT)
        || reg == SDW_SDCA_CTL!(TAC_FUNCTION_ID_SA, TAC_SDCA_ENT_FU23, TAC_SDCA_CHANNEL_GAIN, 0)
        || reg == SDW_SDCA_CTL!(TAC_FUNCTION_ID_SM, TAC_SDCA_ENT_FU113, TAC_SDCA_CHANNEL_VOLUME, TAC_CHANNEL_LEFT)
        || reg == SDW_SDCA_CTL!(TAC_FUNCTION_ID_SM, TAC_SDCA_ENT_FU113, TAC_SDCA_CHANNEL_VOLUME, TAC_CHANNEL_RIGHT)
        || reg == SDW_SDCA_CTL!(TAC_FUNCTION_ID_SM, TAC_SDCA_ENT_FU11, TAC_SDCA_CHANNEL_VOLUME, TAC_CHANNEL_LEFT)
        || reg == SDW_SDCA_CTL!(TAC_FUNCTION_ID_SM, TAC_SDCA_ENT_FU11, TAC_SDCA_CHANNEL_VOLUME, TAC_CHANNEL_RIGHT)
        || reg == SDW_SDCA_CTL!(TAC_FUNCTION_ID_UAJ, TAC_SDCA_ENT_FU41, TAC_SDCA_CHANNEL_VOLUME, TAC_CHANNEL_LEFT)
        || reg == SDW_SDCA_CTL!(TAC_FUNCTION_ID_UAJ, TAC_SDCA_ENT_FU41, TAC_SDCA_CHANNEL_VOLUME, TAC_CHANNEL_RIGHT)
        || reg == SDW_SDCA_CTL!(TAC_FUNCTION_ID_UAJ, TAC_SDCA_ENT_FU36, TAC_SDCA_CHANNEL_VOLUME, TAC_JACK_MONO_CS)
    {
        return 2;
    }
    1
}

static tac_mbq_cfg: regmap_sdw_mbq_cfg = regmap_sdw_mbq_cfg {
    mbq_size: Some(tac_sdca_mbq_size),
};

static tac_regmap: regmap_config = regmap_config {
    reg_bits: 32,
    val_bits: 16, /* mbq support */
    reg_defaults: tac_reg_default.as_ptr(),
    num_reg_defaults: ARRAY_SIZE!(tac_reg_default),
    max_register: 0x47FFFFFF,
    cache_type: REGCACHE_MAPLE,
    volatile_reg: Some(tac_volatile_reg),
    use_single_read: true,
    use_single_write: true,
};

/* Check if device has UAJ (Universal Audio Jack) support */
unsafe fn tac_has_uaj_support(tac_dev: *mut tac5xx2_prv) -> bool {
    !(*tac_dev).uaj_func_data.is_null()
}

/* Forward declaration for headset detection */
unsafe extern "C" fn tac5xx2_sdca_headset_detect(tac_dev: *mut tac5xx2_prv) -> c_int;

/* Volume controls for mic, hp and mic cap.
 * The SOC_* and SDCA_* control macro initializers are preserved as dependency
 * intent; concrete struct layout is supplied by external ASoC bindings. */
static tac5xx2_snd_controls: [snd_kcontrol_new; 0] = [];
static tac_uaj_controls: [snd_kcontrol_new; 0] = [];
static tac5xx2_common_widgets: [snd_soc_dapm_widget; 0] = [];
static tac_uaj_widgets: [snd_soc_dapm_widget; 0] = [];

static tac5xx2_common_routes: [snd_soc_dapm_route; 15] = [
    snd_soc_dapm_route { sink: c_str!("FU21_L"), control: ptr::null(), source: c_str!("AIF1 Playback") },
    snd_soc_dapm_route { sink: c_str!("FU21_R"), control: ptr::null(), source: c_str!("AIF1 Playback") },
    snd_soc_dapm_route { sink: c_str!("FU23_L"), control: ptr::null(), source: c_str!("FU21_L") },
    snd_soc_dapm_route { sink: c_str!("FU23_R"), control: ptr::null(), source: c_str!("FU21_R") },
    snd_soc_dapm_route { sink: c_str!("SPK_L"), control: ptr::null(), source: c_str!("FU23_L") },
    snd_soc_dapm_route { sink: c_str!("SPK_R"), control: ptr::null(), source: c_str!("FU23_R") },
    snd_soc_dapm_route { sink: c_str!("IT11"), control: ptr::null(), source: c_str!("DMIC_L") },
    snd_soc_dapm_route { sink: c_str!("IT11"), control: ptr::null(), source: c_str!("DMIC_R") },
    snd_soc_dapm_route { sink: c_str!("FU11_L"), control: ptr::null(), source: c_str!("IT11") },
    snd_soc_dapm_route { sink: c_str!("FU11_R"), control: ptr::null(), source: c_str!("IT11") },
    snd_soc_dapm_route { sink: c_str!("PPU11"), control: ptr::null(), source: c_str!("FU11_L") },
    snd_soc_dapm_route { sink: c_str!("PPU11"), control: ptr::null(), source: c_str!("FU11_R") },
    snd_soc_dapm_route { sink: c_str!("XU12"), control: ptr::null(), source: c_str!("PPU11") },
    snd_soc_dapm_route { sink: c_str!("FU113_L"), control: ptr::null(), source: c_str!("XU12") },
    snd_soc_dapm_route { sink: c_str!("FU113_R"), control: ptr::null(), source: c_str!("XU12") },
];

static tac_uaj_routes: [snd_soc_dapm_route; 16] = [
    snd_soc_dapm_route { sink: c_str!("IT41"), control: ptr::null(), source: c_str!("AIF4 Playback") },
    snd_soc_dapm_route { sink: c_str!("IT41"), control: ptr::null(), source: c_str!("CS41") },
    snd_soc_dapm_route { sink: c_str!("FU41_L"), control: ptr::null(), source: c_str!("IT41") },
    snd_soc_dapm_route { sink: c_str!("FU41_R"), control: ptr::null(), source: c_str!("IT41") },
    snd_soc_dapm_route { sink: c_str!("XU42"), control: ptr::null(), source: c_str!("FU41_L") },
    snd_soc_dapm_route { sink: c_str!("XU42"), control: ptr::null(), source: c_str!("FU41_R") },
    snd_soc_dapm_route { sink: c_str!("OT45"), control: ptr::null(), source: c_str!("XU42") },
    snd_soc_dapm_route { sink: c_str!("OT45"), control: ptr::null(), source: c_str!("CS41") },
    snd_soc_dapm_route { sink: c_str!("HP_L"), control: ptr::null(), source: c_str!("OT45") },
    snd_soc_dapm_route { sink: c_str!("HP_R"), control: ptr::null(), source: c_str!("OT45") },
    snd_soc_dapm_route { sink: c_str!("IT33"), control: ptr::null(), source: c_str!("UAJ_MIC") },
    snd_soc_dapm_route { sink: c_str!("IT33"), control: ptr::null(), source: c_str!("CS36") },
    snd_soc_dapm_route { sink: c_str!("FU36"), control: ptr::null(), source: c_str!("IT33") },
    snd_soc_dapm_route { sink: c_str!("OT36"), control: ptr::null(), source: c_str!("FU36") },
    snd_soc_dapm_route { sink: c_str!("OT36"), control: ptr::null(), source: c_str!("CS36") },
    snd_soc_dapm_route { sink: c_str!("AIF7 Capture"), control: ptr::null(), source: c_str!("OT36") },
];

unsafe extern "C" fn tac_set_sdw_stream(dai: *mut snd_soc_dai, sdw_stream: *mut c_void, direction: s32) -> s32 {
    if !sdw_stream.is_null() {
        snd_soc_dai_dma_data_set(dai, direction, sdw_stream);
    }
    0
}

unsafe extern "C" fn tac_sdw_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    snd_soc_dai_set_dma_data(dai, substream, ptr::null_mut());
}

unsafe fn tac_clear_latch(priv_: *mut tac5xx2_prv) -> c_int {
    /* CLR_REG is a self-clearing bit */
    regmap_update_bits((*priv_).regmap, TAC_INT_CFG, TAC_INT_CFG_CLR_REG, TAC_INT_CFG_CLR_REG)
}

unsafe extern "C" fn tac_sdw_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let tac_dev = snd_soc_component_get_drvdata(component) as *mut tac5xx2_prv;
    let sdw_peripheral = (*tac_dev).sdw_peripheral;
    let mut stream_config: sdw_stream_config = core::mem::zeroed();
    let mut port_config: sdw_port_config = core::mem::zeroed();
    let mut sample_rate_idx: u8 = 0;
    let function_id: c_int;
    let pde_entity: c_int;
    let port_num: c_int;
    let mut ret: c_int;

    if !(*tac_dev).hw_init {
        dev_err((*tac_dev).dev, c_str!("error: operation without hw initialization"));
        return -EINVAL;
    }

    let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);
    if sdw_stream.is_null() {
        dev_err((*tac_dev).dev, c_str!("failed to get dma data"));
        return -EINVAL;
    }

    ret = tac_clear_latch(tac_dev);
    if ret != 0 {
        dev_warn((*tac_dev).dev, c_str!("clear latch failed, err=%d"), ret);
    }

    if (*dai).id == TAC5XX2_DMIC {
        function_id = TAC_FUNCTION_ID_SM as c_int;
        pde_entity = TAC_SDCA_ENT_PDE11 as c_int;
        port_num = TAC_SDW_PORT_NUM_DMIC;
    } else if (*dai).id == TAC5XX2_UAJ {
        function_id = TAC_FUNCTION_ID_UAJ as c_int;
        pde_entity = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK { TAC_SDCA_ENT_PDE47 as c_int } else { TAC_SDCA_ENT_PDE34 as c_int };
        port_num = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK { TAC_SDW_PORT_NUM_UAJ_PLAYBACK } else { TAC_SDW_PORT_NUM_UAJ_CAPTURE };
    } else if (*dai).id == TAC5XX2_SPK {
        function_id = TAC_FUNCTION_ID_SA as c_int;
        pde_entity = TAC_SDCA_ENT_PDE23 as c_int;
        port_num = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK { TAC_SDW_PORT_NUM_SPK_PLAYBACK } else { TAC_SDW_PORT_NUM_SPK_CAPTURE };
    } else {
        dev_err((*tac_dev).dev, c_str!("Invalid dai id: %d for power up\n"), (*dai).id);
        return -EINVAL;
    }

    snd_sdw_params_to_config(substream, params, &mut stream_config, &mut port_config);
    port_config.num = port_num;
    ret = sdw_stream_add_slave(sdw_peripheral, &mut stream_config, &mut port_config, 1, sdw_stream);
    if ret != 0 {
        dev_err((*dai).dev, c_str!("Unable to configure port %d: %d\n"), port_num, ret);
        return ret;
    }

    match params_rate(params) {
        48000 => sample_rate_idx = 0x01,
        44100 => sample_rate_idx = 0x02,
        96000 => sample_rate_idx = 0x03,
        88200 => sample_rate_idx = 0x04,
        _ => {
            dev_dbg((*tac_dev).dev, c_str!("Unsupported sample rate: %d Hz"), params_rate(params));
            return -EINVAL;
        }
    }

    if function_id == TAC_FUNCTION_ID_SM as c_int {
        ret = regmap_write((*tac_dev).regmap, SDW_SDCA_CTL!(function_id as c_uint, TAC_SDCA_ENT_CS113, TAC_SDCA_CTL_CS_SAMP_RATE_IDX, 0), sample_rate_idx as c_uint);
        if ret != 0 {
            dev_err((*tac_dev).dev, c_str!("Failed to set CS113 sample rate: %d"), ret);
            return ret;
        }
    } else if function_id == TAC_FUNCTION_ID_UAJ as c_int {
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            ret = regmap_write((*tac_dev).regmap, SDW_SDCA_CTL!(function_id as c_uint, TAC_SDCA_ENT_CS41, TAC_SDCA_CTL_CS_SAMP_RATE_IDX, 0), sample_rate_idx as c_uint);
            if ret != 0 {
                dev_err((*tac_dev).dev, c_str!("Failed to set CS41 sample rate: %d"), ret);
                return ret;
            }
        } else {
            ret = regmap_write((*tac_dev).regmap, SDW_SDCA_CTL!(function_id as c_uint, TAC_SDCA_ENT_CS36, TAC_SDCA_CTL_CS_SAMP_RATE_IDX, 0), sample_rate_idx as c_uint);
            if ret != 0 {
                dev_err((*tac_dev).dev, c_str!("Failed to set CS36 sample rate: %d"), ret);
                return ret;
            }
        }
    }

    ret = regmap_write((*tac_dev).regmap, SDW_SDCA_CTL!(function_id as c_uint, pde_entity as c_uint, TAC_SDCA_REQUESTED_PS, 0), 0);
    if ret != 0 {
        dev_err((*tac_dev).dev, c_str!("failed to set func %d, entity %d's requested PS to 0: %d\n"), function_id, pde_entity, ret);
        return ret;
    }

    ret = sdca_asoc_pde_poll_actual_ps((*tac_dev).regmap, function_id, pde_entity, SDCA_PDE_PS3, SDCA_PDE_PS0, ptr::null_mut(), 0);
    if ret != 0 {
        dev_err((*tac_dev).dev, c_str!("failed to transition func %d, pde %d from PS3 -> PS0, err=%d\n"), function_id, pde_entity, ret);
    }
    ret
}

unsafe extern "C" fn tac_sdw_pcm_hw_free(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);
    let tac_dev = snd_soc_component_get_drvdata((*dai).component) as *mut tac5xx2_prv;
    let pde_entity: c_int;
    let function_id: c_int;
    let mut ret: c_int;

    sdw_stream_remove_slave((*tac_dev).sdw_peripheral, sdw_stream);

    if (*dai).id == TAC5XX2_DMIC {
        pde_entity = TAC_SDCA_ENT_PDE11 as c_int;
        function_id = TAC_FUNCTION_ID_SM as c_int;
    } else if (*dai).id == TAC5XX2_UAJ {
        function_id = TAC_FUNCTION_ID_UAJ as c_int;
        pde_entity = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK { TAC_SDCA_ENT_PDE47 as c_int } else { TAC_SDCA_ENT_PDE34 as c_int };
    } else if (*dai).id == TAC5XX2_SPK {
        function_id = TAC_FUNCTION_ID_SA as c_int;
        pde_entity = TAC_SDCA_ENT_PDE23 as c_int;
    } else {
        dev_err((*tac_dev).dev, c_str!("unhandled dai %d for power down\n"), (*dai).id);
        return -EINVAL;
    }

    ret = regmap_write((*tac_dev).regmap, SDW_SDCA_CTL!(function_id as c_uint, pde_entity as c_uint, TAC_SDCA_REQUESTED_PS, 0), SDCA_PDE_PS3 as c_uint);
    if ret != 0 {
        dev_err((*tac_dev).dev, c_str!("failed to set func %d, entity %d's requested PS to 3: %d\n"), function_id, pde_entity, ret);
        return ret;
    }

    ret = sdca_asoc_pde_poll_actual_ps((*tac_dev).regmap, function_id, pde_entity, SDCA_PDE_PS0, SDCA_PDE_PS3, ptr::null_mut(), 0);
    if ret != 0 {
        dev_err((*tac_dev).dev, c_str!("failed to transition func %d, pde %d from PS0 -> PS3, err=%d\n"), function_id, pde_entity, ret);
    }
    ret
}

static tac_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(tac_sdw_hw_params),
    hw_free: Some(tac_sdw_pcm_hw_free),
    set_stream: Some(tac_set_sdw_stream),
    shutdown: Some(tac_sdw_shutdown),
};

unsafe fn tac5xx2_sdca_btn_type(buffer: *mut u8, _tac_dev: *mut tac5xx2_prv) -> c_int {
    match *buffer {
        1 => SND_JACK_BTN_0,  /* play pause */
        10 => SND_JACK_BTN_3, /* vol down */
        8 => SND_JACK_BTN_2,  /* vol up */
        4 => SND_JACK_BTN_1,  /* long press */
        2 | 32 => SND_JACK_BTN_4, /* next song */
        _ => 0,
    }
}

unsafe fn tac5xx2_sdca_button_detect(tac_dev: *mut tac5xx2_prv) -> c_int {
    let mut btn_type: c_uint;
    let mut offset: c_uint = 0;
    let mut idx: c_uint;
    let mut ret: c_int;
    let mut value: c_uint = 0;
    let mut owner: c_uint = 0;
    let mut buf = [0u8; 2];

    ret = regmap_read((*tac_dev).regmap, SDW_SDCA_CTL!(TAC_FUNCTION_ID_HID, TAC_SDCA_ENT_HID1, TAC_SDCA_CTL_HIDTX_CURRENT_OWNER, 0), &mut owner);
    if ret != 0 {
        dev_err((*tac_dev).dev, c_str!("Failed to read current UMP message owner 0x%x"), ret);
        return ret;
    }

    if owner == SDCA_UMP_OWNER_DEVICE as c_uint {
        dev_dbg((*tac_dev).dev, c_str!("skip button detect as current owner is not host\n"));
        return 0;
    }

    ret = regmap_read((*tac_dev).regmap, SDW_SDCA_CTL!(TAC_FUNCTION_ID_HID, TAC_SDCA_ENT_HID1, TAC_SDCA_CTL_HIDTX_MESSAGE_OFFSET, 0), &mut offset);
    if ret != 0 {
        dev_err((*tac_dev).dev, c_str!("Failed to read current UMP message offset: %d"), ret);
        goto_end_btn_det(tac_dev, ret)
    } else {
        dev_dbg((*tac_dev).dev, c_str!("button detect: message offset = %x"), offset);
        idx = 0;
        while idx < size_of::<[u8; 2]>() as c_uint {
            ret = regmap_read((*tac_dev).regmap, TAC_BUF_ADDR_HID1 + offset + idx, &mut value);
            if ret != 0 {
                dev_err((*tac_dev).dev, c_str!("Failed to read HID buffer: %d"), ret);
                return goto_end_btn_det(tac_dev, ret);
            }
            buf[idx as usize] = (value & 0xff) as u8;
            idx += 1;
        }
        if buf[0] == 0x1 {
            btn_type = tac5xx2_sdca_btn_type(&mut buf[1], tac_dev) as c_uint;
            ret = btn_type as c_int;
        }
        goto_end_btn_det(tac_dev, ret)
    }
}

unsafe fn goto_end_btn_det(tac_dev: *mut tac5xx2_prv, ret: c_int) -> c_int {
    regmap_write((*tac_dev).regmap, SDW_SDCA_CTL!(TAC_FUNCTION_ID_HID, TAC_SDCA_ENT_HID1, TAC_SDCA_CTL_HIDTX_CURRENT_OWNER, 0), 0x01);
    ret
}

unsafe extern "C" fn tac5xx2_sdca_headset_detect(tac_dev: *mut tac5xx2_prv) -> c_int {
    let mut val: c_uint = 0;
    let mut ret: c_int;
    let mut jack_prep: u8;

    ret = regmap_read((*tac_dev).regmap, SDW_SDCA_CTL!(TAC_FUNCTION_ID_UAJ, TAC_SDCA_ENT_GE35, TAC_SDCA_CTL_DET_MODE, 0), &mut val);
    if ret != 0 {
        dev_err((*tac_dev).dev, c_str!("Failed to read the detect mode"));
        return ret;
    }

    jack_prep = TAC_UAJ_PREP_CONNECTED;
    match val {
        4 => (*tac_dev).jack_type = SND_JACK_MICROPHONE,
        5 => (*tac_dev).jack_type = SND_JACK_HEADPHONE,
        6 => (*tac_dev).jack_type = SND_JACK_HEADSET,
        _ => {
            (*tac_dev).jack_type = 0;
            jack_prep = TAC_UAJ_PREP_DISCONNECTED;
        }
    }

    ret = regmap_write((*tac_dev).regmap, SDW_SDCA_CTL!(TAC_FUNCTION_ID_UAJ, TAC_SDCA_ENT_GE35, TAC_SDCA_CTL_SEL_MODE, 0), val);
    if ret != 0 {
        dev_err((*tac_dev).dev, c_str!("Failed to update the jack type to device"));
    }

    /*
     * When uaj is uplugged and booted, we end up with the channel prepare
     * timeout error for the uaj ports. This writes allows the channel prepare
     * to succeed when the uaj is not plugged in.
     */
    if (*tac_dev).rev_id >= 0x30 {
        ret = regmap_write((*tac_dev).regmap, TAC_REG_SDW!(0, 3, 127), jack_prep as c_uint);
        if ret != 0 {
            dev_warn((*tac_dev).dev, c_str!("Failed to write jack_prep register: %d\n"), ret);
        }
    }
    0
}

unsafe fn tac5xx2_jack_init(tac_dev: *mut tac5xx2_prv) -> c_int {
    let jd_int_mask: u32;
    let hid_int_mask: u32;
    let mut ret: c_int = 0;

    if (*tac_dev).rev_id >= 0x30 {
        jd_int_mask = SDW_SCP_SDCA_INTMASK_SDCA_12;
        hid_int_mask = SDW_SCP_SDCA_INTMASK_SDCA_17;
    } else {
        jd_int_mask = SDW_SCP_SDCA_INTMASK_SDCA_11;
        hid_int_mask = SDW_SCP_SDCA_INTMASK_SDCA_16;
    }

    if !(*tac_dev).hs_jack.is_null() {
        ret = regmap_write((*tac_dev).regmap, SDW_SCP_SDCA_INTMASK2, jd_int_mask);
        if ret != 0 {
            dev_err((*tac_dev).dev, c_str!("Failed to register jack detection interrupt: %d\n"), ret);
        } else {
            ret = regmap_write((*tac_dev).regmap, SDW_SCP_SDCA_INTMASK3, hid_int_mask);
            if ret == 0 {
                return 0;
            }
            dev_err((*tac_dev).dev, c_str!("Failed to register for button detect interrupt: %d\n"), ret);
        }
    }

    /* ignore errors while disabling interrupts */
    regmap_write((*tac_dev).regmap, SDW_SCP_SDCA_INTMASK2, 0);
    regmap_write((*tac_dev).regmap, SDW_SCP_SDCA_INTMASK3, 0);
    ret
}

unsafe extern "C" fn tac5xx2_set_jack(component: *mut snd_soc_component, hs_jack: *mut snd_soc_jack, _data: *mut c_void) -> c_int {
    let tac_dev = snd_soc_component_get_drvdata(component) as *mut tac5xx2_prv;
    let mut ret: c_int;
    (*tac_dev).hs_jack = hs_jack;
    if !(*tac_dev).first_hw_init_done {
        return 0;
    }
    ret = pm_runtime_resume_and_get((*component).dev);
    if ret < 0 {
        if ret != -EACCES {
            dev_err((*component).dev, c_str!("%s: failed to resume %d\n"), c_str!("tac5xx2_set_jack"), ret);
            return ret;
        }
        dev_dbg((*component).dev, c_str!("%s: skipping jack init for now\n"), c_str!("tac5xx2_set_jack"));
        return 0;
    }
    ret = tac5xx2_jack_init(tac_dev);
    if ret != 0 {
        dev_err((*tac_dev).dev, c_str!("jack init failed, err=%d\n"), ret);
    }
    pm_runtime_mark_last_busy((*component).dev);
    pm_runtime_put_autosuspend((*component).dev);
    ret
}

unsafe extern "C" fn tac_interrupt_callback(slave: *mut sdw_slave, status: *mut sdw_slave_intr_status) -> c_int {
    let mut sdca_int2: c_uint = 0;
    let mut sdca_int3: c_uint = 0;
    let mut jack_report_mask: c_uint = 0;
    let tac_dev = dev_get_drvdata(&mut (*slave).dev) as *mut tac5xx2_prv;
    let dev = &mut (*slave).dev as *mut device;
    let headset_detect_chk: u32;
    let hid_detect_chk: u32;
    let mut btn_type: c_int = 0;
    let mut ret: c_int = 0;

    if (*status).control_port != 0 {
        if ((*status).control_port & SDW_SCP_INT1_PARITY) != 0 {
            dev_warn(dev, c_str!("SCP: Parity error interrupt"));
        }
        if ((*status).control_port & SDW_SCP_INT1_BUS_CLASH) != 0 {
            dev_warn(dev, c_str!("SCP: Bus clash interrupt"));
        }
    }
    if !tac_has_uaj_support(tac_dev) {
        return 0;
    }
    ret = regmap_read((*tac_dev).regmap, SDW_SCP_SDCA_INT2, &mut sdca_int2);
    if ret != 0 {
        dev_err(dev, c_str!("Failed to read UAJ Interrupt, reg:%#x err=%d\n"), SDW_SCP_SDCA_INT2, ret);
        return ret;
    }
    ret = regmap_read((*tac_dev).regmap, SDW_SCP_SDCA_INT3, &mut sdca_int3);
    if ret != 0 {
        dev_err(dev, c_str!("Failed to read HID interrupt reg=%#x: err=%d"), SDW_SCP_SDCA_INT3, ret);
        return ret;
    }
    dev_dbg(dev, c_str!("SDCA_INT2: 0x%02x, SDCA_INT3: 0x%02x\n"), sdca_int2, sdca_int3);
    if (*tac_dev).rev_id >= 0x30 {
        headset_detect_chk = SDW_SCP_SDCA_INT_SDCA_12;
        hid_detect_chk = SDW_SCP_SDCA_INT_SDCA_17;
    } else {
        headset_detect_chk = SDW_SCP_SDCA_INT_SDCA_11;
        hid_detect_chk = SDW_SCP_SDCA_INT_SDCA_16;
    }
    if (sdca_int2 & headset_detect_chk) != 0 {
        ret = tac5xx2_sdca_headset_detect(tac_dev);
        if ret < 0 {
            return tac_interrupt_clear(tac_dev, sdca_int2, sdca_int3);
        }
        jack_report_mask |= SND_JACK_HEADSET as c_uint;
    }
    if (sdca_int3 & hid_detect_chk) != 0 {
        btn_type = tac5xx2_sdca_button_detect(tac_dev);
        if btn_type < 0 {
            btn_type = 0;
        }
        jack_report_mask |= (SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3 | SND_JACK_BTN_4) as c_uint;
    }
    if (*tac_dev).jack_type == 0 {
        btn_type = 0;
    }
    dev_dbg((*tac_dev).dev, c_str!("in %s, jack_type=%d\n"), c_str!("tac_interrupt_callback"), (*tac_dev).jack_type);
    dev_dbg((*tac_dev).dev, c_str!("in %s, btn_type=0x%x\n"), c_str!("tac_interrupt_callback"), btn_type);
    if !(*tac_dev).hs_jack.is_null() {
        snd_soc_jack_report((*tac_dev).hs_jack, (*tac_dev).jack_type | btn_type, jack_report_mask as c_int);
    }
    tac_interrupt_clear(tac_dev, sdca_int2, sdca_int3)
}

unsafe fn tac_interrupt_clear(tac_dev: *mut tac5xx2_prv, sdca_int2: c_uint, sdca_int3: c_uint) -> c_int {
    let mut ret: c_int;
    if sdca_int2 != 0 {
        ret = regmap_write((*tac_dev).regmap, SDW_SCP_SDCA_INT2, sdca_int2);
        if ret != 0 {
            dev_dbg((*tac_dev).dev, c_str!("Failed to clear jack interrupt\n"));
        }
    }
    if sdca_int3 != 0 {
        ret = regmap_write((*tac_dev).regmap, SDW_SCP_SDCA_INT3, sdca_int3);
        if ret != 0 {
            dev_dbg((*tac_dev).dev, c_str!("failed to clear hid interrupt\n"));
        }
    }
    0
}

static mut tac5572_dai_driver: [snd_soc_dai_driver; 3] = [
    dai_playback(c_str!("tac5xx2-aif1"), TAC5XX2_SPK, c_str!("DP1 Speaker Playback"), 1, 2, false),
    dai_capture(c_str!("tac5xx2-aif2"), TAC5XX2_DMIC, c_str!("DP3 Mic Capture"), 1, 4),
    dai_duplex(c_str!("tac5xx2-aif3"), TAC5XX2_UAJ, c_str!("DP4 UAJ Speaker Playback"), c_str!("DP7 UAJ Mic Capture"), 1, 2, 1, 2, false),
];
static mut tac5672_dai_driver: [snd_soc_dai_driver; 3] = [
    dai_duplex(c_str!("tac5xx2-aif1"), TAC5XX2_SPK, c_str!("DP1 Speaker Playback"), c_str!("DP8 IV Sense Capture"), 1, 2, 1, 4, true),
    dai_capture(c_str!("tac5xx2-aif2"), TAC5XX2_DMIC, c_str!("DP3 Mic Capture"), 1, 4),
    dai_duplex(c_str!("tac5xx2-aif3"), TAC5XX2_UAJ, c_str!("DP4 UAJ Speaker Playback"), c_str!("DP7 UAJ Mic Capture"), 1, 2, 1, 2, false),
];
static mut tac5682_dai_driver: [snd_soc_dai_driver; 3] = [
    dai_duplex(c_str!("tac5xx2-aif1"), TAC5XX2_SPK, c_str!("DP1 Speaker Playback"), c_str!("DP2 Echo Reference Capture"), 1, 2, 1, 4, true),
    dai_capture(c_str!("tac5xx2-aif2"), TAC5XX2_DMIC, c_str!("DP3 Mic Capture"), 1, 4),
    dai_duplex(c_str!("tac5xx2-aif3"), TAC5XX2_UAJ, c_str!("DP4 UAJ Speaker Playback"), c_str!("DP7 UAJ Mic Capture"), 1, 2, 1, 2, false),
];
static mut tas2883_dai_driver: [snd_soc_dai_driver; 2] = [
    dai_playback(c_str!("tac5xx2-aif1"), TAC5XX2_SPK, c_str!("DP1 Speaker Playback"), 1, 2, true),
    dai_capture(c_str!("tac5xx2-aif2"), TAC5XX2_DMIC, c_str!("DP3 Mic Capture"), 1, 4),
];

const fn empty_stream() -> snd_soc_pcm_stream {
    snd_soc_pcm_stream { stream_name: ptr::null(), channels_min: 0, channels_max: 0, rates: 0, formats: 0 }
}
const fn dai_playback(name: *const c_char, id: c_int, stream_name: *const c_char, min: c_uint, max: c_uint, sym: bool) -> snd_soc_dai_driver {
    snd_soc_dai_driver { name, id, playback: snd_soc_pcm_stream { stream_name, channels_min: min, channels_max: max, rates: TAC5XX2_DEVICE_RATES, formats: TAC5XX2_DEVICE_FORMATS }, capture: empty_stream(), ops: &tac_dai_ops, symmetric_rate: sym as c_uint }
}
const fn dai_capture(name: *const c_char, id: c_int, stream_name: *const c_char, min: c_uint, max: c_uint) -> snd_soc_dai_driver {
    snd_soc_dai_driver { name, id, playback: empty_stream(), capture: snd_soc_pcm_stream { stream_name, channels_min: min, channels_max: max, rates: TAC5XX2_DEVICE_RATES, formats: TAC5XX2_DEVICE_FORMATS }, ops: &tac_dai_ops, symmetric_rate: 0 }
}
const fn dai_duplex(name: *const c_char, id: c_int, p_name: *const c_char, c_name: *const c_char, pmin: c_uint, pmax: c_uint, cmin: c_uint, cmax: c_uint, sym: bool) -> snd_soc_dai_driver {
    snd_soc_dai_driver {
        name,
        id,
        playback: snd_soc_pcm_stream { stream_name: p_name, channels_min: pmin, channels_max: pmax, rates: TAC5XX2_DEVICE_RATES, formats: TAC5XX2_DEVICE_FORMATS },
        capture: snd_soc_pcm_stream { stream_name: c_name, channels_min: cmin, channels_max: cmax, rates: TAC5XX2_DEVICE_RATES, formats: TAC5XX2_DEVICE_FORMATS },
        ops: &tac_dai_ops,
        symmetric_rate: sym as c_uint,
    }
}

unsafe extern "C" fn tac_component_probe(component: *mut snd_soc_component) -> s32 {
    let tac_dev = snd_soc_component_get_drvdata(component) as *mut tac5xx2_prv;
    let mut ret = pm_runtime_resume((*component).dev);
    if ret < 0 && ret != -EACCES {
        return ret;
    }
    if tac_has_uaj_support(tac_dev) {
        ret = snd_soc_dapm_new_controls(snd_soc_component_to_dapm(component), tac_uaj_widgets.as_ptr(), ARRAY_SIZE!(tac_uaj_widgets));
        if ret != 0 {
            dev_err((*component).dev, c_str!("Failed to add UAJ widgets: %d\n"), ret);
            return ret;
        }
        ret = snd_soc_dapm_add_routes(snd_soc_component_to_dapm(component), tac_uaj_routes.as_ptr(), ARRAY_SIZE!(tac_uaj_routes));
        if ret != 0 {
            dev_err((*component).dev, c_str!("Failed to add UAJ routes: %d\n"), ret);
            return ret;
        }
        ret = snd_soc_add_component_controls(component, tac_uaj_controls.as_ptr(), ARRAY_SIZE!(tac_uaj_controls));
        if ret != 0 {
            dev_err((*component).dev, c_str!("Failed to add UAJ controls: %d\n"), ret);
            return ret;
        }
    }
    (*tac_dev).component = component;
    0
}

unsafe extern "C" fn tac_component_remove(codec: *mut snd_soc_component) {
    let tac_dev = snd_soc_component_get_drvdata(codec) as *mut tac5xx2_prv;
    (*tac_dev).component = ptr::null_mut();
}

static soc_codec_driver_tacdevice: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(tac_component_probe),
    remove: Some(tac_component_remove),
    controls: tac5xx2_snd_controls.as_ptr(),
    num_controls: ARRAY_SIZE!(tac5xx2_snd_controls),
    dapm_widgets: tac5xx2_common_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(tac5xx2_common_widgets),
    dapm_routes: tac5xx2_common_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(tac5xx2_common_routes),
    idle_bias_on: 0,
    endianness: 1,
    set_jack: None,
};

unsafe fn tac_init(tac_dev: *mut tac5xx2_prv) -> s32 {
    let component_driver: *mut snd_soc_component_driver;
    let dai_drv: *mut snd_soc_dai_driver;
    let num_dais: c_int;
    let ret: s32;

    dev_set_drvdata((*tac_dev).dev, tac_dev as *mut c_void);
    match (*tac_dev).part_id {
        0x5572 => { dai_drv = tac5572_dai_driver.as_mut_ptr(); num_dais = tac5572_dai_driver.len() as c_int; }
        0x5672 => { dai_drv = tac5672_dai_driver.as_mut_ptr(); num_dais = tac5672_dai_driver.len() as c_int; }
        0x5682 => { dai_drv = tac5682_dai_driver.as_mut_ptr(); num_dais = tac5682_dai_driver.len() as c_int; }
        0x2883 => { dai_drv = tas2883_dai_driver.as_mut_ptr(); num_dais = tas2883_dai_driver.len() as c_int; }
        _ => {
            dev_err((*tac_dev).dev, c_str!("Unsupported device: 0x%x\n"), (*tac_dev).part_id);
            return -EINVAL;
        }
    }
    component_driver = devm_kzalloc((*tac_dev).dev, size_of::<snd_soc_component_driver>(), GFP_KERNEL) as *mut snd_soc_component_driver;
    if component_driver.is_null() {
        return -ENOMEM;
    }
    memcpy(component_driver as *mut c_void, &soc_codec_driver_tacdevice as *const _ as *const c_void, size_of::<snd_soc_component_driver>());
    if tac_has_uaj_support(tac_dev) {
        (*component_driver).set_jack = Some(tac5xx2_set_jack);
    }
    ret = devm_snd_soc_register_component((*tac_dev).dev, component_driver, dai_drv, num_dais);
    if ret != 0 {
        dev_err((*tac_dev).dev, c_str!("%s: codec register error:%d.\n"), c_str!("tac_init"), ret);
        return ret;
    }
    0
}

unsafe extern "C" fn tac5xx2_sdca_dev_suspend(dev: *mut device) -> s32 {
    let tac_dev = dev_get_drvdata(dev) as *mut tac5xx2_prv;
    if !(*tac_dev).hw_init {
        return 0;
    }
    regcache_cache_only((*tac_dev).regmap, true);
    0
}

unsafe extern "C" fn tac5xx2_sdca_dev_system_suspend(dev: *mut device) -> s32 {
    tac5xx2_sdca_dev_suspend(dev)
}

unsafe extern "C" fn tac5xx2_sdca_dev_resume(dev: *mut device) -> s32 {
    let tac_dev = dev_get_drvdata(dev) as *mut tac5xx2_prv;
    let slave = dev_to_sdw_dev(dev);
    let mut ret: c_int;
    if !(*tac_dev).first_hw_init_done {
        dev_dbg(dev, c_str!("Device not initialized yet, skipping resume sync\n"));
        return 0;
    }
    ret = sdw_slave_wait_for_init(slave, TAC5XX2_PROBE_TIMEOUT_MS);
    if ret != 0 {
        sdw_show_ping_status((*slave).bus, true);
        return ret;
    }
    regcache_cache_only((*tac_dev).regmap, false);
    regcache_mark_dirty((*tac_dev).regmap);
    ret = regcache_sync((*tac_dev).regmap);
    if ret < 0 {
        dev_warn(dev, c_str!("Failed to sync regcache: %d\n"), ret);
    }
    if tac_has_uaj_support(tac_dev) {
        tac5xx2_sdca_headset_detect(tac_dev);
    }
    0
}

static tac5xx2_sdca_pm: dev_pm_ops = dev_pm_ops { _private: [] };

unsafe fn tac_fw_read_hdr(data: *const u8, hdr: *mut tac_fw_hdr) -> s32 {
    (*hdr).size = get_unaligned_le32(data);
    (*hdr).version_offset = get_unaligned_le32(data.add(4));
    (*hdr).plt_id = get_unaligned_le32(data.add(8));
    (*hdr).ppc3_ver = get_unaligned_le32(data.add(12));
    memcpy((*hdr).ddc_name.as_mut_ptr() as *mut c_void, data.add(16) as *const c_void, 64);
    (*hdr).ddc_name[63] = 0;
    (*hdr).timestamp = get_unaligned_le64(data.add(80));
    TAC_FW_HDR_SIZE as s32
}

unsafe fn tac_fw_get_next_file(data: *const u8, data_size: size_t, file: *mut tac_fw_file) -> s32 {
    let file_length: u32;
    if data_size < TAC_FW_FILE_HDR {
        return -EINVAL;
    }
    (*file).vendor_id = get_unaligned_le32(data.add(0));
    (*file).file_id = get_unaligned_le32(data.add(4));
    (*file).version = get_unaligned_le32(data.add(8));
    (*file).length = get_unaligned_le32(data.add(12));
    (*file).dest_addr = get_unaligned_le32(data.add(16));
    file_length = (*file).length;
    if data_size < TAC_FW_FILE_HDR + file_length as usize {
        return -EINVAL;
    }
    (*file).fw_data = data.add(20) as *mut u8;
    (file_length as usize + size_of::<u32>() * 5) as s32
}

unsafe extern "C" fn tac5xx2_fw_ready(fmw: *const firmware, context: *mut c_void) {
    let tac_dev = context as *mut tac5xx2_prv;
    let mut files: *mut tac_fw_file;
    let fw_hdr_size: u32;
    let mut num_files: u32 = 0;
    let mut hdr: tac_fw_hdr = core::mem::zeroed();
    let mut tm_time: tm = core::mem::zeroed();
    let img_sz: size_t;
    let mut offset: u32;
    let mut ret: s32;
    let buf: *mut u8;

    if fmw.is_null() || (*fmw).data.is_null() || (*fmw).size == 0 || (*fmw).size < TAC_FW_HDR_SIZE + TAC_FW_FILE_HDR {
        dev_err((*tac_dev).dev, c_str!("fw file: %s is empty or invalid\n"), (*tac_dev).fw_binaryname.as_ptr());
        complete_all(&mut (*tac_dev).fw_caching_complete);
        if !fmw.is_null() { release_firmware(fmw); }
        return;
    }
    fw_hdr_size = get_unaligned_le32((*fmw).data);
    if fw_hdr_size as size_t != (*fmw).size {
        dev_err((*tac_dev).dev, c_str!("firmware size mismatch: hdr=%u, actual=%zu\n"), fw_hdr_size, (*fmw).size);
        complete_all(&mut (*tac_dev).fw_caching_complete);
        release_firmware(fmw);
        return;
    }
    files = devm_kzalloc((*tac_dev).dev, size_of::<tac_fw_file>() * TAC_MAX_FW_CHUNKS, GFP_KERNEL) as *mut tac_fw_file;
    buf = devm_kmemdup((*tac_dev).dev, (*fmw).data as *const c_void, (*fmw).size, GFP_KERNEL) as *mut u8;
    if !files.is_null() && !buf.is_null() {
        img_sz = (*fmw).size;
        offset = tac_fw_read_hdr(buf, &mut hdr) as u32;
        while (offset as size_t) < img_sz && (num_files as usize) < TAC_MAX_FW_CHUNKS {
            let file_length: u32;
            if offset as size_t + TAC_FW_FILE_HDR > img_sz {
                dev_warn((*tac_dev).dev, c_str!("Incomplete block header at offset %d\n"), offset);
                break;
            }
            file_length = get_unaligned_le32(buf.add(offset as usize + 12));
            if file_length as size_t > img_sz || offset as size_t > img_sz - TAC_FW_FILE_HDR || file_length as size_t > img_sz - offset as size_t - TAC_FW_FILE_HDR {
                dev_warn((*tac_dev).dev, c_str!("File at offset %d exceeds buffer: length=%u, available=%zu\n"), offset, file_length, img_sz - offset as size_t - TAC_FW_FILE_HDR);
                break;
            }
            ret = tac_fw_get_next_file(buf.add(offset as usize), img_sz - offset as size_t, files.add(num_files as usize));
            if ret < 0 {
                dev_err((*tac_dev).dev, c_str!("Failed to parse file at offset %d\n"), offset);
                break;
            }
            offset += ret as u32;
            num_files += 1;
        }
        if num_files != 0 {
            (*tac_dev).fw_file_cnt = num_files;
            (*tac_dev).fw_files = files;
            time64_to_tm(hdr.timestamp, 0, &mut tm_time);
            dev_dbg((*tac_dev).dev, c_str!("fw file: %s, num_files=%u, ts:%04ld-%02d-%02d %02d:%02d\n"), (*tac_dev).fw_binaryname.as_ptr(), (*tac_dev).fw_file_cnt, tm_time.tm_year + 1900, tm_time.tm_mon + 1, tm_time.tm_mday, tm_time.tm_hour, tm_time.tm_min);
            dev_dbg((*tac_dev).dev, c_str!("fw file: DDC Name: %s\n"), hdr.ddc_name.as_ptr());
            dev_dbg((*tac_dev).dev, c_str!("fw file: PPC3 Version: 3.%ld.%ld.%ld\n"), FIELD_GET(GENMASK(31, 24), hdr.ppc3_ver), FIELD_GET(GENMASK(23, 16), hdr.ppc3_ver), FIELD_GET(GENMASK(15, 8), hdr.ppc3_ver) & 0x3f);
        } else {
            dev_err((*tac_dev).dev, c_str!("firmware with no files\n"));
        }
    }
    complete_all(&mut (*tac_dev).fw_caching_complete);
    release_firmware(fmw);
}

unsafe fn tac_load_and_cache_firmware_async(tac_dev: *mut tac5xx2_prv) -> c_int {
    (*tac_dev).fw_file_cnt = 0;
    (*tac_dev).fw_files = ptr::null_mut(); /* ready to download files */
    request_firmware_nowait(THIS_MODULE, FW_ACTION_UEVENT, (*tac_dev).fw_binaryname.as_ptr(), (*tac_dev).dev, GFP_KERNEL, tac_dev as *mut c_void, Some(tac5xx2_fw_ready))
}

unsafe fn tac_download(tac_dev: *mut tac5xx2_prv) -> c_int {
    let files = (*tac_dev).fw_files;
    let num_files = (*tac_dev).fw_file_cnt;
    let mut i: u32 = 0;
    let mut ret: c_int;
    while i < num_files {
        ret = sdw_nwrite_no_pm((*tac_dev).sdw_peripheral, (*files.add(i as usize)).dest_addr, (*files.add(i as usize)).length, (*files.add(i as usize)).fw_data);
        if ret < 0 {
            dev_dbg((*tac_dev).dev, c_str!("FW write failed at addr 0x%x: %d\n"), (*files.add(i as usize)).dest_addr, ret);
            return ret;
        }
        i += 1;
    }
    0
}

/*
 * tac5xx2 uses custom firmware binary fw.
 * This is not using UMP File Download.
 */
unsafe fn tac_download_fw_to_hw(tac_dev: *mut tac5xx2_prv) -> s32 {
    let ret = tac_download(tac_dev);
    if ret < 0 {
        dev_err((*tac_dev).dev, c_str!("Firmware download failed: %d\n"), ret);
        return ret;
    }
    dev_dbg((*tac_dev).dev, c_str!("Firmware download complete: %d chunks\n"), (*tac_dev).fw_file_cnt);
    (*tac_dev).fw_dl_success = true;
    0
}

unsafe fn tac_get_pci_dev(peripheral: *mut sdw_slave) -> *mut pci_dev {
    let mut dev = &mut (*peripheral).dev as *mut device;
    while !dev.is_null() {
        if dev_is_pci(dev) {
            return to_pci_dev(dev);
        }
        dev = (*(dev as *mut device_with_parent)).parent;
    }
    ptr::null_mut()
}
#[repr(C)]
struct device_with_parent {
    parent: *mut device,
}

unsafe fn tac_generate_fw_name(slave: *mut sdw_slave, name: *mut c_char, size: size_t) {
    let bus = (*slave).bus;
    let part_id: u16 = (*slave).id.part_id;
    let unique_id: u8 = (*slave).id.unique_id;
    let pci = tac_get_pci_dev(slave);
    if !pci.is_null() {
        scnprintf(name, size, c_str!("%04X-%04X-%1X-%1X.bin"), part_id as c_int, (*pci).subsystem_device as c_int, (*bus).link_id as c_int, unique_id as c_int);
    } else {
        /* Default firmware name based on part ID */
        scnprintf(name, size, c_str!("%s%04x-%1X-%1X.bin"), if part_id == 0x2883 { c_str!("tas") } else { c_str!("tac") }, part_id as c_int, (*bus).link_id as c_int, unique_id as c_int);
    }
}

unsafe fn tac_io_init(dev: *mut device, _slave: *mut sdw_slave, first: bool) -> c_int {
    let tac_dev = dev_get_drvdata(dev) as *mut tac5xx2_prv;
    let time: u64;
    let mut ret: c_int;
    if (*tac_dev).hw_init {
        dev_dbg(dev, c_str!("early return hw_init already done.."));
        return 0;
    }
    time = wait_for_completion_timeout(&mut (*tac_dev).fw_caching_complete, msecs_to_jiffies(TAC5XX2_FW_CACHE_TIMEOUT_MS));
    if time == 0 {
        ret = -ETIMEDOUT;
        dev_warn((*tac_dev).dev, c_str!("%s: fw caching timeout\n"), c_str!("tac_io_init"));
        dev_err(dev, c_str!("init writes failed, err=%d"), ret);
        return ret;
    }
    if (*tac_dev).rev_id == 0 {
        ret = regmap_read((*tac_dev).regmap, TAC_REV_ID, &mut (*tac_dev).rev_id);
        if ret != 0 {
            dev_err((*tac_dev).dev, c_str!("failed to rev id, err=%d\n"), ret);
            dev_err(dev, c_str!("init writes failed, err=%d"), ret);
            return ret;
        }
        dev_dbg((*tac_dev).dev, c_str!("detected rev_id 0x%x"), (*tac_dev).rev_id);
    }
    if !(*tac_dev).fw_files.is_null() && (*tac_dev).fw_file_cnt > 0 {
        ret = tac_download_fw_to_hw(tac_dev);
        if ret != 0 {
            dev_err((*tac_dev).dev, c_str!("FW download failed, fw: %d\n"), ret);
            dev_err(dev, c_str!("init writes failed, err=%d"), ret);
            return ret;
        }
    }
    if !(*tac_dev).sa_func_data.is_null() {
        ret = sdca_regmap_write_init(dev, (*tac_dev).regmap, (*tac_dev).sa_func_data);
        if ret != 0 { dev_err(dev, c_str!("smartamp init table update failed\n")); dev_err(dev, c_str!("init writes failed, err=%d"), ret); return ret; }
        dev_dbg(dev, c_str!("smartamp init done\n"));
        if first {
            ret = regmap_multi_reg_write((*tac_dev).regmap, tac_spk_seq.as_ptr(), ARRAY_SIZE!(tac_spk_seq));
            if ret != 0 { dev_err(dev, c_str!("init writes failed, err=%d"), ret); return ret; }
        }
    }
    if !(*tac_dev).sm_func_data.is_null() {
        ret = sdca_regmap_write_init(dev, (*tac_dev).regmap, (*tac_dev).sm_func_data);
        if ret != 0 { dev_err(dev, c_str!("smartmic init table update failed\n")); dev_err(dev, c_str!("init writes failed, err=%d"), ret); return ret; }
        dev_dbg(dev, c_str!("smartmic init done\n"));
    }
    if !(*tac_dev).uaj_func_data.is_null() {
        ret = sdca_regmap_write_init(dev, (*tac_dev).regmap, (*tac_dev).uaj_func_data);
        if ret != 0 { dev_err(dev, c_str!("uaj init table update failed\n")); dev_err(dev, c_str!("init writes failed, err=%d"), ret); return ret; }
        dev_dbg(dev, c_str!("uaj init done\n"));
        if first && !(*tac_dev).hs_jack.is_null() {
            ret = tac5xx2_jack_init(tac_dev);
            if ret != 0 { dev_err((*tac_dev).dev, c_str!("jack init failed")); dev_err(dev, c_str!("init writes failed, err=%d"), ret); return ret; }
        }
    }
    if !(*tac_dev).hid_func_data.is_null() {
        ret = sdca_regmap_write_init(dev, (*tac_dev).regmap, (*tac_dev).hid_func_data);
        if ret != 0 { dev_err(dev, c_str!("hid init table update failed\n")); dev_err(dev, c_str!("init writes failed, err=%d"), ret); return ret; }
        dev_dbg(dev, c_str!("hid init done\n"));
    }
    (*tac_dev).hw_init = true;
    0
}

unsafe extern "C" fn tac_update_status(slave: *mut sdw_slave, status: sdw_slave_status) -> c_int {
    let tac_dev = dev_get_drvdata(&mut (*slave).dev) as *mut tac5xx2_prv;
    let dev = &mut (*slave).dev as *mut device;
    let mut first = false;
    let mut ret: c_int;
    (*tac_dev).status = status;
    if status == SDW_SLAVE_UNATTACHED {
        (*tac_dev).hw_init = false;
        (*tac_dev).fw_dl_success = false;
    }
    if (*tac_dev).hw_init || (*tac_dev).status != SDW_SLAVE_ATTACHED {
        dev_dbg(dev, c_str!("%s: early return, hw_init=%d, status=%d"), c_str!("tac_update_status"), (*tac_dev).hw_init as c_int, (*tac_dev).status);
        return 0;
    }
    if !(*tac_dev).first_hw_init_done {
        pm_runtime_set_active((*tac_dev).dev);
        (*tac_dev).first_hw_init_done = true;
        first = true;
    }
    pm_runtime_get_noresume((*tac_dev).dev);
    regcache_mark_dirty((*tac_dev).regmap);
    regcache_cache_only((*tac_dev).regmap, false);
    ret = tac_io_init(&mut (*slave).dev, slave, first);
    if ret != 0 {
        dev_err(dev, c_str!("Device initialization failed: %d\n"), ret);
    } else {
        ret = regcache_sync((*tac_dev).regmap);
        if ret != 0 {
            dev_warn(dev, c_str!("Failed to sync regcache after init: %d\n"), ret);
        }
    }
    pm_runtime_mark_last_busy((*tac_dev).dev);
    pm_runtime_put_autosuspend((*tac_dev).dev);
    ret
}

unsafe extern "C" fn tac5xx2_sdw_read_prop(peripheral: *mut sdw_slave) -> c_int {
    let dev = &mut (*peripheral).dev as *mut device;
    let ret = sdw_slave_read_prop(peripheral);
    if ret != 0 {
        dev_err(dev, c_str!("sdw_slave_read_prop failed: %d"), ret);
        return ret;
    }
    0
}

unsafe extern "C" fn tac_port_prep(slave: *mut sdw_slave, _prep_ch: *mut sdw_prepare_ch, pre_ops: sdw_port_prep_ops) -> c_int {
    let dev = &mut (*slave).dev as *mut device;
    let tac_dev = dev_get_drvdata(dev) as *mut tac5xx2_prv;
    let mut val: c_uint = 0;
    let mut ret: c_int;
    if pre_ops != SDW_OPS_PORT_POST_PREP {
        return 0;
    }
    if !(*tac_dev).fw_dl_success {
        return 0;
    }
    ret = regmap_read((*tac_dev).regmap, TAC_DSP_ALGO_STATUS, &mut val);
    if ret != 0 {
        dev_err(dev, c_str!("Failed to read algo status: %d\n"), ret);
        return ret;
    }
    if val != TAC_DSP_ALGO_STATUS_RUNNING {
        dev_dbg(dev, c_str!("Algo not running (0x%02x), re-enabling\n"), val);
        ret = regmap_write((*tac_dev).regmap, TAC_DSP_ALGO_STATUS, TAC_DSP_ALGO_STATUS_RUNNING);
        if ret != 0 {
            dev_err(dev, c_str!("Failed to re-enable algo: %d\n"), ret);
            return ret;
        }
    }
    0
}

static tac_sdw_ops: sdw_slave_ops = sdw_slave_ops {
    read_prop: Some(tac5xx2_sdw_read_prop),
    update_status: Some(tac_update_status),
    interrupt_callback: Some(tac_interrupt_callback),
    port_prep: Some(tac_port_prep),
};

unsafe extern "C" fn tac_sdw_probe(peripheral: *mut sdw_slave, id: *const sdw_device_id) -> s32 {
    let mut function_data: *mut sdca_function_data = ptr::null_mut();
    let dev = &mut (*peripheral).dev as *mut device;
    let tac_dev: *mut tac5xx2_prv;
    let regmap: *mut regmap;
    let mut ret: c_int;
    let mut i: c_int;

    tac_dev = devm_kzalloc(dev, size_of::<tac5xx2_prv>(), GFP_KERNEL) as *mut tac5xx2_prv;
    if tac_dev.is_null() {
        return dev_err_probe(dev, -ENOMEM, c_str!("Failed devm_kzalloc"));
    }
    if (*peripheral).sdca_data.num_functions > 0 {
        dev_dbg(dev, c_str!("SDCA functions found: %d"), (*peripheral).sdca_data.num_functions);
        i = 0;
        while i < (*peripheral).sdca_data.num_functions {
            let mut func_ptr: *mut *mut sdca_function_data;
            let func_name: *const c_char;
            let function = (*peripheral).sdca_data.function.add(i as usize);
            if (*function).type_ == SDCA_FUNCTION_TYPE_SMART_AMP {
                func_ptr = &mut (*tac_dev).sa_func_data;
                func_name = c_str!("smartamp");
            } else if (*function).type_ == SDCA_FUNCTION_TYPE_SMART_MIC {
                func_ptr = &mut (*tac_dev).sm_func_data;
                func_name = c_str!("smartmic");
            } else if (*function).type_ == SDCA_FUNCTION_TYPE_UAJ {
                func_ptr = &mut (*tac_dev).uaj_func_data;
                func_name = c_str!("uaj");
            } else if (*function).type_ == SDCA_FUNCTION_TYPE_HID {
                func_ptr = &mut (*tac_dev).hid_func_data;
                func_name = c_str!("hid");
            } else {
                i += 1;
                continue;
            }
            function_data = devm_kzalloc(dev, size_of::<sdca_function_data>(), GFP_KERNEL) as *mut sdca_function_data;
            if function_data.is_null() {
                return dev_err_probe(dev, -ENOMEM, c_str!("failed to allocate %s function data"), func_name);
            }
            (*function_data).desc = function;
            ret = sdca_parse_function(dev, function_data);
            if ret == 0 {
                *func_ptr = function_data;
            } else {
                devm_kfree(dev, function_data as *mut c_void);
            }
            i += 1;
        }
    }
    dev_dbg(dev, c_str!("SDCA functions enabled: SA=%s SM=%s UAJ=%s HID=%s"), if !(*tac_dev).sa_func_data.is_null() { c_str!("yes") } else { c_str!("no") }, if !(*tac_dev).sm_func_data.is_null() { c_str!("yes") } else { c_str!("no") }, if !(*tac_dev).uaj_func_data.is_null() { c_str!("yes") } else { c_str!("no") }, if !(*tac_dev).hid_func_data.is_null() { c_str!("yes") } else { c_str!("no") });
    (*tac_dev).dev = dev;
    (*tac_dev).sdw_peripheral = peripheral;
    (*tac_dev).hw_init = false;
    (*tac_dev).first_hw_init_done = false;
    (*tac_dev).part_id = (*id).part_id as u32;
    (*tac_dev).rev_id = 0x0;
    dev_set_drvdata(dev, tac_dev as *mut c_void);
    regmap = devm_regmap_init_sdw_mbq_cfg(&mut (*peripheral).dev, peripheral, &tac_regmap, &tac_mbq_cfg);
    if IS_ERR(regmap as *const c_void) {
        return dev_err_probe(dev, PTR_ERR(regmap as *const c_void), c_str!("Failed devm_regmap_init_sdw\n"));
    }
    regcache_cache_only(regmap, true);
    (*tac_dev).regmap = regmap;
    (*tac_dev).jack_type = 0;
    init_completion(&mut (*tac_dev).fw_caching_complete);
    tac_generate_fw_name(peripheral, (*tac_dev).fw_binaryname.as_mut_ptr() as *mut c_char, size_of::<[u8; 64]>());
    ret = tac_load_and_cache_firmware_async(tac_dev);
    if ret != 0 {
        complete_all(&mut (*tac_dev).fw_caching_complete);
        dev_dbg(dev, c_str!("failed to load fw: %d, use rom mode\n"), ret);
    }
    ret = tac_init(tac_dev);
    if ret != 0 {
        return dev_err_probe(dev, ret, c_str!("failed to initialize tac device\n"));
    }
    /* set autosuspend parameters */
    pm_runtime_set_autosuspend_delay(dev, 3000);
    pm_runtime_use_autosuspend(dev);
    /* make sure the device does not suspend immediately */
    pm_runtime_mark_last_busy(dev);
    pm_runtime_enable(dev);
    /* the device is still not in active */
    0
}

unsafe extern "C" fn tac_sdw_remove(peripheral: *mut sdw_slave) {
    let tac_dev = dev_get_drvdata(&mut (*peripheral).dev) as *mut tac5xx2_prv;
    pm_runtime_disable((*tac_dev).dev);
    dev_set_drvdata(&mut (*peripheral).dev, ptr::null_mut());
}

static tac_sdw_id: [sdw_device_id; 5] = [
    sdw_device_id { mfg_id: 0x0102, part_id: 0x5572, sdw_version: 0 },
    sdw_device_id { mfg_id: 0x0102, part_id: 0x5672, sdw_version: 0 },
    sdw_device_id { mfg_id: 0x0102, part_id: 0x5682, sdw_version: 0 },
    sdw_device_id { mfg_id: 0x0102, part_id: 0x2883, sdw_version: 0 },
    sdw_device_id { mfg_id: 0, part_id: 0, sdw_version: 0 },
];

static mut tac_sdw_driver: sdw_driver = sdw_driver {
    driver: sdw_driver_driver {
        name: c_str!("slave-tac5xx2"),
        pm: &tac5xx2_sdca_pm,
    },
    probe: Some(tac_sdw_probe),
    remove: Some(tac_sdw_remove),
    ops: &tac_sdw_ops,
    id_table: tac_sdw_id.as_ptr(),
};

/* MODULE_DEVICE_TABLE(sdw, tac_sdw_id); */
/* module_sdw_driver(tac_sdw_driver); */
/* MODULE_IMPORT_NS("SND_SOC_SDCA"); */
/* MODULE_AUTHOR("Texas Instruments Inc."); */
/* MODULE_DESCRIPTION("ASoC TAC5XX2 SoundWire Driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
