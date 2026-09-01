// SPDX-License-Identifier: GPL-2.0-only
/*
 *  STM32 ALSA SoC Digital Audio Interface (I2S) driver.
 *
 * Copyright (C) 2017, STMicroelectronics - All Rights Reserved
 * Author(s): Olivier Moysan <olivier.moysan@st.com> for STMicroelectronics.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_t = bool;
type u32 = u32;
type u64 = u64;
type dma_addr_t = u64;
type irqreturn_t = c_int;

const fn BIT(n: u32) -> u32 {
    1u32 << n
}

const fn GENMASK(h: u32, l: u32) -> u32 {
    u32::MAX.wrapping_shl(l) & u32::MAX.wrapping_shr(31 - h)
}

const fn FIELD_GET(mask: u32, reg: u32) -> u32 {
    (reg & mask) >> mask.trailing_zeros()
}

const fn DIV_ROUND_CLOSEST(n: c_ulong, d: c_ulong) -> c_ulong {
    (n + d / 2) / d
}

const fn mul_u32_u32(a: u32, b: u32) -> u64 {
    (a as u64) * (b as u64)
}

const fn div_u64(a: u64, b: u32) -> u64 {
    a / (b as u64)
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
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
pub struct reset_control {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub start: dma_addr_t,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr_width: c_uint,
    pub addr: dma_addr_t,
    pub maxburst: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *mut c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub ops: *const snd_soc_dai_ops,
    pub id: c_int,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub buffer_bytes_max: usize,
    pub period_bytes_min: usize,
    pub period_bytes_max: usize,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
}

#[repr(C)]
pub struct snd_dmaengine_pcm_config {
    pub pcm_hardware: *const snd_pcm_hardware,
    pub prepare_slave_config: Option<unsafe extern "C" fn()>,
    pub prealloc_buffer_size: usize,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub legacy_dai_naming: c_int,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device_with_node,
}

#[repr(C)]
pub struct device_with_node {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct clk_hw {
    pub init: *const clk_init_data,
    pub clk: *mut clk,
}

#[repr(C)]
pub struct clk_init_data {
    pub name: *const c_char,
    pub ops: *const clk_ops,
    pub flags: c_ulong,
    pub parent_names: *const *const c_char,
    pub num_parents: u8,
}

#[repr(C)]
pub struct clk_rate_request {
    pub rate: c_ulong,
    pub best_parent_rate: c_ulong,
}

#[repr(C)]
pub struct clk_ops {
    pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>,
    pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong) -> c_ulong>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> c_int>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong, c_ulong) -> c_int>,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_int,
    pub reg_stride: c_int,
    pub val_bits: c_int,
    pub max_register: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_t>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_t>,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_t>,
    pub num_reg_defaults_raw: c_uint,
    pub fast_io: bool_t,
    pub cache_type: c_uint,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

const STM32_I2S_CR1_REG: c_uint = 0x0;
const STM32_I2S_CFG1_REG: c_uint = 0x08;
const STM32_I2S_CFG2_REG: c_uint = 0x0C;
const STM32_I2S_IER_REG: c_uint = 0x10;
const STM32_I2S_SR_REG: c_uint = 0x14;
const STM32_I2S_IFCR_REG: c_uint = 0x18;
const STM32_I2S_TXDR_REG: c_uint = 0x20;
const STM32_I2S_RXDR_REG: c_uint = 0x30;
const STM32_I2S_CGFR_REG: c_uint = 0x50;
const STM32_I2S_HWCFGR_REG: c_uint = 0x3F0;
const STM32_I2S_VERR_REG: c_uint = 0x3F4;
const STM32_I2S_IPIDR_REG: c_uint = 0x3F8;
const STM32_I2S_SIDR_REG: c_uint = 0x3FC;

/* Bit definition for SPI2S_CR1 register */
const I2S_CR1_SPE: u32 = BIT(0);
const I2S_CR1_CSTART: u32 = BIT(9);
const I2S_CR1_CSUSP: u32 = BIT(10);
const I2S_CR1_HDDIR: u32 = BIT(11);
const I2S_CR1_SSI: u32 = BIT(12);
const I2S_CR1_CRC33_17: u32 = BIT(13);
const I2S_CR1_RCRCI: u32 = BIT(14);
const I2S_CR1_TCRCI: u32 = BIT(15);

/* Bit definition for SPI_CFG2 register */
const I2S_CFG2_IOSWP_SHIFT: u32 = 15;
const I2S_CFG2_IOSWP: u32 = BIT(I2S_CFG2_IOSWP_SHIFT);
const I2S_CFG2_LSBFRST: u32 = BIT(23);
const I2S_CFG2_AFCNTR: u32 = BIT(31);

/* Bit definition for SPI_CFG1 register */
const I2S_CFG1_FTHVL_SHIFT: u32 = 5;
const I2S_CFG1_FTHVL_MASK: u32 = GENMASK(8, I2S_CFG1_FTHVL_SHIFT);
const fn I2S_CFG1_FTHVL_SET(x: u32) -> u32 {
    x << I2S_CFG1_FTHVL_SHIFT
}

const I2S_CFG1_TXDMAEN: u32 = BIT(15);
const I2S_CFG1_RXDMAEN: u32 = BIT(14);

/* Bit definition for SPI2S_IER register */
const I2S_IER_RXPIE: u32 = BIT(0);
const I2S_IER_TXPIE: u32 = BIT(1);
const I2S_IER_DPXPIE: u32 = BIT(2);
const I2S_IER_EOTIE: u32 = BIT(3);
const I2S_IER_TXTFIE: u32 = BIT(4);
const I2S_IER_UDRIE: u32 = BIT(5);
const I2S_IER_OVRIE: u32 = BIT(6);
const I2S_IER_CRCEIE: u32 = BIT(7);
const I2S_IER_TIFREIE: u32 = BIT(8);
const I2S_IER_MODFIE: u32 = BIT(9);
const I2S_IER_TSERFIE: u32 = BIT(10);

/* Bit definition for SPI2S_SR register */
const I2S_SR_RXP: u32 = BIT(0);
const I2S_SR_TXP: u32 = BIT(1);
const I2S_SR_DPXP: u32 = BIT(2);
const I2S_SR_EOT: u32 = BIT(3);
const I2S_SR_TXTF: u32 = BIT(4);
const I2S_SR_UDR: u32 = BIT(5);
const I2S_SR_OVR: u32 = BIT(6);
const I2S_SR_CRCERR: u32 = BIT(7);
const I2S_SR_TIFRE: u32 = BIT(8);
const I2S_SR_MODF: u32 = BIT(9);
const I2S_SR_TSERF: u32 = BIT(10);
const I2S_SR_SUSP: u32 = BIT(11);
const I2S_SR_TXC: u32 = BIT(12);
const I2S_SR_RXPLVL: u32 = GENMASK(14, 13);
const I2S_SR_RXWNE: u32 = BIT(15);

const I2S_SR_MASK: u32 = GENMASK(15, 0);

/* Bit definition for SPI_IFCR register */
const I2S_IFCR_EOTC: u32 = BIT(3);
const I2S_IFCR_TXTFC: u32 = BIT(4);
const I2S_IFCR_UDRC: u32 = BIT(5);
const I2S_IFCR_OVRC: u32 = BIT(6);
const I2S_IFCR_CRCEC: u32 = BIT(7);
const I2S_IFCR_TIFREC: u32 = BIT(8);
const I2S_IFCR_MODFC: u32 = BIT(9);
const I2S_IFCR_TSERFC: u32 = BIT(10);
const I2S_IFCR_SUSPC: u32 = BIT(11);

const I2S_IFCR_MASK: u32 = GENMASK(11, 3);

/* Bit definition for SPI_I2SCGFR register */
const I2S_CGFR_I2SMOD: u32 = BIT(0);

const I2S_CGFR_I2SCFG_SHIFT: u32 = 1;
const I2S_CGFR_I2SCFG_MASK: u32 = GENMASK(3, I2S_CGFR_I2SCFG_SHIFT);
const fn I2S_CGFR_I2SCFG_SET(x: u32) -> u32 {
    x << I2S_CGFR_I2SCFG_SHIFT
}

const I2S_CGFR_I2SSTD_SHIFT: u32 = 4;
const I2S_CGFR_I2SSTD_MASK: u32 = GENMASK(5, I2S_CGFR_I2SSTD_SHIFT);
const fn I2S_CGFR_I2SSTD_SET(x: u32) -> u32 {
    x << I2S_CGFR_I2SSTD_SHIFT
}

const I2S_CGFR_PCMSYNC: u32 = BIT(7);

const I2S_CGFR_DATLEN_SHIFT: u32 = 8;
const I2S_CGFR_DATLEN_MASK: u32 = GENMASK(9, I2S_CGFR_DATLEN_SHIFT);
const fn I2S_CGFR_DATLEN_SET(x: u32) -> u32 {
    x << I2S_CGFR_DATLEN_SHIFT
}

