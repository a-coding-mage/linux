// SPDX-License-Identifier: GPL-2.0-only
// ALSA SoC Audio Layer - Rockchip I2S/TDM Controller driver
//
// Copyright (c) 2018 Rockchip Electronics Co. Ltd.
// Author: Sugar Zhang <sugar.zhang@rock-chips.com>
// Author: Nicolas Frattaroli <frattaroli.nicolas@gmail.com>
//
// Translated from C. Linux, ALSA, regmap, clk, reset, OF, and
// rockchip_i2s_tdm.h symbols are intentionally referenced as external
// dependencies supplied by the surrounding repository.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr::{null, null_mut};

type u32 = c_uint;
type u64 = c_ulong;
type bool_t = bool;

const DRV_NAME: &[u8] = b"rockchip-i2s-tdm\0";
const DEFAULT_MCLK_FS: c_uint = 256;
const CH_GRP_MAX: usize = 4; /* The max channel 8 / 2 */
const MULTIPLEX_CH_MAX: c_int = 10;

const TRCM_TXRX: c_uint = 0;
const TRCM_TX: c_uint = 1;
const TRCM_RX: c_uint = 2;

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}
#[repr(C)]
pub struct device_node {
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
pub struct reset_control {
    _private: [u8; 0],
}
#[repr(C)]
pub struct property {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr: c_ulong,
    pub addr_width: c_uint,
    pub maxburst: c_uint,
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
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
}
#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_bclk_ratio: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub legacy_dai_naming: c_uint,
}
#[repr(C)]
pub struct snd_pcm_str {
    pub substream_opened: c_int,
}
#[repr(C)]
pub struct snd_pcm {
    pub streams: [snd_pcm_str; 2],
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
    pub pcm: *mut snd_pcm,
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
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub precious_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub cache_type: c_uint,
}
#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
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
pub struct dev_pm_ops {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}
#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: device_driver,
}
#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
struct txrx_config {
    addr: u32,
    reg: u32,
    txonly: u32,
    rxonly: u32,
}

#[repr(C)]
struct rk_i2s_soc_data {
    softrst_offset: u32,
    grf_reg_offset: u32,
    grf_shift: u32,
    config_count: c_int,
    configs: *const txrx_config,
    init: Option<unsafe extern "C" fn(*mut device, u32) -> c_int>,
}

#[repr(C)]
struct rk_i2s_tdm_dev {
    dev: *mut device,
    hclk: *mut clk,
    mclk_tx: *mut clk,
    mclk_rx: *mut clk,
    regmap: *mut regmap,
    grf: *mut regmap,
    capture_dma_data: snd_dmaengine_dai_dma_data,
    playback_dma_data: snd_dmaengine_dai_dma_data,
    tx_reset: *mut reset_control,
    rx_reset: *mut reset_control,
    soc_data: *const rk_i2s_soc_data,
    is_master_mode: bool_t,
    io_multiplex: bool_t,
    tdm_mode: bool_t,
    frame_width: c_uint,
    clk_trcm: c_uint,
    i2s_sdis: [c_uint; CH_GRP_MAX],
    i2s_sdos: [c_uint; CH_GRP_MAX],
    refcount: c_int,
    lock: spinlock_t, /* xfer lock */
    has_playback: bool_t,
    has_capture: bool_t,
    dai: *mut snd_soc_dai_driver,
    mclk_rx_freq: c_uint,
    mclk_tx_freq: c_uint,
}

