/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: struct pci_dev is declared by the surrounding PCI headers.
#[allow(non_camel_case_types)]
pub struct pci_dev;

// CONFIG_XEN_DOM0 is a build-time configuration condition preserved from C.
#[cfg(CONFIG_XEN_DOM0)]
unsafe extern "C" {
    pub fn xen_reset_device(dev: *const pci_dev) -> ::core::ffi::c_int;
    pub fn xen_find_device_domain_owner(dev: *mut pci_dev) -> ::core::ffi::c_int;
    pub fn xen_register_device_domain_owner(
        dev: *mut pci_dev,
        domain: u16,
    ) -> ::core::ffi::c_int;
    pub fn xen_unregister_device_domain_owner(dev: *mut pci_dev) -> ::core::ffi::c_int;
}

#[cfg(not(CONFIG_XEN_DOM0))]
#[inline]
pub unsafe fn xen_reset_device(_dev: *const pci_dev) -> ::core::ffi::c_int {
    -1
}

#[cfg(not(CONFIG_XEN_DOM0))]
#[inline]
pub unsafe fn xen_find_device_domain_owner(_dev: *mut pci_dev) -> ::core::ffi::c_int {
    -1
}

#[cfg(not(CONFIG_XEN_DOM0))]
#[inline]
pub unsafe fn xen_register_device_domain_owner(
    _dev: *mut pci_dev,
    _domain: u16,
) -> ::core::ffi::c_int {
    -1
}

#[cfg(not(CONFIG_XEN_DOM0))]
#[inline]
pub unsafe fn xen_unregister_device_domain_owner(_dev: *mut pci_dev) -> ::core::ffi::c_int {
    -1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
