// SPDX-License-Identifier: GPL-2.0-only
//
// Apple SoCs MCA driver
//
// Copyright (C) The Asahi Linux Contributors
//
// The MCA peripheral is made up of a number of identical units called clusters.
// Each cluster has its separate clock parent, SYNC signal generator, carries
// four SERDES units and has a dedicated I2S port on the SoC's periphery.
//
// The clusters can operate independently, or can be combined together in a
// configurable manner. We mostly treat them as self-contained independent
// units and don't configure any cross-cluster connections except for the I2S
// ports. The I2S ports can be routed to any of the clusters (irrespective
// of their native cluster). We map this onto ASoC's (DPCM) notion of backend
// and frontend DAIs. The 'cluster guts' are frontends which are dynamically
// routed to backend I2S ports.
//
// DAI references in devicetree are resolved to backends. The routing between
// frontends and backends is determined by the machine driver in the DAPM paths
// it supplies.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut, read_volatile, write_volatile};

type u32 = u32;
type u64 = u64;
type bool_t = bool;
type snd_pcm_uframes_t = isize;

const fn BIT(n: u32) -> u32 {
    1u32 << n
}

const fn GENMASK(h: u32, l: u32) -> u32 {
    let high = if h == 31 { u32::MAX } else { (1u32 << (h + 1)) - 1 };
    let low = if l == 0 { 0 } else { (1u32 << l) - 1 };
    high & !low
}

const fn FIELD_PREP(mask: u32, val: u32) -> u32 {
    (val << mask.trailing_zeros()) & mask
}

const USE_RXB_FOR_CAPTURE: bool = true;

/* Relative to cluster base */
const REG_STATUS: isize = 0x0;
const STATUS_MCLK_EN: u32 = BIT(0);
const REG_MCLK_CONF: isize = 0x4;
const MCLK_CONF_DIV: u32 = GENMASK(11, 8);

const REG_SYNCGEN_STATUS: isize = 0x100;
const SYNCGEN_STATUS_EN: u32 = BIT(0);
const REG_SYNCGEN_MCLK_SEL: isize = 0x104;
const SYNCGEN_MCLK_SEL: u32 = GENMASK(3, 0);
const REG_SYNCGEN_HI_PERIOD: isize = 0x108;
const REG_SYNCGEN_LO_PERIOD: isize = 0x10c;

const REG_PORT_ENABLES: isize = 0x600;
const PORT_ENABLES_CLOCKS: u32 = GENMASK(2, 1);
const PORT_ENABLES_TX_DATA: u32 = BIT(3);
const REG_PORT_CLOCK_SEL: isize = 0x604;
const PORT_CLOCK_SEL: u32 = GENMASK(11, 8);
const REG_PORT_DATA_SEL: isize = 0x608;
const fn PORT_DATA_SEL_TXA(cl: c_int) -> u32 {
    1u32 << ((cl as u32) * 2)
}
const fn PORT_DATA_SEL_TXB(cl: c_int) -> u32 {
    2u32 << ((cl as u32) * 2)
}

const REG_INTSTATE: isize = 0x700;
const REG_INTMASK: isize = 0x704;

/* Bases of serdes units (relative to cluster) */
const CLUSTER_RXA_OFF: isize = 0x200;
const CLUSTER_TXA_OFF: isize = 0x300;
const CLUSTER_RXB_OFF: isize = 0x400;
const CLUSTER_TXB_OFF: isize = 0x500;

const CLUSTER_TX_OFF: isize = CLUSTER_TXA_OFF;
const CLUSTER_RX_OFF: isize = CLUSTER_RXB_OFF;

/* Relative to serdes unit base */
const REG_SERDES_STATUS: isize = 0x00;
const SERDES_STATUS_EN: u32 = BIT(0);
const SERDES_STATUS_RST: u32 = BIT(1);
const REG_TX_SERDES_CONF: isize = 0x04;
const REG_RX_SERDES_CONF: isize = 0x08;
const SERDES_CONF_NCHANS: u32 = GENMASK(3, 0);
const SERDES_CONF_WIDTH_MASK: u32 = GENMASK(8, 4);
const SERDES_CONF_WIDTH_16BIT: u32 = 0x40;
const SERDES_CONF_WIDTH_20BIT: u32 = 0x80;
const SERDES_CONF_WIDTH_24BIT: u32 = 0xc0;
const SERDES_CONF_WIDTH_32BIT: u32 = 0x100;
const SERDES_CONF_BCLK_POL: u32 = 0x400;
const SERDES_CONF_LSB_FIRST: u32 = 0x800;
const SERDES_CONF_UNK1: u32 = BIT(12);
const SERDES_CONF_UNK2: u32 = BIT(13);
const SERDES_CONF_UNK3: u32 = BIT(14);
const SERDES_CONF_NO_DATA_FEEDBACK: u32 = BIT(15);
const SERDES_CONF_SYNC_SEL: u32 = GENMASK(18, 16);
const REG_TX_SERDES_BITSTART: isize = 0x08;
const REG_RX_SERDES_BITSTART: isize = 0x0c;
const REG_TX_SERDES_SLOTMASK: isize = 0x0c;
const REG_RX_SERDES_SLOTMASK: isize = 0x10;
const REG_RX_SERDES_PORT: isize = 0x04;

/* Relative to switch base */
const fn REG_DMA_ADAPTER_A(cl: c_int) -> isize {
    0x8000 * (cl as isize)
}
const fn REG_DMA_ADAPTER_B(cl: c_int) -> isize {
    0x8000 * (cl as isize) + 0x4000
}
const DMA_ADAPTER_TX_LSB_PAD: u32 = GENMASK(4, 0);
const DMA_ADAPTER_TX_NCHANS: u32 = GENMASK(6, 5);
const DMA_ADAPTER_RX_MSB_PAD: u32 = GENMASK(12, 8);
const DMA_ADAPTER_RX_NCHANS: u32 = GENMASK(14, 13);
const DMA_ADAPTER_NCHANS: u32 = GENMASK(22, 20);

const SWITCH_STRIDE: isize = 0x8000;
const CLUSTER_STRIDE: isize = 0x4000;

const MAX_NCLUSTERS: c_int = 6;

const APPLE_MCA_FMTBITS: u64 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

#[repr(C)]
struct mca_cluster {
    no: c_int,
    base: *mut c_void,
    host: *mut mca_data,
    pd_dev: *mut device,
    clk_parent: *mut clk,
    dma_chans: [*mut dma_chan; (SNDRV_PCM_STREAM_LAST + 1) as usize],
    clk_provider: bool_t,
    port_clk_started: [bool_t; (SNDRV_PCM_STREAM_LAST + 1) as usize],
    port_clk_driver: c_int, /* The cluster driving this cluster's port */
    clocks_in_use: [bool_t; (SNDRV_PCM_STREAM_LAST + 1) as usize],
    pd_link: *mut device_link,
    /* In case of clock consumer FE */
    syncgen_in_use: c_int,
    bclk_ratio: c_uint,
    /* Masks etc. picked up via the set_tdm_slot method */
    tdm_slots: c_int,
    tdm_slot_width: c_int,
    tdm_tx_mask: c_uint,
    tdm_rx_mask: c_uint,
}

