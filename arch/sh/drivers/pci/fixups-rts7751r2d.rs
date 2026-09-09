// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/drivers/pci/fixups-rts7751r2d.c
 *
 * RTS7751R2D / LBOXRE2 PCI fixups
 *
 * Copyright (C) 2003  Lineo uSolutions, Inc.
 * Copyright (C) 2004  Paul Mundt
 * Copyright (C) 2007  Nobuhiro Iwamatsu
 */

// The following types, functions, constants, and register definitions are
// supplied by the corresponding kernel and machine dependencies.
#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_channel {
    _private: [u8; 0],
}

extern "C" {
    fn mach_is_lboxre2() -> bool;
    fn __raw_readl(addr: usize) -> libc::c_ulong;
    fn pci_write_reg(chan: *mut pci_channel, value: libc::c_ulong, reg: usize);
}

const PCIMCR_MRSET_OFF: libc::c_ulong = 0xBFFFFFFF;
const PCIMCR_RFSH_OFF: libc::c_ulong = 0xFFFFFFFB;

static mut rts7751r2d_irq_tab: [u8; 4] = [
    IRQ_PCI_INTA,
    IRQ_PCI_INTB,
    IRQ_PCI_INTC,
    IRQ_PCI_INTD,
];

static mut lboxre2_irq_tab: [i8; 4] = [
    IRQ_ETH0,
    IRQ_ETH1,
    IRQ_INTA,
    IRQ_INTD,
];

pub unsafe fn pcibios_map_platform_irq(
    _pdev: *const pci_dev,
    slot: u8,
    _pin: u8,
) -> i32 {
    if mach_is_lboxre2() {
        lboxre2_irq_tab[slot as usize] as i32
    } else {
        rts7751r2d_irq_tab[slot as usize] as i32
    }
}

pub unsafe fn pci_fixup_pcic(chan: *mut pci_channel) -> i32 {
    let mut bcr1: libc::c_ulong;
    let mut mcr: libc::c_ulong;

    bcr1 = __raw_readl(SH7751_BCR1);
    bcr1 |= 0x40080000; // Enable Bit 19 BREQEN, set PCIC to slave
    pci_write_reg(chan, bcr1, SH4_PCIBCR1);

    // Enable all interrupts, so we known what to fix
    pci_write_reg(chan, 0x0000c3ff, SH4_PCIINTM);
    pci_write_reg(chan, 0x0000380f, SH4_PCIAINTM);

    pci_write_reg(chan, 0xfb900047, SH7751_PCICONF1);
    pci_write_reg(chan, 0xab000001, SH7751_PCICONF4);

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
