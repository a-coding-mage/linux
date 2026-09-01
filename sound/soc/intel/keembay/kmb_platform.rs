// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2020 Intel Corporation.
//
// Intel KeemBay Platform driver.
//

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const PERIODS_MIN: u32 = 2;
const PERIODS_MAX: u32 = 48;
const PERIOD_BYTES_MIN: u32 = 4096;
const BUFFER_BYTES_MAX: u32 = PERIODS_MAX * PERIOD_BYTES_MIN;
const TDM_OPERATION: u32 = 5;
const I2S_OPERATION: u32 = 0;
const DATA_WIDTH_CONFIG_BIT: u32 = 6;
const TDM_CHANNEL_CONFIG_BIT: u32 = 3;

type u32_t = u32;
type u16_t = u16;
type size_t = usize;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_int;

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: u32_t,
    pub rates: u32_t,
    pub rate_min: u32_t,
    pub rate_max: u32_t,
    pub formats: u64,
    pub channels_min: u32_t,
    pub channels_max: u32_t,
    pub buffer_bytes_max: size_t,
    pub period_bytes_min: size_t,
    pub period_bytes_max: size_t,
    pub periods_min: u32_t,
    pub periods_max: u32_t,
    pub fifo_size: size_t,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub dma_area: *mut c_void,
    pub period_size: snd_pcm_uframes_t,
    pub buffer_size: snd_pcm_uframes_t,
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub pcm: *mut c_void,
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr: u64,
    pub addr_width: c_int,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
    pub name: *const c_char,
}

#[repr(C)]
pub struct resource {
    pub start: u64,
}

#[repr(C)]
pub struct i2s_clk_config_data {
    pub data_width: u32_t,
    pub chan_nr: u32_t,
    pub sample_rate: u32_t,
}

