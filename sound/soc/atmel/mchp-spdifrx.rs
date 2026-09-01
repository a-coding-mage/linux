// SPDX-License-Identifier: GPL-2.0
//
// Driver for Microchip S/PDIF RX Controller
//
// Copyright (C) 2020 Microchip Technology Inc. and its subsidiaries
//
// Author: Codrin Ciubotariu <codrin.ciubotariu@microchip.com>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u8 = u8;
type u32 = u32;
type dma_addr_t = c_ulong;
type irqreturn_t = c_uint;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct resource {
    pub start: c_ulong,
}
#[repr(C)]
pub struct platform_device {
    pub dev: device,
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
pub struct snd_soc_dai {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}
#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr: dma_addr_t,
    pub maxburst: c_uint,
}

#[repr(C)]
pub struct snd_ctl_elem_info_integer {
    pub min: c_long,
    pub max: c_long,
}
type c_long = isize;

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub struct snd_ctl_elem_value_iec958 {
    pub status: [u8; 24],
    pub subcode: [u8; 24],
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub iec958: snd_ctl_elem_value_iec958,
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub reg_stride: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub precious_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub cache_type: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub access: c_uint,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
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
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: driver,
}

const EINVAL: c_int = 22;
const EBUSY: c_int = 16;
const ENOMEM: c_int = 12;
const ETIMEDOUT: c_int = 110;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_FLAT: c_uint = 1;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 2;
const SNDRV_PCM_FORMAT_S16_BE: c_int = 3;
const SNDRV_PCM_FORMAT_S20_3LE: c_int = 8;
const SNDRV_PCM_FORMAT_S20_3BE: c_int = 9;
const SNDRV_PCM_FORMAT_S24_3LE: c_int = 10;
const SNDRV_PCM_FORMAT_S24_3BE: c_int = 11;
const SNDRV_PCM_FORMAT_S24_LE: c_int = 6;
const SNDRV_PCM_FORMAT_S24_BE: c_int = 7;
const SNDRV_CTL_ELEM_TYPE_IEC958: c_uint = 4;
const SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_uint = 1;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 2;
const SNDRV_CTL_ELEM_IFACE_PCM: c_uint = 2;
const SNDRV_CTL_ELEM_ACCESS_READ: c_uint = 1;
const SNDRV_CTL_ELEM_ACCESS_VOLATILE: c_uint = 1 << 8;
const SNDRV_PCM_RATE_8000_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 1 << SNDRV_PCM_FORMAT_S16_LE;
const SNDRV_PCM_FMTBIT_U16_BE: c_ulong = 1 << 5;
const SNDRV_PCM_FMTBIT_S20_3LE: c_ulong = 1 << SNDRV_PCM_FORMAT_S20_3LE;
const SNDRV_PCM_FMTBIT_S20_3BE: c_ulong = 1 << SNDRV_PCM_FORMAT_S20_3BE;
const SNDRV_PCM_FMTBIT_S24_3LE: c_ulong = 1 << SNDRV_PCM_FORMAT_S24_3LE;
const SNDRV_PCM_FMTBIT_S24_3BE: c_ulong = 1 << SNDRV_PCM_FORMAT_S24_3BE;
const SNDRV_PCM_FMTBIT_S24_LE: c_ulong = 1 << SNDRV_PCM_FORMAT_S24_LE;
const SNDRV_PCM_FMTBIT_S24_BE: c_ulong = 1 << SNDRV_PCM_FORMAT_S24_BE;

const fn BIT(n: c_uint) -> u32 {
    1u32 << n
}

const fn GENMASK(h: c_uint, l: c_uint) -> u32 {
    (!0u32 << l) & (!0u32 >> (31 - h))
}

const fn __bf_shf(mask: u32) -> c_uint {
    mask.trailing_zeros()
}

const fn FIELD_PREP(mask: u32, val: u32) -> u32 {
    (val << __bf_shf(mask)) & mask
}

const fn FIELD_GET(mask: u32, reg: u32) -> u32 {
    (reg & mask) >> __bf_shf(mask)
}

/*
 * ---- S/PDIF Receiver Controller Register map ----
 */
const SPDIFRX_CR: c_uint = 0x00; /* Control Register */
const SPDIFRX_MR: c_uint = 0x04; /* Mode Register */

const SPDIFRX_IER: c_uint = 0x10; /* Interrupt Enable Register */
const SPDIFRX_IDR: c_uint = 0x14; /* Interrupt Disable Register */
const SPDIFRX_IMR: c_uint = 0x18; /* Interrupt Mask Register */
const SPDIFRX_ISR: c_uint = 0x1c; /* Interrupt Status Register */
const SPDIFRX_RSR: c_uint = 0x20; /* Status Register */
const SPDIFRX_RHR: c_uint = 0x24; /* Holding Register */

const fn SPDIFRX_CHSR(channel: c_uint, reg: c_uint) -> c_uint {
    0x30 + channel * 0x30 + reg * 4
} /* Channel x Status Registers */

const fn SPDIFRX_CHUD(channel: c_uint, reg: c_uint) -> c_uint {
    0x48 + channel * 0x30 + reg * 4
} /* Channel x User Data Registers */

const SPDIFRX_WPMR: c_uint = 0xE4; /* Write Protection Mode Register */
const SPDIFRX_WPSR: c_uint = 0xE8; /* Write Protection Status Register */

const SPDIFRX_VERSION: c_uint = 0xFC; /* Version Register */

/* 32-bit word byte masks */
const SPDIFRX_BYTE_0_MASK: u32 = GENMASK(7, 0);
const SPDIFRX_BYTE_1_MASK: u32 = GENMASK(15, 8);
const SPDIFRX_BYTE_2_MASK: u32 = GENMASK(23, 16);
const SPDIFRX_BYTE_3_MASK: u32 = GENMASK(31, 24);

/*
 * ---- Control Register (Write-only) ----
 */
const SPDIFRX_CR_SWRST: u32 = BIT(0); /* Software Reset */

/*
 * ---- Mode Register (Read/Write) ----
 */
/* Receive Enable */
const SPDIFRX_MR_RXEN_MASK: u32 = GENMASK(0, 0);
const SPDIFRX_MR_RXEN_DISABLE: u32 = 0 << 0; /* SPDIF Receiver Disabled */
const SPDIFRX_MR_RXEN_ENABLE: u32 = 1 << 0; /* SPDIF Receiver Enabled */

/* Validity Bit Mode */
const SPDIFRX_MR_VBMODE_MASK: u32 = GENMASK(1, 1);
const SPDIFRX_MR_VBMODE_ALWAYS_LOAD: u32 = 0 << 1; /* Load sample regardless of validity bit value */
const SPDIFRX_MR_VBMODE_DISCARD_IF_VB1: u32 = 1 << 1; /* Load sample only if validity bit is 0 */