const I2S_CGFR_CHLEN_SHIFT: u32 = 10;
const I2S_CGFR_CHLEN: u32 = BIT(I2S_CGFR_CHLEN_SHIFT);
const I2S_CGFR_CKPOL: u32 = BIT(11);
const I2S_CGFR_FIXCH: u32 = BIT(12);
const I2S_CGFR_WSINV: u32 = BIT(13);
const I2S_CGFR_DATFMT: u32 = BIT(14);

const I2S_CGFR_I2SDIV_SHIFT: u32 = 16;
const I2S_CGFR_I2SDIV_BIT_H: u32 = 23;
const I2S_CGFR_I2SDIV_MASK: u32 = GENMASK(I2S_CGFR_I2SDIV_BIT_H, I2S_CGFR_I2SDIV_SHIFT);
const fn I2S_CGFR_I2SDIV_SET(x: u32) -> u32 {
    x << I2S_CGFR_I2SDIV_SHIFT
}
const I2S_CGFR_I2SDIV_MAX: u32 =
    (1 << (I2S_CGFR_I2SDIV_BIT_H - I2S_CGFR_I2SDIV_SHIFT)) - 1;

const I2S_CGFR_ODD_SHIFT: u32 = 24;
const I2S_CGFR_ODD: u32 = BIT(I2S_CGFR_ODD_SHIFT);
const I2S_CGFR_MCKOE: u32 = BIT(25);

/* Registers below apply to I2S version 1.1 and more */

/* Bit definition for SPI_HWCFGR register */
const I2S_HWCFGR_I2S_SUPPORT_MASK: u32 = GENMASK(15, 12);

/* Bit definition for SPI_VERR register */
const I2S_VERR_MIN_MASK: u32 = GENMASK(3, 0);
const I2S_VERR_MAJ_MASK: u32 = GENMASK(7, 4);

/* Bit definition for SPI_IPIDR register */
const I2S_IPIDR_ID_MASK: u32 = GENMASK(31, 0);

/* Bit definition for SPI_SIDR register */
const I2S_SIDR_ID_MASK: u32 = GENMASK(31, 0);

const I2S_IPIDR_NUMBER: u32 = 0x00130022;

#[repr(C)]
enum i2s_master_mode {
    I2S_MS_NOT_SET,
    I2S_MS_MASTER,
    I2S_MS_SLAVE,
}

#[repr(C)]
enum i2s_mode {
    I2S_I2SMOD_TX_SLAVE,
    I2S_I2SMOD_RX_SLAVE,
    I2S_I2SMOD_TX_MASTER,
    I2S_I2SMOD_RX_MASTER,
    I2S_I2SMOD_FD_SLAVE,
    I2S_I2SMOD_FD_MASTER,
}

#[repr(C)]
enum i2s_fifo_th {
    I2S_FIFO_TH_NONE,
    I2S_FIFO_TH_ONE_QUARTER,
    I2S_FIFO_TH_HALF,
    I2S_FIFO_TH_THREE_QUARTER,
    I2S_FIFO_TH_FULL,
}

#[repr(C)]
enum i2s_std {
    I2S_STD_I2S,
    I2S_STD_LEFT_J,
    I2S_STD_RIGHT_J,
    I2S_STD_DSP,
}

#[repr(C)]
enum i2s_datlen {
    I2S_I2SMOD_DATLEN_16,
    I2S_I2SMOD_DATLEN_24,
    I2S_I2SMOD_DATLEN_32,
}

const STM32_I2S_FIFO_SIZE: u32 = 16;

unsafe fn STM32_I2S_IS_MASTER(x: *mut stm32_i2s_data) -> bool {
    unsafe { (*x).ms_flg == i2s_master_mode::I2S_MS_MASTER as c_int }
}

unsafe fn STM32_I2S_IS_SLAVE(x: *mut stm32_i2s_data) -> bool {
    unsafe { (*x).ms_flg == i2s_master_mode::I2S_MS_SLAVE as c_int }
}

const STM32_I2S_NAME_LEN: usize = 32;
const STM32_I2S_RATE_11K: u32 = 11025;
const STM32_I2S_MAX_SAMPLE_RATE_8K: u32 = 192000;
const STM32_I2S_MAX_SAMPLE_RATE_11K: u32 = 176400;
const STM32_I2S_CLK_RATE_TOLERANCE: u64 = 1000; /* ppm */

/**
 * struct stm32_i2s_data - private data of I2S
 * @conf: I2S configuration pointer
 * @regmap: I2S register map pointer
 * @pdev: device data pointer
 * @dai_drv: DAI driver pointer
 * @dma_data_tx: dma configuration data for tx channel
 * @dma_data_rx: dma configuration data for tx channel
 * @substream: PCM substream data pointer
 * @i2sclk: kernel clock feeding the I2S clock generator
 * @i2smclk: master clock from I2S mclk provider
 * @pclk: peripheral clock driving bus interface
 * @x8kclk: I2S parent clock for sampling frequencies multiple of 8kHz
 * @x11kclk: I2S parent clock for sampling frequencies multiple of 11kHz
 * @base:  mmio register base virtual address
 * @phys_addr: I2S registers physical base address
 * @lock_fd: lock to manage race conditions in full duplex mode
 * @irq_lock: prevent race condition with IRQ
 * @mclk_rate: master clock frequency (Hz)
 * @fmt: DAI protocol
 * @divider: prescaler division ratio
 * @div: prescaler div field
 * @odd: prescaler odd field
 * @i2s_clk_flg: flag set while exclusivity on I2S kernel clock is active
 * @refcount: keep count of opened streams on I2S
 * @ms_flg: master mode flag.
 * @set_i2s_clk_rate: set I2S kernel clock rate
 * @put_i2s_clk_rate: put I2S kernel clock rate
 */
#[repr(C)]
pub struct stm32_i2s_data {
    pub conf: *const stm32_i2s_conf,
    pub regmap: *mut regmap,
    pub pdev: *mut platform_device,
    pub dai_drv: *mut snd_soc_dai_driver,
    pub dma_data_tx: snd_dmaengine_dai_dma_data,
    pub dma_data_rx: snd_dmaengine_dai_dma_data,
    pub substream: *mut snd_pcm_substream,
    pub i2sclk: *mut clk,
    pub i2smclk: *mut clk,
    pub pclk: *mut clk,
    pub x8kclk: *mut clk,
    pub x11kclk: *mut clk,
    pub base: *mut c_void,
    pub phys_addr: dma_addr_t,
    pub lock_fd: spinlock_t, /* Manage race conditions for full duplex */
    pub irq_lock: spinlock_t, /* used to prevent race condition with IRQ */
    pub mclk_rate: c_uint,
    pub fmt: c_uint,
    pub divider: c_uint,
    pub div: c_uint,
    pub odd: bool,
    pub i2s_clk_flg: bool,
    pub refcount: c_int,
    pub ms_flg: c_int,
    pub set_i2s_clk_rate: Option<unsafe extern "C" fn(*mut stm32_i2s_data, c_uint) -> c_int>,
    pub put_i2s_clk_rate: Option<unsafe extern "C" fn(*mut stm32_i2s_data)>,
}

/**
 * struct stm32_i2s_conf - I2S configuration
 * @regmap_conf: regmap configuration pointer
 * @get_i2s_clk_parent: get parent clock of I2S kernel clock
 */
#[repr(C)]
pub struct stm32_i2s_conf {
    pub regmap_conf: *const regmap_config,
    pub get_i2s_clk_parent: Option<unsafe extern "C" fn(*mut stm32_i2s_data) -> c_int>,
}

#[repr(C)]
pub struct stm32_i2smclk_data {
    pub hw: clk_hw,
    pub freq: c_ulong,
    pub i2s_data: *mut stm32_i2s_data,
}

unsafe fn to_mclk_data(_hw: *mut clk_hw) -> *mut stm32_i2smclk_data {
    _hw as *mut stm32_i2smclk_data
}

