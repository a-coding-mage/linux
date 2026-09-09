/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * AMD Secure Processor device driver, security attributes
 *
 * Copyright (C) 2023-2024 Advanced Micro Devices, Inc.
 *
 * Author: Mario Limonciello <mario.limonciello@amd.com>
 */

// C header guard: __HSTI_H

extern "C" {
    pub static mut psp_security_attr_group: attribute_group;

    pub fn psp_init_hsti(psp: *mut psp_device) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
