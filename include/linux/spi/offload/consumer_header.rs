/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2024 Analog Devices Inc.
 * Copyright (C) 2024 BayLibre, SAS
 */

// C dependencies:
// #include <linux/module.h>
// #include <linux/spi/offload/types.h>
// #include <linux/types.h>
// MODULE_IMPORT_NS("SPI_OFFLOAD");

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spi_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spi_offload {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spi_offload_config {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spi_offload_trigger {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spi_offload_trigger_config {
    _private: [u8; 0],
}

pub type spi_offload_trigger_type = i32;

#[repr(C)]
pub struct dma_chan {
    _private: [u8; 0],
}

extern "C" {
    pub fn devm_spi_offload_get(
        dev: *mut device,
        spi: *mut spi_device,
        config: *const spi_offload_config,
    ) -> *mut spi_offload;

    pub fn devm_spi_offload_trigger_get(
        dev: *mut device,
        offload: *mut spi_offload,
        type_: spi_offload_trigger_type,
    ) -> *mut spi_offload_trigger;

    pub fn spi_offload_trigger_validate(
        trigger: *mut spi_offload_trigger,
        config: *mut spi_offload_trigger_config,
    ) -> ::core::ffi::c_int;

    pub fn spi_offload_trigger_enable(
        offload: *mut spi_offload,
        trigger: *mut spi_offload_trigger,
        config: *mut spi_offload_trigger_config,
    ) -> ::core::ffi::c_int;

    pub fn spi_offload_trigger_disable(
        offload: *mut spi_offload,
        trigger: *mut spi_offload_trigger,
    );

    pub fn devm_spi_offload_tx_stream_request_dma_chan(
        dev: *mut device,
        offload: *mut spi_offload,
    ) -> *mut dma_chan;

    pub fn devm_spi_offload_rx_stream_request_dma_chan(
        dev: *mut device,
        offload: *mut spi_offload,
    ) -> *mut dma_chan;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