#[repr(C)]
pub struct kmb_i2s_info {
    pub config: i2s_clk_config_data,
    pub i2s_base: *mut u8,
    pub pss_base: *mut u8,
    pub dev: *mut device,
    pub use_pio: bool,
    pub clock_provider: bool,
    pub iec958_fmt: bool,
    pub active: c_int,
    pub fifo_th: u32_t,
    pub xfer_resolution: u32_t,
    pub ccr: u32_t,
    pub tx_ptr: c_uint,
    pub rx_ptr: c_uint,
    pub tx_substream: *mut snd_pcm_substream,
    pub rx_substream: *mut snd_pcm_substream,
    pub play_dma_data: snd_dmaengine_dai_dma_data,
    pub capture_dma_data: snd_dmaengine_dai_dma_data,
    pub clk_apb: *mut c_void,
    pub clk_i2s: *mut c_void,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> c_int>,
    pub open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    pub legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub channels_min: u32_t,
    pub channels_max: u32_t,
    pub rates: u32_t,
    pub rate_min: u32_t,
    pub rate_max: u32_t,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

unsafe extern "C" {
    static SNDRV_PCM_INFO_INTERLEAVED: u32_t;
    static SNDRV_PCM_INFO_MMAP: u32_t;
    static SNDRV_PCM_INFO_MMAP_VALID: u32_t;
    static SNDRV_PCM_INFO_BATCH: u32_t;
    static SNDRV_PCM_INFO_BLOCK_TRANSFER: u32_t;
    static SNDRV_PCM_RATE_8000: u32_t;
    static SNDRV_PCM_RATE_16000: u32_t;
    static SNDRV_PCM_RATE_48000: u32_t;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE: u64;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_HW_PARAM_PERIODS: c_int;
    static SNDRV_PCM_FORMAT_S16_LE: c_int;
    static SNDRV_PCM_FORMAT_S24_LE: c_int;
    static SNDRV_PCM_FORMAT_S32_LE: c_int;
    static SNDRV_PCM_FORMAT_IEC958_SUBFRAME_LE: c_int;
    static SNDRV_DMA_TYPE_CONTINUOUS: c_int;
    static DMA_SLAVE_BUSWIDTH_2_BYTES: c_int;
    static DMA_SLAVE_BUSWIDTH_4_BYTES: c_int;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_BC_FC: c_uint;
    static SND_SOC_DAIFMT_BP_FP: c_uint;
    static IRQ_NONE: irqreturn_t;
    static IRQ_HANDLED: irqreturn_t;
    static GFP_KERNEL: c_uint;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static EPROBE_DEFER: c_int;
    static MAX_ISR: u32_t;
    static TX_INT_FLAG: u32_t;
    static RX_INT_FLAG: u32_t;
    static ISR_RXDA: u32_t;
    static ISR_TXFE: u32_t;
    static ISR_TXFO: u32_t;
    static ISR_RXFO: u32_t;
    static ITER: usize;
    static IRER: usize;
    static IER: usize;
    static CER: usize;
    static CCR: usize;
    static I2S_DMACR: usize;
    static I2S_RTXDMA: usize;
    static I2S_RRXDMA: usize;
    static I2S_GEN_CFG_0: usize;
    static I2S_COMP_PARAM_1: usize;
    static I2S_TXDMA: u64;
    static I2S_RXDMA: u64;
    static TXFFR: usize;
    static RXFFR: usize;
    static I2S_DMAEN_TXBLOCK: u32_t;
    static I2S_DMAEN_RXBLOCK: u32_t;
    static CLOCK_PROVIDER_MODE: u32_t;

    fn LRBR_LTHR(i: u32_t) -> usize;
    fn RRBR_RTHR(i: u32_t) -> usize;
    fn TER(i: u32_t) -> usize;
    fn RER(i: u32_t) -> usize;
    fn TOR(i: u32_t) -> usize;
    fn ROR(i: u32_t) -> usize;
    fn IMR(i: u32_t) -> usize;
    fn ISR(i: u32_t) -> usize;
    fn TCR(i: u32_t) -> usize;
    fn TFCR(i: u32_t) -> usize;
    fn RCR(i: u32_t) -> usize;
    fn RFCR(i: u32_t) -> usize;
    fn COMP1_FIFO_DEPTH(v: u32_t) -> u32_t;

    fn bitrev32(v: u32_t) -> u32_t;
    fn readl(addr: *mut u8) -> u32_t;
    fn writel(v: u32_t, addr: *mut u8);
    fn snd_pcm_running(substream: *mut snd_pcm_substream) -> bool;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_set_runtime_hwparams(substream: *mut snd_pcm_substream, hw: *const snd_pcm_hardware);
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, param: c_int) -> c_int;
    fn snd_pcm_set_managed_buffer_all(pcm: *mut c_void, typ: c_int, data: *mut c_void, min: size_t, max: size_t);
    fn snd_soc_dai_init_dma_data(dai: *mut snd_soc_dai, playback: *mut snd_dmaengine_dai_dma_data, capture: *mut snd_dmaengine_dai_dma_data);
    fn snd_soc_dai_set_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream, data: *mut snd_dmaengine_dai_dma_data);
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> u32_t;
    fn params_rate(params: *mut snd_pcm_hw_params) -> u32_t;
    fn clk_prepare_enable(clk: *mut c_void) -> c_int;
    fn clk_disable_unprepare(clk: *mut c_void);
    fn clk_set_rate(clk: *mut c_void, rate: u32_t) -> c_int;
    fn devm_add_action_or_reset(dev: *mut device, action: unsafe extern "C" fn(*mut c_void), data: *mut c_void) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut c_void;
    fn devm_platform_get_and_ioremap_resource(pdev: *mut platform_device, index: c_uint, res: *mut *mut resource) -> *mut u8;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut u8;
    fn device_get_match_data(dev: *mut device) -> *const c_void;
    fn of_property_present(np: *mut device_node, propname: *const c_char) -> bool;
    fn platform_get_irq_optional(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn snd_dmaengine_pcm_register(dev: *mut device, config: *mut c_void, flags: c_uint) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

static kmb_pcm_hardware: snd_pcm_hardware = unsafe {
    snd_pcm_hardware {
        info: SNDRV_PCM_INFO_INTERLEAVED
            | SNDRV_PCM_INFO_MMAP
            | SNDRV_PCM_INFO_MMAP_VALID
            | SNDRV_PCM_INFO_BATCH
            | SNDRV_PCM_INFO_BLOCK_TRANSFER,
        rates: SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_48000,
        rate_min: 8000,
        rate_max: 48000,
        formats: SNDRV_PCM_FMTBIT_S16_LE
            | SNDRV_PCM_FMTBIT_S24_LE
            | SNDRV_PCM_FMTBIT_S32_LE
            | SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE,
        channels_min: 2,
        channels_max: 2,
        buffer_bytes_max: BUFFER_BYTES_MAX as size_t,
        period_bytes_min: PERIOD_BYTES_MIN as size_t,
        period_bytes_max: (BUFFER_BYTES_MAX / PERIODS_MIN) as size_t,
        periods_min: PERIODS_MIN,
        periods_max: PERIODS_MAX,
        fifo_size: 16,
    }
};

/*
 * Convert to ADV7511 HDMI hardware format.
 * ADV7511 HDMI chip need parity bit replaced by block start bit and
 * with the preamble bits left out.
 * ALSA IEC958 subframe format:
 * bit 0-3  = preamble (0x8 = block start)
 *     4-7  = AUX (=0)
 *     8-27 = audio data (without AUX if 24bit sample)
 *     28   = validity
 *     29   = user data
 *     30   = channel status
 *     31   = parity
 *
 * ADV7511 IEC958 subframe format:
 * bit 0-23  = audio data
 *     24    = validity
 *     25    = user data
 *     26    = channel status
 *     27    = block start
 *     28-31 = 0
 * MSB to LSB bit reverse by software as hardware not supporting it.
 */
unsafe extern "C" fn hdmi_reformat_iec958(
    runtime: *mut snd_pcm_runtime,
    kmb_i2s: *mut kmb_i2s_info,
    mut tx_ptr: c_uint,
) {
    let buf = (*runtime).dma_area as *mut [u32_t; 2];
    let mut i: u32_t = 0;

    while i < (*kmb_i2s).fifo_th {
        let mut j: u32_t = 0;
        loop {
            let mut temp: c_ulong = (*buf.add(tx_ptr as usize))[j as usize] as c_ulong;
            /* Replace parity with block start*/
            if (temp & (1usize << 3) as c_ulong) != 0 {
                temp |= (1usize << 31) as c_ulong;
            } else {
                temp &= !((1usize << 31) as c_ulong);
            }
            let sample: u32_t = bitrev32(temp as u32_t);
            (*buf.add(tx_ptr as usize))[j as usize] = sample << 4;
            j += 1;
            if j >= 2 {
                break;
            }
        }
        tx_ptr += 1;
        i += 1;
    }
}

unsafe extern "C" fn kmb_pcm_tx_fn(
    kmb_i2s: *mut kmb_i2s_info,
    runtime: *mut snd_pcm_runtime,
    mut tx_ptr: c_uint,
    period_elapsed: *mut bool,
) -> c_uint {
    let mut period_pos: c_uint = tx_ptr % (*runtime).period_size as c_uint;
    let i2s_base = (*kmb_i2s).i2s_base;
    let buf = (*runtime).dma_area;
    let mut i: c_int = 0;

    if (*kmb_i2s).iec958_fmt {
        hdmi_reformat_iec958(runtime, kmb_i2s, tx_ptr);
    }

    /* KMB i2s uses two separate L/R FIFO */
    while i < (*kmb_i2s).fifo_th as c_int {
        if (*kmb_i2s).config.data_width == 16 {
            writel((*(buf as *mut [u16_t; 2]).add(tx_ptr as usize))[0] as u32_t, i2s_base.add(LRBR_LTHR(0)));
            writel((*(buf as *mut [u16_t; 2]).add(tx_ptr as usize))[1] as u32_t, i2s_base.add(RRBR_RTHR(0)));
        } else {
            writel((*(buf as *mut [u32_t; 2]).add(tx_ptr as usize))[0], i2s_base.add(LRBR_LTHR(0)));
            writel((*(buf as *mut [u32_t; 2]).add(tx_ptr as usize))[1], i2s_base.add(RRBR_RTHR(0)));
        }

        period_pos += 1;

        tx_ptr += 1;
        if tx_ptr >= (*runtime).buffer_size as c_uint {
            tx_ptr = 0;
        }
        i += 1;
    }

    *period_elapsed = period_pos >= (*runtime).period_size as c_uint;

    tx_ptr
}

unsafe extern "C" fn kmb_pcm_rx_fn(
    kmb_i2s: *mut kmb_i2s_info,
    runtime: *mut snd_pcm_runtime,
    mut rx_ptr: c_uint,
    period_elapsed: *mut bool,
) -> c_uint {
    let mut period_pos: c_uint = rx_ptr % (*runtime).period_size as c_uint;
    let i2s_base = (*kmb_i2s).i2s_base;
    let chan: c_int = (*kmb_i2s).config.chan_nr as c_int;
    let buf = (*runtime).dma_area;
    let mut i: c_int = 0;

    /* KMB i2s uses two separate L/R FIFO */
    while i < (*kmb_i2s).fifo_th as c_int {
        let mut j: c_int = 0;
        while j < chan / 2 {
            if (*kmb_i2s).config.data_width == 16 {
                *(buf as *mut u16_t).add((rx_ptr as c_int * chan + (j * 2)) as usize) =
                    readl(i2s_base.add(LRBR_LTHR(j as u32_t))) as u16_t;
                *(buf as *mut u16_t).add((rx_ptr as c_int * chan + ((j * 2) + 1)) as usize) =
                    readl(i2s_base.add(RRBR_RTHR(j as u32_t))) as u16_t;
            } else {
                *(buf as *mut u32_t).add((rx_ptr as c_int * chan + (j * 2)) as usize) =
                    readl(i2s_base.add(LRBR_LTHR(j as u32_t)));
                *(buf as *mut u32_t).add((rx_ptr as c_int * chan + ((j * 2) + 1)) as usize) =
                    readl(i2s_base.add(RRBR_RTHR(j as u32_t)));
            }
            j += 1;
        }
        period_pos += 1;

        rx_ptr += 1;
        if rx_ptr >= (*runtime).buffer_size as c_uint {
            rx_ptr = 0;
        }
        i += 1;
    }

    *period_elapsed = period_pos >= (*runtime).period_size as c_uint;

    rx_ptr
}

unsafe extern "C" fn kmb_i2s_disable_channels(kmb_i2s: *mut kmb_i2s_info, stream: u32_t) {
    let mut i: u32_t = 0;

    /* Disable all channels regardless of configuration*/
    if stream as c_int == SNDRV_PCM_STREAM_PLAYBACK {
        while i < MAX_ISR {
            writel(0, (*kmb_i2s).i2s_base.add(TER(i)));
            i += 1;
        }
    } else {
        while i < MAX_ISR {
            writel(0, (*kmb_i2s).i2s_base.add(RER(i)));
            i += 1;
        }
    }
}

unsafe extern "C" fn kmb_i2s_clear_irqs(kmb_i2s: *mut kmb_i2s_info, stream: u32_t) {
    let config = &mut (*kmb_i2s).config as *mut i2s_clk_config_data;
    let mut i: u32_t = 0;

    if stream as c_int == SNDRV_PCM_STREAM_PLAYBACK {
        while i < (*config).chan_nr / 2 {
            readl((*kmb_i2s).i2s_base.add(TOR(i)));
            i += 1;
        }
    } else {
        while i < (*config).chan_nr / 2 {
            readl((*kmb_i2s).i2s_base.add(ROR(i)));
            i += 1;
        }
    }
}

unsafe extern "C" fn kmb_i2s_irq_trigger(
    kmb_i2s: *mut kmb_i2s_info,
    stream: u32_t,
    chan_nr: c_int,
    trigger: bool,
) {
    let mut i: u32_t = 0;
    let flag: u32_t;

    if stream as c_int == SNDRV_PCM_STREAM_PLAYBACK {
        flag = TX_INT_FLAG;
    } else {
        flag = RX_INT_FLAG;
    }

    while i < (chan_nr / 2) as u32_t {
        let mut irq = readl((*kmb_i2s).i2s_base.add(IMR(i)));

        if trigger {
            irq &= !flag;
        } else {
            irq |= flag;
        }

        writel(irq, (*kmb_i2s).i2s_base.add(IMR(i)));
        i += 1;
    }
}

unsafe extern "C" fn kmb_pcm_operation(kmb_i2s: *mut kmb_i2s_info, playback: bool) {
    let substream: *mut snd_pcm_substream = if playback {
        (*kmb_i2s).tx_substream
    } else {
        (*kmb_i2s).rx_substream
    };
    let mut period_elapsed = false;

    if substream.is_null() || !snd_pcm_running(substream) {
        return;
    }

    if playback {
        let ptr = (*kmb_i2s).tx_ptr;
        let new_ptr = kmb_pcm_tx_fn(kmb_i2s, (*substream).runtime, ptr, &mut period_elapsed);
        if (*kmb_i2s).tx_ptr == ptr {
            (*kmb_i2s).tx_ptr = new_ptr;
        }
    } else {
        let ptr = (*kmb_i2s).rx_ptr;
        let new_ptr = kmb_pcm_rx_fn(kmb_i2s, (*substream).runtime, ptr, &mut period_elapsed);
        if (*kmb_i2s).rx_ptr == ptr {
            (*kmb_i2s).rx_ptr = new_ptr;
        }
    }

    if period_elapsed {
        snd_pcm_period_elapsed(substream);
    }
}

unsafe extern "C" fn kmb_pcm_open(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let runtime = (*substream).runtime;
    let rtd = snd_soc_substream_to_rtd(substream);
    let kmb_i2s = snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut kmb_i2s_info;

    snd_soc_set_runtime_hwparams(substream, &kmb_pcm_hardware);
    snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    (*runtime).private_data = kmb_i2s as *mut c_void;

    0
}

unsafe extern "C" fn kmb_pcm_trigger(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let runtime = (*substream).runtime;
    let kmb_i2s = (*runtime).private_data as *mut kmb_i2s_info;

    if cmd == SNDRV_PCM_TRIGGER_START {
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            (*kmb_i2s).tx_ptr = 0;
            (*kmb_i2s).tx_substream = substream;
        } else {
            (*kmb_i2s).rx_ptr = 0;
            (*kmb_i2s).rx_substream = substream;
        }
    } else if cmd == SNDRV_PCM_TRIGGER_STOP {
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            (*kmb_i2s).tx_substream = core::ptr::null_mut();
        } else {
            (*kmb_i2s).rx_substream = core::ptr::null_mut();
        }
        (*kmb_i2s).iec958_fmt = false;
    } else {
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn kmb_i2s_irq_handler(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let kmb_i2s = dev_id as *mut kmb_i2s_info;
    let config = &mut (*kmb_i2s).config as *mut i2s_clk_config_data;
    let mut ret = IRQ_NONE;
    let tx_enabled: u32_t;
    let mut isr = [0u32_t; 4];
    let mut i: c_int = 0;

    while i < ((*config).chan_nr / 2) as c_int {
        isr[i as usize] = readl((*kmb_i2s).i2s_base.add(ISR(i as u32_t)));
        i += 1;
    }

    kmb_i2s_clear_irqs(kmb_i2s, SNDRV_PCM_STREAM_PLAYBACK as u32_t);
    kmb_i2s_clear_irqs(kmb_i2s, SNDRV_PCM_STREAM_CAPTURE as u32_t);
    /* Only check TX interrupt if TX is active */
    tx_enabled = readl((*kmb_i2s).i2s_base.add(ITER));

    /*
     * Data available. Retrieve samples from FIFO
     */

    /*
     * 8 channel audio will have isr[0..2] triggered,
     * reading the specific isr based on the audio configuration,
     * to avoid reading the buffers too early.
     */
    match (*config).chan_nr {
        2 => {
            if (isr[0] & ISR_RXDA) != 0 {
                kmb_pcm_operation(kmb_i2s, false);
            }
            ret = IRQ_HANDLED;
        }
        4 => {
            if (isr[1] & ISR_RXDA) != 0 {
                kmb_pcm_operation(kmb_i2s, false);
            }
            ret = IRQ_HANDLED;
        }
        8 => {
            if (isr[3] & ISR_RXDA) != 0 {
                kmb_pcm_operation(kmb_i2s, false);
            }
            ret = IRQ_HANDLED;
        }
        _ => {}
    }

    i = 0;
    while i < ((*config).chan_nr / 2) as c_int {
        /*
         * Check if TX fifo is empty. If empty fill FIFO with samples
         */
        if (isr[i as usize] & ISR_TXFE) != 0 && tx_enabled != 0 {
            kmb_pcm_operation(kmb_i2s, true);
            ret = IRQ_HANDLED;
        }

        /* Error Handling: TX */
        if (isr[i as usize] & ISR_TXFO) != 0 {
            dev_dbg((*kmb_i2s).dev, c"TX overrun (ch_id=%d)\n".as_ptr(), i);
            ret = IRQ_HANDLED;
        }
        /* Error Handling: RX */
        if (isr[i as usize] & ISR_RXFO) != 0 {
            dev_dbg((*kmb_i2s).dev, c"RX overrun (ch_id=%d)\n".as_ptr(), i);
            ret = IRQ_HANDLED;
        }
        i += 1;
    }

    ret
}

unsafe extern "C" fn kmb_platform_pcm_new(
    _component: *mut snd_soc_component,
    soc_runtime: *mut snd_soc_pcm_runtime,
) -> c_int {
    let size: size_t = kmb_pcm_hardware.buffer_bytes_max;
    /* Use SNDRV_DMA_TYPE_CONTINUOUS as KMB doesn't use PCI sg buffer */
    snd_pcm_set_managed_buffer_all((*soc_runtime).pcm, SNDRV_DMA_TYPE_CONTINUOUS, core::ptr::null_mut(), size, size);
    0
}

unsafe extern "C" fn kmb_pcm_pointer(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let runtime = (*substream).runtime;
    let kmb_i2s = (*runtime).private_data as *mut kmb_i2s_info;
    let pos: snd_pcm_uframes_t = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        (*kmb_i2s).tx_ptr as snd_pcm_uframes_t
    } else {
        (*kmb_i2s).rx_ptr as snd_pcm_uframes_t
    };

    if pos < (*runtime).buffer_size {
        pos
    } else {
        0
    }
}

