// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2010-2011,2013-2015 The Linux Foundation. All rights reserved.
 *
 * lpass-platform.c -- ALSA SoC platform driver for QTi LPASS
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const DRV_NAME: &[u8] = b"lpass-platform\0";

const LPASS_PLATFORM_BUFFER_SIZE: usize = 24 * 2 * 1024;
const LPASS_PLATFORM_PERIODS: usize = 2;
const LPASS_RXTX_CDC_DMA_LPM_BUFF_SIZE: usize = 8 * 1024;
const LPASS_VA_CDC_DMA_LPM_BUFF_SIZE: usize = 12 * 1024;
const LPASS_CDC_DMA_REGISTER_FIELDS_MAX: c_int = 15;

type u32 = u32;
type size_t = usize;
type dma_addr_t = usize;
type snd_pcm_format_t = c_int;
type snd_pcm_uframes_t = isize;
type irqreturn_t = c_int;

const GFP_KERNEL: c_int = 0;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const EFAULT: c_int = 14;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_TRIGGER_RISING: c_ulong = 0x00000001;
const PAGE_SHIFT: c_int = 12;
const MEMREMAP_WC: c_int = 1;
const SNDRV_DMA_TYPE_CONTINUOUS: c_int = 1;
const SNDRV_DMA_TYPE_NONCOHERENT: c_int = 2;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_HW_PARAM_PERIODS: c_int = 0;
const SNDRV_PCM_STATE_DISCONNECTED: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 2;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 3;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 4;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 5;
const SNDRV_PCM_INFO_MMAP: u32 = 1 << 0;
const SNDRV_PCM_INFO_MMAP_VALID: u32 = 1 << 1;
const SNDRV_PCM_INFO_INTERLEAVED: u32 = 1 << 2;
const SNDRV_PCM_INFO_PAUSE: u32 = 1 << 3;
const SNDRV_PCM_INFO_RESUME: u32 = 1 << 4;
const SNDRV_PCM_FMTBIT_S16: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_S24: u64 = 1 << 1;
const SNDRV_PCM_FMTBIT_S32: u64 = 1 << 2;
const SNDRV_PCM_RATE_8000_192000: u32 = 0;

extern "C" {
    static mut MI2S_PRIMARY: u32;
    static mut MI2S_SECONDARY: u32;
    static mut MI2S_TERTIARY: u32;
    static mut MI2S_QUATERNARY: u32;
    static mut MI2S_QUINARY: u32;
    static mut LPASS_DP_RX: u32;
    static mut LPASS_CDC_DMA_RX0: u32;
    static mut LPASS_CDC_DMA_RX9: u32;
    static mut LPASS_CDC_DMA_TX0: u32;
    static mut LPASS_CDC_DMA_TX8: u32;
    static mut LPASS_CDC_DMA_VA_TX0: u32;
    static mut LPASS_CDC_DMA_VA_TX8: u32;
    static mut LPASS_MAX_DMA_CHANNELS: c_int;
    static mut LPASS_MAX_HDMI_DMA_CHANNELS: c_int;
    static mut LPASS_MAX_CDC_DMA_CHANNELS: c_int;
    static mut LPASS_MAX_VA_CDC_DMA_CHANNELS: c_int;
    static mut LPAIF_IRQ_PORT_HOST: c_int;
    static mut LPAIF_DMACTL_BURSTEN_INCR4: u32;
    static mut LPAIF_DMACTL_FIFOWM_8: u32;
    static mut LPAIF_DMACTL_WPSCNT_ONE: u32;
    static mut LPAIF_DMACTL_WPSCNT_TWO: u32;
    static mut LPAIF_DMACTL_WPSCNT_THREE: u32;
    static mut LPAIF_DMACTL_WPSCNT_FOUR: u32;
    static mut LPAIF_DMACTL_WPSCNT_SIX: u32;
    static mut LPAIF_DMACTL_WPSCNT_EIGHT: u32;
    static mut LPAIF_DMACTL_ENABLE_ON: u32;
    static mut LPAIF_DMACTL_ENABLE_OFF: u32;
    static mut LPAIF_DMACTL_DYNCLK_ON: u32;
    static mut LPAIF_DMACTL_DYNCLK_OFF: u32;

    fn LPAIF_DMACTL_AUDINTF(port: c_int) -> u32;
    fn LPAIF_DMACTL_REG(v: *const lpass_variant, ch: c_int, dir: c_int, dai_id: u32) -> u32;
    fn LPAIF_DMABASE_REG(v: *const lpass_variant, ch: c_int, dir: c_int, dai_id: u32) -> u32;
    fn LPAIF_DMABUFF_REG(v: *const lpass_variant, ch: c_int, dir: c_int, dai_id: u32) -> u32;
    fn LPAIF_DMAPER_REG(v: *const lpass_variant, ch: c_int, dir: c_int, dai_id: u32) -> u32;
    fn LPAIF_DMACURR_REG(v: *const lpass_variant, ch: c_int, dir: c_int, dai_id: u32) -> u32;
    fn LPAIF_IRQ_ALL(chan: c_int) -> u32;
    fn LPAIF_IRQ_PER(chan: c_int) -> u32;
    fn LPAIF_IRQ_XRUN(chan: c_int) -> u32;
    fn LPAIF_IRQ_ERR(chan: c_int) -> u32;
    fn LPAIF_IRQCLEAR_REG(v: *const lpass_variant, port: c_int) -> u32;
    fn LPAIF_IRQEN_REG(v: *const lpass_variant, port: c_int) -> u32;
    fn LPAIF_IRQSTAT_REG(v: *const lpass_variant, port: c_int) -> u32;
    fn LPASS_HDMITX_APP_IRQCLEAR_REG(v: *const lpass_variant) -> u32;
    fn LPASS_HDMITX_APP_IRQEN_REG(v: *const lpass_variant) -> u32;
    fn LPASS_HDMITX_APP_IRQSTAT_REG(v: *const lpass_variant) -> u32;
    fn LPAIF_IRQ_HDMI_REQ_ON_PRELOAD(chan: c_int) -> u32;
    fn LPAIF_IRQ_HDMI_SDEEP_AUD_DIS(chan: c_int) -> u32;
    static mut LPAIF_IRQ_HDMI_METADONE: u32;
    fn LPAIF_RXTX_IRQCLEAR_REG(v: *const lpass_variant, port: c_int) -> u32;
    fn LPAIF_RXTX_IRQEN_REG(v: *const lpass_variant, port: c_int) -> u32;
    fn LPAIF_RXTX_IRQSTAT_REG(v: *const lpass_variant, port: c_int) -> u32;
    fn LPAIF_VA_IRQCLEAR_REG(v: *const lpass_variant, port: c_int) -> u32;
    fn LPAIF_VA_IRQEN_REG(v: *const lpass_variant, port: c_int) -> u32;
    fn LPAIF_VA_IRQSTAT_REG(v: *const lpass_variant, port: c_int) -> u32;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_field {
    _private: [u8; 0],
}

#[repr(C)]
pub struct reg_field {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_area_struct {
    pub vm_page_prot: c_ulong,
    pub vm_end: c_ulong,
    pub vm_start: c_ulong,
    pub vm_pgoff: c_ulong,
}

#[repr(C)]
pub struct iov_iter {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: u32,
    pub formats: u64,
    pub rates: u32,
    pub rate_min: u32,
    pub rate_max: u32,
    pub channels_min: u32,
    pub channels_max: u32,
    pub buffer_bytes_max: usize,
    pub period_bytes_min: usize,
    pub period_bytes_max: usize,
    pub periods_min: u32,
    pub periods_max: u32,
    pub fifo_size: usize,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub private_data: *mut c_void,
    pub dma_bytes: usize,
    pub dma_addr: dma_addr_t,
    pub dma_area: *mut u8,
    pub channels: u32,
}

#[repr(C)]
pub struct snd_dma_device {
    pub dev: *mut device,
    pub type_: c_int,
}

#[repr(C)]
pub struct snd_dma_buffer {
    pub dev: snd_dma_device,
    pub private_data: *mut c_void,
    pub bytes: usize,
    pub addr: dma_addr_t,
    pub area: *mut u8,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: c_int,
    pub dma_buffer: snd_dma_buffer,
}

#[repr(C)]
pub struct snd_pcm_str {
    pub substream: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm {
    pub streams: [snd_pcm_str; 2],
    pub card: *mut snd_card,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub id: u32,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub driver: *mut snd_soc_dai_driver,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dev: *mut device,
    pub pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct lpaif_dmactl {
    pub intf: *mut regmap_field,
    pub bursten: *mut regmap_field,
    pub fifowm: *mut regmap_field,
    pub wpscnt: *mut regmap_field,
    pub burst8: *mut regmap_field,
    pub burst16: *mut regmap_field,
    pub dynburst: *mut regmap_field,
    pub enable: *mut regmap_field,
    pub dyncclk: *mut regmap_field,
}

#[repr(C)]
pub struct lpass_variant {
    pub rxtx_rdma_intf: reg_field,
    pub rxtx_wrdma_intf: reg_field,
    pub va_wrdma_intf: reg_field,
    pub rdma_intf: reg_field,
    pub wrdma_intf: reg_field,
    pub hdmi_rdma_bursten: reg_field,
    pub wrdma_channel_start: c_int,
    pub rxtx_wrdma_channel_start: c_int,
    pub va_wrdma_channel_start: c_int,
    pub dmactl_audif_start: c_int,
    pub alloc_dma_channel: Option<unsafe extern "C" fn(*mut lpass_data, c_int, u32) -> c_int>,
    pub free_dma_channel: Option<unsafe extern "C" fn(*mut lpass_data, c_int, u32)>,
}

#[repr(C)]
pub struct lpass_pcm_data {
    pub i2s_port: u32,
    pub dma_ch: c_int,
}

#[repr(C)]
pub struct lpass_data {
    pub variant: *const lpass_variant,
    pub rxtx_rd_dmactl: *mut lpaif_dmactl,
    pub rxtx_wr_dmactl: *mut lpaif_dmactl,
    pub va_wr_dmactl: *mut lpaif_dmactl,
    pub rd_dmactl: *mut lpaif_dmactl,
    pub wr_dmactl: *mut lpaif_dmactl,
    pub hdmi_rd_dmactl: *mut lpaif_dmactl,
    pub lpaif_map: *mut regmap,
    pub hdmiif_map: *mut regmap,
    pub rxtx_lpaif_map: *mut regmap,
    pub va_lpaif_map: *mut regmap,
    pub substream: [*mut snd_pcm_substream; 32],
    pub hdmi_substream: [*mut snd_pcm_substream; 32],
    pub rxtx_substream: [*mut snd_pcm_substream; 32],
    pub va_substream: [*mut snd_pcm_substream; 32],
    pub rxtx_cdc_dma_lpm_buf: dma_addr_t,
    pub va_cdc_dma_lpm_buf: dma_addr_t,
    pub hdmi_port_enable: bool,
    pub codec_dma_enable: bool,
    pub lpaif_irq: c_int,
    pub rxtxif_irq: c_int,
    pub vaif_irq: c_int,
    pub hdmiif_irq: c_int,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    pub mmap: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, *mut vm_area_struct) -> c_int>,
    pub pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> c_int>,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub copy: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, c_int, c_ulong, *mut iov_iter, c_ulong) -> c_int>,
}

extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_int) -> *mut c_void;
    fn kzalloc(size: usize, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn devm_regmap_field_bulk_alloc(dev: *mut device, map: *mut regmap, field: *mut *mut regmap_field, reg: *const reg_field, num: c_int) -> c_int;
    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> c_int;
    fn regmap_write_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> c_int;
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut u32) -> c_int;
    fn regmap_fields_write(field: *mut regmap_field, id: c_int, val: u32) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *const snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_set_runtime_hwparams(substream: *mut snd_pcm_substream, hw: *const snd_pcm_hardware);
    fn snd_pcm_set_runtime_buffer(substream: *mut snd_pcm_substream, buf: *mut snd_dma_buffer);
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, param: c_int) -> c_int;
    fn snd_pcm_format_width(format: snd_pcm_format_t) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> snd_pcm_format_t;
    fn params_channels(params: *mut snd_pcm_hw_params) -> u32;
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> usize;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> usize;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: u32) -> snd_pcm_uframes_t;
    fn pgprot_writecombine(prot: c_ulong) -> c_ulong;
    fn io_remap_pfn_range(vma: *mut vm_area_struct, addr: c_ulong, pfn: c_ulong, size: c_ulong, prot: c_ulong) -> c_int;
    fn snd_pcm_lib_default_mmap(substream: *mut snd_pcm_substream, vma: *mut vm_area_struct) -> c_int;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_pcm_stop_xrun(substream: *mut snd_pcm_substream);
    fn snd_pcm_stop(substream: *mut snd_pcm_substream, state: c_int);
    fn platform_get_irq_byname(pdev: *mut platform_device, name: *const c_char) -> c_int;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, data: *mut c_void) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, driver: *const snd_soc_component_driver, dai_drv: *mut c_void, num_dai: c_int) -> c_int;
    fn snd_pcm_set_fixed_buffer_all(pcm: *mut snd_pcm, type_: c_int, dev: *mut device, size: usize) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn copy_from_iter_toio(dst: *mut c_void, bytes: c_ulong, iter: *mut iov_iter) -> c_ulong;
    fn copy_from_iter(dst: *mut c_void, bytes: c_ulong, iter: *mut iov_iter) -> c_ulong;
    fn copy_to_iter_fromio(src: *mut c_void, bytes: c_ulong, iter: *mut iov_iter) -> c_ulong;
    fn copy_to_iter(src: *mut c_void, bytes: c_ulong, iter: *mut iov_iter) -> c_ulong;
    fn memremap(offset: dma_addr_t, size: usize, flags: c_int) -> *mut c_void;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn_ratelimited(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
}

