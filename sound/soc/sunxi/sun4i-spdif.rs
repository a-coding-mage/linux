// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ALSA SoC SPDIF Audio Layer
 *
 * Copyright 2015 Andrea Venturi <be17068@iperbole.bo.it>
 * Copyright 2015 Marcus Cooper <codekipper@gmail.com>
 *
 * Based on the Allwinner SDK driver, released under the GPL.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type Bool = bool;
type U8 = u8;
type U32 = u32;
type ResourceSize = c_ulong;
type DmaSlaveBuswidth = c_int;
type SpinlockT = c_ulong;

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
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
pub struct reset_control {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub start: ResourceSize,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub channels: c_int,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_int,
    pub count: c_int,
}

#[repr(C)]
pub struct snd_aes_iec958 {
    pub status: [U8; 24],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub iec958: snd_aes_iec958,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr: ResourceSize,
    pub addr_width: DmaSlaveBuswidth,
    pub maxburst: c_int,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub channels_min: c_int,
    pub channels_max: c_int,
    pub rates: c_uint,
    pub formats: c_ulong,
}

type c_uint = u32;

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub name: *const c_char,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_int,
    pub reg_stride: c_int,
    pub val_bits: c_int,
    pub max_register: c_int,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub access: c_uint,
    pub iface: c_uint,
    pub name: *const c_char,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub legacy_dai_naming: c_int,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub idle: *const c_void,
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

const fn bit(nr: u32) -> u32 {
    1u32 << nr
}

const fn genmask(h: u32, l: u32) -> u32 {
    (!0u32 << l) & (!0u32 >> (31 - h))
}

const SUN4I_SPDIF_CTL: u32 = 0x00;
const fn SUN4I_SPDIF_CTL_MCLKDIV(v: u32) -> u32 {
    v << 4
} /* v even */
const SUN4I_SPDIF_CTL_MCLKOUTEN: u32 = bit(2);
const SUN4I_SPDIF_CTL_GEN: u32 = bit(1);
const SUN4I_SPDIF_CTL_RESET: u32 = bit(0);

const SUN4I_SPDIF_TXCFG: u32 = 0x04;
const SUN4I_SPDIF_TXCFG_SINGLEMOD: u32 = bit(31);
const SUN4I_SPDIF_TXCFG_ASS: u32 = bit(17);
const SUN4I_SPDIF_TXCFG_NONAUDIO: u32 = bit(16);
const fn SUN4I_SPDIF_TXCFG_TXRATIO(v: u32) -> u32 {
    v << 4
}
const SUN4I_SPDIF_TXCFG_TXRATIO_MASK: u32 = genmask(8, 4);
const SUN4I_SPDIF_TXCFG_FMTRVD: u32 = genmask(3, 2);
const SUN4I_SPDIF_TXCFG_FMT16BIT: u32 = 0 << 2;
const SUN4I_SPDIF_TXCFG_FMT20BIT: u32 = 1 << 2;
const SUN4I_SPDIF_TXCFG_FMT24BIT: u32 = 2 << 2;
const SUN4I_SPDIF_TXCFG_CHSTMODE: u32 = bit(1);
const SUN4I_SPDIF_TXCFG_TXEN: u32 = bit(0);

const SUN4I_SPDIF_RXCFG: u32 = 0x08;
const SUN4I_SPDIF_RXCFG_LOCKFLAG: u32 = bit(4);
const SUN4I_SPDIF_RXCFG_CHSTSRC: u32 = bit(3);
const SUN4I_SPDIF_RXCFG_CHSTCP: u32 = bit(1);
const SUN4I_SPDIF_RXCFG_RXEN: u32 = bit(0);

const SUN4I_SPDIF_TXFIFO: u32 = 0x0C;
const SUN4I_SPDIF_RXFIFO: u32 = 0x10;

const SUN4I_SPDIF_FCTL: u32 = 0x14;
const SUN4I_SPDIF_FCTL_FIFOSRC: u32 = bit(31);
const SUN4I_SPDIF_FCTL_FTX: u32 = bit(17);
const SUN4I_SPDIF_FCTL_FRX: u32 = bit(16);
const fn SUN4I_SPDIF_FCTL_TXTL(v: u32) -> u32 {
    v << 8
}
const SUN4I_SPDIF_FCTL_TXTL_MASK: u32 = genmask(12, 8);
const fn SUN4I_SPDIF_FCTL_RXTL(v: u32) -> u32 {
    v << 3
}
const SUN4I_SPDIF_FCTL_RXTL_MASK: u32 = genmask(7, 3);
const SUN4I_SPDIF_FCTL_TXIM: u32 = bit(2);
const fn SUN4I_SPDIF_FCTL_RXOM(v: u32) -> u32 {
    v << 0
}
const SUN4I_SPDIF_FCTL_RXOM_MASK: u32 = genmask(1, 0);