/* Data Word Endian Mode */
const SPDIFRX_MR_ENDIAN_MASK: u32 = GENMASK(2, 2);
const SPDIFRX_MR_ENDIAN_LITTLE: u32 = 0 << 2; /* Little Endian Mode */
const SPDIFRX_MR_ENDIAN_BIG: u32 = 1 << 2; /* Big Endian Mode */

/* Parity Bit Mode */
const SPDIFRX_MR_PBMODE_MASK: u32 = GENMASK(3, 3);
const SPDIFRX_MR_PBMODE_PARCHECK: u32 = 0 << 3; /* Parity Check Enabled */
const SPDIFRX_MR_PBMODE_NOPARCHECK: u32 = 1 << 3; /* Parity Check Disabled */

/* Sample Data Width */
const SPDIFRX_MR_DATAWIDTH_MASK: u32 = GENMASK(5, 4);
const fn SPDIFRX_MR_DATAWIDTH(width: c_uint) -> u32 {
    FIELD_PREP(SPDIFRX_MR_DATAWIDTH_MASK, 6 - (width / 4))
}

/* Packed Data Mode in Receive Holding Register */
const SPDIFRX_MR_PACK_MASK: u32 = GENMASK(7, 7);
const SPDIFRX_MR_PACK_DISABLED: u32 = 0 << 7;
const SPDIFRX_MR_PACK_ENABLED: u32 = 1 << 7;

/* Start of Block Bit Mode */
const SPDIFRX_MR_SBMODE_MASK: u32 = GENMASK(8, 8);
const SPDIFRX_MR_SBMODE_ALWAYS_LOAD: u32 = 0 << 8;
const SPDIFRX_MR_SBMODE_DISCARD: u32 = 1 << 8;

/* Consecutive Preamble Error Threshold Automatic Restart */
const SPDIFRX_MR_AUTORST_MASK: u32 = GENMASK(24, 24);
const SPDIFRX_MR_AUTORST_NOACTION: u32 = 0 << 24;
const SPDIFRX_MR_AUTORST_UNLOCK_ON_PRE_ERR: u32 = 1 << 24;

/*
 * ---- Interrupt Enable/Disable/Mask/Status Register (Write/Read-only) ----
 */
const SPDIFRX_IR_RXRDY: u32 = BIT(0);
const SPDIFRX_IR_LOCKED: u32 = BIT(1);
const SPDIFRX_IR_LOSS: u32 = BIT(2);
const SPDIFRX_IR_BLOCKEND: u32 = BIT(3);
const SPDIFRX_IR_SFE: u32 = BIT(4);
const SPDIFRX_IR_PAR_ERR: u32 = BIT(5);
const SPDIFRX_IR_OVERRUN: u32 = BIT(6);
const SPDIFRX_IR_RXFULL: u32 = BIT(7);
const fn SPDIFRX_IR_CSC(ch: c_uint) -> u32 {
    BIT(ch + 8)
}
const SPDIFRX_IR_SECE: u32 = BIT(10);
const SPDIFRX_IR_BLOCKST: u32 = BIT(11);
const SPDIFRX_IR_NRZ_ERR: u32 = BIT(12);
const SPDIFRX_IR_PRE_ERR: u32 = BIT(13);
const SPDIFRX_IR_CP_ERR: u32 = BIT(14);

/*
 * ---- Receiver Status Register (Read/Write) ----
 */
/* Enable Status */
const SPDIFRX_RSR_ULOCK: u32 = BIT(0);
const SPDIFRX_RSR_BADF: u32 = BIT(1);
const SPDIFRX_RSR_LOWF: u32 = BIT(2);
const SPDIFRX_RSR_NOSIGNAL: u32 = BIT(3);
const SPDIFRX_RSR_IFS_MASK: u32 = GENMASK(27, 16);
const fn SPDIFRX_RSR_IFS(reg: u32) -> u32 {
    FIELD_GET(SPDIFRX_RSR_IFS_MASK, reg)
}

/*
 *  ---- Version Register (Read-only) ----
 */
const SPDIFRX_VERSION_MASK: u32 = GENMASK(11, 0);
const SPDIFRX_VERSION_MFN_MASK: u32 = GENMASK(18, 16);
const fn SPDIFRX_VERSION_MFN(reg: u32) -> u32 {
    FIELD_GET(SPDIFRX_VERSION_MFN_MASK, reg)
}

unsafe extern "C" fn mchp_spdifrx_readable_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        SPDIFRX_MR | SPDIFRX_IMR | SPDIFRX_ISR | SPDIFRX_RSR
        | x if x == SPDIFRX_CHSR(0, 0) || x == SPDIFRX_CHSR(0, 1) || x == SPDIFRX_CHSR(0, 2)
            || x == SPDIFRX_CHSR(0, 3) || x == SPDIFRX_CHSR(0, 4) || x == SPDIFRX_CHSR(0, 5)
            || x == SPDIFRX_CHUD(0, 0) || x == SPDIFRX_CHUD(0, 1) || x == SPDIFRX_CHUD(0, 2)
            || x == SPDIFRX_CHUD(0, 3) || x == SPDIFRX_CHUD(0, 4) || x == SPDIFRX_CHUD(0, 5)
            || x == SPDIFRX_CHSR(1, 0) || x == SPDIFRX_CHSR(1, 1) || x == SPDIFRX_CHSR(1, 2)
            || x == SPDIFRX_CHSR(1, 3) || x == SPDIFRX_CHSR(1, 4) || x == SPDIFRX_CHSR(1, 5)
            || x == SPDIFRX_CHUD(1, 0) || x == SPDIFRX_CHUD(1, 1) || x == SPDIFRX_CHUD(1, 2)
            || x == SPDIFRX_CHUD(1, 3) || x == SPDIFRX_CHUD(1, 4) || x == SPDIFRX_CHUD(1, 5)
            || x == SPDIFRX_WPMR || x == SPDIFRX_WPSR || x == SPDIFRX_VERSION => true,
        _ => false,
    }
}

unsafe extern "C" fn mchp_spdifrx_writeable_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        SPDIFRX_CR | SPDIFRX_MR | SPDIFRX_IER | SPDIFRX_IDR | SPDIFRX_WPMR => true,
        _ => false,
    }
}

unsafe extern "C" fn mchp_spdifrx_precious_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        SPDIFRX_ISR | SPDIFRX_RHR => true,
        _ => false,
    }
}

