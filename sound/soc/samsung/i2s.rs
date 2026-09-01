// SPDX-License-Identifier: GPL-2.0
//
// ALSA SoC Audio Layer - Samsung I2S Controller driver
//
// Copyright (c) 2010 Samsung Electronics Co. Ltd.
//	Jaswinder Singh <jassisinghbrar@gmail.com>

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr::{null, null_mut};

// External dependencies originally provided by:
// <dt-bindings/sound/samsung-i2s.h>, <linux/delay.h>, <linux/slab.h>,
// <linux/cleanup.h>, <linux/clk.h>, <linux/clk-provider.h>, <linux/io.h>,
// <linux/module.h>, <linux/of.h>, <linux/pm_runtime.h>, <sound/soc.h>,
// <sound/pcm_params.h>, <linux/platform_data/asoc-s3c.h>, "dma.h",
// "idma.h", "i2s.h", and "i2s-regs.h".

type dma_filter_fn = Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> bool>;
type snd_pcm_sframes_t = isize;
type kernel_ulong_t = c_ulong;
type spinlock_t = c_void;

#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct resource { start: u32 }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_pcm_runtime { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { _private: [u8; 0] }

#[repr(C)]
pub struct device {
    platform_data: *mut c_void,
    of_node: *mut device_node,
}

#[repr(C)]
pub struct platform_device {
    dev: device,
}

#[repr(C)]
pub struct snd_pcm_substream {
    stream: c_int,
}

