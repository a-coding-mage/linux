// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2020, Maxim Integrated

// Translated from the SoundWire MAX98373 C implementation source.
// C include dependencies are intentionally not reimplemented here:
// linux/acpi.h, linux/delay.h, linux/module.h, linux/pm_runtime.h,
// linux/regmap.h, linux/slab.h, sound/pcm.h, sound/pcm_params.h,
// sound/sdw.h, sound/soc.h, sound/tlv.h, linux/of.h,
// linux/soundwire/sdw.h, linux/soundwire/sdw_type.h,
// linux/soundwire/sdw_registers.h, max98373.h, max98373-sdw.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u32 = c_uint;

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
    pub source_ports: c_ulong,
    pub sink_ports: c_ulong,
    pub paging_support: bool_,
    pub clk_stop_timeout: c_uint,
    pub src_dpn_prop: *mut sdw_dpn_prop,
    pub sink_dpn_prop: *mut sdw_dpn_prop,
}

#[repr(C)]
pub struct sdw_dpn_prop {
    pub num: c_uint,
    pub type_: c_uint,
    pub simple_ch_prep_sm: bool_,
    pub ch_prep_timeout: c_uint,
}

#[repr(C)]
pub struct sdw_bus_params {
    pub curr_dr_freq: c_uint,
}

#[repr(C)]
pub struct sdw_stream_config {
    pub ch_count: c_uint,
}

#[repr(C)]
pub struct sdw_port_config {
    pub num: c_uint,
    pub ch_mask: c_uint,
}

#[repr(C)]
pub struct sdw_stream_runtime {
    _private: [u8; 0],
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
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub dev: *mut device,
}

#[repr(C)]
pub struct sdw_device_id {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct acpi_device_id {
    pub id: [c_char; 8],
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct max98373_cache {
    pub reg: c_uint,
    pub val: c_uint,
}

#[repr(C)]
pub struct max98373_priv {
    pub regmap: *mut regmap,
    pub slave: *mut sdw_slave,
    pub cache_num: c_int,
    pub cache: *mut max98373_cache,
    pub first_hw_init: bool_,
    pub hw_init: bool_,
    pub i_slot: c_uint,
    pub v_slot: c_uint,
    pub spkfb_slot: c_uint,
    pub interleave_mode: bool_,
    pub ch_size: c_int,
    pub rx_mask: c_uint,
    pub slot: c_int,
    pub tdm_mode: bool_,
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
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub cache_type: c_uint,
    pub use_single_read: bool_,
    pub use_single_write: bool_,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub set_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, *mut c_void, c_int) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
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
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_slave_ops {
    pub read_prop: Option<unsafe extern "C" fn(*mut sdw_slave) -> c_int>,
    pub update_status: Option<unsafe extern "C" fn(*mut sdw_slave, sdw_slave_status) -> c_int>,
    pub bus_config: Option<unsafe extern "C" fn(*mut sdw_slave, *mut sdw_bus_params) -> c_int>,
}

pub type sdw_slave_status = c_uint;

#[repr(C)]
pub struct driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub acpi_match_table: *const acpi_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct sdw_driver {
    pub driver: driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut sdw_slave, *const sdw_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut sdw_slave)>,
    pub ops: *const sdw_slave_ops,
    pub id_table: *const sdw_device_id,
}

extern "C" {
    static soc_codec_dev_max98373_sdw: snd_soc_component_driver;

    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_sdw(slave: *mut sdw_slave, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regcache_cache_bypass(map: *mut regmap, enable: bool_);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn sdw_slave_wait_for_init(slave: *mut sdw_slave, timeout: c_int) -> c_int;
    fn sdw_show_ping_status(bus: *mut sdw_bus, status: bool_);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_get_noresume(dev: *mut device);
    fn pm_runtime_put_autosuspend(dev: *mut device);
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn max98373_reset(max98373: *mut max98373_priv, dev: *mut device);
    fn max98373_slot_config(dev: *mut device, max98373: *mut max98373_priv);
    fn snd_sdw_params_to_config(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, stream_config: *mut sdw_stream_config, port_config: *mut sdw_port_config);
    fn snd_soc_dai_get_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream) -> *mut sdw_stream_runtime;
    fn snd_soc_dai_dma_data_set(dai: *mut snd_soc_dai, direction: c_int, data: *mut c_void);
    fn snd_soc_dai_set_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream, data: *mut c_void);
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn sdw_stream_add_slave(slave: *mut sdw_slave, stream_config: *mut sdw_stream_config, port_config: *mut sdw_port_config, num_ports: c_uint, stream: *mut sdw_stream_runtime) -> c_int;
    fn sdw_stream_remove_slave(slave: *mut sdw_slave, stream: *mut sdw_stream_runtime);
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_pcm_format_width(format: c_uint) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
}

extern "C" {
    static max98373_of_match: [of_device_id; 2];
    static max98373_acpi_match: [acpi_device_id; 2];
}

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_RBTREE: c_uint = 0;
const SDW_SCP_INT1_BUS_CLASH: c_uint = 0;
const SDW_SCP_INT1_PARITY: c_uint = 0;
const SDW_DPN_FULL: c_uint = 0;
const SDW_SLAVE_UNATTACHED: sdw_slave_status = 0;
const SDW_SLAVE_ATTACHED: sdw_slave_status = 1;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_RATE_8000_96000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 0;

