/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Primary to Sideband (P2SB) bridge access support
 */

use core::ffi::{c_int, c_uint};

/* Opaque declarations corresponding to the C forward declarations. */
#[repr(C)]
pub struct pci_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

/* CONFIG_P2SB build-time condition from <linux/kconfig.h>. */
#[cfg(feature = "CONFIG_P2SB")]
extern "C" {
    pub fn p2sb_bar(bus: *mut pci_bus, devfn: c_uint, mem: *mut resource) -> c_int;
}

/* CONFIG_P2SB is not set. */
#[cfg(not(feature = "CONFIG_P2SB"))]
#[inline]
pub unsafe fn p2sb_bar(_bus: *mut pci_bus, _devfn: c_uint, _mem: *mut resource) -> c_int {
    -19 /* -ENODEV */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
