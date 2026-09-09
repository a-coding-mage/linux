// SPDX-License-Identifier: GPL-2.0-or-later
/* mpi-sub-ui.c - Subtract an unsigned integer from an MPI.
 *
 * Copyright 1991, 1993, 1994, 1996, 1999-2002, 2004, 2012, 2013, 2015
 * Free Software Foundation, Inc.
 *
 * This file was based on the GNU MP Library source file:
 * https://gmplib.org/repo/gmp-6.2/file/510b83519d1c/mpz/aors_ui.h
 *
 * The GNU MP Library is free software; you can redistribute it and/or modify
 * it under the terms of either:
 *
 *   * The GNU Lesser General Public License as published by the Free
 *     Software Foundation; either version 3 of the License, or (at your
 *     option) any later version.
 *
 * or
 *
 *   * the GNU General Public License as published by the Free Software
 *     Foundation; either version 2 of the License, or (at your
 *     option) any later version.
 *
 * or both in parallel, as here.
 */

// Declarations supplied by mpi-internal.h and the kernel build environment.
pub type MpiLimb = ::core::ffi::c_ulong;

#[repr(C)]
pub struct MpiStruct {
    pub nlimbs: ::core::ffi::c_uint,
    pub sign: ::core::ffi::c_int,
    pub d: *mut MpiLimb,
}

pub type MPI = *mut MpiStruct;

unsafe extern "C" {
    fn mpi_resize(a: MPI, nlimbs: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    fn mpihelp_add_1(
        res_ptr: *mut MpiLimb,
        s1_ptr: *const MpiLimb,
        size: ::core::ffi::c_uint,
        s2_limb: MpiLimb,
    ) -> MpiLimb;
    fn mpihelp_sub_1(
        res_ptr: *mut MpiLimb,
        s1_ptr: *const MpiLimb,
        size: ::core::ffi::c_uint,
        s2_limb: MpiLimb,
    ) -> MpiLimb;
    fn mpi_normalize(a: MPI);
}

const ENOMEM: ::core::ffi::c_int = 12;

pub unsafe fn mpi_sub_ui(w: MPI, u: MPI, vval: ::core::ffi::c_ulong) -> ::core::ffi::c_int {
    if (*u).nlimbs == 0 {
        if mpi_resize(w, 1) < 0 {
            return -ENOMEM;
        }
        *(*w).d = vval;
        (*w).nlimbs = (vval != 0) as ::core::ffi::c_uint;
        (*w).sign = (vval != 0) as ::core::ffi::c_int;
        return 0;
    }

    /* If not space for W (and possible carry), increase space. */
    if mpi_resize(w, (*u).nlimbs + 1) != 0 {
        return -ENOMEM;
    }

    if (*u).sign != 0 {
        let cy: MpiLimb;

        cy = mpihelp_add_1((*w).d, (*u).d, (*u).nlimbs, vval as MpiLimb);
        *(*w).d.add((*u).nlimbs as usize) = cy;
        (*w).nlimbs = (*u).nlimbs + cy as ::core::ffi::c_uint;
        (*w).sign = 1;
    } else {
        /* The signs are different.  Need exact comparison to determine
         * which operand to subtract from which.
         */
        if (*u).nlimbs == 1 && *(*u).d < vval {
            *(*w).d = vval - *(*u).d;
            (*w).nlimbs = 1;
            (*w).sign = 1;
        } else {
            mpihelp_sub_1((*w).d, (*u).d, (*u).nlimbs, vval as MpiLimb);
            /* Size can decrease with at most one limb. */
            (*w).nlimbs = (*u).nlimbs
                - (*(*w).d.add(((*u).nlimbs - 1) as usize) == 0) as ::core::ffi::c_uint;
            (*w).sign = 0;
        }
    }

    mpi_normalize(w);
    0
}

// EXPORT_SYMBOL_GPL(mpi_sub_ui);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