const SUN50I_H6_SPDIF_FCTL: u32 = 0x14;
const SUN50I_H6_SPDIF_FCTL_HUB_EN: u32 = bit(31);
const SUN50I_H6_SPDIF_FCTL_FTX: u32 = bit(30);
const SUN50I_H6_SPDIF_FCTL_FRX: u32 = bit(29);
const fn SUN50I_H6_SPDIF_FCTL_TXTL(v: u32) -> u32 {
    v << 12
}
const SUN50I_H6_SPDIF_FCTL_TXTL_MASK: u32 = genmask(19, 12);
const fn SUN50I_H6_SPDIF_FCTL_RXTL(v: u32) -> u32 {
    v << 4
}
const SUN50I_H6_SPDIF_FCTL_RXTL_MASK: u32 = genmask(10, 4);
const SUN50I_H6_SPDIF_FCTL_TXIM: u32 = bit(2);
const fn SUN50I_H6_SPDIF_FCTL_RXOM(v: u32) -> u32 {
    v << 0
}
const SUN50I_H6_SPDIF_FCTL_RXOM_MASK: u32 = genmask(1, 0);

const SUN4I_SPDIF_FSTA: u32 = 0x18;
const SUN4I_SPDIF_FSTA_TXE: u32 = bit(14);
const SUN4I_SPDIF_FSTA_TXECNTSHT: u32 = 8;
const SUN4I_SPDIF_FSTA_RXA: u32 = bit(6);
const SUN4I_SPDIF_FSTA_RXACNTSHT: u32 = 0;

const SUN4I_SPDIF_INT: u32 = 0x1C;
const SUN4I_SPDIF_INT_RXLOCKEN: u32 = bit(18);
const SUN4I_SPDIF_INT_RXUNLOCKEN: u32 = bit(17);
const SUN4I_SPDIF_INT_RXPARERREN: u32 = bit(16);
const SUN4I_SPDIF_INT_TXDRQEN: u32 = bit(7);
const SUN4I_SPDIF_INT_TXUIEN: u32 = bit(6);
const SUN4I_SPDIF_INT_TXOIEN: u32 = bit(5);
const SUN4I_SPDIF_INT_TXEIEN: u32 = bit(4);
const SUN4I_SPDIF_INT_RXDRQEN: u32 = bit(2);
const SUN4I_SPDIF_INT_RXOIEN: u32 = bit(1);
const SUN4I_SPDIF_INT_RXAIEN: u32 = bit(0);

const SUN4I_SPDIF_ISTA: u32 = 0x20;
const SUN4I_SPDIF_ISTA_RXLOCKSTA: u32 = bit(18);
const SUN4I_SPDIF_ISTA_RXUNLOCKSTA: u32 = bit(17);
const SUN4I_SPDIF_ISTA_RXPARERRSTA: u32 = bit(16);
const SUN4I_SPDIF_ISTA_TXUSTA: u32 = bit(6);
const SUN4I_SPDIF_ISTA_TXOSTA: u32 = bit(5);
const SUN4I_SPDIF_ISTA_TXESTA: u32 = bit(4);
const SUN4I_SPDIF_ISTA_RXOSTA: u32 = bit(1);
const SUN4I_SPDIF_ISTA_RXASTA: u32 = bit(0);

const SUN8I_SPDIF_TXFIFO: u32 = 0x20;
const SUN4I_SPDIF_TXCNT: u32 = 0x24;
const SUN4I_SPDIF_RXCNT: u32 = 0x28;

const SUN4I_SPDIF_TXCHSTA0: u32 = 0x2C;
const fn SUN4I_SPDIF_TXCHSTA0_CLK(v: u32) -> u32 {
    v << 28
}
const fn SUN4I_SPDIF_TXCHSTA0_SAMFREQ(v: u32) -> u32 {
    v << 24
}
const SUN4I_SPDIF_TXCHSTA0_SAMFREQ_MASK: u32 = genmask(27, 24);
const fn SUN4I_SPDIF_TXCHSTA0_CHNUM(v: u32) -> u32 {
    v << 20
}
const SUN4I_SPDIF_TXCHSTA0_CHNUM_MASK: u32 = genmask(23, 20);
const fn SUN4I_SPDIF_TXCHSTA0_SRCNUM(v: u32) -> u32 {
    v << 16
}
const fn SUN4I_SPDIF_TXCHSTA0_CATACOD(v: u32) -> u32 {
    v << 8
}
const fn SUN4I_SPDIF_TXCHSTA0_MODE(v: u32) -> u32 {
    v << 6
}
const fn SUN4I_SPDIF_TXCHSTA0_EMPHASIS(v: u32) -> u32 {
    v << 3
}
const SUN4I_SPDIF_TXCHSTA0_CP: u32 = bit(2);
const SUN4I_SPDIF_TXCHSTA0_AUDIO: u32 = bit(1);
const SUN4I_SPDIF_TXCHSTA0_PRO: u32 = bit(0);

