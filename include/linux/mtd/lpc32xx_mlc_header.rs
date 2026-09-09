/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Platform data for LPC32xx SoC MLC NAND controller
 *
 * Copyright © 2012 Roland Stigge
 */

// Dependency supplied by the Linux DMA engine declarations:
// #include <linux/dmaengine.h>

#[repr(C)]
pub struct lpc32xx_mlc_platform_data {
	pub dma_filter: dma_filter_fn,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
