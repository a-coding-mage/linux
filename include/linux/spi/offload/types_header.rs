/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2024 Analog Devices Inc.
 * Copyright (C) 2024 BayLibre, SAS
 */

/* Translated from the Linux SPI offload types header. */

use core::ffi::c_void;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dma_chan {
    _private: [u8; 0],
}

/* This is write xfer but TX uses external data stream rather than tx_buf. */
pub const SPI_OFFLOAD_XFER_TX_STREAM: u32 = 1u32 << 0;
/* This is read xfer but RX uses external data stream rather than rx_buf. */
pub const SPI_OFFLOAD_XFER_RX_STREAM: u32 = 1u32 << 1;

/* Offload can be triggered by external hardware event. */
pub const SPI_OFFLOAD_CAP_TRIGGER: u32 = 1u32 << 0;
/* Offload can record and then play back TX data when triggered. */
pub const SPI_OFFLOAD_CAP_TX_STATIC_DATA: u32 = 1u32 << 1;
/* Offload can get TX data from an external stream source. */
pub const SPI_OFFLOAD_CAP_TX_STREAM_DMA: u32 = 1u32 << 2;
/* Offload can send RX data to an external stream sink. */
pub const SPI_OFFLOAD_CAP_RX_STREAM_DMA: u32 = 1u32 << 3;

/**
 * struct spi_offload_config - offload configuration
 *
 * This is used to request an offload with specific configuration.
 */
#[repr(C)]
pub struct spi_offload_config {
    /** @capability_flags: required capabilities. See %SPI_OFFLOAD_CAP_* */
    pub capability_flags: u32,
}

/**
 * struct spi_offload - offload instance
 */
#[repr(C)]
pub struct spi_offload {
    /** @provider_dev: for get/put reference counting */
    pub provider_dev: *mut device,
    /** @priv: provider driver private data */
    pub priv_: *mut c_void,
    /** @ops: callbacks for offload support */
    pub ops: *const spi_offload_ops,
    /** @xfer_flags: %SPI_OFFLOAD_XFER_* flags supported by provider */
    pub xfer_flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum spi_offload_trigger_type {
    /* Indication from SPI peripheral that data is read to read. */
    SPI_OFFLOAD_TRIGGER_DATA_READY,
    /* Trigger comes from a periodic source such as a clock. */
    SPI_OFFLOAD_TRIGGER_PERIODIC,
}

/**
 * spi_offload_trigger_periodic - configuration parameters for periodic triggers
 * @frequency_hz: The rate that the trigger should fire in Hz.
 * @offset_ns: A delay in nanoseconds between when this trigger fires
 *       compared to another trigger. This requires specialized hardware
 *       that supports such synchronization with a delay between two or
 *       more triggers. Set to 0 when not needed.
 */
#[repr(C)]
pub struct spi_offload_trigger_periodic {
    pub frequency_hz: u64,
    pub offset_ns: u64,
}

#[repr(C)]
pub union spi_offload_trigger_config__bindgen_ty_1 {
    pub periodic: spi_offload_trigger_periodic,
}

#[repr(C)]
pub struct spi_offload_trigger_config {
    /** @type: type discriminator for union */
    pub type_: spi_offload_trigger_type,
    pub periodic: spi_offload_trigger_config__bindgen_ty_1,
}

/**
 * struct spi_offload_ops - callbacks implemented by offload providers
 */
#[repr(C)]
pub struct spi_offload_ops {
    /**
     * @trigger_enable: Optional callback to enable the trigger for the
     * given offload instance.
     */
    pub trigger_enable: Option<unsafe extern "C" fn(offload: *mut spi_offload) -> i32>,
    /**
     * @trigger_disable: Optional callback to disable the trigger for the
     * given offload instance.
     */
    pub trigger_disable: Option<unsafe extern "C" fn(offload: *mut spi_offload)>,
    /**
     * @tx_stream_request_dma_chan: Optional callback for controllers that
     * have an offload where the TX data stream is connected directly to a
     * DMA channel.
     */
    pub tx_stream_request_dma_chan:
        Option<unsafe extern "C" fn(offload: *mut spi_offload) -> *mut dma_chan>,
    /**
     * @rx_stream_request_dma_chan: Optional callback for controllers that
     * have an offload where the RX data stream is connected directly to a
     * DMA channel.
     */
    pub rx_stream_request_dma_chan:
        Option<unsafe extern "C" fn(offload: *mut spi_offload) -> *mut dma_chan>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
