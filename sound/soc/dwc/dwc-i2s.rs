// SPDX-License-Identifier: GPL-2.0
/*
 * ALSA SoC Synopsys I2S Audio Layer
 *
 * sound/soc/dwc/designware_i2s.c
 *
 * Copyright (C) 2010 ST Microelectronics
 * Rajeev Kumar <rajeevkumar.linux@gmail.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type u32 = u32;
type bool_ = bool;
type irqreturn_t = c_uint;

const false_: bool_ = false;
const true_: bool_ = true;

extern "C" {
    fn writel(val: u32, addr: *mut c_void);
    fn readl(addr: *mut c_void) -> u32;
    fn dw_pcm_push_tx(dev: *mut dw_i2s_dev);
    fn dw_pcm_pop_rx(dev: *mut dw_i2s_dev);
    fn dev_err_ratelimited(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn clk_set_rate(clk: *mut clk, rate: u32) -> c_int;
    fn clk_disable(clk: *mut clk);
    fn clk_enable(clk: *mut clk) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> c_int;
    fn clk_bulk_get(dev: *mut device, num_clks: c_uint, clks: *mut clk_bulk_data) -> c_int;
    fn clk_bulk_put(num_clks: c_uint, clks: *mut clk_bulk_data);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_reset_control_array_get_exclusive(dev: *mut device) -> *mut reset_control;
    fn devm_reset_control_array_get_optional_shared(dev: *mut device) -> *mut reset_control;
    fn reset_control_deassert(rstc: *mut reset_control) -> c_int;
    fn reset_control_assert(rstc: *mut reset_control) -> c_int;
    fn syscon_regmap_lookup_by_phandle_args(
        np: *mut device_node,
        property: *const c_char,
        arg_count: c_int,
        out_args: *mut c_uint,
    ) -> *mut regmap;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut dw_i2s_dev;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut dw_i2s_dev;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_dai_stream_active(dai: *mut snd_soc_dai, stream: c_int) -> c_int;
    fn snd_soc_dai_init_dma_data(
        dai: *mut snd_soc_dai,
        playback: *mut snd_dmaengine_dai_dma_data,
        capture: *mut snd_dmaengine_dai_dma_data,
    );
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn ffs(x: c_uint) -> c_int;
    fn fls(x: c_uint) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device,
        index: c_uint,
        res: *mut *mut resource,
    ) -> *mut c_void;
    fn platform_get_irq_optional(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn devm_request_irq(
        dev: *mut device,
        irq: c_int,
        handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        flags: c_uint,
        name: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn devm_clk_get_enabled(dev: *mut device, id: *const c_char) -> *mut clk;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn dw_pcm_register(pdev: *mut platform_device) -> c_int;
    fn devm_snd_dmaengine_pcm_register(
        dev: *mut device,
        config: *const c_void,
        flags: c_uint,
    ) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
}

#[repr(C)]
pub struct device {
    pub platform_data: *const i2s_platform_data,
    pub of_node: *mut device_node,
}
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct reset_control { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_component { _private: [u8; 0] }

#[repr(C)]
pub struct platform_device {
    pub dev: device,
    pub name: *const c_char,
}

#[repr(C)]
pub struct resource {
    pub start: usize,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: u32,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dai_link: *mut snd_soc_dai_link,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub trigger_stop: c_int,
}

#[repr(C)]
pub struct i2s_clk_config_data {
    pub data_width: u32,
    pub chan_nr: u32,
    pub sample_rate: u32,
}

#[repr(C)]
pub struct dma_pd_data {
    pub data: *mut c_void,
    pub addr: usize,
    pub max_burst: u32,
    pub addr_width: u32,
    pub filter: *mut c_void,
}

#[repr(C)]
pub struct dma_dt_data {
    pub addr: usize,
    pub fifo_size: u32,
    pub maxburst: u32,
}

#[repr(C)]
pub union snd_dmaengine_dai_dma_data {
    pub pd: core::mem::ManuallyDrop<dma_pd_data>,
    pub dt: core::mem::ManuallyDrop<dma_dt_data>,
}

#[repr(C)]
pub struct dw_i2s_dev {
    pub i2s_base: *mut c_void,
    pub config: i2s_clk_config_data,
    pub use_pio: bool,
    pub is_jh7110: bool,
    pub tdm_slots: u32,
    pub frame_offset: u32,
    pub tdm_mask: u32,
    pub xfer_resolution: u32,
    pub fifo_th: u32,
    pub ccr: u32,
    pub capability: u32,
    pub quirks: u32,
    pub active: c_int,
    pub dev: *mut device,
    pub clk: *mut clk,
    pub reset: *mut reset_control,
    pub i2s_clk_cfg: Option<unsafe extern "C" fn(*mut i2s_clk_config_data) -> c_int>,
    pub i2s_reg_comp1: c_int,
    pub i2s_reg_comp2: c_int,
    pub play_dma_data: snd_dmaengine_dai_dma_data,
    pub capture_dma_data: snd_dmaengine_dai_dma_data,
    pub l_reg: u32,
    pub r_reg: u32,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub channels_min: u32,
    pub channels_max: u32,
    pub formats: u32,
    pub rates: u32,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub ops: *const snd_soc_dai_ops,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub legacy_dai_naming: c_int,
}

#[repr(C)]
pub struct clk_bulk_data {
    pub id: *const c_char,
    pub clk: *mut clk,
}

#[repr(C)]
pub struct i2s_platform_data {
    pub cap: u32,
    pub channel: u32,
    pub snd_fmts: u32,
    pub snd_rates: u32,
    pub i2s_clk_cfg: Option<unsafe extern "C" fn(*mut i2s_clk_config_data) -> c_int>,
    pub i2s_pd_init: Option<unsafe extern "C" fn(*mut dw_i2s_dev) -> c_int>,
    pub quirks: u32,
    pub i2s_reg_comp1: c_int,
    pub i2s_reg_comp2: c_int,
    pub play_dma_data: *mut c_void,
    pub capture_dma_data: *mut c_void,
    pub filter: *mut c_void,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
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

const SNDRV_PCM_STREAM_PLAYBACK: u32 = 0;
const SNDRV_PCM_STREAM_CAPTURE: u32 = 1;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQ_NONE: irqreturn_t = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const EPROBE_DEFER: c_int = 517;
const GFP_KERNEL: c_uint = 0;
const SND_SOC_TRIGGER_ORDER_LDC: c_int = 0;

const ISR_TXFE: u32 = 1 << 4;
const ISR_RXDA: u32 = 1 << 0;
const ISR_TXFO: u32 = 1 << 5;
const ISR_RXFO: u32 = 1 << 1;
const IER_IEN: u32 = 1;
const IER_TDM_SLOTS_SHIFT: u32 = 8;
const IER_INTF_TYPE: u32 = 1 << 4;
const IER_FRAME_OFF_SHIFT: u32 = 5;
const I2S_DMAEN_TXBLOCK: u32 = 1 << 17;
const I2S_DMAEN_RXBLOCK: u32 = 1 << 16;
const TER_TXCHEN: u32 = 1;
const RER_RXCHEN: u32 = 1;
const TER_TXSLOT_SHIFT: u32 = 8;
const RER_RXSLOT_SHIFT: u32 = 8;

const DW_I2S_MASTER: u32 = 1 << 0;
const DW_I2S_SLAVE: u32 = 1 << 1;
const DWC_I2S_PLAY: u32 = 1 << 2;
const DWC_I2S_RECORD: u32 = 1 << 3;
const DW_I2S_QUIRK_COMP_PARAM1: u32 = 1 << 0;
const DW_I2S_QUIRK_16BIT_IDX_OVERRIDE: u32 = 1 << 1;
const DW_I2S_QUIRK_COMP_REG_OFFSET: u32 = 1 << 2;

const SNDRV_PCM_FORMAT_S16_LE: c_int = 2;
const SNDRV_PCM_FORMAT_S24_LE: c_int = 6;
const SNDRV_PCM_FORMAT_S32_LE: c_int = 10;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0xf00;
const SND_SOC_DAIFMT_BC_FC: c_uint = 0x100;
const SND_SOC_DAIFMT_BP_FP: c_uint = 0x200;
const SND_SOC_DAIFMT_BC_FP: c_uint = 0x300;
const SND_SOC_DAIFMT_BP_FC: c_uint = 0x400;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_RIGHT_J: c_uint = 2;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 3;
const SND_SOC_DAIFMT_DSP_A: c_uint = 4;
const SND_SOC_DAIFMT_DSP_B: c_uint = 5;

const EIGHT_CHANNEL_SUPPORT: u32 = 8;
const SIX_CHANNEL_SUPPORT: u32 = 6;
const FOUR_CHANNEL_SUPPORT: u32 = 4;
const TWO_CHANNEL_SUPPORT: u32 = 2;
const MIN_CHANNEL_NUM: u32 = 2;
const COMP_MAX_WORDSIZE: usize = 8;
const COMP_MAX_DATA_WIDTH: usize = 4;

const DMA_SLAVE_BUSWIDTH_1_BYTE: u32 = 1;
const DMA_SLAVE_BUSWIDTH_2_BYTES: u32 = 2;
const DMA_SLAVE_BUSWIDTH_4_BYTES: u32 = 4;
const DMA_SLAVE_BUSWIDTH_UNDEFINED: u32 = 0;
const SNDRV_PCM_FMTBIT_S16_LE: u32 = 1 << 2;
const SNDRV_PCM_FMTBIT_S24_LE: u32 = 1 << 6;
const SNDRV_PCM_FMTBIT_S32_LE: u32 = 1 << 10;
const SNDRV_PCM_RATE_32000: u32 = 1 << 10;
const SNDRV_PCM_RATE_48000: u32 = 1 << 12;
const SNDRV_PCM_RATE_8000_192000: u32 = 0;

const IER: c_int = 0x000;
const IRER: c_int = 0x004;
const ITER: c_int = 0x008;
const CER: c_int = 0x00c;
const CCR: c_int = 0x010;
const RXFFR: c_int = 0x014;
const TXFFR: c_int = 0x018;
const I2S_DMACR: c_int = 0x200;
const I2S_RRXDMA: c_int = 0x204;
const I2S_RTXDMA: c_int = 0x208;
const I2S_COMP_PARAM_1: c_int = 0x01f4;
const I2S_COMP_PARAM_2: c_int = 0x01f0;
const I2S_TXDMA: usize = 0x01c0;
const I2S_RXDMA: usize = 0x01c8;

const fn BIT(nr: u32) -> u32 { 1u32 << nr }
const fn TCR(i: u32) -> c_int { 0x040 + (i as c_int) * 0x40 }
const fn RCR(i: u32) -> c_int { 0x044 + (i as c_int) * 0x40 }
const fn TER(i: u32) -> c_int { 0x048 + (i as c_int) * 0x40 }
const fn RER(i: u32) -> c_int { 0x04c + (i as c_int) * 0x40 }
const fn TFCR(i: u32) -> c_int { 0x050 + (i as c_int) * 0x40 }
const fn RFCR(i: u32) -> c_int { 0x054 + (i as c_int) * 0x40 }
const fn IMR(i: u32) -> c_int { 0x058 + (i as c_int) * 0x40 }
const fn ISR(i: u32) -> c_int { 0x05c + (i as c_int) * 0x40 }
const fn ROR(i: u32) -> c_int { 0x060 + (i as c_int) * 0x40 }
const fn TOR(i: u32) -> c_int { 0x064 + (i as c_int) * 0x40 }
const fn LRBR_LTHR(i: u32) -> u32 { i }
const fn RRBR_RTHR(i: u32) -> u32 { i }
const fn RSLOT_TSLOT(i: c_int) -> u32 { i as u32 }

fn IS_ERR<T>(ptr: *mut T) -> bool { (ptr as isize) < 0 && (ptr as isize) > -4096 }
fn PTR_ERR<T>(ptr: *mut T) -> c_int { ptr as isize as c_int }
fn WARN_ON(condition: bool) -> bool { condition }
fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize { N }

fn COMP1_FIFO_DEPTH_GLOBAL(x: u32) -> u32 { (x >> 2) & 0x03 }
fn COMP1_TX_ENABLED(x: u32) -> bool { (x & BIT(6)) != 0 }
fn COMP1_RX_ENABLED(x: u32) -> bool { (x & BIT(5)) != 0 }
fn COMP1_TX_WORDSIZE_0(x: u32) -> u32 { (x >> 16) & 0x7 }
fn COMP2_RX_WORDSIZE_0(x: u32) -> u32 { (x >> 0) & 0x7 }
fn COMP1_TX_CHANNELS(x: u32) -> u32 { (x >> 9) & 0x3 }
fn COMP1_RX_CHANNELS(x: u32) -> u32 { (x >> 7) & 0x3 }
fn COMP1_MODE_EN(x: u32) -> bool { (x & BIT(0)) != 0 }
fn COMP1_APB_DATA_WIDTH(x: u32) -> u32 { (x >> 0) & 0x3 }

#[inline]
unsafe fn i2s_write_reg(io_base: *mut c_void, reg: c_int, val: u32) {
    writel(val, (io_base as *mut u8).offset(reg as isize) as *mut c_void);
}

#[inline]
unsafe fn i2s_read_reg(io_base: *mut c_void, reg: c_int) -> u32 {
    readl((io_base as *mut u8).offset(reg as isize) as *mut c_void)
}

#[inline]
unsafe fn i2s_disable_channels(dev: *mut dw_i2s_dev, stream: u32) {
    let mut i: u32 = 0;

    if stream == SNDRV_PCM_STREAM_PLAYBACK {
        while i < 4 {
            i2s_write_reg((*dev).i2s_base, TER(i), 0);
            i += 1;
        }
    } else {
        while i < 4 {
            i2s_write_reg((*dev).i2s_base, RER(i), 0);
            i += 1;
        }
    }
}

#[inline]
unsafe fn i2s_clear_irqs(dev: *mut dw_i2s_dev, stream: u32) {
    let mut i: u32 = 0;

    if stream == SNDRV_PCM_STREAM_PLAYBACK {
        while i < 4 {
            i2s_read_reg((*dev).i2s_base, TOR(i));
            i += 1;
        }
    } else {
        while i < 4 {
            i2s_read_reg((*dev).i2s_base, ROR(i));
            i += 1;
        }
    }
}

#[inline]
unsafe fn i2s_disable_irqs(dev: *mut dw_i2s_dev, stream: u32, chan_nr: c_int) {
    let mut i: u32;
    let mut irq: u32;

    if stream == SNDRV_PCM_STREAM_PLAYBACK {
        i = 0;
        while i < (chan_nr / 2) as u32 {
            irq = i2s_read_reg((*dev).i2s_base, IMR(i));
            i2s_write_reg((*dev).i2s_base, IMR(i), irq | 0x30);
            i += 1;
        }
    } else {
        i = 0;
        while i < (chan_nr / 2) as u32 {
            irq = i2s_read_reg((*dev).i2s_base, IMR(i));
            i2s_write_reg((*dev).i2s_base, IMR(i), irq | 0x03);
            i += 1;
        }
    }
}

#[inline]
unsafe fn i2s_enable_irqs(dev: *mut dw_i2s_dev, stream: u32, chan_nr: c_int) {
    let mut i: u32;
    let mut irq: u32;

    if stream == SNDRV_PCM_STREAM_PLAYBACK {
        i = 0;
        while i < (chan_nr / 2) as u32 {
            irq = i2s_read_reg((*dev).i2s_base, IMR(i));
            i2s_write_reg((*dev).i2s_base, IMR(i), irq & !0x30);
            i += 1;
        }
    } else {
        i = 0;
        while i < (chan_nr / 2) as u32 {
            irq = i2s_read_reg((*dev).i2s_base, IMR(i));
            i2s_write_reg((*dev).i2s_base, IMR(i), irq & !0x03);
            i += 1;
        }
    }
}

unsafe extern "C" fn i2s_irq_handler(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let dev: *mut dw_i2s_dev = dev_id as *mut dw_i2s_dev;
    let mut irq_valid: bool_ = false_;
    let mut isr: [u32; 4] = [0; 4];
    let mut i: c_int;

    i = 0;
    while i < 4 {
        isr[i as usize] = i2s_read_reg((*dev).i2s_base, ISR(i as u32));
        i += 1;
    }

    i2s_clear_irqs(dev, SNDRV_PCM_STREAM_PLAYBACK);
    i2s_clear_irqs(dev, SNDRV_PCM_STREAM_CAPTURE);

    i = 0;
    while i < 4 {
        /*
         * Check if TX fifo is empty. If empty fill FIFO with samples
         * NOTE: Only two channels supported
         */
        if (isr[i as usize] & ISR_TXFE) != 0 && i == 0 && (*dev).use_pio {
            dw_pcm_push_tx(dev);
            irq_valid = true_;
        }

        /*
         * Data available. Retrieve samples from FIFO
         * NOTE: Only two channels supported
         */
        if (isr[i as usize] & ISR_RXDA) != 0 && i == 0 && (*dev).use_pio {
            dw_pcm_pop_rx(dev);
            irq_valid = true_;
        }

        /* Error Handling: TX */
        if (isr[i as usize] & ISR_TXFO) != 0 {
            dev_err_ratelimited((*dev).dev, b"TX overrun (ch_id=%d)\n\0".as_ptr() as *const c_char, i);
            irq_valid = true_;
        }

        /* Error Handling: TX */
        if (isr[i as usize] & ISR_RXFO) != 0 {
            dev_err_ratelimited((*dev).dev, b"RX overrun (ch_id=%d)\n\0".as_ptr() as *const c_char, i);
            irq_valid = true_;
        }
        i += 1;
    }

    if irq_valid {
        IRQ_HANDLED
    } else {
        IRQ_NONE
    }
}