static kmb_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"kmb".as_ptr(),
    pcm_new: Some(kmb_platform_pcm_new),
    open: Some(kmb_pcm_open),
    trigger: Some(kmb_pcm_trigger),
    pointer: Some(kmb_pcm_pointer),
    legacy_dai_naming: 1,
};

static kmb_component_dma: snd_soc_component_driver = snd_soc_component_driver {
    name: c"kmb".as_ptr(),
    pcm_new: None,
    open: None,
    trigger: None,
    pointer: None,
    legacy_dai_naming: 1,
};

unsafe extern "C" fn kmb_probe(cpu_dai: *mut snd_soc_dai) -> c_int {
    let kmb_i2s = snd_soc_dai_get_drvdata(cpu_dai) as *mut kmb_i2s_info;

    if (*kmb_i2s).use_pio {
        return 0;
    }

    snd_soc_dai_init_dma_data(cpu_dai, &mut (*kmb_i2s).play_dma_data, &mut (*kmb_i2s).capture_dma_data);

    0
}

unsafe extern "C" fn kmb_i2s_enable_dma(kmb_i2s: *mut kmb_i2s_info, stream: u32_t) {
    let mut dma_reg = readl((*kmb_i2s).i2s_base.add(I2S_DMACR));
    /* Enable DMA handshake for stream */
    if stream as c_int == SNDRV_PCM_STREAM_PLAYBACK {
        dma_reg |= I2S_DMAEN_TXBLOCK;
    } else {
        dma_reg |= I2S_DMAEN_RXBLOCK;
    }

    writel(dma_reg, (*kmb_i2s).i2s_base.add(I2S_DMACR));
}

