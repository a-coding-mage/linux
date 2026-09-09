// SPDX-License-Identifier: GPL-2.0-only
/*
 * Suspend support specific for power.
 *
 * Copyright (c) 2002 Pavel Machek <pavel@ucw.cz>
 * Copyright (c) 2001 Patrick Mochel <mochel@osdl.org>
 */

// Dependencies supplied by the surrounding kernel translation.

/*
 *	pfn_is_nosave - check if given pfn is in the 'nosave' section
 */

pub unsafe fn pfn_is_nosave(pfn: ::core::ffi::c_ulong) -> ::core::ffi::c_int {
    let nosave_begin_pfn: ::core::ffi::c_ulong =
        __pa(&__nosave_begin as *const _ as *const ::core::ffi::c_void) >> PAGE_SHIFT;
    let nosave_end_pfn: ::core::ffi::c_ulong =
        PAGE_ALIGN(__pa(&__nosave_end as *const _ as *const ::core::ffi::c_void)) >> PAGE_SHIFT;
    if (pfn >= nosave_begin_pfn) && (pfn < nosave_end_pfn) {
        1
    } else {
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
