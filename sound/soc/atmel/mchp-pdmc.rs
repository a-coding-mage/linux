// SPDX-License-Identifier: GPL-2.0
//
// Driver for Microchip Pulse Density Microphone Controller (PDMC) interfaces
//
// Copyright (C) 2019-2022 Microchip Technology Inc. and its subsidiaries
//
// Author: Codrin Ciubotariu <codrin.ciubotariu@microchip.com>

// C dependencies translated as external kernel/ALSA dependencies:
// dt-bindings/sound/microchip,pdmc.h, linux/bitfield.h, linux/clk.h,
// linux/module.h, linux/of.h, linux/pm_runtime.h, linux/regmap.h,
// sound/core.h, sound/dmaengine_pcm.h, sound/pcm_params.h, sound/tlv.h.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type u32 = u32;
type u64 = u64;
type bool_ = bool;
type dma_addr_t = c_ulong;
type irqreturn_t = c_uint;
type atomic_t = c_int;

const fn BIT(n: c_uint) -> u32 {
    1u32 << n
}

const fn GENMASK(h: c_uint, l: c_uint) -> u32 {
    ((!0u32) << l) & ((!0u32) >> (31 - h))
}

const fn FIELD_PREP(mask: u32, val: u32) -> u32 {
    (val << mask.trailing_zeros()) & mask
}

/*
 * ---- PDMC Register map ----
 */
const MCHP_PDMC_CR: c_uint = 0x00; /* Control Register */
const MCHP_PDMC_MR: c_uint = 0x04; /* Mode Register */
const MCHP_PDMC_CFGR: c_uint = 0x08; /* Configuration Register */
const MCHP_PDMC_RHR: c_uint = 0x0C; /* Receive Holding Register */
const MCHP_PDMC_IER: c_uint = 0x14; /* Interrupt Enable Register */
const MCHP_PDMC_IDR: c_uint = 0x18; /* Interrupt Disable Register */
const MCHP_PDMC_IMR: c_uint = 0x1C; /* Interrupt Mask Register */
const MCHP_PDMC_ISR: c_uint = 0x20; /* Interrupt Status Register */
const MCHP_PDMC_VER: c_uint = 0x50; /* Version Register */

/*
 * ---- Control Register (Write-only) ----
 */
const MCHP_PDMC_CR_SWRST: u32 = BIT(0); /* Software Reset */

/*
 * ---- Mode Register (Read/Write) ----
 */
const MCHP_PDMC_MR_PDMCEN_MASK: u32 = GENMASK(3, 0);
const fn MCHP_PDMC_MR_PDMCEN(ch: c_uint) -> u32 {
    BIT(ch) & MCHP_PDMC_MR_PDMCEN_MASK
}

const MCHP_PDMC_MR_OSR_MASK: u32 = GENMASK(17, 16);
const MCHP_PDMC_MR_OSR64: u32 = 1 << 16;
const MCHP_PDMC_MR_OSR128: u32 = 2 << 16;
const MCHP_PDMC_MR_OSR256: u32 = 3 << 16;

const MCHP_PDMC_MR_SINCORDER_MASK: u32 = GENMASK(23, 20);

const MCHP_PDMC_MR_SINC_OSR_MASK: u32 = GENMASK(27, 24);
const MCHP_PDMC_MR_SINC_OSR_DIS: u32 = 0 << 24;
const MCHP_PDMC_MR_SINC_OSR_8: u32 = 1 << 24;
const MCHP_PDMC_MR_SINC_OSR_16: u32 = 2 << 24;
const MCHP_PDMC_MR_SINC_OSR_32: u32 = 3 << 24;
const MCHP_PDMC_MR_SINC_OSR_64: u32 = 4 << 24;
const MCHP_PDMC_MR_SINC_OSR_128: u32 = 5 << 24;
const MCHP_PDMC_MR_SINC_OSR_256: u32 = 6 << 24;

const MCHP_PDMC_MR_CHUNK_MASK: u32 = GENMASK(31, 28);

/*
 * ---- Configuration Register (Read/Write) ----
 */
const MCHP_PDMC_CFGR_BSSEL_MASK: u32 = BIT(0) | BIT(2) | BIT(4) | BIT(6);
const fn MCHP_PDMC_CFGR_BSSEL(ch: c_uint) -> u32 {
    BIT(ch * 2)
}

const MCHP_PDMC_CFGR_PDMSEL_MASK: u32 = BIT(16) | BIT(18) | BIT(20) | BIT(22);
const fn MCHP_PDMC_CFGR_PDMSEL(ch: c_uint) -> u32 {
    BIT(ch * 2 + 16)
}

/*
 * ---- Interrupt Enable/Disable/Mask/Status Registers ----
 */
const MCHP_PDMC_IR_RXRDY: u32 = BIT(0);
const MCHP_PDMC_IR_RXEMPTY: u32 = BIT(1);
const MCHP_PDMC_IR_RXFULL: u32 = BIT(2);
const MCHP_PDMC_IR_RXCHUNK: u32 = BIT(3);
const MCHP_PDMC_IR_RXUDR: u32 = BIT(4);
const MCHP_PDMC_IR_RXOVR: u32 = BIT(5);

/*
 * ---- Version Register (Read-only) ----
 */
const MCHP_PDMC_VER_VERSION: u32 = GENMASK(11, 0);

const MCHP_PDMC_MAX_CHANNELS: usize = 4;
const MCHP_PDMC_DS_NO: usize = 2;
const MCHP_PDMC_EDGE_NO: usize = 2;

/*
 * ---- DMA chunk size allowed ----
 */
const MCHP_PDMC_DMA_8_WORD_CHUNK: c_int = 8;
const MCHP_PDMC_DMA_4_WORD_CHUNK: c_int = 4;
const MCHP_PDMC_DMA_2_WORD_CHUNK: c_int = 2;
const MCHP_PDMC_DMA_1_WORD_CHUNK: c_int = 1;
const fn DMA_BURST_ALIGNED(p: c_int, s: c_int, w: c_int) -> bool {
    p % (s * w) == 0
}

#[repr(C)]
struct mic_map {
    ds_pos: c_int,
    clk_edge: c_int,
}

#[repr(C)]
struct mchp_pdmc_chmap {
    chmap: *mut snd_pcm_chmap_elem,
    dd: *mut mchp_pdmc,
    pcm: *mut snd_pcm,
    kctl: *mut snd_kcontrol,
}

#[repr(C)]
struct mchp_pdmc {
    channel_mic_map: [mic_map; MCHP_PDMC_MAX_CHANNELS],
    dev: *mut device,
    addr: snd_dmaengine_dai_dma_data,
    regmap: *mut regmap,
    pclk: *mut clk,
    gclk: *mut clk,
    pdmcen: u32,
    suspend_irq: u32,
    startup_delay_us: u32,
    mic_no: c_int,
    sinc_order: c_int,
    audio_filter_en: bool_,
    busy_stream: atomic_t,
}