unsafe extern "C" fn kmb_i2s_disable_dma(kmb_i2s: *mut kmb_i2s_info, stream: u32_t) {
    let mut dma_reg = readl((*kmb_i2s).i2s_base.add(I2S_DMACR));
    /* Disable DMA handshake for stream */
    if stream as c_int == SNDRV_PCM_STREAM_PLAYBACK {
        dma_reg &= !I2S_DMAEN_TXBLOCK;
        writel(1, (*kmb_i2s).i2s_base.add(I2S_RTXDMA));
    } else {
        dma_reg &= !I2S_DMAEN_RXBLOCK;
        writel(1, (*kmb_i2s).i2s_base.add(I2S_RRXDMA));
    }
    writel(dma_reg, (*kmb_i2s).i2s_base.add(I2S_DMACR));
}

unsafe extern "C" fn kmb_i2s_start(kmb_i2s: *mut kmb_i2s_info, substream: *mut snd_pcm_substream) {
    let config = &mut (*kmb_i2s).config as *mut i2s_clk_config_data;

    /* I2S Programming sequence in Keem_Bay_VPU_DB_v1.1 */
    writel(1, (*kmb_i2s).i2s_base.add(IER));

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        writel(1, (*kmb_i2s).i2s_base.add(ITER));
    } else {
        writel(1, (*kmb_i2s).i2s_base.add(IRER));
    }

    if (*kmb_i2s).use_pio {
        kmb_i2s_irq_trigger(kmb_i2s, (*substream).stream as u32_t, (*config).chan_nr as c_int, true);
    } else {
        kmb_i2s_enable_dma(kmb_i2s, (*substream).stream as u32_t);
    }

    if (*kmb_i2s).clock_provider {
        writel(1, (*kmb_i2s).i2s_base.add(CER));
    } else {
        writel(0, (*kmb_i2s).i2s_base.add(CER));
    }
}

