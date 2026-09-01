// SPDX-License-Identifier: GPL-2.0-only
// SPDX-FileCopyrightText: Copyright (c) 2025 NVIDIA CORPORATION & AFFILIATES.
// All rights reserved.
//
// ADMA bandwidth calculation

// C dependencies:
// #include <linux/interconnect.h>
// #include <linux/module.h>
// #include <sound/dmaengine_pcm.h>
// #include <sound/pcm_params.h>
// #include <sound/soc.h>
// #include "tegra_isomgr_bw.h"
// #include "tegra210_admaif.h"

const MAX_SAMPLE_RATE: u32 = 192; /* KHz*/
const MAX_BYTES_PER_SAMPLE: u32 = 4;

const GFP_KERNEL: u32 = 0;
const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;

extern "C" {
    static STREAM_TYPE: u32;

    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut core::ffi::c_void;
    fn snd_pcm_format_width(format: i32) -> i32;
    fn icc_set_bw(path: *mut core::ffi::c_void, avg_bw: u32, peak_bw: u32) -> i32;

    fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_kfree(dev: *mut device, p: *mut core::ffi::c_void);
    fn devm_of_icc_get(dev: *mut device, name: *const core::ffi::c_char) -> *mut core::ffi::c_void;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> isize;
    fn dev_err_probe(
        dev: *mut device,
        err: isize,
        fmt: *const core::ffi::c_char,
        ...
    ) -> i32;

    fn mutex_init(lock: *mut mutex);
    fn mutex_destroy(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);

    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const core::ffi::c_char, ...);
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub pcm: *mut snd_pcm,
    pub stream: u32,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub format: i32,
    pub channels: u32,
    pub rate: u32,
}

#[repr(C)]
pub struct snd_pcm {
    pub device: u32,
}

#[repr(C)]
pub struct tegra_admaif {
    pub adma_isomgr: *mut tegra_adma_isomgr,
    pub soc_data: *mut tegra_admaif_soc_data,
}

