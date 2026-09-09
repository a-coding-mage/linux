// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2014 Linaro Ltd.
 *
 * Author: Linus Walleij <linus.walleij@linaro.org>
 */

// Dependency supplied by the architecture headers in the surrounding tree.

pub static realview_dt_platform_compat: [*const core::ffi::c_char; 5] = [
    b"arm,realview-eb\0".as_ptr() as *const core::ffi::c_char,
    b"arm,realview-pb1176\0".as_ptr() as *const core::ffi::c_char,
    b"arm,realview-pba8\0".as_ptr() as *const core::ffi::c_char,
    b"arm,realview-pbx\0".as_ptr() as *const core::ffi::c_char,
    core::ptr::null(),
];

// DT_MACHINE_START(REALVIEW_DT, "ARM RealView Machine (Device Tree Support)")
// #ifdef CONFIG_ZONE_DMA
// .dma_zone_size = SZ_256M,
// #endif
// .dt_compat = realview_dt_platform_compat,
// .l2c_aux_val = 0x0,
// .l2c_aux_mask = ~0x0,
// MACHINE_END
// The machine descriptor is emitted by the architecture's DT machine macros.
#[allow(dead_code)]
pub const REALVIEW_DT: RealviewDtMachine = RealviewDtMachine {
    name: "ARM RealView Machine (Device Tree Support)",
    #[cfg(feature = "CONFIG_ZONE_DMA")]
    dma_zone_size: 256 * 1024 * 1024,
    dt_compat: &realview_dt_platform_compat,
    l2c_aux_val: 0x0,
    l2c_aux_mask: !0x0,
};

// Corresponds to the machine descriptor type provided by <asm/mach/arch.h>.
// Its exact definition is supplied by the surrounding architecture code.
#[allow(dead_code)]
pub struct RealviewDtMachine {
    pub name: &'static str,
    #[cfg(feature = "CONFIG_ZONE_DMA")]
    pub dma_zone_size: usize,
    pub dt_compat: &'static [*const core::ffi::c_char],
    pub l2c_aux_val: u32,
    pub l2c_aux_mask: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