unsafe extern "C" {
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn clk_get_rate(clk: *mut clk) -> c_uint;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_dai_dma_data_set_capture(dai: *mut snd_soc_dai, data: *mut snd_dmaengine_dai_dma_data);
    fn snd_soc_dai_dma_data_set_playback(dai: *mut snd_soc_dai, data: *mut snd_dmaengine_dai_dma_data);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn reset_control_assert(rc: *mut reset_control) -> c_int;
    fn reset_control_deassert(rc: *mut reset_control) -> c_int;
    fn udelay(usecs: c_uint);
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put(dev: *mut device) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_status_suspended(dev: *mut device) -> bool;
    fn syscon_regmap_lookup_by_phandle(np: *mut device_node, property: *const c_char) -> *mut regmap;
    fn devm_reset_control_get_optional_exclusive(dev: *mut device, id: *const c_char) -> *mut reset_control;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_platform_get_and_ioremap_resource(pdev: *mut platform_device, index: c_uint, res: *mut *mut resource) -> *mut c_void;
    fn devm_regmap_init_mmio(dev: *mut device, regs: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kmemdup(dev: *mut device, src: *const c_void, len: usize, flags: c_uint) -> *mut c_void;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn devm_snd_dmaengine_pcm_register(dev: *mut device, config: *const c_void, flags: c_uint) -> c_int;
    fn device_get_match_data(dev: *mut device) -> *const c_void;
    fn of_property_read_bool(np: *mut device_node, propname: *const c_char) -> bool;
    fn of_count_phandle_with_args(np: *mut device_node, list_name: *const c_char, cells_name: *const c_char) -> c_int;
    fn of_property_read_u32_array(np: *mut device_node, propname: *const c_char, out_values: *mut c_uint, sz: c_int) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn IS_ERR_OR_NULL(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
}

// Constants and register helper macros from Linux, ALSA, and rockchip_i2s_tdm.h.
unsafe extern "C" {
    static EACCES: c_int; static EINVAL: c_int; static ENOMEM: c_int; static ENOENT: c_int;
    static GFP_KERNEL: c_uint; static REGCACHE_FLAT: c_uint;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int; static SNDRV_PCM_STREAM_CAPTURE: c_int; static SNDRV_PCM_STREAM_LAST: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int; static SNDRV_PCM_TRIGGER_RESUME: c_int; static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int; static SNDRV_PCM_TRIGGER_STOP: c_int; static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
    static SNDRV_PCM_FORMAT_S8: c_int; static SNDRV_PCM_FORMAT_S16_LE: c_int; static SNDRV_PCM_FORMAT_S20_3LE: c_int;
    static SNDRV_PCM_FORMAT_S24_LE: c_int; static SNDRV_PCM_FORMAT_S32_LE: c_int;
    static SNDRV_PCM_FMTBIT_S8: u64; static SNDRV_PCM_FMTBIT_S16_LE: u64; static SNDRV_PCM_FMTBIT_S20_3LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64; static SNDRV_PCM_FMTBIT_S32_LE: u64; static SNDRV_PCM_RATE_8000_192000: c_uint;
    static DMA_SLAVE_BUSWIDTH_4_BYTES: c_uint;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint; static SND_SOC_DAIFMT_BP_FP: c_uint; static SND_SOC_DAIFMT_BC_FC: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint; static SND_SOC_DAIFMT_NB_NF: c_uint; static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint; static SND_SOC_DAIFMT_IB_IF: c_uint; static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint; static SND_SOC_DAIFMT_LEFT_J: c_uint; static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint; static SND_SOC_DAIFMT_DSP_B: c_uint;
    static I2S_CHN_2: c_uint; static I2S_CHN_4: c_uint; static I2S_CHN_6: c_uint; static I2S_CHN_8: c_uint;
    static I2S_TXCR: c_uint; static I2S_RXCR: c_uint; static I2S_CKR: c_uint; static I2S_DMACR: c_uint; static I2S_INTCR: c_uint;
    static I2S_XFER: c_uint; static I2S_CLR: c_uint; static I2S_TXDR: c_uint; static I2S_RXDR: c_uint; static I2S_TXFIFOLR: c_uint;
    static I2S_INTSR: c_uint; static I2S_RXFIFOLR: c_uint; static I2S_TDM_TXCR: c_uint; static I2S_TDM_RXCR: c_uint; static I2S_CLKDIV: c_uint;
    static I2S_CLR_TXC: c_uint; static I2S_CLR_RXC: c_uint; static I2S_XFER_TXS_START: c_uint; static I2S_XFER_TXS_STOP: c_uint;
    static I2S_XFER_RXS_START: c_uint; static I2S_XFER_RXS_STOP: c_uint;
    static I2S_DMACR_TDE_ENABLE: c_uint; static I2S_DMACR_TDE_DISABLE: c_uint; static I2S_DMACR_RDE_ENABLE: c_uint; static I2S_DMACR_RDE_DISABLE: c_uint;
    static I2S_DMACR_TDL_MASK: c_uint; static I2S_DMACR_RDL_MASK: c_uint;
    static I2S_CKR_MSS_MASK: c_uint; static I2S_CKR_MSS_MASTER: c_uint; static I2S_CKR_MSS_SLAVE: c_uint;
    static I2S_CKR_CKP_MASK: c_uint; static I2S_CKR_TLP_MASK: c_uint; static I2S_CKR_RLP_MASK: c_uint;
    static I2S_CKR_CKP_NORMAL: c_uint; static I2S_CKR_TLP_NORMAL: c_uint; static I2S_CKR_RLP_NORMAL: c_uint;
    static I2S_CKR_CKP_INVERTED: c_uint; static I2S_CKR_TLP_INVERTED: c_uint; static I2S_CKR_RLP_INVERTED: c_uint;
    static I2S_CKR_TSD_MASK: c_uint; static I2S_CKR_RSD_MASK: c_uint; static I2S_CKR_TRCM_MASK: c_uint; static I2S_CKR_TRCM_SHIFT: c_uint;
    static I2S_TXCR_IBM_MASK: c_uint; static I2S_TXCR_TFS_MASK: c_uint; static I2S_TXCR_PBM_MASK: c_uint; static I2S_TXCR_CSR_MASK: c_uint; static I2S_TXCR_VDW_MASK: c_uint;
    static I2S_TXCR_IBM_RSJM: c_uint; static I2S_TXCR_IBM_LSJM: c_uint; static I2S_TXCR_IBM_NORMAL: c_uint; static I2S_TXCR_TFS_PCM: c_uint;
    static I2S_TXCR_TFS_TDM_I2S: c_uint; static I2S_TXCR_TFS_TDM_PCM: c_uint;
    static I2S_RXCR_IBM_MASK: c_uint; static I2S_RXCR_TFS_MASK: c_uint; static I2S_RXCR_PBM_MASK: c_uint; static I2S_RXCR_CSR_MASK: c_uint; static I2S_RXCR_VDW_MASK: c_uint;
    static I2S_RXCR_IBM_RSJM: c_uint; static I2S_RXCR_IBM_LSJM: c_uint; static I2S_RXCR_IBM_NORMAL: c_uint; static I2S_RXCR_TFS_PCM: c_uint;
    static I2S_CLKDIV_TXM_MASK: c_uint; static I2S_CLKDIV_RXM_MASK: c_uint; static I2S_IO_DIRECTION_MASK: c_uint;
    static TDM_FSYNC_WIDTH_SEL1_MSK: c_uint; static TDM_FSYNC_WIDTH_SEL0_MSK: c_uint; static TDM_SHIFT_CTRL_MSK: c_uint;
    static TDM_FSYNC_WIDTH_HALF_FRAME: c_uint; static TDM_SLOT_BIT_WIDTH_MSK: c_uint; static TDM_FRAME_WIDTH_MSK: c_uint;
    static PX30_I2S0_CLK_TXONLY: c_uint; static PX30_I2S0_CLK_RXONLY: c_uint;
    static RK1808_I2S0_CLK_TXONLY: c_uint; static RK1808_I2S0_CLK_RXONLY: c_uint;
    static RK3308_I2S0_CLK_TXONLY: c_uint; static RK3308_I2S0_CLK_RXONLY: c_uint; static RK3308_I2S1_CLK_TXONLY: c_uint; static RK3308_I2S1_CLK_RXONLY: c_uint;
    static RK3568_I2S1_CLK_TXONLY: c_uint; static RK3568_I2S1_CLK_RXONLY: c_uint; static RK3568_I2S1_MCLK_TX_OE: c_uint; static RK3568_I2S1_MCLK_RX_OE: c_uint;
    static RK3568_I2S2_MCLK_OE: c_uint; static RK3568_I2S3_CLK_TXONLY: c_uint; static RK3568_I2S3_CLK_RXONLY: c_uint;
    static RK3568_I2S3_MCLK_TXONLY: c_uint; static RK3568_I2S3_MCLK_RXONLY: c_uint; static RK3568_I2S3_MCLK_OE: c_uint;
    static RV1126_I2S0_CLK_TXONLY: c_uint; static RV1126_I2S0_CLK_RXONLY: c_uint;
}

unsafe extern "C" {
    fn I2S_TXCR_PBM_MODE(v: c_uint) -> c_uint; fn I2S_RXCR_PBM_MODE(v: c_uint) -> c_uint;
    fn TDM_SHIFT_CTRL(v: c_uint) -> c_uint; fn TDM_FSYNC_WIDTH_SEL1(v: c_uint) -> c_uint;
    fn I2S_CLKDIV_TXM(v: c_uint) -> c_uint; fn I2S_CLKDIV_RXM(v: c_uint) -> c_uint;
    fn I2S_CKR_TSD(v: c_uint) -> c_uint; fn I2S_CKR_RSD(v: c_uint) -> c_uint;
    fn I2S_TXCR_VDW(v: c_uint) -> c_uint; fn I2S_RXCR_VDW(v: c_uint) -> c_uint;
    fn TDM_SLOT_BIT_WIDTH(v: c_int) -> c_uint; fn TDM_FRAME_WIDTH(v: c_int) -> c_uint;
    fn I2S_DMACR_TDL(v: c_uint) -> c_uint; fn I2S_DMACR_RDL(v: c_uint) -> c_uint;
    fn I2S_TXCR_PATH_MASK(idx: c_int) -> c_uint; fn I2S_TXCR_PATH(idx: c_int, val: c_uint) -> c_uint;
    fn I2S_RXCR_PATH_MASK(idx: c_int) -> c_uint; fn I2S_RXCR_PATH(idx: c_int, val: c_uint) -> c_uint;
}

#[inline]
unsafe fn div_round_closest(x: c_uint, divisor: c_uint) -> c_uint {
    (x + divisor / 2) / divisor
}

unsafe fn to_ch_num(val: c_uint) -> c_int {
    if val == I2S_CHN_4 { 4 } else if val == I2S_CHN_6 { 6 } else if val == I2S_CHN_8 { 8 } else { 2 }
}

unsafe fn i2s_tdm_disable_unprepare_mclk(i2s_tdm: *mut rk_i2s_tdm_dev) {
    clk_disable_unprepare((*i2s_tdm).mclk_tx);
    clk_disable_unprepare((*i2s_tdm).mclk_rx);
}

/**
 * i2s_tdm_prepare_enable_mclk - prepare to enable all mclks, disable them on
 *				 failure.
 * @i2s_tdm: rk_i2s_tdm_dev struct
 *
 * This function attempts to enable all mclk clocks, but cleans up after
 * itself on failure. Guarantees to balance its calls.
 *
 * Returns success (0) or negative errno.
 */
unsafe fn i2s_tdm_prepare_enable_mclk(i2s_tdm: *mut rk_i2s_tdm_dev) -> c_int {
    let mut ret: c_int = clk_prepare_enable((*i2s_tdm).mclk_tx);
    if ret != 0 {
        return ret;
    }
    ret = clk_prepare_enable((*i2s_tdm).mclk_rx);
    if ret != 0 {
        clk_disable_unprepare((*i2s_tdm).mclk_tx);
        return ret;
    }
    0
}

unsafe extern "C" fn i2s_tdm_runtime_suspend(dev: *mut device) -> c_int {
    let i2s_tdm = dev_get_drvdata(dev) as *mut rk_i2s_tdm_dev;
    regcache_cache_only((*i2s_tdm).regmap, true);
    i2s_tdm_disable_unprepare_mclk(i2s_tdm);
    clk_disable_unprepare((*i2s_tdm).hclk);
    0
}

unsafe extern "C" fn i2s_tdm_runtime_resume(dev: *mut device) -> c_int {
    let i2s_tdm = dev_get_drvdata(dev) as *mut rk_i2s_tdm_dev;
    let mut ret = clk_prepare_enable((*i2s_tdm).hclk);
    if ret != 0 {
        return ret;
    }
    ret = i2s_tdm_prepare_enable_mclk(i2s_tdm);
    if ret != 0 {
        clk_disable_unprepare((*i2s_tdm).hclk);
        return ret;
    }
    regcache_cache_only((*i2s_tdm).regmap, false);
    regcache_mark_dirty((*i2s_tdm).regmap);
    ret = regcache_sync((*i2s_tdm).regmap);
    if ret != 0 {
        i2s_tdm_disable_unprepare_mclk(i2s_tdm);
        clk_disable_unprepare((*i2s_tdm).hclk);
        return ret;
    }
    0
}

#[inline]
unsafe fn to_info(dai: *mut snd_soc_dai) -> *mut rk_i2s_tdm_dev {
    snd_soc_dai_get_drvdata(dai) as *mut rk_i2s_tdm_dev
}

/*
 * Makes sure that both tx and rx are reset at the same time to sync lrck
 * when clk_trcm > 0.
 */
unsafe fn rockchip_snd_xfer_sync_reset(i2s_tdm: *mut rk_i2s_tdm_dev) {
    /* This is technically race-y.
     *
     * In an ideal world, we could atomically assert both resets at the
     * same time, through an atomic bulk reset API. This API however does
     * not exist, so what the downstream vendor code used to do was
     * implement half a reset controller here and require the CRU to be
     * passed to the driver as a device tree node. Violating abstractions
     * like that is bad, especially when it influences something like the
     * bindings which are supposed to describe the hardware, not whatever
     * workarounds the driver needs, so it was dropped.
     *
     * In practice, asserting the resets one by one appears to work just
     * fine for playback. During duplex (playback + capture) operation,
     * this might become an issue, but that should be solved by the
     * implementation of the aforementioned API, not by shoving a reset
     * controller into an audio driver.
     */
    reset_control_assert((*i2s_tdm).tx_reset);
    reset_control_assert((*i2s_tdm).rx_reset);
    udelay(10);
    reset_control_deassert((*i2s_tdm).tx_reset);
    reset_control_deassert((*i2s_tdm).rx_reset);
    udelay(10);
}

unsafe fn rockchip_snd_reset(rc: *mut reset_control) {
    reset_control_assert(rc);
    udelay(10);
    reset_control_deassert(rc);
    udelay(10);
}

unsafe fn rockchip_snd_xfer_clear(i2s_tdm: *mut rk_i2s_tdm_dev, clr: c_uint) {
    let mut xfer_mask: c_uint = 0;
    let mut xfer_val: c_uint = 0;
    let mut val: c_uint = 0;
    let mut retry: c_int = 10;
    let tx = (clr & I2S_CLR_TXC) != 0;
    let rx = (clr & I2S_CLR_RXC) != 0;

    if !(rx || tx) {
        return;
    }
    if tx {
        xfer_mask = I2S_XFER_TXS_START;
        xfer_val = I2S_XFER_TXS_STOP;
    }
    if rx {
        xfer_mask |= I2S_XFER_RXS_START;
        xfer_val |= I2S_XFER_RXS_STOP;
    }
    regmap_update_bits((*i2s_tdm).regmap, I2S_XFER, xfer_mask, xfer_val);
    udelay(150);
    regmap_update_bits((*i2s_tdm).regmap, I2S_CLR, clr, clr);
    regmap_read((*i2s_tdm).regmap, I2S_CLR, &mut val);
    /* Wait on the clear operation to finish */
    while val != 0 {
        udelay(15);
        regmap_read((*i2s_tdm).regmap, I2S_CLR, &mut val);
        retry -= 1;
        if retry == 0 {
            dev_warn((*i2s_tdm).dev, b"clear failed, reset %s%s\n\0".as_ptr() as *const c_char,
                     if tx { b"tx\0".as_ptr() } else { b"\0".as_ptr() },
                     if rx { b"rx\0".as_ptr() } else { b"\0".as_ptr() });
            if rx && tx {
                rockchip_snd_xfer_sync_reset(i2s_tdm);
            } else if tx {
                rockchip_snd_reset((*i2s_tdm).tx_reset);
            } else if rx {
                rockchip_snd_reset((*i2s_tdm).rx_reset);
            }
            break;
        }
    }
}

#[inline] unsafe fn rockchip_enable_tde(map: *mut regmap) { regmap_update_bits(map, I2S_DMACR, I2S_DMACR_TDE_ENABLE, I2S_DMACR_TDE_ENABLE); }
#[inline] unsafe fn rockchip_disable_tde(map: *mut regmap) { regmap_update_bits(map, I2S_DMACR, I2S_DMACR_TDE_ENABLE, I2S_DMACR_TDE_DISABLE); }
#[inline] unsafe fn rockchip_enable_rde(map: *mut regmap) { regmap_update_bits(map, I2S_DMACR, I2S_DMACR_RDE_ENABLE, I2S_DMACR_RDE_ENABLE); }
#[inline] unsafe fn rockchip_disable_rde(map: *mut regmap) { regmap_update_bits(map, I2S_DMACR, I2S_DMACR_RDE_ENABLE, I2S_DMACR_RDE_DISABLE); }

/* only used when clk_trcm > 0 */
unsafe fn rockchip_snd_txrxctrl(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai, on: c_int) {
    let i2s_tdm = to_info(dai);
    // C used guard(spinlock_irqsave)(&i2s_tdm->lock).
    if on != 0 {
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK { rockchip_enable_tde((*i2s_tdm).regmap); } else { rockchip_enable_rde((*i2s_tdm).regmap); }
        (*i2s_tdm).refcount += 1;
        if (*i2s_tdm).refcount == 1 {
            rockchip_snd_xfer_sync_reset(i2s_tdm);
            regmap_update_bits((*i2s_tdm).regmap, I2S_XFER, I2S_XFER_TXS_START | I2S_XFER_RXS_START, I2S_XFER_TXS_START | I2S_XFER_RXS_START);
        }
    } else {
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK { rockchip_disable_tde((*i2s_tdm).regmap); } else { rockchip_disable_rde((*i2s_tdm).regmap); }
        (*i2s_tdm).refcount -= 1;
        if (*i2s_tdm).refcount == 0 {
            rockchip_snd_xfer_clear(i2s_tdm, I2S_CLR_TXC | I2S_CLR_RXC);
        }
    }
}

unsafe fn rockchip_snd_txctrl(i2s_tdm: *mut rk_i2s_tdm_dev, on: c_int) {
    if on != 0 {
        rockchip_enable_tde((*i2s_tdm).regmap);
        regmap_update_bits((*i2s_tdm).regmap, I2S_XFER, I2S_XFER_TXS_START, I2S_XFER_TXS_START);
    } else {
        rockchip_disable_tde((*i2s_tdm).regmap);
        rockchip_snd_xfer_clear(i2s_tdm, I2S_CLR_TXC);
    }
}

unsafe fn rockchip_snd_rxctrl(i2s_tdm: *mut rk_i2s_tdm_dev, on: c_int) {
    if on != 0 {
        rockchip_enable_rde((*i2s_tdm).regmap);
        regmap_update_bits((*i2s_tdm).regmap, I2S_XFER, I2S_XFER_RXS_START, I2S_XFER_RXS_START);
    } else {
        rockchip_disable_rde((*i2s_tdm).regmap);
        rockchip_snd_xfer_clear(i2s_tdm, I2S_CLR_RXC);
    }
}

unsafe extern "C" fn rockchip_i2s_tdm_set_fmt(cpu_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let i2s_tdm = to_info(cpu_dai);
    let mut mask: c_uint;
    let mut val: c_uint = 0;
    let mut tdm_val: c_uint = 0;
    let txcr_val: c_uint;
    let rxcr_val: c_uint;
    let mut ret: c_int;
    let is_tdm = (*i2s_tdm).tdm_mode;

    ret = pm_runtime_resume_and_get((*cpu_dai).dev);
    if ret < 0 && ret != -EACCES { return ret; }

    mask = I2S_CKR_MSS_MASK;
    if (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) == SND_SOC_DAIFMT_BP_FP {
        val = I2S_CKR_MSS_MASTER; (*i2s_tdm).is_master_mode = true;
    } else if (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) == SND_SOC_DAIFMT_BC_FC {
        val = I2S_CKR_MSS_SLAVE; (*i2s_tdm).is_master_mode = false;
    } else { ret = -EINVAL; pm_runtime_put((*cpu_dai).dev); return ret; }
    regmap_update_bits((*i2s_tdm).regmap, I2S_CKR, mask, val);

    mask = I2S_CKR_CKP_MASK | I2S_CKR_TLP_MASK | I2S_CKR_RLP_MASK;
    let inv = fmt & SND_SOC_DAIFMT_INV_MASK;
    if inv == SND_SOC_DAIFMT_NB_NF {
        val = I2S_CKR_CKP_NORMAL | I2S_CKR_TLP_NORMAL | I2S_CKR_RLP_NORMAL;
    } else if inv == SND_SOC_DAIFMT_NB_IF {
        val = I2S_CKR_CKP_NORMAL | I2S_CKR_TLP_INVERTED | I2S_CKR_RLP_INVERTED;
    } else if inv == SND_SOC_DAIFMT_IB_NF {
        val = I2S_CKR_CKP_INVERTED | I2S_CKR_TLP_NORMAL | I2S_CKR_RLP_NORMAL;
    } else if inv == SND_SOC_DAIFMT_IB_IF {
        val = I2S_CKR_CKP_INVERTED | I2S_CKR_TLP_INVERTED | I2S_CKR_RLP_INVERTED;
    } else { ret = -EINVAL; pm_runtime_put((*cpu_dai).dev); return ret; }
    regmap_update_bits((*i2s_tdm).regmap, I2S_CKR, mask, val);

    let format = fmt & SND_SOC_DAIFMT_FORMAT_MASK;
    if format == SND_SOC_DAIFMT_RIGHT_J {
        txcr_val = I2S_TXCR_IBM_RSJM; rxcr_val = I2S_RXCR_IBM_RSJM;
    } else if format == SND_SOC_DAIFMT_LEFT_J {
        txcr_val = I2S_TXCR_IBM_LSJM; rxcr_val = I2S_RXCR_IBM_LSJM;
    } else if format == SND_SOC_DAIFMT_I2S {
        txcr_val = I2S_TXCR_IBM_NORMAL; rxcr_val = I2S_RXCR_IBM_NORMAL;
    } else if format == SND_SOC_DAIFMT_DSP_A { /* PCM delay 1 mode */
        txcr_val = I2S_TXCR_TFS_PCM | I2S_TXCR_PBM_MODE(1);
        rxcr_val = I2S_RXCR_TFS_PCM | I2S_RXCR_PBM_MODE(1);
    } else if format == SND_SOC_DAIFMT_DSP_B { /* PCM no delay mode */
        txcr_val = I2S_TXCR_TFS_PCM; rxcr_val = I2S_RXCR_TFS_PCM;
    } else { ret = -EINVAL; pm_runtime_put((*cpu_dai).dev); return ret; }

    mask = I2S_TXCR_IBM_MASK | I2S_TXCR_TFS_MASK | I2S_TXCR_PBM_MASK;
    regmap_update_bits((*i2s_tdm).regmap, I2S_TXCR, mask, txcr_val);
    mask = I2S_RXCR_IBM_MASK | I2S_RXCR_TFS_MASK | I2S_RXCR_PBM_MASK;
    regmap_update_bits((*i2s_tdm).regmap, I2S_RXCR, mask, rxcr_val);

    if is_tdm {
        if format == SND_SOC_DAIFMT_RIGHT_J { val = I2S_TXCR_TFS_TDM_I2S; tdm_val = TDM_SHIFT_CTRL(2); }
        else if format == SND_SOC_DAIFMT_LEFT_J { val = I2S_TXCR_TFS_TDM_I2S; tdm_val = TDM_SHIFT_CTRL(1); }
        else if format == SND_SOC_DAIFMT_I2S { val = I2S_TXCR_TFS_TDM_I2S; tdm_val = TDM_SHIFT_CTRL(0); }
        else if format == SND_SOC_DAIFMT_DSP_A { val = I2S_TXCR_TFS_TDM_PCM; tdm_val = TDM_SHIFT_CTRL(2); }
        else if format == SND_SOC_DAIFMT_DSP_B { val = I2S_TXCR_TFS_TDM_PCM; tdm_val = TDM_SHIFT_CTRL(4); }
        else { ret = -EINVAL; pm_runtime_put((*cpu_dai).dev); return ret; }
        tdm_val |= TDM_FSYNC_WIDTH_SEL1(1);
        tdm_val |= TDM_FSYNC_WIDTH_HALF_FRAME;
        mask = I2S_TXCR_TFS_MASK;
        regmap_update_bits((*i2s_tdm).regmap, I2S_TXCR, mask, val);
        regmap_update_bits((*i2s_tdm).regmap, I2S_RXCR, mask, val);
        mask = TDM_FSYNC_WIDTH_SEL1_MSK | TDM_FSYNC_WIDTH_SEL0_MSK | TDM_SHIFT_CTRL_MSK;
        regmap_update_bits((*i2s_tdm).regmap, I2S_TDM_TXCR, mask, tdm_val);
        regmap_update_bits((*i2s_tdm).regmap, I2S_TDM_RXCR, mask, tdm_val);
    }
    pm_runtime_put((*cpu_dai).dev);
    ret
}

unsafe fn rockchip_i2s_tdm_xfer_pause(substream: *mut snd_pcm_substream, i2s_tdm: *mut rk_i2s_tdm_dev) {
    let stream = SNDRV_PCM_STREAM_LAST - (*substream).stream;
    if stream == SNDRV_PCM_STREAM_PLAYBACK { rockchip_disable_tde((*i2s_tdm).regmap); } else { rockchip_disable_rde((*i2s_tdm).regmap); }
    rockchip_snd_xfer_clear(i2s_tdm, I2S_CLR_TXC | I2S_CLR_RXC);
}

unsafe fn rockchip_i2s_tdm_xfer_resume(substream: *mut snd_pcm_substream, i2s_tdm: *mut rk_i2s_tdm_dev) {
    let stream = SNDRV_PCM_STREAM_LAST - (*substream).stream;
    if stream == SNDRV_PCM_STREAM_PLAYBACK { rockchip_enable_tde((*i2s_tdm).regmap); } else { rockchip_enable_rde((*i2s_tdm).regmap); }
    regmap_update_bits((*i2s_tdm).regmap, I2S_XFER, I2S_XFER_TXS_START | I2S_XFER_RXS_START, I2S_XFER_TXS_START | I2S_XFER_RXS_START);
}

unsafe fn rockchip_i2s_io_multiplex(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let i2s_tdm = to_info(dai);
    let mut usable_chs: c_int = MULTIPLEX_CH_MAX;
    let mut val: c_uint = 0;
    if !(*i2s_tdm).io_multiplex { return 0; }
    if IS_ERR_OR_NULL((*i2s_tdm).grf as *const c_void) {
        dev_err((*i2s_tdm).dev, b"io multiplex not supported for this device\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        let playback_str = &mut (*(*substream).pcm).streams[SNDRV_PCM_STREAM_PLAYBACK as usize] as *mut snd_pcm_str;
        if (*playback_str).substream_opened != 0 {
            regmap_read((*i2s_tdm).regmap, I2S_TXCR, &mut val);
            val &= I2S_TXCR_CSR_MASK;
            usable_chs = MULTIPLEX_CH_MAX - to_ch_num(val);
        }
        regmap_read((*i2s_tdm).regmap, I2S_RXCR, &mut val);
        val &= I2S_RXCR_CSR_MASK;
        if to_ch_num(val) > usable_chs {
            dev_err((*i2s_tdm).dev, b"Capture channels (%d) > usable channels (%d)\n\0".as_ptr() as *const c_char, to_ch_num(val), usable_chs);
            return -EINVAL;
        }
    } else {
        let capture_str = &mut (*(*substream).pcm).streams[SNDRV_PCM_STREAM_CAPTURE as usize] as *mut snd_pcm_str;
        if (*capture_str).substream_opened != 0 {
            regmap_read((*i2s_tdm).regmap, I2S_RXCR, &mut val);
            val &= I2S_RXCR_CSR_MASK;
            usable_chs = MULTIPLEX_CH_MAX - to_ch_num(val);
        }
        regmap_read((*i2s_tdm).regmap, I2S_TXCR, &mut val);
        val &= I2S_TXCR_CSR_MASK;
        if to_ch_num(val) > usable_chs {
            dev_err((*i2s_tdm).dev, b"Playback channels (%d) > usable channels (%d)\n\0".as_ptr() as *const c_char, to_ch_num(val), usable_chs);
            return -EINVAL;
        }
    }
    val <<= (*(*i2s_tdm).soc_data).grf_shift;
    val |= (I2S_IO_DIRECTION_MASK << (*(*i2s_tdm).soc_data).grf_shift) << 16;
    regmap_write((*i2s_tdm).grf, (*(*i2s_tdm).soc_data).grf_reg_offset, val);
    0
}

unsafe fn rockchip_i2s_trcm_mode(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai, div_bclk: c_uint, div_lrck: c_uint, fmt: c_uint) -> c_int {
    let i2s_tdm = to_info(dai);
    if (*i2s_tdm).clk_trcm == 0 { return 0; }
    // C used guard(spinlock_irqsave)(&i2s_tdm->lock).
    if (*i2s_tdm).refcount != 0 { rockchip_i2s_tdm_xfer_pause(substream, i2s_tdm); }
    regmap_update_bits((*i2s_tdm).regmap, I2S_CLKDIV, I2S_CLKDIV_TXM_MASK | I2S_CLKDIV_RXM_MASK, I2S_CLKDIV_TXM(div_bclk) | I2S_CLKDIV_RXM(div_bclk));
    regmap_update_bits((*i2s_tdm).regmap, I2S_CKR, I2S_CKR_TSD_MASK | I2S_CKR_RSD_MASK, I2S_CKR_TSD(div_lrck) | I2S_CKR_RSD(div_lrck));
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        regmap_update_bits((*i2s_tdm).regmap, I2S_TXCR, I2S_TXCR_VDW_MASK | I2S_TXCR_CSR_MASK, fmt);
    } else {
        regmap_update_bits((*i2s_tdm).regmap, I2S_RXCR, I2S_RXCR_VDW_MASK | I2S_RXCR_CSR_MASK, fmt);
    }
    if (*i2s_tdm).refcount != 0 { rockchip_i2s_tdm_xfer_resume(substream, i2s_tdm); }
    0
}

unsafe extern "C" fn rockchip_i2s_tdm_set_sysclk(cpu_dai: *mut snd_soc_dai, stream: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let i2s_tdm = to_info(cpu_dai);
    if (*i2s_tdm).clk_trcm != 0 {
        (*i2s_tdm).mclk_tx_freq = freq; (*i2s_tdm).mclk_rx_freq = freq;
    } else if stream == SNDRV_PCM_STREAM_PLAYBACK {
        (*i2s_tdm).mclk_tx_freq = freq;
    } else {
        (*i2s_tdm).mclk_rx_freq = freq;
    }
    dev_dbg((*i2s_tdm).dev, b"The target mclk_%s freq is: %d\n\0".as_ptr() as *const c_char,
            if stream != 0 { b"rx\0".as_ptr() } else { b"tx\0".as_ptr() }, freq);
    0
}

unsafe extern "C" fn rockchip_i2s_tdm_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let i2s_tdm = to_info(dai);
    let mut val: c_uint = 0;
    let mut mclk_rate: c_uint = 0;
    let mut bclk_rate: c_uint;
    let mut div_bclk: c_uint = 4;
    let mut div_lrck: c_uint = 64;
    if (*i2s_tdm).is_master_mode {
        let mclk: *mut clk;
        if (*i2s_tdm).clk_trcm == TRCM_TX { mclk = (*i2s_tdm).mclk_tx; mclk_rate = (*i2s_tdm).mclk_tx_freq; }
        else if (*i2s_tdm).clk_trcm == TRCM_RX { mclk = (*i2s_tdm).mclk_rx; mclk_rate = (*i2s_tdm).mclk_rx_freq; }
        else if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK { mclk = (*i2s_tdm).mclk_tx; mclk_rate = (*i2s_tdm).mclk_tx_freq; }
        else { mclk = (*i2s_tdm).mclk_rx; mclk_rate = (*i2s_tdm).mclk_rx_freq; }
        /*
         * When the dai/component driver doesn't need to set mclk-fs for a specific
         * clock, it can skip the call to set_sysclk() for that clock.
         * In that case, simply use the clock rate from the params and multiply it by
         * the default mclk-fs value.
         */
        if mclk_rate == 0 { mclk_rate = DEFAULT_MCLK_FS * params_rate(params); }
        let err = clk_set_rate(mclk, mclk_rate);
        if err != 0 { return err; }
        mclk_rate = clk_get_rate(mclk);
        bclk_rate = (*i2s_tdm).frame_width * params_rate(params);
        if bclk_rate == 0 { return -EINVAL; }
        div_bclk = div_round_closest(mclk_rate, bclk_rate);
        div_lrck = bclk_rate / params_rate(params);
    }
    let format = params_format(params);
    if format == SNDRV_PCM_FORMAT_S8 { val |= I2S_TXCR_VDW(8); }
    else if format == SNDRV_PCM_FORMAT_S16_LE { val |= I2S_TXCR_VDW(16); }
    else if format == SNDRV_PCM_FORMAT_S20_3LE { val |= I2S_TXCR_VDW(20); }
    else if format == SNDRV_PCM_FORMAT_S24_LE { val |= I2S_TXCR_VDW(24); }
    else if format == SNDRV_PCM_FORMAT_S32_LE { val |= I2S_TXCR_VDW(32); }
    else { return -EINVAL; }
    match params_channels(params) {
        8 => val |= I2S_CHN_8,
        6 => val |= I2S_CHN_6,
        4 => val |= I2S_CHN_4,
        2 => val |= I2S_CHN_2,
        _ => return -EINVAL,
    }
    if (*i2s_tdm).clk_trcm != 0 {
        rockchip_i2s_trcm_mode(substream, dai, div_bclk, div_lrck, val);
    } else if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        regmap_update_bits((*i2s_tdm).regmap, I2S_CLKDIV, I2S_CLKDIV_TXM_MASK, I2S_CLKDIV_TXM(div_bclk));
        regmap_update_bits((*i2s_tdm).regmap, I2S_CKR, I2S_CKR_TSD_MASK, I2S_CKR_TSD(div_lrck));
        regmap_update_bits((*i2s_tdm).regmap, I2S_TXCR, I2S_TXCR_VDW_MASK | I2S_TXCR_CSR_MASK, val);
    } else {
        regmap_update_bits((*i2s_tdm).regmap, I2S_CLKDIV, I2S_CLKDIV_RXM_MASK, I2S_CLKDIV_RXM(div_bclk));
        regmap_update_bits((*i2s_tdm).regmap, I2S_CKR, I2S_CKR_RSD_MASK, I2S_CKR_RSD(div_lrck));
        regmap_update_bits((*i2s_tdm).regmap, I2S_RXCR, I2S_RXCR_VDW_MASK | I2S_RXCR_CSR_MASK, val);
    }
    rockchip_i2s_io_multiplex(substream, dai)
}

