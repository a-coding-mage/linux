// SPDX-License-Identifier: GPL-2.0
//
// Xilinx ASoC audio formatter support
//
// Copyright (C) 2018 Xilinx, Inc.
//
// Author: Maruthi Srinivas Bayyavarapu <maruthis@xilinx.com>

// Linux kernel headers and types from:
// <linux/clk.h>, <linux/io.h>, <linux/module.h>, <linux/sizes.h>
// <sound/asoundef.h>, <sound/soc.h>, <sound/pcm_params.h>

use core::ffi::c_void;

const DRV_NAME: &str = "xlnx_formatter_pcm";

const XLNX_S2MM_OFFSET: u32 = 0;
const XLNX_MM2S_OFFSET: u32 = 0x100;

const XLNX_AUD_CORE_CONFIG: u32 = 0x4;
const XLNX_AUD_CTRL: u32 = 0x10;
const XLNX_AUD_STS: u32 = 0x14;

const AUD_CTRL_RESET_MASK: u32 = 1u32 << 1;
const AUD_CFG_MM2S_MASK: u32 = 1u32 << 15;
const AUD_CFG_S2MM_MASK: u32 = 1u32 << 31;

const XLNX_AUD_FS_MULTIPLIER: u32 = 0x18;
const XLNX_AUD_PERIOD_CONFIG: u32 = 0x1C;
const XLNX_AUD_BUFF_ADDR_LSB: u32 = 0x20;
const XLNX_AUD_BUFF_ADDR_MSB: u32 = 0x24;
const XLNX_AUD_XFER_COUNT: u32 = 0x28;
const XLNX_AUD_CH_STS_START: u32 = 0x2C;
const XLNX_BYTES_PER_CH: u32 = 0x44;
const XLNX_AUD_ALIGN_BYTES: u32 = 64;

const AUD_STS_IOC_IRQ_MASK: u32 = 1u32 << 31;
const AUD_STS_CH_STS_MASK: u32 = 1u32 << 29;
const AUD_CTRL_IOC_IRQ_MASK: u32 = 1u32 << 13;
const AUD_CTRL_TOUT_IRQ_MASK: u32 = 1u32 << 14;
const AUD_CTRL_DMA_EN_MASK: u32 = 1u32 << 0;

const CFG_MM2S_CH_MASK: u32 = 0x0F00;
const CFG_MM2S_CH_SHIFT: u32 = 8;
const CFG_MM2S_XFER_MASK: u32 = 0x6000;
const CFG_MM2S_XFER_SHIFT: u32 = 13;
const CFG_MM2S_PKG_MASK: u32 = 1u32 << 12;

const CFG_S2MM_CH_MASK: u32 = 0x0F000000;
const CFG_S2MM_CH_SHIFT: u32 = 24;
const CFG_S2MM_XFER_MASK: u32 = 0x60000000;
const CFG_S2MM_XFER_SHIFT: u32 = 29;
const CFG_S2MM_PKG_MASK: u32 = 1u32 << 28;

const AUD_CTRL_DATA_WIDTH_SHIFT: u32 = 16;
const AUD_CTRL_ACTIVE_CH_SHIFT: u32 = 19;
const PERIOD_CFG_PERIODS_SHIFT: u32 = 16;

const PERIODS_MIN: u32 = 2;
const PERIODS_MAX: u32 = 6;
const PERIOD_BYTES_MIN: u32 = 192;
const PERIOD_BYTES_MAX: u32 = 50 * 1024;
const XLNX_PARAM_UNKNOWN: u32 = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum BitDepth {
    BitDepth8 = 0,
    BitDepth16 = 1,
    BitDepth20 = 2,
    BitDepth24 = 3,
    BitDepth32 = 4,
}

#[repr(C)]
pub struct XlnxPcmDrvData {
    pub mmio: *mut c_void,
    pub s2mm_presence: bool,
    pub mm2s_presence: bool,
    pub s2mm_irq: i32,
    pub mm2s_irq: i32,
    pub play_stream: *mut c_void,
    pub capture_stream: *mut c_void,
    pub axi_clk: *mut c_void,
    pub sysclk: u32,
}

#[repr(C)]
pub struct XlnxPcmStreamParam {
    pub mmio: *mut c_void,
    pub interleaved: bool,
    pub xfer_mode: u32,
    pub ch_limit: u32,
    pub buffer_size: u64,
}

#[repr(C)]
pub struct SndPcmHardware {
    pub info: u32,
    pub formats: u64,
    pub channels_min: u32,
    pub channels_max: u32,
    pub rates: u32,
    pub rate_min: u32,
    pub rate_max: u32,
    pub buffer_bytes_max: u32,
    pub period_bytes_min: u32,
    pub period_bytes_max: u32,
    pub periods_min: u32,
    pub periods_max: u32,
}

// Constants for snd_pcm_hardware flags (from sound/asoundef.h and sound/pcm.h)
const SNDRV_PCM_INFO_INTERLEAVED: u32 = 1 << 0;
const SNDRV_PCM_INFO_BLOCK_TRANSFER: u32 = 1 << 1;
const SNDRV_PCM_INFO_BATCH: u32 = 1 << 2;
const SNDRV_PCM_INFO_PAUSE: u32 = 1 << 3;
const SNDRV_PCM_INFO_RESUME: u32 = 1 << 4;