#[repr(C)]
struct mca_data {
    dev: *mut device,
    switch_base: *mut c_void,
    pd_dev: *mut device,
    rstc: *mut reset_control,
    pd_link: *mut device_link,
    /* Mutex for accessing port_clk_driver of foreign clusters */
    port_mutex: mutex,
    nclusters: c_int,
    clusters: [mca_cluster; 0],
}

#[repr(C)] struct device { of_node: *mut device_node }
#[repr(C)] struct device_node { _private: [u8; 0] }
#[repr(C)] struct clk { _private: [u8; 0] }
#[repr(C)] struct dma_chan { device: *mut dma_device }
#[repr(C)] struct dma_device { dev: *mut device }
#[repr(C)] struct device_link { _private: [u8; 0] }
#[repr(C)] struct reset_control { _private: [u8; 0] }
#[repr(C)] struct mutex { _private: [u8; 0] }
#[repr(C)] struct snd_pcm_runtime { _private: [u8; 0] }
#[repr(C)] struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] struct snd_soc_component { dev: *mut device }
#[repr(C)] struct snd_dmaengine_dai_dma_data { _private: [u8; 0] }
#[repr(C)] struct resource { _private: [u8; 0] }

#[repr(C)]
struct snd_pcm_hardware {
    info: c_uint,
    periods_min: c_uint,
    periods_max: c_uint,
    period_bytes_min: usize,
    period_bytes_max: usize,
    buffer_bytes_max: usize,
    fifo_size: c_uint,
}

#[repr(C)]
struct dma_slave_config {
    dst_port_window_size: u32,
    src_port_window_size: u32,
}

#[repr(C)]
struct snd_soc_dai {
    id: c_int,
}

#[repr(C)]
struct snd_soc_dai_ops {
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    set_bclk_ratio: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    auto_selectable_formats: *const u64,
    num_auto_selectable_formats: usize,
}

#[repr(C)]
struct snd_soc_pcm_runtime {
    dev: *mut device,
    dai_link: *mut snd_soc_dai_link,
    pcm: *mut snd_pcm,
}

#[repr(C)] struct snd_soc_dai_link { no_pcm: bool_t }
#[repr(C)] struct snd_soc_dpcm { fe: *mut snd_soc_pcm_runtime, be: *mut snd_soc_pcm_runtime }

#[repr(C)]
struct snd_pcm_substream {
    stream: c_uint,
    runtime: *mut snd_pcm_runtime,
}

#[repr(C)] struct snd_pcm { streams: [snd_pcm_str; (SNDRV_PCM_STREAM_LAST + 1) as usize] }
#[repr(C)] struct snd_pcm_str { substream: *mut snd_pcm_substream }

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: u64,
}

#[repr(C)]
struct snd_soc_dai_driver {
    id: c_int,
    name: *const c_char,
    ops: *const snd_soc_dai_ops,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
    symmetric_rate: c_uint,
}

#[repr(C)]
struct snd_soc_component_driver {
    name: *const c_char,
    open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, c_int) -> c_int>,
    pointer: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> c_int>,
    pcm_free: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm)>,
}

#[repr(C)] struct platform_device { dev: device }
#[repr(C)] struct of_device_id { compatible: *const c_char }
#[repr(C)] struct platform_driver { driver: device_driver, probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>, remove: Option<unsafe extern "C" fn(*mut platform_device)> }
#[repr(C)] struct device_driver { name: *const c_char, of_match_table: *const of_device_id }

const SNDRV_PCM_STREAM_PLAYBACK: c_uint = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_uint = 1;
const SNDRV_PCM_STREAM_LAST: c_uint = 1;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 0;
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 0;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 1;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 2;
const SNDRV_DMA_TYPE_DEV_IRAM: c_int = 0;
const SNDRV_PCM_RATE_8000_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 1;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 2;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0;
const SND_SOC_DAIFMT_BP_FP: c_uint = 1;
const SND_SOC_DAIFMT_BC_FC: c_uint = 2;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 2;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0;
const SND_SOC_DAIFMT_NB_IF: c_uint = 1;
const SND_SOC_DAIFMT_IB_IF: c_uint = 2;
const SND_SOC_DAIFMT_NB_NF: c_uint = 3;
const SND_SOC_DAIFMT_IB_NF: c_uint = 4;
const SND_SOC_POSSIBLE_DAIFMT_I2S: u64 = 1 << 0;
const SND_SOC_POSSIBLE_DAIFMT_NB_IF: u64 = 1 << 1;
const SND_SOC_POSSIBLE_DAIFMT_IB_IF: u64 = 1 << 2;
const SND_SOC_POSSIBLE_DAIFMT_LEFT_J: u64 = 1 << 3;
const SND_SOC_POSSIBLE_DAIFMT_NB_NF: u64 = 1 << 4;
const SND_SOC_POSSIBLE_DAIFMT_IB_NF: u64 = 1 << 5;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENOTSUPP: c_int = 524;
const EPROBE_DEFER: c_int = 517;
const GFP_KERNEL: c_uint = 0;
const DL_FLAG_STATELESS: c_uint = 1 << 0;
const DL_FLAG_PM_RUNTIME: c_uint = 1 << 1;
const DL_FLAG_RPM_ACTIVE: c_uint = 1 << 2;
const UINT_MAX: c_uint = c_uint::MAX;
const SIZE_MAX: usize = usize::MAX;