unsafe extern "C" fn mchp_spdifrx_volatile_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        SPDIFRX_IMR | SPDIFRX_ISR | SPDIFRX_RSR
        | x if x == SPDIFRX_CHSR(0, 0) || x == SPDIFRX_CHSR(0, 1) || x == SPDIFRX_CHSR(0, 2)
            || x == SPDIFRX_CHSR(0, 3) || x == SPDIFRX_CHSR(0, 4) || x == SPDIFRX_CHSR(0, 5)
            || x == SPDIFRX_CHUD(0, 0) || x == SPDIFRX_CHUD(0, 1) || x == SPDIFRX_CHUD(0, 2)
            || x == SPDIFRX_CHUD(0, 3) || x == SPDIFRX_CHUD(0, 4) || x == SPDIFRX_CHUD(0, 5)
            || x == SPDIFRX_CHSR(1, 0) || x == SPDIFRX_CHSR(1, 1) || x == SPDIFRX_CHSR(1, 2)
            || x == SPDIFRX_CHSR(1, 3) || x == SPDIFRX_CHSR(1, 4) || x == SPDIFRX_CHSR(1, 5)
            || x == SPDIFRX_CHUD(1, 0) || x == SPDIFRX_CHUD(1, 1) || x == SPDIFRX_CHUD(1, 2)
            || x == SPDIFRX_CHUD(1, 3) || x == SPDIFRX_CHUD(1, 4) || x == SPDIFRX_CHUD(1, 5)
            || x == SPDIFRX_VERSION => true,
        _ => false,
    }
}

static mchp_spdifrx_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: SPDIFRX_VERSION,
    readable_reg: Some(mchp_spdifrx_readable_reg),
    writeable_reg: Some(mchp_spdifrx_writeable_reg),
    precious_reg: Some(mchp_spdifrx_precious_reg),
    volatile_reg: Some(mchp_spdifrx_volatile_reg),
    cache_type: REGCACHE_FLAT,
};

const SPDIFRX_GCLK_RATIO_MIN: c_uint = 12 * 64;

const SPDIFRX_CS_BITS: usize = 192;
const SPDIFRX_UD_BITS: usize = 192;

const SPDIFRX_CHANNELS: usize = 2;

/**
 * struct mchp_spdifrx_ch_stat: MCHP SPDIFRX channel status
 * @data: channel status bits
 * @done: completion to signal channel status bits acquisition done
 */
#[repr(C)]
pub struct mchp_spdifrx_ch_stat {
    pub data: [u8; SPDIFRX_CS_BITS / 8],
    pub done: completion,
}

/**
 * struct mchp_spdifrx_user_data: MCHP SPDIFRX user data
 * @data: user data bits
 * @done: completion to signal user data bits acquisition done
 */
#[repr(C)]
pub struct mchp_spdifrx_user_data {
    pub data: [u8; SPDIFRX_UD_BITS / 8],
    pub done: completion,
}

/**
 * struct mchp_spdifrx_mixer_control: MCHP SPDIFRX mixer control data structure
 * @ch_stat: array of channel statuses
 * @user_data: array of user data
 * @ulock: ulock bit status
 * @badf: badf bit status
 * @signal: signal bit status
 */
#[repr(C)]
pub struct mchp_spdifrx_mixer_control {
    pub ch_stat: [mchp_spdifrx_ch_stat; SPDIFRX_CHANNELS],
    pub user_data: [mchp_spdifrx_user_data; SPDIFRX_CHANNELS],
    pub ulock: bool_,
    pub badf: bool_,
    pub signal: bool_,
}

/**
 * struct mchp_spdifrx_dev: MCHP SPDIFRX device data structure
 * @capture: DAI DMA configuration data
 * @control: mixer controls
 * @mlock: mutex to protect concurency b/w configuration and control APIs
 * @dev: struct device
 * @regmap: regmap for this device
 * @pclk: peripheral clock
 * @gclk: generic clock
 * @trigger_enabled: true if enabled though trigger() ops
 */
#[repr(C)]
pub struct mchp_spdifrx_dev {
    pub capture: snd_dmaengine_dai_dma_data,
    pub control: mchp_spdifrx_mixer_control,
    pub mlock: mutex,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub pclk: *mut clk,
    pub gclk: *mut clk,
    pub trigger_enabled: c_uint,
}

extern "C" {
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut u32) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: u32) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: u32, val: u32) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn complete(x: *mut completion);
    fn init_completion(x: *mut completion);
    fn reinit_completion(x: *mut completion);
    fn wait_for_completion_interruptible_timeout(x: *mut completion, timeout: c_ulong) -> c_long;
    fn msecs_to_jiffies(m: c_uint) -> c_ulong;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_init(lock: *mut mutex);
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_enabled(dev: *mut device) -> bool_;
    fn pm_runtime_status_suspended(dev: *mut device) -> bool_;
    fn pm_runtime_disable(dev: *mut device);
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_set_min_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_dai;
    fn snd_soc_dai_init_dma_data(dai: *mut snd_soc_dai, playback: *const c_void, capture: *mut snd_dmaengine_dai_dma_data);
    fn snd_soc_add_dai_controls(dai: *mut snd_soc_dai, controls: *mut snd_kcontrol_new, num_controls: c_uint) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_get_and_ioremap_resource(pdev: *mut platform_device, index: c_uint, res: *mut *mut resource) -> *mut c_void;
    fn devm_regmap_init_mmio(dev: *mut device, base: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn platform_get_irq(pdev: *mut platform_device, index: c_uint) -> c_int;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>, flags: c_ulong, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn devm_snd_dmaengine_pcm_register(dev: *mut device, config: *const c_void, flags: c_uint) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
}

unsafe fn memcpy(dst: *mut u8, src: *const u8, n: usize) {
    ptr::copy_nonoverlapping(src, dst, n);
}

unsafe fn memset(dst: *mut u8, val: c_int, n: usize) {
    ptr::write_bytes(dst, val as u8, n);
}

unsafe fn mchp_spdifrx_channel_status_read(dev: *mut mchp_spdifrx_dev, channel: c_int) {
    let ctrl = &mut (*dev).control as *mut mchp_spdifrx_mixer_control;
    let mut ch_stat = (*ctrl).ch_stat[channel as usize].data.as_mut_ptr();
    let mut val: u32 = 0;
    let mut i: c_int = 0;

    while i < ((*ctrl).ch_stat[channel as usize].data.len() / 4) as c_int {
        regmap_read((*dev).regmap, SPDIFRX_CHSR(channel as c_uint, i as c_uint), &mut val);
        *ch_stat = FIELD_GET(SPDIFRX_BYTE_0_MASK, val) as u8;
        ch_stat = ch_stat.add(1);
        *ch_stat = FIELD_GET(SPDIFRX_BYTE_1_MASK, val) as u8;
        ch_stat = ch_stat.add(1);
        *ch_stat = FIELD_GET(SPDIFRX_BYTE_2_MASK, val) as u8;
        ch_stat = ch_stat.add(1);
        *ch_stat = FIELD_GET(SPDIFRX_BYTE_3_MASK, val) as u8;
        ch_stat = ch_stat.add(1);
        i += 1;
    }
}