#[repr(C)]
pub struct snd_soc_dai {
    id: c_int,
    dev: *mut device,
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    addr: u32,
    addr_width: c_uint,
    chan_name: *const c_char,
    filter_data: *mut c_void,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    name: *const c_char,
    id: c_int,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
    symmetric_rate: c_uint,
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    set_clkdiv: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    delay: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> snd_pcm_sframes_t>,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_component {
    dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    name: *const c_char,
    probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct clk_onecell_data {
    clks: *mut *mut clk,
    clk_num: c_uint,
}

#[repr(C)]
pub struct platform_device_id {
    name: [c_char; 20],
    driver_data: kernel_ulong_t,
}

#[repr(C)]
pub struct of_device_id {
    compatible: *const c_char,
    data: *const c_void,
}

#[repr(C)] pub struct dev_pm_ops { _private: [u8; 0] }

#[repr(C)]
pub struct device_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
    pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    id_table: *const platform_device_id,
    driver: device_driver,
}

#[repr(C)]
pub struct s3c_audio_type {
    quirks: u32,
    idma_addr: u32,
}

#[repr(C)]
pub struct s3c_audio_pdata {
    dma_playback: *mut c_void,
    dma_capture: *mut c_void,
    dma_play_sec: *mut c_void,
    dma_filter: dma_filter_fn,
    cfg_gpio: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    type_: s3c_audio_type,
}

#[repr(C)]
struct samsung_i2s_variant_regs {
    bfs_off: c_uint,
    rfs_off: c_uint,
    sdf_off: c_uint,
    txr_off: c_uint,
    rclksrc_off: c_uint,
    mss_off: c_uint,
    cdclkcon_off: c_uint,
    lrp_off: c_uint,
    bfs_mask: c_uint,
    rfs_mask: c_uint,
    ftx0cnt_off: c_uint,
}

#[repr(C)]
struct samsung_i2s_dai_data {
    quirks: u32,
    pcm_rates: c_uint,
    i2s_variant_regs: *const samsung_i2s_variant_regs,
    fixup_early: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    fixup_late: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
}

#[repr(C)]
struct i2s_dai {
    /* Platform device for this DAI */
    pdev: *mut platform_device,
    /* Frame clock */
    frmclk: c_uint,
    /*
     * Specifically requested RCLK, BCLK by machine driver.
     * 0 indicates CPU driver is free to choose any value.
     */
    rfs: c_uint,
    bfs: c_uint,
    /* Pointer to the Primary_Fifo if this is Sec_Fifo, NULL otherwise */
    pri_dai: *mut i2s_dai,
    /* Pointer to the Secondary_Fifo if it has one, NULL otherwise */
    sec_dai: *mut i2s_dai,
    mode: c_uint,
    /* Driver for this DAI */
    drv: *mut snd_soc_dai_driver,
    /* DMA parameters */
    dma_playback: snd_dmaengine_dai_dma_data,
    dma_capture: snd_dmaengine_dai_dma_data,
    idma_playback: snd_dmaengine_dai_dma_data,
    filter: dma_filter_fn,
    priv_: *mut samsung_i2s_priv,
}

#[repr(C)]
struct samsung_i2s_priv {
    pdev: *mut platform_device,
    pdev_sec: *mut platform_device,
    /* Lock for cross interface checks */
    pcm_lock: spinlock_t,
    /* CPU DAIs and their corresponding drivers */
    dai: *mut i2s_dai,
    dai_drv: *mut snd_soc_dai_driver,
    num_dais: c_int,
    /* The I2S controller's core clock */
    clk: *mut clk,
    /* Clock for generating I2S signals */
    op_clk: *mut clk,
    /* Rate of RCLK source clock */
    rclk_srcrate: c_ulong,
    /* Cache of selected I2S registers for system suspend */
    suspend_i2smod: u32,
    suspend_i2scon: u32,
    suspend_i2spsr: u32,
    variant_regs: *const samsung_i2s_variant_regs,
    fixup_early: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    fixup_late: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    quirks: u32,
    /* The clock provider's data */
    clk_table: [*mut clk; 3],
    clk_data: clk_onecell_data,
    /* Spinlock protecting member fields below */
    lock: spinlock_t,
    /* Memory mapped SFR region */
    addr: *mut c_void,
    /* A flag indicating the I2S slave mode operation */
    slave_mode: bool,
}

const EAGAIN: c_int = 11;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const SAMSUNG_I2S_ID_PRIMARY: c_int = 1;
const SAMSUNG_I2S_ID_SECONDARY: c_int = 2;
const DAI_OPENED: c_uint = 1 << 0; /* DAI is opened */
const DAI_MANAGER: c_uint = 1 << 1; /* DAI is the manager */

// Numeric values below are external C constants/macros from the included Linux
// and Samsung I2S headers. They are left as file-local Rust constants to
// preserve operations from the C source in this isolated translation.
const I2SCON: usize = 0; const I2SMOD: usize = 0; const I2SFICS: usize = 0; const I2SFIC: usize = 0; const I2SPSR: usize = 0; const I2STXD: u32 = 0; const I2SRXD: u32 = 0; const I2STXDS: u32 = 0;
const CON_TXSDMA_ACTIVE: u32 = 0; const CON_TXDMA_ACTIVE: u32 = 0; const CON_RXDMA_ACTIVE: u32 = 0; const CON_ACTIVE: u32 = 0; const CON_TXCH_PAUSE: u32 = 0; const CON_TXSDMA_PAUSE: u32 = 0; const CON_TXDMA_PAUSE: u32 = 0; const CON_RXDMA_PAUSE: u32 = 0; const CON_RXCH_PAUSE: u32 = 0; const CON_RSTCLR: u32 = 0;
const EXYNOS7_MOD_RCLK_192FS: u32 = 7; const EXYNOS7_MOD_RCLK_96FS: u32 = 6; const EXYNOS7_MOD_RCLK_128FS: u32 = 5; const EXYNOS7_MOD_RCLK_64FS: u32 = 4; const MOD_RCLK_768FS: u32 = 3; const MOD_RCLK_384FS: u32 = 2; const MOD_RCLK_512FS: u32 = 1; const MOD_RCLK_256FS: u32 = 0;
const MOD_BCLK_48FS: u32 = 1; const MOD_BCLK_32FS: u32 = 0; const MOD_BCLK_24FS: u32 = 3; const MOD_BCLK_16FS: u32 = 2; const EXYNOS5420_MOD_BCLK_64FS: u32 = 4; const EXYNOS5420_MOD_BCLK_96FS: u32 = 5; const EXYNOS5420_MOD_BCLK_128FS: u32 = 6; const EXYNOS5420_MOD_BCLK_192FS: u32 = 7; const EXYNOS5420_MOD_BCLK_256FS: u32 = 8;
const QUIRK_SUPPORTS_TDM: u32 = 1 << 0; const QUIRK_NO_MUXPSR: u32 = 1 << 1; const QUIRK_NEED_RSTCLR: u32 = 1 << 2; const QUIRK_SUPPORTS_IDMA: u32 = 1 << 3; const QUIRK_SEC_DAI: u32 = 1 << 4; const QUIRK_PRI_6CHAN: u32 = 1 << 5;
const MOD_OPCLK_MASK: u32 = 0; const MOD_OPCLK_SHIFT: u32 = 0; const MOD_OPCLK_PCLK: c_int = 0; const MOD_SDF_MASK: c_int = 0; const MOD_LR_RLOW: c_int = 0; const MOD_SDF_MSB: c_int = 0; const MOD_SDF_LSB: c_int = 0; const MOD_SDF_IIS: c_int = 0;
const MOD_DC2_EN: u32 = 0; const MOD_DC1_EN: u32 = 0; const MOD_BLCS_MASK: u32 = 0; const MOD_BLCP_MASK: u32 = 0; const MOD_BLC_MASK: u32 = 0; const MOD_BLCS_8BIT: u32 = 0; const MOD_BLCP_8BIT: u32 = 0; const MOD_BLC_8BIT: u32 = 0; const MOD_BLCS_16BIT: u32 = 0; const MOD_BLCP_16BIT: u32 = 0; const MOD_BLC_16BIT: u32 = 0; const MOD_BLCS_24BIT: u32 = 0; const MOD_BLCP_24BIT: u32 = 0; const MOD_BLC_24BIT: u32 = 0;
const FIC_RXFLUSH: u32 = 0; const FIC_TXFLUSH: u32 = 0; const PSR_PSREN: u32 = 0;
const SAMSUNG_I2S_OPCLK: c_int = 0; const SAMSUNG_I2S_CDCLK: c_int = 1; const SAMSUNG_I2S_RCLKSRC_0: c_int = 2; const SAMSUNG_I2S_RCLKSRC_1: c_int = 3; const SAMSUNG_I2S_DIV_BCLK: c_int = 4;
const SND_SOC_CLOCK_IN: c_int = 0; const SND_SOC_CLOCK_OUT: c_int = 1; const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0; const SND_SOC_DAIFMT_RIGHT_J: c_uint = 1; const SND_SOC_DAIFMT_LEFT_J: c_uint = 2; const SND_SOC_DAIFMT_I2S: c_uint = 3; const SND_SOC_DAIFMT_INV_MASK: c_uint = 0; const SND_SOC_DAIFMT_NB_NF: c_uint = 0; const SND_SOC_DAIFMT_NB_IF: c_uint = 1; const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0; const SND_SOC_DAIFMT_BC_FC: c_uint = 1; const SND_SOC_DAIFMT_BP_FP: c_uint = 2;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0; const SNDRV_PCM_STREAM_CAPTURE: c_int = 1; const SNDRV_PCM_TRIGGER_START: c_int = 0; const SNDRV_PCM_TRIGGER_RESUME: c_int = 1; const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 2; const SNDRV_PCM_TRIGGER_STOP: c_int = 3; const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 4; const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 5;
const SNDRV_PCM_FMTBIT_S8: u64 = 1 << 0; const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 1; const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 2; const SNDRV_PCM_RATE_8000_96000: c_uint = 0; const SNDRV_PCM_RATE_8000_192000: c_uint = 0;
const CLK_I2S_CDCLK: usize = 0; const CLK_I2S_RCLK_SRC: usize = 1; const CLK_I2S_RCLK_PSR: usize = 2; const CLK_SET_RATE_NO_REPARENT: c_ulong = 0; const CLK_SET_RATE_PARENT: c_ulong = 0; const CLK_GATE_SET_TO_DISABLE: u8 = 0;

unsafe extern "C" {
    static loops_per_jiffy: c_ulong;
    static HZ: c_ulong;
    fn readl(addr: *mut c_void) -> u32;
    fn writel(val: u32, addr: *mut c_void);
    fn cpu_relax();
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_init_dma_data(dai: *mut snd_soc_dai, playback: *mut snd_dmaengine_dai_dma_data, capture: *mut snd_dmaengine_dai_dma_data);
    fn snd_soc_dapm_add_routes(dapm: *mut snd_soc_dapm_context, routes: *const snd_soc_dapm_route, num: c_int) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn pm_runtime_put(dev: *mut device) -> c_int;
    fn pm_runtime_put_noidle(dev: *mut device) -> c_int;
    fn pm_runtime_active(dev: *mut device) -> bool;
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_force_suspend(dev: *mut device) -> c_int;
    fn pm_runtime_force_resume(dev: *mut device) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_put(clk: *mut clk);
    fn clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_get_parent(clk: *mut clk) -> *mut clk;
    fn clk_unregister(clk: *mut clk);
    fn __clk_get_name(clk: *mut clk) -> *const c_char;
    fn clk_register_mux(dev: *mut device, name: *const c_char, parents: *const *const c_char, num: u8, flags: c_ulong, reg: *mut c_void, shift: u8, width: u8, clk_mux_flags: u8, lock: *mut spinlock_t) -> *mut clk;
    fn clk_register_divider(dev: *mut device, name: *const c_char, parent: *const c_char, flags: c_ulong, reg: *mut c_void, shift: u8, width: u8, clk_divider_flags: u8, lock: *mut spinlock_t) -> *mut clk;
    fn clk_register_gate(dev: *mut device, name: *const c_char, parent: *const c_char, flags: c_ulong, reg: *mut c_void, bit_idx: u8, clk_gate_flags: u8, lock: *mut spinlock_t) -> *mut clk;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kasprintf(dev: *mut device, flags: c_uint, fmt: *const c_char, ...) -> *const c_char;
    fn devm_platform_get_and_ioremap_resource(pdev: *mut platform_device, index: c_int, res: *mut *mut resource) -> *mut c_void;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_snd_soc_register_component(dev: *mut device, component: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn samsung_asoc_dma_platform_register(dev: *mut device, filter: dma_filter_fn, tx: *const c_char, rx: *const c_char, parent: *mut device) -> c_int;
    fn idma_reg_addr_init(addr: *mut c_void, idma_addr: u32);
    fn of_property_present(np: *mut device_node, propname: *const c_char) -> bool;
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out: *mut u32) -> c_int;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn of_clk_add_provider(np: *mut device_node, get: *mut c_void, data: *mut c_void) -> c_int;
    fn of_clk_del_provider(np: *mut device_node);
    static mut of_clk_src_onecell_get: c_void;
    fn platform_get_device_id(pdev: *mut platform_device) -> *const platform_device_id;
    fn platform_device_alloc(name: *const c_char, id: c_int) -> *mut platform_device;
    fn platform_device_put(pdev: *mut platform_device);
    fn platform_device_add(pdev: *mut platform_device) -> c_int;
    fn platform_device_unregister(pdev: *mut platform_device);
    fn device_set_driver_override(dev: *mut device, driver: *const c_char) -> c_int;
    fn device_attach(dev: *mut device) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
}

unsafe fn IS_ERR<T>(p: *mut T) -> bool { p as isize >= -4095isize && (p as isize) < 0 }
unsafe fn PTR_ERR<T>(p: *mut T) -> c_int { p as isize as c_int }
fn IS_ENABLED(v: bool) -> bool { v }
unsafe fn WARN_ON(v: bool) -> bool { v }
fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> c_int { N as c_int }
fn FIC_RXCOUNT(reg: u32) -> snd_pcm_sframes_t { (reg & 0x7f) as snd_pcm_sframes_t }
fn FICS_TXCOUNT(reg: u32) -> snd_pcm_sframes_t { (reg & 0x7f) as snd_pcm_sframes_t }
fn PSR_PSVAL(v: u32) -> u32 { v << 8 }
unsafe fn msecs_to_loops(t: c_ulong) -> c_ulong { loops_per_jiffy / 1000 * HZ * t }
unsafe fn reg(base: *mut c_void, off: usize) -> *mut c_void { (base as *mut u8).add(off) as *mut c_void }

/* Returns true if this is the 'overlay' stereo DAI */
unsafe fn is_secondary(i2s: *mut i2s_dai) -> bool {
    (*(*i2s).drv).id == SAMSUNG_I2S_ID_SECONDARY
}

/* If this interface of the controller is transmitting data */
unsafe fn tx_active(i2s: *mut i2s_dai) -> bool {
    let mut active: u32;
    if i2s.is_null() { return false; }
    active = readl(reg((*(*i2s).priv_).addr, I2SCON));
    if is_secondary(i2s) { active &= CON_TXSDMA_ACTIVE; } else { active &= CON_TXDMA_ACTIVE; }
    active != 0
}

/* Return pointer to the other DAI */
unsafe fn get_other_dai(i2s: *mut i2s_dai) -> *mut i2s_dai {
    if !(*i2s).pri_dai.is_null() { (*i2s).pri_dai } else { (*i2s).sec_dai }
}

/* If the other interface of the controller is transmitting data */
unsafe fn other_tx_active(i2s: *mut i2s_dai) -> bool { tx_active(get_other_dai(i2s)) }
/* If any interface of the controller is transmitting data */
unsafe fn any_tx_active(i2s: *mut i2s_dai) -> bool { tx_active(i2s) || other_tx_active(i2s) }

/* If this interface of the controller is receiving data */
unsafe fn rx_active(i2s: *mut i2s_dai) -> bool {
    let active: u32;
    if i2s.is_null() { return false; }
    active = readl(reg((*(*i2s).priv_).addr, I2SCON)) & CON_RXDMA_ACTIVE;
    active != 0
}

/* If the other interface of the controller is receiving data */
unsafe fn other_rx_active(i2s: *mut i2s_dai) -> bool { rx_active(get_other_dai(i2s)) }
/* If any interface of the controller is receiving data */
unsafe fn any_rx_active(i2s: *mut i2s_dai) -> bool { rx_active(i2s) || other_rx_active(i2s) }
/* If the other DAI is transmitting or receiving data */
unsafe fn other_active(i2s: *mut i2s_dai) -> bool { other_rx_active(i2s) || other_tx_active(i2s) }
/* If this DAI is transmitting or receiving data */
unsafe fn this_active(i2s: *mut i2s_dai) -> bool { tx_active(i2s) || rx_active(i2s) }
/* If the controller is active anyway */
unsafe fn any_active(i2s: *mut i2s_dai) -> bool { this_active(i2s) || other_active(i2s) }

unsafe fn to_info(dai: *mut snd_soc_dai) -> *mut i2s_dai {
    let priv_ = snd_soc_dai_get_drvdata(dai) as *mut samsung_i2s_priv;
    (*priv_).dai.add(((*dai).id - 1) as usize)
}

unsafe fn is_opened(i2s: *mut i2s_dai) -> bool {
    !i2s.is_null() && ((*i2s).mode & DAI_OPENED) != 0
}

unsafe fn is_manager(i2s: *mut i2s_dai) -> bool {
    is_opened(i2s) && ((*i2s).mode & DAI_MANAGER) != 0
}

/* Read RCLK of I2S (in multiples of LRCLK) */
unsafe fn get_rfs(i2s: *mut i2s_dai) -> c_uint {
    let priv_ = (*i2s).priv_;
    let mut rfs = readl(reg((*priv_).addr, I2SMOD)) >> (*(*priv_).variant_regs).rfs_off;
    rfs &= (*(*priv_).variant_regs).rfs_mask;
    match rfs { 7 => 192, 6 => 96, 5 => 128, 4 => 64, 3 => 768, 2 => 384, 1 => 512, _ => 256 }
}

/* Write RCLK of I2S (in multiples of LRCLK) */
unsafe fn set_rfs(i2s: *mut i2s_dai, rfs: c_uint) {
    let priv_ = (*i2s).priv_;
    let mut mod_ = readl(reg((*priv_).addr, I2SMOD));
    let rfs_shift = (*(*priv_).variant_regs).rfs_off;
    mod_ &= !((*(*priv_).variant_regs).rfs_mask << rfs_shift);
    mod_ |= match rfs {
        192 => EXYNOS7_MOD_RCLK_192FS << rfs_shift,
        96 => EXYNOS7_MOD_RCLK_96FS << rfs_shift,
        128 => EXYNOS7_MOD_RCLK_128FS << rfs_shift,
        64 => EXYNOS7_MOD_RCLK_64FS << rfs_shift,
        768 => MOD_RCLK_768FS << rfs_shift,
        512 => MOD_RCLK_512FS << rfs_shift,
        384 => MOD_RCLK_384FS << rfs_shift,
        _ => MOD_RCLK_256FS << rfs_shift,
    };
    writel(mod_, reg((*priv_).addr, I2SMOD));
}

/* Read bit-clock of I2S (in multiples of LRCLK) */
unsafe fn get_bfs(i2s: *mut i2s_dai) -> c_uint {
    let priv_ = (*i2s).priv_;
    let mut bfs = readl(reg((*priv_).addr, I2SMOD)) >> (*(*priv_).variant_regs).bfs_off;
    bfs &= (*(*priv_).variant_regs).bfs_mask;
    match bfs { 8 => 256, 7 => 192, 6 => 128, 5 => 96, 4 => 64, 3 => 24, 2 => 16, 1 => 48, _ => 32 }
}

/* Write bit-clock of I2S (in multiples of LRCLK) */
unsafe fn set_bfs(i2s: *mut i2s_dai, bfs: c_uint) {
    let priv_ = (*i2s).priv_;
    let mut mod_ = readl(reg((*priv_).addr, I2SMOD));
    let tdm = (*priv_).quirks & QUIRK_SUPPORTS_TDM;
    let bfs_shift = (*(*priv_).variant_regs).bfs_off;
    /* Non-TDM I2S controllers do not support BCLK > 48 * FS */
    if tdm == 0 && bfs > 48 {
        dev_err(&mut (*(*i2s).pdev).dev, c"Unsupported BCLK divider\n".as_ptr());
        return;
    }
    mod_ &= !((*(*priv_).variant_regs).bfs_mask << bfs_shift);
    match bfs {
        48 => mod_ |= MOD_BCLK_48FS << bfs_shift,
        32 => mod_ |= MOD_BCLK_32FS << bfs_shift,
        24 => mod_ |= MOD_BCLK_24FS << bfs_shift,
        16 => mod_ |= MOD_BCLK_16FS << bfs_shift,
        64 => mod_ |= EXYNOS5420_MOD_BCLK_64FS << bfs_shift,
        96 => mod_ |= EXYNOS5420_MOD_BCLK_96FS << bfs_shift,
        128 => mod_ |= EXYNOS5420_MOD_BCLK_128FS << bfs_shift,
        192 => mod_ |= EXYNOS5420_MOD_BCLK_192FS << bfs_shift,
        256 => mod_ |= EXYNOS5420_MOD_BCLK_256FS << bfs_shift,
        _ => {
            dev_err(&mut (*(*i2s).pdev).dev, c"Wrong BCLK Divider!\n".as_ptr());
            return;
        }
    }
    writel(mod_, reg((*priv_).addr, I2SMOD));
}

/* Sample size */
unsafe fn get_blc(i2s: *mut i2s_dai) -> c_int {
    let mut blc = readl(reg((*(*i2s).priv_).addr, I2SMOD)) as c_int;
    blc = (blc >> 13) & 0x3;
    match blc { 2 => 24, 1 => 8, _ => 16 }
}

/* TX channel control */
unsafe fn i2s_txctrl(i2s: *mut i2s_dai, on: c_int) {
    let priv_ = (*i2s).priv_;
    let addr = (*priv_).addr;
    let txr_off = (*(*priv_).variant_regs).txr_off;
    let mut con = readl(reg(addr, I2SCON));
    let mut mod_ = readl(reg(addr, I2SMOD)) & !(3 << txr_off);
    if on != 0 {
        con |= CON_ACTIVE;
        con &= !CON_TXCH_PAUSE;
        if is_secondary(i2s) {
            con |= CON_TXSDMA_ACTIVE;
            con &= !CON_TXSDMA_PAUSE;
        } else {
            con |= CON_TXDMA_ACTIVE;
            con &= !CON_TXDMA_PAUSE;
        }
        if any_rx_active(i2s) { mod_ |= 2 << txr_off; } else { mod_ |= 0 << txr_off; }
    } else {
        if is_secondary(i2s) {
            con |= CON_TXSDMA_PAUSE;
            con &= !CON_TXSDMA_ACTIVE;
        } else {
            con |= CON_TXDMA_PAUSE;
            con &= !CON_TXDMA_ACTIVE;
        }
        if other_tx_active(i2s) {
            writel(con, reg(addr, I2SCON));
            return;
        }
        con |= CON_TXCH_PAUSE;
        if any_rx_active(i2s) { mod_ |= 1 << txr_off; } else { con &= !CON_ACTIVE; }
    }
    writel(mod_, reg(addr, I2SMOD));
    writel(con, reg(addr, I2SCON));
}

/* RX Channel Control */
unsafe fn i2s_rxctrl(i2s: *mut i2s_dai, on: c_int) {
    let priv_ = (*i2s).priv_;
    let addr = (*priv_).addr;
    let txr_off = (*(*priv_).variant_regs).txr_off;
    let mut con = readl(reg(addr, I2SCON));
    let mut mod_ = readl(reg(addr, I2SMOD)) & !(3 << txr_off);
    if on != 0 {
        con |= CON_RXDMA_ACTIVE | CON_ACTIVE;
        con &= !(CON_RXDMA_PAUSE | CON_RXCH_PAUSE);
        if any_tx_active(i2s) { mod_ |= 2 << txr_off; } else { mod_ |= 1 << txr_off; }
    } else {
        con |= CON_RXDMA_PAUSE | CON_RXCH_PAUSE;
        con &= !CON_RXDMA_ACTIVE;
        if any_tx_active(i2s) { mod_ |= 0 << txr_off; } else { con &= !CON_ACTIVE; }
    }
    writel(mod_, reg(addr, I2SMOD));
    writel(con, reg(addr, I2SCON));
}

/* Flush FIFO of an interface */
unsafe fn i2s_fifo(i2s: *mut i2s_dai, flush: u32) {
    let fic: *mut c_void;
    let mut val: u32;
    if i2s.is_null() { return; }
    if is_secondary(i2s) { fic = reg((*(*i2s).priv_).addr, I2SFICS); } else { fic = reg((*(*i2s).priv_).addr, I2SFIC); }
    /* Flush the FIFO */
    writel(readl(fic) | flush, fic);
    /* Be patient */
    val = (msecs_to_loops(1) / 1000) as u32; /* 1 usec */
    while { val = val.wrapping_sub(1); val != 0 } { cpu_relax(); }
    writel(readl(fic) & !flush, fic);
}

unsafe extern "C" fn i2s_set_sysclk(dai: *mut snd_soc_dai, mut clk_id: c_int, mut rfs: c_uint, dir: c_int) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(dai) as *mut samsung_i2s_priv;
    let i2s = to_info(dai);
    let other = get_other_dai(i2s);
    let i2s_regs = (*priv_).variant_regs;
    let cdcon_mask = 1u32 << (*i2s_regs).cdclkcon_off;
    let rsrc_mask = 1u32 << (*i2s_regs).rclksrc_off;
    let mut mod_: u32 = readl(reg((*priv_).addr, I2SMOD));
    let mask: u32;
    let mut val: u32 = 0;
    let mut ret: c_int;
    match clk_id {
        SAMSUNG_I2S_OPCLK => {
            mask = MOD_OPCLK_MASK;
            val = ((dir as u32) << MOD_OPCLK_SHIFT) & MOD_OPCLK_MASK;
        }
        SAMSUNG_I2S_CDCLK => {
            mask = 1 << (*i2s_regs).cdclkcon_off;
            /* Shouldn't matter in GATING(CLOCK_IN) mode */
            if dir == SND_SOC_CLOCK_IN { rfs = 0; }
            if (rfs != 0 && !other.is_null() && (*other).rfs != 0 && (*other).rfs != rfs)
                || (any_active(i2s)
                    && (((dir == SND_SOC_CLOCK_IN) && (mod_ & cdcon_mask) == 0)
                        || ((dir == SND_SOC_CLOCK_OUT) && (mod_ & cdcon_mask) != 0))) {
                dev_err(&mut (*(*i2s).pdev).dev, c"%s:%d Other DAI busy\n".as_ptr(), c"i2s_set_sysclk".as_ptr(), line!() as c_int);
                return -EAGAIN;
            }
            if dir == SND_SOC_CLOCK_IN { val = 1 << (*i2s_regs).cdclkcon_off; }
            (*i2s).rfs = rfs;
        }
        SAMSUNG_I2S_RCLKSRC_0 | SAMSUNG_I2S_RCLKSRC_1 => {
            mask = 1 << (*i2s_regs).rclksrc_off;
            if ((*priv_).quirks & QUIRK_NO_MUXPSR) != 0 || clk_id == SAMSUNG_I2S_RCLKSRC_0 { clk_id = 0; } else { clk_id = 1; }
            if !any_active(i2s) {
                if !(*priv_).op_clk.is_null() && !IS_ERR((*priv_).op_clk) {
                    if (clk_id != 0 && (mod_ & rsrc_mask) == 0) || (clk_id == 0 && (mod_ & rsrc_mask) != 0) {
                        clk_disable_unprepare((*priv_).op_clk);
                        clk_put((*priv_).op_clk);
                    } else {
                        (*priv_).rclk_srcrate = clk_get_rate((*priv_).op_clk);
                        return 0;
                    }
                }
                (*priv_).op_clk = if clk_id != 0 { clk_get(&mut (*(*i2s).pdev).dev, c"i2s_opclk1".as_ptr()) } else { clk_get(&mut (*(*i2s).pdev).dev, c"i2s_opclk0".as_ptr()) };
                if WARN_ON(IS_ERR((*priv_).op_clk)) {
                    ret = PTR_ERR((*priv_).op_clk);
                    (*priv_).op_clk = null_mut();
                    return ret;
                }
                ret = clk_prepare_enable((*priv_).op_clk);
                if ret != 0 {
                    clk_put((*priv_).op_clk);
                    (*priv_).op_clk = null_mut();
                    return ret;
                }
                (*priv_).rclk_srcrate = clk_get_rate((*priv_).op_clk);
            } else if (clk_id == 0 && (mod_ & rsrc_mask) != 0) || (clk_id != 0 && (mod_ & rsrc_mask) == 0) {
                dev_err(&mut (*(*i2s).pdev).dev, c"%s:%d Other DAI busy\n".as_ptr(), c"i2s_set_sysclk".as_ptr(), line!() as c_int);
                return -EAGAIN;
            } else {
                /* Call can't be on the active DAI */
                return 0;
            }
            if clk_id == 1 { val = 1 << (*i2s_regs).rclksrc_off; }
        }
        _ => {
            dev_err(&mut (*(*i2s).pdev).dev, c"We don't serve that!\n".as_ptr());
            return -EINVAL;
        }
    }
    mod_ = readl(reg((*priv_).addr, I2SMOD));
    mod_ = (mod_ & !mask) | val;
    writel(mod_, reg((*priv_).addr, I2SMOD));
    0
}

