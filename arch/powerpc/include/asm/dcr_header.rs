/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * (c) Copyright 2006 Benjamin Herrenschmidt, IBM Corp.
 *                    <benh@kernel.crashing.org>
 */

// The declarations in this header are available only when building the
// kernel, for non-assembler code, with CONFIG_PPC_DCR enabled.

// Dependency supplied by asm/dcr-native.h.
pub type dcr_host_t = dcr_host_native_t;

#[macro_export]
macro_rules! DCR_MAP_OK {
    ($host:expr) => {
        dcr_map_ok_native($host)
    };
}

#[macro_export]
macro_rules! dcr_map {
    ($dev:expr, $dcr_n:expr, $dcr_c:expr) => {
        dcr_map_native($dev, $dcr_n, $dcr_c)
    };
}

#[macro_export]
macro_rules! dcr_unmap {
    ($host:expr, $dcr_c:expr) => {
        dcr_unmap_native($host, $dcr_c)
    };
}

#[macro_export]
macro_rules! dcr_read {
    ($host:expr, $dcr_n:expr) => {
        dcr_read_native($host, $dcr_n)
    };
}

#[macro_export]
macro_rules! dcr_write {
    ($host:expr, $dcr_n:expr, $value:expr) => {
        dcr_write_native($host, $dcr_n, $value)
    };
}

/*
 * additional helpers to read the DCR * base from the device-tree
 */
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn dcr_resource_start(np: *const device_node, index: ::core::ffi::c_uint)
        -> ::core::ffi::c_uint;
    pub fn dcr_resource_len(np: *const device_node, index: ::core::ffi::c_uint)
        -> ::core::ffi::c_uint;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