const SUN4I_SPDIF_TXCHSTA1: u32 = 0x30;
const fn SUN4I_SPDIF_TXCHSTA1_CGMSA(v: u32) -> u32 {
    v << 8
}
const fn SUN4I_SPDIF_TXCHSTA1_ORISAMFREQ(v: u32) -> u32 {
    v << 4
}
const SUN4I_SPDIF_TXCHSTA1_ORISAMFREQ_MASK: u32 = genmask(7, 4);
const fn SUN4I_SPDIF_TXCHSTA1_SAMWORDLEN(v: u32) -> u32 {
    v << 1
}
const SUN4I_SPDIF_TXCHSTA1_MAXWORDLEN: u32 = bit(0);

const SUN4I_SPDIF_RXCHSTA0: u32 = 0x34;
const fn SUN4I_SPDIF_RXCHSTA0_CLK(v: u32) -> u32 {
    v << 28
}
const fn SUN4I_SPDIF_RXCHSTA0_SAMFREQ(v: u32) -> u32 {
    v << 24
}
const fn SUN4I_SPDIF_RXCHSTA0_CHNUM(v: u32) -> u32 {
    v << 20
}
const fn SUN4I_SPDIF_RXCHSTA0_SRCNUM(v: u32) -> u32 {
    v << 16
}
const fn SUN4I_SPDIF_RXCHSTA0_CATACOD(v: u32) -> u32 {
    v << 8
}
const fn SUN4I_SPDIF_RXCHSTA0_MODE(v: u32) -> u32 {
    v << 6
}
const fn SUN4I_SPDIF_RXCHSTA0_EMPHASIS(v: u32) -> u32 {
    v << 3
}
const SUN4I_SPDIF_RXCHSTA0_CP: u32 = bit(2);
const SUN4I_SPDIF_RXCHSTA0_AUDIO: u32 = bit(1);
const SUN4I_SPDIF_RXCHSTA0_PRO: u32 = bit(0);

const SUN4I_SPDIF_RXCHSTA1: u32 = 0x38;
const fn SUN4I_SPDIF_RXCHSTA1_CGMSA(v: u32) -> u32 {
    v << 8
}
const fn SUN4I_SPDIF_RXCHSTA1_ORISAMFREQ(v: u32) -> u32 {
    v << 4
}
const fn SUN4I_SPDIF_RXCHSTA1_SAMWORDLEN(v: u32) -> u32 {
    v << 1
}
const SUN4I_SPDIF_RXCHSTA1_MAXWORDLEN: u32 = bit(0);

/* Defines for Sampling Frequency */
const SUN4I_SPDIF_SAMFREQ_44_1KHZ: u32 = 0x0;
const SUN4I_SPDIF_SAMFREQ_NOT_INDICATED: u32 = 0x1;
const SUN4I_SPDIF_SAMFREQ_48KHZ: u32 = 0x2;
const SUN4I_SPDIF_SAMFREQ_32KHZ: u32 = 0x3;
const SUN4I_SPDIF_SAMFREQ_22_05KHZ: u32 = 0x4;
const SUN4I_SPDIF_SAMFREQ_24KHZ: u32 = 0x6;
const SUN4I_SPDIF_SAMFREQ_88_2KHZ: u32 = 0x8;
const SUN4I_SPDIF_SAMFREQ_76_8KHZ: u32 = 0x9;
const SUN4I_SPDIF_SAMFREQ_96KHZ: u32 = 0xa;
const SUN4I_SPDIF_SAMFREQ_176_4KHZ: u32 = 0xc;
const SUN4I_SPDIF_SAMFREQ_192KHZ: u32 = 0xe;

const SUN4I_RATES: c_uint = SNDRV_PCM_RATE_8000_192000;
const SUN4I_FORMATS: c_ulong = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const GFP_KERNEL: c_uint = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 2;
const SNDRV_PCM_FORMAT_S20_3LE: c_int = 4;
const SNDRV_PCM_FORMAT_S24_LE: c_int = 6;
const SNDRV_PCM_FORMAT_S32_LE: c_int = 10;
const DMA_SLAVE_BUSWIDTH_2_BYTES: DmaSlaveBuswidth = 2;
const DMA_SLAVE_BUSWIDTH_4_BYTES: DmaSlaveBuswidth = 4;
const SNDRV_CTL_ELEM_TYPE_IEC958: c_int = 6;
const SNDRV_CTL_ELEM_ACCESS_READ: c_uint = 1;
const SNDRV_CTL_ELEM_IFACE_PCM: c_uint = 2;
const IEC958_AES0_NONAUDIO: U8 = 0x02;
const SNDRV_PCM_RATE_8000_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 1 << SNDRV_PCM_FORMAT_S16_LE;
const SNDRV_PCM_FMTBIT_S20_3LE: c_ulong = 1 << SNDRV_PCM_FORMAT_S20_3LE;
const SNDRV_PCM_FMTBIT_S24_LE: c_ulong = 1 << SNDRV_PCM_FORMAT_S24_LE;
const SNDRV_PCM_FMTBIT_S32_LE: c_ulong = 1 << SNDRV_PCM_FORMAT_S32_LE;

