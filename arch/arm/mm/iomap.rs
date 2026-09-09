// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/arm/mm/iomap.c
 *
 * Map IO port and PCI memory spaces so that {read,write}[bwl] can
 * be used to access this memory.
 */

use core::ffi::c_void;

// Symbols supplied by the kernel headers and other translation units.
extern "C" {
    fn __io(port: c_ulong) -> *mut c_void;
    fn iounmap(addr: *mut c_void);
}

type c_ulong = core::ffi::c_ulong;

#[no_mangle]
pub static mut vga_base: c_ulong = 0;
// EXPORT_SYMBOL(vga_base);

// Corresponds to the C preprocessor condition: __io.
#[cfg(feature = "__io")]
#[no_mangle]
pub unsafe extern "C" fn ioport_map(port: c_ulong, _nr: u32) -> *mut c_void {
    __io(port)
}
// EXPORT_SYMBOL(ioport_map);

// Corresponds to the C preprocessor condition: __io.
#[cfg(feature = "__io")]
#[no_mangle]
pub unsafe extern "C" fn ioport_unmap(_addr: *mut c_void) {
}
// EXPORT_SYMBOL(ioport_unmap);

// Corresponds to the C preprocessor condition: CONFIG_PCI.
#[cfg(feature = "CONFIG_PCI")]
#[no_mangle]
pub static mut pcibios_min_io: c_ulong = 0x1000;
// EXPORT_SYMBOL(pcibios_min_io);

// Corresponds to the C preprocessor condition: CONFIG_PCI.
#[cfg(feature = "CONFIG_PCI")]
#[no_mangle]
pub static mut pcibios_min_mem: c_ulong = 0x0100_0000;
// EXPORT_SYMBOL(pcibios_min_mem);

// Corresponds to the C preprocessor condition: CONFIG_PCI.
#[cfg(feature = "CONFIG_PCI")]
#[no_mangle]
pub unsafe extern "C" fn pci_iounmap(
    _dev: *mut pci_dev,
    addr: *mut c_void,
) {
    if (addr as c_ulong) >= VMALLOC_START && (addr as c_ulong) < VMALLOC_END {
        iounmap(addr);
    }
}
// EXPORT_SYMBOL(pci_iounmap);

// Opaque declaration supplied by the PCI headers.
#[cfg(feature = "CONFIG_PCI")]
#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

// VMALLOC_START and VMALLOC_END are supplied by the architecture headers.
#[cfg(feature = "CONFIG_PCI")]
extern "C" {
    static VMALLOC_START: c_ulong;
    static VMALLOC_END: c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
