// SPDX-License-Identifier: GPL-2.0-only
/*
 * STM32 ALSA SoC Digital Audio Interface (SAI) driver.
 *
 * Copyright (C) 2016, STMicroelectronics - All Rights Reserved
 * Author(s): Olivier Moysan <olivier.moysan@st.com> for STMicroelectronics.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u32 = u32;
type u64 = u64;
type dma_addr_t = u64;
type ssize_t = isize;
type irqreturn_t = c_int;
type snd_pcm_state_t = c_int;

#[repr(C)] pub struct device { pub of_node: *mut device_node, pub parent: *mut device }
#[repr(C)] pub struct platform_device { pub dev: device, pub name: *const c_char }
#[repr(C)] pub struct regmap;
#[repr(C)] pub struct regmap_config {
    pub reg_bits: c_int,
    pub reg_stride: c_int,
    pub val_bits: c_int,
    pub max_register: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub fast_io: bool,
    pub cache_type: c_int,
}
type c_uint = u32;
#[repr(C)] pub struct snd_dmaengine_dai_dma_data { pub addr: dma_addr_t, pub maxburst: c_uint, pub addr_width: c_int }
#[repr(C)] pub struct snd_soc_dai_driver { pub id: c_int, pub playback: snd_soc_pcm_stream, pub capture: snd_soc_pcm_stream, pub ops: *const snd_soc_dai_ops, pub name: *const c_char }
#[repr(C)] pub struct snd_soc_pcm_stream { pub channels_min: c_uint, pub channels_max: c_uint, pub rate_min: c_uint, pub rate_max: c_uint, pub rates: c_uint, pub formats: u64 }
#[repr(C)] pub struct snd_soc_dai { pub dev: *mut device, pub component: *mut snd_soc_component }
#[repr(C)] pub struct snd_soc_component { pub card: *mut snd_soc_card }
#[repr(C)] pub struct snd_soc_card;
#[repr(C)] pub struct snd_pcm_substream { pub runtime: *mut snd_pcm_runtime }
#[repr(C)] pub struct snd_pcm_runtime { pub rate: c_uint, pub dma_area: *mut u8, pub dma_bytes: c_ulong, pub channels: c_uint }
#[repr(C)] pub struct snd_pcm_hw_params;
#[repr(C)] pub struct snd_soc_pcm_runtime { pub pcm: *mut snd_pcm }
#[repr(C)] pub struct snd_pcm { pub device: c_int, pub card: *mut snd_card }
#[repr(C)] pub struct snd_card;
#[repr(C)] pub struct stm32_sai_data { pub pdev: *mut platform_device, pub pclk: *mut clk, pub clk_x8k: *mut clk, pub clk_x11k: *mut clk, pub irq: c_uint, pub conf: stm32_sai_conf, pub set_sync: Option<unsafe extern "C" fn(*mut stm32_sai_data, *mut device_node, c_int, c_int) -> c_int> }
#[repr(C)] pub struct stm32_sai_conf { pub version: c_int, pub has_spdif_pdm: bool, pub fifo_size: c_int, pub no_dma_burst: bool, pub get_sai_ck_parent: bool }
#[repr(C)] pub struct device_node { pub full_name: *const c_char }
#[repr(C)] pub struct clk;
#[repr(C)] pub struct clk_hw { pub clk: *mut clk, pub init: *const c_void }
#[repr(C)] pub struct snd_aes_iec958 { pub status: [u8; 24] }
#[repr(C)] pub struct mutex;
#[repr(C)] pub struct spinlock_t;
#[repr(C)] pub struct snd_kcontrol;
#[repr(C)] pub struct snd_ctl_elem_info { pub type_: c_uint, pub count: c_uint }
#[repr(C)] pub union snd_ctl_elem_value_value { pub iec958: snd_ctl_elem_value_iec958 }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_iec958 { pub status: [u8; 24] }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_value }
#[repr(C)] pub struct snd_kcontrol_new { pub access: c_uint, pub iface: c_uint, pub name: *const c_char, pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>, pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>, pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>, pub device: c_int }
#[repr(C)] pub struct clk_rate_request { pub rate: c_ulong, pub best_parent_rate: c_ulong }
#[repr(C)] pub struct clk_ops { pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>, pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>, pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong) -> c_ulong>, pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> c_int>, pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong, c_ulong) -> c_int> }
#[repr(C)] pub struct resource { pub start: dma_addr_t }
#[repr(C)] pub struct of_phandle_args { pub np: *mut device_node, pub args: [c_int; 8] }
#[repr(C)] pub struct snd_soc_dai_ops { pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>, pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>, pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>, pub set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, u32, u32, c_int, c_int) -> c_int>, pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>, pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>, pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>, pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>, pub pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_soc_dai) -> c_int> }
#[repr(C)] pub struct snd_pcm_hardware { pub info: c_uint, pub buffer_bytes_max: c_ulong, pub period_bytes_min: c_ulong, pub period_bytes_max: c_ulong, pub periods_min: c_uint, pub periods_max: c_uint }
#[repr(C)] pub struct snd_dmaengine_pcm_config { pub pcm_hardware: *const snd_pcm_hardware, pub prepare_slave_config: *const c_void, pub process: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, c_ulong, c_ulong) -> c_int> }
#[repr(C)] pub struct snd_soc_component_driver { pub name: *const c_char, pub legacy_dai_naming: c_int }
#[repr(C)] pub struct of_device_id { pub compatible: *const c_char, pub data: *const c_void }
#[repr(C)] pub struct dev_pm_ops;
#[repr(C)] pub struct platform_driver;

const SAI_FREE_PROTOCOL: c_int = 0x0;
const SAI_SPDIF_PROTOCOL: c_int = 0x1;
const SAI_SLOT_SIZE_AUTO: c_int = 0x0;
const SAI_SLOT_SIZE_16: c_int = 0x1;
const SAI_SLOT_SIZE_32: c_int = 0x2;
const SAI_DATASIZE_8: c_int = 0x2;
const SAI_DATASIZE_10: c_int = 0x3;
const SAI_DATASIZE_16: c_int = 0x4;
const SAI_DATASIZE_20: c_int = 0x5;
const SAI_DATASIZE_24: c_int = 0x6;
const SAI_DATASIZE_32: c_int = 0x7;
const STM_SAI_DAI_NAME_SIZE: c_int = 15;
const STM_SAI_A_ID: c_uint = 0x0;
const STM_SAI_B_ID: c_uint = 0x1;
const SAI_SYNC_NONE: c_int = 0x0;
const SAI_SYNC_INTERNAL: c_int = 0x1;
const SAI_SYNC_EXTERNAL: c_int = 0x2;
const SAI_IEC60958_BLOCK_FRAMES: c_uint = 192;
const SAI_IEC60958_STATUS_BYTES: c_int = 24;
const SAI_MCLK_NAME_LEN: usize = 32;
const SAI_RATE_11K: c_uint = 11025;
const SAI_MAX_SAMPLE_RATE_8K: c_uint = 192000;
const SAI_MAX_SAMPLE_RATE_11K: c_uint = 176400;
const SAI_CK_RATE_TOLERANCE: u64 = 1000; /* ppm */

#[repr(C)]
pub struct stm32_sai_sub_data {
    pub pdev: *mut platform_device,
    pub regmap: *mut regmap,
    pub regmap_config: *const regmap_config,
    pub dma_params: snd_dmaengine_dai_dma_data,
    pub cpu_dai_drv: snd_soc_dai_driver,
    pub cpu_dai: *mut snd_soc_dai,
    pub substream: *mut snd_pcm_substream,
    pub pdata: *mut stm32_sai_data,
    pub np_sync_provider: *mut device_node,
    pub sai_ck: *mut clk,
    pub sai_mclk: *mut clk,
    pub phys_addr: dma_addr_t,
    pub mclk_rate: c_uint,
    pub id: c_uint,
    pub dir: c_int,
    pub master: bool,
    pub spdif: bool,
    pub sai_ck_used: bool,
    pub fmt: c_int,
    pub sync: c_int,
    pub synco: c_int,
    pub synci: c_int,
    pub fs_length: c_int,
    pub slots: c_int,
    pub slot_width: c_int,
    pub slot_mask: c_int,
    pub data_size: c_int,
    pub spdif_frm_cnt: c_uint,
    pub iec958: snd_aes_iec958,
    pub ctrl_lock: mutex,
    pub irq_lock: spinlock_t,
    pub set_sai_ck_rate: Option<unsafe extern "C" fn(*mut stm32_sai_sub_data, c_uint) -> c_int>,
    pub put_sai_ck_rate: Option<unsafe extern "C" fn(*mut stm32_sai_sub_data)>,
}

#[repr(C)]
pub enum stm32_sai_fifo_th {
    STM_SAI_FIFO_TH_EMPTY,
    STM_SAI_FIFO_TH_QUARTER,
    STM_SAI_FIFO_TH_HALF,
    STM_SAI_FIFO_TH_3_QUARTER,
    STM_SAI_FIFO_TH_FULL,
}