/**
 * struct sun4i_spdif_quirks - Differences between SoC variants.
 *
 * @reg_dac_txdata: TX FIFO offset for DMA config.
 * @has_reset: SoC needs reset deasserted.
 * @val_fctl_ftx: TX FIFO flush bitmask.
 * @mclk_multiplier: ratio of internal MCLK divider
 * @tx_clk_name: name of TX module clock if split clock design
 */
#[repr(C)]
pub struct sun4i_spdif_quirks {
    pub reg_dac_txdata: c_uint,
    pub has_reset: Bool,
    pub val_fctl_ftx: c_uint,
    pub mclk_multiplier: c_uint,
    pub tx_clk_name: *const c_char,
}

#[repr(C)]
pub struct sun4i_spdif_dev {
    pub pdev: *mut platform_device,
    pub spdif_clk: *mut clk,
    pub apb_clk: *mut clk,
    pub rst: *mut reset_control,
    pub cpu_dai_drv: snd_soc_dai_driver,
    pub regmap: *mut regmap,
    pub dma_params_tx: snd_dmaengine_dai_dma_data,
    pub quirks: *const sun4i_spdif_quirks,
    pub lock: SpinlockT,
}

extern "C" {
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_update_bits_check(
        map: *mut regmap,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
        change: *mut Bool,
    ) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_ulong;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_dai_init_dma_data(
        dai: *mut snd_soc_dai,
        playback: *mut snd_dmaengine_dai_dma_data,
        capture: *mut snd_dmaengine_dai_dma_data,
    );
    fn snd_soc_add_dai_controls(
        dai: *mut snd_soc_dai,
        controls: *mut snd_kcontrol_new,
        num_controls: c_uint,
    ) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn spin_lock_init(lock: *mut SpinlockT);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device,
        index: c_uint,
        res: *mut *mut resource,
    ) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> Bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_reset_control_get_exclusive_deasserted(
        dev: *mut device,
        id: *const c_char,
    ) -> *mut reset_control;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_enabled(dev: *mut device) -> Bool;
    fn devm_snd_dmaengine_pcm_register(dev: *mut device, config: *const c_void, flags: c_uint) -> c_int;
    fn pm_runtime_status_suspended(dev: *mut device) -> Bool;
    fn pm_runtime_disable(dev: *mut device);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn spin_lock_irqsave(lock: *mut SpinlockT, flags: c_ulong);
    fn spin_unlock_irqrestore(lock: *mut SpinlockT, flags: c_ulong);
}

unsafe extern "C" fn sun4i_spdif_configure(host: *mut sun4i_spdif_dev) {
    let quirks = (*host).quirks;

    /* soft reset SPDIF */
    regmap_write((*host).regmap, SUN4I_SPDIF_CTL, SUN4I_SPDIF_CTL_RESET);

    /* flush TX FIFO */
    regmap_update_bits(
        (*host).regmap,
        SUN4I_SPDIF_FCTL,
        (*quirks).val_fctl_ftx,
        (*quirks).val_fctl_ftx,
    );

    /* Valid data at the MSB of TXFIFO Register */
    regmap_update_bits((*host).regmap, SUN4I_SPDIF_FCTL, SUN4I_SPDIF_FCTL_TXIM, 0);

    /* clear TX counter */
    regmap_write((*host).regmap, SUN4I_SPDIF_TXCNT, 0);
}

unsafe extern "C" fn sun4i_snd_txctrl_on(
    substream: *mut snd_pcm_substream,
    host: *mut sun4i_spdif_dev,
) {
    if (*(*substream).runtime).channels == 1 {
        regmap_update_bits(
            (*host).regmap,
            SUN4I_SPDIF_TXCFG,
            SUN4I_SPDIF_TXCFG_SINGLEMOD,
            SUN4I_SPDIF_TXCFG_SINGLEMOD,
        );
    }

    /* SPDIF TX ENABLE */
    regmap_update_bits(
        (*host).regmap,
        SUN4I_SPDIF_TXCFG,
        SUN4I_SPDIF_TXCFG_TXEN,
        SUN4I_SPDIF_TXCFG_TXEN,
    );

    /* DRQ ENABLE */
    regmap_update_bits(
        (*host).regmap,
        SUN4I_SPDIF_INT,
        SUN4I_SPDIF_INT_TXDRQEN,
        SUN4I_SPDIF_INT_TXDRQEN,
    );

    /* Global enable */
    regmap_update_bits(
        (*host).regmap,
        SUN4I_SPDIF_CTL,
        SUN4I_SPDIF_CTL_GEN,
        SUN4I_SPDIF_CTL_GEN,
    );
}

