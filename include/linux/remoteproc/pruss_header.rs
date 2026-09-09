/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * PRU-ICSS Subsystem user interfaces
 *
 * Copyright (C) 2015-2022 Texas Instruments Incorporated - http://www.ti.com
 *	Suman Anna <s-anna@ti.com>
 */

// C dependencies: linux/device.h and linux/types.h.

pub const PRU_RPROC_DRVNAME: &str = "pru-rproc";

/**
 * enum pruss_pru_id - PRU core identifiers
 * @PRUSS_PRU0: PRU Core 0.
 * @PRUSS_PRU1: PRU Core 1.
 * @PRUSS_NUM_PRUS: Total number of PRU Cores available.
 */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pruss_pru_id {
    PRUSS_PRU0 = 0,
    PRUSS_PRU1,
    PRUSS_NUM_PRUS,
}

/* enum pru_ctable_idx - Configurable Constant table index identifiers */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pru_ctable_idx {
    PRU_C24 = 0,
    PRU_C25,
    PRU_C26,
    PRU_C27,
    PRU_C28,
    PRU_C29,
    PRU_C30,
    PRU_C31,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rproc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

// When CONFIG_PRU_REMOTEPROC is enabled, these functions are supplied externally.
extern "C" {
    pub fn pru_rproc_get(
        np: *mut device_node,
        index: ::core::ffi::c_int,
        pru_id: *mut pruss_pru_id,
    ) -> *mut rproc;
    pub fn pru_rproc_put(rproc: *mut rproc);
    pub fn pru_rproc_set_ctable(
        rproc: *mut rproc,
        c: pru_ctable_idx,
        addr: u32,
    ) -> ::core::ffi::c_int;
}

// If CONFIG_PRU_REMOTEPROC is disabled, the C header provides inline stubs
// returning -EOPNOTSUPP (and ERR_PTR(-EOPNOTSUPP) for pru_rproc_get).

extern "C" {
    pub fn dev_driver_string(dev: *const device) -> *const ::core::ffi::c_char;
    pub fn strncmp(
        s1: *const ::core::ffi::c_char,
        s2: *const ::core::ffi::c_char,
        n: usize,
    ) -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn is_pru_rproc(dev: *mut device) -> bool {
    let drv_name = dev_driver_string(dev as *const device);

    if strncmp(
        drv_name,
        PRU_RPROC_DRVNAME.as_ptr() as *const ::core::ffi::c_char,
        PRU_RPROC_DRVNAME.len() + 1,
    ) != 0
    {
        return false;
    }

    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
