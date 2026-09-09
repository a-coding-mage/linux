// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2014 Broadcom Corporation

// Dependency supplied by kona_l2_cache.h.
unsafe extern "C" {
    fn kona_l2_cache_init();
}

unsafe fn bcm21664_init() {
    kona_l2_cache_init();
}

static BCM21664_DT_COMPAT_BCM21664: &[u8] = b"brcm,bcm21664\0";

static BCM21664_DT_COMPAT: [*const core::ffi::c_char; 2] = [
    BCM21664_DT_COMPAT_BCM21664.as_ptr() as *const core::ffi::c_char,
    core::ptr::null(),
];

// Equivalent of DT_MACHINE_START(BCM21664_DT,
// "BCM21664 Broadcom Application Processor"):
//     .init_machine = bcm21664_init,
//     .dt_compat = bcm21664_dt_compat,
// MACHINE_END
// The machine registration structure and macro are supplied by asm/mach/arch.h.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
