// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2023 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

// Opaque external C type from included kernel headers.
#[repr(C)]
pub struct device {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn cirrus_scodec_get_speaker_id(
        dev: *mut device,
        amp_index: core::ffi::c_int,
        num_amps: core::ffi::c_int,
        fixed_gpio_id: core::ffi::c_int,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
