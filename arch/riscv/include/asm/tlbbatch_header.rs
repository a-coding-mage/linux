/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2023 Rivos Inc.
 */

// Dependency: `struct cpumask` is supplied by the Linux cpumask definitions.

#[repr(C)]
pub struct arch_tlbflush_unmap_batch {
    pub cpumask: cpumask,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
