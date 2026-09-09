/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * AMD Geode definitions
 * Copyright (C) 2006, Advanced Micro Devices, Inc.
 */

// Dependencies supplied by the translated processor and platform headers:
// `boot_cpu_data`, `X86_VENDOR_NSC`, and `X86_VENDOR_AMD`.

pub unsafe fn is_geode_gx() -> i32 {
    ((boot_cpu_data.x86_vendor == X86_VENDOR_NSC)
        && (boot_cpu_data.x86 == 5)
        && (boot_cpu_data.x86_model == 5)) as i32
}

pub unsafe fn is_geode_lx() -> i32 {
    ((boot_cpu_data.x86_vendor == X86_VENDOR_AMD)
        && (boot_cpu_data.x86 == 5)
        && (boot_cpu_data.x86_model == 10)) as i32
}

pub unsafe fn is_geode() -> i32 {
    ((is_geode_gx() != 0) || (is_geode_lx() != 0)) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
