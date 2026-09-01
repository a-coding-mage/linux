// SPDX-License-Identifier: GPL-2.0
/*
 * Mediatek 8173 ALSA SoC AFE platform driver
 *
 * Copyright (c) 2015 MediaTek Inc.
 * Author: Koro Chen <koro.chen@mediatek.com>
 *             Sascha Hauer <s.hauer@pengutronix.de>
 *             Hidalgo Huang <hidalgo.huang@mediatek.com>
 *             Ir Lian <ir.lian@mediatek.com>
 */

/* C includes translated as external dependencies:
 * linux/delay.h, linux/module.h, linux/of.h, linux/of_address.h,
 * linux/of_reserved_mem.h, linux/dma-mapping.h, linux/pm_runtime.h,
 * sound/soc.h, mt8173-afe-common.h, mtk-base-afe.h,
 * mtk-afe-platform-driver.h, mtk-afe-fe-dai.h.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr::{null, null_mut};

/*****************************************************************************
 *                  R E G I S T E R       D E F I N I T I O N
 *****************************************************************************/
const AUDIO_TOP_CON0: c_uint = 0x0000;
const AUDIO_TOP_CON1: c_uint = 0x0004;
const AFE_DAC_CON0: c_uint = 0x0010;
const AFE_DAC_CON1: c_uint = 0x0014;
const AFE_I2S_CON1: c_uint = 0x0034;
const AFE_I2S_CON2: c_uint = 0x0038;
const AFE_CONN_24BIT: c_uint = 0x006c;
const AFE_MEMIF_MSB: c_uint = 0x00cc;

const AFE_CONN1: c_uint = 0x0024;
const AFE_CONN2: c_uint = 0x0028;
const AFE_CONN3: c_uint = 0x002c;
const AFE_CONN7: c_uint = 0x0460;
const AFE_CONN8: c_uint = 0x0464;
const AFE_HDMI_CONN0: c_uint = 0x0390;

/* Memory interface */
const AFE_DL1_BASE: c_uint = 0x0040;
const AFE_DL1_CUR: c_uint = 0x0044;
const AFE_DL1_END: c_uint = 0x0048;
const AFE_DL2_BASE: c_uint = 0x0050;
const AFE_DL2_CUR: c_uint = 0x0054;
const AFE_AWB_BASE: c_uint = 0x0070;
const AFE_AWB_CUR: c_uint = 0x007c;
const AFE_VUL_BASE: c_uint = 0x0080;
const AFE_VUL_CUR: c_uint = 0x008c;
const AFE_VUL_END: c_uint = 0x0088;
const AFE_DAI_BASE: c_uint = 0x0090;
const AFE_DAI_CUR: c_uint = 0x009c;
const AFE_MOD_PCM_BASE: c_uint = 0x0330;
const AFE_MOD_PCM_CUR: c_uint = 0x033c;
const AFE_HDMI_OUT_BASE: c_uint = 0x0374;
const AFE_HDMI_OUT_CUR: c_uint = 0x0378;
const AFE_HDMI_OUT_END: c_uint = 0x037c;

const AFE_ADDA_TOP_CON0: c_uint = 0x0120;
const AFE_ADDA2_TOP_CON0: c_uint = 0x0600;

const AFE_HDMI_OUT_CON0: c_uint = 0x0370;

const AFE_IRQ_MCU_CON: c_uint = 0x03a0;
const AFE_IRQ_STATUS: c_uint = 0x03a4;
const AFE_IRQ_CLR: c_uint = 0x03a8;
const AFE_IRQ_CNT1: c_uint = 0x03ac;
const AFE_IRQ_CNT2: c_uint = 0x03b0;
const AFE_IRQ_MCU_EN: c_uint = 0x03b4;
const AFE_IRQ_CNT5: c_uint = 0x03bc;
const AFE_IRQ_CNT7: c_uint = 0x03dc;

const AFE_TDM_CON1: c_uint = 0x0548;
const AFE_TDM_CON2: c_uint = 0x054c;

const AFE_IRQ_STATUS_BITS: c_uint = 0xff;

/* AUDIO_TOP_CON0 (0x0000) */
const AUD_TCON0_PDN_SPDF: c_uint = 0x1 << 21;
const AUD_TCON0_PDN_HDMI: c_uint = 0x1 << 20;
const AUD_TCON0_PDN_24M: c_uint = 0x1 << 9;
const AUD_TCON0_PDN_22M: c_uint = 0x1 << 8;
const AUD_TCON0_PDN_AFE: c_uint = 0x1 << 2;

/* AFE_I2S_CON1 (0x0034) */
const AFE_I2S_CON1_LOW_JITTER_CLK: c_uint = 0x1 << 12;
const fn AFE_I2S_CON1_RATE(x: c_int) -> c_uint { (((x as c_uint) & 0xf) << 8) }
const AFE_I2S_CON1_FORMAT_I2S: c_uint = 0x1 << 3;
const AFE_I2S_CON1_EN: c_uint = 0x1 << 0;

/* AFE_I2S_CON2 (0x0038) */
const AFE_I2S_CON2_LOW_JITTER_CLK: c_uint = 0x1 << 12;
const fn AFE_I2S_CON2_RATE(x: c_int) -> c_uint { (((x as c_uint) & 0xf) << 8) }
const AFE_I2S_CON2_FORMAT_I2S: c_uint = 0x1 << 3;
const AFE_I2S_CON2_EN: c_uint = 0x1 << 0;

/* AFE_CONN_24BIT (0x006c) */
const AFE_CONN_24BIT_O04: c_uint = 0x1 << 4;
const AFE_CONN_24BIT_O03: c_uint = 0x1 << 3;

/* AFE_HDMI_CONN0 (0x0390) */
const AFE_HDMI_CONN0_O37_I37: c_uint = 0x7 << 21;
const AFE_HDMI_CONN0_O36_I36: c_uint = 0x6 << 18;
const AFE_HDMI_CONN0_O35_I33: c_uint = 0x3 << 15;
const AFE_HDMI_CONN0_O34_I32: c_uint = 0x2 << 12;
const AFE_HDMI_CONN0_O33_I35: c_uint = 0x5 << 9;
const AFE_HDMI_CONN0_O32_I34: c_uint = 0x4 << 6;
const AFE_HDMI_CONN0_O31_I31: c_uint = 0x1 << 3;
const AFE_HDMI_CONN0_O30_I30: c_uint = 0x0 << 0;

/* AFE_TDM_CON1 (0x0548) */
const fn AFE_TDM_CON1_LRCK_WIDTH(x: c_uint) -> c_uint { (x - 1) << 24 }
const AFE_TDM_CON1_32_BCK_CYCLES: c_uint = 0x2 << 12;
const AFE_TDM_CON1_WLEN_32BIT: c_uint = 0x2 << 8;
const AFE_TDM_CON1_MSB_ALIGNED: c_uint = 0x1 << 4;
const AFE_TDM_CON1_1_BCK_DELAY: c_uint = 0x1 << 3;
const AFE_TDM_CON1_LRCK_INV: c_uint = 0x1 << 2;
const AFE_TDM_CON1_BCK_INV: c_uint = 0x1 << 1;
const AFE_TDM_CON1_EN: c_uint = 0x1 << 0;

#[repr(C)]
enum afe_tdm_ch_start {
    AFE_TDM_CH_START_O30_O31 = 0,
    AFE_TDM_CH_START_O32_O33,
    AFE_TDM_CH_START_O34_O35,
    AFE_TDM_CH_START_O36_O37,
    AFE_TDM_CH_ZERO,
}

type bool_ = bool;
type size_t = usize;
type irqreturn_t = c_uint;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENXIO: c_int = 6;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_NONE: c_uint = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 0;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 1;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 2;
const SNDRV_PCM_RATE_8000_48000: c_uint = 0;
const SNDRV_PCM_RATE_32000: c_uint = 1 << 0;
const SNDRV_PCM_RATE_44100: c_uint = 1 << 1;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 2;
const SNDRV_PCM_RATE_88200: c_uint = 1 << 3;
const SNDRV_PCM_RATE_96000: c_uint = 1 << 4;
const SNDRV_PCM_RATE_176400: c_uint = 1 << 5;
const SNDRV_PCM_RATE_192000: c_uint = 1 << 6;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 1 << 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 2;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 3;
const SND_SOC_NOPM: c_int = 0;