unsafe fn i2s_enable_dma(dev: *mut dw_i2s_dev, stream: u32) {
    let mut dma_reg: u32 = i2s_read_reg((*dev).i2s_base, I2S_DMACR);

    /* Enable DMA handshake for stream */
    if stream == SNDRV_PCM_STREAM_PLAYBACK {
        dma_reg |= I2S_DMAEN_TXBLOCK;
    } else {
        dma_reg |= I2S_DMAEN_RXBLOCK;
    }

    i2s_write_reg((*dev).i2s_base, I2S_DMACR, dma_reg);
}

unsafe fn i2s_disable_dma(dev: *mut dw_i2s_dev, stream: u32) {
    let mut dma_reg: u32 = i2s_read_reg((*dev).i2s_base, I2S_DMACR);

    /* Disable DMA handshake for stream */
    if stream == SNDRV_PCM_STREAM_PLAYBACK {
        dma_reg &= !I2S_DMAEN_TXBLOCK;
        i2s_write_reg((*dev).i2s_base, I2S_RTXDMA, 1);
    } else {
        dma_reg &= !I2S_DMAEN_RXBLOCK;
        i2s_write_reg((*dev).i2s_base, I2S_RRXDMA, 1);
    }
    i2s_write_reg((*dev).i2s_base, I2S_DMACR, dma_reg);
}

