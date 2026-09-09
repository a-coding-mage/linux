/* SPDX-License-Identifier: GPL-2.0 */
/*
 * The MIPI SDCA specification is available for public downloads at
 * https://www.mipi.org/mipi-sdca-v1-0-download
 *
 * Copyright (C) 2025 Cirrus Logic, Inc. and
 *                    Cirrus Logic International Semiconductor Ltd.
 */

// Linux completion, delayed-work, and mutex types are supplied by dependencies.

pub struct device;
pub struct regmap;
pub struct sdca_fdl_set;
pub struct sdca_function_data;
pub struct sdca_interrupt;
pub struct sdca_interrupt_info;

/**
 * struct fdl_state - FDL state structure to keep data between interrupts
 * @begin: Completion indicating the start of an FDL download cycle.
 * @done: Completion indicating the end of an FDL download cycle.
 * @timeout: Delayed work used for timing out UMP transactions.
 * @lock: Mutex to protect between the timeout work and IRQ handlers.
 * @interrupt: Pointer to the interrupt struct to which this FDL is attached.
 * @set: Pointer to the FDL set currently being downloaded.
 * @file_index: Index of the current file being processed.
 */
#[repr(C)]
pub struct fdl_state {
    pub begin: completion,
    pub done: completion,
    pub timeout: delayed_work,
    pub lock: mutex,
    pub interrupt: *mut sdca_interrupt,
    pub set: *mut sdca_fdl_set,
    pub file_index: i32,
}

pub const SDCA_CTL_XU_FDLH_COMPLETE: u32 = 0;
pub const SDCA_CTL_XU_FDLH_MORE_FILES: u32 = SDCA_CTL_XU_FDLH_SET_IN_PROGRESS;
pub const SDCA_CTL_XU_FDLH_FILE_AVAILABLE: u32 =
    SDCA_CTL_XU_FDLH_TRANSFERRED_FILE | SDCA_CTL_XU_FDLH_SET_IN_PROGRESS;
pub const SDCA_CTL_XU_FDLH_MASK: u32 =
    SDCA_CTL_XU_FDLH_TRANSFERRED_CHUNK
    | SDCA_CTL_XU_FDLH_TRANSFERRED_FILE
    | SDCA_CTL_XU_FDLH_SET_IN_PROGRESS
    | SDCA_CTL_XU_FDLH_RESET_ACK
    | SDCA_CTL_XU_FDLH_REQ_ABORT;

pub const SDCA_CTL_XU_FDLD_COMPLETE: u32 = 0;
pub const SDCA_CTL_XU_FDLD_FILE_OK: u32 =
    SDCA_CTL_XU_FDLH_TRANSFERRED_FILE
    | SDCA_CTL_XU_FDLH_SET_IN_PROGRESS
    | SDCA_CTL_XU_FDLD_ACK_TRANSFER
    | SDCA_CTL_XU_FDLD_NEEDS_SET;
pub const SDCA_CTL_XU_FDLD_MORE_FILES_OK: u32 =
    SDCA_CTL_XU_FDLH_SET_IN_PROGRESS
    | SDCA_CTL_XU_FDLD_ACK_TRANSFER
    | SDCA_CTL_XU_FDLD_NEEDS_SET;
pub const SDCA_CTL_XU_FDLD_MASK: u32 =
    SDCA_CTL_XU_FDLD_REQ_RESET
    | SDCA_CTL_XU_FDLD_REQ_ABORT
    | SDCA_CTL_XU_FDLD_ACK_TRANSFER
    | SDCA_CTL_XU_FDLD_NEEDS_SET;

// CONFIG_SND_SOC_SDCA_FDL is represented by the equivalent Rust feature.
#[cfg(feature = "CONFIG_SND_SOC_SDCA_FDL")]
extern "C" {
    pub fn sdca_fdl_alloc_state(interrupt: *mut sdca_interrupt) -> i32;
    pub fn sdca_fdl_free_state(interrupt: *mut sdca_interrupt);
    pub fn sdca_fdl_process(interrupt: *mut sdca_interrupt) -> i32;
    pub fn sdca_fdl_sync(
        dev: *mut device,
        function: *mut sdca_function_data,
        info: *mut sdca_interrupt_info,
    ) -> i32;
    pub fn sdca_reset_function(
        dev: *mut device,
        function: *mut sdca_function_data,
        regmap: *mut regmap,
    ) -> i32;
}

#[cfg(not(feature = "CONFIG_SND_SOC_SDCA_FDL"))]
pub unsafe fn sdca_fdl_alloc_state(_interrupt: *mut sdca_interrupt) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_SND_SOC_SDCA_FDL"))]
pub unsafe fn sdca_fdl_free_state(_interrupt: *mut sdca_interrupt) {}

#[cfg(not(feature = "CONFIG_SND_SOC_SDCA_FDL"))]
pub unsafe fn sdca_fdl_process(_interrupt: *mut sdca_interrupt) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_SND_SOC_SDCA_FDL"))]
pub unsafe fn sdca_fdl_sync(
    _dev: *mut device,
    _function: *mut sdca_function_data,
    _info: *mut sdca_interrupt_info,
) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_SND_SOC_SDCA_FDL"))]
pub unsafe fn sdca_reset_function(
    _dev: *mut device,
    _function: *mut sdca_function_data,
    _regmap: *mut regmap,
) -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