#[repr(C)]
struct device {
    of_node: *mut device_node,
}
#[repr(C)]
struct device_node;
#[repr(C)]
struct regmap;
#[repr(C)]
struct clk;
#[repr(C)]
struct resource {
    start: c_ulong,
}
#[repr(C)]
struct platform_device {
    dev: device,
}
#[repr(C)]
struct snd_soc_component {
    dev: *mut device,
}
#[repr(C)]
struct snd_soc_dai {
    component: *mut snd_soc_component,
}
#[repr(C)]
struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
    number: c_uint,
    next: *mut snd_pcm_substream,
}
#[repr(C)]
struct snd_pcm_runtime {
    channels: c_uint,
    dma_area: *mut u8,
    dma_bytes: c_ulong,
}
#[repr(C)]
struct snd_pcm_stream {
    substream: *mut snd_pcm_substream,
    substream_count: c_uint,
    chmap_kctl: *mut snd_kcontrol,
}
#[repr(C)]
struct snd_pcm {
    streams: [snd_pcm_stream; 2],
    device: c_int,
    card: *mut snd_card,
}
#[repr(C)]
struct snd_card;
#[repr(C)]
struct snd_kcontrol {
    private_value: c_ulong,
    private_free: Option<unsafe extern "C" fn(*mut snd_kcontrol)>,
}
#[repr(C)]
struct snd_ctl_elem_info {
    type_: c_uint,
    count: c_uint,
    value: snd_ctl_elem_info_value,
}
#[repr(C)]
union snd_ctl_elem_info_value {
    integer: snd_ctl_elem_info_integer,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct snd_ctl_elem_info_integer {
    min: c_long,
    max: c_long,
}
type c_long = isize;
#[repr(C)]
struct snd_ctl_elem_value {
    id: snd_ctl_elem_id,
    value: snd_ctl_elem_value_value,
}
#[repr(C)]
struct snd_ctl_elem_id;
#[repr(C)]
union snd_ctl_elem_value_value {
    enumerated: snd_ctl_elem_value_enumerated,
    integer: snd_ctl_elem_value_integer,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct snd_ctl_elem_value_enumerated {
    item: [c_uint; 4],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct snd_ctl_elem_value_integer {
    value: [c_long; 128],
}
#[repr(C)]
struct soc_enum {
    items: c_uint,
    texts: *const *const c_char,
    values: *const c_uint,
    shift_l: c_uint,
}
#[repr(C)]
struct snd_pcm_chmap_elem {
    channels: c_uint,
    map: [c_uint; 8],
}
#[repr(C)]
struct snd_pcm_hw_constraint_list {
    list: *const c_uint,
    count: c_uint,
}
#[repr(C)]
struct snd_pcm_hw_params;
#[repr(C)]
struct snd_soc_pcm_runtime {
    pcm: *mut snd_pcm,
}
#[repr(C)]
struct snd_dmaengine_dai_dma_data {
    addr: dma_addr_t,
    maxburst: c_int,
}
#[repr(C)]
struct snd_dmaengine_pcm_config {
    process: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, c_ulong, c_ulong) -> c_int>,
    prepare_slave_config: Option<unsafe extern "C" fn() -> c_int>,
}
#[repr(C)]
struct snd_kcontrol_new {
    iface: c_uint,
    access: c_uint,
    name: *const c_char,
    info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    tlv: snd_kcontrol_tlv,
    private_value: c_ulong,
    device: c_int,
    count: c_uint,
}
#[repr(C)]
union snd_kcontrol_tlv {
    c: Option<unsafe extern "C" fn(*mut snd_kcontrol, c_int, c_uint, *mut c_uint) -> c_int>,
}
#[repr(C)]
struct snd_soc_component_driver {
    name: *const c_char,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
}
#[repr(C)]
struct snd_soc_dai_ops {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_soc_dai) -> c_int>,
    auto_selectable_formats: *const u64,
    num_auto_selectable_formats: c_uint,
}
#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}
#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rate_min: c_uint,
    rate_max: c_uint,
    rates: c_uint,
    formats: u64,
}
#[repr(C)]
struct regmap_config {
    reg_bits: c_uint,
    reg_stride: c_uint,
    val_bits: c_uint,
    max_register: c_uint,
    readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    precious_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    cache_type: c_uint,
}
#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}
#[repr(C)]
struct dev_pm_ops;
#[repr(C)]
struct platform_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}
#[repr(C)]
struct device_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
    pm: *const dev_pm_ops,
}