unsafe fn i2s_start(dev: *mut dw_i2s_dev, substream: *mut snd_pcm_substream) {
    let config: *mut i2s_clk_config_data = &mut (*dev).config;
    let mut reg: u32 = IER_IEN;

    if (*dev).tdm_slots != 0 {
        reg |= ((*dev).tdm_slots - 1) << IER_TDM_SLOTS_SHIFT;
        reg |= IER_INTF_TYPE;
        reg |= (*dev).frame_offset << IER_FRAME_OFF_SHIFT;
    }

    i2s_write_reg((*dev).i2s_base, IER, reg);

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        i2s_write_reg((*dev).i2s_base, ITER, 1);
    } else {
        i2s_write_reg((*dev).i2s_base, IRER, 1);
    }

    if !((*dev).use_pio || (*dev).is_jh7110) {
        i2s_enable_dma(dev, (*substream).stream);
    }

    i2s_enable_irqs(dev, (*substream).stream, (*config).chan_nr as c_int);
    i2s_write_reg((*dev).i2s_base, CER, 1);
}

unsafe fn i2s_stop(dev: *mut dw_i2s_dev, substream: *mut snd_pcm_substream) {
    i2s_clear_irqs(dev, (*substream).stream);
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        i2s_write_reg((*dev).i2s_base, ITER, 0);
    } else {
        i2s_write_reg((*dev).i2s_base, IRER, 0);
    }

    if !((*dev).use_pio || (*dev).is_jh7110) {
        i2s_disable_dma(dev, (*substream).stream);
    }

    i2s_disable_irqs(dev, (*substream).stream, 8);

    if (*dev).active == 0 {
        i2s_write_reg((*dev).i2s_base, CER, 0);
        i2s_write_reg((*dev).i2s_base, IER, 0);
    }
}

unsafe extern "C" fn dw_i2s_startup(
    substream: *mut snd_pcm_substream,
    cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let dev: *mut dw_i2s_dev = snd_soc_dai_get_drvdata(cpu_dai);

    if (*dev).is_jh7110 {
        let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
        let dai_link: *mut snd_soc_dai_link = (*rtd).dai_link;

        (*dai_link).trigger_stop = SND_SOC_TRIGGER_ORDER_LDC;
    }

    0
}

