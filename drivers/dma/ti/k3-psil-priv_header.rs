/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Copyright (C) 2019 Texas Instruments Incorporated - http://www.ti.com
 */

// Dependency supplied by linux/dma/k3-psil.h.
pub struct psil_endpoint_config;

#[repr(C)]
pub struct psil_ep {
    pub thread_id: u32,
    pub ep_config: psil_endpoint_config,
}

/**
 * struct psil_ep_map - PSI-L thread ID configuration maps
 * @name:\tName of the map, set it to the name of the SoC
 * @src:\tArray of source PSI-L thread configurations
 * @src_count:\tNumber of entries in the src array
 * @dst:\tArray of destination PSI-L thread configurations
 * @dst_count:\tNumber of entries in the dst array
 *
 * In case of symmetric configuration for a matching src/dst thread (for example
 * 0x4400 and 0xc400) only the src configuration can be present. If no dst
 * configuration found the code will look for (dst_thread_id & ~0x8000) to find
 * the symmetric match.
 */
#[repr(C)]
pub struct psil_ep_map {
    pub name: *mut core::ffi::c_char,
    pub src: *mut psil_ep,
    pub src_count: core::ffi::c_int,
    pub dst: *mut psil_ep,
    pub dst_count: core::ffi::c_int,
}

pub unsafe extern "C" fn psil_get_ep_config(thread_id: u32) -> *mut psil_endpoint_config;

/* SoC PSI-L endpoint maps */
unsafe extern "C" {
    pub static mut am654_ep_map: psil_ep_map;
    pub static mut j721e_ep_map: psil_ep_map;
    pub static mut j7200_ep_map: psil_ep_map;
    pub static mut am64_ep_map: psil_ep_map;
    pub static mut j721s2_ep_map: psil_ep_map;
    pub static mut am62_ep_map: psil_ep_map;
    pub static mut am62a_ep_map: psil_ep_map;
    pub static mut j784s4_ep_map: psil_ep_map;
    pub static mut am62p_ep_map: psil_ep_map;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
