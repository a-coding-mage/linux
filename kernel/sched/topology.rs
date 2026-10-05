// SPDX-License-Identifier: GPL-2.0
// Source continuation: baseline 126a30fae3bba11420ec2fcbde51a0a01bab1b5b.
// Native headers own every ABI layout, constant and storage representation.
//! Scheduler-domain topology, root-domain lifetime, CPU-capacity groups,
//! cache/energy scheduling and NUMA domain construction.
//!
//! Source-only continuation against the pinned native scheduler implementation;
//! native headers remain authoritative and admission is deliberately blocked.
#![no_std]
compile_error!("SOURCE ONLY HOLD: scheduler topology is not admitted");
use kernel::bindings::sched_topology_native::*;
use kernel::ffi::{c_char, c_int, c_uint, c_ulong};
#[cfg(all(CONFIG_ENERGY_MODEL, CONFIG_CPU_FREQ_GOV_SCHEDUTIL, CONFIG_SYSCTL))]
use kernel::ffi::c_void;
use core::ptr::null_mut;
// Native sched_domain.flags is int; use native enum values with that type.
const SD_BALANCE_NEWIDLE: c_int = kernel::bindings::sched_topology_native::SD_BALANCE_NEWIDLE as c_int;
const SD_BALANCE_EXEC: c_int = kernel::bindings::sched_topology_native::SD_BALANCE_EXEC as c_int;
const SD_BALANCE_FORK: c_int = kernel::bindings::sched_topology_native::SD_BALANCE_FORK as c_int;
const SD_BALANCE_WAKE: c_int = kernel::bindings::sched_topology_native::SD_BALANCE_WAKE as c_int;
const SD_WAKE_AFFINE: c_int = kernel::bindings::sched_topology_native::SD_WAKE_AFFINE as c_int;
const SD_ASYM_CPUCAPACITY: c_int = kernel::bindings::sched_topology_native::SD_ASYM_CPUCAPACITY as c_int;
const SD_ASYM_CPUCAPACITY_FULL: c_int = kernel::bindings::sched_topology_native::SD_ASYM_CPUCAPACITY_FULL as c_int;
const SD_SHARE_CPUCAPACITY: c_int = kernel::bindings::sched_topology_native::SD_SHARE_CPUCAPACITY as c_int;
const SD_CLUSTER: c_int = kernel::bindings::sched_topology_native::SD_CLUSTER as c_int;
const SD_SHARE_LLC: c_int = kernel::bindings::sched_topology_native::SD_SHARE_LLC as c_int;
const SD_SERIALIZE: c_int = kernel::bindings::sched_topology_native::SD_SERIALIZE as c_int;
const SD_ASYM_PACKING: c_int = kernel::bindings::sched_topology_native::SD_ASYM_PACKING as c_int;
const SD_PREFER_SIBLING: c_int = kernel::bindings::sched_topology_native::SD_PREFER_SIBLING as c_int;
const SD_NUMA: c_int = kernel::bindings::sched_topology_native::SD_NUMA as c_int;
include!("sched_topology_debug.rs");

include!("sched_topology_energy.rs");

include!("sched_topology_root.rs");

include!("sched_topology_cache.rs");

include!("sched_topology_groups.rs");

include!("sched_topology_init.rs");

include!("sched_topology_numa.rs");
include!("sched_topology_build.rs");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
