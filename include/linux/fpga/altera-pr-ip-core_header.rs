/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Driver for Altera Partial Reconfiguration IP Core
 *
 * Copyright (C) 2016 Intel Corporation
 *
 * Based on socfpga-a10.c Copyright (C) 2015-2016 Altera Corporation
 *  by Alan Tull <atull@opensource.altera.com>
 */

// Dependency supplied externally by the Linux kernel headers:
// #include <linux/io.h>

// Opaque declaration corresponding to `struct device`.
pub enum device {}

// `__iomem` is a kernel address-space annotation; the raw pointer preserves
// the C interface and pointer behavior.
extern "C" {
    pub fn alt_pr_register(
        dev: *mut device,
        reg_base: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