unsafe extern "C" {
    static snd_dmaengine_pcm_prepare_slave_config: c_void;
    fn clk_enable(clk: *mut clk) -> c_int;
    fn clk_disable(clk: *mut clk);
    fn clk_prepare(clk: *mut clk) -> c_int;
    fn clk_unprepare(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_get_rate(clk: *mut clk) -> c_uint;
    fn clk_round_rate(clk: *mut clk, rate: c_uint) -> c_uint;
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> c_int;
    fn clk_rate_exclusive_get(clk: *mut clk);
    fn clk_rate_exclusive_put(clk: *mut clk);
    fn clk_set_rate_exclusive(clk: *mut clk, rate: c_uint) -> c_int;
    fn __clk_get_name(clk: *mut clk) -> *const c_char;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_write_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn snd_soc_dai_get_drvdata(cpu_dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_card_is_instantiated(card: *mut snd_soc_card) -> bool;
    fn snd_pcm_hw_constraint_mask64(runtime: *mut snd_pcm_runtime, param: c_int, mask: u64) -> c_int;
    fn snd_pcm_hw_constraint_single(runtime: *mut snd_pcm_runtime, param: c_int, val: c_uint) -> c_int;
    fn snd_pcm_stop_xrun(substream: *mut snd_pcm_substream);
    fn snd_soc_dai_init_dma_data(cpu_dai: *mut snd_soc_dai, playback: *mut snd_dmaengine_dai_dma_data, capture: *mut snd_dmaengine_dai_dma_data);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(knew: *mut snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn bytes_to_samples(runtime: *mut snd_pcm_runtime, bytes: c_ulong) -> ssize_t;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_get_and_ioremap_resource(pdev: *mut platform_device, index: c_uint, res: *mut *mut resource) -> *mut c_void;
    fn devm_regmap_init_mmio(dev: *mut device, base: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_clk_get_optional(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_request_irq(dev: *mut device, irq: c_uint, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_uint, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn devm_clk_hw_register(dev: *mut device, hw: *mut clk_hw) -> c_int;
    fn devm_of_clk_add_hw_provider(dev: *mut device, get: *const c_void, hw: *mut clk_hw) -> c_int;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn of_property_match_string(np: *mut device_node, propname: *const c_char, string: *const c_char) -> c_int;
    fn of_property_present(np: *mut device_node, propname: *const c_char) -> bool;
    fn of_parse_phandle_with_fixed_args(np: *mut device_node, list: *const c_char, cells: c_int, index: c_int, out_args: *mut of_phandle_args) -> c_int;
    fn of_get_parent(np: *mut device_node) -> *mut device_node;
    fn of_node_put(np: *mut device_node);
    fn snd_dmaengine_pcm_register(dev: *mut device, config: *const snd_dmaengine_pcm_config, flags: c_uint) -> c_int;
    fn snd_dmaengine_pcm_unregister(dev: *mut device);
    fn snd_soc_register_component(dev: *mut device, component_driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn snd_soc_unregister_component(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn mutex_init(mutex: *mut mutex);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char;
}

unsafe extern "C" {
    fn STM_SAI_IS_F4(pdata: *mut stm32_sai_data) -> bool;
    fn SAI_XCR1_MCKDIV_MAX(version: c_int) -> c_int;
    fn SAI_XCR1_MCKDIV_WIDTH(version: c_int) -> c_int;
    fn SAI_XCR1_MCKDIV_MASK(width: c_int) -> c_int;
    fn SAI_XCR1_MCKDIV_SET(div: c_int) -> c_int;
    fn SAI_XCR1_PRTCFG_SET(v: c_int) -> c_int;
    fn SAI_XSLOTR_SLOTSZ_SET(v: c_int) -> c_int;
    fn SAI_XSLOTR_NBSLOT_SET(v: c_int) -> c_int;
    fn SAI_XSLOTR_SLOTEN_SET(v: c_int) -> c_int;
    fn SAI_XFRCR_FRL_SET(v: c_int) -> c_int;
    fn SAI_XFRCR_FSALL_SET(v: c_int) -> c_int;
    fn SAI_XSLOTR_FBOFF_SET(v: c_int) -> c_int;
    fn SAI_XCR1_DS_SET(v: c_int) -> c_int;
    fn SAI_XCR1_SYNCEN_SET(v: c_int) -> c_int;
}

unsafe fn STM_SAI_IS_PLAYBACK(ip: *mut stm32_sai_sub_data) -> bool { (*ip).dir == SNDRV_PCM_STREAM_PLAYBACK }
unsafe fn STM_SAI_IS_CAPTURE(ip: *mut stm32_sai_sub_data) -> bool { (*ip).dir == SNDRV_PCM_STREAM_CAPTURE }
unsafe fn STM_SAI_IS_SUB_A(x: *mut stm32_sai_sub_data) -> bool { (*x).id == STM_SAI_A_ID }
unsafe fn STM_SAI_PROTOCOL_IS_SPDIF(ip: *mut stm32_sai_sub_data) -> bool { (*ip).spdif }
unsafe fn STM_SAI_HAS_SPDIF(x: *mut stm32_sai_sub_data) -> bool { (*(*x).pdata).conf.has_spdif_pdm }
unsafe fn STM_SAI_HAS_PDM(x: *mut stm32_sai_sub_data) -> bool { (*(*x).pdata).conf.has_spdif_pdm }
unsafe fn STM_SAI_HAS_EXT_SYNC(x: *mut stm32_sai_sub_data) -> bool { !STM_SAI_IS_F4((*x).pdata) }

/* Constants supplied by Linux/ALSA/stm32_sai.h in the original file. */
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const ENOENT: c_int = 2;
const EBUSY: c_int = 16;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_FLAT: c_int = 1;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_SHARED: c_uint = 0x80;
const PAGE_SIZE: c_ulong = 4096;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SND_SOC_CLOCK_OUT: c_int = 1;
const SNDRV_PCM_STATE_RUNNING: snd_pcm_state_t = 3;
const SNDRV_PCM_STATE_XRUN: snd_pcm_state_t = 4;
const SNDRV_PCM_STATE_DISCONNECTED: snd_pcm_state_t = 8;
const SNDRV_CTL_ELEM_TYPE_IEC958: c_uint = 4;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 0x3;
const SNDRV_CTL_ELEM_ACCESS_VOLATILE: c_uint = 0x80;
const SNDRV_CTL_ELEM_IFACE_PCM: c_uint = 2;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 1;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 10;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 2;
const SNDRV_PCM_FMTBIT_S8: u64 = 1 << 0;
const SNDRV_PCM_FORMAT_S8: c_int = 0;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 2;
const SNDRV_PCM_FORMAT_S32_LE: c_int = 10;
const SNDRV_PCM_RATE_CONTINUOUS: c_uint = 1 << 30;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1;
const SNDRV_PCM_INFO_MMAP: c_uint = 2;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const DMA_SLAVE_BUSWIDTH_UNDEFINED: c_int = 0;
const STM_SAI_CR1_REGX: c_uint = 0x00;
const STM_SAI_CR2_REGX: c_uint = 0x04;
const STM_SAI_FRCR_REGX: c_uint = 0x0c;
const STM_SAI_SLOTR_REGX: c_uint = 0x10;
const STM_SAI_IMR_REGX: c_uint = 0x14;
const STM_SAI_SR_REGX: c_uint = 0x18;
const STM_SAI_CLRFR_REGX: c_uint = 0x1c;
const STM_SAI_DR_REGX: c_uint = 0x20;
const STM_SAI_PDMCR_REGX: c_uint = 0x24;
const STM_SAI_PDMLY_REGX: c_uint = 0x28;
const SAI_XCR1_MCKEN: c_uint = 1 << 27;
const SAI_XCR1_NODIV: c_int = 1 << 19;
const SAI_XCR1_CKSTR: c_int = 1 << 9;
const SAI_XCR1_SLAVE: c_int = 1 << 5;
const SAI_XCR1_MONO: c_int = 1 << 12;
const SAI_XCR1_OSR: c_int = 1 << 26;
const SAI_XCR1_DMAEN: c_uint = 1 << 17;
const SAI_XCR1_SAIEN: c_uint = 1 << 16;
const SAI_XCR1_RX_TX: c_int = 1 << 2;
const SAI_XCR1_PRTCFG_MASK: c_int = 3 << 6;
const SAI_XCR1_DS_MASK: c_int = 7 << 5;
const SAI_XCR1_SYNCEN_MASK: c_int = 3 << 10;
const SAI_XCR2_FFLUSH: c_uint = 1 << 3;
const SAI_XCR2_FTH_MASK: c_uint = 7;
const SAI_XCR2_MUTECNT_MASK: c_int = 0x3f << 7;
const SAI_XFRCR_FSOFF: c_int = 1 << 18;
const SAI_XFRCR_FSDEF: c_int = 1 << 16;
const SAI_XFRCR_FSPOL: c_int = 1 << 17;
const SAI_XFRCR_FRL_MASK: c_uint = 0xff;
const SAI_XFRCR_FSALL_MASK: c_uint = 0x7f << 8;
const SAI_XSLOTR_SLOTSZ_MASK: c_int = 3 << 6;
const SAI_XSLOTR_NBSLOT_MASK: c_int = 0xf << 8;
const SAI_XSLOTR_SLOTEN_MASK: c_int = 0xffff << 16;
const SAI_XSLOTR_FBOFF_MASK: c_uint = 0x1f;
const SAI_XIMR_OVRUDRIE: c_uint = 1 << 0;
const SAI_XIMR_MUTEDETIE: c_uint = 1 << 1;
const SAI_XIMR_WCKCFGIE: c_uint = 1 << 2;
const SAI_XIMR_CNRDYIE: c_uint = 1 << 4;
const SAI_XIMR_AFSDETIE: c_uint = 1 << 5;
const SAI_XIMR_LFSDETIE: c_uint = 1 << 6;
const SAI_XIMR_MASK: c_uint = 0x7f;
const SAI_XCLRFR_MASK: c_uint = 0x7f;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_MSB: c_uint = 2;
const SND_SOC_DAIFMT_LSB: c_uint = 3;
const SND_SOC_DAIFMT_DSP_A: c_uint = 4;
const SND_SOC_DAIFMT_DSP_B: c_uint = 5;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0x00f0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0x0000;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0x0010;
const SND_SOC_DAIFMT_NB_IF: c_uint = 0x0020;
const SND_SOC_DAIFMT_IB_IF: c_uint = 0x0030;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0x0f00;
const SND_SOC_DAIFMT_BC_FC: c_uint = 0x0000;
const SND_SOC_DAIFMT_BP_FP: c_uint = 0x0100;
const STM_SAI_SYNC_OUT_A: c_int = 1;
const STM_SAI_SYNC_OUT_B: c_int = 2;
const SAI_GCR_SYNCIN_MAX: c_int = 3;
const IEC958_AES0_CON_NOT_COPYRIGHT: u8 = 0x04;
const IEC958_AES0_CON_EMPHASIS_NONE: u8 = 0x00;
const IEC958_AES1_CON_GENERAL: u8 = 0x00;
const IEC958_AES2_CON_SOURCE_UNSPEC: u8 = 0x00;
const IEC958_AES2_CON_CHANNEL_UNSPEC: u8 = 0x00;
const IEC958_AES3_CON_CLOCK_1000PPM: u8 = 0x00;
const IEC958_AES3_CON_FS_NOTID: u8 = 0x01;
const IEC958_AES3_CON_FS_22050: u8 = 0x04;
const IEC958_AES3_CON_FS_44100: u8 = 0x00;
const IEC958_AES3_CON_FS_88200: u8 = 0x08;
const IEC958_AES3_CON_FS_176400: u8 = 0x0c;
const IEC958_AES3_CON_FS_24000: u8 = 0x06;
const IEC958_AES3_CON_FS_48000: u8 = 0x02;
const IEC958_AES3_CON_FS_96000: u8 = 0x0a;
const IEC958_AES3_CON_FS_192000: u8 = 0x0e;
const IEC958_AES3_CON_FS_32000: u8 = 0x03;

const fn SAI_XCR2_FTH_SET(v: stm32_sai_fifo_th) -> c_uint { v as c_uint }
const fn DIV_ROUND_CLOSEST(n: c_uint, d: c_uint) -> c_int { ((n + d / 2) / d) as c_int }
fn roundup_pow_of_two(mut x: c_uint) -> c_uint { if x <= 1 { return 1; } x -= 1; x |= x >> 1; x |= x >> 2; x |= x >> 4; x |= x >> 8; x |= x >> 16; x + 1 }

unsafe extern "C" fn stm32_sai_sub_readable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        STM_SAI_CR1_REGX | STM_SAI_CR2_REGX | STM_SAI_FRCR_REGX |
        STM_SAI_SLOTR_REGX | STM_SAI_IMR_REGX | STM_SAI_SR_REGX |
        STM_SAI_CLRFR_REGX | STM_SAI_DR_REGX | STM_SAI_PDMCR_REGX |
        STM_SAI_PDMLY_REGX => true,
        _ => false,
    }
}

unsafe extern "C" fn stm32_sai_sub_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    matches!(reg, STM_SAI_DR_REGX | STM_SAI_SR_REGX)
}

unsafe extern "C" fn stm32_sai_sub_writeable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        STM_SAI_CR1_REGX | STM_SAI_CR2_REGX | STM_SAI_FRCR_REGX |
        STM_SAI_SLOTR_REGX | STM_SAI_IMR_REGX | STM_SAI_CLRFR_REGX |
        STM_SAI_DR_REGX | STM_SAI_PDMCR_REGX | STM_SAI_PDMLY_REGX => true,
        _ => false,
    }
}

unsafe fn stm32_sai_sub_reg_up(sai: *mut stm32_sai_sub_data, reg: c_uint, mask: c_uint, val: c_uint) -> c_int {
    let ret = clk_enable((*(*sai).pdata).pclk);
    if ret < 0 { return ret; }
    let ret = regmap_update_bits((*sai).regmap, reg, mask, val);
    clk_disable((*(*sai).pdata).pclk);
    ret
}

unsafe fn stm32_sai_sub_reg_wr(sai: *mut stm32_sai_sub_data, reg: c_uint, mask: c_uint, val: c_uint) -> c_int {
    let ret = clk_enable((*(*sai).pdata).pclk);
    if ret < 0 { return ret; }
    let ret = regmap_write_bits((*sai).regmap, reg, mask, val);
    clk_disable((*(*sai).pdata).pclk);
    ret
}

unsafe fn stm32_sai_sub_reg_rd(sai: *mut stm32_sai_sub_data, reg: c_uint, val: *mut c_uint) -> c_int {
    let ret = clk_enable((*(*sai).pdata).pclk);
    if ret < 0 { return ret; }
    let ret = regmap_read((*sai).regmap, reg, val);
    clk_disable((*(*sai).pdata).pclk);
    ret
}

static stm32_sai_sub_regmap_config_f4: regmap_config = regmap_config {
    reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: STM_SAI_DR_REGX,
    readable_reg: Some(stm32_sai_sub_readable_reg), volatile_reg: Some(stm32_sai_sub_volatile_reg),
    writeable_reg: Some(stm32_sai_sub_writeable_reg), fast_io: true, cache_type: REGCACHE_FLAT,
};

static stm32_sai_sub_regmap_config_h7: regmap_config = regmap_config {
    reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: STM_SAI_PDMLY_REGX,
    readable_reg: Some(stm32_sai_sub_readable_reg), volatile_reg: Some(stm32_sai_sub_volatile_reg),
    writeable_reg: Some(stm32_sai_sub_writeable_reg), fast_io: true, cache_type: REGCACHE_FLAT,
};

unsafe extern "C" fn snd_pcm_iec958_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
    (*uinfo).count = 1;
    0
}

unsafe extern "C" fn snd_pcm_iec958_get(kcontrol: *mut snd_kcontrol, uctl: *mut snd_ctl_elem_value) -> c_int {
    let sai = snd_kcontrol_chip(kcontrol) as *mut stm32_sai_sub_data;
    mutex_lock(&mut (*sai).ctrl_lock);
    ptr::copy_nonoverlapping((*sai).iec958.status.as_ptr(), (*uctl).value.iec958.status.as_mut_ptr(), 4);
    mutex_unlock(&mut (*sai).ctrl_lock);
    0
}

unsafe extern "C" fn snd_pcm_iec958_put(kcontrol: *mut snd_kcontrol, uctl: *mut snd_ctl_elem_value) -> c_int {
    let sai = snd_kcontrol_chip(kcontrol) as *mut stm32_sai_sub_data;
    mutex_lock(&mut (*sai).ctrl_lock);
    ptr::copy_nonoverlapping((*uctl).value.iec958.status.as_ptr(), (*sai).iec958.status.as_mut_ptr(), 4);
    mutex_unlock(&mut (*sai).ctrl_lock);
    0
}

static iec958_ctls: snd_kcontrol_new = snd_kcontrol_new {
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
    iface: SNDRV_CTL_ELEM_IFACE_PCM,
    name: b"IEC958 Playback Default\0".as_ptr() as *const c_char,
    info: Some(snd_pcm_iec958_info),
    get: Some(snd_pcm_iec958_get),
    put: Some(snd_pcm_iec958_put),
    device: 0,
};

#[repr(C)]
pub struct stm32_sai_mclk_data {
    pub hw: clk_hw,
    pub freq: c_ulong,
    pub sai_data: *mut stm32_sai_sub_data,
}
const STM32_SAI_MAX_CLKS: c_int = 1;

unsafe fn to_mclk_data(hw: *mut clk_hw) -> *mut stm32_sai_mclk_data {
    hw as *mut stm32_sai_mclk_data
}

unsafe fn stm32_sai_get_clk_div(sai: *mut stm32_sai_sub_data, input_rate: c_ulong, output_rate: c_ulong) -> c_int {
    let version = (*(*sai).pdata).conf.version;
    let div = DIV_ROUND_CLOSEST(input_rate as c_uint, output_rate as c_uint);
    if div > SAI_XCR1_MCKDIV_MAX(version) || div <= 0 {
        dev_err(&mut (*(*sai).pdev).dev, b"Divider %d out of range\n\0".as_ptr() as *const c_char, div);
        return -EINVAL;
    }
    dev_dbg(&mut (*(*sai).pdev).dev, b"SAI divider %d\n\0".as_ptr() as *const c_char, div);
    if input_rate % div as c_ulong != 0 {
        dev_dbg(&mut (*(*sai).pdev).dev, b"Rate not accurate. requested (%ld), actual (%ld)\n\0".as_ptr() as *const c_char, output_rate, input_rate / div as c_ulong);
    }
    div
}

unsafe fn stm32_sai_set_clk_div(sai: *mut stm32_sai_sub_data, div: c_uint) -> c_int {
    let version = (*(*sai).pdata).conf.version;
    if div as c_int > SAI_XCR1_MCKDIV_MAX(version) {
        dev_err(&mut (*(*sai).pdev).dev, b"Divider %d out of range\n\0".as_ptr() as *const c_char, div);
        return -EINVAL;
    }
    let mask = SAI_XCR1_MCKDIV_MASK(SAI_XCR1_MCKDIV_WIDTH(version)) as c_uint;
    let cr1 = SAI_XCR1_MCKDIV_SET(div as c_int) as c_uint;
    let ret = stm32_sai_sub_reg_up(sai, STM_SAI_CR1_REGX, mask, cr1);
    if ret < 0 { dev_err(&mut (*(*sai).pdev).dev, b"Failed to update CR1 register\n\0".as_ptr() as *const c_char); }
    ret
}

fn stm32_sai_rate_accurate(max_rate: c_uint, rate: c_uint) -> bool {
    let ratio = DIV_ROUND_CLOSEST(max_rate, rate);
    if ratio == 0 { return false; }
    let diff = (max_rate as i64 - (ratio as i64 * rate as i64)).abs() as u64;
    let dividend = 1_000_000u64.wrapping_mul(diff);
    let delta = dividend / max_rate as u64;
    delta <= SAI_CK_RATE_TOLERANCE
}

unsafe extern "C" fn stm32_sai_set_parent_clk(sai: *mut stm32_sai_sub_data, rate: c_uint) -> c_int {
    let pdev = (*sai).pdev;
    let mut parent_clk = (*(*sai).pdata).clk_x8k;
    if rate % SAI_RATE_11K == 0 { parent_clk = (*(*sai).pdata).clk_x11k; }
    let ret = clk_set_parent((*sai).sai_ck, parent_clk);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, b" Error %d setting sai_ck parent clock. %s\0".as_ptr() as *const c_char, ret, if ret == -EBUSY { b"Active stream rates conflict\n\0".as_ptr() } else { b"\n\0".as_ptr() });
    }
    ret
}

