// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * AC97 Controller Driver for Loongson-1 SoC
 *
 * Copyright (C) 2025 Keguang Zhang <keguang.zhang@gmail.com>
 */

// Dependencies from the original C includes:
// linux/bitfield.h, linux/dma-mapping.h, linux/init.h, linux/module.h,
// linux/platform_device.h, linux/regmap.h, sound/dmaengine_pcm.h,
// sound/pcm.h, sound/pcm_params.h, sound/soc.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

type dma_addr_t = c_ulong;

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ac97 {
    pub ext_id: c_uint,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub start: dma_addr_t,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_int,
    pub val_bits: c_int,
    pub reg_stride: c_int,
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr: dma_addr_t,
    pub addr_width: c_int,
    pub fifo_size: c_int,
}

#[repr(C)]
pub struct ls1x_ac97 {
    pub reg_base: *mut c_void,
    pub regmap: *mut regmap,
    pub tx_dma_base: dma_addr_t,
    pub rx_dma_base: dma_addr_t,
    pub capture_dma_data: snd_dmaengine_dai_dma_data,
    pub playback_dma_data: snd_dmaengine_dai_dma_data,
}

#[repr(C)]
pub struct snd_ac97_bus_ops {
    pub reset: Option<unsafe extern "C" fn(*mut snd_ac97)>,
    pub write: Option<unsafe extern "C" fn(*mut snd_ac97, u16, u16)>,
    pub read: Option<unsafe extern "C" fn(*mut snd_ac97, u16) -> u16>,
    pub init: Option<unsafe extern "C" fn(*mut snd_ac97)>,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_ulong,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct driver_private {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: driver_private,
}

const fn bit(n: u32) -> c_int {
    (1u32 << n) as c_int
}

const fn genmask(h: u32, l: u32) -> c_int {
    (((!0u32) << l) & ((!0u32) >> (31 - h))) as c_int
}

const fn mask_shift(mask: c_int) -> u32 {
    (mask as u32).trailing_zeros()
}

const fn field_prep(mask: c_int, val: c_int) -> c_int {
    (((val as u32) << mask_shift(mask)) & (mask as u32)) as c_int
}

/* Loongson-1 AC97 Controller Registers */
const AC97_CSR: c_uint = 0x0;
const AC97_OCC0: c_uint = 0x4;
const AC97_ICC: c_uint = 0x10;
const AC97_CRAC: c_uint = 0x18;
const AC97_INTRAW: c_uint = 0x54;
const AC97_INTM: c_uint = 0x58;
const AC97_INT_CW_CLR: c_uint = 0x68;
const AC97_INT_CR_CLR: c_uint = 0x6c;

/* Control Status Register Bits (CSR) */
const CSR_RESUME: c_int = bit(1);
const CSR_RST_FORCE: c_int = bit(0);

/* MIC Channel Configuration Bits */
const M_DMA_EN: c_int = bit(22);
const M_FIFO_THRES: c_int = genmask(21, 20);
const M_FIFO_THRES_FULL: c_int = field_prep(M_FIFO_THRES, 3);
const M_FIFO_THRES_HALF: c_int = field_prep(M_FIFO_THRES, 1);
const M_FIFO_THRES_QUARTER: c_int = field_prep(M_FIFO_THRES, 0);
const M_SW: c_int = genmask(19, 18);
const M_SW_16_BITS: c_int = field_prep(M_SW, 2);
const M_SW_8_BITS: c_int = field_prep(M_SW, 0);
const M_VSR: c_int = bit(17);
const M_CH_EN: c_int = bit(16);
/* Right Channel Configuration Bits */
const R_DMA_EN: c_int = bit(14);
const R_FIFO_THRES: c_int = genmask(13, 12);
const R_FIFO_THRES_EMPTY: c_int = field_prep(R_FIFO_THRES, 3);
const R_FIFO_THRES_HALF: c_int = field_prep(R_FIFO_THRES, 1);
const R_FIFO_THRES_QUARTER: c_int = field_prep(R_FIFO_THRES, 0);
const R_SW: c_int = genmask(11, 10);
const R_SW_16_BITS: c_int = field_prep(R_SW, 2);
const R_SW_8_BITS: c_int = field_prep(R_SW, 0);
const R_VSR: c_int = bit(9);
const R_CH_EN: c_int = bit(8);
/* Left Channel Configuration Bits */
const L_DMA_EN: c_int = bit(6);
const L_FIFO_THRES: c_int = genmask(5, 4);
const L_FIFO_THRES_EMPTY: c_int = field_prep(L_FIFO_THRES, 3);
const L_FIFO_THRES_HALF: c_int = field_prep(L_FIFO_THRES, 1);
const L_FIFO_THRES_QUARTER: c_int = field_prep(L_FIFO_THRES, 0);
const L_SW: c_int = genmask(3, 2);
const L_SW_16_BITS: c_int = field_prep(L_SW, 2);
const L_SW_8_BITS: c_int = field_prep(L_SW, 0);
const L_VSR: c_int = bit(1);
const L_CH_EN: c_int = bit(0);

/* Codec Register Access Command Bits (CRAC) */
const CODEC_WR: c_int = bit(31);
const CODEC_ADR: c_int = genmask(22, 16);
const CODEC_DAT: c_int = genmask(15, 0);

/* Interrupt Register (INTRAW) */
const CW_DONE: c_int = bit(1);
const CR_DONE: c_int = bit(0);

const LS1X_AC97_DMA_TX_EN: dma_addr_t = 1usize.wrapping_shl(31) as dma_addr_t;
const LS1X_AC97_DMA_STEREO: dma_addr_t = 1usize.wrapping_shl(30) as dma_addr_t;
const LS1X_AC97_DMA_TX_BYTES: c_int = genmask(29, 28);
const LS1X_AC97_DMA_TX_4_BYTES: dma_addr_t = field_prep(LS1X_AC97_DMA_TX_BYTES, 2) as dma_addr_t;
const LS1X_AC97_DMA_TX_2_BYTES: dma_addr_t = field_prep(LS1X_AC97_DMA_TX_BYTES, 1) as dma_addr_t;
const LS1X_AC97_DMA_TX_1_BYTE: dma_addr_t = field_prep(LS1X_AC97_DMA_TX_BYTES, 0) as dma_addr_t;
const LS1X_AC97_DMA_DADDR_MASK: dma_addr_t = genmask(27, 0) as dma_addr_t;

const LS1X_AC97_DMA_FIFO_SIZE: c_int = 128;

const LS1X_AC97_TIMEOUT: c_uint = 3000;

const KBUILD_MODNAME: *const c_char = b"loongson1_ac97\0".as_ptr() as *const c_char;
const AC97_EI_VRA: c_uint = 0x0001;
const DMA_SLAVE_BUSWIDTH_4_BYTES: c_int = 4;
const DMA_TO_DEVICE: c_int = 1;
const DMA_FROM_DEVICE: c_int = 2;
const GFP_KERNEL: c_uint = 0;
const IORESOURCE_MEM: c_uint = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENXIO: c_int = 6;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_FORMAT_S8: c_int = 0;
const SNDRV_PCM_FORMAT_U8: c_int = 1;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 2;
const SNDRV_PCM_FORMAT_U16_LE: c_int = 3;
const SNDRV_PCM_FORMAT_S16_BE: c_int = 4;
const SNDRV_PCM_FORMAT_U16_BE: c_int = 5;
const SNDRV_PCM_FMTBIT_S8: c_ulong = 1 << SNDRV_PCM_FORMAT_S8;
const SNDRV_PCM_FMTBIT_U8: c_ulong = 1 << SNDRV_PCM_FORMAT_U8;
const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 1 << SNDRV_PCM_FORMAT_S16_LE;
const SNDRV_PCM_FMTBIT_U16_LE: c_ulong = 1 << SNDRV_PCM_FORMAT_U16_LE;
const SNDRV_PCM_FMTBIT_S16_BE: c_ulong = 1 << SNDRV_PCM_FORMAT_S16_BE;
const SNDRV_PCM_FMTBIT_U16_BE: c_ulong = 1 << SNDRV_PCM_FORMAT_U16_BE;
const SNDRV_PCM_RATE_8000_48000: c_uint = 0;

static mut ls1x_ac97: *mut ls1x_ac97 = ptr::null_mut();

static ls1x_ac97_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    val_bits: 32,
    reg_stride: 4,
};