// Register and bitfield constants are provided by max98373.h/max98373-sdw.h.
extern "C" {
    static MAX98373_R0040_SCP_INIT_STAT_1: c_uint;
    static MAX98373_R0041_SCP_INIT_MASK_1: c_uint;
    static MAX98373_R0042_SCP_INIT_STAT_2: c_uint;
    static MAX98373_R0044_SCP_CTRL: c_uint;
    static MAX98373_R0045_SCP_SYSTEM_CTRL: c_uint;
    static MAX98373_R0046_SCP_DEV_NUMBER: c_uint;
    static MAX98373_R0050_SCP_DEV_ID_0: c_uint;
    static MAX98373_R0051_SCP_DEV_ID_1: c_uint;
    static MAX98373_R0052_SCP_DEV_ID_2: c_uint;
    static MAX98373_R0053_SCP_DEV_ID_3: c_uint;
    static MAX98373_R0054_SCP_DEV_ID_4: c_uint;
    static MAX98373_R0055_SCP_DEV_ID_5: c_uint;
    static MAX98373_R0060_SCP_FRAME_CTLR: c_uint;
    static MAX98373_R0070_SCP_FRAME_CTLR: c_uint;
    static MAX98373_R0100_DP1_INIT_STAT: c_uint;
    static MAX98373_R0137_DP1_BLOCK_CTRL3: c_uint;
    static MAX98373_R0300_DP3_INIT_STAT: c_uint;
    static MAX98373_R0337_DP3_BLOCK_CTRL3: c_uint;
    static MAX98373_R2000_SW_RESET: c_uint;
    static MAX98373_R200C_INT_EN3: c_uint;
    static MAX98373_R2009_INT_FLAG3: c_uint;
    static MAX98373_R2010_IRQ_CTRL: c_uint;
    static MAX98373_R2014_THERM_WARN_THRESH: c_uint;
    static MAX98373_R2018_THERM_FOLDBACK_EN: c_uint;
    static MAX98373_R201E_PIN_DRIVE_STRENGTH: c_uint;
    static MAX98373_R2036_SOUNDWIRE_CTRL: c_uint;
    static MAX98373_R203D_AMP_DIG_VOL_CTRL: c_uint;
    static MAX98373_R2043_AMP_EN: c_uint;
    static MAX98373_R2046_IV_SENSE_ADC_DSP_CFG: c_uint;
    static MAX98373_R2047_IV_SENSE_ADC_EN: c_uint;
    static MAX98373_R2051_MEAS_ADC_SAMPLING_RATE: c_uint;
    static MAX98373_R2054_MEAS_ADC_PVDD_CH_READBACK: c_uint;
    static MAX98373_R2055_MEAS_ADC_THERM_CH_READBACK: c_uint;
    static MAX98373_R2056_MEAS_ADC_PVDD_CH_EN: c_uint;
    static MAX98373_R2090_BDE_LVL_HOLD: c_uint;
    static MAX98373_R2092_BDE_CLIPPER_MODE: c_uint;
    static MAX98373_R2097_BDE_L1_THRESH: c_uint;
    static MAX98373_R209B_BDE_THRESH_HYST: c_uint;
    static MAX98373_R20A8_BDE_L1_CFG_1: c_uint;
    static MAX98373_R20B3_BDE_L4_CFG_3: c_uint;
    static MAX98373_R20B5_BDE_EN: c_uint;
    static MAX98373_R20B6_BDE_CUR_STATE_READBACK: c_uint;
    static MAX98373_R20D1_DHT_CFG: c_uint;
    static MAX98373_R20D4_DHT_EN: c_uint;
    static MAX98373_R20E0_LIMITER_THRESH_CFG: c_uint;
    static MAX98373_R20E2_LIMITER_EN: c_uint;
    static MAX98373_R20FE_DEVICE_AUTO_RESTART_CFG: c_uint;
    static MAX98373_R20FF_GLOBAL_SHDN: c_uint;
    static MAX98373_R21FF_REV_ID: c_uint;
    static MAX98373_R2020_PCM_TX_HIZ_EN_1: c_uint;
    static MAX98373_R2021_PCM_TX_HIZ_EN_2: c_uint;
    static MAX98373_R2022_PCM_TX_SRC_1: c_uint;
    static MAX98373_R2023_PCM_TX_SRC_2: c_uint;
    static MAX98373_R2024_PCM_DATA_FMT_CFG: c_uint;
    static MAX98373_R2025_AUDIO_IF_MODE: c_uint;
    static MAX98373_R2028_PCM_SR_SETUP_2: c_uint;
    static MAX98373_R2029_PCM_TO_SPK_MONO_MIX_1: c_uint;
    static MAX98373_R202A_PCM_TO_SPK_MONO_MIX_2: c_uint;
    static MAX98373_R203F_AMP_DSP_CFG: c_uint;
    static MAX98373_PCM_TX_CH_SRC_A_I_SHIFT: c_uint;
    static MAX98373_PCM_TX_CH_INTERLEAVE_MASK: c_uint;
    static MAX98373_SPK_EN_MASK: c_uint;
    static MAX98373_PCM_MODE_CFG_CHANSZ_16: c_int;
    static MAX98373_PCM_MODE_CFG_CHANSZ_24: c_int;
    static MAX98373_PCM_MODE_CFG_CHANSZ_32: c_int;
    static MAX98373_PCM_MODE_CFG_CHANSZ_MASK: c_uint;
    static MAX98373_PCM_SR_SET1_SR_8000: c_int;
    static MAX98373_PCM_SR_SET1_SR_11025: c_int;
    static MAX98373_PCM_SR_SET1_SR_12000: c_int;
    static MAX98373_PCM_SR_SET1_SR_16000: c_int;
    static MAX98373_PCM_SR_SET1_SR_22050: c_int;
    static MAX98373_PCM_SR_SET1_SR_24000: c_int;
    static MAX98373_PCM_SR_SET1_SR_32000: c_int;
    static MAX98373_PCM_SR_SET1_SR_44100: c_int;
    static MAX98373_PCM_SR_SET1_SR_48000: c_int;
    static MAX98373_PCM_SR_SET1_SR_88200: c_int;
    static MAX98373_PCM_SR_SET1_SR_96000: c_int;
    static MAX98373_PCM_SR_SET2_SR_MASK: c_uint;
    static MAX98373_PCM_SR_SET2_SR_SHIFT: c_uint;
    static MAX98373_PCM_SR_SET2_IVADC_SR_MASK: c_uint;
}

const fn BIT(n: c_uint) -> c_ulong {
    1usize.wrapping_shl(n) as c_ulong
}

const fn GENMASK(h: c_int, l: c_int) -> c_uint {
    if h < l {
        0
    } else {
        ((!0u32).wrapping_shl(l as u32)) & ((!0u32).wrapping_shr((31 - h) as u32))
    }
}

fn hweight32(mut w: c_ulong) -> c_int {
    let mut count = 0;
    w &= 0xffff_ffff;
    while w != 0 {
        count += (w & 1) as c_int;
        w >>= 1;
    }
    count
}

const MAX98373_PROBE_TIMEOUT: c_int = 5000;
const MAX98373_RATES: c_uint = SNDRV_PCM_RATE_8000_96000;
const MAX98373_FORMATS: u64 = SNDRV_PCM_FMTBIT_S32_LE;

static max98373_sdw_cache_reg: [u32; 3] = unsafe {
    [
        MAX98373_R2054_MEAS_ADC_PVDD_CH_READBACK,
        MAX98373_R2055_MEAS_ADC_THERM_CH_READBACK,
        MAX98373_R20B6_BDE_CUR_STATE_READBACK,
    ]
};