extern "C" {
    static mut snd_dmaengine_pcm_prepare_slave_config: Option<unsafe extern "C" fn() -> c_int>;
    static mchp_pdmc_pm_ops: dev_pm_ops;

    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_dai_init_dma_data(dai: *mut snd_soc_dai, playback: *mut c_void, capture: *mut snd_dmaengine_dai_dma_data);
    fn snd_soc_enum_val_to_item(e: *mut soc_enum, val: c_uint) -> c_uint;
    fn snd_soc_enum_item_to_val(e: *mut soc_enum, item: c_uint) -> c_uint;
    fn atomic_read(v: *const atomic_t) -> c_int;
    fn atomic_set(v: *mut atomic_t, i: c_int);
    fn regmap_write(map: *mut regmap, reg: c_uint, val: u32) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut u32) -> c_int;
    fn snd_ctl_get_ioffidx(kcontrol: *mut snd_kcontrol, id: *mut snd_ctl_elem_id) -> c_uint;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn put_user(x: c_uint, ptr: *mut c_uint) -> c_int;
    fn kfree(ptr: *mut c_void);
    fn snd_soc_info_enum_double(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_uint, l: *const snd_pcm_hw_constraint_list) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_physical_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_period_size(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn clk_round_rate(clk: *mut clk, rate: c_ulong) -> c_long;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: u32, val: u32) -> c_int;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: u32) -> c_int;
    fn usleep_range(min: u32, max: u32);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn WARN_ON(condition: bool) -> bool;
    fn snd_ctl_new1(knew: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn of_property_count_u32_elems(np: *mut device_node, propname: *const c_char) -> c_int;
    fn of_property_read_u32_index(np: *mut device_node, propname: *const c_char, index: c_uint, out_value: *mut c_int) -> c_int;
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out_value: *mut u32) -> c_int;
    fn samples_to_bytes(runtime: *mut snd_pcm_runtime, samples: c_uint) -> c_uint;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn platform_get_irq(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_platform_get_and_ioremap_resource(pdev: *mut platform_device, index: c_uint, res: *mut *mut resource) -> *mut c_void;
    fn devm_regmap_init_mmio(dev: *mut device, regs: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_enabled(dev: *mut device) -> bool;
    fn pm_runtime_status_suspended(dev: *mut device) -> bool;
    fn pm_runtime_disable(dev: *mut device);
    fn devm_snd_dmaengine_pcm_register(dev: *mut device, config: *const snd_dmaengine_pcm_config, flags: c_uint) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
}

const EINVAL: c_int = 22;
const EBUSY: c_int = 16;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const EFAULT: c_int = 14;
const GFP_KERNEL: c_uint = 0;
const SNDRV_PCM_STREAM_CAPTURE: usize = 1;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 2;
const SNDRV_CHMAP_MONO: c_uint = 1;
const SNDRV_CHMAP_FL: c_uint = 3;
const SNDRV_CHMAP_FR: c_uint = 4;
const SNDRV_CHMAP_RL: c_uint = 5;
const SNDRV_CHMAP_RR: c_uint = 6;
const SNDRV_CTL_TLVT_CONTAINER: c_uint = 0;
const SNDRV_CTL_TLVT_CHMAP_VAR: c_uint = 0x101;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const SNDRV_CTL_ELEM_IFACE_PCM: c_uint = 3;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 0x3;
const SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint = 0x40000;
const SNDRV_CTL_ELEM_ACCESS_TLV_CALLBACK: c_uint = 0x80000;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_uint = 10;
const SND_SOC_DAIFMT_MASTER_MASK: c_uint = 0xf00;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x00f;
const SND_SOC_DAIFMT_BP_FP: c_uint = 0x100;
const SND_SOC_DAIFMT_BP_FC: c_uint = 0x200;
const SND_SOC_DAIFMT_PDM: c_uint = 0x000b;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 1;
const SNDRV_PCM_TRIGGER_START: c_int = 2;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 3;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 4;
const SNDRV_PCM_TRIGGER_STOP: c_int = 5;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 6;
const SND_SOC_POSSIBLE_DAIFMT_PDM: u64 = 1;
const SNDRV_PCM_RATE_KNOT: c_uint = 1 << 14;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 6;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const REGCACHE_FLAT: c_uint = 1;
const MCHP_PDMC_CLK_POSITIVE: c_int = 0;
const MCHP_PDMC_CLK_NEGATIVE: c_int = 1;

static mchp_pdmc_sinc_filter_order_text: [*const c_char; 5] = [
    b"1\0".as_ptr() as *const c_char,
    b"2\0".as_ptr() as *const c_char,
    b"3\0".as_ptr() as *const c_char,
    b"4\0".as_ptr() as *const c_char,
    b"5\0".as_ptr() as *const c_char,
];

static mchp_pdmc_sinc_filter_order_values: [c_uint; 5] = [1, 2, 3, 4, 5];

static mchp_pdmc_sinc_filter_order_enum: soc_enum = soc_enum {
    items: mchp_pdmc_sinc_filter_order_text.len() as c_uint,
    texts: mchp_pdmc_sinc_filter_order_text.as_ptr(),
    values: mchp_pdmc_sinc_filter_order_values.as_ptr(),
    shift_l: 0,
};

unsafe extern "C" fn mchp_pdmc_sinc_order_get(kcontrol: *mut snd_kcontrol, uvalue: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let dd = snd_soc_component_get_drvdata(component) as *mut mchp_pdmc;
    let e = (*kcontrol).private_value as *mut soc_enum;
    let item: c_uint;

    item = snd_soc_enum_val_to_item(e, (*dd).sinc_order as c_uint);
    (*uvalue).value.enumerated.item[0] = item;

    0
}

unsafe extern "C" fn mchp_pdmc_sinc_order_put(kcontrol: *mut snd_kcontrol, uvalue: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let dd = snd_soc_component_get_drvdata(component) as *mut mchp_pdmc;
    let e = (*kcontrol).private_value as *mut soc_enum;
    let item = (*uvalue).value.enumerated.item.as_mut_ptr();
    let val: c_uint;

    if *item.add(0) >= (*e).items {
        return -EINVAL;
    }

    val = snd_soc_enum_item_to_val(e, *item.add(0)) << (*e).shift_l;

    if atomic_read(&(*dd).busy_stream) != 0 {
        return -EBUSY;
    }

    if val as c_int == (*dd).sinc_order {
        return 0;
    }

    (*dd).sinc_order = val as c_int;

    1
}

unsafe extern "C" fn mchp_pdmc_af_get(kcontrol: *mut snd_kcontrol, uvalue: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let dd = snd_soc_component_get_drvdata(component) as *mut mchp_pdmc;

    (*uvalue).value.integer.value[0] = ((*dd).audio_filter_en as c_int != 0) as c_long;

    0
}

unsafe extern "C" fn mchp_pdmc_af_put(kcontrol: *mut snd_kcontrol, uvalue: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let dd = snd_soc_component_get_drvdata(component) as *mut mchp_pdmc;
    let af = if (*uvalue).value.integer.value[0] != 0 { true } else { false };

    if atomic_read(&(*dd).busy_stream) != 0 {
        return -EBUSY;
    }

    if (*dd).audio_filter_en == af {
        return 0;
    }

    (*dd).audio_filter_en = af;

    1
}

unsafe extern "C" fn mchp_pdmc_chmap_ctl_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let info = snd_kcontrol_chip(kcontrol) as *mut mchp_pdmc_chmap;

    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = (*(*info).dd).mic_no as c_uint;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = SNDRV_CHMAP_RR as c_long; /* maxmimum 4 channels */
    0
}

unsafe fn mchp_pdmc_chmap_substream(info: *mut mchp_pdmc_chmap, idx: c_uint) -> *mut snd_pcm_substream {
    let mut s: *mut snd_pcm_substream;

    s = (*(*info).pcm).streams[SNDRV_PCM_STREAM_CAPTURE].substream;
    while !s.is_null() {
        if (*s).number == idx {
            return s;
        }
        s = (*s).next;
    }
    ptr::null_mut()
}

unsafe fn mchp_pdmc_chmap_get(substream: *mut snd_pcm_substream, ch_info: *mut mchp_pdmc_chmap) -> *mut snd_pcm_chmap_elem {
    let mut map: *mut snd_pcm_chmap_elem;

    map = (*ch_info).chmap;
    while (*map).channels != 0 {
        if (*map).channels == (*(*substream).runtime).channels {
            return map;
        }
        map = map.add(1);
    }
    ptr::null_mut()
}

unsafe extern "C" fn mchp_pdmc_chmap_ctl_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let info = snd_kcontrol_chip(kcontrol) as *mut mchp_pdmc_chmap;
    let dd = (*info).dd;
    let idx = snd_ctl_get_ioffidx(kcontrol, &mut (*ucontrol).id);
    let substream: *mut snd_pcm_substream;
    let map: *const snd_pcm_chmap_elem;
    let mut i: c_int;
    let mut cfgr_val: u32 = 0;

    if (*info).chmap.is_null() {
        return -EINVAL;
    }
    substream = mchp_pdmc_chmap_substream(info, idx);
    if substream.is_null() {
        return -ENODEV;
    }
    memset((*ucontrol).value.integer.value.as_mut_ptr() as *mut c_void, 0, size_of::<c_long>() * (*(*info).dd).mic_no as usize);
    if (*substream).runtime.is_null() {
        return 0; /* no channels set */
    }

    map = mchp_pdmc_chmap_get(substream, info);
    if map.is_null() {
        return -EINVAL;
    }

    i = 0;
    while i < (*map).channels as c_int {
        let map_idx = if (*map).channels == 1 {
            (*map).map[i as usize] as c_int - SNDRV_CHMAP_MONO as c_int
        } else {
            (*map).map[i as usize] as c_int - SNDRV_CHMAP_FL as c_int
        };

        /* make sure the reported channel map is the real one, so write the map */
        if (*dd).channel_mic_map[map_idx as usize].ds_pos != 0 {
            cfgr_val |= MCHP_PDMC_CFGR_PDMSEL(i as c_uint);
        }
        if (*dd).channel_mic_map[map_idx as usize].clk_edge != 0 {
            cfgr_val |= MCHP_PDMC_CFGR_BSSEL(i as c_uint);
        }

        (*ucontrol).value.integer.value[i as usize] = (*map).map[i as usize] as c_long;
        i += 1;
    }

    regmap_write((*dd).regmap, MCHP_PDMC_CFGR, cfgr_val);

    0
}

