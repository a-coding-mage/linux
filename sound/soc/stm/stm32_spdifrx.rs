// SPDX-License-Identifier: GPL-2.0-only
/*
 * STM32 ALSA SoC Digital Audio Interface (SPDIF-rx) driver.
 *
 * Copyright (C) 2017, STMicroelectronics - All Rights Reserved
 * Author(s): Olivier Moysan <olivier.moysan@st.com> for STMicroelectronics.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

/* Includes in the C source provide Linux, ALSA, DMA, regmap, clock, reset,
 * runtime PM, and device-tree declarations used below.
 */

type u8 = u8;
type u16 = u16;
type u32 = u32;
type bool_ = bool;
type dma_addr_t = usize;
type dma_cookie_t = c_int;
type irqreturn_t = c_uint;

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
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
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub reg_stride: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub num_reg_defaults_raw: c_uint,
    pub fast_io: bool_,
    pub cache_type: c_uint,
}
#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr: dma_addr_t,
    pub addr_width: c_uint,
    pub maxburst: c_uint,
}
#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_dma_buffer {
    pub dev: snd_dma_device,
    pub area: *mut c_void,
    pub addr: dma_addr_t,
}
#[repr(C)]
pub struct snd_dma_device {
    pub type_: c_int,
    pub dev: *mut device,
}
#[repr(C)]
pub struct dma_chan {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dma_async_tx_descriptor {
    pub callback: Option<unsafe extern "C" fn(*mut c_void)>,
    pub callback_param: *mut c_void,
}
#[repr(C)]
pub struct dma_slave_config {
    pub direction: c_uint,
    pub src_addr: dma_addr_t,
    pub dst_addr: dma_addr_t,
    pub src_addr_width: c_uint,
    pub src_maxburst: c_uint,
}
#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}
#[repr(C)]
pub struct snd_ctl_elem_value_value {
    pub iec958: snd_aes_iec958,
}
#[repr(C)]
pub struct snd_aes_iec958 {
    pub status: [u8; 24],
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
pub struct snd_soc_dai {
    pub dev: *mut device,
    pub component: *mut snd_soc_component,
}
#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
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
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
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
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub legacy_dai_naming: c_uint,
}
#[repr(C)]
pub struct snd_dmaengine_pcm_config {
    pub pcm_hardware: *const snd_pcm_hardware,
    pub prepare_slave_config: Option<unsafe extern "C" fn()>,
}
#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}
#[repr(C)]
pub struct resource {
    pub start: dma_addr_t,
}
#[repr(C)]
pub struct reset_control {
    _private: [u8; 0],
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

const fn bit(n: c_uint) -> c_uint {
    1u32 << n
}
const fn genmask(h: c_uint, l: c_uint) -> c_uint {
    (!0u32 >> (31 - h)) & (!0u32 << l)
}
const fn field_get(mask: c_uint, reg: c_uint) -> c_uint {
    (reg & mask) >> mask.trailing_zeros()
}

/* SPDIF-rx Register Map */
const STM32_SPDIFRX_CR: c_uint = 0x00;
const STM32_SPDIFRX_IMR: c_uint = 0x04;
const STM32_SPDIFRX_SR: c_uint = 0x08;
const STM32_SPDIFRX_IFCR: c_uint = 0x0C;
const STM32_SPDIFRX_DR: c_uint = 0x10;
const STM32_SPDIFRX_CSR: c_uint = 0x14;
const STM32_SPDIFRX_DIR: c_uint = 0x18;
const STM32_SPDIFRX_VERR: c_uint = 0x3F4;
const STM32_SPDIFRX_IDR: c_uint = 0x3F8;
const STM32_SPDIFRX_SIDR: c_uint = 0x3FC;

/* Bit definition for SPDIF_CR register */
const SPDIFRX_CR_SPDIFEN_SHIFT: c_uint = 0;
const SPDIFRX_CR_SPDIFEN_MASK: c_uint = genmask(1, SPDIFRX_CR_SPDIFEN_SHIFT);
const fn SPDIFRX_CR_SPDIFENSET(x: c_uint) -> c_uint { x << SPDIFRX_CR_SPDIFEN_SHIFT }
const SPDIFRX_CR_RXDMAEN: c_uint = bit(2);
const SPDIFRX_CR_RXSTEO: c_uint = bit(3);
const SPDIFRX_CR_DRFMT_SHIFT: c_uint = 4;
const SPDIFRX_CR_DRFMT_MASK: c_uint = genmask(5, SPDIFRX_CR_DRFMT_SHIFT);
const fn SPDIFRX_CR_DRFMTSET(x: c_uint) -> c_uint { x << SPDIFRX_CR_DRFMT_SHIFT }
const SPDIFRX_CR_PMSK: c_uint = bit(6);
const SPDIFRX_CR_VMSK: c_uint = bit(7);
const SPDIFRX_CR_CUMSK: c_uint = bit(8);
const SPDIFRX_CR_PTMSK: c_uint = bit(9);
const SPDIFRX_CR_CBDMAEN: c_uint = bit(10);
const SPDIFRX_CR_CHSEL_SHIFT: c_uint = 11;
const SPDIFRX_CR_CHSEL: c_uint = bit(SPDIFRX_CR_CHSEL_SHIFT);
const SPDIFRX_CR_NBTR_SHIFT: c_uint = 12;
const SPDIFRX_CR_NBTR_MASK: c_uint = genmask(13, SPDIFRX_CR_NBTR_SHIFT);
const fn SPDIFRX_CR_NBTRSET(x: c_uint) -> c_uint { x << SPDIFRX_CR_NBTR_SHIFT }
const SPDIFRX_CR_WFA: c_uint = bit(14);
const SPDIFRX_CR_INSEL_SHIFT: c_uint = 16;
/* C source spells PDIFRX_CR_INSEL_SHIFT inside this macro. */
const SPDIFRX_CR_INSEL_MASK: c_uint = genmask(18, SPDIFRX_CR_INSEL_SHIFT);
const fn SPDIFRX_CR_INSELSET(x: c_uint) -> c_uint { x << SPDIFRX_CR_INSEL_SHIFT }
const SPDIFRX_CR_CKSEN_SHIFT: c_uint = 20;
const SPDIFRX_CR_CKSEN: c_uint = bit(20);
const SPDIFRX_CR_CKSBKPEN: c_uint = bit(21);

/* Bit definition for SPDIFRX_IMR register */
const SPDIFRX_IMR_RXNEI: c_uint = bit(0);
const SPDIFRX_IMR_CSRNEIE: c_uint = bit(1);
const SPDIFRX_IMR_PERRIE: c_uint = bit(2);
const SPDIFRX_IMR_OVRIE: c_uint = bit(3);
const SPDIFRX_IMR_SBLKIE: c_uint = bit(4);
const SPDIFRX_IMR_SYNCDIE: c_uint = bit(5);
const SPDIFRX_IMR_IFEIE: c_uint = bit(6);
const SPDIFRX_XIMR_MASK: c_uint = genmask(6, 0);

/* Bit definition for SPDIFRX_SR register */
const SPDIFRX_SR_RXNE: c_uint = bit(0);
const SPDIFRX_SR_CSRNE: c_uint = bit(1);
const SPDIFRX_SR_PERR: c_uint = bit(2);
const SPDIFRX_SR_OVR: c_uint = bit(3);
const SPDIFRX_SR_SBD: c_uint = bit(4);
const SPDIFRX_SR_SYNCD: c_uint = bit(5);
const SPDIFRX_SR_FERR: c_uint = bit(6);
const SPDIFRX_SR_SERR: c_uint = bit(7);
const SPDIFRX_SR_TERR: c_uint = bit(8);
const SPDIFRX_SR_WIDTH5_SHIFT: c_uint = 16;
/* C source spells PDIFRX_SR_WIDTH5_SHIFT inside this macro. */
const SPDIFRX_SR_WIDTH5_MASK: c_uint = genmask(30, SPDIFRX_SR_WIDTH5_SHIFT);
const fn SPDIFRX_SR_WIDTH5SET(x: c_uint) -> c_uint { x << SPDIFRX_SR_WIDTH5_SHIFT }

/* Bit definition for SPDIFRX_IFCR register */
const SPDIFRX_IFCR_PERRCF: c_uint = bit(2);
const SPDIFRX_IFCR_OVRCF: c_uint = bit(3);
const SPDIFRX_IFCR_SBDCF: c_uint = bit(4);
const SPDIFRX_IFCR_SYNCDCF: c_uint = bit(5);
const SPDIFRX_XIFCR_MASK: c_uint = genmask(5, 2);

/* Bit definition for SPDIFRX_DR register (DRFMT = 0b00) */
const SPDIFRX_DR0_DR_SHIFT: c_uint = 0;
const SPDIFRX_DR0_DR_MASK: c_uint = genmask(23, SPDIFRX_DR0_DR_SHIFT);
const fn SPDIFRX_DR0_DRSET(x: c_uint) -> c_uint { x << SPDIFRX_DR0_DR_SHIFT }
const SPDIFRX_DR0_PE: c_uint = bit(24);
const SPDIFRX_DR0_V: c_uint = bit(25);
const SPDIFRX_DR0_U: c_uint = bit(26);
const SPDIFRX_DR0_C: c_uint = bit(27);
const SPDIFRX_DR0_PT_SHIFT: c_uint = 28;
const SPDIFRX_DR0_PT_MASK: c_uint = genmask(29, SPDIFRX_DR0_PT_SHIFT);
const fn SPDIFRX_DR0_PTSET(x: c_uint) -> c_uint { x << SPDIFRX_DR0_PT_SHIFT }

/* Bit definition for SPDIFRX_DR register (DRFMT = 0b01) */
const SPDIFRX_DR1_PE: c_uint = bit(0);
const SPDIFRX_DR1_V: c_uint = bit(1);
const SPDIFRX_DR1_U: c_uint = bit(2);
const SPDIFRX_DR1_C: c_uint = bit(3);
const SPDIFRX_DR1_PT_SHIFT: c_uint = 4;
const SPDIFRX_DR1_PT_MASK: c_uint = genmask(5, SPDIFRX_DR1_PT_SHIFT);
const fn SPDIFRX_DR1_PTSET(x: c_uint) -> c_uint { x << SPDIFRX_DR1_PT_SHIFT }
const SPDIFRX_DR1_DR_SHIFT: c_uint = 8;
const SPDIFRX_DR1_DR_MASK: c_uint = genmask(31, SPDIFRX_DR1_DR_SHIFT);
const fn SPDIFRX_DR1_DRSET(x: c_uint) -> c_uint { x << SPDIFRX_DR1_DR_SHIFT }

/* Bit definition for SPDIFRX_DR register (DRFMT = 0b10) */
const SPDIFRX_DR1_DRNL1_SHIFT: c_uint = 0;
const SPDIFRX_DR1_DRNL1_MASK: c_uint = genmask(15, SPDIFRX_DR1_DRNL1_SHIFT);
const fn SPDIFRX_DR1_DRNL1SET(x: c_uint) -> c_uint { x << SPDIFRX_DR1_DRNL1_SHIFT }
const SPDIFRX_DR1_DRNL2_SHIFT: c_uint = 16;
const SPDIFRX_DR1_DRNL2_MASK: c_uint = genmask(31, SPDIFRX_DR1_DRNL2_SHIFT);
const fn SPDIFRX_DR1_DRNL2SET(x: c_uint) -> c_uint { x << SPDIFRX_DR1_DRNL2_SHIFT }

/* Bit definition for SPDIFRX_CSR register */
const SPDIFRX_CSR_USR_SHIFT: c_uint = 0;
const SPDIFRX_CSR_USR_MASK: c_uint = genmask(15, SPDIFRX_CSR_USR_SHIFT);
const fn SPDIFRX_CSR_USRGET(x: c_uint) -> c_uint { (x & SPDIFRX_CSR_USR_MASK) >> SPDIFRX_CSR_USR_SHIFT }
const SPDIFRX_CSR_CS_SHIFT: c_uint = 16;
const SPDIFRX_CSR_CS_MASK: c_uint = genmask(23, SPDIFRX_CSR_CS_SHIFT);
const fn SPDIFRX_CSR_CSGET(x: c_uint) -> c_uint { (x & SPDIFRX_CSR_CS_MASK) >> SPDIFRX_CSR_CS_SHIFT }
const SPDIFRX_CSR_SOB: c_uint = bit(24);

/* Bit definition for SPDIFRX_DIR register */
const SPDIFRX_DIR_THI_SHIFT: c_uint = 0;
const SPDIFRX_DIR_THI_MASK: c_uint = genmask(12, SPDIFRX_DIR_THI_SHIFT);
const fn SPDIFRX_DIR_THI_SET(x: c_uint) -> c_uint { x << SPDIFRX_DIR_THI_SHIFT }
const SPDIFRX_DIR_TLO_SHIFT: c_uint = 16;
const SPDIFRX_DIR_TLO_MASK: c_uint = genmask(28, SPDIFRX_DIR_TLO_SHIFT);
const fn SPDIFRX_DIR_TLO_SET(x: c_uint) -> c_uint { x << SPDIFRX_DIR_TLO_SHIFT }

const SPDIFRX_SPDIFEN_DISABLE: c_uint = 0x0;
const SPDIFRX_SPDIFEN_SYNC: c_uint = 0x1;
const SPDIFRX_SPDIFEN_ENABLE: c_uint = 0x3;
const SPDIFRX_VERR_MIN_MASK: c_uint = genmask(3, 0);
const SPDIFRX_VERR_MAJ_MASK: c_uint = genmask(7, 4);
const SPDIFRX_IDR_ID_MASK: c_uint = genmask(31, 0);
const SPDIFRX_SIDR_SID_MASK: c_uint = genmask(31, 0);
const SPDIFRX_IPIDR_NUMBER: c_uint = 0x00130041;
const SPDIFRX_IN1: c_uint = 0x1;
const SPDIFRX_IN2: c_uint = 0x2;
const SPDIFRX_IN3: c_uint = 0x3;
const SPDIFRX_IN4: c_uint = 0x4;
const SPDIFRX_IN5: c_uint = 0x5;
const SPDIFRX_IN6: c_uint = 0x6;
const SPDIFRX_IN7: c_uint = 0x7;
const SPDIFRX_IN8: c_uint = 0x8;
const SPDIFRX_NBTR_NONE: c_uint = 0x0;
const SPDIFRX_NBTR_3: c_uint = 0x1;
const SPDIFRX_NBTR_15: c_uint = 0x2;
const SPDIFRX_NBTR_63: c_uint = 0x3;
const SPDIFRX_DRFMT_RIGHT: c_uint = 0x0;
const SPDIFRX_DRFMT_LEFT: c_uint = 0x1;
const SPDIFRX_DRFMT_PACKED: c_uint = 0x2;

/* 192 CS bits in S/PDIF frame. i.e 24 CS bytes */
const SPDIFRX_CS_BYTES_NB: usize = 24;
const SPDIFRX_UB_BYTES_NB: usize = 48;

/*
 * CSR register is retrieved as a 32 bits word
 * It contains 1 channel status byte and 2 user data bytes
 * 2 S/PDIF frames are acquired to get all CS/UB bits
 */
const SPDIFRX_CSR_BUF_LENGTH: usize = SPDIFRX_CS_BYTES_NB * 4 * 2;

#[repr(C)]
pub struct stm32_spdifrx_data {
    pub pdev: *mut platform_device,
    pub base: *mut c_void,
    pub regmap: *mut regmap,
    pub regmap_conf: *const regmap_config,
    pub cs_completion: completion,
    pub kclk: *mut clk,
    pub dma_params: snd_dmaengine_dai_dma_data,
    pub substream: *mut snd_pcm_substream,
    pub dmab: *mut snd_dma_buffer,
    pub ctrl_chan: *mut dma_chan,
    pub desc: *mut dma_async_tx_descriptor,
    pub slave_config: dma_slave_config,
    pub phys_addr: dma_addr_t,
    pub lock: spinlock_t, /* Sync enabling lock */
    pub irq_lock: spinlock_t, /* Prevent race condition on stream state */
    pub cs: [u8; SPDIFRX_CS_BYTES_NB],
    pub ub: [u8; SPDIFRX_UB_BYTES_NB],
    pub irq: c_int,
    pub refcount: c_int,
}

unsafe extern "C" {
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn complete(completion: *mut completion);
    fn dmaengine_prep_slave_single(chan: *mut dma_chan, buf: dma_addr_t, len: usize, dir: c_uint, flags: c_uint) -> *mut dma_async_tx_descriptor;
    fn dmaengine_submit(desc: *mut dma_async_tx_descriptor) -> dma_cookie_t;
    fn dma_submit_error(cookie: dma_cookie_t) -> c_int;
    fn dma_async_issue_pending(chan: *mut dma_chan);
    fn dmaengine_terminate_async(chan: *mut dma_chan);
    fn dma_request_chan(dev: *mut device, name: *const c_char) -> *mut dma_chan;
    fn dma_release_channel(chan: *mut dma_chan);
    fn dmaengine_slave_config(chan: *mut dma_chan, config: *mut dma_slave_config) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn snd_dma_alloc_pages(type_: c_int, dev: *mut device, size: usize, dmab: *mut snd_dma_buffer) -> c_int;
    fn snd_dma_free_pages(dmab: *mut snd_dma_buffer);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn wait_for_completion_interruptible_timeout(completion: *mut completion, timeout: c_ulong) -> c_long;
    fn msecs_to_jiffies(msecs: c_uint) -> c_ulong;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_dai;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut stm32_spdifrx_data;
    fn snd_soc_add_dai_controls(dai: *mut snd_soc_dai, controls: *mut snd_kcontrol_new, num_controls: c_uint) -> c_int;
    fn snd_soc_add_component_controls(component: *mut snd_soc_component, controls: *mut snd_kcontrol_new, num_controls: c_uint) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_soc_dai_init_dma_data(dai: *mut snd_soc_dai, playback: *mut snd_dmaengine_dai_dma_data, capture: *mut snd_dmaengine_dai_dma_data);
    fn snd_pcm_stop(substream: *mut snd_pcm_substream, state: c_uint);
    fn snd_pcm_stop_xrun(substream: *mut snd_pcm_substream);
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_platform_get_and_ioremap_resource(pdev: *mut platform_device, index: c_uint, res: *mut *mut resource) -> *mut c_void;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn platform_get_irq(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_regmap_init_mmio_clk(dev: *mut device, clk_id: *const c_char, regs: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>, flags: c_ulong, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn devm_reset_control_get_optional_exclusive(dev: *mut device, id: *const c_char) -> *mut reset_control;
    fn reset_control_assert(rst: *mut reset_control);
    fn reset_control_deassert(rst: *mut reset_control);
    fn udelay(usecs: c_uint);
    fn snd_dmaengine_pcm_register(dev: *mut device, config: *const snd_dmaengine_pcm_config, flags: c_uint) -> c_int;
    fn snd_dmaengine_pcm_unregister(dev: *mut device);
    fn snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn snd_soc_unregister_component(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn init_completion(x: *mut completion);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_err_probe(dev: *mut device, err: isize, fmt: *const c_char, ...) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> isize;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const EAGAIN: c_int = 11;
const GFP_KERNEL: c_uint = 0;
const DMA_DEV_TO_MEM: c_uint = 0;
const DMA_CTRL_ACK: c_uint = 0;
const DMA_SLAVE_BUSWIDTH_4_BYTES: c_uint = 4;
const SNDRV_DMA_TYPE_DEV_IRAM: c_int = 0;
const SNDRV_CTL_ELEM_TYPE_IEC958: c_uint = 0;
const SNDRV_CTL_ELEM_IFACE_PCM: c_uint = 0;
const SNDRV_CTL_ELEM_ACCESS_READ: c_uint = 1;
const SNDRV_CTL_ELEM_ACCESS_VOLATILE: c_uint = 2;
const SNDRV_PCM_STATE_DISCONNECTED: c_uint = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 2;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 4;
const SNDRV_PCM_TRIGGER_STOP: c_int = 5;
const SNDRV_PCM_RATE_8000_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 1;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 0;
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 1;
const PAGE_SIZE: usize = 4096;
const REGCACHE_FLAT: c_uint = 0;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;

unsafe extern "C" fn stm32_spdifrx_dma_complete(data: *mut c_void) {
    let spdifrx = data as *mut stm32_spdifrx_data;
    let pdev = (*spdifrx).pdev;
    let p_start = (*(*spdifrx).dmab).area as *mut u32;
    let p_end = p_start.add(2 * SPDIFRX_CS_BYTES_NB).sub(1);
    let mut ptr = p_start;
    let mut ub_ptr = (*spdifrx).ub.as_mut_ptr() as *mut u16;
    let mut i: c_int = 0;

    regmap_update_bits((*spdifrx).regmap, STM32_SPDIFRX_CR, SPDIFRX_CR_CBDMAEN, !SPDIFRX_CR_CBDMAEN);

    if (*(*spdifrx).dmab).area.is_null() {
        return;
    }

    while ptr <= p_end {
        if (*ptr & SPDIFRX_CSR_SOB) != 0 {
            break;
        }
        ptr = ptr.add(1);
    }

    if ptr > p_end {
        dev_err(&mut (*pdev).dev, c"Start of S/PDIF block not found\n".as_ptr());
        return;
    }

    while i < SPDIFRX_CS_BYTES_NB as c_int {
        (*spdifrx).cs[i as usize] = SPDIFRX_CSR_CSGET(*ptr) as u8;
        *ub_ptr = SPDIFRX_CSR_USRGET(*ptr) as u16;
        ub_ptr = ub_ptr.add(1);
        ptr = ptr.add(1);
        if ptr > p_end {
            dev_err(&mut (*pdev).dev, c"Failed to get channel status\n".as_ptr());
            return;
        }
        i += 1;
    }

    complete(&mut (*spdifrx).cs_completion);
}

unsafe fn stm32_spdifrx_dma_ctrl_start(spdifrx: *mut stm32_spdifrx_data) -> c_int {
    let cookie: dma_cookie_t;
    let mut err: c_int;

    (*spdifrx).desc = dmaengine_prep_slave_single(
        (*spdifrx).ctrl_chan,
        (*(*spdifrx).dmab).addr,
        SPDIFRX_CSR_BUF_LENGTH,
        DMA_DEV_TO_MEM,
        DMA_CTRL_ACK,
    );
    if (*spdifrx).desc.is_null() {
        return -EINVAL;
    }

    (*(*spdifrx).desc).callback = Some(stm32_spdifrx_dma_complete);
    (*(*spdifrx).desc).callback_param = spdifrx as *mut c_void;
    cookie = dmaengine_submit((*spdifrx).desc);
    err = dma_submit_error(cookie);
    if err != 0 {
        return -EINVAL;
    }

    dma_async_issue_pending((*spdifrx).ctrl_chan);

    0
}

unsafe fn stm32_spdifrx_dma_ctrl_stop(spdifrx: *mut stm32_spdifrx_data) {
    dmaengine_terminate_async((*spdifrx).ctrl_chan);
}

unsafe fn stm32_spdifrx_start_sync(spdifrx: *mut stm32_spdifrx_data) -> c_int {
    let mut cr: c_uint = 0;
    let mut cr_mask: c_uint;
    let imr: c_uint;
    let mut ret: c_int;

    /* Enable IRQs */
    imr = SPDIFRX_IMR_IFEIE | SPDIFRX_IMR_SYNCDIE | SPDIFRX_IMR_PERRIE;
    ret = regmap_update_bits((*spdifrx).regmap, STM32_SPDIFRX_IMR, imr, imr);
    if ret != 0 {
        return ret;
    }

    /* C guard(spinlock_irqsave)(&spdifrx->lock) scope begins here. */
    (*spdifrx).refcount += 1;

    regmap_read((*spdifrx).regmap, STM32_SPDIFRX_CR, &mut cr);

    if (cr & SPDIFRX_CR_SPDIFEN_MASK) == 0 {
        /*
         * Start sync if SPDIFRX is still in idle state.
         * SPDIFRX reception enabled when sync done
         */
        dev_dbg(&mut (*(*spdifrx).pdev).dev, c"start synchronization\n".as_ptr());

        /*
         * SPDIFRX configuration:
         * Wait for activity before starting sync process. This avoid
         * to issue sync errors when spdif signal is missing on input.
         * Preamble, CS, user, validity and parity error bits not copied
         * to DR register.
         */
        cr = SPDIFRX_CR_WFA | SPDIFRX_CR_PMSK | SPDIFRX_CR_VMSK |
            SPDIFRX_CR_CUMSK | SPDIFRX_CR_PTMSK | SPDIFRX_CR_RXSTEO;
        cr_mask = cr;

        cr |= SPDIFRX_CR_NBTRSET(SPDIFRX_NBTR_63);
        cr_mask |= SPDIFRX_CR_NBTR_MASK;
        cr |= SPDIFRX_CR_SPDIFENSET(SPDIFRX_SPDIFEN_SYNC);
        cr_mask |= SPDIFRX_CR_SPDIFEN_MASK;
        ret = regmap_update_bits((*spdifrx).regmap, STM32_SPDIFRX_CR, cr_mask, cr);
        if ret < 0 {
            dev_err(&mut (*(*spdifrx).pdev).dev, c"Failed to start synchronization\n".as_ptr());
        }
    }

    ret
}

unsafe fn stm32_spdifrx_stop(spdifrx: *mut stm32_spdifrx_data) {
    let mut cr: c_uint;
    let cr_mask: c_uint;
    let mut reg: c_uint = 0;

    /* C guard(spinlock_irqsave)(&spdifrx->lock) scope begins here. */
    (*spdifrx).refcount -= 1;
    if (*spdifrx).refcount != 0 {
        return;
    }

    cr = SPDIFRX_CR_SPDIFENSET(SPDIFRX_SPDIFEN_DISABLE);
    cr_mask = SPDIFRX_CR_SPDIFEN_MASK | SPDIFRX_CR_RXDMAEN;
    regmap_update_bits((*spdifrx).regmap, STM32_SPDIFRX_CR, cr_mask, cr);
    regmap_update_bits((*spdifrx).regmap, STM32_SPDIFRX_IMR, SPDIFRX_XIMR_MASK, 0);
    regmap_update_bits((*spdifrx).regmap, STM32_SPDIFRX_IFCR, SPDIFRX_XIFCR_MASK, SPDIFRX_XIFCR_MASK);

    /* dummy read to clear CSRNE and RXNE in status register */
    regmap_read((*spdifrx).regmap, STM32_SPDIFRX_DR, &mut reg);
    regmap_read((*spdifrx).regmap, STM32_SPDIFRX_CSR, &mut reg);
}

unsafe fn stm32_spdifrx_dma_ctrl_register(dev: *mut device, spdifrx: *mut stm32_spdifrx_data) -> c_int {
    let mut ret: c_int;

    (*spdifrx).ctrl_chan = dma_request_chan(dev, c"rx-ctrl".as_ptr());
    if IS_ERR((*spdifrx).ctrl_chan as *const c_void) {
        return dev_err_probe(dev, PTR_ERR((*spdifrx).ctrl_chan as *const c_void), c"dma_request_slave_channel error\n".as_ptr());
    }

    (*spdifrx).dmab = devm_kzalloc(dev, core::mem::size_of::<snd_dma_buffer>(), GFP_KERNEL) as *mut snd_dma_buffer;
    if (*spdifrx).dmab.is_null() {
        return -ENOMEM;
    }

    (*(*spdifrx).dmab).dev.type_ = SNDRV_DMA_TYPE_DEV_IRAM;
    (*(*spdifrx).dmab).dev.dev = dev;
    ret = snd_dma_alloc_pages((*(*spdifrx).dmab).dev.type_, dev, SPDIFRX_CSR_BUF_LENGTH, (*spdifrx).dmab);
    if ret < 0 {
        dev_err(dev, c"snd_dma_alloc_pages returned error %d\n".as_ptr(), ret);
        return ret;
    }

    (*spdifrx).slave_config.direction = DMA_DEV_TO_MEM;
    (*spdifrx).slave_config.src_addr = (*spdifrx).phys_addr + STM32_SPDIFRX_CSR as usize;
    (*spdifrx).slave_config.dst_addr = (*(*spdifrx).dmab).addr;
    (*spdifrx).slave_config.src_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    (*spdifrx).slave_config.src_maxburst = 1;

    ret = dmaengine_slave_config((*spdifrx).ctrl_chan, &mut (*spdifrx).slave_config);
    if ret < 0 {
        dev_err(dev, c"dmaengine_slave_config returned error %d\n".as_ptr(), ret);
        (*spdifrx).ctrl_chan = core::ptr::null_mut();
    }

    ret
}

static spdifrx_enum_input: [*const c_char; 4] = [
    c"in0".as_ptr(), c"in1".as_ptr(), c"in2".as_ptr(), c"in3".as_ptr(),
];

/*  By default CS bits are retrieved from channel A */
static spdifrx_enum_cs_channel: [*const c_char; 2] = [
    c"A".as_ptr(), c"B".as_ptr(),
];

/* SOC_ENUM_SINGLE_DECL(ctrl_enum_input, STM32_SPDIFRX_CR, SPDIFRX_CR_INSEL_SHIFT, spdifrx_enum_input); */
/* SOC_ENUM_SINGLE_DECL(ctrl_enum_cs_channel, STM32_SPDIFRX_CR, SPDIFRX_CR_CHSEL_SHIFT, spdifrx_enum_cs_channel); */

unsafe extern "C" fn stm32_spdifrx_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
    (*uinfo).count = 1;
    0
}

unsafe extern "C" fn stm32_spdifrx_ub_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
    (*uinfo).count = 1;
    0
}

unsafe fn stm32_spdifrx_get_ctrl_data(spdifrx: *mut stm32_spdifrx_data) -> c_int {
    let mut ret: c_int = 0;

    memset((*spdifrx).cs.as_mut_ptr() as *mut c_void, 0, SPDIFRX_CS_BYTES_NB);
    memset((*spdifrx).ub.as_mut_ptr() as *mut c_void, 0, SPDIFRX_UB_BYTES_NB);

    ret = stm32_spdifrx_dma_ctrl_start(spdifrx);
    if ret < 0 {
        return ret;
    }

    ret = clk_prepare_enable((*spdifrx).kclk);
    if ret != 0 {
        dev_err(&mut (*(*spdifrx).pdev).dev, c"Enable kclk failed: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = regmap_update_bits((*spdifrx).regmap, STM32_SPDIFRX_CR, SPDIFRX_CR_CBDMAEN, SPDIFRX_CR_CBDMAEN);
    if ret < 0 {
        clk_disable_unprepare((*spdifrx).kclk);
        return ret;
    }

    ret = stm32_spdifrx_start_sync(spdifrx);
    if ret < 0 {
        clk_disable_unprepare((*spdifrx).kclk);
        return ret;
    }

    if wait_for_completion_interruptible_timeout(&mut (*spdifrx).cs_completion, msecs_to_jiffies(100)) <= 0 {
        dev_dbg(&mut (*(*spdifrx).pdev).dev, c"Failed to get control data\n".as_ptr());
        ret = -EAGAIN;
    }

    stm32_spdifrx_stop(spdifrx);
    stm32_spdifrx_dma_ctrl_stop(spdifrx);
    clk_disable_unprepare((*spdifrx).kclk);

    ret
}

unsafe extern "C" fn stm32_spdifrx_capture_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cpu_dai = snd_kcontrol_chip(kcontrol);
    let spdifrx = snd_soc_dai_get_drvdata(cpu_dai);

    stm32_spdifrx_get_ctrl_data(spdifrx);

    (*ucontrol).value.iec958.status[0] = (*spdifrx).cs[0];
    (*ucontrol).value.iec958.status[1] = (*spdifrx).cs[1];
    (*ucontrol).value.iec958.status[2] = (*spdifrx).cs[2];
    (*ucontrol).value.iec958.status[3] = (*spdifrx).cs[3];
    (*ucontrol).value.iec958.status[4] = (*spdifrx).cs[4];

    0
}

unsafe extern "C" fn stm32_spdif_user_bits_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cpu_dai = snd_kcontrol_chip(kcontrol);
    let spdifrx = snd_soc_dai_get_drvdata(cpu_dai);

    stm32_spdifrx_get_ctrl_data(spdifrx);

    (*ucontrol).value.iec958.status[0] = (*spdifrx).ub[0];
    (*ucontrol).value.iec958.status[1] = (*spdifrx).ub[1];
    (*ucontrol).value.iec958.status[2] = (*spdifrx).ub[2];
    (*ucontrol).value.iec958.status[3] = (*spdifrx).ub[3];
    (*ucontrol).value.iec958.status[4] = (*spdifrx).ub[4];

    0
}

static mut stm32_spdifrx_iec_ctrls: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: c"IEC958 Capture Default".as_ptr(),
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(stm32_spdifrx_info),
        get: Some(stm32_spdifrx_capture_get),
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: c"IEC958 User Bit Capture Default".as_ptr(),
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(stm32_spdifrx_ub_info),
        get: Some(stm32_spdif_user_bits_get),
    },
];

/* static struct snd_kcontrol_new stm32_spdifrx_ctrls[] uses SOC_ENUM controls. */
static mut stm32_spdifrx_ctrls: [snd_kcontrol_new; 0] = [];

unsafe fn stm32_spdifrx_dai_register_ctrls(cpu_dai: *mut snd_soc_dai) -> c_int {
    let ret: c_int;

    ret = snd_soc_add_dai_controls(cpu_dai, stm32_spdifrx_iec_ctrls.as_mut_ptr(), stm32_spdifrx_iec_ctrls.len() as c_uint);
    if ret < 0 {
        return ret;
    }

    snd_soc_add_component_controls((*cpu_dai).component, stm32_spdifrx_ctrls.as_mut_ptr(), stm32_spdifrx_ctrls.len() as c_uint)
}

unsafe extern "C" fn stm32_spdifrx_dai_probe(cpu_dai: *mut snd_soc_dai) -> c_int {
    let spdifrx = dev_get_drvdata((*cpu_dai).dev) as *mut stm32_spdifrx_data;

    (*spdifrx).dma_params.addr = (*spdifrx).phys_addr + STM32_SPDIFRX_DR as usize;
    (*spdifrx).dma_params.maxburst = 1;

    snd_soc_dai_init_dma_data(cpu_dai, core::ptr::null_mut(), &mut (*spdifrx).dma_params);

    stm32_spdifrx_dai_register_ctrls(cpu_dai)
}

unsafe extern "C" fn stm32_spdifrx_readable_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        STM32_SPDIFRX_CR | STM32_SPDIFRX_IMR | STM32_SPDIFRX_SR |
        STM32_SPDIFRX_IFCR | STM32_SPDIFRX_DR | STM32_SPDIFRX_CSR |
        STM32_SPDIFRX_DIR | STM32_SPDIFRX_VERR | STM32_SPDIFRX_IDR |
        STM32_SPDIFRX_SIDR => true,
        _ => false,
    }
}