static max98373_reg: [reg_default; 141] = unsafe {
    [
        reg_default { reg: MAX98373_R0040_SCP_INIT_STAT_1, def: 0x00 },
        reg_default { reg: MAX98373_R0041_SCP_INIT_MASK_1, def: 0x00 },
        reg_default { reg: MAX98373_R0042_SCP_INIT_STAT_2, def: 0x00 },
        reg_default { reg: MAX98373_R0044_SCP_CTRL, def: 0x00 },
        reg_default { reg: MAX98373_R0045_SCP_SYSTEM_CTRL, def: 0x00 },
        reg_default { reg: MAX98373_R0046_SCP_DEV_NUMBER, def: 0x00 },
        reg_default { reg: MAX98373_R0050_SCP_DEV_ID_0, def: 0x21 },
        reg_default { reg: MAX98373_R0051_SCP_DEV_ID_1, def: 0x01 },
        reg_default { reg: MAX98373_R0052_SCP_DEV_ID_2, def: 0x9F },
        reg_default { reg: MAX98373_R0053_SCP_DEV_ID_3, def: 0x87 },
        reg_default { reg: MAX98373_R0054_SCP_DEV_ID_4, def: 0x08 },
        reg_default { reg: MAX98373_R0055_SCP_DEV_ID_5, def: 0x00 },
        reg_default { reg: MAX98373_R0060_SCP_FRAME_CTLR, def: 0x00 },
        reg_default { reg: MAX98373_R0070_SCP_FRAME_CTLR, def: 0x00 },
        reg_default { reg: MAX98373_R0100_DP1_INIT_STAT, def: 0x00 },
        reg_default { reg: MAX98373_R0101_DP1_INIT_MASK, def: 0x00 },
        reg_default { reg: MAX98373_R0102_DP1_PORT_CTRL, def: 0x00 },
        reg_default { reg: MAX98373_R0103_DP1_BLOCK_CTRL_1, def: 0x00 },
        reg_default { reg: MAX98373_R0104_DP1_PREPARE_STATUS, def: 0x00 },
        reg_default { reg: MAX98373_R0105_DP1_PREPARE_CTRL, def: 0x00 },
        reg_default { reg: MAX98373_R0120_DP1_CHANNEL_EN, def: 0x00 },
        reg_default { reg: MAX98373_R0122_DP1_SAMPLE_CTRL1, def: 0x00 },
        reg_default { reg: MAX98373_R0123_DP1_SAMPLE_CTRL2, def: 0x00 },
        reg_default { reg: MAX98373_R0124_DP1_OFFSET_CTRL1, def: 0x00 },
        reg_default { reg: MAX98373_R0125_DP1_OFFSET_CTRL2, def: 0x00 },
        reg_default { reg: MAX98373_R0126_DP1_HCTRL, def: 0x00 },
        reg_default { reg: MAX98373_R0127_DP1_BLOCK_CTRL3, def: 0x00 },
        reg_default { reg: MAX98373_R0130_DP1_CHANNEL_EN, def: 0x00 },
        reg_default { reg: MAX98373_R0132_DP1_SAMPLE_CTRL1, def: 0x00 },
        reg_default { reg: MAX98373_R0133_DP1_SAMPLE_CTRL2, def: 0x00 },
        reg_default { reg: MAX98373_R0134_DP1_OFFSET_CTRL1, def: 0x00 },
        reg_default { reg: MAX98373_R0135_DP1_OFFSET_CTRL2, def: 0x00 },
        reg_default { reg: MAX98373_R0136_DP1_HCTRL, def: 0x0136 },
        reg_default { reg: MAX98373_R0137_DP1_BLOCK_CTRL3, def: 0x00 },
        reg_default { reg: MAX98373_R0300_DP3_INIT_STAT, def: 0x00 },
        reg_default { reg: MAX98373_R0301_DP3_INIT_MASK, def: 0x00 },
        reg_default { reg: MAX98373_R0302_DP3_PORT_CTRL, def: 0x00 },
        reg_default { reg: MAX98373_R0303_DP3_BLOCK_CTRL_1, def: 0x00 },
        reg_default { reg: MAX98373_R0304_DP3_PREPARE_STATUS, def: 0x00 },
        reg_default { reg: MAX98373_R0305_DP3_PREPARE_CTRL, def: 0x00 },
        reg_default { reg: MAX98373_R0320_DP3_CHANNEL_EN, def: 0x00 },
        reg_default { reg: MAX98373_R0322_DP3_SAMPLE_CTRL1, def: 0x00 },
        reg_default { reg: MAX98373_R0323_DP3_SAMPLE_CTRL2, def: 0x00 },
        reg_default { reg: MAX98373_R0324_DP3_OFFSET_CTRL1, def: 0x00 },
        reg_default { reg: MAX98373_R0325_DP3_OFFSET_CTRL2, def: 0x00 },
        reg_default { reg: MAX98373_R0326_DP3_HCTRL, def: 0x00 },
        reg_default { reg: MAX98373_R0327_DP3_BLOCK_CTRL3, def: 0x00 },
        reg_default { reg: MAX98373_R0330_DP3_CHANNEL_EN, def: 0x00 },
        reg_default { reg: MAX98373_R0332_DP3_SAMPLE_CTRL1, def: 0x00 },
        reg_default { reg: MAX98373_R0333_DP3_SAMPLE_CTRL2, def: 0x00 },
        reg_default { reg: MAX98373_R0334_DP3_OFFSET_CTRL1, def: 0x00 },
        reg_default { reg: MAX98373_R0335_DP3_OFFSET_CTRL2, def: 0x00 },
        reg_default { reg: MAX98373_R0336_DP3_HCTRL, def: 0x00 },
        reg_default { reg: MAX98373_R0337_DP3_BLOCK_CTRL3, def: 0x00 },
        reg_default { reg: MAX98373_R2000_SW_RESET, def: 0x00 },
        reg_default { reg: MAX98373_R2001_INT_RAW1, def: 0x00 },
        reg_default { reg: MAX98373_R2002_INT_RAW2, def: 0x00 },
        reg_default { reg: MAX98373_R2003_INT_RAW3, def: 0x00 },
        reg_default { reg: MAX98373_R2004_INT_STATE1, def: 0x00 },
        reg_default { reg: MAX98373_R2005_INT_STATE2, def: 0x00 },
        reg_default { reg: MAX98373_R2006_INT_STATE3, def: 0x00 },
        reg_default { reg: MAX98373_R2007_INT_FLAG1, def: 0x00 },
        reg_default { reg: MAX98373_R2008_INT_FLAG2, def: 0x00 },
        reg_default { reg: MAX98373_R2009_INT_FLAG3, def: 0x00 },
        reg_default { reg: MAX98373_R200A_INT_EN1, def: 0x00 },
        reg_default { reg: MAX98373_R200B_INT_EN2, def: 0x00 },
        reg_default { reg: MAX98373_R200C_INT_EN3, def: 0x00 },
        reg_default { reg: MAX98373_R200D_INT_FLAG_CLR1, def: 0x00 },
        reg_default { reg: MAX98373_R200E_INT_FLAG_CLR2, def: 0x00 },
        reg_default { reg: MAX98373_R200F_INT_FLAG_CLR3, def: 0x00 },
        reg_default { reg: MAX98373_R2010_IRQ_CTRL, def: 0x00 },
        reg_default { reg: MAX98373_R2014_THERM_WARN_THRESH, def: 0x10 },
        reg_default { reg: MAX98373_R2015_THERM_SHDN_THRESH, def: 0x27 },
        reg_default { reg: MAX98373_R2016_THERM_HYSTERESIS, def: 0x01 },
        reg_default { reg: MAX98373_R2017_THERM_FOLDBACK_SET, def: 0xC0 },
        reg_default { reg: MAX98373_R2018_THERM_FOLDBACK_EN, def: 0x00 },
        reg_default { reg: MAX98373_R201E_PIN_DRIVE_STRENGTH, def: 0x55 },
        reg_default { reg: MAX98373_R2020_PCM_TX_HIZ_EN_1, def: 0xFE },
        reg_default { reg: MAX98373_R2021_PCM_TX_HIZ_EN_2, def: 0xFF },
        reg_default { reg: MAX98373_R2022_PCM_TX_SRC_1, def: 0x00 },
        reg_default { reg: MAX98373_R2023_PCM_TX_SRC_2, def: 0x00 },
        reg_default { reg: MAX98373_R2024_PCM_DATA_FMT_CFG, def: 0xC0 },
        reg_default { reg: MAX98373_R2025_AUDIO_IF_MODE, def: 0x00 },
        reg_default { reg: MAX98373_R2026_PCM_CLOCK_RATIO, def: 0x04 },
        reg_default { reg: MAX98373_R2027_PCM_SR_SETUP_1, def: 0x08 },
        reg_default { reg: MAX98373_R2028_PCM_SR_SETUP_2, def: 0x88 },
        reg_default { reg: MAX98373_R2029_PCM_TO_SPK_MONO_MIX_1, def: 0x00 },
        reg_default { reg: MAX98373_R202A_PCM_TO_SPK_MONO_MIX_2, def: 0x00 },
        reg_default { reg: MAX98373_R202B_PCM_RX_EN, def: 0x00 },
        reg_default { reg: MAX98373_R202C_PCM_TX_EN, def: 0x00 },
        reg_default { reg: MAX98373_R202E_ICC_RX_CH_EN_1, def: 0x00 },
        reg_default { reg: MAX98373_R202F_ICC_RX_CH_EN_2, def: 0x00 },
        reg_default { reg: MAX98373_R2030_ICC_TX_HIZ_EN_1, def: 0xFF },
        reg_default { reg: MAX98373_R2031_ICC_TX_HIZ_EN_2, def: 0xFF },
        reg_default { reg: MAX98373_R2032_ICC_LINK_EN_CFG, def: 0x30 },
        reg_default { reg: MAX98373_R2034_ICC_TX_CNTL, def: 0x00 },
        reg_default { reg: MAX98373_R2035_ICC_TX_EN, def: 0x00 },
        reg_default { reg: MAX98373_R2036_SOUNDWIRE_CTRL, def: 0x05 },
        reg_default { reg: MAX98373_R203D_AMP_DIG_VOL_CTRL, def: 0x00 },
        reg_default { reg: MAX98373_R203E_AMP_PATH_GAIN, def: 0x08 },
        reg_default { reg: MAX98373_R203F_AMP_DSP_CFG, def: 0x02 },
        reg_default { reg: MAX98373_R2040_TONE_GEN_CFG, def: 0x00 },
        reg_default { reg: MAX98373_R2041_AMP_CFG, def: 0x03 },
        reg_default { reg: MAX98373_R2042_AMP_EDGE_RATE_CFG, def: 0x00 },
        reg_default { reg: MAX98373_R2043_AMP_EN, def: 0x00 },
        reg_default { reg: MAX98373_R2046_IV_SENSE_ADC_DSP_CFG, def: 0x04 },
        reg_default { reg: MAX98373_R2047_IV_SENSE_ADC_EN, def: 0x00 },
        reg_default { reg: MAX98373_R2051_MEAS_ADC_SAMPLING_RATE, def: 0x00 },
        reg_default { reg: MAX98373_R2052_MEAS_ADC_PVDD_FLT_CFG, def: 0x00 },
        reg_default { reg: MAX98373_R2053_MEAS_ADC_THERM_FLT_CFG, def: 0x00 },
        reg_default { reg: MAX98373_R2054_MEAS_ADC_PVDD_CH_READBACK, def: 0x00 },
        reg_default { reg: MAX98373_R2055_MEAS_ADC_THERM_CH_READBACK, def: 0x00 },
        reg_default { reg: MAX98373_R2056_MEAS_ADC_PVDD_CH_EN, def: 0x00 },
        reg_default { reg: MAX98373_R2090_BDE_LVL_HOLD, def: 0x00 },
        reg_default { reg: MAX98373_R2091_BDE_GAIN_ATK_REL_RATE, def: 0x00 },
        reg_default { reg: MAX98373_R2092_BDE_CLIPPER_MODE, def: 0x00 },
        reg_default { reg: MAX98373_R2097_BDE_L1_THRESH, def: 0x00 },
        reg_default { reg: MAX98373_R2098_BDE_L2_THRESH, def: 0x00 },
        reg_default { reg: MAX98373_R2099_BDE_L3_THRESH, def: 0x00 },
        reg_default { reg: MAX98373_R209A_BDE_L4_THRESH, def: 0x00 },
        reg_default { reg: MAX98373_R209B_BDE_THRESH_HYST, def: 0x00 },
        reg_default { reg: MAX98373_R20A8_BDE_L1_CFG_1, def: 0x00 },
        reg_default { reg: MAX98373_R20A9_BDE_L1_CFG_2, def: 0x00 },
        reg_default { reg: MAX98373_R20AA_BDE_L1_CFG_3, def: 0x00 },
        reg_default { reg: MAX98373_R20AB_BDE_L2_CFG_1, def: 0x00 },
        reg_default { reg: MAX98373_R20AC_BDE_L2_CFG_2, def: 0x00 },
        reg_default { reg: MAX98373_R20AD_BDE_L2_CFG_3, def: 0x00 },
        reg_default { reg: MAX98373_R20AE_BDE_L3_CFG_1, def: 0x00 },
        reg_default { reg: MAX98373_R20AF_BDE_L3_CFG_2, def: 0x00 },
        reg_default { reg: MAX98373_R20B0_BDE_L3_CFG_3, def: 0x00 },
        reg_default { reg: MAX98373_R20B1_BDE_L4_CFG_1, def: 0x00 },
        reg_default { reg: MAX98373_R20B2_BDE_L4_CFG_2, def: 0x00 },
        reg_default { reg: MAX98373_R20B3_BDE_L4_CFG_3, def: 0x00 },
        reg_default { reg: MAX98373_R20B4_BDE_INFINITE_HOLD_RELEASE, def: 0x00 },
        reg_default { reg: MAX98373_R20B5_BDE_EN, def: 0x00 },
        reg_default { reg: MAX98373_R20B6_BDE_CUR_STATE_READBACK, def: 0x00 },
        reg_default { reg: MAX98373_R20D1_DHT_CFG, def: 0x01 },
        reg_default { reg: MAX98373_R20D2_DHT_ATTACK_CFG, def: 0x02 },
        reg_default { reg: MAX98373_R20D3_DHT_RELEASE_CFG, def: 0x03 },
        reg_default { reg: MAX98373_R20D4_DHT_EN, def: 0x00 },
        reg_default { reg: MAX98373_R20E0_LIMITER_THRESH_CFG, def: 0x00 },
        reg_default { reg: MAX98373_R20E1_LIMITER_ATK_REL_RATES, def: 0x00 },
        reg_default { reg: MAX98373_R20E2_LIMITER_EN, def: 0x00 },
        reg_default { reg: MAX98373_R20FE_DEVICE_AUTO_RESTART_CFG, def: 0x00 },
        reg_default { reg: MAX98373_R20FF_GLOBAL_SHDN, def: 0x00 },
        reg_default { reg: MAX98373_R21FF_REV_ID, def: 0x42 },
    ]
};

