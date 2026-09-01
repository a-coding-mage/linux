// SPDX-License-Identifier: GPL-2.0
//
// Renesas R-Car MSIOF (Clock-Synchronized Serial Interface with FIFO) I2S driver
//
// Copyright (C) 2025 Renesas Solutions Corp.
// Author: Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
//

/*
 * [NOTE-CLOCK-MODE]
 *
 * This driver doesn't support Clock/Frame Provider Mode
 *
 * Basically MSIOF is created for SPI, but we can use it as I2S (Sound), etc. Because of it, when
 * we use it as I2S (Sound) with Provider Mode, we need to send dummy TX data even though it was
 * used for RX. Because SPI HW needs TX Clock/Frame output for RX purpose.
 * But it makes driver code complex in I2S (Sound).
 *
 * And when we use it as I2S (Sound) as Provider Mode, the clock source is [MSO clock] (= 133.33MHz)
 * SoC internal clock. It is not for 48kHz/44.1kHz base clock. Thus the output/input will not be
 * accurate sound.
 *
 * Because of these reasons, this driver doesn't support Clock/Frame Provider Mode. Use it as
 * Clock/Frame Consumer Mode.
 */

/*
 * [NOTE-RESET]
 *
 * MSIOF has TXRST/RXRST to reset FIFO, but it shouldn't be used during SYNC signal was asserted,
 * because it will be cause of HW issue.
 *
 * When MSIOF is used as Sound driver, this driver is assuming it is used as clock consumer mode
 * (= Codec is clock provider). This means, it can't control SYNC signal by itself.
 *
 * We need to use SW reset (= reset_control_xxx()) instead of TXRST/RXRST.
 */

/*
 * [NOTE-BOTH-SETTING]
 *
 * SITMDRn / SIRMDRn and some other registers should not be updated during working even though it
 * was not related the target direction (for example, do TX settings during RX is working),
 * otherwise it cause a FSERR.
 *
 * Setup both direction (Playback/Capture) in the same time.
 */

/*
 * [NOTE-R/L]
 *
 * The data of Captured might be R/L opposite.
 *
 * This driver is assuming MSIOF is used as Clock/Frame Consumer Mode, and there is a case that some
 * Codec (= Clock/Frame Provider) might output Clock/Frame before setup MSIOF. It depends on Codec
 * driver implementation.
 *
 * MSIOF will capture data without checking SYNC signal Hi/Low (= R/L).
 *
 * This means, if MSIOF RXE bit was set as 1 in case of SYNC signal was Hi (= R) timing, it will
 * start capture data since next SYNC low singla (= L). Because Linux assumes sound data is lined
 * up as R->L->R->L->..., the data R/L will be opposite.
 *
 * The only solution in this case is start CLK/SYNC *after* MSIOF settings, but it depends when and
 * how Codec driver start it.
 */

/*
 * [NOTE-FSERR]
 *
 * We can't remove all FSERR.
 *
 * Renesas have tried to minimize the occurrence of FSERR errors as much as possible, but
 * unfortunately we cannot remove them completely, because MSIOF might setup its register during
 * CLK/SYNC are inputed. It can be happen because MSIOF is working as Clock/Frame Consumer.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr::null_mut;

/* Dependencies from Linux kernel headers:
 * linux/module.h, linux/of.h, linux/of_dma.h, linux/of_graph.h,
 * linux/platform_device.h, linux/pm_runtime.h, linux/reset.h,
 * linux/spi/sh_msiof.h, sound/dmaengine_pcm.h, sound/soc.h
 */