const MT8173_CLK_NUM: usize = 10;
const MT8173_CLK_INFRASYS_AUD: usize = 0;
const MT8173_CLK_TOP_PDN_AUD: usize = 1;
const MT8173_CLK_TOP_PDN_AUD_BUS: usize = 2;
const MT8173_CLK_I2S0_M: usize = 3;
const MT8173_CLK_I2S1_M: usize = 4;
const MT8173_CLK_I2S2_M: usize = 5;
const MT8173_CLK_I2S3_M: usize = 6;
const MT8173_CLK_I2S3_B: usize = 7;
const MT8173_CLK_BCK0: usize = 8;
const MT8173_CLK_BCK1: usize = 9;

const MT8173_AFE_MEMIF_DL1: c_int = 0;
const MT8173_AFE_MEMIF_DL2: c_int = 1;
const MT8173_AFE_MEMIF_VUL: c_int = 2;
const MT8173_AFE_MEMIF_DAI: c_int = 3;
const MT8173_AFE_MEMIF_AWB: c_int = 4;
const MT8173_AFE_MEMIF_MOD_DAI: c_int = 5;
const MT8173_AFE_MEMIF_HDMI: c_int = 6;
const MT8173_AFE_MEMIF_NUM: usize = 7;
const MT8173_AFE_IO_I2S: c_int = 7;
const MT8173_AFE_IO_HDMI: c_int = 8;

const MT8173_AFE_IRQ_DL1: c_int = 0;
const MT8173_AFE_IRQ_DL2: c_int = 1;
const MT8173_AFE_IRQ_VUL: c_int = 2;
const MT8173_AFE_IRQ_DAI: c_int = 3;
const MT8173_AFE_IRQ_AWB: c_int = 4;
const MT8173_AFE_IRQ_HDMI: c_int = 6;
const MT8173_AFE_IRQ_NUM: usize = 7;

const AFE_PCM_NAME: *const c_char = b"AFE_PCM\0".as_ptr() as *const c_char;

#[repr(C)] struct clk { _private: [u8; 0] }
#[repr(C)] struct regmap { _private: [u8; 0] }
#[repr(C)] struct device { _private: [u8; 0] }
#[repr(C)] struct platform_device { dev: device }
#[repr(C)] struct snd_pcm_substream { runtime: *mut snd_pcm_runtime }
#[repr(C)] struct snd_pcm_runtime { rate: c_uint, channels: c_uint }
#[repr(C)] struct snd_soc_dai { name: *const c_char, id: c_int }
#[repr(C)] struct snd_soc_pcm_runtime { _private: [u8; 0] }
#[repr(C)] struct snd_soc_component { _private: [u8; 0] }

#[repr(C)]
struct snd_pcm_hardware {
    info: c_uint,
    buffer_bytes_max: c_uint,
    period_bytes_min: c_uint,
    period_bytes_max: c_uint,
    periods_min: c_uint,
    periods_max: c_uint,
    fifo_size: c_uint,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: c_uint,
}

#[repr(C)]
struct snd_soc_dai_ops {
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
}

#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    id: c_int,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
    symmetric_rate: c_uint,
}

#[repr(C)]
struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)]
struct snd_soc_dapm_widget { _private: [u8; 0] }
#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
struct snd_soc_component_driver {
    name: *const c_char,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    debugfs_prefix: *const c_char,
}

#[repr(C)]
struct mtk_base_memif_data {
    name: *const c_char,
    id: c_int,
    reg_ofs_base: c_int,
    reg_ofs_cur: c_int,
    fs_reg: c_int,
    fs_shift: c_int,
    fs_maskbit: c_int,
    mono_reg: c_int,
    mono_shift: c_int,
    hd_reg: c_int,
    enable_reg: c_int,
    enable_shift: c_int,
    msb_reg: c_int,
    msb_shift: c_int,
    agent_disable_reg: c_int,
}

#[repr(C)]
struct mtk_base_irq_data {
    id: c_int,
    irq_cnt_reg: c_int,
    irq_cnt_shift: c_int,
    irq_cnt_maskbit: c_int,
    irq_en_reg: c_int,
    irq_en_shift: c_int,
    irq_fs_reg: c_int,
    irq_fs_shift: c_int,
    irq_fs_maskbit: c_int,
    irq_clr_reg: c_int,
    irq_clr_shift: c_int,
}

#[repr(C)]
struct mtk_base_afe_irq {
    irq_data: *const mtk_base_irq_data,
    irq_occupyed: bool_,
}

#[repr(C)]
struct mtk_base_afe_memif_data_ptr {
    id: c_int,
}

#[repr(C)]
struct mtk_base_afe_memif {
    data: *const mtk_base_memif_data,
    irq_usage: c_int,
    substream: *mut snd_pcm_substream,
    const_irq: c_int,
}

#[repr(C)]
struct mtk_base_afe {
    regmap: *mut regmap,
    dev: *mut device,
    platform_priv: *mut mt8173_afe_private,
    preallocate_buffers: bool_,
    base_addr: *mut c_void,
    memif_size: c_int,
    memif: *mut mtk_base_afe_memif,
    irqs_size: c_int,
    irqs: *mut mtk_base_afe_irq,
    mtk_afe_hardware: *const snd_pcm_hardware,
    memif_fs: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_uint) -> c_int>,
    irq_fs: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_uint) -> c_int>,
    reg_back_up_list: *const c_uint,
    reg_back_up_list_num: c_uint,
    runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
struct regmap_config {
    reg_bits: c_uint,
    reg_stride: c_uint,
    val_bits: c_uint,
    max_register: c_uint,
    cache_type: c_uint,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
struct dev_pm_ops { _private: [u8; 0] }

#[repr(C)]
struct platform_driver_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
    pm: *const dev_pm_ops,
}