extern "C" {
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_int) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_int) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_int, val: c_int) -> c_int;
    fn regmap_clear_bits(map: *mut regmap, reg: c_uint, bits: c_int) -> c_int;
    fn regmap_set_bits(map: *mut regmap, reg: c_uint, bits: c_int) -> c_int;
    fn pr_err(fmt: *const c_char, ...) -> c_int;
    fn writel(val: c_uint, addr: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_soc_dai_get_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
    ) -> *mut snd_dmaengine_dai_dma_data;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_dai_init_dma_data(
        dai: *mut snd_soc_dai,
        playback: *mut snd_dmaengine_dai_dma_data,
        capture: *mut snd_dmaengine_dai_dma_data,
    );
    fn snd_soc_dai_set_drvdata(dai: *mut snd_soc_dai, data: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn platform_get_resource_byname(
        pdev: *mut platform_device,
        typ: c_uint,
        name: *const c_char,
    ) -> *mut resource;
    fn resource_size(res: *mut resource) -> dma_addr_t;
    fn dma_map_resource(
        dev: *mut device,
        start: dma_addr_t,
        size: dma_addr_t,
        dir: c_int,
        attrs: c_uint,
    ) -> dma_addr_t;
    fn dma_mapping_error(dev: *mut device, dma_addr: dma_addr_t) -> bool;
    fn devm_snd_dmaengine_pcm_register(dev: *mut device, config: *const c_void, flags: c_uint) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: usize,
    ) -> c_int;
    fn snd_soc_set_ac97_ops(ops: *mut snd_ac97_bus_ops) -> c_int;
}

