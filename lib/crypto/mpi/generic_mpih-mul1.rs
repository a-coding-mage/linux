// SPDX-License-Identifier: GPL-2.0-or-later
/* mpihelp-mul_1.c  -  MPI helper functions
 * Copyright (C) 1994, 1996, 1997, 1998, 2001 Free Software Foundation, Inc.
 *
 * This file is part of GnuPG.
 *
 * Note: This code is heavily based on the GNU MP Library.
 *	 Actually it's the same code with only minor changes in the
 *	 way the data is stored; this is to support the abstraction
 *	 of an optional secure memory allocation which may be used
 *	 to avoid revealing of sensitive data due to paging etc.
 *	 The GNU MP Library itself is published under the LGPL;
 *	 however I decided to publish this code under the plain GPL.
 */

// Dependencies supplied by the surrounding MPI implementation provide
// `mpi_limb_t`, `mpi_ptr_t`, `mpi_size_t`, and the `umul_ppmm` operation.

pub unsafe fn mpihelp_mul_1(
    mut res_ptr: mpi_ptr_t,
    mut s1_ptr: mpi_ptr_t,
    s1_size: mpi_size_t,
    s2_limb: mpi_limb_t,
) -> mpi_limb_t {
    let mut cy_limb: mpi_limb_t;
    let mut j: mpi_size_t;
    let mut prod_high: mpi_limb_t;
    let mut prod_low: mpi_limb_t;

    /* The loop counter and index J goes from -S1_SIZE to -1.  This way
     * the loop becomes faster.  */
    j = -s1_size;

    /* Offset the base pointers to compensate for the negative indices.  */
    s1_ptr = s1_ptr.offset(-j);
    res_ptr = res_ptr.offset(-j);

    cy_limb = 0;
    loop {
        umul_ppmm!(prod_high, prod_low, *s1_ptr.offset(j), s2_limb);
        prod_low = prod_low.wrapping_add(cy_limb);
        cy_limb = (if prod_low < cy_limb { 1 } else { 0 }).wrapping_add(prod_high);
        *res_ptr.offset(j) = prod_low;

        j = j.wrapping_add(1);
        if j == 0 {
            break;
        }
    }

    cy_limb
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