extern "C" {
    static snd_dmaengine_pcm_prepare_slave_config: unsafe extern "C" fn();
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_long, fmt: *const c_char, ...) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: u32, val: u32) -> c_int;
    fn regmap_write_bits(map: *mut regmap, reg: c_uint, mask: u32, val: u32) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: u32) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut u32) -> c_int;
    fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> c_int;
    fn clk_rate_exclusive_put(clk: *mut clk);
    fn clk_rate_exclusive_get(clk: *mut clk) -> c_int;
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn clk_round_rate(clk: *mut clk, rate: c_ulong) -> c_long;
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn clk_set_rate_exclusive(clk: *mut clk, rate: c_ulong) -> c_int;
    fn __clk_get_name(clk: *mut clk) -> *const c_char;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn devm_clk_hw_register(dev: *mut device, hw: *mut clk_hw) -> c_int;
    fn devm_of_clk_add_hw_provider(dev: *mut device, get: *const c_void, hw: *mut clk_hw) -> c_int;
    static of_clk_hw_simple_get: c_void;
    fn snd_pcm_stop_xrun(substream: *mut snd_pcm_substream);
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_pcm_hw_constraint_single(runtime: *mut snd_pcm_runtime, var: c_uint, val: c_uint) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn snd_pcm_direction_name(stream: c_int) -> *const c_char;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_soc_dai_init_dma_data(dai: *mut snd_soc_dai, tx: *mut snd_dmaengine_dai_dma_data, rx: *mut snd_dmaengine_dai_dma_data);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_platform_get_and_ioremap_resource(pdev: *mut platform_device, index: c_uint, res: *mut *mut resource) -> *mut c_void;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn of_property_present(np: *mut device_node, propname: *const c_char) -> bool;
    fn platform_get_irq(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn devm_request_irq(dev: *mut device, irq: c_uint, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn devm_reset_control_get_optional_exclusive(dev: *mut device, id: *const c_char) -> *mut reset_control;
    fn reset_control_assert(rst: *mut reset_control) -> c_int;
    fn reset_control_deassert(rst: *mut reset_control) -> c_int;
    fn udelay(usecs: c_ulong);
    fn snd_dmaengine_pcm_unregister(dev: *mut device);
    fn snd_soc_unregister_component(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t) -> c_ulong;
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_regmap_init_mmio_clk(dev: *mut device, clk_id: *const c_char, regs: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn snd_dmaengine_pcm_register(dev: *mut device, config: *const snd_dmaengine_pcm_config, flags: c_uint) -> c_int;
    fn snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
}

type c_long = isize;

const GFP_KERNEL: c_uint = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const EBUSY: c_int = 16;
const EPERM: c_int = 1;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const DMA_SLAVE_BUSWIDTH_UNDEFINED: c_uint = 0;
const REGCACHE_FLAT: c_uint = 1;
const PAGE_SIZE: usize = 4096;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 0;
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 1;
const SNDRV_PCM_RATE_8000_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 1;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_uint = 0;
const SND_SOC_CLOCK_OUT: c_int = 1;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_MSB: c_uint = 2;
const SND_SOC_DAIFMT_LSB: c_uint = 3;
const SND_SOC_DAIFMT_DSP_A: c_uint = 4;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0x00f0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0x0000;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0x0010;
const SND_SOC_DAIFMT_NB_IF: c_uint = 0x0020;
const SND_SOC_DAIFMT_IB_IF: c_uint = 0x0030;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0x0f00;
const SND_SOC_DAIFMT_BC_FC: c_uint = 0x0100;
const SND_SOC_DAIFMT_BP_FP: c_uint = 0x0200;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 2;
const SNDRV_PCM_TRIGGER_STOP: c_int = 3;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 4;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 5;

unsafe fn IS_ERR<T>(ptr: *mut T) -> bool {
    (ptr as isize) < 0 && (ptr as isize) > -4096
}

unsafe fn PTR_ERR<T>(ptr: *mut T) -> c_long {
    ptr as c_long
}

unsafe extern "C" fn stm32_i2s_get_parent_clk(i2s: *mut stm32_i2s_data) -> c_int {
    let dev = unsafe { &mut (*(*i2s).pdev).dev as *mut device_with_node as *mut device };

    unsafe { (*i2s).x8kclk = devm_clk_get(dev, c"x8k".as_ptr()) };
    if unsafe { IS_ERR((*i2s).x8kclk) } {
        return unsafe { dev_err_probe(dev, PTR_ERR((*i2s).x8kclk), c"Cannot get x8k parent clock\n".as_ptr()) };
    }

    unsafe { (*i2s).x11kclk = devm_clk_get(dev, c"x11k".as_ptr()) };
    if unsafe { IS_ERR((*i2s).x11kclk) } {
        return unsafe { dev_err_probe(dev, PTR_ERR((*i2s).x11kclk), c"Cannot get x11k parent clock\n".as_ptr()) };
    }

    0
}

unsafe extern "C" fn stm32_i2s_calc_clk_div(
    i2s: *mut stm32_i2s_data,
    input_rate: c_ulong,
    output_rate: c_ulong,
) -> c_int {
    let ratio: c_uint;
    let div: c_uint;
    let mut divider: c_uint = 1;
    let odd: bool;

    ratio = DIV_ROUND_CLOSEST(input_rate, output_rate) as c_uint;

    /* Check the parity of the divider */
    odd = (ratio & 0x1) != 0;

    /* Compute the div prescaler */
    div = ratio >> 1;

    /* If div is 0 actual divider is 1 */
    if div != 0 {
        divider = (2 * div) + odd as c_uint;
        unsafe {
            dev_dbg(
                &mut (*(*i2s).pdev).dev as *mut device_with_node as *mut device,
                c"Divider: 2*%d(div)+%d(odd) = %d\n".as_ptr(),
                div,
                odd as c_int,
                divider,
            );
        }
    }

    /* Division by three is not allowed by I2S prescaler */
    if (div == 1 && odd) || div > I2S_CGFR_I2SDIV_MAX {
        unsafe {
            dev_err(
                &mut (*(*i2s).pdev).dev as *mut device_with_node as *mut device,
                c"Wrong divider setting\n".as_ptr(),
            );
        }
        return -EINVAL;
    }

    if input_rate % (divider as c_ulong) != 0 {
        unsafe {
            dev_dbg(
                &mut (*(*i2s).pdev).dev as *mut device_with_node as *mut device,
                c"Rate not accurate. requested (%ld), actual (%ld)\n".as_ptr(),
                output_rate,
                input_rate / divider as c_ulong,
            );
        }
    }

    unsafe {
        (*i2s).div = div;
        (*i2s).odd = odd;
        (*i2s).divider = divider;
    }

    0
}

unsafe extern "C" fn stm32_i2s_set_clk_div(i2s: *mut stm32_i2s_data) -> c_int {
    let cgfr: u32;
    let cgfr_mask: u32;

    unsafe {
        cgfr = I2S_CGFR_I2SDIV_SET((*i2s).div) | (((*i2s).odd as u32) << I2S_CGFR_ODD_SHIFT);
    }
    cgfr_mask = I2S_CGFR_I2SDIV_MASK | I2S_CGFR_ODD;

    unsafe { regmap_update_bits((*i2s).regmap, STM32_I2S_CGFR_REG, cgfr_mask, cgfr) }
}

unsafe extern "C" fn stm32_i2s_rate_accurate(
    i2s: *mut stm32_i2s_data,
    max_rate: c_uint,
    rate: c_uint,
) -> bool {
    let pdev = unsafe { (*i2s).pdev };
    let delta: u64;
    let dividend: u64;
    let ratio: c_int;

    if rate == 0 {
        unsafe {
            dev_err(&mut (*pdev).dev as *mut device_with_node as *mut device, c"Unexpected null rate\n".as_ptr());
        }
        return false;
    }

    ratio = DIV_ROUND_CLOSEST(max_rate as c_ulong, rate as c_ulong) as c_int;
    if ratio == 0 {
        return false;
    }

    dividend = mul_u32_u32(1000000, max_rate.abs_diff((ratio as u32).wrapping_mul(rate)));
    delta = div_u64(dividend, max_rate);

    if delta <= STM32_I2S_CLK_RATE_TOLERANCE {
        return true;
    }

    unsafe {
        dev_dbg(&mut (*pdev).dev as *mut device_with_node as *mut device, c"Rate [%u] not accurate\n".as_ptr(), rate);
    }

    false
}

unsafe extern "C" fn stm32_i2s_set_parent_clock(i2s: *mut stm32_i2s_data, rate: c_uint) -> c_int {
    let pdev = unsafe { (*i2s).pdev };
    let parent_clk: *mut clk;
    let ret: c_int;

    if rate % STM32_I2S_RATE_11K == 0 {
        parent_clk = unsafe { (*i2s).x11kclk };
    } else {
        parent_clk = unsafe { (*i2s).x8kclk };
    }

    ret = unsafe { clk_set_parent((*i2s).i2sclk, parent_clk) };
    if ret != 0 {
        unsafe {
            dev_err(
                &mut (*pdev).dev as *mut device_with_node as *mut device,
                c"Error %d setting i2sclk parent clock\n".as_ptr(),
                ret,
            );
        }
    }

    ret
}

unsafe extern "C" fn stm32_i2s_put_parent_rate(i2s: *mut stm32_i2s_data) {
    unsafe {
        if (*i2s).i2s_clk_flg {
            (*i2s).i2s_clk_flg = false;
            clk_rate_exclusive_put((*i2s).i2sclk);
        }
    }
}

unsafe extern "C" fn stm32_i2s_set_parent_rate(i2s: *mut stm32_i2s_data, rate: c_uint) -> c_int {
    let pdev = unsafe { (*i2s).pdev };
    let mut i2s_clk_rate: c_uint;
    let mut i2s_clk_max_rate: c_uint;
    let i2s_curr_rate: c_uint;
    let i2s_new_rate: c_uint;
    let ret: c_int;
    let mut div: c_int;

    /*
     * Set maximum expected kernel clock frequency
     * - mclk on:
     *   f_i2s_ck = MCKDIV * mclk-fs * fs
     *   Here typical 256 ratio is assumed for mclk-fs
     * - mclk off:
     *   f_i2s_ck = MCKDIV * FRL * fs
     *   Where FRL=[16,32], MCKDIV=[1..256]
     *   f_i2s_ck = i2s_clk_max_rate * 32 / 256
     */
    if rate % STM32_I2S_RATE_11K == 0 {
        i2s_clk_max_rate = STM32_I2S_MAX_SAMPLE_RATE_11K * 256;
    } else {
        i2s_clk_max_rate = STM32_I2S_MAX_SAMPLE_RATE_8K * 256;
    }

    if unsafe { (*i2s).i2smclk.is_null() } {
        i2s_clk_max_rate /= 8;
    }

    /* Request exclusivity, as the clock may be shared by I2S instances */
    unsafe {
        clk_rate_exclusive_get((*i2s).i2sclk);
        (*i2s).i2s_clk_flg = true;
    }

    /*
     * Check current kernel clock rate. If it gives the expected accuracy
     * return immediately.
     */
    i2s_curr_rate = unsafe { clk_get_rate((*i2s).i2sclk) as c_uint };
    if unsafe { stm32_i2s_rate_accurate(i2s, i2s_clk_max_rate, i2s_curr_rate) } {
        return 0;
    }

    /*
     * Otherwise try to set the maximum rate and check the new actual rate.
     * If the new rate does not give the expected accuracy, try to set
     * lower rates for the kernel clock.
     */
    i2s_clk_rate = i2s_clk_max_rate;
    div = 1;
    loop {
        /* Check new rate accuracy. Return if ok */
        i2s_new_rate = unsafe { clk_round_rate((*i2s).i2sclk, i2s_clk_rate as c_ulong) as c_uint };
        if unsafe { stm32_i2s_rate_accurate(i2s, i2s_clk_rate, i2s_new_rate) } {
            let set_ret = unsafe { clk_set_rate((*i2s).i2sclk, i2s_clk_rate as c_ulong) };
            if set_ret != 0 {
                unsafe {
                    dev_err(
                        &mut (*pdev).dev as *mut device_with_node as *mut device,
                        c"Error %d setting i2s_clk_rate rate. %s".as_ptr(),
                        set_ret,
                        if set_ret == -EBUSY {
                            c"Active stream rates may be in conflict\n".as_ptr()
                        } else {
                            c"\n".as_ptr()
                        },
                    );
                }
                unsafe { stm32_i2s_put_parent_rate(i2s) };
                return -EINVAL;
            }

            return 0;
        }

        /* Try a lower frequency */
        div += 1;
        i2s_clk_rate = i2s_clk_max_rate / div as u32;
        if i2s_clk_rate <= rate {
            break;
        }
    }

    /* no accurate rate found */
    unsafe {
        dev_err(&mut (*pdev).dev as *mut device_with_node as *mut device, c"Failed to find an accurate rate".as_ptr());
    }

    unsafe { stm32_i2s_put_parent_rate(i2s) };

    ret = -EINVAL;
    ret
}

unsafe extern "C" fn stm32_i2smclk_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int {
    let mclk = unsafe { to_mclk_data(hw) };
    let i2s = unsafe { (*mclk).i2s_data };
    let ret: c_int;

    ret = unsafe { stm32_i2s_calc_clk_div(i2s, (*req).best_parent_rate, (*req).rate) };
    if ret != 0 {
        return ret;
    }

    unsafe {
        (*mclk).freq = (*req).best_parent_rate / (*i2s).divider as c_ulong;
        (*req).rate = (*mclk).freq;
    }

    0
}

unsafe extern "C" fn stm32_i2smclk_recalc_rate(hw: *mut clk_hw, _parent_rate: c_ulong) -> c_ulong {
    let mclk = unsafe { to_mclk_data(hw) };

    unsafe { (*mclk).freq }
}

unsafe extern "C" fn stm32_i2smclk_set_rate(
    hw: *mut clk_hw,
    rate: c_ulong,
    parent_rate: c_ulong,
) -> c_int {
    let mclk = unsafe { to_mclk_data(hw) };
    let i2s = unsafe { (*mclk).i2s_data };
    let mut ret: c_int;

    ret = unsafe { stm32_i2s_calc_clk_div(i2s, parent_rate, rate) };
    if ret != 0 {
        return ret;
    }

    ret = unsafe { stm32_i2s_set_clk_div(i2s) };
    if ret != 0 {
        return ret;
    }

    unsafe { (*mclk).freq = rate };

    0
}

unsafe extern "C" fn stm32_i2smclk_enable(hw: *mut clk_hw) -> c_int {
    let mclk = unsafe { to_mclk_data(hw) };
    let i2s = unsafe { (*mclk).i2s_data };

    unsafe {
        dev_dbg(&mut (*(*i2s).pdev).dev as *mut device_with_node as *mut device, c"Enable master clock\n".as_ptr());
        regmap_update_bits((*i2s).regmap, STM32_I2S_CGFR_REG, I2S_CGFR_MCKOE, I2S_CGFR_MCKOE)
    }
}

unsafe extern "C" fn stm32_i2smclk_disable(hw: *mut clk_hw) {
    let mclk = unsafe { to_mclk_data(hw) };
    let i2s = unsafe { (*mclk).i2s_data };

    unsafe {
        dev_dbg(&mut (*(*i2s).pdev).dev as *mut device_with_node as *mut device, c"Disable master clock\n".as_ptr());
        regmap_update_bits((*i2s).regmap, STM32_I2S_CGFR_REG, I2S_CGFR_MCKOE, 0);
    }
}

static mclk_ops: clk_ops = clk_ops {
    enable: Some(stm32_i2smclk_enable),
    disable: Some(stm32_i2smclk_disable),
    recalc_rate: Some(stm32_i2smclk_recalc_rate),
    determine_rate: Some(stm32_i2smclk_determine_rate),
    set_rate: Some(stm32_i2smclk_set_rate),
};

unsafe extern "C" fn stm32_i2s_add_mclk_provider(i2s: *mut stm32_i2s_data) -> c_int {
    let hw: *mut clk_hw;
    let mclk: *mut stm32_i2smclk_data;
    let dev = unsafe { &mut (*(*i2s).pdev).dev as *mut device_with_node as *mut device };
    let pname = unsafe { __clk_get_name((*i2s).i2sclk) };
    let mclk_name: *mut c_char;
    let mut p: *mut c_char;
    let mut s = pname as *mut c_char;
    let ret: c_int;
    let mut i: c_int = 0;

    mclk = unsafe { devm_kzalloc(dev, size_of::<stm32_i2smclk_data>(), GFP_KERNEL) as *mut stm32_i2smclk_data };
    if mclk.is_null() {
        return -ENOMEM;
    }

    mclk_name = unsafe { devm_kcalloc(dev, size_of::<c_char>(), STM32_I2S_NAME_LEN, GFP_KERNEL) as *mut c_char };
    if mclk_name.is_null() {
        return -ENOMEM;
    }

    /*
     * Forge mclk clock name from parent clock name and suffix.
     * String after "_" char is stripped in parent name.
     */
    p = mclk_name;
    unsafe {
        while *s != 0 && *s != b'_' as c_char && i < (STM32_I2S_NAME_LEN as c_int - 7) {
            *p = *s;
            p = p.add(1);
            s = s.add(1);
            i += 1;
        }
        strcat(p, c"_mclk".as_ptr());
    }

    let init = clk_init_data {
        name: mclk_name,
        ops: &mclk_ops,
        flags: 0,
        parent_names: &pname,
        num_parents: 1,
    };
    let init_ptr = unsafe { devm_kzalloc(dev, size_of::<clk_init_data>(), GFP_KERNEL) as *mut clk_init_data };
    if init_ptr.is_null() {
        return -ENOMEM;
    }
    unsafe {
        *init_ptr = init;
        (*mclk).hw.init = init_ptr;
        (*mclk).i2s_data = i2s;
    }
    hw = unsafe { &mut (*mclk).hw };

    unsafe {
        dev_dbg(dev, c"Register master clock %s\n".as_ptr(), mclk_name);
    }
    ret = unsafe { devm_clk_hw_register(&mut (*(*i2s).pdev).dev as *mut device_with_node as *mut device, hw) };
    if ret != 0 {
        unsafe {
            dev_err(dev, c"mclk register fails with error %d\n".as_ptr(), ret);
        }
        return ret;
    }
    unsafe { (*i2s).i2smclk = (*hw).clk };

    /* register mclk provider */
    unsafe { devm_of_clk_add_hw_provider(dev, &of_clk_hw_simple_get as *const c_void, hw) }
}

unsafe extern "C" fn stm32_i2s_isr(_irq: c_int, devid: *mut c_void) -> irqreturn_t {
    let i2s = devid as *mut stm32_i2s_data;
    let pdev = unsafe { (*i2s).pdev };
    let mut sr: u32 = 0;
    let mut ier: u32 = 0;
    let flags: c_ulong;
    let mut err: c_int = 0;

    unsafe {
        regmap_read((*i2s).regmap, STM32_I2S_SR_REG, &mut sr);
        regmap_read((*i2s).regmap, STM32_I2S_IER_REG, &mut ier);
    }

    flags = (sr & ier) as c_ulong;
    if flags == 0 {
        unsafe {
            dev_dbg(
                &mut (*pdev).dev as *mut device_with_node as *mut device,
                c"Spurious IRQ sr=0x%08x, ier=0x%08x\n".as_ptr(),
                sr,
                ier,
            );
        }
        return IRQ_NONE;
    }

    unsafe {
        regmap_write_bits((*i2s).regmap, STM32_I2S_IFCR_REG, I2S_IFCR_MASK, flags as u32);
    }

    if flags & I2S_SR_OVR as c_ulong != 0 {
        unsafe { dev_dbg(&mut (*pdev).dev as *mut device_with_node as *mut device, c"Overrun\n".as_ptr()) };
        err = 1;
    }

    if flags & I2S_SR_UDR as c_ulong != 0 {
        unsafe { dev_dbg(&mut (*pdev).dev as *mut device_with_node as *mut device, c"Underrun\n".as_ptr()) };
        err = 1;
    }

    if flags & I2S_SR_TIFRE as c_ulong != 0 {
        unsafe { dev_dbg(&mut (*pdev).dev as *mut device_with_node as *mut device, c"Frame error\n".as_ptr()) };
    }

    unsafe {
        spin_lock(&mut (*i2s).irq_lock);
        if err != 0 && !(*i2s).substream.is_null() {
            snd_pcm_stop_xrun((*i2s).substream);
        }
        spin_unlock(&mut (*i2s).irq_lock);
    }

    IRQ_HANDLED
}

unsafe extern "C" fn stm32_i2s_readable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        STM32_I2S_CR1_REG
        | STM32_I2S_CFG1_REG
        | STM32_I2S_CFG2_REG
        | STM32_I2S_IER_REG
        | STM32_I2S_SR_REG
        | STM32_I2S_RXDR_REG
        | STM32_I2S_CGFR_REG
        | STM32_I2S_HWCFGR_REG
        | STM32_I2S_VERR_REG
        | STM32_I2S_IPIDR_REG
        | STM32_I2S_SIDR_REG => true,
        _ => false,
    }
}