#[repr(C)]
struct platform_driver {
    driver: platform_driver_driver,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

#[repr(C)]
struct mt8173_afe_private {
    clocks: [*mut clk; MT8173_CLK_NUM],
}

extern "C" {
    static mtk_afe_fe_ops: snd_soc_dai_ops;
    static mtk_afe_pcm_platform: snd_soc_component_driver;

    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut mtk_base_afe;
    fn snd_soc_dai_active(dai: *mut snd_soc_dai) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtdcom_lookup(rtd: *mut snd_soc_pcm_runtime, name: *const c_char) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut mtk_base_afe;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn dev_get_drvdata(dev: *mut device) -> *mut mtk_base_afe;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dma_set_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn of_reserved_mem_device_init(dev: *mut device) -> c_int;
    fn platform_get_irq(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn devm_regmap_init_mmio(dev: *mut device, regs: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn devm_kcalloc(dev: *mut device, n: size_t, size: size_t, flags: c_uint) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_enabled(dev: *mut device) -> bool_;
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_status_suspended(dev: *mut device) -> bool_;
    fn devm_snd_soc_register_component(dev: *mut device, component: *const snd_soc_component_driver, dais: *mut snd_soc_dai_driver, num_dais: c_int) -> c_int;
    fn snd_soc_register_component(dev: *mut device, component: *const snd_soc_component_driver, dais: *mut snd_soc_dai_driver, num_dais: c_int) -> c_int;
    fn snd_soc_unregister_component(dev: *mut device);
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_uint, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn mtk_afe_suspend(component: *mut snd_soc_component) -> c_int;
    fn mtk_afe_resume(component: *mut snd_soc_component) -> c_int;
    fn pm_ptr(ops: *const dev_pm_ops) -> *const dev_pm_ops;
}

const fn DMA_BIT_MASK(n: u32) -> u64 { if n == 64 { !0 } else { (1u64 << n) - 1 } }

static mt8173_afe_backup_list: [c_uint; 14] = [
    AUDIO_TOP_CON0, AFE_CONN1, AFE_CONN2, AFE_CONN7, AFE_CONN8, AFE_DAC_CON1,
    AFE_DL1_BASE, AFE_DL1_END, AFE_VUL_BASE, AFE_VUL_END, AFE_HDMI_OUT_BASE,
    AFE_HDMI_OUT_END, AFE_HDMI_CONN0, AFE_DAC_CON0,
];

static mt8173_afe_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP_VALID,
    buffer_bytes_max: 256 * 1024,
    period_bytes_min: 512,
    period_bytes_max: 128 * 1024,
    periods_min: 2,
    periods_max: 256,
    fifo_size: 0,
};

#[repr(C)]
struct mt8173_afe_rate {
    rate: c_uint,
    regvalue: c_uint,
}

static mt8173_afe_i2s_rates: [mt8173_afe_rate; 13] = [
    mt8173_afe_rate { rate: 8000, regvalue: 0 },
    mt8173_afe_rate { rate: 11025, regvalue: 1 },
    mt8173_afe_rate { rate: 12000, regvalue: 2 },
    mt8173_afe_rate { rate: 16000, regvalue: 4 },
    mt8173_afe_rate { rate: 22050, regvalue: 5 },
    mt8173_afe_rate { rate: 24000, regvalue: 6 },
    mt8173_afe_rate { rate: 32000, regvalue: 8 },
    mt8173_afe_rate { rate: 44100, regvalue: 9 },
    mt8173_afe_rate { rate: 48000, regvalue: 10 },
    mt8173_afe_rate { rate: 88000, regvalue: 11 },
    mt8173_afe_rate { rate: 96000, regvalue: 12 },
    mt8173_afe_rate { rate: 174000, regvalue: 13 },
    mt8173_afe_rate { rate: 192000, regvalue: 14 },
];

unsafe extern "C" fn mt8173_afe_i2s_fs(sample_rate: c_uint) -> c_int {
    let mut i: usize = 0;
    while i < mt8173_afe_i2s_rates.len() {
        if mt8173_afe_i2s_rates[i].rate == sample_rate {
            return mt8173_afe_i2s_rates[i].regvalue as c_int;
        }
        i += 1;
    }
    -EINVAL
}

unsafe extern "C" fn mt8173_afe_set_i2s(afe: *mut mtk_base_afe, rate: c_uint) -> c_int {
    let fs = mt8173_afe_i2s_fs(rate);
    if fs < 0 {
        return -EINVAL;
    }

    /* from external ADC */
    regmap_update_bits((*afe).regmap, AFE_ADDA_TOP_CON0, 0x1, 0x1);
    regmap_update_bits((*afe).regmap, AFE_ADDA2_TOP_CON0, 0x1, 0x1);

    /* set input */
    let mut val = AFE_I2S_CON2_LOW_JITTER_CLK |
        AFE_I2S_CON2_RATE(fs) |
        AFE_I2S_CON2_FORMAT_I2S;
    regmap_update_bits((*afe).regmap, AFE_I2S_CON2, !AFE_I2S_CON2_EN, val);

    /* set output */
    val = AFE_I2S_CON1_LOW_JITTER_CLK |
        AFE_I2S_CON1_RATE(fs) |
        AFE_I2S_CON1_FORMAT_I2S;
    regmap_update_bits((*afe).regmap, AFE_I2S_CON1, !AFE_I2S_CON1_EN, val);
    0
}

unsafe extern "C" fn mt8173_afe_set_i2s_enable(afe: *mut mtk_base_afe, enable: bool_) {
    let mut val: c_uint = 0;
    regmap_read((*afe).regmap, AFE_I2S_CON2, &mut val);
    if ((val & AFE_I2S_CON2_EN) != 0) == enable {
        return;
    }

    /* input */
    regmap_update_bits((*afe).regmap, AFE_I2S_CON2, 0x1, enable as c_uint);

    /* output */
    regmap_update_bits((*afe).regmap, AFE_I2S_CON1, 0x1, enable as c_uint);
}

unsafe extern "C" fn mt8173_afe_dais_enable_clks(
    afe: *mut mtk_base_afe,
    m_ck: *mut clk,
    b_ck: *mut clk,
) -> c_int {
    let mut ret: c_int;
    if !m_ck.is_null() {
        ret = clk_prepare_enable(m_ck);
        if ret != 0 {
            dev_err((*afe).dev, b"Failed to enable m_ck\n\0".as_ptr() as *const c_char);
            return ret;
        }
    }
    if !b_ck.is_null() {
        ret = clk_prepare_enable(b_ck);
        if ret != 0 {
            dev_err((*afe).dev, b"Failed to enable b_ck\n\0".as_ptr() as *const c_char);
            return ret;
        }
    }
    0
}

unsafe extern "C" fn mt8173_afe_dais_set_clks(
    afe: *mut mtk_base_afe,
    m_ck: *mut clk,
    mck_rate: c_uint,
    b_ck: *mut clk,
    bck_rate: c_uint,
) -> c_int {
    let mut ret: c_int;
    if !m_ck.is_null() {
        ret = clk_set_rate(m_ck, mck_rate);
        if ret != 0 {
            dev_err((*afe).dev, b"Failed to set m_ck rate\n\0".as_ptr() as *const c_char);
            return ret;
        }
    }
    if !b_ck.is_null() {
        ret = clk_set_rate(b_ck, bck_rate);
        if ret != 0 {
            dev_err((*afe).dev, b"Failed to set b_ck rate\n\0".as_ptr() as *const c_char);
            return ret;
        }
    }
    0
}

unsafe extern "C" fn mt8173_afe_dais_disable_clks(
    _afe: *mut mtk_base_afe,
    m_ck: *mut clk,
    b_ck: *mut clk,
) {
    clk_disable_unprepare(m_ck);
    clk_disable_unprepare(b_ck);
}

unsafe extern "C" fn mt8173_afe_i2s_startup(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let afe = snd_soc_dai_get_drvdata(dai);
    if snd_soc_dai_active(dai) != 0 {
        return 0;
    }
    regmap_update_bits((*afe).regmap, AUDIO_TOP_CON0, AUD_TCON0_PDN_22M | AUD_TCON0_PDN_24M, 0);
    0
}

unsafe extern "C" fn mt8173_afe_i2s_shutdown(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let afe = snd_soc_dai_get_drvdata(dai);
    if snd_soc_dai_active(dai) != 0 {
        return;
    }
    mt8173_afe_set_i2s_enable(afe, false);
    regmap_update_bits(
        (*afe).regmap,
        AUDIO_TOP_CON0,
        AUD_TCON0_PDN_22M | AUD_TCON0_PDN_24M,
        AUD_TCON0_PDN_22M | AUD_TCON0_PDN_24M,
    );
}

unsafe extern "C" fn mt8173_afe_i2s_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let runtime = (*substream).runtime;
    let afe = snd_soc_dai_get_drvdata(dai);
    let afe_priv = (*afe).platform_priv;
    mt8173_afe_dais_set_clks(afe, (*afe_priv).clocks[MT8173_CLK_I2S1_M], (*runtime).rate * 256, null_mut(), 0);
    mt8173_afe_dais_set_clks(afe, (*afe_priv).clocks[MT8173_CLK_I2S2_M], (*runtime).rate * 256, null_mut(), 0);
    /* config I2S */
    let ret = mt8173_afe_set_i2s(afe, (*(*substream).runtime).rate);
    if ret != 0 {
        return ret;
    }
    mt8173_afe_set_i2s_enable(afe, true);
    0
}

unsafe extern "C" fn mt8173_afe_hdmi_startup(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let afe = snd_soc_dai_get_drvdata(dai);
    let afe_priv = (*afe).platform_priv;
    if snd_soc_dai_active(dai) != 0 {
        return 0;
    }
    mt8173_afe_dais_enable_clks(afe, (*afe_priv).clocks[MT8173_CLK_I2S3_M], (*afe_priv).clocks[MT8173_CLK_I2S3_B]);
    0
}

unsafe extern "C" fn mt8173_afe_hdmi_shutdown(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let afe = snd_soc_dai_get_drvdata(dai);
    let afe_priv = (*afe).platform_priv;
    if snd_soc_dai_active(dai) != 0 {
        return;
    }
    mt8173_afe_dais_disable_clks(afe, (*afe_priv).clocks[MT8173_CLK_I2S3_M], (*afe_priv).clocks[MT8173_CLK_I2S3_B]);
}

unsafe extern "C" fn mt8173_afe_hdmi_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let runtime = (*substream).runtime;
    let afe = snd_soc_dai_get_drvdata(dai);
    let afe_priv = (*afe).platform_priv;

