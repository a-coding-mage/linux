// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/arch/arm/mach-omap2/common.c
 *
 * Code common to all OMAP2+ machines.
 *
 * Copyright (C) 2009 Texas Instruments
 * Copyright (C) 2010 Nokia Corporation
 * Tony Lindgren <tony@atomide.com>
 * Added OMAP4 support - Santosh Shilimkar <santosh.shilimkar@ti.com>
 */

// Dependencies supplied by the corresponding kernel headers and source files.
extern "C" {
    fn omap_barrier_reserve_memblock();
}

/*
 * Stub function for OMAP2 so that common files
 * continue to build when custom builds are used
 */
// C __weak linkage is preserved by the surrounding build/link configuration.
#[no_mangle]
pub extern "C" fn omap_secure_ram_reserve_memblock() -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn omap_reserve() {
    unsafe {
        omap_secure_ram_reserve_memblock();
        omap_barrier_reserve_memblock();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