unsafe extern "C" fn stm32_spdifrx_volatile_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        STM32_SPDIFRX_DR | STM32_SPDIFRX_CSR | STM32_SPDIFRX_SR | STM32_SPDIFRX_DIR => true,
        _ => false,
    }
}

unsafe extern "C" fn stm32_spdifrx_writeable_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        STM32_SPDIFRX_CR | STM32_SPDIFRX_IMR | STM32_SPDIFRX_IFCR => true,
        _ => false,
    }
}

static stm32_h7_spdifrx_regmap_conf: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: STM32_SPDIFRX_SIDR,
    readable_reg: Some(stm32_spdifrx_readable_reg),
    volatile_reg: Some(stm32_spdifrx_volatile_reg),
    writeable_reg: Some(stm32_spdifrx_writeable_reg),
    num_reg_defaults_raw: STM32_SPDIFRX_SIDR / core::mem::size_of::<u32>() as c_uint + 1,
    fast_io: true,
    cache_type: REGCACHE_FLAT,
};

unsafe extern "C" fn stm32_spdifrx_isr(_irq: c_int, devid: *mut c_void) -> irqreturn_t {
    let spdifrx = devid as *mut stm32_spdifrx_data;
    let pdev = (*spdifrx).pdev;
    let mut cr: c_uint = 0;
    let mut mask: c_uint;
    let mut sr: c_uint = 0;
    let mut imr: c_uint = 0;
    let flags: c_uint;
    let sync_state: c_uint;
    let mut err: c_int = 0;
    let mut err_xrun: c_int = 0;

    regmap_read((*spdifrx).regmap, STM32_SPDIFRX_SR, &mut sr);
    regmap_read((*spdifrx).regmap, STM32_SPDIFRX_IMR, &mut imr);

    mask = imr & SPDIFRX_XIMR_MASK;
    /* SERR, TERR, FERR IRQs are generated if IFEIE is set */
    if (mask & SPDIFRX_IMR_IFEIE) != 0 {
        mask |= (SPDIFRX_IMR_IFEIE << 1) | (SPDIFRX_IMR_IFEIE << 2);
    }

    flags = sr & mask;
    if flags == 0 {
        dev_err(&mut (*pdev).dev, c"Unexpected IRQ. rflags=%#x, imr=%#x\n".as_ptr(), sr, imr);
        return IRQ_NONE;
    }

    /* Clear IRQs */
    regmap_update_bits((*spdifrx).regmap, STM32_SPDIFRX_IFCR, SPDIFRX_XIFCR_MASK, flags);

    if (flags & SPDIFRX_SR_PERR) != 0 {
        dev_dbg(&mut (*pdev).dev, c"Parity error\n".as_ptr());
        err_xrun = 1;
    }
    if (flags & SPDIFRX_SR_OVR) != 0 {
        dev_dbg(&mut (*pdev).dev, c"Overrun error\n".as_ptr());
        err_xrun = 1;
    }
    if (flags & SPDIFRX_SR_SBD) != 0 {
        dev_dbg(&mut (*pdev).dev, c"Synchronization block detected\n".as_ptr());
    }
    if (flags & SPDIFRX_SR_SYNCD) != 0 {
        dev_dbg(&mut (*pdev).dev, c"Synchronization done\n".as_ptr());
        cr = SPDIFRX_CR_SPDIFENSET(SPDIFRX_SPDIFEN_ENABLE);
        regmap_update_bits((*spdifrx).regmap, STM32_SPDIFRX_CR, SPDIFRX_CR_SPDIFEN_MASK, cr);
    }
    if (flags & SPDIFRX_SR_FERR) != 0 {
        dev_dbg(&mut (*pdev).dev, c"Frame error\n".as_ptr());
        err = 1;
    }
    if (flags & SPDIFRX_SR_SERR) != 0 {
        dev_dbg(&mut (*pdev).dev, c"Synchronization error\n".as_ptr());
        err = 1;
    }
    if (flags & SPDIFRX_SR_TERR) != 0 {
        dev_dbg(&mut (*pdev).dev, c"Timeout error\n".as_ptr());
        err = 1;
    }

    if err != 0 {
        regmap_read((*spdifrx).regmap, STM32_SPDIFRX_CR, &mut cr);
        sync_state = (field_get(SPDIFRX_CR_SPDIFEN_MASK, cr) != 0 && SPDIFRX_SPDIFEN_SYNC != 0) as c_uint;

        /* SPDIFRX is in STATE_STOP. Disable SPDIFRX to clear errors */
        cr = SPDIFRX_CR_SPDIFENSET(SPDIFRX_SPDIFEN_DISABLE);
        regmap_update_bits((*spdifrx).regmap, STM32_SPDIFRX_CR, SPDIFRX_CR_SPDIFEN_MASK, cr);

        /* If SPDIFRX was in STATE_SYNC, retry synchro */
        if sync_state != 0 {
            cr = SPDIFRX_CR_SPDIFENSET(SPDIFRX_SPDIFEN_SYNC);
            regmap_update_bits((*spdifrx).regmap, STM32_SPDIFRX_CR, SPDIFRX_CR_SPDIFEN_MASK, cr);
            return IRQ_HANDLED;
        }

        /* C scoped_guard(spinlock, &spdifrx->irq_lock) scope begins here. */
        if !(*spdifrx).substream.is_null() {
            snd_pcm_stop((*spdifrx).substream, SNDRV_PCM_STATE_DISCONNECTED);
        }
        return IRQ_HANDLED;
    }

    /* C scoped_guard(spinlock, &spdifrx->irq_lock) scope begins here. */
    if err_xrun != 0 && !(*spdifrx).substream.is_null() {
        snd_pcm_stop_xrun((*spdifrx).substream);
    }

    IRQ_HANDLED
}

