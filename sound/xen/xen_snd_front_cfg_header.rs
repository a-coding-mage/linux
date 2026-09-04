// SPDX-License-Identifier: GPL-2.0 OR MIT

// Xen para-virtual sound device
//
// Copyright (C) 2016-2018 EPAM Systems Inc.
//
// Author: Oleksandr Andrushchenko <oleksandr_andrushchenko@epam.com>

// Requires: sound/core.h, sound/pcm.h (external kernel headers)

use std::ffi::c_int;
use std::os::raw::c_char;

// Forward declarations - defined elsewhere
pub struct xen_snd_front_info;
pub struct snd_pcm_hardware;

#[repr(C)]
pub struct xen_front_cfg_stream {
    pub index: c_int,
    pub xenstore_path: *const c_char,
    pub pcm_hw: snd_pcm_hardware,
}

#[repr(C)]
pub struct xen_front_cfg_pcm_instance {
    pub name: [c_char; 80],
    pub device_id: c_int,
    pub pcm_hw: snd_pcm_hardware,
    pub num_streams_pb: c_int,
    pub streams_pb: *const xen_front_cfg_stream,
    pub num_streams_cap: c_int,
    pub streams_cap: *const xen_front_cfg_stream,
}

#[repr(C)]
pub struct xen_front_cfg_card {
    pub name_short: [c_char; 32],
    pub name_long: [c_char; 80],
    pub pcm_hw: snd_pcm_hardware,
    pub num_pcm_instances: c_int,
    pub pcm_instances: *const xen_front_cfg_pcm_instance,
}

extern "C" {
    pub fn xen_snd_front_cfg_card(
        front_info: *const xen_snd_front_info,
        stream_cnt: *mut c_int,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
