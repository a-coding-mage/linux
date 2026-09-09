/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the translated interval-tree declarations.

#[repr(C)]
pub struct pfn_address_space {
    pub node: interval_tree_node,
    pub mapping: *mut address_space,
    pub pfn_to_vma_pgoff: Option<
        unsafe extern "C" fn(
            vma: *mut vm_area_struct,
            pfn: c_ulong,
            pgoff: *mut pgoff_t,
        ) -> c_int,
    >,
}

#[cfg(CONFIG_MEMORY_FAILURE)]
extern "C" {
    pub fn register_pfn_address_space(pfn_space: *mut pfn_address_space) -> c_int;
    pub fn unregister_pfn_address_space(pfn_space: *mut pfn_address_space);
}

#[cfg(not(CONFIG_MEMORY_FAILURE))]
pub unsafe fn register_pfn_address_space(_pfn_space: *mut pfn_address_space) -> c_int {
    -EOPNOTSUPP
}

#[cfg(not(CONFIG_MEMORY_FAILURE))]
pub unsafe fn unregister_pfn_address_space(_pfn_space: *mut pfn_address_space) {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