unsafe fn dw_i2s_config(dev: *mut dw_i2s_dev, stream: c_int) {
    let mut ch_reg: u32;
    let config: *mut i2s_clk_config_data = &mut (*dev).config;

    i2s_disable_channels(dev, stream as u32);

    ch_reg = 0;
    while ch_reg < ((*config).chan_nr / 2) {
        if stream as u32 == SNDRV_PCM_STREAM_PLAYBACK {
            i2s_write_reg((*dev).i2s_base, TCR(ch_reg), (*dev).xfer_resolution);
            i2s_write_reg((*dev).i2s_base, TFCR(ch_reg), (*dev).fifo_th - 1);
            i2s_write_reg(
                (*dev).i2s_base,
                TER(ch_reg),
                TER_TXCHEN | ((*dev).tdm_mask << TER_TXSLOT_SHIFT),
            );
        } else {
            i2s_write_reg((*dev).i2s_base, RCR(ch_reg), (*dev).xfer_resolution);
            i2s_write_reg((*dev).i2s_base, RFCR(ch_reg), (*dev).fifo_th - 1);
            i2s_write_reg(
                (*dev).i2s_base,
                RER(ch_reg),
                RER_RXCHEN | ((*dev).tdm_mask << RER_RXSLOT_SHIFT),
            );
        }
        ch_reg += 1;
    }
}

unsafe extern "C" fn dw_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let dev: *mut dw_i2s_dev = snd_soc_dai_get_drvdata(dai);
    let config: *mut i2s_clk_config_data = &mut (*dev).config;
    let mut ret: c_int;

    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => {
            (*config).data_width = 16;
            (*dev).ccr = 0x00;
            (*dev).xfer_resolution = 0x02;
        }
        SNDRV_PCM_FORMAT_S24_LE => {
            (*config).data_width = 24;
            (*dev).ccr = 0x08;
            (*dev).xfer_resolution = 0x04;
        }
        SNDRV_PCM_FORMAT_S32_LE => {
            (*config).data_width = 32;
            (*dev).ccr = 0x10;
            (*dev).xfer_resolution = 0x05;
        }
        _ => {
            dev_err((*dev).dev, b"designware-i2s: unsupported PCM fmt\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    if (*dev).tdm_slots != 0 {
        (*config).data_width = 32;
    }

    (*config).chan_nr = params_channels(params);

    match (*config).chan_nr {
        EIGHT_CHANNEL_SUPPORT | SIX_CHANNEL_SUPPORT | FOUR_CHANNEL_SUPPORT | TWO_CHANNEL_SUPPORT => {}
        _ => {
            dev_err((*dev).dev, b"channel not supported\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    dw_i2s_config(dev, (*substream).stream as c_int);
    i2s_write_reg((*dev).i2s_base, CCR, (*dev).ccr);
    (*config).sample_rate = params_rate(params);

    if ((*dev).capability & DW_I2S_MASTER) != 0 {
        if let Some(i2s_clk_cfg) = (*dev).i2s_clk_cfg {
            ret = i2s_clk_cfg(config);
            if ret < 0 {
                dev_err((*dev).dev, b"runtime audio clk config fail\n\0".as_ptr() as *const c_char);
                return ret;
            }
        } else {
            let bitclk: u32 = (*config).sample_rate * (*config).data_width * 2;
            ret = clk_set_rate((*dev).clk, bitclk);
            if ret != 0 {
                dev_err((*dev).dev, b"Can't set I2S clock rate: %d\n\0".as_ptr() as *const c_char, ret);
                return ret;
            }
        }
    }
    0
}

unsafe extern "C" fn dw_i2s_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let dev: *mut dw_i2s_dev = snd_soc_dai_get_drvdata(dai);

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        i2s_write_reg((*dev).i2s_base, TXFFR, 1);
    } else {
        i2s_write_reg((*dev).i2s_base, RXFFR, 1);
    }

    0
}

unsafe extern "C" fn dw_i2s_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let dev: *mut dw_i2s_dev = snd_soc_dai_get_drvdata(dai);
    let mut ret: c_int = 0;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            (*dev).active += 1;
            i2s_start(dev, substream);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            (*dev).active -= 1;
            i2s_stop(dev, substream);
        }
        _ => {
            ret = -EINVAL;
        }
    }
    ret
}

unsafe extern "C" fn dw_i2s_set_fmt(cpu_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let dev: *mut dw_i2s_dev = snd_soc_dai_get_drvdata(cpu_dai);
    let mut ret: c_int = 0;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BC_FC => {
            if ((*dev).capability & DW_I2S_SLAVE) != 0 { ret = 0; } else { ret = -EINVAL; }
        }
        SND_SOC_DAIFMT_BP_FP => {
            if ((*dev).capability & DW_I2S_MASTER) != 0 { ret = 0; } else { ret = -EINVAL; }
        }
        SND_SOC_DAIFMT_BC_FP | SND_SOC_DAIFMT_BP_FC => ret = -EINVAL,
        _ => {
            dev_dbg((*dev).dev, b"dwc : Invalid clock provider format\n\0".as_ptr() as *const c_char);
            ret = -EINVAL;
        }
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_RIGHT_J => {}
        SND_SOC_DAIFMT_DSP_A => (*dev).frame_offset = 1,
        SND_SOC_DAIFMT_DSP_B => (*dev).frame_offset = 0,
        _ => {
            dev_err((*dev).dev, b"DAI format unsupported\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    ret
}

unsafe extern "C" fn dw_i2s_set_tdm_slot(
    cpu_dai: *mut snd_soc_dai,
    tx_mask: c_uint,
    rx_mask: c_uint,
    slots: c_int,
    slot_width: c_int,
) -> c_int {
    let dev: *mut dw_i2s_dev = snd_soc_dai_get_drvdata(cpu_dai);

    if slot_width != 32 { return -EINVAL; }
    if slots < 0 || slots > 16 { return -EINVAL; }
    if rx_mask != tx_mask { return -EINVAL; }
    if rx_mask == 0 { return -EINVAL; }

    (*dev).tdm_slots = slots as u32;
    (*dev).tdm_mask = rx_mask;
    (*dev).l_reg = RSLOT_TSLOT(ffs(rx_mask) - 1);
    (*dev).r_reg = RSLOT_TSLOT(fls(rx_mask) - 1);

    0
}

unsafe extern "C" fn dw_i2s_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let dev: *mut dw_i2s_dev = snd_soc_dai_get_drvdata(dai);

    snd_soc_dai_init_dma_data(dai, &mut (*dev).play_dma_data, &mut (*dev).capture_dma_data);
    0
}

static dw_i2s_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(dw_i2s_dai_probe),
    startup: Some(dw_i2s_startup),
    hw_params: Some(dw_i2s_hw_params),
    prepare: Some(dw_i2s_prepare),
    trigger: Some(dw_i2s_trigger),
    set_fmt: Some(dw_i2s_set_fmt),
    set_tdm_slot: Some(dw_i2s_set_tdm_slot),
};

unsafe extern "C" fn dw_i2s_runtime_suspend(dev: *mut device) -> c_int {
    let dw_dev: *mut dw_i2s_dev = dev_get_drvdata(dev) as *mut dw_i2s_dev;

    if ((*dw_dev).capability & DW_I2S_MASTER) != 0 {
        clk_disable((*dw_dev).clk);
    }
    0
}