unsafe fn regmap_read_poll_timeout_expr(
    map: *mut regmap,
    reg: c_uint,
    val: *mut c_int,
    mut cond: impl FnMut(c_int) -> bool,
    _sleep_us: c_uint,
    timeout_us: c_uint,
) -> c_int {
    let mut ret: c_int;
    let mut elapsed: c_uint = 0;

    loop {
        ret = regmap_read(map, reg, val);
        if ret != 0 {
            return ret;
        }
        if cond(*val) {
            return 0;
        }
        if elapsed >= timeout_us {
            return -1;
        }
        elapsed = elapsed.wrapping_add(1);
    }
}

unsafe extern "C" fn ls1x_ac97_reset(_ac97: *mut snd_ac97) {
    let mut val: c_int = 0;

    regmap_write((*ls1x_ac97).regmap, AC97_CSR, CSR_RST_FORCE);
    regmap_read_poll_timeout_expr(
        (*ls1x_ac97).regmap,
        AC97_CSR,
        &mut val,
        |v| (v & CSR_RESUME) == 0,
        0,
        LS1X_AC97_TIMEOUT,
    );
}

unsafe extern "C" fn ls1x_ac97_write(_ac97: *mut snd_ac97, reg: u16, val: u16) {
    let mut tmp: c_int;
    let mut ret: c_int;

    tmp = field_prep(CODEC_ADR, reg as c_int) | field_prep(CODEC_DAT, val as c_int);
    regmap_write((*ls1x_ac97).regmap, AC97_CRAC, tmp);
    ret = regmap_read_poll_timeout_expr(
        (*ls1x_ac97).regmap,
        AC97_INTRAW,
        &mut tmp,
        |v| (v & CW_DONE) != 0,
        0,
        LS1X_AC97_TIMEOUT,
    );
    if ret != 0 {
        pr_err(b"timeout on AC97 write! %d\n\0".as_ptr() as *const c_char, ret);
    }

    regmap_read((*ls1x_ac97).regmap, AC97_INT_CW_CLR, &mut ret);
}

