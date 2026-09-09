/* SPDX-License-Identifier: MIT */
/*
 * Copyright © 2021 Intel Corporation
 */

// Dependency provided by linux/gpu_buddy.h.
#[repr(C)]
pub struct gpu_buddy;

#[repr(C)]
pub struct gpu_buddy_block;

#[repr(C)]
pub struct drm_printer;

/* DRM-specific GPU Buddy Allocator print helpers */
extern "C" {
    pub fn drm_buddy_print(mm: *mut gpu_buddy, p: *mut drm_printer);
    pub fn drm_buddy_block_print(
        mm: *mut gpu_buddy,
        block: *mut gpu_buddy_block,
        p: *mut drm_printer,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
