// SPDX-License-Identifier: GPL-2.0-only

// Translated from C implementation source. C include dependencies:
// linux/slab.h, linux/module.h, linux/dma-mapping.h, linux/dmaengine.h,
// linux/dma/pxa-dma.h, sound/core.h, sound/pcm.h, sound/pcm_params.h,
// sound/dmaengine_pcm.h, and "pxa2xx-lib.h".

pub type c_int = i32;
pub type c_char = i8;
pub type size_t = usize;
pub type snd_pcm_uframes_t = usize;

pub const SNDRV_PCM_INFO_MMAP: u32 = 1 << 0;
pub const SNDRV_PCM_INFO_MMAP_VALID: u32 = 1 << 1;
pub const SNDRV_PCM_INFO_INTERLEAVED: u32 = 1 << 2;
pub const SNDRV_PCM_INFO_PAUSE: u32 = 1 << 3;
pub const SNDRV_PCM_INFO_RESUME: u32 = 1 << 4;

pub const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 0;
pub const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 1;
pub const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 2;

pub const SNDRV_PCM_HW_PARAM_PERIOD_BYTES: c_int = 0;
pub const SNDRV_PCM_HW_PARAM_BUFFER_BYTES: c_int = 1;
pub const SNDRV_PCM_HW_PARAM_PERIODS: c_int = 2;
pub const SNDRV_DMA_TYPE_DEV_WC: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hardware {
    pub info: u32,
    pub formats: u64,
    pub rates: u32,
    pub rate_min: u32,
    pub rate_max: u32,
    pub channels_min: u32,
    pub channels_max: u32,
    pub buffer_bytes_max: size_t,
    pub period_bytes_min: size_t,
    pub period_bytes_max: size_t,
    pub periods_min: u32,
    pub periods_max: u32,
    pub fifo_size: size_t,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw: snd_pcm_hardware,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct snd_soc_card {
    pub snd_card: *mut snd_card,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm {
    pub card: *mut snd_card,
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub chan_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dma_chan {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dma_slave_config {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

macro_rules! DMA_BIT_MASK {
    ($nr:expr) => {
        if $nr == 64 {
            !0u64
        } else {
            (1u64 << $nr) - 1
        }
    };
}

macro_rules! EXPORT_SYMBOL {
    ($symbol:ident) => {};
}

macro_rules! MODULE_AUTHOR {
    ($author:expr) => {};
}

macro_rules! MODULE_DESCRIPTION {
    ($description:expr) => {};
}

macro_rules! MODULE_LICENSE {
    ($license:expr) => {};
}

extern "C" {
    fn snd_dmaengine_pcm_get_chan(substream: *mut snd_pcm_substream) -> *mut dma_chan;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_get_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
    ) -> *mut snd_dmaengine_dai_dma_data;
    fn snd_hwparams_to_dma_slave_config(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        config: *mut dma_slave_config,
    ) -> c_int;
    fn snd_dmaengine_pcm_set_config_from_dai_data(
        substream: *mut snd_pcm_substream,
        dma_data: *mut snd_dmaengine_dai_dma_data,
        config: *mut dma_slave_config,
    );
    fn dmaengine_slave_config(chan: *mut dma_chan, config: *mut dma_slave_config) -> c_int;
    fn snd_dmaengine_pcm_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int;
    fn snd_dmaengine_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t;
    fn snd_pcm_hw_constraint_step(
        runtime: *mut snd_pcm_runtime,
        cond: c_int,
        var: c_int,
        step: size_t,
    ) -> c_int;
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, var: c_int) -> c_int;
    fn snd_dmaengine_pcm_open(
        substream: *mut snd_pcm_substream,
        chan: *mut dma_chan,
    ) -> c_int;
    fn dma_request_slave_channel(dev: *mut device, name: *const c_char) -> *mut dma_chan;
    fn snd_dmaengine_pcm_close_release_chan(substream: *mut snd_pcm_substream) -> c_int;
    fn snd_pcm_set_fixed_buffer_all(
        pcm: *mut snd_pcm,
        ty: c_int,
        dev: *mut device,
        size: size_t,
    ) -> c_int;
    fn dma_coerce_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
}

static pxa2xx_pcm_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_RESUME,
    formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
    period_bytes_min: 32,
    period_bytes_max: 8192 - 32,
    periods_min: 1,
    periods_max: 256,
    buffer_bytes_max: 128 * 1024,
    fifo_size: 32,
    rates: 0,
    rate_min: 0,
    rate_max: 0,
    channels_min: 0,
    channels_max: 0,
};

unsafe fn pxa2xx_pcm_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let chan: *mut dma_chan = snd_dmaengine_pcm_get_chan(substream);
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let mut dma_params: *mut snd_dmaengine_dai_dma_data;
    let mut config: dma_slave_config = core::mem::zeroed();
    let mut ret: c_int;

    dma_params = snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(rtd, 0), substream);
    if dma_params.is_null() {
        return 0;
    }

    ret = snd_hwparams_to_dma_slave_config(substream, params, &mut config);
    if ret != 0 {
        return ret;
    }

    snd_dmaengine_pcm_set_config_from_dai_data(
        substream,
        snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(rtd, 0), substream),
        &mut config,
    );

    ret = dmaengine_slave_config(chan, &mut config);
    if ret != 0 {
        return ret;
    }

    0
}