static lpass_platform_pcm_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_RESUME,
    formats: SNDRV_PCM_FMTBIT_S16 | SNDRV_PCM_FMTBIT_S24 | SNDRV_PCM_FMTBIT_S32,
    rates: SNDRV_PCM_RATE_8000_192000,
    rate_min: 8000,
    rate_max: 192000,
    channels_min: 1,
    channels_max: 8,
    buffer_bytes_max: LPASS_PLATFORM_BUFFER_SIZE,
    period_bytes_max: LPASS_PLATFORM_BUFFER_SIZE / LPASS_PLATFORM_PERIODS,
    period_bytes_min: LPASS_PLATFORM_BUFFER_SIZE / LPASS_PLATFORM_PERIODS,
    periods_min: LPASS_PLATFORM_PERIODS as u32,
    periods_max: LPASS_PLATFORM_PERIODS as u32,
    fifo_size: 0,
};

static lpass_platform_rxtx_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_RESUME,
    formats: SNDRV_PCM_FMTBIT_S16 | SNDRV_PCM_FMTBIT_S24 | SNDRV_PCM_FMTBIT_S32,
    rates: SNDRV_PCM_RATE_8000_192000,
    rate_min: 8000,
    rate_max: 192000,
    channels_min: 1,
    channels_max: 8,
    buffer_bytes_max: LPASS_RXTX_CDC_DMA_LPM_BUFF_SIZE,
    period_bytes_max: LPASS_RXTX_CDC_DMA_LPM_BUFF_SIZE / LPASS_PLATFORM_PERIODS,
    period_bytes_min: LPASS_RXTX_CDC_DMA_LPM_BUFF_SIZE / LPASS_PLATFORM_PERIODS,
    periods_min: LPASS_PLATFORM_PERIODS as u32,
    periods_max: LPASS_PLATFORM_PERIODS as u32,
    fifo_size: 0,
};

static lpass_platform_va_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_RESUME,
    formats: SNDRV_PCM_FMTBIT_S16 | SNDRV_PCM_FMTBIT_S24 | SNDRV_PCM_FMTBIT_S32,
    rates: SNDRV_PCM_RATE_8000_192000,
    rate_min: 8000,
    rate_max: 192000,
    channels_min: 1,
    channels_max: 8,
    buffer_bytes_max: LPASS_VA_CDC_DMA_LPM_BUFF_SIZE,
    period_bytes_max: LPASS_VA_CDC_DMA_LPM_BUFF_SIZE / LPASS_PLATFORM_PERIODS,
    period_bytes_min: LPASS_VA_CDC_DMA_LPM_BUFF_SIZE / LPASS_PLATFORM_PERIODS,
    periods_min: LPASS_PLATFORM_PERIODS as u32,
    periods_max: LPASS_PLATFORM_PERIODS as u32,
    fifo_size: 0,
};

unsafe fn is_between(value: u32, start: u32, end: u32) -> bool {
    value >= start && value <= end
}

unsafe fn is_mi2s_port(dai_id: u32) -> bool {
    is_between(dai_id, MI2S_PRIMARY, MI2S_QUINARY)
}

unsafe fn is_cdc_dma_port(dai_id: u32) -> bool {
    is_between(dai_id, LPASS_CDC_DMA_RX0, LPASS_CDC_DMA_RX9)
        || is_between(dai_id, LPASS_CDC_DMA_TX0, LPASS_CDC_DMA_TX8)
        || is_between(dai_id, LPASS_CDC_DMA_VA_TX0, LPASS_CDC_DMA_VA_TX8)
}