unsafe extern "C" fn stm32_i2s_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        STM32_I2S_SR_REG | STM32_I2S_RXDR_REG => true,
        _ => false,
    }
}

unsafe extern "C" fn stm32_i2s_writeable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        STM32_I2S_CR1_REG
        | STM32_I2S_CFG1_REG
        | STM32_I2S_CFG2_REG
        | STM32_I2S_IER_REG
        | STM32_I2S_IFCR_REG
        | STM32_I2S_TXDR_REG
        | STM32_I2S_CGFR_REG => true,
        _ => false,
    }
}

unsafe extern "C" fn stm32_i2s_set_dai_fmt(cpu_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let i2s = unsafe { snd_soc_dai_get_drvdata(cpu_dai) as *mut stm32_i2s_data };
    let mut cgfr: u32;
    let cgfr_mask: u32 = I2S_CGFR_I2SSTD_MASK | I2S_CGFR_CKPOL | I2S_CGFR_WSINV | I2S_CGFR_I2SCFG_MASK;

    unsafe { dev_dbg((*cpu_dai).dev, c"fmt %x\n".as_ptr(), fmt) };

    /*
     * winv = 0 : default behavior (high/low) for all standards
     * ckpol = 0 for all standards.
     */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => cgfr = I2S_CGFR_I2SSTD_SET(i2s_std::I2S_STD_I2S as u32),
        SND_SOC_DAIFMT_MSB => cgfr = I2S_CGFR_I2SSTD_SET(i2s_std::I2S_STD_LEFT_J as u32),
        SND_SOC_DAIFMT_LSB => cgfr = I2S_CGFR_I2SSTD_SET(i2s_std::I2S_STD_RIGHT_J as u32),
        SND_SOC_DAIFMT_DSP_A => cgfr = I2S_CGFR_I2SSTD_SET(i2s_std::I2S_STD_DSP as u32),
        /* DSP_B not mapped on I2S PCM long format. 1 bit offset does not fit */
        _ => {
            unsafe {
                dev_err((*cpu_dai).dev, c"Unsupported protocol %#x\n".as_ptr(), fmt & SND_SOC_DAIFMT_FORMAT_MASK);
            }
            return -EINVAL;
        }
    }

    /* DAI clock strobing */
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_NF => cgfr |= I2S_CGFR_CKPOL,
        SND_SOC_DAIFMT_NB_IF => cgfr |= I2S_CGFR_WSINV,
        SND_SOC_DAIFMT_IB_IF => {
            cgfr |= I2S_CGFR_CKPOL;
            cgfr |= I2S_CGFR_WSINV;
        }
        _ => {
            unsafe {
                dev_err((*cpu_dai).dev, c"Unsupported strobing %#x\n".as_ptr(), fmt & SND_SOC_DAIFMT_INV_MASK);
            }
            return -EINVAL;
        }
    }

    /* DAI clock master masks */
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BC_FC => unsafe { (*i2s).ms_flg = i2s_master_mode::I2S_MS_SLAVE as c_int },
        SND_SOC_DAIFMT_BP_FP => unsafe { (*i2s).ms_flg = i2s_master_mode::I2S_MS_MASTER as c_int },
        _ => {
            unsafe {
                dev_err((*cpu_dai).dev, c"Unsupported mode %#x\n".as_ptr(), fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK);
            }
            return -EINVAL;
        }
    }

    unsafe {
        (*i2s).fmt = fmt;
        regmap_update_bits((*i2s).regmap, STM32_I2S_CGFR_REG, cgfr_mask, cgfr)
    }
}

