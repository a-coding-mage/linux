/* SPDX-License-Identifier: GPL-2.0 */

/* Build-time CONFIG_PCI_XEN condition translated as a Rust feature. */
#[cfg(feature = "CONFIG_PCI_XEN")]
extern "C" {
    pub fn pci_xen_init() -> ::core::ffi::c_int;
    pub fn pci_xen_hvm_init() -> ::core::ffi::c_int;
}

#[cfg(feature = "CONFIG_PCI_XEN")]
pub const pci_xen: ::core::ffi::c_int = 1;

#[cfg(not(feature = "CONFIG_PCI_XEN"))]
pub const pci_xen: ::core::ffi::c_int = 0;

#[cfg(not(feature = "CONFIG_PCI_XEN"))]
pub const pci_xen_init: ::core::ffi::c_int = 0;

#[cfg(not(feature = "CONFIG_PCI_XEN"))]
#[inline]
pub unsafe fn pci_xen_hvm_init() -> ::core::ffi::c_int {
    -1
}

/* Build-time CONFIG_XEN_PV_DOM0 condition translated as a Rust feature. */
#[cfg(feature = "CONFIG_XEN_PV_DOM0")]
extern "C" {
    pub fn pci_xen_initial_domain() -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_XEN_PV_DOM0"))]
#[inline]
pub unsafe fn pci_xen_initial_domain() -> ::core::ffi::c_int {
    -1
}

/* Opaque type supplied by the PCI subsystem. */
#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

/* Build-time CONFIG_PCI_MSI and CONFIG_PCI_XEN conditions translated as
 * Rust features. */
#[cfg(all(feature = "CONFIG_PCI_MSI", feature = "CONFIG_PCI_XEN"))]
#[repr(C)]
pub struct xen_pci_frontend_ops {
    pub enable_msi: Option<unsafe extern "C" fn(
        dev: *mut pci_dev,
        vectors: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int>,
    pub disable_msi: Option<unsafe extern "C" fn(dev: *mut pci_dev)>,
    pub enable_msix: Option<unsafe extern "C" fn(
        dev: *mut pci_dev,
        vectors: *mut ::core::ffi::c_int,
        nvec: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int>,
    pub disable_msix: Option<unsafe extern "C" fn(dev: *mut pci_dev)>,
}

#[cfg(all(feature = "CONFIG_PCI_MSI", feature = "CONFIG_PCI_XEN"))]
extern "C" {
    pub static mut xen_pci_frontend: *mut xen_pci_frontend_ops;
}

#[cfg(all(feature = "CONFIG_PCI_MSI", feature = "CONFIG_PCI_XEN"))]
#[inline]
pub unsafe fn xen_pci_frontend_enable_msi(
    dev: *mut pci_dev,
    vectors: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !xen_pci_frontend.is_null() {
        if let Some(enable_msi) = (*xen_pci_frontend).enable_msi {
            return enable_msi(dev, vectors);
        }
    }
    -ENOSYS
}

#[cfg(all(feature = "CONFIG_PCI_MSI", feature = "CONFIG_PCI_XEN"))]
#[inline]
pub unsafe fn xen_pci_frontend_disable_msi(dev: *mut pci_dev) {
    if !xen_pci_frontend.is_null() {
        if let Some(disable_msi) = (*xen_pci_frontend).disable_msi {
            disable_msi(dev);
        }
    }
}

#[cfg(all(feature = "CONFIG_PCI_MSI", feature = "CONFIG_PCI_XEN"))]
#[inline]
pub unsafe fn xen_pci_frontend_enable_msix(
    dev: *mut pci_dev,
    vectors: *mut ::core::ffi::c_int,
    nvec: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !xen_pci_frontend.is_null() {
        if let Some(enable_msix) = (*xen_pci_frontend).enable_msix {
            return enable_msix(dev, vectors, nvec);
        }
    }
    -ENOSYS
}

#[cfg(all(feature = "CONFIG_PCI_MSI", feature = "CONFIG_PCI_XEN"))]
#[inline]
pub unsafe fn xen_pci_frontend_disable_msix(dev: *mut pci_dev) {
    if !xen_pci_frontend.is_null() {
        if let Some(disable_msix) = (*xen_pci_frontend).disable_msix {
            disable_msix(dev);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
