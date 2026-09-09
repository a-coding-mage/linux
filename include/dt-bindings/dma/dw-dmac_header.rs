/* SPDX-License-Identifier: (GPL-2.0 OR MIT) */

/*
 * Protection Control bits provide protection against illegal transactions.
 * The protection bits[0:2] are one-to-one mapped to AHB HPROT[3:1] signals.
 */
pub const DW_DMAC_HPROT1_PRIVILEGED_MODE: i32 = 1 << 0; /* Privileged Mode */
pub const DW_DMAC_HPROT2_BUFFERABLE: i32 = 1 << 1; /* DMA is bufferable */
pub const DW_DMAC_HPROT3_CACHEABLE: i32 = 1 << 2; /* DMA is cacheable */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
