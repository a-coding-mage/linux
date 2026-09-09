/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by linux/mm_types.h in the original header.
#[repr(C)]
pub struct vm_special_mapping;
#[repr(C)]
pub struct mm_struct;

extern "C" {
    pub static vdso_vvar_mapping: vm_special_mapping;

    pub fn vdso_install_vvar_mapping(
        mm: *mut mm_struct,
        addr: ::core::ffi::c_ulong,
    ) -> *mut vm_area_struct;
}

// Dependency supplied by linux/mm_types.h in the original header.
#[repr(C)]
pub struct vm_area_struct;

// CONFIG_VDSO_DATASTORE selects the external implementation.  The fallback
// is the original !CONFIG_VDSO_DATASTORE static inline definition.
#[cfg(feature = "CONFIG_VDSO_DATASTORE")]
extern "C" {
    // Original declaration carries the kernel __init annotation.
    pub fn vdso_setup_data_pages();
}

#[cfg(not(feature = "CONFIG_VDSO_DATASTORE"))]
#[inline]
pub fn vdso_setup_data_pages() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
