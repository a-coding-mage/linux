/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OpenRISC Linux
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * OpenRISC implementation:
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 * et al.
 */

/* C dependencies: linux/types.h, asm/pgalloc.h, and asm/pgtable.h. */

/*
 * PCI: We do not use IO ports in OpenRISC
 */
pub const IO_SPACE_LIMIT: usize = 0;

/* OpenRISC has no port IO */
pub const HAVE_ARCH_PIO_SIZE: usize = 1;
pub const PIO_RESERVED: usize = 0x0;
pub const PIO_OFFSET: usize = 0;
pub const PIO_MASK: usize = 0;

/*
 * I/O memory mapping functions.
 *
 * PAGE_KERNEL, _PAGE_CI, and pgprot_val are supplied by asm/pgtable.h.
 */
pub const _PAGE_IOREMAP: usize = pgprot_val(PAGE_KERNEL) | _PAGE_CI;

/* The generic I/O declarations from asm-generic/io.h are supplied externally. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
