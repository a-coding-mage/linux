/*
 * Author: MontaVista Software, Inc.
 *       source@mvista.com
 *
 * Copyright 2001-2002 MontaVista Software Inc.
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the Free
 * Software Foundation; either version 2 or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for
 * more details.
 */

// Dependency: <asm/txx9/tx4927.h> supplies IO_BASE, TXX9_CE, TXX9_IRQ_BASE,
// TX4927_NUM_IR, and TX4927_IR_INT.

pub const RBTX4927_PCIMEM: usize = 0x08000000;
pub const RBTX4927_PCIMEM_SIZE: usize = 0x08000000;
pub const RBTX4927_PCIIO: usize = 0x16000000;
pub const RBTX4927_PCIIO_SIZE: usize = 0x01000000;

pub const RBTX4927_LED_ADDR: usize = IO_BASE + TXX9_CE(2) + 0x00001000;
pub const RBTX4927_IMASK_ADDR: usize = IO_BASE + TXX9_CE(2) + 0x00002000;
pub const RBTX4927_IMSTAT_ADDR: usize = IO_BASE + TXX9_CE(2) + 0x00002006;
pub const RBTX4927_SOFTINT_ADDR: usize = IO_BASE + TXX9_CE(2) + 0x00003000;
pub const RBTX4927_SOFTRESET_ADDR: usize = IO_BASE + TXX9_CE(2) + 0x0000f000;
pub const RBTX4927_SOFTRESETLOCK_ADDR: usize = IO_BASE + TXX9_CE(2) + 0x0000f002;
pub const RBTX4927_PCIRESET_ADDR: usize = IO_BASE + TXX9_CE(2) + 0x0000f006;
pub const RBTX4927_BRAMRTC_BASE: usize = IO_BASE + TXX9_CE(2) + 0x00010000;
pub const RBTX4927_ETHER_BASE: usize = IO_BASE + TXX9_CE(2) + 0x00020000;

/* Ethernet port address */
pub const RBTX4927_ETHER_ADDR: usize = RBTX4927_ETHER_BASE + 0x280;

pub const rbtx4927_imask_addr: *mut u8 = RBTX4927_IMASK_ADDR as *mut u8;
pub const rbtx4927_imstat_addr: *mut u8 = RBTX4927_IMSTAT_ADDR as *mut u8;
pub const rbtx4927_softint_addr: *mut u8 = RBTX4927_SOFTINT_ADDR as *mut u8;
pub const rbtx4927_softreset_addr: *mut u8 = RBTX4927_SOFTRESET_ADDR as *mut u8;
pub const rbtx4927_softresetlock_addr: *mut u8 = RBTX4927_SOFTRESETLOCK_ADDR as *mut u8;
pub const rbtx4927_pcireset_addr: *mut u8 = RBTX4927_PCIRESET_ADDR as *mut u8;

/* bits for ISTAT/IMASK/IMSTAT */
pub const RBTX4927_INTB_PCID: usize = 0;
pub const RBTX4927_INTB_PCIC: usize = 1;
pub const RBTX4927_INTB_PCIB: usize = 2;
pub const RBTX4927_INTB_PCIA: usize = 3;
pub const RBTX4927_INTF_PCID: usize = 1 << RBTX4927_INTB_PCID;
pub const RBTX4927_INTF_PCIC: usize = 1 << RBTX4927_INTB_PCIC;
pub const RBTX4927_INTF_PCIB: usize = 1 << RBTX4927_INTB_PCIB;
pub const RBTX4927_INTF_PCIA: usize = 1 << RBTX4927_INTB_PCIA;

pub const RBTX4927_NR_IRQ_IOC: usize = 8; /* IOC */

pub const RBTX4927_IRQ_IOC: usize = TXX9_IRQ_BASE + TX4927_NUM_IR;
pub const RBTX4927_IRQ_IOC_PCID: usize = RBTX4927_IRQ_IOC + RBTX4927_INTB_PCID;
pub const RBTX4927_IRQ_IOC_PCIC: usize = RBTX4927_IRQ_IOC + RBTX4927_INTB_PCIC;
pub const RBTX4927_IRQ_IOC_PCIB: usize = RBTX4927_IRQ_IOC + RBTX4927_INTB_PCIB;
pub const RBTX4927_IRQ_IOC_PCIA: usize = RBTX4927_IRQ_IOC + RBTX4927_INTB_PCIA;

pub const RBTX4927_IRQ_IOCINT: usize = TXX9_IRQ_BASE + TX4927_IR_INT(1);

#[cfg(CONFIG_PCI)]
pub const RBTX4927_ISA_IO_OFFSET: usize = RBTX4927_PCIIO;
#[cfg(not(CONFIG_PCI))]
pub const RBTX4927_ISA_IO_OFFSET: usize = 0;

pub const RBTX4927_RTL_8019_BASE: usize = RBTX4927_ETHER_ADDR - mips_io_port_base;
pub const RBTX4927_RTL_8019_IRQ: usize = TXX9_IRQ_BASE + TX4927_IR_INT(3);

pub unsafe extern "C" {
    pub fn rbtx4927_prom_init();
    pub fn rbtx4927_irq_setup();
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

pub unsafe extern "C" {
    pub fn rbtx4927_pci_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
