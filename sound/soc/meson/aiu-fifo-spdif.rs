// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2020 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>

// Rust translation of dependencies originally pulled from:
// <linux/clk.h>, <sound/pcm_params.h>, <sound/soc.h>,
// <sound/soc-dai.h>, "aiu.h", and "aiu-fifo.h".

use core::ffi::{c_char, c_int, c_uint};

const fn bit(nr: c_uint) -> c_uint {
    1u32 << nr
}

const fn genmask(h: c_uint, l: c_uint) -> c_uint {
    ((!0u32) << l) & ((!0u32) >> (31 - h))
}

pub const AIU_IEC958_DCU_FF_CTRL_EN: c_uint = bit(0);
pub const AIU_IEC958_DCU_FF_CTRL_AUTO_DISABLE: c_uint = bit(1);
pub const AIU_IEC958_DCU_FF_CTRL_IRQ_MODE: c_uint = genmask(3, 2);
pub const AIU_IEC958_DCU_FF_CTRL_IRQ_OUT_THD: c_uint = bit(2);
pub const AIU_IEC958_DCU_FF_CTRL_IRQ_FRAME_READ: c_uint = bit(3);
pub const AIU_IEC958_DCU_FF_CTRL_SYNC_HEAD_EN: c_uint = bit(4);
pub const AIU_IEC958_DCU_FF_CTRL_BYTE_SEEK: c_uint = bit(5);
pub const AIU_IEC958_DCU_FF_CTRL_CONTINUE: c_uint = bit(6);
pub const AIU_MEM_IEC958_CONTROL_ENDIAN: c_uint = genmask(5, 3);
pub const AIU_MEM_IEC958_CONTROL_RD_DDR: c_uint = bit(6);
pub const AIU_MEM_IEC958_CONTROL_MODE_16BIT: c_uint = bit(7);
pub const AIU_MEM_IEC958_CONTROL_MODE_LINEAR: c_uint = bit(8);
pub const AIU_MEM_IEC958_BUF_CNTL_INIT: c_uint = bit(0);
pub const AIU_RST_SOFT_958_FAST: c_uint = bit(2);

pub const AIU_FIFO_SPDIF_BLOCK: c_uint = 8;

extern "C" {
    static AIU_FORMATS: u64;
    static AIU_IEC958_DCU_FF_CTRL: c_uint;
    static AIU_MEM_IEC958_BUF_CNTL: c_uint;
    static AIU_MEM_IEC958_CONTROL: c_uint;
    static AIU_IEC958_BPF: c_uint;
    static AIU_RST_SOFT: c_uint;
    static AIU_MEM_IEC958_START: c_uint;

    static SNDRV_PCM_INFO_INTERLEAVED: c_uint;
    static SNDRV_PCM_INFO_MMAP: c_uint;
    static SNDRV_PCM_INFO_MMAP_VALID: c_uint;
    static SNDRV_PCM_INFO_PAUSE: c_uint;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static EINVAL: c_int;
    static PCLK: usize;
    static USHRT_MAX: c_uint;
    static UINT_MAX: c_uint;

    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut aiu;
    fn snd_soc_dai_dma_data_get_playback(dai: *mut snd_soc_dai) -> *mut aiu_fifo;
    fn params_physical_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_period_bytes(params: *mut snd_pcm_hw_params) -> c_uint;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;

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
    fn aiu_fifo_pcm_new(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream) -> c_int;
    fn aiu_fifo_spdif_dai_probe(dai: *mut snd_soc_dai) -> c_int;
    fn aiu_fifo_dai_remove(dai: *mut snd_soc_dai) -> c_int;
    fn aiu_fifo_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int;
    fn aiu_fifo_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai);
    fn aiu_fifo_dai_probe(dai: *mut snd_soc_dai) -> c_int;
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
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct aiu_clk {
    pub clk: *mut clk,
}

#[repr(C)]
pub struct aiu_spdif {
    pub clks: *mut aiu_clk,
    pub irq: c_int,
}

#[repr(C)]
pub struct aiu {
    pub spdif: aiu_spdif,
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
    pub pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_dai, *mut snd_pcm_substream) -> c_int>,
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub trigger:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
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

static mut fifo_spdif_pcm: snd_pcm_hardware = unsafe {
    snd_pcm_hardware {
        info: SNDRV_PCM_INFO_INTERLEAVED
            | SNDRV_PCM_INFO_MMAP
            | SNDRV_PCM_INFO_MMAP_VALID
            | SNDRV_PCM_INFO_PAUSE,
        formats: AIU_FORMATS,
        rate_min: 5512,
        rate_max: 192000,
        channels_min: 2,
        channels_max: 2,
        period_bytes_min: AIU_FIFO_SPDIF_BLOCK as usize,
        period_bytes_max: AIU_FIFO_SPDIF_BLOCK as usize * USHRT_MAX as usize,
        periods_min: 2,
        periods_max: UINT_MAX,

        /* No real justification for this */
        buffer_bytes_max: 1 * 1024 * 1024,
    }
};