    mt8173_afe_dais_set_clks(
        afe,
        (*afe_priv).clocks[MT8173_CLK_I2S3_M],
        (*runtime).rate * 128,
        (*afe_priv).clocks[MT8173_CLK_I2S3_B],
        (*runtime).rate * (*runtime).channels * 32,
    );

    let mut val = AFE_TDM_CON1_BCK_INV |
        AFE_TDM_CON1_LRCK_INV |
        AFE_TDM_CON1_1_BCK_DELAY |
        AFE_TDM_CON1_MSB_ALIGNED | /* I2S mode */
        AFE_TDM_CON1_WLEN_32BIT |
        AFE_TDM_CON1_32_BCK_CYCLES |
        AFE_TDM_CON1_LRCK_WIDTH(32);
    regmap_update_bits((*afe).regmap, AFE_TDM_CON1, !AFE_TDM_CON1_EN, val);

    /* set tdm2 config */
    match (*runtime).channels {
        1 | 2 => {
            val = afe_tdm_ch_start::AFE_TDM_CH_START_O30_O31 as c_uint;
            val |= (afe_tdm_ch_start::AFE_TDM_CH_ZERO as c_uint) << 4;
            val |= (afe_tdm_ch_start::AFE_TDM_CH_ZERO as c_uint) << 8;
            val |= (afe_tdm_ch_start::AFE_TDM_CH_ZERO as c_uint) << 12;
        }
        3 | 4 => {
            val = afe_tdm_ch_start::AFE_TDM_CH_START_O30_O31 as c_uint;
            val |= (afe_tdm_ch_start::AFE_TDM_CH_START_O32_O33 as c_uint) << 4;
            val |= (afe_tdm_ch_start::AFE_TDM_CH_ZERO as c_uint) << 8;
            val |= (afe_tdm_ch_start::AFE_TDM_CH_ZERO as c_uint) << 12;
        }
        5 | 6 => {
            val = afe_tdm_ch_start::AFE_TDM_CH_START_O30_O31 as c_uint;
            val |= (afe_tdm_ch_start::AFE_TDM_CH_START_O32_O33 as c_uint) << 4;
            val |= (afe_tdm_ch_start::AFE_TDM_CH_START_O34_O35 as c_uint) << 8;
            val |= (afe_tdm_ch_start::AFE_TDM_CH_ZERO as c_uint) << 12;
        }
        7 | 8 => {
            val = afe_tdm_ch_start::AFE_TDM_CH_START_O30_O31 as c_uint;
            val |= (afe_tdm_ch_start::AFE_TDM_CH_START_O32_O33 as c_uint) << 4;
            val |= (afe_tdm_ch_start::AFE_TDM_CH_START_O34_O35 as c_uint) << 8;
            val |= (afe_tdm_ch_start::AFE_TDM_CH_START_O36_O37 as c_uint) << 12;
        }
        _ => val = 0,
    }
    regmap_update_bits((*afe).regmap, AFE_TDM_CON2, 0x0000ffff, val);
    regmap_update_bits((*afe).regmap, AFE_HDMI_OUT_CON0, 0x000000f0, (*runtime).channels << 4);
    0
}

unsafe extern "C" fn mt8173_afe_hdmi_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let afe = snd_soc_dai_get_drvdata(dai);
    dev_info((*afe).dev, b"%s cmd=%d %s\n\0".as_ptr() as *const c_char, b"mt8173_afe_hdmi_trigger\0".as_ptr(), cmd, (*dai).name);

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
            regmap_update_bits((*afe).regmap, AUDIO_TOP_CON0, AUD_TCON0_PDN_HDMI | AUD_TCON0_PDN_SPDF, 0);
            /* set connections:  O30~O37: L/R/LS/RS/C/LFE/CH7/CH8 */
            regmap_write(
                (*afe).regmap,
                AFE_HDMI_CONN0,
                AFE_HDMI_CONN0_O30_I30 |
                    AFE_HDMI_CONN0_O31_I31 |
                    AFE_HDMI_CONN0_O32_I34 |
                    AFE_HDMI_CONN0_O33_I35 |
                    AFE_HDMI_CONN0_O34_I32 |
                    AFE_HDMI_CONN0_O35_I33 |
                    AFE_HDMI_CONN0_O36_I36 |
                    AFE_HDMI_CONN0_O37_I37,
            );
            /* enable Out control */
            regmap_update_bits((*afe).regmap, AFE_HDMI_OUT_CON0, 0x1, 0x1);
            /* enable tdm */
            regmap_update_bits((*afe).regmap, AFE_TDM_CON1, 0x1, 0x1);
            0
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
            /* disable tdm */
            regmap_update_bits((*afe).regmap, AFE_TDM_CON1, 0x1, 0);
            /* disable Out control */
            regmap_update_bits((*afe).regmap, AFE_HDMI_OUT_CON0, 0x1, 0);
            regmap_update_bits(
                (*afe).regmap,
                AUDIO_TOP_CON0,
                AUD_TCON0_PDN_HDMI | AUD_TCON0_PDN_SPDF,
                AUD_TCON0_PDN_HDMI | AUD_TCON0_PDN_SPDF,
            );
            0
        }
        _ => -EINVAL,
    }
}

unsafe extern "C" fn mt8173_memif_fs(substream: *mut snd_pcm_substream, rate: c_uint) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let component = snd_soc_rtdcom_lookup(rtd, AFE_PCM_NAME);
    let afe = snd_soc_component_get_drvdata(component);
    let memif = &mut *(*afe).memif.add((*snd_soc_rtd_to_cpu(rtd, 0)).id as usize);
    let fs: c_int;
    if (*memif.data).id == MT8173_AFE_MEMIF_DAI || (*memif.data).id == MT8173_AFE_MEMIF_MOD_DAI {
        match rate {
            8000 => fs = 0,
            16000 => fs = 1,
            32000 => fs = 2,
            _ => return -EINVAL,
        }
    } else {
        fs = mt8173_afe_i2s_fs(rate);
    }
    fs
}

unsafe extern "C" fn mt8173_irq_fs(_substream: *mut snd_pcm_substream, rate: c_uint) -> c_int {
    mt8173_afe_i2s_fs(rate)
}

/* BE DAIs */
static mt8173_afe_i2s_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(mt8173_afe_i2s_startup),
    shutdown: Some(mt8173_afe_i2s_shutdown),
    prepare: Some(mt8173_afe_i2s_prepare),
    trigger: None,
};

static mt8173_afe_hdmi_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(mt8173_afe_hdmi_startup),
    shutdown: Some(mt8173_afe_hdmi_shutdown),
    prepare: Some(mt8173_afe_hdmi_prepare),
    trigger: Some(mt8173_afe_hdmi_trigger),
};

