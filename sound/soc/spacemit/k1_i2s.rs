// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Troy Mitchell <troy.mitchell@linux.spacemit.com> */

/* Dependencies in the original C source:
 * linux/bitfield.h, linux/clk.h, linux/reset.h,
 * sound/dmaengine_pcm.h, sound/pcm.h, sound/pcm_params.h
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u32 = u32;
type u64 = u64;
type dma_addr_t = c_ulong;

const fn bit(n: u32) -> u32 {
    1u32 << n
}

const fn genmask(h: u32, l: u32) -> u32 {
    ((!0u32) << l) & ((!0u32) >> (31 - h))
}

const fn field_prep(mask: u32, val: u32) -> u32 {
    (val << mask.trailing_zeros()) & mask
}

const SSCR: usize = 0x00; /* SPI/I2S top control register */
const SSFCR: usize = 0x04; /* SPI/I2S FIFO control register */
const SSINTEN: usize = 0x08; /* SPI/I2S interrupt enable register */
const SSDATR: usize = 0x10; /* SPI/I2S data register */
const SSPSP: usize = 0x18; /* SPI/I2S programmable serial protocol control register */
const SSRWT: usize = 0x24; /* SPI/I2S root control register */

/* SPI/I2S Work data size, register bits value 0~31 indicated data size 1~32 bits */
const SSCR_FIELD_DSS: u32 = genmask(9, 5);
const SSCR_DW_8BYTE: u32 = field_prep(SSCR_FIELD_DSS, 0x7);
const SSCR_DW_16BYTE: u32 = field_prep(SSCR_FIELD_DSS, 0xf);
const SSCR_DW_18BYTE: u32 = field_prep(SSCR_FIELD_DSS, 0x11);
const SSCR_DW_32BYTE: u32 = field_prep(SSCR_FIELD_DSS, 0x1f);

const SSCR_SSE: u32 = bit(0); /* SPI/I2S Enable */
const SSCR_FRF_PSP: u32 = genmask(2, 1); /* Frame Format*/
const SSCR_TRAIL: u32 = bit(13); /* Trailing Byte */

const SSFCR_FIELD_TFT: u32 = genmask(3, 0); /* TXFIFO Trigger Threshold */
const SSFCR_FIELD_RFT: u32 = genmask(8, 5); /* RXFIFO Trigger Threshold */
const SSFCR_TSRE: u32 = bit(10); /* Transmit Service Request Enable */
const SSFCR_RSRE: u32 = bit(11); /* Receive Service Request Enable */

const SSPSP_FSRT: u32 = bit(3); /* Frame Sync Relative Timing Bit */
const SSPSP_SFRMP: u32 = bit(4); /* Serial Frame Polarity */
const SSPSP_FIELD_SFRMWDTH: u32 = genmask(17, 12); /* Serial Frame Width field  */

const SSRWT_RWOT: u32 = bit(0); /* Receive Without Transmit */

const SPACEMIT_PCM_RATES: u32 =
    SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_48000;
const SPACEMIT_PCM_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE;

const SPACEMIT_I2S_PERIOD_SIZE: u32 = 1024;

#[repr(C)]
struct device {
    of_node: *mut device_node,
}

#[repr(C)]
struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
struct reset_control {
    _private: [u8; 0],
}

