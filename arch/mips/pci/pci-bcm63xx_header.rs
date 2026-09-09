/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding BCM63xx headers:
// bcm63xx_cpu.h, bcm63xx_io.h, bcm63xx_regs.h, bcm63xx_dev_pci.h

/*
 * Cardbus shares  the PCI bus, but has\t no IDSEL, so a\t special id is
 * reserved for it.  If you have a standard PCI device at this id, you
 * need to change the following definition.
 */
pub const CARDBUS_PCI_IDSEL: u32 = 0x8;

pub const PCIE_BUS_BRIDGE: u32 = 0;
pub const PCIE_BUS_DEVICE: u32 = 1;

/*
 * defined in ops-bcm63xx.c
 */
#[repr(C)]
pub struct pci_ops {
    _private: [u8; 0],
}

extern "C" {
    pub static mut bcm63xx_pci_ops: pci_ops;
    pub static mut bcm63xx_cb_ops: pci_ops;
    pub static mut bcm63xx_pcie_ops: pci_ops;
}

/*
 * defined in pci-bcm63xx.c
 */
extern "C" {
    pub static mut pci_iospace_start: *mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