type U32 = u32;
type U64 = u64;
type ResourceSizeT = usize;
type SndPcmUframesT = usize;
type IrqreturnT = c_uint;
type GfpT = c_uint;

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct resource {
    pub start: ResourceSizeT,
}
#[repr(C)]
pub struct reset_control {
    _private: [u8; 0],
}
#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dma_chan {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_runtime {
    pub format: c_int,
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
    pub runtime: *mut snd_pcm_runtime,
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub pcm: *mut snd_pcm,
    pub card: *mut snd_soc_card,
}
#[repr(C)]
pub struct snd_pcm {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_card {
    pub snd_card: *mut snd_card,
}
#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dma_slave_config {
    pub dst_addr: ResourceSizeT,
    pub src_addr: ResourceSizeT,
}
#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}
#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}
#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}
#[repr(C)]
pub struct snd_soc_dai_stream {
    pub rates: c_uint,
    pub formats: U64,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub auto_selectable_formats: *const U64,
    pub num_auto_selectable_formats: c_uint,
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_dai_stream,
    pub capture: snd_soc_dai_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
    pub symmetric_channels: c_uint,
    pub symmetric_sample_bits: c_uint,
}
#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub buffer_bytes_max: usize,
    pub period_bytes_min: usize,
    pub period_bytes_max: usize,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub fifo_size: usize,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> SndPcmUframesT>,
    pub pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, c_int) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

const FALSE: c_int = 0;
const TRUE: c_int = 1;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const GFP_KERNEL: GfpT = 0;
const IORESOURCE_MEM: c_uint = 0;
const IRQ_HANDLED: IrqreturnT = 1;

extern "C" {
    static SISTR_TFSERR: U32;
    static SISTR_TFOVF: U32;
    static SISTR_TFUDF: U32;
    static SISTR_RFSERR: U32;
    static SISTR_RFOVF: U32;
    static SISTR_RFUDF: U32;
    static SNDRV_PCM_RATE_8000_192000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: U64;
    static SNDRV_PCM_FMTBIT_S32_LE: U64;
    static SNDRV_PCM_STREAM_LAST: usize;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static SITMDR1: U32;
    static SIRMDR1: U32;
    static SITMDR2: U32;
    static SIRMDR2: U32;
    static SITMDR3: U32;
    static SIRMDR3: U32;
    static SIFCTR: U32;
    static SIIER: U32;
    static SISTR: U32;
    static SICTR: U32;
    static SITFDR: ResourceSizeT;
    static SIRFDR: ResourceSizeT;
    static SITMDR1_PCON: U32;
    static SIMDR1_SYNCAC: U32;
    static SIMDR1_XXSTP: U32;
    static SIMDR1_SYNCMD: U32;
    static SIMDR1_SYNCMD_LR: U32;
    static SIMDR1_DTDL: U32;
    static SIMDR2_BITLEN1: U32;
    static SIMDR2_GRP: U32;
    static SIFCTR_TFWM: U32;
    static SIFCTR_TFWM_1: U32;
    static SIFCTR_RFWM: U32;
    static SIFCTR_RFWM_1: U32;
    static SIIER_TDREQE: U32;
    static SIIER_TDMAE: U32;
    static SIIER_RDREQE: U32;
    static SIIER_RDMAE: U32;
    static SICTR_TEDG: U32;
    static SICTR_REDG: U32;
    static SICTR_TXE: U32;
    static SICTR_RXE: U32;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_BC_FC: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_POSSIBLE_DAIFMT_I2S: U64;
    static SND_SOC_POSSIBLE_DAIFMT_LEFT_J: U64;
    static SND_SOC_POSSIBLE_DAIFMT_NB_NF: U64;
    static SNDRV_PCM_INFO_INTERLEAVED: c_uint;
    static SNDRV_PCM_INFO_MMAP: c_uint;
    static SNDRV_PCM_INFO_MMAP_VALID: c_uint;
    static SNDRV_DMA_TYPE_DEV: c_uint;
    static SNDRV_PCM_HW_PARAM_PERIODS: c_uint;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;

    fn ioread32(addr: *mut c_void) -> U32;
    fn iowrite32(val: U32, addr: *mut c_void);
    fn readl_poll_timeout_atomic(addr: *mut c_void, data: *mut U32, mask: U32, expect: U32, delay_us: c_uint, timeout_us: c_uint) -> c_int;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn reset_control_deassert(rstc: *mut reset_control) -> c_int;
    fn reset_control_assert(rstc: *mut reset_control) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn snd_dmaengine_pcm_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int;
    fn snd_pcm_direction_name(stream: c_int) -> *const c_char;
    fn of_dma_request_slave_channel(np: *mut device_node, name: *const c_char) -> *mut dma_chan;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn snd_dmaengine_pcm_open(substream: *mut snd_pcm_substream, chan: *mut dma_chan) -> c_int;
    fn snd_soc_set_runtime_hwparams(substream: *mut snd_pcm_substream, hw: *const snd_pcm_hardware);
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, var: c_uint) -> c_int;
    fn dma_release_channel(chan: *mut dma_chan);
    fn snd_dmaengine_pcm_close_release_chan(substream: *mut snd_pcm_substream) -> c_int;
    fn snd_dmaengine_pcm_pointer(substream: *mut snd_pcm_substream) -> SndPcmUframesT;
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, ty: c_uint, dev: *mut device, size: usize, max: usize);
    fn snd_dmaengine_pcm_get_chan(substream: *mut snd_pcm_substream) -> *mut dma_chan;
    fn snd_hwparams_to_dma_slave_config(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, cfg: *mut dma_slave_config) -> c_int;
    fn dmaengine_slave_config(chan: *mut dma_chan, cfg: *mut dma_slave_config) -> c_int;
    fn of_graph_get_next_port(parent: *mut device_node, prev: *mut device_node) -> *mut device_node;
    fn platform_get_resource(pdev: *mut platform_device, ty: c_uint, num: c_uint) -> *mut resource;
    fn platform_get_irq(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: GfpT) -> *mut c_void;
    fn devm_ioremap_resource(dev: *mut device, res: *mut resource) -> *mut c_void;
    fn devm_reset_control_get_exclusive(dev: *mut device, id: *const c_char) -> *mut reset_control;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> IrqreturnT>, flags: c_uint, name: *const c_char, data: *mut c_void) -> c_int;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t) -> usize;
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_pm_runtime_enable(dev: *mut device) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn module_platform_driver(driver: *mut platform_driver);
}