#[repr(C)]
struct clk {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_substream {
    stream: c_int,
    runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dai {
    dev: *mut device,
}

#[repr(C)]
struct property {
    _private: [u8; 0],
}

#[repr(C)]
struct resource {
    start: dma_addr_t,
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct snd_dmaengine_dai_dma_data {
    addr: dma_addr_t,
    addr_width: c_int,
    maxburst: c_uint,
}

#[repr(C)]
struct snd_pcm_hardware {
    info: u32,
    formats: u64,
    rates: u32,
    rate_min: u32,
    rate_max: u32,
    channels_min: u32,
    channels_max: u32,
    buffer_bytes_max: usize,
    period_bytes_min: usize,
    period_bytes_max: usize,
    periods_min: u32,
    periods_max: u32,
}

#[repr(C)]
struct snd_dmaengine_pcm_config {
    pcm_hardware: *const snd_pcm_hardware,
    prepare_slave_config: Option<unsafe extern "C" fn()>,
    chan_names: [*const c_char; 2],
    prealloc_buffer_size: usize,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: u32,
    rate_min: u32,
    rate_max: u32,
    formats: u64,
}

#[repr(C)]
struct snd_soc_dai_ops {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    auto_selectable_formats: *const u64,
    num_auto_selectable_formats: c_uint,
}

#[repr(C)]
struct snd_soc_dai_driver {
    ops: *const snd_soc_dai_ops,
    symmetric_rate: c_uint,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
}

#[repr(C)]
struct snd_soc_component_driver {
    name: *const c_char,
    legacy_dai_naming: c_uint,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
struct driver_inner {
    name: *const c_char,
    of_match_table: *const of_device_id,
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    driver: driver_inner,
}

#[repr(C)]
struct spacemit_i2s_dev {
    dev: *mut device,

    base: *mut c_void,

    reset: *mut reset_control,

    sysclk: *mut clk,
    bclk: *mut clk,
    func_clk: *mut clk,
    sysclk_div: *mut clk,
    c_sysclk: *mut clk,
    c_bclk: *mut clk,

    capture_dma_data: snd_dmaengine_dai_dma_data,
    playback_dma_data: snd_dmaengine_dai_dma_data,

    has_capture: bool,
    has_playback: bool,

    dai_fmt: c_int,