unsafe extern "C" fn dw_i2s_runtime_resume(dev: *mut device) -> c_int {
    let dw_dev: *mut dw_i2s_dev = dev_get_drvdata(dev) as *mut dw_i2s_dev;
    let ret: c_int;

    if ((*dw_dev).capability & DW_I2S_MASTER) != 0 {
        ret = clk_enable((*dw_dev).clk);
        if ret != 0 { return ret; }
    }
    0
}

/* CONFIG_PM */
unsafe extern "C" fn dw_i2s_suspend(component: *mut snd_soc_component) -> c_int {
    let dev: *mut dw_i2s_dev = snd_soc_component_get_drvdata(component);

    if ((*dev).capability & DW_I2S_MASTER) != 0 {
        clk_disable((*dev).clk);
    }
    0
}

unsafe extern "C" fn dw_i2s_resume(component: *mut snd_soc_component) -> c_int {
    let dev: *mut dw_i2s_dev = snd_soc_component_get_drvdata(component);
    let mut dai: *mut snd_soc_dai = ptr::null_mut();
    let mut stream: c_int;
    let ret: c_int;

    if ((*dev).capability & DW_I2S_MASTER) != 0 {
        ret = clk_enable((*dev).clk);
        if ret != 0 { return ret; }
    }

    /* for_each_component_dais(component, dai) */
    /* for_each_pcm_streams(stream) */
    stream = 0;
    while stream <= 1 {
        if snd_soc_dai_stream_active(dai, stream) != 0 {
            dw_i2s_config(dev, stream);
        }
        stream += 1;
    }

    0
}

static dw_i2s_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"dw-i2s\0".as_ptr() as *const c_char,
    suspend: Some(dw_i2s_suspend),
    resume: Some(dw_i2s_resume),
    legacy_dai_naming: 1,
};

/*
 * The following tables allow a direct lookup of various parameters
 * defined in the I2S block's configuration in terms of sound system
 * parameters.  Each table is sized to the number of entries possible
 * according to the number of configuration bits describing an I2S
 * block parameter.
 */

/* Maximum bit resolution of a channel - not uniformly spaced */
static fifo_width: [u32; COMP_MAX_WORDSIZE] = [12, 16, 20, 24, 32, 0, 0, 0];

/* Width of (DMA) bus */
static bus_widths: [u32; COMP_MAX_DATA_WIDTH] = [
    DMA_SLAVE_BUSWIDTH_1_BYTE,
    DMA_SLAVE_BUSWIDTH_2_BYTES,
    DMA_SLAVE_BUSWIDTH_4_BYTES,
    DMA_SLAVE_BUSWIDTH_UNDEFINED,
];

/* PCM format to support channel resolution */
static formats: [u32; COMP_MAX_WORDSIZE] = [
    SNDRV_PCM_FMTBIT_S16_LE,
    SNDRV_PCM_FMTBIT_S16_LE,
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE,
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE,
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
    0,
    0,
    0,
];

unsafe fn dw_configure_dai(
    dev: *mut dw_i2s_dev,
    dw_i2s_dai: *mut snd_soc_dai_driver,
    rates: c_uint,
) -> c_int {
    /*
     * Read component parameter registers to extract
     * the I2S block's configuration.
     */
    let mut comp1: u32 = i2s_read_reg((*dev).i2s_base, (*dev).i2s_reg_comp1);
    let comp2: u32 = i2s_read_reg((*dev).i2s_base, (*dev).i2s_reg_comp2);
    let fifo_depth: u32 = 1 << (1 + COMP1_FIFO_DEPTH_GLOBAL(comp1));
    let mut idx: u32;

    if ((*dev).capability & DWC_I2S_RECORD) != 0 && ((*dev).quirks & DW_I2S_QUIRK_COMP_PARAM1) != 0 {
        comp1 = comp1 & !BIT(5);
    }

    if ((*dev).capability & DWC_I2S_PLAY) != 0 && ((*dev).quirks & DW_I2S_QUIRK_COMP_PARAM1) != 0 {
        comp1 = comp1 & !BIT(6);
    }

    if COMP1_TX_ENABLED(comp1) {
        dev_dbg((*dev).dev, b" designware: play supported\n\0".as_ptr() as *const c_char);
        idx = COMP1_TX_WORDSIZE_0(comp1);
        if WARN_ON(idx as usize >= ARRAY_SIZE(&formats)) {
            return -EINVAL;
        }
        if ((*dev).quirks & DW_I2S_QUIRK_16BIT_IDX_OVERRIDE) != 0 {
            idx = 1;
        }
        (*dw_i2s_dai).playback.channels_min = MIN_CHANNEL_NUM;
        (*dw_i2s_dai).playback.channels_max = 1 << (COMP1_TX_CHANNELS(comp1) + 1);
        (*dw_i2s_dai).playback.formats = formats[idx as usize];
        (*dw_i2s_dai).playback.rates = rates;
    }

    if COMP1_RX_ENABLED(comp1) {
        dev_dbg((*dev).dev, b"designware: record supported\n\0".as_ptr() as *const c_char);
        idx = COMP2_RX_WORDSIZE_0(comp2);
        if WARN_ON(idx as usize >= ARRAY_SIZE(&formats)) {
            return -EINVAL;
        }
        if ((*dev).quirks & DW_I2S_QUIRK_16BIT_IDX_OVERRIDE) != 0 {
            idx = 1;
        }
        (*dw_i2s_dai).capture.channels_min = MIN_CHANNEL_NUM;
        (*dw_i2s_dai).capture.channels_max = 1 << (COMP1_RX_CHANNELS(comp1) + 1);
        (*dw_i2s_dai).capture.formats = formats[idx as usize];
        (*dw_i2s_dai).capture.rates = rates;
    }

    if COMP1_MODE_EN(comp1) {
        dev_dbg((*dev).dev, b"designware: i2s master mode supported\n\0".as_ptr() as *const c_char);
        (*dev).capability |= DW_I2S_MASTER;
    } else {
        dev_dbg((*dev).dev, b"designware: i2s slave mode supported\n\0".as_ptr() as *const c_char);
        (*dev).capability |= DW_I2S_SLAVE;
    }

    (*dev).fifo_th = fifo_depth / 2;
    0
}