static mut mt8173_afe_pcm_dais: [snd_soc_dai_driver; 3] = [
    /* FE DAIs: memory intefaces to CPU */
    snd_soc_dai_driver {
        name: b"DL1\0".as_ptr() as *const c_char, /* downlink 1 */
        id: MT8173_AFE_MEMIF_DL1,
        playback: snd_soc_pcm_stream { stream_name: b"DL1\0".as_ptr() as *const c_char, channels_min: 1, channels_max: 2, rates: SNDRV_PCM_RATE_8000_48000, formats: SNDRV_PCM_FMTBIT_S16_LE },
        capture: snd_soc_pcm_stream { stream_name: null(), channels_min: 0, channels_max: 0, rates: 0, formats: 0 },
        ops: unsafe { &mtk_afe_fe_ops },
        symmetric_rate: 0,
    },
    snd_soc_dai_driver {
        name: b"VUL\0".as_ptr() as *const c_char, /* voice uplink */
        id: MT8173_AFE_MEMIF_VUL,
        playback: snd_soc_pcm_stream { stream_name: null(), channels_min: 0, channels_max: 0, rates: 0, formats: 0 },
        capture: snd_soc_pcm_stream { stream_name: b"VUL\0".as_ptr() as *const c_char, channels_min: 1, channels_max: 2, rates: SNDRV_PCM_RATE_8000_48000, formats: SNDRV_PCM_FMTBIT_S16_LE },
        ops: unsafe { &mtk_afe_fe_ops },
        symmetric_rate: 0,
    },
    snd_soc_dai_driver {
        /* BE DAIs */
        name: b"I2S\0".as_ptr() as *const c_char,
        id: MT8173_AFE_IO_I2S,
        playback: snd_soc_pcm_stream { stream_name: b"I2S Playback\0".as_ptr() as *const c_char, channels_min: 1, channels_max: 2, rates: SNDRV_PCM_RATE_8000_48000, formats: SNDRV_PCM_FMTBIT_S16_LE },
        capture: snd_soc_pcm_stream { stream_name: b"I2S Capture\0".as_ptr() as *const c_char, channels_min: 1, channels_max: 2, rates: SNDRV_PCM_RATE_8000_48000, formats: SNDRV_PCM_FMTBIT_S16_LE },
        ops: &mt8173_afe_i2s_ops,
        symmetric_rate: 1,
    },
];

static mut mt8173_afe_hdmi_dais: [snd_soc_dai_driver; 2] = [
    /* FE DAIs */
    snd_soc_dai_driver {
        name: b"HDMI\0".as_ptr() as *const c_char,
        id: MT8173_AFE_MEMIF_HDMI,
        playback: snd_soc_pcm_stream {
            stream_name: b"HDMI\0".as_ptr() as *const c_char,
            channels_min: 2,
            channels_max: 8,
            rates: SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_176400 | SNDRV_PCM_RATE_192000,
            formats: SNDRV_PCM_FMTBIT_S16_LE,
        },
        capture: snd_soc_pcm_stream { stream_name: null(), channels_min: 0, channels_max: 0, rates: 0, formats: 0 },
        ops: unsafe { &mtk_afe_fe_ops },
        symmetric_rate: 0,
    },
    snd_soc_dai_driver {
        /* BE DAIs */
        name: b"HDMIO\0".as_ptr() as *const c_char,
        id: MT8173_AFE_IO_HDMI,
        playback: snd_soc_pcm_stream {
            stream_name: b"HDMIO Playback\0".as_ptr() as *const c_char,
            channels_min: 2,
            channels_max: 8,
            rates: SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_176400 | SNDRV_PCM_RATE_192000,
            formats: SNDRV_PCM_FMTBIT_S16_LE,
        },
        capture: snd_soc_pcm_stream { stream_name: null(), channels_min: 0, channels_max: 0, rates: 0, formats: 0 },
        ops: &mt8173_afe_hdmi_ops,
        symmetric_rate: 0,
    },
];

/* SOC_DAPM_SINGLE_AUTODISABLE and SND_SOC_DAPM_MIXER are external C macros.
 * Their invocations are preserved as source-level comments because their Rust
 * structure cannot be derived from this isolated implementation file.
 */
static mt8173_afe_o03_mix: [snd_kcontrol_new; 0] = []; /* SOC_DAPM_SINGLE_AUTODISABLE("I05 Switch", AFE_CONN1, 21, 1, 0) */
static mt8173_afe_o04_mix: [snd_kcontrol_new; 0] = []; /* SOC_DAPM_SINGLE_AUTODISABLE("I06 Switch", AFE_CONN2, 6, 1, 0) */
static mt8173_afe_o09_mix: [snd_kcontrol_new; 0] = []; /* I03 Switch, I17 Switch */
static mt8173_afe_o10_mix: [snd_kcontrol_new; 0] = []; /* I04 Switch, I18 Switch */
static mt8173_afe_pcm_widgets: [snd_soc_dapm_widget; 0] = []; /* inter-connections and O03/O04/O09/O10 mixers */

static mt8173_afe_pcm_routes: [snd_soc_dapm_route; 16] = [
    snd_soc_dapm_route { sink: b"I05\0".as_ptr() as *const c_char, control: null(), source: b"DL1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"I06\0".as_ptr() as *const c_char, control: null(), source: b"DL1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"I2S Playback\0".as_ptr() as *const c_char, control: null(), source: b"O03\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"I2S Playback\0".as_ptr() as *const c_char, control: null(), source: b"O04\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"VUL\0".as_ptr() as *const c_char, control: null(), source: b"O09\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"VUL\0".as_ptr() as *const c_char, control: null(), source: b"O10\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"I03\0".as_ptr() as *const c_char, control: null(), source: b"I2S Capture\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"I04\0".as_ptr() as *const c_char, control: null(), source: b"I2S Capture\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"I17\0".as_ptr() as *const c_char, control: null(), source: b"I2S Capture\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"I18\0".as_ptr() as *const c_char, control: null(), source: b"I2S Capture\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"O03\0".as_ptr() as *const c_char, control: b"I05 Switch\0".as_ptr() as *const c_char, source: b"I05\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"O04\0".as_ptr() as *const c_char, control: b"I06 Switch\0".as_ptr() as *const c_char, source: b"I06\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"O09\0".as_ptr() as *const c_char, control: b"I17 Switch\0".as_ptr() as *const c_char, source: b"I17\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"O09\0".as_ptr() as *const c_char, control: b"I03 Switch\0".as_ptr() as *const c_char, source: b"I03\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"O10\0".as_ptr() as *const c_char, control: b"I18 Switch\0".as_ptr() as *const c_char, source: b"I18\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"O10\0".as_ptr() as *const c_char, control: b"I04 Switch\0".as_ptr() as *const c_char, source: b"I04\0".as_ptr() as *const c_char },
];

static mt8173_afe_hdmi_routes: [snd_soc_dapm_route; 1] = [
    snd_soc_dapm_route { sink: b"HDMIO Playback\0".as_ptr() as *const c_char, control: null(), source: b"HDMI\0".as_ptr() as *const c_char },
];

static mt8173_afe_pcm_dai_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"mt8173-afe-pcm-dai\0".as_ptr() as *const c_char,
    dapm_widgets: mt8173_afe_pcm_widgets.as_ptr(),
    num_dapm_widgets: mt8173_afe_pcm_widgets.len() as c_uint,
    dapm_routes: mt8173_afe_pcm_routes.as_ptr(),
    num_dapm_routes: mt8173_afe_pcm_routes.len() as c_uint,
    suspend: Some(mtk_afe_suspend),
    resume: Some(mtk_afe_resume),
    debugfs_prefix: b"pcm\0".as_ptr() as *const c_char,
};

static mt8173_afe_hdmi_dai_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"mt8173-afe-hdmi-dai\0".as_ptr() as *const c_char,
    dapm_widgets: null(),
    num_dapm_widgets: 0,
    dapm_routes: mt8173_afe_hdmi_routes.as_ptr(),
    num_dapm_routes: mt8173_afe_hdmi_routes.len() as c_uint,
    suspend: Some(mtk_afe_suspend),
    resume: Some(mtk_afe_resume),
    debugfs_prefix: b"hdmi\0".as_ptr() as *const c_char,
};

static aud_clks: [*const c_char; MT8173_CLK_NUM] = [
    b"infra_sys_audio_clk\0".as_ptr() as *const c_char,
    b"top_pdn_audio\0".as_ptr() as *const c_char,
    b"top_pdn_aud_intbus\0".as_ptr() as *const c_char,
    b"i2s0_m\0".as_ptr() as *const c_char,
    b"i2s1_m\0".as_ptr() as *const c_char,
    b"i2s2_m\0".as_ptr() as *const c_char,
    b"i2s3_m\0".as_ptr() as *const c_char,
    b"i2s3_b\0".as_ptr() as *const c_char,
    b"bck0\0".as_ptr() as *const c_char,
    b"bck1\0".as_ptr() as *const c_char,
];