    started_count: c_int,
}

extern "C" {
    static snd_dmaengine_pcm_prepare_slave_config: Option<unsafe extern "C" fn()>;

    fn writel(value: u32, addr: *mut c_void);
    fn readl(addr: *mut c_void) -> u32;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_pcm_hw_constraint_minmax(
        runtime: *mut snd_pcm_runtime,
        var: c_int,
        min: u32,
        max: u32,
    ) -> c_int;
    fn snd_pcm_hw_constraint_mask64(
        runtime: *mut snd_pcm_runtime,
        var: c_int,
        mask: u64,
    ) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_soc_dai_init_dma_data(
        dai: *mut snd_soc_dai,
        playback: *mut snd_dmaengine_dai_dma_data,
        capture: *mut snd_dmaengine_dai_dma_data,
    );
    fn reset_control_deassert(rstc: *mut reset_control) -> c_int;
    fn reset_control_assert(rstc: *mut reset_control) -> c_int;
    fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    fn of_property_for_each_string_next(
        node: *mut device_node,
        name: *const c_char,
        prop: *mut *mut property,
        strp: *mut *const c_char,
    ) -> bool;
    fn devm_kmemdup(dev: *mut device, src: *const c_void, len: usize, gfp: c_uint) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn devm_clk_get_enabled(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_clk_get_optional_enabled(dev: *mut device, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device,
        index: c_uint,
        res: *mut *mut resource,
    ) -> *mut c_void;
    fn devm_reset_control_get_exclusive(dev: *mut device, id: *const c_char) -> *mut reset_control;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn devm_snd_dmaengine_pcm_register(
        dev: *mut device,
        config: *const snd_dmaengine_pcm_config,
        flags: c_uint,
    ) -> c_int;
}

extern "C" {
    static SNDRV_PCM_INFO_INTERLEAVED: u32;
    static SNDRV_PCM_INFO_BATCH: u32;
}

const SNDRV_PCM_RATE_8000: u32 = 1 << 0;
const SNDRV_PCM_RATE_16000: u32 = 1 << 1;
const SNDRV_PCM_RATE_48000: u32 = 1 << 2;
const SNDRV_PCM_RATE_192000: u32 = 1 << 3;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 1;
const SNDRV_PCM_FORMAT_S8: c_int = 0;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 1;
const SNDRV_PCM_FORMAT_S32_LE: c_int = 2;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 0;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 1;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_DSP_A: c_uint = 2;
const SND_SOC_DAIFMT_DSP_B: c_uint = 3;
const SND_SOC_POSSIBLE_DAIFMT_I2S: u64 = 1 << 0;
const SND_SOC_POSSIBLE_DAIFMT_DSP_A: u64 = 1 << 1;
const SND_SOC_POSSIBLE_DAIFMT_DSP_B: u64 = 1 << 2;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 2;
const SNDRV_PCM_TRIGGER_STOP: c_int = 3;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 4;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 5;
const DMA_SLAVE_BUSWIDTH_1_BYTE: c_int = 1;
const DMA_SLAVE_BUSWIDTH_2_BYTES: c_int = 2;
const DMA_SLAVE_BUSWIDTH_4_BYTES: c_int = 4;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;

static spacemit_pcm_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: unsafe { SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BATCH },
    formats: SPACEMIT_PCM_FORMATS,
    rates: SPACEMIT_PCM_RATES,
    rate_min: SNDRV_PCM_RATE_8000,
    rate_max: SNDRV_PCM_RATE_192000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: (SPACEMIT_I2S_PERIOD_SIZE * 4 * 4) as usize,
    period_bytes_min: (SPACEMIT_I2S_PERIOD_SIZE * 2) as usize,
    period_bytes_max: (SPACEMIT_I2S_PERIOD_SIZE * 4) as usize,
    periods_min: 2,
    periods_max: 4,
};

static spacemit_dmaengine_pcm_config: snd_dmaengine_pcm_config = snd_dmaengine_pcm_config {
    pcm_hardware: &spacemit_pcm_hardware,
    prepare_slave_config: unsafe { snd_dmaengine_pcm_prepare_slave_config },
    chan_names: [b"tx\0".as_ptr() as *const c_char, b"rx\0".as_ptr() as *const c_char],
    prealloc_buffer_size: 32 * 1024,
};

unsafe extern "C" fn spacemit_i2s_init(i2s: *mut spacemit_i2s_dev) {
    let sscr_val: u32;
    let sspsp_val: u32;
    let ssfcr_val: u32;
    let ssrwt_val: u32;

    sscr_val = SSCR_TRAIL | SSCR_FRF_PSP;
    ssfcr_val = field_prep(SSFCR_FIELD_TFT, 0xF)
        | field_prep(SSFCR_FIELD_RFT, 0xF)
        | SSFCR_RSRE
        | SSFCR_TSRE;
    ssrwt_val = SSRWT_RWOT;
    sspsp_val = SSPSP_SFRMP;

    writel(sscr_val, ((*i2s).base as *mut u8).add(SSCR) as *mut c_void);
    writel(ssfcr_val, ((*i2s).base as *mut u8).add(SSFCR) as *mut c_void);
    writel(sspsp_val, ((*i2s).base as *mut u8).add(SSPSP) as *mut c_void);
    writel(ssrwt_val, ((*i2s).base as *mut u8).add(SSRWT) as *mut c_void);
    writel(0, ((*i2s).base as *mut u8).add(SSINTEN) as *mut c_void);
}

unsafe extern "C" fn spacemit_i2s_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut spacemit_i2s_dev;

    match ((*i2s).dai_fmt as c_uint) & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            snd_pcm_hw_constraint_minmax(
                (*substream).runtime,
                SNDRV_PCM_HW_PARAM_CHANNELS,
                2,
                2,
            );
            snd_pcm_hw_constraint_mask64(
                (*substream).runtime,
                SNDRV_PCM_HW_PARAM_FORMAT,
                SNDRV_PCM_FMTBIT_S16_LE,
            );
        }
        SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_DSP_B => {
            snd_pcm_hw_constraint_minmax(
                (*substream).runtime,
                SNDRV_PCM_HW_PARAM_CHANNELS,
                1,
                1,
            );
            snd_pcm_hw_constraint_mask64(
                (*substream).runtime,
                SNDRV_PCM_HW_PARAM_FORMAT,
                SNDRV_PCM_FMTBIT_S32_LE,
            );
        }
        _ => {
            dev_dbg((*i2s).dev, b"unexpected format type\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    0
}

unsafe extern "C" fn spacemit_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut spacemit_i2s_dev;
    let mut dma_data: *mut snd_dmaengine_dai_dma_data;
    let data_bits: u32;
    let mut data_width: u32;
    let bclk_rate: c_ulong;
    let mut val: u32;
    let mut ret: c_int;

    dma_data = &mut (*i2s).playback_dma_data;

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        dma_data = &mut (*i2s).capture_dma_data;
    }