unsafe extern "C" fn stm32_i2s_set_sysclk(
    cpu_dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    dir: c_int,
) -> c_int {
    let i2s = unsafe { snd_soc_dai_get_drvdata(cpu_dai) as *mut stm32_i2s_data };
    let mut ret: c_int = 0;

    unsafe {
        dev_dbg(
            (*cpu_dai).dev,
            c"I2S MCLK frequency is %uHz. mode: %s, dir: %s\n".as_ptr(),
            freq,
            if STM32_I2S_IS_MASTER(i2s) { c"master".as_ptr() } else { c"slave".as_ptr() },
            if dir != 0 { c"output".as_ptr() } else { c"input".as_ptr() },
        );
    }

    /* MCLK generation is available only in master mode */
    if unsafe { dir == SND_SOC_CLOCK_OUT && STM32_I2S_IS_MASTER(i2s) } {
        if unsafe { (*i2s).i2smclk.is_null() } {
            unsafe { dev_dbg((*cpu_dai).dev, c"No MCLK registered\n".as_ptr()) };
            return 0;
        }

        /* Assume shutdown if requested frequency is 0Hz */
        if freq == 0 {
            /* Release mclk rate only if rate was actually set */
            unsafe {
                if (*i2s).mclk_rate != 0 {
                    clk_rate_exclusive_put((*i2s).i2smclk);
                    (*i2s).mclk_rate = 0;
                }

                if let Some(put_i2s_clk_rate) = (*i2s).put_i2s_clk_rate {
                    put_i2s_clk_rate(i2s);
                }

                return regmap_update_bits((*i2s).regmap, STM32_I2S_CGFR_REG, I2S_CGFR_MCKOE, 0);
            }
        }
        /* If master clock is used, set parent clock now */
        unsafe {
            ret = (*i2s).set_i2s_clk_rate.unwrap()(i2s, freq);
        }
        if ret != 0 {
            return ret;
        }
        ret = unsafe { clk_set_rate_exclusive((*i2s).i2smclk, freq as c_ulong) };
        if ret != 0 {
            unsafe { dev_err((*cpu_dai).dev, c"Could not set mclk rate\n".as_ptr()) };
            return ret;
        }
        ret = unsafe { regmap_update_bits((*i2s).regmap, STM32_I2S_CGFR_REG, I2S_CGFR_MCKOE, I2S_CGFR_MCKOE) };
        if ret == 0 {
            unsafe { (*i2s).mclk_rate = freq };
        }
    }

    ret
}

