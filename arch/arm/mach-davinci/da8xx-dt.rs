// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 Texas Instruments Incorporated - https://www.ti.com/
 *
 * Modified from mach-omap/omap2/board-generic.c
 */

// Dependency intent from <asm/mach/arch.h>, "common.h", and "da8xx.h".

#[cfg(CONFIG_ARCH_DAVINCI_DA850)]
extern "C" {
    fn davinci_pm_init();
    fn pdata_quirks_init();
    fn da850_init();
    fn davinci_init_late();
}

#[cfg(CONFIG_ARCH_DAVINCI_DA850)]
unsafe extern "C" fn da850_init_machine() {
    davinci_pm_init();
    pdata_quirks_init();
}

#[cfg(CONFIG_ARCH_DAVINCI_DA850)]
static DA850_BOARDS_COMPAT: [*const core::ffi::c_char; 5] = [
    c"enbw,cmc".as_ptr(),
    c"ti,da850-lcdk".as_ptr(),
    c"ti,da850-evm".as_ptr(),
    c"ti,da850".as_ptr(),
    core::ptr::null(),
];

// DT_MACHINE_START(DA850_DT, "Generic DA850/OMAP-L138/AM18x")
//     .map_io       = da850_init,
//     .init_machine = da850_init_machine,
//     .dt_compat    = da850_boards_compat,
//     .init_late    = davinci_init_late,
// MACHINE_END
// The machine-description object and its macro-generated layout are supplied
// by the architecture dependency represented by <asm/mach/arch.h>.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