unsafe extern "C" fn mchp_pdmc_chmap_ctl_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let info = snd_kcontrol_chip(kcontrol) as *mut mchp_pdmc_chmap;
    let dd = (*info).dd;
    let idx = snd_ctl_get_ioffidx(kcontrol, &mut (*ucontrol).id);
    let substream: *mut snd_pcm_substream;
    let map: *mut snd_pcm_chmap_elem;
    let mut cfgr_val: u32 = 0;
    let mut i: c_int;

    if (*info).chmap.is_null() {
        return -EINVAL;
    }
    substream = mchp_pdmc_chmap_substream(info, idx);
    if substream.is_null() {
        return -ENODEV;
    }

    if (*substream).runtime.is_null() {
        return 0; /* just for avoiding error from alsactl restore */
    }

    map = mchp_pdmc_chmap_get(substream, info);
    if map.is_null() {
        return -EINVAL;
    }

    i = 0;
    while i < (*map).channels as c_int {
        let map_idx: c_int;

        (*map).map[i as usize] = (*ucontrol).value.integer.value[i as usize] as c_uint;
        map_idx = if (*map).channels == 1 {
            (*map).map[i as usize] as c_int - SNDRV_CHMAP_MONO as c_int
        } else {
            (*map).map[i as usize] as c_int - SNDRV_CHMAP_FL as c_int
        };

        /* configure IP for the desired channel map */
        if (*dd).channel_mic_map[map_idx as usize].ds_pos != 0 {
            cfgr_val |= MCHP_PDMC_CFGR_PDMSEL(i as c_uint);
        }
        if (*dd).channel_mic_map[map_idx as usize].clk_edge != 0 {
            cfgr_val |= MCHP_PDMC_CFGR_BSSEL(i as c_uint);
        }
        i += 1;
    }

    regmap_write((*dd).regmap, MCHP_PDMC_CFGR, cfgr_val);

    0
}

unsafe extern "C" fn mchp_pdmc_chmap_ctl_private_free(kcontrol: *mut snd_kcontrol) {
    let info = snd_kcontrol_chip(kcontrol) as *mut mchp_pdmc_chmap;

    (*(*info).pcm).streams[SNDRV_PCM_STREAM_CAPTURE].chmap_kctl = ptr::null_mut();
    kfree(info as *mut c_void);
}

unsafe extern "C" fn mchp_pdmc_chmap_ctl_tlv(kcontrol: *mut snd_kcontrol, _op_flag: c_int, mut size: c_uint, tlv: *mut c_uint) -> c_int {
    let info = snd_kcontrol_chip(kcontrol) as *mut mchp_pdmc_chmap;
    let mut map: *const snd_pcm_chmap_elem;
    let mut dst: *mut c_uint;
    let mut c: c_int;
    let mut count: c_int = 0;

    if (*info).chmap.is_null() {
        return -EINVAL;
    }
    if size < 8 {
        return -ENOMEM;
    }
    if put_user(SNDRV_CTL_TLVT_CONTAINER, tlv) != 0 {
        return -EFAULT;
    }
    size -= 8;
    dst = tlv.add(2);
    map = (*info).chmap;
    while (*map).channels != 0 {
        let chs_bytes = (*map).channels * 4;

        if size < 8 {
            return -ENOMEM;
        }
        if put_user(SNDRV_CTL_TLVT_CHMAP_VAR, dst) != 0 || put_user(chs_bytes, dst.add(1)) != 0 {
            return -EFAULT;
        }
        dst = dst.add(2);
        size -= 8;
        count += 8;
        if size < chs_bytes {
            return -ENOMEM;
        }
        size -= chs_bytes;
        count += chs_bytes as c_int;
        c = 0;
        while c < (*map).channels as c_int {
            if put_user((*map).map[c as usize], dst) != 0 {
                return -EFAULT;
            }
            dst = dst.add(1);
            c += 1;
        }
        map = map.add(1);
    }
    if put_user(count as c_uint, tlv.add(1)) != 0 {
        return -EFAULT;
    }
    0
}

// SOC_SINGLE_BOOL_EXT("Audio Filter", 0, &mchp_pdmc_af_get, &mchp_pdmc_af_put)
static mchp_pdmc_snd_controls: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        access: 0,
        name: b"Audio Filter\0".as_ptr() as *const c_char,
        info: None,
        get: Some(mchp_pdmc_af_get),
        put: Some(mchp_pdmc_af_put),
        tlv: snd_kcontrol_tlv { c: None },
        private_value: 0,
        device: 0,
        count: 0,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        access: 0,
        name: b"SINC Filter Order\0".as_ptr() as *const c_char,
        info: Some(snd_soc_info_enum_double),
        get: Some(mchp_pdmc_sinc_order_get),
        put: Some(mchp_pdmc_sinc_order_put),
        tlv: snd_kcontrol_tlv { c: None },
        private_value: &mchp_pdmc_sinc_filter_order_enum as *const soc_enum as c_ulong,
        device: 0,
        count: 0,
    },
];

static mchp_pdmc_dai_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"mchp-pdmc\0".as_ptr() as *const c_char,
    controls: mchp_pdmc_snd_controls.as_ptr(),
    num_controls: mchp_pdmc_snd_controls.len() as c_uint,
};

static mchp_pdmc_1mic: [c_uint; 1] = [1];
static mchp_pdmc_2mic: [c_uint; 2] = [1, 2];
static mchp_pdmc_3mic: [c_uint; 3] = [1, 2, 3];
static mchp_pdmc_4mic: [c_uint; 4] = [1, 2, 3, 4];

static mchp_pdmc_chan_constr: [snd_pcm_hw_constraint_list; 4] = [
    snd_pcm_hw_constraint_list { list: mchp_pdmc_1mic.as_ptr(), count: mchp_pdmc_1mic.len() as c_uint },
    snd_pcm_hw_constraint_list { list: mchp_pdmc_2mic.as_ptr(), count: mchp_pdmc_2mic.len() as c_uint },
    snd_pcm_hw_constraint_list { list: mchp_pdmc_3mic.as_ptr(), count: mchp_pdmc_3mic.len() as c_uint },
    snd_pcm_hw_constraint_list { list: mchp_pdmc_4mic.as_ptr(), count: mchp_pdmc_4mic.len() as c_uint },
];

unsafe extern "C" fn mchp_pdmc_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let dd = snd_soc_dai_get_drvdata(dai) as *mut mchp_pdmc;

    regmap_write((*dd).regmap, MCHP_PDMC_CR, MCHP_PDMC_CR_SWRST);

    snd_pcm_hw_constraint_list(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_CHANNELS,
        &mchp_pdmc_chan_constr[((*dd).mic_no - 1) as usize],
    );

    0
}

