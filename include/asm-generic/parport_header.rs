/* SPDX-License-Identifier: GPL-2.0 */

/*
 * An ISA bus may have i8255 parallel ports at well-known
 * locations in the I/O space, which are scanned by
 * parport_pc_find_isa_ports.
 *
 * Without ISA support, the driver will only attach
 * to devices on the PCI bus.
 */

extern "C" {
    fn parport_pc_find_isa_ports(autoirq: core::ffi::c_int, autodma: core::ffi::c_int)
        -> core::ffi::c_int;
}

unsafe fn parport_pc_find_nonpci_ports(
    autoirq: core::ffi::c_int,
    autodma: core::ffi::c_int,
) -> core::ffi::c_int {
    #[cfg(CONFIG_ISA)]
    {
        parport_pc_find_isa_ports(autoirq, autodma)
    }
    #[cfg(not(CONFIG_ISA))]
    {
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