unsafe extern "C" fn stm32_sai_put_parent_rate(sai: *mut stm32_sai_sub_data) {
    if (*sai).sai_ck_used {
        (*sai).sai_ck_used = false;
        clk_rate_exclusive_put((*sai).sai_ck);
    }
}

unsafe extern "C" fn stm32_sai_set_parent_rate(sai: *mut stm32_sai_sub_data, rate: c_uint) -> c_int {
    let pdev = (*sai).pdev;
    let mut sai_ck_min_rate = rate * 256;
    let mut sai_ck_max_rate = if rate % SAI_RATE_11K == 0 { SAI_MAX_SAMPLE_RATE_11K * 256 } else { SAI_MAX_SAMPLE_RATE_8K * 256 };
    if (*sai).sai_mclk.is_null() && !STM_SAI_PROTOCOL_IS_SPDIF(sai) {
        sai_ck_min_rate = rate * (*sai).fs_length as c_uint;
        sai_ck_max_rate /= DIV_ROUND_CLOSEST(256, roundup_pow_of_two((*sai).fs_length as c_uint)) as c_uint;
    }
    clk_rate_exclusive_get((*sai).sai_ck);
    (*sai).sai_ck_used = true;
    let sai_curr_rate = clk_get_rate((*sai).sai_ck);
    dev_dbg(&mut (*pdev).dev, b"kernel clock rate: min [%u], max [%u], current [%u]\0".as_ptr() as *const c_char, sai_ck_min_rate, sai_ck_max_rate, sai_curr_rate);
    if stm32_sai_rate_accurate(sai_ck_max_rate, sai_curr_rate) && sai_curr_rate >= sai_ck_min_rate { return 0; }
    let mut sai_ck_rate = sai_ck_max_rate;
    let mut div = 1u32;
    while sai_ck_rate >= sai_ck_min_rate {
        let sai_new_rate = clk_round_rate((*sai).sai_ck, sai_ck_rate);
        if stm32_sai_rate_accurate(sai_ck_rate, sai_new_rate) {
            let ret = clk_set_rate((*sai).sai_ck, sai_ck_rate);
            if ret != 0 {
                dev_err(&mut (*pdev).dev, b"Error %d setting sai_ck rate. %s\0".as_ptr() as *const c_char, ret, if ret == -EBUSY { b"Active stream rates may be in conflict\n\0".as_ptr() } else { b"\n\0".as_ptr() });
                stm32_sai_put_parent_rate(sai);
                return -EINVAL;
            }
            return 0;
        }
        div += 1;
        sai_ck_rate = sai_ck_max_rate / div;
    }
    dev_err(&mut (*pdev).dev, b"Failed to find an accurate rate\0".as_ptr() as *const c_char);
    stm32_sai_put_parent_rate(sai);
    -EINVAL
}