unsafe fn mchp_spdifrx_channel_user_data_read(dev: *mut mchp_spdifrx_dev, channel: c_int) {
    let ctrl = &mut (*dev).control as *mut mchp_spdifrx_mixer_control;
    let mut user_data = (*ctrl).user_data[channel as usize].data.as_mut_ptr();
    let mut val: u32 = 0;
    let mut i: c_int = 0;

    while i < ((*ctrl).user_data[channel as usize].data.len() / 4) as c_int {
        regmap_read((*dev).regmap, SPDIFRX_CHUD(channel as c_uint, i as c_uint), &mut val);
        *user_data = FIELD_GET(SPDIFRX_BYTE_0_MASK, val) as u8;
        user_data = user_data.add(1);
        *user_data = FIELD_GET(SPDIFRX_BYTE_1_MASK, val) as u8;
        user_data = user_data.add(1);
        *user_data = FIELD_GET(SPDIFRX_BYTE_2_MASK, val) as u8;
        user_data = user_data.add(1);
        *user_data = FIELD_GET(SPDIFRX_BYTE_3_MASK, val) as u8;
        user_data = user_data.add(1);
        i += 1;
    }
}

unsafe extern "C" fn mchp_spdif_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let dev = dev_id as *mut mchp_spdifrx_dev;
    let ctrl = &mut (*dev).control as *mut mchp_spdifrx_mixer_control;
    let mut sr: u32 = 0;
    let mut imr: u32 = 0;
    let pending: u32;
    let mut ret: irqreturn_t = IRQ_NONE;
    let mut ch: c_int;

    regmap_read((*dev).regmap, SPDIFRX_ISR, &mut sr);
    regmap_read((*dev).regmap, SPDIFRX_IMR, &mut imr);
    pending = sr & imr;
    dev_dbg((*dev).dev, c"ISR: %#x, IMR: %#x, pending: %#x\n".as_ptr(), sr, imr, pending);

    if pending == 0 {
        return IRQ_NONE;
    }

    if pending & SPDIFRX_IR_BLOCKEND != 0 {
        ch = 0;
        while ch < SPDIFRX_CHANNELS as c_int {
            mchp_spdifrx_channel_user_data_read(dev, ch);
            complete(&mut (*ctrl).user_data[ch as usize].done);
            ch += 1;
        }
        regmap_write((*dev).regmap, SPDIFRX_IDR, SPDIFRX_IR_BLOCKEND);
        ret = IRQ_HANDLED;
    }

    ch = 0;
    while ch < SPDIFRX_CHANNELS as c_int {
        if pending & SPDIFRX_IR_CSC(ch as c_uint) != 0 {
            mchp_spdifrx_channel_status_read(dev, ch);
            complete(&mut (*ctrl).ch_stat[ch as usize].done);
            regmap_write((*dev).regmap, SPDIFRX_IDR, SPDIFRX_IR_CSC(ch as c_uint));
            ret = IRQ_HANDLED;
        }
        ch += 1;
    }

    if pending & SPDIFRX_IR_OVERRUN != 0 {
        dev_warn((*dev).dev, c"Overrun detected\n".as_ptr());
        ret = IRQ_HANDLED;
    }

    ret
}

unsafe extern "C" fn mchp_spdifrx_trigger(_substream: *mut snd_pcm_substream, cmd: c_int, dai: *mut snd_soc_dai) -> c_int {
    let dev = snd_soc_dai_get_drvdata(dai) as *mut mchp_spdifrx_dev;
    let mut ret: c_int = 0;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            mutex_lock(&mut (*dev).mlock);
            /* Enable overrun interrupts */
            regmap_write((*dev).regmap, SPDIFRX_IER, SPDIFRX_IR_OVERRUN);

            /* Enable receiver. */
            regmap_update_bits((*dev).regmap, SPDIFRX_MR, SPDIFRX_MR_RXEN_MASK, SPDIFRX_MR_RXEN_ENABLE);
            (*dev).trigger_enabled = true as c_uint;
            mutex_unlock(&mut (*dev).mlock);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            mutex_lock(&mut (*dev).mlock);
            /* Disable overrun interrupts */
            regmap_write((*dev).regmap, SPDIFRX_IDR, SPDIFRX_IR_OVERRUN);

            /* Disable receiver. */
            regmap_update_bits((*dev).regmap, SPDIFRX_MR, SPDIFRX_MR_RXEN_MASK, SPDIFRX_MR_RXEN_DISABLE);
            (*dev).trigger_enabled = false as c_uint;
            mutex_unlock(&mut (*dev).mlock);
        }
        _ => {
            ret = -EINVAL;
        }
    }

    ret
}

unsafe extern "C" fn mchp_spdifrx_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let dev = snd_soc_dai_get_drvdata(dai) as *mut mchp_spdifrx_dev;
    let mut mr: u32 = 0;
    let mut ret: c_int;

    dev_dbg((*dev).dev, c"%s() rate=%u format=%#x width=%u channels=%u\n".as_ptr(), c"mchp_spdifrx_hw_params".as_ptr(), params_rate(params), params_format(params), params_width(params), params_channels(params));

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        dev_err((*dev).dev, c"Playback is not supported\n".as_ptr());
        return -EINVAL;
    }

    if params_channels(params) != SPDIFRX_CHANNELS as c_uint {
        dev_err((*dev).dev, c"unsupported number of channels: %d\n".as_ptr(), params_channels(params));
        return -EINVAL;
    }

    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_BE | SNDRV_PCM_FORMAT_S20_3BE | SNDRV_PCM_FORMAT_S24_3BE | SNDRV_PCM_FORMAT_S24_BE => {
            mr |= SPDIFRX_MR_ENDIAN_BIG;
            mr |= SPDIFRX_MR_DATAWIDTH(params_width(params));
        }
        SNDRV_PCM_FORMAT_S16_LE | SNDRV_PCM_FORMAT_S20_3LE | SNDRV_PCM_FORMAT_S24_3LE | SNDRV_PCM_FORMAT_S24_LE => {
            mr |= SPDIFRX_MR_DATAWIDTH(params_width(params));
        }
        _ => {
            dev_err((*dev).dev, c"unsupported PCM format: %d\n".as_ptr(), params_format(params));
            return -EINVAL;
        }
    }

    mutex_lock(&mut (*dev).mlock);
    if (*dev).trigger_enabled != 0 {
        dev_err((*dev).dev, c"PCM already running\n".as_ptr());
        ret = -EBUSY;
        goto_unlock(dev, ret)
    } else {
        /* GCLK is enabled by runtime PM. */
        clk_disable_unprepare((*dev).gclk);

        ret = clk_set_min_rate((*dev).gclk, (params_rate(params) * SPDIFRX_GCLK_RATIO_MIN + 1) as c_ulong);
        if ret != 0 {
            dev_err((*dev).dev, c"unable to set gclk min rate: rate %u * ratio %u + 1\n".as_ptr(), params_rate(params), SPDIFRX_GCLK_RATIO_MIN);
            /* Restore runtime PM state. */
            clk_prepare_enable((*dev).gclk);
            goto_unlock(dev, ret)
        } else {
            ret = clk_prepare_enable((*dev).gclk);
            if ret != 0 {
                dev_err((*dev).dev, c"unable to enable gclk: %d\n".as_ptr(), ret);
                goto_unlock(dev, ret)
            } else {
                dev_dbg((*dev).dev, c"GCLK range min set to %d\n".as_ptr(), params_rate(params) * SPDIFRX_GCLK_RATIO_MIN + 1);
                ret = regmap_write((*dev).regmap, SPDIFRX_MR, mr);
                goto_unlock(dev, ret)
            }
        }
    }
}