unsafe extern "C" fn stm32_spdifrx_startup(substream: *mut snd_pcm_substream, cpu_dai: *mut snd_soc_dai) -> c_int {
    let spdifrx = snd_soc_dai_get_drvdata(cpu_dai);
    let ret: c_int;

    /* C scoped_guard(spinlock_irqsave, &spdifrx->irq_lock) scope begins here. */
    (*spdifrx).substream = substream;

    ret = clk_prepare_enable((*spdifrx).kclk);
    if ret != 0 {
        dev_err(&mut (*(*spdifrx).pdev).dev, c"Enable kclk failed: %d\n".as_ptr(), ret);
    }

    ret
}

unsafe extern "C" fn stm32_spdifrx_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, cpu_dai: *mut snd_soc_dai) -> c_int {
    let spdifrx = snd_soc_dai_get_drvdata(cpu_dai);
    let data_size: c_int = params_width(params);
    let fmt: c_uint;

    match data_size {
        16 => fmt = SPDIFRX_DRFMT_PACKED,
        32 => fmt = SPDIFRX_DRFMT_LEFT,
        _ => {
            dev_err(&mut (*(*spdifrx).pdev).dev, c"Unexpected data format\n".as_ptr());
            return -EINVAL;
        }
    }

    /*
     * Set buswidth to 4 bytes for all data formats.
     * Packed format: transfer 2 x 2 bytes samples
     * Left format: transfer 1 x 3 bytes samples + 1 dummy byte
     */
    (*spdifrx).dma_params.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    snd_soc_dai_init_dma_data(cpu_dai, core::ptr::null_mut(), &mut (*spdifrx).dma_params);

    regmap_update_bits((*spdifrx).regmap, STM32_SPDIFRX_CR, SPDIFRX_CR_DRFMT_MASK, SPDIFRX_CR_DRFMTSET(fmt))
}

