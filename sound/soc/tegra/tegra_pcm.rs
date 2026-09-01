// SPDX-License-Identifier: GPL-2.0-only
/*
 * tegra_pcm.c - Tegra PCM driver
 *
 * Author: Stephen Warren <swarren@nvidia.com>
 * Copyright (C) 2010,2012 - NVIDIA, Inc.
 *
 * Based on code copyright/by:
 *
 * Copyright (c) 2009-2010, NVIDIA Corporation.
 * Scott Peterson <speterson@nvidia.com>
 * Vijay Mali <vmali@nvidia.com>
 *
 * Copyright (C) 2010 Google, Inc.
 * Iliyan Malchev <malchev@google.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};

// Dependencies from:
// <linux/module.h>
// <linux/dma-mapping.h>
// <sound/core.h>
// <sound/pcm.h>
// <sound/pcm_params.h>
// <sound/soc.h>
// <sound/dmaengine_pcm.h>
// "tegra_pcm.h"

pub type size_t = usize;
pub type snd_pcm_uframes_t = c_ulonglong;

pub const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 0;
pub const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 1;
pub const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 2;
pub const PAGE_SIZE: usize = 4096;
pub const SNDRV_PCM_HW_PARAM_PERIOD_BYTES: c_int = 0;
pub const ENODEV: c_int = 19;
pub const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
pub const DMA_SLAVE_BUSWIDTH_4_BYTES: c_int = 4;
pub const SNDRV_DMA_TYPE_DEV_WC: c_int = 0;

#[inline]
pub const fn DMA_BIT_MASK(nr: u32) -> u64 {
    if nr == 64 {
        !0u64
    } else {
        (1u64 << nr) - 1
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub period_bytes_min: usize,
    pub period_bytes_max: usize,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub buffer_bytes_max: usize,
    pub fifo_size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_dmaengine_pcm_config {
    pub pcm_hardware: *const snd_pcm_hardware,
    pub prepare_slave_config: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            params: *mut snd_pcm_hw_params,
            slave_config: *mut dma_slave_config,
        ) -> c_int,
    >,
    pub prealloc_buffer_size: size_t,
    pub dma_dev: *mut device,
    pub chan_names: [*mut c_char; 2],
}

unsafe impl Sync for snd_dmaengine_pcm_config {}

#[repr(C)]
pub struct device {
    pub parent: *mut device,
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: c_int,
    pub wait_time: c_int,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dai_link: *mut snd_soc_dai_link,
    pub dev: *mut device,
    pub pcm: *mut snd_pcm,
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub no_pcm: bool,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr: u64,
    pub chan_name: *mut c_char,
}

#[repr(C)]
pub struct dma_chan {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_slave_config {
    pub dst_addr_width: c_int,
    pub dst_addr: u64,
    pub dst_maxburst: c_uint,
    pub src_addr_width: c_int,
    pub src_addr: u64,
    pub src_maxburst: c_uint,
}

#[repr(C)]
pub struct snd_pcm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
    pub snd_card: *mut snd_card,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
}

unsafe extern "C" {
    pub fn snd_dmaengine_pcm_prepare_slave_config(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        slave_config: *mut dma_slave_config,
    ) -> c_int;
    pub fn snd_dmaengine_pcm_register(
        dev: *mut device,
        config: *const snd_dmaengine_pcm_config,
        flags: c_uint,
    ) -> c_int;
    pub fn devm_snd_dmaengine_pcm_register(
        dev: *mut device,
        config: *const snd_dmaengine_pcm_config,
        flags: c_uint,
    ) -> c_int;
    pub fn snd_dmaengine_pcm_unregister(dev: *mut device);
    pub fn snd_soc_substream_to_rtd(
        substream: *mut snd_pcm_substream,
    ) -> *mut snd_soc_pcm_runtime;
    pub fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    pub fn snd_soc_dai_get_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
    ) -> *mut snd_dmaengine_dai_dma_data;
    pub fn snd_soc_set_runtime_hwparams(
        substream: *mut snd_pcm_substream,
        hw: *const snd_pcm_hardware,
    );
    pub fn snd_pcm_hw_constraint_step(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_int,
        step: c_uint,
    ) -> c_int;
    pub fn dma_request_chan(dev: *mut device, name: *mut c_char) -> *mut dma_chan;
    pub fn IS_ERR(ptr: *const c_void) -> bool;
    pub fn snd_dmaengine_pcm_open(
        substream: *mut snd_pcm_substream,
        chan: *mut dma_chan,
    ) -> c_int;
    pub fn dma_release_channel(chan: *mut dma_chan);
    pub fn snd_dmaengine_pcm_close_release_chan(substream: *mut snd_pcm_substream);
    pub fn snd_dmaengine_pcm_get_chan(substream: *mut snd_pcm_substream) -> *mut dma_chan;
    pub fn snd_hwparams_to_dma_slave_config(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        slave_config: *mut dma_slave_config,
    ) -> c_int;
    pub fn dmaengine_slave_config(
        chan: *mut dma_chan,
        config: *mut dma_slave_config,
    ) -> c_int;
    pub fn snd_dmaengine_pcm_pointer(
        substream: *mut snd_pcm_substream,
    ) -> snd_pcm_uframes_t;
    pub fn dma_set_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    pub fn snd_pcm_set_fixed_buffer_all(
        pcm: *mut snd_pcm,
        ty: c_int,
        dev: *mut device,
        size: size_t,
    ) -> c_int;
    pub fn of_property_present(np: *mut device_node, propname: *const c_char) -> bool;
    pub fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

static tegra_pcm_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_INTERLEAVED,
    period_bytes_min: 1024,
    period_bytes_max: PAGE_SIZE,
    periods_min: 2,
    periods_max: 8,
    buffer_bytes_max: PAGE_SIZE * 8,
    fifo_size: 4,
};

static tegra_dmaengine_pcm_config: snd_dmaengine_pcm_config = snd_dmaengine_pcm_config {
    pcm_hardware: &tegra_pcm_hardware,
    prepare_slave_config: Some(snd_dmaengine_pcm_prepare_slave_config),
    prealloc_buffer_size: PAGE_SIZE * 8,
    dma_dev: core::ptr::null_mut(),
    chan_names: [core::ptr::null_mut(), core::ptr::null_mut()],
};

pub unsafe extern "C" fn tegra_pcm_platform_register(dev: *mut device) -> c_int {
    unsafe { snd_dmaengine_pcm_register(dev, &tegra_dmaengine_pcm_config, 0) }
}
// EXPORT_SYMBOL_GPL(tegra_pcm_platform_register);

pub unsafe extern "C" fn devm_tegra_pcm_platform_register(dev: *mut device) -> c_int {
    unsafe { devm_snd_dmaengine_pcm_register(dev, &tegra_dmaengine_pcm_config, 0) }
}
// EXPORT_SYMBOL_GPL(devm_tegra_pcm_platform_register);

pub unsafe extern "C" fn tegra_pcm_platform_register_with_chan_names(
    dev: *mut device,
    config: *mut snd_dmaengine_pcm_config,
    txdmachan: *mut c_char,
    rxdmachan: *mut c_char,
) -> c_int {
    unsafe {
        *config = tegra_dmaengine_pcm_config;
        (*config).dma_dev = (*dev).parent;
        (*config).chan_names[0] = txdmachan;
        (*config).chan_names[1] = rxdmachan;

        snd_dmaengine_pcm_register(dev, config, 0)
    }
}
// EXPORT_SYMBOL_GPL(tegra_pcm_platform_register_with_chan_names);

pub unsafe extern "C" fn tegra_pcm_platform_unregister(dev: *mut device) {
    unsafe {
        return snd_dmaengine_pcm_unregister(dev);
    }
}
// EXPORT_SYMBOL_GPL(tegra_pcm_platform_unregister);

pub unsafe extern "C" fn tegra_pcm_open(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    unsafe {
        let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
        let dmap: *mut snd_dmaengine_dai_dma_data;
        let chan: *mut dma_chan;
        let cpu_dai: *mut snd_soc_dai = snd_soc_rtd_to_cpu(rtd, 0);
        let mut ret: c_int;

        if (*(*rtd).dai_link).no_pcm {
            return 0;
        }

        dmap = snd_soc_dai_get_dma_data(cpu_dai, substream);

        /* Set HW params now that initialization is complete */
        snd_soc_set_runtime_hwparams(substream, &tegra_pcm_hardware);

        /* Ensure period size is multiple of 8 */
        ret = snd_pcm_hw_constraint_step(
            (*substream).runtime,
            0,
            SNDRV_PCM_HW_PARAM_PERIOD_BYTES,
            0x8,
        );
        if ret != 0 {
            dev_err(
                (*rtd).dev,
                b"failed to set constraint %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }

        chan = dma_request_chan((*cpu_dai).dev, (*dmap).chan_name);
        if IS_ERR(chan as *const c_void) {
            dev_err(
                (*cpu_dai).dev,
                b"dmaengine request slave channel failed! (%s)\n\0".as_ptr() as *const c_char,
                (*dmap).chan_name,
            );
            return -ENODEV;
        }

        ret = snd_dmaengine_pcm_open(substream, chan);
        if ret != 0 {
            dev_err(
                (*rtd).dev,
                b"dmaengine pcm open failed with err %d (%s)\n\0".as_ptr() as *const c_char,
                ret,
                (*dmap).chan_name,
            );

            dma_release_channel(chan);

            return ret;
        }

        /* Set wait time to 500ms by default */
        (*substream).wait_time = 500;

        0
    }
}
// EXPORT_SYMBOL_GPL(tegra_pcm_open);