unsafe extern "C" fn kmb_i2s_stop(kmb_i2s: *mut kmb_i2s_info, substream: *mut snd_pcm_substream) {
    /* I2S Programming sequence in Keem_Bay_VPU_DB_v1.1 */
    kmb_i2s_clear_irqs(kmb_i2s, (*substream).stream as u32_t);

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        writel(0, (*kmb_i2s).i2s_base.add(ITER));
    } else {
        writel(0, (*kmb_i2s).i2s_base.add(IRER));
    }

    kmb_i2s_irq_trigger(kmb_i2s, (*substream).stream as u32_t, 8, false);

    if (*kmb_i2s).active == 0 {
        writel(0, (*kmb_i2s).i2s_base.add(CER));
        writel(0, (*kmb_i2s).i2s_base.add(IER));
    }
}

unsafe extern "C" fn kmb_disable_clk(clk: *mut c_void) {
    clk_disable_unprepare(clk);
}

unsafe extern "C" fn kmb_set_dai_fmt(cpu_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let kmb_i2s = snd_soc_dai_get_drvdata(cpu_dai) as *mut kmb_i2s_info;
    let ret: c_int;

    if (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) == SND_SOC_DAIFMT_BC_FC {
        (*kmb_i2s).clock_provider = false;
        ret = 0;
    } else if (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) == SND_SOC_DAIFMT_BP_FP {
        writel(CLOCK_PROVIDER_MODE, (*kmb_i2s).pss_base.add(I2S_GEN_CFG_0));

        let r = clk_prepare_enable((*kmb_i2s).clk_i2s);
        if r < 0 {
            return r;
        }

        let r = devm_add_action_or_reset((*kmb_i2s).dev, kmb_disable_clk, (*kmb_i2s).clk_i2s);
        if r != 0 {
            return r;
        }

        (*kmb_i2s).clock_provider = true;
        ret = r;
    } else {
        return -EINVAL;
    }

    ret
}

