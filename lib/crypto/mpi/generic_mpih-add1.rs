// SPDX-License-Identifier: GPL-2.0-or-later
/* mpihelp-add_1.c  -  MPI helper functions
 * Copyright (C) 1994, 1996, 1997, 1998,
 *               2000 Free Software Foundation, Inc.
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

// Dependencies supplied by the surrounding MPI implementation:
// mpi-internal.h and longlong.h

pub unsafe fn mpihelp_add_n(
    mut res_ptr: mpi_ptr_t,
    mut s1_ptr: mpi_ptr_t,
    mut s2_ptr: mpi_ptr_t,
    size: mpi_size_t,
) -> mpi_limb_t {
    let mut x: mpi_limb_t;
    let mut y: mpi_limb_t;
    let mut cy: mpi_limb_t;
    let mut j: mpi_size_t;

    /* The loop counter and index J goes from -SIZE to -1.  This way
       the loop becomes faster.  */
    j = -size;

    /* Offset the base pointers to compensate for the negative indices. */
    s1_ptr = s1_ptr.offset((-j) as isize);
    s2_ptr = s2_ptr.offset((-j) as isize);
    res_ptr = res_ptr.offset((-j) as isize);

    cy = 0;
    loop {
        y = *s2_ptr.offset(j as isize);
        x = *s1_ptr.offset(j as isize);
        y = y.wrapping_add(cy);       /* add previous carry to one addend */
        cy = (y < cy) as mpi_limb_t; /* get out carry from that addition */
        y = y.wrapping_add(x);       /* add other addend */
        cy = cy.wrapping_add((y < x) as mpi_limb_t); /* get out carry from that add, combine */
        *res_ptr.offset(j as isize) = y;
        j = j.wrapping_add(1);
        if j == 0 {
            break;
        }
    }

    cy
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