unsafe fn pxa2xx_pcm_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    snd_dmaengine_pcm_trigger(substream, cmd)
}

unsafe fn pxa2xx_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    snd_dmaengine_pcm_pointer(substream)
}

unsafe fn pxa2xx_pcm_prepare(_substream: *mut snd_pcm_substream) -> c_int {
    0
}

unsafe fn pxa2xx_pcm_open(substream: *mut snd_pcm_substream) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let mut dma_params: *mut snd_dmaengine_dai_dma_data;
    let mut ret: c_int;

    (*runtime).hw = pxa2xx_pcm_hardware;

    dma_params = snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(rtd, 0), substream);
    if dma_params.is_null() {
        return 0;
    }

    /*
     * For mysterious reasons (and despite what the manual says)
     * playback samples are lost if the DMA count is not a multiple
     * of the DMA burst size.  Let's add a rule to enforce that.
     */
    ret = snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_BYTES, 32);
    if ret != 0 {
        return ret;
    }

    ret = snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_BUFFER_BYTES, 32);
    if ret != 0 {
        return ret;
    }

    ret = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if ret < 0 {
        return ret;
    }

    snd_dmaengine_pcm_open(
        substream,
        dma_request_slave_channel((*snd_soc_rtd_to_cpu(rtd, 0)).dev, (*dma_params).chan_name),
    )
}

unsafe fn pxa2xx_pcm_close(substream: *mut snd_pcm_substream) -> c_int {
    snd_dmaengine_pcm_close_release_chan(substream)
}

unsafe fn pxa2xx_pcm_preallocate_dma_buffer(pcm: *mut snd_pcm) -> c_int {
    let size: size_t = pxa2xx_pcm_hardware.buffer_bytes_max;

    snd_pcm_set_fixed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV_WC, (*(*pcm).card).dev, size)
}

#[no_mangle]
pub unsafe extern "C" fn pxa2xx_soc_pcm_new(
    _component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    let card: *mut snd_card = (*(*rtd).card).snd_card;
    let pcm: *mut snd_pcm = (*rtd).pcm;
    let mut ret: c_int;

    ret = dma_coerce_mask_and_coherent((*card).dev, DMA_BIT_MASK!(32));
    if ret != 0 {
        return ret;
    }

    pxa2xx_pcm_preallocate_dma_buffer(pcm)
}
EXPORT_SYMBOL!(pxa2xx_soc_pcm_new);

#[no_mangle]
pub unsafe extern "C" fn pxa2xx_soc_pcm_open(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    pxa2xx_pcm_open(substream)
}
EXPORT_SYMBOL!(pxa2xx_soc_pcm_open);

#[no_mangle]
pub unsafe extern "C" fn pxa2xx_soc_pcm_close(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    pxa2xx_pcm_close(substream)
}
EXPORT_SYMBOL!(pxa2xx_soc_pcm_close);

#[no_mangle]
pub unsafe extern "C" fn pxa2xx_soc_pcm_hw_params(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    pxa2xx_pcm_hw_params(substream, params)
}
EXPORT_SYMBOL!(pxa2xx_soc_pcm_hw_params);

#[no_mangle]
pub unsafe extern "C" fn pxa2xx_soc_pcm_prepare(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    pxa2xx_pcm_prepare(substream)
}
EXPORT_SYMBOL!(pxa2xx_soc_pcm_prepare);

#[no_mangle]
pub unsafe extern "C" fn pxa2xx_soc_pcm_trigger(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    pxa2xx_pcm_trigger(substream, cmd)
}
EXPORT_SYMBOL!(pxa2xx_soc_pcm_trigger);

#[no_mangle]
pub unsafe extern "C" fn pxa2xx_soc_pcm_pointer(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    pxa2xx_pcm_pointer(substream)
}
EXPORT_SYMBOL!(pxa2xx_soc_pcm_pointer);

MODULE_AUTHOR!("Nicolas Pitre");
MODULE_DESCRIPTION!("Intel PXA2xx sound library");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
