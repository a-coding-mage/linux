// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2020 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>

use core::ffi::{c_char, c_int, c_uint};

// Dependencies from linux/bitfield.h, linux/clk.h, sound/pcm_params.h,
// sound/soc.h, sound/soc-dai.h, aiu.h, and aiu-fifo.h are expected to be
// supplied by the surrounding translation.

const fn bit(nr: c_uint) -> c_uint {
    1u32 << nr
}

const fn genmask(h: c_uint, l: c_uint) -> c_uint {
    (!0u32 << l) & (!0u32 >> (31 - h))
}

fn field_prep(mask: c_uint, val: c_uint) -> c_uint {
    (val << mask.trailing_zeros()) & mask
}

const AIU_I2S_SOURCE_DESC_MODE_8CH: c_uint = bit(0);
const AIU_I2S_SOURCE_DESC_MODE_24BIT: c_uint = bit(5);
const AIU_I2S_SOURCE_DESC_MODE_32BIT: c_uint = bit(9);
const AIU_I2S_SOURCE_DESC_MODE_SPLIT: c_uint = bit(11);
const AIU_MEM_I2S_MASKS_IRQ_BLOCK: c_uint = genmask(31, 16);
const AIU_MEM_I2S_CONTROL_MODE_16BIT: c_uint = bit(6);
const AIU_MEM_I2S_BUF_CNTL_INIT: c_uint = bit(0);
const AIU_RST_SOFT_I2S_FAST: c_uint = bit(0);
const AIU_I2S_MISC_HOLD_EN: c_uint = bit(2);
const AIU_I2S_MISC_FORCE_LEFT_RIGHT: c_uint = bit(4);

const AIU_FIFO_I2S_BLOCK: c_uint = 256;

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: u64,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub period_bytes_min: usize,
    pub period_bytes_max: usize,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub buffer_bytes_max: usize,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub trigger: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int,
    >,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
}

#[repr(C)]
pub struct aiu_fifo {
    pub pcm: *const snd_pcm_hardware,
    pub mem_offset: c_uint,
    pub fifo_block: c_uint,
    pub pclk: *mut clk,
    pub irq: c_int,
}

#[repr(C)]
pub struct aiu_clk {
    pub clk: *mut clk,
}

#[repr(C)]
pub struct aiu_i2s {
    pub clks: [aiu_clk; 1],
    pub irq: c_int,
}

#[repr(C)]
pub struct aiu {
    pub i2s: aiu_i2s,
}

const PCLK: usize = 0;

extern "C" {
    static AIU_FORMATS: u64;
    static AIU_RST_SOFT: c_uint;
    static AIU_I2S_SYNC: c_uint;
    static AIU_MEM_I2S_BUF_CNTL: c_uint;
    static AIU_I2S_MISC: c_uint;
    static AIU_MEM_I2S_CONTROL: c_uint;
    static AIU_MEM_I2S_MASKS: c_uint;
    static AIU_MEM_I2S_START: c_uint;

    static SNDRV_PCM_INFO_INTERLEAVED: c_uint;
    static SNDRV_PCM_INFO_MMAP: c_uint;
    static SNDRV_PCM_INFO_MMAP_VALID: c_uint;
    static SNDRV_PCM_INFO_PAUSE: c_uint;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static EINVAL: c_int;

    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint);
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut aiu;
    fn snd_soc_dai_dma_data_get_playback(dai: *mut snd_soc_dai) -> *mut aiu_fifo;
    fn params_physical_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_period_bytes(params: *mut snd_pcm_hw_params) -> c_uint;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);

    fn aiu_fifo_trigger(
        substream: *mut snd_pcm_substream,
        cmd: c_int,
        dai: *mut snd_soc_dai,
    ) -> c_int;
    fn aiu_fifo_prepare(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int;
    fn aiu_fifo_hw_params(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        dai: *mut snd_soc_dai,
    ) -> c_int;
    fn aiu_fifo_pcm_new(dai: *mut snd_soc_dai) -> c_int;
    fn aiu_fifo_dai_remove(dai: *mut snd_soc_dai) -> c_int;
    fn aiu_fifo_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int;
    fn aiu_fifo_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai);
    fn aiu_fifo_dai_probe(dai: *mut snd_soc_dai) -> c_int;
}

static fifo_i2s_pcm: snd_pcm_hardware = snd_pcm_hardware {
    info: unsafe {
        SNDRV_PCM_INFO_INTERLEAVED
            | SNDRV_PCM_INFO_MMAP
            | SNDRV_PCM_INFO_MMAP_VALID
            | SNDRV_PCM_INFO_PAUSE
    },
    formats: unsafe { AIU_FORMATS },
    rate_min: 5512,
    rate_max: 192000,
    channels_min: 2,
    channels_max: 8,
    period_bytes_min: AIU_FIFO_I2S_BLOCK as usize,
    period_bytes_max: AIU_FIFO_I2S_BLOCK as usize * u16::MAX as usize,
    periods_min: 2,
    periods_max: c_uint::MAX,

    /* No real justification for this */
    buffer_bytes_max: 1 * 1024 * 1024,
};