unsafe extern "C" fn stm32_sai_mclk_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int {
    let mclk = to_mclk_data(hw);
    let sai = (*mclk).sai_data;
    let div = stm32_sai_get_clk_div(sai, (*req).best_parent_rate, (*req).rate);
    if div <= 0 { return -EINVAL; }
    (*mclk).freq = (*req).best_parent_rate / div as c_ulong;
    (*req).rate = (*mclk).freq;
    0
}

unsafe extern "C" fn stm32_sai_mclk_recalc_rate(hw: *mut clk_hw, _parent_rate: c_ulong) -> c_ulong {
    (*to_mclk_data(hw)).freq
}

unsafe extern "C" fn stm32_sai_mclk_set_rate(hw: *mut clk_hw, rate: c_ulong, parent_rate: c_ulong) -> c_int {
    let mclk = to_mclk_data(hw);
    let sai = (*mclk).sai_data;
    let div = stm32_sai_get_clk_div(sai, parent_rate, rate);
    if div < 0 { return div; }
    let ret = stm32_sai_set_clk_div(sai, div as c_uint);
    if ret != 0 { return ret; }
    (*mclk).freq = rate;
    0
}

unsafe extern "C" fn stm32_sai_mclk_enable(hw: *mut clk_hw) -> c_int {
    let sai = (*to_mclk_data(hw)).sai_data;
    dev_dbg(&mut (*(*sai).pdev).dev, b"Enable master clock\n\0".as_ptr() as *const c_char);
    stm32_sai_sub_reg_up(sai, STM_SAI_CR1_REGX, SAI_XCR1_MCKEN, SAI_XCR1_MCKEN)
}

unsafe extern "C" fn stm32_sai_mclk_disable(hw: *mut clk_hw) {
    let sai = (*to_mclk_data(hw)).sai_data;
    dev_dbg(&mut (*(*sai).pdev).dev, b"Disable master clock\n\0".as_ptr() as *const c_char);
    stm32_sai_sub_reg_up(sai, STM_SAI_CR1_REGX, SAI_XCR1_MCKEN, 0);
}

static mclk_ops: clk_ops = clk_ops {
    enable: Some(stm32_sai_mclk_enable),
    disable: Some(stm32_sai_mclk_disable),
    recalc_rate: Some(stm32_sai_mclk_recalc_rate),
    determine_rate: Some(stm32_sai_mclk_determine_rate),
    set_rate: Some(stm32_sai_mclk_set_rate),
};

