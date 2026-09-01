// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2021 Intel Corporation
 */

unsafe extern "C" {
    pub static sof_acpi_pm: dev_pm_ops;

    pub fn sof_acpi_probe(
        pdev: *mut platform_device,
        desc: *const sof_dev_desc,
    ) -> ::core::ffi::c_int;

    pub fn sof_acpi_remove(pdev: *mut platform_device);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