unsafe extern "C" fn max98373_readable_register(_dev: *mut device, reg: c_uint) -> bool_ {
    if reg == MAX98373_R21FF_REV_ID || reg == MAX98373_R2010_IRQ_CTRL {
        return true;
    }
    if reg >= MAX98373_R0040_SCP_INIT_STAT_1 && reg <= MAX98373_R0070_SCP_FRAME_CTLR {
        return true;
    }
    if reg >= MAX98373_R0100_DP1_INIT_STAT && reg <= MAX98373_R0137_DP1_BLOCK_CTRL3 {
        return true;
    }
    if reg >= MAX98373_R0300_DP3_INIT_STAT && reg <= MAX98373_R0337_DP3_BLOCK_CTRL3 {
        return true;
    }
    if reg >= MAX98373_R2000_SW_RESET && reg <= MAX98373_R200C_INT_EN3 {
        return true;
    }
    if reg >= MAX98373_R2014_THERM_WARN_THRESH && reg <= MAX98373_R2018_THERM_FOLDBACK_EN {
        return true;
    }
    if reg >= MAX98373_R201E_PIN_DRIVE_STRENGTH && reg <= MAX98373_R2036_SOUNDWIRE_CTRL {
        return true;
    }
    if reg >= MAX98373_R203D_AMP_DIG_VOL_CTRL && reg <= MAX98373_R2043_AMP_EN {
        return true;
    }
    if reg >= MAX98373_R2046_IV_SENSE_ADC_DSP_CFG && reg <= MAX98373_R2047_IV_SENSE_ADC_EN {
        return true;
    }
    if reg >= MAX98373_R2051_MEAS_ADC_SAMPLING_RATE && reg <= MAX98373_R2056_MEAS_ADC_PVDD_CH_EN {
        return true;
    }
    if reg >= MAX98373_R2090_BDE_LVL_HOLD && reg <= MAX98373_R2092_BDE_CLIPPER_MODE {
        return true;
    }
    if reg >= MAX98373_R2097_BDE_L1_THRESH && reg <= MAX98373_R209B_BDE_THRESH_HYST {
        return true;
    }
    if reg >= MAX98373_R20A8_BDE_L1_CFG_1 && reg <= MAX98373_R20B3_BDE_L4_CFG_3 {
        return true;
    }
    if reg >= MAX98373_R20B5_BDE_EN && reg <= MAX98373_R20B6_BDE_CUR_STATE_READBACK {
        return true;
    }
    if reg >= MAX98373_R20D1_DHT_CFG && reg <= MAX98373_R20D4_DHT_EN {
        return true;
    }
    if reg >= MAX98373_R20E0_LIMITER_THRESH_CFG && reg <= MAX98373_R20E2_LIMITER_EN {
        return true;
    }
    if reg >= MAX98373_R20FE_DEVICE_AUTO_RESTART_CFG && reg <= MAX98373_R20FF_GLOBAL_SHDN {
        return true;
    }
    false
}