unsafe fn goto_unlock(dev: *mut mchp_spdifrx_dev, ret: c_int) -> c_int {
    mutex_unlock(&mut (*dev).mlock);
    ret
}

const MCHP_SPDIF_RATES: c_uint = SNDRV_PCM_RATE_8000_192000;

const MCHP_SPDIF_FORMATS: c_ulong = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_U16_BE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S20_3BE
    | SNDRV_PCM_FMTBIT_S24_3LE
    | SNDRV_PCM_FMTBIT_S24_3BE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S24_BE;

unsafe extern "C" fn mchp_spdifrx_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
    (*uinfo).count = 1;
    0
}

unsafe fn mchp_spdifrx_cs_get(dev: *mut mchp_spdifrx_dev, channel: c_int, uvalue: *mut snd_ctl_elem_value) -> c_int {
    let ctrl = &mut (*dev).control as *mut mchp_spdifrx_mixer_control;
    let ch_stat = &mut (*ctrl).ch_stat[channel as usize] as *mut mchp_spdifrx_ch_stat;
    let mut ret: c_int = 0;

    mutex_lock(&mut (*dev).mlock);

    ret = pm_runtime_resume_and_get((*dev).dev);
    if ret >= 0 {
        /*
         * We may reach this point with both clocks enabled but the receiver
         * still disabled. To void waiting for completion and return with
         * timeout check the dev->trigger_enabled.
         *
         * To retrieve data:
         * - if the receiver is enabled CSC IRQ will update the data in software
         *   caches (ch_stat->data)
         * - otherwise we just update it here the software caches with latest
         *   available information and return it; in this case we don't need
         *   spin locking as the IRQ is disabled and will not be raised from
         *   anywhere else.
         */

        if (*dev).trigger_enabled != 0 {
            reinit_completion(&mut (*ch_stat).done);
            regmap_write((*dev).regmap, SPDIFRX_IER, SPDIFRX_IR_CSC(channel as c_uint));
            /* Check for new data available */
            ret = wait_for_completion_interruptible_timeout(&mut (*ch_stat).done, msecs_to_jiffies(100)) as c_int;
            /* Valid stream might not be present */
            if ret <= 0 {
                dev_dbg((*dev).dev, c"channel status for channel %d timeout\n".as_ptr(), channel);
                regmap_write((*dev).regmap, SPDIFRX_IDR, SPDIFRX_IR_CSC(channel as c_uint));
                ret = if ret != 0 { ret } else { -ETIMEDOUT };
                pm_runtime_put_autosuspend((*dev).dev);
                mutex_unlock(&mut (*dev).mlock);
                return ret;
            } else {
                ret = 0;
            }
        } else {
            /* Update software cache with latest channel status. */
            mchp_spdifrx_channel_status_read(dev, channel);
        }

        memcpy((*uvalue).value.iec958.status.as_mut_ptr(), (*ch_stat).data.as_ptr(), size_of_val(&(*ch_stat).data));
        pm_runtime_put_autosuspend((*dev).dev);
    }
    mutex_unlock(&mut (*dev).mlock);
    ret
}

unsafe extern "C" fn mchp_spdifrx_cs1_get(kcontrol: *mut snd_kcontrol, uvalue: *mut snd_ctl_elem_value) -> c_int {
    let dai = snd_kcontrol_chip(kcontrol);
    let dev = snd_soc_dai_get_drvdata(dai) as *mut mchp_spdifrx_dev;

    mchp_spdifrx_cs_get(dev, 0, uvalue)
}

unsafe extern "C" fn mchp_spdifrx_cs2_get(kcontrol: *mut snd_kcontrol, uvalue: *mut snd_ctl_elem_value) -> c_int {
    let dai = snd_kcontrol_chip(kcontrol);
    let dev = snd_soc_dai_get_drvdata(dai) as *mut mchp_spdifrx_dev;

    mchp_spdifrx_cs_get(dev, 1, uvalue)
}

unsafe extern "C" fn mchp_spdifrx_cs_mask(_kcontrol: *mut snd_kcontrol, uvalue: *mut snd_ctl_elem_value) -> c_int {
    memset((*uvalue).value.iec958.status.as_mut_ptr(), 0xff, size_of_val(&(*uvalue).value.iec958.status));

    0
}

unsafe fn mchp_spdifrx_subcode_ch_get(dev: *mut mchp_spdifrx_dev, channel: c_int, uvalue: *mut snd_ctl_elem_value) -> c_int {
    let ctrl = &mut (*dev).control as *mut mchp_spdifrx_mixer_control;
    let user_data = &mut (*ctrl).user_data[channel as usize] as *mut mchp_spdifrx_user_data;
    let mut ret: c_int = 0;

    mutex_lock(&mut (*dev).mlock);

    ret = pm_runtime_resume_and_get((*dev).dev);
    if ret >= 0 {
        /*
         * We may reach this point with both clocks enabled but the receiver
         * still disabled. To void waiting for completion to just timeout we
         * check here the dev->trigger_enabled flag.
         *
         * To retrieve data:
         * - if the receiver is enabled we need to wait for blockend IRQ to read
         *   data to and update it for us in software caches
         * - otherwise reading the SPDIFRX_CHUD() registers is enough.
         */

        if (*dev).trigger_enabled != 0 {
            reinit_completion(&mut (*user_data).done);
            regmap_write((*dev).regmap, SPDIFRX_IER, SPDIFRX_IR_BLOCKEND);
            ret = wait_for_completion_interruptible_timeout(&mut (*user_data).done, msecs_to_jiffies(100)) as c_int;
            /* Valid stream might not be present. */
            if ret <= 0 {
                dev_dbg((*dev).dev, c"user data for channel %d timeout\n".as_ptr(), channel);
                regmap_write((*dev).regmap, SPDIFRX_IDR, SPDIFRX_IR_BLOCKEND);
                ret = if ret != 0 { ret } else { -ETIMEDOUT };
                pm_runtime_put_autosuspend((*dev).dev);
                mutex_unlock(&mut (*dev).mlock);
                return ret;
            } else {
                ret = 0;
            }
        } else {
            /* Update software cache with last available data. */
            mchp_spdifrx_channel_user_data_read(dev, channel);
        }

        memcpy((*uvalue).value.iec958.subcode.as_mut_ptr(), (*user_data).data.as_ptr(), size_of_val(&(*user_data).data));
        pm_runtime_put_autosuspend((*dev).dev);
    }
    mutex_unlock(&mut (*dev).mlock);
    ret
}

