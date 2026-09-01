/* SPDX-License-Identifier: GPL-2.0-only
 * SPDX-FileCopyrightText: Copyright (c) 2025 NVIDIA CORPORATION & AFFILIATES.
 * All rights reserved.
 *
 * tegra_isomgr_bw.h - Definitions for ADMA bandwidth calculation
 *
 */

/* Header guard __TEGRA_ISOMGR_BW_H__ omitted in Rust. */

/* Playback and Capture streams */
pub const STREAM_TYPE: usize = 2;

#[repr(C)]
pub struct tegra_adma_isomgr {
    /* Protect pcm devices bandwidth */
    pub mutex: mutex,
    /* interconnect path handle */
    pub icc_path_handle: *mut icc_path,
    pub bw_per_dev: [*mut u32; STREAM_TYPE],
    pub current_bandwidth: u32,
    pub max_pcm_device: u32,
    pub max_bw: u32,
}

unsafe extern "C" {
    pub fn tegra_isomgr_adma_register(dev: *mut device) -> core::ffi::c_int;
    pub fn tegra_isomgr_adma_unregister(dev: *mut device);
    pub fn tegra_isomgr_adma_setbw(
        substream: *mut snd_pcm_substream,
        dai: *mut snd_soc_dai,
        is_running: bool,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