unsafe extern "C" fn rockchip_i2s_tdm_trigger(substream: *mut snd_pcm_substream, cmd: c_int, dai: *mut snd_soc_dai) -> c_int {
    let i2s_tdm = to_info(dai);
    if cmd == SNDRV_PCM_TRIGGER_START || cmd == SNDRV_PCM_TRIGGER_RESUME || cmd == SNDRV_PCM_TRIGGER_PAUSE_RELEASE {
        if (*i2s_tdm).clk_trcm != 0 { rockchip_snd_txrxctrl(substream, dai, 1); }
        else if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE { rockchip_snd_rxctrl(i2s_tdm, 1); }
        else { rockchip_snd_txctrl(i2s_tdm, 1); }
    } else if cmd == SNDRV_PCM_TRIGGER_SUSPEND || cmd == SNDRV_PCM_TRIGGER_STOP || cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH {
        if (*i2s_tdm).clk_trcm != 0 { rockchip_snd_txrxctrl(substream, dai, 0); }
        else if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE { rockchip_snd_rxctrl(i2s_tdm, 0); }
        else { rockchip_snd_txctrl(i2s_tdm, 0); }
    } else {
        return -EINVAL;
    }
    0
}

unsafe extern "C" fn rockchip_i2s_tdm_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let i2s_tdm = snd_soc_dai_get_drvdata(dai) as *mut rk_i2s_tdm_dev;
    if (*i2s_tdm).has_capture { snd_soc_dai_dma_data_set_capture(dai, &mut (*i2s_tdm).capture_dma_data); }
    if (*i2s_tdm).has_playback { snd_soc_dai_dma_data_set_playback(dai, &mut (*i2s_tdm).playback_dma_data); }
    0
}

