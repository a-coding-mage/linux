/***********************license start***************
 * Author: Cavium Networks
 *
 * Contact: support@caviumnetworks.com
 * This file is part of the OCTEON SDK
 *
 * Copyright (c) 2003-2008 Cavium Networks
 *
 * This file is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License, version 2, as
 * published by the Free Software Foundation.
 *
 * This file is distributed in the hope that it will be useful, but
 * AS-IS and WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE, TITLE, or
 * NONINFRINGEMENT. See the GNU General Public License for more details.
 ***********************license end**************************************/

/* This file contains definitions for the SPI interface. */
/* C header dependency: asm/octeon/cvmx-gmxx-defs.h */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum cvmx_spi_mode {
    CVMX_SPI_MODE_UNKNOWN = 0,
    CVMX_SPI_MODE_TX_HALFPLEX = 1,
    CVMX_SPI_MODE_RX_HALFPLEX = 2,
    CVMX_SPI_MODE_DUPLEX = 3,
}

/** Callbacks structure to customize SPI4 initialization sequence */
#[repr(C)]
pub struct cvmx_spi_callbacks_t {
    /** Called to reset SPI4 DLL */
    pub reset_cb: Option<unsafe extern "C" fn(interface: i32, mode: cvmx_spi_mode) -> i32>,
    /** Called to setup calendar */
    pub calendar_setup_cb: Option<unsafe extern "C" fn(interface: i32, mode: cvmx_spi_mode, num_ports: i32) -> i32>,
    /** Called for Tx and Rx clock detection */
    pub clock_detect_cb: Option<unsafe extern "C" fn(interface: i32, mode: cvmx_spi_mode, timeout: i32) -> i32>,
    /** Called to perform link training */
    pub training_cb: Option<unsafe extern "C" fn(interface: i32, mode: cvmx_spi_mode, timeout: i32) -> i32>,
    /** Called for calendar data synchronization */
    pub calendar_sync_cb: Option<unsafe extern "C" fn(interface: i32, mode: cvmx_spi_mode, timeout: i32) -> i32>,
    /** Called when interface is up */
    pub interface_up_cb: Option<unsafe extern "C" fn(interface: i32, mode: cvmx_spi_mode) -> i32>,
}

extern "C" {
    pub fn cvmx_read_csr(address: u64) -> u64;
    pub fn cvmx_spi_start_interface(interface: i32, mode: cvmx_spi_mode, timeout: i32, num_ports: i32) -> i32;
    pub fn cvmx_spi_restart_interface(interface: i32, mode: cvmx_spi_mode, timeout: i32) -> i32;
    pub fn cvmx_spi_get_callbacks(callbacks: *mut cvmx_spi_callbacks_t);
    pub fn cvmx_spi_set_callbacks(new_callbacks: *mut cvmx_spi_callbacks_t);
    pub fn cvmx_spi_reset_cb(interface: i32, mode: cvmx_spi_mode) -> i32;
    pub fn cvmx_spi_calendar_setup_cb(interface: i32, mode: cvmx_spi_mode, num_ports: i32) -> i32;
    pub fn cvmx_spi_clock_detect_cb(interface: i32, mode: cvmx_spi_mode, timeout: i32) -> i32;
    pub fn cvmx_spi_training_cb(interface: i32, mode: cvmx_spi_mode, timeout: i32) -> i32;
    pub fn cvmx_spi_calendar_sync_cb(interface: i32, mode: cvmx_spi_mode, timeout: i32) -> i32;
    pub fn cvmx_spi_interface_up_cb(interface: i32, mode: cvmx_spi_mode) -> i32;
}

/* CVMX_GMXX_INF_MODE is supplied by the CSR definitions dependency. */
extern "C" {
    pub fn CVMX_GMXX_INF_MODE(interface: i32) -> u64;
}

pub unsafe fn cvmx_spi_is_spi_interface(interface: i32) -> i32 {
    let gmx_state: u64 = cvmx_read_csr(CVMX_GMXX_INF_MODE(interface));
    if (gmx_state & 0x2) != 0 && (gmx_state & 0x1) != 0 { 1 } else { 0 }
}

pub unsafe fn cvmx_spi4000_is_present(_interface: i32) -> i32 { 0 }

pub unsafe fn cvmx_spi4000_initialize(_interface: i32) -> i32 { 0 }

pub unsafe fn cvmx_spi4000_check_speed(
    _interface: i32,
    _port: i32,
) -> cvmx_gmxx_rxx_rx_inbnd {
    // The union type is supplied by cvmx-gmxx-defs.h; C initializes its u64 member to zero.
    core::mem::zeroed()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
