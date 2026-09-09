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
 ***********************license end**************************************/

// Helper functions for common, but complicated tasks.
// Dependencies supplied by the corresponding C headers:
// cvmx-config.h, cvmx-fpa.h, cvmx-wqe.h, and cvmx-helper-* headers.

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cvmx_helper_interface_mode {
    CVMX_HELPER_INTERFACE_MODE_DISABLED,
    CVMX_HELPER_INTERFACE_MODE_RGMII,
    CVMX_HELPER_INTERFACE_MODE_GMII,
    CVMX_HELPER_INTERFACE_MODE_SPI,
    CVMX_HELPER_INTERFACE_MODE_PCIE,
    CVMX_HELPER_INTERFACE_MODE_XAUI,
    CVMX_HELPER_INTERFACE_MODE_SGMII,
    CVMX_HELPER_INTERFACE_MODE_PICMG,
    CVMX_HELPER_INTERFACE_MODE_NPI,
    CVMX_HELPER_INTERFACE_MODE_LOOP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_helper_link_info_s {
    pub reserved_20_63: u64,
    pub link_up: u64,
    pub full_duplex: u64,
    pub speed: u64,
}

#[repr(C)]
pub union cvmx_helper_link_info {
    pub u64_: u64,
    pub s: cvmx_helper_link_info_s,
}

extern "C" {
    pub fn cvmx_helper_ipd_and_packet_input_enable() -> ::core::ffi::c_int;
    pub fn cvmx_helper_initialize_packet_io_global() -> ::core::ffi::c_int;
    pub fn cvmx_helper_ports_on_interface(interface_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn cvmx_helper_get_number_of_interfaces() -> ::core::ffi::c_int;
    pub fn cvmx_helper_interface_get_mode(
        interface_: ::core::ffi::c_int,
    ) -> cvmx_helper_interface_mode;
    pub fn cvmx_helper_link_get(ipd_port: ::core::ffi::c_int) -> cvmx_helper_link_info;
    pub fn cvmx_helper_link_set(
        ipd_port: ::core::ffi::c_int,
        link_info: cvmx_helper_link_info,
    ) -> ::core::ffi::c_int;
    pub fn cvmx_helper_interface_probe(interface_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn cvmx_helper_interface_enumerate(interface_: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
