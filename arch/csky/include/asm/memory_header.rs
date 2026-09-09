/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header <linux/compiler.h> dependencies are supplied
// by the surrounding kernel translation.

pub const FIXADDR_TOP: usize = 0xffffc000usize;
pub const PKMAP_BASE: usize = 0xff800000usize;
pub const VMALLOC_START: usize = PAGE_OFFSET + LOWMEM_LIMIT + (PAGE_SIZE * 8);
pub const VMALLOC_END: usize = PKMAP_BASE - (PAGE_SIZE * 2);

// CONFIG_HAVE_TCM
// CONFIG_HAVE_DTCM selects whether the instruction and data TCM page counts
// are combined. These configuration symbols are supplied by the build.
#[cfg(feature = "CONFIG_HAVE_TCM")]
#[cfg(feature = "CONFIG_HAVE_DTCM")]
pub const TCM_NR_PAGES: usize = CONFIG_ITCM_NR_PAGES + CONFIG_DTCM_NR_PAGES;

#[cfg(feature = "CONFIG_HAVE_TCM")]
#[cfg(not(feature = "CONFIG_HAVE_DTCM"))]
pub const TCM_NR_PAGES: usize = CONFIG_ITCM_NR_PAGES;

#[cfg(feature = "CONFIG_HAVE_TCM")]
pub const FIXADDR_TCM: usize = FIXADDR_TOP - (TCM_NR_PAGES * PAGE_SIZE);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