unsafe extern "C" fn i2s_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(dai) as *mut samsung_i2s_priv;
    let i2s = to_info(dai);
    let lrp_shift = (*(*priv_).variant_regs).lrp_off;
    let sdf_shift = (*(*priv_).variant_regs).sdf_off;
    let mod_slave = 1u32 << (*(*priv_).variant_regs).mss_off;
    let sdf_mask = (MOD_SDF_MASK as u32) << sdf_shift;
    let lrp_rlow = (MOD_LR_RLOW as u32) << lrp_shift;
    let mut tmp: u32 = 0;
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_RIGHT_J => { tmp |= lrp_rlow; tmp |= (MOD_SDF_MSB as u32) << sdf_shift; }
        SND_SOC_DAIFMT_LEFT_J => { tmp |= lrp_rlow; tmp |= (MOD_SDF_LSB as u32) << sdf_shift; }
        SND_SOC_DAIFMT_I2S => tmp |= (MOD_SDF_IIS as u32) << sdf_shift,
        _ => { dev_err(&mut (*(*i2s).pdev).dev, c"Format not supported\n".as_ptr()); return -EINVAL; }
    }
    /*
     * INV flag is relative to the FORMAT flag - if set it simply
     * flips the polarity specified by the Standard
     */
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_NB_IF => { if (tmp & lrp_rlow) != 0 { tmp &= !lrp_rlow; } else { tmp |= lrp_rlow; } }
        _ => { dev_err(&mut (*(*i2s).pdev).dev, c"Polarity not supported\n".as_ptr()); return -EINVAL; }
    }
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BC_FC => tmp |= mod_slave,
        SND_SOC_DAIFMT_BP_FP => {
            /*
             * Set default source clock in Master mode, only when the
             * CLK_I2S_RCLK_SRC clock is not exposed so we ensure any
             * clock configuration assigned in DT is not overwritten.
             */
            if (*priv_).rclk_srcrate == 0 && (*priv_).clk_data.clks.is_null() {
                i2s_set_sysclk(dai, SAMSUNG_I2S_RCLKSRC_0, 0, SND_SOC_CLOCK_IN);
            }
        }
        _ => { dev_err(&mut (*(*i2s).pdev).dev, c"master/slave format not supported\n".as_ptr()); return -EINVAL; }
    }
    pm_runtime_get_sync((*dai).dev);
    let mut mod_ = readl(reg((*priv_).addr, I2SMOD));
    /*
     * Don't change the I2S mode if any controller is active on this
     * channel.
     */
    if any_active(i2s) && ((mod_ & (sdf_mask | lrp_rlow | mod_slave)) != tmp) {
        pm_runtime_put((*dai).dev);
        dev_err(&mut (*(*i2s).pdev).dev, c"%s:%d Other DAI busy\n".as_ptr(), c"i2s_set_fmt".as_ptr(), line!() as c_int);
        return -EAGAIN;
    }
    mod_ &= !(sdf_mask | lrp_rlow | mod_slave);
    mod_ |= tmp;
    writel(mod_, reg((*priv_).addr, I2SMOD));
    (*priv_).slave_mode = (mod_ & mod_slave) != 0;
    pm_runtime_put((*dai).dev);
    0
}