unsafe extern "C" fn ls1x_ac97_read(_ac97: *mut snd_ac97, reg: u16) -> u16 {
    let mut val: c_int;
    let mut ret: c_int;

    val = CODEC_WR | field_prep(CODEC_ADR, reg as c_int);
    regmap_write((*ls1x_ac97).regmap, AC97_CRAC, val);
    ret = regmap_read_poll_timeout_expr(
        (*ls1x_ac97).regmap,
        AC97_INTRAW,
        &mut val,
        |v| (v & CR_DONE) != 0,
        0,
        LS1X_AC97_TIMEOUT,
    );
    if ret != 0 {
        pr_err(b"timeout on AC97 read! %d\n\0".as_ptr() as *const c_char, ret);
        return ret as u16;
    }

    regmap_read((*ls1x_ac97).regmap, AC97_INT_CR_CLR, &mut ret);
    regmap_read((*ls1x_ac97).regmap, AC97_CRAC, &mut ret);

    (ret & CODEC_DAT) as u16
}

unsafe extern "C" fn ls1x_ac97_init(ac97: *mut snd_ac97) {
    writel(0, ((*ls1x_ac97).reg_base as *mut u8).add(AC97_INTRAW as usize) as *mut c_void);
    writel(0, ((*ls1x_ac97).reg_base as *mut u8).add(AC97_INTM as usize) as *mut c_void);

    /* Config output channels */
    regmap_update_bits(
        (*ls1x_ac97).regmap,
        AC97_OCC0,
        R_DMA_EN | R_FIFO_THRES | R_CH_EN | L_DMA_EN | L_FIFO_THRES | L_CH_EN,
        R_DMA_EN | R_FIFO_THRES_EMPTY | R_CH_EN | L_DMA_EN | L_FIFO_THRES_EMPTY | L_CH_EN,
    );

    /* Config inputs channel */
    regmap_update_bits(
        (*ls1x_ac97).regmap,
        AC97_ICC,
        M_DMA_EN | M_FIFO_THRES | M_CH_EN | R_DMA_EN | R_FIFO_THRES | R_CH_EN | L_DMA_EN | L_FIFO_THRES | L_CH_EN,
        M_DMA_EN | M_FIFO_THRES_FULL | M_CH_EN | R_DMA_EN | R_FIFO_THRES_EMPTY | R_CH_EN | L_DMA_EN | L_FIFO_THRES_EMPTY | L_CH_EN,
    );

    if ((*ac97).ext_id & AC97_EI_VRA) != 0 {
        regmap_update_bits((*ls1x_ac97).regmap, AC97_OCC0, R_VSR | L_VSR, R_VSR | L_VSR);
        regmap_update_bits((*ls1x_ac97).regmap, AC97_ICC, M_VSR, M_VSR);
    }
}

static mut ls1x_ac97_ops: snd_ac97_bus_ops = snd_ac97_bus_ops {
    reset: Some(ls1x_ac97_reset),
    write: Some(ls1x_ac97_write),
    read: Some(ls1x_ac97_read),
    init: Some(ls1x_ac97_init),
};

unsafe extern "C" fn ls1x_ac97_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let ac97 = dev_get_drvdata((*cpu_dai).dev) as *mut ls1x_ac97;
    let dma_data = snd_soc_dai_get_dma_data(cpu_dai, substream);

    match params_channels(params) {
        1 => {
            (*dma_data).addr &= !LS1X_AC97_DMA_STEREO;
        }
        2 => {
            (*dma_data).addr |= LS1X_AC97_DMA_STEREO;
        }
        _ => {
            dev_err(
                (*cpu_dai).dev,
                b"unsupported channels! %d\n\0".as_ptr() as *const c_char,
                params_channels(params),
            );
            return -EINVAL;
        }
    }

    match params_format(params) {
        SNDRV_PCM_FORMAT_S8 | SNDRV_PCM_FORMAT_U8 => {
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                regmap_update_bits(
                    (*ac97).regmap,
                    AC97_OCC0,
                    R_SW | L_SW,
                    R_SW_8_BITS | L_SW_8_BITS,
                );
            } else {
                regmap_update_bits(
                    (*ac97).regmap,
                    AC97_ICC,
                    M_SW | R_SW | L_SW,
                    M_SW_8_BITS | R_SW_8_BITS | L_SW_8_BITS,
                );
            }
        }
        SNDRV_PCM_FORMAT_S16_LE | SNDRV_PCM_FORMAT_U16_LE | SNDRV_PCM_FORMAT_S16_BE | SNDRV_PCM_FORMAT_U16_BE => {
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                regmap_update_bits(
                    (*ac97).regmap,
                    AC97_OCC0,
                    R_SW | L_SW,
                    R_SW_16_BITS | L_SW_16_BITS,
                );
            } else {
                regmap_update_bits(
                    (*ac97).regmap,
                    AC97_ICC,
                    M_SW | R_SW | L_SW,
                    M_SW_16_BITS | R_SW_16_BITS | L_SW_16_BITS,
                );
            }
        }
        _ => {
            dev_err(
                (*cpu_dai).dev,
                b"unsupported format! %d\n\0".as_ptr() as *const c_char,
                params_format(params),
            );
            return -EINVAL;
        }
    }

    0
}

