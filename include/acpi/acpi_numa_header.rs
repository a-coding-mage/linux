/* SPDX-License-Identifier: GPL-2.0 */

/* C dependency: <linux/numa.h> */

/* CONFIG_ACPI_NUMA */
#[cfg(feature = "CONFIG_ACPI_NUMA")]
pub const MAX_PXM_DOMAINS: usize = {
    /* MAX_NUMNODES > 256 selects MAX_NUMNODES; otherwise the legacy 8-bit PXM limit is used. */
    #[cfg(feature = "MAX_NUMNODES_GT_256")]
    {
        MAX_NUMNODES
    }
    #[cfg(not(feature = "MAX_NUMNODES_GT_256"))]
    {
        256
    }
};

#[cfg(feature = "CONFIG_ACPI_NUMA")]
unsafe extern "C" {
    pub fn pxm_to_node(pxm: core::ffi::c_int) -> core::ffi::c_int;
    pub fn node_to_pxm(node: core::ffi::c_int) -> core::ffi::c_int;
    pub fn acpi_map_pxm_to_node(pxm: core::ffi::c_int) -> core::ffi::c_int;
    pub static mut acpi_srat_revision: core::ffi::c_uchar;
    pub fn disable_srat();
    pub fn fix_pxm_node_maps(max_nid: core::ffi::c_int) -> core::ffi::c_int;
    pub fn bad_srat();
    pub fn srat_disabled() -> core::ffi::c_int;
}

/* CONFIG_ACPI_NUMA disabled */
#[cfg(not(feature = "CONFIG_ACPI_NUMA"))]
#[inline]
pub fn fix_pxm_node_maps(_max_nid: core::ffi::c_int) -> core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_ACPI_NUMA"))]
#[inline]
pub fn disable_srat() {}

#[cfg(not(feature = "CONFIG_ACPI_NUMA"))]
#[inline]
pub fn pxm_to_node(_pxm: core::ffi::c_int) -> core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_ACPI_NUMA"))]
#[inline]
pub fn node_to_pxm(_node: core::ffi::c_int) -> core::ffi::c_int {
    0
}

/* CONFIG_ACPI_HMAT */
#[cfg(feature = "CONFIG_ACPI_HMAT")]
unsafe extern "C" {
    pub fn disable_hmat();
}

/* CONFIG_ACPI_HMAT disabled */
#[cfg(not(feature = "CONFIG_ACPI_HMAT"))]
#[inline]
pub fn disable_hmat() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
