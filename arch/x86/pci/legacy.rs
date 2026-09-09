// SPDX-License-Identifier: GPL-2.0-only
/*
 * legacy.c - traditional, old school PCI bus probing
 */

// C headers and kernel-provided symbols are supplied by the surrounding tree.

use core::ffi::c_void;

const PCI_VENDOR_ID: i32 = 0x00;
const ENODEV: i32 = 19;

#[repr(C)]
pub struct PciOps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct X86PciInit {
    pub init: unsafe extern "C" fn() -> i32,
    pub init_irq: unsafe extern "C" fn(),
}

#[repr(C)]
pub struct X86Init {
    pub pci: X86PciInit,
}

extern "C" {
    static raw_pci_ops: *mut PciOps;
    static mut pcibios_last_bus: i32;
    static x86_init: X86Init;

    fn jailhouse_paravirt() -> bool;
    fn pci_find_bus(domain: i32, bus: i32) -> *mut c_void;
    fn raw_pci_read(domain: i32, bus: i32, devfn: i32, reg: i32, len: i32, value: *mut u32) -> i32;
    fn pcibios_scan_root(bus: i32);
    fn pcibios_init();
}

macro_rules! DBG {
    ($($arg:tt)*) => {{
        // Kernel DBG() logging is provided by the surrounding environment.
        let _ = format_args!($($arg)*);
    }};
}

macro_rules! pr_info {
    ($($arg:tt)*) => {{
        // Kernel pr_info() logging is provided by the surrounding environment.
        let _ = format_args!($($arg)*);
    }};
}

/*
 * Discover remaining PCI buses in case there are peer host bridges.
 * We use the number of last PCI bus provided by the PCI BIOS.
 */
unsafe fn pcibios_fixup_peer_bridges() {
    let mut n: i32;

    if pcibios_last_bus <= 0 || pcibios_last_bus > 0xff {
        return;
    }
    DBG!("PCI: Peer bridge fixup\n");

    n = 0;
    while n <= pcibios_last_bus {
        pcibios_scan_specific_bus(n);
        n += 1;
    }
}

pub unsafe extern "C" fn pci_legacy_init() -> i32 {
    if raw_pci_ops.is_null() {
        return 1;
    }

    pr_info!("PCI: Probing PCI hardware\n");
    pcibios_scan_root(0);
    0
}

pub unsafe extern "C" fn pcibios_scan_specific_bus(busn: i32) {
    let stride: i32 = if jailhouse_paravirt() { 1 } else { 8 };
    let mut devfn: i32;
    let mut l: u32 = 0;

    if !pci_find_bus(0, busn).is_null() {
        return;
    }

    devfn = 0;
    while devfn < 256 {
        if raw_pci_read(0, busn, devfn, PCI_VENDOR_ID, 2, &mut l) == 0
            && l != 0x0000
            && l != 0xffff
        {
            DBG!("Found device at %02x:%02x [%04x]\n", busn, devfn, l);
            pr_info!("PCI: Discovered peer bus %02x\n", busn);
            pcibios_scan_root(busn);
            return;
        }
        devfn += stride;
    }
}

/* EXPORT_SYMBOL_GPL(pcibios_scan_specific_bus); */

unsafe extern "C" fn pci_subsys_init() -> i32 {
    /*
     * The init function returns an non zero value when
     * pci_legacy_init should be invoked.
     */
    if (x86_init.pci.init)() != 0 {
        if pci_legacy_init() != 0 {
            pr_info!("PCI: System does not support PCI\n");
            return -ENODEV;
        }
    }

    pcibios_fixup_peer_bridges();
    (x86_init.pci.init_irq)();
    pcibios_init();

    0
}

/* subsys_initcall(pci_subsys_init); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