unsafe extern "C" fn i2s_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(dai) as *mut samsung_i2s_priv;
    let i2s = to_info(dai);
    let mut mask: u32 = 0;
    let mut val: u32 = 0;
    WARN_ON(!pm_runtime_active((*dai).dev));
    if !is_secondary(i2s) { mask |= MOD_DC2_EN | MOD_DC1_EN; }
    match params_channels(params) {
        6 => { val |= MOD_DC2_EN; val |= MOD_DC1_EN; }
        4 => val |= MOD_DC1_EN,
        2 => if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK { (*i2s).dma_playback.addr_width = 4; } else { (*i2s).dma_capture.addr_width = 4; },
        1 => if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK { (*i2s).dma_playback.addr_width = 2; } else { (*i2s).dma_capture.addr_width = 2; },
        _ => { dev_err(&mut (*(*i2s).pdev).dev, c"%d channels not supported\n".as_ptr(), params_channels(params)); return -EINVAL; }
    }
    if is_secondary(i2s) { mask |= MOD_BLCS_MASK; } else { mask |= MOD_BLCP_MASK; }
    if is_manager(i2s) { mask |= MOD_BLC_MASK; }
    match params_width(params) {
        8 => { if is_secondary(i2s) { val |= MOD_BLCS_8BIT; } else { val |= MOD_BLCP_8BIT; } if is_manager(i2s) { val |= MOD_BLC_8BIT; } }
        16 => { if is_secondary(i2s) { val |= MOD_BLCS_16BIT; } else { val |= MOD_BLCP_16BIT; } if is_manager(i2s) { val |= MOD_BLC_16BIT; } }
        24 => { if is_secondary(i2s) { val |= MOD_BLCS_24BIT; } else { val |= MOD_BLCP_24BIT; } if is_manager(i2s) { val |= MOD_BLC_24BIT; } }
        _ => { dev_err(&mut (*(*i2s).pdev).dev, c"Format(%d) not supported\n".as_ptr(), params_format(params)); return -EINVAL; }
    }
    let mut mod_ = readl(reg((*priv_).addr, I2SMOD));
    mod_ = (mod_ & !mask) | val;
    writel(mod_, reg((*priv_).addr, I2SMOD));
    snd_soc_dai_init_dma_data(dai, &mut (*i2s).dma_playback, &mut (*i2s).dma_capture);
    (*i2s).frmclk = params_rate(params);
    let rclksrc = (*priv_).clk_table[CLK_I2S_RCLK_SRC];
    if !rclksrc.is_null() && !IS_ERR(rclksrc) { (*priv_).rclk_srcrate = clk_get_rate(rclksrc); }
    0
}