const unsafe fn sistr_err_tx() -> U32 {
    SISTR_TFSERR | SISTR_TFOVF | SISTR_TFUDF
}

const unsafe fn sistr_err_rx() -> U32 {
    SISTR_RFSERR | SISTR_RFOVF | SISTR_RFUDF
}

/*
 * The data on memory in 24bit case is located at <right> side
 *	[  xxxxxx]
 *	[  xxxxxx]
 *	[  xxxxxx]
 *
 * HW assuming signal in 24bit case is located at <left> side
 *	---+         +---------+
 *	   +---------+         +---------+...
 *	   [xxxxxx  ][xxxxxx  ][xxxxxx  ]
 *
 * When we use 24bit data, it will be transferred via 32bit width via DMA,
 * and MSIOF/DMA doesn't support data shift, we can't use 24bit data correctly.
 * There is no such issue on 16/32bit data case.
 */
const unsafe fn msiof_rates() -> c_uint {
    SNDRV_PCM_RATE_8000_192000
}

const unsafe fn msiof_fmts() -> U64 {
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE
}

#[repr(C)]
struct msiof_priv {
    dev: *mut device,
    substream: [*mut snd_pcm_substream; 2],
    reset: *mut reset_control,
    lock: spinlock_t,
    base: *mut c_void,
    phy_addr: ResourceSizeT,

    count: c_int,

    /* for error */
    err_syc: [c_int; 2],
    err_ovf: [c_int; 2],
    err_udf: [c_int; 2],

    /* bit field */
    flags: U32,
}

const MSIOF_FLAGS_NEED_DELAY: U32 = 1 << 0;

unsafe fn msiof_flag_has(priv_: *mut msiof_priv, flag: U32) -> U32 {
    (*priv_).flags & flag
}