unsafe extern "C" fn max98373_volatile_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    if reg == MAX98373_R2054_MEAS_ADC_PVDD_CH_READBACK
        || reg == MAX98373_R2055_MEAS_ADC_THERM_CH_READBACK
        || reg == MAX98373_R20B6_BDE_CUR_STATE_READBACK
        || reg == MAX98373_R20FF_GLOBAL_SHDN
        || reg == MAX98373_R21FF_REV_ID
    {
        return true;
    }
    if reg >= MAX98373_R0040_SCP_INIT_STAT_1 && reg <= MAX98373_R0070_SCP_FRAME_CTLR {
        return true;
    }
    if reg >= MAX98373_R0100_DP1_INIT_STAT && reg <= MAX98373_R0137_DP1_BLOCK_CTRL3 {
        return true;
    }
    if reg >= MAX98373_R0300_DP3_INIT_STAT && reg <= MAX98373_R0337_DP3_BLOCK_CTRL3 {
        return true;
    }
    if reg >= MAX98373_R2000_SW_RESET && reg <= MAX98373_R2009_INT_FLAG3 {
        return true;
    }
    false
}

static max98373_sdw_regmap: regmap_config = unsafe {
    regmap_config {
        reg_bits: 32,
        val_bits: 8,
        max_register: MAX98373_R21FF_REV_ID,
        reg_defaults: max98373_reg.as_ptr(),
        num_reg_defaults: max98373_reg.len() as c_uint,
        readable_reg: Some(max98373_readable_register),
        volatile_reg: Some(max98373_volatile_reg),
        cache_type: REGCACHE_RBTREE,
        use_single_read: true,
        use_single_write: true,
    }
};

/* Power management functions and structure */
unsafe extern "C" fn max98373_suspend(dev: *mut device) -> c_int {
    let max98373 = dev_get_drvdata(dev) as *mut max98373_priv;
    let mut i: c_int = 0;

    /* cache feedback register values before suspend */
    while i < (*max98373).cache_num {
        regmap_read(
            (*max98373).regmap,
            (*(*max98373).cache.add(i as usize)).reg,
            &mut (*(*max98373).cache.add(i as usize)).val,
        );
        i += 1;
    }

    regcache_cache_only((*max98373).regmap, true);

    0
}

unsafe extern "C" fn max98373_resume(dev: *mut device) -> c_int {
    let slave = dev_to_sdw_dev(dev);
    let max98373 = dev_get_drvdata(dev) as *mut max98373_priv;
    let mut ret: c_int;

    if !(*max98373).first_hw_init {
        return 0;
    }

    ret = sdw_slave_wait_for_init(slave, MAX98373_PROBE_TIMEOUT);
    if ret != 0 {
        sdw_show_ping_status((*slave).bus, true);
        return ret;
    }

    regcache_cache_only((*max98373).regmap, false);
    ret = regcache_sync((*max98373).regmap);
    if ret != 0 {
        regcache_cache_only((*max98373).regmap, true);
        regcache_mark_dirty((*max98373).regmap);
        return ret;
    }

    0
}

unsafe fn dev_to_sdw_dev(dev: *mut device) -> *mut sdw_slave {
    dev as *mut sdw_slave
}

// static const struct dev_pm_ops max98373_pm =
// SYSTEM_SLEEP_PM_OPS(max98373_suspend, max98373_resume)
// RUNTIME_PM_OPS(max98373_suspend, max98373_resume, NULL)
static max98373_pm: dev_pm_ops = dev_pm_ops { _private: [] };