unsafe extern "C" fn aiu_fifo_i2s_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;

    match cmd {
        x if x == SNDRV_PCM_TRIGGER_START
            || x == SNDRV_PCM_TRIGGER_RESUME
            || x == SNDRV_PCM_TRIGGER_PAUSE_RELEASE =>
        {
            snd_soc_component_write(component, AIU_RST_SOFT, AIU_RST_SOFT_I2S_FAST);
            snd_soc_component_read(component, AIU_I2S_SYNC);
        }
        _ => {}
    }

    aiu_fifo_trigger(substream, cmd, dai)
}

unsafe extern "C" fn aiu_fifo_i2s_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let ret: c_int;

    ret = aiu_fifo_prepare(substream, dai);
    if ret != 0 {
        return ret;
    }

    snd_soc_component_update_bits(
        component,
        AIU_MEM_I2S_BUF_CNTL,
        AIU_MEM_I2S_BUF_CNTL_INIT,
        AIU_MEM_I2S_BUF_CNTL_INIT,
    );
    snd_soc_component_update_bits(
        component,
        AIU_MEM_I2S_BUF_CNTL,
        AIU_MEM_I2S_BUF_CNTL_INIT,
        0,
    );

    0
}

unsafe extern "C" fn aiu_fifo_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let fifo: *mut aiu_fifo = snd_soc_dai_dma_data_get_playback(dai);
    let mut val: c_uint;
    let ret: c_int;

    snd_soc_component_update_bits(
        component,
        AIU_I2S_MISC,
        AIU_I2S_MISC_HOLD_EN,
        AIU_I2S_MISC_HOLD_EN,
    );

    ret = aiu_fifo_hw_params(substream, params, dai);
    if ret != 0 {
        return ret;
    }

    match params_physical_width(params) {
        16 => {
            val = AIU_MEM_I2S_CONTROL_MODE_16BIT;
        }
        32 => {
            val = 0;
        }
        _ => {
            dev_err(
                (*dai).dev,
                b"Unsupported physical width %u\n\0".as_ptr() as *const c_char,
                params_physical_width(params),
            );
            return -EINVAL;
        }
    }

    snd_soc_component_update_bits(
        component,
        AIU_MEM_I2S_CONTROL,
        AIU_MEM_I2S_CONTROL_MODE_16BIT,
        val,
    );

    /* Setup the irq periodicity */
    val = params_period_bytes(params) / (*fifo).fifo_block;
    val = field_prep(AIU_MEM_I2S_MASKS_IRQ_BLOCK, val);
    snd_soc_component_update_bits(component, AIU_MEM_I2S_MASKS, AIU_MEM_I2S_MASKS_IRQ_BLOCK, val);

    /*
     * Most (all?) supported SoCs have this bit set by default. The vendor
     * driver however sets it manually (depending on the version either
     * while un-setting AIU_I2S_MISC_HOLD_EN or right before that). Follow
     * the same approach for consistency with the vendor driver.
     */
    snd_soc_component_update_bits(
        component,
        AIU_I2S_MISC,
        AIU_I2S_MISC_FORCE_LEFT_RIGHT,
        AIU_I2S_MISC_FORCE_LEFT_RIGHT,
    );

    snd_soc_component_update_bits(component, AIU_I2S_MISC, AIU_I2S_MISC_HOLD_EN, 0);

    0
}

pub static aiu_fifo_i2s_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    pcm_new: Some(aiu_fifo_pcm_new),
    probe: Some(aiu_fifo_i2s_dai_probe),
    remove: Some(aiu_fifo_dai_remove),
    trigger: Some(aiu_fifo_i2s_trigger),
    prepare: Some(aiu_fifo_i2s_prepare),
    hw_params: Some(aiu_fifo_i2s_hw_params),
    startup: Some(aiu_fifo_startup),
    shutdown: Some(aiu_fifo_shutdown),
};

pub unsafe extern "C" fn aiu_fifo_i2s_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let aiu: *mut aiu = snd_soc_component_get_drvdata(component);
    let fifo: *mut aiu_fifo;
    let ret: c_int;

    ret = aiu_fifo_dai_probe(dai);
    if ret != 0 {
        return ret;
    }

    fifo = snd_soc_dai_dma_data_get_playback(dai);

    (*fifo).pcm = &fifo_i2s_pcm;
    (*fifo).mem_offset = AIU_MEM_I2S_START;
    (*fifo).fifo_block = AIU_FIFO_I2S_BLOCK;
    (*fifo).pclk = (*aiu).i2s.clks[PCLK].clk;
    (*fifo).irq = (*aiu).i2s.irq;

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