const SNDRV_PCM_FMTBIT_S8: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 2;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 6;

const SNDRV_PCM_RATE_8000_192000: u32 = 0xFFF;

static XLNX_PCM_HARDWARE: SndPcmHardware = SndPcmHardware {
    info: SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER |
        SNDRV_PCM_INFO_BATCH | SNDRV_PCM_INFO_PAUSE |
        SNDRV_PCM_INFO_RESUME,
    formats: SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_LE |
        SNDRV_PCM_FMTBIT_S24_LE,
    channels_min: 2,
    channels_max: 2,
    rates: SNDRV_PCM_RATE_8000_192000,
    rate_min: 8000,
    rate_max: 192000,
    buffer_bytes_max: PERIODS_MAX * PERIOD_BYTES_MAX,
    period_bytes_min: PERIOD_BYTES_MIN,
    period_bytes_max: PERIOD_BYTES_MAX,
    periods_min: PERIODS_MIN,
    periods_max: PERIODS_MAX,
};

const AES_TO_AES: u32 = 0;
const AES_TO_PCM: u32 = 1;
const PCM_TO_PCM: u32 = 2;
const PCM_TO_AES: u32 = 3;

// IEC958 constants (from sound/asoundef.h)
const IEC958_AES0_PROFESSIONAL: u32 = 0x01;
const IEC958_AES0_PRO_FS: u32 = 0x0F;
const IEC958_AES0_PRO_FS_44100: u32 = 0x00;
const IEC958_AES0_PRO_FS_48000: u32 = 0x04;
const IEC958_AES0_PRO_FS_32000: u32 = 0x03;
const IEC958_AES0_PRO_FS_NOTID: u32 = 0x0F;

const IEC958_AES2_PRO_SBITS: u32 = 0x0F;
const IEC958_AES2_PRO_WORDLEN_NOTID: u32 = 0x00;
const IEC958_AES2_PRO_SBITS_20: u32 = 0x02;
const IEC958_AES2_PRO_SBITS_24: u32 = 0x0D;
const IEC958_AES2_PRO_WORDLEN: u32 = 0xF0;
const IEC958_AES2_PRO_WORDLEN_20_16: u32 = 0x20;
const IEC958_AES2_PRO_WORDLEN_22_18: u32 = 0x40;
const IEC958_AES2_PRO_WORDLEN_23_19: u32 = 0x50;
const IEC958_AES2_PRO_WORDLEN_24_20: u32 = 0x60;

const IEC958_AES3_CON_FS: u32 = 0x0F;
const IEC958_AES3_CON_FS_44100: u32 = 0x00;
const IEC958_AES3_CON_FS_48000: u32 = 0x02;
const IEC958_AES3_CON_FS_32000: u32 = 0x03;

const IEC958_AES4_CON_MAX_WORDLEN_24: u32 = 0x01;
const IEC958_AES4_CON_WORDLEN: u32 = 0x0F;
const IEC958_AES4_CON_WORDLEN_20_16: u32 = 0x02;
const IEC958_AES4_CON_WORDLEN_22_18: u32 = 0x04;
const IEC958_AES4_CON_WORDLEN_23_19: u32 = 0x05;
const IEC958_AES4_CON_WORDLEN_24_20: u32 = 0x06;
const IEC958_AES4_CON_WORDLEN_21_17: u32 = 0x03;
const IEC958_AES4_CON_WORDLEN_NOTID: u32 = 0x00;

const SNDRV_PCM_STREAM_PLAYBACK: u32 = 0;
const SNDRV_PCM_STREAM_CAPTURE: u32 = 1;

const SNDRV_PCM_TRIGGER_START: u32 = 0;
const SNDRV_PCM_TRIGGER_STOP: u32 = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: u32 = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: u32 = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: u32 = 5;
const SNDRV_PCM_TRIGGER_RESUME: u32 = 6;

const SNDRV_PCM_HW_PARAM_PERIOD_BYTES: u32 = 2;
const SNDRV_PCM_HW_PARAM_BUFFER_BYTES: u32 = 1;
const SNDRV_PCM_HW_PARAM_PERIODS: u32 = 6;

const SNDRV_DMA_TYPE_DEV: u32 = 1;

const IRQ_HANDLED: u32 = 1;
const IRQ_NONE: u32 = 0;

const ENODEV: i32 = -19;
const ENOMEM: i32 = -12;
const EINVAL: i32 = -22;