unsafe extern "C" fn rockchip_dai_tdm_slot(dai: *mut snd_soc_dai, _tx_mask: c_uint, _rx_mask: c_uint, slots: c_int, slot_width: c_int) -> c_int {
    let i2s_tdm = snd_soc_dai_get_drvdata(dai) as *mut rk_i2s_tdm_dev;
    (*i2s_tdm).tdm_mode = true;
    (*i2s_tdm).frame_width = (slots * slot_width) as c_uint;
    let mask = TDM_SLOT_BIT_WIDTH_MSK | TDM_FRAME_WIDTH_MSK;
    let val = TDM_SLOT_BIT_WIDTH(slot_width) | TDM_FRAME_WIDTH(slots * slot_width);
    regmap_update_bits((*i2s_tdm).regmap, I2S_TDM_TXCR, mask, val);
    regmap_update_bits((*i2s_tdm).regmap, I2S_TDM_RXCR, mask, val);
    0
}

unsafe extern "C" fn rockchip_i2s_tdm_set_bclk_ratio(dai: *mut snd_soc_dai, ratio: c_uint) -> c_int {
    let i2s_tdm = snd_soc_dai_get_drvdata(dai) as *mut rk_i2s_tdm_dev;
    if ratio < 32 || ratio > 512 || ratio % 2 == 1 { return -EINVAL; }
    (*i2s_tdm).frame_width = ratio;
    0
}

