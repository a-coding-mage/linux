// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2022 Intel Corporation
 */

// Forward declarations from the C header.
#[repr(C)]
pub struct snd_dma_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn snd_sof_create_page_table(
        dev: *mut device,
        dmab: *mut snd_dma_buffer,
        page_table: *mut ::core::ffi::c_uchar,
        size: usize,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
