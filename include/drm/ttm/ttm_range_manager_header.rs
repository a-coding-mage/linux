/* SPDX-License-Identifier: GPL-2.0 OR MIT */

// Translated from ttm_range_manager.h.
//
// Required definitions are supplied by the corresponding TTM/DRM bindings.

/// struct ttm_range_mgr_node
///
/// @base: base clase we extend
/// @mm_nodes: MM nodes, usually 1
///
/// Extending the ttm_resource object to manage an address space allocation with
/// one or more drm_mm_nodes.
#[repr(C)]
pub struct ttm_range_mgr_node {
    pub base: ttm_resource,
    pub mm_nodes: [drm_mm_node; 0],
}

/// to_ttm_range_mgr_node
///
/// @res: the resource to upcast
///
/// Upcast the ttm_resource object into a ttm_range_mgr_node object.
#[inline]
pub unsafe fn to_ttm_range_mgr_node(res: *mut ttm_resource) -> *mut ttm_range_mgr_node {
    // Equivalent to container_of(res, struct ttm_range_mgr_node, base).
    (res as *mut u8).sub(core::mem::offset_of!(ttm_range_mgr_node, base))
        as *mut ttm_range_mgr_node
}

extern "C" {
    pub fn ttm_range_man_init_nocheck(
        bdev: *mut ttm_device,
        type_: core::ffi::c_uint,
        use_tt: bool,
        p_size: core::ffi::c_ulong,
    ) -> core::ffi::c_int;

    pub fn ttm_range_man_fini_nocheck(
        bdev: *mut ttm_device,
        type_: core::ffi::c_uint,
    ) -> core::ffi::c_int;
}

#[inline(always)]
pub unsafe fn ttm_range_man_init(
    bdev: *mut ttm_device,
    type_: core::ffi::c_uint,
    use_tt: bool,
    p_size: core::ffi::c_ulong,
) -> core::ffi::c_int {
    // BUILD_BUG_ON(__builtin_constant_p(type) && type >= TTM_NUM_MEM_TYPES);
    ttm_range_man_init_nocheck(bdev, type_, use_tt, p_size)
}

#[inline(always)]
pub unsafe fn ttm_range_man_fini(
    bdev: *mut ttm_device,
    type_: core::ffi::c_uint,
) -> core::ffi::c_int {
    // BUILD_BUG_ON(__builtin_constant_p(type) && type >= TTM_NUM_MEM_TYPES);
    ttm_range_man_fini_nocheck(bdev, type_)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