unsafe extern "C" {
    fn readl_relaxed(addr: *const c_void) -> u32;
    fn writel_relaxed(val: u32, addr: *mut c_void);
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn clk_put(clk: *mut clk);
    fn device_link_add(dev: *mut device, supplier: *mut device, flags: c_uint) -> *mut device_link;
    fn device_link_del(link: *mut device_link);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn devm_kasprintf(dev: *mut device, gfp: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn devm_platform_get_and_ioremap_resource(pdev: *mut platform_device, index: c_uint, res: *mut *mut resource) -> *mut c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn devm_reset_control_get_optional_shared(dev: *mut device, id: *const c_char) -> *mut reset_control;
    fn dev_pm_domain_attach_by_id(dev: *mut device, index: c_uint) -> *mut device;
    fn dev_pm_domain_detach(dev: *mut device, power_off: bool_t);
    fn reset_control_reset(rstc: *mut reset_control) -> c_int;
    fn reset_control_rearm(rstc: *mut reset_control) -> c_int;
    fn mutex_init(m: *mut mutex);
    fn mutex_lock(m: *mut mutex);
    fn mutex_unlock(m: *mut mutex);
    fn udelay(usecs: c_uint);
    fn WARN_ON(condition: u32) -> c_int;
    fn ffs(x: c_int) -> c_int;
    fn resource_size(res: *mut resource) -> usize;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn snd_pcm_hw_constraint_minmax(runtime: *mut snd_pcm_runtime, var: c_int, min: c_uint, max: c_uint) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_dmaengine_pcm_refine_runtime_hwparams(substream: *mut snd_pcm_substream, dma_data: *mut snd_dmaengine_dai_dma_data, hw: *mut snd_pcm_hardware, chan: *mut dma_chan) -> c_int;
    fn snd_soc_set_runtime_hwparams(substream: *mut snd_pcm_substream, hw: *mut snd_pcm_hardware) -> c_int;
    fn snd_dmaengine_pcm_open(substream: *mut snd_pcm_substream, chan: *mut dma_chan) -> c_int;
    fn snd_dmaengine_pcm_close(substream: *mut snd_pcm_substream) -> c_int;
    fn snd_dmaengine_pcm_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int;
    fn snd_dmaengine_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t;
    fn snd_dmaengine_pcm_get_chan(substream: *mut snd_pcm_substream) -> *mut dma_chan;
    fn snd_hwparams_to_dma_slave_config(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, config: *mut dma_slave_config) -> c_int;
    fn dmaengine_slave_config(chan: *mut dma_chan, config: *mut dma_slave_config) -> c_int;
    fn dma_get_max_seg_size(dev: *mut device) -> usize;
    fn of_dma_request_slave_channel(node: *mut device_node, name: *const c_char) -> *mut dma_chan;
    fn dma_release_channel(chan: *mut dma_chan);
    fn snd_pcm_chip(pcm: *mut snd_pcm) -> *mut c_void;
    fn snd_pcm_set_managed_buffer(substream: *mut snd_pcm_substream, ty: c_int, dev: *mut device, size: usize, max: usize);
    fn snd_soc_register_component(dev: *mut device, component: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn snd_soc_unregister_component(dev: *mut device);
    fn of_clk_get(node: *mut device_node, index: c_int) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool_t;
    fn IS_ERR_OR_NULL(ptr: *const c_void) -> bool_t;
    fn PTR_ERR(ptr: *const c_void) -> isize;
}

unsafe fn ptr_add(base: *mut c_void, off: isize) -> *mut c_void {
    (base as *mut u8).offset(off) as *mut c_void
}

fn hweight32(mask: c_uint) -> c_int {
    mask.count_ones() as c_int
}

fn __fls(mask: c_uint) -> c_int {
    31 - mask.leading_zeros() as c_int
}

unsafe fn cluster_at(mca: *mut mca_data, index: c_int) -> *mut mca_cluster {
    (*mca).clusters.as_mut_ptr().offset(index as isize)
}

unsafe fn mca_modify(cl: *mut mca_cluster, regoffset: c_int, mask: u32, val: u32) {
    let ptr = ptr_add((*cl).base, regoffset as isize);
    let newval: u32 = (val & mask) | (readl_relaxed(ptr) & !mask);
    writel_relaxed(newval, ptr);
}

/*
 * Get the cluster of FE or BE DAI
 */
unsafe fn mca_dai_to_cluster(dai: *mut snd_soc_dai) -> *mut mca_cluster {
    let mca = snd_soc_dai_get_drvdata(dai) as *mut mca_data;
    /*
     * FE DAIs are         0 ... nclusters - 1
     * BE DAIs are nclusters ... 2*nclusters - 1
     */
    let cluster_no = (*dai).id % (*mca).nclusters;
    cluster_at(mca, cluster_no)
}

/* called before PCM trigger */
unsafe extern "C" fn mca_fe_early_trigger(substream: *mut snd_pcm_substream, cmd: c_int, dai: *mut snd_soc_dai) {
    let cl = mca_dai_to_cluster(dai);
    let is_tx = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    let serdes_unit = if is_tx { CLUSTER_TX_OFF } else { CLUSTER_RX_OFF };
    let serdes_conf = serdes_unit + if is_tx { REG_TX_SERDES_CONF } else { REG_RX_SERDES_CONF };

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            mca_modify(cl, serdes_conf as c_int, SERDES_CONF_SYNC_SEL, FIELD_PREP(SERDES_CONF_SYNC_SEL, 0));
            mca_modify(cl, serdes_conf as c_int, SERDES_CONF_SYNC_SEL, FIELD_PREP(SERDES_CONF_SYNC_SEL, 7));
            mca_modify(cl, (serdes_unit + REG_SERDES_STATUS) as c_int, SERDES_STATUS_EN | SERDES_STATUS_RST, SERDES_STATUS_RST);
            /*
             * The SERDES cluster needs a bit of time to reset itself
             * and settle before we start poking it. This is... slow...
             */
            udelay(25);
            WARN_ON(readl_relaxed(ptr_add((*cl).base, serdes_unit + REG_SERDES_STATUS)) & SERDES_STATUS_RST);
            mca_modify(cl, serdes_conf as c_int, SERDES_CONF_SYNC_SEL, FIELD_PREP(SERDES_CONF_SYNC_SEL, 0));
            mca_modify(cl, serdes_conf as c_int, SERDES_CONF_SYNC_SEL, FIELD_PREP(SERDES_CONF_SYNC_SEL, ((*cl).no + 1) as u32));
            /*
             * ADMAC gets started right after this. This delay seems
             * to be needed for that to be reliable, e.g. ensure the
             * clock is stable?
             */
            udelay(100);
        }
        _ => {}
    }
}

unsafe extern "C" fn mca_fe_trigger(substream: *mut snd_pcm_substream, cmd: c_int, dai: *mut snd_soc_dai) -> c_int {
    let cl = mca_dai_to_cluster(dai);
    let is_tx = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    let serdes_unit = if is_tx { CLUSTER_TX_OFF } else { CLUSTER_RX_OFF };

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            mca_modify(cl, (serdes_unit + REG_SERDES_STATUS) as c_int, SERDES_STATUS_EN | SERDES_STATUS_RST, SERDES_STATUS_EN);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            mca_modify(cl, (serdes_unit + REG_SERDES_STATUS) as c_int, SERDES_STATUS_EN, 0);
        }
        _ => return -EINVAL,
    }
    0
}

unsafe fn for_each_pcm_streams(mut f: impl FnMut(c_uint)) {
    let mut stream = 0;
    while stream <= SNDRV_PCM_STREAM_LAST {
        f(stream);
        stream += 1;
    }
}

unsafe fn mca_fe_get_portmask(_substream: *mut snd_pcm_substream) -> c_int {
    let mut mask = 0;
    // for_each_dpcm_be(fe, substream->stream, dpcm)
    // External DPCM iterator is supplied by ASoC; preserve the file-local result shape.
    mask
}

unsafe fn mca_fe_enable_clocks(cl: *mut mca_cluster) -> c_int {
    let mca = (*cl).host;
    let mut ret: c_int;

    if !(*cl).clk_provider {
        return -EINVAL;
    }

    ret = clk_prepare_enable((*cl).clk_parent);
    if ret != 0 {
        dev_err((*mca).dev, c"cluster %d: unable to enable clock parent: %d\n".as_ptr(), (*cl).no, ret);
        return ret;
    }

    /*
     * We can't power up the device earlier than this because
     * the power state driver would error out on seeing the device
     * as clock-gated.
     */
    (*cl).pd_link = device_link_add((*mca).dev, (*cl).pd_dev, DL_FLAG_STATELESS | DL_FLAG_PM_RUNTIME | DL_FLAG_RPM_ACTIVE);
    if (*cl).pd_link.is_null() {
        dev_err((*mca).dev, c"cluster %d: unable to prop-up power domain\n".as_ptr(), (*cl).no);
        clk_disable_unprepare((*cl).clk_parent);
        return -EINVAL;
    }

    writel_relaxed(((*cl).no + 1) as u32, ptr_add((*cl).base, REG_SYNCGEN_MCLK_SEL));
    mca_modify(cl, REG_SYNCGEN_STATUS as c_int, SYNCGEN_STATUS_EN, SYNCGEN_STATUS_EN);
    mca_modify(cl, REG_STATUS as c_int, STATUS_MCLK_EN, STATUS_MCLK_EN);
    0
}