unsafe extern "C" fn max98373_read_prop(slave: *mut sdw_slave) -> c_int {
    let prop = &mut (*slave).prop as *mut sdw_slave_prop;
    let mut nval: c_int;
    let mut i: c_int;
    let mut bit: u32;
    let mut addr: c_ulong;
    let mut dpn: *mut sdw_dpn_prop;

    (*prop).scp_int1_mask = SDW_SCP_INT1_BUS_CLASH | SDW_SCP_INT1_PARITY;

    /* BITMAP: 00001000  Dataport 3 is active */
    (*prop).source_ports = BIT(3);
    /* BITMAP: 00000010  Dataport 1 is active */
    (*prop).sink_ports = BIT(1);
    (*prop).paging_support = true;
    (*prop).clk_stop_timeout = 20;

    nval = hweight32((*prop).source_ports);
    (*prop).src_dpn_prop = devm_kcalloc(
        &mut (*slave).dev,
        nval as usize,
        size_of::<sdw_dpn_prop>(),
        GFP_KERNEL,
    ) as *mut sdw_dpn_prop;
    if (*prop).src_dpn_prop.is_null() {
        return -ENOMEM;
    }

    i = 0;
    dpn = (*prop).src_dpn_prop;
    addr = (*prop).source_ports;
    bit = 0;
    while bit < 32 {
        if (addr & BIT(bit)) != 0 {
            (*dpn.add(i as usize)).num = bit;
            (*dpn.add(i as usize)).type_ = SDW_DPN_FULL;
            (*dpn.add(i as usize)).simple_ch_prep_sm = true;
            (*dpn.add(i as usize)).ch_prep_timeout = 10;
            i += 1;
        }
        bit += 1;
    }

    /* do this again for sink now */
    nval = hweight32((*prop).sink_ports);
    (*prop).sink_dpn_prop = devm_kcalloc(
        &mut (*slave).dev,
        nval as usize,
        size_of::<sdw_dpn_prop>(),
        GFP_KERNEL,
    ) as *mut sdw_dpn_prop;
    if (*prop).sink_dpn_prop.is_null() {
        return -ENOMEM;
    }

    i = 0;
    dpn = (*prop).sink_dpn_prop;
    addr = (*prop).sink_ports;
    bit = 0;
    while bit < 32 {
        if (addr & BIT(bit)) != 0 {
            (*dpn.add(i as usize)).num = bit;
            (*dpn.add(i as usize)).type_ = SDW_DPN_FULL;
            (*dpn.add(i as usize)).simple_ch_prep_sm = true;
            (*dpn.add(i as usize)).ch_prep_timeout = 10;
            i += 1;
        }
        bit += 1;
    }

    /* set the timeout values */
    (*prop).clk_stop_timeout = 20;

    0
}

unsafe extern "C" fn max98373_io_init(slave: *mut sdw_slave) -> c_int {
    let dev = &mut (*slave).dev as *mut device;
    let max98373 = dev_get_drvdata(dev) as *mut max98373_priv;

    regcache_cache_only((*max98373).regmap, false);
    if (*max98373).first_hw_init {
        regcache_cache_bypass((*max98373).regmap, true);
    }

    /*
     * PM runtime status is marked as 'active' only when a Slave reports as Attached
     */
    if !(*max98373).first_hw_init {
        /* update count of parent 'active' children */
        pm_runtime_set_active(dev);
    }

    pm_runtime_get_noresume(dev);

    /* Software Reset */
    max98373_reset(max98373, dev);

    /* Set soundwire mode */
    regmap_write((*max98373).regmap, MAX98373_R2025_AUDIO_IF_MODE, 3);
    /* Enable ADC */
    regmap_write((*max98373).regmap, MAX98373_R2047_IV_SENSE_ADC_EN, 3);
    /* Set default Soundwire clock */
    regmap_write((*max98373).regmap, MAX98373_R2036_SOUNDWIRE_CTRL, 5);
    /* Set default sampling rate for speaker and IVDAC */
    regmap_write((*max98373).regmap, MAX98373_R2028_PCM_SR_SETUP_2, 0x88);
    /* IV default slot configuration */
    regmap_write((*max98373).regmap, MAX98373_R2020_PCM_TX_HIZ_EN_1, 0xFF);
    regmap_write((*max98373).regmap, MAX98373_R2021_PCM_TX_HIZ_EN_2, 0xFF);
    /* L/R mix configuration */
    regmap_write((*max98373).regmap, MAX98373_R2029_PCM_TO_SPK_MONO_MIX_1, 0x80);
    regmap_write((*max98373).regmap, MAX98373_R202A_PCM_TO_SPK_MONO_MIX_2, 0x1);
    /* Enable DC blocker */
    regmap_write((*max98373).regmap, MAX98373_R203F_AMP_DSP_CFG, 0x3);
    /* Enable IMON VMON DC blocker */
    regmap_write((*max98373).regmap, MAX98373_R2046_IV_SENSE_ADC_DSP_CFG, 0x7);
    /* voltage, current slot configuration */
    regmap_write(
        (*max98373).regmap,
        MAX98373_R2022_PCM_TX_SRC_1,
        (((*max98373).i_slot << MAX98373_PCM_TX_CH_SRC_A_I_SHIFT) | (*max98373).v_slot) & 0xFF,
    );
    if (*max98373).v_slot < 8 {
        regmap_update_bits(
            (*max98373).regmap,
            MAX98373_R2020_PCM_TX_HIZ_EN_1,
            1u32 << (*max98373).v_slot,
            0,
        );
    } else {
        regmap_update_bits(
            (*max98373).regmap,
            MAX98373_R2021_PCM_TX_HIZ_EN_2,
            1u32 << ((*max98373).v_slot - 8),
            0,
        );
    }

    if (*max98373).i_slot < 8 {
        regmap_update_bits(
            (*max98373).regmap,
            MAX98373_R2020_PCM_TX_HIZ_EN_1,
            1u32 << (*max98373).i_slot,
            0,
        );
    } else {
        regmap_update_bits(
            (*max98373).regmap,
            MAX98373_R2021_PCM_TX_HIZ_EN_2,
            1u32 << ((*max98373).i_slot - 8),
            0,
        );
    }

    /* speaker feedback slot configuration */
    regmap_write((*max98373).regmap, MAX98373_R2023_PCM_TX_SRC_2, (*max98373).spkfb_slot & 0xFF);

    /* Set interleave mode */
    if (*max98373).interleave_mode {
        regmap_update_bits(
            (*max98373).regmap,
            MAX98373_R2024_PCM_DATA_FMT_CFG,
            MAX98373_PCM_TX_CH_INTERLEAVE_MASK,
            MAX98373_PCM_TX_CH_INTERLEAVE_MASK,
        );
    }

    /* Speaker enable */
    regmap_update_bits((*max98373).regmap, MAX98373_R2043_AMP_EN, MAX98373_SPK_EN_MASK, 1);

    regmap_write((*max98373).regmap, MAX98373_R20B5_BDE_EN, 1);
    regmap_write((*max98373).regmap, MAX98373_R20E2_LIMITER_EN, 1);

    if (*max98373).first_hw_init {
        regcache_cache_bypass((*max98373).regmap, false);
        regcache_mark_dirty((*max98373).regmap);
    }

    (*max98373).first_hw_init = true;
    (*max98373).hw_init = true;

    pm_runtime_put_autosuspend(dev);

    0
}