unsafe extern "C" fn mchp_pdmc_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let dd = snd_soc_dai_get_drvdata(dai) as *mut mchp_pdmc;

    snd_soc_dai_init_dma_data(dai, ptr::null_mut(), &mut (*dd).addr);

    0
}

unsafe extern "C" fn mchp_pdmc_set_fmt(_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let fmt_master = fmt & SND_SOC_DAIFMT_MASTER_MASK;
    let fmt_format = fmt & SND_SOC_DAIFMT_FORMAT_MASK;

    /* IP needs to be bitclock master */
    if fmt_master != SND_SOC_DAIFMT_BP_FP && fmt_master != SND_SOC_DAIFMT_BP_FC {
        return -EINVAL;
    }

    /* IP supports only PDM interface */
    if fmt_format != SND_SOC_DAIFMT_PDM {
        return -EINVAL;
    }

    0
}

fn mchp_pdmc_mr_set_osr(audio_filter_en: c_int, osr: c_uint) -> u32 {
    if audio_filter_en != 0 {
        match osr {
            64 => return MCHP_PDMC_MR_OSR64,
            128 => return MCHP_PDMC_MR_OSR128,
            256 => return MCHP_PDMC_MR_OSR256,
            _ => {}
        }
    } else {
        match osr {
            8 => return MCHP_PDMC_MR_SINC_OSR_8,
            16 => return MCHP_PDMC_MR_SINC_OSR_16,
            32 => return MCHP_PDMC_MR_SINC_OSR_32,
            64 => return MCHP_PDMC_MR_SINC_OSR_64,
            128 => return MCHP_PDMC_MR_SINC_OSR_128,
            256 => return MCHP_PDMC_MR_SINC_OSR_256,
            _ => {}
        }
    }
    0
}

fn mchp_pdmc_period_to_maxburst(period_size: c_int, sample_size: c_int) -> c_int {
    let p_size = period_size;
    let s_size = sample_size;

    if DMA_BURST_ALIGNED(p_size, s_size, MCHP_PDMC_DMA_8_WORD_CHUNK) {
        return MCHP_PDMC_DMA_8_WORD_CHUNK;
    }
    if DMA_BURST_ALIGNED(p_size, s_size, MCHP_PDMC_DMA_4_WORD_CHUNK) {
        return MCHP_PDMC_DMA_4_WORD_CHUNK;
    }
    if DMA_BURST_ALIGNED(p_size, s_size, MCHP_PDMC_DMA_2_WORD_CHUNK) {
        return MCHP_PDMC_DMA_2_WORD_CHUNK;
    }
    MCHP_PDMC_DMA_1_WORD_CHUNK
}

static mut mchp_pdmc_std_chmaps: [snd_pcm_chmap_elem; 5] = [
    snd_pcm_chmap_elem { channels: 1, map: [SNDRV_CHMAP_MONO, 0, 0, 0, 0, 0, 0, 0] },
    snd_pcm_chmap_elem { channels: 2, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, 0, 0, 0, 0, 0, 0] },
    snd_pcm_chmap_elem { channels: 3, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_RL, 0, 0, 0, 0, 0] },
    snd_pcm_chmap_elem { channels: 4, map: [SNDRV_CHMAP_FL, SNDRV_CHMAP_FR, SNDRV_CHMAP_RL, SNDRV_CHMAP_RR, 0, 0, 0, 0] },
    snd_pcm_chmap_elem { channels: 0, map: [0; 8] },
];

unsafe extern "C" fn mchp_pdmc_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let dd = snd_soc_dai_get_drvdata(dai) as *mut mchp_pdmc;
    let comp = (*dai).component;
    let mut gclk_rate: c_ulong = 0;
    let mut best_diff_rate: c_ulong = !0;
    let channels = params_channels(params);
    let mut osr: c_uint = 0;
    let mut osr_start: c_uint;
    let fs = params_rate(params);
    let sample_bytes = (params_physical_width(params) / 8) as c_int;
    let period_bytes = (params_period_size(params) * params_channels(params) * sample_bytes as c_uint) as c_int;
    let maxburst: c_int;
    let mut mr_val: u32 = 0;
    let mut cfgr_val: u32 = 0;
    let mut i: c_int;
    let ret: c_int;

    dev_dbg((*comp).dev, b"%s() rate=%u format=%#x width=%u channels=%u period_bytes=%d\n\0".as_ptr() as *const c_char,
        b"mchp_pdmc_hw_params\0".as_ptr(), params_rate(params), params_format(params),
        params_width(params), params_channels(params), period_bytes);

    if channels > (*dd).mic_no as c_uint {
        dev_err((*comp).dev, b"more channels %u than microphones %d\n\0".as_ptr() as *const c_char, channels, (*dd).mic_no);
        return -EINVAL;
    }

    (*dd).pdmcen = 0;
    i = 0;
    while i < channels as c_int {
        (*dd).pdmcen |= MCHP_PDMC_MR_PDMCEN(i as c_uint);
        if (*dd).channel_mic_map[i as usize].ds_pos != 0 {
            cfgr_val |= MCHP_PDMC_CFGR_PDMSEL(i as c_uint);
        }
        if (*dd).channel_mic_map[i as usize].clk_edge != 0 {
            cfgr_val |= MCHP_PDMC_CFGR_BSSEL(i as c_uint);
        }
        i += 1;
    }

    /*
     * from these point forward, we consider the controller busy, so the
     * audio filter and SINC order can't be changed
     */
    atomic_set(&mut (*dd).busy_stream, 1);
    osr_start = if (*dd).audio_filter_en { 64 } else { 8 };
    while osr_start <= 256 && best_diff_rate != 0 {
        let round_rate: c_long;
        let diff_rate: c_ulong;

        round_rate = clk_round_rate((*dd).gclk, (fs as c_ulong).wrapping_mul(16).wrapping_mul(osr_start as c_ulong));
        if round_rate < 0 {
            osr_start *= 2;
            continue;
        }
        diff_rate = ((fs * 16 * osr_start) as c_long - round_rate).abs() as c_ulong;
        if diff_rate < best_diff_rate {
            best_diff_rate = diff_rate;
            osr = osr_start;
            gclk_rate = (fs * 16 * osr) as c_ulong;
        }
        osr_start *= 2;
    }
    if gclk_rate == 0 {
        dev_err((*comp).dev, b"invalid sampling rate: %u\n\0".as_ptr() as *const c_char, fs);
        return -EINVAL;
    }

    /* CLK is enabled by runtime PM. */
    clk_disable_unprepare((*dd).gclk);

    /* set the rate */
    ret = clk_set_rate((*dd).gclk, gclk_rate);
    clk_prepare_enable((*dd).gclk);
    if ret != 0 {
        dev_err((*comp).dev, b"unable to set rate %lu to GCLK: %d\n\0".as_ptr() as *const c_char, gclk_rate, ret);
        return ret;
    }

    mr_val |= mchp_pdmc_mr_set_osr((*dd).audio_filter_en as c_int, osr);

    mr_val |= FIELD_PREP(MCHP_PDMC_MR_SINCORDER_MASK, (*dd).sinc_order as u32);

    maxburst = mchp_pdmc_period_to_maxburst(period_bytes, sample_bytes);
    (*dd).addr.maxburst = maxburst;
    mr_val |= FIELD_PREP(MCHP_PDMC_MR_CHUNK_MASK, (*dd).addr.maxburst as u32);
    dev_dbg((*comp).dev, b"maxburst set to %d\n\0".as_ptr() as *const c_char, (*dd).addr.maxburst);

    snd_soc_component_update_bits(
        comp,
        MCHP_PDMC_MR,
        MCHP_PDMC_MR_OSR_MASK | MCHP_PDMC_MR_SINCORDER_MASK | MCHP_PDMC_MR_SINC_OSR_MASK | MCHP_PDMC_MR_CHUNK_MASK,
        mr_val,
    );

    snd_soc_component_write(comp, MCHP_PDMC_CFGR, cfgr_val);

    0
}