unsafe fn stm32_sai_add_mclk_provider(sai: *mut stm32_sai_sub_data) -> c_int {
    let dev = &mut (*(*sai).pdev).dev as *mut device;
    let pname = __clk_get_name((*sai).sai_ck);
    let mclk = devm_kzalloc(dev, size_of::<stm32_sai_mclk_data>(), GFP_KERNEL) as *mut stm32_sai_mclk_data;
    if mclk.is_null() { return -ENOMEM; }
    let mclk_name = devm_kcalloc(dev, size_of::<c_char>(), SAI_MCLK_NAME_LEN, GFP_KERNEL) as *mut c_char;
    if mclk_name.is_null() { return -ENOMEM; }
    let mut p = mclk_name;
    let mut s = pname as *mut c_char;
    let mut i = 0usize;
    while *s != 0 && *s != b'_' as c_char && i < SAI_MCLK_NAME_LEN - 7 {
        *p = *s;
        p = p.add(1);
        s = s.add(1);
        i += 1;
    }
    if STM_SAI_IS_SUB_A(sai) { strcat(p, b"a_mclk\0".as_ptr() as *const c_char); } else { strcat(p, b"b_mclk\0".as_ptr() as *const c_char); }
    (*mclk).hw.init = ptr::null(); /* CLK_HW_INIT(mclk_name, pname, &mclk_ops, 0) */
    (*mclk).sai_data = sai;
    dev_dbg(dev, b"Register master clock %s\n\0".as_ptr() as *const c_char, mclk_name);
    let ret = devm_clk_hw_register(dev, &mut (*mclk).hw);
    if ret != 0 {
        dev_err(dev, b"mclk register returned %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    (*sai).sai_mclk = (*mclk).hw.clk;
    devm_of_clk_add_hw_provider(dev, ptr::null(), &mut (*mclk).hw)
}

unsafe extern "C" fn stm32_sai_isr(_irq: c_int, devid: *mut c_void) -> irqreturn_t {
    let sai = devid as *mut stm32_sai_sub_data;
    let pdev = (*sai).pdev;
    let mut imr = 0u32;
    let mut sr = 0u32;
    let mut status = SNDRV_PCM_STATE_RUNNING;
    stm32_sai_sub_reg_rd(sai, STM_SAI_IMR_REGX, &mut imr);
    stm32_sai_sub_reg_rd(sai, STM_SAI_SR_REGX, &mut sr);
    let flags = sr & imr;
    if flags == 0 { return IRQ_NONE; }
    stm32_sai_sub_reg_wr(sai, STM_SAI_CLRFR_REGX, SAI_XCLRFR_MASK, SAI_XCLRFR_MASK);
    if (*sai).substream.is_null() {
        dev_err(&mut (*pdev).dev, b"Device stopped. Spurious IRQ 0x%x\n\0".as_ptr() as *const c_char, sr);
        return IRQ_NONE;
    }
    if flags & SAI_XIMR_OVRUDRIE != 0 {
        dev_err(&mut (*pdev).dev, b"IRQ %s\n\0".as_ptr() as *const c_char, if STM_SAI_IS_PLAYBACK(sai) { b"underrun\0".as_ptr() } else { b"overrun\0".as_ptr() });
        status = SNDRV_PCM_STATE_XRUN;
    }
    if flags & SAI_XIMR_MUTEDETIE != 0 { dev_dbg(&mut (*pdev).dev, b"IRQ mute detected\n\0".as_ptr() as *const c_char); }
    if flags & SAI_XIMR_WCKCFGIE != 0 { dev_err(&mut (*pdev).dev, b"IRQ wrong clock configuration\n\0".as_ptr() as *const c_char); status = SNDRV_PCM_STATE_DISCONNECTED; }
    if flags & SAI_XIMR_CNRDYIE != 0 { dev_err(&mut (*pdev).dev, b"IRQ Codec not ready\n\0".as_ptr() as *const c_char); }
    if flags & SAI_XIMR_AFSDETIE != 0 { dev_err(&mut (*pdev).dev, b"IRQ Anticipated frame synchro\n\0".as_ptr() as *const c_char); status = SNDRV_PCM_STATE_XRUN; }
    if flags & SAI_XIMR_LFSDETIE != 0 { dev_err(&mut (*pdev).dev, b"IRQ Late frame synchro\n\0".as_ptr() as *const c_char); status = SNDRV_PCM_STATE_XRUN; }
    spin_lock(&mut (*sai).irq_lock);
    if status != SNDRV_PCM_STATE_RUNNING && !(*sai).substream.is_null() { snd_pcm_stop_xrun((*sai).substream); }
    spin_unlock(&mut (*sai).irq_lock);
    IRQ_HANDLED
}

unsafe extern "C" fn stm32_sai_set_sysclk(cpu_dai: *mut snd_soc_dai, _clk_id: c_int, freq: c_uint, dir: c_int) -> c_int {
    let sai = snd_soc_dai_get_drvdata(cpu_dai) as *mut stm32_sai_sub_data;
    if !snd_soc_card_is_instantiated((*(*cpu_dai).component).card) { return 0; }
    if dir == SND_SOC_CLOCK_OUT && !(*sai).sai_mclk.is_null() {
        let ret = stm32_sai_sub_reg_up(sai, STM_SAI_CR1_REGX, SAI_XCR1_NODIV as u32, if freq != 0 { 0 } else { SAI_XCR1_NODIV as u32 });
        if ret < 0 { return ret; }
        if freq == 0 {
            if (*sai).mclk_rate != 0 {
                clk_rate_exclusive_put((*sai).sai_mclk);
                (*sai).mclk_rate = 0;
            }
            if let Some(put) = (*sai).put_sai_ck_rate { put(sai); }
            return 0;
        }
        let ret = (*sai).set_sai_ck_rate.unwrap()(sai, freq);
        if ret != 0 { return ret; }
        let ret = clk_set_rate_exclusive((*sai).sai_mclk, freq);
        if ret != 0 {
            dev_err((*cpu_dai).dev, if ret == -EBUSY { b"Active streams have incompatible rates\0".as_ptr() } else { b"Could not set mclk rate\n\0".as_ptr() } as *const c_char);
            return ret;
        }
        dev_dbg((*cpu_dai).dev, b"SAI MCLK frequency is %uHz\n\0".as_ptr() as *const c_char, freq);
        (*sai).mclk_rate = freq;
    }
    0
}

unsafe extern "C" fn stm32_sai_set_dai_tdm_slot(cpu_dai: *mut snd_soc_dai, tx_mask: u32, rx_mask: u32, slots: c_int, slot_width: c_int) -> c_int {
    let sai = snd_soc_dai_get_drvdata(cpu_dai) as *mut stm32_sai_sub_data;
    if STM_SAI_PROTOCOL_IS_SPDIF(sai) {
        dev_warn((*cpu_dai).dev, b"Slot setting relevant only for TDM\n\0".as_ptr() as *const c_char);
        return 0;
    }
    dev_dbg((*cpu_dai).dev, b"Masks tx/rx:%#x/%#x, slots:%d, width:%d\n\0".as_ptr() as *const c_char, tx_mask, rx_mask, slots, slot_width);
    let slot_size = match slot_width { 16 => SAI_SLOT_SIZE_16, 32 => SAI_SLOT_SIZE_32, _ => SAI_SLOT_SIZE_AUTO };
    let mut slotr = SAI_XSLOTR_SLOTSZ_SET(slot_size) | SAI_XSLOTR_NBSLOT_SET(slots - 1);
    let mut slotr_mask = SAI_XSLOTR_SLOTSZ_MASK | SAI_XSLOTR_NBSLOT_MASK;
    if STM_SAI_IS_PLAYBACK(sai) { (*sai).slot_mask = tx_mask as c_int; slotr |= SAI_XSLOTR_SLOTEN_SET(tx_mask as c_int); }
    if STM_SAI_IS_CAPTURE(sai) { (*sai).slot_mask = rx_mask as c_int; slotr |= SAI_XSLOTR_SLOTEN_SET(rx_mask as c_int); }
    slotr_mask |= SAI_XSLOTR_SLOTEN_MASK;
    stm32_sai_sub_reg_up(sai, STM_SAI_SLOTR_REGX, slotr_mask as u32, slotr as u32);
    (*sai).slot_width = slot_width;
    (*sai).slots = slots;
    0
}

unsafe extern "C" fn stm32_sai_set_dai_fmt(cpu_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let sai = snd_soc_dai_get_drvdata(cpu_dai) as *mut stm32_sai_sub_data;
    let mut cr1 = SAI_XCR1_NODIV;
    let mut frcr = 0;
    let mut cr1_mask = SAI_XCR1_NODIV;
    let mut frcr_mask = 0;
    dev_dbg((*cpu_dai).dev, b"fmt %x\n\0".as_ptr() as *const c_char, fmt);
    cr1_mask |= SAI_XCR1_PRTCFG_MASK;
    if STM_SAI_PROTOCOL_IS_SPDIF(sai) { cr1 |= SAI_XCR1_PRTCFG_SET(SAI_SPDIF_PROTOCOL); }
    else {
        cr1 |= SAI_XCR1_PRTCFG_SET(SAI_FREE_PROTOCOL);
        match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
            SND_SOC_DAIFMT_I2S => { cr1 |= SAI_XCR1_CKSTR; frcr |= SAI_XFRCR_FSOFF | SAI_XFRCR_FSDEF; }
            SND_SOC_DAIFMT_MSB => { cr1 |= SAI_XCR1_CKSTR; frcr |= SAI_XFRCR_FSPOL | SAI_XFRCR_FSDEF; }
            SND_SOC_DAIFMT_LSB => { frcr |= SAI_XFRCR_FSPOL | SAI_XFRCR_FSDEF; }
            SND_SOC_DAIFMT_DSP_A => { cr1 |= SAI_XCR1_CKSTR; frcr |= SAI_XFRCR_FSPOL | SAI_XFRCR_FSOFF; }
            SND_SOC_DAIFMT_DSP_B => { cr1 |= SAI_XCR1_CKSTR; frcr |= SAI_XFRCR_FSPOL; }
            _ => { dev_err((*cpu_dai).dev, b"Unsupported protocol %#x\n\0".as_ptr() as *const c_char, fmt & SND_SOC_DAIFMT_FORMAT_MASK); return -EINVAL; }
        }
        cr1_mask |= SAI_XCR1_CKSTR;
        frcr_mask |= SAI_XFRCR_FSPOL | SAI_XFRCR_FSOFF | SAI_XFRCR_FSDEF;
        match fmt & SND_SOC_DAIFMT_INV_MASK {
            SND_SOC_DAIFMT_NB_NF => {}
            SND_SOC_DAIFMT_IB_NF => cr1 ^= SAI_XCR1_CKSTR,
            SND_SOC_DAIFMT_NB_IF => frcr ^= SAI_XFRCR_FSPOL,
            SND_SOC_DAIFMT_IB_IF => { cr1 ^= SAI_XCR1_CKSTR; frcr ^= SAI_XFRCR_FSPOL; }
            _ => { dev_err((*cpu_dai).dev, b"Unsupported strobing %#x\n\0".as_ptr() as *const c_char, fmt & SND_SOC_DAIFMT_INV_MASK); return -EINVAL; }
        }
        cr1_mask |= SAI_XCR1_CKSTR;
        frcr_mask |= SAI_XFRCR_FSPOL;
        stm32_sai_sub_reg_up(sai, STM_SAI_FRCR_REGX, frcr_mask as u32, frcr as u32);
        match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
            SND_SOC_DAIFMT_BC_FC => { cr1 |= SAI_XCR1_SLAVE; (*sai).master = false; }
            SND_SOC_DAIFMT_BP_FP => (*sai).master = true,
            _ => { dev_err((*cpu_dai).dev, b"Unsupported mode %#x\n\0".as_ptr() as *const c_char, fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK); return -EINVAL; }
        }
        if (*sai).sync != 0 {
            dev_dbg((*cpu_dai).dev, b"Synchronized SAI configured as slave\n\0".as_ptr() as *const c_char);
            cr1 |= SAI_XCR1_SLAVE;
            (*sai).master = false;
        }
        cr1_mask |= SAI_XCR1_SLAVE;
    }
    let ret = stm32_sai_sub_reg_up(sai, STM_SAI_CR1_REGX, cr1_mask as u32, cr1 as u32);
    if ret < 0 { dev_err((*cpu_dai).dev, b"Failed to update CR1 register\n\0".as_ptr() as *const c_char); return ret; }
    (*sai).fmt = fmt as c_int;
    0
}

unsafe extern "C" fn stm32_sai_startup(substream: *mut snd_pcm_substream, cpu_dai: *mut snd_soc_dai) -> c_int {
    let sai = snd_soc_dai_get_drvdata(cpu_dai) as *mut stm32_sai_sub_data;
    let mut flags = 0;
    spin_lock_irqsave(&mut (*sai).irq_lock, &mut flags);
    (*sai).substream = substream;
    spin_unlock_irqrestore(&mut (*sai).irq_lock, flags);
    if STM_SAI_PROTOCOL_IS_SPDIF(sai) {
        snd_pcm_hw_constraint_mask64((*substream).runtime, SNDRV_PCM_HW_PARAM_FORMAT, SNDRV_PCM_FMTBIT_S32_LE);
        snd_pcm_hw_constraint_single((*substream).runtime, SNDRV_PCM_HW_PARAM_CHANNELS, 2);
    }
    let ret = clk_prepare_enable((*sai).sai_ck);
    if ret < 0 { dev_err((*cpu_dai).dev, b"Failed to enable clock: %d\n\0".as_ptr() as *const c_char, ret); return ret; }
    stm32_sai_sub_reg_wr(sai, STM_SAI_CLRFR_REGX, SAI_XCLRFR_MASK, SAI_XCLRFR_MASK);
    let mut imr = SAI_XIMR_OVRUDRIE;
    if STM_SAI_IS_CAPTURE(sai) {
        let mut cr2 = 0u32;
        stm32_sai_sub_reg_rd(sai, STM_SAI_CR2_REGX, &mut cr2);
        if (cr2 as c_int & SAI_XCR2_MUTECNT_MASK) != 0 { imr |= SAI_XIMR_MUTEDETIE; }
    }
    if (*sai).master { imr |= SAI_XIMR_WCKCFGIE; } else { imr |= SAI_XIMR_AFSDETIE | SAI_XIMR_LFSDETIE; }
    stm32_sai_sub_reg_up(sai, STM_SAI_IMR_REGX, SAI_XIMR_MASK, imr);
    0
}

unsafe fn stm32_sai_set_config(cpu_dai: *mut snd_soc_dai, _substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    let sai = snd_soc_dai_get_drvdata(cpu_dai) as *mut stm32_sai_sub_data;
    stm32_sai_sub_reg_wr(sai, STM_SAI_CR2_REGX, SAI_XCR2_FFLUSH | SAI_XCR2_FTH_MASK, SAI_XCR2_FFLUSH | SAI_XCR2_FTH_SET(stm32_sai_fifo_th::STM_SAI_FIFO_TH_HALF));
    if STM_SAI_PROTOCOL_IS_SPDIF(sai) { (*sai).spdif_frm_cnt = 0; return 0; }
    let mut cr1_mask = SAI_XCR1_DS_MASK;
    let mut cr1 = match params_format(params) {
        SNDRV_PCM_FORMAT_S8 => SAI_XCR1_DS_SET(SAI_DATASIZE_8),
        SNDRV_PCM_FORMAT_S16_LE => SAI_XCR1_DS_SET(SAI_DATASIZE_16),
        SNDRV_PCM_FORMAT_S32_LE => SAI_XCR1_DS_SET(SAI_DATASIZE_32),
        _ => { dev_err((*cpu_dai).dev, b"Data format not supported\n\0".as_ptr() as *const c_char); return -EINVAL; }
    };
    cr1_mask |= SAI_XCR1_MONO;
    if (*sai).slots == 2 && params_channels(params) == 1 { cr1 |= SAI_XCR1_MONO; }
    let ret = stm32_sai_sub_reg_up(sai, STM_SAI_CR1_REGX, cr1_mask as u32, cr1 as u32);
    if ret < 0 { dev_err((*cpu_dai).dev, b"Failed to update CR1 register\n\0".as_ptr() as *const c_char); return ret; }
    0
}

