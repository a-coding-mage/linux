// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mach-mmp/mmp2-dt.c
 *
 *  Copyright (C) 2012 Marvell Technology Group Ltd.
 *  Author: Haojian Zhuang <haojian.zhuang@marvell.com>
 */

// Dependencies supplied by the Linux kernel and the surrounding translation:
// linux/of_clk.h, linux/clocksource.h, asm/mach/arch.h,
// asm/hardware/cache-tauros2.h, and "common.h".

extern "C" {
    fn tauros2_init(value: i32);
    fn of_clk_init(data: *const core::ffi::c_void);
    fn timer_probe();
    fn mmp2_map_io();
}

unsafe fn mmp_init_time() {
    // Preserved conditional: CONFIG_CACHE_TAUROS2.
    #[cfg(CONFIG_CACHE_TAUROS2)]
    {
        tauros2_init(0);
    }
    of_clk_init(core::ptr::null());
    timer_probe();
}

#[repr(C)]
pub static MMP2_DT_BOARD_COMPAT: [*const core::ffi::c_char; 2] = [
    c"mrvl,mmp2".as_ptr(),
    core::ptr::null(),
];

// DT_MACHINE_START(MMP2_DT, "Marvell MMP2 (Device Tree Support)")
//     .map_io      = mmp2_map_io,
//     .init_time   = mmp_init_time,
//     .dt_compat   = mmp2_dt_board_compat,
// MACHINE_END
// The machine descriptor is provided by the architecture support and retains
// the fields above: map_io = mmp2_map_io, init_time = mmp_init_time, and
// dt_compat = mmp2_dt_board_compat.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
