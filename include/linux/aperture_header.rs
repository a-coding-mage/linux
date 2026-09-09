/* SPDX-License-Identifier: MIT */

// Dependency provided by the Linux types translation: resource_size_t.

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_APERTURE_HELPERS")]
extern "C" {
    pub fn devm_aperture_acquire_for_platform_device(
        pdev: *mut platform_device,
        base: resource_size_t,
        size: resource_size_t,
    ) -> ::core::ffi::c_int;

    pub fn aperture_remove_conflicting_devices(
        base: resource_size_t,
        size: resource_size_t,
        name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;

    pub fn __aperture_remove_legacy_vga_devices(
        pdev: *mut pci_dev,
    ) -> ::core::ffi::c_int;

    pub fn aperture_remove_conflicting_pci_devices(
        pdev: *mut pci_dev,
        name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_APERTURE_HELPERS"))]
pub unsafe fn devm_aperture_acquire_for_platform_device(
    _pdev: *mut platform_device,
    _base: resource_size_t,
    _size: resource_size_t,
) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_APERTURE_HELPERS"))]
pub unsafe fn aperture_remove_conflicting_devices(
    _base: resource_size_t,
    _size: resource_size_t,
    _name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_APERTURE_HELPERS"))]
pub unsafe fn __aperture_remove_legacy_vga_devices(
    _pdev: *mut pci_dev,
) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_APERTURE_HELPERS"))]
pub unsafe fn aperture_remove_conflicting_pci_devices(
    _pdev: *mut pci_dev,
    _name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    0
}

/**
 * aperture_remove_all_conflicting_devices - remove all existing framebuffers
 * @name: a descriptive name of the requesting driver
 *
 * This function removes all graphics device drivers. Use this function on systems
 * that can have their framebuffer located anywhere in memory.
 *
 * Returns:
 * 0 on success, or a negative errno code otherwise
 */
pub unsafe fn aperture_remove_all_conflicting_devices(
    name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    aperture_remove_conflicting_devices(0 as resource_size_t, resource_size_t::MAX, name)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