    match params_format(params) {
        SNDRV_PCM_FORMAT_S8 => {
            data_bits = 8;
            data_width = SSCR_DW_8BYTE;
            (*dma_data).maxburst = 8;
            (*dma_data).addr_width = DMA_SLAVE_BUSWIDTH_1_BYTE;
        }
        SNDRV_PCM_FORMAT_S16_LE => {
            data_bits = 16;
            data_width = SSCR_DW_16BYTE;
            (*dma_data).maxburst = 16;
            (*dma_data).addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
        }
        SNDRV_PCM_FORMAT_S32_LE => {
            data_bits = 32;
            data_width = SSCR_DW_32BYTE;
            (*dma_data).maxburst = 32;
            (*dma_data).addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
        }
        _ => {
            dev_dbg((*i2s).dev, b"unexpected data width type\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    match ((*i2s).dai_fmt as c_uint) & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            if data_bits == 16 {
                data_width = SSCR_DW_32BYTE;
                (*dma_data).maxburst = 32;
                (*dma_data).addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
            }
        }
        SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_DSP_B => {}
        _ => {
            dev_dbg((*i2s).dev, b"unexpected format type\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    val = readl(((*i2s).base as *mut u8).add(SSCR) as *mut c_void);
    if (val & SSCR_SSE) != 0 {
        return 0;
    }

    val &= !SSCR_DW_32BYTE;
    val |= data_width;
    writel(val, ((*i2s).base as *mut u8).add(SSCR) as *mut c_void);

    bclk_rate = (params_channels(params) as c_ulong)
        .wrapping_mul(params_rate(params) as c_ulong)
        .wrapping_mul(data_bits as c_ulong);

    ret = clk_set_rate((*i2s).c_sysclk, bclk_rate.wrapping_mul(2));
    if ret != 0 {
        return ret;
    }

    ret = clk_set_rate((*i2s).c_bclk, bclk_rate);
    if ret != 0 {
        return ret;
    }

    ret = clk_set_rate((*i2s).bclk, bclk_rate);
    if ret != 0 {
        return ret;
    }

    clk_set_rate((*i2s).func_clk, bclk_rate)
}

unsafe extern "C" fn spacemit_i2s_set_sysclk(
    cpu_dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let i2s = dev_get_drvdata((*cpu_dai).dev) as *mut spacemit_i2s_dev;
    let ret: c_int;

    if freq == 0 {
        return 0;
    }

    if !(*i2s).sysclk_div.is_null() {
        ret = clk_set_rate((*i2s).sysclk_div, freq as c_ulong);
        if ret != 0 {
            return ret;
        }
    }

    clk_set_rate((*i2s).sysclk, freq as c_ulong)
}

unsafe extern "C" fn spacemit_i2s_set_fmt(cpu_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let i2s = dev_get_drvdata((*cpu_dai).dev) as *mut spacemit_i2s_dev;
    let mut sspsp_val: u32;

    sspsp_val = readl(((*i2s).base as *mut u8).add(SSPSP) as *mut c_void);
    sspsp_val &= !SSPSP_FIELD_SFRMWDTH;
    sspsp_val |= SSPSP_FSRT;

    (*i2s).dai_fmt = fmt as c_int;

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            sspsp_val |= field_prep(SSPSP_FIELD_SFRMWDTH, 0x10);
        }
        SND_SOC_DAIFMT_DSP_B => {
            /* DSP_B: next frame asserted after previous frame end, so clear FSRT */
            sspsp_val &= !SSPSP_FSRT;
            sspsp_val |= field_prep(SSPSP_FIELD_SFRMWDTH, 0x1);
        }
        SND_SOC_DAIFMT_DSP_A => {
            sspsp_val |= field_prep(SSPSP_FIELD_SFRMWDTH, 0x1);
        }
        _ => {
            dev_dbg((*i2s).dev, b"unexpected format type\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    writel(sspsp_val, ((*i2s).base as *mut u8).add(SSPSP) as *mut c_void);

    0
}

unsafe extern "C" fn spacemit_i2s_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut spacemit_i2s_dev;
    let mut val: u32;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            if (*i2s).started_count == 0 {
                val = readl(((*i2s).base as *mut u8).add(SSCR) as *mut c_void);
                val |= SSCR_SSE;
                writel(val, ((*i2s).base as *mut u8).add(SSCR) as *mut c_void);
            }
            (*i2s).started_count += 1;
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            if (*i2s).started_count != 0 {
                (*i2s).started_count -= 1;
            }

            if (*i2s).started_count == 0 {
                val = readl(((*i2s).base as *mut u8).add(SSCR) as *mut c_void);
                val &= !SSCR_SSE;
                writel(val, ((*i2s).base as *mut u8).add(SSCR) as *mut c_void);
            }
        }
        _ => {
            return -EINVAL;
        }
    }

    0
}

unsafe extern "C" fn spacemit_i2s_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut spacemit_i2s_dev;

