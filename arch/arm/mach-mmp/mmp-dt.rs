// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mach-mmp/mmp-dt.c
 *
 *  Copyright (C) 2012 Marvell Technology Group Ltd.
 *  Author: Haojian Zhuang <haojian.zhuang@marvell.com>
 */

// Dependencies supplied by the surrounding kernel translation unit.
extern "C" {
    fn of_clk_init(np: *const core::ffi::c_void);
    fn timer_probe();
    #[cfg(CONFIG_CACHE_TAUROS2)]
    fn tauros2_init(value: i32);
    fn mmp_map_io();
}

static PXA168_DT_BOARD_COMPAT: [Option<&'static core::ffi::CStr>; 2] = [
    Some(c"mrvl,pxa168-aspenite"),
    None,
];

static PXA910_DT_BOARD_COMPAT: [Option<&'static core::ffi::CStr>; 2] = [
    Some(c"mrvl,pxa910-dkb"),
    None,
];

unsafe fn mmp_init_time() {
    #[cfg(CONFIG_CACHE_TAUROS2)]
    tauros2_init(0);
    of_clk_init(core::ptr::null());
    timer_probe();
}

// DT_MACHINE_START(PXA168_DT, "Marvell PXA168 (Device Tree Support)")
//     .map_io = mmp_map_io,
//     .init_time = mmp_init_time,
//     .dt_compat = pxa168_dt_board_compat,
// MACHINE_END
//
// DT_MACHINE_START(PXA910_DT, "Marvell PXA910 (Device Tree Support)")
//     .map_io = mmp_map_io,
//     .init_time = mmp_init_time,
//     .dt_compat = pxa910_dt_board_compat,
// MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
