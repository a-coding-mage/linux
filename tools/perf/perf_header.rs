/* SPDX-License-Identifier: GPL-2.0 */

pub const MAX_NR_CPUS: u32 = 4096;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum perf_affinity {
    PERF_AFFINITY_SYS = 0,
    PERF_AFFINITY_NODE,
    PERF_AFFINITY_CPU,
    PERF_AFFINITY_MAX,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