/* We set constraints on the substream according to the version of I2S */
unsafe extern "C" fn i2s_startup(_substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(dai) as *mut samsung_i2s_priv;
    let i2s = to_info(dai);
    let other = get_other_dai(i2s);
    pm_runtime_get_sync((*dai).dev);
    (*i2s).mode |= DAI_OPENED;
    if is_manager(other) { (*i2s).mode &= !DAI_MANAGER; } else { (*i2s).mode |= DAI_MANAGER; }
    if !any_active(i2s) && ((*priv_).quirks & QUIRK_NEED_RSTCLR) != 0 { writel(CON_RSTCLR, reg((*i2s).priv_.as_mut().unwrap().addr, I2SCON)); }
    0
}

unsafe extern "C" fn i2s_shutdown(_substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    let priv_ = snd_soc_dai_get_drvdata(dai) as *mut samsung_i2s_priv;
    let i2s = to_info(dai);
    let other = get_other_dai(i2s);
    (*i2s).mode &= !DAI_OPENED;
    (*i2s).mode &= !DAI_MANAGER;
    if is_opened(other) { (*other).mode |= DAI_MANAGER; }
    /* Reset any constraint on RFS and BFS */
    (*i2s).rfs = 0;
    (*i2s).bfs = 0;
    pm_runtime_put((*dai).dev);
}

unsafe fn config_setup(i2s: *mut i2s_dai) -> c_int {
    let priv_ = (*i2s).priv_;
    let other = get_other_dai(i2s);
    let blc = get_blc(i2s) as c_uint;
    let mut bfs = (*i2s).bfs;
    if bfs == 0 && !other.is_null() { bfs = (*other).bfs; }
    /* Select least possible multiple(2) if no constraint set */
    if bfs == 0 { bfs = blc * 2; }
    let mut rfs = (*i2s).rfs;
    if rfs == 0 && !other.is_null() { rfs = (*other).rfs; }
    if (rfs == 256 || rfs == 512) && blc == 24 {
        dev_err(&mut (*(*i2s).pdev).dev, c"%d-RFS not supported for 24-blc\n".as_ptr(), rfs);
        return -EINVAL;
    }
    if rfs == 0 { if bfs == 16 || bfs == 32 { rfs = 256; } else { rfs = 384; } }
    /* If already setup and running */
    if any_active(i2s) && (get_rfs(i2s) != rfs || get_bfs(i2s) != bfs) {
        dev_err(&mut (*(*i2s).pdev).dev, c"%s:%d Other DAI busy\n".as_ptr(), c"config_setup".as_ptr(), line!() as c_int);
        return -EAGAIN;
    }
    set_bfs(i2s, bfs);
    set_rfs(i2s, rfs);
    /* Don't bother with PSR in Slave mode */
    if (*priv_).slave_mode { return 0; }
    if ((*priv_).quirks & QUIRK_NO_MUXPSR) == 0 {
        let psr = ((*priv_).rclk_srcrate / (*i2s).frmclk as c_ulong / rfs as c_ulong) as u32;
        writel(((psr - 1) << 8) | PSR_PSREN, reg((*priv_).addr, I2SPSR));
        dev_dbg(&mut (*(*i2s).pdev).dev, c"RCLK_SRC=%luHz PSR=%u, RCLK=%dfs, BCLK=%dfs\n".as_ptr(), (*priv_).rclk_srcrate, psr, rfs, bfs);
    }
    0
}

unsafe extern "C" fn i2s_trigger(substream: *mut snd_pcm_substream, cmd: c_int, dai: *mut snd_soc_dai) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(dai) as *mut samsung_i2s_priv;
    let capture = (*substream).stream == SNDRV_PCM_STREAM_CAPTURE;
    let rtd = snd_soc_substream_to_rtd(substream);
    let i2s = to_info(snd_soc_rtd_to_cpu(rtd, 0));
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            pm_runtime_get_sync((*dai).dev);
            if let Some(f) = (*priv_).fixup_early { f(substream, dai); }
            if config_setup(i2s) != 0 { return -EINVAL; }
            if let Some(f) = (*priv_).fixup_late { f(substream, dai); }
            if capture { i2s_rxctrl(i2s, 1); } else { i2s_txctrl(i2s, 1); }
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            if capture {
                i2s_rxctrl(i2s, 0);
                i2s_fifo(i2s, FIC_RXFLUSH);
            } else {
                i2s_txctrl(i2s, 0);
                i2s_fifo(i2s, FIC_TXFLUSH);
            }
            pm_runtime_put((*dai).dev);
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn i2s_set_clkdiv(dai: *mut snd_soc_dai, div_id: c_int, div: c_int) -> c_int {
    let i2s = to_info(dai);
    let other = get_other_dai(i2s);
    match div_id {
        SAMSUNG_I2S_DIV_BCLK => {
            pm_runtime_get_sync((*dai).dev);
            if (any_active(i2s) && div != 0 && get_bfs(i2s) != div as c_uint)
                || (!other.is_null() && (*other).bfs != 0 && (*other).bfs != div as c_uint) {
                pm_runtime_put((*dai).dev);
                dev_err(&mut (*(*i2s).pdev).dev, c"%s:%d Other DAI busy\n".as_ptr(), c"i2s_set_clkdiv".as_ptr(), line!() as c_int);
                return -EAGAIN;
            }
            (*i2s).bfs = div as c_uint;
            pm_runtime_put((*dai).dev);
        }
        _ => { dev_err(&mut (*(*i2s).pdev).dev, c"Invalid clock divider(%d)\n".as_ptr(), div_id); return -EINVAL; }
    }
    0
}

unsafe extern "C" fn i2s_delay(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> snd_pcm_sframes_t {
    let priv_ = snd_soc_dai_get_drvdata(dai) as *mut samsung_i2s_priv;
    let i2s = to_info(dai);
    let regv = readl(reg((*priv_).addr, I2SFIC));
    WARN_ON(!pm_runtime_active((*dai).dev));
    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        FIC_RXCOUNT(regv)
    } else if is_secondary(i2s) {
        FICS_TXCOUNT(readl(reg((*priv_).addr, I2SFICS)))
    } else {
        ((regv >> (*(*priv_).variant_regs).ftx0cnt_off) & 0x7f) as snd_pcm_sframes_t
    }
}

unsafe extern "C" fn i2s_suspend(component: *mut snd_soc_component) -> c_int { pm_runtime_force_suspend((*component).dev) }
unsafe extern "C" fn i2s_resume(component: *mut snd_soc_component) -> c_int { pm_runtime_force_resume((*component).dev) }

unsafe extern "C" fn samsung_i2s_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(dai) as *mut samsung_i2s_priv;
    let i2s = to_info(dai);
    let other = get_other_dai(i2s);
    pm_runtime_get_sync((*dai).dev);
    if is_secondary(i2s) {
        /* If this is probe on the secondary DAI */
        snd_soc_dai_init_dma_data(dai, &mut (*i2s).dma_playback, null_mut());
    } else {
        snd_soc_dai_init_dma_data(dai, &mut (*i2s).dma_playback, &mut (*i2s).dma_capture);
        if ((*priv_).quirks & QUIRK_NEED_RSTCLR) != 0 { writel(CON_RSTCLR, reg((*priv_).addr, I2SCON)); }
        if ((*priv_).quirks & QUIRK_SUPPORTS_IDMA) != 0 { idma_reg_addr_init((*priv_).addr, (*other).idma_playback.addr); }
    }
    /* Reset any constraint on RFS and BFS */
    (*i2s).rfs = 0;
    (*i2s).bfs = 0;
    i2s_txctrl(i2s, 0);
    i2s_rxctrl(i2s, 0);
    i2s_fifo(i2s, FIC_TXFLUSH);
    i2s_fifo(other, FIC_TXFLUSH);
    i2s_fifo(i2s, FIC_RXFLUSH);
    /* Gate CDCLK by default */
    if !is_opened(other) { i2s_set_sysclk(dai, SAMSUNG_I2S_CDCLK, 0, SND_SOC_CLOCK_IN); }
    pm_runtime_put((*dai).dev);
    0
}