unsafe fn mchp_pdmc_noise_filter_workaround(dd: *mut mchp_pdmc) {
    let mut tmp: u32 = 0;
    let mut steps: u32 = 16;

    /*
     * PDMC doesn't wait for microphones' startup time thus the acquisition
     * may start before the microphones are ready leading to poc noises at
     * the beginning of capture. To avoid this, we need to wait 50ms (in
     * normal startup procedure) or 150 ms (worst case after resume from sleep
     * states) after microphones are enabled and then clear the FIFOs (by
     * reading the RHR 16 times) and possible interrupts before continuing.
     * Also, for this to work the DMA needs to be started after interrupts
     * are enabled.
     */
    usleep_range((*dd).startup_delay_us, (*dd).startup_delay_us + 5);

    while steps != 0 {
        steps -= 1;
        regmap_read((*dd).regmap, MCHP_PDMC_RHR, &mut tmp);
    }

    /* Clear interrupts. */
    regmap_read((*dd).regmap, MCHP_PDMC_ISR, &mut tmp);
}

unsafe extern "C" fn mchp_pdmc_trigger(_substream: *mut snd_pcm_substream, cmd: c_int, dai: *mut snd_soc_dai) -> c_int {
    let dd = snd_soc_dai_get_drvdata(dai) as *mut mchp_pdmc;
    let cpu = (*dai).component;

    match cmd {
        SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            snd_soc_component_update_bits(cpu, MCHP_PDMC_MR, MCHP_PDMC_MR_PDMCEN_MASK, (*dd).pdmcen);

            mchp_pdmc_noise_filter_workaround(dd);

            /* Enable interrupts. */
            regmap_write((*dd).regmap, MCHP_PDMC_IER, (*dd).suspend_irq | MCHP_PDMC_IR_RXOVR | MCHP_PDMC_IR_RXUDR);
            (*dd).suspend_irq = 0;
        }
        SNDRV_PCM_TRIGGER_SUSPEND => {
            regmap_read((*dd).regmap, MCHP_PDMC_IMR, &mut (*dd).suspend_irq);
            regmap_write((*dd).regmap, MCHP_PDMC_IDR, (*dd).suspend_irq | MCHP_PDMC_IR_RXOVR | MCHP_PDMC_IR_RXUDR);
            snd_soc_component_update_bits(cpu, MCHP_PDMC_MR, MCHP_PDMC_MR_PDMCEN_MASK, 0);
        }
        SNDRV_PCM_TRIGGER_STOP => {
            /* Disable overrun and underrun error interrupts */
            regmap_write((*dd).regmap, MCHP_PDMC_IDR, (*dd).suspend_irq | MCHP_PDMC_IR_RXOVR | MCHP_PDMC_IR_RXUDR);
            snd_soc_component_update_bits(cpu, MCHP_PDMC_MR, MCHP_PDMC_MR_PDMCEN_MASK, 0);
        }
        SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            snd_soc_component_update_bits(cpu, MCHP_PDMC_MR, MCHP_PDMC_MR_PDMCEN_MASK, 0);
        }
        _ => return -EINVAL,
    }

    // DEBUG-only register dumps from the C source are intentionally preserved as conditional intent.

    0
}

unsafe fn mchp_pdmc_add_chmap_ctls(pcm: *mut snd_pcm, dd: *mut mchp_pdmc) -> c_int {
    let info: *mut mchp_pdmc_chmap;
    let mut knew = snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ | SNDRV_CTL_ELEM_ACCESS_TLV_CALLBACK,
        name: ptr::null(),
        info: Some(mchp_pdmc_chmap_ctl_info),
        get: Some(mchp_pdmc_chmap_ctl_get),
        put: Some(mchp_pdmc_chmap_ctl_put),
        tlv: snd_kcontrol_tlv { c: Some(mchp_pdmc_chmap_ctl_tlv) },
        private_value: 0,
        device: 0,
        count: 0,
    };
    let err: c_int;

    if WARN_ON(!(*pcm).streams[SNDRV_PCM_STREAM_CAPTURE].chmap_kctl.is_null()) {
        return -EBUSY;
    }
    info = kzalloc(size_of::<mchp_pdmc_chmap>(), GFP_KERNEL) as *mut mchp_pdmc_chmap;
    if info.is_null() {
        return -ENOMEM;
    }
    (*info).pcm = pcm;
    (*info).dd = dd;
    (*info).chmap = mchp_pdmc_std_chmaps.as_mut_ptr();
    knew.name = b"Capture Channel Map\0".as_ptr() as *const c_char;
    knew.device = (*pcm).device;
    knew.count = (*pcm).streams[SNDRV_PCM_STREAM_CAPTURE].substream_count;
    (*info).kctl = snd_ctl_new1(&knew, info as *mut c_void);
    if (*info).kctl.is_null() {
        kfree(info as *mut c_void);
        return -ENOMEM;
    }
    (*(*info).kctl).private_free = Some(mchp_pdmc_chmap_ctl_private_free);
    err = snd_ctl_add((*pcm).card, (*info).kctl);
    if err < 0 {
        return err;
    }
    (*pcm).streams[SNDRV_PCM_STREAM_CAPTURE].chmap_kctl = (*info).kctl;
    0
}

unsafe extern "C" fn mchp_pdmc_pcm_new(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int {
    let dd = snd_soc_dai_get_drvdata(dai) as *mut mchp_pdmc;
    let ret: c_int;

    ret = mchp_pdmc_add_chmap_ctls((*rtd).pcm, dd);
    if ret < 0 {
        dev_err((*dd).dev, b"failed to add channel map controls: %d\n\0".as_ptr() as *const c_char, ret);
    }

    ret
}

static mchp_selectable_formats: u64 = SND_SOC_POSSIBLE_DAIFMT_PDM;

static mchp_pdmc_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(mchp_pdmc_dai_probe),
    set_fmt: Some(mchp_pdmc_set_fmt),
    startup: Some(mchp_pdmc_startup),
    hw_params: Some(mchp_pdmc_hw_params),
    trigger: Some(mchp_pdmc_trigger),
    pcm_new: Some(mchp_pdmc_pcm_new),
    auto_selectable_formats: &mchp_selectable_formats,
    num_auto_selectable_formats: 1,
};

static mut mchp_pdmc_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"mchp-pdmc\0".as_ptr() as *const c_char,
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 4,
        rate_min: 8000,
        rate_max: 192000,
        rates: SNDRV_PCM_RATE_KNOT,
        formats: SNDRV_PCM_FMTBIT_S24_LE,
    },
    ops: &mchp_pdmc_dai_ops,
};