unsafe extern "C" fn max98373_clock_calculate(slave: *mut sdw_slave, clk_freq: c_uint) -> c_int {
    let mut x: c_int;
    let mut y: usize;
    static max98373_clk_family: [c_int; 7] = [
        7680000, 8400000, 9600000, 11289600,
        12000000, 12288000, 13000000,
    ];

    x = 0;
    while x < 4 {
        y = 0;
        while y < max98373_clk_family.len() {
            if clk_freq == ((max98373_clk_family[y] >> x) as c_uint) {
                return (x << 3) + y as c_int;
            }
            y += 1;
        }
        x += 1;
    }

    /* Set default clock (12.288 Mhz) if the value is not in the list */
    dev_err(&mut (*slave).dev, b"Requested clock not found. (clk_freq = %d)\n\0".as_ptr() as *const c_char, clk_freq);
    0x5
}

unsafe extern "C" fn max98373_clock_config(slave: *mut sdw_slave, params: *mut sdw_bus_params) -> c_int {
    let dev = &mut (*slave).dev as *mut device;
    let max98373 = dev_get_drvdata(dev) as *mut max98373_priv;
    let clk_freq: c_uint;
    let value: c_uint;

    clk_freq = (*params).curr_dr_freq >> 1;

    /*
     * Select the proper value for the register based on the
     * requested clock. If the value is not in the list,
     * use reasonable default - 12.288 Mhz
     */
    value = max98373_clock_calculate(slave, clk_freq) as c_uint;

    /* SWCLK */
    regmap_write((*max98373).regmap, MAX98373_R2036_SOUNDWIRE_CTRL, value);

    /* The default Sampling Rate value for IV is 48KHz*/
    regmap_write((*max98373).regmap, MAX98373_R2028_PCM_SR_SETUP_2, 0x88);

    0
}

unsafe extern "C" fn max98373_sdw_dai_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let max98373 = snd_soc_component_get_drvdata(component) as *mut max98373_priv;
    let mut stream_config: sdw_stream_config = core::mem::zeroed();
    let mut port_config: sdw_port_config = core::mem::zeroed();
    let sdw_stream: *mut sdw_stream_runtime;
    let mut ret: c_int;
    let chan_sz: c_int;
    let sampling_rate: c_int;

    sdw_stream = snd_soc_dai_get_dma_data(dai, substream);

    if sdw_stream.is_null() {
        return -EINVAL;
    }

    if (*max98373).slave.is_null() {
        return -EINVAL;
    }

    snd_sdw_params_to_config(substream, params, &mut stream_config, &mut port_config);

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        port_config.num = 1;

        if (*max98373).slot != 0 {
            stream_config.ch_count = (*max98373).slot as c_uint;
            port_config.ch_mask = (*max98373).rx_mask;
        }
    } else {
        port_config.num = 3;

        /* only IV are supported by capture */
        stream_config.ch_count = 2;
        port_config.ch_mask = GENMASK(stream_config.ch_count as c_int - 1, 0);
    }

    ret = sdw_stream_add_slave((*max98373).slave, &mut stream_config, &mut port_config, 1, sdw_stream);
    if ret != 0 {
        dev_err((*dai).dev, b"Unable to configure port\n\0".as_ptr() as *const c_char);
        return ret;
    }

    if params_channels(params) > 16 {
        dev_err((*component).dev, b"Unsupported channels %d\n\0".as_ptr() as *const c_char, params_channels(params));
        return -EINVAL;
    }

    /* Channel size configuration */
    chan_sz = match snd_pcm_format_width(params_format(params)) {
        16 => MAX98373_PCM_MODE_CFG_CHANSZ_16,
        24 => MAX98373_PCM_MODE_CFG_CHANSZ_24,
        32 => MAX98373_PCM_MODE_CFG_CHANSZ_32,
        _ => {
            dev_err((*component).dev, b"Channel size unsupported %d\n\0".as_ptr() as *const c_char, params_format(params));
            return -EINVAL;
        }
    };

    (*max98373).ch_size = snd_pcm_format_width(params_format(params));

    regmap_update_bits(
        (*max98373).regmap,
        MAX98373_R2024_PCM_DATA_FMT_CFG,
        MAX98373_PCM_MODE_CFG_CHANSZ_MASK,
        chan_sz as c_uint,
    );

    dev_dbg((*component).dev, b"Format supported %d\0".as_ptr() as *const c_char, params_format(params));

    /* Sampling rate configuration */
    sampling_rate = match params_rate(params) {
        8000 => MAX98373_PCM_SR_SET1_SR_8000,
        11025 => MAX98373_PCM_SR_SET1_SR_11025,
        12000 => MAX98373_PCM_SR_SET1_SR_12000,
        16000 => MAX98373_PCM_SR_SET1_SR_16000,
        22050 => MAX98373_PCM_SR_SET1_SR_22050,
        24000 => MAX98373_PCM_SR_SET1_SR_24000,
        32000 => MAX98373_PCM_SR_SET1_SR_32000,
        44100 => MAX98373_PCM_SR_SET1_SR_44100,
        48000 => MAX98373_PCM_SR_SET1_SR_48000,
        88200 => MAX98373_PCM_SR_SET1_SR_88200,
        96000 => MAX98373_PCM_SR_SET1_SR_96000,
        _ => {
            dev_err((*component).dev, b"Rate %d is not supported\n\0".as_ptr() as *const c_char, params_rate(params));
            return -EINVAL;
        }
    };

    /* set correct sampling frequency */
    regmap_update_bits(
        (*max98373).regmap,
        MAX98373_R2028_PCM_SR_SETUP_2,
        MAX98373_PCM_SR_SET2_SR_MASK,
        (sampling_rate as c_uint) << MAX98373_PCM_SR_SET2_SR_SHIFT,
    );

    /* set sampling rate of IV */
    regmap_update_bits(
        (*max98373).regmap,
        MAX98373_R2028_PCM_SR_SETUP_2,
        MAX98373_PCM_SR_SET2_IVADC_SR_MASK,
        sampling_rate as c_uint,
    );

    0
}

unsafe extern "C" fn max98373_pcm_hw_free(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let max98373 = snd_soc_component_get_drvdata(component) as *mut max98373_priv;
    let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);

    if (*max98373).slave.is_null() {
        return -EINVAL;
    }

    sdw_stream_remove_slave((*max98373).slave, sdw_stream);
    0
}

unsafe extern "C" fn max98373_set_sdw_stream(dai: *mut snd_soc_dai, sdw_stream: *mut c_void, direction: c_int) -> c_int {
    snd_soc_dai_dma_data_set(dai, direction, sdw_stream);

    0
}