unsafe fn stm32_sai_set_slots(cpu_dai: *mut snd_soc_dai) -> c_int {
    let sai = snd_soc_dai_get_drvdata(cpu_dai) as *mut stm32_sai_sub_data;
    let mut slotr = 0u32;
    stm32_sai_sub_reg_rd(sai, STM_SAI_SLOTR_REGX, &mut slotr);
    let slot_sz = slotr as c_int & SAI_XSLOTR_SLOTSZ_MASK;
    if slot_sz == SAI_XSLOTR_SLOTSZ_SET(SAI_SLOT_SIZE_AUTO) { (*sai).slot_width = (*sai).data_size; }
    if (*sai).slot_width < (*sai).data_size {
        dev_err((*cpu_dai).dev, b"Data size %d larger than slot width\n\0".as_ptr() as *const c_char, (*sai).data_size);
        return -EINVAL;
    }
    if (*sai).slots == 0 { (*sai).slots = 2; }
    stm32_sai_sub_reg_up(sai, STM_SAI_SLOTR_REGX, SAI_XSLOTR_NBSLOT_MASK as u32, SAI_XSLOTR_NBSLOT_SET((*sai).slots - 1) as u32);
    if (slotr as c_int & SAI_XSLOTR_SLOTEN_MASK) == 0 {
        (*sai).slot_mask = (1 << (*sai).slots) - 1;
        stm32_sai_sub_reg_up(sai, STM_SAI_SLOTR_REGX, SAI_XSLOTR_SLOTEN_MASK as u32, SAI_XSLOTR_SLOTEN_SET((*sai).slot_mask) as u32);
    }
    dev_dbg((*cpu_dai).dev, b"Slots %d, slot width %d\n\0".as_ptr() as *const c_char, (*sai).slots, (*sai).slot_width);
    0
}

unsafe fn stm32_sai_set_frame(cpu_dai: *mut snd_soc_dai) {
    let sai = snd_soc_dai_get_drvdata(cpu_dai) as *mut stm32_sai_sub_data;
    let format = (*sai).fmt as c_uint & SND_SOC_DAIFMT_FORMAT_MASK;
    (*sai).fs_length = (*sai).slot_width * (*sai).slots;
    let mut fs_active = (*sai).fs_length / 2;
    if format == SND_SOC_DAIFMT_DSP_A || format == SND_SOC_DAIFMT_DSP_B { fs_active = 1; }
    let frcr = SAI_XFRCR_FRL_SET((*sai).fs_length - 1) | SAI_XFRCR_FSALL_SET(fs_active - 1);
    let frcr_mask = SAI_XFRCR_FRL_MASK | SAI_XFRCR_FSALL_MASK;
    dev_dbg((*cpu_dai).dev, b"Frame length %d, frame active %d\n\0".as_ptr() as *const c_char, (*sai).fs_length, fs_active);
    stm32_sai_sub_reg_up(sai, STM_SAI_FRCR_REGX, frcr_mask, frcr as u32);
    if ((*sai).fmt as c_uint & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_LSB {
        let offset = (*sai).slot_width - (*sai).data_size;
        stm32_sai_sub_reg_up(sai, STM_SAI_SLOTR_REGX, SAI_XSLOTR_FBOFF_MASK, SAI_XSLOTR_FBOFF_SET(offset) as u32);
    }
}

unsafe fn stm32_sai_init_iec958_status(sai: *mut stm32_sai_sub_data) {
    let cs = (*sai).iec958.status.as_mut_ptr();
    *cs.add(0) = IEC958_AES0_CON_NOT_COPYRIGHT | IEC958_AES0_CON_EMPHASIS_NONE;
    *cs.add(1) = IEC958_AES1_CON_GENERAL;
    *cs.add(2) = IEC958_AES2_CON_SOURCE_UNSPEC | IEC958_AES2_CON_CHANNEL_UNSPEC;
    *cs.add(3) = IEC958_AES3_CON_CLOCK_1000PPM | IEC958_AES3_CON_FS_NOTID;
}

unsafe fn stm32_sai_set_iec958_status(sai: *mut stm32_sai_sub_data, runtime: *mut snd_pcm_runtime) {
    if runtime.is_null() { return; }
    mutex_lock(&mut (*sai).ctrl_lock);
    (*sai).iec958.status[3] = match (*runtime).rate {
        22050 => IEC958_AES3_CON_FS_22050,
        44100 => IEC958_AES3_CON_FS_44100,
        88200 => IEC958_AES3_CON_FS_88200,
        176400 => IEC958_AES3_CON_FS_176400,
        24000 => IEC958_AES3_CON_FS_24000,
        48000 => IEC958_AES3_CON_FS_48000,
        96000 => IEC958_AES3_CON_FS_96000,
        192000 => IEC958_AES3_CON_FS_192000,
        32000 => IEC958_AES3_CON_FS_32000,
        _ => IEC958_AES3_CON_FS_NOTID,
    };
    mutex_unlock(&mut (*sai).ctrl_lock);
}

unsafe fn stm32_sai_configure_clock(cpu_dai: *mut snd_soc_dai, params: *mut snd_pcm_hw_params) -> c_int {
    let sai = snd_soc_dai_get_drvdata(cpu_dai) as *mut stm32_sai_sub_data;
    let mut div = 0;
    let mut cr1 = 0;
    let rate = params_rate(params);
    if (*sai).sai_mclk.is_null() {
        let ret = (*sai).set_sai_ck_rate.unwrap()(sai, rate);
        if ret != 0 { return ret; }
    }
    let sai_clk_rate = clk_get_rate((*sai).sai_ck) as c_int;
    if STM_SAI_IS_F4((*sai).pdata) {
        if (*sai).mclk_rate == 0 { return 0; }
        if 2 * sai_clk_rate >= 3 * (*sai).mclk_rate as c_int {
            div = stm32_sai_get_clk_div(sai, sai_clk_rate as c_ulong, (2 * (*sai).mclk_rate) as c_ulong);
            if div < 0 { return div; }
        }
    } else if STM_SAI_PROTOCOL_IS_SPDIF(sai) {
        div = stm32_sai_get_clk_div(sai, sai_clk_rate as c_ulong, (rate * 128) as c_ulong);
        if div < 0 { return div; }
    } else if (*sai).mclk_rate != 0 {
        let mclk_ratio = (*sai).mclk_rate / rate;
        if mclk_ratio == 512 { cr1 = SAI_XCR1_OSR; }
        else if mclk_ratio != 256 {
            dev_err((*cpu_dai).dev, b"Wrong mclk ratio %d\n\0".as_ptr() as *const c_char, mclk_ratio);
            return -EINVAL;
        }
        stm32_sai_sub_reg_up(sai, STM_SAI_CR1_REGX, SAI_XCR1_OSR as u32, cr1 as u32);
        div = stm32_sai_get_clk_div(sai, sai_clk_rate as c_ulong, (*sai).mclk_rate as c_ulong);
        if div < 0 { return div; }
    } else {
        let den = (*sai).fs_length * params_rate(params) as c_int;
        div = stm32_sai_get_clk_div(sai, sai_clk_rate as c_ulong, den as c_ulong);
        if div < 0 { return div; }
    }
    stm32_sai_set_clk_div(sai, div as c_uint)
}

unsafe extern "C" fn stm32_sai_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, cpu_dai: *mut snd_soc_dai) -> c_int {
    let sai = snd_soc_dai_get_drvdata(cpu_dai) as *mut stm32_sai_sub_data;
    (*sai).data_size = params_width(params);
    if STM_SAI_PROTOCOL_IS_SPDIF(sai) {
        (*(*substream).runtime).rate = params_rate(params);
        stm32_sai_set_iec958_status(sai, (*substream).runtime);
    } else {
        let ret = stm32_sai_set_slots(cpu_dai);
        if ret < 0 { return ret; }
        stm32_sai_set_frame(cpu_dai);
    }
    let mut ret = stm32_sai_set_config(cpu_dai, substream, params);
    if ret != 0 { return ret; }
    if (*sai).master { ret = stm32_sai_configure_clock(cpu_dai, params); }
    ret
}

unsafe extern "C" fn stm32_sai_trigger(_substream: *mut snd_pcm_substream, cmd: c_int, cpu_dai: *mut snd_soc_dai) -> c_int {
    let sai = snd_soc_dai_get_drvdata(cpu_dai) as *mut stm32_sai_sub_data;
    let ret;
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            dev_dbg((*cpu_dai).dev, b"Enable DMA and SAI\n\0".as_ptr() as *const c_char);
            stm32_sai_sub_reg_up(sai, STM_SAI_CR1_REGX, SAI_XCR1_DMAEN, SAI_XCR1_DMAEN);
            ret = stm32_sai_sub_reg_up(sai, STM_SAI_CR1_REGX, SAI_XCR1_SAIEN, SAI_XCR1_SAIEN);
            if ret < 0 { dev_err((*cpu_dai).dev, b"Failed to update CR1 register\n\0".as_ptr() as *const c_char); }
        }
        SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_STOP => {
            dev_dbg((*cpu_dai).dev, b"Disable DMA and SAI\n\0".as_ptr() as *const c_char);
            stm32_sai_sub_reg_up(sai, STM_SAI_IMR_REGX, SAI_XIMR_MASK, 0);
            stm32_sai_sub_reg_up(sai, STM_SAI_CR1_REGX, SAI_XCR1_SAIEN, !SAI_XCR1_SAIEN);
            ret = stm32_sai_sub_reg_up(sai, STM_SAI_CR1_REGX, SAI_XCR1_DMAEN, !SAI_XCR1_DMAEN);
            if ret < 0 { dev_err((*cpu_dai).dev, b"Failed to update CR1 register\n\0".as_ptr() as *const c_char); }
            if STM_SAI_PROTOCOL_IS_SPDIF(sai) { (*sai).spdif_frm_cnt = 0; }
        }
        _ => return -EINVAL,
    }
    ret
}