    snd_soc_dai_init_dma_data(
        dai,
        if (*i2s).has_playback {
            &mut (*i2s).playback_dma_data
        } else {
            ptr::null_mut()
        },
        if (*i2s).has_capture {
            &mut (*i2s).capture_dma_data
        } else {
            ptr::null_mut()
        },
    );

    reset_control_deassert((*i2s).reset);

    spacemit_i2s_init(i2s);

    0
}

unsafe extern "C" fn spacemit_i2s_dai_remove(dai: *mut snd_soc_dai) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut spacemit_i2s_dev;

    reset_control_assert((*i2s).reset);

    0
}

static spacemit_i2s_selectable_formats: u64 = SND_SOC_POSSIBLE_DAIFMT_I2S
    | SND_SOC_POSSIBLE_DAIFMT_DSP_A
    | SND_SOC_POSSIBLE_DAIFMT_DSP_B;

static spacemit_i2s_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(spacemit_i2s_dai_probe),
    remove: Some(spacemit_i2s_dai_remove),
    startup: Some(spacemit_i2s_startup),
    hw_params: Some(spacemit_i2s_hw_params),
    set_sysclk: Some(spacemit_i2s_set_sysclk),
    set_fmt: Some(spacemit_i2s_set_fmt),
    trigger: Some(spacemit_i2s_trigger),
    auto_selectable_formats: &spacemit_i2s_selectable_formats,
    num_auto_selectable_formats: 1,
};

static mut spacemit_i2s_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    ops: &spacemit_i2s_dai_ops,
    symmetric_rate: 1,
    playback: snd_soc_pcm_stream {
        stream_name: ptr::null(),
        channels_min: 0,
        channels_max: 0,
        rates: 0,
        rate_min: 0,
        rate_max: 0,
        formats: 0,
    },
    capture: snd_soc_pcm_stream {
        stream_name: ptr::null(),
        channels_min: 0,
        channels_max: 0,
        rates: 0,
        rate_min: 0,
        rate_max: 0,
        formats: 0,
    },
};

