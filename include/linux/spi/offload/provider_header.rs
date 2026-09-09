/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2024 Analog Devices Inc.
 * Copyright (C) 2024 BayLibre, SAS
 */

/* MODULE_IMPORT_NS("SPI_OFFLOAD"); */

use core::ffi::c_void;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spi_offload_trigger {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spi_offload;

#[repr(C)]
pub struct fwnode_handle;

/* Supplied by linux/spi/offload/types.h. */
#[allow(non_camel_case_types)]
pub type spi_offload_trigger_type = u32;

#[repr(C)]
pub struct spi_offload_trigger_config {
    _private: [u8; 0],
}

extern "C" {
    pub fn devm_spi_offload_alloc(dev: *mut device, priv_size: usize) -> *mut spi_offload;
}

#[repr(C)]
pub struct spi_offload_trigger_ops {
    pub r#match: Option<
        unsafe extern "C" fn(
            trigger: *mut spi_offload_trigger,
            r#type: spi_offload_trigger_type,
            args: *mut u64,
            nargs: u32,
        ) -> bool,
    >,
    pub request: Option<
        unsafe extern "C" fn(
            trigger: *mut spi_offload_trigger,
            r#type: spi_offload_trigger_type,
            args: *mut u64,
            nargs: u32,
        ) -> i32,
    >,
    pub release: Option<unsafe extern "C" fn(trigger: *mut spi_offload_trigger)>,
    pub validate: Option<
        unsafe extern "C" fn(
            trigger: *mut spi_offload_trigger,
            config: *mut spi_offload_trigger_config,
        ) -> i32,
    >,
    pub enable: Option<
        unsafe extern "C" fn(
            trigger: *mut spi_offload_trigger,
            config: *mut spi_offload_trigger_config,
        ) -> i32,
    >,
    pub disable: Option<unsafe extern "C" fn(trigger: *mut spi_offload_trigger)>,
}

#[repr(C)]
pub struct spi_offload_trigger_info {
    /** @fwnode: Provider fwnode, used to match to consumer. */
    pub fwnode: *mut fwnode_handle,
    /** @ops: Provider-specific callbacks. */
    pub ops: *const spi_offload_trigger_ops,
    /** Provider-specific state to be used in callbacks. */
    pub priv_: *mut c_void,
}

extern "C" {
    pub fn devm_spi_offload_trigger_register(
        dev: *mut device,
        info: *mut spi_offload_trigger_info,
    ) -> i32;
    pub fn spi_offload_trigger_get_priv(trigger: *mut spi_offload_trigger) -> *mut c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
