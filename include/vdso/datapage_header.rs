/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from vdso/datapage.h. C preprocessor configuration and
 * dependencies from the original header are supplied by the surrounding
 * build. */

#[cfg(not(feature = "config_arch_has_vdso_time_data"))]
#[repr(C)]
pub struct arch_vdso_time_data {}

#[cfg(not(feature = "config_arch_has_vdso_arch_data"))]
#[repr(C)]
pub struct vdso_arch_data {
    /* Needed for the generic code, never actually used at runtime */
    pub __unused: core::ffi::c_char,
}

pub const VDSO_BASES: usize = (CLOCK_TAI + 1) as usize;
pub const VDSO_BASE_AUX: usize = 0;
pub const VDSO_HRES: u64 = (1u64 << CLOCK_REALTIME)
    | (1u64 << CLOCK_MONOTONIC)
    | (1u64 << CLOCK_BOOTTIME)
    | (1u64 << CLOCK_TAI);
pub const VDSO_COARSE: u64 = (1u64 << CLOCK_REALTIME_COARSE)
    | (1u64 << CLOCK_MONOTONIC_COARSE);
pub const VDSO_RAW: u64 = 1u64 << CLOCK_MONOTONIC_RAW;
pub const VDSO_AUX: u64 = __GENMASK(CLOCK_AUX_LAST, CLOCK_AUX);

pub const CS_HRES_COARSE: usize = 0;
pub const CS_RAW: usize = 1;
pub const CS_BASES: usize = CS_RAW + 1;

/// struct vdso_timestamp - basetime per clock_id
#[repr(C)]
pub struct vdso_timestamp {
    pub sec: u64,
    pub nsec: u64,
}

/// struct vdso_clock - vdso per clocksource datapage representation
#[repr(C)]
pub union vdso_clock_basetime_or_offset {
    pub basetime: [vdso_timestamp; VDSO_BASES],
    pub offset: [timens_offset; VDSO_BASES],
}

#[repr(C)]
pub struct vdso_clock {
    pub seq: u32,
    pub clock_mode: i32,
    pub cycle_last: u64,
    #[cfg(feature = "config_generic_vdso_overflow_protect")]
    pub max_cycles: u64,
    pub mask: u64,
    pub mult: u32,
    pub shift: u32,
    pub basetime_or_offset: vdso_clock_basetime_or_offset,
}

#[repr(C)]
pub struct vdso_time_data {
    pub arch_data: arch_vdso_time_data,
    pub clock_data: [vdso_clock; CS_BASES],
    pub aux_clock_data: [vdso_clock; MAX_AUX_CLOCKS],
    pub tz_minuteswest: i32,
    pub tz_dsttime: i32,
    pub hrtimer_res: u32,
    pub __unused: u32,
}

#[repr(C)]
pub struct vdso_rng_data {
    pub generation: u64,
    pub is_ready: u8,
}

extern "C" {
    pub static mut vdso_u_time_data: vdso_time_data;
    pub static mut vdso_u_rng_data: vdso_rng_data;
    pub static mut vdso_u_arch_data: vdso_arch_data;

    pub static mut vdso_k_time_data: *mut vdso_time_data;
    pub static mut vdso_k_rng_data: *mut vdso_rng_data;
    pub static mut vdso_k_arch_data: *mut vdso_arch_data;
}

pub const VDSO_ARCH_DATA_SIZE: usize = ALIGN(core::mem::size_of::<vdso_arch_data>(), PAGE_SIZE);
pub const VDSO_ARCH_DATA_PAGES: usize = VDSO_ARCH_DATA_SIZE >> PAGE_SHIFT;

#[repr(i32)]
pub enum vdso_pages {
    VDSO_TIME_PAGE_OFFSET,
    VDSO_TIMENS_PAGE_OFFSET,
    VDSO_RNG_PAGE_OFFSET,
    VDSO_ARCH_PAGES_START,
    VDSO_ARCH_PAGES_END = VDSO_ARCH_PAGES_START as isize + VDSO_ARCH_DATA_PAGES as isize - 1,
    VDSO_NR_PAGES,
}

/* The assembler-only PROVIDE/linker-script macros have no executable Rust
 * equivalent and are intentionally preserved here as source-level intent. */
// __vdso_u_rng_data, __vdso_u_arch_data, and VDSO_VVAR_SYMS define linker
// symbols for the vDSO data pages when assembling/linking the vDSO.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