unsafe extern "C" fn stm32_i2s_configure_clock(
    cpu_dai: *mut snd_soc_dai,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let i2s = unsafe { snd_soc_dai_get_drvdata(cpu_dai) as *mut stm32_i2s_data };
    let i2s_clock_rate: c_ulong;
    let nb_bits: c_uint;
    let mut frame_len: c_uint;
    let rate = unsafe { params_rate(params) };
    let mut cgfr: u32 = 0;
    let mut ret: c_int;

    unsafe {
        if (*i2s).mclk_rate == 0 {
            ret = (*i2s).set_i2s_clk_rate.unwrap()(i2s, rate);
            if ret != 0 {
                return ret;
            }
        }
        i2s_clock_rate = clk_get_rate((*i2s).i2sclk);
    }

    /*
     * mckl = mclk_ratio x ws
     *   i2s mode : mclk_ratio = 256
     *   dsp mode : mclk_ratio = 128
     *
     * mclk on
     *   i2s mode : div = i2s_clk / (mclk_ratio * ws)
     *   dsp mode : div = i2s_clk / (mclk_ratio * ws)
     * mclk off
     *   i2s mode : div = i2s_clk / (nb_bits x ws)
     *   dsp mode : div = i2s_clk / (nb_bits x ws)
     */
    unsafe {
        if (*i2s).mclk_rate != 0 {
            ret = stm32_i2s_calc_clk_div(i2s, i2s_clock_rate, (*i2s).mclk_rate as c_ulong);
            if ret != 0 {
                return ret;
            }
        } else {
            frame_len = 32;
            if ((*i2s).fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_DSP_A {
                frame_len = 16;
            }

            /* master clock not enabled */
            ret = regmap_read((*i2s).regmap, STM32_I2S_CGFR_REG, &mut cgfr);
            if ret < 0 {
                return ret;
            }

            nb_bits = frame_len * (FIELD_GET(I2S_CGFR_CHLEN, cgfr) + 1);
            ret = stm32_i2s_calc_clk_div(i2s, i2s_clock_rate, (nb_bits * rate) as c_ulong);
            if ret != 0 {
                return ret;
            }
        }

        ret = stm32_i2s_set_clk_div(i2s);
    }
    if ret < 0 {
        return ret;
    }

    /* Set bitclock and frameclock to their inactive state */
    unsafe { regmap_update_bits((*i2s).regmap, STM32_I2S_CFG2_REG, I2S_CFG2_AFCNTR, I2S_CFG2_AFCNTR) }
}

unsafe extern "C" fn stm32_i2s_configure(
    cpu_dai: *mut snd_soc_dai,
    params: *mut snd_pcm_hw_params,
    _substream: *mut snd_pcm_substream,
) -> c_int {
    let i2s = unsafe { snd_soc_dai_get_drvdata(cpu_dai) as *mut stm32_i2s_data };
    let format = unsafe { params_width(params) };
    let mut cfgr: u32;
    let mut cfgr_mask: u32;
    let cfg1: u32;
    let fthlv: c_uint;
    let ret: c_int;

    match format {
        16 => {
            cfgr = I2S_CGFR_DATLEN_SET(i2s_datlen::I2S_I2SMOD_DATLEN_16 as u32);
            cfgr_mask = I2S_CGFR_DATLEN_MASK | I2S_CGFR_CHLEN;
        }
        32 => {
            cfgr = I2S_CGFR_DATLEN_SET(i2s_datlen::I2S_I2SMOD_DATLEN_32 as u32) | I2S_CGFR_CHLEN;
            cfgr_mask = I2S_CGFR_DATLEN_MASK | I2S_CGFR_CHLEN;
        }
        _ => {
            unsafe { dev_err((*cpu_dai).dev, c"Unexpected format %d".as_ptr(), format) };
            return -EINVAL;
        }
    }

    if unsafe { STM32_I2S_IS_SLAVE(i2s) } {
        cfgr |= I2S_CGFR_I2SCFG_SET(i2s_mode::I2S_I2SMOD_FD_SLAVE as u32);

        /* As data length is either 16 or 32 bits, fixch always set */
        cfgr |= I2S_CGFR_FIXCH;
        cfgr_mask |= I2S_CGFR_FIXCH;
    } else {
        cfgr |= I2S_CGFR_I2SCFG_SET(i2s_mode::I2S_I2SMOD_FD_MASTER as u32);
    }
    cfgr_mask |= I2S_CGFR_I2SCFG_MASK;

    ret = unsafe { regmap_update_bits((*i2s).regmap, STM32_I2S_CGFR_REG, cfgr_mask, cfgr) };
    if ret < 0 {
        return ret;
    }

    fthlv = STM32_I2S_FIFO_SIZE * i2s_fifo_th::I2S_FIFO_TH_ONE_QUARTER as u32 / 4;
    cfg1 = I2S_CFG1_FTHVL_SET(fthlv - 1);

    unsafe { regmap_update_bits((*i2s).regmap, STM32_I2S_CFG1_REG, I2S_CFG1_FTHVL_MASK, cfg1) }
}

unsafe extern "C" fn stm32_i2s_startup(
    substream: *mut snd_pcm_substream,
    cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let i2s = unsafe { snd_soc_dai_get_drvdata(cpu_dai) as *mut stm32_i2s_data };
    let ret: c_int;

    unsafe {
        let flags = spin_lock_irqsave(&mut (*i2s).irq_lock);
        (*i2s).substream = substream;
        spin_unlock_irqrestore(&mut (*i2s).irq_lock, flags);
    }

    unsafe {
        if ((*i2s).fmt & SND_SOC_DAIFMT_FORMAT_MASK) != SND_SOC_DAIFMT_DSP_A {
            snd_pcm_hw_constraint_single((*substream).runtime, SNDRV_PCM_HW_PARAM_CHANNELS, 2);
        }

        ret = clk_prepare_enable((*i2s).i2sclk);
    }
    if ret < 0 {
        unsafe { dev_err((*cpu_dai).dev, c"Failed to enable clock: %d\n".as_ptr(), ret) };
        return ret;
    }

    unsafe { regmap_write_bits((*i2s).regmap, STM32_I2S_IFCR_REG, I2S_IFCR_MASK, I2S_IFCR_MASK) }
}

unsafe extern "C" fn stm32_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let i2s = unsafe { snd_soc_dai_get_drvdata(cpu_dai) as *mut stm32_i2s_data };
    let mut ret: c_int;

    ret = unsafe { stm32_i2s_configure(cpu_dai, params, substream) };
    if ret < 0 {
        unsafe { dev_err((*cpu_dai).dev, c"Configuration returned error %d\n".as_ptr(), ret) };
        return ret;
    }

    if unsafe { STM32_I2S_IS_MASTER(i2s) } {
        ret = unsafe { stm32_i2s_configure_clock(cpu_dai, params) };
    }

    ret
}

