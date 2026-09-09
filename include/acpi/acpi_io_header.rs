/* SPDX-License-Identifier: GPL-2.0 */

// Translated from acpi_io.h.
//
// The Linux <linux/io.h> and <asm/acpi.h> dependencies are supplied by the
// surrounding translation unit.

// #ifndef acpi_os_ioremap
#[inline]
pub unsafe fn acpi_os_ioremap(
    phys: acpi_physical_address,
    size: acpi_size,
) -> *mut core::ffi::c_void {
    // __iomem
    ioremap_cache(phys, size)
}
// #endif

extern "C" {
    pub static mut acpi_permanent_mmap: bool;

    // __iomem __ref
    pub fn acpi_os_map_iomem(
        phys: acpi_physical_address,
        size: acpi_size,
    ) -> *mut core::ffi::c_void;

    // __ref
    pub fn acpi_os_unmap_iomem(virt: *mut core::ffi::c_void, size: acpi_size);

    // __iomem
    pub fn acpi_os_get_iomem(
        phys: acpi_physical_address,
        size: core::ffi::c_uint,
    ) -> *mut core::ffi::c_void;

    // __iomem
    pub fn acpi_os_map_generic_address(
        addr: *mut acpi_generic_address,
    ) -> *mut core::ffi::c_void;

    pub fn acpi_os_unmap_generic_address(addr: *mut acpi_generic_address);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