/* PDMC interrupt handler */
unsafe extern "C" fn mchp_pdmc_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let dd = dev_id as *mut mchp_pdmc;
    let mut isr: u32 = 0;
    let mut msr: u32 = 0;
    let pending: u32;
    let mut ret: irqreturn_t = IRQ_NONE;

    regmap_read((*dd).regmap, MCHP_PDMC_ISR, &mut isr);
    regmap_read((*dd).regmap, MCHP_PDMC_IMR, &mut msr);

    pending = isr & msr;
    dev_dbg((*dd).dev, b"ISR (0x%02x): 0x%08x, IMR (0x%02x): 0x%08x, pending: 0x%08x\n\0".as_ptr() as *const c_char,
        MCHP_PDMC_ISR, isr, MCHP_PDMC_IMR, msr, pending);
    if pending == 0 {
        return IRQ_NONE;
    }

    if pending & MCHP_PDMC_IR_RXUDR != 0 {
        dev_warn((*dd).dev, b"underrun detected\n\0".as_ptr() as *const c_char);
        regmap_write((*dd).regmap, MCHP_PDMC_IDR, MCHP_PDMC_IR_RXUDR);
        ret = IRQ_HANDLED;
    }
    if pending & MCHP_PDMC_IR_RXOVR != 0 {
        dev_warn((*dd).dev, b"overrun detected\n\0".as_ptr() as *const c_char);
        regmap_write((*dd).regmap, MCHP_PDMC_IDR, MCHP_PDMC_IR_RXOVR);
        ret = IRQ_HANDLED;
    }

    ret
}

/* regmap configuration */
unsafe extern "C" fn mchp_pdmc_readable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        MCHP_PDMC_MR | MCHP_PDMC_CFGR | MCHP_PDMC_IMR | MCHP_PDMC_ISR | MCHP_PDMC_RHR | MCHP_PDMC_VER => true,
        _ => false,
    }
}

unsafe extern "C" fn mchp_pdmc_writeable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        MCHP_PDMC_CR | MCHP_PDMC_MR | MCHP_PDMC_CFGR | MCHP_PDMC_IER | MCHP_PDMC_IDR => true,
        _ => false,
    }
}

unsafe extern "C" fn mchp_pdmc_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        MCHP_PDMC_ISR | MCHP_PDMC_RHR => true,
        _ => false,
    }
}

unsafe extern "C" fn mchp_pdmc_precious_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        MCHP_PDMC_RHR | MCHP_PDMC_ISR => true,
        _ => false,
    }
}

static mchp_pdmc_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: MCHP_PDMC_VER,
    readable_reg: Some(mchp_pdmc_readable_reg),
    writeable_reg: Some(mchp_pdmc_writeable_reg),
    precious_reg: Some(mchp_pdmc_precious_reg),
    volatile_reg: Some(mchp_pdmc_volatile_reg),
    cache_type: REGCACHE_FLAT,
};