unsafe extern "C" fn ls1x_ac97_dai_probe(cpu_dai: *mut snd_soc_dai) -> c_int {
    let ac97 = dev_get_drvdata((*cpu_dai).dev) as *mut ls1x_ac97;

    (*ac97).capture_dma_data.addr = (*ac97).rx_dma_base & LS1X_AC97_DMA_DADDR_MASK;
    (*ac97).capture_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    (*ac97).capture_dma_data.fifo_size = LS1X_AC97_DMA_FIFO_SIZE;

    (*ac97).playback_dma_data.addr = (*ac97).tx_dma_base & LS1X_AC97_DMA_DADDR_MASK;
    (*ac97).playback_dma_data.addr |= LS1X_AC97_DMA_TX_4_BYTES;
    (*ac97).playback_dma_data.addr |= LS1X_AC97_DMA_TX_EN;
    (*ac97).playback_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    (*ac97).playback_dma_data.fifo_size = LS1X_AC97_DMA_FIFO_SIZE;

    snd_soc_dai_init_dma_data(cpu_dai, &mut (*ac97).playback_dma_data, &mut (*ac97).capture_dma_data);
    snd_soc_dai_set_drvdata(cpu_dai, ac97 as *mut c_void);

    0
}

static ls1x_ac97_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(ls1x_ac97_dai_probe),
    hw_params: Some(ls1x_ac97_hw_params),
};

const LS1X_AC97_FMTS: c_ulong = SNDRV_PCM_FMTBIT_S8
    | SNDRV_PCM_FMTBIT_U8
    | SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S16_BE
    | SNDRV_PCM_FMTBIT_U16_LE
    | SNDRV_PCM_FMTBIT_U16_BE;

static mut ls1x_ac97_dai: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: b"ls1x-ac97\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"AC97 Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_48000,
        formats: LS1X_AC97_FMTS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"AC97 Capture\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_48000,
        formats: LS1X_AC97_FMTS,
    },
    ops: &ls1x_ac97_dai_ops,
}];

static ls1x_ac97_component: snd_soc_component_driver = snd_soc_component_driver {
    name: KBUILD_MODNAME,
    legacy_dai_naming: 1,
};

unsafe extern "C" fn ls1x_ac97_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let mut ac97: *mut ls1x_ac97;
    let mut res: *mut resource;
    let mut ret: c_int;

    ac97 = devm_kzalloc(dev, core::mem::size_of::<ls1x_ac97>(), GFP_KERNEL) as *mut ls1x_ac97;
    if ac97.is_null() {
        return -ENOMEM;
    }
    ls1x_ac97 = ac97;
    platform_set_drvdata(pdev, ac97 as *mut c_void);

    (*ac97).reg_base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*ac97).reg_base) {
        return PTR_ERR((*ac97).reg_base);
    }

    (*ac97).regmap = devm_regmap_init_mmio(dev, (*ac97).reg_base, &ls1x_ac97_regmap_config);
    if IS_ERR((*ac97).regmap as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*ac97).regmap as *const c_void),
            b"devm_regmap_init_mmio failed\n\0".as_ptr() as *const c_char,
        );
    }

    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, b"audio-tx\0".as_ptr() as *const c_char);
    if res.is_null() {
        return dev_err_probe(
            dev,
            -EINVAL,
            b"Missing 'audio-tx' in reg-names property\n\0".as_ptr() as *const c_char,
        );
    }

    (*ac97).tx_dma_base = dma_map_resource(dev, (*res).start, resource_size(res), DMA_TO_DEVICE, 0);
    if dma_mapping_error(dev, (*ac97).tx_dma_base) {
        return -ENXIO;
    }

    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, b"audio-rx\0".as_ptr() as *const c_char);
    if res.is_null() {
        return dev_err_probe(
            dev,
            -EINVAL,
            b"Missing 'audio-rx' in reg-names property\n\0".as_ptr() as *const c_char,
        );
    }

    (*ac97).rx_dma_base = dma_map_resource(dev, (*res).start, resource_size(res), DMA_FROM_DEVICE, 0);
    if dma_mapping_error(dev, (*ac97).rx_dma_base) {
        return -ENXIO;
    }

    ret = devm_snd_dmaengine_pcm_register(dev, ptr::null(), 0);
    if ret != 0 {
        dev_err_probe(dev, ret, b"failed to register PCM\n\0".as_ptr() as *const c_char);
    }

    ret = devm_snd_soc_register_component(
        dev,
        &ls1x_ac97_component,
        ls1x_ac97_dai.as_mut_ptr(),
        ls1x_ac97_dai.len(),
    );
    if ret != 0 {
        dev_err_probe(dev, ret, b"failed to register DAI\n\0".as_ptr() as *const c_char);
    }

    snd_soc_set_ac97_ops(&mut ls1x_ac97_ops)
}