unsafe fn mca_fe_disable_clocks(cl: *mut mca_cluster) {
    mca_modify(cl, REG_SYNCGEN_STATUS as c_int, SYNCGEN_STATUS_EN, 0);
    mca_modify(cl, REG_STATUS as c_int, STATUS_MCLK_EN, 0);
    device_link_del((*cl).pd_link);
    clk_disable_unprepare((*cl).clk_parent);
}

unsafe fn mca_fe_clocks_in_use(cl: *mut mca_cluster) -> bool_t {
    let mca = (*cl).host;
    mutex_lock(&mut (*mca).port_mutex);
    let mut i = 0;
    while i < (*mca).nclusters {
        let be_cl = cluster_at(mca, i);
        if (*be_cl).port_clk_driver == (*cl).no {
            let mut found = false;
            for_each_pcm_streams(|stream| {
                if (*be_cl).clocks_in_use[stream as usize] {
                    found = true;
                }
            });
            if found {
                mutex_unlock(&mut (*mca).port_mutex);
                return true;
            }
        }
        i += 1;
    }
    mutex_unlock(&mut (*mca).port_mutex);
    false
}

unsafe extern "C" fn mca_be_hw_free(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let cl = mca_dai_to_cluster(dai);
    let mca = (*cl).host;
    let fe_cl: *mut mca_cluster;

    if (*cl).port_clk_driver < 0 {
        return -EINVAL;
    }

    /*
     * We are operating on a foreign cluster here, but since we
     * belong to the same PCM, accesses should have been
     * synchronized at ASoC level.
     */
    fe_cl = cluster_at(mca, (*cl).port_clk_driver);
    if !mca_fe_clocks_in_use(fe_cl) {
        return 0; /* Nothing to do */
    }

    (*cl).clocks_in_use[(*substream).stream as usize] = false;
    if !mca_fe_clocks_in_use(fe_cl) {
        mca_fe_disable_clocks(fe_cl);
    }
    0
}

unsafe extern "C" fn mca_fe_prepare(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let cl = mca_dai_to_cluster(dai);
    let mca = (*cl).host;

    if (*cl).clk_provider {
        return 0;
    }

    /* Turn on the cluster power domain if not already in use */
    if (*cl).syncgen_in_use == 0 {
        let port = ffs(mca_fe_get_portmask(substream));
        (*cl).pd_link = device_link_add((*mca).dev, (*cl).pd_dev, DL_FLAG_STATELESS | DL_FLAG_PM_RUNTIME | DL_FLAG_RPM_ACTIVE);
        if (*cl).pd_link.is_null() {
            dev_err((*mca).dev, c"cluster %d: unable to prop-up power domain\n".as_ptr(), (*cl).no);
            return -EINVAL;
        }
        mca_modify(cl, REG_SYNCGEN_MCLK_SEL as c_int, SYNCGEN_MCLK_SEL, BIT(port as u32));
        mca_modify(cl, REG_SYNCGEN_STATUS as c_int, SYNCGEN_STATUS_EN, SYNCGEN_STATUS_EN);
    }
    (*cl).syncgen_in_use |= 1 << (*substream).stream;
    0
}

unsafe extern "C" fn mca_fe_hw_free(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let cl = mca_dai_to_cluster(dai);
    if (*cl).clk_provider {
        return 0;
    }
    (*cl).syncgen_in_use &= !(1 << (*substream).stream);
    if (*cl).syncgen_in_use != 0 {
        return 0;
    }
    mca_modify(cl, REG_SYNCGEN_STATUS as c_int, SYNCGEN_STATUS_EN, 0);
    if !(*cl).pd_link.is_null() {
        device_link_del((*cl).pd_link);
    }
    0
}

fn mca_crop_mask(mut mask: c_uint, nchans: c_int) -> c_uint {
    while hweight32(mask) > nchans {
        mask &= !(1 << __fls(mask));
    }
    mask
}

unsafe fn mca_configure_serdes(cl: *mut mca_cluster, serdes_unit: c_int, mask: c_uint, slots: c_int, nchans: c_int, slot_width: c_int, is_tx: bool_t, portmask: c_int) -> c_int {
    let serdes_base = ptr_add((*cl).base, serdes_unit as isize);
    let mut serdes_conf_mask: u32 = SERDES_CONF_WIDTH_MASK | SERDES_CONF_NCHANS;
    let mut serdes_conf: u32 = FIELD_PREP(SERDES_CONF_NCHANS, (if slots > 1 { slots } else { 1 } - 1) as u32);

    match slot_width {
        16 => serdes_conf |= SERDES_CONF_WIDTH_16BIT,
        20 => serdes_conf |= SERDES_CONF_WIDTH_20BIT,
        24 => serdes_conf |= SERDES_CONF_WIDTH_24BIT,
        32 => serdes_conf |= SERDES_CONF_WIDTH_32BIT,
        _ => {
            dev_err((*(*cl).host).dev, c"unsupported SERDES configuration requested (mask=0x%x slots=%d slot_width=%d)\n".as_ptr(), mask, slots, slot_width);
            return -EINVAL;
        }
    }

    serdes_conf_mask |= SERDES_CONF_SYNC_SEL;
    serdes_conf |= FIELD_PREP(SERDES_CONF_SYNC_SEL, ((*cl).no + 1) as u32);

    if is_tx {
        serdes_conf_mask |= SERDES_CONF_UNK1 | SERDES_CONF_UNK2 | SERDES_CONF_UNK3;
        serdes_conf |= SERDES_CONF_UNK1 | SERDES_CONF_UNK2 | SERDES_CONF_UNK3;
    } else {
        serdes_conf_mask |= SERDES_CONF_UNK1 | SERDES_CONF_UNK2 | SERDES_CONF_UNK3 | SERDES_CONF_NO_DATA_FEEDBACK;
        serdes_conf |= SERDES_CONF_UNK1 | SERDES_CONF_UNK2 | SERDES_CONF_NO_DATA_FEEDBACK;
    }

    mca_modify(cl, serdes_unit + if is_tx { REG_TX_SERDES_CONF as c_int } else { REG_RX_SERDES_CONF as c_int }, serdes_conf_mask, serdes_conf);

    if is_tx {
        writel_relaxed(0xffffffff, ptr_add(serdes_base, REG_TX_SERDES_SLOTMASK));
        writel_relaxed(!(mca_crop_mask(mask, nchans) as u32), ptr_add(serdes_base, REG_TX_SERDES_SLOTMASK + 0x4));
        writel_relaxed(0xffffffff, ptr_add(serdes_base, REG_TX_SERDES_SLOTMASK + 0x8));
        writel_relaxed(!(mask as u32), ptr_add(serdes_base, REG_TX_SERDES_SLOTMASK + 0xc));
    } else {
        writel_relaxed(0xffffffff, ptr_add(serdes_base, REG_RX_SERDES_SLOTMASK));
        writel_relaxed(!(mca_crop_mask(mask, nchans) as u32), ptr_add(serdes_base, REG_RX_SERDES_SLOTMASK + 0x4));
        writel_relaxed(portmask as u32, ptr_add(serdes_base, REG_RX_SERDES_PORT));
    }
    0
}

