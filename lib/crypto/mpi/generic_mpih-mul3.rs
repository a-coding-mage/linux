// SPDX-License-Identifier: GPL-2.0-or-later
/* mpihelp-mul_3.c  -  MPI helper functions
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

// Dependencies supplied by the surrounding MPI implementation.

pub unsafe fn mpihelp_submul_1(
    mut res_ptr: mpi_ptr_t,
    mut s1_ptr: mpi_ptr_t,
    s1_size: mpi_size_t,
    s2_limb: mpi_limb_t,
) -> mpi_limb_t {
    let mut cy_limb: mpi_limb_t;
    let mut j: mpi_size_t;
    let mut prod_high: mpi_limb_t;
    let mut prod_low: mpi_limb_t;
    let mut x: mpi_limb_t;

    /* The loop counter and index J goes from -SIZE to -1.  This way
     * the loop becomes faster.  */
    j = -s1_size;
    res_ptr = res_ptr.offset(-j as isize);
    s1_ptr = s1_ptr.offset(-j as isize);

    cy_limb = 0;
    loop {
        umul_ppmm(
            &mut prod_high,
            &mut prod_low,
            *s1_ptr.offset(j as isize),
            s2_limb,
        );

        prod_low = prod_low.wrapping_add(cy_limb);
        cy_limb = (if prod_low < cy_limb { 1 } else { 0 }).wrapping_add(prod_high);

        x = *res_ptr.offset(j as isize);
        prod_low = x.wrapping_sub(prod_low);
        cy_limb = cy_limb.wrapping_add(if prod_low > x { 1 } else { 0 });
        *res_ptr.offset(j as isize) = prod_low;

        j += 1;
        if j == 0 {
            break;
        }
    }

    cy_limb
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
