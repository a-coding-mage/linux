/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (ST) 2012 Vipin Kumar (vipin.kumar@st.com)
 */

use core::ffi::c_void;

// Opaque type supplied by the DMA subsystem.
#[repr(C)]
pub struct dma_chan {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spear_spdif_platform_data {
    /* DMA params */
    pub dma_params: *mut c_void,
    pub filter: Option<unsafe extern "C" fn(chan: *mut dma_chan, slave: *mut c_void) -> bool>,
    pub reset_perip: Option<unsafe extern "C" fn()>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