unsafe extern "C" fn samsung_i2s_dai_remove(dai: *mut snd_soc_dai) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(dai) as *mut samsung_i2s_priv;
    let i2s = to_info(dai);
    pm_runtime_get_sync((*dai).dev);
    if !is_secondary(i2s) {
        if ((*priv_).quirks & QUIRK_NEED_RSTCLR) != 0 { writel(0, reg((*priv_).addr, I2SCON)); }
    }
    pm_runtime_put((*dai).dev);
    0
}

static samsung_i2s_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(samsung_i2s_dai_probe),
    remove: Some(samsung_i2s_dai_remove),
    trigger: Some(i2s_trigger),
    hw_params: Some(i2s_hw_params),
    set_fmt: Some(i2s_set_fmt),
    set_clkdiv: Some(i2s_set_clkdiv),
    set_sysclk: Some(i2s_set_sysclk),
    startup: Some(i2s_startup),
    shutdown: Some(i2s_shutdown),
    delay: Some(i2s_delay),
};

// DAPM widgets use construction macros in C; represented as external opaque values.
static samsung_i2s_widgets: [snd_soc_dapm_widget; 3] = [
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
];

static samsung_i2s_dapm_routes: [snd_soc_dapm_route; 3] = [
    snd_soc_dapm_route { sink: c"Playback Mixer".as_ptr(), control: null(), source: c"Primary Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"Mixer DAI TX".as_ptr(), control: null(), source: c"Playback Mixer".as_ptr() },
    snd_soc_dapm_route { sink: c"Primary Capture".as_ptr(), control: null(), source: c"Mixer DAI RX".as_ptr() },
];

static samsung_i2s_dapm_routes_sec_play: [snd_soc_dapm_route; 1] = [
    snd_soc_dapm_route { sink: c"Playback Mixer".as_ptr(), control: null(), source: c"Secondary Playback".as_ptr() },
];

unsafe extern "C" fn samsung_i2s_component_probe(component: *mut snd_soc_component) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut samsung_i2s_priv;
    if ((*priv_).quirks & QUIRK_SEC_DAI) != 0 {
        snd_soc_dapm_add_routes(
            snd_soc_component_to_dapm(component),
            samsung_i2s_dapm_routes_sec_play.as_ptr(),
            ARRAY_SIZE(&samsung_i2s_dapm_routes_sec_play),
        );
    }
    0
}

static samsung_i2s_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"samsung-i2s".as_ptr(),
    probe: Some(samsung_i2s_component_probe),
    dapm_widgets: samsung_i2s_widgets.as_ptr(),
    num_dapm_widgets: 3,
    dapm_routes: samsung_i2s_dapm_routes.as_ptr(),
    num_dapm_routes: 3,
    suspend: Some(i2s_suspend),
    resume: Some(i2s_resume),
    legacy_dai_naming: 1,
};

const SAMSUNG_I2S_FMTS: u64 = SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE;

unsafe fn i2s_alloc_dais(priv_: *mut samsung_i2s_priv, i2s_dai_data: *const samsung_i2s_dai_data, num_dais: c_int) -> c_int {
    static dai_names: [*const c_char; 2] = [c"samsung-i2s".as_ptr(), c"samsung-i2s-sec".as_ptr()];
    static stream_names: [*const c_char; 2] = [c"Primary Playback".as_ptr(), c"Secondary Playback".as_ptr()];
    (*priv_).dai = devm_kcalloc(&mut (*(*priv_).pdev).dev, num_dais as usize, core::mem::size_of::<i2s_dai>(), GFP_KERNEL) as *mut i2s_dai;
    if (*priv_).dai.is_null() { return -ENOMEM; }
    (*priv_).dai_drv = devm_kcalloc(&mut (*(*priv_).pdev).dev, num_dais as usize, core::mem::size_of::<snd_soc_dai_driver>(), GFP_KERNEL) as *mut snd_soc_dai_driver;
    if (*priv_).dai_drv.is_null() { return -ENOMEM; }
    for i in 0..num_dais as usize {
        let dai_drv = (*priv_).dai_drv.add(i);
        (*dai_drv).symmetric_rate = 1;
        (*dai_drv).ops = &samsung_i2s_dai_ops;
        (*dai_drv).playback.channels_min = 1;
        (*dai_drv).playback.channels_max = 2;
        (*dai_drv).playback.rates = (*i2s_dai_data).pcm_rates;
        (*dai_drv).playback.formats = SAMSUNG_I2S_FMTS;
        (*dai_drv).playback.stream_name = stream_names[i];
        (*dai_drv).id = i as c_int + 1;
        (*dai_drv).name = dai_names[i];
        (*(*priv_).dai.add(i)).drv = dai_drv;
        (*(*priv_).dai.add(i)).pdev = (*priv_).pdev;
    }
    /* Initialize capture only for the primary DAI */
    let dai_drv = (*priv_).dai_drv.add((SAMSUNG_I2S_ID_PRIMARY - 1) as usize);
    (*dai_drv).capture.channels_min = 1;
    (*dai_drv).capture.channels_max = 2;
    (*dai_drv).capture.rates = (*i2s_dai_data).pcm_rates;
    (*dai_drv).capture.formats = SAMSUNG_I2S_FMTS;
    (*dai_drv).capture.stream_name = c"Primary Capture".as_ptr();
    0
}

unsafe extern "C" fn i2s_runtime_suspend(dev: *mut device) -> c_int {
    let priv_ = dev_get_drvdata(dev) as *mut samsung_i2s_priv;
    (*priv_).suspend_i2smod = readl(reg((*priv_).addr, I2SMOD));
    (*priv_).suspend_i2scon = readl(reg((*priv_).addr, I2SCON));
    (*priv_).suspend_i2spsr = readl(reg((*priv_).addr, I2SPSR));
    clk_disable_unprepare((*priv_).op_clk);
    clk_disable_unprepare((*priv_).clk);
    0
}

unsafe extern "C" fn i2s_runtime_resume(dev: *mut device) -> c_int {
    let priv_ = dev_get_drvdata(dev) as *mut samsung_i2s_priv;
    let mut ret = clk_prepare_enable((*priv_).clk);
    if ret != 0 { return ret; }
    if !(*priv_).op_clk.is_null() {
        ret = clk_prepare_enable((*priv_).op_clk);
        if ret != 0 {
            clk_disable_unprepare((*priv_).clk);
            return ret;
        }
    }
    writel((*priv_).suspend_i2scon, reg((*priv_).addr, I2SCON));
    writel((*priv_).suspend_i2smod, reg((*priv_).addr, I2SMOD));
    writel((*priv_).suspend_i2spsr, reg((*priv_).addr, I2SPSR));
    0
}

unsafe fn i2s_unregister_clocks(priv_: *mut samsung_i2s_priv) {
    for i in 0..(*priv_).clk_data.clk_num as usize {
        if !IS_ERR((*priv_).clk_table[i]) { clk_unregister((*priv_).clk_table[i]); }
    }
}

unsafe fn i2s_unregister_clock_provider(priv_: *mut samsung_i2s_priv) {
    of_clk_del_provider((*(*priv_).pdev).dev.of_node);
    i2s_unregister_clocks(priv_);
}

unsafe fn i2s_register_clock_provider(priv_: *mut samsung_i2s_priv) -> c_int {
    let i2s_clk_desc: [*const c_char; 3] = [c"cdclk".as_ptr(), c"rclk_src".as_ptr(), c"prescaler".as_ptr()];
    let clk_name: [*const c_char; 2] = [c"i2s_opclk0".as_ptr(), c"i2s_opclk1".as_ptr()];
    let mut p_names: [*const c_char; 2] = [null(), null()];
    let dev = &mut (*(*priv_).pdev).dev as *mut device;
    let reg_info = (*priv_).variant_regs;
    let mut i2s_clk_name: [*const c_char; 3] = [null(), null(), null()];
    /* Register the clock provider only if it's expected in the DTB */
    if !of_property_present((*dev).of_node, c"#clock-cells".as_ptr()) { return 0; }
    /* Get the RCLKSRC mux clock parent clock names */
    for i in 0..p_names.len() {
        let rclksrc = clk_get(dev, clk_name[i]);
        if IS_ERR(rclksrc) { continue; }
        p_names[i] = __clk_get_name(rclksrc);
        clk_put(rclksrc);
    }
    for i in 0..i2s_clk_desc.len() {
        i2s_clk_name[i] = devm_kasprintf(dev, GFP_KERNEL, c"%s_%s".as_ptr(), dev_name(dev), i2s_clk_desc[i]);
        if i2s_clk_name[i].is_null() { return -ENOMEM; }
    }
    if ((*priv_).quirks & QUIRK_NO_MUXPSR) == 0 {
        /* Activate the prescaler */
        let val = readl(reg((*priv_).addr, I2SPSR));
        writel(val | PSR_PSREN, reg((*priv_).addr, I2SPSR));
        (*priv_).clk_table[CLK_I2S_RCLK_SRC] = clk_register_mux(dev, i2s_clk_name[CLK_I2S_RCLK_SRC], p_names.as_ptr(), ARRAY_SIZE(&p_names) as u8, CLK_SET_RATE_NO_REPARENT | CLK_SET_RATE_PARENT, reg((*priv_).addr, I2SMOD), (*reg_info).rclksrc_off as u8, 1, 0, &mut (*priv_).lock);
        (*priv_).clk_table[CLK_I2S_RCLK_PSR] = clk_register_divider(dev, i2s_clk_name[CLK_I2S_RCLK_PSR], i2s_clk_name[CLK_I2S_RCLK_SRC], CLK_SET_RATE_PARENT, reg((*priv_).addr, I2SPSR), 8, 6, 0, &mut (*priv_).lock);
        p_names[0] = i2s_clk_name[CLK_I2S_RCLK_PSR];
        (*priv_).clk_data.clk_num = 2;
    }
    (*priv_).clk_table[CLK_I2S_CDCLK] = clk_register_gate(dev, i2s_clk_name[CLK_I2S_CDCLK], p_names[0], CLK_SET_RATE_PARENT, reg((*priv_).addr, I2SMOD), (*reg_info).cdclkcon_off as u8, CLK_GATE_SET_TO_DISABLE, &mut (*priv_).lock);
    (*priv_).clk_data.clk_num += 1;
    (*priv_).clk_data.clks = (*priv_).clk_table.as_mut_ptr();
    let ret = of_clk_add_provider((*dev).of_node, &mut of_clk_src_onecell_get, &mut (*priv_).clk_data as *mut _ as *mut c_void);
    if ret < 0 {
        dev_err(dev, c"failed to add clock provider: %d\n".as_ptr(), ret);
        i2s_unregister_clocks(priv_);
    }
    ret
}