unsafe extern "C" fn sun4i_snd_txctrl_off(
    _substream: *mut snd_pcm_substream,
    host: *mut sun4i_spdif_dev,
) {
    /* SPDIF TX DISABLE */
    regmap_update_bits((*host).regmap, SUN4I_SPDIF_TXCFG, SUN4I_SPDIF_TXCFG_TXEN, 0);

    /* DRQ DISABLE */
    regmap_update_bits((*host).regmap, SUN4I_SPDIF_INT, SUN4I_SPDIF_INT_TXDRQEN, 0);

    /* Global disable */
    regmap_update_bits((*host).regmap, SUN4I_SPDIF_CTL, SUN4I_SPDIF_CTL_GEN, 0);
}

unsafe extern "C" fn sun4i_spdif_startup(
    substream: *mut snd_pcm_substream,
    _cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let host = snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut sun4i_spdif_dev;

    if (*substream).stream != SNDRV_PCM_STREAM_PLAYBACK {
        return -EINVAL;
    }

    sun4i_spdif_configure(host);

    0
}

unsafe extern "C" fn sun4i_spdif_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let mut ret: c_int;
    let mut fmt: c_int;
    let rate: c_ulong = params_rate(params);
    let mut mclk_div: U32;
    let mut mclk: c_uint;
    let mut reg_val: U32;
    let host = snd_soc_dai_get_drvdata(cpu_dai) as *mut sun4i_spdif_dev;
    let pdev = (*host).pdev;

    /* Add the PCM and raw data select interface */
    match params_channels(params) {
        1 | 2 => {
            /* PCM mode */
            fmt = 0;
        }
        4 => {
            /* raw data mode */
            fmt = SUN4I_SPDIF_TXCFG_NONAUDIO as c_int;
        }
        _ => return -EINVAL,
    }

    (*host).dma_params_tx.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => {
            fmt |= SUN4I_SPDIF_TXCFG_FMT16BIT as c_int;
            (*host).dma_params_tx.addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
        }
        SNDRV_PCM_FORMAT_S20_3LE => {
            fmt |= SUN4I_SPDIF_TXCFG_FMT20BIT as c_int;
        }
        SNDRV_PCM_FORMAT_S24_LE | SNDRV_PCM_FORMAT_S32_LE => {
            fmt |= SUN4I_SPDIF_TXCFG_FMT24BIT as c_int;
        }
        _ => return -EINVAL,
    }

    match rate {
        22050 | 44100 | 88200 | 176400 => {
            mclk = 22579200;
        }
        24000 | 32000 | 48000 | 96000 | 192000 => {
            mclk = 24576000;
        }
        _ => return -EINVAL,
    }
    mclk = mclk.wrapping_mul((*(*host).quirks).mclk_multiplier);

    ret = clk_set_rate((*host).spdif_clk, mclk as c_ulong);
    if ret < 0 {
        dev_err(
            &mut (*pdev).dev,
            b"Setting SPDIF clock rate for %d Hz failed!\n\0".as_ptr() as *const c_char,
            mclk,
        );
        return ret;
    }

    match rate {
        22050 | 24000 => {
            mclk_div = 8;
        }
        32000 => {
            mclk_div = 6;
        }
        44100 | 48000 => {
            mclk_div = 4;
        }
        88200 | 96000 => {
            mclk_div = 2;
        }
        176400 | 192000 => {
            mclk_div = 1;
        }
        _ => return -EINVAL,
    }
    mclk_div = mclk_div.wrapping_mul((*(*host).quirks).mclk_multiplier);

    reg_val = 0;
    reg_val |= SUN4I_SPDIF_TXCFG_ASS;
    reg_val |= fmt as U32; /* set non audio and bit depth */
    reg_val |= SUN4I_SPDIF_TXCFG_CHSTMODE;
    reg_val |= SUN4I_SPDIF_TXCFG_TXRATIO(mclk_div.wrapping_sub(1));
    regmap_write((*host).regmap, SUN4I_SPDIF_TXCFG, reg_val);

    let _ = substream;
    0
}

unsafe extern "C" fn sun4i_spdif_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let mut ret: c_int = 0;
    let host = snd_soc_dai_get_drvdata(dai) as *mut sun4i_spdif_dev;

    if (*substream).stream != SNDRV_PCM_STREAM_PLAYBACK {
        return -EINVAL;
    }

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            sun4i_snd_txctrl_on(substream, host);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            sun4i_snd_txctrl_off(substream, host);
        }
        _ => {
            ret = -EINVAL;
        }
    }
    ret
}

unsafe extern "C" fn sun4i_spdif_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
    (*uinfo).count = 1;

    0
}

unsafe extern "C" fn sun4i_spdif_get_status_mask(
    _kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let status = (*ucontrol).value.iec958.status.as_mut_ptr();

    *status.add(0) = 0xff;
    *status.add(1) = 0xff;
    *status.add(2) = 0xff;
    *status.add(3) = 0xff;
    *status.add(4) = 0xff;
    *status.add(5) = 0x03;

    0
}