unsafe extern "C" fn mchp_spdifrx_subcode_ch1_get(kcontrol: *mut snd_kcontrol, uvalue: *mut snd_ctl_elem_value) -> c_int {
    let dai = snd_kcontrol_chip(kcontrol);
    let dev = snd_soc_dai_get_drvdata(dai) as *mut mchp_spdifrx_dev;

    mchp_spdifrx_subcode_ch_get(dev, 0, uvalue)
}

unsafe extern "C" fn mchp_spdifrx_subcode_ch2_get(kcontrol: *mut snd_kcontrol, uvalue: *mut snd_ctl_elem_value) -> c_int {
    let dai = snd_kcontrol_chip(kcontrol);
    let dev = snd_soc_dai_get_drvdata(dai) as *mut mchp_spdifrx_dev;

    mchp_spdifrx_subcode_ch_get(dev, 1, uvalue)
}

unsafe extern "C" fn mchp_spdifrx_boolean_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 1;

    0
}

unsafe extern "C" fn mchp_spdifrx_ulock_get(kcontrol: *mut snd_kcontrol, uvalue: *mut snd_ctl_elem_value) -> c_int {
    let dai = snd_kcontrol_chip(kcontrol);
    let dev = snd_soc_dai_get_drvdata(dai) as *mut mchp_spdifrx_dev;
    let ctrl = &mut (*dev).control as *mut mchp_spdifrx_mixer_control;
    let mut val: u32 = 0;
    let mut ret: c_int;
    let ulock_old = (*ctrl).ulock;

    mutex_lock(&mut (*dev).mlock);

    ret = pm_runtime_resume_and_get((*dev).dev);
    if ret >= 0 {
        /*
         * The RSR.ULOCK has wrong value if both pclk and gclk are enabled
         * and the receiver is disabled. Thus we take into account the
         * dev->trigger_enabled here to return a real status.
         */
        if (*dev).trigger_enabled != 0 {
            regmap_read((*dev).regmap, SPDIFRX_RSR, &mut val);
            (*ctrl).ulock = !(val & SPDIFRX_RSR_ULOCK != 0);
        } else {
            (*ctrl).ulock = false;
        }

        (*uvalue).value.integer.value[0] = (*ctrl).ulock as c_long;

        pm_runtime_put_autosuspend((*dev).dev);
    }
    mutex_unlock(&mut (*dev).mlock);

    (ulock_old != (*ctrl).ulock) as c_int
}

unsafe extern "C" fn mchp_spdifrx_badf_get(kcontrol: *mut snd_kcontrol, uvalue: *mut snd_ctl_elem_value) -> c_int {
    let dai = snd_kcontrol_chip(kcontrol);
    let dev = snd_soc_dai_get_drvdata(dai) as *mut mchp_spdifrx_dev;
    let ctrl = &mut (*dev).control as *mut mchp_spdifrx_mixer_control;
    let mut val: u32 = 0;
    let mut ret: c_int;
    let badf_old = (*ctrl).badf;

    mutex_lock(&mut (*dev).mlock);

    ret = pm_runtime_resume_and_get((*dev).dev);
    if ret >= 0 {
        /*
         * The RSR.ULOCK has wrong value if both pclk and gclk are enabled
         * and the receiver is disabled. Thus we take into account the
         * dev->trigger_enabled here to return a real status.
         */
        if (*dev).trigger_enabled != 0 {
            regmap_read((*dev).regmap, SPDIFRX_RSR, &mut val);
            (*ctrl).badf = val & SPDIFRX_RSR_BADF != 0;
        } else {
            (*ctrl).badf = false;
        }

        pm_runtime_put_autosuspend((*dev).dev);
    }
    mutex_unlock(&mut (*dev).mlock);

    (*uvalue).value.integer.value[0] = (*ctrl).badf as c_long;

    (badf_old != (*ctrl).badf) as c_int
}

unsafe extern "C" fn mchp_spdifrx_signal_get(kcontrol: *mut snd_kcontrol, uvalue: *mut snd_ctl_elem_value) -> c_int {
    let dai = snd_kcontrol_chip(kcontrol);
    let dev = snd_soc_dai_get_drvdata(dai) as *mut mchp_spdifrx_dev;
    let ctrl = &mut (*dev).control as *mut mchp_spdifrx_mixer_control;
    let mut val: u32 = !0u32;
    let mut loops: u32 = 10;
    let mut ret: c_int;
    let signal_old = (*ctrl).signal;

    mutex_lock(&mut (*dev).mlock);

    ret = pm_runtime_resume_and_get((*dev).dev);
    if ret >= 0 {
        /*
         * To get the signal we need to have receiver enabled. This
         * could be enabled also from trigger() function thus we need to
         * take care of not disabling the receiver when it runs.
         */
        if (*dev).trigger_enabled == 0 {
            regmap_update_bits((*dev).regmap, SPDIFRX_MR, SPDIFRX_MR_RXEN_MASK, SPDIFRX_MR_RXEN_ENABLE);

            /* Wait for RSR.ULOCK bit. */
            loop {
                loops = loops.wrapping_sub(1);
                if loops == 0 {
                    break;
                }
                regmap_read((*dev).regmap, SPDIFRX_RSR, &mut val);
                if !(val & SPDIFRX_RSR_ULOCK != 0) {
                    break;
                }
                usleep_range(100, 150);
            }

            regmap_update_bits((*dev).regmap, SPDIFRX_MR, SPDIFRX_MR_RXEN_MASK, SPDIFRX_MR_RXEN_DISABLE);
        } else {
            regmap_read((*dev).regmap, SPDIFRX_RSR, &mut val);
        }

        pm_runtime_put_autosuspend((*dev).dev);
    }

    mutex_unlock(&mut (*dev).mlock);

    if !(val & SPDIFRX_RSR_ULOCK != 0) {
        (*ctrl).signal = !(val & SPDIFRX_RSR_NOSIGNAL != 0);
    } else {
        (*ctrl).signal = false;
    }
    (*uvalue).value.integer.value[0] = (*ctrl).signal as c_long;

    (signal_old != (*ctrl).signal) as c_int
}