static memif_data: [mtk_base_memif_data; MT8173_AFE_MEMIF_NUM] = [
    mtk_base_memif_data { name: b"DL1\0".as_ptr() as *const c_char, id: MT8173_AFE_MEMIF_DL1, reg_ofs_base: AFE_DL1_BASE as c_int, reg_ofs_cur: AFE_DL1_CUR as c_int, fs_reg: AFE_DAC_CON1 as c_int, fs_shift: 0, fs_maskbit: 0xf, mono_reg: AFE_DAC_CON1 as c_int, mono_shift: 21, hd_reg: -1, enable_reg: AFE_DAC_CON0 as c_int, enable_shift: 1, msb_reg: AFE_MEMIF_MSB as c_int, msb_shift: 0, agent_disable_reg: -1 },
    mtk_base_memif_data { name: b"DL2\0".as_ptr() as *const c_char, id: MT8173_AFE_MEMIF_DL2, reg_ofs_base: AFE_DL2_BASE as c_int, reg_ofs_cur: AFE_DL2_CUR as c_int, fs_reg: AFE_DAC_CON1 as c_int, fs_shift: 4, fs_maskbit: 0xf, mono_reg: AFE_DAC_CON1 as c_int, mono_shift: 22, hd_reg: -1, enable_reg: AFE_DAC_CON0 as c_int, enable_shift: 2, msb_reg: AFE_MEMIF_MSB as c_int, msb_shift: 1, agent_disable_reg: -1 },
    mtk_base_memif_data { name: b"VUL\0".as_ptr() as *const c_char, id: MT8173_AFE_MEMIF_VUL, reg_ofs_base: AFE_VUL_BASE as c_int, reg_ofs_cur: AFE_VUL_CUR as c_int, fs_reg: AFE_DAC_CON1 as c_int, fs_shift: 16, fs_maskbit: 0xf, mono_reg: AFE_DAC_CON1 as c_int, mono_shift: 27, hd_reg: -1, enable_reg: AFE_DAC_CON0 as c_int, enable_shift: 3, msb_reg: AFE_MEMIF_MSB as c_int, msb_shift: 6, agent_disable_reg: -1 },
    mtk_base_memif_data { name: b"DAI\0".as_ptr() as *const c_char, id: MT8173_AFE_MEMIF_DAI, reg_ofs_base: AFE_DAI_BASE as c_int, reg_ofs_cur: AFE_DAI_CUR as c_int, fs_reg: AFE_DAC_CON0 as c_int, fs_shift: 24, fs_maskbit: 0x3, mono_reg: -1, mono_shift: -1, hd_reg: -1, enable_reg: AFE_DAC_CON0 as c_int, enable_shift: 4, msb_reg: AFE_MEMIF_MSB as c_int, msb_shift: 5, agent_disable_reg: -1 },
    mtk_base_memif_data { name: b"AWB\0".as_ptr() as *const c_char, id: MT8173_AFE_MEMIF_AWB, reg_ofs_base: AFE_AWB_BASE as c_int, reg_ofs_cur: AFE_AWB_CUR as c_int, fs_reg: AFE_DAC_CON1 as c_int, fs_shift: 12, fs_maskbit: 0xf, mono_reg: AFE_DAC_CON1 as c_int, mono_shift: 24, hd_reg: -1, enable_reg: AFE_DAC_CON0 as c_int, enable_shift: 6, msb_reg: AFE_MEMIF_MSB as c_int, msb_shift: 3, agent_disable_reg: -1 },
    mtk_base_memif_data { name: b"MOD_DAI\0".as_ptr() as *const c_char, id: MT8173_AFE_MEMIF_MOD_DAI, reg_ofs_base: AFE_MOD_PCM_BASE as c_int, reg_ofs_cur: AFE_MOD_PCM_CUR as c_int, fs_reg: AFE_DAC_CON1 as c_int, fs_shift: 30, fs_maskbit: 0x3, mono_reg: AFE_DAC_CON1 as c_int, mono_shift: 30, hd_reg: -1, enable_reg: AFE_DAC_CON0 as c_int, enable_shift: 7, msb_reg: AFE_MEMIF_MSB as c_int, msb_shift: 4, agent_disable_reg: -1 },
    mtk_base_memif_data { name: b"HDMI\0".as_ptr() as *const c_char, id: MT8173_AFE_MEMIF_HDMI, reg_ofs_base: AFE_HDMI_OUT_BASE as c_int, reg_ofs_cur: AFE_HDMI_OUT_CUR as c_int, fs_reg: -1, fs_shift: -1, fs_maskbit: -1, mono_reg: -1, mono_shift: -1, hd_reg: -1, enable_reg: -1, enable_shift: 0, msb_reg: AFE_MEMIF_MSB as c_int, msb_shift: 8, agent_disable_reg: -1 },
];

static irq_data: [mtk_base_irq_data; MT8173_AFE_IRQ_NUM] = [
    mtk_base_irq_data { id: MT8173_AFE_IRQ_DL1, irq_cnt_reg: AFE_IRQ_CNT1 as c_int, irq_cnt_shift: 0, irq_cnt_maskbit: 0x3ffff, irq_en_reg: AFE_IRQ_MCU_CON as c_int, irq_en_shift: 0, irq_fs_reg: AFE_IRQ_MCU_CON as c_int, irq_fs_shift: 4, irq_fs_maskbit: 0xf, irq_clr_reg: AFE_IRQ_CLR as c_int, irq_clr_shift: 0 },
    mtk_base_irq_data { id: MT8173_AFE_IRQ_DL2, irq_cnt_reg: AFE_IRQ_CNT1 as c_int, irq_cnt_shift: 20, irq_cnt_maskbit: 0x3ffff, irq_en_reg: AFE_IRQ_MCU_CON as c_int, irq_en_shift: 2, irq_fs_reg: AFE_IRQ_MCU_CON as c_int, irq_fs_shift: 16, irq_fs_maskbit: 0xf, irq_clr_reg: AFE_IRQ_CLR as c_int, irq_clr_shift: 2 },
    mtk_base_irq_data { id: MT8173_AFE_IRQ_VUL, irq_cnt_reg: AFE_IRQ_CNT2 as c_int, irq_cnt_shift: 0, irq_cnt_maskbit: 0x3ffff, irq_en_reg: AFE_IRQ_MCU_CON as c_int, irq_en_shift: 1, irq_fs_reg: AFE_IRQ_MCU_CON as c_int, irq_fs_shift: 8, irq_fs_maskbit: 0xf, irq_clr_reg: AFE_IRQ_CLR as c_int, irq_clr_shift: 1 },
    mtk_base_irq_data { id: MT8173_AFE_IRQ_DAI, irq_cnt_reg: AFE_IRQ_CNT2 as c_int, irq_cnt_shift: 20, irq_cnt_maskbit: 0x3ffff, irq_en_reg: AFE_IRQ_MCU_CON as c_int, irq_en_shift: 3, irq_fs_reg: AFE_IRQ_MCU_CON as c_int, irq_fs_shift: 20, irq_fs_maskbit: 0xf, irq_clr_reg: AFE_IRQ_CLR as c_int, irq_clr_shift: 3 },
    mtk_base_irq_data { id: MT8173_AFE_IRQ_AWB, irq_cnt_reg: AFE_IRQ_CNT7 as c_int, irq_cnt_shift: 0, irq_cnt_maskbit: 0x3ffff, irq_en_reg: AFE_IRQ_MCU_CON as c_int, irq_en_shift: 14, irq_fs_reg: AFE_IRQ_MCU_CON as c_int, irq_fs_shift: 24, irq_fs_maskbit: 0xf, irq_clr_reg: AFE_IRQ_CLR as c_int, irq_clr_shift: 6 },
    mtk_base_irq_data { id: MT8173_AFE_IRQ_DAI, irq_cnt_reg: AFE_IRQ_CNT2 as c_int, irq_cnt_shift: 20, irq_cnt_maskbit: 0x3ffff, irq_en_reg: AFE_IRQ_MCU_CON as c_int, irq_en_shift: 3, irq_fs_reg: AFE_IRQ_MCU_CON as c_int, irq_fs_shift: 20, irq_fs_maskbit: 0xf, irq_clr_reg: AFE_IRQ_CLR as c_int, irq_clr_shift: 3 },
    mtk_base_irq_data { id: MT8173_AFE_IRQ_HDMI, irq_cnt_reg: AFE_IRQ_CNT5 as c_int, irq_cnt_shift: 0, irq_cnt_maskbit: 0x3ffff, irq_en_reg: AFE_IRQ_MCU_CON as c_int, irq_en_shift: 12, irq_fs_reg: -1, irq_fs_shift: 0, irq_fs_maskbit: -1, irq_clr_reg: AFE_IRQ_CLR as c_int, irq_clr_shift: 4 },
];

