/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * hdmi-audio.c -- OMAP4+ DSS HDMI audio support library
 *
 * Copyright (C) 2014 Texas Instruments Incorporated - https://www.ti.com
 *
 * Author: Jyri Sarha <jsarha@ti.com>
 */

// Dependency supplied by the surrounding kernel translation.
use core::ffi::c_void;

// Opaque external C types referenced by this header.
pub type Device = c_void;
pub type SndAesIec958 = c_void;
pub type SndCea861AudIf = c_void;
pub type PhysAddr = usize;

#[repr(C)]
pub struct omap_dss_audio {
    pub iec: *mut SndAesIec958,
    pub cea: *mut SndCea861AudIf,
}

#[repr(C)]
pub struct omap_hdmi_audio_ops {
    pub audio_startup:
        Option<unsafe extern "C" fn(dev: *mut Device, abort_cb: Option<unsafe extern "C" fn(dev: *mut Device)>) -> i32>,
    pub audio_shutdown: Option<unsafe extern "C" fn(dev: *mut Device) -> i32>,
    pub audio_start: Option<unsafe extern "C" fn(dev: *mut Device) -> i32>,
    pub audio_stop: Option<unsafe extern "C" fn(dev: *mut Device)>,
    pub audio_config:
        Option<unsafe extern "C" fn(dev: *mut Device, dss_audio: *mut omap_dss_audio) -> i32>,
}

/* HDMI audio initalization data */
#[repr(C)]
pub struct omap_hdmi_audio_pdata {
    pub dev: *mut Device,
    pub version: u32,
    pub audio_dma_addr: PhysAddr,
    pub ops: *const omap_hdmi_audio_ops,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