unsafe extern "C" fn mca_fe_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let cl = mca_dai_to_cluster(dai);
    let nchannels: c_uint;

    if (*cl).tdm_slots != 0 {
        let mask = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK { (*cl).tdm_tx_mask } else { (*cl).tdm_rx_mask };
        nchannels = hweight32(mask) as c_uint;
    } else {
        nchannels = 2;
    }

    snd_pcm_hw_constraint_minmax((*substream).runtime, SNDRV_PCM_HW_PARAM_CHANNELS, 1, nchannels)
}

unsafe extern "C" fn mca_fe_set_tdm_slot(dai: *mut snd_soc_dai, tx_mask: c_uint, rx_mask: c_uint, slots: c_int, slot_width: c_int) -> c_int {
    let cl = mca_dai_to_cluster(dai);
    (*cl).tdm_slots = slots;
    (*cl).tdm_slot_width = slot_width;
    (*cl).tdm_tx_mask = tx_mask;
    (*cl).tdm_rx_mask = rx_mask;
    0
}

unsafe extern "C" fn mca_fe_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let cl = mca_dai_to_cluster(dai);
    let mca = (*cl).host;
    let mut fpol_inv = false;
    let mut serdes_conf: u32 = 0;
    let bitstart: u32;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BP_FP => (*cl).clk_provider = true,
        SND_SOC_DAIFMT_BC_FC => (*cl).clk_provider = false,
        _ => {
            dev_err((*mca).dev, c"unsupported DAI format (0x%x) requested\n".as_ptr(), fmt);
            return -EINVAL;
        }
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            fpol_inv = false;
            bitstart = 1;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            fpol_inv = true;
            bitstart = 0;
        }
        _ => {
            dev_err((*mca).dev, c"unsupported DAI format (0x%x) requested\n".as_ptr(), fmt);
            return -EINVAL;
        }
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_IF | SND_SOC_DAIFMT_IB_IF => fpol_inv ^= true,
        _ => {}
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_NB_IF => serdes_conf |= SERDES_CONF_BCLK_POL,
        _ => {}
    }

    if !fpol_inv {
        dev_err((*mca).dev, c"unsupported DAI format (0x%x) requested\n".as_ptr(), fmt);
        return -EINVAL;
    }

    mca_modify(cl, (CLUSTER_TX_OFF + REG_TX_SERDES_CONF) as c_int, SERDES_CONF_BCLK_POL, serdes_conf);
    mca_modify(cl, (CLUSTER_RX_OFF + REG_RX_SERDES_CONF) as c_int, SERDES_CONF_BCLK_POL, serdes_conf);
    writel_relaxed(bitstart, ptr_add((*cl).base, CLUSTER_TX_OFF + REG_TX_SERDES_BITSTART));
    writel_relaxed(bitstart, ptr_add((*cl).base, CLUSTER_RX_OFF + REG_RX_SERDES_BITSTART));
    0
}

unsafe extern "C" fn mca_set_bclk_ratio(dai: *mut snd_soc_dai, ratio: c_uint) -> c_int {
    let cl = mca_dai_to_cluster(dai);
    (*cl).bclk_ratio = ratio;
    0
}

unsafe extern "C" fn mca_fe_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let cl = mca_dai_to_cluster(dai);
    let mca = (*cl).host;
    let dev = (*mca).dev;
    let samp_rate = params_rate(params);
    let is_tx = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    let mut refine_tdm = false;
    let mut tdm_slots: c_uint;
    let mut tdm_slot_width: c_uint;
    let mut tdm_mask: c_uint = 0;
    let mut ret: c_int;

    if (*cl).tdm_slot_width == 0 {
        /*
         * We were not given TDM settings from above, set initial
         * guesses which will later be refined.
         */
        tdm_slot_width = params_width(params);
        tdm_slots = params_channels(params);
        refine_tdm = true;
    } else {
        tdm_slot_width = (*cl).tdm_slot_width as c_uint;
        tdm_slots = (*cl).tdm_slots as c_uint;
        tdm_mask = if is_tx { (*cl).tdm_tx_mask } else { (*cl).tdm_rx_mask };
    }

    let mut bclk_ratio: c_ulong = if (*cl).bclk_ratio != 0 {
        (*cl).bclk_ratio as c_ulong
    } else {
        (tdm_slot_width * tdm_slots) as c_ulong
    };

    if refine_tdm {
        let nchannels = params_channels(params);
        if nchannels > 2 {
            dev_err(dev, c"missing TDM for stream with two or more channels\n".as_ptr());
            return -EINVAL;
        }
        if bclk_ratio % nchannels as c_ulong != 0 {
            dev_err(dev, c"BCLK ratio (%ld) not divisible by no. of channels (%d)\n".as_ptr(), bclk_ratio, nchannels);
            return -EINVAL;
        }
        tdm_slot_width = (bclk_ratio / nchannels as c_ulong) as c_uint;
        if tdm_slot_width > 32 && nchannels == 1 {
            tdm_slot_width = 32;
        }
        if tdm_slot_width < params_width(params) {
            dev_err(dev, c"TDM slots too narrow (tdm=%u params=%d)\n".as_ptr(), tdm_slot_width, params_width(params));
            return -EINVAL;
        }
        tdm_mask = (1 << tdm_slots) - 1;
    }

    let portmask = mca_fe_get_portmask(substream);
    if portmask == 0 {
        return -EINVAL;
    }

    ret = mca_configure_serdes(cl, if is_tx { CLUSTER_TX_OFF as c_int } else { CLUSTER_RX_OFF as c_int }, tdm_mask, tdm_slots as c_int, params_channels(params) as c_int, tdm_slot_width as c_int, is_tx, portmask);
    if ret != 0 {
        return ret;
    }

    let pad = 32 - params_width(params);
    /*
     * TODO: Here the register semantics aren't clear.
     */
    let nchans_ceiled = core::cmp::min(params_channels(params), 4);
    let regval = FIELD_PREP(DMA_ADAPTER_NCHANS, nchans_ceiled)
        | FIELD_PREP(DMA_ADAPTER_TX_NCHANS, 0x2)
        | FIELD_PREP(DMA_ADAPTER_RX_NCHANS, 0x2)
        | FIELD_PREP(DMA_ADAPTER_TX_LSB_PAD, pad)
        | FIELD_PREP(DMA_ADAPTER_RX_MSB_PAD, pad);

    if is_tx {
        writel_relaxed(regval, ptr_add((*mca).switch_base, REG_DMA_ADAPTER_A((*cl).no)));
    } else {
        writel_relaxed(regval, ptr_add((*mca).switch_base, REG_DMA_ADAPTER_B((*cl).no)));
    }

    if !mca_fe_clocks_in_use(cl) {
        /*
         * Set up FSYNC duty cycle as even as possible.
         */
        writel_relaxed(((bclk_ratio / 2) - 1) as u32, ptr_add((*cl).base, REG_SYNCGEN_HI_PERIOD));
        writel_relaxed((((bclk_ratio + 1) / 2) - 1) as u32, ptr_add((*cl).base, REG_SYNCGEN_LO_PERIOD));
        writel_relaxed(FIELD_PREP(MCLK_CONF_DIV, 0x1), ptr_add((*cl).base, REG_MCLK_CONF));

        ret = clk_set_rate((*cl).clk_parent, bclk_ratio * samp_rate as c_ulong);
        if ret != 0 {
            dev_err((*mca).dev, c"cluster %d: unable to set clock parent: %d\n".as_ptr(), (*cl).no, ret);
            return ret;
        }
    }
    0
}