unsafe fn dw_configure_dai_by_pd(
    dev: *mut dw_i2s_dev,
    dw_i2s_dai: *mut snd_soc_dai_driver,
    res: *mut resource,
    pdata: *const i2s_platform_data,
) -> c_int {
    let comp1: u32 = i2s_read_reg((*dev).i2s_base, (*dev).i2s_reg_comp1);
    let mut idx: u32 = COMP1_APB_DATA_WIDTH(comp1);
    let mut ret: c_int;

    if WARN_ON(idx as usize >= ARRAY_SIZE(&bus_widths)) {
        return -EINVAL;
    }

    ret = dw_configure_dai(dev, dw_i2s_dai, (*pdata).snd_rates);
    if ret < 0 { return ret; }

    if ((*dev).quirks & DW_I2S_QUIRK_16BIT_IDX_OVERRIDE) != 0 {
        idx = 1;
    }

    if (*dev).is_jh7110 {
        /* Use platform data and snd_dmaengine_dai_dma_data struct at the same time */
        let comp2: u32 = i2s_read_reg((*dev).i2s_base, I2S_COMP_PARAM_2);
        let mut idx2: u32;

        if COMP1_TX_ENABLED(comp1) {
            idx2 = COMP1_TX_WORDSIZE_0(comp1);
            (*dev).play_dma_data.dt.addr = (*res).start + I2S_TXDMA;
            (*dev).play_dma_data.dt.fifo_size = ((*dev).fifo_th * 2 * fifo_width[idx2 as usize]) >> 8;
            (*dev).play_dma_data.dt.maxburst = 16;
        }
        if COMP1_RX_ENABLED(comp1) {
            idx2 = COMP2_RX_WORDSIZE_0(comp2);
            (*dev).capture_dma_data.dt.addr = (*res).start + I2S_RXDMA;
            (*dev).capture_dma_data.dt.fifo_size = (*dev).fifo_th * 2 * (fifo_width[idx2 as usize] >> 8);
            (*dev).capture_dma_data.dt.maxburst = 16;
        }
    } else {
        /* Set DMA slaves info */
        (*dev).play_dma_data.pd.data = (*pdata).play_dma_data;
        (*dev).capture_dma_data.pd.data = (*pdata).capture_dma_data;
        (*dev).play_dma_data.pd.addr = (*res).start + I2S_TXDMA;
        (*dev).capture_dma_data.pd.addr = (*res).start + I2S_RXDMA;
        (*dev).play_dma_data.pd.max_burst = 16;
        (*dev).capture_dma_data.pd.max_burst = 16;
        (*dev).play_dma_data.pd.addr_width = bus_widths[idx as usize];
        (*dev).capture_dma_data.pd.addr_width = bus_widths[idx as usize];
        (*dev).play_dma_data.pd.filter = (*pdata).filter;
        (*dev).capture_dma_data.pd.filter = (*pdata).filter;
    }

    0
}

unsafe fn dw_configure_dai_by_dt(
    dev: *mut dw_i2s_dev,
    dw_i2s_dai: *mut snd_soc_dai_driver,
    res: *mut resource,
) -> c_int {
    let comp1: u32 = i2s_read_reg((*dev).i2s_base, I2S_COMP_PARAM_1);
    let comp2: u32 = i2s_read_reg((*dev).i2s_base, I2S_COMP_PARAM_2);
    let fifo_depth: u32 = 1 << (1 + COMP1_FIFO_DEPTH_GLOBAL(comp1));
    let mut idx2: u32;
    let ret: c_int;

    ret = dw_configure_dai(dev, dw_i2s_dai, SNDRV_PCM_RATE_8000_192000);
    if ret < 0 { return ret; }

    if COMP1_TX_ENABLED(comp1) {
        idx2 = COMP1_TX_WORDSIZE_0(comp1);
        (*dev).capability |= DWC_I2S_PLAY;
        (*dev).play_dma_data.dt.addr = (*res).start + I2S_TXDMA;
        (*dev).play_dma_data.dt.fifo_size = (fifo_depth * fifo_width[idx2 as usize]) >> 8;
        (*dev).play_dma_data.dt.maxburst = 16;
    }
    if COMP1_RX_ENABLED(comp1) {
        idx2 = COMP2_RX_WORDSIZE_0(comp2);
        (*dev).capability |= DWC_I2S_RECORD;
        (*dev).capture_dma_data.dt.addr = (*res).start + I2S_RXDMA;
        (*dev).capture_dma_data.dt.fifo_size = fifo_depth * (fifo_width[idx2 as usize] >> 8);
        (*dev).capture_dma_data.dt.maxburst = 16;
    }

    0
}

/* CONFIG_OF */
/* clocks initialization with master mode on JH7110 SoC */
unsafe extern "C" fn jh7110_i2s_crg_master_init(dev: *mut dw_i2s_dev) -> c_int {
    static mut clks: [clk_bulk_data; 5] = [
        clk_bulk_data { id: b"mclk\0".as_ptr() as *const c_char, clk: ptr::null_mut() },
        clk_bulk_data { id: b"mclk_ext\0".as_ptr() as *const c_char, clk: ptr::null_mut() },
        clk_bulk_data { id: b"mclk_inner\0".as_ptr() as *const c_char, clk: ptr::null_mut() },
        clk_bulk_data { id: b"apb\0".as_ptr() as *const c_char, clk: ptr::null_mut() },
        clk_bulk_data { id: b"i2sclk\0".as_ptr() as *const c_char, clk: ptr::null_mut() },
    ];
    let resets: *mut reset_control = devm_reset_control_array_get_exclusive((*dev).dev);
    let mut ret: c_int;
    let pclk: *mut clk;
    let bclk_mst: *mut clk;
    let mclk: *mut clk;
    let mclk_ext: *mut clk;
    let mclk_inner: *mut clk;

    if IS_ERR(resets) {
        return dev_err_probe((*dev).dev, PTR_ERR(resets), b"failed to get i2s resets\n\0".as_ptr() as *const c_char);
    }

    ret = clk_bulk_get((*dev).dev, ARRAY_SIZE(&clks) as c_uint, clks.as_mut_ptr());
    if ret != 0 {
        return dev_err_probe((*dev).dev, ret, b"failed to get i2s clocks\n\0".as_ptr() as *const c_char);
    }

    mclk = clks[0].clk;
    mclk_ext = clks[1].clk;
    mclk_inner = clks[2].clk;
    pclk = clks[3].clk;
    bclk_mst = clks[4].clk;

    ret = clk_prepare_enable(pclk);
    if ret != 0 { goto_exit_master(ret, &mut clks); return ret; }

    /* Use inner mclk first and avoid uninitialized gpio for external mclk */
    ret = clk_set_parent(mclk, mclk_inner);
    if ret != 0 { clk_disable_unprepare(pclk); goto_exit_master(ret, &mut clks); return ret; }

    ret = clk_prepare_enable(bclk_mst);
    if ret != 0 { clk_disable_unprepare(pclk); goto_exit_master(ret, &mut clks); return ret; }

    /* deassert resets before set clock parent */
    ret = reset_control_deassert(resets);
    if ret != 0 { clk_disable_unprepare(bclk_mst); clk_disable_unprepare(pclk); goto_exit_master(ret, &mut clks); return ret; }

    /* external clock (12.288MHz) for Audio */
    ret = clk_set_parent(mclk, mclk_ext);
    if ret != 0 { clk_disable_unprepare(bclk_mst); clk_disable_unprepare(pclk); goto_exit_master(ret, &mut clks); return ret; }

    /* i2sclk will be got and enabled repeatedly later and should be disabled now. */
    clk_disable_unprepare(bclk_mst);
    clk_bulk_put(ARRAY_SIZE(&clks) as c_uint, clks.as_mut_ptr());
    (*dev).is_jh7110 = true;

    0
}

unsafe fn goto_exit_master(ret: c_int, clks: &mut [clk_bulk_data; 5]) {
    clk_bulk_put(ARRAY_SIZE(clks) as c_uint, clks.as_mut_ptr());
    let _ = ret;
}

