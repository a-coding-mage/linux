// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Dependency intent: declarations from "os_types.h" are supplied externally.

/**
 * enum dc_probe_type - What DM wants to probe.
 *
 * Each value names a measurable quantity as an abstraction. DC resolves it to
 * whatever HW measurement block fulfills it. DM never selects the HW block.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dc_probe_type {
    DC_PROBE_PEAK_MEM_BW = 0,
    DC_PROBE_AVG_MEM_BW,
    DC_PROBE_MEM_LATENCY,
    DC_PROBE_URGENT_RAMP_LATENCY,
    DC_PROBE_URGENT_ASSERTION_COUNT,
    DC_PROBE_PREFETCH_DATA_SIZE,
}

/**
 * enum dc_probe_target_state - Target lifecycle state DM wants DC to reach.
 *
 * DM sets this to describe the final state DC must reach by the end of the
 * commit. DC performs whatever HW transition sequence is needed.
 *
 * @DC_PROBE_NOT_MEASURING: probe inactive, no valid data available.
 * @DC_PROBE_MEASURING:     probe runs continuously. The latest value can be
 *   read back at any time and may differ on each read.
 * @DC_PROBE_MEASURED:      probe performed one shot. The result is latched and
 *   stays valid until DM transitions back to DC_PROBE_NOT_MEASURING.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dc_probe_target_state {
    DC_PROBE_NOT_MEASURING = 0,
    DC_PROBE_MEASURING,
    DC_PROBE_MEASURED,
}

/**
 * enum dc_probe_scope_type - What the probe is scoped to.
 * @DC_PROBE_SCOPE_GLOBAL: whole memory subsystem, no stream/plane selector.
 *
 * Only GLOBAL is implemented. Per-stream/plane scoping must select targets by
 * stable id, not object pointer — dc_state copy semantics would dangle a raw
 * pointer when the absolute-set commit removes or replaces the target.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dc_probe_scope_type {
    DC_PROBE_SCOPE_GLOBAL = 0,
}

/**
 * struct dc_probe_scope - Selects what a probe measures against.
 * @type: scope kind, only DC_PROBE_SCOPE_GLOBAL is implemented.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct dc_probe_scope {
    pub type_: dc_probe_scope_type,
}

/**
 * struct dc_probe_state - DM-authored descriptor of a single probe.
 *
 * A plain inline value with copy semantics: no allocation, no refcount. DC
 * resolves each descriptor to a HW measurement instance and diffs the desired
 * set against the committed set to plan the transition.
 *
 * @type:         what to measure.
 * @target_state: desired lifecycle state for this probe.
 * @scope:        what the probe is scoped to (GLOBAL only for now).
 */
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct dc_probe_state {
    pub type_: dc_probe_type,
    pub target_state: dc_probe_target_state,
    pub scope: dc_probe_scope,
}

pub const MAX_PROBES: usize = 1;

/**
 * struct dc_probe_updates - Absolute set of probes DM wants active.
 *
 * Mirrors the plane/stream absolute-set model: the array is the complete
 * desired set. DC compares it against the committed set to add, remove, or
 * transition probes.
 *
 * @probes:      desired probe descriptors.
 * @probe_count: number of valid entries in @probes.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct dc_probe_updates {
    pub probes: [dc_probe_state; MAX_PROBES],
    pub probe_count: core::ffi::c_int,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
