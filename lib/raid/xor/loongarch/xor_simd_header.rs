/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Simple interface to link xor_simd.c and xor_simd_glue.c
 *
 * Separating these files ensures that no SIMD instructions are run outside of
 * the kfpu critical section.
 */

/* The following declarations are present when CONFIG_CPU_HAS_LSX is enabled. */
#[cfg(CONFIG_CPU_HAS_LSX)]
extern "C" {
    pub fn __xor_lsx_2(bytes: ::core::ffi::c_ulong, p1: *mut ::core::ffi::c_ulong,
                        p2: *const ::core::ffi::c_ulong);
    pub fn __xor_lsx_3(bytes: ::core::ffi::c_ulong, p1: *mut ::core::ffi::c_ulong,
                        p2: *const ::core::ffi::c_ulong,
                        p3: *const ::core::ffi::c_ulong);
    pub fn __xor_lsx_4(bytes: ::core::ffi::c_ulong, p1: *mut ::core::ffi::c_ulong,
                        p2: *const ::core::ffi::c_ulong,
                        p3: *const ::core::ffi::c_ulong,
                        p4: *const ::core::ffi::c_ulong);
    pub fn __xor_lsx_5(bytes: ::core::ffi::c_ulong, p1: *mut ::core::ffi::c_ulong,
                        p2: *const ::core::ffi::c_ulong,
                        p3: *const ::core::ffi::c_ulong,
                        p4: *const ::core::ffi::c_ulong,
                        p5: *const ::core::ffi::c_ulong);
}

/* The following declarations are present when CONFIG_CPU_HAS_LASX is enabled. */
#[cfg(CONFIG_CPU_HAS_LASX)]
extern "C" {
    pub fn __xor_lasx_2(bytes: ::core::ffi::c_ulong, p1: *mut ::core::ffi::c_ulong,
                         p2: *const ::core::ffi::c_ulong);
    pub fn __xor_lasx_3(bytes: ::core::ffi::c_ulong, p1: *mut ::core::ffi::c_ulong,
                         p2: *const ::core::ffi::c_ulong,
                         p3: *const ::core::ffi::c_ulong);
    pub fn __xor_lasx_4(bytes: ::core::ffi::c_ulong, p1: *mut ::core::ffi::c_ulong,
                         p2: *const ::core::ffi::c_ulong,
                         p3: *const ::core::ffi::c_ulong,
                         p4: *const ::core::ffi::c_ulong);
    pub fn __xor_lasx_5(bytes: ::core::ffi::c_ulong, p1: *mut ::core::ffi::c_ulong,
                         p2: *const ::core::ffi::c_ulong,
                         p3: *const ::core::ffi::c_ulong,
                         p4: *const ::core::ffi::c_ulong,
                         p5: *const ::core::ffi::c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