/* clocks initialization with slave mode on JH7110 SoC */
unsafe extern "C" fn jh7110_i2s_crg_slave_init(dev: *mut dw_i2s_dev) -> c_int {
    static mut clks: [clk_bulk_data; 9] = [
        clk_bulk_data { id: b"mclk\0".as_ptr() as *const c_char, clk: ptr::null_mut() },
        clk_bulk_data { id: b"mclk_ext\0".as_ptr() as *const c_char, clk: ptr::null_mut() },
        clk_bulk_data { id: b"apb\0".as_ptr() as *const c_char, clk: ptr::null_mut() },
        clk_bulk_data { id: b"bclk_ext\0".as_ptr() as *const c_char, clk: ptr::null_mut() },
        clk_bulk_data { id: b"lrck_ext\0".as_ptr() as *const c_char, clk: ptr::null_mut() },
        clk_bulk_data { id: b"bclk\0".as_ptr() as *const c_char, clk: ptr::null_mut() },
        clk_bulk_data { id: b"lrck\0".as_ptr() as *const c_char, clk: ptr::null_mut() },
        clk_bulk_data { id: b"mclk_inner\0".as_ptr() as *const c_char, clk: ptr::null_mut() },
        clk_bulk_data { id: b"i2sclk\0".as_ptr() as *const c_char, clk: ptr::null_mut() },
    ];
    let resets: *mut reset_control = devm_reset_control_array_get_exclusive((*dev).dev);
    let mut ret: c_int;
    let pclk: *mut clk;
    let bclk_mst: *mut clk;
    let bclk_ext: *mut clk;
    let lrck_ext: *mut clk;
    let bclk: *mut clk;
    let lrck: *mut clk;
    let mclk: *mut clk;
    let mclk_ext: *mut clk;
    let mclk_inner: *mut clk;

    if IS_ERR(resets) {
        return dev_err_probe((*dev).dev, PTR_ERR(resets), b"failed to get i2s resets\n\0".as_ptr() as *const c_char);
    }

    ret = clk_bulk_get((*dev).dev, ARRAY_SIZE(&clks) as c_uint, clks.as_mut_ptr());
    if ret != 0 {
        return dev_err_probe((*dev).dev, ret, b"failed to get i2s clocks\n\0".as_ptr() as *const c_char);
    }

    mclk = clks[0].clk;
    mclk_ext = clks[1].clk;
    pclk = clks[2].clk;
    bclk_ext = clks[3].clk;
    lrck_ext = clks[4].clk;
    bclk = clks[5].clk;
    lrck = clks[6].clk;
    mclk_inner = clks[7].clk;
    bclk_mst = clks[8].clk;

    ret = clk_prepare_enable(pclk);
    if ret != 0 { goto_exit_slave(ret, &mut clks); return ret; }

    ret = clk_set_parent(mclk, mclk_inner);
    if ret != 0 { clk_disable_unprepare(pclk); goto_exit_slave(ret, &mut clks); return ret; }

    ret = clk_prepare_enable(bclk_mst);
    if ret != 0 { clk_disable_unprepare(pclk); goto_exit_slave(ret, &mut clks); return ret; }

    ret = reset_control_deassert(resets);
    if ret != 0 { clk_disable_unprepare(bclk_mst); clk_disable_unprepare(pclk); goto_exit_slave(ret, &mut clks); return ret; }

    /* The sources of BCLK and LRCK are the external codec. */
    ret = clk_set_parent(bclk, bclk_ext);
    if ret != 0 { clk_disable_unprepare(bclk_mst); clk_disable_unprepare(pclk); goto_exit_slave(ret, &mut clks); return ret; }

    ret = clk_set_parent(lrck, lrck_ext);
    if ret != 0 { clk_disable_unprepare(bclk_mst); clk_disable_unprepare(pclk); goto_exit_slave(ret, &mut clks); return ret; }

    ret = clk_set_parent(mclk, mclk_ext);
    if ret != 0 { clk_disable_unprepare(bclk_mst); clk_disable_unprepare(pclk); goto_exit_slave(ret, &mut clks); return ret; }

    /* The i2sclk will be got and enabled repeatedly later and should be disabled now. */
    clk_disable_unprepare(bclk_mst);
    clk_bulk_put(ARRAY_SIZE(&clks) as c_uint, clks.as_mut_ptr());
    (*dev).is_jh7110 = true;

    0
}

unsafe fn goto_exit_slave(ret: c_int, clks: &mut [clk_bulk_data; 9]) {
    clk_bulk_put(ARRAY_SIZE(clks) as c_uint, clks.as_mut_ptr());
    let _ = ret;
}

/* Special syscon initialization about RX channel with slave mode on JH7110 SoC */
unsafe extern "C" fn jh7110_i2srx_crg_init(dev: *mut dw_i2s_dev) -> c_int {
    let regmap: *mut regmap;
    let mut args: [c_uint; 2] = [0; 2];

    regmap = syscon_regmap_lookup_by_phandle_args(
        (*(*dev).dev).of_node,
        b"starfive,syscon\0".as_ptr() as *const c_char,
        2,
        args.as_mut_ptr(),
    );
    if IS_ERR(regmap) {
        return dev_err_probe((*dev).dev, PTR_ERR(regmap), b"getting the regmap failed\n\0".as_ptr() as *const c_char);
    }

    /* Enable I2Srx with syscon register, args[0]: offset, args[1]: mask */
    regmap_update_bits(regmap, args[0], args[1], args[1]);

    jh7110_i2s_crg_slave_init(dev)
}

unsafe extern "C" fn jh7110_i2stx0_clk_cfg(config: *mut i2s_clk_config_data) -> c_int {
    let dev: *mut dw_i2s_dev = (config as *mut u8).offset(-(0 as isize)) as *mut dw_i2s_dev;
    let bclk_rate: u32 = (*config).sample_rate * 64;

    clk_set_rate((*dev).clk, bclk_rate)
}

