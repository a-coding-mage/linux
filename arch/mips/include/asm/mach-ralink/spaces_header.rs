/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Translated from asm/mach-ralink/spaces.h.
 * The symbols mips_io_port_base and SZ_64K are supplied by dependencies.
 */

macro_rules! PCI_IOBASE {
    () => {
        (mips_io_port_base as *mut core::ffi::c_void)
    };
}

macro_rules! PCI_IOSIZE {
    () => {
        SZ_64K
    };
}

macro_rules! IO_SPACE_LIMIT {
    () => {
        (PCI_IOSIZE!() - 1)
    };
}

/*
 * #ifdef CONFIG_PCI_DRIVERS_GENERIC
 * #define pci_remap_iospace pci_remap_iospace
 *
 * The self-aliasing C preprocessor macro preserves the existing symbol name
 * and has no distinct Rust item to introduce.
 */

/* Dependency equivalent of: #include <asm/mach-generic/spaces.h> */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
