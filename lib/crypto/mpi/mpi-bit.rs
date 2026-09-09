/* mpi-bit.c  -  MPI bit level functions
 * Copyright (C) 1998, 1999 Free Software Foundation, Inc.
 *
 * This file is part of GnuPG.
 *
 * GnuPG is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License; either version 2
 * or (at your option) any later version.
 */

/* Dependencies supplied by the surrounding translation unit:
 * mpi-internal.h, longlong.h, BITS_PER_MPI_LIMB, mpi_limb_t, mpi_size_t,
 * MPI, count_leading_zeros, mpi_resize, mpihelp_rshift, and MPN_NORMALIZE.
 */

const A_LIMB_1: mpi_limb_t = 1 as mpi_limb_t;

pub unsafe fn mpi_normalize(a: MPI) {
    while (*a).nlimbs != 0 && (*a).d[((*a).nlimbs - 1) as usize] == 0 {
        (*a).nlimbs -= 1;
    }
}

pub unsafe fn mpi_get_nbits(a: MPI) -> u32 {
    let n: u32;

    mpi_normalize(a);

    if (*a).nlimbs != 0 {
        let alimb: mpi_limb_t = (*a).d[((*a).nlimbs - 1) as usize];
        let leading = if alimb != 0 {
            count_leading_zeros(alimb)
        } else {
            BITS_PER_MPI_LIMB
        };
        n = BITS_PER_MPI_LIMB - leading
            + (((*a).nlimbs - 1) * BITS_PER_MPI_LIMB);
    } else {
        n = 0;
    }
    n
}

pub unsafe fn mpi_test_bit(a: MPI, n: u32) -> i32 {
    let limbno = n / BITS_PER_MPI_LIMB;
    let bitno = n % BITS_PER_MPI_LIMB;

    if limbno >= (*a).nlimbs {
        return 0; /* too far left: this is a 0 */
    }
    let limb = (*a).d[limbno as usize];
    if (limb & (A_LIMB_1 << bitno)) != 0 { 1 } else { 0 }
}

pub unsafe fn mpi_set_bit(a: MPI, n: u32) -> i32 {
    let limbno = n / BITS_PER_MPI_LIMB;
    let bitno = n % BITS_PER_MPI_LIMB;

    if limbno >= (*a).nlimbs {
        let mut i = (*a).nlimbs;
        while i < (*a).alloced {
            (*a).d[i as usize] = 0;
            i += 1;
        }
        let err = mpi_resize(a, limbno + 1);
        if err != 0 {
            return err;
        }
        (*a).nlimbs = limbno + 1;
    }
    (*a).d[limbno as usize] |= A_LIMB_1 << bitno;
    0
}

/* Shift A by N bits to the right. */
pub unsafe fn mpi_rshift(x: MPI, a: MPI, n: u32) -> i32 {
    let nlimbs = n / BITS_PER_MPI_LIMB;
    let nbits = n % BITS_PER_MPI_LIMB;
    let mut i: u32;

    if x == a {
        /* In-place operation. */
        if nlimbs >= (*x).nlimbs {
            (*x).nlimbs = 0;
            return 0;
        }

        if nlimbs != 0 {
            i = 0;
            while i < (*x).nlimbs - nlimbs {
                (*x).d[i as usize] = (*x).d[(i + nlimbs) as usize];
                i += 1;
            }
            (*x).d[i as usize] = 0;
            (*x).nlimbs -= nlimbs;
        }
        if (*x).nlimbs != 0 && nbits != 0 {
            mpihelp_rshift((*x).d, (*x).d, (*x).nlimbs, nbits);
        }
    } else if nlimbs != 0 {
        /* Copy and shift by more or equal bits than in a limb. */
        let xsize = (*a).nlimbs;
        (*x).sign = (*a).sign;
        let err = RESIZE_IF_NEEDED!(x, xsize);
        if err != 0 { return err; }
        (*x).nlimbs = xsize;
        i = 0;
        while i < (*a).nlimbs {
            (*x).d[i as usize] = (*a).d[i as usize];
            i += 1;
        }
        (*x).nlimbs = i;

        if nlimbs >= (*x).nlimbs {
            (*x).nlimbs = 0;
            return 0;
        }
        i = 0;
        while i < (*x).nlimbs - nlimbs {
            (*x).d[i as usize] = (*x).d[(i + nlimbs) as usize];
            i += 1;
        }
        (*x).d[i as usize] = 0;
        (*x).nlimbs -= nlimbs;
        if (*x).nlimbs != 0 && nbits != 0 {
            mpihelp_rshift((*x).d, (*x).d, (*x).nlimbs, nbits);
        }
    } else {
        /* Copy and shift by less than bits in a limb. */
        let xsize = (*a).nlimbs;
        (*x).sign = (*a).sign;
        let err = RESIZE_IF_NEEDED!(x, xsize);
        if err != 0 { return err; }
        (*x).nlimbs = xsize;

        if xsize != 0 {
            if nbits != 0 {
                mpihelp_rshift((*x).d, (*a).d, (*x).nlimbs, nbits);
            } else {
                /* The rshift helper function is not specified for NBITS==0. */
                i = 0;
                while i < (*x).nlimbs {
                    (*x).d[i as usize] = (*a).d[i as usize];
                    i += 1;
                }
            }
        }
    }
    MPN_NORMALIZE!((*x).d, (*x).nlimbs);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