unsafe extern "C" fn stm32_sai_shutdown(_substream: *mut snd_pcm_substream, cpu_dai: *mut snd_soc_dai) {
    let sai = snd_soc_dai_get_drvdata(cpu_dai) as *mut stm32_sai_sub_data;
    stm32_sai_sub_reg_up(sai, STM_SAI_IMR_REGX, SAI_XIMR_MASK, 0);
    clk_disable_unprepare((*sai).sai_ck);
    if (*sai).sai_mclk.is_null() {
        if let Some(put) = (*sai).put_sai_ck_rate { put(sai); }
    }
    let mut flags = 0;
    spin_lock_irqsave(&mut (*sai).irq_lock, &mut flags);
    (*sai).substream = ptr::null_mut();
    spin_unlock_irqrestore(&mut (*sai).irq_lock, flags);
}

unsafe extern "C" fn stm32_sai_pcm_new(rtd: *mut snd_soc_pcm_runtime, cpu_dai: *mut snd_soc_dai) -> c_int {
    let sai = dev_get_drvdata((*cpu_dai).dev) as *mut stm32_sai_sub_data;
    let mut knew = iec958_ctls;
    if STM_SAI_PROTOCOL_IS_SPDIF(sai) {
        dev_dbg(&mut (*(*sai).pdev).dev, b"%s: register iec controls\0".as_ptr() as *const c_char, b"stm32_sai_pcm_new\0".as_ptr());
        knew.device = (*(*rtd).pcm).device;
        return snd_ctl_add((*(*rtd).pcm).card, snd_ctl_new1(&mut knew, sai as *mut c_void));
    }
    0
}

unsafe extern "C" fn stm32_sai_dai_probe(cpu_dai: *mut snd_soc_dai) -> c_int {
    let sai = dev_get_drvdata((*cpu_dai).dev) as *mut stm32_sai_sub_data;
    (*sai).cpu_dai = cpu_dai;
    (*sai).dma_params.addr = (*sai).phys_addr + STM_SAI_DR_REGX as dma_addr_t;
    (*sai).dma_params.maxburst = 4;
    if (*(*sai).pdata).conf.fifo_size < 8 || (*(*sai).pdata).conf.no_dma_burst { (*sai).dma_params.maxburst = 1; }
    (*sai).dma_params.addr_width = DMA_SLAVE_BUSWIDTH_UNDEFINED;
    if STM_SAI_IS_PLAYBACK(sai) { snd_soc_dai_init_dma_data(cpu_dai, &mut (*sai).dma_params, ptr::null_mut()); }
    else { snd_soc_dai_init_dma_data(cpu_dai, ptr::null_mut(), &mut (*sai).dma_params); }
    if STM_SAI_PROTOCOL_IS_SPDIF(sai) { return 0; }
    let mut cr1 = 0;
    let mut cr1_mask = SAI_XCR1_RX_TX;
    if STM_SAI_IS_CAPTURE(sai) { cr1 |= SAI_XCR1_RX_TX; }
    if (*sai).sync == SAI_SYNC_EXTERNAL {
        let ret = (*(*sai).pdata).set_sync.unwrap()((*sai).pdata, (*sai).np_sync_provider, (*sai).synco, (*sai).synci);
        if ret != 0 { return ret; }
    }
    cr1_mask |= SAI_XCR1_SYNCEN_MASK;
    cr1 |= SAI_XCR1_SYNCEN_SET((*sai).sync);
    stm32_sai_sub_reg_up(sai, STM_SAI_CR1_REGX, cr1_mask as u32, cr1 as u32)
}

static stm32_sai_pcm_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(stm32_sai_dai_probe), set_sysclk: Some(stm32_sai_set_sysclk), set_fmt: Some(stm32_sai_set_dai_fmt),
    set_tdm_slot: Some(stm32_sai_set_dai_tdm_slot), startup: Some(stm32_sai_startup), hw_params: Some(stm32_sai_hw_params),
    trigger: Some(stm32_sai_trigger), shutdown: Some(stm32_sai_shutdown), pcm_new: Some(stm32_sai_pcm_new),
};
static stm32_sai_pcm_dai_ops2: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(stm32_sai_dai_probe), set_sysclk: Some(stm32_sai_set_sysclk), set_fmt: Some(stm32_sai_set_dai_fmt),
    set_tdm_slot: Some(stm32_sai_set_dai_tdm_slot), startup: Some(stm32_sai_startup), hw_params: Some(stm32_sai_hw_params),
    trigger: Some(stm32_sai_trigger), shutdown: Some(stm32_sai_shutdown), pcm_new: None,
};

unsafe extern "C" fn stm32_sai_pcm_process_spdif(substream: *mut snd_pcm_substream, channel: c_int, hwoff: c_ulong, bytes: c_ulong) -> c_int {
    let runtime = (*substream).runtime;
    let rtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let sai = dev_get_drvdata((*cpu_dai).dev) as *mut stm32_sai_sub_data;
    let mut ptr_i = (*runtime).dma_area.add(hwoff as usize + channel as usize * ((*runtime).dma_bytes / (*runtime).channels as c_ulong) as usize) as *mut c_int;
    let mut cnt = bytes_to_samples(runtime, bytes);
    let mut frm_cnt = (*sai).spdif_frm_cnt;
    loop {
        *ptr_i = (*ptr_i >> 8) & 0x00ff_ffff;
        let byte = frm_cnt >> 3;
        let mask = 1u32 << (frm_cnt - (byte << 3));
        if ((*sai).iec958.status[byte as usize] as u32 & mask) != 0 { *ptr_i |= 0x0400_0000; }
        ptr_i = ptr_i.add(1);
        if cnt % 2 == 0 { frm_cnt += 1; }
        if frm_cnt == SAI_IEC60958_BLOCK_FRAMES { frm_cnt = 0; }
        cnt -= 1;
        if cnt == 0 { break; }
    }
    (*sai).spdif_frm_cnt = frm_cnt;
    0
}

static stm32_sai_pcm_hw_spdif: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED, buffer_bytes_max: 8 * PAGE_SIZE,
    period_bytes_min: 1024, period_bytes_max: PAGE_SIZE, periods_min: 2, periods_max: 8,
};
static stm32_sai_pcm_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP, buffer_bytes_max: 8 * PAGE_SIZE,
    period_bytes_min: 1024, period_bytes_max: PAGE_SIZE, periods_min: 2, periods_max: 8,
};

static mut stm32_sai_playback_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    id: 1,
    playback: snd_soc_pcm_stream { channels_min: 1, channels_max: 16, rate_min: 8000, rate_max: 192000, rates: SNDRV_PCM_RATE_CONTINUOUS, formats: SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE },
    capture: snd_soc_pcm_stream { channels_min: 0, channels_max: 0, rate_min: 0, rate_max: 0, rates: 0, formats: 0 },
    ops: &stm32_sai_pcm_dai_ops,
    name: ptr::null(),
};
static mut stm32_sai_capture_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    id: 1,
    playback: snd_soc_pcm_stream { channels_min: 0, channels_max: 0, rate_min: 0, rate_max: 0, rates: 0, formats: 0 },
    capture: snd_soc_pcm_stream { channels_min: 1, channels_max: 16, rate_min: 8000, rate_max: 192000, rates: SNDRV_PCM_RATE_CONTINUOUS, formats: SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE },
    ops: &stm32_sai_pcm_dai_ops2,
    name: ptr::null(),
};

