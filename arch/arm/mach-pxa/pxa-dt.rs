// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mach-pxa/pxa-dt.c
 *
 *  Copyright (C) 2012 Daniel Mack
 */

// Dependency equivalent of <asm/mach/arch.h> and "generic.h".

#[cfg(CONFIG_PXA25x)]
static PXA25X_DT_BOARD_COMPAT: [*const ::core::ffi::c_char; 2] = [
    b"marvell,pxa250\0".as_ptr() as *const ::core::ffi::c_char,
    ::core::ptr::null(),
];

#[cfg(CONFIG_PXA25x)]
#[allow(non_upper_case_globals)]
pub static PXA25X_DT: MachineDesc = MachineDesc {
    name: b"Marvell PXA25x (Device Tree Support)\0".as_ptr() as *const ::core::ffi::c_char,
    map_io: Some(pxa25x_map_io),
    restart: Some(pxa_restart),
    dt_compat: PXA25X_DT_BOARD_COMPAT.as_ptr(),
};

#[cfg(CONFIG_PXA27x)]
static PXA27X_DT_BOARD_COMPAT: [*const ::core::ffi::c_char; 2] = [
    b"marvell,pxa270\0".as_ptr() as *const ::core::ffi::c_char,
    ::core::ptr::null(),
];

#[cfg(CONFIG_PXA27x)]
#[allow(non_upper_case_globals)]
pub static PXA27X_DT: MachineDesc = MachineDesc {
    name: b"Marvell PXA27x (Device Tree Support)\0".as_ptr() as *const ::core::ffi::c_char,
    map_io: Some(pxa27x_map_io),
    restart: Some(pxa_restart),
    dt_compat: PXA27X_DT_BOARD_COMPAT.as_ptr(),
};

#[cfg(CONFIG_PXA3xx)]
static PXA3XX_DT_BOARD_COMPAT: [*const ::core::ffi::c_char; 4] = [
    b"marvell,pxa300\0".as_ptr() as *const ::core::ffi::c_char,
    b"marvell,pxa310\0".as_ptr() as *const ::core::ffi::c_char,
    b"marvell,pxa320\0".as_ptr() as *const ::core::ffi::c_char,
    ::core::ptr::null(),
];

#[cfg(CONFIG_PXA3xx)]
#[allow(non_upper_case_globals)]
pub static PXA_DT: MachineDesc = MachineDesc {
    name: b"Marvell PXA3xx (Device Tree Support)\0".as_ptr() as *const ::core::ffi::c_char,
    map_io: Some(pxa3xx_map_io),
    restart: Some(pxa_restart),
    dt_compat: PXA3XX_DT_BOARD_COMPAT.as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