static mt8173_afe_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: AFE_ADDA2_TOP_CON0,
    cache_type: REGCACHE_NONE,
};

unsafe extern "C" fn mt8173_afe_irq_handler(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let afe = dev_id as *mut mtk_base_afe;
    let mut reg_value: c_uint = 0;
    let mut ret = regmap_read((*afe).regmap, AFE_IRQ_STATUS, &mut reg_value);
    if ret != 0 {
        dev_err((*afe).dev, b"%s irq status err\n\0".as_ptr() as *const c_char, b"mt8173_afe_irq_handler\0".as_ptr());
        reg_value = AFE_IRQ_STATUS_BITS;
    } else {
        let mut i = 0usize;
        while i < MT8173_AFE_MEMIF_NUM {
            let memif = &mut *(*afe).memif.add(i);
            if memif.irq_usage < 0 {
                i += 1;
                continue;
            }
            let irq_p = &mut *(*afe).irqs.add(memif.irq_usage as usize);
            if (reg_value & (1 << (*(*irq_p).irq_data).irq_clr_shift)) == 0 {
                i += 1;
                continue;
            }
            snd_pcm_period_elapsed(memif.substream);
            i += 1;
        }
    }
    /* clear irq */
    ret = regmap_write((*afe).regmap, AFE_IRQ_CLR, reg_value & AFE_IRQ_STATUS_BITS);
    let _ = ret;
    IRQ_HANDLED
}

unsafe extern "C" fn mt8173_afe_runtime_suspend(dev: *mut device) -> c_int {
    let afe = dev_get_drvdata(dev);
    let afe_priv = (*afe).platform_priv;
    /* disable AFE */
    regmap_update_bits((*afe).regmap, AFE_DAC_CON0, 0x1, 0);
    /* disable AFE clk */
    regmap_update_bits((*afe).regmap, AUDIO_TOP_CON0, AUD_TCON0_PDN_AFE, AUD_TCON0_PDN_AFE);
    clk_disable_unprepare((*afe_priv).clocks[MT8173_CLK_I2S1_M]);
    clk_disable_unprepare((*afe_priv).clocks[MT8173_CLK_I2S2_M]);
    clk_disable_unprepare((*afe_priv).clocks[MT8173_CLK_BCK0]);
    clk_disable_unprepare((*afe_priv).clocks[MT8173_CLK_BCK1]);
    clk_disable_unprepare((*afe_priv).clocks[MT8173_CLK_TOP_PDN_AUD]);
    clk_disable_unprepare((*afe_priv).clocks[MT8173_CLK_TOP_PDN_AUD_BUS]);
    clk_disable_unprepare((*afe_priv).clocks[MT8173_CLK_INFRASYS_AUD]);
    0
}

unsafe extern "C" fn mt8173_afe_runtime_resume(dev: *mut device) -> c_int {
    let afe = dev_get_drvdata(dev);
    let afe_priv = (*afe).platform_priv;
    let mut ret = clk_prepare_enable((*afe_priv).clocks[MT8173_CLK_INFRASYS_AUD]);
    if ret != 0 { return ret; }

    ret = clk_prepare_enable((*afe_priv).clocks[MT8173_CLK_TOP_PDN_AUD_BUS]);
    if ret != 0 { clk_disable_unprepare((*afe_priv).clocks[MT8173_CLK_INFRASYS_AUD]); return ret; }

    ret = clk_prepare_enable((*afe_priv).clocks[MT8173_CLK_TOP_PDN_AUD]);
    if ret != 0 {
        clk_disable_unprepare((*afe_priv).clocks[MT8173_CLK_TOP_PDN_AUD_BUS]);
        clk_disable_unprepare((*afe_priv).clocks[MT8173_CLK_INFRASYS_AUD]);
        return ret;
    }

    ret = clk_prepare_enable((*afe_priv).clocks[MT8173_CLK_BCK0]);
    if ret != 0 {
        clk_disable_unprepare((*afe_priv).clocks[MT8173_CLK_TOP_PDN_AUD]);
        clk_disable_unprepare((*afe_priv).clocks[MT8173_CLK_TOP_PDN_AUD_BUS]);
        clk_disable_unprepare((*afe_priv).clocks[MT8173_CLK_INFRASYS_AUD]);
        return ret;
    }

    ret = clk_prepare_enable((*afe_priv).clocks[MT8173_CLK_BCK1]);
    if ret != 0 {
        clk_disable_unprepare((*afe_priv).clocks[MT8173_CLK_BCK0]);
        clk_disable_unprepare((*afe_priv).clocks[MT8173_CLK_TOP_PDN_AUD]);
        clk_disable_unprepare((*afe_priv).clocks[MT8173_CLK_TOP_PDN_AUD_BUS]);
        clk_disable_unprepare((*afe_priv).clocks[MT8173_CLK_INFRASYS_AUD]);
        return ret;
    }
    ret = clk_prepare_enable((*afe_priv).clocks[MT8173_CLK_I2S1_M]);
    if ret != 0 {
        clk_disable_unprepare((*afe_priv).clocks[MT8173_CLK_I2S1_M]);
        clk_disable_unprepare((*afe_priv).clocks[MT8173_CLK_I2S2_M]);
        clk_disable_unprepare((*afe_priv).clocks[MT8173_CLK_BCK0]);
        clk_disable_unprepare((*afe_priv).clocks[MT8173_CLK_TOP_PDN_AUD]);
        clk_disable_unprepare((*afe_priv).clocks[MT8173_CLK_TOP_PDN_AUD_BUS]);
        clk_disable_unprepare((*afe_priv).clocks[MT8173_CLK_INFRASYS_AUD]);
        return ret;
    }
    ret = clk_prepare_enable((*afe_priv).clocks[MT8173_CLK_I2S2_M]);
    if ret != 0 {
        clk_disable_unprepare((*afe_priv).clocks[MT8173_CLK_I2S2_M]);
        clk_disable_unprepare((*afe_priv).clocks[MT8173_CLK_BCK0]);
        clk_disable_unprepare((*afe_priv).clocks[MT8173_CLK_TOP_PDN_AUD]);
        clk_disable_unprepare((*afe_priv).clocks[MT8173_CLK_TOP_PDN_AUD_BUS]);
        clk_disable_unprepare((*afe_priv).clocks[MT8173_CLK_INFRASYS_AUD]);
        return ret;
    }

    /* enable AFE clk */
    regmap_update_bits((*afe).regmap, AUDIO_TOP_CON0, AUD_TCON0_PDN_AFE, 0);
    /* set O3/O4 16bits */
    regmap_update_bits((*afe).regmap, AFE_CONN_24BIT, AFE_CONN_24BIT_O03 | AFE_CONN_24BIT_O04, 0);
    /* unmask all IRQs */
    regmap_update_bits((*afe).regmap, AFE_IRQ_MCU_EN, 0xff, 0xff);
    /* enable AFE */
    regmap_update_bits((*afe).regmap, AFE_DAC_CON0, 0x1, 0x1);
    0
}