static rockchip_i2s_tdm_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(rockchip_i2s_tdm_dai_probe),
    hw_params: Some(rockchip_i2s_tdm_hw_params),
    set_bclk_ratio: Some(rockchip_i2s_tdm_set_bclk_ratio),
    set_fmt: Some(rockchip_i2s_tdm_set_fmt),
    set_sysclk: Some(rockchip_i2s_tdm_set_sysclk),
    set_tdm_slot: Some(rockchip_dai_tdm_slot),
    trigger: Some(rockchip_i2s_tdm_trigger),
};

static rockchip_i2s_tdm_component: snd_soc_component_driver = snd_soc_component_driver {
    name: DRV_NAME.as_ptr() as *const c_char,
    legacy_dai_naming: 1,
};

unsafe extern "C" fn rockchip_i2s_tdm_wr_reg(_dev: *mut device, reg: c_uint) -> bool {
    reg == I2S_TXCR || reg == I2S_RXCR || reg == I2S_CKR || reg == I2S_DMACR || reg == I2S_INTCR ||
    reg == I2S_XFER || reg == I2S_CLR || reg == I2S_TXDR || reg == I2S_TDM_TXCR || reg == I2S_TDM_RXCR || reg == I2S_CLKDIV
}

unsafe extern "C" fn rockchip_i2s_tdm_rd_reg(_dev: *mut device, reg: c_uint) -> bool {
    reg == I2S_TXCR || reg == I2S_RXCR || reg == I2S_CKR || reg == I2S_DMACR || reg == I2S_INTCR ||
    reg == I2S_XFER || reg == I2S_CLR || reg == I2S_TXDR || reg == I2S_RXDR || reg == I2S_TXFIFOLR ||
    reg == I2S_INTSR || reg == I2S_RXFIFOLR || reg == I2S_TDM_TXCR || reg == I2S_TDM_RXCR || reg == I2S_CLKDIV
}