static MCA_FE_SELECTABLE_FORMATS: [u64; 2] = [
    /* pattern 1 */
    SND_SOC_POSSIBLE_DAIFMT_I2S | SND_SOC_POSSIBLE_DAIFMT_NB_IF | SND_SOC_POSSIBLE_DAIFMT_IB_IF,
    /* pattern 2 */
    SND_SOC_POSSIBLE_DAIFMT_LEFT_J | SND_SOC_POSSIBLE_DAIFMT_NB_NF | SND_SOC_POSSIBLE_DAIFMT_IB_NF,
];

static MCA_FE_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(mca_fe_startup),
    set_fmt: Some(mca_fe_set_fmt),
    set_bclk_ratio: Some(mca_set_bclk_ratio),
    set_tdm_slot: Some(mca_fe_set_tdm_slot),
    hw_params: Some(mca_fe_hw_params),
    trigger: Some(mca_fe_trigger),
    prepare: Some(mca_fe_prepare),
    hw_free: Some(mca_fe_hw_free),
    shutdown: None,
    auto_selectable_formats: MCA_FE_SELECTABLE_FORMATS.as_ptr(),
    num_auto_selectable_formats: MCA_FE_SELECTABLE_FORMATS.len(),
};

/*
 * Is there a FE attached which will be feeding this port's clocks?
 */
unsafe fn mca_be_clk_started(cl: *mut mca_cluster) -> bool_t {
    let mut ret = false;
    for_each_pcm_streams(|stream| {
        if (*cl).port_clk_started[stream as usize] {
            ret = true;
        }
    });
    ret
}

unsafe fn mca_be_get_fe(be: *mut snd_soc_pcm_runtime, _stream: c_int) -> *mut snd_soc_pcm_runtime {
    let fe: *mut snd_soc_pcm_runtime = null_mut();
    // for_each_dpcm_fe(be, stream, dpcm)
    // External DPCM iterator is supplied by ASoC.
    fe
}

unsafe extern "C" fn mca_be_prepare(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let be = snd_soc_substream_to_rtd(substream);
    let fe = mca_be_get_fe(be, (*substream).stream as c_int);
    let cl = mca_dai_to_cluster(dai);
    let mca = (*cl).host;
    let fe_cl = mca_dai_to_cluster(snd_soc_rtd_to_cpu(fe, 0));

    if !(*fe_cl).clk_provider {
        return 0;
    }
    if (*cl).port_clk_driver < 0 {
        return 0;
    }

    let fe_clk_cl = cluster_at(mca, (*cl).port_clk_driver);
    /*
     * Typically the CODECs we are paired with will require clocks
     * to be present at time of unmute with the 'mute_stream' op
     * or at time of DAPM widget power-up. We need to enable clocks
     * here at the latest (frontend prepare would be too late).
     */
    if !mca_fe_clocks_in_use(fe_clk_cl) {
        let ret = mca_fe_enable_clocks(fe_clk_cl);
        if ret < 0 {
            return ret;
        }
    }
    (*cl).clocks_in_use[(*substream).stream as usize] = true;
    0
}

unsafe extern "C" fn mca_be_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let be = snd_soc_substream_to_rtd(substream);
    let fe = mca_be_get_fe(be, (*substream).stream as c_int);
    let cl = mca_dai_to_cluster(dai);
    let mca = (*cl).host;

    if fe.is_null() {
        return -EINVAL;
    }
    let fe_cl = mca_dai_to_cluster(snd_soc_rtd_to_cpu(fe, 0));

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        writel_relaxed(PORT_DATA_SEL_TXA((*fe_cl).no), ptr_add((*cl).base, REG_PORT_DATA_SEL));
        mca_modify(cl, REG_PORT_ENABLES as c_int, PORT_ENABLES_TX_DATA, PORT_ENABLES_TX_DATA);
    }

    if !(*fe_cl).clk_provider {
        return 0;
    }

    if mca_be_clk_started(cl) {
        /*
         * Port is already started in the other direction.
         * Make sure there isn't a conflict with another cluster
         * driving the port clocks.
         */
        if (*cl).port_clk_driver != (*fe_cl).no {
            return -EINVAL;
        }
        (*cl).port_clk_started[(*substream).stream as usize] = true;
        return 0;
    }

    writel_relaxed(FIELD_PREP(PORT_CLOCK_SEL, ((*fe_cl).no + 1) as u32), ptr_add((*cl).base, REG_PORT_CLOCK_SEL));
    writel_relaxed(PORT_DATA_SEL_TXA((*fe_cl).no), ptr_add((*cl).base, REG_PORT_DATA_SEL));
    mca_modify(cl, REG_PORT_ENABLES as c_int, PORT_ENABLES_CLOCKS, PORT_ENABLES_CLOCKS);

    mutex_lock(&mut (*mca).port_mutex);
    (*cl).port_clk_driver = (*fe_cl).no;
    mutex_unlock(&mut (*mca).port_mutex);
    (*cl).port_clk_started[(*substream).stream as usize] = true;
    0
}

unsafe extern "C" fn mca_be_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    let be = snd_soc_substream_to_rtd(substream);
    let fe = mca_be_get_fe(be, (*substream).stream as c_int);
    let cl = mca_dai_to_cluster(dai);
    let mca = (*cl).host;

    if fe.is_null() {
        return;
    }
    let fe_cl = mca_dai_to_cluster(snd_soc_rtd_to_cpu(fe, 0));

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        mca_modify(cl, REG_PORT_ENABLES as c_int, PORT_ENABLES_TX_DATA, 0);
        writel_relaxed(0, ptr_add((*cl).base, REG_PORT_DATA_SEL));
    }

    if !(*fe_cl).clk_provider {
        return;
    }

    (*cl).port_clk_started[(*substream).stream as usize] = false;
    if !mca_be_clk_started(cl) {
        /*
         * Were we the last direction to shutdown?
         * Turn off the lights (clocks).
         */
        mca_modify(cl, REG_PORT_ENABLES as c_int, PORT_ENABLES_CLOCKS, 0);
        writel_relaxed(0, ptr_add((*cl).base, REG_PORT_CLOCK_SEL));
        mutex_lock(&mut (*mca).port_mutex);
        (*cl).port_clk_driver = -1;
        mutex_unlock(&mut (*mca).port_mutex);
    }
}

static MCA_BE_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    prepare: Some(mca_be_prepare),
    hw_free: Some(mca_be_hw_free),
    startup: Some(mca_be_startup),
    shutdown: Some(mca_be_shutdown),
    set_fmt: None,
    set_bclk_ratio: None,
    set_tdm_slot: None,
    hw_params: None,
    trigger: None,
    auto_selectable_formats: null(),
    num_auto_selectable_formats: 0,
};

