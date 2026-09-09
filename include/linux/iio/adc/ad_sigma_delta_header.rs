/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Support code for Analog Devices Sigma-Delta ADCs
 *
 * Copyright 2012 Analog Devices Inc.
 *  Author: Lars-Peter Clausen <lars@metafoo.de>
 */

// Dependency declarations supplied by the surrounding kernel translation.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ad_sigma_delta_mode {
    AD_SD_MODE_CONTINUOUS = 0,
    AD_SD_MODE_SINGLE = 1,
    AD_SD_MODE_IDLE = 2,
    AD_SD_MODE_POWERDOWN = 3,
}

/**
 * struct ad_sigma_delta_calib_data - Calibration data for Sigma Delta devices
 * @mode: Calibration mode.
 * @channel: Calibration channel.
 */
#[repr(C)]
pub struct ad_sd_calib_data {
    pub mode: u32,
    pub channel: u32,
}

#[repr(C)]
pub struct ad_sigma_delta;
#[repr(C)]
pub struct device;
#[repr(C)]
pub struct gpio_desc;
#[repr(C)]
pub struct iio_dev;
#[repr(C)]
pub struct spi_device;
#[repr(C)]
pub struct iio_trigger;
#[repr(C)]
pub struct completion;
#[repr(C)]
pub struct spinlock_t;
#[repr(C)]
pub struct spi_message;
#[repr(C)]
pub struct spi_transfer;
#[repr(C)]
pub struct spi_offload;
#[repr(C)]
pub struct spi_offload_trigger;
#[repr(C)]
pub struct iio_chan_spec;

/**
 * struct ad_sigma_delta_info - Sigma Delta driver specific callbacks and options
 * Callback and field documentation preserved from the C header.
 */
#[repr(C)]
pub struct ad_sigma_delta_info {
    pub set_channel: Option<unsafe extern "C" fn(*mut ad_sigma_delta, u32) -> i32>,
    pub append_status: Option<unsafe extern "C" fn(*mut ad_sigma_delta, bool) -> i32>,
    pub set_mode: Option<unsafe extern "C" fn(*mut ad_sigma_delta, ad_sigma_delta_mode) -> i32>,
    pub disable_all: Option<unsafe extern "C" fn(*mut ad_sigma_delta) -> i32>,
    pub disable_one: Option<unsafe extern "C" fn(*mut ad_sigma_delta, u32) -> i32>,
    pub postprocess_sample: Option<unsafe extern "C" fn(*mut ad_sigma_delta, u32) -> i32>,
    pub has_registers: bool,
    pub has_named_irqs: bool,
    pub supports_spi_offload: bool,
    pub addr_shift: u32,
    pub read_mask: u32,
    pub status_ch_mask: u32,
    pub data_reg: u32,
    pub irq_flags: usize,
    pub num_slots: u32,
    pub num_resetclks: u32,
}

/** Sigma Delta device struct. */
#[repr(C)]
pub struct ad_sigma_delta {
    pub spi: *mut spi_device,
    pub trig: *mut iio_trigger,
    pub completion: completion,
    pub irq_lock: spinlock_t,
    pub irq_dis: bool,
    pub bus_locked: bool,
    pub keep_cs_asserted: bool,
    pub comm: u8,
    pub info: *const ad_sigma_delta_info,
    pub active_slots: u32,
    pub current_slot: u32,
    pub num_slots: u32,
    pub rdy_gpiod: *mut gpio_desc,
    pub irq_line: i32,
    pub status_appended: bool,
    pub slots: *mut u32,
    pub sample_msg: spi_message,
    pub sample_xfer: [spi_transfer; 2],
    pub samples_buf: *mut u8,
    pub offload: *mut spi_offload,
    pub offload_trigger: *mut spi_offload_trigger,
    // __aligned(IIO_DMA_MINALIGN)
    pub tx_buf: [u8; 4],
    // __aligned(8)
    pub rx_buf: [u8; 16],
    pub sample_addr: u8,
}

#[inline]
pub unsafe fn ad_sigma_delta_has_spi_offload(sd: *mut ad_sigma_delta) -> bool {
    !(*sd).offload.is_null()
}

#[inline]
pub unsafe fn ad_sigma_delta_set_channel(sd: *mut ad_sigma_delta, channel: u32) -> i32 {
    match (*(*sd).info).set_channel {
        Some(f) => f(sd, channel),
        None => 0,
    }
}

#[inline]
pub unsafe fn ad_sigma_delta_append_status(sd: *mut ad_sigma_delta, append: bool) -> i32 {
    if let Some(f) = (*(*sd).info).append_status {
        let ret = f(sd, append);
        if ret < 0 { return ret; }
        (*sd).status_appended = append;
    }
    0
}

#[inline]
pub unsafe fn ad_sigma_delta_disable_all(sd: *mut ad_sigma_delta) -> i32 {
    match (*(*sd).info).disable_all { Some(f) => f(sd), None => 0 }
}

#[inline]
pub unsafe fn ad_sigma_delta_disable_one(sd: *mut ad_sigma_delta, chan: u32) -> i32 {
    match (*(*sd).info).disable_one { Some(f) => f(sd, chan), None => 0 }
}

#[inline]
pub unsafe fn ad_sigma_delta_set_mode(sd: *mut ad_sigma_delta, mode: u32) -> i32 {
    match (*(*sd).info).set_mode { Some(f) => f(sd, mode as ad_sigma_delta_mode), None => 0 }
}

#[inline]
pub unsafe fn ad_sigma_delta_postprocess_sample(sd: *mut ad_sigma_delta, raw_sample: u32) -> i32 {
    match (*(*sd).info).postprocess_sample { Some(f) => f(sd, raw_sample), None => 0 }
}

extern "C" {
    pub fn ad_sd_set_comm(sigma_delta: *mut ad_sigma_delta, comm: u8);
    pub fn ad_sd_write_reg(sigma_delta: *mut ad_sigma_delta, reg: u32, size: u32, val: u32) -> i32;
    pub fn ad_sd_read_reg(sigma_delta: *mut ad_sigma_delta, reg: u32, size: u32, val: *mut u32) -> i32;
    pub fn ad_sd_reset(sigma_delta: *mut ad_sigma_delta) -> i32;
    pub fn ad_sigma_delta_single_conversion(indio_dev: *mut iio_dev, chan: *const iio_chan_spec, val: *mut i32) -> i32;
    pub fn ad_sd_calibrate(sigma_delta: *mut ad_sigma_delta, mode: u32, channel: u32) -> i32;
    pub fn ad_sd_calibrate_all(sigma_delta: *mut ad_sigma_delta, cd: *const ad_sd_calib_data, n: u32) -> i32;
    pub fn ad_sd_init(sigma_delta: *mut ad_sigma_delta, indio_dev: *mut iio_dev, spi: *mut spi_device, info: *const ad_sigma_delta_info) -> i32;
    pub fn devm_ad_sd_setup_buffer_and_trigger(dev: *mut device, indio_dev: *mut iio_dev) -> i32;
    pub fn ad_sd_validate_trigger(indio_dev: *mut iio_dev, trig: *mut iio_trigger) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