unsafe extern "C" fn sun4i_spdif_get_status(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cpu_dai = snd_kcontrol_chip(kcontrol) as *mut snd_soc_dai;
    let host = snd_soc_dai_get_drvdata(cpu_dai) as *mut sun4i_spdif_dev;
    let status = (*ucontrol).value.iec958.status.as_mut_ptr();
    let flags: c_ulong = 0;
    let mut reg: c_uint = 0;

    spin_lock_irqsave(&mut (*host).lock, flags);

    regmap_read((*host).regmap, SUN4I_SPDIF_TXCHSTA0, &mut reg);

    *status.add(0) = (reg & 0xff) as U8;
    *status.add(1) = ((reg >> 8) & 0xff) as U8;
    *status.add(2) = ((reg >> 16) & 0xff) as U8;
    *status.add(3) = ((reg >> 24) & 0xff) as U8;

    regmap_read((*host).regmap, SUN4I_SPDIF_TXCHSTA1, &mut reg);

    *status.add(4) = (reg & 0xff) as U8;
    *status.add(5) = ((reg >> 8) & 0x3) as U8;

    spin_unlock_irqrestore(&mut (*host).lock, flags);

    0
}

unsafe extern "C" fn sun4i_spdif_set_status(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cpu_dai = snd_kcontrol_chip(kcontrol) as *mut snd_soc_dai;
    let host = snd_soc_dai_get_drvdata(cpu_dai) as *mut sun4i_spdif_dev;
    let status = (*ucontrol).value.iec958.status.as_mut_ptr();
    let flags: c_ulong = 0;
    let mut reg: c_uint;
    let mut chg0: Bool = false;
    let mut chg1: Bool = false;

    spin_lock_irqsave(&mut (*host).lock, flags);

    reg = (*status.add(3) as U32) << 24;
    reg |= (*status.add(2) as U32) << 16;
    reg |= (*status.add(1) as U32) << 8;
    reg |= *status.add(0) as U32;

    regmap_update_bits_check(
        (*host).regmap,
        SUN4I_SPDIF_TXCHSTA0,
        genmask(31, 0),
        reg,
        &mut chg0,
    );

    reg = (*status.add(5) as U32) << 8;
    reg |= *status.add(4) as U32;

    regmap_update_bits_check(
        (*host).regmap,
        SUN4I_SPDIF_TXCHSTA1,
        genmask(9, 0),
        reg,
        &mut chg1,
    );

    reg = SUN4I_SPDIF_TXCFG_CHSTMODE;
    if (*status.add(0) & IEC958_AES0_NONAUDIO) != 0 {
        reg |= SUN4I_SPDIF_TXCFG_NONAUDIO;
    }

    regmap_update_bits(
        (*host).regmap,
        SUN4I_SPDIF_TXCFG,
        SUN4I_SPDIF_TXCFG_CHSTMODE | SUN4I_SPDIF_TXCFG_NONAUDIO,
        reg,
    );

    spin_unlock_irqrestore(&mut (*host).lock, flags);

    (chg0 || chg1) as c_int
}

static mut sun4i_spdif_controls: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new {
        access: SNDRV_CTL_ELEM_ACCESS_READ,
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: b"IEC958 Playback Mask\0".as_ptr() as *const c_char,
        info: Some(sun4i_spdif_info),
        get: Some(sun4i_spdif_get_status_mask),
        put: None,
    },
    snd_kcontrol_new {
        access: 0,
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: b"IEC958 Playback Default\0".as_ptr() as *const c_char,
        info: Some(sun4i_spdif_info),
        get: Some(sun4i_spdif_get_status),
        put: Some(sun4i_spdif_set_status),
    },
];

unsafe extern "C" fn sun4i_spdif_soc_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let host = snd_soc_dai_get_drvdata(dai) as *mut sun4i_spdif_dev;

    snd_soc_dai_init_dma_data(dai, &mut (*host).dma_params_tx, ptr::null_mut());
    snd_soc_add_dai_controls(
        dai,
        sun4i_spdif_controls.as_mut_ptr(),
        sun4i_spdif_controls.len() as c_uint,
    );

    0
}

static sun4i_spdif_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(sun4i_spdif_soc_dai_probe),
    startup: Some(sun4i_spdif_startup),
    trigger: Some(sun4i_spdif_trigger),
    hw_params: Some(sun4i_spdif_hw_params),
};

static sun4i_spdif_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: SUN4I_SPDIF_RXCHSTA1 as c_int,
};

static mut sun4i_spdif_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    playback: snd_soc_pcm_stream {
        channels_min: 1,
        channels_max: 2,
        rates: SUN4I_RATES,
        formats: SUN4I_FORMATS,
    },
    ops: &sun4i_spdif_dai_ops,
    name: b"spdif\0".as_ptr() as *const c_char,
};

static sun4i_a10_spdif_quirks: sun4i_spdif_quirks = sun4i_spdif_quirks {
    reg_dac_txdata: SUN4I_SPDIF_TXFIFO,
    has_reset: false,
    val_fctl_ftx: SUN4I_SPDIF_FCTL_FTX,
    mclk_multiplier: 1,
    tx_clk_name: ptr::null(),
};