unsafe extern "C" fn spacemit_i2s_init_dai(
    i2s: *mut spacemit_i2s_dev,
    dp: *mut *mut snd_soc_dai_driver,
    addr: dma_addr_t,
) -> c_int {
    let node = (*(*i2s).dev).of_node;
    let mut dai: *mut snd_soc_dai_driver;
    let mut dma_names: *mut property = ptr::null_mut();
    let mut dma_name: *const c_char = ptr::null();

    while of_property_for_each_string_next(
        node,
        b"dma-names\0".as_ptr() as *const c_char,
        &mut dma_names,
        &mut dma_name,
    ) {
        if strcmp(dma_name, b"tx\0".as_ptr() as *const c_char) == 0 {
            (*i2s).has_playback = true;
        }
        if strcmp(dma_name, b"rx\0".as_ptr() as *const c_char) == 0 {
            (*i2s).has_capture = true;
        }
    }

    dai = devm_kmemdup(
        (*i2s).dev,
        &spacemit_i2s_dai as *const _ as *const c_void,
        size_of::<snd_soc_dai_driver>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_driver;
    if dai.is_null() {
        return -ENOMEM;
    }

    if (*i2s).has_playback {
        (*dai).playback.stream_name = b"Playback\0".as_ptr() as *const c_char;
        (*dai).playback.channels_min = 1;
        (*dai).playback.channels_max = 2;
        (*dai).playback.rates = SPACEMIT_PCM_RATES;
        (*dai).playback.rate_min = SNDRV_PCM_RATE_8000;
        (*dai).playback.rate_max = SNDRV_PCM_RATE_48000;
        (*dai).playback.formats = SPACEMIT_PCM_FORMATS;

        (*i2s).playback_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
        (*i2s).playback_dma_data.maxburst = 32;
        (*i2s).playback_dma_data.addr = addr;
    }

    if (*i2s).has_capture {
        (*dai).capture.stream_name = b"Capture\0".as_ptr() as *const c_char;
        (*dai).capture.channels_min = 1;
        (*dai).capture.channels_max = 2;
        (*dai).capture.rates = SPACEMIT_PCM_RATES;
        (*dai).capture.rate_min = SNDRV_PCM_RATE_8000;
        (*dai).capture.rate_max = SNDRV_PCM_RATE_48000;
        (*dai).capture.formats = SPACEMIT_PCM_FORMATS;

        (*i2s).capture_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
        (*i2s).capture_dma_data.maxburst = 32;
        (*i2s).capture_dma_data.addr = addr;
    }

    if !dp.is_null() {
        *dp = dai;
    }

    0
}

static spacemit_i2s_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"i2s-k1\0".as_ptr() as *const c_char,
    legacy_dai_naming: 1,
};

