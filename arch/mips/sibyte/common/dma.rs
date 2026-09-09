// SPDX-License-Identifier: GPL-2.0+
/*
 *\tDMA support for Broadcom SiByte platforms.
 *
 *\tCopyright (c) 2018  Maciej W. Rozycki
 */

// Dependencies supplied by the surrounding kernel translation unit.

/// Platform initialization hook.
pub unsafe fn plat_swiotlb_setup() {
    swiotlb_init(true, SWIOTLB_VERBOSE);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
