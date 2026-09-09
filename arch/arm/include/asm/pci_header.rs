/* SPDX-License-Identifier: GPL-2.0 */

/* The following declarations are available only when building the kernel. */
#[cfg(__KERNEL__)]
extern "C" {
    pub static mut pcibios_min_io: ::core::ffi::c_ulong;
    pub static mut pcibios_min_mem: ::core::ffi::c_ulong;

    pub fn pcibios_report_status(
        status_mask: ::core::ffi::c_uint,
        warn: ::core::ffi::c_int,
    );
}

#[cfg(__KERNEL__)]
#[inline(always)]
pub unsafe fn PCIBIOS_MIN_IO() -> ::core::ffi::c_ulong {
    pcibios_min_io
}

#[cfg(__KERNEL__)]
#[inline(always)]
pub unsafe fn PCIBIOS_MIN_MEM() -> ::core::ffi::c_ulong {
    pcibios_min_mem
}

/* pcibios_assign_all_busses() expands to pci_has_flag(PCI_REASSIGN_ALL_BUS). */
#[cfg(__KERNEL__)]
macro_rules! pcibios_assign_all_busses {
    () => {
        pci_has_flag(PCI_REASSIGN_ALL_BUS)
    };
}

/* CONFIG_PCI_DOMAINS controls whether this declaration is available. */
#[cfg(all(__KERNEL__, CONFIG_PCI_DOMAINS))]
#[inline(always)]
pub unsafe fn pci_proc_domain(bus: *mut pci_bus) -> ::core::ffi::c_int {
    pci_domain_nr(bus)
}

/* #define HAVE_PCI_MMAP */
/* #define ARCH_GENERIC_PCI_MMAP_RESOURCE */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
