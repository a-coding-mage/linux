/* SPDX-License-Identifier: GPL-2.0 */

// `CONFIG_PCI` is represented as the Rust feature `CONFIG_PCI`.

extern "C" {
    pub fn pq2_restart(cmd: *mut core::ffi::c_char) -> !;
}

#[cfg(feature = "CONFIG_PCI")]
extern "C" {
    pub fn pq2ads_pci_init_irq() -> core::ffi::c_int;
    pub fn pq2_init_pci();
}

#[cfg(not(feature = "CONFIG_PCI"))]
#[inline]
pub fn pq2ads_pci_init_irq() -> core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_PCI"))]
#[inline]
pub fn pq2_init_pci() {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
