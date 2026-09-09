// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependency from dml_top_display_cfg_types.h is supplied externally.

// Delay and uncertainty structure
#[repr(C)]
pub struct delay_uncertainty_t {
    pub delay: i32,
    pub uncertainty: i32,
}

// Latency structure with group, pipeline, and pixel delays
#[repr(C)]
pub struct latency_t {
    pub groups: i32, // latency in groups - Number of groups needed to be sent before output can begin
    pub pipeline: i32, // pipeline delay latency - Propagation delay through the bitstream construction layer in number of pixel containers
    pub pixels: i32, // latency in pixels - Number of groups multiplied by cycles per group

    // Extra variables needed for functional coverage
    pub additional_group_delay: i32,
    pub lines_to_reach_ixd: i32,
    pub groups_to_reach_ixd: i32,
    pub slice_width_groups: i32,
    pub initial_xmit_delay: i32,
    pub number_of_lines_to_reach_ixd: i32,
    pub slice_width_modified: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
