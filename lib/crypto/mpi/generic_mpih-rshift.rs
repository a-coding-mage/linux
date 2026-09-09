// SPDX-License-Identifier: GPL-2.0-or-later
/* mpih-rshift.c  -  MPI helper functions
 * Copyright (C) 1994, 1996, 1998, 1999,
 *               2000, 2001 Free Software Foundation, Inc.
 *
 * This file is part of GNUPG
 *
 * Note: This code is heavily based on the GNU MP Library.
 *	 Actually it's the same code with only minor changes in the
 *	 way the data is stored; this is to support the abstraction
 *	 of an optional secure memory allocation which may be used
 *	 to avoid revealing of sensitive data due to paging etc.
 *	 The GNU MP Library itself is published under the LGPL;
 *	 however I decided to publish this code under the plain GPL.
 */

// Dependency declarations and build-time definitions are supplied by mpi-internal.

/* Shift U (pointed to by UP and USIZE limbs long) CNT bits to the right
 * and store the USIZE least significant limbs of the result at WP.
 * The bits shifted out to the right are returned.
 *
 * Argument constraints:
 * 1. 0 < CNT < BITS_PER_MP_LIMB
 * 2. If the result is to be written over the input, WP must be <= UP.
 */
pub unsafe fn mpihelp_rshift(
    mut wp: mpi_ptr_t,
    up: mpi_ptr_t,
    usize: mpi_size_t,
    cnt: ::core::primitive::u32,
) -> mpi_limb_t {
    let mut high_limb: mpi_limb_t;
    let mut low_limb: mpi_limb_t;
    let sh_1: ::core::primitive::u32;
    let sh_2: ::core::primitive::u32;
    let mut i: mpi_size_t;
    let retval: mpi_limb_t;

    sh_1 = cnt;
    wp = wp.sub(1);
    sh_2 = BITS_PER_MPI_LIMB - sh_1;
    high_limb = *up;
    retval = high_limb << sh_2;
    low_limb = high_limb;
    i = 1;
    while i < usize {
        high_limb = *up.add(i as usize);
        *wp.add(i as usize) = (low_limb >> sh_1) | (high_limb << sh_2);
        low_limb = high_limb;
        i += 1;
    }
    *wp.add(i as usize) = low_limb >> sh_1;

    retval
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