unsafe extern "C" fn stm32_i2s_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let i2s = unsafe { snd_soc_dai_get_drvdata(cpu_dai) as *mut stm32_i2s_data };
    let playback_flg = unsafe { (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK };
    let cfg1_mask: u32;
    let mut ier: u32;
    let ret: c_int;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            /* Enable i2s */
            unsafe {
                dev_dbg((*cpu_dai).dev, c"start I2S %s\n".as_ptr(), snd_pcm_direction_name((*substream).stream));
            }

            cfg1_mask = I2S_CFG1_RXDMAEN | I2S_CFG1_TXDMAEN;
            unsafe {
                regmap_update_bits((*i2s).regmap, STM32_I2S_CFG1_REG, cfg1_mask, cfg1_mask);

                ret = regmap_update_bits((*i2s).regmap, STM32_I2S_CR1_REG, I2S_CR1_SPE, I2S_CR1_SPE);
            }
            if ret < 0 {
                unsafe { dev_err((*cpu_dai).dev, c"Error %d enabling I2S\n".as_ptr(), ret) };
                return ret;
            }

            let start_ret = unsafe { regmap_write_bits((*i2s).regmap, STM32_I2S_CR1_REG, I2S_CR1_CSTART, I2S_CR1_CSTART) };
            if start_ret < 0 {
                unsafe { dev_err((*cpu_dai).dev, c"Error %d starting I2S\n".as_ptr(), start_ret) };
                return start_ret;
            }

            unsafe {
                regmap_write_bits((*i2s).regmap, STM32_I2S_IFCR_REG, I2S_IFCR_MASK, I2S_IFCR_MASK);

                spin_lock(&mut (*i2s).lock_fd);
                (*i2s).refcount += 1;
                if playback_flg {
                    ier = I2S_IER_UDRIE;
                } else {
                    ier = I2S_IER_OVRIE;

                    if STM32_I2S_IS_MASTER(i2s) && (*i2s).refcount == 1 {
                        /* dummy write to gate bus clocks */
                        regmap_write((*i2s).regmap, STM32_I2S_TXDR_REG, 0);
                    }
                }
                spin_unlock(&mut (*i2s).lock_fd);

                if STM32_I2S_IS_SLAVE(i2s) {
                    ier |= I2S_IER_TIFREIE;
                }

                regmap_update_bits((*i2s).regmap, STM32_I2S_IER_REG, ier, ier);
            }
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            unsafe {
                dev_dbg((*cpu_dai).dev, c"stop I2S %s\n".as_ptr(), snd_pcm_direction_name((*substream).stream));
            }

            unsafe {
                if playback_flg {
                    regmap_update_bits((*i2s).regmap, STM32_I2S_IER_REG, I2S_IER_UDRIE, !I2S_IER_UDRIE);
                } else {
                    regmap_update_bits((*i2s).regmap, STM32_I2S_IER_REG, I2S_IER_OVRIE, !I2S_IER_OVRIE);
                }

                spin_lock(&mut (*i2s).lock_fd);
                (*i2s).refcount -= 1;
                if (*i2s).refcount != 0 {
                    spin_unlock(&mut (*i2s).lock_fd);
                    return 0;
                }

                let disable_ret = regmap_update_bits((*i2s).regmap, STM32_I2S_CR1_REG, I2S_CR1_SPE, 0);
                if disable_ret < 0 {
                    spin_unlock(&mut (*i2s).lock_fd);
                    dev_err((*cpu_dai).dev, c"Error %d disabling I2S\n".as_ptr(), disable_ret);
                    return disable_ret;
                }
                spin_unlock(&mut (*i2s).lock_fd);

                cfg1_mask = I2S_CFG1_RXDMAEN | I2S_CFG1_TXDMAEN;
                regmap_update_bits((*i2s).regmap, STM32_I2S_CFG1_REG, cfg1_mask, 0);
            }
        }
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn stm32_i2s_shutdown(
    _substream: *mut snd_pcm_substream,
    cpu_dai: *mut snd_soc_dai,
) {
    let i2s = unsafe { snd_soc_dai_get_drvdata(cpu_dai) as *mut stm32_i2s_data };

    unsafe {
        clk_disable_unprepare((*i2s).i2sclk);

        /*
         * Release kernel clock if following conditions are fulfilled
         * - Master clock is not used. Kernel clock won't be released trough sysclk
         * - Put handler is defined. Involve that clock is managed exclusively
         */
        if (*i2s).i2smclk.is_null() {
            if let Some(put_i2s_clk_rate) = (*i2s).put_i2s_clk_rate {
                put_i2s_clk_rate(i2s);
            }
        }

        let flags = spin_lock_irqsave(&mut (*i2s).irq_lock);
        (*i2s).substream = ptr::null_mut();
        spin_unlock_irqrestore(&mut (*i2s).irq_lock, flags);
    }
}

unsafe extern "C" fn stm32_i2s_dai_probe(cpu_dai: *mut snd_soc_dai) -> c_int {
    let i2s = unsafe { dev_get_drvdata((*cpu_dai).dev) as *mut stm32_i2s_data };
    let dma_data_tx = unsafe { &mut (*i2s).dma_data_tx as *mut snd_dmaengine_dai_dma_data };
    let dma_data_rx = unsafe { &mut (*i2s).dma_data_rx as *mut snd_dmaengine_dai_dma_data };

    /* Buswidth will be set by framework */
    unsafe {
        (*dma_data_tx).addr_width = DMA_SLAVE_BUSWIDTH_UNDEFINED;
        (*dma_data_tx).addr = (*i2s).phys_addr + STM32_I2S_TXDR_REG as dma_addr_t;
        (*dma_data_tx).maxburst = 1;
        (*dma_data_rx).addr_width = DMA_SLAVE_BUSWIDTH_UNDEFINED;
        (*dma_data_rx).addr = (*i2s).phys_addr + STM32_I2S_RXDR_REG as dma_addr_t;
        (*dma_data_rx).maxburst = 1;

        snd_soc_dai_init_dma_data(cpu_dai, dma_data_tx, dma_data_rx);
    }

    0
}

static stm32_h7_i2s_regmap_conf: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: STM32_I2S_SIDR_REG,
    readable_reg: Some(stm32_i2s_readable_reg),
    volatile_reg: Some(stm32_i2s_volatile_reg),
    writeable_reg: Some(stm32_i2s_writeable_reg),
    num_reg_defaults_raw: STM32_I2S_SIDR_REG / size_of::<u32>() as u32 + 1,
    fast_io: true,
    cache_type: REGCACHE_FLAT,
};

static stm32_i2s_pcm_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(stm32_i2s_dai_probe),
    set_sysclk: Some(stm32_i2s_set_sysclk),
    set_fmt: Some(stm32_i2s_set_dai_fmt),
    startup: Some(stm32_i2s_startup),
    hw_params: Some(stm32_i2s_hw_params),
    trigger: Some(stm32_i2s_trigger),
    shutdown: Some(stm32_i2s_shutdown),
};

static stm32_i2s_pcm_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP,
    buffer_bytes_max: 8 * PAGE_SIZE,
    period_bytes_min: 1024,
    period_bytes_max: 4 * PAGE_SIZE,
    periods_min: 2,
    periods_max: 8,
};

static stm32_i2s_pcm_config: snd_dmaengine_pcm_config = snd_dmaengine_pcm_config {
    pcm_hardware: &stm32_i2s_pcm_hw,
    prepare_slave_config: Some(snd_dmaengine_pcm_prepare_slave_config),
    prealloc_buffer_size: PAGE_SIZE * 8,
};

static stm32_i2s_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"stm32-i2s".as_ptr(),
    legacy_dai_naming: 1,
};

unsafe extern "C" fn stm32_i2s_dai_init(stream: *mut snd_soc_pcm_stream, stream_name: *mut c_char) {
    unsafe {
        (*stream).stream_name = stream_name;
        (*stream).channels_min = 1;
        (*stream).channels_max = 2;
        (*stream).rates = SNDRV_PCM_RATE_8000_192000;
        (*stream).formats = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE;
    }
}

unsafe extern "C" fn stm32_i2s_dais_init(
    pdev: *mut platform_device,
    i2s: *mut stm32_i2s_data,
) -> c_int {
    let dai_ptr: *mut snd_soc_dai_driver;

    dai_ptr = unsafe {
        devm_kzalloc(
            &mut (*pdev).dev as *mut device_with_node as *mut device,
            size_of::<snd_soc_dai_driver>(),
            GFP_KERNEL,
        ) as *mut snd_soc_dai_driver
    };
    if dai_ptr.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*dai_ptr).ops = &stm32_i2s_pcm_dai_ops;
        (*dai_ptr).id = 1;
        stm32_i2s_dai_init(&mut (*dai_ptr).playback, c"playback".as_ptr() as *mut c_char);
        stm32_i2s_dai_init(&mut (*dai_ptr).capture, c"capture".as_ptr() as *mut c_char);
        (*i2s).dai_drv = dai_ptr;
    }

    0
}

static stm32_i2s_conf_h7: stm32_i2s_conf = stm32_i2s_conf {
    regmap_conf: &stm32_h7_i2s_regmap_conf,
    get_i2s_clk_parent: Some(stm32_i2s_get_parent_clk),
};

static stm32_i2s_conf_mp25: stm32_i2s_conf = stm32_i2s_conf {
    regmap_conf: &stm32_h7_i2s_regmap_conf,
    get_i2s_clk_parent: None,
};