unsafe extern "C" fn fifo_spdif_dcu_enable(component: *mut snd_soc_component, enable: bool) {
    snd_soc_component_update_bits(
        component,
        AIU_IEC958_DCU_FF_CTRL,
        AIU_IEC958_DCU_FF_CTRL_EN,
        if enable {
            AIU_IEC958_DCU_FF_CTRL_EN
        } else {
            0
        },
    );
}

unsafe extern "C" fn fifo_spdif_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let ret: c_int;

    ret = aiu_fifo_trigger(substream, cmd, dai);
    if ret != 0 {
        return ret;
    }

    if cmd == SNDRV_PCM_TRIGGER_START
        || cmd == SNDRV_PCM_TRIGGER_RESUME
        || cmd == SNDRV_PCM_TRIGGER_PAUSE_RELEASE
    {
        snd_soc_component_write(component, AIU_RST_SOFT, AIU_RST_SOFT_958_FAST);
        fifo_spdif_dcu_enable(component, true);
    } else if cmd == SNDRV_PCM_TRIGGER_SUSPEND
        || cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH
        || cmd == SNDRV_PCM_TRIGGER_STOP
    {
        snd_soc_component_write(component, AIU_RST_SOFT, AIU_RST_SOFT_958_FAST);
        fifo_spdif_dcu_enable(component, false);
    } else {
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn fifo_spdif_prepare(
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
        AIU_MEM_IEC958_BUF_CNTL,
        AIU_MEM_IEC958_BUF_CNTL_INIT,
        AIU_MEM_IEC958_BUF_CNTL_INIT,
    );
    snd_soc_component_update_bits(
        component,
        AIU_MEM_IEC958_BUF_CNTL,
        AIU_MEM_IEC958_BUF_CNTL_INIT,
        0,
    );

    0
}

unsafe extern "C" fn fifo_spdif_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let mut val: c_uint;
    let ret: c_int;

    ret = aiu_fifo_hw_params(substream, params, dai);
    if ret != 0 {
        return ret;
    }

    val = AIU_MEM_IEC958_CONTROL_RD_DDR | AIU_MEM_IEC958_CONTROL_MODE_LINEAR;

    match params_physical_width(params) {
        16 => {
            val |= AIU_MEM_IEC958_CONTROL_MODE_16BIT;
        }
        32 => {}
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
        AIU_MEM_IEC958_CONTROL,
        AIU_MEM_IEC958_CONTROL_ENDIAN
            | AIU_MEM_IEC958_CONTROL_RD_DDR
            | AIU_MEM_IEC958_CONTROL_MODE_LINEAR
            | AIU_MEM_IEC958_CONTROL_MODE_16BIT,
        val,
    );

    /* Number bytes read by the FIFO between each IRQ */
    snd_soc_component_write(component, AIU_IEC958_BPF, params_period_bytes(params));

    /*
     * AUTO_DISABLE and SYNC_HEAD are enabled by default but
     * this should be disabled in PCM (uncompressed) mode
     */
    snd_soc_component_update_bits(
        component,
        AIU_IEC958_DCU_FF_CTRL,
        AIU_IEC958_DCU_FF_CTRL_AUTO_DISABLE
            | AIU_IEC958_DCU_FF_CTRL_IRQ_MODE
            | AIU_IEC958_DCU_FF_CTRL_SYNC_HEAD_EN,
        AIU_IEC958_DCU_FF_CTRL_IRQ_FRAME_READ,
    );

    0
}

#[no_mangle]
pub static aiu_fifo_spdif_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    pcm_new: Some(aiu_fifo_pcm_new),
    probe: Some(aiu_fifo_spdif_dai_probe),
    remove: Some(aiu_fifo_dai_remove),
    trigger: Some(fifo_spdif_trigger),
    prepare: Some(fifo_spdif_prepare),
    hw_params: Some(fifo_spdif_hw_params),
    startup: Some(aiu_fifo_startup),
    shutdown: Some(aiu_fifo_shutdown),
};

#[no_mangle]
pub unsafe extern "C" fn aiu_fifo_spdif_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let aiu: *mut aiu = snd_soc_component_get_drvdata(component);
    let fifo: *mut aiu_fifo;
    let ret: c_int;

    ret = aiu_fifo_dai_probe(dai);
    if ret != 0 {
        return ret;
    }

    fifo = snd_soc_dai_dma_data_get_playback(dai);

    (*fifo).pcm = &raw const fifo_spdif_pcm;
    (*fifo).mem_offset = AIU_MEM_IEC958_START;
    (*fifo).fifo_block = 1;
    (*fifo).pclk = (*(*aiu).spdif.clks.add(PCLK)).clk;
    (*fifo).irq = (*aiu).spdif.irq;

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
