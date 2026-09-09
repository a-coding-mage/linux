/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 */

// fsl dma API for external start
extern "C" {
    pub fn fsl_dma_external_start(dchan: *mut dma_chan, enable: ::core::ffi::c_int)
        -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
