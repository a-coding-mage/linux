/* SPDX-License-Identifier: GPL-2.0 */

// The original declaration is conditional on CONFIG_64BIT.
#[cfg(CONFIG_64BIT)]
pub const CAC_BASE: u64 = 0x9800_0000_0000_0000;

/* Skip 128k to trap NULL pointer dereferences */
pub const PCI_PORT_BASE: usize = 0xc000_0000_0000_0000usize + SZ_128K;
pub const PCI_IOBASE: *mut core::ffi::c_void = PCI_PORT_BASE as *mut core::ffi::c_void;
pub const PCI_IOSIZE: usize = SZ_16M;
pub const MAP_BASE: usize = PCI_PORT_BASE + PCI_IOSIZE;

pub const IO_SPACE_LIMIT: usize = PCI_IOSIZE - 1;

// The original header includes <asm/mach-generic/spaces.h>; its supplied
// declarations are expected to be available to this translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
