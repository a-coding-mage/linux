/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1994 by Waldorf Electronics
 * Copyright (C) 1995 - 2000, 01, 03 by Ralf Baechle
 * Copyright (C) 1999, 2000 Silicon Graphics, Inc.
 * Copyright (C) 2007, 2014 Maciej W. Rozycki
 */

// The C implementation is compiled only when CONFIG_HAVE_PLAT_DELAY is not set.

#[repr(C)]
pub struct RawCurrentCpuData {
    pub udelay_val: ::core::ffi::c_uint,
}

extern "C" {
    pub static raw_current_cpu_data: RawCurrentCpuData;
    // HZ is supplied by the platform/kernel configuration (a C preprocessor constant).
    pub static HZ: ::core::ffi::c_ulong;
}

#[cfg(not(feature = "CONFIG_HAVE_PLAT_DELAY"))]
#[no_mangle]
pub unsafe extern "C" fn __delay(mut loops: ::core::ffi::c_ulong) {
    while loops != 0 {
        loops = loops.wrapping_sub(1);
    }
}

#[cfg(not(feature = "CONFIG_HAVE_PLAT_DELAY"))]
#[no_mangle]
pub unsafe extern "C" fn __udelay(us: ::core::ffi::c_ulong) {
    let lpj: ::core::ffi::c_uint = raw_current_cpu_data.udelay_val;

    __delay(
        ((us as u64)
            .wrapping_mul(0x0000_10c7u64)
            .wrapping_mul(HZ as u64)
            .wrapping_mul(lpj as u64)
            >> 32) as ::core::ffi::c_ulong,
    );
}

#[cfg(not(feature = "CONFIG_HAVE_PLAT_DELAY"))]
#[no_mangle]
pub unsafe extern "C" fn __ndelay(ns: ::core::ffi::c_ulong) {
    let lpj: ::core::ffi::c_uint = raw_current_cpu_data.udelay_val;

    __delay(
        ((ns as u64)
            .wrapping_mul(0x0000_0005u64)
            .wrapping_mul(HZ as u64)
            .wrapping_mul(lpj as u64)
            >> 32) as ::core::ffi::c_ulong,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