unsafe extern "C" fn lpass_platform_alloc_rxtx_dmactl_fields(dev: *mut device, map: *mut regmap) -> c_int {
    let drvdata = dev_get_drvdata(dev) as *mut lpass_data;
    let v = (*drvdata).variant;
    let mut rval: c_int;

    let rd_dmactl = devm_kzalloc(dev, size_of::<lpaif_dmactl>(), GFP_KERNEL) as *mut lpaif_dmactl;
    if rd_dmactl.is_null() {
        return -ENOMEM;
    }

    let wr_dmactl = devm_kzalloc(dev, size_of::<lpaif_dmactl>(), GFP_KERNEL) as *mut lpaif_dmactl;
    if wr_dmactl.is_null() {
        return -ENOMEM;
    }

    (*drvdata).rxtx_rd_dmactl = rd_dmactl;
    (*drvdata).rxtx_wr_dmactl = wr_dmactl;

    rval = devm_regmap_field_bulk_alloc(dev, map, &mut (*rd_dmactl).intf, &(*v).rxtx_rdma_intf, LPASS_CDC_DMA_REGISTER_FIELDS_MAX);
    if rval != 0 {
        return rval;
    }

    devm_regmap_field_bulk_alloc(dev, map, &mut (*wr_dmactl).intf, &(*v).rxtx_wrdma_intf, LPASS_CDC_DMA_REGISTER_FIELDS_MAX)
}

unsafe extern "C" fn lpass_platform_alloc_va_dmactl_fields(dev: *mut device, map: *mut regmap) -> c_int {
    let drvdata = dev_get_drvdata(dev) as *mut lpass_data;
    let v = (*drvdata).variant;
    let wr_dmactl = devm_kzalloc(dev, size_of::<lpaif_dmactl>(), GFP_KERNEL) as *mut lpaif_dmactl;
    if wr_dmactl.is_null() {
        return -ENOMEM;
    }

    (*drvdata).va_wr_dmactl = wr_dmactl;
    devm_regmap_field_bulk_alloc(dev, map, &mut (*wr_dmactl).intf, &(*v).va_wrdma_intf, LPASS_CDC_DMA_REGISTER_FIELDS_MAX)
}

unsafe extern "C" fn lpass_platform_alloc_dmactl_fields(dev: *mut device, map: *mut regmap) -> c_int {
    let drvdata = dev_get_drvdata(dev) as *mut lpass_data;
    let v = (*drvdata).variant;
    let mut rval: c_int;

    (*drvdata).rd_dmactl = devm_kzalloc(dev, size_of::<lpaif_dmactl>(), GFP_KERNEL) as *mut lpaif_dmactl;
    if (*drvdata).rd_dmactl.is_null() {
        return -ENOMEM;
    }

    (*drvdata).wr_dmactl = devm_kzalloc(dev, size_of::<lpaif_dmactl>(), GFP_KERNEL) as *mut lpaif_dmactl;
    if (*drvdata).wr_dmactl.is_null() {
        return -ENOMEM;
    }

    let rd_dmactl = (*drvdata).rd_dmactl;
    let wr_dmactl = (*drvdata).wr_dmactl;

    rval = devm_regmap_field_bulk_alloc(dev, map, &mut (*rd_dmactl).intf, &(*v).rdma_intf, 6);
    if rval != 0 {
        return rval;
    }

    devm_regmap_field_bulk_alloc(dev, map, &mut (*wr_dmactl).intf, &(*v).wrdma_intf, 6)
}

unsafe extern "C" fn lpass_platform_alloc_hdmidmactl_fields(dev: *mut device, map: *mut regmap) -> c_int {
    let drvdata = dev_get_drvdata(dev) as *mut lpass_data;
    let v = (*drvdata).variant;
    let rd_dmactl = devm_kzalloc(dev, size_of::<lpaif_dmactl>(), GFP_KERNEL) as *mut lpaif_dmactl;
    if rd_dmactl.is_null() {
        return -ENOMEM;
    }

    (*drvdata).hdmi_rd_dmactl = rd_dmactl;

    devm_regmap_field_bulk_alloc(dev, map, &mut (*rd_dmactl).bursten, &(*v).hdmi_rdma_bursten, 8)
}