static stm32_i2s_ids: [of_device_id; 3] = [
    of_device_id {
        compatible: c"st,stm32h7-i2s".as_ptr(),
        data: &stm32_i2s_conf_h7 as *const _ as *const c_void,
    },
    of_device_id {
        compatible: c"st,stm32mp25-i2s".as_ptr(),
        data: &stm32_i2s_conf_mp25 as *const _ as *const c_void,
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];

unsafe extern "C" fn stm32_i2s_parse_dt(
    pdev: *mut platform_device,
    i2s: *mut stm32_i2s_data,
) -> c_int {
    let np = unsafe { (*pdev).dev.of_node };
    let rst: *mut reset_control;
    let mut res: *mut resource = ptr::null_mut();
    let irq: c_int;
    let mut ret: c_int;

    if np.is_null() {
        return -ENODEV;
    }

    unsafe { (*i2s).conf = device_get_match_data(&mut (*pdev).dev as *mut device_with_node as *mut device) as *const stm32_i2s_conf };
    if unsafe { (*i2s).conf.is_null() } {
        return -EINVAL;
    }

    unsafe { (*i2s).base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res) };
    if unsafe { IS_ERR((*i2s).base) } {
        return unsafe { PTR_ERR((*i2s).base) as c_int };
    }

    unsafe { (*i2s).phys_addr = (*res).start };

    /* Get clocks */
    unsafe { (*i2s).pclk = devm_clk_get(&mut (*pdev).dev as *mut device_with_node as *mut device, c"pclk".as_ptr()) };
    if unsafe { IS_ERR((*i2s).pclk) } {
        return unsafe {
            dev_err_probe(
                &mut (*pdev).dev as *mut device_with_node as *mut device,
                PTR_ERR((*i2s).pclk),
                c"Could not get pclk\n".as_ptr(),
            )
        };
    }

    unsafe { (*i2s).i2sclk = devm_clk_get(&mut (*pdev).dev as *mut device_with_node as *mut device, c"i2sclk".as_ptr()) };
    if unsafe { IS_ERR((*i2s).i2sclk) } {
        return unsafe {
            dev_err_probe(
                &mut (*pdev).dev as *mut device_with_node as *mut device,
                PTR_ERR((*i2s).i2sclk),
                c"Could not get i2sclk\n".as_ptr(),
            )
        };
    }

    unsafe {
        if (*(*i2s).conf).get_i2s_clk_parent.is_some() {
            (*i2s).set_i2s_clk_rate = Some(stm32_i2s_set_parent_clock);
        } else {
            (*i2s).set_i2s_clk_rate = Some(stm32_i2s_set_parent_rate);
            (*i2s).put_i2s_clk_rate = Some(stm32_i2s_put_parent_rate);
        }

        if let Some(get_i2s_clk_parent) = (*(*i2s).conf).get_i2s_clk_parent {
            ret = get_i2s_clk_parent(i2s);
            if ret != 0 {
                return ret;
            }
        }
    }

    /* Register mclk provider if requested */
    if unsafe { of_property_present(np, c"#clock-cells".as_ptr()) } {
        ret = unsafe { stm32_i2s_add_mclk_provider(i2s) };
        if ret < 0 {
            return ret;
        }
    }

    /* Get irqs */
    irq = unsafe { platform_get_irq(pdev, 0) };
    if irq < 0 {
        return irq;
    }

    ret = unsafe {
        devm_request_irq(
            &mut (*pdev).dev as *mut device_with_node as *mut device,
            irq as c_uint,
            stm32_i2s_isr,
            0,
            dev_name(&mut (*pdev).dev as *mut device_with_node as *mut device),
            i2s as *mut c_void,
        )
    };
    if ret != 0 {
        return ret;
    }

    /* Reset */
    rst = unsafe {
        devm_reset_control_get_optional_exclusive(
            &mut (*pdev).dev as *mut device_with_node as *mut device,
            ptr::null(),
        )
    };
    if unsafe { IS_ERR(rst) } {
        return unsafe {
            dev_err_probe(
                &mut (*pdev).dev as *mut device_with_node as *mut device,
                PTR_ERR(rst),
                c"Reset controller error\n".as_ptr(),
            )
        };
    }

    unsafe {
        reset_control_assert(rst);
        udelay(2);
        reset_control_deassert(rst);
    }

    0
}

unsafe extern "C" fn stm32_i2s_remove(pdev: *mut platform_device) {
    unsafe {
        snd_dmaengine_pcm_unregister(&mut (*pdev).dev as *mut device_with_node as *mut device);
        snd_soc_unregister_component(&mut (*pdev).dev as *mut device_with_node as *mut device);
        pm_runtime_disable(&mut (*pdev).dev as *mut device_with_node as *mut device);
    }
}

unsafe extern "C" fn stm32_i2s_probe(pdev: *mut platform_device) -> c_int {
    let i2s: *mut stm32_i2s_data;
    let mut val: u32 = 0;
    let mut ret: c_int;

    i2s = unsafe {
        devm_kzalloc(
            &mut (*pdev).dev as *mut device_with_node as *mut device,
            size_of::<stm32_i2s_data>(),
            GFP_KERNEL,
        ) as *mut stm32_i2s_data
    };
    if i2s.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*i2s).pdev = pdev;
        (*i2s).ms_flg = i2s_master_mode::I2S_MS_NOT_SET as c_int;
        spin_lock_init(&mut (*i2s).lock_fd);
        spin_lock_init(&mut (*i2s).irq_lock);
        platform_set_drvdata(pdev, i2s as *mut c_void);
    }

    ret = unsafe { stm32_i2s_parse_dt(pdev, i2s) };
    if ret != 0 {
        return ret;
    }

    ret = unsafe { stm32_i2s_dais_init(pdev, i2s) };
    if ret != 0 {
        return ret;
    }

    unsafe {
        (*i2s).regmap = devm_regmap_init_mmio_clk(
            &mut (*pdev).dev as *mut device_with_node as *mut device,
            c"pclk".as_ptr(),
            (*i2s).base,
            (*(*i2s).conf).regmap_conf,
        );
    }
    if unsafe { IS_ERR((*i2s).regmap) } {
        return unsafe {
            dev_err_probe(
                &mut (*pdev).dev as *mut device_with_node as *mut device,
                PTR_ERR((*i2s).regmap),
                c"Regmap init error\n".as_ptr(),
            )
        };
    }

    ret = unsafe {
        snd_dmaengine_pcm_register(
            &mut (*pdev).dev as *mut device_with_node as *mut device,
            &stm32_i2s_pcm_config,
            0,
        )
    };
    if ret != 0 {
        return ret;
    }

    ret = unsafe {
        snd_soc_register_component(
            &mut (*pdev).dev as *mut device_with_node as *mut device,
            &stm32_i2s_component,
            (*i2s).dai_drv,
            1,
        )
    };
    if ret != 0 {
        unsafe { snd_dmaengine_pcm_unregister(&mut (*pdev).dev as *mut device_with_node as *mut device) };
        return ret;
    }

    /* Set SPI/I2S in i2s mode */
    ret = unsafe { regmap_update_bits((*i2s).regmap, STM32_I2S_CGFR_REG, I2S_CGFR_I2SMOD, I2S_CGFR_I2SMOD) };
    if ret != 0 {
        unsafe { stm32_i2s_remove(pdev) };
        return ret;
    }

    ret = unsafe { regmap_read((*i2s).regmap, STM32_I2S_IPIDR_REG, &mut val) };
    if ret != 0 {
        unsafe { stm32_i2s_remove(pdev) };
        return ret;
    }

    if val == I2S_IPIDR_NUMBER {
        ret = unsafe { regmap_read((*i2s).regmap, STM32_I2S_HWCFGR_REG, &mut val) };
        if ret != 0 {
            unsafe { stm32_i2s_remove(pdev) };
            return ret;
        }

        if FIELD_GET(I2S_HWCFGR_I2S_SUPPORT_MASK, val) == 0 {
            unsafe {
                dev_err(
                    &mut (*pdev).dev as *mut device_with_node as *mut device,
                    c"Device does not support i2s mode\n".as_ptr(),
                );
            }
            ret = -EPERM;
            unsafe { stm32_i2s_remove(pdev) };
            return ret;
        }

        ret = unsafe { regmap_read((*i2s).regmap, STM32_I2S_VERR_REG, &mut val) };
        if ret != 0 {
            unsafe { stm32_i2s_remove(pdev) };
            return ret;
        }

        unsafe {
            dev_dbg(
                &mut (*pdev).dev as *mut device_with_node as *mut device,
                c"I2S version: %lu.%lu registered\n".as_ptr(),
                FIELD_GET(I2S_VERR_MAJ_MASK, val) as c_ulong,
                FIELD_GET(I2S_VERR_MIN_MASK, val) as c_ulong,
            );
        }
    }

    unsafe { pm_runtime_enable(&mut (*pdev).dev as *mut device_with_node as *mut device) };

    ret
}

/* MODULE_DEVICE_TABLE(of, stm32_i2s_ids); */

unsafe extern "C" fn stm32_i2s_suspend(dev: *mut device) -> c_int {
    let i2s = unsafe { dev_get_drvdata(dev) as *mut stm32_i2s_data };

    unsafe {
        regcache_cache_only((*i2s).regmap, true);
        regcache_mark_dirty((*i2s).regmap);
    }

    0
}

unsafe extern "C" fn stm32_i2s_resume(dev: *mut device) -> c_int {
    let i2s = unsafe { dev_get_drvdata(dev) as *mut stm32_i2s_data };

    unsafe {
        regcache_cache_only((*i2s).regmap, false);
        regcache_sync((*i2s).regmap)
    }
}

static stm32_i2s_pm_ops: dev_pm_ops = dev_pm_ops {
    /* SYSTEM_SLEEP_PM_OPS(stm32_i2s_suspend, stm32_i2s_resume) */
    suspend: Some(stm32_i2s_suspend),
    resume: Some(stm32_i2s_resume),
};

static mut stm32_i2s_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"st,stm32-i2s".as_ptr(),
        of_match_table: stm32_i2s_ids.as_ptr(),
        pm: &stm32_i2s_pm_ops,
    },
    probe: Some(stm32_i2s_probe),
    remove: Some(stm32_i2s_remove),
};

/* module_platform_driver(stm32_i2s_driver); */

/* MODULE_DESCRIPTION("STM32 Soc i2s Interface"); */
/* MODULE_AUTHOR("Olivier Moysan, <olivier.moysan@st.com>"); */
/* MODULE_ALIAS("platform:stm32-i2s"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