static sun6i_a31_spdif_quirks: sun4i_spdif_quirks = sun4i_spdif_quirks {
    reg_dac_txdata: SUN4I_SPDIF_TXFIFO,
    val_fctl_ftx: SUN4I_SPDIF_FCTL_FTX,
    has_reset: true,
    mclk_multiplier: 1,
    tx_clk_name: ptr::null(),
};

static sun8i_h3_spdif_quirks: sun4i_spdif_quirks = sun4i_spdif_quirks {
    reg_dac_txdata: SUN8I_SPDIF_TXFIFO,
    val_fctl_ftx: SUN4I_SPDIF_FCTL_FTX,
    has_reset: true,
    mclk_multiplier: 4,
    tx_clk_name: ptr::null(),
};

static sun50i_h6_spdif_quirks: sun4i_spdif_quirks = sun4i_spdif_quirks {
    reg_dac_txdata: SUN8I_SPDIF_TXFIFO,
    val_fctl_ftx: SUN50I_H6_SPDIF_FCTL_FTX,
    has_reset: true,
    mclk_multiplier: 1,
    tx_clk_name: ptr::null(),
};

static sun55i_a523_spdif_quirks: sun4i_spdif_quirks = sun4i_spdif_quirks {
    reg_dac_txdata: SUN8I_SPDIF_TXFIFO,
    val_fctl_ftx: SUN50I_H6_SPDIF_FCTL_FTX,
    has_reset: true,
    mclk_multiplier: 1,
    tx_clk_name: b"tx\0".as_ptr() as *const c_char,
};

static sun4i_spdif_of_match: [of_device_id; 7] = [
    of_device_id {
        compatible: b"allwinner,sun4i-a10-spdif\0".as_ptr() as *const c_char,
        data: &sun4i_a10_spdif_quirks as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"allwinner,sun6i-a31-spdif\0".as_ptr() as *const c_char,
        data: &sun6i_a31_spdif_quirks as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"allwinner,sun8i-h3-spdif\0".as_ptr() as *const c_char,
        data: &sun8i_h3_spdif_quirks as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"allwinner,sun50i-h6-spdif\0".as_ptr() as *const c_char,
        data: &sun50i_h6_spdif_quirks as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"allwinner,sun50i-h616-spdif\0".as_ptr() as *const c_char,
        /* Essentially the same as the H6, but without RX */
        data: &sun50i_h6_spdif_quirks as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"allwinner,sun55i-a523-spdif\0".as_ptr() as *const c_char,
        /*
         * Almost the same as H6, but has split the TX and RX clocks,
         * has a separate reset bit for the RX side, and has some
         * expanded features for the RX side.
         */
        data: &sun55i_a523_spdif_quirks as *const _ as *const c_void,
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    }, /* sentinel */
];
/* MODULE_DEVICE_TABLE(of, sun4i_spdif_of_match); */

static sun4i_spdif_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"sun4i-spdif\0".as_ptr() as *const c_char,
    legacy_dai_naming: 1,
};

unsafe extern "C" fn sun4i_spdif_runtime_suspend(dev: *mut device) -> c_int {
    let host = dev_get_drvdata(dev) as *mut sun4i_spdif_dev;

    clk_disable_unprepare((*host).spdif_clk);
    clk_disable_unprepare((*host).apb_clk);

    0
}

unsafe extern "C" fn sun4i_spdif_runtime_resume(dev: *mut device) -> c_int {
    let host = dev_get_drvdata(dev) as *mut sun4i_spdif_dev;
    let mut ret: c_int;

    ret = clk_prepare_enable((*host).spdif_clk);
    if ret != 0 {
        return ret;
    }
    ret = clk_prepare_enable((*host).apb_clk);
    if ret != 0 {
        clk_disable_unprepare((*host).spdif_clk);
    }

    ret
}