/* Create platform device for the secondary PCM */
unsafe fn i2s_create_secondary_device(priv_: *mut samsung_i2s_priv) -> c_int {
    let devname = devm_kasprintf(&mut (*(*priv_).pdev).dev, GFP_KERNEL, c"%s-sec".as_ptr(), dev_name(&mut (*(*priv_).pdev).dev));
    if devname.is_null() { return -ENOMEM; }
    let pdev_sec = platform_device_alloc(devname, -1);
    if pdev_sec.is_null() { return -ENOMEM; }
    let mut ret = device_set_driver_override(&mut (*pdev_sec).dev, c"samsung-i2s".as_ptr());
    if ret != 0 { platform_device_put(pdev_sec); return ret; }
    ret = platform_device_add(pdev_sec);
    if ret < 0 { platform_device_put(pdev_sec); return ret; }
    ret = device_attach(&mut (*pdev_sec).dev);
    if ret <= 0 {
        platform_device_unregister((*priv_).pdev_sec);
        dev_info(&mut (*pdev_sec).dev, c"device_attach() failed\n".as_ptr());
        return ret;
    }
    (*priv_).pdev_sec = pdev_sec;
    0
}

unsafe fn i2s_delete_secondary_device(priv_: *mut samsung_i2s_priv) {
    platform_device_unregister((*priv_).pdev_sec);
    (*priv_).pdev_sec = null_mut();
}

unsafe extern "C" fn samsung_i2s_probe(pdev: *mut platform_device) -> c_int {
    let mut sec_dai: *mut i2s_dai = null_mut();
    let i2s_pdata = (*pdev).dev.platform_data as *mut s3c_audio_pdata;
    let mut idma_addr: u32 = 0;
    let np = (*pdev).dev.of_node;
    let i2s_dai_data: *const samsung_i2s_dai_data;
    if IS_ENABLED(true) && !(*pdev).dev.of_node.is_null() {
        i2s_dai_data = of_device_get_match_data(&mut (*pdev).dev) as *const samsung_i2s_dai_data;
    } else {
        let id = platform_get_device_id(pdev);
        /* Nothing to do if it is the secondary device probe */
        if id.is_null() { return 0; }
        i2s_dai_data = (*id).driver_data as *const samsung_i2s_dai_data;
    }
    let priv_ = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<samsung_i2s_priv>(), GFP_KERNEL) as *mut samsung_i2s_priv;
    if priv_.is_null() { return -ENOMEM; }
    if !np.is_null() {
        (*priv_).quirks = (*i2s_dai_data).quirks;
        (*priv_).fixup_early = (*i2s_dai_data).fixup_early;
        (*priv_).fixup_late = (*i2s_dai_data).fixup_late;
    } else {
        if i2s_pdata.is_null() {
            dev_err(&mut (*pdev).dev, c"Missing platform data\n".as_ptr());
            return -EINVAL;
        }
        (*priv_).quirks = (*i2s_pdata).type_.quirks;
    }
    let num_dais = if ((*priv_).quirks & QUIRK_SEC_DAI) != 0 { 2 } else { 1 };
    (*priv_).pdev = pdev;
    (*priv_).variant_regs = (*i2s_dai_data).i2s_variant_regs;
    let mut ret = i2s_alloc_dais(priv_, i2s_dai_data, num_dais);
    if ret < 0 { return ret; }
    let pri_dai = (*priv_).dai.add((SAMSUNG_I2S_ID_PRIMARY - 1) as usize);
    spin_lock_init(&mut (*priv_).lock);
    spin_lock_init(&mut (*priv_).pcm_lock);
    if np.is_null() {
        (*pri_dai).dma_playback.filter_data = (*i2s_pdata).dma_playback;
        (*pri_dai).dma_capture.filter_data = (*i2s_pdata).dma_capture;
        (*pri_dai).filter = (*i2s_pdata).dma_filter;
        idma_addr = (*i2s_pdata).type_.idma_addr;
    } else if of_property_read_u32(np, c"samsung,idma-addr".as_ptr(), &mut idma_addr) != 0 {
        if ((*priv_).quirks & QUIRK_SUPPORTS_IDMA) != 0 {
            dev_info(&mut (*pdev).dev, c"idma address is notspecified".as_ptr());
        }
    }
    let mut res: *mut resource = null_mut();
    (*priv_).addr = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR((*priv_).addr) { return PTR_ERR((*priv_).addr); }
    let regs_base = (*res).start;
    (*priv_).clk = devm_clk_get(&mut (*pdev).dev, c"iis".as_ptr());
    if IS_ERR((*priv_).clk) { return dev_err_probe(&mut (*pdev).dev, PTR_ERR((*priv_).clk), c"Failed to get iis clock\n".as_ptr()); }
    ret = clk_prepare_enable((*priv_).clk);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, c"failed to enable clock: %d\n".as_ptr(), ret);
        return ret;
    }
    (*pri_dai).dma_playback.addr = regs_base + I2STXD;
    (*pri_dai).dma_capture.addr = regs_base + I2SRXD;
    (*pri_dai).dma_playback.chan_name = c"tx".as_ptr();
    (*pri_dai).dma_capture.chan_name = c"rx".as_ptr();
    (*pri_dai).dma_playback.addr_width = 4;
    (*pri_dai).dma_capture.addr_width = 4;
    (*pri_dai).priv_ = priv_;
    if ((*priv_).quirks & QUIRK_PRI_6CHAN) != 0 { (*(*pri_dai).drv).playback.channels_max = 6; }
    ret = samsung_asoc_dma_platform_register(&mut (*pdev).dev, (*pri_dai).filter, c"tx".as_ptr(), c"rx".as_ptr(), null_mut());
    if ret < 0 { clk_disable_unprepare((*priv_).clk); return ret; }
    if ((*priv_).quirks & QUIRK_SEC_DAI) != 0 {
        sec_dai = (*priv_).dai.add((SAMSUNG_I2S_ID_SECONDARY - 1) as usize);
        (*sec_dai).dma_playback.addr = regs_base + I2STXDS;
        (*sec_dai).dma_playback.chan_name = c"tx-sec".as_ptr();
        if np.is_null() {
            (*sec_dai).dma_playback.filter_data = (*i2s_pdata).dma_play_sec;
            (*sec_dai).filter = (*i2s_pdata).dma_filter;
        }
        (*sec_dai).dma_playback.addr_width = 4;
        (*sec_dai).idma_playback.addr = idma_addr;
        (*sec_dai).pri_dai = pri_dai;
        (*sec_dai).priv_ = priv_;
        (*pri_dai).sec_dai = sec_dai;
        ret = i2s_create_secondary_device(priv_);
        if ret < 0 { clk_disable_unprepare((*priv_).clk); return ret; }
        ret = samsung_asoc_dma_platform_register(&mut (*(*priv_).pdev_sec).dev, (*sec_dai).filter, c"tx-sec".as_ptr(), null(), &mut (*pdev).dev);
        if ret < 0 { i2s_delete_secondary_device(priv_); clk_disable_unprepare((*priv_).clk); return ret; }
    }
    if !i2s_pdata.is_null() {
        if let Some(cfg_gpio) = (*i2s_pdata).cfg_gpio {
            if cfg_gpio(pdev) != 0 {
                dev_err(&mut (*pdev).dev, c"Unable to configure gpio\n".as_ptr());
                i2s_delete_secondary_device(priv_);
                clk_disable_unprepare((*priv_).clk);
                return -EINVAL;
            }
        }
    }
    dev_set_drvdata(&mut (*pdev).dev, priv_ as *mut c_void);
    ret = devm_snd_soc_register_component(&mut (*pdev).dev, &samsung_i2s_component, (*priv_).dai_drv, num_dais);
    if ret < 0 { i2s_delete_secondary_device(priv_); clk_disable_unprepare((*priv_).clk); return ret; }
    pm_runtime_set_active(&mut (*pdev).dev);
    pm_runtime_enable(&mut (*pdev).dev);
    ret = i2s_register_clock_provider(priv_);
    if ret < 0 {
        pm_runtime_disable(&mut (*pdev).dev);
        i2s_delete_secondary_device(priv_);
        clk_disable_unprepare((*priv_).clk);
        return ret;
    }
    (*priv_).op_clk = clk_get_parent((*priv_).clk_table[CLK_I2S_RCLK_SRC]);
    0
}

