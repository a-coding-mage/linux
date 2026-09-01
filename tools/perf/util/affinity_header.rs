// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/affinity.h.

use core::ffi::{c_int, c_ulong};

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct affinity {
    pub orig_cpus: *mut c_ulong,
    pub sched_cpus: *mut c_ulong,
    pub changed: bool,
}

unsafe extern "C" {
    pub fn affinity__cleanup(a: *mut affinity);
    pub fn affinity__set(a: *mut affinity, cpu: c_int);
    pub fn affinity__setup(a: *mut affinity) -> c_int;
    pub fn cpu_map__set_affinity(cpumap: *const perf_cpu_map);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