unsafe extern "C" fn lpass_platform_pcmops_open(component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let soc_runtime = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(soc_runtime, 0);
    let drvdata = snd_soc_component_get_drvdata(component) as *mut lpass_data;
    let v = (*drvdata).variant;
    let dir = (*substream).stream;
    let dai_id = (*(*cpu_dai).driver).id;
    let mut map: *mut regmap = ptr::null_mut();
    let mut ret: c_int;
    let data = kzalloc(size_of::<lpass_pcm_data>(), GFP_KERNEL) as *mut lpass_pcm_data;
    if data.is_null() {
        return -ENOMEM;
    }

    (*data).i2s_port = dai_id;
    (*runtime).private_data = data as *mut c_void;

    let dma_ch = if let Some(alloc_dma_channel) = (*v).alloc_dma_channel {
        alloc_dma_channel(drvdata, dir, dai_id)
    } else {
        0
    };

    if dma_ch < 0 {
        kfree(data as *mut c_void);
        return dma_ch;
    }

    if is_mi2s_port(dai_id) {
        map = (*drvdata).lpaif_map;
        (*drvdata).substream[dma_ch as usize] = substream;
    } else if dai_id == LPASS_DP_RX {
        map = (*drvdata).hdmiif_map;
        (*drvdata).hdmi_substream[dma_ch as usize] = substream;
    } else if is_between(dai_id, LPASS_CDC_DMA_RX0, LPASS_CDC_DMA_RX9) || is_between(dai_id, LPASS_CDC_DMA_TX0, LPASS_CDC_DMA_TX8) {
        map = (*drvdata).rxtx_lpaif_map;
        (*drvdata).rxtx_substream[dma_ch as usize] = substream;
    } else if is_between(dai_id, LPASS_CDC_DMA_VA_TX0, LPASS_CDC_DMA_VA_TX8) {
        map = (*drvdata).va_lpaif_map;
        (*drvdata).va_substream[dma_ch as usize] = substream;
    }

    (*data).dma_ch = dma_ch;
    if is_mi2s_port(dai_id) || dai_id == LPASS_DP_RX {
        ret = regmap_write(map, LPAIF_DMACTL_REG(v, dma_ch, dir, (*data).i2s_port), 0);
        if ret != 0 {
            kfree(data as *mut c_void);
            dev_err((*soc_runtime).dev, b"error writing to rdmactl reg: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
        snd_soc_set_runtime_hwparams(substream, &lpass_platform_pcm_hardware);
        (*runtime).dma_bytes = lpass_platform_pcm_hardware.buffer_bytes_max;
    } else if is_between(dai_id, LPASS_CDC_DMA_RX0, LPASS_CDC_DMA_RX9) || is_between(dai_id, LPASS_CDC_DMA_TX0, LPASS_CDC_DMA_TX8) {
        snd_soc_set_runtime_hwparams(substream, &lpass_platform_rxtx_hardware);
        (*runtime).dma_bytes = lpass_platform_rxtx_hardware.buffer_bytes_max;
        snd_pcm_set_runtime_buffer(substream, &mut (*substream).dma_buffer);
    } else if is_between(dai_id, LPASS_CDC_DMA_VA_TX0, LPASS_CDC_DMA_VA_TX8) {
        snd_soc_set_runtime_hwparams(substream, &lpass_platform_va_hardware);
        (*runtime).dma_bytes = lpass_platform_va_hardware.buffer_bytes_max;
        snd_pcm_set_runtime_buffer(substream, &mut (*substream).dma_buffer);
    }

    ret = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if ret < 0 {
        kfree(data as *mut c_void);
        dev_err((*soc_runtime).dev, b"setting constraints failed: %d\n\0".as_ptr() as *const c_char, ret);
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn lpass_platform_pcmops_close(component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let soc_runtime = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(soc_runtime, 0);
    let drvdata = snd_soc_component_get_drvdata(component) as *mut lpass_data;
    let v = (*drvdata).variant;
    let data = (*runtime).private_data as *mut lpass_pcm_data;
    let dai_id = (*(*cpu_dai).driver).id;

    if is_mi2s_port(dai_id) {
        (*drvdata).substream[(*data).dma_ch as usize] = ptr::null_mut();
    } else if dai_id == LPASS_DP_RX {
        (*drvdata).hdmi_substream[(*data).dma_ch as usize] = ptr::null_mut();
    } else if is_between(dai_id, LPASS_CDC_DMA_RX0, LPASS_CDC_DMA_RX9) || is_between(dai_id, LPASS_CDC_DMA_TX0, LPASS_CDC_DMA_TX8) {
        (*drvdata).rxtx_substream[(*data).dma_ch as usize] = ptr::null_mut();
    } else if is_between(dai_id, LPASS_CDC_DMA_VA_TX0, LPASS_CDC_DMA_VA_TX8) {
        (*drvdata).va_substream[(*data).dma_ch as usize] = ptr::null_mut();
    }

    if let Some(free_dma_channel) = (*v).free_dma_channel {
        free_dma_channel(drvdata, (*data).dma_ch, dai_id);
    }

    kfree(data as *mut c_void);
    0
}

unsafe extern "C" fn __lpass_get_dmactl_handle(substream: *const snd_pcm_substream, component: *mut snd_soc_component) -> *mut lpaif_dmactl {
    let soc_runtime = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(soc_runtime, 0);
    let drvdata = snd_soc_component_get_drvdata(component) as *mut lpass_data;
    let mut dmactl: *mut lpaif_dmactl = ptr::null_mut();
    let dai_id = (*(*cpu_dai).driver).id;

    if is_mi2s_port(dai_id) {
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            dmactl = (*drvdata).rd_dmactl;
        } else {
            dmactl = (*drvdata).wr_dmactl;
        }
    } else if dai_id == LPASS_DP_RX {
        dmactl = (*drvdata).hdmi_rd_dmactl;
    } else if is_between(dai_id, LPASS_CDC_DMA_RX0, LPASS_CDC_DMA_RX9) {
        dmactl = (*drvdata).rxtx_rd_dmactl;
    } else if is_between(dai_id, LPASS_CDC_DMA_TX0, LPASS_CDC_DMA_TX8) {
        dmactl = (*drvdata).rxtx_wr_dmactl;
    } else if is_between(dai_id, LPASS_CDC_DMA_VA_TX0, LPASS_CDC_DMA_VA_TX8) {
        dmactl = (*drvdata).va_wr_dmactl;
    }

    dmactl
}

unsafe extern "C" fn __lpass_get_id(substream: *const snd_pcm_substream, component: *mut snd_soc_component) -> c_int {
    let soc_runtime = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(soc_runtime, 0);
    let drvdata = snd_soc_component_get_drvdata(component) as *mut lpass_data;
    let rt = (*substream).runtime;
    let pcm_data = (*rt).private_data as *mut lpass_pcm_data;
    let v = (*drvdata).variant;
    let mut id: c_int = 0;
    let dai_id = (*(*cpu_dai).driver).id;

    if is_mi2s_port(dai_id) {
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            id = (*pcm_data).dma_ch;
        } else {
            id = (*pcm_data).dma_ch - (*v).wrdma_channel_start;
        }
    } else if dai_id == LPASS_DP_RX || is_between(dai_id, LPASS_CDC_DMA_RX0, LPASS_CDC_DMA_RX9) {
        id = (*pcm_data).dma_ch;
    } else if is_between(dai_id, LPASS_CDC_DMA_TX0, LPASS_CDC_DMA_TX8) {
        id = (*pcm_data).dma_ch - (*v).rxtx_wrdma_channel_start;
    } else if is_between(dai_id, LPASS_CDC_DMA_VA_TX0, LPASS_CDC_DMA_VA_TX8) {
        id = (*pcm_data).dma_ch - (*v).va_wrdma_channel_start;
    }

    id
}

unsafe extern "C" fn __lpass_get_regmap_handle(substream: *const snd_pcm_substream, component: *mut snd_soc_component) -> *mut regmap {
    let soc_runtime = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(soc_runtime, 0);
    let drvdata = snd_soc_component_get_drvdata(component) as *mut lpass_data;
    let dai_id = (*(*cpu_dai).driver).id;

    if is_mi2s_port(dai_id) {
        (*drvdata).lpaif_map
    } else if dai_id == LPASS_DP_RX {
        (*drvdata).hdmiif_map
    } else if is_between(dai_id, LPASS_CDC_DMA_RX0, LPASS_CDC_DMA_RX9) || is_between(dai_id, LPASS_CDC_DMA_TX0, LPASS_CDC_DMA_TX8) {
        (*drvdata).rxtx_lpaif_map
    } else if is_between(dai_id, LPASS_CDC_DMA_VA_TX0, LPASS_CDC_DMA_VA_TX8) {
        (*drvdata).va_lpaif_map
    } else {
        ptr::null_mut()
    }
}

unsafe extern "C" fn lpass_platform_pcmops_hw_params(component: *mut snd_soc_component, substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    let soc_runtime = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(soc_runtime, 0);
    let drvdata = snd_soc_component_get_drvdata(component) as *mut lpass_data;
    let rt = (*substream).runtime;
    let pcm_data = (*rt).private_data as *mut lpass_pcm_data;
    let v = (*drvdata).variant;
    let format = params_format(params);
    let channels = params_channels(params);
    let mut regval: u32;
    let dmactl = __lpass_get_dmactl_handle(substream, component);
    let id = __lpass_get_id(substream, component);
    let bitwidth = snd_pcm_format_width(format);
    let mut ret: c_int;
    let dma_port = (*pcm_data).i2s_port as c_int + (*v).dmactl_audif_start;
    let dai_id = (*(*cpu_dai).driver).id;

    if bitwidth < 0 {
        dev_err((*soc_runtime).dev, b"invalid bit width given: %d\n\0".as_ptr() as *const c_char, bitwidth);
        return bitwidth;
    }

    ret = regmap_fields_write((*dmactl).bursten, id, LPAIF_DMACTL_BURSTEN_INCR4);
    if ret != 0 {
        dev_err((*soc_runtime).dev, b"error updating bursten field: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = regmap_fields_write((*dmactl).fifowm, id, LPAIF_DMACTL_FIFOWM_8);
    if ret != 0 {
        dev_err((*soc_runtime).dev, b"error updating fifowm field: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    if dai_id == LPASS_DP_RX {
        ret = regmap_fields_write((*dmactl).burst8, id, LPAIF_DMACTL_BURSTEN_INCR4);
        if ret != 0 {
            dev_err((*soc_runtime).dev, b"error updating burst8en field: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
        ret = regmap_fields_write((*dmactl).burst16, id, LPAIF_DMACTL_BURSTEN_INCR4);
        if ret != 0 {
            dev_err((*soc_runtime).dev, b"error updating burst16en field: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
        ret = regmap_fields_write((*dmactl).dynburst, id, LPAIF_DMACTL_BURSTEN_INCR4);
        if ret != 0 {
            dev_err((*soc_runtime).dev, b"error updating dynbursten field: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
    } else if dai_id == MI2S_PRIMARY || dai_id == MI2S_SECONDARY || dai_id == MI2S_TERTIARY || dai_id == MI2S_QUATERNARY || dai_id == MI2S_QUINARY {
        ret = regmap_fields_write((*dmactl).intf, id, LPAIF_DMACTL_AUDINTF(dma_port));
        if ret != 0 {
            dev_err((*soc_runtime).dev, b"error updating audio interface field: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
    } else if is_between(dai_id, LPASS_CDC_DMA_RX0, LPASS_CDC_DMA_RX9)
        || is_between(dai_id, LPASS_CDC_DMA_TX0, LPASS_CDC_DMA_TX8)
        || is_between(dai_id, LPASS_CDC_DMA_VA_TX0, LPASS_CDC_DMA_VA_TX0) {
    } else {
        dev_err((*soc_runtime).dev, b"%s: invalid  interface: %d\n\0".as_ptr() as *const c_char, b"lpass_platform_pcmops_hw_params\0".as_ptr(), dai_id);
    }

    match bitwidth {
        16 => match channels {
            1 | 2 => regval = LPAIF_DMACTL_WPSCNT_ONE,
            4 => regval = LPAIF_DMACTL_WPSCNT_TWO,
            6 => regval = LPAIF_DMACTL_WPSCNT_THREE,
            8 => regval = LPAIF_DMACTL_WPSCNT_FOUR,
            _ => {
                dev_err((*soc_runtime).dev, b"invalid PCM config given: bw=%d, ch=%u\n\0".as_ptr() as *const c_char, bitwidth, channels);
                return -EINVAL;
            }
        },
        24 | 32 => match channels {
            1 => regval = LPAIF_DMACTL_WPSCNT_ONE,
            2 => regval = if dai_id == LPASS_DP_RX { LPAIF_DMACTL_WPSCNT_ONE } else { LPAIF_DMACTL_WPSCNT_TWO },
            4 => regval = if dai_id == LPASS_DP_RX { LPAIF_DMACTL_WPSCNT_TWO } else { LPAIF_DMACTL_WPSCNT_FOUR },
            6 => regval = if dai_id == LPASS_DP_RX { LPAIF_DMACTL_WPSCNT_THREE } else { LPAIF_DMACTL_WPSCNT_SIX },
            8 => regval = if dai_id == LPASS_DP_RX { LPAIF_DMACTL_WPSCNT_FOUR } else { LPAIF_DMACTL_WPSCNT_EIGHT },
            _ => {
                dev_err((*soc_runtime).dev, b"invalid PCM config given: bw=%d, ch=%u\n\0".as_ptr() as *const c_char, bitwidth, channels);
                return -EINVAL;
            }
        },
        _ => {
            dev_err((*soc_runtime).dev, b"invalid PCM config given: bw=%d, ch=%u\n\0".as_ptr() as *const c_char, bitwidth, channels);
            return -EINVAL;
        }
    }

    ret = regmap_fields_write((*dmactl).wpscnt, id, regval);
    if ret != 0 {
        dev_err((*soc_runtime).dev, b"error writing to dmactl reg: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    0
}

unsafe extern "C" fn lpass_platform_pcmops_hw_free(component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> c_int {
    let soc_runtime = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(soc_runtime, 0);
    let drvdata = snd_soc_component_get_drvdata(component) as *mut lpass_data;
    let rt = (*substream).runtime;
    let pcm_data = (*rt).private_data as *mut lpass_pcm_data;
    let v = (*drvdata).variant;
    let dai_id = (*(*cpu_dai).driver).id;

    if is_cdc_dma_port(dai_id) {
        return 0;
    }
    let map = __lpass_get_regmap_handle(substream, component);
    let reg = LPAIF_DMACTL_REG(v, (*pcm_data).dma_ch, (*substream).stream, dai_id);
    let ret = regmap_write(map, reg, 0);
    if ret != 0 {
        dev_err((*soc_runtime).dev, b"error writing to rdmactl reg: %d\n\0".as_ptr() as *const c_char, ret);
    }

    ret
}

unsafe extern "C" fn lpass_platform_pcmops_prepare(component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let soc_runtime = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(soc_runtime, 0);
    let drvdata = snd_soc_component_get_drvdata(component) as *mut lpass_data;
    let rt = (*substream).runtime;
    let pcm_data = (*rt).private_data as *mut lpass_pcm_data;
    let v = (*drvdata).variant;
    let dir = (*substream).stream;
    let dai_id = (*(*cpu_dai).driver).id;
    let ch = (*pcm_data).dma_ch;
    let dmactl = __lpass_get_dmactl_handle(substream, component);
    let id = __lpass_get_id(substream, component);
    let map = __lpass_get_regmap_handle(substream, component);
    let mut ret: c_int;

    ret = regmap_write(map, LPAIF_DMABASE_REG(v, ch, dir, dai_id), (*runtime).dma_addr as u32);
    if ret != 0 {
        dev_err((*soc_runtime).dev, b"error writing to rdmabase reg: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = regmap_write(map, LPAIF_DMABUFF_REG(v, ch, dir, dai_id), ((snd_pcm_lib_buffer_bytes(substream) >> 2) - 1) as u32);
    if ret != 0 {
        dev_err((*soc_runtime).dev, b"error writing to rdmabuff reg: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = regmap_write(map, LPAIF_DMAPER_REG(v, ch, dir, dai_id), ((snd_pcm_lib_period_bytes(substream) >> 2) - 1) as u32);
    if ret != 0 {
        dev_err((*soc_runtime).dev, b"error writing to rdmaper reg: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    if is_cdc_dma_port(dai_id) {
        ret = regmap_fields_write((*dmactl).fifowm, id, LPAIF_DMACTL_FIFOWM_8);
        if ret != 0 {
            dev_err((*soc_runtime).dev, b"error writing fifowm field to dmactl reg: %d, id: %d\n\0".as_ptr() as *const c_char, ret, id);
            return ret;
        }
    }
    ret = regmap_fields_write((*dmactl).enable, id, LPAIF_DMACTL_ENABLE_ON);
    if ret != 0 {
        dev_err((*soc_runtime).dev, b"error writing to rdmactl reg: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    0
}

unsafe extern "C" fn lpass_platform_pcmops_trigger(component: *mut snd_soc_component, substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let soc_runtime = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(soc_runtime, 0);
    let drvdata = snd_soc_component_get_drvdata(component) as *mut lpass_data;
    let rt = (*substream).runtime;
    let pcm_data = (*rt).private_data as *mut lpass_pcm_data;
    let v = (*drvdata).variant;
    let ch = (*pcm_data).dma_ch;
    let dmactl = __lpass_get_dmactl_handle(substream, component);
    let id = __lpass_get_id(substream, component);
    let map = __lpass_get_regmap_handle(substream, component);
    let dai_id = (*(*cpu_dai).driver).id;
    let mut ret: c_int;
    let mut reg_irqclr: u32 = 0;
    let mut val_irqclr: u32 = 0;
    let mut reg_irqen: u32 = 0;
    let mut val_irqen: u32 = 0;
    let mut val_mask: u32 = 0;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            ret = regmap_fields_write((*dmactl).enable, id, LPAIF_DMACTL_ENABLE_ON);
            if ret != 0 {
                dev_err((*soc_runtime).dev, b"error writing to rdmactl reg: %d\n\0".as_ptr() as *const c_char, ret);
                return ret;
            }
            if dai_id == LPASS_DP_RX {
                ret = regmap_fields_write((*dmactl).dyncclk, id, LPAIF_DMACTL_DYNCLK_ON);
                if ret != 0 {
                    dev_err((*soc_runtime).dev, b"error writing to rdmactl reg: %d\n\0".as_ptr() as *const c_char, ret);
                    return ret;
                }
                reg_irqclr = LPASS_HDMITX_APP_IRQCLEAR_REG(v);
                val_irqclr = LPAIF_IRQ_ALL(ch) | LPAIF_IRQ_HDMI_REQ_ON_PRELOAD(ch) | LPAIF_IRQ_HDMI_METADONE | LPAIF_IRQ_HDMI_SDEEP_AUD_DIS(ch);
                reg_irqen = LPASS_HDMITX_APP_IRQEN_REG(v);
                val_mask = val_irqclr;
                val_irqen = val_irqclr;
            } else if is_mi2s_port(dai_id) {
                reg_irqclr = LPAIF_IRQCLEAR_REG(v, LPAIF_IRQ_PORT_HOST);
                val_irqclr = LPAIF_IRQ_ALL(ch);
                reg_irqen = LPAIF_IRQEN_REG(v, LPAIF_IRQ_PORT_HOST);
                val_mask = LPAIF_IRQ_ALL(ch);
                val_irqen = LPAIF_IRQ_ALL(ch);
            } else if is_between(dai_id, LPASS_CDC_DMA_RX0, LPASS_CDC_DMA_RX9) || is_between(dai_id, LPASS_CDC_DMA_TX0, LPASS_CDC_DMA_TX8) {
                ret = regmap_fields_write((*dmactl).dyncclk, id, LPAIF_DMACTL_DYNCLK_ON);
                if ret != 0 {
                    dev_err((*soc_runtime).dev, b"error writing to rdmactl reg field: %d\n\0".as_ptr() as *const c_char, ret);
                    return ret;
                }
                reg_irqclr = LPAIF_RXTX_IRQCLEAR_REG(v, LPAIF_IRQ_PORT_HOST);
                val_irqclr = LPAIF_IRQ_ALL(ch);
                reg_irqen = LPAIF_RXTX_IRQEN_REG(v, LPAIF_IRQ_PORT_HOST);
                val_mask = LPAIF_IRQ_ALL(ch);
                val_irqen = LPAIF_IRQ_ALL(ch);
            } else if is_between(dai_id, LPASS_CDC_DMA_VA_TX0, LPASS_CDC_DMA_VA_TX8) {
                ret = regmap_fields_write((*dmactl).dyncclk, id, LPAIF_DMACTL_DYNCLK_ON);
                if ret != 0 {
                    dev_err((*soc_runtime).dev, b"error writing to rdmactl reg field: %d\n\0".as_ptr() as *const c_char, ret);
                    return ret;
                }
                reg_irqclr = LPAIF_VA_IRQCLEAR_REG(v, LPAIF_IRQ_PORT_HOST);
                val_irqclr = LPAIF_IRQ_ALL(ch);
                reg_irqen = LPAIF_VA_IRQEN_REG(v, LPAIF_IRQ_PORT_HOST);
                val_mask = LPAIF_IRQ_ALL(ch);
                val_irqen = LPAIF_IRQ_ALL(ch);
            } else {
                dev_err((*soc_runtime).dev, b"%s: invalid %d interface\n\0".as_ptr() as *const c_char, b"lpass_platform_pcmops_trigger\0".as_ptr(), dai_id);
                return -EINVAL;
            }

            ret = regmap_write_bits(map, reg_irqclr, val_irqclr, val_irqclr);
            if ret != 0 {
                dev_err((*soc_runtime).dev, b"error writing to irqclear reg: %d\n\0".as_ptr() as *const c_char, ret);
                return ret;
            }
            ret = regmap_update_bits(map, reg_irqen, val_mask, val_irqen);
            if ret != 0 {
                dev_err((*soc_runtime).dev, b"error writing to irqen reg: %d\n\0".as_ptr() as *const c_char, ret);
                return ret;
            }
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            ret = regmap_fields_write((*dmactl).enable, id, LPAIF_DMACTL_ENABLE_OFF);
            if ret != 0 {
                dev_err((*soc_runtime).dev, b"error writing to rdmactl reg: %d\n\0".as_ptr() as *const c_char, ret);
                return ret;
            }
            if dai_id == LPASS_DP_RX {
                ret = regmap_fields_write((*dmactl).dyncclk, id, LPAIF_DMACTL_DYNCLK_OFF);
                if ret != 0 {
                    dev_err((*soc_runtime).dev, b"error writing to rdmactl reg: %d\n\0".as_ptr() as *const c_char, ret);
                    return ret;
                }
                reg_irqen = LPASS_HDMITX_APP_IRQEN_REG(v);
                val_mask = LPAIF_IRQ_ALL(ch) | LPAIF_IRQ_HDMI_REQ_ON_PRELOAD(ch) | LPAIF_IRQ_HDMI_METADONE | LPAIF_IRQ_HDMI_SDEEP_AUD_DIS(ch);
                val_irqen = 0;
            } else if is_mi2s_port(dai_id) {
                reg_irqen = LPAIF_IRQEN_REG(v, LPAIF_IRQ_PORT_HOST);
                val_mask = LPAIF_IRQ_ALL(ch);
                val_irqen = 0;
            } else if is_between(dai_id, LPASS_CDC_DMA_RX0, LPASS_CDC_DMA_RX9) || is_between(dai_id, LPASS_CDC_DMA_TX0, LPASS_CDC_DMA_TX8) {
                ret = regmap_fields_write((*dmactl).dyncclk, id, LPAIF_DMACTL_DYNCLK_OFF);
                if ret != 0 {
                    dev_err((*soc_runtime).dev, b"error writing to rdmactl reg field: %d\n\0".as_ptr() as *const c_char, ret);
                    return ret;
                }
                reg_irqclr = LPAIF_RXTX_IRQCLEAR_REG(v, LPAIF_IRQ_PORT_HOST);
                val_irqclr = LPAIF_IRQ_ALL(ch);
                reg_irqen = LPAIF_RXTX_IRQEN_REG(v, LPAIF_IRQ_PORT_HOST);
                val_mask = LPAIF_IRQ_ALL(ch);
                val_irqen = LPAIF_IRQ_ALL(ch);
            } else if is_between(dai_id, LPASS_CDC_DMA_VA_TX0, LPASS_CDC_DMA_VA_TX8) {
                ret = regmap_fields_write((*dmactl).dyncclk, id, LPAIF_DMACTL_DYNCLK_OFF);
                if ret != 0 {
                    dev_err((*soc_runtime).dev, b"error writing to rdmactl reg field: %d\n\0".as_ptr() as *const c_char, ret);
                    return ret;
                }
                reg_irqclr = LPAIF_VA_IRQCLEAR_REG(v, LPAIF_IRQ_PORT_HOST);
                val_irqclr = LPAIF_IRQ_ALL(ch);
                reg_irqen = LPAIF_VA_IRQEN_REG(v, LPAIF_IRQ_PORT_HOST);
                val_mask = LPAIF_IRQ_ALL(ch);
                val_irqen = LPAIF_IRQ_ALL(ch);
            } else {
                dev_err((*soc_runtime).dev, b"%s: invalid %d interface\n\0".as_ptr() as *const c_char, b"lpass_platform_pcmops_trigger\0".as_ptr(), dai_id);
                return -EINVAL;
            }

            ret = regmap_update_bits(map, reg_irqen, val_mask, val_irqen);
            if ret != 0 {
                dev_err((*soc_runtime).dev, b"error writing to irqen reg: %d\n\0".as_ptr() as *const c_char, ret);
                return ret;
            }
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn lpass_platform_pcmops_pointer(component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let soc_runtime = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(soc_runtime, 0);
    let drvdata = snd_soc_component_get_drvdata(component) as *mut lpass_data;
    let rt = (*substream).runtime;
    let pcm_data = (*rt).private_data as *mut lpass_pcm_data;
    let v = (*drvdata).variant;
    let dir = (*substream).stream;
    let dai_id = (*(*cpu_dai).driver).id;
    let map = __lpass_get_regmap_handle(substream, component);
    let ch = (*pcm_data).dma_ch;
    let mut base_addr: u32 = 0;
    let mut curr_addr: u32 = 0;
    let mut ret = regmap_read(map, LPAIF_DMABASE_REG(v, ch, dir, dai_id), &mut base_addr);
    if ret != 0 {
        dev_err((*soc_runtime).dev, b"error reading from rdmabase reg: %d\n\0".as_ptr() as *const c_char, ret);
        return ret as snd_pcm_uframes_t;
    }

    ret = regmap_read(map, LPAIF_DMACURR_REG(v, ch, dir, dai_id), &mut curr_addr);
    if ret != 0 {
        dev_err((*soc_runtime).dev, b"error reading from rdmacurr reg: %d\n\0".as_ptr() as *const c_char, ret);
        return ret as snd_pcm_uframes_t;
    }

    bytes_to_frames((*substream).runtime, curr_addr.wrapping_sub(base_addr))
}

unsafe extern "C" fn lpass_platform_cdc_dma_mmap(substream: *mut snd_pcm_substream, vma: *mut vm_area_struct) -> c_int {
    let runtime = (*substream).runtime;
    (*vma).vm_page_prot = pgprot_writecombine((*vma).vm_page_prot);
    let size = (*vma).vm_end - (*vma).vm_start;
    let offset = (*vma).vm_pgoff << PAGE_SHIFT;
    io_remap_pfn_range(vma, (*vma).vm_start, ((*runtime).dma_addr + offset as usize) as c_ulong >> PAGE_SHIFT, size, (*vma).vm_page_prot)
}

unsafe extern "C" fn lpass_platform_pcmops_mmap(_component: *mut snd_soc_component, substream: *mut snd_pcm_substream, vma: *mut vm_area_struct) -> c_int {
    let soc_runtime = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(soc_runtime, 0);
    let dai_id = (*(*cpu_dai).driver).id;

    if is_cdc_dma_port(dai_id) {
        return lpass_platform_cdc_dma_mmap(substream, vma);
    }

    snd_pcm_lib_default_mmap(substream, vma)
}

unsafe extern "C" fn lpass_dma_interrupt_handler(substream: *mut snd_pcm_substream, drvdata: *mut lpass_data, chan: c_int, interrupts: u32) -> irqreturn_t {
    let soc_runtime = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(soc_runtime, 0);
    let v = (*drvdata).variant;
    let mut ret: irqreturn_t = IRQ_NONE;
    let dai_id = (*(*cpu_dai).driver).id;
    let mask = LPAIF_IRQ_ALL(chan);
    let (map, reg, val) = if dai_id == LPASS_DP_RX {
        ((*drvdata).hdmiif_map, LPASS_HDMITX_APP_IRQCLEAR_REG(v), LPAIF_IRQ_HDMI_REQ_ON_PRELOAD(chan) | LPAIF_IRQ_HDMI_METADONE | LPAIF_IRQ_HDMI_SDEEP_AUD_DIS(chan))
    } else if is_mi2s_port(dai_id) {
        ((*drvdata).lpaif_map, LPAIF_IRQCLEAR_REG(v, LPAIF_IRQ_PORT_HOST), 0)
    } else if is_between(dai_id, LPASS_CDC_DMA_RX0, LPASS_CDC_DMA_RX9) || is_between(dai_id, LPASS_CDC_DMA_TX0, LPASS_CDC_DMA_TX8) {
        ((*drvdata).rxtx_lpaif_map, LPAIF_RXTX_IRQCLEAR_REG(v, LPAIF_IRQ_PORT_HOST), 0)
    } else if is_between(dai_id, LPASS_CDC_DMA_VA_TX0, LPASS_CDC_DMA_VA_TX8) {
        ((*drvdata).va_lpaif_map, LPAIF_VA_IRQCLEAR_REG(v, LPAIF_IRQ_PORT_HOST), 0)
    } else {
        dev_err((*soc_runtime).dev, b"%s: invalid  %d interface\n\0".as_ptr() as *const c_char, b"lpass_dma_interrupt_handler\0".as_ptr(), dai_id);
        return -EINVAL;
    };

    if interrupts & LPAIF_IRQ_PER(chan) != 0 {
        let rv = regmap_write_bits(map, reg, mask, LPAIF_IRQ_PER(chan) | val);
        if rv != 0 {
            dev_err((*soc_runtime).dev, b"error writing to irqclear reg: %d\n\0".as_ptr() as *const c_char, rv);
            return IRQ_NONE;
        }
        snd_pcm_period_elapsed(substream);
        ret = IRQ_HANDLED;
    }

    if interrupts & LPAIF_IRQ_XRUN(chan) != 0 {
        let rv = regmap_write_bits(map, reg, mask, LPAIF_IRQ_XRUN(chan) | val);
        if rv != 0 {
            dev_err((*soc_runtime).dev, b"error writing to irqclear reg: %d\n\0".as_ptr() as *const c_char, rv);
            return IRQ_NONE;
        }
        dev_warn_ratelimited((*soc_runtime).dev, b"xrun warning\n\0".as_ptr() as *const c_char);
        snd_pcm_stop_xrun(substream);
        ret = IRQ_HANDLED;
    }

    if interrupts & LPAIF_IRQ_ERR(chan) != 0 {
        let rv = regmap_write_bits(map, reg, mask, LPAIF_IRQ_ERR(chan) | val);
        if rv != 0 {
            dev_err((*soc_runtime).dev, b"error writing to irqclear reg: %d\n\0".as_ptr() as *const c_char, rv);
            return IRQ_NONE;
        }
        dev_err((*soc_runtime).dev, b"bus access error\n\0".as_ptr() as *const c_char);
        snd_pcm_stop(substream, SNDRV_PCM_STATE_DISCONNECTED);
        ret = IRQ_HANDLED;
    }

    if interrupts & val != 0 {
        let rv = regmap_write(map, reg, val);
        if rv != 0 {
            dev_err((*soc_runtime).dev, b"error writing to irqclear reg: %d\n\0".as_ptr() as *const c_char, rv);
            return IRQ_NONE;
        }
        ret = IRQ_HANDLED;
    }

    ret
}

unsafe extern "C" fn lpass_platform_lpaif_irq(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let drvdata = data as *mut lpass_data;
    let v = (*drvdata).variant;
    let mut irqs: u32 = 0;
    let mut rv = regmap_read((*drvdata).lpaif_map, LPAIF_IRQSTAT_REG(v, LPAIF_IRQ_PORT_HOST), &mut irqs);
    if rv != 0 {
        pr_err(b"error reading from irqstat reg: %d\n\0".as_ptr() as *const c_char, rv);
        return IRQ_NONE;
    }

    /* Handle per channel interrupts */
    let mut chan = 0;
    while chan < LPASS_MAX_DMA_CHANNELS {
        if irqs & LPAIF_IRQ_ALL(chan) != 0 && !(*drvdata).substream[chan as usize].is_null() {
            rv = lpass_dma_interrupt_handler((*drvdata).substream[chan as usize], drvdata, chan, irqs);
            if rv != IRQ_HANDLED {
                return rv;
            }
        }
        chan += 1;
    }

    IRQ_HANDLED
}

unsafe extern "C" fn lpass_platform_hdmiif_irq(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let drvdata = data as *mut lpass_data;
    let v = (*drvdata).variant;
    let mut irqs: u32 = 0;
    let mut rv = regmap_read((*drvdata).hdmiif_map, LPASS_HDMITX_APP_IRQSTAT_REG(v), &mut irqs);
    if rv != 0 {
        pr_err(b"error reading from irqstat reg: %d\n\0".as_ptr() as *const c_char, rv);
        return IRQ_NONE;
    }

    /* Handle per channel interrupts */
    let mut chan = 0;
    while chan < LPASS_MAX_HDMI_DMA_CHANNELS {
        if irqs & (LPAIF_IRQ_ALL(chan) | LPAIF_IRQ_HDMI_REQ_ON_PRELOAD(chan) | LPAIF_IRQ_HDMI_METADONE | LPAIF_IRQ_HDMI_SDEEP_AUD_DIS(chan)) != 0
            && !(*drvdata).hdmi_substream[chan as usize].is_null() {
            rv = lpass_dma_interrupt_handler((*drvdata).hdmi_substream[chan as usize], drvdata, chan, irqs);
            if rv != IRQ_HANDLED {
                return rv;
            }
        }
        chan += 1;
    }
    IRQ_HANDLED
}

unsafe extern "C" fn lpass_platform_rxtxif_irq(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let drvdata = data as *mut lpass_data;
    let v = (*drvdata).variant;
    let mut irqs: u32 = 0;
    let mut rv = regmap_read((*drvdata).rxtx_lpaif_map, LPAIF_RXTX_IRQSTAT_REG(v, LPAIF_IRQ_PORT_HOST), &mut irqs);

    /* Handle per channel interrupts */
    let mut chan = 0;
    while chan < LPASS_MAX_CDC_DMA_CHANNELS {
        if irqs & LPAIF_IRQ_ALL(chan) != 0 && !(*drvdata).rxtx_substream[chan as usize].is_null() {
            rv = lpass_dma_interrupt_handler((*drvdata).rxtx_substream[chan as usize], drvdata, chan, irqs);
            if rv != IRQ_HANDLED {
                return rv;
            }
        }
        chan += 1;
    }

    IRQ_HANDLED
}

unsafe extern "C" fn lpass_platform_vaif_irq(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let drvdata = data as *mut lpass_data;
    let v = (*drvdata).variant;
    let mut irqs: u32 = 0;
    let mut rv = regmap_read((*drvdata).va_lpaif_map, LPAIF_VA_IRQSTAT_REG(v, LPAIF_IRQ_PORT_HOST), &mut irqs);

    /* Handle per channel interrupts */
    let mut chan = 0;
    while chan < LPASS_MAX_VA_CDC_DMA_CHANNELS {
        if irqs & LPAIF_IRQ_ALL(chan) != 0 && !(*drvdata).va_substream[chan as usize].is_null() {
            rv = lpass_dma_interrupt_handler((*drvdata).va_substream[chan as usize], drvdata, chan, irqs);
            if rv != IRQ_HANDLED {
                return rv;
            }
        }
        chan += 1;
    }
    IRQ_HANDLED
}

unsafe extern "C" fn lpass_platform_prealloc_cdc_dma_buffer(component: *mut snd_soc_component, pcm: *mut snd_pcm, dai_id: c_int) -> c_int {
    let drvdata = snd_soc_component_get_drvdata(component) as *mut lpass_data;
    let substream = if !(*pcm).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].substream.is_null() {
        (*pcm).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].substream
    } else {
        (*pcm).streams[SNDRV_PCM_STREAM_CAPTURE as usize].substream
    };

    let buf = &mut (*substream).dma_buffer as *mut snd_dma_buffer;
    (*buf).dev.dev = (*(*pcm).card).dev;
    (*buf).private_data = ptr::null_mut();

    /* Assign Codec DMA buffer pointers */
    (*buf).dev.type_ = SNDRV_DMA_TYPE_CONTINUOUS;

    if is_between(dai_id as u32, LPASS_CDC_DMA_RX0, LPASS_CDC_DMA_RX9) {
        (*buf).bytes = lpass_platform_rxtx_hardware.buffer_bytes_max;
        (*buf).addr = (*drvdata).rxtx_cdc_dma_lpm_buf;
    } else if is_between(dai_id as u32, LPASS_CDC_DMA_TX0, LPASS_CDC_DMA_TX8) {
        (*buf).bytes = lpass_platform_rxtx_hardware.buffer_bytes_max;
        (*buf).addr = (*drvdata).rxtx_cdc_dma_lpm_buf + LPASS_RXTX_CDC_DMA_LPM_BUFF_SIZE;
    } else if is_between(dai_id as u32, LPASS_CDC_DMA_VA_TX0, LPASS_CDC_DMA_VA_TX8) {
        (*buf).bytes = lpass_platform_va_hardware.buffer_bytes_max;
        (*buf).addr = (*drvdata).va_cdc_dma_lpm_buf;
    }

    (*buf).area = memremap((*buf).addr, (*buf).bytes, MEMREMAP_WC) as *mut u8;

    0
}

unsafe extern "C" fn lpass_platform_pcm_new(component: *mut snd_soc_component, soc_runtime: *mut snd_soc_pcm_runtime) -> c_int {
    let pcm = (*soc_runtime).pcm;
    let cpu_dai = snd_soc_rtd_to_cpu(soc_runtime, 0);
    let dai_id = (*(*cpu_dai).driver).id;
    let size: size_t = lpass_platform_pcm_hardware.buffer_bytes_max;

    /*
     * Lpass codec dma can access only lpass lpm hardware memory.
     * ioremap is for HLOS to access hardware memory.
     */
    if is_cdc_dma_port(dai_id) {
        return lpass_platform_prealloc_cdc_dma_buffer(component, pcm, dai_id as c_int);
    }

    snd_pcm_set_fixed_buffer_all(pcm, SNDRV_DMA_TYPE_NONCOHERENT, (*component).dev, size)
}

unsafe extern "C" fn lpass_platform_pcmops_suspend(component: *mut snd_soc_component) -> c_int {
    let drvdata = snd_soc_component_get_drvdata(component) as *mut lpass_data;
    let mut map: *mut regmap;

    if (*drvdata).hdmi_port_enable {
        map = (*drvdata).hdmiif_map;
        regcache_cache_only(map, true);
        regcache_mark_dirty(map);
    }

    map = (*drvdata).lpaif_map;
    regcache_cache_only(map, true);
    regcache_mark_dirty(map);

    0
}

unsafe extern "C" fn lpass_platform_pcmops_resume(component: *mut snd_soc_component) -> c_int {
    let drvdata = snd_soc_component_get_drvdata(component) as *mut lpass_data;
    let mut map: *mut regmap;
    let mut ret: c_int;

    if (*drvdata).hdmi_port_enable {
        map = (*drvdata).hdmiif_map;
        regcache_cache_only(map, false);
        ret = regcache_sync(map);
        if ret != 0 {
            return ret;
        }
    }

    map = (*drvdata).lpaif_map;
    regcache_cache_only(map, false);

    regcache_sync(map)
}

unsafe extern "C" fn lpass_platform_copy(_component: *mut snd_soc_component, substream: *mut snd_pcm_substream, channel: c_int, pos: c_ulong, buf: *mut iov_iter, bytes: c_ulong) -> c_int {
    let rt = (*substream).runtime;
    let soc_runtime = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(soc_runtime, 0);
    let dai_id = (*(*cpu_dai).driver).id;
    let mut ret = 0;
    let dma_buf = (*rt).dma_area.add(pos as usize + channel as usize * ((*rt).dma_bytes / (*rt).channels as usize)) as *mut c_void;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        if is_cdc_dma_port(dai_id) {
            if copy_from_iter_toio(dma_buf, bytes, buf) != bytes {
                ret = -EFAULT;
            }
        } else if copy_from_iter(dma_buf, bytes, buf) != bytes {
            ret = -EFAULT;
        }
    } else if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        if is_cdc_dma_port(dai_id) {
            if copy_to_iter_fromio(dma_buf, bytes, buf) != bytes {
                ret = -EFAULT;
            }
        } else if copy_to_iter(dma_buf, bytes, buf) != bytes {
            ret = -EFAULT;
        }
    }

    ret
}

static lpass_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    name: DRV_NAME.as_ptr() as *const c_char,
    open: Some(lpass_platform_pcmops_open),
    close: Some(lpass_platform_pcmops_close),
    hw_params: Some(lpass_platform_pcmops_hw_params),
    hw_free: Some(lpass_platform_pcmops_hw_free),
    prepare: Some(lpass_platform_pcmops_prepare),
    trigger: Some(lpass_platform_pcmops_trigger),
    pointer: Some(lpass_platform_pcmops_pointer),
    mmap: Some(lpass_platform_pcmops_mmap),
    pcm_new: Some(lpass_platform_pcm_new),
    suspend: Some(lpass_platform_pcmops_suspend),
    resume: Some(lpass_platform_pcmops_resume),
    copy: Some(lpass_platform_copy),
};

#[no_mangle]
pub unsafe extern "C" fn asoc_qcom_lpass_platform_register(pdev: *mut platform_device) -> c_int {
    let drvdata = platform_get_drvdata(pdev) as *mut lpass_data;
    let v = (*drvdata).variant;
    let mut ret: c_int;

    (*drvdata).lpaif_irq = platform_get_irq_byname(pdev, b"lpass-irq-lpaif\0".as_ptr() as *const c_char);
    if (*drvdata).lpaif_irq < 0 {
        return -ENODEV;
    }

    /* ensure audio hardware is disabled */
    ret = regmap_write((*drvdata).lpaif_map, LPAIF_IRQEN_REG(v, LPAIF_IRQ_PORT_HOST), 0);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, b"error writing to irqen reg: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = devm_request_irq(&mut (*pdev).dev, (*drvdata).lpaif_irq, lpass_platform_lpaif_irq, IRQF_TRIGGER_RISING, b"lpass-irq-lpaif\0".as_ptr() as *const c_char, drvdata as *mut c_void);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, b"irq request failed: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = lpass_platform_alloc_dmactl_fields(&mut (*pdev).dev, (*drvdata).lpaif_map);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, b"error initializing dmactl fields: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    if (*drvdata).codec_dma_enable {
        ret = regmap_write((*drvdata).rxtx_lpaif_map, LPAIF_RXTX_IRQEN_REG(v, LPAIF_IRQ_PORT_HOST), 0x0);
        if ret != 0 {
            dev_err(&mut (*pdev).dev, b"error writing to rxtx irqen reg: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
        ret = regmap_write((*drvdata).va_lpaif_map, LPAIF_VA_IRQEN_REG(v, LPAIF_IRQ_PORT_HOST), 0x0);
        if ret != 0 {
            dev_err(&mut (*pdev).dev, b"error writing to rxtx irqen reg: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
        (*drvdata).rxtxif_irq = platform_get_irq_byname(pdev, b"lpass-irq-rxtxif\0".as_ptr() as *const c_char);
        if (*drvdata).rxtxif_irq < 0 {
            return -ENODEV;
        }

        ret = devm_request_irq(&mut (*pdev).dev, (*drvdata).rxtxif_irq, lpass_platform_rxtxif_irq, 0, b"lpass-irq-rxtxif\0".as_ptr() as *const c_char, drvdata as *mut c_void);
        if ret != 0 {
            dev_err(&mut (*pdev).dev, b"rxtx irq request failed: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }

        ret = lpass_platform_alloc_rxtx_dmactl_fields(&mut (*pdev).dev, (*drvdata).rxtx_lpaif_map);
        if ret != 0 {
            dev_err(&mut (*pdev).dev, b"error initializing rxtx dmactl fields: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }

        (*drvdata).vaif_irq = platform_get_irq_byname(pdev, b"lpass-irq-vaif\0".as_ptr() as *const c_char);
        if (*drvdata).vaif_irq < 0 {
            return -ENODEV;
        }

        ret = devm_request_irq(&mut (*pdev).dev, (*drvdata).vaif_irq, lpass_platform_vaif_irq, 0, b"lpass-irq-vaif\0".as_ptr() as *const c_char, drvdata as *mut c_void);
        if ret != 0 {
            dev_err(&mut (*pdev).dev, b"va irq request failed: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }

        ret = lpass_platform_alloc_va_dmactl_fields(&mut (*pdev).dev, (*drvdata).va_lpaif_map);
        if ret != 0 {
            dev_err(&mut (*pdev).dev, b"error initializing va dmactl fields: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
    }

    if (*drvdata).hdmi_port_enable {
        (*drvdata).hdmiif_irq = platform_get_irq_byname(pdev, b"lpass-irq-hdmi\0".as_ptr() as *const c_char);
        if (*drvdata).hdmiif_irq < 0 {
            return -ENODEV;
        }

        ret = devm_request_irq(&mut (*pdev).dev, (*drvdata).hdmiif_irq, lpass_platform_hdmiif_irq, 0, b"lpass-irq-hdmi\0".as_ptr() as *const c_char, drvdata as *mut c_void);
        if ret != 0 {
            dev_err(&mut (*pdev).dev, b"irq hdmi request failed: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
        ret = regmap_write((*drvdata).hdmiif_map, LPASS_HDMITX_APP_IRQEN_REG(v), 0);
        if ret != 0 {
            dev_err(&mut (*pdev).dev, b"error writing to hdmi irqen reg: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }

        ret = lpass_platform_alloc_hdmidmactl_fields(&mut (*pdev).dev, (*drvdata).hdmiif_map);
        if ret != 0 {
            dev_err(&mut (*pdev).dev, b"error initializing hdmidmactl fields: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
    }
    devm_snd_soc_register_component(&mut (*pdev).dev, &lpass_component_driver, ptr::null_mut(), 0)
}

/* EXPORT_SYMBOL_GPL(asoc_qcom_lpass_platform_register); */

/* MODULE_DESCRIPTION("QTi LPASS Platform Driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
