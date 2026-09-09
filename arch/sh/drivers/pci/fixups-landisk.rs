// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/drivers/pci/fixups-landisk.c
 *
 * PCI initialization for the I-O DATA Device, Inc. LANDISK board
 *
 * Copyright (C) 2006 kogiidena
 * Copyright (C) 2010 Nobuhiro Iwamatsu
 */

use core::ffi::{c_char, c_int, c_ulong};

// Dependencies supplied by the surrounding kernel translation unit.
#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_channel {
    _private: [u8; 0],
}

extern "C" {
    static SH7751_BCR1: c_ulong;
    static SH7751_MCR: c_ulong;
    static KERN_WARNING: *const c_char;

    fn evt2irq(event: c_int) -> c_int;
    fn __raw_readl(addr: c_ulong) -> c_ulong;
    fn pci_write_reg(chan: *mut pci_channel, value: c_ulong, reg: c_ulong);
    fn printk(fmt: *const c_char, ...) -> c_int;
}

const PCIMCR_MRSET_OFF: c_ulong = 0xBFFFFFFF;
const PCIMCR_RFSH_OFF: c_ulong = 0xFFFFFFFB;

// Register constants supplied by pci-sh4.h.
extern "C" {
    static SH4_PCIBCR1: c_ulong;
    static SH4_PCIMCR: c_ulong;
    static SH7751_PCICONF5: c_ulong;
    static SH7751_PCICONF6: c_ulong;
    static SH4_PCILAR0: c_ulong;
    static SH4_PCILAR1: c_ulong;
}

pub unsafe extern "C" fn pcibios_map_platform_irq(
    _pdev: *const pci_dev,
    slot: u8,
    pin: u8,
) -> c_int {
    /*
     * slot0: pin1-4 = irq5,6,7,8
     * slot1: pin1-4 = irq6,7,8,5
     * slot2: pin1-4 = irq7,8,5,6
     * slot3: pin1-4 = irq8,5,6,7
     */
    let irq = ((((slot as c_int) + (pin as c_int) - 1) & 0x3)
        + evt2irq(0x2a0)) as c_int;

    if (((slot as c_int) | ((pin as c_int) - 1)) > 0x3) {
        let format = b"PCI: Bad IRQ mapping request for slot %d pin %c\n\0";
        let _ = printk(
            format.as_ptr() as *const c_char,
            slot as c_int,
            ((pin as c_int) - 1 + b'A' as c_int),
        );
        return -1;
    }
    irq
}

pub unsafe extern "C" fn pci_fixup_pcic(chan: *mut pci_channel) -> c_int {
    let mut bcr1: c_ulong;
    let mut mcr: c_ulong;

    bcr1 = __raw_readl(SH7751_BCR1);
    bcr1 |= 0x40080000; /* Enable Bit 19 BREQEN, set PCIC to slave */
    pci_write_reg(chan, bcr1, SH4_PCIBCR1);

    mcr = __raw_readl(SH7751_MCR);
    mcr = (mcr & PCIMCR_MRSET_OFF) & PCIMCR_RFSH_OFF;
    pci_write_reg(chan, mcr, SH4_PCIMCR);

    pci_write_reg(chan, 0x0c000000, SH7751_PCICONF5);
    pci_write_reg(chan, 0xd0000000, SH7751_PCICONF6);
    pci_write_reg(chan, 0x0c000000, SH4_PCILAR0);
    pci_write_reg(chan, 0x00000000, SH4_PCILAR1);

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
