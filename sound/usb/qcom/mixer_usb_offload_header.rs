// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2022-2025 Qualcomm Innovation Center, Inc. All rights reserved.

use std::os::raw::c_int;

// Opaque types from external translation units
pub struct snd_usb_audio;
pub struct device;

extern "C" {
    pub fn snd_usb_offload_create_ctl(
        chip: *mut snd_usb_audio,
        bedev: *mut device,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