unsafe extern "C" fn stm32_spdifrx_trigger(_substream: *mut snd_pcm_substream, cmd: c_int, cpu_dai: *mut snd_soc_dai) -> c_int {
    let spdifrx = snd_soc_dai_get_drvdata(cpu_dai);
    let mut ret: c_int = 0;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            regmap_update_bits((*spdifrx).regmap, STM32_SPDIFRX_IMR, SPDIFRX_IMR_OVRIE, SPDIFRX_IMR_OVRIE);
            regmap_update_bits((*spdifrx).regmap, STM32_SPDIFRX_CR, SPDIFRX_CR_RXDMAEN, SPDIFRX_CR_RXDMAEN);
            ret = stm32_spdifrx_start_sync(spdifrx);
        }
        SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_STOP => {
            stm32_spdifrx_stop(spdifrx);
        }
        _ => return -EINVAL,
    }

    ret
}

unsafe extern "C" fn stm32_spdifrx_shutdown(_substream: *mut snd_pcm_substream, cpu_dai: *mut snd_soc_dai) {
    let spdifrx = snd_soc_dai_get_drvdata(cpu_dai);

    /* C scoped_guard(spinlock_irqsave, &spdifrx->irq_lock) scope begins here. */
    (*spdifrx).substream = core::ptr::null_mut();

    clk_disable_unprepare((*spdifrx).kclk);
}