unsafe extern "C" fn rockchip_i2s_tdm_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    reg == I2S_TXFIFOLR || reg == I2S_INTSR || reg == I2S_CLR || reg == I2S_TXDR || reg == I2S_RXDR || reg == I2S_RXFIFOLR
}

unsafe extern "C" fn rockchip_i2s_tdm_precious_reg(_dev: *mut device, reg: c_uint) -> bool {
    reg == I2S_RXDR
}

static rockchip_i2s_tdm_reg_defaults: [reg_default; 8] = [
    reg_default { reg: 0x00, def: 0x7200000f },
    reg_default { reg: 0x04, def: 0x01c8000f },
    reg_default { reg: 0x08, def: 0x00001f1f },
    reg_default { reg: 0x10, def: 0x001f0000 },
    reg_default { reg: 0x14, def: 0x01f00000 },
    reg_default { reg: 0x30, def: 0x00003eff },
    reg_default { reg: 0x34, def: 0x00003eff },
    reg_default { reg: 0x38, def: 0x00000707 },
];

static rockchip_i2s_tdm_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: unsafe { I2S_CLKDIV },
    reg_defaults: rockchip_i2s_tdm_reg_defaults.as_ptr(),
    num_reg_defaults: rockchip_i2s_tdm_reg_defaults.len() as c_uint,
    writeable_reg: Some(rockchip_i2s_tdm_wr_reg),
    readable_reg: Some(rockchip_i2s_tdm_rd_reg),
    volatile_reg: Some(rockchip_i2s_tdm_volatile_reg),
    precious_reg: Some(rockchip_i2s_tdm_precious_reg),
    cache_type: unsafe { REGCACHE_FLAT },
};

