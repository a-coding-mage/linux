/* SPDX-License-Identifier: GPL-2.0 */

// Translated from linux/of_pci.h.  The original declarations are conditional
// on CONFIG_OF and CONFIG_PCI; the `of_pci` feature represents that condition.

use core::ffi::c_int;

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[cfg(feature = "of_pci")]
extern "C" {
    pub fn of_pci_find_child_device(
        parent: *mut device_node,
        devfn: u32,
    ) -> *mut device_node;
    pub fn of_pci_get_devfn(np: *mut device_node) -> c_int;
    pub fn of_pci_check_probe_only();
}

#[cfg(not(feature = "of_pci"))]
#[inline]
pub unsafe fn of_pci_find_child_device(
    _parent: *mut device_node,
    _devfn: u32,
) -> *mut device_node {
    core::ptr::null_mut()
}

#[cfg(not(feature = "of_pci"))]
#[inline]
pub unsafe fn of_pci_get_devfn(_np: *mut device_node) -> c_int {
    // -EINVAL; EINVAL is supplied by the translated errno dependency.
    -crate::EINVAL
}

#[cfg(not(feature = "of_pci"))]
#[inline]
pub unsafe fn of_pci_check_probe_only() {}

// The original declarations are conditional on CONFIG_OF_IRQ; the
// `of_irq` feature represents that condition.
#[cfg(feature = "of_irq")]
extern "C" {
    pub fn of_irq_parse_and_map_pci(
        dev: *const pci_dev,
        slot: u8,
        pin: u8,
    ) -> c_int;
    pub fn pci_configure_of_wake_gpio(dev: *mut pci_dev);
    pub fn pci_remove_of_wake_gpio(dev: *mut pci_dev);
}

#[cfg(not(feature = "of_irq"))]
#[inline]
pub unsafe fn of_irq_parse_and_map_pci(
    _dev: *const pci_dev,
    _slot: u8,
    _pin: u8,
) -> c_int {
    0
}

#[cfg(not(feature = "of_irq"))]
#[inline]
pub unsafe fn pci_configure_of_wake_gpio(_dev: *mut pci_dev) {}

#[cfg(not(feature = "of_irq"))]
#[inline]
pub unsafe fn pci_remove_of_wake_gpio(_dev: *mut pci_dev) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