unsafe fn msiof_flag_set(priv_: *mut msiof_priv, flag: U32) {
    (*priv_).flags |= flag;
}

unsafe fn msiof_is_play(substream: *mut snd_pcm_substream) -> bool {
    (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK
}

unsafe fn msiof_reg(priv_: *mut msiof_priv, reg: U32) -> *mut c_void {
    ((*priv_).base as *mut u8).add(reg as usize) as *mut c_void
}

unsafe fn msiof_read(priv_: *mut msiof_priv, reg: U32) -> U32 {
    ioread32(msiof_reg(priv_, reg))
}

unsafe fn msiof_write(priv_: *mut msiof_priv, reg: U32, val: U32) {
    iowrite32(val, msiof_reg(priv_, reg));
}

unsafe fn FIELD_PREP(mask: U32, val: U32) -> U32 {
    /* External Linux FIELD_PREP intent; bit placement depends on the mask definition. */
    (val << mask.trailing_zeros()) & mask
}

unsafe extern "C" fn msiof_update(priv_: *mut msiof_priv, reg: U32, mask: U32, val: U32) -> c_int {
    let old = msiof_read(priv_, reg);
    let new = (old & !mask) | (val & mask);
    let mut updated = FALSE;

    if old != new {
        msiof_write(priv_, reg, new);
        updated = TRUE;
    }

    updated
}

unsafe extern "C" fn msiof_update_and_wait(priv_: *mut msiof_priv, reg: U32, mask: U32, val: U32, expect: U32) {
    let mut data: U32 = 0;
    let mut ret: c_int;

    ret = msiof_update(priv_, reg, mask, val);
    if ret == 0 {
        /* no update */
        return;
    }

    ret = readl_poll_timeout_atomic(msiof_reg(priv_, reg), &mut data, mask, expect, 1, 128);
    if ret != 0 {
        dev_warn((*priv_).dev, b"write timeout [0x%02x] 0x%08x / 0x%08x\n\0".as_ptr() as *const c_char, reg, data, expect);
    }
}

unsafe extern "C" fn msiof_hw_start(component: *mut snd_soc_component, substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut msiof_priv;
    let runtime = (*substream).runtime;
    let is_play = msiof_is_play(substream);
    let width = snd_pcm_format_width((*runtime).format);
    let mut val: U32;

    /*
     * see
     *	[NOTE-CLOCK-MODE] on top of this driver
     */
    /*
     * see
     *	Datasheet 109.3.6 [Transmit and Receive Procedures]
     *
     *	TX: Fig 109.14	- Fig 109.23
     *	RX: Fig 109.15
     */

    /*
     * Use reset_control_xx() instead of TXRST/RXRST.
     * see
     *	[NOTE-RESET]
     */
    if (*priv_).count == 0 {
        reset_control_deassert((*priv_).reset);
    }

    (*priv_).count += 1;

    /*
     * Reset errors. ignore 1st FSERR
     *
     * see
     *	[NOTE-FSERR]
     */
    (*priv_).err_syc[(*substream).stream as usize] = -1;
    (*priv_).err_ovf[(*substream).stream as usize] = 0;
    (*priv_).err_udf[(*substream).stream as usize] = 0;

    /* Start DMAC */
    snd_dmaengine_pcm_trigger(substream, cmd);

    /*
     * setup both direction (Playback/Capture) in the same time.
     * see
     *	above [NOTE-BOTH-SETTING]
     */

    /* SITMDRx */
    val = SITMDR1_PCON | SIMDR1_SYNCAC | SIMDR1_XXSTP | FIELD_PREP(SIMDR1_SYNCMD, SIMDR1_SYNCMD_LR);
    if msiof_flag_has(priv_, MSIOF_FLAGS_NEED_DELAY) != 0 {
        val |= FIELD_PREP(SIMDR1_DTDL, 1);
    }

    msiof_write(priv_, SITMDR1, val);

    val = FIELD_PREP(SIMDR2_BITLEN1, (width - 1) as U32);
    msiof_write(priv_, SITMDR2, val | FIELD_PREP(SIMDR2_GRP, 1));
    msiof_write(priv_, SITMDR3, val);

    /* SIRMDRx */
    val = SIMDR1_SYNCAC | FIELD_PREP(SIMDR1_SYNCMD, SIMDR1_SYNCMD_LR);
    if msiof_flag_has(priv_, MSIOF_FLAGS_NEED_DELAY) != 0 {
        val |= FIELD_PREP(SIMDR1_DTDL, 1);
    }

    msiof_write(priv_, SIRMDR1, val);

    val = FIELD_PREP(SIMDR2_BITLEN1, (width - 1) as U32);
    msiof_write(priv_, SIRMDR2, val | FIELD_PREP(SIMDR2_GRP, 1));
    msiof_write(priv_, SIRMDR3, val);

    /* SIFCTR */
    msiof_write(priv_, SIFCTR, FIELD_PREP(SIFCTR_TFWM, SIFCTR_TFWM_1) | FIELD_PREP(SIFCTR_RFWM, SIFCTR_RFWM_1));

    /* SIIER */
    if is_play {
        val = SIIER_TDREQE | SIIER_TDMAE | sistr_err_tx();
    } else {
        val = SIIER_RDREQE | SIIER_RDMAE | sistr_err_rx();
    }
    msiof_update(priv_, SIIER, val, val);

    /* clear status */
    if is_play {
        val = sistr_err_tx();
    } else {
        val = sistr_err_rx();
    }
    msiof_update(priv_, SISTR, val, val);

    /* SICTR */
    val = SICTR_TEDG | SICTR_REDG;
    if is_play {
        val |= SICTR_TXE;
    } else {
        val |= SICTR_RXE;
    }
    msiof_update_and_wait(priv_, SICTR, val, val, val);

    0
}

unsafe extern "C" fn msiof_hw_stop(component: *mut snd_soc_component, substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut msiof_priv;
    let dev = (*component).dev;
    let is_play = msiof_is_play(substream);
    let mut val: U32;

    /* SIIER */
    if is_play {
        val = SIIER_TDREQE | SIIER_TDMAE | sistr_err_tx();
    } else {
        val = SIIER_RDREQE | SIIER_RDMAE | sistr_err_rx();
    }
    msiof_update(priv_, SIIER, val, 0);

    /* SICTR */
    if is_play {
        val = SICTR_TXE;
    } else {
        val = SICTR_RXE;
    }
    msiof_update_and_wait(priv_, SICTR, val, 0, 0);

    /* Stop DMAC */
    snd_dmaengine_pcm_trigger(substream, cmd);

    /*
     * Ignore 1st FSERR
     *
     * see
     *	[NOTE-FSERR]
     */
    if (*priv_).err_syc[(*substream).stream as usize] < 0 {
        (*priv_).err_syc[(*substream).stream as usize] = 0;
    }

    /* indicate error status if exist */
    if (*priv_).err_syc[(*substream).stream as usize] != 0
        || (*priv_).err_ovf[(*substream).stream as usize] != 0
        || (*priv_).err_udf[(*substream).stream as usize] != 0
    {
        dev_warn(
            dev,
            b"%s: FSERR = %d, FOVF = %d, FUDF = %d\n\0".as_ptr() as *const c_char,
            snd_pcm_direction_name((*substream).stream),
            (*priv_).err_syc[(*substream).stream as usize],
            (*priv_).err_ovf[(*substream).stream as usize],
            (*priv_).err_udf[(*substream).stream as usize],
        );
    }

    (*priv_).count -= 1;

    if (*priv_).count == 0 {
        reset_control_assert((*priv_).reset);
    }

    0
}

unsafe extern "C" fn msiof_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(dai) as *mut msiof_priv;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        /*
         * It supports Clock/Frame Consumer Mode only
         * see
         *	[NOTE] on top of this driver
         */
        x if x == SND_SOC_DAIFMT_BC_FC => {}
        /* others are error */
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        /* it supports NB_NF only */
        x if x == SND_SOC_DAIFMT_NB_NF => {}
        x if x == SND_SOC_DAIFMT_NB_IF || x == SND_SOC_DAIFMT_IB_NF || x == SND_SOC_DAIFMT_IB_IF => return -EINVAL,
        _ => {}
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => {
            msiof_flag_set(priv_, MSIOF_FLAGS_NEED_DELAY);
        }
        x if x == SND_SOC_DAIFMT_LEFT_J => {}
        _ => return -EINVAL,
    }

    0
}

