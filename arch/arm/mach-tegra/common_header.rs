/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2011, ARM Ltd.
 * Copyright (c) 2013, NVIDIA Corporation. All rights reserved.
 */

// C header guard: __MACH_TEGRA_COMMON_H

extern "C" {
    pub static tegra_smp_ops: crate::smp_operations;

    pub fn tegra_cpu_kill(cpu: u32) -> i32;
    pub fn tegra_cpu_die(cpu: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
