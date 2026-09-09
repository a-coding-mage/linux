/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2003, 2007 Simtec Electronics
 *	http://armlinux.simtec.co.uk/
 *	Ben Dooks <ben@simtec.co.uk>
 *
 * S3C - Memory map definitions (virtual addresses)
 */

/* Fit all our registers in at 0xF6000000 upwards, trying to use as
 * little of the VA space as possible so vmalloc and friends have a
 * better chance of getting memory.
 *
 * we try to ensure stuff like the IRQ registers are available for
 * an single MOVS instruction (ie, only 8 bits of set data)
 */

pub const S3C_ADDR_BASE: usize = 0xF600_0000;

/* In C, S3C_ADDR is a forced __iomem pointer outside assembler builds.
 * The integer address is retained here so both address arithmetic and the
 * assembler form have the same value.
 */
#[inline]
pub const fn S3C_ADDR(x: usize) -> usize {
    S3C_ADDR_BASE + x
}

pub const S3C_VA_IRQ: usize = S3C_ADDR(0x0000_0000); /* irq controller(s) */
pub const S3C_VA_SYS: usize = S3C_ADDR(0x0010_0000); /* system control */
pub const S3C_VA_MEM: usize = S3C_ADDR(0x0020_0000); /* memory control */
pub const S3C_VA_TIMER: usize = S3C_ADDR(0x0030_0000); /* timer block */
pub const S3C_VA_WATCHDOG: usize = S3C_ADDR(0x0040_0000); /* watchdog */
pub const S3C_VA_UART: usize = S3C_ADDR(0x0100_0000); /* UART */

/* ISA device mapping for BAST to use with inb()/outb() on 8-bit I/O.
 * 16-bit I/O on BAST now requires driver modifications to manually
 * ioremap CS3.
 */
pub const S3C24XX_VA_ISA_BYTE: usize = PCI_IOBASE;

/* This is used for the CPU specific mappings that may be needed, so that
 * they do not need to directly used S3C_ADDR() and thus make it easier to
 * modify the space for mapping.
 */
#[inline]
pub const fn S3C_ADDR_CPU(x: usize) -> usize {
    S3C_ADDR(0x0050_0000 + x)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