unsafe extern "C" fn samsung_i2s_remove(pdev: *mut platform_device) {
    let priv_ = dev_get_drvdata(&mut (*pdev).dev) as *mut samsung_i2s_priv;
    /* The secondary device has no driver data assigned */
    if priv_.is_null() { return; }
    pm_runtime_get_sync(&mut (*pdev).dev);
    pm_runtime_disable(&mut (*pdev).dev);
    i2s_unregister_clock_provider(priv_);
    i2s_delete_secondary_device(priv_);
    clk_disable_unprepare((*priv_).clk);
    pm_runtime_put_noidle(&mut (*pdev).dev);
}

unsafe extern "C" fn fsd_i2s_fixup_early(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    let rtd = snd_soc_substream_to_rtd(substream);
    let i2s = to_info(snd_soc_rtd_to_cpu(rtd, 0));
    let other = get_other_dai(i2s);
    if !is_opened(other) {
        i2s_set_sysclk(dai, SAMSUNG_I2S_CDCLK, 0, SND_SOC_CLOCK_OUT);
        i2s_set_sysclk(dai, SAMSUNG_I2S_OPCLK, 0, MOD_OPCLK_PCLK);
    }
}

unsafe extern "C" fn fsd_i2s_fixup_late(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    let rtd = snd_soc_substream_to_rtd(substream);
    let priv_ = snd_soc_dai_get_drvdata(dai) as *mut samsung_i2s_priv;
    let i2s = to_info(snd_soc_rtd_to_cpu(rtd, 0));
    let other = get_other_dai(i2s);
    if !is_opened(other) { writel(PSR_PSVAL(2) | PSR_PSREN, reg((*priv_).addr, I2SPSR)); }
}

static i2sv3_regs: samsung_i2s_variant_regs = samsung_i2s_variant_regs { bfs_off: 1, rfs_off: 3, sdf_off: 5, txr_off: 8, rclksrc_off: 10, mss_off: 11, cdclkcon_off: 12, lrp_off: 7, bfs_mask: 0x3, rfs_mask: 0x3, ftx0cnt_off: 8 };
static i2sv6_regs: samsung_i2s_variant_regs = samsung_i2s_variant_regs { bfs_off: 0, rfs_off: 4, sdf_off: 6, txr_off: 8, rclksrc_off: 10, mss_off: 11, cdclkcon_off: 12, lrp_off: 15, bfs_mask: 0xf, rfs_mask: 0x3, ftx0cnt_off: 8 };
static i2sv7_regs: samsung_i2s_variant_regs = samsung_i2s_variant_regs { bfs_off: 0, rfs_off: 4, sdf_off: 7, txr_off: 9, rclksrc_off: 11, mss_off: 12, cdclkcon_off: 22, lrp_off: 15, bfs_mask: 0xf, rfs_mask: 0x7, ftx0cnt_off: 0 };
static i2sv5_i2s1_regs: samsung_i2s_variant_regs = samsung_i2s_variant_regs { bfs_off: 0, rfs_off: 3, sdf_off: 6, txr_off: 8, rclksrc_off: 10, mss_off: 11, cdclkcon_off: 12, lrp_off: 15, bfs_mask: 0x7, rfs_mask: 0x7, ftx0cnt_off: 8 };

static i2sv3_dai_type: samsung_i2s_dai_data = samsung_i2s_dai_data { quirks: QUIRK_NO_MUXPSR, pcm_rates: SNDRV_PCM_RATE_8000_96000, i2s_variant_regs: &i2sv3_regs, fixup_early: None, fixup_late: None };
static i2sv5_dai_type: samsung_i2s_dai_data = samsung_i2s_dai_data { quirks: QUIRK_PRI_6CHAN | QUIRK_SEC_DAI | QUIRK_NEED_RSTCLR | QUIRK_SUPPORTS_IDMA, pcm_rates: SNDRV_PCM_RATE_8000_96000, i2s_variant_regs: &i2sv3_regs, fixup_early: None, fixup_late: None };
static i2sv6_dai_type: samsung_i2s_dai_data = samsung_i2s_dai_data { quirks: QUIRK_PRI_6CHAN | QUIRK_SEC_DAI | QUIRK_NEED_RSTCLR | QUIRK_SUPPORTS_TDM | QUIRK_SUPPORTS_IDMA, pcm_rates: SNDRV_PCM_RATE_8000_96000, i2s_variant_regs: &i2sv6_regs, fixup_early: None, fixup_late: None };
static i2sv7_dai_type: samsung_i2s_dai_data = samsung_i2s_dai_data { quirks: QUIRK_PRI_6CHAN | QUIRK_NEED_RSTCLR | QUIRK_SUPPORTS_TDM, pcm_rates: SNDRV_PCM_RATE_8000_192000, i2s_variant_regs: &i2sv7_regs, fixup_early: None, fixup_late: None };
static i2sv5_dai_type_i2s1: samsung_i2s_dai_data = samsung_i2s_dai_data { quirks: QUIRK_PRI_6CHAN | QUIRK_NEED_RSTCLR, pcm_rates: SNDRV_PCM_RATE_8000_96000, i2s_variant_regs: &i2sv5_i2s1_regs, fixup_early: None, fixup_late: None };
static fsd_dai_type: samsung_i2s_dai_data = samsung_i2s_dai_data { quirks: QUIRK_SEC_DAI | QUIRK_NEED_RSTCLR | QUIRK_SUPPORTS_TDM, pcm_rates: SNDRV_PCM_RATE_8000_192000, i2s_variant_regs: &i2sv7_regs, fixup_early: Some(fsd_i2s_fixup_early), fixup_late: Some(fsd_i2s_fixup_late) };

static samsung_i2s_driver_ids: [platform_device_id; 2] = [
    platform_device_id { name: [b's' as c_char, b'a' as c_char, b'm' as c_char, b's' as c_char, b'u' as c_char, b'n' as c_char, b'g' as c_char, b'-' as c_char, b'i' as c_char, b'2' as c_char, b's' as c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0], driver_data: &i2sv3_dai_type as *const _ as kernel_ulong_t },
    platform_device_id { name: [0; 20], driver_data: 0 },
];
// MODULE_DEVICE_TABLE(platform, samsung_i2s_driver_ids);

// CONFIG_OF: device-tree match table.
static exynos_i2s_match: [of_device_id; 7] = [
    of_device_id { compatible: c"samsung,s3c6410-i2s".as_ptr(), data: &i2sv3_dai_type as *const _ as *const c_void },
    of_device_id { compatible: c"samsung,s5pv210-i2s".as_ptr(), data: &i2sv5_dai_type as *const _ as *const c_void },
    of_device_id { compatible: c"samsung,exynos5420-i2s".as_ptr(), data: &i2sv6_dai_type as *const _ as *const c_void },
    of_device_id { compatible: c"samsung,exynos7-i2s".as_ptr(), data: &i2sv7_dai_type as *const _ as *const c_void },
    of_device_id { compatible: c"samsung,exynos7-i2s1".as_ptr(), data: &i2sv5_dai_type_i2s1 as *const _ as *const c_void },
    of_device_id { compatible: c"tesla,fsd-i2s".as_ptr(), data: &fsd_dai_type as *const _ as *const c_void },
    of_device_id { compatible: null(), data: null() },
];
// MODULE_DEVICE_TABLE(of, exynos_i2s_match);

// RUNTIME_PM_OPS(i2s_runtime_suspend, i2s_runtime_resume, NULL)
// SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume)
static samsung_i2s_pm: dev_pm_ops = dev_pm_ops { _private: [] };

static mut samsung_i2s_driver: platform_driver = platform_driver {
    probe: Some(samsung_i2s_probe),
    remove: Some(samsung_i2s_remove),
    id_table: samsung_i2s_driver_ids.as_ptr(),
    driver: device_driver {
        name: c"samsung-i2s".as_ptr(),
        of_match_table: exynos_i2s_match.as_ptr(),
        pm: &samsung_i2s_pm,
    },
};

// module_platform_driver(samsung_i2s_driver);

/* Module information */
// MODULE_AUTHOR("Jaswinder Singh, <jassisinghbrar@gmail.com>");
// MODULE_DESCRIPTION("Samsung I2S Interface");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