unsafe fn mchp_pdmc_dt_init(dd: *mut mchp_pdmc) -> c_int {
    let np = (*(*dd).dev).of_node;
    let mut mic_ch = [[false; MCHP_PDMC_EDGE_NO]; MCHP_PDMC_DS_NO];
    let mut i: c_int;
    let mut ret: c_int;

    if np.is_null() {
        dev_err((*dd).dev, b"device node not found\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    (*dd).mic_no = of_property_count_u32_elems(np, b"microchip,mic-pos\0".as_ptr() as *const c_char);
    if (*dd).mic_no < 0 {
        dev_err((*dd).dev, b"failed to get microchip,mic-pos: %d\0".as_ptr() as *const c_char, (*dd).mic_no);
        return (*dd).mic_no;
    }
    if (*dd).mic_no == 0 || (*dd).mic_no % 2 != 0 || (*dd).mic_no / 2 > MCHP_PDMC_MAX_CHANNELS as c_int {
        dev_err((*dd).dev, b"invalid array length for microchip,mic-pos: %d\0".as_ptr() as *const c_char, (*dd).mic_no);
        return -EINVAL;
    }

    (*dd).mic_no /= 2;

    dev_info((*dd).dev, b"%d PDM microphones declared\n\0".as_ptr() as *const c_char, (*dd).mic_no);

    /*
     * by default, we consider the order of microphones in
     * microchip,mic-pos to be the same with the channel mapping;
     * 1st microphone channel 0, 2nd microphone channel 1, etc.
     */
    i = 0;
    while i < (*dd).mic_no {
        let mut ds: c_int = 0;
        let mut edge: c_int = 0;

        ret = of_property_read_u32_index(np, b"microchip,mic-pos\0".as_ptr() as *const c_char, (i * 2) as c_uint, &mut ds);
        if ret != 0 {
            dev_err((*dd).dev, b"failed to get value no %d value from microchip,mic-pos: %d\0".as_ptr() as *const c_char, i * 2, ret);
            return ret;
        }
        if ds >= MCHP_PDMC_DS_NO as c_int {
            dev_err((*dd).dev, b"invalid DS index in microchip,mic-pos array: %d\0".as_ptr() as *const c_char, ds);
            return -EINVAL;
        }

        ret = of_property_read_u32_index(np, b"microchip,mic-pos\0".as_ptr() as *const c_char, (i * 2 + 1) as c_uint, &mut edge);
        if ret != 0 {
            dev_err((*dd).dev, b"failed to get value no %d value from microchip,mic-pos: %d\0".as_ptr() as *const c_char, i * 2 + 1, ret);
            return ret;
        }

        if edge != MCHP_PDMC_CLK_POSITIVE && edge != MCHP_PDMC_CLK_NEGATIVE {
            dev_err((*dd).dev, b"invalid edge in microchip,mic-pos array: %d\0".as_ptr() as *const c_char, edge);
            return -EINVAL;
        }
        if mic_ch[ds as usize][edge as usize] {
            dev_err((*dd).dev, b"duplicated mic (DS %d, edge %d) in microchip,mic-pos array\0".as_ptr() as *const c_char, ds, edge);
            return -EINVAL;
        }
        mic_ch[ds as usize][edge as usize] = true;
        (*dd).channel_mic_map[i as usize].ds_pos = ds;
        (*dd).channel_mic_map[i as usize].clk_edge = edge;
        i += 1;
    }

    (*dd).startup_delay_us = 150000;
    of_property_read_u32(np, b"microchip,startup-delay-us\0".as_ptr() as *const c_char, &mut (*dd).startup_delay_us);

    0
}

/* used to clean the channel index found on RHR's MSB */
unsafe extern "C" fn mchp_pdmc_process(substream: *mut snd_pcm_substream, channel: c_int, hwoff: c_ulong, bytes: c_ulong) -> c_int {
    let runtime = (*substream).runtime;
    let mut dma_ptr = (*runtime).dma_area.add(hwoff as usize).add((channel as c_ulong * ((*runtime).dma_bytes / (*runtime).channels as c_ulong)) as usize);
    let dma_ptr_end = dma_ptr.add(bytes as usize);
    let sample_size = samples_to_bytes(runtime, 1);

    while dma_ptr < dma_ptr_end {
        *dma_ptr = 0;
        dma_ptr = dma_ptr.add(sample_size as usize);
    }

    0
}

static mchp_pdmc_config: snd_dmaengine_pcm_config = snd_dmaengine_pcm_config {
    process: Some(mchp_pdmc_process),
    prepare_slave_config: unsafe { snd_dmaengine_pcm_prepare_slave_config },
};

unsafe extern "C" fn mchp_pdmc_runtime_suspend(dev: *mut device) -> c_int {
    let dd = dev_get_drvdata(dev) as *mut mchp_pdmc;

    regcache_cache_only((*dd).regmap, true);

    clk_disable_unprepare((*dd).gclk);
    clk_disable_unprepare((*dd).pclk);

    0
}

unsafe extern "C" fn mchp_pdmc_runtime_resume(dev: *mut device) -> c_int {
    let dd = dev_get_drvdata(dev) as *mut mchp_pdmc;
    let mut ret: c_int;

    ret = clk_prepare_enable((*dd).pclk);
    if ret != 0 {
        dev_err((*dd).dev, b"failed to enable the peripheral clock: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    ret = clk_prepare_enable((*dd).gclk);
    if ret != 0 {
        dev_err((*dd).dev, b"failed to enable generic clock: %d\n\0".as_ptr() as *const c_char, ret);
        clk_disable_unprepare((*dd).pclk);
        return ret;
    }

    regcache_cache_only((*dd).regmap, false);
    regcache_mark_dirty((*dd).regmap);
    ret = regcache_sync((*dd).regmap);
    if ret != 0 {
        regcache_cache_only((*dd).regmap, true);
        clk_disable_unprepare((*dd).gclk);
        clk_disable_unprepare((*dd).pclk);
    }

    ret
}

unsafe extern "C" fn mchp_pdmc_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let dd: *mut mchp_pdmc;
    let mut res: *mut resource = ptr::null_mut();
    let io_base: *mut c_void;
    let mut version: u32 = 0;
    let irq: c_int;
    let mut ret: c_int;

    dd = devm_kzalloc(dev, size_of::<mchp_pdmc>(), GFP_KERNEL) as *mut mchp_pdmc;
    if dd.is_null() {
        return -ENOMEM;
    }

    (*dd).dev = &mut (*pdev).dev;
    ret = mchp_pdmc_dt_init(dd);
    if ret < 0 {
        return ret;
    }

    irq = platform_get_irq(pdev, 0);
    if irq < 0 {
        return irq;
    }

    (*dd).pclk = devm_clk_get(dev, b"pclk\0".as_ptr() as *const c_char);
    if IS_ERR((*dd).pclk as *const c_void) {
        ret = PTR_ERR((*dd).pclk as *const c_void);
        dev_err(dev, b"failed to get peripheral clock: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    (*dd).gclk = devm_clk_get(dev, b"gclk\0".as_ptr() as *const c_char);
    if IS_ERR((*dd).gclk as *const c_void) {
        ret = PTR_ERR((*dd).gclk as *const c_void);
        dev_err(dev, b"failed to get GCK: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    io_base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR(io_base as *const c_void) {
        ret = PTR_ERR(io_base as *const c_void);
        dev_err(dev, b"failed to remap register memory: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    (*dd).regmap = devm_regmap_init_mmio(dev, io_base, &mchp_pdmc_regmap_config);
    if IS_ERR((*dd).regmap as *const c_void) {
        ret = PTR_ERR((*dd).regmap as *const c_void);
        dev_err(dev, b"failed to init register map: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = devm_request_irq(dev, irq, mchp_pdmc_interrupt, 0, dev_name(&mut (*pdev).dev), dd as *mut c_void);
    if ret < 0 {
        dev_err(dev, b"can't register ISR for IRQ %u (ret=%i)\n\0".as_ptr() as *const c_char, irq, ret);
        return ret;
    }

    /* by default audio filter is enabled and the SINC Filter order
     * will be set to the recommended value, 3
     */
    (*dd).audio_filter_en = true;
    (*dd).sinc_order = 3;

    (*dd).addr.addr = (*res).start + MCHP_PDMC_RHR as c_ulong;
    platform_set_drvdata(pdev, dd as *mut c_void);

    pm_runtime_enable((*dd).dev);
    if !pm_runtime_enabled((*dd).dev) {
        ret = mchp_pdmc_runtime_resume((*dd).dev);
        if ret != 0 {
            return ret;
        }
    }

    /* register platform */
    ret = devm_snd_dmaengine_pcm_register(dev, &mchp_pdmc_config, 0);
    if ret != 0 {
        dev_err(dev, b"could not register platform: %d\n\0".as_ptr() as *const c_char, ret);
        if !pm_runtime_status_suspended((*dd).dev) {
            mchp_pdmc_runtime_suspend((*dd).dev);
        }
        pm_runtime_disable((*dd).dev);
        return ret;
    }

    ret = devm_snd_soc_register_component(dev, &mchp_pdmc_dai_component, &mut mchp_pdmc_dai, 1);
    if ret != 0 {
        dev_err(dev, b"could not register CPU DAI: %d\n\0".as_ptr() as *const c_char, ret);
        if !pm_runtime_status_suspended((*dd).dev) {
            mchp_pdmc_runtime_suspend((*dd).dev);
        }
        pm_runtime_disable((*dd).dev);
        return ret;
    }

    /* print IP version */
    regmap_read((*dd).regmap, MCHP_PDMC_VER, &mut version);
    dev_info((*dd).dev, b"hw version: %#lx\n\0".as_ptr() as *const c_char, version & MCHP_PDMC_VER_VERSION);

    0
}

unsafe extern "C" fn mchp_pdmc_remove(pdev: *mut platform_device) {
    let dd = platform_get_drvdata(pdev) as *mut mchp_pdmc;

    atomic_set(&mut (*dd).busy_stream, 0);

    if !pm_runtime_status_suspended((*dd).dev) {
        mchp_pdmc_runtime_suspend((*dd).dev);
    }

    pm_runtime_disable((*dd).dev);
}

static mchp_pdmc_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"microchip,sama7g5-pdmc\0".as_ptr() as *const c_char,
    },
    of_device_id {
        /* sentinel */
        compatible: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, mchp_pdmc_of_match);

// static const struct dev_pm_ops mchp_pdmc_pm_ops = {
//     SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume)
//     RUNTIME_PM_OPS(mchp_pdmc_runtime_suspend, mchp_pdmc_runtime_resume, NULL)
// };

static mut mchp_pdmc_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"mchp-pdmc\0".as_ptr() as *const c_char,
        of_match_table: mchp_pdmc_of_match.as_ptr(),
        pm: unsafe { &mchp_pdmc_pm_ops },
    },
    probe: Some(mchp_pdmc_probe),
    remove: Some(mchp_pdmc_remove),
};
// module_platform_driver(mchp_pdmc_driver);

// MODULE_DESCRIPTION("Microchip PDMC driver under ALSA SoC architecture");
// MODULE_AUTHOR("Codrin Ciubotariu <codrin.ciubotariu@microchip.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
