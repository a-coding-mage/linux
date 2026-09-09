/* SPDX-License-Identifier: GPL-2.0 */

// Translated from linux/crash_reserve.h.
// C header dependencies and configuration conditions are preserved below as comments.
// #include <linux/linkage.h>
// #include <linux/elfcore.h>
// #include <linux/elf.h>
// #ifdef CONFIG_ARCH_HAS_GENERIC_CRASHKERNEL_RESERVATION
// #include <asm/crash_reserve.h>
// #endif

use core::ffi::{c_char, c_int, c_ulonglong};

// Opaque declarations supplied by the included Linux headers.
#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

#[repr(C)]
pub struct range {
    _private: [u8; 0],
}

/* Location of a reserved region to hold the crash kernel. */
extern "C" {
    pub static mut crashk_res: resource;
    pub static mut crashk_low_res: resource;
    pub static mut crashk_cma_ranges: [range; 4];
}

pub const CRASHK_CMA_RANGES_MAX: c_int = 4;

// Defined when CONFIG_CMA && CONFIG_ARCH_HAS_GENERIC_CRASHKERNEL_RESERVATION.
// #define CRASHKERNEL_CMA
// #define CRASHKERNEL_CMA_RANGES_MAX (CRASHK_CMA_RANGES_MAX)
#[cfg(all(feature = "CONFIG_CMA", feature = "CONFIG_ARCH_HAS_GENERIC_CRASHKERNEL_RESERVATION"))]
pub const CRASHKERNEL_CMA_RANGES_MAX: c_int = CRASHK_CMA_RANGES_MAX;

#[cfg(not(all(feature = "CONFIG_CMA", feature = "CONFIG_ARCH_HAS_GENERIC_CRASHKERNEL_RESERVATION")))]
pub const CRASHKERNEL_CMA_RANGES_MAX: c_int = 0;

#[cfg(all(feature = "CONFIG_CMA", feature = "CONFIG_ARCH_HAS_GENERIC_CRASHKERNEL_RESERVATION"))]
extern "C" {
    pub static mut crashk_cma_cnt: c_int;
}

#[cfg(not(all(feature = "CONFIG_CMA", feature = "CONFIG_ARCH_HAS_GENERIC_CRASHKERNEL_RESERVATION")))]
pub const crashk_cma_cnt: c_int = 0;

extern "C" {
    pub fn parse_crashkernel(
        cmdline: *mut c_char,
        system_ram: c_ulonglong,
        crash_size: *mut c_ulonglong,
        crash_base: *mut c_ulonglong,
        low_size: *mut c_ulonglong,
        cma_size: *mut c_ulonglong,
        high: *mut bool,
    ) -> c_int;

    pub fn reserve_crashkernel_cma(cma_size: c_ulonglong);
}

// CONFIG_ARCH_HAS_GENERIC_CRASHKERNEL_RESERVATION
#[cfg(feature = "CONFIG_ARCH_HAS_GENERIC_CRASHKERNEL_RESERVATION")]
#[inline]
pub fn arch_add_crash_res_to_iomem() -> bool {
    true
}

#[cfg(feature = "CONFIG_ARCH_HAS_GENERIC_CRASHKERNEL_RESERVATION")]
pub const DEFAULT_CRASH_KERNEL_LOW_SIZE: usize = 128usize << 20;

// The C defaults are SZ_2M, SZ_4G, and memblock_end_of_DRAM(), respectively.
#[cfg(feature = "CONFIG_ARCH_HAS_GENERIC_CRASHKERNEL_RESERVATION")]
pub const CRASH_ALIGN: usize = 2usize << 20;

#[cfg(feature = "CONFIG_ARCH_HAS_GENERIC_CRASHKERNEL_RESERVATION")]
pub const CRASH_ADDR_LOW_MAX: usize = 4usize << 30;

#[cfg(feature = "CONFIG_ARCH_HAS_GENERIC_CRASHKERNEL_RESERVATION")]
extern "C" {
    pub fn memblock_end_of_DRAM() -> c_ulonglong;
}

#[cfg(feature = "CONFIG_ARCH_HAS_GENERIC_CRASHKERNEL_RESERVATION")]
#[inline]
pub unsafe fn crash_addr_high_max() -> c_ulonglong {
    memblock_end_of_DRAM()
}

#[cfg(feature = "CONFIG_ARCH_HAS_GENERIC_CRASHKERNEL_RESERVATION")]
extern "C" {
    pub fn reserve_crashkernel_generic(
        crash_size: c_ulonglong,
        crash_base: c_ulonglong,
        crash_low_size: c_ulonglong,
        high: bool,
    );
}

#[cfg(not(feature = "CONFIG_ARCH_HAS_GENERIC_CRASHKERNEL_RESERVATION"))]
#[inline]
pub fn reserve_crashkernel_generic(
    _crash_size: c_ulonglong,
    _crash_base: c_ulonglong,
    _crash_low_size: c_ulonglong,
    _high: bool,
) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
