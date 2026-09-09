// SPDX-License-Identifier: GPL-2.0-or-later
/* mpihelp-lshift.c  - MPI helper functions
 * Copyright (C) 1994, 1996, 1998, 2001 Free Software Foundation, Inc.
 *
 * This file is part of GnuPG.
 *
 * Note: This code is heavily based on the GNU MP Library.
 *      Actually it's the same code with only minor changes in the
 *      way the data is stored; this is to support the abstraction
 *      of an optional secure memory allocation which may be used
 *      to avoid revealing of sensitive data due to paging etc.
 *      The GNU MP Library itself is published under the LGPL;
 *      however I decided to publish this code under the plain GPL.
 */

// Dependency declarations such as mpi_limb_t, mpi_ptr_t, mpi_size_t, and
// BITS_PER_MPI_LIMB are supplied by mpi-internal.h in the surrounding code.

/* Shift U (pointed to by UP and USIZE digits long) CNT bits to the left
 * and store the USIZE least significant digits of the result at WP.
 * Return the bits shifted out from the most significant digit.
 *
 * Argument constraints:
 * 1. 0 < CNT < BITS_PER_MP_LIMB
 * 2. If the result is to be written over the input, WP must be >= UP.
 */
pub unsafe fn mpihelp_lshift(
    mut wp: mpi_ptr_t,
    up: mpi_ptr_t,
    usize: mpi_size_t,
    cnt: ::core::ffi::c_uint,
) -> mpi_limb_t {
    let mut high_limb: mpi_limb_t;
    let mut low_limb: mpi_limb_t;
    let sh_1: ::core::ffi::c_uint;
    let sh_2: ::core::ffi::c_uint;
    let mut i: mpi_size_t;
    let retval: mpi_limb_t;

    sh_1 = cnt;
    wp = wp.add(1);
    sh_2 = BITS_PER_MPI_LIMB - sh_1;
    i = usize - 1;
    low_limb = *up.offset(i as isize);
    retval = low_limb >> sh_2;
    high_limb = low_limb;
    loop {
        i -= 1;
        if i < 0 {
            break;
        }
        low_limb = *up.offset(i as isize);
        *wp.offset(i as isize) = (high_limb << sh_1) | (low_limb >> sh_2);
        high_limb = low_limb;
    }
    *wp.offset(i as isize) = high_limb << sh_1;

    retval
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