unsafe extern "C" fn sun4i_spdif_probe(pdev: *mut platform_device) -> c_int {
    let mut host: *mut sun4i_spdif_dev;
    let mut res: *mut resource = ptr::null_mut();
    let mut quirks: *const sun4i_spdif_quirks;
    let mut ret: c_int;
    let base: *mut c_void;
    let mut tx_clk_name: *const c_char = b"spdif\0".as_ptr() as *const c_char;

    dev_dbg(&mut (*pdev).dev, b"Entered %s\n\0".as_ptr() as *const c_char, b"sun4i_spdif_probe\0".as_ptr() as *const c_char);

    host = devm_kzalloc(
        &mut (*pdev).dev,
        size_of::<sun4i_spdif_dev>(),
        GFP_KERNEL,
    ) as *mut sun4i_spdif_dev;
    if host.is_null() {
        return -ENOMEM;
    }

    (*host).pdev = pdev;
    spin_lock_init(&mut (*host).lock);

    /* Initialize this copy of the CPU DAI driver structure */
    memcpy(
        &mut (*host).cpu_dai_drv as *mut _ as *mut c_void,
        &sun4i_spdif_dai as *const _ as *const c_void,
        size_of::<snd_soc_dai_driver>(),
    );
    (*host).cpu_dai_drv.name = dev_name(&mut (*pdev).dev);

    /* Get the addresses */
    base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR(base) {
        return PTR_ERR(base);
    }

    quirks = of_device_get_match_data(&mut (*pdev).dev) as *const sun4i_spdif_quirks;
    if quirks.is_null() {
        dev_err(
            &mut (*pdev).dev,
            b"Failed to determine the quirks to use\n\0".as_ptr() as *const c_char,
        );
        return -ENODEV;
    }
    (*host).quirks = quirks;

    (*host).regmap = devm_regmap_init_mmio(
        &mut (*pdev).dev,
        base,
        &sun4i_spdif_regmap_config,
    );
    if IS_ERR((*host).regmap as *const c_void) {
        return dev_err_probe(
            &mut (*pdev).dev,
            PTR_ERR((*host).regmap as *const c_void),
            b"failed to initialise regmap.\n\0".as_ptr() as *const c_char,
        );
    }

    /* Clocks */
    (*host).apb_clk = devm_clk_get(&mut (*pdev).dev, b"apb\0".as_ptr() as *const c_char);
    if IS_ERR((*host).apb_clk as *const c_void) {
        return dev_err_probe(
            &mut (*pdev).dev,
            PTR_ERR((*host).apb_clk as *const c_void),
            b"failed to get a apb clock.\n\0".as_ptr() as *const c_char,
        );
    }

    if !(*quirks).tx_clk_name.is_null() {
        tx_clk_name = (*quirks).tx_clk_name;
    }
    (*host).spdif_clk = devm_clk_get(&mut (*pdev).dev, tx_clk_name);
    if IS_ERR((*host).spdif_clk as *const c_void) {
        return dev_err_probe(
            &mut (*pdev).dev,
            PTR_ERR((*host).spdif_clk as *const c_void),
            b"failed to get the \"%s\" clock.\n\0".as_ptr() as *const c_char,
            tx_clk_name,
        );
    }

    (*host).dma_params_tx.addr = (*res).start.wrapping_add((*quirks).reg_dac_txdata as ResourceSize);
    (*host).dma_params_tx.maxburst = 8;
    (*host).dma_params_tx.addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;

    platform_set_drvdata(pdev, host as *mut c_void);

    if (*quirks).has_reset {
        (*host).rst = devm_reset_control_get_exclusive_deasserted(&mut (*pdev).dev, ptr::null());
        if IS_ERR((*host).rst as *const c_void) {
            return dev_err_probe(
                &mut (*pdev).dev,
                PTR_ERR((*host).rst as *const c_void),
                b"Failed to get reset\n\0".as_ptr() as *const c_char,
            );
        }
    }

    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &sun4i_spdif_component,
        &mut sun4i_spdif_dai,
        1,
    );
    if ret != 0 {
        return ret;
    }

    pm_runtime_enable(&mut (*pdev).dev);
    if !pm_runtime_enabled(&mut (*pdev).dev) {
        ret = sun4i_spdif_runtime_resume(&mut (*pdev).dev);
        if ret != 0 {
            pm_runtime_disable(&mut (*pdev).dev);
            return ret;
        }
    }

    ret = devm_snd_dmaengine_pcm_register(&mut (*pdev).dev, ptr::null(), 0);
    if ret != 0 {
        if !pm_runtime_status_suspended(&mut (*pdev).dev) {
            sun4i_spdif_runtime_suspend(&mut (*pdev).dev);
        }
        pm_runtime_disable(&mut (*pdev).dev);
        return ret;
    }
    0
}

unsafe extern "C" fn sun4i_spdif_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
    if !pm_runtime_status_suspended(&mut (*pdev).dev) {
        sun4i_spdif_runtime_suspend(&mut (*pdev).dev);
    }
}

static sun4i_spdif_pm: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(sun4i_spdif_runtime_suspend),
    runtime_resume: Some(sun4i_spdif_runtime_resume),
    idle: ptr::null(),
};

static mut sun4i_spdif_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"sun4i-spdif\0".as_ptr() as *const c_char,
        of_match_table: sun4i_spdif_of_match.as_ptr(),
        pm: &sun4i_spdif_pm,
    },
    probe: Some(sun4i_spdif_probe),
    remove: Some(sun4i_spdif_remove),
};

/* module_platform_driver(sun4i_spdif_driver); */

/* MODULE_AUTHOR("Marcus Cooper <codekipper@gmail.com>"); */
/* MODULE_AUTHOR("Andrea Venturi <be17068@iperbole.bo.it>"); */
/* MODULE_DESCRIPTION("Allwinner sun4i SPDIF SoC Interface"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_ALIAS("platform:sun4i-spdif"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
