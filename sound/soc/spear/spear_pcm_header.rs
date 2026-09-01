// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2013, NVIDIA CORPORATION.  All rights reserved.
 */

// Header guard __SPEAR_PCM_H__ omitted in Rust.

extern "C" {
    pub fn devm_spear_pcm_platform_register(
        dev: *mut device,
        config: *mut snd_dmaengine_pcm_config,
        filter: Option<unsafe extern "C" fn(chan: *mut dma_chan, slave: *mut core::ffi::c_void) -> bool>,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
