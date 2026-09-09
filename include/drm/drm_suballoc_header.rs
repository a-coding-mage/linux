/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/*
 * Copyright 2011 Red Hat Inc.
 * Copyright © 2022 Intel Corporation
 */

// Dependency declarations from <drm/drm_mm.h>, <linux/dma-fence.h>, and
// <linux/types.h> are supplied by the surrounding translation unit.

pub const DRM_SUBALLOC_MAX_QUEUES: usize = 32;

/**
 * struct drm_suballoc_manager - fenced range allocations
 * @wq: Wait queue for sleeping allocations on contention.
 * @hole: Pointer to first hole node.
 * @olist: List of allocated ranges.
 * @flist: Array[fence context hash] of queues of fenced allocated ranges.
 * @size: Size of the managed range.
 * @align: Default alignment for the managed range.
 */
#[repr(C)]
pub struct drm_suballoc_manager {
    pub wq: wait_queue_head_t,
    pub hole: *mut list_head,
    pub olist: list_head,
    pub flist: [list_head; DRM_SUBALLOC_MAX_QUEUES],
    pub size: usize,
    pub align: usize,
}

/**
 * struct drm_suballoc - Sub-allocated range
 * @olist: List link for list of allocated ranges.
 * @flist: List linkk for the manager fenced allocated ranges queues.
 * @manager: The drm_suballoc_manager.
 * @soffset: Start offset.
 * @eoffset: End offset + 1 so that @eoffset - @soffset = size.
 * @fence: The fence protecting the allocation.
 */
#[repr(C)]
pub struct drm_suballoc {
    pub olist: list_head,
    pub flist: list_head,
    pub manager: *mut drm_suballoc_manager,
    pub soffset: usize,
    pub eoffset: usize,
    pub fence: *mut dma_fence,
}

extern "C" {
    pub fn drm_suballoc_manager_init(
        sa_manager: *mut drm_suballoc_manager,
        size: usize,
        align: usize,
    );

    pub fn drm_suballoc_manager_fini(sa_manager: *mut drm_suballoc_manager);

    pub fn drm_suballoc_alloc(gfp: gfp_t) -> *mut drm_suballoc;

    pub fn drm_suballoc_insert(
        sa_manager: *mut drm_suballoc_manager,
        sa: *mut drm_suballoc,
        size: usize,
        intr: bool,
        align: usize,
    ) -> i32;

    pub fn drm_suballoc_new(
        sa_manager: *mut drm_suballoc_manager,
        size: usize,
        gfp: gfp_t,
        intr: bool,
        align: usize,
    ) -> *mut drm_suballoc;

    pub fn drm_suballoc_free(sa: *mut drm_suballoc, fence: *mut dma_fence);
}

/**
 * drm_suballoc_soffset - Range start.
 * @sa: The struct drm_suballoc.
 *
 * Return: The start of the allocated range.
 */
#[inline]
pub unsafe fn drm_suballoc_soffset(sa: *mut drm_suballoc) -> usize {
    (*sa).soffset
}

/**
 * drm_suballoc_eoffset - Range end.
 * @sa: The struct drm_suballoc.
 *
 * Return: The end of the allocated range + 1.
 */
#[inline]
pub unsafe fn drm_suballoc_eoffset(sa: *mut drm_suballoc) -> usize {
    (*sa).eoffset
}

/**
 * drm_suballoc_size - Range size.
 * @sa: The struct drm_suballoc.
 *
 * Return: The size of the allocated range.
 */
#[inline]
pub unsafe fn drm_suballoc_size(sa: *mut drm_suballoc) -> usize {
    (*sa).eoffset - (*sa).soffset
}

#[cfg(feature = "CONFIG_DEBUG_FS")]
extern "C" {
    pub fn drm_suballoc_dump_debug_info(
        sa_manager: *mut drm_suballoc_manager,
        p: *mut drm_printer,
        suballoc_base: u64,
    );
}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub unsafe fn drm_suballoc_dump_debug_info(
    _sa_manager: *mut drm_suballoc_manager,
    _p: *mut drm_printer,
    _suballoc_base: u64,
) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
