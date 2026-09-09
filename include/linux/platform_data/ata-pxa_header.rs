/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Generic PXA PATA driver
 *
 * Copyright (C) 2010 Marek Vasut <marek.vasut@gmail.com>
 */

/* Original header guard: __MACH_PATA_PXA_H__ */

#[repr(C)]
pub struct pata_pxa_pdata {
	/* PXA DMA DREQ<0:2> pin */
	pub dma_dreq: u32,
	/* Register shift */
	pub reg_shift: u32,
	/* IRQ flags */
	pub irq_flags: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
