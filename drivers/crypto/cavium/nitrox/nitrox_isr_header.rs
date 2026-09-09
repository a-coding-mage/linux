/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by nitrox_dev.h in the C source.
#[repr(C)]
pub struct nitrox_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn nitrox_register_interrupts(ndev: *mut nitrox_device) -> core::ffi::c_int;
    pub fn nitrox_unregister_interrupts(ndev: *mut nitrox_device);
    pub fn nitrox_sriov_register_interupts(ndev: *mut nitrox_device) -> core::ffi::c_int;
    pub fn nitrox_sriov_unregister_interrupts(ndev: *mut nitrox_device);
}

// CONFIG_PCI_IOV conditional from the C source.
#[cfg(feature = "CONFIG_PCI_IOV")]
unsafe extern "C" {
    pub fn nitrox_sriov_configure(
        pdev: *mut pci_dev,
        num_vfs: core::ffi::c_int,
    ) -> core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_PCI_IOV"))]
#[inline]
pub unsafe fn nitrox_sriov_configure(
    _pdev: *mut pci_dev,
    _num_vfs: core::ffi::c_int,
) -> core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