unsafe extern "C" fn kmb_dai_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let kmb_i2s = snd_soc_dai_get_drvdata(cpu_dai) as *mut kmb_i2s_info;

    if cmd == SNDRV_PCM_TRIGGER_START {
        /* Keep track of i2s activity before turn off
         * the i2s interface
         */
        (*kmb_i2s).active += 1;
        kmb_i2s_start(kmb_i2s, substream);
    } else if cmd == SNDRV_PCM_TRIGGER_STOP {
        (*kmb_i2s).active -= 1;
        if (*kmb_i2s).use_pio {
            kmb_i2s_stop(kmb_i2s, substream);
        }
    } else {
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn kmb_i2s_config(kmb_i2s: *mut kmb_i2s_info, stream: c_int) {
    let config = &mut (*kmb_i2s).config as *mut i2s_clk_config_data;
    let mut ch_reg: u32_t;

    kmb_i2s_disable_channels(kmb_i2s, stream as u32_t);

    ch_reg = 0;
    while ch_reg < (*config).chan_nr / 2 {
        if stream == SNDRV_PCM_STREAM_PLAYBACK {
            writel((*kmb_i2s).xfer_resolution, (*kmb_i2s).i2s_base.add(TCR(ch_reg)));
            writel((*kmb_i2s).fifo_th - 1, (*kmb_i2s).i2s_base.add(TFCR(ch_reg)));
            writel(1, (*kmb_i2s).i2s_base.add(TER(ch_reg)));
        } else {
            writel((*kmb_i2s).xfer_resolution, (*kmb_i2s).i2s_base.add(RCR(ch_reg)));
            writel((*kmb_i2s).fifo_th - 1, (*kmb_i2s).i2s_base.add(RFCR(ch_reg)));
            writel(1, (*kmb_i2s).i2s_base.add(RER(ch_reg)));
        }
        ch_reg += 1;
    }
}

unsafe extern "C" fn kmb_dai_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
    cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let kmb_i2s = snd_soc_dai_get_drvdata(cpu_dai) as *mut kmb_i2s_info;
    let config = &mut (*kmb_i2s).config as *mut i2s_clk_config_data;
    let write_val: u32_t;
    let ret: c_int;

    if params_format(hw_params) == SNDRV_PCM_FORMAT_S16_LE {
        (*config).data_width = 16;
        (*kmb_i2s).ccr = 0x00;
        (*kmb_i2s).xfer_resolution = 0x02;
        (*kmb_i2s).play_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
        (*kmb_i2s).capture_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
    } else if params_format(hw_params) == SNDRV_PCM_FORMAT_S24_LE {
        (*config).data_width = 32;
        (*kmb_i2s).ccr = 0x14;
        (*kmb_i2s).xfer_resolution = 0x05;
        (*kmb_i2s).play_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
        (*kmb_i2s).capture_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    } else if params_format(hw_params) == SNDRV_PCM_FORMAT_IEC958_SUBFRAME_LE {
        (*kmb_i2s).iec958_fmt = true;
        (*config).data_width = 32;
        (*kmb_i2s).ccr = 0x10;
        (*kmb_i2s).xfer_resolution = 0x05;
        (*kmb_i2s).play_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
        (*kmb_i2s).capture_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    } else if params_format(hw_params) == SNDRV_PCM_FORMAT_S32_LE {
        (*config).data_width = 32;
        (*kmb_i2s).ccr = 0x10;
        (*kmb_i2s).xfer_resolution = 0x05;
        (*kmb_i2s).play_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
        (*kmb_i2s).capture_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    } else {
        dev_err((*kmb_i2s).dev, c"kmb: unsupported PCM fmt".as_ptr());
        return -EINVAL;
    }

    (*config).chan_nr = params_channels(hw_params);

    match (*config).chan_nr {
        8 | 4 => {
            /*
             * Platform is not capable of providing clocks for
             * multi channel audio
             */
            if (*kmb_i2s).clock_provider {
                return -EINVAL;
            }

            write_val = (((*config).chan_nr / 2) << TDM_CHANNEL_CONFIG_BIT)
                | ((*config).data_width << DATA_WIDTH_CONFIG_BIT)
                | TDM_OPERATION;

            writel(write_val, (*kmb_i2s).pss_base.add(I2S_GEN_CFG_0));
        }
        2 => {
            /*
             * Platform is only capable of providing clocks need for
             * 2 channel master mode
             */
            if !(*kmb_i2s).clock_provider {
                return -EINVAL;
            }

            write_val = (((*config).chan_nr / 2) << TDM_CHANNEL_CONFIG_BIT)
                | ((*config).data_width << DATA_WIDTH_CONFIG_BIT)
                | CLOCK_PROVIDER_MODE
                | I2S_OPERATION;

            writel(write_val, (*kmb_i2s).pss_base.add(I2S_GEN_CFG_0));
        }
        _ => {
            dev_dbg((*kmb_i2s).dev, c"channel not supported\n".as_ptr());
            return -EINVAL;
        }
    }

    kmb_i2s_config(kmb_i2s, (*substream).stream);

    writel((*kmb_i2s).ccr, (*kmb_i2s).i2s_base.add(CCR));

    (*config).sample_rate = params_rate(hw_params);

    if (*kmb_i2s).clock_provider {
        /* Only 2 ch supported in Master mode */
        let bitclk: u32_t = (*config).sample_rate * (*config).data_width * 2;

        ret = clk_set_rate((*kmb_i2s).clk_i2s, bitclk);
        if ret != 0 {
            dev_err((*kmb_i2s).dev, c"Can't set I2S clock rate: %d\n".as_ptr(), ret);
            return ret;
        }
    }

    0
}

