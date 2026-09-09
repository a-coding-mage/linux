// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the Linux SH PCI implementation are intentionally
// left as external symbols.

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct PciResource {
    pub start: c_ulong,
}

#[repr(C)]
pub struct pci_channel {
    pub resources: *mut PciResource,
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

extern "C" {
    fn evt2irq(event: u32) -> c_int;
    fn printk(format: *const c_char, ...) -> c_int;
    fn writel(value: u32, address: *mut c_void);
    fn readl(address: *const c_void) -> u32;
    fn pci_reg(reg: c_ulong) -> *mut c_void;
}

// The following processor and PCIC register constants are provided by the
// surrounding SH7751 PCI headers.
extern "C" {
    static SH7751_BCR1: c_ulong;
    static SH7751_BCR2: c_ulong;
    static SH7751_WCR1: c_ulong;
    static SH7751_WCR2: c_ulong;
    static SH7751_WCR3: c_ulong;
    static SH7751_MCR: c_ulong;
    static SH7751_PCIBCR1: c_ulong;
    static SH7751_PCIBCR2: c_ulong;
    static SH7751_PCIWCR1: c_ulong;
    static SH7751_PCIWCR2: c_ulong;
    static SH7751_PCIWCR3: c_ulong;
    static SH7751_PCIMCR: c_ulong;
    static SH7751_PCIINTM: c_ulong;
    static SH7751_PCIAINTM: c_ulong;
    static SH7751_PCICONF1: c_ulong;
    static SH7751_PCICONF2: c_ulong;
    static SH7751_PCICONF4: c_ulong;
    static SH7751_PCICONF5: c_ulong;
    static SH7751_PCICONF6: c_ulong;
    static SH7751_PCICONF11: c_ulong;
    static SH7751_PCILSR0: c_ulong;
    static SH7751_PCILSR1: c_ulong;
    static SH7751_PCILAR0: c_ulong;
    static SH7751_PCILAR1: c_ulong;
    static SH7751_PCICR: c_ulong;
    static SH7751_PCIMBR: c_ulong;
    static SH7751_PCIIOBR: c_ulong;
    static SH7751_PCI_MEMORY_BASE: c_ulong;
    static SH7751_PCIIOBR_MASK: c_ulong;
}

const PCIMCR_MRSET_OFF: u32 = 0xBFFFFFFF;
const PCIMCR_RFSH_OFF: u32 = 0xFFFFFFFB;

#[inline]
unsafe fn pcic_write(reg: c_ulong, value: u32) {
    writel(value, pci_reg(reg));
}

#[allow(dead_code)]
#[inline]
unsafe fn pcic_read(reg: c_ulong) -> u32 {
    readl(pci_reg(reg))
}

pub unsafe fn pcibios_map_platform_irq(_dev: *const pci_dev, slot: u8, _pin: u8) -> c_int {
    match slot {
        0 => evt2irq(0x3a0),
        1 => evt2irq(0x3a0), // AMD Ethernet controller
        2 => -1,
        3 => -1,
        4 => -1,
        _ => {
            printk(b"PCI: Bad IRQ mapping request for slot %d\n\0".as_ptr() as *const c_char, slot as c_int);
            -1
        }
    }
}

pub unsafe fn pci_fixup_pcic(chan: *mut pci_channel) -> c_int {
    let mut bcr1: u32 = core::ptr::read_volatile(SH7751_BCR1 as *const u32);
    let bcr2: u16 = core::ptr::read_volatile(SH7751_BCR2 as *const u16);
    let wcr1: u32 = core::ptr::read_volatile(SH7751_WCR1 as *const u32);
    let wcr2: u32 = core::ptr::read_volatile(SH7751_WCR2 as *const u32);
    let wcr3: u32 = core::ptr::read_volatile(SH7751_WCR3 as *const u32);
    let mut mcr: u32 = core::ptr::read_volatile(SH7751_MCR as *const u32);

    bcr1 |= 0x00080000;
    core::ptr::write_volatile(SH7751_BCR1 as *mut u32, bcr1);

    bcr1 |= 0x40080000;
    pcic_write(SH7751_PCIBCR1, bcr1);
    pcic_write(SH7751_PCIBCR2, bcr2 as u32);
    pcic_write(SH7751_PCIWCR1, wcr1);
    pcic_write(SH7751_PCIWCR2, wcr2);
    pcic_write(SH7751_PCIWCR3, wcr3);
    mcr = (mcr & PCIMCR_MRSET_OFF) & PCIMCR_RFSH_OFF;
    pcic_write(SH7751_PCIMCR, mcr);

    pcic_write(SH7751_PCIINTM, 0x0000c3ff);
    pcic_write(SH7751_PCIAINTM, 0x0000380f);
    pcic_write(SH7751_PCICONF1, 0xF39000C7);
    pcic_write(SH7751_PCICONF2, 0x00000000);
    pcic_write(SH7751_PCICONF4, 0xab000001);
    pcic_write(SH7751_PCICONF5, 0x0c000000);
    pcic_write(SH7751_PCICONF6, 0xd0000000);
    pcic_write(SH7751_PCICONF11, 0x35051054);
    pcic_write(SH7751_PCILSR0, 0x03f00000);
    pcic_write(SH7751_PCILSR1, 0x00000000);
    pcic_write(SH7751_PCILAR0, 0x0c000000);
    pcic_write(SH7751_PCILAR1, 0x00000000);
    pcic_write(SH7751_PCICR, 0xa5000001);

    let resources = (*chan).resources;
    let memory_start = (*resources.add(1)).start;
    if memory_start != SH7751_PCI_MEMORY_BASE {
        panic!("BUG_ON");
    }
    pcic_write(SH7751_PCIMBR, memory_start as u32);
    pcic_write(SH7751_PCIIOBR, ((*resources).start & SH7751_PCIIOBR_MASK) as u32);
    printk(b"SH7751 PCI: Finished initialization of the PCI controller\n\0".as_ptr() as *const c_char);
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