static stm32_sai_pcm_config: snd_dmaengine_pcm_config = snd_dmaengine_pcm_config {
    pcm_hardware: &stm32_sai_pcm_hw,
    prepare_slave_config: unsafe { &snd_dmaengine_pcm_prepare_slave_config as *const _ as *const c_void },
    process: None,
};
static stm32_sai_pcm_config_spdif: snd_dmaengine_pcm_config = snd_dmaengine_pcm_config {
    pcm_hardware: &stm32_sai_pcm_hw_spdif,
    prepare_slave_config: unsafe { &snd_dmaengine_pcm_prepare_slave_config as *const _ as *const c_void },
    process: Some(stm32_sai_pcm_process_spdif),
};
static stm32_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"stm32-sai\0".as_ptr() as *const c_char,
    legacy_dai_naming: 1,
};
static stm32_sai_sub_ids: [of_device_id; 3] = [
    of_device_id { compatible: b"st,stm32-sai-sub-a\0".as_ptr() as *const c_char, data: STM_SAI_A_ID as usize as *const c_void },
    of_device_id { compatible: b"st,stm32-sai-sub-b\0".as_ptr() as *const c_char, data: STM_SAI_B_ID as usize as *const c_void },
    of_device_id { compatible: ptr::null(), data: ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, stm32_sai_sub_ids); */

unsafe fn IS_ERR<T>(p: *mut T) -> bool { (p as isize) < 0 && (p as isize) > -4096 }
unsafe fn PTR_ERR<T>(p: *mut T) -> c_int { p as isize as c_int }

unsafe fn stm32_sai_sub_parse_of(pdev: *mut platform_device, sai: *mut stm32_sai_sub_data) -> c_int {
    let np = (*pdev).dev.of_node;
    let mut res: *mut resource = ptr::null_mut();
    let mut args = of_phandle_args { np: ptr::null_mut(), args: [0; 8] };
    if np.is_null() { return -ENODEV; }
    let base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR(base) { return PTR_ERR(base); }
    (*sai).phys_addr = (*res).start;
    (*sai).regmap_config = &stm32_sai_sub_regmap_config_f4;
    if STM_SAI_HAS_PDM(sai) && STM_SAI_IS_SUB_A(sai) { (*sai).regmap_config = &stm32_sai_sub_regmap_config_h7; }
    (*sai).regmap = devm_regmap_init_mmio(&mut (*pdev).dev, base, (*sai).regmap_config);
    if IS_ERR((*sai).regmap) { return dev_err_probe(&mut (*pdev).dev, PTR_ERR((*sai).regmap), b"Regmap init error\n\0".as_ptr() as *const c_char); }
    if of_property_match_string(np, b"dma-names\0".as_ptr() as *const c_char, b"tx\0".as_ptr() as *const c_char) >= 0 {
        (*sai).dir = SNDRV_PCM_STREAM_PLAYBACK;
    } else if of_property_match_string(np, b"dma-names\0".as_ptr() as *const c_char, b"rx\0".as_ptr() as *const c_char) >= 0 {
        (*sai).dir = SNDRV_PCM_STREAM_CAPTURE;
    } else {
        dev_err(&mut (*pdev).dev, b"Unsupported direction\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    (*sai).spdif = false;
    if of_property_present(np, b"st,iec60958\0".as_ptr() as *const c_char) {
        if !STM_SAI_HAS_SPDIF(sai) || (*sai).dir == SNDRV_PCM_STREAM_CAPTURE {
            dev_err(&mut (*pdev).dev, b"S/PDIF IEC60958 not supported\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
        stm32_sai_init_iec958_status(sai);
        (*sai).spdif = true;
        (*sai).master = true;
    }
    let mut ret = of_parse_phandle_with_fixed_args(np, b"st,sync\0".as_ptr() as *const c_char, 1, 0, &mut args);
    if ret < 0 && ret != -ENOENT {
        dev_err(&mut (*pdev).dev, b"Failed to get st,sync property\n\0".as_ptr() as *const c_char);
        return ret;
    }
    (*sai).sync = SAI_SYNC_NONE;
    if !args.np.is_null() {
        if args.np == np {
            dev_err(&mut (*pdev).dev, b"%pOFn sync own reference\n\0".as_ptr() as *const c_char, np);
            of_node_put(args.np);
            return -EINVAL;
        }
        (*sai).np_sync_provider = of_get_parent(args.np);
        if (*sai).np_sync_provider.is_null() {
            dev_err(&mut (*pdev).dev, b"%pOFn parent node not found\n\0".as_ptr() as *const c_char, np);
            of_node_put(args.np);
            return -ENODEV;
        }
        (*sai).sync = SAI_SYNC_INTERNAL;
        if (*sai).np_sync_provider != (*(*(*sai).pdata).pdev).dev.of_node {
            if !STM_SAI_HAS_EXT_SYNC(sai) {
                dev_err(&mut (*pdev).dev, b"External synchro not supported\n\0".as_ptr() as *const c_char);
                of_node_put(args.np);
                ret = -EINVAL;
                of_node_put((*sai).np_sync_provider);
                return ret;
            }
            (*sai).sync = SAI_SYNC_EXTERNAL;
            (*sai).synci = args.args[0];
            if (*sai).synci < 1 || (*sai).synci > SAI_GCR_SYNCIN_MAX + 1 {
                dev_err(&mut (*pdev).dev, b"Wrong SAI index\n\0".as_ptr() as *const c_char);
                of_node_put(args.np);
                ret = -EINVAL;
                of_node_put((*sai).np_sync_provider);
                return ret;
            }
            if of_property_match_string(args.np, b"compatible\0".as_ptr() as *const c_char, b"st,stm32-sai-sub-a\0".as_ptr() as *const c_char) >= 0 { (*sai).synco = STM_SAI_SYNC_OUT_A; }
            if of_property_match_string(args.np, b"compatible\0".as_ptr() as *const c_char, b"st,stm32-sai-sub-b\0".as_ptr() as *const c_char) >= 0 { (*sai).synco = STM_SAI_SYNC_OUT_B; }
            if (*sai).synco == 0 {
                dev_err(&mut (*pdev).dev, b"Unknown SAI sub-block\n\0".as_ptr() as *const c_char);
                of_node_put(args.np);
                ret = -EINVAL;
                of_node_put((*sai).np_sync_provider);
                return ret;
            }
        }
        dev_dbg(&mut (*pdev).dev, b"%s synchronized with %s\n\0".as_ptr() as *const c_char, (*pdev).name, (*args.np).full_name);
    }
    of_node_put(args.np);
    (*sai).sai_ck = devm_clk_get(&mut (*pdev).dev, b"sai_ck\0".as_ptr() as *const c_char);
    if IS_ERR((*sai).sai_ck) {
        ret = dev_err_probe(&mut (*pdev).dev, PTR_ERR((*sai).sai_ck), b"Missing kernel clock sai_ck\n\0".as_ptr() as *const c_char);
        of_node_put((*sai).np_sync_provider);
        return ret;
    }
    ret = clk_prepare((*(*sai).pdata).pclk);
    if ret < 0 { of_node_put((*sai).np_sync_provider); return ret; }
    if STM_SAI_IS_F4((*sai).pdata) { return 0; }
    if of_property_present(np, b"#clock-cells\0".as_ptr() as *const c_char) {
        ret = stm32_sai_add_mclk_provider(sai);
        if ret < 0 { clk_unprepare((*(*sai).pdata).pclk); of_node_put((*sai).np_sync_provider); return ret; }
    } else {
        (*sai).sai_mclk = devm_clk_get_optional(&mut (*pdev).dev, b"MCLK\0".as_ptr() as *const c_char);
        if IS_ERR((*sai).sai_mclk) {
            ret = PTR_ERR((*sai).sai_mclk);
            clk_unprepare((*(*sai).pdata).pclk);
            of_node_put((*sai).np_sync_provider);
            return ret;
        }
    }
    0
}

unsafe extern "C" fn stm32_sai_sub_probe(pdev: *mut platform_device) -> c_int {
    let mut conf: *const snd_dmaengine_pcm_config = &stm32_sai_pcm_config;
    let sai = devm_kzalloc(&mut (*pdev).dev, size_of::<stm32_sai_sub_data>(), GFP_KERNEL) as *mut stm32_sai_sub_data;
    if sai.is_null() { return -ENOMEM; }
    (*sai).id = device_get_match_data(&mut (*pdev).dev) as usize as c_uint;
    (*sai).pdev = pdev;
    mutex_init(&mut (*sai).ctrl_lock);
    spin_lock_init(&mut (*sai).irq_lock);
    platform_set_drvdata(pdev, sai as *mut c_void);
    (*sai).pdata = dev_get_drvdata((*pdev).dev.parent) as *mut stm32_sai_data;
    if (*sai).pdata.is_null() {
        dev_err(&mut (*pdev).dev, b"Parent device data not available\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    if (*(*sai).pdata).conf.get_sai_ck_parent { (*sai).set_sai_ck_rate = Some(stm32_sai_set_parent_clk); }
    else { (*sai).set_sai_ck_rate = Some(stm32_sai_set_parent_rate); (*sai).put_sai_ck_rate = Some(stm32_sai_put_parent_rate); }
    let mut ret = stm32_sai_sub_parse_of(pdev, sai);
    if ret != 0 { return ret; }
    if STM_SAI_IS_PLAYBACK(sai) { (*sai).cpu_dai_drv = stm32_sai_playback_dai; }
    else { (*sai).cpu_dai_drv = stm32_sai_capture_dai; }
    (*sai).cpu_dai_drv.name = dev_name(&mut (*pdev).dev);
    ret = devm_request_irq(&mut (*pdev).dev, (*(*sai).pdata).irq, stm32_sai_isr, IRQF_SHARED, dev_name(&mut (*pdev).dev), sai as *mut c_void);
    if ret != 0 { clk_unprepare((*(*sai).pdata).pclk); of_node_put((*sai).np_sync_provider); return ret; }
    if STM_SAI_PROTOCOL_IS_SPDIF(sai) { conf = &stm32_sai_pcm_config_spdif; }
    ret = snd_dmaengine_pcm_register(&mut (*pdev).dev, conf, 0);
    if ret != 0 { clk_unprepare((*(*sai).pdata).pclk); of_node_put((*sai).np_sync_provider); return ret; }
    ret = snd_soc_register_component(&mut (*pdev).dev, &stm32_component, &mut (*sai).cpu_dai_drv, 1);
    if ret != 0 {
        snd_dmaengine_pcm_unregister(&mut (*pdev).dev);
        clk_unprepare((*(*sai).pdata).pclk);
        of_node_put((*sai).np_sync_provider);
        return ret;
    }
    pm_runtime_enable(&mut (*pdev).dev);
    0
}

unsafe extern "C" fn stm32_sai_sub_remove(pdev: *mut platform_device) {
    let sai = dev_get_drvdata(&mut (*pdev).dev) as *mut stm32_sai_sub_data;
    clk_unprepare((*(*sai).pdata).pclk);
    snd_dmaengine_pcm_unregister(&mut (*pdev).dev);
    snd_soc_unregister_component(&mut (*pdev).dev);
    pm_runtime_disable(&mut (*pdev).dev);
    of_node_put((*sai).np_sync_provider);
}

unsafe extern "C" fn stm32_sai_sub_suspend(dev: *mut device) -> c_int {
    let sai = dev_get_drvdata(dev) as *mut stm32_sai_sub_data;
    let ret = clk_enable((*(*sai).pdata).pclk);
    if ret < 0 { return ret; }
    regcache_cache_only((*sai).regmap, true);
    regcache_mark_dirty((*sai).regmap);
    clk_disable((*(*sai).pdata).pclk);
    0
}

unsafe extern "C" fn stm32_sai_sub_resume(dev: *mut device) -> c_int {
    let sai = dev_get_drvdata(dev) as *mut stm32_sai_sub_data;
    let ret = clk_enable((*(*sai).pdata).pclk);
    if ret < 0 { return ret; }
    regcache_cache_only((*sai).regmap, false);
    let ret = regcache_sync((*sai).regmap);
    clk_disable((*(*sai).pdata).pclk);
    ret
}

/* static const struct dev_pm_ops stm32_sai_sub_pm_ops = {
 *     SYSTEM_SLEEP_PM_OPS(stm32_sai_sub_suspend, stm32_sai_sub_resume)
 * };
 *
 * static struct platform_driver stm32_sai_sub_driver = {
 *     .driver = {
 *         .name = "st,stm32-sai-sub",
 *         .of_match_table = stm32_sai_sub_ids,
 *         .pm = pm_ptr(&stm32_sai_sub_pm_ops),
 *     },
 *     .probe = stm32_sai_sub_probe,
 *     .remove = stm32_sai_sub_remove,
 * };
 *
 * module_platform_driver(stm32_sai_sub_driver);
 *
 * MODULE_DESCRIPTION("STM32 Soc SAI sub-block Interface");
 * MODULE_AUTHOR("Olivier Moysan <olivier.moysan@st.com>");
 * MODULE_ALIAS("platform:st,stm32-sai-sub");
 * MODULE_LICENSE("GPL v2");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