unsafe extern "C" fn kmb_dai_prepare(
    substream: *mut snd_pcm_substream,
    cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let kmb_i2s = snd_soc_dai_get_drvdata(cpu_dai) as *mut kmb_i2s_info;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        writel(1, (*kmb_i2s).i2s_base.add(TXFFR));
    } else {
        writel(1, (*kmb_i2s).i2s_base.add(RXFFR));
    }

    0
}

unsafe extern "C" fn kmb_dai_startup(
    substream: *mut snd_pcm_substream,
    cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let kmb_i2s = snd_soc_dai_get_drvdata(cpu_dai) as *mut kmb_i2s_info;
    let dma_data: *mut snd_dmaengine_dai_dma_data;

    if (*kmb_i2s).use_pio {
        return 0;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        dma_data = &mut (*kmb_i2s).play_dma_data;
    } else {
        dma_data = &mut (*kmb_i2s).capture_dma_data;
    }

    snd_soc_dai_set_dma_data(cpu_dai, substream, dma_data);

    0
}

unsafe extern "C" fn kmb_dai_hw_free(
    substream: *mut snd_pcm_substream,
    cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let kmb_i2s = snd_soc_dai_get_drvdata(cpu_dai) as *mut kmb_i2s_info;
    /* I2S Programming sequence in Keem_Bay_VPU_DB_v1.1 */
    if (*kmb_i2s).use_pio {
        kmb_i2s_clear_irqs(kmb_i2s, (*substream).stream as u32_t);
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        writel(0, (*kmb_i2s).i2s_base.add(ITER));
    } else {
        writel(0, (*kmb_i2s).i2s_base.add(IRER));
    }

    if (*kmb_i2s).use_pio {
        kmb_i2s_irq_trigger(kmb_i2s, (*substream).stream as u32_t, 8, false);
    } else {
        kmb_i2s_disable_dma(kmb_i2s, (*substream).stream as u32_t);
    }

    if (*kmb_i2s).active == 0 {
        writel(0, (*kmb_i2s).i2s_base.add(CER));
        writel(0, (*kmb_i2s).i2s_base.add(IER));
    }

    0
}

static kmb_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(kmb_probe),
    startup: Some(kmb_dai_startup),
    trigger: Some(kmb_dai_trigger),
    hw_params: Some(kmb_dai_hw_params),
    hw_free: Some(kmb_dai_hw_free),
    prepare: Some(kmb_dai_prepare),
    set_fmt: Some(kmb_set_dai_fmt),
};

static mut intel_kmb_hdmi_dai: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c"intel_kmb_hdmi_i2s".as_ptr(),
    playback: unsafe {
        snd_soc_pcm_stream {
            channels_min: 2,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_48000,
            rate_min: 48000,
            rate_max: 48000,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE,
        }
    },
    capture: snd_soc_pcm_stream { channels_min: 0, channels_max: 0, rates: 0, rate_min: 0, rate_max: 0, formats: 0 },
    ops: &kmb_dai_ops,
}];

static mut intel_kmb_i2s_dai: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c"intel_kmb_i2s".as_ptr(),
    playback: unsafe {
        snd_soc_pcm_stream {
            channels_min: 2,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_48000,
            rate_min: 8000,
            rate_max: 48000,
            formats: SNDRV_PCM_FMTBIT_S32_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S16_LE,
        }
    },
    capture: unsafe {
        snd_soc_pcm_stream {
            channels_min: 2,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_48000,
            rate_min: 8000,
            rate_max: 48000,
            formats: SNDRV_PCM_FMTBIT_S32_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S16_LE,
        }
    },
    ops: &kmb_dai_ops,
}];

static mut intel_kmb_tdm_dai: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c"intel_kmb_tdm".as_ptr(),
    playback: snd_soc_pcm_stream { channels_min: 0, channels_max: 0, rates: 0, rate_min: 0, rate_max: 0, formats: 0 },
    capture: unsafe {
        snd_soc_pcm_stream {
            channels_min: 4,
            channels_max: 8,
            rates: SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_48000,
            rate_min: 8000,
            rate_max: 48000,
            formats: SNDRV_PCM_FMTBIT_S32_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S16_LE,
        }
    },
    ops: &kmb_dai_ops,
}];

