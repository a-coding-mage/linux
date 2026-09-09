/* SPDX-License-Identifier: MIT */

//! Rust translation of drm_module.h.
//!
//! The Linux kernel, DRM, PCI, platform-device, and module-driver symbols
//! referenced here are supplied by other translated dependencies.

/**
 * DOC: overview
 *
 * This library provides helpers registering DRM drivers during module
 * initialization and shutdown. The provided helpers act like bus-specific
 * module helpers, such as module_pci_driver(), but respect additional
 * parameters that control DRM driver registration.
 *
 * Below is an example of initializing a DRM driver for a device on the
 * PCI bus.
 *
 * The generated code will test if DRM drivers are enabled and register
 * the PCI driver. For more complex module initialization, module_init()
 * and module_exit() can still be used in the driver.
 */

/* PCI drivers */

#[inline]
pub unsafe fn drm_pci_register_driver(pci_drv: *mut pci_driver) -> ::core::ffi::c_int {
    if drm_firmware_drivers_only() {
        return -ENODEV;
    }

    pci_register_driver(pci_drv)
}

/**
 * drm_module_pci_driver - Register a DRM driver for PCI-based devices
 * @__pci_drv: the PCI driver structure
 *
 * Registers a DRM driver for devices on the PCI bus. The helper behaves like
 * module_pci_driver() but tests the state of drm_firmware_drivers_only().
 * Each module may only use this macro once.
 */
#[macro_export]
macro_rules! drm_module_pci_driver {
    ($pci_drv:expr) => {
        module_driver!($pci_drv, drm_pci_register_driver, pci_unregister_driver);
    };
}

#[inline]
pub unsafe fn drm_pci_register_driver_if_modeset(
    pci_drv: *mut pci_driver,
    modeset: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if drm_firmware_drivers_only() && modeset == -1 {
        return -ENODEV;
    }
    if modeset == 0 {
        return -ENODEV;
    }

    pci_register_driver(pci_drv)
}

#[inline]
pub unsafe fn drm_pci_unregister_driver_if_modeset(
    pci_drv: *mut pci_driver,
    _modeset: ::core::ffi::c_int,
) {
    pci_unregister_driver(pci_drv);
}

/**
 * drm_module_pci_driver_if_modeset - Register a DRM driver for PCI-based devices
 * @__pci_drv: the PCI driver structure
 * @__modeset: an additional parameter that disables the driver
 *
 * This macro is deprecated and only provided for existing drivers. For new
 * drivers, use drm_module_pci_driver(). If __modeset is 0, the driver is
 * disabled; if it is -1, the driver state depends on the global DRM state.
 */
#[macro_export]
macro_rules! drm_module_pci_driver_if_modeset {
    ($pci_drv:expr, $modeset:expr) => {
        module_driver!(
            $pci_drv,
            drm_pci_register_driver_if_modeset,
            drm_pci_unregister_driver_if_modeset,
            $modeset
        );
    };
}

/* Platform drivers */

#[inline]
pub unsafe fn drm_platform_driver_register(
    platform_drv: *mut platform_driver,
) -> ::core::ffi::c_int {
    if drm_firmware_drivers_only() {
        return -ENODEV;
    }

    platform_driver_register(platform_drv)
}

/**
 * drm_module_platform_driver - Register a DRM driver for platform devices
 * @__platform_drv: the platform driver structure
 *
 * Registers a DRM driver for devices on the platform bus. The helper behaves
 * like module_platform_driver() but tests the state of
 * drm_firmware_drivers_only(). Each module may only use this macro once.
 */
#[macro_export]
macro_rules! drm_module_platform_driver {
    ($platform_drv:expr) => {
        module_driver!(
            $platform_drv,
            drm_platform_driver_register,
            platform_driver_unregister
        );
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