static stm32_spdifrx_pcm_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(stm32_spdifrx_dai_probe),
    startup: Some(stm32_spdifrx_startup),
    hw_params: Some(stm32_spdifrx_hw_params),
    trigger: Some(stm32_spdifrx_trigger),
    shutdown: Some(stm32_spdifrx_shutdown),
};

static mut stm32_spdifrx_dai: [snd_soc_dai_driver; 1] = [
    snd_soc_dai_driver {
        capture: snd_soc_pcm_stream {
            stream_name: c"CPU-Capture".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: SNDRV_PCM_FMTBIT_S32_LE | SNDRV_PCM_FMTBIT_S16_LE,
        },
        ops: &stm32_spdifrx_pcm_dai_ops,
    },
];

static stm32_spdifrx_pcm_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP,
    buffer_bytes_max: 8 * PAGE_SIZE,
    period_bytes_min: 1024,
    period_bytes_max: 4 * PAGE_SIZE,
    periods_min: 2,
    periods_max: 8,
};

static stm32_spdifrx_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"stm32-spdifrx".as_ptr(),
    legacy_dai_naming: 1,
};

unsafe extern "C" fn snd_dmaengine_pcm_prepare_slave_config() {}

static stm32_spdifrx_pcm_config: snd_dmaengine_pcm_config = snd_dmaengine_pcm_config {
    pcm_hardware: &stm32_spdifrx_pcm_hw,
    prepare_slave_config: Some(snd_dmaengine_pcm_prepare_slave_config),
};

