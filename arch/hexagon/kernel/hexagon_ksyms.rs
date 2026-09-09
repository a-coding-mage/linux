// SPDX-License-Identifier: GPL-2.0-only
/*
 * Export of symbols defined in assembly files and/or libgcc.
 *
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the kernel and architecture-specific headers are
// intentionally left as external symbols.

/* Additional functions */
// EXPORT_SYMBOL(__clear_user_hexagon);
// EXPORT_SYMBOL(raw_copy_from_user);
// EXPORT_SYMBOL(raw_copy_to_user);
// EXPORT_SYMBOL(__vmgetie);
// EXPORT_SYMBOL(__vmsetie);
// EXPORT_SYMBOL(__vmyield);
// EXPORT_SYMBOL(memcpy);
// EXPORT_SYMBOL(memset);

/* Additional variables */
// EXPORT_SYMBOL(__phys_offset);
// EXPORT_SYMBOL(_dflt_cache_att);

unsafe extern "C" {
    pub fn __clear_user_hexagon(to: *mut core::ffi::c_void, n: usize) -> usize;
    pub fn raw_copy_from_user(
        to: *mut core::ffi::c_void,
        from: *const core::ffi::c_void,
        n: usize,
    ) -> usize;
    pub fn raw_copy_to_user(
        to: *mut core::ffi::c_void,
        from: *const core::ffi::c_void,
        n: usize,
    ) -> usize;
    pub fn __vmgetie() -> usize;
    pub fn __vmsetie(value: usize);
    pub fn __vmyield();
    pub fn memcpy(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: usize,
    ) -> *mut core::ffi::c_void;
    pub fn memset(
        dest: *mut core::ffi::c_void,
        value: core::ffi::c_int,
        n: usize,
    ) -> *mut core::ffi::c_void;

    pub static __phys_offset: usize;
    pub static _dflt_cache_att: core::ffi::c_int;

    /* Symbols found in libgcc that assorted kernel modules need */
    pub fn __hexagon_memcpy_likely_aligned_min32bytes_mult8bytes();

    /* Additional functions */
    pub fn __hexagon_divsi3();
    pub fn __hexagon_modsi3();
    pub fn __hexagon_udivsi3();
    pub fn __hexagon_umodsi3();
    pub fn csum_tcpudp_magic();
}

// The EXPORT_SYMBOL declarations above make these externally defined kernel
// symbols available to modules.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
