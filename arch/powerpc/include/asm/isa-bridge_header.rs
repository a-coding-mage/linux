/* SPDX-License-Identifier: GPL-2.0 */

/* The CONFIG_PPC64 build-time condition is represented by the
 * `CONFIG_PPC64` Cargo configuration option where available. */
#[cfg(feature = "CONFIG_PPC64")]
extern "C" {
    pub fn isa_bridge_find_early(hose: *mut pci_controller);
    pub fn isa_bridge_init_non_pci(np: *mut device_node);
}

#[cfg(feature = "CONFIG_PPC64")]
#[inline]
pub unsafe fn isa_vaddr_is_ioport(address: *mut core::ffi::c_void) -> i32 {
    /* Check if address hits the reserved legacy IO range */
    let ea = address as usize;
    if ea >= ISA_IO_BASE && ea < ISA_IO_END {
        1
    } else {
        0
    }
}

#[cfg(not(feature = "CONFIG_PPC64"))]
#[inline]
pub unsafe fn isa_vaddr_is_ioport(_address: *mut core::ffi::c_void) -> i32 {
    /* No specific ISA handling on ppc32 at this stage, it
     * all goes through PCI
     */
    0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