static kmb_plat_of_match: [of_device_id; 4] = [
    of_device_id {
        compatible: c"intel,keembay-i2s".as_ptr(),
        data: unsafe { intel_kmb_i2s_dai.as_ptr() as *const c_void },
    },
    of_device_id {
        compatible: c"intel,keembay-hdmi-i2s".as_ptr(),
        data: unsafe { intel_kmb_hdmi_dai.as_ptr() as *const c_void },
    },
    of_device_id {
        compatible: c"intel,keembay-tdm".as_ptr(),
        data: unsafe { intel_kmb_tdm_dai.as_ptr() as *const c_void },
    },
    of_device_id {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, kmb_plat_of_match); */

unsafe extern "C" fn kmb_plat_dai_probe(pdev: *mut platform_device) -> c_int {
    let np = (*pdev).dev.of_node;
    let kmb_i2s_dai: *mut snd_soc_dai_driver;
    let dev = &mut (*pdev).dev as *mut device;
    let kmb_i2s: *mut kmb_i2s_info;
    let mut res: *mut resource = core::ptr::null_mut();
    let mut ret: c_int;
    let irq: c_int;
    let comp1_reg: u32_t;

    kmb_i2s = devm_kzalloc(dev, core::mem::size_of::<kmb_i2s_info>(), GFP_KERNEL) as *mut kmb_i2s_info;
    if kmb_i2s.is_null() {
        return -ENOMEM;
    }

    kmb_i2s_dai = device_get_match_data(&mut (*pdev).dev) as *mut snd_soc_dai_driver;

    /* Prepare the related clocks */
    (*kmb_i2s).clk_apb = devm_clk_get(dev, c"apb_clk".as_ptr());
    if IS_ERR((*kmb_i2s).clk_apb) {
        dev_err(dev, c"Failed to get apb clock\n".as_ptr());
        return PTR_ERR((*kmb_i2s).clk_apb);
    }

    ret = clk_prepare_enable((*kmb_i2s).clk_apb);
    if ret < 0 {
        return ret;
    }

    ret = devm_add_action_or_reset(dev, kmb_disable_clk, (*kmb_i2s).clk_apb);
    if ret != 0 {
        dev_err(dev, c"Failed to add clk_apb reset action\n".as_ptr());
        return ret;
    }

    (*kmb_i2s).clk_i2s = devm_clk_get(dev, c"osc".as_ptr());
    if IS_ERR((*kmb_i2s).clk_i2s) {
        dev_err(dev, c"Failed to get osc clock\n".as_ptr());
        return PTR_ERR((*kmb_i2s).clk_i2s);
    }

    (*kmb_i2s).i2s_base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR((*kmb_i2s).i2s_base as *const c_void) {
        return PTR_ERR((*kmb_i2s).i2s_base as *const c_void);
    }

    (*kmb_i2s).pss_base = devm_platform_ioremap_resource(pdev, 1);
    if IS_ERR((*kmb_i2s).pss_base as *const c_void) {
        return PTR_ERR((*kmb_i2s).pss_base as *const c_void);
    }

    (*kmb_i2s).dev = &mut (*pdev).dev;

    comp1_reg = readl((*kmb_i2s).i2s_base.add(I2S_COMP_PARAM_1));

    (*kmb_i2s).fifo_th = (1u32 << COMP1_FIFO_DEPTH(comp1_reg)) / 2;

    (*kmb_i2s).use_pio = !of_property_present(np, c"dmas".as_ptr());

    if (*kmb_i2s).use_pio {
        irq = platform_get_irq_optional(pdev, 0);
        if irq == -EPROBE_DEFER {
            return irq;
        }
        if irq > 0 {
            ret = devm_request_irq(dev, irq, kmb_i2s_irq_handler, 0, (*pdev).name, kmb_i2s as *mut c_void);
            if ret < 0 {
                dev_err(dev, c"failed to request irq\n".as_ptr());
                return ret;
            }
        }
        ret = devm_snd_soc_register_component(dev, &kmb_component, kmb_i2s_dai, 1);
    } else {
        (*kmb_i2s).play_dma_data.addr = (*res).start + I2S_TXDMA;
        (*kmb_i2s).capture_dma_data.addr = (*res).start + I2S_RXDMA;
        ret = snd_dmaengine_pcm_register(&mut (*pdev).dev, core::ptr::null_mut(), 0);
        if ret != 0 {
            dev_err(&mut (*pdev).dev, c"could not register dmaengine: %d\n".as_ptr(), ret);
            return ret;
        }
        ret = devm_snd_soc_register_component(dev, &kmb_component_dma, kmb_i2s_dai, 1);
    }

    if ret != 0 {
        dev_err(dev, c"not able to register dai\n".as_ptr());
        return ret;
    }

    /* To ensure none of the channels are enabled at boot up */
    kmb_i2s_disable_channels(kmb_i2s, SNDRV_PCM_STREAM_PLAYBACK as u32_t);
    kmb_i2s_disable_channels(kmb_i2s, SNDRV_PCM_STREAM_CAPTURE as u32_t);

    dev_set_drvdata(dev, kmb_i2s as *mut c_void);

    ret
}

static mut kmb_plat_dai_driver: platform_driver = platform_driver {
    driver: driver_inner {
        name: c"kmb-plat-dai".as_ptr(),
        of_match_table: kmb_plat_of_match.as_ptr(),
    },
    probe: Some(kmb_plat_dai_probe),
};

/* module_platform_driver(kmb_plat_dai_driver); */

/* MODULE_DESCRIPTION("ASoC Intel KeemBay Platform driver"); */
/* MODULE_AUTHOR("Sia Jee Heng <jee.heng.sia@intel.com>"); */
/* MODULE_AUTHOR("Sit, Michael Wei Hong <michael.wei.hong.sit@intel.com>"); */
/* MODULE_LICENSE("GPL v2"); */
/* MODULE_ALIAS("platform:kmb_platform"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