unsafe extern "C" fn spacemit_i2s_probe(pdev: *mut platform_device) -> c_int {
    let mut dai: *mut snd_soc_dai_driver = ptr::null_mut();
    let mut i2s: *mut spacemit_i2s_dev;
    let mut res: *mut resource = ptr::null_mut();
    let mut clk: *mut clk;
    let mut ret: c_int;

    i2s = devm_kzalloc(
        &mut (*pdev).dev,
        size_of::<spacemit_i2s_dev>(),
        GFP_KERNEL,
    ) as *mut spacemit_i2s_dev;
    if i2s.is_null() {
        return -ENOMEM;
    }

    (*i2s).dev = &mut (*pdev).dev;

    (*i2s).sysclk = devm_clk_get_enabled((*i2s).dev, b"sysclk\0".as_ptr() as *const c_char);
    if IS_ERR((*i2s).sysclk as *const c_void) {
        return dev_err_probe(
            (*i2s).dev,
            PTR_ERR((*i2s).sysclk as *const c_void),
            b"failed to enable sysbase clock\n\0".as_ptr() as *const c_char,
        );
    }

    (*i2s).bclk = devm_clk_get_enabled((*i2s).dev, b"bclk\0".as_ptr() as *const c_char);
    if IS_ERR((*i2s).bclk as *const c_void) {
        return dev_err_probe(
            (*i2s).dev,
            PTR_ERR((*i2s).bclk as *const c_void),
            b"failed to enable bit clock\n\0".as_ptr() as *const c_char,
        );
    }

    clk = devm_clk_get_enabled((*i2s).dev, b"bus\0".as_ptr() as *const c_char);
    if IS_ERR(clk as *const c_void) {
        return dev_err_probe(
            (*i2s).dev,
            PTR_ERR(clk as *const c_void),
            b"failed to enable bus clock\n\0".as_ptr() as *const c_char,
        );
    }

    (*i2s).func_clk = devm_clk_get_enabled((*i2s).dev, b"func\0".as_ptr() as *const c_char);
    if IS_ERR((*i2s).func_clk as *const c_void) {
        return dev_err_probe(
            (*i2s).dev,
            PTR_ERR((*i2s).func_clk as *const c_void),
            b"failed to enable func clock\n\0".as_ptr() as *const c_char,
        );
    }

    (*i2s).sysclk_div =
        devm_clk_get_optional_enabled((*i2s).dev, b"sysclk_div\0".as_ptr() as *const c_char);
    if IS_ERR((*i2s).sysclk_div as *const c_void) {
        return dev_err_probe(
            (*i2s).dev,
            PTR_ERR((*i2s).sysclk_div as *const c_void),
            b"failed to enable sysclk_div clock\n\0".as_ptr() as *const c_char,
        );
    }

    (*i2s).c_sysclk =
        devm_clk_get_optional_enabled((*i2s).dev, b"c_sysclk\0".as_ptr() as *const c_char);
    if IS_ERR((*i2s).c_sysclk as *const c_void) {
        return dev_err_probe(
            (*i2s).dev,
            PTR_ERR((*i2s).c_sysclk as *const c_void),
            b"failed to enable c_sysclk clock\n\0".as_ptr() as *const c_char,
        );
    }

    (*i2s).c_bclk =
        devm_clk_get_optional_enabled((*i2s).dev, b"c_bclk\0".as_ptr() as *const c_char);
    if IS_ERR((*i2s).c_bclk as *const c_void) {
        return dev_err_probe(
            (*i2s).dev,
            PTR_ERR((*i2s).c_bclk as *const c_void),
            b"failed to enable c_bclk clock\n\0".as_ptr() as *const c_char,
        );
    }

    (*i2s).base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR((*i2s).base as *const c_void) {
        return PTR_ERR((*i2s).base as *const c_void);
    }

    (*i2s).reset = devm_reset_control_get_exclusive(&mut (*pdev).dev, ptr::null());
    if IS_ERR((*i2s).reset as *const c_void) {
        return dev_err_probe(
            (*i2s).dev,
            PTR_ERR((*i2s).reset as *const c_void),
            b"failed to get reset control\0".as_ptr() as *const c_char,
        );
    }

    dev_set_drvdata((*i2s).dev, i2s as *mut c_void);

    ret = spacemit_i2s_init_dai(i2s, &mut dai, (*res).start.wrapping_add(SSDATR as dma_addr_t));
    if ret != 0 {
        return ret;
    }

    ret = devm_snd_soc_register_component((*i2s).dev, &spacemit_i2s_component, dai, 1);
    if ret != 0 {
        return ret;
    }

    devm_snd_dmaengine_pcm_register(&mut (*pdev).dev, &spacemit_dmaengine_pcm_config, 0)
}

static spacemit_i2s_of_match: [of_device_id; 3] = [
    of_device_id {
        compatible: b"spacemit,k1-i2s\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: b"spacemit,k3-i2s\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, spacemit_i2s_of_match); */

static mut spacemit_i2s_driver: platform_driver = platform_driver {
    probe: Some(spacemit_i2s_probe),
    driver: driver_inner {
        name: b"i2s-k1\0".as_ptr() as *const c_char,
        of_match_table: spacemit_i2s_of_match.as_ptr(),
    },
};
/* module_platform_driver(spacemit_i2s_driver); */

/* MODULE_LICENSE("GPL"); */
/* MODULE_DESCRIPTION("I2S bus driver for SpacemiT K1/K3 SoC"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
