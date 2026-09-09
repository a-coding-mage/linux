/* mpi-mod.c -  Modular reduction
 * Copyright (C) 1998, 1999, 2001, 2002, 2003,
 *               2007  Free Software Foundation, Inc.
 *
 * This file is part of Libgcrypt.
 */

// Dependency supplied by mpi-internal.h in the C source.
use crate::mpi_internal::MPI;

extern "C" {
    // External function declared by mpi-internal.h.
    fn mpi_fdiv_r(rem: MPI, dividend: MPI, divisor: MPI) -> i32;
}

pub unsafe fn mpi_mod(rem: MPI, dividend: MPI, divisor: MPI) -> i32 {
    mpi_fdiv_r(rem, dividend, divisor)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