static mut MSIOF_DAI_FORMATS: U64 = 0;

static mut MSIOF_DAI_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(msiof_dai_set_fmt),
    auto_selectable_formats: unsafe { &MSIOF_DAI_FORMATS as *const U64 },
    num_auto_selectable_formats: 1,
};

static mut MSIOF_DAI_DRIVER: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"msiof-dai\0".as_ptr() as *const c_char,
    playback: snd_soc_dai_stream {
        rates: 0,
        formats: 0,
        channels_min: 2,
        channels_max: 2,
    },
    capture: snd_soc_dai_stream {
        rates: 0,
        formats: 0,
        channels_min: 2,
        channels_max: 2,
    },
    ops: unsafe { &MSIOF_DAI_OPS as *const snd_soc_dai_ops },
    symmetric_rate: 1,
    symmetric_channels: 1,
    symmetric_sample_bits: 1,
};

static mut MSIOF_PCM_HARDWARE: snd_pcm_hardware = snd_pcm_hardware {
    info: 0,
    buffer_bytes_max: 64 * 1024,
    period_bytes_min: 32,
    period_bytes_max: 8192,
    periods_min: 1,
    periods_max: 32,
    fifo_size: 64,
};

unsafe extern "C" fn msiof_open(component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> c_int {
    let dev = (*component).dev;
    let mut chan: *mut dma_chan;
    static DMA_NAMES: [*const c_char; 2] = [
        b"rx\0".as_ptr() as *const c_char,
        b"tx\0".as_ptr() as *const c_char,
    ];
    let is_play = msiof_is_play(substream) as usize;
    let mut ret: c_int;

    chan = of_dma_request_slave_channel((*dev).of_node, DMA_NAMES[is_play]);
    if IS_ERR(chan as *const c_void) {
        return PTR_ERR(chan as *const c_void);
    }

    ret = snd_dmaengine_pcm_open(substream, chan);
    if ret < 0 {
        goto_open_err_dma(chan, ret)
    } else {
        snd_soc_set_runtime_hwparams(substream, &MSIOF_PCM_HARDWARE as *const snd_pcm_hardware);

        ret = snd_pcm_hw_constraint_integer((*substream).runtime, SNDRV_PCM_HW_PARAM_PERIODS);

        if ret < 0 {
            dma_release_channel(chan);
        }

        ret
    }
}

unsafe fn goto_open_err_dma(chan: *mut dma_chan, ret: c_int) -> c_int {
    if ret < 0 {
        dma_release_channel(chan);
    }

    ret
}

unsafe extern "C" fn msiof_close(_component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> c_int {
    snd_dmaengine_pcm_close_release_chan(substream)
}

unsafe extern "C" fn msiof_pointer(_component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> SndPcmUframesT {
    snd_dmaengine_pcm_pointer(substream)
}

const PREALLOC_BUFFER: usize = 32 * 1024;
const PREALLOC_BUFFER_MAX: usize = 32 * 1024;

unsafe extern "C" fn msiof_new(_component: *mut snd_soc_component, rtd: *mut snd_soc_pcm_runtime) -> c_int {
    snd_pcm_set_managed_buffer_all(
        (*rtd).pcm,
        SNDRV_DMA_TYPE_DEV,
        (*(*(*rtd).card).snd_card).dev,
        PREALLOC_BUFFER,
        PREALLOC_BUFFER_MAX,
    );
    0
}

unsafe extern "C" fn msiof_trigger(component: *mut snd_soc_component, substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let dev = (*component).dev;
    let priv_ = dev_get_drvdata(dev) as *mut msiof_priv;
    let mut ret = -EINVAL;
    let flags = spin_lock_irqsave(&mut (*priv_).lock);

    match cmd {
        x if x == SNDRV_PCM_TRIGGER_START => {
            (*priv_).substream[(*substream).stream as usize] = substream;
            ret = msiof_hw_start(component, substream, cmd);
        }
        x if x == SNDRV_PCM_TRIGGER_RESUME => {
            ret = msiof_hw_start(component, substream, cmd);
        }
        x if x == SNDRV_PCM_TRIGGER_STOP => {
            (*priv_).substream[(*substream).stream as usize] = null_mut();
            ret = msiof_hw_stop(component, substream, cmd);
        }
        x if x == SNDRV_PCM_TRIGGER_SUSPEND => {
            ret = msiof_hw_stop(component, substream, cmd);
        }
        _ => {}
    }

    spin_unlock_irqrestore(&mut (*priv_).lock, flags);
    ret
}

unsafe extern "C" fn msiof_hw_params(component: *mut snd_soc_component, substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    let priv_ = dev_get_drvdata((*component).dev) as *mut msiof_priv;
    let chan = snd_dmaengine_pcm_get_chan(substream);
    let mut cfg = dma_slave_config {
        dst_addr: 0,
        src_addr: 0,
    };
    let mut ret: c_int;
    let flags = spin_lock_irqsave(&mut (*priv_).lock);

    ret = snd_hwparams_to_dma_slave_config(substream, params, &mut cfg);
    if ret < 0 {
        spin_unlock_irqrestore(&mut (*priv_).lock, flags);
        return ret;
    }

    cfg.dst_addr = (*priv_).phy_addr + SITFDR;
    cfg.src_addr = (*priv_).phy_addr + SIRFDR;

    ret = dmaengine_slave_config(chan, &mut cfg);
    spin_unlock_irqrestore(&mut (*priv_).lock, flags);
    ret
}

static MSIOF_COMPONENT_DRIVER: snd_soc_component_driver = snd_soc_component_driver {
    name: b"msiof\0".as_ptr() as *const c_char,
    open: Some(msiof_open),
    close: Some(msiof_close),
    pointer: Some(msiof_pointer),
    pcm_new: Some(msiof_new),
    trigger: Some(msiof_trigger),
    hw_params: Some(msiof_hw_params),
};

unsafe extern "C" fn msiof_interrupt(_irq: c_int, data: *mut c_void) -> IrqreturnT {
    let priv_ = data as *mut msiof_priv;
    let mut substream: *mut snd_pcm_substream;
    let sistr: U32;

    spin_lock(&mut (*priv_).lock);
    sistr = msiof_read(priv_, SISTR);
    msiof_write(priv_, SISTR, sistr_err_tx() | sistr_err_rx());
    spin_unlock(&mut (*priv_).lock);

    /* overflow/underflow error */
    substream = (*priv_).substream[SNDRV_PCM_STREAM_PLAYBACK as usize];
    if !substream.is_null() && (sistr & sistr_err_tx()) != 0 {
        // snd_pcm_stop_xrun(substream);
        if (sistr & SISTR_TFSERR) != 0 {
            (*priv_).err_syc[SNDRV_PCM_STREAM_PLAYBACK as usize] += 1;
        }
        if (sistr & SISTR_TFOVF) != 0 {
            (*priv_).err_ovf[SNDRV_PCM_STREAM_PLAYBACK as usize] += 1;
        }
        if (sistr & SISTR_TFUDF) != 0 {
            (*priv_).err_udf[SNDRV_PCM_STREAM_PLAYBACK as usize] += 1;
        }
    }

    substream = (*priv_).substream[SNDRV_PCM_STREAM_CAPTURE as usize];
    if !substream.is_null() && (sistr & sistr_err_rx()) != 0 {
        // snd_pcm_stop_xrun(substream);
        if (sistr & SISTR_RFSERR) != 0 {
            (*priv_).err_syc[SNDRV_PCM_STREAM_CAPTURE as usize] += 1;
        }
        if (sistr & SISTR_RFOVF) != 0 {
            (*priv_).err_ovf[SNDRV_PCM_STREAM_CAPTURE as usize] += 1;
        }
        if (sistr & SISTR_RFUDF) != 0 {
            (*priv_).err_udf[SNDRV_PCM_STREAM_CAPTURE as usize] += 1;
        }
    }

    IRQ_HANDLED
}

unsafe extern "C" fn msiof_probe(pdev: *mut platform_device) -> c_int {
    let mut priv_: *mut msiof_priv;
    let dev = &mut (*pdev).dev as *mut device;
    let mut res: *mut resource;
    let irq: c_int;
    let mut ret: c_int;

    /* Check MSIOF as Sound mode or SPI mode */
    let port = of_graph_get_next_port((*dev).of_node, null_mut());
    if port.is_null() {
        return -ENODEV;
    }

    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if res.is_null() {
        return -ENODEV;
    }

    irq = platform_get_irq(pdev, 0);
    if irq <= 0 {
        return irq;
    }

    priv_ = devm_kzalloc(dev, size_of::<msiof_priv>(), GFP_KERNEL) as *mut msiof_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    (*priv_).base = devm_ioremap_resource(dev, res);
    if IS_ERR((*priv_).base as *const c_void) {
        return PTR_ERR((*priv_).base as *const c_void);
    }

    (*priv_).reset = devm_reset_control_get_exclusive(dev, core::ptr::null());
    if IS_ERR((*priv_).reset as *const c_void) {
        return PTR_ERR((*priv_).reset as *const c_void);
    }

    reset_control_assert((*priv_).reset);

    ret = devm_request_irq(dev, irq, Some(msiof_interrupt), 0, dev_name(dev), priv_ as *mut c_void);
    if ret != 0 {
        return ret;
    }

    (*priv_).dev = dev;
    (*priv_).phy_addr = (*res).start;
    (*priv_).count = 0;

    spin_lock_init(&mut (*priv_).lock);
    platform_set_drvdata(pdev, priv_ as *mut c_void);

    devm_pm_runtime_enable(dev);

    MSIOF_DAI_FORMATS = SND_SOC_POSSIBLE_DAIFMT_I2S | SND_SOC_POSSIBLE_DAIFMT_LEFT_J | SND_SOC_POSSIBLE_DAIFMT_NB_NF;
    MSIOF_DAI_DRIVER.playback.rates = msiof_rates();
    MSIOF_DAI_DRIVER.playback.formats = msiof_fmts();
    MSIOF_DAI_DRIVER.capture.rates = msiof_rates();
    MSIOF_DAI_DRIVER.capture.formats = msiof_fmts();
    MSIOF_PCM_HARDWARE.info = SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID;

    ret = devm_snd_soc_register_component(dev, &MSIOF_COMPONENT_DRIVER, &mut MSIOF_DAI_DRIVER, 1);

    ret
}

static MSIOF_OF_MATCH: [of_device_id; 2] = [
    of_device_id {
        compatible: b"renesas,rcar-gen4-msiof\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];

/* MODULE_DEVICE_TABLE(of, msiof_of_match); */

static mut MSIOF_DRIVER: platform_driver = platform_driver {
    driver: device_driver {
        name: b"msiof-pcm-audio\0".as_ptr() as *const c_char,
        of_match_table: MSIOF_OF_MATCH.as_ptr(),
    },
    probe: Some(msiof_probe),
};

unsafe fn init_msiof_driver() {
    module_platform_driver(&mut MSIOF_DRIVER);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