#[repr(C)]
pub struct tegra_admaif_soc_data {
    pub num_ch: u32,
    pub max_stream_ch: u32,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tegra_adma_isomgr {
    pub icc_path_handle: *mut core::ffi::c_void,
    pub max_pcm_device: u32,
    pub max_bw: u32,
    pub bw_per_dev: *mut *mut u32,
    pub current_bandwidth: u32,
    pub mutex: mutex,
}

#[no_mangle]
pub unsafe extern "C" fn tegra_isomgr_adma_setbw(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
    is_running: bool,
) -> i32 {
    let dev = (*dai).dev;
    let admaif = snd_soc_dai_get_drvdata(dai) as *mut tegra_admaif;
    let adma_isomgr = (*admaif).adma_isomgr;
    let runtime = (*substream).runtime;
    let pcm = (*substream).pcm;
    let type_ = (*substream).stream;
    let mut bandwidth: u32 = 0;
    let sample_bytes: i32;

    if adma_isomgr.is_null() {
        return 0;
    }

    if runtime.is_null() || pcm.is_null() {
        return -EINVAL;
    }

    if (*pcm).device >= (*adma_isomgr).max_pcm_device {
        dev_err(
            dev,
            b"%s: PCM device number %d is greater than %d\n\0".as_ptr()
                as *const core::ffi::c_char,
            b"tegra_isomgr_adma_setbw\0".as_ptr() as *const core::ffi::c_char,
            (*pcm).device,
            (*adma_isomgr).max_pcm_device,
        );
        return -EINVAL;
    }

    /*
     * No action if  stream is running and bandwidth is already set or
     * stream is not running and bandwidth is already reset
     */
    let bw_for_type = *(*adma_isomgr).bw_per_dev.add(type_ as usize);
    let bw_for_dev = bw_for_type.add((*pcm).device as usize);
    if ((*bw_for_dev != 0) && is_running) || ((*bw_for_dev == 0) && !is_running) {
        return 0;
    }

    if is_running {
        sample_bytes = snd_pcm_format_width((*runtime).format) / 8;
        if sample_bytes < 0 {
            return sample_bytes;
        }

        /* KB/s kilo bytes per sec */
        bandwidth = (*runtime)
            .channels
            .wrapping_mul((*runtime).rate / 1000)
            .wrapping_mul(sample_bytes as u32);
    }

    mutex_lock(&mut (*adma_isomgr).mutex);
    if is_running {
        if bandwidth.wrapping_add((*adma_isomgr).current_bandwidth) > (*adma_isomgr).max_bw {
            bandwidth = (*adma_isomgr)
                .max_bw
                .wrapping_sub((*adma_isomgr).current_bandwidth);
        }

        (*adma_isomgr).current_bandwidth =
            (*adma_isomgr).current_bandwidth.wrapping_add(bandwidth);
    } else {
        (*adma_isomgr).current_bandwidth =
            (*adma_isomgr).current_bandwidth.wrapping_sub(*bw_for_dev);
    }
    mutex_unlock(&mut (*adma_isomgr).mutex);

    *bw_for_dev = bandwidth;

    dev_dbg(
        dev,
        b"Setting up bandwidth to %d KBps\n\0".as_ptr() as *const core::ffi::c_char,
        (*adma_isomgr).current_bandwidth,
    );

    icc_set_bw(
        (*adma_isomgr).icc_path_handle,
        (*adma_isomgr).current_bandwidth,
        (*adma_isomgr).max_bw,
    )
}

#[no_mangle]
pub unsafe extern "C" fn tegra_isomgr_adma_register(dev: *mut device) -> i32 {
    let admaif = dev_get_drvdata(dev) as *mut tegra_admaif;
    let adma_isomgr: *mut tegra_adma_isomgr;
    let mut i: i32;

    adma_isomgr = devm_kzalloc(
        dev,
        core::mem::size_of::<tegra_adma_isomgr>(),
        GFP_KERNEL,
    ) as *mut tegra_adma_isomgr;
    if adma_isomgr.is_null() {
        return -ENOMEM;
    }

    (*adma_isomgr).icc_path_handle =
        devm_of_icc_get(dev, b"write\0".as_ptr() as *const core::ffi::c_char);
    if IS_ERR((*adma_isomgr).icc_path_handle) {
        return dev_err_probe(
            dev,
            PTR_ERR((*adma_isomgr).icc_path_handle),
            b"failed to acquire interconnect path\n\0".as_ptr() as *const core::ffi::c_char,
        );
    }

    /* Either INTERCONNECT config OR interconnect property is not defined */
    if (*adma_isomgr).icc_path_handle.is_null() {
        devm_kfree(dev, adma_isomgr as *mut core::ffi::c_void);
        return 0;
    }

    (*adma_isomgr).max_pcm_device = (*(*admaif).soc_data).num_ch;
    (*adma_isomgr).max_bw = STREAM_TYPE
        .wrapping_mul(MAX_SAMPLE_RATE)
        .wrapping_mul(MAX_BYTES_PER_SAMPLE)
        .wrapping_mul((*(*admaif).soc_data).max_stream_ch)
        .wrapping_mul((*adma_isomgr).max_pcm_device);

    i = 0;
    while i < STREAM_TYPE as i32 {
        let slot = (*adma_isomgr).bw_per_dev.add(i as usize);
        *slot = devm_kzalloc(
            dev,
            ((*adma_isomgr).max_pcm_device as usize).wrapping_mul(core::mem::size_of::<u32>()),
            GFP_KERNEL,
        ) as *mut u32;
        if (*slot).is_null() {
            return -ENOMEM;
        }
        i += 1;
    }

    (*adma_isomgr).current_bandwidth = 0;
    mutex_init(&mut (*adma_isomgr).mutex);
    (*admaif).adma_isomgr = adma_isomgr;

    0
}

#[no_mangle]
pub unsafe extern "C" fn tegra_isomgr_adma_unregister(dev: *mut device) {
    let admaif = dev_get_drvdata(dev) as *mut tegra_admaif;

    if (*admaif).adma_isomgr.is_null() {
        return;
    }

    mutex_destroy(&mut (*(*admaif).adma_isomgr).mutex);
}

// MODULE_AUTHOR("Mohan Kumar <mkumard@nvidia.com>");
// MODULE_DESCRIPTION("Tegra ADMA Bandwidth Request driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