extern "C" {
    fn readl(addr: *const c_void) -> u32;
    fn writel(val: u32, addr: *mut c_void);
    fn mdelay(msecs: u32);

    fn dev_info(dev: *const c_void, fmt: *const u8, ...);
    fn dev_err(dev: *const c_void, fmt: *const u8, ...);
    fn dev_warn(dev: *const c_void, fmt: *const u8, ...);
    fn dev_get_drvdata(dev: *const c_void) -> *mut c_void;
    fn dev_set_drvdata(dev: *const c_void, data: *mut c_void);

    fn snd_soc_set_runtime_hwparams(substream: *mut c_void, hw: *const SndPcmHardware);
    fn snd_pcm_hw_constraint_step(runtime: *mut c_void, cond: u32, var: u32, step: u32) -> i32;
    fn snd_pcm_hw_constraint_integer(runtime: *mut c_void, var: u32) -> i32;
    fn snd_pcm_period_elapsed(substream: *mut c_void);
    fn bytes_to_frames(runtime: *const c_void, bytes: u32) -> u32;
    fn params_channels(params: *const c_void) -> u32;
    fn params_rate(params: *const c_void) -> u32;
    fn params_buffer_bytes(params: *const c_void) -> u64;
    fn params_width(params: *const c_void) -> u32;
    fn params_periods(params: *const c_void) -> u32;
    fn params_period_bytes(params: *const c_void) -> u32;

    fn devm_kzalloc(dev: *const c_void, size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn devm_clk_get(dev: *const c_void, id: *const u8) -> *mut c_void;
    fn clk_prepare_enable(clk: *mut c_void) -> i32;
    fn clk_disable_unprepare(clk: *mut c_void);
    fn devm_platform_ioremap_resource(pdev: *mut c_void, index: u32) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> i32;
    fn platform_get_irq_byname(pdev: *mut c_void, name: *const u8) -> i32;
    fn devm_request_irq(dev: *const c_void, irq: u32, handler: extern "C" fn(i32, *mut c_void) -> u32,
                        flags: u32, name: *const u8, dev_id: *mut c_void) -> i32;

    fn devm_snd_soc_register_component(dev: *const c_void, component_driver: *const c_void,
                                       dai_drv: *const c_void, num_dais: i32) -> i32;
    fn snd_pcm_set_managed_buffer_all(pcm: *mut c_void, dma_type: u32, dev: *const c_void,
                                       size: u32, max_size: u32);
}

fn lower_32_bits(val: u64) -> u32 {
    (val & 0xFFFFFFFF) as u32
}

fn upper_32_bits(val: u64) -> u32 {
    ((val >> 32) & 0xFFFFFFFF) as u32
}

fn div_round_up(dividend: u32, divisor: u32) -> u32 {
    (dividend + divisor - 1) / divisor
}

unsafe fn xlnx_parse_aes_params(chsts_reg1_val: u32, chsts_reg2_val: u32, dev: *const c_void) {
    let mut padded: u32;
    let mut srate: u32 = XLNX_PARAM_UNKNOWN;
    let mut bit_depth: u32 = XLNX_PARAM_UNKNOWN;
    let mut status: [u32; 2] = [0; 2];

    if (chsts_reg1_val & IEC958_AES0_PROFESSIONAL) != 0 {
        status[0] = chsts_reg1_val & 0xff;
        status[1] = (chsts_reg1_val >> 16) & 0xff;

        match status[0] & IEC958_AES0_PRO_FS {
            IEC958_AES0_PRO_FS_44100 => {
                srate = 44100;
            }
            IEC958_AES0_PRO_FS_48000 => {
                srate = 48000;
            }
            IEC958_AES0_PRO_FS_32000 => {
                srate = 32000;
            }
            _ => {
                srate = XLNX_PARAM_UNKNOWN;
            }
        }

        match status[1] & IEC958_AES2_PRO_SBITS {
            IEC958_AES2_PRO_WORDLEN_NOTID | IEC958_AES2_PRO_SBITS_20 => {
                padded = 0;
            }
            IEC958_AES2_PRO_SBITS_24 => {
                padded = 4;
            }
            _ => {
                bit_depth = XLNX_PARAM_UNKNOWN;
                goto_log_params(dev, srate, bit_depth);
                return;
            }
        }

        match status[1] & IEC958_AES2_PRO_WORDLEN {
            IEC958_AES2_PRO_WORDLEN_20_16 => {
                bit_depth = 16 + padded;
            }
            IEC958_AES2_PRO_WORDLEN_22_18 => {
                bit_depth = 18 + padded;
            }
            IEC958_AES2_PRO_WORDLEN_23_19 => {
                bit_depth = 19 + padded;
            }
            IEC958_AES2_PRO_WORDLEN_24_20 => {
                bit_depth = 20 + padded;
            }
            _ => {
                bit_depth = XLNX_PARAM_UNKNOWN;
            }
        }
    } else {
        status[0] = (chsts_reg1_val >> 24) & 0xff;
        status[1] = chsts_reg2_val & 0xff;

        match status[0] & IEC958_AES3_CON_FS {
            IEC958_AES3_CON_FS_44100 => {
                srate = 44100;
            }
            IEC958_AES3_CON_FS_48000 => {
                srate = 48000;
            }
            IEC958_AES3_CON_FS_32000 => {
                srate = 32000;
            }
            _ => {
                srate = XLNX_PARAM_UNKNOWN;
            }
        }

        if (status[1] & IEC958_AES4_CON_MAX_WORDLEN_24) != 0 {
            padded = 4;
        } else {
            padded = 0;
        }

        match status[1] & IEC958_AES4_CON_WORDLEN {
            IEC958_AES4_CON_WORDLEN_20_16 => {
                bit_depth = 16 + padded;
            }
            IEC958_AES4_CON_WORDLEN_22_18 => {
                bit_depth = 18 + padded;
            }
            IEC958_AES4_CON_WORDLEN_23_19 => {
                bit_depth = 19 + padded;
            }
            IEC958_AES4_CON_WORDLEN_24_20 => {
                bit_depth = 20 + padded;
            }
            IEC958_AES4_CON_WORDLEN_21_17 => {
                bit_depth = 17 + padded;
            }
            _ => {
                bit_depth = XLNX_PARAM_UNKNOWN;
            }
        }
    }

    goto_log_params(dev, srate, bit_depth);
}

unsafe fn goto_log_params(dev: *const c_void, srate: u32, bit_depth: u32) {
    if srate != XLNX_PARAM_UNKNOWN {
        dev_info(dev, b"sample rate = %d\n\0".as_ptr(), srate);
    } else {
        dev_info(dev, b"sample rate = unknown\n\0".as_ptr());
    }

    if bit_depth != XLNX_PARAM_UNKNOWN {
        dev_info(dev, b"bit_depth = %d\n\0".as_ptr(), bit_depth);
    } else {
        dev_info(dev, b"bit_depth = unknown\n\0".as_ptr());
    }
}

unsafe fn xlnx_formatter_pcm_reset(mmio_base: *mut c_void) -> i32 {
    let mut val: u32;
    let mut retries: u32 = 0;

    val = readl(mmio_base);
    val |= AUD_CTRL_RESET_MASK;
    writel(val, mmio_base);

    val = readl(mmio_base);
    while (val & AUD_CTRL_RESET_MASK) != 0 && retries < 100 {
        mdelay(1);
        retries += 1;
        val = readl(mmio_base);
    }
    if (val & AUD_CTRL_RESET_MASK) != 0 {
        return ENODEV;
    }

    0
}

unsafe fn xlnx_formatter_disable_irqs(mmio_base: *mut c_void, stream: u32) {
    let mut val: u32;

    val = readl(mmio_base);
    val &= !AUD_CTRL_IOC_IRQ_MASK;
    if stream == SNDRV_PCM_STREAM_CAPTURE {
        val &= !AUD_CTRL_TOUT_IRQ_MASK;
    }

    writel(val, mmio_base);
}

unsafe extern "C" fn xlnx_mm2s_irq_handler(irq: i32, arg: *mut c_void) -> u32 {
    let val: u32;
    let reg: *mut c_void;
    let adata = arg as *mut XlnxPcmDrvData;

    reg = ((*adata).mmio as usize + XLNX_MM2S_OFFSET as usize + XLNX_AUD_STS as usize) as *mut c_void;
    val = readl(reg as *const c_void);
    if (val & AUD_STS_IOC_IRQ_MASK) != 0 {
        writel(val & AUD_STS_IOC_IRQ_MASK, reg);
        if !(*adata).play_stream.is_null() {
            snd_pcm_period_elapsed((*adata).play_stream);
        }
        return IRQ_HANDLED;
    }

    IRQ_NONE
}

unsafe extern "C" fn xlnx_s2mm_irq_handler(irq: i32, arg: *mut c_void) -> u32 {
    let val: u32;
    let reg: *mut c_void;
    let adata = arg as *mut XlnxPcmDrvData;

    reg = ((*adata).mmio as usize + XLNX_S2MM_OFFSET as usize + XLNX_AUD_STS as usize) as *mut c_void;
    val = readl(reg as *const c_void);
    if (val & AUD_STS_IOC_IRQ_MASK) != 0 {
        writel(val & AUD_STS_IOC_IRQ_MASK, reg);
        if !(*adata).capture_stream.is_null() {
            snd_pcm_period_elapsed((*adata).capture_stream);
        }
        return IRQ_HANDLED;
    }

    IRQ_NONE
}

unsafe extern "C" fn xlnx_formatter_set_sysclk(component: *mut c_void, clk_id: i32, source: i32,
                                              freq: u32, dir: i32) -> i32 {
    let dev = (component as usize + 0) as *const c_void; // Simplified - normally gets component->dev
    let adata = dev_get_drvdata(dev) as *mut XlnxPcmDrvData;

    (*adata).sysclk = freq;
    0
}

unsafe extern "C" fn xlnx_formatter_pcm_open(component: *mut c_void, substream: *mut c_void) -> i32 {
    let err: i32;
    let mut val: u32;
    let data_format_mode: u32;
    let mut ch_count_mask: u32;
    let mut ch_count_shift: u32;
    let mut data_xfer_mode: u32;
    let mut data_xfer_shift: u32;
    let stream_data: *mut XlnxPcmStreamParam;
    let runtime: *mut c_void;
    let adata: *mut XlnxPcmDrvData;
    let dev: *const c_void;
    let stream: u32;

    dev = (component as usize + 0) as *const c_void;
    adata = dev_get_drvdata(dev) as *mut XlnxPcmDrvData;

    stream = (substream as usize) as u32; // Simplified access to substream->stream

    if stream == SNDRV_PCM_STREAM_PLAYBACK && !(*adata).mm2s_presence {
        return ENODEV;
    } else if stream == SNDRV_PCM_STREAM_CAPTURE && !(*adata).s2mm_presence {
        return ENODEV;
    }

    stream_data = devm_kzalloc(dev, core::mem::size_of::<XlnxPcmStreamParam>(), 0) as *mut XlnxPcmStreamParam;
    if stream_data.is_null() {
        return ENOMEM;
    }

    if stream == SNDRV_PCM_STREAM_PLAYBACK {
        ch_count_mask = CFG_MM2S_CH_MASK;
        ch_count_shift = CFG_MM2S_CH_SHIFT;
        data_xfer_mode = CFG_MM2S_XFER_MASK;
        data_xfer_shift = CFG_MM2S_XFER_SHIFT;
        data_format_mode = CFG_MM2S_PKG_MASK;
        (*stream_data).mmio = ((*adata).mmio as usize + XLNX_MM2S_OFFSET as usize) as *mut c_void;
        (*adata).play_stream = substream;
    } else {
        ch_count_mask = CFG_S2MM_CH_MASK;
        ch_count_shift = CFG_S2MM_CH_SHIFT;
        data_xfer_mode = CFG_S2MM_XFER_MASK;
        data_xfer_shift = CFG_S2MM_XFER_SHIFT;
        data_format_mode = CFG_S2MM_PKG_MASK;
        (*stream_data).mmio = ((*adata).mmio as usize + XLNX_S2MM_OFFSET as usize) as *mut c_void;
        (*adata).capture_stream = substream;
    }

    val = readl(((*adata).mmio as usize + XLNX_AUD_CORE_CONFIG as usize) as *const c_void);

    if (val & data_format_mode) == 0 {
        (*stream_data).interleaved = true;
    }

    (*stream_data).xfer_mode = (val & data_xfer_mode) >> data_xfer_shift;
    (*stream_data).ch_limit = (val & ch_count_mask) >> ch_count_shift;
    dev_info(dev,
        b"stream %d : format = %d mode = %d ch_limit = %d\n\0".as_ptr(),
        stream, (*stream_data).interleaved as u32,
        (*stream_data).xfer_mode, (*stream_data).ch_limit);

    snd_soc_set_runtime_hwparams(substream, &XLNX_PCM_HARDWARE);

    runtime = (substream as usize + 0) as *mut c_void; // Simplified - normally gets substream->runtime

    let err = snd_pcm_hw_constraint_step(runtime, 0,
                        SNDRV_PCM_HW_PARAM_PERIOD_BYTES,
                        XLNX_AUD_ALIGN_BYTES);
    if err != 0 {
        dev_err(dev,
            b"Unable to set constraint on period bytes\n\0".as_ptr());
        goto_error_open(dev, substream, adata, stream_data, err)
    } else {
        let err = snd_pcm_hw_constraint_step(runtime, 0,
                            SNDRV_PCM_HW_PARAM_BUFFER_BYTES,
                            XLNX_AUD_ALIGN_BYTES);
        if err != 0 {
            dev_err(dev,
                b"Unable to set constraint on buffer bytes\n\0".as_ptr());
            goto_error_open(dev, substream, adata, stream_data, err)
        } else {
            let err = snd_pcm_hw_constraint_integer(runtime,
                                SNDRV_PCM_HW_PARAM_PERIODS);
            if err < 0 {
                dev_err(dev,
                    b"Unable to set constraint on periods to be integer\n\0".as_ptr());
                goto_error_open(dev, substream, adata, stream_data, err)
            } else {
                val = readl(((*stream_data).mmio as usize + XLNX_AUD_CTRL as usize) as *const c_void);
                val |= AUD_CTRL_IOC_IRQ_MASK;
                writel(val, ((*stream_data).mmio as usize + XLNX_AUD_CTRL as usize) as *mut c_void);

                0
            }
        }
    }
}

unsafe fn goto_error_open(dev: *const c_void, substream: *mut c_void, adata: *mut XlnxPcmDrvData,
                          stream_data: *mut XlnxPcmStreamParam, err: i32) -> i32 {
    let stream: u32 = (substream as usize) as u32;
    if stream == SNDRV_PCM_STREAM_PLAYBACK {
        (*adata).play_stream = core::ptr::null_mut();
    } else {
        (*adata).capture_stream = core::ptr::null_mut();
    }
    kfree(stream_data as *mut c_void);
    err
}

unsafe extern "C" fn xlnx_formatter_pcm_close(component: *mut c_void, substream: *mut c_void) -> i32 {
    let ret: i32;
    let stream_data: *mut XlnxPcmStreamParam;
    let dev: *const c_void;

    dev = (component as usize + 0) as *const c_void;
    stream_data = (substream as usize + 0) as *mut XlnxPcmStreamParam; // Simplified access to substream->runtime->private_data

    ret = xlnx_formatter_pcm_reset(((*stream_data).mmio as usize + XLNX_AUD_CTRL as usize) as *mut c_void);
    if ret != 0 {
        dev_err(dev, b"audio formatter reset failed\n\0".as_ptr());
    }
    xlnx_formatter_disable_irqs(((*stream_data).mmio as usize + XLNX_AUD_CTRL as usize) as *mut c_void,
                                 (substream as usize) as u32);

    kfree(stream_data as *mut c_void);
    0
}

unsafe extern "C" fn xlnx_formatter_pcm_pointer(component: *mut c_void, substream: *mut c_void) -> u32 {
    let pos: u32;
    let runtime: *mut c_void;
    let stream_data: *mut XlnxPcmStreamParam;

    runtime = (substream as usize + 0) as *mut c_void;
    stream_data = (runtime as usize + 0) as *mut XlnxPcmStreamParam;

    pos = readl(((*stream_data).mmio as usize + XLNX_AUD_XFER_COUNT as usize) as *const c_void);

    let final_pos = if pos >= (*stream_data).buffer_size as u32 {
        0
    } else {
        pos
    };

    bytes_to_frames(runtime, final_pos)
}

unsafe extern "C" fn xlnx_formatter_pcm_hw_params(component: *mut c_void, substream: *mut c_void,
                                                   params: *mut c_void) -> i32 {
    let mut low: u32;
    let mut high: u32;
    let active_ch: u32;
    let mut val: u32;
    let bytes_per_ch: u32;
    let bits_per_sample: u32;
    let aes_reg1_val: u32;
    let aes_reg2_val: u32;
    let size: u64;
    let runtime: *mut c_void;
    let stream_data: *mut XlnxPcmStreamParam;
    let adata: *mut XlnxPcmDrvData;
    let dev: *const c_void;
    let stream: u32;
    let dma_addr: u64 = 0; // Simplified - normally gets runtime->dma_addr

    dev = (component as usize + 0) as *const c_void;
    adata = dev_get_drvdata(dev) as *mut XlnxPcmDrvData;
    runtime = (substream as usize + 0) as *mut c_void;
    stream_data = (runtime as usize + 0) as *mut XlnxPcmStreamParam;
    stream = (substream as usize) as u32;

    let active_ch = params_channels(params);
    if active_ch > (*stream_data).ch_limit {
        return EINVAL;
    }

    if stream == SNDRV_PCM_STREAM_PLAYBACK && (*adata).sysclk != 0 {
        let mclk_fs = (*adata).sysclk / params_rate(params);

        if (*adata).sysclk % params_rate(params) != 0 {
            dev_warn(dev, b"sysclk %u not divisible by rate %u\n\0".as_ptr(),
                (*adata).sysclk, params_rate(params));
            return EINVAL;
        }

        writel(mclk_fs, ((*stream_data).mmio as usize + XLNX_AUD_FS_MULTIPLIER as usize) as *mut c_void);
    }

    if stream == SNDRV_PCM_STREAM_CAPTURE && (*stream_data).xfer_mode == AES_TO_PCM {
        val = readl(((*stream_data).mmio as usize + XLNX_AUD_STS as usize) as *const c_void);
        if (val & AUD_STS_CH_STS_MASK) != 0 {
            aes_reg1_val = readl(((*stream_data).mmio as usize + XLNX_AUD_CH_STS_START as usize) as *const c_void);
            aes_reg2_val = readl(((*stream_data).mmio as usize + XLNX_AUD_CH_STS_START as usize + 0x4) as *const c_void);

            xlnx_parse_aes_params(aes_reg1_val, aes_reg2_val, dev);
        }
    }

    let size = params_buffer_bytes(params);
    (*stream_data).buffer_size = size;

    low = lower_32_bits(dma_addr);
    high = upper_32_bits(dma_addr);
    writel(low, ((*stream_data).mmio as usize + XLNX_AUD_BUFF_ADDR_LSB as usize) as *mut c_void);
    writel(high, ((*stream_data).mmio as usize + XLNX_AUD_BUFF_ADDR_MSB as usize) as *mut c_void);

    val = readl(((*stream_data).mmio as usize + XLNX_AUD_CTRL as usize) as *const c_void);
    let bits_per_sample = params_width(params);
    match bits_per_sample {
        8 => {
            val |= (BitDepth::BitDepth8 as u32) << AUD_CTRL_DATA_WIDTH_SHIFT;
        }
        16 => {
            val |= (BitDepth::BitDepth16 as u32) << AUD_CTRL_DATA_WIDTH_SHIFT;
        }
        20 => {
            val |= (BitDepth::BitDepth20 as u32) << AUD_CTRL_DATA_WIDTH_SHIFT;
        }
        24 => {
            val |= (BitDepth::BitDepth24 as u32) << AUD_CTRL_DATA_WIDTH_SHIFT;
        }
        32 => {
            val |= (BitDepth::BitDepth32 as u32) << AUD_CTRL_DATA_WIDTH_SHIFT;
        }
        _ => {
            return EINVAL;
        }
    }

    val |= active_ch << AUD_CTRL_ACTIVE_CH_SHIFT;
    writel(val, ((*stream_data).mmio as usize + XLNX_AUD_CTRL as usize) as *mut c_void);

    val = (params_periods(params) << PERIOD_CFG_PERIODS_SHIFT) | params_period_bytes(params);
    writel(val, ((*stream_data).mmio as usize + XLNX_AUD_PERIOD_CONFIG as usize) as *mut c_void);
    let bytes_per_ch = div_round_up(params_period_bytes(params), active_ch);
    writel(bytes_per_ch, ((*stream_data).mmio as usize + XLNX_BYTES_PER_CH as usize) as *mut c_void);

    0
}

unsafe extern "C" fn xlnx_formatter_pcm_trigger(component: *mut c_void, substream: *mut c_void, cmd: i32) -> i32 {
    let mut val: u32;
    let stream_data: *mut XlnxPcmStreamParam;

    stream_data = (substream as usize + 0) as *mut XlnxPcmStreamParam;

    match cmd as u32 {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_PAUSE_RELEASE | SNDRV_PCM_TRIGGER_RESUME => {
            val = readl(((*stream_data).mmio as usize + XLNX_AUD_CTRL as usize) as *const c_void);
            val |= AUD_CTRL_DMA_EN_MASK;
            writel(val, ((*stream_data).mmio as usize + XLNX_AUD_CTRL as usize) as *mut c_void);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_SUSPEND => {
            val = readl(((*stream_data).mmio as usize + XLNX_AUD_CTRL as usize) as *const c_void);
            val &= !AUD_CTRL_DMA_EN_MASK;
            writel(val, ((*stream_data).mmio as usize + XLNX_AUD_CTRL as usize) as *mut c_void);
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn xlnx_formatter_pcm_new(component: *mut c_void, rtd: *mut c_void) -> i32 {
    let pcm: *mut c_void = (rtd as usize + 0) as *mut c_void;
    let dev: *const c_void = (component as usize + 0) as *const c_void;

    snd_pcm_set_managed_buffer_all(pcm,
            SNDRV_DMA_TYPE_DEV, dev,
            XLNX_PCM_HARDWARE.buffer_bytes_max,
            XLNX_PCM_HARDWARE.buffer_bytes_max);
    0
}

#[repr(C)]
pub struct SndSocComponentDriver {
    pub name: *const u8,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut c_void, i32, i32, u32, i32) -> i32>,
    pub open: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> i32>,
    pub close: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> i32>,
    pub hw_params: Option<unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> i32>,
    pub trigger: Option<unsafe extern "C" fn(*mut c_void, *mut c_void, i32) -> i32>,
    pub pointer: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> u32>,
    pub pcm_new: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> i32>,
}

static XLNX_ASOC_COMPONENT: SndSocComponentDriver = SndSocComponentDriver {
    name: DRV_NAME.as_bytes().as_ptr(),
    set_sysclk: Some(xlnx_formatter_set_sysclk),
    open: Some(xlnx_formatter_pcm_open),
    close: Some(xlnx_formatter_pcm_close),
    hw_params: Some(xlnx_formatter_pcm_hw_params),
    trigger: Some(xlnx_formatter_pcm_trigger),
    pointer: Some(xlnx_formatter_pcm_pointer),
    pcm_new: Some(xlnx_formatter_pcm_new),
};

unsafe extern "C" fn xlnx_formatter_pcm_probe(pdev: *mut c_void) -> i32 {
    let ret: i32;
    let mut val: u32;
    let aud_drv_data: *mut XlnxPcmDrvData;
    let dev: *const c_void = pdev as *const c_void;

    aud_drv_data = devm_kzalloc(dev, core::mem::size_of::<XlnxPcmDrvData>(), 0) as *mut XlnxPcmDrvData;
    if aud_drv_data.is_null() {
        return ENOMEM;
    }

    (*aud_drv_data).axi_clk = devm_clk_get(dev, b"s_axi_lite_aclk\0".as_ptr());
    if IS_ERR((*aud_drv_data).axi_clk) {
        ret = PTR_ERR((*aud_drv_data).axi_clk);
        dev_err(dev, b"failed to get s_axi_lite_aclk(%d)\n\0".as_ptr(), ret);
        return ret;
    }
    ret = clk_prepare_enable((*aud_drv_data).axi_clk);
    if ret != 0 {
        dev_err(dev, b"failed to enable s_axi_lite_aclk(%d)\n\0".as_ptr(), ret);
        return ret;
    }

    (*aud_drv_data).mmio = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*aud_drv_data).mmio as *const c_void) {
        dev_err(dev, b"audio formatter ioremap failed\n\0".as_ptr());
        ret = PTR_ERR((*aud_drv_data).mmio as *const c_void);
        clk_disable_unprepare((*aud_drv_data).axi_clk);
        return ret;
    }

    val = readl(((*aud_drv_data).mmio as usize + XLNX_AUD_CORE_CONFIG as usize) as *const c_void);
    if (val & AUD_CFG_MM2S_MASK) != 0 {
        (*aud_drv_data).mm2s_presence = true;
        ret = xlnx_formatter_pcm_reset(((*aud_drv_data).mmio as usize + XLNX_MM2S_OFFSET as usize) as *mut c_void);
        if ret != 0 {
            dev_err(dev, b"audio formatter reset failed\n\0".as_ptr());
            clk_disable_unprepare((*aud_drv_data).axi_clk);
            return ret;
        }
        xlnx_formatter_disable_irqs(((*aud_drv_data).mmio as usize + XLNX_MM2S_OFFSET as usize) as *mut c_void,
                            SNDRV_PCM_STREAM_PLAYBACK);

        (*aud_drv_data).mm2s_irq = platform_get_irq_byname(pdev, b"irq_mm2s\0".as_ptr());
        if (*aud_drv_data).mm2s_irq < 0 {
            ret = (*aud_drv_data).mm2s_irq;
            clk_disable_unprepare((*aud_drv_data).axi_clk);
            return ret;
        }
        ret = devm_request_irq(dev, (*aud_drv_data).mm2s_irq as u32,
                       xlnx_mm2s_irq_handler, 0,
                       b"xlnx_formatter_pcm_mm2s_irq\0".as_ptr(), aud_drv_data as *mut c_void);
        if ret != 0 {
            dev_err(dev, b"xlnx audio mm2s irq request failed\n\0".as_ptr());
            clk_disable_unprepare((*aud_drv_data).axi_clk);
            return ret;
        }
    }
    if (val & AUD_CFG_S2MM_MASK) != 0 {
        (*aud_drv_data).s2mm_presence = true;
        ret = xlnx_formatter_pcm_reset(((*aud_drv_data).mmio as usize + XLNX_S2MM_OFFSET as usize) as *mut c_void);
        if ret != 0 {
            dev_err(dev, b"audio formatter reset failed\n\0".as_ptr());
            clk_disable_unprepare((*aud_drv_data).axi_clk);
            return ret;
        }
        xlnx_formatter_disable_irqs(((*aud_drv_data).mmio as usize + XLNX_S2MM_OFFSET as usize) as *mut c_void,
                            SNDRV_PCM_STREAM_CAPTURE);

        (*aud_drv_data).s2mm_irq = platform_get_irq_byname(pdev, b"irq_s2mm\0".as_ptr());
        if (*aud_drv_data).s2mm_irq < 0 {
            ret = (*aud_drv_data).s2mm_irq;
            clk_disable_unprepare((*aud_drv_data).axi_clk);
            return ret;
        }
        ret = devm_request_irq(dev, (*aud_drv_data).s2mm_irq as u32,
                       xlnx_s2mm_irq_handler, 0,
                       b"xlnx_formatter_pcm_s2mm_irq\0".as_ptr(),
                       aud_drv_data as *mut c_void);
        if ret != 0 {
            dev_err(dev, b"xlnx audio s2mm irq request failed\n\0".as_ptr());
            clk_disable_unprepare((*aud_drv_data).axi_clk);
            return ret;
        }
    }

    dev_set_drvdata(dev, aud_drv_data as *mut c_void);

    ret = devm_snd_soc_register_component(dev, &XLNX_ASOC_COMPONENT as *const _ as *const c_void,
                          core::ptr::null(), 0);
    if ret != 0 {
        dev_err(dev, b"pcm platform device register failed\n\0".as_ptr());
        clk_disable_unprepare((*aud_drv_data).axi_clk);
        return ret;
    }

    0
}

unsafe extern "C" fn xlnx_formatter_pcm_remove(pdev: *mut c_void) {
    let mut ret: i32 = 0;
    let adata: *mut XlnxPcmDrvData = dev_get_drvdata(pdev as *const c_void) as *mut XlnxPcmDrvData;

    if (*adata).s2mm_presence {
        ret = xlnx_formatter_pcm_reset(((*adata).mmio as usize + XLNX_S2MM_OFFSET as usize) as *mut c_void);
    }

    if (*adata).mm2s_presence {
        ret = xlnx_formatter_pcm_reset(((*adata).mmio as usize + XLNX_MM2S_OFFSET as usize) as *mut c_void);
    }

    if ret != 0 {
        dev_err(pdev as *const c_void, b"audio formatter reset failed\n\0".as_ptr());
    }

    clk_disable_unprepare((*adata).axi_clk);
}

#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const u8,
    pub data: *const c_void,
}

static XLNX_FORMATTER_PCM_OF_MATCH: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: b"xlnx,audio-formatter-1.0\0".as_ptr(),
        data: core::ptr::null(),
    },
    OfDeviceId {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

#[repr(C)]
pub struct PlatformDriver {
    pub probe: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut c_void)>,
    pub driver: PlatformDriverInfo,
}

#[repr(C)]
pub struct PlatformDriverInfo {
    pub name: *const u8,
    pub of_match_table: *const OfDeviceId,
}

static XLNX_FORMATTER_PCM_DRIVER: PlatformDriver = PlatformDriver {
    probe: Some(xlnx_formatter_pcm_probe),
    remove: Some(xlnx_formatter_pcm_remove),
    driver: PlatformDriverInfo {
        name: DRV_NAME.as_bytes().as_ptr(),
        of_match_table: &XLNX_FORMATTER_PCM_OF_MATCH[0],
    },
};

// Module initialization and metadata
// Corresponds to: module_platform_driver(xlnx_formatter_pcm_driver)
// MODULE_DESCRIPTION, MODULE_AUTHOR, MODULE_LICENSE, MODULE_DEVICE_TABLE are kernel metadata

// Note: The actual module_platform_driver macro expansion would register the platform driver
// with the kernel's driver framework. This requires kernel module infrastructure.

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
