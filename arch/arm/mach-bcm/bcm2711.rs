// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2019 Stefan Wahren
 */

// Dependency intent from <linux/of_address.h>, <asm/mach/arch.h>, and
// "platsmp.h" is supplied by the surrounding kernel translation.

#[cfg(feature = "CONFIG_ARCH_MULTI_V7")]
static BCM2711_COMPAT_ENTRY: &[u8] = b"brcm,bcm2711\0";

// The C source terminates the compatibility list with a NULL pointer.
static BCM2711_COMPAT: &[*const core::ffi::c_char] = &[
    #[cfg(feature = "CONFIG_ARCH_MULTI_V7")]
    BCM2711_COMPAT_ENTRY.as_ptr() as *const core::ffi::c_char,
    core::ptr::null(),
];

// External SMP operations supplied by platsmp.h.
#[repr(C)]
pub struct SmpOperations {
    _private: [u8; 0],
}

extern "C" {
    pub static bcm2836_smp_ops: SmpOperations;
}

#[repr(C)]
pub struct Bcm2711Machine {
    #[cfg(feature = "CONFIG_ZONE_DMA")]
    pub dma_zone_size: usize,
    pub dt_compat: *const *const core::ffi::c_char,
    pub smp: *const SmpOperations,
}

pub static BCM2711_MACHINE: Bcm2711Machine = Bcm2711Machine {
    #[cfg(feature = "CONFIG_ZONE_DMA")]
    dma_zone_size: 1usize << 30, // SZ_1G
    dt_compat: BCM2711_COMPAT.as_ptr(),
    smp: &bcm2836_smp_ops,
};


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