unsafe extern "C" fn dw_i2s_probe(pdev: *mut platform_device) -> c_int {
    let pdata: *const i2s_platform_data = (*pdev).dev.platform_data;
    let mut dev: *mut dw_i2s_dev;
    let mut res: *mut resource = ptr::null_mut();
    let mut ret: c_int;
    let irq: c_int;
    let dw_i2s_dai: *mut snd_soc_dai_driver;
    let clk_id: *const c_char;

    dev = devm_kzalloc(&mut (*pdev).dev, size_of::<dw_i2s_dev>(), GFP_KERNEL) as *mut dw_i2s_dev;
    if dev.is_null() { return -ENOMEM; }

    dw_i2s_dai = devm_kzalloc(&mut (*pdev).dev, size_of::<snd_soc_dai_driver>(), GFP_KERNEL) as *mut snd_soc_dai_driver;
    if dw_i2s_dai.is_null() { return -ENOMEM; }

    (*dw_i2s_dai).ops = &dw_i2s_dai_ops;

    (*dev).i2s_base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR((*dev).i2s_base) {
        return PTR_ERR((*dev).i2s_base);
    }

    (*dev).dev = &mut (*pdev).dev;
    (*dev).is_jh7110 = false;
    if !pdata.is_null() {
        if let Some(i2s_pd_init) = (*pdata).i2s_pd_init {
            ret = i2s_pd_init(dev);
            if ret != 0 { return ret; }
        }
    }

    if !(*dev).is_jh7110 {
        (*dev).reset = devm_reset_control_array_get_optional_shared(&mut (*pdev).dev);
        if IS_ERR((*dev).reset) {
            return PTR_ERR((*dev).reset);
        }

        ret = reset_control_deassert((*dev).reset);
        if ret != 0 { return ret; }
    }

    irq = platform_get_irq_optional(pdev, 0);
    if irq == -EPROBE_DEFER { return irq; }
    if irq > 0 {
        ret = devm_request_irq(
            &mut (*pdev).dev,
            irq,
            Some(i2s_irq_handler),
            0,
            (*pdev).name,
            dev as *mut c_void,
        );
        if ret < 0 {
            dev_err(&mut (*pdev).dev, b"failed to request irq\n\0".as_ptr() as *const c_char);
            reset_control_assert((*dev).reset);
            return ret;
        }
    }

    (*dev).i2s_reg_comp1 = I2S_COMP_PARAM_1;
    (*dev).i2s_reg_comp2 = I2S_COMP_PARAM_2;
    if !pdata.is_null() {
        (*dev).capability = (*pdata).cap;
        clk_id = ptr::null();
        (*dev).quirks = (*pdata).quirks;
        if ((*dev).quirks & DW_I2S_QUIRK_COMP_REG_OFFSET) != 0 {
            (*dev).i2s_reg_comp1 = (*pdata).i2s_reg_comp1;
            (*dev).i2s_reg_comp2 = (*pdata).i2s_reg_comp2;
        }
        ret = dw_configure_dai_by_pd(dev, dw_i2s_dai, res, pdata);
    } else {
        clk_id = b"i2sclk\0".as_ptr() as *const c_char;
        ret = dw_configure_dai_by_dt(dev, dw_i2s_dai, res);
    }
    if ret < 0 {
        reset_control_assert((*dev).reset);
        return ret;
    }

    if ((*dev).capability & DW_I2S_MASTER) != 0 {
        if !pdata.is_null() {
            (*dev).i2s_clk_cfg = (*pdata).i2s_clk_cfg;
            if (*dev).i2s_clk_cfg.is_none() {
                dev_err(&mut (*pdev).dev, b"no clock configure method\n\0".as_ptr() as *const c_char);
                ret = -ENODEV;
                reset_control_assert((*dev).reset);
                return ret;
            }
        }
        (*dev).clk = devm_clk_get_enabled(&mut (*pdev).dev, clk_id);

        if IS_ERR((*dev).clk) {
            ret = PTR_ERR((*dev).clk);
            reset_control_assert((*dev).reset);
            return ret;
        }
    }

    dev_set_drvdata(&mut (*pdev).dev, dev as *mut c_void);
    ret = devm_snd_soc_register_component(&mut (*pdev).dev, &dw_i2s_component, dw_i2s_dai, 1);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, b"not able to register dai\n\0".as_ptr() as *const c_char);
        reset_control_assert((*dev).reset);
        return ret;
    }

    if pdata.is_null() || (*dev).is_jh7110 {
        if irq >= 0 {
            ret = dw_pcm_register(pdev);
            (*dev).use_pio = true;
            (*dev).l_reg = LRBR_LTHR(0);
            (*dev).r_reg = RRBR_RTHR(0);
        } else {
            ret = devm_snd_dmaengine_pcm_register(&mut (*pdev).dev, ptr::null(), 0);
            (*dev).use_pio = false;
        }

        if ret != 0 {
            dev_err(&mut (*pdev).dev, b"could not register pcm: %d\n\0".as_ptr() as *const c_char, ret);
            reset_control_assert((*dev).reset);
            return ret;
        }
    }

    pm_runtime_enable(&mut (*pdev).dev);
    0
}

unsafe extern "C" fn dw_i2s_remove(pdev: *mut platform_device) {
    let dev: *mut dw_i2s_dev = dev_get_drvdata(&mut (*pdev).dev) as *mut dw_i2s_dev;

    reset_control_assert((*dev).reset);
    pm_runtime_disable(&mut (*pdev).dev);
}

/* CONFIG_OF */
static jh7110_i2stx0_data: i2s_platform_data = i2s_platform_data {
    cap: DWC_I2S_PLAY | DW_I2S_MASTER,
    channel: TWO_CHANNEL_SUPPORT,
    snd_fmts: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE,
    snd_rates: SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_48000,
    i2s_clk_cfg: Some(jh7110_i2stx0_clk_cfg),
    i2s_pd_init: Some(jh7110_i2s_crg_master_init),
    quirks: 0,
    i2s_reg_comp1: 0,
    i2s_reg_comp2: 0,
    play_dma_data: ptr::null_mut(),
    capture_dma_data: ptr::null_mut(),
    filter: ptr::null_mut(),
};

static jh7110_i2stx1_data: i2s_platform_data = i2s_platform_data {
    cap: DWC_I2S_PLAY | DW_I2S_SLAVE,
    channel: TWO_CHANNEL_SUPPORT,
    snd_fmts: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE,
    snd_rates: SNDRV_PCM_RATE_8000_192000,
    i2s_clk_cfg: None,
    i2s_pd_init: Some(jh7110_i2s_crg_slave_init),
    quirks: 0,
    i2s_reg_comp1: 0,
    i2s_reg_comp2: 0,
    play_dma_data: ptr::null_mut(),
    capture_dma_data: ptr::null_mut(),
    filter: ptr::null_mut(),
};

static jh7110_i2srx_data: i2s_platform_data = i2s_platform_data {
    cap: DWC_I2S_RECORD | DW_I2S_SLAVE,
    channel: TWO_CHANNEL_SUPPORT,
    snd_fmts: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE,
    snd_rates: SNDRV_PCM_RATE_8000_192000,
    i2s_clk_cfg: None,
    i2s_pd_init: Some(jh7110_i2srx_crg_init),
    quirks: 0,
    i2s_reg_comp1: 0,
    i2s_reg_comp2: 0,
    play_dma_data: ptr::null_mut(),
    capture_dma_data: ptr::null_mut(),
    filter: ptr::null_mut(),
};

static dw_i2s_of_match: [of_device_id; 5] = [
    of_device_id { compatible: b"snps,designware-i2s\0".as_ptr() as *const c_char, data: ptr::null() },
    of_device_id { compatible: b"starfive,jh7110-i2stx0\0".as_ptr() as *const c_char, data: &jh7110_i2stx0_data as *const _ as *const c_void },
    of_device_id { compatible: b"starfive,jh7110-i2stx1\0".as_ptr() as *const c_char, data: &jh7110_i2stx1_data as *const _ as *const c_void },
    of_device_id { compatible: b"starfive,jh7110-i2srx\0".as_ptr() as *const c_char, data: &jh7110_i2srx_data as *const _ as *const c_void },
    of_device_id { compatible: ptr::null(), data: ptr::null() },
];

/* MODULE_DEVICE_TABLE(of, dw_i2s_of_match); */

static dwc_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(dw_i2s_runtime_suspend),
    runtime_resume: Some(dw_i2s_runtime_resume),
};

static mut dw_i2s_driver: platform_driver = platform_driver {
    probe: Some(dw_i2s_probe),
    remove: Some(dw_i2s_remove),
    driver: device_driver {
        name: b"designware-i2s\0".as_ptr() as *const c_char,
        of_match_table: dw_i2s_of_match.as_ptr(),
        pm: &dwc_pm_ops,
    },
};

/* module_platform_driver(dw_i2s_driver); */

/* MODULE_AUTHOR("Rajeev Kumar <rajeevkumar.linux@gmail.com>"); */
/* MODULE_DESCRIPTION("DESIGNWARE I2S SoC Interface"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_ALIAS("platform:designware_i2s"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
