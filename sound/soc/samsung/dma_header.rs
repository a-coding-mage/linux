/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * ALSA PCM interface for the Samsung SoC
 */

// C dependency: <sound/dmaengine_pcm.h>

/*
 * @tx, @rx arguments can be NULL if the DMA channel names are "tx", "rx",
 * otherwise actual DMA channel names must be passed to this function.
 */
unsafe extern "C" {
    pub fn samsung_asoc_dma_platform_register(
        dev: *mut device,
        filter: dma_filter_fn,
        tx: *const ::core::ffi::c_char,
        rx: *const ::core::ffi::c_char,
        dma_dev: *mut device,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