unsafe extern "C" fn mchp_spdifrx_rate_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 192000;

    0
}

unsafe extern "C" fn mchp_spdifrx_rate_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dai = snd_kcontrol_chip(kcontrol);
    let dev = snd_soc_dai_get_drvdata(dai) as *mut mchp_spdifrx_dev;
    let rate: c_ulong;
    let mut val: u32 = 0;
    let mut ret: c_int;

    mutex_lock(&mut (*dev).mlock);

    ret = pm_runtime_resume_and_get((*dev).dev);
    if ret >= 0 {
        /*
         * The RSR.ULOCK has wrong value if both pclk and gclk are enabled
         * and the receiver is disabled. Thus we take into account the
         * dev->trigger_enabled here to return a real status.
         */
        if (*dev).trigger_enabled != 0 {
            regmap_read((*dev).regmap, SPDIFRX_RSR, &mut val);
            /* If the receiver is not locked, ISF data is invalid. */
            if val & SPDIFRX_RSR_ULOCK != 0 || !(val & SPDIFRX_RSR_IFS_MASK != 0) {
                (*ucontrol).value.integer.value[0] = 0;
                pm_runtime_put_autosuspend((*dev).dev);
                mutex_unlock(&mut (*dev).mlock);
                return ret;
            }
        } else {
            /* Reveicer is not locked, IFS data is invalid. */
            (*ucontrol).value.integer.value[0] = 0;
            pm_runtime_put_autosuspend((*dev).dev);
            mutex_unlock(&mut (*dev).mlock);
            return ret;
        }

        rate = clk_get_rate((*dev).gclk);

        (*ucontrol).value.integer.value[0] = (rate / (32 * SPDIFRX_RSR_IFS(val)) as c_ulong) as c_long;

        pm_runtime_put_autosuspend((*dev).dev);
    }
    mutex_unlock(&mut (*dev).mlock);
    ret
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

static mut mchp_spdifrx_ctrls: [snd_kcontrol_new; 9] = [
    /* Channel status controller */
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: cstr!("IEC958 Capture Default Channel 1"),
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(mchp_spdifrx_info),
        get: Some(mchp_spdifrx_cs1_get),
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: cstr!("IEC958 Capture Default Channel 2"),
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(mchp_spdifrx_info),
        get: Some(mchp_spdifrx_cs2_get),
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: cstr!("IEC958 Capture Mask"),
        access: SNDRV_CTL_ELEM_ACCESS_READ,
        info: Some(mchp_spdifrx_info),
        get: Some(mchp_spdifrx_cs_mask),
    },
    /* User bits controller */
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: cstr!("IEC958 Subcode Capture Default Channel 1"),
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(mchp_spdifrx_info),
        get: Some(mchp_spdifrx_subcode_ch1_get),
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: cstr!("IEC958 Subcode Capture Default Channel 2"),
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(mchp_spdifrx_info),
        get: Some(mchp_spdifrx_subcode_ch2_get),
    },
    /* Lock status */
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: cstr!("IEC958 Capture None Unlocked"),
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(mchp_spdifrx_boolean_info),
        get: Some(mchp_spdifrx_ulock_get),
    },
    /* Bad format */
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: cstr!("IEC958 Capture NoneBad Format"),
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(mchp_spdifrx_boolean_info),
        get: Some(mchp_spdifrx_badf_get),
    },
    /* Signal */
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: cstr!("IEC958 Capture None Signal"),
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(mchp_spdifrx_boolean_info),
        get: Some(mchp_spdifrx_signal_get),
    },
    /* Sampling rate */
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: cstr!("IEC958 Capture None Rate"),
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(mchp_spdifrx_rate_info),
        get: Some(mchp_spdifrx_rate_get),
    },
];

unsafe extern "C" fn mchp_spdifrx_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let dev = snd_soc_dai_get_drvdata(dai) as *mut mchp_spdifrx_dev;
    let ctrl = &mut (*dev).control as *mut mchp_spdifrx_mixer_control;
    let mut ch: c_int;

    snd_soc_dai_init_dma_data(dai, ptr::null(), &mut (*dev).capture);

    /* Software reset the IP */
    regmap_write((*dev).regmap, SPDIFRX_CR, SPDIFRX_CR_SWRST);

    /* Default configuration */
    regmap_write(
        (*dev).regmap,
        SPDIFRX_MR,
        SPDIFRX_MR_VBMODE_DISCARD_IF_VB1
            | SPDIFRX_MR_SBMODE_DISCARD
            | SPDIFRX_MR_AUTORST_NOACTION
            | SPDIFRX_MR_PACK_DISABLED,
    );

    ch = 0;
    while ch < SPDIFRX_CHANNELS as c_int {
        init_completion(&mut (*ctrl).ch_stat[ch as usize].done);
        init_completion(&mut (*ctrl).user_data[ch as usize].done);
        ch += 1;
    }

    /* Add controls */
    snd_soc_add_dai_controls(dai, mchp_spdifrx_ctrls.as_mut_ptr(), mchp_spdifrx_ctrls.len() as c_uint);

    0
}

unsafe extern "C" fn mchp_spdifrx_dai_remove(dai: *mut snd_soc_dai) -> c_int {
    let dev = snd_soc_dai_get_drvdata(dai) as *mut mchp_spdifrx_dev;

    /* Disable interrupts */
    regmap_write((*dev).regmap, SPDIFRX_IDR, GENMASK(14, 0));

    0
}

static mchp_spdifrx_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(mchp_spdifrx_dai_probe),
    remove: Some(mchp_spdifrx_dai_remove),
    trigger: Some(mchp_spdifrx_trigger),
    hw_params: Some(mchp_spdifrx_hw_params),
};

static mut mchp_spdifrx_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: cstr!("mchp-spdifrx"),
    capture: snd_soc_pcm_stream {
        stream_name: cstr!("Capture"),
        channels_min: SPDIFRX_CHANNELS as c_uint,
        channels_max: SPDIFRX_CHANNELS as c_uint,
        rates: MCHP_SPDIF_RATES,
        formats: MCHP_SPDIF_FORMATS,
    },
    ops: &mchp_spdifrx_dai_ops,
};

static mchp_spdifrx_component: snd_soc_component_driver = snd_soc_component_driver {
    name: cstr!("mchp-spdifrx"),
    legacy_dai_naming: 1,
};