unsafe extern "C" fn ls1x_ac97_remove(_pdev: *mut platform_device) {
    ls1x_ac97 = ptr::null_mut();
    snd_soc_set_ac97_ops(ptr::null_mut());
}

// CONFIG_PM_SLEEP conditional in the original C source.
unsafe extern "C" fn ls1x_ac97_suspend(_dev: *mut device) -> c_int {
    let mut val: c_int = 0;

    regmap_clear_bits((*ls1x_ac97).regmap, AC97_OCC0, R_DMA_EN | R_CH_EN | L_DMA_EN | L_CH_EN);
    regmap_clear_bits(
        (*ls1x_ac97).regmap,
        AC97_ICC,
        M_DMA_EN | M_CH_EN | R_DMA_EN | R_CH_EN | L_DMA_EN | L_CH_EN,
    );
    regmap_set_bits((*ls1x_ac97).regmap, AC97_CSR, CSR_RESUME);

    regmap_read_poll_timeout_expr(
        (*ls1x_ac97).regmap,
        AC97_CSR,
        &mut val,
        |v| (v & CSR_RESUME) != 0,
        0,
        LS1X_AC97_TIMEOUT,
    )
}

unsafe extern "C" fn ls1x_ac97_resume(_dev: *mut device) -> c_int {
    let mut val: c_int = 0;

    regmap_set_bits((*ls1x_ac97).regmap, AC97_OCC0, R_DMA_EN | R_CH_EN | L_DMA_EN | L_CH_EN);
    regmap_set_bits(
        (*ls1x_ac97).regmap,
        AC97_ICC,
        M_DMA_EN | M_CH_EN | R_DMA_EN | R_CH_EN | L_DMA_EN | L_CH_EN,
    );
    regmap_set_bits((*ls1x_ac97).regmap, AC97_CSR, CSR_RESUME);

    regmap_read_poll_timeout_expr(
        (*ls1x_ac97).regmap,
        AC97_CSR,
        &mut val,
        |v| (v & CSR_RESUME) == 0,
        0,
        LS1X_AC97_TIMEOUT,
    )
}

static ls1x_ac97_pm_ops: dev_pm_ops = dev_pm_ops {
    suspend: Some(ls1x_ac97_suspend),
    resume: Some(ls1x_ac97_resume),
};

static ls1x_ac97_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"loongson,ls1b-ac97\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, ls1x_ac97_match);

static mut ls1x_ac97_driver: platform_driver = platform_driver {
    probe: Some(ls1x_ac97_probe),
    remove: Some(ls1x_ac97_remove),
    driver: driver_private {
        name: KBUILD_MODNAME,
        of_match_table: ls1x_ac97_match.as_ptr(),
        pm: &ls1x_ac97_pm_ops,
    },
};

// module_platform_driver(ls1x_ac97_driver);

// MODULE_AUTHOR("Keguang Zhang <keguang.zhang@gmail.com>");
// MODULE_DESCRIPTION("Loongson-1 AC97 Controller Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