pub unsafe extern "C" fn tegra_pcm_close(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    unsafe {
        let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);

        if (*(*rtd).dai_link).no_pcm {
            return 0;
        }

        snd_dmaengine_pcm_close_release_chan(substream);

        0
    }
}
// EXPORT_SYMBOL_GPL(tegra_pcm_close);

pub unsafe extern "C" fn tegra_pcm_hw_params(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    unsafe {
        let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
        let dmap: *mut snd_dmaengine_dai_dma_data;
        let mut slave_config: dma_slave_config = core::mem::zeroed();
        let chan: *mut dma_chan;
        let mut ret: c_int;

        if (*(*rtd).dai_link).no_pcm {
            return 0;
        }

        dmap = snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(rtd, 0), substream);
        if dmap.is_null() {
            return 0;
        }

        chan = snd_dmaengine_pcm_get_chan(substream);

        ret = snd_hwparams_to_dma_slave_config(substream, params, &mut slave_config);
        if ret != 0 {
            dev_err(
                (*rtd).dev,
                b"hw params config failed with err %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }

        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            slave_config.dst_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
            slave_config.dst_addr = (*dmap).addr;
            slave_config.dst_maxburst = 8;
        } else {
            slave_config.src_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
            slave_config.src_addr = (*dmap).addr;
            slave_config.src_maxburst = 8;
        }

        ret = dmaengine_slave_config(chan, &mut slave_config);
        if ret < 0 {
            dev_err(
                (*rtd).dev,
                b"dma slave config failed with err %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }

        0
    }
}
// EXPORT_SYMBOL_GPL(tegra_pcm_hw_params);

pub unsafe extern "C" fn tegra_pcm_pointer(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    unsafe { snd_dmaengine_pcm_pointer(substream) }
}
// EXPORT_SYMBOL_GPL(tegra_pcm_pointer);

unsafe extern "C" fn tegra_pcm_dma_allocate(
    dev: *mut device,
    rtd: *mut snd_soc_pcm_runtime,
    size: size_t,
) -> c_int {
    unsafe {
        let pcm: *mut snd_pcm = (*rtd).pcm;
        let ret: c_int;

        ret = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(32));
        if ret < 0 {
            return ret;
        }

        snd_pcm_set_fixed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV_WC, dev, size)
    }
}

pub unsafe extern "C" fn tegra_pcm_new(
    component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    unsafe {
        let mut dev: *mut device = (*component).dev;

        /*
         * Fallback for backwards-compatibility with older device trees that
         * have the iommus property in the virtual, top-level "sound" node.
         */
        if !of_property_present((*dev).of_node, b"iommus\0".as_ptr() as *const c_char) {
            dev = (*(*(*rtd).card).snd_card).dev;
        }

        tegra_pcm_dma_allocate(dev, rtd, tegra_pcm_hardware.buffer_bytes_max)
    }
}
// EXPORT_SYMBOL_GPL(tegra_pcm_new);

// MODULE_AUTHOR("Stephen Warren <swarren@nvidia.com>");
// MODULE_DESCRIPTION("Tegra PCM ASoC driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