static mchp_spdifrx_dt_ids: [of_device_id; 2] = [
    of_device_id {
        compatible: cstr!("microchip,sama7g5-spdifrx"),
    },
    of_device_id {
        /* sentinel */
        compatible: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, mchp_spdifrx_dt_ids); */

unsafe extern "C" fn mchp_spdifrx_runtime_suspend(dev: *mut device) -> c_int {
    let spdifrx = dev_get_drvdata(dev) as *mut mchp_spdifrx_dev;

    regcache_cache_only((*spdifrx).regmap, true);
    clk_disable_unprepare((*spdifrx).gclk);
    clk_disable_unprepare((*spdifrx).pclk);

    0
}

unsafe extern "C" fn mchp_spdifrx_runtime_resume(dev: *mut device) -> c_int {
    let spdifrx = dev_get_drvdata(dev) as *mut mchp_spdifrx_dev;
    let mut ret: c_int;

    ret = clk_prepare_enable((*spdifrx).pclk);
    if ret != 0 {
        return ret;
    }

    ret = clk_prepare_enable((*spdifrx).gclk);
    if ret != 0 {
        clk_disable_unprepare((*spdifrx).pclk);
        return ret;
    }

    regcache_cache_only((*spdifrx).regmap, false);
    regcache_mark_dirty((*spdifrx).regmap);
    ret = regcache_sync((*spdifrx).regmap);
    if ret != 0 {
        regcache_cache_only((*spdifrx).regmap, true);
        clk_disable_unprepare((*spdifrx).gclk);
        clk_disable_unprepare((*spdifrx).pclk);
    }

    ret
}

static mchp_spdifrx_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(mchp_spdifrx_runtime_suspend),
    runtime_resume: Some(mchp_spdifrx_runtime_resume),
};

unsafe extern "C" fn mchp_spdifrx_probe(pdev: *mut platform_device) -> c_int {
    let mut dev: *mut mchp_spdifrx_dev;
    let mut mem: *mut resource = ptr::null_mut();
    let mut regmap_: *mut regmap;
    let base: *mut c_void;
    let mut irq: c_int;
    let mut err: c_int;
    let mut vers: u32 = 0;

    /* Get memory for driver data. */
    dev = devm_kzalloc(&mut (*pdev).dev, size_of::<mchp_spdifrx_dev>(), GFP_KERNEL) as *mut mchp_spdifrx_dev;
    if dev.is_null() {
        return -ENOMEM;
    }

    /* Map I/O registers. */
    base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut mem);
    if IS_ERR(base) {
        return PTR_ERR(base);
    }

    regmap_ = devm_regmap_init_mmio(&mut (*pdev).dev, base, &mchp_spdifrx_regmap_config);
    if IS_ERR(regmap_ as *const c_void) {
        return PTR_ERR(regmap_ as *const c_void);
    }

    /* Request IRQ. */
    irq = platform_get_irq(pdev, 0);
    if irq < 0 {
        return irq;
    }

    err = devm_request_irq(&mut (*pdev).dev, irq, Some(mchp_spdif_interrupt), 0, dev_name(&mut (*pdev).dev), dev as *mut c_void);
    if err != 0 {
        return err;
    }

    /* Get the peripheral clock */
    (*dev).pclk = devm_clk_get(&mut (*pdev).dev, c"pclk".as_ptr());
    if IS_ERR((*dev).pclk as *const c_void) {
        err = PTR_ERR((*dev).pclk as *const c_void);
        dev_err(&mut (*pdev).dev, c"failed to get the peripheral clock: %d\n".as_ptr(), err);
        return err;
    }

    /* Get the generated clock */
    (*dev).gclk = devm_clk_get(&mut (*pdev).dev, c"gclk".as_ptr());
    if IS_ERR((*dev).gclk as *const c_void) {
        err = PTR_ERR((*dev).gclk as *const c_void);
        dev_err(&mut (*pdev).dev, c"failed to get the PMC generated clock: %d\n".as_ptr(), err);
        return err;
    }

    /*
     * Signal control need a valid rate on gclk. hw_params() configures
     * it propertly but requesting signal before any hw_params() has been
     * called lead to invalid value returned for signal. Thus, configure
     * gclk at a valid rate, here, in initialization, to simplify the
     * control path.
     */
    clk_set_min_rate((*dev).gclk, (48000 * SPDIFRX_GCLK_RATIO_MIN + 1) as c_ulong);

    mutex_init(&mut (*dev).mlock);

    (*dev).dev = &mut (*pdev).dev;
    (*dev).regmap = regmap_;
    platform_set_drvdata(pdev, dev as *mut c_void);

    pm_runtime_enable((*dev).dev);
    if !pm_runtime_enabled((*dev).dev) {
        err = mchp_spdifrx_runtime_resume((*dev).dev);
        if err != 0 {
            pm_runtime_disable((*dev).dev);
            return err;
        }
    }

    (*dev).capture.addr = (*mem).start as dma_addr_t + SPDIFRX_RHR as dma_addr_t;
    (*dev).capture.maxburst = 1;

    err = devm_snd_dmaengine_pcm_register(&mut (*pdev).dev, ptr::null(), 0);
    if err != 0 {
        dev_err(&mut (*pdev).dev, c"failed to register PCM: %d\n".as_ptr(), err);
        if !pm_runtime_status_suspended((*dev).dev) {
            mchp_spdifrx_runtime_suspend((*dev).dev);
        }
        pm_runtime_disable((*dev).dev);
        return err;
    }

    err = devm_snd_soc_register_component(&mut (*pdev).dev, &mchp_spdifrx_component, &mut mchp_spdifrx_dai, 1);
    if err != 0 {
        dev_err(&mut (*pdev).dev, c"fail to register dai\n".as_ptr());
        if !pm_runtime_status_suspended((*dev).dev) {
            mchp_spdifrx_runtime_suspend((*dev).dev);
        }
        pm_runtime_disable((*dev).dev);
        return err;
    }

    regmap_read(regmap_, SPDIFRX_VERSION, &mut vers);
    dev_info(&mut (*pdev).dev, c"hw version: %#lx\n".as_ptr(), vers & SPDIFRX_VERSION_MASK);

    0
}

unsafe extern "C" fn mchp_spdifrx_remove(pdev: *mut platform_device) {
    let dev = platform_get_drvdata(pdev) as *mut mchp_spdifrx_dev;

    pm_runtime_disable((*dev).dev);
    if !pm_runtime_status_suspended((*dev).dev) {
        mchp_spdifrx_runtime_suspend((*dev).dev);
    }
}

static mut mchp_spdifrx_driver: platform_driver = platform_driver {
    probe: Some(mchp_spdifrx_probe),
    remove: Some(mchp_spdifrx_remove),
    driver: driver {
        name: cstr!("mchp_spdifrx"),
        of_match_table: mchp_spdifrx_dt_ids.as_ptr(),
        pm: &mchp_spdifrx_pm_ops,
    },
};

/* module_platform_driver(mchp_spdifrx_driver); */

/* MODULE_AUTHOR("Codrin Ciubotariu <codrin.ciubotariu@microchip.com>"); */
/* MODULE_DESCRIPTION("Microchip S/PDIF RX Controller Driver"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