unsafe extern "C" fn mca_set_runtime_hwparams(_component: *mut snd_soc_component, substream: *mut snd_pcm_substream, chan: *mut dma_chan) -> c_int {
    let dma_dev = (*(*chan).device).dev;
    let mut dma_data: snd_dmaengine_dai_dma_data = zeroed();
    let mut hw: snd_pcm_hardware = zeroed();

    hw.info = SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_INTERLEAVED;
    hw.periods_min = 2;
    hw.periods_max = UINT_MAX;
    hw.period_bytes_min = 256;
    hw.period_bytes_max = dma_get_max_seg_size(dma_dev);
    hw.buffer_bytes_max = SIZE_MAX;
    hw.fifo_size = 16;

    let ret = snd_dmaengine_pcm_refine_runtime_hwparams(substream, &mut dma_data, &mut hw, chan);
    if ret != 0 {
        return ret;
    }
    snd_soc_set_runtime_hwparams(substream, &mut hw)
}

unsafe extern "C" fn mca_pcm_open(component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let cl = mca_dai_to_cluster(snd_soc_rtd_to_cpu(rtd, 0));
    let chan = (*cl).dma_chans[(*substream).stream as usize];
    let mut ret: c_int;

    if (*(*rtd).dai_link).no_pcm {
        return 0;
    }

    ret = mca_set_runtime_hwparams(component, substream, chan);
    if ret != 0 {
        return ret;
    }
    snd_dmaengine_pcm_open(substream, chan)
}

unsafe extern "C" fn mca_hw_params(_component: *mut snd_soc_component, substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let chan = snd_dmaengine_pcm_get_chan(substream);
    let mut slave_config: dma_slave_config = zeroed();

    if (*(*rtd).dai_link).no_pcm {
        return 0;
    }

    let ret = snd_hwparams_to_dma_slave_config(substream, params, &mut slave_config);
    if ret < 0 {
        return ret;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        slave_config.dst_port_window_size = core::cmp::min(params_channels(params), 4);
    } else {
        slave_config.src_port_window_size = core::cmp::min(params_channels(params), 4);
    }
    dmaengine_slave_config(chan, &mut slave_config)
}

unsafe extern "C" fn mca_close(_component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    if (*(*rtd).dai_link).no_pcm {
        return 0;
    }
    snd_dmaengine_pcm_close(substream)
}

unsafe extern "C" fn mca_trigger(_component: *mut snd_soc_component, substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    if (*(*rtd).dai_link).no_pcm {
        return 0;
    }
    /*
     * Before we do the PCM trigger proper, insert an opportunity
     * to reset the frontend's SERDES.
     */
    mca_fe_early_trigger(substream, cmd, snd_soc_rtd_to_cpu(rtd, 0));
    snd_dmaengine_pcm_trigger(substream, cmd)
}

unsafe extern "C" fn mca_pointer(_component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let rtd = snd_soc_substream_to_rtd(substream);
    if (*(*rtd).dai_link).no_pcm {
        return -ENOTSUPP as snd_pcm_uframes_t;
    }
    snd_dmaengine_pcm_pointer(substream)
}

unsafe fn mca_request_dma_channel(cl: *mut mca_cluster, stream: c_uint) -> *mut dma_chan {
    let is_tx = stream == SNDRV_PCM_STREAM_PLAYBACK;
    let name = devm_kasprintf((*(*cl).host).dev, GFP_KERNEL, if is_tx { c"tx%da".as_ptr() } else { c"rx%db".as_ptr() }, (*cl).no);
    of_dma_request_slave_channel((*(*(*cl).host).dev).of_node, name)
}

unsafe extern "C" fn mca_pcm_free(_component: *mut snd_soc_component, pcm: *mut snd_pcm) {
    let rtd = snd_pcm_chip(pcm) as *mut snd_soc_pcm_runtime;
    let cl = mca_dai_to_cluster(snd_soc_rtd_to_cpu(rtd, 0));

    if (*(*rtd).dai_link).no_pcm {
        return;
    }

    for_each_pcm_streams(|i| {
        let substream = (*(*rtd).pcm).streams[i as usize].substream;
        if substream.is_null() || (*cl).dma_chans[i as usize].is_null() {
            return;
        }
        dma_release_channel((*cl).dma_chans[i as usize]);
        (*cl).dma_chans[i as usize] = null_mut();
    });
}

unsafe extern "C" fn mca_pcm_new(component: *mut snd_soc_component, rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let cl = mca_dai_to_cluster(snd_soc_rtd_to_cpu(rtd, 0));

    if (*(*rtd).dai_link).no_pcm {
        return 0;
    }

    let mut i = 0;
    while i <= SNDRV_PCM_STREAM_LAST {
        let substream = (*(*rtd).pcm).streams[i as usize].substream;
        if !substream.is_null() {
            let chan = mca_request_dma_channel(cl, i);
            if IS_ERR_OR_NULL(chan as *const c_void) {
                mca_pcm_free(component, (*rtd).pcm);
                if !chan.is_null() && PTR_ERR(chan as *const c_void) == -(EPROBE_DEFER as isize) {
                    return PTR_ERR(chan as *const c_void) as c_int;
                }
                dev_err((*component).dev, c"unable to obtain DMA channel (stream %d cluster %d): %pe\n".as_ptr(), i, (*cl).no, chan);
                if chan.is_null() {
                    return -EINVAL;
                }
                return PTR_ERR(chan as *const c_void) as c_int;
            }
            (*cl).dma_chans[i as usize] = chan;
            snd_pcm_set_managed_buffer(substream, SNDRV_DMA_TYPE_DEV_IRAM, (*(*chan).device).dev, 512 * 1024 * 6, SIZE_MAX);
        }
        i += 1;
    }
    0
}

static MCA_COMPONENT: snd_soc_component_driver = snd_soc_component_driver {
    name: c"apple-mca".as_ptr(),
    open: Some(mca_pcm_open),
    close: Some(mca_close),
    hw_params: Some(mca_hw_params),
    trigger: Some(mca_trigger),
    pointer: Some(mca_pointer),
    pcm_new: Some(mca_pcm_new),
    pcm_free: Some(mca_pcm_free),
};

unsafe fn apple_mca_release(mca: *mut mca_data) {
    let mut i = 0;
    while i < (*mca).nclusters {
        let cl = cluster_at(mca, i);
        if !IS_ERR_OR_NULL((*cl).clk_parent as *const c_void) {
            clk_put((*cl).clk_parent);
        }
        if !IS_ERR_OR_NULL((*cl).pd_dev as *const c_void) {
            dev_pm_domain_detach((*cl).pd_dev, true);
        }
        i += 1;
    }

    if !(*mca).pd_link.is_null() {
        device_link_del((*mca).pd_link);
    }
    if !IS_ERR_OR_NULL((*mca).pd_dev as *const c_void) {
        dev_pm_domain_detach((*mca).pd_dev, true);
    }
    reset_control_rearm((*mca).rstc);
}