unsafe extern "C" fn mt8173_afe_init_audio_clk(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv;
    let mut i: size_t = 0;
    while i < aud_clks.len() {
        (*afe_priv).clocks[i] = devm_clk_get((*afe).dev, aud_clks[i]);
        if IS_ERR((*afe_priv).clocks[i] as *const c_void) {
            dev_err((*afe).dev, b"%s devm_clk_get %s fail\n\0".as_ptr() as *const c_char, b"mt8173_afe_init_audio_clk\0".as_ptr(), aud_clks[i]);
            return PTR_ERR((*afe_priv).clocks[i] as *const c_void);
        }
        i += 1;
    }
    clk_set_rate((*afe_priv).clocks[MT8173_CLK_BCK0], 22579200); /* 22M */
    clk_set_rate((*afe_priv).clocks[MT8173_CLK_BCK1], 24576000); /* 24M */
    0
}

unsafe extern "C" fn mt8173_afe_pcm_dev_probe(pdev: *mut platform_device) -> c_int {
    let mut ret: c_int;
    let mut i: c_int;
    let irq_id: c_int;
    let afe: *mut mtk_base_afe;
    let afe_priv: *mut mt8173_afe_private;
    let dev = &mut (*pdev).dev as *mut device;

    ret = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(33));
    if ret != 0 { return ret; }

    afe = devm_kzalloc(dev, size_of::<mtk_base_afe>(), GFP_KERNEL) as *mut mtk_base_afe;
    if afe.is_null() { return -ENOMEM; }

    (*afe).platform_priv = devm_kzalloc(dev, size_of::<mt8173_afe_private>(), GFP_KERNEL) as *mut mt8173_afe_private;
    afe_priv = (*afe).platform_priv;
    if afe_priv.is_null() { return -ENOMEM; }

    (*afe).dev = dev;

    ret = of_reserved_mem_device_init(dev);
    if ret != 0 {
        dev_info(dev, b"no reserved memory found, pre-allocating buffers instead\n\0".as_ptr() as *const c_char);
        (*afe).preallocate_buffers = true;
    }

    irq_id = platform_get_irq(pdev, 0);
    if irq_id <= 0 {
        return if irq_id < 0 { irq_id } else { -ENXIO };
    }

    (*afe).base_addr = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*afe).base_addr as *const c_void) {
        return PTR_ERR((*afe).base_addr as *const c_void);
    }

    (*afe).regmap = devm_regmap_init_mmio(dev, (*afe).base_addr, &mt8173_afe_regmap_config);
    if IS_ERR((*afe).regmap as *const c_void) {
        return PTR_ERR((*afe).regmap as *const c_void);
    }

    /* initial audio related clock */
    ret = mt8173_afe_init_audio_clk(afe);
    if ret != 0 {
        dev_err(dev, b"mt8173_afe_init_audio_clk fail\n\0".as_ptr() as *const c_char);
        return ret;
    }

    /* memif % irq initialize*/
    (*afe).memif_size = MT8173_AFE_MEMIF_NUM as c_int;
    (*afe).memif = devm_kcalloc(dev, (*afe).memif_size as size_t, size_of::<mtk_base_afe_memif>(), GFP_KERNEL) as *mut mtk_base_afe_memif;
    if (*afe).memif.is_null() { return -ENOMEM; }

    (*afe).irqs_size = MT8173_AFE_IRQ_NUM as c_int;
    (*afe).irqs = devm_kcalloc(dev, (*afe).irqs_size as size_t, size_of::<mtk_base_afe_irq>(), GFP_KERNEL) as *mut mtk_base_afe_irq;
    if (*afe).irqs.is_null() { return -ENOMEM; }

    i = 0;
    while i < (*afe).irqs_size {
        (*(*afe).memif.add(i as usize)).data = &memif_data[i as usize];
        (*(*afe).irqs.add(i as usize)).irq_data = &irq_data[i as usize];
        (*(*afe).irqs.add(i as usize)).irq_occupyed = true;
        (*(*afe).memif.add(i as usize)).irq_usage = i;
        (*(*afe).memif.add(i as usize)).const_irq = 1;
        i += 1;
    }

    (*afe).mtk_afe_hardware = &mt8173_afe_hardware;
    (*afe).memif_fs = Some(mt8173_memif_fs);
    (*afe).irq_fs = Some(mt8173_irq_fs);

    platform_set_drvdata(pdev, afe as *mut c_void);

    pm_runtime_enable(dev);
    if !pm_runtime_enabled(dev) {
        ret = mt8173_afe_runtime_resume(dev);
        if ret != 0 {
            pm_runtime_disable(dev);
            return ret;
        }
    }

    (*afe).reg_back_up_list = mt8173_afe_backup_list.as_ptr();
    (*afe).reg_back_up_list_num = mt8173_afe_backup_list.len() as c_uint;
    (*afe).runtime_resume = Some(mt8173_afe_runtime_resume);
    (*afe).runtime_suspend = Some(mt8173_afe_runtime_suspend);

    ret = devm_snd_soc_register_component(dev, &mtk_afe_pcm_platform, null_mut(), 0);
    if ret != 0 {
        pm_runtime_disable(dev);
        return ret;
    }

    ret = snd_soc_register_component(dev, &mt8173_afe_pcm_dai_component, mt8173_afe_pcm_dais.as_mut_ptr(), mt8173_afe_pcm_dais.len() as c_int);
    if ret != 0 {
        pm_runtime_disable(dev);
        return ret;
    }

    ret = snd_soc_register_component(dev, &mt8173_afe_hdmi_dai_component, mt8173_afe_hdmi_dais.as_mut_ptr(), mt8173_afe_hdmi_dais.len() as c_int);
    if ret != 0 {
        snd_soc_unregister_component(dev);
        pm_runtime_disable(dev);
        return ret;
    }

    ret = devm_request_irq(dev, irq_id, mt8173_afe_irq_handler, 0, b"Afe_ISR_Handle\0".as_ptr() as *const c_char, afe as *mut c_void);
    if ret != 0 {
        dev_err(dev, b"could not request_irq\n\0".as_ptr() as *const c_char);
        snd_soc_unregister_component(dev);
        pm_runtime_disable(dev);
        return ret;
    }

    dev_info(dev, b"MT8173 AFE driver initialized.\n\0".as_ptr() as *const c_char);
    0
}

unsafe extern "C" fn mt8173_afe_pcm_dev_remove(pdev: *mut platform_device) {
    let dev = &mut (*pdev).dev as *mut device;
    snd_soc_unregister_component(dev);
    pm_runtime_disable(dev);
    if !pm_runtime_status_suspended(dev) {
        mt8173_afe_runtime_suspend(dev);
    }
}

static mt8173_afe_pcm_dt_match: [of_device_id; 2] = [
    of_device_id { compatible: b"mediatek,mt8173-afe-pcm\0".as_ptr() as *const c_char },
    of_device_id { compatible: null() },
];
/* MODULE_DEVICE_TABLE(of, mt8173_afe_pcm_dt_match); */

/* RUNTIME_PM_OPS(mt8173_afe_runtime_suspend, mt8173_afe_runtime_resume, NULL) */
static mt8173_afe_pm_ops: dev_pm_ops = dev_pm_ops { _private: [] };

static mut mt8173_afe_pcm_driver: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: b"mt8173-afe-pcm\0".as_ptr() as *const c_char,
        of_match_table: mt8173_afe_pcm_dt_match.as_ptr(),
        pm: unsafe { pm_ptr(&mt8173_afe_pm_ops) },
    },
    probe: Some(mt8173_afe_pcm_dev_probe),
    remove: Some(mt8173_afe_pcm_dev_remove),
};

/* module_platform_driver(mt8173_afe_pcm_driver); */

/* MODULE_DESCRIPTION("Mediatek ALSA SoC AFE platform driver"); */
/* MODULE_AUTHOR("Koro Chen <koro.chen@mediatek.com>"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
