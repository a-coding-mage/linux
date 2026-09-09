// SPDX-License-Identifier: GPL-2.0-or-later
/* mpihelp-sub.c  -  MPI helper functions
 *	Copyright (C) 1994, 1996 Free Software Foundation, Inc.
 *	Copyright (C) 1998, 1999, 2000, 2001 Free Software Foundation, Inc.
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

// Dependency supplied by the surrounding translation unit: mpi-internal.h

/****************
 * Compare OP1_PTR/OP1_SIZE with OP2_PTR/OP2_SIZE.
 * There are no restrictions on the relative sizes of
 * the two arguments.
 * Return 1 if OP1 > OP2, 0 if they are equal, and -1 if OP1 < OP2.
 */
pub unsafe fn mpihelp_cmp(
    op1_ptr: mpi_ptr_t,
    op2_ptr: mpi_ptr_t,
    size: mpi_size_t,
) -> i32 {
    let mut i: mpi_size_t;
    let mut op1_word: mpi_limb_t;
    let mut op2_word: mpi_limb_t;

    i = size - 1;
    while i >= 0 {
        op1_word = *op1_ptr.add(i as usize);
        op2_word = *op2_ptr.add(i as usize);
        if op1_word != op2_word {
            return if op1_word > op2_word { 1 } else { -1 };
        }
        i -= 1;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
