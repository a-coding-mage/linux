/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: linux/hugetlb.h supplies `folio`, `gfp_t`, `nodemask_t`, and
// `hstate`. The CONFIG_CMA condition is represented by the CONFIG_CMA feature.

#[cfg(feature = "CONFIG_CMA")]
extern "C" {
    pub fn hugetlb_cma_free_frozen_folio(folio: *mut folio);
    pub fn hugetlb_cma_alloc_frozen_folio(
        order: core::ffi::c_int,
        gfp_mask: gfp_t,
        nid: core::ffi::c_int,
        nodemask: *mut nodemask_t,
    ) -> *mut folio;
    pub fn hugetlb_cma_alloc_bootmem(
        h: *mut hstate,
        nid: core::ffi::c_int,
        node_exact: bool,
    ) -> *mut core::ffi::c_void;
    pub fn hugetlb_cma_exclusive_alloc() -> bool;
    pub fn hugetlb_cma_total_size() -> core::ffi::c_ulong;
    pub fn hugetlb_cma_validate_params();
    pub fn hugetlb_early_cma(h: *mut hstate) -> bool;
}

#[cfg(not(feature = "CONFIG_CMA"))]
#[inline]
pub unsafe fn hugetlb_cma_free_frozen_folio(_folio: *mut folio) {}

#[cfg(not(feature = "CONFIG_CMA"))]
#[inline]
pub unsafe fn hugetlb_cma_alloc_frozen_folio(
    _order: core::ffi::c_int,
    _gfp_mask: gfp_t,
    _nid: core::ffi::c_int,
    _nodemask: *mut nodemask_t,
) -> *mut folio {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_CMA"))]
#[inline]
pub unsafe fn hugetlb_cma_alloc_bootmem(
    _h: *mut hstate,
    _nid: core::ffi::c_int,
    _node_exact: bool,
) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_CMA"))]
#[inline]
pub unsafe fn hugetlb_cma_exclusive_alloc() -> bool {
    false
}

#[cfg(not(feature = "CONFIG_CMA"))]
#[inline]
pub unsafe fn hugetlb_cma_total_size() -> core::ffi::c_ulong {
    0
}

#[cfg(not(feature = "CONFIG_CMA"))]
#[inline]
pub unsafe fn hugetlb_cma_validate_params() {}

#[cfg(not(feature = "CONFIG_CMA"))]
#[inline]
pub unsafe fn hugetlb_early_cma(_h: *mut hstate) -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