unsafe extern "C" fn common_soc_init(dev: *mut device, addr: u32) -> c_int {
    let i2s_tdm = dev_get_drvdata(dev) as *mut rk_i2s_tdm_dev;
    let configs = (*(*i2s_tdm).soc_data).configs;
    let mut trcm = (*i2s_tdm).clk_trcm;
    if trcm == TRCM_TXRX { return 0; }
    if IS_ERR_OR_NULL((*i2s_tdm).grf as *const c_void) {
        dev_err((*i2s_tdm).dev, b"no grf present but non-txrx TRCM specified\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    let mut i: c_int = 0;
    while i < (*(*i2s_tdm).soc_data).config_count {
        let cfg = configs.add(i as usize);
        if addr == (*cfg).addr {
            let reg = (*cfg).reg;
            let val = if trcm == TRCM_TX { (*cfg).txonly } else { (*cfg).rxonly };
            if reg != 0 { regmap_write((*i2s_tdm).grf, reg, val); }
        }
        i += 1;
    }
    0
}

static px30_txrx_config: [txrx_config; 1] = [txrx_config { addr: 0xff060000, reg: 0x184, txonly: unsafe { PX30_I2S0_CLK_TXONLY }, rxonly: unsafe { PX30_I2S0_CLK_RXONLY } }];
static rk1808_txrx_config: [txrx_config; 1] = [txrx_config { addr: 0xff7e0000, reg: 0x190, txonly: unsafe { RK1808_I2S0_CLK_TXONLY }, rxonly: unsafe { RK1808_I2S0_CLK_RXONLY } }];
static rk3308_txrx_config: [txrx_config; 2] = [
    txrx_config { addr: 0xff300000, reg: 0x308, txonly: unsafe { RK3308_I2S0_CLK_TXONLY }, rxonly: unsafe { RK3308_I2S0_CLK_RXONLY } },
    txrx_config { addr: 0xff310000, reg: 0x308, txonly: unsafe { RK3308_I2S1_CLK_TXONLY }, rxonly: unsafe { RK3308_I2S1_CLK_RXONLY } },
];
static rk3568_txrx_config: [txrx_config; 6] = [
    txrx_config { addr: 0xfe410000, reg: 0x504, txonly: unsafe { RK3568_I2S1_CLK_TXONLY }, rxonly: unsafe { RK3568_I2S1_CLK_RXONLY } },
    txrx_config { addr: 0xfe410000, reg: 0x508, txonly: unsafe { RK3568_I2S1_MCLK_TX_OE }, rxonly: unsafe { RK3568_I2S1_MCLK_RX_OE } },
    txrx_config { addr: 0xfe420000, reg: 0x508, txonly: unsafe { RK3568_I2S2_MCLK_OE }, rxonly: unsafe { RK3568_I2S2_MCLK_OE } },
    txrx_config { addr: 0xfe430000, reg: 0x504, txonly: unsafe { RK3568_I2S3_CLK_TXONLY }, rxonly: unsafe { RK3568_I2S3_CLK_RXONLY } },
    txrx_config { addr: 0xfe430000, reg: 0x508, txonly: unsafe { RK3568_I2S3_MCLK_TXONLY }, rxonly: unsafe { RK3568_I2S3_MCLK_RXONLY } },
    txrx_config { addr: 0xfe430000, reg: 0x508, txonly: unsafe { RK3568_I2S3_MCLK_OE }, rxonly: unsafe { RK3568_I2S3_MCLK_OE } },
];
static rv1126_txrx_config: [txrx_config; 1] = [txrx_config { addr: 0xff800000, reg: 0x10260, txonly: unsafe { RV1126_I2S0_CLK_TXONLY }, rxonly: unsafe { RV1126_I2S0_CLK_RXONLY } }];

static px30_i2s_soc_data: rk_i2s_soc_data = rk_i2s_soc_data { softrst_offset: 0x0300, grf_reg_offset: 0, grf_shift: 0, config_count: 1, configs: px30_txrx_config.as_ptr(), init: Some(common_soc_init) };
static rk1808_i2s_soc_data: rk_i2s_soc_data = rk_i2s_soc_data { softrst_offset: 0x0300, grf_reg_offset: 0, grf_shift: 0, config_count: 1, configs: rk1808_txrx_config.as_ptr(), init: Some(common_soc_init) };
static rk3308_i2s_soc_data: rk_i2s_soc_data = rk_i2s_soc_data { softrst_offset: 0x0400, grf_reg_offset: 0x0308, grf_shift: 5, config_count: 2, configs: rk3308_txrx_config.as_ptr(), init: Some(common_soc_init) };
static rk3568_i2s_soc_data: rk_i2s_soc_data = rk_i2s_soc_data { softrst_offset: 0x0400, grf_reg_offset: 0, grf_shift: 0, config_count: 6, configs: rk3568_txrx_config.as_ptr(), init: Some(common_soc_init) };
static rv1126_i2s_soc_data: rk_i2s_soc_data = rk_i2s_soc_data { softrst_offset: 0x0300, grf_reg_offset: 0, grf_shift: 0, config_count: 1, configs: rv1126_txrx_config.as_ptr(), init: Some(common_soc_init) };

static rockchip_i2s_tdm_match: [of_device_id; 7] = [
    of_device_id { compatible: b"rockchip,px30-i2s-tdm\0".as_ptr() as *const c_char, data: &px30_i2s_soc_data as *const _ as *const c_void },
    of_device_id { compatible: b"rockchip,rk1808-i2s-tdm\0".as_ptr() as *const c_char, data: &rk1808_i2s_soc_data as *const _ as *const c_void },
    of_device_id { compatible: b"rockchip,rk3308-i2s-tdm\0".as_ptr() as *const c_char, data: &rk3308_i2s_soc_data as *const _ as *const c_void },
    of_device_id { compatible: b"rockchip,rk3568-i2s-tdm\0".as_ptr() as *const c_char, data: &rk3568_i2s_soc_data as *const _ as *const c_void },
    of_device_id { compatible: b"rockchip,rk3588-i2s-tdm\0".as_ptr() as *const c_char, data: null() },
    of_device_id { compatible: b"rockchip,rv1126-i2s-tdm\0".as_ptr() as *const c_char, data: &rv1126_i2s_soc_data as *const _ as *const c_void },
    of_device_id { compatible: null(), data: null() },
];
// MODULE_DEVICE_TABLE(of, rockchip_i2s_tdm_match);

static i2s_tdm_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    probe: None,
    playback: snd_soc_pcm_stream { stream_name: null(), channels_min: 0, channels_max: 0, rates: 0, formats: 0 },
    capture: snd_soc_pcm_stream { stream_name: null(), channels_min: 0, channels_max: 0, rates: 0, formats: 0 },
    ops: &rockchip_i2s_tdm_dai_ops,
    symmetric_rate: 0,
};

unsafe fn rockchip_i2s_tdm_init_dai(i2s_tdm: *mut rk_i2s_tdm_dev) -> c_int {
    let formats: u64 = SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;
    let _node = (*(*i2s_tdm).dev).of_node;
    // of_property_for_each_string(node, "dma-names", dma_names, dma_name) translated as an external iterator dependency.
    // The loop body preserved:
    // if (!strcmp(dma_name, "tx")) i2s_tdm->has_playback = true;
    // if (!strcmp(dma_name, "rx")) i2s_tdm->has_capture = true;
    let dai = devm_kmemdup((*i2s_tdm).dev, &i2s_tdm_dai as *const _ as *const c_void, size_of::<snd_soc_dai_driver>(), GFP_KERNEL) as *mut snd_soc_dai_driver;
    if dai.is_null() { return -ENOMEM; }
    if (*i2s_tdm).has_playback {
        (*dai).playback.stream_name = b"Playback\0".as_ptr() as *const c_char;
        (*dai).playback.channels_min = 2;
        (*dai).playback.channels_max = 8;
        (*dai).playback.rates = SNDRV_PCM_RATE_8000_192000;
        (*dai).playback.formats = formats;
    }
    if (*i2s_tdm).has_capture {
        (*dai).capture.stream_name = b"Capture\0".as_ptr() as *const c_char;
        (*dai).capture.channels_min = 2;
        (*dai).capture.channels_max = 8;
        (*dai).capture.rates = SNDRV_PCM_RATE_8000_192000;
        (*dai).capture.formats = formats;
    }
    if (*i2s_tdm).clk_trcm != TRCM_TXRX { (*dai).symmetric_rate = 1; }
    (*i2s_tdm).dai = dai;
    0
}

unsafe fn rockchip_i2s_tdm_path_check(i2s_tdm: *mut rk_i2s_tdm_dev, num: c_int, is_rx_path: bool) -> c_int {
    let i2s_data = if is_rx_path { (*i2s_tdm).i2s_sdis.as_mut_ptr() } else { (*i2s_tdm).i2s_sdos.as_mut_ptr() };
    let mut i = 0;
    while i < num {
        if *i2s_data.add(i as usize) > (CH_GRP_MAX as c_uint - 1) {
            dev_err((*i2s_tdm).dev, b"%s path i2s_data[%d]: %d is too high, max is: %d\n\0".as_ptr() as *const c_char,
                    if is_rx_path { b"RX\0".as_ptr() } else { b"TX\0".as_ptr() }, i, *i2s_data.add(i as usize), CH_GRP_MAX as c_int);
            return -EINVAL;
        }
        let mut j = 0;
        while j < num {
            if i != j && *i2s_data.add(i as usize) == *i2s_data.add(j as usize) {
                dev_err((*i2s_tdm).dev, b"%s path invalid routed i2s_data: [%d]%d == [%d]%d\n\0".as_ptr() as *const c_char,
                        if is_rx_path { b"RX\0".as_ptr() } else { b"TX\0".as_ptr() }, i, *i2s_data.add(i as usize), j, *i2s_data.add(j as usize));
                return -EINVAL;
            }
            j += 1;
        }
        i += 1;
    }
    0
}

unsafe fn rockchip_i2s_tdm_tx_path_config(i2s_tdm: *mut rk_i2s_tdm_dev, num: c_int) {
    let mut idx = 0;
    while idx < num {
        regmap_update_bits((*i2s_tdm).regmap, I2S_TXCR, I2S_TXCR_PATH_MASK(idx), I2S_TXCR_PATH(idx, (*i2s_tdm).i2s_sdos[idx as usize]));
        idx += 1;
    }
}

unsafe fn rockchip_i2s_tdm_rx_path_config(i2s_tdm: *mut rk_i2s_tdm_dev, num: c_int) {
    let mut idx = 0;
    while idx < num {
        regmap_update_bits((*i2s_tdm).regmap, I2S_RXCR, I2S_RXCR_PATH_MASK(idx), I2S_RXCR_PATH(idx, (*i2s_tdm).i2s_sdis[idx as usize]));
        idx += 1;
    }
}

unsafe fn rockchip_i2s_tdm_path_config(i2s_tdm: *mut rk_i2s_tdm_dev, num: c_int, is_rx_path: bool) {
    if is_rx_path { rockchip_i2s_tdm_rx_path_config(i2s_tdm, num); } else { rockchip_i2s_tdm_tx_path_config(i2s_tdm, num); }
}

unsafe fn rockchip_i2s_tdm_path_prepare(i2s_tdm: *mut rk_i2s_tdm_dev, np: *mut device_node, is_rx_path: bool) -> c_int {
    let i2s_tx_path_prop = b"rockchip,i2s-tx-route\0".as_ptr() as *const c_char;
    let i2s_rx_path_prop = b"rockchip,i2s-rx-route\0".as_ptr() as *const c_char;
    let i2s_path_prop = if is_rx_path { i2s_rx_path_prop } else { i2s_tx_path_prop };
    let i2s_data = if is_rx_path { (*i2s_tdm).i2s_sdis.as_mut_ptr() } else { (*i2s_tdm).i2s_sdos.as_mut_ptr() };
    let mut ret: c_int = 0;
    let num = of_count_phandle_with_args(np, i2s_path_prop, null());
    if num < 0 {
        if num != -ENOENT {
            dev_err((*i2s_tdm).dev, b"Failed to read '%s' num: %d\n\0".as_ptr() as *const c_char, i2s_path_prop, num);
            ret = num;
        }
        return ret;
    } else if num != CH_GRP_MAX as c_int {
        dev_err((*i2s_tdm).dev, b"The num: %d should be: %d\n\0".as_ptr() as *const c_char, num, CH_GRP_MAX as c_int);
        return -EINVAL;
    }
    ret = of_property_read_u32_array(np, i2s_path_prop, i2s_data, num);
    if ret < 0 {
        dev_err((*i2s_tdm).dev, b"Failed to read '%s': %d\n\0".as_ptr() as *const c_char, i2s_path_prop, ret);
        return ret;
    }
    ret = rockchip_i2s_tdm_path_check(i2s_tdm, num, is_rx_path);
    if ret < 0 {
        dev_err((*i2s_tdm).dev, b"Failed to check i2s data bus: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    rockchip_i2s_tdm_path_config(i2s_tdm, num, is_rx_path);
    0
}

unsafe fn rockchip_i2s_tdm_tx_path_prepare(i2s_tdm: *mut rk_i2s_tdm_dev, np: *mut device_node) -> c_int {
    rockchip_i2s_tdm_path_prepare(i2s_tdm, np, false)
}

unsafe fn rockchip_i2s_tdm_rx_path_prepare(i2s_tdm: *mut rk_i2s_tdm_dev, np: *mut device_node) -> c_int {
    rockchip_i2s_tdm_path_prepare(i2s_tdm, np, true)
}

unsafe extern "C" fn rockchip_i2s_tdm_probe(pdev: *mut platform_device) -> c_int {
    let node = (*pdev).dev.of_node;
    let mut res: *mut resource = null_mut();
    let i2s_tdm = devm_kzalloc(&mut (*pdev).dev, size_of::<rk_i2s_tdm_dev>(), GFP_KERNEL) as *mut rk_i2s_tdm_dev;
    if i2s_tdm.is_null() { return -ENOMEM; }
    (*i2s_tdm).dev = &mut (*pdev).dev;
    spin_lock_init(&mut (*i2s_tdm).lock);
    (*i2s_tdm).soc_data = device_get_match_data(&mut (*pdev).dev) as *const rk_i2s_soc_data;
    (*i2s_tdm).frame_width = 64;
    (*i2s_tdm).clk_trcm = TRCM_TXRX;
    if of_property_read_bool(node, b"rockchip,trcm-sync-tx-only\0".as_ptr() as *const c_char) { (*i2s_tdm).clk_trcm = TRCM_TX; }
    if of_property_read_bool(node, b"rockchip,trcm-sync-rx-only\0".as_ptr() as *const c_char) {
        if (*i2s_tdm).clk_trcm != 0 {
            dev_err((*i2s_tdm).dev, b"invalid trcm-sync configuration\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
        (*i2s_tdm).clk_trcm = TRCM_RX;
    }
    let mut ret = rockchip_i2s_tdm_init_dai(i2s_tdm);
    if ret != 0 { return ret; }
    (*i2s_tdm).grf = syscon_regmap_lookup_by_phandle(node, b"rockchip,grf\0".as_ptr() as *const c_char);
    (*i2s_tdm).tx_reset = devm_reset_control_get_optional_exclusive(&mut (*pdev).dev, b"tx-m\0".as_ptr() as *const c_char);
    if IS_ERR((*i2s_tdm).tx_reset as *const c_void) { return dev_err_probe((*i2s_tdm).dev, PTR_ERR((*i2s_tdm).tx_reset as *const c_void), b"Error in tx-m reset control\n\0".as_ptr() as *const c_char); }
    (*i2s_tdm).rx_reset = devm_reset_control_get_optional_exclusive(&mut (*pdev).dev, b"rx-m\0".as_ptr() as *const c_char);
    if IS_ERR((*i2s_tdm).rx_reset as *const c_void) { return dev_err_probe((*i2s_tdm).dev, PTR_ERR((*i2s_tdm).rx_reset as *const c_void), b"Error in rx-m reset control\n\0".as_ptr() as *const c_char); }
    (*i2s_tdm).hclk = devm_clk_get(&mut (*pdev).dev, b"hclk\0".as_ptr() as *const c_char);
    if IS_ERR((*i2s_tdm).hclk as *const c_void) { return dev_err_probe((*i2s_tdm).dev, PTR_ERR((*i2s_tdm).hclk as *const c_void), b"Failed to get clock hclk\n\0".as_ptr() as *const c_char); }
    (*i2s_tdm).mclk_tx = devm_clk_get(&mut (*pdev).dev, b"mclk_tx\0".as_ptr() as *const c_char);
    if IS_ERR((*i2s_tdm).mclk_tx as *const c_void) { return dev_err_probe((*i2s_tdm).dev, PTR_ERR((*i2s_tdm).mclk_tx as *const c_void), b"Failed to get clock mclk_tx\n\0".as_ptr() as *const c_char); }
    (*i2s_tdm).mclk_rx = devm_clk_get(&mut (*pdev).dev, b"mclk_rx\0".as_ptr() as *const c_char);
    if IS_ERR((*i2s_tdm).mclk_rx as *const c_void) { return dev_err_probe((*i2s_tdm).dev, PTR_ERR((*i2s_tdm).mclk_rx as *const c_void), b"Failed to get clock mclk_rx\n\0".as_ptr() as *const c_char); }
    (*i2s_tdm).io_multiplex = of_property_read_bool(node, b"rockchip,io-multiplex\0".as_ptr() as *const c_char);
    let regs = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR(regs as *const c_void) { return dev_err_probe((*i2s_tdm).dev, PTR_ERR(regs as *const c_void), b"Failed to get resource IORESOURCE_MEM\n\0".as_ptr() as *const c_char); }
    (*i2s_tdm).regmap = devm_regmap_init_mmio(&mut (*pdev).dev, regs, &rockchip_i2s_tdm_regmap_config);
    if IS_ERR((*i2s_tdm).regmap as *const c_void) { return dev_err_probe((*i2s_tdm).dev, PTR_ERR((*i2s_tdm).regmap as *const c_void), b"Failed to initialise regmap\n\0".as_ptr() as *const c_char); }
    if (*i2s_tdm).has_playback {
        (*i2s_tdm).playback_dma_data.addr = (*res).start + I2S_TXDR as c_ulong;
        (*i2s_tdm).playback_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
        (*i2s_tdm).playback_dma_data.maxburst = 8;
    }
    if (*i2s_tdm).has_capture {
        (*i2s_tdm).capture_dma_data.addr = (*res).start + I2S_RXDR as c_ulong;
        (*i2s_tdm).capture_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
        (*i2s_tdm).capture_dma_data.maxburst = 8;
    }
    ret = rockchip_i2s_tdm_tx_path_prepare(i2s_tdm, node);
    if ret < 0 { dev_err(&mut (*pdev).dev, b"I2S TX path prepare failed: %d\n\0".as_ptr() as *const c_char, ret); return ret; }
    ret = rockchip_i2s_tdm_rx_path_prepare(i2s_tdm, node);
    if ret < 0 { dev_err(&mut (*pdev).dev, b"I2S RX path prepare failed: %d\n\0".as_ptr() as *const c_char, ret); return ret; }
    dev_set_drvdata(&mut (*pdev).dev, i2s_tdm as *mut c_void);
    ret = clk_prepare_enable((*i2s_tdm).hclk);
    if ret != 0 { return dev_err_probe((*i2s_tdm).dev, ret, b"Failed to enable clock hclk\n\0".as_ptr() as *const c_char); }
    ret = i2s_tdm_prepare_enable_mclk(i2s_tdm);
    if ret != 0 {
        dev_err_probe((*i2s_tdm).dev, ret, b"Failed to enable one or more mclks\n\0".as_ptr() as *const c_char);
        clk_disable_unprepare((*i2s_tdm).hclk);
        return ret;
    }
    pm_runtime_enable(&mut (*pdev).dev);
    regmap_update_bits((*i2s_tdm).regmap, I2S_DMACR, I2S_DMACR_TDL_MASK, I2S_DMACR_TDL(16));
    regmap_update_bits((*i2s_tdm).regmap, I2S_DMACR, I2S_DMACR_RDL_MASK, I2S_DMACR_RDL(16));
    regmap_update_bits((*i2s_tdm).regmap, I2S_CKR, I2S_CKR_TRCM_MASK, (*i2s_tdm).clk_trcm << I2S_CKR_TRCM_SHIFT);
    if !(*i2s_tdm).soc_data.is_null() {
        if let Some(init) = (*(*i2s_tdm).soc_data).init { init(&mut (*pdev).dev, (*res).start as u32); }
    }
    ret = devm_snd_soc_register_component(&mut (*pdev).dev, &rockchip_i2s_tdm_component, (*i2s_tdm).dai, 1);
    if ret != 0 {
        if !pm_runtime_status_suspended(&mut (*pdev).dev) { i2s_tdm_runtime_suspend(&mut (*pdev).dev); }
        pm_runtime_disable(&mut (*pdev).dev);
        clk_disable_unprepare((*i2s_tdm).hclk);
        return ret;
    }
    ret = devm_snd_dmaengine_pcm_register(&mut (*pdev).dev, null(), 0);
    if ret != 0 {
        if !pm_runtime_status_suspended(&mut (*pdev).dev) { i2s_tdm_runtime_suspend(&mut (*pdev).dev); }
        pm_runtime_disable(&mut (*pdev).dev);
        clk_disable_unprepare((*i2s_tdm).hclk);
        return ret;
    }
    0
}

unsafe extern "C" fn rockchip_i2s_tdm_remove(pdev: *mut platform_device) {
    if !pm_runtime_status_suspended(&mut (*pdev).dev) { i2s_tdm_runtime_suspend(&mut (*pdev).dev); }
    pm_runtime_disable(&mut (*pdev).dev);
}

unsafe extern "C" fn rockchip_i2s_tdm_suspend(dev: *mut device) -> c_int {
    let i2s_tdm = dev_get_drvdata(dev) as *mut rk_i2s_tdm_dev;
    regcache_mark_dirty((*i2s_tdm).regmap);
    0
}

unsafe extern "C" fn rockchip_i2s_tdm_resume(dev: *mut device) -> c_int {
    let i2s_tdm = dev_get_drvdata(dev) as *mut rk_i2s_tdm_dev;
    let mut ret = pm_runtime_resume_and_get(dev);
    if ret < 0 { return ret; }
    ret = regcache_sync((*i2s_tdm).regmap);
    pm_runtime_put(dev);
    ret
}

// static const struct dev_pm_ops rockchip_i2s_tdm_pm_ops = {
//	RUNTIME_PM_OPS(i2s_tdm_runtime_suspend, i2s_tdm_runtime_resume, NULL)
//	SYSTEM_SLEEP_PM_OPS(rockchip_i2s_tdm_suspend, rockchip_i2s_tdm_resume)
// };
static rockchip_i2s_tdm_pm_ops: dev_pm_ops = dev_pm_ops { _private: [] };

static mut rockchip_i2s_tdm_driver: platform_driver = platform_driver {
    probe: Some(rockchip_i2s_tdm_probe),
    remove: Some(rockchip_i2s_tdm_remove),
    driver: device_driver {
        name: DRV_NAME.as_ptr() as *const c_char,
        of_match_table: rockchip_i2s_tdm_match.as_ptr(),
        pm: &rockchip_i2s_tdm_pm_ops,
    },
};
// module_platform_driver(rockchip_i2s_tdm_driver);
//
// MODULE_DESCRIPTION("ROCKCHIP I2S/TDM ASoC Interface");
// MODULE_AUTHOR("Sugar Zhang <sugar.zhang@rock-chips.com>");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:" DRV_NAME);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