unsafe extern "C" fn apple_mca_probe(pdev: *mut platform_device) -> c_int {
    let mut res: *mut resource = null_mut();
    let base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR(base) {
        return PTR_ERR(base) as c_int;
    }

    if resource_size(res) < CLUSTER_STRIDE as usize {
        return -EINVAL;
    }
    let nclusters = ((resource_size(res) - CLUSTER_STRIDE as usize) / CLUSTER_STRIDE as usize + 1) as c_int;

    let mca = devm_kzalloc(&mut (*pdev).dev, size_of::<mca_data>() + size_of::<mca_cluster>() * nclusters as usize, GFP_KERNEL) as *mut mca_data;
    if mca.is_null() {
        return -ENOMEM;
    }
    (*mca).dev = &mut (*pdev).dev;
    (*mca).nclusters = nclusters;
    mutex_init(&mut (*mca).port_mutex);
    platform_set_drvdata(pdev, mca as *mut c_void);
    let clusters = (*mca).clusters.as_mut_ptr();

    (*mca).switch_base = devm_platform_ioremap_resource(pdev, 1);
    if IS_ERR((*mca).switch_base) {
        return PTR_ERR((*mca).switch_base) as c_int;
    }

    (*mca).rstc = devm_reset_control_get_optional_shared(&mut (*pdev).dev, null());
    if IS_ERR((*mca).rstc as *const c_void) {
        return PTR_ERR((*mca).rstc as *const c_void) as c_int;
    }

    let dai_drivers = devm_kzalloc(&mut (*pdev).dev, size_of::<snd_soc_dai_driver>() * 2 * nclusters as usize, GFP_KERNEL) as *mut snd_soc_dai_driver;
    if dai_drivers.is_null() {
        return -ENOMEM;
    }

    (*mca).pd_dev = dev_pm_domain_attach_by_id(&mut (*pdev).dev, 0);
    if IS_ERR((*mca).pd_dev as *const c_void) {
        return -EINVAL;
    }

    (*mca).pd_link = device_link_add(&mut (*pdev).dev, (*mca).pd_dev, DL_FLAG_STATELESS | DL_FLAG_PM_RUNTIME | DL_FLAG_RPM_ACTIVE);
    if (*mca).pd_link.is_null() {
        let ret = -EINVAL;
        /* Prevent an unbalanced reset rearm */
        (*mca).rstc = null_mut();
        apple_mca_release(mca);
        return ret;
    }

    reset_control_reset((*mca).rstc);

    let mut i = 0;
    while i < nclusters {
        let cl = clusters.offset(i as isize);
        let fe = dai_drivers.offset(((*mca).nclusters + i) as isize);
        let be = dai_drivers.offset(i as isize);

        (*cl).host = mca;
        (*cl).no = i;
        (*cl).base = ptr_add(base, CLUSTER_STRIDE * i as isize);
        (*cl).port_clk_driver = -1;
        (*cl).clk_parent = of_clk_get((*pdev).dev.of_node, i);
        if IS_ERR((*cl).clk_parent as *const c_void) {
            dev_err(&mut (*pdev).dev, c"unable to obtain clock %d: %ld\n".as_ptr(), i, PTR_ERR((*cl).clk_parent as *const c_void));
            let ret = PTR_ERR((*cl).clk_parent as *const c_void) as c_int;
            apple_mca_release(mca);
            return ret;
        }
        (*cl).pd_dev = dev_pm_domain_attach_by_id(&mut (*pdev).dev, (i + 1) as c_uint);
        if IS_ERR((*cl).pd_dev as *const c_void) {
            dev_err(&mut (*pdev).dev, c"unable to obtain cluster %d PD: %ld\n".as_ptr(), i, PTR_ERR((*cl).pd_dev as *const c_void));
            let ret = PTR_ERR((*cl).pd_dev as *const c_void) as c_int;
            apple_mca_release(mca);
            return ret;
        }

        (*fe).id = i;
        (*fe).name = devm_kasprintf(&mut (*pdev).dev, GFP_KERNEL, c"mca-pcm-%d".as_ptr(), i);
        if (*fe).name.is_null() {
            apple_mca_release(mca);
            return -ENOMEM;
        }
        (*fe).ops = &MCA_FE_OPS;
        (*fe).playback.channels_min = 1;
        (*fe).playback.channels_max = 32;
        (*fe).playback.rates = SNDRV_PCM_RATE_8000_192000;
        (*fe).playback.formats = APPLE_MCA_FMTBITS;
        (*fe).capture.channels_min = 1;
        (*fe).capture.channels_max = 32;
        (*fe).capture.rates = SNDRV_PCM_RATE_8000_192000;
        (*fe).capture.formats = APPLE_MCA_FMTBITS;
        (*fe).symmetric_rate = 1;

        (*fe).playback.stream_name = devm_kasprintf(&mut (*pdev).dev, GFP_KERNEL, c"PCM%d TX".as_ptr(), i);
        (*fe).capture.stream_name = devm_kasprintf(&mut (*pdev).dev, GFP_KERNEL, c"PCM%d RX".as_ptr(), i);
        if (*fe).playback.stream_name.is_null() || (*fe).capture.stream_name.is_null() {
            apple_mca_release(mca);
            return -ENOMEM;
        }

        (*be).id = i + nclusters;
        (*be).name = devm_kasprintf(&mut (*pdev).dev, GFP_KERNEL, c"mca-i2s-%d".as_ptr(), i);
        if (*be).name.is_null() {
            apple_mca_release(mca);
            return -ENOMEM;
        }
        (*be).ops = &MCA_BE_OPS;
        (*be).playback.channels_min = 1;
        (*be).playback.channels_max = 32;
        (*be).playback.rates = SNDRV_PCM_RATE_8000_192000;
        (*be).playback.formats = APPLE_MCA_FMTBITS;
        (*be).capture.channels_min = 1;
        (*be).capture.channels_max = 32;
        (*be).capture.rates = SNDRV_PCM_RATE_8000_192000;
        (*be).capture.formats = APPLE_MCA_FMTBITS;

        (*be).playback.stream_name = devm_kasprintf(&mut (*pdev).dev, GFP_KERNEL, c"I2S%d TX".as_ptr(), i);
        (*be).capture.stream_name = devm_kasprintf(&mut (*pdev).dev, GFP_KERNEL, c"I2S%d RX".as_ptr(), i);
        if (*be).playback.stream_name.is_null() || (*be).capture.stream_name.is_null() {
            apple_mca_release(mca);
            return -ENOMEM;
        }
        i += 1;
    }

    let ret = snd_soc_register_component(&mut (*pdev).dev, &MCA_COMPONENT, dai_drivers, nclusters * 2);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, c"unable to register ASoC component: %d\n".as_ptr(), ret);
        apple_mca_release(mca);
        return ret;
    }
    0
}

unsafe extern "C" fn apple_mca_remove(pdev: *mut platform_device) {
    let mca = platform_get_drvdata(pdev) as *mut mca_data;
    snd_soc_unregister_component(&mut (*pdev).dev);
    apple_mca_release(mca);
}

static APPLE_MCA_OF_MATCH: [of_device_id; 3] = [
    of_device_id { compatible: c"apple,t8103-mca".as_ptr() },
    of_device_id { compatible: c"apple,mca".as_ptr() },
    of_device_id { compatible: null() },
];
// MODULE_DEVICE_TABLE(of, apple_mca_of_match);

static APPLE_MCA_DRIVER: platform_driver = platform_driver {
    driver: device_driver {
        name: c"apple-mca".as_ptr(),
        of_match_table: APPLE_MCA_OF_MATCH.as_ptr(),
    },
    probe: Some(apple_mca_probe),
    remove: Some(apple_mca_remove),
};
// module_platform_driver(apple_mca_driver);

// MODULE_AUTHOR("Martin Povišer <povik+lin@cutebit.org>");
// MODULE_DESCRIPTION("ASoC Apple MCA driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