static stm32_spdifrx_ids: [of_device_id; 2] = [
    of_device_id {
        compatible: c"st,stm32h7-spdifrx".as_ptr(),
        data: &stm32_h7_spdifrx_regmap_conf as *const regmap_config as *const c_void,
    },
    of_device_id {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

unsafe fn stm32_spdifrx_parse_of(pdev: *mut platform_device, spdifrx: *mut stm32_spdifrx_data) -> c_int {
    let np = (*pdev).dev.of_node;
    let mut res: *mut resource = core::ptr::null_mut();

    if np.is_null() {
        return -ENODEV;
    }

    (*spdifrx).regmap_conf = device_get_match_data(&mut (*pdev).dev) as *const regmap_config;
    if (*spdifrx).regmap_conf.is_null() {
        return -EINVAL;
    }

    (*spdifrx).base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR((*spdifrx).base as *const c_void) {
        return PTR_ERR((*spdifrx).base as *const c_void) as c_int;
    }

    (*spdifrx).phys_addr = (*res).start;

    (*spdifrx).kclk = devm_clk_get(&mut (*pdev).dev, c"kclk".as_ptr());
    if IS_ERR((*spdifrx).kclk as *const c_void) {
        return dev_err_probe(&mut (*pdev).dev, PTR_ERR((*spdifrx).kclk as *const c_void), c"Could not get kclk\n".as_ptr());
    }

    (*spdifrx).irq = platform_get_irq(pdev, 0);
    if (*spdifrx).irq < 0 {
        return (*spdifrx).irq;
    }

    0
}

unsafe extern "C" fn stm32_spdifrx_remove(pdev: *mut platform_device) {
    let spdifrx = platform_get_drvdata(pdev) as *mut stm32_spdifrx_data;

    if !IS_ERR((*spdifrx).ctrl_chan as *const c_void) {
        dma_release_channel((*spdifrx).ctrl_chan);
    }

    if !(*spdifrx).dmab.is_null() {
        snd_dma_free_pages((*spdifrx).dmab);
    }

    snd_dmaengine_pcm_unregister(&mut (*pdev).dev);
    snd_soc_unregister_component(&mut (*pdev).dev);
    pm_runtime_disable(&mut (*pdev).dev);
}

unsafe extern "C" fn stm32_spdifrx_probe(pdev: *mut platform_device) -> c_int {
    let mut spdifrx: *mut stm32_spdifrx_data;
    let rst: *mut reset_control;
    let mut pcm_config: *const snd_dmaengine_pcm_config = core::ptr::null();
    let mut ver: c_uint = 0;
    let mut idr: c_uint = 0;
    let mut ret: c_int;

    spdifrx = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<stm32_spdifrx_data>(), GFP_KERNEL) as *mut stm32_spdifrx_data;
    if spdifrx.is_null() {
        return -ENOMEM;
    }

    (*spdifrx).pdev = pdev;
    init_completion(&mut (*spdifrx).cs_completion);
    spin_lock_init(&mut (*spdifrx).lock);
    spin_lock_init(&mut (*spdifrx).irq_lock);

    platform_set_drvdata(pdev, spdifrx as *mut c_void);

    ret = stm32_spdifrx_parse_of(pdev, spdifrx);
    if ret != 0 {
        return ret;
    }

    (*spdifrx).regmap = devm_regmap_init_mmio_clk(&mut (*pdev).dev, c"kclk".as_ptr(), (*spdifrx).base, (*spdifrx).regmap_conf);
    if IS_ERR((*spdifrx).regmap as *const c_void) {
        return dev_err_probe(&mut (*pdev).dev, PTR_ERR((*spdifrx).regmap as *const c_void), c"Regmap init error\n".as_ptr());
    }

    ret = devm_request_irq(&mut (*pdev).dev, (*spdifrx).irq, Some(stm32_spdifrx_isr), 0, dev_name(&mut (*pdev).dev), spdifrx as *mut c_void);
    if ret != 0 {
        return ret;
    }

    rst = devm_reset_control_get_optional_exclusive(&mut (*pdev).dev, core::ptr::null());
    if IS_ERR(rst as *const c_void) {
        return dev_err_probe(&mut (*pdev).dev, PTR_ERR(rst as *const c_void), c"Reset controller error\n".as_ptr());
    }

    reset_control_assert(rst);
    udelay(2);
    reset_control_deassert(rst);

    pcm_config = &stm32_spdifrx_pcm_config;
    ret = snd_dmaengine_pcm_register(&mut (*pdev).dev, pcm_config, 0);
    if ret != 0 {
        return ret;
    }

    ret = snd_soc_register_component(&mut (*pdev).dev, &stm32_spdifrx_component, stm32_spdifrx_dai.as_mut_ptr(), stm32_spdifrx_dai.len() as c_int);
    if ret != 0 {
        snd_dmaengine_pcm_unregister(&mut (*pdev).dev);
        return ret;
    }

    ret = stm32_spdifrx_dma_ctrl_register(&mut (*pdev).dev, spdifrx);
    if ret != 0 {
        stm32_spdifrx_remove(pdev);
        return ret;
    }

    ret = regmap_read((*spdifrx).regmap, STM32_SPDIFRX_IDR, &mut idr);
    if ret != 0 {
        stm32_spdifrx_remove(pdev);
        return ret;
    }

    if idr == SPDIFRX_IPIDR_NUMBER {
        ret = regmap_read((*spdifrx).regmap, STM32_SPDIFRX_VERR, &mut ver);
        if ret != 0 {
            stm32_spdifrx_remove(pdev);
            return ret;
        }

        dev_dbg(&mut (*pdev).dev, c"SPDIFRX version: %lu.%lu registered\n".as_ptr(),
                field_get(SPDIFRX_VERR_MAJ_MASK, ver) as c_ulong,
                field_get(SPDIFRX_VERR_MIN_MASK, ver) as c_ulong);
    }

    pm_runtime_enable(&mut (*pdev).dev);

    ret
}

/* MODULE_DEVICE_TABLE(of, stm32_spdifrx_ids); */

unsafe extern "C" fn stm32_spdifrx_suspend(dev: *mut device) -> c_int {
    let spdifrx = dev_get_drvdata(dev) as *mut stm32_spdifrx_data;

    regcache_cache_only((*spdifrx).regmap, true);
    regcache_mark_dirty((*spdifrx).regmap);

    0
}

unsafe extern "C" fn stm32_spdifrx_resume(dev: *mut device) -> c_int {
    let spdifrx = dev_get_drvdata(dev) as *mut stm32_spdifrx_data;

    regcache_cache_only((*spdifrx).regmap, false);

    regcache_sync((*spdifrx).regmap)
}

static stm32_spdifrx_pm_ops: dev_pm_ops = dev_pm_ops {
    suspend: Some(stm32_spdifrx_suspend),
    resume: Some(stm32_spdifrx_resume),
};

static mut stm32_spdifrx_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"st,stm32-spdifrx".as_ptr(),
        of_match_table: stm32_spdifrx_ids.as_ptr(),
        pm: &stm32_spdifrx_pm_ops,
    },
    probe: Some(stm32_spdifrx_probe),
    remove: Some(stm32_spdifrx_remove),
};

/* module_platform_driver(stm32_spdifrx_driver); */
/* MODULE_DESCRIPTION("STM32 Soc spdifrx Interface"); */
/* MODULE_AUTHOR("Olivier Moysan, <olivier.moysan@st.com>"); */
/* MODULE_ALIAS("platform:stm32-spdifrx"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