unsafe extern "C" fn max98373_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    snd_soc_dai_set_dma_data(dai, substream, ptr::null_mut());
}

unsafe extern "C" fn max98373_sdw_set_tdm_slot(
    dai: *mut snd_soc_dai,
    tx_mask: c_uint,
    rx_mask: c_uint,
    slots: c_int,
    _slot_width: c_int,
) -> c_int {
    let component = (*dai).component;
    let max98373 = snd_soc_component_get_drvdata(component) as *mut max98373_priv;

    /* tx_mask is unused since it's irrelevant for I/V feedback */
    if tx_mask != 0 {
        return -EINVAL;
    }

    if rx_mask == 0 && slots == 0 && _slot_width == 0 {
        (*max98373).tdm_mode = false;
    } else {
        (*max98373).tdm_mode = true;
    }

    (*max98373).rx_mask = rx_mask;
    (*max98373).slot = slots;

    0
}

static max98373_dai_sdw_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(max98373_sdw_dai_hw_params),
    hw_free: Some(max98373_pcm_hw_free),
    set_stream: Some(max98373_set_sdw_stream),
    shutdown: Some(max98373_shutdown),
    set_tdm_slot: Some(max98373_sdw_set_tdm_slot),
};

static mut max98373_sdw_dai: [snd_soc_dai_driver; 1] = [
    snd_soc_dai_driver {
        name: b"max98373-aif1\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream {
            stream_name: b"HiFi Playback\0".as_ptr() as *const c_char,
            channels_min: 1,
            channels_max: 2,
            rates: MAX98373_RATES,
            formats: MAX98373_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: b"HiFi Capture\0".as_ptr() as *const c_char,
            channels_min: 1,
            channels_max: 2,
            rates: MAX98373_RATES,
            formats: MAX98373_FORMATS,
        },
        ops: &max98373_dai_sdw_ops,
    },
];

unsafe extern "C" fn max98373_init(slave: *mut sdw_slave, regmap: *mut regmap) -> c_int {
    let max98373: *mut max98373_priv;
    let mut ret: c_int;
    let mut i: c_int;
    let dev = &mut (*slave).dev as *mut device;

    /* Allocate and assign private driver data structure */
    max98373 = devm_kzalloc(dev, size_of::<max98373_priv>(), GFP_KERNEL) as *mut max98373_priv;
    if max98373.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(dev, max98373 as *mut c_void);
    (*max98373).regmap = regmap;
    (*max98373).slave = slave;

    regcache_cache_only((*max98373).regmap, true);

    (*max98373).cache_num = max98373_sdw_cache_reg.len() as c_int;
    (*max98373).cache = devm_kcalloc(
        dev,
        (*max98373).cache_num as usize,
        size_of::<max98373_cache>(),
        GFP_KERNEL,
    ) as *mut max98373_cache;
    if (*max98373).cache.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < (*max98373).cache_num {
        (*(*max98373).cache.add(i as usize)).reg = max98373_sdw_cache_reg[i as usize];
        i += 1;
    }

    /* Read voltage and slot configuration */
    max98373_slot_config(dev, max98373);

    (*max98373).hw_init = false;
    (*max98373).first_hw_init = false;

    /* codec registration */
    ret = devm_snd_soc_register_component(
        dev,
        &soc_codec_dev_max98373_sdw,
        max98373_sdw_dai.as_mut_ptr(),
        max98373_sdw_dai.len() as c_int,
    );
    if ret < 0 {
        dev_err(dev, b"Failed to register codec: %d\n\0".as_ptr() as *const c_char, ret);
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

    0
}

unsafe extern "C" fn max98373_update_status(slave: *mut sdw_slave, status: sdw_slave_status) -> c_int {
    let max98373 = dev_get_drvdata(&mut (*slave).dev) as *mut max98373_priv;

    if status == SDW_SLAVE_UNATTACHED {
        (*max98373).hw_init = false;
    }

    /*
     * Perform initialization only if slave status is SDW_SLAVE_ATTACHED
     */
    if (*max98373).hw_init || status != SDW_SLAVE_ATTACHED {
        return 0;
    }

    /* perform I/O transfers required for Slave initialization */
    max98373_io_init(slave)
}

unsafe extern "C" fn max98373_bus_config(slave: *mut sdw_slave, params: *mut sdw_bus_params) -> c_int {
    let ret: c_int;

    ret = max98373_clock_config(slave, params);
    if ret < 0 {
        dev_err(&mut (*slave).dev, b"Invalid clk config\0".as_ptr() as *const c_char);
    }

    ret
}

/*
 * slave_ops: callbacks for get_clock_stop_mode, clock_stop and
 * port_prep are not defined for now
 */
static max98373_slave_ops: sdw_slave_ops = sdw_slave_ops {
    read_prop: Some(max98373_read_prop),
    update_status: Some(max98373_update_status),
    bus_config: Some(max98373_bus_config),
};

unsafe extern "C" fn max98373_sdw_probe(slave: *mut sdw_slave, _id: *const sdw_device_id) -> c_int {
    let regmap: *mut regmap;

    /* Regmap Initialization */
    regmap = devm_regmap_init_sdw(slave, &max98373_sdw_regmap);
    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }

    max98373_init(slave, regmap)
}

unsafe extern "C" fn max98373_sdw_remove(slave: *mut sdw_slave) {
    pm_runtime_disable(&mut (*slave).dev);
}

// #if defined(CONFIG_OF)
// static const struct of_device_id max98373_of_match[] = {
//     { .compatible = "maxim,max98373", },
//     {},
// };
// MODULE_DEVICE_TABLE(of, max98373_of_match);
// #endif

// #ifdef CONFIG_ACPI
// static const struct acpi_device_id max98373_acpi_match[] = {
//     { "MX98373", 0 },
//     {},
// };
// MODULE_DEVICE_TABLE(acpi, max98373_acpi_match);
// #endif

static max98373_id: [sdw_device_id; 2] = [
    // SDW_SLAVE_ENTRY(0x019F, 0x8373, 0)
    sdw_device_id { _private: [] },
    sdw_device_id { _private: [] },
];
// MODULE_DEVICE_TABLE(sdw, max98373_id);

static mut max98373_sdw_driver: sdw_driver = sdw_driver {
    driver: driver_inner {
        name: b"max98373\0".as_ptr() as *const c_char,
        of_match_table: unsafe { max98373_of_match.as_ptr() },
        acpi_match_table: unsafe { max98373_acpi_match.as_ptr() },
        pm: &max98373_pm,
    },
    probe: Some(max98373_sdw_probe),
    remove: Some(max98373_sdw_remove),
    ops: &max98373_slave_ops,
    id_table: max98373_id.as_ptr(),
};

// module_sdw_driver(max98373_sdw_driver);

// MODULE_DESCRIPTION("ASoC MAX98373 driver SDW");
// MODULE_AUTHOR("Oleg Sherbakov <oleg.sherbakov@maximintegrated.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
